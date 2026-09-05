//! Rinde lo que el usuario **ve** con cada perfil interactivo, para poder
//! juzgar la mitigación de la Tarea 7.1 con los ojos y no solo con la tabla.
//!
//! ```text
//! cargo run --release --example profile_preview
//! ```
//!
//! # Por qué ampliado y no a su tamaño
//!
//! Un PNG de `320 × 240` visto al `100 %` en un visor no es lo que el
//! usuario ve: la ventana escala ese cuadro a `800 × 600` por vecino más
//! cercano mientras algo se mueve. Comparar el archivo pequeño contra el
//! grande diría que el perfil bajo se ve *más nítido*, que es exactamente lo
//! contrario de lo que pasa en pantalla.
//!
//! Así que los dos perfiles se guardan **ya ampliados** con
//! `Framebuffer::blit_upscaled`, el mismo camino que dibuja la ventana, y
//! junto al cuadro final a resolución completa que sirve de referencia.
//!
//! # Los dos encuadres
//!
//! La toma hero, que es lo que se presenta, y el peor encuadre alcanzable
//! —el zoom más cercano mirando a la bahía—, que es donde la mitigación
//! decide. El segundo importa más de lo que parece: el perfil bajo se nota
//! sobre todo en los detalles finos, y ahí es donde están el barco, la
//! cadena y el ancla.

use std::path::{Path, PathBuf};

use expedition33_continente_inacabado::camera::Camera;
use expedition33_continente_inacabado::framebuffer::Framebuffer;
use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
use expedition33_continente_inacabado::renderer::{render, InteractiveProfile, Shading};
use expedition33_continente_inacabado::reveal::RevealState;
use expedition33_continente_inacabado::scene_builder::Blockout;
use expedition33_continente_inacabado::scenes::{safe_level_con, WaterPreset};

const ANCHO: usize = 800;
const ALTO: usize = 600;

const SALIDA: &str = "evidence/hito7";

fn nivel_texturizado() -> Blockout {
    let raiz = PathBuf::from(".");

    match safe_level_con(WaterPreset::RefractiveWater, Some(&raiz)) {
        Ok(nivel) => nivel,
        Err(e) => {
            eprintln!("error: {e}");
            eprintln!("  esta comparacion es de calidad de imagen; sin texturas no dice nada.");
            eprintln!("  generalos con: cargo run --release --bin generate_assets");
            std::process::exit(1);
        }
    }
}

/// Guarda, o aborta con la ruta.
///
/// Sin fallback silencioso: un generador de evidencia que termina en éxito
/// sin haber escrito el archivo es el fallo que corrigió el Hito 5.
fn guardar(framebuffer: &Framebuffer, ruta: &str) {
    if let Err(e) = framebuffer.save_png(Path::new(ruta)) {
        eprintln!("error: no se pudo guardar {ruta}: {e}");
        std::process::exit(1);
    }

    println!("  {ruta}");
}

fn main() {
    let diorama = nivel_texturizado();
    let luces = luces_del_diorama(&diorama.anchors, &diorama.scale);

    // El peor encuadre de la rejilla, buscado por su etiqueta y no por una
    // posición: si la rejilla cambia, esto sigue apuntando al mismo sitio o
    // falla diciéndolo.
    let camaras = diorama.measurement_cameras();
    let peor: &(String, Camera) = camaras
        .iter()
        .find(|(etiqueta, _)| etiqueta == "y+0 e+35 cerca")
        .expect("la rejilla tiene que contener el zoom cercano de la toma hero");

    let encuadres = [("hero", &diorama.hero_camera()), ("cerca", &peor.1)];

    // El estado más caro, que además es el que más detalle fino tiene en
    // pantalla: el Continente pintado con el Monolito a medio revelar.
    let reveal = RevealState::worst_case();

    println!("profile_preview · que se ve con cada perfil\n");
    println!("  escena   safe-refractive-water, worst_case()");
    println!("  ampliado con blit_upscaled, el mismo camino que dibuja la ventana");
    println!(
        "  perfil por defecto: {} x {}\n",
        InteractiveProfile::default().width,
        InteractiveProfile::default().height
    );

    if let Err(e) = std::fs::create_dir_all(SALIDA) {
        eprintln!("error: no se pudo crear {SALIDA}: {e}");
        std::process::exit(1);
    }

    for (nombre_encuadre, camara) in encuadres {
        // La referencia: el cuadro final, que es lo que se ve en reposo.
        let mut completo = Framebuffer::new(ANCHO, ALTO);
        render(
            &mut completo,
            &diorama.scene,
            &diorama.accel,
            &luces,
            &reveal,
            camara,
            Shading::Material,
        );
        guardar(&completo, &format!("{SALIDA}/{nombre_encuadre}-final.png"));

        for (nombre_perfil, perfil) in [
            ("media", InteractiveProfile::MEDIA),
            ("baja", InteractiveProfile::BAJA),
        ] {
            let mut borrador = Framebuffer::new(perfil.width, perfil.height);
            render(
                &mut borrador,
                &diorama.scene,
                &diorama.accel,
                &luces,
                &reveal,
                camara,
                Shading::Material,
            );

            let mut ampliado = Framebuffer::new(ANCHO, ALTO);
            ampliado.blit_upscaled(&borrador);

            guardar(
                &ampliado,
                &format!("{SALIDA}/{nombre_encuadre}-{nombre_perfil}.png"),
            );
        }
    }

    println!("\n  Comparar por pares el mismo encuadre: -final es la referencia en");
    println!("  reposo, -media y -baja son lo que se ve mientras algo se mueve.");
    println!("  La pregunta no es si se pierde detalle —se pierde—, sino si se");
    println!("  pierde el que hace falta para orbitar y apuntar un clic.");
}
