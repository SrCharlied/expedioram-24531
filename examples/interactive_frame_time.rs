//! Mide `interactive_frame_time`, el número del que sale la duración de la
//! revelación.
//!
//! ```text
//! cargo run --release --example interactive_frame_time
//! ```
//!
//! # Por qué hace falta un probe propio
//!
//! `0.0490` está escrito en cuatro sitios —la ventana, la línea de tiempo y
//! dos suites de tests— y de él depende `reveal_duration`. Los tests prueban
//! la aritmética **alrededor** de esa cifra; ninguno prueba que la cifra
//! siga siendo cierta.
//!
//! `interactive_probe` no sirve para esto: usa `InteriorVisible` en vez del
//! preset refractivo, mide una sola pasada en vez de una distribución, y no
//! separa el lienzo del estado pintado.
//!
//! Este mide el caso real y registra con qué:
//!
//! - `400 x 300`, el perfil interactivo.
//! - `RefractiveWater`, el preset canónico.
//! - Assets cargados, o aborta.
//! - `reveal 0.0` y `reveal 1.0`, porque **no cuestan lo mismo**.
//! - Quince repeticiones, con mínimo, mediana y máximo.

use std::path::PathBuf;
use std::time::Instant;

use expedition33_continente_inacabado::framebuffer::Framebuffer;
use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
use expedition33_continente_inacabado::renderer::{render, InteractiveProfile, Shading};
use expedition33_continente_inacabado::reveal::{
    reveal_duration, RevealState, MINIMUM_REVEAL_FRAMES, REVEAL_DURATION_CEILING,
};
use expedition33_continente_inacabado::scene_builder::Blockout;
use expedition33_continente_inacabado::scenes::{safe_level_con, WaterPreset};

const REPETICIONES: usize = 15;

fn nivel_texturizado() -> Blockout {
    let raiz = PathBuf::from(".");

    match safe_level_con(WaterPreset::RefractiveWater, Some(&raiz)) {
        Ok(nivel) => nivel,
        Err(e) => {
            eprintln!("error: {e}");
            eprintln!("  esta medicion es la que fija la duracion de la revelacion");
            eprintln!("  y sin texturas mediria otra escena.");
            eprintln!("  generalos con: cargo run --release --bin generate_assets");
            std::process::exit(1);
        }
    }
}

/// Mínimo, mediana y máximo de una distribución de tiempos.
fn distribucion(mut tiempos: Vec<f64>) -> (f64, f64, f64) {
    tiempos.sort_by(|a, b| a.partial_cmp(b).expect("no hay NaN"));

    (
        tiempos[0],
        tiempos[tiempos.len() / 2],
        tiempos[tiempos.len() - 1],
    )
}

fn main() {
    let diorama = nivel_texturizado();
    let luces = luces_del_diorama(&diorama.anchors, &diorama.scale);
    let camara = diorama.hero_camera();
    let perfil = InteractiveProfile::MEDIA;

    println!("interactive_frame_time · perfil interactivo\n");
    println!("  perfil      {} x {}", perfil.width, perfil.height);
    println!("  preset      safe-refractive-water");
    println!(
        "  escena      {} primitivas, {} texturas, {} luces",
        diorama.scene.objects.len(),
        diorama.scene.textures.len(),
        luces.len()
    );
    println!("  camara      toma hero");
    println!("  repeticiones {REPETICIONES}");
    println!("  comando     cargo run --release --example interactive_frame_time");

    println!(
        "\n  {:<14} {:>10} {:>10} {:>10}",
        "reveal", "minimo", "mediana", "maximo"
    );

    let mut peor_mediana = 0.0_f64;

    for (nombre, estado) in [
        ("0.0 lienzo", RevealState::unpainted()),
        ("1.0 pintado", RevealState::painted()),
    ] {
        let mut framebuffer = Framebuffer::new(perfil.width, perfil.height);
        let mut tiempos = Vec::with_capacity(REPETICIONES);

        for _ in 0..REPETICIONES {
            let inicio = Instant::now();
            render(
                &mut framebuffer,
                &diorama.scene,
                &diorama.accel,
                &luces,
                &estado,
                &camara,
                Shading::Material,
            );
            tiempos.push(inicio.elapsed().as_secs_f64());
        }

        let (minimo, mediana, maximo) = distribucion(tiempos);
        peor_mediana = peor_mediana.max(mediana);

        println!("  {nombre:<14} {minimo:>10.4} {mediana:>10.4} {maximo:>10.4}");
    }

    // La derivación toma el **peor** de los dos: el lienzo no lanza rayos
    // secundarios y sale más barato, así que garantizar los quince cuadros
    // con su tiempo dejaría el final de la transición sin margen.
    let medido = peor_mediana as f32;

    println!("\n  interactive_frame_time = {medido:.4} s   (la peor de las dos medianas)");

    match reveal_duration(medido) {
        Ok(duracion) => {
            let critico = REVEAL_DURATION_CEILING / MINIMUM_REVEAL_FRAMES;

            println!("  reveal_duration        = {duracion:.2} s");
            println!("  cuadros de transicion  = {:.0}", duracion / medido);
            println!(
                "  margen al critico      = {:.1}x   (critico {critico:.4} s)",
                critico / medido
            );
        }
        Err(fallo) => {
            println!(
                "  FALLA el gate de fluidez: quince cuadros exigen {:.2} s y el techo son {:.2} s",
                fallo.required, REVEAL_DURATION_CEILING
            );
            println!("  baja la resolucion del perfil en vez de alargar la animacion");
            std::process::exit(1);
        }
    }

    println!("\n  Registrar junto a la cifra: commit, fecha, hardware y toolchain.");
    println!("  Un tiempo solo significa algo junto a los que se midieron con el.");
}
