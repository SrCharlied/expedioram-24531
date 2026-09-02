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
use expedition33_continente_inacabado::light::{diorama as luces_del_diorama, PointLight};
use expedition33_continente_inacabado::renderer::{render, Shading};
use expedition33_continente_inacabado::scene::{cubo_de_prueba, Scene};
use expedition33_continente_inacabado::scene_builder::{SceneScale, HERO_YAW_DEGREES};
use expedition33_continente_inacabado::scenes::continent::blockout;

const USO: &str = "\
Render sin ventana del Continente Inacabado.

  --preset <nombre>   escena a renderizar (por defecto: blockout)
  --width <n>         ancho en pixeles (por defecto: 800)
  --height <n>        alto en pixeles (por defecto: 600)
  --yaw <grados>      angulo de orbita; por defecto el de la toma hero
  --elevation <grados> elevacion del ojo; por defecto 35 (la de la orbita)
  --shading <modo>    material | albedo | normals (por defecto: material)
  --output <ruta>     PNG de salida (por defecto: evidence/renders/hero.png)
  --help              esta ayuda

Presets disponibles:
  blockout   composicion global del Blockout 1, en cuboides grises
  cubo       un cuboide centrado, para verificar geometria y camara";

struct Opciones {
    preset: String,
    width: usize,
    height: usize,
    /// `None` significa "el yaw propio del preset", que para el blockout es
    /// la toma hero.
    yaw: Option<f32>,
    /// `None` usa la elevacion orbital estandar.
    elevation: Option<f32>,
    shading: Shading,
    output: PathBuf,
}

impl Default for Opciones {
    fn default() -> Self {
        Opciones {
            preset: "blockout".to_string(),
            width: 800,
            height: 600,
            yaw: None,
            elevation: None,
            shading: Shading::Material,
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
            "--yaw" => {
                opciones.yaw = Some(
                    valor
                        .parse()
                        .map_err(|_| format!("--yaw espera grados, no {valor:?}"))?,
                )
            }
            "--elevation" => {
                opciones.elevation = Some(
                    valor
                        .parse()
                        .map_err(|_| format!("--elevation espera grados, no {valor:?}"))?,
                )
            }
            "--shading" => {
                opciones.shading = match valor.as_str() {
                    "material" => Shading::Material,
                    "albedo" => Shading::Albedo,
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

/// Devuelve la escena, la camara y --si el preset la tiene-- su escala
/// medida.
///
/// El yaw explicito manda sobre el propio del preset: es lo que permite
/// producir los cuatro angulos que valida la Tarea 2.5 sin recompilar.
type Preset = (Scene, Vec<PointLight>, Camera, Option<SceneScale>);

fn preset(nombre: &str, yaw: Option<f32>, elevation: Option<f32>) -> Result<Preset, String> {
    match nombre {
        "blockout" => {
            let blockout = blockout();
            let grados_yaw = yaw.unwrap_or(HERO_YAW_DEGREES);
            let camera = match elevation {
                Some(elev) => blockout.camera_at(grados_yaw, elev),
                None => blockout.camera_at_yaw(grados_yaw),
            };
            let escala = blockout.scale;
            let lights = luces_del_diorama(&blockout.anchors, &blockout.scale);

            Ok((blockout.scene, lights, camera, Some(escala)))
        }
        "cubo" => {
            let eye = match yaw {
                Some(grados) => {
                    let theta = grados.to_radians();
                    Vec3::new(5.0 * theta.cos(), 0.0, 5.0 * theta.sin())
                }
                None => Vec3::new(0.0, 0.0, 5.0),
            };

            let camera = Camera::new(
                eye,
                Vec3::zeros(),
                Vec3::zeros(),
                Vec3::new(0.0, 1.0, 0.0),
                DEFAULT_VERTICAL_FOV,
            );

            // El cubo de prueba no tiene luces propias: se ve por albedo
            // o por normales.
            Ok((cubo_de_prueba(), Vec::new(), camera, None))
        }
        otro => Err(format!("preset desconocido: {otro}")),
    }
}

fn ejecutar(opciones: Opciones) -> Result<(), String> {
    let (scene, lights, camera, escala) =
        preset(&opciones.preset, opciones.yaw, opciones.elevation)?;

    let mut framebuffer = Framebuffer::new(opciones.width, opciones.height);

    let inicio = Instant::now();
    render(&mut framebuffer, &scene, &lights, &camera, opciones.shading);
    let transcurrido = inicio.elapsed();

    framebuffer
        .save_png(&opciones.output)
        .map_err(|e| format!("no se pudo escribir {}: {e}", opciones.output.display()))?;

    println!("preset    {}", opciones.preset);
    println!("tamano    {} x {}", opciones.width, opciones.height);
    println!("shading   {:?}", opciones.shading);
    println!("objetos   {}", scene.objects.len());
    println!("luces     {}", lights.len());

    // Los parametros de escala son medidos, no elegidos. Imprimirlos aqui
    // es lo que permite copiarlos a docs/evidence.md sin transcribir a mano.
    if let Some(escala) = escala {
        println!("scene_radius     {:.4}", escala.scene_radius);
        println!("monolith_height  {:.4}", escala.monolith_height);
        println!("water_surface_y  {:.4}", escala.water_surface_y);
        println!(
            "orbit_radius     {:.4}  ({:.3} x scene_radius, derivado)",
            escala.orbit_radius,
            escala.orbit_radius / escala.scene_radius
        );
        println!(
            "view_pitch       {:.2} grados",
            camera.view_pitch().to_degrees()
        );
    }

    println!(
        "yaw       {}",
        match opciones.yaw {
            Some(grados) => format!("{grados} grados"),
            None => "el del preset".to_string(),
        }
    );
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
