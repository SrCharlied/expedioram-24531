use std::fmt;
use std::ops::{Add, Mul};

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

    /// Interpreta un `0xRRGGBB` como color lineal dividiendo cada canal
    /// entre 255.
    pub fn from_hex(hex: u32) -> Self {
        Color {
            r: ((hex >> 16) & 0xFF) as f32 / 255.0,
            g: ((hex >> 8) & 0xFF) as f32 / 255.0,
            b: (hex & 0xFF) as f32 / 255.0,
        }
    }

    /// Empaca a `0xRRGGBB`. Este es el único punto donde se pierde el rango
    /// extendido: aquí sí se recorta a `0.0..=1.0`.
    pub fn to_hex(self) -> u32 {
        let channel = |value: f32| -> u32 { (value.clamp(0.0, 1.0) * 255.0).round() as u32 };

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
    fn to_hex_recorta_el_rango_extendido() {
        // 0.5 * 255 = 127.5, que redondea a 128.
        assert_eq!(Color::new(1.5, -0.2, 0.5).to_hex(), 0xFF0080);
    }

    #[test]
    fn to_hex_no_da_la_vuelta() {
        // El fallo clásico de empacar sin recortar: 2.0 * 255 = 510, que
        // truncado a u8 da 254 en lugar de 255.
        let desbordado = Color::new(2.0, 3.0, 10.0).to_hex();

        assert_eq!(desbordado, 0xFFFFFF);
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
    fn black_es_el_cero_de_la_suma() {
        let color = Color::new(0.2, 0.4, 0.6);
        let suma = color + Color::black();

        assert_close(suma.r, color.r);
        assert_close(suma.g, color.g);
        assert_close(suma.b, color.b);
    }
}
