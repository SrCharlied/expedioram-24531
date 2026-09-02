//! Constructores de escena, uno por región, más el ensamblado del nivel
//! seguro.
//!
//! El inventario fija conteos exactos por entrada y los suma a **160
//! primitivas trazables** en nivel seguro. Esos números no son
//! orientativos: son el presupuesto contra el que se mide el rendimiento,
//! así que el ensamblado los comprueba entrada por entrada en vez de
//! confiar en que cuadren.

pub mod breakwater;
pub mod continent;
pub mod flying_waters;
pub mod meadows;

use crate::accel::{ClusterPlan, SceneAccel};
use crate::color::Color;
use crate::cuboid::Cuboid;
use crate::material::{Material, ShadowMode};
use crate::scene::{MaterialId, Scene};
use crate::scene::{RevealGroup, SceneObject, SpatialGroupId};
use crate::scene_builder::{
    derive_orbit_radius, eye_at_yaw, measure_scene_radius, Blockout, SceneAnchors, SceneScale,
    HERO_YAW_DEGREES, LOOK_AT_HEIGHT_FRACTION,
};
use nalgebra_glm::Vec3;

/// Generador pseudoaleatorio determinista, sin dependencias.
///
/// El inventario exige `seed: fixed` para toda entrada generada: el
/// blockout y los renders de evidencia tienen que salir idénticos en cada
/// corrida, y dos capturas que difieran por azar no sirven para comparar
/// nada.
pub(crate) struct Xorshift32(u32);

impl Xorshift32 {
    pub(crate) fn new(semilla: u32) -> Self {
        // El cero es un punto fijo del xorshift: se quedaría clavado.
        Xorshift32(if semilla == 0 { 0x9E37_79B9 } else { semilla })
    }

    /// Siguiente valor en `0.0..1.0`.
    pub(crate) fn siguiente(&mut self) -> f32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;

        // Los 24 bits altos: los bajos de un xorshift están peor
        // distribuidos.
        (x >> 8) as f32 / (1u32 << 24) as f32
    }

    /// Siguiente valor en `-1.0..1.0`.
    pub(crate) fn simetrico(&mut self) -> f32 {
        self.siguiente() * 2.0 - 1.0
    }
}

/// Añade un cuboide centrado con su grupo espacial y de revelación.
///
/// En nivel seguro el material inicial y el final coinciden: la
/// interpolación desde `canvas_unpainted` llega en la Tarea 4.4.
pub(crate) fn masa(
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

/// Cómo se representa `A-01`, el volumen de agua, antes de que exista
/// óptica.
///
/// El inventario obliga a distinguirlos porque **miden cosas distintas**, y
/// confundirlos daría un benchmark optimista por accidente.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaterPreset {
    /// El volumen **no** se inserta como primitiva trazable. Quedan 159, y
    /// los rayos alcanzan barco, mástil, cadena, ancla, kelp, rocas y
    /// lecho.
    ///
    /// Es el preset canónico del benchmark temprano: mide el coste real de
    /// mirar dentro de la bahía.
    InteriorVisible,
    /// El volumen se inserta como cuboide azul opaco. Conserva las 160
    /// primitivas y sirve para validar composición.
    ///
    /// **No sirve para aprobar rendimiento:** al ser opaco oculta las 44
    /// primitivas del interior, que dejan de probarse. Un tiempo medido así
    /// parece bueno por la razón equivocada.
    OpaqueWater,
}

/// Los cinco materiales finales del inventario más el lienzo inicial.
///
/// Los techos ópticos ya llevan los valores del inventario aunque el Hito 5
/// sea quien los use: forman parte de la definición del material, no del
/// momento en que se leen.
#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub canvas: MaterialId,
    pub water: MaterialId,
    pub wet_basalt: MaterialId,
    pub aged_wood: MaterialId,
    pub meadow: MaterialId,
    pub pictorial_crystal: MaterialId,
}

impl Palette {
    pub fn registrar(scene: &mut Scene) -> Self {
        // Lienzo sin pintar: marfil, mate, opaco.
        let canvas = scene.add_material(Material::new(Color::new(0.90, 0.87, 0.79)));

        // Agua: caps 0.9 y no 1.0. Con caps unitarios kl queda en cero y el
        // albedo del agua no contribuye nunca; con 0.9 queda un 10 % fijo
        // que porta su color propio.
        let water = scene.add_material(Material {
            reflection_cap: 0.9,
            transmission_cap: 0.9,
            ior: 1.333,
            specular_strength: 0.18,
            shininess: 128.0,
            shadow_mode: ShadowMode::Ignore,
            ..Material::new(Color::new(0.22, 0.45, 0.72))
        });

        // Roca húmeda: brillo local alto, cero rebotes.
        let wet_basalt = scene.add_material(Material::wet_basalt(Color::new(0.26, 0.27, 0.30)));

        let aged_wood = scene
            .add_material(Material::new(Color::new(0.32, 0.22, 0.14)).with_specular(0.06, 16.0));

        let meadow = scene
            .add_material(Material::new(Color::new(0.30, 0.52, 0.24)).with_specular(0.04, 8.0));

        // Cristal pictórico: brillo y transparencia parcial. El modo de
        // sombra lo decide cada objeto, no el material.
        let pictorial_crystal = scene.add_material(Material {
            reflection_cap: 0.35,
            transmission_cap: 0.25,
            ior: 1.45,
            specular_strength: 0.55,
            shininess: 110.0,
            ..Material::new(Color::new(0.62, 0.86, 0.92))
        });

        Palette {
            canvas,
            water,
            wet_basalt,
            aged_wood,
            meadow,
            pictorial_crystal,
        }
    }
}

/// Presupuesto de primitivas por región, según el inventario.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Presupuesto {
    pub global: usize,
    pub meadows: usize,
    pub breakwater: usize,
    pub flying_waters: usize,
}

impl Presupuesto {
    pub fn total(&self) -> usize {
        self.global + self.meadows + self.breakwater + self.flying_waters
    }
}

/// Conteos del nivel seguro. Vienen del presupuesto consolidado del
/// inventario y suman 160.
pub const SAFE: Presupuesto = Presupuesto {
    global: 27,
    meadows: 37,
    breakwater: 38,
    flying_waters: 58,
};

/// Construye el nivel seguro completo.
///
/// Comprueba el conteo de cada región contra el presupuesto en vez de
/// confiar en que cuadre: una entrada que se pase de largo desplazaría el
/// total y el benchmark mediría otra escena distinta de la declarada.
pub fn safe_level(water: WaterPreset) -> Blockout {
    let mut scene = Scene::new();
    let mut plan = ClusterPlan::new();
    let paleta = Palette::registrar(&mut scene);

    let anchors_base = anclas_del_diorama();

    let antes = scene.objects.len();
    let monolith_height = continent::globales(&mut scene, &paleta, &anchors_base);
    verificar("Global", scene.objects.len() - antes, SAFE.global);

    let antes = scene.objects.len();
    meadows::praderas(&mut scene, &paleta, anchors_base.meadows_anchor);
    verificar("Praderas", scene.objects.len() - antes, SAFE.meadows);

    let antes = scene.objects.len();
    breakwater::rompeolas_seguro(
        &mut scene,
        &mut plan,
        &paleta,
        anchors_base.breakwater_anchor,
    );
    verificar("Rompeolas", scene.objects.len() - antes, SAFE.breakwater);

    let antes = scene.objects.len();
    let water_surface_y = flying_waters::aguas_voladoras(
        &mut scene,
        &paleta,
        anchors_base.flying_waters_anchor,
        anchors_base.broken_edge_anchor,
        water,
    );
    let esperado = match water {
        WaterPreset::OpaqueWater => SAFE.flying_waters,
        // Sin el volumen de agua queda una primitiva menos.
        WaterPreset::InteriorVisible => SAFE.flying_waters - 1,
    };
    verificar("Aguas Voladoras", scene.objects.len() - antes, esperado);

    // Medición: primero la geometría, después la escala.
    let orbit_center = anchors_base.monolith_base_anchor;
    let scene_radius = measure_scene_radius(&scene, orbit_center);
    let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

    let anchors = SceneAnchors {
        look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
        hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
        flying_waters_anchor: Vec3::new(
            anchors_base.flying_waters_anchor.x,
            water_surface_y,
            anchors_base.flying_waters_anchor.z,
        ),
        ..anchors_base
    };

    let accel =
        SceneAccel::build_from_plan(&scene, &plan).expect("el nivel seguro tiene geometria");

    Blockout {
        scene,
        accel,
        anchors,
        scale: SceneScale {
            scene_radius,
            monolith_height,
            water_surface_y,
            orbit_radius,
        },
    }
}

fn verificar(region: &str, obtenido: usize, esperado: usize) {
    assert_eq!(
        obtenido, esperado,
        "{region} genero {obtenido} primitivas y el inventario declara {esperado}"
    );
}

/// Anclas del diorama, compartidas por el blockout y el nivel seguro.
///
/// Son las que quedaron aprobadas en la validación del Blockout 1; el nivel
/// seguro puebla esa misma composición con el detalle del inventario en vez
/// de proponer una nueva.
pub fn anclas_del_diorama() -> SceneAnchors {
    let origen = Vec3::zeros();

    SceneAnchors {
        scene_origin: origen,
        monolith_base_anchor: origen,
        orbit_center: origen,
        look_at: origen,
        meadows_anchor: Vec3::new(-4.2, 5.6, -4.6),
        breakwater_anchor: Vec3::new(-4.2, 2.4, -1.9),
        flying_waters_anchor: Vec3::new(0.0, 0.0, 4.2),
        palette_anchor: Vec3::new(6.6, 0.4, 5.8),
        hero_camera_anchor: origen,
        broken_edge_anchor: Vec3::new(0.0, 1.2, 6.6),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn el_presupuesto_seguro_suma_160() {
        assert_eq!(SAFE.total(), 160);
    }

    #[test]
    fn el_preset_opaco_conserva_las_160_primitivas() {
        let nivel = safe_level(WaterPreset::OpaqueWater);

        assert_eq!(nivel.scene.objects.len(), 160);
    }

    #[test]
    fn el_preset_de_interior_visible_deja_159() {
        let nivel = safe_level(WaterPreset::InteriorVisible);

        assert_eq!(nivel.scene.objects.len(), 159);
    }

    #[test]
    fn la_unica_diferencia_entre_presets_es_el_volumen_de_agua() {
        let opaco = safe_level(WaterPreset::OpaqueWater);
        let visible = safe_level(WaterPreset::InteriorVisible);

        assert_eq!(opaco.scene.objects.len(), visible.scene.objects.len() + 1);

        // Y todo lo demas es identico, en el mismo orden: el volumen se
        // inserta al final justo para que quitarlo no desplace nada.
        for (a, b) in visible.scene.objects.iter().zip(&opaco.scene.objects) {
            assert_eq!(a.primitive.bounds(), b.primitive.bounds());
            assert_eq!(a.spatial_group, b.spatial_group);
        }
    }

    #[test]
    fn el_preset_opaco_oculta_44_primitivas_del_interior() {
        // La razon por la que el inventario prohibe medir rendimiento con
        // agua opaca: 44 primitivas dejan de probarse.
        assert_eq!(flying_waters::PRIMITIVAS_INTERIORES, 44);
    }

    #[test]
    fn cada_region_respeta_su_presupuesto() {
        let nivel = safe_level(WaterPreset::OpaqueWater);
        let cuenta = |grupo| {
            nivel
                .scene
                .objects
                .iter()
                .filter(|o| o.spatial_group == grupo)
                .count()
        };

        use crate::scene::SpatialGroupId as G;
        let global = cuenta(G::Global)
            + cuenta(G::ContinentBackground)
            + cuenta(G::Monolith)
            + cuenta(G::InteractionProps);

        assert_eq!(global, SAFE.global, "Global");
        assert_eq!(cuenta(G::Meadows), SAFE.meadows, "Praderas");
        assert_eq!(cuenta(G::Breakwater), SAFE.breakwater, "Rompeolas");
        assert_eq!(cuenta(G::FlyingWaters), SAFE.flying_waters, "Aguas");
    }

    #[test]
    fn rompeolas_conserva_sus_cuatro_clusters_en_el_nivel_seguro() {
        let nivel = safe_level(WaterPreset::InteriorVisible);

        let grupo = nivel
            .accel
            .groups
            .iter()
            .find(|g| g.id == crate::scene::SpatialGroupId::Breakwater)
            .expect("existe Rompeolas");

        // Los 28 pilares en cuatro tramos, mas el sendero y las masas de
        // soporte que caen en el cluster por defecto.
        assert!(
            grupo.clusters.len() >= 4,
            "los cuatro tramos del arco deben sobrevivir al ensamblado: {}",
            grupo.clusters.len()
        );
    }

    #[test]
    fn el_xorshift_no_se_queda_clavado_en_cero() {
        let mut generador = Xorshift32::new(0);
        let valores: Vec<f32> = (0..8).map(|_| generador.siguiente()).collect();

        assert!(valores.iter().all(|v| (0.0..1.0).contains(v)));
        assert!(valores.windows(2).any(|par| par[0] != par[1]));
    }
}
