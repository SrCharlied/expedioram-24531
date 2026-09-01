//! Luces puntuales, atenuación y *light linking*.
//!
//! Las tres luces del diorama se definen respecto de las anclas y de
//! `scene_radius`, nunca en unidades absolutas: así la iluminación sobrevive
//! a cualquier reescalado del blockout sin recalibrarse.

use crate::color::Color;
use crate::scene::SpatialGroupId;
use crate::scene_builder::{SceneAnchors, SceneScale};
use nalgebra_glm::Vec3;

/// Conjunto de grupos espaciales, como máscara de bits.
///
/// Es lo que implementa el *light linking*: una luz puede declarar a qué
/// grupos ilumina y qué grupos pueden bloquearla. Se comprueba **antes** de
/// evaluar la iluminación y antes de lanzar el rayo de sombra, así que un
/// grupo excluido no cuesta nada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupMask(u8);

impl GroupMask {
    /// Los siete grupos. Es el caso por defecto: una luz normal ilumina
    /// todo lo que alcanza.
    pub const ALL: GroupMask = GroupMask(0b0111_1111);

    pub const NONE: GroupMask = GroupMask(0);

    pub fn only(grupos: &[SpatialGroupId]) -> Self {
        let mut bits = 0;
        for grupo in grupos {
            bits |= 1 << grupo.index();
        }

        GroupMask(bits)
    }

    pub fn contains(self, grupo: SpatialGroupId) -> bool {
        self.0 & (1 << grupo.index()) != 0
    }
}

/// Luz puntual con atenuación cuadrática normalizada.
#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    /// Identificador del inventario: `L-01`, `L-02`, `L-03`.
    pub id: &'static str,
    pub position: Vec3,
    pub color: Color,
    pub intensity: f32,
    /// Distancia a la que la contribución cae a la mitad.
    pub range: f32,
    pub casts_shadows: bool,
    /// Qué grupos ilumina.
    pub affected_groups: GroupMask,
    /// Qué grupos pueden bloquearla.
    pub occluder_groups: GroupMask,
}

impl PointLight {
    /// Atenuación a una distancia dada.
    ///
    /// ```text
    /// attenuation(d) = intensity / (1 + (d / range)²)
    /// ```
    ///
    /// La forma normalizada, y no el `1/d²` puro, por dos razones: en `d = 0`
    /// el inverso del cuadrado diverge, y `range` da un mando legible —la
    /// distancia a la que la contribución cae justo a la mitad— en vez de
    /// obligar a elegir intensidades a ojo.
    pub fn attenuation(&self, distance: f32) -> f32 {
        if self.range <= 0.0 {
            return 0.0;
        }

        let relativa = distance / self.range;

        self.intensity / (1.0 + relativa * relativa)
    }

    /// Color que aporta la luz a esa distancia, ya atenuado.
    pub fn contribution(&self, distance: f32) -> Color {
        self.color * self.attenuation(distance)
    }

    pub fn distance_to(&self, punto: &Vec3) -> f32 {
        (self.position - punto).magnitude()
    }

    /// ¿Esta luz ilumina a un objeto de ese grupo?
    ///
    /// Se consulta antes de evaluar nada. Para `L-02` la respuesta es «no»
    /// fuera de Aguas Voladoras, y eso ahorra tanto el sombreado como el
    /// rayo de sombra.
    pub fn affects(&self, grupo: SpatialGroupId) -> bool {
        self.affected_groups.contains(grupo)
    }

    /// ¿Un objeto de ese grupo puede proyectar sombra de esta luz?
    ///
    /// Filtro separado del anterior a propósito. Si `L-02` iluminara solo
    /// Aguas pero cualquier grupo pudiera bloquearla, Praderas proyectaría
    /// sombras de una luz que no la ilumina, y el rayo de sombra recorrería
    /// grupos que no pueden aportar nada.
    pub fn can_be_occluded_by(&self, grupo: SpatialGroupId) -> bool {
        self.casts_shadows && self.occluder_groups.contains(grupo)
    }
}

/// Sol cálido: la luz principal que separa terrazas, barco y pilares.
pub const WARM_SUN: Color = Color {
    r: 1.0,
    g: 0.92,
    b: 0.78,
};

/// Azul frío de la bahía.
pub const COOL_BLUE: Color = Color {
    r: 0.42,
    g: 0.62,
    b: 1.0,
};

/// Cian pictórico del Monolito.
pub const PICTORIAL_CYAN: Color = Color {
    r: 0.55,
    g: 0.95,
    b: 1.0,
};

/// Las tres luces del inventario, colocadas contra las anclas y la escala
/// medidas del blockout.
///
/// Los valores de `L-02` son **provisionales**: el inventario los marca
/// `calibration: provisional_until_blockout_4`. La Tarea 5.7 mide las
/// distancias reales al barco y recalcula `range` e `intensity`; heredarlos
/// sin medir es exactamente lo que el plan prohíbe.
pub fn diorama(anchors: &SceneAnchors, scale: &SceneScale) -> Vec<PointLight> {
    let s = scale.scene_radius;

    vec![
        // L-01 · luz principal cálida.
        PointLight {
            id: "L-01",
            position: anchors.monolith_base_anchor + Vec3::new(-0.8 * s, 1.2 * s, 0.6 * s),
            color: WARM_SUN,
            intensity: 1.0,
            range: 2.5 * s,
            casts_shadows: true,
            affected_groups: GroupMask::ALL,
            occluder_groups: GroupMask::ALL,
        },
        // L-02 · azul de Aguas Voladoras, confinada por light linking.
        PointLight {
            id: "L-02",
            position: anchors.flying_waters_anchor + Vec3::new(0.0, 0.15 * s, 0.10 * s),
            color: COOL_BLUE,
            intensity: 2.0,
            range: 0.20 * s,
            casts_shadows: true,
            affected_groups: GroupMask::only(&[SpatialGroupId::FlyingWaters]),
            occluder_groups: GroupMask::only(&[SpatialGroupId::FlyingWaters]),
        },
        // L-03 · acento del Monolito. Opcional y sin sombras.
        PointLight {
            id: "L-03",
            position: anchors.monolith_base_anchor + Vec3::new(0.0, 0.5 * s, -0.25 * s),
            color: PICTORIAL_CYAN,
            intensity: 0.8,
            range: 0.4 * s,
            casts_shadows: false,
            affected_groups: GroupMask::ALL,
            occluder_groups: GroupMask::NONE,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenes::continent::blockout;

    fn luz(intensity: f32, range: f32) -> PointLight {
        PointLight {
            id: "prueba",
            position: Vec3::zeros(),
            color: Color::new(1.0, 1.0, 1.0),
            intensity,
            range,
            casts_shadows: true,
            affected_groups: GroupMask::ALL,
            occluder_groups: GroupMask::ALL,
        }
    }

    fn cerca(a: f32, b: f32, tol: f32) -> bool {
        (a - b).abs() < tol
    }

    #[test]
    fn a_distancia_cero_la_atenuacion_es_la_intensidad() {
        assert!(cerca(luz(1.0, 5.0).attenuation(0.0), 1.0, 1e-6));
        assert!(cerca(luz(2.5, 0.3).attenuation(0.0), 2.5, 1e-6));
    }

    #[test]
    fn a_distancia_range_la_atenuacion_cae_a_la_mitad() {
        for (intensidad, rango) in [(1.0, 5.0), (2.0, 0.2), (0.8, 12.5)] {
            let l = luz(intensidad, rango);

            assert!(
                cerca(l.attenuation(rango), intensidad / 2.0, 1e-5),
                "intensidad {intensidad}, rango {rango}"
            );
        }
    }

    #[test]
    fn la_atenuacion_decrece_y_nunca_diverge() {
        let l = luz(1.0, 1.0);
        let mut anterior = l.attenuation(0.0);

        for paso in 1..50 {
            let actual = l.attenuation(paso as f32 * 0.5);

            assert!(actual < anterior, "no decrece en el paso {paso}");
            assert!(actual.is_finite() && actual >= 0.0);
            anterior = actual;
        }
    }

    #[test]
    fn un_range_no_positivo_no_ilumina_en_vez_de_dividir_entre_cero() {
        assert_eq!(luz(1.0, 0.0).attenuation(1.0), 0.0);
        assert_eq!(luz(1.0, -3.0).attenuation(1.0), 0.0);
    }

    #[test]
    fn reproduce_la_tabla_de_calibracion_de_l02_del_inventario() {
        // Con scene_radius = 1, las distancias se leen como multiplos de S.
        // Los cuatro valores y las dos caidas estan tabulados en el
        // inventario; si el codigo se desvia, este test lo detecta.
        let ancho = luz(2.0, 0.55);
        let estrecho = luz(2.0, 0.20);

        assert!(cerca(ancho.attenuation(0.15), 1.8615, 1e-4));
        assert!(cerca(ancho.attenuation(0.25), 1.6575, 1e-4));
        assert!(cerca(estrecho.attenuation(0.15), 1.2800, 1e-4));
        assert!(cerca(estrecho.attenuation(0.25), 0.7805, 1e-4));

        let caida = |l: &PointLight| {
            let cerca_ = l.attenuation(0.15);
            100.0 * (cerca_ - l.attenuation(0.25)) / cerca_
        };

        assert!(cerca(caida(&ancho), 10.96, 0.01));
        assert!(cerca(caida(&estrecho), 39.02, 0.01));

        // El rango estrecho es 3.56 veces mas sensible, no cinco.
        assert!(cerca(caida(&estrecho) / caida(&ancho), 3.56, 0.01));
    }

    #[test]
    fn sin_linking_la_atenuacion_sola_no_confina_la_luz() {
        // La cifra que justifica el light linking: a 0.45S, Praderas
        // seguiria recibiendo el 25.77% de lo que recibe el barco a 0.15S.
        // Un tinte azul de esa magnitud es perfectamente visible.
        let l = luz(2.0, 0.20);
        let relativo = 100.0 * l.attenuation(0.45) / l.attenuation(0.15);

        assert!(cerca(relativo, 25.77, 0.01), "{relativo}");
    }

    #[test]
    fn l02_ignora_receptores_fuera_de_aguas_voladoras() {
        let diorama_ = blockout();
        let luces = diorama(&diorama_.anchors, &diorama_.scale);
        let l02 = luces.iter().find(|l| l.id == "L-02").expect("existe L-02");

        assert!(l02.affects(SpatialGroupId::FlyingWaters));

        for grupo in SpatialGroupId::ALL {
            if grupo == SpatialGroupId::FlyingWaters {
                continue;
            }

            assert!(!l02.affects(grupo), "L-02 no debe iluminar {grupo:?}");
        }
    }

    #[test]
    fn l02_solo_consulta_oclusores_de_aguas_voladoras() {
        let diorama_ = blockout();
        let luces = diorama(&diorama_.anchors, &diorama_.scale);
        let l02 = luces.iter().find(|l| l.id == "L-02").expect("existe L-02");

        assert!(l02.can_be_occluded_by(SpatialGroupId::FlyingWaters));

        for grupo in SpatialGroupId::ALL {
            if grupo == SpatialGroupId::FlyingWaters {
                continue;
            }

            assert!(
                !l02.can_be_occluded_by(grupo),
                "Praderas no debe proyectar sombras de una luz que no la ilumina: {grupo:?}"
            );
        }
    }

    #[test]
    fn l01_ilumina_y_puede_ser_bloqueada_por_todo() {
        let diorama_ = blockout();
        let luces = diorama(&diorama_.anchors, &diorama_.scale);
        let l01 = luces.iter().find(|l| l.id == "L-01").expect("existe L-01");

        for grupo in SpatialGroupId::ALL {
            assert!(l01.affects(grupo));
            assert!(l01.can_be_occluded_by(grupo));
        }
    }

    #[test]
    fn l03_ilumina_pero_no_proyecta_sombras() {
        let diorama_ = blockout();
        let luces = diorama(&diorama_.anchors, &diorama_.scale);
        let l03 = luces.iter().find(|l| l.id == "L-03").expect("existe L-03");

        assert!(!l03.casts_shadows);

        for grupo in SpatialGroupId::ALL {
            assert!(l03.affects(grupo), "L-03 si ilumina {grupo:?}");
            assert!(
                !l03.can_be_occluded_by(grupo),
                "una luz sin sombras no consulta oclusores"
            );
        }
    }

    #[test]
    fn las_luces_se_colocan_relativas_a_la_escala_medida() {
        let diorama_ = blockout();
        let luces = diorama(&diorama_.anchors, &diorama_.scale);
        let s = diorama_.scale.scene_radius;

        assert_eq!(luces.len(), 3);

        let l01 = &luces[0];
        assert!(cerca(l01.range, 2.5 * s, 1e-3));
        assert!(cerca(
            l01.position.y - diorama_.anchors.monolith_base_anchor.y,
            1.2 * s,
            1e-3
        ));

        let l02 = &luces[1];
        assert!(cerca(l02.range, 0.20 * s, 1e-3));
        // Anclada sobre la superficie del agua, no sobre el origen.
        assert!(cerca(
            l02.position.z - diorama_.anchors.flying_waters_anchor.z,
            0.10 * s,
            1e-3
        ));
    }

    #[test]
    fn la_mascara_de_grupos_distingue_los_siete() {
        assert_eq!(GroupMask::NONE, GroupMask::only(&[]));

        for grupo in SpatialGroupId::ALL {
            assert!(GroupMask::ALL.contains(grupo));
            assert!(!GroupMask::NONE.contains(grupo));

            let sola = GroupMask::only(&[grupo]);
            assert!(sola.contains(grupo));

            for otro in SpatialGroupId::ALL {
                if otro != grupo {
                    assert!(!sola.contains(otro), "{grupo:?} colisiona con {otro:?}");
                }
            }
        }
    }

    #[test]
    fn la_contribucion_escala_el_color_de_la_luz() {
        let l = PointLight {
            color: Color::new(0.4, 0.6, 1.0),
            ..luz(2.0, 1.0)
        };

        // A distancia range la atenuacion es intensity/2 = 1.0.
        let c = l.contribution(1.0);

        assert!(cerca(c.r, 0.4, 1e-5));
        assert!(cerca(c.g, 0.6, 1e-5));
        assert!(cerca(c.b, 1.0, 1e-5));
    }
}
