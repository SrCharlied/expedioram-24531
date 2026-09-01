use crate::color::Color;
use crate::hit::Hit;
use crate::ray::Ray;

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

/// Lo que sabe hacer una primitiva trazable: decir si un rayo la toca y,
/// si lo hace, describir el impacto.
///
/// La respuesta es «no tocó» o «tocó, y esto es lo que hay ahí», que en
/// Rust es exactamente un `Option`: no hace falta una bandera
/// `is_intersecting` ni un impacto vacío con material de mentira.
pub trait RayIntersect {
    fn ray_intersect(&self, ray: &Ray) -> Option<Hit>;
}
