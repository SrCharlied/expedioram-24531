//! Render headless del gate del Hito 1.
//!
//! No abre ventana: renderiza un framebuffer diminuto y comprueba que el
//! resultado sea una imagen y no basura, incluido el PNG en disco.

use expedition33_continente_inacabado::camera::{Camera, DEFAULT_VERTICAL_FOV};
use expedition33_continente_inacabado::color::Color;
use expedition33_continente_inacabado::cuboid::Cuboid;
use expedition33_continente_inacabado::framebuffer::Framebuffer;
use expedition33_continente_inacabado::material::Material;
use expedition33_continente_inacabado::ray::Ray;
use expedition33_continente_inacabado::renderer::{cast_ray, render, Shading, BACKGROUND_COLOR};
use expedition33_continente_inacabado::scene::{
    MaterialId, RevealGroup, Scene, SceneObject, SpatialGroupId,
};
use nalgebra_glm::Vec3;

const ANCHO: usize = 32;
const ALTO: usize = 24;

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
    let (scene, _) = escena_de_un_cubo();
    let mut framebuffer = Framebuffer::new(ANCHO, ALTO);

    render(
        &mut framebuffer,
        &scene,
        &[],
        &camara_hero(),
        Shading::Normals,
    );

    assert_eq!(framebuffer.buffer.len(), ANCHO * ALTO);
}

#[test]
fn al_menos_un_pixel_no_es_el_fondo() {
    let (scene, _) = escena_de_un_cubo();
    let mut framebuffer = Framebuffer::new(ANCHO, ALTO);

    render(
        &mut framebuffer,
        &scene,
        &[],
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
    let (scene, _) = escena_de_un_cubo();
    let camera = camara_hero();

    // Se comprueba sobre el Color y no sobre el framebuffer: al empacar a
    // u32 un NaN ya se habria convertido en un entero cualquiera y el
    // defecto pasaria inadvertido.
    for y in 0..ALTO {
        for x in 0..ANCHO {
            // Misma generacion de rayo que usa el render, no una copia.
            let ray = camera.ray_from_pixel(x, y, ANCHO, ALTO);

            for shading in [Shading::Normals, Shading::Albedo, Shading::Material] {
                let color = cast_ray(&ray, &scene, &[], shading);

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
    let (scene, piedra) = escena_de_un_cubo();
    let camera = camara_hero();

    // Rayo al centro del encuadre: pega de lleno en la cara frontal.
    let ray = Ray::new(camera.eye, Vec3::new(0.0, 0.0, -1.0));
    let color = cast_ray(&ray, &scene, &[], Shading::Albedo);

    assert_eq!(color, scene.material(piedra).albedo);
}

#[test]
fn el_fondo_se_devuelve_cuando_el_rayo_no_toca_nada() {
    let (scene, _) = escena_de_un_cubo();

    // Rayo que se aleja del cubo.
    let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 1.0, 0.0));
    let color = cast_ray(&ray, &scene, &[], Shading::Albedo);

    assert_eq!(color.to_hex(), BACKGROUND_COLOR);
}

#[test]
fn guarda_un_png_valido_y_decodificable() {
    let (scene, _) = escena_de_un_cubo();
    let mut framebuffer = Framebuffer::new(ANCHO, ALTO);

    render(
        &mut framebuffer,
        &scene,
        &[],
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
