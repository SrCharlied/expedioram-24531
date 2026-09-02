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
//! El recorrido ordena los candidatos por `t_enter` y corta en cuanto el
//! impacto más cercano conocido queda por delante del siguiente candidato.
//! Sin ese orden la poda apenas sirve: probar primero un grupo lejano
//! obliga a probar igual todos los demás.

use crate::bounds::Aabb;
use crate::hit::Hit;
use crate::light::GroupMask;
use crate::ray::Ray;
use crate::ray_intersect::RayIntersect;
use crate::scene::{Scene, SceneObject, SpatialGroupId};
use crate::EPSILON;

/// Tope de clusters por grupo.
///
/// Las listas de candidatos del recorrido viven en la pila, sin asignar
/// memoria: hay medio millón de rayos por cuadro y reservar dos vectores en
/// cada uno costaría más que la poda que se ahorra. `build_with` rechaza
/// una jerarquía que lo exceda en vez de degradarse en silencio. El máximo
/// que pide el inventario son los cuatro tramos de Rompeolas.
pub const MAX_CLUSTERS_PER_GROUP: usize = 16;

/// Tope de grupos. Son los siete de `SpatialGroupId::ALL`.
const MAX_GROUPS: usize = SpatialGroupId::ALL.len();

/// Ordena candidatos por `t_enter` ascendente, in situ.
///
/// Ordenamiento por inserción a propósito: los arreglos son de a lo sumo
/// dieciséis elementos y casi siempre de uno o dos, y ahí la inserción gana
/// a cualquier algoritmo con mejor complejidad asintótica.
fn ordenar_por_t_enter(candidatos: &mut [(f32, usize)]) {
    for i in 1..candidatos.len() {
        let mut j = i;
        while j > 0 && candidatos[j - 1].0 > candidatos[j].0 {
            candidatos.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Partición en clusters declarada por los generadores.
///
/// La jerarquía no adivina cómo repartir un generador: el inventario fija
/// cuántos clusters produce cada entrada y por qué criterio. Este plan es
/// el canal por el que un generador se lo comunica a `build_with`.
///
/// Los objetos sin asignación explícita caen en el cluster `0`, que es el
/// caso correcto para las entradas hero compactas.
#[derive(Debug, Clone, Default)]
pub struct ClusterPlan {
    asignaciones: Vec<u16>,
}

impl ClusterPlan {
    pub fn new() -> Self {
        ClusterPlan::default()
    }

    /// Asigna un objeto a un cluster dentro de su grupo espacial.
    pub fn asignar(&mut self, object_index: usize, cluster: u16) {
        if self.asignaciones.len() <= object_index {
            self.asignaciones.resize(object_index + 1, 0);
        }

        self.asignaciones[object_index] = cluster;
    }

    pub fn cluster_of(&self, object_index: usize) -> u16 {
        self.asignaciones.get(object_index).copied().unwrap_or(0)
    }
}

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

    /// Construye la jerarquía siguiendo la partición que declararon los
    /// generadores.
    pub fn build_from_plan(scene: &Scene, plan: &ClusterPlan) -> Option<SceneAccel> {
        SceneAccel::build_with(scene, |index, _| plan.cluster_of(index))
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
    /// geometría existe.
    ///
    /// Devuelve `None` si la escena está vacía —una jerarquía sin caja
    /// envolvente no tiene nada que podar— o si algún grupo supera
    /// `MAX_CLUSTERS_PER_GROUP`.
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

            if clusters.len() > MAX_CLUSTERS_PER_GROUP {
                return None;
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

    /// Impacto más cercano contra la escena.
    ///
    /// El recorrido sigue el orden que fija el inventario:
    ///
    /// 1. Probar la envolvente de la escena.
    /// 2. Calcular `t_enter` de cada grupo alcanzado.
    /// 3. Ordenarlos por `t_enter` ascendente.
    /// 4. Cortar cuando el impacto más cercano conocido esté por delante
    ///    del `t_enter` del siguiente grupo.
    /// 5. Repetir el orden y la poda con los clusters de cada grupo.
    /// 6. Solo entonces probar primitivas.
    ///
    /// El paso 4 es el que hace útil al 3: sin ordenar, un grupo lejano
    /// visitado primero deja el impacto conocido tan atrás que ya no poda
    /// nada, y el recorrido degenera en probarlos todos.
    pub fn intersect(&self, scene: &Scene, ray: &Ray, stats: &mut TraversalStats) -> Option<Hit> {
        let lejos = f32::INFINITY;

        // Si el rayo no toca la envolvente de la escena, no hay nada que
        // recorrer: es la primera y mas barata de las podas.
        self.bounds.hit(ray, EPSILON, lejos)?;

        let mut candidatos = [(0.0_f32, 0_usize); MAX_GROUPS];
        let mut cuantos = 0;

        for (indice, grupo) in self.groups.iter().enumerate() {
            stats.group_bounds_tests += 1;

            if let Some(intervalo) = grupo.bounds.hit(ray, EPSILON, lejos) {
                candidatos[cuantos] = (intervalo.t_enter, indice);
                cuantos += 1;
            }
        }

        ordenar_por_t_enter(&mut candidatos[..cuantos]);

        let mut closest: Option<Hit> = None;

        for &(t_enter, indice) in &candidatos[..cuantos] {
            // Todo lo que queda empieza más lejos de lo que ya se impactó.
            if closest.is_some_and(|previo| previo.distance <= t_enter) {
                break;
            }

            self.recorrer_grupo(scene, ray, &self.groups[indice], &mut closest, stats);
        }

        closest
    }

    fn recorrer_grupo(
        &self,
        scene: &Scene,
        ray: &Ray,
        grupo: &SpatialGroup,
        closest: &mut Option<Hit>,
        stats: &mut TraversalStats,
    ) {
        let lejos = f32::INFINITY;

        let mut candidatos = [(0.0_f32, 0_usize); MAX_CLUSTERS_PER_GROUP];
        let mut cuantos = 0;

        for (indice, cluster) in grupo.clusters.iter().enumerate() {
            stats.cluster_bounds_tests += 1;

            if let Some(intervalo) = cluster.bounds.hit(ray, EPSILON, lejos) {
                candidatos[cuantos] = (intervalo.t_enter, indice);
                cuantos += 1;
            }
        }

        ordenar_por_t_enter(&mut candidatos[..cuantos]);

        for &(t_enter, indice) in &candidatos[..cuantos] {
            if closest.is_some_and(|previo| previo.distance <= t_enter) {
                break;
            }

            for &objeto in &grupo.clusters[indice].object_indices {
                stats.primitive_tests += 1;

                if let Some(mut hit) = scene.objects[objeto].primitive.ray_intersect(ray) {
                    if closest.is_none_or(|previo| hit.distance < previo.distance) {
                        hit.object_index = objeto;
                        *closest = Some(hit);
                    }
                }
            }
        }
    }

    /// ¿Hay algo entre el origen del rayo y `t_max`?
    ///
    /// Es la consulta de los rayos de sombra, y es mucho más barata que
    /// `intersect`: no necesita el impacto más cercano, solo saber si existe
    /// alguno, así que **termina en el primero que encuentra**. Tampoco
    /// ordena: para responder «sí» cualquier bloqueador sirve igual.
    ///
    /// `t_max` es la distancia a la luz menos un epsilon. Sin ese tope, un
    /// objeto situado *detrás* de la luz bloquearía una sombra que no le
    /// corresponde.
    ///
    /// `occluder_groups` es el *light linking* de las sombras. Se comprueba
    /// **antes** que los bounds del grupo, así que un grupo excluido no
    /// cuesta ni una prueba de caja: si `L-02` solo puede ser bloqueada por
    /// Aguas Voladoras, el rayo de sombra ni mira Praderas.
    ///
    /// Solo `ShadowMode::Opaque` cuenta como bloqueador. El agua deja pasar
    /// el rayo; sin eso el barco quedaría negro bajo su propio volumen.
    pub fn occluded(
        &self,
        scene: &Scene,
        ray: &Ray,
        t_max: f32,
        occluder_groups: GroupMask,
        stats: &mut TraversalStats,
    ) -> bool {
        if self.bounds.hit(ray, EPSILON, t_max).is_none() {
            return false;
        }

        for grupo in &self.groups {
            // Filtro de grupo primero: es el mas barato de todos.
            if !occluder_groups.contains(grupo.id) {
                continue;
            }

            stats.group_bounds_tests += 1;
            if grupo.bounds.hit(ray, EPSILON, t_max).is_none() {
                continue;
            }

            for cluster in &grupo.clusters {
                stats.cluster_bounds_tests += 1;
                if cluster.bounds.hit(ray, EPSILON, t_max).is_none() {
                    continue;
                }

                for &objeto in &cluster.object_indices {
                    let entrada = scene.objects[objeto];

                    // El modo del material FINAL, no del inicial: la
                    // revelación no interpola `shadow_mode`.
                    if !scene.material(entrada.final_material).blocks_shadows() {
                        continue;
                    }

                    stats.primitive_tests += 1;

                    if let Some(hit) = entrada.primitive.ray_intersect(ray) {
                        if hit.distance < t_max {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::cuboid::Cuboid;
    use crate::material::Material;
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

    /// Recorrido lineal independiente, solo para los tests.
    ///
    /// No usa `Scene::intersect` a propósito: el oráculo contra el que se
    /// compara la jerarquía tiene que ser código que la jerarquía no toca.
    fn fuerza_bruta(scene: &Scene, ray: &Ray) -> Option<Hit> {
        let mut closest: Option<Hit> = None;

        for (index, object) in scene.objects.iter().enumerate() {
            if let Some(mut hit) = object.primitive.ray_intersect(ray) {
                if closest.is_none_or(|previo| hit.distance < previo.distance) {
                    hit.object_index = index;
                    closest = Some(hit);
                }
            }
        }

        closest
    }

    /// Dos objetos alineados con el eje -Z, en grupos elegidos para que el
    /// **lejano** aparezca antes en el orden de `SpatialGroupId::ALL`.
    ///
    /// Es la escena que distingue un recorrido ordenado de uno secuencial:
    /// sin ordenar, el grupo lejano se visita primero y el cercano no puede
    /// podarlo.
    fn escena_lejano_primero() -> Scene {
        let mut scene = Scene::new();
        let material = scene.add_material(Material::new(Color::new(0.5, 0.5, 0.5)));

        // Global es el indice 0 de ALL; Monolith el 5.
        let piezas = [
            (Vec3::new(0.0, 0.0, -40.0), SpatialGroupId::Global),
            (Vec3::new(0.0, 0.0, 0.0), SpatialGroupId::Monolith),
        ];

        for (centro, grupo) in piezas {
            scene.add_object(SceneObject {
                primitive: Cuboid::centrado(centro, Vec3::new(2.0, 2.0, 2.0)).into(),
                initial_material: material,
                final_material: material,
                spatial_group: grupo,
                reveal_group: RevealGroup::Finale,
            });
        }

        scene
    }

    #[test]
    fn el_grupo_cercano_se_visita_antes_que_el_lejano() {
        let scene = escena_lejano_primero();
        let accel = SceneAccel::build(&scene).expect("hay geometria");

        let ray = Ray::new(Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 0.0, -1.0));
        let mut stats = TraversalStats::default();

        let hit = accel
            .intersect(&scene, &ray, &mut stats)
            .expect("debe impactar");

        // Gana el cercano, que es el objeto 1 aunque su grupo vaya despues.
        assert_eq!(hit.object_index, 1);
        assert!((hit.distance - 9.0).abs() < 1e-5, "{}", hit.distance);
    }

    #[test]
    fn closest_t_menor_que_el_siguiente_t_enter_poda_el_grupo_lejano() {
        let scene = escena_lejano_primero();
        let accel = SceneAccel::build(&scene).expect("hay geometria");

        let ray = Ray::new(Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 0.0, -1.0));
        let mut stats = TraversalStats::default();

        accel.intersect(&scene, &ray, &mut stats).expect("impacta");

        // Los dos grupos son candidatos: el rayo atraviesa ambas cajas.
        assert_eq!(stats.group_bounds_tests, 2);
        // Pero solo se prueba la primitiva del cercano. Sin orden se
        // probarian las dos.
        assert_eq!(
            stats.primitive_tests, 1,
            "el grupo lejano no debio probarse: {stats:?}"
        );
    }

    #[test]
    fn sin_impacto_no_se_poda_nada() {
        // Control del test anterior: si el rayo pasa de largo por el
        // cercano, el lejano si tiene que probarse.
        let scene = escena_lejano_primero();
        let accel = SceneAccel::build(&scene).expect("hay geometria");

        // Desplazado en Y para fallar el cubo cercano pero no el lejano,
        // que es del mismo tamano: se apunta ligeramente hacia abajo.
        let ray = Ray::new(
            Vec3::new(0.0, 3.0, 10.0),
            Vec3::new(0.0, -0.06, -1.0).normalize(),
        );
        let mut stats = TraversalStats::default();

        let acelerado = accel.intersect(&scene, &ray, &mut stats);

        assert_eq!(
            acelerado.map(|h| h.object_index),
            fuerza_bruta(&scene, &ray).map(|h| h.object_index)
        );
        assert!(stats.primitive_tests >= 1, "algo debio probarse: {stats:?}");
    }

    #[test]
    fn el_resultado_coincide_con_fuerza_bruta_en_un_barrido() {
        let scene = escena();
        let accel = SceneAccel::build(&scene).expect("hay geometria");

        // Barrido determinista de direcciones desde varios origenes.
        let mut comprobados = 0;
        for ox in [-25.0_f32, -10.0, 0.0, 10.0, 25.0] {
            for oy in [-5.0_f32, 0.0, 5.0] {
                for dx in [-0.6_f32, -0.2, 0.0, 0.2, 0.6] {
                    for dy in [-0.4_f32, 0.0, 0.4] {
                        let ray =
                            Ray::new(Vec3::new(ox, oy, 30.0), Vec3::new(dx, dy, -1.0).normalize());

                        let mut stats = TraversalStats::default();
                        let a = accel.intersect(&scene, &ray, &mut stats);
                        let b = fuerza_bruta(&scene, &ray);

                        match (a, b) {
                            (None, None) => {}
                            (Some(a), Some(b)) => {
                                assert_eq!(
                                    a.object_index, b.object_index,
                                    "origen ({ox},{oy}) dir ({dx},{dy})"
                                );
                                assert!((a.distance - b.distance).abs() < 1e-4);
                                assert_eq!(a.normal, b.normal);
                            }
                            (a, b) => panic!(
                                "discrepancia en ({ox},{oy}) dir ({dx},{dy}): {} contra {}",
                                a.is_some(),
                                b.is_some()
                            ),
                        }

                        comprobados += 1;
                    }
                }
            }
        }

        assert_eq!(comprobados, 225);
    }

    #[test]
    fn el_any_hit_termina_en_el_primer_bloqueador() {
        let scene = escena();
        let accel = SceneAccel::build(&scene).expect("hay geometria");

        let ray = Ray::new(Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 0.0, -1.0));
        let mut stats = TraversalStats::default();

        assert!(accel.occluded(&scene, &ray, 100.0, GroupMask::ALL, &mut stats));
        // Corta en cuanto encuentra uno; no recorre las cinco primitivas.
        assert!(
            stats.primitive_tests < scene.objects.len(),
            "recorrio de mas: {stats:?}"
        );
    }

    #[test]
    fn un_bloqueador_detras_de_la_luz_no_cuenta() {
        let scene = escena();
        let accel = SceneAccel::build(&scene).expect("hay geometria");

        // El cubo del origen esta a distancia 9.5 del origen del rayo.
        let ray = Ray::new(Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 0.0, -1.0));
        let mut stats = TraversalStats::default();

        assert!(
            !accel.occluded(&scene, &ray, 5.0, GroupMask::ALL, &mut stats),
            "un objeto mas alla de t_max no debe bloquear"
        );
        assert!(accel.occluded(
            &scene,
            &ray,
            20.0,
            GroupMask::ALL,
            &mut TraversalStats::default()
        ));
    }

    #[test]
    fn el_any_hit_coincide_con_fuerza_bruta() {
        let scene = escena();
        let accel = SceneAccel::build(&scene).expect("hay geometria");

        for ox in [-22.0_f32, -20.0, 0.0, 20.0, 8.0] {
            for t_max in [1.0_f32, 5.0, 9.6, 50.0] {
                let ray = Ray::new(Vec3::new(ox, 0.0, 10.0), Vec3::new(0.0, 0.0, -1.0));

                let esperado = fuerza_bruta(&scene, &ray).is_some_and(|h| h.distance < t_max);
                let obtenido = accel.occluded(
                    &scene,
                    &ray,
                    t_max,
                    GroupMask::ALL,
                    &mut TraversalStats::default(),
                );

                assert_eq!(obtenido, esperado, "origen x={ox}, t_max={t_max}");
            }
        }
    }

    #[test]
    fn el_agua_con_ignore_no_bloquea_pero_el_monolito_opaco_si() {
        use crate::material::ShadowMode;

        let mut scene = Scene::new();
        let agua = scene.add_material(
            Material::new(Color::new(0.2, 0.4, 0.8)).with_shadow_mode(ShadowMode::Ignore),
        );
        let cristal = scene.add_material(Material::new(Color::new(0.7, 0.9, 1.0)));

        // Agua cerca, Monolito lejos, ambos sobre el mismo rayo.
        for (centro, material, grupo) in [
            (Vec3::new(0.0, 0.0, 4.0), agua, SpatialGroupId::FlyingWaters),
            (Vec3::new(0.0, 0.0, 0.0), cristal, SpatialGroupId::Monolith),
        ] {
            scene.add_object(SceneObject {
                primitive: Cuboid::centrado(centro, Vec3::new(2.0, 2.0, 2.0)).into(),
                initial_material: material,
                final_material: material,
                spatial_group: grupo,
                reveal_group: RevealGroup::Finale,
            });
        }

        let accel = SceneAccel::build(&scene).expect("hay geometria");
        let ray = Ray::new(Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 0.0, -1.0));

        // Hasta antes del Monolito solo esta el agua: no bloquea.
        assert!(!accel.occluded(
            &scene,
            &ray,
            8.0,
            GroupMask::ALL,
            &mut TraversalStats::default()
        ));

        // Mas alla si aparece el Monolito, que es opaco.
        assert!(accel.occluded(
            &scene,
            &ray,
            20.0,
            GroupMask::ALL,
            &mut TraversalStats::default()
        ));
    }

    #[test]
    fn el_agua_ni_siquiera_se_prueba_como_bloqueador() {
        use crate::material::ShadowMode;

        let mut scene = Scene::new();
        let agua = scene.add_material(
            Material::new(Color::new(0.2, 0.4, 0.8)).with_shadow_mode(ShadowMode::Ignore),
        );

        scene.add_object(SceneObject {
            primitive: Cuboid::centrado(Vec3::zeros(), Vec3::new(2.0, 2.0, 2.0)).into(),
            initial_material: agua,
            final_material: agua,
            spatial_group: SpatialGroupId::FlyingWaters,
            reveal_group: RevealGroup::FlyingWaters,
        });

        let accel = SceneAccel::build(&scene).expect("hay geometria");
        let ray = Ray::new(Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 0.0, -1.0));
        let mut stats = TraversalStats::default();

        assert!(!accel.occluded(&scene, &ray, 50.0, GroupMask::ALL, &mut stats));
        assert_eq!(
            stats.primitive_tests, 0,
            "un material que no bloquea no deberia costar una prueba"
        );
    }

    #[test]
    fn el_modo_de_sombra_sale_del_material_final_no_del_inicial() {
        use crate::material::ShadowMode;

        // Decision cerrada: `shadow_mode` no se interpola durante la
        // revelacion. Un objeto cuyo material inicial es opaco pero cuyo
        // final ignora sombras, no bloquea desde el primer momento.
        let mut scene = Scene::new();
        let lienzo = scene.add_material(Material::new(Color::new(0.9, 0.9, 0.85)));
        let agua = scene.add_material(
            Material::new(Color::new(0.2, 0.4, 0.8)).with_shadow_mode(ShadowMode::Ignore),
        );

        scene.add_object(SceneObject {
            primitive: Cuboid::centrado(Vec3::zeros(), Vec3::new(2.0, 2.0, 2.0)).into(),
            initial_material: lienzo,
            final_material: agua,
            spatial_group: SpatialGroupId::FlyingWaters,
            reveal_group: RevealGroup::FlyingWaters,
        });

        let accel = SceneAccel::build(&scene).expect("hay geometria");
        let ray = Ray::new(Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 0.0, -1.0));

        assert!(!accel.occluded(
            &scene,
            &ray,
            50.0,
            GroupMask::ALL,
            &mut TraversalStats::default()
        ));
    }

    #[test]
    fn la_mascara_de_oclusores_descarta_grupos_sin_probar_sus_bounds() {
        let scene = escena();
        let accel = SceneAccel::build(&scene).expect("hay geometria");

        // Rayo que atraviesa el Monolito, en el origen.
        let ray = Ray::new(Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 0.0, -1.0));

        // Con la mascara limitada a Praderas, el Monolito no puede bloquear.
        let mut solo_praderas = TraversalStats::default();
        assert!(!accel.occluded(
            &scene,
            &ray,
            50.0,
            GroupMask::only(&[SpatialGroupId::Meadows]),
            &mut solo_praderas
        ));

        // Y el filtro actua antes de los bounds: solo se probo un grupo.
        assert_eq!(
            solo_praderas.group_bounds_tests, 1,
            "el filtro de grupo deberia ahorrarse hasta las pruebas de caja: {solo_praderas:?}"
        );

        // Con el Monolito habilitado, si bloquea.
        assert!(accel.occluded(
            &scene,
            &ray,
            50.0,
            GroupMask::only(&[SpatialGroupId::Monolith]),
            &mut TraversalStats::default()
        ));
    }

    #[test]
    fn la_ordenacion_por_insercion_ordena() {
        let mut candidatos = [(3.0, 30), (1.0, 10), (2.0, 20), (0.5, 5)];
        ordenar_por_t_enter(&mut candidatos);

        assert_eq!(candidatos, [(0.5, 5), (1.0, 10), (2.0, 20), (3.0, 30)]);

        // Estable e idempotente sobre una lista ya ordenada.
        ordenar_por_t_enter(&mut candidatos);
        assert_eq!(candidatos[0], (0.5, 5));

        // Y no se cae con listas vacias o de un elemento.
        ordenar_por_t_enter(&mut []);
        ordenar_por_t_enter(&mut [(1.0, 1)]);
    }

    #[test]
    fn se_rechaza_un_grupo_con_demasiados_clusters() {
        let mut scene = Scene::new();
        let material = scene.add_material(Material::new(Color::new(0.5, 0.5, 0.5)));

        for i in 0..(MAX_CLUSTERS_PER_GROUP + 1) {
            scene.add_object(SceneObject {
                primitive: Cuboid::centrado(
                    Vec3::new(i as f32 * 3.0, 0.0, 0.0),
                    Vec3::new(1.0, 1.0, 1.0),
                )
                .into(),
                initial_material: material,
                final_material: material,
                spatial_group: SpatialGroupId::Breakwater,
                reveal_group: RevealGroup::Breakwater,
            });
        }

        // Un cluster por objeto excede el tope y la construccion falla en
        // vez de degradarse en silencio.
        assert!(SceneAccel::build_with(&scene, |i, _| i as u16).is_none());
        // Con el tope justo, se acepta.
        assert!(
            SceneAccel::build_with(&scene, |i, _| (i % MAX_CLUSTERS_PER_GROUP) as u16).is_some()
        );
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
