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
//! | `safe-revealing` | refractivo, `160` primitivas | `worst_case()` |
//! | `target-water` | refractivo, `175` primitivas | pintado, `1.0` |
//!
//! Y una sexta que el plan no nombra y la Tarea 7.2 necesita:
//! `target-revealing`, el candidato en el peor estado. Es la fila que decide,
//! porque es la que se compara contra el gate.
//!
//! Las filas están ordenadas de forma que cada una añade **un** cambio sobre
//! la anterior: primero los materiales pintados, luego el volumen
//! refractivo, luego el doble muestreo de la transición, y al final la
//! densidad del nivel objetivo. Así la diferencia entre dos filas
//! consecutivas es atribuible a un solo cambio.
//!
//! Un escalón mal nombrado, corregido por la columna de rayos: el primero
//! **no es «las texturas»**. Al pasar de lienzo a pintado se encienden a la
//! vez las texturas y los techos ópticos de los materiales finales, y los
//! conteos dicen cuál pesa: los rayos secundarios por cuadro pasan de `597`
//! a `8 819`. Es la óptica, no el muestreo.
//!
//! Los dos escalones que **no** compran rayos son los que lo confirman por el
//! otro lado: el doble muestreo de la transición añade `231` rayos y un
//! `2 %` a `6 %` de tiempo, y las quince primitivas del lote de la Tarea 7.2
//! añaden **cero** —son lecho, kelp y roca, materiales sin techos— y un
//! `7 %`. El conteo de primitivas y el conteo de rayos son dos presupuestos
//! distintos, y el caro es el segundo.
//!
//! `target-water` ya se mide: existe desde el primer lote incremental de la
//! Tarea 7.2 —`+15` primitivas, todas dentro de la bahía—. **No es lo que se
//! envía**: el nivel seguro sigue intacto en `160` y es el que abre la
//! ventana. El candidato vive para poder medirlo y mirarlo antes de decidir
//! si se conserva, y para poder retirarlo cambiando un parámetro.
//!
//! # Método
//!
//! Release, o la comparación no significa nada.
//!
//! **Tres bloques.** El cuadro final a `800 x 600` y el perfil interactivo con
//! la resolución que el programa envía, que son los dos regímenes reales,
//! los dos en la toma hero para que las filas sean comparables entre sí. Y
//! un tercero con la **rejilla de cuarenta y ocho cámaras** sobre la fila que
//! decide, porque un presupuesto medido en un solo encuadre promete un margen
//! que el primer giro puede gastarse.
//!
//! **Rondas intercaladas y rotadas.** Un cuadro de cada celda por ronda, y
//! el orden de la ronda rota. Intercalar reparte la deriva térmica; rotar
//! reparte la posición dentro de la ronda, que no es neutra. Sin rotación la
//! instrumentación favorece sistemáticamente a las primeras filas, que es
//! precisamente el sesgo que esta versión corrige.
//!
//! **Estadística.** Quince rondas y `stats::summarize`, que calcula bien la
//! mediana de un conteo par. Las atribuciones entre filas usan
//! `stats::median_ratio`: cociente ronda contra ronda y mediana de los
//! cocientes, no cociente de medianas.
//!
//! **Conteos de rayos.** Junto a cada celda se registran los rayos
//! secundarios por cuadro. Sin ellos, decir *por qué* una fila cuesta más
//! que la anterior sería una hipótesis; con ellos es una medición.
//!
//! Los valores absolutos de bloques distintos **no** son comparables entre
//! sí: cada bloque deja la máquina más caliente para el siguiente. Lo
//! comparable es lo de dentro de un bloque, que es donde está el intercalado.

use std::path::PathBuf;
use std::time::Instant;

use expedition33_continente_inacabado::accel::TraversalStats;
use expedition33_continente_inacabado::camera::Camera;
use expedition33_continente_inacabado::framebuffer::Framebuffer;
use expedition33_continente_inacabado::light::{diorama as luces_del_diorama, PointLight};
use expedition33_continente_inacabado::renderer::{render, InteractiveProfile, Shading};
use expedition33_continente_inacabado::reveal::{
    reveal_duration, RevealState, MINIMUM_REVEAL_FRAMES, REVEAL_DURATION_CEILING,
    WORST_CASE_PROGRESS,
};
use expedition33_continente_inacabado::scene_builder::Blockout;
use expedition33_continente_inacabado::scenes::{
    safe_level_con, target_level_con, Density, WaterPreset,
};
use expedition33_continente_inacabado::stats::{median_ratio, summarize};

const ANCHO: usize = 800;
const ALTO: usize = 600;

/// Rondas por celda. **Impar**: la mediana es un valor observado.
const RONDAS: usize = 15;

/// Margen mínimo que se le exige al candidato sobre el crítico del gate.
///
/// Es el umbral operativo de la Tarea 7.1, y se mantiene para la 7.2 por una
/// razón que va más allá de este lote: **quedan lotes posteriores**, y el
/// renderer mide `render()`, no todo el coste de presentar un cuadro. Gastar
/// la reserva hasta el borde en el primer lote dejaría el segundo sin sitio y
/// el gate sin colchón para lo que la medición no ve.
const MARGEN_MINIMO: f64 = 1.30;

/// Una celda de la matriz: un nivel, un estado, una cámara y su muestra.
struct Celda {
    nombre: String,
    /// Índice del blockout: `0` sin volumen, `1` refractivo.
    nivel: usize,
    reveal: RevealState,
    camara: Camera,
    tiempos: Vec<f64>,
    /// Contadores del último cuadro trazado. Son deterministas para un
    /// estado y una cámara dados, así que uno basta.
    stats: TraversalStats,
}

impl Celda {
    fn nueva(nombre: String, nivel: usize, reveal: RevealState, camara: Camera) -> Self {
        Celda {
            nombre,
            nivel,
            reveal,
            camara,
            tiempos: Vec::with_capacity(RONDAS),
            stats: TraversalStats::default(),
        }
    }

    /// Rayos secundarios por cuadro.
    fn secundarios(&self) -> usize {
        self.stats.reflection_rays + self.stats.refraction_rays
    }
}

fn nivel(preset: WaterPreset, densidad: Density) -> Blockout {
    let raiz = PathBuf::from(".");

    let construido = match densidad {
        Density::Safe => safe_level_con(preset, Some(&raiz)),
        Density::Target => target_level_con(preset, Some(&raiz)),
    };

    match construido {
        Ok(nivel) => nivel,
        Err(e) => {
            eprintln!("error: {e}");
            eprintln!("  la matriz mide la escena texturizada; sin assets mediria otra.");
            eprintln!("  generalos con: cargo run --release --bin generate_assets");
            std::process::exit(1);
        }
    }
}

/// Mide las celdas, un cuadro de cada una por ronda, **rotando** el orden.
///
/// `tiempos[r]` queda siendo la ronda `r` para todas las celdas, que es lo
/// que hace válidos los cocientes pareados.
fn medir(
    celdas: &mut [Celda],
    niveles: &[Blockout],
    luces: &[Vec<PointLight>],
    ancho: usize,
    alto: usize,
) {
    let mut framebuffer = Framebuffer::new(ancho, alto);
    let n = celdas.len();

    for celda in celdas.iter_mut() {
        celda.tiempos.clear();
    }

    for ronda in 0..RONDAS {
        for k in 0..n {
            let i = (k + ronda) % n;
            let diorama = &niveles[celdas[i].nivel];

            let inicio = Instant::now();
            let stats = render(
                &mut framebuffer,
                &diorama.scene,
                &diorama.accel,
                &luces[celdas[i].nivel],
                &celdas[i].reveal,
                &celdas[i].camara,
                Shading::Material,
            );
            celdas[i].tiempos.push(inicio.elapsed().as_secs_f64());
            celdas[i].stats = stats;
        }
    }
}

fn reportar(celdas: &[Celda], niveles: &[Blockout], ancho: usize, alto: usize, titulo: &str) {
    println!("\n  {titulo}   {ancho} x {alto}");
    println!(
        "  {:<18} {:>6} {:>9} {:>9} {:>9} {:>6} {:>12}",
        "celda", "prim.", "minimo", "mediana", "maximo", "fps", "2os rayos"
    );

    for celda in celdas {
        let d = summarize(&celda.tiempos);

        println!(
            "  {:<18} {:>6} {:>9.4} {:>9.4} {:>9.4} {:>6.1} {:>12}",
            celda.nombre,
            niveles[celda.nivel].scene.objects.len(),
            d.min,
            d.median,
            d.max,
            1.0 / d.median,
            celda.secundarios()
        );
    }
}

/// Imprime los escalones entre filas consecutivas, con cociente pareado.
fn escalones(celdas: &[Celda], etiqueta: &str) {
    println!("\n  escalones {etiqueta} (cociente pareado, ronda contra ronda)");

    for par in celdas.windows(2) {
        let (antes, despues) = (&par[0], &par[1]);
        let cociente = median_ratio(&despues.tiempos, &antes.tiempos);
        let rayos = despues.secundarios() as i64 - antes.secundarios() as i64;

        println!(
            "  {:<18} -> {:<18} {:+6.1} %   2os rayos {rayos:+}",
            antes.nombre,
            despues.nombre,
            100.0 * (cociente - 1.0)
        );
    }
}

fn main() {
    let niveles = [
        nivel(WaterPreset::InteriorVisible, Density::Safe),
        nivel(WaterPreset::RefractiveWater, Density::Safe),
        nivel(WaterPreset::RefractiveWater, Density::Target),
    ];
    let luces: Vec<Vec<PointLight>> = niveles
        .iter()
        .map(|n| luces_del_diorama(&n.anchors, &n.scale))
        .collect();

    // El perfil que envia el programa. Ver el bloque de mitigaciones.
    let perfil = InteractiveProfile::default();
    let hero = niveles[1].hero_camera();

    let presets = || {
        vec![
            Celda::nueva("safe-canvas".to_string(), 0, RevealState::unpainted(), hero),
            Celda::nueva("safe-painted".to_string(), 0, RevealState::painted(), hero),
            Celda::nueva("safe-water".to_string(), 1, RevealState::painted(), hero),
            Celda::nueva(
                "safe-revealing".to_string(),
                1,
                RevealState::worst_case(),
                hero,
            ),
            Celda::nueva("target-water".to_string(), 2, RevealState::painted(), hero),
            Celda::nueva(
                "target-revealing".to_string(),
                2,
                RevealState::worst_case(),
                hero,
            ),
        ]
    };

    println!("performance_matrix · Tarea 7.1\n");
    println!("  release     si, obligatorio");
    println!(
        "  escena      {} texturas, {} luces",
        niveles[1].scene.textures.len(),
        luces[1].len()
    );
    println!("  rondas      {RONDAS}, intercaladas y con el orden rotado");
    println!("  peor estado Continente pintado y grupo Finale en {WORST_CASE_PROGRESS:.2}");
    println!("  comando     cargo run --release --example performance_matrix");

    // ---------------------------------------------- bloque 1: cuadro final
    let mut finales = presets();
    medir(&mut finales, &niveles, &luces, ANCHO, ALTO);
    reportar(&finales, &niveles, ANCHO, ALTO, "cuadro final, toma hero");
    escalones(&finales, "a resolucion completa");

    // ------------------------------------------ bloque 2: perfil interactivo
    let mut interactivas = presets();
    medir(
        &mut interactivas,
        &niveles,
        &luces,
        perfil.width,
        perfil.height,
    );
    reportar(
        &interactivas,
        &niveles,
        perfil.width,
        perfil.height,
        "perfil interactivo, toma hero",
    );
    escalones(&interactivas, "en el perfil interactivo");

    // ---------------------------------------------- bloque 3: las cámaras
    //
    // Sobre el **candidato**, que es la fila que decide: si el lote no cabe
    // en el peor encuadre, no cabe.
    let mut camaras: Vec<Celda> = niveles[2]
        .measurement_cameras()
        .into_iter()
        .map(|(etiqueta, camara)| Celda::nueva(etiqueta, 2, RevealState::worst_case(), camara))
        .collect();

    medir(&mut camaras, &niveles, &luces, perfil.width, perfil.height);
    reportar(
        &camaras,
        &niveles,
        perfil.width,
        perfil.height,
        "target-revealing en la rejilla de 48 camaras",
    );

    let peor_camara = camaras
        .iter()
        .max_by(|a, b| {
            summarize(&a.tiempos)
                .median
                .partial_cmp(&summarize(&b.tiempos).median)
                .expect("no hay NaN")
        })
        .expect("hay camaras");

    println!(
        "\n  peor camara  {}   {:.4} s   ({:+.1} % sobre la hero, pareado)",
        peor_camara.nombre,
        summarize(&peor_camara.tiempos).median,
        100.0 * (median_ratio(&peor_camara.tiempos, &camaras[0].tiempos) - 1.0)
    );

    // ------------------------------------------------ el presupuesto
    //
    // El gate que puede fallar de verdad es el de fluidez, y solo mira el
    // perfil interactivo: el cuadro final se produce una vez al soltar los
    // controles y nadie lo anima.
    //
    // Se toma el peor de los dos bloques interactivos, que es lo que hace
    // que el presupuesto no dependa de haber medido el encuadre afortunado.
    let critico = REVEAL_DURATION_CEILING / MINIMUM_REVEAL_FRAMES;
    let peor = summarize(&interactivas[5].tiempos)
        .median
        .max(summarize(&peor_camara.tiempos).median);

    // Lo que el lote costó, pareado y en la toma hero, que es donde las dos
    // filas son comparables.
    let coste_del_lote = median_ratio(&interactivas[5].tiempos, &interactivas[3].tiempos);

    println!("\n  el lote de la Tarea 7.2");
    println!(
        "  +15 primitivas cuestan     {:+.1} %   (target-revealing vs safe-revealing, pareado)",
        100.0 * (coste_del_lote - 1.0)
    );
    println!(
        "  rayos secundarios          {:+}",
        interactivas[5].secundarios() as i64 - interactivas[3].secundarios() as i64
    );

    println!("\n  presupuesto del candidato");
    println!("  peor cuadro interactivo    {peor:.4} s");
    println!(
        "  critico del gate           {critico:.4} s   (15 cuadros en {REVEAL_DURATION_CEILING:.1} s)"
    );
    let reserva = critico as f64 / peor;

    println!("  reserva                    {reserva:.2}x en tiempo");

    if reserva < MARGEN_MINIMO {
        println!(
            "\n  AVISO: el candidato deja {reserva:.2}x y el umbral operativo es {MARGEN_MINIMO:.2}x."
        );
        println!("  El lote hay que retirarlo o recortarlo.");
    } else {
        println!("  umbral operativo           {MARGEN_MINIMO:.2}x   ->  el lote cabe");
    }

    match reveal_duration(peor as f32) {
        Ok(duracion) => println!(
            "  reveal_duration            {duracion:.2} s   ({:.0} cuadros)",
            duracion / peor as f32
        ),
        Err(_) => {
            println!("  FALLA el gate de fluidez con el peor cuadro.");
            std::process::exit(1);
        }
    }

    println!("\n  La reserva esta en tiempo, no en primitivas: el coste no es lineal");
    println!("  en el conteo, asi que traducirla a densidad exige medir el nivel");
    println!("  objetivo. Y el escalon mas caro de la tabla no es el conteo sino la");
    println!("  optica: mirar la columna de rayos secundarios junto a los tiempos.");
    println!("\n  Registrar junto a las cifras: commit, fecha, arbol, hardware y toolchain.");
}
