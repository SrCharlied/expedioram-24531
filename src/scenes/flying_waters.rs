//! Aguas Voladoras en nivel seguro: **58 primitivas trazables**.
//!
//! Es la región estrella y la más densa del inventario.
//!
//! | Entrada | Primitivas |
//! |---|---:|
//! | `A-01` volumen de agua | 1 |
//! | `A-02` masas del lecho | 5 |
//! | `A-03` casco del barco | 12 |
//! | `A-04` mástil y soportes | 3 |
//! | `A-05` cadena | 8 |
//! | `A-06` ancla | 3 |
//! | `A-07` kelp | 12 |
//! | `A-08` rocas submarinas | 6 |
//! | `A-11` borde roto | 8 |
//!
//! `A-09` (concha) y `A-10` (fragmentos) son opcionales y valen cero.
//!
//! Las **44 primitivas del interior** —casco, mástil, cadena, ancla, kelp y
//! rocas— son las que el preset de agua opaca oculta. Por eso ese preset
//! no puede usarse para aprobar rendimiento.

use super::{masa, Palette, WaterPreset, Xorshift32};
use crate::scene::{RevealGroup, Scene, SpatialGroupId};
use nalgebra_glm::Vec3;

const GRUPO: SpatialGroupId = SpatialGroupId::FlyingWaters;
const REVELA: RevealGroup = RevealGroup::FlyingWaters;

/// Altura de la superficie del agua sobre el ancla de la bahía.
const ALTURA_SUPERFICIE: f32 = 2.6;
/// Espesor del volumen de agua.
const ESPESOR_AGUA: f32 = 2.3;

/// Cuántas primitivas quedan dentro del volumen y desaparecen de la vista
/// cuando el agua se representa como cuboide opaco.
pub const PRIMITIVAS_INTERIORES: usize = 44;

/// Construye la región. Devuelve la altura mundial de la superficie del
/// agua, que es el parámetro `water_surface_y` de la escena.
pub fn aguas_voladoras(
    scene: &mut Scene,
    paleta: &Palette,
    ancla: Vec3,
    borde: Vec3,
    water: WaterPreset,
) -> f32 {
    let superficie = ancla.y + ALTURA_SUPERFICIE;

    lecho(scene, paleta, ancla);
    casco(scene, paleta, ancla);
    mastil(scene, paleta, ancla);
    cadena(scene, paleta, ancla);
    ancla_del_barco(scene, paleta, ancla);
    kelp(scene, paleta, ancla);
    rocas(scene, paleta, ancla);
    borde_roto(scene, paleta, borde);

    // `A-01` va al final para que su presencia o ausencia no desplace los
    // índices de lo que hay dentro.
    if water == WaterPreset::OpaqueWater {
        volumen_de_agua(scene, paleta, ancla, superficie);
    }

    superficie
}

/// `A-01` · el volumen, un **único cuboide cerrado**.
///
/// Nunca varios apilados: el aspecto rasgado del borde lo producen los
/// cuboides de terreno de `A-11` ocluyendo su cara frontal, no un AABB
/// roto. Rasgar el volumen multiplicaría las fronteras que refractar.
fn volumen_de_agua(scene: &mut Scene, paleta: &Palette, ancla: Vec3, superficie: f32) {
    masa(
        scene,
        Vec3::new(ancla.x, superficie - ESPESOR_AGUA * 0.5, ancla.z),
        Vec3::new(8.6, ESPESOR_AGUA, 5.0),
        paleta.canvas,
        paleta.water,
        GRUPO,
        REVELA,
    );
}

/// `A-02` · cinco masas de lecho.
fn lecho(scene: &mut Scene, paleta: &Palette, ancla: Vec3) {
    let masas = [
        (Vec3::new(0.0, 0.25, 0.0), Vec3::new(9.0, 0.8, 5.4)),
        (Vec3::new(-2.6, 0.60, -1.1), Vec3::new(3.2, 0.7, 2.2)),
        (Vec3::new(2.4, 0.55, 1.0), Vec3::new(2.8, 0.6, 2.0)),
        (Vec3::new(0.6, 0.70, -1.7), Vec3::new(2.2, 0.5, 1.4)),
        (Vec3::new(-3.2, 0.45, 1.5), Vec3::new(2.0, 0.5, 1.6)),
    ];

    for (offset, tamano) in masas {
        masa(
            scene,
            ancla + offset,
            tamano,
            paleta.canvas,
            paleta.wet_basalt,
            GRUPO,
            REVELA,
        );
    }
}

/// `A-03` · casco del barco, doce primitivas.
///
/// Se prioriza la silueta rota y suspendida, no la precisión naval.
fn casco(scene: &mut Scene, paleta: &Palette, ancla: Vec3) {
    let centro = ancla + Vec3::new(-0.3, 2.05, 0.2);

    // Cuerpo: cinco secciones que se estrechan hacia proa.
    for i in 0..5 {
        let t = i as f32;
        let ancho = 1.05 - 0.13 * t;

        masa(
            scene,
            centro + Vec3::new(-1.5 + t * 0.75, 0.02 * t, 0.0),
            Vec3::new(0.78, 0.52 - 0.04 * t, ancho),
            paleta.canvas,
            paleta.aged_wood,
            GRUPO,
            REVELA,
        );
    }

    // Cubierta partida en tres tramos: el hueco central es la rotura.
    for (dx, largo) in [(-1.35_f32, 1.0_f32), (0.35, 0.9), (1.45, 0.7)] {
        masa(
            scene,
            centro + Vec3::new(dx, 0.34, 0.0),
            Vec3::new(largo, 0.14, 0.92),
            paleta.canvas,
            paleta.aged_wood,
            GRUPO,
            REVELA,
        );
    }

    // Costillas expuestas por la brecha.
    for i in 0..3 {
        masa(
            scene,
            centro + Vec3::new(-0.55 + i as f32 * 0.38, 0.30, 0.0),
            Vec3::new(0.09, 0.46, 0.86),
            paleta.canvas,
            paleta.aged_wood,
            GRUPO,
            REVELA,
        );
    }

    // Popa, algo más alta.
    masa(
        scene,
        centro + Vec3::new(-1.95, 0.28, 0.0),
        Vec3::new(0.42, 0.86, 0.95),
        paleta.canvas,
        paleta.aged_wood,
        GRUPO,
        REVELA,
    );
}

/// `A-04` · mástil y sus dos soportes.
fn mastil(scene: &mut Scene, paleta: &Palette, ancla: Vec3) {
    let base = ancla + Vec3::new(-0.6, 2.35, 0.2);

    masa(
        scene,
        base + Vec3::new(0.0, 1.15, 0.0),
        Vec3::new(0.16, 2.3, 0.16),
        paleta.canvas,
        paleta.aged_wood,
        GRUPO,
        REVELA,
    );
    masa(
        scene,
        base + Vec3::new(0.0, 1.95, 0.0),
        Vec3::new(1.5, 0.11, 0.11),
        paleta.canvas,
        paleta.aged_wood,
        GRUPO,
        REVELA,
    );
    masa(
        scene,
        base + Vec3::new(0.35, 0.45, 0.0),
        Vec3::new(0.7, 0.09, 0.09),
        paleta.canvas,
        paleta.aged_wood,
        GRUPO,
        REVELA,
    );
}

/// `A-05` · ocho segmentos de cadena, del barco al ancla.
///
/// Siguen una curva suave; no se modelan eslabones. Reutiliza
/// `wet_basalt` para no crear un sexto material final: es metal, y se
/// distingue por escala UV, albedo gris y specular local. Sigue teniendo
/// `reflection_cap = 0`.
fn cadena(scene: &mut Scene, paleta: &Palette, ancla: Vec3) {
    let arriba = ancla + Vec3::new(0.9, 2.0, 0.35);
    let abajo = ancla + Vec3::new(2.1, 0.95, 0.9);

    for i in 0..8 {
        let t = (i as f32 + 0.5) / 8.0;
        // Comba: la cadena cuelga, no va recta.
        let comba = -0.35 * (t * std::f32::consts::PI).sin();
        let punto = arriba + (abajo - arriba) * t + Vec3::new(0.0, comba, 0.0);

        masa(
            scene,
            punto,
            Vec3::new(0.13, 0.13, 0.13),
            paleta.canvas,
            paleta.wet_basalt,
            GRUPO,
            REVELA,
        );
    }
}

/// `A-06` · el ancla, tres primitivas.
fn ancla_del_barco(scene: &mut Scene, paleta: &Palette, ancla: Vec3) {
    let base = ancla + Vec3::new(2.2, 0.95, 0.95);

    masa(
        scene,
        base,
        Vec3::new(0.12, 0.7, 0.12),
        paleta.canvas,
        paleta.wet_basalt,
        GRUPO,
        REVELA,
    );
    masa(
        scene,
        base + Vec3::new(0.0, -0.3, 0.0),
        Vec3::new(0.72, 0.11, 0.11),
        paleta.canvas,
        paleta.wet_basalt,
        GRUPO,
        REVELA,
    );
    masa(
        scene,
        base + Vec3::new(0.0, 0.3, 0.0),
        Vec3::new(0.34, 0.10, 0.10),
        paleta.canvas,
        paleta.wet_basalt,
        GRUPO,
        REVELA,
    );
}

/// `A-07` · doce grupos de kelp sobre el lecho.
///
/// Reutiliza `meadow` con tinte submarino; no se crea un sexto material
/// final solo para el kelp.
fn kelp(scene: &mut Scene, paleta: &Palette, ancla: Vec3) {
    let mut azar = Xorshift32::new(0x4B45_4C50);

    for _ in 0..12 {
        let alto = 0.7 + 0.9 * azar.siguiente();
        let offset = Vec3::new(3.6 * azar.simetrico(), 0.0, 2.0 * azar.simetrico());

        masa(
            scene,
            ancla + offset + Vec3::new(0.0, 0.65 + alto * 0.5, 0.0),
            Vec3::new(0.14, alto, 0.14),
            paleta.canvas,
            paleta.meadow,
            GRUPO,
            REVELA,
        );
    }
}

/// `A-08` · seis rocas submarinas.
fn rocas(scene: &mut Scene, paleta: &Palette, ancla: Vec3) {
    let mut azar = Xorshift32::new(0x524F_4341);

    for _ in 0..6 {
        let lado = 0.34 + 0.42 * azar.siguiente();
        let offset = Vec3::new(3.4 * azar.simetrico(), 0.0, 1.9 * azar.simetrico());

        masa(
            scene,
            ancla + offset + Vec3::new(0.0, 0.65 + lado * 0.5, 0.0),
            Vec3::new(lado, lado * 0.8, lado * 0.9),
            paleta.canvas,
            paleta.wet_basalt,
            GRUPO,
            REVELA,
        );
    }
}

/// `A-11` · ocho cuboides de terreno en primer plano.
///
/// Son ellos los que producen el borde rasgado, ocluyendo parcialmente la
/// cara frontal del agua. Es terreno (`wet_basalt`), no agua.
fn borde_roto(scene: &mut Scene, paleta: &Palette, borde: Vec3) {
    let mut azar = Xorshift32::new(0x0B0D_DE00);

    for i in 0..8 {
        let t = i as f32;
        let ancho = 0.85 + 0.5 * azar.siguiente();
        let altura = 2.2 + 1.0 * azar.siguiente();

        masa(
            scene,
            borde + Vec3::new(-3.5 + t * 1.0, -1.2 + altura * 0.5, 0.28 * azar.simetrico()),
            Vec3::new(ancho, altura, 1.1),
            paleta.canvas,
            paleta.wet_basalt,
            GRUPO,
            REVELA,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenes::SAFE;

    fn construir(water: WaterPreset) -> (Scene, Palette) {
        let mut scene = Scene::new();
        let paleta = Palette::registrar(&mut scene);
        aguas_voladoras(
            &mut scene,
            &paleta,
            Vec3::new(0.0, 0.0, 4.2),
            Vec3::new(0.0, 1.2, 6.6),
            water,
        );

        (scene, paleta)
    }

    #[test]
    fn con_agua_opaca_produce_las_58_del_inventario() {
        let (scene, _) = construir(WaterPreset::OpaqueWater);

        assert_eq!(scene.objects.len(), SAFE.flying_waters);
        assert_eq!(scene.objects.len(), 58);
    }

    #[test]
    fn sin_el_volumen_queda_una_primitiva_menos() {
        let (scene, _) = construir(WaterPreset::InteriorVisible);

        assert_eq!(scene.objects.len(), 57);
    }

    #[test]
    fn el_volumen_de_agua_es_uno_solo_y_va_al_final() {
        let (scene, paleta) = construir(WaterPreset::OpaqueWater);

        let de_agua: Vec<usize> = scene
            .objects
            .iter()
            .enumerate()
            .filter(|(_, o)| o.final_material == paleta.water)
            .map(|(i, _)| i)
            .collect();

        assert_eq!(de_agua.len(), 1, "un unico cuboide cerrado");
        assert_eq!(
            de_agua[0],
            scene.objects.len() - 1,
            "va al final para no desplazar los indices del interior"
        );
    }

    #[test]
    fn quitar_el_agua_no_desplaza_los_indices_del_interior() {
        let (con, _) = construir(WaterPreset::OpaqueWater);
        let (sin, _) = construir(WaterPreset::InteriorVisible);

        for (a, b) in sin.objects.iter().zip(&con.objects) {
            assert_eq!(a.primitive.bounds(), b.primitive.bounds());
        }
    }

    #[test]
    fn el_interior_son_44_primitivas() {
        // Casco 12 + mastil 3 + cadena 8 + ancla 3 + kelp 12 + rocas 6.
        assert_eq!(12 + 3 + 8 + 3 + 12 + 6, PRIMITIVAS_INTERIORES);

        let (scene, paleta) = construir(WaterPreset::InteriorVisible);
        let lecho_y_borde = 5 + 8;

        assert_eq!(
            scene.objects.len() - lecho_y_borde,
            PRIMITIVAS_INTERIORES,
            "las que el preset opaco ocultaria"
        );

        // Y ninguna de ellas es de agua.
        assert!(scene
            .objects
            .iter()
            .all(|o| o.final_material != paleta.water));
    }

    #[test]
    fn el_borde_roto_es_terreno_y_no_agua() {
        let (scene, paleta) = construir(WaterPreset::InteriorVisible);

        let de_basalto = scene
            .objects
            .iter()
            .filter(|o| o.final_material == paleta.wet_basalt)
            .count();

        // Lecho 5 + cadena 8 + ancla 3 + rocas 6 + borde 8.
        assert_eq!(de_basalto, 30);
    }

    #[test]
    fn la_generacion_es_determinista() {
        let (a, _) = construir(WaterPreset::OpaqueWater);
        let (b, _) = construir(WaterPreset::OpaqueWater);

        for (x, y) in a.objects.iter().zip(&b.objects) {
            assert_eq!(x.primitive.bounds(), y.primitive.bounds());
        }
    }
}
