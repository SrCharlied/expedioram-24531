use crate::bounds::Aabb;
use crate::cuboid::Cuboid;
#[cfg(feature = "hex-prism")]
use crate::hex_prism::HexPrism;
use crate::hit::Hit;
use crate::ray::Ray;
use crate::ray_intersect::RayIntersect;

/// Las formas trazables de la escena.
///
/// Es un `enum` y no `Vec<Box<dyn RayIntersect>>` a propósito: la
/// intersección está en el camino más caliente del renderer —cientos de
/// primitivas por rayo, medio millón de rayos por cuadro— y un despacho
/// dinámico ahí cuesta una indirección por prueba, además de impedirle al
/// compilador insertar el código en línea.
///
/// La Ruta A del prisma hexagonal agrega su variante detrás de la feature
/// `hex-prism`, autorizada en la Tarea 7.3. Apagada, este enum es
/// exactamente el que aprobó la Ruta B: una sola variante, sin rama muerta
/// ni comprobación de etiqueta en el camino caliente.
#[derive(Debug, Clone, Copy)]
pub enum Primitive {
    Cuboid(Cuboid),
    #[cfg(feature = "hex-prism")]
    HexPrism(HexPrism),
}

impl Primitive {
    /// Caja envolvente de la primitiva. La estructura de aceleración del
    /// Hito 3 la usa para armar los bounds de cada cluster.
    pub fn bounds(&self) -> Aabb {
        match self {
            Primitive::Cuboid(cuboid) => cuboid.bounds,
            #[cfg(feature = "hex-prism")]
            Primitive::HexPrism(prisma) => prisma.bounds(),
        }
    }
}

impl RayIntersect for Primitive {
    fn ray_intersect(&self, ray: &Ray) -> Option<Hit> {
        match self {
            Primitive::Cuboid(cuboid) => cuboid.ray_intersect(ray),
            #[cfg(feature = "hex-prism")]
            Primitive::HexPrism(prisma) => prisma.ray_intersect(ray),
        }
    }
}

impl From<Cuboid> for Primitive {
    fn from(cuboid: Cuboid) -> Self {
        Primitive::Cuboid(cuboid)
    }
}

#[cfg(feature = "hex-prism")]
impl From<HexPrism> for Primitive {
    fn from(prisma: HexPrism) -> Self {
        Primitive::HexPrism(prisma)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra_glm::Vec3;

    #[test]
    fn un_cuboide_entra_en_primitive_y_conserva_su_caja() {
        let cuboid = Cuboid::centrado(Vec3::zeros(), Vec3::new(2.0, 2.0, 2.0));
        let primitiva: Primitive = cuboid.into();

        assert_eq!(primitiva.bounds(), cuboid.bounds);
    }

    #[cfg(feature = "hex-prism")]
    #[test]
    fn un_prisma_entra_en_primitive_y_conserva_su_caja() {
        // La variante existe solo con la feature. Sin ella, `Primitive`
        // tiene que seguir siendo exactamente el enum de una variante que
        // aprobo la Ruta B.
        use crate::hex_prism::HexPrism;

        let prisma = HexPrism::new(Vec3::zeros(), 1.0, 2.0);
        let primitiva: Primitive = prisma.into();

        assert_eq!(primitiva.bounds(), prisma.bounds());
    }

    #[cfg(feature = "hex-prism")]
    #[test]
    fn primitive_delega_la_interseccion_en_el_prisma() {
        use crate::hex_prism::HexPrism;
        use crate::ray::Ray;

        let primitiva: Primitive = HexPrism::new(Vec3::zeros(), 1.0, 2.0).into();
        let ray = Ray::new(Vec3::new(5.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0));

        let hit = primitiva.ray_intersect(&ray).expect("el prisma se toca");

        assert!(hit.front_face);
        assert!((hit.normal - Vec3::new(1.0, 0.0, 0.0)).magnitude() < 1e-5);
    }
}
