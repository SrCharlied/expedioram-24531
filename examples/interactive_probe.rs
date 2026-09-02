//! Compara el cuadro interactivo escalado contra el cuadro final, para
//! comprobar que el escalado degrada detalle pero no rompe la imagen.
use expedition33_continente_inacabado::framebuffer::Framebuffer;
use expedition33_continente_inacabado::light::diorama;
use expedition33_continente_inacabado::renderer::{render, InteractiveProfile, Shading};
use expedition33_continente_inacabado::reveal::RevealState;
use expedition33_continente_inacabado::scenes::{safe_level, WaterPreset};
use std::time::Instant;

const ANCHO: usize = 800;
const ALTO: usize = 600;

fn main() {
    let nivel = safe_level(WaterPreset::InteriorVisible);
    let camera = nivel.hero_camera();
    let lights = diorama(&nivel.anchors, &nivel.scale);

    let mut completo = Framebuffer::new(ANCHO, ALTO);
    let inicio = Instant::now();
    render(
        &mut completo,
        &nivel.scene,
        &nivel.accel,
        &lights,
        &RevealState::painted(),
        &camera,
        Shading::Material,
    );
    let t_completo = inicio.elapsed().as_secs_f64();

    for (nombre, perfil) in [
        ("MEDIA", InteractiveProfile::MEDIA),
        ("BAJA", InteractiveProfile::BAJA),
    ] {
        let mut borrador = Framebuffer::new(perfil.width, perfil.height);
        let inicio = Instant::now();
        render(
            &mut borrador,
            &nivel.scene,
            &nivel.accel,
            &lights,
            &RevealState::painted(),
            &camera,
            Shading::Material,
        );
        let t = inicio.elapsed().as_secs_f64();

        let mut escalado = Framebuffer::new(ANCHO, ALTO);
        let inicio_blit = Instant::now();
        escalado.blit_upscaled(&borrador);
        let t_blit = inicio_blit.elapsed().as_secs_f64();

        // Cuantos pixeles difieren del cuadro final.
        let distintos = escalado
            .buffer
            .iter()
            .zip(&completo.buffer)
            .filter(|(a, b)| a != b)
            .count();

        println!(
            "  {:<6} {}x{}  trazado {:.4} s + escalado {:.4} s = {:.4} s  ({:.1}x mas rapido)",
            nombre,
            perfil.width,
            perfil.height,
            t,
            t_blit,
            t + t_blit,
            t_completo / (t + t_blit)
        );
        println!(
            "         pixeles distintos del cuadro final: {:.1}%",
            100.0 * distintos as f64 / (ANCHO * ALTO) as f64
        );

        escalado
            .save_png(std::path::Path::new(&format!(
                "{}/interactivo_{}.png",
                std::env::args().nth(1).unwrap_or_else(|| ".".to_string()),
                nombre.to_lowercase()
            )))
            .expect("guardar");
    }

    println!("  {:<6} {ANCHO}x{ALTO}  {:.4} s", "FINAL", t_completo);
}
