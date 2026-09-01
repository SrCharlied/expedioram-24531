//! Estructura de aceleración: `escena → grupo → cluster → primitiva`.
//!
//! Sin ella el renderer prueba todas las primitivas contra todos los rayos.
//! A `800 × 600` con las 160 del nivel seguro eso son 76,8 millones de
//! pruebas por cuadro solo de rayos primarios, y con dos luces que
//! proyectan sombra la cota sube a 230 millones. La aceleración no es una
//! optimización opcional del proyecto: es un requisito.
//!
//! La jerarquía es **estática**. Se construye una vez, después de generar
//! la escena, y no se invalida nunca: pintar solo cambia un escalar en
//! `RevealState`, y la geometría no se mueve. Esa es la razón de que
//! `SceneObject` sea inmutable.
//!
//! Este módulo cubre la construcción y el recorrido con poda por bounds.
//! El orden por `t_enter` y la poda con el impacto más cercano llegan en la
//! Tarea 3.2.

use crate::bounds::Aabb;
use crate::hit::Hit;
use crate::ray::Ray;
use crate::ray_intersect::RayIntersect;
use crate::scene::{Scene, SceneObject, SpatialGroupId};
use crate::EPSILON;

/// Contadores del recorrido.
///
/// Existen para que las mediciones del Hito 3 sean comprobables y no
/// impresiones: dicen cuántas pruebas se evitaron, no solo cuánto tardó.
/// También son lo que permite escribir un test que demuestre que un grupo
/// fallado no llega a tocar sus primitivas.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TraversalStats {
    pub group_bounds_tests: usize,
    pub cluster_bounds_tests: usize,
    pub primitive_tests: usize,
}

/// Conjunto compacto de primitivas con su propia caja.
///
/// Una entrada hero produce normalmente un cluster. Un generador puede
/// producir varios cuando su distribución es larga, curva o dispersa: la
/// formación de pilares de Rompeolas se parte en cuatro tramos contiguos
/// del arco, porque un AABB único para todo el arco estaría lleno de aire.
#[derive(Debug, Clone)]
pub struct SpatialCluster {
    pub id: u16,
    pub bounds: Aabb,
    pub object_indices: Vec<usize>,
}

/// Una región de la escena.
#[derive(Debug, Clone)]
pub struct SpatialGroup {
    pub id: SpatialGroupId,
    pub bounds: Aabb,
    pub clusters: Vec<SpatialCluster>,
}

/// Raíz de la jerarquía.
#[derive(Debug, Clone)]
pub struct SceneAccel {
    pub bounds: Aabb,
    pub groups: Vec<SpatialGroup>,
}

impl SceneAccel {
    /// Construye la jerarquía con un cluster por grupo.
    ///
    /// Es el caso por defecto y el correcto para las entradas hero. Los
    /// generadores que necesiten partirse usan `build_with`.
    pub fn build(scene: &Scene) -> Option<SceneAccel> {
        SceneAccel::build_with(scene, |_, _| 0)
    }

    /// Construye la jerarquía dejando que quien llama decida en qué
    /// sub-cluster cae cada objeto dentro de su grupo espacial.
    ///
    /// La partición la declara el generador, no se deduce de la geometría:
    /// el inventario fija cuántos clusters produce cada entrada, y deducirlo
    /// automáticamente haría que el árbol dependiera de detalles de la
    /// distribución en vez del contrato.
    ///
    /// Los bounds se calculan de abajo hacia arriba, después de que toda la
    /// geometría existe. Devuelve `None` si la escena está vacía: una
    /// jerarquía sin caja envolvente no tiene nada que podar.
    pub fn build_with<F>(scene: &Scene, cluster_of: F) -> Option<SceneAccel>
    where
        F: Fn(usize, &SceneObject) -> u16,
    {
        let mut groups = Vec::new();

        // Orden fijo sobre los siete grupos conocidos: el árbol tiene que
        // salir idéntico en cada corrida para que las mediciones sean
        // comparables entre sí.
        for id in SpatialGroupId::ALL {
            let mut clusters: Vec<SpatialCluster> = Vec::new();

            for (index, object) in scene.objects.iter().enumerate() {
                if object.spatial_group != id {
                    continue;
                }

                let clave = cluster_of(index, object);
                let caja = object.primitive.bounds();

                match clusters.iter_mut().find(|c| c.id == clave) {
                    Some(cluster) => {
                        cluster.bounds = cluster.bounds.union(&caja);
                        cluster.object_indices.push(index);
                    }
                    None => clusters.push(SpatialCluster {
                        id: clave,
                        bounds: caja,
                        object_indices: vec![index],
                    }),
                }
            }

            if clusters.is_empty() {
                continue;
            }

            clusters.sort_by_key(|cluster| cluster.id);

            let bounds = clusters
                .iter()
                .map(|cluster| cluster.bounds)
                .reduce(|acumulado, caja| acumulado.union(&caja))
                .expect("el grupo tiene al menos un cluster");

            groups.push(SpatialGroup {
                id,
                bounds,
                clusters,
            });
        }

        let bounds = groups
            .iter()
            .map(|grupo| grupo.bounds)
            .reduce(|acumulado, caja| acumulado.union(&caja))?;

        Some(SceneAccel { bounds, groups })
    }

    /// Cantidad total de primitivas referenciadas por la jerarquía.
    pub fn primitive_count(&self) -> usize {
        self.groups
            .iter()
            .flat_map(|grupo| &grupo.clusters)
            .map(|cluster| cluster.object_indices.len())
            .sum()
    }

    /// Impacto más cercano, probando bounds antes que primitivas.
    ///
    /// Todavía sin ordenar por `t_enter`: recorre los candidatos en el orden
    /// del árbol y solo descarta los que el rayo ni siquiera alcanza. El
    /// orden y la poda por distancia llegan en la Tarea 3.2.
    pub fn intersect(&self, scene: &Scene, ray: &Ray, stats: &mut TraversalStats) -> Option<Hit> {
        let lejos = f32::INFINITY;

        // Si el rayo no toca la envolvente de la escena, no hay nada que
        // recorrer: es la primera y mas barata de las podas.
        self.bounds.hit(ray, EPSILON, lejos)?;

        let mut closest: Option<Hit> = None;

        for grupo in &self.groups {
            stats.group_bounds_tests += 1;
            if grupo.bounds.hit(ray, EPSILON, lejos).is_none() {
                continue;
            }

            for cluster in &grupo.clusters {
                stats.cluster_bounds_tests += 1;
                if cluster.bounds.hit(ray, EPSILON, lejos).is_none() {
                    continue;
                }

                for &index in &cluster.object_indices {
                    stats.primitive_tests += 1;

                    if let Some(mut hit) = scene.objects[index].primitive.ray_intersect(ray) {
                        if closest.is_none_or(|previo| hit.distance < previo.distance) {
                            hit.object_index = index;
                            closest = Some(hit);
                        }
                    }
                }
            }
        }

        closest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::cuboid::Cuboid;
    use crate::ray_intersect::Material;
    use crate::scene::{RevealGroup, SceneObject};
    use nalgebra_glm::Vec3;

    fn escena() -> Scene {
        let mut scene = Scene::new();
        let material = scene.add_material(Material::new(Color::new(0.5, 0.5, 0.5)));

        let piezas = [
            (Vec3::new(0.0, 0.0, 0.0), SpatialGroupId::Monolith),
            (Vec3::new(0.5, 2.0, 0.0), SpatialGroupId::Monolith),
            (Vec3::new(-20.0, 0.0, 0.0), SpatialGroupId::Meadows),
            (Vec3::new(-22.0, 1.0, 1.0), SpatialGroupId::Meadows),
            (Vec3::new(20.0, 0.0, 0.0), SpatialGroupId::Breakwater),
        ];

        for (centro, grupo) in piezas {
            scene.add_object(SceneObject {
                primitive: Cuboid::centrado(centro, Vec3::new(1.0, 1.0, 1.0)).into(),
                initial_material: material,
                final_material: material,
                spatial_group: grupo,
                reveal_group: RevealGroup::Finale,
            });
        }

        scene
    }

    #[test]
    fn los_bounds_del_cluster_contienen_sus_primitivas() {
        let scene = escena();
        let accel = SceneAccel::build(&scene).expect("hay geometria");

        for grupo in &accel.groups {
            for cluster in &grupo.clusters {
                for &index in &cluster.object_indices {
                    let caja = scene.objects[index].primitive.bounds();

                    assert!(
                        cluster.bounds.contiene(&caja.min) && cluster.bounds.contiene(&caja.max),
                        "el objeto {index} se sale de su cluster"
                    );
                }
            }
        }
    }

    #[test]
    fn los_bounds_del_grupo_contienen_sus_clusters() {
        let accel = SceneAccel::build(&escena()).expect("hay geometria");

        for grupo in &accel.groups {
            for cluster in &grupo.clusters {
                assert!(
                    grupo.bounds.contiene(&cluster.bounds.min)
                        && grupo.bounds.contiene(&cluster.bounds.max),
                    "el cluster {} se sale de su grupo {:?}",
                    cluster.id,
                    grupo.id
                );
            }
        }
    }

    #[test]
    fn los_bounds_de_la_escena_contienen_sus_grupos() {
        let accel = SceneAccel::build(&escena()).expect("hay geometria");

        for grupo in &accel.groups {
            assert!(
                accel.bounds.contiene(&grupo.bounds.min)
                    && accel.bounds.contiene(&grupo.bounds.max),
                "el grupo {:?} se sale de la escena",
                grupo.id
            );
        }
    }

    #[test]
    fn no_se_pierde_ninguna_primitiva() {
        let scene = escena();
        let accel = SceneAccel::build(&scene).expect("hay geometria");

        assert_eq!(accel.primitive_count(), scene.objects.len());

        let mut vistos: Vec<usize> = accel
            .groups
            .iter()
            .flat_map(|g| &g.clusters)
            .flat_map(|c| &c.object_indices)
            .copied()
            .collect();
        vistos.sort_unstable();

        assert_eq!(vistos, (0..scene.objects.len()).collect::<Vec<_>>());
    }

    #[test]
    fn cambiar_materiales_no_modifica_los_bounds() {
        // La revelación solo interpola materiales; la geometría es estática.
        // Si esto fallara, la jerarquía habría que reconstruirla al pintar y
        // toda la arquitectura del proyecto se caeria.
        let mut scene = escena();
        let antes = SceneAccel::build(&scene).expect("hay geometria");

        let nuevo = scene.add_material(Material::new(Color::new(1.0, 0.0, 0.0)));
        for objeto in &mut scene.objects {
            objeto.final_material = nuevo;
        }

        let despues = SceneAccel::build(&scene).expect("hay geometria");

        assert_eq!(antes.bounds, despues.bounds);
        for (a, d) in antes.groups.iter().zip(&despues.groups) {
            assert_eq!(a.bounds, d.bounds);
            for (ca, cd) in a.clusters.iter().zip(&d.clusters) {
                assert_eq!(ca.bounds, cd.bounds);
            }
        }
    }

    #[test]
    fn un_grupo_fallado_no_llega_a_probar_sus_primitivas() {
        let scene = escena();
        let accel = SceneAccel::build(&scene).expect("hay geometria");

        // Rayo dirigido solo al Monolito, en el origen. Los grupos de
        // Praderas y Rompeolas quedan a 20 unidades a los lados.
        let ray = Ray::new(Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 0.0, -1.0));

        let mut stats = TraversalStats::default();
        let hit = accel
            .intersect(&scene, &ray, &mut stats)
            .expect("debe impactar");

        assert_eq!(hit.object_index, 0);
        // Solo las dos primitivas del Monolito debieron probarse; las tres
        // de los grupos laterales quedaron descartadas por sus bounds.
        assert_eq!(stats.primitive_tests, 2, "{stats:?}");
        assert_eq!(stats.group_bounds_tests, 3, "se visitaron los tres grupos");
    }

    #[test]
    fn el_resultado_coincide_con_el_recorrido_lineal() {
        let scene = escena();
        let accel = SceneAccel::build(&scene).expect("hay geometria");

        let rayos = [
            Ray::new(Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 0.0, -1.0)),
            Ray::new(Vec3::new(-20.0, 0.0, 10.0), Vec3::new(0.0, 0.0, -1.0)),
            Ray::new(Vec3::new(20.0, 0.0, 10.0), Vec3::new(0.0, 0.0, -1.0)),
            Ray::new(Vec3::new(0.0, 50.0, 0.0), Vec3::new(0.0, -1.0, 0.0)),
            Ray::new(Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 1.0, 0.0)),
        ];

        for (i, ray) in rayos.iter().enumerate() {
            let mut stats = TraversalStats::default();
            let acelerado = accel.intersect(&scene, ray, &mut stats);
            let lineal = scene.intersect(ray);

            match (acelerado, lineal) {
                (None, None) => {}
                (Some(a), Some(l)) => {
                    assert_eq!(a.object_index, l.object_index, "rayo {i}");
                    assert!((a.distance - l.distance).abs() < 1e-5, "rayo {i}");
                }
                (a, l) => panic!(
                    "rayo {i}: acelerado {:?}, lineal {:?}",
                    a.is_some(),
                    l.is_some()
                ),
            }
        }
    }

    #[test]
    fn build_with_reparte_un_grupo_en_varios_clusters() {
        let scene = escena();

        // Los dos objetos del Monolito van a clusters distintos.
        let accel = SceneAccel::build_with(&scene, |index, _| if index == 1 { 1 } else { 0 })
            .expect("hay geometria");

        let monolito = accel
            .groups
            .iter()
            .find(|g| g.id == SpatialGroupId::Monolith)
            .expect("existe el grupo");

        assert_eq!(monolito.clusters.len(), 2);
        assert_eq!(monolito.clusters[0].id, 0);
        assert_eq!(monolito.clusters[1].id, 1);

        // Y el grupo sigue conteniendo a los dos.
        for cluster in &monolito.clusters {
            assert!(monolito.bounds.contiene(&cluster.bounds.min));
            assert!(monolito.bounds.contiene(&cluster.bounds.max));
        }
    }

    #[test]
    fn una_escena_vacia_no_produce_jerarquia() {
        assert!(SceneAccel::build(&Scene::new()).is_none());
    }
}
