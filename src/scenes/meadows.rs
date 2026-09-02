//! Praderas Primaverales en nivel seguro: **37 primitivas trazables**.
//!
//! | Entrada | Primitivas |
//! |---|---:|
//! | `P-01` meseta principal | 6 |
//! | `P-02` frente de la cascada | 8 |
//! | `P-03` superficies de césped | 4 |
//! | `P-04` árboles simplificados | 6 |
//! | `P-05` cascada | 1 |
//! | `P-07` grupos de flores | 12 |
//!
//! `P-06` (ruinas) y `P-08` (rocas flotantes) son opcionales y valen cero
//! en nivel seguro.

use super::{masa, Palette, Xorshift32};
use crate::scene::{RevealGroup, Scene, SpatialGroupId};
use nalgebra_glm::Vec3;

const GRUPO: SpatialGroupId = SpatialGroupId::Meadows;
const REVELA: RevealGroup = RevealGroup::Meadows;

/// Construye la región completa. Devuelve el número de primitivas creadas.
pub fn praderas(scene: &mut Scene, paleta: &Palette, ancla: Vec3) -> usize {
    let antes = scene.objects.len();

    meseta(scene, paleta, ancla);
    frente_de_cascada(scene, paleta, ancla);
    cesped(scene, paleta, ancla);
    arboles(scene, paleta, ancla);
    cascada(scene, paleta, ancla);
    flores(scene, paleta, ancla);

    scene.objects.len() - antes
}

/// `P-01` · seis masas que forman la meseta y sus terrazas.
fn meseta(scene: &mut Scene, paleta: &Palette, ancla: Vec3) {
    let masas = [
        (Vec3::new(0.0, -0.70, 0.0), Vec3::new(7.0, 1.4, 5.6)),
        (Vec3::new(-0.8, 0.40, -0.8), Vec3::new(4.4, 1.0, 3.4)),
        (Vec3::new(1.9, 0.10, 0.9), Vec3::new(2.6, 0.8, 2.4)),
        (Vec3::new(-2.4, 0.05, 1.4), Vec3::new(2.2, 0.7, 2.0)),
        (Vec3::new(0.4, 0.95, -1.6), Vec3::new(2.8, 0.6, 1.8)),
        (Vec3::new(-1.6, -0.20, -2.1), Vec3::new(3.0, 0.9, 1.6)),
    ];

    for (offset, tamano) in masas {
        masa(scene, ancla + offset, tamano, paleta.meadow, GRUPO, REVELA);
    }
}

/// `P-02` · el frente oscuro por el que cae la cascada.
///
/// Pertenece a Praderas y **no** duplica el muro de contención de
/// Rompeolas: son dos caídas distintas del terreno.
fn frente_de_cascada(scene: &mut Scene, paleta: &Palette, ancla: Vec3) {
    for i in 0..8 {
        let t = i as f32;
        let altura = 1.6 + 0.5 * (t * 0.9).sin();

        masa(
            scene,
            ancla + Vec3::new(-2.6 + t * 0.78, -1.4 - altura * 0.5, 2.7 + 0.18 * t.cos()),
            Vec3::new(0.9, altura, 0.9),
            paleta.wet_basalt,
            GRUPO,
            REVELA,
        );
    }
}

/// `P-03` · cuatro láminas de césped sobre la meseta.
fn cesped(scene: &mut Scene, paleta: &Palette, ancla: Vec3) {
    let laminas = [
        (Vec3::new(-1.2, 0.06, 0.4), Vec3::new(4.2, 0.12, 3.0)),
        (Vec3::new(1.7, 0.56, 0.8), Vec3::new(2.2, 0.12, 2.0)),
        (Vec3::new(-0.9, 0.96, -1.5), Vec3::new(2.4, 0.12, 1.5)),
        (Vec3::new(-2.3, 0.46, 1.3), Vec3::new(1.9, 0.12, 1.7)),
    ];

    for (offset, tamano) in laminas {
        masa(scene, ancla + offset, tamano, paleta.meadow, GRUPO, REVELA);
    }
}

/// `P-04` · dos árboles de tres primitivas cada uno.
///
/// El inventario cuenta primitivas, no árboles: tres por árbol es el techo
/// declarado, así que seis primitivas son exactamente dos árboles.
fn arboles(scene: &mut Scene, paleta: &Palette, ancla: Vec3) {
    for (base, escala) in [
        (Vec3::new(-2.0, 0.10, -0.6), 1.0_f32),
        (Vec3::new(1.4, 0.60, -0.9), 0.8),
    ] {
        // Tronco.
        masa(
            scene,
            ancla + base + Vec3::new(0.0, 0.55 * escala, 0.0),
            Vec3::new(0.22 * escala, 1.1 * escala, 0.22 * escala),
            paleta.aged_wood,
            GRUPO,
            REVELA,
        );
        // Dos masas de copa, desalineadas para que no parezca un poste.
        masa(
            scene,
            ancla + base + Vec3::new(0.05 * escala, 1.35 * escala, 0.0),
            Vec3::new(1.15 * escala, 0.75 * escala, 1.15 * escala),
            paleta.meadow,
            GRUPO,
            REVELA,
        );
        masa(
            scene,
            ancla + base + Vec3::new(-0.10 * escala, 1.85 * escala, 0.08 * escala),
            Vec3::new(0.75 * escala, 0.55 * escala, 0.75 * escala),
            paleta.meadow,
            GRUPO,
            REVELA,
        );
    }
}

/// `P-05` · la cascada, en **un solo** volumen alargado.
///
/// El inventario lo restringe expresamente: uno o dos volúmenes largos,
/// nunca una pila de cubos transparentes. Cada cubo apilado sería otra
/// frontera que refractar en el Hito 5.
fn cascada(scene: &mut Scene, paleta: &Palette, ancla: Vec3) {
    masa(
        scene,
        ancla + Vec3::new(0.2, -1.5, 2.9),
        Vec3::new(1.8, 3.2, 0.5),
        paleta.water,
        GRUPO,
        REVELA,
    );
}

/// `P-07` · doce grupos de flores, dispersos con semilla fija.
fn flores(scene: &mut Scene, paleta: &Palette, ancla: Vec3) {
    let mut azar = Xorshift32::new(0x00F1_0BE5);

    for _ in 0..12 {
        let offset = Vec3::new(2.9 * azar.simetrico(), 0.16, 2.2 * azar.simetrico());
        let lado = 0.14 + 0.10 * azar.siguiente();

        masa(
            scene,
            ancla + offset,
            Vec3::new(lado, lado * 1.4, lado),
            paleta.meadow,
            GRUPO,
            REVELA,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenes::SAFE;

    #[test]
    fn praderas_produce_las_37_primitivas_del_inventario() {
        let mut scene = Scene::new();
        let paleta = Palette::registrar(&mut scene);

        let creadas = praderas(&mut scene, &paleta, Vec3::new(-4.2, 5.6, -4.6));

        assert_eq!(creadas, SAFE.meadows);
        assert_eq!(creadas, 37);
    }

    #[test]
    fn todo_queda_en_el_grupo_de_praderas() {
        let mut scene = Scene::new();
        let paleta = Palette::registrar(&mut scene);
        praderas(&mut scene, &paleta, Vec3::zeros());

        for objeto in &scene.objects {
            assert_eq!(objeto.spatial_group, SpatialGroupId::Meadows);
            assert_eq!(objeto.reveal_group, RevealGroup::Meadows);
        }
    }

    #[test]
    fn la_cascada_es_un_solo_volumen_de_agua() {
        let mut scene = Scene::new();
        let paleta = Palette::registrar(&mut scene);
        praderas(&mut scene, &paleta, Vec3::zeros());

        let de_agua = scene
            .objects
            .iter()
            .filter(|o| o.final_material == paleta.water)
            .count();

        assert_eq!(de_agua, 1, "el inventario prohibe apilar cubos de agua");
    }

    #[test]
    fn la_generacion_es_determinista() {
        let construir = || {
            let mut scene = Scene::new();
            let paleta = Palette::registrar(&mut scene);
            praderas(&mut scene, &paleta, Vec3::zeros());
            scene
        };

        for (a, b) in construir().objects.iter().zip(&construir().objects) {
            assert_eq!(a.primitive.bounds(), b.primitive.bounds());
        }
    }
}
