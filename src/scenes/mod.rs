//! Constructores de escena, uno por región, más el ensamblado del nivel
//! seguro.
//!
//! El inventario fija conteos exactos por entrada y los suma a **160
//! primitivas trazables** en nivel seguro. Esos números no son
//! orientativos: son el presupuesto contra el que se mide el rendimiento,
//! así que el ensamblado los comprueba entrada por entrada en vez de
//! confiar en que cuadren.

pub mod breakwater;
pub mod continent;
pub mod flying_waters;
pub mod meadows;

use crate::accel::{ClusterPlan, SceneAccel};
use crate::color::Color;
use crate::cuboid::Cuboid;
use crate::material::{Material, ShadowMode};
use crate::scene::{MaterialId, Scene};
use crate::scene::{RevealGroup, SceneObject, SpatialGroupId};
use crate::scene_builder::{
    derive_orbit_radius, eye_at_yaw, measure_scene_radius, Blockout, SceneAnchors, SceneScale,
    HERO_YAW_DEGREES, LOOK_AT_HEIGHT_FRACTION,
};
use crate::skybox::Skybox;
use crate::texture::{Texture, TextureError};
use nalgebra_glm::Vec3;
use std::path::Path;

/// Generador pseudoaleatorio determinista, sin dependencias.
///
/// El inventario exige `seed: fixed` para toda entrada generada: el
/// blockout y los renders de evidencia tienen que salir idénticos en cada
/// corrida, y dos capturas que difieran por azar no sirven para comparar
/// nada.
pub(crate) struct Xorshift32(u32);

impl Xorshift32 {
    pub(crate) fn new(semilla: u32) -> Self {
        // El cero es un punto fijo del xorshift: se quedaría clavado.
        Xorshift32(if semilla == 0 { 0x9E37_79B9 } else { semilla })
    }

    /// Siguiente valor en `0.0..1.0`.
    pub(crate) fn siguiente(&mut self) -> f32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;

        // Los 24 bits altos: los bajos de un xorshift están peor
        // distribuidos.
        (x >> 8) as f32 / (1u32 << 24) as f32
    }

    /// Siguiente valor en `-1.0..1.0`.
    pub(crate) fn simetrico(&mut self) -> f32 {
        self.siguiente() * 2.0 - 1.0
    }
}

/// Añade un cuboide que **se revela**: nace en lienzo y termina en
/// `final_material`.
///
/// Es el caso normal. Todo el Continente arranca sin pintar, y lo que la
/// revelación hace es interpolar de `canvas_unpainted` al material final;
/// si los dos coincidieran, pintar no cambiaría nada.
pub(crate) fn masa(
    scene: &mut Scene,
    centro: Vec3,
    tamano: Vec3,
    canvas: MaterialId,
    final_material: MaterialId,
    spatial_group: SpatialGroupId,
    reveal_group: RevealGroup,
) {
    scene.add_object(SceneObject {
        primitive: Cuboid::centrado(centro, tamano).into(),
        initial_material: canvas,
        final_material,
        spatial_group,
        reveal_group,
    });
}

/// Añade un cuboide **inerte**: mismo material inicial y final.
///
/// Solo dos entradas del inventario lo son, y por razones opuestas: `G-01`
/// (el plinto) es lienzo y nunca se pinta, y `G-04` (la paleta y el pincel)
/// nace ya en cristal porque es la herramienta con la que se pinta, no parte
/// del cuadro.
pub(crate) fn masa_inerte(
    scene: &mut Scene,
    centro: Vec3,
    tamano: Vec3,
    material: MaterialId,
    spatial_group: SpatialGroupId,
    reveal_group: RevealGroup,
) {
    scene.add_object(SceneObject {
        primitive: Cuboid::centrado(centro, tamano).into(),
        initial_material: material,
        final_material: material,
        spatial_group,
        reveal_group,
    });
}

/// Cómo se representa `A-01`, el volumen de agua.
///
/// Los tres miden cosas distintas, y confundirlos daría un benchmark
/// optimista por accidente. `RefractiveWater` es el preset **canónico**
/// desde la Tarea 5.4; los otros dos son instrumentos de medición.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaterPreset {
    /// El volumen **no** se inserta como primitiva trazable. Quedan 159, y
    /// los rayos alcanzan barco, mástil, cadena, ancla, kelp, rocas y
    /// lecho sin cruzar ninguna frontera óptica.
    ///
    /// Es el preset del benchmark temprano: mide el coste de mirar dentro
    /// de la bahía **sin** el coste de la refracción. Sigue siendo la
    /// referencia con la que se compara todo lo medido en el Hito 3.
    InteriorVisible,
    /// El volumen cerrado con su material real: `0.9 / 0.9`, `ior 1.333`.
    /// Ciento sesenta primitivas y óptica completa.
    ///
    /// Es lo que se presenta y lo que hay que medir en el gate de Aguas
    /// Voladoras: el rayo primario refracta en la cara frontal, cruza el
    /// volumen y alcanza el interior.
    RefractiveWater,
    /// El mismo volumen con los **techos ópticos forzados a cero**: sin
    /// reflejo y sin transmisión. Conserva las 160 primitivas.
    ///
    /// Es un control de oclusión, no una escena presentable. Al no
    /// transmitir, oculta las 44 primitivas del interior, que dejan de
    /// probarse; un tiempo medido así parece bueno por la razón
    /// equivocada.
    ///
    /// Conserva `ShadowMode::Ignore` a propósito: el inventario prohíbe que
    /// `A-01` bloquee sombras, y cambiarlo aquí rompería la comparación con
    /// los tiempos del Hito 3.
    OpaqueWater,
}

/// Los cinco materiales finales del inventario más el lienzo inicial.
///
/// Los techos ópticos ya llevan los valores del inventario aunque el Hito 5
/// sea quien los use: forman parte de la definición del material, no del
/// momento en que se leen.
#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub canvas: MaterialId,
    pub water: MaterialId,
    pub wet_basalt: MaterialId,
    pub aged_wood: MaterialId,
    pub meadow: MaterialId,
    pub pictorial_crystal: MaterialId,
}

impl Palette {
    pub fn registrar(scene: &mut Scene) -> Self {
        // Los albedos se escriben con `from_srgb`: son colores elegidos a
        // ojo, no cantidades de energia. Escribirlos con `new` los dejaria
        // como lineales y saldrian bastante mas claros de lo previsto.

        // Lienzo sin pintar: marfil, mate, opaco.
        let canvas = scene.add_material(Material::new(Color::from_srgb(0.90, 0.87, 0.79)));

        // Agua: caps 0.9 y no 1.0. Con caps unitarios kl queda en cero y el
        // albedo del agua no contribuye nunca; con 0.9 queda un 10 % fijo
        // que porta su color propio.
        let water = scene.add_material(Material {
            reflection_cap: 0.9,
            transmission_cap: 0.9,
            ior: 1.333,
            specular_strength: 0.18,
            shininess: 128.0,
            shadow_mode: ShadowMode::Ignore,
            ..Material::new(Color::from_srgb(0.22, 0.45, 0.72))
        });

        // Roca húmeda: brillo local alto, cero rebotes.
        let wet_basalt =
            scene.add_material(Material::wet_basalt(Color::from_srgb(0.26, 0.27, 0.30)));

        let aged_wood = scene.add_material(
            Material::new(Color::from_srgb(0.32, 0.22, 0.14)).with_specular(0.06, 16.0),
        );

        let meadow = scene.add_material(
            Material::new(Color::from_srgb(0.30, 0.52, 0.24)).with_specular(0.04, 8.0),
        );

        // Cristal pictórico: brillo y transparencia parcial. El modo de
        // sombra lo decide cada objeto, no el material.
        let pictorial_crystal = scene.add_material(Material {
            reflection_cap: 0.35,
            transmission_cap: 0.25,
            ior: 1.45,
            specular_strength: 0.55,
            shininess: 110.0,
            ..Material::new(Color::from_srgb(0.62, 0.86, 0.92))
        });

        Palette {
            canvas,
            water,
            wet_basalt,
            aged_wood,
            meadow,
            pictorial_crystal,
        }
    }
}

/// Rutas de las seis texturas de material, relativas a la raíz del
/// proyecto. Las genera `cargo run --bin generate_assets`.
pub const RUTAS_TEXTURAS: [(&str, &str); 6] = [
    ("canvas", "assets/textures/canvas.png"),
    ("water", "assets/textures/water.png"),
    ("wet_basalt", "assets/textures/wet_basalt.png"),
    ("aged_wood", "assets/textures/aged_wood.png"),
    ("meadow", "assets/textures/meadow.png"),
    ("pictorial_crystal", "assets/textures/pictorial_crystal.png"),
];

impl Palette {
    /// Registra la paleta **con las texturas cargadas desde disco**.
    ///
    /// Devuelve error si falta alguna: el plan exige que un asset ausente se
    /// note de inmediato y con su ruta, no que se sustituya en silencio por
    /// un color plano que nadie distinguiría de un material mal ajustado.
    ///
    /// Las escalas UV salen del tamaño de las superficies: el lienzo del
    /// plinto y el césped son grandes y necesitan repetir, mientras que el
    /// casco del barco cabe en una sola aplicación.
    pub fn registrar_con_texturas(scene: &mut Scene, raiz: &Path) -> Result<Palette, TextureError> {
        let base = Palette::registrar(scene);

        let mut ids = Vec::with_capacity(RUTAS_TEXTURAS.len());
        for (_, ruta) in RUTAS_TEXTURAS {
            let textura = Texture::load(&raiz.join(ruta))?;
            ids.push(scene.add_texture(textura));
        }

        let escalas = [
            (base.canvas, ids[0], 6.0),
            (base.water, ids[1], 2.0),
            (base.wet_basalt, ids[2], 3.0),
            (base.aged_wood, ids[3], 1.0),
            (base.meadow, ids[4], 4.0),
            (base.pictorial_crystal, ids[5], 1.5),
        ];

        for (material_id, textura_id, escala) in escalas {
            let material = scene.material(material_id);
            scene.palette[material_id.0] = material.with_texture(textura_id).with_uv_scale(escala);
        }

        Ok(base)
    }
}

/// Rutas de los dos panoramas del skybox, relativas a la raíz del proyecto.
pub const RUTAS_SKYBOX: [(&str, &str); 2] = [
    ("pale", "assets/skybox/pale.png"),
    ("painted", "assets/skybox/painted.png"),
];

/// Carga los dos panoramas y devuelve el cielo que los interpola.
///
/// Igual que las texturas de material: si falta uno, error con su ruta y no
/// un degradado de relleno. Un cielo que se sustituye en silencio es
/// justamente lo que no se nota mirando la imagen, porque un fondo plano
/// también es un fondo plausible.
///
/// Los panoramas se quedan con `WrapMode::Repeat`, que es lo correcto para
/// el azimut: dan la vuelta completa y la columna final empalma con la
/// primera. El cenit lo resuelve `Skybox`, que recorta `v` antes de
/// muestrear.
pub fn cargar_skybox(scene: &mut Scene, raiz: &Path) -> Result<Skybox, TextureError> {
    let mut ids = Vec::with_capacity(RUTAS_SKYBOX.len());
    for (_, ruta) in RUTAS_SKYBOX {
        let panorama = Texture::load(&raiz.join(ruta))?;
        ids.push(scene.add_texture(panorama));
    }

    Ok(Skybox::Panorama {
        pale: ids[0],
        painted: ids[1],
    })
}

/// Cuánta densidad lleva el nivel.
///
/// # Por qué es un parámetro y no dos constructores separados
///
/// Porque el nivel objetivo tiene que ser el seguro **más el lote**, y no
/// otra escena. Si fueran dos generadores, la diferencia entre las dos filas
/// de la matriz dejaría de ser atribuible al lote: cualquier ajuste que se
/// le hiciera a uno y no al otro entraría en la medición disfrazado de
/// densidad.
///
/// Con un parámetro, el nivel seguro sigue siendo **bit a bit** el que
/// aprobó el Hito 6: las entradas generadas continúan la secuencia de la
/// misma semilla en vez de redistribuirse, y las colocadas a mano se
/// añaden al final de su lista.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Density {
    /// Las `160` primitivas del nivel seguro. Es lo que se presenta y lo
    /// que sostiene todos los gates hasta la Tarea 7.1.
    Safe,
    /// El nivel seguro más el lote incremental de la Tarea 7.2 que esté en
    /// evaluación. Es un **candidato**, no lo que se envía; vive para poder
    /// medirlo y mirarlo antes de decidir si se conserva, y para poder
    /// retirarlo cambiando este valor.
    ///
    /// Va por `TARGET`, no por una cifra escrita aquí: han sido `175`, `164`
    /// y `162` en tres revisiones, y una constante repetida en dos sitios se
    /// desincroniza en la primera.
    Target,
}

/// Presupuesto de primitivas por región, según el inventario.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Presupuesto {
    pub global: usize,
    pub meadows: usize,
    pub breakwater: usize,
    pub flying_waters: usize,
}

impl Presupuesto {
    pub fn total(&self) -> usize {
        self.global + self.meadows + self.breakwater + self.flying_waters
    }
}

/// Conteos del nivel seguro. Vienen del presupuesto consolidado del
/// inventario y suman 160.
pub const SAFE: Presupuesto = Presupuesto {
    global: 27,
    meadows: 37,
    breakwater: 38,
    flying_waters: 58,
};

/// Conteos del nivel objetivo con lo que quedó del **lote 2**.
///
/// | Entrada | Seguro | Lote | Objetivo | Máximo del inventario |
/// |---|---:|---:|---:|---:|
/// | `A-11` borde roto | 8 | `+2` | 10 | **10, cerrada** |
///
/// # Dos lotes descartados debajo de este
///
/// El **lote 1** fueron quince primitivas submarinas —lecho, kelp y rocas—:
/// cabía de sobra y cambiaba un `0.05 %` del cuadro que se presenta.
///
/// El **lote 2** añadió dos piezas al casco y dos al borde roto. Las del
/// borde se leen; las del casco cambiaban un `0.04 %` y se retiraron con el
/// mismo argumento que el lote 1, porque tienen la misma causa: están dentro
/// del volumen de agua, y ahí el reflejo se come el detalle.
///
/// Ninguno sigue debajo de este candidato. Cada lote parte otra vez de las
/// `160` del nivel seguro; no se acumulan.
///
/// # Por qué solo estas dos
///
/// Porque son las únicas que la medición defiende. Están en **primer plano**
/// y no detrás de la superficie refractiva, que es la diferencia que separa
/// lo que se lee de lo que no en los tres experimentos.
///
/// Y con ellas `A-11` llega a su máximo del inventario. Si se conservan, esa
/// entrada queda cerrada.
pub const TARGET: Presupuesto = Presupuesto {
    global: 27,
    meadows: 37,
    breakwater: 38,
    flying_waters: 60,
};

/// Construye el nivel seguro completo.
///
/// Comprueba el conteo de cada región contra el presupuesto en vez de
/// confiar en que cuadre: una entrada que se pase de largo desplazaría el
/// total y el benchmark mediría otra escena distinta de la declarada.
pub fn safe_level(water: WaterPreset) -> Blockout {
    // Sin texturas no hay nada que cargar, asi que no puede fallar.
    safe_level_con(water, None).expect("sin assets no hay error posible")
}

/// El nivel objetivo con el primer lote de la Tarea 7.2. Ver `Density`.
pub fn target_level(water: WaterPreset) -> Blockout {
    nivel_con(water, Density::Target, None).expect("sin assets no hay error posible")
}

/// Igual que `target_level`, pero cargando las texturas desde `raiz`.
pub fn target_level_con(
    water: WaterPreset,
    raiz_assets: Option<&Path>,
) -> Result<Blockout, TextureError> {
    nivel_con(water, Density::Target, raiz_assets)
}

/// Igual que `safe_level`, pero cargando las texturas desde `raiz`.
///
/// Con `None` la escena queda con colores planos, que es lo que usan los
/// tests: así `cargo test` no depende de que los assets estén generados.
/// Los binarios pasan la raíz del proyecto y obtienen la versión
/// texturizada, o un error con la ruta del asset que falte.
pub fn safe_level_con(
    water: WaterPreset,
    raiz_assets: Option<&Path>,
) -> Result<Blockout, TextureError> {
    nivel_con(water, Density::Safe, raiz_assets)
}

/// El constructor de verdad, con la densidad como parámetro.
///
/// Los dos niveles salen de aquí para que no puedan divergir en nada que no
/// sea el lote. Ver `Density`.
fn nivel_con(
    water: WaterPreset,
    densidad: Density,
    raiz_assets: Option<&Path>,
) -> Result<Blockout, TextureError> {
    let mut scene = Scene::new();
    let mut plan = ClusterPlan::new();
    let paleta = match raiz_assets {
        Some(raiz) => {
            let paleta = Palette::registrar_con_texturas(&mut scene, raiz)?;
            // El cielo se carga con el resto de los assets: sin panoramas
            // la escena se queda con el color plano por defecto, que es lo
            // que ven los tests.
            let cielo = cargar_skybox(&mut scene, raiz)?;
            scene.skybox = cielo;

            paleta
        }
        None => Palette::registrar(&mut scene),
    };

    let anchors_base = anclas_del_diorama();

    let antes = scene.objects.len();
    let monolith_height = continent::globales(&mut scene, &paleta, &anchors_base);
    verificar("Global", scene.objects.len() - antes, SAFE.global);

    let antes = scene.objects.len();
    meadows::praderas(&mut scene, &paleta, anchors_base.meadows_anchor);
    verificar("Praderas", scene.objects.len() - antes, SAFE.meadows);

    let antes = scene.objects.len();
    breakwater::rompeolas_seguro(
        &mut scene,
        &mut plan,
        &paleta,
        anchors_base.breakwater_anchor,
    );
    verificar("Rompeolas", scene.objects.len() - antes, SAFE.breakwater);

    let antes = scene.objects.len();
    let water_surface_y = flying_waters::aguas_voladoras(
        &mut scene,
        &paleta,
        anchors_base.flying_waters_anchor,
        anchors_base.broken_edge_anchor,
        water,
        densidad,
    );

    let presupuesto = match densidad {
        Density::Safe => SAFE,
        Density::Target => TARGET,
    };
    let esperado = match water {
        WaterPreset::RefractiveWater | WaterPreset::OpaqueWater => presupuesto.flying_waters,
        // Sin el volumen de agua queda una primitiva menos.
        WaterPreset::InteriorVisible => presupuesto.flying_waters - 1,
    };
    verificar("Aguas Voladoras", scene.objects.len() - antes, esperado);

    // Medición: primero la geometría, después la escala.
    let orbit_center = anchors_base.monolith_base_anchor;
    let scene_radius = measure_scene_radius(&scene, orbit_center);
    let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

    let anchors = SceneAnchors {
        look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
        hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
        flying_waters_anchor: Vec3::new(
            anchors_base.flying_waters_anchor.x,
            water_surface_y,
            anchors_base.flying_waters_anchor.z,
        ),
        // Del ancla **base**, no de la ya desplazada a la superficie.
        boat_anchor: flying_waters::centro_visible_del_barco(anchors_base.flying_waters_anchor),
        ..anchors_base
    };

    let accel =
        SceneAccel::build_from_plan(&scene, &plan).expect("el nivel seguro tiene geometria");

    Ok(Blockout {
        scene,
        accel,
        anchors,
        scale: SceneScale {
            scene_radius,
            monolith_height,
            water_surface_y,
            orbit_radius,
        },
    })
}

fn verificar(region: &str, obtenido: usize, esperado: usize) {
    assert_eq!(
        obtenido, esperado,
        "{region} genero {obtenido} primitivas y el inventario declara {esperado}"
    );
}

/// Anclas del diorama, compartidas por el blockout y el nivel seguro.
///
/// Son las que quedaron aprobadas en la validación del Blockout 1; el nivel
/// seguro puebla esa misma composición con el detalle del inventario en vez
/// de proponer una nueva.
pub fn anclas_del_diorama() -> SceneAnchors {
    let origen = Vec3::zeros();

    SceneAnchors {
        scene_origin: origen,
        monolith_base_anchor: origen,
        orbit_center: origen,
        look_at: origen,
        meadows_anchor: Vec3::new(-4.2, 5.6, -4.6),
        breakwater_anchor: Vec3::new(-4.2, 2.4, -1.9),
        flying_waters_anchor: Vec3::new(0.0, 0.0, 4.2),
        boat_anchor: flying_waters::centro_visible_del_barco(Vec3::new(0.0, 0.0, 4.2)),
        palette_anchor: Vec3::new(6.6, 0.4, 5.8),
        hero_camera_anchor: origen,
        broken_edge_anchor: Vec3::new(0.0, 1.2, 6.6),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Bordes de una escena, para comparar geometría sin comparar índices.
    fn cajas(nivel: &Blockout) -> Vec<crate::bounds::Aabb> {
        nivel
            .scene
            .objects
            .iter()
            .map(|o| o.primitive.bounds())
            .collect()
    }

    /// Los ocho bloques de `A-11` en un nivel, por sus propiedades.
    ///
    /// Por profundidad y no solo por cercania en Z: el borde se genera con
    /// una profundidad fija de `1.1`, y filtrar solo por Z recogia dos masas
    /// de otra entrada que pasan cerca.
    fn fila_del_borde(nivel: &Blockout, borde: Vec3) -> Vec<crate::bounds::Aabb> {
        const PROFUNDIDAD_DE_LA_FILA: f32 = 1.1;

        cajas(nivel)
            .into_iter()
            .filter(|c| {
                (0.5 * (c.min.z + c.max.z) - borde.z).abs() < 0.5
                    && (c.max.z - c.min.z - PROFUNDIDAD_DE_LA_FILA).abs() < 1e-3
            })
            .collect()
    }

    /// Lo que el objetivo tiene y el seguro no: el lote, sin depender del
    /// orden en que se generen las entradas.
    fn sobrante(seguro: &Blockout, objetivo: &Blockout) -> Vec<crate::bounds::Aabb> {
        let mut disponibles = cajas(objetivo);

        for caja in cajas(seguro) {
            let i = disponibles
                .iter()
                .position(|c| {
                    (c.min - caja.min).magnitude() < 1e-5 && (c.max - caja.max).magnitude() < 1e-5
                })
                .expect("una primitiva del nivel seguro no esta en el objetivo");

            disponibles.swap_remove(i);
        }

        disponibles
    }

    #[test]
    fn el_lote_son_dos_primitivas_y_agota_el_maximo_de_a11() {
        // El conteo region por region y no solo en el total.
        assert_eq!(TARGET.global, SAFE.global);
        assert_eq!(TARGET.meadows, SAFE.meadows);
        assert_eq!(TARGET.breakwater, SAFE.breakwater);
        assert_eq!(TARGET.flying_waters, SAFE.flying_waters + 2);

        assert_eq!(SAFE.total(), 160);
        assert_eq!(TARGET.total(), 162);
    }

    #[test]
    fn el_lote_no_toca_el_casco_y_deja_a11_en_su_maximo() {
        // Se cuenta la escena, no se relee el presupuesto: si el generador y
        // la constante discreparan, releer la constante no lo notaria.
        //
        // `A-03` tiene que seguir en doce. El lote 2 le probo dos piezas y se
        // retiraron por no leerse; que vuelvan sin querer es justo el tipo de
        // regresion que este test existe para ver.
        let objetivo = flying_waters::entradas_del_objetivo();

        assert_eq!(
            objetivo.casco, 12,
            "A-03 no debe crecer: sus dos piezas se retiraron"
        );
        assert_eq!(
            objetivo.borde_roto, 10,
            "A-11 tiene un maximo de 10 y el lote lo agota"
        );
    }

    #[test]
    fn el_nivel_seguro_no_se_movio_con_el_lote() {
        // La comprobacion que hace honesta la matriz: la fila `safe` tiene
        // que seguir midiendo **la misma escena** que aprobo el Hito 6.
        let seguro = safe_level(WaterPreset::RefractiveWater);

        assert_eq!(seguro.scene.objects.len(), 160);
    }

    #[test]
    fn el_objetivo_es_el_seguro_mas_el_lote() {
        // Superconjunto y no escena nueva: cada primitiva del nivel seguro
        // tiene que aparecer **igual** en el objetivo. Se compara por cajas y
        // no por indices porque el orden de generacion no tiene por que
        // conservarse aunque la geometria si.
        let seguro = safe_level(WaterPreset::RefractiveWater);
        let objetivo = target_level(WaterPreset::RefractiveWater);

        assert_eq!(objetivo.scene.objects.len(), 162);
        assert_eq!(sobrante(&seguro, &objetivo).len(), 2);
    }

    #[test]
    fn el_lote_cruza_la_cara_frontal_del_agua() {
        // Es lo que hace que el desgarro **ocluya** el agua en vez de
        // flotar delante de ella. Y no es solo composicion: los doscientos
        // rayos secundarios que este lote ahorra salen justo de esa
        // oclusion, asi que un bloque despegado de la cara frontal costaria
        // en vez de ahorrar.
        //
        // La revision 2c los adelanto en Z, que es exactamente el cambio que
        // podia despegarlos. Por eso este test existe desde esa revision.
        let seguro = safe_level(WaterPreset::RefractiveWater);
        let objetivo = target_level(WaterPreset::RefractiveWater);

        let (centro, tamano) =
            flying_waters::caja_del_volumen(anclas_del_diorama().flying_waters_anchor);
        let cara_frontal = centro.z + tamano.z * 0.5;

        for caja in sobrante(&seguro, &objetivo) {
            assert!(
                caja.min.z < cara_frontal && caja.max.z > cara_frontal,
                "una pieza del lote no cruza la cara frontal en z = {cara_frontal}: {caja:?}"
            );
        }
    }

    #[test]
    fn el_lote_rompe_la_alineacion_de_la_fila() {
        // El diagnostico de la revision 2b: alargar la fila la convertia en
        // una pared. La 2c adelanta los dos bloques, y esto exige que el
        // adelanto se note **contra la propia fila** en vez de contra una
        // constante escrita aqui.
        //
        // La referencia sale de la escena: la sacudida en Z que ya tenian
        // los ocho bloques del nivel seguro. Si un bloque nuevo cae dentro
        // de esa banda, no rompe nada.
        let seguro = safe_level(WaterPreset::RefractiveWater);
        let objetivo = target_level(WaterPreset::RefractiveWater);
        let borde = anclas_del_diorama().broken_edge_anchor;

        // Los ocho del borde, identificados por **propiedades** y no por
        // indice: centro cerca del ancla del borde y la profundidad fija de
        // `1.1` con la que se generan. Solo por cercania en Z se colaban dos
        // masas de otra entrada.
        let fila: Vec<f32> = fila_del_borde(&seguro, borde)
            .iter()
            .map(|c| 0.5 * (c.min.z + c.max.z))
            .collect();

        assert_eq!(
            fila.len(),
            8,
            "no se reconocieron los ocho bloques del borde"
        );

        let sacudida = fila
            .iter()
            .map(|z| (z - borde.z).abs())
            .fold(0.0_f32, f32::max);

        for caja in sobrante(&seguro, &objetivo) {
            let adelanto = 0.5 * (caja.min.z + caja.max.z) - borde.z;

            assert!(
                adelanto > sacudida,
                "un bloque del lote se queda dentro de la sacudida de la fila: \
                 adelanto {adelanto}, sacudida {sacudida}"
            );
        }
    }

    #[test]
    fn el_lote_no_ensancha_la_fila() {
        // La otra mitad del diagnostico. La revision 2b crecia por los
        // extremos y la silueta se hacia mas ancha; la 2c tiene que caber
        // **dentro** del ancho que la fila ya ocupaba.
        let seguro = safe_level(WaterPreset::RefractiveWater);
        let objetivo = target_level(WaterPreset::RefractiveWater);
        let borde = anclas_del_diorama().broken_edge_anchor;

        let fila = fila_del_borde(&seguro, borde);

        let izquierda = fila.iter().map(|c| c.min.x).fold(f32::INFINITY, f32::min);
        let derecha = fila
            .iter()
            .map(|c| c.max.x)
            .fold(f32::NEG_INFINITY, f32::max);

        for caja in sobrante(&seguro, &objetivo) {
            assert!(
                caja.min.x >= izquierda && caja.max.x <= derecha,
                "un bloque del lote ensancha la fila: {caja:?} fuera de [{izquierda}, {derecha}]"
            );
        }
    }

    #[test]
    fn el_lote_esta_delante_del_agua_y_no_dentro() {
        // La leccion de los dos lotes descartados, convertida en test. Lo que
        // esta dentro del volumen no se lee, asi que el lote que sobrevivio
        // tiene que estar **fuera y delante**: entre el agua y la camara
        // hero, que mira desde `+Z`.
        //
        // Los tres ejes, y no dos: la primera version comprobaba X y Z, y una
        // pieza que asomara por encima de la superficie habria pasado.
        let seguro = safe_level(WaterPreset::RefractiveWater);
        let objetivo = target_level(WaterPreset::RefractiveWater);

        let (centro, tamano) =
            flying_waters::caja_del_volumen(anclas_del_diorama().flying_waters_anchor);

        for caja in sobrante(&seguro, &objetivo) {
            let dentro = caja.min.x > centro.x - tamano.x * 0.5
                && caja.max.x < centro.x + tamano.x * 0.5
                && caja.min.y > centro.y - tamano.y * 0.5
                && caja.max.y < centro.y + tamano.y * 0.5
                && caja.min.z > centro.z - tamano.z * 0.5
                && caja.max.z < centro.z + tamano.z * 0.5;

            assert!(
                !dentro,
                "una pieza del lote quedo dentro del agua: {caja:?}"
            );
            assert!(
                caja.min.z > centro.z,
                "una pieza del lote quedo detras del agua: {caja:?}"
            );
        }
    }

    #[test]
    fn el_lote_no_tapa_la_cadena_ni_el_ancla() {
        // El criterio explicito de la autorizacion. La cadena baja por el
        // centro de la bahia hacia `+X`, y los bloques del lote van a los
        // lados: se exige separacion en X en vez de confiarla al comentario.
        //
        // El corredor se **mide**, no se rellena a ojo. La primera version
        // puso `+0.5 .. +2.6` a mano y fallo contra un bloque que empieza en
        // `+2.575`, cuando el brazo del ancla acaba en `+2.56`: el test
        // estaba midiendo su propio margen y no la cadena.
        let seguro = safe_level(WaterPreset::RefractiveWater);
        let objetivo = target_level(WaterPreset::RefractiveWater);
        let bahia = anclas_del_diorama().flying_waters_anchor;

        let (corredor_min, corredor_max) = flying_waters::corredor_de_la_cadena(bahia);

        for caja in sobrante(&seguro, &objetivo) {
            let solapa = caja.max.x > corredor_min && caja.min.x < corredor_max;

            assert!(
                !solapa,
                "una pieza del lote se cruza con la cadena en X: {caja:?}                  contra el corredor [{corredor_min}, {corredor_max}]"
            );
        }
    }

    #[test]
    fn el_lote_es_reversible_sin_tocar_el_generador() {
        // El criterio de la autorizacion: retirar el lote tiene que ser
        // cambiar un parametro, no deshacer un cambio.
        for water in [WaterPreset::RefractiveWater, WaterPreset::InteriorVisible] {
            let seguro = safe_level(water);
            let objetivo = target_level(water);

            assert_eq!(objetivo.scene.objects.len() - seguro.scene.objects.len(), 2);
        }
    }

    /// En el estado sin pintar, lo unico que no es lienzo son las seis
    /// piezas de `G-04`.
    ///
    /// Es el contrato de la revelacion visto del lado de la escena: si un
    /// objeto naciera ya con su material final, pintarlo no cambiaria nada
    /// y el fallo pasaria inadvertido hasta el Hito 6. Es exactamente lo
    /// que ocurrio antes de partir `masa` en dos.
    #[test]
    fn sin_pintar_solo_la_paleta_escapa_del_lienzo() {
        for water in [WaterPreset::InteriorVisible, WaterPreset::OpaqueWater] {
            let diorama = safe_level(water);
            let paleta_canvas = diorama.scene.objects[0].initial_material;

            let ajenos: Vec<_> = diorama
                .scene
                .objects
                .iter()
                .filter(|objeto| objeto.initial_material != paleta_canvas)
                .collect();

            assert_eq!(
                ajenos.len(),
                6,
                "{water:?}: {} objetos no nacen en lienzo",
                ajenos.len()
            );

            for objeto in ajenos {
                assert_eq!(
                    objeto.spatial_group,
                    SpatialGroupId::InteractionProps,
                    "un objeto fuera de G-04 no nace en lienzo"
                );
                // Y son inertes: la herramienta no se pinta.
                assert_eq!(objeto.initial_material, objeto.final_material);
            }
        }
    }

    #[test]
    fn el_presupuesto_seguro_suma_160() {
        assert_eq!(SAFE.total(), 160);
    }

    #[test]
    fn el_preset_opaco_conserva_las_160_primitivas() {
        let nivel = safe_level(WaterPreset::OpaqueWater);

        assert_eq!(nivel.scene.objects.len(), 160);
    }

    #[test]
    fn el_preset_de_interior_visible_deja_159() {
        let nivel = safe_level(WaterPreset::InteriorVisible);

        assert_eq!(nivel.scene.objects.len(), 159);
    }

    #[test]
    fn la_unica_diferencia_entre_presets_es_el_volumen_de_agua() {
        let opaco = safe_level(WaterPreset::OpaqueWater);
        let visible = safe_level(WaterPreset::InteriorVisible);

        assert_eq!(opaco.scene.objects.len(), visible.scene.objects.len() + 1);

        // Y todo lo demas es identico, en el mismo orden: el volumen se
        // inserta al final justo para que quitarlo no desplace nada.
        for (a, b) in visible.scene.objects.iter().zip(&opaco.scene.objects) {
            assert_eq!(a.primitive.bounds(), b.primitive.bounds());
            assert_eq!(a.spatial_group, b.spatial_group);
        }
    }

    #[test]
    fn el_preset_opaco_oculta_44_primitivas_del_interior() {
        // La razon por la que el inventario prohibe medir rendimiento con
        // agua opaca: 44 primitivas dejan de probarse.
        assert_eq!(flying_waters::PRIMITIVAS_INTERIORES, 44);
    }

    #[test]
    fn cada_region_respeta_su_presupuesto() {
        let nivel = safe_level(WaterPreset::OpaqueWater);
        let cuenta = |grupo| {
            nivel
                .scene
                .objects
                .iter()
                .filter(|o| o.spatial_group == grupo)
                .count()
        };

        use crate::scene::SpatialGroupId as G;
        let global = cuenta(G::Global)
            + cuenta(G::ContinentBackground)
            + cuenta(G::Monolith)
            + cuenta(G::InteractionProps);

        assert_eq!(global, SAFE.global, "Global");
        assert_eq!(cuenta(G::Meadows), SAFE.meadows, "Praderas");
        assert_eq!(cuenta(G::Breakwater), SAFE.breakwater, "Rompeolas");
        assert_eq!(cuenta(G::FlyingWaters), SAFE.flying_waters, "Aguas");
    }

    #[test]
    fn rompeolas_conserva_sus_cuatro_clusters_en_el_nivel_seguro() {
        let nivel = safe_level(WaterPreset::InteriorVisible);

        let grupo = nivel
            .accel
            .groups
            .iter()
            .find(|g| g.id == crate::scene::SpatialGroupId::Breakwater)
            .expect("existe Rompeolas");

        // Los 28 pilares en cuatro tramos, mas el sendero y las masas de
        // soporte que caen en el cluster por defecto.
        assert!(
            grupo.clusters.len() >= 4,
            "los cuatro tramos del arco deben sobrevivir al ensamblado: {}",
            grupo.clusters.len()
        );
    }

    #[test]
    fn el_xorshift_no_se_queda_clavado_en_cero() {
        let mut generador = Xorshift32::new(0);
        let valores: Vec<f32> = (0..8).map(|_| generador.siguiente()).collect();

        assert!(valores.iter().all(|v| (0.0..1.0).contains(v)));
        assert!(valores.windows(2).any(|par| par[0] != par[1]));
    }
}
