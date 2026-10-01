//! Preview de fidelidad de **Praderas Primaverales**: la entrega vigente
//! contra tres variantes que solo reescriben las 37 piezas de Praderas: la
//! candidata aprobada, su pulido `refined` y `refined_water_polish`, que
//! solo ajusta el agua de `refined`.
//!
//! ```bash
//! cargo run --release --example meadows_fidelity_preview -- <carpeta>
//! ```
//!
//! La carpeta es **obligatoria**: el ejemplo no escribe evidencia oficial
//! por su cuenta. Deja ocho PNG a `800 × 600`, todos con
//! `RevealState::painted` y los assets reales:
//!
//! - `{current,candidate,refined,refined_water_polish}_hero.png` — la toma
//!   hero.
//! - `{current,candidate,refined,refined_water_polish}_top78.png` — la misma
//!   órbita a `78°` de elevación: mismo centro, mismo punto de mira, mismo
//!   radio.
//!
//! Las cuatro columnas comparten **la misma** cámara y **las mismas** luces,
//! medidas una sola vez sobre la entrega: ninguna variante mueve anclas ni
//! escala, y un test lo exige. Además imprime, por variante, píxeles por
//! rol, rayos por tipo, tiempos intercalados, contraste de cada caída con
//! su roca y el reparto del magenta del tapiz por teselas.
//!
//! # Qué cambia
//!
//! Parte de `scenes::delivery_level_previo_con` —la entrega **anterior** a
//! la promoción de Praderas, `168` primitivas, `37` en Praderas— y sustituye **en su sitio** primitiva y material final de las
//! 37 piezas de Praderas. El índice, el grupo espacial, el grupo de
//! revelado y el material inicial (lienzo) de cada una se conservan; todo
//! lo ajeno a Praderas queda igual byte a byte. Después reconstruye la
//! jerarquía con el mismo plan de clusters que la entrega.
//!
//! | Entrada | Actual | Candidato |
//! |---|---:|---:|
//! | `P-01` meseta | 6 | 6 — base flotante intacta y cinco terrazas apoyadas en ella |
//! | `P-02` frente | 8 | 8 — repisas oscuras de ancho, caída y vuelo desiguales |
//! | `P-03` césped | 4 | 4 láminas con **tapiz** procedural verde y magenta |
//! | `P-04` árboles | 6 | 6 — dos árboles más bajos, a los flancos |
//! | `P-05` cascada | 1 | 1 caída |
//! | `P-07` flores | 12 | 5 caídas + 6 acentos bajos + 1 lámina de tapiz |
//! | **Total** | **37** | **37** |
//!
//! Las doce flores pagan cinco caídas más (`F-15`: seis en total), seis
//! acentos bajos y una quinta lámina de tapiz (`F-14`). Ninguna primitiva
//! nueva, ningún tipo nuevo, ningún cambio del motor; todo son cuboides,
//! así que Praderas sale igual en la Ruta A y en la B.
//!
//! # Por qué seis caídas y no siete
//!
//! En la hero el Monolito tapa el centro del frente —la cascada actual cae
//! justo detrás y solo deja 13 píxeles a la vista— y el corredor de `F-16`
//! le añade `1.5°` a cada lado. Quedan dos flancos de algo más de un metro:
//! caben dos caídas por flanco con frente oscuro entre ellas (`D-H`: muchas
//! y la pared desaparece). Las otras dos caen por la cara de la pared alta
//! del fondo derecho, que es la que da a la cámara. Una séptima caería tras
//! el Monolito o dentro del corredor.
//!
//! # Materiales de la candidata
//!
//! Ocho materiales **nuevos**, añadidos al final de la paleta y usados solo
//! por Praderas, sobre seis texturas procedurales construidas en memoria
//! con `Texture::from_pixels` (las dos variantes del tapiz comparten una, y
//! la base y el frente otra). Ningún material ni textura compartidos se
//! modifican; los troncos siguen en `aged_wood`, que se reutiliza sin
//! tocarlo.
//!
//! # `refined`
//!
//! El pulido aprobado sobre la candidata, con las mismas 37 piezas y la
//! misma base, frente y caídas en planta. La candidata sigue intacta: una
//! huella en los tests lo comprueba.
//!
//! - **Agua.** Las caídas de la candidata son opacas. Las de `refined` son
//!   copias **por valor** del agua vigente de Praderas —`0.9 / 0.9`, `ior
//!   1.333`, sombras `Ignore`, mismo brillo— con textura longitudinal
//!   propia (espuma al nacer y al desembocar, hebras y huecos) y un tinte
//!   blanco frío. El material compartido no se toca. El tradeoff es real:
//!   con esos techos el color propio pesa un diez por ciento y el resto es
//!   lo que hay detrás, así que el chorro se lee como agua translúcida
//!   sobre la roca y no como franja blanca. Para que siga leyéndose, el
//!   frente y la pared de las caídas interiores pasan a una roca más
//!   oscura, y las cuatro caídas al vacío desembocan en el canto inferior de
//!   su roca.
//! - **Tapiz.** Discos a tres escalas sobre un campo de densidad con claros
//!   y bordes deformados; cada lámina pequeña con su semilla, para que no
//!   repitan el mismo recorte.
//! - **Silueta.** Las mismas cinco terrazas redistribuidas: el escalón
//!   derecho vuela sobre el canto delantero y la pared izquierda por
//!   detrás; el saliente no va al centro porque desde el otro lado taparía
//!   más Monolito. Ver `TERRAZAS_REFINADAS`.
//!
//! # `refined_water_polish`
//!
//! `refined` **exacto** —geometría, tapiz, terrazas, todos los materiales—
//! salvo las dos aguas locales de las caídas: mismos techos `0.9 / 0.9`,
//! `ior 1.333` y sombras `Ignore`, con brillo directo ancho (`0.80 / 6` y
//! `0.65 / 9` frente al `0.18 / 128` vigente) y una textura de más
//! contraste. El brillo es la única palanca que no limita el diez por
//! ciento de color propio; ver `AGUA_PULIDA`. Huellas en los tests impiden
//! que la candidata o `refined` se muevan.
//!
//! # Promoción
//!
//! `refined_water_polish` es el default desde que Charlie lo aprobó: vive
//! en `src/scenes/meadows_delivery.rs` y `delivery_level_con` lo aplica.
//! Este preview conserva su implementación propia como **referencia**, y
//! el test `la_entrega_de_produccion_es_el_pulido_de_agua_aprobado` exige
//! que las dos den la misma escena byte a byte. `current` sigue siendo la
//! entrega previa, de modo que los ocho PNG se reproducen idénticos.

fn main() -> std::process::ExitCode {
    imp::correr()
}

mod imp {
    use std::path::{Path, PathBuf};
    use std::process::ExitCode;
    use std::time::Instant;

    use expedition33_continente_inacabado::accel::{ClusterPlan, SceneAccel};
    use expedition33_continente_inacabado::bounds::Aabb;
    use expedition33_continente_inacabado::camera::Camera;
    use expedition33_continente_inacabado::color::Color;
    use expedition33_continente_inacabado::cuboid::Cuboid;
    use expedition33_continente_inacabado::framebuffer::Framebuffer;
    use expedition33_continente_inacabado::hit::Hit;
    use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
    use expedition33_continente_inacabado::material::Material;
    use expedition33_continente_inacabado::renderer::{render, Shading};
    use expedition33_continente_inacabado::reveal::{resolve, RevealState};
    use expedition33_continente_inacabado::scene::{MaterialId, Scene, SpatialGroupId};
    use expedition33_continente_inacabado::scene_builder::{
        eye_at, Blockout, HALF_VERTICAL_FOV_DEGREES, HERO_YAW_DEGREES, MAX_RADIUS_FACTOR,
        MIN_RADIUS_FACTOR,
    };
    use expedition33_continente_inacabado::scenes::{delivery_level_previo_con, WaterPreset};
    use expedition33_continente_inacabado::texture::Texture;
    use nalgebra_glm::Vec3;

    pub const ANCHO: usize = 800;
    pub const ALTO: usize = 600;

    /// Elevación de la toma cenital, la misma de los previews del Rompeolas.
    pub const ELEVACION_CENITAL: f32 = 78.0;

    /// Piezas del Rompeolas original en la entrega; el resto es la segunda
    /// formación. Replica el plan de `edge_island::medir_y_acelerar`.
    const ROMPEOLAS_ORIGINAL: usize = 38;

    // -----------------------------------------------------------------
    // Roles y materiales
    // -----------------------------------------------------------------

    /// Qué hace cada pieza de Praderas en la composición.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Rol {
        Meseta,
        Frente,
        Cesped,
        Tronco,
        Copa,
        Caida,
        Acento,
        /// Solo en la entrega actual: las doce flores de `P-07`.
        Flor,
    }

    impl Rol {
        /// Lo que no es terreno: lo que no debe ocupar el corredor.
        #[cfg(test)]
        pub fn es_decoracion(self) -> bool {
            matches!(
                self,
                Rol::Tronco | Rol::Copa | Rol::Caida | Rol::Acento | Rol::Flor
            )
        }

        /// Lo que puede hacer de frente oscuro detrás de una caída.
        pub fn es_roca(self) -> bool {
            matches!(self, Rol::Meseta | Rol::Frente)
        }
    }

    /// Los materiales de Praderas en el candidato.
    ///
    /// Todos nuevos salvo `madera`, que es `aged_wood` reutilizado sin
    /// tocar.
    #[derive(Debug, Clone, Copy)]
    pub struct MaterialesPraderas {
        /// La base flotante: roca oscura, la misma que el frente.
        pub zocalo: MaterialId,
        /// Las terrazas: roca cálida con musgo.
        pub roca: MaterialId,
        pub frente: MaterialId,
        /// Tapiz de cada una de las cuatro láminas pequeñas. En la candidata
        /// es el mismo material cuatro veces; en `refined`, cuatro semillas.
        pub tapiz: [MaterialId; 4],
        /// El mismo tapiz, más repetido, para la lámina grande.
        pub tapiz_amplio: MaterialId,
        pub copa: MaterialId,
        pub caida: MaterialId,
        /// Las caídas estrechas. En la candidata es el mismo `caida`; en
        /// `refined` es otra agua con su propia textura.
        pub caida_fina: MaterialId,
        pub acento: MaterialId,
        pub madera: MaterialId,
    }

    impl MaterialesPraderas {
        /// Los materiales que este preview añade a la paleta.
        pub fn propios(&self) -> Vec<MaterialId> {
            let mut propios = vec![
                self.zocalo,
                self.roca,
                self.frente,
                self.tapiz[0],
                self.tapiz[1],
                self.tapiz[2],
                self.tapiz[3],
                self.tapiz_amplio,
                self.copa,
                self.caida,
                self.caida_fina,
                self.acento,
            ];
            propios.dedup();
            propios
        }

        fn rol_de(&self, material: MaterialId) -> Option<Rol> {
            [
                (self.zocalo, Rol::Meseta),
                (self.roca, Rol::Meseta),
                (self.frente, Rol::Frente),
                (self.tapiz[0], Rol::Cesped),
                (self.tapiz[1], Rol::Cesped),
                (self.tapiz[2], Rol::Cesped),
                (self.tapiz[3], Rol::Cesped),
                (self.tapiz_amplio, Rol::Cesped),
                (self.madera, Rol::Tronco),
                (self.copa, Rol::Copa),
                (self.caida, Rol::Caida),
                (self.caida_fina, Rol::Caida),
                (self.acento, Rol::Acento),
            ]
            .iter()
            .find(|(id, _)| *id == material)
            .map(|(_, rol)| *rol)
        }
    }

    pub struct Candidato {
        pub diorama: Blockout,
        pub materiales: MaterialesPraderas,
        /// Longitud de la paleta y de la tabla de texturas de la entrega:
        /// todo lo que está por debajo es compartido y no se toca.
        pub paleta_previa: usize,
        pub texturas_previas: usize,
    }

    /// La línea base del preview: la entrega tal y como estaba cuando se
    /// aprobaron las variantes. Desde la promoción, `delivery_level_con` ya
    /// lleva `refined_water_polish`; esta es la de antes.
    pub fn actual(raiz: &Path) -> Blockout {
        delivery_level_previo_con(WaterPreset::RefractiveWater, Some(raiz))
            .expect("los assets reales tienen que estar en la raiz del proyecto")
    }

    pub fn indices_de_praderas(scene: &Scene) -> Vec<usize> {
        scene
            .objects
            .iter()
            .enumerate()
            .filter(|(_, o)| o.spatial_group == SpatialGroupId::Meadows)
            .map(|(i, _)| i)
            .collect()
    }

    /// Roles de la entrega actual, en el orden de `meadows::praderas`.
    pub fn roles_actuales() -> Vec<Rol> {
        let mut roles = Vec::with_capacity(37);
        roles.extend([Rol::Meseta; 6]);
        roles.extend([Rol::Frente; 8]);
        roles.extend([Rol::Cesped; 4]);
        for _ in 0..2 {
            roles.extend([Rol::Tronco, Rol::Copa, Rol::Copa]);
        }
        roles.push(Rol::Caida);
        roles.extend([Rol::Flor; 12]);
        roles
    }

    /// Rol de cada pieza del candidato, **medido** por su material final.
    pub fn roles(c: &Candidato) -> Vec<Option<Rol>> {
        indices_de_praderas(&c.diorama.scene)
            .into_iter()
            .map(|i| {
                c.materiales
                    .rol_de(c.diorama.scene.objects[i].final_material)
            })
            .collect()
    }

    // -----------------------------------------------------------------
    // Texturas procedurales, en memoria
    // -----------------------------------------------------------------

    /// Hash entero a `0.0..1.0`, el mismo esquema que `generate_assets`.
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

    fn suavizar(t: f32) -> f32 {
        t * t * (3.0 - 2.0 * t)
    }

    /// Ruido de valor **periódico** en `celdas_x × celdas_y`: la textura
    /// repite sin costura, que es lo que pide `WrapMode::Repeat`.
    fn ruido(u: f32, v: f32, celdas_x: i32, celdas_y: i32, semilla: u32) -> f32 {
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

    /// Evalúa `patron(u, v)` en el centro de cada texel, con `v = 0` abajo
    /// como `Texture::sample`.
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

    /// El único magenta de Praderas (`F-18`, `D-I` pendiente): el mismo
    /// valor en el tapiz y en los acentos.
    pub fn magenta() -> Color {
        Color::from_srgb(0.84, 0.24, 0.56)
    }

    fn rosa() -> Color {
        Color::from_srgb(0.80, 0.46, 0.66)
    }

    /// Discos de flor en una rejilla periódica de `celdas × celdas`, con
    /// radio y presencia por celda. Devuelve `Some(true)` en el núcleo de
    /// un disco, `Some(false)` en su halo y `None` fuera.
    ///
    /// Cada disco aparece solo donde `densidad` lo deja: así las manchas se
    /// agrupan en **derivas** en vez de repartirse parejas.
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

    /// `F-14` · el tapiz: verde con derivas de manchas magenta.
    ///
    /// Las manchas son **grandes a propósito**: discos de entre un quinto y
    /// un tercio de metro, de tres a seis píxeles en la hero. Un punto de un
    /// texel sería invisible o, peor, centelleo bajo el muestreo por vecino
    /// más cercano.
    fn textura_tapiz() -> Texture {
        let oscuro = Color::from_srgb(0.20, 0.36, 0.14);
        let claro = Color::from_srgb(0.40, 0.58, 0.22);
        let ocre = Color::from_srgb(0.52, 0.53, 0.26);

        textura(128, 128, |u, v| {
            let pasto = fbm(u, v, 4, 3, 0x7A91_0001);
            let mut color = mezclar(oscuro, claro, (pasto - 0.30) * 1.9);

            let seco = ruido(u, v, 3, 3, 0x7A91_0002);
            if seco > 0.70 {
                color = mezclar(color, ocre, (seco - 0.70) * 2.5);
            }

            let deriva = fbm(u, v, 2, 2, 0x7A91_0003);
            match disco(u, v, 7, (deriva - 0.22) * 1.9, 0x7A91_0004) {
                Some(true) => magenta(),
                Some(false) => rosa(),
                None => color,
            }
        })
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

    /// El frente: roca casi negra con regueros verticales húmedos.
    fn textura_frente() -> Texture {
        let fondo = Color::from_srgb(0.13, 0.14, 0.16);
        let reguero = Color::from_srgb(0.23, 0.25, 0.28);

        textura(64, 64, |u, v| {
            let vertical = ruido(u, v, 14, 2, 0xF0E7_0001);
            let grano = fbm(u, v, 5, 2, 0xF0E7_0002);

            mezclar(
                fondo,
                reguero,
                (vertical - 0.45) * 2.2 + (grano - 0.5) * 0.4,
            )
        })
    }

    /// Las caídas: hebras verticales blancas y celestes.
    fn textura_caida() -> Texture {
        let espuma = Color::from_srgb(0.94, 0.97, 1.00);
        let celeste = Color::from_srgb(0.62, 0.82, 0.93);

        textura(16, 64, |u, v| {
            let hebra = ruido(u, v, 4, 1, 0xCA1D_0001);
            let pulso = ruido(u, v, 2, 6, 0xCA1D_0002);

            mezclar(celeste, espuma, hebra * 0.8 + pulso * 0.4 - 0.1)
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

    /// `refined` · el tapiz sin estampado: discos a tres escalas sobre un
    /// campo de densidad con claros, y bordes deformados.
    ///
    /// - El **campo** es ruido de baja frecuencia con contraste: donde vale
    ///   cero queda un claro verde, donde vale uno una deriva densa.
    /// - Tres **escalas** de disco —racimos grandes, manchas medias y
    ///   salpicado— entran a distinta densidad, así que ninguna mancha se
    ///   repite al mismo tamaño.
    /// - El **desplazamiento** de dominio rompe el borde redondo.
    ///
    /// El salpicado más fino mide unos dos píxeles en la hero: más pequeño
    /// sería centelleo bajo el muestreo por vecino más cercano.
    fn textura_tapiz_refinada(semilla: u32) -> Texture {
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

    /// `refined` · el frente, más oscuro, para que el agua se lea encima.
    fn textura_frente_umbria() -> Texture {
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

    /// `refined` · el agua de una caída a lo largo: espuma al nacer y al
    /// desembocar, hebras que se abren y se cierran por el medio, y huecos
    /// más oscuros donde el chorro adelgaza.
    ///
    /// Con la óptica del agua el color propio pesa solo un diez por ciento:
    /// la textura modula el chorro, no lo pinta.
    /// `refined_water_polish` · la misma lectura longitudinal con más
    /// contraste: espuma blanca, huecos más hondos y pulsos más marcados.
    /// Solo modula el diez por ciento de color propio; la presencia la da
    /// el brillo de `AGUA_PULIDA`.
    fn textura_caida_pulida(semilla: u32) -> Texture {
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

    fn textura_caida_refinada(semilla: u32) -> Texture {
        let espuma = Color::from_srgb(0.96, 0.98, 1.00);
        let celeste = Color::from_srgb(0.58, 0.78, 0.90);
        let hueco = Color::from_srgb(0.30, 0.44, 0.54);

        textura(24, 128, |u, v| {
            let hebra = ruido(u, v, 5, 3, semilla);
            let pulso = fbm(u, v, 3, 2, semilla.wrapping_add(1));
            let mut color = mezclar(celeste, espuma, hebra * 0.8 + pulso * 0.5 - 0.35);

            if hebra < 0.28 {
                color = mezclar(color, hueco, (0.28 - hebra) * 3.0);
            }

            let nacimiento = ((v - 0.86) / 0.14).clamp(0.0, 1.0);
            let desembocadura = ((0.12 - v) / 0.12).clamp(0.0, 1.0);

            mezclar(color, espuma, nacimiento.max(desembocadura))
        })
    }

    fn con_textura(scene: &mut Scene, base: Material, tex: Texture, escala: f32) -> MaterialId {
        let id = scene.add_texture(tex);

        scene.add_material(base.with_texture(id).with_uv_scale(escala))
    }

    /// Firma común de los dos registros de materiales: la escena, el
    /// `aged_wood` compartido y el agua vigente de Praderas.
    type Registro = fn(&mut Scene, MaterialId, Material) -> MaterialesPraderas;

    /// Materiales de la candidata. Sus caídas son **opacas**: es la
    /// primera propuesta tal y como se aprobó, y `refined` lo corrige.
    fn registrar_materiales(
        scene: &mut Scene,
        madera: MaterialId,
        _agua: Material,
    ) -> MaterialesPraderas {
        let blanco = Material::new(Color::new(1.0, 1.0, 1.0));
        let mojado = blanco.with_specular(0.55, 64.0);

        // La base y el frente son la misma roca: una textura, dos escalas.
        let frente_tex = scene.add_texture(textura_frente());
        let zocalo = scene.add_material(mojado.with_texture(frente_tex).with_uv_scale(3.0));
        let frente = scene.add_material(mojado.with_texture(frente_tex).with_uv_scale(1.0));
        let roca = con_textura(scene, blanco.with_specular(0.08, 16.0), textura_roca(), 2.0);

        // Las dos variantes del tapiz comparten **una** textura.
        let tapiz_tex = scene.add_texture(textura_tapiz());
        let pasto = blanco.with_specular(0.04, 8.0).with_texture(tapiz_tex);
        // La repetición sale del tamaño: las láminas pequeñas miden de metro
        // y medio a dos, la grande casi siete por cuatro y medio. Con estas
        // dos escalas un disco mide lo mismo en las cinco.
        let tapiz = scene.add_material(pasto.with_uv_scale(0.7));
        let tapiz_amplio = scene.add_material(pasto.with_uv_scale(2.5));

        let copa = con_textura(scene, blanco.with_specular(0.04, 8.0), textura_copa(), 1.0);
        let caida = con_textura(
            scene,
            blanco.with_specular(0.35, 48.0),
            textura_caida(),
            1.0,
        );
        let acento = con_textura(
            scene,
            blanco.with_specular(0.04, 8.0),
            textura_acento(),
            1.0,
        );

        MaterialesPraderas {
            zocalo,
            roca,
            frente,
            tapiz: [tapiz; 4],
            tapiz_amplio,
            copa,
            caida,
            caida_fina: caida,
            acento,
            madera,
        }
    }

    /// Materiales de `refined`.
    ///
    /// El agua **no** es una caja blanca: cada caída es una copia del agua
    /// vigente de Praderas —mismos techos, mismo `ior`, mismo brillo, mismo
    /// modo de sombra— a la que solo se le cambian textura y tinte. El
    /// material compartido no se toca; se copia por valor.
    fn registrar_materiales_refinados(
        scene: &mut Scene,
        madera: MaterialId,
        agua: Material,
    ) -> MaterialesPraderas {
        registrar_con_agua(scene, madera, agua, &AGUA_REFINADA)
    }

    /// Materiales de `refined_water_polish`: los de `refined`, con **solo**
    /// las dos aguas locales ajustadas. Ver `AGUA_PULIDA`.
    fn registrar_materiales_agua(
        scene: &mut Scene,
        madera: MaterialId,
        agua: Material,
    ) -> MaterialesPraderas {
        registrar_con_agua(scene, madera, agua, &AGUA_PULIDA)
    }

    /// Qué se le cambia al agua vigente en cada variante: textura, tinte y
    /// brillo. Los techos, el `ior` y el modo de sombra **no** están aquí,
    /// así que ninguna variante puede tocarlos.
    struct AjusteAgua {
        textura: fn(u32) -> Texture,
        /// Tinte en sRGB.
        tinte: [f32; 3],
        /// `(fuerza, exponente)` del brillo de `[caida, caida_fina]`;
        /// `None` conserva el del agua vigente.
        brillo: [Option<(f32, f32)>; 2],
    }

    /// `refined` tal y como se aprobó: brillo del agua vigente.
    const AGUA_REFINADA: AjusteAgua = AjusteAgua {
        textura: textura_caida_refinada,
        tinte: [0.92, 0.97, 1.00],
        brillo: [None, None],
    };

    /// `refined_water_polish`: el brillo es la palanca.
    ///
    /// El color propio del agua pesa un diez por ciento —`kl` con techos
    /// `0.9 / 0.9`— y lo transmitido no se tiñe, así que la textura apenas
    /// separa el chorro de la roca. El brillo directo, en cambio, se suma
    /// **después** del reparto de Fresnel y no lo escala `kl`. Con el
    /// exponente `128` del agua vigente no llega a la cámara hero: en las
    /// caras de las caídas `n·h` vale de `0.75` a `0.84` con `L-01`, y
    /// `0.8^128` es cero. Con un exponente bajo el brillo cubre el chorro
    /// como un velo ancho y no como un punto quemado.
    ///
    /// Las dos aguas llevan brillos distintos para que las anchas y las
    /// finas no se lean iguales. Barrido medido en la hero, contraste medio
    /// caída/roca (`refined` 4.02, candidata opaca 8.50):
    ///
    /// | Anchas | Finas | Contraste | Lectura |
    /// |---|---|---:|---|
    /// | `0.55 / 10` | `0.45 / 14` | 4.87 | apenas cambia |
    /// | **`0.80 / 6`** | **`0.65 / 9`** | **5.98** | chorro gris azulado legible |
    /// | `1.00 / 4` | `0.85 / 6` | 7.16 | velo uniforme y cálido: franja plana |
    ///
    /// Ninguno quema un píxel. El brillo no lleva textura: la variación a lo
    /// largo del chorro sigue saliendo del color propio y de la roca de
    /// detrás, y es débil.
    const AGUA_PULIDA: AjusteAgua = AjusteAgua {
        textura: textura_caida_pulida,
        tinte: [0.90, 0.96, 1.00],
        brillo: [Some((0.80, 6.0)), Some((0.65, 9.0))],
    };

    fn registrar_con_agua(
        scene: &mut Scene,
        madera: MaterialId,
        agua: Material,
        ajuste: &AjusteAgua,
    ) -> MaterialesPraderas {
        let blanco = Material::new(Color::new(1.0, 1.0, 1.0));
        let mojado = blanco.with_specular(0.55, 64.0);

        // Frente más umbrío: el agua transmite casi todo, así que lo que la
        // separa de la roca es su diez por ciento de color propio contra un
        // fondo más oscuro.
        let frente_tex = scene.add_texture(textura_frente_umbria());
        let zocalo = scene.add_material(mojado.with_texture(frente_tex).with_uv_scale(3.0));
        let frente = scene.add_material(mojado.with_texture(frente_tex).with_uv_scale(1.0));
        let roca = con_textura(scene, blanco.with_specular(0.08, 16.0), textura_roca(), 2.0);

        // Una repetición de unos cinco metros en las cinco láminas: la
        // lámina grande no repite su tapiz y una mancha mide lo mismo en
        // todas. Cada lámina pequeña lleva **su** semilla: como todas
        // empiezan en `uv = 0`, con una sola textura mostrarían el mismo
        // recorte. La tercera, la del escalón saliente, se eligió porque
        // con la correlativa la lámina más visible del flanco derecho caía
        // entera en un claro.
        let pasto = blanco.with_specular(0.04, 8.0);
        let amplio_tex = scene.add_texture(textura_tapiz_refinada(0x7A92_0000));
        let tapiz_amplio = scene.add_material(pasto.with_texture(amplio_tex).with_uv_scale(1.25));
        let tapiz = [0x7A93_0000, 0x7A94_0000, 0x7A9A_0000, 0x7A96_0000].map(|semilla| {
            let tex = scene.add_texture(textura_tapiz_refinada(semilla));
            scene.add_material(pasto.with_texture(tex).with_uv_scale(0.35))
        });

        let copa = con_textura(scene, blanco.with_specular(0.04, 8.0), textura_copa(), 1.0);
        let acento = con_textura(
            scene,
            blanco.with_specular(0.04, 8.0),
            textura_acento(),
            1.0,
        );

        let [r, g, b] = ajuste.tinte;
        let tinte = Color::from_srgb(r, g, b);
        let [caida, caida_fina] = [
            (0xCA1D_1001, ajuste.brillo[0]),
            (0xCA1D_2002, ajuste.brillo[1]),
        ]
        .map(|(semilla, brillo)| {
            let base = match brillo {
                Some((fuerza, exponente)) => agua.with_specular(fuerza, exponente),
                None => agua,
            };
            let id = con_textura(scene, base, (ajuste.textura)(semilla), 1.0);
            tenir(scene, id, tinte)
        });

        MaterialesPraderas {
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

    /// Tiñe un material **recién añadido** por este preview.
    fn tenir(scene: &mut Scene, id: MaterialId, tinte: Color) -> MaterialId {
        scene.palette[id.0] = scene.material(id).with_tint(tinte);
        id
    }

    // -----------------------------------------------------------------
    // Composición
    // -----------------------------------------------------------------

    /// Qué material lleva una pieza del candidato.
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

    /// Lo que sobresale una caída por encima de su labio: su nacimiento se
    /// ve desde arriba como una marca clara en el borde.
    const NACIMIENTO: f32 = 0.02;

    /// Profundidad de una caída, contra la cara que la respalda.
    const ESPESOR_DE_CAIDA: f32 = 0.14;

    // --- P-01 · meseta: la base y cinco terrazas apoyadas en ella

    /// La base flotante, idéntica en caja a la masa mayor de la entrega.
    const BASE: Caja = [-3.5, -1.4, -2.8, 3.5, 0.0, 2.8];

    /// Terrazas: pared alta al fondo, escalones a los flancos. Cada una
    /// muerde la base `0.10` y se queda a `0.12` de sus cantos.
    const TERRAZAS: [Caja; 5] = [
        // Pared izquierda del fondo, la más alta.
        [-3.38, -0.10, -2.68, -1.30, 1.35, -1.25],
        // Pared derecha del fondo, la de las caídas interiores.
        [1.20, -0.10, -2.68, 3.38, 1.45, -1.50],
        // Cresta central baja, detrás del Monolito: fondo tranquilo.
        [-1.40, -0.10, -2.68, 1.32, 0.70, -1.95],
        // Escalón bajo del flanco izquierdo, al frente.
        [-3.38, -0.10, 0.55, -1.75, 0.38, 2.62],
        // Escalón medio del flanco derecho.
        [1.65, -0.10, -0.85, 3.38, 0.50, 1.45],
    ];

    // --- P-02 · frente: repisas oscuras colgadas de la base

    /// Las ocho piezas cuelgan de la base y la muerden `0.10`. Las cuatro
    /// que respaldan una caída tienen la cara entre `2.785` y `2.79`: a ras
    /// de la base pero no coplanares con ella. Las demás vuelan hacia
    /// delante y hacen de repisa.
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

    // --- P-03 · láminas de tapiz

    /// Cinco láminas: el llano y los techos de cuatro terrazas; la cresta
    /// central se queda en roca. La quinta sale de una de las doce flores.
    /// `(caja en planta [x0, z0, x1, z1], cota del apoyo, amplia)`.
    const LAMINAS: [([f32; 4], f32, bool); 5] = [
        // Todo el techo de la base: el llano y lo que asoma a los flancos.
        ([-3.44, -1.88, 3.44, 2.74], 0.0, true),
        ([-3.35, 0.58, -1.78, 2.59], 0.38, false),
        ([-3.35, -2.65, -1.33, -1.28], 1.35, false),
        ([1.68, -0.82, 3.35, 1.42], 0.50, false),
        ([1.23, -2.65, 3.35, -1.55], 1.45, false),
    ];

    // --- P-04 · dos árboles a los flancos

    /// `(pie del tronco, escala)`. Tres primitivas por árbol, como en
    /// `meadows::arboles`, y más bajos que allí.
    const ARBOLES: [([f32; 3], f32); 2] =
        [([-3.05, 0.08, -0.98], 0.70), ([3.02, 0.08, 1.95], 0.66)];

    // --- P-05 · caídas

    /// `(x, ancho, pie, labio, cara)`: cada caída nace a la cota `labio`
    /// de la pieza cuya cara frontal está en `z = cara`, y cae hasta `pie`.
    const CAIDAS: [(f32, f32, f32, f32, f32); 6] = [
        // Al vacío, sobre la base y el frente, flanco izquierdo.
        (-3.15, 0.28, -3.00, 0.0, 2.80),
        (-2.62, 0.16, -2.10, 0.0, 2.80),
        // Al vacío, flanco derecho.
        (2.62, 0.22, -3.30, 0.0, 2.80),
        (3.18, 0.14, -1.60, 0.0, 2.80),
        // Interiores: de la pared derecha al llano.
        (2.80, 0.20, 0.0, 1.45, -1.50),
        (3.22, 0.26, 0.0, 1.45, -1.50),
    ];

    // --- P-07 · acentos bajos

    /// Seis macizos bajos de flor, cada uno posado en su apoyo.
    const ACENTOS: [Caja; 6] = [
        // Al pie del árbol derecho, en el canto frontal.
        [2.50, 0.04, 2.25, 3.30, 0.22, 2.70],
        // Rebosando el canto del escalón izquierdo.
        [-3.30, 0.42, 2.00, -2.55, 0.62, 2.62],
        // Al pie de las caídas interiores.
        [2.62, 0.04, -1.36, 3.30, 0.20, -0.95],
        // En lo alto de la pared izquierda.
        [-3.25, 1.39, -2.40, -2.70, 1.58, -1.85],
        [2.55, 0.54, -0.60, 3.20, 0.72, -0.10],
        // Al pie del árbol izquierdo.
        [-3.42, 0.04, -0.70, -2.80, 0.20, -0.30],
    ];

    /// Una lámina: `(caja en planta [x0, z0, x1, z1], cota del apoyo,
    /// amplia)`.
    type Lamina = ([f32; 4], f32, bool);

    /// Un árbol: `(pie del tronco, escala)`.
    type Arbol = ([f32; 3], f32);

    /// Una caída: `(x, ancho, pie, labio, cara)`.
    type Chorro = (f32, f32, f32, f32, f32);

    /// Lo que distingue una variante de otra. La base y el frente son
    /// comunes: los dos planos los heredan de la entrega aprobada.
    struct Plano {
        terrazas: [Caja; 5],
        /// Qué terrazas llevan la roca oscura de la base en vez de la cálida.
        oscuras: [bool; 5],
        laminas: [Lamina; 5],
        arboles: [Arbol; 2],
        caidas: [Chorro; 6],
        acentos: [Caja; 6],
    }

    const PLANO_CANDIDATA: Plano = Plano {
        terrazas: TERRAZAS,
        oscuras: [false; 5],
        laminas: LAMINAS,
        arboles: ARBOLES,
        caidas: CAIDAS,
        acentos: ACENTOS,
    };

    // --- refined: misma base, mismo frente, las seis caídas en su sitio

    /// `refined` · el contorno escalonado.
    ///
    /// Se redistribuyen las cinco terrazas existentes, sin añadir ninguna:
    ///
    /// - el **escalón derecho** pasa al frente, sube y **vuela** por encima
    ///   del canto: un saliente que rompe la recta delantera. Va a la
    ///   derecha del corredor y no en el centro, porque ahí cualquier
    ///   saliente subiría la rasante y taparía más Monolito visto desde el
    ///   otro lado (lo mide `desde_el_lado_de_praderas_…`);
    /// - el canto delantero queda **asimétrico**: escalón bajo a la
    ///   izquierda, llano al centro, saliente alto a la derecha y la esquina
    ///   otra vez baja, donde el árbol se queda a ras del llano para no
    ///   tapar las caídas interiores;
    /// - la **pared izquierda** adelgaza y vuela un poco por detrás; la
    ///   derecha se retranquea. Entre ellas y sus escalones quedan dos
    ///   entrantes verdes a los flancos;
    /// - la **cresta central** se queda atrás, baja: el fondo tranquilo del
    ///   Monolito.
    const TERRAZAS_REFINADAS: [Caja; 5] = [
        [-3.38, -0.10, -2.85, -1.55, 1.35, -1.05],
        [1.05, -0.10, -2.60, 3.38, 1.45, -1.50],
        [-1.40, -0.10, -2.68, 1.32, 0.70, -1.95],
        [-3.38, -0.10, 0.55, -1.75, 0.38, 2.62],
        // Saliente: el centro cae dentro de la base y vuela `0.30` fuera.
        [1.35, -0.10, 0.85, 2.40, 0.55, 3.10],
    ];

    const LAMINAS_REFINADAS: [Lamina; 5] = [
        ([-3.44, -1.88, 3.44, 2.74], 0.0, true),
        ([-3.35, 0.58, -1.78, 2.59], 0.38, false),
        ([-3.35, -2.82, -1.58, -1.08], 1.35, false),
        ([1.40, 0.90, 2.35, 3.05], 0.55, false),
        ([1.08, -2.57, 3.35, -1.55], 1.45, false),
    ];

    /// El árbol derecho, en la esquina baja junto al escalón.
    const ARBOLES_REFINADOS: [Arbol; 2] =
        [([-3.05, 0.08, -0.85], 0.70), ([3.08, 0.08, 2.20], 0.66)];

    /// Las seis caídas en el mismo sitio. Las cuatro al vacío desembocan
    /// en el canto inferior de la roca que las respalda: el agua deja el
    /// frente oscuro donde este termina, no a media pared.
    const CAIDAS_REFINADAS: [Chorro; 6] = [
        (-3.15, 0.28, -3.08, 0.0, 2.80),
        (-2.62, 0.16, -2.23, 0.0, 2.80),
        (2.62, 0.22, -3.43, 0.0, 2.80),
        (3.18, 0.14, -2.18, 0.0, 2.80),
        (2.80, 0.20, 0.0, 1.45, -1.50),
        (3.22, 0.26, 0.0, 1.45, -1.50),
    ];

    const ACENTOS_REFINADOS: [Caja; 6] = [
        // Al pie del árbol derecho, en el canto.
        [2.95, 0.04, 2.40, 3.44, 0.20, 2.72],
        [-3.30, 0.42, 2.00, -2.55, 0.62, 2.62],
        [2.62, 0.04, -1.36, 3.30, 0.20, -0.95],
        [-3.25, 1.39, -2.40, -2.70, 1.58, -1.85],
        // En el entrante del flanco derecho.
        [2.95, 0.04, 0.25, 3.42, 0.18, 0.70],
        [-3.42, 0.04, -0.62, -2.80, 0.20, -0.22],
    ];

    const PLANO_REFINADO: Plano = Plano {
        terrazas: TERRAZAS_REFINADAS,
        // La pared derecha, la de las caídas interiores, pasa a roca oscura:
        // con la óptica del agua, su caída solo se lee contra un fondo
        // oscuro, como la pared de cascadas de la referencia.
        oscuras: [false, true, false, false, false],
        laminas: LAMINAS_REFINADAS,
        arboles: ARBOLES_REFINADOS,
        caidas: CAIDAS_REFINADAS,
        acentos: ACENTOS_REFINADOS,
    };

    /// Las caídas de menos de este ancho llevan el agua `caida_fina`.
    const ANCHO_DE_CAIDA_FINA: f32 = 0.19;

    /// Las 37 piezas de una variante, en el orden de los índices de
    /// Praderas de la entrega: mismo hueco, otra forma.
    fn piezas(plano: &Plano) -> Vec<(Caja, Acabado)> {
        let mut piezas = Vec::with_capacity(37);

        piezas.push((BASE, Acabado::Zocalo));
        piezas.extend(
            plano
                .terrazas
                .iter()
                .zip(plano.oscuras)
                .map(|(&c, oscura)| {
                    (
                        c,
                        if oscura {
                            Acabado::Zocalo
                        } else {
                            Acabado::Roca
                        },
                    )
                }),
        );
        piezas.extend(FRENTE.iter().map(|&c| (c, Acabado::Frente)));

        let mut pequenas = 0;
        for &([x0, z0, x1, z1], apoyo, amplia) in &plano.laminas {
            let acabado = if amplia {
                Acabado::TapizAmplio
            } else {
                pequenas += 1;
                Acabado::Tapiz(pequenas - 1)
            };
            piezas.push(([x0, apoyo - EMPOTRADO, z0, x1, apoyo + LAMINA, z1], acabado));
        }

        for &([x, y, z], s) in &plano.arboles {
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

        for &(x, ancho, pie, labio, cara) in &plano.caidas {
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

        piezas.extend(plano.acentos.iter().map(|&c| (c, Acabado::Acento)));

        piezas
    }

    /// La candidata aprobada: la entrega con las 37 piezas de Praderas
    /// reescritas. Una huella en los tests impide que `refined` la mueva.
    pub fn candidato(actual: Blockout) -> Candidato {
        construir(actual, &PLANO_CANDIDATA, registrar_materiales)
    }

    /// La variante `refined`: el pulido aprobado sobre la candidata.
    pub fn refinado(actual: Blockout) -> Candidato {
        construir(actual, &PLANO_REFINADO, registrar_materiales_refinados)
    }

    /// La variante `refined_water_polish`: `refined` con las dos aguas
    /// locales ajustadas. Misma geometría, mismo tapiz, mismas terrazas.
    pub fn refinado_agua(actual: Blockout) -> Candidato {
        construir(actual, &PLANO_REFINADO, registrar_materiales_agua)
    }

    fn construir(actual: Blockout, plano: &Plano, registro: Registro) -> Candidato {
        let mut diorama = actual;
        let paleta_previa = diorama.scene.palette.len();
        let texturas_previas = diorama.scene.textures.len();
        let praderas = indices_de_praderas(&diorama.scene);

        // `aged_wood`: el material del primer tronco de la entrega.
        let madera = diorama.scene.objects[praderas[18]].final_material;
        // El agua vigente de Praderas: la de la cascada de la entrega.
        let agua = diorama
            .scene
            .material(diorama.scene.objects[praderas[24]].final_material);
        let m = registro(&mut diorama.scene, madera, agua);
        let ancla = diorama.anchors.meadows_anchor;
        let piezas = piezas(plano);

        assert_eq!(
            piezas.len(),
            praderas.len(),
            "el candidato reparte las mismas 37"
        );

        for (&i, ([x0, y0, z0, x1, y1, z1], acabado)) in praderas.iter().zip(piezas) {
            let objeto = &mut diorama.scene.objects[i];

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

        // La jerarquía guarda posiciones: se reconstruye **después** de
        // fijar la geometría.
        diorama.accel = reacelerar(&diorama.scene);

        Candidato {
            diorama,
            materiales: m,
            paleta_previa,
            texturas_previas,
        }
    }

    /// Reconstruye la jerarquía con el mismo plan que la entrega.
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

    // -----------------------------------------------------------------
    // Cámaras y trazado
    // -----------------------------------------------------------------

    /// La toma hero a `78°`: misma órbita, mismo `look_at`, mismo radio.
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

    /// Primer impacto de cada píxel, con la jerarquía real.
    pub fn trazar(
        diorama: &Blockout,
        camara: &Camera,
        ancho: usize,
        alto: usize,
    ) -> Vec<Option<Hit>> {
        let mut stats = Default::default();
        let mut impactos = Vec::with_capacity(ancho * alto);

        for y in 0..alto {
            for x in 0..ancho {
                let rayo = camara.ray_from_pixel(x, y, ancho, alto);
                impactos.push(diorama.accel.intersect(&diorama.scene, &rayo, &mut stats));
            }
        }

        impactos
    }

    #[cfg(test)]
    pub fn union(cajas: impl Iterator<Item = Aabb>) -> Option<Aabb> {
        cajas.reduce(|a, b| a.union(&b))
    }

    /// Píxeles de Praderas por rol en una toma: qué se lee de cada cosa a
    /// resolución real.
    pub fn pixeles_por_rol(
        diorama: &Blockout,
        roles: &[Rol],
        camara: &Camera,
    ) -> Vec<(Rol, usize)> {
        let praderas = indices_de_praderas(&diorama.scene);
        let mut cuenta: Vec<(Rol, usize)> = Vec::new();

        for h in trazar(diorama, camara, ANCHO, ALTO).into_iter().flatten() {
            let Some(k) = praderas.iter().position(|&i| i == h.object_index) else {
                continue;
            };
            match cuenta.iter_mut().find(|(r, _)| *r == roles[k]) {
                Some((_, n)) => *n += 1,
                None => cuenta.push((roles[k], 1)),
            }
        }

        cuenta
    }

    // -----------------------------------------------------------------
    // Salida
    // -----------------------------------------------------------------

    /// Píxeles que cambian de verdad: un salto de al menos `8/255`.
    fn perceptibles(a: &Framebuffer, b: &Framebuffer) -> f64 {
        let canal = |c: u32, d: u32| ((c >> d) & 0xFF) as i32;
        let cambian = a
            .buffer
            .iter()
            .zip(&b.buffer)
            .filter(|(x, y)| {
                [16, 8, 0]
                    .iter()
                    .any(|&d| (canal(**x, d) - canal(**y, d)).abs() >= 8)
            })
            .count();

        100.0 * cambian as f64 / a.buffer.len() as f64
    }

    /// Luma de un píxel del framebuffer, sobre los bytes sRGB.
    fn luma(c: u32) -> f32 {
        let canal = |d: u32| ((c >> d) & 0xFF) as f32;

        0.2126 * canal(16) + 0.7152 * canal(8) + 0.0722 * canal(0)
    }

    /// Contraste de cada caída con la roca que la rodea **en el cuadro
    /// renderizado**: luma media de sus píxeles entre la luma media de los
    /// píxeles de roca a tres o menos a izquierda y derecha, en sus filas.
    ///
    /// Es una medida de legibilidad, no de gusto: `1.0` es una caída que no
    /// se distingue de su pared.
    pub fn contraste_de_caidas(
        diorama: &Blockout,
        roles: &[Rol],
        camara: &Camera,
        cuadro: &Framebuffer,
    ) -> Vec<f32> {
        let impactos = trazar(diorama, camara, ANCHO, ALTO);
        let praderas = indices_de_praderas(&diorama.scene);
        let rol_en = |p: usize| {
            impactos[p]
                .and_then(|h| praderas.iter().position(|&i| i == h.object_index))
                .map(|k| roles[k])
        };

        (0..praderas.len())
            .filter(|&k| roles[k] == Rol::Caida)
            .map(|k| {
                let (mut agua, mut n_agua, mut roca, mut n_roca) = (0.0, 0, 0.0, 0);

                for (p, impacto) in impactos.iter().enumerate() {
                    if !impacto.is_some_and(|h| h.object_index == praderas[k]) {
                        continue;
                    }
                    agua += luma(cuadro.buffer[p]);
                    n_agua += 1;

                    let x = p % ANCHO;
                    for dx in 1..=3usize {
                        for q in [x.checked_sub(dx), Some(x + dx).filter(|&v| v < ANCHO)] {
                            let Some(qx) = q else { continue };
                            let vecino = p - x + qx;
                            if rol_en(vecino).is_some_and(Rol::es_roca) {
                                roca += luma(cuadro.buffer[vecino]);
                                n_roca += 1;
                            }
                        }
                    }
                }

                (agua / n_agua.max(1) as f32) / (roca / n_roca.max(1) as f32).max(1.0)
            })
            .collect()
    }

    /// Fracción de los píxeles de cada caída que el cuadro **quema**: los
    /// tres canales a `250` o más.
    pub fn quemado_por_caida(
        diorama: &Blockout,
        roles: &[Rol],
        camara: &Camera,
        cuadro: &Framebuffer,
    ) -> Vec<f32> {
        let impactos = trazar(diorama, camara, ANCHO, ALTO);
        let praderas = indices_de_praderas(&diorama.scene);

        (0..praderas.len())
            .filter(|&k| roles[k] == Rol::Caida)
            .map(|k| {
                let (mut suyos, mut quemados) = (0usize, 0usize);
                for (p, impacto) in impactos.iter().enumerate() {
                    if impacto.is_some_and(|h| h.object_index == praderas[k]) {
                        suyos += 1;
                        let c = cuadro.buffer[p];
                        quemados += usize::from([16, 8, 0].iter().all(|&d| (c >> d) & 0xFF >= 250));
                    }
                }
                quemados as f32 / suyos.max(1) as f32
            })
            .collect()
    }

    /// Reparto del magenta del tapiz en teselas de `12 × 12` píxeles:
    /// `(fracción magenta, coeficiente de variación entre teselas,
    /// fracción de teselas que son claro verde)`.
    ///
    /// Un estampado parejo da un coeficiente bajo y casi ningún claro; uno
    /// con derivas y claros, lo contrario. Solo cuentan las teselas con al
    /// menos la mitad de sus píxeles en tapiz.
    pub fn reparto_del_tapiz(
        diorama: &Blockout,
        roles: &[Rol],
        camara: &Camera,
    ) -> (f32, f32, f32) {
        const TESELA: usize = 12;

        let impactos = trazar(diorama, camara, ANCHO, ALTO);
        let praderas = indices_de_praderas(&diorama.scene);
        let pintado = RevealState::painted();
        let (mut fracciones, mut magenta, mut tapiz) = (Vec::new(), 0usize, 0usize);

        for ty in 0..ALTO / TESELA {
            for tx in 0..ANCHO / TESELA {
                let (mut en_tapiz, mut en_magenta) = (0usize, 0usize);

                for y in ty * TESELA..(ty + 1) * TESELA {
                    for x in tx * TESELA..(tx + 1) * TESELA {
                        let Some(h) = impactos[y * ANCHO + x] else {
                            continue;
                        };
                        let Some(k) = praderas.iter().position(|&i| i == h.object_index) else {
                            continue;
                        };
                        if roles[k] != Rol::Cesped {
                            continue;
                        }
                        let objeto = &diorama.scene.objects[h.object_index];
                        let a = resolve(&diorama.scene, objeto, &pintado, &h.uv).albedo;
                        en_tapiz += 1;
                        en_magenta += usize::from(a.r > 1.6 * a.g && a.b > 1.2 * a.g);
                    }
                }

                tapiz += en_tapiz;
                magenta += en_magenta;
                if en_tapiz * 2 >= TESELA * TESELA {
                    fracciones.push(en_magenta as f32 / en_tapiz as f32);
                }
            }
        }

        let n = fracciones.len().max(1) as f32;
        let media = fracciones.iter().sum::<f32>() / n;
        let varianza = fracciones.iter().map(|f| (f - media).powi(2)).sum::<f32>() / n;
        let claros = fracciones.iter().filter(|&&f| f < 0.03).count() as f32 / n;

        (
            magenta as f32 / tapiz.max(1) as f32,
            varianza.sqrt() / media.max(1e-6),
            claros,
        )
    }

    /// Veces que se repite cada render para el tiempo mediano.
    const REPETICIONES: usize = 9;

    pub fn correr() -> ExitCode {
        let Some(destino) = std::env::args().nth(1).map(PathBuf::from) else {
            eprintln!("uso: cargo run --release --example meadows_fidelity_preview -- <carpeta>");
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
        let pulido = refinado(actual(&raiz));
        let agua_pulida = refinado_agua(actual(&raiz));

        // Cámaras y luces medidas **una vez**, sobre la entrega, y usadas en
        // las tres columnas.
        let hero = vigente.hero_camera();
        let cenital = camara_cenital(&vigente);
        let luces = luces_del_diorama(&vigente.anchors, &vigente.scale);

        let roles_candidato: Vec<Rol> = roles(&propuesto).into_iter().flatten().collect();
        let roles_refinado: Vec<Rol> = roles(&pulido).into_iter().flatten().collect();
        let roles_agua: Vec<Rol> = roles(&agua_pulida).into_iter().flatten().collect();
        let roles_vigentes = roles_actuales();
        let columnas: [(&str, &Blockout, &[Rol]); 4] = [
            ("current", &vigente, &roles_vigentes),
            ("candidate", &propuesto.diorama, &roles_candidato),
            ("refined", &pulido.diorama, &roles_refinado),
            ("refined_water_polish", &agua_pulida.diorama, &roles_agua),
        ];

        println!("meadows_fidelity_preview\n");
        println!("  destino     {}", destino.display());
        println!("  dimension   {ANCHO} x {ALTO}, RevealState::painted, assets reales");
        for (nombre, diorama, _) in columnas {
            println!(
                "  {nombre:<10}  {} primitivas, Praderas {}",
                diorama.scene.objects.len(),
                indices_de_praderas(&diorama.scene).len()
            );
        }
        for (nombre, c) in [
            ("candidate", &propuesto),
            ("refined", &pulido),
            ("refined_water_polish", &agua_pulida),
        ] {
            let mut cuenta = std::collections::BTreeMap::new();
            for rol in roles(c).into_iter().flatten() {
                *cuenta.entry(format!("{rol:?}")).or_insert(0) += 1;
            }
            println!(
                "  {nombre:<10}  roles {cuenta:?}; {} materiales nuevos ({} compartidos intactos), {} texturas nuevas",
                c.materiales.propios().len(),
                c.paleta_previa,
                c.diorama.scene.textures.len() - c.texturas_previas
            );
        }

        let agua = |c: &Candidato| c.diorama.scene.material(c.materiales.caida);
        for (nombre, m) in [
            (
                "vigente",
                vigente.scene.material(
                    vigente.scene.objects[indices_de_praderas(&vigente.scene)[24]].final_material,
                ),
            ),
            ("candidate", agua(&propuesto)),
            ("refined", agua(&pulido)),
            ("water_polish", agua(&agua_pulida)),
        ] {
            println!(
                "  agua {nombre:<12} reflexion {:.2} transmision {:.2} ior {:.3} sombras {:?} brillo {:.2}/{:.0}",
                m.reflection_cap, m.transmission_cap, m.ior, m.shadow_mode, m.specular_strength, m.shininess
            );
        }
        println!(
            "  hero        ojo ({:.2}, {:.2}, {:.2})  cenital ojo ({:.2}, {:.2}, {:.2})\n",
            hero.eye.x, hero.eye.y, hero.eye.z, cenital.eye.x, cenital.eye.y, cenital.eye.z
        );

        for (toma, camara) in [("hero", &hero), ("top78", &cenital)] {
            for (nombre, diorama, roles) in columnas {
                println!(
                    "  px {toma:<6} {nombre:<10} {:?}",
                    pixeles_por_rol(diorama, roles, camara)
                );
            }
        }
        println!();

        let mut ok = true;

        for (toma, camara) in [("hero", &hero), ("top78", &cenital)] {
            // Un render por variante para el PNG y los rayos, que son
            // deterministas; después los tiempos, **intercalados**: cada
            // repetición pasa por las tres variantes, así que comparten las
            // condiciones de la máquina en vez de medirse en tandas.
            let pintar = |diorama: &Blockout| {
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
                (fb, stats)
            };

            let mut cuadros = Vec::new();
            let mut rayos = Vec::new();
            for (nombre, diorama, _) in columnas {
                let (fb, stats) = pintar(diorama);
                let ruta = destino.join(format!("{nombre}_{toma}.png"));
                match fb.save_png(&ruta) {
                    Ok(()) => println!("  escrito   {}", ruta.display()),
                    Err(e) => {
                        eprintln!("error: no se pudo escribir {}: {e}", ruta.display());
                        ok = false;
                    }
                }
                cuadros.push(fb);
                rayos.push(stats);
            }

            let mut tiempos = vec![Vec::with_capacity(REPETICIONES); columnas.len()];
            for _ in 0..REPETICIONES {
                for (k, (_, diorama, _)) in columnas.iter().enumerate() {
                    let inicio = Instant::now();
                    let _ = pintar(diorama);
                    tiempos[k].push(inicio.elapsed().as_secs_f64());
                }
            }

            for (k, (nombre, _, _)) in columnas.iter().enumerate() {
                let t = &mut tiempos[k];
                t.sort_by(f64::total_cmp);
                let r = &rayos[k];
                println!(
                    "    {toma:<6} {nombre:<10} min {:.4} s, mediana {:.4} s ({REPETICIONES} intercalados) · rayos: sombra {} reflejados {} refractados {} · pruebas de primitiva {}",
                    t[0],
                    t[REPETICIONES / 2],
                    r.shadow_rays,
                    r.reflection_rays,
                    r.refraction_rays,
                    r.primitive_tests
                );
            }

            println!(
                "  {toma:<6}    px perceptibles: candidate/current {:.2} %, refined/current {:.2} %, water_polish/refined {:.2} %",
                perceptibles(&cuadros[0], &cuadros[1]),
                perceptibles(&cuadros[0], &cuadros[2]),
                perceptibles(&cuadros[2], &cuadros[3])
            );

            for (k, (nombre, diorama, roles)) in columnas.iter().enumerate().skip(1) {
                let contraste = contraste_de_caidas(diorama, roles, camara, &cuadros[k]);
                let quemado = quemado_por_caida(diorama, roles, camara, &cuadros[k]);
                let media = contraste.iter().sum::<f32>() / contraste.len().max(1) as f32;
                let texto: Vec<String> = contraste.iter().map(|c| format!("{c:.2}")).collect();
                let quemas: Vec<String> = quemado
                    .iter()
                    .map(|q| format!("{:.1}", 100.0 * q))
                    .collect();
                println!(
                    "  {toma:<6} caida/roca {nombre:<20} media {media:.2}  [{}]  quemado % [{}]",
                    texto.join(", "),
                    quemas.join(", ")
                );
            }
            for (nombre, diorama, roles) in columnas.iter().skip(1) {
                let (fraccion, variacion, claros) = reparto_del_tapiz(diorama, roles, camara);
                println!(
                    "  tapiz {toma:<6} {nombre:<10} magenta {:.1} %, variacion entre teselas {variacion:.2}, teselas claro {:.1} %",
                    100.0 * fraccion,
                    100.0 * claros
                );
            }
            println!();
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
    use expedition33_continente_inacabado::accel::TraversalStats;
    use expedition33_continente_inacabado::bounds::Aabb;
    use expedition33_continente_inacabado::camera::Camera;
    use expedition33_continente_inacabado::framebuffer::Framebuffer;
    use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
    use expedition33_continente_inacabado::material::Material;
    use expedition33_continente_inacabado::ray_intersect::RayIntersect;
    use expedition33_continente_inacabado::renderer::{cast_ray, render, Shading};
    use expedition33_continente_inacabado::reveal::{resolve, RevealState};
    use expedition33_continente_inacabado::scene::MaterialId;
    use expedition33_continente_inacabado::scene::{Scene, SpatialGroupId};
    use expedition33_continente_inacabado::scene_builder::{
        measure_scene_radius, Blockout, EYE_ELEVATION_DEGREES, HALF_VERTICAL_FOV_DEGREES,
        HERO_YAW_DEGREES,
    };
    use expedition33_continente_inacabado::scenes::DELIVERY;
    use std::path::PathBuf;

    /// Cuánto puede hundirse una pieza en lo que la sostiene.
    const EMPOTRADO_MAXIMO: f32 = 0.15;
    const EPS: f32 = 1.0e-4;
    /// Ancho angular extra del corredor a cada lado del Monolito (D-G).
    const CORREDOR_GRADOS: f32 = 1.5;

    fn raiz() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn entrega() -> Blockout {
        actual(&raiz())
    }

    /// Cómo se construye una variante a partir de la entrega.
    type Construir = fn(Blockout) -> Candidato;

    fn caja(scene: &Scene, i: usize) -> Aabb {
        scene.objects[i].primitive.bounds()
    }

    fn tamano(c: &Aabb) -> [f32; 3] {
        [c.max.x - c.min.x, c.max.y - c.min.y, c.max.z - c.min.z]
    }

    fn centro_xz_dentro(p: &Aabb, s: &Aabb) -> bool {
        let (x, z) = ((p.min.x + p.max.x) * 0.5, (p.min.z + p.max.z) * 0.5);
        x >= s.min.x && x <= s.max.x && z >= s.min.z && z <= s.max.z
    }

    fn apoya_encima(p: &Aabb, s: &Aabb) -> bool {
        let hundida = s.max.y - p.min.y;
        (-EPS..=EMPOTRADO_MAXIMO).contains(&hundida) && centro_xz_dentro(p, s)
    }

    fn cuelga_de(p: &Aabb, s: &Aabb) -> bool {
        let hundida = p.max.y - s.min.y;
        (-EPS..=EMPOTRADO_MAXIMO).contains(&hundida) && centro_xz_dentro(p, s)
    }

    fn engarzada(p: &Aabb, s: &Aabb) -> bool {
        p.min.y > s.min.y && p.min.y <= s.max.y + EPS && centro_xz_dentro(p, s)
    }

    /// Roles del candidato; falla si alguna pieza no tiene material local.
    fn roles_completos(c: &Candidato) -> Vec<Rol> {
        let roles = roles(c);
        let sin_rol = roles.iter().filter(|r| r.is_none()).count();
        assert_eq!(
            sin_rol, 0,
            "{sin_rol} piezas de Praderas sin material propio"
        );
        roles.into_iter().map(|r| r.expect("comprobado")).collect()
    }

    /// La raíz es la masa de mayor planta: la base flotante aprobada.
    fn raiz_de(scene: &Scene, praderas: &[usize], roles: &[Rol]) -> usize {
        let area = |i: usize| {
            let t = tamano(&caja(scene, i));
            t[0] * t[2]
        };
        (0..praderas.len())
            .filter(|&k| roles[k] == Rol::Meseta)
            .max_by(|&a, &b| area(praderas[a]).total_cmp(&area(praderas[b])))
            .expect("hay meseta")
    }

    fn caidas(c: &Candidato) -> Vec<Aabb> {
        let praderas = indices_de_praderas(&c.diorama.scene);
        roles_completos(c)
            .iter()
            .zip(&praderas)
            .filter(|(r, _)| **r == Rol::Caida)
            .map(|(_, &i)| caja(&c.diorama.scene, i))
            .collect()
    }

    // ------------------------------------------------------------ conteo

    fn cuenta_37_en_praderas_y_168_en_total_en(construir: Construir) {
        let c = construir(entrega());
        let scene = &c.diorama.scene;
        let cuenta = |g| {
            scene
                .objects
                .iter()
                .filter(|o| o.spatial_group == g)
                .count()
        };

        assert_eq!(cuenta(SpatialGroupId::Meadows), 37);
        assert_eq!(cuenta(SpatialGroupId::Meadows), DELIVERY.meadows);
        assert_eq!(cuenta(SpatialGroupId::Breakwater), DELIVERY.breakwater);
        assert_eq!(cuenta(SpatialGroupId::FlyingWaters), DELIVERY.flying_waters);
        assert_eq!(scene.objects.len(), 168);
        assert_eq!(scene.objects.len(), DELIVERY.total());
    }

    #[test]
    fn cuenta_37_en_praderas_y_168_en_total() {
        cuenta_37_en_praderas_y_168_en_total_en(candidato);
    }

    #[test]
    fn cuenta_37_en_praderas_y_168_en_total_refinada() {
        cuenta_37_en_praderas_y_168_en_total_en(refinado);
    }

    #[test]
    fn cuenta_37_en_praderas_y_168_en_total_agua() {
        cuenta_37_en_praderas_y_168_en_total_en(refinado_agua);
    }

    // ------------------------------------------------------- invariancia

    /// Muestrea cada texel: dos texturas iguales dan la misma lista.
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

    fn fuera_de_praderas_nada_cambia_byte_a_byte_en(construir: Construir) {
        let a = entrega();
        let c = construir(entrega());
        let b = &c.diorama;

        assert_eq!(a.scene.objects.len(), b.scene.objects.len());
        for (i, (x, y)) in a.scene.objects.iter().zip(&b.scene.objects).enumerate() {
            assert_eq!(
                x.spatial_group, y.spatial_group,
                "objeto {i} cambio de grupo"
            );
            assert_eq!(
                x.reveal_group, y.reveal_group,
                "objeto {i} cambio de revelado"
            );
            if x.spatial_group == SpatialGroupId::Meadows {
                assert_eq!(
                    x.initial_material, y.initial_material,
                    "{i} no nace en lienzo"
                );
                continue;
            }
            assert_eq!(
                format!("{x:?}"),
                format!("{y:?}"),
                "objeto {i} ajeno a Praderas cambio"
            );
        }

        // Paleta y texturas previas, intactas: lo nuevo se añade al final.
        assert_eq!(c.paleta_previa, a.scene.palette.len());
        assert_eq!(c.texturas_previas, a.scene.textures.len());
        for (i, (x, y)) in a.scene.palette.iter().zip(&b.scene.palette).enumerate() {
            assert_eq!(
                format!("{x:?}"),
                format!("{y:?}"),
                "material compartido {i} mutado"
            );
        }
        for (i, (x, y)) in a.scene.textures.iter().zip(&b.scene.textures).enumerate() {
            assert!(texeles(x) == texeles(y), "textura compartida {i} mutada");
        }
        assert_eq!(
            format!("{:?}", a.scene.skybox),
            format!("{:?}", b.scene.skybox)
        );

        // Anclas, escala, cámara y luces comunes.
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

        // Jerarquía: mismos grupos y, fuera de Praderas, mismos clusters.
        assert_eq!(
            format!("{:?}", a.accel.bounds),
            format!("{:?}", b.accel.bounds)
        );
        assert_eq!(a.accel.groups.len(), b.accel.groups.len());
        for (x, y) in a.accel.groups.iter().zip(&b.accel.groups) {
            assert_eq!(x.id, y.id);
            if x.id != SpatialGroupId::Meadows {
                assert_eq!(
                    format!("{x:?}"),
                    format!("{y:?}"),
                    "grupo {:?} cambio",
                    x.id
                );
            }
        }
    }

    #[test]
    fn fuera_de_praderas_nada_cambia_byte_a_byte() {
        fuera_de_praderas_nada_cambia_byte_a_byte_en(candidato);
    }

    #[test]
    fn fuera_de_praderas_nada_cambia_byte_a_byte_refinada() {
        fuera_de_praderas_nada_cambia_byte_a_byte_en(refinado);
    }

    #[test]
    fn fuera_de_praderas_nada_cambia_byte_a_byte_agua() {
        fuera_de_praderas_nada_cambia_byte_a_byte_en(refinado_agua);
    }

    fn el_candidato_es_determinista_en(construir: Construir) {
        let a = construir(entrega());
        let b = construir(entrega());

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
            assert!(
                texeles(x) == texeles(y),
                "una textura procedural no es determinista"
            );
        }
    }

    #[test]
    fn el_candidato_es_determinista() {
        el_candidato_es_determinista_en(candidato);
    }

    #[test]
    fn el_candidato_es_determinista_refinada() {
        el_candidato_es_determinista_en(refinado);
    }

    #[test]
    fn el_candidato_es_determinista_agua() {
        el_candidato_es_determinista_en(refinado_agua);
    }

    // --------------------------------------------------------- territorio

    fn praderas_no_sale_de_su_territorio_ni_mueve_la_escala_en(construir: Construir) {
        let a = entrega();
        let c = construir(entrega());
        let territorio = union(
            indices_de_praderas(&a.scene)
                .into_iter()
                .map(|i| caja(&a.scene, i)),
        )
        .expect("Praderas tiene piezas");

        for i in indices_de_praderas(&c.diorama.scene) {
            let p = caja(&c.diorama.scene, i);
            assert!(
                p.min.x >= territorio.min.x - EPS
                    && p.min.y >= territorio.min.y - EPS
                    && p.min.z >= territorio.min.z - EPS
                    && p.max.x <= territorio.max.x + EPS
                    && p.max.y <= territorio.max.y + EPS
                    && p.max.z <= territorio.max.z + EPS,
                "la pieza {i} sale del territorio actual: {p:?} contra {territorio:?}"
            );
        }

        let radio = measure_scene_radius(&c.diorama.scene, c.diorama.anchors.orbit_center);
        assert_eq!(
            radio, a.scale.scene_radius,
            "el radio medido cambiaria la camara"
        );
    }

    #[test]
    fn praderas_no_sale_de_su_territorio_ni_mueve_la_escala() {
        praderas_no_sale_de_su_territorio_ni_mueve_la_escala_en(candidato);
    }

    #[test]
    fn praderas_no_sale_de_su_territorio_ni_mueve_la_escala_refinada() {
        praderas_no_sale_de_su_territorio_ni_mueve_la_escala_en(refinado);
    }

    #[test]
    fn praderas_no_sale_de_su_territorio_ni_mueve_la_escala_agua() {
        praderas_no_sale_de_su_territorio_ni_mueve_la_escala_en(refinado_agua);
    }

    fn las_escalas_y_los_materiales_nuevos_son_validos_en(construir: Construir) {
        let c = construir(entrega());
        let scene = &c.diorama.scene;

        for i in indices_de_praderas(scene) {
            for lado in tamano(&caja(scene, i)) {
                assert!(
                    lado.is_finite() && lado >= 0.05,
                    "pieza {i} degenerada: {lado}"
                );
            }
        }
        for (k, m) in scene.palette[c.paleta_previa..].iter().enumerate() {
            assert!(m.is_valid(), "material nuevo {k} fuera de rango");
            assert!(
                m.albedo_texture.is_some(),
                "material nuevo {k} sin textura procedural"
            );
            assert!(m.uv_scale > 0.0);
        }
    }

    #[test]
    fn las_escalas_y_los_materiales_nuevos_son_validos() {
        las_escalas_y_los_materiales_nuevos_son_validos_en(candidato);
    }

    #[test]
    fn las_escalas_y_los_materiales_nuevos_son_validos_refinada() {
        las_escalas_y_los_materiales_nuevos_son_validos_en(refinado);
    }

    #[test]
    fn las_escalas_y_los_materiales_nuevos_son_validos_agua() {
        las_escalas_y_los_materiales_nuevos_son_validos_en(refinado_agua);
    }

    // ---------------------------------------------------------- materiales

    fn materiales_y_texturas_son_exclusivos_de_praderas_en(construir: Construir) {
        let c = construir(entrega());
        let scene = &c.diorama.scene;
        let roles = roles_completos(&c);
        let m = c.materiales;
        let propios = m.propios();

        for &id in &propios {
            assert!(id.0 >= c.paleta_previa, "{id:?} no es un material nuevo");
        }
        for (k, i) in indices_de_praderas(scene).into_iter().enumerate() {
            let material = scene.objects[i].final_material;
            let compartido_permitido = roles[k] == Rol::Tronco && material == m.madera;
            assert!(
                propios.contains(&material) || compartido_permitido,
                "pieza {i} de Praderas usa un material compartido"
            );
        }
        for (i, o) in scene.objects.iter().enumerate() {
            if o.spatial_group != SpatialGroupId::Meadows {
                assert!(
                    !propios.contains(&o.final_material),
                    "objeto {i} usa un material de Praderas"
                );
                assert!(!propios.contains(&o.initial_material));
            }
        }
        for material in &scene.palette[..c.paleta_previa] {
            if let Some(t) = material.albedo_texture {
                assert!(
                    t.0 < c.texturas_previas,
                    "un material compartido apunta a textura nueva"
                );
            }
        }
        for material in &scene.palette[c.paleta_previa..] {
            let t = material.albedo_texture.expect("textura procedural");
            assert!(
                t.0 >= c.texturas_previas,
                "un material nuevo reutiliza una textura compartida"
            );
        }
    }

    #[test]
    fn materiales_y_texturas_son_exclusivos_de_praderas() {
        materiales_y_texturas_son_exclusivos_de_praderas_en(candidato);
    }

    #[test]
    fn materiales_y_texturas_son_exclusivos_de_praderas_refinada() {
        materiales_y_texturas_son_exclusivos_de_praderas_en(refinado);
    }

    #[test]
    fn materiales_y_texturas_son_exclusivos_de_praderas_agua() {
        materiales_y_texturas_son_exclusivos_de_praderas_en(refinado_agua);
    }

    // --------------------------------------------------------------- apoyo

    fn cada_pieza_tiene_apoyo_real_hasta_la_base_en(construir: Construir) {
        let c = construir(entrega());
        let scene = &c.diorama.scene;
        let praderas = indices_de_praderas(scene);
        let roles = roles_completos(&c);
        let raiz = raiz_de(scene, &praderas, &roles);
        let cajas: Vec<Aabb> = praderas.iter().map(|&i| caja(scene, i)).collect();

        // De qué piezas puede colgar o sobre cuáles puede posarse cada una.
        let apoyos = |k: usize| -> Vec<usize> {
            let p = &cajas[k];
            (0..cajas.len())
                .filter(|&s| s != k)
                .filter(|&s| {
                    let base = &cajas[s];
                    match roles[k] {
                        Rol::Frente => roles[s] == Rol::Meseta && cuelga_de(p, base),
                        Rol::Copa => {
                            matches!(roles[s], Rol::Tronco | Rol::Copa) && engarzada(p, base)
                        }
                        Rol::Caida => {
                            matches!(roles[s], Rol::Meseta | Rol::Cesped) && nace_en(p, base)
                        }
                        _ => matches!(roles[s], Rol::Meseta | Rol::Cesped) && apoya_encima(p, base),
                    }
                })
                .collect()
        };

        let mut conectada = vec![false; cajas.len()];
        conectada[raiz] = true;
        loop {
            let mut cambio = false;
            for k in 0..cajas.len() {
                if !conectada[k] && apoyos(k).iter().any(|&s| conectada[s]) {
                    conectada[k] = true;
                    cambio = true;
                }
            }
            if !cambio {
                break;
            }
        }

        let sueltas: Vec<(usize, Rol)> = (0..cajas.len())
            .filter(|&k| !conectada[k])
            .map(|k| (praderas[k], roles[k]))
            .collect();
        assert!(
            sueltas.is_empty(),
            "piezas sin apoyo real hasta la base: {sueltas:?}"
        );
    }

    #[test]
    fn cada_pieza_tiene_apoyo_real_hasta_la_base() {
        cada_pieza_tiene_apoyo_real_hasta_la_base_en(candidato);
    }

    #[test]
    fn cada_pieza_tiene_apoyo_real_hasta_la_base_refinada() {
        cada_pieza_tiene_apoyo_real_hasta_la_base_en(refinado);
    }

    #[test]
    fn cada_pieza_tiene_apoyo_real_hasta_la_base_agua() {
        cada_pieza_tiene_apoyo_real_hasta_la_base_en(refinado_agua);
    }

    /// Una caída nace en el labio de `s`: su remate está a la cota del techo
    /// de `s` y su planta lo toca.
    fn nace_en(caida: &Aabb, s: &Aabb) -> bool {
        let cota = (caida.max.y - s.max.y).abs() <= 0.12;
        let toca_x = caida.min.x >= s.min.x - EPS && caida.max.x <= s.max.x + EPS;
        let toca_z = caida.min.z <= s.max.z + EPS && caida.max.z >= s.max.z - 0.10;
        cota && toca_x && toca_z
    }

    fn la_meseta_son_terrazas_sobre_la_base_y_no_una_pila_en(construir: Construir) {
        let c = construir(entrega());
        let scene = &c.diorama.scene;
        let praderas = indices_de_praderas(scene);
        let roles = roles_completos(&c);
        let raiz = raiz_de(scene, &praderas, &roles);
        let base = caja(scene, praderas[raiz]);

        let mut cotas: Vec<i32> = Vec::new();
        for (k, &i) in praderas.iter().enumerate() {
            if !matches!(roles[k], Rol::Meseta | Rol::Cesped) {
                continue;
            }
            let p = caja(scene, i);
            if roles[k] == Rol::Meseta && k != raiz {
                assert!(
                    apoya_encima(&p, &base),
                    "la terraza {i} no descansa en la base: pila"
                );
            }
            let cota = (p.max.y * 20.0).round() as i32;
            if !cotas.contains(&cota) {
                cotas.push(cota);
            }
        }
        assert!(
            cotas.len() >= 5,
            "solo {} niveles de terreno: {cotas:?}",
            cotas.len()
        );
    }

    #[test]
    fn la_meseta_son_terrazas_sobre_la_base_y_no_una_pila() {
        la_meseta_son_terrazas_sobre_la_base_y_no_una_pila_en(candidato);
    }

    #[test]
    fn la_meseta_son_terrazas_sobre_la_base_y_no_una_pila_refinada() {
        la_meseta_son_terrazas_sobre_la_base_y_no_una_pila_en(refinado);
    }

    #[test]
    fn la_meseta_son_terrazas_sobre_la_base_y_no_una_pila_agua() {
        la_meseta_son_terrazas_sobre_la_base_y_no_una_pila_en(refinado_agua);
    }

    // ------------------------------------------------------------- flores

    /// Pieza delgada y más alta que ancha: un poste.
    fn es_poste(c: &Aabb) -> bool {
        let [x, y, z] = tamano(c);
        y > x.max(z) && x.max(z) < 0.35
    }

    #[test]
    fn la_entrega_actual_tiene_doce_postes_verdes() {
        // Control de la medición: sin esto el test del candidato podría
        // pasar por no medir nada.
        let a = entrega();
        let praderas = indices_de_praderas(&a.scene);
        let postes = roles_actuales()
            .iter()
            .zip(&praderas)
            .filter(|(r, &i)| **r == Rol::Flor && es_poste(&caja(&a.scene, i)))
            .count();
        assert_eq!(postes, 12);
    }

    fn la_vegetacion_baja_es_tapiz_y_acentos_bajos_no_postes_en(construir: Construir) {
        let c = construir(entrega());
        let scene = &c.diorama.scene;
        let praderas = indices_de_praderas(scene);
        let roles = roles_completos(&c);

        let mut acentos = 0;
        for (k, &i) in praderas.iter().enumerate() {
            let p = caja(scene, i);
            if !matches!(roles[k], Rol::Tronco | Rol::Caida) {
                assert!(
                    !es_poste(&p),
                    "pieza {i} ({:?}) es un poste: {p:?}",
                    roles[k]
                );
            }
            if matches!(roles[k], Rol::Acento | Rol::Cesped) {
                let [x, y, z] = tamano(&p);
                assert!(
                    y <= 0.35 && y <= 0.6 * x.min(z),
                    "pieza {i} no es baja: {p:?}"
                );
            }
            acentos += usize::from(roles[k] == Rol::Acento);
        }
        assert!(acentos >= 4, "solo {acentos} acentos bajos");
        assert!(roles.iter().all(|r| *r != Rol::Flor));
    }

    #[test]
    fn la_vegetacion_baja_es_tapiz_y_acentos_bajos_no_postes() {
        la_vegetacion_baja_es_tapiz_y_acentos_bajos_no_postes_en(candidato);
    }

    #[test]
    fn la_vegetacion_baja_es_tapiz_y_acentos_bajos_no_postes_refinada() {
        la_vegetacion_baja_es_tapiz_y_acentos_bajos_no_postes_en(refinado);
    }

    #[test]
    fn la_vegetacion_baja_es_tapiz_y_acentos_bajos_no_postes_agua() {
        la_vegetacion_baja_es_tapiz_y_acentos_bajos_no_postes_en(refinado_agua);
    }

    // ------------------------------------------------------------ cascadas

    fn las_cascadas_son_varias_delgadas_y_distintas_en(construir: Construir) {
        let c = construir(entrega());
        let caidas = caidas(&c);

        assert!((4..=7).contains(&caidas.len()), "{} caidas", caidas.len());

        let alturas: Vec<f32> = caidas.iter().map(|f| tamano(f)[1]).collect();
        let anchos: Vec<f32> = caidas.iter().map(|f| tamano(f)[0]).collect();
        for f in &caidas {
            let [x, y, z] = tamano(f);
            assert!(
                x <= 0.5 && z <= 0.25 && y >= 4.0 * x,
                "caida no delgada: {f:?}"
            );
        }
        let razon = |v: &[f32]| {
            v.iter().cloned().fold(f32::MIN, f32::max) / v.iter().cloned().fold(f32::MAX, f32::min)
        };
        assert!(razon(&alturas) >= 1.6, "alturas poco variadas: {alturas:?}");
        assert!(razon(&anchos) >= 1.4, "anchos poco variados: {anchos:?}");

        for (a, f) in caidas.iter().enumerate() {
            for g in &caidas[a + 1..] {
                let separadas = f.max.x < g.min.x
                    || g.max.x < f.min.x
                    || f.max.y < g.min.y
                    || g.max.y < f.min.y
                    || f.max.z < g.min.z
                    || g.max.z < f.min.z;
                assert!(separadas, "dos caidas se solapan");
            }
        }
    }

    #[test]
    fn las_cascadas_son_varias_delgadas_y_distintas() {
        las_cascadas_son_varias_delgadas_y_distintas_en(candidato);
    }

    #[test]
    fn las_cascadas_son_varias_delgadas_y_distintas_refinada() {
        las_cascadas_son_varias_delgadas_y_distintas_en(refinado);
    }

    #[test]
    fn las_cascadas_son_varias_delgadas_y_distintas_agua() {
        las_cascadas_son_varias_delgadas_y_distintas_en(refinado_agua);
    }

    fn cada_caida_nace_apoyada_y_corre_sobre_roca_oscura_en(construir: Construir) {
        let c = construir(entrega());
        let scene = &c.diorama.scene;
        let praderas = indices_de_praderas(scene);
        let roles = roles_completos(&c);
        let piezas: Vec<(Rol, Aabb)> = roles
            .iter()
            .zip(&praderas)
            .map(|(r, &i)| (*r, caja(scene, i)))
            .collect();

        for f in caidas(&c) {
            let nacimiento = piezas
                .iter()
                .any(|(r, s)| matches!(r, Rol::Meseta | Rol::Cesped) && nace_en(&f, s));
            assert!(nacimiento, "caida sin nacimiento apoyado: {f:?}");

            // Muestreo a lo alto: detrás de la caída hay roca a ras.
            const MUESTRAS: usize = 16;
            let x = (f.min.x + f.max.x) * 0.5;
            let respaldadas = (0..MUESTRAS)
                .filter(|&k| {
                    let y = f.min.y + (k as f32 + 0.5) / MUESTRAS as f32 * (f.max.y - f.min.y);
                    piezas.iter().any(|(r, d)| {
                        r.es_roca()
                            && x >= d.min.x
                            && x <= d.max.x
                            && y >= d.min.y
                            && y <= d.max.y
                            && (-0.10..=0.03).contains(&(f.min.z - d.max.z))
                    })
                })
                .count();
            assert!(
                respaldadas * 10 >= MUESTRAS * 8,
                "caida {f:?} sin frente oscuro detras: {respaldadas}/{MUESTRAS}"
            );
        }
    }

    #[test]
    fn cada_caida_nace_apoyada_y_corre_sobre_roca_oscura() {
        cada_caida_nace_apoyada_y_corre_sobre_roca_oscura_en(candidato);
    }

    #[test]
    fn cada_caida_nace_apoyada_y_corre_sobre_roca_oscura_refinada() {
        cada_caida_nace_apoyada_y_corre_sobre_roca_oscura_en(refinado);
    }

    #[test]
    fn cada_caida_nace_apoyada_y_corre_sobre_roca_oscura_agua() {
        cada_caida_nace_apoyada_y_corre_sobre_roca_oscura_en(refinado_agua);
    }

    /// Píxeles donde la caída es lo primero que se ve, y donde se vería sin
    /// nada delante.
    fn lectura(d: &Blockout, camara: &Camera, objeto: usize) -> (usize, usize, usize, usize) {
        let primitiva = d.scene.objects[objeto].primitive;
        let impactos = trazar(d, camara, ANCHO, ALTO);
        let (mut vistos, mut proyectados) = (0, 0);
        let (mut columnas, mut filas) = (Vec::new(), Vec::new());

        for (p, impacto) in impactos.iter().enumerate() {
            let (x, y) = (p % ANCHO, p / ANCHO);
            if primitiva
                .ray_intersect(&camara.ray_from_pixel(x, y, ANCHO, ALTO))
                .is_some()
            {
                proyectados += 1;
            }
            if impacto.is_some_and(|h| h.object_index == objeto) {
                vistos += 1;
                columnas.push(x);
                filas.push(y);
            }
        }
        let tramo = |v: &[usize]| {
            v.iter()
                .max()
                .map_or(0, |m| m - v.iter().min().unwrap() + 1)
        };
        (vistos, proyectados, tramo(&columnas), tramo(&filas))
    }

    fn cada_caida_se_lee_en_la_hero_a_resolucion_real_en(construir: Construir) {
        let c = construir(entrega());
        let d = &c.diorama;
        let camara = d.hero_camera();
        let praderas = indices_de_praderas(&d.scene);
        let roles = roles_completos(&c);

        for (k, &i) in praderas.iter().enumerate() {
            if roles[k] != Rol::Caida {
                continue;
            }
            let (vistos, proyectados, columnas, filas) = lectura(d, &camara, i);
            assert!(vistos >= 40, "caida {i}: {vistos} px visibles");
            assert!(
                vistos * 10 >= proyectados * 6,
                "caida {i} tapada: {vistos}/{proyectados}"
            );
            assert!(
                columnas >= 2 && filas >= 12,
                "caida {i} ilegible: {columnas}x{filas} px"
            );
        }
    }

    #[test]
    fn cada_caida_se_lee_en_la_hero_a_resolucion_real() {
        cada_caida_se_lee_en_la_hero_a_resolucion_real_en(candidato);
    }

    #[test]
    fn cada_caida_se_lee_en_la_hero_a_resolucion_real_refinada() {
        cada_caida_se_lee_en_la_hero_a_resolucion_real_en(refinado);
    }

    #[test]
    fn cada_caida_se_lee_en_la_hero_a_resolucion_real_agua() {
        cada_caida_se_lee_en_la_hero_a_resolucion_real_en(refinado_agua);
    }

    #[test]
    fn en_la_entrega_actual_la_cascada_queda_tras_el_monolito() {
        let a = entrega();
        let i = indices_de_praderas(&a.scene)[24];
        let (vistos, proyectados, _, _) = lectura(&a, &a.hero_camera(), i);
        assert!(
            vistos * 10 < proyectados * 6,
            "la medicion no ve la oclusion actual: {vistos}/{proyectados}"
        );
    }

    // ------------------------------------------------------------ corredor

    /// Píxeles de decoración de Praderas en el corredor hero: las columnas
    /// del Monolito, ensanchadas `CORREDOR_GRADOS`, por encima de su pie.
    fn decoracion_en_el_corredor(d: &Blockout, roles: &[Rol]) -> Vec<(usize, Rol, usize)> {
        let camara = d.hero_camera();
        let impactos = trazar(d, &camara, ANCHO, ALTO);
        let praderas = indices_de_praderas(&d.scene);
        let monolito: Vec<usize> = impactos
            .iter()
            .enumerate()
            .filter(|(_, h)| {
                h.is_some_and(|h| {
                    d.scene.objects[h.object_index].spatial_group == SpatialGroupId::Monolith
                })
            })
            .map(|(p, _)| p)
            .collect();
        assert!(!monolito.is_empty(), "el Monolito no se ve en la hero");

        let tan = (HALF_VERTICAL_FOV_DEGREES.to_radians()).tan() * ANCHO as f32 / ALTO as f32;
        let fov_h = 2.0 * tan.atan().to_degrees();
        let margen = (CORREDOR_GRADOS / fov_h * ANCHO as f32).ceil() as usize;
        let c0 = monolito
            .iter()
            .map(|p| p % ANCHO)
            .min()
            .unwrap()
            .saturating_sub(margen);
        let c1 = (monolito.iter().map(|p| p % ANCHO).max().unwrap() + margen).min(ANCHO - 1);
        let pie = monolito.iter().map(|p| p / ANCHO).max().unwrap();

        // `(índice, rol, píxeles)` de cada pieza de decoración que asoma.
        let mut invasoras: Vec<(usize, Rol, usize)> = Vec::new();
        let dentro = impactos
            .iter()
            .enumerate()
            .filter(|(p, _)| (c0..=c1).contains(&(p % ANCHO)) && p / ANCHO <= pie)
            .filter_map(|(_, h)| *h)
            .filter_map(|h| praderas.iter().position(|&i| i == h.object_index))
            .filter(|&k| roles[k].es_decoracion());

        for k in dentro {
            match invasoras.iter_mut().find(|(i, _, _)| *i == praderas[k]) {
                Some((_, _, n)) => *n += 1,
                None => invasoras.push((praderas[k], roles[k], 1)),
            }
        }
        invasoras
    }

    #[test]
    fn en_la_entrega_actual_hay_decoracion_en_el_corredor() {
        let a = entrega();
        assert!(!decoracion_en_el_corredor(&a, &roles_actuales()).is_empty());
    }

    fn el_corredor_hero_al_monolito_queda_libre_en(construir: Construir) {
        let c = construir(entrega());
        let roles = roles_completos(&c);
        let invasoras = decoracion_en_el_corredor(&c.diorama, &roles);
        assert!(
            invasoras.is_empty(),
            "decoracion en el corredor: {invasoras:?}"
        );
    }

    #[test]
    fn el_corredor_hero_al_monolito_queda_libre() {
        el_corredor_hero_al_monolito_queda_libre_en(candidato);
    }

    #[test]
    fn el_corredor_hero_al_monolito_queda_libre_refinada() {
        el_corredor_hero_al_monolito_queda_libre_en(refinado);
    }

    #[test]
    fn el_corredor_hero_al_monolito_queda_libre_agua() {
        el_corredor_hero_al_monolito_queda_libre_en(refinado_agua);
    }

    /// Píxeles del Monolito tapados por Praderas desde el lado opuesto a la
    /// hero: (por cualquier pieza, por decoración).
    fn monolito_tapado(d: &Blockout, roles: &[Rol]) -> (usize, usize) {
        let camara = d.camera_at(HERO_YAW_DEGREES + 180.0, EYE_ELEVATION_DEGREES);
        let impactos = trazar(d, &camara, ANCHO, ALTO);
        let praderas = indices_de_praderas(&d.scene);
        let monolito: Vec<usize> = (0..d.scene.objects.len())
            .filter(|&i| d.scene.objects[i].spatial_group == SpatialGroupId::Monolith)
            .collect();
        let (mut tapado, mut por_decoracion) = (0, 0);

        for (p, impacto) in impactos.iter().enumerate() {
            let rayo = camara.ray_from_pixel(p % ANCHO, p / ANCHO, ANCHO, ALTO);
            let t_monolito = monolito
                .iter()
                .filter_map(|&i| d.scene.objects[i].primitive.ray_intersect(&rayo))
                .map(|h| h.distance)
                .fold(f32::INFINITY, f32::min);
            let Some(h) = impacto else { continue };
            if !t_monolito.is_finite() || h.distance >= t_monolito {
                continue;
            }
            if let Some(k) = praderas.iter().position(|&i| i == h.object_index) {
                tapado += 1;
                por_decoracion += usize::from(roles[k].es_decoracion());
            }
        }
        (tapado, por_decoracion)
    }

    fn desde_el_lado_de_praderas_la_decoracion_no_tapa_el_monolito_en(construir: Construir) {
        let a = entrega();
        let c = construir(entrega());
        let roles = roles_completos(&c);
        let (antes, _) = monolito_tapado(&a, &roles_actuales());
        let (despues, por_decoracion) = monolito_tapado(&c.diorama, &roles);

        assert_eq!(por_decoracion, 0, "arboles o caidas tapan el Monolito");
        assert!(
            despues <= antes,
            "Praderas tapa mas Monolito: {despues} contra {antes}"
        );
    }

    #[test]
    fn desde_el_lado_de_praderas_la_decoracion_no_tapa_el_monolito() {
        desde_el_lado_de_praderas_la_decoracion_no_tapa_el_monolito_en(candidato);
    }

    #[test]
    fn desde_el_lado_de_praderas_la_decoracion_no_tapa_el_monolito_refinada() {
        desde_el_lado_de_praderas_la_decoracion_no_tapa_el_monolito_en(refinado);
    }

    #[test]
    fn desde_el_lado_de_praderas_la_decoracion_no_tapa_el_monolito_agua() {
        desde_el_lado_de_praderas_la_decoracion_no_tapa_el_monolito_en(refinado_agua);
    }

    // --------------------------------------------------------------- tapiz

    fn el_tapiz_se_lee_a_resolucion_real_en(construir: Construir) {
        let c = construir(entrega());
        let d = &c.diorama;
        let praderas = indices_de_praderas(&d.scene);
        let roles = roles_completos(&c);
        let pintado = RevealState::painted();

        for (nombre, camara) in [("hero", d.hero_camera()), ("cenital", camara_cenital(d))] {
            let impactos = trazar(d, &camara, ANCHO, ALTO);
            let mut magenta = vec![false; ANCHO * ALTO];
            let (mut tapiz, mut verdes, mut magentas) = (0, 0, 0);

            for (p, impacto) in impactos.iter().enumerate() {
                let Some(h) = impacto else { continue };
                let Some(k) = praderas.iter().position(|&i| i == h.object_index) else {
                    continue;
                };
                if roles[k] != Rol::Cesped {
                    continue;
                }
                let objeto = &d.scene.objects[h.object_index];
                let a = resolve(&d.scene, objeto, &pintado, &h.uv).albedo;
                tapiz += 1;
                if a.r > 1.6 * a.g && a.b > 1.2 * a.g {
                    magentas += 1;
                    magenta[p] = true;
                } else if a.g > a.r && a.g > a.b {
                    verdes += 1;
                }
            }

            let agrupados = (0..ANCHO * ALTO)
                .filter(|&p| magenta[p])
                .filter(|&p| {
                    let (x, y) = (p % ANCHO, p / ANCHO);
                    (x > 0 && magenta[p - 1])
                        || (x + 1 < ANCHO && magenta[p + 1])
                        || (y > 0 && magenta[p - ANCHO])
                        || (y + 1 < ALTO && magenta[p + ANCHO])
                })
                .count();

            assert!(tapiz >= 1500, "{nombre}: solo {tapiz} px de tapiz");
            assert!(
                magentas * 100 >= tapiz * 6 && magentas * 100 <= tapiz * 40,
                "{nombre}: magenta {magentas} de {tapiz}"
            );
            assert!(
                verdes * 100 >= tapiz * 45,
                "{nombre}: verde {verdes} de {tapiz}"
            );
            assert!(
                agrupados * 2 >= magentas,
                "{nombre}: magenta en puntos sueltos"
            );
        }
    }

    #[test]
    fn el_tapiz_se_lee_a_resolucion_real() {
        el_tapiz_se_lee_a_resolucion_real_en(candidato);
    }

    #[test]
    fn el_tapiz_se_lee_a_resolucion_real_refinada() {
        el_tapiz_se_lee_a_resolucion_real_en(refinado);
    }

    #[test]
    fn el_tapiz_se_lee_a_resolucion_real_agua() {
        el_tapiz_se_lee_a_resolucion_real_en(refinado_agua);
    }

    // ----------------------------------------------------------- jerarquia

    fn la_jerarquia_reconstruida_coincide_con_el_oraculo_lineal_en(construir: Construir) {
        let c = construir(entrega());
        let d = &c.diorama;
        let (w, h) = (200, 150);

        for camara in [d.hero_camera(), camara_cenital(d)] {
            let impactos = trazar(d, &camara, w, h);
            for (p, impacto) in impactos.iter().enumerate() {
                let rayo = camara.ray_from_pixel(p % w, p / w, w, h);
                let oraculo = d.scene.intersect(&rayo);
                assert_eq!(
                    impacto.map(|x| x.object_index),
                    oraculo.map(|x| x.object_index),
                    "pixel {p}: la jerarquia no es la de la escena editada"
                );
            }
        }

        let grupo = d
            .accel
            .groups
            .iter()
            .find(|g| g.id == SpatialGroupId::Meadows)
            .expect("grupo de Praderas");
        let praderas = indices_de_praderas(&d.scene);
        let esperado = union(praderas.iter().map(|&i| caja(&d.scene, i))).unwrap();
        assert_eq!(format!("{:?}", grupo.bounds), format!("{esperado:?}"));
    }

    #[test]
    fn la_jerarquia_reconstruida_coincide_con_el_oraculo_lineal() {
        la_jerarquia_reconstruida_coincide_con_el_oraculo_lineal_en(candidato);
    }

    #[test]
    fn la_jerarquia_reconstruida_coincide_con_el_oraculo_lineal_refinada() {
        la_jerarquia_reconstruida_coincide_con_el_oraculo_lineal_en(refinado);
    }

    #[test]
    fn la_jerarquia_reconstruida_coincide_con_el_oraculo_lineal_agua() {
        la_jerarquia_reconstruida_coincide_con_el_oraculo_lineal_en(refinado_agua);
    }

    // ------------------------------------------------------------ refined

    /// El agua vigente de Praderas: el material de la cascada de la entrega.
    fn agua_vigente(a: &Blockout) -> (usize, Material) {
        let i = indices_de_praderas(&a.scene)[24];
        let id = a.scene.objects[i].final_material;
        (id.0, a.scene.material(id))
    }

    #[test]
    fn la_refinada_conserva_la_optica_del_agua_de_praderas() {
        let a = entrega();
        let (id_compartido, agua) = agua_vigente(&a);
        assert!(
            agua.is_reflective_or_refractive(),
            "la cascada vigente es agua"
        );

        let c = refinado(entrega());
        let scene = &c.diorama.scene;
        let roles = roles_completos(&c);
        let mut caidas = 0;

        for (k, i) in indices_de_praderas(scene).into_iter().enumerate() {
            if roles[k] != Rol::Caida {
                continue;
            }
            caidas += 1;
            let id = scene.objects[i].final_material;
            let m = scene.material(id);

            assert!(
                id.0 >= c.paleta_previa,
                "la caida {i} usa el agua compartida"
            );
            assert_eq!(
                m.reflection_cap, agua.reflection_cap,
                "caida {i}: reflexion"
            );
            assert_eq!(
                m.transmission_cap, agua.transmission_cap,
                "caida {i}: transmision"
            );
            assert_eq!(m.ior, agua.ior, "caida {i}: ior");
            assert_eq!(m.shadow_mode, agua.shadow_mode, "caida {i}: sombras");
            let t = m.albedo_texture.expect("textura procedural");
            assert!(
                t.0 >= c.texturas_previas,
                "caida {i} con textura compartida"
            );
        }

        assert_eq!(caidas, 6, "refined mantiene las seis caidas");
        assert_eq!(
            format!("{:?}", scene.palette[id_compartido]),
            format!("{agua:?}"),
            "el agua compartida no se toca"
        );
    }

    /// Rayos refractados que lanza el píxel más central de cada caída en la
    /// hero, trazado con el renderer real.
    fn refracciones_por_caida(c: &Candidato) -> Vec<(usize, usize)> {
        let d = &c.diorama;
        let camara = d.hero_camera();
        let luces = luces_del_diorama(&d.anchors, &d.scale);
        let impactos = trazar(d, &camara, ANCHO, ALTO);
        let praderas = indices_de_praderas(&d.scene);
        let roles = roles_completos(c);

        (0..praderas.len())
            .filter(|&k| roles[k] == Rol::Caida)
            .map(|k| {
                let i = praderas[k];
                let suyos: Vec<usize> = (0..impactos.len())
                    .filter(|&p| impactos[p].is_some_and(|h| h.object_index == i))
                    .collect();
                assert!(!suyos.is_empty(), "la caida {i} no se ve en la hero");
                let p = suyos[suyos.len() / 2];
                let rayo = camara.ray_from_pixel(p % ANCHO, p / ANCHO, ANCHO, ALTO);
                let mut stats = TraversalStats::default();
                cast_ray(
                    &rayo,
                    &d.scene,
                    &d.accel,
                    &luces,
                    &RevealState::painted(),
                    Shading::Material,
                    &mut stats,
                );
                (i, stats.refraction_rays)
            })
            .collect()
    }

    #[test]
    fn las_caidas_refinadas_trazan_refraccion_real() {
        let c = refinado(entrega());
        for (i, rayos) in refracciones_por_caida(&c) {
            assert!(rayos > 0, "la caida {i} no refracta: es opaca");
        }
    }

    #[test]
    fn en_la_candidata_original_las_caidas_son_opacas() {
        // Control de la medición anterior: la primera candidata perdió la
        // óptica, y el test tiene que verlo.
        let c = candidato(entrega());
        for (i, rayos) in refracciones_por_caida(&c) {
            assert_eq!(rayos, 0, "la caida {i} de la candidata ya refractaba");
        }
    }

    /// FNV-1a de 64 bits: una huella estable sin dependencias.
    fn huella(textos: &[String]) -> u64 {
        let mut h: u64 = 0xCBF2_9CE4_8422_2325;
        for t in textos {
            for b in t.bytes() {
                h ^= u64::from(b);
                h = h.wrapping_mul(0x0100_0000_01B3);
            }
        }
        h
    }

    /// La huella de la candidata aprobada: piezas de Praderas, materiales y
    /// texturas nuevos. Refined no puede moverla.
    const HUELLA_CANDIDATA: u64 = 9_137_066_942_943_242_934;

    #[test]
    fn la_candidata_original_no_cambia() {
        let c = candidato(entrega());
        let scene = &c.diorama.scene;
        let mut textos: Vec<String> = indices_de_praderas(scene)
            .into_iter()
            .map(|i| format!("{:?}", scene.objects[i]))
            .collect();
        textos.push(format!("{:?}", &scene.palette[c.paleta_previa..]));
        for t in &scene.textures[c.texturas_previas..] {
            textos.extend(texeles(t));
        }

        assert_eq!(
            huella(&textos),
            HUELLA_CANDIDATA,
            "la candidata aprobada cambio"
        );
    }

    // ----------------------------------------------- refined_water_polish

    /// Huella de una variante: piezas de Praderas, materiales y texturas
    /// nuevos. La misma receta que `la_candidata_original_no_cambia`.
    fn huella_de(c: &Candidato) -> u64 {
        let scene = &c.diorama.scene;
        let mut textos: Vec<String> = indices_de_praderas(scene)
            .into_iter()
            .map(|i| format!("{:?}", scene.objects[i]))
            .collect();
        textos.push(format!("{:?}", &scene.palette[c.paleta_previa..]));
        for t in &scene.textures[c.texturas_previas..] {
            textos.extend(texeles(t));
        }
        huella(&textos)
    }

    /// La huella de `refined` tal y como se aprobó: tapiz, terrazas,
    /// caídas y agua. El pulido del agua no puede moverla.
    const HUELLA_REFINADA: u64 = 17_242_488_278_132_778_984;

    #[test]
    fn la_refinada_aprobada_no_cambia() {
        assert_eq!(
            huella_de(&refinado(entrega())),
            HUELLA_REFINADA,
            "refined aprobada cambio"
        );
    }

    /// Los dos materiales de agua locales de una variante.
    fn aguas(c: &Candidato) -> [MaterialId; 2] {
        [c.materiales.caida, c.materiales.caida_fina]
    }

    #[test]
    fn el_pulido_de_agua_solo_cambia_el_agua() {
        let r = refinado(entrega());
        let w = refinado_agua(entrega());
        let (a, b) = (&r.diorama, &w.diorama);

        // Geometría, material por objeto y grupos de los 168: idénticos.
        // Incluye las seis caídas, el tapiz y las terrazas.
        assert_eq!(
            format!("{:?}", a.scene.objects),
            format!("{:?}", b.scene.objects),
            "el pulido movio geometria o reasigno materiales"
        );
        assert_eq!(format!("{:?}", r.materiales), format!("{:?}", w.materiales));
        assert_eq!(a.scene.palette.len(), b.scene.palette.len());
        assert_eq!(a.scene.textures.len(), b.scene.textures.len());

        let agua = aguas(&r);
        let texturas_del_agua: Vec<usize> = agua
            .iter()
            .filter_map(|&id| a.scene.material(id).albedo_texture)
            .map(|t| t.0)
            .collect();

        for (i, (x, y)) in a.scene.palette.iter().zip(&b.scene.palette).enumerate() {
            if agua.iter().any(|id| id.0 == i) {
                continue;
            }
            assert_eq!(
                format!("{x:?}"),
                format!("{y:?}"),
                "material {i} ajeno al agua cambio"
            );
        }
        for (i, (x, y)) in a.scene.textures.iter().zip(&b.scene.textures).enumerate() {
            if texturas_del_agua.contains(&i) {
                continue;
            }
            assert!(texeles(x) == texeles(y), "textura {i} ajena al agua cambio");
        }

        assert_eq!(format!("{:?}", a.anchors), format!("{:?}", b.anchors));
        assert_eq!(format!("{:?}", a.scale), format!("{:?}", b.scale));
        assert_eq!(format!("{:?}", a.accel), format!("{:?}", b.accel));
        assert_eq!(
            format!("{:?}", a.scene.skybox),
            format!("{:?}", b.scene.skybox)
        );
    }

    #[test]
    fn el_pulido_conserva_la_optica_y_ajusta_solo_el_agua_local() {
        let e = entrega();
        let (id_compartido, agua) = agua_vigente(&e);
        let r = refinado(entrega());
        let w = refinado_agua(entrega());
        let mut ajustada = false;

        for (&id_r, &id_w) in aguas(&r).iter().zip(&aguas(&w)) {
            let mr = r.diorama.scene.material(id_r);
            let mw = w.diorama.scene.material(id_w);

            assert!(
                id_w.0 >= w.paleta_previa,
                "el pulido usa el agua compartida"
            );
            assert_eq!(mw.reflection_cap, agua.reflection_cap, "reflexion");
            assert_eq!(mw.transmission_cap, agua.transmission_cap, "transmision");
            assert_eq!(mw.ior, agua.ior, "ior");
            assert_eq!(mw.shadow_mode, agua.shadow_mode, "sombras");
            assert!(mw.is_valid(), "material de agua fuera de rango");

            let tw = mw.albedo_texture.expect("textura procedural");
            let tr = mr.albedo_texture.expect("textura procedural");
            assert!(tw.0 >= w.texturas_previas, "agua con textura compartida");

            ajustada |= mw.specular_strength != mr.specular_strength
                || mw.shininess != mr.shininess
                || mw.albedo != mr.albedo
                || texeles(w.diorama.scene.texture(tw)) != texeles(r.diorama.scene.texture(tr));
        }

        assert!(ajustada, "el pulido no ajusta el agua local");
        assert_eq!(
            format!("{:?}", w.diorama.scene.palette[id_compartido]),
            format!("{agua:?}"),
            "el agua compartida no se toca"
        );
    }

    #[test]
    fn las_caidas_del_pulido_trazan_refraccion_real() {
        let c = refinado_agua(entrega());
        for (i, rayos) in refracciones_por_caida(&c) {
            assert!(rayos > 0, "la caida {i} no refracta: es opaca");
        }
    }

    /// Fracción de los píxeles de cada caída que la hero **quema**: los
    /// tres canales a `250` o más.
    fn caidas_quemadas(c: &Candidato) -> Vec<(usize, f32)> {
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
        let impactos = trazar(d, &camara, ANCHO, ALTO);
        let praderas = indices_de_praderas(&d.scene);
        let roles = roles_completos(c);

        (0..praderas.len())
            .filter(|&k| roles[k] == Rol::Caida)
            .map(|k| {
                let suyos: Vec<usize> = (0..impactos.len())
                    .filter(|&p| impactos[p].is_some_and(|h| h.object_index == praderas[k]))
                    .collect();
                let quemados = suyos
                    .iter()
                    .filter(|&&p| {
                        [16, 8, 0]
                            .iter()
                            .all(|&s| (fb.buffer[p] >> s) & 0xFF >= 250)
                    })
                    .count();
                (praderas[k], quemados as f32 / suyos.len().max(1) as f32)
            })
            .collect()
    }

    #[test]
    fn el_pulido_no_quema_las_caidas() {
        let c = refinado_agua(entrega());
        for (i, fraccion) in caidas_quemadas(&c) {
            assert!(
                fraccion <= 0.01,
                "la caida {i} quema {:.1} % de sus pixeles",
                100.0 * fraccion
            );
        }
    }

    // --------------------------------------------------------- produccion

    /// La entrega de producción es, byte a byte, la variante aprobada:
    /// mismos objetos, misma paleta, mismas texturas texel a texel, mismas
    /// anclas, escala, jerarquía y cielo. Con los assets reales.
    #[test]
    fn la_entrega_de_produccion_es_el_pulido_de_agua_aprobado() {
        use expedition33_continente_inacabado::scenes::{
            delivery_level_previo_aguas_con, WaterPreset,
        };

        let aprobada = refinado_agua(entrega());
        let a = &aprobada.diorama;
        // La etapa de Praderas: desde la promoción de Aguas, `delivery_level_con`
        // reescribe además la bahía, y eso lo comprueba su propio preview.
        let b = delivery_level_previo_aguas_con(WaterPreset::RefractiveWater, Some(&raiz()))
            .expect("los assets reales estan en el repositorio");

        assert_eq!(
            format!("{:?}", a.scene.objects),
            format!("{:?}", b.scene.objects)
        );
        assert_eq!(
            format!("{:?}", a.scene.palette),
            format!("{:?}", b.scene.palette)
        );
        assert_eq!(a.scene.textures.len(), b.scene.textures.len());
        for (i, (x, y)) in a.scene.textures.iter().zip(&b.scene.textures).enumerate() {
            assert!(texeles(x) == texeles(y), "textura {i} distinta");
        }
        assert_eq!(
            format!("{:?}", a.scene.skybox),
            format!("{:?}", b.scene.skybox)
        );
        assert_eq!(format!("{:?}", a.anchors), format!("{:?}", b.anchors));
        assert_eq!(format!("{:?}", a.scale), format!("{:?}", b.scale));
        assert_eq!(format!("{:?}", a.accel), format!("{:?}", b.accel));
        assert_eq!(a.hero_preset(), b.hero_preset());
        assert_eq!(
            format!("{:?}", camara_cenital(a)),
            format!("{:?}", camara_cenital(&b))
        );
    }
}
