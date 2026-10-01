//! La entrega por defecto: la composición de **isla de borde** con la
//! segunda formación del Rompeolas.
//!
//! # Qué es
//!
//! El nivel seguro de `154` primitivas sigue existiendo tal cual: es la
//! escena que midieron los hitos 3 a 7 y con la que se siguen reproduciendo
//! sus gates. La entrega se construye **encima** de él, con la composición
//! que se aprobó mirando los renders del preview de fidelidad:
//!
//! 1. **Macroformación.** Las 34 piezas de `R-01` y `R-02` pasan de un arco
//!    a una retícula de cinco filas desfasadas, con cresta en el interior y
//!    caída hacia la bahía.
//! 2. **Territorio de `22 × 19`.** El plinto crece en planta y las tres
//!    regiones se separan, cada una en bloque y con un claro medido entre
//!    ellas. El Monolito no se mueve.
//! 3. **Apoyo.** La masa mayor de `G-02` pasa a ser la losa del Rompeolas, y
//!    el Rompeolas baja entero hasta empotrarse en ella.
//! 4. **Isla larga al oeste.** Otra masa de `G-02` se convierte en una isla
//!    larga en `z` y fina en `x`, del canto sur del lienzo hasta tres
//!    unidades dentro de la base.
//! 5. **Transición.** Una tercera masa hace de terraza entre la base y la
//!    isla, un escalón por encima de las dos.
//! 6. **Segunda formación.** Las dos masas de `G-02` que nadie veía —una
//!    enterrada en la isla, otra bajo la meseta de Praderas— se retiran, y
//!    sobre la isla entra una escalera de dieciséis prismas que crece hacia
//!    el Rompeolas original y se queda a `0.40` de él.
//! 7. **Praderas.** Sus 37 piezas se reescriben con la composición aprobada
//!    en su preview de fidelidad, dentro del territorio que les dejó el
//!    reparto. Vive en `meadows_delivery`; la entrega sin este paso es
//!    `delivery_level_previo_con`, y se conserva como línea base.
//!
//! # El presupuesto
//!
//! `154 - 2 + 16 = 168`. Es el único cambio de conteo: salen dos masas de
//! `G-02` y entran dieciséis prismas en el Rompeolas. Ver `DELIVERY`.
//!
//! # Todo se mide
//!
//! Ninguna coordenada de la composición está escrita a mano. Cada paso se
//! expresa contra algo ya medido —el plinto, el Monolito, la huella de una
//! región, la losa, la isla—, así que si el nivel seguro cambiara, la
//! entrega se recolocaría con él en vez de quedarse descuadrada.

use super::{meadows_delivery, nivel_con, Density, Presupuesto, WaterPreset, Xorshift32, SAFE};
use crate::accel::{ClusterPlan, SceneAccel};
use crate::bounds::Aabb;
use crate::cuboid::Cuboid;
use crate::primitive::Primitive;
use crate::scene::{SceneObject, SpatialGroupId};
use crate::scene_builder::{
    derive_orbit_radius, eye_at_yaw, measure_scene_radius, Blockout, SceneAnchors, SceneScale,
    HERO_YAW_DEGREES, LOOK_AT_HEIGHT_FRACTION,
};
use crate::texture::TextureError;
use nalgebra_glm::Vec3;
use std::path::Path;

// ---------------------------------------------------------------------
// Presupuesto
// ---------------------------------------------------------------------

/// Masas de `G-02` que se retiran de la escena.
pub const MASAS_RETIRADAS: usize = 2;

/// Prismas de la segunda formación del Rompeolas.
pub const PRISMAS_SECUNDARIOS: usize = 16;

/// Conteos de la entrega: el nivel seguro menos dos masas de `G-02` más
/// dieciséis prismas.
///
/// | Región | Seguro | Cambio | Entrega |
/// |---|---:|---:|---:|
/// | Global (`G-01`, `G-02`, Monolito) | 21 | `-2` | 19 |
/// | Praderas | 37 | | 37 |
/// | Rompeolas | 38 | `+16` | 54 |
/// | Aguas Voladoras | 58 | | 58 |
/// | **Total** | **154** | | **168** |
pub const DELIVERY: Presupuesto = Presupuesto {
    global: SAFE.global - MASAS_RETIRADAS,
    meadows: SAFE.meadows,
    breakwater: SAFE.breakwater + PRISMAS_SECUNDARIOS,
    flying_waters: SAFE.flying_waters,
};

// ---------------------------------------------------------------------
// Constantes de composición
// ---------------------------------------------------------------------

/// Planta del plinto de la entrega.
pub const PLINTO_X: f32 = 22.0;
pub const PLINTO_Z: f32 = 19.0;

/// El claro entre dos territorios.
const CLARO: f32 = 1.00;

/// Lo que se le deja al borde del plinto al arrimar una región.
const MARGEN_DEL_PLINTO: f32 = 0.20;

/// La calle entre Praderas y el Monolito.
const CORREDOR: f32 = CLARO * 1.5;

/// Lo que sobresale la losa por dentro del plinto.
const VUELO: f32 = 0.10;

/// Cuánto se hunde una pieza en lo que la sostiene. Evita caras
/// coplanares, que es lo que la política de empates de `SceneAccel` pide
/// separar.
const EMPOTRADO: f32 = 0.10;

/// Suelo libre que se deja al oeste de la parcela de la isla.
const RESPIRO_OESTE: f32 = 1.40;

/// Anchura de la isla respecto de su fondo.
const ESBELTEZ_DE_LA_ISLA: f32 = 0.42;

/// Cuánto se mete la isla dentro de la base.
const SOLAPE_CON_LA_BASE: f32 = 3.00;

/// Cuánto sube la terraza de transición sobre la base y la isla.
const ESCALON_DE_TRANSICION: f32 = 0.45;

/// Cuánto muerde la terraza dentro de la base y de la isla.
const MORDIDA_DE_LA_TRANSICION: f32 = 0.95;

/// Holgura de cada prisma dentro de la isla.
const HOLGURA_EN_LA_ISLA: f32 = 0.10;

/// Claro entre la segunda formación y el Rompeolas original: reducido
/// pero positivo. La segunda formación se lee como su continuación sin
/// tocarla.
pub const CLARO_AL_ROMPEOLAS_ORIGINAL: f32 = 0.40;

// --- macroformación

/// Piezas de la macroformación: los 28 pilares de `R-01` y los 6 tramos de
/// `R-02`.
const MACROFORMACION: usize = 34;

/// Piezas del Rompeolas original: la macroformación y los 4 soportes de
/// `R-03`.
const ROMPEOLAS_ORIGINAL: usize = MACROFORMACION + 4;

/// Pilares de `R-01`, los primeros de la macroformación.
const PILARES_R01: usize = 28;

/// Filas de la retícula, de la cresta hacia la costa.
const FILAS: [usize; 5] = [7, 7, 7, 7, 6];

/// Semilla del ruido de altura. La misma del arco de `R-01`.
const SEMILLA: u32 = 0x0A5C_1F03;

/// Grosor por racimos de tres piezas consecutivas.
const RACIMOS: [f32; 6] = [1.00, 0.74, 1.22, 0.88, 1.12, 0.80];

/// Altura mínima de una pieza de la retícula.
const ALTURA_MINIMA: f32 = 0.12;

// --- segunda formación

/// Prismas por banda, de la cima —junto al Rompeolas original— al extremo
/// libre de la isla.
const BANDAS: [usize; 4] = [2, 3, 5, 6];

/// Grosor de cada banda, en el mismo orden: cuatro factores de `RACIMOS`,
/// de mayor a menor.
const GROSOR_DE_LAS_BANDAS: [f32; 4] = [RACIMOS[2], RACIMOS[4], RACIMOS[3], RACIMOS[1]];

/// Altura de la segunda formación respecto de la primera.
const ESCALA_SECUNDARIA: f32 = 0.85;

/// Clusters del Rompeolas en la entrega.
const CLUSTER_ORIGINAL: u16 = 0;
const CLUSTER_SECUNDARIO: u16 = 1;

// ---------------------------------------------------------------------
// API
// ---------------------------------------------------------------------

/// La entrega por defecto, sin texturas.
pub fn delivery_level(water: WaterPreset) -> Blockout {
    delivery_level_con(water, None).expect("sin assets no hay error posible")
}

/// La entrega por defecto, cargando las texturas desde `raiz`.
///
/// La composición se mide siempre con el volumen de agua dentro: la huella
/// de Aguas lo incluye, y medirla sin él movería la bahía. Con
/// `InteriorVisible` el volumen se retira **al final**, así que los tres
/// presets comparten geometría y sólo difieren en él, como en el nivel
/// seguro.
///
/// Praderas lleva la composición aprobada en `meadows_delivery`: las mismas
/// 37 piezas, en su territorio, reescritas sobre el reparto de esta
/// composición. Lo ajeno a Praderas es idéntico a `delivery_level_previo_con`.
pub fn delivery_level_con(
    water: WaterPreset,
    raiz_assets: Option<&Path>,
) -> Result<Blockout, TextureError> {
    entrega_con(water, raiz_assets, true)
}

/// La entrega **anterior** a la promoción de Praderas, sin texturas.
pub fn delivery_level_previo(water: WaterPreset) -> Blockout {
    delivery_level_previo_con(water, None).expect("sin assets no hay error posible")
}

/// La entrega **anterior** a la promoción de Praderas: la isla de borde con
/// las 37 piezas del nivel seguro trasladadas en bloque.
///
/// No es lo que se presenta. Se conserva como línea base: es la escena sobre
/// la que se aprobó el preview de Praderas
/// (`examples/meadows_fidelity_preview.rs`) y contra la que se comprueba que
/// la promoción no toca nada fuera de Praderas.
pub fn delivery_level_previo_con(
    water: WaterPreset,
    raiz_assets: Option<&Path>,
) -> Result<Blockout, TextureError> {
    entrega_con(water, raiz_assets, false)
}

fn entrega_con(
    water: WaterPreset,
    raiz_assets: Option<&Path>,
    praderas_aprobadas: bool,
) -> Result<Blockout, TextureError> {
    let medida = match water {
        WaterPreset::InteriorVisible => WaterPreset::RefractiveWater,
        otro => otro,
    };

    let seguro = nivel_con(medida, Density::Safe, raiz_assets)?;
    let mut entrega = componer(seguro);

    // Antes de medir: la escala y la jerarquía se toman sobre la geometría
    // final. Praderas no sale de su territorio, así que el radio medido no
    // cambia, y lo comprueba un test.
    if praderas_aprobadas {
        let ancla = entrega.anchors.meadows_anchor;
        meadows_delivery::aplicar(&mut entrega.scene, ancla);
    }

    if water == WaterPreset::InteriorVisible {
        let volumen = volumen_de_agua(&entrega.scene.objects);

        entrega.scene.objects.remove(volumen);
    }

    Ok(medir_y_acelerar(entrega))
}

/// Los prismas de la segunda formación: las piezas del Rompeolas que van
/// después de las treinta y ocho originales.
pub fn formacion_secundaria(scene: &crate::scene::Scene) -> Vec<usize> {
    indices(&scene.objects, SpatialGroupId::Breakwater)
        .into_iter()
        .skip(ROMPEOLAS_ORIGINAL)
        .collect()
}

// ---------------------------------------------------------------------
// Composición
// ---------------------------------------------------------------------

/// Aplica los seis pasos al nivel seguro. Ver la cabecera del módulo.
fn componer(seguro: Blockout) -> Blockout {
    let Blockout {
        mut scene,
        accel,
        mut anchors,
        scale,
    } = seguro;

    macroformacion(&mut scene.objects);

    let reparto = separar(&mut scene.objects);

    anchors.meadows_anchor += reparto.praderas;
    anchors.breakwater_anchor += reparto.rompeolas;
    anchors.flying_waters_anchor += reparto.aguas;
    anchors.boat_anchor += reparto.aguas;
    anchors.broken_edge_anchor += reparto.aguas;

    anchors.breakwater_anchor += apoyar(&mut scene.objects);

    // De aquí en adelante todo se mide sobre el territorio apoyado: la isla
    // y la transición cambian masas de `G-02`, y medir sobre la escena ya
    // cambiada devolvería otras.
    let apoyado = scene.objects.clone();
    let territorio = Territorio::medir(&apoyado);

    let isla = isla_larga(&apoyado, &territorio);
    let conector = transicion(&apoyado, &territorio, &isla);

    scene.objects[territorio.isla].primitive = Cuboid::new(isla).into();
    scene.objects[conector.indice].primitive = Cuboid::new(conector.caja).into();

    let retiradas = masas_tapadas(
        &scene.objects,
        &[territorio.base, territorio.isla, conector.indice],
    );
    let formacion = segunda_formacion(&apoyado, &isla, &conector.caja);

    let ultimo = *indices(&apoyado, SpatialGroupId::Breakwater)
        .last()
        .expect("el Rompeolas tiene piezas");
    let mut objetos = Vec::with_capacity(scene.objects.len() - retiradas.len() + formacion.len());

    for (i, objeto) in scene.objects.iter().enumerate() {
        if retiradas.contains(&i) {
            continue;
        }

        objetos.push(*objeto);

        if i == ultimo {
            objetos.extend(formacion.iter().copied());
        }
    }

    scene.objects = objetos;

    Blockout {
        scene,
        accel,
        anchors,
        scale,
    }
}

/// Mide la escala sobre la geometría final y reconstruye la jerarquía.
///
/// La jerarquía se construye **después** de fijar el vector de objetos: es
/// la invariante de `Scene`, que guarda posiciones y no identidades.
fn medir_y_acelerar(nivel: Blockout) -> Blockout {
    let Blockout {
        scene,
        anchors,
        scale,
        ..
    } = nivel;

    let orbit_center = anchors.orbit_center;
    let monolith_height = scale.monolith_height;
    let scene_radius = measure_scene_radius(&scene, orbit_center);
    let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

    let anchors = SceneAnchors {
        look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
        hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
        ..anchors
    };

    // Dos clusters en el Rompeolas: el original sobre su losa y la segunda
    // formación sobre la isla. Una sola caja para los dos cubriría el aire
    // que hay entre ellos, y un rayo que la rozara pagaría las cincuenta y
    // cuatro pruebas.
    let mut plan = ClusterPlan::new();

    for (k, i) in indices(&scene.objects, SpatialGroupId::Breakwater)
        .into_iter()
        .enumerate()
    {
        let cluster = if k < ROMPEOLAS_ORIGINAL {
            CLUSTER_ORIGINAL
        } else {
            CLUSTER_SECUNDARIO
        };

        plan.asignar(i, cluster);
    }

    let accel = SceneAccel::build_from_plan(&scene, &plan).expect("la entrega tiene geometria");

    Blockout {
        scene,
        accel,
        anchors,
        scale: SceneScale {
            scene_radius,
            orbit_radius,
            ..scale
        },
    }
}

// --- 1. macroformación

/// Las 34 piezas de `R-01` y `R-02`, redistribuidas en una retícula de
/// cinco filas sobre la huella que ya ocupaban.
///
/// Cada pieza conserva su material y su grupo; sólo cambia la primitiva.
/// Las filas se desfasan media celda —empaquetado hexagonal—, el perfil
/// pesa en el centro y cae hacia la bahía, y el grosor va por racimos de
/// tres.
fn macroformacion(objetos: &mut [SceneObject]) {
    let piezas = indices(objetos, SpatialGroupId::Breakwater);
    let sustituidos = &piezas[..MACROFORMACION];
    let caja = huella(objetos, sustituidos);

    let base = caja.min.y;
    let ancho_huella = caja.max.x - caja.min.x;
    let fondo_huella = caja.max.z - caja.min.z;
    let alto_original = caja.max.y - caja.min.y;

    let mut azar = Xorshift32::new(SEMILLA);
    let mut k = 0usize;

    for (fila, &columnas) in FILAS.iter().enumerate() {
        let hacia_la_costa = fila as f32 / (FILAS.len() - 1) as f32;
        let paso = ancho_huella / columnas as f32;
        let desfase = if fila % 2 == 1 { paso * 0.5 } else { 0.0 };

        for columna in 0..columnas {
            let x = caja.min.x + desfase + (columna as f32 + 0.5) * paso;
            let z = caja.min.z + (hacia_la_costa * 0.92 + 0.04) * fondo_huella;

            let centrado = (x - (caja.min.x + ancho_huella * 0.5)) / (ancho_huella * 0.5);
            let cresta = 1.0 - 0.45 * centrado * centrado;
            let caida = 1.0 - 0.78 * hacia_la_costa.powf(1.6);

            let ruido = 0.10 * azar.simetrico();
            let altura = (alto_original * (cresta * caida + ruido)).max(ALTURA_MINIMA);
            let ancho = paso * 0.86 * RACIMOS[(k / 3) % RACIMOS.len()];

            objetos[sustituidos[k]].primitive =
                prisma(Vec3::new(x, base + altura * 0.5, z), ancho, altura);
            k += 1;
        }
    }

    debug_assert_eq!(k, MACROFORMACION, "FILAS tiene que sumar la macroformacion");
}

// --- 2. territorio de 22 x 19

/// Adónde va cada una de las tres regiones.
struct Reparto {
    praderas: Vec3,
    rompeolas: Vec3,
    aguas: Vec3,
}

/// Crece el plinto y separa las tres regiones.
///
/// - **Rompeolas**: flanco izquierdo, arrimado al canto del plinto y un
///   claro por delante del Monolito.
/// - **Praderas**: centrada en el Monolito y detrás de él, con una calle
///   de claro y medio.
/// - **Aguas**: a la derecha del Rompeolas ya movido, con su claro, y
///   arrimada al canto frontal.
///
/// Cada región se mueve **en bloque**: un solo vector para todas sus
/// piezas. El Monolito y el arco costero no se mueven.
fn separar(objetos: &mut [SceneObject]) -> Reparto {
    let plinto = huella_del_grupo(objetos, SpatialGroupId::Global);
    let centro = (plinto.min + plinto.max) * 0.5;
    let medio = Vec3::new(
        PLINTO_X * 0.5,
        (plinto.max.y - plinto.min.y) * 0.5,
        PLINTO_Z * 0.5,
    );
    let suelo = Aabb::new(centro - medio, centro + medio);

    let monolito = huella_del_grupo(objetos, SpatialGroupId::Monolith);
    let praderas = huella_del_grupo(objetos, SpatialGroupId::Meadows);
    let rompeolas = huella_del_grupo(objetos, SpatialGroupId::Breakwater);
    let aguas = huella_del_grupo(objetos, SpatialGroupId::FlyingWaters);

    let rompeolas_delta = Vec3::new(
        (suelo.min.x + MARGEN_DEL_PLINTO) - rompeolas.min.x,
        0.0,
        (monolito.max.z + CLARO) - rompeolas.min.z,
    );
    let praderas_delta = Vec3::new(
        (monolito.min.x + monolito.max.x) * 0.5 - (praderas.min.x + praderas.max.x) * 0.5,
        0.0,
        (monolito.min.z - CORREDOR) - praderas.max.z,
    );
    // Contra el canto del Rompeolas **ya movido**.
    let aguas_delta = Vec3::new(
        (rompeolas.max.x + rompeolas_delta.x + CLARO) - aguas.min.x,
        0.0,
        (suelo.max.z - MARGEN_DEL_PLINTO) - aguas.max.z,
    );

    for objeto in objetos.iter_mut() {
        if objeto.spatial_group == SpatialGroupId::Global {
            let caja = objeto.primitive.bounds();
            let centro = (caja.min + caja.max) * 0.5;
            let espesor = caja.max.y - caja.min.y;

            objeto.primitive =
                Cuboid::centrado(centro, Vec3::new(PLINTO_X, espesor, PLINTO_Z)).into();

            continue;
        }

        let delta = match objeto.spatial_group {
            SpatialGroupId::Meadows => praderas_delta,
            SpatialGroupId::Breakwater => rompeolas_delta,
            SpatialGroupId::FlyingWaters => aguas_delta,
            _ => continue,
        };

        objeto.primitive = trasladada(&objeto.primitive, delta);
    }

    Reparto {
        praderas: praderas_delta,
        rompeolas: rompeolas_delta,
        aguas: aguas_delta,
    }
}

// --- 3. apoyo

/// Posa la masa mayor de `G-02` en el plinto, centrada bajo el Rompeolas,
/// y baja el Rompeolas entero hasta empotrarse en ella.
///
/// Devuelve cuánto bajó el Rompeolas, para que su ancla lo siga.
fn apoyar(objetos: &mut [SceneObject]) -> Vec3 {
    let plinto = huella_del_grupo(objetos, SpatialGroupId::Global);
    let piezas = indices(objetos, SpatialGroupId::Breakwater);
    let rompeolas = huella(objetos, &piezas);

    let indice = masa_mayor(objetos);
    let losa = objetos[indice].primitive.bounds();

    let centrar = |medio: f32, objetivo: f32, minimo: f32, maximo: f32| {
        objetivo.clamp(minimo + medio, maximo - medio)
    };

    let centro_x = centrar(
        (losa.max.x - losa.min.x) * 0.5,
        (rompeolas.min.x + rompeolas.max.x) * 0.5,
        plinto.min.x + VUELO,
        plinto.max.x - VUELO,
    );
    let centro_z = centrar(
        (losa.max.z - losa.min.z) * 0.5,
        (rompeolas.min.z + rompeolas.max.z) * 0.5,
        plinto.min.z + VUELO,
        plinto.max.z - VUELO,
    );

    let pedestal = Vec3::new(
        centro_x - (losa.min.x + losa.max.x) * 0.5,
        plinto.max.y - losa.min.y,
        centro_z - (losa.min.z + losa.max.z) * 0.5,
    );

    // La base más alta de la macroformación es la que tiene que llegar al
    // techo de la losa: con la media, las que arrancaran más arriba se
    // quedarían colgadas.
    let techo = losa.max.y + pedestal.y;
    let base = piezas[..MACROFORMACION]
        .iter()
        .map(|&i| objetos[i].primitive.bounds().min.y)
        .fold(f32::MIN, f32::max);

    let bajada = Vec3::new(0.0, (techo - EMPOTRADO) - base, 0.0);

    for (i, objeto) in objetos.iter_mut().enumerate() {
        let delta = if i == indice {
            pedestal
        } else if objeto.spatial_group == SpatialGroupId::Breakwater {
            bajada
        } else {
            continue;
        };

        objeto.primitive = trasladada(&objeto.primitive, delta);
    }

    bajada
}

// --- 4 y 5. isla y transición

/// Las masas de `G-02` que forman el territorio, medidas sobre la escena
/// apoyada.
struct Territorio {
    base: usize,
    isla: usize,
    parcela: Aabb,
}

impl Territorio {
    fn medir(objetos: &[SceneObject]) -> Territorio {
        let base = masa_mayor(objetos);
        let principal = objetos[base].primitive.bounds();
        let plinto = huella_del_grupo(objetos, SpatialGroupId::Global);
        let monolito = huella_del_grupo(objetos, SpatialGroupId::Monolith);
        let praderas = huella_del_grupo(objetos, SpatialGroupId::Meadows);

        // La parcela libre entre el canto oeste, Praderas, el Monolito y la
        // base, cada lado a su claro.
        let parcela = Aabb::new(
            Vec3::new(
                plinto.min.x + MARGEN_DEL_PLINTO + RESPIRO_OESTE,
                plinto.max.y,
                praderas.max.z + CLARO,
            ),
            Vec3::new(
                monolito.min.x - CLARO,
                principal.max.y,
                principal.min.z - CLARO,
            ),
        );

        // La isla sale de la masa más cercana a esa parcela.
        let isla = masa_mas_cercana(objetos, centro_xz(&parcela), &[base]);

        Territorio {
            base,
            isla,
            parcela,
        }
    }
}

/// La isla larga: del canto sur del lienzo hasta tres unidades dentro de la
/// base, y tan ancha como deja Praderas con su claro.
fn isla_larga(objetos: &[SceneObject], territorio: &Territorio) -> Aabb {
    let plinto = huella_del_grupo(objetos, SpatialGroupId::Global);
    let praderas = huella_del_grupo(objetos, SpatialGroupId::Meadows);
    let principal = objetos[territorio.base].primitive.bounds();
    let parcela = territorio.parcela;

    // El fondo: del claro con la base al canto sur del lienzo. La anchura
    // sale del fondo y se recorta a un claro de Praderas.
    let z0 = plinto.min.z + MARGEN_DEL_PLINTO;
    let z1 = principal.min.z - CLARO;
    let fondo = z1 - z0;

    let x0 = plinto.min.x + MARGEN_DEL_PLINTO;
    let x1 = (x0 + fondo * ESBELTEZ_DE_LA_ISLA).min(praderas.min.x - CLARO);

    // Ensanchada hasta donde Praderas deja, y nunca hacia atrás.
    let este = (praderas.min.x - CLARO).max(x1);

    // Alargada dentro de la base: el único solape autorizado.
    let norte = (principal.min.z + SOLAPE_CON_LA_BASE).min(principal.max.z);

    Aabb::new(
        Vec3::new(x0, parcela.min.y, z0),
        Vec3::new(este, parcela.max.y, norte),
    )
}

/// La terraza de transición, con la masa de `G-02` que la forma.
struct Conector {
    indice: usize,
    caja: Aabb,
}

/// La zona donde la base y la isla se cruzan, un escalón por encima.
fn zona_de_transicion(base: &Aabb, isla: &Aabb, rompeolas: &Aabb, monolito: &Aabb) -> Aabb {
    Aabb::new(
        Vec3::new(
            isla.max.x - MORDIDA_DE_LA_TRANSICION,
            base.min.y,
            base.min.z - MORDIDA_DE_LA_TRANSICION * 2.0,
        ),
        Vec3::new(
            monolito.min.x - CLARO,
            base.max.y + ESCALON_DE_TRANSICION,
            rompeolas.min.z - MORDIDA_DE_LA_TRANSICION * 0.2,
        ),
    )
}

/// La terraza entre la base y la isla larga.
///
/// La masa se elige por cercanía a la zona de cruce **tal como estaba el
/// territorio apoyado**, con la isla todavía en su sitio; la caja se mide
/// contra la isla ya larga.
fn transicion(objetos: &[SceneObject], territorio: &Territorio, isla_larga: &Aabb) -> Conector {
    let base = objetos[territorio.base].primitive.bounds();
    let isla = objetos[territorio.isla].primitive.bounds();
    let rompeolas = huella_del_grupo(objetos, SpatialGroupId::Breakwater);
    let monolito = huella_del_grupo(objetos, SpatialGroupId::Monolith);

    let zona = zona_de_transicion(&base, &isla, &rompeolas, &monolito);
    let indice = masa_mas_cercana(
        objetos,
        centro_xz(&zona),
        &[territorio.base, territorio.isla],
    );

    Conector {
        indice,
        caja: zona_de_transicion(&base, isla_larga, &rompeolas, &monolito),
    }
}

// --- 6. segunda formación

/// Las masas de `G-02` que nadie ve desde arriba: su huella cabe entera en
/// la de otro objeto que remata más alto. Se retiran las dos de mayor
/// huella; la base, la isla y el conector quedan fuera siempre.
fn masas_tapadas(objetos: &[SceneObject], intocables: &[usize]) -> Vec<usize> {
    let mut tapadas: Vec<usize> = indices(objetos, SpatialGroupId::ContinentBackground)
        .into_iter()
        .filter(|i| !intocables.contains(i))
        .filter(|&i| {
            let caja = objetos[i].primitive.bounds();

            objetos.iter().enumerate().any(|(j, otro)| {
                let encima = otro.primitive.bounds();

                j != i && encima.max.y > caja.max.y && contiene_en_planta(&encima, &caja)
            })
        })
        .collect();

    tapadas.sort_by(|a, b| {
        area_en_planta(&objetos[*b].primitive.bounds())
            .partial_cmp(&area_en_planta(&objetos[*a].primitive.bounds()))
            .expect("no hay NaN en G-02")
            .then(a.cmp(b))
    });

    assert!(
        tapadas.len() >= MASAS_RETIRADAS,
        "solo hay {} masas de G-02 tapadas y la entrega retira {MASAS_RETIRADAS}",
        tapadas.len()
    );

    tapadas.truncate(MASAS_RETIRADAS);

    tapadas
}

/// La escalera de dieciséis prismas sobre la isla.
///
/// Cuatro bandas: pequeñas y bajas en el extremo libre, más gruesas y
/// altas hacia el Rompeolas original. El lenguaje es el de la
/// macroformación, **medido** sobre ella: el paso dentro de una fila y
/// entre filas, los grosores de `RACIMOS`, la fórmula de cresta y caída y
/// `SEMILLA`. Cada prisma toma material y grupo de su homólogo de `R-01`.
///
/// Dos ajustes, documentados:
///
/// - El paso dentro de una banda se **aprieta** si la banda no cabe en la
///   isla; nunca se separa más que el paso de `R-01`.
/// - Las bandas que llegan más al norte que el conector se recortan en `x`
///   al oeste de él. Es lo que deja acercar la cima al Rompeolas.
fn segunda_formacion(objetos: &[SceneObject], isla: &Aabb, conector: &Aabb) -> Vec<SceneObject> {
    let rompeolas_original = indices(objetos, SpatialGroupId::Breakwater);
    let r01 = &rompeolas_original[..PILARES_R01];
    let centro = |i: usize| centro_xz(&objetos[i].primitive.bounds());

    let paso_x = (centro(r01[FILAS[0] - 1]).0 - centro(r01[0]).0) / (FILAS[0] - 1) as f32;
    let paso_z = centro(r01[FILAS[0]]).1 - centro(r01[0]).1;
    let alto = r01
        .iter()
        .map(|&i| {
            let c = objetos[i].primitive.bounds();

            c.max.y - c.min.y
        })
        .fold(f32::MIN, f32::max)
        * ESCALA_SECUNDARIA;

    let rompeolas = huella(objetos, &rompeolas_original);
    let eje = (isla.min.x + isla.max.x) * 0.5;
    let suelo = isla.max.y - EMPOTRADO;

    // Alturas y grosores por banda, de la cima al pie.
    let mut azar = Xorshift32::new(SEMILLA);
    let mut bandas: Vec<(f32, Vec<f32>, f32)> = Vec::new();

    for (banda, &cuantos) in BANDAS.iter().enumerate() {
        let desde_la_cima = banda as f32 / (BANDAS.len() - 1) as f32;
        let ancho = paso_x * 0.86 * GROSOR_DE_LAS_BANDAS[banda];

        let alturas: Vec<f32> = (0..cuantos)
            .map(|columna| {
                let lado = columna as f32 - (cuantos - 1) as f32 * 0.5;
                let centrado = lado / (cuantos as f32 * 0.5);
                let cresta = 1.0 - 0.45 * centrado * centrado;
                let caida = 1.0 - 0.78 * desde_la_cima.powf(1.6);
                let ruido = 0.10 * azar.simetrico();

                (alto * (cresta * caida + ruido)).max(ALTURA_MINIMA)
            })
            .collect();

        let medio_fondo = {
            let tanteo = prisma(Vec3::zeros(), ancho, 1.0).bounds();

            (tanteo.max.z - tanteo.min.z) * 0.5
        };

        bandas.push((ancho, alturas, medio_fondo));
    }

    // En z: la banda alta a su claro del Rompeolas original.
    let dz = bandas
        .iter()
        .enumerate()
        .map(|(banda, &(_, _, medio_fondo))| {
            rompeolas.min.z - CLARO_AL_ROMPEOLAS_ORIGINAL - (-(banda as f32) * paso_z + medio_fondo)
        })
        .fold(f32::MAX, f32::min);

    // En x: cada banda, en el tramo de isla que le toca.
    let mut formacion = Vec::with_capacity(PRISMAS_SECUNDARIOS);

    for (banda, (ancho, alturas, medio_fondo)) in bandas.iter().enumerate() {
        let z = -(banda as f32) * paso_z + dz;
        let frente_al_conector = z + medio_fondo > conector.min.z - HOLGURA_EN_LA_ISLA
            && z - medio_fondo < conector.max.z + HOLGURA_EN_LA_ISLA;

        let oeste = isla.min.x + HOLGURA_EN_LA_ISLA + ancho * 0.5;
        let este = if frente_al_conector {
            conector.min.x - HOLGURA_EN_LA_ISLA
        } else {
            isla.max.x - HOLGURA_EN_LA_ISLA
        } - ancho * 0.5;

        let cuantos = alturas.len();
        let paso = if cuantos > 1 {
            paso_x.min((este - oeste) / (cuantos - 1) as f32)
        } else {
            paso_x
        };
        let medio_tramo = paso * (cuantos - 1) as f32 * 0.5;
        let desfase = if banda % 2 == 1 {
            paso_x * 0.25
        } else {
            -paso_x * 0.25
        };
        let centro_de_la_banda = encajado(eje + desfase, oeste + medio_tramo, este - medio_tramo);

        for (columna, &altura) in alturas.iter().enumerate() {
            let lado = columna as f32 - (cuantos - 1) as f32 * 0.5;
            let k = formacion.len();

            formacion.push(SceneObject {
                primitive: prisma(
                    Vec3::new(centro_de_la_banda + lado * paso, suelo + altura * 0.5, z),
                    *ancho,
                    altura,
                ),
                ..objetos[r01[k]]
            });
        }
    }

    debug_assert_eq!(
        formacion.len(),
        PRISMAS_SECUNDARIOS,
        "las bandas suman dieciseis"
    );

    formacion
}

// ---------------------------------------------------------------------
// Medición
// ---------------------------------------------------------------------

fn indices(objetos: &[SceneObject], grupo: SpatialGroupId) -> Vec<usize> {
    objetos
        .iter()
        .enumerate()
        .filter(|(_, objeto)| objeto.spatial_group == grupo)
        .map(|(i, _)| i)
        .collect()
}

fn huella(objetos: &[SceneObject], piezas: &[usize]) -> Aabb {
    piezas
        .iter()
        .map(|&i| objetos[i].primitive.bounds())
        .reduce(|a, b| a.union(&b))
        .expect("una huella necesita piezas")
}

fn huella_del_grupo(objetos: &[SceneObject], grupo: SpatialGroupId) -> Aabb {
    huella(objetos, &indices(objetos, grupo))
}

fn centro_xz(caja: &Aabb) -> (f32, f32) {
    (
        (caja.min.x + caja.max.x) * 0.5,
        (caja.min.z + caja.max.z) * 0.5,
    )
}

fn area_en_planta(caja: &Aabb) -> f32 {
    (caja.max.x - caja.min.x) * (caja.max.z - caja.min.z)
}

fn contiene_en_planta(fuera: &Aabb, dentro: &Aabb) -> bool {
    fuera.min.x <= dentro.min.x
        && fuera.max.x >= dentro.max.x
        && fuera.min.z <= dentro.min.z
        && fuera.max.z >= dentro.max.z
}

/// La masa de `G-02` de mayor huella. El empate se queda con la primera.
fn masa_mayor(objetos: &[SceneObject]) -> usize {
    indices(objetos, SpatialGroupId::ContinentBackground)
        .into_iter()
        .fold(None, |mejor: Option<usize>, i| match mejor {
            Some(m)
                if area_en_planta(&objetos[m].primitive.bounds())
                    >= area_en_planta(&objetos[i].primitive.bounds()) =>
            {
                Some(m)
            }
            _ => Some(i),
        })
        .expect("G-02 tiene masas")
}

/// La masa de `G-02` con el centro más cerca de `destino`, sin contar las
/// excluidas. El empate se queda con la primera.
fn masa_mas_cercana(objetos: &[SceneObject], destino: (f32, f32), excluidas: &[usize]) -> usize {
    let distancia = |i: usize| {
        let (x, z) = centro_xz(&objetos[i].primitive.bounds());

        (x - destino.0).powi(2) + (z - destino.1).powi(2)
    };

    indices(objetos, SpatialGroupId::ContinentBackground)
        .into_iter()
        .filter(|i| !excluidas.contains(i))
        .fold(None, |mejor: Option<usize>, i| match mejor {
            Some(m) if distancia(m) <= distancia(i) => Some(m),
            _ => Some(i),
        })
        .expect("G-02 tiene masas libres")
}

/// `A-01`: la pieza de Aguas de mayor volumen.
fn volumen_de_agua(objetos: &[SceneObject]) -> usize {
    let volumen = |i: usize| {
        let c = objetos[i].primitive.bounds();
        let d = c.max - c.min;

        d.x * d.y * d.z
    };

    indices(objetos, SpatialGroupId::FlyingWaters)
        .into_iter()
        .fold(None, |mejor: Option<usize>, i| match mejor {
            Some(m) if volumen(m) >= volumen(i) => Some(m),
            _ => Some(i),
        })
        .expect("Aguas Voladoras tiene piezas")
}

/// Encaja un centro en un intervalo; si el intervalo se invierte, lo
/// centra en vez de entrar en pánico como `f32::clamp`.
fn encajado(valor: f32, minimo: f32, maximo: f32) -> f32 {
    if minimo > maximo {
        (minimo + maximo) * 0.5
    } else {
        valor.clamp(minimo, maximo)
    }
}

// ---------------------------------------------------------------------
// Primitivas
// ---------------------------------------------------------------------

/// Un prisma de la retícula: cuboide en la Ruta B.
///
/// La misma parametrización que `breakwater::pilar`: la anchura entre caras
/// planas es el lado del cuboide.
#[cfg(not(feature = "hex-prism"))]
fn prisma(centro: Vec3, ancho: f32, altura: f32) -> Primitive {
    Cuboid::centrado(centro, Vec3::new(ancho, altura, ancho)).into()
}

/// Un prisma de la retícula: hexagonal en la Ruta A.
#[cfg(feature = "hex-prism")]
fn prisma(centro: Vec3, ancho: f32, altura: f32) -> Primitive {
    let apotema = ancho * 0.5;
    let radio = apotema / (std::f32::consts::PI / 6.0).cos();

    crate::hex_prism::HexPrism::new(centro, radio, altura).into()
}

/// La misma primitiva, movida.
///
/// Se reconstruye con su propio constructor a partir de su caja, así que
/// conserva tipo y tamaño. En el prisma el circunradio se lee del eje `z`:
/// en `x` el alcance es la apotema.
fn trasladada(primitiva: &Primitive, delta: Vec3) -> Primitive {
    match primitiva {
        Primitive::Cuboid(cuboide) => Cuboid::new(Aabb::new(
            cuboide.bounds.min + delta,
            cuboide.bounds.max + delta,
        ))
        .into(),
        #[cfg(feature = "hex-prism")]
        Primitive::HexPrism(prisma) => {
            let caja = prisma.bounds();
            let centro = (caja.min + caja.max) * 0.5 + delta;
            let radio = (caja.max.z - caja.min.z) * 0.5;
            let altura = caja.max.y - caja.min.y;

            crate::hex_prism::HexPrism::new(centro, radio, altura).into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounds::Aabb;
    use crate::primitive::Primitive;
    use crate::scene::{Scene, SpatialGroupId};
    use crate::scene_builder::{
        derive_orbit_radius, eye_at_yaw, measure_scene_radius, Blockout, HERO_YAW_DEGREES,
    };
    use crate::scenes::{delivery_level_previo, safe_level, WaterPreset, SAFE};

    /// Tolerancia de las comparaciones geométricas.
    const EPS: f32 = 1.0e-4;

    /// Claro mínimo contra el Monolito, Praderas y Aguas. El de composición
    /// es `1.00`; esto deja holgura de coma flotante.
    const CLARO_MINIMO: f32 = 0.90;

    /// Cuánto puede hundirse una pieza en su soporte.
    const EMPOTRADO_MAXIMO: f32 = 0.60;

    /// Piezas del Rompeolas original: `R-01`, `R-02` y `R-03`.
    const ROMPEOLAS_ORIGINAL: usize = 38;

    /// Piezas de la macroformación: `R-01` y `R-02`.
    const MACROFORMACION: usize = 34;

    fn entrega() -> Blockout {
        delivery_level(WaterPreset::RefractiveWater)
    }

    fn indices(scene: &Scene, grupo: SpatialGroupId) -> Vec<usize> {
        scene
            .objects
            .iter()
            .enumerate()
            .filter(|(_, o)| o.spatial_group == grupo)
            .map(|(i, _)| i)
            .collect()
    }

    fn caja(scene: &Scene, i: usize) -> Aabb {
        scene.objects[i].primitive.bounds()
    }

    fn huella(scene: &Scene, piezas: &[usize]) -> Aabb {
        piezas
            .iter()
            .map(|&i| caja(scene, i))
            .reduce(|a, b| a.union(&b))
            .expect("hay piezas")
    }

    fn huella_del_grupo(scene: &Scene, grupo: SpatialGroupId) -> Aabb {
        huella(scene, &indices(scene, grupo))
    }

    /// Hueco libre en planta entre dos cajas; negativo si se solapan.
    fn separacion_xz(a: &Aabb, b: &Aabb) -> f32 {
        let en_x = (b.min.x - a.max.x).max(a.min.x - b.max.x);
        let en_z = (b.min.z - a.max.z).max(a.min.z - b.max.z);

        en_x.max(en_z)
    }

    fn contiene_en_planta(fuera: &Aabb, dentro: &Aabb) -> bool {
        fuera.min.x <= dentro.min.x
            && fuera.max.x >= dentro.max.x
            && fuera.min.z <= dentro.min.z
            && fuera.max.z >= dentro.max.z
    }

    fn centro_xz(c: &Aabb) -> (f32, f32) {
        ((c.min.x + c.max.x) * 0.5, (c.min.z + c.max.z) * 0.5)
    }

    /// Las tres masas de `G-02` que forman el territorio, **medidas** sobre
    /// la escena entregada y no preguntadas al constructor.
    struct Territorio {
        base: usize,
        isla: usize,
        conector: usize,
    }

    fn territorio(scene: &Scene) -> Territorio {
        let g02 = indices(scene, SpatialGroupId::ContinentBackground);
        let macro_huella = huella(
            scene,
            &indices(scene, SpatialGroupId::Breakwater)[..MACROFORMACION],
        );

        // La base es la losa que contiene la huella de la macroformación.
        let base = *g02
            .iter()
            .find(|&&i| contiene_en_planta(&caja(scene, i), &macro_huella))
            .expect("hay una losa bajo el Rompeolas original");

        // La isla es la masa que toca el canto oeste del plinto.
        let plinto = huella_del_grupo(scene, SpatialGroupId::Global);
        let isla = *g02
            .iter()
            .filter(|&&i| i != base)
            .min_by(|&&a, &&b| {
                (caja(scene, a).min.x - plinto.min.x)
                    .partial_cmp(&(caja(scene, b).min.x - plinto.min.x))
                    .expect("no hay NaN")
            })
            .expect("hay isla");

        // El conector es la que muerde a las dos y remata por encima.
        let conector = *g02
            .iter()
            .filter(|&&i| i != base && i != isla)
            .find(|&&i| {
                let c = caja(scene, i);

                separacion_xz(&c, &caja(scene, base)) < 0.0
                    && separacion_xz(&c, &caja(scene, isla)) < 0.0
            })
            .expect("hay una terraza de transicion");

        Territorio {
            base,
            isla,
            conector,
        }
    }

    /// Las bandas de la formación secundaria, de sur a norte: prismas con
    /// el mismo centro en `z`.
    fn bandas(scene: &Scene) -> Vec<Vec<usize>> {
        let mut piezas = formacion_secundaria(scene);
        let z = |i: usize| centro_xz(&caja(scene, i)).1;

        piezas.sort_by(|a, b| z(*a).partial_cmp(&z(*b)).expect("no hay NaN"));

        let mut bandas: Vec<Vec<usize>> = Vec::new();

        for i in piezas {
            match bandas.last_mut() {
                Some(banda) if (z(banda[0]) - z(i)).abs() < 0.05 => banda.push(i),
                _ => bandas.push(vec![i]),
            }
        }

        bandas
    }

    // ------------------------------------------------------------ conteos

    #[test]
    fn la_entrega_suma_168_y_declara_el_cambio_de_presupuesto() {
        assert_eq!(MASAS_RETIRADAS, 2);
        assert_eq!(PRISMAS_SECUNDARIOS, 16);

        // Region por region: salen dos masas de G-02 y entran dieciseis
        // prismas en el Rompeolas.
        assert_eq!(DELIVERY.global, SAFE.global - MASAS_RETIRADAS);
        assert_eq!(DELIVERY.meadows, SAFE.meadows);
        assert_eq!(DELIVERY.breakwater, SAFE.breakwater + PRISMAS_SECUNDARIOS);
        assert_eq!(DELIVERY.flying_waters, SAFE.flying_waters);

        assert_eq!(DELIVERY.total(), 168);
        assert_eq!(
            DELIVERY.total(),
            SAFE.total() - MASAS_RETIRADAS + PRISMAS_SECUNDARIOS
        );
    }

    #[test]
    fn cada_region_de_la_entrega_respeta_su_presupuesto() {
        let nivel = entrega();
        let cuenta = |g| indices(&nivel.scene, g).len();

        assert_eq!(nivel.scene.objects.len(), DELIVERY.total());
        assert_eq!(
            cuenta(SpatialGroupId::Global)
                + cuenta(SpatialGroupId::ContinentBackground)
                + cuenta(SpatialGroupId::Monolith)
                + cuenta(SpatialGroupId::InteractionProps),
            DELIVERY.global
        );
        assert_eq!(cuenta(SpatialGroupId::Meadows), DELIVERY.meadows);
        assert_eq!(cuenta(SpatialGroupId::Breakwater), DELIVERY.breakwater);
        assert_eq!(cuenta(SpatialGroupId::FlyingWaters), DELIVERY.flying_waters);
        assert_eq!(cuenta(SpatialGroupId::ContinentBackground), 8);
    }

    #[test]
    fn los_presets_de_la_entrega_solo_difieren_en_el_volumen_de_agua() {
        let opaco = delivery_level(WaterPreset::OpaqueWater);
        let visible = delivery_level(WaterPreset::InteriorVisible);
        let refractivo = entrega();

        assert_eq!(opaco.scene.objects.len(), DELIVERY.total());
        assert_eq!(visible.scene.objects.len(), DELIVERY.total() - 1);

        for otro in [&opaco, &visible] {
            for (a, b) in otro.scene.objects.iter().zip(&refractivo.scene.objects) {
                assert_eq!(a.primitive.bounds(), b.primitive.bounds());
                assert_eq!(a.spatial_group, b.spatial_group);
            }
        }
    }

    #[test]
    fn el_nivel_seguro_historico_sigue_en_154() {
        // La entrega se construye **encima** del nivel seguro; el nivel
        // seguro no cambia, y con el siguen midiendo los gates historicos.
        assert_eq!(
            safe_level(WaterPreset::RefractiveWater).scene.objects.len(),
            154
        );
    }

    // ---------------------------------------------------------- territorio

    #[test]
    fn el_plinto_de_la_entrega_mide_22_por_19() {
        let plinto = huella_del_grupo(&entrega().scene, SpatialGroupId::Global);

        assert!((plinto.max.x - plinto.min.x - 22.0).abs() < EPS);
        assert!((plinto.max.z - plinto.min.z - 19.0).abs() < EPS);
    }

    #[test]
    fn el_monolito_no_se_mueve_y_praderas_y_aguas_se_trasladan_en_bloque() {
        let clasico = safe_level(WaterPreset::RefractiveWater);
        let nivel = entrega();

        let monolito_antes = indices(&clasico.scene, SpatialGroupId::Monolith);
        let monolito_ahora = indices(&nivel.scene, SpatialGroupId::Monolith);

        assert_eq!(monolito_antes.len(), monolito_ahora.len());

        for (&a, &b) in monolito_antes.iter().zip(&monolito_ahora) {
            assert_eq!(
                caja(&clasico.scene, a),
                caja(&nivel.scene, b),
                "el Monolito se movio"
            );
        }

        // Praderas y Aguas: la misma region, entera, en otro sitio. Cada
        // pieza conserva su tamano y todas comparten un solo vector.
        //
        // Praderas se mide sobre la composicion **previa** a su promocion:
        // el reparto la traslada en bloque y despues `meadows_delivery`
        // reescribe sus 37 piezas dentro de ese mismo territorio, que es lo
        // que comprueban sus propios tests.
        let previo = delivery_level_previo(WaterPreset::RefractiveWater);

        for (grupo, nivel) in [
            (SpatialGroupId::Meadows, &previo),
            (SpatialGroupId::FlyingWaters, &nivel),
        ] {
            let antes = indices(&clasico.scene, grupo);
            let ahora = indices(&nivel.scene, grupo);

            assert_eq!(antes.len(), ahora.len(), "{grupo:?}");

            let delta = caja(&nivel.scene, ahora[0]).min - caja(&clasico.scene, antes[0]).min;

            for (&a, &b) in antes.iter().zip(&ahora) {
                let x = caja(&clasico.scene, a);
                let y = caja(&nivel.scene, b);

                assert!(
                    ((y.max - y.min) - (x.max - x.min)).norm() < EPS,
                    "{grupo:?} cambio de forma"
                );
                assert!(
                    ((y.min - x.min) - delta).norm() < EPS,
                    "{grupo:?} no se movio en bloque"
                );
            }
        }

        // `A-01` sigue siendo un unico volumen cerrado, el ultimo de Aguas.
        let aguas = indices(&nivel.scene, SpatialGroupId::FlyingWaters);
        let volumen = |i: usize| {
            let c = caja(&nivel.scene, i);
            let d = c.max - c.min;

            d.x * d.y * d.z
        };
        let mayor =
            aguas
                .iter()
                .copied()
                .fold(aguas[0], |m, i| if volumen(i) > volumen(m) { i } else { m });

        assert_eq!(
            mayor,
            *aguas.last().expect("hay Aguas"),
            "A-01 no es el ultimo de Aguas"
        );
    }

    #[test]
    fn el_rompeolas_original_se_apoya_en_su_losa() {
        let nivel = entrega();
        let base = caja(&nivel.scene, territorio(&nivel.scene).base);

        for &i in &indices(&nivel.scene, SpatialGroupId::Breakwater)[..MACROFORMACION] {
            let c = caja(&nivel.scene, i);

            assert!(
                contiene_en_planta(&base, &c),
                "la pieza {i} sale de su losa"
            );
            assert!(
                c.min.y < base.max.y && c.min.y >= base.max.y - EMPOTRADO_MAXIMO,
                "la pieza {i} arranca en {:.3} sobre una losa de {:.3}",
                c.min.y,
                base.max.y
            );
        }
    }

    #[test]
    fn la_isla_larga_queda_al_oeste_y_solapa_la_base() {
        let nivel = entrega();
        let t = territorio(&nivel.scene);

        let plinto = huella_del_grupo(&nivel.scene, SpatialGroupId::Global);
        let isla = caja(&nivel.scene, t.isla);
        let base = caja(&nivel.scene, t.base);

        // En el canto oeste y el sur del plinto, con su margen.
        assert!((isla.min.x - plinto.min.x - 0.20).abs() < EPS);
        assert!((isla.min.z - plinto.min.z - 0.20).abs() < EPS);

        // Larga en z, fina en x.
        assert!(isla.max.z - isla.min.z > 2.0 * (isla.max.x - isla.min.x));

        // Se mete tres unidades en la base, a su misma cota.
        assert!((isla.max.z - base.min.z - 3.0).abs() < EPS);
        assert!((isla.max.y - base.max.y).abs() < EPS);

        for grupo in [
            SpatialGroupId::Meadows,
            SpatialGroupId::Monolith,
            SpatialGroupId::FlyingWaters,
        ] {
            assert!(
                separacion_xz(&isla, &huella_del_grupo(&nivel.scene, grupo)) >= CLARO_MINIMO,
                "la isla invade {grupo:?}"
            );
        }
    }

    #[test]
    fn la_transicion_muerde_la_base_y_la_isla_y_sube_un_escalon() {
        let nivel = entrega();
        let t = territorio(&nivel.scene);

        let conector = caja(&nivel.scene, t.conector);
        let base = caja(&nivel.scene, t.base);
        let isla = caja(&nivel.scene, t.isla);

        assert!(separacion_xz(&conector, &base) < 0.0);
        assert!(separacion_xz(&conector, &isla) < 0.0);
        assert!(conector.max.y > base.max.y + EPS);
        assert!(conector.max.y > isla.max.y + EPS);
        assert!(
            separacion_xz(
                &conector,
                &huella_del_grupo(&nivel.scene, SpatialGroupId::Monolith)
            ) >= CLARO_MINIMO
        );
    }

    #[test]
    fn no_queda_ninguna_masa_de_g02_enterrada() {
        // Las dos que se retiran son las que nadie veia: su huella cabia
        // bajo otro objeto mas alto. En la entrega no queda ninguna asi.
        let nivel = entrega();

        for &i in &indices(&nivel.scene, SpatialGroupId::ContinentBackground) {
            let c = caja(&nivel.scene, i);

            let tapada = nivel.scene.objects.iter().enumerate().any(|(j, o)| {
                let encima = o.primitive.bounds();

                j != i && encima.max.y > c.max.y && contiene_en_planta(&encima, &c)
            });

            assert!(!tapada, "la masa {i} de G-02 sigue tapada");
        }
    }

    // -------------------------------------------------- segunda formacion

    #[test]
    fn la_formacion_secundaria_son_16_prismas_de_la_ruta_tras_el_original() {
        let nivel = entrega();
        let rompeolas = indices(&nivel.scene, SpatialGroupId::Breakwater);
        let nuevos = formacion_secundaria(&nivel.scene);

        assert_eq!(nuevos.len(), PRISMAS_SECUNDARIOS);
        assert_eq!(nuevos, rompeolas[ROMPEOLAS_ORIGINAL..].to_vec());

        // El grupo sigue contiguo, como lo emite el generador.
        for par in rompeolas.windows(2) {
            assert_eq!(par[1], par[0] + 1);
        }

        let r01 = nivel.scene.objects[rompeolas[0]];

        for &i in &nuevos {
            let o = nivel.scene.objects[i];

            #[cfg(feature = "hex-prism")]
            assert!(
                matches!(o.primitive, Primitive::HexPrism(_)),
                "el prisma {i} no es HexPrism"
            );
            #[cfg(not(feature = "hex-prism"))]
            assert!(
                matches!(o.primitive, Primitive::Cuboid(_)),
                "el prisma {i} no es Cuboid"
            );

            assert_eq!(o.initial_material, r01.initial_material);
            assert_eq!(o.final_material, r01.final_material);
            assert_eq!(o.reveal_group, r01.reveal_group);
        }
    }

    #[test]
    fn la_ruta_a_da_cincuenta_prismas_en_la_entrega() {
        let nivel = entrega();

        let prismas = nivel
            .scene
            .objects
            .iter()
            .filter(|o| !matches!(o.primitive, Primitive::Cuboid(_)))
            .inspect(|o| assert_eq!(o.spatial_group, SpatialGroupId::Breakwater))
            .count();

        let esperados = if cfg!(feature = "hex-prism") {
            MACROFORMACION + PRISMAS_SECUNDARIOS
        } else {
            0
        };

        assert_eq!(prismas, esperados);
    }

    #[test]
    fn la_formacion_crece_por_bandas_hacia_el_rompeolas_original() {
        let nivel = entrega();
        let original = huella(
            &nivel.scene,
            &indices(&nivel.scene, SpatialGroupId::Breakwater)[..ROMPEOLAS_ORIGINAL],
        );

        let bandas = bandas(&nivel.scene);
        let cuentas: Vec<usize> = bandas.iter().map(|b| b.len()).collect();

        // Del extremo libre de la isla al Rompeolas original.
        assert_eq!(cuentas, vec![6, 5, 3, 2]);

        let media = |banda: &[usize], f: &dyn Fn(&Aabb) -> f32| {
            banda
                .iter()
                .map(|&i| f(&caja(&nivel.scene, i)))
                .sum::<f32>()
                / banda.len() as f32
        };

        for par in bandas.windows(2) {
            let ancho = |c: &Aabb| c.max.x - c.min.x;
            let techo = |c: &Aabb| c.max.y;
            let lejos = |c: &Aabb| separacion_xz(c, &original);

            assert!(
                media(&par[1], &ancho) > media(&par[0], &ancho),
                "el ancho no crece"
            );
            assert!(
                media(&par[1], &techo) > media(&par[0], &techo),
                "la altura no crece"
            );
            assert!(
                media(&par[1], &lejos) < media(&par[0], &lejos),
                "no se acerca al Rompeolas"
            );
        }

        // Y cada banda muerde la siguiente: una escalera, no bloques.
        for par in bandas.windows(2) {
            let norte = par[0]
                .iter()
                .map(|&i| caja(&nivel.scene, i).max.z)
                .fold(f32::MIN, f32::max);
            let sur = par[1]
                .iter()
                .map(|&i| caja(&nivel.scene, i).min.z)
                .fold(f32::MAX, f32::min);

            assert!(sur <= norte, "entre dos bandas queda un hueco");
        }
    }

    #[test]
    fn la_formacion_deja_0_40_al_rompeolas_original_sin_tocarlo() {
        let nivel = entrega();
        let originales = &indices(&nivel.scene, SpatialGroupId::Breakwater)[..ROMPEOLAS_ORIGINAL];
        let original = huella(&nivel.scene, originales);

        let claro = formacion_secundaria(&nivel.scene)
            .iter()
            .map(|&i| separacion_xz(&caja(&nivel.scene, i), &original))
            .fold(f32::MAX, f32::min);

        assert!(
            (claro - CLARO_AL_ROMPEOLAS_ORIGINAL).abs() < EPS,
            "claro {claro}"
        );
        assert!((0.25..=0.60).contains(&claro));

        for &i in &formacion_secundaria(&nivel.scene) {
            for &j in originales {
                assert!(
                    separacion_xz(&caja(&nivel.scene, i), &caja(&nivel.scene, j)) > 0.0,
                    "el prisma {i} solapa la pieza {j} del Rompeolas original"
                );
            }
        }
    }

    #[test]
    fn la_formacion_se_apoya_en_la_isla_y_no_invade() {
        let nivel = entrega();
        let t = territorio(&nivel.scene);
        let isla = caja(&nivel.scene, t.isla);
        let conector = caja(&nivel.scene, t.conector);

        for &i in &formacion_secundaria(&nivel.scene) {
            let c = caja(&nivel.scene, i);

            assert!(
                contiene_en_planta(&isla, &c),
                "el prisma {i} sale de la isla"
            );
            assert!(
                c.min.y < isla.max.y && c.min.y >= isla.max.y - EMPOTRADO_MAXIMO,
                "el prisma {i} no se apoya en la isla"
            );
            assert!(
                separacion_xz(&c, &conector) > 0.0,
                "el prisma {i} entra en el conector"
            );

            for grupo in [
                SpatialGroupId::Monolith,
                SpatialGroupId::Meadows,
                SpatialGroupId::FlyingWaters,
            ] {
                assert!(
                    separacion_xz(&c, &huella_del_grupo(&nivel.scene, grupo)) >= CLARO_MINIMO,
                    "el prisma {i} invade {grupo:?}"
                );
            }
        }

        // La formacion es un solo racimo compacto.
        let centros: Vec<(f32, f32)> = formacion_secundaria(&nivel.scene)
            .iter()
            .map(|&i| centro_xz(&caja(&nivel.scene, i)))
            .collect();

        for (k, p) in centros.iter().enumerate() {
            let vecino = centros
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != k)
                .map(|(_, o)| ((p.0 - o.0).powi(2) + (p.1 - o.1).powi(2)).sqrt())
                .fold(f32::MAX, f32::min);

            assert!(vecino <= 1.60, "el prisma {k} queda suelto a {vecino:.2}");
        }
    }

    // ------------------------------------------------- aceleracion y escala

    #[test]
    fn la_jerarquia_de_la_entrega_es_coherente() {
        let nivel = entrega();

        assert_eq!(nivel.accel.primitive_count(), nivel.scene.objects.len());

        let mut vistos = vec![0usize; nivel.scene.objects.len()];

        for grupo in &nivel.accel.groups {
            for cluster in &grupo.clusters {
                for &i in &cluster.object_indices {
                    vistos[i] += 1;

                    assert_eq!(nivel.scene.objects[i].spatial_group, grupo.id);
                    assert!(cluster.bounds.union(&caja(&nivel.scene, i)) == cluster.bounds);
                }
            }
        }

        assert!(
            vistos.iter().all(|&n| n == 1),
            "un objeto falta o se repite en la jerarquia"
        );

        // El Rompeolas en dos clusters: el original y la formacion de la
        // isla. Una sola caja para los dos estaria llena de aire.
        let rompeolas = nivel
            .accel
            .groups
            .iter()
            .find(|g| g.id == SpatialGroupId::Breakwater)
            .expect("hay Rompeolas");

        assert_eq!(rompeolas.clusters.len(), 2);

        let secundario = rompeolas
            .clusters
            .iter()
            .find(|c| c.object_indices == formacion_secundaria(&nivel.scene))
            .expect("la formacion secundaria tiene su propio cluster");
        let principal = rompeolas
            .clusters
            .iter()
            .find(|c| c.id != secundario.id)
            .expect("hay cluster principal");

        assert!(separacion_xz(&secundario.bounds, &principal.bounds) > 0.0);
    }

    #[test]
    fn la_escala_de_la_entrega_se_mide_de_nuevo() {
        let nivel = entrega();
        let centro = nivel.anchors.orbit_center;

        assert_eq!(
            nivel.scale.scene_radius,
            measure_scene_radius(&nivel.scene, centro)
        );
        assert_eq!(
            nivel.scale.orbit_radius,
            derive_orbit_radius(nivel.scale.scene_radius, nivel.scale.monolith_height)
        );
        assert_eq!(
            nivel.anchors.hero_camera_anchor,
            eye_at_yaw(centro, nivel.scale.orbit_radius, HERO_YAW_DEGREES)
        );
    }

    #[test]
    fn la_entrega_es_determinista() {
        let uno = entrega();
        let dos = entrega();

        for (a, b) in uno.scene.objects.iter().zip(&dos.scene.objects) {
            assert_eq!(a.primitive.bounds(), b.primitive.bounds());
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.anchors.breakwater_anchor, dos.anchors.breakwater_anchor);
    }
}
