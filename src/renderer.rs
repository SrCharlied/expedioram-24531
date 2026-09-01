//! Trazado de la escena: de un píxel a un color.
//!
//! Estaba en `main.rs` junto al ciclo de ventana. Vive aparte porque es la
//! parte que hay que poder ejecutar sin abrir una ventana, tanto en tests
//! como en el render headless que exige el plan.

use crate::camera::Camera;
use crate::color::Color;
use crate::cuboid::Cuboid;
use crate::framebuffer::Framebuffer;
use crate::hit::Hit;
use crate::ray::Ray;
use crate::ray_intersect::RayIntersect;
use nalgebra_glm::{normalize, Vec3};
use std::f32::consts::PI;

/// Color que devuelve un rayo que no toca nada.
pub const BACKGROUND_COLOR: u32 = 0x040C24;

/// Campo de visión vertical. Las etapas anteriores lo tenían implícito en
/// 90 grados por poner el plano de proyección a una unidad de distancia.
pub const FOV: f32 = PI / 3.0;

/// Impacto más cercano del rayo contra la escena, con `object_index` ya
/// asignado.
///
/// La primitiva no sabe en qué posición de la escena vive, así que el
/// índice lo pone este recorrido. Es lo que después permite resolver el
/// material sin haberlo copiado dentro del impacto.
pub fn closest_hit(ray: &Ray, objects: &[Cuboid]) -> Option<Hit> {
    let mut closest: Option<Hit> = None;

    for (index, object) in objects.iter().enumerate() {
        if let Some(mut hit) = object.ray_intersect(ray) {
            if closest.is_none_or(|previo| hit.distance < previo.distance) {
                hit.object_index = index;
                closest = Some(hit);
            }
        }
    }

    closest
}

/// Traduce una normal a color, llevando el rango `-1.0..=1.0` de cada
/// componente a `0.0..=1.0`.
///
/// Es una vista de depuración, no sombreado: sirve para verificar a simple
/// vista que las seis caras del cuboide miran hacia donde deben. Cada eje
/// se ve como un color distinto, así que una cara mal orientada salta de
/// inmediato.
pub fn color_por_normal(hit: &Hit) -> Color {
    Color::new(
        hit.normal.x * 0.5 + 0.5,
        hit.normal.y * 0.5 + 0.5,
        hit.normal.z * 0.5 + 0.5,
    )
}

/// Devuelve el color del objeto más cercano que toca el rayo.
///
/// Todavía colorea por normal: el cuboide es geometría pura y el material
/// no le pertenece, sino al `SceneObject` que lo envuelve en la Tarea 1.6.
pub fn cast_ray(ray: &Ray, objects: &[Cuboid]) -> Color {
    match closest_hit(ray, objects) {
        Some(hit) => color_por_normal(&hit),
        None => Color::from_hex(BACKGROUND_COLOR),
    }
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Cuboid], camera: &Camera) {
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
            let ray = Ray::new(camera.eye, camera.basis_change(&ray_direction));

            framebuffer.set_current_color(cast_ray(&ray, objects).to_hex());
            framebuffer.point(x, y);
        }
    }
}
