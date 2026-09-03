//! Diagnóstico de oclusión del conjunto cadena-ancla.
//!
//! ```text
//! cargo run --release --example chain_placement
//! ```
//!
//! Sirvió para encontrar **qué** tapaba la cadena en la toma hero, cuando el
//! gate de la Tarea 5.8 midió `61` píxeles contra `137` a `yaw 45°`. La
//! respuesta fue el bloque `5` del borde roto, y de ahí salió la muesca.
//!
//! # Lo que mide, y lo que NO mide
//!
//! Mide **centros de pieza con línea de visión libre**: para cada uno de los
//! once centros, si el segmento del ojo hasta él está libre de oclusores.
//!
//! Eso **no** es legibilidad, y su resultado no debe leerse como tal. Un
//! centro puede estar tapado mientras varias caras de la misma pieza se ven
//! perfectamente, y de hecho es lo que pasa: los primeros eslabones se
//! ocultan el centro entre ellos —que es lo que hace una cadena vista de
//! canto— y el gate rasterizado cuenta `167` píxeles de superficie visible
//! al mismo tiempo que este ejemplo reporta `0` centros libres.
//!
//! **El gate de legibilidad es el recorrido rasterizado de
//! `gate_flying_waters`**, que cuenta superficie visible píxel por píxel.
//! Esto es un instrumento de diagnóstico: sirve para saber quién ocluye y
//! hacia dónde conviene moverse, no para aprobar nada.

use expedition33_continente_inacabado::accel::TraversalStats;
use expedition33_continente_inacabado::light::GroupMask;
use expedition33_continente_inacabado::ray::Ray;
use expedition33_continente_inacabado::ray_intersect::RayIntersect;
use expedition33_continente_inacabado::scenes::{anclas_del_diorama, safe_level, WaterPreset};
use expedition33_continente_inacabado::EPSILON;
use nalgebra_glm::Vec3;

/// Los once puntos del conjunto, tal como los coloca `flying_waters`.
fn conjunto(ancla: Vec3, desplazamiento: Vec3) -> Vec<Vec3> {
    let arriba = ancla + Vec3::new(0.9, 2.0, 0.35) + desplazamiento;
    let abajo = ancla + Vec3::new(2.1, 0.95, 0.9) + desplazamiento;
    let base = ancla + Vec3::new(2.2, 0.95, 0.95) + desplazamiento;

    let mut puntos = Vec::with_capacity(11);

    for i in 0..8 {
        let t = (i as f32 + 0.5) / 8.0;
        let comba = -0.20 * (t * std::f32::consts::PI).sin();

        puntos.push(arriba + (abajo - arriba) * t + Vec3::new(0.0, comba, 0.0));
    }

    puntos.push(base);
    puntos.push(base + Vec3::new(0.0, -0.3, 0.0));
    puntos.push(base + Vec3::new(0.0, 0.3, 0.0));

    puntos
}

fn main() {
    let diorama = safe_level(WaterPreset::RefractiveWater);
    let ancla = anclas_del_diorama().flying_waters_anchor;
    let ojo = diorama.anchors.hero_camera_anchor;

    // Visible = el segmento del ojo al punto no lo corta nada. Se consultan
    // todos los grupos como oclusores: lo que tapa es el borde roto, que
    // vive en Aguas Voladoras, pero no hay que asumirlo.
    let visibles = |desplazamiento: Vec3| -> usize {
        conjunto(ancla, desplazamiento)
            .iter()
            .filter(|punto| {
                let hacia = *punto - ojo;
                let distancia = hacia.magnitude();
                let direccion = hacia / distancia;

                !diorama.accel.occluded(
                    &diorama.scene,
                    &Ray::new(ojo, direccion),
                    distancia - EPSILON * 4.0,
                    GroupMask::ALL,
                    &mut TraversalStats::default(),
                )
            })
            .count()
    };

    println!("Diagnóstico de oclusión del conjunto cadena-ancla");
    println!("Métrica: centros de pieza con línea de visión libre, de 11.");
    println!("NO es legibilidad: eso lo mide `gate_flying_waters` por pixel.\n");
    println!("  ojo hero en {ojo:?}");
    println!("\n  {:>6}", "dy \\ dx");

    let dxs = [-1.2_f32, -0.8, -0.4, 0.0, 0.4, 0.8];
    print!("  {:>7}", "");
    for dx in dxs {
        print!("{dx:>7.1}");
    }
    println!();

    for dy in [0.0_f32, 0.1, 0.2, 0.3, 0.4, 0.5] {
        print!("  {dy:>7.1}");

        for dx in dxs {
            print!("{:>7}", visibles(Vec3::new(dx, dy, 0.0)));
        }

        println!();
    }

    println!(
        "\n  actual: {} de 11 centros con linea de vision libre",
        visibles(Vec3::zeros())
    );

    // Y **quien** las tapa. Diagnosticar antes de mover: si el oclusor es
    // el borde roto hay que salir de detras de el, y si es el propio casco
    // hay que alejarse de el. Son direcciones distintas.
    println!("\n  oclusor de cada pieza en la posicion actual");

    for (i, punto) in conjunto(ancla, Vec3::zeros()).iter().enumerate() {
        let hacia = punto - ojo;
        let distancia = hacia.magnitude();
        let direccion = hacia / distancia;
        let rayo = Ray::new(ojo, direccion);

        // El primer **oclusor**, no el primer impacto: el volumen de agua y
        // el kelp llevan `ShadowMode::Ignore` y `occluded` los salta, asi
        // que reportarlos como oclusores manda a buscar en el lugar
        // equivocado. Se recorre a mano y se queda el mas cercano que
        // realmente bloquea.
        let mut oclusor = None;
        let mut mas_cerca = distancia - EPSILON * 4.0;

        for (indice, objeto) in diorama.scene.objects.iter().enumerate() {
            let Some(h) = objeto.primitive.ray_intersect(&rayo) else {
                continue;
            };

            if h.distance < mas_cerca
                && diorama
                    .scene
                    .material(objeto.final_material)
                    .blocks_shadows()
            {
                mas_cerca = h.distance;
                oclusor = Some((indice, h.point.y));
            }
        }

        match oclusor {
            Some((indice, altura)) => {
                let caja = diorama.scene.objects[indice].primitive.bounds();
                let lados = caja.max - caja.min;

                println!(
                    "      pieza {i:>2}  <- objeto {indice:>3}  lados {:>5.2} x{:>5.2} x{:>5.2}  impacto en y {altura:>5.2}",
                    lados.x, lados.y, lados.z
                );
            }
            None => println!("      pieza {i:>2}  visible"),
        }
    }
}
