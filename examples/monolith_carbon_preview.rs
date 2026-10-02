//! Preview del **Monolito de carbón con el 33** mirando hacia Praderas.
//!
//! ```bash
//! cargo run --release --example monolith_carbon_preview -- <carpeta>
//! ```
//!
//! La carpeta es **obligatoria**: el ejemplo no escribe evidencia oficial ni
//! toca producción. Compara la entrega anterior al Monolito de carbón
//! (`delivery_level_previo_monolito_con`, 218 primitivas) con una candidata
//! construida **solo en este archivo**:
//!
//! - los diez tramos del Monolito pasan de cristal pictórico a **carbón**:
//!   gris oscuro mate con grano, opaco, sin reflejo ni refracción trazados y
//!   con un brillo local contenido. Material y textura **propios** —el
//!   cristal y su textura no se tocan—; el lienzo inicial y el revelado
//!   `Finale` se conservan, y la geometría también;
//! - un **33** marfil pálido sobre la cara del tramo central que mira a
//!   Praderas. Un material vale para el objeto entero —no hay mapeo por
//!   cara sin tocar el motor—, así que el número va en **una** placa
//!   delgada (`0.004`) apoyada sobre esa cara: `219` primitivas. Su fondo es
//!   el mismo carbón que la cara muestreado en sus mismas coordenadas, de
//!   modo que solo se distingue la tinta.
//!
//! Tomas, a `800 x 600` con los assets reales, todo revelado y el renderer
//! artístico de la ventana con las máscaras vacías: hero, cenital `78°`,
//! desde Praderas (la órbita de la ventana girada `180°` y acercada al
//! mínimo artístico, el encuadre de la captura), su recorte, un cercano del
//! 33 y los dos laterales; desde Praderas y el cercano también a
//! `320 x 240`, el perfil interactivo. Al final, una medición ligera
//! intercalada de tiempos y rayos.
//!
//! # Promoción
//!
//! `carbon_33` se aprobó como default y se promovió a
//! `src/scenes/monolith_carbon_delivery.rs`: `delivery_level_con` la aplica.
//! Este preview conserva su candidata **independiente** sobre la línea base
//! de 218, y el test `la_entrega_de_produccion_es_carbon_33` comprueba con los
//! assets reales, en los tres presets, que producción es esa misma escena.
//! La columna `delivery` del render es producción, y tiene que salir idéntica
//! a `carbon_33`.

use std::process::ExitCode;

fn main() -> ExitCode {
    imp::correr()
}

mod imp {
    use std::f32::consts::PI;
    use std::path::{Path, PathBuf};
    use std::process::ExitCode;
    use std::time::Instant;

    use expedition33_continente_inacabado::accel::{ClusterPlan, SceneAccel, TraversalStats};
    use expedition33_continente_inacabado::bounds::Aabb;
    use expedition33_continente_inacabado::brush::{BrushMasks, PigmentMasks};
    use expedition33_continente_inacabado::camera::Camera;
    use expedition33_continente_inacabado::color::Color;
    use expedition33_continente_inacabado::cuboid::Cuboid;
    use expedition33_continente_inacabado::framebuffer::Framebuffer;
    use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
    use expedition33_continente_inacabado::material::{Material, ShadowMode};
    use expedition33_continente_inacabado::primitive::Primitive;
    use expedition33_continente_inacabado::renderer::{
        render_artistic, InteractiveProfile, Shading,
    };
    use expedition33_continente_inacabado::reveal::RevealState;
    use expedition33_continente_inacabado::scene::{
        MaterialId, RevealGroup, Scene, SceneObject, SpatialGroupId, TextureId,
    };
    use expedition33_continente_inacabado::scene_builder::{Blockout, HERO_YAW_DEGREES};
    use expedition33_continente_inacabado::scenes::{
        delivery_level_con, delivery_level_previo_monolito_con, WaterPreset,
    };
    use expedition33_continente_inacabado::texture::{Texture, WrapMode};
    use nalgebra_glm::Vec3;

    pub const ANCHO: usize = 800;
    pub const ALTO: usize = 600;
    pub const ELEVACION_CENITAL: f32 = 78.0;
    /// El mínimo de órbita del modo artístico, como en `main.rs`.
    pub const FACTOR_ARTISTICO: f32 = 1.7;
    /// Pilares del Rompeolas original: el plan de clusters de producción.
    const ROMPEOLAS_ORIGINAL: usize = 38;

    // ------------------------------------------------ la placa del 33

    /// Margen lateral de la placa respecto de la cara del tramo.
    pub const MARGEN_X: f32 = 0.08;
    /// Alto de la placa, dentro de la franja que se ve desde Praderas
    /// (`y >= 4.6`, medido con rayos).
    pub const PLACA_Y: (f32, f32) = (5.3, 6.9);
    /// Grosor: hacia Praderas, apoyada sobre la cara.
    pub const GROSOR: f32 = 0.004;
    /// Alto de los dígitos en mundo, y su proporción.
    pub const ALTO_DIGITO: f32 = 1.2;
    /// Ancho de un dígito y hueco entre los dos, en altos de dígito.
    pub const ANCHO_DIGITO: f32 = 0.52;
    pub const HUECO_DIGITOS: f32 = 0.12;
    /// Texels por unidad de mundo de las texturas de la placa.
    pub const TEXELS_POR_UNIDAD: f32 = 400.0;

    /// Marfil cálido pálido del número, en sRGB, **compensado**: la cara
    /// que mira a Praderas recibe de lleno `L-03`, el acento cian del
    /// Monolito (`0.55, 0.95, 1.0`, delante de esa cara). Sin compensar, un
    /// marfil neutro se ve verde menta; las luces no se tocan.
    pub const MARFIL: (f32, f32, f32) = (1.0, 0.79, 0.60);

    /// Tono base del carbón en sRGB, compensado igual: algo cálido para que
    /// bajo el cian se lea gris carbón y no pizarra azul.
    pub const CARBON: (f32, f32, f32) = (0.185, 0.17, 0.168);

    /// La línea base: la entrega anterior al Monolito de carbón, 218.
    pub fn actual(raiz: &Path) -> Blockout {
        actual_con(WaterPreset::RefractiveWater, raiz)
    }

    pub fn actual_con(water: WaterPreset, raiz: &Path) -> Blockout {
        delivery_level_previo_monolito_con(water, Some(raiz))
            .expect("los assets reales tienen que estar en la raiz del proyecto")
    }

    /// La entrega de producción de ahora, con el Monolito promovido.
    pub fn produccion_con(water: WaterPreset, raiz: &Path) -> Blockout {
        delivery_level_con(water, Some(raiz))
            .expect("los assets reales tienen que estar en la raiz del proyecto")
    }

    /// El plan de clusters de `edge_island::medir_y_acelerar`, reconstruido
    /// para la candidata, que tiene una primitiva más.
    pub fn reacelerar(scene: &Scene) -> SceneAccel {
        let mut plan = ClusterPlan::new();
        let rompeolas: Vec<usize> = (0..scene.objects.len())
            .filter(|&i| scene.objects[i].spatial_group == SpatialGroupId::Breakwater)
            .collect();
        for (k, i) in rompeolas.into_iter().enumerate() {
            plan.asignar(i, u16::from(k >= ROMPEOLAS_ORIGINAL));
        }
        SceneAccel::build_from_plan(scene, &plan).expect("la escena tiene geometria")
    }

    pub fn indices_del_monolito(scene: &Scene) -> Vec<usize> {
        (0..scene.objects.len())
            .filter(|&i| scene.objects[i].spatial_group == SpatialGroupId::Monolith)
            .collect()
    }

    /// Dirección horizontal del Monolito hacia Praderas, **de las anclas**.
    pub fn hacia_praderas(d: &Blockout) -> Vec3 {
        let v = d.anchors.meadows_anchor - d.anchors.monolith_base_anchor;
        Vec3::new(v.x, 0.0, v.z).normalize()
    }

    /// El tramo central: el de `y` de `4.4` a `7.6`, cuya cara hacia
    /// Praderas es la que se ve entera desde allí.
    pub fn tramo_del_33(scene: &Scene) -> usize {
        indices_del_monolito(scene)
            .into_iter()
            .find(|&i| {
                let b = scene.objects[i].primitive.bounds();
                (b.min.y - 4.4).abs() < 1e-3 && (b.max.y - 7.6).abs() < 1e-3
            })
            .expect("el tramo central del Monolito")
    }

    /// La caja de la placa sobre la cara `-Z` del tramo.
    pub fn caja_de_la_placa(tramo: &Aabb) -> Aabb {
        Aabb::from_corners(
            Vec3::new(tramo.min.x + MARGEN_X, PLACA_Y.0, tramo.min.z - GROSOR),
            Vec3::new(tramo.max.x - MARGEN_X, PLACA_Y.1, tramo.min.z),
        )
    }

    // ------------------------------------------------ el dígito

    /// Tinta del **33** en coordenadas de **lectura**: `s` crece hacia la
    /// derecha de quien lo lee y `t` hacia arriba, ambas en altos de
    /// dígito, con el origen en la esquina inferior izquierda del bloque.
    /// Devuelve la distancia con signo al trazo (negativa dentro).
    pub fn distancia_al_33(s: f32, t: f32) -> f32 {
        let segundo = ANCHO_DIGITO + HUECO_DIGITOS;
        distancia_al_3(s, t).min(distancia_al_3(s - segundo, t))
    }

    /// Medio grosor del trazo, en altos de dígito.
    const TRAZO: f32 = 0.075;

    /// Un «3» de dos panzas: dos arcos abiertos a la izquierda —el de
    /// arriba algo menor— y una lengüeta horizontal donde se juntan.
    /// `s` de `0` a `ANCHO_DIGITO`, `t` de `0` a `1`.
    fn distancia_al_3(s: f32, t: f32) -> f32 {
        let arriba = arco(s, t, (0.24, 0.735), 0.19, -90.0, 140.0);
        let abajo = arco(s, t, (0.24, 0.29), 0.205, -140.0, 90.0);
        let lengua = segmento(s, t, (0.13, 0.52), (0.24, 0.52));
        arriba.min(abajo).min(lengua) - TRAZO
    }

    /// Distancia (sin grosor) a un arco de circunferencia entre dos
    /// ángulos en grados, recorrido en sentido antihorario de `desde` a
    /// `hasta`.
    fn arco(s: f32, t: f32, c: (f32, f32), r: f32, desde: f32, hasta: f32) -> f32 {
        let (dx, dy) = (s - c.0, t - c.1);
        let angulo = dy.atan2(dx).to_degrees();
        if angulo >= desde && angulo <= hasta {
            return ((dx * dx + dy * dy).sqrt() - r).abs();
        }
        let punta = |g: f32| {
            let g = g.to_radians();
            let (px, py) = (c.0 + r * g.cos(), c.1 + r * g.sin());
            ((s - px).powi(2) + (t - py).powi(2)).sqrt()
        };
        punta(desde).min(punta(hasta))
    }

    fn segmento(s: f32, t: f32, a: (f32, f32), b: (f32, f32)) -> f32 {
        let (abx, aby) = (b.0 - a.0, b.1 - a.1);
        let k = (((s - a.0) * abx + (t - a.1) * aby) / (abx * abx + aby * aby)).clamp(0.0, 1.0);
        ((s - a.0 - k * abx).powi(2) + (t - a.1 - k * aby).powi(2)).sqrt()
    }

    /// Ancho del bloque `33`, en altos de dígito.
    pub fn ancho_del_bloque() -> f32 {
        2.0 * ANCHO_DIGITO + HUECO_DIGITOS
    }

    // ------------------------------------------------ texturas

    /// Lado de la textura de carbón y su semilla.
    pub const LADO_CARBON: usize = 128;
    const SEMILLA: u32 = 0x0C4A_2B33;

    fn azar(x: u32, y: u32, octava: u32) -> f32 {
        let mut h = x.wrapping_mul(0x8DA6_B343)
            ^ y.wrapping_mul(0xD816_3841)
            ^ octava.wrapping_mul(0xCB1A_B31F)
            ^ SEMILLA;
        h ^= h >> 13;
        h = h.wrapping_mul(0x5BD1_E995);
        h ^= h >> 15;
        (h & 0x00FF_FFFF) as f32 / 16_777_215.0
    }

    /// Ruido de valor periódico sobre la rejilla: celdas de `celda`
    /// texels, interpolación suave, índices envueltos para que la textura
    /// repita sin costura.
    fn ruido(x: usize, y: usize, celda: usize, octava: u32) -> f32 {
        let n = (LADO_CARBON / celda) as u32;
        let (fx, fy) = (x as f32 / celda as f32, y as f32 / celda as f32);
        let (x0, y0) = (fx.floor() as u32, fy.floor() as u32);
        let suave = |f: f32| f * f * (3.0 - 2.0 * f);
        let (tx, ty) = (suave(fx - x0 as f32), suave(fy - y0 as f32));
        let v = |i: u32, j: u32| azar(i % n, j % n, octava);
        let a = v(x0, y0) + (v(x0 + 1, y0) - v(x0, y0)) * tx;
        let b = v(x0, y0 + 1) + (v(x0 + 1, y0 + 1) - v(x0, y0 + 1)) * tx;
        a + (b - a) * ty
    }

    /// Carbón: gris oscuro algo frío, con grano de cuatro octavas y vetas
    /// claras escasas. Sin brillo horneado. `Repeat`.
    pub fn textura_de_carbon() -> Texture {
        let mut pixels = Vec::with_capacity(LADO_CARBON * LADO_CARBON);
        for y in 0..LADO_CARBON {
            for x in 0..LADO_CARBON {
                let n = 0.45 * ruido(x, y, 32, 1)
                    + 0.27 * ruido(x, y, 16, 2)
                    + 0.17 * ruido(x, y, 8, 3)
                    + 0.11 * ruido(x, y, 4, 4);
                let veta = ((ruido(x, y, 8, 5) - 0.78).max(0.0) * 4.0).min(1.0);
                let k = 0.80 + 0.40 * n + 0.18 * veta;
                pixels.push(Color::from_srgb(CARBON.0 * k, CARBON.1 * k, CARBON.2 * k));
            }
        }
        Texture::from_pixels(LADO_CARBON, LADO_CARBON, pixels).expect("dimensiones validas")
    }

    /// Una textura para la placa: cada texel es el punto del mundo de la
    /// cara del tramo que tapa, `color(uv_de_la_cara, tinta)`. Así el fondo
    /// continúa la cara y la tinta va donde dice el diseño, leída desde
    /// Praderas: desde allí la derecha de la pantalla es `-X`, así que la
    /// lectura crece al bajar `x` (el espejo en `u` se compensa aquí).
    fn hornear_la_placa(
        tramo: &Aabb,
        placa: &Aabb,
        color: impl Fn(nalgebra_glm::Vec2, f32) -> Color,
    ) -> Texture {
        let (ancho, alto) = (placa.max.x - placa.min.x, placa.max.y - placa.min.y);
        let (w, h) = (
            (ancho * TEXELS_POR_UNIDAD).round() as usize,
            (alto * TEXELS_POR_UNIDAD).round() as usize,
        );
        let centro_x = (placa.min.x + placa.max.x) / 2.0;
        let y0 = (placa.min.y + placa.max.y) / 2.0 - ALTO_DIGITO / 2.0;
        let mut pixels = Vec::with_capacity(w * h);
        for fila in 0..h {
            for col in 0..w {
                // Cuatro por cuatro muestras por texel para el borde de la
                // tinta; el fondo se toma en el centro.
                let mut cubierta = 0.0;
                for sy in 0..4 {
                    for sx in 0..4 {
                        let u = (col as f32 + (sx as f32 + 0.5) / 4.0) / w as f32;
                        let v = 1.0 - (fila as f32 + (sy as f32 + 0.5) / 4.0) / h as f32;
                        let (x, y) = (placa.min.x + u * ancho, placa.min.y + v * alto);
                        let s = (centro_x - x) / ALTO_DIGITO + ancho_del_bloque() / 2.0;
                        let t = (y - y0) / ALTO_DIGITO;
                        if distancia_al_33(s, t) < 0.0 {
                            cubierta += 1.0 / 16.0;
                        }
                    }
                }
                let u = (col as f32 + 0.5) / w as f32;
                let v = 1.0 - (fila as f32 + 0.5) / h as f32;
                let (x, y) = (placa.min.x + u * ancho, placa.min.y + v * alto);
                let uv_cara = nalgebra_glm::Vec2::new(
                    (x - tramo.min.x) / (tramo.max.x - tramo.min.x),
                    (y - tramo.min.y) / (tramo.max.y - tramo.min.y),
                );
                pixels.push(color(uv_cara, cubierta));
            }
        }
        Texture::from_pixels(w, h, pixels)
            .expect("dimensiones validas")
            .with_wrap(WrapMode::Clamp)
    }

    /// La candidata y lo que añade.
    pub struct Candidata {
        pub diorama: Blockout,
        pub placa: usize,
        pub tramo: usize,
        pub carbon: MaterialId,
        pub numero: MaterialId,
        pub lienzo_de_la_placa: MaterialId,
        pub textura_carbon: TextureId,
        pub textura_numero: TextureId,
        pub textura_lienzo: TextureId,
    }

    /// Material del carbón: opaco, sin rebotes, con un brillo local
    /// contenido para que las aristas sigan leyéndose de noche.
    pub fn material_de_carbon(textura: TextureId) -> Material {
        Material::new(Color::from_srgb(0.2, 0.2, 0.2))
            .with_texture(textura)
            .with_uv_scale(1.0)
            .with_specular(0.14, 28.0)
    }

    /// Solo el carbón en los diez tramos, sin número: la otra opción
    /// artística y la referencia para medir la costura de la placa.
    pub fn solo_carbon(actual: Blockout) -> (Blockout, MaterialId, TextureId) {
        let Blockout {
            mut scene,
            anchors,
            scale,
            accel,
        } = actual;
        let textura_carbon = scene.add_texture(textura_de_carbon());
        let carbon = scene.add_material(material_de_carbon(textura_carbon));
        for i in indices_del_monolito(&scene) {
            scene.objects[i].final_material = carbon;
        }
        // Mismos objetos y misma geometría: la jerarquía no cambia.
        (
            Blockout {
                scene,
                accel,
                anchors,
                scale,
            },
            carbon,
            textura_carbon,
        )
    }

    pub fn candidata(actual: Blockout) -> Candidata {
        let (
            Blockout {
                mut scene,
                anchors,
                scale,
                ..
            },
            carbon,
            textura_carbon,
        ) = solo_carbon(actual);
        let tramo = tramo_del_33(&scene);
        let caja_tramo = scene.objects[tramo].primitive.bounds();
        let caja = caja_de_la_placa(&caja_tramo);
        let mat_carbon = scene.material(carbon);
        let carbon_t = scene.textures[textura_carbon.0].clone();

        // La placa revelada: el carbón de la cara, en sus coordenadas, con
        // la tinta marfil encima.
        let marfil = Color::from_srgb(MARFIL.0, MARFIL.1, MARFIL.2);
        let numero_t = hornear_la_placa(&caja_tramo, &caja, |uv, cubierta| {
            let fondo = mat_carbon.albedo
                * carbon_t.sample(uv.x * mat_carbon.uv_scale, uv.y * mat_carbon.uv_scale);
            fondo * (1.0 - cubierta) + marfil * cubierta
        });
        let textura_numero = scene.add_texture(numero_t);
        let numero = scene.add_material(Material {
            albedo_texture: Some(textura_numero),
            albedo: Color::new(1.0, 1.0, 1.0),
            uv_scale: 1.0,
            // Es un calco sobre la cara: no hace sombra sobre ella.
            shadow_mode: ShadowMode::Ignore,
            ..mat_carbon
        });

        // La placa sin revelar: el lienzo de la cara, horneado igual.
        let lienzo_id = scene.objects[tramo].initial_material;
        let lienzo = scene.material(lienzo_id);
        let lienzo_t = {
            let s = &scene;
            hornear_la_placa(&caja_tramo, &caja, |uv, _| s.albedo_at(&lienzo, &uv))
        };
        let textura_lienzo = scene.add_texture(lienzo_t);
        let lienzo_de_la_placa = scene.add_material(Material {
            albedo_texture: Some(textura_lienzo),
            albedo: Color::new(1.0, 1.0, 1.0),
            uv_scale: 1.0,
            shadow_mode: ShadowMode::Ignore,
            ..lienzo
        });

        scene.objects.push(SceneObject {
            primitive: Primitive::Cuboid(Cuboid::new(caja)),
            initial_material: lienzo_de_la_placa,
            final_material: numero,
            spatial_group: SpatialGroupId::Monolith,
            reveal_group: RevealGroup::Finale,
        });
        let placa = scene.objects.len() - 1;

        let accel = reacelerar(&scene);
        Candidata {
            diorama: Blockout {
                scene,
                accel,
                anchors,
                scale,
            },
            placa,
            tramo,
            carbon,
            numero,
            lienzo_de_la_placa,
            textura_carbon,
            textura_numero,
            textura_lienzo,
        }
    }

    // ------------------------------------------------ cámaras

    /// La cámara de arranque de la ventana.
    pub fn camara_hero(d: &Blockout) -> Camera {
        let hero = d.hero_camera();
        hero.with_radius_limits(d.scale.scene_radius * FACTOR_ARTISTICO, hero.max_radius)
    }

    pub fn camara_cenital(d: &Blockout) -> Camera {
        d.camera_at(HERO_YAW_DEGREES, ELEVACION_CENITAL)
    }

    /// La de la ventana girada `giro` grados con las flechas (pasos de
    /// `PI / 60`, como `ROTATION_SPEED`).
    pub fn camara_girada(d: &Blockout, giro: u32) -> Camera {
        let mut c = camara_hero(d);
        for _ in 0..giro / 3 {
            c.orbit(PI / 60.0, 0.0);
        }
        c
    }

    /// Desde Praderas: girada `180°` y acercada al mínimo artístico. Es el
    /// encuadre de la captura de Charlie.
    pub fn camara_desde_praderas(d: &Blockout) -> Camera {
        let mut c = camara_girada(d, 180);
        let r = c.radius();
        c.zoom(c.min_radius - r);
        c
    }

    /// Cercano explícito del 33, de frente y algo por encima, desde el lado
    /// de Praderas.
    pub fn camara_del_33(d: &Blockout) -> Camera {
        let objetivo = Vec3::new(0.25, 6.1, -1.0);
        let ojo = objetivo + hacia_praderas(d) * 9.0 + Vec3::new(0.0, 3.5, 0.0);
        Camera::new(
            ojo,
            objetivo,
            objetivo,
            Vec3::new(0.0, 1.0, 0.0),
            30f32.to_radians(),
        )
    }

    // ------------------------------------------------ render

    pub fn pintar(
        d: &Blockout,
        camara: &Camera,
        ancho: usize,
        alto: usize,
    ) -> (Framebuffer, TraversalStats) {
        pintar_con(d, camara, ancho, alto, &RevealState::painted())
    }

    pub fn pintar_con(
        d: &Blockout,
        camara: &Camera,
        ancho: usize,
        alto: usize,
        reveal: &RevealState,
    ) -> (Framebuffer, TraversalStats) {
        let luces = luces_del_diorama(&d.anchors, &d.scale);
        let (m, p) = (BrushMasks::new(128, 128), PigmentMasks::new(128, 128));
        let mut fb = Framebuffer::new(ancho, alto);
        let st = render_artistic(
            &mut fb,
            &d.scene,
            &d.accel,
            &luces,
            reveal,
            camara,
            Shading::Material,
            &m,
            &p,
        );
        (fb, st)
    }

    pub fn correr() -> ExitCode {
        let Some(destino) = std::env::args().nth(1).map(PathBuf::from) else {
            eprintln!("uso: cargo run --release --example monolith_carbon_preview -- <carpeta>");
            return ExitCode::FAILURE;
        };
        if let Err(e) = std::fs::create_dir_all(&destino) {
            eprintln!("error: no se pudo crear {}: {e}", destino.display());
            return ExitCode::FAILURE;
        }
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let vigente = actual(&raiz);
        let (carbon, _, _) = solo_carbon(actual(&raiz));
        let c = candidata(actual(&raiz));
        let entrega = produccion_con(WaterPreset::RefractiveWater, &raiz);
        let variantes: [(&str, &Blockout); 4] = [
            ("current", &vigente),
            ("carbon", &carbon),
            ("carbon_33", &c.diorama),
            ("delivery", &entrega),
        ];
        let perfil = InteractiveProfile::default();
        let d = &vigente;
        let tomas: [(&str, Camera, usize, usize); 9] = [
            ("hero", camara_hero(d), ANCHO, ALTO),
            ("top78", camara_cenital(d), ANCHO, ALTO),
            ("praderas", camara_desde_praderas(d), ANCHO, ALTO),
            (
                "praderas_320",
                camara_desde_praderas(d),
                perfil.width,
                perfil.height,
            ),
            ("cercano_33", camara_del_33(d), ANCHO, ALTO),
            (
                "cercano_33_320",
                camara_del_33(d),
                perfil.width,
                perfil.height,
            ),
            ("lateral_90", camara_girada(d, 90), ANCHO, ALTO),
            ("lateral_270", camara_girada(d, 270), ANCHO, ALTO),
            (
                "praderas_sin_revelar",
                camara_desde_praderas(d),
                ANCHO,
                ALTO,
            ),
        ];
        println!("monolith_carbon_preview -> {}", destino.display());
        println!(
            "  objetos current {} carbon {} carbon_33 {} (placa {}, tramo {})",
            vigente.scene.objects.len(),
            carbon.scene.objects.len(),
            c.diorama.scene.objects.len(),
            c.placa,
            c.tramo
        );
        println!(
            "  placa {:?}",
            c.diorama.scene.objects[c.placa].primitive.bounds()
        );
        println!(
            "  materiales nuevos: carbon {:?} numero {:?} lienzo de la placa {:?}; texturas {:?} {:?} {:?}",
            c.carbon, c.numero, c.lienzo_de_la_placa, c.textura_carbon, c.textura_numero, c.textura_lienzo
        );
        let mut ok = true;
        for (toma, camara, w, h) in &tomas {
            for (nombre, v) in variantes {
                let reveal = if toma.ends_with("sin_revelar") {
                    RevealState::unpainted()
                } else {
                    RevealState::painted()
                };
                let (fb, st) = pintar_con(v, camara, *w, *h, &reveal);
                let ruta = destino.join(format!("{nombre}_{toma}.png"));
                if let Err(e) = fb.save_png(&ruta) {
                    eprintln!("error: {}: {e}", ruta.display());
                    ok = false;
                }
                println!(
                    "  {toma:<22} {nombre:<10} {w}x{h} refl {} refr {} sombra {} primitivas {}",
                    st.reflection_rays, st.refraction_rays, st.shadow_rays, st.primitive_tests
                );
                // El recorte de la captura: la zona de la escena que enseña.
                if *toma == "praderas" {
                    let (x0, y0, cw, ch) = (180, 20, 467, 465);
                    let mut r = Framebuffer::new(cw * 2, ch * 2);
                    for y in 0..ch * 2 {
                        for x in 0..cw * 2 {
                            r.buffer[y * cw * 2 + x] = fb.buffer[(y0 + y / 2) * ANCHO + x0 + x / 2];
                        }
                    }
                    let ruta = destino.join(format!("{nombre}_praderas_recorte_x2.png"));
                    if let Err(e) = r.save_png(&ruta) {
                        eprintln!("error: {}: {e}", ruta.display());
                        ok = false;
                    }
                }
            }
        }

        // Medición ligera: tres rondas intercaladas, misma vista y
        // resolución, vigente contra candidata.
        println!("\n  tiempos, 3 rondas intercaladas (s por cuadro):");
        let casos: [(&str, Camera, usize, usize); 4] = [
            ("praderas", camara_desde_praderas(d), ANCHO, ALTO),
            (
                "praderas_320",
                camara_desde_praderas(d),
                perfil.width,
                perfil.height,
            ),
            ("hero", camara_hero(d), ANCHO, ALTO),
            ("hero_320", camara_hero(d), perfil.width, perfil.height),
        ];
        for (toma, camara, w, h) in &casos {
            let mut tiempos = [Vec::new(), Vec::new()];
            let mut rayos = [TraversalStats::default(); 2];
            for ronda in 0..3 {
                for k in 0..2 {
                    let i = (k + ronda) % 2;
                    let v = if i == 0 { &vigente } else { &c.diorama };
                    let inicio = Instant::now();
                    let (_, st) = pintar(v, camara, *w, *h);
                    tiempos[i].push(inicio.elapsed().as_secs_f64());
                    rayos[i] = st;
                }
            }
            let mediana = |t: &Vec<f64>| {
                let mut t = t.clone();
                t.sort_by(|a, b| a.partial_cmp(b).unwrap());
                t[1]
            };
            println!(
                "  {toma:<14} current {:.4} {:?} | carbon_33 {:.4} {:?} | cociente {:.3}",
                mediana(&tiempos[0]),
                tiempos[0]
                    .iter()
                    .map(|x| (x * 1e4).round() / 1e4)
                    .collect::<Vec<_>>(),
                mediana(&tiempos[1]),
                tiempos[1]
                    .iter()
                    .map(|x| (x * 1e4).round() / 1e4)
                    .collect::<Vec<_>>(),
                mediana(&tiempos[1]) / mediana(&tiempos[0])
            );
            for (i, n) in ["current", "carbon_33"].iter().enumerate() {
                println!(
                    "      {n:<10} primarios {} sombra {} refl {} refr {} primitivas {} clusters {}",
                    rayos[i].primary_rays,
                    rayos[i].shadow_rays,
                    rayos[i].reflection_rays,
                    rayos[i].refraction_rays,
                    rayos[i].primitive_tests,
                    rayos[i].cluster_bounds_tests
                );
            }
        }
        println!("\nexit=0");
        if ok {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::imp::*;
    use expedition33_continente_inacabado::camera::Camera;
    use expedition33_continente_inacabado::framebuffer::Framebuffer;
    use expedition33_continente_inacabado::material::ShadowMode;
    use expedition33_continente_inacabado::renderer::InteractiveProfile;
    use expedition33_continente_inacabado::reveal::RevealState;
    use expedition33_continente_inacabado::scene::{RevealGroup, Scene, SpatialGroupId};
    use expedition33_continente_inacabado::scene_builder::Blockout;
    use expedition33_continente_inacabado::texture::{Texture, WrapMode};
    use std::path::PathBuf;
    use std::sync::OnceLock;

    fn raiz() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn vigente() -> &'static Blockout {
        static V: OnceLock<Blockout> = OnceLock::new();
        V.get_or_init(|| actual(&raiz()))
    }

    fn cand() -> &'static Candidata {
        static C: OnceLock<Candidata> = OnceLock::new();
        C.get_or_init(|| candidata(actual(&raiz())))
    }

    fn luma(c: expedition33_continente_inacabado::color::Color) -> f32 {
        0.2126 * c.r + 0.7152 * c.g + 0.0722 * c.b
    }

    fn texeles(t: &Texture) -> Vec<String> {
        let mut v = Vec::with_capacity(t.width() * t.height());
        for y in 0..t.height() {
            for x in 0..t.width() {
                let u = (x as f32 + 0.5) / t.width() as f32;
                let w = (y as f32 + 0.5) / t.height() as f32;
                v.push(format!("{:?}", t.sample(u, w)));
            }
        }
        v
    }

    /// Lo que no es el Monolito queda exactamente igual: objetos, los 54
    /// materiales y las 49 texturas previas, el cielo, anclas y escala.
    #[test]
    fn fuera_del_monolito_todo_es_identico() {
        let (v, c) = (&vigente().scene, &cand().diorama.scene);
        let monolito = indices_del_monolito(v);
        for (i, o) in v.objects.iter().enumerate() {
            let n = &c.objects[i];
            if monolito.contains(&i) {
                assert_eq!(
                    format!("{:?}", o.primitive),
                    format!("{:?}", n.primitive),
                    "geometria {i}"
                );
                assert_eq!(o.initial_material, n.initial_material, "lienzo {i}");
                assert_eq!(
                    (o.spatial_group, o.reveal_group),
                    (n.spatial_group, n.reveal_group)
                );
            } else {
                assert_eq!(format!("{o:?}"), format!("{n:?}"), "objeto {i}");
            }
        }
        assert_eq!(c.palette.len(), v.palette.len() + 3);
        for (i, m) in v.palette.iter().enumerate() {
            assert_eq!(
                format!("{m:?}"),
                format!("{:?}", c.palette[i]),
                "material {i}"
            );
        }
        assert_eq!(c.textures.len(), v.textures.len() + 3);
        for (i, t) in v.textures.iter().enumerate() {
            assert_eq!(texeles(t), texeles(&c.textures[i]), "textura {i}");
        }
        assert_eq!(
            format!("{:?}", v.skybox).len(),
            format!("{:?}", c.skybox).len()
        );
        let (dv, dc) = (vigente(), &cand().diorama);
        assert_eq!(format!("{:?}", dv.anchors), format!("{:?}", dc.anchors));
        assert_eq!(format!("{:?}", dv.scale), format!("{:?}", dc.scale));
    }

    /// Diez tramos con la misma geometría, y una sola primitiva más: la
    /// placa, apoyada en la cara del tramo central sin salirse de ella.
    #[test]
    fn la_geometria_es_la_misma_mas_una_placa() {
        let (v, c) = (&vigente().scene, &cand().diorama.scene);
        assert_eq!(v.objects.len(), 218);
        assert_eq!(c.objects.len(), 219);
        assert_eq!(indices_del_monolito(v).len(), 10);
        let placa = cand().placa;
        assert_eq!(placa, 218);
        let o = &c.objects[placa];
        assert_eq!(o.spatial_group, SpatialGroupId::Monolith);
        assert_eq!(o.reveal_group, RevealGroup::Finale);
        let (b, t) = (
            o.primitive.bounds(),
            c.objects[cand().tramo].primitive.bounds(),
        );
        assert_eq!(cand().tramo, 10);
        assert!((b.max.z - t.min.z).abs() < 1e-6, "apoyada: {b:?} {t:?}");
        assert!(b.max.z - b.min.z <= 0.005 + 1e-6, "delgada");
        assert!(
            b.min.x >= t.min.x && b.max.x <= t.max.x && b.min.y >= t.min.y && b.max.y <= t.max.y
        );
        // La escena no crece: el radio medido sería el mismo.
        let (sv, sc) = (v.bounds().unwrap(), c.bounds().unwrap());
        assert_eq!(format!("{sv:?}"), format!("{sc:?}"));
    }

    /// El plan de clusters reconstruido es el de producción: sobre la
    /// entrega vigente da su misma jerarquía.
    #[test]
    fn el_plan_de_aceleracion_es_el_de_produccion() {
        let d = vigente();
        assert_eq!(
            format!("{:?}", reacelerar(&d.scene)),
            format!("{:?}", d.accel)
        );
    }

    /// Carbón: los diez tramos acaban en un material propio, opaco, sin
    /// reflejo ni refracción, con brillo contenido y una textura propia
    /// oscura **con grano**, ni negra plana ni clara.
    #[test]
    fn el_carbon_es_mate_opaco_oscuro_y_con_grano() {
        let c = cand();
        let s = &c.diorama.scene;
        for i in indices_del_monolito(&vigente().scene) {
            assert_eq!(s.objects[i].final_material, c.carbon, "tramo {i}");
        }
        let m = s.material(c.carbon);
        assert_eq!(
            (m.reflection_cap, m.transmission_cap, m.ior),
            (0.0, 0.0, 1.0)
        );
        assert_eq!(m.shadow_mode, ShadowMode::Opaque);
        assert!(
            m.specular_strength > 0.0 && m.specular_strength <= 0.2,
            "{m:?}"
        );
        assert_eq!(m.albedo_texture, Some(c.textura_carbon));
        let usan: Vec<usize> = (0..s.palette.len())
            .filter(|&k| s.palette[k].albedo_texture == Some(c.textura_carbon))
            .collect();
        assert_eq!(usan, [c.carbon.0], "la textura de carbon es exclusiva");
        let t = &s.textures[c.textura_carbon.0];
        assert!(t.width() >= 64 && t.height() >= 64);
        let lumas: Vec<f32> = texeles(t)
            .iter()
            .enumerate()
            .map(|(k, _)| {
                let (x, y) = (k % t.width(), k / t.width());
                luma(t.sample(
                    (x as f32 + 0.5) / t.width() as f32,
                    (y as f32 + 0.5) / t.height() as f32,
                ))
            })
            .collect();
        let media = lumas.iter().sum::<f32>() / lumas.len() as f32;
        let desv =
            (lumas.iter().map(|l| (l - media).powi(2)).sum::<f32>() / lumas.len() as f32).sqrt();
        let minimo = lumas.iter().cloned().fold(f32::INFINITY, f32::min);
        println!("carbon: luma lineal media {media:.4} desv {desv:.4} minimo {minimo:.4}");
        assert!(
            (0.02..=0.08).contains(&media),
            "oscuro pero no negro: {media}"
        );
        assert!(minimo > 0.008, "sin negro plano: {minimo}");
        assert!(desv / media > 0.08, "con grano: {}", desv / media);
    }

    /// Las anclas dicen que Praderas está hacia `-Z`, y la cara de la placa
    /// que se ve desde allí mira hacia Praderas.
    #[test]
    fn el_33_mira_hacia_praderas() {
        let d = &cand().diorama;
        let h = hacia_praderas(d);
        println!("hacia Praderas, de las anclas: {h:?}");
        assert!(h.z < -0.99, "{h:?}");
        let s = &d.scene;
        let placa = cand().placa;
        let camara = camara_desde_praderas(d);
        let mut normales = 0;
        for p in 0..ANCHO * ALTO {
            let rayo = camara.ray_from_pixel(p % ANCHO, p / ANCHO, ANCHO, ALTO);
            let mut st = Default::default();
            if let Some(hit) = d.accel.intersect(s, &rayo, &mut st) {
                if hit.object_index == placa {
                    assert!(hit.normal.dot(&h) > 0.99, "{:?}", hit.normal);
                    normales += 1;
                }
            }
        }
        assert!(normales > 500, "{normales}");
    }

    struct Lectura {
        tinta: usize,
        acuerdo: usize,
        en_placa: usize,
        alto_px: usize,
        componentes: usize,
        orden_ok: bool,
    }

    /// Lee el 33 con rayos desde una cámara: qué píxeles caen en la placa,
    /// cuáles son tinta según el **material efectivo** (la textura final en
    /// la `uv` del impacto), y si esa tinta coincide con el diseño leído en
    /// el orden de la pantalla —izquierda a derecha y abajo arriba—.
    fn leer(c: &Candidata, camara: &Camera, ancho: usize, alto: usize) -> Lectura {
        let d = &c.diorama;
        let s = &d.scene;
        let numero = s.material(c.numero);
        let placa = s.objects[c.placa].primitive.bounds();
        let centro_x = (placa.min.x + placa.max.x) / 2.0;
        let y0 = (placa.min.y + placa.max.y) / 2.0 - ALTO_DIGITO / 2.0;
        let mut mascara = vec![false; ancho * alto];
        let (mut tinta, mut acuerdo, mut en_placa) = (0, 0, 0);
        let mut filas = (usize::MAX, 0);
        let mut orden_ok = true;
        for py in 0..alto {
            let mut anterior: Option<f32> = None;
            for px in 0..ancho {
                let rayo = camara.ray_from_pixel(px, py, ancho, alto);
                let mut st = Default::default();
                let Some(hit) = d.accel.intersect(s, &rayo, &mut st) else {
                    continue;
                };
                if hit.object_index != c.placa {
                    continue;
                }
                en_placa += 1;
                let es_tinta = luma(s.albedo_at(&numero, &hit.uv)) > 0.3;
                // El diseño, leído en coordenadas de pantalla: la derecha de
                // la pantalla es la derecha de la lectura.
                let lectura_s = (centro_x - hit.point.x) / ALTO_DIGITO + ancho_del_bloque() / 2.0;
                let lectura_t = (hit.point.y - y0) / ALTO_DIGITO;
                if let Some(a) = anterior {
                    if lectura_s + 1e-4 < a {
                        orden_ok = false;
                    }
                }
                anterior = Some(lectura_s);
                let diseno = distancia_al_33(lectura_s, lectura_t) < 0.0;
                if es_tinta == diseno {
                    acuerdo += 1;
                }
                if es_tinta {
                    tinta += 1;
                    mascara[py * ancho + px] = true;
                    filas = (filas.0.min(py), filas.1.max(py));
                }
            }
        }
        // Componentes conexas de la tinta en pantalla (8-vecindad).
        let mut visto = vec![false; ancho * alto];
        let mut componentes = 0;
        for i in 0..ancho * alto {
            if !mascara[i] || visto[i] {
                continue;
            }
            componentes += 1;
            let mut pila = vec![i];
            visto[i] = true;
            while let Some(k) = pila.pop() {
                let (x, y) = ((k % ancho) as i32, (k / ancho) as i32);
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let (nx, ny) = (x + dx, y + dy);
                        if nx < 0 || ny < 0 || nx >= ancho as i32 || ny >= alto as i32 {
                            continue;
                        }
                        let j = ny as usize * ancho + nx as usize;
                        if mascara[j] && !visto[j] {
                            visto[j] = true;
                            pila.push(j);
                        }
                    }
                }
            }
        }
        let alto_px = if tinta > 0 { filas.1 - filas.0 + 1 } else { 0 };
        Lectura {
            tinta,
            acuerdo,
            en_placa,
            alto_px,
            componentes,
            orden_ok,
        }
    }

    /// Desde Praderas, a `800 x 600`: el 33 se lee **derecho y sin
    /// espejar** —la tinta del material efectivo coincide con el diseño
    /// leído en el orden de la pantalla—, en dos piezas y a buen tamaño.
    #[test]
    fn desde_praderas_el_33_se_lee_derecho_a_800x600() {
        let c = cand();
        let l = leer(c, &camara_desde_praderas(&c.diorama), ANCHO, ALTO);
        println!(
            "800x600: placa {} px, tinta {} px, acuerdo {:.3}, alto {} px, componentes {}",
            l.en_placa,
            l.tinta,
            l.acuerdo as f32 / l.en_placa.max(1) as f32,
            l.alto_px,
            l.componentes
        );
        assert!(
            l.orden_ok,
            "la derecha de la pantalla tiene que ser la derecha de la lectura"
        );
        assert!(
            l.en_placa > 800 && l.tinta > 150,
            "{} {}",
            l.en_placa,
            l.tinta
        );
        assert!(
            l.acuerdo as f32 >= 0.97 * l.en_placa as f32,
            "{} de {}",
            l.acuerdo,
            l.en_placa
        );
        assert_eq!(l.componentes, 2, "dos digitos");
        assert!(l.alto_px >= 28, "{} px", l.alto_px);
    }

    /// Y en el perfil interactivo de la ventana, `320 x 240`.
    #[test]
    fn desde_praderas_el_33_se_lee_a_320x240() {
        let c = cand();
        let p = InteractiveProfile::default();
        assert_eq!((p.width, p.height), (320, 240));
        let l = leer(c, &camara_desde_praderas(&c.diorama), p.width, p.height);
        println!(
            "320x240: placa {} px, tinta {} px, acuerdo {:.3}, alto {} px, componentes {}",
            l.en_placa,
            l.tinta,
            l.acuerdo as f32 / l.en_placa.max(1) as f32,
            l.alto_px,
            l.componentes
        );
        assert!(l.orden_ok);
        assert!(
            l.acuerdo as f32 >= 0.93 * l.en_placa as f32,
            "{} de {}",
            l.acuerdo,
            l.en_placa
        );
        assert!(l.alto_px >= 11, "{} px", l.alto_px);
    }

    /// Una sola vez y solo hacia Praderas: desde la hero, la cenital y los
    /// dos laterales no se ve tinta, ni cantos de la placa.
    #[test]
    fn el_33_solo_se_ve_hacia_praderas() {
        let c = cand();
        let d = &c.diorama;
        for (nombre, camara) in [
            ("hero", camara_hero(d)),
            ("cenital", camara_cenital(d)),
            ("lateral 90", camara_girada(d, 90)),
            ("lateral 270", camara_girada(d, 270)),
        ] {
            let l = leer(c, &camara, ANCHO, ALTO);
            println!("{nombre}: placa {} px, tinta {} px", l.en_placa, l.tinta);
            assert_eq!(l.tinta, 0, "{nombre}");
            assert!(l.en_placa <= 2, "{nombre}: {}", l.en_placa);
        }
        // Y ningún otro material usa la textura del número.
        let s = &d.scene;
        let usan: Vec<usize> = (0..s.palette.len())
            .filter(|&k| s.palette[k].albedo_texture == Some(c.textura_numero))
            .collect();
        assert_eq!(usan, [c.numero.0]);
        let con_numero: Vec<usize> = (0..s.objects.len())
            .filter(|&i| s.objects[i].final_material == c.numero)
            .collect();
        assert_eq!(con_numero, [c.placa]);
    }

    /// La textura del número: dos dígitos (dos piezas de tinta), en
    /// `Clamp`, y determinista.
    #[test]
    fn la_textura_del_33_tiene_dos_digitos_y_es_determinista() {
        let c = cand();
        let s = &c.diorama.scene;
        let t = &s.textures[c.textura_numero.0];
        assert_eq!(t.wrap, WrapMode::Clamp);
        let (w, h) = (t.width(), t.height());
        let tinta: Vec<bool> = (0..w * h)
            .map(|k| {
                let (x, y) = (k % w, k / w);
                luma(t.sample(
                    (x as f32 + 0.5) / w as f32,
                    1.0 - (y as f32 + 0.5) / h as f32,
                )) > 0.3
            })
            .collect();
        let mut visto = vec![false; w * h];
        let mut piezas = 0;
        for i in 0..w * h {
            if !tinta[i] || visto[i] {
                continue;
            }
            piezas += 1;
            let mut pila = vec![i];
            visto[i] = true;
            while let Some(k) = pila.pop() {
                let (x, y) = ((k % w) as i32, (k / w) as i32);
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let (nx, ny) = (x + dx, y + dy);
                    if nx >= 0 && ny >= 0 && (nx as usize) < w && (ny as usize) < h {
                        let j = ny as usize * w + nx as usize;
                        if tinta[j] && !visto[j] {
                            visto[j] = true;
                            pila.push(j);
                        }
                    }
                }
            }
        }
        assert_eq!(piezas, 2);
        let otra = candidata(actual(&raiz()));
        for id in [c.textura_carbon, c.textura_numero, c.textura_lienzo] {
            assert_eq!(
                texeles(&s.textures[id.0]),
                texeles(&otra.diorama.scene.textures[id.0])
            );
        }
        for id in [c.carbon, c.numero, c.lienzo_de_la_placa] {
            assert_eq!(
                format!("{:?}", s.material(id)),
                format!("{:?}", otra.diorama.scene.material(id))
            );
        }
    }

    /// La placa nace lienzo, como el Monolito: su material inicial es un
    /// lienzo con la textura horneada en las coordenadas de la cara, y sin
    /// revelar se confunde con ella.
    #[test]
    fn sin_revelar_la_placa_es_lienzo() {
        let c = cand();
        let s: &Scene = &c.diorama.scene;
        let o = &s.objects[c.placa];
        assert_eq!(o.initial_material, c.lienzo_de_la_placa);
        let (lienzo, propio) = (
            s.material(vigente().scene.objects[c.tramo].initial_material),
            s.material(c.lienzo_de_la_placa),
        );
        assert_eq!(
            (
                lienzo.specular_strength,
                lienzo.shininess,
                lienzo.reflection_cap,
                lienzo.transmission_cap
            ),
            (
                propio.specular_strength,
                propio.shininess,
                propio.reflection_cap,
                propio.transmission_cap
            )
        );
        // El color de la placa sin revelar es el de la cara en el mismo
        // punto del mundo, muestra a muestra.
        let placa = o.primitive.bounds();
        let tramo = s.objects[c.tramo].primitive.bounds();
        let mut iguales = 0;
        let lt = &s.textures[c.textura_lienzo.0];
        let (tw, th) = (lt.width(), lt.height());
        for k in 0..400 {
            // En centros de texel: ahí el horneado y la cara miran el mismo
            // punto del mundo.
            let (col, fila) = ((k % 20) * tw / 20, (k / 20) * th / 20);
            let (fu, fv) = (
                (col as f32 + 0.5) / tw as f32,
                (fila as f32 + 0.5) / th as f32,
            );
            let x = placa.min.x + fu * (placa.max.x - placa.min.x);
            let y = placa.min.y + fv * (placa.max.y - placa.min.y);
            let uv_cara = nalgebra_glm::Vec2::new(
                (x - tramo.min.x) / (tramo.max.x - tramo.min.x),
                (y - tramo.min.y) / (tramo.max.y - tramo.min.y),
            );
            let a = s.albedo_at(&lienzo, &uv_cara);
            let b = s.albedo_at(&propio, &nalgebra_glm::Vec2::new(fu, fv));
            if (luma(a) - luma(b)).abs() < 0.02 {
                iguales += 1;
            }
        }
        assert!(iguales >= 380, "{iguales}/400");
    }

    /// Sin revelar, desde Praderas: la placa se confunde con la cara. En
    /// los píxeles donde la candidata ve la placa, la diferencia de luma con
    /// la vigente —que ahí ve la cara de lienzo— es pequeña.
    #[test]
    fn sin_revelar_la_placa_no_se_nota_en_pantalla() {
        let c = cand();
        let camara = camara_desde_praderas(vigente());
        let reveal = RevealState::unpainted();
        let (a, _) = pintar_con(vigente(), &camara, ANCHO, ALTO, &reveal);
        let (b, _) = pintar_con(&c.diorama, &camara, ANCHO, ALTO, &reveal);
        let (suma, n) = diferencia_en_placa(c, &camara, &a, &b, false);
        println!(
            "sin revelar: diferencia media de luma en la placa {:.4} sobre {n} px",
            suma / n as f32
        );
        assert!(n > 800);
        assert!(suma / n as f32 <= 0.03, "{}", suma / n as f32);
    }

    /// Revelado: fuera de la tinta, la placa es el carbón de la cara. Se
    /// compara contra la variante `carbon`, que no tiene placa.
    #[test]
    fn revelada_la_placa_es_carbon_fuera_de_la_tinta() {
        let c = cand();
        let camara = camara_desde_praderas(vigente());
        let (carbon, _, _) = solo_carbon(actual(&raiz()));
        let (a, _) = pintar(&carbon, &camara, ANCHO, ALTO);
        let (b, _) = pintar(&c.diorama, &camara, ANCHO, ALTO);
        let (suma, n) = diferencia_en_placa(c, &camara, &a, &b, true);
        println!(
            "revelada, fuera de la tinta: diferencia media de luma {:.4} sobre {n} px",
            suma / n as f32
        );
        assert!(n > 500);
        assert!(suma / n as f32 <= 0.03, "{}", suma / n as f32);
    }

    /// Diferencia media de luma (sRGB empaquetado, 0..1) entre dos cuadros
    /// en los píxeles cuyo primer impacto en la candidata es la placa;
    /// `sin_tinta` descarta los píxeles de tinta y su borde.
    fn diferencia_en_placa(
        c: &Candidata,
        camara: &Camera,
        a: &Framebuffer,
        b: &Framebuffer,
        sin_tinta: bool,
    ) -> (f32, usize) {
        let d = &c.diorama;
        let s = &d.scene;
        let numero = s.material(c.numero);
        let placa = s.objects[c.placa].primitive.bounds();
        let centro_x = (placa.min.x + placa.max.x) / 2.0;
        let y0 = (placa.min.y + placa.max.y) / 2.0 - ALTO_DIGITO / 2.0;
        let l = |p: u32| {
            let (r, g, b) = (
                (p >> 16 & 0xFF) as f32,
                (p >> 8 & 0xFF) as f32,
                (p & 0xFF) as f32,
            );
            (0.2126 * r + 0.7152 * g + 0.0722 * b) / 255.0
        };
        let (mut suma, mut n) = (0.0, 0);
        for py in 0..ALTO {
            for px in 0..ANCHO {
                let rayo = camara.ray_from_pixel(px, py, ANCHO, ALTO);
                let mut st = Default::default();
                let Some(hit) = d.accel.intersect(s, &rayo, &mut st) else {
                    continue;
                };
                if hit.object_index != c.placa {
                    continue;
                }
                if sin_tinta {
                    let s_ = (centro_x - hit.point.x) / ALTO_DIGITO + ancho_del_bloque() / 2.0;
                    let t_ = (hit.point.y - y0) / ALTO_DIGITO;
                    // Fuera de la tinta y de su borde, con un píxel de holgura.
                    if distancia_al_33(s_, t_) < 0.06 || luma(s.albedo_at(&numero, &hit.uv)) > 0.06
                    {
                        continue;
                    }
                }
                let i = py * ANCHO + px;
                suma += (l(a.buffer[i]) - l(b.buffer[i])).abs();
                n += 1;
            }
        }
        (suma, n)
    }

    /// En el render, desde el cercano del lado de Praderas: la tinta se lee
    /// clara y cálida sobre un carbón oscuro que no es negro.
    #[test]
    fn la_tinta_es_marfil_calida_y_contrasta_con_el_carbon() {
        let c = cand();
        let d = &c.diorama;
        let s = &d.scene;
        let camara = camara_del_33(d);
        let (fb, _) = pintar(d, &camara, ANCHO, ALTO);
        let placa = s.objects[c.placa].primitive.bounds();
        let centro_x = (placa.min.x + placa.max.x) / 2.0;
        let y0 = (placa.min.y + placa.max.y) / 2.0 - ALTO_DIGITO / 2.0;
        let (mut tinta, mut fondo) = ([0.0f32; 3], [0.0f32; 3]);
        let (mut nt, mut nf) = (0usize, 0usize);
        for py in 0..ALTO {
            for px in 0..ANCHO {
                let rayo = camara.ray_from_pixel(px, py, ANCHO, ALTO);
                let mut st = Default::default();
                let Some(hit) = d.accel.intersect(s, &rayo, &mut st) else {
                    continue;
                };
                if hit.object_index != c.placa {
                    continue;
                }
                let s_ = (centro_x - hit.point.x) / ALTO_DIGITO + ancho_del_bloque() / 2.0;
                let t_ = (hit.point.y - y0) / ALTO_DIGITO;
                let dist = distancia_al_33(s_, t_);
                let p = fb.buffer[py * ANCHO + px];
                let rgb = [
                    (p >> 16 & 0xFF) as f32 / 255.0,
                    (p >> 8 & 0xFF) as f32 / 255.0,
                    (p & 0xFF) as f32 / 255.0,
                ];
                let (acc, n) = if dist < -0.02 {
                    (&mut tinta, &mut nt)
                } else if dist > 0.06 {
                    (&mut fondo, &mut nf)
                } else {
                    continue;
                };
                for k in 0..3 {
                    acc[k] += rgb[k];
                }
                *n += 1;
            }
        }
        let media = |a: [f32; 3], n: usize| [a[0] / n as f32, a[1] / n as f32, a[2] / n as f32];
        let (t, f) = (media(tinta, nt), media(fondo, nf));
        let l = |c: [f32; 3]| 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2];
        println!(
            "tinta {t:?} ({nt} px) fondo {f:?} ({nf} px) contraste {:.2}",
            l(t) / l(f)
        );
        assert!(nt > 2000 && nf > 2000);
        assert!(l(t) / l(f) >= 2.5, "contraste {}", l(t) / l(f));
        assert!(t[0] > t[2] + 0.08, "calida: {t:?}");
        assert!(l(f) > 0.08 && l(f) < 0.35, "carbon oscuro, no negro: {f:?}");
    }

    /// La promoción: con los assets reales y en los tres presets,
    /// `delivery_level_con` es `carbon_33` exacta sobre la línea base de 218
    /// —objetos, materiales efectivos, texturas texel a texel, anclas,
    /// escala, cámaras, luces y jerarquía—.
    #[test]
    fn la_entrega_de_produccion_es_carbon_33() {
        use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
        use expedition33_continente_inacabado::scenes::WaterPreset;
        for (water, total) in [
            (WaterPreset::RefractiveWater, 219),
            (WaterPreset::OpaqueWater, 219),
            (WaterPreset::InteriorVisible, 218),
        ] {
            let aprobada = candidata(actual_con(water, &raiz()));
            let (x, y) = (&aprobada.diorama, &produccion_con(water, &raiz()));
            assert_eq!(y.scene.objects.len(), total, "{water:?}");
            assert_eq!(
                format!("{:?}", x.scene.objects),
                format!("{:?}", y.scene.objects),
                "{water:?}"
            );
            assert_eq!(
                format!("{:?}", x.scene.palette),
                format!("{:?}", y.scene.palette),
                "{water:?}"
            );
            assert_eq!(x.scene.textures.len(), y.scene.textures.len());
            for (i, (a, b)) in x.scene.textures.iter().zip(&y.scene.textures).enumerate() {
                assert_eq!(
                    (a.width(), a.height(), a.wrap),
                    (b.width(), b.height(), b.wrap),
                    "textura {i}"
                );
                assert_eq!(texeles(a), texeles(b), "{water:?} textura {i}");
            }
            assert_eq!(
                format!("{:?}", x.scene.skybox).len(),
                format!("{:?}", y.scene.skybox).len()
            );
            assert_eq!(
                format!("{:?}", x.anchors),
                format!("{:?}", y.anchors),
                "{water:?}"
            );
            assert_eq!(
                format!("{:?}", x.scale),
                format!("{:?}", y.scale),
                "{water:?}"
            );
            for (cx, cy) in [
                (camara_hero(x), camara_hero(y)),
                (camara_cenital(x), camara_cenital(y)),
                (camara_desde_praderas(x), camara_desde_praderas(y)),
                (camara_del_33(x), camara_del_33(y)),
            ] {
                assert_eq!(format!("{cx:?}"), format!("{cy:?}"), "{water:?}");
            }
            assert_eq!(
                format!("{:?}", luces_del_diorama(&x.anchors, &x.scale)),
                format!("{:?}", luces_del_diorama(&y.anchors, &y.scale))
            );
            assert_eq!(
                format!("{:?}", x.accel),
                format!("{:?}", y.accel),
                "{water:?}"
            );
        }
    }

    /// Y en el render: desde Praderas a `320 x 240`, producción y la
    /// candidata dan el mismo cuadro bit a bit.
    #[test]
    fn la_entrega_de_produccion_se_ve_como_carbon_33() {
        use expedition33_continente_inacabado::scenes::WaterPreset;
        let p = InteractiveProfile::default();
        let y = produccion_con(WaterPreset::RefractiveWater, &raiz());
        let x = &cand().diorama;
        let camara = camara_desde_praderas(x);
        let (a, _) = pintar(x, &camara, p.width, p.height);
        let (b, _) = pintar(&y, &camara, p.width, p.height);
        assert!(a.buffer == b.buffer);
    }
}
