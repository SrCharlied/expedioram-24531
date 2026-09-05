//! Trazado de la escena: de un píxel a un color.
//!
//! Estaba en `main.rs` junto al ciclo de ventana. Vive aparte porque es la
//! parte que hay que poder ejecutar sin abrir una ventana, tanto en tests
//! como en el render headless que exige el plan.

use crate::accel::{SceneAccel, TraversalStats};
use crate::camera::Camera;
use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::hit::Hit;
use crate::light::PointLight;
use crate::material::{direct_diffuse, direct_specular, AMBIENT};
use crate::optics::{fresnel, reflected_ray, refracted_ray, EnergySplit};
use crate::ray::Ray;
use crate::reveal::{resolve, RevealState};
use crate::scene::Scene;
use crate::skybox::FALLBACK_COLOR;
use crate::EPSILON;
use nalgebra_glm::Vec3;

/// Color que devuelve un rayo que no toca nada **cuando la escena no
/// tiene panoramas cargados**.
///
/// Se conserva el nombre porque es el que usan los tests del Hito 1, pero
/// desde la Tarea 4.5 ya no es el fondo: el fondo lo decide `Skybox`, y
/// este es solo el color de su variante plana. Vive en `skybox`, que es
/// quien manda sobre el cielo.
pub const BACKGROUND_COLOR: u32 = FALLBACK_COLOR;

/// Resolución a la que se dibujan los cuadros mientras algo se mueve.
///
/// A `800 × 600` el nivel seguro refractivo tarda `0.4463 s` por cuadro
/// —unos 2 fps—, y eso es latencia perceptible al orbitar. Mientras la
/// cámara o la revelación cambian se dibuja a menor resolución y se escala;
/// al quedar todo quieto se produce un cuadro final a resolución completa.
///
/// Las dos opciones salen de medición, no de una suposición. Las cifras de
/// abajo se remidieron con la óptica del Hito 5 activa: las de la Tarea 3.8
/// eran de **antes** de la refracción y quedaron a menos de la mitad del
/// costo real.
///
/// Todas: preset `safe-refractive-water`, estado pintado, **toma hero**,
/// mediana de quince rondas intercaladas y rotadas en release; commit
/// `20a974e`, 4 de septiembre de 2026, Ryzen 7 6800H, rustc 1.97.0. Se
/// rederivan con
/// `cargo run --release --example performance_matrix`.
///
/// # La toma hero no es el peor encuadre
///
/// Estas cifras son del encuadre que se presenta. Los dos encuadres pegados
/// al radio mínimo cuestan **más del triple** que esto, porque llenan la
/// pantalla de bahía refractiva. Son con los que la ventana calibra; ver
/// `Blockout::calibration_cameras`.
///
/// # Ráfaga contra carga sostenida
///
/// Son más altas que las que registró el Hito 6 para los mismos estados, y
/// la diferencia no es ruido: las de entonces salían de una ráfaga de
/// quince renders seguidos, y estas de corridas que sostienen la carga
/// varios minutos sobre cuarenta y ocho encuadres. La máquina baja de
/// frecuencia. Las sostenidas son las que hay que usar para dimensionar
/// —una sesión de la demo se parece más a eso que a una ráfaga—, y el
/// sentido del sesgo es el seguro: sobreestiman el coste.
///
/// Por lo mismo, **no compares un número de aquí con uno de otra corrida**:
/// el suelo se movió más de un `50 %` entre corridas del mismo día según lo
/// caliente que estuviera la máquina. Lo que reproduce son los cocientes
/// dentro de una corrida.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InteractiveProfile {
    pub width: usize,
    pub height: usize,
}

impl InteractiveProfile {
    /// Media resolución. Fue el punto de partida que fija el plan y el
    /// defecto hasta la Tarea 7.1; ahora es la opción de calidad, no la de
    /// serie.
    ///
    /// En el peor encuadre alcanzable, con el zoom ya recortado a `1.8 S`,
    /// deja `1.48x` de margen sobre el crítico del gate de fluidez. Es un
    /// aprobado, pero con una dispersión de máquina del `16 %` entre
    /// corridas del mismo día es un aprobado que depende del día; `BAJA`
    /// deja `2.37x`. Ver la Tarea 7.1.
    pub const MEDIA: InteractiveProfile = InteractiveProfile {
        width: 400,
        height: 300,
    };

    /// El perfil **de serie** desde la Tarea 7.1. El plan ya lo preveía
    /// como reserva; la medición lo convirtió en el defecto.
    ///
    /// `0.0717 s` en el peor estado de la toma hero —unos 14 fps— y
    /// `0.1259 s` en el peor encuadre alcanzable, que deja los quince
    /// cuadros en `1.89 s` contra un techo de `4.0 s`: `2.1x` de margen.
    ///
    /// Lo que se paga es resolución **mientras algo se mueve**. Al soltar
    /// los controles se produce el cuadro final a `800 × 600`, que es lo que
    /// se mira de verdad. La comparación visual está en
    /// `cargo run --release --example profile_preview`.
    pub const BAJA: InteractiveProfile = InteractiveProfile {
        width: 320,
        height: 240,
    };

    pub fn pixels(&self) -> usize {
        self.width * self.height
    }
}

impl Default for InteractiveProfile {
    /// `BAJA`, y no `MEDIA`, desde la Tarea 7.1.
    ///
    /// `MEDIA` era el punto de partida del plan y aguanta de sobra la toma
    /// hero. Lo que no aguanta es el peor encuadre alcanzable: con el zoom
    /// recortado a `1.8 S` deja `1.44x` de margen sobre el crítico del gate
    /// de fluidez, contra `2.34x` de `BAJA`. Con una dispersión de máquina
    /// del `16 %` entre corridas del mismo día, `1.44x` es un aprobado que
    /// depende del día.
    ///
    /// Lo que se paga es resolución **mientras algo se mueve**, y solo
    /// entonces: al soltar los controles se produce el cuadro final a
    /// `800 × 600`. El plan ya preveía `320 × 240` para esto.
    fn default() -> Self {
        InteractiveProfile::BAJA
    }
}

/// Qué hacer con el cuadro que toca presentar.
///
/// Es la política del *dirty rendering* del Hito 6, sacada del ciclo de la
/// ventana para poder comprobarla sin abrirla. Dentro del ciclo era una
/// pareja de banderas mutables cuya corrección había que leer siguiendo el
/// flujo; aquí es una tabla de tres filas con tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FramePlan {
    /// Nada cambió y el cuadro definitivo ya está dibujado: se vuelve a
    /// presentar el mismo framebuffer.
    Reuse,
    /// Algo se está moviendo. Se traza al perfil interactivo y se escala.
    Interactive,
    /// Se detuvo el movimiento y falta el cuadro a resolución completa.
    Final,
}

/// Decide el plan del cuadro.
///
/// `changing` es cierto mientras haya un cambio **sostenido** —órbita,
/// zoom o una región revelándose—, es decir uno que va a seguir cambiando
/// el cuadro siguiente. Un cambio instantáneo no entra aquí: ya terminó, y
/// gastarle un cuadro de baja resolución no aporta nada. Lo que hace es
/// dejar `final_pending` en cierto.
pub fn plan_frame(changing: bool, final_pending: bool) -> FramePlan {
    match (changing, final_pending) {
        (true, _) => FramePlan::Interactive,
        (false, true) => FramePlan::Final,
        (false, false) => FramePlan::Reuse,
    }
}

impl FramePlan {
    /// ¿Hay que trazar la escena en este cuadro?
    pub fn draws(self) -> bool {
        self != FramePlan::Reuse
    }

    /// ¿Conviene dormir después de presentar?
    ///
    /// Solo en reposo. Dormir mientras algo se mueve le quita cuadros a la
    /// animación, y desde la Tarea 7.1 no hay de dónde quitarlos: la
    /// duración se deriva del peor encuadre y da **exactamente** los quince
    /// cuadros del criterio, sin sobrantes. Un `16 ms` de más por cuadro se
    /// los comería.
    pub fn should_sleep(self) -> bool {
        self == FramePlan::Reuse
    }
}

/// Cómo se resuelve el color de un impacto.
///
/// `Normals` no es sombreado sino una vista de depuración: cada eje se ve
/// de un color distinto, así que una cara mal orientada salta a simple
/// vista. Es lo que hace verificable el cubo mientras no haya luces —hasta
/// el Hito 3 un color plano solo daría una silueta—, y sigue sirviendo
/// después para revisar geometría nueva.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Shading {
    /// Iluminación completa: ambiente, difusa y specular directa.
    #[default]
    Material,
    /// Albedo plano, sin luces. Es con lo que se juzgó la composición del
    /// Blockout 1, y sigue sirviendo para reproducir aquellas imágenes.
    Albedo,
    Normals,
}

/// Traduce una normal a color, llevando el rango `-1.0..=1.0` de cada
/// componente a `0.0..=1.0`.
pub fn color_por_normal(hit: &Hit) -> Color {
    Color::new(
        hit.normal.x * 0.5 + 0.5,
        hit.normal.y * 0.5 + 0.5,
        hit.normal.z * 0.5 + 0.5,
    )
}

/// Profundidad de recursión inicial.
///
/// **Tres, no dos.** Un rayo primario que entra al volumen cerrado de
/// Aguas gasta un nivel al refractar en la cara frontal; si no impacta el
/// barco, gasta el segundo en la cara interna trasera y necesita el tercero
/// para llegar al lecho, las rocas o el cielo del otro lado. Con dos, todo
/// lo que está **detrás** del volumen se pierde.
///
/// Bajarlo a dos es la mitigación número 2 del gate de Aguas Voladoras, y
/// solo si la medición lo exige, registrando qué se pierde.
pub const MAX_DEPTH: u32 = 3;

/// Devuelve el color del objeto más cercano que toca el rayo.
///
/// Entra con `MAX_DEPTH` y acota el resultado al final. Es la puerta que
/// usan la cámara y el picking; la recursión vive en `trace`.
pub fn cast_ray(
    ray: &Ray,
    scene: &Scene,
    accel: &SceneAccel,
    lights: &[PointLight],
    reveal: &RevealState,
    shading: Shading,
    stats: &mut TraversalStats,
) -> Color {
    cast_ray_depth(ray, scene, accel, lights, reveal, shading, MAX_DEPTH, stats)
}

/// Igual que `cast_ray`, con la profundidad explícita.
///
/// Existe para poder medir qué se pierde al bajarla: el plan exige
/// documentar esa comparación antes de aceptar `max_depth = 2`, y eso no se
/// puede hacer con la profundidad clavada en una constante.
#[allow(clippy::too_many_arguments)]
pub fn cast_ray_depth(
    ray: &Ray,
    scene: &Scene,
    accel: &SceneAccel,
    lights: &[PointLight],
    reveal: &RevealState,
    shading: Shading,
    max_depth: u32,
    stats: &mut TraversalStats,
) -> Color {
    // Paso 6 del orden: acotar. Se hace **una vez**, aquí arriba, y no en
    // cada nivel de la recursión. Recortar dentro descartaría energía antes
    // de que el padre la pese por `kr` o `kt`, y un reflejo brillante se
    // vería apagado por una razón que no está en ninguna parte del modelo.
    acotar(trace(
        ray, scene, accel, lights, reveal, shading, max_depth, stats,
    ))
}

/// El trazado recursivo.
///
/// `depth` es el número de superficies que **todavía** se pueden sombrear.
/// Al llegar a cero el rayo devuelve el cielo en su dirección, nunca negro:
/// con `kl = 0.1` no hay color local suficiente para disimular un terminal
/// oscuro, y se vería como manchas dentro del agua.
///
/// El orden de las contribuciones es el del plan:
///
/// 1. Impacto.
/// 2. Iluminación directa difusa, con sombras y light linking.
/// 3. Specular directo.
/// 4. Reparto de Fresnel y rayo reflejado si vale la pena.
/// 5. Rayo refractado si vale la pena.
/// 6. Acotado, que hace `cast_ray_depth` una sola vez.
#[allow(clippy::too_many_arguments)]
fn trace(
    ray: &Ray,
    scene: &Scene,
    accel: &SceneAccel,
    lights: &[PointLight],
    reveal: &RevealState,
    shading: Shading,
    depth: u32,
    stats: &mut TraversalStats,
) -> Color {
    if depth == 0 {
        return scene.skybox.sample(scene, &ray.direction, reveal);
    }

    let Some(hit) = accel.intersect(scene, ray, stats) else {
        // Un miss no devuelve un color fijo: devuelve el cielo en la
        // dirección del rayo.
        return scene.skybox.sample(scene, &ray.direction, reveal);
    };

    let objeto = scene.objects[hit.object_index];

    // Un solo lugar resuelve el material visible: muestrea las texturas,
    // interpola lienzo hacia material final según el progreso del grupo, y
    // toma el `shadow_mode` del final sin interpolarlo. Se hace una vez por
    // impacto y no una por luz: hay hasta tres luces por punto.
    let material = resolve(scene, &objeto, reveal, &hit.uv);

    match shading {
        Shading::Normals => color_por_normal(&hit),
        Shading::Albedo => material.albedo,
        Shading::Material => {
            // Ambiente: no es física, es el suelo que impide que lo no
            // iluminado quede en negro absoluto y pierda su silueta. Cuenta
            // como color propio, así que va dentro de lo local.
            let mut local = material.albedo * AMBIENT;
            let mut especular = Color::black();

            // El ojo, no la luz: el specular depende de desde dónde se mira.
            let hacia_ojo = -ray.direction;

            for light in lights {
                // Light linking, antes de calcular nada: si esta luz no
                // ilumina a este grupo, no cuesta ni una operación más.
                if !light.affects(objeto.spatial_group) {
                    continue;
                }

                let hacia_luz = light.position - hit.point;
                let distancia = hacia_luz.magnitude();

                if distancia <= f32::EPSILON {
                    continue;
                }

                let atenuacion = light.attenuation(distancia);
                if atenuacion <= 0.0 {
                    continue;
                }

                let direccion = hacia_luz / distancia;

                if light.casts_shadows
                    && en_sombra(scene, accel, &hit, &direccion, distancia, light, stats)
                {
                    continue;
                }

                local = local
                    + direct_diffuse(&material, &hit.normal, &direccion, light.color, atenuacion);
                especular = especular
                    + direct_specular(
                        &material,
                        &hit.normal,
                        &direccion,
                        &hacia_ojo,
                        light.color,
                        atenuacion,
                    );
            }

            // Reparto de Fresnel. `fresnel` resuelve el lado por
            // `front_face` y devuelve exactamente `1.0` en reflexión total
            // interna, lo que deja `kt = 0` y manda toda la energía al rayo
            // reflejado sin que aquí haga falta un caso especial.
            let f = fresnel(&ray.direction, &hit.normal, hit.front_face, material.ior);
            let reparto = EnergySplit::for_material(&material, f);

            // Lo local se pesa por `kl`; el brillo se suma después, sin
            // pesar. Ver `material::direct_specular`.
            let mut color = local * reparto.local + especular;

            if reparto.worth_reflecting() {
                stats.reflection_rays += 1;

                let secundario = reflected_ray(&hit, &ray.direction);
                let aporte = trace(
                    &secundario,
                    scene,
                    accel,
                    lights,
                    reveal,
                    shading,
                    depth - 1,
                    stats,
                );

                color = color + aporte * reparto.reflected;
            }

            if reparto.worth_refracting() {
                // `refracted_ray` no puede fallar aquí: `kt > 0` implica
                // `F < 1`, que es la misma condición que descarta la
                // reflexión total. Se escribe con `if let` y no con
                // `expect` porque las dos comprobaciones son aritmética de
                // punto flotante y un `panic` en el camino caliente sería
                // una forma pésima de descubrir que difieren.
                if let Some(secundario) = refracted_ray(&hit, &ray.direction, material.ior) {
                    stats.refraction_rays += 1;

                    let aporte = trace(
                        &secundario,
                        scene,
                        accel,
                        lights,
                        reveal,
                        shading,
                        depth - 1,
                        stats,
                    );

                    color = color + aporte * reparto.transmitted;
                }
            }

            color
        }
    }
}

/// Acota el color al rango representable.
///
/// No es tone mapping filmico: es un recorte. Un `NaN` se vuelve negro y no
/// un byte cualquiera, y un desborde se satura en blanco. `Color::to_hex`
/// ya hace lo mismo al escribir el píxel; esto existe para que el valor que
/// devuelve `cast_ray` —el que consultan los tests y el picking— ya venga
/// acotado, y no solo el que llega al framebuffer.
fn acotar(color: Color) -> Color {
    let canal = |v: f32| if v.is_nan() { 0.0 } else { v.clamp(0.0, 1.0) };

    Color::new(canal(color.r), canal(color.g), canal(color.b))
}

/// ¿Hay algo entre el punto y la luz?
///
/// Dos precauciones concentran casi todos los defectos clásicos de sombras:
///
/// - El origen se separa de la superficie a lo largo de la normal. Sin ese
///   desplazamiento el rayo vuelve a impactar el punto del que sale por
///   error de redondeo, y la superficie se cubre del moteado que se conoce
///   como acné de sombras.
/// - La búsqueda se corta en `distancia - EPSILON`. Sin ese tope, un objeto
///   situado **detrás** de la luz proyectaría una sombra que no existe.
fn en_sombra(
    scene: &Scene,
    accel: &SceneAccel,
    hit: &Hit,
    hacia_luz: &Vec3,
    distancia: f32,
    light: &PointLight,
    stats: &mut TraversalStats,
) -> bool {
    stats.shadow_rays += 1;

    let origen = hit.point + hit.normal * EPSILON;
    let rayo_de_sombra = Ray::new(origen, *hacia_luz);

    accel.occluded(
        scene,
        &rayo_de_sombra,
        distancia - EPSILON,
        light.occluder_groups,
        stats,
    )
}

/// Renderiza la escena completa y devuelve los contadores del recorrido.
///
/// Los contadores no son adorno: son lo que permite que las mediciones del
/// Hito 3 digan cuántas pruebas se evitaron, y no solo cuánto se tardó.
pub fn render(
    framebuffer: &mut Framebuffer,
    scene: &Scene,
    accel: &SceneAccel,
    lights: &[PointLight],
    reveal: &RevealState,
    camera: &Camera,
    shading: Shading,
) -> TraversalStats {
    let (ancho, alto) = (framebuffer.width, framebuffer.height);
    let mut stats = TraversalStats::default();

    for y in 0..alto {
        for x in 0..ancho {
            // La generación del rayo vive en la cámara: el picking del Hito
            // 6 tiene que usar exactamente la misma función para que un clic
            // caiga en el píxel que se ve.
            let ray = camera.ray_from_pixel(x, y, ancho, alto);
            stats.primary_rays += 1;

            let color = cast_ray(&ray, scene, accel, lights, reveal, shading, &mut stats);

            framebuffer.set_current_color(color.to_hex());
            framebuffer.point(x, y);
        }
    }

    stats
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::cuboid::Cuboid;
    use crate::light::{GroupMask, PointLight};
    use crate::material::{Material, ShadowMode};
    use crate::scene::{RevealGroup, SceneObject, SpatialGroupId};
    use crate::skybox::Skybox;

    /// Suelo amplio en el origen, para tener una superficie que iluminar.
    fn suelo() -> (Scene, SceneAccel) {
        let mut scene = Scene::new();
        let material = scene.add_material(Material::new(Color::new(0.8, 0.8, 0.8)));

        scene.add_object(SceneObject {
            primitive: Cuboid::centrado(Vec3::new(0.0, -0.5, 0.0), Vec3::new(20.0, 1.0, 20.0))
                .into(),
            initial_material: material,
            final_material: material,
            spatial_group: SpatialGroupId::Global,
            reveal_group: RevealGroup::Finale,
        });

        let accel = SceneAccel::build(&scene).expect("hay geometria");

        (scene, accel)
    }

    /// Suelo mas un bloqueador encima, con el modo de sombra indicado.
    fn suelo_con_bloqueador(modo: ShadowMode, grupo: SpatialGroupId) -> (Scene, SceneAccel) {
        let mut scene = Scene::new();
        let piso = scene.add_material(Material::new(Color::new(0.8, 0.8, 0.8)));
        let tapa =
            scene.add_material(Material::new(Color::new(0.4, 0.4, 0.4)).with_shadow_mode(modo));

        scene.add_object(SceneObject {
            primitive: Cuboid::centrado(Vec3::new(0.0, -0.5, 0.0), Vec3::new(20.0, 1.0, 20.0))
                .into(),
            initial_material: piso,
            final_material: piso,
            spatial_group: SpatialGroupId::Global,
            reveal_group: RevealGroup::Finale,
        });

        // Losa flotando entre el suelo y la luz.
        scene.add_object(SceneObject {
            primitive: Cuboid::centrado(Vec3::new(0.0, 3.0, 0.0), Vec3::new(4.0, 0.5, 4.0)).into(),
            initial_material: tapa,
            final_material: tapa,
            spatial_group: grupo,
            reveal_group: RevealGroup::Finale,
        });

        let accel = SceneAccel::build(&scene).expect("hay geometria");

        (scene, accel)
    }

    fn luz_cenital(altura: f32) -> PointLight {
        PointLight {
            id: "prueba",
            position: Vec3::new(0.0, altura, 0.0),
            color: Color::new(1.0, 1.0, 1.0),
            intensity: 1.0,
            range: 100.0,
            casts_shadows: true,
            affected_groups: GroupMask::ALL,
            occluder_groups: GroupMask::ALL,
        }
    }

    /// Rayo que mira el origen desde arriba y en diagonal.
    fn rayo_al_origen() -> Ray {
        let origen = Vec3::new(0.0, 6.0, 6.0);
        Ray::new(origen, (Vec3::zeros() - origen).normalize())
    }

    fn brillo(color: Color) -> f32 {
        color.r + color.g + color.b
    }

    #[test]
    fn el_epsilon_evita_el_acne_de_sombras() {
        // Una superficie iluminada de frente no puede hacerse sombra a si
        // misma. Sin desplazar el origen del rayo de sombra a lo largo de
        // la normal, el rayo reimpacta el punto del que sale y el suelo se
        // cubre de moteado.
        let (scene, accel) = suelo();
        let luces = [luz_cenital(10.0)];
        let mut stats = TraversalStats::default();

        let color = cast_ray(
            &rayo_al_origen(),
            &scene,
            &accel,
            &luces,
            &RevealState::painted(),
            Shading::Material,
            &mut stats,
        );

        // Muy por encima del ambiente: el punto esta iluminado, no en sombra.
        assert!(
            brillo(color) > 3.0 * AMBIENT * 3.0,
            "el suelo se sombreo a si mismo: {color}"
        );
    }

    #[test]
    fn un_opaco_entre_el_punto_y_la_luz_bloquea() {
        let (scene, accel) = suelo_con_bloqueador(ShadowMode::Opaque, SpatialGroupId::Monolith);
        let luces = [luz_cenital(10.0)];
        let mut stats = TraversalStats::default();

        let color = cast_ray(
            &rayo_al_origen(),
            &scene,
            &accel,
            &luces,
            &RevealState::painted(),
            Shading::Material,
            &mut stats,
        );

        // Solo queda el ambiente.
        let solo_ambiente = 0.8 * AMBIENT * 3.0;
        assert!(
            brillo(color) < solo_ambiente + 1e-4,
            "deberia estar en sombra: {color}"
        );
    }

    #[test]
    fn un_opaco_detras_de_la_luz_no_bloquea() {
        // Misma losa a y = 3, pero la luz ahora esta por debajo de ella.
        let (scene, accel) = suelo_con_bloqueador(ShadowMode::Opaque, SpatialGroupId::Monolith);
        let luces = [luz_cenital(2.0)];
        let mut stats = TraversalStats::default();

        let color = cast_ray(
            &rayo_al_origen(),
            &scene,
            &accel,
            &luces,
            &RevealState::painted(),
            Shading::Material,
            &mut stats,
        );

        assert!(
            brillo(color) > 0.8 * AMBIENT * 3.0 + 0.1,
            "un objeto detras de la luz no debe proyectar sombra: {color}"
        );
    }

    #[test]
    fn el_agua_con_ignore_no_proyecta_sombra_pero_el_monolito_si() {
        let luces = [luz_cenital(10.0)];

        let (con_agua, accel_agua) =
            suelo_con_bloqueador(ShadowMode::Ignore, SpatialGroupId::FlyingWaters);
        let iluminado = cast_ray(
            &rayo_al_origen(),
            &con_agua,
            &accel_agua,
            &luces,
            &RevealState::painted(),
            Shading::Material,
            &mut TraversalStats::default(),
        );

        let (con_monolito, accel_monolito) =
            suelo_con_bloqueador(ShadowMode::Opaque, SpatialGroupId::Monolith);
        let sombreado = cast_ray(
            &rayo_al_origen(),
            &con_monolito,
            &accel_monolito,
            &luces,
            &RevealState::painted(),
            Shading::Material,
            &mut TraversalStats::default(),
        );

        assert!(
            brillo(iluminado) > brillo(sombreado) + 0.1,
            "el agua no deberia sombrear: {iluminado} contra {sombreado}"
        );
    }

    #[test]
    fn l02_no_consulta_praderas_como_oclusor() {
        // Bloqueador opaco en Praderas, y una luz que solo admite
        // oclusores de Aguas Voladoras: la sombra no debe aparecer.
        let (scene, accel) = suelo_con_bloqueador(ShadowMode::Opaque, SpatialGroupId::Meadows);

        let confinada = PointLight {
            occluder_groups: GroupMask::only(&[SpatialGroupId::FlyingWaters]),
            ..luz_cenital(10.0)
        };
        let abierta = luz_cenital(10.0);

        let sin_sombra = cast_ray(
            &rayo_al_origen(),
            &scene,
            &accel,
            &[confinada],
            &RevealState::painted(),
            Shading::Material,
            &mut TraversalStats::default(),
        );
        let con_sombra = cast_ray(
            &rayo_al_origen(),
            &scene,
            &accel,
            &[abierta],
            &RevealState::painted(),
            Shading::Material,
            &mut TraversalStats::default(),
        );

        assert!(
            brillo(sin_sombra) > brillo(con_sombra) + 0.1,
            "Praderas no debe bloquear una luz que no la ilumina: {sin_sombra} contra {con_sombra}"
        );
    }

    #[test]
    fn una_luz_sin_sombras_no_lanza_rayos_de_sombra() {
        let (scene, accel) = suelo_con_bloqueador(ShadowMode::Opaque, SpatialGroupId::Monolith);

        let sin_sombras = PointLight {
            casts_shadows: false,
            ..luz_cenital(10.0)
        };

        let color = cast_ray(
            &rayo_al_origen(),
            &scene,
            &accel,
            &[sin_sombras],
            &RevealState::painted(),
            Shading::Material,
            &mut TraversalStats::default(),
        );

        assert!(
            brillo(color) > 0.8 * AMBIENT * 3.0 + 0.1,
            "una luz sin sombras ilumina aunque haya un opaco delante: {color}"
        );
    }

    /// Cielo de dos filas: claro arriba, oscuro abajo. Un panorama de
    /// `1 x 2` no tiene azimut, y eso es justo lo que hace medible la
    /// altura por separado.
    fn cielo_de_dos_franjas(scene: &mut Scene) -> (Color, Color) {
        let claro = Color::new(0.80, 0.82, 0.90);
        let oscuro = Color::new(0.04, 0.04, 0.07);

        // Las texturas no entran en la estructura de aceleracion, asi que
        // registrarlas despues de construirla no la invalida.
        let panorama = crate::texture::Texture::from_pixels(1, 2, vec![claro, oscuro])
            .expect("1x2 con dos pixeles");
        let id = scene.add_texture(panorama);

        scene.skybox = Skybox::Panorama {
            pale: id,
            painted: id,
        };

        (claro, oscuro)
    }

    #[test]
    fn un_rayo_perdido_devuelve_el_cielo_y_no_un_color_fijo() {
        let (mut scene, accel) = suelo();
        let (claro, oscuro) = cielo_de_dos_franjas(&mut scene);

        // Rayo que se va hacia el cenit, muy por encima del suelo.
        let hacia_arriba = Ray::new(Vec3::new(0.0, 50.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        // Y otro que cae hacia el nadir, pero fuera del suelo de 20 x 20.
        let hacia_abajo = Ray::new(Vec3::new(0.0, 50.0, 100.0), Vec3::new(0.0, -1.0, 0.0));

        let arriba = cast_ray(
            &hacia_arriba,
            &scene,
            &accel,
            &[],
            &RevealState::unpainted(),
            Shading::Material,
            &mut TraversalStats::default(),
        );
        let abajo = cast_ray(
            &hacia_abajo,
            &scene,
            &accel,
            &[],
            &RevealState::unpainted(),
            Shading::Material,
            &mut TraversalStats::default(),
        );

        assert_eq!(arriba, claro, "el cenit no trajo el cielo");
        assert_eq!(abajo, oscuro, "el nadir no trajo el suelo del panorama");
        // Lo que exige el plan: el fondo depende de la direccion. Si el
        // miss siguiera devolviendo una constante, los dos serian iguales.
        assert_ne!(arriba, abajo);
        assert_ne!(arriba.to_hex(), BACKGROUND_COLOR);
    }

    #[test]
    fn el_cielo_del_miss_avanza_con_la_revelacion() {
        let (mut scene, accel) = suelo();

        let sin_pintar = Color::new(0.9, 0.9, 0.85);
        let pintado = Color::new(0.05, 0.08, 0.20);
        let pale = scene.add_texture(
            crate::texture::Texture::from_pixels(1, 1, vec![sin_pintar]).expect("1x1"),
        );
        let painted = scene
            .add_texture(crate::texture::Texture::from_pixels(1, 1, vec![pintado]).expect("1x1"));
        scene.skybox = Skybox::Panorama { pale, painted };

        let perdido = Ray::new(Vec3::new(0.0, 50.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let cielo = |reveal: &RevealState| {
            cast_ray(
                &perdido,
                &scene,
                &accel,
                &[],
                reveal,
                Shading::Material,
                &mut TraversalStats::default(),
            )
        };

        assert_eq!(cielo(&RevealState::unpainted()), sin_pintar);
        assert_eq!(cielo(&RevealState::painted()), pintado);

        // A medio camino, entre los dos y distinto de ambos.
        let mut medio = RevealState::unpainted();
        for grupo in [
            RevealGroup::Meadows,
            RevealGroup::Breakwater,
            RevealGroup::FlyingWaters,
            RevealGroup::Finale,
        ] {
            medio.set_progress(grupo, 0.5);
        }
        let a_medias = cielo(&medio);

        assert!((a_medias.r - 0.475).abs() < 1e-5, "{a_medias}");
    }

    #[test]
    fn el_render_devuelve_contadores_no_vacios() {
        use crate::framebuffer::Framebuffer;

        let (scene, accel) = suelo();
        let camera = Camera::new(
            Vec3::new(0.0, 6.0, 6.0),
            Vec3::zeros(),
            Vec3::zeros(),
            Vec3::new(0.0, 1.0, 0.0),
            crate::camera::DEFAULT_VERTICAL_FOV,
        );
        let mut framebuffer = Framebuffer::new(16, 12);

        let stats = render(
            &mut framebuffer,
            &scene,
            &accel,
            &[luz_cenital(10.0)],
            &RevealState::painted(),
            &camera,
            Shading::Material,
        );

        assert_eq!(stats.primary_rays, 16 * 12);
        assert!(stats.shadow_rays > 0, "la luz proyecta sombras");
        assert!(stats.primitive_tests > 0);
        assert!(stats.group_bounds_tests > 0);
    }

    // ------------------------------------------------ recursion limitada

    /// Espejo horizontal en `y = 0`, con albedo negro para que lo único que
    /// aporte sea el reflejo, y un cielo de dos franjas.
    ///
    /// `ior = 20` no describe ningún material real: es lo que hace que
    /// Schlick dé `R0 = 0.82` y el espejo refleje de verdad también de
    /// frente. Con `ior = 1.0` la reflectancia sería cero a incidencia
    /// perpendicular —ver `optics::con_ior_uno_schlick_degenera_y_hay_que_saberlo`—
    /// y no habría espejo que probar.
    fn espejo_con_cielo() -> (Scene, SceneAccel, Color) {
        let mut scene = Scene::new();
        let espejo = scene.add_material(Material::new(Color::black()).with_caps(1.0, 0.0, 20.0));

        scene.add_object(SceneObject {
            primitive: Cuboid::centrado(Vec3::new(0.0, -0.5, 0.0), Vec3::new(40.0, 1.0, 40.0))
                .into(),
            initial_material: espejo,
            final_material: espejo,
            spatial_group: SpatialGroupId::Global,
            reveal_group: RevealGroup::Finale,
        });

        let (claro, _) = cielo_de_dos_franjas(&mut scene);
        let accel = SceneAccel::build(&scene).expect("hay geometria");

        (scene, accel, claro)
    }

    /// Losa de agua de `y = -1` a `y = 1`, con un objeto rojo dentro.
    ///
    /// Con `transparente` en falso el mismo material queda con los techos
    /// en cero: es el control que aísla la refracción del resto.
    fn agua_con_objeto_dentro(transparente: bool) -> (Scene, SceneAccel) {
        let mut scene = Scene::new();

        let base = Material::new(Color::new(0.0, 0.2, 0.5));
        let agua = scene.add_material(if transparente {
            base.with_caps(0.9, 0.9, 1.333)
                .with_shadow_mode(ShadowMode::Ignore)
        } else {
            base
        });
        let rojo = scene.add_material(Material::new(Color::new(1.0, 0.0, 0.0)));

        scene.add_object(SceneObject {
            primitive: Cuboid::centrado(Vec3::zeros(), Vec3::new(10.0, 2.0, 10.0)).into(),
            initial_material: agua,
            final_material: agua,
            spatial_group: SpatialGroupId::FlyingWaters,
            reveal_group: RevealGroup::FlyingWaters,
        });
        scene.add_object(SceneObject {
            primitive: Cuboid::centrado(Vec3::new(0.0, -0.3, 0.0), Vec3::new(2.0, 0.6, 2.0)).into(),
            initial_material: rojo,
            final_material: rojo,
            spatial_group: SpatialGroupId::FlyingWaters,
            reveal_group: RevealGroup::FlyingWaters,
        });

        scene.skybox = Skybox::Flat(Color::black());
        let accel = SceneAccel::build(&scene).expect("hay geometria");

        (scene, accel)
    }

    /// Volumen cerrado de agua con un lecho **detrás**, y cielo magenta.
    ///
    /// Es la geometría de la que sale la decisión `max_depth = 3`: cruzar el
    /// volumen cuesta dos niveles —cara frontal y cara interna trasera— y el
    /// lecho necesita el tercero.
    fn volumen_con_lecho() -> (Scene, SceneAccel, Color) {
        let mut scene = Scene::new();

        let agua = scene.add_material(
            Material::new(Color::new(0.0, 0.2, 0.5))
                .with_caps(0.9, 0.9, 1.333)
                .with_shadow_mode(ShadowMode::Ignore),
        );
        let lecho = scene.add_material(Material::new(Color::new(0.0, 1.0, 0.0)));

        scene.add_object(SceneObject {
            primitive: Cuboid::centrado(Vec3::zeros(), Vec3::new(4.0, 4.0, 4.0)).into(),
            initial_material: agua,
            final_material: agua,
            spatial_group: SpatialGroupId::FlyingWaters,
            reveal_group: RevealGroup::FlyingWaters,
        });
        scene.add_object(SceneObject {
            primitive: Cuboid::centrado(Vec3::new(0.0, 0.0, -6.0), Vec3::new(8.0, 8.0, 1.0)).into(),
            initial_material: lecho,
            final_material: lecho,
            spatial_group: SpatialGroupId::FlyingWaters,
            reveal_group: RevealGroup::FlyingWaters,
        });

        let cielo = Color::new(0.6, 0.0, 0.6);
        scene.skybox = Skybox::Flat(cielo);
        let accel = SceneAccel::build(&scene).expect("hay geometria");

        (scene, accel, cielo)
    }

    /// Rayo que entra de frente al volumen, por el eje `-Z`.
    fn rayo_frontal() -> Ray {
        Ray::new(Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 0.0, -1.0))
    }

    fn trazar(
        ray: &Ray,
        scene: &Scene,
        accel: &SceneAccel,
        lights: &[PointLight],
        depth: u32,
        stats: &mut TraversalStats,
    ) -> Color {
        cast_ray_depth(
            ray,
            scene,
            accel,
            lights,
            &RevealState::painted(),
            Shading::Material,
            depth,
            stats,
        )
    }

    #[test]
    fn con_profundidad_cero_no_recurre_ni_traza() {
        let (scene, accel) = suelo();
        let mut stats = TraversalStats::default();

        let color = trazar(&rayo_al_origen(), &scene, &accel, &[], 0, &mut stats);

        // El presupuesto agotado devuelve cielo, no negro y no el suelo.
        assert_eq!(color, Color::from_hex(crate::skybox::FALLBACK_COLOR));
        // Y no llega a probar una sola primitiva.
        assert_eq!(stats.primitive_tests, 0, "trazo con presupuesto cero");
        assert_eq!(stats.reflection_rays, 0);
        assert_eq!(stats.refraction_rays, 0);
    }

    #[test]
    fn un_espejo_simple_refleja_el_cielo() {
        let (scene, accel, cielo) = espejo_con_cielo();
        let mut stats = TraversalStats::default();

        // Cae a 45 grados: el reflejado sube y muestrea la franja de arriba.
        let ray = Ray::new(
            Vec3::new(0.0, 4.0, 4.0),
            Vec3::new(0.0, -1.0, -1.0).normalize(),
        );
        let color = trazar(&ray, &scene, &accel, &[], MAX_DEPTH, &mut stats);

        // Schlick a 45 grados con ior 20 da F = 0.818985, y el techo de
        // reflexion es uno: el espejo devuelve ese porcentaje del cielo.
        let f = 0.818_985;

        assert!(
            (color.r - f * cielo.r).abs() < 2e-3,
            "el espejo dio {color} y el cielo es {cielo}"
        );
        assert!((color.g - f * cielo.g).abs() < 2e-3);
        assert!((color.b - f * cielo.b).abs() < 2e-3);

        // Un rayo reflejado, ninguno refractado: el techo de transmision
        // del espejo es cero.
        assert_eq!(stats.reflection_rays, 1);
        assert_eq!(stats.refraction_rays, 0);
    }

    #[test]
    fn el_agua_deja_ver_el_objeto_de_adentro() {
        let rayo = Ray::new(Vec3::new(0.0, 5.0, 0.0), Vec3::new(0.0, -1.0, 0.0));

        let (transparente, accel_t) = agua_con_objeto_dentro(true);
        let mut stats_t = TraversalStats::default();
        let a_traves = trazar(&rayo, &transparente, &accel_t, &[], MAX_DEPTH, &mut stats_t);

        let (opaca, accel_o) = agua_con_objeto_dentro(false);
        let mut stats_o = TraversalStats::default();
        let tapado = trazar(&rayo, &opaca, &accel_o, &[], MAX_DEPTH, &mut stats_o);

        // El objeto es rojo y el agua no tiene rojo: el canal rojo solo
        // puede venir de haber atravesado la superficie.
        assert!(a_traves.r > 0.04, "el rojo de adentro no llego: {a_traves}");
        assert!(
            tapado.r < 1e-6,
            "con techos en cero no deberia pasar nada: {tapado}"
        );

        assert_eq!(stats_o.refraction_rays, 0, "el control no debe refractar");
        assert!(stats_t.refraction_rays > 0, "el agua tiene que refractar");
    }

    #[test]
    fn con_profundidad_tres_el_rayo_cruza_el_volumen_y_alcanza_el_lecho() {
        let (scene, accel, _) = volumen_con_lecho();
        let mut stats = TraversalStats::default();

        let color = trazar(&rayo_frontal(), &scene, &accel, &[], 3, &mut stats);

        // El lecho es verde y ni el agua ni el cielo lo son: el verde solo
        // puede venir de haber cruzado el volumen completo.
        assert!(
            color.g > 0.02,
            "el lecho no se alcanzo con profundidad 3: {color}"
        );

        // Dos refracciones para cruzar: cara frontal y cara interna trasera.
        assert!(
            stats.refraction_rays >= 2,
            "solo {} refracciones",
            stats.refraction_rays
        );
    }

    #[test]
    fn con_profundidad_dos_el_rayo_termina_en_cielo_y_no_en_negro() {
        let (scene, accel, cielo) = volumen_con_lecho();

        let mut stats_tres = TraversalStats::default();
        let con_tres = trazar(&rayo_frontal(), &scene, &accel, &[], 3, &mut stats_tres);

        let mut stats_dos = TraversalStats::default();
        let con_dos = trazar(&rayo_frontal(), &scene, &accel, &[], 2, &mut stats_dos);

        // Lo que se pierde al bajar la profundidad: el lecho.
        assert!(
            con_dos.g < 0.006,
            "con profundidad 2 el lecho no deberia verse: {con_dos}"
        );
        assert!(
            con_tres.g > con_dos.g + 0.02,
            "bajar la profundidad tenia que perder el lecho: {con_tres} contra {con_dos}"
        );

        // Y lo que **no** pasa: quedarse en negro. El terminal es cielo.
        assert!(
            con_dos.r > 0.1 && con_dos.b > 0.1,
            "el terminal salio oscuro en vez de cielo: {con_dos}"
        );
        assert!(
            brillo(con_dos) > 0.2,
            "manchas oscuras dentro del agua: {con_dos}"
        );
        // El cielo es magenta, asi que el rojo y el azul tienen que
        // dominar sobre el verde.
        assert!(cielo.g < cielo.r, "el cielo de la prueba no es magenta");
        assert!(con_dos.r > con_dos.g * 10.0);
    }

    #[test]
    fn el_resultado_siempre_es_finito_y_acotado() {
        // El paso 6 del orden. Un `NaN` o un desborde en el reparto se
        // convertiria en un pixel de un color arbitrario, y eso es de las
        // cosas mas difíciles de rastrear mirando una imagen.
        let (scene, accel, _) = volumen_con_lecho();
        let luces = [luz_cenital(8.0)];

        for i in 0..9 {
            for j in 0..9 {
                let x = (i as f32 / 4.0) - 1.0;
                let y = (j as f32 / 4.0) - 1.0;
                let direccion = Vec3::new(x, y, -1.0).normalize();
                let ray = Ray::new(Vec3::new(0.0, 0.0, 10.0), direccion);

                let color = trazar(
                    &ray,
                    &scene,
                    &accel,
                    &luces,
                    MAX_DEPTH,
                    &mut TraversalStats::default(),
                );

                for (canal, nombre) in [(color.r, "r"), (color.g, "g"), (color.b, "b")] {
                    assert!(
                        canal.is_finite(),
                        "canal {nombre} no finito en ({x}, {y}): {color}"
                    );
                    assert!(
                        (0.0..=1.0).contains(&canal),
                        "canal {nombre} fuera de rango en ({x}, {y}): {color}"
                    );
                }
            }
        }
    }

    #[test]
    fn el_highlight_del_agua_no_se_escala_por_kl() {
        // Con los caps 0.9/0.9 lo local queda al diez por ciento. Si el
        // specular entrara en ese reparto, el highlight del agua se veria
        // diez veces mas debil y el gate de Aguas Voladoras no pasaria.
        // El plan lo dice explicito: el specular directo se suma **despues**
        // del reparto de Fresnel.
        let fuerza = 0.6;
        let escena = |strength: f32| {
            let mut scene = Scene::new();
            let agua = scene.add_material(
                Material::new(Color::new(0.0, 0.2, 0.5))
                    .with_caps(0.9, 0.9, 1.333)
                    .with_specular(strength, 32.0)
                    .with_shadow_mode(ShadowMode::Ignore),
            );

            // Volumen deliberadamente profundo: la cara superior queda en
            // `y = 1` y la de salida 200 unidades mas abajo.
            //
            // La razon es que el rayo refractado impacta esa cara interna
            // de salida, y **esa tambien** recibe su propio highlight,
            // pesado por `kt`. Para medir el de una sola superficie hay que
            // alejarla: con la luz a `range = 1.0`, su atenuacion a 204
            // unidades cae a `2.4e-5` y su aporte queda por debajo de la
            // tolerancia. Ojo: `range` es la distancia de media
            // contribucion, no un corte, asi que alejar sin bajar `range`
            // no alcanza.
            scene.add_object(SceneObject {
                primitive: Cuboid::centrado(
                    Vec3::new(0.0, -99.0, 0.0),
                    Vec3::new(10.0, 200.0, 10.0),
                )
                .into(),
                initial_material: agua,
                final_material: agua,
                spatial_group: SpatialGroupId::FlyingWaters,
                reveal_group: RevealGroup::FlyingWaters,
            });
            scene.skybox = Skybox::Flat(Color::black());

            let accel = SceneAccel::build(&scene).expect("hay geometria");

            (scene, accel)
        };

        // Rayo y luz en la vertical: el vector medio coincide con la normal
        // y el brillo de Blinn-Phong vale uno exacto.
        let rayo = Ray::new(Vec3::new(0.0, 5.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        let luz = PointLight {
            range: 1.0,
            ..luz_cenital(5.0)
        };

        let (con_brillo, accel_con) = escena(fuerza);
        let con = trazar(
            &rayo,
            &con_brillo,
            &accel_con,
            &[luz],
            MAX_DEPTH,
            &mut TraversalStats::default(),
        );

        let (sin_brillo, accel_sin) = escena(0.0);
        let sin = trazar(
            &rayo,
            &sin_brillo,
            &accel_sin,
            &[luz],
            MAX_DEPTH,
            &mut TraversalStats::default(),
        );

        // La luz esta a 4 unidades de la superficie, que esta en y = 1.
        let esperado = luz.color * luz.attenuation(4.0) * fuerza;
        let delta = brillo(con) - brillo(sin);

        assert!(
            (delta - brillo(esperado)).abs() < 1e-4,
            "el highlight aporto {delta} y el especular completo es {}",
            brillo(esperado)
        );
        // Y la comprobacion que da nombre al test: escalado por kl seria
        // una decima parte.
        assert!(
            delta > 5.0 * 0.1 * brillo(esperado),
            "el highlight quedo escalado por kl: {delta}"
        );
    }

    #[test]
    fn un_material_opaco_no_lanza_secundarios() {
        // Cuatro de los cinco materiales finales son opacos. El reparto no
        // debe gastarles un rayo, ni siquiera uno que se descarte despues.
        let (scene, accel) = suelo();
        let mut stats = TraversalStats::default();

        trazar(
            &rayo_al_origen(),
            &scene,
            &accel,
            &[luz_cenital(10.0)],
            MAX_DEPTH,
            &mut stats,
        );

        assert_eq!(stats.reflection_rays, 0);
        assert_eq!(stats.refraction_rays, 0);
    }

    #[test]
    fn la_profundidad_inicial_es_tres() {
        // La decision cerrada del plan, amarrada: si alguien la baja a dos
        // sin registrar qué se pierde, esto lo detiene.
        assert_eq!(MAX_DEPTH, 3);
    }

    // ------------------------------------------------- dirty rendering

    #[test]
    fn la_tabla_del_plan_de_cuadro_tiene_tres_filas() {
        assert_eq!(plan_frame(true, true), FramePlan::Interactive);
        assert_eq!(plan_frame(true, false), FramePlan::Interactive);
        assert_eq!(plan_frame(false, true), FramePlan::Final);
        assert_eq!(plan_frame(false, false), FramePlan::Reuse);
    }

    #[test]
    fn en_reposo_no_se_traza_nada() {
        // Es la reutilizacion del framebuffer que pide el plan: cuando nada
        // cambia, el cuadro no se recalcula.
        let plan = plan_frame(false, false);

        assert!(!plan.draws());
        assert!(plan.should_sleep());
    }

    #[test]
    fn mientras_algo_se_mueve_se_traza_y_no_se_duerme() {
        let plan = plan_frame(true, false);

        assert!(plan.draws());
        assert!(
            !plan.should_sleep(),
            "dormir le quita cuadros a la animacion"
        );
    }

    #[test]
    fn el_cuadro_final_se_traza_una_sola_vez() {
        // Se dibuja al detenerse, y el siguiente cuadro ya reutiliza. La
        // secuencia completa de una interaccion: dos cuadros moviendose,
        // uno final, y reposo.
        let mut pendiente = false;
        let mut planes = Vec::new();

        for sostenido in [true, true, false, false, false] {
            let plan = plan_frame(sostenido, pendiente);

            pendiente = match plan {
                FramePlan::Interactive => true,
                FramePlan::Final => false,
                FramePlan::Reuse => pendiente,
            };

            planes.push(plan);
        }

        assert_eq!(
            planes,
            vec![
                FramePlan::Interactive,
                FramePlan::Interactive,
                FramePlan::Final,
                FramePlan::Reuse,
                FramePlan::Reuse,
            ]
        );
    }

    #[test]
    fn un_cambio_instantaneo_va_directo_al_cuadro_final() {
        // Un reinicio al lienzo no es sostenido: no tiene sentido gastarle
        // un cuadro de baja resolucion cuando ya termino.
        let plan = plan_frame(false, true);

        assert_eq!(plan, FramePlan::Final);
        assert!(plan.draws());
        assert!(!plan.should_sleep());
    }
}
