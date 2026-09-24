use image::{ImageFormat, RgbImage};
use std::error::Error;
use std::fs;
use std::path::Path;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

/// Píxel fuente que el escalado muestra en un píxel de destino.
///
/// Es la **única** definición del mapeo, y por eso es pública: la usan
/// `blit_upscaled` para dibujar y el picking para resolver un clic contra lo
/// que se está mostrando. Dos copias de esta división se desincronizan, y el
/// síntoma sería un clic que elige lo que hay un píxel al lado.
///
/// División entera a propósito: es vecino más cercano por truncamiento, así
/// que un bloque de destino entero muestra el **mismo** píxel fuente. Ese
/// bloque es lo que el usuario ve como un solo punto de color, y es la razón
/// de que el picking tenga que pasar por aquí en vez de interpolar.
pub fn source_pixel(destino: usize, lado_destino: usize, lado_fuente: usize) -> usize {
    if lado_destino == 0 {
        return 0;
    }

    destino * lado_fuente / lado_destino
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            background_color: 0x000000,
            current_color: 0xFFFFFF,
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = self.current_color;
        }
    }

    /// Rellena un rectángulo, recortado contra el buffer.
    ///
    /// Existe para la interfaz del pincel, que se dibuja **encima** del
    /// cuadro ya trazado. Recorta en vez de exigir que quepa: un panel
    /// anclado a una esquina puede quedar a caballo del borde en una ventana
    /// pequeña, y eso tiene que recortarse, no reventar.
    ///
    /// Un rectángulo sin área no pinta nada.
    pub fn fill_rect(&mut self, x: usize, y: usize, ancho: usize, alto: usize, color: u32) {
        if ancho == 0 || alto == 0 || x >= self.width || y >= self.height {
            return;
        }

        let hasta_x = (x + ancho).min(self.width);
        let hasta_y = (y + alto).min(self.height);

        for fila in y..hasta_y {
            for columna in x..hasta_x {
                self.buffer[fila * self.width + columna] = color;
            }
        }
    }

    /// Dibuja el **marco** de un rectángulo, de un píxel de grosor.
    ///
    /// Recorta igual que `fill_rect`: los lados que caen fuera del buffer
    /// simplemente no se dibujan.
    pub fn stroke_rect(&mut self, x: usize, y: usize, ancho: usize, alto: usize, color: u32) {
        if ancho == 0 || alto == 0 {
            return;
        }

        // Arriba y abajo.
        self.fill_rect(x, y, ancho, 1, color);
        self.fill_rect(x, y + alto - 1, ancho, 1, color);

        // Izquierda y derecha.
        self.fill_rect(x, y, 1, alto, color);
        self.fill_rect(x + ancho - 1, y, 1, alto, color);
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    /// Escribe el framebuffer como PNG, creando los directorios que falten.
    ///
    /// El buffer guarda `0x00RRGGBB` empaquetado, asi que hay que
    /// desempacarlo a tres bytes por pixel. El canal alfa se descarta: el
    /// diorama es opaco y un PNG RGB pesa un cuarto menos.
    pub fn save_png(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        if let Some(directorio) = path.parent() {
            if !directorio.as_os_str().is_empty() {
                fs::create_dir_all(directorio)?;
            }
        }

        let mut rgb = Vec::with_capacity(self.width * self.height * 3);
        for pixel in &self.buffer {
            rgb.push(((pixel >> 16) & 0xFF) as u8);
            rgb.push(((pixel >> 8) & 0xFF) as u8);
            rgb.push((pixel & 0xFF) as u8);
        }

        let imagen = RgbImage::from_raw(self.width as u32, self.height as u32, rgb)
            .ok_or("el buffer no coincide con las dimensiones declaradas")?;

        imagen.save_with_format(path, ImageFormat::Png)?;

        Ok(())
    }

    /// Copia un framebuffer más pequeño sobre este, escalándolo por vecino
    /// más cercano.
    ///
    /// Vecino más cercano y no interpolación bilineal a propósito: el
    /// diorama es de caras planas y aristas duras, y suavizar al escalar
    /// emborronaría precisamente los bordes que dan la lectura de volumen.
    /// Además cuesta una indexación por píxel, que es lo que se puede pagar
    /// en un cuadro que ya va contrarreloj.
    ///
    /// Funciona con cualquier proporción entre origen y destino; no exige
    /// que una sea múltiplo entero de la otra.
    pub fn blit_upscaled(&mut self, origen: &Framebuffer) {
        if origen.width == 0 || origen.height == 0 || self.width == 0 || self.height == 0 {
            return;
        }

        for y in 0..self.height {
            let fuente_y = source_pixel(y, self.height, origen.height);

            for x in 0..self.width {
                let fuente_x = source_pixel(x, self.width, origen.width);

                self.buffer[y * self.width + x] = origen.buffer[fuente_y * origen.width + fuente_x];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn con_patron(ancho: usize, alto: usize) -> Framebuffer {
        let mut fb = Framebuffer::new(ancho, alto);

        for y in 0..alto {
            for x in 0..ancho {
                fb.set_current_color((y * ancho + x) as u32);
                fb.point(x, y);
            }
        }

        fb
    }

    #[test]
    fn escalar_conserva_las_dimensiones_del_destino() {
        let origen = con_patron(4, 3);
        let mut destino = Framebuffer::new(8, 6);

        destino.blit_upscaled(&origen);

        assert_eq!(destino.width, 8);
        assert_eq!(destino.height, 6);
        assert_eq!(destino.buffer.len(), 48);
    }

    #[test]
    fn al_doble_cada_pixel_se_convierte_en_un_bloque_de_dos_por_dos() {
        let origen = con_patron(2, 2);
        let mut destino = Framebuffer::new(4, 4);

        destino.blit_upscaled(&origen);

        // Esquina superior izquierda: el pixel 0 del origen.
        for (x, y) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
            assert_eq!(destino.buffer[y * 4 + x], 0, "({x}, {y})");
        }
        // Esquina inferior derecha: el pixel 3.
        for (x, y) in [(2, 2), (3, 2), (2, 3), (3, 3)] {
            assert_eq!(destino.buffer[y * 4 + x], 3, "({x}, {y})");
        }
    }

    #[test]
    fn al_mismo_tamano_es_una_copia_exacta() {
        let origen = con_patron(5, 4);
        let mut destino = Framebuffer::new(5, 4);

        destino.blit_upscaled(&origen);

        assert_eq!(destino.buffer, origen.buffer);
    }

    #[test]
    fn funciona_con_proporciones_no_enteras() {
        // 320x240 sobre 800x600 no es un multiplo entero, y es justo el
        // perfil mas agresivo que contempla el plan.
        let origen = con_patron(320, 240);
        let mut destino = Framebuffer::new(800, 600);

        destino.blit_upscaled(&origen);

        // Ningun pixel queda sin escribir y todos vienen del origen.
        assert!(destino.buffer.iter().all(|p| (*p as usize) < 320 * 240));
    }

    #[test]
    fn un_origen_vacio_no_entra_en_panico() {
        let origen = Framebuffer::new(0, 0);
        let mut destino = Framebuffer::new(4, 4);

        destino.blit_upscaled(&origen);

        assert_eq!(destino.buffer.len(), 16);
    }

    // --------------------------------------------- primitivas 2D de la UI

    #[test]
    fn un_rectangulo_relleno_cubre_exactamente_su_area() {
        let mut fb = Framebuffer::new(20, 10);
        fb.fill_rect(3, 2, 5, 4, 0x00FF0000);

        for y in 0..10 {
            for x in 0..20 {
                let dentro = (3..8).contains(&x) && (2..6).contains(&y);
                let pixel = fb.buffer[y * 20 + x];

                assert_eq!(
                    pixel != 0,
                    dentro,
                    "({x}, {y}) deberia estar {}",
                    if dentro { "dentro" } else { "fuera" }
                );
            }
        }
    }

    #[test]
    fn un_rectangulo_que_se_sale_se_recorta_sin_desbordar() {
        // Lo que este test existe para atrapar: una UI anclada a la esquina
        // con un panel un pixel mas grande que la ventana no puede escribir
        // fuera del buffer ni entrar en panico.
        let mut fb = Framebuffer::new(8, 6);

        fb.fill_rect(6, 4, 100, 100, 0x00FFFFFF);
        fb.fill_rect(0, 0, 1000, 1000, 0x0000FF00);
        fb.stroke_rect(7, 5, 50, 50, 0x000000FF);

        assert_eq!(fb.buffer.len(), 8 * 6, "el buffer cambio de tamano");
    }

    #[test]
    fn un_rectangulo_degenerado_no_pinta_nada() {
        let mut fb = Framebuffer::new(10, 10);
        let limpio = fb.buffer.clone();

        fb.fill_rect(2, 2, 0, 5, 0x00FF0000);
        fb.fill_rect(2, 2, 5, 0, 0x00FF0000);
        fb.stroke_rect(2, 2, 0, 0, 0x00FF0000);

        assert_eq!(fb.buffer, limpio, "un rectangulo sin area pinto algo");
    }

    #[test]
    fn un_borde_dibuja_el_marco_y_deja_el_interior() {
        let mut fb = Framebuffer::new(12, 12);
        fb.stroke_rect(2, 2, 6, 5, 0x0000FF00);

        // Las cuatro esquinas del marco.
        for (x, y) in [(2, 2), (7, 2), (2, 6), (7, 6)] {
            assert_ne!(fb.buffer[y * 12 + x], 0, "falta la esquina ({x}, {y})");
        }

        // Y el interior sigue vacio.
        assert_eq!(fb.buffer[4 * 12 + 4], 0, "el borde relleno el interior");
    }
}
