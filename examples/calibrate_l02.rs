//! Calibración de `L-02` con el blockout real — Tarea 5.7.
//!
//! ```text
//! cargo run --release --example calibrate_l02
//! ```
//!
//! Los cinco pasos que exige el inventario, en orden:
//!
//! 1. Medir la distancia real de `L-02` al centro **visible** del barco.
//! 2. Medir la distancia al objeto obligatorio más lejano de Aguas.
//! 3. Elegir `range` para que los dos queden legibles.
//! 4. Elegir `E_boat` y derivar
//!    `intensity = E_boat × (1 + (distance_boat / range)²)`.
//! 5. Registrar lo medido; no heredar `2.0 / 0.20S` sin validar.
//!
//! No decide nada por sí solo: imprime el barrido con el que se decide, y
//! los valores elegidos se escriben en `light::diorama`.

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

/// Guarda un PNG de evidencia, o **aborta**.
///
/// No basta con abortar cuando faltan los assets de entrada: un generador
/// de evidencia que imprime sus cifras y no logra escribir la imagen
/// terminaba con código `0`, así que un guion que lo invoque lo daba por
/// bueno. Abortar al cargar y no al escribir deja la mitad de la promesa
/// sin cumplir.
fn guardar(framebuffer: &Framebuffer, destino: &std::path::Path) {
    if let Err(error) = framebuffer.save_png(destino) {
        eprintln!("error: no se pudo escribir {}: {error}", destino.display());
        std::process::exit(1);
    }
}

fn brillo(color: Color) -> f32 {
    color.r + color.g + color.b
}

/// De lineal a byte sRGB, que es la escala en la que se juzga «legible».
fn byte(canal: f32) -> u32 {
    let c = canal.clamp(0.0, 1.0);
    let srgb = if c <= 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    };

    (srgb * 255.0).round() as u32
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
    // Con los assets, no sin ellos: el barrido de `E_boat` se lee en bytes
    // sRGB del casco, y la madera texturizada no tiene el albedo de la
    // madera plana. La primera versión de este ejemplo medía la escena sin
    // texturas y calibraba contra un casco que no es el que se presenta.
    let diorama = nivel_texturizado();
    let luces = luces_del_diorama(&diorama.anchors, &diorama.scale);
    let l02 = *luces
        .iter()
        .find(|l| l.id == "L-02")
        .expect("el rig tiene L-02");

    let s = diorama.scale.scene_radius;
    let base = anclas_del_diorama().flying_waters_anchor;
    let (centro, tamano) = caja_del_volumen(base);
    let superficie = centro.y + tamano.y * 0.5;
    let (minimo, maximo) = (centro - tamano * 0.5, centro + tamano * 0.5);

    // ------------------------------------------- 1 · el centro visible del barco
    //
    // No el centro de la caja del casco: el **centro visible**, que es lo
    // que pide el inventario. Se obtiene promediando los puntos donde un
    // rayo vertical, disparado desde justo bajo la superficie del agua,
    // toca una pieza del casco. Es la misma rejilla con la que se validaron
    // las sombras en la Tarea 5.6.
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

    let mut visibles = Vec::new();

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
                    visibles.push(impacto.point);
                }
            }
        }
    }

    let centro_visible =
        visibles.iter().fold(Vec3::zeros(), |acc, p| acc + p) / visibles.len() as f32;
    let distance_boat = (l02.position - centro_visible).magnitude();

    // ------------------------------------------- 2 · el obligatorio más lejano
    //
    // Todas las entradas de Aguas Voladoras del nivel seguro son
    // obligatorias: `A-09` y `A-10` son las opcionales y valen cero
    // primitivas. Se mide al centro de cada objeto, que es lo
    // representativo para iluminarlo, y se reporta también la esquina más
    // lejana como cota superior.
    let mut distance_farthest = 0.0_f32;
    let mut esquina_mas_lejana = 0.0_f32;
    let mut cual = 0;

    for (indice, objeto) in diorama.scene.objects.iter().enumerate() {
        if objeto.spatial_group != SpatialGroupId::FlyingWaters {
            continue;
        }

        let caja = objeto.primitive.bounds();
        let d = (l02.position - caja.centro()).magnitude();

        if d > distance_farthest {
            distance_farthest = d;
            cual = indice;
        }

        for cx in [caja.min.x, caja.max.x] {
            for cy in [caja.min.y, caja.max.y] {
                for cz in [caja.min.z, caja.max.z] {
                    let e = (l02.position - Vec3::new(cx, cy, cz)).magnitude();
                    esquina_mas_lejana = esquina_mas_lejana.max(e);
                }
            }
        }
    }

    println!("Calibración de L-02 · Tarea 5.7\n");
    println!("  scene_radius                    S = {s:.4}");
    println!("  posición de L-02                  {:?}", l02.position);
    println!(
        "  centro visible del barco          {centro_visible:?}  ({} muestras)",
        visibles.len()
    );
    println!(
        "  distance_boat                     {distance_boat:.4}  = {:.4} S",
        distance_boat / s
    );
    println!(
        "  distance_farthest (centro)        {distance_farthest:.4}  = {:.4} S   objeto {cual}",
        distance_farthest / s
    );
    println!(
        "  distance_farthest (esquina)       {esquina_mas_lejana:.4}  = {:.4} S",
        esquina_mas_lejana / s
    );
    println!("\n  valores heredados, sin validar    intensity 2.0, range 0.20 S");

    // ------------------------------------------- 3 · barrido de range
    let atenuacion = |intensity: f32, range: f32, d: f32| intensity / (1.0 + (d / range).powi(2));

    println!(
        "\n  {:>8} {:>8} {:>10} {:>10} {:>9}",
        "range", "= S x", "at(barco)", "at(lejano)", "lejano/barco"
    );

    for fraccion in [0.15_f32, 0.20, 0.25, 0.30, 0.40, 0.55, 0.80] {
        let range = fraccion * s;
        let a_barco = atenuacion(1.0, range, distance_boat);
        let a_lejano = atenuacion(1.0, range, distance_farthest);

        println!(
            "  {:>8.4} {:>8.2} {:>10.4} {:>10.4} {:>8.1} %",
            range,
            fraccion,
            a_barco,
            a_lejano,
            100.0 * a_lejano / a_barco
        );
    }

    // ------------------------------------------- 4 · E_boat y la intensidad
    //
    // `E_boat` se elige por el brillo que se quiere en el casco, no a ojo.
    // Se mide el resultado sobre las mismas muestras visibles.
    println!(
        "\n  {:>7} {:>7} {:>10} {:>10} {:>8} {:>8}",
        "E_boat", "range", "intensity", "media", "byte", "byte max"
    );

    for e_boat in [1.0_f32, 1.5, 2.0, 2.5, 3.0, 4.0] {
        for fraccion in [0.20_f32, 0.30, 0.40] {
            let range = fraccion * s;
            let intensity = e_boat * (1.0 + (distance_boat / range).powi(2));

            let candidata = PointLight {
                intensity,
                range,
                ..l02
            };

            let mut suma = 0.0;
            let mut maxima = 0.0_f32;

            for punto in &visibles {
                let rayo = Ray::new(punto + Vec3::new(0.0, 0.06, 0.0), Vec3::new(0.0, -1.0, 0.0));
                let color = cast_ray(
                    &rayo,
                    &diorama.scene,
                    &diorama.accel,
                    &[candidata],
                    &RevealState::painted(),
                    Shading::Material,
                    &mut TraversalStats::default(),
                );

                suma += brillo(color);
                maxima = maxima.max(color.r.max(color.g).max(color.b));
            }

            let media = suma / visibles.len() as f32;

            println!(
                "  {:>7.2} {:>7.2} {:>10.4} {:>10.4} {:>8} {:>8}",
                e_boat,
                fraccion,
                intensity,
                media,
                byte(media / 3.0),
                byte(maxima)
            );
        }
    }

    // ------------------------------------------- 5 · el antes y el despues
    //
    // Los dos renders salen de aqui y no de dos corridas separadas del
    // binario, porque el «antes» **ya no se puede reproducir** de otra
    // forma: los valores heredados no viven en ninguna parte del codigo.
    // La primera version de esta evidencia apunto el antes a
    // `safe-refractive-water.png`, y una remedicion posterior lo sobrescribio
    // con la escena ya calibrada. El antes se habia perdido.
    let heredada = PointLight {
        intensity: 2.0,
        range: 0.20 * s,
        ..l02
    };

    let camara = diorama.hero_camera();
    let otras: Vec<PointLight> = luces.iter().filter(|l| l.id != "L-02").copied().collect();

    for (nombre, rig) in [
        ("l02-antes", {
            let mut v = otras.clone();
            v.push(heredada);
            v
        }),
        ("l02-despues", luces.clone()),
    ] {
        let mut framebuffer = Framebuffer::new(800, 600);
        render(
            &mut framebuffer,
            &diorama.scene,
            &diorama.accel,
            &rig,
            &RevealState::painted(),
            &camara,
            Shading::Material,
        );

        let destino = std::path::PathBuf::from("evidence/hito5").join(format!("{nombre}.png"));
        guardar(&framebuffer, &destino);

        println!("\n  {nombre:<12} {}", destino.display());
    }
}
