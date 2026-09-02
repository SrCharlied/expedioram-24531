//! Render controlado de sombras submarinas — Tarea 5.6.
//!
//! Produce tres renders de la misma escena cambiando **solo la plataforma
//! de luces**, y mide los cuatro criterios del plan sobre puntos concretos.
//!
//! ```text
//! cargo run --release --example submarine_shadows
//! ```
//!
//! Los criterios se verifican además como tests, en
//! `tests/submarine_shadows.rs`. Este ejemplo existe para producir la
//! imagen y los números que van a la evidencia; el test es lo que protege
//! de una regresión.

use std::path::PathBuf;

use expedition33_continente_inacabado::accel::TraversalStats;
use expedition33_continente_inacabado::color::Color;
use expedition33_continente_inacabado::framebuffer::Framebuffer;
use expedition33_continente_inacabado::light::{diorama as luces_del_diorama, PointLight};
use expedition33_continente_inacabado::ray::Ray;
use expedition33_continente_inacabado::renderer::{cast_ray, render, Shading};
use expedition33_continente_inacabado::reveal::RevealState;
use expedition33_continente_inacabado::scene::SpatialGroupId;
use expedition33_continente_inacabado::scene_builder::Blockout;
use expedition33_continente_inacabado::scenes::flying_waters::caja_del_volumen;
use expedition33_continente_inacabado::scenes::{anclas_del_diorama, safe_level_con, WaterPreset};
use nalgebra_glm::Vec3;

const ANCHO: usize = 800;
const ALTO: usize = 600;

fn brillo(color: Color) -> f32 {
    color.r + color.g + color.b
}

/// Carga el nivel seguro refractivo **con los assets**, o aborta.
///
/// Sin fallback a colores planos, a propósito. Un generador de evidencia
/// que cae a la escena sin texturas sigue imprimiendo números y guardando
/// PNG, pero de **otra escena** que la que la evidencia dice describir: los
/// albedos no son los mismos, así que ninguna luminancia medida vale. Es un
/// éxito silencioso, que es peor que un fallo.
fn nivel_texturizado() -> Blockout {
    let raiz = std::path::PathBuf::from(".");

    match safe_level_con(WaterPreset::RefractiveWater, Some(&raiz)) {
        Ok(nivel) => nivel,
        Err(e) => {
            eprintln!("error: {e}");
            eprintln!("  este generador de evidencia exige los assets: las cifras");
            eprintln!("  que imprime son luminancias, y sin texturas medirian otra escena.");
            eprintln!("  generalos con: cargo run --release --bin generate_assets");
            std::process::exit(1);
        }
    }
}

fn main() {
    let diorama = nivel_texturizado();

    let luces = luces_del_diorama(&diorama.anchors, &diorama.scale);
    let camara = diorama.hero_camera();

    let solo =
        |id: &str| -> Vec<PointLight> { luces.iter().filter(|l| l.id == id).copied().collect() };

    let configuraciones: [(&str, Vec<PointLight>); 4] = [
        ("rig-completo", luces.clone()),
        ("solo-l02", solo("L-02")),
        ("solo-l01", solo("L-01")),
        ("sin-luces", Vec::new()),
    ];

    println!("Sombras submarinas · Tarea 5.6");
    println!(
        "  escena    nivel seguro refractivo, {} primitivas, {} texturas",
        diorama.scene.objects.len(),
        diorama.scene.textures.len()
    );

    // ------------------------------------------------ los puntos que se miden
    //
    // Rejilla de rayos verticales sobre la huella del casco, disparados
    // desde justo por debajo de la superficie del agua. Ese origen
    // garantiza que lo primero que se toque sea una cara **expuesta**: el
    // casco esta apilado —cuerpo, cubierta, costillas— y un rayo que nazca
    // sobre la cara de una pieza puede nacer dentro de la de encima y medir
    // su cara interna, que no ve ninguna luz.
    let base = anclas_del_diorama().flying_waters_anchor;
    let (centro, tamano) = caja_del_volumen(base);
    let superficie = centro.y + tamano.y * 0.5;
    let (minimo, maximo) = (centro - tamano * 0.5, centro + tamano * 0.5);

    let piezas: Vec<usize> = diorama
        .scene
        .objects
        .iter()
        .enumerate()
        .filter(|(_, o)| o.spatial_group == SpatialGroupId::FlyingWaters)
        .filter(|(_, o)| {
            let caja = o.primitive.bounds();

            caja.min.x > minimo.x
                && caja.max.x < maximo.x
                && caja.min.z > minimo.z
                && caja.max.z < maximo.z
                && caja.min.y > minimo.y + 1.2
                && (caja.max.z - caja.min.z) > 0.5
        })
        .map(|(i, _)| i)
        .collect();

    let huella = piezas
        .iter()
        .map(|i| diorama.scene.objects[*i].primitive.bounds())
        .reduce(|a, b| a.union(&b))
        .expect("el casco tiene piezas");

    let mut muestras = Vec::new();

    for i in 0..24 {
        for j in 0..12 {
            let x = huella.min.x + (i as f32 + 0.5) / 24.0 * (huella.max.x - huella.min.x);
            let z = huella.min.z + (j as f32 + 0.5) / 12.0 * (huella.max.z - huella.min.z);
            let rayo = Ray::new(
                Vec3::new(x, superficie - 0.01, z),
                Vec3::new(0.0, -1.0, 0.0),
            );

            if let Some(impacto) =
                diorama
                    .accel
                    .intersect(&diorama.scene, &rayo, &mut TraversalStats::default())
            {
                if piezas.contains(&impacto.object_index) {
                    muestras.push(rayo);
                }
            }
        }
    }

    println!(
        "  casco     {} piezas, {} rayos que dan en ellas, superficie en y = {superficie:.2}",
        piezas.len(),
        muestras.len()
    );

    println!(
        "
  {:<14} {:>9} {:>9} {:>9} {:>10}",
        "luces", "minimo", "media", "maximo", "iluminadas"
    );

    for (nombre, rig) in &configuraciones {
        let mut minimo_b = f32::MAX;
        let mut maximo_b = 0.0_f32;
        let mut suma = 0.0;
        let mut iluminadas = 0;

        for rayo in &muestras {
            let trazar = |luces: &[PointLight]| {
                cast_ray(
                    rayo,
                    &diorama.scene,
                    &diorama.accel,
                    luces,
                    &RevealState::painted(),
                    Shading::Material,
                    &mut TraversalStats::default(),
                )
            };

            let b = brillo(trazar(rig));
            let ambiente = brillo(trazar(&[]));

            minimo_b = minimo_b.min(b);
            maximo_b = maximo_b.max(b);
            suma += b;

            if b > ambiente * 3.0 {
                iluminadas += 1;
            }
        }

        println!(
            "  {:<14} {:>9.4} {:>9.4} {:>9.4} {:>6} / {}",
            nombre,
            minimo_b,
            suma / muestras.len() as f32,
            maximo_b,
            iluminadas,
            muestras.len()
        );
    }

    // ------------------------------------------------ los renders
    println!();

    for (nombre, rig) in &configuraciones {
        let mut framebuffer = Framebuffer::new(ANCHO, ALTO);
        let stats = render(
            &mut framebuffer,
            &diorama.scene,
            &diorama.accel,
            rig,
            &RevealState::painted(),
            &camara,
            Shading::Material,
        );

        let destino = PathBuf::from("evidence/hito5").join(format!("sombras-{nombre}.png"));
        match framebuffer.save_png(&destino) {
            Ok(()) => println!(
                "  {:<14} {} ({} rayos de sombra)",
                nombre,
                destino.display(),
                stats.shadow_rays
            ),
            Err(e) => eprintln!("  no se pudo escribir {}: {e}", destino.display()),
        }
    }
}
