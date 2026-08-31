use crate::ray_intersect::{Intersect, Material, RayIntersect};
use nalgebra_glm::{dot, Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        // Sustituir el rayo `origen + t · dirección` en la ecuación de la
        // esfera deja una cuadrática en t: a·t² + b·t + c = 0.
        let oc = ray_origin - self.center;

        let a = dot(ray_direction, ray_direction);
        let b = 2.0 * dot(&oc, ray_direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant <= 0.0 {
            return None;
        }

        // De las dos soluciones, la del signo negativo es la más pequeña:
        // el punto por donde el rayo entra a la esfera.
        let t = (-b - discriminant.sqrt()) / (2.0 * a);

        // Una t negativa significa que la esfera quedó detrás de la cámara.
        // La recta sí la cruza, el rayo no.
        if t <= 0.0 {
            return None;
        }

        let point = ray_origin + ray_direction * t;

        // La normal de una esfera es trivial: apunta del centro hacia el
        // punto de impacto. Se normaliza para que mida 1.
        let normal = (point - self.center).normalize();

        Some(Intersect {
            point,
            normal,
            distance: t,
            material: self.material,
        })
    }
}
