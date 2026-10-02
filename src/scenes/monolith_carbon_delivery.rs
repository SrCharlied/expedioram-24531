//! El Monolito de carbón con el **33** hacia Praderas: la composición
//! aprobada.
//!
//! Es la variante `carbon_33` del preview (`examples/monolith_carbon_preview.rs`),
//! aprobada por Charlie como default. Este módulo es su versión de
//! producción: el mismo resultado, sin el arnés. Un test del preview
//! comprueba con los assets reales que la entrega que sale de aquí es la
//! escena aprobada —objetos, materiales efectivos, texturas texel a texel,
//! anclas, escala, cámaras, luces, jerarquía y renders—.
//!
//! # Qué hace
//!
//! `aplicar`, sobre la entrega ya compuesta —después de retirar `A-01` si el
//! preset lo pide y antes de medir y acelerar—:
//!
//! 1. Los diez tramos del Monolito acaban en **carbón**: un material y una
//!    textura propios —gris oscuro mate con grano, opaco, sin reflejo ni
//!    refracción y con un brillo local contenido—. El cristal pictórico y su
//!    textura no se tocan; el lienzo inicial, el revelado `Finale` y la
//!    geometría tampoco.
//! 2. Añade **una** placa delgada (`GROSOR`) apoyada en la cara del tramo
//!    central que mira a Praderas (`-Z`, de las anclas), con el **33** en
//!    marfil. Un material vale para el objeto entero —no hay mapeo por cara
//!    sin tocar el motor—, de ahí la placa. Su fondo es el carbón de la cara
//!    horneado en sus mismas coordenadas, y su lienzo inicial, el lienzo de
//!    la cara horneado igual: solo se distingue la tinta.
//!
//! Son `+1` primitiva, `+3` materiales y `+3` texturas: `219` en los presets
//! con volumen y `218` en `InteriorVisible`.
//!
//! # Por qué así
//!
//! - **La cara.** Desde Praderas, la de la base queda oculta por la meseta;
//!   la del tramo central se ve entera desde `y = 4.6` (medido con rayos en
//!   el preview).
//! - **El espejo.** En las caras `Z` del cuboide `u` crece con `+X`, y desde
//!   Praderas —mirando hacia `+Z`— la derecha de la pantalla es `-X`: la
//!   tinta se escribe invertida en `u` para leerse derecha.
//! - **El color.** `L-03`, el acento cian del Monolito, ilumina de lleno esa
//!   cara. El marfil y el carbón van compensados para leerse marfil cálido y
//!   gris carbón bajo él; las luces no se tocan.

use nalgebra_glm::{Vec2, Vec3};

use crate::bounds::Aabb;
use crate::color::Color;
use crate::cuboid::Cuboid;
use crate::material::{Material, ShadowMode};
use crate::primitive::Primitive;
use crate::scene::{MaterialId, RevealGroup, Scene, SceneObject, SpatialGroupId, TextureId};
use crate::texture::{Texture, WrapMode};

/// Primitivas, materiales y texturas que añade.
pub const PIEZAS: usize = 1;
pub const MATERIALES: usize = 3;
pub const TEXTURAS: usize = 3;

/// Margen lateral de la placa respecto de la cara del tramo.
pub const MARGEN_X: f32 = 0.08;
/// Alto de la placa, dentro de la franja que se ve desde Praderas.
pub const PLACA_Y: (f32, f32) = (5.3, 6.9);
/// Grosor: hacia Praderas, apoyada sobre la cara.
pub const GROSOR: f32 = 0.004;
/// Alto de los dígitos en mundo.
pub const ALTO_DIGITO: f32 = 1.2;
/// Ancho de un dígito y hueco entre los dos, en altos de dígito.
pub const ANCHO_DIGITO: f32 = 0.52;
pub const HUECO_DIGITOS: f32 = 0.12;
/// Texels por unidad de mundo de las texturas de la placa.
pub const TEXELS_POR_UNIDAD: f32 = 400.0;

/// Marfil cálido pálido del número, en sRGB, compensado por `L-03`.
pub const MARFIL: (f32, f32, f32) = (1.0, 0.79, 0.60);
/// Tono base del carbón en sRGB, compensado igual.
pub const CARBON: (f32, f32, f32) = (0.185, 0.17, 0.168);

/// Lado de la textura de carbón y su semilla.
pub const LADO_CARBON: usize = 128;
const SEMILLA: u32 = 0x0C4A_2B33;

/// Lo que `aplicar` añadió, por índice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Monolito {
    pub placa: usize,
    pub tramo: usize,
    pub carbon: MaterialId,
    pub numero: MaterialId,
    pub lienzo_de_la_placa: MaterialId,
    pub textura_carbon: TextureId,
    pub textura_numero: TextureId,
    pub textura_lienzo: TextureId,
}

pub fn indices_del_monolito(scene: &Scene) -> Vec<usize> {
    (0..scene.objects.len())
        .filter(|&i| scene.objects[i].spatial_group == SpatialGroupId::Monolith)
        .collect()
}

/// El tramo central: el de `y` de `4.4` a `7.6`, cuya cara hacia Praderas
/// es la que se ve entera desde allí.
pub fn tramo_del_33(scene: &Scene) -> usize {
    indices_del_monolito(scene)
        .into_iter()
        .find(|&i| {
            let b = scene.objects[i].primitive.bounds();
            (b.min.y - 4.4).abs() < 1e-3 && (b.max.y - 7.6).abs() < 1e-3
        })
        .expect("el tramo central del Monolito")
}

/// La caja de la placa sobre la cara `-Z` del tramo.
pub fn caja_de_la_placa(tramo: &Aabb) -> Aabb {
    Aabb::from_corners(
        Vec3::new(tramo.min.x + MARGEN_X, PLACA_Y.0, tramo.min.z - GROSOR),
        Vec3::new(tramo.max.x - MARGEN_X, PLACA_Y.1, tramo.min.z),
    )
}

// --------------------------------------------------------------- el dígito

/// Tinta del **33** en coordenadas de **lectura**: `s` crece hacia la
/// derecha de quien lo lee y `t` hacia arriba, ambas en altos de dígito, con
/// el origen en la esquina inferior izquierda del bloque. Devuelve la
/// distancia con signo al trazo (negativa dentro).
pub fn distancia_al_33(s: f32, t: f32) -> f32 {
    let segundo = ANCHO_DIGITO + HUECO_DIGITOS;
    distancia_al_3(s, t).min(distancia_al_3(s - segundo, t))
}

/// Ancho del bloque `33`, en altos de dígito.
pub fn ancho_del_bloque() -> f32 {
    2.0 * ANCHO_DIGITO + HUECO_DIGITOS
}

/// Medio grosor del trazo, en altos de dígito.
const TRAZO: f32 = 0.075;

/// Un «3» de dos panzas: dos arcos abiertos a la izquierda —el de arriba
/// algo menor— y una lengüeta horizontal donde se juntan.
fn distancia_al_3(s: f32, t: f32) -> f32 {
    let arriba = arco(s, t, (0.24, 0.735), 0.19, -90.0, 140.0);
    let abajo = arco(s, t, (0.24, 0.29), 0.205, -140.0, 90.0);
    let lengua = segmento(s, t, (0.13, 0.52), (0.24, 0.52));
    arriba.min(abajo).min(lengua) - TRAZO
}

/// Distancia (sin grosor) a un arco entre dos ángulos en grados, recorrido
/// en sentido antihorario de `desde` a `hasta`.
fn arco(s: f32, t: f32, c: (f32, f32), r: f32, desde: f32, hasta: f32) -> f32 {
    let (dx, dy) = (s - c.0, t - c.1);
    let angulo = dy.atan2(dx).to_degrees();
    if angulo >= desde && angulo <= hasta {
        return ((dx * dx + dy * dy).sqrt() - r).abs();
    }
    let punta = |g: f32| {
        let g = g.to_radians();
        let (px, py) = (c.0 + r * g.cos(), c.1 + r * g.sin());
        ((s - px).powi(2) + (t - py).powi(2)).sqrt()
    };
    punta(desde).min(punta(hasta))
}

fn segmento(s: f32, t: f32, a: (f32, f32), b: (f32, f32)) -> f32 {
    let (abx, aby) = (b.0 - a.0, b.1 - a.1);
    let k = (((s - a.0) * abx + (t - a.1) * aby) / (abx * abx + aby * aby)).clamp(0.0, 1.0);
    ((s - a.0 - k * abx).powi(2) + (t - a.1 - k * aby).powi(2)).sqrt()
}

// ----------------------------------------------------------------- texturas

fn azar(x: u32, y: u32, octava: u32) -> f32 {
    let mut h = x.wrapping_mul(0x8DA6_B343)
        ^ y.wrapping_mul(0xD816_3841)
        ^ octava.wrapping_mul(0xCB1A_B31F)
        ^ SEMILLA;
    h ^= h >> 13;
    h = h.wrapping_mul(0x5BD1_E995);
    h ^= h >> 15;
    (h & 0x00FF_FFFF) as f32 / 16_777_215.0
}

/// Ruido de valor periódico: celdas de `celda` texels, interpolación suave,
/// índices envueltos para que la textura repita sin costura.
fn ruido(x: usize, y: usize, celda: usize, octava: u32) -> f32 {
    let n = (LADO_CARBON / celda) as u32;
    let (fx, fy) = (x as f32 / celda as f32, y as f32 / celda as f32);
    let (x0, y0) = (fx.floor() as u32, fy.floor() as u32);
    let suave = |f: f32| f * f * (3.0 - 2.0 * f);
    let (tx, ty) = (suave(fx - x0 as f32), suave(fy - y0 as f32));
    let v = |i: u32, j: u32| azar(i % n, j % n, octava);
    let a = v(x0, y0) + (v(x0 + 1, y0) - v(x0, y0)) * tx;
    let b = v(x0, y0 + 1) + (v(x0 + 1, y0 + 1) - v(x0, y0 + 1)) * tx;
    a + (b - a) * ty
}

/// Carbón: gris oscuro con grano de cuatro octavas y vetas claras escasas.
/// Sin brillo horneado. `Repeat`.
pub fn textura_de_carbon() -> Texture {
    let mut pixels = Vec::with_capacity(LADO_CARBON * LADO_CARBON);
    for y in 0..LADO_CARBON {
        for x in 0..LADO_CARBON {
            let n = 0.45 * ruido(x, y, 32, 1)
                + 0.27 * ruido(x, y, 16, 2)
                + 0.17 * ruido(x, y, 8, 3)
                + 0.11 * ruido(x, y, 4, 4);
            let veta = ((ruido(x, y, 8, 5) - 0.78).max(0.0) * 4.0).min(1.0);
            let k = 0.80 + 0.40 * n + 0.18 * veta;
            pixels.push(Color::from_srgb(CARBON.0 * k, CARBON.1 * k, CARBON.2 * k));
        }
    }
    Texture::from_pixels(LADO_CARBON, LADO_CARBON, pixels).expect("dimensiones validas")
}

/// Material del carbón: opaco, sin rebotes, con un brillo local contenido.
pub fn material_de_carbon(textura: TextureId) -> Material {
    Material::new(Color::from_srgb(0.2, 0.2, 0.2))
        .with_texture(textura)
        .with_uv_scale(1.0)
        .with_specular(0.14, 28.0)
}

/// Una textura para la placa: cada texel es el punto del mundo de la cara
/// del tramo que tapa, `color(uv_de_la_cara, tinta)`. La lectura crece al
/// bajar `x`: desde Praderas la derecha de la pantalla es `-X`.
fn hornear_la_placa(tramo: &Aabb, placa: &Aabb, color: impl Fn(Vec2, f32) -> Color) -> Texture {
    let (ancho, alto) = (placa.max.x - placa.min.x, placa.max.y - placa.min.y);
    let (w, h) = (
        (ancho * TEXELS_POR_UNIDAD).round() as usize,
        (alto * TEXELS_POR_UNIDAD).round() as usize,
    );
    let centro_x = (placa.min.x + placa.max.x) / 2.0;
    let y0 = (placa.min.y + placa.max.y) / 2.0 - ALTO_DIGITO / 2.0;
    let mut pixels = Vec::with_capacity(w * h);
    for fila in 0..h {
        for col in 0..w {
            // Cuatro por cuatro muestras por texel para el borde de la
            // tinta; el fondo se toma en el centro.
            let mut cubierta = 0.0;
            for sy in 0..4 {
                for sx in 0..4 {
                    let u = (col as f32 + (sx as f32 + 0.5) / 4.0) / w as f32;
                    let v = 1.0 - (fila as f32 + (sy as f32 + 0.5) / 4.0) / h as f32;
                    let (x, y) = (placa.min.x + u * ancho, placa.min.y + v * alto);
                    let s = (centro_x - x) / ALTO_DIGITO + ancho_del_bloque() / 2.0;
                    let t = (y - y0) / ALTO_DIGITO;
                    if distancia_al_33(s, t) < 0.0 {
                        cubierta += 1.0 / 16.0;
                    }
                }
            }
            let u = (col as f32 + 0.5) / w as f32;
            let v = 1.0 - (fila as f32 + 0.5) / h as f32;
            let (x, y) = (placa.min.x + u * ancho, placa.min.y + v * alto);
            let uv_cara = Vec2::new(
                (x - tramo.min.x) / (tramo.max.x - tramo.min.x),
                (y - tramo.min.y) / (tramo.max.y - tramo.min.y),
            );
            pixels.push(color(uv_cara, cubierta));
        }
    }
    Texture::from_pixels(w, h, pixels)
        .expect("dimensiones validas")
        .with_wrap(WrapMode::Clamp)
}

// -------------------------------------------------------------------- aplicar

/// Aplica el carbón y el 33. Solo añade materiales, texturas y la placa al
/// final, y reasigna el material **final** de los diez tramos.
pub fn aplicar(scene: &mut Scene) -> Monolito {
    // El carbón de los diez tramos.
    let textura_carbon = scene.add_texture(textura_de_carbon());
    let carbon = scene.add_material(material_de_carbon(textura_carbon));
    for i in indices_del_monolito(scene) {
        scene.objects[i].final_material = carbon;
    }

    let tramo = tramo_del_33(scene);
    let caja_tramo = scene.objects[tramo].primitive.bounds();
    let caja = caja_de_la_placa(&caja_tramo);
    let mat_carbon = scene.material(carbon);
    let carbon_t = scene.textures[textura_carbon.0].clone();

    // La placa revelada: el carbón de la cara, en sus coordenadas, con la
    // tinta marfil encima.
    let marfil = Color::from_srgb(MARFIL.0, MARFIL.1, MARFIL.2);
    let numero_t = hornear_la_placa(&caja_tramo, &caja, |uv, cubierta| {
        let fondo = mat_carbon.albedo
            * carbon_t.sample(uv.x * mat_carbon.uv_scale, uv.y * mat_carbon.uv_scale);
        fondo * (1.0 - cubierta) + marfil * cubierta
    });
    let textura_numero = scene.add_texture(numero_t);
    let numero = scene.add_material(Material {
        albedo_texture: Some(textura_numero),
        albedo: Color::new(1.0, 1.0, 1.0),
        uv_scale: 1.0,
        // Es un calco sobre la cara: no hace sombra sobre ella.
        shadow_mode: ShadowMode::Ignore,
        ..mat_carbon
    });

    // La placa sin revelar: el lienzo de la cara, horneado igual.
    let lienzo = scene.material(scene.objects[tramo].initial_material);
    let lienzo_t = {
        let s = &*scene;
        hornear_la_placa(&caja_tramo, &caja, |uv, _| s.albedo_at(&lienzo, &uv))
    };
    let textura_lienzo = scene.add_texture(lienzo_t);
    let lienzo_de_la_placa = scene.add_material(Material {
        albedo_texture: Some(textura_lienzo),
        albedo: Color::new(1.0, 1.0, 1.0),
        uv_scale: 1.0,
        shadow_mode: ShadowMode::Ignore,
        ..lienzo
    });

    scene.objects.push(SceneObject {
        primitive: Primitive::Cuboid(Cuboid::new(caja)),
        initial_material: lienzo_de_la_placa,
        final_material: numero,
        spatial_group: SpatialGroupId::Monolith,
        reveal_group: RevealGroup::Finale,
    });

    Monolito {
        placa: scene.objects.len() - 1,
        tramo,
        carbon,
        numero,
        lienzo_de_la_placa,
        textura_carbon,
        textura_numero,
        textura_lienzo,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene_builder::Blockout;
    use crate::scenes::{delivery_level, delivery_level_previo_monolito, WaterPreset};

    const PRESETS: [WaterPreset; 3] = [
        WaterPreset::RefractiveWater,
        WaterPreset::OpaqueWater,
        WaterPreset::InteriorVisible,
    ];

    fn luma(c: Color) -> f32 {
        0.2126 * c.r + 0.7152 * c.g + 0.0722 * c.b
    }

    /// Lo que se reconoce de la entrega como su Monolito: la placa es la
    /// última pieza y lleva el material del número.
    fn monolito(d: &Blockout) -> Monolito {
        let s = &d.scene;
        let placa = s.objects.len() - 1;
        let o = &s.objects[placa];
        let numero = o.final_material;
        let lienzo_de_la_placa = o.initial_material;
        let tramo = tramo_del_33(s);
        let carbon = s.objects[tramo].final_material;
        Monolito {
            placa,
            tramo,
            carbon,
            numero,
            lienzo_de_la_placa,
            textura_carbon: s.material(carbon).albedo_texture.unwrap(),
            textura_numero: s.material(numero).albedo_texture.unwrap(),
            textura_lienzo: s.material(lienzo_de_la_placa).albedo_texture.unwrap(),
        }
    }

    #[test]
    fn la_entrega_suma_una_placa_tres_materiales_y_tres_texturas() {
        for (water, total) in PRESETS.into_iter().zip([219, 219, 218]) {
            let (previa, d) = (delivery_level_previo_monolito(water), delivery_level(water));
            assert_eq!(d.scene.objects.len(), total, "{water:?}");
            assert_eq!(previa.scene.objects.len() + PIEZAS, total, "{water:?}");
            assert_eq!(
                d.scene.palette.len(),
                previa.scene.palette.len() + MATERIALES
            );
            assert_eq!(
                d.scene.textures.len(),
                previa.scene.textures.len() + TEXTURAS
            );
            // Lo previo, igual: objetos salvo el material final del
            // Monolito, la paleta y las texturas.
            let tramos = indices_del_monolito(&previa.scene);
            assert_eq!(tramos.len(), 10);
            for (i, o) in previa.scene.objects.iter().enumerate() {
                let n = &d.scene.objects[i];
                assert_eq!(format!("{:?}", o.primitive), format!("{:?}", n.primitive));
                assert_eq!(o.initial_material, n.initial_material);
                assert_eq!(
                    (o.spatial_group, o.reveal_group),
                    (n.spatial_group, n.reveal_group)
                );
                if !tramos.contains(&i) {
                    assert_eq!(o.final_material, n.final_material, "{water:?} objeto {i}");
                }
            }
            for (i, m) in previa.scene.palette.iter().enumerate() {
                assert_eq!(format!("{m:?}"), format!("{:?}", d.scene.palette[i]));
            }
            for (i, t) in previa.scene.textures.iter().enumerate() {
                let n = &d.scene.textures[i];
                assert_eq!(
                    (t.width(), t.height(), t.wrap),
                    (n.width(), n.height(), n.wrap)
                );
            }
            assert_eq!(format!("{:?}", previa.anchors), format!("{:?}", d.anchors));
            assert_eq!(format!("{:?}", previa.scale), format!("{:?}", d.scale));
        }
    }

    #[test]
    fn el_monolito_es_carbon_y_la_placa_mira_a_praderas() {
        for water in PRESETS {
            let d = delivery_level(water);
            let s = &d.scene;
            let m = monolito(&d);
            for i in indices_del_monolito(s) {
                if i != m.placa {
                    assert_eq!(s.objects[i].final_material, m.carbon);
                }
            }
            let c = s.material(m.carbon);
            assert_eq!(
                (c.reflection_cap, c.transmission_cap, c.ior),
                (0.0, 0.0, 1.0)
            );
            assert_eq!(c.shadow_mode, ShadowMode::Opaque);
            assert!(c.specular_strength <= 0.2);
            let o = &s.objects[m.placa];
            assert_eq!(o.spatial_group, SpatialGroupId::Monolith);
            assert_eq!(o.reveal_group, RevealGroup::Finale);
            assert_eq!(s.material(m.numero).shadow_mode, ShadowMode::Ignore);
            assert_eq!(s.textures[m.textura_numero.0].wrap, WrapMode::Clamp);
            // Praderas, de las anclas, está hacia `-Z`; la placa se apoya en
            // la cara `-Z` del tramo central, sin salirse.
            let v = d.anchors.meadows_anchor - d.anchors.monolith_base_anchor;
            assert!(v.z < 0.0 && v.z.abs() > 100.0 * v.x.abs(), "{v:?}");
            let (b, t) = (o.primitive.bounds(), s.objects[m.tramo].primitive.bounds());
            assert!((b.max.z - t.min.z).abs() < 1e-6 && b.max.z - b.min.z <= GROSOR + 1e-6);
            assert!(b.min.x >= t.min.x && b.max.x <= t.max.x);
            assert!(b.min.y >= t.min.y && b.max.y <= t.max.y);
        }
    }

    /// El 33 se lee derecho desde Praderas: un rayo que llega a la placa por
    /// su cara `-Z` encuentra tinta donde el diseño la pone en orden de
    /// lectura —derecha de la lectura hacia `-X`—, y la tinta forma
    /// exactamente dos dígitos.
    #[test]
    fn el_33_se_lee_desde_praderas_sin_espejo() {
        use crate::ray::Ray;
        let d = delivery_level(WaterPreset::RefractiveWater);
        let s = &d.scene;
        let m = monolito(&d);
        let numero = s.material(m.numero);
        let b = s.objects[m.placa].primitive.bounds();
        let centro_x = (b.min.x + b.max.x) / 2.0;
        let y0 = (b.min.y + b.max.y) / 2.0 - ALTO_DIGITO / 2.0;
        let (n, mut acuerdo, mut tinta) = (120usize, 0, 0);
        let mut mascara = vec![false; n * n];
        for j in 0..n {
            for i in 0..n {
                // De izquierda a derecha **de quien mira desde Praderas**:
                // hacia `-X`.
                let x = b.max.x - (i as f32 + 0.5) / n as f32 * (b.max.x - b.min.x);
                let y = b.max.y - (j as f32 + 0.5) / n as f32 * (b.max.y - b.min.y);
                // Justo delante de la placa: más lejos, otras piezas de la
                // escena se interponen.
                let rayo = Ray::new(Vec3::new(x, y, b.min.z - 0.05), Vec3::new(0.0, 0.0, 1.0));
                let mut st = Default::default();
                let hit = d.accel.intersect(s, &rayo, &mut st).expect("la placa");
                assert_eq!(hit.object_index, m.placa);
                let es_tinta = luma(s.albedo_at(&numero, &hit.uv)) > 0.3;
                let s_ = (centro_x - x) / ALTO_DIGITO + ancho_del_bloque() / 2.0;
                let t_ = (y - y0) / ALTO_DIGITO;
                if es_tinta == (distancia_al_33(s_, t_) < 0.0) {
                    acuerdo += 1;
                }
                if es_tinta {
                    tinta += 1;
                    mascara[j * n + i] = true;
                }
            }
        }
        assert!(acuerdo as f32 >= 0.98 * (n * n) as f32, "{acuerdo}");
        assert!(tinta > 500);
        // Dos piezas de tinta en esa retícula.
        let mut visto = vec![false; n * n];
        let mut piezas = 0;
        for k in 0..n * n {
            if !mascara[k] || visto[k] {
                continue;
            }
            piezas += 1;
            let mut pila = vec![k];
            visto[k] = true;
            while let Some(q) = pila.pop() {
                let (x, y) = ((q % n) as i32, (q / n) as i32);
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let (a, c) = (x + dx, y + dy);
                    if a >= 0 && c >= 0 && (a as usize) < n && (c as usize) < n {
                        let r = c as usize * n + a as usize;
                        if mascara[r] && !visto[r] {
                            visto[r] = true;
                            pila.push(r);
                        }
                    }
                }
            }
        }
        assert_eq!(piezas, 2);
    }

    /// La jerarquía de la entrega lleva los once objetos del Monolito en su
    /// grupo, y la entrega es determinista.
    #[test]
    fn el_monolito_entra_en_la_jerarquia_y_es_determinista() {
        let (a, b) = (
            delivery_level(WaterPreset::RefractiveWater),
            delivery_level(WaterPreset::RefractiveWater),
        );
        let g = a
            .accel
            .groups
            .iter()
            .find(|g| g.id == SpatialGroupId::Monolith)
            .expect("grupo Monolito");
        let n: usize = g.clusters.iter().map(|c| c.object_indices.len()).sum();
        assert_eq!(n, 11);
        assert_eq!(
            format!("{:?}", a.scene.objects),
            format!("{:?}", b.scene.objects)
        );
        assert_eq!(
            format!("{:?}", a.scene.palette),
            format!("{:?}", b.scene.palette)
        );
        assert_eq!(format!("{:?}", a.accel), format!("{:?}", b.accel));
    }

    /// Huella de lo que añade el Monolito: placa, materiales nuevos y
    /// texturas nuevas texel a texel, sin assets.
    fn huella(d: &Blockout, previa: &Blockout) -> u64 {
        let mut h: u64 = 0xCBF2_9CE4_8422_2325;
        let mut mezclar = |s: String| {
            for b in s.bytes() {
                h ^= u64::from(b);
                h = h.wrapping_mul(0x0100_0000_01B3);
            }
        };
        for o in &d.scene.objects[previa.scene.objects.len()..] {
            mezclar(format!("{o:?}"));
        }
        for m in &d.scene.palette[previa.scene.palette.len()..] {
            mezclar(format!("{m:?}"));
        }
        for t in &d.scene.textures[previa.scene.textures.len()..] {
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

    /// La misma en las dos rutas: la placa y sus texturas no dependen del
    /// Rompeolas.
    const HUELLA_DEL_MONOLITO: u64 = 2_732_666_650_031_609_387;

    #[test]
    fn la_huella_del_monolito_queda_fijada() {
        let (d, p) = (
            delivery_level(WaterPreset::RefractiveWater),
            delivery_level_previo_monolito(WaterPreset::RefractiveWater),
        );
        assert_eq!(huella(&d, &p), HUELLA_DEL_MONOLITO);
    }
}
