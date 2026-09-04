//! Genera estados visuales representativos de la línea temporal de
//! revelación.
//!
//! ```text
//! cargo run --release --example demo_timeline
//! ```
//!
//! # Lo que es, y lo que NO es
//!
//! Es un **generador de estados**: mueve `RevealState` con un reloj
//! sintético y renderiza en los puntos que importan. Sirve para ver que la
//! interpolación produce estados coherentes y que el Monolito arranca solo,
//! y sale idéntico en cada corrida.
//!
//! **No es una reproducción de la ventana.** No pasa por `demo_action` ni
//! por `pick_region`, no procesa eventos de `minifb`, no ejerce el
//! antirrebote del botón, no usa `plan_frame` ni el perfil interactivo
//! —renderiza siempre a `800 x 600`—, no prueba `L` ni `R`, y usa un tiempo
//! por cuadro fijo en vez de medir el de la máquina.
//!
//! Lo que sí atraviesa esa integración es `tests/demo_completa.rs` para el
//! estado, y el recorrido humano para el resto.

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

/// Tiempo por cuadro del perfil interactivo, **registrado**.
///
/// No es la cifra que usa la ventana: esa se **mide al arrancar**, en la
/// máquina que corre. Esta es una medición archivada, y está aquí para que
/// esta línea de tiempo salga idéntica en cada corrida.
///
/// Se rederiva con `cargo run --release --example interactive_frame_time`.
/// Es la cifra de la **toma hero**, que es el encuadre con el que la
/// ventana se calibra al arrancar. Otras cámaras alcanzables cuestan mucho
/// más —el zoom más cercano llega a `0.26 s`— y eso no cabe en una
/// constante: vive en el presupuesto de la matriz.
///
/// Procedencia: `400 x 300`, preset refractivo, `RevealState::worst_case()`
/// en la toma hero, mediana de quince rondas intercaladas y rotadas;
/// **árbol de trabajo sin commitear** sobre `2c7960a`, 4 de septiembre de
/// 2026, Ryzen 7 6800H, rustc 1.97.0. Se rederiva con
/// `cargo run --release --example interactive_frame_time`.
///
/// # Las dos versiones anteriores
///
/// `0.0524` salía del peor de `reveal 0.0` y `reveal 1.0`, y los dos
/// extremos son justo los que evitan el doble muestreo de texturas.
/// `0.0820` ya medía el estado correcto, pero con una mediana mal calculada
/// —el mayor de los dos centrales de un conteo par— y sin rotar el orden de
/// la ronda, que favorece a los primeros puntos. Ver la Tarea 7.1.
const FRAME_TIME: f32 = 0.0736;

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

        // Un intermedio del finale: es lo que respalda que el Monolito
        // **se interpola** y no aparece de golpe. Sin el, la secuencia solo
        // demuestra extremos coherentes.
        if cuadros == 15 {
            println!("      Monolito a medio pintar, cuadro 15");
            dibujar(&reveal, "5-monolito-a-medias");
        }
    }

    println!("  6 · Monolito pintado en {cuadros} cuadros");
    dibujar(&reveal, "6-monolito");

    println!(
        "\n  progreso global final {:.2}, todas las regiones pintadas: {}",
        reveal.global_progress(),
        reveal.all_regions_painted()
    );
}
