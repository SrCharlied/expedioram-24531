use crate::ray::Ray;
use nalgebra_glm::{dot, Vec2, Vec3};

/// Qué parametrización produjo la `uv` de un impacto.
///
/// # Para qué
///
/// Una `uv` sola no identifica un punto de una superficie. Las seis caras de
/// un cuboide recorren las mismas `0..1`, así que `(0.5, 0.5)` en la cara
/// `+X` y `(0.5, 0.5)` en la `-X` son coordenadas **iguales** de puntos
/// **opuestos**. Una máscara de pincel indexada solo por `uv` pintaría las
/// dos a la vez, y el síntoma sería pintura apareciendo en la cara oculta.
///
/// La carta es lo que las separa: dos impactos pertenecen al mismo trozo
/// pintable si coinciden **el objeto y la carta**.
///
/// # Quién la escribe
///
/// La primitiva, que es la única que sabe qué cara tocó. Clasificar después
/// por la normal sería reconstruir a ojo algo que ya se sabía con exactitud:
/// en un hexágono dos laterales difieren `60°`, y una comparación por umbral
/// que hoy acierta se rompe en cuanto alguien gire o achate una forma.
///
/// # Qué **no** es
///
/// No es un identificador global. El número solo tiene sentido junto al
/// `object_index` del impacto: la carta `1` de un cuboide y la `1` de un
/// prisma no tienen nada que ver. Cada primitiva documenta su propio reparto.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct UvChart(u8);

impl UvChart {
    /// La superficie tiene **una sola** parametrización y no hay nada que
    /// distinguir. Es el valor por defecto.
    pub const WHOLE: UvChart = UvChart(0);

    /// Carta `id` de la primitiva que la escribe.
    pub const fn new(id: u8) -> Self {
        UvChart(id)
    }

    /// El número, para quien necesite indexar por carta.
    pub const fn id(self) -> u8 {
        self.0
    }
}

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
    /// Sobre qué carta se midió `uv`. Ver `UvChart`.
    pub uv_chart: UvChart,
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
            uv_chart: UvChart::WHOLE,
        }
    }

    /// Declara sobre qué carta se midió la `uv`.
    ///
    /// Va aparte de `new` y no como un quinto argumento para que las
    /// primitivas de una sola carta —y los tests que solo miran normales o
    /// distancias— sigan escribiéndose igual. La primitiva que sí distingue
    /// caras encadena esta llamada y queda dicho en el punto donde se sabe
    /// qué cara era.
    pub fn on_chart(mut self, chart: UvChart) -> Self {
        self.uv_chart = chart;
        self
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

    // ------------------------------------------------------- carta UV

    fn impacto() -> Hit {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

        Hit::new(&ray, 4.0, Vec3::new(0.0, 0.0, 1.0), Vec2::new(0.25, 0.75))
    }

    #[test]
    fn un_impacto_sin_declarar_carta_usa_la_unica() {
        // La compatibilidad que sostiene el corte: `Hit::new` conserva sus
        // cuatro argumentos y las primitivas que no distinguen caras no
        // tienen que decir nada.
        let hit = impacto();

        assert_eq!(hit.uv_chart, UvChart::WHOLE);
        assert_eq!(UvChart::default(), UvChart::WHOLE);
    }

    #[test]
    fn declarar_una_carta_no_toca_nada_mas_del_impacto() {
        let base = impacto();
        let marcado = impacto().on_chart(UvChart::new(4));

        assert_eq!(marcado.uv_chart, UvChart::new(4));
        assert_eq!(marcado.uv, base.uv, "la uv es la que midio la primitiva");
        assert_eq!(marcado.normal, base.normal);
        assert_eq!(marcado.distance, base.distance);
        assert_eq!(marcado.front_face, base.front_face);
        assert_eq!(marcado.object_index, base.object_index);
    }

    #[test]
    fn dos_cartas_se_comparan_por_identidad() {
        // Es lo único que se le pide al tipo: decir si dos impactos caen en
        // la misma parametrización. No hay orden ni aritmética.
        assert_eq!(UvChart::new(2), UvChart::new(2));
        assert_ne!(UvChart::new(2), UvChart::new(3));
        assert_ne!(UvChart::new(1), UvChart::WHOLE);
    }
}
