//! Paleta del pincel: la única fuente de las herramientas y su interfaz.
//!
//! Aquí viven las definiciones de los pigmentos y de las telas, el estado de
//! la paleta y su disposición en pantalla. `main.rs` no declara ninguna
//! tabla propia: traduce teclas de `minifb` a las que este módulo nombra, y
//! consulta lo mismo que dibuja la interfaz. Con dos tablas —una para el
//! teclado y otra para la UI— bastaría un retoque en una para que el botón
//! prometiera un color y la tecla diera otro.
//!
//! # Por qué no aparece `minifb`
//!
//! La misma razón que sostiene el resto de la librería: lo que está en
//! `lib.rs` tiene que poder probarse sin abrir una ventana. Las teclas se
//! nombran con un `char`, el cursor llega como un par de `f32` y la
//! disposición se calcula en píxeles. El binario traduce; aquí se decide.

use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::texture::Texture;

/// La tecla que pliega y despliega la paleta.
pub const TECLA_DE_LA_PALETA: char = 'p';

/// La tecla que vuelve a la herramienta de revelar.
const TECLA_DE_REVELAR: char = 'q';

/// Un pigmento plano de la paleta.
pub struct Pigmento {
    pub nombre: &'static str,
    /// Tecla que lo activa, en minúscula.
    pub tecla: char,
    /// Color en **sRGB**, que es como se eligen mirando una pantalla.
    pub srgb: [f32; 3],
}

impl Pigmento {
    /// Color lineal, que es donde se compone.
    ///
    /// El **único** punto donde la paleta cruza de sRGB a lineal.
    pub fn color(&self) -> Color {
        let [r, g, b] = self.srgb;

        Color::from_srgb(r, g, b)
    }
}

/// Una tela de la paleta.
pub struct Tela {
    pub nombre: &'static str,
    pub tecla: char,
    /// Índice en `Scene::textures`, según el orden de
    /// `scenes::RUTAS_TEXTURAS`.
    pub indice: usize,
    /// Escala `uv` con la que se estampa, la misma que usa su material.
    pub escala: f32,
}

/// Los cinco pigmentos curados.
///
/// Son fijos y no hay selector libre: una paleta pequeña y elegida es una
/// decisión de composición; un `RGB` abierto la traslada a quien mueva el
/// ratón.
pub const PIGMENTOS: [Pigmento; 5] = [
    Pigmento {
        nombre: "Carmesi",
        tecla: '4',
        srgb: [0.66, 0.09, 0.15],
    },
    Pigmento {
        nombre: "Oro",
        tecla: '5',
        srgb: [0.85, 0.65, 0.17],
    },
    Pigmento {
        nombre: "Violeta",
        tecla: '6',
        srgb: [0.42, 0.24, 0.60],
    },
    Pigmento {
        nombre: "Cian",
        tecla: '7',
        srgb: [0.18, 0.62, 0.68],
    },
    Pigmento {
        nombre: "Obsidiana",
        tecla: '8',
        srgb: [0.09, 0.10, 0.13],
    },
];

/// Las seis telas, con el índice que ocupan en la escena.
///
/// El orden es el de `scenes::RUTAS_TEXTURAS`: `0` canvas, `1` water,
/// `2` wet_basalt, `3` aged_wood, `4` meadow, `5` pictorial_crystal. Las dos
/// del skybox vienen después y no se ofrecen como pincel.
pub const TEXTURAS: [Tela; 6] = [
    Tela {
        nombre: "Lienzo",
        tecla: '9',
        indice: 0,
        escala: 6.0,
    },
    Tela {
        nombre: "Pradera",
        tecla: '0',
        indice: 4,
        escala: 4.0,
    },
    Tela {
        nombre: "Basalto",
        tecla: 'z',
        indice: 2,
        escala: 3.0,
    },
    Tela {
        nombre: "Madera",
        tecla: 'x',
        indice: 3,
        escala: 1.0,
    },
    Tela {
        nombre: "Cristal",
        tecla: 'c',
        indice: 5,
        escala: 1.5,
    },
    Tela {
        nombre: "Agua",
        tecla: 'v',
        indice: 1,
        escala: 2.0,
    },
];

/// Qué hace el botón izquierdo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Herramienta {
    /// Descubre el material final donde se arrastra.
    Revelar,
    /// Pinta un color plano. El índice es una entrada de `PIGMENTOS`.
    Pigmento(usize),
    /// Estampa una tela. El índice es una entrada de `TEXTURAS`.
    Textura(usize),
}

impl Herramienta {
    /// Nombre para la consola y para la interfaz.
    pub fn nombre(self) -> &'static str {
        match self {
            Herramienta::Revelar => "revelar",
            Herramienta::Pigmento(i) => PIGMENTOS[i].nombre,
            Herramienta::Textura(i) => TEXTURAS[i].nombre,
        }
    }

    /// La tecla que la activa.
    pub fn tecla(self) -> char {
        match self {
            Herramienta::Revelar => TECLA_DE_REVELAR,
            Herramienta::Pigmento(i) => PIGMENTOS[i].tecla,
            Herramienta::Textura(i) => TEXTURAS[i].tecla,
        }
    }
}

/// Herramienta que activa una tecla, o `None` si esa tecla no es de la
/// paleta.
///
/// Es la **única** traducción de tecla a herramienta, y la misma tabla que
/// alimenta la interfaz. El binario traduce su `minifb::Key` a un `char` y
/// pregunta aquí.
pub fn herramienta_de_tecla(tecla: char) -> Option<Herramienta> {
    let tecla = tecla.to_ascii_lowercase();

    if tecla == TECLA_DE_REVELAR {
        return Some(Herramienta::Revelar);
    }

    if let Some(i) = PIGMENTOS.iter().position(|p| p.tecla == tecla) {
        return Some(Herramienta::Pigmento(i));
    }

    TEXTURAS
        .iter()
        .position(|t| t.tecla == tecla)
        .map(Herramienta::Textura)
}

/// Un rectángulo en píxeles de ventana.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub ancho: usize,
    pub alto: usize,
}

impl Rect {
    fn contiene(&self, cursor: (f32, f32)) -> bool {
        if !cursor.0.is_finite() || !cursor.1.is_finite() || cursor.0 < 0.0 || cursor.1 < 0.0 {
            return false;
        }

        let (x, y) = (cursor.0 as usize, cursor.1 as usize);

        x >= self.x && y >= self.y && x < self.x + self.ancho && y < self.y + self.alto
    }
}

/// Una celda del panel.
#[derive(Debug, Clone, Copy)]
pub struct Celda {
    pub rect: Rect,
    pub herramienta: Herramienta,
    /// `false` si su asset no está cargado. La misma política que el
    /// teclado: sin tela no hay herramienta.
    pub activable: bool,
}

/// Qué hay bajo el cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Impacto {
    /// La cápsula que pliega y despliega.
    Capsula,
    /// Una celda del panel, por índice en `Disposicion::celdas`.
    Celda(usize),
    /// Ni una cosa ni la otra: el clic es para el diorama.
    Fuera,
}

/// Dónde cae cada pieza de la interfaz.
#[derive(Debug, Clone)]
pub struct Disposicion {
    /// Siempre presente: es lo que anuncia que la paleta existe.
    pub capsula: Rect,
    /// Solo cuando está abierta.
    pub panel: Option<Rect>,
    /// Vacío cuando está plegada.
    pub celdas: Vec<Celda>,
}

/// Márgenes y tamaños del panel, en píxeles.
const MARGEN: usize = 12;
const CAPSULA: (usize, usize) = (128, 26);
const CELDA: usize = 34;
const HUECO: usize = 6;
const COLUMNAS: usize = 6;
const CABECERA: usize = 18;

impl Disposicion {
    /// Calcula la disposición para una ventana y un estado de plegado.
    ///
    /// `telas_disponibles` dice, por índice de `TEXTURAS`, si su asset está
    /// cargado. Una tela ausente sigue ocupando su celda —para que la
    /// retícula no baile— pero no se puede elegir.
    ///
    /// Todo se ancla **abajo a la derecha** y se recorta contra la ventana:
    /// en una ventana diminuta la interfaz se apoya en el origen antes que
    /// salirse.
    pub fn calcular(
        abierta: bool,
        ancho: usize,
        alto: usize,
        telas_disponibles: &[bool],
    ) -> Disposicion {
        let capsula = Rect {
            x: ancho.saturating_sub(CAPSULA.0 + MARGEN),
            y: alto.saturating_sub(CAPSULA.1 + MARGEN),
            ancho: CAPSULA.0.min(ancho),
            alto: CAPSULA.1.min(alto),
        };

        if !abierta {
            return Disposicion {
                capsula,
                panel: None,
                celdas: Vec::new(),
            };
        }

        let total = 1 + PIGMENTOS.len() + TEXTURAS.len();
        let filas = total.div_ceil(COLUMNAS);
        let panel_ancho = MARGEN * 2 + COLUMNAS * CELDA + (COLUMNAS - 1) * HUECO;
        let panel_alto = MARGEN * 2 + CABECERA + filas * CELDA + (filas - 1) * HUECO;

        // El panel se apoya sobre la cápsula, que se queda abajo del todo.
        let panel = Rect {
            x: ancho.saturating_sub(panel_ancho + MARGEN),
            y: alto.saturating_sub(panel_alto + CAPSULA.1 + MARGEN * 2),
            ancho: panel_ancho.min(ancho),
            alto: panel_alto.min(alto),
        };

        let mut celdas = Vec::with_capacity(total);
        let mut poner = |indice: usize, herramienta: Herramienta, activable: bool| {
            let fila = indice / COLUMNAS;
            let columna = indice % COLUMNAS;

            celdas.push(Celda {
                rect: Rect {
                    x: panel.x + MARGEN + columna * (CELDA + HUECO),
                    y: panel.y + MARGEN + CABECERA + fila * (CELDA + HUECO),
                    ancho: CELDA,
                    alto: CELDA,
                },
                herramienta,
                activable,
            });
        };

        poner(0, Herramienta::Revelar, true);

        for i in 0..PIGMENTOS.len() {
            poner(1 + i, Herramienta::Pigmento(i), true);
        }

        for i in 0..TEXTURAS.len() {
            let disponible = telas_disponibles.get(i).copied().unwrap_or(false);

            poner(1 + PIGMENTOS.len() + i, Herramienta::Textura(i), disponible);
        }

        Disposicion {
            capsula,
            panel: Some(panel),
            celdas,
        }
    }

    /// Qué hay bajo el cursor.
    ///
    /// Las celdas ganan a la cápsula y la cápsula al fondo. Un cursor no
    /// finito o fuera de la ventana es `Fuera`, igual que en el picking.
    pub fn impacto(&self, cursor: (f32, f32)) -> Impacto {
        for (i, celda) in self.celdas.iter().enumerate() {
            if celda.rect.contiene(cursor) {
                return Impacto::Celda(i);
            }
        }

        if self.capsula.contiene(cursor) {
            return Impacto::Capsula;
        }

        // El cuerpo del panel también consume: un clic en el hueco entre dos
        // celdas no puede colarse y pintar el diorama que hay detrás.
        if let Some(panel) = self.panel {
            if panel.contiene(cursor) {
                return Impacto::Celda(usize::MAX);
            }
        }

        Impacto::Fuera
    }
}

/// El estado de la paleta: qué herramienta está activa y si el panel se ve.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paleta {
    abierta: bool,
    herramienta: Herramienta,
}

impl Default for Paleta {
    fn default() -> Self {
        Paleta {
            abierta: false,
            herramienta: Herramienta::Revelar,
        }
    }
}

impl Paleta {
    pub fn abierta(&self) -> bool {
        self.abierta
    }

    pub fn herramienta(&self) -> Herramienta {
        self.herramienta
    }

    /// Pliega o despliega el panel.
    pub fn alternar(&mut self) {
        self.abierta = !self.abierta;
    }

    /// Activa una herramienta. Devuelve si **cambió**.
    ///
    /// El panel **no** se pliega: se sigue pintando con él abierto, que es
    /// lo que pide la dirección. Quien lo cierra es la tecla o la cápsula.
    ///
    /// El valor de retorno es lo que el binario usa para cortar el trazo:
    /// cambiar de herramienta a mitad de un arrastre no puede continuar la
    /// pincelada anterior.
    pub fn elegir(&mut self, herramienta: Herramienta) -> bool {
        if self.herramienta == herramienta {
            return false;
        }

        self.herramienta = herramienta;

        true
    }
}

// ------------------------------------------------------ microtipografía

/// Ancho y alto de un glifo, en puntos de la rejilla.
///
/// `5 x 7` es la rejilla más pequeña en la que una mayúscula sigue siendo
/// inequívoca: con cuatro columnas la `B` y la `R` se confunden, y con seis
/// filas la `P` pierde el asta.
pub const ANCHO_GLIFO: usize = 5;
pub const ALTO_GLIFO: usize = 7;

/// A cuánto se amplía cada punto del glifo dentro de la cápsula.
///
/// Dos: a `800 x 600` un glifo de `5 x 7` sin ampliar es una mancha, y a
/// tres la etiqueta no cabe en la cápsula sin invadir el diorama.
pub const ESCALA_DE_LA_ETIQUETA: usize = 2;

/// Separación entre letras, en puntos de la rejilla.
const AVANCE: usize = ANCHO_GLIFO + 1;

/// El trazo de una letra: siete filas de cinco bits, el más significativo a
/// la izquierda.
///
/// Se dibujan a mano y no se cargan de ningún asset: son ocho glifos, el
/// proyecto no tiene tipografía y añadir una dependencia para dos palabras
/// sería desproporcionado. Solo existen las letras que las dos etiquetas
/// necesitan; cualquier otra devuelve `None` y no se dibuja.
pub fn glifo(letra: char) -> Option<[u8; ALTO_GLIFO]> {
    let trazo = match letra.to_ascii_uppercase() {
        ' ' => [0, 0, 0, 0, 0, 0, 0],
        'P' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'A' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'B' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
        ],
        'R' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
        ],
        'I' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111,
        ],
        'C' => [
            0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110,
        ],
        'E' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
        ],
        _ => return None,
    };

    Some(trazo)
}

/// Lo que dice la cápsula según el estado del panel.
///
/// Es la única lectura que el usuario tiene de qué hace ese botón, así que
/// nombra la acción y no el estado: pulsarla **abre** o **cierra**.
pub fn etiqueta_de_la_capsula(abierta: bool) -> &'static str {
    if abierta {
        "P CERRAR"
    } else {
        "P ABRIR"
    }
}

/// Ancho en píxeles que ocupa un texto a una escala dada.
pub fn ancho_de_texto(texto: &str, escala: usize) -> usize {
    let letras = texto.chars().count();

    if letras == 0 {
        return 0;
    }

    // La última letra no arrastra su separación.
    (letras * AVANCE - 1) * escala
}

/// Dibuja un texto y devuelve el ancho en píxeles que ocupó.
///
/// Cada punto encendido del glifo se pinta como un bloque de `escala x
/// escala`. Se apoya en `Framebuffer::fill_rect`, que ya recorta contra el
/// buffer, así que un texto pegado al borde se corta en vez de desbordar.
///
/// Las letras sin glifo se saltan **dejando su hueco**: así una etiqueta con
/// un carácter no previsto pierde una letra pero no descoloca las demás.
pub fn dibujar_texto(
    framebuffer: &mut Framebuffer,
    x: usize,
    y: usize,
    texto: &str,
    escala: usize,
    color: u32,
) -> usize {
    if escala == 0 {
        return 0;
    }

    let mut avance = x;

    for letra in texto.chars() {
        if let Some(trazo) = glifo(letra) {
            for (fila, bits) in trazo.iter().enumerate() {
                for columna in 0..ANCHO_GLIFO {
                    if bits & (1 << (ANCHO_GLIFO - 1 - columna)) != 0 {
                        framebuffer.fill_rect(
                            avance + columna * escala,
                            y + fila * escala,
                            escala,
                            escala,
                            color,
                        );
                    }
                }
            }
        }

        avance += AVANCE * escala;
    }

    ancho_de_texto(texto, escala)
}

// ----------------------------------------------------------------- dibujo

/// Colores de la interfaz, en sRGB empaquetado.
const FONDO_PANEL: u32 = 0x001B1A24;
const BORDE_PANEL: u32 = 0x00C8B489;
const FONDO_CAPSULA: u32 = 0x00272231;
const TINTA: u32 = 0x00E8DFC8;
const SELECCION: u32 = 0x00F0C86A;
const APAGADA: u32 = 0x003A3542;

/// Dibuja la paleta sobre el framebuffer ya trazado.
///
/// Va **después** del render, del escalado y del pincel: es interfaz, y
/// tiene que leerse por encima de todo lo demás.
///
/// Las miniaturas de las telas se muestrean de la **textura cargada**, no de
/// un color inventado: lo que se ve en la celda es lo que va a estampar el
/// pincel. Una tela ausente se dibuja apagada y tachada.
pub fn dibujar_paleta(
    framebuffer: &mut Framebuffer,
    disposicion: &Disposicion,
    paleta: &Paleta,
    texturas: &[Texture],
) {
    if let Some(panel) = disposicion.panel {
        framebuffer.fill_rect(panel.x, panel.y, panel.ancho, panel.alto, FONDO_PANEL);
        framebuffer.stroke_rect(panel.x, panel.y, panel.ancho, panel.alto, BORDE_PANEL);

        // La cabecera: una banda fina que separa el título de la retícula.
        framebuffer.fill_rect(
            panel.x + MARGEN,
            panel.y + MARGEN + CABECERA - 6,
            panel.ancho - MARGEN * 2,
            2,
            BORDE_PANEL,
        );

        for celda in &disposicion.celdas {
            dibujar_celda(framebuffer, celda, paleta.herramienta(), texturas);
        }
    }

    // La cápsula, siempre. Plegada es lo único que se ve; abierta hace de
    // botón de cerrar.
    let c = disposicion.capsula;
    framebuffer.fill_rect(c.x, c.y, c.ancho, c.alto, FONDO_CAPSULA);
    framebuffer.stroke_rect(c.x, c.y, c.ancho, c.alto, BORDE_PANEL);

    // La etiqueta, centrada: es lo único que le dice al usuario qué hace
    // ese botón. Antes había una «P» y unas barras que no decían nada.
    let texto = etiqueta_de_la_capsula(paleta.abierta());
    let ancho_texto = ancho_de_texto(texto, ESCALA_DE_LA_ETIQUETA);
    let alto_texto = ALTO_GLIFO * ESCALA_DE_LA_ETIQUETA;

    dibujar_texto(
        framebuffer,
        c.x + c.ancho.saturating_sub(ancho_texto) / 2,
        c.y + c.alto.saturating_sub(alto_texto) / 2,
        texto,
        ESCALA_DE_LA_ETIQUETA,
        TINTA,
    );
}

/// Una celda: su muestra, su marco y el realce si está activa.
fn dibujar_celda(
    framebuffer: &mut Framebuffer,
    celda: &Celda,
    activa: Herramienta,
    texturas: &[Texture],
) {
    let r = celda.rect;

    match celda.herramienta {
        Herramienta::Revelar => {
            // Medio lienzo y medio revelado: la herramienta que descubre.
            framebuffer.fill_rect(r.x, r.y, r.ancho, r.alto, 0x00D9CDB0);
            framebuffer.fill_rect(r.x + r.ancho / 2, r.y, r.ancho / 2, r.alto, 0x00335A46);
        }
        Herramienta::Pigmento(i) => {
            framebuffer.fill_rect(r.x, r.y, r.ancho, r.alto, PIGMENTOS[i].color().to_hex());
        }
        Herramienta::Textura(i) => {
            let tela = texturas.get(TEXTURAS[i].indice);

            match (tela, celda.activable) {
                (Some(tela), true) => miniatura(framebuffer, r, tela, TEXTURAS[i].escala),
                _ => {
                    // Ausente: apagada y tachada, para que se vea que existe
                    // y que no se puede elegir.
                    framebuffer.fill_rect(r.x, r.y, r.ancho, r.alto, APAGADA);

                    for k in 0..r.ancho.min(r.alto) {
                        framebuffer.fill_rect(r.x + k, r.y + k, 2, 2, TINTA);
                    }
                }
            }
        }
    }

    let marco = if celda.herramienta == activa {
        SELECCION
    } else {
        BORDE_PANEL
    };

    framebuffer.stroke_rect(r.x, r.y, r.ancho, r.alto, marco);

    // La activa lleva un segundo marco por dentro: se distingue de un
    // vistazo sin depender del color del borde.
    if celda.herramienta == activa {
        framebuffer.stroke_rect(r.x + 2, r.y + 2, r.ancho - 4, r.alto - 4, SELECCION);
    }
}

/// Miniatura real de una tela: se muestrea la textura cargada.
fn miniatura(framebuffer: &mut Framebuffer, r: Rect, tela: &Texture, escala: f32) {
    for fila in 0..r.alto {
        for columna in 0..r.ancho {
            let u = columna as f32 / r.ancho as f32;
            let v = 1.0 - fila as f32 / r.alto as f32;
            let muestra = tela.sample(u * escala, v * escala);

            framebuffer.fill_rect(r.x + columna, r.y + fila, 1, 1, muestra.to_hex());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ANCHO: usize = 800;
    const ALTO: usize = 600;

    /// Todas las telas disponibles, que es el caso normal del diorama.
    fn todas() -> [bool; TEXTURAS.len()] {
        [true; TEXTURAS.len()]
    }

    #[test]
    fn la_capsula_esta_siempre_disponible_aunque_el_panel_este_plegado() {
        let plegado = Disposicion::calcular(false, ANCHO, ALTO, &todas());

        assert!(plegado.panel.is_none(), "plegado no dibuja panel");
        assert!(plegado.celdas.is_empty(), "plegado no ofrece celdas");

        // La cápsula existe, tiene área y cae abajo a la derecha.
        let capsula = plegado.capsula;
        assert!(capsula.ancho > 0 && capsula.alto > 0);
        assert!(
            capsula.x + capsula.ancho <= ANCHO && capsula.y + capsula.alto <= ALTO,
            "la capsula se sale del cuadro: {capsula:?}"
        );
        assert!(
            capsula.x > ANCHO / 2 && capsula.y > ALTO / 2,
            "la capsula deberia caer abajo a la derecha: {capsula:?}"
        );
    }

    #[test]
    fn abierto_el_panel_ofrece_revelar_los_pigmentos_y_las_telas() {
        let abierto = Disposicion::calcular(true, ANCHO, ALTO, &todas());
        let panel = abierto.panel.expect("abierto dibuja panel");

        assert_eq!(
            abierto.celdas.len(),
            1 + PIGMENTOS.len() + TEXTURAS.len(),
            "faltan celdas"
        );

        // Revelar, los cinco pigmentos y las seis telas, en ese orden.
        assert_eq!(abierto.celdas[0].herramienta, Herramienta::Revelar);
        for i in 0..PIGMENTOS.len() {
            assert_eq!(
                abierto.celdas[1 + i].herramienta,
                Herramienta::Pigmento(i),
                "celda {i} de pigmento"
            );
        }
        for i in 0..TEXTURAS.len() {
            assert_eq!(
                abierto.celdas[1 + PIGMENTOS.len() + i].herramienta,
                Herramienta::Textura(i),
                "celda {i} de textura"
            );
        }

        // Cada celda cabe dentro del panel, y el panel dentro del cuadro.
        assert!(panel.x + panel.ancho <= ANCHO && panel.y + panel.alto <= ALTO);

        for celda in &abierto.celdas {
            let r = celda.rect;

            assert!(
                r.x >= panel.x
                    && r.y >= panel.y
                    && r.x + r.ancho <= panel.x + panel.ancho
                    && r.y + r.alto <= panel.y + panel.alto,
                "la celda {:?} se sale del panel {panel:?}",
                celda.herramienta
            );
        }
    }

    #[test]
    fn las_celdas_no_se_solapan() {
        let abierto = Disposicion::calcular(true, ANCHO, ALTO, &todas());

        for (i, a) in abierto.celdas.iter().enumerate() {
            for b in abierto.celdas.iter().skip(i + 1) {
                let solapa = a.rect.x < b.rect.x + b.rect.ancho
                    && b.rect.x < a.rect.x + a.rect.ancho
                    && a.rect.y < b.rect.y + b.rect.alto
                    && b.rect.y < a.rect.y + a.rect.alto;

                assert!(
                    !solapa,
                    "{:?} y {:?} se solapan",
                    a.herramienta, b.herramienta
                );
            }
        }
    }

    #[test]
    fn el_hit_test_distingue_capsula_celda_y_fuera() {
        let abierto = Disposicion::calcular(true, ANCHO, ALTO, &todas());
        let centro = |r: Rect| {
            (
                r.x as f32 + r.ancho as f32 / 2.0,
                r.y as f32 + r.alto as f32 / 2.0,
            )
        };

        assert_eq!(
            abierto.impacto(centro(abierto.capsula)),
            Impacto::Capsula,
            "el centro de la capsula"
        );

        for (i, celda) in abierto.celdas.iter().enumerate() {
            assert_eq!(
                abierto.impacto(centro(celda.rect)),
                Impacto::Celda(i),
                "el centro de la celda {i}"
            );
        }

        // Arriba a la izquierda no hay interfaz: ahi se pinta.
        assert_eq!(abierto.impacto((10.0, 10.0)), Impacto::Fuera);
        // Y tampoco fuera de la ventana ni en un cursor roto.
        assert_eq!(abierto.impacto((-5.0, 300.0)), Impacto::Fuera);
        assert_eq!(abierto.impacto((f32::NAN, 300.0)), Impacto::Fuera);
    }

    #[test]
    fn plegado_solo_la_capsula_consume_el_clic() {
        let plegado = Disposicion::calcular(false, ANCHO, ALTO, &todas());
        let centro = (
            plegado.capsula.x as f32 + plegado.capsula.ancho as f32 / 2.0,
            plegado.capsula.y as f32 + plegado.capsula.alto as f32 / 2.0,
        );

        assert_eq!(plegado.impacto(centro), Impacto::Capsula);

        // Justo encima del panel plegado —donde estaria si estuviera
        // abierto— se pinta, porque no hay panel.
        let abierto = Disposicion::calcular(true, ANCHO, ALTO, &todas());
        let panel = abierto.panel.expect("abierto tiene panel");
        let dentro_del_panel = (panel.x as f32 + 5.0, panel.y as f32 + 5.0);

        assert_eq!(plegado.impacto(dentro_del_panel), Impacto::Fuera);
        assert_ne!(abierto.impacto(dentro_del_panel), Impacto::Fuera);
    }

    #[test]
    fn una_tela_que_falta_no_se_puede_elegir() {
        // La misma politica que el teclado: si el asset no esta cargado, la
        // herramienta no se activa.
        let mut disponibles = todas();
        disponibles[2] = false;

        let abierto = Disposicion::calcular(true, ANCHO, ALTO, &disponibles);
        let celda = abierto
            .celdas
            .iter()
            .find(|c| c.herramienta == Herramienta::Textura(2))
            .expect("la celda existe aunque la tela falte");

        assert!(!celda.activable, "la celda de una tela ausente no se elige");

        // Y las demas siguen activables.
        for celda in &abierto.celdas {
            if celda.herramienta != Herramienta::Textura(2) {
                assert!(celda.activable, "{:?} deberia activarse", celda.herramienta);
            }
        }
    }

    #[test]
    fn el_estado_recuerda_la_herramienta_y_el_plegado() {
        let mut estado = Paleta::default();

        assert!(!estado.abierta(), "arranca plegada");
        assert_eq!(estado.herramienta(), Herramienta::Revelar);

        estado.alternar();
        assert!(estado.abierta(), "P la abre");

        // Elegir una celda **no** la cierra: se sigue pintando con ella
        // abierta, que es lo que pidio la direccion.
        assert!(estado.elegir(Herramienta::Pigmento(3)));
        assert_eq!(estado.herramienta(), Herramienta::Pigmento(3));
        assert!(estado.abierta(), "elegir no pliega el panel");

        // Elegir lo mismo otra vez no cuenta como cambio.
        assert!(!estado.elegir(Herramienta::Pigmento(3)));

        estado.alternar();
        assert!(!estado.abierta(), "P la pliega");
    }

    #[test]
    fn las_teclas_y_la_interfaz_leen_la_misma_tabla() {
        // Una sola fuente: la tecla que anuncia la celda es la que activa la
        // herramienta. Si hubiera dos tablas, esto seria lo que se
        // desincronizaria sin que nada avisara.
        for (i, pigmento) in PIGMENTOS.iter().enumerate() {
            assert_eq!(
                herramienta_de_tecla(pigmento.tecla),
                Some(Herramienta::Pigmento(i)),
                "la tecla {} del pigmento {}",
                pigmento.tecla,
                pigmento.nombre
            );
        }

        for (i, tela) in TEXTURAS.iter().enumerate() {
            assert_eq!(
                herramienta_de_tecla(tela.tecla),
                Some(Herramienta::Textura(i)),
                "la tecla {} de la tela {}",
                tela.tecla,
                tela.nombre
            );
        }

        assert_eq!(herramienta_de_tecla('q'), Some(Herramienta::Revelar));
        assert_eq!(herramienta_de_tecla('k'), None, "una tecla ajena no elige");
    }

    #[test]
    fn ninguna_tecla_de_la_paleta_choca_con_las_de_la_demo() {
        // `1`, `2` y `3` revelan regiones; `L` reinicia; `R` restaura el
        // encuadre; `W` y `S` son zoom; `M` y `N` el grosor; `P` abre la
        // paleta.
        const RESERVADAS: [char; 9] = ['1', '2', '3', 'l', 'r', 'w', 's', 'm', 'n'];

        for reservada in RESERVADAS {
            assert_eq!(
                herramienta_de_tecla(reservada),
                None,
                "la tecla {reservada} ya tiene dueno"
            );
        }

        assert_eq!(
            herramienta_de_tecla(TECLA_DE_LA_PALETA),
            None,
            "la tecla de la paleta no puede elegir herramienta"
        );
    }

    #[test]
    fn los_pigmentos_tienen_color_lineal_utilizable() {
        for pigmento in PIGMENTOS.iter() {
            let color = pigmento.color();

            assert!(
                color.r.is_finite() && color.g.is_finite() && color.b.is_finite(),
                "{} no da un color finito",
                pigmento.nombre
            );
            assert!(
                (0.0..=1.0).contains(&color.r)
                    && (0.0..=1.0).contains(&color.g)
                    && (0.0..=1.0).contains(&color.b),
                "{} se sale de rango: {color:?}",
                pigmento.nombre
            );
        }
    }

    // ------------------------------------------------ microtipografia

    /// Las letras que la capsula necesita poder escribir.
    const LETRAS_NECESARIAS: [char; 7] = ['P', 'A', 'B', 'R', 'I', 'C', 'E'];

    #[test]
    fn hay_glifo_para_cada_letra_de_la_capsula() {
        for texto in [etiqueta_de_la_capsula(false), etiqueta_de_la_capsula(true)] {
            for letra in texto.chars() {
                let trazo = glifo(letra).unwrap_or_else(|| {
                    panic!("no hay glifo para {letra:?}, que aparece en {texto:?}")
                });

                if letra != ' ' {
                    assert!(
                        trazo.iter().any(|fila| *fila != 0),
                        "el glifo de {letra:?} esta vacio"
                    );
                }
            }
        }

        // Y las siete letras que componen las dos etiquetas existen.
        for letra in LETRAS_NECESARIAS {
            assert!(glifo(letra).is_some(), "falta el glifo de {letra:?}");
        }
    }

    #[test]
    fn el_texto_dibuja_la_huella_exacta_de_sus_glifos() {
        // Lo que este test exige no es que la funcion devuelva una cadena,
        // sino que **los pixeles encendidos sean los del glifo**, en su
        // sitio y a su escala. Una barra decorativa en lugar de una letra
        // fallaria aqui aunque ocupara el mismo hueco.
        const ESCALA: usize = 2;
        const TINTA_DE_PRUEBA: u32 = 0x00ABCDEF;

        let texto = etiqueta_de_la_capsula(false);
        let mut fb = Framebuffer::new(300, 40);
        let ancho = dibujar_texto(&mut fb, 4, 5, texto, ESCALA, TINTA_DE_PRUEBA);

        assert!(ancho > 0, "no se dibujo nada");

        let mut avance = 4usize;

        for letra in texto.chars() {
            let trazo = glifo(letra).expect("la letra tiene glifo");

            for (fila, bits) in trazo.iter().enumerate() {
                for columna in 0..ANCHO_GLIFO {
                    let encendido = bits & (1 << (ANCHO_GLIFO - 1 - columna)) != 0;

                    // El bloque de `ESCALA x ESCALA` que le toca a ese punto
                    // del glifo tiene que estar entero encendido o entero
                    // apagado, y del color pedido.
                    for dy in 0..ESCALA {
                        for dx in 0..ESCALA {
                            let x = avance + columna * ESCALA + dx;
                            let y = 5 + fila * ESCALA + dy;
                            let pixel = fb.buffer[y * 300 + x];

                            assert_eq!(
                                pixel == TINTA_DE_PRUEBA,
                                encendido,
                                "{letra:?} fila {fila} columna {columna} en ({x}, {y})"
                            );
                        }
                    }
                }
            }

            avance += (ANCHO_GLIFO + 1) * ESCALA;
        }
    }

    #[test]
    fn las_dos_etiquetas_se_distinguen_en_pantalla() {
        // Plegada y abierta no pueden pintar lo mismo: es lo unico que le
        // dice al usuario en que estado esta.
        const ESCALA: usize = 2;

        let mut plegada = Framebuffer::new(300, 40);
        let mut abierta = Framebuffer::new(300, 40);

        dibujar_texto(
            &mut plegada,
            4,
            5,
            etiqueta_de_la_capsula(false),
            ESCALA,
            0x00FFFFFF,
        );
        dibujar_texto(
            &mut abierta,
            4,
            5,
            etiqueta_de_la_capsula(true),
            ESCALA,
            0x00FFFFFF,
        );

        assert_ne!(
            plegada.buffer, abierta.buffer,
            "las dos etiquetas pintan lo mismo"
        );
    }

    #[test]
    fn la_etiqueta_cabe_dentro_de_la_capsula() {
        // La capsula no puede crecer hasta tapar el diorama, asi que el
        // texto tiene que caber en ella con margen por los cuatro lados.
        let disposicion = Disposicion::calcular(false, ANCHO, ALTO, &todas());
        let capsula = disposicion.capsula;

        let mut fb = Framebuffer::new(ANCHO, ALTO);
        let ancho = dibujar_texto(
            &mut fb,
            0,
            0,
            etiqueta_de_la_capsula(true),
            ESCALA_DE_LA_ETIQUETA,
            0x00FFFFFF,
        );
        let alto = ALTO_GLIFO * ESCALA_DE_LA_ETIQUETA;

        assert!(
            ancho < capsula.ancho,
            "la etiqueta mide {ancho} px y la capsula {}",
            capsula.ancho
        );
        assert!(
            alto < capsula.alto,
            "la etiqueta mide {alto} px de alto y la capsula {}",
            capsula.alto
        );
    }

    #[test]
    fn el_texto_se_recorta_contra_el_framebuffer() {
        // Dibujar pegado al borde no puede desbordar ni entrar en panico.
        let mut fb = Framebuffer::new(20, 12);

        dibujar_texto(&mut fb, 15, 8, etiqueta_de_la_capsula(true), 2, 0x00FFFFFF);

        assert_eq!(fb.buffer.len(), 20 * 12, "el buffer cambio de tamano");
    }
}
