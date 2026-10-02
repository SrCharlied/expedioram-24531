//! Estado de la música de fondo, sin audio.
//!
//! La ventana suena con `exp33/musica_mundo_e33.mp3` en bucle y un único
//! control: la bocina, que la silencia o la reanuda. Aquí solo vive **qué
//! estado tiene** y qué se le pide a la salida en cada transición; abrir el
//! dispositivo y decodificar es cosa del binario (`src/music_rodio.rs`),
//! así que este módulo se prueba sin altavoces y sin hilo de audio.
//!
//! Lo que el estado **no** prueba es que algo suene: eso solo lo puede
//! decir alguien con el sonido de Windows encendido.

/// El archivo, relativo a la raíz del proyecto, igual que las texturas.
pub const RUTA_DE_LA_MUSICA: &str = "exp33/musica_mundo_e33.mp3";

/// Volumen de arranque, como factor sobre la señal decodificada. Moderado:
/// es fondo, no tiene que tapar a quien presente la obra.
pub const VOLUMEN_POR_DEFECTO: f32 = 0.25;

/// Lo que la música le pide a un reproductor. La implementación real es la
/// del binario; los tests usan una que solo anota las llamadas.
pub trait SalidaDeMusica {
    fn reanudar(&mut self);
    fn pausar(&mut self);
}

/// Lo que muestra la bocina.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EstadoMusica {
    Sonando,
    Silenciada,
    /// No se pudo abrir el archivo, decodificarlo o abrir el dispositivo.
    /// La obra sigue; la bocina se muestra apagada y no hace nada.
    NoDisponible,
}

/// La música y su único control.
pub struct Musica<S: SalidaDeMusica> {
    salida: Option<S>,
    activa: bool,
}

impl<S: SalidaDeMusica> Musica<S> {
    /// Con una salida ya abierta: arranca sonando.
    pub fn con_salida(mut salida: S) -> Self {
        salida.reanudar();

        Musica {
            salida: Some(salida),
            activa: true,
        }
    }

    /// Sin salida: la música no está disponible.
    pub fn no_disponible() -> Self {
        Musica {
            salida: None,
            activa: false,
        }
    }

    /// Desde el resultado de abrir la salida. En el fallo devuelve el aviso
    /// que hay que imprimir **una vez**; la obra sigue sin música.
    pub fn desde(resultado: Result<S, String>) -> (Self, Option<String>) {
        match resultado {
            Ok(salida) => (Musica::con_salida(salida), None),
            Err(motivo) => (
                Musica::no_disponible(),
                Some(format!(
                    "  aviso: sin musica ({motivo}); la obra sigue en silencio"
                )),
            ),
        }
    }

    pub fn estado(&self) -> EstadoMusica {
        match (&self.salida, self.activa) {
            (None, _) => EstadoMusica::NoDisponible,
            (Some(_), true) => EstadoMusica::Sonando,
            (Some(_), false) => EstadoMusica::Silenciada,
        }
    }

    /// Silencia o reanuda. Devuelve si el estado cambió: sin música
    /// disponible no cambia nada.
    pub fn alternar(&mut self) -> bool {
        let Some(salida) = self.salida.as_mut() else {
            return false;
        };

        if self.activa {
            salida.pausar();
        } else {
            salida.reanudar();
        }
        self.activa = !self.activa;

        true
    }

    /// La salida, para quien necesite consultarla.
    pub fn salida(&self) -> Option<&S> {
        self.salida.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Salida falsa: anota lo que se le pide. No reproduce nada y no lo
    /// pretende; solo permite comprobar las transiciones.
    #[derive(Default)]
    struct Anotadora {
        llamadas: Vec<&'static str>,
    }

    impl SalidaDeMusica for Anotadora {
        fn reanudar(&mut self) {
            self.llamadas.push("reanudar");
        }
        fn pausar(&mut self) {
            self.llamadas.push("pausar");
        }
    }

    #[test]
    fn la_ruta_y_el_volumen_son_los_acordados() {
        assert_eq!(RUTA_DE_LA_MUSICA, "exp33/musica_mundo_e33.mp3");
        assert!((VOLUMEN_POR_DEFECTO - 0.25).abs() < 1e-6);
    }

    #[test]
    fn con_salida_arranca_sonando_y_le_pide_reanudar_una_vez() {
        let m = Musica::con_salida(Anotadora::default());
        assert_eq!(m.estado(), EstadoMusica::Sonando);
        assert_eq!(m.salida().unwrap().llamadas, ["reanudar"]);
    }

    #[test]
    fn alternar_silencia_y_reanuda_en_la_salida() {
        let mut m = Musica::con_salida(Anotadora::default());
        assert!(m.alternar());
        assert_eq!(m.estado(), EstadoMusica::Silenciada);
        assert!(m.alternar());
        assert_eq!(m.estado(), EstadoMusica::Sonando);
        assert!(m.alternar());
        assert_eq!(m.estado(), EstadoMusica::Silenciada);
        assert_eq!(
            m.salida().unwrap().llamadas,
            ["reanudar", "pausar", "reanudar", "pausar"]
        );
    }

    #[test]
    fn sin_salida_no_esta_disponible_y_alternar_no_hace_nada() {
        let mut m: Musica<Anotadora> = Musica::no_disponible();
        assert_eq!(m.estado(), EstadoMusica::NoDisponible);
        assert!(!m.alternar());
        assert_eq!(m.estado(), EstadoMusica::NoDisponible);
        assert!(m.salida().is_none());
    }

    #[test]
    fn un_fallo_al_abrir_deja_la_musica_no_disponible_con_un_aviso() {
        let (m, aviso) = Musica::<Anotadora>::desde(Err("sin dispositivo".to_string()));
        assert_eq!(m.estado(), EstadoMusica::NoDisponible);
        let aviso = aviso.expect("hay que avisar del fallo");
        assert!(aviso.contains("sin dispositivo"), "{aviso}");
        assert!(aviso.contains("sin musica"), "{aviso}");
    }

    #[test]
    fn abrir_bien_no_avisa_y_arranca_sonando() {
        let (m, aviso) = Musica::desde(Ok(Anotadora::default()));
        assert!(aviso.is_none());
        assert_eq!(m.estado(), EstadoMusica::Sonando);
    }
}
