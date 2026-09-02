//! Generador `R-01`: la formación de basalto del Acantilado Rompeolas.
//!
//! Es el objeto más numeroso del inventario —28 pilares en nivel seguro, 42
//! en objetivo— y el único que declara obligatoriamente **cuatro clusters**.
//! La razón es de aceleración, no de arte: los pilares siguen un arco, y un
//! AABB único que lo cubra entero está lleno de aire. Un rayo que roce ese
//! AABB pagaría las 28 pruebas de primitiva sin tocar nada. Partido en
//! cuatro tramos contiguos, cada caja se ciñe a su tramo y el rayo paga solo
//! el que atraviesa.
//!
//! Se implementa la **Ruta B** del inventario: cuboides verticales en filas
//! desfasadas. La Ruta A —prismas hexagonales reales— depende de una
//! autorización del profesor y llega, si llega, en la Tarea 7.3.

use crate::accel::ClusterPlan;
use crate::cuboid::Cuboid;
use crate::scene::{MaterialId, RevealGroup, Scene, SceneObject, SpatialGroupId};
use nalgebra_glm::Vec3;

/// Nivel de detalle de la formación.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetailLevel {
    Safe,
    Target,
}

impl DetailLevel {
    /// Pilares por cluster, según el inventario. La suma es el conteo total
    /// de la entrada: 28 en seguro, 42 en objetivo.
    pub fn pilares_por_cluster(self) -> [usize; CLUSTERS] {
        match self {
            DetailLevel::Safe => [7, 7, 7, 7],
            DetailLevel::Target => [10, 10, 11, 11],
        }
    }

    pub fn total(self) -> usize {
        self.pilares_por_cluster().iter().sum()
    }
}

/// Cuatro tramos contiguos del arco. Lo fija el inventario.
pub const CLUSTERS: usize = 4;

/// Generador pseudoaleatorio determinista, sin dependencias.
///
/// El inventario exige `seed: fixed` para toda entrada generada: el blockout
/// y los renders de evidencia tienen que salir idénticos en cada corrida, y
/// dos capturas que difieran por el azar no sirven para comparar nada.
/// `rand` traería esa garantía solo si se fijara la semilla igual, así que
/// no compensa la dependencia.
struct Xorshift32(u32);

impl Xorshift32 {
    fn new(semilla: u32) -> Self {
        // El cero es un punto fijo del xorshift: se quedaría clavado.
        Xorshift32(if semilla == 0 { 0x9E37_79B9 } else { semilla })
    }

    /// Siguiente valor en `0.0..1.0`.
    fn siguiente(&mut self) -> f32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;

        // Se usan los 24 bits altos: los bajos de un xorshift tienen peor
        // distribución.
        (x >> 8) as f32 / (1u32 << 24) as f32
    }

    /// Siguiente valor en `-1.0..1.0`.
    fn simetrico(&mut self) -> f32 {
        self.siguiente() * 2.0 - 1.0
    }
}

/// Forma del arco sobre el que se apoya la formación.
#[derive(Debug, Clone, Copy)]
pub struct Arco {
    /// Punto medio del arco, al nivel de la base de los pilares.
    pub ancla: Vec3,
    /// Radio de curvatura. Mayor radio, arco más plano.
    pub radio: f32,
    /// Apertura angular total, en radianes.
    pub apertura: f32,
    /// Lado del pilar.
    pub ancho: f32,
    /// Altura mínima.
    pub altura_base: f32,
    /// Cuánto puede crecer un pilar por encima de la mínima.
    pub variacion: f32,
    /// Desfase radial de las filas alternas. Es el truco de la Ruta B: dos
    /// filas desfasadas leen como un empaquetado hexagonal sin serlo.
    pub desfase: f32,
    pub semilla: u32,
}

impl Default for Arco {
    fn default() -> Self {
        Arco {
            ancla: Vec3::zeros(),
            radio: 9.0,
            apertura: 1.15,
            ancho: 0.62,
            altura_base: 2.2,
            variacion: 2.6,
            desfase: 0.42,
            semilla: 0x0A5C_1F03,
        }
    }
}

/// Resultado de generar la formación.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Formacion {
    /// Índice del primer pilar dentro de `Scene::objects`.
    pub primer_indice: usize,
    pub pilares: usize,
    pub por_cluster: [usize; CLUSTERS],
}

/// Genera la formación y declara su partición en el plan de clusters.
///
/// Los pilares se recorren en orden angular, así que el reparto por bloques
/// consecutivos produce **tramos contiguos del arco** y no cuatro conjuntos
/// entremezclados. Eso es lo que hace que cada AABB se ciña a su tramo.
pub fn generar(
    scene: &mut Scene,
    plan: &mut ClusterPlan,
    arco: &Arco,
    nivel: DetailLevel,
    material: MaterialId,
) -> Formacion {
    let por_cluster = nivel.pilares_por_cluster();
    let total = nivel.total();

    let primer_indice = scene.objects.len();
    let mut aleatorio = Xorshift32::new(arco.semilla);

    // Centro de curvatura, detrás del ancla: el arco queda cóncavo visto
    // desde el frente, como pide el inventario.
    let centro = arco.ancla + Vec3::new(0.0, 0.0, -arco.radio);

    for i in 0..total {
        // Reparto en el eje angular. Con un solo pilar el divisor seria
        // cero, de ahi el maximo.
        let t = i as f32 / (total.max(2) - 1) as f32;
        let angulo = (t - 0.5) * arco.apertura;

        // Filas desfasadas: los pares se adelantan, los impares se atrasan.
        let radio = arco.radio + if i % 2 == 0 { 0.0 } else { arco.desfase };

        let altura = arco.altura_base + arco.variacion * aleatorio.siguiente();
        // Jitter suave para que la hilera no se lea como una reja.
        let deriva = 0.06 * arco.ancho * aleatorio.simetrico();

        let centro_pilar = centro
            + Vec3::new(
                radio * angulo.sin() + deriva,
                altura * 0.5,
                radio * angulo.cos(),
            );

        scene.add_object(SceneObject {
            primitive: Cuboid::centrado(
                Vec3::new(centro_pilar.x, arco.ancla.y + altura * 0.5, centro_pilar.z),
                Vec3::new(arco.ancho, altura, arco.ancho),
            )
            .into(),
            initial_material: material,
            final_material: material,
            spatial_group: SpatialGroupId::Breakwater,
            reveal_group: RevealGroup::Breakwater,
        });

        plan.asignar(primer_indice + i, cluster_de(i, &por_cluster));
    }

    Formacion {
        primer_indice,
        pilares: total,
        por_cluster,
    }
}

/// Cluster al que pertenece el pilar `i`, por bloques consecutivos.
fn cluster_de(i: usize, por_cluster: &[usize; CLUSTERS]) -> u16 {
    let mut acumulado = 0;

    for (indice, cantidad) in por_cluster.iter().enumerate() {
        acumulado += cantidad;
        if i < acumulado {
            return indice as u16;
        }
    }

    (CLUSTERS - 1) as u16
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accel::SceneAccel;
    use crate::bounds::Aabb;
    use crate::color::Color;
    use crate::material::Material;

    fn generar_nivel(nivel: DetailLevel) -> (Scene, ClusterPlan, Formacion) {
        let mut scene = Scene::new();
        let mut plan = ClusterPlan::new();
        let material = scene.add_material(Material::new(Color::new(0.3, 0.3, 0.32)));

        let formacion = generar(
            &mut scene,
            &mut plan,
            &Arco {
                ancla: Vec3::new(0.0, 0.0, 0.0),
                ..Arco::default()
            },
            nivel,
            material,
        );

        (scene, plan, formacion)
    }

    fn volumen(caja: &Aabb) -> f32 {
        let d = caja.max - caja.min;
        d.x * d.y * d.z
    }

    #[test]
    fn el_nivel_seguro_produce_28_pilares_en_cuatro_clusters() {
        let (scene, _, formacion) = generar_nivel(DetailLevel::Safe);

        assert_eq!(formacion.pilares, 28);
        assert_eq!(formacion.por_cluster, [7, 7, 7, 7]);
        assert_eq!(scene.objects.len(), 28);
    }

    #[test]
    fn el_nivel_objetivo_produce_42_pilares_en_cuatro_clusters() {
        let (scene, _, formacion) = generar_nivel(DetailLevel::Target);

        assert_eq!(formacion.pilares, 42);
        assert_eq!(formacion.por_cluster, [10, 10, 11, 11]);
        assert_eq!(scene.objects.len(), 42);
    }

    #[test]
    fn la_jerarquia_recibe_exactamente_cuatro_clusters() {
        for nivel in [DetailLevel::Safe, DetailLevel::Target] {
            let (scene, plan, formacion) = generar_nivel(nivel);
            let accel = SceneAccel::build_from_plan(&scene, &plan).expect("hay geometria");

            let grupo = accel
                .groups
                .iter()
                .find(|g| g.id == SpatialGroupId::Breakwater)
                .expect("existe el grupo");

            assert_eq!(grupo.clusters.len(), CLUSTERS, "{nivel:?}");

            for (indice, cluster) in grupo.clusters.iter().enumerate() {
                assert_eq!(
                    cluster.object_indices.len(),
                    formacion.por_cluster[indice],
                    "{nivel:?} cluster {indice}"
                );
            }
        }
    }

    #[test]
    fn cada_cluster_cubre_un_tramo_contiguo_del_arco() {
        let (scene, plan, _) = generar_nivel(DetailLevel::Safe);
        let accel = SceneAccel::build_from_plan(&scene, &plan).expect("hay geometria");

        let grupo = accel
            .groups
            .iter()
            .find(|g| g.id == SpatialGroupId::Breakwater)
            .expect("existe el grupo");

        // Contiguo en indice: cada cluster es un bloque consecutivo.
        let mut esperado = 0;
        for cluster in &grupo.clusters {
            for &indice in &cluster.object_indices {
                assert_eq!(indice, esperado, "los indices no son consecutivos");
                esperado += 1;
            }
        }

        // Y contiguo en el arco: los tramos se suceden en X sin
        // entremezclarse. Basta con que cada cluster empiece donde el
        // anterior termina, con la tolerancia del ancho de un pilar.
        let mut anterior_max = f32::NEG_INFINITY;
        for (indice, cluster) in grupo.clusters.iter().enumerate() {
            assert!(
                cluster.bounds.min.x >= anterior_max - 1.5,
                "el cluster {indice} se solapa hacia atras con el anterior"
            );
            anterior_max = cluster.bounds.max.x;
        }
    }

    #[test]
    fn los_cuatro_aabb_son_mas_ajustados_que_uno_solo() {
        // Es la razon de ser de la particion: un AABB unico sobre un arco
        // esta lleno de aire, y un rayo que lo roce paga las 28 pruebas.
        let (scene, plan, _) = generar_nivel(DetailLevel::Safe);
        let accel = SceneAccel::build_from_plan(&scene, &plan).expect("hay geometria");

        let grupo = accel
            .groups
            .iter()
            .find(|g| g.id == SpatialGroupId::Breakwater)
            .expect("existe el grupo");

        let suma: f32 = grupo.clusters.iter().map(|c| volumen(&c.bounds)).sum();
        let entero = volumen(&grupo.bounds);

        assert!(
            suma < entero * 0.75,
            "los cuatro clusters suman {suma:.2} contra {entero:.2} del AABB unico: la particion no esta ganando nada"
        );
    }

    #[test]
    fn la_particion_ahorra_pruebas_de_primitiva() {
        use crate::accel::TraversalStats;
        use crate::ray::Ray;

        let (scene, plan, _) = generar_nivel(DetailLevel::Safe);
        let particionado = SceneAccel::build_from_plan(&scene, &plan).expect("hay geometria");
        let entero = SceneAccel::build(&scene).expect("hay geometria");

        // Rayo que roza un extremo del arco.
        let ray = Ray::new(Vec3::new(-4.2, 1.0, 8.0), Vec3::new(0.0, 0.0, -1.0));

        let mut con = TraversalStats::default();
        particionado.intersect(&scene, &ray, &mut con);

        let mut sin = TraversalStats::default();
        entero.intersect(&scene, &ray, &mut sin);

        assert!(
            con.primitive_tests < sin.primitive_tests,
            "particionado {} contra entero {}",
            con.primitive_tests,
            sin.primitive_tests
        );
    }

    #[test]
    fn la_semilla_fija_da_siempre_la_misma_formacion() {
        let (a, _, _) = generar_nivel(DetailLevel::Safe);
        let (b, _, _) = generar_nivel(DetailLevel::Safe);

        for (x, y) in a.objects.iter().zip(&b.objects) {
            assert_eq!(x.primitive.bounds(), y.primitive.bounds());
        }
    }

    #[test]
    fn una_semilla_distinta_da_otra_formacion() {
        let mut scene = Scene::new();
        let mut plan = ClusterPlan::new();
        let material = scene.add_material(Material::new(Color::new(0.3, 0.3, 0.32)));

        let arco = Arco {
            semilla: 0x1234_5678,
            ..Arco::default()
        };
        generar(&mut scene, &mut plan, &arco, DetailLevel::Safe, material);

        let (referencia, _, _) = generar_nivel(DetailLevel::Safe);

        let iguales = scene
            .objects
            .iter()
            .zip(&referencia.objects)
            .filter(|(a, b)| a.primitive.bounds() == b.primitive.bounds())
            .count();

        assert!(iguales < 28, "la semilla no esta influyendo");
    }

    #[test]
    fn los_pilares_varian_de_altura_y_son_verticales() {
        let (scene, _, _) = generar_nivel(DetailLevel::Safe);

        let mut alturas: Vec<f32> = Vec::new();
        for objeto in &scene.objects {
            let caja = objeto.primitive.bounds();
            let d = caja.max - caja.min;

            // Ruta B: cuboides verticales, mas altos que anchos.
            assert!(d.y > d.x && d.y > d.z, "un pilar no es vertical: {d:?}");
            // Y apoyados en la base del arco.
            assert!(caja.min.y.abs() < 1e-4, "un pilar no arranca en la base");

            alturas.push(d.y);
        }

        let minima = alturas.iter().cloned().fold(f32::INFINITY, f32::min);
        let maxima = alturas.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

        assert!(
            maxima - minima > 1.0,
            "las alturas apenas varian: {minima:.2}..{maxima:.2}"
        );
    }

    #[test]
    fn cluster_de_reparte_por_bloques_consecutivos() {
        let safe = [7, 7, 7, 7];

        assert_eq!(cluster_de(0, &safe), 0);
        assert_eq!(cluster_de(6, &safe), 0);
        assert_eq!(cluster_de(7, &safe), 1);
        assert_eq!(cluster_de(20, &safe), 2);
        assert_eq!(cluster_de(21, &safe), 3);
        assert_eq!(cluster_de(27, &safe), 3);
        // Fuera de rango cae en el ultimo, no entra en panico.
        assert_eq!(cluster_de(99, &safe), 3);
    }

    #[test]
    fn el_xorshift_no_se_queda_clavado_en_cero() {
        let mut generador = Xorshift32::new(0);
        let valores: Vec<f32> = (0..8).map(|_| generador.siguiente()).collect();

        assert!(valores.iter().all(|v| (0.0..1.0).contains(v)));
        assert!(
            valores.windows(2).any(|par| par[0] != par[1]),
            "la secuencia es constante"
        );
    }
}
