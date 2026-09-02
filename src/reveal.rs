//! Estado de revelación e interpolación de materiales.
//!
//! Pintar el Continente **no crea, mueve ni destruye geometría**. Toda la
//! escena existe desde el arranque en su posición final; lo único que cambia
//! es qué material se ve, interpolado entre `canvas_unpainted` y el material
//! final.
//!
//! Esa decisión es la que sostiene la arquitectura: si la revelación tocara
//! la geometría, habría que reconstruir la jerarquía de aceleración en cada
//! cuadro de la transición. Como no la toca, se construye una vez y no se
//! invalida nunca.
//!
//! El progreso vive **centralizado** en `RevealState`, un escalar por grupo.
//! `SceneObject` no lo guarda: con 160 primitivas serían 160 copias mutables
//! del mismo dato, y un clic tendría que recorrerlas todas en vez de
//! modificar un `f32`.

use crate::material::{Material, ShadowMode};
use crate::scene::{RevealGroup, Scene, SceneObject};
use nalgebra_glm::Vec2;

/// Progreso de pintura, uno por grupo de revelación.
///
/// Es la **única** fuente de progreso del proyecto. `RevealPhase` y el
/// avance por tiempo llegan en la Tarea 6.3 y se derivan de estos cuatro
/// escalares, sin duplicarlos.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RevealState {
    progress_by_group: [f32; RevealGroup::COUNT],
}

impl Default for RevealState {
    fn default() -> Self {
        RevealState::unpainted()
    }
}

impl RevealState {
    /// Todo en lienzo. Es el estado inicial del diorama.
    pub fn unpainted() -> Self {
        RevealState {
            progress_by_group: [0.0; RevealGroup::COUNT],
        }
    }

    /// Todo pintado. Útil para los renders de evidencia del estado final.
    pub fn painted() -> Self {
        RevealState {
            progress_by_group: [1.0; RevealGroup::COUNT],
        }
    }

    pub fn progress(&self, group: RevealGroup) -> f32 {
        self.progress_by_group[group.index()]
    }

    /// Fija el progreso de un grupo, recortado a `0.0..=1.0`.
    pub fn set_progress(&mut self, group: RevealGroup, progress: f32) {
        self.progress_by_group[group.index()] = if progress.is_nan() {
            0.0
        } else {
            progress.clamp(0.0, 1.0)
        };
    }

    pub fn is_painted(&self, group: RevealGroup) -> bool {
        self.progress(group) >= 1.0
    }

    /// ¿Están pintadas las tres regiones?
    ///
    /// Es la condición que habilita el finale: el Monolito no empieza a
    /// revelarse hasta que Praderas, Rompeolas y Aguas Voladoras llegan a
    /// `1.0`. `Finale` queda fuera a propósito, o la condición se cumpliría
    /// a sí misma.
    pub fn all_regions_painted(&self) -> bool {
        [
            RevealGroup::Meadows,
            RevealGroup::Breakwater,
            RevealGroup::FlyingWaters,
        ]
        .iter()
        .all(|grupo| self.is_painted(*grupo))
    }
}

/// Interpolación lineal entre dos escalares.
fn mezcla(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Material visible de un objeto, con la interpolación ya aplicada.
///
/// Devuelve un `Material` cuyo `albedo` es el color **ya resuelto**: las dos
/// texturas —la del lienzo y la del material final— se muestrean y se
/// mezclan aquí, y `albedo_texture` queda en `None` para que nadie las
/// vuelva a muestrear más abajo.
///
/// Mezclar los colores ya muestreados y no las texturas es lo único posible:
/// una textura no se puede interpolar como dato, solo su resultado en un
/// punto. Y se mezcla en **lineal**, que es donde un punto medio se lee como
/// un punto medio.
///
/// `shadow_mode` **no se interpola**: se toma siempre del material final.
/// Interpolarlo haría que el agua bloqueara sombras a medio camino y el
/// barco parpadearía entre iluminado y negro justo durante la transición
/// estrella del diorama.
pub fn resolve(scene: &Scene, object: &SceneObject, reveal: &RevealState, uv: &Vec2) -> Material {
    let inicial = scene.material(object.initial_material);
    let final_ = scene.material(object.final_material);
    let t = reveal.progress(object.reveal_group).clamp(0.0, 1.0);

    // Atajos para los dos extremos: son el caso común —una región está
    // pintada o no lo está la mayor parte del tiempo— y se ahorran un
    // muestreo de textura por impacto.
    let albedo = if t <= 0.0 {
        scene.albedo_at(&inicial, uv)
    } else if t >= 1.0 {
        scene.albedo_at(&final_, uv)
    } else {
        let a = scene.albedo_at(&inicial, uv);
        let b = scene.albedo_at(&final_, uv);

        a * (1.0 - t) + b * t
    };

    Material {
        albedo,
        // Ya resuelto: volver a muestrear duplicaría el trabajo y, a medio
        // camino, daría el color de una sola de las dos texturas.
        albedo_texture: None,
        specular_strength: mezcla(inicial.specular_strength, final_.specular_strength, t),
        shininess: mezcla(inicial.shininess, final_.shininess, t),
        reflection_cap: mezcla(inicial.reflection_cap, final_.reflection_cap, t),
        transmission_cap: mezcla(inicial.transmission_cap, final_.transmission_cap, t),
        ior: mezcla(inicial.ior, final_.ior, t),
        uv_scale: final_.uv_scale,
        shadow_mode: final_.shadow_mode,
    }
}

/// Modo de sombra efectivo de un objeto.
///
/// Existe como función aparte para dejar constancia de que la consulta de
/// sombras **no pasa por la interpolación**: el recorrido de sombras no
/// necesita resolver texturas ni escalares, solo saber si el objeto bloquea.
pub fn shadow_mode_of(scene: &Scene, object: &SceneObject) -> ShadowMode {
    scene.material(object.final_material).shadow_mode
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accel::SceneAccel;
    use crate::color::Color;
    use crate::cuboid::Cuboid;
    use crate::scene::{MaterialId, SceneObject, SpatialGroupId};
    use crate::texture::Texture;
    use nalgebra_glm::Vec3;

    /// Escena con un objeto que va de lienzo a agua.
    fn escena() -> (Scene, SceneObject, MaterialId, MaterialId) {
        let mut scene = Scene::new();

        let lienzo = scene.add_material(Material::new(Color::new(0.8, 0.8, 0.7)));
        let agua = scene.add_material(
            Material::new(Color::new(0.1, 0.2, 0.6))
                .with_caps(0.9, 0.9, 1.333)
                .with_specular(0.18, 128.0)
                .with_shadow_mode(ShadowMode::Ignore),
        );

        let objeto = SceneObject {
            primitive: Cuboid::centrado(Vec3::zeros(), Vec3::new(2.0, 2.0, 2.0)).into(),
            initial_material: lienzo,
            final_material: agua,
            spatial_group: SpatialGroupId::FlyingWaters,
            reveal_group: RevealGroup::FlyingWaters,
        };
        scene.add_object(objeto);

        (scene, objeto, lienzo, agua)
    }

    fn uv() -> Vec2 {
        Vec2::new(0.4, 0.6)
    }

    #[test]
    fn el_progreso_cero_produce_el_lienzo() {
        let (scene, objeto, lienzo, _) = escena();
        let estado = RevealState::unpainted();

        let visible = resolve(&scene, &objeto, &estado, &uv());
        let esperado = scene.material(lienzo);

        assert_eq!(visible.albedo, esperado.albedo);
        assert_eq!(visible.reflection_cap, esperado.reflection_cap);
        assert_eq!(visible.ior, esperado.ior);
    }

    #[test]
    fn el_progreso_uno_produce_el_material_final() {
        let (scene, objeto, _, agua) = escena();
        let estado = RevealState::painted();

        let visible = resolve(&scene, &objeto, &estado, &uv());
        let esperado = scene.material(agua);

        assert_eq!(visible.albedo, esperado.albedo);
        assert_eq!(visible.reflection_cap, esperado.reflection_cap);
        assert!((visible.ior - esperado.ior).abs() < 1e-6);
    }

    #[test]
    fn el_progreso_intermedio_mezcla_de_forma_estable() {
        let (scene, objeto, lienzo, agua) = escena();
        let a = scene.material(lienzo);
        let b = scene.material(agua);

        let mut anterior = a.albedo.r;
        let mut anterior_cap = a.reflection_cap;

        for paso in 0..=20 {
            let t = paso as f32 / 20.0;
            let mut estado = RevealState::unpainted();
            estado.set_progress(RevealGroup::FlyingWaters, t);

            let visible = resolve(&scene, &objeto, &estado, &uv());

            // Monotono, sin saltos y siempre dentro de los extremos.
            assert!(visible.albedo.r <= anterior + 1e-6, "paso {paso}");
            assert!(visible.albedo.r >= b.albedo.r - 1e-6);
            assert!(visible.reflection_cap >= anterior_cap - 1e-6, "paso {paso}");
            assert!(visible.reflection_cap <= b.reflection_cap + 1e-6);
            assert!(visible.is_valid(), "el material intermedio no es valido");

            anterior = visible.albedo.r;
            anterior_cap = visible.reflection_cap;
        }

        // Y el punto medio esta a medio camino.
        let mut medio = RevealState::unpainted();
        medio.set_progress(RevealGroup::FlyingWaters, 0.5);
        let visible = resolve(&scene, &objeto, &medio, &uv());

        assert!((visible.albedo.r - (a.albedo.r + b.albedo.r) * 0.5).abs() < 1e-5);
        assert!((visible.reflection_cap - 0.45).abs() < 1e-5);
    }

    #[test]
    fn el_shadow_mode_no_se_interpola() {
        // El agua ignora sombras desde el primer instante, aunque todavia se
        // vea como lienzo. Interpolarlo haria parpadear al barco.
        let (scene, objeto, _, _) = escena();

        for paso in 0..=10 {
            let mut estado = RevealState::unpainted();
            estado.set_progress(RevealGroup::FlyingWaters, paso as f32 / 10.0);

            let visible = resolve(&scene, &objeto, &estado, &uv());

            assert_eq!(
                visible.shadow_mode,
                ShadowMode::Ignore,
                "en el progreso {paso}/10"
            );
            assert!(!visible.blocks_shadows());
        }

        // Y la consulta directa que usa el recorrido de sombras coincide.
        assert_eq!(shadow_mode_of(&scene, &objeto), ShadowMode::Ignore);
    }

    #[test]
    fn cambiar_el_progreso_no_toca_la_geometria_ni_los_bounds() {
        // Es la afirmacion de la que cuelga toda la arquitectura: si fallara,
        // habria que reconstruir la jerarquia en cada cuadro de la
        // transicion.
        let (scene, _, _, _) = escena();
        let antes = SceneAccel::build(&scene).expect("hay geometria");
        let objetos_antes = scene.objects.len();
        let bounds_antes: Vec<_> = scene.objects.iter().map(|o| o.primitive.bounds()).collect();

        for paso in 0..=10 {
            let mut estado = RevealState::unpainted();
            estado.set_progress(RevealGroup::FlyingWaters, paso as f32 / 10.0);

            // Resolver el material no muta nada.
            let _ = resolve(&scene, &scene.objects[0], &estado, &uv());

            assert_eq!(scene.objects.len(), objetos_antes);
            let ahora: Vec<_> = scene.objects.iter().map(|o| o.primitive.bounds()).collect();
            assert_eq!(ahora, bounds_antes);
        }

        let despues = SceneAccel::build(&scene).expect("hay geometria");
        assert_eq!(antes.bounds, despues.bounds);
    }

    #[test]
    fn una_entrada_inerte_da_el_mismo_material_en_todo_el_rango() {
        // `G-01` (plinto) y `G-04` (paleta) tienen el mismo material inicial
        // y final: su pertenencia a `Finale` no debe cambiar como se ven.
        let mut scene = Scene::new();
        let unico = scene.add_material(Material::new(Color::new(0.6, 0.6, 0.55)));

        let inerte = SceneObject {
            primitive: Cuboid::centrado(Vec3::zeros(), Vec3::new(1.0, 1.0, 1.0)).into(),
            initial_material: unico,
            final_material: unico,
            spatial_group: SpatialGroupId::Global,
            reveal_group: RevealGroup::Finale,
        };
        scene.add_object(inerte);

        let referencia = resolve(&scene, &inerte, &RevealState::unpainted(), &uv());

        for paso in 0..=10 {
            let mut estado = RevealState::unpainted();
            estado.set_progress(RevealGroup::Finale, paso as f32 / 10.0);

            assert_eq!(resolve(&scene, &inerte, &estado, &uv()), referencia);
        }
    }

    #[test]
    fn el_progreso_se_recorta_y_el_nan_cae_a_cero() {
        let mut estado = RevealState::unpainted();

        estado.set_progress(RevealGroup::Meadows, 3.0);
        assert_eq!(estado.progress(RevealGroup::Meadows), 1.0);

        estado.set_progress(RevealGroup::Meadows, -2.0);
        assert_eq!(estado.progress(RevealGroup::Meadows), 0.0);

        estado.set_progress(RevealGroup::Meadows, f32::NAN);
        assert_eq!(estado.progress(RevealGroup::Meadows), 0.0);
    }

    #[test]
    fn los_cuatro_grupos_son_independientes() {
        let mut estado = RevealState::unpainted();
        estado.set_progress(RevealGroup::Meadows, 1.0);

        assert_eq!(estado.progress(RevealGroup::Meadows), 1.0);
        for otro in [
            RevealGroup::Breakwater,
            RevealGroup::FlyingWaters,
            RevealGroup::Finale,
        ] {
            assert_eq!(estado.progress(otro), 0.0, "{otro:?} se contamino");
        }
    }

    #[test]
    fn el_finale_espera_a_las_tres_regiones() {
        let mut estado = RevealState::unpainted();
        assert!(!estado.all_regions_painted());

        estado.set_progress(RevealGroup::Meadows, 1.0);
        estado.set_progress(RevealGroup::Breakwater, 1.0);
        assert!(!estado.all_regions_painted(), "falta Aguas");

        estado.set_progress(RevealGroup::FlyingWaters, 0.99);
        assert!(!estado.all_regions_painted(), "0.99 no es pintado");

        estado.set_progress(RevealGroup::FlyingWaters, 1.0);
        assert!(estado.all_regions_painted());

        // `Finale` no cuenta para su propia condicion.
        let mut solo_finale = RevealState::unpainted();
        solo_finale.set_progress(RevealGroup::Finale, 1.0);
        assert!(!solo_finale.all_regions_painted());
    }

    #[test]
    fn con_texturas_se_mezclan_las_muestras_no_las_texturas() {
        let mut scene = Scene::new();

        // Dos texturas de un pixel, bien distintas.
        let blanca =
            scene.add_texture(Texture::from_pixels(1, 1, vec![Color::new(1.0, 1.0, 1.0)]).unwrap());
        let negra =
            scene.add_texture(Texture::from_pixels(1, 1, vec![Color::new(0.0, 0.0, 0.0)]).unwrap());

        let lienzo = scene.add_material(Material::new(Color::black()).with_texture(blanca));
        let final_ = scene.add_material(Material::new(Color::black()).with_texture(negra));

        let objeto = SceneObject {
            primitive: Cuboid::centrado(Vec3::zeros(), Vec3::new(1.0, 1.0, 1.0)).into(),
            initial_material: lienzo,
            final_material: final_,
            spatial_group: SpatialGroupId::Meadows,
            reveal_group: RevealGroup::Meadows,
        };

        let mut estado = RevealState::unpainted();
        estado.set_progress(RevealGroup::Meadows, 0.25);

        let visible = resolve(&scene, &objeto, &estado, &uv());

        // Tres cuartos de blanco: la mezcla de las dos MUESTRAS.
        assert!((visible.albedo.r - 0.75).abs() < 1e-6);
        // Y la textura queda resuelta, no pendiente de muestrear.
        assert_eq!(visible.albedo_texture, None);
    }
}
