use crate::color::Color;
use nalgebra_glm::Vec3;

/// Propiedades de la superficie de un objeto. Por ahora solo el color
/// difuso; la reflexión, la refracción y el brillo especular se agregan
/// en las etapas siguientes.
#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
}

impl Material {
    pub fn new(diffuse: Color) -> Self {
        Material { diffuse }
    }
}

/// Todo lo que se sabe de un impacto. `point` y `normal` no se usan
/// todavía para colorear, pero son la base de la iluminación: la normal
/// dice hacia dónde ve la superficie y el punto dice desde dónde se lanza
/// el rayo hacia la luz.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)] // `point` y `normal` entran en uso con la iluminación.
pub struct Intersect {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub material: Material,
}

/// La etapa anterior contestaba `bool`. Ahora la respuesta es «no tocó» o
/// «tocó, y esto es lo que hay ahí», que en Rust es exactamente un
/// `Option`: no hace falta una bandera `is_intersecting` ni un impacto
/// vacío con material de mentira.
pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect>;
}
