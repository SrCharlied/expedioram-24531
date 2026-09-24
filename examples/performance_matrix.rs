//! Matriz de rendimiento de la Tarea 7.1: cuánto presupuesto existe de
//! verdad antes de gastar la reserva del Hito 7.
//!
//! ```text
//! cargo run --release --example performance_matrix
//! ```
//!
//! # Los cinco presets del plan
//!
//! El plan nombra cinco presets y no los define. Cada fila declara aquí su
//! definición exacta —volumen de agua y estado de revelación— porque son
//! las dos dimensiones que mueven el coste, y confundirlas es lo que dio un
//! benchmark optimista en el Hito 3:
//!
//! | Nombre del plan | Volumen | Revelación |
//! |---|---|---|
//! | `safe-canvas` | sin volumen, `153` primitivas | lienzo, `0.0` |
//! | `safe-painted` | sin volumen, `153` primitivas | pintado, `1.0` |
//! | `safe-water` | refractivo, `154` primitivas | pintado, `1.0` |
//! | `safe-revealing` | refractivo, `154` primitivas | `worst_case()` |
//! | `target-water` | refractivo, el candidato de `TARGET` | pintado, `1.0` |
//!
//! Y una sexta que el plan no nombra y la Tarea 7.2 necesita:
//! `target-revealing`, el candidato en el peor estado. Es la fila que decide,
//! porque es la que se compara contra el gate.
//!
//! Las filas están ordenadas de forma que cada una añade **un** cambio sobre
//! la anterior: primero los materiales pintados, luego el volumen
//! refractivo, luego el doble muestreo de la transición, y al final la
//! densidad del nivel objetivo. Así la diferencia entre dos filas
//! consecutivas es atribuible a un solo cambio.
//!
//! Un escalón mal nombrado, corregido por la columna de rayos: el primero
//! **no es «las texturas»**. Al pasar de lienzo a pintado se encienden a la
//! vez las texturas y los techos ópticos de los materiales finales, y los
//! conteos dicen cuál pesa: los rayos secundarios por cuadro pasan de `597`
//! a `8 819`. Es la óptica, no el muestreo.
//!
//! El doble muestreo de la transición lo confirma por el otro lado: añade
//! `231` rayos y un `2 %` a `6 %` de tiempo. El conteo de primitivas y el
//! conteo de rayos son dos presupuestos distintos, y el caro es el segundo.
//!
//! El lote de la Tarea 7.2 empuja en la dirección contraria: sus primitivas
//! **restan** rayos secundarios, porque ocluyen parte de la cara frontal del
//! agua. Eso **compensa parte** de lo que cuestan de recorrido; no prueba
//! que el coste neto baje. La columna de tiempo es la que decide, y en las
//! corridas registradas el neto salió positivo.
//!
//! `target-water` ya se mide: es el candidato incremental de la Tarea 7.2
//! que esté en evaluación, y el conteo sale de `TARGET`. **No es lo que se
//! envía**: el nivel seguro está en `154` desde que se retiró `G-04`, y es
//! el que abre la
//! ventana. El candidato vive para poder medirlo y mirarlo antes de decidir
//! si se conserva, y para poder retirarlo cambiando un parámetro.
//!
//! # Método
//!
//! Release, o la comparación no significa nada.
//!
//! **Tres bloques.** El cuadro final a `800 x 600` y el perfil interactivo con
//! la resolución que el programa envía, que son los dos regímenes reales,
//! los dos en la toma hero para que las filas sean comparables entre sí. Y
//! un tercero con la **rejilla de cuarenta y ocho cámaras** sobre la fila que
//! decide, porque un presupuesto medido en un solo encuadre promete un margen
//! que el primer giro puede gastarse.
//!
//! **Rondas intercaladas y rotadas.** Un cuadro de cada celda por ronda, y
//! el orden de la ronda rota. Intercalar reparte la deriva térmica; rotar
//! reparte la posición dentro de la ronda, que no es neutra. Sin rotación la
//! instrumentación favorece sistemáticamente a las primeras filas, que es
//! precisamente el sesgo que esta versión corrige.
//!
//! **Estadística.** Quince rondas y `stats::summarize`, que calcula bien la
//! mediana de un conteo par. Las atribuciones entre filas usan
//! `stats::median_ratio`: cociente ronda contra ronda y mediana de los
//! cocientes, no cociente de medianas.
//!
//! **Conteos de rayos.** Junto a cada celda se registran los rayos
//! secundarios por cuadro. Sin ellos, decir *por qué* una fila cuesta más
//! que la anterior sería una hipótesis; con ellos es una medición.
//!
//! Los valores absolutos de bloques distintos **no** son comparables entre
//! sí: cada bloque deja la máquina más caliente para el siguiente. Lo
//! comparable es lo de dentro de un bloque, que es donde está el intercalado.

use std::path::PathBuf;
use std::time::Instant;

use expedition33_continente_inacabado::accel::TraversalStats;
use expedition33_continente_inacabado::brush::{BrushMasks, PigmentMasks, SurfaceKey};
use expedition33_continente_inacabado::camera::Camera;
use expedition33_continente_inacabado::color::Color;
use expedition33_continente_inacabado::framebuffer::Framebuffer;
use expedition33_continente_inacabado::input::{pick_artistic, PresentedFrame};
use expedition33_continente_inacabado::light::{diorama as luces_del_diorama, PointLight};
use expedition33_continente_inacabado::renderer::{
    render, render_artistic, InteractiveProfile, Shading,
};
use expedition33_continente_inacabado::reveal::{
    reveal_duration, RevealState, MINIMUM_REVEAL_FRAMES, REVEAL_DURATION_CEILING,
    WORST_CASE_PROGRESS,
};
use expedition33_continente_inacabado::scene_builder::Blockout;
use expedition33_continente_inacabado::scenes::{
    safe_level_con, target_level_con, Density, WaterPreset, SAFE, TARGET,
};
use expedition33_continente_inacabado::stats::{median_ratio, summarize};

const ANCHO: usize = 800;
const ALTO: usize = 600;

/// Rondas por celda. **Impar**: la mediana es un valor observado.
const RONDAS: usize = 15;

/// Resolución de las máscaras artísticas, la misma que usa la ventana.
const BRUSH_RESOLUTION: usize = 128;

/// Radio del pincel como fracción de `scene_radius`, el mismo que arranca en
/// la ventana. Se escribe aquí y no se importa: `main.rs` lo declara privado
/// y detrás de una feature, así que copiarlo con su procedencia dicha es más
/// honesto que fingir que se comparte.
const BRUSH_RADIUS_FACTOR: f32 = 0.025;

/// Radio mínimo de órbita del modo artístico, como factor de `scene_radius`.
///
/// Es el valor que `main.rs` aplica bajo `artistic-brush`. Igual que el radio
/// del pincel, se declara aquí con su procedencia en vez de leerse: la
/// constante de `main` es privada y este ejemplo no puede —ni debe— tocarla.
/// Si allí cambia, esta cifra deja de describir lo que se envía, y por eso se
/// imprime junto a la tabla.
const ARTISTIC_MIN_RADIUS_FACTOR: f32 = 1.7;

/// Rejilla de cursores con la que se construye la fixture artística.
const REJILLA: (usize, usize) = (9, 7);

/// Margen mínimo que se le exige al candidato sobre el crítico del gate.
///
/// Es el umbral operativo de la Tarea 7.1, y se mantiene para la 7.2 por una
/// razón que va más allá de este lote: **quedan lotes posteriores**, y el
/// renderer mide `render()`, no todo el coste de presentar un cuadro. Gastar
/// la reserva hasta el borde en el primer lote dejaría el segundo sin sitio y
/// el gate sin colchón para lo que la medición no ve.
const MARGEN_MINIMO: f64 = 1.30;

/// Con qué renderer se mide una celda.
///
/// Los tres son APIs de la librería y ninguno depende de la feature
/// `artistic-brush`: lo que esa feature enciende es el pincel de la ventana,
/// no el renderer. Medir los tres aquí es lo que permite separar el coste de
/// **tener** las capas del de **usarlas**.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Modo {
    /// `render`: lo que se envía hoy.
    Base,
    /// `render_artistic` con las dos máscaras vacías. Aísla el coste de
    /// consultar los dos mapas en cada impacto, sin pintura que componer.
    ArtisticoVacio,
    /// `render_artistic` con la fixture pintada sobre superficies reales.
    ArtisticoPintado,
}

impl Modo {
    fn etiqueta(self) -> &'static str {
        match self {
            Modo::Base => "base",
            Modo::ArtisticoVacio => "art-vacio",
            Modo::ArtisticoPintado => "art-pintado",
        }
    }
}

/// Las dos capas artísticas y las superficies que tocan.
struct Artistico {
    masks: BrushMasks,
    pigment: PigmentMasks,
    /// Las `SurfaceKey` pintadas, en el orden en que el picking las
    /// encontró. Se guardan para poder afirmar de dónde salieron.
    claves: Vec<SurfaceKey>,
}

impl Artistico {
    /// Contenedores vacíos, para el escenario que aísla el coste de consulta.
    fn vacio() -> Self {
        Artistico {
            masks: BrushMasks::new(BRUSH_RESOLUTION, BRUSH_RESOLUTION),
            pigment: PigmentMasks::new(BRUSH_RESOLUTION, BRUSH_RESOLUTION),
            claves: Vec::new(),
        }
    }
}

/// El encuadre más cercano que permite el modo artístico.
///
/// La órbita del modo artístico baja su mínimo a `ARTISTIC_MIN_RADIUS_FACTOR`
/// veces `scene_radius`, frente al `1.8` del modo base. Para llegar ahí hay
/// que bajar **primero** el límite de la cámara: `Camera::zoom` recorta contra
/// `min_radius`, así que sin ese paso la cámara se detendría en el mínimo del
/// modo base y el bloque mediría un encuadre distinto del que dice medir.
///
/// La camara se deriva aquí y no se lee de `main.rs`: aquella constante es
/// privada y vive detrás de una feature. Lo que este ejemplo garantiza es que
/// el radio resultante **es** el factor declarado, y eso lo fija un test.
fn camara_de_zoom_artistico(diorama: &Blockout) -> Camera {
    let objetivo = diorama.scale.scene_radius * ARTISTIC_MIN_RADIUS_FACTOR;
    let hero = diorama.hero_camera();
    let mut cercana = hero.with_radius_limits(objetivo, hero.max_radius);

    cercana.zoom(objetivo - cercana.radius());

    cercana
}

/// Los cursores de la rejilla, en píxeles de ventana.
///
/// Fijos y derivados del tamaño: la fixture tiene que salir igual en cada
/// corrida, o las tres columnas de la tabla dejarían de ser comparables.
fn cursores_de_la_fixture(ancho: usize, alto: usize) -> Vec<(f32, f32)> {
    let (columnas, filas) = REJILLA;
    let mut cursores = Vec::with_capacity(columnas * filas);

    for fila in 0..filas {
        for columna in 0..columnas {
            cursores.push((
                (columna as f32 + 0.5) * ancho as f32 / columnas as f32,
                (fila as f32 + 0.5) * alto as f32 / filas as f32,
            ));
        }
    }

    cursores
}

/// Pinta la fixture artística sobre superficies **reales** del diorama.
///
/// Recorre una rejilla de cursores con el mismo `pick_artistic` que usa la
/// ventana, y sobre cada impacto aplica revelado y pigmento. Las claves salen
/// del picking, no de recorrer el vector de objetos: así la fixture toca las
/// superficies que de verdad se ven desde esa cámara, con las cartas que
/// entregan sus primitivas.
///
/// El radio en `uv` se deriva de la métrica de cada superficie, igual que en
/// la ventana, así que la huella mide lo mismo sobre una losa enorme y sobre
/// un tablón.
///
/// Las texturas se **leen**: `stamp_texture` muestrea y estampa el color en
/// la capa. Ninguna textura de la escena se modifica.
fn fixture_artistica(diorama: &Blockout, camara: &Camera, ancho: usize, alto: usize) -> Artistico {
    let mut artistico = Artistico::vacio();
    let cuadro = PresentedFrame::full(*camara, (ancho, alto));
    let radio_mundo = diorama.scale.scene_radius * BRUSH_RADIUS_FACTOR;

    // Tres pigmentos planos y una tela, alternados por orden de impacto para
    // que la fixture ejercite las dos rutas de composición.
    let paleta = [
        Color::from_srgb(0.66, 0.09, 0.15),
        Color::from_srgb(0.85, 0.65, 0.17),
        Color::from_srgb(0.18, 0.62, 0.68),
    ];

    for (i, cursor) in cursores_de_la_fixture(ancho, alto).into_iter().enumerate() {
        let Some(objetivo) = pick_artistic(&diorama.scene, &diorama.accel, &cuadro, cursor) else {
            continue;
        };

        let clave = SurfaceKey::new(objetivo.object_index, objetivo.uv_chart);
        let Some((radio_u, radio_v)) = objetivo.uv_world_scale.uv_radii(radio_mundo) else {
            continue;
        };
        let (u, v) = (objetivo.uv.x, objetivo.uv.y);

        // Revelado en todas: es la herramienta por defecto de la ventana.
        artistico
            .masks
            .stamp_ellipse(clave, u, v, radio_u, radio_v, 1.0);

        // Y pigmento alternando plano y tela, para que la tabla incluya el
        // coste de las dos composiciones y no solo de la barata.
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

/// Imprime un bloque artístico: las tres columnas de una misma escena y
/// cámara, con los conteos y las superficies que toca la fixture.
///
/// No compara contra ningún umbral ni dice si cabe: eso lo decide quien lea
/// la tabla. Lo que se imprime es lo que hace falta para decidirlo.
fn reportar_artistico(
    celdas: &[Celda],
    niveles: &[Blockout],
    artistico: &Artistico,
    ancho: usize,
    alto: usize,
    titulo: &str,
) {
    println!("\n  {titulo}   {ancho} x {alto}");
    println!(
        "  fixture representativa: {} superficies con revelado, {} con pigmento, {} claves distintas",
        artistico.masks.len(),
        artistico.pigment.len(),
        artistico.claves.len()
    );
    println!(
        "  {:<14} {:>6} {:>9} {:>9} {:>9} {:>6} {:>12}",
        "modo", "prim.", "minimo", "mediana", "maximo", "fps", "2os rayos"
    );

    for celda in celdas {
        let d = summarize(&celda.tiempos);
        let primitivas = niveles[celda.nivel].scene.objects.len();

        println!(
            "  {:<14} {:>6} {:>9.4} {:>9.4} {:>9.4} {:>6.1} {:>12}",
            celda.modo.etiqueta(),
            primitivas,
            d.min,
            d.median,
            d.max,
            1.0 / d.median,
            celda.secundarios()
        );
    }

    // Cocientes **pareados**: ronda contra ronda, que es lo que cancela la
    // deriva térmica de cada vuelta. Ver `stats::median_ratio`.
    if celdas.len() == 3 {
        let base = &celdas[0].tiempos;

        println!(
            "  cociente pareado contra base:  vacio {:.3}x   pintado {:.3}x",
            median_ratio(&celdas[1].tiempos, base),
            median_ratio(&celdas[2].tiempos, base)
        );
    }
}

/// Una celda de la matriz: un nivel, un estado, una cámara y su muestra.
struct Celda {
    nombre: String,
    /// Índice del blockout: `0` sin volumen, `1` refractivo.
    nivel: usize,
    reveal: RevealState,
    camara: Camera,
    /// Qué renderer mide. Ver `Modo`.
    modo: Modo,
    tiempos: Vec<f64>,
    /// Contadores del último cuadro trazado. Son deterministas para un
    /// estado y una cámara dados, así que uno basta.
    stats: TraversalStats,
}

impl Celda {
    fn nueva(nombre: String, nivel: usize, reveal: RevealState, camara: Camera) -> Self {
        Celda::con_modo(nombre, nivel, reveal, camara, Modo::Base)
    }

    fn con_modo(
        nombre: String,
        nivel: usize,
        reveal: RevealState,
        camara: Camera,
        modo: Modo,
    ) -> Self {
        Celda {
            nombre,
            nivel,
            reveal,
            camara,
            modo,
            tiempos: Vec::with_capacity(RONDAS),
            stats: TraversalStats::default(),
        }
    }

    /// Rayos secundarios por cuadro.
    fn secundarios(&self) -> usize {
        self.stats.reflection_rays + self.stats.refraction_rays
    }
}

fn nivel(preset: WaterPreset, densidad: Density) -> Blockout {
    let raiz = PathBuf::from(".");

    let construido = match densidad {
        Density::Safe => safe_level_con(preset, Some(&raiz)),
        Density::Target => target_level_con(preset, Some(&raiz)),
    };

    match construido {
        Ok(nivel) => nivel,
        Err(e) => {
            eprintln!("error: {e}");
            eprintln!("  la matriz mide la escena texturizada; sin assets mediria otra.");
            eprintln!("  generalos con: cargo run --release --bin generate_assets");
            std::process::exit(1);
        }
    }
}

/// Mide las celdas, un cuadro de cada una por ronda, **rotando** el orden.
///
/// `tiempos[r]` queda siendo la ronda `r` para todas las celdas, que es lo
/// que hace válidos los cocientes pareados.
fn medir(
    celdas: &mut [Celda],
    niveles: &[Blockout],
    luces: &[Vec<PointLight>],
    ancho: usize,
    alto: usize,
) {
    medir_con(celdas, niveles, luces, ancho, alto, &Artistico::vacio());
}

/// Como `medir`, con una fixture artística concreta para las celdas que la
/// pidan.
///
/// La fixture se presta: no se construye ni se clona por ronda, así que lo
/// que se cronometra es el trazado y no su preparación.
fn medir_con(
    celdas: &mut [Celda],
    niveles: &[Blockout],
    luces: &[Vec<PointLight>],
    ancho: usize,
    alto: usize,
    artistico: &Artistico,
) {
    let mut framebuffer = Framebuffer::new(ancho, alto);
    let n = celdas.len();

    for celda in celdas.iter_mut() {
        celda.tiempos.clear();
    }

    for ronda in 0..RONDAS {
        for k in 0..n {
            let i = (k + ronda) % n;
            let diorama = &niveles[celdas[i].nivel];

            let vacio = Artistico::vacio();
            let capas = match celdas[i].modo {
                Modo::ArtisticoPintado => artistico,
                _ => &vacio,
            };

            let inicio = Instant::now();
            let stats = match celdas[i].modo {
                Modo::Base => render(
                    &mut framebuffer,
                    &diorama.scene,
                    &diorama.accel,
                    &luces[celdas[i].nivel],
                    &celdas[i].reveal,
                    &celdas[i].camara,
                    Shading::Material,
                ),
                Modo::ArtisticoVacio | Modo::ArtisticoPintado => render_artistic(
                    &mut framebuffer,
                    &diorama.scene,
                    &diorama.accel,
                    &luces[celdas[i].nivel],
                    &celdas[i].reveal,
                    &celdas[i].camara,
                    Shading::Material,
                    &capas.masks,
                    &capas.pigment,
                ),
            };
            celdas[i].tiempos.push(inicio.elapsed().as_secs_f64());
            celdas[i].stats = stats;
        }
    }
}

fn reportar(celdas: &[Celda], niveles: &[Blockout], ancho: usize, alto: usize, titulo: &str) {
    println!("\n  {titulo}   {ancho} x {alto}");
    println!(
        "  {:<18} {:>6} {:>9} {:>9} {:>9} {:>6} {:>12}",
        "celda", "prim.", "minimo", "mediana", "maximo", "fps", "2os rayos"
    );

    for celda in celdas {
        let d = summarize(&celda.tiempos);

        println!(
            "  {:<18} {:>6} {:>9.4} {:>9.4} {:>9.4} {:>6.1} {:>12}",
            celda.nombre,
            niveles[celda.nivel].scene.objects.len(),
            d.min,
            d.median,
            d.max,
            1.0 / d.median,
            celda.secundarios()
        );
    }
}

/// Imprime los escalones entre filas consecutivas, con cociente pareado.
/// Imprime el escalón entre filas consecutivas, **solo cuando es un
/// escalón**.
///
/// Un par de filas es atribuible si cambia una cosa: o la geometría del
/// nivel, o el estado de revelación. El par `safe-revealing → target-water`
/// cambia **las dos**, así que su cociente no mide la densidad del lote ni
/// el doble muestreo de la transición: mide la suma de ambos con signos
/// distintos. Imprimirlo en la misma columna que los demás invitaba a
/// leerlo como el coste del lote, que es justo lo que no es.
///
/// La comparación honesta del lote es **mismo estado**: `target-revealing`
/// contra `safe-revealing`, que sí cambia solo la geometría. La imprime el
/// bloque del presupuesto.
fn escalones(celdas: &[Celda], etiqueta: &str) {
    println!("\n  escalones {etiqueta} (cociente pareado, ronda contra ronda)");

    for par in celdas.windows(2) {
        let (antes, despues) = (&par[0], &par[1]);

        if antes.nivel != despues.nivel && antes.reveal != despues.reveal {
            println!(
                "  {:<18} -> {:<18}   no es un escalon: cambian geometria y reveal",
                antes.nombre, despues.nombre
            );
            continue;
        }

        let cociente = median_ratio(&despues.tiempos, &antes.tiempos);
        let rayos = despues.secundarios() as i64 - antes.secundarios() as i64;

        println!(
            "  {:<18} -> {:<18} {:+6.1} %   2os rayos {rayos:+}",
            antes.nombre,
            despues.nombre,
            100.0 * (cociente - 1.0)
        );
    }
}

fn main() {
    let niveles = [
        nivel(WaterPreset::InteriorVisible, Density::Safe),
        nivel(WaterPreset::RefractiveWater, Density::Safe),
        nivel(WaterPreset::RefractiveWater, Density::Target),
    ];
    let luces: Vec<Vec<PointLight>> = niveles
        .iter()
        .map(|n| luces_del_diorama(&n.anchors, &n.scale))
        .collect();

    // El perfil que envia el programa. Ver el bloque de mitigaciones.
    let perfil = InteractiveProfile::default();
    let hero = niveles[1].hero_camera();

    let presets = || {
        vec![
            Celda::nueva("safe-canvas".to_string(), 0, RevealState::unpainted(), hero),
            Celda::nueva("safe-painted".to_string(), 0, RevealState::painted(), hero),
            Celda::nueva("safe-water".to_string(), 1, RevealState::painted(), hero),
            Celda::nueva(
                "safe-revealing".to_string(),
                1,
                RevealState::worst_case(),
                hero,
            ),
            Celda::nueva("target-water".to_string(), 2, RevealState::painted(), hero),
            Celda::nueva(
                "target-revealing".to_string(),
                2,
                RevealState::worst_case(),
                hero,
            ),
        ]
    };

    println!("performance_matrix · Tarea 7.1\n");
    println!("  release     si, obligatorio");
    println!(
        "  escena      {} texturas, {} luces",
        niveles[1].scene.textures.len(),
        luces[1].len()
    );
    println!("  rondas      {RONDAS}, intercaladas y con el orden rotado");
    println!("  peor estado Continente pintado y grupo Finale en {WORST_CASE_PROGRESS:.2}");
    println!("  comando     cargo run --release --example performance_matrix");

    // ---------------------------------------------- bloque 1: cuadro final
    let mut finales = presets();
    medir(&mut finales, &niveles, &luces, ANCHO, ALTO);
    reportar(&finales, &niveles, ANCHO, ALTO, "cuadro final, toma hero");
    escalones(&finales, "a resolucion completa");

    // ------------------------------------------ bloque 2: perfil interactivo
    let mut interactivas = presets();
    medir(
        &mut interactivas,
        &niveles,
        &luces,
        perfil.width,
        perfil.height,
    );
    reportar(
        &interactivas,
        &niveles,
        perfil.width,
        perfil.height,
        "perfil interactivo, toma hero",
    );
    escalones(&interactivas, "en el perfil interactivo");

    // ---------------------------------------------- bloque 3: las cámaras
    //
    // Sobre el **candidato**, que es la fila que decide: si el lote no cabe
    // en el peor encuadre, no cabe.
    let mut camaras: Vec<Celda> = niveles[2]
        .measurement_cameras()
        .into_iter()
        .map(|(etiqueta, camara)| Celda::nueva(etiqueta, 2, RevealState::worst_case(), camara))
        .collect();

    medir(&mut camaras, &niveles, &luces, perfil.width, perfil.height);
    reportar(
        &camaras,
        &niveles,
        perfil.width,
        perfil.height,
        "target-revealing en la rejilla de 48 camaras",
    );

    let peor_camara = camaras
        .iter()
        .max_by(|a, b| {
            summarize(&a.tiempos)
                .median
                .partial_cmp(&summarize(&b.tiempos).median)
                .expect("no hay NaN")
        })
        .expect("hay camaras");

    // La hero **no** es la primera de la rejilla: se pide por indice y no se
    // asume. Una version anterior dividia por `camaras[0]`, que es
    // `y+0 e-84 cerca` —la vista desde debajo del plinto, la mas barata de
    // las cuarenta y ocho—, e inflaba el cociente sin avisar.
    let hero_i = niveles[2].hero_index();

    println!(
        "\n  peor camara  {}   {:.4} s   ({:+.1} % sobre la hero, pareado)",
        peor_camara.nombre,
        summarize(&peor_camara.tiempos).median,
        100.0 * (median_ratio(&peor_camara.tiempos, &camaras[hero_i].tiempos) - 1.0)
    );

    // ------------------------------------------------ el presupuesto
    //
    // El gate que puede fallar de verdad es el de fluidez, y solo mira el
    // perfil interactivo: el cuadro final se produce una vez al soltar los
    // controles y nadie lo anima.
    //
    // Se toma el peor de los dos bloques interactivos, que es lo que hace
    // que el presupuesto no dependa de haber medido el encuadre afortunado.
    let critico = REVEAL_DURATION_CEILING / MINIMUM_REVEAL_FRAMES;
    let peor = summarize(&interactivas[5].tiempos)
        .median
        .max(summarize(&peor_camara.tiempos).median);

    // Lo que el lote costó, pareado y en la toma hero, que es donde las dos
    // filas son comparables.
    let coste_del_lote = median_ratio(&interactivas[5].tiempos, &interactivas[3].tiempos);

    println!("\n  el lote de la Tarea 7.2");
    println!(
        "  +{} primitivas cuestan      {:+.1} %   (target-revealing vs safe-revealing, pareado)",
        TARGET.total() - SAFE.total(),
        100.0 * (coste_del_lote - 1.0)
    );
    println!(
        "  rayos secundarios          {:+}",
        interactivas[5].secundarios() as i64 - interactivas[3].secundarios() as i64
    );

    println!("\n  presupuesto del candidato");
    println!("  peor cuadro interactivo    {peor:.4} s");
    println!(
        "  critico del gate           {critico:.4} s   (15 cuadros en {REVEAL_DURATION_CEILING:.1} s)"
    );
    let reserva = critico as f64 / peor;

    println!("  reserva                    {reserva:.2}x en tiempo");

    // El veredicto llega al codigo de salida. Una version anterior lo
    // imprimia y salia con cero: la matriz podia decir que el candidato no
    // cabe y terminar en exito, que es la misma puerta lateral que la Tarea
    // 7.1 cerro en la fase 3 del otro ejemplo.
    let cabe = reserva >= MARGEN_MINIMO;

    if cabe {
        println!("  umbral operativo           {MARGEN_MINIMO:.2}x   ->  el lote cabe");
    } else {
        println!(
            "\n  AVISO: el candidato deja {reserva:.2}x y el umbral operativo es {MARGEN_MINIMO:.2}x."
        );
        println!("  El lote hay que retirarlo o recortarlo.");
    }

    match reveal_duration(peor as f32) {
        Ok(duracion) => println!(
            "  reveal_duration            {duracion:.2} s   ({:.0} cuadros)",
            duracion / peor as f32
        ),
        Err(_) => {
            println!("  FALLA el gate de fluidez con el peor cuadro.");
            std::process::exit(1);
        }
    }

    // ------------------------------------------- bloques artisticos
    //
    // Tres columnas por escenario, siempre sobre la **misma** escena, cámara
    // y estado: lo único que cambia es qué renderer las traza y si las capas
    // llevan pintura. Así la diferencia entre columnas es atribuible dentro
    // de la corrida, que es lo que los cocientes pareados saben medir.
    let artistico_hero = fixture_artistica(&niveles[1], &hero, ANCHO, ALTO);

    let artisticas = |camara: Camera| {
        vec![
            Celda::con_modo(
                "safe-revealing".to_string(),
                1,
                RevealState::worst_case(),
                camara,
                Modo::Base,
            ),
            Celda::con_modo(
                "safe-revealing".to_string(),
                1,
                RevealState::worst_case(),
                camara,
                Modo::ArtisticoVacio,
            ),
            Celda::con_modo(
                "safe-revealing".to_string(),
                1,
                RevealState::worst_case(),
                camara,
                Modo::ArtisticoPintado,
            ),
        ]
    };

    println!("\n\n  ====== modo artistico ======");
    println!("  render_artistic, BrushMasks y PigmentMasks son API de la libreria:");
    println!("  no dependen de la feature artistic-brush, que solo enciende el");
    println!("  pincel de la ventana. Por eso esta tabla sale igual en las dos rutas.");
    println!(
        "  mascaras    {BRUSH_RESOLUTION} x {BRUSH_RESOLUTION} por superficie, radio {:.3} de mundo",
        niveles[1].scale.scene_radius * BRUSH_RADIUS_FACTOR
    );
    println!(
        "  fixture     rejilla de {} x {} cursores resueltos con pick_artistic",
        REJILLA.0, REJILLA.1
    );

    let mut art_final = artisticas(hero);
    medir_con(
        &mut art_final,
        &niveles,
        &luces,
        ANCHO,
        ALTO,
        &artistico_hero,
    );
    reportar_artistico(
        &art_final,
        &niveles,
        &artistico_hero,
        ANCHO,
        ALTO,
        "cuadro final, toma hero",
    );

    let artistico_perfil = fixture_artistica(&niveles[1], &hero, perfil.width, perfil.height);
    let mut art_perfil = artisticas(hero);
    medir_con(
        &mut art_perfil,
        &niveles,
        &luces,
        perfil.width,
        perfil.height,
        &artistico_perfil,
    );
    reportar_artistico(
        &art_perfil,
        &niveles,
        &artistico_perfil,
        perfil.width,
        perfil.height,
        "perfil interactivo, toma hero",
    );

    // ------------------------------------------ el zoom mas cercano
    //
    // El modo artistico baja el minimo de orbita a `1.7 x scene_radius`,
    // frente al `1.8` del modo base. Acercarse llena mas el cuadro de
    // geometria, y la Tarea 7.1 identifico el encuadre como el factor
    // dominante: por eso este encuadre tiene su propio bloque.
    //
    // La camara se deriva aqui, no se lee de `main.rs`: aquella constante es
    // privada y esta detras de una feature. La cifra que se imprime es la que
    // usa este ejemplo, y si alla cambia, esta tabla deja de describirla.
    let cercana = camara_de_zoom_artistico(&niveles[1]);

    let artistico_cerca = fixture_artistica(&niveles[1], &cercana, ANCHO, ALTO);
    let mut art_cerca = artisticas(cercana);
    medir_con(
        &mut art_cerca,
        &niveles,
        &luces,
        ANCHO,
        ALTO,
        &artistico_cerca,
    );
    reportar_artistico(
        &art_cerca,
        &niveles,
        &artistico_cerca,
        ANCHO,
        ALTO,
        "cuadro final, zoom artistico minimo",
    );
    println!(
        "  radio orbital              {:.3}   ({ARTISTIC_MIN_RADIUS_FACTOR:.2} x scene_radius {:.3})",
        cercana.radius(),
        niveles[1].scale.scene_radius
    );
    println!("  el modo base no llega aqui: su minimo es 1.8 x scene_radius.");

    // ------------------------- el zoom mas cercano, en el perfil interactivo
    //
    // Este es el caso que de verdad se paga al pintar. Mientras el boton
    // esta abajo la ventana esta en cambio sostenido, asi que dibuja al
    // perfil interactivo y **no** al cuadro final: el bloque de `800 x 600`
    // de arriba mide lo que cuesta el cuadro que se presenta al soltar, no
    // lo que cuesta cada cuadro de un arrastre.
    //
    // Misma camara y mismo factor que el bloque anterior; lo unico que
    // cambia es la resolucion, y la fixture se reconstruye a ella porque el
    // picking depende del tamano del cuadro presentado.
    let artistico_cerca_perfil =
        fixture_artistica(&niveles[1], &cercana, perfil.width, perfil.height);
    let mut art_cerca_perfil = artisticas(cercana);
    medir_con(
        &mut art_cerca_perfil,
        &niveles,
        &luces,
        perfil.width,
        perfil.height,
        &artistico_cerca_perfil,
    );
    reportar_artistico(
        &art_cerca_perfil,
        &niveles,
        &artistico_cerca_perfil,
        perfil.width,
        perfil.height,
        "perfil interactivo, zoom artistico minimo",
    );
    println!("  es el cuadro que se traza mientras se arrastra el pincel.");

    println!("\n  Las tres columnas artisticas comparten escena, camara y estado.");
    println!("  La diferencia entre ellas es atribuible dentro de la corrida; la que");
    println!("  hay entre bloques distintos no lo es, porque cambia el encuadre.");
    println!("  Aqui no se declara margen ni aprobacion: la tabla es el dato.");

    println!("\n  La reserva esta en tiempo, no en primitivas: el coste no es lineal");
    println!("  en el conteo, asi que traducirla a densidad exige medir el nivel");
    println!("  objetivo. Y el escalon mas caro de la tabla no es el conteo sino la");
    println!("  optica: mirar la columna de rayos secundarios junto a los tiempos.");
    println!("\n  Registrar junto a las cifras: commit, fecha, arbol, hardware y toolchain.");

    if !cabe {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expedition33_continente_inacabado::input::{pick_artistic, PresentedFrame};

    /// La fixture artistica tiene que ser **de verdad**: pintada sobre
    /// superficies que existen, identificadas por el mismo picking que usa
    /// la ventana, y no sobre claves inventadas.
    ///
    /// Sin esto, el tercer escenario mediria el coste de consultar dos mapas
    /// vacios y lo llamaria «con pigmento», que es exactamente la clase de
    /// cifra que no sirve para decidir nada.
    #[test]
    fn la_fixture_artistica_nace_de_impactos_reales() {
        let diorama = nivel(WaterPreset::RefractiveWater, Density::Safe);
        let camara = diorama.hero_camera();
        let fixture = fixture_artistica(&diorama, &camara, ANCHO, ALTO);

        // No esta vacia, y toca varias superficies distintas.
        assert!(
            fixture.masks.len() >= 3,
            "el revelado toca {} superficies",
            fixture.masks.len()
        );
        assert!(
            fixture.pigment.len() >= 3,
            "el pigmento toca {} superficies",
            fixture.pigment.len()
        );
        assert!(
            !fixture.claves.is_empty(),
            "la fixture no registro ninguna clave"
        );

        // Y cada clave corresponde a un objeto real de la escena y a una
        // carta que esa primitiva reconoce.
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

    /// Las claves salen del picking, no de un recorrido del vector de
    /// objetos: tienen que coincidir con lo que devuelve `pick_artistic`
    /// para el mismo cursor.
    #[test]
    fn las_claves_coinciden_con_el_picking_de_la_ventana() {
        let diorama = nivel(WaterPreset::RefractiveWater, Density::Safe);
        let camara = diorama.hero_camera();
        let fixture = fixture_artistica(&diorama, &camara, ANCHO, ALTO);
        let cuadro = PresentedFrame::full(camara, (ANCHO, ALTO));

        let mut confirmadas = 0usize;

        for cursor in cursores_de_la_fixture(ANCHO, ALTO) {
            if let Some(objetivo) = pick_artistic(&diorama.scene, &diorama.accel, &cuadro, cursor) {
                let clave = SurfaceKey::new(objetivo.object_index, objetivo.uv_chart);

                assert!(
                    fixture.claves.contains(&clave),
                    "el picking en {cursor:?} da {clave:?} y la fixture no la tiene"
                );
                confirmadas += 1;
            }
        }

        assert!(
            confirmadas >= 3,
            "solo {confirmadas} cursores de la rejilla cayeron sobre el diorama"
        );
    }

    /// El pigmento no puede dejar la textura compartida tocada: se muestrea,
    /// no se modifica.
    #[test]
    fn la_fixture_no_altera_las_texturas_de_la_escena() {
        let diorama = nivel(WaterPreset::RefractiveWater, Density::Safe);
        let camara = diorama.hero_camera();

        let antes: Vec<_> = diorama
            .scene
            .textures
            .iter()
            .map(|t| (t.width(), t.height(), t.peak()))
            .collect();

        let _ = fixture_artistica(&diorama, &camara, ANCHO, ALTO);

        let despues: Vec<_> = diorama
            .scene
            .textures
            .iter()
            .map(|t| (t.width(), t.height(), t.peak()))
            .collect();

        assert_eq!(antes, despues, "la fixture toco una textura compartida");
    }

    /// El encuadre mas cercano del modo artistico tiene que quedar
    /// **exactamente** donde dice su factor.
    ///
    /// Se comprueba el radio resultante y no el codigo que lo calcula: la
    /// camara pasa por `with_radius_limits` y `zoom`, y `zoom` recorta contra
    /// el minimo. Si ese recorte quedara mal puesto, la camara se detendria
    /// en el minimo del modo base y el bloque mediria otro encuadre sin que
    /// nada avisara.
    #[test]
    fn el_zoom_artistico_queda_exactamente_en_su_factor_de_escena() {
        let diorama = nivel(WaterPreset::RefractiveWater, Density::Safe);
        let camara = camara_de_zoom_artistico(&diorama);
        let esperado = diorama.scale.scene_radius * ARTISTIC_MIN_RADIUS_FACTOR;

        assert!(
            (camara.radius() - esperado).abs() < 1e-3,
            "radio {} y se esperaba {esperado}",
            camara.radius()
        );

        // Y es de verdad mas cerca que la toma hero, o el bloque no mediria
        // nada distinto.
        assert!(
            camara.radius() < diorama.hero_camera().radius(),
            "el zoom artistico no se acerco: {} contra hero {}",
            camara.radius(),
            diorama.hero_camera().radius()
        );
    }

    /// Y ese mismo encuadre tiene que poder alimentar un bloque **en el
    /// perfil interactivo**, que es la resolucion a la que se pinta de
    /// verdad.
    #[test]
    fn el_zoom_artistico_alimenta_una_fixture_en_el_perfil_interactivo() {
        let diorama = nivel(WaterPreset::RefractiveWater, Density::Safe);
        let camara = camara_de_zoom_artistico(&diorama);
        let perfil = InteractiveProfile::default();

        let fixture = fixture_artistica(&diorama, &camara, perfil.width, perfil.height);

        assert!(
            fixture.masks.len() >= 3,
            "el revelado toca {} superficies en el perfil",
            fixture.masks.len()
        );
        assert!(
            fixture.pigment.len() >= 3,
            "el pigmento toca {} superficies en el perfil",
            fixture.pigment.len()
        );

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
}
