//! Ciclo de ventana del diorama.
//!
//! Solo tres responsabilidades: abrir la ventana, leer el teclado y
//! presentar el framebuffer. Todo lo que se puede probar sin ventana vive
//! en la librería del paquete.

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec3;
use std::f32::consts::PI;
use std::time::Duration;

use expedition33_continente_inacabado::camera::{Camera, DEFAULT_VERTICAL_FOV};
use expedition33_continente_inacabado::color::Color;
use expedition33_continente_inacabado::cuboid::Cuboid;
use expedition33_continente_inacabado::framebuffer::Framebuffer;
use expedition33_continente_inacabado::ray_intersect::Material;
use expedition33_continente_inacabado::renderer::{render, Shading};
use expedition33_continente_inacabado::scene::{RevealGroup, Scene, SceneObject, SpatialGroupId};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

/// Cuánto gira la cámara por cuadro mientras se sostiene una flecha.
const ROTATION_SPEED: f32 = PI / 60.0;

/// Escena de verificación del Hito 1: un cuboide centrado en el origen.
///
/// No es todavía el diorama. Su función es que el gate del hito sea
/// comprobable a simple vista: al orbitar, cada cara debe conservar su
/// color y su orientación.
fn escena_de_prueba() -> Scene {
    let mut scene = Scene::new();

    let piedra = scene.add_material(Material::new(Color::new(0.62, 0.60, 0.55)));

    scene.add_object(SceneObject {
        primitive: Cuboid::centrado(Vec3::zeros(), Vec3::new(2.0, 2.0, 2.0)).into(),
        initial_material: piedra,
        final_material: piedra,
        spatial_group: SpatialGroupId::Monolith,
        reveal_group: RevealGroup::Finale,
    });

    scene
}

fn main() {
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new("Lakitu", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let scene = escena_de_prueba();

    // Hasta que existan luces, en el Hito 3, un color plano solo daría una
    // silueta. La vista por normales es lo que permite verificar que las
    // seis caras miran hacia donde deben.
    let shading = Shading::Normals;

    // El eje de órbita y el punto de encuadre ya son independientes. En la
    // escena de prueba todavía coinciden en el origen; el Blockout 1 los
    // separa de verdad, con el encuadre por encima de la base del Monolito.
    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::zeros(),
        Vec3::zeros(),
        Vec3::new(0.0, 1.0, 0.0),
        DEFAULT_VERTICAL_FOV,
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
            render(&mut framebuffer, &scene, &camera, shading);
            camera_moved = false;
        }

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
