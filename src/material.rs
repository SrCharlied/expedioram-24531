//! Materiales e iluminación directa.
//!
//! El color de un punto se arma con tres términos separados, y separarlos
//! importa porque se comportan distinto:
//!
//! - **Ambiente** — un mínimo constante. No es física: evita que lo que no
//!   ve ninguna luz quede en negro absoluto, donde no se distingue silueta.
//! - **Difusa de Lambert** — depende solo del ángulo con la luz. Da la forma.
//! - **Specular directa de Blinn–Phong** — depende también de dónde está el
//!   ojo. Da el brillo, y es lo único que hace que `wet_basalt` parezca
//!   mojado sin lanzar un solo rayo secundario.
//!
//! Esa última es la razón de que el specular directo exista aparte del
//! reflejo: `wet_basalt` tiene `reflection_cap = 0.0` —no rebota nada— y aun
//! así debe verse húmedo. El brillo es local y barato; el reflejo es un rayo
//! más y se reserva para agua y cristal.

use crate::color::Color;
use nalgebra_glm::{dot, Vec3};

/// Propiedades de superficie de un objeto.
///
/// Los techos `reflection_cap` y `transmission_cap` ya existen aunque el
/// Hito 5 sea quien los use: son parte del contrato del material y dejarlos
/// fuera obligaría a tocar cada constructor más adelante.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub albedo: Color,
    /// Intensidad del brillo local, `0.0..=1.0`.
    pub specular_strength: f32,
    /// Exponente del brillo. Mayor exponente, brillo más concentrado.
    pub shininess: f32,
    /// Techo del componente reflejado. `0.0` significa que no lanza rayo.
    pub reflection_cap: f32,
    /// Techo del componente refractado.
    pub transmission_cap: f32,
    /// Índice de refracción; `1.0` cuando no aplica.
    pub ior: f32,
    /// Repetición de textura. La usa el Hito 4.
    pub uv_scale: f32,
}

impl Material {
    /// Material opaco corriente: sin reflejo, sin refracción, brillo
    /// discreto. Son los valores por defecto que fija el inventario.
    pub fn new(albedo: Color) -> Self {
        Material {
            albedo,
            specular_strength: 0.10,
            shininess: 32.0,
            reflection_cap: 0.0,
            transmission_cap: 0.0,
            ior: 1.0,
            uv_scale: 1.0,
        }
    }

    /// Roca húmeda del Rompeolas y el sendero.
    ///
    /// Es el caso que justifica separar el specular del reflejo: brillo
    /// alto y concentrado, pero `reflection_cap = 0.0`. Parece mojada sin
    /// generar un solo rayo secundario.
    pub fn wet_basalt(albedo: Color) -> Self {
        Material {
            specular_strength: 0.85,
            shininess: 96.0,
            ..Material::new(albedo)
        }
    }

    pub fn with_specular(mut self, strength: f32, shininess: f32) -> Self {
        self.specular_strength = strength;
        self.shininess = shininess;
        self
    }

    /// ¿Este material genera rayos secundarios?
    pub fn is_reflective_or_refractive(&self) -> bool {
        self.reflection_cap > 0.0 || self.transmission_cap > 0.0
    }
}

/// Componente difusa de Lambert: el coseno del ángulo entre la normal y la
/// dirección a la luz, recortado a cero.
///
/// El recorte es lo que impide que una superficie de espaldas a la luz
/// reciba iluminación negativa y termine restando color.
pub fn lambert(normal: &Vec3, to_light: &Vec3) -> f32 {
    dot(normal, to_light).max(0.0)
}

/// Specular de Blinn–Phong.
///
/// Usa el vector medio entre la luz y el ojo en vez del reflejado de Phong:
/// una normalización menos por muestra, y el brillo no se corta de golpe en
/// ángulos rasantes, que es justo donde va a mirarse el agua.
pub fn blinn_phong(normal: &Vec3, to_light: &Vec3, to_view: &Vec3, shininess: f32) -> f32 {
    // De espaldas a la luz no hay brillo que valga.
    if dot(normal, to_light) <= 0.0 {
        return 0.0;
    }

    let medio = (to_light + to_view).normalize();

    dot(normal, &medio).max(0.0).powf(shininess)
}

/// Fracción de ambiente. Deliberadamente baja: solo lo justo para que la
/// geometría en sombra conserve silueta.
pub const AMBIENT: f32 = 0.06;

/// Aporte de **una** luz sobre un punto, ya atenuado.
///
/// No sabe nada de sombras ni de light linking: el renderer decide si esta
/// luz llega a este objeto antes de llamar. Devolver solo el aporte
/// mantiene la función pura y comprobable sin escena.
pub fn direct_light(
    material: &Material,
    normal: &Vec3,
    to_light: &Vec3,
    to_view: &Vec3,
    light_color: Color,
    attenuation: f32,
) -> Color {
    let difusa = lambert(normal, to_light);

    if difusa <= 0.0 {
        return Color::black();
    }

    let brillo = blinn_phong(normal, to_light, to_view, material.shininess);

    let luz = light_color * attenuation;

    // La difusa se tiñe con el albedo; el brillo no. Un highlight toma el
    // color de la luz, no el del objeto: es luz reflejada en la superficie,
    // no luz absorbida y reemitida.
    material.albedo * luz * difusa + luz * (material.specular_strength * brillo)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cerca(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-5
    }

    fn arriba() -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }

    #[test]
    fn una_superficie_de_frente_recibe_la_difusa_maxima() {
        assert!(cerca(lambert(&arriba(), &arriba()), 1.0));
    }

    #[test]
    fn una_superficie_opuesta_no_recibe_difusa() {
        assert_eq!(lambert(&arriba(), &-arriba()), 0.0);

        // Y tampoco en el limite exacto de 90 grados.
        assert!(lambert(&arriba(), &Vec3::new(1.0, 0.0, 0.0)).abs() < 1e-6);
    }

    #[test]
    fn la_difusa_sigue_el_coseno_del_angulo() {
        for grados in [0.0_f32, 30.0, 45.0, 60.0, 89.0] {
            let r = grados.to_radians();
            let hacia_luz = Vec3::new(r.sin(), r.cos(), 0.0);

            assert!(
                cerca(lambert(&arriba(), &hacia_luz), r.cos()),
                "a {grados} grados"
            );
        }
    }

    #[test]
    fn un_shininess_alto_estrecha_el_highlight() {
        // Luz y ojo del MISMO lado, no simetricos respecto de la normal:
        // asi el vector medio queda inclinado y el exponente se nota. Con
        // luz y ojo simetricos el medio coincide con la normal y el brillo
        // vale 1.0 para cualquier shininess --ese caso lo cubre el test
        // siguiente--.
        let hacia_luz = Vec3::new(0.20, 1.0, 0.0).normalize();
        let hacia_ojo = Vec3::new(0.95, 1.0, 0.0).normalize();

        let mut anterior = f32::INFINITY;
        for shininess in [4.0_f32, 16.0, 64.0, 256.0] {
            let brillo = blinn_phong(&arriba(), &hacia_luz, &hacia_ojo, shininess);

            assert!(
                brillo < anterior,
                "shininess {shininess} no estrecho el brillo: {brillo} contra {anterior}"
            );
            assert!((0.0..=1.0).contains(&brillo));
            anterior = brillo;
        }
    }

    #[test]
    fn el_highlight_es_maximo_cuando_el_vector_medio_es_la_normal() {
        // Luz y ojo simetricos respecto de la normal.
        let hacia_luz = Vec3::new(0.6, 0.8, 0.0).normalize();
        let hacia_ojo = Vec3::new(-0.6, 0.8, 0.0).normalize();

        // El medio de esos dos es exactamente la normal.
        assert!(cerca(
            blinn_phong(&arriba(), &hacia_luz, &hacia_ojo, 64.0),
            1.0
        ));
    }

    #[test]
    fn no_hay_brillo_de_espaldas_a_la_luz() {
        let desde_atras = Vec3::new(0.0, -1.0, 0.0);
        let hacia_ojo = arriba();

        assert_eq!(blinn_phong(&arriba(), &desde_atras, &hacia_ojo, 32.0), 0.0);
    }

    #[test]
    fn la_atenuacion_reduce_la_contribucion() {
        let material = Material::new(Color::new(1.0, 1.0, 1.0));
        let blanco = Color::new(1.0, 1.0, 1.0);

        let plena = direct_light(&material, &arriba(), &arriba(), &arriba(), blanco, 1.0);
        let media = direct_light(&material, &arriba(), &arriba(), &arriba(), blanco, 0.5);
        let nula = direct_light(&material, &arriba(), &arriba(), &arriba(), blanco, 0.0);

        assert!(media.r < plena.r && media.r > 0.0);
        assert!(cerca(media.r, plena.r * 0.5));
        assert!(cerca(nula.r, 0.0));
    }

    #[test]
    fn una_superficie_de_espaldas_no_aporta_nada() {
        let material = Material::new(Color::new(1.0, 1.0, 1.0));
        let aporte = direct_light(
            &material,
            &arriba(),
            &-arriba(),
            &arriba(),
            Color::new(1.0, 1.0, 1.0),
            1.0,
        );

        assert_eq!(aporte, Color::black());
    }

    #[test]
    fn la_difusa_se_tine_con_el_albedo_y_el_brillo_no() {
        // Objeto rojo bajo luz blanca, visto de frente: la difusa solo
        // aporta al canal rojo, pero el highlight es blanco.
        let material = Material::new(Color::new(1.0, 0.0, 0.0)).with_specular(1.0, 1.0);
        let aporte = direct_light(
            &material,
            &arriba(),
            &arriba(),
            &arriba(),
            Color::new(1.0, 1.0, 1.0),
            1.0,
        );

        // Rojo: difusa (1.0) mas brillo (1.0).
        assert!(cerca(aporte.r, 2.0));
        // Verde y azul: solo el brillo.
        assert!(cerca(aporte.g, 1.0));
        assert!(cerca(aporte.b, 1.0));
    }

    #[test]
    fn wet_basalt_brilla_sin_reflejar() {
        let material = Material::wet_basalt(Color::new(0.3, 0.3, 0.33));

        assert!(material.specular_strength > 0.5, "deberia verse mojado");
        assert!(
            material.shininess > 64.0,
            "el brillo deberia ser concentrado"
        );
        assert_eq!(
            material.reflection_cap, 0.0,
            "no debe lanzar rayos secundarios"
        );
        assert!(!material.is_reflective_or_refractive());
    }

    #[test]
    fn los_opacos_no_generan_rayos_secundarios_por_defecto() {
        let material = Material::new(Color::new(0.5, 0.5, 0.5));

        assert_eq!(material.reflection_cap, 0.0);
        assert_eq!(material.transmission_cap, 0.0);
        assert_eq!(material.ior, 1.0);
        assert!(!material.is_reflective_or_refractive());
    }
}
