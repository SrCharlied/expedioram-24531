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
  --reveal <0..1>     progreso de pintura de los cuatro grupos a la vez
  --paint <grupo>     pinta solo los grupos nombrados; repetible.
                      Grupos: meadows, breakwater, waters, finale.
                      Lo nombrado queda en 1 y lo demas en lienzo.
                      No se combina con --reveal.
                      finale exige nombrar tambien las tres regiones,
                      porque el Monolito no se elige: se revela al
                      completarse el Continente.
  --output <ruta>     PNG de salida (por defecto: evidence/renders/hero.png)
  --help              esta ayuda

Sin --reveal ni --paint se pinta todo (equivale a --reveal 1).

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

/// Que estado de revelacion pidio la linea de ordenes.
///
/// Son dos modos y no un escalar con excepciones. El global existe desde el
/// Hito 3 y sirve para mirar la interpolacion; el de regiones lo pide la
/// Tarea 8.3, que necesita un PNG por region pintada y no cuatro copias del
/// mismo degradado.
#[derive(Debug, Clone, PartialEq)]
enum Revelacion {
    /// El mismo progreso en los cuatro grupos.
    Global(f32),
    /// Los grupos nombrados a `1.0`; el resto se queda en lienzo.
    PorRegion(Vec<RevealGroup>),
}

impl Revelacion {
    /// Nombre de un grupo en la linea de ordenes.
    ///
    /// `waters` y no `flying_waters` porque es lo que se teclea; el mapa
    /// vive aqui para que la ayuda, el parseo y la salida no puedan
    /// discrepar.
    fn nombre(grupo: RevealGroup) -> &'static str {
        match grupo {
            RevealGroup::Meadows => "meadows",
            RevealGroup::Breakwater => "breakwater",
            RevealGroup::FlyingWaters => "waters",
            RevealGroup::Finale => "finale",
        }
    }

    /// El grupo que nombra `valor`, o un error que lista los validos.
    fn grupo(valor: &str) -> Result<RevealGroup, String> {
        RevealGroup::ALL
            .into_iter()
            .find(|g| Self::nombre(*g) == valor)
            .ok_or_else(|| {
                let validos: Vec<&str> = RevealGroup::ALL.into_iter().map(Self::nombre).collect();

                format!(
                    "--paint no conoce {valor:?}; los grupos son {}",
                    validos.join(", ")
                )
            })
    }

    /// El estado que hay que trazar.
    fn estado(&self) -> RevealState {
        let mut reveal = RevealState::unpainted();

        match self {
            Revelacion::Global(progreso) => {
                for grupo in RevealGroup::ALL {
                    reveal.set_progress(grupo, *progreso);
                }
            }
            Revelacion::PorRegion(grupos) => {
                for grupo in grupos {
                    reveal.set_progress(*grupo, 1.0);
                }
            }
        }

        reveal
    }
}

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
    /// Que se pinta y cuanto. Ver `Revelacion`.
    revelacion: Revelacion,
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
            // Sin banderas se pinta todo: es el defecto historico y el que
            // usan los renders de evidencia de los hitos anteriores.
            revelacion: Revelacion::Global(1.0),
            shading: Shading::Material,
            output: PathBuf::from("evidence/renders/hero.png"),
        }
    }
}

/// Parseo a mano sobre `std::env::args`. Son cinco banderas; una
/// dependencia de parseo costaria mas de lo que ahorra.
fn parsear(args: &[String]) -> Result<Option<Opciones>, String> {
    let mut opciones = Opciones::default();

    // `None` mientras nadie haya pedido nada, para poder distinguir «no
    // paso ninguna bandera» de «paso --reveal 1». Sin esa distincion, el
    // conflicto entre --reveal y --paint no se puede detectar.
    let mut revelacion: Option<Revelacion> = None;
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
                if matches!(revelacion, Some(Revelacion::PorRegion(_))) {
                    return Err(
                        "--reveal y --paint no se combinan: uno pinta los cuatro grupos \
                         a la vez y el otro elige cuales"
                            .to_string(),
                    );
                }

                let progreso = valor
                    .parse()
                    .map_err(|_| format!("--reveal espera 0..1, no {valor:?}"))?;

                revelacion = Some(Revelacion::Global(progreso));
            }
            "--paint" => {
                let grupo = Revelacion::grupo(valor)?;

                match revelacion {
                    Some(Revelacion::Global(_)) => {
                        return Err("--paint y --reveal no se combinan: uno elige que grupos \
                             se pintan y el otro los pinta todos"
                            .to_string())
                    }
                    Some(Revelacion::PorRegion(ref mut grupos)) => {
                        if grupos.contains(&grupo) {
                            return Err(format!("--paint {valor} esta repetido"));
                        }

                        grupos.push(grupo);
                    }
                    None => revelacion = Some(Revelacion::PorRegion(vec![grupo])),
                }
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

    if let Some(pedida) = revelacion {
        // El Monolito no se elige: `RevealState::activate` lo prohibe hasta
        // que las tres regiones estan pintadas, y un PNG que lo mostrara
        // solo ensenaria un estado que la obra no alcanza.
        if let Revelacion::PorRegion(ref grupos) = pedida {
            if grupos.contains(&RevealGroup::Finale) {
                let faltan: Vec<&str> = RevealGroup::ALL
                    .into_iter()
                    .filter(|g| *g != RevealGroup::Finale && !grupos.contains(g))
                    .map(Revelacion::nombre)
                    .collect();

                if !faltan.is_empty() {
                    return Err(format!(
                        "--paint finale exige pintar antes {}: el Monolito se revela al \
                         completarse el Continente, no se elige",
                        faltan.join(", ")
                    ));
                }
            }
        }

        opciones.revelacion = pedida;
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

    let reveal = opciones.revelacion.estado();

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
    // El estado grupo a grupo, y no un unico numero.
    //
    // Con `--paint` un «reveal 1.00» seria mentira: tres de los cuatro
    // grupos estan en lienzo. Se imprimen los cuatro siempre, tambien en
    // modo global, para que el pie de un PNG diga que se estaba viendo.
    let estado: Vec<String> = RevealGroup::ALL
        .into_iter()
        .map(|g| format!("{} {:.2}", Revelacion::nombre(g), reveal.progress(g)))
        .collect();

    println!("reveal    {}", estado.join("  "));
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Los argumentos tal como llegan de `std::env::args`, ya sin el `argv[0]`.
    fn args(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    /// El estado resuelto, o el mensaje de error del parseo.
    fn estado(v: &[&str]) -> Result<RevealState, String> {
        let opciones = parsear(&args(v))?.expect("no es --help");

        Ok(opciones.revelacion.estado())
    }

    /// Progreso de los cuatro grupos, en el orden de `RevealGroup::ALL`.
    fn progresos(reveal: &RevealState) -> [f32; 4] {
        [
            reveal.progress(RevealGroup::Meadows),
            reveal.progress(RevealGroup::Breakwater),
            reveal.progress(RevealGroup::FlyingWaters),
            reveal.progress(RevealGroup::Finale),
        ]
    }

    #[test]
    fn sin_banderas_se_pinta_todo() {
        // El defecto historico, y el que usan los renders de evidencia de
        // los hitos anteriores. Anadir `--paint` no puede cambiarlo.
        let reveal = estado(&[]).expect("sin banderas parsea");

        assert_eq!(progresos(&reveal), [1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn paint_de_una_region_deja_el_resto_en_lienzo() {
        let reveal = estado(&["--paint", "meadows"]).expect("parsea");

        assert_eq!(progresos(&reveal), [1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn paint_acumula_las_tres_regiones() {
        // Repetible, y el orden no importa: es el estado justo antes de que
        // el Monolito arranque solo.
        let reveal = estado(&[
            "--paint",
            "waters",
            "--paint",
            "meadows",
            "--paint",
            "breakwater",
        ])
        .expect("parsea");

        assert_eq!(progresos(&reveal), [1.0, 1.0, 1.0, 0.0]);
    }

    #[test]
    fn paint_con_las_cuatro_pinta_el_diorama_entero() {
        let reveal = estado(&[
            "--paint",
            "meadows",
            "--paint",
            "breakwater",
            "--paint",
            "waters",
            "--paint",
            "finale",
        ])
        .expect("parsea");

        assert_eq!(progresos(&reveal), [1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn paint_y_reveal_no_se_combinan_en_ningun_orden() {
        // Los dos modos se contradicen: uno elige grupos y el otro los pinta
        // todos. Sin el error, el ultimo en la linea ganaria en silencio.
        let primero = estado(&["--reveal", "0.5", "--paint", "meadows"]).unwrap_err();
        let segundo = estado(&["--paint", "meadows", "--reveal", "0.5"]).unwrap_err();

        assert!(primero.contains("no se combinan"), "{primero}");
        assert!(segundo.contains("no se combinan"), "{segundo}");
    }

    #[test]
    fn el_finale_sin_las_tres_regiones_es_un_error() {
        // `RevealState::activate` prohibe el Finale prematuro, asi que un
        // PNG con el Monolito revelado sobre lienzo mostraria un estado que
        // la obra no alcanza.
        let solo = estado(&["--paint", "finale"]).unwrap_err();

        assert!(solo.contains("meadows"), "{solo}");
        assert!(solo.contains("breakwater"), "{solo}");
        assert!(solo.contains("waters"), "{solo}");

        // Y tambien con dos de las tres: falta una y hay que decir cual.
        let casi = estado(&[
            "--paint",
            "meadows",
            "--paint",
            "breakwater",
            "--paint",
            "finale",
        ])
        .unwrap_err();

        assert!(casi.contains("waters"), "{casi}");
        assert!(
            !casi.contains("meadows"),
            "no deberia pedir lo ya pintado: {casi}"
        );
    }

    #[test]
    fn un_grupo_repetido_es_un_error() {
        // Repetir no es acumular: o el usuario se equivoco de grupo o
        // escribio dos veces el mismo, y las dos merecen aviso.
        let error = estado(&["--paint", "meadows", "--paint", "meadows"]).unwrap_err();

        assert!(error.contains("repetido"), "{error}");
        assert!(error.contains("meadows"), "{error}");
    }

    #[test]
    fn un_grupo_desconocido_lista_los_validos() {
        // `flying_waters` es el nombre del enum y **no** el de la bandera:
        // es el error mas probable, asi que el mensaje tiene que dar la
        // lista en vez de limitarse a rechazar.
        let error = estado(&["--paint", "flying_waters"]).unwrap_err();

        assert!(error.contains("flying_waters"), "{error}");
        for nombre in ["meadows", "breakwater", "waters", "finale"] {
            assert!(error.contains(nombre), "falta {nombre} en: {error}");
        }
    }

    #[test]
    fn reveal_global_sigue_llegando_a_los_cuatro_grupos() {
        let reveal = estado(&["--reveal", "0.25"]).expect("parsea");

        assert_eq!(progresos(&reveal), [0.25, 0.25, 0.25, 0.25]);
    }
}
