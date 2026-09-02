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
use crate::material::{Material, ShadowMode};
use crate::scene::{MaterialId, RevealGroup, Scene, SpatialGroupId};
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
    if let Some(material) = material_del_volumen(scene, paleta, water) {
        volumen_de_agua(scene, paleta.canvas, material, ancla);
    }

    superficie
}

/// Caja del volumen de agua: centro y tamaño, dada el ancla de la bahía.
///
/// Vive aparte porque no la usa solo el constructor: la comprobación de que
/// el borde roto ocluye la cara frontal necesita saber dónde está esa cara,
/// y dos copias de la misma caja se desincronizan en el primer ajuste.
pub fn caja_del_volumen(ancla: Vec3) -> (Vec3, Vec3) {
    let superficie = ancla.y + ALTURA_SUPERFICIE;

    (
        Vec3::new(ancla.x, superficie - ESPESOR_AGUA * 0.5, ancla.z),
        Vec3::new(8.6, ESPESOR_AGUA, 5.0),
    )
}

/// Material con el que se inserta `A-01`, o `None` si el preset lo omite.
///
/// El control opaco **deriva del agua** en vez de ser un material nuevo:
/// conserva su albedo, su textura y su escala UV, y solo le quita la
/// óptica. Así el inventario sigue teniendo cinco materiales finales y el
/// control se diferencia de la escena real en exactamente una cosa, que es
/// lo que un control tiene que hacer.
fn material_del_volumen(
    scene: &mut Scene,
    paleta: &Palette,
    water: WaterPreset,
) -> Option<MaterialId> {
    match water {
        WaterPreset::InteriorVisible => None,
        WaterPreset::RefractiveWater => Some(paleta.water),
        WaterPreset::OpaqueWater => {
            let opaco = Material {
                reflection_cap: 0.0,
                transmission_cap: 0.0,
                // `ShadowMode::Ignore` se conserva: ver `WaterPreset`.
                shadow_mode: ShadowMode::Ignore,
                ..scene.material(paleta.water)
            };

            Some(scene.add_material(opaco))
        }
    }
}

/// `A-01` · el volumen, un **único cuboide cerrado**.
///
/// Nunca varios apilados, y esto ya no es solo una preferencia de
/// presupuesto: desde la Tarea 5.3 cada frontera cuesta un nivel de
/// recursión. Un volumen partido en tres losas gastaría los tres niveles de
/// `MAX_DEPTH` solo en atravesarse, y el interior de la bahía terminaría en
/// cielo antes de llegar al barco.
///
/// El aspecto rasgado del borde lo producen los cuboides de terreno de
/// `A-11` ocluyendo esta cara frontal, no un AABB roto.
fn volumen_de_agua(scene: &mut Scene, canvas: MaterialId, material: MaterialId, ancla: Vec3) {
    let (centro, tamano) = caja_del_volumen(ancla);

    masa(scene, centro, tamano, canvas, material, GRUPO, REVELA);
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
    use crate::cuboid::Cuboid;
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

    /// Ancla de la bahía y del borde, las mismas que usa el nivel seguro.
    const ANCLA: Vec3 = Vec3::new(0.0, 0.0, 4.2);
    const BORDE: Vec3 = Vec3::new(0.0, 1.2, 6.6);

    #[test]
    fn los_dos_presets_con_volumen_producen_las_58_del_inventario() {
        for water in [WaterPreset::RefractiveWater, WaterPreset::OpaqueWater] {
            let (scene, _) = construir(water);

            assert_eq!(scene.objects.len(), SAFE.flying_waters, "{water:?}");
            assert_eq!(scene.objects.len(), 58, "{water:?}");
        }
    }

    #[test]
    fn el_volumen_refractivo_usa_el_material_de_agua_del_inventario() {
        let (scene, paleta) = construir(WaterPreset::RefractiveWater);
        let volumen = scene.objects.last().expect("hay volumen");

        assert_eq!(volumen.final_material, paleta.water);

        let material = scene.material(volumen.final_material);
        assert_eq!(material.reflection_cap, 0.9);
        assert_eq!(material.transmission_cap, 0.9);
        assert!((material.ior - 1.333).abs() < 1e-6);
        assert_eq!(material.shadow_mode, ShadowMode::Ignore);
    }

    #[test]
    fn el_control_opaco_es_agua_sin_optica_y_nada_mas() {
        // El fallo que este test vigila: el preset "opaco" insertaba
        // `paleta.water` tal cual. Mientras `cast_ray` ignoraba los techos
        // eso daba lo mismo, pero al llegar la recursion de la Tarea 5.3 el
        // control dejo de ser un control y empezo a refractar.
        let (scene, paleta) = construir(WaterPreset::OpaqueWater);
        let volumen = scene.objects.last().expect("hay volumen");

        assert_ne!(
            volumen.final_material, paleta.water,
            "el control no puede compartir material con el agua real"
        );

        let control = scene.material(volumen.final_material);
        let agua = scene.material(paleta.water);

        assert_eq!(control.reflection_cap, 0.0, "el control refleja");
        assert_eq!(control.transmission_cap, 0.0, "el control transmite");

        // Y se diferencia del agua **solo** en eso: mismo color, misma
        // textura, misma escala UV, mismo modo de sombra.
        assert_eq!(control.albedo, agua.albedo);
        assert_eq!(control.albedo_texture, agua.albedo_texture);
        assert_eq!(control.uv_scale, agua.uv_scale);
        assert_eq!(control.shadow_mode, agua.shadow_mode);
        assert_eq!(control.specular_strength, agua.specular_strength);
    }

    #[test]
    fn el_volumen_no_se_rasga() {
        // Una sola primitiva con la caja del volumen, en los dos presets
        // que lo insertan. Partirlo en losas gastaria los tres niveles de
        // `MAX_DEPTH` solo en atravesarse.
        let (centro, tamano) = caja_del_volumen(ANCLA);
        let esperada = Cuboid::centrado(centro, tamano).bounds;

        for water in [WaterPreset::RefractiveWater, WaterPreset::OpaqueWater] {
            let (scene, _) = construir(water);

            let con_esa_caja = scene
                .objects
                .iter()
                .filter(|o| {
                    let caja = o.primitive.bounds();

                    (caja.min - esperada.min).magnitude() < 1e-5
                        && (caja.max - esperada.max).magnitude() < 1e-5
                })
                .count();

            assert_eq!(con_esa_caja, 1, "{water:?}: el volumen no es uno solo");
        }
    }

    #[test]
    fn sin_el_volumen_queda_una_primitiva_menos() {
        let (scene, _) = construir(WaterPreset::InteriorVisible);

        assert_eq!(scene.objects.len(), 57);
    }

    #[test]
    fn el_volumen_de_agua_va_al_final() {
        let (centro, tamano) = caja_del_volumen(ANCLA);
        let esperada = Cuboid::centrado(centro, tamano).bounds;

        let (scene, _) = construir(WaterPreset::RefractiveWater);
        let ultimo = scene.objects.last().expect("hay volumen");

        assert!(
            (ultimo.primitive.bounds().min - esperada.min).magnitude() < 1e-5,
            "el volumen no va al final y desplazaria los indices del interior"
        );
    }

    #[test]
    fn el_borde_roto_ocluye_parcialmente_la_cara_frontal() {
        // La cara frontal del volumen es un rectangulo limpio. El aspecto
        // rasgado sale de que los ocho cuboides de terreno de `A-11` se
        // planten delante y la tapen **a medias**: si la taparan del todo
        // no se veria el agua, y si no la taparan nada se veria una caja.
        let (centro, tamano) = caja_del_volumen(ANCLA);
        let cara_z = centro.z + tamano.z * 0.5;
        let (x0, x1) = (centro.x - tamano.x * 0.5, centro.x + tamano.x * 0.5);
        let (y0, y1) = (centro.y - tamano.y * 0.5, centro.y + tamano.y * 0.5);

        // Se construye **solo** el borde, sin el resto de la region: no se
        // puede identificar por «lo que sobresale de la cara», porque la
        // masa principal del lecho mide 5.4 de fondo contra los 5.0 del
        // volumen y tambien asoma.
        let mut scene = Scene::new();
        let paleta = Palette::registrar(&mut scene);
        borde_roto(&mut scene, &paleta, BORDE);

        let delante: Vec<_> = scene.objects.iter().map(|o| o.primitive.bounds()).collect();

        assert_eq!(delante.len(), 8, "el borde roto son ocho cuboides");
        assert!(
            delante.len() <= 10,
            "el inventario permite diez como maximo"
        );

        // Cada uno atraviesa el plano de la cara: delante y detras a la vez.
        for caja in &delante {
            assert!(
                caja.min.z < cara_z && caja.max.z > cara_z,
                "un cuboide del borde no cruza la cara frontal: {:?}",
                caja
            );
        }

        // Cobertura muestreada de la cara. Medida: 88.7 %.
        let pasos = 120;
        let mut cubiertos = 0;

        for i in 0..pasos {
            let x = x0 + (i as f32 + 0.5) / pasos as f32 * (x1 - x0);

            for j in 0..pasos {
                let y = y0 + (j as f32 + 0.5) / pasos as f32 * (y1 - y0);

                if delante
                    .iter()
                    .any(|c| x >= c.min.x && x <= c.max.x && y >= c.min.y && y <= c.max.y)
                {
                    cubiertos += 1;
                }
            }
        }

        let cobertura = cubiertos as f32 / (pasos * pasos) as f32;

        assert!(
            (0.55..0.95).contains(&cobertura),
            "la oclusion de la cara frontal salio del rango parcial: {:.1} %",
            cobertura * 100.0
        );
    }

    #[test]
    fn quitar_el_agua_no_desplaza_los_indices_del_interior() {
        let (con, _) = construir(WaterPreset::RefractiveWater);
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
