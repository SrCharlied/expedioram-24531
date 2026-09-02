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

use expedition33_continente_inacabado::accel::{SceneAccel, TraversalStats};
use expedition33_continente_inacabado::camera::{Camera, DEFAULT_VERTICAL_FOV};
use expedition33_continente_inacabado::framebuffer::Framebuffer;
use expedition33_continente_inacabado::light::{diorama as luces_del_diorama, PointLight};
use expedition33_continente_inacabado::renderer::{render, Shading};
use expedition33_continente_inacabado::reveal::RevealState;
use expedition33_continente_inacabado::scene::RevealGroup;
use expedition33_continente_inacabado::scene::{cubo_de_prueba, Scene};
use expedition33_continente_inacabado::scene_builder::{SceneScale, HERO_YAW_DEGREES};
use expedition33_continente_inacabado::scenes::continent::blockout;
use expedition33_continente_inacabado::scenes::{safe_level_con, WaterPreset};

const USO: &str = "\
Render sin ventana del Continente Inacabado.

  --preset <nombre>   escena a renderizar (por defecto: blockout)
  --width <n>         ancho en pixeles (por defecto: 800)
  --height <n>        alto en pixeles (por defecto: 600)
  --yaw <grados>      angulo de orbita; por defecto el de la toma hero
  --elevation <grados> elevacion del ojo; por defecto 35 (la de la orbita)
  --shading <modo>    material | albedo | normals (por defecto: material)
  --benchmark <n>     repite el render n veces y reporta min/mediana/max
  --no-textures       color plano, sin cargar los assets de textura
  --reveal <0..1>     progreso de pintura de las cuatro regiones (por defecto 1)
  --output <ruta>     PNG de salida (por defecto: evidence/renders/hero.png)
  --help              esta ayuda

Presets disponibles:
  safe-refractive-water   nivel seguro con el volumen de agua real (160
                          primitivas): 0.9/0.9, ior 1.333. Es el preset
                          canonico desde la Tarea 5.4 y el que se presenta.
  safe-interior-visible   nivel seguro sin el volumen de agua (159).
                          Mide el interior de la bahia sin el coste de la
                          refraccion; es la referencia del Hito 3.
  safe-opaque-water       el mismo volumen con los techos opticos en cero
                          (160). Control de oclusion, NO rendimiento:
                          oculta 44 primitivas del interior.
  blockout                composicion global del Blockout 1, en grises
  cubo                    un cuboide centrado, para geometria y camara";

struct Opciones {
    preset: String,
    width: usize,
    height: usize,
    /// `None` significa "el yaw propio del preset", que para el blockout es
    /// la toma hero.
    yaw: Option<f32>,
    /// `None` usa la elevacion orbital estandar.
    elevation: Option<f32>,
    /// Repeticiones cronometradas. Una sola pasada no es una medicion.
    benchmark: usize,
    /// Con texturas por defecto; se pueden desactivar para comparar.
    texturas: bool,
    /// Progreso de revelacion aplicado a los cuatro grupos.
    reveal: f32,
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
            benchmark: 1,
            texturas: true,
            reveal: 1.0,
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

        // Bandera sin valor: se consume sola.
        if bandera == "--no-textures" {
            opciones.texturas = false;
            i += 1;
            continue;
        }

        let valor = args
            .get(i + 1)
            .ok_or_else(|| format!("{bandera} necesita un valor"))?;

        match bandera {
            "--preset" => opciones.preset = valor.clone(),
            "--width" => opciones.width = numero(bandera, valor)?,
            "--height" => opciones.height = numero(bandera, valor)?,
            "--output" => opciones.output = PathBuf::from(valor),
            "--reveal" => {
                opciones.reveal = valor
                    .parse()
                    .map_err(|_| format!("--reveal espera 0..1, no {valor:?}"))?;
            }
            "--benchmark" => {
                opciones.benchmark = numero(bandera, valor)?.max(1);
            }
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
type Preset = (
    Scene,
    SceneAccel,
    Vec<PointLight>,
    Camera,
    Option<SceneScale>,
);

fn preset(
    nombre: &str,
    yaw: Option<f32>,
    elevation: Option<f32>,
    texturas: bool,
) -> Result<Preset, String> {
    match nombre {
        "safe-refractive-water" | "safe-interior-visible" | "safe-opaque-water" => {
            let water = match nombre {
                "safe-refractive-water" => WaterPreset::RefractiveWater,
                "safe-opaque-water" => WaterPreset::OpaqueWater,
                _ => WaterPreset::InteriorVisible,
            };

            // Con texturas, la raiz del proyecto es el directorio actual.
            let raiz = std::path::PathBuf::from(".");
            let nivel =
                safe_level_con(water, if texturas { Some(&raiz) } else { None }).map_err(|e| {
                    format!(
                        "{e}
  genera los assets con: cargo run --release --bin generate_assets"
                    )
                })?;
            let grados_yaw = yaw.unwrap_or(HERO_YAW_DEGREES);
            let camera = match elevation {
                Some(elev) => nivel.camera_at(grados_yaw, elev),
                None => nivel.camera_at_yaw(grados_yaw),
            };
            let escala = nivel.scale;
            let lights = luces_del_diorama(&nivel.anchors, &nivel.scale);

            Ok((nivel.scene, nivel.accel, lights, camera, Some(escala)))
        }
        "blockout" => {
            let blockout = blockout();
            let grados_yaw = yaw.unwrap_or(HERO_YAW_DEGREES);
            let camera = match elevation {
                Some(elev) => blockout.camera_at(grados_yaw, elev),
                None => blockout.camera_at_yaw(grados_yaw),
            };
            let escala = blockout.scale;
            let lights = luces_del_diorama(&blockout.anchors, &blockout.scale);

            Ok((blockout.scene, blockout.accel, lights, camera, Some(escala)))
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
            let scene = cubo_de_prueba();
            let accel = SceneAccel::build(&scene).expect("el cubo existe");

            Ok((scene, accel, Vec::new(), camera, None))
        }
        otro => Err(format!("preset desconocido: {otro}")),
    }
}

fn ejecutar(opciones: Opciones) -> Result<(), String> {
    let (scene, accel, lights, camera, escala) = preset(
        &opciones.preset,
        opciones.yaw,
        opciones.elevation,
        opciones.texturas,
    )?;

    let mut framebuffer = Framebuffer::new(opciones.width, opciones.height);

    // Un solo progreso para los cuatro grupos: basta para inspeccionar la
    // interpolacion, y la revelacion por region llega con el picking.
    let mut reveal = RevealState::unpainted();
    for grupo in [
        RevealGroup::Meadows,
        RevealGroup::Breakwater,
        RevealGroup::FlyingWaters,
        RevealGroup::Finale,
    ] {
        reveal.set_progress(grupo, opciones.reveal);
    }

    // Repetir y quedarse con la distribucion: una sola pasada mide tanto
    // el estado de la cache como el renderer.
    let mut tiempos = Vec::with_capacity(opciones.benchmark);
    let mut stats = TraversalStats::default();

    for _ in 0..opciones.benchmark {
        let inicio = Instant::now();
        stats = render(
            &mut framebuffer,
            &scene,
            &accel,
            &lights,
            &reveal,
            &camera,
            opciones.shading,
        );
        tiempos.push(inicio.elapsed().as_secs_f64());
    }

    tiempos.sort_by(|a, b| a.partial_cmp(b).expect("los tiempos no son NaN"));

    framebuffer
        .save_png(&opciones.output)
        .map_err(|e| format!("no se pudo escribir {}: {e}", opciones.output.display()))?;

    println!("preset    {}", opciones.preset);
    println!("tamano    {} x {}", opciones.width, opciones.height);
    println!("shading   {:?}", opciones.shading);
    println!("objetos   {}", scene.objects.len());
    println!("luces     {}", lights.len());
    println!("texturas  {}", scene.textures.len());
    println!("reveal    {:.2}", opciones.reveal);
    println!(
        "grupos    {} ({} clusters)",
        accel.groups.len(),
        accel.groups.iter().map(|g| g.clusters.len()).sum::<usize>()
    );
    println!(
        "rayos     {} primarios, {} de sombra",
        stats.primary_rays, stats.shadow_rays
    );
    println!(
        "          {} reflejados, {} refractados (profundidad {})",
        stats.reflection_rays,
        stats.refraction_rays,
        expedition33_continente_inacabado::renderer::MAX_DEPTH
    );
    println!(
        "pruebas   {} de primitiva, {} de bounds",
        stats.primitive_tests,
        stats.group_bounds_tests + stats.cluster_bounds_tests
    );
    println!(
        "por rayo  {:.2} pruebas de primitiva",
        stats.primitive_tests as f64 / (stats.primary_rays + stats.shadow_rays) as f64
    );

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
    if opciones.benchmark > 1 {
        println!(
            "tiempo    min {:.4} s | mediana {:.4} s | max {:.4} s  ({} repeticiones)",
            tiempos[0],
            tiempos[tiempos.len() / 2],
            tiempos[tiempos.len() - 1],
            opciones.benchmark
        );
    } else {
        // Una sola pasada no es una medicion: sin repeticiones no se
        // distingue el renderer del estado de la cache.
        println!(
            "tiempo    {:.4} s (informativo, sin repeticiones)",
            tiempos[0]
        );
    }
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
