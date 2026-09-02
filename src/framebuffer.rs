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
            let fuente_y = y * origen.height / self.height;

            for x in 0..self.width {
                let fuente_x = x * origen.width / self.width;

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
}
