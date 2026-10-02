//! Praderas Primaverales en la entrega: la composición aprobada.
//!
//! Es la variante `refined_water_polish` del preview de fidelidad
//! (`examples/meadows_fidelity_preview.rs`), aprobada por Charlie como
//! default. Este módulo es su versión de producción: el mismo resultado, sin
//! el arnés de comparación ni las variantes descartadas. Un test del preview
//! comprueba que la entrega que sale de aquí es **byte a byte** la escena
//! que se aprobó.
//!
//! # Qué hace
//!
//! `aplicar` reescribe **en su sitio** primitiva y material final de las 37
//! piezas de Praderas de la entrega. Índice, grupo espacial, grupo de
//! revelado y material inicial —el lienzo— se conservan; nada ajeno a
//! Praderas se toca. No crea primitivas, tipos ni features del motor: todo
//! son cuboides, así que la Ruta A y la B dan la misma Praderas.
//!
//! | Entrada | Nivel seguro | Entrega |
//! |---|---:|---|
//! | `P-01` meseta | 6 | la base flotante intacta y cinco terrazas apoyadas en ella |
//! | `P-02` frente | 8 | repisas oscuras de ancho, caída y vuelo desiguales |
//! | `P-03` césped | 4 | cuatro láminas de tapiz procedural |
//! | `P-04` árboles | 6 | dos árboles bajos, a los flancos y fuera del corredor |
//! | `P-05` cascada | 1 | una caída de agua |
//! | `P-07` flores | 12 | cinco caídas, seis acentos bajos y una lámina de tapiz |
//! | **Total** | **37** | **37** |
//!
//! # Materiales
//!
//! Doce materiales y once texturas **nuevos**, añadidos al final de la
//! paleta y construidos en memoria con `Texture::from_pixels`. Ningún
//! material ni textura compartidos se modifican; los troncos reutilizan
//! `aged_wood` sin tocarlo.
//!
//! Las seis caídas son **agua**: copias por valor del agua vigente de
//! Praderas —techos `0.9 / 0.9`, `ior 1.333`, sombras `Ignore`— con textura
//! longitudinal propia, un tinte blanco frío y un brillo directo ancho
//! (`0.80 / 6` las anchas, `0.65 / 9` las finas). El brillo es la palanca:
//! con esos techos el color propio pesa un diez por ciento, y el brillo se
//! suma después del reparto de Fresnel. Ver `BRILLO`.
//!
//! # Lo que se midió en el preview
//!
//! Las decisiones de este módulo se tomaron allí, con renders y tests de
//! rayos reales: corredor despejado de `1.5°` alrededor del Monolito en la
//! hero, caídas legibles a `800 × 600`, apoyo por rol, tapiz legible y sin
//! estampado, y ningún píxel quemado. Este módulo no repite esas mediciones;
//! las hereda por la equivalencia exacta.

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

/// Hash entero a `0.0..1.0`, el mismo esquema que `generate_assets`.
pub(super) fn hash01(x: i32, y: i32, semilla: u32) -> f32 {
    let mut h =
        semilla ^ (x as u32).wrapping_mul(0x27D4_EB2D) ^ (y as u32).wrapping_mul(0x1656_67B1);

    h ^= h >> 15;
    h = h.wrapping_mul(0x2C1B_3C6D);
    h ^= h >> 12;
    h = h.wrapping_mul(0x297A_2D39);
    h ^= h >> 15;

    (h >> 8) as f32 / (1u32 << 24) as f32
}

fn suavizar(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// Ruido de valor **periódico** en `celdas_x × celdas_y`: la textura repite
/// sin costura, que es lo que pide `WrapMode::Repeat`.
pub(super) fn ruido(u: f32, v: f32, celdas_x: i32, celdas_y: i32, semilla: u32) -> f32 {
    let x = u * celdas_x as f32;
    let y = v * celdas_y as f32;
    let (x0, y0) = (x.floor() as i32, y.floor() as i32);
    let (sx, sy) = (suavizar(x - x0 as f32), suavizar(y - y0 as f32));

    let en = |dx: i32, dy: i32| {
        hash01(
            (x0 + dx).rem_euclid(celdas_x),
            (y0 + dy).rem_euclid(celdas_y),
            semilla,
        )
    };

    let arriba = en(0, 0) + (en(1, 0) - en(0, 0)) * sx;
    let abajo = en(0, 1) + (en(1, 1) - en(0, 1)) * sx;

    arriba + (abajo - arriba) * sy
}

pub(super) fn fbm(u: f32, v: f32, celdas: i32, octavas: u32, semilla: u32) -> f32 {
    let (mut suma, mut amplitud, mut total) = (0.0, 1.0, 0.0);

    for octava in 0..octavas {
        let c = celdas << octava;
        suma += amplitud * ruido(u, v, c, c, semilla.wrapping_add(octava));
        total += amplitud;
        amplitud *= 0.5;
    }

    suma / total
}

pub(super) fn mezclar(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);

    a * (1.0 - t) + b * t
}

/// Evalúa `patron(u, v)` en el centro de cada texel, con `v = 0` abajo como
/// `Texture::sample`.
pub(super) fn textura<F: Fn(f32, f32) -> Color>(ancho: usize, alto: usize, patron: F) -> Texture {
    let mut pixeles = Vec::with_capacity(ancho * alto);

    for fila in 0..alto {
        for columna in 0..ancho {
            let u = (columna as f32 + 0.5) / ancho as f32;
            let v = 1.0 - (fila as f32 + 0.5) / alto as f32;
            pixeles.push(patron(u, v));
        }
    }

    Texture::from_pixels(ancho, alto, pixeles).expect("dimensiones coherentes")
}

/// El único magenta de Praderas (`F-18`; `D-I` sigue abierta): el mismo
/// valor en el tapiz y en los acentos.
pub fn magenta() -> Color {
    Color::from_srgb(0.84, 0.24, 0.56)
}

fn rosa() -> Color {
    Color::from_srgb(0.80, 0.46, 0.66)
}

/// Discos de flor en una rejilla periódica de `celdas × celdas`. Devuelve
/// `Some(true)` en el núcleo de un disco, `Some(false)` en su halo y `None`
/// fuera. Cada disco aparece solo donde `densidad` lo deja.
fn disco(u: f32, v: f32, celdas: i32, densidad: f32, semilla: u32) -> Option<bool> {
    let (x, y) = (u * celdas as f32, v * celdas as f32);
    let (cx, cy) = (x.floor() as i32, y.floor() as i32);
    let mut mejor: Option<bool> = None;

    for dy in -1..=1 {
        for dx in -1..=1 {
            let (i, j) = (cx + dx, cy + dy);
            let (ei, ej) = (i.rem_euclid(celdas), j.rem_euclid(celdas));

            if hash01(ei, ej, semilla) > densidad {
                continue;
            }

            let px = i as f32 + hash01(ei, ej, semilla ^ 0x51) * 0.8 + 0.1;
            let py = j as f32 + hash01(ei, ej, semilla ^ 0xA2) * 0.8 + 0.1;
            let radio = 0.30 + 0.28 * hash01(ei, ej, semilla ^ 0xF3);
            let d = ((x - px).powi(2) + (y - py).powi(2)).sqrt();

            if d < radio * 0.72 {
                return Some(true);
            }
            if d < radio {
                mejor = Some(false);
            }
        }
    }

    mejor
}

/// Terrazas: roca cálida con estratos horizontales y musgo.
fn textura_roca() -> Texture {
    let clara = Color::from_srgb(0.38, 0.35, 0.31);
    let oscura = Color::from_srgb(0.21, 0.20, 0.19);
    let musgo = Color::from_srgb(0.27, 0.36, 0.17);

    textura(64, 64, |u, v| {
        let estrato = ruido(u, v, 2, 9, 0x5E0C_0001);
        let grano = fbm(u, v, 6, 2, 0x5E0C_0002);
        let color = mezclar(oscura, clara, estrato * 0.7 + grano * 0.5 - 0.15);

        let verde = fbm(u, v, 3, 2, 0x5E0C_0003);
        mezclar(color, musgo, (verde - 0.50) * 3.0)
    })
}

/// La base, el frente y la pared de las caídas interiores: roca casi negra
/// con regueros húmedos. Oscura a propósito: el agua transmite casi todo, y
/// lo que la separa de la roca es su color propio y su brillo contra un
/// fondo oscuro.
fn textura_frente() -> Texture {
    let fondo = Color::from_srgb(0.09, 0.10, 0.12);
    let reguero = Color::from_srgb(0.18, 0.20, 0.23);

    textura(64, 64, |u, v| {
        let vertical = ruido(u, v, 14, 2, 0xF0E8_0001);
        let grano = fbm(u, v, 5, 2, 0xF0E8_0002);

        mezclar(
            fondo,
            reguero,
            (vertical - 0.45) * 2.2 + (grano - 0.5) * 0.4,
        )
    })
}

/// `F-14` · el tapiz: discos a tres escalas sobre un campo de densidad con
/// claros, y bordes deformados.
///
/// - El **campo** es ruido de baja frecuencia con contraste: donde vale cero
///   queda un claro verde, donde vale uno una deriva densa.
/// - Tres **escalas** de disco —racimos, manchas y salpicado— entran a
///   distinta densidad, así que ninguna mancha se repite al mismo tamaño.
/// - El **desplazamiento** de dominio rompe el borde redondo.
///
/// El salpicado más fino mide unos dos píxeles en la hero: más pequeño sería
/// centelleo bajo el muestreo por vecino más cercano.
fn textura_tapiz(semilla: u32) -> Texture {
    let oscuro = Color::from_srgb(0.20, 0.36, 0.14);
    let claro = Color::from_srgb(0.40, 0.58, 0.22);
    let ocre = Color::from_srgb(0.52, 0.53, 0.26);

    textura(256, 256, |u, v| {
        let pasto = fbm(u, v, 8, 3, semilla + 1);
        let mut color = mezclar(oscuro, claro, (pasto - 0.30) * 1.9);

        let seco = ruido(u, v, 5, 5, semilla + 2);
        if seco > 0.72 {
            color = mezclar(color, ocre, (seco - 0.72) * 2.5);
        }

        let campo = fbm(u, v, 3, 3, semilla + 3);
        let densidad = ((campo - 0.40) / 0.22).clamp(0.0, 1.0);

        let du = 0.03 * (fbm(u, v, 9, 2, semilla + 4) - 0.5);
        let dv = 0.03 * (fbm(u, v, 9, 2, semilla + 5) - 0.5);
        let (wu, wv) = (u + du, v + dv);

        let escalas = [
            disco(wu, wv, 7, densidad * 0.32, semilla + 6),
            disco(wu, wv, 13, densidad * 0.66 - 0.13, semilla + 7),
            disco(wu, wv, 22, densidad * 0.90 - 0.33, semilla + 8),
        ];

        if escalas.contains(&Some(true)) {
            magenta()
        } else if escalas.contains(&Some(false)) {
            rosa()
        } else {
            color
        }
    })
}

/// Copas: manchas verdes redondeadas, sin silueta de follaje.
fn textura_copa() -> Texture {
    let sombra = Color::from_srgb(0.14, 0.27, 0.12);
    let luz = Color::from_srgb(0.30, 0.47, 0.19);

    textura(64, 64, |u, v| {
        mezclar(sombra, luz, (fbm(u, v, 3, 3, 0xC0FA_0001) - 0.3) * 2.0)
    })
}

/// Acentos: macizos bajos de flor, más magenta que verde.
fn textura_acento() -> Texture {
    let hoja = Color::from_srgb(0.19, 0.33, 0.13);

    textura(64, 64, |u, v| match disco(u, v, 2, 0.95, 0xACE7_0001) {
        Some(true) => magenta(),
        Some(false) => rosa(),
        None => hoja,
    })
}

/// El agua de una caída a lo largo: espuma al nacer y al desembocar, hebras
/// y huecos por el medio. Solo modula el diez por ciento de color propio.
fn textura_caida(semilla: u32) -> Texture {
    let espuma = Color::from_srgb(1.00, 1.00, 1.00);
    let celeste = Color::from_srgb(0.62, 0.82, 0.94);
    let hueco = Color::from_srgb(0.16, 0.25, 0.33);

    textura(24, 128, |u, v| {
        let hebra = ruido(u, v, 5, 3, semilla);
        let pulso = fbm(u, v, 3, 3, semilla.wrapping_add(1));
        let mut color = mezclar(celeste, espuma, hebra * 1.0 + pulso * 0.7 - 0.55);

        if hebra < 0.34 {
            color = mezclar(color, hueco, (0.34 - hebra) * 3.5);
        }

        let nacimiento = ((v - 0.84) / 0.16).clamp(0.0, 1.0);
        let desembocadura = ((0.14 - v) / 0.14).clamp(0.0, 1.0);

        mezclar(color, espuma, nacimiento.max(desembocadura))
    })
}

// ---------------------------------------------------------------------
// Materiales
// ---------------------------------------------------------------------

/// Semilla del tapiz de la lámina grande y de las cuatro pequeñas. Cada
/// lámina pequeña lleva la suya: todas empiezan en `uv = 0`, y con una sola
/// textura mostrarían el mismo recorte. La tercera se eligió porque con la
/// correlativa la lámina más visible del flanco derecho caía en un claro.
const SEMILLA_TAPIZ_AMPLIO: u32 = 0x7A92_0000;
const SEMILLAS_TAPIZ: [u32; 4] = [0x7A93_0000, 0x7A94_0000, 0x7A9A_0000, 0x7A96_0000];

/// `(semilla de textura, fuerza, exponente)` del brillo de las dos aguas:
/// `[caida, caida_fina]`.
///
/// Con el exponente `128` del agua vigente el brillo no llega a la hero:
/// en las caras de las caídas `n·h` vale de `0.75` a `0.84` con `L-01`, y
/// `0.8^128` es cero. El barrido del preview, contraste medio caída/roca en
/// la hero, dio `4.02` con el brillo vigente, `5.98` con este y `7.16` con
/// `1.00 / 4`, que ya se leía como franja uniforme y cálida. Ninguno quemó
/// un píxel.
const BRILLO: [(u32, f32, f32); 2] = [(0xCA1D_1001, 0.80, 6.0), (0xCA1D_2002, 0.65, 9.0)];

/// Tinte del agua, en sRGB.
const TINTE_DEL_AGUA: [f32; 3] = [0.90, 0.96, 1.00];

#[derive(Debug, Clone, Copy)]
struct Materiales {
    zocalo: MaterialId,
    roca: MaterialId,
    frente: MaterialId,
    tapiz: [MaterialId; 4],
    tapiz_amplio: MaterialId,
    copa: MaterialId,
    caida: MaterialId,
    caida_fina: MaterialId,
    acento: MaterialId,
    madera: MaterialId,
}

fn con_textura(scene: &mut Scene, base: Material, tex: Texture, escala: f32) -> MaterialId {
    let id = scene.add_texture(tex);

    scene.add_material(base.with_texture(id).with_uv_scale(escala))
}

/// Registra los materiales de Praderas al final de la paleta.
///
/// El orden de registro forma parte del resultado —fija los índices de
/// material y textura— y es el del preview aprobado.
fn registrar(scene: &mut Scene, madera: MaterialId, agua: Material) -> Materiales {
    let blanco = Material::new(Color::new(1.0, 1.0, 1.0));
    let mojado = blanco.with_specular(0.55, 64.0);

    // La base y el frente son la misma roca: una textura, dos escalas.
    let frente_tex = scene.add_texture(textura_frente());
    let zocalo = scene.add_material(mojado.with_texture(frente_tex).with_uv_scale(3.0));
    let frente = scene.add_material(mojado.with_texture(frente_tex).with_uv_scale(1.0));
    let roca = con_textura(scene, blanco.with_specular(0.08, 16.0), textura_roca(), 2.0);

    // Una repetición de unos cinco metros en las cinco láminas.
    let pasto = blanco.with_specular(0.04, 8.0);
    let amplio_tex = scene.add_texture(textura_tapiz(SEMILLA_TAPIZ_AMPLIO));
    let tapiz_amplio = scene.add_material(pasto.with_texture(amplio_tex).with_uv_scale(1.25));
    let tapiz = SEMILLAS_TAPIZ.map(|semilla| {
        let tex = scene.add_texture(textura_tapiz(semilla));
        scene.add_material(pasto.with_texture(tex).with_uv_scale(0.35))
    });

    let copa = con_textura(scene, blanco.with_specular(0.04, 8.0), textura_copa(), 1.0);
    let acento = con_textura(
        scene,
        blanco.with_specular(0.04, 8.0),
        textura_acento(),
        1.0,
    );

    // El agua: copia por valor del agua vigente. Solo cambian brillo,
    // textura y tinte; techos, `ior` y modo de sombra son los suyos.
    let [r, g, b] = TINTE_DEL_AGUA;
    let tinte = Color::from_srgb(r, g, b);
    let [caida, caida_fina] = BRILLO.map(|(semilla, fuerza, exponente)| {
        let base = agua.with_specular(fuerza, exponente);
        let id = con_textura(scene, base, textura_caida(semilla), 1.0);
        scene.palette[id.0] = scene.material(id).with_tint(tinte);
        id
    });

    Materiales {
        zocalo,
        roca,
        frente,
        tapiz,
        tapiz_amplio,
        copa,
        caida,
        caida_fina,
        acento,
        madera,
    }
}

// ---------------------------------------------------------------------
// Composición
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
enum Acabado {
    Zocalo,
    Roca,
    Frente,
    /// El tapiz de la lámina pequeña `k`.
    Tapiz(usize),
    TapizAmplio,
    Madera,
    Copa,
    Caida,
    CaidaFina,
    Acento,
}

/// Una caja relativa al ancla de Praderas: `[x0, y0, z0, x1, y1, z1]`.
type Caja = [f32; 6];

/// Lo que se le hunde a una pieza en su apoyo: separa caras coplanares.
const EMPOTRADO: f32 = 0.04;

/// Grosor de una lámina de tapiz por encima de su apoyo.
const LAMINA: f32 = 0.08;

/// Lo que sobresale una caída por encima de su labio: su nacimiento se ve
/// desde arriba como una marca clara en el borde.
const NACIMIENTO: f32 = 0.02;

/// Profundidad de una caída, contra la cara que la respalda.
const ESPESOR_DE_CAIDA: f32 = 0.14;

/// Las caídas de menos de este ancho llevan el agua `caida_fina`.
const ANCHO_DE_CAIDA_FINA: f32 = 0.19;

/// `P-01` · la base flotante, idéntica en caja a la masa mayor del nivel
/// seguro.
const BASE: Caja = [-3.5, -1.4, -2.8, 3.5, 0.0, 2.8];

/// `P-01` · las cinco terrazas, cada una mordiendo la base `0.10`, con
/// contorno escalonado y asimétrico:
///
/// - pared izquierda del fondo, que vuela un poco por detrás;
/// - pared derecha retranqueada, la de las caídas interiores, en roca
///   oscura (`OSCURAS`);
/// - cresta central baja, el fondo tranquilo del Monolito;
/// - escalón bajo a la izquierda del frente;
/// - escalón saliente a la derecha, que vuela `0.30` sobre el canto. No va
///   al centro: ahí cualquier saliente subiría la rasante y taparía más
///   Monolito desde el lado opuesto a la hero.
const TERRAZAS: [Caja; 5] = [
    [-3.38, -0.10, -2.85, -1.55, 1.35, -1.05],
    [1.05, -0.10, -2.60, 3.38, 1.45, -1.50],
    [-1.40, -0.10, -2.68, 1.32, 0.70, -1.95],
    [-3.38, -0.10, 0.55, -1.75, 0.38, 2.62],
    [1.35, -0.10, 0.85, 2.40, 0.55, 3.10],
];

/// Qué terrazas llevan la roca oscura del frente en vez de la cálida.
const OSCURAS: [bool; 5] = [false, true, false, false, false];

/// `P-02` · ocho repisas colgadas de la base, mordiéndola `0.10`. Las cuatro
/// que respaldan una caída tienen la cara entre `2.785` y `2.79`: a ras de
/// la base pero no coplanares con ella.
const FRENTE: [Caja; 8] = [
    [-3.45, -3.10, 2.00, -2.65, -1.30, 2.79],
    [-2.70, -2.25, 1.95, -2.05, -1.30, 2.785],
    [-2.10, -2.75, 2.10, -1.15, -1.30, 3.20],
    [-1.20, -3.40, 1.90, -0.25, -1.30, 2.95],
    [-0.30, -2.50, 2.05, 0.85, -1.30, 3.30],
    [0.80, -3.00, 1.90, 1.85, -1.30, 3.05],
    [1.80, -3.45, 2.00, 2.70, -1.30, 2.79],
    [2.65, -2.20, 1.95, 3.45, -1.30, 2.785],
];

/// `P-03` · cinco láminas de tapiz: `(planta [x0, z0, x1, z1], cota del
/// apoyo, amplia)`. La grande cubre todo el techo de la base; las demás,
/// cuatro terrazas. La cresta central se queda en roca.
const LAMINAS: [([f32; 4], f32, bool); 5] = [
    ([-3.44, -1.88, 3.44, 2.74], 0.0, true),
    ([-3.35, 0.58, -1.78, 2.59], 0.38, false),
    ([-3.35, -2.82, -1.58, -1.08], 1.35, false),
    ([1.40, 0.90, 2.35, 3.05], 0.55, false),
    ([1.08, -2.57, 3.35, -1.55], 1.45, false),
];

/// `P-04` · dos árboles de tres primitivas: `(pie del tronco, escala)`.
const ARBOLES: [([f32; 3], f32); 2] = [([-3.05, 0.08, -0.85], 0.70), ([3.08, 0.08, 2.20], 0.66)];

/// `P-05` · seis caídas: `(x, ancho, pie, labio, cara)`. Cada una nace a la
/// cota `labio` de la pieza cuya cara frontal está en `z = cara`. Las cuatro
/// al vacío desembocan en el canto inferior de su roca; las dos interiores
/// caen de la pared derecha al llano.
const CAIDAS: [(f32, f32, f32, f32, f32); 6] = [
    (-3.15, 0.28, -3.08, 0.0, 2.80),
    (-2.62, 0.16, -2.23, 0.0, 2.80),
    (2.62, 0.22, -3.43, 0.0, 2.80),
    (3.18, 0.14, -2.18, 0.0, 2.80),
    (2.80, 0.20, 0.0, 1.45, -1.50),
    (3.22, 0.26, 0.0, 1.45, -1.50),
];

/// `P-07` · seis macizos bajos de flor, cada uno posado en su apoyo.
const ACENTOS: [Caja; 6] = [
    [2.95, 0.04, 2.40, 3.44, 0.20, 2.72],
    [-3.30, 0.42, 2.00, -2.55, 0.62, 2.62],
    [2.62, 0.04, -1.36, 3.30, 0.20, -0.95],
    [-3.25, 1.39, -2.40, -2.70, 1.58, -1.85],
    [2.95, 0.04, 0.25, 3.42, 0.18, 0.70],
    [-3.42, 0.04, -0.62, -2.80, 0.20, -0.22],
];

/// Las 37 piezas, en el orden de los índices de Praderas: mismo hueco, otra
/// forma.
fn piezas() -> Vec<(Caja, Acabado)> {
    let mut piezas = Vec::with_capacity(PIEZAS);

    piezas.push((BASE, Acabado::Zocalo));
    for (&caja, oscura) in TERRAZAS.iter().zip(OSCURAS) {
        let acabado = if oscura {
            Acabado::Zocalo
        } else {
            Acabado::Roca
        };
        piezas.push((caja, acabado));
    }
    piezas.extend(FRENTE.iter().map(|&c| (c, Acabado::Frente)));

    let mut pequenas = 0;
    for &([x0, z0, x1, z1], apoyo, amplia) in &LAMINAS {
        let acabado = if amplia {
            Acabado::TapizAmplio
        } else {
            pequenas += 1;
            Acabado::Tapiz(pequenas - 1)
        };
        piezas.push(([x0, apoyo - EMPOTRADO, z0, x1, apoyo + LAMINA, z1], acabado));
    }

    for &([x, y, z], s) in &ARBOLES {
        let centrada = |dx: f32, cy: f32, dz: f32, lado: f32, alto: f32| -> Caja {
            let (cx, cz) = (x + dx * s, z + dz * s);
            let (l, a) = (lado * s * 0.5, alto * s * 0.5);
            let cy = y + cy * s;
            [cx - l, cy - a, cz - l, cx + l, cy + a, cz + l]
        };
        piezas.push((centrada(0.0, 0.55, 0.0, 0.22, 1.1), Acabado::Madera));
        piezas.push((centrada(0.05, 1.35, 0.0, 1.15, 0.75), Acabado::Copa));
        piezas.push((centrada(-0.10, 1.85, 0.08, 0.75, 0.55), Acabado::Copa));
    }

    for &(x, ancho, pie, labio, cara) in &CAIDAS {
        let z0 = cara - 0.02;
        let acabado = if ancho < ANCHO_DE_CAIDA_FINA {
            Acabado::CaidaFina
        } else {
            Acabado::Caida
        };
        piezas.push((
            [
                x - ancho * 0.5,
                pie,
                z0,
                x + ancho * 0.5,
                labio + NACIMIENTO,
                z0 + ESPESOR_DE_CAIDA,
            ],
            acabado,
        ));
    }

    piezas.extend(ACENTOS.iter().map(|&c| (c, Acabado::Acento)));

    piezas
}

// ---------------------------------------------------------------------
// API
// ---------------------------------------------------------------------

/// Piezas de Praderas: las del inventario del nivel seguro.
const PIEZAS: usize = 37;

/// Posición, dentro de las piezas de Praderas en el orden de
/// `meadows::praderas`, del primer tronco (`P-04`) y de la cascada (`P-05`).
/// De ellas salen `aged_wood` y el agua vigente sin tener que pasar la
/// paleta hasta aquí.
const PRIMER_TRONCO: usize = 18;
const CASCADA: usize = 24;

/// Reescribe en su sitio las 37 piezas de Praderas de una entrega.
///
/// `ancla` es el ancla de Praderas **ya repartida** por la composición de la
/// entrega. Hay que llamarla antes de medir la escala y de construir la
/// jerarquía, que guarda posiciones.
pub fn aplicar(scene: &mut Scene, ancla: Vec3) {
    let praderas: Vec<usize> = scene
        .objects
        .iter()
        .enumerate()
        .filter(|(_, o)| o.spatial_group == SpatialGroupId::Meadows)
        .map(|(i, _)| i)
        .collect();

    assert_eq!(
        praderas.len(),
        PIEZAS,
        "Praderas tiene que llegar con las piezas del inventario"
    );

    let madera = scene.objects[praderas[PRIMER_TRONCO]].final_material;
    let agua = scene.material(scene.objects[praderas[CASCADA]].final_material);
    assert!(
        agua.is_reflective_or_refractive(),
        "la pieza {CASCADA} de Praderas tiene que ser la cascada de agua"
    );

    let m = registrar(scene, madera, agua);

    for (&i, ([x0, y0, z0, x1, y1, z1], acabado)) in praderas.iter().zip(piezas()) {
        let objeto = &mut scene.objects[i];

        objeto.primitive = Cuboid::new(Aabb::new(
            ancla + Vec3::new(x0, y0, z0),
            ancla + Vec3::new(x1, y1, z1),
        ))
        .into();
        objeto.final_material = match acabado {
            Acabado::Zocalo => m.zocalo,
            Acabado::Roca => m.roca,
            Acabado::Frente => m.frente,
            Acabado::Tapiz(k) => m.tapiz[k],
            Acabado::TapizAmplio => m.tapiz_amplio,
            Acabado::Madera => m.madera,
            Acabado::Copa => m.copa,
            Acabado::Caida => m.caida,
            Acabado::CaidaFina => m.caida_fina,
            Acabado::Acento => m.acento,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::bounds::Aabb;
    use crate::material::ShadowMode;
    use crate::scene::{Scene, SpatialGroupId};
    use crate::scene_builder::measure_scene_radius;
    use crate::scenes::{
        delivery_level, delivery_level_previo, delivery_level_previo_aguas,
        delivery_level_previo_cuenca, safe_level, WaterPreset, DELIVERY,
    };

    const PRESETS: [WaterPreset; 3] = [
        WaterPreset::RefractiveWater,
        WaterPreset::InteriorVisible,
        WaterPreset::OpaqueWater,
    ];

    fn praderas(scene: &Scene) -> Vec<usize> {
        scene
            .objects
            .iter()
            .enumerate()
            .filter(|(_, o)| o.spatial_group == SpatialGroupId::Meadows)
            .map(|(i, _)| i)
            .collect()
    }

    fn caja(scene: &Scene, i: usize) -> Aabb {
        scene.objects[i].primitive.bounds()
    }

    /// Las piezas de Praderas con material refractivo: las caídas.
    fn caidas(scene: &Scene) -> Vec<usize> {
        praderas(scene)
            .into_iter()
            .filter(|&i| {
                scene
                    .material(scene.objects[i].final_material)
                    .is_reflective_or_refractive()
            })
            .collect()
    }

    #[test]
    fn la_entrega_por_defecto_lleva_las_seis_caidas_de_agua_aprobadas() {
        for water in PRESETS {
            let previo = delivery_level_previo(water);
            let entrega = delivery_level(water);
            let caidas = caidas(&entrega.scene);

            assert_eq!(
                caidas.len(),
                6,
                "{water:?}: la entrega no lleva seis caidas"
            );

            let mut brillos: Vec<(f32, f32)> = Vec::new();
            for &i in &caidas {
                let id = entrega.scene.objects[i].final_material;
                let m = entrega.scene.material(id);

                assert!(
                    id.0 >= previo.scene.palette.len(),
                    "{water:?}: la caida {i} usa el agua compartida"
                );
                assert_eq!(m.reflection_cap, 0.9);
                assert_eq!(m.transmission_cap, 0.9);
                assert_eq!(m.ior, 1.333);
                assert_eq!(m.shadow_mode, ShadowMode::Ignore);
                assert!(m.albedo_texture.is_some(), "agua sin textura procedural");
                brillos.push((m.specular_strength, m.shininess));
            }

            brillos.sort_by(|a, b| a.partial_cmp(b).expect("sin NaN"));
            brillos.dedup();
            assert_eq!(
                brillos,
                vec![(0.65, 9.0), (0.80, 6.0)],
                "{water:?}: brillo distinto del aprobado"
            );
        }
    }

    #[test]
    fn fuera_de_praderas_la_entrega_es_la_previa_byte_a_byte() {
        // La etapa de Praderas se mide antes de la promoción de Aguas, que
        // tiene su propia comprobación en `flying_waters_delivery`.
        for water in PRESETS {
            let a = delivery_level_previo(water);
            let b = delivery_level_previo_aguas(water);

            assert_eq!(a.scene.objects.len(), b.scene.objects.len());
            for (i, (x, y)) in a.scene.objects.iter().zip(&b.scene.objects).enumerate() {
                assert_eq!(x.spatial_group, y.spatial_group, "{i}");
                assert_eq!(x.reveal_group, y.reveal_group, "{i}");
                if x.spatial_group == SpatialGroupId::Meadows {
                    assert_eq!(
                        x.initial_material, y.initial_material,
                        "{i} no nace en lienzo"
                    );
                    continue;
                }
                assert_eq!(format!("{x:?}"), format!("{y:?}"), "{water:?}: objeto {i}");
            }

            // Lo compartido no se toca: lo de Praderas va al final.
            assert!(b.scene.palette.len() > a.scene.palette.len());
            for (i, (x, y)) in a.scene.palette.iter().zip(&b.scene.palette).enumerate() {
                assert_eq!(format!("{x:?}"), format!("{y:?}"), "material {i}");
            }
            assert_eq!(
                format!("{:?}", a.scene.skybox),
                format!("{:?}", b.scene.skybox)
            );

            assert_eq!(format!("{:?}", a.anchors), format!("{:?}", b.anchors));
            assert_eq!(format!("{:?}", a.scale), format!("{:?}", b.scale));
            assert_eq!(a.hero_preset(), b.hero_preset());
            assert_eq!(
                format!("{:?}", a.accel.bounds),
                format!("{:?}", b.accel.bounds)
            );
            for (x, y) in a.accel.groups.iter().zip(&b.accel.groups) {
                assert_eq!(x.id, y.id);
                if x.id != SpatialGroupId::Meadows {
                    assert_eq!(format!("{x:?}"), format!("{y:?}"), "{water:?}: {:?}", x.id);
                }
            }
        }
    }

    #[test]
    fn praderas_cuenta_37_y_la_entrega_168() {
        for water in PRESETS {
            // La etapa de Praderas: desde la promoción de la cuenca,
            // `delivery_level` añade sus piezas y alarga cuatro caídas, fuera
            // del territorio de Praderas; eso lo comprueba
            // `monolith_basin_delivery`.
            let entrega = delivery_level_previo_cuenca(water);
            let sin_volumen = usize::from(water == WaterPreset::InteriorVisible);

            assert_eq!(praderas(&entrega.scene).len(), 37);
            assert_eq!(praderas(&entrega.scene).len(), DELIVERY.meadows);
            assert_eq!(entrega.scene.objects.len(), DELIVERY.total() - sin_volumen);
        }
    }

    #[test]
    fn praderas_no_sale_de_su_territorio_ni_mueve_la_escala() {
        let previo = delivery_level_previo(WaterPreset::RefractiveWater);
        // La etapa de Praderas: desde la promoción de la cuenca,
        // `delivery_level` añade sus piezas y alarga cuatro caídas, fuera
        // del territorio de Praderas; eso lo comprueba
        // `monolith_basin_delivery`.
        let entrega = delivery_level_previo_cuenca(WaterPreset::RefractiveWater);
        let territorio = praderas(&previo.scene)
            .into_iter()
            .map(|i| caja(&previo.scene, i))
            .reduce(|a, b| a.union(&b))
            .expect("Praderas tiene piezas");

        for i in praderas(&entrega.scene) {
            let p = caja(&entrega.scene, i);
            assert!(
                p.min.x >= territorio.min.x - 1e-4
                    && p.min.y >= territorio.min.y - 1e-4
                    && p.min.z >= territorio.min.z - 1e-4
                    && p.max.x <= territorio.max.x + 1e-4
                    && p.max.y <= territorio.max.y + 1e-4
                    && p.max.z <= territorio.max.z + 1e-4,
                "la pieza {i} sale del territorio: {p:?}"
            );
        }

        assert_eq!(
            measure_scene_radius(&entrega.scene, entrega.anchors.orbit_center),
            previo.scale.scene_radius
        );
    }

    /// Apoyo por contacto: cada pieza toca o muerde otra, y todas llegan a
    /// la base flotante —la de mayor planta— por esa cadena. El preview
    /// comprueba además el apoyo por rol (posada, colgada, engarzada, caída
    /// nacida en su labio) sobre la misma escena.
    #[test]
    fn cada_pieza_de_praderas_llega_a_la_base_por_contacto() {
        let entrega = delivery_level(WaterPreset::RefractiveWater);
        let cajas: Vec<Aabb> = praderas(&entrega.scene)
            .into_iter()
            .map(|i| caja(&entrega.scene, i))
            .collect();
        let area = |c: &Aabb| (c.max.x - c.min.x) * (c.max.z - c.min.z);
        let base = (0..cajas.len())
            .max_by(|&a, &b| area(&cajas[a]).total_cmp(&area(&cajas[b])))
            .expect("hay piezas");
        let tocan = |a: &Aabb, b: &Aabb| {
            a.min.x <= b.max.x + 1e-4
                && b.min.x <= a.max.x + 1e-4
                && a.min.y <= b.max.y + 1e-4
                && b.min.y <= a.max.y + 1e-4
                && a.min.z <= b.max.z + 1e-4
                && b.min.z <= a.max.z + 1e-4
        };

        let mut conectada = vec![false; cajas.len()];
        conectada[base] = true;
        loop {
            let mut cambio = false;
            for k in 0..cajas.len() {
                if !conectada[k]
                    && (0..cajas.len()).any(|s| conectada[s] && tocan(&cajas[k], &cajas[s]))
                {
                    conectada[k] = true;
                    cambio = true;
                }
            }
            if !cambio {
                break;
            }
        }

        let sueltas: Vec<usize> = (0..cajas.len()).filter(|&k| !conectada[k]).collect();
        assert!(
            sueltas.is_empty(),
            "piezas sin contacto con la base: {sueltas:?}"
        );

        // Y ninguna pieza de la entrega previa queda como estaba: las 37 se
        // reescriben, no se añaden encima.
        let previo = delivery_level_previo(WaterPreset::RefractiveWater);
        let iguales = praderas(&previo.scene)
            .into_iter()
            .filter(|&i| caja(&previo.scene, i) != caja(&entrega.scene, i))
            .count();
        assert!(iguales >= 36, "solo cambiaron {iguales} de 37 piezas");
    }

    #[test]
    fn praderas_en_la_entrega_es_determinista() {
        let a = delivery_level(WaterPreset::RefractiveWater);
        let b = delivery_level(WaterPreset::RefractiveWater);

        assert_eq!(
            format!("{:?}", a.scene.objects),
            format!("{:?}", b.scene.objects)
        );
        assert_eq!(
            format!("{:?}", a.scene.palette),
            format!("{:?}", b.scene.palette)
        );
        assert_eq!(a.scene.textures.len(), b.scene.textures.len());
    }

    #[test]
    fn el_nivel_seguro_conserva_las_praderas_historicas() {
        // La línea base de los hitos 3 a 7 no se toca: una sola cascada, la
        // del agua compartida, y las 154 primitivas.
        let seguro = safe_level(WaterPreset::RefractiveWater);

        assert_eq!(seguro.scene.objects.len(), 154);
        assert_eq!(caidas(&seguro.scene).len(), 1);
    }

    /// FNV-1a de 64 bits sobre las piezas de Praderas, los materiales y las
    /// texturas nuevos (texel a texel), sin assets.
    fn huella(entrega: &crate::scene_builder::Blockout, previo: &Scene) -> u64 {
        let scene = &entrega.scene;
        let mut h: u64 = 0xCBF2_9CE4_8422_2325;
        let mut mezclar = |texto: String| {
            for b in texto.bytes() {
                h ^= u64::from(b);
                h = h.wrapping_mul(0x0100_0000_01B3);
            }
        };

        for i in praderas(scene) {
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

    /// Huella de la Praderas de producción, tomada **después** de que el
    /// test `la_entrega_de_produccion_es_el_pulido_de_agua_aprobado` del
    /// preview comprobara, con los assets reales, que esta entrega es byte a
    /// byte la variante aprobada. Aquí no hay assets —es el convenio de los
    /// tests de la biblioteca—, así que es un seguro de regresión, no la
    /// prueba de equivalencia.
    const HUELLA_DE_PRADERAS: u64 = 11_950_215_931_138_958_719;

    #[test]
    fn la_huella_de_praderas_queda_fijada() {
        // Sobre la etapa de Praderas: la promoción de Aguas añade materiales
        // al final de la paleta y no debe mover esta huella.
        let previo = delivery_level_previo(WaterPreset::RefractiveWater);
        let entrega = delivery_level_previo_aguas(WaterPreset::RefractiveWater);

        assert_eq!(
            huella(&entrega, &previo.scene),
            HUELLA_DE_PRADERAS,
            "Praderas de produccion cambio respecto de la aprobada"
        );
    }
}
