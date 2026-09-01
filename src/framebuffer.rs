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
}
