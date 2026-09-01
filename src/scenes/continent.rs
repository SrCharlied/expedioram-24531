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
    derive_orbit_radius, eye_at_yaw, measure_scene_radius, Blockout, SceneAnchors, SceneScale,
    HERO_YAW_DEGREES, LOOK_AT_HEIGHT_FRACTION,
};
use nalgebra_glm::Vec3;

/// Altura de la superficie del agua en la bahía.
const WATER_SURFACE_Y: f32 = 1.9;

/// Grises del blockout. Se diferencian por región a propósito: con
/// sombreado plano y sin luces, un gris único convierte la escena en una
/// silueta y deja de servir para juzgar la composición.
struct Paleta {
    plinto: MaterialId,
    continente: MaterialId,
    praderas: MaterialId,
    rompeolas: MaterialId,
    aguas: MaterialId,
    borde_roto: MaterialId,
    monolito: MaterialId,
}

impl Paleta {
    fn registrar(scene: &mut Scene) -> Self {
        let mut gris = |v: f32| scene.add_material(Material::new(Color::new(v, v, v * 1.02)));

        Paleta {
            plinto: gris(0.80),
            continente: gris(0.44),
            praderas: gris(0.60),
            rompeolas: gris(0.30),
            aguas: gris(0.68),
            // El borde roto es terreno, no agua: el inventario le asigna
            // wet_basalt. Compartir gris con el agua lo volvia invisible y
            // ocultaba justo lo que la toma hero tiene que mostrar.
            borde_roto: gris(0.22),
            monolito: gris(0.88),
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
///
/// La composición es **vertical**, no plana: Praderas es una meseta alta,
/// Rompeolas son los pilares que la sostienen por su borde frontal, y
/// Aguas Voladoras ocupa el nivel bajo al frente. Un diorama plano deja al
/// Monolito como una astilla y pierde su papel de eje visual.
pub fn blockout() -> Blockout {
    let mut scene = Scene::new();
    let paleta = Paleta::registrar(&mut scene);

    // Origen de la escena: la base del Monolito, sobre el terreno.
    let monolith_base_anchor = Vec3::zeros();
    let meadows_anchor = Vec3::new(-4.2, 5.6, -4.6);
    let breakwater_anchor = Vec3::new(-4.2, 2.4, -1.9);
    let bay_center = Vec3::new(0.0, 0.0, 4.2);
    let broken_edge_anchor = Vec3::new(0.0, 1.2, 6.6);
    let palette_anchor = Vec3::new(6.6, 0.4, 5.8);

    let monolith_height = monolito(&mut scene, monolith_base_anchor, paleta.monolito);

    plinto(&mut scene, paleta.plinto);
    continente(&mut scene, paleta.continente);
    praderas(&mut scene, meadows_anchor, paleta.praderas);
    rompeolas(&mut scene, breakwater_anchor, paleta.rompeolas);
    aguas_voladoras(
        &mut scene,
        bay_center,
        broken_edge_anchor,
        paleta.aguas,
        paleta.borde_roto,
    );

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
        hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
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
///
/// Tiene que dominar la silueta desde cualquier ángulo de la órbita: si no
/// sobresale claramente por encima de las tres regiones, la composición
/// pierde su centro y el finale se queda sin remate.
fn monolito(scene: &mut Scene, base: Vec3, material: MaterialId) -> f32 {
    // Cuatro masas desalineadas a propósito: debe leerse como una pieza
    // rota que se estrecha hacia arriba, no como una torre regular.
    let tramos = [
        (Vec3::new(0.00, 2.20, 0.00), Vec3::new(2.2, 4.4, 2.2)),
        (Vec3::new(0.25, 6.00, -0.20), Vec3::new(1.6, 3.2, 1.6)),
        (Vec3::new(-0.20, 9.00, 0.15), Vec3::new(1.1, 2.8, 1.1)),
        (Vec3::new(0.40, 11.20, 0.30), Vec3::new(0.7, 1.6, 0.7)),
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
        Vec3::new(0.0, -0.55, 0.0),
        Vec3::new(17.0, 1.1, 15.0),
        material,
        SpatialGroupId::Global,
        RevealGroup::Finale,
    );
}

/// Masas del arco costero: el nivel bajo del Continente.
fn continente(scene: &mut Scene, material: MaterialId) {
    let masas = [
        (Vec3::new(0.0, 0.10, -2.0), Vec3::new(12.0, 1.2, 7.0)),
        (Vec3::new(-4.5, 0.10, 1.5), Vec3::new(4.0, 1.2, 4.0)),
        (Vec3::new(5.0, 0.10, -0.5), Vec3::new(3.5, 1.2, 5.0)),
        (Vec3::new(4.6, 0.90, -3.6), Vec3::new(3.0, 1.6, 3.2)),
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

/// Praderas Primaverales: la meseta alta. Es el nivel superior del
/// Continente y lo que Rompeolas sostiene.
fn praderas(scene: &mut Scene, ancla: Vec3, material: MaterialId) {
    let masas = [
        (Vec3::new(0.0, -0.70, 0.0), Vec3::new(7.0, 1.4, 5.6)),
        (Vec3::new(-0.8, 0.40, -0.8), Vec3::new(4.4, 1.0, 3.4)),
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

/// Acantilado Rompeolas: los pilares que sostienen el borde frontal de la
/// meseta. La relación tiene que leerse en la silueta desde los cuatro
/// angulos: si los pilares no llegan hasta el fondo de la meseta, Praderas
/// parece flotar.
fn rompeolas(scene: &mut Scene, ancla: Vec3, material: MaterialId) {
    // Muro de contención bajo el borde de la meseta.
    masa(
        scene,
        ancla + Vec3::new(0.0, -0.30, -0.9),
        Vec3::new(6.4, 3.4, 1.8),
        material,
        SpatialGroupId::Breakwater,
        RevealGroup::Breakwater,
    );

    // Pilares de altura variable a lo largo del arco. La variación es
    // determinista: el blockout debe verse igual en cada corrida.
    for i in 0..5 {
        let t = i as f32;
        let altura = 3.2 + 0.7 * ((t * 1.7).sin() + 1.0);

        masa(
            scene,
            ancla + Vec3::new(-2.7 + t * 1.35, -2.2 + altura * 0.5, 0.6 + 0.3 * t.cos()),
            Vec3::new(0.8, altura, 0.8),
            material,
            SpatialGroupId::Breakwater,
            RevealGroup::Breakwater,
        );
    }
}

/// Aguas Voladoras: lecho, volumen de agua y borde roto al frente. Es la
/// región que encara la toma hero.
fn aguas_voladoras(
    scene: &mut Scene,
    centro: Vec3,
    borde: Vec3,
    material: MaterialId,
    material_borde: MaterialId,
) {
    // Lecho de la bahía, hundido respecto del terreno para que haya
    // volumen de agua real donde suspender el barco.
    masa(
        scene,
        centro + Vec3::new(0.0, 0.25, 0.0),
        Vec3::new(9.0, 0.8, 5.4),
        material,
        SpatialGroupId::FlyingWaters,
        RevealGroup::FlyingWaters,
    );

    // Volumen de agua: un solo cuboide cerrado, nunca varios apilados. Su
    // cara superior define `water_surface_y`.
    let alto_agua = 1.2;
    masa(
        scene,
        Vec3::new(centro.x, WATER_SURFACE_Y - alto_agua * 0.5, centro.z),
        Vec3::new(8.6, alto_agua, 5.0),
        material,
        SpatialGroupId::FlyingWaters,
        RevealGroup::FlyingWaters,
    );

    // Borde roto: cuboides irregulares de terreno en primer plano. Son
    // ellos los que producen el aspecto rasgado ocluyendo parcialmente la
    // cara frontal del agua; el AABB del volumen nunca se rasga.
    // Su altura tiene que superar `water_surface_y` con holgura: si solo la
    // roza, el borde deja de recortar la lamina de agua y la bahia se lee
    // como una losa plana.
    for i in 0..4 {
        let t = i as f32;
        let ancho = 1.9 + 0.5 * (t * 2.3).cos();
        let altura = 2.4 + 0.6 * (t * 1.1).sin();

        masa(
            scene,
            borde + Vec3::new(-3.2 + t * 2.1, -1.2 + altura * 0.5, 0.25 * (t * 1.9).sin()),
            Vec3::new(ancho, altura, 1.3),
            material_borde,
            SpatialGroupId::FlyingWaters,
            RevealGroup::FlyingWaters,
        );
    }
}
