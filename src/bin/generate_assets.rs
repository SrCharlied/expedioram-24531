//! Generador de los assets base del diorama.
//!
//! ```text
//! cargo run --release --bin generate_assets
//! ```
//!
//! Produce ocho PNG: las seis texturas de material y los dos panoramas del
//! skybox. Los archivos se versionan en el repositorio —son los que carga
//! el renderer— y este generador se conserva como su fuente reproducible.
//!
//! # Por qué generarlos y no descargarlos
//!
//! Tres razones, en orden de importancia:
//!
//! 1. **Procedencia.** Un asset generado aquí tiene autoría clara. Uno
//!    descargado arrastra una licencia que hay que rastrear y acreditar, y
//!    el plan prohíbe imágenes sin procedencia.
//! 2. **Reproducibilidad.** Con semilla fija, cualquiera regenera bytes
//!    idénticos desde un clon limpio, sin red.
//! 3. **Sin costura.** El ruido es periódico por construcción, así que las
//!    texturas repiten sin junta visible. Una imagen cualquiera de internet
//!    casi nunca tiene esa propiedad.
//!
//! # Espacio de color
//!
//! Los colores se escriben con `Color::from_srgb` —valores elegidos a ojo—,
//! se mezclan en **lineal**, y `Framebuffer` los codifica de vuelta a sRGB
//! al guardar el PNG. Mezclar en lineal es justamente el motivo del
//! pipeline de color: un degradado mezclado en sRGB sale con los medios
//! tonos apagados.

use std::path::PathBuf;
use std::process::ExitCode;

use expedition33_continente_inacabado::color::Color;
use expedition33_continente_inacabado::framebuffer::Framebuffer;

/// Lado de las texturas de material. Cuadradas y potencia de dos.
const LADO_TEXTURA: usize = 256;
/// Panorama equirectangular: el doble de ancho que de alto.
const ANCHO_SKYBOX: usize = 1024;
const ALTO_SKYBOX: usize = 512;

// ---------------------------------------------------------------- ruido

/// Hash entero a `0.0..1.0`. Determinista y sin estado.
fn hash01(x: i32, y: i32, semilla: u32) -> f32 {
    let mut h =
        semilla ^ (x as u32).wrapping_mul(0x27D4_EB2D) ^ (y as u32).wrapping_mul(0x1656_67B1);

    h ^= h >> 15;
    h = h.wrapping_mul(0x2C1B_3C6D);
    h ^= h >> 12;
    h = h.wrapping_mul(0x297A_2D39);
    h ^= h >> 15;

    (h >> 8) as f32 / (1u32 << 24) as f32
}

/// Interpolación suave: derivada nula en los extremos, así que las celdas
/// empalman sin marcar la rejilla.
fn suavizar(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// Ruido de valor **periódico** sobre una rejilla de `celdas_x × celdas_y`.
///
/// La periodicidad sale de envolver los índices de la rejilla con
/// `rem_euclid`: la celda que sigue a la última es otra vez la primera. Eso
/// es lo que hace que la textura repita sin costura, y es la propiedad que
/// el modo `Repeat` necesita para no marcar una junta cada vez que se
/// repite.
fn ruido(u: f32, v: f32, celdas_x: i32, celdas_y: i32, semilla: u32) -> f32 {
    let x = u * celdas_x as f32;
    let y = v * celdas_y as f32;

    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let sx = suavizar(x - x0 as f32);
    let sy = suavizar(y - y0 as f32);

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

/// Suma de octavas. Cada una duplica la rejilla y halva la amplitud.
///
/// Duplicar mantiene la periodicidad: si la rejilla base envuelve en
/// `celdas`, la de `2 × celdas` envuelve también en el mismo tile.
fn fbm(u: f32, v: f32, celdas: i32, octavas: u32, semilla: u32) -> f32 {
    let mut suma = 0.0;
    let mut amplitud = 1.0;
    let mut total = 0.0;

    for octava in 0..octavas {
        let factor = 1 << octava;
        suma += amplitud * ruido(u, v, celdas * factor, celdas * factor, semilla + octava);
        total += amplitud;
        amplitud *= 0.5;
    }

    suma / total
}

/// Mezcla lineal entre dos colores. Se hace en lineal a propósito.
fn mezclar(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);

    a * (1.0 - t) + b * t
}

// ------------------------------------------------------------ texturas

/// Dibuja una textura evaluando `patron(u, v)` en cada píxel.
fn pintar<F>(lado: usize, patron: F) -> Framebuffer
where
    F: Fn(f32, f32) -> Color,
{
    let mut fb = Framebuffer::new(lado, lado);

    for fila in 0..lado {
        for columna in 0..lado {
            // Centro del píxel: evita que la columna 0 y la última caigan
            // exactamente sobre la misma muestra del ruido periódico.
            let u = (columna as f32 + 0.5) / lado as f32;
            // `v = 0` abajo, como el muestreo de `Texture`.
            let v = 1.0 - (fila as f32 + 0.5) / lado as f32;

            fb.set_current_color(patron(u, v).to_hex());
            fb.point(columna, fila);
        }
    }

    fb
}

/// Lienzo sin pintar: lino marfil con trama visible.
fn canvas() -> Framebuffer {
    let claro = Color::from_srgb(0.93, 0.90, 0.83);
    let sombra = Color::from_srgb(0.82, 0.78, 0.69);

    pintar(LADO_TEXTURA, |u, v| {
        // Trama y urdimbre: dos senos perpendiculares, periódicos por
        // construcción.
        let hilos = 48.0;
        let trama = (u * hilos * std::f32::consts::TAU).sin();
        let urdimbre = (v * hilos * std::f32::consts::TAU).sin();
        let tejido = 0.5 + 0.25 * (trama + urdimbre);

        // Grano fino encima, para que no se lea como una rejilla perfecta.
        let grano = fbm(u, v, 16, 4, 0x0CA1_7A50);

        mezclar(sombra, claro, 0.55 * tejido + 0.45 * grano)
    })
}

/// Agua: azul profundo con ondulaciones suaves.
fn water() -> Framebuffer {
    let profundo = Color::from_srgb(0.09, 0.24, 0.42);
    let somero = Color::from_srgb(0.32, 0.62, 0.78);

    pintar(LADO_TEXTURA, |u, v| {
        // Dos trenes de onda cruzados, pero **deformados** por ruido antes
        // de evaluarse. Sin esa deformación el cruce de dos senos regulares
        // se lee como un tartán y no como agua: el ojo reconoce la rejilla
        // enseguida.
        let alabeo = 0.16 * (fbm(u, v, 5, 3, 0x0A90_0AA0) - 0.5);
        let onda = 0.5
            + 0.18 * (((u + alabeo) * 5.0 + (v + alabeo) * 2.0) * std::f32::consts::TAU).sin()
            + 0.18 * (((v - alabeo) * 4.0 - (u + alabeo) * 3.0) * std::f32::consts::TAU).sin();

        // La turbulencia pesa más que las ondas: manda el detalle irregular.
        let turbulencia = fbm(u, v, 8, 5, 0x0A90_A900);
        let profundidad = 0.32 * onda + 0.68 * turbulencia;

        // Realces claros en las crestas, estrechos, como reflejos rotos.
        let cresta = (profundidad - 0.62).max(0.0) * 2.6;

        mezclar(profundo, somero, profundidad + cresta * 0.35)
    })
}

/// Roca húmeda: gris oscuro con vetas y motas.
fn wet_basalt() -> Framebuffer {
    let roca = Color::from_srgb(0.17, 0.18, 0.21);
    let brillo = Color::from_srgb(0.42, 0.44, 0.48);

    pintar(LADO_TEXTURA, |u, v| {
        let masa = fbm(u, v, 6, 5, 0x0BA5_A170);
        // Motas pequeñas: una octava suelta de rejilla fina.
        let motas = ruido(u, v, 64, 64, 0x5A17_0501);
        let humedo = 0.75 * masa + 0.25 * motas.powf(3.0);

        mezclar(roca, brillo, humedo)
    })
}

/// Madera envejecida: veta vertical, tono cálido y apagado.
fn aged_wood() -> Framebuffer {
    let oscura = Color::from_srgb(0.22, 0.14, 0.09);
    let clara = Color::from_srgb(0.46, 0.32, 0.20);

    pintar(LADO_TEXTURA, |u, v| {
        // La veta corre en v; se distorsiona con ruido para que no salgan
        // franjas rectas.
        let deriva = 0.12 * fbm(u, v, 4, 3, 0x0DE0_71BA);
        let anillos = ((u + deriva) * 9.0 * std::f32::consts::TAU).sin() * 0.5 + 0.5;
        let fibra = fbm(u * 0.3, v, 12, 4, 0xF1B0_4400);

        mezclar(oscura, clara, 0.6 * anillos.powf(1.6) + 0.4 * fibra)
    })
}

/// Pradera: verde con matas y calvas.
fn meadow() -> Framebuffer {
    let tierra = Color::from_srgb(0.18, 0.28, 0.14);
    let hierba = Color::from_srgb(0.40, 0.62, 0.28);

    pintar(LADO_TEXTURA, |u, v| {
        let matas = fbm(u, v, 10, 5, 0x6BA5_5A00);
        let brotes = ruido(u, v, 48, 48, 0xB007_E500);

        mezclar(tierra, hierba, 0.7 * matas + 0.3 * brotes)
    })
}

/// Cristal pictórico: cian pálido con facetas.
fn pictorial_crystal() -> Framebuffer {
    let hondo = Color::from_srgb(0.34, 0.62, 0.70);
    let filo = Color::from_srgb(0.82, 0.96, 0.98);

    pintar(LADO_TEXTURA, |u, v| {
        // Facetas: el ruido escalonado en bandas da aristas en vez de
        // degradados, que es lo que distingue un cristal de una nube.
        let base = fbm(u, v, 5, 4, 0xC157_A100);
        let facetas = (base * 6.0).floor() / 6.0;
        let arista = ((base * 6.0).fract() - 0.5).abs() * 2.0;

        mezclar(hondo, filo, 0.65 * facetas + 0.35 * arista.powf(4.0))
    })
}

// ------------------------------------------------------------- skybox

/// Panorama equirectangular que cubre la **esfera completa**.
///
/// Convención de coordenadas, y la Tarea 4.5 tiene que muestrear con la
/// misma o el cielo saldrá desplazado:
///
/// | `v` | Dirección |
/// |---|---|
/// | `0.0` | nadir, mirando recto hacia abajo |
/// | `0.5` | horizonte |
/// | `1.0` | cenit, mirando recto hacia arriba |
///
/// Cubrir la esfera entera y no solo la mitad superior no es opcional: la
/// cámara orbita a 35 grados de elevación **mirando hacia abajo**, así que
/// hay rayos que fallan la geometría viajando por debajo del horizonte. Si
/// el panorama solo tuviera cielo, esos rayos muestrearían un tono de
/// altura equivocada.
fn panorama<F>(patron: F) -> Framebuffer
where
    F: Fn(f32, f32) -> Color,
{
    let mut fb = Framebuffer::new(ANCHO_SKYBOX, ALTO_SKYBOX);

    for fila in 0..ALTO_SKYBOX {
        for columna in 0..ANCHO_SKYBOX {
            let u = (columna as f32 + 0.5) / ANCHO_SKYBOX as f32;
            let v = 1.0 - (fila as f32 + 0.5) / ALTO_SKYBOX as f32;

            fb.set_current_color(patron(u, v).to_hex());
            fb.point(columna, fila);
        }
    }

    fb
}

/// Cielo sin pintar: el lienzo detrás del Continente.
///
/// Contraste deliberadamente bajo. Es el fondo del estado inicial, y no
/// debe competir con una escena que también arranca en marfil.
fn skybox_pale() -> Framebuffer {
    let cenit = Color::from_srgb(0.88, 0.86, 0.80);
    let horizonte = Color::from_srgb(0.96, 0.94, 0.89);
    let nadir = Color::from_srgb(0.74, 0.71, 0.65);

    panorama(|u, v| {
        let nubes = fbm(u, v, 6, 4, 0x0A1E_0001);
        // -1 en el nadir, 0 en el horizonte, +1 en el cenit.
        let altura = (v - 0.5) * 2.0;

        let base = if altura >= 0.0 {
            mezclar(horizonte, cenit, altura.powf(0.8))
        } else {
            mezclar(horizonte, nadir, (-altura).powf(0.7))
        };

        mezclar(base, cenit, 0.14 * nubes)
    })
}

/// Cielo pintado: el fondo del estado final.
fn skybox_painted() -> Framebuffer {
    let cenit = Color::from_srgb(0.07, 0.13, 0.34);
    let medio = Color::from_srgb(0.22, 0.38, 0.62);
    let horizonte = Color::from_srgb(0.72, 0.58, 0.44);

    let nadir = Color::from_srgb(0.10, 0.09, 0.14);

    panorama(|u, v| {
        let nubes = fbm(u, v, 8, 5, 0x0A17_7ED0);
        // -1 en el nadir, 0 en el horizonte, +1 en el cenit.
        let altura = (v - 0.5) * 2.0;

        let base = if altura < 0.0 {
            // Por debajo del horizonte cae rápido a un tono neutro oscuro:
            // ahí no hay cielo que describir, solo un fondo que no compita.
            mezclar(horizonte, nadir, (-altura).powf(0.55))
        } else if altura < 0.30 {
            // Franja cálida al ras del horizonte.
            mezclar(horizonte, medio, altura / 0.30)
        } else {
            mezclar(medio, cenit, (altura - 0.30) / 0.70)
        };

        // Las nubes tiñen hacia el tono medio, sin blanquear el cielo, y
        // solo por encima del horizonte.
        let peso = if altura > 0.0 { 0.22 } else { 0.05 };

        mezclar(base, medio, peso * nubes)
    })
}

// ------------------------------------------------------------- salida

struct Asset {
    ruta: &'static str,
    construir: fn() -> Framebuffer,
}

const ASSETS: &[Asset] = &[
    Asset {
        ruta: "assets/textures/canvas.png",
        construir: canvas,
    },
    Asset {
        ruta: "assets/textures/water.png",
        construir: water,
    },
    Asset {
        ruta: "assets/textures/wet_basalt.png",
        construir: wet_basalt,
    },
    Asset {
        ruta: "assets/textures/aged_wood.png",
        construir: aged_wood,
    },
    Asset {
        ruta: "assets/textures/meadow.png",
        construir: meadow,
    },
    Asset {
        ruta: "assets/textures/pictorial_crystal.png",
        construir: pictorial_crystal,
    },
    Asset {
        ruta: "assets/skybox/pale.png",
        construir: skybox_pale,
    },
    Asset {
        ruta: "assets/skybox/painted.png",
        construir: skybox_painted,
    },
];

fn main() -> ExitCode {
    let raiz = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    println!("generando {} assets en {}", ASSETS.len(), raiz.display());

    for asset in ASSETS {
        let destino = raiz.join(asset.ruta);
        let imagen = (asset.construir)();

        if let Err(e) = imagen.save_png(&destino) {
            eprintln!("error: no se pudo escribir {}: {e}", destino.display());
            return ExitCode::FAILURE;
        }

        let bytes = std::fs::metadata(&destino).map(|m| m.len()).unwrap_or(0);
        println!(
            "  {:<40} {:>4} x {:<4} {:>7} bytes",
            asset.ruta, imagen.width, imagen.height, bytes
        );
    }

    println!();
    println!("Semillas fijas: regenerar produce bytes identicos.");
    println!("Los PNG estan en sRGB; el renderer los decodifica a lineal al cargar.");

    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Comprueba que una textura repite sin costura.
    ///
    /// Es la propiedad que justifica el ruido periódico: si la columna 0 no
    /// continúa a la última, cada repetición marca una línea vertical.
    ///
    /// El criterio es comparativo, no absoluto: la costura no tiene que ser
    /// plana, tiene que **no destacar**. Una textura con contraste alto ya
    /// tiene saltos grandes entre columnas vecinas, así que la referencia
    /// correcta es el mayor salto que ya existe dentro de la propia imagen.
    /// Compararla contra un único par interior daría un resultado que
    /// depende de si ese par cayó en una zona plana o en una arista.
    fn sin_costura(fb: &Framebuffer, nombre: &str) {
        let ancho = fb.width;
        let alto = fb.height;

        let canal = |pixel: u32, desplazamiento: u32| ((pixel >> desplazamiento) & 0xFF) as i64;
        let distancia = |a: u32, b: u32| {
            [16, 8, 0]
                .iter()
                .map(|d| (canal(a, *d) - canal(b, *d)).abs())
                .sum::<i64>()
        };

        // Salto acumulado en la costura: última columna contra la primera.
        let costura: i64 = (0..alto)
            .map(|fila| distancia(fb.buffer[fila * ancho + ancho - 1], fb.buffer[fila * ancho]))
            .sum();

        // El mayor salto entre dos columnas vecinas del interior.
        let maximo_interior: i64 = (0..ancho - 1)
            .map(|columna| {
                (0..alto)
                    .map(|fila| {
                        distancia(
                            fb.buffer[fila * ancho + columna],
                            fb.buffer[fila * ancho + columna + 1],
                        )
                    })
                    .sum::<i64>()
            })
            .max()
            .unwrap_or(0);

        // Una costura real salta muchas veces por encima del máximo
        // interior; este margen la detectaría sin castigar el ruido normal.
        assert!(
            costura <= (maximo_interior * 3) / 2,
            "{nombre}: la costura salta {costura} contra un maximo interior de {maximo_interior}"
        );
    }

    #[test]
    fn las_texturas_repiten_sin_costura() {
        for asset in ASSETS.iter().take(6) {
            let fb = (asset.construir)();
            sin_costura(&fb, asset.ruta);
        }
    }

    #[test]
    fn el_panorama_repite_sin_costura_horizontal() {
        // Solo en u: el panorama da la vuelta completa, y ahi la junta se
        // veria como una linea vertical en el cielo.
        for asset in ASSETS.iter().skip(6) {
            let fb = (asset.construir)();
            sin_costura(&fb, asset.ruta);
        }
    }

    #[test]
    fn la_generacion_es_determinista() {
        for asset in ASSETS {
            let a = (asset.construir)();
            let b = (asset.construir)();

            assert_eq!(a.buffer, b.buffer, "{} no es determinista", asset.ruta);
        }
    }

    #[test]
    fn todas_las_texturas_usan_el_rango_disponible() {
        // Una textura plana no aporta nada; si el patron se aplasto, esto
        // lo detecta antes de mirar la imagen.
        for asset in ASSETS {
            let fb = (asset.construir)();

            let minimo = fb.buffer.iter().min().copied().unwrap_or(0);
            let maximo = fb.buffer.iter().max().copied().unwrap_or(0);

            assert_ne!(minimo, maximo, "{} salio plana", asset.ruta);
        }
    }

    #[test]
    fn el_ruido_periodico_envuelve_en_los_dos_ejes() {
        for semilla in [0u32, 7, 0xDEAD_BEEF] {
            for coordenada in [0.0_f32, 0.13, 0.5, 0.77] {
                let a = ruido(coordenada, 0.31, 8, 8, semilla);
                let b = ruido(coordenada + 1.0, 0.31, 8, 8, semilla);
                assert!((a - b).abs() < 1e-5, "u no envuelve: {a} contra {b}");

                let a = ruido(0.31, coordenada, 8, 8, semilla);
                let b = ruido(0.31, coordenada + 1.0, 8, 8, semilla);
                assert!((a - b).abs() < 1e-5, "v no envuelve: {a} contra {b}");
            }
        }
    }

    #[test]
    fn el_fbm_se_mantiene_en_rango_unitario() {
        for i in 0..40 {
            for j in 0..40 {
                let valor = fbm(i as f32 / 40.0, j as f32 / 40.0, 6, 5, 0x1234);

                assert!((0.0..=1.0).contains(&valor), "fbm salio de rango: {valor}");
            }
        }
    }

    #[test]
    fn los_paneles_del_skybox_son_dos_a_uno() {
        for asset in ASSETS.iter().skip(6) {
            let fb = (asset.construir)();

            assert_eq!(
                fb.width,
                fb.height * 2,
                "{} no es equirectangular",
                asset.ruta
            );
        }
    }

    #[test]
    fn hay_ocho_assets_y_ninguna_ruta_repetida() {
        assert_eq!(ASSETS.len(), 8);

        let mut rutas: Vec<&str> = ASSETS.iter().map(|a| a.ruta).collect();
        rutas.sort_unstable();
        let antes = rutas.len();
        rutas.dedup();

        assert_eq!(rutas.len(), antes, "hay rutas duplicadas");
    }

    #[test]
    fn cada_asset_se_puede_cargar_como_textura() {
        use expedition33_continente_inacabado::texture::Texture;

        let raiz = std::env::temp_dir().join(format!("continente-assets-{}", std::process::id()));

        for asset in ASSETS {
            let destino = raiz.join(asset.ruta);
            (asset.construir)().save_png(&destino).expect("escribir");

            let tex = Texture::load(&destino).expect("deberia cargar");
            assert!(tex.width() > 0 && tex.height() > 0);
        }

        let _ = std::fs::remove_dir_all(&raiz);
    }

    #[test]
    fn mezclar_en_lineal_no_oscurece_los_medios_tonos() {
        // La razon del pipeline de color: el punto medio entre negro y
        // blanco en lineal codifica al byte 188, no al 128. Mezclado en
        // sRGB daria 128 y el degradado se veria hundido.
        let negro = Color::new(0.0, 0.0, 0.0);
        let blanco = Color::new(1.0, 1.0, 1.0);

        assert_eq!(mezclar(negro, blanco, 0.5).to_hex(), 0xBCBCBC);
    }
}
