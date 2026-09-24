//! Ciclo de ventana del diorama.
//!
//! Solo tres responsabilidades: abrir la ventana, leer el teclado y
//! presentar el framebuffer. Todo lo que se puede probar sin ventana vive
//! en la librería del paquete.
//!
//! # La feature `artistic-brush`
//!
//! Encendida por defecto, el botón izquierdo pinta el revelado **donde se
//! arrastra**, sobre la máscara de la superficie que toca. La ruta clásica
//! de revelado por región sigue disponible con `--no-default-features`.
//!
//! Hay tres clases de herramienta: revelar, que descubre el material final
//! donde se arrastra; cinco pigmentos planos, que pintan un color encima de
//! lo que haya; y seis pinceles de textura, que estampan una de las telas
//! que el diorama ya tiene cargadas.
//!
//! El objetivo es **todo el diorama** —el plinto, el continente de fondo, el
//! Monolito—, y no solo las tres regiones que revela la entrega. Hoy no hay
//! ninguna superficie excluida: `input::pick_artistic` reserva el filtro
//! para el atrezo, y desde que se retiró la paleta de cristal ese grupo está
//! vacío.
//!
//! El reparto de responsabilidades no cambia: las máscaras, el picking
//! detallado y la composición viven en la librería y están probados sin
//! ventana. Lo que este archivo aporta bajo la feature es lo que solo
//! existe con un ratón delante —qué herramienta está activa, cuándo empieza
//! un trazo y cuándo se corta—.

use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window, WindowOptions};
use std::f32::consts::PI;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::{Duration, Instant};

#[cfg(feature = "artistic-brush")]
use expedition33_continente_inacabado::brush::{BrushMasks, PigmentMasks, SurfaceKey};
#[cfg(feature = "artistic-brush")]
use expedition33_continente_inacabado::brush_gizmo::draw_brush_gizmo;
#[cfg(feature = "artistic-brush")]
use expedition33_continente_inacabado::brush_palette::{
    dibujar_paleta, herramienta_de_tecla, Disposicion, Herramienta, Impacto, Paleta,
    TECLA_DE_LA_PALETA, TEXTURAS,
};
use expedition33_continente_inacabado::framebuffer::Framebuffer;
#[cfg(feature = "artistic-brush")]
use expedition33_continente_inacabado::input::pick_artistic;
use expedition33_continente_inacabado::input::{demo_action, FrameIntent, PresentedFrame};
#[cfg(not(feature = "artistic-brush"))]
use expedition33_continente_inacabado::input::{pick_region, DemoAction};
use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
#[cfg(feature = "artistic-brush")]
use expedition33_continente_inacabado::renderer::render_artistic;
use expedition33_continente_inacabado::renderer::{
    plan_frame, render, FramePlan, InteractiveProfile, Shading,
};
use expedition33_continente_inacabado::reveal::{reveal_duration, reveal_speed, RevealState};
#[cfg(feature = "artistic-brush")]
use expedition33_continente_inacabado::scene::Scene;
use expedition33_continente_inacabado::scenes::{safe_level_con, WaterPreset};
#[cfg(feature = "artistic-brush")]
use nalgebra_glm::{Vec2, Vec3};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

/// Cuánto gira la cámara por cuadro mientras se sostiene una flecha.
const ROTATION_SPEED: f32 = PI / 60.0;

/// Cuánto se acerca o aleja la cámara por paso de zoom, como fracción del
/// radio actual.
///
/// Es relativo y no absoluto para que el zoom se sienta igual de rápido de
/// lejos que de cerca: un paso fijo en unidades de mundo sería imperceptible
/// desde lejos y brusco desde cerca.
const ZOOM_FRACTION: f32 = 0.06;

/// La rueda del ratón entrega magnitudes muy distintas según el sistema;
/// solo se usa su signo.
const WHEEL_STEPS: f32 = 1.0;

/// Lado de la máscara de pincel, en celdas, para **cada** superficie.
///
/// Es una rejilla por cara y no por objeto, así que `128` da un trazo
/// visiblemente curvo sin que una losa entera cueste una textura grande.
#[cfg(feature = "artistic-brush")]
const BRUSH_RESOLUTION: usize = 128;

/// Radio inicial del pincel, como fracción de `scene_radius`.
///
/// En unidades de **mundo** y no de `uv`. La diferencia se ve en cuanto se
/// pinta: una `uv` es adimensional, así que un radio fijo en `uv` sale
/// diminuto sobre el lecho de la bahía y enorme sobre la tapa de un pilar,
/// y sobre una cara alargada sale ovalado porque sus dos ejes no miden lo
/// mismo. Con un radio de mundo, el trazo mide igual en todas partes.
///
/// La conversión a semiejes `uv` la hace `UvWorldScale::uv_radii` con la
/// métrica que entrega la propia primitiva en cada impacto.
///
/// Se expresa como fracción de la escala **medida** del blockout, no en
/// unidades absolutas, por lo mismo que el resto del proyecto: así sobrevive
/// a un cambio de escala de la escena.
#[cfg(feature = "artistic-brush")]
const BRUSH_RADIUS_INICIAL: f32 = 0.025;

/// Cuánto cambia el radio por pulsación, como factor.
///
/// Multiplicativo y no aditivo: un paso fijo sería imperceptible cerca del
/// máximo y brusco cerca del mínimo, que es el mismo argumento por el que el
/// zoom usa una fracción del radio actual.
#[cfg(feature = "artistic-brush")]
const BRUSH_RADIUS_PASO: f32 = 1.25;

/// Límites del radio del pincel, también como fracción de `scene_radius`.
///
/// Por abajo, un radio demasiado fino no alcanzaría el centro de ninguna
/// celda sobre las superficies grandes y dejaría de pintar. Por arriba, deja
/// de ser un pincel y pasa a ser una brocha que cubre piezas enteras.
#[cfg(feature = "artistic-brush")]
const BRUSH_RADIUS_MINIMO: f32 = 0.005;

#[cfg(feature = "artistic-brush")]
const BRUSH_RADIUS_MAXIMO: f32 = 0.10;

/// Radio mínimo de órbita del modo artístico, como factor de
/// `scene_radius`.
///
/// El modo base usa `scene_builder::MIN_RADIUS_FACTOR`, que vale `1.8` desde
/// la Tarea 7.1, y ahí se queda: no se toca. Pintar pide acercarse más que
/// mirar, así que este modo baja el mínimo a `1.7`.
///
/// # Lo que se sabe y lo que no
///
/// La matriz artística mide el factor `1.7` con fixture real de revelado y
/// pigmento: en el perfil interactivo `320 × 240` deja `1.83x` de margen en
/// Ruta A y `1.75x` en Ruta B sobre el crítico de `0.2667 s/frame`. Por eso
/// esta distancia forma parte de la entrega y no es un número inventado.
#[cfg(feature = "artistic-brush")]
const ARTISTIC_MIN_RADIUS_FACTOR: f32 = 1.7;

/// Opacidad de una pasada.
///
/// Uno: lo que el pincel toca queda revelado del todo. Un valor menor
/// dejaría medias tintas que, con el revelado por región corriendo encima,
/// son indistinguibles de una transición a medias. Ver
/// `reveal::resolve_with_brush`.
#[cfg(feature = "artistic-brush")]
const BRUSH_OPACITY: f32 = 1.0;

/// Traducción de `minifb::Key` a la tecla que nombra `brush_palette`.
///
/// Es lo único que este archivo sabe de la paleta: qué botón físico
/// corresponde a qué carácter. Qué hace ese carácter lo decide el módulo, y
/// es la misma tabla que alimenta la interfaz.
#[cfg(feature = "artistic-brush")]
const TECLAS_DE_LA_PALETA: [(Key, char); 12] = [
    (Key::Q, 'q'),
    (Key::Key4, '4'),
    (Key::Key5, '5'),
    (Key::Key6, '6'),
    (Key::Key7, '7'),
    (Key::Key8, '8'),
    (Key::Key9, '9'),
    (Key::Key0, '0'),
    (Key::Z, 'z'),
    (Key::X, 'x'),
    (Key::C, 'c'),
    (Key::V, 'v'),
];

/// Índice de `aged_wood` en `scene.textures`, para el mango del pincel
/// visual.
///
/// Es la madera del pecio: la herramienta está hecha del mismo material que
/// el naufragio que flota en Aguas Voladoras.
#[cfg(feature = "artistic-brush")]
const TEXTURA_DEL_MANGO: usize = 3;

/// Color de las cerdas del gizmo cuando la herramienta revela.
///
/// Dorado claro: no es ninguno de los cinco pigmentos, así que se distingue
/// de «voy a pintar de ese color» a simple vista.
#[cfg(feature = "artistic-brush")]
const CERDAS_AL_REVELAR: u32 = 0x00E8C86A;

/// Color de reserva si una tela no se puede muestrear.
#[cfg(feature = "artistic-brush")]
const CERDAS_DE_RESERVA: u32 = 0x00D8D2C4;

/// De qué color se ven las cerdas del pincel visual.
///
/// Es lo que el pincel **va a dejar**, no un color decorativo: revelar tiene
/// su dorado, un pigmento muestra el suyo, y una tela muestra el color que
/// esa tela tiene justo en el punto señalado. Así el gizmo responde a la
/// pregunta «¿qué pasa si arrastro aquí?» antes de arrastrar.
#[cfg(feature = "artistic-brush")]
fn cerdas_de(herramienta: Herramienta, scene: &Scene, uv: &Vec2) -> u32 {
    match herramienta {
        Herramienta::Revelar => CERDAS_AL_REVELAR,
        Herramienta::Pigmento(i) => expedition33_continente_inacabado::brush_palette::PIGMENTOS[i]
            .color()
            .to_hex(),
        Herramienta::Textura(i) => {
            let tela = &TEXTURAS[i];

            match scene.textures.get(tela.indice) {
                Some(textura) => textura
                    .sample(uv.x * tela.escala, uv.y * tela.escala)
                    .to_hex(),
                None => CERDAS_DE_RESERVA,
            }
        }
    }
}

/// Cuadros de calibración que se trazan al arrancar.
///
/// Tres y no uno: el primero paga el calentamiento de cachés y sale
/// sistemáticamente más lento. Se toma la mediana.
const CUADROS_DE_CALIBRACION: usize = 3;

fn main() -> ExitCode {
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    // El título usa el nombre de presentación de la obra, no el del
    // paquete ni el del repositorio.
    let mut window = Window::new(
        "El Continente Inacabado",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    // Nivel seguro con el volumen de agua **refractivo**, que es el preset
    // canónico desde la Tarea 5.4: 154 primitivas, techos `0.9 / 0.9` e
    // `ior 1.333`. Los rayos cruzan la superficie y alcanzan las 44
    // primitivas del interior —barco, mástil, cadena, ancla, kelp y rocas—.
    //
    // Las texturas se cargan desde la raíz del proyecto. Si falta alguna,
    // se aborta con su ruta en vez de arrancar con colores planos que nadie
    // distinguiría de un material mal ajustado.
    let raiz = PathBuf::from(".");
    let diorama = match safe_level_con(WaterPreset::RefractiveWater, Some(&raiz)) {
        Ok(diorama) => diorama,
        Err(e) => {
            eprintln!("error: {e}");
            eprintln!("  genera los assets con: cargo run --release --bin generate_assets");
            return ExitCode::FAILURE;
        }
    };
    let lights = luces_del_diorama(&diorama.anchors, &diorama.scale);

    // Arranca **sin pintar**, que es el estado inicial de la obra: un
    // diorama de lienzo esperando a que alguien lo pinte. Es también lo que
    // hace observable el picking, porque un clic sobre una región ya
    // pintada no cambiaría un solo píxel.
    //
    // Para ver el estado final sin interactuar está el render headless:
    // `render_scene --reveal 1.0`.
    let mut reveal = RevealState::unpainted();

    // Encuadre hero **de la escena**, capturado antes de mover sus campos.
    // Los tres puntos salen de las anclas medidas del blockout; no hay
    // ninguna constante de cámara en este archivo ni en `input`.
    let hero = diorama.hero_preset();

    // Ya hay luces: el sombreado completo dice más que el albedo plano.
    // `Shading::Albedo` reproduce las imágenes con las que se aprobó el
    // Blockout 1, y `Normals` sigue disponible para revisar geometría.
    let shading = Shading::Material;

    // El encuadre queda por encima del eje de órbita, y el radio sale de la
    // escala medida: ni la altura ni la distancia se eligieron a mano.
    let mut camera = diorama.hero_camera();

    // El modo artístico se acerca más que el base. Se deriva de la escala
    // **medida** del blockout, no de una constante en unidades de mundo, y
    // conserva el máximo que ya traía la cámara. Ver
    // `ARTISTIC_MIN_RADIUS_FACTOR`.
    // La escala medida del blockout, que es de donde salen tanto los
    // límites de órbita como el grosor del pincel.
    #[cfg(feature = "artistic-brush")]
    let scene_radius = diorama.scale.scene_radius;

    #[cfg(feature = "artistic-brush")]
    let radio_minimo_artistico = scene_radius * ARTISTIC_MIN_RADIUS_FACTOR;

    #[cfg(feature = "artistic-brush")]
    {
        camera = camera.with_radius_limits(radio_minimo_artistico, camera.max_radius);
    }

    // Los encuadres con los que se calibra: los dos más caros de la
    // rejilla medida. Se capturan aquí, antes de consumir el blockout, por
    // lo mismo que la hero.
    let camaras_de_calibracion = diorama.calibration_cameras();

    // La cámara se construye antes de consumir el blockout: `hero_camera`
    // necesita las anclas y la escala, que viven junto a la escena.
    let scene = diorama.scene;
    let accel = diorama.accel;

    // A 800 x 600 el nivel seguro refractivo cuesta ~0.20 s por cuadro:
    // orbitar a 5 fps se siente pegajoso. Mientras algo se mueve se dibuja
    // en el perfil interactivo y se escala; al soltar los controles se
    // produce un cuadro final a resolución completa.
    let perfil = InteractiveProfile::default();
    let mut borrador = Framebuffer::new(perfil.width, perfil.height);

    // ------------------------------------------------ autocalibración
    //
    // El tiempo por cuadro se mide **en esta máquina y ahora**, no se
    // hereda de una constante. Una cifra escrita al compilar es la de la
    // máquina de quien la escribió, y de ella sale la duración de la
    // revelación: en un equipo más lento la animación se quedaría corta de
    // cuadros sin que nada avisara, y el gate de fluidez no llegaría a
    // dispararse nunca.
    //
    // Se mide con el estado y los encuadres más caros que la demo puede
    // presentar: `RevealState::worst_case()` —el clímax, con el Continente
    // pintado y el `Finale` a medio revelar— en las dos cámaras de
    // `calibration_cameras()`, y se toma la peor de las dos medianas.
    //
    // Las tres elecciones son por lo mismo. La duración se fija **una vez**
    // al arrancar, y después el usuario puede pintar, orbitar y acercarse
    // mientras la transición corre. Una cifra tomada del estado o del
    // encuadre que se presentan prometería quince cuadros que un clic o un
    // zoom bastarían para incumplir, sin que nada avisara. Calibrar con lo
    // caro alarga un poco la animación donde sobran cuadros a cambio de que
    // el criterio se cumpla en cualquier sitio al que el usuario pueda
    // llegar.
    //
    // Dos cámaras y no una porque entre esas dos no hay campeón: se turnan
    // el máximo entre corridas, dentro de la dispersión de la máquina. Ver
    // `Blockout::calibration_cameras`.
    //
    // Antes se medía con `reveal 1.0` en la toma hero, que eran las dos
    // elecciones baratas a la vez. Ver la Tarea 7.1.
    let mut frame_time = 0.0_f32;

    for (_, camara) in camaras_de_calibracion {
        let mut muestras = Vec::with_capacity(CUADROS_DE_CALIBRACION);

        for _ in 0..CUADROS_DE_CALIBRACION {
            let inicio = Instant::now();
            render(
                &mut borrador,
                &scene,
                &accel,
                &lights,
                &RevealState::worst_case(),
                &camara,
                shading,
            );
            muestras.push(inicio.elapsed().as_secs_f32());
        }

        muestras.sort_by(|a, b| a.partial_cmp(b).expect("no hay NaN"));
        frame_time = frame_time.max(muestras[muestras.len() / 2]);
    }

    // Duración de la revelación, **derivada** de esa medición, con piso de
    // 1.5 s y techo de 4.0 s. Si el perfil no diera para quince cuadros
    // dentro del techo, esto aborta en vez de alargar la animación, que es
    // lo que el plan prohíbe expresamente.
    let duracion = match reveal_duration(frame_time) {
        Ok(duracion) => duracion,
        Err(fallo) => {
            eprintln!(
                "error: el perfil interactivo falla el gate de fluidez: {:.4} s por cuadro\n  \
                 quince cuadros exigirian {:.2} s y el techo son 4.00 s",
                fallo.interactive_frame_time, fallo.required
            );
            eprintln!("  baja la resolucion del perfil en vez de alargar la animacion");
            return ExitCode::FAILURE;
        }
    };
    let velocidad = reveal_speed(duracion);

    // Un cuadro de calibración no es un cuadro presentado: el framebuffer
    // sigue vacío, así que el primero de verdad se dibuja igual.

    println!("El Continente Inacabado");
    println!(
        "  escena   nivel seguro, {} primitivas, {} luces, {} texturas",
        scene.objects.len(),
        lights.len(),
        scene.textures.len()
    );
    println!(
        "  perfil   {} x {} en movimiento, {WIDTH} x {HEIGHT} en reposo",
        perfil.width, perfil.height
    );
    println!("  flechas  orbitar     W / S / rueda  zoom     Escape  salir");
    #[cfg(not(feature = "artistic-brush"))]
    {
        println!("  clic     pintar la region señalada     1 / 2 / 3  pintar por teclado");
        println!("  L        volver al lienzo     R  restaurar encuadre hero");
    }

    #[cfg(feature = "artistic-brush")]
    {
        println!("  clic     arrastrar para aplicar la herramienta sobre cualquier superficie");
        println!("           todo el diorama es lienzo: nada queda fuera");
        println!("  Q        revelar     4 / 5 / 6 / 7 / 8  pigmento plano");
        println!(
            "  1 / 2 / 3  revelar la region entera     L  volver al lienzo     R  encuadre hero"
        );

        println!("  9 / 0 / Z / X / C / V  pincel de textura");

        let paleta: Vec<&str> = expedition33_continente_inacabado::brush_palette::PIGMENTOS
            .iter()
            .map(|p| p.nombre)
            .collect();
        println!("  paleta   {}", paleta.join(", "));

        let telas: Vec<String> = TEXTURAS
            .iter()
            .map(|tela| format!("{} x{:.1}", tela.nombre, tela.escala))
            .collect();
        println!("  telas    {}", telas.join(", "));
        println!("  M / N    engordar / afinar el pincel");
        println!(
            "  {}        abrir y plegar la paleta     clic en la capsula abajo a la derecha",
            TECLA_DE_LA_PALETA.to_ascii_uppercase()
        );
        println!("           pincel visual al pintar: se apoya en la superficie senalada");
        println!(
            "  pincel   mascara {BRUSH_RESOLUTION} x {BRUSH_RESOLUTION} por superficie, radio {:.3} de mundo",
            scene_radius * BRUSH_RADIUS_INICIAL
        );
        println!("           el grosor se normaliza por superficie: mismo tamano en toda la obra");
        println!(
            "  zoom     radio {:.2} a {:.2}, mas cerca que el modo base ({ARTISTIC_MIN_RADIUS_FACTOR:.1} x scene_radius)",
            radio_minimo_artistico, camera.max_radius
        );
    }
    println!(
        "  revelado {duracion:.2} s por region, {:.0} cuadros, medidos {frame_time:.4} s por cuadro",
        duracion / frame_time
    );

    // El primer cuadro cuenta como cambio pendiente, para que la ventana
    // arranque ya con la imagen definitiva.
    let mut cuadro_final_pendiente = true;

    // Estado del botón en el cuadro anterior, para disparar **una vez por
    // clic**. `get_mouse_down` informa un nivel, no un evento: sin esta
    // comparación, sostener el botón reactivaría la región en cada cuadro.
    //
    // Con el pincel no hace falta: ahí sostener el botón **es** la
    // interacción, y lo que se recuerda entre cuadros es el punto anterior
    // del trazo.
    #[cfg(not(feature = "artistic-brush"))]
    let mut boton_anterior = false;

    // El flanco del botón, para que un clic sobre la interfaz cuente una
    // vez. Al pintar no hace falta —sostener **es** la interacción— pero la
    // paleta sí necesita distinguir pulsar de mantener.
    #[cfg(feature = "artistic-brush")]
    let mut boton_anterior_artistico = false;

    // Una máscara por superficie pintada, para todo el ciclo. Se presta al
    // renderer por referencia: no se construye ni se clona nada por cuadro.
    #[cfg(feature = "artistic-brush")]
    let mut masks = BrushMasks::new(BRUSH_RESOLUTION, BRUSH_RESOLUTION);

    // Último punto del trazo en curso: qué superficie y dónde.
    //
    // `None` significa que el siguiente punto empieza un trazo nuevo, y es
    // lo que impide unir dos arrastres separados con una raya que cruza
    // todo lo que hubiera en medio. Se vuelve `None` al soltar el botón, al
    // salirse de la superficie y al reiniciar el lienzo.
    #[cfg(feature = "artistic-brush")]
    let mut trazo: Option<(SurfaceKey, f32, f32)> = None;

    // Pigmento plano por superficie, con la misma resolución y la misma
    // asignación perezosa que la máscara de revelado.
    #[cfg(feature = "artistic-brush")]
    let mut pigmentos = PigmentMasks::new(BRUSH_RESOLUTION, BRUSH_RESOLUTION);

    // Qué hace el botón izquierdo. Arranca revelando, que es la herramienta
    // que la demostración necesita si nadie toca nada.
    #[cfg(feature = "artistic-brush")]
    let mut paleta = Paleta::default();

    // Qué telas tienen su asset cargado. Se calcula una vez: la escena no
    // cambia, y una celda que no se puede elegir tiene que verse así desde
    // el primer cuadro.
    #[cfg(feature = "artistic-brush")]
    let telas_disponibles: Vec<bool> = TEXTURAS
        .iter()
        .map(|tela| scene.textures.get(tela.indice).is_some())
        .collect();

    // Grosor vigente del pincel, en unidades de **mundo**. Alimenta las
    // tres herramientas, para que cambiar de tela no cambie el trazo, y se
    // normaliza por superficie en el momento de pintar.
    #[cfg(feature = "artistic-brush")]
    let mut brush_radius = scene_radius * BRUSH_RADIUS_INICIAL;

    // Si el cuadro anterior mostró el pincel, el siguiente tiene que
    // volver a trazarse aunque nada más haya cambiado: el gizmo se pinta
    // **encima** del framebuffer, así que reutilizarlo con `FramePlan::Reuse`
    // dejaría el pincel congelado en pantalla después de soltar el botón.
    #[cfg(feature = "artistic-brush")]
    let mut gizmo_anterior = false;

    // Reloj del avance. Se toma justo antes del ciclo para que el primer
    // delta no incluya el tiempo de cargar los assets.
    let mut ultimo_cuadro = Instant::now();

    // Con qué se dibujó **lo que está en pantalla**: cámara y resolución
    // fuente.
    //
    // Las dos pueden diferir de las actuales. La cámara avanza antes de que
    // se lea el clic, porque las teclas de órbita se procesan primero en el
    // mismo cuadro. Y la resolución fuente es la del perfil interactivo
    // mientras algo se mueve, donde un píxel fuente ocupa un bloque de la
    // ventana: el clic tiene que resolverse contra el píxel que produjo la
    // imagen, no contra una coordenada interpolada. Ver
    // `input::PresentedFrame`.
    //
    // Arranca a resolución completa porque el primer cuadro presentado será
    // el final: `cuadro_final_pendiente` empieza en cierto.
    let mut presentado = PresentedFrame::full(camera, (WIDTH, HEIGHT));

    // Qué renderer usa el ciclo. Es lo **único** que la feature cambia en
    // el dibujo, y se escribe una vez para que las dos ramas del plan de
    // cuadro no puedan desincronizarse.
    //
    // La autocalibración de más arriba no pasa por aquí a propósito: mide
    // el renderer normal con el peor estado, que es lo que fija la duración
    // de la revelación. Medirla con la máscara interactiva —vacía al
    // arrancar— daría una cifra que depende de cuánto se haya pintado.
    macro_rules! dibujar {
        ($destino:expr) => {{
            #[cfg(feature = "artistic-brush")]
            {
                render_artistic(
                    $destino, &scene, &accel, &lights, &reveal, &camera, shading, &masks,
                    &pigmentos,
                );
            }

            #[cfg(not(feature = "artistic-brush"))]
            {
                render($destino, &scene, &accel, &lights, &reveal, &camera, shading);
            }
        }};
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let orbit = [
            (Key::Left, ROTATION_SPEED, 0.0),
            (Key::Right, -ROTATION_SPEED, 0.0),
            (Key::Up, 0.0, -ROTATION_SPEED),
            (Key::Down, 0.0, ROTATION_SPEED),
        ];

        // ------------------------------------------------ fuentes de cambio
        //
        // El plan enumera cuatro: cámara, zoom, avance de la revelación y
        // región seleccionada. Cada una se nombra por separado en vez de
        // acumularse en una bandera anónima, porque de esa distinción sale
        // **qué resolución** se usa: las que se sostienen en el tiempo
        // —órbita, zoom, transición— dibujan al perfil interactivo, y las
        // instantáneas —un reinicio— van directas al cuadro final.
        let mut camara_cambio = false;

        for (key, delta_yaw, delta_pitch) in orbit {
            if window.is_key_down(key) {
                camera.orbit(delta_yaw, delta_pitch);
                camara_cambio = true;
            }
        }

        // Zoom por teclado y por rueda. El paso es proporcional al radio
        // actual, así que se siente parejo a cualquier distancia.
        let mut pasos = 0.0;
        if window.is_key_down(Key::W) {
            pasos -= 1.0;
        }
        if window.is_key_down(Key::S) {
            pasos += 1.0;
        }
        if let Some((_, vertical)) = window.get_scroll_wheel() {
            if vertical > 0.0 {
                pasos -= WHEEL_STEPS;
            } else if vertical < 0.0 {
                pasos += WHEEL_STEPS;
            }
        }

        let zoom_cambio = pasos != 0.0;

        if zoom_cambio {
            camera.zoom(pasos * ZOOM_FRACTION * camera.radius());
        }

        // ------------------------------------------------ pintar una región
        //
        // El ratón es la interacción principal y el teclado el respaldo de
        // presentación: una bahía que ocupa el 2.4 % del cuadro no es un
        // blanco fiable delante de público.
        //
        // `MouseMode::Discard` no devuelve posición cuando el puntero salió
        // del área de dibujo, que es la misma política que aplica
        // `input::ray_under_cursor`. Se dejan las dos: la de minifb evita el
        // trabajo y la de la librería es la que se puede probar sin ventana.
        // Se recogen **todas** las acciones del cuadro y se reducen antes
        // de aplicar ninguna. Un solo `Option` dejaba que la última ganara
        // —un clic y una tecla, o `1`, `2` y `3` en el mismo sondeo— y que
        // una selección se aplicara después de un reinicio del mismo
        // cuadro, sobre el lienzo recién limpiado. Ver `input::FrameIntent`.
        let mut acciones = Vec::new();

        let boton = window.get_mouse_down(MouseButton::Left);

        #[cfg(not(feature = "artistic-brush"))]
        {
            let clic = boton && !boton_anterior;
            boton_anterior = boton;

            if clic {
                if let Some(cursor) = window.get_mouse_pos(MouseMode::Discard) {
                    // Contra `camara_presentada`: el clic apunta a lo que se ve.
                    if let Some(grupo) = pick_region(&scene, &accel, &presentado, cursor) {
                        acciones.push(DemoAction::Paint(grupo));
                    }
                }
            }
        }

        // ¿Cambió la interfaz este cuadro? Si sí, hay que volver a trazar
        // aunque nada más se mueva: la paleta se dibuja **encima** del
        // framebuffer, y `FramePlan::Reuse` la dejaría congelada.
        #[cfg(feature = "artistic-brush")]
        let mut paleta_cambio = false;

        // ------------------------------------------------ elegir herramienta
        //
        // Fuera de `input::demo_action` a propósito: esa lista es la del
        // teclado de presentación —las tres regiones, el lienzo y el
        // encuadre—, y sigue siendo la misma con la feature y sin ella. Lo
        // de aquí solo existe bajo la feature.
        #[cfg(feature = "artistic-brush")]
        {
            let mut elegida = None;

            // Una sola traducción: de `minifb::Key` a `char`, y de ahí a
            // `brush_palette`. La tabla que dice qué hace cada tecla es la
            // misma que dibuja la interfaz.
            for (tecla, caracter) in TECLAS_DE_LA_PALETA {
                if window.is_key_pressed(tecla, KeyRepeat::No) {
                    elegida = herramienta_de_tecla(caracter);
                }
            }

            // Una herramienta de textura que apunte a un asset ausente no se
            // activa. Se comprueba **al elegirla** y no al pintar: un aviso
            // por cuadro durante un arrastre llenaría la consola de la misma
            // línea, y esto se sabe una sola vez.
            if let Some(Herramienta::Textura(i)) = elegida {
                if !telas_disponibles[i] {
                    eprintln!(
                        "  aviso: la textura {} no esta cargada (indice {})",
                        TEXTURAS[i].nombre, TEXTURAS[i].indice
                    );
                    elegida = None;
                }
            }

            if let Some(nueva) = elegida {
                if paleta.elegir(nueva) {
                    // Cambiar de herramienta corta el trazo: sin esto, un
                    // arrastre iniciado revelando seguiría como una línea
                    // de color desde el punto donde se pulsó la tecla.
                    trazo = None;

                    println!("  herramienta: {}", nueva.nombre());
                }
            }

            // `P` pliega y despliega el panel.
            if window.is_key_pressed(Key::P, KeyRepeat::No) {
                paleta.alternar();
                paleta_cambio = true;

                println!(
                    "  paleta: {}",
                    if paleta.abierta() {
                        "abierta"
                    } else {
                        "plegada"
                    }
                );
            }
        }

        // ------------------------------------------------ grosor del pincel
        //
        // `M` de *more*, y `N` a su lado. Son letras y no los signos de
        // sumar y restar porque esos dos, aunque compilan, no entregaron
        // evento en el teclado donde se probó: un control que no responde
        // es peor que no tenerlo. `L` queda fuera, que es el reinicio.
        //
        // Aparte de la selección de herramienta y **sin cortar el trazo**:
        // engordar el pincel a mitad de un arrastre es un ajuste continuo,
        // no el principio de otra pincelada.
        #[cfg(feature = "artistic-brush")]
        {
            let mut factor = 1.0;

            if window.is_key_pressed(Key::M, KeyRepeat::No) {
                factor *= BRUSH_RADIUS_PASO;
            }
            if window.is_key_pressed(Key::N, KeyRepeat::No) {
                factor /= BRUSH_RADIUS_PASO;
            }

            if factor != 1.0 {
                let nuevo = (brush_radius * factor).clamp(
                    scene_radius * BRUSH_RADIUS_MINIMO,
                    scene_radius * BRUSH_RADIUS_MAXIMO,
                );

                if nuevo != brush_radius {
                    brush_radius = nuevo;

                    println!("  grosor: {brush_radius:.3} de mundo");
                }
            }
        }

        // ------------------------------------------------ pintar con el pincel
        //
        // Sostener el botón pinta; soltarlo corta el trazo. Se resuelve
        // contra `presentado`, igual que el picking por región: mientras se
        // arrastra, el cuadro se dibuja al perfil interactivo, y el punto
        // tiene que salir del píxel **fuente** que produjo la imagen.
        #[cfg(feature = "artistic-brush")]
        let mut pincel_cambio = false;

        // La disposición de este cuadro. Se recalcula siempre porque es
        // barato y porque el estado de plegado puede haber cambiado.
        #[cfg(feature = "artistic-brush")]
        let disposicion =
            Disposicion::calcular(paleta.abierta(), WIDTH, HEIGHT, &telas_disponibles);

        // ------------------------------------------------ la interfaz primero
        //
        // Un clic sobre la paleta **se consume** y no llega nunca a
        // `pick_artistic`: pulsar un color no puede además pintar un trazo
        // en el diorama que hay detrás del panel.
        #[cfg(feature = "artistic-brush")]
        let mut clic_consumido = false;

        #[cfg(feature = "artistic-brush")]
        {
            let clic = boton && !boton_anterior_artistico;
            boton_anterior_artistico = boton;

            if clic {
                if let Some(cursor) = window.get_mouse_pos(MouseMode::Discard) {
                    match disposicion.impacto(cursor) {
                        Impacto::Capsula => {
                            paleta.alternar();
                            paleta_cambio = true;
                            clic_consumido = true;
                            trazo = None;

                            println!(
                                "  paleta: {}",
                                if paleta.abierta() {
                                    "abierta"
                                } else {
                                    "plegada"
                                }
                            );
                        }
                        Impacto::Celda(i) => {
                            // Todo el panel consume, incluidos sus huecos.
                            clic_consumido = true;

                            if let Some(celda) = disposicion.celdas.get(i) {
                                if celda.activable && paleta.elegir(celda.herramienta) {
                                    // Igual que por teclado: cambiar de
                                    // herramienta corta el trazo.
                                    trazo = None;
                                    paleta_cambio = true;

                                    println!("  herramienta: {}", celda.herramienta.nombre());
                                } else if !celda.activable {
                                    eprintln!(
                                        "  aviso: la textura {} no esta cargada",
                                        celda.herramienta.nombre()
                                    );
                                }
                            }
                        }
                        Impacto::Fuera => {}
                    }
                }
            }

            // Un clic que abrió o plegó el panel no pinta **en ese mismo
            // cuadro**: sería pintar donde el usuario solo quería tocar la
            // interfaz.
            if paleta_cambio {
                clic_consumido = true;
            }
        }

        // El pincel visual de **este** cuadro: dónde apoyarlo, hacia dónde y
        // de qué color. Nace vacío en cada vuelta, así que soltar el botón lo
        // hace desaparecer sin que nadie tenga que borrarlo.
        #[cfg(feature = "artistic-brush")]
        let mut gizmo: Option<(Vec3, Vec3, u32)> = None;

        #[cfg(feature = "artistic-brush")]
        {
            let objetivo = if boton && !clic_consumido {
                window.get_mouse_pos(MouseMode::Discard).and_then(|cursor| {
                    match disposicion.impacto(cursor) {
                        // Arrastrar por encima del panel tampoco pinta: el
                        // panel tapa lo que hay detrás, y pintar a ciegas
                        // ahí sería una sorpresa.
                        Impacto::Fuera => pick_artistic(&scene, &accel, &presentado, cursor),
                        _ => None,
                    }
                })
            } else {
                None
            };

            match objetivo {
                Some(objetivo) => {
                    let clave = SurfaceKey::new(objetivo.object_index, objetivo.uv_chart);
                    let (u, v) = (objetivo.uv.x, objetivo.uv.y);

                    // Continuidad: se une con el punto anterior solo si
                    // era el **mismo** trozo pintable. Cruzar a otra cara o
                    // a otra pieza empieza un sello, porque unir dos cartas
                    // por su `uv` trazaría una raya entre dos puntos que no
                    // son vecinos en la superficie.
                    let continua = matches!(trazo, Some((anterior, _, _)) if anterior == clave);

                    // El radio de mundo, repartido entre los dos ejes `uv`
                    // de **esta** cara. Es lo que hace que el trazo mida lo
                    // mismo sobre una losa enorme y sobre un tablón, y que
                    // salga redondo sobre una cara alargada.
                    let Some((radio_u, radio_v)) = objetivo.uv_world_scale.uv_radii(brush_radius)
                    else {
                        // Sin conversión no hay trazo, y tampoco hay punto
                        // anterior que enlazar: unir desde aquí cruzaría una
                        // raya hasta el siguiente punto válido.
                        trazo = None;
                        continue;
                    };

                    // El punto anterior, solo si el trazo continúa.
                    let desde = match trazo {
                        Some((_, au, av)) if continua => Some((au, av)),
                        _ => None,
                    };

                    match paleta.herramienta() {
                        // Revelado, igual que antes de que hubiera paleta.
                        Herramienta::Revelar => match desde {
                            Some((au, av)) => masks.stroke_ellipse(
                                clave,
                                au,
                                av,
                                u,
                                v,
                                radio_u,
                                radio_v,
                                BRUSH_OPACITY,
                            ),
                            None => {
                                masks.stamp_ellipse(clave, u, v, radio_u, radio_v, BRUSH_OPACITY)
                            }
                        },
                        // Pigmento plano, siempre opaco: la herramienta
                        // aplica el color elegido, y las medias tintas salen
                        // de pasar por encima de otro pigmento.
                        Herramienta::Pigmento(i) => {
                            let color = expedition33_continente_inacabado::brush_palette::PIGMENTOS
                                [i]
                                .color();

                            match desde {
                                Some((au, av)) => pigmentos.stroke_ellipse(
                                    clave, au, av, u, v, radio_u, radio_v, color, 1.0,
                                ),
                                None => pigmentos
                                    .stamp_ellipse(clave, u, v, radio_u, radio_v, color, 1.0),
                            }
                        }
                        // Tela estampada. El índice se comprobó al elegir la
                        // herramienta, pero se vuelve a consultar con `get`:
                        // indexar aquí convertiría un asset ausente en un
                        // panic a mitad de un arrastre.
                        Herramienta::Textura(i) => {
                            let (indice, escala) = (TEXTURAS[i].indice, TEXTURAS[i].escala);

                            if let Some(tela) = scene.textures.get(indice) {
                                match desde {
                                    Some((au, av)) => pigmentos.stroke_texture_ellipse(
                                        clave, au, av, u, v, radio_u, radio_v, tela, escala, 1.0,
                                    ),
                                    None => pigmentos.stamp_texture_ellipse(
                                        clave, u, v, radio_u, radio_v, tela, escala, 1.0,
                                    ),
                                }
                            }
                        }
                    }

                    // El pincel visual de este cuadro. Se guarda el punto y
                    // la normal que entregó el propio impacto: no se
                    // reconstruyen ni se aproximan.
                    gizmo = Some((
                        objetivo.point,
                        objetivo.normal,
                        cerdas_de(paleta.herramienta(), &scene, &objetivo.uv),
                    ));

                    trazo = Some((clave, u, v));
                    pincel_cambio = true;
                }
                // Botón suelto, puntero fuera de la ventana, o nada
                // pintable bajo el cursor: el trazo se corta.
                None => trazo = None,
            }
        }

        // Las teclas pasan por `input::demo_action`, que es la única lista
        // de qué hace cada una. Aquí solo se traduce de `minifb::Key` al
        // carácter que esa lista entiende.
        for (tecla, caracter) in [
            (Key::Key1, '1'),
            (Key::Key2, '2'),
            (Key::Key3, '3'),
            (Key::L, 'L'),
            (Key::R, 'R'),
        ] {
            if window.is_key_pressed(tecla, KeyRepeat::No) {
                acciones.extend(demo_action(caracter));
            }
        }

        let intent = FrameIntent::from_actions(acciones);
        let mut reinicio = false;

        if intent.reset_canvas {
            reveal = RevealState::unpainted();
            reinicio = true;

            // `L` es un reinicio completo: sin borrar las máscaras, el
            // lienzo volvería con lo pintado a mano todavía revelado.
            #[cfg(feature = "artistic-brush")]
            {
                masks.clear();
                pigmentos.clear();
                trazo = None;
            }

            println!("  reiniciado al lienzo");
        }

        if intent.reset_camera {
            // El encuadre lo aporta la escena, no esta tecla.
            camera.restore(hero);
            reinicio = true;

            println!("  encuadre hero restaurado");
        }

        // `paints` ya respeta la precedencia: con un reinicio en el mismo
        // cuadro, la lista viene vacía.
        let mut region_cambio = false;

        for grupo in intent.paints() {
            // Activar, no saltar: el avance lo hace el reloj más abajo.
            if reveal.activate(grupo) {
                println!("  pintando {grupo:?}");
                region_cambio = true;
            }
        }

        // ------------------------------------------------ avance por reloj
        //
        // Por tiempo de pared y no por cuadros: una máquina lenta termina la
        // transición en aproximadamente el mismo tiempo, con menos cuadros.
        let ahora = Instant::now();
        let delta = ahora.duration_since(ultimo_cuadro).as_secs_f32();
        ultimo_cuadro = ahora;

        let revelando = reveal.advance(delta, velocidad);

        // ------------------------------------------------ dirty rendering
        //
        // Un cambio **sostenido** es el que va a seguir cambiando el cuadro
        // siguiente: mientras dura, se dibuja barato. Un cambio instantáneo
        // ya terminó, así que no tiene sentido gastarle un cuadro de baja
        // resolución: se marca el cuadro final y se dibuja bien de una vez.
        let sostenido = camara_cambio || zoom_cambio || revelando || region_cambio;

        // Pintar es un cambio sostenido: mientras el botón siga abajo el
        // cuadro va a seguir cambiando, así que se dibuja barato.
        #[cfg(feature = "artistic-brush")]
        let sostenido = sostenido || pincel_cambio;

        // Un cambio instantáneo —el reinicio— no dibuja barato: solo deja
        // pendiente el cuadro definitivo.
        if sostenido || reinicio {
            cuadro_final_pendiente = true;
        }

        // El pincel visual desapareció: hay que volver a trazar para
        // borrarlo. Sin esto, `FramePlan::Reuse` presentaría otra vez el
        // framebuffer con el gizmo ya pintado encima y se quedaría clavado.
        #[cfg(feature = "artistic-brush")]
        if gizmo_anterior && gizmo.is_none() {
            cuadro_final_pendiente = true;
        }

        // La interfaz cambió: hay que volver a trazar para que el panel
        // aparezca o desaparezca. Sin esto, `FramePlan::Reuse` presentaría
        // otra vez el framebuffer anterior con la paleta congelada.
        #[cfg(feature = "artistic-brush")]
        if paleta_cambio {
            cuadro_final_pendiente = true;
        }

        // La decisión vive en `renderer::plan_frame`, con su tabla probada.
        match plan_frame(sostenido, cuadro_final_pendiente) {
            FramePlan::Interactive => {
                // Cuadro barato: se traza a la resolución del perfil y se
                // escala al tamaño de la ventana.
                dibujar!(&mut borrador);
                framebuffer.blit_upscaled(&borrador);
                presentado = PresentedFrame {
                    camera,
                    source: (perfil.width, perfil.height),
                    window: (WIDTH, HEIGHT),
                };
            }
            FramePlan::Final => {
                // Todo quieto: una sola pasada a resolución completa.
                dibujar!(&mut framebuffer);
                cuadro_final_pendiente = false;
                presentado = PresentedFrame::full(camera, (WIDTH, HEIGHT));
            }
            // Nada cambió: se reutiliza el framebuffer sin trazar.
            FramePlan::Reuse => {}
        }

        // El pincel visual se dibuja **después** del render y del escalado,
        // sobre el framebuffer completo y con la cámara del cuadro que se va
        // a presentar. No entra en la escena ni cuesta un rayo: son cuatro
        // puntos proyectados y tres segmentos.
        #[cfg(feature = "artistic-brush")]
        {
            if let Some((punto, normal, cerdas)) = gizmo {
                // La madera se **presta**: ni se carga por cuadro ni se
                // modifica. Si faltara, el gizmo sale con un mango plano en
                // vez de no salir.
                draw_brush_gizmo(
                    &mut framebuffer,
                    &presentado.camera,
                    &punto,
                    &normal,
                    brush_radius,
                    cerdas,
                    scene.textures.get(TEXTURA_DEL_MANGO),
                );
            }

            gizmo_anterior = gizmo.is_some();

            // La paleta va la **última**: es interfaz, y tiene que leerse
            // por encima del diorama y del propio pincel.
            dibujar_paleta(&mut framebuffer, &disposicion, &paleta, &scene.textures);
        }

        // `update_with_buffer` va siempre, también cuando no se dibujó: es
        // lo que bombea los eventos de la ventana. Cuando nada cambia
        // presenta el mismo framebuffer otra vez, que es la reutilización
        // que pide el plan.
        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();

        // El descanso es **solo para el ciclo en reposo**; ver
        // `FramePlan::should_sleep`.
        if plan_frame(sostenido, cuadro_final_pendiente).should_sleep() {
            std::thread::sleep(frame_delay);
        }
    }

    ExitCode::SUCCESS
}
