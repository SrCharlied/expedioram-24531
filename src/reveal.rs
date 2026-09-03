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

    /// Fase de un grupo, derivada de su escalar.
    pub fn phase(&self, group: RevealGroup) -> RevealPhase {
        match self.progress(group) {
            p if p <= 0.0 => RevealPhase::Unpainted,
            p if p >= 1.0 => RevealPhase::Painted,
            _ => RevealPhase::Revealing,
        }
    }

    /// Arranca la revelación de un grupo. Devuelve si algo cambió.
    ///
    /// No hace nada sobre un grupo que ya está pintado o ya en curso: un
    /// segundo clic sobre la misma región no la reinicia ni la acelera.
    pub fn activate(&mut self, group: RevealGroup) -> bool {
        if self.phase(group) != RevealPhase::Unpainted {
            return false;
        }

        self.set_progress(group, ACTIVATION_NUDGE);

        true
    }

    /// Avanza los grupos en curso por **tiempo real**. Devuelve si algo
    /// cambió.
    ///
    /// El avance es por segundos y no por cuadros a propósito: una máquina
    /// lenta tiene que terminar la transición en aproximadamente el mismo
    /// tiempo de pared, con menos cuadros. Los quince cuadros son el
    /// criterio de aceptación del perfil, no el mecanismo de avance.
    ///
    /// Al completarse las tres regiones, **activa el finale**. Esa es la
    /// regla del inventario: el Monolito no se elige, se revela cuando el
    /// Continente está pintado. La condición se deriva del propio estado, así
    /// que no añade nada que pueda desincronizarse.
    pub fn advance(&mut self, delta_seconds: f32, speed: f32) -> bool {
        // Se exige **finito** y no solo positivo. Un `inf` pasaría un
        // guardián de signo y completaría la transición entera en un cuadro,
        // que es exactamente el corte que los quince cuadros existen para
        // evitar; y un `NaN` envenenaría los cuatro escalares. Los dos
        // vienen de un reloj que hipó, y perder un cuadro de animación es
        // mejor que cualquiera de las dos cosas.
        let sano = |v: f32| v.is_finite() && v > 0.0;

        if !sano(delta_seconds) || !sano(speed) {
            return false;
        }

        let paso = delta_seconds * speed;
        let mut cambio = false;

        for grupo in RevealGroup::ALL {
            if self.phase(grupo) != RevealPhase::Revealing {
                continue;
            }

            self.set_progress(grupo, self.progress(grupo) + paso);
            cambio = true;
        }

        // Al final del tick: el finale arranca en el siguiente, y así no se
        // le aplica un paso parcial del mismo en el que las regiones
        // terminaron.
        if self.all_regions_painted() && self.activate(RevealGroup::Finale) {
            cambio = true;
        }

        cambio
    }

    /// Progreso del diorama completo: la media de los cuatro grupos.
    ///
    /// Es lo que interpola el skybox, y por eso incluye `Finale`: el cielo
    /// termina de pintarse cuando termina el diorama, y el Monolito es lo
    /// último que se revela. Con las tres regiones listas y el finale sin
    /// empezar, el cielo va por tres cuartos de camino.
    ///
    /// Media simple y no ponderada por primitivas: lo que se percibe al
    /// mirar el fondo es cuántas regiones están hechas, no cuántos cubos
    /// tenía cada una.
    pub fn global_progress(&self) -> f32 {
        self.progress_by_group.iter().sum::<f32>() / RevealGroup::COUNT as f32
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

/// Fase de un grupo, **derivada** del escalar.
///
/// No se guarda en ninguna parte y nada la escribe: es una vista de solo
/// lectura sobre `progress`. Guardarla sería reintroducir exactamente el
/// estado duplicado que la decisión de centralizar el progreso eliminó, con
/// la posibilidad de que la fase y el escalar dejen de coincidir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RevealPhase {
    Unpainted,
    Revealing,
    Painted,
}

/// Cuadros de transición que se exigen como mínimo.
///
/// Es el **criterio de aceptación**, no el mecanismo: el avance se hace por
/// tiempo real, y estos quince cuadros son lo que garantiza que la
/// transición se lea como animación y no como un corte.
pub const MINIMUM_REVEAL_FRAMES: f32 = 15.0;

/// Piso de la duración. Evita que una máquina rápida convierta la
/// revelación en un parpadeo.
pub const REVEAL_DURATION_FLOOR: f32 = 1.5;

/// Techo de la duración. **No se levanta** si los quince cuadros no caben:
/// en ese caso falla el gate de fluidez, y lo que hay que bajar es la
/// resolución, no alargar la animación.
pub const REVEAL_DURATION_CEILING: f32 = 4.0;

/// El perfil interactivo no alcanza para quince cuadros dentro del techo.
///
/// Es un fallo de gate y no un valor a corregir: alargar la animación para
/// que quepan sería exactamente lo que el plan prohíbe.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FluidityFailure {
    /// Tiempo por cuadro medido, en segundos.
    pub interactive_frame_time: f32,
    /// Duración que harían falta los quince cuadros.
    pub required: f32,
}

/// Duración de la revelación, derivada del tiempo por cuadro medido.
///
/// ```text
/// reveal_duration = clamp(15 x interactive_frame_time, 1.5, 4.0)
/// ```
///
/// No se elige por gusto. El tiempo por cuadro se **mide** en el perfil
/// interactivo, y de ahí sale la duración que garantiza los quince cuadros.
///
/// Devuelve error si el perfil no cabe: con `interactive_frame_time` por
/// encima de `0.267 s`, quince cuadros ya no entran en cuatro segundos y el
/// techo no se levanta.
pub fn reveal_duration(interactive_frame_time: f32) -> Result<f32, FluidityFailure> {
    let required = MINIMUM_REVEAL_FRAMES * interactive_frame_time;

    // La finitud se exige aparte del techo, y no como una comparación
    // negada, para que se lea la intención: un `NaN` no es una duración que
    // quepa, es una medición inválida, y en los dos casos falla el gate.
    let cabe = required.is_finite() && required <= REVEAL_DURATION_CEILING;

    if !cabe {
        return Err(FluidityFailure {
            interactive_frame_time,
            required,
        });
    }

    Ok(required.max(REVEAL_DURATION_FLOOR))
}

/// Progreso por segundo que corresponde a una duración.
pub fn reveal_speed(duration: f32) -> f32 {
    if duration <= 0.0 {
        return 0.0;
    }

    1.0 / duration
}

/// Empujón mínimo que saca a un grupo de cero al activarlo.
///
/// Es el precio de **no** guardar la activación aparte, y se paga a
/// propósito. `Revealing` se deriva del escalar, así que un grupo en cero
/// exacto es indistinguible de uno que nadie tocó: si `activate` no lo
/// moviera, el siguiente `advance` no tendría cómo saber que hay que
/// avanzarlo.
///
/// A la velocidad del proyecto —`0.667` por segundo— este valor equivale a
/// `0.15 ms` de animación: invisible, y suficiente para que el `f32` lo
/// distinga de cero.
const ACTIVATION_NUDGE: f32 = 1e-4;

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

    // ------------------------------------------------- fase derivada

    #[test]
    fn la_fase_se_deriva_del_escalar_y_no_se_guarda() {
        let mut estado = RevealState::unpainted();

        assert_eq!(estado.phase(RevealGroup::Meadows), RevealPhase::Unpainted);

        estado.set_progress(RevealGroup::Meadows, 0.5);
        assert_eq!(estado.phase(RevealGroup::Meadows), RevealPhase::Revealing);

        estado.set_progress(RevealGroup::Meadows, 1.0);
        assert_eq!(estado.phase(RevealGroup::Meadows), RevealPhase::Painted);

        // Y volviendo atras, la fase vuelve con el escalar: no hay nada
        // guardado que pueda quedarse desincronizado.
        estado.set_progress(RevealGroup::Meadows, 0.0);
        assert_eq!(estado.phase(RevealGroup::Meadows), RevealPhase::Unpainted);
    }

    #[test]
    fn los_extremos_pertenecen_a_las_fases_terminales() {
        let mut estado = RevealState::unpainted();

        // Justo por encima de cero ya esta revelando; justo por debajo de
        // uno todavia. Es lo que hace que `advance` sepa a quien mover.
        estado.set_progress(RevealGroup::Breakwater, 1e-6);
        assert_eq!(
            estado.phase(RevealGroup::Breakwater),
            RevealPhase::Revealing
        );

        estado.set_progress(RevealGroup::Breakwater, 1.0 - 1e-6);
        assert_eq!(
            estado.phase(RevealGroup::Breakwater),
            RevealPhase::Revealing
        );
    }

    // ------------------------------------------------- duracion derivada

    #[test]
    fn la_duracion_reproduce_la_tabla_del_plan() {
        // Las cuatro filas tabuladas, con el matiz del techo incluido.
        assert_eq!(reveal_duration(0.05), Ok(1.5), "piso");
        assert_eq!(reveal_duration(0.10), Ok(1.5));
        assert_eq!(reveal_duration(0.20), Ok(3.0));

        let fallo = reveal_duration(0.30).expect_err("0.30 debe fallar el gate");
        assert!((fallo.required - 4.5).abs() < 1e-6, "{:?}", fallo);
    }

    #[test]
    fn el_techo_no_se_levanta_nunca() {
        // El matiz cerrado: pasados los 0.267 s por cuadro, quince cuadros
        // no caben en cuatro segundos y **falla el gate**. No se alarga la
        // animacion para que quepan.
        let critico = REVEAL_DURATION_CEILING / MINIMUM_REVEAL_FRAMES;

        assert!(
            (critico - 0.266_666_7).abs() < 1e-6,
            "el critico es {critico}"
        );

        assert!(reveal_duration(critico - 1e-4).is_ok());
        assert!(reveal_duration(critico + 1e-4).is_err());

        // Y ninguna duracion valida pasa del techo.
        for paso in 0..=40 {
            let cuadro = paso as f32 / 100.0;

            if let Ok(duracion) = reveal_duration(cuadro) {
                assert!(
                    duracion <= REVEAL_DURATION_CEILING,
                    "con {cuadro} s por cuadro salio {duracion}"
                );
                assert!(duracion >= REVEAL_DURATION_FLOOR);
            }
        }
    }

    #[test]
    fn un_tiempo_por_cuadro_absurdo_falla_en_vez_de_colarse() {
        assert!(
            reveal_duration(f32::NAN).is_err(),
            "un NaN no es una duracion"
        );
        assert!(reveal_duration(f32::INFINITY).is_err());
        // Cero o negativo caen al piso: no hay quince cuadros que garantizar
        // si el cuadro no cuesta nada, y el piso sigue evitando el
        // parpadeo.
        assert_eq!(reveal_duration(0.0), Ok(REVEAL_DURATION_FLOOR));
    }

    #[test]
    fn el_perfil_medido_del_proyecto_pasa_el_gate_con_margen() {
        // `interactive_frame_time` medido en el perfil MEDIA con optica
        // completa, escena refractiva y reveal 1.0: el caso mas caro.
        let medido = 0.0490;
        let duracion = reveal_duration(medido).expect("el perfil MEDIA cabe");

        assert_eq!(duracion, REVEAL_DURATION_FLOOR, "cae al piso");

        // Y con margen: el critico esta cinco veces mas arriba.
        let critico = REVEAL_DURATION_CEILING / MINIMUM_REVEAL_FRAMES;
        assert!(critico / medido > 5.0, "margen {}", critico / medido);

        // A esa duracion y ese cuadro salen unos treinta cuadros, el doble
        // del minimo exigido.
        let cuadros = duracion / medido;
        assert!(
            cuadros > MINIMUM_REVEAL_FRAMES * 2.0,
            "solo {cuadros} cuadros"
        );
    }

    #[test]
    fn la_velocidad_es_el_reciproco_de_la_duracion() {
        assert!((reveal_speed(1.5) - 0.666_666_7).abs() < 1e-6);
        assert!((reveal_speed(4.0) - 0.25).abs() < 1e-6);
        // Una duracion degenerada no divide entre cero.
        assert_eq!(reveal_speed(0.0), 0.0);
        assert_eq!(reveal_speed(-1.0), 0.0);
    }

    // ------------------------------------------------- avance temporizado

    #[test]
    fn activar_saca_al_grupo_de_cero_para_que_advance_lo_vea() {
        // El precio de derivar la fase: sin el empujon, `advance` no tendria
        // como distinguir un grupo activado de uno que nadie toco.
        let mut estado = RevealState::unpainted();

        assert!(estado.activate(RevealGroup::Meadows));
        assert_eq!(estado.phase(RevealGroup::Meadows), RevealPhase::Revealing);
        assert!(estado.progress(RevealGroup::Meadows) > 0.0);

        // Y el empujon es invisible: menos de un milesimo del recorrido.
        assert!(estado.progress(RevealGroup::Meadows) < 1e-3);
    }

    #[test]
    fn activar_dos_veces_no_reinicia_ni_acelera() {
        let mut estado = RevealState::unpainted();

        estado.activate(RevealGroup::Meadows);
        estado.advance(0.5, reveal_speed(1.5));
        let a_medias = estado.progress(RevealGroup::Meadows);

        assert!(
            !estado.activate(RevealGroup::Meadows),
            "no deberia reactivar"
        );
        assert_eq!(estado.progress(RevealGroup::Meadows), a_medias);

        // Ni sobre una region ya pintada.
        estado.set_progress(RevealGroup::Breakwater, 1.0);
        assert!(!estado.activate(RevealGroup::Breakwater));
    }

    #[test]
    fn el_avance_es_por_tiempo_de_pared_y_no_por_cuadros() {
        // La misma duracion con dos cadencias distintas: una maquina con la
        // mitad de cuadros llega al mismo punto en el mismo tiempo.
        let velocidad = reveal_speed(1.5);

        let mut rapida = RevealState::unpainted();
        let mut lenta = RevealState::unpainted();

        rapida.activate(RevealGroup::Meadows);
        lenta.activate(RevealGroup::Meadows);

        // Treinta cuadros de 1/60 s contra quince de 1/30 s: un segundo en
        // los dos casos.
        for _ in 0..30 {
            rapida.advance(1.0 / 60.0, velocidad);
        }
        for _ in 0..15 {
            lenta.advance(1.0 / 30.0, velocidad);
        }

        let diferencia =
            (rapida.progress(RevealGroup::Meadows) - lenta.progress(RevealGroup::Meadows)).abs();

        assert!(diferencia < 1e-5, "difieren en {diferencia}");
    }

    #[test]
    fn la_revelacion_termina_en_la_duracion_derivada() {
        let duracion = reveal_duration(0.0490).expect("cabe");
        let velocidad = reveal_speed(duracion);

        let mut estado = RevealState::unpainted();
        estado.activate(RevealGroup::FlyingWaters);

        // Un cuadro antes de tiempo todavia no ha terminado.
        let paso = 0.0490;
        let mut transcurrido = 0.0;

        while transcurrido < duracion - paso {
            estado.advance(paso, velocidad);
            transcurrido += paso;
        }

        assert_eq!(
            estado.phase(RevealGroup::FlyingWaters),
            RevealPhase::Revealing,
            "termino antes de la duracion"
        );

        // Y al pasarse, se queda en pintado sin desbordar.
        for _ in 0..5 {
            estado.advance(paso, velocidad);
        }

        assert_eq!(
            estado.phase(RevealGroup::FlyingWaters),
            RevealPhase::Painted
        );
        assert_eq!(estado.progress(RevealGroup::FlyingWaters), 1.0);
    }

    #[test]
    fn el_avance_no_toca_lo_que_nadie_activo() {
        let mut estado = RevealState::unpainted();
        estado.activate(RevealGroup::Meadows);

        assert!(estado.advance(0.1, reveal_speed(1.5)));

        assert!(estado.progress(RevealGroup::Meadows) > 0.0);
        assert_eq!(estado.progress(RevealGroup::Breakwater), 0.0);
        assert_eq!(estado.progress(RevealGroup::FlyingWaters), 0.0);
    }

    #[test]
    fn un_delta_absurdo_no_envenena_el_estado() {
        let mut estado = RevealState::unpainted();
        estado.activate(RevealGroup::Meadows);
        let antes = estado.progress(RevealGroup::Meadows);

        for delta in [0.0, -1.0, f32::NAN, f32::INFINITY] {
            assert!(!estado.advance(delta, reveal_speed(1.5)), "delta {delta}");
            assert_eq!(estado.progress(RevealGroup::Meadows), antes);
        }

        // Y una velocidad absurda tampoco.
        assert!(!estado.advance(0.1, f32::NAN));
        assert!(!estado.advance(0.1, 0.0));
        assert_eq!(estado.progress(RevealGroup::Meadows), antes);
    }

    #[test]
    fn el_finale_arranca_solo_al_terminar_las_tres_regiones() {
        // La regla del inventario: el Monolito no se elige, se revela cuando
        // el Continente esta pintado. Y se deriva del propio estado.
        let velocidad = reveal_speed(1.5);
        let mut estado = RevealState::unpainted();

        for grupo in [
            RevealGroup::Meadows,
            RevealGroup::Breakwater,
            RevealGroup::FlyingWaters,
        ] {
            estado.activate(grupo);
        }

        // Con dos regiones listas el finale sigue sin arrancar.
        estado.set_progress(RevealGroup::Meadows, 1.0);
        estado.set_progress(RevealGroup::Breakwater, 1.0);
        estado.advance(0.1, velocidad);

        assert_eq!(estado.phase(RevealGroup::Finale), RevealPhase::Unpainted);

        // Al completarse la tercera, arranca.
        estado.set_progress(RevealGroup::FlyingWaters, 1.0);
        assert!(
            estado.advance(0.1, velocidad),
            "el tick tenia que activarlo"
        );
        assert_eq!(estado.phase(RevealGroup::Finale), RevealPhase::Revealing);

        // Y termina como cualquier otro grupo.
        for _ in 0..40 {
            estado.advance(0.05, velocidad);
        }

        assert_eq!(estado.phase(RevealGroup::Finale), RevealPhase::Painted);
    }

    #[test]
    fn el_finale_no_se_activa_a_si_mismo_con_dos_regiones() {
        // `all_regions_painted` deja fuera al finale a proposito, o la
        // condicion se cumpliria sola. Este test lo comprueba desde el tick.
        let mut estado = RevealState::unpainted();
        estado.set_progress(RevealGroup::Finale, 1.0);

        assert!(!estado.all_regions_painted());
        estado.advance(0.1, reveal_speed(1.5));

        for grupo in [
            RevealGroup::Meadows,
            RevealGroup::Breakwater,
            RevealGroup::FlyingWaters,
        ] {
            assert_eq!(estado.phase(grupo), RevealPhase::Unpainted);
        }
    }
}
