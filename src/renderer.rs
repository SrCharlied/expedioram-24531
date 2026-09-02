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
use crate::material::{direct_light, AMBIENT};
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
/// A `800 × 600` el nivel seguro tarda `0.0956 s` por cuadro —unos 10.5
/// fps—, y eso es latencia perceptible al orbitar. Los Hitos 4 a 6 hay que
/// poder probarlos de forma interactiva, así que mientras la cámara o la
/// revelación cambian se dibuja a menor resolución y se escala; al quedar
/// todo quieto se produce un cuadro final a resolución completa.
///
/// Las dos opciones salen de la medición de la Tarea 3.8, no de una
/// suposición.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InteractiveProfile {
    pub width: usize,
    pub height: usize,
}

impl InteractiveProfile {
    /// Media resolución: `0.0242 s` medidos, unos 41 fps. Es el punto de
    /// partida que fija el plan.
    pub const MEDIA: InteractiveProfile = InteractiveProfile {
        width: 400,
        height: 300,
    };

    /// Un paso más agresivo: `0.0157 s`, unos 64 fps. Reserva para cuando
    /// la escena crezca con texturas y óptica.
    pub const BAJA: InteractiveProfile = InteractiveProfile {
        width: 320,
        height: 240,
    };

    pub fn pixels(&self) -> usize {
        self.width * self.height
    }
}

impl Default for InteractiveProfile {
    fn default() -> Self {
        InteractiveProfile::MEDIA
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

/// Devuelve el color del objeto más cercano que toca el rayo.
///
/// El material se resuelve por `object_index` contra la paleta de la
/// escena; el impacto nunca lo carga. Por ahora usa `final_material`
/// directamente: la interpolación desde `canvas_unpainted` llega en la
/// Tarea 4.4, cuando exista `RevealState`.
pub fn cast_ray(
    ray: &Ray,
    scene: &Scene,
    accel: &SceneAccel,
    lights: &[PointLight],
    reveal: &RevealState,
    shading: Shading,
    stats: &mut TraversalStats,
) -> Color {
    let Some(hit) = accel.intersect(scene, ray, stats) else {
        // Un miss no devuelve un color fijo: devuelve el cielo en la
        // dirección del rayo. Es también el terminal que el plan reserva
        // para un rayo que agote `max_depth` en el Hito 5, así que la
        // profundidad agotada no necesitará un color propio.
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
            // iluminado quede en negro absoluto y pierda su silueta.
            let mut color = material.albedo * AMBIENT;

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

                color = color
                    + direct_light(
                        &material,
                        &hit.normal,
                        &direccion,
                        &hacia_ojo,
                        light.color,
                        atenuacion,
                    );
            }

            color
        }
    }
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
}
