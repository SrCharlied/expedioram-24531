//! Preview de fidelidad de **Aguas Voladoras**: la entrega vigente contra
//! dos variantes que solo reescriben el **interior** de la bahía: la
//! candidata del esqueleto entero y `split_wreck`, el pecio partido.
//!
//! ```bash
//! cargo run --release --example flying_waters_fidelity_preview -- <carpeta>
//! ```
//!
//! La carpeta es **obligatoria**: el ejemplo no escribe evidencia oficial.
//! Deja quince PNG a `800 × 600`, con los assets reales y `RevealState::painted`,
//! para `current`, `candidate`, `split_wreck`, `split_wreck_thin_chain` y
//! `delivery` —la entrega de producción, `delivery_level_con`, que tras la
//! promoción tiene que salir idéntica a `split_wreck_thin_chain`—:
//!
//! - `<variante>_hero.png` — la toma hero.
//! - `<variante>_top78.png` — la misma órbita a `78°`.
//! - `<variante>_closeup.png` — un encuadre cercano
//!   **explícito** (`camara_cercana`) solo para entender el barco y la curva
//!   de la cadena. No sustituye a la hero.
//!
//! Las cinco columnas comparten las mismas cámaras y las mismas luces,
//! medidas sobre la entrega. La línea base es
//! `delivery_level_previo_aguas_con`: la entrega anterior a la promoción de
//! Aguas, con el Rompeolas, la isla y Praderas aprobados. Es la escena sobre
//! la que se aprobaron las variantes, así que los doce PNG no cambian con la
//! promoción. Solo `delivery` se construye con `delivery_level_con`.
//!
//! # Qué cambia
//!
//! Las 58 piezas de Aguas se reescriben **en su sitio** —mismo índice,
//! grupo y lienzo—; `A-01` (el volumen) y `A-11` (el borde roto) quedan
//! **intactos**, igual que todo lo ajeno a Aguas. La jerarquía se
//! reconstruye con el plan de la entrega. Reasignación explícita:
//!
//! | Entrada | Entrega | Candidata |
//! |---|---:|---|
//! | `A-01` volumen | 1 | 1, intacto |
//! | `A-02` lecho | 5 | 5: el suelo con su caja y un material de profundidad; cuatro mesetas someras más bajas a los bordes |
//! | `A-03` casco | 12 masas | 12: quilla, nueve costillas y dos largueros, con aire entre costillas (`F-09`) |
//! | `A-04` mástil | 3 | 3: el palo baja a la quilla; verga y botalón, iguales |
//! | `A-05` cadena | 8 | 8: un gesto curvo y descendente, sin eslabones (`F-11`) |
//! | `A-06` ancla | 3 | 3: la misma forma, posada al final del gesto |
//! | `A-07` kelp | 12 | 12: verticales, de alturas y grosores variados, en arboledas (`F-13`) |
//! | `A-08` rocas | 6 | 3 rocas en rastro bajo la cadena + **3 corales** (`A-12`, `F-12`) |
//! | `A-11` borde | 8 | 8, intacto |
//! | **Total** | **58** | **58** |
//!
//! `A-12` no tiene presupuesto en la entrega: sus tres corales salen de
//! tres rocas de `A-08`, no se suman.
//!
//! # Materiales
//!
//! Tres materiales nuevos, cada uno con su textura procedural en memoria:
//! el suelo del lecho, con degradado de profundidad (`F-04`); las mesetas
//! someras; y el coral, con **el mismo magenta de Praderas**
//! (`meadows_delivery::magenta`, `F-18`). El resto reusa, sin tocarlos, los
//! materiales que Aguas ya usaba: la madera y el metal del pecio, el kelp y
//! el basalto. El agua no cambia: ni techos, ni `ior`, ni modo de sombra.
//!
//! # `split_wreck`
//!
//! La lectura que dio Charlie del juego, y la que manda: el barco no está
//! entero. Abajo, el **cuerpo destruido** pegado al lecho; arriba, la
//! **proa de costillas** flotando; entre los dos, la cadena. Las 26 piezas
//! de barco de la entrega (`A-03` 12, `A-04` 3, `A-05` 8, `A-06` 3) se
//! reparten en fragmento alto 10, fragmento bajo 7, cadena 6 y ancla 3; ver
//! `roles_split`. El lecho, el kelp, `A-01` y `A-11` son los de la
//! candidata; los corales, más bajos y en racimos.
//!
//! El espacio lo fija `A-01`, que no se mueve: del lecho (`0.65`) a la
//! superficie (`2.60`) hay `1.95`. El fragmento bajo ocupa `0.65 .. 1.38`
//! y el alto `1.58 .. 2.50`, detrás y arriba, para que desde la hero se lea
//! encima. Ver `QUILLA_ALTA`.
//!
//! # `split_wreck_thin_chain`
//!
//! `split_wreck` byte a byte salvo la cadena: las mismas seis piezas, fina
//! (`0.07`) y de un carbón negro con brillo discreto, siguiendo el mismo
//! gesto entre los mismos contactos. Bloques pequeños de verdad no caben en
//! seis piezas sin dejar huecos; ver `split_cadena_fina`.
//!
//! # Qué no es
//!
//! No añade absorción, niebla, cáusticas ni animación.
//!
//! # Promoción
//!
//! Charlie aprobó `split_wreck_thin_chain` tal como está y se promovió al
//! default en `src/scenes/flying_waters_delivery.rs`: `delivery_level_con`
//! es ahora esta variante. El test
//! `la_entrega_de_produccion_es_split_wreck_thin_chain_aprobado` lo
//! comprueba byte a byte, con los assets reales. El preview se queda como
//! registro de las alternativas y de sus medidas.

fn main() -> std::process::ExitCode {
    imp::correr()
}

mod imp {
    use std::path::{Path, PathBuf};
    use std::process::ExitCode;

    use expedition33_continente_inacabado::accel::{ClusterPlan, SceneAccel, TraversalStats};
    use expedition33_continente_inacabado::bounds::Aabb;
    use expedition33_continente_inacabado::camera::Camera;
    use expedition33_continente_inacabado::color::Color;
    use expedition33_continente_inacabado::cuboid::Cuboid;
    use expedition33_continente_inacabado::framebuffer::Framebuffer;
    use expedition33_continente_inacabado::hit::Hit;
    use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
    use expedition33_continente_inacabado::material::Material;
    use expedition33_continente_inacabado::optics::refracted_ray;
    use expedition33_continente_inacabado::renderer::{render, Shading};
    use expedition33_continente_inacabado::reveal::RevealState;
    use expedition33_continente_inacabado::scene::MaterialId;
    use expedition33_continente_inacabado::scene::{Scene, SpatialGroupId};
    use expedition33_continente_inacabado::scene_builder::{
        eye_at, Blockout, HALF_VERTICAL_FOV_DEGREES, HERO_YAW_DEGREES, MAX_RADIUS_FACTOR,
        MIN_RADIUS_FACTOR,
    };
    use expedition33_continente_inacabado::scenes::meadows_delivery::magenta;
    use expedition33_continente_inacabado::scenes::{
        delivery_level_con, delivery_level_previo_aguas_con, WaterPreset,
    };
    use expedition33_continente_inacabado::texture::Texture;
    use nalgebra_glm::Vec3;

    pub const ANCHO: usize = 800;
    pub const ALTO: usize = 600;
    pub const ELEVACION_CENITAL: f32 = 78.0;
    const ROMPEOLAS_ORIGINAL: usize = 38;

    /// Qué hace cada una de las 58 piezas de Aguas, por su posición dentro
    /// del grupo. El orden es el de `flying_waters::aguas_voladoras`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Rol {
        Lecho,
        Quilla,
        Costilla,
        Larguero,
        Mastil,
        Cadena,
        Ancla,
        Kelp,
        Roca,
        Coral,
        Borde,
        Volumen,
        /// `split_wreck`: la roda del fragmento alto, en la proa.
        Roda,
        /// `split_wreck`: una pieza del fragmento bajo, el casco destruido
        /// sobre el lecho.
        CascoBajo,
    }

    /// Roles del candidato, por posición. La reasignación es explícita:
    /// `A-03` pasa de doce masas a quilla, nueve costillas y dos largueros;
    /// `A-08` cede tres rocas a corales (`A-12`, sin presupuesto propio).
    pub fn roles_candidato() -> Vec<Rol> {
        let mut r = Vec::with_capacity(58);
        r.extend([Rol::Lecho; 5]);
        r.push(Rol::Quilla);
        r.extend([Rol::Costilla; 9]);
        r.extend([Rol::Larguero; 2]);
        r.extend([Rol::Mastil; 3]);
        r.extend([Rol::Cadena; 8]);
        r.extend([Rol::Ancla; 3]);
        r.extend([Rol::Kelp; 12]);
        r.extend([Rol::Roca; 3]);
        r.extend([Rol::Coral; 3]);
        r.extend([Rol::Borde; 8]);
        r.push(Rol::Volumen);
        r
    }

    /// Roles de `split_wreck`, por posición. Las 26 piezas de barco de la
    /// entrega —`A-03` 12, `A-04` 3, `A-05` 8, `A-06` 3— se reparten así:
    ///
    /// | Hueco | Entrada | Rol |
    /// |---|---|---|
    /// | 5 | `A-03` | quilla del fragmento alto |
    /// | 6–11 | `A-03` | seis costillas del fragmento alto |
    /// | 12–13 | `A-03` | dos largueros del fragmento alto |
    /// | 14–16 | `A-03` | tres piezas del fragmento bajo |
    /// | 17 | `A-04` | roda del fragmento alto |
    /// | 18–19 | `A-04` | dos piezas del fragmento bajo |
    /// | 20–25 | `A-05` | seis tramos de cadena |
    /// | 26–27 | `A-05` | dos piezas del fragmento bajo |
    /// | 28–30 | `A-06` | el ancla |
    ///
    /// Fragmento alto: 10. Fragmento bajo: 7. Cadena: 6. Ancla: 3. Lo demás
    /// de Aguas, como la candidata.
    pub fn roles_split() -> Vec<Rol> {
        let mut r = Vec::with_capacity(58);
        r.extend([Rol::Lecho; 5]);
        r.push(Rol::Quilla);
        r.extend([Rol::Costilla; 6]);
        r.extend([Rol::Larguero; 2]);
        r.extend([Rol::CascoBajo; 3]);
        r.push(Rol::Roda);
        r.extend([Rol::CascoBajo; 2]);
        r.extend([Rol::Cadena; 6]);
        r.extend([Rol::CascoBajo; 2]);
        r.extend([Rol::Ancla; 3]);
        r.extend([Rol::Kelp; 12]);
        r.extend([Rol::Roca; 3]);
        r.extend([Rol::Coral; 3]);
        r.extend([Rol::Borde; 8]);
        r.push(Rol::Volumen);
        r
    }

    /// Roles de la entrega actual, por posición: el casco son doce masas
    /// (cinco secciones, tres tramos de cubierta, tres costillas y la popa),
    /// que aquí se cuentan como `Costilla` solo para agrupar el barco.
    pub fn roles_actuales() -> Vec<Rol> {
        let mut r = Vec::with_capacity(58);
        r.extend([Rol::Lecho; 5]);
        r.extend([Rol::Costilla; 12]);
        r.extend([Rol::Mastil; 3]);
        r.extend([Rol::Cadena; 8]);
        r.extend([Rol::Ancla; 3]);
        r.extend([Rol::Kelp; 12]);
        r.extend([Rol::Roca; 6]);
        r.extend([Rol::Borde; 8]);
        r.push(Rol::Volumen);
        r
    }

    impl Rol {
        pub fn es_barco(self) -> bool {
            matches!(
                self,
                Rol::Quilla | Rol::Costilla | Rol::Larguero | Rol::Mastil
            )
        }
        /// `split_wreck`: el fragmento delantero, suspendido.
        pub fn es_alto(self) -> bool {
            matches!(
                self,
                Rol::Quilla | Rol::Costilla | Rol::Larguero | Rol::Roda
            )
        }

        #[cfg(test)]
        pub fn es_metal(self) -> bool {
            matches!(self, Rol::Cadena | Rol::Ancla)
        }
    }

    pub struct Candidato {
        pub diorama: Blockout,
        pub paleta_previa: usize,
        pub texturas_previas: usize,
    }

    /// La entrega anterior a la promoción de Aguas: la línea base sobre la
    /// que se aprobaron las variantes.
    pub fn actual(raiz: &Path) -> Blockout {
        actual_con(WaterPreset::RefractiveWater, raiz)
    }

    pub fn actual_con(water: WaterPreset, raiz: &Path) -> Blockout {
        delivery_level_previo_aguas_con(water, Some(raiz))
            .expect("los assets reales tienen que estar en la raiz del proyecto")
    }

    /// La entrega de producción, con la variante promovida.
    pub fn produccion_con(water: WaterPreset, raiz: &Path) -> Blockout {
        delivery_level_con(water, Some(raiz))
            .expect("los assets reales tienen que estar en la raiz del proyecto")
    }

    pub fn indices_de_aguas(scene: &Scene) -> Vec<usize> {
        scene
            .objects
            .iter()
            .enumerate()
            .filter(|(_, o)| o.spatial_group == SpatialGroupId::FlyingWaters)
            .map(|(i, _)| i)
            .collect()
    }

    // -----------------------------------------------------------------
    // Texturas procedurales, en memoria
    // -----------------------------------------------------------------

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

    fn ruido(u: f32, v: f32, celdas_x: i32, celdas_y: i32, semilla: u32) -> f32 {
        let suavizar = |t: f32| t * t * (3.0 - 2.0 * t);
        let (x, y) = (u * celdas_x as f32, v * celdas_y as f32);
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

    fn fbm(u: f32, v: f32, celdas: i32, octavas: u32, semilla: u32) -> f32 {
        let (mut suma, mut amplitud, mut total) = (0.0, 1.0, 0.0);
        for octava in 0..octavas {
            let c = celdas << octava;
            suma += amplitud * ruido(u, v, c, c, semilla.wrapping_add(octava));
            total += amplitud;
            amplitud *= 0.5;
        }
        suma / total
    }

    fn mezclar(a: Color, b: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        a * (1.0 - t) + b * t
    }

    /// Evalúa `patron(u, v)` en el centro de cada texel, con `v = 0` abajo.
    fn textura<F: Fn(f32, f32) -> Color>(ancho: usize, alto: usize, patron: F) -> Texture {
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

    /// `F-04` · el suelo del lecho: hondo y azul en el centro de la bahía,
    /// somero y claro hacia los bordes.
    ///
    /// La cara superior de un cuboide recorre `u` en `x` y `v` en `z`, de
    /// borde a borde. Con escala UV `1` la textura cubre el suelo una sola
    /// vez, así que el degradado cae exactamente sobre la planta de la
    /// bahía sin una pieza más.
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

    /// Las mesetas someras de los bordes: arena gris verdosa con piedra.
    fn textura_somera() -> Texture {
        let arena = Color::from_srgb(0.44, 0.49, 0.44);
        let piedra = Color::from_srgb(0.27, 0.32, 0.31);

        textura(64, 64, |u, v| {
            mezclar(arena, piedra, (fbm(u, v, 5, 3, 0x50E7_0001) - 0.35) * 2.2)
        })
    }

    /// `F-12` · coral: el magenta **de Praderas**, el mismo valor
    /// (`meadows_delivery::magenta`), con un rosa y una sombra.
    fn textura_coral() -> Texture {
        let rosa = Color::from_srgb(0.80, 0.46, 0.66);
        let sombra = Color::from_srgb(0.36, 0.12, 0.30);

        textura(32, 32, |u, v| {
            let n = fbm(u, v, 2, 3, 0xC0A1_0001);
            if n > 0.47 {
                magenta()
            } else if n > 0.42 {
                rosa
            } else {
                sombra
            }
        })
    }

    /// `split_wreck` · coral en agrupaciones: racimos magenta irregulares
    /// sobre una base oscura, con huecos, en vez de una placa uniforme.
    fn textura_coral_agrupada() -> Texture {
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

    /// `split_wreck_thin_chain` · metal negro: carbón con motas algo más
    /// claras. Ni negro absoluto, que bajo el agua sería un agujero, ni
    /// claro: el brillo discreto del material es lo que lo separa del lecho.
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

    // -----------------------------------------------------------------
    // Composición
    // -----------------------------------------------------------------

    /// Una caja relativa a la bahía `(ancla.x, 0, ancla.z)`:
    /// `[x0, y0, z0, x1, y1, z1]`.
    type Caja = [f32; 6];

    /// Cuánto se hunde una pieza posada en su apoyo.
    const EMPOTRADO: f32 = 0.04;
    /// Techo del suelo principal del lecho, que no se mueve.
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

    // --- A-02 · lecho: el suelo queda; cuatro mesetas bajas a los bordes

    /// Cuatro mesetas someras, bajas, pegadas a las paredes: dejan el centro
    /// hondo bajo el barco. Muerden el suelo `0.10`.
    const MESETAS: [Caja; 4] = [
        [-4.15, 0.55, -2.40, -1.60, 0.84, -1.00],
        [3.40, 0.55, -2.40, 4.20, 0.80, 0.20],
        [-1.40, 0.55, -2.42, 1.00, 0.74, -1.85],
        [-4.15, 0.55, 0.60, -2.30, 0.78, 1.70],
    ];

    // --- A-03 · casco: quilla, nueve costillas y dos largueros (`F-09`)

    /// La quilla, del codaste a la proa.
    const QUILLA: Caja = [-2.46, 1.74, 0.14, 1.62, 1.86, 0.26];

    /// Techo de cada costilla, de popa a proa: el arrufo del casco. La
    /// primera es el codaste, más alto y la única pieza del casco que rompe
    /// la superficie, como la popa de la entrega; hacia proa la borda baja.
    const TECHOS: [f32; 9] = [2.76, 2.46, 2.47, 2.46, 2.44, 2.40, 2.34, 2.26, 2.16];

    /// Costilla `i`: `x` de su plano, de popa a proa, su manga —que se
    /// cierra hacia proa— y su techo.
    fn costilla(i: usize) -> Caja {
        let x = -2.33 + 0.475 * i as f32;
        let manga = 1.00 - 0.075 * i as f32;
        [
            x - 0.05,
            1.80,
            0.20 - manga * 0.5,
            x + 0.05,
            TECHOS[i],
            0.20 + manga * 0.5,
        ]
    }

    /// Los dos largueros de la borda, rotos a distinta altura: el de la
    /// banda que mira a la cámara se acaba antes.
    const LARGUEROS: [Caja; 2] = [
        [-2.38, 2.30, 0.56, -0.80, 2.40, 0.64],
        [-2.38, 2.30, -0.14, 0.10, 2.40, -0.06],
    ];

    // --- A-04 · mástil: el palo baja hasta la quilla

    const PALO: Caja = [-0.68, 1.84, 0.12, -0.52, 4.65, 0.28];

    // --- A-05 · cadena: un gesto, no eslabones (`F-11`)

    /// Grosor de cada tramo. Algo más que los `0.22` de la entrega: el
    /// recorrido es más largo y cada tramo cubre más trecho.
    const GROSOR_DE_CADENA: f32 = 0.30;

    /// Centros de los ocho tramos: sale de la proa hacia fuera, gira y baja
    /// hacia el fondo de la bahía hasta tenderse en el lecho junto al ancla.
    /// Cada paso es menor que el grosor en los tres ejes, así que los tramos
    /// se tocan y la cadena se lee continua.
    const CADENA: [[f32; 3]; 8] = [
        [1.72, 1.80, 0.18],
        [2.01, 1.59, 0.08],
        [2.30, 1.38, -0.09],
        [2.56, 1.18, -0.33],
        [2.78, 1.00, -0.62],
        [2.92, 0.86, -0.91],
        [2.99, 0.76, -1.20],
        [2.99, 0.70, -1.49],
    ];

    // --- A-06 · ancla: la misma forma, al final del gesto

    const ANCLA: [Caja; 3] = [
        [2.89, 0.60, -1.83, 3.11, 1.30, -1.61],
        [2.64, 0.54, -1.83, 3.36, 0.76, -1.61],
        [2.83, 1.15, -1.82, 3.17, 1.35, -1.62],
    ];

    // --- A-07 · kelp: vertical, alturas y grosores variados, en arboledas

    /// `(x, z, grosor, alto, apoyo)`.
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

    // --- A-08 · tres rocas en rastro bajo la cadena, y tres corales

    /// `(x, z, ancho, alto, fondo)`: siguen la curva de la cadena en el
    /// lecho, del pie del barco hacia el ancla.
    const RASTRO: [(f32, f32, f32, f32, f32); 3] = [
        (2.40, 0.25, 0.30, 0.20, 0.26),
        (2.85, -0.35, 0.24, 0.16, 0.22),
        (3.20, -0.95, 0.34, 0.22, 0.28),
    ];

    /// `(x, z, ancho, alto, fondo, apoyo)`: `A-12` sin presupuesto propio,
    /// pagado con tres rocas de `A-08`. Sobre las mesetas del fondo, donde
    /// el agua deja verlos.
    const CORALES: [(f32, f32, f32, f32, f32, f32); 3] = [
        (-3.00, -1.70, 0.55, 0.22, 0.40, 0.84),
        (3.80, -1.10, 0.42, 0.26, 0.50, 0.80),
        (0.30, -2.15, 0.50, 0.20, 0.36, 0.74),
    ];

    /// Las 58 piezas en el orden de los índices de Aguas, con el material
    /// que les toca. `None` deja la pieza como está en la entrega.
    fn piezas(m: &Materiales) -> Vec<Option<(Caja, MaterialId)>> {
        let mut p: Vec<Option<(Caja, MaterialId)>> = Vec::with_capacity(58);

        // A-02: el suelo conserva su caja; cambia el material.
        p.push(Some(([f32::NAN; 6], m.lecho)));
        p.extend(MESETAS.iter().map(|&c| Some((c, m.somera))));
        // A-03.
        p.push(Some((QUILLA, m.madera)));
        p.extend((0..9).map(|i| Some((costilla(i), m.madera))));
        p.extend(LARGUEROS.iter().map(|&c| Some((c, m.madera))));
        // A-04: el palo baja; la verga y el botalón quedan.
        p.push(Some((PALO, m.mastil)));
        p.extend([None, None]);
        // A-05.
        let g = GROSOR_DE_CADENA * 0.5;
        p.extend(
            CADENA
                .iter()
                .map(|&[x, y, z]| Some(([x - g, y - g, z - g, x + g, y + g, z + g], m.cadena))),
        );
        // A-06.
        p.extend(ANCLA.iter().map(|&c| Some((c, m.ancla))));
        // A-07.
        p.extend(
            KELP.iter()
                .map(|&(x, z, w, h, apoyo)| Some((posada(x, z, w, h, w, apoyo), m.kelp))),
        );
        // A-08 → rastro + corales.
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

    // -----------------------------------------------------------------
    // split_wreck
    // -----------------------------------------------------------------
    //
    // El espacio vertical lo fija `A-01`, que no se mueve: del techo del
    // lecho (`0.65`) a la superficie (`2.60`) hay `1.95`. El fragmento bajo
    // ocupa `0.65 .. 1.38`; el alto, `1.58 .. 2.50`. Entre los dos quedan
    // `0.20` en vertical y `0.50` en `z`: el alto va detrás y arriba, y
    // desde la hero —que mira hacia abajo— se lee encima del bajo. La
    // cadena cuelga entre los dos.

    /// Fragmento alto: la quilla de la proa, suspendida **detrás** del
    /// fragmento bajo y más arriba, para que desde la hero se lea encima.
    const QUILLA_ALTA: Caja = [-0.70, 1.58, -1.51, 1.55, 1.70, -1.39];

    /// Eje del fragmento alto en `z`.
    const EJE_ALTO: f32 = -1.45;

    /// Costillas del fragmento alto, de la rotura a la proa:
    /// `(x, grosor, manga, techo, desplazamiento en z)`. Paso, techo y manga
    /// irregulares; la quinta está partida y solo conserva una banda.
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

    /// Largueros de la borda: el de la banda que mira a la cámara llega más
    /// lejos que el otro, roto antes.
    const LARGUEROS_ALTOS: [Caja; 2] = [
        [-0.60, 2.22, -1.07, 0.72, 2.32, -0.99],
        [-0.60, 2.24, -1.91, 0.30, 2.32, -1.83],
    ];

    /// La roda: la única pieza del pecio que rompe la superficie.
    const RODA: Caja = [1.48, 1.58, -1.51, 1.60, 2.74, -1.39];

    /// Fragmento bajo: el cuerpo destruido, empotrado en el lecho **delante**
    /// del alto. Siete piezas, con más masa que el esqueleto de arriba:
    const CASCO_BAJO: [Caja; 7] = [
        // Fondo enterrado, muerde el lecho `0.10`.
        [-1.05, 0.55, -0.20, 1.10, 0.80, 0.85],
        // Banda del fondo, la alta.
        [-1.00, 0.60, -0.35, 0.85, 1.25, -0.15],
        // Banda que mira a la cámara, rota baja: deja ver dentro.
        [-0.90, 0.60, 0.75, 0.30, 0.98, 0.95],
        // Espejo de popa.
        [-1.15, 0.60, -0.30, -0.95, 1.32, 0.90],
        // Cuaderna en pie.
        [-0.15, 0.75, -0.25, -0.05, 1.38, 0.85],
        // Cuaderna partida junto a la rotura.
        [0.60, 0.75, -0.25, 0.70, 1.15, 0.40],
        // Tablón caído sobre la banda alta.
        [-0.80, 1.20, -0.35, 0.20, 1.30, 0.10],
    ];

    /// La cadena: seis tramos que cuelgan de la quilla del fragmento alto,
    /// caen deprisa y se tienden hacia delante hasta el fondo del bajo, por
    /// la rotura, junto al final de su banda alta.
    const GROSOR_DE_CADENA_SPLIT: f32 = 0.26;
    const CADENA_SPLIT: [[f32; 3]; 6] = [
        [1.05, 1.64, -1.40],
        [1.04, 1.38, -1.18],
        [1.03, 1.18, -0.94],
        [1.02, 1.05, -0.70],
        [1.01, 0.97, -0.45],
        [1.00, 0.92, -0.20],
    ];

    /// El ancla, posada junto al canto roto del fragmento bajo.
    const ANCLA_SPLIT: [Caja; 3] = [
        [1.35, 0.60, 0.20, 1.57, 1.30, 0.42],
        [1.10, 0.54, 0.20, 1.82, 0.76, 0.42],
        [1.29, 1.15, 0.21, 1.63, 1.35, 0.41],
    ];

    /// Rastro de restos en el lecho bajo el fragmento alto, caídos de la
    /// proa. Bajos: no ocupan el vacío de debajo.
    const RASTRO_SPLIT: [(f32, f32, f32, f32, f32); 3] = [
        (0.20, -1.15, 0.30, 0.18, 0.26),
        (0.85, -1.70, 0.24, 0.15, 0.22),
        (1.45, -1.20, 0.34, 0.20, 0.28),
    ];

    /// Corales más bajos y anchos que en la candidata, en los mismos sitios.
    const CORALES_SPLIT: [(f32, f32, f32, f32, f32, f32); 3] = [
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

    /// Las 58 piezas de `split_wreck`, hueco a hueco; ver `roles_split`.
    fn piezas_split(m: &Materiales) -> Vec<Option<(Caja, MaterialId)>> {
        let base = piezas(m);
        let mut p: Vec<Option<(Caja, MaterialId)>> = Vec::with_capacity(58);
        let madera = m.madera;

        // A-02: el lecho de la candidata.
        p.extend(base[..5].iter().copied());
        // A-03 (12): quilla, seis costillas y dos largueros arriba; tres
        // piezas del casco bajo.
        p.push(Some((QUILLA_ALTA, madera)));
        p.extend(
            COSTILLAS_ALTAS
                .iter()
                .map(|c| Some((costilla_alta(c), madera))),
        );
        p.extend(LARGUEROS_ALTOS.iter().map(|&c| Some((c, madera))));
        p.extend(CASCO_BAJO[..3].iter().map(|&c| Some((c, madera))));
        // A-04 (3): la roda arriba; dos piezas del casco bajo.
        p.push(Some((RODA, madera)));
        p.extend(CASCO_BAJO[3..5].iter().map(|&c| Some((c, madera))));
        // A-05 (8): seis tramos de cadena; dos piezas del casco bajo.
        let g = GROSOR_DE_CADENA_SPLIT * 0.5;
        p.extend(
            CADENA_SPLIT
                .iter()
                .map(|&[x, y, z]| Some(([x - g, y - g, z - g, x + g, y + g, z + g], m.cadena))),
        );
        p.extend(CASCO_BAJO[5..].iter().map(|&c| Some((c, madera))));
        // A-06 (3): el ancla.
        p.extend(ANCLA_SPLIT.iter().map(|&c| Some((c, m.ancla))));
        // A-07: el kelp de la candidata.
        p.extend(base[31..43].iter().copied());
        // A-08: rastro y corales.
        p.extend(
            RASTRO_SPLIT
                .iter()
                .map(|&(x, z, w, h, f)| Some((posada(x, z, w, h, f, SUELO), m.roca))),
        );
        p.extend(
            CORALES_SPLIT
                .iter()
                .map(|&(x, z, w, h, f, apoyo)| Some((posada(x, z, w, h, f, apoyo), m.coral))),
        );
        // A-11 y A-01: intactos.
        p.extend(base[49..].iter().copied());

        debug_assert_eq!(p.len(), 58);
        p
    }

    /// Materiales de la candidata: los de Aguas que ya existían, reusados
    /// sin tocar, y tres nuevos con textura en memoria.
    #[derive(Debug, Clone, Copy)]
    struct Materiales {
        lecho: MaterialId,
        somera: MaterialId,
        coral: MaterialId,
        madera: MaterialId,
        mastil: MaterialId,
        cadena: MaterialId,
        ancla: MaterialId,
        kelp: MaterialId,
        roca: MaterialId,
    }

    fn con_textura(scene: &mut Scene, base: Material, tex: Texture) -> MaterialId {
        let id = scene.add_texture(tex);
        scene.add_material(base.with_texture(id).with_uv_scale(1.0))
    }

    /// La candidata: la entrega con las piezas interiores de Aguas
    /// reescritas en su sitio. `A-01` y `A-11` no se tocan.
    pub fn candidato(actual: Blockout) -> Candidato {
        construir(actual, piezas, textura_coral)
    }

    /// `split_wreck`: el pecio partido en dos fragmentos, por la lectura
    /// que dio Charlie del juego —abajo el cuerpo destruido pegado al
    /// lecho, arriba la proa de costillas suspendida, la cadena entre los
    /// dos—. Mismo lecho, mismo kelp, mismos `A-01` y `A-11`.
    pub fn split(actual: Blockout) -> Candidato {
        construir(actual, piezas_split, textura_coral_agrupada)
    }

    /// `split_wreck_thin_chain`: `split_wreck` exacto salvo la cadena, que
    /// pasa a ser fina y negra. Mismas seis piezas, mismo gesto, mismos
    /// extremos; un material nuevo con su textura.
    ///
    /// # Por qué seis segmentos y no bloques pequeños
    ///
    /// Dos piezas solo se tocan si su paso en cada eje no supera su grosor.
    /// El gesto recorre `1.32` en `z` y cae `0.78`: con cubos de `0.10` no
    /// bastan menos de dieciséis piezas, y eslabones de `0.20 × 0.08`
    /// alternados piden al menos diez. La cadena tiene seis y el presupuesto
    /// no se amplía sin autorización, así que cada pieza cubre **un tramo**
    /// del trazado: la caja que va de un punto al siguiente, con solo `0.07`
    /// de grueso en `x`, el eje que la hero y el encuadre cercano ven de
    /// frente. Desde un lateral se vería como una cinta escalonada.
    pub fn split_cadena_fina(actual: Blockout) -> Candidato {
        let mut c = split(actual);
        let scene = &mut c.diorama.scene;
        let aguas = indices_de_aguas(scene);
        let negro = {
            let tex = scene.add_texture(textura_metal_negro());
            scene.add_material(
                Material::new(Color::new(1.0, 1.0, 1.0))
                    .with_texture(tex)
                    .with_uv_scale(1.0)
                    .with_specular(0.40, 40.0),
            )
        };
        let ancla = c.diorama.anchors.flying_waters_anchor;
        let base = Vec3::new(ancla.x, 0.0, ancla.z);
        let g = GROSOR_DE_CADENA_FINA * 0.5;

        for (k, par) in TRAZADO_FINO.windows(2).enumerate() {
            let (a, b) = (par[0], par[1]);
            let minimo = Vec3::new(a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2]));
            let maximo = Vec3::new(a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2]));
            let objeto = &mut c.diorama.scene.objects[aguas[PRIMER_HUECO_DE_CADENA + k]];
            objeto.primitive = Cuboid::new(Aabb::new(
                base + minimo - Vec3::new(g, g, g),
                base + maximo + Vec3::new(g, g, g),
            ))
            .into();
            objeto.final_material = negro;
        }

        c.diorama.accel = reacelerar(&c.diorama.scene);
        c
    }

    /// Primer hueco de cadena dentro de Aguas en `roles_split`.
    const PRIMER_HUECO_DE_CADENA: usize = 20;

    /// Grosor de la cadena fina.
    const GROSOR_DE_CADENA_FINA: f32 = 0.07;

    /// Siete puntos del gesto: de dentro de la quilla del fragmento alto al
    /// fondo del casco bajo. Siguen la curva de la cadena de `split_wreck`
    /// —cae deprisa y se tiende hacia delante— y el último baja lo justo para
    /// que el último tramo muerda el fondo.
    const TRAZADO_FINO: [[f32; 3]; 7] = [
        [1.05, 1.62, -1.42],
        [1.04, 1.40, -1.20],
        [1.03, 1.20, -0.96],
        [1.02, 1.06, -0.72],
        [1.01, 0.97, -0.48],
        [1.005, 0.91, -0.26],
        [1.00, 0.83, -0.10],
    ];

    type Tabla = fn(&Materiales) -> Vec<Option<(Caja, MaterialId)>>;

    fn construir(actual: Blockout, tabla: Tabla, coral_tex: fn() -> Texture) -> Candidato {
        let mut diorama = actual;
        let paleta_previa = diorama.scene.palette.len();
        let texturas_previas = diorama.scene.textures.len();
        let aguas = indices_de_aguas(&diorama.scene);
        assert_eq!(aguas.len(), 58, "Aguas llega con su inventario");

        let usado = |k: usize| diorama.scene.objects[aguas[k]].final_material;
        let (madera, mastil, cadena, ancla, kelp, roca) = (
            usado(5),
            usado(17),
            usado(20),
            usado(28),
            usado(31),
            usado(43),
        );

        let blanco = Material::new(Color::new(1.0, 1.0, 1.0));
        let lecho = con_textura(
            &mut diorama.scene,
            blanco.with_specular(0.20, 32.0),
            textura_lecho(),
        );
        let somera = con_textura(
            &mut diorama.scene,
            blanco.with_specular(0.15, 24.0),
            textura_somera(),
        );
        let coral = con_textura(
            &mut diorama.scene,
            blanco.with_specular(0.10, 16.0),
            coral_tex(),
        );
        let m = Materiales {
            lecho,
            somera,
            coral,
            madera,
            mastil,
            cadena,
            ancla,
            kelp,
            roca,
        };

        let ancla_bahia = diorama.anchors.flying_waters_anchor;
        let base = Vec3::new(ancla_bahia.x, 0.0, ancla_bahia.z);

        for (k, pieza) in tabla(&m).into_iter().enumerate() {
            let Some((c, material)) = pieza else { continue };
            let objeto = &mut diorama.scene.objects[aguas[k]];
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

        diorama.accel = reacelerar(&diorama.scene);

        Candidato {
            diorama,
            paleta_previa,
            texturas_previas,
        }
    }

    pub fn reacelerar(scene: &Scene) -> SceneAccel {
        let mut plan = ClusterPlan::new();
        let rompeolas = scene
            .objects
            .iter()
            .enumerate()
            .filter(|(_, o)| o.spatial_group == SpatialGroupId::Breakwater)
            .map(|(i, _)| i);

        for (k, i) in rompeolas.enumerate() {
            plan.asignar(i, u16::from(k >= ROMPEOLAS_ORIGINAL));
        }

        SceneAccel::build_from_plan(scene, &plan).expect("la escena tiene geometria")
    }

    pub fn camara_cenital(diorama: &Blockout) -> Camera {
        let ojo = eye_at(
            diorama.anchors.orbit_center,
            diorama.scale.orbit_radius,
            HERO_YAW_DEGREES,
            ELEVACION_CENITAL,
        );

        Camera::new(
            ojo,
            diorama.anchors.orbit_center,
            diorama.anchors.look_at,
            Vec3::new(0.0, 1.0, 0.0),
            (2.0 * HALF_VERTICAL_FOV_DEGREES).to_radians(),
        )
        .with_radius_limits(
            diorama.scale.scene_radius * MIN_RADIUS_FACTOR,
            diorama.scale.scene_radius * MAX_RADIUS_FACTOR,
        )
    }

    /// Lo que ve un píxel de Aguas: el primer impacto y, si es la
    /// superficie del agua, el del primer rayo refractado real.
    #[derive(Debug, Clone, Copy)]
    pub struct Vista {
        pub primero: Hit,
        pub detras: Option<Hit>,
    }

    /// Recorre la toma con la jerarquía real. Para cada píxel cuyo primer
    /// impacto sea el volumen `A-01`, repite el primer nivel de refracción
    /// del renderer —`optics::refracted_ray` con el `ior` del material— y
    /// guarda lo que encuentra detrás.
    pub fn mirar(
        diorama: &Blockout,
        camara: &Camera,
        ancho: usize,
        alto: usize,
    ) -> Vec<Option<Vista>> {
        let aguas = indices_de_aguas(&diorama.scene);
        let volumen = *aguas.last().expect("A-01 va al final");
        let ior = diorama
            .scene
            .material(diorama.scene.objects[volumen].final_material)
            .ior;
        let mut stats = TraversalStats::default();
        let mut vistas = Vec::with_capacity(ancho * alto);

        for y in 0..alto {
            for x in 0..ancho {
                let rayo = camara.ray_from_pixel(x, y, ancho, alto);
                let Some(primero) = diorama.accel.intersect(&diorama.scene, &rayo, &mut stats)
                else {
                    vistas.push(None);
                    continue;
                };
                let detras = (primero.object_index == volumen)
                    .then(|| refracted_ray(&primero, &rayo.direction, ior))
                    .flatten()
                    .and_then(|r| diorama.accel.intersect(&diorama.scene, &r, &mut stats));
                vistas.push(Some(Vista { primero, detras }));
            }
        }

        vistas
    }

    /// Píxeles por rol de Aguas: `(directos, a través del agua)`.
    pub fn lectura_por_rol(
        diorama: &Blockout,
        roles: &[Rol],
        camara: &Camera,
    ) -> Vec<(Rol, usize, usize)> {
        let aguas = indices_de_aguas(&diorama.scene);
        let rol_de = |i: usize| aguas.iter().position(|&a| a == i).map(|k| roles[k]);
        let mut cuenta: Vec<(Rol, usize, usize)> = Vec::new();
        let mut sumar = |rol: Rol, directo: bool| {
            let fila = match cuenta.iter().position(|(r, _, _)| *r == rol) {
                Some(p) => p,
                None => {
                    cuenta.push((rol, 0, 0));
                    cuenta.len() - 1
                }
            };
            if directo {
                cuenta[fila].1 += 1;
            } else {
                cuenta[fila].2 += 1;
            }
        };

        for vista in mirar(diorama, camara, ANCHO, ALTO).into_iter().flatten() {
            if let Some(rol) = rol_de(vista.primero.object_index) {
                if rol != Rol::Volumen {
                    sumar(rol, true);
                } else if let Some(rol) = vista.detras.and_then(|h| rol_de(h.object_index)) {
                    sumar(rol, false);
                }
            }
        }

        cuenta
    }

    pub fn union(cajas: impl Iterator<Item = Aabb>) -> Option<Aabb> {
        cajas.reduce(|a, b| a.union(&b))
    }

    /// Fracción de los rayos refractados reales de una toma que, cruzando la
    /// caja del casco, la atraviesan sin tocar madera: cuánto aire deja ver
    /// el casco a través del agua.
    pub fn huecos_del_casco(diorama: &Blockout, roles: &[Rol], camara: &Camera) -> f32 {
        huecos_de(diorama, roles, camara, |r| r.es_barco() && r != Rol::Mastil)
    }

    /// Lo mismo que `huecos_del_casco`, para las piezas que elija `filtro`.
    pub fn huecos_de(
        diorama: &Blockout,
        roles: &[Rol],
        camara: &Camera,
        filtro: fn(Rol) -> bool,
    ) -> f32 {
        let aguas = indices_de_aguas(&diorama.scene);
        let casco: Vec<usize> = aguas
            .iter()
            .zip(roles)
            .filter(|(_, r)| filtro(**r))
            .map(|(&i, _)| i)
            .collect();
        let caja_casco = union(
            casco
                .iter()
                .map(|&i| diorama.scene.objects[i].primitive.bounds()),
        )
        .expect("hay casco");
        let volumen = *aguas.last().expect("volumen");
        let ior = diorama
            .scene
            .material(diorama.scene.objects[volumen].final_material)
            .ior;
        let (mut cruzan, mut pasan) = (0usize, 0usize);
        let mut stats = TraversalStats::default();

        for p in 0..ANCHO * ALTO {
            let rayo = camara.ray_from_pixel(p % ANCHO, p / ANCHO, ANCHO, ALTO);
            let Some(h) = diorama.accel.intersect(&diorama.scene, &rayo, &mut stats) else {
                continue;
            };
            if h.object_index != volumen {
                continue;
            }
            let Some(dentro) = refracted_ray(&h, &rayo.direction, ior) else {
                continue;
            };
            if caja_casco.hit(&dentro, 1e-4, f32::INFINITY).is_none() {
                continue;
            }
            cruzan += 1;
            let toca = diorama
                .accel
                .intersect(&diorama.scene, &dentro, &mut stats)
                .is_some_and(|x| casco.contains(&x.object_index));
            pasan += usize::from(!toca);
        }

        pasan as f32 / cruzan.max(1) as f32
    }

    /// Ancho de la cadena en pantalla —mediana por fila de sus píxeles,
    /// directos o a través del agua— y su luma media contra la de los
    /// píxeles a tres de distancia en la misma fila: `(ancho, cadena,
    /// vecinos)`.
    pub fn lectura_de_cadena(
        diorama: &Blockout,
        roles: &[Rol],
        camara: &Camera,
        cuadro: &Framebuffer,
    ) -> (usize, f32, f32) {
        let aguas = indices_de_aguas(&diorama.scene);
        let cadena: Vec<usize> = aguas
            .iter()
            .zip(roles)
            .filter(|(_, r)| **r == Rol::Cadena)
            .map(|(&i, _)| i)
            .collect();
        let vistas = mirar(diorama, camara, ANCHO, ALTO);
        let es = |p: usize| {
            vistas[p].is_some_and(|v| {
                cadena.contains(&v.primero.object_index)
                    || (aguas.last() == Some(&v.primero.object_index)
                        && v.detras.is_some_and(|h| cadena.contains(&h.object_index)))
            })
        };
        let luma = |c: u32| {
            0.2126 * ((c >> 16) & 0xFF) as f32
                + 0.7152 * ((c >> 8) & 0xFF) as f32
                + 0.0722 * (c & 0xFF) as f32
        };
        let mut por_fila = vec![0usize; ALTO];
        let (mut suya, mut n, mut vecina, mut m) = (0.0, 0usize, 0.0, 0usize);
        for p in 0..ANCHO * ALTO {
            if !es(p) {
                continue;
            }
            por_fila[p / ANCHO] += 1;
            suya += luma(cuadro.buffer[p]);
            n += 1;
            for q in [p.saturating_sub(3), p + 3] {
                if q < ANCHO * ALTO && !es(q) {
                    vecina += luma(cuadro.buffer[q]);
                    m += 1;
                }
            }
        }
        let mut filas: Vec<usize> = por_fila.into_iter().filter(|&k| k > 0).collect();
        filas.sort_unstable();
        (
            filas.get(filas.len() / 2).copied().unwrap_or(0),
            suya / n.max(1) as f32,
            vecina / m.max(1) as f32,
        )
    }

    /// Encuadre cercano **explícito**, solo para entender barco y curva:
    /// desde delante y arriba de la bahía, por encima del borde roto. No
    /// sustituye a la hero.
    pub fn camara_cercana() -> Camera {
        let mira = Vec3::new(6.5, 1.3, 5.6);
        Camera::new(
            Vec3::new(6.8, 9.5, 15.0),
            mira,
            mira,
            Vec3::new(0.0, 1.0, 0.0),
            40f32.to_radians(),
        )
    }

    pub fn correr() -> ExitCode {
        let Some(destino) = std::env::args().nth(1).map(PathBuf::from) else {
            eprintln!(
                "uso: cargo run --release --example flying_waters_fidelity_preview -- <carpeta>"
            );
            eprintln!("  la carpeta es obligatoria: este ejemplo no escribe evidencia oficial.");
            return ExitCode::FAILURE;
        };
        if let Err(e) = std::fs::create_dir_all(&destino) {
            eprintln!("error: no se pudo crear {}: {e}", destino.display());
            return ExitCode::FAILURE;
        }

        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let vigente = actual(&raiz);
        let propuesto = candidato(actual(&raiz));
        let partido = split(actual(&raiz));
        let fina = split_cadena_fina(actual(&raiz));
        let entregada = produccion_con(WaterPreset::RefractiveWater, &raiz);
        let hero = vigente.hero_camera();
        let cenital = camara_cenital(&vigente);
        let cerca = camara_cercana();
        let luces = luces_del_diorama(&vigente.anchors, &vigente.scale);
        let columnas: [(&str, &Blockout, Vec<Rol>); 5] = [
            ("current", &vigente, roles_actuales()),
            ("candidate", &propuesto.diorama, roles_candidato()),
            ("split_wreck", &partido.diorama, roles_split()),
            ("split_wreck_thin_chain", &fina.diorama, roles_split()),
            ("delivery", &entregada, roles_split()),
        ];

        println!("flying_waters_fidelity_preview\n");
        println!("  destino   {}", destino.display());
        println!(
            "  primitivas entrega {} · candidata {} · Aguas {}",
            vigente.scene.objects.len(),
            propuesto.diorama.scene.objects.len(),
            indices_de_aguas(&propuesto.diorama.scene).len()
        );
        println!(
            "  materiales nuevos {} ({} compartidos intactos) · texturas nuevas {}\n",
            propuesto.diorama.scene.palette.len() - propuesto.paleta_previa,
            propuesto.paleta_previa,
            propuesto.diorama.scene.textures.len() - propuesto.texturas_previas
        );
        let mut ok = true;
        for (toma, camara) in [("hero", &hero), ("top78", &cenital), ("closeup", &cerca)] {
            for (nombre, diorama, roles) in &columnas {
                let mut fb = Framebuffer::new(ANCHO, ALTO);
                let stats = render(
                    &mut fb,
                    &diorama.scene,
                    &diorama.accel,
                    &luces,
                    &RevealState::painted(),
                    camara,
                    Shading::Material,
                );
                let ruta = destino.join(format!("{nombre}_{toma}.png"));
                if let Err(e) = fb.save_png(&ruta) {
                    eprintln!("error: no se pudo escribir {}: {e}", ruta.display());
                    ok = false;
                }
                let partida = nombre.starts_with("split_wreck") || *nombre == "delivery";
                let huecos = if partida {
                    huecos_de(diorama, roles, camara, Rol::es_alto)
                } else {
                    huecos_del_casco(diorama, roles, camara)
                };
                println!(
                    "  {toma:<8} {nombre:<11} rayos reflejados {} refractados {} · huecos del casco {huecos:.2}",
                    stats.reflection_rays,
                    stats.refraction_rays,
                );
                println!(
                    "           px (directos, a traves del agua) {:?}",
                    lectura_por_rol(diorama, roles, camara)
                );
                if partida {
                    let (ancho, cadena, vecinos) = lectura_de_cadena(diorama, roles, camara, &fb);
                    println!(
                        "           cadena: ancho mediano {ancho} px por fila, luma {cadena:.1} contra {vecinos:.1} de su entorno"
                    );
                }
            }
        }

        if ok {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::imp::*;
    use expedition33_continente_inacabado::bounds::Aabb;
    use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
    use expedition33_continente_inacabado::scene::{Scene, SpatialGroupId};
    use expedition33_continente_inacabado::scene_builder::{measure_scene_radius, Blockout};
    use expedition33_continente_inacabado::scenes::{WaterPreset, DELIVERY};
    use std::path::PathBuf;

    const EPS: f32 = 1.0e-4;
    /// Cuánto puede hundirse una pieza en lo que la sostiene.
    const EMPOTRADO_MAXIMO: f32 = 0.15;

    fn raiz() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn entrega() -> Blockout {
        actual(&raiz())
    }

    fn nuevo() -> Candidato {
        candidato(entrega())
    }

    fn caja(scene: &Scene, i: usize) -> Aabb {
        scene.objects[i].primitive.bounds()
    }

    fn tamano(c: &Aabb) -> [f32; 3] {
        [c.max.x - c.min.x, c.max.y - c.min.y, c.max.z - c.min.z]
    }

    fn tocan(a: &Aabb, b: &Aabb) -> bool {
        a.min.x <= b.max.x + EPS
            && b.min.x <= a.max.x + EPS
            && a.min.y <= b.max.y + EPS
            && b.min.y <= a.max.y + EPS
            && a.min.z <= b.max.z + EPS
            && b.min.z <= a.max.z + EPS
    }

    /// Cajas de Aguas del candidato, en orden, con su rol.
    fn piezas(c: &Candidato) -> Vec<(Rol, Aabb)> {
        let scene = &c.diorama.scene;
        indices_de_aguas(scene)
            .into_iter()
            .zip(roles_candidato())
            .map(|(i, r)| (r, caja(scene, i)))
            .collect()
    }

    fn del_rol(p: &[(Rol, Aabb)], rol: Rol) -> Vec<Aabb> {
        p.iter()
            .filter(|(r, _)| *r == rol)
            .map(|(_, c)| *c)
            .collect()
    }

    fn volumen(p: &[(Rol, Aabb)]) -> Aabb {
        del_rol(p, Rol::Volumen)[0]
    }

    fn texeles(t: &expedition33_continente_inacabado::texture::Texture) -> Vec<String> {
        let (w, h) = (t.width(), t.height());
        let mut v = Vec::with_capacity(w * h);
        for y in 0..h {
            for x in 0..w {
                let u = (x as f32 + 0.5) / w as f32;
                let vv = (y as f32 + 0.5) / h as f32;
                v.push(format!("{:?}", t.sample(u, vv)));
            }
        }
        v
    }

    // -------------------------------------------------------------- conteo

    #[test]
    fn cuenta_58_en_aguas_y_168_en_total() {
        let c = nuevo();
        let scene = &c.diorama.scene;
        let cuenta = |g| {
            scene
                .objects
                .iter()
                .filter(|o| o.spatial_group == g)
                .count()
        };

        assert_eq!(cuenta(SpatialGroupId::FlyingWaters), 58);
        assert_eq!(cuenta(SpatialGroupId::FlyingWaters), DELIVERY.flying_waters);
        assert_eq!(cuenta(SpatialGroupId::Meadows), DELIVERY.meadows);
        assert_eq!(cuenta(SpatialGroupId::Breakwater), DELIVERY.breakwater);
        assert_eq!(scene.objects.len(), 168);
        assert_eq!(roles_candidato().len(), 58);
        assert_eq!(roles_actuales().len(), 58);
    }

    #[test]
    fn la_reasignacion_es_explicita_entrada_por_entrada() {
        let roles = roles_candidato();
        let n = |r: Rol| roles.iter().filter(|x| **x == r).count();

        // A-01 1, A-02 5, A-03 12 = 1 + 9 + 2, A-04 3, A-05 8, A-06 3,
        // A-07 12, A-08 6 = 3 rocas + 3 corales, A-11 8.
        assert_eq!(n(Rol::Volumen), 1);
        assert_eq!(n(Rol::Lecho), 5);
        assert_eq!(n(Rol::Quilla) + n(Rol::Costilla) + n(Rol::Larguero), 12);
        assert_eq!(n(Rol::Mastil), 3);
        assert_eq!(n(Rol::Cadena), 8);
        assert_eq!(n(Rol::Ancla), 3);
        assert_eq!(n(Rol::Kelp), 12);
        assert_eq!(n(Rol::Roca) + n(Rol::Coral), 6);
        assert_eq!(n(Rol::Borde), 8);
    }

    // ---------------------------------------------------------- invariancia

    #[test]
    fn fuera_de_aguas_nada_cambia_byte_a_byte() {
        let a = entrega();
        let c = nuevo();
        let b = &c.diorama;

        assert_eq!(a.scene.objects.len(), b.scene.objects.len());
        for (i, (x, y)) in a.scene.objects.iter().zip(&b.scene.objects).enumerate() {
            assert_eq!(x.spatial_group, y.spatial_group, "objeto {i}");
            assert_eq!(x.reveal_group, y.reveal_group, "objeto {i}");
            if x.spatial_group == SpatialGroupId::FlyingWaters {
                assert_eq!(
                    x.initial_material, y.initial_material,
                    "{i} no nace en lienzo"
                );
                continue;
            }
            assert_eq!(
                format!("{x:?}"),
                format!("{y:?}"),
                "objeto {i} ajeno a Aguas cambio"
            );
        }

        assert_eq!(c.paleta_previa, a.scene.palette.len());
        assert_eq!(c.texturas_previas, a.scene.textures.len());
        for (i, (x, y)) in a.scene.palette.iter().zip(&b.scene.palette).enumerate() {
            assert_eq!(format!("{x:?}"), format!("{y:?}"), "material {i} mutado");
        }
        for (i, (x, y)) in a.scene.textures.iter().zip(&b.scene.textures).enumerate() {
            assert!(texeles(x) == texeles(y), "textura {i} mutada");
        }
        assert_eq!(
            format!("{:?}", a.scene.skybox),
            format!("{:?}", b.scene.skybox)
        );
        assert_eq!(format!("{:?}", a.anchors), format!("{:?}", b.anchors));
        assert_eq!(format!("{:?}", a.scale), format!("{:?}", b.scale));
        assert_eq!(a.hero_preset(), b.hero_preset());
        assert_eq!(
            format!("{:?}", camara_cenital(&a)),
            format!("{:?}", camara_cenital(b))
        );
        assert_eq!(
            format!("{:?}", luces_del_diorama(&a.anchors, &a.scale)),
            format!("{:?}", luces_del_diorama(&b.anchors, &b.scale))
        );
        assert_eq!(
            format!("{:?}", a.accel.bounds),
            format!("{:?}", b.accel.bounds)
        );
        assert_eq!(a.accel.groups.len(), b.accel.groups.len());
        for (x, y) in a.accel.groups.iter().zip(&b.accel.groups) {
            assert_eq!(x.id, y.id);
            if x.id != SpatialGroupId::FlyingWaters {
                assert_eq!(format!("{x:?}"), format!("{y:?}"), "grupo {:?}", x.id);
            } else {
                assert_eq!(x.clusters.len(), y.clusters.len(), "clusters de Aguas");
            }
        }
    }

    #[test]
    fn el_volumen_a01_y_el_borde_a11_quedan_integros() {
        let a = entrega();
        let c = nuevo();
        let aguas_a = indices_de_aguas(&a.scene);
        let aguas_b = indices_de_aguas(&c.diorama.scene);
        let roles = roles_candidato();

        for (k, (&i, &j)) in aguas_a.iter().zip(&aguas_b).enumerate() {
            if matches!(roles[k], Rol::Borde | Rol::Volumen) {
                assert_eq!(
                    format!("{:?}", a.scene.objects[i]),
                    format!("{:?}", c.diorama.scene.objects[j]),
                    "{:?} {k} cambio",
                    roles[k]
                );
            }
        }
    }

    #[test]
    fn el_candidato_es_determinista() {
        let a = nuevo();
        let b = nuevo();
        assert_eq!(
            format!("{:?}", a.diorama.scene.objects),
            format!("{:?}", b.diorama.scene.objects)
        );
        assert_eq!(
            format!("{:?}", a.diorama.scene.palette),
            format!("{:?}", b.diorama.scene.palette)
        );
        assert_eq!(
            a.diorama.scene.textures.len(),
            b.diorama.scene.textures.len()
        );
        for (x, y) in a.diorama.scene.textures[a.texturas_previas..]
            .iter()
            .zip(&b.diorama.scene.textures[b.texturas_previas..])
        {
            assert!(texeles(x) == texeles(y));
        }
    }

    #[test]
    fn los_materiales_nuevos_son_locales_de_aguas() {
        let a = entrega();
        let c = nuevo();
        let scene = &c.diorama.scene;
        let de_aguas_antes: Vec<usize> = indices_de_aguas(&a.scene)
            .into_iter()
            .map(|i| a.scene.objects[i].final_material.0)
            .collect();

        for i in indices_de_aguas(scene) {
            let m = scene.objects[i].final_material.0;
            assert!(
                m >= c.paleta_previa || de_aguas_antes.contains(&m),
                "la pieza {i} usa un material que Aguas no usaba"
            );
        }
        for (i, o) in scene.objects.iter().enumerate() {
            if o.spatial_group != SpatialGroupId::FlyingWaters {
                assert!(
                    o.final_material.0 < c.paleta_previa,
                    "{i} usa un material nuevo"
                );
            }
        }
        for (k, m) in scene.palette[c.paleta_previa..].iter().enumerate() {
            assert!(m.is_valid(), "material nuevo {k} fuera de rango");
            let t = m.albedo_texture.expect("textura procedural en memoria");
            assert!(
                t.0 >= c.texturas_previas,
                "material nuevo {k} con textura compartida"
            );
        }
    }

    #[test]
    fn la_jerarquia_reconstruida_coincide_con_el_oraculo() {
        let c = nuevo();
        let d = &c.diorama;
        let (w, h) = (200, 150);
        for camara in [d.hero_camera(), camara_cenital(d)] {
            for p in 0..w * h {
                let rayo = camara.ray_from_pixel(p % w, p / w, w, h);
                let mut stats = Default::default();
                assert_eq!(
                    d.accel
                        .intersect(&d.scene, &rayo, &mut stats)
                        .map(|x| x.object_index),
                    d.scene.intersect(&rayo).map(|x| x.object_index),
                    "pixel {p}"
                );
            }
        }
    }

    // --------------------------------------------------------- territorio

    #[test]
    fn aguas_no_sale_de_su_territorio_y_el_interior_cabe_en_el_volumen() {
        let a = entrega();
        let c = nuevo();
        let territorio = union(
            indices_de_aguas(&a.scene)
                .into_iter()
                .map(|i| caja(&a.scene, i)),
        )
        .expect("Aguas tiene piezas");
        let p = piezas(&c);
        let v = volumen(&p);

        for (k, (rol, b)) in p.iter().enumerate() {
            assert!(
                b.min.x >= territorio.min.x - EPS
                    && b.min.y >= territorio.min.y - EPS
                    && b.min.z >= territorio.min.z - EPS
                    && b.max.x <= territorio.max.x + EPS
                    && b.max.y <= territorio.max.y + EPS
                    && b.max.z <= territorio.max.z + EPS,
                "{rol:?} {k} sale del territorio: {b:?}"
            );
            if matches!(rol, Rol::Borde | Rol::Volumen) || k == 0 {
                continue;
            }
            // El interior, en planta, dentro del volumen; en altura, bajo la
            // superficie salvo el mástil y la popa, que la atraviesan.
            assert!(
                b.min.x > v.min.x && b.max.x < v.max.x && b.min.z > v.min.z && b.max.z < v.max.z,
                "{rol:?} {k} sale del volumen en planta: {b:?}"
            );
        }
        assert_eq!(
            measure_scene_radius(&c.diorama.scene, c.diorama.anchors.orbit_center),
            a.scale.scene_radius
        );
    }

    #[test]
    fn solo_el_mastil_y_la_popa_rompen_la_superficie() {
        let c = nuevo();
        let p = piezas(&c);
        let superficie = volumen(&p).max.y;

        let asoman: Vec<(usize, Rol)> = p
            .iter()
            .enumerate()
            .filter(|(_, (r, b))| !matches!(r, Rol::Borde | Rol::Volumen) && b.max.y > superficie)
            .map(|(k, (r, _))| (k, *r))
            .collect();
        let mastil = asoman.iter().filter(|(_, r)| *r == Rol::Mastil).count();
        let casco = asoman
            .iter()
            .filter(|(_, r)| r.es_barco() && *r != Rol::Mastil)
            .count();

        assert!(mastil >= 1, "el mastil no atraviesa la superficie");
        assert!(casco <= 1, "asoma mas de una pieza del casco: {asoman:?}");
        assert!(
            asoman.iter().all(|(_, r)| r.es_barco()),
            "asoma algo que no es barco: {asoman:?}"
        );
    }

    // --------------------------------------------------------------- apoyo

    /// Cada pieza del interior llega al suelo del lecho por contactos. El
    /// barco se sostiene por la cadena y el ancla: está suspendido, no
    /// varado, y la cadena es lo que lo une al fondo.
    #[test]
    fn todo_el_interior_llega_al_lecho_sin_huecos() {
        let c = nuevo();
        let p = piezas(&c);
        let interior: Vec<usize> = (0..p.len())
            .filter(|&k| !matches!(p[k].0, Rol::Borde | Rol::Volumen))
            .collect();
        let mut conectada = vec![false; p.len()];
        conectada[0] = true;

        loop {
            let mut cambio = false;
            for &k in &interior {
                if !conectada[k]
                    && interior
                        .iter()
                        .any(|&s| conectada[s] && tocan(&p[k].1, &p[s].1))
                {
                    conectada[k] = true;
                    cambio = true;
                }
            }
            if !cambio {
                break;
            }
        }

        let sueltas: Vec<(usize, Rol)> = interior
            .iter()
            .filter(|&&k| !conectada[k])
            .map(|&k| (k, p[k].0))
            .collect();
        assert!(
            sueltas.is_empty(),
            "piezas sin apoyo hasta el lecho: {sueltas:?}"
        );
    }

    /// Lo que se posa en el fondo —rocas, corales, kelp, ancla— lo hace de
    /// verdad: su cara inferior muerde el techo de una pieza de lecho o roca.
    #[test]
    fn lo_posado_muerde_su_apoyo() {
        let c = nuevo();
        let p = piezas(&c);
        let apoyos: Vec<Aabb> = p
            .iter()
            .filter(|(r, _)| matches!(r, Rol::Lecho | Rol::Roca))
            .map(|(_, b)| *b)
            .collect();

        for (k, (rol, b)) in p.iter().enumerate() {
            if !matches!(rol, Rol::Roca | Rol::Coral | Rol::Kelp) {
                continue;
            }
            let posada = apoyos.iter().any(|s| {
                s != b
                    && (-EPS..=EMPOTRADO_MAXIMO).contains(&(s.max.y - b.min.y))
                    && b.min.x < s.max.x
                    && b.max.x > s.min.x
                    && b.min.z < s.max.z
                    && b.max.z > s.min.z
            });
            assert!(posada, "{rol:?} {k} no se posa en el lecho: {b:?}");
        }
    }

    // ---------------------------------------------------------------- barco

    #[test]
    fn el_casco_es_un_esqueleto_de_costillas_sobre_quilla() {
        let c = nuevo();
        let p = piezas(&c);
        let quilla = del_rol(&p, Rol::Quilla);
        let costillas = del_rol(&p, Rol::Costilla);
        let largueros = del_rol(&p, Rol::Larguero);

        assert_eq!(quilla.len(), 1);
        assert_eq!(costillas.len(), 9);
        assert_eq!(largueros.len(), 2);

        let casco =
            union(quilla.iter().chain(&costillas).chain(&largueros).copied()).expect("hay casco");
        let largo = casco.max.x - casco.min.x;
        let q = quilla[0];
        let [qx, qy, qz] = tamano(&q);
        assert!(
            qx >= 0.8 * largo && qy <= 0.2 && qz <= 0.2,
            "la quilla no es una quilla: {q:?}"
        );

        let mut xs: Vec<f32> = Vec::new();
        for r in &costillas {
            let [x, y, z] = tamano(r);
            assert!(
                x <= 0.15 && y >= 3.0 * x && z >= 0.4,
                "costilla no delgada: {r:?}"
            );
            assert!(tocan(r, &q), "una costilla no nace de la quilla: {r:?}");
            xs.push((r.min.x + r.max.x) * 0.5);
        }
        xs.sort_by(f32::total_cmp);
        let grosor = costillas
            .iter()
            .map(|r| r.max.x - r.min.x)
            .fold(0.0, f32::max);
        for par in xs.windows(2) {
            assert!(
                par[1] - par[0] >= 3.0 * grosor,
                "costillas sin aire entre ellas: {xs:?}"
            );
        }
        for l in &largueros {
            let [x, y, z] = tamano(l);
            assert!(
                x >= 1.5 && y <= 0.15 && z <= 0.15,
                "larguero no es un larguero: {l:?}"
            );
            let atadas = costillas.iter().filter(|r| tocan(r, l)).count();
            assert!(atadas >= 3, "un larguero toca solo {atadas} costillas");
        }

        // Huecos deliberados: la madera ocupa poco de la caja del casco.
        let volumen_de = |b: &Aabb| {
            let [x, y, z] = tamano(b);
            x * y * z
        };
        let madera: f32 = quilla
            .iter()
            .chain(&costillas)
            .chain(&largueros)
            .map(volumen_de)
            .sum();
        assert!(
            madera <= 0.25 * volumen_de(&casco),
            "el casco sigue siendo masa: {:.0} % de su caja",
            100.0 * madera / volumen_de(&casco)
        );
    }

    #[test]
    fn el_casco_actual_es_masa_vista_desde_arriba() {
        // Control: la medición distingue el casco macizo de la entrega.
        let a = entrega();
        let huecos = huecos_del_casco(&a, &roles_actuales(), &camara_cenital(&a));
        assert!(
            huecos < 0.25,
            "la entrega ya dejaba ver a traves del casco: {huecos:.2}"
        );
    }

    #[test]
    fn a_traves_del_agua_el_casco_deja_ver_entre_costillas() {
        let c = nuevo();
        let d = &c.diorama;
        let huecos = huecos_del_casco(d, &roles_candidato(), &camara_cenital(d));
        assert!(
            huecos >= 0.35,
            "desde arriba el casco sigue tapando: {huecos:.2}"
        );
    }

    #[test]
    fn el_mastil_se_apoya_en_la_quilla() {
        let c = nuevo();
        let p = piezas(&c);
        let quilla = del_rol(&p, Rol::Quilla)[0];
        let mastiles = del_rol(&p, Rol::Mastil);
        let palo = mastiles
            .iter()
            .max_by(|a, b| tamano(a)[1].total_cmp(&tamano(b)[1]))
            .expect("hay mastil");
        assert!(
            tocan(palo, &quilla),
            "el mastil cuelga sin llegar a la quilla: {palo:?}"
        );
    }

    // ---------------------------------------------------- cadena y ancla

    #[test]
    fn la_cadena_es_un_gesto_curvo_continuo_del_barco_al_ancla() {
        let c = nuevo();
        let p = piezas(&c);
        let cadena = del_rol(&p, Rol::Cadena);
        let ancla = del_rol(&p, Rol::Ancla);
        let barco: Vec<Aabb> = p
            .iter()
            .filter(|(r, _)| r.es_barco())
            .map(|(_, b)| *b)
            .collect();
        let centro = |b: &Aabb| (b.min + b.max) * 0.5;

        assert_eq!(cadena.len(), 8);
        assert!(
            barco.iter().any(|b| tocan(b, &cadena[0])),
            "la cadena no sale del barco"
        );
        assert!(
            ancla.iter().any(|b| tocan(b, &cadena[7])),
            "la cadena no llega al ancla"
        );
        for par in cadena.windows(2) {
            assert!(tocan(&par[0], &par[1]), "la cadena se corta: {par:?}");
            assert!(
                centro(&par[1]).y < centro(&par[0]).y,
                "la cadena no desciende"
            );
        }

        // Recorrido: más largo que la cadena actual, y curvo en planta.
        let largo: f32 = cadena
            .windows(2)
            .map(|w| (centro(&w[1]) - centro(&w[0])).magnitude())
            .sum();
        assert!(largo >= 2.5, "la cadena recorre solo {largo:.2}");
        let (a, b) = (centro(&cadena[0]), centro(&cadena[7]));
        let eje = Vec2::new(b.x - a.x, b.z - a.z).normalize();
        let desvio = cadena
            .iter()
            .map(|s| {
                let q = centro(s);
                let d = Vec2::new(q.x - a.x, q.z - a.z);
                (d.x * eje.y - d.y * eje.x).abs()
            })
            .fold(0.0_f32, f32::max);
        assert!(desvio >= 0.25, "la cadena va recta en planta: {desvio:.2}");
    }

    use nalgebra_glm::Vec2;

    #[test]
    fn el_ancla_reposa_en_el_lecho() {
        let c = nuevo();
        let p = piezas(&c);
        let lecho = del_rol(&p, Rol::Lecho);
        let ancla = del_rol(&p, Rol::Ancla);
        let pie = ancla.iter().map(|b| b.min.y).fold(f32::MAX, f32::min);
        let apoyada = lecho.iter().any(|s| {
            ancla.iter().any(|b| tocan(b, s))
                && (-EPS..=EMPOTRADO_MAXIMO).contains(&(s.max.y - pie))
        });
        assert!(apoyada, "el ancla no reposa en el lecho");
    }

    /// Píxeles de un grupo de roles en la hero, directos más a través del
    /// agua, con la refracción real.
    fn pixeles(d: &Blockout, roles: &[Rol], filtro: fn(Rol) -> bool) -> usize {
        lectura_por_rol(d, roles, &d.hero_camera())
            .into_iter()
            .filter(|(r, _, _)| filtro(*r))
            .map(|(_, a, b)| a + b)
            .sum()
    }

    #[test]
    fn barco_cadena_y_ancla_se_leen_en_la_hero_a_traves_del_agua() {
        let a = entrega();
        let c = nuevo();
        let metal_antes = pixeles(&a, &roles_actuales(), Rol::es_metal);
        let metal = pixeles(&c.diorama, &roles_candidato(), Rol::es_metal);
        let barco = pixeles(&c.diorama, &roles_candidato(), Rol::es_barco);

        assert!(
            metal >= 2 * metal_antes.max(1),
            "cadena y ancla: {metal} contra {metal_antes}"
        );
        assert!(metal >= 150, "cadena y ancla apenas se ven: {metal} px");
        assert!(barco >= 600, "el barco apenas se ve: {barco} px");
    }

    // ------------------------------------------------------ lecho y kelp

    #[test]
    fn el_kelp_es_vertical_y_variado() {
        let c = nuevo();
        let p = piezas(&c);
        let kelp = del_rol(&p, Rol::Kelp);
        let superficie = volumen(&p).max.y;
        let barco_y_metal: Vec<Aabb> = p
            .iter()
            .filter(|(r, _)| r.es_barco() || r.es_metal())
            .map(|(_, b)| *b)
            .collect();

        assert_eq!(kelp.len(), 12);
        let (mut altos, mut gruesos) = (Vec::new(), Vec::new());
        for k in &kelp {
            let [x, y, z] = tamano(k);
            assert!(y >= 4.0 * x.max(z), "fronda no vertical: {k:?}");
            assert!(k.max.y < superficie - 0.1, "fronda que asoma: {k:?}");
            assert!(
                !barco_y_metal.iter().any(|b| tocan(b, k)),
                "fronda que atraviesa el pecio: {k:?}"
            );
            altos.push(y);
            gruesos.push(x.max(z));
        }
        let razon = |v: &[f32]| {
            v.iter().cloned().fold(f32::MIN, f32::max) / v.iter().cloned().fold(f32::MAX, f32::min)
        };
        assert!(razon(&altos) >= 2.0, "alturas parejas: {altos:?}");
        assert!(razon(&gruesos) >= 1.6, "grosores parejos: {gruesos:?}");
    }

    #[test]
    fn el_lecho_es_bajo_y_el_suelo_no_se_mueve() {
        let a = entrega();
        let c = nuevo();
        let p = piezas(&c);
        let suelo_a = caja(&a.scene, indices_de_aguas(&a.scene)[0]);
        assert_eq!(
            p[0].1, suelo_a,
            "el suelo principal del lecho cambio de caja"
        );
        for (k, b) in del_rol(&p, Rol::Lecho).iter().enumerate().skip(1) {
            assert!(
                b.max.y <= suelo_a.max.y + 0.22,
                "meseta {k} no es baja: {b:?}"
            );
        }
    }

    #[test]
    fn hay_tres_corales_magenta_de_praderas_bajos() {
        use expedition33_continente_inacabado::scenes::meadows_delivery::magenta;

        let c = nuevo();
        let scene = &c.diorama.scene;
        let aguas = indices_de_aguas(scene);
        let roles = roles_candidato();
        let corales: Vec<usize> = aguas
            .iter()
            .zip(&roles)
            .filter(|(_, r)| **r == Rol::Coral)
            .map(|(&i, _)| i)
            .collect();

        assert_eq!(corales.len(), 3);
        for &i in &corales {
            let b = caja(scene, i);
            assert!(tamano(&b)[1] <= 0.35, "coral alto: {b:?}");
            let m = scene.material(scene.objects[i].final_material);
            let t = scene.texture(m.albedo_texture.expect("textura del coral"));
            let objetivo = magenta();
            let tiene = (0..t.height()).any(|y| {
                (0..t.width()).any(|x| {
                    let s = t.sample(
                        (x as f32 + 0.5) / t.width() as f32,
                        (y as f32 + 0.5) / t.height() as f32,
                    );
                    s == objetivo
                })
            });
            assert!(tiene, "el coral {i} no usa el magenta de Praderas");
        }
    }

    // ---------------------------------------------- huella de la candidata

    /// FNV-1a de 64 bits sobre las piezas de Aguas, los materiales nuevos y
    /// sus texturas texel a texel.
    fn huella_de(c: &Candidato) -> u64 {
        let scene = &c.diorama.scene;
        let mut textos: Vec<String> = indices_de_aguas(scene)
            .into_iter()
            .map(|i| format!("{:?}", scene.objects[i]))
            .collect();
        textos.push(format!("{:?}", &scene.palette[c.paleta_previa..]));
        for t in &scene.textures[c.texturas_previas..] {
            textos.extend(texeles(t));
        }
        let mut h: u64 = 0xCBF2_9CE4_8422_2325;
        for t in textos {
            for b in t.bytes() {
                h ^= u64::from(b);
                h = h.wrapping_mul(0x0100_0000_01B3);
            }
        }
        h
    }

    /// La candidata tal y como se entregó. `split_wreck` no puede moverla.
    const HUELLA_CANDIDATA: u64 = 6_237_831_568_856_971_198;

    #[test]
    fn la_candidata_anterior_no_cambia() {
        assert_eq!(huella_de(&nuevo()), HUELLA_CANDIDATA, "la candidata cambio");
    }

    // ------------------------------------------------------- split_wreck

    /// Cajas de Aguas de una variante con sus roles.
    fn piezas_con(c: &Candidato, roles: &[Rol]) -> Vec<(Rol, Aabb)> {
        let scene = &c.diorama.scene;
        indices_de_aguas(scene)
            .into_iter()
            .zip(roles.iter().copied())
            .map(|(i, r)| (r, caja(scene, i)))
            .collect()
    }

    fn partido() -> Candidato {
        split(entrega())
    }

    fn de(p: &[(Rol, Aabb)], filtro: impl Fn(Rol) -> bool) -> Vec<Aabb> {
        p.iter()
            .filter(|(r, _)| filtro(*r))
            .map(|(_, b)| *b)
            .collect()
    }

    fn madera(cajas: &[Aabb]) -> f32 {
        cajas
            .iter()
            .map(|b| {
                let [x, y, z] = tamano(b);
                x * y * z
            })
            .sum()
    }

    /// Lo que una caja ocupa de su caja envolvente.
    fn relleno(cajas: &[Aabb]) -> f32 {
        let caja = union(cajas.iter().copied()).expect("hay piezas");
        let [x, y, z] = tamano(&caja);
        madera(cajas) / (x * y * z)
    }

    #[test]
    fn split_cuenta_58_y_168_con_la_reasignacion_explicita() {
        let c = partido();
        let scene = &c.diorama.scene;
        let roles = roles_split();
        let n = |r: Rol| roles.iter().filter(|x| **x == r).count();

        assert_eq!(indices_de_aguas(scene).len(), 58);
        assert_eq!(scene.objects.len(), 168);
        assert_eq!(roles.len(), 58);
        // A-03 + A-04 + A-05 + A-06 = 26 = alto 10 + bajo 7 + cadena 6 + ancla 3.
        assert_eq!(
            n(Rol::Quilla) + n(Rol::Costilla) + n(Rol::Larguero) + n(Rol::Roda),
            10
        );
        assert_eq!(n(Rol::CascoBajo), 7);
        assert_eq!(n(Rol::Cadena), 6);
        assert_eq!(n(Rol::Ancla), 3);
        assert_eq!(n(Rol::Mastil), 0);
        // Lo demás de Aguas no cede piezas.
        assert_eq!(n(Rol::Lecho), 5);
        assert_eq!(n(Rol::Kelp), 12);
        assert_eq!(n(Rol::Roca) + n(Rol::Coral), 6);
        assert_eq!(n(Rol::Borde), 8);
        assert_eq!(n(Rol::Volumen), 1);
    }

    #[test]
    fn split_no_toca_nada_fuera_de_aguas_ni_a01_a11() {
        let a = entrega();
        let c = partido();
        let b = &c.diorama;
        let roles = roles_split();
        let aguas_a = indices_de_aguas(&a.scene);

        for (i, (x, y)) in a.scene.objects.iter().zip(&b.scene.objects).enumerate() {
            assert_eq!(x.spatial_group, y.spatial_group, "objeto {i}");
            assert_eq!(x.reveal_group, y.reveal_group, "objeto {i}");
            assert_eq!(x.initial_material, y.initial_material, "objeto {i}");
            let k = aguas_a.iter().position(|&j| j == i);
            let intacto = k.is_none_or(|k| matches!(roles[k], Rol::Borde | Rol::Volumen));
            if intacto {
                assert_eq!(format!("{x:?}"), format!("{y:?}"), "objeto {i} cambio");
            }
        }
        for (i, (x, y)) in a.scene.palette.iter().zip(&b.scene.palette).enumerate() {
            assert_eq!(format!("{x:?}"), format!("{y:?}"), "material {i} mutado");
        }
        for (i, (x, y)) in a.scene.textures.iter().zip(&b.scene.textures).enumerate() {
            assert!(texeles(x) == texeles(y), "textura {i} mutada");
        }
        assert_eq!(
            format!("{:?}", a.scene.skybox),
            format!("{:?}", b.scene.skybox)
        );
        assert_eq!(format!("{:?}", a.anchors), format!("{:?}", b.anchors));
        assert_eq!(format!("{:?}", a.scale), format!("{:?}", b.scale));
        assert_eq!(a.hero_preset(), b.hero_preset());
        assert_eq!(
            format!("{:?}", camara_cenital(&a)),
            format!("{:?}", camara_cenital(b))
        );
        assert_eq!(
            format!("{:?}", luces_del_diorama(&a.anchors, &a.scale)),
            format!("{:?}", luces_del_diorama(&b.anchors, &b.scale))
        );
        assert_eq!(
            format!("{:?}", a.accel.bounds),
            format!("{:?}", b.accel.bounds)
        );
        for (x, y) in a.accel.groups.iter().zip(&b.accel.groups) {
            assert_eq!(x.id, y.id);
            if x.id == SpatialGroupId::FlyingWaters {
                assert_eq!(x.clusters.len(), y.clusters.len());
            } else {
                assert_eq!(format!("{x:?}"), format!("{y:?}"), "grupo {:?}", x.id);
            }
        }
        // Materiales: los de Aguas de antes o nuevos, y los nuevos solo en Aguas.
        let de_aguas_antes: Vec<usize> = aguas_a
            .iter()
            .map(|&i| a.scene.objects[i].final_material.0)
            .collect();
        for (i, o) in b.scene.objects.iter().enumerate() {
            let m = o.final_material.0;
            if o.spatial_group == SpatialGroupId::FlyingWaters {
                assert!(m >= c.paleta_previa || de_aguas_antes.contains(&m), "{i}");
            } else {
                assert!(m < c.paleta_previa, "{i} usa un material nuevo");
            }
        }
        for m in &b.scene.palette[c.paleta_previa..] {
            assert!(m.is_valid());
            assert!(m.albedo_texture.expect("textura").0 >= c.texturas_previas);
        }
        assert_eq!(
            measure_scene_radius(&b.scene, b.anchors.orbit_center),
            a.scale.scene_radius
        );
    }

    #[test]
    fn split_es_determinista_y_su_jerarquia_es_la_de_la_escena() {
        let a = partido();
        let b = partido();
        assert_eq!(
            format!("{:?}", a.diorama.scene.objects),
            format!("{:?}", b.diorama.scene.objects)
        );
        assert_eq!(
            format!("{:?}", a.diorama.scene.palette),
            format!("{:?}", b.diorama.scene.palette)
        );
        for (x, y) in a
            .diorama
            .scene
            .textures
            .iter()
            .zip(&b.diorama.scene.textures)
        {
            assert!(texeles(x) == texeles(y));
        }
        let d = &a.diorama;
        let (w, h) = (200, 150);
        for camara in [d.hero_camera(), camara_cenital(d)] {
            for p in 0..w * h {
                let rayo = camara.ray_from_pixel(p % w, p / w, w, h);
                let mut stats = Default::default();
                assert_eq!(
                    d.accel
                        .intersect(&d.scene, &rayo, &mut stats)
                        .map(|x| x.object_index),
                    d.scene.intersect(&rayo).map(|x| x.object_index)
                );
            }
        }
    }

    #[test]
    fn split_cabe_en_su_territorio_y_en_el_volumen() {
        let a = entrega();
        let c = partido();
        let territorio = union(
            indices_de_aguas(&a.scene)
                .into_iter()
                .map(|i| caja(&a.scene, i)),
        )
        .expect("Aguas tiene piezas");
        let p = piezas_con(&c, &roles_split());
        let v = volumen(&p);

        for (k, (rol, b)) in p.iter().enumerate() {
            assert!(
                b.min.x >= territorio.min.x - EPS
                    && b.min.z >= territorio.min.z - EPS
                    && b.max.x <= territorio.max.x + EPS
                    && b.max.z <= territorio.max.z + EPS
                    && b.min.y >= territorio.min.y - EPS
                    && b.max.y <= territorio.max.y + EPS,
                "{rol:?} {k} sale del territorio"
            );
            if k == 0 || matches!(rol, Rol::Borde | Rol::Volumen) {
                continue;
            }
            assert!(
                b.min.x > v.min.x && b.max.x < v.max.x && b.min.z > v.min.z && b.max.z < v.max.z,
                "{rol:?} {k} sale del volumen en planta"
            );
        }

        // Solo la roda rompe la superficie.
        let asoman: Vec<Rol> = p
            .iter()
            .filter(|(r, b)| !matches!(r, Rol::Borde | Rol::Volumen) && b.max.y > v.max.y)
            .map(|(r, _)| *r)
            .collect();
        assert!(
            asoman.iter().all(|r| *r == Rol::Roda),
            "asoma algo mas: {asoman:?}"
        );
    }

    /// Semántica de apoyo de `split_wreck`, que **sustituye** a
    /// `todo_el_interior_llega_al_lecho_sin_huecos` solo para esta variante:
    /// el fragmento alto está suspendido a propósito. Lo que se exige es que
    /// todo lo **demás** —lecho, rocas, corales, kelp, ancla, fragmento bajo
    /// y cadena— llegue al suelo por contactos sin pasar por el fragmento
    /// alto.
    #[test]
    fn split_todo_salvo_el_fragmento_alto_llega_al_lecho() {
        let c = partido();
        let p = piezas_con(&c, &roles_split());
        let abajo: Vec<usize> = (0..p.len())
            .filter(|&k| !matches!(p[k].0, Rol::Borde | Rol::Volumen) && !p[k].0.es_alto())
            .collect();
        let mut conectada = vec![false; p.len()];
        conectada[0] = true;
        loop {
            let mut cambio = false;
            for &k in &abajo {
                if !conectada[k]
                    && abajo
                        .iter()
                        .any(|&s| conectada[s] && tocan(&p[k].1, &p[s].1))
                {
                    conectada[k] = true;
                    cambio = true;
                }
            }
            if !cambio {
                break;
            }
        }
        let sueltas: Vec<(usize, Rol)> = abajo
            .iter()
            .filter(|&&k| !conectada[k])
            .map(|&k| (k, p[k].0))
            .collect();
        assert!(sueltas.is_empty(), "sin apoyo hasta el lecho: {sueltas:?}");
    }

    #[test]
    fn split_el_fragmento_bajo_esta_empotrado_y_pesa_mas() {
        let c = partido();
        let p = piezas_con(&c, &roles_split());
        let suelo = p[0].1;
        let bajo = de(&p, |r| r == Rol::CascoBajo);
        let alto = de(&p, Rol::es_alto);

        let hundido = bajo
            .iter()
            .map(|b| suelo.max.y - b.min.y)
            .fold(f32::MIN, f32::max);
        assert!(
            (0.05..=EMPOTRADO_MAXIMO).contains(&hundido),
            "el fragmento bajo no muerde el lecho: {hundido:.2}"
        );
        assert!(
            madera(&bajo) > madera(&alto),
            "el fragmento bajo no tiene mas masa"
        );
        assert!(
            relleno(&bajo) >= 0.35,
            "el fragmento bajo es un esqueleto: {:.2}",
            relleno(&bajo)
        );
    }

    #[test]
    fn split_el_fragmento_alto_es_un_esqueleto_de_proa_suspendido_y_conexo() {
        let c = partido();
        let p = piezas_con(&c, &roles_split());
        let alto = de(&p, Rol::es_alto);
        let quilla = de(&p, |r| r == Rol::Quilla);
        let roda = de(&p, |r| r == Rol::Roda);
        let costillas = de(&p, |r| r == Rol::Costilla);
        let fondo: Vec<Aabb> = de(&p, |r| {
            !r.es_alto() && !matches!(r, Rol::Borde | Rol::Volumen | Rol::Cadena)
        });

        assert_eq!(quilla.len(), 1);
        assert_eq!(roda.len(), 1);
        assert!(costillas.len() >= 6);

        // Conexo por dentro, sin la cadena.
        let mut conectada = vec![false; alto.len()];
        conectada[0] = true;
        loop {
            let mut cambio = false;
            for k in 0..alto.len() {
                if !conectada[k]
                    && (0..alto.len()).any(|s| conectada[s] && tocan(&alto[k], &alto[s]))
                {
                    conectada[k] = true;
                    cambio = true;
                }
            }
            if !cambio {
                break;
            }
        }
        assert!(conectada.iter().all(|&x| x), "el fragmento alto se deshace");

        // Suspendido: no toca nada del fondo y deja vacío debajo.
        for a in &alto {
            assert!(
                !fondo.iter().any(|f| tocan(a, f)),
                "el fragmento alto toca el fondo: {a:?}"
            );
        }
        let caja = union(alto.iter().copied()).expect("hay fragmento alto");
        assert!(
            caja.min.y - p[0].1.max.y >= 0.9,
            "no hay vacio debajo: {caja:?}"
        );
        let debajo = Aabb::new(
            nalgebra_glm::Vec3::new(caja.min.x, 1.0, caja.min.z),
            nalgebra_glm::Vec3::new(caja.max.x, caja.min.y - EPS, caja.max.z),
        );
        for (r, b) in &p {
            if *r == Rol::Cadena || matches!(r, Rol::Borde | Rol::Volumen) {
                continue;
            }
            let entra = b.min.x < debajo.max.x
                && b.max.x > debajo.min.x
                && b.min.y < debajo.max.y
                && b.max.y > debajo.min.y
                && b.min.z < debajo.max.z
                && b.max.z > debajo.min.z;
            assert!(!entra, "{r:?} ocupa el vacio bajo el fragmento alto: {b:?}");
        }

        // Esqueleto: costillas delgadas con aire, irregulares, que se
        // cierran hacia la roda.
        let q = quilla[0];
        let x_roda = (roda[0].min.x + roda[0].max.x) * 0.5;
        let mut xs: Vec<(f32, f32, f32, f32)> = costillas
            .iter()
            .map(|r| {
                let [x, y, z] = tamano(r);
                assert!(
                    x <= 0.15 && y >= 3.0 * x && z >= 0.3,
                    "costilla no delgada: {r:?}"
                );
                assert!(tocan(r, &q), "costilla suelta de la quilla: {r:?}");
                ((r.min.x + r.max.x) * 0.5, x, z, r.max.y)
            })
            .collect();
        xs.sort_by(|a, b| (a.0 - x_roda).abs().total_cmp(&(b.0 - x_roda).abs()));
        assert!(
            xs[0].2 <= 0.5 * xs[xs.len() - 1].2,
            "la proa no se estrecha: {xs:?}"
        );
        let techos: Vec<f32> = xs.iter().map(|t| t.3).collect();
        let rango = techos.iter().cloned().fold(f32::MIN, f32::max)
            - techos.iter().cloned().fold(f32::MAX, f32::min);
        assert!(rango >= 0.1, "costillas de altura uniforme: {techos:?}");
        let mut orden: Vec<f32> = xs.iter().map(|t| t.0).collect();
        orden.sort_by(f32::total_cmp);
        let pasos: Vec<f32> = orden.windows(2).map(|w| w[1] - w[0]).collect();
        let grosor = xs.iter().map(|t| t.1).fold(0.0, f32::max);
        assert!(
            pasos.iter().all(|&d| d >= 2.0 * grosor),
            "costillas sin aire: {pasos:?}"
        );
        let razon = pasos.iter().cloned().fold(f32::MIN, f32::max)
            / pasos.iter().cloned().fold(f32::MAX, f32::min);
        assert!(razon >= 1.15, "costillas a paso uniforme: {pasos:?}");
        assert!(
            relleno(&alto) <= 0.25,
            "el fragmento alto es masa: {:.2}",
            relleno(&alto)
        );
    }

    #[test]
    fn split_los_fragmentos_estan_separados_y_la_cadena_los_une() {
        let c = partido();
        let p = piezas_con(&c, &roles_split());
        let alto = de(&p, Rol::es_alto);
        let bajo = de(&p, |r| r == Rol::CascoBajo);
        let cadena = de(&p, |r| r == Rol::Cadena);
        let (ca, cb) = (
            union(alto.iter().copied()).unwrap(),
            union(bajo.iter().copied()).unwrap(),
        );

        let hueco = (cb.min.x - ca.max.x)
            .max(ca.min.x - cb.max.x)
            .max(cb.min.y - ca.max.y)
            .max(ca.min.y - cb.max.y)
            .max(cb.min.z - ca.max.z)
            .max(ca.min.z - cb.max.z);
        assert!(hueco >= 0.3, "los fragmentos casi se tocan: {hueco:.2}");

        assert!(
            alto.iter().any(|a| tocan(a, &cadena[0])),
            "la cadena no sale del fragmento alto"
        );
        assert!(
            bajo.iter().any(|b| tocan(b, &cadena[cadena.len() - 1])),
            "la cadena no llega al fragmento bajo"
        );
        let centro = |b: &Aabb| (b.min + b.max) * 0.5;
        for par in cadena.windows(2) {
            assert!(tocan(&par[0], &par[1]), "la cadena se corta");
            assert!(
                centro(&par[1]).y < centro(&par[0]).y,
                "la cadena no desciende"
            );
        }
        // Cuelga: los tramos interiores quedan bajo la recta de sus extremos.
        let (a, b) = (centro(&cadena[0]), centro(&cadena[cadena.len() - 1]));
        let bajo_la_recta = cadena[1..cadena.len() - 1].iter().any(|s| {
            let q = centro(s);
            let t = (q - a).dot(&(b - a)) / (b - a).dot(&(b - a));
            q.y < a.y + (b.y - a.y) * t - 0.03
        });
        assert!(bajo_la_recta, "la cadena va tiesa, sin comba");
    }

    #[test]
    fn split_el_ancla_reposa_en_el_lecho_junto_al_fragmento_bajo() {
        let c = partido();
        let p = piezas_con(&c, &roles_split());
        let ancla = de(&p, |r| r == Rol::Ancla);
        let bajo = de(&p, |r| r == Rol::CascoBajo);
        let pie = ancla.iter().map(|b| b.min.y).fold(f32::MAX, f32::min);
        assert!(
            (-EPS..=EMPOTRADO_MAXIMO).contains(&(p[0].1.max.y - pie)),
            "el ancla no reposa"
        );
        assert!(
            ancla.iter().any(|a| bajo.iter().any(|b| tocan(a, b))),
            "el ancla no toca el casco bajo"
        );
    }

    #[test]
    fn split_los_dos_fragmentos_y_la_cadena_se_leen_a_traves_del_agua() {
        let c = partido();
        let d = &c.diorama;
        let roles = roles_split();
        let lectura = lectura_por_rol(d, &roles, &d.hero_camera());
        let suma = |f: fn(Rol) -> bool| -> usize {
            lectura
                .iter()
                .filter(|(r, _, _)| f(*r))
                .map(|(_, a, b)| a + b)
                .sum()
        };
        let alto = suma(Rol::es_alto);
        let bajo = suma(|r| r == Rol::CascoBajo);
        let cadena = suma(|r| r == Rol::Cadena);

        assert!(alto >= 300, "el fragmento alto apenas se ve: {alto} px");
        assert!(bajo >= 200, "el fragmento bajo apenas se ve: {bajo} px");
        assert!(cadena >= 80, "la cadena apenas se ve: {cadena} px");

        let huecos = huecos_de(d, &roles, &camara_cenital(d), Rol::es_alto);
        assert!(
            huecos >= 0.35,
            "desde arriba el esqueleto tapa: {huecos:.2}"
        );
    }

    #[test]
    fn split_conserva_kelp_lecho_y_corales_de_la_candidata() {
        let c = partido();
        let p = piezas_con(&c, &roles_split());
        let base = nuevo();
        let q = piezas(&base);
        // Kelp y lecho, idénticos en geometría a la candidata.
        for (k, ((r, b), (_, a))) in p.iter().zip(&q).enumerate() {
            if matches!(r, Rol::Kelp | Rol::Lecho) {
                assert_eq!(a, b, "{r:?} {k} cambio respecto de la candidata");
            }
        }
        // Corales bajos.
        for b in de(&p, |r| r == Rol::Coral) {
            assert!(tamano(&b)[1] <= 0.25, "coral alto: {b:?}");
        }
    }

    /// `split_wreck` tal y como se aprobó. La cadena fina no puede moverlo.
    const HUELLA_SPLIT: u64 = 1_748_721_582_335_294_780;

    #[test]
    fn split_wreck_aprobado_no_cambia() {
        assert_eq!(huella_de(&partido()), HUELLA_SPLIT, "split_wreck cambio");
    }

    // --------------------------------------------- split_wreck_thin_chain

    /// Huecos de cadena en `roles_split`.
    const HUECOS_DE_CADENA: std::ops::Range<usize> = 20..26;

    fn fina() -> Candidato {
        split_cadena_fina(entrega())
    }

    fn cadena_de(c: &Candidato) -> Vec<Aabb> {
        de(&piezas_con(c, &roles_split()), |r| r == Rol::Cadena)
    }

    #[test]
    fn fina_cuenta_58_y_168_con_los_mismos_roles() {
        let c = fina();
        let scene = &c.diorama.scene;
        assert_eq!(indices_de_aguas(scene).len(), 58);
        assert_eq!(scene.objects.len(), 168);
        assert_eq!(cadena_de(&c).len(), 6, "la cadena sigue en seis piezas");
    }

    /// Todo lo que no es cadena es `split_wreck` byte a byte: barcos, ancla,
    /// lecho, kelp, corales, `A-01`, `A-11` y el resto de la escena. La
    /// paleta y las texturas de `split_wreck` quedan delante, intactas, y
    /// solo se añade el material negro con su textura.
    #[test]
    fn fina_solo_cambia_la_cadena_y_un_material_nuevo() {
        let a = partido();
        let b = fina();
        let aguas = indices_de_aguas(&a.diorama.scene);
        let (x, y) = (&a.diorama, &b.diorama);

        assert_eq!(x.scene.objects.len(), y.scene.objects.len());
        for (i, (o, n)) in x.scene.objects.iter().zip(&y.scene.objects).enumerate() {
            let es_cadena = aguas
                .iter()
                .position(|&j| j == i)
                .is_some_and(|k| HUECOS_DE_CADENA.contains(&k));
            if es_cadena {
                assert_eq!(o.spatial_group, n.spatial_group);
                assert_eq!(o.reveal_group, n.reveal_group);
                assert_eq!(o.initial_material, n.initial_material);
            } else {
                assert_eq!(format!("{o:?}"), format!("{n:?}"), "objeto {i} cambio");
            }
        }
        assert_eq!(
            y.scene.palette.len(),
            x.scene.palette.len() + 1,
            "un material nuevo"
        );
        assert_eq!(
            y.scene.textures.len(),
            x.scene.textures.len() + 1,
            "una textura nueva"
        );
        for (i, (m, n)) in x.scene.palette.iter().zip(&y.scene.palette).enumerate() {
            assert_eq!(format!("{m:?}"), format!("{n:?}"), "material {i} mutado");
        }
        for (i, (t, u)) in x.scene.textures.iter().zip(&y.scene.textures).enumerate() {
            assert!(texeles(t) == texeles(u), "textura {i} mutada");
        }
        assert_eq!(
            format!("{:?}", x.scene.skybox),
            format!("{:?}", y.scene.skybox)
        );
        assert_eq!(format!("{:?}", x.anchors), format!("{:?}", y.anchors));
        assert_eq!(format!("{:?}", x.scale), format!("{:?}", y.scale));
        assert_eq!(x.hero_preset(), y.hero_preset());
        assert_eq!(
            format!("{:?}", camara_cenital(x)),
            format!("{:?}", camara_cenital(y))
        );
        assert_eq!(
            format!("{:?}", luces_del_diorama(&x.anchors, &x.scale)),
            format!("{:?}", luces_del_diorama(&y.anchors, &y.scale))
        );
        for (g, h) in x.accel.groups.iter().zip(&y.accel.groups) {
            assert_eq!(g.id, h.id);
            if g.id == SpatialGroupId::FlyingWaters {
                assert_eq!(g.clusters.len(), h.clusters.len());
            } else {
                assert_eq!(format!("{g:?}"), format!("{h:?}"), "grupo {:?}", g.id);
            }
        }
        // Y la escena entera respecto de la entrega, fuera de Aguas.
        let e = entrega();
        for (i, (o, n)) in e.scene.objects.iter().zip(&y.scene.objects).enumerate() {
            if o.spatial_group != SpatialGroupId::FlyingWaters {
                assert_eq!(
                    format!("{o:?}"),
                    format!("{n:?}"),
                    "objeto {i} ajeno a Aguas"
                );
            }
        }
    }

    #[test]
    fn fina_el_agua_conserva_su_optica() {
        let e = entrega();
        let c = fina();
        let vol_e = *indices_de_aguas(&e.scene).last().unwrap();
        let vol_c = *indices_de_aguas(&c.diorama.scene).last().unwrap();
        let (a, b) = (
            e.scene.material(e.scene.objects[vol_e].final_material),
            c.diorama
                .scene
                .material(c.diorama.scene.objects[vol_c].final_material),
        );
        assert_eq!(format!("{a:?}"), format!("{b:?}"));
        assert_eq!(
            (b.reflection_cap, b.transmission_cap, b.ior),
            (0.9, 0.9, 1.333)
        );
        assert_eq!(
            b.shadow_mode,
            expedition33_continente_inacabado::material::ShadowMode::Ignore
        );
    }

    #[test]
    fn fina_la_cadena_es_continua_y_conserva_extremos_y_recorrido() {
        let c = fina();
        let p = piezas_con(&c, &roles_split());
        let cadena = cadena_de(&c);
        let gruesa = cadena_de(&partido());
        let alto = de(&p, Rol::es_alto);
        let bajo = de(&p, |r| r == Rol::CascoBajo);
        let centro = |b: &Aabb| (b.min + b.max) * 0.5;

        assert!(
            alto.iter().any(|a| tocan(a, &cadena[0])),
            "no cuelga del fragmento alto"
        );
        assert!(
            bajo.iter().any(|b| tocan(b, &cadena[5])),
            "no llega al fragmento bajo"
        );
        for par in cadena.windows(2) {
            assert!(tocan(&par[0], &par[1]), "la cadena se corta: {par:?}");
            assert!(
                centro(&par[1]).y < centro(&par[0]).y,
                "la cadena no desciende"
            );
        }
        // El recorrido no se acorta: la envolvente cubre la de la cadena
        // gruesa en los ejes por los que avanza.
        let (f, g) = (
            union(cadena.iter().copied()).unwrap(),
            union(gruesa.iter().copied()).unwrap(),
        );
        assert!(
            f.max.z - f.min.z >= (g.max.z - g.min.z) - 0.10,
            "recorrido en z acortado"
        );
        // La caída se mide entre los contactos y no por la envolvente, que
        // incluye el grosor: la fina baja al menos tan hondo como la gruesa y
        // arranca dentro de la quilla del fragmento alto.
        let quilla = de(&p, |r| r == Rol::Quilla)[0];
        assert!(f.min.y <= g.min.y + 0.02, "caida acortada por abajo");
        assert!(f.max.y >= quilla.min.y, "caida acortada por arriba");
        // Y sigue el gesto: cada tramo fino queda cerca del trazado grueso.
        for t in &cadena {
            let q = centro(t);
            let cerca = gruesa.iter().any(|g| (centro(g) - q).magnitude() <= 0.30);
            assert!(cerca, "un tramo se aparta del gesto: {t:?}");
        }
    }

    /// Ancho de la cadena en pantalla: mediana, por fila, de los píxeles
    /// donde la cadena es lo primero que se ve, directa o a través del agua.
    fn ancho_en_pantalla(
        c: &Candidato,
        camara: &expedition33_continente_inacabado::camera::Camera,
    ) -> f32 {
        let d = &c.diorama;
        let aguas = indices_de_aguas(&d.scene);
        let cadena: Vec<usize> = HUECOS_DE_CADENA.map(|k| aguas[k]).collect();
        let vistas = mirar(d, camara, ANCHO, ALTO);
        let mut por_fila = vec![0usize; ALTO];
        for (p, v) in vistas.iter().enumerate() {
            let Some(v) = v else { continue };
            let i = if cadena.contains(&v.primero.object_index) {
                Some(v.primero.object_index)
            } else {
                v.detras
                    .map(|h| h.object_index)
                    .filter(|i| cadena.contains(i))
            };
            if i.is_some() {
                por_fila[p / ANCHO] += 1;
            }
        }
        let mut filas: Vec<usize> = por_fila.into_iter().filter(|&n| n > 0).collect();
        filas.sort_unstable();
        filas.get(filas.len() / 2).copied().unwrap_or(0) as f32
    }

    #[test]
    fn fina_la_cadena_es_mas_fina_medida_en_escena_y_en_pantalla() {
        let gruesa = partido();
        let c = fina();
        for t in cadena_de(&c) {
            assert!(tamano(&t)[0] <= 0.10, "tramo grueso en x: {t:?}");
        }
        for camara in [c.diorama.hero_camera(), camara_cercana()] {
            let (antes, ahora) = (
                ancho_en_pantalla(&gruesa, &camara),
                ancho_en_pantalla(&c, &camara),
            );
            assert!(ahora > 0.0, "la cadena desaparece");
            assert!(
                ahora <= 0.6 * antes,
                "no se adelgaza en pantalla: {ahora} contra {antes} px"
            );
        }
    }

    #[test]
    fn fina_el_material_negro_es_propio_de_la_cadena() {
        let a = partido();
        let c = fina();
        let scene = &c.diorama.scene;
        let aguas = indices_de_aguas(scene);
        let id = scene.objects[aguas[HUECOS_DE_CADENA.start]].final_material;

        assert!(
            id.0 >= a.diorama.scene.palette.len(),
            "la cadena no usa un material nuevo"
        );
        for (i, o) in scene.objects.iter().enumerate() {
            let es_cadena = aguas
                .iter()
                .position(|&j| j == i)
                .is_some_and(|k| HUECOS_DE_CADENA.contains(&k));
            assert_eq!(o.final_material == id, es_cadena, "objeto {i}");
        }
        let m = scene.material(id);
        let t = scene.texture(m.albedo_texture.expect("textura procedural"));
        assert!(m.albedo_texture.unwrap().0 >= a.diorama.scene.textures.len());
        let pico = t.peak();
        let luz = |c: expedition33_continente_inacabado::color::Color| {
            0.2126 * c.r * m.albedo.r + 0.7152 * c.g * m.albedo.g + 0.0722 * c.b * m.albedo.b
        };
        assert!(luz(pico) <= 0.05, "no es negro: pico {:.3}", luz(pico));
        assert!(luz(pico) >= 0.004, "negro absoluto: pico {:.4}", luz(pico));
        assert!(
            (0.2..=0.6).contains(&m.specular_strength),
            "brillo {}",
            m.specular_strength
        );
        assert!(m.shininess >= 16.0);
        assert_eq!((m.reflection_cap, m.transmission_cap), (0.0, 0.0));
        assert!(m.is_valid());
    }

    /// Lectura bajo el agua: la cadena negra se ve, se lee más oscura que lo
    /// que la rodea y ni es un agujero negro ni se quema.
    #[test]
    fn fina_la_cadena_negra_se_lee_bajo_el_agua() {
        use expedition33_continente_inacabado::framebuffer::Framebuffer;
        use expedition33_continente_inacabado::renderer::{render, Shading};
        use expedition33_continente_inacabado::reveal::RevealState;

        let c = fina();
        let d = &c.diorama;
        let camara = d.hero_camera();
        let luces = luces_del_diorama(&d.anchors, &d.scale);
        let mut fb = Framebuffer::new(ANCHO, ALTO);
        render(
            &mut fb,
            &d.scene,
            &d.accel,
            &luces,
            &RevealState::painted(),
            &camara,
            Shading::Material,
        );
        let aguas = indices_de_aguas(&d.scene);
        let cadena: Vec<usize> = HUECOS_DE_CADENA.map(|k| aguas[k]).collect();
        let vistas = mirar(d, &camara, ANCHO, ALTO);
        let es = |p: usize| {
            vistas[p].is_some_and(|v| {
                cadena.contains(&v.primero.object_index)
                    || v.detras.is_some_and(|h| cadena.contains(&h.object_index))
            })
        };
        let luma = |c: u32| {
            0.2126 * ((c >> 16) & 0xFF) as f32
                + 0.7152 * ((c >> 8) & 0xFF) as f32
                + 0.0722 * (c & 0xFF) as f32
        };
        let (mut suya, mut n, mut vecina, mut m, mut quemados) = (0.0, 0, 0.0, 0, 0);
        for p in 0..ANCHO * ALTO {
            if !es(p) {
                continue;
            }
            suya += luma(fb.buffer[p]);
            n += 1;
            quemados += usize::from(
                [16, 8, 0]
                    .iter()
                    .all(|&s| (fb.buffer[p] >> s) & 0xFF >= 250),
            );
            for q in [p.saturating_sub(3), p + 3] {
                if q < ANCHO * ALTO && !es(q) {
                    vecina += luma(fb.buffer[q]);
                    m += 1;
                }
            }
        }
        assert!(n >= 25, "la cadena apenas se ve: {n} px");
        let (suya, vecina) = (suya / n as f32, vecina / m.max(1) as f32);
        assert!(suya >= 8.0, "agujero negro: luma {suya:.1}");
        assert!(
            suya <= 0.8 * vecina,
            "no se lee oscura: {suya:.1} contra {vecina:.1}"
        );
        assert_eq!(quemados, 0, "brillo quemado");
    }

    #[test]
    fn fina_es_determinista_y_su_jerarquia_es_la_de_la_escena() {
        let a = fina();
        let b = fina();
        assert_eq!(huella_de(&a), huella_de(&b));
        let d = &a.diorama;
        let (w, h) = (200, 150);
        for camara in [d.hero_camera(), camara_cenital(d), camara_cercana()] {
            for p in 0..w * h {
                let rayo = camara.ray_from_pixel(p % w, p / w, w, h);
                let mut stats = Default::default();
                assert_eq!(
                    d.accel
                        .intersect(&d.scene, &rayo, &mut stats)
                        .map(|x| x.object_index),
                    d.scene.intersect(&rayo).map(|x| x.object_index)
                );
            }
        }
    }

    // ---------------------------------------------------------- promoción

    /// La entrega de producción con los assets reales.
    fn produccion(water: WaterPreset) -> Blockout {
        produccion_con(water, &raiz())
    }

    /// Todo lo observable de dos dioramas, campo a campo: objetos, paleta,
    /// texturas texel a texel, cielo, anclas, escala, cámaras, luces y la
    /// jerarquía completa.
    fn iguales(x: &Blockout, y: &Blockout, que: &str) {
        assert_eq!(x.scene.objects.len(), y.scene.objects.len(), "{que}");
        for (i, (o, n)) in x.scene.objects.iter().zip(&y.scene.objects).enumerate() {
            assert_eq!(format!("{o:?}"), format!("{n:?}"), "{que}: objeto {i}");
        }
        assert_eq!(x.scene.palette.len(), y.scene.palette.len(), "{que}");
        for (i, (m, n)) in x.scene.palette.iter().zip(&y.scene.palette).enumerate() {
            assert_eq!(format!("{m:?}"), format!("{n:?}"), "{que}: material {i}");
        }
        assert_eq!(x.scene.textures.len(), y.scene.textures.len(), "{que}");
        for (i, (t, u)) in x.scene.textures.iter().zip(&y.scene.textures).enumerate() {
            assert_eq!((t.width(), t.height()), (u.width(), u.height()));
            assert!(texeles(t) == texeles(u), "{que}: textura {i}");
        }
        assert_eq!(
            format!("{:?}", x.scene.skybox),
            format!("{:?}", y.scene.skybox),
            "{que}"
        );
        assert_eq!(format!("{:?}", x.anchors), format!("{:?}", y.anchors));
        assert_eq!(format!("{:?}", x.scale), format!("{:?}", y.scale));
        assert_eq!(x.hero_preset(), y.hero_preset());
        for camara in [
            (x.hero_camera(), y.hero_camera()),
            (camara_cenital(x), camara_cenital(y)),
        ] {
            assert_eq!(format!("{:?}", camara.0), format!("{:?}", camara.1));
        }
        assert_eq!(
            format!("{:?}", luces_del_diorama(&x.anchors, &x.scale)),
            format!("{:?}", luces_del_diorama(&y.anchors, &y.scale))
        );
        assert_eq!(
            format!("{:?}", x.accel),
            format!("{:?}", y.accel),
            "{que}: jerarquia"
        );
    }

    /// La promoción: `delivery_level_con` es `split_wreck_thin_chain` byte
    /// a byte, con los assets reales, en los dos presets que tienen el
    /// volumen.
    #[test]
    fn la_entrega_de_produccion_es_split_wreck_thin_chain_aprobado() {
        for water in [WaterPreset::RefractiveWater, WaterPreset::OpaqueWater] {
            let aprobada = split_cadena_fina(actual_con(water, &raiz()));
            iguales(&aprobada.diorama, &produccion(water), &format!("{water:?}"));
        }
    }

    /// `InteriorVisible` no tiene volumen que reescribir alrededor: es la
    /// entrega refractiva sin `A-01`, con el mismo pecio.
    #[test]
    fn el_preset_interior_lleva_el_mismo_pecio_sin_el_volumen() {
        let refractiva = produccion(WaterPreset::RefractiveWater);
        let interior = produccion(WaterPreset::InteriorVisible);
        let volumen = *indices_de_aguas(&refractiva.scene)
            .last()
            .expect("A-01 va al final");

        assert_eq!(interior.scene.objects.len(), 167);
        assert_eq!(indices_de_aguas(&interior.scene).len(), 57);
        let sin_volumen = refractiva
            .scene
            .objects
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != volumen)
            .map(|(_, o)| o);
        for (i, (o, n)) in sin_volumen.zip(&interior.scene.objects).enumerate() {
            assert_eq!(format!("{o:?}"), format!("{n:?}"), "objeto {i}");
        }
        assert_eq!(refractiva.scene.palette.len(), interior.scene.palette.len());
        assert_eq!(
            refractiva.scene.textures.len(),
            interior.scene.textures.len()
        );
    }
}
