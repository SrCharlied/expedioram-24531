//! Trazado de la escena: de un píxel a un color.
//!
//! Estaba en `main.rs` junto al ciclo de ventana. Vive aparte porque es la
//! parte que hay que poder ejecutar sin abrir una ventana, tanto en tests
//! como en el render headless que exige el plan.

use crate::camera::Camera;
use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::ray_intersect::RayIntersect;
use crate::sphere::Sphere;
use nalgebra_glm::{normalize, Vec3};
use std::f32::consts::PI;

/// Color que devuelve un rayo que no toca nada.
pub const BACKGROUND_COLOR: u32 = 0x040C24;

/// Campo de visión vertical. Las etapas anteriores lo tenían implícito en
/// 90 grados por poner el plano de proyección a una unidad de distancia.
pub const FOV: f32 = PI / 3.0;

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
