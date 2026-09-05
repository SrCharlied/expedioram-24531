//! Rinde el nivel seguro contra el candidato de la Tarea 7.2, para poder
//! decidir con los ojos si el lote compra lectura o solo cuesta tiempo.
//!
//! ```text
//! cargo run --release --example density_preview
//! ```
//!
//! # La pregunta que responden estos renders
//!
//! No es «¿se nota?». Quince primitivas siempre se notan si uno las busca.
//! Es **si el diorama se lee mejor**: si el lecho deja de parecer una caja,
//! si el kelp da escala al barco, si las rocas rompen el plano del fondo.
//! Un lote que solo añade puntos verdes en el mismo sitio no aporta lectura,
//! y la autorización dice explícitamente que en ese caso se retira.
//!
//! # Los tres encuadres
//!
//! La toma hero, que es lo que se presenta, y las dos cámaras de calibración
//! —radio mínimo, elevaciones alta y cenital—, que son las que llenan la
//! pantalla de bahía. El lote entero cae dentro del agua, así que es en esas
//! dos donde se ve de verdad; la hero dice si el detalle llega a leerse a la
//! distancia de presentación, que es la pregunta más difícil de las dos.
//!
//! Todo a `800 × 600` y en el estado **pintado**: aquí se juzga la
//! composición, no el coste, y el estado intermedio del `Finale` dejaría
//! medio Continente en lienzo.

use std::path::{Path, PathBuf};

use expedition33_continente_inacabado::camera::Camera;
use expedition33_continente_inacabado::framebuffer::Framebuffer;
use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
use expedition33_continente_inacabado::renderer::{render, Shading};
use expedition33_continente_inacabado::reveal::RevealState;
use expedition33_continente_inacabado::scene_builder::Blockout;
use expedition33_continente_inacabado::scenes::{
    safe_level_con, target_level_con, Density, WaterPreset,
};

const ANCHO: usize = 800;
const ALTO: usize = 600;

const SALIDA: &str = "evidence/hito7/densidad";

fn nivel(densidad: Density) -> Blockout {
    let raiz = PathBuf::from(".");

    let construido = match densidad {
        Density::Safe => safe_level_con(WaterPreset::RefractiveWater, Some(&raiz)),
        Density::Target => target_level_con(WaterPreset::RefractiveWater, Some(&raiz)),
    };

    match construido {
        Ok(nivel) => nivel,
        Err(e) => {
            eprintln!("error: {e}");
            eprintln!("  esta comparacion es de composicion; sin texturas no dice nada.");
            eprintln!("  generalos con: cargo run --release --bin generate_assets");
            std::process::exit(1);
        }
    }
}

/// Guarda, o aborta con la ruta.
fn guardar(framebuffer: &Framebuffer, ruta: &str) {
    if let Err(e) = framebuffer.save_png(Path::new(ruta)) {
        eprintln!("error: no se pudo guardar {ruta}: {e}");
        std::process::exit(1);
    }

    println!("  {ruta}");
}

/// Cuántos píxeles cambian entre dos cuadros, y cuántos cambian **de
/// verdad**.
///
/// El primero cuenta cualquier diferencia, incluida la de un byte que nadie
/// distingue. El segundo exige un salto perceptible en algún canal, que es
/// lo que decide si el lote se lee o solo existe.
///
/// El umbral es `8` de `255`: por debajo de eso, dos grises contiguos son el
/// mismo gris en una pantalla y en un proyector de aula todavía más.
fn diferencia(a: &Framebuffer, b: &Framebuffer) -> (f64, f64) {
    const PERCEPTIBLE: i32 = 8;

    let total = a.buffer.len() as f64;
    let mut distintos = 0usize;
    let mut perceptibles = 0usize;

    for (x, y) in a.buffer.iter().zip(&b.buffer) {
        if x == y {
            continue;
        }

        distintos += 1;

        let canal = |c: u32, desplazamiento: u32| ((c >> desplazamiento) & 0xFF) as i32;
        let salto = [16, 8, 0]
            .iter()
            .map(|d| (canal(*x, *d) - canal(*y, *d)).abs())
            .max()
            .expect("tres canales");

        if salto >= PERCEPTIBLE {
            perceptibles += 1;
        }
    }

    (
        100.0 * distintos as f64 / total,
        100.0 * perceptibles as f64 / total,
    )
}

fn main() {
    let niveles = [
        ("safe", nivel(Density::Safe)),
        ("target", nivel(Density::Target)),
    ];

    if let Err(e) = std::fs::create_dir_all(SALIDA) {
        eprintln!("error: no se pudo crear {SALIDA}: {e}");
        std::process::exit(1);
    }

    println!(
        "density_preview · el lote de la Tarea 7.2
"
    );

    for (nombre, diorama) in &niveles {
        println!("  {nombre}: {} primitivas", diorama.scene.objects.len());
    }
    println!();

    // Los tres encuadres salen del nivel seguro y se usan **los mismos** para
    // los dos. Pedírselos a cada blockout por separado sería arriesgarse a
    // comparar dos imágenes tomadas desde sitios distintos si el lote llegara
    // a mover la escala medida.
    let hero = niveles[0].1.hero_camera();
    let calibracion = niveles[0].1.calibration_cameras();

    let encuadres: Vec<(&str, &Camera)> = std::iter::once(("hero", &hero))
        .chain(calibracion.iter().map(|(e, c)| (*e, c)))
        .collect();

    let mut resumen = Vec::with_capacity(encuadres.len());

    for (encuadre, camara) in encuadres {
        let mut cuadros = Vec::with_capacity(niveles.len());

        for (nombre, diorama) in &niveles {
            let luces = luces_del_diorama(&diorama.anchors, &diorama.scale);
            let mut framebuffer = Framebuffer::new(ANCHO, ALTO);

            render(
                &mut framebuffer,
                &diorama.scene,
                &diorama.accel,
                &luces,
                &RevealState::painted(),
                camara,
                Shading::Material,
            );

            let ranura = encuadre.replace([' ', '+'], "");
            guardar(&framebuffer, &format!("{SALIDA}/{ranura}-{nombre}.png"));
            cuadros.push(framebuffer);
        }

        let (distintos, perceptibles) = diferencia(&cuadros[0], &cuadros[1]);
        resumen.push((encuadre.to_string(), distintos, perceptibles));
    }

    println!(
        "
  cuanto cambia el cuadro con el lote"
    );
    println!(
        "  {:<14} {:>12} {:>14}",
        "encuadre", "px distintos", "px perceptibles"
    );

    for (encuadre, distintos, perceptibles) in &resumen {
        println!("  {encuadre:<14} {distintos:>11.2} % {perceptibles:>13.2} %");
    }

    println!(
        "
  «Perceptible» es un salto de al menos 8/255 en algun canal."
    );
    println!("  La cifra no decide: dice cuanto hay que mirar. Comparar por pares");
    println!("  el mismo encuadre, -safe contra -target, y decidir si el lote");
    println!("  compra lectura. Si no la compra, se retira; lo dice la autorizacion.");
}
