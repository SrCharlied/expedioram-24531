//! Preview reproducible de la paleta del pincel, sobre el diorama real.
//!
//! Produce dos PNG con la toma hero a `800 × 600`: la cápsula plegada y el
//! panel abierto, los dos con la herramienta **Cian** activa y con las
//! miniaturas muestreadas de las texturas cargadas.
//!
//! # Exige la feature
//!
//! ```bash
//! cargo run --release --features artistic-brush --example brush_palette_preview -- <carpeta>
//! ```
//!
//! Sin `artistic-brush` el ejemplo compila —para que `--all-targets` no se
//! rompa— y se limita a decir qué falta.
//!
//! El directorio de salida es **obligatorio**: este ejemplo no escribe
//! evidencia oficial por su cuenta.

#[cfg(not(feature = "artistic-brush"))]
fn main() -> std::process::ExitCode {
    eprintln!("brush_palette_preview exige la feature artistic-brush:");
    eprintln!(
        "  cargo run --release --features artistic-brush --example brush_palette_preview -- <carpeta>"
    );

    std::process::ExitCode::FAILURE
}

#[cfg(feature = "artistic-brush")]
fn main() -> std::process::ExitCode {
    imp::correr()
}

#[cfg(feature = "artistic-brush")]
mod imp {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::ExitCode;

    use expedition33_continente_inacabado::brush_palette::{
        dibujar_paleta, herramienta_de_tecla, Disposicion, Paleta, TEXTURAS,
    };
    use expedition33_continente_inacabado::framebuffer::Framebuffer;
    use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
    use expedition33_continente_inacabado::renderer::{render, Shading};
    use expedition33_continente_inacabado::reveal::RevealState;
    use expedition33_continente_inacabado::scenes::{safe_level_con, WaterPreset};

    const ANCHO: usize = 800;
    const ALTO: usize = 600;

    /// La herramienta que se muestra activa, por su tecla.
    ///
    /// Cian: un pigmento de la paleta, para que el realce de la celda activa
    /// se vea sobre una muestra de color y no sobre la de revelar.
    const ACTIVA: char = '7';

    fn guardar(framebuffer: &Framebuffer, destino: &Path, nombre: &str) -> bool {
        let ruta = destino.join(nombre);

        match framebuffer.save_png(&ruta) {
            Ok(()) => {
                println!("  escrito   {}", ruta.display());
                true
            }
            Err(e) => {
                eprintln!("error: no se pudo escribir {}: {e}", ruta.display());
                false
            }
        }
    }

    pub fn correr() -> ExitCode {
        let Some(destino) = std::env::args().nth(1).map(PathBuf::from) else {
            eprintln!("uso: ... --example brush_palette_preview -- <carpeta de salida>");
            eprintln!("  el directorio es obligatorio: este ejemplo no escribe evidencia oficial.");
            return ExitCode::FAILURE;
        };

        if let Err(e) = fs::create_dir_all(&destino) {
            eprintln!("error: no se pudo crear {}: {e}", destino.display());
            return ExitCode::FAILURE;
        }

        let diorama = match safe_level_con(WaterPreset::RefractiveWater, Some(&PathBuf::from(".")))
        {
            Ok(diorama) => diorama,
            Err(e) => {
                eprintln!("error: {e}");
                eprintln!("  las miniaturas se muestrean de las texturas reales.");
                eprintln!("  generalos con: cargo run --release --bin generate_assets");
                return ExitCode::FAILURE;
            }
        };

        let lights = luces_del_diorama(&diorama.anchors, &diorama.scale);
        let camara = diorama.hero_camera();
        let scene = diorama.scene;
        let accel = diorama.accel;

        let telas_disponibles: Vec<bool> = TEXTURAS
            .iter()
            .map(|tela| scene.textures.get(tela.indice).is_some())
            .collect();

        let mut paleta = Paleta::default();

        if let Some(herramienta) = herramienta_de_tecla(ACTIVA) {
            paleta.elegir(herramienta);
        }

        // El diorama revelado: la interfaz se juzga sobre la obra terminada,
        // que es el fondo más exigente que va a tener detrás.
        let mut fondo = Framebuffer::new(ANCHO, ALTO);
        render(
            &mut fondo,
            &scene,
            &accel,
            &lights,
            &RevealState::painted(),
            &camara,
            Shading::Material,
        );

        println!("brush_palette_preview\n");
        println!("  destino     {}", destino.display());
        println!("  dimension   {ANCHO} x {ALTO}, toma hero");
        println!("  objetos     {} primitivas", scene.objects.len());
        println!("  activa      {}", paleta.herramienta().nombre());
        println!(
            "  telas       {} de {} disponibles",
            telas_disponibles.iter().filter(|d| **d).count(),
            TEXTURAS.len()
        );

        for (abierta, nombre) in [(false, "palette-collapsed.png"), (true, "palette-open.png")] {
            let disposicion = Disposicion::calcular(abierta, ANCHO, ALTO, &telas_disponibles);

            // Se parte del mismo fondo cada vez: lo único que cambia entre
            // las dos imágenes es la interfaz.
            let mut cuadro = Framebuffer::new(ANCHO, ALTO);
            cuadro.buffer.copy_from_slice(&fondo.buffer);

            let mut vista = paleta;

            if abierta != vista.abierta() {
                vista.alternar();
            }

            dibujar_paleta(&mut cuadro, &disposicion, &vista, &scene.textures);

            println!(
                "  {} celdas en {}",
                disposicion.celdas.len(),
                if abierta { "abierta" } else { "plegada" }
            );

            if !guardar(&cuadro, &destino, nombre) {
                return ExitCode::FAILURE;
            }
        }

        println!("\n  Las dos comparten fondo, camara y resolucion: lo unico que cambia");
        println!("  es si el panel esta desplegado. Esto no es una aprobacion estetica.");

        ExitCode::SUCCESS
    }
}
