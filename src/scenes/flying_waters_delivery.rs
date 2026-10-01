//! Aguas Voladoras en la entrega: el pecio partido aprobado.
//!
//! Es la variante `split_wreck_thin_chain` del preview de fidelidad
//! (`examples/flying_waters_fidelity_preview.rs`), aprobada por Charlie como
//! default. Este módulo es su versión de producción: el mismo resultado, sin
//! el arnés ni las variantes descartadas. Un test del preview comprueba que la
//! entrega que sale de aquí es **byte a byte** la escena aprobada.
//!
//! # El pecio
//!
//! El barco no está entero. Abajo, el **cuerpo destruido** empotrado en el
//! lecho; arriba, la **proa de costillas** suspendida en el agua, detrás y
//! más alta para que desde la hero se lea encima; entre los dos, una
//! **cadena fina y negra**. El fragmento alto flota a propósito: no es un
//! hueco de apoyo.
//!
//! # Reasignación (Aguas 58, total 168)
//!
//! | Huecos | Entrada | Rol |
//! |---|---|---|
//! | 0 | `A-02` | suelo del lecho: misma caja, material de profundidad |
//! | 1–4 | `A-02` | cuatro mesetas someras |
//! | 5 | `A-03` | quilla del fragmento alto |
//! | 6–11 | `A-03` | seis costillas del fragmento alto |
//! | 12–13 | `A-03` | dos largueros del fragmento alto |
//! | 14–16 | `A-03` | tres piezas del fragmento bajo |
//! | 17 | `A-04` | roda del fragmento alto |
//! | 18–19 | `A-04` | dos piezas del fragmento bajo |
//! | 20–25 | `A-05` | seis tramos de cadena fina |
//! | 26–27 | `A-05` | dos piezas del fragmento bajo |
//! | 28–30 | `A-06` | el ancla |
//! | 31–42 | `A-07` | kelp vertical, de alturas y grosores variados |
//! | 43–45 | `A-08` | rastro de restos bajo la proa |
//! | 46–48 | `A-08` | tres corales magenta (`A-12` sin presupuesto propio) |
//! | 49–56 | `A-11` | el borde roto, **intacto** |
//! | 57 | `A-01` | el volumen de agua, **intacto** |
//!
//! # La cadena
//!
//! Seis segmentos: cada uno es la caja de un tramo del trazado, con `0.07` de
//! grueso en `x`, el eje que la hero y el encuadre cercano ven de frente.
//! Desde un lateral se lee como una cinta escalonada; es lo aprobado. Una
//! cadena de bloques pequeños pediría más piezas y no está autorizada.
//!
//! # Materiales
//!
//! Cuatro nuevos, añadidos al final de la paleta con su textura procedural
//! en memoria: el suelo con degradado de profundidad, las mesetas someras,
//! el coral con el magenta de Praderas y el metal negro de la cadena. El
//! resto reusa sin tocarlos los materiales que Aguas ya tenía. El agua no
//! cambia.

use super::meadows_delivery::{fbm, magenta, mezclar, ruido, textura};
use crate::bounds::Aabb;
use crate::color::Color;
use crate::cuboid::Cuboid;
use crate::material::Material;
use crate::scene::{MaterialId, Scene, SpatialGroupId};
use crate::texture::Texture;
use nalgebra_glm::Vec3;

// ---------------------------------------------------------------------
// Texturas procedurales
// ---------------------------------------------------------------------

/// `F-04` · el suelo del lecho: hondo y azul en el centro de la bahía,
/// somero y claro hacia los bordes. La cara superior de un cuboide recorre
/// `u` en `x` y `v` en `z`, de borde a borde, así que con escala UV `1` el
/// degradado cae exactamente sobre la planta de la bahía.
fn textura_lecho() -> Texture {
    let hondo = Color::from_srgb(0.09, 0.16, 0.22);
    let medio = Color::from_srgb(0.20, 0.29, 0.32);
    let somero = Color::from_srgb(0.42, 0.48, 0.44);

    textura(144, 96, |u, v| {
        let (dx, dz) = ((u - 0.5) * 2.0, (v - 0.5) * 2.0);
        let r = (dx * dx + dz * dz).sqrt() / std::f32::consts::SQRT_2;
        let grano = fbm(u, v, 6, 3, 0x1EC0_0001);
        let t = (r - 0.25) / 0.55 + (grano - 0.5) * 0.35;
        if t < 0.5 {
            mezclar(hondo, medio, t * 2.0)
        } else {
            mezclar(medio, somero, (t - 0.5) * 2.0)
        }
    })
}

/// Las mesetas someras: arena gris verdosa con piedra.
fn textura_somera() -> Texture {
    let arena = Color::from_srgb(0.44, 0.49, 0.44);
    let piedra = Color::from_srgb(0.27, 0.32, 0.31);

    textura(64, 64, |u, v| {
        mezclar(arena, piedra, (fbm(u, v, 5, 3, 0x50E7_0001) - 0.35) * 2.2)
    })
}

/// `F-12` · coral en racimos magenta —el magenta de Praderas— sobre una base
/// oscura, con huecos.
fn textura_coral() -> Texture {
    let rosa = Color::from_srgb(0.80, 0.46, 0.66);
    let base = Color::from_srgb(0.20, 0.18, 0.22);

    textura(48, 48, |u, v| {
        let racimo = fbm(u, v, 3, 3, 0xC0A2_0001);
        let grano = ruido(u, v, 9, 9, 0xC0A2_0002);
        if racimo > 0.50 && grano > 0.30 {
            magenta()
        } else if racimo > 0.44 {
            rosa
        } else {
            base
        }
    })
}

/// El metal negro de la cadena: carbón con motas algo más claras. Ni negro
/// absoluto, que bajo el agua sería un agujero, ni claro.
fn textura_metal_negro() -> Texture {
    let carbon = Color::from_srgb(0.09, 0.09, 0.10);
    let mota = Color::from_srgb(0.17, 0.17, 0.19);
    let hondo = Color::from_srgb(0.05, 0.05, 0.06);

    textura(16, 16, |u, v| {
        let n = ruido(u, v, 4, 4, 0x0E60_0001);
        if n > 0.72 {
            mota
        } else if n < 0.25 {
            hondo
        } else {
            carbon
        }
    })
}

// ---------------------------------------------------------------------
// Composición
// ---------------------------------------------------------------------

/// Una caja relativa a la bahía `(ancla.x, 0, ancla.z)`:
/// `[x0, y0, z0, x1, y1, z1]`.
type Caja = [f32; 6];

/// Cuánto se hunde una pieza posada en su apoyo.
const EMPOTRADO: f32 = 0.04;

/// Techo del suelo del lecho, que no se mueve.
const SUELO: f32 = 0.65;

/// Una pieza posada en `apoyo`, centrada en `(x, z)`.
fn posada(x: f32, z: f32, ancho: f32, alto: f32, fondo: f32, apoyo: f32) -> Caja {
    let y0 = apoyo - EMPOTRADO;
    [
        x - ancho * 0.5,
        y0,
        z - fondo * 0.5,
        x + ancho * 0.5,
        y0 + alto,
        z + fondo * 0.5,
    ]
}

/// `A-02` · cuatro mesetas someras pegadas a las paredes, que muerden el
/// suelo `0.10` y dejan el centro hondo.
const MESETAS: [Caja; 4] = [
    [-4.15, 0.55, -2.40, -1.60, 0.84, -1.00],
    [3.40, 0.55, -2.40, 4.20, 0.80, 0.20],
    [-1.40, 0.55, -2.42, 1.00, 0.74, -1.85],
    [-4.15, 0.55, 0.60, -2.30, 0.78, 1.70],
];

// El espacio vertical lo fija `A-01`, que no se mueve: del techo del lecho
// (`0.65`) a la superficie (`2.60`) hay `1.95`. El fragmento bajo ocupa
// `0.65 .. 1.38`; el alto, `1.58 .. 2.50`, detrás y arriba.

/// Fragmento alto: la quilla de la proa.
const QUILLA_ALTA: Caja = [-0.70, 1.58, -1.51, 1.55, 1.70, -1.39];

/// Eje del fragmento alto en `z`.
const EJE_ALTO: f32 = -1.45;

/// Costillas del fragmento alto, de la rotura a la proa:
/// `(x, grosor, manga, techo, desplazamiento en z)`.
const COSTILLAS_ALTAS: [(f32, f32, f32, f32, f32); 6] = [
    (-0.50, 0.10, 1.20, 2.48, 0.0),
    (-0.12, 0.12, 1.12, 2.50, 0.0),
    (0.26, 0.10, 1.00, 2.32, 0.0),
    (0.66, 0.12, 0.84, 2.46, 0.0),
    (1.04, 0.10, 0.44, 2.36, -0.10),
    (1.38, 0.10, 0.40, 2.24, 0.0),
];

/// Pie común de las costillas altas: muerde la quilla.
const PIE_DE_COSTILLA: f32 = 1.62;

/// Largueros de la borda, rotos a distinta longitud.
const LARGUEROS_ALTOS: [Caja; 2] = [
    [-0.60, 2.22, -1.07, 0.72, 2.32, -0.99],
    [-0.60, 2.24, -1.91, 0.30, 2.32, -1.83],
];

/// La roda: la única pieza del pecio que rompe la superficie.
const RODA: Caja = [1.48, 1.58, -1.51, 1.60, 2.74, -1.39];

/// Fragmento bajo: el cuerpo destruido, empotrado en el lecho delante del
/// alto. Fondo enterrado, banda alta, banda rota baja, espejo de popa,
/// cuaderna en pie, cuaderna partida y tablón caído.
const CASCO_BAJO: [Caja; 7] = [
    [-1.05, 0.55, -0.20, 1.10, 0.80, 0.85],
    [-1.00, 0.60, -0.35, 0.85, 1.25, -0.15],
    [-0.90, 0.60, 0.75, 0.30, 0.98, 0.95],
    [-1.15, 0.60, -0.30, -0.95, 1.32, 0.90],
    [-0.15, 0.75, -0.25, -0.05, 1.38, 0.85],
    [0.60, 0.75, -0.25, 0.70, 1.15, 0.40],
    [-0.80, 1.20, -0.35, 0.20, 1.30, 0.10],
];

/// Grosor de la cadena.
const GROSOR_DE_CADENA: f32 = 0.07;

/// Siete puntos del gesto, de dentro de la quilla del fragmento alto al
/// fondo del casco bajo: cae deprisa y se tiende hacia delante. Cada tramo
/// es la caja entre dos puntos consecutivos; comparten sus extremos, así que
/// la cadena es continua.
const TRAZADO_DE_CADENA: [[f32; 3]; 7] = [
    [1.05, 1.62, -1.42],
    [1.04, 1.40, -1.20],
    [1.03, 1.20, -0.96],
    [1.02, 1.06, -0.72],
    [1.01, 0.97, -0.48],
    [1.005, 0.91, -0.26],
    [1.00, 0.83, -0.10],
];

/// El ancla, posada junto al canto roto del fragmento bajo.
const ANCLA: [Caja; 3] = [
    [1.35, 0.60, 0.20, 1.57, 1.30, 0.42],
    [1.10, 0.54, 0.20, 1.82, 0.76, 0.42],
    [1.29, 1.15, 0.21, 1.63, 1.35, 0.41],
];

/// `A-07` · `(x, z, grosor, alto, apoyo)`.
const KELP: [(f32, f32, f32, f32, f32); 12] = [
    (-3.85, -2.15, 0.22, 1.45, 0.84),
    (-3.45, -1.75, 0.12, 1.05, 0.84),
    (-3.95, -1.35, 0.16, 1.55, 0.84),
    (-3.10, -2.25, 0.10, 0.75, 0.84),
    (-2.65, -1.25, 0.18, 1.20, 0.84),
    (-3.70, -0.55, 0.14, 0.95, SUELO),
    (-4.00, 0.10, 0.24, 1.70, SUELO),
    (-3.75, 1.10, 0.12, 0.80, 0.78),
    (-2.85, 1.40, 0.20, 1.10, 0.78),
    (3.85, -2.05, 0.20, 1.50, 0.80),
    (4.05, -1.45, 0.12, 0.95, 0.80),
    (3.65, -0.35, 0.16, 1.25, 0.80),
];

/// `A-08` · restos bajo la proa: `(x, z, ancho, alto, fondo)`.
const RASTRO: [(f32, f32, f32, f32, f32); 3] = [
    (0.20, -1.15, 0.30, 0.18, 0.26),
    (0.85, -1.70, 0.24, 0.15, 0.22),
    (1.45, -1.20, 0.34, 0.20, 0.28),
];

/// `A-08` → `A-12` · corales bajos: `(x, z, ancho, alto, fondo, apoyo)`.
const CORALES: [(f32, f32, f32, f32, f32, f32); 3] = [
    (-3.00, -1.70, 0.70, 0.18, 0.50, 0.84),
    (3.80, -1.10, 0.50, 0.20, 0.62, 0.80),
    (0.30, -2.15, 0.56, 0.16, 0.42, 0.74),
];

fn costilla_alta(&(x, grosor, manga, techo, dz): &(f32, f32, f32, f32, f32)) -> Caja {
    [
        x - grosor * 0.5,
        PIE_DE_COSTILLA,
        EJE_ALTO + dz - manga * 0.5,
        x + grosor * 0.5,
        techo,
        EJE_ALTO + dz + manga * 0.5,
    ]
}

/// Un tramo de la cadena, en coordenadas de mundo: la caja que va de un
/// punto del trazado al siguiente, con el grosor alrededor.
///
/// Se suma primero la base y después el grosor, en ese orden: es el
/// redondeo del preview aprobado, y con otro orden un extremo cae a un ULP.
fn tramo_de_cadena(base: Vec3, a: [f32; 3], b: [f32; 3]) -> Aabb {
    let g = GROSOR_DE_CADENA * 0.5;
    let minimo = Vec3::new(a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2]));
    let maximo = Vec3::new(a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2]));
    Aabb::new(
        base + minimo - Vec3::new(g, g, g),
        base + maximo + Vec3::new(g, g, g),
    )
}

#[derive(Debug, Clone, Copy)]
struct Materiales {
    lecho: MaterialId,
    somera: MaterialId,
    coral: MaterialId,
    madera: MaterialId,
    ancla: MaterialId,
    kelp: MaterialId,
    roca: MaterialId,
}

/// Las 58 piezas, hueco a hueco. `None` deja la pieza como está; una caja
/// `NaN` cambia solo el material. Los seis huecos de la cadena van en `None`
/// aquí y los tiende `aplicar`, en coordenadas de mundo.
fn piezas(m: &Materiales) -> Vec<Option<(Caja, MaterialId)>> {
    let mut p: Vec<Option<(Caja, MaterialId)>> = Vec::with_capacity(PIEZAS);

    p.push(Some(([f32::NAN; 6], m.lecho)));
    p.extend(MESETAS.iter().map(|&c| Some((c, m.somera))));
    // A-03 (12): alto 9 + bajo 3.
    p.push(Some((QUILLA_ALTA, m.madera)));
    p.extend(
        COSTILLAS_ALTAS
            .iter()
            .map(|c| Some((costilla_alta(c), m.madera))),
    );
    p.extend(LARGUEROS_ALTOS.iter().map(|&c| Some((c, m.madera))));
    p.extend(CASCO_BAJO[..3].iter().map(|&c| Some((c, m.madera))));
    // A-04 (3): roda + bajo 2.
    p.push(Some((RODA, m.madera)));
    p.extend(CASCO_BAJO[3..5].iter().map(|&c| Some((c, m.madera))));
    // A-05 (8): cadena 6 + bajo 2.
    p.extend(std::iter::repeat_n(None, TRAZADO_DE_CADENA.len() - 1));
    p.extend(CASCO_BAJO[5..].iter().map(|&c| Some((c, m.madera))));
    // A-06.
    p.extend(ANCLA.iter().map(|&c| Some((c, m.ancla))));
    // A-07.
    p.extend(
        KELP.iter()
            .map(|&(x, z, w, h, apoyo)| Some((posada(x, z, w, h, w, apoyo), m.kelp))),
    );
    // A-08.
    p.extend(
        RASTRO
            .iter()
            .map(|&(x, z, w, h, f)| Some((posada(x, z, w, h, f, SUELO), m.roca))),
    );
    p.extend(
        CORALES
            .iter()
            .map(|&(x, z, w, h, f, apoyo)| Some((posada(x, z, w, h, f, apoyo), m.coral))),
    );
    // A-11 y A-01: intactos.
    p.extend(std::iter::repeat_n(None, 9));

    p
}

fn con_textura(scene: &mut Scene, base: Material, tex: Texture) -> MaterialId {
    let id = scene.add_texture(tex);
    scene.add_material(base.with_texture(id).with_uv_scale(1.0))
}

// ---------------------------------------------------------------------
// API
// ---------------------------------------------------------------------

/// Piezas de Aguas del inventario, con el volumen dentro.
const PIEZAS: usize = 58;

/// Huecos de los que salen los materiales que se reusan: la madera del
/// pecio (`A-03`), el metal del ancla (`A-06`), el kelp (`A-07`) y el
/// basalto de las rocas (`A-08`).
const HUECO_MADERA: usize = 5;
const HUECO_ANCLA: usize = 28;
const HUECO_KELP: usize = 31;
const HUECO_ROCA: usize = 43;
/// Primer hueco de la cadena dentro de Aguas.
const PRIMER_HUECO_DE_CADENA: usize = 20;

/// Reescribe en su sitio el interior de Aguas Voladoras de una entrega.
///
/// `ancla` es el ancla de la bahía **ya repartida** por la composición. Hay
/// que llamarla con el volumen `A-01` todavía dentro —antes de retirarlo
/// para `InteriorVisible`— y antes de medir la escala y construir la
/// jerarquía, que guarda posiciones.
pub fn aplicar(scene: &mut Scene, ancla: Vec3) {
    let aguas: Vec<usize> = scene
        .objects
        .iter()
        .enumerate()
        .filter(|(_, o)| o.spatial_group == SpatialGroupId::FlyingWaters)
        .map(|(i, _)| i)
        .collect();

    assert_eq!(
        aguas.len(),
        PIEZAS,
        "Aguas tiene que llegar con sus 58 piezas, el volumen incluido"
    );
    let volumen = scene.material(scene.objects[aguas[PIEZAS - 1]].final_material);
    assert!(
        volumen.ior > 1.0,
        "la última pieza de Aguas tiene que ser el volumen A-01"
    );

    let usado = |k: usize| scene.objects[aguas[k]].final_material;
    let (madera, ancla_metal, kelp, roca) = (
        usado(HUECO_MADERA),
        usado(HUECO_ANCLA),
        usado(HUECO_KELP),
        usado(HUECO_ROCA),
    );

    // El orden de registro forma parte del resultado: fija los índices de
    // material y textura, y es el del preview aprobado.
    let blanco = Material::new(Color::new(1.0, 1.0, 1.0));
    let lecho = con_textura(scene, blanco.with_specular(0.20, 32.0), textura_lecho());
    let somera = con_textura(scene, blanco.with_specular(0.15, 24.0), textura_somera());
    let coral = con_textura(scene, blanco.with_specular(0.10, 16.0), textura_coral());
    let negro = {
        let tex = scene.add_texture(textura_metal_negro());
        scene.add_material(
            Material::new(Color::new(1.0, 1.0, 1.0))
                .with_texture(tex)
                .with_uv_scale(1.0)
                .with_specular(0.40, 40.0),
        )
    };
    let m = Materiales {
        lecho,
        somera,
        coral,
        madera,
        ancla: ancla_metal,
        kelp,
        roca,
    };

    let base = Vec3::new(ancla.x, 0.0, ancla.z);

    for (k, pieza) in piezas(&m).into_iter().enumerate() {
        let Some((c, material)) = pieza else { continue };
        let objeto = &mut scene.objects[aguas[k]];
        objeto.final_material = material;
        if c[0].is_nan() {
            continue;
        }
        objeto.primitive = Cuboid::new(Aabb::new(
            base + Vec3::new(c[0], c[1], c[2]),
            base + Vec3::new(c[3], c[4], c[5]),
        ))
        .into();
    }

    // A-05: la cadena fina, seis tramos entre los dos fragmentos.
    for (k, par) in TRAZADO_DE_CADENA.windows(2).enumerate() {
        let objeto = &mut scene.objects[aguas[PRIMER_HUECO_DE_CADENA + k]];
        objeto.primitive = Cuboid::new(tramo_de_cadena(base, par[0], par[1])).into();
        objeto.final_material = negro;
    }
}

#[cfg(test)]
mod tests {
    use crate::bounds::Aabb;
    use crate::material::ShadowMode;
    use crate::scene::{RevealGroup, Scene, SpatialGroupId};
    use crate::scene_builder::{measure_scene_radius, Blockout};
    use crate::scenes::{
        delivery_level, delivery_level_previo_aguas, safe_level, WaterPreset, DELIVERY,
    };

    const PRESETS: [WaterPreset; 3] = [
        WaterPreset::RefractiveWater,
        WaterPreset::InteriorVisible,
        WaterPreset::OpaqueWater,
    ];

    /// Huecos de Aguas, en el orden de `flying_waters::aguas_voladoras`.
    const SUELO: usize = 0;
    const FRAGMENTO_ALTO: [usize; 10] = [5, 6, 7, 8, 9, 10, 11, 12, 13, 17];
    const FRAGMENTO_BAJO: [usize; 7] = [14, 15, 16, 18, 19, 26, 27];
    const CADENA: std::ops::Range<usize> = 20..26;
    const BORDE: std::ops::Range<usize> = 49..57;

    fn aguas(scene: &Scene) -> Vec<usize> {
        scene
            .objects
            .iter()
            .enumerate()
            .filter(|(_, o)| o.spatial_group == SpatialGroupId::FlyingWaters)
            .map(|(i, _)| i)
            .collect()
    }

    fn caja(scene: &Scene, i: usize) -> Aabb {
        scene.objects[i].primitive.bounds()
    }

    fn tocan(a: &Aabb, b: &Aabb) -> bool {
        const EPS: f32 = 1e-4;
        a.min.x <= b.max.x + EPS
            && b.min.x <= a.max.x + EPS
            && a.min.y <= b.max.y + EPS
            && b.min.y <= a.max.y + EPS
            && a.min.z <= b.max.z + EPS
            && b.min.z <= a.max.z + EPS
    }

    fn entrega() -> Blockout {
        delivery_level(WaterPreset::RefractiveWater)
    }

    #[test]
    fn aguas_cuenta_58_y_la_entrega_168_en_los_tres_presets() {
        for water in PRESETS {
            let d = delivery_level(water);
            let sin_volumen = usize::from(water == WaterPreset::InteriorVisible);

            assert_eq!(aguas(&d.scene).len(), 58 - sin_volumen, "{water:?}");
            assert_eq!(aguas(&d.scene).len(), DELIVERY.flying_waters - sin_volumen);
            assert_eq!(
                d.scene.objects.len(),
                DELIVERY.total() - sin_volumen,
                "{water:?}"
            );
        }
    }

    #[test]
    fn la_entrega_lleva_el_pecio_partido_con_la_cadena_fina() {
        let previo = delivery_level_previo_aguas(WaterPreset::RefractiveWater);
        let d = entrega();
        let a = aguas(&d.scene);
        let suelo = caja(&d.scene, a[SUELO]);

        // Fragmento alto: suspendido, con vacío debajo.
        let alto: Vec<Aabb> = FRAGMENTO_ALTO
            .iter()
            .map(|&k| caja(&d.scene, a[k]))
            .collect();
        let pie_alto = alto.iter().map(|b| b.min.y).fold(f32::MAX, f32::min);
        assert!(
            pie_alto - suelo.max.y >= 0.9,
            "el fragmento alto no flota: {pie_alto}"
        );

        // Fragmento bajo: empotrado en el lecho.
        let bajo: Vec<Aabb> = FRAGMENTO_BAJO
            .iter()
            .map(|&k| caja(&d.scene, a[k]))
            .collect();
        let hundido = bajo
            .iter()
            .map(|b| suelo.max.y - b.min.y)
            .fold(f32::MIN, f32::max);
        assert!(
            (0.05..=0.15).contains(&hundido),
            "el fragmento bajo no muerde: {hundido}"
        );

        // Cadena: seis tramos finos con material propio y negro.
        let cadena: Vec<usize> = CADENA.map(|k| a[k]).collect();
        let negro = d.scene.objects[cadena[0]].final_material;
        assert!(
            negro.0 >= previo.scene.palette.len(),
            "la cadena usa un material compartido"
        );
        for &i in &cadena {
            let b = caja(&d.scene, i);
            assert!(b.max.x - b.min.x <= 0.10, "tramo grueso: {b:?}");
            assert_eq!(d.scene.objects[i].final_material, negro);
        }
        let m = d.scene.material(negro);
        let pico = d.scene.texture(m.albedo_texture.expect("textura")).peak();
        let luz = 0.2126 * pico.r * m.albedo.r
            + 0.7152 * pico.g * m.albedo.g
            + 0.0722 * pico.b * m.albedo.b;
        assert!((0.004..=0.05).contains(&luz), "no es negro carbon: {luz}");
        assert_eq!((m.reflection_cap, m.transmission_cap), (0.0, 0.0));
    }

    /// Todo lo ajeno a Aguas, más `A-01` y `A-11`, es la entrega previa a la
    /// promoción byte a byte, en los tres presets.
    #[test]
    fn fuera_de_aguas_la_entrega_es_la_previa_byte_a_byte() {
        for water in PRESETS {
            let x = delivery_level_previo_aguas(water);
            let y = delivery_level(water);
            let ax = aguas(&x.scene);

            assert_eq!(x.scene.objects.len(), y.scene.objects.len());
            for (i, (o, n)) in x.scene.objects.iter().zip(&y.scene.objects).enumerate() {
                assert_eq!(o.spatial_group, n.spatial_group, "{i}");
                assert_eq!(o.reveal_group, n.reveal_group, "{i}");
                assert_eq!(o.initial_material, n.initial_material, "{i}");
                let k = ax.iter().position(|&j| j == i);
                let intacto = match k {
                    None => true,
                    Some(k) => BORDE.contains(&k) || k == 57,
                };
                if intacto {
                    assert_eq!(format!("{o:?}"), format!("{n:?}"), "{water:?}: objeto {i}");
                }
            }
            assert_eq!(
                y.scene.palette.len(),
                x.scene.palette.len() + 4,
                "cuatro materiales"
            );
            assert_eq!(
                y.scene.textures.len(),
                x.scene.textures.len() + 4,
                "cuatro texturas"
            );
            for (i, (m, n)) in x.scene.palette.iter().zip(&y.scene.palette).enumerate() {
                assert_eq!(format!("{m:?}"), format!("{n:?}"), "material {i}");
            }
            assert_eq!(
                format!("{:?}", x.scene.skybox),
                format!("{:?}", y.scene.skybox)
            );
            assert_eq!(format!("{:?}", x.anchors), format!("{:?}", y.anchors));
            assert_eq!(format!("{:?}", x.scale), format!("{:?}", y.scale));
            assert_eq!(x.hero_preset(), y.hero_preset());
            assert_eq!(
                format!("{:?}", x.accel.bounds),
                format!("{:?}", y.accel.bounds)
            );
            for (g, h) in x.accel.groups.iter().zip(&y.accel.groups) {
                assert_eq!(g.id, h.id);
                if g.id == SpatialGroupId::FlyingWaters {
                    assert_eq!(g.clusters.len(), h.clusters.len());
                } else {
                    assert_eq!(format!("{g:?}"), format!("{h:?}"), "{water:?}: {:?}", g.id);
                }
            }
            assert_eq!(
                measure_scene_radius(&y.scene, y.anchors.orbit_center),
                x.scale.scene_radius
            );
        }
    }

    #[test]
    fn el_agua_conserva_su_optica_en_los_tres_presets() {
        for water in PRESETS {
            let d = delivery_level(water);
            let a = aguas(&d.scene);
            let opticas: Vec<usize> = a
                .iter()
                .copied()
                .filter(|&i| {
                    let m = d.scene.material(d.scene.objects[i].final_material);
                    m.reflection_cap > 0.0 || m.transmission_cap > 0.0
                })
                .collect();
            match water {
                WaterPreset::RefractiveWater => {
                    assert_eq!(opticas, vec![*a.last().unwrap()], "solo A-01 es agua");
                    let m = d.scene.material(d.scene.objects[opticas[0]].final_material);
                    assert_eq!(
                        (m.reflection_cap, m.transmission_cap, m.ior),
                        (0.9, 0.9, 1.333)
                    );
                    assert_eq!(m.shadow_mode, ShadowMode::Ignore);
                }
                WaterPreset::OpaqueWater | WaterPreset::InteriorVisible => {
                    assert!(opticas.is_empty(), "{water:?}: {opticas:?}");
                }
            }
        }
    }

    /// Apoyo con la excepción intencional: el fragmento alto **flota**. Todo
    /// lo demás del interior llega al suelo por contactos sin pasar por él;
    /// el alto es conexo por dentro y no toca nada del fondo; la cadena une
    /// los dos fragmentos.
    #[test]
    fn soportes_con_la_suspension_intencional_del_fragmento_alto() {
        let d = entrega();
        let a = aguas(&d.scene);
        let cajas: Vec<Aabb> = a.iter().map(|&i| caja(&d.scene, i)).collect();
        let es_alto = |k: usize| FRAGMENTO_ALTO.contains(&k);
        let interior: Vec<usize> = (0..49).collect();

        let alcanzables = |dentro: &dyn Fn(usize) -> bool, inicio: usize| {
            let mut conectada = vec![false; cajas.len()];
            conectada[inicio] = true;
            loop {
                let mut cambio = false;
                for &k in &interior {
                    if !conectada[k]
                        && dentro(k)
                        && interior
                            .iter()
                            .any(|&s| conectada[s] && dentro(s) && tocan(&cajas[k], &cajas[s]))
                    {
                        conectada[k] = true;
                        cambio = true;
                    }
                }
                if !cambio {
                    break conectada;
                }
            }
        };

        let abajo = alcanzables(&|k| !es_alto(k), SUELO);
        let sueltas: Vec<usize> = interior
            .iter()
            .copied()
            .filter(|&k| !es_alto(k) && !abajo[k])
            .collect();
        assert!(sueltas.is_empty(), "sin apoyo hasta el lecho: {sueltas:?}");

        let arriba = alcanzables(&es_alto, FRAGMENTO_ALTO[0]);
        assert!(
            FRAGMENTO_ALTO.iter().all(|&k| arriba[k]),
            "el fragmento alto se deshace"
        );
        for &k in &FRAGMENTO_ALTO {
            for &s in &interior {
                if !es_alto(s) && !CADENA.contains(&s) {
                    assert!(
                        !tocan(&cajas[k], &cajas[s]),
                        "el alto {k} toca el fondo {s}"
                    );
                }
            }
        }

        let cadena: Vec<Aabb> = CADENA.map(|k| cajas[k]).collect();
        assert!(
            FRAGMENTO_ALTO.iter().any(|&k| tocan(&cajas[k], &cadena[0])),
            "la cadena no cuelga del alto"
        );
        assert!(
            FRAGMENTO_BAJO.iter().any(|&k| tocan(&cajas[k], &cadena[5])),
            "la cadena no llega al bajo"
        );
        for par in cadena.windows(2) {
            assert!(tocan(&par[0], &par[1]), "la cadena se corta");
            assert!(
                (par[1].min.y + par[1].max.y) < (par[0].min.y + par[0].max.y),
                "no desciende"
            );
        }
    }

    #[test]
    fn los_materiales_nuevos_son_locales_de_aguas_y_los_grupos_no_cambian() {
        let previo = delivery_level_previo_aguas(WaterPreset::RefractiveWater);
        let d = entrega();
        let antes = previo.scene.palette.len();
        let a = aguas(&d.scene);

        for (i, o) in d.scene.objects.iter().enumerate() {
            if o.final_material.0 >= antes {
                assert_eq!(
                    o.spatial_group,
                    SpatialGroupId::FlyingWaters,
                    "{i} usa un material de Aguas"
                );
            }
        }
        for m in &d.scene.palette[antes..] {
            assert!(m.is_valid());
            assert!(m.albedo_texture.expect("textura").0 >= previo.scene.textures.len());
        }
        let lienzo = d.scene.objects[0].initial_material;
        for &i in &a {
            let o = &d.scene.objects[i];
            assert_eq!(o.reveal_group, RevealGroup::FlyingWaters);
            assert_eq!(o.initial_material, lienzo, "{i} no nace en lienzo");
        }
        let negro = d.scene.objects[a[CADENA.start]].final_material;
        let con_negro = d
            .scene
            .objects
            .iter()
            .filter(|o| o.final_material == negro)
            .count();
        assert_eq!(con_negro, 6, "el negro es solo de la cadena");
    }

    #[test]
    fn la_entrega_de_aguas_es_determinista() {
        let (x, y) = (entrega(), entrega());
        assert_eq!(
            format!("{:?}", x.scene.objects),
            format!("{:?}", y.scene.objects)
        );
        assert_eq!(
            format!("{:?}", x.scene.palette),
            format!("{:?}", y.scene.palette)
        );
    }

    #[test]
    fn el_nivel_seguro_conserva_el_pecio_historico() {
        // La línea base de los hitos 3 a 7: 154 primitivas y la cadena de
        // ocho cubos de `0.22`.
        let seguro = safe_level(WaterPreset::RefractiveWater);
        let a = aguas(&seguro.scene);

        assert_eq!(seguro.scene.objects.len(), 154);
        for (k, &i) in a.iter().enumerate().take(28).skip(20) {
            let b = caja(&seguro.scene, i);
            assert!(
                (b.max.x - b.min.x - 0.22).abs() < 1e-5,
                "eslabon {k}: {b:?}"
            );
        }
    }

    /// FNV-1a de 64 bits sobre las piezas de Aguas, los materiales y las
    /// texturas nuevos (texel a texel), sin assets.
    fn huella(entrega: &Blockout, previo: &Scene) -> u64 {
        let scene = &entrega.scene;
        let mut h: u64 = 0xCBF2_9CE4_8422_2325;
        let mut mezclar = |texto: String| {
            for b in texto.bytes() {
                h ^= u64::from(b);
                h = h.wrapping_mul(0x0100_0000_01B3);
            }
        };

        for i in aguas(scene) {
            mezclar(format!("{:?}", scene.objects[i]));
        }
        mezclar(format!("{:?}", &scene.palette[previo.palette.len()..]));
        for t in &scene.textures[previo.textures.len()..] {
            for y in 0..t.height() {
                for x in 0..t.width() {
                    let u = (x as f32 + 0.5) / t.width() as f32;
                    let v = (y as f32 + 0.5) / t.height() as f32;
                    mezclar(format!("{:?}", t.sample(u, v)));
                }
            }
        }
        h
    }

    /// Huella de las Aguas de producción, tomada **después** de que el test
    /// `la_entrega_de_produccion_es_split_wreck_thin_chain_aprobado` del
    /// preview comprobara, con los assets reales, que esta entrega es byte a
    /// byte la variante aprobada. Aquí no hay assets —es el convenio de los
    /// tests de la biblioteca—, así que es un seguro de regresión, no la
    /// prueba de equivalencia.
    const HUELLA_DE_AGUAS: u64 = 11_444_163_205_226_346_912;

    #[test]
    fn la_huella_de_aguas_queda_fijada() {
        let previo = delivery_level_previo_aguas(WaterPreset::RefractiveWater);

        assert_eq!(
            huella(&entrega(), &previo.scene),
            HUELLA_DE_AGUAS,
            "Aguas de produccion cambio respecto de la aprobada"
        );
    }
}
