//! Ciclo de ventana del diorama.
//!
//! Solo tres responsabilidades: abrir la ventana, leer el teclado y
//! presentar el framebuffer. Todo lo que se puede probar sin ventana vive
//! en la librería del paquete.

use minifb::{Key, Window, WindowOptions};
use std::f32::consts::PI;
use std::time::Duration;

use expedition33_continente_inacabado::framebuffer::Framebuffer;
use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
use expedition33_continente_inacabado::renderer::{render, Shading};
use expedition33_continente_inacabado::scenes::continent::blockout;

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

fn main() {
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new("Lakitu", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let diorama = blockout();
    let lights = luces_del_diorama(&diorama.anchors, &diorama.scale);

    // Ya hay luces: el sombreado completo dice mas que el albedo plano.
    // `Shading::Albedo` reproduce las imagenes con las que se aprobo el
    // Blockout 1, y `Normals` sigue disponible para revisar geometria.
    let shading = Shading::Material;

    // El encuadre queda por encima del eje de órbita, y el radio sale de la
    // escala medida: ni la altura ni la distancia se eligieron a mano.
    let mut camera = diorama.hero_camera();

    // La cámara se construye antes de consumir el blockout: `hero_camera`
    // necesita las anclas y la escala, que viven junto a la escena.
    let scene = diorama.scene;

    // Renderizar cuesta 480 000 rayos. Mientras la cámara esté quieta la
    // imagen es la misma, así que solo se vuelve a calcular cuando algo
    // cambió; el primer cuadro cuenta como cambio.
    let mut camera_moved = true;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let orbit = [
            (Key::Left, ROTATION_SPEED, 0.0),
            (Key::Right, -ROTATION_SPEED, 0.0),
            (Key::Up, 0.0, -ROTATION_SPEED),
            (Key::Down, 0.0, ROTATION_SPEED),
        ];

        for (key, delta_yaw, delta_pitch) in orbit {
            if window.is_key_down(key) {
                camera.orbit(delta_yaw, delta_pitch);
                camera_moved = true;
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
            camera_moved = true;
        }

        if camera_moved {
            render(&mut framebuffer, &scene, &lights, &camera, shading);
            camera_moved = false;
        }

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
