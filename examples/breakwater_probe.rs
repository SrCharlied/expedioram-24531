//! Cuantifica el ahorro de partir R-01 en cuatro clusters, sobre un barrido
//! de rayos que roza el arco. No es un benchmark de tiempo.
use expedition33_continente_inacabado::accel::{ClusterPlan, SceneAccel, TraversalStats};
use expedition33_continente_inacabado::color::Color;
use expedition33_continente_inacabado::ray::Ray;
use expedition33_continente_inacabado::ray_intersect::Material;
use expedition33_continente_inacabado::scene::Scene;
use expedition33_continente_inacabado::scenes::breakwater::{generar, Arco, DetailLevel};
use nalgebra_glm::Vec3;

fn main() {
    for nivel in [DetailLevel::Safe, DetailLevel::Target] {
        let mut scene = Scene::new();
        let mut plan = ClusterPlan::new();
        let material = scene.add_material(Material::new(Color::new(0.3, 0.3, 0.32)));
        generar(&mut scene, &mut plan, &Arco::default(), nivel, material);

        let particionado = SceneAccel::build_from_plan(&scene, &plan).unwrap();
        let entero = SceneAccel::build(&scene).unwrap();

        let (mut con, mut sin) = (TraversalStats::default(), TraversalStats::default());
        let mut rayos = 0;

        for i in 0..120 {
            for j in 0..40 {
                let x = -6.0 + 12.0 * (i as f32 / 119.0);
                let y = 0.2 + 5.0 * (j as f32 / 39.0);
                let ray = Ray::new(Vec3::new(x, y, 9.0), Vec3::new(0.0, 0.0, -1.0));
                particionado.intersect(&scene, &ray, &mut con);
                entero.intersect(&scene, &ray, &mut sin);
                rayos += 1;
            }
        }

        println!(
            "  {:?}  ({} pilares, {} rayos)",
            nivel,
            scene.objects.len(),
            rayos
        );
        println!(
            "     un AABB unico    {:>7} pruebas de primitiva",
            sin.primitive_tests
        );
        println!(
            "     cuatro clusters  {:>7} pruebas de primitiva",
            con.primitive_tests
        );
        println!(
            "     ahorro           {:>6.1}%",
            100.0 * (1.0 - con.primitive_tests as f64 / sin.primitive_tests as f64)
        );
    }
}
