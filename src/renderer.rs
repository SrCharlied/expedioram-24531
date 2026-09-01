//! Trazado de la escena: de un píxel a un color.
//!
//! Estaba en `main.rs` junto al ciclo de ventana. Vive aparte porque es la
//! parte que hay que poder ejecutar sin abrir una ventana, tanto en tests
//! como en el render headless que exige el plan.

use crate::camera::Camera;
use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::hit::Hit;
use crate::ray::Ray;
use crate::scene::Scene;
use nalgebra_glm::{normalize, Vec3};
use std::f32::consts::PI;

/// Color que devuelve un rayo que no toca nada.
pub const BACKGROUND_COLOR: u32 = 0x040C24;

/// Campo de visión vertical. Las etapas anteriores lo tenían implícito en
/// 90 grados por poner el plano de proyección a una unidad de distancia.
pub const FOV: f32 = PI / 3.0;

/// Cómo se resuelve el color de un impacto.
///
/// `Normals` no es sombreado sino una vista de depuración: cada eje se ve
/// de un color distinto, así que una cara mal orientada salta a simple
/// vista. Es lo que hace verificable el cubo mientras no haya luces —hasta
/// el Hito 3 un color plano solo daría una silueta—, y sigue sirviendo
/// después para revisar geometría nueva.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Shading {
    #[default]
    Material,
    Normals,
}

/// Traduce una normal a color, llevando el rango `-1.0..=1.0` de cada
/// componente a `0.0..=1.0`.
pub fn color_por_normal(hit: &Hit) -> Color {
    Color::new(
        hit.normal.x * 0.5 + 0.5,
        hit.normal.y * 0.5 + 0.5,
        hit.normal.z * 0.5 + 0.5,
    )
}

/// Devuelve el color del objeto más cercano que toca el rayo.
///
/// El material se resuelve por `object_index` contra la paleta de la
/// escena; el impacto nunca lo carga. Por ahora usa `final_material`
/// directamente: la interpolación desde `canvas_unpainted` llega en la
/// Tarea 4.4, cuando exista `RevealState`.
pub fn cast_ray(ray: &Ray, scene: &Scene, shading: Shading) -> Color {
    let Some(hit) = scene.intersect(ray) else {
        return Color::from_hex(BACKGROUND_COLOR);
    };

    match shading {
        Shading::Normals => color_por_normal(&hit),
        Shading::Material => {
            let objeto = scene.objects[hit.object_index];

            scene.material(objeto.final_material).diffuse
        }
    }
}

pub fn render(framebuffer: &mut Framebuffer, scene: &Scene, camera: &Camera, shading: Shading) {
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

            framebuffer.set_current_color(cast_ray(&ray, scene, shading).to_hex());
            framebuffer.point(x, y);
        }
    }
}
