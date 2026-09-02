//! Render headless del gate del Hito 1.
//!
//! No abre ventana: renderiza un framebuffer diminuto y comprueba que el
//! resultado sea una imagen y no basura, incluido el PNG en disco.

use expedition33_continente_inacabado::accel::{SceneAccel, TraversalStats};
use expedition33_continente_inacabado::camera::{Camera, DEFAULT_VERTICAL_FOV};
use expedition33_continente_inacabado::color::Color;
use expedition33_continente_inacabado::cuboid::Cuboid;
use expedition33_continente_inacabado::framebuffer::Framebuffer;
use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
use expedition33_continente_inacabado::material::Material;
use expedition33_continente_inacabado::ray::Ray;
use expedition33_continente_inacabado::renderer::{cast_ray, render, Shading, BACKGROUND_COLOR};
use expedition33_continente_inacabado::reveal::RevealState;
use expedition33_continente_inacabado::scene::{
    MaterialId, RevealGroup, Scene, SceneObject, SpatialGroupId,
};
use expedition33_continente_inacabado::scenes::{safe_level, WaterPreset};
use nalgebra_glm::Vec3;

const ANCHO: usize = 32;
const ALTO: usize = 24;

fn escena_y_accel() -> (Scene, SceneAccel, MaterialId) {
    let (scene, material) = escena_de_un_cubo();
    let accel = SceneAccel::build(&scene).expect("hay geometria");

    (scene, accel, material)
}

fn escena_de_un_cubo() -> (Scene, MaterialId) {
    let mut scene = Scene::new();
    let piedra = scene.add_material(Material::new(Color::new(0.62, 0.60, 0.55)));

    scene.add_object(SceneObject {
        primitive: Cuboid::centrado(Vec3::zeros(), Vec3::new(2.0, 2.0, 2.0)).into(),
        initial_material: piedra,
        final_material: piedra,
        spatial_group: SpatialGroupId::Monolith,
        reveal_group: RevealGroup::Finale,
    });

    (scene, piedra)
}

fn camara_hero() -> Camera {
    Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::zeros(),
        Vec3::zeros(),
        Vec3::new(0.0, 1.0, 0.0),
        DEFAULT_VERTICAL_FOV,
    )
}

#[test]
fn render_pequeno_termina_y_llena_el_framebuffer() {
    let (scene, accel, _) = escena_y_accel();
    let mut framebuffer = Framebuffer::new(ANCHO, ALTO);

    render(
        &mut framebuffer,
        &scene,
        &accel,
        &[],
        &RevealState::painted(),
        &camara_hero(),
        Shading::Normals,
    );

    assert_eq!(framebuffer.buffer.len(), ANCHO * ALTO);
}

#[test]
fn al_menos_un_pixel_no_es_el_fondo() {
    let (scene, accel, _) = escena_y_accel();
    let mut framebuffer = Framebuffer::new(ANCHO, ALTO);

    render(
        &mut framebuffer,
        &scene,
        &accel,
        &[],
        &RevealState::painted(),
        &camara_hero(),
        Shading::Normals,
    );

    let del_cubo = framebuffer
        .buffer
        .iter()
        .filter(|pixel| **pixel != BACKGROUND_COLOR)
        .count();

    assert!(del_cubo > 0, "el cubo no aparecio en la imagen");

    // El cubo ocupa buena parte del encuadre, pero no todo: si cubriera el
    // 100% seria senal de que el fondo dejo de escribirse.
    assert!(
        del_cubo < ANCHO * ALTO,
        "el cubo cubrio el frame entero: {del_cubo} pixeles"
    );
}

#[test]
fn ningun_pixel_produce_nan() {
    let (scene, accel, _) = escena_y_accel();
    let camera = camara_hero();

    // Se comprueba sobre el Color y no sobre el framebuffer: al empacar a
    // u32 un NaN ya se habria convertido en un entero cualquiera y el
    // defecto pasaria inadvertido.
    for y in 0..ALTO {
        for x in 0..ANCHO {
            // Misma generacion de rayo que usa el render, no una copia.
            let ray = camera.ray_from_pixel(x, y, ANCHO, ALTO);

            for shading in [Shading::Normals, Shading::Albedo, Shading::Material] {
                let color = cast_ray(
                    &ray,
                    &scene,
                    &accel,
                    &[],
                    &RevealState::painted(),
                    shading,
                    &mut TraversalStats::default(),
                );

                assert!(
                    color.r.is_finite() && color.g.is_finite() && color.b.is_finite(),
                    "color no finito en ({x}, {y}): {color}"
                );
            }
        }
    }
}

#[test]
fn el_sombreado_por_albedo_resuelve_la_paleta() {
    let (scene, accel, piedra) = escena_y_accel();
    let camera = camara_hero();

    // Rayo al centro del encuadre: pega de lleno en la cara frontal.
    let ray = Ray::new(camera.eye, Vec3::new(0.0, 0.0, -1.0));
    let color = cast_ray(
        &ray,
        &scene,
        &accel,
        &[],
        &RevealState::painted(),
        Shading::Albedo,
        &mut TraversalStats::default(),
    );

    assert_eq!(color, scene.material(piedra).albedo);
}

#[test]
fn el_fondo_se_devuelve_cuando_el_rayo_no_toca_nada() {
    let (scene, accel, _) = escena_y_accel();

    // Rayo que se aleja del cubo.
    let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 1.0, 0.0));
    let color = cast_ray(
        &ray,
        &scene,
        &accel,
        &[],
        &RevealState::painted(),
        Shading::Albedo,
        &mut TraversalStats::default(),
    );

    assert_eq!(color.to_hex(), BACKGROUND_COLOR);
}

#[test]
fn guarda_un_png_valido_y_decodificable() {
    let (scene, accel, _) = escena_y_accel();
    let mut framebuffer = Framebuffer::new(ANCHO, ALTO);

    render(
        &mut framebuffer,
        &scene,
        &accel,
        &[],
        &RevealState::painted(),
        &camara_hero(),
        Shading::Normals,
    );

    // Directorio propio por ejecucion: los tests corren en paralelo y dos
    // que escriban el mismo archivo se pisarian.
    let destino = std::env::temp_dir()
        .join(format!("continente-smoke-{}", std::process::id()))
        .join("anidado")
        .join("hero.png");

    framebuffer
        .save_png(&destino)
        .expect("save_png deberia crear los directorios que falten");

    let bytes = std::fs::read(&destino).expect("el archivo deberia existir");
    // Firma PNG por valor numerico, sin depender de escapes.
    const FIRMA_PNG: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
    assert_eq!(&bytes[..8], &FIRMA_PNG, "la firma no es la de un PNG");

    // Decodificarlo de verdad: la firma sola no prueba que el resto sirva.
    let imagen = image::open(&destino).expect("el PNG deberia decodificar");
    assert_eq!(imagen.width(), ANCHO as u32);
    assert_eq!(imagen.height(), ALTO as u32);

    // Y el contenido debe coincidir con el framebuffer, no ser una imagen
    // en blanco que casualmente tiene el tamano correcto.
    let rgb = imagen.to_rgb8();
    let esperado = framebuffer.buffer[0];
    let pixel = rgb.get_pixel(0, 0);
    let obtenido = ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | pixel[2] as u32;
    assert_eq!(obtenido, esperado, "el primer pixel no sobrevivio al PNG");

    let _ = std::fs::remove_file(&destino);
}

/// Cuántos píxeles difieren entre dos framebuffers del mismo tamaño.
fn pixeles_distintos(a: &Framebuffer, b: &Framebuffer) -> usize {
    a.buffer
        .iter()
        .zip(&b.buffer)
        .filter(|(uno, otro)| uno != otro)
        .count()
}

/// Render headless del nivel seguro con el preset dado.
fn render_del_nivel(water: WaterPreset) -> (Framebuffer, TraversalStats) {
    // Sin assets: los tests no dependen de que las texturas esten generadas.
    let nivel = safe_level(water);
    let luces = luces_del_diorama(&nivel.anchors, &nivel.scale);
    let mut framebuffer = Framebuffer::new(160, 120);

    let stats = render(
        &mut framebuffer,
        &nivel.scene,
        &nivel.accel,
        &luces,
        &RevealState::painted(),
        &nivel.hero_camera(),
        Shading::Material,
    );

    (framebuffer, stats)
}

#[test]
fn el_volumen_refractivo_cambia_la_imagen_frente_al_control_opaco() {
    // El test visual headless de la Tarea 5.4. La comprobacion no es «se
    // ve bonito» sino que la optica **llega al pixel**: el mismo volumen,
    // con y sin techos opticos, tiene que producir imagenes distintas.
    let (refractivo, stats_refractivo) = render_del_nivel(WaterPreset::RefractiveWater);
    let (opaco, stats_opaco) = render_del_nivel(WaterPreset::OpaqueWater);

    // Los dos presets insertan la misma geometria: 160 primitivas.
    assert_eq!(stats_refractivo.primary_rays, stats_opaco.primary_rays);

    // Los contadores son de la escena, no del agua: el cristal pictorico
    // tambien transmite —`transmission_cap = 0.25`— y el monolito ocupa
    // buena parte del cuadro. Lo que se compara es la **diferencia**, que
    // solo puede venir del volumen.
    // Medido: 1629 contra 932, o sea unos 700 rayos de mas, que es del
    // orden de los pixeles donde la superficie del agua se ve.
    assert!(
        stats_refractivo.refraction_rays >= stats_opaco.refraction_rays + 400,
        "el volumen apenas refracto: {} contra {} del control",
        stats_refractivo.refraction_rays,
        stats_opaco.refraction_rays
    );

    // Y la diferencia se ve. El umbral es deliberadamente bajo: la bahia
    // ocupa una fraccion del cuadro y el resto del diorama es identico en
    // los dos renders.
    let distintos = pixeles_distintos(&refractivo, &opaco);
    let total = 160 * 120;

    assert!(
        distintos > total / 100,
        "solo {distintos} de {total} pixeles cambiaron: la optica no llega al pixel"
    );
}

#[test]
fn quitar_el_volumen_no_deja_la_bahia_en_negro() {
    // El terminal de la recursion es cielo, no negro, y eso tiene que
    // sostenerse tambien en la escena real: ningun pixel del render debe
    // quedar completamente apagado.
    let (refractivo, _) = render_del_nivel(WaterPreset::RefractiveWater);

    let negros = refractivo
        .buffer
        .iter()
        .filter(|pixel| **pixel & 0x00FF_FFFF == 0)
        .count();

    assert_eq!(negros, 0, "{negros} pixeles salieron en negro absoluto");
}
