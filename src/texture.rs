//! Texturas y muestreo por coordenadas UV.
//!
//! Una textura es una rejilla de colores que se consulta con las `uv` que
//! trae el impacto. Dos decisiones fijan su comportamiento y conviene
//! tenerlas presentes al generar los assets del Hito 4:
//!
//! - **`v = 0` está abajo.** Es la convención de OpenGL, no la de los
//!   archivos de imagen, donde la fila 0 es la de arriba. Se elige esta
//!   porque las `uv` del cuboide crecen con el eje del mundo, y ese crece
//!   hacia arriba: sin invertir, cada textura saldría del revés.
//! - **Muestreo por vecino más cercano.** Igual que el escalado del perfil
//!   interactivo: el diorama es de caras planas y aristas duras, y el
//!   aspecto de lienzo pintado se lleva mejor con píxeles definidos que con
//!   una interpolación que los difumina.

use crate::color::{srgb_to_linear, Color};
use std::fmt;
use std::path::{Path, PathBuf};

/// Qué hacer con coordenadas fuera de `0.0..=1.0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WrapMode {
    /// La textura se repite. Es lo que necesitan las superficies con
    /// `uv_scale` mayor que uno: césped, lienzo, sendero.
    ///
    /// Ojo con la costura: `u = 1.0` envuelve a `0.0`, porque en un mosaico
    /// ahí empieza la repetición siguiente. El borde derecho de la textura
    /// solo se alcanza con valores estrictamente menores que uno.
    #[default]
    Repeat,
    /// Se recorta al borde. Para texturas que se aplican una sola vez y no
    /// deben mostrar costura, como el panorama del skybox.
    Clamp,
}

impl WrapMode {
    /// Lleva una coordenada al rango `0.0..=1.0`.
    pub fn apply(self, coordenada: f32) -> f32 {
        match self {
            // `rem_euclid` y no `%`: con coordenadas negativas el resto de
            // Rust también es negativo, y eso dejaría la coordenada fuera
            // de rango en vez de envolverla.
            WrapMode::Repeat => coordenada.rem_euclid(1.0),
            WrapMode::Clamp => coordenada.clamp(0.0, 1.0),
        }
    }
}

/// Por qué no se pudo cargar una textura.
///
/// El plan exige que una textura ausente dé un error explícito y **no** un
/// color de relleno silencioso: un asset que falta y se sustituye por
/// magenta o blanco se descubre mirando la imagen final, cuando ya cuesta
/// caro; uno que aborta con la ruta en el mensaje se arregla en el momento.
#[derive(Debug)]
pub enum TextureError {
    NoEncontrada(PathBuf),
    NoSePudoDecodificar { ruta: PathBuf, causa: String },
    Vacia(PathBuf),
}

impl fmt::Display for TextureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TextureError::NoEncontrada(ruta) => {
                write!(f, "no existe la textura {}", ruta.display())
            }
            TextureError::NoSePudoDecodificar { ruta, causa } => {
                write!(f, "no se pudo decodificar {}: {causa}", ruta.display())
            }
            TextureError::Vacia(ruta) => {
                write!(f, "la textura {} no tiene pixeles", ruta.display())
            }
        }
    }
}

impl std::error::Error for TextureError {}

/// Rejilla de colores muestreable por UV.
#[derive(Debug, Clone)]
pub struct Texture {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
    pub wrap: WrapMode,
}

impl Texture {
    /// Carga un PNG desde disco, **decodificando sRGB a lineal**.
    ///
    /// Un PNG guarda valores percibidos, no energía. El renderer trabaja en
    /// lineal, así que la conversión ocurre aquí, una sola vez al cargar, y
    /// no en cada muestreo: son millones de muestras por cuadro contra unas
    /// pocas cargas al arrancar.
    pub fn load(path: &Path) -> Result<Texture, TextureError> {
        if !path.exists() {
            return Err(TextureError::NoEncontrada(path.to_path_buf()));
        }

        let imagen = image::open(path).map_err(|e| TextureError::NoSePudoDecodificar {
            ruta: path.to_path_buf(),
            causa: e.to_string(),
        })?;

        let rgb = imagen.to_rgb8();
        let (width, height) = (rgb.width() as usize, rgb.height() as usize);

        if width == 0 || height == 0 {
            return Err(TextureError::Vacia(path.to_path_buf()));
        }

        let pixels = rgb
            .pixels()
            .map(|p| {
                Color::new(
                    srgb_to_linear(p[0] as f32 / 255.0),
                    srgb_to_linear(p[1] as f32 / 255.0),
                    srgb_to_linear(p[2] as f32 / 255.0),
                )
            })
            .collect();

        Ok(Texture {
            width,
            height,
            pixels,
            wrap: WrapMode::default(),
        })
    }

    /// Construye una textura en memoria. Los píxeles llegan en orden de
    /// lectura de imagen: fila 0 arriba.
    pub fn from_pixels(width: usize, height: usize, pixels: Vec<Color>) -> Option<Texture> {
        if width == 0 || height == 0 || pixels.len() != width * height {
            return None;
        }

        Some(Texture {
            width,
            height,
            pixels,
            wrap: WrapMode::default(),
        })
    }

    pub fn with_wrap(mut self, wrap: WrapMode) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Valor más alto de **cada canal** en toda la textura, en lineal.
    ///
    /// Sirve para acotar una **ganancia**: el albedo efectivo de un material
    /// texturizado es `albedo × muestra`, así que subir el albedo por encima
    /// de `1 / pico` haría que algún píxel devolviera más luz de la que
    /// recibe. Se calcula una vez al derivar el material, no por muestreo.
    ///
    /// Por canal y no un solo escalar: un pico común acotaría el azul de una
    /// textura rojiza contra el rojo, que es mucho más alto, y la ganancia
    /// dejaría de coincidir con la del mismo material sin textura. El azul
    /// de la madera del pecio llega a `0.017` y su rojo a `0.171`; un techo
    /// común los trataría igual.
    pub fn peak(&self) -> Color {
        self.pixels.iter().fold(Color::black(), |acc, c| {
            Color::new(acc.r.max(c.r), acc.g.max(c.g), acc.b.max(c.b))
        })
    }

    /// Color en la coordenada `(u, v)`, con `v = 0` abajo.
    pub fn sample(&self, u: f32, v: f32) -> Color {
        // Una coordenada NaN no debe indexar fuera de rango; se trata como
        // el origen de la textura.
        let u = if u.is_finite() {
            self.wrap.apply(u)
        } else {
            0.0
        };
        let v = if v.is_finite() {
            self.wrap.apply(v)
        } else {
            0.0
        };

        // `u = 1.0` exacto daría `x = width`; de ahí el recorte al último
        // índice válido.
        let x = ((u * self.width as f32) as usize).min(self.width - 1);
        let fila = (((1.0 - v) * self.height as f32) as usize).min(self.height - 1);

        self.pixels[fila * self.width + x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Textura 2 x 2 con un color por cuadrante, en orden de lectura:
    /// arriba-izquierda, arriba-derecha, abajo-izquierda, abajo-derecha.
    fn cuatro_cuadrantes() -> Texture {
        Texture::from_pixels(
            2,
            2,
            vec![
                Color::new(1.0, 0.0, 0.0), // rojo    arriba-izquierda
                Color::new(0.0, 1.0, 0.0), // verde   arriba-derecha
                Color::new(0.0, 0.0, 1.0), // azul    abajo-izquierda
                Color::new(1.0, 1.0, 0.0), // amarillo abajo-derecha
            ],
        )
        .expect("2x2 con cuatro pixeles")
    }

    fn ruta_temporal(nombre: &str) -> PathBuf {
        std::env::temp_dir().join(format!("continente-tex-{}-{}", std::process::id(), nombre))
    }

    #[test]
    fn las_cuatro_esquinas_dan_el_color_esperado() {
        // Con `Clamp`, porque `u = 1.0` es un borde y no una costura. Ver
        // el test siguiente para lo que hace `Repeat` en ese mismo punto.
        let tex = cuatro_cuadrantes().with_wrap(WrapMode::Clamp);

        // v = 0 abajo, v = 1 arriba.
        assert_eq!(tex.sample(0.0, 0.0), Color::new(0.0, 0.0, 1.0), "abajo-izq");
        assert_eq!(tex.sample(1.0, 0.0), Color::new(1.0, 1.0, 0.0), "abajo-der");
        assert_eq!(
            tex.sample(0.0, 1.0),
            Color::new(1.0, 0.0, 0.0),
            "arriba-izq"
        );
        assert_eq!(
            tex.sample(1.0, 1.0),
            Color::new(0.0, 1.0, 0.0),
            "arriba-der"
        );
    }

    #[test]
    fn en_repeat_la_coordenada_uno_es_la_costura_y_vuelve_al_origen() {
        // `1.0.rem_euclid(1.0)` da 0.0, no 1.0: en un mosaico, u = 1 es
        // donde empieza la siguiente repeticion. Es correcto y conviene
        // tenerlo presente al escribir UV a mano, porque el borde derecho
        // de la textura solo se alcanza con valores estrictamente menores
        // que uno.
        let tex = cuatro_cuadrantes().with_wrap(WrapMode::Repeat);

        assert_eq!(tex.sample(1.0, 0.5), tex.sample(0.0, 0.5));
        assert_eq!(tex.sample(0.5, 1.0), tex.sample(0.5, 0.0));

        // El borde derecho se alcanza justo por debajo de uno.
        assert_eq!(tex.sample(0.999, 0.25), Color::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn v_cero_esta_abajo_no_arriba() {
        // Si esta convencion se invirtiera, toda textura saldria del reves
        // sobre las caras del cuboide.
        let tex = cuatro_cuadrantes();

        let abajo = tex.sample(0.25, 0.1);
        let arriba = tex.sample(0.25, 0.9);

        assert_eq!(abajo, Color::new(0.0, 0.0, 1.0));
        assert_eq!(arriba, Color::new(1.0, 0.0, 0.0));
        assert_ne!(abajo, arriba);
    }

    #[test]
    fn el_modo_repeat_envuelve_las_coordenadas() {
        let tex = cuatro_cuadrantes().with_wrap(WrapMode::Repeat);

        // 1.25 equivale a 0.25; -0.75 tambien.
        assert_eq!(tex.sample(1.25, 0.25), tex.sample(0.25, 0.25));
        assert_eq!(tex.sample(-0.75, 0.25), tex.sample(0.25, 0.25));
        assert_eq!(tex.sample(3.25, 0.25), tex.sample(0.25, 0.25));
    }

    #[test]
    fn el_modo_clamp_se_queda_en_el_borde() {
        let tex = cuatro_cuadrantes().with_wrap(WrapMode::Clamp);

        assert_eq!(tex.sample(5.0, 0.25), tex.sample(1.0, 0.25));
        assert_eq!(tex.sample(-3.0, 0.25), tex.sample(0.0, 0.25));
    }

    #[test]
    fn repeat_y_clamp_difieren_fuera_de_rango() {
        let repite = cuatro_cuadrantes().with_wrap(WrapMode::Repeat);
        let recorta = cuatro_cuadrantes().with_wrap(WrapMode::Clamp);

        // A u = 1.25, Repeat cae en el cuadrante izquierdo y Clamp en el
        // derecho.
        assert_ne!(repite.sample(1.25, 0.25), recorta.sample(1.25, 0.25));
    }

    #[test]
    fn rem_euclid_maneja_negativos_donde_el_resto_normal_falla() {
        // `-0.25 % 1.0` da -0.25 en Rust, que quedaria fuera de rango.
        assert!((WrapMode::Repeat.apply(-0.25) - 0.75).abs() < 1e-6);
        assert!((WrapMode::Repeat.apply(-1.25) - 0.75).abs() < 1e-6);
        assert!((0.0..1.0).contains(&WrapMode::Repeat.apply(-1000.3)));
    }

    #[test]
    fn una_coordenada_no_finita_no_indexa_fuera_de_rango() {
        let tex = cuatro_cuadrantes();

        // No debe entrar en panico.
        let _ = tex.sample(f32::NAN, 0.5);
        let _ = tex.sample(0.5, f32::INFINITY);
        let _ = tex.sample(f32::NEG_INFINITY, f32::NAN);
    }

    #[test]
    fn from_pixels_rechaza_dimensiones_incoherentes() {
        assert!(Texture::from_pixels(0, 4, vec![]).is_none());
        assert!(Texture::from_pixels(2, 2, vec![Color::black()]).is_none());
        assert!(Texture::from_pixels(2, 2, vec![Color::black(); 5]).is_none());
        assert!(Texture::from_pixels(2, 2, vec![Color::black(); 4]).is_some());
    }

    #[test]
    fn carga_un_png_desde_disco() {
        let ruta = ruta_temporal("carga.png");

        // Un PNG 2 x 2 con los cuatro cuadrantes conocidos.
        let mut imagen = image::RgbImage::new(2, 2);
        imagen.put_pixel(0, 0, image::Rgb([255, 0, 0]));
        imagen.put_pixel(1, 0, image::Rgb([0, 255, 0]));
        imagen.put_pixel(0, 1, image::Rgb([0, 0, 255]));
        imagen.put_pixel(1, 1, image::Rgb([255, 255, 0]));
        imagen.save(&ruta).expect("escribir el PNG de prueba");

        let tex = Texture::load(&ruta).expect("deberia cargar");

        assert_eq!(tex.width(), 2);
        assert_eq!(tex.height(), 2);
        // Y respeta la misma convencion de v que la textura en memoria.
        // Los valores llegan decodificados: 255 sRGB es 1.0 lineal, y 0 es 0.
        assert_eq!(tex.sample(0.25, 0.75), Color::new(1.0, 0.0, 0.0));
        assert_eq!(tex.sample(0.25, 0.25), Color::new(0.0, 0.0, 1.0));

        let _ = std::fs::remove_file(&ruta);
    }

    #[test]
    fn una_textura_ausente_da_error_con_la_ruta_y_no_un_color() {
        let ruta = ruta_temporal("no-existe.png");
        let _ = std::fs::remove_file(&ruta);

        let error = Texture::load(&ruta).expect_err("no deberia cargar");

        assert!(matches!(error, TextureError::NoEncontrada(_)));

        // El mensaje tiene que nombrar el archivo: es lo que convierte el
        // fallo en algo accionable en vez de una imagen rara.
        let mensaje = error.to_string();
        assert!(
            mensaje.contains("no-existe.png"),
            "el mensaje no nombra el archivo: {mensaje}"
        );
    }

    #[test]
    fn un_archivo_que_no_es_png_da_error_de_decodificacion() {
        let ruta = ruta_temporal("basura.png");
        std::fs::write(&ruta, b"esto no es un PNG").expect("escribir");

        let error = Texture::load(&ruta).expect_err("no deberia decodificar");

        assert!(matches!(error, TextureError::NoSePudoDecodificar { .. }));
        assert!(error.to_string().contains("basura.png"));

        let _ = std::fs::remove_file(&ruta);
    }

    #[test]
    fn el_muestreo_cubre_toda_la_textura_sin_salirse() {
        // Barrido denso: ninguna coordenada debe indexar fuera de rango ni
        // dejar un pixel del borde inalcanzable.
        let tex = Texture::from_pixels(
            5,
            3,
            (0..15)
                .map(|i| Color::new(i as f32 / 15.0, 0.0, 0.0))
                .collect(),
        )
        .expect("5x3");

        let mut vistos = std::collections::HashSet::new();
        for i in 0..=200 {
            for j in 0..=200 {
                let color = tex.sample(i as f32 / 200.0, j as f32 / 200.0);
                vistos.insert((color.r * 15.0).round() as i32);
            }
        }

        assert_eq!(
            vistos.len(),
            15,
            "no se alcanzaron los 15 pixeles: {vistos:?}"
        );
    }
}
