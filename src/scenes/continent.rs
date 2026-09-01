//! Blockout 1: la composición global en cuboides grises.
//!
//! Solo lo que el plan pide para este blockout: plinto, masas del arco
//! costero, las tres regiones marcadas por sus masas principales, la bahía
//! y el Monolito. Sin detalle, sin generadores y sin materiales finales.
//!
//! **Las coordenadas de este archivo son una propuesta, no una medición.**
//! El plan las declara explícitamente por validar: la Tarea 2.5 renderiza
//! cuatro ángulos y ajusta las anclas —no los materiales— hasta que la
//! composición se lea. Todo está expresado respecto de un ancla para que
//! mover una región sea cambiar un vector, no veinte.

use crate::color::Color;
use crate::cuboid::Cuboid;
use crate::ray_intersect::Material;
use crate::scene::{MaterialId, RevealGroup, Scene, SceneObject, SpatialGroupId};
use crate::scene_builder::{
    derive_orbit_radius, measure_scene_radius, Blockout, SceneAnchors, SceneScale,
    LOOK_AT_HEIGHT_FRACTION,
};
use nalgebra_glm::Vec3;

/// Altura de la superficie del agua en la bahía.
const WATER_SURFACE_Y: f32 = 1.4;

/// Grises del blockout. Se diferencian por región a propósito: con
/// sombreado plano y sin luces, un gris único convierte la escena en una
/// silueta y deja de servir para juzgar la composición.
struct Paleta {
    plinto: MaterialId,
    continente: MaterialId,
    praderas: MaterialId,
    rompeolas: MaterialId,
    aguas: MaterialId,
    monolito: MaterialId,
}

impl Paleta {
    fn registrar(scene: &mut Scene) -> Self {
        let mut gris = |v: f32| scene.add_material(Material::new(Color::new(v, v, v * 1.02)));

        Paleta {
            plinto: gris(0.82),
            continente: gris(0.46),
            praderas: gris(0.58),
            rompeolas: gris(0.34),
            aguas: gris(0.66),
            monolito: gris(0.72),
        }
    }
}

/// Añade un cuboide centrado, con su grupo espacial y de revelación.
fn masa(
    scene: &mut Scene,
    centro: Vec3,
    tamano: Vec3,
    material: MaterialId,
    spatial_group: SpatialGroupId,
    reveal_group: RevealGroup,
) {
    scene.add_object(SceneObject {
        primitive: Cuboid::centrado(centro, tamano).into(),
        initial_material: material,
        final_material: material,
        spatial_group,
        reveal_group,
    });
}

/// Construye el blockout y mide sus parámetros de escala.
///
/// El orden importa: primero la geometría, después la medición. Ni
/// `scene_radius` ni `monolith_height` se eligen a mano, y `orbit_radius`
/// se deriva de los dos.
pub fn blockout() -> Blockout {
    let mut scene = Scene::new();
    let paleta = Paleta::registrar(&mut scene);

    // Origen de la escena: la base del Monolito, sobre el terreno.
    let monolith_base_anchor = Vec3::zeros();
    let meadows_anchor = Vec3::new(-5.0, 3.2, -7.0);
    let breakwater_anchor = Vec3::new(6.0, 2.2, 1.5);
    let bay_center = Vec3::new(0.0, 0.0, 5.5);
    let broken_edge_anchor = Vec3::new(0.0, 1.0, 8.8);
    let palette_anchor = Vec3::new(-9.0, 0.2, 8.0);

    let monolith_height = monolito(&mut scene, monolith_base_anchor, paleta.monolito);

    plinto(&mut scene, paleta.plinto);
    continente(&mut scene, paleta.continente);
    praderas(&mut scene, meadows_anchor, paleta.praderas);
    rompeolas(&mut scene, breakwater_anchor, paleta.rompeolas);
    aguas_voladoras(&mut scene, bay_center, broken_edge_anchor, paleta.aguas);

    // ---- medición, ya con la geometría en su sitio ----
    let orbit_center = monolith_base_anchor;
    let scene_radius = measure_scene_radius(&scene, orbit_center);
    let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

    let look_at = orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0);

    let anchors = SceneAnchors {
        scene_origin: monolith_base_anchor,
        monolith_base_anchor,
        orbit_center,
        look_at,
        meadows_anchor,
        breakwater_anchor,
        flying_waters_anchor: Vec3::new(bay_center.x, WATER_SURFACE_Y, bay_center.z),
        palette_anchor,
        hero_camera_anchor: crate::scene_builder::eye_at_yaw(
            orbit_center,
            orbit_radius,
            crate::scene_builder::HERO_YAW_DEGREES,
        ),
        broken_edge_anchor,
    };

    let scale = SceneScale {
        scene_radius,
        monolith_height,
        water_surface_y: WATER_SURFACE_Y,
        orbit_radius,
    };

    Blockout {
        scene,
        anchors,
        scale,
    }
}

/// El eje visual del diorama. Devuelve su altura medida, que es lo que
/// alimenta el encuadre y la derivación del radio orbital.
fn monolito(scene: &mut Scene, base: Vec3, material: MaterialId) -> f32 {
    // Cuatro masas desalineadas a propósito: el Monolito debe leerse como
    // una pieza rota, no como una torre regular.
    let tramos = [
        (Vec3::new(0.0, 1.40, 0.0), Vec3::new(1.6, 2.8, 1.6)),
        (Vec3::new(0.15, 3.40, -0.10), Vec3::new(1.2, 1.6, 1.2)),
        (Vec3::new(-0.10, 4.90, 0.05), Vec3::new(0.9, 1.6, 0.9)),
        (Vec3::new(0.30, 5.90, 0.20), Vec3::new(0.5, 1.4, 0.5)),
    ];

    let mut cima: f32 = 0.0;
    for (offset, tamano) in tramos {
        masa(
            scene,
            base + offset,
            tamano,
            material,
            SpatialGroupId::Monolith,
            RevealGroup::Finale,
        );
        cima = cima.max(offset.y + tamano.y * 0.5);
    }

    cima
}

/// El lienzo sobre el que nace el Continente.
fn plinto(scene: &mut Scene, material: MaterialId) {
    masa(
        scene,
        Vec3::new(0.0, -0.60, 0.0),
        Vec3::new(22.0, 1.2, 20.0),
        material,
        SpatialGroupId::Global,
        RevealGroup::Finale,
    );
}

/// Masas del arco costero: la silueta del Continente, sin terrazas finas.
fn continente(scene: &mut Scene, material: MaterialId) {
    let masas = [
        (Vec3::new(0.0, -0.10, -3.0), Vec3::new(18.0, 1.2, 10.0)),
        (Vec3::new(-4.0, 0.80, -5.5), Vec3::new(9.0, 1.6, 6.0)),
        (Vec3::new(3.5, 0.60, -6.0), Vec3::new(7.0, 1.2, 5.0)),
        (Vec3::new(-7.5, 0.10, -1.0), Vec3::new(4.0, 1.0, 6.0)),
        (Vec3::new(7.0, 0.10, -1.5), Vec3::new(4.5, 1.0, 6.0)),
    ];

    for (centro, tamano) in masas {
        masa(
            scene,
            centro,
            tamano,
            material,
            SpatialGroupId::ContinentBackground,
            RevealGroup::Finale,
        );
    }
}

/// Praderas Primaverales: la meseta alta, al fondo.
fn praderas(scene: &mut Scene, ancla: Vec3, material: MaterialId) {
    let masas = [
        (Vec3::new(0.0, -1.0, 0.0), Vec3::new(8.0, 2.0, 6.0)),
        (Vec3::new(-1.0, 0.2, -1.0), Vec3::new(5.0, 1.0, 4.0)),
    ];

    for (offset, tamano) in masas {
        masa(
            scene,
            ancla + offset,
            tamano,
            material,
            SpatialGroupId::Meadows,
            RevealGroup::Meadows,
        );
    }
}

/// Acantilado Rompeolas: la masa de soporte y los pilares que la sostienen.
fn rompeolas(scene: &mut Scene, ancla: Vec3, material: MaterialId) {
    masa(
        scene,
        ancla + Vec3::new(0.0, -1.3, 0.0),
        Vec3::new(5.0, 2.6, 5.0),
        material,
        SpatialGroupId::Breakwater,
        RevealGroup::Breakwater,
    );

    // Pilares de altura variable siguiendo el borde del arco. La variación
    // es determinista: el blockout tiene que verse igual en cada corrida.
    for i in 0..5 {
        let t = i as f32;
        let altura = 2.2 + 0.55 * ((t * 1.7).sin() + 1.0);
        let centro = ancla + Vec3::new(-1.6 + t * 0.9, -2.6 + altura * 0.5, 2.9 + 0.35 * t.cos());

        masa(
            scene,
            centro,
            Vec3::new(0.7, altura, 0.7),
            material,
            SpatialGroupId::Breakwater,
            RevealGroup::Breakwater,
        );
    }
}

/// Aguas Voladoras: lecho, volumen de agua y borde roto al frente.
fn aguas_voladoras(scene: &mut Scene, centro: Vec3, borde: Vec3, material: MaterialId) {
    // Lecho.
    masa(
        scene,
        centro + Vec3::new(0.0, -0.2, 0.0),
        Vec3::new(12.0, 1.0, 7.0),
        material,
        SpatialGroupId::FlyingWaters,
        RevealGroup::FlyingWaters,
    );

    // Volumen de agua: un solo cuboide cerrado, nunca varios apilados. Su
    // cara superior define `water_surface_y`.
    let alto_agua = 1.6;
    masa(
        scene,
        Vec3::new(centro.x, WATER_SURFACE_Y - alto_agua * 0.5, centro.z),
        Vec3::new(11.5, alto_agua, 6.5),
        material,
        SpatialGroupId::FlyingWaters,
        RevealGroup::FlyingWaters,
    );

    // Borde roto: cuboides irregulares de terreno en primer plano. Son
    // ellos los que producen el aspecto rasgado ocluyendo parcialmente la
    // cara frontal del agua; el AABB del volumen nunca se rasga.
    for i in 0..4 {
        let t = i as f32;
        let ancho = 2.0 + 0.6 * (t * 2.3).cos();
        let altura = 1.0 + 0.4 * (t * 1.1).sin();

        masa(
            scene,
            borde + Vec3::new(-4.2 + t * 2.8, -0.3 + altura * 0.5, 0.3 * (t * 1.9).sin()),
            Vec3::new(ancho, altura, 1.4),
            material,
            SpatialGroupId::FlyingWaters,
            RevealGroup::FlyingWaters,
        );
    }
}
