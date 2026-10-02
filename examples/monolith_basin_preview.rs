//! Preview de la **cuenca del Monolito**: la entrega vigente contra una
//! candidata en la que las cataratas de Praderas bajan hasta la superficie
//! que las recibe y un agua poco profunda, de huella irregular, ocupa los
//! claros de la base alrededor del Monolito.
//!
//! ```bash
//! cargo run --release --example monolith_basin_preview -- <carpeta>
//! ```
//!
//! La carpeta es **obligatoria**: el ejemplo no escribe evidencia oficial.
//! Deja veintiún PNG a `800 × 600`, con los assets reales y
//! `RevealState::painted`, para `current`, `candidate`, `flooded_gaps`,
//! `flooded_blue`, `flooded_reflective`, `flooded_blue_raised` y `delivery`
//! —la entrega de producción, `delivery_level_con`, que tras la promoción
//! tiene que salir idéntica a `flooded_blue_raised`—:
//!
//! - `<variante>_hero.png` — la toma hero.
//! - `<variante>_top78.png` — la misma órbita a `78°`, la cámara de
//!   `render_scene --elevation 78`.
//! - `<variante>_monolith.png` — un encuadre cercano **explícito**
//!   (`camara_del_monolito`) del pie del Monolito, solo para entender la
//!   cuenca. No sustituye a la hero.
//!
//! Las siete columnas comparten cámaras y luces, medidas sobre la entrega.
//!
//! # Qué cambia
//!
//! Las variantes se construyen sobre `delivery_level_previo_cuenca_con`, la
//! entrega anterior a la cuenca. `flooded_blue_raised` se promovió al
//! default en `src/scenes/monolith_basin_delivery.rs`; el test
//! `la_entrega_de_produccion_es_flooded_blue_raised` lo comprueba con los
//! assets reales.
//!
//! - **Cuatro caídas se alargan.** Las cuatro exteriores de Praderas nacen en
//!   el canto de la meseta flotante y hoy terminan en el aire, entre `2.17` y
//!   `3.42`. Cada una baja, en su misma planta, hasta la superficie que tiene
//!   debajo: tres al plinto (`y = 0`) y la cuarta a la masa de fondo que
//!   tiene debajo, a `1.70`. Ver `prolongar_caidas`. Las dos interiores ya
//!   desembocan en el llano de la meseta y no se tocan.
//! - **Seis pozas nuevas.** Agua somera —de `0.08` a `0.12` sobre el
//!   plinto—, apoyada en él y separada por costuras secas: ni piscina
//!   rectangular ni inundación. Ver `POZAS`.
//! - **Un material nuevo**, copia por valor del agua aprobada de `A-01`:
//!   mismos techos `0.9 / 0.9`, `ior 1.333` y modo de sombra; color propio
//!   verde azulado y un brillo moderado. Reusa la textura del agua sin
//!   tocarla. Ver `TINTE_DE_LA_CUENCA`.
//!
//! Todo lo demás —Rompeolas, Aguas con su pecio, su cadena y su volumen,
//! Praderas salvo esas cuatro alturas, la base, sus masas y el Monolito— es la
//! entrega byte a byte.
//!
//! # Presupuesto
//!
//! `168 + 6 = 174` primitivas. Es **experimental**: el conteo se informa para
//! que se pueda decidir, no autoriza nada.
//!
//! # `flooded_gaps`
//!
//! La corrección de Charlie sobre `exp33/rojito.png`: más agua en todo lo
//! que no tenga bloques encima —los huecos a los lados del Monolito, el gran
//! claro del lado derecho detrás de Aguas y el canal entre el Rompeolas y
//! Aguas hasta el frente—, como una lámina continua y no como pozas sueltas.
//!
//! - Las mismas cuatro caídas alargadas que la candidata.
//! - **La base libre se mide**, no se dibuja: el techo del plinto menos su
//!   `BORDE` y menos la planta de **toda** pieza que haya encima —masas,
//!   islas, Rompeolas, Aguas, la meseta flotante con lo que cuelga de ella y
//!   el Monolito—, con `HOLGURA`. Se reparte en rectángulos de interiores
//!   disjuntos, el mayor primero, y se descartan las astillas. Ver
//!   `huella_libre`. Se mide en tiempo de ejecución, así que vale para las
//!   dos rutas.
//! - **Una lámina de altura común**, `TECHO_DEL_AGUA` sobre el plinto, con
//!   el agua aprobada de `A-01`; las celdas se tocan y no se solapan.
//! - **Un lecho oscuro** local, entre `LECHO.0` y `LECHO.1`, retirado
//!   `RETRANQUEO_DEL_LECHO` del borde del agua: lo que el rayo refractado ve
//!   en vez del lienzo claro. Ver `lechos`.
//!
//! El coste —celdas de agua más lecho— se imprime al correr y lo fijan los
//! tests. Es **experimental**: no autoriza el default.
//!
//! Límite del renderer: cada impacto decide entrar o salir del agua por
//! `front_face`, sin pila de medios. Un rayo refractado que llega a la cara
//! que comparten dos celdas «sale» al aire —o se refleja por completo— aunque
//! al otro lado siga el agua. La lámina es fina sobre el lecho para que casi
//! ningún rayo llegue antes a esa cara; el test de costuras lo mide.
//!
//! # `flooded_blue`
//!
//! El pulido que pidió Charlie sobre `flooded_gaps`: agua más clara y
//! azulada, con un fondo de piedra submarina debajo y el continente como
//! varias islas. **La geometría es la de `flooded_gaps`, entera**
//! —`168 + 19 + 18 = 205`—: solo cambia el material final de las celdas y
//! del lecho.
//!
//! - El agua: copia del agua de `flooded_gaps` —mismos techos, `ior`,
//!   sombra y textura—, con un tinte cian marino claro y un brillo algo más
//!   suave. Ver `TINTE_MARINO`.
//! - El fondo: con los techos aprobados, mirando de frente, el rayo
//!   refractado se lleva el `88 %` y ve el lecho, así que el color del agua
//!   lo decide sobre todo el lecho. Cada pieza de lecho lleva su material y
//!   su textura procedural, muestreada **en coordenadas de mundo** sobre un
//!   único campo de piedras azul pizarra (`campo_de_roca`): el fondo sigue
//!   de una pieza a la vecina y no se estira en las grandes, que es lo que
//!   haría una textura compartida con la `uv` normalizada por cara.
//!
//! Sin luces nuevas, sin emisión y sin tocar techos ni refracción.
//!
//! # `flooded_reflective`
//!
//! `flooded_blue` entera con un agua más satinada: solo cambia el brillo
//! especular directo de las 19 celdas (`BRILLO_REFLECTANTE`). Tinte,
//! textura, techos, `ior`, sombra, lecho, fondos y geometría son los de
//! `flooded_blue`.
//!
//! Por qué no es un espejo: el reflejo trazado pesa `0.9 × F`, y con
//! `ior 1.333` un plano horizontal visto desde la hero o la cenital refleja
//! el `2–4 %`, **igual que `A-01`**. Lo que hace leerse a Aguas Voladoras
//! como agua es otra cosa, y está medido en `reflejo_diagnostico_de_la_causa`:
//! la profundidad (`1.7` frente a `0.06` de recorrido refractado), las
//! paredes del volumen y la luz `L-02`, que el light linking reserva a
//! `FlyingWaters`.
//!
//! # `flooded_blue_raised`
//!
//! La elegida por Charlie —`flooded_blue`— con más agua: el techo común de
//! las 19 celdas sube de `0.08` a `NIVEL_ELEVADO` sobre el plinto. Es el
//! **único** cambio de geometría; el fondo de las celdas, su planta, el
//! lecho, los materiales azules, las caídas y lo demás son los de
//! `flooded_blue`. El nivel se eligió midiendo en 3D; ver `NIVEL_ELEVADO`.
//!
//! # Qué no es
//!
//! Sin animación, sin simulación, sin absorción y sin cambios en el motor.

fn main() -> std::process::ExitCode {
    imp::correr()
}

mod imp {
    use std::path::{Path, PathBuf};
    use std::process::ExitCode;

    use expedition33_continente_inacabado::accel::{ClusterPlan, SceneAccel};
    use expedition33_continente_inacabado::bounds::Aabb;
    use expedition33_continente_inacabado::camera::Camera;
    use expedition33_continente_inacabado::color::Color;
    use expedition33_continente_inacabado::cuboid::Cuboid;
    use expedition33_continente_inacabado::framebuffer::Framebuffer;
    use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
    use expedition33_continente_inacabado::renderer::{render, Shading};
    use expedition33_continente_inacabado::reveal::RevealState;
    use expedition33_continente_inacabado::scene::{
        MaterialId, RevealGroup, Scene, SceneObject, SpatialGroupId,
    };
    use expedition33_continente_inacabado::scene_builder::{Blockout, HERO_YAW_DEGREES};
    use expedition33_continente_inacabado::scenes::{
        delivery_level_previo_cuenca_con, delivery_level_previo_monolito_con,
        delivery_level_previo_relleno_con, delivery_level_previo_trasero_con, WaterPreset,
    };
    use expedition33_continente_inacabado::texture::Texture;
    use nalgebra_glm::Vec3;

    pub const ANCHO: usize = 800;
    pub const ALTO: usize = 600;
    pub const ELEVACION_CENITAL: f32 = 78.0;

    /// Las treinta y ocho piezas del Rompeolas original: el plan de la
    /// entrega las pone en un cluster y la segunda formación en otro.
    const ROMPEOLAS_ORIGINAL: usize = 38;

    /// Lo que una pieza se hunde en su apoyo: separa caras coplanares. Es el
    /// mismo valor que usa Praderas.
    pub const EMPOTRADO: f32 = 0.04;

    /// Las seis pozas: `(planta [x0, z0, x1, z1], techo)`, relativas al ancla
    /// de la base del Monolito. El fondo de todas va `EMPOTRADO` dentro del
    /// plinto.
    ///
    /// - dos al pie de las caídas, detrás del Monolito: la del oeste recibe
    ///   dos caídas y la del este una; entre ellas queda el corredor seco de
    ///   detrás del pedestal;
    /// - una franja a cada flanco, partida en dos al oeste por una costura
    ///   seca;
    /// - las costuras se miden en las dos rutas: en la B la losa del
    ///   Rompeolas y sus dos masas vecinas quedan `0.15` más al fondo;
    /// - una al frente este, entre la masa del Rompeolas y la bahía, a
    ///   `0.30` del lecho de Aguas (`A-02`, que empieza en `z = 3.404`).
    pub const POZAS: [([f32; 4], f32); 6] = [
        ([-3.45, -2.99, -1.75, -1.55], 0.12),
        ([1.75, -2.99, 3.00, -1.20], 0.10),
        ([-2.05, -1.40, -1.55, 0.00], 0.09),
        ([1.60, -1.00, 3.10, 1.90], 0.11),
        ([1.25, 2.05, 2.90, 3.10], 0.08),
        ([-2.10, 0.20, -1.65, 1.70], 0.10),
    ];

    /// Tinte de la cuenca, en sRGB: un verde azulado oscuro.
    ///
    /// El reparto del agua aprobada es `0.9 / 0.9`: mirando casi de frente,
    /// el rayo refractado se lleva el `88 %` y ve el lienzo claro del plinto,
    /// y el renderer no tiñe la transmisión. Lo único que separa la poza del
    /// lienzo seco es el `10 %` de color propio y el brillo. Un color propio
    /// oscuro y verdoso deja la poza algo más oscura y fría que el suelo seco
    /// que la rodea, que es lo que la hace leerse como agua somera.
    const TINTE_DE_LA_CUENCA: [f32; 3] = [0.22, 0.50, 0.55];

    /// Brillo de las pozas: `(fuerza, exponente)`. Más concentrado que el
    /// de las caídas: en un plano horizontal amplio, el `0.80 / 6` de ellas
    /// lo blanquea entero bajo la luz principal.
    const BRILLO: (f32, f32) = (0.30, 32.0);

    /// La entrega anterior a la promoción de la cuenca: la línea base sobre
    /// la que se aprobaron todas las variantes. Desde la promoción,
    /// `delivery_level_con` ya lleva `flooded_blue_raised`.
    pub fn actual(raiz: &Path) -> Blockout {
        delivery_level_previo_cuenca_con(WaterPreset::RefractiveWater, Some(raiz))
            .expect("los assets reales tienen que estar en la raiz del proyecto")
    }

    pub fn indices(scene: &Scene, grupo: SpatialGroupId) -> Vec<usize> {
        scene
            .objects
            .iter()
            .enumerate()
            .filter(|(_, o)| o.spatial_group == grupo)
            .map(|(i, _)| i)
            .collect()
    }

    /// Índices de escena de las seis caídas de Praderas: sus piezas de
    /// agua, en orden de índice. Las cuatro primeras caen al vacío desde el
    /// canto; las dos últimas, de la pared interior al llano.
    pub fn caidas(scene: &Scene) -> Vec<usize> {
        let caidas: Vec<usize> = indices(scene, SpatialGroupId::Meadows)
            .into_iter()
            .filter(|&i| scene.material(scene.objects[i].final_material).ior > 1.0)
            .collect();
        assert_eq!(caidas.len(), 6, "Praderas tiene seis caidas de agua");
        caidas
    }

    /// Una caída alargada: qué objeto, dónde acababa, dónde acaba y sobre qué.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Prolongacion {
        pub objeto: usize,
        pub pie_previo: f32,
        pub pie: f32,
        pub apoyo: usize,
    }

    pub struct Candidato {
        pub diorama: Blockout,
        pub objetos_previos: usize,
        pub paleta_previa: usize,
        pub texturas_previas: usize,
        pub prolongadas: Vec<Prolongacion>,
        pub agua: Option<MaterialId>,
    }

    /// Fracción de la planta de `a` que cubre la de `b`.
    fn cubierta(a: &Aabb, b: &Aabb) -> f32 {
        let dx = (a.max.x.min(b.max.x) - a.min.x.max(b.min.x)).max(0.0);
        let dz = (a.max.z.min(b.max.z) - a.min.z.max(b.min.z)).max(0.0);
        dx * dz / ((a.max.x - a.min.x) * (a.max.z - a.min.z))
    }

    /// Lo que se le pide a una pieza para contar como apoyo de una caída:
    /// cubrir al menos la mitad de su planta. La roca que respalda cada caída
    /// exterior solo la muerde un centímetro y no la sostiene.
    const APOYO_MINIMO: f32 = 0.5;

    const EPS: f32 = 1.0e-4;

    /// Alarga cada caída que termina en el aire hasta la superficie que
    /// tiene debajo, en su misma planta, y la hunde `EMPOTRADO` en ella.
    ///
    /// No se fuerza un plano común: cada caída baja hasta **lo que tiene
    /// debajo**. Una caída que ya apoya —las dos interiores, en el llano de
    /// la meseta— no se toca.
    fn prolongar_caidas(scene: &mut Scene) -> Vec<Prolongacion> {
        let mut prolongadas = Vec::new();

        for i in caidas(scene) {
            let b = scene.objects[i].primitive.bounds();
            let opacas =
                |j: usize| j != i && scene.material(scene.objects[j].final_material).ior <= 1.0;
            let apoya = (0..scene.objects.len()).filter(|&j| opacas(j)).any(|j| {
                let o = scene.objects[j].primitive.bounds();
                cubierta(&b, &o) >= APOYO_MINIMO
                    && o.min.y - EPS <= b.min.y
                    && b.min.y <= o.max.y + EPS
            });
            if apoya {
                continue;
            }

            let (apoyo, techo) = (0..scene.objects.len())
                .filter(|&j| opacas(j))
                .filter_map(|j| {
                    let o = scene.objects[j].primitive.bounds();
                    (cubierta(&b, &o) >= APOYO_MINIMO && o.max.y <= b.min.y + EPS)
                        .then_some((j, o.max.y))
                })
                .max_by(|x, y| x.1.total_cmp(&y.1))
                .expect("bajo cada caida hay al menos el plinto");

            let pie = techo - EMPOTRADO;
            scene.objects[i].primitive =
                Cuboid::new(Aabb::new(Vec3::new(b.min.x, pie, b.min.z), b.max)).into();
            prolongadas.push(Prolongacion {
                objeto: i,
                pie_previo: b.min.y,
                pie,
                apoyo,
            });
        }

        prolongadas
    }

    /// La candidata: la entrega con las caídas alargadas y la cuenca.
    pub fn candidato(actual: Blockout) -> Candidato {
        let mut diorama = actual;
        let objetos_previos = diorama.scene.objects.len();
        let paleta_previa = diorama.scene.palette.len();
        let texturas_previas = diorama.scene.textures.len();

        let prolongadas = prolongar_caidas(&mut diorama.scene);

        // El agua: copia por valor de `A-01`. Techos, `ior` y modo de
        // sombra son los suyos; el brillo y el tinte, los de las caídas.
        let scene = &mut diorama.scene;
        let a01 = *indices(scene, SpatialGroupId::FlyingWaters)
            .last()
            .expect("A-01 va al final de Aguas");
        let base = scene.material(scene.objects[a01].final_material);
        assert!(
            base.ior > 1.0,
            "la ultima pieza de Aguas es el volumen A-01"
        );
        let [r, g, b] = TINTE_DE_LA_CUENCA;
        let agua = scene.add_material(
            base.with_specular(BRILLO.0, BRILLO.1)
                .with_tint(Color::from_srgb(r, g, b)),
        );

        let lienzo = scene.objects[caidas(scene)[0]].initial_material;
        let plinto = indices(scene, SpatialGroupId::Global);
        assert_eq!(plinto.len(), 1, "G-01 es la unica pieza global");
        let piso = scene.objects[plinto[0]].primitive.bounds().max.y;
        let ancla = diorama.anchors.monolith_base_anchor;

        for &([x0, z0, x1, z1], techo) in &POZAS {
            scene.objects.push(SceneObject {
                primitive: Cuboid::new(Aabb::new(
                    ancla + Vec3::new(x0, piso - EMPOTRADO, z0),
                    ancla + Vec3::new(x1, piso + techo, z1),
                ))
                .into(),
                initial_material: lienzo,
                final_material: agua,
                spatial_group: SpatialGroupId::Global,
                reveal_group: RevealGroup::Meadows,
            });
        }

        diorama.accel = reacelerar(&diorama.scene);

        Candidato {
            diorama,
            objetos_previos,
            paleta_previa,
            texturas_previas,
            prolongadas,
            agua: Some(agua),
        }
    }

    // ======================================================== flooded_gaps

    /// Techo común de la lámina, sobre el plinto.
    pub const TECHO_DEL_AGUA: f32 = 0.08;
    /// Holgura en planta del agua a las piezas opacas: separa caras.
    pub const HOLGURA: f32 = 0.03;
    /// Holgura a las piezas refractivas —las caídas—: sin contacto, pero lo
    /// bastante cerca para que desemboquen.
    pub const HOLGURA_CAIDA: f32 = 0.02;
    /// Lo que la lámina deja seco junto al canto del plinto.
    pub const BORDE: f32 = 0.05;
    /// Lo que el lecho se retira del borde del agua.
    pub const RETRANQUEO_DEL_LECHO: f32 = 0.02;

    /// La variante `flooded_gaps`.
    pub struct Inundada {
        pub diorama: Blockout,
        pub objetos_previos: usize,
        pub paleta_previa: usize,
        pub texturas_previas: usize,
        pub prolongadas: Vec<Prolongacion>,
        pub agua: Option<MaterialId>,
        pub lecho: Option<MaterialId>,
        pub celdas: Vec<usize>,
        pub lechos: Vec<usize>,
        pub area_libre: f32,
        pub area_descartada: f32,
    }

    /// Celdas más estrechas o más pequeñas que esto no se inundan: son
    /// astillas entre cortes, no claros.
    pub const ANCHO_MINIMO: f32 = 0.25;
    pub const AREA_MINIMA: f32 = 0.30;
    /// Los trozos de lecho más pequeños se omiten: el agua que queda sin
    /// lecho debajo es poca y sale en los tests.
    pub const AREA_MINIMA_DEL_LECHO: f32 = 0.25;
    /// Lado mínimo de una celda que recibe una caída.
    pub const ANCHO_DE_DESEMBOCADURA: f32 = 0.10;
    /// El lecho, entre estas dos cotas sobre el plinto: abajo dentro de él,
    /// arriba bajo la lámina. Ninguna coincide con el plinto ni con el agua.
    pub const LECHO: (f32, f32) = (-0.02, 0.03);
    /// Color del lecho, en sRGB: un fondo verde azulado muy oscuro, lo que
    /// el rayo refractado ve en vez del lienzo claro del plinto.
    const COLOR_DEL_LECHO: [f32; 3] = [0.08, 0.16, 0.17];

    /// ¿Es `j` una de las caídas de Praderas? Solo a ellas se les deja la
    /// holgura corta: el cristal del Monolito también refracta, pero es un
    /// bloque y lleva la holgura de los bloques.
    pub fn es_caida(scene: &Scene, j: usize) -> bool {
        scene.objects[j].spatial_group == SpatialGroupId::Meadows
            && scene.material(scene.objects[j].final_material).ior > 1.0
    }

    /// Una planta `[x0, z0, x1, z1]`.
    pub type Planta = [f32; 4];

    fn area(p: &Planta) -> f32 {
        (p[2] - p[0]) * (p[3] - p[1])
    }

    /// Cortes ordenados y sin repetir.
    fn cortes(valores: impl Iterator<Item = f32>) -> Vec<f32> {
        let mut v: Vec<f32> = valores.collect();
        v.sort_by(f32::total_cmp);
        v.dedup();
        v
    }

    /// Fusiona las celdas marcadas de una rejilla en rectángulos de
    /// interiores disjuntos, el mayor primero. Determinista: a igual área
    /// gana el primero encontrado.
    fn fusionar(xs: &[f32], zs: &[f32], marcada: &[Vec<bool>]) -> Vec<Planta> {
        let (nx, nz) = (xs.len() - 1, zs.len() - 1);
        let mut libre: Vec<Vec<bool>> = marcada.to_vec();
        let mut rects = Vec::new();

        loop {
            // Celdas libres consecutivas hacia +z desde cada una.
            let mut racha = vec![vec![0usize; nz + 1]; nx];
            for (i, columna) in racha.iter_mut().enumerate() {
                for k in (0..nz).rev() {
                    columna[k] = if libre[i][k] { columna[k + 1] + 1 } else { 0 };
                }
            }

            let mut mejor: Option<(f32, usize, usize, usize, usize)> = None;
            for i in 0..nx {
                for k in 0..nz {
                    let mut alto = usize::MAX;
                    for (j, columna) in racha.iter().enumerate().skip(i) {
                        alto = alto.min(columna[k]);
                        if alto == 0 {
                            break;
                        }
                        let a = (xs[j + 1] - xs[i]) * (zs[k + alto] - zs[k]);
                        if mejor.is_none_or(|m| a > m.0) {
                            mejor = Some((a, i, k, j, k + alto - 1));
                        }
                    }
                }
            }

            let Some((_, i, k, j, q)) = mejor else {
                break;
            };
            for columna in &mut libre[i..=j] {
                for celda in &mut columna[k..=q] {
                    *celda = false;
                }
            }
            rects.push([xs[i], zs[k], xs[j + 1], zs[q + 1]]);
        }

        rects
    }

    /// La base libre: el techo del plinto menos su `BORDE` y menos la planta
    /// de toda pieza que haya sobre él, con su holgura. Devuelve las celdas
    /// que se inundan, el área libre total y el área de astillas que queda
    /// seca.
    pub fn huella_libre(scene: &Scene, plinto: usize) -> (Vec<Planta>, f32, f32) {
        let p = scene.objects[plinto].primitive.bounds();
        let base = [
            p.min.x + BORDE,
            p.min.z + BORDE,
            p.max.x - BORDE,
            p.max.z - BORDE,
        ];

        let todos: Vec<Planta> = (0..scene.objects.len())
            .filter(|&j| j != plinto)
            .map(|j| {
                let b = scene.objects[j].primitive.bounds();
                let h = if es_caida(scene, j) {
                    HOLGURA_CAIDA
                } else {
                    HOLGURA
                };
                [b.min.x - h, b.min.z - h, b.max.x + h, b.max.z + h]
            })
            .collect();

        // Una planta contenida en otra no cambia la huella, pero sus cortes
        // partirían las celdas vecinas. Se descarta; de dos iguales, la
        // segunda.
        let contiene =
            |a: &Planta, b: &Planta| a[0] <= b[0] && a[1] <= b[1] && b[2] <= a[2] && b[3] <= a[3];
        let obstaculos: Vec<Planta> = (0..todos.len())
            .filter(|&i| {
                !(0..todos.len()).any(|j| {
                    j != i && contiene(&todos[j], &todos[i]) && (todos[i] != todos[j] || j < i)
                })
            })
            .map(|i| todos[i])
            .collect();

        let recorte = |v: f32, a: f32, b: f32| v.clamp(a, b);
        let xs = cortes(
            [base[0], base[2]]
                .into_iter()
                .chain(obstaculos.iter().flat_map(|o| [o[0], o[2]]))
                .map(|v| recorte(v, base[0], base[2])),
        );
        let zs = cortes(
            [base[1], base[3]]
                .into_iter()
                .chain(obstaculos.iter().flat_map(|o| [o[1], o[3]]))
                .map(|v| recorte(v, base[1], base[3])),
        );

        let libre: Vec<Vec<bool>> = (0..xs.len() - 1)
            .map(|i| {
                let cx = 0.5 * (xs[i] + xs[i + 1]);
                (0..zs.len() - 1)
                    .map(|k| {
                        let cz = 0.5 * (zs[k] + zs[k + 1]);
                        !obstaculos
                            .iter()
                            .any(|o| o[0] < cx && cx < o[2] && o[1] < cz && cz < o[3])
                    })
                    .collect()
            })
            .collect();

        let mut total = 0.0;
        for (i, columna) in libre.iter().enumerate() {
            for (k, &l) in columna.iter().enumerate() {
                if l {
                    total += (xs[i + 1] - xs[i]) * (zs[k + 1] - zs[k]);
                }
            }
        }

        // Las caídas que ya llegan al plinto: el agua que tienen delante es
        // su desembocadura y se conserva aunque sea estrecha.
        let piso = p.max.y;
        let receptoras: Vec<Aabb> = caidas(scene)
            .into_iter()
            .map(|i| scene.objects[i].primitive.bounds())
            .filter(|b| b.min.y < piso)
            .collect();
        let desemboca = |r: &Planta| {
            receptoras.iter().any(|b| {
                let hueco = r[1] - b.max.z;
                hueco > 0.0
                    && hueco <= HOLGURA_CAIDA + 1.0e-4
                    && r[0] < b.max.x
                    && b.min.x < r[2]
                    && (r[2] - r[0]).min(r[3] - r[1]) >= ANCHO_DE_DESEMBOCADURA
            })
        };

        let (celdas, astillas): (Vec<Planta>, Vec<Planta>) =
            fusionar(&xs, &zs, &libre).into_iter().partition(|r| {
                (r[2] - r[0]).min(r[3] - r[1]) >= ANCHO_MINIMO && area(r) >= AREA_MINIMA
                    || desemboca(r)
            });

        (celdas, total, astillas.iter().map(area).sum())
    }

    /// El lecho: la unión de las celdas, retirada `RETRANQUEO_DEL_LECHO`
    /// hacia dentro, en rectángulos de interiores disjuntos. Así ninguna cara
    /// del lecho coincide con una cara exterior del agua.
    pub fn lechos(celdas: &[Planta]) -> Vec<Planta> {
        let r = RETRANQUEO_DEL_LECHO;
        let gx = cortes(celdas.iter().flat_map(|c| [c[0], c[2]]));
        let gz = cortes(celdas.iter().flat_map(|c| [c[1], c[3]]));
        let union: Vec<Vec<bool>> = (0..gx.len() - 1)
            .map(|i| {
                let cx = 0.5 * (gx[i] + gx[i + 1]);
                (0..gz.len() - 1)
                    .map(|k| {
                        let cz = 0.5 * (gz[k] + gz[k + 1]);
                        celdas
                            .iter()
                            .any(|c| c[0] < cx && cx < c[2] && c[1] < cz && cz < c[3])
                    })
                    .collect()
            })
            .collect();

        // ¿Está `[a0, a1] × [b0, b1]` dentro de la unión?
        let cubierto = |a0: f32, a1: f32, b0: f32, b1: f32| {
            let is: Vec<usize> = (0..gx.len() - 1)
                .filter(|&i| gx[i + 1] > a0 && gx[i] < a1)
                .collect();
            let ks: Vec<usize> = (0..gz.len() - 1)
                .filter(|&k| gz[k + 1] > b0 && gz[k] < b1)
                .collect();
            a0 >= gx[0]
                && a1 <= gx[gx.len() - 1]
                && b0 >= gz[0]
                && b1 <= gz[gz.len() - 1]
                && is.iter().all(|&i| ks.iter().all(|&k| union[i][k]))
        };

        let xs = cortes(celdas.iter().flat_map(|c| [c[0] + r, c[2] - r, c[0], c[2]]));
        let zs = cortes(celdas.iter().flat_map(|c| [c[1] + r, c[3] - r, c[1], c[3]]));
        let marcada: Vec<Vec<bool>> = (0..xs.len() - 1)
            .map(|i| {
                (0..zs.len() - 1)
                    .map(|k| cubierto(xs[i] - r, xs[i + 1] + r, zs[k] - r, zs[k + 1] + r))
                    .collect()
            })
            .collect();

        fusionar(&xs, &zs, &marcada)
            .into_iter()
            .filter(|p| (p[2] - p[0]).min(p[3] - p[1]) >= 0.05 && area(p) >= AREA_MINIMA_DEL_LECHO)
            .collect()
    }

    /// `flooded_gaps`: las caídas de la candidata y una lámina somera, de
    /// altura común, sobre toda la base libre, con su lecho oscuro.
    pub fn flooded_gaps(actual: Blockout) -> Inundada {
        let mut diorama = actual;
        let objetos_previos = diorama.scene.objects.len();
        let paleta_previa = diorama.scene.palette.len();
        let texturas_previas = diorama.scene.textures.len();

        let prolongadas = prolongar_caidas(&mut diorama.scene);

        let scene = &mut diorama.scene;
        let plinto = indices(scene, SpatialGroupId::Global);
        assert_eq!(plinto.len(), 1, "G-01 es la unica pieza global");
        let plinto = plinto[0];
        let (celdas, area_libre, area_descartada) = huella_libre(scene, plinto);
        let fondos = lechos(&celdas);

        let a01 = *indices(scene, SpatialGroupId::FlyingWaters)
            .last()
            .expect("A-01 va al final de Aguas");
        let base = scene.material(scene.objects[a01].final_material);
        assert!(
            base.ior > 1.0,
            "la ultima pieza de Aguas es el volumen A-01"
        );
        let [r, g, b] = TINTE_DE_LA_CUENCA;
        let agua = scene.add_material(
            base.with_specular(BRILLO.0, BRILLO.1)
                .with_tint(Color::from_srgb(r, g, b)),
        );
        let [r, g, b] = COLOR_DEL_LECHO;
        let lecho = scene.add_material(
            expedition33_continente_inacabado::material::Material::new(Color::from_srgb(r, g, b))
                .with_specular(0.05, 8.0),
        );

        let lienzo = scene.objects[caidas(scene)[0]].initial_material;
        let piso = scene.objects[plinto].primitive.bounds().max.y;
        let pieza = |p: &Planta, abajo: f32, arriba: f32, material: MaterialId| SceneObject {
            primitive: Cuboid::new(Aabb::new(
                Vec3::new(p[0], piso + abajo, p[1]),
                Vec3::new(p[2], piso + arriba, p[3]),
            ))
            .into(),
            initial_material: lienzo,
            final_material: material,
            spatial_group: SpatialGroupId::Global,
            reveal_group: RevealGroup::Meadows,
        };

        let primera = scene.objects.len();
        for c in &celdas {
            scene
                .objects
                .push(pieza(c, -EMPOTRADO, TECHO_DEL_AGUA, agua));
        }
        let primer_lecho = scene.objects.len();
        for f in &fondos {
            scene.objects.push(pieza(f, LECHO.0, LECHO.1, lecho));
        }
        let fin = scene.objects.len();

        diorama.accel = reacelerar(&diorama.scene);

        Inundada {
            diorama,
            objetos_previos,
            paleta_previa,
            texturas_previas,
            prolongadas,
            agua: Some(agua),
            lecho: Some(lecho),
            celdas: (primera..primer_lecho).collect(),
            lechos: (primer_lecho..fin).collect(),
            area_libre,
            area_descartada,
        }
    }

    // ======================================================== flooded_blue

    /// Densidad de las texturas del lecho, en texels por unidad de mundo.
    pub const TEXELS_POR_UNIDAD: f32 = 24.0;

    /// La variante `flooded_blue`: la geometría entera de `flooded_gaps` con
    /// otros materiales efectivos en el agua y en el lecho.
    pub struct Azulada {
        pub diorama: Blockout,
        pub celdas: Vec<usize>,
        pub lechos: Vec<usize>,
        /// Paleta y texturas de `flooded_gaps`, que quedan delante intactas.
        pub paleta_previa: usize,
        pub texturas_previas: usize,
        pub agua: MaterialId,
        /// Un material por pieza de lecho, en el orden de `lechos`.
        pub fondos: Vec<MaterialId>,
    }

    /// Tinte del agua de `flooded_blue`, en sRGB: un cian marino claro. Con
    /// el reparto `0.9 / 0.9` pesa un `10 %`; lo que más decide el color es
    /// el fondo que el rayo refractado ve.
    const TINTE_MARINO: [f32; 3] = [0.62, 0.88, 1.00];

    /// Brillo del agua de `flooded_blue`: algo más suave que el de la
    /// candidata, para que el reflejo de la luz principal no la blanquee.
    const BRILLO_MARINO: (f32, f32) = (0.22, 40.0);

    /// Lado de las piedras del fondo, en unidades de mundo.
    const PIEDRA: f32 = 0.42;

    /// Los tres tonos del fondo, en sRGB: la grieta entre piedras, la
    /// piedra oscura y la clara. Azul pizarra, más oscuro que el lienzo seco
    /// pero no negro.
    const GRIETA: [f32; 3] = [0.17, 0.27, 0.37];
    const PIEDRA_OSCURA: [f32; 3] = [0.28, 0.42, 0.55];
    const PIEDRA_CLARA: [f32; 3] = [0.40, 0.57, 0.69];

    fn azar(x: i32, z: i32, semilla: u32) -> f32 {
        let mut h =
            semilla ^ (x as u32).wrapping_mul(0x27D4_EB2D) ^ (z as u32).wrapping_mul(0x1656_67B1);
        h ^= h >> 15;
        h = h.wrapping_mul(0x2C1B_3C6D);
        h ^= h >> 12;
        h = h.wrapping_mul(0x297A_2D39);
        h ^= h >> 15;
        (h >> 8) as f32 / (1u32 << 24) as f32
    }

    /// Ruido de valor suave en el plano, de periodo `escala`.
    fn ruido_de_valor(x: f32, z: f32, escala: f32, semilla: u32) -> f32 {
        let (u, v) = (x / escala, z / escala);
        let (x0, z0) = (u.floor(), v.floor());
        let suave = |t: f32| t * t * (3.0 - 2.0 * t);
        let (sx, sz) = (suave(u - x0), suave(v - z0));
        let en = |dx: i32, dz: i32| azar(x0 as i32 + dx, z0 as i32 + dz, semilla);
        let a = en(0, 0) + (en(1, 0) - en(0, 0)) * sx;
        let b = en(0, 1) + (en(1, 1) - en(0, 1)) * sx;
        a + (b - a) * sz
    }

    fn mezclar(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
        let t = t.clamp(0.0, 1.0);
        [
            a[0] + (b[0] - a[0]) * t,
            a[1] + (b[1] - a[1]) * t,
            a[2] + (b[2] - a[2]) * t,
        ]
    }

    /// El fondo rocoso, **en coordenadas de mundo**: piedras de unos
    /// `PIEDRA` de lado —celdas de Voronoi con un tono propio cada una—,
    /// grietas oscuras entre ellas y manchas amplias de luz y sombra. Lo
    /// muestrea cada pieza de lecho en su planta, así que el fondo sigue de
    /// una pieza a la vecina sin costura ni estiramiento.
    pub fn campo_de_roca(x: f32, z: f32) -> Color {
        let (u, v) = (x / PIEDRA, z / PIEDRA);
        let (cu, cv) = (u.floor() as i32, v.floor() as i32);
        let (mut d1, mut d2, mut tono) = (f32::MAX, f32::MAX, 0.0);
        for dz in -1..=1 {
            for dx in -1..=1 {
                let (gx, gz) = (cu + dx, cv + dz);
                let px = gx as f32 + 0.15 + 0.7 * azar(gx, gz, 0xB10E_0001);
                let pz = gz as f32 + 0.15 + 0.7 * azar(gx, gz, 0xB10E_0002);
                let d = ((u - px).powi(2) + (v - pz).powi(2)).sqrt();
                if d < d1 {
                    d2 = d1;
                    d1 = d;
                    tono = azar(gx, gz, 0xB10E_0003);
                } else if d < d2 {
                    d2 = d;
                }
            }
        }

        let mancha = 0.6 * ruido_de_valor(x, z, 1.7, 0xB10E_0004)
            + 0.4 * ruido_de_valor(x, z, 0.55, 0xB10E_0005);
        let piedra = mezclar(
            PIEDRA_OSCURA,
            PIEDRA_CLARA,
            0.15 + 0.5 * tono + 0.35 * mancha,
        );
        // La grieta: donde las dos piedras más cercanas están casi a la
        // misma distancia. Su fuerza varía con otra mancha amplia, para que
        // unas se marquen y otras se pierdan y el fondo no lea como un
        // empedrado regular.
        let borde = ((d2 - d1) / 0.09).clamp(0.0, 1.0);
        let fuerza = 0.35 + 0.65 * ruido_de_valor(x, z, 1.1, 0xB10E_0006);
        let grieta = (1.0 - borde * borde * (3.0 - 2.0 * borde)) * fuerza;
        let [r, g, b] = mezclar(piedra, GRIETA, grieta);
        Color::from_srgb(r, g, b)
    }

    /// La textura de una pieza de lecho: `campo_de_roca` en el centro de cada
    /// texel de su cara superior, a `TEXELS_POR_UNIDAD`.
    ///
    /// La cara superior de un cuboide recorre `x` en `u` y `z` en `v`, y la
    /// fila `0` de la textura es `v = 1`. Ver `Cuboid::uv_en_cara` y
    /// `Texture::sample`.
    fn textura_de_fondo(b: &Aabb) -> Texture {
        let (w, h) = (b.max.x - b.min.x, b.max.z - b.min.z);
        let ancho = ((w * TEXELS_POR_UNIDAD).round() as usize).max(2);
        let alto = ((h * TEXELS_POR_UNIDAD).round() as usize).max(2);
        let mut pixeles = Vec::with_capacity(ancho * alto);
        for fila in 0..alto {
            let v = 1.0 - (fila as f32 + 0.5) / alto as f32;
            for col in 0..ancho {
                let u = (col as f32 + 0.5) / ancho as f32;
                pixeles.push(campo_de_roca(b.min.x + u * w, b.min.z + v * h));
            }
        }
        Texture::from_pixels(ancho, alto, pixeles).expect("dimensiones validas")
    }

    /// `flooded_blue`: la geometría entera de `flooded_gaps`, sin una
    /// primitiva más ni menos, con agua marina clara y un fondo de piedra
    /// azul pizarra.
    ///
    /// Solo cambia el material final de las celdas y de las piezas de lecho.
    /// La paleta y las texturas de `flooded_gaps` quedan delante intactas; la
    /// jerarquía no se reconstruye porque no se ha movido nada.
    pub fn flooded_blue(actual: Blockout) -> Azulada {
        let f = flooded_gaps(actual);
        let mut diorama = f.diorama;
        let scene = &mut diorama.scene;
        let paleta_previa = scene.palette.len();
        let texturas_previas = scene.textures.len();

        let previa = scene.material(f.agua.expect("flooded_gaps lleva agua"));
        let [r, g, b] = TINTE_MARINO;
        let agua = scene.add_material(
            previa
                .with_specular(BRILLO_MARINO.0, BRILLO_MARINO.1)
                .with_tint(Color::from_srgb(r, g, b)),
        );

        let fondos: Vec<MaterialId> = f
            .lechos
            .iter()
            .map(|&i| {
                let tex = scene.add_texture(textura_de_fondo(&scene.objects[i].primitive.bounds()));
                scene.add_material(
                    expedition33_continente_inacabado::material::Material::new(Color::new(
                        1.0, 1.0, 1.0,
                    ))
                    .with_texture(tex)
                    .with_uv_scale(1.0)
                    .with_specular(0.06, 10.0),
                )
            })
            .collect();

        for &i in &f.celdas {
            scene.objects[i].final_material = agua;
        }
        for (&i, &m) in f.lechos.iter().zip(&fondos) {
            scene.objects[i].final_material = m;
        }

        Azulada {
            diorama,
            celdas: f.celdas,
            lechos: f.lechos,
            paleta_previa,
            texturas_previas,
            agua,
            fondos,
        }
    }

    // ================================================== flooded_reflective

    /// La variante `flooded_reflective`: `flooded_blue` entera con otro
    /// material efectivo en las 19 celdas de agua.
    pub struct Reflectante {
        pub diorama: Blockout,
        pub celdas: Vec<usize>,
        pub lechos: Vec<usize>,
        /// Paleta y texturas de `flooded_blue`, que quedan delante intactas.
        pub paleta_previa: usize,
        pub texturas_previas: usize,
        /// El agua de `flooded_blue` y la nueva.
        pub agua_previa: MaterialId,
        pub agua: MaterialId,
    }

    /// Brillo especular directo del agua de `flooded_reflective`:
    /// `(fuerza, exponente)`.
    ///
    /// El reflejo **trazado** no se toca: pesa `reflection_cap × F`, y con los
    /// techos y el `ior` aprobados un plano horizontal mirado desde la hero o
    /// la cenital refleja el `2–4 %`, igual que la superficie de `A-01`. Lo
    /// que sí se puede ajustar sin cambiar la óptica es el brillo de las
    /// luces sobre la lámina, que el renderer suma después del reparto de
    /// Fresnel y sin lanzar rayos: un satinado, no un espejo.
    ///
    /// Barrido en el preview, luma media del agua en hero / top78:
    /// `0.22 / 40` (el de `flooded_blue`) da `73 / 75`; este, `79 / 82`, con
    /// el p95 de la cenital de `131` a `154` y ningún píxel quemado. Con `12`
    /// se blanquea la lámina entera (`86 / 92`); con `128` solo aparecen
    /// destellos en la cenital y en la hero no cambia nada.
    const BRILLO_REFLECTANTE: (f32, f32) = (0.50, 24.0);

    /// `flooded_reflective`: `flooded_blue` entera, con el agua de sus 19
    /// celdas más satinada. Mismo tinte, textura, techos, `ior` y sombra.
    pub fn flooded_reflective(actual: Blockout) -> Reflectante {
        let a = flooded_blue(actual);
        let mut diorama = a.diorama;
        let scene = &mut diorama.scene;
        let paleta_previa = scene.palette.len();
        let texturas_previas = scene.textures.len();
        let agua = scene.add_material(
            scene
                .material(a.agua)
                .with_specular(BRILLO_REFLECTANTE.0, BRILLO_REFLECTANTE.1),
        );
        for &i in &a.celdas {
            scene.objects[i].final_material = agua;
        }

        Reflectante {
            diorama,
            celdas: a.celdas,
            lechos: a.lechos,
            paleta_previa,
            texturas_previas,
            agua_previa: a.agua,
            agua,
        }
    }

    // ================================================== flooded_blue_raised

    /// La variante `flooded_blue_raised`: `flooded_blue` entera con el techo
    /// común de sus 19 celdas de agua más alto.
    pub struct Elevada {
        pub diorama: Blockout,
        pub celdas: Vec<usize>,
        pub lechos: Vec<usize>,
        /// Techo común de la lámina sobre el plinto, antes y ahora.
        pub nivel_previo: f32,
        pub nivel: f32,
    }

    /// Techo común de la lámina de `flooded_blue_raised`, sobre el plinto.
    ///
    /// Medido en 3D antes de elegirlo, en las dos rutas: ninguna celda corta
    /// nada a ningún nivel —la huella excluyó en planta toda pieza a
    /// cualquier altura—, así que lo que limita es otra cosa:
    ///
    /// - **la emergencia**: la pieza vecina más baja es el lecho de Aguas
    ///   `A-02`, techo `0.65`, junto a la celda del este del Monolito. Con
    ///   `0.30` asoma `0.35`;
    /// - **el renderer**: sin pila de medios, el rayo refractado que llega a
    ///   la cara que comparten dos celdas «sale» al aire. Con más agua sobre
    ///   el lecho llegan más: en el encuadre de detalle, `0.34 %` a `0.08`,
    ///   `1.59 %` a `0.30`, `1.96 %` a `0.35`, `2.45 %` a `0.40` y `3.42 %` a
    ///   `0.50`. `0.30` queda por debajo del `2 %` en las tres tomas con
    ///   margen.
    ///
    /// La masa de fondo 4 flota a `0.10` del plinto: con este nivel, el agua
    /// sube junto a su hueco inferior —a `0.03`, sin tocarla—.
    pub const NIVEL_ELEVADO: f32 = 0.30;

    /// `flooded_blue_raised`: `flooded_blue` entera con el techo común de sus
    /// 19 celdas en `NIVEL_ELEVADO`. El fondo de cada celda, su planta, el
    /// lecho, los materiales y todo lo demás no se tocan.
    pub fn flooded_blue_raised(actual: Blockout) -> Elevada {
        let a = flooded_blue(actual);
        let mut diorama = a.diorama;
        let scene = &mut diorama.scene;
        let piso = scene.objects[indices(scene, SpatialGroupId::Global)[0]]
            .primitive
            .bounds()
            .max
            .y;
        for &i in &a.celdas {
            let b = scene.objects[i].primitive.bounds();
            scene.objects[i].primitive = Cuboid::new(Aabb::new(
                b.min,
                Vec3::new(b.max.x, piso + NIVEL_ELEVADO, b.max.z),
            ))
            .into();
        }
        diorama.accel = reacelerar(&diorama.scene);

        Elevada {
            diorama,
            celdas: a.celdas,
            lechos: a.lechos,
            nivel_previo: TECHO_DEL_AGUA,
            nivel: NIVEL_ELEVADO,
        }
    }

    // ================================================== under_meadows_fill

    /// Hueco vertical mínimo, sobre el agua, que deja una pieza elevada de
    /// Praderas para que la cuenca pueda pasar por debajo.
    pub const HUECO_BAJO_PRADERAS: f32 = 0.50;

    /// Lo que la región bajo Praderas se extiende más allá de la planta de sus
    /// piezas: alcanza a las celdas vecinas sin dejar costura seca.
    const ALCANCE: f32 = 0.10;

    /// ¿Es `j` una pieza de Praderas que vuela por encima de la cuenca, con
    /// sitio para el agua y el lecho debajo?
    fn elevada_de_praderas(scene: &Scene, j: usize, piso: f32) -> bool {
        scene.objects[j].spatial_group == SpatialGroupId::Meadows
            && scene.objects[j].primitive.bounds().min.y
                >= piso + NIVEL_ELEVADO + HUECO_BAJO_PRADERAS
    }

    /// Las plantas del relleno bajo Praderas: `(agua, lecho)`.
    ///
    /// La base libre de `aplicar` excluye en planta toda pieza a cualquier
    /// altura, y la meseta de Praderas vuela a `4.2`: bajo ella quedaba lienzo
    /// seco. Aquí se mide la base libre **solo bajo las piezas elevadas de
    /// Praderas**, dejando de excluirlas, pero no a nada más: las masas de fondo
    /// que flotan bajo la meseta, las caídas que llegan al suelo y las celdas y
    /// lechos ya puestos siguen fuera. Las celdas ya puestas no se tocan; las
    /// nuevas las tocan por su borde.
    fn plantas_bajo_praderas(
        scene: &Scene,
        plinto: usize,
        previas: &[usize],
        franja: (f32, f32),
    ) -> (Vec<Planta>, Vec<Planta>) {
        let p = scene.objects[plinto].primitive.bounds();
        let piso = p.max.y;
        let base = [
            p.min.x + BORDE,
            p.min.z + BORDE,
            p.max.x - BORDE,
            p.max.z - BORDE,
        ];
        let planta = |j: usize| {
            let b = scene.objects[j].primitive.bounds();
            [b.min.x, b.min.z, b.max.x, b.max.z]
        };
        let agua_previa: Vec<Planta> = previas
            .iter()
            .filter(|&&j| scene.objects[j].primitive.bounds().max.y > piso + LECHO.1 + EPS)
            .map(|&j| planta(j))
            .collect();
        let lecho_previo: Vec<Planta> = previas
            .iter()
            .filter(|&&j| scene.objects[j].primitive.bounds().max.y <= piso + LECHO.1 + EPS)
            .map(|&j| planta(j))
            .collect();

        let bajo: Vec<Planta> = (0..scene.objects.len())
            .filter(|&j| j != plinto && elevada_de_praderas(scene, j, piso))
            .map(planta)
            .map(|[x0, z0, x1, z1]| [x0 - ALCANCE, z0 - ALCANCE, x1 + ALCANCE, z1 + ALCANCE])
            .filter(|r| r[3] > franja.0 && r[1] < franja.1)
            .map(|[x0, z0, x1, z1]| [x0, z0.max(franja.0), x1, z1.min(franja.1)])
            .collect();
        let mut obstaculos: Vec<Planta> = (0..scene.objects.len())
            .filter(|&j| {
                j != plinto && !previas.contains(&j) && !elevada_de_praderas(scene, j, piso)
            })
            .map(|j| {
                let h = if es_caida(scene, j) {
                    HOLGURA_CAIDA
                } else {
                    HOLGURA
                };
                let [x0, z0, x1, z1] = planta(j);
                [x0 - h, z0 - h, x1 + h, z1 + h]
            })
            .collect();
        obstaculos.extend(agua_previa.iter().copied());

        let recorte = |v: f32, a: f32, b: f32| v.clamp(a, b);
        let xs = cortes(
            [base[0], base[2]]
                .into_iter()
                .chain(obstaculos.iter().chain(&bajo).flat_map(|o| [o[0], o[2]]))
                .map(|v| recorte(v, base[0], base[2])),
        );
        let zs = cortes(
            [base[1], base[3]]
                .into_iter()
                .chain(obstaculos.iter().chain(&bajo).flat_map(|o| [o[1], o[3]]))
                .map(|v| recorte(v, base[1], base[3])),
        );
        let dentro = |r: &Planta, x: f32, z: f32| r[0] < x && x < r[2] && r[1] < z && z < r[3];
        let libre: Vec<Vec<bool>> = (0..xs.len() - 1)
            .map(|i| {
                let cx = 0.5 * (xs[i] + xs[i + 1]);
                (0..zs.len() - 1)
                    .map(|k| {
                        let cz = 0.5 * (zs[k] + zs[k + 1]);
                        bajo.iter().any(|r| dentro(r, cx, cz))
                            && !obstaculos.iter().any(|o| dentro(o, cx, cz))
                    })
                    .collect()
            })
            .collect();
        let agua: Vec<Planta> = fusionar(&xs, &zs, &libre)
            .into_iter()
            .filter(|r| (r[2] - r[0]).min(r[3] - r[1]) >= ANCHO_MINIMO && area(r) >= AREA_MINIMA)
            .collect();

        // El lecho nuevo: la unión de **todas** las celdas retirada
        // `RETRANQUEO_DEL_LECHO`, sin pisar el lecho previo y solo junto a las
        // celdas nuevas. Así cubre también el reborde que el lecho previo dejó
        // en la frontera con ellas y no queda costura sin fondo.
        let r = RETRANQUEO_DEL_LECHO;
        let todas: Vec<Planta> = agua_previa.iter().chain(&agua).copied().collect();
        let cubierto = |a0: f32, a1: f32, b0: f32, b1: f32| {
            // Muestreo exacto sobre la rejilla de las celdas: el cuadrado entero
            // dentro de la unión.
            let gx = cortes(todas.iter().flat_map(|c| [c[0], c[2]]));
            let gz = cortes(todas.iter().flat_map(|c| [c[1], c[3]]));
            (0..gx.len() - 1)
                .filter(|&i| gx[i + 1] > a0 && gx[i] < a1)
                .all(|i| {
                    (0..gz.len() - 1)
                        .filter(|&k| gz[k + 1] > b0 && gz[k] < b1)
                        .all(|k| {
                            let (cx, cz) = (0.5 * (gx[i] + gx[i + 1]), 0.5 * (gz[k] + gz[k + 1]));
                            todas.iter().any(|c| dentro(c, cx, cz))
                        })
                })
                && a0 >= base[0]
                && a1 <= base[2]
                && b0 >= base[1]
                && b1 <= base[3]
        };
        let cerca: Vec<Planta> = agua
            .iter()
            .map(|c| [c[0] - r, c[1] - r, c[2] + r, c[3] + r])
            .collect();
        let xs = cortes(
            todas
                .iter()
                .flat_map(|c| [c[0] + r, c[2] - r, c[0], c[2]])
                .chain(lecho_previo.iter().flat_map(|l| [l[0], l[2]]))
                .chain(cerca.iter().flat_map(|c| [c[0], c[2]])),
        );
        let zs = cortes(
            todas
                .iter()
                .flat_map(|c| [c[1] + r, c[3] - r, c[1], c[3]])
                .chain(lecho_previo.iter().flat_map(|l| [l[1], l[3]]))
                .chain(cerca.iter().flat_map(|c| [c[1], c[3]])),
        );
        let marcada: Vec<Vec<bool>> = (0..xs.len() - 1)
            .map(|i| {
                let cx = 0.5 * (xs[i] + xs[i + 1]);
                (0..zs.len() - 1)
                    .map(|k| {
                        let cz = 0.5 * (zs[k] + zs[k + 1]);
                        cerca.iter().any(|c| dentro(c, cx, cz))
                            && !lecho_previo.iter().any(|l| dentro(l, cx, cz))
                            && cubierto(xs[i] - r, xs[i + 1] + r, zs[k] - r, zs[k + 1] + r)
                    })
                    .collect()
            })
            .collect();
        let lecho: Vec<Planta> = fusionar(&xs, &zs, &marcada)
            .into_iter()
            .filter(|p| (p[2] - p[0]).min(p[3] - p[1]) >= 0.01 && area(p) >= 0.005)
            .collect();

        (agua, lecho)
    }

    /// Rellena la cuenca bajo la meseta de Praderas, delante de las masas que
    /// flotan bajo ella: la franja que `aplicar` dejaba seca.
    ///
    /// Se llama después de `aplicar` y, como ella, con `A-01` todavía dentro.
    /// No toca ninguna pieza existente: añade al final `RELLENO_AGUA` celdas con
    /// el **mismo** material de agua de la cuenca —mismo nivel, color, brillo y
    /// óptica, también por preset— y `RELLENO_LECHO` piezas de lecho, cada una
    /// con su fondo muestreado del mismo campo de piedra en coordenadas de mundo.
    fn rellenar_bajo_praderas(scene: &mut Scene) {
        rellenar_tramo(scene, Tramo::Delante);
    }

    /// La franja detrás de las masas que flotan bajo la meseta: la que Charlie
    /// vio seca desde una órbita trasera.
    fn rellenar_tras_las_masas(scene: &mut Scene) {
        rellenar_tramo(scene, Tramo::Detras);
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Tramo {
        Delante,
        Detras,
    }

    fn rellenar_tramo(scene: &mut Scene, tramo: Tramo) {
        let plinto = (0..scene.objects.len())
            .find(|&i| scene.objects[i].spatial_group == SpatialGroupId::Global)
            .expect("G-01 es la primera pieza global");
        let de_la_cuenca = |i: usize| {
            scene.objects[i].spatial_group == SpatialGroupId::Global
                && scene.objects[i].reveal_group == RevealGroup::Meadows
        };
        let previas: Vec<usize> = (0..scene.objects.len())
            .filter(|&i| de_la_cuenca(i))
            .collect();
        let piso = scene.objects[plinto].primitive.bounds().max.y;
        let una_celda = *previas
            .iter()
            .find(|&&i| scene.objects[i].primitive.bounds().max.y > piso + LECHO.1 + EPS)
            .expect("la cuenca se aplica antes que su relleno");
        let agua = scene.objects[una_celda].final_material;
        let lienzo = scene.objects[una_celda].initial_material;

        // El frente: delante del fondo de las masas que flotan con su centro bajo
        // la meseta de Praderas.
        let bajo: Vec<Aabb> = (0..scene.objects.len())
            .filter(|&j| j != plinto && elevada_de_praderas(scene, j, piso))
            .map(|j| scene.objects[j].primitive.bounds())
            .collect();
        let frente = (0..scene.objects.len())
            .filter(|&j| {
                let o = &scene.objects[j];
                let b = o.primitive.bounds();
                let (cx, cz) = (0.5 * (b.min.x + b.max.x), 0.5 * (b.min.z + b.max.z));
                o.spatial_group != SpatialGroupId::Meadows
                    && !de_la_cuenca(j)
                    && j != plinto
                    && b.min.y > piso + EPS
                    && bajo
                        .iter()
                        .any(|m| m.min.x < cx && cx < m.max.x && m.min.z < cz && cz < m.max.z)
            })
            .map(|j| scene.objects[j].primitive.bounds().min.z)
            .max_by(f32::total_cmp)
            .expect("bajo la meseta flotan las masas de fondo");

        let franja = match tramo {
            Tramo::Delante => (frente, f32::INFINITY),
            Tramo::Detras => (f32::NEG_INFINITY, frente),
        };
        let (celdas, fondos) = plantas_bajo_praderas(scene, plinto, &previas, franja);
        let pieza = |p: &Planta, abajo: f32, arriba: f32, material: MaterialId| SceneObject {
            primitive: Cuboid::new(Aabb::new(
                Vec3::new(p[0], piso + abajo, p[1]),
                Vec3::new(p[2], piso + arriba, p[3]),
            ))
            .into(),
            initial_material: lienzo,
            final_material: material,
            spatial_group: SpatialGroupId::Global,
            reveal_group: RevealGroup::Meadows,
        };
        for c in &celdas {
            scene
                .objects
                .push(pieza(c, -EMPOTRADO, NIVEL_ELEVADO, agua));
        }
        for f in &fondos {
            let caja = Aabb::new(
                Vec3::new(f[0], piso + LECHO.0, f[1]),
                Vec3::new(f[2], piso + LECHO.1, f[3]),
            );
            let tex = scene.add_texture(textura_de_fondo(&caja));
            let fondo = scene.add_material(
                expedition33_continente_inacabado::material::Material::new(Color::new(
                    1.0, 1.0, 1.0,
                ))
                .with_texture(tex)
                .with_uv_scale(1.0)
                .with_specular(0.06, 10.0),
            );
            scene.objects.push(pieza(f, LECHO.0, LECHO.1, fondo));
        }
    }

    /// La variante `under_meadows_fill`: `flooded_blue_raised` entera, con
    /// la franja bajo la meseta de Praderas llena —el hueco que Charlie vio
    /// seco—. Solo añade piezas detrás de las suyas.
    pub struct Rellena {
        pub diorama: Blockout,
        pub objetos_previos: usize,
        pub paleta_previa: usize,
        pub texturas_previas: usize,
    }

    pub fn under_meadows_fill(actual: Blockout) -> Rellena {
        let e = flooded_blue_raised(actual);
        let mut diorama = e.diorama;
        let objetos_previos = diorama.scene.objects.len();
        let paleta_previa = diorama.scene.palette.len();
        let texturas_previas = diorama.scene.textures.len();
        rellenar_bajo_praderas(&mut diorama.scene);
        diorama.accel = reacelerar(&diorama.scene);
        Rellena {
            diorama,
            objetos_previos,
            paleta_previa,
            texturas_previas,
        }
    }

    /// La variante `behind_masses_fill`: `under_meadows_fill` entera, con la
    /// franja de detrás de las masas llena. Solo añade piezas.
    pub fn behind_masses_fill(actual: Blockout) -> Rellena {
        let r = under_meadows_fill(actual);
        let mut diorama = r.diorama;
        let objetos_previos = diorama.scene.objects.len();
        let paleta_previa = diorama.scene.palette.len();
        let texturas_previas = diorama.scene.textures.len();
        rellenar_tras_las_masas(&mut diorama.scene);
        diorama.accel = reacelerar(&diorama.scene);
        Rellena {
            diorama,
            objetos_previos,
            paleta_previa,
            texturas_previas,
        }
    }

    /// La órbita trasera desde la que se vio la franja seca: la hero girada
    /// `180°`, a `35°`. Es una cámara que la ventana alcanza con las flechas.
    pub fn camara_trasera(d: &Blockout) -> Camera {
        d.camera_at(HERO_YAW_DEGREES + 180.0, 35.0)
    }

    /// Encuadre cercano del hueco bajo la meseta de Praderas, desde delante y
    /// abajo, a la altura del agua.
    pub fn camara_bajo_praderas() -> Camera {
        let mira = Vec3::new(-0.6, 0.3, -3.8);
        Camera::new(
            Vec3::new(1.2, 2.4, 3.2),
            mira,
            mira,
            Vec3::new(0.0, 1.0, 0.0),
            55f32.to_radians(),
        )
    }

    pub fn reacelerar(scene: &Scene) -> SceneAccel {
        let mut plan = ClusterPlan::new();
        for (k, i) in indices(scene, SpatialGroupId::Breakwater)
            .into_iter()
            .enumerate()
        {
            plan.asignar(i, u16::from(k >= ROMPEOLAS_ORIGINAL));
        }
        SceneAccel::build_from_plan(scene, &plan).expect("la escena tiene geometria")
    }

    pub fn camara_cenital(d: &Blockout) -> Camera {
        d.camera_at(HERO_YAW_DEGREES, ELEVACION_CENITAL)
    }

    /// Encuadre cercano del pie del Monolito, alto y desde delante, por
    /// encima del Rompeolas y de la bahía.
    pub fn camara_del_monolito() -> Camera {
        let mira = Vec3::new(0.0, 0.3, -0.6);
        Camera::new(
            Vec3::new(1.5, 11.0, 7.5),
            mira,
            mira,
            Vec3::new(0.0, 1.0, 0.0),
            45f32.to_radians(),
        )
    }

    pub fn correr() -> ExitCode {
        let Some(destino) = std::env::args().nth(1).map(PathBuf::from) else {
            eprintln!("uso: cargo run --release --example monolith_basin_preview -- <carpeta>");
            eprintln!("  la carpeta es obligatoria: este ejemplo no escribe evidencia oficial.");
            return ExitCode::FAILURE;
        };
        if let Err(e) = std::fs::create_dir_all(&destino) {
            eprintln!("error: no se pudo crear {}: {e}", destino.display());
            return ExitCode::FAILURE;
        }

        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let vigente = actual(&raiz);
        let propuesta = candidato(actual(&raiz));
        let inundada = flooded_gaps(actual(&raiz));
        let azulada = flooded_blue(actual(&raiz));
        let reflectante = flooded_reflective(actual(&raiz));
        let elevada = flooded_blue_raised(actual(&raiz));
        // `delivery` es la cuenca aprobada de 205, la entrega anterior al
        // relleno bajo Praderas: sus PNG no cambian. `delivery_fill` es la
        // entrega de producción de ahora.
        let entregada =
            delivery_level_previo_relleno_con(WaterPreset::RefractiveWater, Some(&raiz))
                .expect("los assets reales tienen que estar en la raiz del proyecto");
        let rellena = under_meadows_fill(actual(&raiz));
        // `delivery_fill` es la entrega con el relleno delantero, de 214, la
        // anterior al relleno trasero: sus PNG no cambian. `delivery_behind`
        // es la entrega de producción de ahora.
        let entregada_con_relleno =
            delivery_level_previo_trasero_con(WaterPreset::RefractiveWater, Some(&raiz))
                .expect("los assets reales tienen que estar en la raiz del proyecto");
        let detras = behind_masses_fill(actual(&raiz));
        // Desde el Monolito de carbón, `delivery_level_con` añade su placa:
        // la entrega con los dos rellenos es la línea base de 218.
        let entregada_detras =
            delivery_level_previo_monolito_con(WaterPreset::RefractiveWater, Some(&raiz))
                .expect("los assets reales tienen que estar en la raiz del proyecto");
        let camaras = [
            ("hero", vigente.hero_camera()),
            ("top78", camara_cenital(&vigente)),
            ("monolith", camara_del_monolito()),
        ];
        let luces = luces_del_diorama(&vigente.anchors, &vigente.scale);

        println!("monolith_basin_preview\n");
        println!("  destino     {}", destino.display());
        println!(
            "  primitivas  current {} · candidate {} (+{} pozas, experimental)",
            vigente.scene.objects.len(),
            propuesta.diorama.scene.objects.len(),
            propuesta.diorama.scene.objects.len() - propuesta.objetos_previos
        );
        println!(
            "  materiales  +{} (agua {:?}) · texturas +{}",
            propuesta.diorama.scene.palette.len() - propuesta.paleta_previa,
            propuesta.agua,
            propuesta.diorama.scene.textures.len() - propuesta.texturas_previas
        );
        for p in &propuesta.prolongadas {
            println!(
                "  caida {:3}   pie {:.3} -> {:.3} sobre el objeto {}",
                p.objeto, p.pie_previo, p.pie, p.apoyo
            );
        }
        println!(
            "  flooded_gaps {} = {} + {} agua + {} lecho (experimental) · materiales +{} · texturas +{}",
            inundada.diorama.scene.objects.len(),
            inundada.objetos_previos,
            inundada.celdas.len(),
            inundada.lechos.len(),
            inundada.diorama.scene.palette.len() - inundada.paleta_previa,
            inundada.diorama.scene.textures.len() - inundada.texturas_previas
        );
        println!(
            "  base libre   {:.2} u2 · astillas secas {:.2} u2 · agua {:?} · lecho {:?} · {} caidas alargadas",
            inundada.area_libre,
            inundada.area_descartada,
            inundada.agua,
            inundada.lecho,
            inundada.prolongadas.len()
        );

        println!(
            "  flooded_blue {} (la geometria de flooded_gaps) · agua {:?} · {} fondos {:?}..{:?} · materiales +{} · texturas +{} a {} texels por unidad",
            azulada.diorama.scene.objects.len(),
            azulada.agua,
            azulada.fondos.len(),
            azulada.fondos.first(),
            azulada.fondos.last(),
            azulada.diorama.scene.palette.len() - azulada.paleta_previa,
            azulada.diorama.scene.textures.len() - azulada.texturas_previas,
            TEXELS_POR_UNIDAD
        );
        println!(
            "  flooded_blue celdas {:?} · lechos {:?}",
            azulada.celdas, azulada.lechos
        );
        println!(
            "  flooded_reflective {} · agua {:?} (antes {:?}), brillo {:?} · {} celdas, {} lechos · materiales +{} · texturas +{}",
            reflectante.diorama.scene.objects.len(),
            reflectante.agua,
            reflectante.agua_previa,
            BRILLO_REFLECTANTE,
            reflectante.celdas.len(),
            reflectante.lechos.len(),
            reflectante.diorama.scene.palette.len() - reflectante.paleta_previa,
            reflectante.diorama.scene.textures.len() - reflectante.texturas_previas
        );

        println!(
            "  flooded_blue_raised {} · {} celdas, {} lechos · nivel {:.2} -> {:.2} sobre el plinto (agua sobre el lecho {:.2} -> {:.2})",
            elevada.diorama.scene.objects.len(),
            elevada.celdas.len(),
            elevada.lechos.len(),
            elevada.nivel_previo,
            elevada.nivel,
            elevada.nivel_previo - LECHO.1,
            elevada.nivel - LECHO.1
        );

        let mut ok = true;
        for (toma, camara) in &camaras {
            for (nombre, d) in [
                ("current", &vigente),
                ("candidate", &propuesta.diorama),
                ("flooded_gaps", &inundada.diorama),
                ("flooded_blue", &azulada.diorama),
                ("flooded_reflective", &reflectante.diorama),
                ("flooded_blue_raised", &elevada.diorama),
                ("delivery", &entregada),
                ("under_meadows_fill", &rellena.diorama),
                ("delivery_fill", &entregada_con_relleno),
                ("behind_masses_fill", &detras.diorama),
                ("delivery_behind", &entregada_detras),
            ] {
                let mut fb = Framebuffer::new(ANCHO, ALTO);
                let stats = render(
                    &mut fb,
                    &d.scene,
                    &d.accel,
                    &luces,
                    &RevealState::painted(),
                    camara,
                    Shading::Material,
                );
                let ruta = destino.join(format!("{nombre}_{toma}.png"));
                if let Err(e) = fb.save_png(&ruta) {
                    eprintln!("error: no se pudo escribir {}: {e}", ruta.display());
                    ok = false;
                }
                println!(
                    "  {toma:<9} {nombre:<10} reflejados {} refractados {}",
                    stats.reflection_rays, stats.refraction_rays
                );
            }
        }

        // El encuadre cercano del hueco, solo para la cuenca de 205 y el
        // relleno.
        let bajo = camara_bajo_praderas();
        for (nombre, d) in [
            ("delivery", &entregada),
            ("under_meadows_fill", &rellena.diorama),
            ("delivery_fill", &entregada_con_relleno),
            ("behind_masses_fill", &detras.diorama),
            ("delivery_behind", &entregada_detras),
        ] {
            let mut fb = Framebuffer::new(ANCHO, ALTO);
            render(
                &mut fb,
                &d.scene,
                &d.accel,
                &luces,
                &RevealState::painted(),
                &bajo,
                Shading::Material,
            );
            let ruta = destino.join(format!("{nombre}_under_meadows.png"));
            if let Err(e) = fb.save_png(&ruta) {
                eprintln!("error: no se pudo escribir {}: {e}", ruta.display());
                ok = false;
            }
        }
        // La órbita trasera de la captura, para el relleno delantero y el
        // trasero.
        let trasera = camara_trasera(&vigente);
        for (nombre, d) in [
            ("delivery_fill", &entregada_con_relleno),
            ("behind_masses_fill", &detras.diorama),
            ("delivery_behind", &entregada_detras),
        ] {
            let mut fb = Framebuffer::new(ANCHO, ALTO);
            render(
                &mut fb,
                &d.scene,
                &d.accel,
                &luces,
                &RevealState::painted(),
                &trasera,
                Shading::Material,
            );
            let ruta = destino.join(format!("{nombre}_back.png"));
            if let Err(e) = fb.save_png(&ruta) {
                eprintln!("error: no se pudo escribir {}: {e}", ruta.display());
                ok = false;
            }
        }
        println!(
            "  behind_masses_fill {} = {} + {} (relleno trasero)",
            detras.diorama.scene.objects.len(),
            detras.objetos_previos,
            detras.diorama.scene.objects.len() - detras.objetos_previos
        );
        println!(
            "  under_meadows_fill {} = {} + {} (relleno) · materiales +{} · texturas +{}",
            rellena.diorama.scene.objects.len(),
            rellena.objetos_previos,
            rellena.diorama.scene.objects.len() - rellena.objetos_previos,
            rellena.diorama.scene.palette.len() - rellena.paleta_previa,
            rellena.diorama.scene.textures.len() - rellena.texturas_previas
        );

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
    use expedition33_continente_inacabado::bounds::Aabb;
    use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
    use expedition33_continente_inacabado::material::ShadowMode;
    use expedition33_continente_inacabado::scene::{RevealGroup, Scene, SpatialGroupId};
    use expedition33_continente_inacabado::scene_builder::{measure_scene_radius, Blockout};
    use expedition33_continente_inacabado::texture::Texture;
    use std::path::PathBuf;

    /// Lo que una caja puede invadir a otra sin contar como solape.
    const EPS: f32 = 1.0e-4;
    /// Costura seca mínima entre dos pozas y entre una poza y cualquier
    /// otra pieza que no sea el plinto.
    const COSTURA: f32 = 0.10;
    /// Anillo seco alrededor de la huella del Monolito.
    const ANILLO_SECO: f32 = 0.30;
    /// Distancia máxima entre la cara de una caída y el borde de su poza
    /// para que la caída cuente como desembocando en ella.
    const DESEMBOCADURA: f32 = 0.03;

    /// El plinto `G-01` y la masa de fondo bajo la cuarta caída.
    const PLINTO: usize = 0;
    const MASA_BAJO_LA_CUARTA: usize = 4;

    /// Corredores que tienen que quedar secos: `[x0, z0, x1, z1]`. El de
    /// detrás del pedestal, entre las dos pozas de las caídas, y el de
    /// delante, entre el pedestal y la masa del Rompeolas.
    const CORREDORES: [[f32; 4]; 2] = [[-1.50, -3.00, 1.55, -1.40], [-1.50, 1.85, 1.10, 2.04]];

    fn raiz() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn entrega() -> Blockout {
        actual(&raiz())
    }

    fn nueva() -> Candidato {
        candidato(entrega())
    }

    fn caja(scene: &Scene, i: usize) -> Aabb {
        scene.objects[i].primitive.bounds()
    }

    fn solapan(a: &Aabb, b: &Aabb) -> bool {
        a.min.x < b.max.x - EPS
            && b.min.x < a.max.x - EPS
            && a.min.y < b.max.y - EPS
            && b.min.y < a.max.y - EPS
            && a.min.z < b.max.z - EPS
            && b.min.z < a.max.z - EPS
    }

    /// Separación horizontal entre dos plantas; `0` si se solapan.
    fn hueco_en_planta(a: &Aabb, b: &Aabb) -> f32 {
        let dx = (b.min.x - a.max.x).max(a.min.x - b.max.x).max(0.0);
        let dz = (b.min.z - a.max.z).max(a.min.z - b.max.z).max(0.0);
        dx.hypot(dz)
    }

    /// Las pozas de la candidata. Exige que estén todas: ningún test de la
    /// cuenca puede pasar en vacío.
    fn pozas(c: &Candidato) -> Vec<usize> {
        let p: Vec<usize> = (c.objetos_previos..c.diorama.scene.objects.len()).collect();
        assert_eq!(p.len(), POZAS.len(), "la candidata no trae sus pozas");
        p
    }

    fn texeles(t: &Texture) -> u64 {
        let mut h: u64 = 0xCBF2_9CE4_8422_2325;
        for y in 0..t.height() {
            for x in 0..t.width() {
                let u = (x as f32 + 0.5) / t.width() as f32;
                let v = (y as f32 + 0.5) / t.height() as f32;
                for b in format!("{:?}", t.sample(u, v)).bytes() {
                    h ^= u64::from(b);
                    h = h.wrapping_mul(0x0100_0000_01B3);
                }
            }
        }
        h
    }

    #[test]
    fn la_candidata_suma_seis_pozas_a_las_168() {
        let e = entrega();
        let c = nueva();
        assert_eq!(e.scene.objects.len(), 168);
        assert_eq!(c.objetos_previos, 168);
        assert_eq!(c.diorama.scene.objects.len(), 174, "experimental: 168 + 6");
        let agua = c.agua.expect("material de la cuenca");
        for i in pozas(&c) {
            let o = &c.diorama.scene.objects[i];
            assert_eq!(o.final_material, agua, "poza {i}");
            assert_eq!(o.spatial_group, SpatialGroupId::Global);
            assert_eq!(o.reveal_group, RevealGroup::Meadows);
            assert_eq!(
                o.initial_material,
                e.scene.objects[caidas(&e.scene)[0]].initial_material,
                "nace en el lienzo, como las caidas"
            );
        }
    }

    /// Lo que no es la cuenca ni las cuatro alturas es la entrega byte a
    /// byte: objetos, paleta, texturas, cielo, anclas, escala, cámaras,
    /// luces y la jerarquía de los grupos intactos.
    #[test]
    fn fuera_de_la_cuenca_y_las_caidas_nada_cambia() {
        let e = entrega();
        let c = nueva();
        let d = &c.diorama;
        let alargadas: Vec<usize> = c.prolongadas.iter().map(|p| p.objeto).collect();
        assert_eq!(alargadas.len(), 4, "cuatro caidas exteriores");

        for (i, (x, y)) in e.scene.objects.iter().zip(&d.scene.objects).enumerate() {
            if alargadas.contains(&i) {
                let (a, b) = (x.primitive.bounds(), y.primitive.bounds());
                assert_eq!(a.max, b.max, "caida {i}: solo baja el pie");
                assert_eq!((a.min.x, a.min.z), (b.min.x, b.min.z), "caida {i}");
                assert!(b.min.y < a.min.y, "caida {i} no bajo");
                assert_eq!(x.final_material, y.final_material);
                assert_eq!(x.initial_material, y.initial_material);
                assert_eq!(x.spatial_group, y.spatial_group);
                assert_eq!(x.reveal_group, y.reveal_group);
            } else {
                assert_eq!(format!("{x:?}"), format!("{y:?}"), "objeto {i}");
            }
        }
        assert_eq!(d.scene.palette.len(), e.scene.palette.len() + 1);
        for (i, (m, n)) in e.scene.palette.iter().zip(&d.scene.palette).enumerate() {
            assert_eq!(format!("{m:?}"), format!("{n:?}"), "material {i}");
        }
        assert_eq!(d.scene.textures.len(), e.scene.textures.len());
        assert_eq!(
            c.texturas_previas,
            e.scene.textures.len(),
            "sin texturas nuevas"
        );
        for (i, (t, u)) in e.scene.textures.iter().zip(&d.scene.textures).enumerate() {
            assert_eq!((t.width(), t.height()), (u.width(), u.height()));
            assert_eq!(texeles(t), texeles(u), "textura {i}");
        }
        assert_eq!(
            format!("{:?}", e.scene.skybox),
            format!("{:?}", d.scene.skybox)
        );
        assert_eq!(format!("{:?}", e.anchors), format!("{:?}", d.anchors));
        assert_eq!(format!("{:?}", e.scale), format!("{:?}", d.scale));
        assert_eq!(e.hero_preset(), d.hero_preset());
        assert_eq!(
            format!("{:?}", e.hero_camera()),
            format!("{:?}", d.hero_camera())
        );
        assert_eq!(
            format!("{:?}", camara_cenital(&e)),
            format!("{:?}", camara_cenital(d))
        );
        assert_eq!(
            format!("{:?}", luces_del_diorama(&e.anchors, &e.scale)),
            format!("{:?}", luces_del_diorama(&d.anchors, &d.scale))
        );
        for (g, h) in e.accel.groups.iter().zip(&d.accel.groups) {
            assert_eq!(g.id, h.id);
            if g.id != SpatialGroupId::Global && g.id != SpatialGroupId::Meadows {
                assert_eq!(format!("{g:?}"), format!("{h:?}"), "grupo {:?}", g.id);
            }
        }
    }

    #[test]
    fn la_candidata_es_determinista() {
        let (a, b) = (nueva(), nueva());
        assert_eq!(
            format!("{:?}", a.diorama.scene.objects),
            format!("{:?}", b.diorama.scene.objects)
        );
        assert_eq!(
            format!("{:?}", a.diorama.scene.palette),
            format!("{:?}", b.diorama.scene.palette)
        );
        assert_eq!(a.prolongadas, b.prolongadas);
    }

    /// El agua de la cuenca es local y conserva la óptica aprobada: los
    /// techos, el `ior` y el modo de sombra de `A-01` y de las caídas.
    #[test]
    fn el_agua_de_la_cuenca_conserva_la_optica_aprobada() {
        let c = nueva();
        let scene = &c.diorama.scene;
        let agua = c.agua.expect("material de la cuenca");
        assert!(agua.0 >= c.paleta_previa, "el material es nuevo");

        let a01 = *indices(scene, SpatialGroupId::FlyingWaters)
            .last()
            .expect("A-01");
        let mut referencias = vec![scene.objects[a01].final_material];
        referencias.extend(
            caidas(scene)
                .iter()
                .map(|&i| scene.objects[i].final_material),
        );
        let m = scene.material(agua);
        for r in referencias {
            let r = scene.material(r);
            assert_eq!((m.reflection_cap, m.transmission_cap), (0.9, 0.9));
            assert_eq!(
                (m.reflection_cap, m.transmission_cap, m.ior),
                (r.reflection_cap, r.transmission_cap, r.ior)
            );
            assert_eq!(m.shadow_mode, r.shadow_mode);
        }
        assert_eq!(m.shadow_mode, ShadowMode::Ignore);
        assert!(m.is_valid());
        for (i, o) in scene.objects.iter().enumerate() {
            assert_eq!(o.final_material == agua, i >= c.objetos_previos, "{i}");
        }
    }

    /// Sin volúmenes refractivos superpuestos ni caras que se toquen: entre
    /// pozas queda una costura seca, y una poza solo invade el plinto.
    #[test]
    fn las_pozas_no_se_solapan_ni_invaden_nada_salvo_el_plinto() {
        let c = nueva();
        let scene = &c.diorama.scene;
        let p = pozas(&c);
        let alimentan: Vec<usize> = c
            .prolongadas
            .iter()
            .filter(|x| x.apoyo == PLINTO)
            .map(|x| x.objeto)
            .collect();

        for (k, &i) in p.iter().enumerate() {
            let a = caja(scene, i);
            for &j in &p[k + 1..] {
                let b = caja(scene, j);
                assert!(
                    hueco_en_planta(&a, &b) >= COSTURA - EPS,
                    "pozas {i} y {j}: costura {}",
                    hueco_en_planta(&a, &b)
                );
            }
            for j in 0..c.objetos_previos {
                if j == PLINTO {
                    continue;
                }
                let b = caja(scene, j);
                assert!(!solapan(&a, &b), "la poza {i} invade el objeto {j}");
                let refractiva = scene.material(scene.objects[j].final_material).ior > 1.0;
                if alimentan.contains(&j) {
                    // Excepción intencional: la caída desemboca en el borde
                    // de su poza. Sin contacto —ninguna cara compartida
                    // entre dos aguas—, pero a menos de `DESEMBOCADURA`.
                    let hueco = hueco_en_planta(&a, &b);
                    assert!(hueco >= 0.01, "poza {i} toca la caida {j}: {hueco}");
                } else if refractiva || b.min.y < a.max.y {
                    assert!(
                        hueco_en_planta(&a, &b) >= COSTURA - EPS,
                        "poza {i} pegada al objeto {j}: {}",
                        hueco_en_planta(&a, &b)
                    );
                }
            }
        }
    }

    /// Profundidad positiva sobre un apoyo real: el fondo dentro del
    /// plinto, el agua por encima, somera, y lejos de los bordes.
    #[test]
    fn las_pozas_apoyan_en_el_plinto_y_son_someras() {
        let c = nueva();
        let scene = &c.diorama.scene;
        let plinto = caja(scene, PLINTO);
        for i in pozas(&c) {
            let a = caja(scene, i);
            assert!(a.min.y < plinto.max.y, "poza {i} flota");
            assert!(a.min.y >= plinto.max.y - 0.05, "poza {i} demasiado hundida");
            let hondo = a.max.y - plinto.max.y;
            assert!((0.05..=0.15).contains(&hondo), "poza {i}: {hondo}");
            for (dentro, fuera) in [
                (a.min.x - plinto.min.x, "oeste"),
                (plinto.max.x - a.max.x, "este"),
                (a.min.z - plinto.min.z, "fondo"),
                (plinto.max.z - a.max.z, "frente"),
            ] {
                assert!(dentro >= 1.0, "poza {i} al borde {fuera}: {dentro}");
            }
        }
    }

    /// Cada caída exterior baja hasta la superficie que tiene debajo, en su
    /// misma planta; las que caen al plinto desembocan en el borde de una
    /// poza. Las dos interiores ya apoyaban y no cambian.
    #[test]
    fn las_caidas_exteriores_tocan_su_superficie_receptora() {
        let e = entrega();
        let c = nueva();
        let scene = &c.diorama.scene;
        let todas = caidas(scene);
        let p = pozas(&c);

        assert_eq!(
            c.prolongadas.iter().map(|x| x.objeto).collect::<Vec<_>>(),
            todas[..4].to_vec()
        );
        for x in &c.prolongadas {
            let apoyo = caja(scene, x.apoyo);
            let b = caja(scene, x.objeto);
            assert!((b.min.y - (apoyo.max.y - EMPOTRADO)).abs() < EPS);
            assert_eq!(x.pie, b.min.y);
            assert_eq!(x.pie_previo, caja(&e.scene, x.objeto).min.y);
            if x.apoyo == PLINTO {
                let recibe = p.iter().any(|&i| {
                    let a = caja(scene, i);
                    a.min.x <= b.min.x
                        && b.max.x <= a.max.x
                        && (a.min.z - b.max.z) >= 0.0
                        && (a.min.z - b.max.z) <= DESEMBOCADURA
                });
                assert!(recibe, "la caida {} no desemboca en una poza", x.objeto);
                for &i in &p {
                    assert!(!solapan(&b, &caja(scene, i)), "caida {} en poza", x.objeto);
                }
            }
        }
        let apoyos: Vec<usize> = c.prolongadas.iter().map(|x| x.apoyo).collect();
        assert_eq!(apoyos, [PLINTO, PLINTO, PLINTO, MASA_BAJO_LA_CUARTA]);
        for &i in &todas[4..] {
            assert_eq!(
                format!("{:?}", e.scene.objects[i]),
                format!("{:?}", scene.objects[i])
            );
        }
    }

    /// El tramo nuevo de cada caída no atraviesa nada salvo su apoyo y la
    /// roca que ya la respaldaba.
    #[test]
    fn el_tramo_nuevo_de_las_caidas_no_atraviesa_nada() {
        let e = entrega();
        let c = nueva();
        let scene = &c.diorama.scene;
        for x in &c.prolongadas {
            let b = caja(scene, x.objeto);
            let tramo = Aabb::new(
                b.min,
                nalgebra_glm::Vec3::new(b.max.x, x.pie_previo, b.max.z),
            );
            let original = caja(&e.scene, x.objeto);
            for j in 0..scene.objects.len() {
                if j == x.objeto || j == x.apoyo || j == PLINTO {
                    continue;
                }
                let o = caja(scene, j);
                if solapan(&tramo, &o) {
                    assert!(
                        solapan(&original, &o),
                        "la caida {} atraviesa el objeto {j}",
                        x.objeto
                    );
                }
            }
        }
    }

    /// El pedestal emerge con un anillo seco, y los corredores de delante y
    /// de detrás quedan sin agua.
    #[test]
    fn el_pedestal_y_los_corredores_quedan_secos() {
        let c = nueva();
        let scene = &c.diorama.scene;
        let monolito = indices(scene, SpatialGroupId::Monolith)
            .into_iter()
            .map(|i| caja(scene, i))
            .reduce(|a, b| Aabb::new(a.min.inf(&b.min), a.max.sup(&b.max)))
            .expect("Monolito");
        let p = pozas(&c);
        for &i in &p {
            let a = caja(scene, i);
            assert!(
                hueco_en_planta(&a, &monolito) >= ANILLO_SECO,
                "poza {i} a {} del Monolito",
                hueco_en_planta(&a, &monolito)
            );
            assert!(a.max.y < 0.2 && monolito.max.y > 10.0);
            for [x0, z0, x1, z1] in CORREDORES {
                let corredor = Aabb::new(
                    nalgebra_glm::Vec3::new(x0, -1.0, z0),
                    nalgebra_glm::Vec3::new(x1, 1.0, z1),
                );
                assert!(!solapan(&a, &corredor), "poza {i} moja un corredor");
            }
        }
    }

    /// La cuenca no se une a Aguas Voladoras: queda separada en planta y
    /// muy por debajo de su volumen.
    #[test]
    fn la_cuenca_no_se_conecta_con_aguas_voladoras() {
        let c = nueva();
        let scene = &c.diorama.scene;
        for j in indices(scene, SpatialGroupId::FlyingWaters) {
            let b = caja(scene, j);
            for i in pozas(&c) {
                let a = caja(scene, i);
                assert!(
                    hueco_en_planta(&a, &b) >= 0.30,
                    "poza {i} junto a Aguas {j}"
                );
            }
        }
        let a01 = *indices(scene, SpatialGroupId::FlyingWaters).last().unwrap();
        for i in pozas(&c) {
            assert!(caja(scene, i).max.y + 2.0 < caja(scene, a01).max.y);
        }
    }

    /// La escala medida no cambia: la cuenca cabe en la base, así que las
    /// cámaras comparativas siguen siendo las mismas.
    #[test]
    fn la_escala_medida_no_cambia() {
        let e = entrega();
        let c = nueva();
        let d = &c.diorama;
        assert_eq!(
            measure_scene_radius(&d.scene, d.anchors.orbit_center),
            e.scale.scene_radius
        );
    }

    #[test]
    fn la_jerarquia_coincide_con_el_oraculo_lineal() {
        let c = nueva();
        let d = &c.diorama;
        let (w, h) = (200, 150);
        for camara in [d.hero_camera(), camara_cenital(d), camara_del_monolito()] {
            for p in 0..w * h {
                let rayo = camara.ray_from_pixel(p % w, p / w, w, h);
                let mut stats = Default::default();
                assert_eq!(
                    d.accel
                        .intersect(&d.scene, &rayo, &mut stats)
                        .map(|x| x.object_index),
                    d.scene.intersect(&rayo).map(|x| x.object_index)
                );
            }
        }
    }

    /// Píxeles cuyo primer impacto cumple `filtro`.
    fn pixeles(
        d: &Blockout,
        camara: &expedition33_continente_inacabado::camera::Camera,
        filtro: impl Fn(usize) -> bool,
    ) -> usize {
        let mut n = 0;
        for p in 0..ANCHO * ALTO {
            let rayo = camara.ray_from_pixel(p % ANCHO, p / ANCHO, ANCHO, ALTO);
            let mut stats = Default::default();
            if let Some(hit) = d.accel.intersect(&d.scene, &rayo, &mut stats) {
                n += usize::from(filtro(hit.object_index));
            }
        }
        n
    }

    /// El Monolito se sigue leyendo igual: en la hero y en la cenital, los
    /// píxeles que lo ven primero no bajan.
    #[test]
    fn el_monolito_sigue_legible() {
        let e = entrega();
        let c = nueva();
        let mono = indices(&e.scene, SpatialGroupId::Monolith);
        for (camara, toma) in [(e.hero_camera(), "hero"), (camara_cenital(&e), "top78")] {
            let antes = pixeles(&e, &camara, |i| mono.contains(&i));
            let ahora = pixeles(&c.diorama, &camara, |i| mono.contains(&i));
            assert!(ahora >= antes, "{toma}: {ahora} contra {antes}");
        }
    }

    /// La cuenca se ve en la hero y en la cenital, y no quema.
    #[test]
    fn la_cuenca_se_lee_y_no_quema() {
        use expedition33_continente_inacabado::framebuffer::Framebuffer;
        use expedition33_continente_inacabado::renderer::{render, Shading};
        use expedition33_continente_inacabado::reveal::RevealState;

        let c = nueva();
        let d = &c.diorama;
        let luces = luces_del_diorama(&d.anchors, &d.scale);
        let primera = c.objetos_previos;
        for (camara, minimo) in [(d.hero_camera(), 400), (camara_cenital(d), 1500)] {
            let n = pixeles(d, &camara, |i| i >= primera);
            assert!(n >= minimo, "la cuenca apenas se ve: {n} px");
            let mut fb = Framebuffer::new(ANCHO, ALTO);
            render(
                &mut fb,
                &d.scene,
                &d.accel,
                &luces,
                &RevealState::painted(),
                &camara,
                Shading::Material,
            );
            let quemados = fb
                .buffer
                .iter()
                .filter(|&&px| [16, 8, 0].iter().all(|&s| (px >> s) & 0xFF >= 250))
                .count();
            let mut base = Framebuffer::new(ANCHO, ALTO);
            let e = entrega();
            render(
                &mut base,
                &e.scene,
                &e.accel,
                &luces,
                &RevealState::painted(),
                &camara,
                Shading::Material,
            );
            let quemados_antes = base
                .buffer
                .iter()
                .filter(|&&px| [16, 8, 0].iter().all(|&s| (px >> s) & 0xFF >= 250))
                .count();
            assert!(
                quemados <= quemados_antes,
                "quema {quemados} contra {quemados_antes}"
            );
        }
    }

    // ======================================================== flooded_gaps
    //
    // Tests de la variante `flooded_gaps`. Los de arriba, de la candidata,
    // no cambian.

    /// Distancia máxima en planta entre una caída y el agua que la recibe.
    const DESEMBOCADURA_INUNDADA: f32 = 0.03;
    /// Paso del muestreo con el que se mide la base libre, independiente de
    /// la descomposición en celdas.
    const PASO: f32 = 0.05;

    /// La variante. Exige su lámina y sus caídas: ningún test de
    /// `flooded_gaps` puede pasar en vacío.
    /// Separación en planta **por ejes**: la mayor de las dos holguras. Es
    /// la que impide caras coplanares, y la que aplica la variante.
    fn separacion(a: &Aabb, b: &Aabb) -> f32 {
        let dx = (b.min.x - a.max.x).max(a.min.x - b.max.x);
        let dz = (b.min.z - a.max.z).max(a.min.z - b.max.z);
        dx.max(dz)
    }

    fn inundada() -> Inundada {
        let f = flooded_gaps(entrega());
        assert!(!f.celdas.is_empty(), "flooded_gaps no trae su lamina");
        assert_eq!(f.prolongadas.len(), 4, "flooded_gaps no alarga las caidas");
        f
    }

    fn rect(a: &Aabb) -> [f32; 4] {
        [a.min.x, a.min.z, a.max.x, a.max.z]
    }

    fn dentro(r: &[f32; 4], x: f32, z: f32) -> bool {
        r[0] <= x && x <= r[2] && r[1] <= z && z <= r[3]
    }

    /// ¿Está libre el punto de la base? Se mide contra las cajas reales de
    /// la entrega: dentro del plinto con su borde, y fuera de la planta de
    /// cualquier otra pieza más su holgura.
    fn libre(e: &Blockout, x: f32, z: f32) -> bool {
        let plinto = caja(&e.scene, PLINTO);
        if x < plinto.min.x + BORDE
            || x > plinto.max.x - BORDE
            || z < plinto.min.z + BORDE
            || z > plinto.max.z - BORDE
        {
            return false;
        }
        (0..e.scene.objects.len())
            .filter(|&j| j != PLINTO)
            .all(|j| {
                let b = caja(&e.scene, j);
                let h = if es_caida(&e.scene, j) {
                    HOLGURA_CAIDA
                } else {
                    HOLGURA
                };
                !(b.min.x - h < x && x < b.max.x + h && b.min.z - h < z && z < b.max.z + h)
            })
    }

    /// Muestras `(x, z)` de una región, al paso de `PASO`.
    fn muestras(r: [f32; 4]) -> Vec<(f32, f32)> {
        let mut v = Vec::new();
        let mut x = r[0] + PASO * 0.5;
        while x < r[2] {
            let mut z = r[1] + PASO * 0.5;
            while z < r[3] {
                v.push((x, z));
                z += PASO;
            }
            x += PASO;
        }
        v
    }

    fn cajas_de(d: &Blockout, indices: &[usize]) -> Vec<[f32; 4]> {
        indices.iter().map(|&i| rect(&caja(&d.scene, i))).collect()
    }

    #[test]
    fn inundada_cuenta_y_rotula_su_coste() {
        let f = inundada();
        let d = &f.diorama;
        let (agua, lecho) = (f.celdas.len(), f.lechos.len());
        println!(
            "flooded_gaps: {} + {agua} agua + {lecho} lecho = {} (experimental)",
            f.objetos_previos,
            d.scene.objects.len()
        );
        assert_eq!(f.objetos_previos, 168);
        assert!(agua >= 12, "pocas celdas: {agua}");
        assert!(lecho >= 1);
        assert!(
            agua + lecho <= 40,
            "explosion de geometria: {}",
            agua + lecho
        );
        assert_eq!(d.scene.objects.len(), 168 + agua + lecho);
        let esperado: Vec<usize> = (168..168 + agua + lecho).collect();
        assert_eq!([f.celdas.clone(), f.lechos.clone()].concat(), esperado);
    }

    /// Cobertura **en área**, medida por muestreo contra las cajas reales y
    /// no contra la descomposición: el agua cubre al menos el 90 % de la base
    /// libre, en total y en cada una de las cuatro zonas que marcó Charlie, y
    /// nunca moja un punto que no esté libre.
    #[test]
    fn inundada_cubre_la_base_libre_y_las_zonas_marcadas() {
        let e = entrega();
        let f = inundada();
        let agua = cajas_de(&f.diorama, &f.celdas);
        let mojada = |x: f32, z: f32| agua.iter().any(|r| dentro(r, x, z));

        let plinto = rect(&caja(&e.scene, PLINTO));
        let mono = indices(&e.scene, SpatialGroupId::Monolith)
            .iter()
            .map(|&i| rect(&caja(&e.scene, i)))
            .reduce(|a, b| {
                [
                    a[0].min(b[0]),
                    a[1].min(b[1]),
                    a[2].max(b[2]),
                    a[3].max(b[3]),
                ]
            })
            .unwrap();
        let meseta = rect(&caja(
            &e.scene,
            indices(&e.scene, SpatialGroupId::Meadows)[0],
        ));
        let lecho = rect(&caja(
            &e.scene,
            indices(&e.scene, SpatialGroupId::FlyingWaters)[0],
        ));
        let masa = |i: usize| rect(&caja(&e.scene, i));

        let zonas = [
            ("todo el plinto", plinto),
            (
                "oeste del Monolito",
                [masa(5)[2], plinto[1], mono[0], masa(2)[1]],
            ),
            (
                "este del Monolito",
                [mono[2], meseta[3], masa(3)[0], lecho[1]],
            ),
            ("claro derecho", [meseta[2], plinto[1], plinto[2], lecho[1]]),
            ("canal", [masa(1)[2], lecho[1], lecho[0], plinto[3]]),
        ];
        for (nombre, zona) in zonas {
            let (mut libres, mut cubiertas) = (0usize, 0usize);
            for (x, z) in muestras(zona) {
                let l = libre(&e, x, z);
                if mojada(x, z) {
                    assert!(l, "{nombre}: agua en ({x}, {z}), que no esta libre");
                }
                if l {
                    libres += 1;
                    cubiertas += usize::from(mojada(x, z));
                }
            }
            let cobertura = cubiertas as f32 / libres.max(1) as f32;
            println!(
                "{nombre}: libre {:.2} u2, cubierta {:.1} %",
                libres as f32 * PASO * PASO,
                100.0 * cobertura
            );
            assert!(libres > 0, "{nombre}: sin base libre");
            assert!(cobertura >= 0.90, "{nombre}: {:.3}", cobertura);
        }
    }

    /// Las celdas forman **una** lámina continua: cada una comparte un tramo
    /// de borde con otra, y el grafo de contactos es conexo.
    #[test]
    fn inundada_es_una_sola_lamina_continua() {
        let f = inundada();
        let r = cajas_de(&f.diorama, &f.celdas);
        let tocan = |a: &[f32; 4], b: &[f32; 4]| {
            let tramo_z = a[3].min(b[3]) - a[1].max(b[1]);
            let tramo_x = a[2].min(b[2]) - a[0].max(b[0]);
            ((a[2] - b[0]).abs() < EPS || (b[2] - a[0]).abs() < EPS) && tramo_z > 0.05
                || ((a[3] - b[1]).abs() < EPS || (b[3] - a[1]).abs() < EPS) && tramo_x > 0.05
        };
        let mut visto = vec![false; r.len()];
        let mut pila = vec![0];
        visto[0] = true;
        while let Some(i) = pila.pop() {
            for j in 0..r.len() {
                if !visto[j] && tocan(&r[i], &r[j]) {
                    visto[j] = true;
                    pila.push(j);
                }
            }
        }
        let sueltas: Vec<usize> = (0..r.len()).filter(|&i| !visto[i]).collect();
        assert!(sueltas.is_empty(), "celdas sueltas: {sueltas:?}");
    }

    /// Sin doble medio: las celdas de agua no se solapan entre sí ni con
    /// ninguna pieza de la entrega salvo el plinto; el lecho, tampoco.
    #[test]
    fn inundada_tiene_interiores_disjuntos() {
        let f = inundada();
        let s = &f.diorama.scene;
        for grupo in [&f.celdas, &f.lechos] {
            for (k, &i) in grupo.iter().enumerate() {
                for &j in &grupo[k + 1..] {
                    assert!(!solapan(&caja(s, i), &caja(s, j)), "{i} y {j} se solapan");
                }
                for j in (0..f.objetos_previos).filter(|&j| j != PLINTO) {
                    assert!(
                        !solapan(&caja(s, i), &caja(s, j)),
                        "{i} invade el objeto {j}"
                    );
                }
            }
        }
    }

    /// Agua apoyada en la base a una altura común; lecho somero dentro del
    /// agua, sin caras coplanares con el plinto ni con el agua.
    #[test]
    fn inundada_apoya_en_la_base_con_el_lecho_bajo_el_agua() {
        let f = inundada();
        let s = &f.diorama.scene;
        let piso = caja(s, PLINTO).max.y;
        for &i in &f.celdas {
            let a = caja(s, i);
            assert!((a.min.y - (piso - EMPOTRADO)).abs() < EPS, "agua {i}");
            assert!((a.max.y - (piso + TECHO_DEL_AGUA)).abs() < EPS, "agua {i}");
        }
        let agua = cajas_de(&f.diorama, &f.celdas);
        let mojada = |x: f32, z: f32| agua.iter().any(|r| dentro(r, x, z));
        for &i in &f.lechos {
            let b = caja(s, i);
            assert!(b.min.y < piso && piso < b.max.y, "lecho {i}");
            assert!(b.max.y < piso + TECHO_DEL_AGUA - 0.02, "lecho {i}");
            let r = rect(&b);
            let m = RETRANQUEO_DEL_LECHO * 0.99;
            let mut puntos = muestras(r);
            puntos.extend([(r[0], r[1]), (r[0], r[3]), (r[2], r[1]), (r[2], r[3])]);
            for (x, z) in puntos {
                for (dx, dz) in [(-m, -m), (-m, m), (m, -m), (m, m)] {
                    assert!(mojada(x + dx, z + dz), "lecho {i} sin agua en ({x}, {z})");
                }
            }
        }
        // Y casi todo el agua lleva lecho debajo.
        let lechos = cajas_de(&f.diorama, &f.lechos);
        let (mut total, mut con_lecho) = (0usize, 0usize);
        for r in &agua {
            for (x, z) in muestras(*r) {
                total += 1;
                con_lecho += usize::from(lechos.iter().any(|l| dentro(l, x, z)));
            }
        }
        let fraccion = con_lecho as f32 / total as f32;
        println!("agua con lecho debajo: {:.1} %", 100.0 * fraccion);
        assert!(fraccion >= 0.85, "{fraccion}");
    }

    /// Los bloques, las islas y el Monolito emergen: todo lo que había
    /// sobre la base asoma por encima del agua, y el Monolito no cambia.
    #[test]
    fn inundada_deja_emergidos_bloques_y_pedestal() {
        let e = entrega();
        let f = inundada();
        let s = &f.diorama.scene;
        let techo = caja(s, PLINTO).max.y + TECHO_DEL_AGUA;
        for j in (0..f.objetos_previos).filter(|&j| j != PLINTO) {
            assert!(
                caja(s, j).max.y > techo + 0.05,
                "el objeto {j} queda sumergido"
            );
        }
        for i in indices(&e.scene, SpatialGroupId::Monolith) {
            assert_eq!(
                format!("{:?}", e.scene.objects[i]),
                format!("{:?}", s.objects[i])
            );
            for &k in &f.celdas {
                assert!(separacion(&caja(s, k), &caja(s, i)) >= HOLGURA - EPS);
            }
        }
    }

    /// Óptica local: el agua es la de `A-01` —techos, `ior`, sombra—; el
    /// lecho es opaco. Los dos materiales son nuevos y de uso exclusivo.
    #[test]
    fn inundada_conserva_la_optica_y_sus_materiales_son_locales() {
        let e = entrega();
        let f = inundada();
        let s = &f.diorama.scene;
        let (agua, lecho) = (f.agua.expect("agua"), f.lecho.expect("lecho"));
        assert!(agua.0 >= f.paleta_previa && lecho.0 >= f.paleta_previa);
        assert_eq!(s.palette.len(), e.scene.palette.len() + 2);
        assert_eq!(s.textures.len(), e.scene.textures.len());
        let a01 = s.material(
            s.objects[*indices(s, SpatialGroupId::FlyingWaters).last().unwrap()].final_material,
        );
        let m = s.material(agua);
        assert_eq!(
            (m.reflection_cap, m.transmission_cap, m.ior, m.shadow_mode),
            (
                a01.reflection_cap,
                a01.transmission_cap,
                a01.ior,
                a01.shadow_mode
            )
        );
        let l = s.material(lecho);
        assert_eq!(
            (l.reflection_cap, l.transmission_cap, l.ior),
            (0.0, 0.0, 1.0)
        );
        assert_eq!(l.shadow_mode, ShadowMode::Opaque);
        assert!(m.is_valid() && l.is_valid());
        for (i, o) in s.objects.iter().enumerate() {
            assert_eq!(o.final_material == agua, f.celdas.contains(&i), "{i}");
            assert_eq!(o.final_material == lecho, f.lechos.contains(&i), "{i}");
        }
    }

    /// Todo lo demás es la entrega: las mismas cuatro caídas alargadas que
    /// la candidata y ningún otro cambio.
    #[test]
    fn inundada_no_cambia_nada_mas() {
        let e = entrega();
        let c = nueva();
        let f = inundada();
        let d = &f.diorama;
        assert_eq!(f.prolongadas, c.prolongadas);
        for i in 0..f.objetos_previos {
            assert_eq!(
                format!("{:?}", c.diorama.scene.objects[i]),
                format!("{:?}", d.scene.objects[i]),
                "objeto {i}"
            );
        }
        for (i, (m, n)) in e.scene.palette.iter().zip(&d.scene.palette).enumerate() {
            assert_eq!(format!("{m:?}"), format!("{n:?}"), "material {i}");
        }
        for (i, (t, u)) in e.scene.textures.iter().zip(&d.scene.textures).enumerate() {
            assert_eq!(texeles(t), texeles(u), "textura {i}");
        }
        assert_eq!(
            format!("{:?}", e.scene.skybox),
            format!("{:?}", d.scene.skybox)
        );
        assert_eq!(format!("{:?}", e.anchors), format!("{:?}", d.anchors));
        assert_eq!(format!("{:?}", e.scale), format!("{:?}", d.scale));
        assert_eq!(
            format!("{:?}", camara_cenital(&e)),
            format!("{:?}", camara_cenital(d))
        );
        assert_eq!(
            format!("{:?}", luces_del_diorama(&e.anchors, &e.scale)),
            format!("{:?}", luces_del_diorama(&d.anchors, &d.scale))
        );
        for (g, h) in e.accel.groups.iter().zip(&d.accel.groups) {
            if g.id != SpatialGroupId::Global && g.id != SpatialGroupId::Meadows {
                assert_eq!(format!("{g:?}"), format!("{h:?}"), "grupo {:?}", g.id);
            }
        }
        assert_eq!(
            measure_scene_radius(&d.scene, d.anchors.orbit_center),
            e.scale.scene_radius
        );
    }

    /// Las tres caídas que llegan al plinto desembocan en el borde de una
    /// celda, sin tocarla; la cuarta sigue en su masa elevada.
    #[test]
    fn inundada_recibe_las_caidas_donde_la_geometria_lo_permite() {
        let f = inundada();
        let s = &f.diorama.scene;
        let agua = cajas_de(&f.diorama, &f.celdas);
        for x in &f.prolongadas {
            let b = caja(s, x.objeto);
            for &i in &f.celdas {
                assert!(!solapan(&b, &caja(s, i)), "caida {} en agua {i}", x.objeto);
            }
            if x.apoyo == PLINTO {
                // El frente entero de la caída da al agua: la unión de las
                // celdas que empiezan a menos de `DESEMBOCADURA_INUNDADA`
                // delante de ella cubre todo su ancho.
                let mut tramos: Vec<(f32, f32)> = agua
                    .iter()
                    .filter(|r| {
                        let hueco = r[1] - b.max.z;
                        hueco > 0.0 && hueco <= DESEMBOCADURA_INUNDADA
                    })
                    .map(|r| (r[0], r[2]))
                    .collect();
                tramos.sort_by(|a, c| a.0.total_cmp(&c.0));
                let mut hasta = b.min.x;
                for (x0, x1) in tramos {
                    if x0 <= hasta + EPS {
                        hasta = hasta.max(x1);
                    }
                }
                assert!(
                    hasta >= b.max.x - EPS,
                    "la caida {} no desemboca entera",
                    x.objeto
                );
            } else {
                assert_eq!(x.apoyo, MASA_BAJO_LA_CUARTA);
            }
        }
    }

    /// Aguas Voladoras no se toca: ni solapes ni contacto, y su superficie
    /// queda muy por encima de la lámina. No hay conexión hidráulica.
    #[test]
    fn inundada_no_se_une_a_aguas_voladoras() {
        let f = inundada();
        let s = &f.diorama.scene;
        let aguas = indices(s, SpatialGroupId::FlyingWaters);
        for &i in f.celdas.iter().chain(&f.lechos) {
            for &j in &aguas {
                assert!(!solapan(&caja(s, i), &caja(s, j)));
                assert!(separacion(&caja(s, i), &caja(s, j)) >= HOLGURA - EPS);
            }
        }
        let a01 = caja(s, *aguas.last().unwrap());
        let techo = caja(s, PLINTO).max.y + TECHO_DEL_AGUA;
        println!("desnivel con A-01: {:.2}", a01.max.y - techo);
        assert!(a01.min.y > techo && a01.max.y - techo > 2.4);
    }

    #[test]
    fn inundada_es_determinista_y_su_jerarquia_es_la_lineal() {
        let (a, b) = (inundada(), inundada());
        assert_eq!(
            format!("{:?}", a.diorama.scene.objects),
            format!("{:?}", b.diorama.scene.objects)
        );
        assert_eq!(
            format!("{:?}", a.diorama.scene.palette),
            format!("{:?}", b.diorama.scene.palette)
        );
        let d = &a.diorama;
        let (w, h) = (200, 150);
        for camara in [d.hero_camera(), camara_cenital(d), camara_del_monolito()] {
            for p in 0..w * h {
                let rayo = camara.ray_from_pixel(p % w, p / w, w, h);
                let mut stats = Default::default();
                assert_eq!(
                    d.accel
                        .intersect(&d.scene, &rayo, &mut stats)
                        .map(|x| x.object_index),
                    d.scene.intersect(&rayo).map(|x| x.object_index)
                );
            }
        }
    }

    /// Lo que ve el primer rayo refractado de cada píxel cuyo primer impacto
    /// es la cara superior de una celda: `(lecho, cara lateral de agua, otra
    /// cosa)`.
    fn a_traves(
        f: &Inundada,
        camara: &expedition33_continente_inacabado::camera::Camera,
    ) -> Recuento {
        use expedition33_continente_inacabado::optics::refracted_ray;
        let d = &f.diorama;
        let agua = cajas_de(d, &f.celdas);
        // ¿Está el punto en una cara que dos celdas comparten?
        let compartida = |x: f32, z: f32| agua.iter().filter(|r| dentro(r, x, z)).count() >= 2;
        let mut n = Recuento::default();
        for p in 0..ANCHO * ALTO {
            let rayo = camara.ray_from_pixel(p % ANCHO, p / ANCHO, ANCHO, ALTO);
            let mut stats = Default::default();
            let Some(hit) = d.accel.intersect(&d.scene, &rayo, &mut stats) else {
                continue;
            };
            if !f.celdas.contains(&hit.object_index) || !hit.front_face || hit.normal.y < 0.9 {
                continue;
            }
            n.agua += 1;
            let ior = d
                .scene
                .material(d.scene.objects[hit.object_index].final_material)
                .ior;
            let Some(dentro) = refracted_ray(&hit, &rayo.direction, ior) else {
                continue;
            };
            match d.accel.intersect(&d.scene, &dentro, &mut stats) {
                Some(h) if f.lechos.contains(&h.object_index) => n.lecho += 1,
                Some(h) if f.celdas.contains(&h.object_index) && h.normal.y.abs() < 0.5 => {
                    if compartida(h.point.x, h.point.z) {
                        n.costura += 1;
                    } else {
                        n.borde += 1;
                    }
                }
                _ => n.otra += 1,
            }
        }
        n
    }

    /// Lo que ve el primer rayo refractado de los píxeles de agua.
    #[derive(Debug, Default)]
    struct Recuento {
        agua: usize,
        lecho: usize,
        /// Una cara lateral que dos celdas comparten: la costura.
        costura: usize,
        /// Una cara lateral exterior: el borde real de la lámina.
        borde: usize,
        otra: usize,
    }

    /// El lecho se ve **a través** del agua, por refracción: la mayoría de
    /// los píxeles de agua ve el lecho en su primer rayo refractado.
    #[test]
    fn inundada_deja_ver_el_lecho_por_refraccion() {
        let f = inundada();
        let d = &f.diorama;
        for (camara, toma, minimo) in [
            (d.hero_camera(), "hero", 0.70),
            (camara_cenital(d), "top78", 0.85),
        ] {
            let n = a_traves(&f, &camara);
            let fraccion = n.lecho as f32 / n.agua.max(1) as f32;
            println!("{toma}: {n:?}");
            assert!(
                n.agua >= 500,
                "{toma}: el agua apenas se ve ({} px)",
                n.agua
            );
            assert!(fraccion >= minimo, "{toma}: {fraccion}");
        }
    }

    /// El riesgo de las costuras entre celdas que se tocan, medido: el
    /// renderer decide entrar o salir del agua por `front_face`, así que un
    /// rayo refractado que alcanza la cara compartida «sale» al aire —o se
    /// refleja por completo— aunque al otro lado siga el agua. Con la lámina
    /// fina sobre el lecho, casi ningún rayo llega a esa cara antes que al
    /// lecho.
    #[test]
    fn inundada_acota_el_riesgo_de_las_costuras() {
        let f = inundada();
        let d = &f.diorama;
        for (camara, toma) in [(d.hero_camera(), "hero"), (camara_cenital(d), "top78")] {
            let n = a_traves(&f, &camara);
            let costura = n.costura as f32 / n.agua.max(1) as f32;
            let borde = n.borde as f32 / n.agua.max(1) as f32;
            println!(
                "{toma}: de {} px de agua, {} ven una costura ({:.2} %) y {} un borde exterior ({:.2} %)",
                n.agua,
                n.costura,
                100.0 * costura,
                n.borde,
                100.0 * borde
            );
            assert!(costura <= 0.02, "{toma}: costuras {costura}");
        }
    }

    #[test]
    fn inundada_deja_legible_el_monolito_y_no_quema() {
        use expedition33_continente_inacabado::framebuffer::Framebuffer;
        use expedition33_continente_inacabado::renderer::{render, Shading};
        use expedition33_continente_inacabado::reveal::RevealState;

        let e = entrega();
        let f = inundada();
        let d = &f.diorama;
        let mono = indices(&e.scene, SpatialGroupId::Monolith);
        let luces = luces_del_diorama(&e.anchors, &e.scale);
        let quemados =
            |b: &Blockout, camara: &expedition33_continente_inacabado::camera::Camera| {
                let mut fb = Framebuffer::new(ANCHO, ALTO);
                render(
                    &mut fb,
                    &b.scene,
                    &b.accel,
                    &luces,
                    &RevealState::painted(),
                    camara,
                    Shading::Material,
                );
                fb.buffer
                    .iter()
                    .filter(|&&px| [16, 8, 0].iter().all(|&s| (px >> s) & 0xFF >= 250))
                    .count()
            };
        for camara in [e.hero_camera(), camara_cenital(&e)] {
            let antes = pixeles(&e, &camara, |i| mono.contains(&i));
            let ahora = pixeles(d, &camara, |i| mono.contains(&i));
            assert!(ahora >= antes, "Monolito {ahora} contra {antes}");
            assert!(quemados(d, &camara) <= quemados(&e, &camara));
        }
    }

    // ======================================================== flooded_blue
    //
    // Tests de `flooded_blue`. Los de arriba no cambian.

    /// La variante. Exige un material por pieza de lecho: ningún test puede
    /// pasar en vacío.
    fn azulada() -> Azulada {
        let a = flooded_blue(entrega());
        assert_eq!(
            a.fondos.len(),
            a.lechos.len(),
            "flooded_blue no trae sus fondos"
        );
        assert!(!a.fondos.is_empty());
        a
    }

    /// La geometría es la de `flooded_gaps`, entera: 205 piezas con la misma
    /// primitiva, lienzo, grupos y jerarquía. Solo cambia el material final
    /// de las celdas y del lecho.
    #[test]
    fn azul_congela_la_geometria_de_flooded_gaps() {
        let f = inundada();
        let a = azulada();
        let (x, y) = (&f.diorama, &a.diorama);
        assert_eq!(y.scene.objects.len(), 205);
        assert_eq!((a.celdas.len(), a.lechos.len()), (19, 18));
        assert_eq!(a.celdas, f.celdas);
        assert_eq!(a.lechos, f.lechos);
        for (i, (o, n)) in x.scene.objects.iter().zip(&y.scene.objects).enumerate() {
            assert_eq!(
                format!("{:?}", o.primitive),
                format!("{:?}", n.primitive),
                "{i}"
            );
            assert_eq!(o.initial_material, n.initial_material, "{i}");
            assert_eq!(o.spatial_group, n.spatial_group, "{i}");
            assert_eq!(o.reveal_group, n.reveal_group, "{i}");
            if !a.celdas.contains(&i) && !a.lechos.contains(&i) {
                assert_eq!(o.final_material, n.final_material, "{i}");
            }
        }
        assert_eq!(format!("{:?}", x.accel), format!("{:?}", y.accel));
        assert_eq!(format!("{:?}", x.anchors), format!("{:?}", y.anchors));
        assert_eq!(format!("{:?}", x.scale), format!("{:?}", y.scale));
        assert_eq!(
            format!("{:?}", x.scene.skybox),
            format!("{:?}", y.scene.skybox)
        );
        assert_eq!(
            format!("{:?}", x.hero_camera()),
            format!("{:?}", y.hero_camera())
        );
        assert_eq!(
            format!("{:?}", camara_cenital(x)),
            format!("{:?}", camara_cenital(y))
        );
        assert_eq!(
            format!("{:?}", luces_del_diorama(&x.anchors, &x.scale)),
            format!("{:?}", luces_del_diorama(&y.anchors, &y.scale))
        );
    }

    /// Lo compartido no se toca: la paleta y las texturas de `flooded_gaps`
    /// quedan delante, iguales; solo se añaden el agua y un fondo con su
    /// textura por pieza de lecho.
    #[test]
    fn azul_no_toca_materiales_ni_texturas_previos() {
        let f = inundada();
        let a = azulada();
        let (x, y) = (&f.diorama.scene, &a.diorama.scene);
        assert_eq!(a.paleta_previa, x.palette.len());
        assert_eq!(a.texturas_previas, x.textures.len());
        assert_eq!(y.palette.len(), x.palette.len() + 1 + a.lechos.len());
        assert_eq!(y.textures.len(), x.textures.len() + a.lechos.len());
        for (i, (m, n)) in x.palette.iter().zip(&y.palette).enumerate() {
            assert_eq!(format!("{m:?}"), format!("{n:?}"), "material {i}");
        }
        for (i, (t, u)) in x.textures.iter().zip(&y.textures).enumerate() {
            assert_eq!(texeles(t), texeles(u), "textura {i}");
        }
    }

    /// El agua conserva la óptica aprobada —techos, `ior`, sombra—, es local
    /// y más clara que la de `flooded_gaps`.
    #[test]
    fn azul_conserva_la_optica_del_agua() {
        let f = inundada();
        let a = azulada();
        let s = &a.diorama.scene;
        let previa = s.material(f.agua.unwrap());
        let m = s.material(a.agua);
        assert!(a.agua.0 >= a.paleta_previa);
        assert_eq!(
            (m.reflection_cap, m.transmission_cap, m.ior, m.shadow_mode),
            (
                previa.reflection_cap,
                previa.transmission_cap,
                previa.ior,
                previa.shadow_mode
            )
        );
        assert_eq!(
            m.albedo_texture, previa.albedo_texture,
            "reusa la textura del agua"
        );
        assert!(m.is_valid());
        let luma = |c: expedition33_continente_inacabado::color::Color| {
            0.2126 * c.r + 0.7152 * c.g + 0.0722 * c.b
        };
        assert!(luma(m.albedo) > luma(previa.albedo), "no es mas clara");
        assert!(m.albedo.b >= m.albedo.r, "no es azul");
        for (i, o) in s.objects.iter().enumerate() {
            assert_eq!(o.final_material == a.agua, a.celdas.contains(&i), "{i}");
        }
    }

    /// Cada pieza de lecho lleva su propio fondo: opaco, con una textura
    /// procedural no vacía y con variación, que nadie más usa.
    #[test]
    fn azul_cada_lecho_lleva_su_fondo_local_texturado() {
        let a = azulada();
        let s = &a.diorama.scene;
        for (k, &i) in a.lechos.iter().enumerate() {
            let id = a.fondos[k];
            assert_eq!(s.objects[i].final_material, id);
            assert!(id.0 >= a.paleta_previa);
            let m = s.material(id);
            assert_eq!(
                (m.reflection_cap, m.transmission_cap, m.ior),
                (0.0, 0.0, 1.0)
            );
            assert_eq!(m.shadow_mode, ShadowMode::Opaque);
            assert!(m.is_valid());
            let tex = m.albedo_texture.expect("fondo con textura");
            assert!(tex.0 >= a.texturas_previas);
            let t = s.texture(tex);
            assert!(t.width() >= 2 && t.height() >= 2);
            let b = caja(s, i);
            let (w, h) = (b.max.x - b.min.x, b.max.z - b.min.z);
            assert!(
                (t.width() as f32 - w * TEXELS_POR_UNIDAD).abs() <= 1.0,
                "{k}"
            );
            assert!(
                (t.height() as f32 - h * TEXELS_POR_UNIDAD).abs() <= 1.0,
                "{k}"
            );
            let mut lumas: Vec<f32> = Vec::new();
            for y in 0..t.height() {
                for x in 0..t.width() {
                    let c = t.sample(
                        (x as f32 + 0.5) / t.width() as f32,
                        (y as f32 + 0.5) / t.height() as f32,
                    );
                    lumas.push(0.2126 * c.r + 0.7152 * c.g + 0.0722 * c.b);
                }
            }
            let (min, max) = lumas
                .iter()
                .fold((f32::MAX, f32::MIN), |(a, b), &l| (a.min(l), b.max(l)));
            assert!(max - min > 0.02, "lecho {k} plano: {min}..{max}");
            for (j, o) in s.objects.iter().enumerate() {
                if j != i {
                    assert_ne!(o.final_material, id, "fondo {k} compartido con {j}");
                }
            }
        }
    }

    /// El fondo es **un solo campo en coordenadas de mundo**: el texel de
    /// cada pieza es `campo_de_roca` en el punto que cubre. Así el lecho
    /// sigue entre piezas y no se estira en las grandes.
    #[test]
    fn azul_el_fondo_es_continuo_en_coordenadas_de_mundo() {
        let a = azulada();
        let s = &a.diorama.scene;
        for (k, &i) in a.lechos.iter().enumerate() {
            let b = caja(s, i);
            let t = s.texture(s.material(a.fondos[k]).albedo_texture.unwrap());
            for (u, v) in [(0.1, 0.1), (0.5, 0.5), (0.9, 0.3), (0.3, 0.9), (0.97, 0.97)] {
                let x = b.min.x + u * (b.max.x - b.min.x);
                let z = b.min.z + v * (b.max.z - b.min.z);
                let tx = ((u * t.width() as f32) as usize).min(t.width() - 1);
                // La misma fila que `Texture::sample`: la `0` es `v = 1`.
                let fila = (((1.0 - v) * t.height() as f32) as usize).min(t.height() - 1);
                let cx = b.min.x + (tx as f32 + 0.5) / t.width() as f32 * (b.max.x - b.min.x);
                let cz =
                    b.min.z + (1.0 - (fila as f32 + 0.5) / t.height() as f32) * (b.max.z - b.min.z);
                let esperado = campo_de_roca(cx, cz);
                let visto = t.sample(u, v);
                assert!(
                    (visto.r - esperado.r).abs() < 1e-6
                        && (visto.g - esperado.g).abs() < 1e-6
                        && (visto.b - esperado.b).abs() < 1e-6,
                    "lecho {k} en ({x}, {z}): {visto:?} contra {esperado:?}"
                );
            }
        }
    }

    #[test]
    fn azul_es_determinista() {
        let (a, b) = (azulada(), azulada());
        assert_eq!(
            format!("{:?}", a.diorama.scene.palette),
            format!("{:?}", b.diorama.scene.palette)
        );
        for (t, u) in a
            .diorama
            .scene
            .textures
            .iter()
            .zip(&b.diorama.scene.textures)
        {
            assert_eq!(texeles(t), texeles(u));
        }
        assert_eq!(
            format!("{:?}", a.diorama.scene.objects),
            format!("{:?}", b.diorama.scene.objects)
        );
    }

    /// Color medio, en sRGB `0..255`, de los píxeles cuyo primer impacto es
    /// la cara superior de una celda, en el render final.
    fn color_del_agua(
        d: &Blockout,
        celdas: &[usize],
        camara: &expedition33_continente_inacabado::camera::Camera,
    ) -> ([f32; 3], usize, usize) {
        use expedition33_continente_inacabado::framebuffer::Framebuffer;
        use expedition33_continente_inacabado::renderer::{render, Shading};
        use expedition33_continente_inacabado::reveal::RevealState;

        let luces = luces_del_diorama(&d.anchors, &d.scale);
        let mut fb = Framebuffer::new(ANCHO, ALTO);
        render(
            &mut fb,
            &d.scene,
            &d.accel,
            &luces,
            &RevealState::painted(),
            camara,
            Shading::Material,
        );
        let (mut suma, mut n, mut quemados) = ([0.0f32; 3], 0usize, 0usize);
        for p in 0..ANCHO * ALTO {
            let rayo = camara.ray_from_pixel(p % ANCHO, p / ANCHO, ANCHO, ALTO);
            let mut stats = Default::default();
            let Some(hit) = d.accel.intersect(&d.scene, &rayo, &mut stats) else {
                continue;
            };
            if !celdas.contains(&hit.object_index) || hit.normal.y < 0.9 {
                continue;
            }
            let px = fb.buffer[p];
            let c = [(px >> 16) & 0xFF, (px >> 8) & 0xFF, px & 0xFF];
            for (s, v) in suma.iter_mut().zip(c) {
                *s += v as f32;
            }
            n += 1;
            quemados += usize::from(c.iter().all(|&v| v >= 250));
        }
        (suma.map(|s| s / n.max(1) as f32), n, quemados)
    }

    /// En el render final —no en el albedo— el agua se lee más clara y azul
    /// que en `flooded_gaps`, sin quemarse y sin un azul saturado: el fondo
    /// sigue siendo más oscuro que lo seco pero no negro.
    #[test]
    fn azul_se_lee_mas_clara_y_azul_en_el_render_final() {
        let f = inundada();
        let a = azulada();
        for (toma, cf, ca) in [
            ("hero", f.diorama.hero_camera(), a.diorama.hero_camera()),
            (
                "top78",
                camara_cenital(&f.diorama),
                camara_cenital(&a.diorama),
            ),
        ] {
            let (antes, _, _) = color_del_agua(&f.diorama, &f.celdas, &cf);
            let (ahora, n, quemados) = color_del_agua(&a.diorama, &a.celdas, &ca);
            let luma = |c: [f32; 3]| 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2];
            let [r, g, b] = ahora;
            let croma = (r.max(g).max(b) - r.min(g).min(b)) / r.max(g).max(b).max(1.0);
            println!(
                "{toma}: agua flooded_gaps {:?} (luma {:.1}) -> flooded_blue {:?} (luma {:.1}), croma {:.2}, {n} px, quemados {quemados}",
                antes.map(|v| v.round()),
                luma(antes),
                ahora.map(|v| v.round()),
                luma(ahora),
                croma
            );
            assert!(luma(ahora) > luma(antes) + 10.0, "{toma}: no es mas clara");
            assert!(b > r + 15.0 && b >= g, "{toma}: no es azul");
            assert!(croma <= 0.65, "{toma}: azul saturado artificial ({croma})");
            assert!(luma(ahora) <= 200.0, "{toma}: se blanquea");
            assert_eq!(quemados, 0, "{toma}: agua quemada");
        }
    }

    // ================================================== flooded_reflective
    //
    // Diagnóstico y tests de `flooded_reflective`. Los de arriba no cambian.

    /// Lo que pasa, de media, en los píxeles cuyo primer impacto es la cara
    /// superior de unas piezas de agua.
    #[derive(Debug, Default, Clone, Copy)]
    struct Superficie {
        pixeles: usize,
        /// `kr = reflection_cap × F`: la fracción que se lleva el rayo
        /// reflejado trazado.
        kr: f32,
        /// Fracción de rayos reflejados que encuentran geometría de la
        /// escena —reflejos de escena— y no el cielo.
        refleja_escena: f32,
        /// Luma media del brillo especular directo de cada luz (`L-01`,
        /// `L-02`, `L-03`), sin sombras: una cota superior.
        especular: [f32; 3],
        /// Distancia media que recorre el rayo refractado hasta su primer
        /// impacto: la profundidad aparente.
        profundidad: f32,
    }

    fn superficie(
        d: &Blockout,
        piezas: &[usize],
        camara: &expedition33_continente_inacabado::camera::Camera,
    ) -> Superficie {
        use expedition33_continente_inacabado::material::direct_specular;
        use expedition33_continente_inacabado::optics::{fresnel, reflected_ray, refracted_ray};

        let luces = luces_del_diorama(&d.anchors, &d.scale);
        let luma = |c: expedition33_continente_inacabado::color::Color| {
            0.2126 * c.r + 0.7152 * c.g + 0.0722 * c.b
        };
        let mut s = Superficie::default();
        let (mut escena, mut prof, mut con_prof) = (0usize, 0.0f32, 0usize);
        for p in 0..ANCHO * ALTO {
            let rayo = camara.ray_from_pixel(p % ANCHO, p / ANCHO, ANCHO, ALTO);
            let mut stats = Default::default();
            let Some(hit) = d.accel.intersect(&d.scene, &rayo, &mut stats) else {
                continue;
            };
            if !piezas.contains(&hit.object_index) || !hit.front_face || hit.normal.y < 0.9 {
                continue;
            }
            let o = &d.scene.objects[hit.object_index];
            let m = d.scene.material(o.final_material);
            s.pixeles += 1;
            s.kr += m.reflection_cap * fresnel(&rayo.direction, &hit.normal, true, m.ior);
            let reflejo = reflected_ray(&hit, &rayo.direction);
            escena += usize::from(d.accel.intersect(&d.scene, &reflejo, &mut stats).is_some());
            if let Some(dentro) = refracted_ray(&hit, &rayo.direction, m.ior) {
                if let Some(h) = d.accel.intersect(&d.scene, &dentro, &mut stats) {
                    prof += h.distance;
                    con_prof += 1;
                }
            }
            for (k, luz) in luces.iter().enumerate() {
                if !luz.affects(o.spatial_group) {
                    continue;
                }
                let hacia = luz.position - hit.point;
                let distancia = hacia.magnitude();
                let c = direct_specular(
                    &m,
                    &hit.normal,
                    &(hacia / distancia),
                    &(-rayo.direction),
                    luz.color,
                    luz.attenuation(distancia),
                );
                s.especular[k] += luma(c);
            }
        }
        let n = s.pixeles.max(1) as f32;
        s.kr /= n;
        s.refleja_escena = escena as f32 / n;
        s.especular = s.especular.map(|v| v / n);
        s.profundidad = prof / con_prof.max(1) as f32;
        s
    }

    /// La causa, medida y no supuesta: por qué el agua de Aguas Voladoras se
    /// lee reflectante y la de la cuenca no, con los mismos techos.
    #[test]
    fn reflejo_diagnostico_de_la_causa() {
        let a = azulada();
        let d = &a.diorama;
        let a01 = *indices(&d.scene, SpatialGroupId::FlyingWaters)
            .last()
            .unwrap();
        for (toma, camara) in [("hero", d.hero_camera()), ("top78", camara_cenital(d))] {
            let bahia = superficie(d, &[a01], &camara);
            let cuenca = superficie(d, &a.celdas, &camara);
            println!("{toma} A-01:   {bahia:?}");
            println!("{toma} cuenca: {cuenca:?}");
            // Mismos techos e ior, planos horizontales vistos casi igual: el
            // reflejo trazado pesa lo mismo.
            let r = cuenca.kr / bahia.kr;
            assert!((0.7..=1.4).contains(&r), "{toma}: kr {r}");
            // L-02 ilumina la bahía y, por light linking, nunca la cuenca.
            assert_eq!(cuenca.especular[1], 0.0, "{toma}: L-02 en la cuenca");
            assert!(bahia.especular[1] > 0.0, "{toma}: L-02 no llega a A-01");
            // La bahía es honda; la cuenca, una lámina sobre su lecho.
            assert!(bahia.profundidad > 5.0 * cuenca.profundidad, "{toma}");
        }
    }

    /// La variante. Exige un agua nueva: ningún test puede pasar en vacío.
    fn reflectante() -> Reflectante {
        let r = flooded_reflective(entrega());
        assert_ne!(r.agua, r.agua_previa, "flooded_reflective no trae su agua");
        r
    }

    /// Todo es `flooded_blue` salvo el material final de las 19 celdas:
    /// geometría, lechos con sus fondos y texturas, paleta previa, jerarquía,
    /// cámaras, luces y escala.
    #[test]
    fn reflectiva_solo_cambia_el_material_del_agua() {
        let a = azulada();
        let r = reflectante();
        let (x, y) = (&a.diorama, &r.diorama);
        assert_eq!(y.scene.objects.len(), 205);
        assert_eq!(
            (r.celdas.clone(), r.lechos.clone()),
            (a.celdas.clone(), a.lechos.clone())
        );
        for (i, (o, n)) in x.scene.objects.iter().zip(&y.scene.objects).enumerate() {
            if r.celdas.contains(&i) {
                assert_eq!(format!("{:?}", o.primitive), format!("{:?}", n.primitive));
                assert_eq!(o.initial_material, n.initial_material);
                assert_eq!(
                    (o.spatial_group, o.reveal_group),
                    (n.spatial_group, n.reveal_group)
                );
                assert_eq!(n.final_material, r.agua, "{i}");
            } else {
                assert_eq!(format!("{o:?}"), format!("{n:?}"), "objeto {i}");
            }
        }
        assert_eq!(r.paleta_previa, x.scene.palette.len());
        assert_eq!(y.scene.palette.len(), x.scene.palette.len() + 1);
        for (i, (m, n)) in x.scene.palette.iter().zip(&y.scene.palette).enumerate() {
            assert_eq!(format!("{m:?}"), format!("{n:?}"), "material {i}");
        }
        assert_eq!(r.texturas_previas, x.scene.textures.len());
        assert_eq!(y.scene.textures.len(), x.scene.textures.len());
        for (i, (t, u)) in x.scene.textures.iter().zip(&y.scene.textures).enumerate() {
            assert_eq!(texeles(t), texeles(u), "textura {i}");
        }
        assert_eq!(format!("{:?}", x.accel), format!("{:?}", y.accel));
        assert_eq!(format!("{:?}", x.anchors), format!("{:?}", y.anchors));
        assert_eq!(format!("{:?}", x.scale), format!("{:?}", y.scale));
        assert_eq!(
            format!("{:?}", camara_cenital(x)),
            format!("{:?}", camara_cenital(y))
        );
        assert_eq!(
            format!("{:?}", luces_del_diorama(&x.anchors, &x.scale)),
            format!("{:?}", luces_del_diorama(&y.anchors, &y.scale))
        );
    }

    /// El agua nueva es local y conserva todas las constantes ópticas, la
    /// textura compartida y el tinte azul aprobados. Solo cambia el brillo
    /// especular directo.
    #[test]
    fn reflectiva_conserva_la_optica_y_el_azul_aprobados() {
        let r = reflectante();
        let s = &r.diorama.scene;
        assert!(r.agua.0 >= r.paleta_previa);
        let (p, m) = (s.material(r.agua_previa), s.material(r.agua));
        assert_eq!(
            (m.reflection_cap, m.transmission_cap, m.ior, m.shadow_mode),
            (p.reflection_cap, p.transmission_cap, p.ior, p.shadow_mode)
        );
        assert_eq!(
            (m.reflection_cap, m.transmission_cap, m.ior),
            (0.9, 0.9, 1.333)
        );
        assert_eq!(m.shadow_mode, ShadowMode::Ignore);
        assert_eq!(m.albedo_texture, p.albedo_texture, "textura compartida");
        assert_eq!(
            format!("{:?}", m.albedo),
            format!("{:?}", p.albedo),
            "tinte"
        );
        assert_eq!(m.uv_scale, p.uv_scale);
        assert!(
            (m.specular_strength, m.shininess) != (p.specular_strength, p.shininess),
            "el brillo no cambio"
        );
        assert!(m.is_valid());
        for (i, o) in s.objects.iter().enumerate() {
            assert_eq!(o.final_material == r.agua, r.celdas.contains(&i), "{i}");
        }
    }

    /// El reflejo **trazado** no cambia: el mismo `kr` en cada píxel de agua
    /// y los mismos rayos reflejados y refractados por toma. Lo que cambia es
    /// el brillo especular directo, que el renderer suma sin lanzar rayos.
    #[test]
    fn reflectiva_distingue_brillo_superficial_de_reflejo_trazado() {
        use expedition33_continente_inacabado::framebuffer::Framebuffer;
        use expedition33_continente_inacabado::renderer::{render, Shading};
        use expedition33_continente_inacabado::reveal::RevealState;

        let a = azulada();
        let r = reflectante();
        for (toma, camara) in [
            ("hero", a.diorama.hero_camera()),
            ("top78", camara_cenital(&a.diorama)),
            ("detalle", camara_del_monolito()),
        ] {
            let antes = superficie(&a.diorama, &a.celdas, &camara);
            let ahora = superficie(&r.diorama, &r.celdas, &camara);
            assert_eq!(antes.pixeles, ahora.pixeles, "{toma}");
            assert_eq!(antes.kr, ahora.kr, "{toma}: kr");
            assert_eq!(antes.refleja_escena, ahora.refleja_escena, "{toma}");
            let espec = |s: &Superficie| s.especular.iter().sum::<f32>();
            assert!(espec(&ahora) > espec(&antes), "{toma}: el brillo no subio");

            let rayos = |d: &Blockout| {
                let luces = luces_del_diorama(&d.anchors, &d.scale);
                let mut fb = Framebuffer::new(ANCHO, ALTO);
                let st = render(
                    &mut fb,
                    &d.scene,
                    &d.accel,
                    &luces,
                    &RevealState::painted(),
                    &camara,
                    Shading::Material,
                );
                (st.reflection_rays, st.refraction_rays)
            };
            let (ra, rr) = (rayos(&a.diorama), rayos(&r.diorama));
            println!(
                "{toma}: kr {:.4} (igual), refleja escena {:.3} (igual), especular {:.5} -> {:.5}, rayos {ra:?} = {rr:?}",
                ahora.kr,
                ahora.refleja_escena,
                espec(&antes),
                espec(&ahora)
            );
            assert_eq!(ra, rr, "{toma}: rayos trazados");
        }
    }

    /// Medidas del render final, por toma: color medio, percentiles de luma
    /// y píxeles quemados del agua. Solo se exige no quemar; lo demás se
    /// informa para el juicio de Charlie.
    #[test]
    fn reflectiva_mide_brillo_color_y_quemados() {
        let a = azulada();
        let r = reflectante();
        for (toma, camara) in [
            ("hero", a.diorama.hero_camera()),
            ("top78", camara_cenital(&a.diorama)),
            ("detalle", camara_del_monolito()),
        ] {
            let (antes, _, _) = color_del_agua(&a.diorama, &a.celdas, &camara);
            let (ahora, n, quemados) = color_del_agua(&r.diorama, &r.celdas, &camara);
            println!(
                "{toma}: agua flooded_blue {:?} -> flooded_reflective {:?}, {n} px, quemados {quemados}",
                antes.map(|v| v.round()),
                ahora.map(|v| v.round())
            );
            assert_eq!(quemados, 0, "{toma}: agua quemada");
        }
    }

    #[test]
    fn reflectiva_es_determinista() {
        let (a, b) = (reflectante(), reflectante());
        assert_eq!(
            format!("{:?}", a.diorama.scene.palette),
            format!("{:?}", b.diorama.scene.palette)
        );
        assert_eq!(
            format!("{:?}", a.diorama.scene.objects),
            format!("{:?}", b.diorama.scene.objects)
        );
    }

    // ================================================== flooded_blue_raised
    //
    // Tests de `flooded_blue_raised`. Los de arriba no cambian.

    /// Separación por ejes a partir de la cual una pieza cuenta como vecina
    /// de una celda: lo bastante cerca para verse junto al agua.
    const VECINDAD: f32 = 0.10;
    /// Lo que una pieza vecina tiene que asomar sobre el agua.
    const EMERGENCIA: f32 = 0.10;

    /// La variante. Exige un nivel más alto: ningún test pasa en vacío.
    fn elevada() -> Elevada {
        let e = flooded_blue_raised(entrega());
        assert!(
            e.nivel > e.nivel_previo,
            "flooded_blue_raised no sube el agua"
        );
        e
    }

    /// Todo es `flooded_blue` salvo el techo de las 19 celdas: mismas 205
    /// piezas, el mismo fondo de cada celda, sus mismos materiales, lechos,
    /// texturas, cámaras, luces, anclas y escala.
    #[test]
    fn elevada_solo_sube_el_techo_de_las_19_celdas() {
        let a = azulada();
        let e = elevada();
        let (x, y) = (&a.diorama, &e.diorama);
        let piso = caja(&x.scene, PLINTO).max.y;
        assert_eq!(y.scene.objects.len(), 205);
        assert_eq!(
            (e.celdas.clone(), e.lechos.clone()),
            (a.celdas.clone(), a.lechos.clone())
        );
        assert_eq!(e.celdas.len(), 19);
        for (i, (o, n)) in x.scene.objects.iter().zip(&y.scene.objects).enumerate() {
            if e.celdas.contains(&i) {
                let (b, c) = (o.primitive.bounds(), n.primitive.bounds());
                assert_eq!(b.min, c.min, "celda {i}: el fondo no se mueve");
                assert_eq!((b.max.x, b.max.z), (c.max.x, c.max.z), "celda {i}: planta");
                assert!((c.max.y - (piso + e.nivel)).abs() < EPS, "celda {i}");
                assert_eq!(
                    (
                        o.initial_material,
                        o.final_material,
                        o.spatial_group,
                        o.reveal_group
                    ),
                    (
                        n.initial_material,
                        n.final_material,
                        n.spatial_group,
                        n.reveal_group
                    )
                );
            } else {
                assert_eq!(format!("{o:?}"), format!("{n:?}"), "objeto {i}");
            }
        }
        assert_eq!(
            format!("{:?}", x.scene.palette),
            format!("{:?}", y.scene.palette)
        );
        assert_eq!(x.scene.textures.len(), y.scene.textures.len());
        for (i, (t, u)) in x.scene.textures.iter().zip(&y.scene.textures).enumerate() {
            assert_eq!(texeles(t), texeles(u), "textura {i}");
        }
        assert_eq!(format!("{:?}", x.anchors), format!("{:?}", y.anchors));
        assert_eq!(format!("{:?}", x.scale), format!("{:?}", y.scale));
        assert_eq!(
            format!("{:?}", camara_cenital(x)),
            format!("{:?}", camara_cenital(y))
        );
        assert_eq!(
            format!("{:?}", luces_del_diorama(&x.anchors, &x.scale)),
            format!("{:?}", luces_del_diorama(&y.anchors, &y.scale))
        );
        assert_eq!(
            measure_scene_radius(&y.scene, y.anchors.orbit_center),
            x.scale.scene_radius
        );
    }

    /// Un solo nivel para todas las celdas, apoyadas en el plinto, y el
    /// lecho intacto debajo.
    #[test]
    fn elevada_tiene_altura_comun_sobre_su_lecho() {
        let e = elevada();
        let s = &e.diorama.scene;
        let piso = caja(s, PLINTO).max.y;
        let techos: Vec<f32> = e.celdas.iter().map(|&i| caja(s, i).max.y).collect();
        assert!(
            techos.iter().all(|&t| (t - techos[0]).abs() < EPS),
            "{techos:?}"
        );
        for &i in &e.celdas {
            assert!((caja(s, i).min.y - (piso - EMPOTRADO)).abs() < EPS);
        }
        for &i in &e.lechos {
            let b = caja(s, i);
            assert!((b.max.y - (piso + LECHO.1)).abs() < EPS && b.max.y < techos[0]);
        }
        println!(
            "nivel {:.2} -> {:.2} sobre el plinto; agua sobre el lecho {:.2} -> {:.2}",
            e.nivel_previo,
            e.nivel,
            e.nivel_previo - LECHO.1,
            e.nivel - LECHO.1
        );
    }

    /// En 3D, no solo en planta: ninguna celda corta a otra ni a ninguna
    /// pieza salvo el plinto y su lecho. Las caídas, excepción controlada de
    /// siempre: sin contacto, a no más de `DESEMBOCADURA_INUNDADA`.
    #[test]
    fn elevada_no_corta_nada_en_volumen() {
        let e = elevada();
        let s = &e.diorama.scene;
        for (k, &i) in e.celdas.iter().enumerate() {
            let c = caja(s, i);
            for &j in &e.celdas[k + 1..] {
                assert!(!solapan(&c, &caja(s, j)), "celdas {i} y {j}");
            }
            for j in 0..s.objects.len() {
                if j == PLINTO || e.celdas.contains(&j) || e.lechos.contains(&j) {
                    continue;
                }
                let o = caja(s, j);
                assert!(!solapan(&c, &o), "la celda {i} corta el objeto {j}");
                if es_caida(s, j) {
                    assert!(separacion(&c, &o) > 0.0, "la celda {i} toca la caida {j}");
                }
            }
        }
        let a01 = *indices(s, SpatialGroupId::FlyingWaters).last().unwrap();
        for &i in &e.celdas {
            let sep = separacion(&caja(s, i), &caja(s, a01));
            assert!(sep >= 0.2, "celda {i} a {sep} de A-01");
        }
    }

    /// Los bloques vecinos siguen emergiendo, con `EMERGENCIA` de margen; se
    /// informa cuál limita el nivel y cuánto queda. El Monolito no cambia.
    #[test]
    fn elevada_deja_emerger_bloques_y_pedestal() {
        let a = azulada();
        let e = elevada();
        let s = &e.diorama.scene;
        let techo = caja(s, PLINTO).max.y + e.nivel;
        let mut limite = (f32::MAX, 0usize);
        let mut flotantes = Vec::new();
        for &i in &e.celdas {
            let c = caja(s, i);
            for j in 0..s.objects.len() {
                if j == PLINTO || e.celdas.contains(&j) || e.lechos.contains(&j) {
                    continue;
                }
                let o = caja(s, j);
                if separacion(&c, &o) > VECINDAD {
                    continue;
                }
                let margen = o.max.y - techo;
                assert!(margen >= EMERGENCIA, "el objeto {j} no emerge: {margen}");
                if margen < limite.0 {
                    limite = (margen, j);
                }
                if o.min.y > caja(s, PLINTO).max.y + EPS
                    && o.min.y < techo
                    && !flotantes.contains(&j)
                {
                    flotantes.push(j);
                }
            }
        }
        println!(
            "pieza limitante {} ({:?}), techo {:.3}, margen {:.3}; flotantes con el bajo por debajo del nivel {flotantes:?}",
            limite.1,
            s.objects[limite.1].spatial_group,
            caja(s, limite.1).max.y,
            limite.0
        );
        for i in indices(s, SpatialGroupId::Monolith) {
            assert_eq!(
                format!("{:?}", a.diorama.scene.objects[i]),
                format!("{:?}", s.objects[i])
            );
        }
    }

    /// Las desembocaduras son las de siempre: las caídas no se mueven y el
    /// frente de las tres que llegan al plinto sigue dando al agua.
    #[test]
    fn elevada_conserva_las_desembocaduras() {
        let a = azulada();
        let e = elevada();
        let s = &e.diorama.scene;
        let techo = caja(s, PLINTO).max.y + e.nivel;
        let agua = cajas_de(&e.diorama, &e.celdas);
        for i in caidas(s) {
            assert_eq!(
                format!("{:?}", a.diorama.scene.objects[i]),
                format!("{:?}", s.objects[i])
            );
            let b = caja(s, i);
            if b.min.y >= caja(s, PLINTO).max.y {
                continue;
            }
            assert!(b.min.y < techo && b.max.y > techo, "caida {i}");
            let mut tramos: Vec<(f32, f32)> = agua
                .iter()
                .filter(|r| {
                    let hueco = r[1] - b.max.z;
                    hueco > 0.0 && hueco <= DESEMBOCADURA_INUNDADA
                })
                .map(|r| (r[0], r[2]))
                .collect();
            tramos.sort_by(|p, q| p.0.total_cmp(&q.0));
            let mut hasta = b.min.x;
            for (x0, x1) in tramos {
                if x0 <= hasta + EPS {
                    hasta = hasta.max(x1);
                }
            }
            assert!(hasta >= b.max.x - EPS, "la caida {i} no desemboca entera");
        }
    }

    #[test]
    fn elevada_es_determinista_y_su_jerarquia_es_la_lineal() {
        let (a, b) = (elevada(), elevada());
        assert_eq!(
            format!("{:?}", a.diorama.scene.objects),
            format!("{:?}", b.diorama.scene.objects)
        );
        let d = &a.diorama;
        let (w, h) = (200, 150);
        for camara in [d.hero_camera(), camara_cenital(d), camara_del_monolito()] {
            for p in 0..w * h {
                let rayo = camara.ray_from_pixel(p % w, p / w, w, h);
                let mut stats = Default::default();
                assert_eq!(
                    d.accel
                        .intersect(&d.scene, &rayo, &mut stats)
                        .map(|x| x.object_index),
                    d.scene.intersect(&rayo).map(|x| x.object_index)
                );
            }
        }
    }

    /// Lo que ve el primer rayo refractado con el agua más honda, por toma,
    /// y la profundidad que recorre: el lecho, una costura entre celdas o un
    /// canto exterior. Se acota la costura —el límite del renderer, el mismo
    /// criterio que en `flooded_gaps`— y se informa lo demás.
    #[test]
    fn elevada_mide_refraccion_cantos_y_costuras() {
        use expedition33_continente_inacabado::optics::refracted_ray;

        let a = azulada();
        let e = elevada();
        let mirar =
            |d: &Blockout,
             celdas: &[usize],
             lechos: &[usize],
             camara: &expedition33_continente_inacabado::camera::Camera| {
                let agua = cajas_de(d, celdas);
                let compartida =
                    |x: f32, z: f32| agua.iter().filter(|r| dentro(r, x, z)).count() >= 2;
                let (mut n, mut lecho, mut costura, mut canto, mut prof) =
                    (0usize, 0usize, 0usize, 0usize, 0.0f32);
                for p in 0..ANCHO * ALTO {
                    let rayo = camara.ray_from_pixel(p % ANCHO, p / ANCHO, ANCHO, ALTO);
                    let mut st = Default::default();
                    let Some(hit) = d.accel.intersect(&d.scene, &rayo, &mut st) else {
                        continue;
                    };
                    if !celdas.contains(&hit.object_index) || !hit.front_face || hit.normal.y < 0.9
                    {
                        continue;
                    }
                    n += 1;
                    let ior = d
                        .scene
                        .material(d.scene.objects[hit.object_index].final_material)
                        .ior;
                    let Some(dentro) = refracted_ray(&hit, &rayo.direction, ior) else {
                        continue;
                    };
                    match d.accel.intersect(&d.scene, &dentro, &mut st) {
                        Some(h) if lechos.contains(&h.object_index) => {
                            lecho += 1;
                            prof += h.distance;
                        }
                        Some(h) if celdas.contains(&h.object_index) && h.normal.y.abs() < 0.5 => {
                            if compartida(h.point.x, h.point.z) {
                                costura += 1;
                            } else {
                                canto += 1;
                            }
                        }
                        _ => {}
                    }
                }
                let f = |v: usize| v as f32 / n.max(1) as f32;
                (
                    n,
                    f(lecho),
                    f(costura),
                    f(canto),
                    prof / lecho.max(1) as f32,
                )
            };
        for (toma, camara) in [
            ("hero", a.diorama.hero_camera()),
            ("top78", camara_cenital(&a.diorama)),
            ("detalle", camara_del_monolito()),
        ] {
            let antes = mirar(&a.diorama, &a.celdas, &a.lechos, &camara);
            let ahora = mirar(&e.diorama, &e.celdas, &e.lechos, &camara);
            println!(
                "{toma}: blue {} px, lecho {:.1} %, costura {:.2} %, canto {:.2} %, recorrido {:.3} -> raised {} px, lecho {:.1} %, costura {:.2} %, canto {:.2} %, recorrido {:.3}",
                antes.0, 100.0 * antes.1, 100.0 * antes.2, 100.0 * antes.3, antes.4,
                ahora.0, 100.0 * ahora.1, 100.0 * ahora.2, 100.0 * ahora.3, ahora.4
            );
            assert!(ahora.2 <= 0.02, "{toma}: costuras {}", ahora.2);
            assert!(ahora.4 > antes.4, "{toma}: el agua no es mas honda");
        }
    }

    /// Color y quemados del agua en el render final, por toma. Solo se exige
    /// no quemar; el color se informa para el juicio de Charlie.
    #[test]
    fn elevada_mide_color_y_quemados() {
        let a = azulada();
        let e = elevada();
        for (toma, camara) in [
            ("hero", a.diorama.hero_camera()),
            ("top78", camara_cenital(&a.diorama)),
            ("detalle", camara_del_monolito()),
        ] {
            let (antes, _, _) = color_del_agua(&a.diorama, &a.celdas, &camara);
            let (ahora, n, quemados) = color_del_agua(&e.diorama, &e.celdas, &camara);
            println!(
                "{toma}: agua blue {:?} -> raised {:?}, {n} px, quemados {quemados}",
                antes.map(|v| v.round()),
                ahora.map(|v| v.round())
            );
            assert_eq!(quemados, 0, "{toma}");
        }
    }

    // ================================================== promoción

    /// La entrega de producción con los assets reales.
    /// La cuenca aprobada de producción, con los assets reales: desde el
    /// relleno bajo Praderas, `delivery_level_con` añade además el relleno,
    /// y la cuenca aprobada es `delivery_level_previo_relleno_con`.
    fn produccion() -> Blockout {
        use expedition33_continente_inacabado::scenes::{
            delivery_level_previo_relleno_con, WaterPreset,
        };
        delivery_level_previo_relleno_con(WaterPreset::RefractiveWater, Some(&raiz()))
            .expect("assets reales")
    }

    /// La entrega con el relleno delantero, de 214: desde el relleno tras
    /// las masas, `delivery_level_con` añade además sus piezas, y esta etapa
    /// es `delivery_level_previo_trasero_con`.
    fn produccion_con_relleno() -> Blockout {
        use expedition33_continente_inacabado::scenes::{
            delivery_level_previo_trasero_con, WaterPreset,
        };
        delivery_level_previo_trasero_con(WaterPreset::RefractiveWater, Some(&raiz()))
            .expect("assets reales")
    }

    /// La entrega con los dos rellenos, de 218: desde el Monolito de carbón,
    /// `delivery_level_con` añade además su placa, y esta etapa es
    /// `delivery_level_previo_monolito_con`.
    fn produccion_con_trasero() -> Blockout {
        use expedition33_continente_inacabado::scenes::{
            delivery_level_previo_monolito_con, WaterPreset,
        };
        delivery_level_previo_monolito_con(WaterPreset::RefractiveWater, Some(&raiz()))
            .expect("assets reales")
    }

    /// La promoción: la entrega de producción es `flooded_blue_raised`
    /// exacta, con los assets reales. Geometría, lienzo y grupos objeto a
    /// objeto; materiales **efectivos** por valor —producción no arrastra los
    /// materiales sin uso de las variantes intermedias, así que los índices de
    /// la paleta no tienen por qué coincidir—; la paleta compartida y las
    /// texturas texel a texel; cielo, anclas, escala, cámaras, luces y
    /// jerarquía; y los tres renders, píxel a píxel.
    #[test]
    fn la_entrega_de_produccion_es_flooded_blue_raised() {
        use expedition33_continente_inacabado::framebuffer::Framebuffer;
        use expedition33_continente_inacabado::renderer::{render, Shading};
        use expedition33_continente_inacabado::reveal::RevealState;

        let aprobada = elevada();
        let (x, y) = (&aprobada.diorama, &produccion());
        let e = entrega();
        assert_eq!(x.scene.objects.len(), y.scene.objects.len());
        for (i, (o, n)) in x.scene.objects.iter().zip(&y.scene.objects).enumerate() {
            assert_eq!(
                format!("{:?}", o.primitive),
                format!("{:?}", n.primitive),
                "{i}"
            );
            assert_eq!(o.initial_material, n.initial_material, "{i}");
            assert_eq!(
                (o.spatial_group, o.reveal_group),
                (n.spatial_group, n.reveal_group)
            );
            assert_eq!(
                format!("{:?}", x.scene.material(o.final_material)),
                format!("{:?}", y.scene.material(n.final_material)),
                "material efectivo {i}"
            );
        }
        let compartida = e.scene.palette.len();
        for (i, (m, n)) in x.scene.palette[..compartida]
            .iter()
            .zip(&y.scene.palette[..compartida])
            .enumerate()
        {
            assert_eq!(
                format!("{m:?}"),
                format!("{n:?}"),
                "material compartido {i}"
            );
        }
        assert_eq!(x.scene.textures.len(), y.scene.textures.len());
        for (i, (t, u)) in x.scene.textures.iter().zip(&y.scene.textures).enumerate() {
            assert_eq!(
                (t.width(), t.height()),
                (u.width(), u.height()),
                "textura {i}"
            );
            assert_eq!(texeles(t), texeles(u), "textura {i}");
        }
        assert_eq!(
            format!("{:?}", x.scene.skybox),
            format!("{:?}", y.scene.skybox)
        );
        assert_eq!(format!("{:?}", x.anchors), format!("{:?}", y.anchors));
        assert_eq!(format!("{:?}", x.scale), format!("{:?}", y.scale));
        assert_eq!(x.hero_preset(), y.hero_preset());
        assert_eq!(
            format!("{:?}", x.accel),
            format!("{:?}", y.accel),
            "jerarquia"
        );
        let luces = luces_del_diorama(&x.anchors, &x.scale);
        assert_eq!(
            format!("{:?}", luces),
            format!("{:?}", luces_del_diorama(&y.anchors, &y.scale))
        );
        for (toma, cx, cy) in [
            ("hero", x.hero_camera(), y.hero_camera()),
            ("top78", camara_cenital(x), camara_cenital(y)),
            ("detalle", camara_del_monolito(), camara_del_monolito()),
        ] {
            assert_eq!(format!("{cx:?}"), format!("{cy:?}"), "{toma}");
            let pinta = |d: &Blockout, c| {
                let mut fb = Framebuffer::new(ANCHO, ALTO);
                render(
                    &mut fb,
                    &d.scene,
                    &d.accel,
                    &luces,
                    &RevealState::painted(),
                    c,
                    Shading::Material,
                );
                fb.buffer
            };
            assert!(pinta(x, &cx) == pinta(y, &cy), "{toma}: el render difiere");
        }
    }

    /// El relleno bajo Praderas: la entrega de producción de ahora es
    /// `under_meadows_fill` exacta, con los assets reales, con la misma
    /// comparación que la promoción de la cuenca.
    #[test]
    fn la_entrega_de_produccion_es_under_meadows_fill() {
        use expedition33_continente_inacabado::framebuffer::Framebuffer;
        use expedition33_continente_inacabado::renderer::{render, Shading};
        use expedition33_continente_inacabado::reveal::RevealState;

        let aprobada = rellena();
        let (x, y) = (&aprobada.diorama, &produccion_con_relleno());
        let e = entrega();
        assert_eq!(x.scene.objects.len(), y.scene.objects.len());
        for (i, (o, n)) in x.scene.objects.iter().zip(&y.scene.objects).enumerate() {
            assert_eq!(
                format!("{:?}", o.primitive),
                format!("{:?}", n.primitive),
                "{i}"
            );
            assert_eq!(o.initial_material, n.initial_material, "{i}");
            assert_eq!(
                (o.spatial_group, o.reveal_group),
                (n.spatial_group, n.reveal_group)
            );
            assert_eq!(
                format!("{:?}", x.scene.material(o.final_material)),
                format!("{:?}", y.scene.material(n.final_material)),
                "material efectivo {i}"
            );
        }
        let compartida = e.scene.palette.len();
        for (i, (m, n)) in x.scene.palette[..compartida]
            .iter()
            .zip(&y.scene.palette[..compartida])
            .enumerate()
        {
            assert_eq!(
                format!("{m:?}"),
                format!("{n:?}"),
                "material compartido {i}"
            );
        }
        assert_eq!(x.scene.textures.len(), y.scene.textures.len());
        for (i, (t, u)) in x.scene.textures.iter().zip(&y.scene.textures).enumerate() {
            assert_eq!(
                (t.width(), t.height()),
                (u.width(), u.height()),
                "textura {i}"
            );
            assert_eq!(texeles(t), texeles(u), "textura {i}");
        }
        assert_eq!(
            format!("{:?}", x.scene.skybox),
            format!("{:?}", y.scene.skybox)
        );
        assert_eq!(format!("{:?}", x.anchors), format!("{:?}", y.anchors));
        assert_eq!(format!("{:?}", x.scale), format!("{:?}", y.scale));
        assert_eq!(x.hero_preset(), y.hero_preset());
        assert_eq!(
            format!("{:?}", x.accel),
            format!("{:?}", y.accel),
            "jerarquia"
        );
        let luces = luces_del_diorama(&x.anchors, &x.scale);
        assert_eq!(
            format!("{:?}", luces),
            format!("{:?}", luces_del_diorama(&y.anchors, &y.scale))
        );
        for (toma, cx, cy) in [
            ("hero", x.hero_camera(), y.hero_camera()),
            ("top78", camara_cenital(x), camara_cenital(y)),
            ("detalle", camara_del_monolito(), camara_del_monolito()),
        ] {
            assert_eq!(format!("{cx:?}"), format!("{cy:?}"), "{toma}");
            let pinta = |d: &Blockout, c| {
                let mut fb = Framebuffer::new(ANCHO, ALTO);
                render(
                    &mut fb,
                    &d.scene,
                    &d.accel,
                    &luces,
                    &RevealState::painted(),
                    c,
                    Shading::Material,
                );
                fb.buffer
            };
            assert!(pinta(x, &cx) == pinta(y, &cy), "{toma}: el render difiere");
        }
    }

    /// La franja tras las masas: la entrega de producción de ahora es
    /// `behind_masses_fill` exacta, con los assets reales.
    #[test]
    fn la_entrega_de_produccion_es_behind_masses_fill() {
        use expedition33_continente_inacabado::framebuffer::Framebuffer;
        use expedition33_continente_inacabado::renderer::{render, Shading};
        use expedition33_continente_inacabado::reveal::RevealState;

        let aprobada = rellena_detras();
        let (x, y) = (&aprobada.diorama, &produccion_con_trasero());
        let e = entrega();
        assert_eq!(x.scene.objects.len(), y.scene.objects.len());
        for (i, (o, n)) in x.scene.objects.iter().zip(&y.scene.objects).enumerate() {
            assert_eq!(
                format!("{:?}", o.primitive),
                format!("{:?}", n.primitive),
                "{i}"
            );
            assert_eq!(o.initial_material, n.initial_material, "{i}");
            assert_eq!(
                (o.spatial_group, o.reveal_group),
                (n.spatial_group, n.reveal_group)
            );
            assert_eq!(
                format!("{:?}", x.scene.material(o.final_material)),
                format!("{:?}", y.scene.material(n.final_material)),
                "material efectivo {i}"
            );
        }
        let compartida = e.scene.palette.len();
        for (i, (m, n)) in x.scene.palette[..compartida]
            .iter()
            .zip(&y.scene.palette[..compartida])
            .enumerate()
        {
            assert_eq!(
                format!("{m:?}"),
                format!("{n:?}"),
                "material compartido {i}"
            );
        }
        assert_eq!(x.scene.textures.len(), y.scene.textures.len());
        for (i, (t, u)) in x.scene.textures.iter().zip(&y.scene.textures).enumerate() {
            assert_eq!(
                (t.width(), t.height()),
                (u.width(), u.height()),
                "textura {i}"
            );
            assert_eq!(texeles(t), texeles(u), "textura {i}");
        }
        assert_eq!(
            format!("{:?}", x.scene.skybox),
            format!("{:?}", y.scene.skybox)
        );
        assert_eq!(format!("{:?}", x.anchors), format!("{:?}", y.anchors));
        assert_eq!(format!("{:?}", x.scale), format!("{:?}", y.scale));
        assert_eq!(x.hero_preset(), y.hero_preset());
        assert_eq!(
            format!("{:?}", x.accel),
            format!("{:?}", y.accel),
            "jerarquia"
        );
        let luces = luces_del_diorama(&x.anchors, &x.scale);
        assert_eq!(
            format!("{:?}", luces),
            format!("{:?}", luces_del_diorama(&y.anchors, &y.scale))
        );
        for (toma, cx, cy) in [
            ("hero", x.hero_camera(), y.hero_camera()),
            ("top78", camara_cenital(x), camara_cenital(y)),
            ("detalle", camara_del_monolito(), camara_del_monolito()),
        ] {
            assert_eq!(format!("{cx:?}"), format!("{cy:?}"), "{toma}");
            let pinta = |d: &Blockout, c| {
                let mut fb = Framebuffer::new(ANCHO, ALTO);
                render(
                    &mut fb,
                    &d.scene,
                    &d.accel,
                    &luces,
                    &RevealState::painted(),
                    c,
                    Shading::Material,
                );
                fb.buffer
            };
            assert!(pinta(x, &cx) == pinta(y, &cy), "{toma}: el render difiere");
        }
    }

    // ============================================ under_meadows_fill

    fn rellena() -> Rellena {
        let r = under_meadows_fill(entrega());
        assert!(
            r.diorama.scene.objects.len() > r.objetos_previos,
            "sin relleno"
        );
        r
    }

    /// El relleno solo se añade: las 205 piezas de `flooded_blue_raised`,
    /// su paleta y sus texturas quedan delante intactas.
    #[test]
    fn el_relleno_solo_anade_piezas_a_flooded_blue_raised() {
        let e = elevada();
        let r = rellena();
        let (x, y) = (&e.diorama, &r.diorama);
        assert_eq!(r.objetos_previos, 205);
        assert_eq!(y.scene.objects.len(), 214, "3 de agua y 6 de lecho");
        for (i, (o, n)) in x.scene.objects.iter().zip(&y.scene.objects).enumerate() {
            assert_eq!(format!("{o:?}"), format!("{n:?}"), "objeto {i}");
        }
        for (i, (m, n)) in x.scene.palette.iter().zip(&y.scene.palette).enumerate() {
            assert_eq!(format!("{m:?}"), format!("{n:?}"), "material {i}");
        }
        for (i, (t, u)) in x.scene.textures.iter().zip(&y.scene.textures).enumerate() {
            assert_eq!(texeles(t), texeles(u), "textura {i}");
        }
        assert_eq!(y.scene.palette.len(), x.scene.palette.len() + 6);
        assert_eq!(y.scene.textures.len(), x.scene.textures.len() + 6);
        assert_eq!(format!("{:?}", x.anchors), format!("{:?}", y.anchors));
        assert_eq!(format!("{:?}", x.scale), format!("{:?}", y.scale));
    }

    #[test]
    fn el_relleno_es_determinista_y_su_jerarquia_es_la_lineal() {
        let (a, b) = (rellena(), rellena());
        assert_eq!(
            format!("{:?}", a.diorama.scene.objects),
            format!("{:?}", b.diorama.scene.objects)
        );
        let d = &a.diorama;
        let (w, h) = (200, 150);
        for camara in [
            d.hero_camera(),
            camara_cenital(d),
            camara_del_monolito(),
            camara_bajo_praderas(),
        ] {
            for p in 0..w * h {
                let rayo = camara.ray_from_pixel(p % w, p / w, w, h);
                let mut stats = Default::default();
                assert_eq!(
                    d.accel
                        .intersect(&d.scene, &rayo, &mut stats)
                        .map(|x| x.object_index),
                    d.scene.intersect(&rayo).map(|x| x.object_index)
                );
            }
        }
    }

    // ============================================ behind_masses_fill

    fn rellena_detras() -> Rellena {
        let r = behind_masses_fill(entrega());
        assert!(
            r.diorama.scene.objects.len() > r.objetos_previos,
            "sin relleno trasero"
        );
        r
    }

    /// El relleno trasero solo se añade tras las 214 de `under_meadows_fill`.
    #[test]
    fn el_relleno_trasero_solo_anade_piezas_a_under_meadows_fill() {
        let r = rellena();
        let t = rellena_detras();
        let (x, y) = (&r.diorama, &t.diorama);
        assert_eq!(t.objetos_previos, 214);
        assert_eq!(y.scene.objects.len(), 218, "2 de agua y 2 de lecho");
        for (i, (o, n)) in x.scene.objects.iter().zip(&y.scene.objects).enumerate() {
            assert_eq!(format!("{o:?}"), format!("{n:?}"), "objeto {i}");
        }
        for (i, (m, n)) in x.scene.palette.iter().zip(&y.scene.palette).enumerate() {
            assert_eq!(format!("{m:?}"), format!("{n:?}"), "material {i}");
        }
        for (i, (a, b)) in x.scene.textures.iter().zip(&y.scene.textures).enumerate() {
            assert_eq!(texeles(a), texeles(b), "textura {i}");
        }
    }

    /// Con los assets reales y el renderer de la ventana a `800 × 600`,
    /// desde la órbita trasera de la captura: el plinto seco tras las masas
    /// se veía con el relleno delantero y ya no se ve. Queda, y se cuenta
    /// aparte, el plinto **bajo** la masa de fondo que flota a `0.75`, visto
    /// por su hueco de `0.45` sobre el agua: la regla del hueco lo excluye y
    /// llenarlo es una decisión de Charlie.
    #[test]
    fn desde_la_orbita_trasera_la_franja_ya_no_se_ve_seca() {
        let antes = rellena();
        let ahora = rellena_detras();
        let seco = |d: &Blockout| {
            let s = &d.scene;
            let meseta = caja(s, indices(s, SpatialGroupId::Meadows)[0]);
            let camara = camara_trasera(d);
            let piso = caja(s, 0).max.y;
            let bajo_o_junto_a_una_pieza = |x: f32, z: f32| {
                (1..s.objects.len()).any(|j| {
                    let b = caja(s, j);
                    b.min.y < piso + NIVEL_ELEVADO + HUECO_BAJO_PRADERAS
                        && b.min.x - 0.05 < x
                        && x < b.max.x + 0.05
                        && b.min.z - 0.05 < z
                        && z < b.max.z + 0.05
                })
            };
            let (mut libre, mut bajo) = (0, 0);
            for p in 0..ANCHO * ALTO {
                let rayo = camara.ray_from_pixel(p % ANCHO, p / ANCHO, ANCHO, ALTO);
                let mut st = Default::default();
                let Some(h) = d.accel.intersect(s, &rayo, &mut st) else {
                    continue;
                };
                let (x, z) = (h.point.x, h.point.z);
                if h.object_index == 0
                    && h.normal.y > 0.9
                    && meseta.min.x < x
                    && x < meseta.max.x
                    && z < -6.0
                    && meseta.min.z < z
                    && !(-0.1..=0.1).contains(&x)
                {
                    if bajo_o_junto_a_una_pieza(x, z) {
                        bajo += 1;
                    } else {
                        libre += 1;
                    }
                }
            }
            (libre, bajo)
        };
        let ((a, a_bajo), (b, b_bajo)) = (seco(&antes.diorama), seco(&ahora.diorama));
        println!(
            "plinto seco tras las masas desde atras: {a} px -> {b} px; bajo o junto a una pieza: {a_bajo} -> {b_bajo}"
        );
        assert!(a > 2000, "{a}");
        assert_eq!(b, 0);
        assert!(b_bajo <= a_bajo, "{b_bajo} de {a_bajo}");
    }
}
