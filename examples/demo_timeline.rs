//! Evidencia visual del gate del Hito 6: la demo, cuadro por hito.
//!
//! ```text
//! cargo run --release --example demo_timeline
//! ```
//!
//! Simula el recorrido de la presentación —lienzo, las tres regiones una a
//! una, el Monolito que arranca solo— con el mismo reloj y la misma
//! velocidad derivada que usa la ventana, y guarda un render en cada
//! momento que importa.
//!
//! Es la evidencia que acompaña al recorrido humano, no su sustituto: lo que
//! no se puede comprobar aquí es que el ratón apunte donde el usuario cree.

use std::path::PathBuf;

use expedition33_continente_inacabado::framebuffer::Framebuffer;
use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
use expedition33_continente_inacabado::renderer::{render, Shading};
use expedition33_continente_inacabado::reveal::{
    reveal_duration, reveal_speed, RevealPhase, RevealState,
};
use expedition33_continente_inacabado::scene::RevealGroup;
use expedition33_continente_inacabado::scene_builder::Blockout;
use expedition33_continente_inacabado::scenes::{safe_level_con, WaterPreset};

const ANCHO: usize = 800;
const ALTO: usize = 600;

/// Tiempo por cuadro del perfil interactivo, medido en release. El mismo
/// que usa la ventana.
const FRAME_TIME: f32 = 0.0490;

/// Carga el nivel con los assets, o aborta.
///
/// Sin fallback: los PNG de esta línea de tiempo son evidencia visual, y
/// generarlos con colores planos mostraría otra escena que la documentada.
fn nivel_texturizado() -> Blockout {
    let raiz = PathBuf::from(".");

    match safe_level_con(WaterPreset::RefractiveWater, Some(&raiz)) {
        Ok(nivel) => nivel,
        Err(e) => {
            eprintln!("error: {e}");
            eprintln!("  esta linea de tiempo es evidencia visual y exige los assets.");
            eprintln!("  generalos con: cargo run --release --bin generate_assets");
            std::process::exit(1);
        }
    }
}

fn guardar(framebuffer: &Framebuffer, nombre: &str) {
    let destino = PathBuf::from("evidence/hito6").join(format!("{nombre}.png"));

    if let Err(error) = framebuffer.save_png(&destino) {
        eprintln!("error: no se pudo escribir {}: {error}", destino.display());
        std::process::exit(1);
    }

    println!("      {}", destino.display());
}

fn main() {
    let diorama = nivel_texturizado();
    let luces = luces_del_diorama(&diorama.anchors, &diorama.scale);
    let camara = diorama.hero_camera();

    let duracion = match reveal_duration(FRAME_TIME) {
        Ok(duracion) => duracion,
        Err(fallo) => {
            eprintln!(
                "error: el perfil falla el gate de fluidez: {:.4} s por cuadro exigen {:.2} s",
                fallo.interactive_frame_time, fallo.required
            );
            std::process::exit(1);
        }
    };
    let velocidad = reveal_speed(duracion);

    println!("Linea de tiempo de la demo · gate del Hito 6");
    println!("  duracion {duracion:.2} s por region, {FRAME_TIME:.4} s por cuadro");

    let mut framebuffer = Framebuffer::new(ANCHO, ALTO);
    let mut reveal = RevealState::unpainted();
    let mut dibujar = |reveal: &RevealState, nombre: &str| {
        render(
            &mut framebuffer,
            &diorama.scene,
            &diorama.accel,
            &luces,
            reveal,
            &camara,
            Shading::Material,
        );
        guardar(&framebuffer, nombre);
    };

    println!("\n  0 · el lienzo");
    dibujar(&reveal, "0-lienzo");

    // Las tres regiones, una a una, con un cuadro intermedio de la primera
    // para dejar constancia de que la transicion existe.
    for (indice, (grupo, nombre)) in [
        (RevealGroup::Meadows, "praderas"),
        (RevealGroup::Breakwater, "rompeolas"),
        (RevealGroup::FlyingWaters, "aguas"),
    ]
    .into_iter()
    .enumerate()
    {
        reveal.activate(grupo);

        let mut cuadros = 0;

        while reveal.phase(grupo) == RevealPhase::Revealing {
            reveal.advance(FRAME_TIME, velocidad);
            cuadros += 1;

            // De la primera region se guarda tambien el punto medio.
            if indice == 0 && cuadros == 15 {
                println!("  1 · praderas a medio pintar, cuadro 15");
                dibujar(&reveal, "1-praderas-a-medias");
            }
        }

        println!("  {} · {nombre} listas en {cuadros} cuadros", indice + 2);
        dibujar(&reveal, &format!("{}-{nombre}", indice + 2));
    }

    // El Monolito arranca solo. Un tick lo activa.
    reveal.advance(FRAME_TIME, velocidad);

    if reveal.phase(RevealGroup::Finale) != RevealPhase::Revealing {
        eprintln!("error: el Monolito no arranco al completarse el Continente");
        std::process::exit(1);
    }

    println!("  5 · el Monolito arranco solo");

    let mut cuadros = 1;

    while reveal.phase(RevealGroup::Finale) == RevealPhase::Revealing {
        reveal.advance(FRAME_TIME, velocidad);
        cuadros += 1;
    }

    println!("  6 · Monolito pintado en {cuadros} cuadros");
    dibujar(&reveal, "6-monolito");

    println!(
        "\n  progreso global final {:.2}, todas las regiones pintadas: {}",
        reveal.global_progress(),
        reveal.all_regions_painted()
    );
}
