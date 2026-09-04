//! Ciclo de ventana del diorama.
//!
//! Solo tres responsabilidades: abrir la ventana, leer el teclado y
//! presentar el framebuffer. Todo lo que se puede probar sin ventana vive
//! en la librería del paquete.

use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window, WindowOptions};
use std::f32::consts::PI;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::{Duration, Instant};

use expedition33_continente_inacabado::framebuffer::Framebuffer;
use expedition33_continente_inacabado::input::{
    demo_action, pick_region, DemoAction, FrameIntent, PresentedFrame,
};
use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
use expedition33_continente_inacabado::renderer::{
    plan_frame, render, FramePlan, InteractiveProfile, Shading,
};
use expedition33_continente_inacabado::reveal::{reveal_duration, reveal_speed, RevealState};
use expedition33_continente_inacabado::scenes::{safe_level_con, WaterPreset};

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
    // canónico desde la Tarea 5.4: 160 primitivas, techos `0.9 / 0.9` e
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

    // Arranca con todo pintado: el picking de la Tarea 6.2 y la
    // temporizacion de la 6.3 son las que lo vuelven interactivo. Mostrar el
    // lienzo entero ahora dejaria una ventana de un solo color.
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
    // Se mide con `reveal 1.0`, que es el caso caro: en lienzo los techos
    // del agua están interpolados desde cero y no se lanza un solo rayo
    // secundario. Garantizar los quince cuadros con el tiempo del lienzo
    // dejaría el final de la transición sin margen.
    let mut calibracion = Vec::with_capacity(CUADROS_DE_CALIBRACION);

    for _ in 0..CUADROS_DE_CALIBRACION {
        let inicio = Instant::now();
        render(
            &mut borrador,
            &scene,
            &accel,
            &lights,
            &RevealState::painted(),
            &camera,
            shading,
        );
        calibracion.push(inicio.elapsed().as_secs_f32());
    }

    calibracion.sort_by(|a, b| a.partial_cmp(b).expect("no hay NaN"));
    let frame_time = calibracion[calibracion.len() / 2];

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
    println!("  clic     pintar la region señalada     1 / 2 / 3  pintar por teclado");
    println!("  L        volver al lienzo     R  restaurar encuadre hero");
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
    let mut boton_anterior = false;

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

        // Un cambio instantáneo —el reinicio— no dibuja barato: solo deja
        // pendiente el cuadro definitivo.
        if sostenido || reinicio {
            cuadro_final_pendiente = true;
        }

        // La decisión vive en `renderer::plan_frame`, con su tabla probada.
        match plan_frame(sostenido, cuadro_final_pendiente) {
            FramePlan::Interactive => {
                // Cuadro barato: se traza a la resolución del perfil y se
                // escala al tamaño de la ventana.
                render(
                    &mut borrador,
                    &scene,
                    &accel,
                    &lights,
                    &reveal,
                    &camera,
                    shading,
                );
                framebuffer.blit_upscaled(&borrador);
                presentado = PresentedFrame {
                    camera,
                    source: (perfil.width, perfil.height),
                    window: (WIDTH, HEIGHT),
                };
            }
            FramePlan::Final => {
                // Todo quieto: una sola pasada a resolución completa.
                render(
                    &mut framebuffer,
                    &scene,
                    &accel,
                    &lights,
                    &reveal,
                    &camera,
                    shading,
                );
                cuadro_final_pendiente = false;
                presentado = PresentedFrame::full(camera, (WIDTH, HEIGHT));
            }
            // Nada cambió: se reutiliza el framebuffer sin trazar.
            FramePlan::Reuse => {}
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
