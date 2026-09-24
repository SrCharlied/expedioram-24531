//! Evidencia reproducible de la versión artística vigente.
//!
//! Produce cuatro PNG que muestran, en orden, el lienzo actual, el modo
//! artístico sin nada pintado, el mismo con la fixture aplicada, y el pincel
//! en uso sobre un impacto real.
//!
//! # Exige la feature
//!
//! ```bash
//! cargo run --release --features artistic-brush --example artistic_evidence
//! ```
//!
//! Sin `artistic-brush` el ejemplo compila —para que `--all-targets` no se
//! rompa— y se limita a decir qué falta.
//!
//! Por defecto escribe en `evidence/hito9_artistic/`. Acepta un directorio
//! como argumento para poder comprobarlo fuera del repositorio antes de
//! tocar la carpeta de evidencia.

#[cfg(not(feature = "artistic-brush"))]
fn main() -> std::process::ExitCode {
    eprintln!("artistic_evidence exige la feature artistic-brush:");
    eprintln!(
        "  cargo run --release --features artistic-brush --example artistic_evidence [carpeta]"
    );

    std::process::ExitCode::FAILURE
}

#[cfg(feature = "artistic-brush")]
fn main() -> std::process::ExitCode {
    imp::correr()
}

#[cfg(feature = "artistic-brush")]
mod imp {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::ExitCode;

    use expedition33_continente_inacabado::brush::{BrushMasks, PigmentMasks, SurfaceKey};
    use expedition33_continente_inacabado::brush_gizmo::draw_brush_gizmo;
    use expedition33_continente_inacabado::camera::Camera;
    use expedition33_continente_inacabado::color::Color;
    use expedition33_continente_inacabado::framebuffer::Framebuffer;
    use expedition33_continente_inacabado::input::{pick_artistic, PaintHit, PresentedFrame};
    use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
    use expedition33_continente_inacabado::renderer::{render_artistic, Shading};
    use expedition33_continente_inacabado::reveal::RevealState;
    use expedition33_continente_inacabado::scene_builder::Blockout;
    use expedition33_continente_inacabado::scenes::{safe_level_con, WaterPreset};

    pub const ANCHO: usize = 800;
    pub const ALTO: usize = 600;

    /// Dónde se escriben los PNG si no se pasa otra cosa.
    const DESTINO_POR_DEFECTO: &str = "evidence/hito9_artistic";

    /// Resolución de las máscaras, la misma que usa la ventana.
    const BRUSH_RESOLUTION: usize = 128;

    /// Radio del pincel como fracción de `scene_radius`, el mismo con el que
    /// arranca la ventana.
    ///
    /// Se escribe aquí y no se importa: `main.rs` lo declara privado y tras
    /// una feature. Queda dicho para que se vea de dónde sale.
    const BRUSH_RADIUS_FACTOR: f32 = 0.025;

    /// Índice de `aged_wood` en `scene.textures`, el mango del pincel.
    const TEXTURA_DEL_MANGO: usize = 3;

    /// Rejilla de cursores con la que se construye la fixture.
    const REJILLA: (usize, usize) = (9, 7);

    /// Color de las cerdas del gizmo con la herramienta de revelar.
    const CERDAS_AL_REVELAR: u32 = 0x00E8C86A;

    /// Las dos capas artísticas y las superficies que tocan.
    pub struct Artistico {
        pub masks: BrushMasks,
        pub pigment: PigmentMasks,
        /// Las `SurfaceKey` pintadas, en el orden en que las encontró el
        /// picking. Ninguna está escrita a mano.
        pub claves: Vec<SurfaceKey>,
    }

    impl Artistico {
        pub fn vacio() -> Self {
            Artistico {
                masks: BrushMasks::new(BRUSH_RESOLUTION, BRUSH_RESOLUTION),
                pigment: PigmentMasks::new(BRUSH_RESOLUTION, BRUSH_RESOLUTION),
                claves: Vec::new(),
            }
        }
    }

    /// El nivel que se retrata: el seguro con el volumen refractivo, que es
    /// el preset canónico, con sus texturas cargadas desde la raíz.
    pub fn nivel() -> Blockout {
        match safe_level_con(WaterPreset::RefractiveWater, Some(&PathBuf::from("."))) {
            Ok(nivel) => nivel,
            Err(e) => {
                eprintln!("error: {e}");
                eprintln!("  la evidencia retrata la escena texturizada; sin assets seria otra.");
                eprintln!("  generalos con: cargo run --release --bin generate_assets");
                std::process::exit(1);
            }
        }
    }

    /// Los cursores de la rejilla, en píxeles de ventana.
    ///
    /// Fijos y derivados del tamaño: la evidencia tiene que salir igual en
    /// cada corrida o dejaría de ser reproducible.
    pub fn cursores(ancho: usize, alto: usize) -> Vec<(f32, f32)> {
        let (columnas, filas) = REJILLA;
        let mut salida = Vec::with_capacity(columnas * filas);

        for fila in 0..filas {
            for columna in 0..columnas {
                salida.push((
                    (columna as f32 + 0.5) * ancho as f32 / columnas as f32,
                    (fila as f32 + 0.5) * alto as f32 / filas as f32,
                ));
            }
        }

        salida
    }

    /// Pinta la fixture sobre superficies **reales** del diorama.
    ///
    /// Recorre la rejilla con el mismo `pick_artistic` que usa la ventana, y
    /// sobre cada impacto aplica revelado local y pigmento. Las claves salen
    /// del picking: ninguna se construye a mano, así que la evidencia
    /// retrata las superficies que de verdad se ven desde esa cámara.
    ///
    /// El radio en `uv` se deriva de la métrica de cada superficie, igual que
    /// en la ventana, así que la huella mide lo mismo sobre una losa enorme y
    /// sobre un tablón.
    ///
    /// Las texturas se **leen**: `stamp_texture_ellipse` muestrea y estampa
    /// el color en la capa. Ninguna textura de la escena se modifica.
    pub fn fixture_artistica(
        diorama: &Blockout,
        camara: &Camera,
        ancho: usize,
        alto: usize,
    ) -> Artistico {
        let mut artistico = Artistico::vacio();
        let cuadro = PresentedFrame::full(*camara, (ancho, alto));
        let radio_mundo = diorama.scale.scene_radius * BRUSH_RADIUS_FACTOR;

        let paleta = [
            Color::from_srgb(0.66, 0.09, 0.15),
            Color::from_srgb(0.85, 0.65, 0.17),
            Color::from_srgb(0.18, 0.62, 0.68),
        ];

        for (i, cursor) in cursores(ancho, alto).into_iter().enumerate() {
            let Some(objetivo) = pick_artistic(&diorama.scene, &diorama.accel, &cuadro, cursor)
            else {
                continue;
            };

            let clave = SurfaceKey::new(objetivo.object_index, objetivo.uv_chart);
            let Some((radio_u, radio_v)) = objetivo.uv_world_scale.uv_radii(radio_mundo) else {
                continue;
            };
            let (u, v) = (objetivo.uv.x, objetivo.uv.y);

            artistico
                .masks
                .stamp_ellipse(clave, u, v, radio_u, radio_v, 1.0);

            // Pigmento plano y tela alternados, para que la evidencia
            // muestre las dos composiciones y no solo la barata.
            match (i % 4, diorama.scene.textures.first()) {
                (3, Some(tela)) => artistico
                    .pigment
                    .stamp_texture_ellipse(clave, u, v, radio_u, radio_v, tela, 2.0, 1.0),
                _ => artistico.pigment.stamp_ellipse(
                    clave,
                    u,
                    v,
                    radio_u,
                    radio_v,
                    paleta[i % paleta.len()],
                    1.0,
                ),
            }

            if !artistico.claves.contains(&clave) {
                artistico.claves.push(clave);
            }
        }

        artistico
    }

    /// El impacto sobre el que se apoya el pincel de la cuarta imagen.
    ///
    /// **Uno solo**, y del mismo picking que todo lo demás: el primero de la
    /// rejilla que cae sobre una superficie pintable. Tres pinceles a la vez
    /// no es lo que se ve en la ventana.
    pub fn impacto_para_el_gizmo(
        diorama: &Blockout,
        camara: &Camera,
        ancho: usize,
        alto: usize,
    ) -> Option<PaintHit> {
        let cuadro = PresentedFrame::full(*camara, (ancho, alto));

        cursores(ancho, alto)
            .into_iter()
            .find_map(|cursor| pick_artistic(&diorama.scene, &diorama.accel, &cuadro, cursor))
    }

    /// Traza un cuadro con las capas dadas.
    fn trazar(
        diorama: &Blockout,
        camara: &Camera,
        reveal: &RevealState,
        capas: &Artistico,
    ) -> Framebuffer {
        let mut framebuffer = Framebuffer::new(ANCHO, ALTO);
        let luces = luces_del_diorama(&diorama.anchors, &diorama.scale);

        render_artistic(
            &mut framebuffer,
            &diorama.scene,
            &diorama.accel,
            &luces,
            reveal,
            camara,
            Shading::Material,
            &capas.masks,
            &capas.pigment,
        );

        framebuffer
    }

    /// Guarda un PNG o aborta diciendo por qué.
    fn guardar(framebuffer: &Framebuffer, destino: &Path, nombre: &str) {
        let ruta = destino.join(nombre);

        match framebuffer.save_png(&ruta) {
            Ok(()) => println!("  escrito   {}", ruta.display()),
            Err(e) => {
                eprintln!("error: no se pudo escribir {}: {e}", ruta.display());
                std::process::exit(1);
            }
        }
    }

    pub fn correr() -> ExitCode {
        let destino = std::env::args()
            .nth(1)
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(DESTINO_POR_DEFECTO));

        if let Err(e) = fs::create_dir_all(&destino) {
            eprintln!("error: no se pudo crear {}: {e}", destino.display());
            return ExitCode::FAILURE;
        }

        let diorama = nivel();
        let camara = diorama.hero_camera();
        let radio_mundo = diorama.scale.scene_radius * BRUSH_RADIUS_FACTOR;
        let fixture = fixture_artistica(&diorama, &camara, ANCHO, ALTO);
        let vacio = Artistico::vacio();

        println!("artistic_evidence · version artistica vigente\n");
        println!("  destino     {}", destino.display());
        println!("  feature     artistic-brush (obligatoria para este ejemplo)");
        println!(
            "  ruta        {}",
            if cfg!(feature = "hex-prism") {
                "A, hex-prism por defecto"
            } else {
                "B, --no-default-features"
            }
        );
        println!("  preset      safe-refractive-water, texturas desde la raiz");
        println!("  objetos     {} primitivas", diorama.scene.objects.len());
        println!("  texturas    {}", diorama.scene.textures.len());
        println!("  dimension   {ANCHO} x {ALTO}, toma hero");
        println!(
            "  mascaras    {BRUSH_RESOLUTION} x {BRUSH_RESOLUTION} por superficie, radio {radio_mundo:.4} de mundo"
        );
        println!(
            "  fixture     rejilla {} x {} resuelta con pick_artistic",
            REJILLA.0, REJILLA.1
        );
        println!(
            "  revelado    {} superficies con mascara",
            fixture.masks.len()
        );
        println!(
            "  pigmento    {} superficies con color",
            fixture.pigment.len()
        );
        println!(
            "  claves      {} SurfaceKey distintas\n",
            fixture.claves.len()
        );

        // 01 · el lienzo: lo que se ve al arrancar, sin nada pintado.
        let uno = trazar(&diorama, &camara, &RevealState::unpainted(), &vacio);
        guardar(&uno, &destino, "01-canvas-actual.png");

        // 02 · el diorama revelado, con las capas artisticas vacias. Es la
        // referencia contra la que se mira lo que anaden las capas.
        let dos = trazar(&diorama, &camara, &RevealState::painted(), &vacio);
        guardar(&dos, &destino, "02-artistico-vacio.png");

        // 03 · el mismo estado con la fixture aplicada.
        let tres = trazar(&diorama, &camara, &RevealState::painted(), &fixture);
        guardar(&tres, &destino, "03-artistico-pintado.png");

        // 04 · la 03 mas **un** pincel sobre un impacto real.
        let mut cuatro = trazar(&diorama, &camara, &RevealState::painted(), &fixture);

        match impacto_para_el_gizmo(&diorama, &camara, ANCHO, ALTO) {
            Some(objetivo) => {
                let dibujo = draw_brush_gizmo(
                    &mut cuatro,
                    &camara,
                    &objetivo.point,
                    &objetivo.normal,
                    radio_mundo,
                    CERDAS_AL_REVELAR,
                    diorama.scene.textures.get(TEXTURA_DEL_MANGO),
                );

                println!(
                    "  pincel      objeto {}, carta {:?}, dibujado {dibujo}",
                    objetivo.object_index, objetivo.uv_chart
                );
            }
            None => {
                eprintln!("error: ningun cursor de la rejilla cayo sobre una superficie");
                return ExitCode::FAILURE;
            }
        }

        guardar(&cuatro, &destino, "04-pincel-en-uso.png");

        println!("\n  Las cuatro comparten escena, camara y resolucion. Lo unico que");
        println!("  cambia entre ellas es el estado de revelado y que las capas lleven");
        println!("  pintura. Esto no es una aprobacion visual: son los cuadros que hay");
        println!("  que mirar para darla.");

        ExitCode::SUCCESS
    }
}

#[cfg(all(test, feature = "artistic-brush"))]
mod tests {
    use super::imp::*;

    /// La fixture tiene que ser **de verdad**: pintada sobre superficies que
    /// existen, identificadas por el mismo picking que usa la ventana.
    ///
    /// Sin esto, la tercera y la cuarta imagen serían idénticas a la segunda
    /// y la evidencia diría que el modo artístico no cambia nada.
    #[test]
    fn la_fixture_nace_del_picking_y_no_esta_vacia() {
        let diorama = nivel();
        let camara = diorama.hero_camera();
        let fixture = fixture_artistica(&diorama, &camara, ANCHO, ALTO);

        assert!(
            !fixture.claves.is_empty(),
            "la fixture no registro ninguna SurfaceKey"
        );
        assert!(
            fixture.masks.len() >= 1,
            "el revelado local toca {} superficies",
            fixture.masks.len()
        );
        assert!(
            fixture.pigment.len() >= 1,
            "el pigmento toca {} superficies",
            fixture.pigment.len()
        );

        // Y cada clave apunta a un objeto real, con una carta que su
        // primitiva reconoce.
        for clave in &fixture.claves {
            let objeto = diorama
                .scene
                .objects
                .get(clave.object_index)
                .unwrap_or_else(|| panic!("la clave {clave:?} apunta fuera de la escena"));

            assert!(
                objeto.primitive.uv_world_scale(clave.uv_chart).is_some(),
                "la carta de {clave:?} no es de su primitiva"
            );
        }
    }

    /// El impacto que sostiene el gizmo también sale del picking, no de un
    /// punto escrito a mano.
    #[test]
    fn hay_un_impacto_real_para_el_pincel() {
        let diorama = nivel();
        let camara = diorama.hero_camera();

        let objetivo = impacto_para_el_gizmo(&diorama, &camara, ANCHO, ALTO)
            .expect("ningun cursor de la rejilla cayo sobre el diorama");

        assert!(
            diorama.scene.objects.get(objetivo.object_index).is_some(),
            "el impacto apunta fuera de la escena"
        );
        assert!(
            objetivo.normal.magnitude() > 0.0,
            "el impacto no trae normal"
        );
    }
}
