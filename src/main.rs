//! Ciclo de ventana del diorama.
//!
//! Solo tres responsabilidades: abrir la ventana, leer el teclado y
//! presentar el framebuffer. Todo lo que se puede probar sin ventana vive
//! en la librería del paquete.

use minifb::{Key, Window, WindowOptions};
use std::f32::consts::PI;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

use expedition33_continente_inacabado::framebuffer::Framebuffer;
use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
use expedition33_continente_inacabado::renderer::{render, InteractiveProfile, Shading};
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
    let diorama = match safe_level_con(WaterPreset::InteriorVisible, Some(&raiz)) {
        Ok(diorama) => diorama,
        Err(e) => {
            eprintln!("error: {e}");
            eprintln!("  genera los assets con: cargo run --release --bin generate_assets");
            return ExitCode::FAILURE;
        }
    };
    let lights = luces_del_diorama(&diorama.anchors, &diorama.scale);

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

    // El primer cuadro cuenta como cambio pendiente, para que la ventana
    // arranque ya con la imagen definitiva.
    let mut cuadro_final_pendiente = true;

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

        if en_movimiento {
            // Cuadro barato: se traza a la resolución del perfil y se
            // escala al tamaño de la ventana.
            render(&mut borrador, &scene, &accel, &lights, &camera, shading);
            framebuffer.blit_upscaled(&borrador);
            cuadro_final_pendiente = true;
        } else if cuadro_final_pendiente {
            // Todo quieto: una sola pasada a resolución completa. Mientras
            // nada cambie se reutiliza el framebuffer, como en la rama del
            // profesor.
            render(&mut framebuffer, &scene, &accel, &lights, &camera, shading);
            cuadro_final_pendiente = false;
        }

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();

        std::thread::sleep(frame_delay);
    }

    ExitCode::SUCCESS
}
