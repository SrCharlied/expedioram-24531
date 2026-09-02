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
use crate::color::Color;
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

/// Aplica un factor multiplicativo al color de un material, conservando su
/// textura.
///
/// Es la única forma de teñir que se comporta igual con textura y sin ella,
/// y la distinción no es teórica: `with_tint` **reemplaza** el albedo.
/// Sobre un material texturizado —cuyo albedo es blanco por diseño, para no
/// oscurecer dos veces— reemplazar equivale a multiplicar la muestra. Pero
/// sobre uno de color plano sustituye el color, y entonces el «tinte» deja
/// de teñir: pasa a ser el color entero.
///
/// El proyecto corre en los dos modos —con assets y con `--no-textures`, que
/// es lo que usan todos los tests—, así que un tinte absoluto daría dos
/// materiales distintos según hubiera texturas cargadas. Multiplicar el
/// albedo que el material ya tiene da el mismo resultado en los dos casos.
///
/// El factor va en **lineal**: es una atenuación de energía por canal, no un
/// color elegido a ojo.
fn tenir(material: Material, factor: Color) -> Material {
    let albedo = material.albedo * factor;

    material.with_tint(albedo)
}

/// Metal de la cadena y del ancla: `wet_basalt` **reusado**.
///
/// El inventario lo pide así para no crear un sexto material final. Tres
/// cosas lo separan del basalto del acantilado, y ninguna cuesta una
/// entrada de paleta:
///
/// - **Tinte frío**: atenúa el rojo y deja el azul intacto. El resultado es
///   un gris de acero, algo más oscuro que la roca y claramente menos
///   cálido. Va como factor multiplicativo, no como color absoluto; ver
///   `tenir`.
/// - **Escala UV de `12.0`**, cuatro veces la del basalto. Los eslabones
///   miden `0.13`: con la escala del acantilado la textura no alcanzaría a
///   repetir ni una vez sobre una cara y el metal se vería plano.
/// - **Brillo más estrecho.** No más fuerte: `wet_basalt` ya viene con
///   `specular_strength = 0.85`, porque la roca mojada brilla mucho. Lo que
///   separa al metal es el **tamaño del lóbulo**: `shininess 220` contra
///   `96`, un punto de luz pequeño e intenso en vez de un brillo extendido.
///
/// `reflection_cap` sigue en cero, heredado del basalto: la cadena **no
/// lanza rayos**. Son once primitivas pequeñas dentro del volumen de agua,
/// y cada una reflejando costaría un nivel de recursión de los tres que
/// hay, justo donde el rayo ya gastó dos en entrar.
fn metal_reusado(scene: &mut Scene, paleta: &Palette) -> MaterialId {
    let base = scene.material(paleta.wet_basalt);
    let metal = tenir(base, Color::new(0.70, 0.78, 1.00))
        .with_uv_scale(12.0)
        .with_specular(0.80, 220.0);

    scene.add_material(metal)
}

/// `A-05` · ocho segmentos de cadena, del barco al ancla.
///
/// Siguen una curva suave; no se modelan eslabones. El material es el metal
/// reusado de `metal_reusado`, no el basalto a secas.
fn cadena(scene: &mut Scene, paleta: &Palette, ancla: Vec3) {
    let metal = metal_reusado(scene, paleta);
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
            metal,
            GRUPO,
            REVELA,
        );
    }
}

/// `A-06` · el ancla, tres primitivas.
///
/// Mismo metal que la cadena: es la pieza a la que la cadena llega, y dos
/// grises distintos ahí romperían la lectura de una sola cadena continua.
fn ancla_del_barco(scene: &mut Scene, paleta: &Palette, ancla: Vec3) {
    let metal = metal_reusado(scene, paleta);
    let base = ancla + Vec3::new(2.2, 0.95, 0.95);

    masa(
        scene,
        base,
        Vec3::new(0.12, 0.7, 0.12),
        paleta.canvas,
        metal,
        GRUPO,
        REVELA,
    );
    masa(
        scene,
        base + Vec3::new(0.0, -0.3, 0.0),
        Vec3::new(0.72, 0.11, 0.11),
        paleta.canvas,
        metal,
        GRUPO,
        REVELA,
    );
    masa(
        scene,
        base + Vec3::new(0.0, 0.3, 0.0),
        Vec3::new(0.34, 0.10, 0.10),
        paleta.canvas,
        metal,
        GRUPO,
        REVELA,
    );
}

/// Verde submarino del kelp: `meadow` **reusado**.
///
/// Igual que el metal de la cadena, y por la misma razón: el inventario
/// limita el proyecto a cinco materiales finales.
///
/// El tinte **corta el rojo** y conserva verde y azul. Eso no es una
/// preferencia de paleta, es lo que hace el agua: absorbe primero las
/// longitudes de onda largas, así que a un metro de profundidad lo primero
/// que se pierde es el rojo. Un césped al que se le quita el rojo se lee
/// submarino sin necesidad de un material nuevo.
///
/// Ojo con la dirección del tinte: el factor solo puede **quitar**, nunca
/// añadir. La textura de pradera tiene el azul como canal más bajo, y
/// ningún factor azulado la volvería turquesa. Lo que sí funciona es
/// atenuar el rojo hasta que el azul lo supere; ahí el verde vira solo.
///
/// `ShadowMode::Ignore`, además: son doce frondas delgadas dentro de la
/// bahía, y sombras duras proyectadas por doce palos motearían el lecho con
/// un patrón que nadie lee como sombra de kelp. Cuesta además un rayo de
/// sombra por fronda y por luz para producir ese moteado.
fn verde_submarino(scene: &mut Scene, paleta: &Palette) -> MaterialId {
    let base = scene.material(paleta.meadow);
    let kelp = tenir(base, Color::new(0.30, 0.85, 1.00)).with_shadow_mode(ShadowMode::Ignore);

    scene.add_material(kelp)
}

/// `A-07` · doce grupos de kelp sobre el lecho.
///
/// Reutiliza `meadow` con el tinte submarino de `verde_submarino`; no se
/// crea un sexto material final solo para el kelp.
fn kelp(scene: &mut Scene, paleta: &Palette, ancla: Vec3) {
    let verde = verde_submarino(scene, paleta);
    let mut azar = Xorshift32::new(0x4B45_4C50);

    for _ in 0..12 {
        let alto = 0.7 + 0.9 * azar.siguiente();
        let offset = Vec3::new(3.6 * azar.simetrico(), 0.0, 2.0 * azar.simetrico());

        masa(
            scene,
            ancla + offset + Vec3::new(0.0, 0.65 + alto * 0.5, 0.0),
            Vec3::new(0.14, alto, 0.14),
            paleta.canvas,
            verde,
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

        // Lecho 5 + rocas 6 + borde 8. La cadena y el ancla ya **no**
        // cuentan: usan el metal reusado, que es otro `MaterialId`.
        assert_eq!(de_basalto, 19);
    }

    #[test]
    fn la_generacion_es_determinista() {
        let (a, _) = construir(WaterPreset::OpaqueWater);
        let (b, _) = construir(WaterPreset::OpaqueWater);

        for (x, y) in a.objects.iter().zip(&b.objects) {
            assert_eq!(x.primitive.bounds(), y.primitive.bounds());
        }
    }

    /// Construye una sola entrada en una escena vacía y devuelve cuántas
    /// primitivas produjo, con la paleta para poder inspeccionarlas.
    fn solo(entrada: fn(&mut Scene, &Palette, Vec3)) -> (Scene, Palette) {
        let mut scene = Scene::new();
        let paleta = Palette::registrar(&mut scene);
        entrada(&mut scene, &paleta, ANCLA);

        (scene, paleta)
    }

    #[test]
    fn cada_entrada_del_barco_respeta_su_presupuesto() {
        // Los cuatro números del plan para la Tarea 5.5, entrada por
        // entrada y no solo en el total: un casco que se pase de largo y un
        // mástil que se quede corto se cancelarían en la suma.
        for (nombre, entrada, esperado) in [
            ("A-03 casco", casco as fn(&mut Scene, &Palette, Vec3), 12),
            ("A-04 mastil", mastil, 3),
            ("A-05 cadena", cadena, 8),
            ("A-06 ancla", ancla_del_barco, 3),
        ] {
            let (scene, _) = solo(entrada);

            assert_eq!(scene.objects.len(), esperado, "{nombre}");
        }
    }

    #[test]
    fn el_casco_se_lee_como_silueta_y_no_como_bloque() {
        // «Se prioriza la silueta, no la precisión naval». Lo que hace que
        // se lea como un casco roto: se estrecha hacia proa, la cubierta va
        // partida en tres con hueco en medio, y la popa es la pieza más
        // alta.
        let (scene, _) = solo(casco);
        let cajas: Vec<_> = scene.objects.iter().map(|o| o.primitive.bounds()).collect();

        // Se estrecha: el ancho en Z de las cinco secciones del cuerpo
        // decrece de forma monótona.
        let cuerpo: Vec<f32> = cajas[..5].iter().map(|c| c.max.z - c.min.z).collect();
        for par in cuerpo.windows(2) {
            assert!(
                par[1] < par[0],
                "el cuerpo no se estrecha hacia proa: {cuerpo:?}"
            );
        }

        // La popa es la pieza más alta del casco.
        let mas_alta = cajas
            .iter()
            .map(|c| c.max.y - c.min.y)
            .fold(0.0_f32, f32::max);
        let popa = cajas.last().expect("hay popa");
        assert!(
            (popa.max.y - popa.min.y - mas_alta).abs() < 1e-6,
            "la popa deberia ser la pieza mas alta"
        );

        // Y la silueta no es una caja: el casco es claramente más largo que
        // ancho.
        let largo = cajas.iter().map(|c| c.max.x).fold(f32::MIN, f32::max)
            - cajas.iter().map(|c| c.min.x).fold(f32::MAX, f32::min);
        let ancho = cajas.iter().map(|c| c.max.z).fold(f32::MIN, f32::max)
            - cajas.iter().map(|c| c.min.z).fold(f32::MAX, f32::min);

        assert!(largo > ancho * 3.0, "largo {largo} contra ancho {ancho}");
    }

    #[test]
    fn el_mastil_atraviesa_la_superficie_del_agua() {
        // Es la decisión de composición que hace legible al barco: el casco
        // queda sumergido y solo el mástil rompe la superficie. Sin eso, con
        // la bahía en penumbra, el barco sería una mancha.
        let (_, tamano) = caja_del_volumen(ANCLA);
        let superficie = ANCLA.y + ALTURA_SUPERFICIE;

        let (scene, _) = solo(mastil);
        let cajas: Vec<_> = scene.objects.iter().map(|o| o.primitive.bounds()).collect();

        let cima = cajas.iter().map(|c| c.max.y).fold(f32::MIN, f32::max);
        assert!(
            cima > superficie,
            "el mastil se queda bajo el agua: {cima} contra {superficie}"
        );

        // El mastil no se queda corto: sobresale mas de una unidad.
        assert!(
            cima > superficie + 1.0,
            "el mastil apenas asoma: {cima} contra {superficie}"
        );

        let (scene_casco, _) = solo(casco);

        // Del casco solo la popa rompe la superficie, y eso es parte de la
        // silueta: un pecio escorado con la popa levantada se lee mucho
        // mejor que un casco enteramente sumergido. Lo que no puede pasar
        // es que asome medio barco.
        let asoman = scene_casco
            .objects
            .iter()
            .filter(|o| o.primitive.bounds().max.y > superficie)
            .count();

        assert!(
            (1..=2).contains(&asoman),
            "{asoman} piezas del casco rompen la superficie de 12"
        );

        // Y la que asoma es la mas alta: la popa.
        let mas_alta = scene_casco
            .objects
            .iter()
            .map(|o| {
                let caja = o.primitive.bounds();

                caja.max.y - caja.min.y
            })
            .fold(0.0_f32, f32::max);
        let popa = scene_casco
            .objects
            .last()
            .expect("hay popa")
            .primitive
            .bounds();

        assert!(
            (popa.max.y - popa.min.y - mas_alta).abs() < 1e-6,
            "la pieza que asoma no es la popa"
        );

        // Y el casco tampoco atraviesa el fondo del volumen.
        let piso = superficie - tamano.y;
        let quilla = scene_casco
            .objects
            .iter()
            .map(|o| o.primitive.bounds().min.y)
            .fold(f32::MAX, f32::min);
        assert!(quilla > piso, "el casco se sale por el fondo del volumen");
    }

    #[test]
    fn la_cadena_y_el_ancla_usan_metal_reusado_y_no_basalto() {
        // El inventario limita el proyecto a cinco materiales finales, y el
        // metal no es uno de ellos: es basalto con otro tinte y otra escala.
        for entrada in [cadena as fn(&mut Scene, &Palette, Vec3), ancla_del_barco] {
            let (scene, paleta) = solo(entrada);
            let basalto = scene.material(paleta.wet_basalt);

            for objeto in &scene.objects {
                assert_ne!(
                    objeto.final_material, paleta.wet_basalt,
                    "usa el basalto tal cual en vez del metal reusado"
                );

                let metal = scene.material(objeto.final_material);

                // Lo que lo distingue.
                assert_ne!(metal.albedo, basalto.albedo, "mismo tinte que la roca");
                // Frio: el rojo se atenua mas que el azul, asi que la
                // proporcion azul/rojo sube respecto de la roca.
                assert!(
                    metal.albedo.b / metal.albedo.r > basalto.albedo.b / basalto.albedo.r,
                    "el metal no salio mas frio que la roca"
                );
                assert!(metal.uv_scale > basalto.uv_scale, "misma escala UV");
                // El brillo no es mas fuerte, es mas **estrecho**: la roca
                // mojada ya viene con specular 0.85, y competir en fuerza
                // no distinguiria nada. El lobulo si.
                assert!(
                    metal.shininess > basalto.shininess * 2.0,
                    "el lobulo del metal no es mas estrecho que el de la roca: {} contra {}",
                    metal.shininess,
                    basalto.shininess
                );
                assert!(
                    metal.specular_strength >= 0.7,
                    "el metal perdio su brillo: {}",
                    metal.specular_strength
                );

                // Y lo que **no** cambia: no lanza rayos.
                assert_eq!(
                    metal.reflection_cap, 0.0,
                    "la cadena no puede lanzar rayos reflejados"
                );
                assert_eq!(metal.transmission_cap, 0.0);
                assert_eq!(metal.shadow_mode, ShadowMode::Opaque);
                assert!(metal.is_valid());
            }
        }
    }

    #[test]
    fn la_cadena_cuelga_del_barco_al_ancla() {
        // La comba no es adorno: una cadena recta entre dos puntos se lee
        // como una varilla. Y los extremos tienen que quedar donde estan el
        // barco y el ancla, o la cadena no une nada.
        let (scene, _) = solo(cadena);
        let centros: Vec<Vec3> = scene
            .objects
            .iter()
            .map(|o| {
                let caja = o.primitive.bounds();

                (caja.min + caja.max) * 0.5
            })
            .collect();

        // Baja de forma monótona: el primer segmento es el más alto.
        for par in centros.windows(2) {
            assert!(par[1].y < par[0].y, "la cadena no baja de forma monotona");
        }

        // Y cuelga: cada segmento queda por debajo de la recta que une el
        // primero con el último.
        let (a, b) = (centros[0], centros[centros.len() - 1]);
        let mut alguno_debajo = false;

        for punto in &centros[1..centros.len() - 1] {
            let t = (punto.x - a.x) / (b.x - a.x);
            let recta = a.y + (b.y - a.y) * t;

            assert!(punto.y <= recta + 1e-5, "un segmento sube sobre la recta");
            if punto.y < recta - 0.05 {
                alguno_debajo = true;
            }
        }

        assert!(alguno_debajo, "la cadena va recta, sin comba");

        // El ancla empieza donde la cadena termina.
        let (escena_ancla, _) = solo(ancla_del_barco);
        let cima_del_ancla = escena_ancla
            .objects
            .iter()
            .map(|o| o.primitive.bounds().max.y)
            .fold(f32::MIN, f32::max);
        let final_de_cadena = b.y;

        assert!(
            (final_de_cadena - cima_del_ancla).abs() < 0.6,
            "la cadena termina a {final_de_cadena} y el ancla empieza a {cima_del_ancla}"
        );
    }

    #[test]
    fn el_barco_completo_cabe_dentro_del_volumen_en_planta() {
        // En planta, no en altura: el mastil sale por arriba a proposito,
        // pero nada del barco debe asomar por los lados de la bahia.
        let (centro, tamano) = caja_del_volumen(ANCLA);

        for entrada in [
            casco as fn(&mut Scene, &Palette, Vec3),
            mastil,
            cadena,
            ancla_del_barco,
        ] {
            let (scene, _) = solo(entrada);

            for objeto in &scene.objects {
                let caja = objeto.primitive.bounds();

                assert!(
                    caja.min.x > centro.x - tamano.x * 0.5
                        && caja.max.x < centro.x + tamano.x * 0.5,
                    "algo del barco se sale en X: {caja:?}"
                );
                assert!(
                    caja.min.z > centro.z - tamano.z * 0.5
                        && caja.max.z < centro.z + tamano.z * 0.5,
                    "algo del barco se sale en Z: {caja:?}"
                );
            }
        }
    }

    #[test]
    fn el_kelp_usa_verde_submarino_y_no_cesped_de_pradera() {
        let (scene, paleta) = solo(kelp);
        let pradera = scene.material(paleta.meadow);

        assert_eq!(scene.objects.len(), 12, "A-07 son doce frondas");

        for objeto in &scene.objects {
            assert_ne!(
                objeto.final_material, paleta.meadow,
                "el kelp usa el cesped de la pradera tal cual"
            );

            let kelp = scene.material(objeto.final_material);

            // El tinte corta el rojo y conserva verde y azul: es lo que
            // hace el agua con la luz.
            assert!(
                kelp.albedo.r < pradera.albedo.r,
                "el tinte no atenua el rojo"
            );
            assert!(
                kelp.albedo.b > kelp.albedo.r,
                "bajo el agua el azul tiene que superar al rojo: {:?}",
                kelp.albedo
            );
            assert!(
                kelp.albedo.g > kelp.albedo.r,
                "el kelp dejo de ser verde: {:?}",
                kelp.albedo
            );

            // Y no proyecta sombra: doce palos delgados motearian el lecho.
            assert_eq!(kelp.shadow_mode, ShadowMode::Ignore);
            assert!(!kelp.blocks_shadows());
            assert!(kelp.is_valid());
        }
    }

    #[test]
    fn el_kelp_no_es_el_unico_que_ignora_las_sombras_dentro_de_la_bahia() {
        // El volumen de agua tambien las ignora, por decision del
        // inventario. Lo que sigue proyectando sombra dentro de la bahia
        // son las rocas y el barco, que es lo que da profundidad al lecho.
        let (scene, _) = construir(WaterPreset::RefractiveWater);

        let opacos = scene
            .objects
            .iter()
            .filter(|o| scene.material(o.final_material).blocks_shadows())
            .count();

        // 58 menos el volumen y menos las doce frondas.
        assert_eq!(opacos, 58 - 1 - 12);
    }

    #[test]
    fn tenir_da_el_mismo_resultado_con_textura_y_sin_ella() {
        // El fallo que este test existe para atrapar: `with_tint` reemplaza
        // el albedo. Con un tinte absoluto, el kelp sin texturas salia mas
        // claro que la pradera en vez de mas oscuro, porque el tinte pasaba
        // a ser el color entero. Y sin texturas es como corren los tests y
        // como corre `--no-textures`.
        use crate::texture::Texture;

        let factor = Color::new(0.30, 0.85, 1.00);

        // Material de color plano.
        let plano = Material::new(Color::new(0.4, 0.6, 0.2));
        let plano_tenido = tenir(plano, factor);

        // El mismo material, texturizado: su albedo pasa a blanco y el
        // color lo aporta la muestra.
        let mut scene = Scene::new();
        let textura = Texture::from_pixels(1, 1, vec![Color::new(0.4, 0.6, 0.2)]).expect("1x1");
        let id = scene.add_texture(textura);
        let texturizado = Material::new(Color::new(0.4, 0.6, 0.2)).with_texture(id);
        let texturizado_tenido = tenir(texturizado, factor);

        // El color efectivo de los dos, resuelto por la escena.
        let uv = nalgebra_glm::Vec2::new(0.5, 0.5);
        let a = scene.albedo_at(&plano_tenido, &uv);
        let b = scene.albedo_at(&texturizado_tenido, &uv);

        for (uno, otro, canal) in [(a.r, b.r, "r"), (a.g, b.g, "g"), (a.b, b.b, "b")] {
            assert!(
                (uno - otro).abs() < 1e-6,
                "el canal {canal} difiere con textura y sin ella: {uno} contra {otro}"
            );
        }

        // Y en los dos casos el factor **atenuo**: nunca aclara.
        assert!(a.r < 0.4 && a.g < 0.6);
        assert!(b.r < 0.4 && b.g < 0.6);
    }
}
