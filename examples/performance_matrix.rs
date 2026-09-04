//! Matriz de rendimiento de la Tarea 7.1: cuánto presupuesto existe de
//! verdad antes de gastar la reserva del Hito 7.
//!
//! ```text
//! cargo run --release --example performance_matrix
//! ```
//!
//! # Los cinco presets del plan
//!
//! El plan nombra cinco presets y no los define. Cada fila declara aquí su
//! definición exacta —volumen de agua y estado de revelación— porque son
//! las dos dimensiones que mueven el coste, y confundirlas es lo que dio un
//! benchmark optimista en el Hito 3:
//!
//! | Nombre del plan | Volumen | Revelación |
//! |---|---|---|
//! | `safe-canvas` | sin volumen, `159` primitivas | lienzo, `0.0` |
//! | `safe-painted` | sin volumen, `159` primitivas | pintado, `1.0` |
//! | `safe-water` | refractivo, `160` primitivas | pintado, `1.0` |
//! | `safe-revealing` | refractivo, `160` primitivas | **el peor cuadro** |
//! | `target-water` | nivel objetivo | pintado, `1.0` |
//!
//! Las filas están ordenadas de forma que cada una añade **un** coste sobre
//! la anterior: primero las texturas del estado pintado, luego la óptica del
//! volumen, luego el doble muestreo de la transición, y al final la
//! densidad del nivel objetivo. Así la diferencia entre dos filas
//! consecutivas es atribuible.
//!
//! `target-water` **no se mide**: el nivel objetivo no existe todavía —es la
//! Tarea 7.2, y decidir si se construye es justo para lo que sirve esta
//! matriz—. La fila se imprime como pendiente en vez de omitirse, para que
//! quien lea la tabla vea que falta y no que no hacía falta.
//!
//! # Método
//!
//! Release, o la comparación no significa nada. Dos resoluciones: la
//! ventana completa y el perfil interactivo, que son los dos regímenes
//! reales del programa.
//!
//! Las filas se **intercalan por ronda**: un cuadro de cada fila, y otra
//! vuelta. En esta máquina el estado térmico se mueve dentro de una sola
//! corrida, así que agotar una fila antes de pasar a la siguiente le carga
//! el calentamiento a la que tocara y las diferencias dejan de ser
//! atribuibles.
//!
//! Se reportan mínimo, mediana y máximo: la mediana dimensiona y el mínimo
//! compara. Los valores absolutos de las dos resoluciones **no** son
//! comparables entre sí —la fase de resolución completa deja la máquina
//! caliente para la siguiente—; lo comparable es lo de dentro de cada
//! bloque, que es donde está el intercalado.

use std::path::PathBuf;
use std::time::Instant;

use expedition33_continente_inacabado::framebuffer::Framebuffer;
use expedition33_continente_inacabado::light::{diorama as luces_del_diorama, PointLight};
use expedition33_continente_inacabado::renderer::{render, InteractiveProfile, Shading};
use expedition33_continente_inacabado::reveal::{
    reveal_duration, RevealState, MINIMUM_REVEAL_FRAMES, REVEAL_DURATION_CEILING,
    WORST_CASE_PROGRESS,
};
use expedition33_continente_inacabado::scene_builder::Blockout;
use expedition33_continente_inacabado::scenes::{safe_level_con, WaterPreset};

const ANCHO: usize = 800;
const ALTO: usize = 600;

/// Cuadros por celda, cada uno en una ronda distinta.
const MUESTRAS: usize = 14;

/// Una fila de la matriz, con su distribución de tiempos por resolución.
struct Fila {
    nombre: &'static str,
    /// Índice del blockout: `0` sin volumen, `1` refractivo.
    nivel: usize,
    reveal: RevealState,
    tiempos: Vec<f64>,
}

impl Fila {
    fn nueva(nombre: &'static str, nivel: usize, reveal: RevealState) -> Self {
        Fila {
            nombre,
            nivel,
            reveal,
            tiempos: Vec::with_capacity(MUESTRAS),
        }
    }

    fn distribucion(&self) -> (f64, f64, f64) {
        let mut t = self.tiempos.clone();
        t.sort_by(|a, b| a.partial_cmp(b).expect("no hay NaN"));

        (t[0], t[t.len() / 2], t[t.len() - 1])
    }
}

fn nivel(preset: WaterPreset) -> Blockout {
    let raiz = PathBuf::from(".");

    match safe_level_con(preset, Some(&raiz)) {
        Ok(nivel) => nivel,
        Err(e) => {
            eprintln!("error: {e}");
            eprintln!("  la matriz mide la escena texturizada; sin assets mediria otra.");
            eprintln!("  generalos con: cargo run --release --bin generate_assets");
            std::process::exit(1);
        }
    }
}

/// Mide las filas a una resolución, con pasadas intercaladas.
fn medir(
    filas: &mut [Fila],
    niveles: &[Blockout],
    luces: &[Vec<PointLight>],
    ancho: usize,
    alto: usize,
) {
    let mut framebuffer = Framebuffer::new(ancho, alto);

    for fila in filas.iter_mut() {
        fila.tiempos.clear();
    }

    for _ in 0..MUESTRAS {
        for fila in filas.iter_mut() {
            let diorama = &niveles[fila.nivel];
            let camara = diorama.hero_camera();

            let inicio = Instant::now();
            render(
                &mut framebuffer,
                &diorama.scene,
                &diorama.accel,
                &luces[fila.nivel],
                &fila.reveal,
                &camara,
                Shading::Material,
            );
            fila.tiempos.push(inicio.elapsed().as_secs_f64());
        }
    }
}

fn reportar(filas: &[Fila], niveles: &[Blockout], ancho: usize, alto: usize, titulo: &str) {
    println!("\n  {titulo}   {ancho} x {alto}");
    println!(
        "  {:<18} {:>6} {:>9} {:>9} {:>9} {:>7}",
        "preset", "prim.", "minimo", "mediana", "maximo", "fps"
    );

    for fila in filas {
        let (minimo, mediana, maximo) = fila.distribucion();
        let fps = 1.0 / mediana;

        println!(
            "  {:<18} {:>6} {minimo:>9.4} {mediana:>9.4} {maximo:>9.4} {fps:>7.1}",
            fila.nombre,
            niveles[fila.nivel].scene.objects.len()
        );
    }

    println!(
        "  {:<18} {:>6} {:>9} {:>9} {:>9} {:>7}",
        "target-water", "?", "-", "-", "-", "-"
    );
    println!("    pendiente: el nivel objetivo es la Tarea 7.2 y no existe todavia.");
}

fn main() {
    let niveles = [
        nivel(WaterPreset::InteriorVisible),
        nivel(WaterPreset::RefractiveWater),
    ];
    let luces: Vec<Vec<PointLight>> = niveles
        .iter()
        .map(|n| luces_del_diorama(&n.anchors, &n.scale))
        .collect();

    let perfil = InteractiveProfile::MEDIA;

    let mut filas = vec![
        Fila::nueva("safe-canvas", 0, RevealState::unpainted()),
        Fila::nueva("safe-painted", 0, RevealState::painted()),
        Fila::nueva("safe-water", 1, RevealState::painted()),
        Fila::nueva("safe-revealing", 1, RevealState::worst_case()),
    ];

    println!("performance_matrix · Tarea 7.1\n");
    println!("  release     si, obligatorio");
    println!(
        "  escena      {} texturas, {} luces",
        niveles[1].scene.textures.len(),
        luces[1].len()
    );
    println!("  camara      toma hero");
    println!("  muestras    {MUESTRAS} por celda, intercaladas por ronda");
    println!("  peor cuadro Continente pintado y grupo Finale en {WORST_CASE_PROGRESS:.2}");
    println!("  comando     cargo run --release --example performance_matrix");

    medir(&mut filas, &niveles, &luces, ANCHO, ALTO);
    reportar(&filas, &niveles, ANCHO, ALTO, "cuadro final");
    let final_medianas: Vec<f64> = filas.iter().map(|f| f.distribucion().1).collect();

    medir(&mut filas, &niveles, &luces, perfil.width, perfil.height);
    reportar(
        &filas,
        &niveles,
        perfil.width,
        perfil.height,
        "perfil interactivo",
    );
    let interactivas: Vec<f64> = filas.iter().map(|f| f.distribucion().1).collect();

    // ------------------------------------------------ el presupuesto
    //
    // El gate que puede fallar de verdad es el de fluidez, y solo mira el
    // perfil interactivo: el cuadro final se produce una vez al soltar los
    // controles y nadie lo anima.
    let critico = REVEAL_DURATION_CEILING / MINIMUM_REVEAL_FRAMES;
    let peor = interactivas[3];

    println!("\n  presupuesto");
    println!("  peor estado interactivo    {peor:.4} s   (safe-revealing)");
    println!("  critico del gate           {critico:.4} s   (15 cuadros en {REVEAL_DURATION_CEILING:.1} s)");
    println!(
        "  reserva                    {:.1}x en tiempo",
        critico as f64 / peor
    );

    match reveal_duration(peor as f32) {
        Ok(duracion) => println!(
            "  reveal_duration            {duracion:.2} s   ({:.0} cuadros)",
            duracion / peor as f32
        ),
        Err(_) => {
            println!("  FALLA el gate de fluidez con el peor estado.");
            std::process::exit(1);
        }
    }

    println!("\n  lo que cuesta cada cosa, en el perfil interactivo");
    println!(
        "  texturas del estado pintado  {:+.1} %",
        100.0 * (interactivas[1] / interactivas[0] - 1.0)
    );
    println!(
        "  volumen refractivo           {:+.1} %",
        100.0 * (interactivas[2] / interactivas[1] - 1.0)
    );
    println!(
        "  doble muestreo de la transicion {:+.1} %",
        100.0 * (interactivas[3] / interactivas[2] - 1.0)
    );

    println!("\n  y en el cuadro final");
    println!(
        "  texturas del estado pintado  {:+.1} %",
        100.0 * (final_medianas[1] / final_medianas[0] - 1.0)
    );
    println!(
        "  volumen refractivo           {:+.1} %",
        100.0 * (final_medianas[2] / final_medianas[1] - 1.0)
    );
    println!(
        "  doble muestreo de la transicion {:+.1} %",
        100.0 * (final_medianas[3] / final_medianas[2] - 1.0)
    );

    println!("\n  La reserva esta en tiempo, no en primitivas: el coste no es lineal");
    println!("  en el conteo —la jerarquia poda el 92 % de los tests del Hito 3—, asi");
    println!("  que traducirla a densidad exige medir el nivel objetivo, no dividir.");
    println!("\n  Registrar junto a las cifras: commit, fecha, hardware y toolchain.");
}
