use crate::bounds::Aabb;
use crate::color::Color;
use crate::cuboid::Cuboid;
use crate::hit::Hit;
use crate::material::Material;
use crate::primitive::Primitive;
use crate::ray::Ray;
use crate::ray_intersect::RayIntersect;
use crate::skybox::Skybox;
use crate::texture::Texture;
use nalgebra_glm::{Vec2, Vec3};

/// Índice dentro de la paleta de materiales de la escena.
///
/// Los objetos guardan el índice y no el material: durante la revelación
/// cada objeto se refiere a dos materiales a la vez, y duplicar la
/// descripción completa en cada una de las 160 primitivas sería copiar la
/// misma tabla decenas de veces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterialId(pub usize);

/// Índice dentro de la tabla de texturas de la escena.
///
/// Misma razón que `MaterialId`: `Material` es `Copy` y se lee en el camino
/// caliente del renderer. Una `Texture` pesa cientos de kilobytes, así que
/// el material guarda un índice y la escena posee los datos.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureId(pub usize);

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
    /// Cantidad de grupos: el tamaño del arreglo `[f32; 4]` que guarda el
    /// progreso.
    pub const COUNT: usize = 4;

    /// Los cuatro grupos, en el orden de sus índices.
    ///
    /// Existe para poder recorrerlos sin escribirlos a mano en cada sitio:
    /// una lista repetida se queda corta el día que aparezca un quinto
    /// grupo, y el compilador no avisa de una lista incompleta.
    pub const ALL: [RevealGroup; RevealGroup::COUNT] = [
        RevealGroup::Meadows,
        RevealGroup::Breakwater,
        RevealGroup::FlyingWaters,
        RevealGroup::Finale,
    ];

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

impl SceneObject {
    /// ¿Cambia de aspecto al pintarse?
    ///
    /// Dos entradas del inventario no lo hacen, por razones opuestas:
    /// `G-01`, el plinto, es lienzo y se queda así, y `G-04`, la paleta y el
    /// pincel, nace ya en cristal porque es la herramienta con la que se
    /// pinta. Las dos necesitan grupo por tipado y ninguna se revela.
    pub fn is_revealable(&self) -> bool {
        self.initial_material != self.final_material
    }
}

/// La escena completa: los objetos y la paleta que sus índices resuelven.
#[derive(Debug, Default)]
pub struct Scene {
    pub objects: Vec<SceneObject>,
    pub palette: Vec<Material>,
    pub textures: Vec<Texture>,
    /// Cielo de la escena. Por defecto un color plano; con los assets del
    /// Hito 4 cargados, los dos panoramas equirectangulares.
    ///
    /// Vive aquí y no en el renderer por la misma razón que la paleta: es
    /// parte de la descripción de la escena, sus panoramas están en esta
    /// misma tabla de texturas, y así el trazado no necesita un parámetro
    /// más que arrastrar por cada firma.
    pub skybox: Skybox,
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

    /// Registra una textura y devuelve su índice.
    pub fn add_texture(&mut self, texture: Texture) -> TextureId {
        self.textures.push(texture);

        TextureId(self.textures.len() - 1)
    }

    pub fn material(&self, id: MaterialId) -> Material {
        self.palette[id.0]
    }

    /// Grupo de revelación que un clic sobre este objeto debe activar, o
    /// `None` si el objeto no se pinta.
    ///
    /// Es **lo único** que el picking puede leer de un impacto, y por eso
    /// existe como método en vez de dejar que quien recibe el `Hit` husmee
    /// el objeto. El plan lo dice en negativo —«no pintar por vóxel ni
    /// modificar textura libremente»— y esta firma lo hace cumplir: de un
    /// clic se obtiene un grupo, no un objeto, no una cara y no una
    /// coordenada de textura.
    ///
    /// Devuelve `None` para las entradas **inertes**, las que nacen y
    /// mueren con el mismo material. Sin ese filtro, un clic en el plinto
    /// —`G-01`, que ocupa toda la base del diorama— activaría el finale del
    /// Monolito, porque comparte grupo con él por tipado. El plinto es
    /// lienzo y nunca se pinta; pincharlo no debe hacer nada.
    pub fn paintable_group(&self, object_index: usize) -> Option<RevealGroup> {
        let objeto = self.objects.get(object_index)?;

        objeto.is_revealable().then_some(objeto.reveal_group)
    }

    pub fn texture(&self, id: TextureId) -> &Texture {
        &self.textures[id.0]
    }

    /// Albedo efectivo de un material en un punto de su superficie.
    ///
    /// Si el material tiene textura, el color sale de muestrearla y se
    /// **modula por el albedo**, que actúa como tinte. Ese producto es lo
    /// que permite lo que pide el inventario para la cadena del ancla:
    /// reutilizar la textura de `wet_basalt` con otro tinte y otra escala
    /// UV en vez de crear un sexto material final.
    ///
    /// Sin textura, el albedo es el color y punto.
    pub fn albedo_at(&self, material: &Material, uv: &Vec2) -> Color {
        match material.albedo_texture {
            None => material.albedo,
            Some(id) => {
                let muestra = self
                    .texture(id)
                    .sample(uv.x * material.uv_scale, uv.y * material.uv_scale);

                material.albedo * muestra
            }
        }
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
