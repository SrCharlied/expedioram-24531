mod camera;
mod color;
mod framebuffer;
mod ray_intersect;
mod sphere;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{normalize, Vec3};
use std::f32::consts::PI;
use std::time::Duration;

use crate::camera::Camera;
use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::ray_intersect::{Material, RayIntersect};
use crate::sphere::Sphere;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const BACKGROUND_COLOR: u32 = 0x040C24;

/// Campo de visión vertical. Las etapas anteriores lo tenían implícito en
/// 90 grados por poner el plano de proyección a una unidad de distancia.
const FOV: f32 = PI / 3.0;

/// Cuánto gira la cámara por cuadro mientras se sostiene una flecha.
const ROTATION_SPEED: f32 = PI / 60.0;

/// Devuelve el color del objeto más cercano que toca el rayo.
pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere]) -> Color {
    let mut closest: Option<f32> = None;
    let mut color = Color::from_hex(BACKGROUND_COLOR);

    for object in objects {
        if let Some(intersect) = object.ray_intersect(ray_origin, ray_direction) {
            if closest.is_none_or(|distance| intersect.distance < distance) {
                closest = Some(intersect.distance);
                color = intersect.material.diffuse;
            }
        }
    }

    color
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Sphere], camera: &Camera) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    // Media altura del plano de proyección, que está a una unidad de la
    // cámara. Abrir el campo de visión ensancha el plano.
    let perspective_scale = (FOV / 2.0).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // De coordenadas de píxel a coordenadas de pantalla, de -1 a 1.
            // La y se invierte porque el píxel 0 está arriba y el eje Y
            // del mundo crece hacia arriba.
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            // El rayo nace en coordenadas de cámara —viendo hacia -Z— y el
            // cambio de base lo lleva al mundo, donde están los objetos.
            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));
            let ray_direction = camera.basis_change(&ray_direction);

            framebuffer.set_current_color(cast_ray(&camera.eye, &ray_direction, objects).to_hex());
            framebuffer.point(x, y);
        }
    }
}

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
