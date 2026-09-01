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

/// Color que devuelve un rayo que no toca nada.
pub const BACKGROUND_COLOR: u32 = 0x040C24;

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
    let (ancho, alto) = (framebuffer.width, framebuffer.height);

    for y in 0..alto {
        for x in 0..ancho {
            // La generación del rayo vive en la cámara: el picking del Hito
            // 6 tiene que usar exactamente la misma función para que un clic
            // caiga en el píxel que se ve.
            let ray = camera.ray_from_pixel(x, y, ancho, alto);

            framebuffer.set_current_color(cast_ray(&ray, scene, shading).to_hex());
            framebuffer.point(x, y);
        }
    }
}
