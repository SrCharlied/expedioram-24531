//! Render sin ventana, a PNG.
//!
//! Existe por tres razones que el binario con ventana no cubre: producir la
//! evidencia que exige el plan, permitir comparar dos renders sin depender
//! de una captura de pantalla, y poder medir tiempos en el Hito 3 sin que
//! el coste de presentar el framebuffer contamine la medición.
//!
//! ```text
//! cargo run --release --bin render_scene -- \
//!   --preset cubo --width 800 --height 600 \
//!   --output evidence/renders/hero.png
//! ```

use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use nalgebra_glm::Vec3;

use expedition33_continente_inacabado::camera::{Camera, DEFAULT_VERTICAL_FOV};
use expedition33_continente_inacabado::framebuffer::Framebuffer;
use expedition33_continente_inacabado::renderer::{render, Shading};
use expedition33_continente_inacabado::scene::{cubo_de_prueba, Scene};

const USO: &str = "\
Render sin ventana del Continente Inacabado.

  --preset <nombre>   escena a renderizar (por defecto: cubo)
  --width <n>         ancho en pixeles (por defecto: 800)
  --height <n>        alto en pixeles (por defecto: 600)
  --shading <modo>    material | normals (por defecto: normals)
  --output <ruta>     PNG de salida (por defecto: evidence/renders/hero.png)
  --help              esta ayuda

Presets disponibles:
  cubo       un cuboide centrado, para verificar geometria y camara

Presets que todavia no existen:
  blockout   llega con la Tarea 2.4, cuando existan las anclas de escena";

struct Opciones {
    preset: String,
    width: usize,
    height: usize,
    shading: Shading,
    output: PathBuf,
}

impl Default for Opciones {
    fn default() -> Self {
        Opciones {
            preset: "cubo".to_string(),
            width: 800,
            height: 600,
            shading: Shading::Normals,
            output: PathBuf::from("evidence/renders/hero.png"),
        }
    }
}

/// Parseo a mano sobre `std::env::args`. Son cinco banderas; una
/// dependencia de parseo costaria mas de lo que ahorra.
fn parsear(args: &[String]) -> Result<Option<Opciones>, String> {
    let mut opciones = Opciones::default();
    let mut i = 0;

    while i < args.len() {
        let bandera = args[i].as_str();

        if bandera == "--help" || bandera == "-h" {
            return Ok(None);
        }

        let valor = args
            .get(i + 1)
            .ok_or_else(|| format!("{bandera} necesita un valor"))?;

        match bandera {
            "--preset" => opciones.preset = valor.clone(),
            "--width" => opciones.width = numero(bandera, valor)?,
            "--height" => opciones.height = numero(bandera, valor)?,
            "--output" => opciones.output = PathBuf::from(valor),
            "--shading" => {
                opciones.shading = match valor.as_str() {
                    "material" => Shading::Material,
                    "normals" => Shading::Normals,
                    otro => return Err(format!("shading desconocido: {otro}")),
                }
            }
            otro => return Err(format!("bandera desconocida: {otro}")),
        }

        i += 2;
    }

    if opciones.width == 0 || opciones.height == 0 {
        return Err("el ancho y el alto deben ser mayores que cero".to_string());
    }

    Ok(Some(opciones))
}

fn numero(bandera: &str, valor: &str) -> Result<usize, String> {
    valor
        .parse()
        .map_err(|_| format!("{bandera} espera un entero, no {valor:?}"))
}

/// Devuelve la escena y la camara de un preset.
///
/// Por ahora solo existe el cuboide de verificacion. Cuando la Tarea 2.4
/// construya las anclas y el blockout, esta funcion delega en el
/// `scene_builder` en vez de armar la escena aqui.
fn preset(nombre: &str) -> Result<(Scene, Camera), String> {
    match nombre {
        "cubo" => {
            let camera = Camera::new(
                Vec3::new(0.0, 0.0, 5.0),
                Vec3::zeros(),
                Vec3::zeros(),
                Vec3::new(0.0, 1.0, 0.0),
                DEFAULT_VERTICAL_FOV,
            );

            Ok((cubo_de_prueba(), camera))
        }
        "blockout" => {
            Err("el preset 'blockout' todavia no existe; llega con la Tarea 2.4".to_string())
        }
        otro => Err(format!("preset desconocido: {otro}")),
    }
}

fn ejecutar(opciones: Opciones) -> Result<(), String> {
    let (scene, camera) = preset(&opciones.preset)?;

    let mut framebuffer = Framebuffer::new(opciones.width, opciones.height);

    let inicio = Instant::now();
    render(&mut framebuffer, &scene, &camera, opciones.shading);
    let transcurrido = inicio.elapsed();

    framebuffer
        .save_png(&opciones.output)
        .map_err(|e| format!("no se pudo escribir {}: {e}", opciones.output.display()))?;

    println!("preset    {}", opciones.preset);
    println!("tamano    {} x {}", opciones.width, opciones.height);
    println!("shading   {:?}", opciones.shading);
    println!("objetos   {}", scene.objects.len());
    // Informativo, no una medicion: un solo render, sin repeticiones y sin
    // registrar el hardware. Las mediciones formales llegan en el Hito 3.
    println!(
        "tiempo    {:.3} s (informativo)",
        transcurrido.as_secs_f64()
    );
    println!("salida    {}", opciones.output.display());

    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match parsear(&args) {
        Ok(None) => {
            println!("{USO}");
            ExitCode::SUCCESS
        }
        Ok(Some(opciones)) => match ejecutar(opciones) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("error: {e}");
                ExitCode::FAILURE
            }
        },
        Err(e) => {
            eprintln!("error: {e}\n\n{USO}");
            ExitCode::FAILURE
        }
    }
}
