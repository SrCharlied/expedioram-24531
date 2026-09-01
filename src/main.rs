//! Ciclo de ventana del diorama.
//!
//! Solo tres responsabilidades: abrir la ventana, leer el teclado y
//! presentar el framebuffer. Todo lo que se puede probar sin ventana vive
//! en la librería del paquete.

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec3;
use std::f32::consts::PI;
use std::time::Duration;

use expedition33_continente_inacabado::camera::Camera;
use expedition33_continente_inacabado::color::Color;
use expedition33_continente_inacabado::framebuffer::Framebuffer;
use expedition33_continente_inacabado::ray_intersect::Material;
use expedition33_continente_inacabado::renderer::render;
use expedition33_continente_inacabado::sphere::Sphere;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

/// Cuánto gira la cámara por cuadro mientras se sostiene una flecha.
const ROTATION_SPEED: f32 = PI / 60.0;

fn main() {
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new("Lakitu", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let ivory = Material::new(Color::new(100, 100, 80));
    let rubber = Material::new(Color::new(80, 0, 0));
    let cobalt = Material::new(Color::new(40, 80, 140));

    // La escena se acomoda alrededor del origen, que es el punto que la
    // cámara orbita. Las esferas están a distintas profundidades para que
    // al girar se vea cuál pasa frente a cuál.
    let objects = [
        Sphere {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 1.0,
            material: ivory,
        },
        Sphere {
            center: Vec3::new(1.8, 0.0, -0.8),
            radius: 0.5,
            material: rubber,
        },
        Sphere {
            center: Vec3::new(-1.2, 0.6, 1.0),
            radius: 0.6,
            material: cobalt,
        },
    ];

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

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

        if camera_moved {
            render(&mut framebuffer, &objects, &camera);
            camera_moved = false;
        }

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
