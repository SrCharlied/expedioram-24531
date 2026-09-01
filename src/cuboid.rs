use crate::bounds::Aabb;
use crate::hit::Hit;
use crate::ray::Ray;
use crate::ray_intersect::RayIntersect;
use crate::EPSILON;
use nalgebra_glm::{Vec2, Vec3};

/// Cuboide alineado a los ejes: la primitiva obligatoria del proyecto.
///
/// La distancia sale del slab test de `Aabb`. Lo que agrega este tipo es lo
/// que el AABB no necesita saber para acelerar pero el sombreado sí: qué
/// cara se tocó, hacia dónde mira y qué coordenada de textura corresponde.
#[derive(Debug, Clone, Copy)]
pub struct Cuboid {
    pub bounds: Aabb,
}

impl Cuboid {
    pub fn new(bounds: Aabb) -> Self {
        Cuboid { bounds }
    }

    /// Cuboide a partir de dos esquinas cualesquiera.
    pub fn from_corners(a: Vec3, b: Vec3) -> Self {
        Cuboid::new(Aabb::from_corners(a, b))
    }

    /// Cuboide centrado con un tamaño dado por eje. La escala no uniforme
    /// es lo normal en el diorama: pilares, tablones y masas de terreno son
    /// todos cajas estiradas.
    pub fn centrado(centro: Vec3, tamano: Vec3) -> Self {
        let medio = tamano * 0.5;
        Cuboid::new(Aabb::new(centro - medio, centro + medio))
    }

    /// Normal exterior de la cara perpendicular a `eje`.
    ///
    /// El signo sale de hacia dónde viaja el rayo: si avanza en `+eje`,
    /// entró por el plano `min` y esa cara mira hacia `-eje`.
    fn normal_de_cara(eje: usize, direccion_en_eje: f32, entrando: bool) -> Vec3 {
        let mut normal = Vec3::zeros();
        let hacia_adelante = direccion_en_eje > 0.0;

        // Al entrar se toca la cara opuesta al avance; al salir, la del
        // mismo lado hacia el que se avanza.
        normal[eje] = if hacia_adelante == entrando {
            -1.0
        } else {
            1.0
        };

        normal
    }

    /// Coordenada de textura dentro de la cara perpendicular a `eje`.
    ///
    /// Se usan los otros dos ejes como tangentes, normalizando la posición
    /// contra la extensión de la caja. Una cara degenerada —extensión cero—
    /// devuelve `0.0` en vez de dividir entre cero.
    fn uv_en_cara(&self, punto: &Vec3, eje: usize) -> Vec2 {
        let (eje_u, eje_v) = match eje {
            0 => (2, 1), // cara X: u recorre Z, v recorre Y
            1 => (0, 2), // cara Y: u recorre X, v recorre Z
            _ => (0, 1), // cara Z: u recorre X, v recorre Y
        };

        Vec2::new(self.fraccion(punto, eje_u), self.fraccion(punto, eje_v))
    }

    fn fraccion(&self, punto: &Vec3, eje: usize) -> f32 {
        let extension = self.bounds.max[eje] - self.bounds.min[eje];

        if extension.abs() < f32::EPSILON {
            return 0.0;
        }

        ((punto[eje] - self.bounds.min[eje]) / extension).clamp(0.0, 1.0)
    }
}

impl RayIntersect for Cuboid {
    fn ray_intersect(&self, ray: &Ray) -> Option<Hit> {
        let intervalo = self.bounds.hit(ray, EPSILON, f32::INFINITY)?;

        // Un rayo que nace dentro del cuboide no tiene cara de entrada
        // visible: lo que va a tocar es la cara por la que sale. Es el caso
        // del rayo refractado que viaja dentro del volumen de agua.
        let desde_adentro = self.bounds.contiene(&ray.origin);

        let (t, eje) = if desde_adentro {
            (intervalo.t_exit, intervalo.exit_axis)
        } else {
            (intervalo.t_enter, intervalo.enter_axis)
        };

        if t <= EPSILON {
            return None;
        }

        let punto = ray.at(t);
        let normal = Cuboid::normal_de_cara(eje, ray.direction[eje], !desde_adentro);

        Some(Hit::new(ray, t, normal, self.uv_en_cara(&punto, eje)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cubo_unitario() -> Cuboid {
        Cuboid::from_corners(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0))
    }

    /// Las seis caras, cada una con la dirección desde la que se le dispara
    /// y la normal exterior que debe reportar.
    const CARAS: [([f32; 3], [f32; 3]); 6] = [
        ([1.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        ([-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]),
        ([0.0, 1.0, 0.0], [0.0, 1.0, 0.0]),
        ([0.0, -1.0, 0.0], [0.0, -1.0, 0.0]),
        ([0.0, 0.0, 1.0], [0.0, 0.0, 1.0]),
        ([0.0, 0.0, -1.0], [0.0, 0.0, -1.0]),
    ];

    #[test]
    fn las_seis_caras_devuelven_su_normal() {
        for (desde, normal_esperada) in CARAS {
            let fuera = Vec3::new(desde[0], desde[1], desde[2]) * 5.0;
            let esperada = Vec3::new(normal_esperada[0], normal_esperada[1], normal_esperada[2]);

            let ray = Ray::new(fuera, -fuera.normalize());
            let hit = cubo_unitario()
                .ray_intersect(&ray)
                .unwrap_or_else(|| panic!("la cara {esperada:?} debe impactar"));

            assert_eq!(hit.normal, esperada, "cara {esperada:?}");
            assert!(hit.front_face, "cara {esperada:?} se toca desde afuera");
            assert!((hit.distance - 4.0).abs() < 1e-5, "cara {esperada:?}");
        }
    }

    #[test]
    fn uv_permanece_en_rango_unitario() {
        for (desde, _) in CARAS {
            let fuera = Vec3::new(desde[0], desde[1], desde[2]) * 5.0;
            let ray = Ray::new(fuera, -fuera.normalize());
            let hit = cubo_unitario().ray_intersect(&ray).expect("debe impactar");

            assert!((0.0..=1.0).contains(&hit.uv.x), "u = {}", hit.uv.x);
            assert!((0.0..=1.0).contains(&hit.uv.y), "v = {}", hit.uv.y);
        }
    }

    #[test]
    fn uv_recorre_la_cara_completa() {
        // Dos rayos hacia la cara +Z, uno cerca de la esquina inferior
        // izquierda y otro cerca de la superior derecha.
        let cubo = cubo_unitario();
        let direccion = Vec3::new(0.0, 0.0, -1.0);

        let bajo = cubo
            .ray_intersect(&Ray::new(Vec3::new(-0.9, -0.9, 5.0), direccion))
            .expect("debe impactar");
        let alto = cubo
            .ray_intersect(&Ray::new(Vec3::new(0.9, 0.9, 5.0), direccion))
            .expect("debe impactar");

        assert!(bajo.uv.x < 0.1 && bajo.uv.y < 0.1, "{:?}", bajo.uv);
        assert!(alto.uv.x > 0.9 && alto.uv.y > 0.9, "{:?}", alto.uv);
    }

    #[test]
    fn rayo_interno_usa_la_cara_de_salida() {
        let ray = Ray::new(Vec3::zeros(), Vec3::new(0.0, 0.0, -1.0));
        let hit = cubo_unitario().ray_intersect(&ray).expect("debe salir");

        assert!((hit.distance - 1.0).abs() < 1e-5);
        assert!(!hit.front_face, "se toca desde adentro");
        // La normal exterior de la cara -Z es (0,0,-1); Hit la voltea para
        // dejarla contra el rayo, que viaja hacia -Z.
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn escala_no_uniforme_funciona() {
        // Caja de 2 x 4 x 6 centrada en el origen.
        let caja = Cuboid::centrado(Vec3::zeros(), Vec3::new(2.0, 4.0, 6.0));

        let por_x = caja
            .ray_intersect(&Ray::new(
                Vec3::new(5.0, 0.0, 0.0),
                Vec3::new(-1.0, 0.0, 0.0),
            ))
            .expect("debe impactar en X");
        let por_y = caja
            .ray_intersect(&Ray::new(
                Vec3::new(0.0, 5.0, 0.0),
                Vec3::new(0.0, -1.0, 0.0),
            ))
            .expect("debe impactar en Y");
        let por_z = caja
            .ray_intersect(&Ray::new(
                Vec3::new(0.0, 0.0, 5.0),
                Vec3::new(0.0, 0.0, -1.0),
            ))
            .expect("debe impactar en Z");

        assert!((por_x.distance - 4.0).abs() < 1e-5, "{}", por_x.distance);
        assert!((por_y.distance - 3.0).abs() < 1e-5, "{}", por_y.distance);
        assert!((por_z.distance - 2.0).abs() < 1e-5, "{}", por_z.distance);

        assert_eq!(por_x.normal, Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(por_y.normal, Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(por_z.normal, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn objeto_detras_de_la_camara_falla() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 1.0));

        assert!(cubo_unitario().ray_intersect(&ray).is_none());
    }

    #[test]
    fn rayo_que_pasa_de_lado_falla() {
        let ray = Ray::new(Vec3::new(3.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

        assert!(cubo_unitario().ray_intersect(&ray).is_none());
    }
}
