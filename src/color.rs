//! Color lineal y conversión sRGB.
//!
//! # Decisión del Hito 4: pipeline sRGB completo
//!
//! El proyecto separa dos espacios de color y no los mezcla nunca:
//!
//! - **Lineal**, donde vive todo el cálculo. Sumar luces, multiplicar por
//!   una atenuación, repartir energía entre reflejo y refracción: todo eso
//!   solo es correcto si los valores son proporcionales a la energía. `Color`
//!   siempre guarda lineal.
//! - **sRGB**, donde viven los archivos y la pantalla. Un PNG no guarda
//!   energía sino valores percibidos, comprimidos por una curva que dedica
//!   más precisión a los tonos oscuros, que es donde el ojo distingue más.
//!
//! Las fronteras son cuatro, y todas están en este módulo o en `Texture`:
//!
//! | Frontera | Dirección |
//! |---|---|
//! | `Color::from_hex` | sRGB → lineal |
//! | `Color::from_srgb` | sRGB → lineal |
//! | `Texture::load` | sRGB → lineal |
//! | `Color::to_hex` | lineal → sRGB |
//!
//! `Color::new` **no** convierte: su contrato es recibir lineal. Para
//! escribir un color «como se ve» está `from_srgb`.
//!
//! ## Por qué importa
//!
//! Sin esta separación, promediar o atenuar produce resultados demasiado
//! oscuros en los medios tonos: interpolar entre negro y blanco al 50 % en
//! sRGB da un gris que la vista lee como bastante más oscuro que la mitad.
//! Con luces sumándose y con la interpolación de la revelación del Hito 6,
//! ese error se acumula.
//!
//! No hay gestión de color más allá de esto: nada de perfiles ICC ni de
//! espacios distintos de sRGB. Es la curva estándar y nada más.

use std::fmt;
use std::ops::{Add, Mul};

/// Umbral del tramo lineal de la curva sRGB, en el lado sRGB.
const UMBRAL_SRGB: f32 = 0.04045;
/// El mismo umbral visto desde el lado lineal.
const UMBRAL_LINEAL: f32 = 0.0031308;
/// Pendiente del tramo lineal.
const PENDIENTE: f32 = 12.92;

/// Convierte un canal de sRGB a lineal.
///
/// La curva tiene dos tramos: uno recto cerca del negro y una potencia para
/// el resto. El tramo recto existe porque la potencia tiene pendiente
/// infinita en cero, y eso amplificaría el ruido de cuantización de los
/// tonos más oscuros.
pub fn srgb_to_linear(canal: f32) -> f32 {
    if canal <= UMBRAL_SRGB {
        canal / PENDIENTE
    } else {
        ((canal + 0.055) / 1.055).powf(2.4)
    }
}

/// Convierte un canal de lineal a sRGB. Inversa exacta de
/// `srgb_to_linear`.
pub fn linear_to_srgb(canal: f32) -> f32 {
    if canal <= UMBRAL_LINEAL {
        canal * PENDIENTE
    } else {
        1.055 * canal.powf(1.0 / 2.4) - 0.055
    }
}

/// Color en punto flotante lineal, un canal por componente.
///
/// El framebuffer guarda enteros de 32 bits, pero la iluminación recursiva
/// suma aportes que pueden pasarse de `1.0` antes del clamp final: un
/// reflejo más un specular más la luz directa. Guardar eso en `u8` obliga a
/// saturar en cada paso intermedio y el error se acumula. Los canales viven
/// en `f32` durante todo el render y se empacan una sola vez, al escribir el
/// píxel.
///
/// El rango normal de trabajo es `0.0..=1.0`, pero valores mayores son
/// legítimos mientras no se convierta a `u32`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b }
    }

    pub fn black() -> Self {
        Color::new(0.0, 0.0, 0.0)
    }

    /// Interpreta un `0xRRGGBB` **sRGB** y lo lleva a lineal.
    ///
    /// Un literal hexadecimal es un valor de archivo o de paleta, escrito
    /// como se ve; nunca energía. De ahí la decodificación.
    pub fn from_hex(hex: u32) -> Self {
        let canal =
            |desplazamiento: u32| srgb_to_linear(((hex >> desplazamiento) & 0xFF) as f32 / 255.0);

        Color {
            r: canal(16),
            g: canal(8),
            b: canal(0),
        }
    }

    /// Construye desde componentes **sRGB** en `0.0..=1.0`.
    ///
    /// Es el constructor para escribir un color «como se ve»: un gris medio
    /// percibido es `from_srgb(0.5, 0.5, 0.5)`, que en lineal vale `0.214`.
    /// `Color::new` con `0.5` daría un gris bastante más claro.
    pub fn from_srgb(r: f32, g: f32, b: f32) -> Self {
        Color {
            r: srgb_to_linear(r),
            g: srgb_to_linear(g),
            b: srgb_to_linear(b),
        }
    }

    /// Empaca a `0xRRGGBB` **sRGB**, listo para el framebuffer o un PNG.
    ///
    /// Aquí ocurren las dos únicas pérdidas del pipeline, y en este orden:
    /// primero el recorte del rango extendido a `0.0..=1.0`, después la
    /// codificación sRGB y la cuantización a ocho bits. Recortar antes de
    /// codificar importa: la potencia de la curva no está definida para
    /// valores negativos.
    pub fn to_hex(self) -> u32 {
        let channel = |value: f32| -> u32 {
            // Solo el NaN necesita trato especial: no tiene un recorte
            // sensato y se trata como negro. Los infinitos si lo tienen
            // —`clamp` los lleva a 1.0 y a 0.0—, y llevar un desbordamiento
            // de brillo a blanco lo hace visible en la imagen, mientras que
            // llevarlo a negro lo escondería entre las sombras.
            let recortado = if value.is_nan() {
                0.0
            } else {
                value.clamp(0.0, 1.0)
            };

            (linear_to_srgb(recortado) * 255.0).round() as u32
        };

        (channel(self.r) << 16) | (channel(self.g) << 8) | channel(self.b)
    }
}

/// Sumar dos colores es sumar luz. A diferencia de la versión en `u8`, no
/// se satura: el resultado puede pasar de `1.0` y el recorte se hace al
/// final, en `to_hex`.
impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

/// Multiplicar por un escalar es subir o bajar la intensidad.
impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, scalar: f32) -> Color {
        Color {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
        }
    }
}

/// Multiplicar dos colores es filtrar uno a través del otro: es lo que hace
/// el albedo con la luz que recibe. Un objeto rojo bajo luz azul se ve casi
/// negro, y eso sale solo de multiplicar canal a canal.
impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color(r: {}, g: {}, b: {})", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tolerancia para comparar flotantes.
    const EPS: f32 = 1e-6;

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < EPS,
            "esperado {expected}, obtenido {actual}"
        );
    }

    #[test]
    fn construye_en_rango_normalizado() {
        let color = Color::new(1.0, 0.5, 0.0);

        assert_close(color.r, 1.0);
        assert_close(color.g, 0.5);
        assert_close(color.b, 0.0);
    }

    #[test]
    fn suma_sin_perdida_prematura() {
        // Dos aportes que juntos pasan de 1.0. La versión en u8 saturaba
        // aquí y perdía el exceso; en flotante debe conservarse hasta el
        // empaque final.
        let suma = Color::new(0.6, 0.3, 0.0) + Color::new(0.6, 0.3, 0.0);

        assert_close(suma.r, 1.2);
        assert_close(suma.g, 0.6);
        assert_close(suma.b, 0.0);
    }

    #[test]
    fn multiplicacion_escalar_no_recorta() {
        let escalado = Color::new(0.5, 0.25, 0.125) * 4.0;

        assert_close(escalado.r, 2.0);
        assert_close(escalado.g, 1.0);
        assert_close(escalado.b, 0.5);
    }

    #[test]
    fn to_hex_no_da_la_vuelta() {
        // El fallo clásico de empacar sin recortar: 2.0 * 255 = 510, que
        // truncado a u8 da 254 en lugar de 255.
        let desbordado = Color::new(2.0, 3.0, 10.0).to_hex();

        assert_eq!(desbordado, 0xFFFFFF);
    }

    #[test]
    fn to_hex_recorta_el_rango_extendido() {
        // Lineal 0.5 codifica al byte 188, no al 128: la curva sRGB dedica
        // mas codigo a los tonos oscuros, asi que la mitad de la energia
        // queda por encima de la mitad del rango de bytes.
        assert_eq!(Color::new(1.5, -0.2, 0.5).to_hex(), 0xFF00BC);
    }

    #[test]
    fn la_curva_srgb_es_continua_en_el_codo() {
        // Los dos tramos tienen que empalmar: una discontinuidad ahi seria
        // un salto visible en los tonos oscuros.
        let eps = 1e-6;

        let izq = srgb_to_linear(0.04045 - eps);
        let der = srgb_to_linear(0.04045 + eps);
        assert!((izq - der).abs() < 1e-5, "{izq} contra {der}");

        let izq = linear_to_srgb(0.0031308 - eps);
        let der = linear_to_srgb(0.0031308 + eps);
        assert!((izq - der).abs() < 1e-4, "{izq} contra {der}");
    }

    #[test]
    fn los_extremos_de_la_curva_son_puntos_fijos() {
        for extremo in [0.0_f32, 1.0] {
            assert_close(srgb_to_linear(extremo), extremo);
            assert_close(linear_to_srgb(extremo), extremo);
        }
    }

    #[test]
    fn las_dos_conversiones_son_inversas() {
        for paso in 0..=100 {
            let srgb = paso as f32 / 100.0;
            let ida_vuelta = linear_to_srgb(srgb_to_linear(srgb));

            assert!(
                (ida_vuelta - srgb).abs() < 1e-5,
                "srgb {srgb} volvio como {ida_vuelta}"
            );
        }

        for paso in 0..=100 {
            let lineal = paso as f32 / 100.0;
            let ida_vuelta = srgb_to_linear(linear_to_srgb(lineal));

            assert!(
                (ida_vuelta - lineal).abs() < 1e-5,
                "lineal {lineal} volvio como {ida_vuelta}"
            );
        }
    }

    #[test]
    fn los_256_bytes_sobreviven_la_ida_y_vuelta_exactos() {
        // Es la garantia que necesita el color de fondo del renderer: se
        // escribe como literal hexadecimal, se decodifica a lineal y se
        // vuelve a codificar al escribir el pixel. Si un solo byte se
        // desviara, el fondo cambiaria de tono sin que nadie lo tocara.
        for byte in 0..=255u32 {
            let hex = (byte << 16) | (byte << 8) | byte;
            let vuelta = Color::from_hex(hex).to_hex();

            assert_eq!(vuelta, hex, "el byte {byte} no volvio exacto");
        }
    }

    #[test]
    fn el_color_de_fondo_del_renderer_vuelve_intacto() {
        let fondo = 0x040C24;

        assert_eq!(Color::from_hex(fondo).to_hex(), fondo);
    }

    #[test]
    fn los_valores_de_referencia_de_la_curva() {
        // Los cuatro numeros que caracterizan la curva estandar.
        assert!((srgb_to_linear(0.5) - 0.214041).abs() < 1e-5);
        assert!((linear_to_srgb(0.5) - 0.735357).abs() < 1e-5);
        assert!((srgb_to_linear(0.04045) - 0.0031308).abs() < 1e-7);
        assert_eq!((linear_to_srgb(0.5) * 255.0).round() as u32, 188);
    }

    #[test]
    fn from_srgb_escribe_colores_como_se_ven() {
        // Un gris medio percibido no es 0.5 lineal.
        let percibido = Color::from_srgb(0.5, 0.5, 0.5);

        assert_close(percibido.r, 0.214041);
        // Y al salir vuelve a ser el mismo gris medio: 0.5 * 255 = 127.5,
        // que redondea a 128.
        assert_eq!(percibido.to_hex(), 0x808080);
    }

    #[test]
    fn from_srgb_y_from_hex_coinciden() {
        for byte in [0u32, 1, 10, 64, 128, 200, 255] {
            let por_hex = Color::from_hex((byte << 16) | (byte << 8) | byte);
            let por_srgb = {
                let c = byte as f32 / 255.0;
                Color::from_srgb(c, c, c)
            };

            assert_close(por_hex.r, por_srgb.r);
            assert_close(por_hex.g, por_srgb.g);
            assert_close(por_hex.b, por_srgb.b);
        }
    }

    #[test]
    fn new_conserva_su_contrato_lineal() {
        // `new` no convierte nada: lo que entra es lo que se guarda.
        let color = Color::new(0.214041, 0.5, 1.0);

        assert_close(color.r, 0.214041);
        assert_close(color.g, 0.5);
        assert_close(color.b, 1.0);
    }

    #[test]
    fn un_nan_se_codifica_como_negro_y_no_como_un_byte_cualquiera() {
        let hex = Color::new(f32::NAN, f32::INFINITY, f32::NEG_INFINITY).to_hex();

        // NaN -> negro; +inf recorta a 1.0; -inf recorta a 0.0.
        assert_eq!(hex, 0x00FF00);
    }

    #[test]
    fn negativos_no_dan_la_vuelta() {
        assert_eq!(Color::new(-1.0, -0.5, 0.0).to_hex(), 0x000000);
    }

    #[test]
    fn from_hex_y_to_hex_son_inversos() {
        for hex in [0x000000, 0xFFFFFF, 0x040C24, 0x6496C8] {
            assert_eq!(Color::from_hex(hex).to_hex(), hex);
        }
    }

    #[test]
    fn el_producto_de_colores_filtra_canal_a_canal() {
        let rojo = Color::new(1.0, 0.0, 0.0);
        let luz_azul = Color::new(0.2, 0.4, 1.0);
        let filtrado = rojo * luz_azul;

        assert_close(filtrado.r, 0.2);
        assert_close(filtrado.g, 0.0);
        assert_close(filtrado.b, 0.0);
    }

    #[test]
    fn el_blanco_es_el_neutro_del_producto() {
        let color = Color::new(0.3, 0.6, 0.9);
        let igual = color * Color::new(1.0, 1.0, 1.0);

        assert_close(igual.r, color.r);
        assert_close(igual.g, color.g);
        assert_close(igual.b, color.b);
    }

    #[test]
    fn black_es_el_cero_de_la_suma() {
        let color = Color::new(0.2, 0.4, 0.6);
        let suma = color + Color::black();

        assert_close(suma.r, color.r);
        assert_close(suma.g, color.g);
        assert_close(suma.b, color.b);
    }
}
