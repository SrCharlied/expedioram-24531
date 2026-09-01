use crate::ray::Ray;
use nalgebra_glm::{dot, Vec2, Vec3};

/// Todo lo que se sabe de un impacto.
///
/// Reemplaza al antiguo `Intersect`, que cargaba una copia del `Material`.
/// Aquí no hay material: durante la revelación un objeto no tiene *un*
/// material sino `initial_material`, `final_material` y el progreso de su
/// grupo. Copiar eso en cada impacto sería copiar estado mutable en el
/// camino caliente. El impacto solo dice **qué** objeto se tocó, con
/// `object_index`, y el renderer resuelve el resto.
#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub distance: f32,
    pub point: Vec3,
    /// Siempre orientada **contra** el rayo. Ver `front_face`.
    pub normal: Vec3,
    pub uv: Vec2,
    /// `true` si el rayo golpeó la cara exterior de la superficie.
    ///
    /// Un rayo que sale del volumen de agua golpea la misma geometría desde
    /// adentro, y ahí la normal geométrica apunta en el sentido equivocado
    /// para iluminar o para calcular refracción. Guardar de qué lado se
    /// entró permite voltear la normal y recordar que se volteó, que es lo
    /// que necesita Fresnel para elegir la razón de índices correcta.
    pub front_face: bool,
    /// Índice del objeto dentro de la escena. Lo asigna quien recorre la
    /// escena, no la primitiva: una primitiva no sabe dónde vive.
    pub object_index: usize,
}

impl Hit {
    /// Construye un impacto orientando la normal contra el rayo.
    ///
    /// `outward_normal` es la normal geométrica de la superficie, la que
    /// apunta hacia afuera del sólido. Este constructor decide de qué lado
    /// venía el rayo y guarda ambas cosas.
    pub fn new(ray: &Ray, distance: f32, outward_normal: Vec3, uv: Vec2) -> Self {
        let front_face = dot(&ray.direction, &outward_normal) < 0.0;

        Hit {
            distance,
            point: ray.at(distance),
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            uv,
            front_face,
            object_index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_frontal_apunta_contra_el_rayo() {
        // Rayo que viaja hacia -Z contra una cara cuya normal exterior
        // apunta hacia +Z: se golpea desde afuera.
        let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = Hit::new(&ray, 4.0, Vec3::new(0.0, 0.0, 1.0), Vec2::zeros());

        assert!(hit.front_face);
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
        assert!(dot(&hit.normal, &ray.direction) < 0.0);
    }

    #[test]
    fn normal_interna_se_invierte_y_marca_front_face_falso() {
        // Mismo rayo hacia -Z, pero ahora la normal exterior también apunta
        // hacia -Z: el rayo la alcanza por dentro del sólido.
        let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = Hit::new(&ray, 4.0, Vec3::new(0.0, 0.0, -1.0), Vec2::zeros());

        assert!(!hit.front_face);
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
        assert!(dot(&hit.normal, &ray.direction) < 0.0);
    }

    #[test]
    fn el_punto_se_deriva_del_rayo_y_la_distancia() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = Hit::new(&ray, 4.0, Vec3::new(0.0, 0.0, 1.0), Vec2::zeros());

        assert_eq!(hit.point, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn object_index_arranca_en_cero_y_lo_fija_la_escena() {
        let ray = Ray::new(Vec3::zeros(), Vec3::new(0.0, 0.0, -1.0));
        let hit = Hit::new(&ray, 1.0, Vec3::new(0.0, 0.0, 1.0), Vec2::zeros());

        assert_eq!(hit.object_index, 0);
    }
}
