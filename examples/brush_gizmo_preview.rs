//! Preview reproducible del pincel visual sobre el diorama real.
//!
//! Traza el nivel seguro con la cámara hero, resuelve un punto de la
//! superficie con el mismo picking que usa la ventana y dibuja el gizmo
//! encima. Produce dos PNG: la vista completa y un recorte ampliado de la
//! herramienta.
//!
//! Existe porque el pincel solo se ve moviendo el ratón, y una decisión
//! visual no se toma leyendo código. Esto no prueba la interacción: prueba
//! cómo se ve el gizmo sobre la obra, de forma repetible.
//!
//! # Exige la feature
//!
//! ```bash
//! cargo run --release --features artistic-brush --example brush_gizmo_preview -- <carpeta>
//! ```
//!
//! Sin `artistic-brush` el ejemplo compila —para que `--all-targets` no se
//! rompa— y se limita a decir qué falta.

#[cfg(not(feature = "artistic-brush"))]
fn main() {
    eprintln!("brush_gizmo_preview exige la feature artistic-brush:");
    eprintln!(
        "  cargo run --release --features artistic-brush --example brush_gizmo_preview -- <carpeta>"
    );
}

#[cfg(feature = "artistic-brush")]
fn main() -> std::process::ExitCode {
    imp::correr()
}

#[cfg(feature = "artistic-brush")]
mod imp {
    use expedition33_continente_inacabado::brush_gizmo::draw_brush_gizmo;
    use expedition33_continente_inacabado::framebuffer::Framebuffer;
    use expedition33_continente_inacabado::input::{pick_artistic, PresentedFrame};
    use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
    use expedition33_continente_inacabado::renderer::{render, Shading};
    use expedition33_continente_inacabado::reveal::RevealState;
    use expedition33_continente_inacabado::scenes::{safe_level_con, WaterPreset};
    use std::path::{Path, PathBuf};
    use std::process::ExitCode;

    const ANCHO: usize = 800;
    const ALTO: usize = 600;

    /// Índice de `aged_wood`, el mismo que usa el binario de la ventana.
    const TEXTURA_DEL_MANGO: usize = 3;

    /// Lado del recorte ampliado, en píxeles de la imagen completa.
    const LADO_DEL_CLOSEUP: usize = 190;

    /// Cuánto se amplía el recorte.
    const AUMENTO: usize = 3;

    /// Los tres pinceles del preview: dónde apunta el cursor y de qué color
    /// salen las cerdas.
    ///
    /// Uno por fuente de color de la ventana: el dorado de revelar, un
    /// pigmento de la paleta y el tono que daría una tela.
    const MUESTRAS: [((f32, f32), u32, &str); 3] = [
        ((250.0, 385.0), 0x00E8C86A, "revelar"),
        ((350.0, 400.0), 0x00A8172B, "carmesi"),
        ((300.0, 370.0), 0x002E9EAE, "cian"),
    ];

    pub fn correr() -> ExitCode {
        let destino = match std::env::args().nth(1) {
            Some(ruta) => PathBuf::from(ruta),
            None => {
                eprintln!("uso: ... --example brush_gizmo_preview -- <carpeta de salida>");
                return ExitCode::FAILURE;
            }
        };

        let raiz = PathBuf::from(".");
        let diorama = match safe_level_con(WaterPreset::RefractiveWater, Some(&raiz)) {
            Ok(diorama) => diorama,
            Err(e) => {
                eprintln!("error: {e}");
                eprintln!("  genera los assets con: cargo run --release --bin generate_assets");
                return ExitCode::FAILURE;
            }
        };

        let lights = luces_del_diorama(&diorama.anchors, &diorama.scale);
        let camara = diorama.hero_camera();
        let radio_del_pincel = diorama.scale.scene_radius * 0.025;
        let scene = diorama.scene;
        let accel = diorama.accel;

        // El diorama revelado: se mira la herramienta sobre la obra
        // terminada, no sobre el lienzo.
        let reveal = RevealState::painted();
        let mut framebuffer = Framebuffer::new(ANCHO, ALTO);

        render(
            &mut framebuffer,
            &scene,
            &accel,
            &lights,
            &reveal,
            &camara,
            Shading::Material,
        );

        let cuadro = PresentedFrame::full(camara, (ANCHO, ALTO));
        let mut centro_del_closeup = None;

        for (cursor, cerdas, nombre) in MUESTRAS {
            let Some(objetivo) = pick_artistic(&scene, &accel, &cuadro, cursor) else {
                eprintln!("aviso: {nombre} en {cursor:?} no cayo sobre ninguna superficie");
                continue;
            };

            let dibujo = draw_brush_gizmo(
                &mut framebuffer,
                &camara,
                &objetivo.point,
                &objetivo.normal,
                radio_del_pincel,
                cerdas,
                scene.textures.get(TEXTURA_DEL_MANGO),
            );

            println!(
                "  {nombre:<8} cursor {cursor:?}  objeto {}  dibujado {dibujo}",
                objetivo.object_index
            );

            if centro_del_closeup.is_none() && dibujo {
                centro_del_closeup = Some(cursor);
            }
        }

        let completa = destino.join("gizmo-diorama.png");

        if let Err(e) = framebuffer.save_png(&completa) {
            eprintln!("error al escribir {}: {e}", completa.display());
            return ExitCode::FAILURE;
        }
        println!("completa  {}", completa.display());

        // El recorte ampliado: el mismo cuadro, alrededor del primer pincel
        // que llegó a dibujarse.
        let Some((cx, cy)) = centro_del_closeup else {
            eprintln!("aviso: ningun pincel se dibujo, no hay recorte que ampliar");
            return ExitCode::SUCCESS;
        };

        let closeup = destino.join("gizmo-closeup.png");
        let recorte = ampliar(&framebuffer, cx as usize, cy as usize);

        if let Err(e) = recorte.save_png(&closeup) {
            eprintln!("error al escribir {}: {e}", closeup.display());
            return ExitCode::FAILURE;
        }
        println!("closeup   {}", closeup.display());

        ExitCode::SUCCESS
    }

    /// Recorte cuadrado alrededor de un punto, ampliado por vecino más
    /// cercano.
    ///
    /// Vecino más cercano y no interpolado, por lo mismo que el resto del
    /// proyecto: lo que se quiere ver son las facetas, y un suavizado las
    /// disolvería justo donde hay que juzgarlas.
    fn ampliar(origen: &Framebuffer, cx: usize, cy: usize) -> Framebuffer {
        let medio = LADO_DEL_CLOSEUP / 2;
        let x0 = cx
            .saturating_sub(medio)
            .min(ANCHO.saturating_sub(LADO_DEL_CLOSEUP));
        let y0 = cy
            .saturating_sub(medio)
            .min(ALTO.saturating_sub(LADO_DEL_CLOSEUP));

        let lado = LADO_DEL_CLOSEUP * AUMENTO;
        let mut destino = Framebuffer::new(lado, lado);

        for y in 0..lado {
            for x in 0..lado {
                let fuente_x = x0 + x / AUMENTO;
                let fuente_y = y0 + y / AUMENTO;

                if fuente_x < ANCHO && fuente_y < ALTO {
                    destino.set_current_color(origen.buffer[fuente_y * ANCHO + fuente_x]);
                    destino.point(x, y);
                }
            }
        }

        destino
    }

    /// Un `Path` sin usar fuera de `correr`, para que el import no sobre.
    #[allow(dead_code)]
    fn _path(_: &Path) {}
}
