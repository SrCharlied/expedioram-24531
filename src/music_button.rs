//! La bocina: el único control de la música.
//!
//! Siempre visible, en la fila de la cápsula de la paleta y a su izquierda,
//! de modo que no tapa el panel desplegado. Muestra un altavoz con ondas y
//! `ON` mientras suena, tachado y `OFF` en silencio, y apagado con `N/A`
//! si no hay música. Se dibuja opaca sobre su rectángulo completo, así que
//! repintarla sobre un cuadro reutilizado (`FramePlan::Reuse`) no acumula
//! nada y alternar no obliga a trazar la escena.
//!
//! No depende de `brush_palette`, que solo existe con `artistic-brush`: la
//! ruta clásica también tiene bocina. Por eso trae sus cinco glifos.

use crate::framebuffer::Framebuffer;
use crate::music::EstadoMusica;

/// Un rectángulo en píxeles del **buffer**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zona {
    pub x: usize,
    pub y: usize,
    pub ancho: usize,
    pub alto: usize,
}

impl Zona {
    /// Si el cursor, en píxeles del buffer, cae dentro.
    pub fn contiene(&self, cursor: (f32, f32)) -> bool {
        if !cursor.0.is_finite() || !cursor.1.is_finite() || cursor.0 < 0.0 || cursor.1 < 0.0 {
            return false;
        }

        // Igual que `brush_palette::Rect`: el píxel bajo el cursor.
        let (x, y) = (cursor.0 as usize, cursor.1 as usize);

        x >= self.x && y >= self.y && x < self.x + self.ancho && y < self.y + self.alto
    }

    pub fn se_solapa_con(&self, otra: &Zona) -> bool {
        self.x < otra.x + otra.ancho
            && otra.x < self.x + self.ancho
            && self.y < otra.y + otra.alto
            && otra.y < self.y + self.alto
    }
}

/// Tamaño del botón.
pub const ANCHO_DEL_BOTON: usize = 72;
pub const ALTO_DEL_BOTON: usize = 26;

/// Separación con la cápsula, y margen con el borde si no hay cápsula.
pub const SEPARACION: usize = 8;
pub const MARGEN: usize = 12;

/// Dónde va la bocina. Con cápsula —la paleta del pincel—, a su izquierda
/// y en su misma fila; sin ella —la ruta clásica—, abajo a la derecha.
pub fn colocar(ancho: usize, alto: usize, capsula: Option<Zona>) -> Zona {
    match capsula {
        Some(c) => Zona {
            x: c.x.saturating_sub(SEPARACION + ANCHO_DEL_BOTON),
            y: c.y,
            ancho: ANCHO_DEL_BOTON.min(ancho),
            alto: c.alto.min(alto),
        },
        None => Zona {
            x: ancho.saturating_sub(MARGEN + ANCHO_DEL_BOTON),
            y: alto.saturating_sub(MARGEN + ALTO_DEL_BOTON),
            ancho: ANCHO_DEL_BOTON.min(ancho),
            alto: ALTO_DEL_BOTON.min(alto),
        },
    }
}

/// Qué hace el botón izquierdo con la bocina en este cuadro.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Respuesta {
    /// Alternar la música: solo en el flanco de bajada sobre la bocina.
    pub alternar: bool,
    /// El clic o el arrastre es de la bocina: ni la paleta, ni el pincel, ni
    /// el picking por región lo ven.
    pub consumir: bool,
}

/// El botón izquierdo frente a la bocina, entre cuadros.
///
/// Un clic que **empieza** sobre ella la alterna una vez y se queda con el
/// arrastre hasta soltar, salga o no del rectángulo: no se puede alternar y
/// además empezar un trazo. Un arrastre de pincel que **pasa** por encima no
/// la alterna, pero tampoco pinta bajo ella, igual que sobre el panel.
#[derive(Debug, Clone, Copy, Default)]
pub struct Pulsacion {
    anterior: bool,
    capturada: bool,
}

impl Pulsacion {
    /// `boton` es el estado del botón izquierdo este cuadro y `cursor` el
    /// puntero en píxeles del **buffer** (`None` fuera del cuadro).
    pub fn procesar(&mut self, boton: bool, cursor: Option<(f32, f32)>, zona: Zona) -> Respuesta {
        let flanco = boton && !self.anterior;
        self.anterior = boton;

        if !boton {
            self.capturada = false;
            return Respuesta::default();
        }

        let sobre = cursor.is_some_and(|c| zona.contiene(c));

        if flanco && sobre {
            self.capturada = true;
            return Respuesta {
                alternar: true,
                consumir: true,
            };
        }

        Respuesta {
            alternar: false,
            consumir: self.capturada || sobre,
        }
    }
}

/// La etiqueta que acompaña al altavoz.
pub fn etiqueta(estado: EstadoMusica) -> &'static str {
    match estado {
        EstadoMusica::Sonando => "ON",
        EstadoMusica::Silenciada => "OFF",
        EstadoMusica::NoDisponible => "N/A",
    }
}

// ----------------------------------------------------------------- dibujo

/// Los mismos tonos que la cápsula de la paleta, para que se lean como una
/// familia. Se repiten aquí porque `brush_palette` no existe en la ruta
/// clásica.
const FONDO: u32 = 0x00272231;
const BORDE: u32 = 0x00C8B489;
const TINTA: u32 = 0x00E8DFC8;
/// Lo apagado: sin música, todo el botón baja de tono.
const BORDE_APAGADO: u32 = 0x005A5466;
const TINTA_APAGADA: u32 = 0x00807A8A;
/// El tachado del silencio: un rojo cálido que se lee a `800 x 600`.
const TACHADO: u32 = 0x00E0705A;

const ANCHO_GLIFO: usize = 5;
const ALTO_GLIFO: usize = 7;
const AVANCE: usize = ANCHO_GLIFO + 1;
const ESCALA: usize = 2;

/// Siete filas de cinco bits, el más significativo a la izquierda, como los
/// glifos de la paleta. Solo los que necesitan las tres etiquetas.
fn glifo(letra: char) -> Option<[u8; ALTO_GLIFO]> {
    Some(match letra {
        'O' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'N' => [
            0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001,
        ],
        'F' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'A' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        '/' => [
            0b00001, 0b00010, 0b00010, 0b00100, 0b01000, 0b01000, 0b10000,
        ],
        _ => return None,
    })
}

fn texto(framebuffer: &mut Framebuffer, x: usize, y: usize, texto: &str, color: u32) {
    for (i, letra) in texto.chars().enumerate() {
        let Some(trazo) = glifo(letra) else {
            continue;
        };
        let x0 = x + i * AVANCE * ESCALA;
        for (fila, bits) in trazo.iter().enumerate() {
            for columna in 0..ANCHO_GLIFO {
                if bits & (1 << (ANCHO_GLIFO - 1 - columna)) != 0 {
                    framebuffer.fill_rect(
                        x0 + columna * ESCALA,
                        y + fila * ESCALA,
                        ESCALA,
                        ESCALA,
                        color,
                    );
                }
            }
        }
    }
}

/// Dibuja la bocina, opaca sobre toda su zona.
///
/// El altavoz es un cuerpo y un cono escalonado; sonando lleva dos ondas, y
/// en silencio o sin música, una diagonal que lo tacha. Todo cae dentro de
/// la zona de `72 x 26`; en una zona más pequeña solo se pinta el fondo.
pub fn dibujar_boton(framebuffer: &mut Framebuffer, zona: Zona, estado: EstadoMusica) {
    let (borde, tinta) = match estado {
        EstadoMusica::NoDisponible => (BORDE_APAGADO, TINTA_APAGADA),
        _ => (BORDE, TINTA),
    };

    framebuffer.fill_rect(zona.x, zona.y, zona.ancho, zona.alto, FONDO);
    framebuffer.stroke_rect(zona.x, zona.y, zona.ancho, zona.alto, borde);

    if zona.ancho < ANCHO_DEL_BOTON || zona.alto < ALTO_DEL_BOTON {
        return;
    }

    // El altavoz, centrado en la vertical.
    let x0 = zona.x + 7;
    let cy = zona.y + zona.alto / 2;

    framebuffer.fill_rect(x0, cy - 3, 4, 6, tinta);
    for i in 0..6 {
        let medio = 3 + i;
        framebuffer.fill_rect(x0 + 4 + i, cy - medio, 1, 2 * medio, tinta);
    }

    match estado {
        EstadoMusica::Sonando => {
            // Dos ondas: un arco corto y otro largo.
            framebuffer.fill_rect(x0 + 12, cy - 3, 1, 6, tinta);
            framebuffer.fill_rect(x0 + 11, cy - 4, 1, 1, tinta);
            framebuffer.fill_rect(x0 + 11, cy + 3, 1, 1, tinta);
            framebuffer.fill_rect(x0 + 15, cy - 5, 1, 10, tinta);
            framebuffer.fill_rect(x0 + 14, cy - 7, 1, 2, tinta);
            framebuffer.fill_rect(x0 + 14, cy + 5, 1, 2, tinta);
        }
        EstadoMusica::Silenciada | EstadoMusica::NoDisponible => {
            let color = match estado {
                EstadoMusica::Silenciada => TACHADO,
                _ => tinta,
            };
            // Diagonal de abajo a la izquierda a arriba a la derecha, de dos
            // píxeles de grueso.
            for paso in 0..16 {
                framebuffer.fill_rect(x0 - 1 + paso, cy + 8 - paso, 2, 2, color);
            }
        }
    }

    let alto_texto = ALTO_GLIFO * ESCALA;
    texto(
        framebuffer,
        zona.x + 30,
        zona.y + (zona.alto - alto_texto) / 2,
        etiqueta(estado),
        tinta,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const ESTADOS: [EstadoMusica; 3] = [
        EstadoMusica::Sonando,
        EstadoMusica::Silenciada,
        EstadoMusica::NoDisponible,
    ];

    #[test]
    fn las_etiquetas_dicen_el_estado() {
        assert_eq!(etiqueta(EstadoMusica::Sonando), "ON");
        assert_eq!(etiqueta(EstadoMusica::Silenciada), "OFF");
        assert_eq!(etiqueta(EstadoMusica::NoDisponible), "N/A");
    }

    #[test]
    fn el_rectangulo_consume_entero_y_nada_mas() {
        let z = Zona {
            x: 100,
            y: 50,
            ancho: ANCHO_DEL_BOTON,
            alto: ALTO_DEL_BOTON,
        };
        for c in [
            (100.0, 50.0),
            (171.9, 75.9),
            (135.0, 60.0),
            (100.0, 75.0),
            (171.0, 50.0),
        ] {
            assert!(z.contiene(c), "{c:?}");
        }
        for c in [
            (99.9, 60.0),
            (172.0, 60.0),
            (130.0, 49.9),
            (130.0, 76.0),
            (-5.0, 60.0),
        ] {
            assert!(!z.contiene(c), "{c:?}");
        }
        assert!(!z.contiene((f32::NAN, 60.0)));
        assert!(!z.contiene((130.0, f32::INFINITY)));
    }

    #[test]
    fn sin_capsula_va_abajo_a_la_derecha() {
        let z = colocar(800, 600, None);
        assert_eq!(
            z,
            Zona {
                x: 800 - MARGEN - ANCHO_DEL_BOTON,
                y: 600 - MARGEN - ALTO_DEL_BOTON,
                ancho: ANCHO_DEL_BOTON,
                alto: ALTO_DEL_BOTON,
            }
        );
    }

    #[test]
    fn con_capsula_va_a_su_izquierda_en_la_misma_fila() {
        let capsula = Zona {
            x: 660,
            y: 562,
            ancho: 128,
            alto: 26,
        };
        let z = colocar(800, 600, Some(capsula));
        assert_eq!(z.x + z.ancho + SEPARACION, capsula.x);
        assert_eq!(z.y, capsula.y);
        assert_eq!(z.alto, capsula.alto);
        assert!(!z.se_solapa_con(&capsula));
    }

    /// Opaca sobre toda la zona: dibujarla dos veces sobre el mismo cuadro,
    /// como hace `FramePlan::Reuse`, da los mismos píxeles, y fuera de la
    /// zona no toca nada.
    #[test]
    fn se_dibuja_opaca_idempotente_y_solo_dentro_de_su_zona() {
        let z = Zona {
            x: 40,
            y: 30,
            ancho: ANCHO_DEL_BOTON,
            alto: ALTO_DEL_BOTON,
        };
        for estado in ESTADOS {
            for fondo in [0x00000000u32, 0x00FFFFFF, 0x00336699] {
                let mut fb = Framebuffer::new(160, 90);
                fb.buffer.fill(fondo);
                dibujar_boton(&mut fb, z, estado);
                let una = fb.buffer.clone();
                dibujar_boton(&mut fb, z, estado);
                assert_eq!(una, fb.buffer, "{estado:?}");
                for y in 0..90 {
                    for x in 0..160 {
                        let dentro = z.contiene((x as f32, y as f32));
                        if !dentro {
                            assert_eq!(fb.buffer[y * 160 + x], fondo, "{estado:?} {x} {y}");
                        }
                    }
                }
            }
            // Opaca: el resultado dentro no depende del fondo.
            let pinta = |fondo: u32| {
                let mut fb = Framebuffer::new(160, 90);
                fb.buffer.fill(fondo);
                dibujar_boton(&mut fb, z, estado);
                let mut dentro = Vec::new();
                for y in z.y..z.y + z.alto {
                    dentro.extend_from_slice(&fb.buffer[y * 160 + z.x..y * 160 + z.x + z.ancho]);
                }
                dentro
            };
            assert_eq!(pinta(0x00000000), pinta(0x00FFFFFF), "{estado:?}");
        }
    }

    /// Los tres estados se distinguen a simple vista: sus dibujos difieren.
    #[test]
    fn los_tres_estados_se_ven_distintos() {
        let z = Zona {
            x: 0,
            y: 0,
            ancho: ANCHO_DEL_BOTON,
            alto: ALTO_DEL_BOTON,
        };
        let pinta = |estado| {
            let mut fb = Framebuffer::new(ANCHO_DEL_BOTON, ALTO_DEL_BOTON);
            dibujar_boton(&mut fb, z, estado);
            fb.buffer
        };
        let (a, b, c) = (
            pinta(EstadoMusica::Sonando),
            pinta(EstadoMusica::Silenciada),
            pinta(EstadoMusica::NoDisponible),
        );
        assert_ne!(a, b);
        assert_ne!(b, c);
        assert_ne!(a, c);
        // Y hay tinta: no es un rectángulo liso.
        let distintos = |v: &[u32]| {
            let mut d: Vec<u32> = v.to_vec();
            d.sort_unstable();
            d.dedup();
            d.len()
        };
        for v in [&a, &b, &c] {
            assert!(distintos(v) >= 3);
        }
    }

    /// La bocina nunca se solapa con la paleta, abierta o plegada, en
    /// cualquier tamaño de buffer en el que el panel cabe sobre la cápsula.
    #[cfg(feature = "artistic-brush")]
    #[test]
    fn no_se_solapa_con_la_paleta_abierta_ni_plegada() {
        use crate::brush_palette::{Disposicion, Rect, TEXTURAS};
        let a_zona = |r: Rect| Zona {
            x: r.x,
            y: r.y,
            ancho: r.ancho,
            alto: r.alto,
        };
        let telas = vec![true; TEXTURAS.len()];
        let mut probados = 0;
        for ancho in (300..=1600).step_by(13) {
            for alto in (200..=1200).step_by(11) {
                for abierta in [false, true] {
                    let d = Disposicion::calcular(abierta, ancho, alto, &telas);
                    let z = colocar(ancho, alto, Some(a_zona(d.capsula)));
                    assert!(
                        z.x + z.ancho <= ancho && z.y + z.alto <= alto,
                        "{ancho}x{alto}"
                    );
                    assert!(
                        !z.se_solapa_con(&a_zona(d.capsula)),
                        "{ancho}x{alto} {abierta}"
                    );
                    if let Some(panel) = d.panel {
                        assert!(!z.se_solapa_con(&a_zona(panel)), "{ancho}x{alto} {abierta}");
                    }
                    for celda in &d.celdas {
                        assert!(!z.se_solapa_con(&a_zona(celda.rect)), "{ancho}x{alto}");
                    }
                    // Lo que consume la bocina no lo consume la paleta, y
                    // al revés: ningún punto de la bocina es de la paleta.
                    for (dx, dy) in [
                        (0, 0),
                        (z.ancho - 1, 0),
                        (0, z.alto - 1),
                        (z.ancho - 1, z.alto - 1),
                    ] {
                        let p = ((z.x + dx) as f32 + 0.5, (z.y + dy) as f32 + 0.5);
                        assert_eq!(d.impacto(p), crate::brush_palette::Impacto::Fuera);
                    }
                    probados += 1;
                }
            }
        }
        assert!(probados > 1000);
        // Y a 800 x 600, el tamaño del cuadro de la obra.
        for abierta in [false, true] {
            let d = Disposicion::calcular(abierta, 800, 600, &telas);
            let z = colocar(800, 600, Some(a_zona(d.capsula)));
            assert!(!z.se_solapa_con(&a_zona(d.capsula)));
            if let Some(panel) = d.panel {
                assert!(!z.se_solapa_con(&a_zona(panel)));
            }
        }
    }

    // ------------------------------------------------ enrutado del clic

    const Z: Zona = Zona {
        x: 100,
        y: 100,
        ancho: ANCHO_DEL_BOTON,
        alto: ALTO_DEL_BOTON,
    };
    const SOBRE: Option<(f32, f32)> = Some((120.0, 110.0));
    const FUERA: Option<(f32, f32)> = Some((400.0, 300.0));

    #[test]
    fn un_clic_sobre_la_bocina_alterna_una_vez_y_consume_hasta_soltar() {
        let mut p = Pulsacion::default();
        assert_eq!(
            p.procesar(false, SOBRE, Z),
            Respuesta {
                alternar: false,
                consumir: false
            }
        );
        // Flanco: alterna y consume.
        assert_eq!(
            p.procesar(true, SOBRE, Z),
            Respuesta {
                alternar: true,
                consumir: true
            }
        );
        // Sostener no vuelve a alternar, y sigue siendo de la bocina.
        assert_eq!(
            p.procesar(true, SOBRE, Z),
            Respuesta {
                alternar: false,
                consumir: true
            }
        );
        // Arrastrar fuera tampoco pinta: el arrastre es de la bocina.
        assert_eq!(
            p.procesar(true, FUERA, Z),
            Respuesta {
                alternar: false,
                consumir: true
            }
        );
        assert_eq!(
            p.procesar(true, None, Z),
            Respuesta {
                alternar: false,
                consumir: true
            }
        );
        // Soltar libera.
        assert_eq!(
            p.procesar(false, FUERA, Z),
            Respuesta {
                alternar: false,
                consumir: false
            }
        );
        assert_eq!(
            p.procesar(true, FUERA, Z),
            Respuesta {
                alternar: false,
                consumir: false
            }
        );
    }

    #[test]
    fn un_trazo_que_pasa_por_encima_no_la_alterna_ni_pinta_bajo_ella() {
        let mut p = Pulsacion::default();
        assert_eq!(
            p.procesar(true, FUERA, Z),
            Respuesta {
                alternar: false,
                consumir: false
            }
        );
        assert_eq!(
            p.procesar(true, SOBRE, Z),
            Respuesta {
                alternar: false,
                consumir: true
            }
        );
        assert_eq!(
            p.procesar(true, FUERA, Z),
            Respuesta {
                alternar: false,
                consumir: false
            }
        );
        assert_eq!(
            p.procesar(false, SOBRE, Z),
            Respuesta {
                alternar: false,
                consumir: false
            }
        );
    }

    #[test]
    fn dos_clics_alternan_dos_veces_y_un_clic_fuera_del_cuadro_no_hace_nada() {
        let mut p = Pulsacion::default();
        let mut veces = 0;
        for (boton, cursor) in [(true, SOBRE), (false, SOBRE), (true, SOBRE), (false, SOBRE)] {
            veces += p.procesar(boton, cursor, Z).alternar as u32;
        }
        assert_eq!(veces, 2);
        let mut p = Pulsacion::default();
        assert_eq!(
            p.procesar(true, None, Z),
            Respuesta {
                alternar: false,
                consumir: false
            }
        );
        // Entrar con el botón ya abajo no es un clic.
        assert_eq!(
            p.procesar(true, SOBRE, Z),
            Respuesta {
                alternar: false,
                consumir: true
            }
        );
    }

    /// El clic llega de la ventana estirada: el punto de cliente se lleva
    /// al buffer con `viewport::al_buffer`, y entonces la bocina responde
    /// en el mismo sitio en que se ve, también con bandas.
    #[test]
    fn la_bocina_responde_donde_se_ve_en_la_ventana_estirada() {
        use crate::viewport::{al_buffer, encuadre};
        let z = colocar(
            800,
            600,
            Some(Zona {
                x: 660,
                y: 562,
                ancho: 128,
                alto: 26,
            }),
        );
        for v in [
            (800, 600),
            (1200, 900),
            (1600, 900),
            (800, 900),
            (1000, 1000),
            (643, 517),
        ] {
            let e = encuadre(v, (800, 600)).unwrap();
            let a_cliente = |bx: f32, by: f32| {
                (
                    e.x as f32 + (bx + 0.5) * e.ancho as f32 / 800.0,
                    e.y as f32 + (by + 0.5) * e.alto as f32 / 600.0,
                )
            };
            let centro = a_cliente(
                z.x as f32 + z.ancho as f32 / 2.0,
                z.y as f32 + z.alto as f32 / 2.0,
            );
            let p = al_buffer(centro, v, (800, 600));
            assert!(p.is_some_and(|p| z.contiene(p)), "{v:?} {centro:?} {p:?}");
            let mut pulsacion = Pulsacion::default();
            assert!(pulsacion.procesar(true, p, z).alternar, "{v:?}");
            // Justo fuera del borde izquierdo de la bocina, en cliente, ya no.
            let fuera = a_cliente(z.x as f32 - 2.0, z.y as f32 + 5.0);
            let q = al_buffer(fuera, v, (800, 600));
            assert!(!q.is_some_and(|q| z.contiene(q)), "{v:?} {q:?}");
        }
    }
}
