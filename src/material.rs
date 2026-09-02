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
use crate::scene::TextureId;
use nalgebra_glm::{dot, Vec3};

/// Cómo trata un objeto a los rayos de sombra.
///
/// Es el **único** campo de sombras del objeto. Antes convivía con un
/// `casts_shadow` booleano y los dos competían por la misma
/// responsabilidad, lo que garantizaba que tarde o temprano el renderer
/// eligiera mal; `Ignore` sustituye exactamente al antiguo `casts_shadow`
/// en falso. El `casts_shadows` de las **luces** es otra cosa y sí existe:
/// dice si la luz llega a generar rayos de sombra.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShadowMode {
    /// Bloquea por completo. Es el caso normal.
    #[default]
    Opaque,
    /// El rayo lo atraviesa como si no estuviera.
    ///
    /// Es lo que usa el agua. Sin esto, el volumen de la bahía se
    /// interpondría entre el barco y toda luz exterior, y el barco —el
    /// objeto estrella del diorama— se vería negro.
    Ignore,
    /// Atenúa en vez de bloquear. **Fuera del MVP.**
    ///
    /// Requiere seguir buscando intersecciones en vez de cortar en el
    /// primer bloqueador, así que no es tan barato como un any-hit opaco.
    /// Si se implementa, la visibilidad se multiplica por
    /// `transmission_cap`, **no** por `1 - transmission_cap`.
    Attenuate,
}

/// Propiedades de superficie de un objeto.
///
/// Los techos `reflection_cap` y `transmission_cap` ya existen aunque el
/// Hito 5 sea quien los use: son parte del contrato del material y dejarlos
/// fuera obligaría a tocar cada constructor más adelante.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    /// Color propio de la superficie. Con textura actúa como **tinte**: el
    /// color final es el producto de los dos.
    pub albedo: Color,
    /// Textura de albedo, si la tiene. `None` significa color plano.
    pub albedo_texture: Option<TextureId>,
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
    /// Qué hace este material ante un rayo de sombra.
    ///
    /// **No se interpola durante la revelación.** El modo del material
    /// final rige desde `progress = 0.0`: el agua no bloquea sombras ni
    /// siquiera mientras todavía se ve como lienzo. Interpolarlo haría que
    /// el barco parpadeara entre iluminado y negro justo durante la
    /// transición estrella del diorama.
    pub shadow_mode: ShadowMode,
}

impl Material {
    /// Material opaco corriente: sin reflejo, sin refracción, brillo
    /// discreto. Son los valores por defecto que fija el inventario.
    pub fn new(albedo: Color) -> Self {
        Material {
            albedo,
            albedo_texture: None,
            specular_strength: 0.10,
            shininess: 32.0,
            reflection_cap: 0.0,
            transmission_cap: 0.0,
            ior: 1.0,
            uv_scale: 1.0,
            shadow_mode: ShadowMode::Opaque,
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

    /// Asocia una textura de albedo.
    ///
    /// Pone el tinte en blanco a propósito: las texturas del proyecto ya
    /// llevan el color del material, y multiplicarlas por un tinte de color
    /// además lo oscurecería dos veces. Para las variantes que sí quieren
    /// teñir —la cadena sobre la textura de basalto— está `with_tint`
    /// después de esta.
    pub fn with_texture(mut self, texture: TextureId) -> Self {
        self.albedo_texture = Some(texture);
        self.albedo = Color::new(1.0, 1.0, 1.0);
        self
    }

    /// Tinte sobre la textura. Sin textura, es el color plano.
    pub fn with_tint(mut self, tint: Color) -> Self {
        self.albedo = tint;
        self
    }

    /// Repetición de la textura sobre la cara. Valores mayores que uno
    /// necesitan que la textura envuelva sin costura.
    pub fn with_uv_scale(mut self, uv_scale: f32) -> Self {
        self.uv_scale = uv_scale.max(0.0);
        self
    }

    /// Techos ópticos, recortados a `0.0..=1.0`.
    ///
    /// El recorte no es cosmético: son fracciones de energía, y un techo
    /// mayor que uno haría que una superficie devolviera más luz de la que
    /// recibe. El reparto de Fresnel del Hito 5 da
    /// `kr + kt = cap_r · F + cap_t · (1 - F)`, que con ambos techos en
    /// rango nunca pasa de uno.
    pub fn with_caps(mut self, reflection: f32, transmission: f32, ior: f32) -> Self {
        self.reflection_cap = reflection.clamp(0.0, 1.0);
        self.transmission_cap = transmission.clamp(0.0, 1.0);
        self.ior = ior.max(1.0);
        self
    }

    /// ¿Están todos los campos acotados dentro de su rango legal?
    pub fn is_valid(&self) -> bool {
        (0.0..=1.0).contains(&self.specular_strength)
            && (0.0..=1.0).contains(&self.reflection_cap)
            && (0.0..=1.0).contains(&self.transmission_cap)
            && self.shininess > 0.0
            && self.ior >= 1.0
            && self.uv_scale >= 0.0
    }

    /// Energía máxima que puede devolver el reparto de Fresnel, sobre
    /// cualquier ángulo. Nunca debería pasar de uno.
    pub fn max_energy(&self) -> f32 {
        self.reflection_cap.max(self.transmission_cap)
    }

    pub fn with_shadow_mode(mut self, shadow_mode: ShadowMode) -> Self {
        self.shadow_mode = shadow_mode;
        self
    }

    /// ¿Este material detiene un rayo de sombra?
    pub fn blocks_shadows(&self) -> bool {
        matches!(self.shadow_mode, ShadowMode::Opaque)
    }

    pub fn with_specular(mut self, strength: f32, shininess: f32) -> Self {
        self.specular_strength = strength.clamp(0.0, 1.0);
        // Un exponente nulo o negativo daría un brillo constante sobre toda
        // la superficie, que no es un brillo.
        self.shininess = shininess.max(1.0);
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
///
/// El material que llega aquí es el que devuelve `reveal::resolve`: su
/// `albedo` es el color ya muestreado y ya interpolado, y su
/// `albedo_texture` es `None`. Esta función no muestrea nada.
pub fn direct_light(
    material: &Material,
    normal: &Vec3,
    to_light: &Vec3,
    to_view: &Vec3,
    light_color: Color,
    attenuation: f32,
) -> Color {
    direct_diffuse(material, normal, to_light, light_color, attenuation)
        + direct_specular(
            material,
            normal,
            to_light,
            to_view,
            light_color,
            attenuation,
        )
}

/// Componente **difusa** de una luz, ya teñida por el albedo.
///
/// Está separada del brillo porque el reparto de Fresnel las trata
/// distinto: la difusa es color propio de la superficie y se escala por
/// `kl`, mientras que el brillo se suma después del reparto. Ver el orden
/// de `renderer::cast_ray`.
pub fn direct_diffuse(
    material: &Material,
    normal: &Vec3,
    to_light: &Vec3,
    light_color: Color,
    attenuation: f32,
) -> Color {
    let difusa = lambert(normal, to_light);

    if difusa <= 0.0 {
        return Color::black();
    }

    material.albedo * (light_color * attenuation) * difusa
}

/// Componente **especular** de una luz.
///
/// No se tiñe con el albedo: un highlight toma el color de la luz, no el
/// del objeto, porque es luz reflejada en la superficie y no luz absorbida
/// y reemitida.
///
/// Tampoco se escala por `kl`. Con los caps `0.9 / 0.9` del agua eso lo
/// dejaría al diez por ciento, y el gate de Aguas Voladoras exige que el
/// highlight del agua se vea. El specular directo es, precisamente, la
/// parte de la reflexión que se resuelve sin lanzar un rayo.
pub fn direct_specular(
    material: &Material,
    normal: &Vec3,
    to_light: &Vec3,
    to_view: &Vec3,
    light_color: Color,
    attenuation: f32,
) -> Color {
    if lambert(normal, to_light) <= 0.0 {
        return Color::black();
    }

    let brillo = blinn_phong(normal, to_light, to_view, material.shininess);

    (light_color * attenuation) * (material.specular_strength * brillo)
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
    fn los_valores_por_defecto_son_validos_y_conservadores() {
        let material = Material::new(Color::new(0.5, 0.5, 0.5));

        assert!(material.is_valid());
        assert_eq!(material.albedo_texture, None, "sin textura por defecto");
        assert_eq!(material.reflection_cap, 0.0, "sin reflejo");
        assert_eq!(material.transmission_cap, 0.0, "sin refraccion");
        assert_eq!(material.ior, 1.0, "sin desviar el rayo");
        assert_eq!(material.uv_scale, 1.0);
        assert_eq!(material.shadow_mode, ShadowMode::Opaque);
        assert!((0.0..=1.0).contains(&material.specular_strength));
        assert!(material.shininess > 0.0);
    }

    #[test]
    fn los_techos_se_recortan_al_rango_legal() {
        let excesivo = Material::new(Color::black()).with_caps(1.7, -0.4, 0.3);

        assert_eq!(excesivo.reflection_cap, 1.0);
        assert_eq!(excesivo.transmission_cap, 0.0);
        // El indice de refraccion nunca baja de uno: por debajo, el vacio
        // seria mas denso que el medio y la refraccion se invertiria.
        assert_eq!(excesivo.ior, 1.0);
        assert!(excesivo.is_valid());
    }

    #[test]
    fn el_specular_se_recorta_y_el_exponente_no_se_anula() {
        let material = Material::new(Color::black()).with_specular(3.0, 0.0);

        assert_eq!(material.specular_strength, 1.0);
        // Un exponente nulo daria brillo constante en toda la superficie,
        // que no es un brillo.
        assert!(material.shininess >= 1.0);
        assert!(material.is_valid());

        let negativo = Material::new(Color::black()).with_specular(-2.0, -8.0);
        assert_eq!(negativo.specular_strength, 0.0);
        assert!(negativo.is_valid());
    }

    #[test]
    fn la_escala_uv_no_puede_ser_negativa() {
        assert_eq!(
            Material::new(Color::black()).with_uv_scale(-3.0).uv_scale,
            0.0
        );
        assert_eq!(
            Material::new(Color::black()).with_uv_scale(6.0).uv_scale,
            6.0
        );
    }

    #[test]
    fn ningun_material_del_proyecto_devuelve_mas_energia_de_la_que_recibe() {
        // Los techos son fracciones de energia. Con ambos en rango, el
        // reparto de Fresnel nunca pasa de uno para ningun angulo.
        let materiales = [
            Material::new(Color::black()),
            Material::wet_basalt(Color::black()),
            Material::new(Color::black()).with_caps(0.9, 0.9, 1.333),
            Material::new(Color::black()).with_caps(0.35, 0.25, 1.45),
        ];

        for material in materiales {
            assert!(material.is_valid());
            assert!(
                material.max_energy() <= 1.0,
                "energia {} para {material:?}",
                material.max_energy()
            );

            // Comprobacion directa del reparto, barriendo Fresnel.
            for paso in 0..=20 {
                let f = paso as f32 / 20.0;
                let kr = material.reflection_cap * f;
                let kt = material.transmission_cap * (1.0 - f);

                assert!(kr + kt <= 1.0 + 1e-6, "kr + kt = {} con F = {f}", kr + kt);
            }
        }
    }

    #[test]
    fn with_texture_pone_el_tinte_en_blanco() {
        // Las texturas del proyecto ya llevan el color del material;
        // multiplicarlas por un tinte de color lo oscureceria dos veces.
        let material = Material::new(Color::from_srgb(0.3, 0.5, 0.2)).with_texture(TextureId(0));

        assert_eq!(material.albedo_texture, Some(TextureId(0)));
        assert_eq!(material.albedo, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn with_tint_despues_de_with_texture_permite_variantes() {
        // Es como el inventario resuelve la cadena del ancla: la textura de
        // basalto con otro tinte y otra escala UV, sin un sexto material.
        let cadena = Material::wet_basalt(Color::black())
            .with_texture(TextureId(2))
            .with_tint(Color::from_srgb(0.55, 0.56, 0.58))
            .with_uv_scale(4.0);

        assert_eq!(cadena.albedo_texture, Some(TextureId(2)));
        assert_ne!(cadena.albedo, Color::new(1.0, 1.0, 1.0));
        assert_eq!(cadena.uv_scale, 4.0);
        // Y sigue sin reflejar: su brillo es local.
        assert_eq!(cadena.reflection_cap, 0.0);
    }

    #[test]
    fn el_albedo_texturizado_es_el_producto_del_tinte_y_la_muestra() {
        use crate::scene::Scene;
        use crate::texture::Texture;
        use nalgebra_glm::Vec2;

        let mut scene = Scene::new();
        // Textura de un solo pixel, medio gris lineal.
        let tex = Texture::from_pixels(1, 1, vec![Color::new(0.5, 0.5, 0.5)]).unwrap();
        let id = scene.add_texture(tex);

        // Sin tinte: el color es la muestra.
        let plano = Material::new(Color::black()).with_texture(id);
        let color = scene.albedo_at(&plano, &Vec2::new(0.3, 0.7));
        assert!((color.r - 0.5).abs() < 1e-6);

        // Con tinte a la mitad: el producto.
        let tenido = plano.with_tint(Color::new(0.5, 1.0, 0.0));
        let color = scene.albedo_at(&tenido, &Vec2::new(0.3, 0.7));
        assert!((color.r - 0.25).abs() < 1e-6);
        assert!((color.g - 0.5).abs() < 1e-6);
        assert!((color.b - 0.0).abs() < 1e-6);
    }

    #[test]
    fn sin_textura_el_albedo_es_el_color_plano() {
        use crate::scene::Scene;
        use nalgebra_glm::Vec2;

        let scene = Scene::new();
        let material = Material::new(Color::new(0.2, 0.4, 0.6));

        // La UV no influye cuando no hay textura.
        for uv in [
            Vec2::new(0.0, 0.0),
            Vec2::new(0.7, 0.3),
            Vec2::new(9.0, -2.0),
        ] {
            assert_eq!(scene.albedo_at(&material, &uv), material.albedo);
        }
    }

    #[test]
    fn la_escala_uv_repite_la_textura() {
        use crate::scene::Scene;
        use crate::texture::Texture;
        use nalgebra_glm::Vec2;

        let mut scene = Scene::new();
        // Dos mitades bien distintas, en horizontal.
        let tex = Texture::from_pixels(
            2,
            1,
            vec![Color::new(1.0, 0.0, 0.0), Color::new(0.0, 0.0, 1.0)],
        )
        .unwrap();
        let id = scene.add_texture(tex);

        let una_vez = Material::new(Color::black()).with_texture(id);
        let cuatro_veces = una_vez.with_uv_scale(4.0);

        // A u = 0.3, una repeticion cae en la mitad izquierda; con escala 4
        // cae en 1.2, que envuelve a 0.2: la misma mitad. A u = 0.15 con
        // escala 4 cae en 0.6: la mitad derecha.
        assert_eq!(scene.albedo_at(&una_vez, &Vec2::new(0.15, 0.5)).r, 1.0);
        assert_eq!(scene.albedo_at(&cuatro_veces, &Vec2::new(0.15, 0.5)).b, 1.0);
    }

    #[test]
    fn por_defecto_un_material_es_opaco_a_las_sombras() {
        let material = Material::new(Color::new(0.5, 0.5, 0.5));

        assert_eq!(material.shadow_mode, ShadowMode::Opaque);
        assert!(material.blocks_shadows());
    }

    #[test]
    fn ignore_no_bloquea_y_attenuate_tampoco_corta() {
        let agua = Material::new(Color::new(0.2, 0.4, 0.8)).with_shadow_mode(ShadowMode::Ignore);
        assert!(!agua.blocks_shadows());

        // Attenuate esta declarado pero fuera del MVP: no es un bloqueador
        // opaco, asi que el any-hit no puede cortar en el.
        let atenuante =
            Material::new(Color::new(0.5, 0.5, 0.5)).with_shadow_mode(ShadowMode::Attenuate);
        assert!(!atenuante.blocks_shadows());
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
