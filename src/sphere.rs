use crate::hit::Hit;
use crate::ray::Ray;
use crate::ray_intersect::{Material, RayIntersect};
use crate::EPSILON;
use nalgebra_glm::{dot, Vec2, Vec3};
use std::f32::consts::PI;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray: &Ray) -> Option<Hit> {
        // Sustituir el rayo `origen + t · dirección` en la ecuación de la
        // esfera deja una cuadrática en t: a·t² + b·t + c = 0.
        let oc = ray.origin - self.center;

        let a = dot(&ray.direction, &ray.direction);
        let b = 2.0 * dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant <= 0.0 {
            return None;
        }

        let raiz = discriminant.sqrt();

        // De las dos soluciones, la del signo negativo es la más cercana.
        // Si esa quedó detrás del origen —o demasiado pegada, que es el
        // autoimpacto— se prueba la otra: eso ocurre cuando el rayo nace
        // dentro de la esfera y solo puede salir por la cara lejana.
        let mut t = (-b - raiz) / (2.0 * a);
        if t <= EPSILON {
            t = (-b + raiz) / (2.0 * a);
        }
        if t <= EPSILON {
            return None;
        }

        let point = ray.at(t);

        // La normal exterior de una esfera es trivial: apunta del centro
        // hacia el punto de impacto. `Hit::new` decide si hay que voltearla.
        let outward_normal = (point - self.center) / self.radius;

        Some(Hit::new(
            ray,
            t,
            outward_normal,
            uv_esferica(&outward_normal),
        ))
    }
}

/// Coordenadas equirectangulares sobre la esfera: longitud en `u`, latitud
/// en `v`, ambas en `0.0..=1.0`.
fn uv_esferica(normal: &Vec3) -> Vec2 {
    let u = 0.5 + normal.z.atan2(normal.x) / (2.0 * PI);
    let v = 0.5 - normal.y.clamp(-1.0, 1.0).asin() / PI;

    Vec2::new(u, v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    fn esfera_unitaria() -> Sphere {
        Sphere {
            center: Vec3::zeros(),
            radius: 1.0,
            material: Material::new(Color::new(1.0, 1.0, 1.0)),
        }
    }

    #[test]
    fn impacto_frontal_devuelve_cara_exterior() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = esfera_unitaria()
            .ray_intersect(&ray)
            .expect("debe impactar");

        assert!((hit.distance - 4.0).abs() < 1e-5);
        assert!(hit.front_face);
    }

    #[test]
    fn esfera_detras_del_origen_no_impacta() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 1.0));

        assert!(esfera_unitaria().ray_intersect(&ray).is_none());
    }

    #[test]
    fn rayo_nacido_adentro_sale_por_la_cara_lejana() {
        let ray = Ray::new(Vec3::zeros(), Vec3::new(0.0, 0.0, -1.0));
        let hit = esfera_unitaria().ray_intersect(&ray).expect("debe salir");

        assert!((hit.distance - 1.0).abs() < 1e-5);
        assert!(!hit.front_face, "se golpea desde adentro");
        // La normal se volteó para quedar contra el rayo.
        assert!(dot(&hit.normal, &ray.direction) < 0.0);
    }

    #[test]
    fn uv_permanece_en_rango_unitario() {
        for direccion in [
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 1.0).normalize(),
        ] {
            let ray = Ray::new(direccion * -5.0, direccion);
            let hit = esfera_unitaria()
                .ray_intersect(&ray)
                .expect("debe impactar");

            assert!(
                (0.0..=1.0).contains(&hit.uv.x),
                "u fuera de rango: {}",
                hit.uv.x
            );
            assert!(
                (0.0..=1.0).contains(&hit.uv.y),
                "v fuera de rango: {}",
                hit.uv.y
            );
        }
    }
}
