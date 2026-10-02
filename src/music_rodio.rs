//! La salida de música real de la ventana, con `rodio`.
//!
//! Vive en el binario y no en la librería: es lo único que toca un
//! dispositivo de audio, y así ni los tests de la librería, ni los ejemplos,
//! ni el render headless abren uno. El estado y su control son de
//! `music::Musica`, probados sin audio.
//!
//! La decodificación no ocurre en el ciclo del raytracer: `rodio` la hace en
//! el hilo de audio, a medida que el mezclador pide muestras. Aquí solo se
//! abre el archivo y se lee su cabecera al arrancar.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use expedition33_continente_inacabado::music::SalidaDeMusica;
use rodio::decoder::LoopedDecoder;
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player};

/// La música en bucle sobre el dispositivo por defecto.
///
/// El orden de los campos importa: el reproductor se suelta antes que el
/// dispositivo, y `Drop` lo detiene de forma explícita.
pub struct SalidaRodio {
    reproductor: Player,
    _dispositivo: MixerDeviceSink,
}

/// El MP3 como fuente que vuelve a empezar al terminar.
pub fn fuente_en_bucle(ruta: &Path) -> Result<LoopedDecoder<BufReader<File>>, String> {
    let archivo =
        File::open(ruta).map_err(|e| format!("no se pudo abrir {}: {e}", ruta.display()))?;
    let bytes = archivo
        .metadata()
        .map_err(|e| format!("no se pudo leer {}: {e}", ruta.display()))?
        .len();

    // Lo mismo que `Decoder::try_from(File)` —longitud y búsqueda, que es
    // lo que deja volver al principio—, más la pista del formato.
    Decoder::builder()
        .with_data(BufReader::new(archivo))
        .with_byte_len(bytes)
        .with_seekable(true)
        .with_hint("mp3")
        .build_looped()
        .map_err(|e| format!("no se pudo decodificar {}: {e}", ruta.display()))
}

impl SalidaRodio {
    /// Abre el archivo y el dispositivo por defecto y deja la música
    /// **pausada** con su volumen: la arranca `Musica::con_salida`.
    ///
    /// El archivo va primero, para que un MP3 ausente se diga como tal y no
    /// como un problema del dispositivo.
    pub fn abrir(ruta: &Path, volumen: f32) -> Result<SalidaRodio, String> {
        let fuente = fuente_en_bucle(ruta)?;

        // Un error del flujo a mitad de la obra —un auricular que se
        // desconecta— se avisa una vez, no en cada llamada del hilo de audio.
        let avisado = Arc::new(AtomicBool::new(false));
        let al_fallar = move |e: rodio::cpal::StreamError| {
            if !avisado.swap(true, Ordering::Relaxed) {
                eprintln!("  aviso: error en la salida de audio ({e}); la musica puede cortarse");
            }
        };

        let mut dispositivo = DeviceSinkBuilder::from_default_device()
            .map_err(|e| format!("sin dispositivo de audio: {e}"))?
            .with_error_callback(al_fallar)
            .open_sink_or_fallback()
            .map_err(|e| format!("no se pudo abrir el audio: {e}"))?;

        // Cerrar la ventana corta la música; no hace falta anunciarlo.
        dispositivo.log_on_drop(false);

        let reproductor = Player::connect_new(dispositivo.mixer());
        reproductor.pause();
        reproductor.set_volume(volumen);
        reproductor.append(fuente);

        Ok(SalidaRodio {
            reproductor,
            _dispositivo: dispositivo,
        })
    }

    /// Cuánto lleva reproducido, según `rodio`. Para el smoke de
    /// dispositivo; no dice si se oye.
    #[allow(dead_code)]
    pub fn posicion(&self) -> Duration {
        self.reproductor.get_pos()
    }
}

impl SalidaDeMusica for SalidaRodio {
    fn reanudar(&mut self) {
        self.reproductor.play();
    }

    fn pausar(&mut self) {
        self.reproductor.pause();
    }
}

impl Drop for SalidaRodio {
    fn drop(&mut self) {
        self.reproductor.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expedition33_continente_inacabado::music::{RUTA_DE_LA_MUSICA, VOLUMEN_POR_DEFECTO};
    use rodio::Source;
    use std::time::Instant;

    fn ruta() -> &'static Path {
        Path::new(RUTA_DE_LA_MUSICA)
    }

    /// El MP3 lo aporta el usuario y no está versionado: en un clon sin él,
    /// los tests que lo decodifican **lo dicen y se saltan** en vez de
    /// fallar. Con el archivo presente se ejecutan enteros.
    fn sin_mp3() -> bool {
        let falta = !ruta().exists();
        if falta {
            eprintln!("SALTADO: falta {RUTA_DE_LA_MUSICA} (no versionado)");
        }
        falta
    }

    /// El MP3 real se decodifica entero, sin dispositivo. Se informan sus
    /// datos; la duración sale de contar muestras, no de la cabecera.
    #[test]
    fn el_mp3_real_se_decodifica_entero() {
        if sin_mp3() {
            return;
        }
        let archivo = File::open(ruta()).expect("el MP3 tiene que estar en exp33/");
        let inicio = Instant::now();
        let decoder = Decoder::try_from(archivo).expect("decodificable");
        let (frecuencia, canales) = (decoder.sample_rate().get(), decoder.channels().get());
        let cabecera = decoder.total_duration();
        let muestras = decoder.count();
        let segundos = muestras as f64 / (frecuencia as f64 * canales as f64);
        println!(
            "mp3: {frecuencia} Hz, {canales} canales, {muestras} muestras = {segundos:.2} s; \
             duracion de cabecera {cabecera:?}; decodificado en {:.2} s",
            inicio.elapsed().as_secs_f64()
        );
        assert!(frecuencia >= 8_000 && (1..=2).contains(&canales));
        assert!(segundos > 30.0, "{segundos}");
    }

    /// La fuente en bucle sigue dando muestras después del final del
    /// archivo: vuelve a empezar.
    #[test]
    fn el_bucle_sigue_despues_del_final() {
        if sin_mp3() {
            return;
        }
        let una_vez = Decoder::try_from(File::open(ruta()).unwrap())
            .unwrap()
            .count();
        let bucle = fuente_en_bucle(ruta()).expect("bucle");
        let un_segundo = (bucle.sample_rate().get() * bucle.channels().get() as u32) as usize;
        let tomadas = bucle.take(una_vez + un_segundo).count();
        assert_eq!(tomadas, una_vez + un_segundo);
    }

    /// Un archivo ausente o que no es audio da un error con su ruta, sin
    /// pánico y sin tocar ningún dispositivo.
    #[test]
    fn un_archivo_ausente_o_invalido_da_error_sin_panico() {
        let ausente = fuente_en_bucle(Path::new("exp33/no_existe.mp3"))
            .err()
            .unwrap();
        assert!(ausente.contains("no_existe.mp3"), "{ausente}");
        let invalido = fuente_en_bucle(Path::new("Cargo.toml")).err().unwrap();
        assert!(invalido.contains("Cargo.toml"), "{invalido}");
        // Y `abrir` falla igual, antes de llegar al dispositivo.
        let abrir = SalidaRodio::abrir(Path::new("exp33/no_existe.mp3"), VOLUMEN_POR_DEFECTO);
        assert!(abrir.is_err());
    }

    /// **Smoke de dispositivo**, explícito: `cargo test -- --ignored smoke`.
    /// Abre el dispositivo real a volumen `0` —no suena— y comprueba que el
    /// reproductor avanza. No dice nada de si se oye.
    #[test]
    #[ignore = "abre el dispositivo de audio real"]
    fn smoke_dispositivo_real_a_volumen_cero() {
        let inicio = Instant::now();
        let sonda = fuente_en_bucle(ruta())
            .map(|_| inicio.elapsed())
            .expect("mp3");
        let inicio = Instant::now();
        let mut salida = SalidaRodio::abrir(ruta(), 0.0).expect("dispositivo");
        let apertura = inicio.elapsed();
        println!("smoke: sonda del mp3 sola {:.3} s", sonda.as_secs_f64());
        salida.reanudar();
        std::thread::sleep(Duration::from_millis(1500));
        let avance = salida.posicion();
        salida.pausar();
        std::thread::sleep(Duration::from_millis(300));
        let en_pausa = salida.posicion();
        std::thread::sleep(Duration::from_millis(500));
        println!(
            "smoke: apertura {:.3} s, avance tras 1.5 s {:.3} s, pausa {:.3} -> {:.3} s",
            apertura.as_secs_f64(),
            avance.as_secs_f64(),
            en_pausa.as_secs_f64(),
            salida.posicion().as_secs_f64()
        );
        assert!(avance > Duration::from_millis(500), "{avance:?}");
        assert!(salida.posicion() <= en_pausa + Duration::from_millis(100));
    }
}
