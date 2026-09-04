//! Mide `interactive_frame_time`: el cuadro **más caro que la demo puede
//! presentar**, que es el número del que sale la duración de la revelación.
//!
//! ```text
//! cargo run --release --example interactive_frame_time
//! ```
//!
//! # Por qué hace falta un probe propio
//!
//! De esta cifra depende `reveal_duration`, y de ella los quince cuadros de
//! la transición. Los tests prueban la aritmética **alrededor** del número;
//! ninguno prueba que el número siga siendo cierto.
//!
//! `interactive_probe` no sirve para esto: usa `InteriorVisible` en vez del
//! preset refractivo, mide una sola pasada en vez de una distribución, y no
//! separa los estados de revelación.
//!
//! # Lo que corrigió la Tarea 7.1
//!
//! La versión anterior medía dos estados —`reveal 0.0` y `reveal 1.0`— en la
//! toma hero, y se quedaba con el peor de los dos. Le faltaban las dos
//! dimensiones que mueven el coste.
//!
//! **La revelación.** `reveal::resolve` corta en `t <= 0` y en `t >= 1` con
//! un único muestreo de textura; en cualquier `t` intermedio muestrea las
//! dos y las mezcla. Los dos estados que se medían son, exactamente, los dos
//! que toman el atajo. Y el material de lienzo no tiene techos ópticos, así
//! que donde queda lienzo no se lanza un solo rayo secundario: el lienzo
//! entero es el estado barato, no una referencia neutra.
//!
//! **La cámara.** El coste de un cuadro depende de qué ocupa la pantalla, y
//! la bahía refractiva no cubre la misma fracción del cuadro desde todos los
//! ángulos. Medir en un solo encuadre promete un margen que el primer giro
//! puede gastarse. Ver `Blockout::measurement_cameras`.
//!
//! # Método
//!
//! `400 x 300` —el perfil interactivo—, preset `safe-refractive-water` con
//! los ocho assets cargados, o aborta: sin texturas no hay doble muestreo y
//! la medición perdería justo lo que busca.
//!
//! **Fase 1**, los estados de revelación en la toma hero. **Fase 2**, el
//! peor estado en las diez cámaras alcanzables. **Fase 3**, el cruce de los
//! dos perfiles interactivos con el mejor y el peor encuadre, que es lo que
//! hace falta para decidir un perfil y no solo para diagnosticar. Las tres
//! comparten tres decisiones de método:
//!
//! 1. **Un cuadro de cada punto por ronda, y el orden de la ronda rota.**
//!    Intercalar reparte la deriva térmica entre todos los puntos; rotar
//!    reparte también la **posición** dentro de la ronda, que no es neutra:
//!    con un orden fijo, el punto que va primero paga el arranque de cada
//!    ronda en todas las rondas. Sin rotación, la instrumentación favorece
//!    sistemáticamente a unos estados sobre otros.
//! 2. **La mediana se calcula bien.** Quince rondas, y `stats::summarize`
//!    promedia los dos centrales cuando el conteo es par. La primera versión
//!    tomaba `ordenadas[n / 2]` sobre treinta muestras, que es el mayor de
//!    los dos centrales: un sesgo pequeño, sistemático y siempre hacia
//!    arriba.
//! 3. **Los cocientes son pareados.** Para decir cuánto cuesta un estado
//!    frente a otro se divide ronda contra ronda y se toma la mediana de los
//!    cocientes, no el cociente de las medianas. Ver `stats::median_ratio`.
//!
//! # Lo que este ejemplo no puede decir
//!
//! Cuál es el **máximo global**. Dentro de la banda alta las diferencias son
//! menores que la dispersión de la máquina, y el orden de los puntos de esa
//! banda cambia entre corridas. Lo que sí resuelve son las bandas y el
//! escalón que las separa; `RevealState::worst_case()` es un **representante**
//! de la banda alta elegido por la estructura de la escena, no un máximo
//! demostrado.

use std::path::PathBuf;
use std::time::Instant;

use expedition33_continente_inacabado::camera::Camera;
use expedition33_continente_inacabado::framebuffer::Framebuffer;
use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
use expedition33_continente_inacabado::renderer::{render, InteractiveProfile, Shading};
use expedition33_continente_inacabado::reveal::{
    reveal_duration, RevealState, MINIMUM_REVEAL_FRAMES, REVEAL_DURATION_CEILING,
    WORST_CASE_PROGRESS,
};
use expedition33_continente_inacabado::scene::RevealGroup;
use expedition33_continente_inacabado::scene_builder::Blockout;
use expedition33_continente_inacabado::scenes::{safe_level_con, WaterPreset};
use expedition33_continente_inacabado::stats::{median_ratio, summarize, Distribution};

/// Rondas de medición. **Impar** a propósito: la mediana de un conteo impar
/// es un valor observado y no un promedio de dos.
const RONDAS: usize = 15;

/// Puntos del barrido de estados, con las tres regiones en el mismo
/// progreso.
///
/// Los extremos siguen midiéndose: son la referencia contra la que se lee
/// cuánto cuesta el estado intermedio. `0.02` está para ver dónde despega el
/// coste, que no es gradual —los rayos secundarios se encienden en cuanto
/// `t` deja de ser cero—.
const BARRIDO: [f32; 9] = [0.0, 0.02, 0.1, 0.25, 0.5, 0.75, 0.9, 0.98, 1.0];

/// El mismo barrido para el grupo `Finale`, sobre las tres regiones ya
/// pintadas.
///
/// Hace falta un barrido propio porque `Finale` **no es solo el Monolito**:
/// son sus diez masas y las diez del arco costero, que juntas ocupan casi
/// todo el cuadro de la toma hero. Los extremos se omiten porque ya están
/// medidos con otro nombre: `finale 0.0` es `regiones 1.00` y `finale 1.0`
/// es el estado pintado completo.
const BARRIDO_FINALE: [f32; 7] = [0.02, 0.1, 0.25, 0.5, 0.75, 0.9, 0.98];

/// Cuánto puede superar un estado al de calibración antes de que valga la
/// pena mirarlo.
///
/// No es una cifra de gusto: dentro de la banda alta **el orden no
/// reproduce**. Cinco corridas seguidas de este ejemplo pusieron el máximo
/// en `finale 0.98`, `finale 0.75`, `finale 0.98`, `finale 0.90` y
/// `finale 0.98`, con medianas que se separaban menos de un `5 %` entre sí.
/// Un aviso que se disparara con esas diferencias solo enseñaría a
/// ignorarlo.
///
/// El `15 %` sale de lo que sí se movió entre corridas: las **medianas** de
/// la banda alta, un `13 %` de la más baja a la más alta. Por debajo de eso
/// el aviso estaría midiendo la máquina y no la escena.
const TOLERANCIA_BANDA: f64 = 0.15;

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

/// Las tres regiones en el mismo progreso, el grupo `Finale` sin pintar.
fn regiones_en(t: f32) -> RevealState {
    let mut estado = RevealState::unpainted();

    for grupo in RevealGroup::ALL {
        if grupo != RevealGroup::Finale {
            estado.set_progress(grupo, t);
        }
    }

    estado
}

/// El `Finale` a medio revelar sobre las tres regiones ya pintadas.
fn finale_en(t: f32) -> RevealState {
    let mut estado = regiones_en(1.0);
    estado.set_progress(RevealGroup::Finale, t);

    estado
}

/// Un punto de medición: un estado, una cámara y su muestra de tiempos.
///
/// `tiempos[r]` es la ronda `r` para **todos** los puntos de una fase, y eso
/// es lo que hace válidos los cocientes pareados.
struct Punto {
    etiqueta: String,
    estado: RevealState,
    camara: Camera,
    tiempos: Vec<f64>,
}

impl Punto {
    fn nuevo(etiqueta: String, estado: RevealState, camara: Camera) -> Self {
        Punto {
            etiqueta,
            estado,
            camara,
            tiempos: Vec::with_capacity(RONDAS),
        }
    }

    fn resumen(&self) -> Distribution {
        summarize(&self.tiempos)
    }
}

/// Mide todos los puntos, un cuadro de cada uno por ronda, **rotando** el
/// orden de la ronda.
fn medir(puntos: &mut [Punto], diorama: &Blockout, framebuffer: &mut Framebuffer) {
    let luces = luces_del_diorama(&diorama.anchors, &diorama.scale);
    let n = puntos.len();

    for ronda in 0..RONDAS {
        for k in 0..n {
            // La rotación: en la ronda `r` empieza el punto `r % n`. Cada
            // punto ocupa cada posición de la ronda el mismo número de
            // veces, así que el coste de ir primero se reparte.
            let i = (k + ronda) % n;

            let inicio = Instant::now();
            render(
                framebuffer,
                &diorama.scene,
                &diorama.accel,
                &luces,
                &puntos[i].estado,
                &puntos[i].camara,
                Shading::Material,
            );
            puntos[i].tiempos.push(inicio.elapsed().as_secs_f64());
        }
    }
}

/// Imprime una fase y devuelve el índice de su punto más caro por mínimo.
fn reportar(puntos: &[Punto], referencia: usize, nombre_referencia: &str) -> usize {
    println!(
        "\n  {:<16} {:>9} {:>9} {:>9} {:>12}",
        "punto", "minimo", "mediana", "maximo", nombre_referencia
    );

    let mut peor = 0usize;

    for (i, punto) in puntos.iter().enumerate() {
        let d = punto.resumen();

        if d.min > puntos[peor].resumen().min {
            peor = i;
        }

        // El cociente es pareado; el `min` de la izquierda solo ordena.
        let relativo = median_ratio(&punto.tiempos, &puntos[referencia].tiempos);

        println!(
            "  {:<16} {:>9.4} {:>9.4} {:>9.4} {relativo:>11.2}x",
            punto.etiqueta, d.min, d.median, d.max
        );
    }

    peor
}

/// Mide un punto en los dos perfiles interactivos y dice cuál pasa el gate.
///
/// Existe porque el fallo del gate no es una conclusión, es una pregunta:
/// el plan dice que lo que se baja es la resolución, y quien lea el fallo
/// necesita saber **qué** resolución alcanza. Las rondas se intercalan entre
/// los dos perfiles por la misma razón que en las otras fases.
fn fase_de_perfiles(diorama: &Blockout, encuadres: &[&Punto]) {
    let luces = luces_del_diorama(&diorama.anchors, &diorama.scale);
    let perfiles = [
        ("MEDIA", InteractiveProfile::MEDIA),
        ("BAJA", InteractiveProfile::BAJA),
    ];

    // Las cuatro celdas del cruce, en una sola tanda de rondas rotadas: es
    // la única forma de que los dos perfiles y los dos encuadres se
    // comparen entre sí y no contra el estado térmico de su turno.
    let celdas: Vec<(&str, InteractiveProfile, &Punto)> = perfiles
        .iter()
        .flat_map(|(nombre, perfil)| encuadres.iter().map(move |p| (*nombre, *perfil, *p)))
        .collect();

    let mut buffers: Vec<Framebuffer> = celdas
        .iter()
        .map(|(_, p, _)| Framebuffer::new(p.width, p.height))
        .collect();
    let mut tiempos: Vec<Vec<f64>> = vec![Vec::with_capacity(RONDAS); celdas.len()];

    for ronda in 0..RONDAS {
        for k in 0..celdas.len() {
            let i = (k + ronda) % celdas.len();

            let inicio = Instant::now();
            render(
                &mut buffers[i],
                &diorama.scene,
                &diorama.accel,
                &luces,
                &celdas[i].2.estado,
                &celdas[i].2.camara,
                Shading::Material,
            );
            tiempos[i].push(inicio.elapsed().as_secs_f64());
        }
    }

    println!("\n=== Fase 3 · que perfil aguanta, y en que encuadre");
    println!(
        "  {:<8} {:>11} {:<12} {:>9} {:>11} {:>14}",
        "perfil", "resolucion", "encuadre", "mediana", "15 cuadros", "gate"
    );

    for (i, (nombre, perfil, punto)) in celdas.iter().enumerate() {
        let d = summarize(&tiempos[i]);
        let exigidos = MINIMUM_REVEAL_FRAMES * d.median as f32;
        let veredicto = match reveal_duration(d.median as f32) {
            Ok(duracion) => format!("PASA ({duracion:.2} s)"),
            Err(_) => "FALLA".to_string(),
        };

        println!(
            "  {nombre:<8} {:>5} x {:<3} {:<12} {:>9.4} {exigidos:>9.2} s {veredicto:>14}",
            perfil.width, perfil.height, punto.etiqueta, d.median
        );
    }

    println!("\n  Bajar el perfil por defecto es una decision de presentacion, no una");
    println!("  correccion de medicion: cambia lo que el usuario ve mientras orbita.");
    println!("  Queda planteada, no aplicada.");
}

fn main() {
    let diorama = nivel_texturizado();
    let perfil = InteractiveProfile::MEDIA;
    let mut framebuffer = Framebuffer::new(perfil.width, perfil.height);
    let hero = diorama.hero_camera();

    println!("interactive_frame_time · el cuadro mas caro de la demo\n");
    println!("  perfil      {} x {}", perfil.width, perfil.height);
    println!("  preset      safe-refractive-water");
    println!(
        "  escena      {} primitivas, {} texturas",
        diorama.scene.objects.len(),
        diorama.scene.textures.len()
    );
    println!("  rondas      {RONDAS}, intercaladas y con el orden rotado");
    println!("  comando     cargo run --release --example interactive_frame_time");

    // ------------------------------------------------ fase 1: estados
    let mut estados: Vec<Punto> = BARRIDO
        .iter()
        .map(|&t| Punto::nuevo(format!("regiones {t:.2}"), regiones_en(t), hero))
        .collect();
    estados.extend(
        BARRIDO_FINALE
            .iter()
            .map(|&t| Punto::nuevo(format!("finale {t:.2}"), finale_en(t), hero)),
    );
    estados.push(Punto::nuevo(
        "todo pintado".to_string(),
        RevealState::painted(),
        hero,
    ));

    println!("\n=== Fase 1 · estados de revelacion, toma hero");
    println!("  regiones t   = las tres regiones en t, grupo Finale en lienzo");
    println!("  finale t     = Finale en t sobre las tres regiones pintadas");
    println!("  todo pintado = los cuatro grupos en 1.0");

    medir(&mut estados, &diorama, &mut framebuffer);
    let peor_estado = reportar(&estados, 0, "vs lienzo");

    // Las bandas, que es lo que el barrido resuelve de verdad. Dentro de
    // cada una el progreso apenas mueve el coste; entre ellas hay un
    // escalón, y el escalón es el hallazgo.
    //
    // Todas las cifras de este bloque son **cocientes pareados contra el
    // lienzo**, el mismo estadístico de la columna de la tabla. Una versión
    // anterior mezclaba: los rangos por mínimos y la última fila por
    // cociente pareado, que es cómo se acaba comparando dos números que no
    // miden lo mismo.
    let banda = |prefijo: &str, salvo: &str| {
        let cocientes = estados
            .iter()
            .filter(|p| p.etiqueta.starts_with(prefijo) && !p.etiqueta.ends_with(salvo))
            .map(|p| median_ratio(&p.tiempos, &estados[0].tiempos));

        (
            cocientes.clone().fold(f64::INFINITY, f64::min),
            cocientes.fold(0.0_f64, f64::max),
        )
    };

    let lienzo = estados[0].resumen();
    let (reg_baja, reg_alta) = banda("regiones", "0.00");
    let (fin_baja, fin_alta) = banda("finale", "nada");

    println!("\n  bandas, en cociente pareado contra el lienzo");
    println!(
        "  lienzo                  {:.4} s   1.00x   (la referencia)",
        lienzo.median
    );
    println!("  regiones reveladas      {reg_baja:.2}x .. {reg_alta:.2}x");
    println!("  finale revelado         {fin_baja:.2}x .. {fin_alta:.2}x");
    println!(
        "  todo pintado            {:.2}x",
        median_ratio(&estados[estados.len() - 1].tiempos, &estados[0].tiempos)
    );

    // ------------------------------------------------ fase 2: cámaras
    let mut camaras: Vec<Punto> = diorama
        .measurement_cameras()
        .into_iter()
        .map(|(etiqueta, camara)| Punto::nuevo(etiqueta, RevealState::worst_case(), camara))
        .collect();

    println!("\n=== Fase 2 · worst_case() en las camaras alcanzables");

    medir(&mut camaras, &diorama, &mut framebuffer);
    let peor_camara = reportar(&camaras, 0, "vs hero");

    // ------------------------------------------------ la derivación
    //
    // El estado de calibración se mide en las dos fases —es `finale 0.50` en
    // la primera y `hero` en la segunda—, así que la diferencia entre las
    // dos es una medida del ruido del propio instrumento en una sola
    // corrida.
    let calibrado_f1 = estados
        .iter()
        .find(|p| p.estado == RevealState::worst_case())
        .expect("worst_case tiene que estar en el barrido de estados")
        .resumen();
    let calibrado_f2 = camaras[0].resumen();

    println!("\n  reproducibilidad del instrumento");
    println!(
        "  worst_case en hero: {:.4} s en la fase 1 y {:.4} s en la fase 2   ({:+.1} %)",
        calibrado_f1.median,
        calibrado_f2.median,
        100.0 * (calibrado_f2.median / calibrado_f1.median - 1.0)
    );

    let peor_de_estados = estados[peor_estado].resumen();
    let peor_de_camaras = camaras[peor_camara].resumen();

    println!(
        "\n  peor punto de la fase 1  {:<14} {:.4} s (mediana)",
        estados[peor_estado].etiqueta, peor_de_estados.median
    );
    println!(
        "  peor punto de la fase 2  {:<14} {:.4} s (mediana)",
        camaras[peor_camara].etiqueta, peor_de_camaras.median
    );

    // Dos avisos, y no uno. Una versión anterior metía el estado y la
    // cámara en la misma lista de «puntos fuera de banda» y recomendaba
    // mover `WORST_CASE_PROGRESS` para los dos. Es un error de categoría: la
    // constante fija un **estado de revelación**, y un encuadre caro no se
    // arregla moviéndola.
    println!(
        "\n  aviso 1 · el estado   calibracion: worst_case(), finale {WORST_CASE_PROGRESS:.2}"
    );

    // Se compara por medianas y no por mínimos: con quince muestras el
    // mínimo es un estadístico de orden ruidoso, y entre corridas los
    // mínimos de la banda alta se movieron un `15 %` mientras sus medianas
    // se movían menos de un `5 %`.
    let exceso_estado = peor_de_estados.median / calibrado_f1.median - 1.0;

    if exceso_estado > TOLERANCIA_BANDA {
        println!("  AVISO: un estado de revelacion se sale de la banda por arriba.");
        println!(
            "  {:<16} {:.4} s   ({:+.1} % sobre la calibracion)",
            estados[peor_estado].etiqueta,
            peor_de_estados.median,
            100.0 * exceso_estado
        );
        println!("  Revisar si aparecio un coste nuevo antes de mover la constante.");
    } else {
        println!(
            "  al dia: el estado mas caro le saca {:+.1} % y la tolerancia son {:.0} %.",
            100.0 * exceso_estado,
            100.0 * TOLERANCIA_BANDA
        );
    }

    println!("\n  aviso 2 · el encuadre");

    let exceso_camara = median_ratio(&camaras[peor_camara].tiempos, &camaras[0].tiempos) - 1.0;

    println!(
        "  {:<16} {:.4} s   ({:+.0} % sobre la hero, pareado)",
        camaras[peor_camara].etiqueta,
        peor_de_camaras.median,
        100.0 * exceso_camara
    );

    if peor_de_camaras.median > peor_de_estados.median {
        println!("  El encuadre domina sobre el estado. Esto NO se corrige con");
        println!("  WORST_CASE_PROGRESS: va al presupuesto y a la eleccion de perfil.");
    }

    // Para dimensionar se usa la **mediana** del peor punto de las dos
    // fases: el mínimo compara estados y es malo para prometer un tiempo.
    let medido = peor_de_estados.median.max(peor_de_camaras.median) as f32;

    println!(
        "\n  interactive_frame_time = {medido:.4} s   (mediana del peor punto de las dos fases)"
    );

    let mut fallo_de_gate = false;

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
            println!("  El plan manda bajar la resolucion, no alargar la animacion.");
            fallo_de_gate = true;
        }
    }

    // La fase 3 corre **siempre**, y no solo cuando el gate falla. El peor
    // encuadre alcanzable cae justo en el límite: dos corridas seguidas de
    // este ejemplo lo dejaron a un lado y al otro del crítico. Con el
    // veredicto oscilando, saber qué da el otro perfil no es información
    // opcional.
    fase_de_perfiles(&diorama, &[&camaras[0], &camaras[peor_camara]]);

    println!("\n  Registrar junto a la cifra: commit, fecha, arbol, hardware y toolchain.");
    println!("  Un tiempo solo significa algo junto a los que se midieron con el.");

    if fallo_de_gate {
        std::process::exit(1);
    }
}
