use crate::bounds::Aabb;
use crate::cuboid::Cuboid;
#[cfg(feature = "hex-prism")]
use crate::hex_prism::HexPrism;
use crate::hit::{Hit, UvChart};
use crate::ray::Ray;
use crate::ray_intersect::RayIntersect;

/// Cuánto mundo recorre cada coordenada `uv` de una carta.
///
/// # Para qué
///
/// Una `uv` es adimensional: `0..1` sobre la tapa de un pilar y `0..1` sobre
/// el lecho de la bahía recorren distancias que se diferencian en un orden
/// de magnitud. Un pincel de radio fijo en `uv` sale diminuto sobre lo
/// grande y enorme sobre lo pequeño, y peor aún: sobre una cara alargada
/// sale ovalado, porque sus dos ejes no miden lo mismo.
///
/// Esto es lo que falta para corregirlo. Quien pinte puede dividir un radio
/// **en unidades de mundo** por estas dos escalas y obtener los semiejes
/// `uv` que dibujan un trazo redondo sobre la pieza. Ese cálculo no vive
/// aquí: este tipo solo dice cuánto mide la superficie.
///
/// # Invariante
///
/// Las dos escalas son finitas y estrictamente positivas. Una cara
/// degenerada no produce una escala cero, produce `None`: cero no es una
/// medida, es una división pendiente de estallar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UvWorldScale {
    /// Unidades de mundo que recorre `u` de `0` a `1`.
    pub u: f32,
    /// Unidades de mundo que recorre `v` de `0` a `1`.
    pub v: f32,
}

impl UvWorldScale {
    /// Métrica de una carta, o `None` si alguna extensión no es una medida
    /// utilizable.
    pub fn new(u: f32, v: f32) -> Option<Self> {
        (Self::util(u) && Self::util(v)).then_some(UvWorldScale { u, v })
    }

    /// Semiejes `uv` que dibujan un pincel de radio `radio_mundo` sobre esta
    /// superficie.
    ///
    /// Es la división que da sentido a todo el tipo: cada eje `uv` recibe el
    /// radio dividido por lo que ese eje recorre. Sobre una cara que mide el
    /// doble de ancha que de alta, el mismo radio ocupa la mitad de `u` que
    /// de `v`, y el trazo se ve **redondo sobre la pieza** en vez de ovalado.
    ///
    /// Devuelve `None` con un radio que no sea finito y positivo, y también
    /// si el reparto no produjera dos semiejes utilizables: una métrica
    /// diminuta podría desbordar la división, y un semieje infinito no
    /// dibuja, revienta.
    pub fn uv_radii(&self, radio_mundo: f32) -> Option<(f32, f32)> {
        if !Self::util(radio_mundo) {
            return None;
        }

        let (radio_u, radio_v) = (radio_mundo / self.u, radio_mundo / self.v);

        (Self::util(radio_u) && Self::util(radio_v)).then_some((radio_u, radio_v))
    }

    /// ¿Es una longitud utilizable? Finita y estrictamente positiva.
    fn util(e: f32) -> bool {
        e.is_finite() && e > 0.0
    }
}

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

    /// Métrica mundo/`uv` de una de sus cartas, o `None` si esa carta no es
    /// suya o no da una medida utilizable.
    ///
    /// Delega: la primitiva es la única que sabe qué recorre cada
    /// coordenada de cada una de sus caras. Deducirlo desde fuera —por la
    /// normal, o por el tamaño de la caja envolvente— sería reconstruir a
    /// ojo algo que aquí es exacto, y se rompería en cuanto una forma nueva
    /// no siguiera la convención que se hubiera supuesto.
    pub fn uv_world_scale(&self, chart: UvChart) -> Option<UvWorldScale> {
        match self {
            Primitive::Cuboid(cuboid) => cuboid.uv_world_scale(chart),
            #[cfg(feature = "hex-prism")]
            Primitive::HexPrism(prisma) => prisma.uv_world_scale(chart),
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

    // ------------------------------------- radio de mundo a semiejes uv

    #[test]
    fn un_radio_de_mundo_se_reparte_entre_los_dos_ejes() {
        // Una cara que mide `4` de ancho y `2` de alto: el mismo radio de
        // mundo ocupa la mitad de `u` que de `v`, que es justo lo que hace
        // que el trazo se vea redondo sobre la pieza y no ovalado.
        let escala = UvWorldScale::new(4.0, 2.0).expect("metrica valida");

        let (radio_u, radio_v) = escala.uv_radii(1.0).expect("radio valido");

        assert!((radio_u - 0.25).abs() < 1e-6, "radio_u = {radio_u}");
        assert!((radio_v - 0.5).abs() < 1e-6, "radio_v = {radio_v}");
    }

    #[test]
    fn una_cara_cuadrada_da_los_dos_semiejes_iguales() {
        let escala = UvWorldScale::new(2.0, 2.0).expect("metrica valida");

        let (radio_u, radio_v) = escala.uv_radii(0.5).expect("radio valido");

        assert_eq!(radio_u, radio_v);
        assert!((radio_u - 0.25).abs() < 1e-6);
    }

    #[test]
    fn el_radio_escala_de_forma_proporcional() {
        let escala = UvWorldScale::new(8.0, 3.0).expect("metrica valida");

        let (u1, v1) = escala.uv_radii(1.0).expect("radio valido");
        let (u2, v2) = escala.uv_radii(2.0).expect("radio valido");

        assert!((u2 - 2.0 * u1).abs() < 1e-6);
        assert!((v2 - 2.0 * v1).abs() < 1e-6);
    }

    #[test]
    fn un_radio_de_mundo_invalido_no_da_semiejes() {
        let escala = UvWorldScale::new(4.0, 2.0).expect("metrica valida");

        for radio in [0.0f32, -1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert_eq!(escala.uv_radii(radio), None, "radio {radio}");
        }
    }

    #[test]
    fn una_metrica_invalida_no_se_construye() {
        for malo in [0.0f32, -2.0, f32::NAN, f32::INFINITY] {
            assert_eq!(UvWorldScale::new(malo, 1.0), None, "u = {malo}");
            assert_eq!(UvWorldScale::new(1.0, malo), None, "v = {malo}");
        }
    }

    #[test]
    fn primitive_delega_la_metrica_en_el_cuboide() {
        use crate::hit::UvChart;

        let cuboid = Cuboid::centrado(Vec3::zeros(), Vec3::new(2.0, 4.0, 6.0));
        let primitiva: Primitive = cuboid.into();

        for carta in 1..=6u8 {
            assert_eq!(
                primitiva.uv_world_scale(UvChart::new(carta)),
                cuboid.uv_world_scale(UvChart::new(carta)),
                "la carta {carta} no se delego"
            );
        }

        assert_eq!(primitiva.uv_world_scale(UvChart::WHOLE), None);
    }

    #[cfg(feature = "hex-prism")]
    #[test]
    fn primitive_delega_la_metrica_en_el_prisma() {
        use crate::hex_prism::HexPrism;
        use crate::hit::UvChart;

        let prisma = HexPrism::new(Vec3::zeros(), 1.0, 2.0);
        let primitiva: Primitive = prisma.into();

        for carta in 0..=4u8 {
            assert_eq!(
                primitiva.uv_world_scale(UvChart::new(carta)),
                prisma.uv_world_scale(UvChart::new(carta)),
                "la carta {carta} no se delego"
            );
        }
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
