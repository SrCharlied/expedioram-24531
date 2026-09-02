use crate::bounds::Aabb;
use crate::color::Color;
use crate::cuboid::Cuboid;
use crate::hit::Hit;
use crate::material::Material;
use crate::primitive::Primitive;
use crate::ray::Ray;
use crate::ray_intersect::RayIntersect;
use nalgebra_glm::Vec3;

/// Índice dentro de la paleta de materiales de la escena.
///
/// Los objetos guardan el índice y no el material: durante la revelación
/// cada objeto se refiere a dos materiales a la vez, y duplicar la
/// descripción completa en cada una de las 160 primitivas sería copiar la
/// misma tabla decenas de veces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterialId(pub usize);

/// Grupo de aceleración al que pertenece el objeto. Son los siete nodos de
/// nivel medio del árbol `escena → región → cluster → primitiva`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialGroupId {
    Global,
    ContinentBackground,
    Meadows,
    Breakwater,
    FlyingWaters,
    Monolith,
    InteractionProps,
}

/// Grupo de revelación. Son exactamente cuatro, y el progreso es un escalar
/// por grupo guardado centralmente en `RevealState` a partir de la Tarea
/// 6.3. El objeto solo dice a cuál pertenece.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RevealGroup {
    Meadows,
    Breakwater,
    FlyingWaters,
    Finale,
}

impl SpatialGroupId {
    /// Los siete grupos, en orden fijo. La estructura de aceleración los
    /// recorre así para que el árbol salga idéntico en cada corrida.
    pub const ALL: [SpatialGroupId; 7] = [
        SpatialGroupId::Global,
        SpatialGroupId::ContinentBackground,
        SpatialGroupId::Meadows,
        SpatialGroupId::Breakwater,
        SpatialGroupId::FlyingWaters,
        SpatialGroupId::Monolith,
        SpatialGroupId::InteractionProps,
    ];
}

impl SpatialGroupId {
    /// Posición del grupo dentro de `ALL`. Es el bit que le corresponde en
    /// las máscaras de light linking.
    pub fn index(self) -> usize {
        match self {
            SpatialGroupId::Global => 0,
            SpatialGroupId::ContinentBackground => 1,
            SpatialGroupId::Meadows => 2,
            SpatialGroupId::Breakwater => 3,
            SpatialGroupId::FlyingWaters => 4,
            SpatialGroupId::Monolith => 5,
            SpatialGroupId::InteractionProps => 6,
        }
    }
}

impl RevealGroup {
    /// Cantidad de grupos: el tamaño del arreglo `[f32; 4]` que guardará el
    /// progreso.
    pub const COUNT: usize = 4;

    /// Posición del grupo dentro de ese arreglo.
    pub fn index(self) -> usize {
        match self {
            RevealGroup::Meadows => 0,
            RevealGroup::Breakwater => 1,
            RevealGroup::FlyingWaters => 2,
            RevealGroup::Finale => 3,
        }
    }
}

/// Un objeto de la escena: geometría más a qué pertenece.
///
/// Es inmutable una vez construida la escena. No lleva `reveal_progress`:
/// ese estado vive centralizado en `RevealState`, uno por grupo. Mantener
/// el objeto inmutable es lo que permite construir la estructura de
/// aceleración una sola vez y no invalidarla nunca al pintar.
#[derive(Debug, Clone, Copy)]
pub struct SceneObject {
    pub primitive: Primitive,
    pub initial_material: MaterialId,
    pub final_material: MaterialId,
    pub spatial_group: SpatialGroupId,
    pub reveal_group: RevealGroup,
}

/// La escena completa: los objetos y la paleta que sus índices resuelven.
#[derive(Debug, Default)]
pub struct Scene {
    pub objects: Vec<SceneObject>,
    pub palette: Vec<Material>,
}

impl Scene {
    pub fn new() -> Self {
        Scene::default()
    }

    /// Registra un material y devuelve su índice.
    pub fn add_material(&mut self, material: Material) -> MaterialId {
        self.palette.push(material);

        MaterialId(self.palette.len() - 1)
    }

    pub fn add_object(&mut self, object: SceneObject) {
        self.objects.push(object);
    }

    pub fn material(&self, id: MaterialId) -> Material {
        self.palette[id.0]
    }

    /// Caja envolvente de toda la geometría, o `None` si la escena está
    /// vacía.
    pub fn bounds(&self) -> Option<Aabb> {
        self.objects
            .iter()
            .map(|objeto| objeto.primitive.bounds())
            .reduce(|acumulado, caja| acumulado.union(&caja))
    }

    /// Impacto más cercano contra la escena, con `object_index` asignado.
    ///
    /// Recorrido lineal: prueba todas las primitivas contra todos los rayos.
    /// Es correcto pero no escala; el Hito 3 lo reemplaza por la jerarquía
    /// de grupos y clusters y conserva esta versión como oráculo contra el
    /// cual comparar en los tests.
    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let mut closest: Option<Hit> = None;

        for (index, object) in self.objects.iter().enumerate() {
            if let Some(mut hit) = object.primitive.ray_intersect(ray) {
                if closest.is_none_or(|previo| hit.distance < previo.distance) {
                    hit.object_index = index;
                    closest = Some(hit);
                }
            }
        }

        closest
    }
}

/// Escena de verificacion: un cuboide centrado en el origen.
///
/// No es todavia el diorama. Existe para que los dos binarios --el de
/// ventana y el headless-- rendericen exactamente lo mismo, y para que el
/// gate del Hito 1 siga siendo comprobable. La Tarea 2.4 la reemplaza por
/// el blockout real.
pub fn cubo_de_prueba() -> Scene {
    let mut scene = Scene::new();

    let piedra = scene.add_material(Material::new(Color::new(0.62, 0.60, 0.55)));

    scene.add_object(SceneObject {
        primitive: Cuboid::centrado(Vec3::zeros(), Vec3::new(2.0, 2.0, 2.0)).into(),
        initial_material: piedra,
        final_material: piedra,
        spatial_group: SpatialGroupId::Monolith,
        reveal_group: RevealGroup::Finale,
    });

    scene
}

#[cfg(test)]
mod tests {
    use super::*;

    fn escena_con(centros_z: &[f32]) -> Scene {
        let mut scene = Scene::new();
        let material = scene.add_material(Material::new(Color::new(1.0, 1.0, 1.0)));

        for z in centros_z {
            scene.add_object(SceneObject {
                primitive: Cuboid::centrado(Vec3::new(0.0, 0.0, *z), Vec3::new(1.0, 1.0, 1.0))
                    .into(),
                initial_material: material,
                final_material: material,
                spatial_group: SpatialGroupId::Global,
                reveal_group: RevealGroup::Finale,
            });
        }

        scene
    }

    #[test]
    fn devuelve_el_mas_cercano_aunque_el_lejano_este_primero() {
        // El objeto en z = -10 se registra primero, pero el de z = 0 está
        // mucho más cerca de una cámara en z = +5. El orden de inserción no
        // debe decidir el resultado.
        let scene = escena_con(&[-10.0, 0.0]);
        let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = scene.intersect(&ray).expect("debe impactar");

        assert_eq!(hit.object_index, 1, "ganó el lejano");
        assert!((hit.distance - 4.5).abs() < 1e-5, "{}", hit.distance);
    }

    #[test]
    fn el_orden_inverso_da_el_mismo_resultado() {
        let scene = escena_con(&[0.0, -10.0]);
        let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = scene.intersect(&ray).expect("debe impactar");

        assert_eq!(hit.object_index, 0);
        assert!((hit.distance - 4.5).abs() < 1e-5);
    }

    #[test]
    fn escena_vacia_no_impacta_ni_tiene_bounds() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

        assert!(Scene::new().intersect(&ray).is_none());
        assert!(Scene::new().bounds().is_none());
    }

    #[test]
    fn object_index_resuelve_los_dos_materiales() {
        let mut scene = Scene::new();
        let rojo = scene.add_material(Material::new(Color::new(1.0, 0.0, 0.0)));
        let azul = scene.add_material(Material::new(Color::new(0.0, 0.0, 1.0)));

        scene.add_object(SceneObject {
            primitive: Cuboid::centrado(Vec3::zeros(), Vec3::new(1.0, 1.0, 1.0)).into(),
            initial_material: rojo,
            final_material: azul,
            spatial_group: SpatialGroupId::Monolith,
            reveal_group: RevealGroup::Finale,
        });

        let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = scene.intersect(&ray).expect("debe impactar");
        let objeto = scene.objects[hit.object_index];

        // El impacto no carga material; lo resuelve el índice del objeto.
        assert_eq!(scene.material(objeto.initial_material).albedo.r, 1.0);
        assert_eq!(scene.material(objeto.final_material).albedo.b, 1.0);
    }

    #[test]
    fn los_cuatro_grupos_de_revelacion_indexan_sin_choques() {
        let grupos = [
            RevealGroup::Meadows,
            RevealGroup::Breakwater,
            RevealGroup::FlyingWaters,
            RevealGroup::Finale,
        ];

        let indices: Vec<usize> = grupos.iter().map(|grupo| grupo.index()).collect();

        assert_eq!(indices, vec![0, 1, 2, 3]);
        assert_eq!(grupos.len(), RevealGroup::COUNT);
    }

    #[test]
    fn bounds_envuelve_toda_la_geometria() {
        let scene = escena_con(&[-10.0, 0.0]);
        let caja = scene.bounds().expect("hay geometría");

        assert!((caja.min.z + 10.5).abs() < 1e-5, "{}", caja.min.z);
        assert!((caja.max.z - 0.5).abs() < 1e-5, "{}", caja.max.z);
    }
}
