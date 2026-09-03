//! Gate de Aguas Voladoras — Tarea 5.8.
//!
//! ```text
//! cargo run --release --example gate_flying_waters
//! ```
//!
//! Mide los seis criterios del plan sobre la toma hero a `800 × 600`, cada
//! uno contra un número y no contra una impresión:
//!
//! 1. La superficie devuelve skybox.
//! 2. El borde frontal permite ver el barco.
//! 3. El highlight del agua se ve.
//! 4. Barco, cadena y ancla legibles.
//! 5. Ni acné severo ni negro total.
//! 6. Tiempo en release registrado.
//!
//! La decisión de aprobar es humana. Esto produce la imagen y las cifras
//! con las que se decide.

use std::time::Instant;

use expedition33_continente_inacabado::accel::TraversalStats;
use expedition33_continente_inacabado::camera::Camera;
use expedition33_continente_inacabado::color::Color;
use expedition33_continente_inacabado::cuboid::Cuboid;
use expedition33_continente_inacabado::framebuffer::Framebuffer;
use expedition33_continente_inacabado::light::{diorama as luces_del_diorama, PointLight};
use expedition33_continente_inacabado::optics::refracted_ray;
use expedition33_continente_inacabado::ray::Ray;
use expedition33_continente_inacabado::renderer::{cast_ray, render, Shading, MAX_DEPTH};
use expedition33_continente_inacabado::reveal::RevealState;
use expedition33_continente_inacabado::scene::{MaterialId, SpatialGroupId};
use expedition33_continente_inacabado::scene_builder::Blockout;
use expedition33_continente_inacabado::scenes::flying_waters::caja_del_volumen;
use expedition33_continente_inacabado::scenes::{anclas_del_diorama, safe_level_con, WaterPreset};
use expedition33_continente_inacabado::skybox::Skybox;

const ANCHO: usize = 800;
const ALTO: usize = 600;
const REPETICIONES: usize = 7;

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

fn raiz() -> std::path::PathBuf {
    std::path::PathBuf::from(".")
}

/// Luminancia aproximada de un píxel ya codificado, en `0..=1`.
fn luma(pixel: u32) -> f32 {
    let r = ((pixel >> 16) & 0xFF) as f32;
    let g = ((pixel >> 8) & 0xFF) as f32;
    let b = (pixel & 0xFF) as f32;

    (0.2126 * r + 0.7152 * g + 0.0722 * b) / 255.0
}

fn brillo(color: Color) -> f32 {
    color.r + color.g + color.b
}

/// Qué es cada objeto de Aguas Voladoras, para poder clasificar impactos.
struct Partes {
    volumen: usize,
    volumen_material: MaterialId,
    casco: Vec<usize>,
    /// Cadena y ancla: se identifican por el material derivado, que es el
    /// único con `uv_scale = 12.0`. Geométricamente serían indistinguibles
    /// del kelp, que también es delgado y también vive dentro del volumen.
    metal: Vec<usize>,
    borde: Vec<usize>,
}

fn clasificar(diorama: &Blockout) -> Partes {
    let base = anclas_del_diorama().flying_waters_anchor;
    let (centro, tamano) = caja_del_volumen(base);
    let caja_volumen = Cuboid::centrado(centro, tamano).bounds;
    let (minimo, maximo) = (centro - tamano * 0.5, centro + tamano * 0.5);
    let cara_z = maximo.z;

    let mut partes = Partes {
        volumen: usize::MAX,
        volumen_material: MaterialId(0),
        casco: Vec::new(),
        metal: Vec::new(),
        borde: Vec::new(),
    };

    for (indice, objeto) in diorama.scene.objects.iter().enumerate() {
        if objeto.spatial_group != SpatialGroupId::FlyingWaters {
            continue;
        }

        let caja = objeto.primitive.bounds();

        if (caja.min - caja_volumen.min).magnitude() < 1e-5 {
            partes.volumen = indice;
            partes.volumen_material = objeto.final_material;
            continue;
        }

        if diorama.scene.material(objeto.final_material).uv_scale == 12.0 {
            partes.metal.push(indice);
            continue;
        }

        let dentro = caja.min.x > minimo.x
            && caja.max.x < maximo.x
            && caja.min.z > minimo.z
            && caja.max.z < maximo.z;

        if dentro && caja.min.y > minimo.y + 1.2 && (caja.max.z - caja.min.z) > 0.5 {
            partes.casco.push(indice);
        } else if caja.max.z > cara_z && caja.min.z < cara_z && (caja.max.y - caja.min.y) > 1.5 {
            // La altura descarta la masa principal del lecho, que mide 5.4
            // de fondo contra los 5.0 del volumen y tambien asoma por la
            // cara frontal. Ver la Tarea 5.4.
            partes.borde.push(indice);
        }
    }

    partes
}

/// Los rayos primarios de la toma hero, con lo que toca cada uno.
struct Impactos {
    /// Píxeles cuyo rayo primario toca la superficie del agua.
    superficie: Vec<(usize, Ray)>,
    /// Píxeles donde el barco se ve **directo**, sin cruzar el agua.
    casco_directo: Vec<usize>,
    /// Píxeles donde el barco se ve **a través** de la superficie.
    casco_refractado: Vec<usize>,
    /// Ídem para la cadena y el ancla.
    metal_directo: Vec<usize>,
    metal_refractado: Vec<usize>,
    /// Píxeles que dan en el borde roto.
    borde: Vec<usize>,
    /// Lo que encuentra el rayo refractado cuando no es barco ni metal:
    /// lecho, kelp, rocas, o la cara interna del propio volumen.
    otro_refractado: usize,
    /// Refractados que no encuentran nada y terminan en cielo.
    cielo_refractado: usize,
}

fn recorrer(diorama: &Blockout, camara: &Camera, partes: &Partes) -> Impactos {
    let mut impactos = Impactos {
        superficie: Vec::new(),
        casco_directo: Vec::new(),
        casco_refractado: Vec::new(),
        metal_directo: Vec::new(),
        metal_refractado: Vec::new(),
        borde: Vec::new(),
        otro_refractado: 0,
        cielo_refractado: 0,
    };

    let ior = diorama.scene.material(partes.volumen_material).ior;

    for y in 0..ALTO {
        for x in 0..ANCHO {
            let rayo = camara.ray_from_pixel(x, y, ANCHO, ALTO);
            let Some(primero) =
                diorama
                    .accel
                    .intersect(&diorama.scene, &rayo, &mut TraversalStats::default())
            else {
                continue;
            };

            let indice = primero.object_index;
            let pixel = y * ANCHO + x;

            if partes.casco.contains(&indice) {
                impactos.casco_directo.push(pixel);
            } else if partes.metal.contains(&indice) {
                impactos.metal_directo.push(pixel);
            } else if partes.borde.contains(&indice) {
                impactos.borde.push(pixel);
            } else if indice == partes.volumen {
                impactos.superficie.push((y * ANCHO + x, rayo));

                // Lo que hay detrás de la superficie: se repite el primer
                // nivel de refracción que hace el renderer.
                if let Some(dentro) = refracted_ray(&primero, &rayo.direction, ior) {
                    if let Some(segundo) = diorama.accel.intersect(
                        &diorama.scene,
                        &dentro,
                        &mut TraversalStats::default(),
                    ) {
                        if partes.casco.contains(&segundo.object_index) {
                            impactos.casco_refractado.push(pixel);
                        } else if partes.metal.contains(&segundo.object_index) {
                            impactos.metal_refractado.push(pixel);
                        } else {
                            impactos.otro_refractado += 1;
                        }
                    } else {
                        impactos.cielo_refractado += 1;
                    }
                }
            }
        }
    }

    impactos
}

fn trazar(diorama: &Blockout, luces: &[PointLight], rayo: &Ray) -> Color {
    cast_ray(
        rayo,
        &diorama.scene,
        &diorama.accel,
        luces,
        &RevealState::painted(),
        Shading::Material,
        &mut TraversalStats::default(),
    )
}

fn main() {
    // Aborta si faltan los assets, y no cae a colores planos: las cifras
    // de este gate son luminancias.
    let mut diorama = match safe_level_con(WaterPreset::RefractiveWater, Some(&raiz())) {
        Ok(nivel) => nivel,
        Err(e) => {
            eprintln!("error: {e}");
            eprintln!("  el gate mide luminancias y sin texturas mediria otra escena.");
            eprintln!("  generalos con: cargo run --release --bin generate_assets");
            std::process::exit(1);
        }
    };

    let luces = luces_del_diorama(&diorama.anchors, &diorama.scale);
    let camara = diorama.hero_camera();
    let partes = clasificar(&diorama);

    println!("Gate de Aguas Voladoras · Tarea 5.8");
    println!(
        "  escena     {} primitivas, profundidad {MAX_DEPTH}, {} x {}",
        diorama.scene.objects.len(),
        ANCHO,
        ALTO
    );
    println!(
        "  partes     volumen 1, casco {}, cadena y ancla {}, borde roto {}",
        partes.casco.len(),
        partes.metal.len(),
        partes.borde.len()
    );

    // ------------------------------------------------------ 6 · el tiempo
    let mut framebuffer = Framebuffer::new(ANCHO, ALTO);
    let mut tiempos = Vec::with_capacity(REPETICIONES);
    let mut stats = TraversalStats::default();

    for _ in 0..REPETICIONES {
        let inicio = Instant::now();
        stats = render(
            &mut framebuffer,
            &diorama.scene,
            &diorama.accel,
            &luces,
            &RevealState::painted(),
            &camara,
            Shading::Material,
        );
        tiempos.push(inicio.elapsed().as_secs_f64());
    }

    tiempos.sort_by(|a, b| a.partial_cmp(b).expect("no hay NaN"));

    let destino = std::path::PathBuf::from("evidence/hito5/gate-hero.png");
    guardar(&framebuffer, &destino);

    println!(
        "\n  6 · tiempo release   min {:.4} s | mediana {:.4} s | max {:.4} s  ({REPETICIONES} repeticiones)",
        tiempos[0],
        tiempos[REPETICIONES / 2],
        tiempos[REPETICIONES - 1]
    );
    println!(
        "      rayos            {} primarios, {} de sombra, {} reflejados, {} refractados",
        stats.primary_rays, stats.shadow_rays, stats.reflection_rays, stats.refraction_rays
    );

    // --------------------------------------------- 2 y 4 · qué se ve de qué
    let impactos = recorrer(&diorama, &camara, &partes);
    let total = ANCHO * ALTO;

    println!("\n  2 y 4 · qué alcanza el rayo primario, sobre {total} pixeles");
    println!(
        "      superficie del agua      {:>6}  ({:.2} %)",
        impactos.superficie.len(),
        100.0 * impactos.superficie.len() as f32 / total as f32
    );
    println!("      borde roto               {:>6}", impactos.borde.len());
    println!(
        "      casco directo            {:>6}   a traves del agua {:>6}",
        impactos.casco_directo.len(),
        impactos.casco_refractado.len()
    );
    println!(
        "      cadena y ancla directo   {:>6}   a traves del agua {:>6}",
        impactos.metal_directo.len(),
        impactos.metal_refractado.len()
    );
    println!(
        "      otro detras del agua     {:>6}   refractados a cielo {:>6}",
        impactos.otro_refractado, impactos.cielo_refractado
    );

    // ---------------------------------- 4 · legibilidad, en luminancia
    //
    // «Legible» no es una impresion: es contraste contra el entorno. Se
    // mide la luminancia media de los pixeles de cada parte sobre la imagen
    // ya codificada, que es la escala que el ojo compara.
    let stats_de = |pixeles: &[usize]| -> (f32, f32, f32) {
        if pixeles.is_empty() {
            return (0.0, 0.0, 0.0);
        }

        let mut minimo = f32::MAX;
        let mut maximo = 0.0_f32;
        let mut suma = 0.0;

        for p in pixeles {
            let l = luma(framebuffer.buffer[*p]);

            minimo = minimo.min(l);
            maximo = maximo.max(l);
            suma += l;
        }

        (minimo, suma / pixeles.len() as f32, maximo)
    };

    let superficie_px: Vec<usize> = impactos.superficie.iter().map(|(p, _)| *p).collect();
    let casco_px: Vec<usize> = impactos
        .casco_directo
        .iter()
        .chain(&impactos.casco_refractado)
        .copied()
        .collect();
    let metal_px: Vec<usize> = impactos
        .metal_directo
        .iter()
        .chain(&impactos.metal_refractado)
        .copied()
        .collect();

    println!(
        "
  4 · luminancia por parte (0..1, sobre la imagen final)"
    );
    println!(
        "      {:<24} {:>6} {:>8} {:>8} {:>8}",
        "parte", "px", "min", "media", "max"
    );

    for (nombre, pixeles) in [
        ("superficie del agua", &superficie_px),
        ("casco visible", &casco_px),
        ("cadena y ancla", &metal_px),
        ("borde roto", &impactos.borde),
    ] {
        let (minimo, media, maximo) = stats_de(pixeles);

        println!(
            "      {:<24} {:>6} {:>8.4} {:>8.4} {:>8.4}",
            nombre,
            pixeles.len(),
            minimo,
            media,
            maximo
        );
    }

    // El agua que **rodea** al casco, no la media global de la superficie.
    //
    // La primera version comparo la luminancia del casco contra la media de
    // los 11 000 pixeles de superficie del cuadro, que incluye el fondo de
    // la bahia y los highlights del borde lejano. Eso no es «el agua que lo
    // rodea»: es otra cosa. Aqui se dilata el conjunto de pixeles del casco
    // y se promedian **solo** los de superficie que caen en el anillo.
    let contorno_del_casco = {
        let mut es_casco = vec![false; ANCHO * ALTO];
        for p in &casco_px {
            es_casco[*p] = true;
        }

        let mut es_superficie = vec![false; ANCHO * ALTO];
        for p in &superficie_px {
            es_superficie[*p] = true;
        }

        const RADIO: i32 = 6;
        let mut anillo = Vec::new();

        for y in 0..ALTO {
            for x in 0..ANCHO {
                let p = y * ANCHO + x;
                // De superficie, y que **no** muestre el casco a traves del
                // agua: esos pixeles son casco, no entorno, y contarlos
                // hacia que el entorno subiera junto con la ganancia del
                // casco y el contraste no se moviera.
                if !es_superficie[p] || es_casco[p] {
                    continue;
                }

                let mut vecino_del_casco = false;

                for dy in -RADIO..=RADIO {
                    for dx in -RADIO..=RADIO {
                        let (vx, vy) = (x as i32 + dx, y as i32 + dy);

                        if vx < 0 || vy < 0 || vx >= ANCHO as i32 || vy >= ALTO as i32 {
                            continue;
                        }
                        if es_casco[vy as usize * ANCHO + vx as usize] {
                            vecino_del_casco = true;
                        }
                    }
                }

                if vecino_del_casco {
                    anillo.push(p);
                }
            }
        }

        anillo
    };

    let (_, media_contorno, _) = stats_de(&contorno_del_casco);
    let (_, media_casco, _) = stats_de(&casco_px);

    // Color medio, no solo luminancia: el contraste puede ser cromatico.
    let color_medio = |pixeles: &[usize]| -> (f32, f32, f32) {
        let mut suma = (0.0, 0.0, 0.0);

        for p in pixeles {
            let pixel = framebuffer.buffer[*p];

            suma.0 += ((pixel >> 16) & 0xFF) as f32;
            suma.1 += ((pixel >> 8) & 0xFF) as f32;
            suma.2 += (pixel & 0xFF) as f32;
        }

        let n = pixeles.len().max(1) as f32;

        (suma.0 / n, suma.1 / n, suma.2 / n)
    };

    let (cr, cg, cb) = color_medio(&casco_px);
    let (ar, ag, ab) = color_medio(&contorno_del_casco);

    println!("\n      color medio (bytes sRGB)");
    println!(
        "      casco             {cr:>6.1} {cg:>6.1} {cb:>6.1}   rojo/azul {:.2}",
        cr / cb
    );
    println!(
        "      agua que lo rodea {ar:>6.1} {ag:>6.1} {ab:>6.1}   rojo/azul {:.2}",
        ar / ab
    );

    // Y la cabeza que deja la cota fisica de la ganancia.
    let madera = diorama.scene.objects[partes.casco[0]].final_material;
    let material = diorama.scene.material(madera);

    if let Some(id) = material.albedo_texture {
        let pico = diorama.scene.texture(id).peak();

        println!(
            "\n      textura del casco: pico rojo {:.4}, techo de ganancia {:.2}, albedo actual {:.2}",
            pico.r,
            1.0 / pico.r,
            material.albedo.r
        );
    }

    println!(
        "\n      contraste casco / agua que lo rodea: {:.4} / {:.4} = {:.2}",
        media_casco,
        media_contorno,
        media_casco / media_contorno
    );
    println!(
        "      el anillo son {} pixeles de superficie a menos de 6 px del casco",
        contorno_del_casco.len()
    );

    // ------------------------------------------- 1 · la superficie devuelve cielo
    //
    // Se traza la superficie con el cielo real y con un cielo negro. Si el
    // reflejo no llegara al pixel, los dos darian lo mismo.
    let muestras: Vec<Ray> = impactos
        .superficie
        .iter()
        .step_by(7)
        .map(|(_, r)| *r)
        .collect();

    let con_cielo: Vec<Color> = muestras
        .iter()
        .map(|r| trazar(&diorama, &luces, r))
        .collect();

    let cielo_original = diorama.scene.skybox;
    diorama.scene.skybox = Skybox::Flat(Color::black());
    let sin_cielo: Vec<Color> = muestras
        .iter()
        .map(|r| trazar(&diorama, &luces, r))
        .collect();
    diorama.scene.skybox = cielo_original;

    let mut cambiaron = 0;
    let mut caida_media = 0.0;
    let mut caida_maxima = 0.0_f32;

    for (con, sin) in con_cielo.iter().zip(&sin_cielo) {
        let caida = brillo(*con) - brillo(*sin);

        caida_media += caida;
        caida_maxima = caida_maxima.max(caida);

        if caida > 0.01 {
            cambiaron += 1;
        }
    }

    println!(
        "\n  1 · la superficie devuelve cielo, sobre {} muestras",
        muestras.len()
    );
    println!(
        "      pixeles que cambian al apagar el cielo   {cambiaron} / {}",
        muestras.len()
    );
    println!(
        "      aporte del cielo    medio {:.4}   maximo {:.4}",
        caida_media / muestras.len() as f32,
        caida_maxima
    );

    // ------------------------------------------- 3 · el highlight del agua
    let material_agua = diorama.scene.material(partes.volumen_material);
    diorama.scene.palette[partes.volumen_material.0] = material_agua.with_specular(0.0, 1.0);
    let sin_brillo: Vec<Color> = muestras
        .iter()
        .map(|r| trazar(&diorama, &luces, r))
        .collect();
    diorama.scene.palette[partes.volumen_material.0] = material_agua;

    let mut con_highlight = 0;
    let mut mayor = 0.0_f32;

    for (con, sin) in con_cielo.iter().zip(&sin_brillo) {
        let aporte = brillo(*con) - brillo(*sin);

        mayor = mayor.max(aporte);
        if aporte > 0.02 {
            con_highlight += 1;
        }
    }

    println!("\n  3 · el highlight del agua");
    println!(
        "      pixeles con aporte especular             {con_highlight} / {}",
        muestras.len()
    );
    println!("      aporte maximo del especular              {mayor:.4}");

    // ------------------------------------- 5 · ni acne severo ni negro total
    let mut negros = 0;
    let mut moteados = 0;

    for y in 1..ALTO - 1 {
        for x in 1..ANCHO - 1 {
            let pixel = framebuffer.buffer[y * ANCHO + x];

            if pixel & 0x00FF_FFFF == 0 {
                negros += 1;
            }

            // Acné: un pixel mucho más oscuro que **todos** sus vecinos. Un
            // borde de sombra legítimo tiene vecinos oscuros a un lado; un
            // moteado por reimpacto está rodeado de luz.
            let propio = luma(pixel);
            let mut minimo_vecino = f32::MAX;

            for dy in -1i32..=1 {
                for dx in -1i32..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let vecino = framebuffer.buffer
                        [(y as i32 + dy) as usize * ANCHO + (x as i32 + dx) as usize];
                    minimo_vecino = minimo_vecino.min(luma(vecino));
                }
            }

            if propio < minimo_vecino * 0.5 && minimo_vecino > 0.05 {
                moteados += 1;
            }
        }
    }

    println!("\n  5 · limpieza de la imagen");
    println!("      pixeles en negro absoluto                {negros}");
    println!(
        "      pixeles aislados mas oscuros que todos sus vecinos   {moteados}  ({:.4} %)",
        100.0 * moteados as f32 / total as f32
    );
    // ------------------------- el barco y el metal desde otros yaw
    //
    // El diorama es orbital: si una pieza queda tapada desde la toma hero
    // no esta perdida, y conviene saber si el problema es de tamano o de
    // oclusion. Son dos arreglos distintos.
    println!("\n  visibilidad al orbitar (px que alcanzan cada parte)");
    println!(
        "      {:>6} {:>10} {:>16}",
        "yaw", "casco", "cadena y ancla"
    );

    for yaw in [45.0_f32, 90.0, 135.0, 180.0, 225.0, 270.0, 315.0] {
        let camara_yaw = diorama.camera_at_yaw(yaw);
        let vista = recorrer(&diorama, &camara_yaw, &partes);

        println!(
            "      {:>6.0} {:>10} {:>16}",
            yaw,
            vista.casco_directo.len() + vista.casco_refractado.len(),
            vista.metal_directo.len() + vista.metal_refractado.len()
        );
    }

    println!("\n  render     {}", destino.display());
}
