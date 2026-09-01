use crate::bounds::Aabb;
use crate::cuboid::Cuboid;
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
/// La Ruta A del prisma hexagonal agregará una variante detrás de la
/// feature `hex-prism` en la Tarea 7.3, si el profesor la autoriza.
#[derive(Debug, Clone, Copy)]
pub enum Primitive {
    Cuboid(Cuboid),
}

impl Primitive {
    /// Caja envolvente de la primitiva. La estructura de aceleración del
    /// Hito 3 la usa para armar los bounds de cada cluster.
    pub fn bounds(&self) -> Aabb {
        match self {
            Primitive::Cuboid(cuboid) => cuboid.bounds,
        }
    }
}

impl RayIntersect for Primitive {
    fn ray_intersect(&self, ray: &Ray) -> Option<Hit> {
        match self {
            Primitive::Cuboid(cuboid) => cuboid.ray_intersect(ray),
        }
    }
}

impl From<Cuboid> for Primitive {
    fn from(cuboid: Cuboid) -> Self {
        Primitive::Cuboid(cuboid)
    }
}
