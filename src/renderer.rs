//! Trazado de la escena: de un píxel a un color.
//!
//! Estaba en `main.rs` junto al ciclo de ventana. Vive aparte porque es la
//! parte que hay que poder ejecutar sin abrir una ventana, tanto en tests
//! como en el render headless que exige el plan.

use crate::camera::Camera;
use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::hit::Hit;
use crate::light::PointLight;
use crate::material::{direct_light, AMBIENT};
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
    /// Iluminación completa: ambiente, difusa y specular directa.
    #[default]
    Material,
    /// Albedo plano, sin luces. Es con lo que se juzgó la composición del
    /// Blockout 1, y sigue sirviendo para reproducir aquellas imágenes.
    Albedo,
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
pub fn cast_ray(ray: &Ray, scene: &Scene, lights: &[PointLight], shading: Shading) -> Color {
    let Some(hit) = scene.intersect(ray) else {
        return Color::from_hex(BACKGROUND_COLOR);
    };

    let objeto = scene.objects[hit.object_index];
    let material = scene.material(objeto.final_material);

    match shading {
        Shading::Normals => color_por_normal(&hit),
        Shading::Albedo => material.albedo,
        Shading::Material => {
            // Ambiente: no es física, es el suelo que impide que lo no
            // iluminado quede en negro absoluto y pierda su silueta.
            let mut color = material.albedo * AMBIENT;

            // El ojo, no la luz: el specular depende de desde dónde se mira.
            let hacia_ojo = -ray.direction;

            for light in lights {
                // Light linking, antes de calcular nada: si esta luz no
                // ilumina a este grupo, no cuesta ni una operación más.
                if !light.affects(objeto.spatial_group) {
                    continue;
                }

                let hacia_luz = light.position - hit.point;
                let distancia = hacia_luz.magnitude();

                if distancia <= f32::EPSILON {
                    continue;
                }

                let atenuacion = light.attenuation(distancia);
                if atenuacion <= 0.0 {
                    continue;
                }

                // Las sombras llegan en la Tarea 3.6; aquí toda luz que
                // alcanza el punto lo ilumina.
                color = color
                    + direct_light(
                        &material,
                        &hit.normal,
                        &(hacia_luz / distancia),
                        &hacia_ojo,
                        light.color,
                        atenuacion,
                    );
            }

            color
        }
    }
}

pub fn render(
    framebuffer: &mut Framebuffer,
    scene: &Scene,
    lights: &[PointLight],
    camera: &Camera,
    shading: Shading,
) {
    let (ancho, alto) = (framebuffer.width, framebuffer.height);

    for y in 0..alto {
        for x in 0..ancho {
            // La generación del rayo vive en la cámara: el picking del Hito
            // 6 tiene que usar exactamente la misma función para que un clic
            // caiga en el píxel que se ve.
            let ray = camera.ray_from_pixel(x, y, ancho, alto);

            framebuffer.set_current_color(cast_ray(&ray, scene, lights, shading).to_hex());
            framebuffer.point(x, y);
        }
    }
}
