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
use expedition33_continente_inacabado::input::{demo_region, pick_region};
use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
use expedition33_continente_inacabado::renderer::{render, InteractiveProfile, Shading};
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

/// Tiempo por cuadro del perfil interactivo, **medido**.
///
/// `400 x 300`, preset refractivo y `reveal 1.0` —el caso más caro, porque
/// el lienzo no lanza rayos secundarios y cuesta `0.0347 s—`, mediana de
/// quince repeticiones en release.
///
/// No se hereda del Hito 3: aquella medición dio `0.0242 s` y fue **antes**
/// de la óptica, que duplicó el costo. De aquí sale la duración de la
/// revelación; medirlo mal alargaría o acortaría la animación sin que nada
/// avisara.
const INTERACTIVE_FRAME_TIME: f32 = 0.0490;

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

    // Nivel seguro con el interior de la bahía visible.
    //
    // El volumen de agua no se inserta: hasta que el Hito 5 traiga
    // refracción, un cuboide azul opaco taparía las 44 primitivas del
    // interior —barco, mástil, cadena, ancla, kelp y rocas— y el plan
    // prohíbe expresamente fingir transparencia sin óptica. Mostrar el
    // interior es también lo que hace útil la ventana mientras se trabajan
    // los materiales de los Hitos 4 a 6.
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

    // Duración de la revelación, **derivada** del tiempo por cuadro medido
    // en el perfil interactivo. No se elige: sale de la medición, con piso
    // de 1.5 s y techo de 4.0 s. Si el perfil no diera para quince cuadros
    // dentro del techo, esto aborta en vez de alargar la animación, que es
    // lo que el plan prohíbe expresamente.
    let duracion = match reveal_duration(INTERACTIVE_FRAME_TIME) {
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

    // A 800 x 600 el nivel seguro cuesta ~0.096 s por cuadro: orbitar a
    // 10 fps se siente pegajoso. Mientras algo se mueve se dibuja en el
    // perfil interactivo y se escala; al soltar los controles se produce
    // un cuadro final a resolución completa.
    let perfil = InteractiveProfile::default();
    let mut borrador = Framebuffer::new(perfil.width, perfil.height);

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
    println!("  R        volver al lienzo");
    println!(
        "  revelado {duracion:.2} s por region, derivados de {INTERACTIVE_FRAME_TIME:.4} s por cuadro"
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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let orbit = [
            (Key::Left, ROTATION_SPEED, 0.0),
            (Key::Right, -ROTATION_SPEED, 0.0),
            (Key::Up, 0.0, -ROTATION_SPEED),
            (Key::Down, 0.0, ROTATION_SPEED),
        ];

        let mut en_movimiento = false;

        for (key, delta_yaw, delta_pitch) in orbit {
            if window.is_key_down(key) {
                camera.orbit(delta_yaw, delta_pitch);
                en_movimiento = true;
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

        if pasos != 0.0 {
            camera.zoom(pasos * ZOOM_FRACTION * camera.radius());
            en_movimiento = true;
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
        let boton = window.get_mouse_down(MouseButton::Left);
        let clic = boton && !boton_anterior;
        boton_anterior = boton;

        let mut elegida = None;

        if clic {
            if let Some(cursor) = window.get_mouse_pos(MouseMode::Discard) {
                elegida = pick_region(&scene, &accel, &camera, cursor, WIDTH, HEIGHT);
            }
        }

        for (tecla, digito) in [(Key::Key1, 1), (Key::Key2, 2), (Key::Key3, 3)] {
            if window.is_key_pressed(tecla, KeyRepeat::No) {
                elegida = demo_region(digito);
            }
        }

        // Reiniciar al lienzo. No está en la lista del plan, y se añade por
        // la misma razón que existe el fallback de teclado: una
        // presentación que solo se puede dar una vez por arranque no es
        // fiable. Con `R` la revelación se puede mostrar de nuevo sin
        // cerrar la ventana.
        if window.is_key_pressed(Key::R, KeyRepeat::No) {
            reveal = RevealState::unpainted();
            cuadro_final_pendiente = true;

            println!("  reiniciado al lienzo");
        }

        if let Some(grupo) = elegida {
            // Activar, no saltar: el avance lo hace el reloj más abajo.
            if reveal.activate(grupo) {
                println!("  pintando {grupo:?}");
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

        if revelando {
            // Mientras algo se revela, el cuadro cambia: se dibuja al perfil
            // interactivo igual que al orbitar, y el cuadro final llega al
            // quedarse todo quieto.
            en_movimiento = true;
        }

        if en_movimiento {
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
            cuadro_final_pendiente = true;
        } else if cuadro_final_pendiente {
            // Todo quieto: una sola pasada a resolución completa. Mientras
            // nada cambie se reutiliza el framebuffer, como en la rama del
            // profesor.
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
        }

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();

        std::thread::sleep(frame_delay);
    }

    ExitCode::SUCCESS
}
