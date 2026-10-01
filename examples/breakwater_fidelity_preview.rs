//! Preview comparativo del Acantilado Rompeolas: lo que hay contra un
//! candidato de fidelidad.
//!
//! Produce tres PNG con el diorama revelado a `800 × 600` y los assets
//! reales:
//!
//! - `current.png` — el diorama tal y como lo construye el generador vigente.
//! - `candidate.png` — el mismo diorama con **solo** la lectura visual de
//!   `R-01` y `R-02` sustituida por una macroformación continua y escalonada.
//! - `spacious.png` — el candidato sobre un **territorio mayor**: plinto
//!   `22 × 19` y las tres regiones redistribuidas alrededor del Monolito,
//!   que se queda quieto de ancla.
//! - `connected.png` — la misma base de `22 × 19`, pero estirando el
//!   territorio en el eje de la cámara para que la cadena
//!   Praderas → Rompeolas → terreno continental → bahía no se rompa. El
//!   ensayo territorial había dejado el Rompeolas de isla.
//! - `refined.png` — la cadena de `connected` con el **labio costero
//!   escalonado**: los ocho bloques del borde roto se reparten entre los
//!   tramos que ya ocupaban, altos a los flancos y bajos al centro, para
//!   que la bahía se abra por el eje en vez de quedar tapiada.
//! - `asymmetric.png` — el labio de `refined` con la **boca fuera del
//!   eje**: un bloque cruza el canal y el flanco largo retrocede, de modo
//!   que el borde deja de leerse como una brecha simétrica.
//! - `separated.png` — otra premisa. Las cinco anteriores buscan una
//!   **cadena física**; esta reparte el diorama en **tres territorios sin
//!   contacto**, cada uno con su parcela sobre la misma base de `22 × 19`
//!   y el Monolito solo en el centro.
//! - `grounded.png` — la composición separada con el Rompeolas
//!   **apoyado**: una masa de `G-02` pasa a hacerle de losa y la región
//!   baja en bloque hasta posarse en ella. Corrige los prismas que
//!   flotaban sin tocar la planta aprobada.
//! - `staircase.png` — los veintiocho prismas de `R-01` dejan el macizo y
//!   se reparten en una **escalera irregular** que sube en diagonal por el
//!   flanco izquierdo, del Rompeolas hacia el borde izquierdo de Praderas.
//!   No va hacia Aguas Voladoras.
//! - `combined.png` — la mezcla de las dos anteriores: **macizo** apretado
//!   que se queda en la parcela, **ascenso** de sólo una parte de los
//!   prismas hacia Praderas, y dos **racimos laterales** que dibujan un
//!   barranco en el flanco que mira a la cámara. Descartada como ruta
//!   visual; se conserva como historial.
//! - `grounded_ascent.png` — sobre `grounded`, que es la base aprobada y no
//!   se toca: los **seis prismas más altos** de `R-01` salen de la cima y
//!   forman una extensión de avances desiguales hacia la izquierda de
//!   Praderas. El macizo se queda entero donde estaba.
//! - `backfill.png` — sobre la anterior: cinco prismas del fondo de la
//!   formación se traen a la cara **`−Z`** de la parcela, la que mira a
//!   Praderas, para darle cuerpo. Se quedan dentro de la parcela, así que
//!   ni pierden apoyo ni se acercan a la meseta. Descartada como ruta.
//! - `monolith_stair.png` — sobre `grounded`: siete prismas salen del
//!   macizo y forman una ruta de **escalones** que rodea el Monolito por su
//!   flanco y se adentra en la banda que queda detrás de él, hasta el claro
//!   de Praderas. La cota de cada escalón la pone el terreno. Descartada:
//!   no se quería rodear el Monolito.
//! - `edge_breakwater.png` — las veintiocho piezas de `R-01` ocupan **el
//!   corredor entero** entre Aguas Voladoras y el claro de Praderas, como
//!   un borde geólogico continuo de racimos irregulares. Junto al Monolito
//!   la formación baja, para no competir con él. Descartada.
//! - `green_shelf.png` — sobre `grounded`, sin mover un solo prisma: la
//!   masa verde de `G-02` que le hace de losa se **redimensiona** para
//!   prolongar su terraza por el borde. El Monolito la para antes de lo que
//!   se querría; el propio nivel explica hasta dónde y por qué.
//! - `platforms.png` — la salida a ese tope: en vez de una losa que no
//!   puede pasar del Monolito, una **cadena de tres plataformas** verdes,
//!   todas masas de `G-02` que ya existían, escalonadas por el costado
//!   oeste hasta el claro de Praderas y con el lienzo visible entre ellas.
//! - `platforms_thin.png` y `platforms_thin_top.png` — la misma cadena con
//!   las plataformas **adelgazadas en `x`**, lo transversal al corredor:
//!   misma ruta en `z`, mismos claros, mismo espesor. Van dos tomas de la
//!   misma escena: la hero y una **cenital** a `78°` de elevación, con el
//!   mismo centro, el mismo punto de mira y el mismo radio derivado.
//! - `marked_platforms.png` — la corrección a esa cenital: en vez de
//!   cornisas largas siguiendo el borde, **módulos compactos** estrechos en
//!   `x` y alargados en `z`, sueltos en el corredor oeste entre el Monolito
//!   y Praderas. Ver `_MARCA` para lo que se leyó de la marca roja. Va
//!   también en cenital —`marked_platforms_top.png`— porque la marca se
//!   hizo sobre una cenital y ahí es donde se compara.
//! - `modular_support.png` — la lectura completa de la marca: no dos
//!   módulos junto a una losa, sino **todo el soporte** del Rompeolas
//!   hecho de módulos. Las tres masas de `G-02` pasan a ser tres franjas
//!   estrechas en `x` y largas en `z`, con lienzo entre ellas, y las
//!   treinta y ocho piezas se posan en la que les toca. Descartada: no se
//!   quería reemplazar la base original.
//! - `second_island.png` — lo que sí se quería: `grounded` **intacto**, y
//!   una **segunda isla verde** grande añadida en la parcela marcada
//!   reutilizando una masa de `G-02`. Una sola pieza compacta con área
//!   útil, separada por claro de la base, del Monolito, de Praderas y de
//!   Aguas. Va también en cenital.
//! - `edge_island.png` — la misma isla, **alargada y afinada**: corrida al
//!   oeste hasta bordear el canto del lienzo, con lo que el eje `z` se
//!   libera del claro con Praderas y su fondo se multiplica. No hay una
//!   tercera isla: es la misma masa con otra caja.
//! - `edge_island_wide.png` y `edge_island_wide_top.png` — la misma isla
//!   **ensanchada hasta el tope**: crece sólo hacia `+x` y se para donde
//!   Praderas le deja exactamente un claro. Canto oeste, tramo en `z` y
//!   espesor sin tocar.
//! - `long_island_stair.png` y `long_island_stair_top.png` — la isla
//!   **alargada hasta meterse en la base** —el único solape autorizado— y
//!   con una **montaña-escalera** de seis prismas encima, apoyada sólo en
//!   ella y rematando a la mitad de la altura del Monolito.
//! - `connected_island_stair.png` y su cenital — una **terraza de
//!   transición** entre la base y la isla: otra masa de `G-02`, un escalón
//!   por encima de la cota que comparten y con la orientación cambiada,
//!   para que la secuencia sea base → transición → isla y no una L verde
//!   con junta mecánica.
//! - `clustered_stair.png` y su cenital — los seis escalones dejan de ir
//!   sueltos y se juntan en **tres racimos de dos**: el alto sobre la
//!   terraza conectora, los otros dos sobre la isla.
//! - `island_mountain.png` y su cenital — doce prismas en vez de seis y
//!   en tres zonas: **pie** denso y bajo donde la isla se pega a la base,
//!   **centro** con menos piezas y la cima en la espina, y una **salida**
//!   de dos sobre la terraza conectora.
//! - `spacious_island.png` y su cenital — el Rompeolas vuelve **exacto** a
//!   `grounded` y la isla larga se compone como Praderas en `spacious`:
//!   masa principal y dos terrazas anidadas de `G-02`, sin prismas.
//! - `second_breakwater.png` y su cenital — el **único** nivel con otro
//!   conteo, por autorización expresa: `154 - 2 + 10 = 162`. Salen las dos
//!   masas verdes de los escalones de `spacious_island` y entra una segunda
//!   formación de diez prismas con el lenguaje del primer Rompeolas.
//! - `second_breakwater_turned.png` y su cenital — la misma formación
//!   con la jerarquía dada la vuelta: la cima y el racimo miran al
//!   Rompeolas original y a la base; la masa basal, al lado libre.
//! - `second_breakwater_stair.png` y su cenital — esa orientación hecha
//!   **escalera**: cuatro bandas que ganan grosor y altura desde el extremo
//!   libre de la isla hasta el Rompeolas original.
//! - `second_breakwater_stair_dense.png` y su cenital — la misma escalera
//!   con **dieciséis** prismas (`154 - 2 + 16 = 168`), bandas `6/5/3/2`
//!   y la cima a `0.40` del Rompeolas original.
//!
//! Las dos primeras comparten cámara: entre ellas lo único que cambia son
//! treinta y cuatro piezas. La tercera y la cuarta **no pueden**
//! compartirla —la escena crece y el encuadre se deriva de lo que mide— y
//! por eso cada una usa su propia toma hero, medida sobre su propia
//! escala.
//!
//! # Qué es y qué no es
//!
//! Es un **banco de pruebas visual**, no una propuesta de código. No publica
//! ni altera el generador real de `src/scenes/breakwater.rs`: reconstruye una
//! escena en memoria a partir de la que ese generador produjo, cambia la
//! forma y la posición de treinta y cuatro objetos, y traza. El conteo del
//! nivel no se mueve —`154`—, no aparece ninguna primitiva nueva, y los
//! cuatro soportes de `R-03` quedan intactos.
//!
//! El ensayo territorial de `spacious.png` sigue la misma regla: no escala
//! nada ni añade nada, **reconstruye** cada primitiva con su tipo y su
//! tamaño en otra posición. Es una propuesta de composición para que la
//! mire una persona, no un cambio de producción.
//!
//! La intención visual que se ensaya viene de la propuesta de fidelidad:
//! una macroformación continua con cresta hacia el interior, descenso
//! escalonado hacia la bahía y racimos de grosor variable, en lugar del arco
//! de pilares separados que hay hoy. El sendero de `R-02` deja de leerse
//! como una entrada aparte y pasa a ser la cola baja de la misma formación.
//!
//! # La partición de clusters
//!
//! Este preview reconstruye la jerarquía de aceleración con `SceneAccel::
//! build`, que mete todo el grupo espacial en un solo cluster. **Eso no es
//! lo que haría el generador real**, que declara su partición en el
//! `ClusterPlan`. La diferencia no afecta a la imagen —el recorrido devuelve
//! el mismo impacto más cercano— pero sí al rendimiento, y la partición
//! buena habrá que validarla al integrar el generador de verdad. Aquí no se
//! mide tiempo por esa misma razón.
//!
//! # Uso
//!
//! ```bash
//! cargo run --release --example breakwater_fidelity_preview -- <carpeta>
//! ```
//!
//! El directorio de salida es **obligatorio**: este ejemplo no escribe
//! evidencia oficial por su cuenta.

fn main() -> std::process::ExitCode {
    imp::correr()
}

mod imp {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::ExitCode;

    use expedition33_continente_inacabado::accel::{SceneAccel, TraversalStats};
    use expedition33_continente_inacabado::bounds::Aabb;
    use expedition33_continente_inacabado::camera::Camera;
    use expedition33_continente_inacabado::cuboid::Cuboid;
    use expedition33_continente_inacabado::framebuffer::Framebuffer;
    #[cfg(feature = "hex-prism")]
    use expedition33_continente_inacabado::hex_prism::HexPrism;
    use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
    use expedition33_continente_inacabado::primitive::Primitive;
    use expedition33_continente_inacabado::renderer::{render, Shading};
    use expedition33_continente_inacabado::reveal::RevealState;
    use expedition33_continente_inacabado::scene::{Scene, SceneObject, SpatialGroupId};
    use expedition33_continente_inacabado::scene_builder::{
        derive_orbit_radius, eye_at, eye_at_yaw, measure_scene_radius, Blockout, SceneAnchors,
        SceneScale, EYE_ELEVATION_DEGREES, HALF_VERTICAL_FOV_DEGREES, HERO_YAW_DEGREES,
        LOOK_AT_HEIGHT_FRACTION, MAX_RADIUS_FACTOR, MIN_RADIUS_FACTOR,
    };
    use expedition33_continente_inacabado::scenes::{safe_level_con, WaterPreset};
    use nalgebra_glm::Vec3;

    pub const ANCHO: usize = 800;
    pub const ALTO: usize = 600;

    /// Cuántas piezas sustituye el candidato: `28` de `R-01` más `6` de
    /// `R-02`.
    const PIEZAS: usize = 34;

    /// Filas de la retícula, de la cresta hacia la costa. Suman `34`.
    ///
    /// Cinco bandas y no cuatro tramos de arco: es la clusterización que
    /// propone la fidelidad, por altura en lugar de por ángulo. Aquí solo
    /// gobierna la disposición; la partición de verdad se decide al integrar
    /// el generador real.
    const FILAS: [usize; 5] = [7, 7, 7, 7, 6];

    /// Semilla del ruido de altura. Fija, como el resto del proyecto: el
    /// preview tiene que salir igual en cada corrida o no sirve para
    /// comparar.
    const SEMILLA: u32 = 0x0A5C_1F03;

    /// Factores de grosor por racimo.
    ///
    /// `F-06` de la propuesta: las referencias tienen racimos de prismas
    /// gruesos y racimos delgados, no un reparto parejo. Se aplican por
    /// grupos de tres piezas consecutivas, que es lo que separa una
    /// formación de un cepillo de dientes.
    const RACIMOS: [f32; 6] = [1.00, 0.74, 1.22, 0.88, 1.12, 0.80];

    /// Generador congruente mínimo, determinista.
    ///
    /// `scenes::Xorshift32` es `pub(crate)`, así que un ejemplo no puede
    /// usarlo. Esto es una copia local con el mismo papel: ruido reproducible
    /// de amplitud pequeña. No pretende ser el mismo flujo de bits que el
    /// generador real, y por eso el candidato no se compara con él pieza a
    /// pieza sino por su lectura.
    struct Azar(u32);

    impl Azar {
        fn new(semilla: u32) -> Self {
            Azar(semilla.max(1))
        }

        /// Siguiente valor en `0.0..1.0`.
        fn siguiente(&mut self) -> f32 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 17;
            self.0 ^= self.0 << 5;

            (self.0 >> 8) as f32 / ((1u32 << 24) as f32)
        }

        /// Siguiente valor en `-1.0..1.0`.
        fn simetrico(&mut self) -> f32 {
            self.siguiente() * 2.0 - 1.0
        }
    }

    /// El nivel vigente, con sus texturas reales.
    pub fn nivel() -> Blockout {
        match safe_level_con(WaterPreset::RefractiveWater, Some(&PathBuf::from("."))) {
            Ok(nivel) => nivel,
            Err(e) => {
                eprintln!("error: {e}");
                eprintln!("  el preview compara la escena texturizada; sin assets seria otra.");
                eprintln!("  generalos con: cargo run --release --bin generate_assets");
                std::process::exit(1);
            }
        }
    }

    /// Índices de las piezas del Rompeolas, en el orden en que la escena las
    /// declara.
    ///
    /// El generador vigente las emite seguidas y en este orden: `R-01` los
    /// veintiocho pilares, `R-02` los seis segmentos de sendero y `R-03` los
    /// cuatro soportes. De ahí que las treinta y cuatro primeras sean
    /// exactamente lo que el candidato sustituye, y las cuatro últimas lo que
    /// conserva. Que sean contiguas lo comprueba un test.
    pub fn indices_de_rompeolas(scene: &Scene) -> Vec<usize> {
        indices_por_grupo(scene, SpatialGroupId::Breakwater)
    }

    /// Los objetos de un grupo espacial, en el orden en que la escena los
    /// declara.
    pub fn indices_por_grupo(scene: &Scene, grupo: SpatialGroupId) -> Vec<usize> {
        scene
            .objects
            .iter()
            .enumerate()
            .filter(|(_, objeto)| objeto.spatial_group == grupo)
            .map(|(i, _)| i)
            .collect()
    }

    /// La primitiva de **una** pieza de la formación.
    ///
    /// Adaptada de `scenes::breakwater::pilar`, que es privado. Conserva su
    /// contrato: en la Ruta A un prisma hexagonal cuya **anchura entre caras
    /// planas** es el lado del cuboide al que sustituye, y en la Ruta B ese
    /// cuboide. Con el circunradio la formación cambiaría de peso visual por
    /// un detalle de parametrización, y el preview dejaría de ser comparable
    /// entre rutas.
    #[cfg(not(feature = "hex-prism"))]
    fn pieza(centro: Vec3, ancho: f32, altura: f32) -> Primitive {
        Cuboid::centrado(centro, Vec3::new(ancho, altura, ancho)).into()
    }

    /// Ver la versión de la Ruta B para la explicación completa.
    #[cfg(feature = "hex-prism")]
    fn pieza(centro: Vec3, ancho: f32, altura: f32) -> Primitive {
        let apotema = ancho * 0.5;
        let radio = apotema / (std::f32::consts::PI / 6.0).cos();

        HexPrism::new(centro, radio, altura).into()
    }

    /// Caja que envuelve un conjunto de objetos de la escena.
    pub fn huella(scene: &Scene, indices: &[usize]) -> Aabb {
        let mut caja = scene.objects[indices[0]].primitive.bounds();

        for &i in &indices[1..] {
            let otra = scene.objects[i].primitive.bounds();

            caja = Aabb::new(
                Vec3::new(
                    caja.min.x.min(otra.min.x),
                    caja.min.y.min(otra.min.y),
                    caja.min.z.min(otra.min.z),
                ),
                Vec3::new(
                    caja.max.x.max(otra.max.x),
                    caja.max.y.max(otra.max.y),
                    caja.max.z.max(otra.max.z),
                ),
            );
        }

        caja
    }

    /// El nivel con la lectura de `R-01` y `R-02` sustituida por la
    /// macroformación candidata.
    ///
    /// # Qué cambia y qué no
    ///
    /// Se reemplazan **en su sitio** las treinta y cuatro primeras piezas del
    /// Rompeolas: mismo índice, mismos materiales, mismos grupos espacial y
    /// de revelación. El conteo del nivel no se mueve y no aparece ninguna
    /// primitiva nueva. Los cuatro soportes de `R-03` quedan donde estaban:
    /// la propuesta los conserva como losa de relleno bajo la formación, y
    /// ahí siguen haciendo ese papel.
    ///
    /// # La forma
    ///
    /// Una retícula hexagonal sobre la huella real de la región —medida de la
    /// escena, no escrita a mano—, con cinco filas desde el interior hasta la
    /// bahía. La altura de cada pieza es el producto de un perfil de cresta
    /// por una caída monótona hacia la costa, más un ruido pequeño de semilla
    /// fija. Con el mismo bucle salen el macizo, la ladera escalonada y el
    /// empedrado bajo que antes era `R-02`.
    ///
    /// La jerarquía de aceleración se reconstruye con `SceneAccel::build`,
    /// que no reproduce la partición del generador real. Ver la cabecera del
    /// ejemplo.
    pub fn nivel_candidato() -> Blockout {
        let mut diorama = nivel();
        let indices = indices_de_rompeolas(&diorama.scene);
        let sustituidos = &indices[..PIEZAS];
        let caja = huella(&diorama.scene, sustituidos);

        // La huella real de la región, medida. El interior queda en `z`
        // pequeña —hacia la meseta de Praderas— y la bahía en `z` grande.
        let base = caja.min.y;
        let ancho_huella = caja.max.x - caja.min.x;
        let fondo_huella = caja.max.z - caja.min.z;
        let alto_original = caja.max.y - caja.min.y;

        let mut azar = Azar::new(SEMILLA);
        let mut pieza_actual = 0usize;

        for (fila, &columnas) in FILAS.iter().enumerate() {
            // Avance hacia la costa, de `0` en la cresta a `1` en la orilla.
            let hacia_la_costa = fila as f32 / (FILAS.len() - 1) as f32;

            // Empaquetado hexagonal: las filas impares se desplazan media
            // celda. Es lo que hace que la formación se lea como un macizo
            // continuo y no como una reja de columnas alineadas.
            let paso = ancho_huella / columnas as f32;
            let desfase = if fila % 2 == 1 { paso * 0.5 } else { 0.0 };

            for columna in 0..columnas {
                let x = caja.min.x + desfase + (columna as f32 + 0.5) * paso;
                let z = caja.min.z + (hacia_la_costa * 0.92 + 0.04) * fondo_huella;

                // Perfil de cresta: el macizo pesa en el centro y afina en
                // los extremos, como una formación que nace de un punto.
                let centrado = (x - (caja.min.x + ancho_huella * 0.5)) / (ancho_huella * 0.5);
                let cresta = 1.0 - 0.45 * centrado * centrado;

                // Caída monótona hacia la bahía. No lineal: los dos primeros
                // escalones bajan poco y el último se hunde, que es lo que
                // produce la ladera escalonada en vez de una rampa.
                let caida = 1.0 - 0.78 * hacia_la_costa.powf(1.6);

                let ruido = 0.10 * azar.simetrico();
                let altura = (alto_original * (cresta * caida + ruido)).max(0.12);

                // Grosor por racimos, no parejo. Ver `RACIMOS`.
                let ancho = paso * 0.86 * RACIMOS[(pieza_actual / 3) % RACIMOS.len()];

                let centro = Vec3::new(x, base + altura * 0.5, z);
                let objeto = &mut diorama.scene.objects[sustituidos[pieza_actual]];

                objeto.primitive = pieza(centro, ancho, altura);
                pieza_actual += 1;
            }
        }

        debug_assert_eq!(pieza_actual, PIEZAS, "FILAS tiene que sumar PIEZAS");

        // La escena cambió de forma, así que la jerarquía anterior ya no
        // describe sus cajas. Se reconstruye entera: el conteo y los índices
        // no se movieron, que es lo que exige el invariante de `Scene`.
        diorama.accel =
            SceneAccel::build(&diorama.scene).expect("la escena del candidato tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // El ensayo territorial
    // ------------------------------------------------------------------

    /// Planta del plinto `G-01` vigente.
    const PLINTO_ACTUAL_X: f32 = 17.0;
    const PLINTO_ACTUAL_Z: f32 = 15.0;

    /// Planta del plinto en el ensayo territorial, tal y como la pidió la
    /// supervisión.
    const PLINTO_ESPACIOSO_X: f32 = 22.0;
    const PLINTO_ESPACIOSO_Z: f32 = 19.0;

    /// Suelo nuevo por lado, en cada eje de la planta.
    ///
    /// Todos los desplazamientos de región salen de aquí y no de números
    /// sueltos: si mañana el plinto cambia de medida, la redistribución se
    /// mueve con él en lugar de quedarse descuadrada. Con `17 → 22` y
    /// `15 → 19` valen `2.5` y `2.0`.
    const HOLGURA_X: f32 = (PLINTO_ESPACIOSO_X - PLINTO_ACTUAL_X) * 0.5;
    const HOLGURA_Z: f32 = (PLINTO_ESPACIOSO_Z - PLINTO_ACTUAL_Z) * 0.5;

    /// Orientación del encuadre, para leer las direcciones sin adivinar.
    ///
    /// La toma hero mira con `yaw = 90°`: el ojo está en `+Z` y mira hacia
    /// el origen. De ahí salen las tres direcciones del encargo —**frente**
    /// es `+Z`, **atrás** es `−Z` e **izquierda** es `−X`—, y no de una
    /// intuición sobre los ejes.
    ///
    /// Praderas retrocede y sube: se despega del Monolito hacia el fondo y
    /// gana altura de meseta.
    fn desplazamiento_praderas() -> Vec3 {
        Vec3::new(0.0, HOLGURA_Z * 0.5, -HOLGURA_Z)
    }

    /// El Rompeolas se abre hacia la izquierda y hacia el frente: ocupa el
    /// suelo nuevo del flanco en lugar de amontonarse bajo la meseta.
    fn desplazamiento_rompeolas() -> Vec3 {
        Vec3::new(-HOLGURA_X, 0.0, HOLGURA_Z * 0.5)
    }

    /// Aguas Voladoras gana fondo frontal: la bahía se aleja de la costa
    /// hacia la cámara, que es donde el plinto creció.
    pub fn desplazamiento_aguas() -> Vec3 {
        Vec3::new(0.0, 0.0, HOLGURA_Z)
    }

    /// La misma primitiva, movida.
    ///
    /// No hay API de traslación en el proyecto y este ejemplo no va a
    /// inventar una: cada forma se **reconstruye** con su propio
    /// constructor a partir de lo que su caja envolvente mide, así que
    /// conserva tipo y tamaño por construcción. La Ruta A devuelve
    /// `HexPrism` y la Ruta B `Cuboid`, cada una la suya.
    ///
    /// Para el prisma, el circunradio se lee del eje `Z` y no del `X`: con
    /// la arista mirando a `+X`, el alcance en `X` es la **apotema**.
    /// Confundirlos engordaría la pieza un `15 %` en cada traslación.
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

                HexPrism::new(centro, radio, altura).into()
            }
        }
    }

    /// El candidato sobre un territorio mayor.
    ///
    /// # Qué cambia
    ///
    /// Parte de `nivel_candidato` —la macroformación del Rompeolas ya
    /// está— y sólo redistribuye **espacio**:
    ///
    /// - `G-01` pasa de `17 × 15` a `22 × 19`. Crece en planta, no en
    ///   espesor ni de sitio: es el suelo, y moverlo arrastraría todo.
    /// - Praderas retrocede y sube, el Rompeolas se abre a izquierda y
    ///   frente, y Aguas Voladoras gana fondo frontal.
    ///
    /// # Qué no cambia
    ///
    /// El Monolito no se toca: es el ancla visual contra la que se mide
    /// todo lo demás, y su altura sigue alimentando el encuadre. El arco
    /// costero de `G-02` tampoco, porque el encargo enumera lo que se
    /// mueve y el fondo no está en esa lista. El conteo sigue en `154`, no
    /// aparece ninguna primitiva nueva y nada se escala: cada pieza se
    /// reconstruye con su tipo y su tamaño, sólo que en otro sitio.
    ///
    /// # La escala se vuelve a medir
    ///
    /// Un territorio mayor no puede quedarse con el radio del anterior.
    /// `scene_radius` se mide de nuevo sobre la geometría movida con
    /// `measure_scene_radius`, `orbit_radius` sale de `derive_orbit_radius`
    /// y el ojo de `eye_at_yaw`: las tres son las funciones reales del
    /// proyecto, no una aritmética paralela. Las anclas de región se
    /// desplazan con su geometría para que las luces —que se arman contra
    /// estas anclas y esta escala— sigan apuntando a lo que iluminaban.
    ///
    /// La jerarquía de aceleración se reconstruye con `SceneAccel::build`,
    /// con la misma salvedad de partición que explica la cabecera.
    ///
    /// **La composición exacta es candidata para revisión humana.** Los
    /// desplazamientos son deterministas y derivados del plinto, pero que
    /// esta repartición se lea mejor que la vigente lo decide quien mire.
    pub fn nivel_espacioso() -> Blockout {
        let mut diorama = nivel_candidato();

        // `G-01` · el suelo crece en planta, alrededor de su mismo centro.
        for objeto in &mut diorama.scene.objects {
            if objeto.spatial_group != SpatialGroupId::Global {
                continue;
            }

            let caja = objeto.primitive.bounds();
            let centro = (caja.min + caja.max) * 0.5;
            let espesor = caja.max.y - caja.min.y;

            objeto.primitive = Cuboid::centrado(
                centro,
                Vec3::new(PLINTO_ESPACIOSO_X, espesor, PLINTO_ESPACIOSO_Z),
            )
            .into();
        }

        // Las tres regiones, cada una en bloque: un solo vector por región,
        // aplicado a todas sus piezas. Eso es lo que hace que la región
        // siga siendo la misma y sólo esté en otro sitio.
        let reparto = [
            (SpatialGroupId::Meadows, desplazamiento_praderas()),
            (SpatialGroupId::Breakwater, desplazamiento_rompeolas()),
            (SpatialGroupId::FlyingWaters, desplazamiento_aguas()),
        ];

        for (grupo, delta) in reparto {
            for objeto in &mut diorama.scene.objects {
                if objeto.spatial_group == grupo {
                    objeto.primitive = trasladada(&objeto.primitive, delta);
                }
            }
        }

        // Medición: primero la geometría, después la escala. El mismo orden
        // que sigue el generador real.
        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);
        let aguas = desplazamiento_aguas();

        diorama.anchors = SceneAnchors {
            meadows_anchor: diorama.anchors.meadows_anchor + desplazamiento_praderas(),
            breakwater_anchor: diorama.anchors.breakwater_anchor + desplazamiento_rompeolas(),
            // El ancla de Aguas ya viene elevada a la superficie del agua;
            // el desplazamiento no toca la altura, así que sigue ahí.
            flying_waters_anchor: diorama.anchors.flying_waters_anchor + aguas,
            // `L-02` apunta al barco. Si el barco se va y el ancla se queda,
            // la luz ilumina bahía vacía.
            boat_anchor: diorama.anchors.boat_anchor + aguas,
            broken_edge_anchor: diorama.anchors.broken_edge_anchor + aguas,
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            monolith_height,
            water_surface_y: diorama.scale.water_surface_y + aguas.y,
            orbit_radius,
        };

        diorama.accel =
            SceneAccel::build(&diorama.scene).expect("la escena espaciosa tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // La cadena reconstruida
    // ------------------------------------------------------------------

    /// Praderas retrocede **media holgura**.
    ///
    /// Media y no entera: con `−2.0` la meseta quedaba a ras del borde
    /// trasero del plinto nuevo, y una meseta que termina exactamente donde
    /// termina el suelo se lee como un corte, no como un fondo. Con `−1.0`
    /// quedan `1.0` de plinto por detrás.
    pub fn desplazamiento_praderas_conectado() -> Vec3 {
        Vec3::new(0.0, 0.0, -HOLGURA_Z * 0.5)
    }

    /// El Rompeolas acompaña a Praderas **con el mismo vector**.
    ///
    /// Esto es la corrección de fondo respecto del ensayo territorial. Allí
    /// cada región recibió un desplazamiento propio, y el Rompeolas se
    /// apartó `2.5` a la izquierda de la meseta que sostenía: quedó de
    /// isla. Moviendo los dos juntos, la relación de apoyo se conserva
    /// **por construcción** y no por suerte: la cresta sigue exactamente
    /// bajo el mismo punto de Praderas que antes.
    pub fn desplazamiento_rompeolas_conectado() -> Vec3 {
        desplazamiento_praderas_conectado()
    }

    /// Las terrazas de `G-02` avanzan media holgura hacia la bahía.
    ///
    /// La mitad de lo que avanza Aguas. Ese medio paso es lo que convierte
    /// el hueco que deja la bahía al abrirse en una **pendiente**: si las
    /// terrazas avanzaran lo mismo que el agua, el hueco se trasladaría
    /// entero y no se habría rellenado nada.
    pub fn desplazamiento_terrazas_conectado() -> Vec3 {
        Vec3::new(0.0, 0.0, HOLGURA_Z * 0.5)
    }

    /// La pieza más alta del Rompeolas: su cresta.
    ///
    /// Se mide, no se nombra por índice. La macroformación del candidato
    /// pone la cresta en la fila del interior, pero el día que esa fila
    /// cambie de sitio esta función seguirá encontrándola. El empate se
    /// rompe por el primer índice, así que el resultado es determinista.
    pub fn cresta_del_rompeolas(scene: &Scene) -> Aabb {
        let indices = indices_por_grupo(scene, SpatialGroupId::Breakwater);
        let mut mejor = scene.objects[indices[0]].primitive.bounds();

        for &i in &indices[1..] {
            let caja = scene.objects[i].primitive.bounds();

            if caja.max.y > mejor.max.y {
                mejor = caja;
            }
        }

        mejor
    }

    /// Las masas de `G-02` que entran en la cadena de terrazas.
    ///
    /// # La regla
    ///
    /// Una masa del arco costero se suma a la cadena si cumple las dos
    /// cosas, medidas sobre sus cajas:
    ///
    /// 1. **Comparte flanco con el Rompeolas**: su huella en `x` solapa con
    ///    la del Rompeolas. Lo que está al otro lado del diorama no forma
    ///    parte de esta ladera y no tiene por qué moverse.
    /// 2. **Queda por delante de la cresta**: su centro en `z` está más
    ///    cerca de la bahía que la cresta del Rompeolas. Es la costa, no la
    ///    cordillera del fondo.
    ///
    /// Lo que no cumple las dos se queda **exactamente** donde estaba. Esa
    /// es la diferencia con el ensayo territorial, donde `G-02` no se movió
    /// en absoluto y la ladera se quedó sin continuación; y con la
    /// tentación opuesta, trasladar el arco entero, que arrastraría también
    /// el telón de fondo de detrás de Praderas.
    ///
    /// Con la escena vigente la regla elige cuatro de las diez. Cuáles, lo
    /// imprime el propio ejemplo: no hay una lista de índices escrita a
    /// mano en ninguna parte.
    pub fn masas_de_terraza(scene: &Scene) -> Vec<usize> {
        let rompeolas = huella(scene, &indices_por_grupo(scene, SpatialGroupId::Breakwater));
        let cresta = cresta_del_rompeolas(scene);
        let z_de_la_cresta = (cresta.min.z + cresta.max.z) * 0.5;

        indices_por_grupo(scene, SpatialGroupId::ContinentBackground)
            .into_iter()
            .filter(|&i| {
                let caja = scene.objects[i].primitive.bounds();

                let comparte_flanco = caja.min.x < rompeolas.max.x && caja.max.x > rompeolas.min.x;
                let al_frente_de_la_cresta = (caja.min.z + caja.max.z) * 0.5 > z_de_la_cresta;

                comparte_flanco && al_frente_de_la_cresta
            })
            .collect()
    }

    /// El candidato sobre el territorio de `22 × 19`, con la cadena
    /// Praderas → Rompeolas → terreno continental → bahía reconstruida.
    ///
    /// # Qué cambia respecto del ensayo territorial
    ///
    /// El ensayo territorial acertó la base y falló la continuidad: dio a
    /// cada región un desplazamiento propio y dejó `G-02` quieto, así que
    /// el Rompeolas se apartó de la meseta que sostiene y de la costa que
    /// lo continúa. Aquí el territorio se estira **en el eje de la cámara**
    /// y en pasos de media holgura, de modo que lo que estaba encadenado
    /// siga encadenado:
    ///
    /// | Pieza | Desplazamiento |
    /// |---|---|
    /// | `G-01` plinto | crece a `22 × 19` sobre su mismo centro |
    /// | Monolito | `0` — es el ancla |
    /// | Praderas | `−1.0` en `z` (media holgura hacia atrás) |
    /// | Rompeolas | `−1.0` en `z`, **el mismo vector que Praderas** |
    /// | `G-02`, las masas de terraza | `+1.0` en `z` |
    /// | `G-02`, el resto | `0` — telón de fondo |
    /// | Aguas Voladoras | `+2.0` en `z` (se abre al frente) |
    ///
    /// Ninguno tiene componente en `x` ni en `y`: la continuidad se
    /// reconstruye separando las cosas a lo largo de la profundidad, que es
    /// donde el plinto creció y donde la cámara puede leerlo.
    ///
    /// # Qué no cambia
    ///
    /// La macroformación de `R-01`/`R-02` es la del candidato, intacta. El
    /// Monolito no se toca. El conteo sigue en `154`, no aparece ninguna
    /// primitiva nueva, nada se escala y no hay overlays: cada pieza se
    /// reconstruye con su tipo y su tamaño en otro sitio.
    ///
    /// # La escala se vuelve a medir
    ///
    /// Igual que en el ensayo territorial: `measure_scene_radius`,
    /// `derive_orbit_radius` y `eye_at_yaw` sobre la geometría ya movida, y
    /// las anclas de las regiones que de verdad se movieron siguen a su
    /// geometría para que las luces apunten a donde están las cosas.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_conectado() -> Blockout {
        let mut diorama = nivel_candidato();

        // La selección se decide **antes** de mover nada: la regla mide la
        // escena candidata, que es la misma que miran los tests.
        let terrazas = masas_de_terraza(&diorama.scene);

        let praderas = desplazamiento_praderas_conectado();
        let rompeolas = desplazamiento_rompeolas_conectado();
        let aguas = desplazamiento_aguas();
        let terraza = desplazamiento_terrazas_conectado();

        for objeto in &mut diorama.scene.objects {
            // `G-01` · el suelo crece en planta, alrededor de su centro.
            if objeto.spatial_group == SpatialGroupId::Global {
                let caja = objeto.primitive.bounds();
                let centro = (caja.min + caja.max) * 0.5;
                let espesor = caja.max.y - caja.min.y;

                objeto.primitive = Cuboid::centrado(
                    centro,
                    Vec3::new(PLINTO_ESPACIOSO_X, espesor, PLINTO_ESPACIOSO_Z),
                )
                .into();

                continue;
            }

            let delta = match objeto.spatial_group {
                SpatialGroupId::Meadows => praderas,
                SpatialGroupId::Breakwater => rompeolas,
                SpatialGroupId::FlyingWaters => aguas,
                // El Monolito, el arco costero y todo lo demás se tratan
                // aparte o no se tratan.
                _ => continue,
            };

            objeto.primitive = trasladada(&objeto.primitive, delta);
        }

        // Las terrazas, una por una y por índice: es el único sitio del
        // preview donde el reparto no es «toda la región».
        for &i in &terrazas {
            let objeto = &mut diorama.scene.objects[i];

            objeto.primitive = trasladada(&objeto.primitive, terraza);
        }

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            meadows_anchor: diorama.anchors.meadows_anchor + praderas,
            breakwater_anchor: diorama.anchors.breakwater_anchor + rompeolas,
            flying_waters_anchor: diorama.anchors.flying_waters_anchor + aguas,
            boat_anchor: diorama.anchors.boat_anchor + aguas,
            broken_edge_anchor: diorama.anchors.broken_edge_anchor + aguas,
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            // `G-02` no tiene ancla propia: las terrazas se mueven como
            // geometría y nada más apunta a ellas.
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            monolith_height,
            water_surface_y: diorama.scale.water_surface_y + aguas.y,
            orbit_radius,
        };

        diorama.accel =
            SceneAccel::build(&diorama.scene).expect("la escena conectada tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // El labio costero
    // ------------------------------------------------------------------

    /// Columnas y filas de la rejilla con la que se mide cuánta lámina de
    /// agua se ve. No es la resolución de la imagen: es un muestreo del
    /// mismo encuadre, suficientemente fino para distinguir una franja de
    /// una astilla y suficientemente barato para un test.
    const MUESTRAS_X: usize = 100;
    const MUESTRAS_Y: usize = 75;

    /// El volumen de agua `A-01`, por su índice en la escena.
    ///
    /// Se localiza **midiendo**: es la pieza de Aguas Voladoras con la caja
    /// más voluminosa, y por un factor grande —`8.6 × 2.3 × 5.0` contra los
    /// `9.0 × 0.8 × 5.4` de la masa mayor del lecho—. Un test comprueba que
    /// ninguna otra se le acerca, que es lo que convierte la regla en una
    /// identificación y no en una corazonada.
    pub fn volumen_de_agua(scene: &Scene) -> usize {
        let volumen = |i: usize| {
            let caja = scene.objects[i].primitive.bounds();

            (caja.max.x - caja.min.x) * (caja.max.y - caja.min.y) * (caja.max.z - caja.min.z)
        };

        indices_por_grupo(scene, SpatialGroupId::FlyingWaters)
            .into_iter()
            .fold(None, |mejor: Option<usize>, i| match mejor {
                Some(m) if volumen(m) >= volumen(i) => Some(m),
                _ => Some(i),
            })
            .expect("Aguas Voladoras tiene piezas")
    }

    /// Los ocho bloques de `A-11`, el borde roto.
    ///
    /// El generador los emite **justo antes** del volumen, y el volumen va
    /// el último de la región para que su presencia no desplace los índices
    /// de lo que hay dentro: está escrito así en `flying_waters.rs`. De ahí
    /// que sean los ocho anteriores a `A-01`.
    ///
    /// La identificación no se queda en el orden: un test comprueba que los
    /// ocho comparten fondo y base, que es lo que los hace una fila y lo
    /// que delataría un reordenado del generador.
    pub fn labio_costero(scene: &Scene) -> Vec<usize> {
        let a01 = volumen_de_agua(scene);
        let aguas = indices_por_grupo(scene, SpatialGroupId::FlyingWaters);
        let posicion = aguas
            .iter()
            .position(|&i| i == a01)
            .expect("A-01 es de Aguas Voladoras");

        aguas[posicion - LABIO_COSTERO..posicion].to_vec()
    }

    /// Cuántos bloques tiene el borde roto en el nivel seguro.
    const LABIO_COSTERO: usize = 8;

    /// Las dos cotas del labio, releídas de la escena.
    ///
    /// Devuelve `(exteriores, centrales)`: los cuatro bloques de los tramos
    /// de los flancos y los cuatro de los tramos del centro, separados por
    /// la distancia de su centro al centro de la fila. Es la misma partición
    /// que aplica `nivel_refinado`, pero **medida sobre el resultado**, así
    /// que un test puede comprobar el escalón sin creerse la construcción.
    pub fn cotas_del_labio(scene: &Scene) -> (Vec<usize>, Vec<usize>) {
        let labio = labio_costero(scene);
        let fila = huella(scene, &labio);
        let centro_x = (fila.min.x + fila.max.x) * 0.5;

        let distancia = |i: usize| {
            let caja = scene.objects[i].primitive.bounds();

            ((caja.min.x + caja.max.x) * 0.5 - centro_x).abs()
        };

        let mut ordenados = labio.clone();
        ordenados.sort_by(|a, b| {
            distancia(*a)
                .partial_cmp(&distancia(*b))
                .expect("no hay NaN en la fila")
        });

        let mitad = LABIO_COSTERO / 2;
        let centrales = ordenados[..mitad].to_vec();
        let mut exteriores = ordenados[mitad..].to_vec();

        exteriores.sort_unstable();

        let mut centrales_ordenados = centrales;
        centrales_ordenados.sort_unstable();

        (exteriores, centrales_ordenados)
    }

    /// Cuánta lámina de agua se ve desde una cámara, y si sale de una pieza.
    #[derive(Debug, Clone, Copy)]
    pub struct VistaDelAgua {
        /// Celdas muestreadas en total.
        pub muestras: usize,
        /// Celdas en las que lo primero que toca el rayo es la **cara de
        /// arriba** de `A-01`.
        pub visibles: usize,
        /// Celdas de la mancha conexa más grande, por vecindad de cuatro.
        pub mayor_mancha: usize,
    }

    /// Mide la lámina de agua visible desde una cámara.
    ///
    /// Lanza el **mismo rayo primario** que lanzaría el renderer —
    /// `Camera::ray_from_pixel` y `SceneAccel::intersect`— sobre una rejilla
    /// del encuadre. No dibuja nada y no sombrea: sólo pregunta qué es lo
    /// primero que hay en cada dirección.
    ///
    /// Una celda cuenta como agua sólo si el impacto es de `A-01` **y** su
    /// normal mira hacia arriba. Ver la cara frontal del volumen de canto no
    /// es ver agua: es ver una losa oscura, que es precisamente la lectura
    /// que hay que corregir.
    pub fn vista_del_agua(diorama: &Blockout, camara: &Camera) -> VistaDelAgua {
        let a01 = volumen_de_agua(&diorama.scene);
        let mut celdas = vec![false; MUESTRAS_X * MUESTRAS_Y];
        let mut stats = TraversalStats::default();

        for fila in 0..MUESTRAS_Y {
            for columna in 0..MUESTRAS_X {
                let x = columna * ANCHO / MUESTRAS_X + ANCHO / (2 * MUESTRAS_X);
                let y = fila * ALTO / MUESTRAS_Y + ALTO / (2 * MUESTRAS_Y);

                let rayo = camara.ray_from_pixel(x, y, ANCHO, ALTO);

                if let Some(impacto) = diorama.accel.intersect(&diorama.scene, &rayo, &mut stats) {
                    celdas[fila * MUESTRAS_X + columna] =
                        impacto.object_index == a01 && impacto.normal.y > 0.5;
                }
            }
        }

        let visibles = celdas.iter().filter(|c| **c).count();

        // Mancha conexa mayor, por vecindad de cuatro. Una región de agua
        // partida en trozos por los bloques del labio da muchas manchas
        // pequeñas; una franja abierta da una sola grande.
        let mut vistas = vec![false; celdas.len()];
        let mut mayor_mancha = 0usize;
        let mut pila: Vec<usize> = Vec::new();

        for inicio in 0..celdas.len() {
            if !celdas[inicio] || vistas[inicio] {
                continue;
            }

            let mut tamano = 0usize;
            pila.push(inicio);
            vistas[inicio] = true;

            while let Some(celda) = pila.pop() {
                tamano += 1;

                let columna = celda % MUESTRAS_X;
                let fila = celda / MUESTRAS_X;

                let mut visitar = |c: usize, f: usize, pila: &mut Vec<usize>| {
                    let vecina = f * MUESTRAS_X + c;

                    if celdas[vecina] && !vistas[vecina] {
                        vistas[vecina] = true;
                        pila.push(vecina);
                    }
                };

                if columna > 0 {
                    visitar(columna - 1, fila, &mut pila);
                }
                if columna + 1 < MUESTRAS_X {
                    visitar(columna + 1, fila, &mut pila);
                }
                if fila > 0 {
                    visitar(columna, fila - 1, &mut pila);
                }
                if fila + 1 < MUESTRAS_Y {
                    visitar(columna, fila + 1, &mut pila);
                }
            }

            mayor_mancha = mayor_mancha.max(tamano);
        }

        VistaDelAgua {
            muestras: celdas.len(),
            visibles,
            mayor_mancha,
        }
    }

    /// La cadena territorial con el labio costero escalonado.
    ///
    /// # Qué corrige
    ///
    /// La revisión de `connected` encontró que el borde de la bahía se leía
    /// como un muro negro continuo —una cavidad— y que el agua no se leía
    /// como región acuática. Las dos cosas salen del mismo sitio: los ocho
    /// bloques de `A-11` forman **una sola fila** de basalto mojado a la
    /// altura justa para tapar la lámina, con los ocho tramos ocupados y
    /// sin un hueco por el que mirar la bahía.
    ///
    /// # Qué hace
    ///
    /// **La fila se parte por la mitad y las dos mitades se apartan un
    /// tramo hacia fuera.** En el eje de la toma hero queda una puerta, y
    /// por esa puerta se ve la lámina hasta el fondo de la bahía.
    ///
    /// No hay piezas nuevas, ni tamaños distintos, ni materiales, ni
    /// overlays: los mismos ocho bloques, en otras abscisas de su propia
    /// fila.
    ///
    /// | Grupo | A dónde va | Paso en `z` |
    /// |---|---|---|
    /// | los cuatro **más altos** | a los cuatro tramos **más exteriores** | `−0.25 × fondo` |
    /// | los cuatro **más bajos** | a los cuatro tramos de dentro | `−0.75 × fondo` |
    ///
    /// El reparto es por techo y por lejanía al centro a la vez: el bloque
    /// más alto va al tramo más exterior, el siguiente al siguiente, y así.
    /// El labio **desciende hacia la puerta** en altura y retrocede en
    /// profundidad: dos cotas, y la baja junto al hueco.
    ///
    /// # Por qué apartarse y no sólo reordenarse
    ///
    /// La primera versión de este corte intercambiaba los bloques entre los
    /// ocho tramos que ya ocupaban. La medición de lámina visible dio
    /// **exactamente la misma cifra** que `connected`, y con razón: en
    /// planta los ocho tramos seguían ocupados, así que la superficie de
    /// agua tapada era la misma pieza a pieza. Mover una fila dentro de sí
    /// misma no abre nada. Lo que abre es vaciar tramos.
    ///
    /// El retranqueo global existe por una razón medible además de visual:
    /// en `connected` la fila dejaba exactamente `0.10` hasta el borde del
    /// plinto, y eso es el mínimo, no un margen.
    ///
    /// # Qué no toca
    ///
    /// Nada más. `A-01` sigue siendo **un solo volumen** en el mismo sitio
    /// —partirlo costaría niveles de recursión y el interior de la bahía
    /// terminaría en cielo, que es lo que explica `flying_waters.rs`—, el
    /// Monolito no se mueve, el plinto sigue en `22 × 19`, la cadena
    /// territorial es la de `connected` y el conteo sigue en `154`.
    ///
    /// El desequilibrio de densidad entre la izquierda y la derecha que
    /// también señaló la revisión **queda sin tocar**: llenar el vacío de la
    /// derecha estaba excluido del encargo, y moverle masa desde la
    /// izquierda rompería la cadena que este mismo preview acaba de
    /// reconstruir.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_refinado() -> Blockout {
        let mut diorama = nivel_conectado();

        let labio = labio_costero(&diorama.scene);
        let cajas: Vec<Aabb> = labio
            .iter()
            .map(|&i| diorama.scene.objects[i].primitive.bounds())
            .collect();

        let centro_x_de = |k: usize| (cajas[k].min.x + cajas[k].max.x) * 0.5;

        // El fondo de los propios bloques da la unidad del paso: el labio se
        // escalona en múltiplos de su propia pieza, no de un número suelto.
        let fondo = cajas[0].max.z - cajas[0].min.z;

        // Las ocho abscisas de la fila, y su paso.
        let mut tramos: Vec<f32> = (0..labio.len()).map(centro_x_de).collect();
        tramos.sort_by(|a, b| a.partial_cmp(b).expect("no hay NaN en la fila"));

        let mitad = labio.len() / 2;
        let paso = (tramos[tramos.len() - 1] - tramos[0]) / (tramos.len() - 1) as f32;

        // La fila se parte por la mitad y cada mitad se aparta un tramo
        // hacia fuera. Los dos tramos del centro quedan vacíos: eso es la
        // puerta, y es lo único que de verdad destapa lámina de agua.
        let tramos: Vec<f32> = tramos
            .iter()
            .enumerate()
            .map(|(k, x)| if k < mitad { x - paso } else { x + paso })
            .collect();

        let centro_de_la_fila = (tramos[0] + tramos[tramos.len() - 1]) * 0.5;

        // Tramos de fuera hacia dentro. Los empates de un par simétrico los
        // rompe el orden ascendente en `x`, que la ordenación estable
        // conserva: el reparto es determinista.
        let mut por_lejania: Vec<usize> = (0..tramos.len()).collect();
        por_lejania.sort_by(|a, b| {
            (tramos[*b] - centro_de_la_fila)
                .abs()
                .partial_cmp(&(tramos[*a] - centro_de_la_fila).abs())
                .expect("no hay NaN en la fila")
        });

        // Bloques de más alto a más bajo, empate por índice.
        let mut por_altura: Vec<usize> = (0..labio.len()).collect();
        por_altura.sort_by(|a, b| {
            cajas[*b]
                .max
                .y
                .partial_cmp(&cajas[*a].max.y)
                .expect("no hay NaN en la fila")
        });

        let retranqueo = -fondo * 0.25;

        for (rango, (&bloque, &tramo)) in por_altura.iter().zip(&por_lejania).enumerate() {
            // Las cuatro más altas se quedan al frente y enmarcan; las
            // cuatro de dentro retroceden medio fondo y bajan hacia la
            // puerta. El labio se lee como un escalón, no como un muro.
            let z = if rango < mitad {
                retranqueo
            } else {
                retranqueo - fondo * 0.5
            };

            let delta = Vec3::new(tramos[tramo] - centro_x_de(bloque), 0.0, z);
            let objeto = &mut diorama.scene.objects[labio[bloque]];

            objeto.primitive = trasladada(&objeto.primitive, delta);
        }

        // Ninguna ancla de región se movió: el labio se reordena **dentro**
        // de Aguas Voladoras y el ancla de la bahía sigue donde estaba. Aun
        // así la escala se vuelve a medir, porque la geometría cambió.
        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel =
            SceneAccel::build(&diorama.scene).expect("la escena refinada tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // La boca asimétrica
    // ------------------------------------------------------------------

    /// La boca costera: el mayor hueco de la fila del labio y lo que queda a
    /// cada lado.
    #[derive(Debug, Clone)]
    pub struct BocaCostera {
        /// Bloques a la izquierda del hueco, de fuera hacia dentro.
        pub izquierda: Vec<usize>,
        /// Bloques a la derecha del hueco.
        pub derecha: Vec<usize>,
        /// Las dos abscisas libres entre los dos flancos.
        pub hueco: (f32, f32),
    }

    impl BocaCostera {
        /// Anchura del canal libre.
        pub fn ancho(&self) -> f32 {
            self.hueco.1 - self.hueco.0
        }

        /// Abscisa del centro del canal.
        pub fn centro(&self) -> f32 {
            (self.hueco.0 + self.hueco.1) * 0.5
        }
    }

    /// Localiza la boca **midiendo**: el mayor hueco libre entre dos bloques
    /// consecutivos de la fila del labio.
    ///
    /// No se le dice dónde está. Eso es lo que permite que un test compare
    /// la boca de `refined` con la de `asymmetric` sin creerse ninguna de
    /// las dos construcciones.
    pub fn boca_costera(scene: &Scene) -> BocaCostera {
        let labio = labio_costero(scene);
        let mut ordenados: Vec<(usize, Aabb)> = labio
            .iter()
            .map(|&i| (i, scene.objects[i].primitive.bounds()))
            .collect();

        ordenados.sort_by(|a, b| {
            ((a.1.min.x + a.1.max.x) * 0.5)
                .partial_cmp(&((b.1.min.x + b.1.max.x) * 0.5))
                .expect("no hay NaN en la fila")
        });

        let mut corte = 0usize;
        let mut mayor = f32::MIN;

        for k in 0..ordenados.len() - 1 {
            let hueco = ordenados[k + 1].1.min.x - ordenados[k].1.max.x;

            if hueco > mayor {
                mayor = hueco;
                corte = k;
            }
        }

        BocaCostera {
            izquierda: ordenados[..=corte].iter().map(|(i, _)| *i).collect(),
            derecha: ordenados[corte + 1..].iter().map(|(i, _)| *i).collect(),
            hueco: (ordenados[corte].1.max.x, ordenados[corte + 1].1.min.x),
        }
    }

    /// El labio de `refined` con la boca **fuera del eje**.
    ///
    /// # Qué corrige
    ///
    /// `refined` abrió la bahía partiendo la fila por la mitad, y eso dejó
    /// una boca exactamente centrada con cuatro bloques a cada lado. Una
    /// brecha simétrica en el eje de la toma hero se lee como una pieza
    /// fabricada, no como un borde roto.
    ///
    /// # Qué hace
    ///
    /// **Un solo bloque cruza la boca.** El más interior del flanco
    /// izquierdo se coloca un paso por dentro del flanco derecho, y el
    /// flanco derecho entero —los cuatro que ya estaban más el recién
    /// llegado— retrocede medio fondo.
    ///
    /// | Bloques | Δx | Δz |
    /// |---|---|---|
    /// | flanco corto, los tres de la izquierda | `0` | `0` |
    /// | el que cruza | `+2 × paso` | `−0.5 × fondo` |
    /// | flanco largo, los cuatro de la derecha | `0` | `−0.5 × fondo` |
    ///
    /// El resultado es una boca de tres contra cinco: el canal se corre un
    /// paso hacia la izquierda del eje, el flanco derecho queda
    /// inequívocamente más largo y además más retranqueado, y el izquierdo
    /// se queda corto y al frente.
    ///
    /// El paso y el fondo se **miden** de la propia fila: el paso es la
    /// menor separación entre dos tramos consecutivos y el fondo, la
    /// profundidad de un bloque. Ni uno ni otro está escrito a mano.
    ///
    /// # Qué no toca
    ///
    /// Sólo se mueven bloques de `A-11`, y sólo por traslación. `A-01` sigue
    /// siendo un único volumen en su sitio, el Monolito no se mueve, el
    /// plinto sigue en `22 × 19`, el solape Praderas–Rompeolas de `refined`
    /// —que la revisión dio por bueno— se conserva intacto, y el conteo
    /// sigue en `154`.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_asimetrico() -> Blockout {
        let mut diorama = nivel_refinado();

        let labio = labio_costero(&diorama.scene);
        let cajas: Vec<Aabb> = labio
            .iter()
            .map(|&i| diorama.scene.objects[i].primitive.bounds())
            .collect();

        let centro_x_de = |k: usize| (cajas[k].min.x + cajas[k].max.x) * 0.5;
        let fondo = cajas[0].max.z - cajas[0].min.z;

        let mut orden: Vec<usize> = (0..labio.len()).collect();
        orden.sort_by(|a, b| {
            centro_x_de(*a)
                .partial_cmp(&centro_x_de(*b))
                .expect("no hay NaN en la fila")
        });

        // Dos medidas de la misma fila: la menor separación entre tramos es
        // el paso, y la mayor es la boca que abrió `refined`.
        let mut paso = f32::MAX;
        let mut corte = 0usize;
        let mut mayor = f32::MIN;

        for k in 0..orden.len() - 1 {
            let separacion = centro_x_de(orden[k + 1]) - centro_x_de(orden[k]);

            paso = paso.min(separacion);

            if separacion > mayor {
                mayor = separacion;
                corte = k;
            }
        }

        // El bloque interior del flanco izquierdo cruza y se coloca un paso
        // por dentro del derecho.
        let destino = centro_x_de(orden[corte + 1]) - paso;
        let retranqueo = -fondo * 0.5;

        for (k, &bloque) in orden.iter().enumerate() {
            let delta = match k.cmp(&corte) {
                std::cmp::Ordering::Less => continue,
                std::cmp::Ordering::Equal => {
                    Vec3::new(destino - centro_x_de(bloque), 0.0, retranqueo)
                }
                std::cmp::Ordering::Greater => Vec3::new(0.0, 0.0, retranqueo),
            };

            let objeto = &mut diorama.scene.objects[labio[bloque]];

            objeto.primitive = trasladada(&objeto.primitive, delta);
        }

        // Ninguna ancla se movió: el labio se reordena dentro de Aguas. La
        // escala se vuelve a medir igual, porque la geometría cambió.
        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel =
            SceneAccel::build(&diorama.scene).expect("la escena asimetrica tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Tres territorios
    // ------------------------------------------------------------------

    /// El claro entre dos territorios. Es la unidad de la composición
    /// separada: todas las distancias de colocación son múltiplos suyos.
    const CLARO: f32 = 1.00;

    /// Lo que se le deja al borde del plinto al arrimar una región.
    const MARGEN_DEL_PLINTO: f32 = 0.20;

    /// Anchura del corredor entre Praderas y el Monolito.
    ///
    /// Más que un claro cualquiera: no es sólo que las dos huellas no se
    /// toquen, es que entre ellas hay una calle a la vista.
    const CORREDOR: f32 = CLARO * 1.5;

    /// Separación de dos huellas en planta.
    ///
    /// Positiva si las cajas no se tocan en `XZ` —y entonces vale el hueco
    /// libre por el eje que mejor las separa—, cero si se rozan, negativa si
    /// se solapan. Basta con que un eje separe: dos cajas alineadas en `x`
    /// pero alejadas en `z` no se tocan.
    pub fn separacion_xz(a: &Aabb, b: &Aabb) -> f32 {
        let en_x = (b.min.x - a.max.x).max(a.min.x - b.max.x);
        let en_z = (b.min.z - a.max.z).max(a.min.z - b.max.z);

        en_x.max(en_z)
    }

    /// A dónde va cada una de las tres regiones en la composición separada.
    #[derive(Debug, Clone, Copy)]
    pub struct RepartoSeparado {
        pub praderas: Vec3,
        pub rompeolas: Vec3,
        pub aguas: Vec3,
    }

    /// Calcula el reparto midiendo la escena de partida.
    ///
    /// # La composición
    ///
    /// Tres territorios, cada uno con su parcela, y el Monolito solo en el
    /// centro de todas. Nada se coloca en coordenadas escritas a mano: cada
    /// destino se expresa **contra otra cosa ya medida**.
    ///
    /// - **Rompeolas** — flanco izquierdo, arrimado al borde del plinto y
    ///   por delante del Monolito. Conserva su anchura y su silueta: es la
    ///   región más ancha del diorama y el flanco es el único sitio donde
    ///   cabe entera sin pasar por encima del Monolito.
    /// - **Praderas** — al fondo, **centrada con el Monolito** en `x`. Que
    ///   compartan banda es lo que convierte el hueco entre las dos en un
    ///   corredor recto y no en una diagonal.
    /// - **Aguas Voladoras** — al frente y a la derecha del Rompeolas,
    ///   arrimada al borde frontal. Sigue siendo lo más cercano a la cámara.
    ///
    /// El suelo que se usa para arrimar no es el plinto de la escena de
    /// partida sino el de `22 × 19` que va a tener: si no, las regiones se
    /// colocarían contra un borde que ya no existe.
    pub fn reparto_separado(scene: &Scene) -> RepartoSeparado {
        let plinto = huella_del_grupo(scene, SpatialGroupId::Global);
        let centro = (plinto.min + plinto.max) * 0.5;
        let medio = Vec3::new(
            PLINTO_ESPACIOSO_X * 0.5,
            (plinto.max.y - plinto.min.y) * 0.5,
            PLINTO_ESPACIOSO_Z * 0.5,
        );
        let suelo = Aabb::new(centro - medio, centro + medio);

        let monolito = huella_del_grupo(scene, SpatialGroupId::Monolith);
        let praderas = huella_del_grupo(scene, SpatialGroupId::Meadows);
        let rompeolas = huella_del_grupo(scene, SpatialGroupId::Breakwater);
        let aguas = huella_del_grupo(scene, SpatialGroupId::FlyingWaters);

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

        let aguas_delta = Vec3::new(
            // Contra el borde derecho del Rompeolas **ya movido**: si se
            // midiera contra el de partida, el claro saldría donde no es.
            (rompeolas.max.x + rompeolas_delta.x + CLARO) - aguas.min.x,
            0.0,
            (suelo.max.z - MARGEN_DEL_PLINTO) - aguas.max.z,
        );

        RepartoSeparado {
            praderas: praderas_delta,
            rompeolas: rompeolas_delta,
            aguas: aguas_delta,
        }
    }

    /// Tres territorios separados sobre la base de `22 × 19`.
    ///
    /// # Qué cambia respecto de las candidatas anteriores
    ///
    /// Todas ellas partían de que el diorama era **una cadena física**:
    /// Praderas apoyada sobre el Rompeolas, el Rompeolas continuando en el
    /// terreno continental, el terreno muriendo en la bahía. La corrección
    /// del usuario retira esa premisa. Aquí cada zona tiene parcela propia y
    /// entre ellas hay claro a la vista.
    ///
    /// | Región | Dónde | Contra qué se mide |
    /// |---|---|---|
    /// | Rompeolas | flanco izquierdo, delante | borde del plinto `+0.20`; Monolito `+1.00` en `z` |
    /// | Praderas | fondo, centrada con el Monolito | Monolito `−1.50` en `z`; su mismo eje en `x` |
    /// | Aguas | frente, derecha | Rompeolas `+1.00` en `x`; borde frontal `−0.20` |
    ///
    /// Ninguna pareja de huellas se toca en planta, y un test lo mide pareja
    /// a pareja. El Monolito queda libre por los tres lados y con un
    /// corredor recto hacia Praderas.
    ///
    /// # Qué se conserva
    ///
    /// La macroformación de `R-01`/`R-02` del candidato, intacta: el
    /// Rompeolas se traslada entero y no cambia de forma. El conteo sigue en
    /// `154`, el Monolito no se mueve, `A-01` sigue siendo un único volumen
    /// —trasladado con su región, no partido—, la base es la de `22 × 19` y
    /// no aparece ninguna primitiva, material ni overlay.
    ///
    /// # Qué no se toca, y por qué
    ///
    /// **El arco costero de `G-02` se queda donde está.** Moverlo era
    /// opcional y hacerlo habría significado repartir diez masas de terreno
    /// entre tres parcelas: o se convierte en tres pedestales —y entonces la
    /// base vuelve a ser un continente continuo, que es justo lo que el
    /// encargo excluye— o quedan trozos sueltos rellenando huecos, que es lo
    /// otro que excluye. Se queda como telón de fondo bajo, a `y ≤ 1.95`,
    /// por debajo de las tres regiones.
    ///
    /// La consecuencia hay que decirla: Praderas arranca en `y = 2.11` y no
    /// se apoya en nada. Flota sobre el terreno que le queda debajo, sin
    /// tocarlo. En una composición de diorama eso es una decisión; en una
    /// cadena física sería un fallo.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_separado() -> Blockout {
        let mut diorama = nivel_candidato();
        let reparto = reparto_separado(&diorama.scene);

        for objeto in &mut diorama.scene.objects {
            // `G-01` · el suelo crece en planta, alrededor de su centro.
            if objeto.spatial_group == SpatialGroupId::Global {
                let caja = objeto.primitive.bounds();
                let centro = (caja.min + caja.max) * 0.5;
                let espesor = caja.max.y - caja.min.y;

                objeto.primitive = Cuboid::centrado(
                    centro,
                    Vec3::new(PLINTO_ESPACIOSO_X, espesor, PLINTO_ESPACIOSO_Z),
                )
                .into();

                continue;
            }

            let delta = match objeto.spatial_group {
                SpatialGroupId::Meadows => reparto.praderas,
                SpatialGroupId::Breakwater => reparto.rompeolas,
                SpatialGroupId::FlyingWaters => reparto.aguas,
                // El Monolito y el arco costero se quedan.
                _ => continue,
            };

            objeto.primitive = trasladada(&objeto.primitive, delta);
        }

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            meadows_anchor: diorama.anchors.meadows_anchor + reparto.praderas,
            breakwater_anchor: diorama.anchors.breakwater_anchor + reparto.rompeolas,
            flying_waters_anchor: diorama.anchors.flying_waters_anchor + reparto.aguas,
            boat_anchor: diorama.anchors.boat_anchor + reparto.aguas,
            broken_edge_anchor: diorama.anchors.broken_edge_anchor + reparto.aguas,
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            monolith_height,
            water_surface_y: diorama.scale.water_surface_y + reparto.aguas.y,
            orbit_radius,
        };

        diorama.accel =
            SceneAccel::build(&diorama.scene).expect("la escena separada tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Apoyo vertical
    // ------------------------------------------------------------------

    /// Cuánto sobresale el pedestal por fuera de lo que sostiene, y cuánto
    /// se le deja al borde del plinto.
    const VUELO: f32 = 0.10;

    /// Cuánto se hunde la macroformación en su pedestal.
    ///
    /// No es un detalle de gusto: posarla a ras dejaría la cara de abajo de
    /// treinta y cuatro piezas **coplanar** con la de arriba de la losa, y
    /// dos caras a la misma distancia exacta son justo lo que la política de
    /// empates de `SceneAccel` dice que hay que evitar separando geometría.
    /// Empotrando un poco, el contacto está garantizado y no hay empate.
    const EMPOTRADO: f32 = 0.10;

    /// La masa de `G-02` que puede hacer de pedestal: la de mayor huella en
    /// planta.
    ///
    /// Se elige midiendo. En el inventario vigente gana por mucho —`12 × 7`
    /// contra `3.5 × 5` de la siguiente—, y es la única del arco lo bastante
    /// grande para cubrir la huella entera del Rompeolas.
    pub fn masa_pedestal(scene: &Scene) -> usize {
        let area = |i: usize| {
            let caja = scene.objects[i].primitive.bounds();

            (caja.max.x - caja.min.x) * (caja.max.z - caja.min.z)
        };

        indices_por_grupo(scene, SpatialGroupId::ContinentBackground)
            .into_iter()
            .fold(None, |mejor: Option<usize>, i| match mejor {
                Some(m) if area(m) >= area(i) => Some(m),
                _ => Some(i),
            })
            .expect("G-02 tiene masas")
    }

    /// El techo de lo que sostiene al objeto `i`, o `None` si no hay nada.
    ///
    /// Sostener es **contener la huella**, no rozarla: una losa que toca la
    /// esquina de un pilar no lo apoya, y contar esos contactos es como un
    /// preview se convence a sí mismo de que nada flota. Cuentan el plinto y
    /// las masas del arco costero; nada más es terreno.
    pub fn pedestal_bajo(scene: &Scene, i: usize) -> Option<f32> {
        let caja = scene.objects[i].primitive.bounds();

        pedestal_de_la_huella(scene, &caja, caja.max.y)
    }

    /// Lo mismo, para una huella que todavía no es un objeto de la escena.
    ///
    /// Lo necesita quien coloca una pieza **antes** de saber su altura: la
    /// escalera pregunta aquí dónde se posa cada prisma, así que el apoyo no
    /// es una promesa del constructor sino exactamente la misma medida que
    /// después comprueba el test. Dos reglas separadas era como se colaba
    /// un pilar enterrado en una masa que el constructor no miraba.
    pub fn pedestal_de_la_huella(scene: &Scene, caja: &Aabb, techo_pieza: f32) -> Option<f32> {
        let mut techo: Option<f32> = None;

        for grupo in [SpatialGroupId::Global, SpatialGroupId::ContinentBackground] {
            for &j in &indices_por_grupo(scene, grupo) {
                let soporte = scene.objects[j].primitive.bounds();

                let contiene = soporte.min.x <= caja.min.x
                    && soporte.max.x >= caja.max.x
                    && soporte.min.z <= caja.min.z
                    && soporte.max.z >= caja.max.z;

                if contiene && soporte.max.y < techo_pieza {
                    techo = Some(techo.map_or(soporte.max.y, |t: f32| t.max(soporte.max.y)));
                }
            }
        }

        techo
    }

    /// Cómo se apoya el Rompeolas: dónde va su losa y cuánto baja él.
    #[derive(Debug, Clone, Copy)]
    pub struct ApoyoDelRompeolas {
        /// La masa de `G-02` que pasa a hacer de pedestal.
        pub indice: usize,
        /// Adónde va esa masa.
        pub pedestal: Vec3,
        /// Cuánto baja el Rompeolas entero. Sólo tiene componente en `y`.
        pub rompeolas: Vec3,
    }

    /// Calcula el apoyo midiendo la escena separada.
    ///
    /// # El diagnóstico
    ///
    /// Las treinta y cuatro piezas de la macroformación arrancan todas en la
    /// misma cota y descansaban sobre los cuatro soportes de `R-03`. Esos
    /// cuatro no cubren la huella entera de la región: en la composición
    /// separada, el Rompeolas se lleva su pedestal parcial a una parcela de
    /// lienzo desnudo, y las piezas que caen fuera de `R-03` se quedan a
    /// `2.40` del plinto. Eso es lo que la revisión vio flotar.
    ///
    /// # La corrección
    ///
    /// Dos traslaciones, ninguna pieza nueva:
    ///
    /// 1. La masa mayor de `G-02` se posa en el plinto y se centra bajo el
    ///    Rompeolas. Pasa a ser su losa: es lo bastante grande para
    ///    **contener** la huella de las treinta y cuatro piezas.
    /// 2. El Rompeolas baja **en bloque** hasta empotrarse `0.10` en esa
    ///    losa. En bloque y no pieza a pieza: la macroformación es la que la
    ///    revisión aprobó, y nivelar cada prisma por su cuenta la aplanaría.
    ///
    /// La huella en planta no se toca, así que los claros entre las tres
    /// regiones son exactamente los de `separated`.
    pub fn apoyo_del_rompeolas(scene: &Scene) -> ApoyoDelRompeolas {
        let plinto = huella_del_grupo(scene, SpatialGroupId::Global);
        let piezas = indices_por_grupo(scene, SpatialGroupId::Breakwater);
        let rompeolas = huella(scene, &piezas);

        let indice = masa_pedestal(scene);
        let losa = scene.objects[indice].primitive.bounds();

        // Centrada bajo el Rompeolas, y recortada para que no se salga del
        // plinto: una losa en voladizo sería otra cosa flotando.
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
            // Se posa en el plinto: su cara de abajo en la de arriba del
            // lienzo. Más alto flotaría ella.
            plinto.max.y - losa.min.y,
            centro_z - (losa.min.z + losa.max.z) * 0.5,
        );

        // La base **más alta** de la macroformación es la que hay que hacer
        // llegar al techo de la losa: con la media, las piezas que
        // arrancaran más arriba se quedarían colgadas.
        let techo = losa.max.y + pedestal.y;
        let base = piezas[..PIEZAS]
            .iter()
            .map(|&i| scene.objects[i].primitive.bounds().min.y)
            .fold(f32::MIN, f32::max);

        ApoyoDelRompeolas {
            indice,
            pedestal,
            rompeolas: Vec3::new(0.0, (techo - EMPOTRADO) - base, 0.0),
        }
    }

    /// La composición separada, con el Rompeolas apoyado.
    ///
    /// Corrige el único hallazgo de la revisión sobre `separated` —prismas
    /// que flotan— sin tocar nada más: Praderas y Aguas se quedan donde
    /// están, el Monolito y `A-01` no se mueven, el plinto sigue en
    /// `22 × 19`, las huellas en planta de las tres regiones son las mismas
    /// y el conteo sigue en `154`.
    ///
    /// Lo único que cambia son dos traslaciones: la losa de `G-02` que pasa
    /// a hacer de pedestal y el Rompeolas, que baja en bloque. Ver
    /// `apoyo_del_rompeolas`.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_apoyado() -> Blockout {
        let mut diorama = nivel_separado();
        let apoyo = apoyo_del_rompeolas(&diorama.scene);

        for (i, objeto) in diorama.scene.objects.iter_mut().enumerate() {
            let delta = if i == apoyo.indice {
                apoyo.pedestal
            } else if objeto.spatial_group == SpatialGroupId::Breakwater {
                apoyo.rompeolas
            } else {
                continue;
            };

            objeto.primitive = trasladada(&objeto.primitive, delta);
        }

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            // La única ancla que sigue a su geometría: el Rompeolas bajó.
            breakwater_anchor: diorama.anchors.breakwater_anchor + apoyo.rompeolas,
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel =
            SceneAccel::build(&diorama.scene).expect("la escena apoyada tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // La escalera irregular
    // ------------------------------------------------------------------

    /// Cuántos pilares lleva cada peldaño, de abajo arriba. Suman `28`.
    ///
    /// No es una progresión: `4, 3, 3, 2, 3, 2, 3, 2, 2, 2, 2`. La base es
    /// ancha —la escalera nace de una masa— y arriba queda en pares. Que el
    /// reparto no sea regular es la mitad de lo que separa una formación de
    /// una escalinata.
    const PELDANOS: [usize; 11] = [4, 3, 3, 2, 3, 2, 3, 2, 2, 2, 2];

    /// Semilla del trazado. Fija, como el resto del preview.
    const SEMILLA_ESCALERA: u32 = 0x3E5C_A1E2;

    /// Anchura de la banda de pilares al pie de la subida.
    const BANDA: f32 = 1.90;

    /// Cuánto se estrecha esa banda al llegar arriba.
    ///
    /// La formación se abre abajo y se afila arriba. Sin esto, la subida
    /// llegaría a Praderas igual de ancha que salió del Rompeolas y se
    /// leería como un muro inclinado.
    const ESTRECHAMIENTO: f32 = 0.55;

    /// Altura mínima de un pilar de la escalera.
    const ALTURA_MINIMA: f32 = 0.30;

    /// Los pilares de `R-01` ordenados por su avance en la subida.
    ///
    /// El avance se mide como `−z`: la escalera sale del Rompeolas y va
    /// hacia el fondo, así que profundizar **es** avanzar. Medirlo así, y no
    /// reconstruyendo el eje del trazado, deja que un test lea el resultado
    /// sin creerse la construcción.
    pub fn pilares_en_orden(scene: &Scene) -> Vec<usize> {
        let piezas = indices_por_grupo(scene, SpatialGroupId::Breakwater);
        let centro_z = |i: usize| {
            let caja = scene.objects[i].primitive.bounds();

            (caja.min.z + caja.max.z) * 0.5
        };

        let mut orden = piezas[..PILARES_R01].to_vec();
        orden.sort_by(|a, b| {
            centro_z(*b)
                .partial_cmp(&centro_z(*a))
                .expect("no hay NaN en la escalera")
        });

        orden
    }

    /// Las veintiocho piezas de `R-01`.
    const PILARES_R01: usize = 28;

    /// La composición apoyada, con los pilares de `R-01` reorganizados en
    /// una escalera irregular hacia la izquierda de Praderas.
    ///
    /// # Qué hace
    ///
    /// Los veintiocho prismas dejan el macizo y se reparten en once peldaños
    /// a lo largo de una diagonal que sale del Rompeolas —de su flanco
    /// izquierdo, hacia el fondo— y sube hasta el borde izquierdo de
    /// Praderas, deteniéndose un claro antes de su caja. **No va hacia
    /// Aguas Voladoras**: la bahía queda en el flanco contrario y la
    /// diagonal se aleja de ella.
    ///
    /// # Por qué no parece una escalinata
    ///
    /// Cinco cosas, todas deterministas y todas medidas por un test:
    ///
    /// - El reparto por peldaño no es regular: `4, 3, 3, 2, 3, 2, 3, 2, 2,
    ///   2, 2`.
    /// - El avance de cada peldaño se sortea entre `0.45` y `1.55` veces el
    ///   paso medio, y se acumula: irregular, pero siempre hacia arriba.
    /// - Cada pilar se aparta de lado dentro de una banda que se estrecha al
    ///   subir, así que la nube **no se ajusta a una recta**.
    /// - Cada pilar recibe además un pequeño adelanto o retraso sobre su
    ///   propio peldaño: ni siquiera los del mismo escalón quedan alineados.
    /// - El techo de cada pilar es el de su peldaño más un ruido de `±0.45`,
    ///   frente a una subida media de `0.44` por peldaño. La consecuencia es
    ///   que el conjunto sube y las parejas vecinas se cruzan: un test exige
    ///   las dos cosas a la vez.
    ///
    /// Los anchos no se inventan: cada pilar conserva el suyo, el que le dio
    /// el reparto por racimos de la macroformación. De ahí salen huellas de
    /// tamaños distintos sin tocar una sola constante nueva.
    ///
    /// # Qué se conserva
    ///
    /// Sólo se tocan esos veintiocho. `R-02`, `R-03` y la losa que los
    /// sostiene se quedan donde estaban, así que el Rompeolas sigue teniendo
    /// sus treinta y ocho piezas y su soporte. Praderas y Aguas no se mueven,
    /// el Monolito y `A-01` tampoco, la base sigue en `22 × 19` y el conteo
    /// en `154`. No aparece ninguna primitiva, material ni overlay: los
    /// prismas se reconstruyen con `pieza`, el mismo constructor de la
    /// macroformación, así que la Ruta A sigue dando `HexPrism` y la B
    /// `Cuboid`.
    ///
    /// # Apoyo
    ///
    /// Cada pilar se posa en lo que tenga debajo, y lo pregunta con la misma
    /// función que luego lo comprueba: la losa mientras su huella cabe
    /// entera en ella, cualquier otra masa del arco que la contenga, y el
    /// lienzo cuando no hay nada más. Empotra `0.10`, igual que en
    /// `grounded` y por la misma razón. Las **bases** dan saltos de hasta
    /// `1.20` de un pilar al siguiente, invisibles porque quedan dentro del
    /// terreno; lo que se ve es el perfil de los techos, que no los acusa.
    ///
    /// # Dos recortes
    ///
    /// El trazado apunta al borde izquierdo de Praderas, pero un pilar es
    /// ancho y se aparta de lado: el punto de llegada no es la caja de
    /// llegada. Por eso, ya colocados los veintiocho y **antes** de darles
    /// altura, el conjunto se recorta en bloque hasta dejar un claro entero
    /// contra Praderas y contra el Monolito. Recortar después de asignar las
    /// bases movería pilares dentro y fuera de la losa y los dejaría
    /// flotando, que es el fallo que `grounded` acaba de arreglar.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_escalera() -> Blockout {
        let mut diorama = nivel_apoyado();

        let piezas = indices_por_grupo(&diorama.scene, SpatialGroupId::Breakwater);
        let pilares: Vec<usize> = piezas[..PILARES_R01].to_vec();

        let plinto = huella_del_grupo(&diorama.scene, SpatialGroupId::Global);
        let praderas = huella_del_grupo(&diorama.scene, SpatialGroupId::Meadows);
        let monolito = huella_del_grupo(&diorama.scene, SpatialGroupId::Monolith);
        let rompeolas = huella(&diorama.scene, &piezas);

        // El trazado. Sale del flanco izquierdo del Rompeolas, por su lado
        // de atrás, y apunta al borde izquierdo de Praderas dejándole un
        // claro. Las cuatro cifras salen de cajas medidas.
        let inicio = (
            rompeolas.min.x + (rompeolas.max.x - rompeolas.min.x) * 0.28,
            rompeolas.max.z - (rompeolas.max.z - rompeolas.min.z) * 0.55,
        );
        let fin = (praderas.min.x, praderas.max.z + CLARO);

        // Sube desde la cota de los soportes que ya había hasta justo por
        // debajo de la meseta.
        let techo_inicio = huella(&diorama.scene, &piezas[PIEZAS..]).max.y;
        let techo_fin = praderas.max.y - CLARO;

        // Cada pilar conserva su propio ancho.
        let anchos: Vec<f32> = pilares
            .iter()
            .map(|&i| {
                let caja = diorama.scene.objects[i].primitive.bounds();

                caja.max.x - caja.min.x
            })
            .collect();

        let direccion = (fin.0 - inicio.0, fin.1 - inicio.1);
        let largo = (direccion.0 * direccion.0 + direccion.1 * direccion.1).sqrt();
        let tangente = (direccion.0 / largo, direccion.1 / largo);
        let perpendicular = (-tangente.1, tangente.0);

        let mut azar = Azar::new(SEMILLA_ESCALERA);

        // Avance irregular pero monótono: incrementos positivos acumulados.
        let mut incrementos = [0.0_f32; PELDANOS.len()];
        let mut total = 0.0_f32;

        for incremento in incrementos.iter_mut().skip(1) {
            *incremento = 1.0 + 0.55 * azar.simetrico();
            total += *incremento;
        }

        let mut avance = 0.0_f32;
        let mut recorrido = [0.0_f32; PELDANOS.len()];

        for (k, incremento) in incrementos.iter().enumerate().skip(1) {
            avance += incremento;
            recorrido[k] = avance / total;
        }

        // Primera pasada: dónde cae cada pilar y hasta dónde llega su techo.
        let mut centros: Vec<(f32, f32)> = Vec::with_capacity(PILARES_R01);
        let mut techos: Vec<f32> = Vec::with_capacity(PILARES_R01);

        for (k, &cuantos) in PELDANOS.iter().enumerate() {
            let t = recorrido[k];
            let banda = BANDA * (1.0 - ESTRECHAMIENTO * t);
            let techo = techo_inicio + (techo_fin - techo_inicio) * t.powf(0.9);
            let pie = (inicio.0 + direccion.0 * t, inicio.1 + direccion.1 * t);

            for j in 0..cuantos {
                let reparto = if cuantos == 1 {
                    0.0
                } else {
                    j as f32 / (cuantos - 1) as f32 * 2.0 - 1.0
                };

                let lado = (reparto * 0.5 + 0.45 * azar.simetrico()) * banda;
                let adelanto = 0.35 * azar.simetrico();

                centros.push((
                    pie.0 + perpendicular.0 * lado + tangente.0 * adelanto,
                    pie.1 + perpendicular.1 * lado + tangente.1 * adelanto,
                ));
                techos.push(techo + 0.45 * azar.simetrico());
            }
        }

        debug_assert_eq!(centros.len(), PILARES_R01, "PELDANOS tiene que sumar 28");

        // Huella provisional de cada pilar, para medir el conjunto antes de
        // decidir alturas.
        let provisional =
            |centro: (f32, f32), ancho: f32| pieza(Vec3::new(centro.0, 0.0, centro.1), ancho, 1.0);

        let mut conjunto = provisional(centros[0], anchos[0]).bounds();

        for k in 1..PILARES_R01 {
            let caja = provisional(centros[k], anchos[k]).bounds();

            conjunto = Aabb::new(
                Vec3::new(
                    conjunto.min.x.min(caja.min.x),
                    conjunto.min.y.min(caja.min.y),
                    conjunto.min.z.min(caja.min.z),
                ),
                Vec3::new(
                    conjunto.max.x.max(caja.max.x),
                    conjunto.max.y.max(caja.max.y),
                    conjunto.max.z.max(caja.max.z),
                ),
            );
        }

        // Los dos recortes: un claro entero contra Praderas y otro contra el
        // Monolito. Se aplican al conjunto, así que el trazado no se deforma.
        let recorte = (
            (monolito.min.x - CLARO - conjunto.max.x).min(0.0),
            (praderas.max.z + CLARO - conjunto.min.z).max(0.0),
        );

        // Segunda pasada: apoyo y altura, ya en su sitio definitivo.
        for k in 0..PILARES_R01 {
            let centro = (centros[k].0 + recorte.0, centros[k].1 + recorte.1);
            let caja = provisional(centro, anchos[k]).bounds();

            // Lo que de verdad hay bajo esta huella, sea la losa, sea otra
            // masa del arco o sea el lienzo. Preguntarlo en vez de suponerlo
            // es lo que evita un pilar enterrado en un resto de terreno.
            let suelo =
                pedestal_de_la_huella(&diorama.scene, &caja, techos[k]).unwrap_or(plinto.max.y);
            let base = suelo - EMPOTRADO;
            let altura = (techos[k] - base).max(ALTURA_MINIMA);

            diorama.scene.objects[pilares[k]].primitive = pieza(
                Vec3::new(centro.0, base + altura * 0.5, centro.1),
                anchos[k],
                altura,
            );
        }

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        // Ninguna ancla se mueve: el Rompeolas no se traslada en bloque, se
        // reorganiza por dentro, y no hay un vector que su ancla pueda
        // seguir. Las luces no la usan.
        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel =
            SceneAccel::build(&diorama.scene).expect("la escena de la escalera tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Macizo, ascenso y barranco
    // ------------------------------------------------------------------

    /// Lo que se ve en `exp33/im1`, `im2` e `im3` y guía este reparto.
    ///
    /// Son tres observaciones de las capturas, no interpretaciones:
    ///
    /// 1. **`im1` · el macizo no tiene huecos.** Los prismas del bloque se
    ///    tocan hombro con hombro; lo que hace irregular la silueta es que
    ///    cada uno termina a una altura distinta, no que estén separados.
    ///    Lejos del bloque, sobre suelo llano, sólo quedan tocones cortos en
    ///    grupos de dos o tres.
    /// 2. **`im2` · el flanco es una pared.** El costado del macizo cae casi
    ///    a plomo: lo que se ve de esa cara son los **lados** de los
    ///    prismas, no sus tapas, y las tapas bajan de golpe. A su pie hay un
    ///    abanico de columnas mucho más bajas.
    /// 3. **`im3` · varios macizos y nada equiespaciado.** Hay masas
    ///    separadas por suelo llano, cada una con su cima, y entre ellas
    ///    pavimentos de tapas hexagonales a ras de suelo. En ninguna de las
    ///    tres capturas hay una fila de columnas a paso constante.
    ///
    /// De ahí las tres partes de este nivel: un **macizo** apretado y bajo
    /// —tan apretado que la suma de sus huellas cubre su propia caja—, un
    /// **ascenso** de pocos prismas en racimos menguantes, y dos **racimos
    /// laterales** en el flanco que mira a la cámara, uno alto de pared y
    /// otro bajo de abanico.
    ///
    /// Lo que las referencias **no** dan y por tanto no se copia: el número
    /// de columnas, sus proporciones exactas y el color. El preview trabaja
    /// con las veintiocho piezas que hay y con los anchos que ya tenían.
    const _REFERENCIAS: () = ();

    /// Reparto de las veintiocho piezas de `R-01`.
    const MACIZO: usize = 14;
    const ASCENSO: usize = 9;
    const LATERALES: usize = 5;

    /// Filas del macizo, en retícula hexagonal. Suman `MACIZO`.
    const MACIZO_FILAS: [usize; 4] = [4, 4, 3, 3];

    /// Prismas por peldaño del ascenso, de abajo arriba. Suman `ASCENSO`.
    ///
    /// Mengua: sale de una masa y termina en piezas sueltas, que es como se
    /// deshilacha el borde del bloque en `im1`.
    const ASCENSO_PELDANOS: [usize; 5] = [3, 2, 2, 1, 1];

    /// Los dos racimos del barranco: `(u, v, cuántos)` en fracciones de la
    /// parcela. El primero es la pared, el segundo el abanico de su pie.
    const BARRANCO: [(f32, f32, usize); 2] = [(0.11, 0.78, 3), (0.36, 0.82, 2)];

    /// Radio de un racimo alrededor de su centro.
    const RADIO_RACIMO: f32 = 0.55;

    /// Semilla del reparto combinado.
    const SEMILLA_COMBINADA: u32 = 0x7C0B_11DA;

    /// Anchura de la banda del ascenso al pie, y cuánto se estrecha arriba.
    const BANDA_ASCENSO: f32 = 1.60;
    const ESTRECHAMIENTO_ASCENSO: f32 = 0.60;

    /// Los prismas del macizo basal.
    pub fn macizo_basal(scene: &Scene) -> Vec<usize> {
        indices_por_grupo(scene, SpatialGroupId::Breakwater)[..MACIZO].to_vec()
    }

    /// Los prismas del ascenso, ordenados por su avance hacia Praderas.
    ///
    /// El avance se mide como `−z`, igual que en `staircase`: la subida va
    /// hacia el fondo, así que profundizar es avanzar.
    pub fn ascenso_combinado(scene: &Scene) -> Vec<usize> {
        let piezas = indices_por_grupo(scene, SpatialGroupId::Breakwater);
        let centro_z = |i: usize| {
            let caja = scene.objects[i].primitive.bounds();

            (caja.min.z + caja.max.z) * 0.5
        };

        let mut orden = piezas[MACIZO..MACIZO + ASCENSO].to_vec();
        orden.sort_by(|a, b| {
            centro_z(*b)
                .partial_cmp(&centro_z(*a))
                .expect("no hay NaN en el ascenso")
        });

        orden
    }

    /// Los prismas de los racimos laterales.
    pub fn laterales_combinados(scene: &Scene) -> Vec<usize> {
        indices_por_grupo(scene, SpatialGroupId::Breakwater)
            [MACIZO + ASCENSO..MACIZO + ASCENSO + LATERALES]
            .to_vec()
    }

    /// La composición apoyada con el Rompeolas repartido en macizo, ascenso
    /// y barranco.
    ///
    /// # Qué corrige
    ///
    /// `staircase` se llevó **las veintiocho** piezas a la subida y dejó la
    /// parcela del Rompeolas con sus losas bajas y nada más. La corrección
    /// humana es que eso parece que todos los prismas se fueron de
    /// excursión. Aquí sólo sube una parte.
    ///
    /// | Parte | Prismas | Qué es |
    /// |---|---:|---|
    /// | macizo basal | `14` | masa apretada y baja, en la parcela |
    /// | ascenso | `9` | racimos menguantes por la diagonal a Praderas |
    /// | barranco | `5` | dos racimos en el flanco que mira a la cámara |
    ///
    /// Las tres partes salen de las tres observaciones de las referencias;
    /// ver `_REFERENCIAS`.
    ///
    /// # Qué se conserva
    ///
    /// `R-02`, `R-03` y la losa pedestal no se tocan, así que el Rompeolas
    /// sigue con sus treinta y ocho piezas y su soporte. Praderas y Aguas no
    /// se mueven, el Monolito y `A-01` tampoco, la base sigue en `22 × 19`,
    /// el conteo en `154`, y no aparece ninguna primitiva, material ni
    /// overlay: los prismas se reconstruyen con `pieza`, conservando cada
    /// uno el ancho que ya tenía.
    ///
    /// El ascenso se detiene un claro entero antes de Praderas y **no va
    /// hacia Aguas Voladoras**: la bahía está en el flanco contrario.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_combinado() -> Blockout {
        let mut diorama = nivel_apoyado();

        let piezas = indices_por_grupo(&diorama.scene, SpatialGroupId::Breakwater);
        let pilares: Vec<usize> = piezas[..PILARES_R01].to_vec();

        let plinto = huella_del_grupo(&diorama.scene, SpatialGroupId::Global);
        let praderas = huella_del_grupo(&diorama.scene, SpatialGroupId::Meadows);
        let monolito = huella_del_grupo(&diorama.scene, SpatialGroupId::Monolith);
        let soportes = huella(&diorama.scene, &piezas[PIEZAS..]);
        let parcela = diorama.scene.objects[masa_pedestal(&diorama.scene)]
            .primitive
            .bounds();

        let anchos: Vec<f32> = pilares
            .iter()
            .map(|&i| {
                let caja = diorama.scene.objects[i].primitive.bounds();

                caja.max.x - caja.min.x
            })
            .collect();

        let ancho_parcela = parcela.max.x - parcela.min.x;
        let fondo_parcela = parcela.max.z - parcela.min.z;
        let en_parcela = |u: f32, v: f32| {
            (
                parcela.min.x + ancho_parcela * u,
                parcela.min.z + fondo_parcela * v,
            )
        };

        // Las tres cotas de la composición, medidas: la de los soportes que
        // ya había, la cima del macizo y el techo al que llega el ascenso.
        let cota_baja = soportes.max.y;
        let cota_macizo = praderas.min.y + (praderas.max.y - praderas.min.y) * 0.35;
        let cota_cima = praderas.max.y - CLARO;

        let mut azar = Azar::new(SEMILLA_COMBINADA);
        let mut centros: Vec<(f32, f32)> = Vec::with_capacity(PILARES_R01);
        let mut techos: Vec<f32> = Vec::with_capacity(PILARES_R01);

        // ---------------------------------------------------- el macizo
        //
        // Retícula hexagonal apretada: el paso es menor que el ancho de los
        // prismas, así que se solapan y no queda hueco entre ellos. `im1`.
        let centro_macizo = en_parcela(0.22, 0.40);
        let medio_macizo = (ancho_parcela * 0.14, fondo_parcela * 0.15);

        for (fila, &columnas) in MACIZO_FILAS.iter().enumerate() {
            let v = fila as f32 / (MACIZO_FILAS.len() - 1) as f32;
            let z = centro_macizo.1 - medio_macizo.1 + v * 2.0 * medio_macizo.1;
            let desfase = if fila % 2 == 1 { 0.5 } else { 0.0 };

            for columna in 0..columnas {
                let u = (columna as f32 + 0.5 + desfase) / columnas as f32;
                let x = centro_macizo.0 - medio_macizo.0 + u * 2.0 * medio_macizo.0;

                // Cresta: pesa en el centro y cae hacia los bordes. La
                // silueta la rompe el ruido de cada pieza, no su posición.
                let cresta = (1.0
                    - 0.55 * (2.0 * u - 1.0) * (2.0 * u - 1.0)
                    - 0.35 * (2.0 * v - 1.0) * (2.0 * v - 1.0))
                    .max(0.0);

                centros.push((x + 0.18 * azar.simetrico(), z + 0.18 * azar.simetrico()));
                techos
                    .push(cota_baja + (cota_macizo - cota_baja) * cresta + 0.35 * azar.simetrico());
            }
        }

        // ---------------------------------------------------- el ascenso
        //
        // Sale del canto interior del macizo y sube hacia el borde
        // izquierdo de Praderas. Racimos menguantes, avance irregular.
        let inicio = (
            centro_macizo.0 + medio_macizo.0 * 0.5,
            centro_macizo.1 - medio_macizo.1 * 0.9,
        );
        let fin = (praderas.min.x, praderas.max.z + CLARO);
        let direccion = (fin.0 - inicio.0, fin.1 - inicio.1);
        let largo = (direccion.0 * direccion.0 + direccion.1 * direccion.1).sqrt();
        let tangente = (direccion.0 / largo, direccion.1 / largo);
        let perpendicular = (-tangente.1, tangente.0);

        let mut incrementos = [0.0_f32; ASCENSO_PELDANOS.len()];
        let mut total = 0.0_f32;

        for incremento in incrementos.iter_mut().skip(1) {
            *incremento = 1.0 + 0.55 * azar.simetrico();
            total += *incremento;
        }

        let mut recorrido = [0.0_f32; ASCENSO_PELDANOS.len()];
        let mut acumulado = 0.0_f32;

        for (k, incremento) in incrementos.iter().enumerate().skip(1) {
            acumulado += incremento;
            recorrido[k] = acumulado / total;
        }

        let primero_del_ascenso = centros.len();

        for (k, &cuantos) in ASCENSO_PELDANOS.iter().enumerate() {
            let t = recorrido[k];
            let banda = BANDA_ASCENSO * (1.0 - ESTRECHAMIENTO_ASCENSO * t);
            let techo = cota_macizo + (cota_cima - cota_macizo) * t.powf(0.9);
            let pie = (inicio.0 + direccion.0 * t, inicio.1 + direccion.1 * t);

            for j in 0..cuantos {
                let reparto = if cuantos == 1 {
                    0.0
                } else {
                    j as f32 / (cuantos - 1) as f32 * 2.0 - 1.0
                };

                let lado = (reparto * 0.5 + 0.45 * azar.simetrico()) * banda;
                let adelanto = 0.35 * azar.simetrico();

                centros.push((
                    pie.0 + perpendicular.0 * lado + tangente.0 * adelanto,
                    pie.1 + perpendicular.1 * lado + tangente.1 * adelanto,
                ));
                techos.push(techo + 0.40 * azar.simetrico());
            }
        }

        // --------------------------------------------------- el barranco
        //
        // Dos racimos en el flanco que mira a la camara: uno alto, que hace
        // de pared, y otro bajo a su pie. `im2`.
        for (indice, &(u, v, cuantos)) in BARRANCO.iter().enumerate() {
            let centro = en_parcela(u, v);
            let techo = if indice == 0 {
                cota_macizo
            } else {
                cota_baja - 0.70
            };

            for j in 0..cuantos {
                let angulo =
                    (j as f32 / cuantos as f32 + 0.17 * azar.siguiente()) * std::f32::consts::TAU;
                let radio = RADIO_RACIMO * (0.35 + 0.65 * azar.siguiente());

                centros.push((
                    centro.0 + radio * angulo.cos(),
                    centro.1 + radio * angulo.sin(),
                ));
                techos.push(techo + 0.30 * azar.simetrico());
            }
        }

        debug_assert_eq!(centros.len(), PILARES_R01, "el reparto tiene que sumar 28");

        // Huella provisional, para recortar el ascenso antes de darle altura.
        let provisional =
            |centro: (f32, f32), ancho: f32| pieza(Vec3::new(centro.0, 0.0, centro.1), ancho, 1.0);

        let mut caja_ascenso =
            provisional(centros[primero_del_ascenso], anchos[primero_del_ascenso]).bounds();

        for k in primero_del_ascenso + 1..primero_del_ascenso + ASCENSO {
            let caja = provisional(centros[k], anchos[k]).bounds();

            caja_ascenso = Aabb::new(
                Vec3::new(
                    caja_ascenso.min.x.min(caja.min.x),
                    caja_ascenso.min.y.min(caja.min.y),
                    caja_ascenso.min.z.min(caja.min.z),
                ),
                Vec3::new(
                    caja_ascenso.max.x.max(caja.max.x),
                    caja_ascenso.max.y.max(caja.max.y),
                    caja_ascenso.max.z.max(caja.max.z),
                ),
            );
        }

        // Sólo el ascenso se recorta: el macizo y el barranco están en la
        // parcela y no se acercan a nada.
        let recorte = (
            (monolito.min.x - CLARO - caja_ascenso.max.x).min(0.0),
            (praderas.max.z + CLARO - caja_ascenso.min.z).max(0.0),
        );

        for k in 0..PILARES_R01 {
            let dentro_del_ascenso = k >= primero_del_ascenso && k < primero_del_ascenso + ASCENSO;
            let centro = if dentro_del_ascenso {
                (centros[k].0 + recorte.0, centros[k].1 + recorte.1)
            } else {
                centros[k]
            };

            let caja = provisional(centro, anchos[k]).bounds();
            let suelo =
                pedestal_de_la_huella(&diorama.scene, &caja, techos[k]).unwrap_or(plinto.max.y);
            let base = suelo - EMPOTRADO;
            let altura = (techos[k] - base).max(ALTURA_MINIMA);

            diorama.scene.objects[pilares[k]].primitive = pieza(
                Vec3::new(centro.0, base + altura * 0.5, centro.1),
                anchos[k],
                altura,
            );
        }

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel =
            SceneAccel::build(&diorama.scene).expect("la escena combinada tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Extensión desde la cima
    // ------------------------------------------------------------------

    /// Cuántos prismas de la cima se llevan a la extensión.
    ///
    /// Seis de veintiocho. Pocos a propósito: `grounded` es la base
    /// aprobada y lo que se ensaya aquí es una continuidad **secundaria**,
    /// no otro reparto del Rompeolas.
    const CIMAS: usize = 6;

    /// Semilla de la extensión.
    const SEMILLA_CIMA: u32 = 0x1C1A_5E33;

    /// Cuánto se aparta de lado cada prisma respecto del trazado.
    const DESVIO_CIMA: f32 = 1.30;

    /// Los prismas más altos de `R-01`, medidos sobre la escena que se pase.
    ///
    /// La selección es una **función de la geometría**, no una lista de
    /// índices: se ordena por el techo de cada caja y se toman los primeros.
    /// El empate se rompe por índice, así que es determinista, y un test
    /// comprueba que cualquiera de los elegidos remata más alto que
    /// cualquiera de los que se quedan.
    pub fn cimas_seleccionadas(scene: &Scene) -> Vec<usize> {
        let piezas = indices_por_grupo(scene, SpatialGroupId::Breakwater);
        let techo = |i: usize| scene.objects[i].primitive.bounds().max.y;

        let mut orden = piezas[..PILARES_R01].to_vec();
        orden.sort_by(|a, b| {
            techo(*b)
                .partial_cmp(&techo(*a))
                .expect("no hay NaN en el Rompeolas")
        });
        orden.truncate(CIMAS);
        orden.sort_unstable();

        orden
    }

    /// `grounded` con una extensión que nace de su cima y avanza hacia la
    /// izquierda de Praderas.
    ///
    /// # Qué toca y qué no
    ///
    /// `grounded` es la base aprobada, así que **nada de ella se mueve**: ni
    /// la losa pedestal, ni la posición del Rompeolas, ni Praderas, Aguas,
    /// el Monolito o `A-01`. Lo único que cambia son los **seis prismas más
    /// altos** de `R-01`, elegidos midiendo su techo.
    ///
    /// Los otros veintidós se quedan donde están y siguen siendo el macizo:
    /// mayoritarios —más del triple que los que salen— y en su misma huella.
    ///
    /// # La extensión
    ///
    /// Los seis salen del canto del macizo que mira a Praderas y se colocan
    /// a lo largo de una diagonal con avances **muy desiguales**: el
    /// incremento de cada uno se sortea entre `0.35` y `1.65`, así que unos
    /// quedan casi pegados y otros sueltos. Se detiene un claro entero antes
    /// de la meseta y no se dirige a Aguas Voladoras, que está en el flanco
    /// contrario.
    ///
    /// # Por qué se lee montañosa y no como una escalera
    ///
    /// Porque **no se les cambia la altura**. Cada prisma se traslada
    /// entero, con el tamaño que ya tenía, y se posa en lo que haya bajo su
    /// nueva huella: la losa, una masa suelta del arco costero o el propio
    /// lienzo. Esos suelos están a `1.20`, `1.10`, `0.70`, `0.60` y `0.00`,
    /// así que seis prismas de altura parecida rematan a cotas que difieren
    /// hasta en `1.20` sin que nadie lo haya decidido: lo decide el terreno.
    /// De ahí también que los techos no suban en orden.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_ascenso_desde_grounded() -> Blockout {
        let mut diorama = nivel_apoyado();

        let cimas = cimas_seleccionadas(&diorama.scene);
        let cajas: Vec<Aabb> = cimas
            .iter()
            .map(|&i| diorama.scene.objects[i].primitive.bounds())
            .collect();

        let plinto = huella_del_grupo(&diorama.scene, SpatialGroupId::Global);
        let praderas = huella_del_grupo(&diorama.scene, SpatialGroupId::Meadows);
        let monolito = huella_del_grupo(&diorama.scene, SpatialGroupId::Monolith);
        let cumbre = huella(&diorama.scene, &cimas);

        // Nace en el canto **de atrás** de la cima y cruza el macizo hacia el
        // borde izquierdo de Praderas, dejándole un claro. Arranca ahí y no
        // en el canto delantero porque seis prismas de metro y medio no
        // caben separados en un trazado más corto: quedarían tocandose, que
        // es el puente que no se quiere.
        let origen = ((cumbre.min.x + cumbre.max.x) * 0.5, cumbre.max.z);
        let destino = (praderas.min.x, praderas.max.z + CLARO);
        let direccion = (destino.0 - origen.0, destino.1 - origen.1);
        let largo = (direccion.0 * direccion.0 + direccion.1 * direccion.1).sqrt();
        let tangente = (direccion.0 / largo, direccion.1 / largo);
        let perpendicular = (-tangente.1, tangente.0);

        let mut azar = Azar::new(SEMILLA_CIMA);

        // Avances desiguales, y **muy** desiguales: el sorteo va al cuadrado,
        // asi que la mayoria de los pasos salen cortos y alguno sale largo.
        // Un reparto uniforme dejaba seis prismas casi tocandose a lo largo
        // del trazado, que es un puente; con este, unos quedan pegados y
        // otro abre un vacio de verdad.
        let mut incrementos = [0.0_f32; CIMAS];
        let mut total = 0.0_f32;

        for incremento in incrementos.iter_mut().skip(1) {
            let sorteo = azar.siguiente();

            *incremento = 0.25 + 1.75 * sorteo * sorteo;
            total += *incremento;
        }

        let mut recorrido = [0.0_f32; CIMAS];
        let mut acumulado = 0.0_f32;

        for (k, incremento) in incrementos.iter().enumerate().skip(1) {
            acumulado += incremento;
            recorrido[k] = acumulado / total;
        }

        let mut centros: Vec<(f32, f32)> = Vec::with_capacity(CIMAS);

        for &t in recorrido.iter() {
            let lado = DESVIO_CIMA * azar.simetrico();
            let pie = (origen.0 + direccion.0 * t, origen.1 + direccion.1 * t);

            centros.push((
                pie.0 + perpendicular.0 * lado,
                pie.1 + perpendicular.1 * lado,
            ));
        }

        // La caja del conjunto, para recortarlo antes de posarlo: el trazado
        // apunta al borde de Praderas, pero un prisma es ancho.
        let desplazada = |k: usize, centro: (f32, f32)| {
            let actual = (
                (cajas[k].min.x + cajas[k].max.x) * 0.5,
                (cajas[k].min.z + cajas[k].max.z) * 0.5,
            );

            Aabb::new(
                Vec3::new(
                    cajas[k].min.x + centro.0 - actual.0,
                    cajas[k].min.y,
                    cajas[k].min.z + centro.1 - actual.1,
                ),
                Vec3::new(
                    cajas[k].max.x + centro.0 - actual.0,
                    cajas[k].max.y,
                    cajas[k].max.z + centro.1 - actual.1,
                ),
            )
        };

        let mut conjunto = desplazada(0, centros[0]);

        for (k, centro) in centros.iter().enumerate().skip(1) {
            let caja = desplazada(k, *centro);

            conjunto = Aabb::new(
                Vec3::new(
                    conjunto.min.x.min(caja.min.x),
                    conjunto.min.y.min(caja.min.y),
                    conjunto.min.z.min(caja.min.z),
                ),
                Vec3::new(
                    conjunto.max.x.max(caja.max.x),
                    conjunto.max.y.max(caja.max.y),
                    conjunto.max.z.max(caja.max.z),
                ),
            );
        }

        let recorte = (
            (monolito.min.x - CLARO - conjunto.max.x).min(0.0),
            (praderas.max.z + CLARO - conjunto.min.z).max(0.0),
        );

        for (k, &i) in cimas.iter().enumerate() {
            let centro = (centros[k].0 + recorte.0, centros[k].1 + recorte.1);
            let caja = desplazada(k, centro);

            // Se posa en lo que haya bajo su nueva huella. Nada de alturas
            // inventadas: la traslación en `y` es la que hace falta para que
            // la base toque, y el prisma conserva su tamaño.
            let suelo =
                pedestal_de_la_huella(&diorama.scene, &caja, caja.max.y).unwrap_or(plinto.max.y);
            let delta = Vec3::new(
                caja.min.x - cajas[k].min.x,
                (suelo - EMPOTRADO) - cajas[k].min.y,
                caja.min.z - cajas[k].min.z,
            );

            diorama.scene.objects[i].primitive =
                trasladada(&diorama.scene.objects[i].primitive, delta);
        }

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel =
            SceneAccel::build(&diorama.scene).expect("la escena de la extension tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Relleno de la cara −Z
    // ------------------------------------------------------------------

    /// Cuántos prismas se traen a la cara `−Z`.
    const TRASEROS: usize = 5;

    /// Fondo de la franja que se puebla, en unidades de mundo.
    ///
    /// Algo más que la profundidad de un prisma: cabe una segunda fila
    /// escalonada por delante de la que ya hay, y no más.
    const FONDO_DE_LA_FRANJA: f32 = 1.30;

    /// Semilla del relleno.
    const SEMILLA_RELLENO: u32 = 0x5A17_B4C0;

    /// La franja `−Z` de la parcela del Rompeolas: su borde hacia Praderas.
    ///
    /// **`−Z` no es `−X`.** El lado que hay que poblar es el que mira a la
    /// meseta, y en este diorama la meseta está al fondo, no a la izquierda.
    /// La franja se mide sobre la losa pedestal, que es el suelo propio de
    /// la región: quedarse dentro de ella garantiza a la vez el apoyo y el
    /// claro, porque la parcela ya está a más de un claro de Praderas.
    pub fn franja_trasera(scene: &Scene) -> (f32, f32) {
        let parcela = scene.objects[masa_pedestal(scene)].primitive.bounds();

        (parcela.min.z, parcela.min.z + FONDO_DE_LA_FRANJA)
    }

    /// Cuánta abscisa cubren los prismas de `R-01` que pisan una franja.
    ///
    /// Es la **unión** de sus intervalos en `x`, no la suma: dos prismas
    /// solapados no cubren el doble. Sirve para decir si una cara se pobló
    /// de verdad o si sólo se le amontonaron piezas encima.
    pub fn cobertura_de_la_franja(scene: &Scene, franja: (f32, f32)) -> f32 {
        let piezas = indices_por_grupo(scene, SpatialGroupId::Breakwater);

        let mut tramos: Vec<(f32, f32)> = piezas[..PILARES_R01]
            .iter()
            .map(|&i| scene.objects[i].primitive.bounds())
            .filter(|caja| caja.min.z < franja.1 && caja.max.z > franja.0)
            .map(|caja| (caja.min.x, caja.max.x))
            .collect();

        tramos.sort_by(|a, b| a.0.partial_cmp(&b.0).expect("no hay NaN en la franja"));

        let mut cubierto = 0.0_f32;
        let mut hasta = f32::MIN;

        for (desde, fin) in tramos {
            let arranque = desde.max(hasta);

            if fin > arranque {
                cubierto += fin - arranque;
                hasta = fin;
            }
        }

        cubierto
    }

    /// Los prismas de `R-01` que están más atrás, medidos.
    ///
    /// Se ordenan por el centro de su caja en `z` y se toman los del fondo:
    /// son los que menos falta hacen donde están —allí ya están las losas de
    /// `R-02`— y los que más falta hacen delante. Un test comprueba que
    /// cualquiera de los elegidos queda más atrás que cualquiera de los que
    /// se quedan, y que la selección no pisa la de la extensión.
    pub fn traseros_seleccionados(scene: &Scene) -> Vec<usize> {
        let piezas = indices_por_grupo(scene, SpatialGroupId::Breakwater);
        let fondo = |i: usize| {
            let caja = scene.objects[i].primitive.bounds();

            (caja.min.z + caja.max.z) * 0.5
        };

        let mut orden = piezas[..PILARES_R01].to_vec();
        orden.sort_by(|a, b| {
            fondo(*b)
                .partial_cmp(&fondo(*a))
                .expect("no hay NaN en el Rompeolas")
        });
        orden.truncate(TRASEROS);
        orden.sort_unstable();

        orden
    }

    /// `grounded_ascent` con la cara `−Z` del Rompeolas poblada.
    ///
    /// # Qué corrige
    ///
    /// La aclaración humana: el lado que había que llenar es el `−Z` de la
    /// parcela del Rompeolas —el que mira a Praderas—, no el `−X`. En
    /// `grounded_ascent` esa cara la forman tres prismas sueltos de la
    /// primera fila y poco más, porque la cresta se fue a la extensión.
    ///
    /// # Qué hace
    ///
    /// Trae **cinco** prismas del fondo de la formación, donde sobran
    /// porque allí ya están las losas de `R-02`, a una segunda fila
    /// escalonada delante de la primera. Los cinco son los más bajos de la
    /// formación —el fondo del macizo lo es—, así que la cara gana cuerpo
    /// sin levantar una pared que tape el macizo: es el pie de un barranco,
    /// no otra cresta.
    ///
    /// La fila no es una fila: los avances en `x` se sortean al cuadrado
    /// —la mayoría cortos, alguno largo— y cada prisma se adelanta o se
    /// retrasa en `z` por su cuenta.
    ///
    /// # Qué no toca
    ///
    /// Todo lo demás de `grounded_ascent`, que a su vez no tocaba
    /// `grounded`: la losa pedestal, la posición del Rompeolas, Praderas,
    /// Aguas, el Monolito, `A-01`, los claros y la extensión de seis
    /// prismas. El conteo sigue en `154` y no aparece ninguna primitiva,
    /// material ni overlay.
    ///
    /// Los cinco se quedan **dentro de la parcela**, y eso es lo que
    /// garantiza a la vez que cada uno se apoya en la losa y que el claro a
    /// Praderas no se toca: la parcela empieza en `z = 2.04` y la meseta
    /// acaba en `−2.60`.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_relleno_trasero() -> Blockout {
        let mut diorama = nivel_ascenso_desde_grounded();

        let traseros = traseros_seleccionados(&diorama.scene);
        let cajas: Vec<Aabb> = traseros
            .iter()
            .map(|&i| diorama.scene.objects[i].primitive.bounds())
            .collect();

        let plinto = huella_del_grupo(&diorama.scene, SpatialGroupId::Global);
        let parcela = diorama.scene.objects[masa_pedestal(&diorama.scene)]
            .primitive
            .bounds();
        let franja = franja_trasera(&diorama.scene);

        // El tramo de cara que se puebla: el flanco del macizo por donde
        // sale la extensión, no la cara entera. Llenar once unidades con
        // cinco prismas sería salpicarlos.
        let piezas = indices_por_grupo(&diorama.scene, SpatialGroupId::Breakwater);
        let macizo = huella(&diorama.scene, &piezas[..PILARES_R01]);
        let ancho_macizo = macizo.max.x - macizo.min.x;
        let desde = macizo.min.x + ancho_macizo * 0.12;
        let hasta = macizo.min.x + ancho_macizo * 0.56;

        let mut azar = Azar::new(SEMILLA_RELLENO);

        // Avances desiguales al cuadrado: la mayoría cortos y alguno largo,
        // que es lo que impide que cinco prismas en fila parezcan una reja.
        let mut incrementos = [0.0_f32; TRASEROS];
        let mut total = 0.0_f32;

        for incremento in incrementos.iter_mut().skip(1) {
            let sorteo = azar.siguiente();

            *incremento = 0.30 + 1.70 * sorteo * sorteo;
            total += *incremento;
        }

        let mut recorrido = [0.0_f32; TRASEROS];
        let mut acumulado = 0.0_f32;

        for (k, incremento) in incrementos.iter().enumerate().skip(1) {
            acumulado += incremento;
            recorrido[k] = acumulado / total;
        }

        let centro_franja = (franja.0 + franja.1) * 0.5;

        for (k, &i) in traseros.iter().enumerate() {
            let medio_x = (cajas[k].max.x - cajas[k].min.x) * 0.5;
            let medio_z = (cajas[k].max.z - cajas[k].min.z) * 0.5;

            // Dentro de la parcela por construcción: la abscisa se recorta a
            // lo que cabe y la profundidad se sortea sin salir de la franja.
            let x = (desde + (hasta - desde) * recorrido[k])
                .clamp(parcela.min.x + medio_x, parcela.max.x - medio_x);
            let z = (centro_franja + 0.35 * azar.simetrico())
                .clamp(parcela.min.z + medio_z, parcela.max.z - medio_z);

            let caja = Aabb::new(
                Vec3::new(x - medio_x, cajas[k].min.y, z - medio_z),
                Vec3::new(x + medio_x, cajas[k].max.y, z + medio_z),
            );

            let suelo =
                pedestal_de_la_huella(&diorama.scene, &caja, caja.max.y).unwrap_or(plinto.max.y);
            let delta = Vec3::new(
                caja.min.x - cajas[k].min.x,
                (suelo - EMPOTRADO) - cajas[k].min.y,
                caja.min.z - cajas[k].min.z,
            );

            diorama.scene.objects[i].primitive =
                trasladada(&diorama.scene.objects[i].primitive, delta);
        }

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel =
            SceneAccel::build(&diorama.scene).expect("la escena del relleno tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Escalones tectónicos tras el Monolito
    // ------------------------------------------------------------------

    /// Cuántos prismas forman la ruta.
    const PRISMAS_DE_LA_RUTA: usize = 7;

    /// Cómo se reparten: cuatro de aproximación y tres en el hombro trasero.
    const EN_LA_APROXIMACION: usize = 4;

    /// Semilla de la ruta.
    const SEMILLA_RUTA: u32 = 0x4D01_17A5;

    /// Cuánto se aparta de lado cada prisma respecto del trazado.
    const DESVIO_RUTA: f32 = 0.85;

    /// Holgura que se le deja al flanco del Monolito.
    const HOLGURA_MONOLITO: f32 = 0.45;

    /// Los prismas de `R-01` que forman la ruta: los más altos, medidos.
    ///
    /// Los mismos criterios que la extensión de `grounded_ascent` —techo de
    /// la caja, empate por índice— porque son los que se ven desde lejos y
    /// los que tiene sentido llevarse: la ruta es la cresta saliendo del
    /// macizo, no un trozo cualquiera.
    pub fn prismas_de_la_ruta(scene: &Scene) -> Vec<usize> {
        let piezas = indices_por_grupo(scene, SpatialGroupId::Breakwater);
        let techo = |i: usize| scene.objects[i].primitive.bounds().max.y;

        let mut orden = piezas[..PILARES_R01].to_vec();
        orden.sort_by(|a, b| {
            techo(*b)
                .partial_cmp(&techo(*a))
                .expect("no hay NaN en el Rompeolas")
        });
        orden.truncate(PRISMAS_DE_LA_RUTA);
        orden.sort_unstable();

        orden
    }

    /// `grounded` con una ruta de escalones que sale del Rompeolas, rodea el
    /// Monolito por su flanco y alcanza la banda que queda detrás de él,
    /// hasta el claro de Praderas.
    ///
    /// # Lo que la geometría no deja hacer, y hay que decirlo
    ///
    /// Pasar **por dentro** de la banda que hay justo detrás del Monolito es
    /// imposible aquí. Su cara trasera está en `z = −1.10` y el claro de
    /// Praderas obliga a no bajar de `z = −1.60`: quedan `0.50` de fondo
    /// libre, y los prismas de `R-01` miden entre `1.1` y `1.9` de fondo.
    /// Ninguno cabe. Cualquier prisma metido en la abscisa del Monolito y en
    /// esa profundidad cortaría su caja.
    ///
    /// Tampoco puede rodearlo por la derecha: el Rompeolas no puede pasar de
    /// `x = 0.58` sin comerse el claro con Aguas Voladoras.
    ///
    /// Así que la ruta lo **rodea por la izquierda** y se adentra en la
    /// banda trasera pegada a su flanco: varios prismas terminan con su cara
    /// de atrás más allá de `z = −1.10`, es decir por detrás del Monolito,
    /// sin tocar su caja y a menos de tres unidades de su costado.
    ///
    /// # La ruta
    ///
    /// Una polilínea de dos tramos —de la cresta al costado del Monolito, y
    /// de ahí a la banda trasera— con siete prismas repartidos en avances
    /// desiguales que se acortan al final, así que se agolpan donde importa.
    /// Cada uno se aparta de lado por su cuenta, y si aun así su caja
    /// cortara la del Monolito se le empuja hacia fuera hasta despejarla.
    ///
    /// # Por qué son escalones
    ///
    /// Porque **no se les cambia la altura**: cada prisma se traslada entero
    /// y se posa en lo que haya bajo su nueva huella. A lo largo de la ruta
    /// eso son la losa (`1.20`), las masas sueltas del arco (`0.70`, `1.10`)
    /// y el lienzo (`0.00`). Siete prismas de altura parecida sobre cuatro
    /// cotas distintas dan una escalera que nadie ha dibujado: la dibuja el
    /// terreno.
    ///
    /// # Qué no toca
    ///
    /// `grounded` entero salvo esos siete: la losa pedestal, el resto del
    /// macizo —veintiuno, mayoritario y en su misma huella—, Praderas,
    /// Aguas, el Monolito, `A-01`, la base de `22 × 19` y el conteo de
    /// `154`. No aparece ninguna primitiva, material ni overlay.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_escalera_monolito() -> Blockout {
        let mut diorama = nivel_apoyado();

        let elegidos = prismas_de_la_ruta(&diorama.scene);
        let cajas: Vec<Aabb> = elegidos
            .iter()
            .map(|&i| diorama.scene.objects[i].primitive.bounds())
            .collect();

        let plinto = huella_del_grupo(&diorama.scene, SpatialGroupId::Global);
        let praderas = huella_del_grupo(&diorama.scene, SpatialGroupId::Meadows);
        let monolito = huella_del_grupo(&diorama.scene, SpatialGroupId::Monolith);
        let parcela = diorama.scene.objects[masa_pedestal(&diorama.scene)]
            .primitive
            .bounds();
        let cumbre = huella(&diorama.scene, &elegidos);

        // El medio fondo del prisma más profundo: lo que hay que descontarle
        // al claro para que su cara de atrás no lo cruce.
        let medio_fondo = cajas
            .iter()
            .map(|caja| (caja.max.z - caja.min.z) * 0.5)
            .fold(f32::MIN, f32::max);

        // La polilínea. Tres puntos, todos medidos contra cajas:
        //
        // - de la cresta, dentro de la parcela;
        // - al costado izquierdo del Monolito, a su altura en `z`;
        // - a la banda trasera, pegada a ese mismo costado.
        let paso = [
            ((cumbre.min.x + cumbre.max.x) * 0.5, parcela.min.z + 0.60),
            (
                monolito.min.x - HOLGURA_MONOLITO * 2.6,
                (monolito.min.z + monolito.max.z) * 0.5,
            ),
            (
                monolito.min.x - HOLGURA_MONOLITO * 2.9,
                // El final se pone donde el prisma más hondo **ya cabe**
                // dentro del claro, no más allá. Apuntar más abajo obligaba
                // a recortar todo el conjunto hacia atrás, y el recorte
                // deshacía justo la profundidad que se buscaba: la primera
                // versión de este corte dejaba dos prismas detrás del
                // Monolito en vez de los tres que se le exigen.
                praderas.max.z + CLARO + medio_fondo,
            ),
        ];

        let mut azar = Azar::new(SEMILLA_RUTA);

        // La ruta va en dos tramos con reparto **declarado**, no sorteado:
        // cuatro prismas de aproximación y tres en el hombro de detrás del
        // Monolito.
        //
        // Por qué declarado: la primera versión repartía los siete a lo largo
        // de una polilínea con avances que se acortaban, y cuántos caían en
        // la banda trasera dependía del fondo de los prismas. En la Ruta A
        // los prismas hexagonales son un `15 %` más profundos que los
        // cuboides de la Ruta B, y eso bastaba para que la misma
        // construcción dejara tres prismas detrás del Monolito en una ruta y
        // dos en la otra. Contarlos aquí lo hace igual en las dos.
        let mut centros: Vec<(f32, f32)> = Vec::with_capacity(PRISMAS_DE_LA_RUTA);

        // --- aproximación: del macizo al costado del Monolito
        let mut incrementos = [0.0_f32; EN_LA_APROXIMACION];
        let mut total = 0.0_f32;

        for incremento in incrementos.iter_mut().skip(1) {
            *incremento = (1.0 + 0.55 * azar.simetrico()).max(0.2);
            total += *incremento;
        }

        let direccion = (paso[1].0 - paso[0].0, paso[1].1 - paso[0].1);
        let largo = (direccion.0 * direccion.0 + direccion.1 * direccion.1).sqrt();
        let perpendicular = (-direccion.1 / largo, direccion.0 / largo);
        let mut acumulado = 0.0_f32;

        for incremento in incrementos.iter() {
            acumulado += incremento;

            let t = acumulado / total;
            let lado = DESVIO_RUTA * azar.simetrico();

            centros.push((
                paso[0].0 + direccion.0 * t + perpendicular.0 * lado,
                paso[0].1 + direccion.1 * t + perpendicular.1 * lado,
            ));
        }

        // --- hombro: pegado al flanco del Monolito y por detrás de su cara
        //
        // El fondo se fija donde el prisma **ya cabe** dentro del claro, y el
        // sorteo sólo lo sube: así ninguno cruza el claro y todos quedan por
        // detrás de la cara trasera del Monolito, en las dos rutas.
        let mut borde = monolito.min.x - HOLGURA_MONOLITO;

        for caja in cajas
            .iter()
            .take(PRISMAS_DE_LA_RUTA)
            .skip(EN_LA_APROXIMACION)
        {
            let medio_ancho = (caja.max.x - caja.min.x) * 0.5;
            let medio_propio = (caja.max.z - caja.min.z) * 0.5;

            let centro_x = borde - medio_ancho;
            let centro_z = praderas.max.z + CLARO + medio_propio + 0.30 * azar.siguiente();

            centros.push((centro_x, centro_z));

            // Hueco irregular hasta el siguiente: el hombro no es una fila.
            borde = centro_x - medio_ancho - (0.25 + 1.10 * azar.siguiente());
        }

        debug_assert_eq!(
            centros.len(),
            PRISMAS_DE_LA_RUTA,
            "el reparto tiene que sumar siete"
        );

        // Caja provisional de cada prisma en su nuevo sitio.
        let desplazada = |k: usize, centro: (f32, f32)| {
            let medio_x = (cajas[k].max.x - cajas[k].min.x) * 0.5;
            let medio_z = (cajas[k].max.z - cajas[k].min.z) * 0.5;

            Aabb::new(
                Vec3::new(centro.0 - medio_x, cajas[k].min.y, centro.1 - medio_z),
                Vec3::new(centro.0 + medio_x, cajas[k].max.y, centro.1 + medio_z),
            )
        };

        // Los del hombro ya nacen dentro del claro; los de la aproximación
        // van muy por delante de él. No hace falta recortar el conjunto.
        let recorte = 0.0_f32;

        for (k, &i) in elegidos.iter().enumerate() {
            let mut centro = (centros[k].0, centros[k].1 + recorte);
            let mut caja = desplazada(k, centro);

            // Si aun así cortara la caja del Monolito, se le empuja hacia su
            // costado izquierdo hasta despejarla. Es el único ajuste que la
            // ruta hace pieza a pieza, y existe porque el Monolito es lo que
            // hay que rodear.
            let corta = caja.min.x < monolito.max.x
                && caja.max.x > monolito.min.x
                && caja.min.z < monolito.max.z
                && caja.max.z > monolito.min.z;

            if corta {
                centro.0 -= caja.max.x - (monolito.min.x - HOLGURA_MONOLITO);
                caja = desplazada(k, centro);
            }

            let suelo =
                pedestal_de_la_huella(&diorama.scene, &caja, caja.max.y).unwrap_or(plinto.max.y);
            let delta = Vec3::new(
                caja.min.x - cajas[k].min.x,
                (suelo - EMPOTRADO) - cajas[k].min.y,
                caja.min.z - cajas[k].min.z,
            );

            diorama.scene.objects[i].primitive =
                trasladada(&diorama.scene.objects[i].primitive, delta);
        }

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel =
            SceneAccel::build(&diorama.scene).expect("la escena de la ruta tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Rompeolas de borde
    // ------------------------------------------------------------------

    /// Cuántos prismas lleva cada estación del corredor, de Aguas al claro
    /// de Praderas. Suman `28`.
    ///
    /// No es una progresión: `5, 4, 3, 4, 2, 3, 2, 3, 2`. La formación pesa
    /// donde arranca, se afina, vuelve a engordar y se deshilacha al final.
    /// Eso son racimos; un reparto parejo sería una reja.
    const ESTACIONES: [usize; 9] = [5, 4, 3, 4, 2, 3, 2, 3, 2];

    /// Semilla del borde.
    const SEMILLA_BORDE: u32 = 0x80DE_2C11;

    /// Anchura de la banda de prismas al arrancar, y cuánto se estrecha al
    /// llegar al final del corredor.
    const BANDA_BORDE: f32 = 2.80;
    const ESTRECHAMIENTO_BORDE: f32 = 0.45;

    /// El Rompeolas ocupando **todo** el borde disponible.
    ///
    /// # Qué corrige
    ///
    /// `monolith_stair` sacaba siete prismas a rodear el Monolito y dejaba
    /// el resto en el macizo. La corrección es otra: la región tiene que
    /// **ocupar el corredor entero** que queda libre entre Aguas Voladoras y
    /// Praderas, y leerse como un borde geológico continuo.
    ///
    /// # El corredor
    ///
    /// Tres puntos, todos medidos contra cajas:
    ///
    /// 1. junto a Aguas, dentro de la parcela, a un claro de la bahía;
    /// 2. el costado izquierdo del Monolito, por delante de él;
    /// 3. el final, donde el prisma más hondo **ya cabe** dentro del claro
    ///    de Praderas.
    ///
    /// Las veintiocho piezas de `R-01` se reparten en nueve estaciones a lo
    /// largo de él, con avances desiguales y una banda que se estrecha al
    /// final. `R-02`, `R-03` y la losa pedestal se quedan donde estaban.
    ///
    /// # El descenso junto al Monolito
    ///
    /// No se consigue cambiando alturas —aquí no se cambia ninguna— sino
    /// **eligiendo qué prisma va a qué sitio**: se ordenan las plazas por su
    /// distancia al Monolito y los prismas por su altura, y se emparejan al
    /// revés. Los más altos van a las plazas más lejanas y los más bajos a
    /// las que quedan a su lado. La formación baja al acercarse a él porque
    /// allí sólo hay piezas cortas, y un test lo mide comparando las dos
    /// medias.
    ///
    /// Cada prisma conserva su tamaño y se posa en lo que haya bajo su nueva
    /// huella: la losa, las masas sueltas del arco o el lienzo. Las cotas de
    /// arranque cambian a lo largo del corredor, y eso quiebra el remate sin
    /// que nadie lo dibuje.
    ///
    /// # Los límites
    ///
    /// Tres recortes por pieza, todos medidos: no pasar del claro con Aguas,
    /// no bajar del claro con Praderas y no cortar la caja del Monolito. El
    /// último empuja hacia el costado izquierdo, que es por donde el
    /// corredor pasa.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_rompeolas_de_borde() -> Blockout {
        let mut diorama = nivel_apoyado();

        let piezas = indices_por_grupo(&diorama.scene, SpatialGroupId::Breakwater);
        let pilares: Vec<usize> = piezas[..PILARES_R01].to_vec();
        let cajas: Vec<Aabb> = pilares
            .iter()
            .map(|&i| diorama.scene.objects[i].primitive.bounds())
            .collect();

        let plinto = huella_del_grupo(&diorama.scene, SpatialGroupId::Global);
        let praderas = huella_del_grupo(&diorama.scene, SpatialGroupId::Meadows);
        let aguas = huella_del_grupo(&diorama.scene, SpatialGroupId::FlyingWaters);
        let monolito = huella_del_grupo(&diorama.scene, SpatialGroupId::Monolith);
        let parcela = diorama.scene.objects[masa_pedestal(&diorama.scene)]
            .primitive
            .bounds();

        let medio_ancho = cajas
            .iter()
            .map(|caja| (caja.max.x - caja.min.x) * 0.5)
            .fold(f32::MIN, f32::max);
        let medio_fondo = cajas
            .iter()
            .map(|caja| (caja.max.z - caja.min.z) * 0.5)
            .fold(f32::MIN, f32::max);

        // Los dos topes del corredor, y sus tres puntos.
        let tope_derecho = aguas.min.x - CLARO;
        let tope_hondo = praderas.max.z + CLARO;

        let paso = [
            (tope_derecho - medio_ancho, parcela.min.z + 4.50),
            (monolito.min.x - CLARO * 1.4, monolito.max.z + 0.40),
            (monolito.min.x - CLARO * 1.5, tope_hondo + medio_fondo),
        ];

        let mut azar = Azar::new(SEMILLA_BORDE);

        // Avances desiguales, acumulados: irregular pero siempre hacia el
        // final del corredor.
        let mut incrementos = [0.0_f32; ESTACIONES.len()];
        let mut total = 0.0_f32;

        for incremento in incrementos.iter_mut().skip(1) {
            *incremento = (1.0 + 0.60 * azar.simetrico()).max(0.25);
            total += *incremento;
        }

        let mut recorrido = [0.0_f32; ESTACIONES.len()];
        let mut acumulado = 0.0_f32;

        for (k, incremento) in incrementos.iter().enumerate().skip(1) {
            acumulado += incremento;
            recorrido[k] = acumulado / total;
        }

        // Punto del corredor y su tangente. Dos tramos.
        let en_el_corredor = |t: f32| -> ((f32, f32), (f32, f32)) {
            let (a, b, u) = if t <= 0.5 {
                (paso[0], paso[1], t * 2.0)
            } else {
                (paso[1], paso[2], (t - 0.5) * 2.0)
            };

            let direccion = (b.0 - a.0, b.1 - a.1);
            let largo = (direccion.0 * direccion.0 + direccion.1 * direccion.1).sqrt();

            (
                (a.0 + direccion.0 * u, a.1 + direccion.1 * u),
                (direccion.0 / largo, direccion.1 / largo),
            )
        };

        // Las veintiocho plazas del corredor.
        let mut plazas: Vec<(f32, f32)> = Vec::with_capacity(PILARES_R01);

        for (k, &cuantos) in ESTACIONES.iter().enumerate() {
            let t = recorrido[k];
            let (pie, tangente) = en_el_corredor(t);
            let perpendicular = (-tangente.1, tangente.0);
            let banda = BANDA_BORDE * (1.0 - ESTRECHAMIENTO_BORDE * t);

            for j in 0..cuantos {
                let reparto = if cuantos == 1 {
                    0.0
                } else {
                    j as f32 / (cuantos - 1) as f32 * 2.0 - 1.0
                };

                let lado = (reparto * 0.5 + 0.40 * azar.simetrico()) * banda;
                let adelanto = 0.30 * azar.simetrico();

                plazas.push((
                    pie.0 + perpendicular.0 * lado + tangente.0 * adelanto,
                    pie.1 + perpendicular.1 * lado + tangente.1 * adelanto,
                ));
            }
        }

        debug_assert_eq!(plazas.len(), PILARES_R01, "ESTACIONES tiene que sumar 28");

        // El emparejamiento que produce el descenso: plazas de más lejos a
        // más cerca del Monolito, prismas de más alto a más bajo.
        let distancia = |plaza: (f32, f32)| {
            let en_x = (monolito.min.x - plaza.0).max(plaza.0 - monolito.max.x);
            let en_z = (monolito.min.z - plaza.1).max(plaza.1 - monolito.max.z);

            en_x.max(en_z)
        };

        let mut orden_de_plazas: Vec<usize> = (0..PILARES_R01).collect();
        orden_de_plazas.sort_by(|a, b| {
            distancia(plazas[*b])
                .partial_cmp(&distancia(plazas[*a]))
                .expect("no hay NaN en el corredor")
        });

        let mut orden_de_prismas: Vec<usize> = (0..PILARES_R01).collect();
        orden_de_prismas.sort_by(|a, b| {
            (cajas[*b].max.y - cajas[*b].min.y)
                .partial_cmp(&(cajas[*a].max.y - cajas[*a].min.y))
                .expect("no hay NaN en el Rompeolas")
        });

        for (&plaza, &prisma) in orden_de_plazas.iter().zip(&orden_de_prismas) {
            let mitad_x = (cajas[prisma].max.x - cajas[prisma].min.x) * 0.5;
            let mitad_z = (cajas[prisma].max.z - cajas[prisma].min.z) * 0.5;

            // Los tres recortes, en orden: la bahía, la meseta y el Monolito.
            let mut centro = (
                plazas[plaza]
                    .0
                    .clamp(plinto.min.x + mitad_x, tope_derecho - mitad_x),
                plazas[plaza]
                    .1
                    .clamp(tope_hondo + mitad_z, plinto.max.z - mitad_z),
            );

            let mut caja = Aabb::new(
                Vec3::new(centro.0 - mitad_x, cajas[prisma].min.y, centro.1 - mitad_z),
                Vec3::new(centro.0 + mitad_x, cajas[prisma].max.y, centro.1 + mitad_z),
            );

            let corta = caja.min.x < monolito.max.x
                && caja.max.x > monolito.min.x
                && caja.min.z < monolito.max.z
                && caja.max.z > monolito.min.z;

            if corta {
                centro.0 -= caja.max.x - (monolito.min.x - CLARO * 0.45);
                caja = Aabb::new(
                    Vec3::new(centro.0 - mitad_x, cajas[prisma].min.y, centro.1 - mitad_z),
                    Vec3::new(centro.0 + mitad_x, cajas[prisma].max.y, centro.1 + mitad_z),
                );
            }

            let suelo =
                pedestal_de_la_huella(&diorama.scene, &caja, caja.max.y).unwrap_or(plinto.max.y);
            let delta = Vec3::new(
                caja.min.x - cajas[prisma].min.x,
                (suelo - EMPOTRADO) - cajas[prisma].min.y,
                caja.min.z - cajas[prisma].min.z,
            );

            diorama.scene.objects[pilares[prisma]].primitive =
                trasladada(&diorama.scene.objects[pilares[prisma]].primitive, delta);
        }

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel =
            SceneAccel::build(&diorama.scene).expect("la escena del borde tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Terraza verde de borde
    // ------------------------------------------------------------------

    /// Holgura que se le deja al Monolito al prolongar la terraza.
    const HOLGURA_TERRAZA: f32 = 0.25;

    /// `grounded` con la terraza de soporte prolongada por el borde.
    ///
    /// # Qué hace
    ///
    /// Redimensiona **una sola pieza**: la masa de `G-02` que `grounded`
    /// convirtió en losa del Rompeolas. No se mueve ni un prisma; `R-01`,
    /// `R-02` y `R-03` quedan exactos. La losa conserva su material, su
    /// grupo espacial y su grupo de revelación: sólo cambia su caja.
    ///
    /// | | antes | después |
    /// |---|---|---|
    /// | `x` | `−10.90 … 1.10` | igual |
    /// | `z` | `2.04 … 9.04` | `1.80 … 9.30` |
    /// | `y` | `0.00 … 1.20` | igual |
    ///
    /// # Hasta dónde llega, y por qué no más
    ///
    /// La terraza tiene que **cubrir el Rompeolas**, y el Rompeolas llega
    /// hasta `x = 0.58`. El Monolito ocupa `x ∈ [−1.20, 1.25]`, así que
    /// cualquier caja que cubra el Rompeolas está dentro de su abscisa. Y su
    /// cara delantera está en `z = 1.55`. Consecuencia: una caja única que
    /// cubra el Rompeolas **no puede bajar de `z = 1.55`** sin tragarse el
    /// Monolito.
    ///
    /// Eso deja la prolongación en `0.24` hacia `−Z` y `0.26` hacia el
    /// frente, hasta el borde del plinto. La banda que queda libre detrás
    /// del Monolito —`0.50` entre su cara trasera y el claro de Praderas—
    /// es inalcanzable para esta losa: habría que saltar por encima de él, y
    /// una caja no salta.
    ///
    /// Ir más allá exigiría una de estas tres, y ninguna está autorizada
    /// aquí: una segunda masa que continúe la terraza al otro lado del
    /// Monolito, mover el Monolito, o dejar de cubrir la parte derecha del
    /// Rompeolas —que es justo lo que le da apoyo—. Queda anotado para quien
    /// decida.
    ///
    /// # Qué no toca
    ///
    /// El plinto, el Monolito, `A-01`, Praderas, Aguas, el resto del arco
    /// costero y las treinta y ocho piezas del Rompeolas. El conteo sigue en
    /// `154` y no aparece ninguna primitiva, material ni overlay.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_terraza_verde_de_borde() -> Blockout {
        let mut diorama = nivel_apoyado();

        let losa = masa_pedestal(&diorama.scene);
        let caja = diorama.scene.objects[losa].primitive.bounds();

        let plinto = huella_del_grupo(&diorama.scene, SpatialGroupId::Global);
        let monolito = huella_del_grupo(&diorama.scene, SpatialGroupId::Monolith);
        let rompeolas = huella_del_grupo(&diorama.scene, SpatialGroupId::Breakwater);

        // Hacia `−Z`: hasta la cara delantera del Monolito, con holgura. Y
        // nunca más allá de donde empieza el Rompeolas, porque la terraza
        // tiene que seguir cubriéndolo.
        let fondo = (monolito.max.z + HOLGURA_TERRAZA).min(rompeolas.min.z);

        // Hacia el frente: hasta el borde del plinto, con su margen.
        let frente = (plinto.max.z - MARGEN_DEL_PLINTO).max(rompeolas.max.z);

        diorama.scene.objects[losa].primitive = Cuboid::new(Aabb::new(
            Vec3::new(caja.min.x, caja.min.y, fondo.min(caja.min.z)),
            Vec3::new(caja.max.x, caja.max.y, frente.max(caja.max.z)),
        ))
        .into();

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel =
            SceneAccel::build(&diorama.scene).expect("la escena de la terraza tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Cadena de plataformas verdes
    // ------------------------------------------------------------------

    /// Cuántas plataformas forman la cadena, contando la principal.
    const PLATAFORMAS_DE_LA_CADENA: usize = 3;

    /// Claro que se deja entre una plataforma y la siguiente.
    ///
    /// Es lo que impide que las tres se lean como una placa: el suelo del
    /// plinto tiene que verse entre ellas.
    const CLARO_ENTRE_PLATAFORMAS: f32 = 0.60;

    /// Cuánto baja cada plataforma respecto de la anterior.
    const ESCALON_DE_PLATAFORMA: f32 = 0.30;

    /// Las tres masas de `G-02` que forman la cadena, de mayor a menor
    /// huella.
    ///
    /// La primera es la que `grounded` convirtió en losa del Rompeolas y
    /// `green_shelf` prolongó; las otras dos son las siguientes por tamaño
    /// en planta. Todo medido, sin índices escritos a mano.
    ///
    /// **Se calcula sobre la escena de partida.** Después de
    /// redimensionarlas su huella cambia y el orden por tamaño ya no sería
    /// el mismo: quien quiera saber cuáles son se lo pregunta a
    /// `green_shelf`, no al resultado.
    pub fn plataformas_de_borde(scene: &Scene) -> Vec<usize> {
        let area = |i: usize| {
            let caja = scene.objects[i].primitive.bounds();

            (caja.max.x - caja.min.x) * (caja.max.z - caja.min.z)
        };

        let mut orden = indices_por_grupo(scene, SpatialGroupId::ContinentBackground);
        orden.sort_by(|a, b| {
            area(*b)
                .partial_cmp(&area(*a))
                .expect("no hay NaN en el arco costero")
        });
        orden.truncate(PLATAFORMAS_DE_LA_CADENA);

        orden
    }

    /// `green_shelf` convertido en una **cadena** de plataformas verdes.
    ///
    /// # Qué corrige
    ///
    /// `green_shelf` intentó prolongar una losa única y se estrelló contra
    /// el Monolito: una caja que cubra el Rompeolas está dentro de su
    /// abscisa, y no puede pasar de su cara delantera sin tragárselo. La
    /// salida no era estirar más esa caja sino **usar otras**.
    ///
    /// # La cadena
    ///
    /// Tres plataformas, todas masas de `G-02` que ya existían:
    ///
    /// | | dónde | qué hace |
    /// |---|---|---|
    /// | principal | junto a Aguas | la de `green_shelf`, intacta: sostiene el Rompeolas |
    /// | segunda | al costado del Monolito | primer tramo del corredor |
    /// | tercera | por detrás, hacia Praderas | último tramo, hasta el claro |
    ///
    /// Las dos nuevas se colocan al **oeste** del Monolito, que es el único
    /// lado por donde se puede bajar hacia Praderas sin cortarlo, y bajan un
    /// escalón cada una: `1.20`, `0.90`, `0.60` de cota. El corredor entre
    /// el frente de la principal y el claro de Praderas se reparte entre las
    /// dos dejando un hueco de `0.60` entre ellas y otro contra la
    /// principal, así que entre plataforma y plataforma se ve el lienzo. Eso
    /// es lo que las hace una cadena y no una placa.
    ///
    /// # Qué no toca
    ///
    /// Ni un prisma: `R-01`, `R-02` y `R-03` quedan exactos, igual que el
    /// plinto, el Monolito, `A-01`, Praderas y Aguas. Las dos masas que se
    /// mueven conservan su material, su grupo espacial y su grupo de
    /// revelación; sólo cambia su caja. El conteo sigue en `154` y no
    /// aparece ninguna primitiva ni material nuevo.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_plataformas_de_borde() -> Blockout {
        let mut diorama = nivel_terraza_verde_de_borde();

        let plataformas = plataformas_de_borde(&diorama.scene);
        let principal = diorama.scene.objects[plataformas[0]].primitive.bounds();

        let plinto = huella_del_grupo(&diorama.scene, SpatialGroupId::Global);
        let monolito = huella_del_grupo(&diorama.scene, SpatialGroupId::Monolith);
        let praderas = huella_del_grupo(&diorama.scene, SpatialGroupId::Meadows);

        // El corredor que queda por poblar, en `z`: del frente de la
        // principal al claro de Praderas, descontando el hueco que se le
        // deja a la principal.
        let desde = praderas.max.z + CLARO;
        let hasta = principal.min.z - CLARO_ENTRE_PLATAFORMAS;
        let fondo = ((hasta - desde) - CLARO_ENTRE_PLATAFORMAS) * 0.5;

        // En `x`, al oeste del Monolito: el único lado por el que se baja
        // hacia Praderas sin cortarlo.
        let izquierda = plinto.min.x + MARGEN_DEL_PLINTO;
        let derecha = monolito.min.x - CLARO;
        let ancho = derecha - izquierda;

        // La segunda es ancha y pegada a la principal; la tercera es más
        // corta y se mete hacia Praderas. Que no midan lo mismo es lo que
        // las hace dos plataformas y no dos mitades.
        let tramos = [
            (izquierda, derecha, hasta - fondo, hasta),
            (
                izquierda + ancho * 0.25,
                derecha - ancho * 0.05,
                desde,
                desde + fondo,
            ),
        ];

        for (paso, &(x0, x1, z0, z1)) in tramos.iter().enumerate() {
            let indice = plataformas[paso + 1];
            let techo = principal.max.y - ESCALON_DE_PLATAFORMA * (paso + 1) as f32;

            diorama.scene.objects[indice].primitive = Cuboid::new(Aabb::new(
                Vec3::new(x0, plinto.max.y, z0),
                Vec3::new(x1, techo, z1),
            ))
            .into();
        }

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel = SceneAccel::build(&diorama.scene)
            .expect("la escena de las plataformas tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Plataformas finas y toma cenital
    // ------------------------------------------------------------------

    /// Qué parte de su ancho transversal conservan las dos cornisas.
    ///
    /// `0.45`: se quedan en poco menos de la mitad. Adelgazan **en `x`**,
    /// que es lo transversal al corredor; su tramo en `z` y por tanto los
    /// claros entre plataformas no se tocan.
    const ANCHO_DE_CORNISA: f32 = 0.45;

    /// Lo que le sobra a la plataforma principal por cada lado sobre la
    /// huella del Rompeolas.
    ///
    /// Es lo único que puede adelgazar: tiene que seguir conteniendo la
    /// huella entera, o los prismas se quedan sin apoyo.
    const VUELO_DE_LA_PRINCIPAL: f32 = 0.10;

    /// Elevación de la toma cenital, en grados.
    ///
    /// No `90°`. En el polo la base de la cámara se vuelve degenerada —por
    /// eso `Camera::orbit` recorta antes de llegar— y este ángulo se queda
    /// bien por encima de la toma hero, que va a `35°`, sin acercarse a él.
    pub const ELEVACION_CENITAL: f32 = 78.0;

    /// Cámara cenital de un diorama.
    ///
    /// Mismo centro de órbita, mismo punto de mira y mismo campo que la toma
    /// hero, y el **mismo radio derivado** de la escala medida: lo único que
    /// cambia es la elevación del ojo, de `35°` a `78°`. Sale de `eye_at`,
    /// la misma función que coloca la hero, así que no es un encuadre
    /// inventado ni una captura de ventana: es la misma cámara mirando desde
    /// arriba.
    pub fn camara_cenital(diorama: &Blockout) -> Camera {
        let ojo = eye_at(
            diorama.anchors.orbit_center,
            diorama.scale.orbit_radius,
            HERO_YAW_DEGREES,
            ELEVACION_CENITAL,
        );

        Camera::new(
            ojo,
            diorama.anchors.orbit_center,
            diorama.anchors.look_at,
            Vec3::new(0.0, 1.0, 0.0),
            (2.0 * HALF_VERTICAL_FOV_DEGREES).to_radians(),
        )
        .with_radius_limits(
            diorama.scale.scene_radius * MIN_RADIUS_FACTOR,
            diorama.scale.scene_radius * MAX_RADIUS_FACTOR,
        )
    }

    /// La cadena de plataformas, adelgazada.
    ///
    /// # Qué corrige
    ///
    /// La revisión las vio demasiado anchas. Adelgazan **en `x`**, que es lo
    /// transversal al corredor: el tramo que cada una ocupa en `z` no se
    /// toca, así que la ruta hacia Praderas y los claros entre plataformas
    /// son exactamente los mismos. El espesor tampoco cambia.
    ///
    /// | | antes | después |
    /// |---|---|---|
    /// | principal | `12.00` de ancho | lo justo para cubrir el Rompeolas más `0.10` por lado |
    /// | segunda | `8.60` | `45 %` |
    /// | tercera | `6.02` | `45 %` |
    ///
    /// Las dos cornisas adelgazan **por su lado oeste** y conservan el
    /// borde que da al Monolito: es el que marca el corredor, y moverlo
    /// cambiaría la ruta en vez de afinarla.
    ///
    /// La principal no puede adelgazar más: tiene que seguir conteniendo la
    /// huella del Rompeolas o sus treinta y ocho piezas se quedan sin apoyo.
    /// Lo que sí gana es margen con Aguas.
    ///
    /// # Qué no toca
    ///
    /// Ni un prisma, ni el plinto, ni el Monolito, ni `A-01`, ni Praderas,
    /// ni Aguas. Sólo cambian las cajas de las tres plataformas, que
    /// conservan material y grupos. El conteo sigue en `154`.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_plataformas_finas() -> Blockout {
        let mut diorama = nivel_plataformas_de_borde();

        let plataformas = plataformas_de_borde(&nivel_terraza_verde_de_borde().scene);
        let rompeolas = huella_del_grupo(&diorama.scene, SpatialGroupId::Breakwater);

        for (n, &i) in plataformas.iter().enumerate() {
            let caja = diorama.scene.objects[i].primitive.bounds();

            let (x0, x1) = if n == 0 {
                // La principal: lo justo para cubrir el Rompeolas.
                (
                    (rompeolas.min.x - VUELO_DE_LA_PRINCIPAL).min(caja.min.x),
                    (rompeolas.max.x + VUELO_DE_LA_PRINCIPAL).max(caja.min.x),
                )
            } else {
                // Las cornisas: conservan el borde que da al Monolito y
                // recortan por el oeste.
                (
                    caja.max.x - (caja.max.x - caja.min.x) * ANCHO_DE_CORNISA,
                    caja.max.x,
                )
            };

            diorama.scene.objects[i].primitive = Cuboid::new(Aabb::new(
                Vec3::new(x0, caja.min.y, caja.min.z),
                Vec3::new(x1.min(caja.max.x), caja.max.y, caja.max.z),
            ))
            .into();
        }

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel = SceneAccel::build(&diorama.scene)
            .expect("la escena de las plataformas finas tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Módulos de la parcela marcada
    // ------------------------------------------------------------------

    /// Lo que se leyó de la marca roja en `exp33/platforms_thin_top.png`.
    ///
    /// Tres cosas, y sólo tres, porque son las que la imagen deja ver:
    ///
    /// 1. **Es un rectángulo estrecho y alto.** En la toma cenital, con el
    ///    yaw hero, la vertical de la pantalla es el eje `z` y la horizontal
    ///    el eje `x`: la marca pide fondo mucho mayor que anchura.
    /// 2. **Está en el corredor oeste.** Cae a la izquierda del Monolito
    ///    —la torre pálida del centro— y por delante de la masa verde del
    ///    fondo, que es Praderas.
    /// 3. **Encierra una pieza suelta, no una franja.** Lo marcado es un
    ///    módulo aislado; la cornisa larga que lo cruzaba es justo lo que
    ///    sobraba.
    ///
    /// Lo que la marca **no** dice y por tanto no se inventa: cuántos
    /// módulos, su altura y su separación exacta.
    const _MARCA: () = ();

    /// Proporción de un módulo: su anchura en `x` como fracción de su fondo
    /// en `z`. `0.40` deja un rectángulo dos veces y media más largo que
    /// ancho.
    const ESBELTEZ_DEL_MODULO: f32 = 0.40;

    /// Hueco entre los dos módulos, en unidades de mundo.
    const HUECO_ENTRE_MODULOS: f32 = 1.30;

    /// Cuánto baja cada módulo respecto de la plataforma principal.
    const ESCALON_DEL_MODULO: f32 = 0.25;

    /// Qué parte del corredor ocupa el módulo pequeño.
    const MODULO_CORTO: f32 = 0.75;

    /// `grounded` con las plataformas verdes en la lógica de la parcela
    /// marcada.
    ///
    /// # Qué corrige
    ///
    /// `platforms_thin` dejó dos cornisas largas que seguían el borde de
    /// lado a lado. La marca pide lo contrario: **módulos compactos**,
    /// estrechos en `x` y alargados en `z`, sueltos en el corredor oeste
    /// entre el Monolito y Praderas. Islas de soporte territorial, no una
    /// franja.
    ///
    /// # Las tres piezas
    ///
    /// Todas son masas de `G-02` que ya existían, elegidas por su huella:
    ///
    /// | | dónde | forma |
    /// |---|---|---|
    /// | principal | bajo el Rompeolas | lo justo para cubrirlo, más `0.10` por lado |
    /// | módulo largo | corredor oeste | todo el fondo del corredor |
    /// | módulo corto | corredor oeste, más al este | `75 %` de ese fondo |
    ///
    /// Los dos módulos arrancan en el claro de Praderas y suben hacia la
    /// plataforma principal, dejándole su propio hueco. Entre ellos queda
    /// `1.30` de lienzo: es lo que los hace dos islas y no una pieza con una
    /// muesca.
    ///
    /// La principal **no** cumple la proporción de la marca, y no puede:
    /// tiene que cubrir la huella del Rompeolas, que mide `11.38` de ancho
    /// por `5.98` de fondo. La regla de esbeltez es de los módulos.
    ///
    /// # Qué no toca
    ///
    /// Ni un prisma: `R-01`, `R-02` y `R-03` quedan exactos, igual que el
    /// plinto, el Monolito, `A-01`, Praderas y Aguas. Las tres masas
    /// conservan material y grupos; sólo cambia su caja. El conteo sigue en
    /// `154` y no aparece ninguna primitiva ni material nuevo.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_plataformas_marcadas() -> Blockout {
        let mut diorama = nivel_apoyado();

        let plataformas = plataformas_de_borde(&diorama.scene);
        let principal = diorama.scene.objects[plataformas[0]].primitive.bounds();

        let plinto = huella_del_grupo(&diorama.scene, SpatialGroupId::Global);
        let monolito = huella_del_grupo(&diorama.scene, SpatialGroupId::Monolith);
        let praderas = huella_del_grupo(&diorama.scene, SpatialGroupId::Meadows);
        let rompeolas = huella_del_grupo(&diorama.scene, SpatialGroupId::Breakwater);

        // --- la principal: lo justo para cubrir el macizo
        diorama.scene.objects[plataformas[0]].primitive = Cuboid::new(Aabb::new(
            Vec3::new(
                (rompeolas.min.x - VUELO_DE_LA_PRINCIPAL).min(principal.min.x),
                principal.min.y,
                principal.min.z,
            ),
            Vec3::new(
                (rompeolas.max.x + VUELO_DE_LA_PRINCIPAL).max(principal.min.x),
                principal.max.y,
                principal.max.z,
            ),
        ))
        .into();

        // --- el corredor de la marca, medido
        //
        // De borde a borde: del claro de Praderas al hueco que se le deja a
        // la principal, y al oeste del Monolito con su claro.
        let fondo_del_corredor =
            (principal.min.z - CLARO_ENTRE_PLATAFORMAS) - (praderas.max.z + CLARO);
        let arranque = praderas.max.z + CLARO;
        let borde_este = monolito.min.x - CLARO;

        // --- los dos módulos: estrechos en `x`, largos en `z`
        let modulos = [
            // El corto, pegado al Monolito.
            (
                fondo_del_corredor * MODULO_CORTO,
                borde_este,
                ESCALON_DEL_MODULO * 2.0,
            ),
            // El largo, un hueco más al oeste. Su abscisa se calcula
            // después, cuando se sabe lo que ocupó el primero.
            (fondo_del_corredor, 0.0, ESCALON_DEL_MODULO),
        ];

        let mut derecha = borde_este;

        for (paso, &(fondo, borde, escalon)) in modulos.iter().enumerate() {
            let ancho = fondo * ESBELTEZ_DEL_MODULO;
            let x1 = if paso == 0 { borde } else { derecha };
            let x0 = (x1 - ancho).max(plinto.min.x + MARGEN_DEL_PLINTO);

            diorama.scene.objects[plataformas[paso + 1]].primitive = Cuboid::new(Aabb::new(
                Vec3::new(x0, plinto.max.y, arranque),
                Vec3::new(x1, principal.max.y - escalon, arranque + fondo),
            ))
            .into();

            derecha = x0 - HUECO_ENTRE_MODULOS;
        }

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel = SceneAccel::build(&diorama.scene)
            .expect("la escena de las plataformas marcadas tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Soporte modular
    // ------------------------------------------------------------------

    /// Hueco entre módulos del soporte, en unidades de mundo.
    const HUECO_MODULAR: f32 = 1.40;

    /// Cuánto se acorta cada módulo por delante y por detrás respecto del
    /// anterior, para que los tres no midan lo mismo.
    const MERMA_MODULAR: f32 = 0.25;

    /// Cuánto baja la cota de cada módulo respecto del anterior.
    const ESCALON_MODULAR: f32 = 0.15;

    /// Holgura que se le deja a cada pieza dentro de su módulo.
    ///
    /// No es estética: sin ella el encaje deja la pieza **a ras** del borde
    /// del módulo, y entonces «el módulo la contiene» pasa a depender del
    /// último bit de un `f32`. La primera versión de este corte dejó así la
    /// pieza `83` y el test de apoyo la dio por flotando.
    const HOLGURA_EN_EL_MODULO: f32 = 0.02;

    /// `grounded` con el soporte del Rompeolas hecho **modular**.
    ///
    /// # Qué corrige
    ///
    /// `marked_platforms` puso dos módulos esbeltos en el corredor, pero
    /// dejó intacta la megaplataforma: una losa de `11.58 × 7.00` bajo todo
    /// el macizo. La lectura que pide la marca no es «dos módulos al lado de
    /// una losa», es que **el soporte entero** sea modular.
    ///
    /// # Los tres módulos
    ///
    /// Las mismas tres masas de `G-02` que ya hacían de plataformas, ahora
    /// las tres con la forma de la parcela marcada: estrechas en `x`,
    /// largas en `z`. Se reparten la anchura que ocupaba la losa dejando
    /// `1.40` de lienzo entre una y otra, y cada una es un poco menos
    /// profunda y un poco más baja que la anterior, de oeste a este.
    ///
    /// Ninguna llega a `3.50` de ancho —la losa medía `12.00`— y las tres
    /// tienen el fondo por encima del doble de su anchura.
    ///
    /// # Qué pasa con las treinta y ocho piezas
    ///
    /// Se quedan donde están salvo lo estrictamente necesario: cada una
    /// busca el módulo **más cercano que pueda contenerla** y se desplaza en
    /// `x` lo mínimo para entrar en él; después se posa en su cota. Las que
    /// no caben en ninguno —los soportes de `R-03`, que son losas de hasta
    /// `6.4` de ancho— se posan en el plinto.
    ///
    /// No se toca la profundidad de ninguna: el orden de la formación en
    /// `z` es el que aprobó la revisión, y moverlo la desarmaría. Sólo se
    /// ajustan la abscisa y la cota, que es lo que el soporte obliga.
    ///
    /// # Qué no toca
    ///
    /// El plinto, el Monolito, `A-01`, Praderas y Aguas. Materiales y grupos
    /// de las tres masas de `G-02` y de las treinta y ocho piezas. El conteo
    /// sigue en `154` y no aparece ninguna primitiva ni material nuevo.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_soporte_modular() -> Blockout {
        let mut diorama = nivel_apoyado();

        let plataformas = plataformas_de_borde(&diorama.scene);
        let losa = diorama.scene.objects[plataformas[0]].primitive.bounds();

        let plinto = huella_del_grupo(&diorama.scene, SpatialGroupId::Global);
        let aguas = huella_del_grupo(&diorama.scene, SpatialGroupId::FlyingWaters);
        let rompeolas = huella_del_grupo(&diorama.scene, SpatialGroupId::Breakwater);

        // El corredor que ocupaba la losa, medido de borde a borde de la
        // formación y recortado por el claro con Aguas.
        let izquierda = rompeolas.min.x - VUELO_DE_LA_PRINCIPAL;
        let derecha = (rompeolas.max.x + VUELO_DE_LA_PRINCIPAL).min(aguas.min.x - CLARO);
        let ancho_modulo = (derecha - izquierda - HUECO_MODULAR * (plataformas.len() - 1) as f32)
            / plataformas.len() as f32;

        let mut cajas_modulos: Vec<Aabb> = Vec::with_capacity(plataformas.len());

        for (k, &i) in plataformas.iter().enumerate() {
            let merma = MERMA_MODULAR * k as f32;
            let x0 = izquierda + (ancho_modulo + HUECO_MODULAR) * k as f32;

            let caja = Aabb::new(
                Vec3::new(x0, plinto.max.y, losa.min.z + merma),
                Vec3::new(
                    x0 + ancho_modulo,
                    losa.max.y - ESCALON_MODULAR * k as f32,
                    losa.max.z - merma,
                ),
            );

            diorama.scene.objects[i].primitive = Cuboid::new(caja).into();
            cajas_modulos.push(caja);
        }

        // Cada pieza busca el módulo más cercano que pueda contenerla.
        let piezas = indices_por_grupo(&diorama.scene, SpatialGroupId::Breakwater);

        for &i in &piezas {
            let caja = diorama.scene.objects[i].primitive.bounds();
            let medio = (caja.max.x - caja.min.x) * 0.5;
            let centro = (caja.min.x + caja.max.x) * 0.5;

            let mut mejor: Option<(f32, f32)> = None;

            for modulo in &cajas_modulos {
                let cabe_en_x = modulo.max.x - modulo.min.x
                    >= caja.max.x - caja.min.x + HOLGURA_EN_EL_MODULO * 2.0;
                let cabe_en_z = modulo.min.z <= caja.min.z - HOLGURA_EN_EL_MODULO
                    && modulo.max.z >= caja.max.z + HOLGURA_EN_EL_MODULO;

                if !cabe_en_x || !cabe_en_z {
                    continue;
                }

                let destino = centro.clamp(
                    modulo.min.x + medio + HOLGURA_EN_EL_MODULO,
                    modulo.max.x - medio - HOLGURA_EN_EL_MODULO,
                );
                let salto = destino - centro;

                if mejor.is_none_or(|(actual, _)| salto.abs() < actual.abs()) {
                    mejor = Some((salto, modulo.max.y));
                }
            }

            // Lo que no cabe en ningún módulo —las losas anchas de `R-03`—
            // se posa en el lienzo.
            let (salto, techo) = mejor.unwrap_or((0.0, plinto.max.y));
            let delta = Vec3::new(salto, (techo - EMPOTRADO) - caja.min.y, 0.0);

            diorama.scene.objects[i].primitive =
                trasladada(&diorama.scene.objects[i].primitive, delta);
        }

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel = SceneAccel::build(&diorama.scene)
            .expect("la escena del soporte modular tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Segunda isla verde
    // ------------------------------------------------------------------

    /// Lo que se le deja al borde oeste del plinto por detrás de la isla.
    ///
    /// La isla no llega de pared a pared: si tocara el canto del lienzo
    /// dejaría de leerse como una isla y pasaría a ser una repisa del
    /// plinto.
    const RESPIRO_OESTE: f32 = 1.40;

    /// La parcela marcada, medida: el rectángulo libre entre la base
    /// principal, el Monolito y Praderas.
    ///
    /// Es el hueco que la marca roja de `exp33/platforms_thin_top.png`
    /// encierra. Sus cuatro bordes salen de cajas, no de números escritos:
    ///
    /// | borde | contra qué |
    /// |---|---|
    /// | `+z` | la base principal, menos un claro |
    /// | `−z` | Praderas, más un claro |
    /// | `+x` | el Monolito, menos un claro |
    /// | `−x` | el canto del plinto, más su margen y un respiro |
    pub fn parcela_marcada(scene: &Scene) -> Aabb {
        let principal = scene.objects[masa_pedestal(scene)].primitive.bounds();
        let plinto = huella_del_grupo(scene, SpatialGroupId::Global);
        let monolito = huella_del_grupo(scene, SpatialGroupId::Monolith);
        let praderas = huella_del_grupo(scene, SpatialGroupId::Meadows);

        Aabb::new(
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
        )
    }

    /// La masa de `G-02` que se convierte en la segunda isla.
    ///
    /// Se elige midiendo: de todas las masas del arco costero, **sin contar
    /// la que ya sostiene el Rompeolas**, la que tiene su centro más cerca
    /// de la parcela marcada. Es la que menos hay que arrastrar, y la que
    /// menos terreno deja huérfano donde estaba.
    pub fn masa_de_la_segunda_isla(scene: &Scene) -> usize {
        let parcela = parcela_marcada(scene);
        let destino = (
            (parcela.min.x + parcela.max.x) * 0.5,
            (parcela.min.z + parcela.max.z) * 0.5,
        );
        let principal = masa_pedestal(scene);

        let distancia = |i: usize| {
            let caja = scene.objects[i].primitive.bounds();
            let centro = (
                (caja.min.x + caja.max.x) * 0.5,
                (caja.min.z + caja.max.z) * 0.5,
            );

            (centro.0 - destino.0).powi(2) + (centro.1 - destino.1).powi(2)
        };

        indices_por_grupo(scene, SpatialGroupId::ContinentBackground)
            .into_iter()
            .filter(|&i| i != principal)
            .fold(None, |mejor: Option<usize>, i| match mejor {
                Some(m) if distancia(m) <= distancia(i) => Some(m),
                _ => Some(i),
            })
            .expect("G-02 tiene mas masas que la principal")
    }

    /// `grounded` con una **segunda isla verde** en la parcela marcada.
    ///
    /// # Qué hace, y qué no
    ///
    /// No toca `grounded`: la plataforma verde original se queda exactamente
    /// como está, con sus treinta y ocho piezas y sus soportes. Lo único que
    /// cambia es **una** masa de `G-02` —la que ya estaba más cerca del
    /// sitio—, que se reubica y se redimensiona para ocupar la parcela
    /// entera.
    ///
    /// No son barras ni módulos: es **una sola pieza compacta** con
    /// superficie de sobra para que en el futuro se le monte encima una
    /// montaña o una escalera. La proporción no se fuerza; lo que se busca
    /// es área útil.
    ///
    /// # Dónde cabe
    ///
    /// La parcela es lo que queda libre entre la base principal, el Monolito
    /// y Praderas, y la isla la ocupa entera dejando un claro de `1.00` por
    /// los tres lados y un respiro contra el canto del plinto por el cuarto.
    /// Ver `parcela_marcada`: sus cuatro bordes se miden.
    ///
    /// Su cota es la misma que la de la base original, y su base se posa en
    /// el plinto: son dos parcelas del mismo terreno, separadas por lienzo a
    /// la vista.
    ///
    /// # Qué no toca
    ///
    /// `R-01`, `R-02`, `R-03`, la plataforma principal, el plinto, el
    /// Monolito, `A-01`, Praderas y Aguas. El conteo sigue en `154` y no
    /// aparece ninguna primitiva ni material nuevo.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_segunda_isla() -> Blockout {
        let mut diorama = nivel_apoyado();

        let parcela = parcela_marcada(&diorama.scene);
        let elegida = masa_de_la_segunda_isla(&diorama.scene);

        diorama.scene.objects[elegida].primitive = Cuboid::new(parcela).into();

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel = SceneAccel::build(&diorama.scene)
            .expect("la escena de la segunda isla tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Isla de borde
    // ------------------------------------------------------------------

    /// Anchura de la isla de borde como fracción de su fondo.
    ///
    /// `0.42` la deja dos veces y media más larga que ancha. No se fija la
    /// anchura y se deduce el fondo, sino al revés: el fondo lo dicta el
    /// lienzo y la anchura lo sigue, que es lo que hace que la isla se
    /// afine cuando se alarga.
    const ESBELTEZ_DE_LA_ISLA: f32 = 0.42;

    /// `second_island` con la isla **alargada y arrimada al canto oeste**.
    ///
    /// # Qué corrige
    ///
    /// `second_island` puso la isla en el hueco que quedaba entre la base,
    /// el Monolito y Praderas. Ese hueco sólo da `2.64` de fondo, así que la
    /// isla salió ancha y poco profunda: `7.20 × 2.64`, justo lo contrario
    /// de lo que se pedía.
    ///
    /// # De dónde sale el fondo
    ///
    /// De correrla al oeste. Praderas ocupa `x ∈ [−3.47, 3.53]`: mientras la
    /// isla esté dentro de esa abscisa, su fondo lo limita el claro con la
    /// meseta. **En cuanto la isla queda al oeste de Praderas, el eje `z` se
    /// libera** y puede bajar hasta el canto sur del lienzo.
    ///
    /// Con eso el fondo pasa de `2.64` a más de `10`, y la anchura se deduce
    /// de él por la proporción: la isla se alarga y se afina a la vez.
    ///
    /// | | `second_island` | aquí |
    /// |---|---|---|
    /// | fondo `z` | `2.64` | del claro con la base al canto sur |
    /// | ancho `x` | `7.20` | `0.42 ×` el fondo |
    /// | sitio | pegada al Monolito | arrimada al canto oeste |
    ///
    /// # Los cuatro bordes, medidos
    ///
    /// - `+z`: la base principal, menos un claro.
    /// - `−z`: el canto sur del plinto, más su margen.
    /// - `−x`: el canto oeste del plinto, más su margen. Ahí es donde
    ///   bordea el lienzo.
    /// - `+x`: lo que dé la proporción, y nunca más al este de lo que
    ///   Praderas permite con su claro. El recorte está puesto: si algún día
    ///   la meseta se mueve, la isla se estrecha antes que invadirla.
    ///
    /// # Qué no toca
    ///
    /// La plataforma principal, `R-01`, `R-02`, `R-03`, el plinto, el
    /// Monolito, `A-01`, Praderas y Aguas. Sólo cambia la caja de la misma
    /// masa de `G-02` que ya era la isla: no hay una tercera. El conteo
    /// sigue en `154`.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_isla_de_borde() -> Blockout {
        let mut diorama = nivel_segunda_isla();

        // La eleccion se hace sobre `grounded`, **no** sobre la escena que
        // se esta modificando: `masa_de_la_segunda_isla` elige por cercania
        // a la parcela, y en cuanto la isla se mueve deja de ser la mas
        // cercana. Preguntarselo a la escena ya movida devolveria otra masa.
        let elegida = masa_de_la_segunda_isla(&nivel_apoyado().scene);
        let caja = diorama.scene.objects[elegida].primitive.bounds();

        let plinto = huella_del_grupo(&diorama.scene, SpatialGroupId::Global);
        let praderas = huella_del_grupo(&diorama.scene, SpatialGroupId::Meadows);
        let principal = diorama.scene.objects[masa_pedestal(&diorama.scene)]
            .primitive
            .bounds();

        // El fondo: del claro con la base al canto sur del lienzo.
        let z0 = plinto.min.z + MARGEN_DEL_PLINTO;
        let z1 = principal.min.z - CLARO;
        let fondo = z1 - z0;

        // La anchura sale del fondo, y se recorta si llegara a acercarse a
        // Praderas más de un claro.
        let x0 = plinto.min.x + MARGEN_DEL_PLINTO;
        let x1 = (x0 + fondo * ESBELTEZ_DE_LA_ISLA).min(praderas.min.x - CLARO);

        diorama.scene.objects[elegida].primitive = Cuboid::new(Aabb::new(
            Vec3::new(x0, caja.min.y, z0),
            Vec3::new(x1, caja.max.y, z1),
        ))
        .into();

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel = SceneAccel::build(&diorama.scene)
            .expect("la escena de la isla de borde tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Isla de borde ancha
    // ------------------------------------------------------------------

    /// `edge_island` ensanchada hasta el máximo que permite Praderas.
    ///
    /// # Qué cambia
    ///
    /// Una sola cosa: el borde este de la isla. Crece hacia `+x` hasta
    /// `praderas.min.x - CLARO`, que es el punto exacto donde la meseta
    /// deja de admitir más. El canto oeste, el tramo en `z` y el espesor
    /// quedan como los dejó `edge_island`.
    ///
    /// No hay cálculo de proporción aquí: la anchura ya no la decide una
    /// relación con el fondo, la decide **el obstáculo**. Es lo que se pidió
    /// —todo el territorio disponible para la escalera futura— y tiene la
    /// ventaja de que el límite se recalcula solo si Praderas se mueve.
    ///
    /// # Qué no toca
    ///
    /// La plataforma principal, `R-01`, `R-02`, `R-03`, el plinto, el
    /// Monolito, `A-01`, Praderas y Aguas. Sigue siendo la misma masa de
    /// `G-02`: no hay isla nueva. El conteo sigue en `154`.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_isla_de_borde_ancha() -> Blockout {
        let mut diorama = nivel_isla_de_borde();

        // Por la misma razon que en `nivel_isla_de_borde`: la eleccion sale
        // de `grounded`, donde la masa todavia esta en su sitio.
        let elegida = masa_de_la_segunda_isla(&nivel_apoyado().scene);
        let caja = diorama.scene.objects[elegida].primitive.bounds();
        let praderas = huella_del_grupo(&diorama.scene, SpatialGroupId::Meadows);

        // Hasta donde Praderas deja, y nunca hacia atrás: si algún día la
        // meseta se acercara, esto no encogeria la isla, la dejaria igual.
        let este = (praderas.min.x - CLARO).max(caja.max.x);

        diorama.scene.objects[elegida].primitive = Cuboid::new(Aabb::new(
            Vec3::new(caja.min.x, caja.min.y, caja.min.z),
            Vec3::new(este, caja.max.y, caja.max.z),
        ))
        .into();

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel =
            SceneAccel::build(&diorama.scene).expect("la escena de la isla ancha tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Isla larga con escalera
    // ------------------------------------------------------------------

    /// Cuánto se mete la isla dentro de la base principal.
    ///
    /// El solape con la base está autorizado; con nadie más. Tres unidades
    /// bastan para ganar fondo sin que la isla se coma la base entera y las
    /// dos dejen de leerse como dos piezas.
    const SOLAPE_CON_LA_BASE: f32 = 3.00;

    /// Holgura que se le deja a cada escalón dentro de la isla.
    const HOLGURA_EN_LA_ISLA: f32 = 0.10;

    /// Cuánto se aparta de lado cada escalón respecto del eje de la isla.
    const DESVIO_EN_LA_ISLA: f32 = 1.60;

    /// Semilla de la escalera de la isla.
    const SEMILLA_ISLA: u32 = 0x151A_5C41;

    /// Qué dos escalones intercambian su cota.
    ///
    /// Con seis escalones repartidos sobre un desnivel de cuatro unidades,
    /// cada paso sube unas `0.80` y el ruido tendría que pasar de eso para
    /// que dos vecinos se cruzaran. Fiarlo al sorteo salió mal: con esta
    /// semilla no se cruzaba ninguno y la subida era una rampa perfecta.
    ///
    /// Este intercambio lo garantiza **por construcción** y en las dos
    /// rutas: el tercer escalón remata más alto que el cuarto. Es el quiebro
    /// que convierte una rampa en una montaña.
    const QUIEBRO_DE_LA_ESCALERA: usize = 2;

    /// `edge_island_wide` alargada hasta meterse en la base, con una
    /// **montaña-escalera** encima.
    ///
    /// # La isla
    ///
    /// Crece sólo hacia `+z`, hasta `3.00` dentro de la base principal. Ese
    /// solape es el que la decisión autoriza y es el único que hay: a
    /// Praderas, a Aguas y al Monolito les sigue dejando su claro, y el
    /// canto oeste, el canto sur y la anchura no se tocan.
    ///
    /// Alargarla por ahí es lo único que quedaba: al sur la para el canto
    /// del lienzo y al este, Praderas.
    ///
    /// # La escalera
    ///
    /// Seis prismas de `R-01` —los más altos, medidos— suben por el eje de
    /// la isla de sur a norte. Los otros veintidós se quedan en el macizo,
    /// donde `grounded` los dejó, junto con `R-02` y `R-03`.
    ///
    /// A diferencia de los cortes anteriores, aquí **sí** se les cambia la
    /// altura: la isla es plana, así que el terreno no puede dibujar la
    /// subida y hay que darla. Cada prisma conserva su anchura y se
    /// reconstruye con `pieza`, el mismo constructor de la macroformación,
    /// así que la Ruta A sigue dando `HexPrism` y la B `Cuboid`.
    ///
    /// La subida va de `isla + 1.20` a **la mitad de la altura del
    /// Monolito**, con ruido: sube de conjunto y se cruza en las parejas
    /// vecinas. El tope no es decorativo — es la forma de no competir con
    /// él, y hay un test que lo mide.
    ///
    /// Los avances son desiguales y cada escalón se aparta de lado por su
    /// cuenta, recortado para que su huella caiga entera dentro de la isla:
    /// se apoya en ella y en nada más.
    ///
    /// # Qué no toca
    ///
    /// La base principal, el plinto, el Monolito, `A-01`, Praderas, Aguas,
    /// `R-02`, `R-03` y los veintidós prismas del macizo. El conteo sigue en
    /// `154` y no aparece ninguna primitiva ni material nuevo.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_isla_larga_con_escalera() -> Blockout {
        let mut diorama = nivel_isla_de_borde_ancha();

        let elegida = masa_de_la_segunda_isla(&nivel_apoyado().scene);
        let caja = diorama.scene.objects[elegida].primitive.bounds();
        let principal = diorama.scene.objects[masa_pedestal(&nivel_apoyado().scene)]
            .primitive
            .bounds();
        let monolito = huella_del_grupo(&diorama.scene, SpatialGroupId::Monolith);

        // --- la isla se alarga hacia la base
        let isla = Aabb::new(
            caja.min,
            Vec3::new(
                caja.max.x,
                caja.max.y,
                (principal.min.z + SOLAPE_CON_LA_BASE).min(principal.max.z),
            ),
        );

        diorama.scene.objects[elegida].primitive = Cuboid::new(isla).into();

        // --- la escalera sobre ella
        let escalones = cimas_seleccionadas(&nivel_apoyado().scene);
        let anchos: Vec<f32> = escalones
            .iter()
            .map(|&i| {
                let c = diorama.scene.objects[i].primitive.bounds();

                c.max.x - c.min.x
            })
            .collect();

        let eje = (isla.min.x + isla.max.x) * 0.5;
        let cota = isla.max.y - EMPOTRADO;
        let techo_bajo = isla.max.y + 0.80;
        let techo_alto = monolito.max.y * 0.5;

        let mut azar = Azar::new(SEMILLA_ISLA);

        let mut incrementos = vec![0.0_f32; escalones.len()];
        let mut total = 0.0_f32;

        for incremento in incrementos.iter_mut().skip(1) {
            *incremento = (1.0 + 0.55 * azar.simetrico()).max(0.25);
            total += *incremento;
        }

        let mut recorrido = vec![0.0_f32; escalones.len()];
        let mut acumulado = 0.0_f32;

        for (k, incremento) in incrementos.iter().enumerate().skip(1) {
            acumulado += incremento;
            recorrido[k] = acumulado / total;
        }

        // De sur a norte: nace en el canto del lienzo y sube hacia el macizo.
        let desde = isla.min.z + 1.00;
        let hasta = isla.max.z - 1.00;

        // Las cotas se sortean en el orden del recorrido y después dos
        // vecinas se intercambian: ver `QUIEBRO_DE_LA_ESCALERA`.
        let mut cotas: Vec<f32> = recorrido
            .iter()
            .map(|t| techo_bajo + (techo_alto - techo_bajo) * t.powf(0.9) + 0.65 * azar.simetrico())
            .collect();

        cotas.swap(QUIEBRO_DE_LA_ESCALERA, QUIEBRO_DE_LA_ESCALERA + 1);

        for (k, &i) in escalones.iter().enumerate() {
            let t = recorrido[k];
            let altura_objetivo = cotas[k];

            let medio_x = anchos[k] * 0.5;
            let x = (eje + DESVIO_EN_LA_ISLA * azar.simetrico()).clamp(
                isla.min.x + medio_x + HOLGURA_EN_LA_ISLA,
                isla.max.x - medio_x - HOLGURA_EN_LA_ISLA,
            );

            // El fondo del prisma depende de la ruta, así que se mide sobre
            // una construcción provisional antes de recortarlo en `z`.
            let tanteo = pieza(Vec3::new(x, 0.0, 0.0), anchos[k], 1.0).bounds();
            let medio_z = (tanteo.max.z - tanteo.min.z) * 0.5;
            let z = (desde + (hasta - desde) * t).clamp(
                isla.min.z + medio_z + HOLGURA_EN_LA_ISLA,
                isla.max.z - medio_z - HOLGURA_EN_LA_ISLA,
            );

            let altura = (altura_objetivo - cota).max(ALTURA_MINIMA);

            diorama.scene.objects[i].primitive =
                pieza(Vec3::new(x, cota + altura * 0.5, z), anchos[k], altura);
        }

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel =
            SceneAccel::build(&diorama.scene).expect("la escena de la isla larga tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Terraza de transición
    // ------------------------------------------------------------------

    /// Cuánto se levanta la terraza sobre la cota que comparten la base y la
    /// isla.
    ///
    /// Un escalón, no otra losa a ras: si rematara a `1.20` como las dos
    /// piezas que une, sería un trozo más de la misma superficie y no se
    /// leería como transición.
    const ESCALON_DE_TRANSICION: f32 = 0.45;

    /// Cuánto muerde la terraza dentro de la base y dentro de la isla.
    ///
    /// Tiene que solapar con las dos: es lo que encadena base → transición →
    /// isla en vez de dejar tres piezas sueltas.
    const MORDIDA_DE_LA_TRANSICION: f32 = 0.95;

    /// La zona donde la base y la isla se cruzan, que es donde va la
    /// terraza.
    ///
    /// Se mide sobre `grounded` y la isla larga: es la franja que queda
    /// entre el canto sur de la base y el arranque del Rompeolas, al este
    /// del flanco de la isla. Ahí caben las tres cosas sin tocar nada.
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

    /// La masa de `G-02` que se convierte en la terraza de transición.
    ///
    /// Se elige midiendo, igual que la isla: de las masas del arco costero,
    /// **sin contar la base ni la isla**, la que tiene su centro más cerca
    /// de la zona donde las dos se cruzan.
    pub fn masa_de_la_transicion(scene: &Scene) -> usize {
        let base = scene.objects[masa_pedestal(scene)].primitive.bounds();
        let isla = scene.objects[masa_de_la_segunda_isla(scene)]
            .primitive
            .bounds();
        let rompeolas = huella_del_grupo(scene, SpatialGroupId::Breakwater);
        let monolito = huella_del_grupo(scene, SpatialGroupId::Monolith);
        let zona = zona_de_transicion(&base, &isla, &rompeolas, &monolito);

        let destino = (
            (zona.min.x + zona.max.x) * 0.5,
            (zona.min.z + zona.max.z) * 0.5,
        );
        let excluidas = [masa_pedestal(scene), masa_de_la_segunda_isla(scene)];

        let distancia = |i: usize| {
            let caja = scene.objects[i].primitive.bounds();

            ((caja.min.x + caja.max.x) * 0.5 - destino.0).powi(2)
                + ((caja.min.z + caja.max.z) * 0.5 - destino.1).powi(2)
        };

        indices_por_grupo(scene, SpatialGroupId::ContinentBackground)
            .into_iter()
            .filter(|i| !excluidas.contains(i))
            .fold(None, |mejor: Option<usize>, i| match mejor {
                Some(m) if distancia(m) <= distancia(i) => Some(m),
                _ => Some(i),
            })
            .expect("G-02 tiene mas masas que la base y la isla")
    }

    /// `long_island_stair` con una **terraza de transición** entre la base y
    /// la isla.
    ///
    /// # Qué corrige
    ///
    /// La isla larga ya se mete dentro de la base, pero las dos rematan a la
    /// misma cota: en la zona de solape no hay junta que se vea, y el
    /// conjunto se lee como una **L verde** con un ángulo mecánico, no como
    /// una extensión del territorio.
    ///
    /// # La terraza
    ///
    /// Una tercera masa de `G-02` —ni la base ni la isla, elegida por
    /// cercanía a la zona donde las dos se cruzan— se coloca justo ahí, y
    /// **no a ras**: remata `0.45` por encima de la cota que comparten. Ese
    /// escalón es lo que da la secuencia base → transición → isla en vez de
    /// una superficie continua con una esquina.
    ///
    /// Muerde las dos piezas por construcción, así que no hay junta abierta:
    /// solapa con la base por el norte y con la isla por el oeste. Un test
    /// exige las dos cosas.
    ///
    /// Tampoco repite su forma: la isla es larga en `z` y la terraza lo es
    /// en `x`. Otra cota y otra orientación son lo que la distinguen de un
    /// rectángulo puente.
    ///
    /// # Dónde cabe
    ///
    /// Entre el canto sur de la base y el arranque del Rompeolas, al este
    /// del flanco de la isla y al oeste del Monolito con su claro. Es una
    /// franja estrecha —la marca el Rompeolas por un lado y el Monolito por
    /// el otro— y la terraza la ocupa entera.
    ///
    /// # Qué no toca
    ///
    /// La isla, la escalera de seis prismas, el macizo, `R-02`, `R-03`, la
    /// base, el plinto, el Monolito, `A-01`, Praderas y Aguas. El conteo
    /// sigue en `154`.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_isla_larga_con_transicion() -> Blockout {
        let mut diorama = nivel_isla_larga_con_escalera();

        let referencia = nivel_apoyado();
        let conector = masa_de_la_transicion(&referencia.scene);

        let base = diorama.scene.objects[masa_pedestal(&referencia.scene)]
            .primitive
            .bounds();
        let isla = diorama.scene.objects[masa_de_la_segunda_isla(&referencia.scene)]
            .primitive
            .bounds();
        // El Rompeolas de **`grounded`**, no el de esta escena: aqui seis de
        // sus prismas ya se fueron a la isla y su caja llega hasta el canto
        // sur del lienzo. Medir con ella arrastraba la zona de transicion
        // once unidades al sur, lejos de la base.
        let rompeolas = huella_del_grupo(&referencia.scene, SpatialGroupId::Breakwater);
        let monolito = huella_del_grupo(&referencia.scene, SpatialGroupId::Monolith);

        diorama.scene.objects[conector].primitive =
            Cuboid::new(zona_de_transicion(&base, &isla, &rompeolas, &monolito)).into();

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel =
            SceneAccel::build(&diorama.scene).expect("la escena de la transicion tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Escalera en racimos
    // ------------------------------------------------------------------

    /// Cuántos escalones lleva cada racimo, del más alto al más bajo.
    ///
    /// Tres parejas. Ninguna de una sola pieza: un racimo de uno es un
    /// poste, que es lo que la revisión pidió quitar.
    const RACIMOS_DE_LA_ESCALERA: [usize; 3] = [2, 2, 2];

    /// En qué racimo se intercambian las cotas de sus dos piezas.
    ///
    /// Igual que en `long_island_stair`: el cruce se declara en vez de
    /// esperarlo del sorteo. Aquí hace que dentro de ese racimo la pieza del
    /// norte remate **más baja** que la del sur, y eso es la bajada local
    /// que el conjunto necesita para no ser una rampa.
    const QUIEBRO_DEL_RACIMO: usize = 1;

    /// Separación entre las dos piezas de un racimo.
    ///
    /// Menos que su propio fondo, para que las cajas se toquen: es lo que
    /// convierte dos piezas en un racimo y no en dos vecinos.
    const APRETON_DEL_RACIMO: f32 = 1.35;

    /// Semilla de la escalera agrupada.
    const SEMILLA_RACIMOS: u32 = 0x2AC1_3005;

    /// `connected_island_stair` con los seis escalones **en racimos**.
    ///
    /// # Qué corrige
    ///
    /// La escalera de `long_island_stair` repartía los seis prismas a lo
    /// largo de la isla, uno por parada. Con gaps de más de una unidad entre
    /// todos, se leían como postes sueltos.
    ///
    /// # Los tres racimos
    ///
    /// | racimo | dónde | qué remata |
    /// |---|---|---|
    /// | alto | **sobre la terraza conectora** | la cota máxima |
    /// | medio | isla, tramo central | la cota media, con el quiebro |
    /// | bajo | isla, hacia el canto sur | la cota mínima |
    ///
    /// Dentro de cada racimo las dos piezas quedan a `1.35`, menos que su
    /// propio fondo, así que sus cajas se tocan. Entre racimos hay varias
    /// unidades: los huecos están donde separan, no entre compañeros.
    ///
    /// El racimo alto **nace en el conector**: sus dos piezas caben enteras
    /// dentro de la terraza, así que es ella quien las sostiene y no la isla
    /// que pasa por debajo. Se colocan una al lado de otra en `x` porque la
    /// terraza es ancha y poco profunda; en la isla, que es lo contrario,
    /// los otros dos racimos se ordenan en `z`.
    ///
    /// # Qué no toca
    ///
    /// La base, el conector, la isla larga, el macizo, `R-02`, `R-03`, el
    /// plinto, el Monolito, `A-01`, Praderas y Aguas. Sólo cambian las seis
    /// piezas de la escalera. El conteo sigue en `154`.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_escalera_agrupada() -> Blockout {
        let mut diorama = nivel_isla_larga_con_transicion();

        let referencia = nivel_apoyado();
        let escalones = cimas_seleccionadas(&referencia.scene);
        let conector = diorama.scene.objects[masa_de_la_transicion(&referencia.scene)]
            .primitive
            .bounds();
        let isla = diorama.scene.objects[masa_de_la_segunda_isla(&referencia.scene)]
            .primitive
            .bounds();
        let monolito = huella_del_grupo(&referencia.scene, SpatialGroupId::Monolith);

        let anchos: Vec<f32> = escalones
            .iter()
            .map(|&i| {
                let c = diorama.scene.objects[i].primitive.bounds();

                c.max.x - c.min.x
            })
            .collect();

        // Las tres cotas: del conector al canto sur de la isla.
        let cota_alta = monolito.max.y * 0.5;
        let cota_baja = isla.max.y + 0.90;

        // Los anclajes de cada racimo. El primero es el centro de la
        // terraza; los otros dos, dos puntos del eje de la isla.
        let eje = (isla.min.x + isla.max.x) * 0.5;
        let fondo_isla = isla.max.z - isla.min.z;
        let anclas = [
            (
                (conector.min.x + conector.max.x) * 0.5,
                (conector.min.z + conector.max.z) * 0.5,
            ),
            (eje + 0.45, isla.max.z - fondo_isla * 0.45),
            (eje - 0.70, isla.max.z - fondo_isla * 0.78),
        ];

        let mut azar = Azar::new(SEMILLA_RACIMOS);
        let mut k = 0usize;

        for (racimo, &cuantos) in RACIMOS_DE_LA_ESCALERA.iter().enumerate() {
            let t = racimo as f32 / (RACIMOS_DE_LA_ESCALERA.len() - 1) as f32;
            let cota = cota_alta + (cota_baja - cota_alta) * t;
            let ancla = anclas[racimo];

            // Cotas de las dos piezas del racimo, y el quiebro declarado.
            let mut cotas: Vec<f32> = (0..cuantos)
                .map(|j| {
                    let lado = if j == 0 { 0.25 } else { -0.25 };

                    cota + lado + 0.20 * azar.simetrico()
                })
                .collect();

            if racimo == QUIEBRO_DEL_RACIMO {
                cotas.reverse();
            }

            for (j, &altura_objetivo) in cotas.iter().enumerate() {
                let paso = j as f32 - (cuantos - 1) as f32 * 0.5;
                let medio_x = anchos[k] * 0.5;

                // Sobre la terraza el racimo se abre en `x`; sobre la isla,
                // en `z`. Cada plataforma tiene su eje largo y el racimo lo
                // sigue.
                let (x, z, plataforma) = if racimo == 0 {
                    (ancla.0 + paso * APRETON_DEL_RACIMO, ancla.1, conector)
                } else {
                    (
                        ancla.0 + 0.30 * azar.simetrico(),
                        ancla.1 + paso * APRETON_DEL_RACIMO,
                        isla,
                    )
                };

                let tanteo = pieza(Vec3::new(0.0, 0.0, 0.0), anchos[k], 1.0).bounds();
                let medio_z = (tanteo.max.z - tanteo.min.z) * 0.5;

                let x = x.clamp(
                    plataforma.min.x + medio_x + HOLGURA_EN_LA_ISLA,
                    plataforma.max.x - medio_x - HOLGURA_EN_LA_ISLA,
                );
                let z = z.clamp(
                    plataforma.min.z + medio_z + HOLGURA_EN_LA_ISLA,
                    plataforma.max.z - medio_z - HOLGURA_EN_LA_ISLA,
                );

                let cota_del_suelo = plataforma.max.y - EMPOTRADO;
                let altura = (altura_objetivo - cota_del_suelo).max(ALTURA_MINIMA);

                diorama.scene.objects[escalones[k]].primitive = pieza(
                    Vec3::new(x, cota_del_suelo + altura * 0.5, z),
                    anchos[k],
                    altura,
                );

                k += 1;
            }
        }

        debug_assert_eq!(k, escalones.len(), "los racimos tienen que sumar seis");

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel = SceneAccel::build(&diorama.scene)
            .expect("la escena de la escalera agrupada tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Montaña en la isla
    // ------------------------------------------------------------------

    /// Cuántos prismas de `R-01` bajan a la isla y a su conector.
    ///
    /// Doce de veintiocho. `clustered_stair` movía seis y la revisión humana
    /// dijo que en la isla seguían leyéndose sueltos: seis piezas no hacen
    /// un macizo por muy juntas que se pongan. Con doce hay bastante para
    /// tres racimos con cuerpo, y los dieciséis que se quedan siguen siendo
    /// mayoría en la parcela del Rompeolas.
    const PRISMAS_DE_LA_MONTANA: usize = 12;

    /// Cuántos van a cada zona: pie, centro y salida.
    ///
    /// Menguante y no parejo. El pie —el extremo de la isla que se pega a la
    /// base original— es el que la revisión pidió poblar y se lleva la
    /// mitad; el centro lleva la cima con menos piezas; la salida es sólo un
    /// racimo de transición sobre la terraza conectora.
    const ZONAS_DE_LA_MONTANA: [usize; 3] = [6, 4, 2];

    /// Separación entre piezas vecinas dentro de un racimo.
    ///
    /// Menos que el fondo de una pieza, así que las cajas se tocan. Es lo
    /// que convierte un puñado de prismas en una masa: en `im1` el macizo no
    /// tiene huecos entre piezas, los huecos están **entre** masas.
    const APRETON_DE_LA_MONTANA: f32 = 1.15;

    /// Posiciones de la retícula de un racimo, en múltiplos del apretón.
    ///
    /// No es una rejilla: los seis puntos rodean al primero a distancias
    /// parecidas pero desiguales, para que el racimo se lea como un
    /// amontonamiento y no como una formación.
    const RETICULA_DEL_RACIMO: [(f32, f32); 6] = [
        (0.00, 0.00),
        (1.00, 0.18),
        (-0.95, 0.26),
        (0.48, -0.94),
        (-0.52, -1.02),
        (0.06, 1.05),
    ];

    /// Techo de cada zona, en fracciones de la altura del Monolito.
    ///
    /// Pie bajo, centro alto, salida intermedia. La progresión que pide la
    /// revisión —más piezas y menos altura abajo, menos piezas y más altura
    /// en el centro— está aquí y un test la mide sobre el resultado.
    const COTAS_DE_LA_MONTANA: [f32; 3] = [0.19, 0.48, 0.26];

    /// Escalón entre piezas consecutivas del mismo racimo.
    const ESCALON_DE_LA_MONTANA: f32 = 0.18;

    /// Ruido de altura dentro de un racimo.
    const RUIDO_DE_LA_MONTANA: f32 = 0.26;

    /// Cuánto se aparta cada pieza de su casilla de la retícula.
    const TEMBLOR_DE_LA_MONTANA: f32 = 0.18;

    /// Dónde cae el pie, medido desde el canto norte de la isla.
    ///
    /// La isla se mete `3.00` dentro de la base principal; a `0.13` de su
    /// fondo desde el norte, el racimo queda dentro de esa franja de solape.
    /// Eso es «pegado a la base original» sin salirse de la isla.
    const AVANCE_DEL_PIE: f32 = 0.13;

    /// Dónde cae el centro, medido desde el canto sur.
    ///
    /// La mitad exacta del eje largo. La cima va aquí y no en un canto,
    /// que es la otra mitad de la corrección.
    const AVANCE_DEL_CENTRO: f32 = 0.50;

    /// Semilla de la montaña.
    const SEMILLA_MONTANA: u32 = 0x3B0A_17C4;

    /// A qué distancia dos prismas de la montaña cuentan como del mismo
    /// racimo.
    const RACIMO_DE_LA_MONTANA: f32 = 2.20;

    /// Los doce prismas de `R-01` que forman la montaña, medidos.
    ///
    /// La regla es la de `cimas_seleccionadas` con otro corte: se ordena por
    /// el techo de cada caja y se toman los primeros. Así los seis que ya
    /// subían siguen estando y los otros seis salen del mismo criterio, no
    /// de una lista escrita a mano.
    pub fn prismas_de_la_montana(scene: &Scene) -> Vec<usize> {
        let piezas = indices_por_grupo(scene, SpatialGroupId::Breakwater);
        let techo = |i: usize| scene.objects[i].primitive.bounds().max.y;

        let mut orden = piezas[..PILARES_R01].to_vec();
        orden.sort_by(|a, b| {
            techo(*b)
                .partial_cmp(&techo(*a))
                .expect("no hay NaN en el Rompeolas")
        });
        orden.truncate(PRISMAS_DE_LA_MONTANA);
        orden.sort_unstable();

        orden
    }

    /// Los prismas de `R-01` que se quedan en el macizo basal.
    pub fn macizo_de_la_montana(scene: &Scene) -> Vec<usize> {
        let montana = prismas_de_la_montana(scene);

        indices_por_grupo(scene, SpatialGroupId::Breakwater)[..PILARES_R01]
            .iter()
            .copied()
            .filter(|i| !montana.contains(i))
            .collect()
    }

    /// Los racimos de la montaña, agrupados por cercanía en planta.
    ///
    /// Enlace simple sobre los centros: dos prismas son del mismo racimo si
    /// distan menos que `RACIMO_DE_LA_MONTANA`. Se agrupa **midiendo el
    /// resultado**, no leyendo el reparto del constructor, que es lo que
    /// permite que un test diga si de verdad hay tres masas o hay doce
    /// postes.
    pub fn racimos_de_la_montana(diorama: &Blockout) -> Vec<Vec<usize>> {
        let piezas = prismas_de_la_montana(&nivel_apoyado().scene);
        let centros: Vec<(f32, f32)> = piezas
            .iter()
            .map(|&i| {
                let c = diorama.scene.objects[i].primitive.bounds();

                ((c.min.x + c.max.x) * 0.5, (c.min.z + c.max.z) * 0.5)
            })
            .collect();

        let mut grupo: Vec<Option<usize>> = vec![None; centros.len()];
        let mut racimos: Vec<Vec<usize>> = Vec::new();

        for i in 0..centros.len() {
            if grupo[i].is_some() {
                continue;
            }

            let id = racimos.len();
            racimos.push(Vec::new());

            let mut pila = vec![i];
            grupo[i] = Some(id);

            while let Some(k) = pila.pop() {
                racimos[id].push(piezas[k]);

                for j in 0..centros.len() {
                    if grupo[j].is_some() {
                        continue;
                    }

                    let d = ((centros[k].0 - centros[j].0).powi(2)
                        + (centros[k].1 - centros[j].1).powi(2))
                    .sqrt();

                    if d < RACIMO_DE_LA_MONTANA {
                        grupo[j] = Some(id);
                        pila.push(j);
                    }
                }
            }

            racimos[id].sort_unstable();
        }

        racimos
    }

    /// Los tres racimos etiquetados: `[pie, centro, salida]`.
    ///
    /// Las etiquetas también se miden. El de la **salida** es el que cabe
    /// entero en la terraza conectora; de los otros dos, el que queda más al
    /// norte es el **pie** —el que mira a la base original— y el que queda
    /// al sur, el **centro**. Si la montaña no saliera en tres racimos, las
    /// listas quedarían cojas y el test que cuenta racimos lo diría.
    pub fn zonas_de_la_montana(diorama: &Blockout) -> [Vec<usize>; 3] {
        let referencia = nivel_apoyado();
        let conector = diorama.scene.objects[masa_de_la_transicion(&referencia.scene)]
            .primitive
            .bounds();

        let medio_z = |racimo: &[usize]| -> f32 {
            racimo
                .iter()
                .map(|&i| {
                    let c = diorama.scene.objects[i].primitive.bounds();

                    (c.min.z + c.max.z) * 0.5
                })
                .sum::<f32>()
                / racimo.len().max(1) as f32
        };

        let en_la_terraza = |racimo: &[usize]| -> bool {
            racimo.iter().all(|&i| {
                let c = diorama.scene.objects[i].primitive.bounds();

                conector.min.x <= c.min.x
                    && conector.max.x >= c.max.x
                    && conector.min.z <= c.min.z
                    && conector.max.z >= c.max.z
            })
        };

        let mut salida: Vec<usize> = Vec::new();
        let mut resto: Vec<Vec<usize>> = Vec::new();

        for racimo in racimos_de_la_montana(diorama) {
            if salida.is_empty() && en_la_terraza(&racimo) {
                salida = racimo;
            } else {
                resto.push(racimo);
            }
        }

        resto.sort_by(|a, b| {
            medio_z(b)
                .partial_cmp(&medio_z(a))
                .expect("no hay NaN en la montana")
        });

        [
            resto.first().cloned().unwrap_or_default(),
            resto.get(1).cloned().unwrap_or_default(),
            salida,
        ]
    }

    /// Encaja un centro dentro de una plataforma, dejando holgura.
    ///
    /// Si la pieza no cupiera, se centra en vez de recortarse:
    /// `f32::clamp` entra en pánico cuando el intervalo se invierte, y eso
    /// convertiría un cambio de tamaño en una caída del preview. Centrada,
    /// la pieza queda mal apoyada y es el test de apoyo quien lo dice, que
    /// es donde tiene que decirse.
    fn encajado(valor: f32, minimo: f32, maximo: f32) -> f32 {
        if minimo > maximo {
            (minimo + maximo) * 0.5
        } else {
            valor.clamp(minimo, maximo)
        }
    }

    /// `connected_island_stair` con una **montaña** sobre la isla en vez de
    /// una escalera de prismas sueltos.
    ///
    /// # Qué corrige
    ///
    /// La revisión humana sobre `clustered_stair` es que los postes sueltos
    /// problemáticos están en la isla larga, y que el reparto está al revés:
    /// el extremo pegado a la base original es el que hay que poblar, y la
    /// cima tiene que caer en la espina de la isla, no en un canto.
    ///
    /// # Las tres zonas
    ///
    /// | zona | prismas | dónde | cota |
    /// |---|---:|---|---|
    /// | pie | `6` | isla, franja que solapa la base | la más baja |
    /// | centro | `4` | mitad exacta del eje largo | la cima |
    /// | salida | `2` | terraza conectora | intermedia |
    ///
    /// Menguan y ganan altura en ese orden: seis piezas bajas y apretadas
    /// donde el territorio arranca, cuatro más altas en el centro, dos de
    /// transición en el conector. Es la lógica de macizo → escalones de
    /// `combined`, no seis prismas repartidos por una isla vacía.
    ///
    /// # Por qué son racimos y no postes
    ///
    /// Dentro de un racimo las piezas se colocan en una retícula de seis
    /// casillas separadas `1.15`, menos que el fondo de una pieza: las cajas
    /// se tocan. Entre racimos hay más de tres unidades. Un test agrupa los
    /// doce por cercanía sobre el resultado y exige exactamente tres grupos,
    /// ninguno de uno.
    ///
    /// # Qué no toca
    ///
    /// La base, el conector, la isla larga, `R-02`, `R-03`, el plinto, el
    /// Monolito, `A-01`, Praderas y Aguas. De `R-01` sólo cambian doce; los
    /// otros dieciséis se quedan donde `grounded` los dejó y siguen siendo
    /// el macizo basal —mayoría, y un bloque, los dos medidos—. El conteo
    /// sigue en `154` y no aparece ninguna primitiva ni material nuevo.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_montana_en_isla() -> Blockout {
        let mut diorama = nivel_isla_larga_con_transicion();

        let referencia = nivel_apoyado();
        let montana = prismas_de_la_montana(&referencia.scene);
        let isla = diorama.scene.objects[masa_de_la_segunda_isla(&referencia.scene)]
            .primitive
            .bounds();
        let conector = diorama.scene.objects[masa_de_la_transicion(&referencia.scene)]
            .primitive
            .bounds();
        let monolito = huella_del_grupo(&referencia.scene, SpatialGroupId::Monolith);

        let anchos: Vec<f32> = montana
            .iter()
            .map(|&i| {
                let c = diorama.scene.objects[i].primitive.bounds();

                c.max.x - c.min.x
            })
            .collect();

        let eje = (isla.min.x + isla.max.x) * 0.5;
        let fondo = isla.max.z - isla.min.z;

        // Los tres anclajes, cada uno con la plataforma que lo sostiene.
        let anclas = [
            (eje - 0.55, isla.max.z - fondo * AVANCE_DEL_PIE, isla),
            (eje, isla.min.z + fondo * AVANCE_DEL_CENTRO, isla),
            (
                (conector.min.x + conector.max.x) * 0.5,
                (conector.min.z + conector.max.z) * 0.5,
                conector,
            ),
        ];

        let mut azar = Azar::new(SEMILLA_MONTANA);
        let mut k = 0usize;

        for (zona, &cuantos) in ZONAS_DE_LA_MONTANA.iter().enumerate() {
            let (ancla_x, ancla_z, plataforma) = anclas[zona];

            // La retícula se centra en su ancla: así el racimo queda donde
            // se le pide y no colgando de su primera pieza.
            let casillas = &RETICULA_DEL_RACIMO[..cuantos];
            let medio = (
                casillas.iter().map(|c| c.0).sum::<f32>() / cuantos as f32,
                casillas.iter().map(|c| c.1).sum::<f32>() / cuantos as f32,
            );

            for (j, casilla) in casillas.iter().enumerate() {
                let medio_x = anchos[k] * 0.5;

                // El fondo del prisma depende de la ruta, así que se mide
                // sobre una construcción provisional.
                let tanteo = pieza(Vec3::new(0.0, 0.0, 0.0), anchos[k], 1.0).bounds();
                let medio_z = (tanteo.max.z - tanteo.min.z) * 0.5;

                let x = encajado(
                    ancla_x
                        + (casilla.0 - medio.0) * APRETON_DE_LA_MONTANA
                        + TEMBLOR_DE_LA_MONTANA * azar.simetrico(),
                    plataforma.min.x + medio_x + HOLGURA_EN_LA_ISLA,
                    plataforma.max.x - medio_x - HOLGURA_EN_LA_ISLA,
                );
                let z = encajado(
                    ancla_z
                        + (casilla.1 - medio.1) * APRETON_DE_LA_MONTANA
                        + TEMBLOR_DE_LA_MONTANA * azar.simetrico(),
                    plataforma.min.z + medio_z + HOLGURA_EN_LA_ISLA,
                    plataforma.max.z - medio_z - HOLGURA_EN_LA_ISLA,
                );

                let peldano = ESCALON_DE_LA_MONTANA * (j as f32 - (cuantos - 1) as f32 * 0.5);
                let techo = monolito.max.y * COTAS_DE_LA_MONTANA[zona]
                    + peldano
                    + RUIDO_DE_LA_MONTANA * azar.simetrico();

                let suelo = plataforma.max.y - EMPOTRADO;
                let altura = (techo - suelo).max(ALTURA_MINIMA);

                diorama.scene.objects[montana[k]].primitive =
                    pieza(Vec3::new(x, suelo + altura * 0.5, z), anchos[k], altura);

                k += 1;
            }
        }

        debug_assert_eq!(k, montana.len(), "las zonas tienen que sumar doce");

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel =
            SceneAccel::build(&diorama.scene).expect("la escena de la montana tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Isla estilo spacious
    // ------------------------------------------------------------------

    /// Cuánto tiene que sobresalir una pieza de Praderas por encima de su
    /// masa principal para contar como terraza.
    const SALTO_MINIMO_DE_TERRAZA: f32 = 0.05;

    /// Espesor mínimo de una terraza. Deja fuera las láminas de césped de
    /// `0.12`, que visten la meseta pero no la escalonan.
    const ESPESOR_MINIMO_DE_TERRAZA: f32 = 0.30;

    /// Huella mínima de una terraza, en fracción de la masa principal. Deja
    /// fuera los accesorios: piedras, postes y matas.
    const HUELLA_MINIMA_DE_TERRAZA: f32 = 0.10;

    /// La lógica compositiva de `spacious`, leída de su geometría.
    ///
    /// En `spacious`, Praderas es **una masa principal** —la losa mayor, de
    /// `7.0 × 5.6`— con **terrazas encima**: una grande que ocupa algo más
    /// de la mitad de cada eje y sube casi un metro, y dentro de ella otra
    /// más pequeña que sube un tercio de eso. Masa → terraza → terraza: un
    /// salto grande y después uno corto.
    ///
    /// Esto guarda esas relaciones y **no sus coordenadas**: proporciones de
    /// cada terraza respecto de lo que la sostiene, eje largo con eje largo,
    /// y los dos saltos de cota.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct LogicaSpacious {
        /// Terraza grande / masa principal, en el eje largo.
        pub largo_t1: f32,
        /// Terraza grande / masa principal, en el eje corto.
        pub corto_t1: f32,
        /// Terraza alta / terraza grande, en el eje largo.
        pub largo_t2: f32,
        /// Terraza alta / terraza grande, en el eje corto.
        pub corto_t2: f32,
        /// Cuánto sube la terraza grande sobre la masa.
        pub salto_t1: f32,
        /// Cuánto sube la terraza alta sobre la grande.
        pub salto_t2: f32,
    }

    fn largo_y_corto(caja: &Aabb) -> (f32, f32) {
        let x = caja.max.x - caja.min.x;
        let z = caja.max.z - caja.min.z;

        (x.max(z), x.min(z))
    }

    fn area_en_planta(caja: &Aabb) -> f32 {
        (caja.max.x - caja.min.x) * (caja.max.z - caja.min.z)
    }

    /// Lee la lógica de `spacious` sobre su escena.
    ///
    /// Todo se mide. La masa principal es la pieza de Praderas con más
    /// huella; las terrazas, las piezas que sobresalen de ella con espesor y
    /// huella de terraza. De esas, la de **más huella** es la grande y la de
    /// **más cota** es la alta. Con la escena vigente salen la losa de
    /// `4.4 × 3.4` que sube `0.90` y la de `2.8 × 1.8` que sube `0.35` más;
    /// el ejemplo lo imprime.
    pub fn logica_de_spacious(scene: &Scene) -> LogicaSpacious {
        let cajas: Vec<Aabb> = indices_por_grupo(scene, SpatialGroupId::Meadows)
            .iter()
            .map(|&i| scene.objects[i].primitive.bounds())
            .collect();

        let mayor_huella = |a: Aabb, b: Aabb| {
            if area_en_planta(&b) > area_en_planta(&a) {
                b
            } else {
                a
            }
        };

        let masa = cajas
            .iter()
            .copied()
            .reduce(mayor_huella)
            .expect("Praderas tiene piezas");

        let terrazas: Vec<Aabb> = cajas
            .iter()
            .copied()
            .filter(|c| {
                c.max.y > masa.max.y + SALTO_MINIMO_DE_TERRAZA
                    && c.max.y - c.min.y >= ESPESOR_MINIMO_DE_TERRAZA
                    && area_en_planta(c) >= area_en_planta(&masa) * HUELLA_MINIMA_DE_TERRAZA
            })
            .collect();

        let t1 = terrazas
            .iter()
            .copied()
            .reduce(mayor_huella)
            .expect("spacious tiene terrazas sobre Praderas");
        let t2 = terrazas
            .iter()
            .copied()
            .reduce(|a, b| if b.max.y > a.max.y { b } else { a })
            .expect("spacious tiene terrazas sobre Praderas");

        let (masa_l, masa_c) = largo_y_corto(&masa);
        let (t1_l, t1_c) = largo_y_corto(&t1);
        let (t2_l, t2_c) = largo_y_corto(&t2);

        LogicaSpacious {
            largo_t1: t1_l / masa_l,
            corto_t1: t1_c / masa_c,
            largo_t2: t2_l / t1_l,
            corto_t2: t2_c / t1_c,
            salto_t1: t1.max.y - masa.max.y,
            salto_t2: t2.max.y - t1.max.y,
        }
    }

    /// Las masas de `G-02` que nadie ve desde arriba: su huella cabe entera
    /// en la de otro objeto que remata más alto.
    ///
    /// Son las que se pueden reutilizar sin tocar lo que se ve. La base, la
    /// isla y el conector quedan fuera siempre. En `connected_island_stair`
    /// salen dos: la que la isla larga se tragó al crecer y la que queda
    /// bajo la meseta de Praderas. Un test comprueba con los rayos de la
    /// cámara cenital que ninguna recibía un solo impacto.
    ///
    /// De mayor a menor huella.
    pub fn masas_tapadas(scene: &Scene) -> Vec<usize> {
        let referencia = nivel_apoyado();
        let intocables = [
            masa_pedestal(&referencia.scene),
            masa_de_la_segunda_isla(&referencia.scene),
            masa_de_la_transicion(&referencia.scene),
        ];

        let mut tapadas: Vec<usize> = indices_por_grupo(scene, SpatialGroupId::ContinentBackground)
            .into_iter()
            .filter(|i| !intocables.contains(i))
            .filter(|&i| {
                let caja = scene.objects[i].primitive.bounds();

                scene.objects.iter().enumerate().any(|(j, otro)| {
                    let encima = otro.primitive.bounds();

                    j != i
                        && encima.max.y > caja.max.y
                        && encima.min.x <= caja.min.x
                        && encima.max.x >= caja.max.x
                        && encima.min.z <= caja.min.z
                        && encima.max.z >= caja.max.z
                })
            })
            .collect();

        tapadas.sort_by(|a, b| {
            let ca = scene.objects[*a].primitive.bounds();
            let cb = scene.objects[*b].primitive.bounds();

            area_en_planta(&cb)
                .partial_cmp(&area_en_planta(&ca))
                .expect("no hay NaN en G-02")
                .then(a.cmp(b))
        });

        tapadas
    }

    /// Cuántos rayos primarios de la cámara acaban en cada objeto.
    ///
    /// La misma rejilla de `100 × 75` que mide el agua, con los rayos del
    /// propio renderer: es lo que la toma ve, sin renderizar.
    pub fn impactos_por_objeto(diorama: &Blockout, camara: &Camera) -> Vec<usize> {
        let mut cuenta = vec![0usize; diorama.scene.objects.len()];
        let mut stats = TraversalStats::default();

        for fila in 0..MUESTRAS_Y {
            for columna in 0..MUESTRAS_X {
                let x = columna * ANCHO / MUESTRAS_X + ANCHO / (2 * MUESTRAS_X);
                let y = fila * ALTO / MUESTRAS_Y + ALTO / (2 * MUESTRAS_Y);

                let rayo = camara.ray_from_pixel(x, y, ANCHO, ALTO);

                if let Some(impacto) = diorama.accel.intersect(&diorama.scene, &rayo, &mut stats) {
                    cuenta[impacto.object_index] += 1;
                }
            }
        }

        cuenta
    }

    /// `connected_island_stair` con el Rompeolas de `grounded` y la isla
    /// larga compuesta **como Praderas en `spacious`**.
    ///
    /// # Qué corrige
    ///
    /// `island_mountain` vació el macizo para poblar la isla y la revisión
    /// humana lo rechaza: el Rompeolas principal vuelve **exactamente** a
    /// `grounded`. Las treinta y ocho piezas —`R-01`, `R-02` y `R-03`— se
    /// copian de allí, incluidos los seis prismas que `long_island_stair`
    /// había subido a la isla.
    ///
    /// # La isla
    ///
    /// Sin prismas: se compone sólo con `G-02`. La lógica es la de Praderas
    /// en `spacious`, leída de su geometría por `logica_de_spacious`:
    ///
    /// | pieza | qué es | de dónde sale |
    /// |---|---|---|
    /// | masa principal | la isla larga, intacta | `connected_island_stair` |
    /// | terraza grande | encima de la masa, un salto grande | la masa tapada mayor |
    /// | terraza alta | dentro de la grande, un salto corto | la masa tapada menor |
    ///
    /// Cada terraza toma de `spacious` su proporción respecto de lo que la
    /// sostiene —eje largo con eje largo— y su salto de cota. No toma sus
    /// coordenadas, ni su plinto, ni su posición al fondo de la meseta: la
    /// pila se centra en la espina de la isla, porque la corrección
    /// anterior pidió la cima ahí y no en un canto. Por el norte se para un
    /// claro entero antes del Rompeolas.
    ///
    /// Las dos masas que se reutilizan son las que **nadie veía desde
    /// arriba**: una estaba enterrada dentro de la isla larga y la otra
    /// bajo la meseta de Praderas. Se miden con `masas_tapadas`.
    ///
    /// # Qué no toca
    ///
    /// El Rompeolas de `grounded`, la base, la isla, el conector, el plinto,
    /// el Monolito, `A-01`, Praderas y Aguas. El conteo sigue en `154` y no
    /// aparece ninguna primitiva ni material nuevo: las terrazas son
    /// cuboides de `G-02` que ya lo eran.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_isla_estilo_spacious() -> Blockout {
        let mut diorama = nivel_isla_larga_con_transicion();

        let referencia = nivel_apoyado();
        let donantes = masas_tapadas(&diorama.scene);
        let logica = logica_de_spacious(&nivel_espacioso().scene);

        // --- el Rompeolas vuelve a grounded, pieza a pieza
        for i in indices_por_grupo(&referencia.scene, SpatialGroupId::Breakwater) {
            diorama.scene.objects[i].primitive = referencia.scene.objects[i].primitive;
        }

        // --- la pila de terrazas sobre la isla
        let isla = diorama.scene.objects[masa_de_la_segunda_isla(&referencia.scene)]
            .primitive
            .bounds();
        let rompeolas = huella_del_grupo(&referencia.scene, SpatialGroupId::Breakwater);

        let ancho = isla.max.x - isla.min.x;
        let fondo = isla.max.z - isla.min.z;
        let larga_en_z = fondo >= ancho;

        // Proporción de spacious aplicada eje largo con eje largo.
        let medidas = |base_x: f32, base_z: f32, largo: f32, corto: f32| {
            if larga_en_z {
                (base_x * corto, base_z * largo)
            } else {
                (base_x * largo, base_z * corto)
            }
        };

        let (t1_x, t1_z) = medidas(ancho, fondo, logica.largo_t1, logica.corto_t1);
        let (t2_x, t2_z) = medidas(t1_x, t1_z, logica.largo_t2, logica.corto_t2);

        // Centrada en la espina; por el norte, un claro antes del Rompeolas.
        let eje = (isla.min.x + isla.max.x) * 0.5;
        let centro_z = ((isla.min.z + isla.max.z) * 0.5).min(rompeolas.min.z - CLARO - t1_z * 0.5);

        let t1 = Aabb::new(
            Vec3::new(
                eje - t1_x * 0.5,
                isla.max.y - EMPOTRADO,
                centro_z - t1_z * 0.5,
            ),
            Vec3::new(
                eje + t1_x * 0.5,
                isla.max.y + logica.salto_t1,
                centro_z + t1_z * 0.5,
            ),
        );
        let t2 = Aabb::new(
            Vec3::new(
                eje - t2_x * 0.5,
                t1.max.y - EMPOTRADO,
                centro_z - t2_z * 0.5,
            ),
            Vec3::new(
                eje + t2_x * 0.5,
                t1.max.y + logica.salto_t2,
                centro_z + t2_z * 0.5,
            ),
        );

        diorama.scene.objects[donantes[0]].primitive = Cuboid::new(t1).into();
        diorama.scene.objects[donantes[1]].primitive = Cuboid::new(t2).into();

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel = SceneAccel::build(&diorama.scene)
            .expect("la escena de la isla estilo spacious tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Segundo Rompeolas
    // ------------------------------------------------------------------

    /// Las piezas del Rompeolas original: `R-01`, `R-02` y `R-03`.
    const PIEZAS_DEL_ROMPEOLAS: usize = PIEZAS + 4;

    /// Filas de la formación secundaria, de la cima hacia la base.
    ///
    /// Suman diez: una cima, un racimo intermedio de tres y una masa basal
    /// de seis en dos filas. Es la misma retícula por filas de `FILAS`, con
    /// menos piezas por fila porque la isla es estrecha.
    const FILAS_SECUNDARIAS: [usize; 4] = [1, 3, 3, 3];

    /// Altura de la formación secundaria respecto de la primera.
    ///
    /// Secundaria de verdad: con las mismas reglas de cresta y caída, a tres
    /// cuartos de altura su cima queda por debajo de la cresta original, y
    /// un test lo comprueba.
    const ESCALA_SECUNDARIA: f32 = 0.75;

    /// Altura mínima de un prisma de la formación: el mismo suelo que la
    /// macroformación.
    const ALTURA_MINIMA_DE_LA_FORMACION: f32 = 0.12;

    /// Los prismas de la formación secundaria, en el orden de la escena.
    ///
    /// Son las piezas del Rompeolas que van **después** de las treinta y
    /// ocho originales. En cualquier otro nivel no hay ninguna.
    pub fn segundo_rompeolas(scene: &Scene) -> Vec<usize> {
        indices_por_grupo(scene, SpatialGroupId::Breakwater)
            .into_iter()
            .skip(PIEZAS_DEL_ROMPEOLAS)
            .collect()
    }

    /// `connected_island_stair` con el Rompeolas de `grounded` y una
    /// **segunda formación** de basalto sobre la isla larga.
    ///
    /// # El presupuesto
    ///
    /// Es el único nivel del preview que cambia el conteo, y lo hace por
    /// autorización explícita: `154 - 2 + 10 = 162`.
    ///
    /// - **Salen dos** masas de `G-02`: las dos que `spacious_island`
    ///   convirtió en escalones verdes sobre la isla. Aquí se retiran del
    ///   vector de objetos, no se esconden.
    /// - **Entran diez** prismas nuevos, en el grupo del Rompeolas y justo
    ///   después de sus treinta y ocho piezas, para que el grupo siga siendo
    ///   contiguo como lo emite el generador.
    ///
    /// Nada se roba al Rompeolas original: sus treinta y ocho piezas se
    /// copian de `grounded` tal cual, incluidos los seis prismas que
    /// `long_island_stair` había subido a la isla.
    ///
    /// # El lenguaje
    ///
    /// La formación secundaria se construye con las reglas de la primera y
    /// no con sus coordenadas:
    ///
    /// | regla | de dónde sale |
    /// |---|---|
    /// | paso dentro de una fila y entre filas | medido sobre `R-01` en `grounded` |
    /// | filas desfasadas media celda | `FILAS`, empaquetado hexagonal |
    /// | grosor por racimos de tres | `RACIMOS`, sobre el paso medido |
    /// | perfil de cresta y caída | la misma fórmula de `nivel_candidato` |
    /// | ruido de altura | `SEMILLA`, la misma |
    /// | material y grupo de revelación | los de `R-01`, pieza a pieza |
    /// | primitiva | `pieza`: `HexPrism` en la Ruta A, `Cuboid` en la B |
    ///
    /// # La forma
    ///
    /// | filas | prismas | qué es |
    /// |---|---:|---|
    /// | la del sur | `1` | la cima |
    /// | la siguiente | `3` | el racimo intermedio |
    /// | las dos del norte | `6` | la masa basal |
    ///
    /// La cima es la fila de cresta y la masa basal cae hacia la base
    /// original, igual que la primera formación cae hacia la bahía. La
    /// formación se centra en la espina de la isla y se para antes del
    /// conector por el norte: nada se apoya a medias en él. Todo se apoya
    /// en la isla, empotrado `0.10`.
    ///
    /// # Qué no toca
    ///
    /// El Rompeolas de `grounded`, la base, la isla, el conector, el plinto,
    /// el Monolito, `A-01`, Praderas y Aguas. No aparece ningún material ni
    /// textura nuevos.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_segundo_rompeolas() -> Blockout {
        let mut diorama = nivel_isla_larga_con_transicion();

        let referencia = nivel_apoyado();
        let retiradas: Vec<usize> = masas_tapadas(&diorama.scene).into_iter().take(2).collect();

        // --- el Rompeolas vuelve a grounded, pieza a pieza
        let rompeolas_original = indices_por_grupo(&referencia.scene, SpatialGroupId::Breakwater);

        for &i in &rompeolas_original {
            diorama.scene.objects[i].primitive = referencia.scene.objects[i].primitive;
        }

        // --- el lenguaje de la primera formación, medido
        let r01 = &rompeolas_original[..PILARES_R01];
        let centro = |i: usize| {
            let c = referencia.scene.objects[i].primitive.bounds();

            ((c.min.x + c.max.x) * 0.5, (c.min.z + c.max.z) * 0.5)
        };

        let paso_x = (centro(r01[FILAS[0] - 1]).0 - centro(r01[0]).0) / (FILAS[0] - 1) as f32;
        let paso_z = centro(r01[FILAS[0]]).1 - centro(r01[0]).1;
        let alto = r01
            .iter()
            .map(|&i| {
                let c = referencia.scene.objects[i].primitive.bounds();

                c.max.y - c.min.y
            })
            .fold(f32::MIN, f32::max)
            * ESCALA_SECUNDARIA;

        let isla = diorama.scene.objects[masa_de_la_segunda_isla(&referencia.scene)]
            .primitive
            .bounds();
        let conector = diorama.scene.objects[masa_de_la_transicion(&referencia.scene)]
            .primitive
            .bounds();
        let rompeolas = huella(&referencia.scene, &rompeolas_original);

        let eje = (isla.min.x + isla.max.x) * 0.5;
        let suelo = isla.max.y - EMPOTRADO;

        // --- la formación, primero en coordenadas propias
        let mut azar = Azar::new(SEMILLA);
        let mut piezas: Vec<(Vec3, f32, f32)> = Vec::new();

        for (fila, &columnas) in FILAS_SECUNDARIAS.iter().enumerate() {
            let hacia_la_base = fila as f32 / (FILAS_SECUNDARIAS.len() - 1) as f32;
            let desfase = if fila % 2 == 1 {
                paso_x * 0.25
            } else {
                -paso_x * 0.25
            };

            for columna in 0..columnas {
                let k = piezas.len();
                let lado = columna as f32 - (columnas - 1) as f32 * 0.5;

                // Perfil de cresta y caída: la fórmula de la macroformación.
                let centrado = lado / (columnas as f32 * 0.5);
                let cresta = 1.0 - 0.45 * centrado * centrado;
                let caida = 1.0 - 0.78 * hacia_la_base.powf(1.6);
                let ruido = 0.10 * azar.simetrico();
                let altura = (alto * (cresta * caida + ruido)).max(ALTURA_MINIMA_DE_LA_FORMACION);

                let ancho = paso_x * 0.86 * RACIMOS[(k / 3) % RACIMOS.len()];

                piezas.push((
                    Vec3::new(
                        eje + desfase + lado * paso_x,
                        suelo + altura * 0.5,
                        fila as f32 * paso_z,
                    ),
                    ancho,
                    altura,
                ));
            }
        }

        // --- y después se lleva en bloque a su sitio
        //
        // Por el norte se para antes del conector y a un claro del
        // Rompeolas; en `x`, si se saliera de la isla, se recorta en bloque.
        let cajas: Vec<Aabb> = piezas
            .iter()
            .map(|&(c, ancho, altura)| pieza(c, ancho, altura).bounds())
            .collect();
        let oeste = cajas.iter().map(|c| c.min.x).fold(f32::MAX, f32::min);
        let este = cajas.iter().map(|c| c.max.x).fold(f32::MIN, f32::max);
        let norte = cajas.iter().map(|c| c.max.z).fold(f32::MIN, f32::max);

        let tope_norte = (rompeolas.min.z - CLARO).min(conector.min.z - HOLGURA_EN_LA_ISLA);
        let dz = tope_norte - norte;

        let mut dx = 0.0_f32;

        if este > isla.max.x - HOLGURA_EN_LA_ISLA {
            dx = isla.max.x - HOLGURA_EN_LA_ISLA - este;
        }
        if oeste + dx < isla.min.x + HOLGURA_EN_LA_ISLA {
            dx = isla.min.x + HOLGURA_EN_LA_ISLA - oeste;
        }

        let nuevos: Vec<SceneObject> = piezas
            .iter()
            .enumerate()
            .map(|(k, &(c, ancho, altura))| {
                let origen = &referencia.scene.objects[r01[k]];

                SceneObject {
                    primitive: pieza(c + Vec3::new(dx, 0.0, dz), ancho, altura),
                    ..*origen
                }
            })
            .collect();

        // --- el vector nuevo: sin las dos masas, con los diez detras del
        // Rompeolas original
        let ultimo = *rompeolas_original
            .last()
            .expect("el Rompeolas tiene piezas");
        let mut objetos: Vec<SceneObject> = Vec::with_capacity(diorama.scene.objects.len() + 8);

        for (i, objeto) in diorama.scene.objects.iter().enumerate() {
            if retiradas.contains(&i) {
                continue;
            }

            objetos.push(*objeto);

            if i == ultimo {
                objetos.extend(nuevos.iter().copied());
            }
        }

        diorama.scene.objects = objetos;

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        // La jerarquía se construye **después** de fijar el vector: es la
        // invariante de `Scene`, que guarda posiciones y no identidades.
        diorama.accel = SceneAccel::build(&diorama.scene)
            .expect("la escena del segundo Rompeolas tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Segundo Rompeolas invertido
    // ------------------------------------------------------------------

    /// `second_breakwater` con la jerarquía de la formación **dada la
    /// vuelta** a lo largo del eje isla ↔ Rompeolas.
    ///
    /// # Qué cambia
    ///
    /// En `second_breakwater` la cima miraba al lado libre de la isla y la
    /// masa basal caía hacia la base original. La revisión humana la aprueba
    /// y pide lo contrario: **la cima y el racimo alto del lado del
    /// Rompeolas original y de la base**, y las piezas bajas abriéndose
    /// hacia el espacio libre.
    ///
    /// No se gira ningún prisma: siguen verticales, con su ancho, su altura
    /// y su abscisa. Lo que se invierte es el **orden de las filas** en `z`,
    /// reflejándolas sobre el plano medio de la formación. El paso entre
    /// filas, el desfase de media celda y la estructura `6/3/1` se
    /// conservan por construcción.
    ///
    /// # Dónde se para
    ///
    /// Después del reflejo la formación se lleva en bloque hacia el norte
    /// hasta que la **primera pieza** toca su límite, con la misma regla de
    /// apoyo de `second_breakwater` pero pieza a pieza: la que comparte
    /// abscisa con el conector se para antes de él y el resto, a un claro
    /// entero del Rompeolas original. Con la cima delante, eso deja que se
    /// acerque más de lo que podía la masa basal.
    ///
    /// # Qué no toca
    ///
    /// Todo lo demás es `second_breakwater`: 162 objetos, las dos masas
    /// retiradas, el Rompeolas de `grounded`, la base, la isla, el conector,
    /// el Monolito, `A-01`, Praderas y Aguas.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_segundo_rompeolas_invertido() -> Blockout {
        let mut diorama = nivel_segundo_rompeolas();

        let referencia = nivel_apoyado();
        let nuevos = segundo_rompeolas(&diorama.scene);

        let conector = diorama.scene.objects[masa_de_la_transicion(&referencia.scene)]
            .primitive
            .bounds();
        let rompeolas = huella(
            &referencia.scene,
            &indices_por_grupo(&referencia.scene, SpatialGroupId::Breakwater),
        );

        let cajas: Vec<Aabb> = nuevos
            .iter()
            .map(|&i| diorama.scene.objects[i].primitive.bounds())
            .collect();

        let centro_z = |c: &Aabb| (c.min.z + c.max.z) * 0.5;
        let sur = cajas.iter().map(centro_z).fold(f32::MAX, f32::min);
        let norte = cajas.iter().map(centro_z).fold(f32::MIN, f32::max);

        // --- el reflejo: cada fila pasa al sitio de su simétrica
        let reflejadas: Vec<(Vec3, f32, f32)> = cajas
            .iter()
            .map(|c| {
                (
                    Vec3::new(
                        (c.min.x + c.max.x) * 0.5,
                        (c.min.y + c.max.y) * 0.5,
                        sur + norte - centro_z(c),
                    ),
                    c.max.x - c.min.x,
                    c.max.y - c.min.y,
                )
            })
            .collect();

        // --- y el bloque sube al norte hasta que la primera pieza topa
        let dz = reflejadas
            .iter()
            .map(|&(centro, ancho, altura)| {
                let caja = pieza(centro, ancho, altura).bounds();
                let frente_al_conector = caja.min.x < conector.max.x && caja.max.x > conector.min.x;
                let tope = if frente_al_conector {
                    conector.min.z - HOLGURA_EN_LA_ISLA
                } else {
                    rompeolas.min.z - CLARO
                };

                tope - caja.max.z
            })
            .fold(f32::MAX, f32::min);

        for (&i, &(centro, ancho, altura)) in nuevos.iter().zip(&reflejadas) {
            diorama.scene.objects[i].primitive =
                pieza(centro + Vec3::new(0.0, 0.0, dz), ancho, altura);
        }

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel = SceneAccel::build(&diorama.scene)
            .expect("la escena del segundo Rompeolas invertido tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Segundo Rompeolas escalonado
    // ------------------------------------------------------------------

    /// Prismas por banda, de la cima —junto al Rompeolas original— al
    /// extremo libre de la isla. Suman diez.
    ///
    /// Crecen hacia el extremo libre para que la escalera se abra al pie y
    /// se afile arriba, como una ladera y no como un muro.
    const BANDAS_DE_LA_ESCALERA: [usize; 4] = [1, 2, 3, 4];

    /// Grosor de cada banda, en el mismo orden: cuatro factores de
    /// `RACIMOS`, de mayor a menor.
    ///
    /// El tamaño de la escalera se lee en el grosor, y el grosor sale del
    /// repertorio de la primera formación: cada banda es uno de sus racimos.
    const GROSOR_DE_LAS_BANDAS: [f32; 4] = [RACIMOS[2], RACIMOS[4], RACIMOS[3], RACIMOS[1]];

    /// `second_breakwater_turned` convertido en una **escalera-montaña**
    /// continua que crece hacia el Rompeolas original.
    ///
    /// # Qué corrige
    ///
    /// La revisión humana aprueba la orientación de `turned` y pide que se
    /// lea como una escalera y no como un bloque `6/3/1`: prismas pequeños y
    /// bajos en el extremo libre de la isla, y cada vez más gruesos y altos
    /// acercándose al Rompeolas original y a la base.
    ///
    /// # Las bandas
    ///
    /// | banda | prismas | grosor | dónde |
    /// |---|---:|---|---|
    /// | cima | `1` | `1.22` | la más cercana al Rompeolas |
    /// | segunda | `2` | `1.12` | |
    /// | tercera | `3` | `0.88` | |
    /// | pie | `4` | `0.74` | el extremo libre |
    ///
    /// La altura sale de la fórmula de cresta y caída de la macroformación,
    /// con la banda como avance desde la cresta y `SEMILLA` como ruido; el
    /// grosor, de `RACIMOS`. Las bandas van al paso de `R-01` y desfasadas
    /// media celda, así que cada una **muerde** la siguiente: no hay hueco
    /// entre peldaños, que es lo que hace continua la escalera.
    ///
    /// La colocación es la de `turned`: centrada en la espina, recortada en
    /// bloque para caber en la isla y llevada al norte hasta que la primera
    /// pieza topa con el conector —si comparte abscisa con él— o con un
    /// claro del Rompeolas original.
    ///
    /// # Qué no toca
    ///
    /// Todo lo que no sean los diez prismas secundarios: 162 objetos, el
    /// Rompeolas de `grounded`, la base, la isla, el conector, el Monolito,
    /// `A-01`, Praderas y Aguas. Cada prisma conserva su material, su grupo
    /// y su tipo de primitiva.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_segundo_rompeolas_escalonado() -> Blockout {
        let mut diorama = nivel_segundo_rompeolas_invertido();

        let referencia = nivel_apoyado();
        let nuevos = segundo_rompeolas(&diorama.scene);
        let rompeolas_original = indices_por_grupo(&referencia.scene, SpatialGroupId::Breakwater);

        // --- el lenguaje de la primera formación, medido
        let r01 = &rompeolas_original[..PILARES_R01];
        let centro = |i: usize| {
            let c = referencia.scene.objects[i].primitive.bounds();

            ((c.min.x + c.max.x) * 0.5, (c.min.z + c.max.z) * 0.5)
        };

        let paso_x = (centro(r01[FILAS[0] - 1]).0 - centro(r01[0]).0) / (FILAS[0] - 1) as f32;
        let paso_z = centro(r01[FILAS[0]]).1 - centro(r01[0]).1;
        let alto = r01
            .iter()
            .map(|&i| {
                let c = referencia.scene.objects[i].primitive.bounds();

                c.max.y - c.min.y
            })
            .fold(f32::MIN, f32::max)
            * ESCALA_SECUNDARIA;

        let isla = diorama.scene.objects[masa_de_la_segunda_isla(&referencia.scene)]
            .primitive
            .bounds();
        let conector = diorama.scene.objects[masa_de_la_transicion(&referencia.scene)]
            .primitive
            .bounds();
        let rompeolas = huella(&referencia.scene, &rompeolas_original);

        let eje = (isla.min.x + isla.max.x) * 0.5;
        let suelo = isla.max.y - EMPOTRADO;

        // --- las bandas, de la cima hacia el sur, en coordenadas propias
        let mut azar = Azar::new(SEMILLA);
        let mut piezas: Vec<(Vec3, f32, f32)> = Vec::new();

        for (banda, &cuantos) in BANDAS_DE_LA_ESCALERA.iter().enumerate() {
            let desde_la_cima = banda as f32 / (BANDAS_DE_LA_ESCALERA.len() - 1) as f32;
            let desfase = if banda % 2 == 1 {
                paso_x * 0.25
            } else {
                -paso_x * 0.25
            };
            let ancho = paso_x * 0.86 * GROSOR_DE_LAS_BANDAS[banda];

            for columna in 0..cuantos {
                let lado = columna as f32 - (cuantos - 1) as f32 * 0.5;

                let centrado = lado / (cuantos as f32 * 0.5);
                let cresta = 1.0 - 0.45 * centrado * centrado;
                let caida = 1.0 - 0.78 * desde_la_cima.powf(1.6);
                let ruido = 0.10 * azar.simetrico();
                let altura = (alto * (cresta * caida + ruido)).max(ALTURA_MINIMA_DE_LA_FORMACION);

                piezas.push((
                    Vec3::new(
                        eje + desfase + lado * paso_x,
                        suelo + altura * 0.5,
                        -(banda as f32) * paso_z,
                    ),
                    ancho,
                    altura,
                ));
            }
        }

        debug_assert_eq!(
            piezas.len(),
            nuevos.len(),
            "las bandas tienen que sumar diez"
        );

        // --- en bloque: primero que quepa en la isla, luego al norte
        let cajas: Vec<Aabb> = piezas
            .iter()
            .map(|&(c, ancho, altura)| pieza(c, ancho, altura).bounds())
            .collect();

        let oeste = cajas.iter().map(|c| c.min.x).fold(f32::MAX, f32::min);
        let este = cajas.iter().map(|c| c.max.x).fold(f32::MIN, f32::max);

        let mut dx = 0.0_f32;

        if este > isla.max.x - HOLGURA_EN_LA_ISLA {
            dx = isla.max.x - HOLGURA_EN_LA_ISLA - este;
        }
        if oeste + dx < isla.min.x + HOLGURA_EN_LA_ISLA {
            dx = isla.min.x + HOLGURA_EN_LA_ISLA - oeste;
        }

        let dz = cajas
            .iter()
            .map(|c| {
                let frente_al_conector =
                    c.min.x + dx < conector.max.x && c.max.x + dx > conector.min.x;
                let tope = if frente_al_conector {
                    conector.min.z - HOLGURA_EN_LA_ISLA
                } else {
                    rompeolas.min.z - CLARO
                };

                tope - c.max.z
            })
            .fold(f32::MAX, f32::min);

        for (&i, &(centro, ancho, altura)) in nuevos.iter().zip(&piezas) {
            diorama.scene.objects[i].primitive =
                pieza(centro + Vec3::new(dx, 0.0, dz), ancho, altura);
        }

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        diorama.accel = SceneAccel::build(&diorama.scene)
            .expect("la escena del segundo Rompeolas escalonado tiene geometria");

        diorama
    }

    // ------------------------------------------------------------------
    // Segundo Rompeolas escalonado denso
    // ------------------------------------------------------------------

    /// Prismas por banda, de la cima —junto al Rompeolas original— al
    /// extremo libre. Suman dieciséis: `2 + 3 + 5 + 6`.
    const BANDAS_DENSAS: [usize; 4] = [2, 3, 5, 6];

    /// Altura de la escalera densa respecto de la primera formación.
    ///
    /// Más que `ESCALA_SECUNDARIA`: con dos prismas en la banda alta la
    /// cresta reparte la altura entre ellos, y a `0.75` la cima quedaría por
    /// debajo de la de la escalera de diez. A `0.85` no baja, y sigue por
    /// debajo de la cresta original; un test mide las dos cosas.
    const ESCALA_DE_LA_ESCALERA_DENSA: f32 = 0.85;

    /// Claro entre la escalera y el Rompeolas original.
    ///
    /// Reducido pero positivo: la revisión pide acercarla. Un claro entero
    /// la dejaba como otra formación; esto la deja como su continuación sin
    /// tocarla.
    const CLARO_AL_PRIMER_ROMPEOLAS: f32 = 0.40;

    /// Paso de la rejilla con que se mide la cobertura de la isla.
    const PASO_DE_COBERTURA: f32 = 0.05;

    /// Fracción de la planta de la isla que cubren las huellas de la
    /// formación secundaria.
    ///
    /// Se muestrea con una rejilla de `0.05`: una celda cuenta si su centro
    /// cae dentro de alguna huella. Las huellas son las cajas envolventes,
    /// así que en la Ruta A mide algo de más; mide igual en las dos
    /// versiones que se comparan, que es lo que importa.
    pub fn cobertura_de_la_isla(diorama: &Blockout) -> f32 {
        let isla = diorama.scene.objects[masa_de_la_segunda_isla(&nivel_apoyado().scene)]
            .primitive
            .bounds();
        let cajas: Vec<Aabb> = segundo_rompeolas(&diorama.scene)
            .iter()
            .map(|&i| diorama.scene.objects[i].primitive.bounds())
            .collect();

        let columnas = ((isla.max.x - isla.min.x) / PASO_DE_COBERTURA) as usize;
        let filas = ((isla.max.z - isla.min.z) / PASO_DE_COBERTURA) as usize;
        let mut cubiertas = 0usize;

        for f in 0..filas {
            for c in 0..columnas {
                let x = isla.min.x + (c as f32 + 0.5) * PASO_DE_COBERTURA;
                let z = isla.min.z + (f as f32 + 0.5) * PASO_DE_COBERTURA;

                if cajas
                    .iter()
                    .any(|k| k.min.x <= x && k.max.x >= x && k.min.z <= z && k.max.z >= z)
                {
                    cubiertas += 1;
                }
            }
        }

        cubiertas as f32 / (columnas * filas).max(1) as f32
    }

    /// `second_breakwater_stair` con **dieciséis** prismas, más cerca del
    /// Rompeolas original y llenando más la isla.
    ///
    /// # El presupuesto
    ///
    /// La autorización sigue para esta formación: `154 - 2 + 16 = 168`. Los
    /// diez prismas de la escalera se reaprovechan y seis más entran justo
    /// detrás de ellos, así que el Rompeolas sigue contiguo. Cada prisma `k`
    /// toma material y grupo de la pieza `k` de `R-01`, como en
    /// `second_breakwater`.
    ///
    /// # Las bandas
    ///
    /// | banda | prismas | grosor | |
    /// |---|---:|---|---|
    /// | cima | `2` | `1.22` | a `0.40` del Rompeolas original |
    /// | segunda | `3` | `1.12` | |
    /// | tercera | `5` | `0.88` | |
    /// | pie | `6` | `0.74` | el extremo libre, de canto a canto |
    ///
    /// Grosores y alturas salen como en la escalera de diez: `RACIMOS` y la
    /// fórmula de cresta y caída con `SEMILLA`. Las bandas van al paso de
    /// `R-01` entre filas y se muerden.
    ///
    /// # Dos ajustes, los dos a la vista
    ///
    /// - **El paso dentro de una banda se aprieta si no cabe.** Seis prismas
    ///   al paso de `R-01` no caben en la anchura de la isla; el pie y la
    ///   tercera banda se aprietan hasta caber. Nunca se separan más que
    ///   ese paso.
    /// - **Las bandas altas se apartan del conector.** Las que llegan más al
    ///   norte que el conector se recortan en `x` hasta quedar al oeste de
    ///   él, en vez de pararse todas antes. Es lo que permite que la cima se
    ///   acerque al Rompeolas.
    ///
    /// # Qué no toca
    ///
    /// El Rompeolas de `grounded`, `G-02` —base, isla y conector incluidos—,
    /// el plinto, el Monolito, `A-01`, Praderas y Aguas.
    ///
    /// **La composición exacta es candidata para revisión humana.**
    pub fn nivel_segundo_rompeolas_escalonado_denso() -> Blockout {
        let mut diorama = nivel_segundo_rompeolas_escalonado();

        let referencia = nivel_apoyado();
        let secundarios = segundo_rompeolas(&diorama.scene);
        let rompeolas_original = indices_por_grupo(&referencia.scene, SpatialGroupId::Breakwater);

        // --- el lenguaje de la primera formación, medido
        let r01 = &rompeolas_original[..PILARES_R01];
        let centro = |i: usize| {
            let c = referencia.scene.objects[i].primitive.bounds();

            ((c.min.x + c.max.x) * 0.5, (c.min.z + c.max.z) * 0.5)
        };

        let paso_x = (centro(r01[FILAS[0] - 1]).0 - centro(r01[0]).0) / (FILAS[0] - 1) as f32;
        let paso_z = centro(r01[FILAS[0]]).1 - centro(r01[0]).1;
        let alto = r01
            .iter()
            .map(|&i| {
                let c = referencia.scene.objects[i].primitive.bounds();

                c.max.y - c.min.y
            })
            .fold(f32::MIN, f32::max)
            * ESCALA_DE_LA_ESCALERA_DENSA;

        let isla = diorama.scene.objects[masa_de_la_segunda_isla(&referencia.scene)]
            .primitive
            .bounds();
        let conector = diorama.scene.objects[masa_de_la_transicion(&referencia.scene)]
            .primitive
            .bounds();
        let rompeolas = huella(&referencia.scene, &rompeolas_original);

        let eje = (isla.min.x + isla.max.x) * 0.5;
        let suelo = isla.max.y - EMPOTRADO;

        // --- alturas y grosores por banda, de la cima al pie
        let mut azar = Azar::new(SEMILLA);
        let mut bandas: Vec<(f32, Vec<f32>, f32)> = Vec::new();

        for (banda, &cuantos) in BANDAS_DENSAS.iter().enumerate() {
            let desde_la_cima = banda as f32 / (BANDAS_DENSAS.len() - 1) as f32;
            let ancho = paso_x * 0.86 * GROSOR_DE_LAS_BANDAS[banda];

            let alturas: Vec<f32> = (0..cuantos)
                .map(|columna| {
                    let lado = columna as f32 - (cuantos - 1) as f32 * 0.5;
                    let centrado = lado / (cuantos as f32 * 0.5);
                    let cresta = 1.0 - 0.45 * centrado * centrado;
                    let caida = 1.0 - 0.78 * desde_la_cima.powf(1.6);
                    let ruido = 0.10 * azar.simetrico();

                    (alto * (cresta * caida + ruido)).max(ALTURA_MINIMA_DE_LA_FORMACION)
                })
                .collect();

            let medio_fondo = {
                let tanteo = pieza(Vec3::new(0.0, 0.0, 0.0), ancho, 1.0).bounds();

                (tanteo.max.z - tanteo.min.z) * 0.5
            };

            bandas.push((ancho, alturas, medio_fondo));
        }

        // --- en z: la banda alta a su claro del Rompeolas original
        let dz = bandas
            .iter()
            .enumerate()
            .map(|(banda, &(_, _, medio_fondo))| {
                rompeolas.min.z
                    - CLARO_AL_PRIMER_ROMPEOLAS
                    - (-(banda as f32) * paso_z + medio_fondo)
            })
            .fold(f32::MAX, f32::min);

        // --- en x: cada banda, en el tramo de isla que le toca
        let mut piezas: Vec<(Vec3, f32, f32)> = Vec::new();

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
            let centro_de_la_banda =
                encajado(eje + desfase, oeste + medio_tramo, este - medio_tramo);

            for (columna, &altura) in alturas.iter().enumerate() {
                let lado = columna as f32 - (cuantos - 1) as f32 * 0.5;

                piezas.push((
                    Vec3::new(centro_de_la_banda + lado * paso, suelo + altura * 0.5, z),
                    *ancho,
                    altura,
                ));
            }
        }

        // --- el vector nuevo: los dieciséis en el sitio de los diez
        let primero = *secundarios.first().expect("la escalera tiene prismas");
        let ultimo = *secundarios.last().expect("la escalera tiene prismas");

        let formacion: Vec<SceneObject> = piezas
            .iter()
            .enumerate()
            .map(|(k, &(c, ancho, altura))| SceneObject {
                primitive: pieza(c, ancho, altura),
                ..referencia.scene.objects[r01[k]]
            })
            .collect();

        let mut objetos: Vec<SceneObject> = Vec::with_capacity(diorama.scene.objects.len() + 6);

        objetos.extend_from_slice(&diorama.scene.objects[..primero]);
        objetos.extend(formacion);
        objetos.extend_from_slice(&diorama.scene.objects[ultimo + 1..]);

        diorama.scene.objects = objetos;

        let orbit_center = diorama.anchors.orbit_center;
        let monolith_height = diorama.scale.monolith_height;
        let scene_radius = measure_scene_radius(&diorama.scene, orbit_center);
        let orbit_radius = derive_orbit_radius(scene_radius, monolith_height);

        diorama.anchors = SceneAnchors {
            look_at: orbit_center + Vec3::new(0.0, LOOK_AT_HEIGHT_FRACTION * monolith_height, 0.0),
            hero_camera_anchor: eye_at_yaw(orbit_center, orbit_radius, HERO_YAW_DEGREES),
            ..diorama.anchors
        };

        diorama.scale = SceneScale {
            scene_radius,
            orbit_radius,
            ..diorama.scale
        };

        // La jerarquía, después de fijar el vector: la invariante de `Scene`.
        diorama.accel = SceneAccel::build(&diorama.scene)
            .expect("la escena del segundo Rompeolas denso tiene geometria");

        diorama
    }

    /// Caja que envuelve un grupo espacial entero.
    pub fn huella_del_grupo(scene: &Scene, grupo: SpatialGroupId) -> Aabb {
        huella(scene, &indices_por_grupo(scene, grupo))
    }

    /// Una línea del informe territorial: qué ocupa la región en cada eje.
    fn linea_territorial(nombre: &str, antes: &Aabb, ahora: &Aabb) {
        let extension = |caja: &Aabb| {
            (
                caja.max.x - caja.min.x,
                caja.max.y - caja.min.y,
                caja.max.z - caja.min.z,
            )
        };

        let (ax, ay, az) = extension(antes);
        let (bx, by, bz) = extension(ahora);
        let delta = (ahora.min + ahora.max) * 0.5 - (antes.min + antes.max) * 0.5;

        println!(
            "  {nombre:<12} {ax:5.2} x {ay:5.2} x {az:5.2}  ->  {bx:5.2} x {by:5.2} x {bz:5.2}   \
             centro {:+.2} {:+.2} {:+.2}",
            delta.x, delta.y, delta.z
        );
    }

    /// Traza un diorama con la toma hero.
    fn trazar(diorama: &Blockout, camara: &Camera) -> Framebuffer {
        let mut framebuffer = Framebuffer::new(ANCHO, ALTO);
        let luces = luces_del_diorama(&diorama.anchors, &diorama.scale);

        render(
            &mut framebuffer,
            &diorama.scene,
            &diorama.accel,
            &luces,
            &RevealState::painted(),
            camara,
            Shading::Material,
        );

        framebuffer
    }

    fn guardar(framebuffer: &Framebuffer, destino: &Path, nombre: &str) -> bool {
        let ruta = destino.join(nombre);

        match framebuffer.save_png(&ruta) {
            Ok(()) => {
                println!("  escrito   {}", ruta.display());
                true
            }
            Err(e) => {
                eprintln!("error: no se pudo escribir {}: {e}", ruta.display());
                false
            }
        }
    }

    pub fn correr() -> ExitCode {
        let Some(destino) = std::env::args().nth(1).map(PathBuf::from) else {
            eprintln!("uso: ... --example breakwater_fidelity_preview -- <carpeta de salida>");
            eprintln!("  el directorio es obligatorio: este ejemplo no escribe evidencia oficial.");
            return ExitCode::FAILURE;
        };

        if let Err(e) = fs::create_dir_all(&destino) {
            eprintln!("error: no se pudo crear {}: {e}", destino.display());
            return ExitCode::FAILURE;
        }

        let actual = nivel();
        let candidato = nivel_candidato();
        let espacioso = nivel_espacioso();
        let conectado = nivel_conectado();
        let refinado = nivel_refinado();
        let asimetrico = nivel_asimetrico();
        let separado = nivel_separado();
        let apoyado = nivel_apoyado();
        let escalera = nivel_escalera();
        let combinado = nivel_combinado();
        let extension = nivel_ascenso_desde_grounded();
        let relleno = nivel_relleno_trasero();
        let ruta = nivel_escalera_monolito();
        let borde = nivel_rompeolas_de_borde();
        let terraza = nivel_terraza_verde_de_borde();
        let cadena = nivel_plataformas_de_borde();
        let finas = nivel_plataformas_finas();
        let marcadas = nivel_plataformas_marcadas();
        let modular = nivel_soporte_modular();
        let isla = nivel_segunda_isla();
        let isla_borde = nivel_isla_de_borde();
        let isla_ancha = nivel_isla_de_borde_ancha();
        let isla_larga = nivel_isla_larga_con_escalera();
        let unida = nivel_isla_larga_con_transicion();
        let agrupada = nivel_escalera_agrupada();

        // La **misma** cámara para los dos: lo único que cambia entre las dos
        // imágenes es la forma de treinta y cuatro piezas.
        let camara = actual.hero_camera();
        let indices = indices_de_rompeolas(&actual.scene);

        println!("breakwater_fidelity_preview\n");
        println!("  destino     {}", destino.display());
        println!("  dimension   {ANCHO} x {ALTO}, toma hero, RevealState::painted");
        println!(
            "  ruta        {}",
            if cfg!(feature = "hex-prism") {
                "A, hex-prism por defecto"
            } else {
                "B, --no-default-features"
            }
        );
        println!(
            "  objetos     {} en los dos niveles",
            actual.scene.objects.len()
        );
        println!("  rompeolas   {} piezas", indices.len());
        println!("  sustituidas {PIEZAS} (R-01 y R-02)");
        println!("  conservadas {} (R-03)", indices.len() - PIEZAS);
        println!("  reticula    {:?} filas de la cresta a la bahia\n", FILAS);

        println!(
            "  territorio  plinto {PLINTO_ACTUAL_X:.0} x {PLINTO_ACTUAL_Z:.0} \
             -> {PLINTO_ESPACIOSO_X:.0} x {PLINTO_ESPACIOSO_Z:.0}"
        );
        for (nombre, grupo) in [
            ("plinto", SpatialGroupId::Global),
            ("fondo", SpatialGroupId::ContinentBackground),
            ("monolito", SpatialGroupId::Monolith),
            ("praderas", SpatialGroupId::Meadows),
            ("rompeolas", SpatialGroupId::Breakwater),
            ("aguas", SpatialGroupId::FlyingWaters),
        ] {
            linea_territorial(
                nombre,
                &huella_del_grupo(&candidato.scene, grupo),
                &huella_del_grupo(&espacioso.scene, grupo),
            );
        }
        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}\n",
            actual.scale.scene_radius,
            espacioso.scale.scene_radius,
            actual.scale.orbit_radius,
            espacioso.scale.orbit_radius
        );

        let terrazas = masas_de_terraza(&candidato.scene);

        println!("  cadena      candidato -> connected");
        for (nombre, grupo) in [
            ("plinto", SpatialGroupId::Global),
            ("fondo", SpatialGroupId::ContinentBackground),
            ("monolito", SpatialGroupId::Monolith),
            ("praderas", SpatialGroupId::Meadows),
            ("rompeolas", SpatialGroupId::Breakwater),
            ("aguas", SpatialGroupId::FlyingWaters),
        ] {
            linea_territorial(
                nombre,
                &huella_del_grupo(&candidato.scene, grupo),
                &huella_del_grupo(&conectado.scene, grupo),
            );
        }
        linea_territorial(
            "terrazas",
            &huella(&candidato.scene, &terrazas),
            &huella(&conectado.scene, &terrazas),
        );
        println!(
            "  terrazas    {} de {} masas de G-02, por indice de escena: {:?}",
            terrazas.len(),
            indices_por_grupo(&candidato.scene, SpatialGroupId::ContinentBackground).len(),
            terrazas
        );
        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}\n",
            actual.scale.scene_radius,
            conectado.scale.scene_radius,
            actual.scale.orbit_radius,
            conectado.scale.orbit_radius
        );

        let labio = labio_costero(&refinado.scene);
        let (exteriores, centrales) = cotas_del_labio(&refinado.scene);
        let techo = |ids: &[usize]| -> Vec<f32> {
            let mut ys: Vec<f32> = ids
                .iter()
                .map(|&i| refinado.scene.objects[i].primitive.bounds().max.y)
                .collect();
            ys.sort_by(|a, b| b.partial_cmp(a).expect("no hay NaN"));
            ys
        };

        println!("  labio       connected -> refined");
        linea_territorial(
            "A-11",
            &huella(&conectado.scene, &labio),
            &huella(&refinado.scene, &labio),
        );
        println!(
            "  cota alta   flancos {:?}",
            techo(&exteriores)
                .iter()
                .map(|y| (y * 100.0).round() / 100.0)
                .collect::<Vec<f32>>()
        );
        println!(
            "  cota baja   centro  {:?}",
            techo(&centrales)
                .iter()
                .map(|y| (y * 100.0).round() / 100.0)
                .collect::<Vec<f32>>()
        );

        let agua_antes = vista_del_agua(&conectado, &conectado.hero_camera());
        let agua_ahora = vista_del_agua(&refinado, &refinado.hero_camera());

        println!(
            "  lamina      visible {} -> {} celdas de {}, mayor mancha {} -> {}",
            agua_antes.visibles,
            agua_ahora.visibles,
            agua_ahora.muestras,
            agua_antes.mayor_mancha,
            agua_ahora.mayor_mancha
        );

        let aguas = huella_del_grupo(&refinado.scene, SpatialGroupId::FlyingWaters);
        let plinto = huella_del_grupo(&refinado.scene, SpatialGroupId::Global);
        println!(
            "  margen      Aguas al borde del plinto: -x {:.2}  +x {:.2}  -z {:.2}  +z {:.2}\n",
            aguas.min.x - plinto.min.x,
            plinto.max.x - aguas.max.x,
            aguas.min.z - plinto.min.z,
            plinto.max.z - aguas.max.z
        );

        let boca_simetrica = boca_costera(&refinado.scene);
        let boca = boca_costera(&asimetrico.scene);
        let eje = |escena: &Scene| {
            let fila = huella(escena, &labio_costero(escena));

            (fila.min.x + fila.max.x) * 0.5
        };
        let flanco = |escena: &Scene, ids: &[usize]| {
            let caja = huella(escena, ids);

            (caja.max.x - caja.min.x, (caja.min.z + caja.max.z) * 0.5)
        };

        println!("  boca        refined -> asymmetric");
        linea_territorial(
            "A-11",
            &huella(&refinado.scene, &labio_costero(&refinado.scene)),
            &huella(&asimetrico.scene, &labio_costero(&asimetrico.scene)),
        );
        println!(
            "  reparto     {} | {} bloques  ->  {} | {}",
            boca_simetrica.izquierda.len(),
            boca_simetrica.derecha.len(),
            boca.izquierda.len(),
            boca.derecha.len()
        );
        println!(
            "  canal       ancho {:.2} -> {:.2},  centro {:+.2} -> {:+.2} respecto del eje",
            boca_simetrica.ancho(),
            boca.ancho(),
            boca_simetrica.centro() - eje(&refinado.scene),
            boca.centro() - eje(&asimetrico.scene)
        );

        let (largo_izq, z_izq) = flanco(&asimetrico.scene, &boca.izquierda);
        let (largo_der, z_der) = flanco(&asimetrico.scene, &boca.derecha);

        println!(
            "  flancos     izquierda largo {largo_izq:.2} en z {z_izq:.2}   \
             derecha largo {largo_der:.2} en z {z_der:.2}"
        );

        let agua_asimetrica = vista_del_agua(&asimetrico, &asimetrico.hero_camera());
        let aguas = huella_del_grupo(&asimetrico.scene, SpatialGroupId::FlyingWaters);
        let plinto = huella_del_grupo(&asimetrico.scene, SpatialGroupId::Global);

        println!(
            "  lamina      visible {} -> {} celdas, mayor mancha {} -> {}",
            agua_ahora.visibles,
            agua_asimetrica.visibles,
            agua_ahora.mayor_mancha,
            agua_asimetrica.mayor_mancha
        );
        println!(
            "  margen      Aguas al borde del plinto: -x {:.2}  +x {:.2}  -z {:.2}  +z {:.2}\n",
            aguas.min.x - plinto.min.x,
            plinto.max.x - aguas.max.x,
            aguas.min.z - plinto.min.z,
            plinto.max.z - aguas.max.z
        );

        // ------------------------------------------------ separated
        let reparto = reparto_separado(&candidato.scene);

        println!("  territorios candidato -> separated");
        for (nombre, grupo, delta) in [
            ("praderas", SpatialGroupId::Meadows, reparto.praderas),
            ("rompeolas", SpatialGroupId::Breakwater, reparto.rompeolas),
            ("aguas", SpatialGroupId::FlyingWaters, reparto.aguas),
            ("monolito", SpatialGroupId::Monolith, Vec3::zeros()),
            ("fondo", SpatialGroupId::ContinentBackground, Vec3::zeros()),
        ] {
            let caja = huella_del_grupo(&separado.scene, grupo);

            println!(
                "  {nombre:<11} x [{:6.2},{:6.2}]  z [{:6.2},{:6.2}]   delta {:+.2} {:+.2} {:+.2}",
                caja.min.x, caja.max.x, caja.min.z, caja.max.z, delta.x, delta.y, delta.z
            );
        }

        let huellas = [
            (
                "Praderas",
                huella_del_grupo(&separado.scene, SpatialGroupId::Meadows),
            ),
            (
                "Rompeolas",
                huella_del_grupo(&separado.scene, SpatialGroupId::Breakwater),
            ),
            (
                "Aguas",
                huella_del_grupo(&separado.scene, SpatialGroupId::FlyingWaters),
            ),
            (
                "Monolito",
                huella_del_grupo(&separado.scene, SpatialGroupId::Monolith),
            ),
        ];

        for a in 0..huellas.len() {
            for b in a + 1..huellas.len() {
                println!(
                    "  claro       {:<10} | {:<10} {:6.2}",
                    huellas[a].0,
                    huellas[b].0,
                    separacion_xz(&huellas[a].1, &huellas[b].1)
                );
            }
        }

        let aguas = huella_del_grupo(&separado.scene, SpatialGroupId::FlyingWaters);
        let plinto = huella_del_grupo(&separado.scene, SpatialGroupId::Global);
        println!(
            "  margen      Aguas al borde del plinto: -x {:.2}  +x {:.2}  -z {:.2}  +z {:.2}",
            aguas.min.x - plinto.min.x,
            plinto.max.x - aguas.max.x,
            aguas.min.z - plinto.min.z,
            plinto.max.z - aguas.max.z
        );
        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}\n",
            actual.scale.scene_radius,
            separado.scale.scene_radius,
            actual.scale.orbit_radius,
            separado.scale.orbit_radius
        );

        // ------------------------------------------------ grounded
        let apoyo = apoyo_del_rompeolas(&separado.scene);
        let sin_apoyo = |escena: &Scene| {
            let piezas = indices_por_grupo(escena, SpatialGroupId::Breakwater);

            piezas[..28]
                .iter()
                .filter(|&&i| {
                    let caja = escena.objects[i].primitive.bounds();

                    match pedestal_bajo(escena, i) {
                        None => true,
                        Some(techo) => caja.min.y > techo,
                    }
                })
                .count()
        };

        println!("  apoyo       separated -> grounded");
        println!(
            "  pilares     {} de 28 pilares de R-01 sin pedestal  ->  {}",
            sin_apoyo(&separado.scene),
            sin_apoyo(&apoyado.scene)
        );
        linea_territorial(
            "losa G-02",
            &separado.scene.objects[apoyo.indice].primitive.bounds(),
            &apoyado.scene.objects[apoyo.indice].primitive.bounds(),
        );
        linea_territorial(
            "rompeolas",
            &huella_del_grupo(&separado.scene, SpatialGroupId::Breakwater),
            &huella_del_grupo(&apoyado.scene, SpatialGroupId::Breakwater),
        );

        let losa = apoyado.scene.objects[apoyo.indice].primitive.bounds();
        let rompeolas_apoyado = huella_del_grupo(&apoyado.scene, SpatialGroupId::Breakwater);
        let plinto_apoyado = huella_del_grupo(&apoyado.scene, SpatialGroupId::Global);

        println!(
            "  losa        y [{:.2},{:.2}] sobre plinto {:.2};  macroforma arranca en {:.2}",
            losa.min.y,
            losa.max.y,
            plinto_apoyado.max.y,
            apoyado.scene.objects[indices_por_grupo(&apoyado.scene, SpatialGroupId::Breakwater)[0]]
                .primitive
                .bounds()
                .min
                .y
        );
        println!(
            "  rompeolas   y [{:.2},{:.2}];  bajo {:+.2} en bloque",
            rompeolas_apoyado.min.y, rompeolas_apoyado.max.y, apoyo.rompeolas.y
        );

        let huellas_apoyadas = [
            (
                "Praderas",
                huella_del_grupo(&apoyado.scene, SpatialGroupId::Meadows),
            ),
            (
                "Rompeolas",
                huella_del_grupo(&apoyado.scene, SpatialGroupId::Breakwater),
            ),
            (
                "Aguas",
                huella_del_grupo(&apoyado.scene, SpatialGroupId::FlyingWaters),
            ),
        ];

        for a in 0..huellas_apoyadas.len() {
            for b in a + 1..huellas_apoyadas.len() {
                println!(
                    "  claro       {:<10} | {:<10} {:6.2}",
                    huellas_apoyadas[a].0,
                    huellas_apoyadas[b].0,
                    separacion_xz(&huellas_apoyadas[a].1, &huellas_apoyadas[b].1)
                );
            }
        }

        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}\n",
            separado.scale.scene_radius,
            apoyado.scale.scene_radius,
            separado.scale.orbit_radius,
            apoyado.scale.orbit_radius
        );

        // ------------------------------------------------ staircase
        let orden = pilares_en_orden(&escalera.scene);
        let caja_de = |i: usize| escalera.scene.objects[i].primitive.bounds();
        let tercio = orden.len() / 3;
        let media = |ids: &[usize], f: &dyn Fn(&Aabb) -> f32| {
            ids.iter().map(|&i| f(&caja_de(i))).sum::<f32>() / ids.len() as f32
        };
        let techo = |c: &Aabb| c.max.y;
        let centro_x = |c: &Aabb| (c.min.x + c.max.x) * 0.5;

        let praderas_escalera = huella_del_grupo(&escalera.scene, SpatialGroupId::Meadows);
        let escalones = huella(&escalera.scene, &orden);
        let techos: Vec<f32> = orden.iter().map(|&i| caja_de(i).max.y).collect();
        let inversiones = techos.windows(2).filter(|par| par[1] < par[0]).count();

        println!("  escalera    grounded -> staircase");
        linea_territorial(
            "R-01",
            &huella(
                &apoyado.scene,
                &indices_por_grupo(&apoyado.scene, SpatialGroupId::Breakwater)[..28],
            ),
            &escalones,
        );
        println!(
            "  diagonal    x {:6.2} -> {:6.2}   z {:6.2} -> {:6.2}   (tercio bajo -> alto)",
            media(&orden[..tercio], &centro_x),
            media(&orden[orden.len() - tercio..], &centro_x),
            media(&orden[..tercio], &|c: &Aabb| (c.min.z + c.max.z) * 0.5),
            media(&orden[orden.len() - tercio..], &|c: &Aabb| (c.min.z
                + c.max.z)
                * 0.5)
        );
        println!(
            "  subida      techo {:6.2} -> {:6.2};  {inversiones} bajadas locales de {}",
            media(&orden[..tercio], &techo),
            media(&orden[orden.len() - tercio..], &techo),
            techos.len() - 1
        );

        let ultimo = caja_de(*orden.last().expect("hay pilares"));

        println!(
            "  llegada     ultimo pilar x [{:6.2},{:6.2}] z [{:6.2},{:6.2}];  claro a Praderas {:.2}",
            ultimo.min.x,
            ultimo.max.x,
            ultimo.min.z,
            ultimo.max.z,
            ultimo.min.z - praderas_escalera.max.z
        );

        let huellas_escalera = [
            (
                "Praderas",
                huella_del_grupo(&escalera.scene, SpatialGroupId::Meadows),
            ),
            (
                "Rompeolas",
                huella_del_grupo(&escalera.scene, SpatialGroupId::Breakwater),
            ),
            (
                "Aguas",
                huella_del_grupo(&escalera.scene, SpatialGroupId::FlyingWaters),
            ),
        ];

        for a in 0..huellas_escalera.len() {
            for b in a + 1..huellas_escalera.len() {
                println!(
                    "  claro       {:<10} | {:<10} {:6.2}",
                    huellas_escalera[a].0,
                    huellas_escalera[b].0,
                    separacion_xz(&huellas_escalera[a].1, &huellas_escalera[b].1)
                );
            }
        }

        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}\n",
            apoyado.scale.scene_radius,
            escalera.scale.scene_radius,
            apoyado.scale.orbit_radius,
            escalera.scale.orbit_radius
        );

        // ------------------------------------------------ combined
        let macizo = macizo_basal(&combinado.scene);
        let ascenso = ascenso_combinado(&combinado.scene);
        let laterales = laterales_combinados(&combinado.scene);
        let reparto_de = |ids: &[usize]| {
            let caja = huella(&combinado.scene, ids);
            let superficie = (caja.max.x - caja.min.x) * (caja.max.z - caja.min.z);
            let suma: f32 = ids
                .iter()
                .map(|&i| {
                    let c = combinado.scene.objects[i].primitive.bounds();

                    (c.max.x - c.min.x) * (c.max.z - c.min.z)
                })
                .sum();
            let techo = ids
                .iter()
                .map(|&i| combinado.scene.objects[i].primitive.bounds().max.y)
                .fold(f32::MIN, f32::max);

            (caja, superficie, suma, techo)
        };

        println!("  combinado   staircase -> combined");

        for (nombre, ids) in [
            ("macizo", &macizo),
            ("ascenso", &ascenso),
            ("barranco", &laterales),
        ] {
            let (caja, superficie, suma, techo) = reparto_de(ids);

            println!(
                "  {nombre:<11} {:2} prismas  x [{:6.2},{:6.2}] z [{:6.2},{:6.2}]  \
                 caja {superficie:5.2} huellas {suma:5.2}  cima {techo:.2}",
                ids.len(),
                caja.min.x,
                caja.max.x,
                caja.min.z,
                caja.max.z
            );
        }

        let praderas_comb = huella_del_grupo(&combinado.scene, SpatialGroupId::Meadows);
        let ultimo_comb = combinado.scene.objects[*ascenso.last().expect("hay ascenso")]
            .primitive
            .bounds();

        println!(
            "  llegada     ultimo prisma x [{:6.2},{:6.2}] z [{:6.2},{:6.2}];  claro a Praderas {:.2}",
            ultimo_comb.min.x,
            ultimo_comb.max.x,
            ultimo_comb.min.z,
            ultimo_comb.max.z,
            ultimo_comb.min.z - praderas_comb.max.z
        );

        let huellas_comb = [
            (
                "Praderas",
                huella_del_grupo(&combinado.scene, SpatialGroupId::Meadows),
            ),
            (
                "Rompeolas",
                huella_del_grupo(&combinado.scene, SpatialGroupId::Breakwater),
            ),
            (
                "Aguas",
                huella_del_grupo(&combinado.scene, SpatialGroupId::FlyingWaters),
            ),
        ];

        for a in 0..huellas_comb.len() {
            for b in a + 1..huellas_comb.len() {
                println!(
                    "  claro       {:<10} | {:<10} {:6.2}",
                    huellas_comb[a].0,
                    huellas_comb[b].0,
                    separacion_xz(&huellas_comb[a].1, &huellas_comb[b].1)
                );
            }
        }

        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}\n",
            escalera.scale.scene_radius,
            combinado.scale.scene_radius,
            escalera.scale.orbit_radius,
            combinado.scale.orbit_radius
        );

        // ------------------------------------------ grounded_ascent
        let cimas = cimas_seleccionadas(&apoyado.scene);
        let piezas_extension = indices_por_grupo(&extension.scene, SpatialGroupId::Breakwater);
        let macizo_restante: Vec<usize> = piezas_extension[..28]
            .iter()
            .copied()
            .filter(|i| !cimas.contains(i))
            .collect();

        let mut orden_cimas = cimas.clone();
        orden_cimas.sort_by(|a, b| {
            let za = extension.scene.objects[*b].primitive.bounds();
            let zb = extension.scene.objects[*a].primitive.bounds();

            ((za.min.z + za.max.z) * 0.5)
                .partial_cmp(&((zb.min.z + zb.max.z) * 0.5))
                .expect("no hay NaN")
        });

        println!("  extension   grounded -> grounded_ascent");
        println!(
            "  seleccion   {} prismas de 28;  macizo se queda con {}",
            cimas.len(),
            macizo_restante.len()
        );

        for &i in &orden_cimas {
            let antes = apoyado.scene.objects[i].primitive.bounds();
            let ahora = extension.scene.objects[i].primitive.bounds();
            let suelo = pedestal_bajo(&extension.scene, i).unwrap_or(0.0);

            println!(
                "  prisma {i:3}  x [{:6.2},{:6.2}] z [{:6.2},{:6.2}]  base {:5.2} sobre {:5.2}  \
                 techo {:5.2} (era {:5.2})",
                ahora.min.x,
                ahora.max.x,
                ahora.min.z,
                ahora.max.z,
                ahora.min.y,
                suelo,
                ahora.max.y,
                antes.max.y
            );
        }

        let praderas_ext = huella_del_grupo(&extension.scene, SpatialGroupId::Meadows);
        let ultimo_ext = extension.scene.objects[*orden_cimas.last().expect("hay extension")]
            .primitive
            .bounds();

        let huecos: Vec<f32> = orden_cimas
            .windows(2)
            .map(|par| {
                separacion_xz(
                    &extension.scene.objects[par[0]].primitive.bounds(),
                    &extension.scene.objects[par[1]].primitive.bounds(),
                )
            })
            .collect();

        println!(
            "  huecos      {:?}",
            huecos
                .iter()
                .map(|h| (h * 100.0).round() / 100.0)
                .collect::<Vec<f32>>()
        );
        println!(
            "  llegada     claro a Praderas {:.2}",
            ultimo_ext.min.z - praderas_ext.max.z
        );

        let huellas_ext = [
            (
                "Praderas",
                huella_del_grupo(&extension.scene, SpatialGroupId::Meadows),
            ),
            (
                "Rompeolas",
                huella_del_grupo(&extension.scene, SpatialGroupId::Breakwater),
            ),
            (
                "Aguas",
                huella_del_grupo(&extension.scene, SpatialGroupId::FlyingWaters),
            ),
        ];

        for a in 0..huellas_ext.len() {
            for b in a + 1..huellas_ext.len() {
                println!(
                    "  claro       {:<10} | {:<10} {:6.2}",
                    huellas_ext[a].0,
                    huellas_ext[b].0,
                    separacion_xz(&huellas_ext[a].1, &huellas_ext[b].1)
                );
            }
        }

        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}\n",
            apoyado.scale.scene_radius,
            extension.scale.scene_radius,
            apoyado.scale.orbit_radius,
            extension.scale.orbit_radius
        );

        // ------------------------------------------------ backfill
        let traseros = traseros_seleccionados(&extension.scene);
        let franja = franja_trasera(&relleno.scene);

        println!("  relleno     grounded_ascent -> backfill");
        println!(
            "  franja      z [{:.2},{:.2}] de la parcela;  cobertura en x {:.2} -> {:.2}",
            franja.0,
            franja.1,
            cobertura_de_la_franja(&extension.scene, franja),
            cobertura_de_la_franja(&relleno.scene, franja)
        );
        println!("  seleccion   {} prismas del fondo", traseros.len());

        for &i in &traseros {
            let antes = extension.scene.objects[i].primitive.bounds();
            let ahora = relleno.scene.objects[i].primitive.bounds();

            println!(
                "  prisma {i:3}  z {:5.2} -> {:5.2}   x [{:6.2},{:6.2}]  base {:5.2}  techo {:5.2}",
                (antes.min.z + antes.max.z) * 0.5,
                (ahora.min.z + ahora.max.z) * 0.5,
                ahora.min.x,
                ahora.max.x,
                ahora.min.y,
                ahora.max.y
            );
        }

        let huellas_bf = [
            (
                "Praderas",
                huella_del_grupo(&relleno.scene, SpatialGroupId::Meadows),
            ),
            (
                "Rompeolas",
                huella_del_grupo(&relleno.scene, SpatialGroupId::Breakwater),
            ),
            (
                "Aguas",
                huella_del_grupo(&relleno.scene, SpatialGroupId::FlyingWaters),
            ),
        ];

        for a in 0..huellas_bf.len() {
            for b in a + 1..huellas_bf.len() {
                println!(
                    "  claro       {:<10} | {:<10} {:6.2}",
                    huellas_bf[a].0,
                    huellas_bf[b].0,
                    separacion_xz(&huellas_bf[a].1, &huellas_bf[b].1)
                );
            }
        }

        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}\n",
            extension.scale.scene_radius,
            relleno.scale.scene_radius,
            extension.scale.orbit_radius,
            relleno.scale.orbit_radius
        );

        // ------------------------------------------ monolith_stair
        let elegidos_ruta = prismas_de_la_ruta(&apoyado.scene);
        let monolito_ruta = huella_del_grupo(&ruta.scene, SpatialGroupId::Monolith);
        let praderas_ruta = huella_del_grupo(&ruta.scene, SpatialGroupId::Meadows);

        let mut orden_ruta = elegidos_ruta.clone();
        orden_ruta.sort_by(|a, b| {
            let ca = ruta.scene.objects[*b].primitive.bounds();
            let cb = ruta.scene.objects[*a].primitive.bounds();

            ((ca.min.z + ca.max.z) * 0.5)
                .partial_cmp(&((cb.min.z + cb.max.z) * 0.5))
                .expect("no hay NaN")
        });

        println!("  ruta        grounded -> monolith_stair");
        println!(
            "  monolito    x [{:.2},{:.2}] z [{:.2},{:.2}];  banda libre detras: {:.2}",
            monolito_ruta.min.x,
            monolito_ruta.max.x,
            monolito_ruta.min.z,
            monolito_ruta.max.z,
            monolito_ruta.min.z - (praderas_ruta.max.z + CLARO)
        );
        println!(
            "  seleccion   {} prismas;  macizo se queda con {}",
            elegidos_ruta.len(),
            28 - elegidos_ruta.len()
        );

        for &i in &orden_ruta {
            let caja = ruta.scene.objects[i].primitive.bounds();
            let suelo = pedestal_bajo(&ruta.scene, i).unwrap_or(0.0);

            println!(
                "  prisma {i:3}  x [{:6.2},{:6.2}] z [{:6.2},{:6.2}]  suelo {:5.2}  techo {:5.2}  \
                 al monolito {:5.2}{}",
                caja.min.x,
                caja.max.x,
                caja.min.z,
                caja.max.z,
                suelo,
                caja.max.y,
                separacion_xz(&caja, &monolito_ruta),
                if caja.min.z < monolito_ruta.min.z {
                    "  (detras)"
                } else {
                    ""
                }
            );
        }

        let huellas_ruta = [
            (
                "Praderas",
                huella_del_grupo(&ruta.scene, SpatialGroupId::Meadows),
            ),
            (
                "Rompeolas",
                huella_del_grupo(&ruta.scene, SpatialGroupId::Breakwater),
            ),
            (
                "Aguas",
                huella_del_grupo(&ruta.scene, SpatialGroupId::FlyingWaters),
            ),
        ];

        for a in 0..huellas_ruta.len() {
            for b in a + 1..huellas_ruta.len() {
                println!(
                    "  claro       {:<10} | {:<10} {:6.2}",
                    huellas_ruta[a].0,
                    huellas_ruta[b].0,
                    separacion_xz(&huellas_ruta[a].1, &huellas_ruta[b].1)
                );
            }
        }

        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}\n",
            apoyado.scale.scene_radius,
            ruta.scale.scene_radius,
            apoyado.scale.orbit_radius,
            ruta.scale.orbit_radius
        );

        // ------------------------------------------ edge_breakwater
        let pilares_borde = &indices_por_grupo(&borde.scene, SpatialGroupId::Breakwater)[..28];
        let formacion = huella(&borde.scene, pilares_borde);
        let aguas_borde = huella_del_grupo(&borde.scene, SpatialGroupId::FlyingWaters);
        let praderas_borde = huella_del_grupo(&borde.scene, SpatialGroupId::Meadows);
        let monolito_borde = huella_del_grupo(&borde.scene, SpatialGroupId::Monolith);

        println!("  borde       grounded -> edge_breakwater");
        linea_territorial(
            "R-01",
            &huella(
                &apoyado.scene,
                &indices_por_grupo(&apoyado.scene, SpatialGroupId::Breakwater)[..28],
            ),
            &formacion,
        );
        println!(
            "  corredor    de Aguas {:.2} -> al claro de Praderas {:.2}",
            aguas_borde.min.x - formacion.max.x,
            formacion.min.z - praderas_borde.max.z
        );

        let fondo_formacion = formacion.max.z - formacion.min.z;
        let paso_banda = fondo_formacion / 6.0;
        let mut por_banda = [0usize; 6];

        for &i in pilares_borde {
            let caja = borde.scene.objects[i].primitive.bounds();

            for (b, cuenta) in por_banda.iter_mut().enumerate() {
                let desde = formacion.min.z + paso_banda * b as f32;

                if caja.min.z < desde + paso_banda && caja.max.z > desde {
                    *cuenta += 1;
                }
            }
        }

        println!("  bandas      {por_banda:?} prismas por tramo del corredor");

        let mut cerca: Vec<f32> = Vec::new();
        let mut lejos: Vec<f32> = Vec::new();

        for &i in pilares_borde {
            let caja = borde.scene.objects[i].primitive.bounds();

            if separacion_xz(&caja, &monolito_borde) <= 2.50 {
                cerca.push(caja.max.y);
            } else {
                lejos.push(caja.max.y);
            }
        }

        let media = |v: &[f32]| v.iter().sum::<f32>() / v.len() as f32;

        println!(
            "  descenso    {} prismas junto al Monolito rematan en {:.2};  los {} del \
             macizo lateral en {:.2}",
            cerca.len(),
            media(&cerca),
            lejos.len(),
            media(&lejos)
        );

        let suelos: std::collections::BTreeSet<i64> = pilares_borde
            .iter()
            .map(|&i| (borde.scene.objects[i].primitive.bounds().min.y * 100.0).round() as i64)
            .collect();

        println!(
            "  cotas       {} cotas de arranque distintas entre los 28",
            suelos.len()
        );

        let huellas_borde = [
            ("Praderas", praderas_borde),
            (
                "Rompeolas",
                huella_del_grupo(&borde.scene, SpatialGroupId::Breakwater),
            ),
            ("Aguas", aguas_borde),
        ];

        for a in 0..huellas_borde.len() {
            for b in a + 1..huellas_borde.len() {
                println!(
                    "  claro       {:<10} | {:<10} {:6.2}",
                    huellas_borde[a].0,
                    huellas_borde[b].0,
                    separacion_xz(&huellas_borde[a].1, &huellas_borde[b].1)
                );
            }
        }

        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}\n",
            apoyado.scale.scene_radius,
            borde.scale.scene_radius,
            apoyado.scale.orbit_radius,
            borde.scale.orbit_radius
        );

        // ------------------------------------------------ green_shelf
        let losa_id = masa_pedestal(&terraza.scene);
        let losa_antes = apoyado.scene.objects[losa_id].primitive.bounds();
        let losa_ahora = terraza.scene.objects[losa_id].primitive.bounds();
        let monolito_t = huella_del_grupo(&terraza.scene, SpatialGroupId::Monolith);
        let rompeolas_t = huella_del_grupo(&terraza.scene, SpatialGroupId::Breakwater);
        let praderas_t = huella_del_grupo(&terraza.scene, SpatialGroupId::Meadows);
        let aguas_t = huella_del_grupo(&terraza.scene, SpatialGroupId::FlyingWaters);

        println!("  terraza     grounded -> green_shelf");
        linea_territorial("losa G-02", &losa_antes, &losa_ahora);
        println!(
            "  planta      x [{:6.2},{:6.2}] z [{:6.2},{:6.2}]  (antes z [{:6.2},{:6.2}])",
            losa_ahora.min.x,
            losa_ahora.max.x,
            losa_ahora.min.z,
            losa_ahora.max.z,
            losa_antes.min.z,
            losa_antes.max.z
        );
        println!(
            "  cubre       Rompeolas x [{:6.2},{:6.2}] z [{:6.2},{:6.2}]:  sobra {:.2} / {:.2} \
             / {:.2} / {:.2}",
            rompeolas_t.min.x,
            rompeolas_t.max.x,
            rompeolas_t.min.z,
            rompeolas_t.max.z,
            rompeolas_t.min.x - losa_ahora.min.x,
            losa_ahora.max.x - rompeolas_t.max.x,
            rompeolas_t.min.z - losa_ahora.min.z,
            losa_ahora.max.z - rompeolas_t.max.z
        );
        println!(
            "  el tope     Monolito x [{:.2},{:.2}] z [{:.2},{:.2}];  la terraza para a {:.2} \
             de su cara delantera",
            monolito_t.min.x,
            monolito_t.max.x,
            monolito_t.min.z,
            monolito_t.max.z,
            losa_ahora.min.z - monolito_t.max.z
        );
        println!(
            "  inalcanzable  banda libre detras del Monolito: {:.2}",
            monolito_t.min.z - (praderas_t.max.z + CLARO)
        );
        println!(
            "  margenes    a Aguas {:.2}, a Praderas {:.2}, al borde del plinto {:.2}",
            aguas_t.min.x - losa_ahora.max.x,
            losa_ahora.min.z - praderas_t.max.z,
            huella_del_grupo(&terraza.scene, SpatialGroupId::Global)
                .max
                .z
                - losa_ahora.max.z
        );
        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}\n",
            apoyado.scale.scene_radius,
            terraza.scale.scene_radius,
            apoyado.scale.orbit_radius,
            terraza.scale.orbit_radius
        );

        // ------------------------------------------------- platforms
        let ids_plataformas = plataformas_de_borde(&terraza.scene);
        let praderas_p = huella_del_grupo(&cadena.scene, SpatialGroupId::Meadows);
        let monolito_p = huella_del_grupo(&cadena.scene, SpatialGroupId::Monolith);

        println!("  plataformas green_shelf -> platforms");

        for (n, &i) in ids_plataformas.iter().enumerate() {
            let caja = cadena.scene.objects[i].primitive.bounds();
            let nombre = match n {
                0 => "principal",
                1 => "segunda",
                _ => "tercera",
            };

            println!(
                "  {nombre:<11} obj {i:2}  x [{:6.2},{:6.2}] z [{:6.2},{:6.2}]  cota {:5.2}  \
                 al monolito {:5.2}",
                caja.min.x,
                caja.max.x,
                caja.min.z,
                caja.max.z,
                caja.max.y,
                separacion_xz(&caja, &monolito_p)
            );
        }

        for a in 0..ids_plataformas.len() {
            for b in a + 1..ids_plataformas.len() {
                println!(
                    "  hueco       obj {:2} | obj {:2}  {:6.2}",
                    ids_plataformas[a],
                    ids_plataformas[b],
                    separacion_xz(
                        &cadena.scene.objects[ids_plataformas[a]].primitive.bounds(),
                        &cadena.scene.objects[ids_plataformas[b]].primitive.bounds()
                    )
                );
            }
        }

        let mas_adelantada = ids_plataformas
            .iter()
            .map(|&i| cadena.scene.objects[i].primitive.bounds().min.z)
            .fold(f32::MAX, f32::min);

        println!(
            "  llegada     la mas adelantada queda a {:.2} de Praderas",
            mas_adelantada - praderas_p.max.z
        );
        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}\n",
            terraza.scale.scene_radius,
            cadena.scale.scene_radius,
            terraza.scale.orbit_radius,
            cadena.scale.orbit_radius
        );

        // ------------------------------------------- platforms_thin
        println!("  finas       platforms -> platforms_thin");

        for (n, &i) in ids_plataformas.iter().enumerate() {
            let antes = cadena.scene.objects[i].primitive.bounds();
            let ahora = finas.scene.objects[i].primitive.bounds();
            let nombre = match n {
                0 => "principal",
                1 => "segunda",
                _ => "tercera",
            };

            println!(
                "  {nombre:<11} ancho {:5.2} -> {:5.2} ({:3.0} %)   x [{:6.2},{:6.2}]   \
                 z [{:6.2},{:6.2}] sin tocar",
                antes.max.x - antes.min.x,
                ahora.max.x - ahora.min.x,
                (ahora.max.x - ahora.min.x) / (antes.max.x - antes.min.x) * 100.0,
                ahora.min.x,
                ahora.max.x,
                ahora.min.z,
                ahora.max.z
            );
        }

        for a in 0..ids_plataformas.len() {
            for b in a + 1..ids_plataformas.len() {
                println!(
                    "  hueco       obj {:2} | obj {:2}  {:6.2}",
                    ids_plataformas[a],
                    ids_plataformas[b],
                    separacion_xz(
                        &finas.scene.objects[ids_plataformas[a]].primitive.bounds(),
                        &finas.scene.objects[ids_plataformas[b]].primitive.bounds()
                    )
                );
            }
        }

        let hero_finas = finas.hero_camera();
        let cenital = camara_cenital(&finas);

        println!(
            "  hero        yaw {:.0} grados, elevacion {:.0}, radio {:.2};  ojo ({:.2}, {:.2}, {:.2})",
            HERO_YAW_DEGREES,
            EYE_ELEVATION_DEGREES,
            finas.scale.orbit_radius,
            hero_finas.eye.x,
            hero_finas.eye.y,
            hero_finas.eye.z
        );
        println!(
            "  cenital     yaw {:.0} grados, elevacion {:.0}, radio {:.2};  ojo ({:.2}, {:.2}, {:.2})",
            HERO_YAW_DEGREES,
            ELEVACION_CENITAL,
            finas.scale.orbit_radius,
            cenital.eye.x,
            cenital.eye.y,
            cenital.eye.z
        );
        println!(
            "  las dos     mismo centro ({:.2}, {:.2}, {:.2}) y mismo punto de mira ({:.2}, {:.2}, {:.2})\n",
            cenital.orbit_center.x,
            cenital.orbit_center.y,
            cenital.orbit_center.z,
            cenital.look_at.x,
            cenital.look_at.y,
            cenital.look_at.z
        );

        // ------------------------------------------ marked_platforms
        let ids_marcadas = plataformas_de_borde(&apoyado.scene);
        let monolito_m = huella_del_grupo(&marcadas.scene, SpatialGroupId::Monolith);
        let praderas_m = huella_del_grupo(&marcadas.scene, SpatialGroupId::Meadows);
        let aguas_m = huella_del_grupo(&marcadas.scene, SpatialGroupId::FlyingWaters);

        println!("  marcadas    grounded -> marked_platforms");

        for (n, &i) in ids_marcadas.iter().enumerate() {
            let caja = marcadas.scene.objects[i].primitive.bounds();
            let ancho = caja.max.x - caja.min.x;
            let fondo = caja.max.z - caja.min.z;
            let nombre = match n {
                0 => "principal",
                1 => "modulo corto",
                _ => "modulo largo",
            };

            println!(
                "  {nombre:<13} x [{:6.2},{:6.2}] z [{:6.2},{:6.2}]  cota {:4.2}  \
                 ancho {ancho:5.2} fondo {fondo:5.2}  z/x {:4.2}",
                caja.min.x,
                caja.max.x,
                caja.min.z,
                caja.max.z,
                caja.max.y,
                fondo / ancho
            );
        }

        for a in 0..ids_marcadas.len() {
            for b in a + 1..ids_marcadas.len() {
                println!(
                    "  hueco       obj {:2} | obj {:2}  {:6.2}",
                    ids_marcadas[a],
                    ids_marcadas[b],
                    separacion_xz(
                        &marcadas.scene.objects[ids_marcadas[a]].primitive.bounds(),
                        &marcadas.scene.objects[ids_marcadas[b]].primitive.bounds()
                    )
                );
            }
        }

        for &i in &ids_marcadas[1..] {
            let caja = marcadas.scene.objects[i].primitive.bounds();

            println!(
                "  sitio  {i:2}   al oeste del Monolito {:5.2};  a Praderas {:5.2};  \
                 a Aguas {:5.2} en x y {:5.2} en z",
                monolito_m.min.x - caja.max.x,
                caja.min.z - praderas_m.max.z,
                aguas_m.min.x - caja.max.x,
                aguas_m.min.z - caja.max.z
            );
        }

        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}\n",
            apoyado.scale.scene_radius,
            marcadas.scale.scene_radius,
            apoyado.scale.orbit_radius,
            marcadas.scale.orbit_radius
        );

        // -------------------------------------------- modular_support
        let ids_modular = plataformas_de_borde(&apoyado.scene);
        let losa_original = apoyado.scene.objects[ids_modular[0]].primitive.bounds();

        println!("  modular     grounded -> modular_support");
        println!(
            "  la losa     media {:.2} x {:.2};  ahora ninguna pasa de {:.2} de ancho",
            losa_original.max.x - losa_original.min.x,
            losa_original.max.z - losa_original.min.z,
            ids_modular
                .iter()
                .map(|&i| {
                    let c = modular.scene.objects[i].primitive.bounds();
                    c.max.x - c.min.x
                })
                .fold(f32::MIN, f32::max)
        );

        for (n, &i) in ids_modular.iter().enumerate() {
            let caja = modular.scene.objects[i].primitive.bounds();
            let ancho = caja.max.x - caja.min.x;
            let fondo = caja.max.z - caja.min.z;

            println!(
                "  modulo {n}    obj {i:2}  x [{:6.2},{:6.2}] z [{:6.2},{:6.2}]  cota {:4.2}  \
                 {ancho:5.2} x {fondo:5.2}   z/x {:4.2}",
                caja.min.x,
                caja.max.x,
                caja.min.z,
                caja.max.z,
                caja.max.y,
                fondo / ancho
            );
        }

        for a in 0..ids_modular.len() {
            for b in a + 1..ids_modular.len() {
                println!(
                    "  hueco       obj {:2} | obj {:2}  {:6.2}",
                    ids_modular[a],
                    ids_modular[b],
                    separacion_xz(
                        &modular.scene.objects[ids_modular[a]].primitive.bounds(),
                        &modular.scene.objects[ids_modular[b]].primitive.bounds()
                    )
                );
            }
        }

        let piezas_modular = indices_por_grupo(&modular.scene, SpatialGroupId::Breakwater);
        let movidas = piezas_modular
            .iter()
            .filter(|&&i| {
                let a = apoyado.scene.objects[i].primitive.bounds();
                let b = modular.scene.objects[i].primitive.bounds();

                (a.min.x - b.min.x).abs() > 1.0e-4
            })
            .count();
        let techo_del_plinto = huella_del_grupo(&modular.scene, SpatialGroupId::Global)
            .max
            .y;
        let en_el_plinto = piezas_modular
            .iter()
            .filter(|&&i| {
                modular.scene.objects[i].primitive.bounds().min.y < techo_del_plinto + 0.05
            })
            .count();

        println!(
            "  piezas      {} de 38 se corrieron en x para entrar en un modulo;  {} se posan \
             en el lienzo",
            movidas, en_el_plinto
        );
        println!(
            "  macizo      huella [{:6.2},{:6.2}] x [{:6.2},{:6.2}]",
            huella_del_grupo(&modular.scene, SpatialGroupId::Breakwater)
                .min
                .x,
            huella_del_grupo(&modular.scene, SpatialGroupId::Breakwater)
                .max
                .x,
            huella_del_grupo(&modular.scene, SpatialGroupId::Breakwater)
                .min
                .z,
            huella_del_grupo(&modular.scene, SpatialGroupId::Breakwater)
                .max
                .z
        );
        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}\n",
            apoyado.scale.scene_radius,
            modular.scale.scene_radius,
            apoyado.scale.orbit_radius,
            modular.scale.orbit_radius
        );

        // ------------------------------------------- second_island
        let elegida = masa_de_la_segunda_isla(&apoyado.scene);
        let antes_isla = apoyado.scene.objects[elegida].primitive.bounds();
        let caja_isla = isla.scene.objects[elegida].primitive.bounds();
        let principal_isla = isla.scene.objects[masa_pedestal(&apoyado.scene)]
            .primitive
            .bounds();

        println!("  isla        grounded -> second_island");
        println!(
            "  base        principal x [{:6.2},{:6.2}] z [{:6.2},{:6.2}] cota {:.2}  (intacta)",
            principal_isla.min.x,
            principal_isla.max.x,
            principal_isla.min.z,
            principal_isla.max.z,
            principal_isla.max.y
        );
        linea_territorial("masa G-02", &antes_isla, &caja_isla);
        println!(
            "  segunda     obj {elegida:2}  x [{:6.2},{:6.2}] z [{:6.2},{:6.2}]  cota {:.2}  \
             {:.2} x {:.2} = {:.2} de superficie",
            caja_isla.min.x,
            caja_isla.max.x,
            caja_isla.min.z,
            caja_isla.max.z,
            caja_isla.max.y,
            caja_isla.max.x - caja_isla.min.x,
            caja_isla.max.z - caja_isla.min.z,
            (caja_isla.max.x - caja_isla.min.x) * (caja_isla.max.z - caja_isla.min.z)
        );

        for (nombre, otra) in [
            ("base principal", principal_isla),
            (
                "Monolito",
                huella_del_grupo(&isla.scene, SpatialGroupId::Monolith),
            ),
            (
                "Praderas",
                huella_del_grupo(&isla.scene, SpatialGroupId::Meadows),
            ),
            (
                "Aguas",
                huella_del_grupo(&isla.scene, SpatialGroupId::FlyingWaters),
            ),
        ] {
            println!(
                "  claro       a {nombre:<15} {:6.2}",
                separacion_xz(&caja_isla, &otra)
            );
        }

        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}\n",
            apoyado.scale.scene_radius,
            isla.scale.scene_radius,
            apoyado.scale.orbit_radius,
            isla.scale.orbit_radius
        );

        // --------------------------------------------- edge_island
        let antes_borde = isla.scene.objects[elegida].primitive.bounds();
        let ahora_borde = isla_borde.scene.objects[elegida].primitive.bounds();
        let plinto_borde = huella_del_grupo(&isla_borde.scene, SpatialGroupId::Global);
        let principal_borde = isla_borde.scene.objects[masa_pedestal(&apoyado.scene)]
            .primitive
            .bounds();

        println!("  borde       second_island -> edge_island");
        println!(
            "  isla        {:.2} x {:.2} = {:.2}   ->   {:.2} x {:.2} = {:.2}",
            antes_borde.max.x - antes_borde.min.x,
            antes_borde.max.z - antes_borde.min.z,
            (antes_borde.max.x - antes_borde.min.x) * (antes_borde.max.z - antes_borde.min.z),
            ahora_borde.max.x - ahora_borde.min.x,
            ahora_borde.max.z - ahora_borde.min.z,
            (ahora_borde.max.x - ahora_borde.min.x) * (ahora_borde.max.z - ahora_borde.min.z)
        );
        println!(
            "  planta      x [{:6.2},{:6.2}] z [{:6.2},{:6.2}]  (antes x [{:6.2},{:6.2}] \
             z [{:6.2},{:6.2}])",
            ahora_borde.min.x,
            ahora_borde.max.x,
            ahora_borde.min.z,
            ahora_borde.max.z,
            antes_borde.min.x,
            antes_borde.max.x,
            antes_borde.min.z,
            antes_borde.max.z
        );
        println!(
            "  al lienzo   canto oeste {:.2},  canto sur {:.2}",
            ahora_borde.min.x - plinto_borde.min.x,
            ahora_borde.min.z - plinto_borde.min.z
        );

        for (nombre, otra) in [
            ("base principal", principal_borde),
            (
                "Monolito",
                huella_del_grupo(&isla_borde.scene, SpatialGroupId::Monolith),
            ),
            (
                "Praderas",
                huella_del_grupo(&isla_borde.scene, SpatialGroupId::Meadows),
            ),
            (
                "Aguas",
                huella_del_grupo(&isla_borde.scene, SpatialGroupId::FlyingWaters),
            ),
            (
                "Rompeolas",
                huella_del_grupo(&isla_borde.scene, SpatialGroupId::Breakwater),
            ),
        ] {
            println!(
                "  claro       a {nombre:<15} {:6.2}",
                separacion_xz(&ahora_borde, &otra)
            );
        }

        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}\n",
            isla.scale.scene_radius,
            isla_borde.scale.scene_radius,
            isla.scale.orbit_radius,
            isla_borde.scale.orbit_radius
        );

        // ---------------------------------------- edge_island_wide
        let antes_ancha = isla_borde.scene.objects[elegida].primitive.bounds();
        let ahora_ancha = isla_ancha.scene.objects[elegida].primitive.bounds();
        let praderas_ancha = huella_del_grupo(&isla_ancha.scene, SpatialGroupId::Meadows);

        println!("  ancha       edge_island -> edge_island_wide");
        println!(
            "  isla        {:.2} x {:.2} = {:.2}   ->   {:.2} x {:.2} = {:.2}",
            antes_ancha.max.x - antes_ancha.min.x,
            antes_ancha.max.z - antes_ancha.min.z,
            (antes_ancha.max.x - antes_ancha.min.x) * (antes_ancha.max.z - antes_ancha.min.z),
            ahora_ancha.max.x - ahora_ancha.min.x,
            ahora_ancha.max.z - ahora_ancha.min.z,
            (ahora_ancha.max.x - ahora_ancha.min.x) * (ahora_ancha.max.z - ahora_ancha.min.z)
        );
        println!(
            "  planta      x [{:6.2},{:6.2}] z [{:6.2},{:6.2}];  canto oeste y tramo z sin tocar",
            ahora_ancha.min.x, ahora_ancha.max.x, ahora_ancha.min.z, ahora_ancha.max.z
        );
        println!(
            "  el tope     Praderas arranca en x {:.2};  la isla para en {:.2}: claro {:.2}",
            praderas_ancha.min.x,
            ahora_ancha.max.x,
            praderas_ancha.min.x - ahora_ancha.max.x
        );

        for (nombre, otra) in [
            (
                "base principal",
                isla_ancha.scene.objects[masa_pedestal(&apoyado.scene)]
                    .primitive
                    .bounds(),
            ),
            (
                "Monolito",
                huella_del_grupo(&isla_ancha.scene, SpatialGroupId::Monolith),
            ),
            (
                "Aguas",
                huella_del_grupo(&isla_ancha.scene, SpatialGroupId::FlyingWaters),
            ),
            (
                "Rompeolas",
                huella_del_grupo(&isla_ancha.scene, SpatialGroupId::Breakwater),
            ),
        ] {
            println!(
                "  claro       a {nombre:<15} {:6.2}",
                separacion_xz(&ahora_ancha, &otra)
            );
        }

        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}\n",
            isla_borde.scale.scene_radius,
            isla_ancha.scale.scene_radius,
            isla_borde.scale.orbit_radius,
            isla_ancha.scale.orbit_radius
        );

        // -------------------------------------- long_island_stair
        let antes_larga = isla_ancha.scene.objects[elegida].primitive.bounds();
        let ahora_larga = isla_larga.scene.objects[elegida].primitive.bounds();
        let principal_larga = isla_larga.scene.objects[masa_pedestal(&apoyado.scene)]
            .primitive
            .bounds();
        let monolito_larga = huella_del_grupo(&isla_larga.scene, SpatialGroupId::Monolith);
        let escalones = cimas_seleccionadas(&apoyado.scene);

        println!("  larga       edge_island_wide -> long_island_stair");
        println!(
            "  isla        {:.2} x {:.2} = {:.2}   ->   {:.2} x {:.2} = {:.2}",
            antes_larga.max.x - antes_larga.min.x,
            antes_larga.max.z - antes_larga.min.z,
            (antes_larga.max.x - antes_larga.min.x) * (antes_larga.max.z - antes_larga.min.z),
            ahora_larga.max.x - ahora_larga.min.x,
            ahora_larga.max.z - ahora_larga.min.z,
            (ahora_larga.max.x - ahora_larga.min.x) * (ahora_larga.max.z - ahora_larga.min.z)
        );
        println!(
            "  planta      x [{:6.2},{:6.2}] z [{:6.2},{:6.2}];  solapa la base {:.2} en z",
            ahora_larga.min.x,
            ahora_larga.max.x,
            ahora_larga.min.z,
            ahora_larga.max.z,
            ahora_larga.max.z - principal_larga.min.z
        );

        for (nombre, otra) in [
            (
                "Monolito",
                huella_del_grupo(&isla_larga.scene, SpatialGroupId::Monolith),
            ),
            (
                "Praderas",
                huella_del_grupo(&isla_larga.scene, SpatialGroupId::Meadows),
            ),
            (
                "Aguas",
                huella_del_grupo(&isla_larga.scene, SpatialGroupId::FlyingWaters),
            ),
        ] {
            println!(
                "  claro       a {nombre:<10} {:6.2}",
                separacion_xz(&ahora_larga, &otra)
            );
        }

        let mut orden_escalera = escalones.clone();
        orden_escalera.sort_by(|a, b| {
            let ca = isla_larga.scene.objects[*a].primitive.bounds();
            let cb = isla_larga.scene.objects[*b].primitive.bounds();

            ((ca.min.z + ca.max.z) * 0.5)
                .partial_cmp(&((cb.min.z + cb.max.z) * 0.5))
                .expect("no hay NaN")
        });

        println!(
            "  escalera    {} prismas sobre la isla;  macizo se queda con {}",
            escalones.len(),
            28 - escalones.len()
        );

        for &i in &orden_escalera {
            let c = isla_larga.scene.objects[i].primitive.bounds();

            println!(
                "  escalon {i:3}  x [{:6.2},{:6.2}] z [{:6.2},{:6.2}]  base {:5.2}  techo {:5.2}",
                c.min.x, c.max.x, c.min.z, c.max.z, c.min.y, c.max.y
            );
        }

        println!(
            "  tope        la escalera remata en {:.2} y el Monolito en {:.2} ({:.0} %)",
            orden_escalera
                .iter()
                .map(|&i| isla_larga.scene.objects[i].primitive.bounds().max.y)
                .fold(f32::MIN, f32::max),
            monolito_larga.max.y,
            orden_escalera
                .iter()
                .map(|&i| isla_larga.scene.objects[i].primitive.bounds().max.y)
                .fold(f32::MIN, f32::max)
                / monolito_larga.max.y
                * 100.0
        );
        println!();

        // ------------------------------ connected_island_stair
        let conector = masa_de_la_transicion(&apoyado.scene);
        let caja_conector = unida.scene.objects[conector].primitive.bounds();
        let base_unida = unida.scene.objects[masa_pedestal(&apoyado.scene)]
            .primitive
            .bounds();
        let isla_unida = unida.scene.objects[masa_de_la_segunda_isla(&apoyado.scene)]
            .primitive
            .bounds();

        println!("  transicion  long_island_stair -> connected_island_stair");
        println!(
            "  conector    obj {conector:2}  x [{:6.2},{:6.2}] z [{:6.2},{:6.2}]  cota {:4.2}  \
             {:.2} x {:.2}",
            caja_conector.min.x,
            caja_conector.max.x,
            caja_conector.min.z,
            caja_conector.max.z,
            caja_conector.max.y,
            caja_conector.max.x - caja_conector.min.x,
            caja_conector.max.z - caja_conector.min.z
        );
        println!(
            "  cotas       base {:4.2}  ->  transicion {:4.2}  ->  isla {:4.2}",
            base_unida.max.y, caja_conector.max.y, isla_unida.max.y
        );
        println!(
            "  encadena    muerde la base {:.2} y la isla {:.2}",
            -separacion_xz(&caja_conector, &base_unida),
            -separacion_xz(&caja_conector, &isla_unida)
        );

        for (nombre, grupo) in [
            ("Monolito", SpatialGroupId::Monolith),
            ("Praderas", SpatialGroupId::Meadows),
            ("Aguas", SpatialGroupId::FlyingWaters),
            ("Rompeolas", SpatialGroupId::Breakwater),
        ] {
            println!(
                "  claro       a {nombre:<10} {:6.2}",
                separacion_xz(&caja_conector, &huella_del_grupo(&unida.scene, grupo))
            );
        }

        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}\n",
            isla_larga.scale.scene_radius,
            unida.scale.scene_radius,
            isla_larga.scale.orbit_radius,
            unida.scale.orbit_radius
        );

        // ------------------------------------------ clustered_stair
        let caja_conector_ag = agrupada.scene.objects[conector].primitive.bounds();
        let mut orden_ag = escalones.clone();
        orden_ag.sort_by(|a, b| {
            let ca = agrupada.scene.objects[*a].primitive.bounds();
            let cb = agrupada.scene.objects[*b].primitive.bounds();

            ((ca.min.z + ca.max.z) * 0.5)
                .partial_cmp(&((cb.min.z + cb.max.z) * 0.5))
                .expect("no hay NaN")
        });

        println!("  racimos     connected_island_stair -> clustered_stair");

        for &i in &orden_ag {
            let c = agrupada.scene.objects[i].primitive.bounds();
            let en_conector = caja_conector_ag.min.x <= c.min.x
                && caja_conector_ag.max.x >= c.max.x
                && caja_conector_ag.min.z <= c.min.z
                && caja_conector_ag.max.z >= c.max.z;

            println!(
                "  escalon {i:3}  x [{:6.2},{:6.2}] z [{:6.2},{:6.2}]  base {:5.2}  techo {:5.2}{}",
                c.min.x,
                c.max.x,
                c.min.z,
                c.max.z,
                c.min.y,
                c.max.y,
                if en_conector { "  (conector)" } else { "" }
            );
        }

        let centros_ag: Vec<(f32, f32)> = orden_ag
            .iter()
            .map(|&i| {
                let c = agrupada.scene.objects[i].primitive.bounds();

                ((c.min.x + c.max.x) * 0.5, (c.min.z + c.max.z) * 0.5)
            })
            .collect();

        let mas_solo = (0..centros_ag.len())
            .map(|k| {
                (0..centros_ag.len())
                    .filter(|j| *j != k)
                    .map(|j| {
                        ((centros_ag[k].0 - centros_ag[j].0).powi(2)
                            + (centros_ag[k].1 - centros_ag[j].1).powi(2))
                        .sqrt()
                    })
                    .fold(f32::MAX, f32::min)
            })
            .fold(f32::MIN, f32::max);

        println!("  aislamiento el escalon mas solo tiene companero a {mas_solo:.2}");
        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}",
            unida.scale.scene_radius,
            agrupada.scale.scene_radius,
            unida.scale.orbit_radius,
            agrupada.scale.orbit_radius
        );
        println!();

        // ------------------------------------------- island_mountain
        let montanosa = nivel_montana_en_isla();
        let doce = prismas_de_la_montana(&apoyado.scene);
        let zonas = zonas_de_la_montana(&montanosa);

        println!("  montana     clustered_stair -> island_mountain");
        println!(
            "  reparto     {} prismas bajan a la isla;  {} se quedan en el macizo basal",
            doce.len(),
            macizo_de_la_montana(&apoyado.scene).len()
        );

        for (nombre, zona) in ["pie", "centro", "salida"].iter().zip(&zonas) {
            if zona.is_empty() {
                println!("  zona {nombre:<7} vacia");
                continue;
            }

            let envuelve = huella(&montanosa.scene, zona);
            let techo_medio = zona
                .iter()
                .map(|&i| montanosa.scene.objects[i].primitive.bounds().max.y)
                .sum::<f32>()
                / zona.len() as f32;

            println!(
                "  zona {nombre:<7}{:2} prismas  x [{:6.2},{:6.2}] z [{:6.2},{:6.2}]  techo medio \
                 {:5.2}",
                zona.len(),
                envuelve.min.x,
                envuelve.max.x,
                envuelve.min.z,
                envuelve.max.z,
                techo_medio
            );

            for &i in zona {
                let c = montanosa.scene.objects[i].primitive.bounds();

                println!(
                    "    prisma {i:3}  x [{:6.2},{:6.2}] z [{:6.2},{:6.2}]  base {:5.2}  techo \
                     {:5.2}",
                    c.min.x, c.max.x, c.min.z, c.max.z, c.min.y, c.max.y
                );
            }
        }

        let monolito_mt = huella_del_grupo(&montanosa.scene, SpatialGroupId::Monolith);
        let cima = doce
            .iter()
            .map(|&i| montanosa.scene.objects[i].primitive.bounds().max.y)
            .fold(f32::MIN, f32::max);

        println!(
            "  tope        la montana remata en {cima:.2} y el Monolito en {:.2} ({:.0} %)",
            monolito_mt.max.y,
            cima / monolito_mt.max.y * 100.0
        );
        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}",
            agrupada.scale.scene_radius,
            montanosa.scale.scene_radius,
            agrupada.scale.orbit_radius,
            montanosa.scale.orbit_radius
        );
        println!();

        // ------------------------------------------- spacious_island
        let estilo = nivel_isla_estilo_spacious();
        let logica = logica_de_spacious(&nivel_espacioso().scene);
        let donantes_si = masas_tapadas(&unida.scene);
        let vistas_desde_arriba = impactos_por_objeto(&unida, &camara_cenital(&unida));

        println!("  spacious    connected_island_stair -> spacious_island");

        let restauradas = indices_por_grupo(&apoyado.scene, SpatialGroupId::Breakwater)
            .iter()
            .filter(|&&i| {
                unida.scene.objects[i].primitive.bounds()
                    != estilo.scene.objects[i].primitive.bounds()
            })
            .count();
        let distintas_de_grounded = indices_por_grupo(&apoyado.scene, SpatialGroupId::Breakwater)
            .iter()
            .filter(|&&i| {
                apoyado.scene.objects[i].primitive.bounds()
                    != estilo.scene.objects[i].primitive.bounds()
            })
            .count();

        println!(
            "  rompeolas   {restauradas} piezas vuelven a grounded;  distintas de grounded: \
             {distintas_de_grounded} de 38"
        );
        println!(
            "  logica      terraza grande {:.3} x {:.3} de la masa, sube {:.2};  alta {:.3} x \
             {:.3} de la grande, sube {:.2}",
            logica.largo_t1,
            logica.corto_t1,
            logica.salto_t1,
            logica.largo_t2,
            logica.corto_t2,
            logica.salto_t2
        );

        let isla_si = estilo.scene.objects[masa_de_la_segunda_isla(&apoyado.scene)]
            .primitive
            .bounds();

        println!(
            "  masa        obj {:2}  x [{:6.2},{:6.2}] z [{:6.2},{:6.2}]  cota {:4.2}  (intacta)",
            masa_de_la_segunda_isla(&apoyado.scene),
            isla_si.min.x,
            isla_si.max.x,
            isla_si.min.z,
            isla_si.max.z,
            isla_si.max.y
        );

        for (nombre, &i) in ["terraza", "alta"].iter().zip(donantes_si.iter()) {
            let antes = unida.scene.objects[i].primitive.bounds();
            let c = estilo.scene.objects[i].primitive.bounds();

            println!(
                "  {nombre:<11} obj {i:2}  x [{:6.2},{:6.2}] z [{:6.2},{:6.2}]  cota {:4.2}  \
                 {:.2} x {:.2}   (antes tapada: cota {:4.2}, {} rayos cenitales)",
                c.min.x,
                c.max.x,
                c.min.z,
                c.max.z,
                c.max.y,
                c.max.x - c.min.x,
                c.max.z - c.min.z,
                antes.max.y,
                vistas_desde_arriba[i]
            );

            for (otro, grupo) in [
                ("Monolito", SpatialGroupId::Monolith),
                ("Praderas", SpatialGroupId::Meadows),
                ("Aguas", SpatialGroupId::FlyingWaters),
                ("Rompeolas", SpatialGroupId::Breakwater),
            ] {
                println!(
                    "    claro     a {otro:<10} {:6.2}",
                    separacion_xz(&c, &huella_del_grupo(&estilo.scene, grupo))
                );
            }
        }

        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}",
            unida.scale.scene_radius,
            estilo.scale.scene_radius,
            unida.scale.orbit_radius,
            estilo.scale.orbit_radius
        );
        println!();

        // ------------------------------------------ second_breakwater
        let segundo = nivel_segundo_rompeolas();
        let nuevos = segundo_rompeolas(&segundo.scene);

        println!("  segundo     connected_island_stair -> second_breakwater");
        println!(
            "  conteo      {} - {} + {} = {}",
            unida.scene.objects.len(),
            indices_por_grupo(&unida.scene, SpatialGroupId::ContinentBackground).len()
                - indices_por_grupo(&segundo.scene, SpatialGroupId::ContinentBackground).len(),
            nuevos.len(),
            segundo.scene.objects.len()
        );

        let original_ok = indices_por_grupo(&apoyado.scene, SpatialGroupId::Breakwater)
            .iter()
            .zip(indices_por_grupo(
                &segundo.scene,
                SpatialGroupId::Breakwater,
            ))
            .filter(|(&a, b)| {
                apoyado.scene.objects[a].primitive.bounds()
                    == segundo.scene.objects[*b].primitive.bounds()
            })
            .count();

        println!("  rompeolas   {original_ok} de 38 piezas exactas a grounded");

        for &i in &nuevos {
            let c = segundo.scene.objects[i].primitive.bounds();

            println!(
                "  prisma {i:3}  x [{:6.2},{:6.2}] z [{:6.2},{:6.2}]  base {:5.2}  techo {:5.2}",
                c.min.x, c.max.x, c.min.z, c.max.z, c.min.y, c.max.y
            );
        }

        let formacion = huella(&segundo.scene, &nuevos);
        let original_huella = huella(
            &apoyado.scene,
            &indices_por_grupo(&apoyado.scene, SpatialGroupId::Breakwater),
        );

        for (otro, caja) in [
            (
                "Monolito",
                huella_del_grupo(&segundo.scene, SpatialGroupId::Monolith),
            ),
            (
                "Praderas",
                huella_del_grupo(&segundo.scene, SpatialGroupId::Meadows),
            ),
            (
                "Aguas",
                huella_del_grupo(&segundo.scene, SpatialGroupId::FlyingWaters),
            ),
            ("Rompeolas", original_huella),
        ] {
            println!(
                "  claro       a {otro:<10} {:6.2}",
                separacion_xz(&formacion, &caja)
            );
        }

        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}",
            unida.scale.scene_radius,
            segundo.scale.scene_radius,
            unida.scale.orbit_radius,
            segundo.scale.orbit_radius
        );
        println!();

        // ------------------------------------ second_breakwater_turned
        let invertido = nivel_segundo_rompeolas_invertido();
        let base_principal = apoyado.scene.objects[masa_pedestal(&apoyado.scene)]
            .primitive
            .bounds();

        println!("  invertido   second_breakwater -> second_breakwater_turned");
        println!("  conteo      {}", invertido.scene.objects.len());

        for &i in &segundo_rompeolas(&invertido.scene) {
            let antes = segundo.scene.objects[i].primitive.bounds();
            let c = invertido.scene.objects[i].primitive.bounds();

            println!(
                "  prisma {i:3}  techo {:5.2}  z [{:6.2},{:6.2}] -> [{:6.2},{:6.2}]  al Rompeolas \
                 {:5.2} -> {:5.2}  a la base {:5.2} -> {:5.2}",
                c.max.y,
                antes.min.z,
                antes.max.z,
                c.min.z,
                c.max.z,
                separacion_xz(&antes, &original_huella),
                separacion_xz(&c, &original_huella),
                separacion_xz(&antes, &base_principal),
                separacion_xz(&c, &base_principal)
            );
        }

        let formacion_invertida = huella(&invertido.scene, &segundo_rompeolas(&invertido.scene));

        for (otro, caja) in [
            (
                "Monolito",
                huella_del_grupo(&invertido.scene, SpatialGroupId::Monolith),
            ),
            (
                "Praderas",
                huella_del_grupo(&invertido.scene, SpatialGroupId::Meadows),
            ),
            (
                "Aguas",
                huella_del_grupo(&invertido.scene, SpatialGroupId::FlyingWaters),
            ),
            ("Rompeolas", original_huella),
        ] {
            println!(
                "  claro       a {otro:<10} {:6.2}",
                separacion_xz(&formacion_invertida, &caja)
            );
        }

        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}",
            segundo.scale.scene_radius,
            invertido.scale.scene_radius,
            segundo.scale.orbit_radius,
            invertido.scale.orbit_radius
        );
        println!();

        // ------------------------------------- second_breakwater_stair
        let escalonado = nivel_segundo_rompeolas_escalonado();
        let secundarios = segundo_rompeolas(&escalonado.scene);

        println!("  escalera    second_breakwater_turned -> second_breakwater_stair");
        println!("  conteo      {}", escalonado.scene.objects.len());

        // Bandas medidas: prismas con el mismo centro en `z`, de sur a norte.
        let mut por_z = secundarios.clone();
        let centro_z_de = |i: usize| {
            let c = escalonado.scene.objects[i].primitive.bounds();

            (c.min.z + c.max.z) * 0.5
        };

        por_z.sort_by(|a, b| {
            centro_z_de(*a)
                .partial_cmp(&centro_z_de(*b))
                .expect("no hay NaN")
        });

        let mut bandas: Vec<Vec<usize>> = Vec::new();

        for i in por_z {
            match bandas.last_mut() {
                Some(banda) if (centro_z_de(banda[0]) - centro_z_de(i)).abs() < 0.05 => {
                    banda.push(i)
                }
                _ => bandas.push(vec![i]),
            }
        }

        for (n, banda) in bandas.iter().enumerate() {
            let cajas: Vec<Aabb> = banda
                .iter()
                .map(|&i| escalonado.scene.objects[i].primitive.bounds())
                .collect();
            let cuantos = cajas.len() as f32;

            println!(
                "  banda {n}     {} prismas  z [{:6.2},{:6.2}]  ancho medio {:4.2}  techo medio \
                 {:4.2}  al Rompeolas {:5.2}  a la base {:5.2}",
                cajas.len(),
                cajas.iter().map(|c| c.min.z).fold(f32::MAX, f32::min),
                cajas.iter().map(|c| c.max.z).fold(f32::MIN, f32::max),
                cajas.iter().map(|c| c.max.x - c.min.x).sum::<f32>() / cuantos,
                cajas.iter().map(|c| c.max.y).sum::<f32>() / cuantos,
                cajas
                    .iter()
                    .map(|c| separacion_xz(c, &original_huella))
                    .sum::<f32>()
                    / cuantos,
                cajas
                    .iter()
                    .map(|c| separacion_xz(c, &base_principal))
                    .sum::<f32>()
                    / cuantos
            );
        }

        let escalera_huella = huella(&escalonado.scene, &secundarios);

        for (otro, caja) in [
            (
                "Monolito",
                huella_del_grupo(&escalonado.scene, SpatialGroupId::Monolith),
            ),
            (
                "Praderas",
                huella_del_grupo(&escalonado.scene, SpatialGroupId::Meadows),
            ),
            (
                "Aguas",
                huella_del_grupo(&escalonado.scene, SpatialGroupId::FlyingWaters),
            ),
            ("Rompeolas", original_huella),
        ] {
            println!(
                "  claro       a {otro:<10} {:6.2}",
                separacion_xz(&escalera_huella, &caja)
            );
        }

        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}",
            invertido.scale.scene_radius,
            escalonado.scale.scene_radius,
            invertido.scale.orbit_radius,
            escalonado.scale.orbit_radius
        );
        println!();

        // ------------------------------- second_breakwater_stair_dense
        let denso = nivel_segundo_rompeolas_escalonado_denso();
        let secundarios_densos = segundo_rompeolas(&denso.scene);

        println!("  densa       second_breakwater_stair -> second_breakwater_stair_dense");
        println!(
            "  conteo      {} - 2 + {} = {}",
            unida.scene.objects.len(),
            secundarios_densos.len(),
            denso.scene.objects.len()
        );

        let mut por_z_denso = secundarios_densos.clone();
        let centro_z_denso = |i: usize| {
            let c = denso.scene.objects[i].primitive.bounds();

            (c.min.z + c.max.z) * 0.5
        };

        por_z_denso.sort_by(|a, b| {
            centro_z_denso(*a)
                .partial_cmp(&centro_z_denso(*b))
                .expect("no hay NaN")
        });

        let mut bandas_densas: Vec<Vec<usize>> = Vec::new();

        for i in por_z_denso {
            match bandas_densas.last_mut() {
                Some(banda) if (centro_z_denso(banda[0]) - centro_z_denso(i)).abs() < 0.05 => {
                    banda.push(i)
                }
                _ => bandas_densas.push(vec![i]),
            }
        }

        for (n, banda) in bandas_densas.iter().enumerate() {
            let cajas: Vec<Aabb> = banda
                .iter()
                .map(|&i| denso.scene.objects[i].primitive.bounds())
                .collect();
            let cuantos = cajas.len() as f32;

            println!(
                "  banda {n}     {} prismas  x [{:6.2},{:6.2}] z [{:6.2},{:6.2}]  ancho {:4.2}  \
                 techo medio {:4.2}  al Rompeolas {:5.2}",
                cajas.len(),
                cajas.iter().map(|c| c.min.x).fold(f32::MAX, f32::min),
                cajas.iter().map(|c| c.max.x).fold(f32::MIN, f32::max),
                cajas.iter().map(|c| c.min.z).fold(f32::MAX, f32::min),
                cajas.iter().map(|c| c.max.z).fold(f32::MIN, f32::max),
                cajas.iter().map(|c| c.max.x - c.min.x).sum::<f32>() / cuantos,
                cajas.iter().map(|c| c.max.y).sum::<f32>() / cuantos,
                cajas
                    .iter()
                    .map(|c| separacion_xz(c, &original_huella))
                    .fold(f32::MAX, f32::min)
            );
        }

        println!(
            "  cobertura   {:.1} % de la isla  (la escalera de diez: {:.1} %)",
            cobertura_de_la_isla(&denso) * 100.0,
            cobertura_de_la_isla(&escalonado) * 100.0
        );

        let densa_huella = huella(&denso.scene, &secundarios_densos);

        for (otro, caja) in [
            (
                "Monolito",
                huella_del_grupo(&denso.scene, SpatialGroupId::Monolith),
            ),
            (
                "Praderas",
                huella_del_grupo(&denso.scene, SpatialGroupId::Meadows),
            ),
            (
                "Aguas",
                huella_del_grupo(&denso.scene, SpatialGroupId::FlyingWaters),
            ),
            ("Rompeolas", original_huella),
        ] {
            println!(
                "  claro       a {otro:<10} {:6.2}",
                separacion_xz(&densa_huella, &caja)
            );
        }

        println!(
            "  escala      S {:.4} -> {:.4},  orbita {:.2} -> {:.2}",
            escalonado.scale.scene_radius,
            denso.scale.scene_radius,
            escalonado.scale.orbit_radius,
            denso.scale.orbit_radius
        );
        println!();

        if !guardar(&trazar(&actual, &camara), &destino, "current.png") {
            return ExitCode::FAILURE;
        }
        if !guardar(&trazar(&candidato, &camara), &destino, "candidate.png") {
            return ExitCode::FAILURE;
        }

        // El ensayo territorial usa **su** toma hero. La cámara del nivel
        // vigente encuadra una esfera envolvente que ya no es la suya: con
        // ella el plinto nuevo saldría recortado, y la comparación mediría
        // el encuadre en vez de la composición.
        if !guardar(
            &trazar(&espacioso, &espacioso.hero_camera()),
            &destino,
            "spacious.png",
        ) {
            return ExitCode::FAILURE;
        }

        // La cadena reconstruida, también con **su** toma hero.
        if !guardar(
            &trazar(&conectado, &conectado.hero_camera()),
            &destino,
            "connected.png",
        ) {
            return ExitCode::FAILURE;
        }

        // El labio refinado, con **su** toma hero.
        if !guardar(
            &trazar(&refinado, &refinado.hero_camera()),
            &destino,
            "refined.png",
        ) {
            return ExitCode::FAILURE;
        }

        // La boca asimetrica, con **su** toma hero.
        if !guardar(
            &trazar(&asimetrico, &asimetrico.hero_camera()),
            &destino,
            "asymmetric.png",
        ) {
            return ExitCode::FAILURE;
        }

        // Los tres territorios, con **su** toma hero.
        if !guardar(
            &trazar(&separado, &separado.hero_camera()),
            &destino,
            "separated.png",
        ) {
            return ExitCode::FAILURE;
        }

        // El Rompeolas apoyado, con **su** toma hero.
        if !guardar(
            &trazar(&apoyado, &apoyado.hero_camera()),
            &destino,
            "grounded.png",
        ) {
            return ExitCode::FAILURE;
        }

        // La escalera irregular, con **su** toma hero.
        if !guardar(
            &trazar(&escalera, &escalera.hero_camera()),
            &destino,
            "staircase.png",
        ) {
            return ExitCode::FAILURE;
        }

        // El reparto combinado, con **su** toma hero.
        if !guardar(
            &trazar(&combinado, &combinado.hero_camera()),
            &destino,
            "combined.png",
        ) {
            return ExitCode::FAILURE;
        }

        // La extension desde la cima, con **su** toma hero.
        if !guardar(
            &trazar(&extension, &extension.hero_camera()),
            &destino,
            "grounded_ascent.png",
        ) {
            return ExitCode::FAILURE;
        }

        // El relleno de la cara -Z, con **su** toma hero.
        if !guardar(
            &trazar(&relleno, &relleno.hero_camera()),
            &destino,
            "backfill.png",
        ) {
            return ExitCode::FAILURE;
        }

        // La ruta tras el Monolito, con **su** toma hero.
        if !guardar(
            &trazar(&ruta, &ruta.hero_camera()),
            &destino,
            "monolith_stair.png",
        ) {
            return ExitCode::FAILURE;
        }

        // El rompeolas de borde, con **su** toma hero.
        if !guardar(
            &trazar(&borde, &borde.hero_camera()),
            &destino,
            "edge_breakwater.png",
        ) {
            return ExitCode::FAILURE;
        }

        // La terraza verde, con **su** toma hero.
        if !guardar(
            &trazar(&terraza, &terraza.hero_camera()),
            &destino,
            "green_shelf.png",
        ) {
            return ExitCode::FAILURE;
        }

        // La cadena de plataformas, con **su** toma hero.
        if !guardar(
            &trazar(&cadena, &cadena.hero_camera()),
            &destino,
            "platforms.png",
        ) {
            return ExitCode::FAILURE;
        }

        // Las plataformas finas, en sus dos tomas.
        if !guardar(
            &trazar(&finas, &finas.hero_camera()),
            &destino,
            "platforms_thin.png",
        ) {
            return ExitCode::FAILURE;
        }
        if !guardar(
            &trazar(&finas, &camara_cenital(&finas)),
            &destino,
            "platforms_thin_top.png",
        ) {
            return ExitCode::FAILURE;
        }

        // El escalonado denso, en sus dos tomas.
        if !guardar(
            &trazar(&denso, &denso.hero_camera()),
            &destino,
            "second_breakwater_stair_dense.png",
        ) {
            return ExitCode::FAILURE;
        }
        if !guardar(
            &trazar(&denso, &camara_cenital(&denso)),
            &destino,
            "second_breakwater_stair_dense_top.png",
        ) {
            return ExitCode::FAILURE;
        }

        // El segundo Rompeolas escalonado, en sus dos tomas.
        if !guardar(
            &trazar(&escalonado, &escalonado.hero_camera()),
            &destino,
            "second_breakwater_stair.png",
        ) {
            return ExitCode::FAILURE;
        }
        if !guardar(
            &trazar(&escalonado, &camara_cenital(&escalonado)),
            &destino,
            "second_breakwater_stair_top.png",
        ) {
            return ExitCode::FAILURE;
        }

        // El segundo Rompeolas invertido, en sus dos tomas.
        if !guardar(
            &trazar(&invertido, &invertido.hero_camera()),
            &destino,
            "second_breakwater_turned.png",
        ) {
            return ExitCode::FAILURE;
        }
        if !guardar(
            &trazar(&invertido, &camara_cenital(&invertido)),
            &destino,
            "second_breakwater_turned_top.png",
        ) {
            return ExitCode::FAILURE;
        }

        // El segundo Rompeolas, en sus dos tomas.
        if !guardar(
            &trazar(&segundo, &segundo.hero_camera()),
            &destino,
            "second_breakwater.png",
        ) {
            return ExitCode::FAILURE;
        }
        if !guardar(
            &trazar(&segundo, &camara_cenital(&segundo)),
            &destino,
            "second_breakwater_top.png",
        ) {
            return ExitCode::FAILURE;
        }

        // La isla estilo spacious, en sus dos tomas.
        if !guardar(
            &trazar(&estilo, &estilo.hero_camera()),
            &destino,
            "spacious_island.png",
        ) {
            return ExitCode::FAILURE;
        }
        if !guardar(
            &trazar(&estilo, &camara_cenital(&estilo)),
            &destino,
            "spacious_island_top.png",
        ) {
            return ExitCode::FAILURE;
        }

        // La montana sobre la isla, en sus dos tomas.
        if !guardar(
            &trazar(&montanosa, &montanosa.hero_camera()),
            &destino,
            "island_mountain.png",
        ) {
            return ExitCode::FAILURE;
        }
        if !guardar(
            &trazar(&montanosa, &camara_cenital(&montanosa)),
            &destino,
            "island_mountain_top.png",
        ) {
            return ExitCode::FAILURE;
        }

        // La escalera en racimos, en sus dos tomas.
        if !guardar(
            &trazar(&agrupada, &agrupada.hero_camera()),
            &destino,
            "clustered_stair.png",
        ) {
            return ExitCode::FAILURE;
        }
        if !guardar(
            &trazar(&agrupada, &camara_cenital(&agrupada)),
            &destino,
            "clustered_stair_top.png",
        ) {
            return ExitCode::FAILURE;
        }

        // La isla con su transicion, en sus dos tomas.
        if !guardar(
            &trazar(&unida, &unida.hero_camera()),
            &destino,
            "connected_island_stair.png",
        ) {
            return ExitCode::FAILURE;
        }
        if !guardar(
            &trazar(&unida, &camara_cenital(&unida)),
            &destino,
            "connected_island_stair_top.png",
        ) {
            return ExitCode::FAILURE;
        }

        // La isla larga con escalera, en sus dos tomas.
        if !guardar(
            &trazar(&isla_larga, &isla_larga.hero_camera()),
            &destino,
            "long_island_stair.png",
        ) {
            return ExitCode::FAILURE;
        }
        if !guardar(
            &trazar(&isla_larga, &camara_cenital(&isla_larga)),
            &destino,
            "long_island_stair_top.png",
        ) {
            return ExitCode::FAILURE;
        }

        // La isla ancha, en sus dos tomas.
        if !guardar(
            &trazar(&isla_ancha, &isla_ancha.hero_camera()),
            &destino,
            "edge_island_wide.png",
        ) {
            return ExitCode::FAILURE;
        }
        if !guardar(
            &trazar(&isla_ancha, &camara_cenital(&isla_ancha)),
            &destino,
            "edge_island_wide_top.png",
        ) {
            return ExitCode::FAILURE;
        }

        // La isla de borde, en sus dos tomas.
        if !guardar(
            &trazar(&isla_borde, &isla_borde.hero_camera()),
            &destino,
            "edge_island.png",
        ) {
            return ExitCode::FAILURE;
        }
        if !guardar(
            &trazar(&isla_borde, &camara_cenital(&isla_borde)),
            &destino,
            "edge_island_top.png",
        ) {
            return ExitCode::FAILURE;
        }

        // La segunda isla, en sus dos tomas: la marca se hizo sobre una
        // cenital y ahi es donde se ve si la parcela quedo ocupada.
        if !guardar(
            &trazar(&isla, &isla.hero_camera()),
            &destino,
            "second_island.png",
        ) {
            return ExitCode::FAILURE;
        }
        if !guardar(
            &trazar(&isla, &camara_cenital(&isla)),
            &destino,
            "second_island_top.png",
        ) {
            return ExitCode::FAILURE;
        }

        // El soporte modular, en sus dos tomas: la marca se hizo sobre una
        // cenital, asi que la comparacion honesta va tambien en cenital.
        if !guardar(
            &trazar(&modular, &modular.hero_camera()),
            &destino,
            "modular_support.png",
        ) {
            return ExitCode::FAILURE;
        }
        if !guardar(
            &trazar(&modular, &camara_cenital(&modular)),
            &destino,
            "modular_support_top.png",
        ) {
            return ExitCode::FAILURE;
        }

        // Los modulos de la parcela marcada, en sus dos tomas. La cenital
        // usa la misma camara derivada que `platforms_thin_top`: mismo
        // centro, mismo punto de mira, mismo radio orbital y 78 grados de
        // elevacion. Es la vista sobre la que se hizo la marca roja, asi
        // que es la unica en la que la comparacion es directa.
        if !guardar(
            &trazar(&marcadas, &marcadas.hero_camera()),
            &destino,
            "marked_platforms.png",
        ) {
            return ExitCode::FAILURE;
        }
        if !guardar(
            &trazar(&marcadas, &camara_cenital(&marcadas)),
            &destino,
            "marked_platforms_top.png",
        ) {
            return ExitCode::FAILURE;
        }

        println!("\n  current y candidate comparten camara, luces, estado y assets:");
        println!("  entre esas dos la diferencia es geometrica, sin overlays ni 2D.");
        println!("  spacious y connected traen ademas otro territorio y, por eso,");
        println!("  cada uno su toma hero derivada de su propia escala medida.");
        println!("  connected corrige lo que la revision senalo en spacious: la");
        println!("  cadena Praderas -> Rompeolas -> terreno -> bahia sigue unida.");
        println!("  refined corrige lo que la revision senalo en connected: el");
        println!("  labio costero se escalona y la bahia se abre por el centro.");
        println!("  asymmetric saca esa boca del eje: un bloque cruza el canal y");
        println!("  el flanco largo retrocede. El canal sigue abierto.");
        println!("  separated abandona la premisa de cadena fisica: tres territorios");
        println!("  con parcela propia y claro medido entre cada par.");
        println!("  grounded corrige el unico hallazgo sobre separated: el Rompeolas");
        println!("  se posa en una losa de G-02 y ningun prisma queda en el aire.");
        println!("  staircase reparte los 28 prismas de R-01 en una subida diagonal");
        println!("  hacia la izquierda de Praderas, irregular y sin tocarla.");
        println!("  combined deja macizo en la parcela, sube solo una parte y anade");
        println!("  dos racimos de barranco en el flanco que mira a la camara.");
        println!("  grounded_ascent no toca grounded: solo saca sus seis prismas mas");
        println!("  altos hacia Praderas, y el terreno bajo cada uno fija su cota.");
        println!("  backfill trae cinco prismas del fondo a la cara -Z de la parcela");
        println!("  sin salir de ella: la cara gana cuerpo y el claro no se toca.");
        println!("  monolith_stair saca siete prismas del macizo y los escalona");
        println!("  rodeando el Monolito hasta la banda que queda detras de el.");
        println!("  edge_breakwater reparte los 28 por el corredor entero entre");
        println!("  Aguas y Praderas, y los baja al pasar junto al Monolito.");
        println!("  green_shelf no mueve prismas: alarga la losa verde de G-02 por el");
        println!("  borde hasta donde el Monolito la deja, que es menos de lo pedido.");
        println!("  platforms rodea ese tope con dos masas mas de G-02: tres plataformas");
        println!("  escalonadas por el oeste, con lienzo visible entre ellas.");
        println!("  platforms_thin las adelgaza en x sin tocar su ruta en z, y va en dos");
        println!("  tomas: la hero y una cenital a 78 grados, misma orbita derivada.");
        println!("  marked_platforms cambia la logica: modulos compactos, estrechos en x");
        println!("  y largos en z, sueltos en el corredor oeste del Monolito.");
        println!("  modular_support lleva esa logica al soporte entero: se acabo la");
        println!("  megaplataforma, las tres masas de G-02 son franjas con hueco.");
        println!("  second_island deja grounded intacto y anade una segunda plataforma");
        println!("  verde grande en la parcela marcada, con claro por los cuatro lados.");
        println!("  edge_island alarga esa misma isla corriendola al oeste: fuera de la");
        println!("  abscisa de Praderas el eje z se libera y el fondo se multiplica.");
        println!("  edge_island_wide la ensancha hasta el tope: crece solo hacia +x y");
        println!("  se para donde Praderas le deja exactamente un claro.");
        println!("  long_island_stair la alarga dentro de la base -unico solape que se");
        println!("  autorizo- y le monta encima una escalera de seis prismas.");
        println!("  connected_island_stair mete una terraza de transicion en la zona de");
        println!("  solape: un escalon entre la base y la isla, con otra orientacion.");
        println!("  clustered_stair junta los seis escalones en tres racimos de dos: el");
        println!("  alto pisa la terraza conectora y los otros dos van sobre la isla.");
        println!("  island_mountain baja doce prismas y los reparte en tres zonas: pie");
        println!("  denso junto a la base, centro con la cima en la espina y salida corta.");
        println!("  spacious_island devuelve el Rompeolas a grounded y compone la isla como");
        println!("  Praderas en spacious: masa principal y dos terrazas anidadas de G-02.");
        println!("  second_breakwater quita esas dos masas y pone sobre la isla diez prismas");
        println!("  con el lenguaje del primer Rompeolas: 154 - 2 + 10 = 162 objetos.");
        println!("  second_breakwater_turned le da la vuelta: cima y racimo hacia el");
        println!("  Rompeolas original y la base, masa basal hacia el lado libre.");
        println!("  second_breakwater_stair la hace escalera: cuatro bandas que ganan");
        println!("  grosor y altura desde el extremo libre hasta el Rompeolas original.");
        println!("  second_breakwater_stair_dense sube a dieciseis prismas en bandas 6/5/3/2");
        println!("  y acerca la cima a 0.40 del Rompeolas original: 154 - 2 + 16 = 168.");
        println!("  Esto no es una aprobacion estetica: es lo que hay que mirar.");

        ExitCode::SUCCESS
    }
}

#[cfg(test)]
mod tests {
    use super::imp::*;
    use expedition33_continente_inacabado::bounds::Aabb;
    use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
    use expedition33_continente_inacabado::primitive::Primitive;
    use expedition33_continente_inacabado::scene::{Scene, SpatialGroupId};
    use expedition33_continente_inacabado::scene_builder::{
        derive_orbit_radius, eye_at_yaw, measure_scene_radius, Blockout, HERO_YAW_DEGREES,
    };
    use nalgebra_glm::Vec3;

    /// El conteo del nivel seguro vigente. Si esto cambia, el candidato deja
    /// de ser comparable y hay que revisar el preview entero.
    const OBJETOS: usize = 154;

    /// Las treinta y cuatro piezas que el candidato sustituye: veintiocho de
    /// `R-01` y seis de `R-02`.
    const SUSTITUIDOS: usize = 34;

    /// Los cuatro soportes de `R-03`, que no se tocan.
    const CONSERVADOS: usize = 4;

    #[test]
    fn el_candidato_conserva_el_conteo_del_nivel() {
        let actual = nivel();
        let candidato = nivel_candidato();

        assert_eq!(
            actual.scene.objects.len(),
            OBJETOS,
            "el nivel vigente ya no tiene {OBJETOS} objetos"
        );
        assert_eq!(
            candidato.scene.objects.len(),
            OBJETOS,
            "el candidato cambio el conteo del nivel"
        );
    }

    #[test]
    fn el_rompeolas_tiene_las_treinta_y_ocho_piezas_esperadas() {
        let actual = nivel();
        let indices = indices_de_rompeolas(&actual.scene);

        assert_eq!(
            indices.len(),
            SUSTITUIDOS + CONSERVADOS,
            "el Rompeolas no tiene 38 piezas"
        );

        // Y son contiguas: el generador las emite seguidas, que es lo que
        // permite separar `R-01`/`R-02` de `R-03` por posicion.
        for par in indices.windows(2) {
            assert_eq!(
                par[1],
                par[0] + 1,
                "las piezas del Rompeolas no son contiguas"
            );
        }
    }

    #[test]
    fn la_sustitucion_se_limita_a_las_treinta_y_cuatro_piezas() {
        let actual = nivel();
        let candidato = nivel_candidato();
        let indices = indices_de_rompeolas(&actual.scene);
        let (sustituidos, conservados) = indices.split_at(SUSTITUIDOS);

        assert_eq!(conservados.len(), CONSERVADOS);

        // Todo lo que no es `R-01`/`R-02` queda **byte a byte** igual: misma
        // caja, mismos materiales, mismos grupos.
        let mut tocados = 0usize;

        for i in 0..OBJETOS {
            let a = &actual.scene.objects[i];
            let b = &candidato.scene.objects[i];

            let igual = a.primitive.bounds() == b.primitive.bounds()
                && a.initial_material == b.initial_material
                && a.final_material == b.final_material
                && a.spatial_group == b.spatial_group
                && a.reveal_group == b.reveal_group;

            if sustituidos.contains(&i) {
                if !igual {
                    tocados += 1;
                }
            } else {
                assert!(igual, "el objeto {i} cambio y no deberia");
            }
        }

        assert_eq!(
            tocados, SUSTITUIDOS,
            "solo {tocados} de {SUSTITUIDOS} piezas cambiaron: el candidato no sustituyo todo R-01/R-02"
        );
    }

    #[test]
    fn los_cuatro_soportes_quedan_intactos() {
        let actual = nivel();
        let candidato = nivel_candidato();
        let indices = indices_de_rompeolas(&actual.scene);

        for &i in &indices[SUSTITUIDOS..] {
            assert_eq!(
                actual.scene.objects[i].primitive.bounds(),
                candidato.scene.objects[i].primitive.bounds(),
                "el soporte {i} de R-03 se movio"
            );
        }
    }

    #[test]
    fn el_candidato_es_determinista() {
        let uno = nivel_candidato();
        let dos = nivel_candidato();

        assert_eq!(uno.scene.objects.len(), dos.scene.objects.len());

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }
    }

    #[test]
    fn el_candidato_conserva_materiales_y_grupos_del_rompeolas() {
        let actual = nivel();
        let candidato = nivel_candidato();

        for &i in &indices_de_rompeolas(&actual.scene) {
            let a = &actual.scene.objects[i];
            let b = &candidato.scene.objects[i];

            assert_eq!(a.initial_material, b.initial_material, "objeto {i}");
            assert_eq!(a.final_material, b.final_material, "objeto {i}");
            assert_eq!(b.spatial_group, SpatialGroupId::Breakwater, "objeto {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "objeto {i}");
        }
    }

    /// Ruta A: las veintiocho piezas de `R-01` siguen siendo prismas
    /// hexagonales.
    #[cfg(feature = "hex-prism")]
    #[test]
    fn en_la_ruta_a_las_piezas_de_r01_son_prismas() {
        let candidato = nivel_candidato();
        let indices = indices_de_rompeolas(&candidato.scene);

        for &i in &indices[..28] {
            assert!(
                matches!(candidato.scene.objects[i].primitive, Primitive::HexPrism(_)),
                "la pieza {i} de R-01 dejo de ser un prisma en la Ruta A"
            );
        }
    }

    /// Ruta B: las veintiocho piezas de `R-01` siguen siendo cuboides.
    #[cfg(not(feature = "hex-prism"))]
    #[test]
    fn en_la_ruta_b_las_piezas_de_r01_son_cuboides() {
        let candidato = nivel_candidato();
        let indices = indices_de_rompeolas(&candidato.scene);

        for &i in &indices[..28] {
            assert!(
                matches!(candidato.scene.objects[i].primitive, Primitive::Cuboid(_)),
                "la pieza {i} de R-01 dejo de ser un cuboide en la Ruta B"
            );
        }
    }

    #[test]
    fn la_formacion_desciende_hacia_la_bahia() {
        // La intencion visual que se ensaya: cresta hacia el interior y
        // caida escalonada hacia la costa. Se mide sobre las cajas, no
        // sobre la imagen.
        let candidato = nivel_candidato();
        let indices = indices_de_rompeolas(&candidato.scene);
        let piezas: Vec<_> = indices[..SUSTITUIDOS]
            .iter()
            .map(|&i| candidato.scene.objects[i].primitive.bounds())
            .collect();

        let z_medio = piezas.iter().map(|b| b.max.z).sum::<f32>() / piezas.len() as f32;

        let altura = |lejos: bool| {
            let grupo: Vec<f32> = piezas
                .iter()
                .filter(|b| (b.max.z > z_medio) == lejos)
                .map(|b| b.max.y - b.min.y)
                .collect();

            grupo.iter().sum::<f32>() / grupo.len().max(1) as f32
        };

        let interior = altura(false);
        let costa = altura(true);

        assert!(
            interior > costa * 1.25,
            "la cresta mide {interior:.2} y la costa {costa:.2}: no hay descenso"
        );
    }

    #[test]
    fn la_formacion_varia_de_grosor_en_racimos() {
        // F-06 de la propuesta: racimos de grosor distinto, no un cepillo de
        // dientes con todas las piezas iguales.
        let candidato = nivel_candidato();
        let indices = indices_de_rompeolas(&candidato.scene);

        let anchos: Vec<f32> = indices[..28]
            .iter()
            .map(|&i| {
                let b = candidato.scene.objects[i].primitive.bounds();

                b.max.x - b.min.x
            })
            .collect();

        let minimo = anchos.iter().copied().fold(f32::MAX, f32::min);
        let maximo = anchos.iter().copied().fold(f32::MIN, f32::max);

        assert!(
            maximo > minimo * 1.3,
            "los anchos van de {minimo:.3} a {maximo:.3}: no hay racimos"
        );
    }

    // ------------------------------------------------------------------
    // El ensayo territorial: `spacious`
    // ------------------------------------------------------------------

    /// Tolerancia de las comparaciones de tamaño y desplazamiento.
    ///
    /// Reconstruir una primitiva trasladada pasa por su caja envolvente: el
    /// centro y los semiejes se miden ahí y la forma se vuelve a construir
    /// desde esos números. El viaje introduce error de redondeo de `f32`.
    /// No es una traslación aproximada, es coma flotante; comparar por
    /// igualdad exacta convertiría un test de geometría en un test de ULPs.
    const EPS: f32 = 1.0e-4;

    /// Piezas por región, según el inventario del nivel seguro.
    const PRADERAS: usize = 37;
    const ROMPEOLAS: usize = 38;
    const AGUAS: usize = 58;

    /// El plinto `G-01` del ensayo territorial. El vigente mide `17 x 15`.
    const PLINTO_X: f32 = 22.0;
    const PLINTO_Z: f32 = 19.0;

    fn indices_del_grupo(diorama: &Blockout, grupo: SpatialGroupId) -> Vec<usize> {
        indices_por_grupo(&diorama.scene, grupo)
    }

    fn centro_de(caja: &Aabb) -> Vec3 {
        (caja.min + caja.max) * 0.5
    }

    fn tamano_de(caja: &Aabb) -> Vec3 {
        caja.max - caja.min
    }

    fn mismo_tipo(a: &Primitive, b: &Primitive) -> bool {
        match (a, b) {
            (Primitive::Cuboid(_), Primitive::Cuboid(_)) => true,
            #[cfg(feature = "hex-prism")]
            (Primitive::HexPrism(_), Primitive::HexPrism(_)) => true,
            #[cfg(feature = "hex-prism")]
            _ => false,
        }
    }

    #[test]
    fn el_nivel_espacioso_conserva_el_conteo_del_nivel() {
        let espacioso = nivel_espacioso();

        assert_eq!(
            espacioso.scene.objects.len(),
            OBJETOS,
            "el ensayo territorial cambio el conteo del nivel"
        );
    }

    #[test]
    fn el_plinto_espacioso_mide_veintidos_por_diecinueve() {
        let candidato = nivel_candidato();
        let espacioso = nivel_espacioso();

        let indices = indices_del_grupo(&espacioso, SpatialGroupId::Global);
        assert_eq!(indices.len(), 1, "G-01 deberia ser la unica pieza Global");

        let i = indices[0];
        let antes = candidato.scene.objects[i].primitive.bounds();
        let ahora = espacioso.scene.objects[i].primitive.bounds();

        let tamano = tamano_de(&ahora);

        assert!(
            (tamano.x - PLINTO_X).abs() < EPS,
            "el plinto mide {:.4} de ancho y el ensayo pide {PLINTO_X}",
            tamano.x
        );
        assert!(
            (tamano.z - PLINTO_Z).abs() < EPS,
            "el plinto mide {:.4} de fondo y el ensayo pide {PLINTO_Z}",
            tamano.z
        );

        // El plinto crece en planta, no en altura ni de sitio: es el suelo
        // del diorama, y moverlo desplazaria todo lo que se apoya en el.
        assert!(
            (tamano.y - tamano_de(&antes).y).abs() < EPS,
            "el plinto cambio de espesor"
        );
        assert!(
            (centro_de(&ahora) - centro_de(&antes)).magnitude() < EPS,
            "el plinto se movio de centro"
        );
    }

    #[test]
    fn el_monolito_no_se_mueve() {
        let candidato = nivel_candidato();
        let espacioso = nivel_espacioso();

        let indices = indices_del_grupo(&espacioso, SpatialGroupId::Monolith);
        assert!(!indices.is_empty(), "no hay Monolito que comprobar");

        for &i in &indices {
            assert_eq!(
                candidato.scene.objects[i].primitive.bounds(),
                espacioso.scene.objects[i].primitive.bounds(),
                "la pieza {i} del Monolito se movio: deja de servir de ancla"
            );
        }

        // Y la altura medida que alimenta el encuadre tampoco.
        assert_eq!(
            candidato.scale.monolith_height, espacioso.scale.monolith_height,
            "el Monolito cambio de altura"
        );
    }

    #[test]
    fn el_fondo_continental_no_se_mueve() {
        // Decision explicita del ensayo: el arco costero de `G-02` se queda
        // donde esta. El encargo enumera lo que se mueve —Praderas,
        // Rompeolas, Aguas— y el fondo no esta en esa lista.
        let candidato = nivel_candidato();
        let espacioso = nivel_espacioso();

        for &i in &indices_del_grupo(&espacioso, SpatialGroupId::ContinentBackground) {
            assert_eq!(
                candidato.scene.objects[i].primitive.bounds(),
                espacioso.scene.objects[i].primitive.bounds(),
                "la masa {i} del fondo continental se movio"
            );
        }
    }

    #[test]
    fn las_regiones_solo_se_trasladan() {
        let candidato = nivel_candidato();
        let espacioso = nivel_espacioso();

        for (grupo, piezas) in [
            (SpatialGroupId::Meadows, PRADERAS),
            (SpatialGroupId::Breakwater, ROMPEOLAS),
            (SpatialGroupId::FlyingWaters, AGUAS),
        ] {
            let indices = indices_del_grupo(&espacioso, grupo);

            assert_eq!(
                indices.len(),
                piezas,
                "{grupo:?} deberia tener {piezas} piezas"
            );

            let mut desplazamiento: Option<Vec3> = None;

            for &i in &indices {
                let a = &candidato.scene.objects[i];
                let b = &espacioso.scene.objects[i];

                assert!(
                    mismo_tipo(&a.primitive, &b.primitive),
                    "la pieza {i} de {grupo:?} cambio de primitiva"
                );
                assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
                assert_eq!(a.final_material, b.final_material, "pieza {i}");
                assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
                assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

                let antes = a.primitive.bounds();
                let ahora = b.primitive.bounds();

                let delta_tamano = tamano_de(&ahora) - tamano_de(&antes);
                assert!(
                    delta_tamano.magnitude() < EPS,
                    "la pieza {i} de {grupo:?} cambio de tamano en {:.5}",
                    delta_tamano.magnitude()
                );

                let delta = centro_de(&ahora) - centro_de(&antes);

                match desplazamiento {
                    None => desplazamiento = Some(delta),
                    Some(comun) => assert!(
                        (delta - comun).magnitude() < EPS,
                        "la pieza {i} de {grupo:?} se movio {delta:?} y el resto {comun:?}: eso no es una traslacion de region"
                    ),
                }
            }

            let comun = desplazamiento.expect("la region tiene piezas");

            assert!(
                comun.magnitude() > EPS,
                "{grupo:?} no se movio: el ensayo territorial no la redistribuyo"
            );
        }
    }

    #[test]
    fn el_nivel_espacioso_es_determinista() {
        let uno = nivel_espacioso();
        let dos = nivel_espacioso();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
        assert_eq!(
            uno.anchors.hero_camera_anchor,
            dos.anchors.hero_camera_anchor
        );
    }

    #[test]
    fn la_escala_espaciosa_se_deriva_de_la_geometria_movida() {
        let espacioso = nivel_espacioso();

        // `scene_radius` se mide sobre la escena nueva, no se hereda.
        let medido = measure_scene_radius(&espacioso.scene, espacioso.anchors.orbit_center);

        assert_eq!(
            espacioso.scale.scene_radius, medido,
            "scene_radius no corresponde con la geometria redistribuida"
        );

        // Y `orbit_radius` sale de la funcion real, no de un numero a mano.
        assert_eq!(
            espacioso.scale.orbit_radius,
            derive_orbit_radius(
                espacioso.scale.scene_radius,
                espacioso.scale.monolith_height
            ),
            "orbit_radius no corresponde con scene_radius y monolith_height"
        );
    }

    #[test]
    fn el_territorio_espacioso_es_mayor_que_el_actual() {
        let actual = nivel();
        let espacioso = nivel_espacioso();

        assert!(
            espacioso.scale.scene_radius > actual.scale.scene_radius,
            "el radio medido paso de {:.4} a {:.4}: el territorio no crecio",
            actual.scale.scene_radius,
            espacioso.scale.scene_radius
        );

        // Un territorio mayor aleja la camara: el encuadre tiene que
        // contener mas escena con el mismo campo de vision.
        assert!(
            espacioso.scale.orbit_radius > actual.scale.orbit_radius,
            "el radio orbital no crecio con el territorio"
        );
    }

    // ------------------------------------------------------------------
    // La cadena reconstruida: `connected`
    // ------------------------------------------------------------------

    /// Las diez masas del arco costero `G-02`.
    const MASAS_G02: usize = 10;

    /// Cuántas de esas diez entran en la cadena de terrazas.
    ///
    /// El número está escrito para que la regla no pueda cambiar en
    /// silencio: la selección se calcula por cajas, pero si un ajuste la
    /// convierte en «ninguna» o en «las diez» —que es justo lo que la
    /// revisión rechazó— este test lo dice.
    const TERRAZAS: usize = 4;

    fn solapan(a: (f32, f32), b: (f32, f32)) -> f32 {
        a.1.min(b.1) - a.0.max(b.0)
    }

    #[test]
    fn el_nivel_conectado_conserva_el_conteo_del_nivel() {
        let conectado = nivel_conectado();

        assert_eq!(
            conectado.scene.objects.len(),
            OBJETOS,
            "la cadena reconstruida cambio el conteo del nivel"
        );
    }

    #[test]
    fn el_plinto_conectado_mide_veintidos_por_diecinueve() {
        let candidato = nivel_candidato();
        let conectado = nivel_conectado();

        let indices = indices_del_grupo(&conectado, SpatialGroupId::Global);
        assert_eq!(indices.len(), 1, "G-01 deberia ser la unica pieza Global");

        let i = indices[0];
        let antes = candidato.scene.objects[i].primitive.bounds();
        let ahora = conectado.scene.objects[i].primitive.bounds();
        let tamano = tamano_de(&ahora);

        assert!(
            (tamano.x - PLINTO_X).abs() < EPS,
            "el plinto mide {:.4} de ancho y el ensayo pide {PLINTO_X}",
            tamano.x
        );
        assert!(
            (tamano.z - PLINTO_Z).abs() < EPS,
            "el plinto mide {:.4} de fondo y el ensayo pide {PLINTO_Z}",
            tamano.z
        );
        assert!(
            (tamano.y - tamano_de(&antes).y).abs() < EPS,
            "el plinto cambio de espesor"
        );
        assert!(
            (centro_de(&ahora) - centro_de(&antes)).magnitude() < EPS,
            "el plinto se movio de centro"
        );
    }

    #[test]
    fn el_monolito_sigue_exacto_en_el_conectado() {
        let candidato = nivel_candidato();
        let conectado = nivel_conectado();

        let indices = indices_del_grupo(&conectado, SpatialGroupId::Monolith);
        assert!(!indices.is_empty(), "no hay Monolito que comprobar");

        for &i in &indices {
            assert_eq!(
                candidato.scene.objects[i].primitive.bounds(),
                conectado.scene.objects[i].primitive.bounds(),
                "la pieza {i} del Monolito se movio: deja de servir de ancla"
            );
        }

        assert_eq!(
            candidato.scale.monolith_height, conectado.scale.monolith_height,
            "el Monolito cambio de altura"
        );
    }

    #[test]
    fn aguas_se_abre_al_frente_sin_deformarse() {
        let candidato = nivel_candidato();
        let conectado = nivel_conectado();

        let delta = desplazamiento_aguas();

        // Al frente es `+Z`: la toma hero mira desde `+Z` hacia el origen.
        assert!(delta.z > 0.0, "Aguas no se abrio al frente");
        assert_eq!(delta.x, 0.0, "Aguas se movio de lado");
        assert_eq!(delta.y, 0.0, "Aguas cambio de altura");

        for &i in &indices_del_grupo(&conectado, SpatialGroupId::FlyingWaters) {
            let antes = candidato.scene.objects[i].primitive.bounds();
            let ahora = conectado.scene.objects[i].primitive.bounds();

            assert!(
                (tamano_de(&ahora) - tamano_de(&antes)).magnitude() < EPS,
                "la pieza {i} de Aguas cambio de tamano"
            );
            assert!(
                (centro_de(&ahora) - centro_de(&antes) - delta).magnitude() < EPS,
                "la pieza {i} de Aguas no siguio el desplazamiento de la region"
            );
        }
    }

    #[test]
    fn cada_masa_de_g02_se_queda_o_se_traslada_segun_la_lista() {
        let candidato = nivel_candidato();
        let conectado = nivel_conectado();

        let todas = indices_del_grupo(&conectado, SpatialGroupId::ContinentBackground);
        assert_eq!(todas.len(), MASAS_G02, "G-02 no tiene diez masas");

        let terrazas = masas_de_terraza(&candidato.scene);

        // Ni ninguna ni todas: la revision rechazo las dos.
        assert_eq!(
            terrazas.len(),
            TERRAZAS,
            "la regla de terrazas eligio {} masas de {MASAS_G02}",
            terrazas.len()
        );
        for &i in &terrazas {
            assert!(
                todas.contains(&i),
                "la lista de terrazas nombra el objeto {i}, que no es de G-02"
            );
        }

        let delta = desplazamiento_terrazas_conectado();

        for &i in &todas {
            let antes = candidato.scene.objects[i].primitive.bounds();
            let ahora = conectado.scene.objects[i].primitive.bounds();

            if terrazas.contains(&i) {
                assert!(
                    (tamano_de(&ahora) - tamano_de(&antes)).magnitude() < EPS,
                    "la masa {i} de G-02 cambio de tamano"
                );
                assert!(
                    (centro_de(&ahora) - centro_de(&antes) - delta).magnitude() < EPS,
                    "la masa {i} de G-02 esta en la lista y no siguio el desplazamiento"
                );
            } else {
                assert_eq!(
                    antes, ahora,
                    "la masa {i} de G-02 no esta en la lista y aun asi se movio"
                );
            }
        }
    }

    #[test]
    fn praderas_y_rompeolas_se_trasladan_puras() {
        let candidato = nivel_candidato();
        let conectado = nivel_conectado();

        for (grupo, piezas, delta) in [
            (
                SpatialGroupId::Meadows,
                PRADERAS,
                desplazamiento_praderas_conectado(),
            ),
            (
                SpatialGroupId::Breakwater,
                ROMPEOLAS,
                desplazamiento_rompeolas_conectado(),
            ),
        ] {
            let indices = indices_del_grupo(&conectado, grupo);
            assert_eq!(indices.len(), piezas, "{grupo:?} no tiene {piezas} piezas");

            for &i in &indices {
                let antes = candidato.scene.objects[i].primitive.bounds();
                let ahora = conectado.scene.objects[i].primitive.bounds();

                assert!(
                    (tamano_de(&ahora) - tamano_de(&antes)).magnitude() < EPS,
                    "la pieza {i} de {grupo:?} cambio de tamano"
                );
                assert!(
                    (centro_de(&ahora) - centro_de(&antes) - delta).magnitude() < EPS,
                    "la pieza {i} de {grupo:?} no siguio el desplazamiento de su region"
                );
            }
        }
    }

    #[test]
    fn ninguna_pieza_salvo_el_plinto_cambia_de_forma_ni_de_grupo() {
        let candidato = nivel_candidato();
        let conectado = nivel_conectado();

        let plinto = indices_del_grupo(&conectado, SpatialGroupId::Global);

        for i in 0..OBJETOS {
            if plinto.contains(&i) {
                continue;
            }

            let a = &candidato.scene.objects[i];
            let b = &conectado.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert!(
                (tamano_de(&b.primitive.bounds()) - tamano_de(&a.primitive.bounds())).magnitude()
                    < EPS,
                "la pieza {i} cambio de tamano"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");
        }
    }

    #[test]
    fn la_cresta_del_rompeolas_apoya_bajo_praderas() {
        // Lo que la revision echo en falta: la cara alta del Rompeolas
        // tiene que quedar **debajo** de la meseta, no a un lado.
        let conectado = nivel_conectado();

        let cresta = cresta_del_rompeolas(&conectado.scene);
        let praderas = huella_del_grupo(&conectado.scene, SpatialGroupId::Meadows);
        let centro = centro_de(&cresta);

        assert!(
            centro.x > praderas.min.x && centro.x < praderas.max.x,
            "la cresta cae en x = {:.2} y Praderas ocupa [{:.2}, {:.2}]",
            centro.x,
            praderas.min.x,
            praderas.max.x
        );
        assert!(
            centro.z > praderas.min.z && centro.z < praderas.max.z,
            "la cresta cae en z = {:.2} y Praderas ocupa [{:.2}, {:.2}]",
            centro.z,
            praderas.min.z,
            praderas.max.z
        );
        assert!(
            cresta.max.y >= praderas.min.y,
            "la cresta llega a y = {:.2} y Praderas arranca en {:.2}: no la sostiene",
            cresta.max.y,
            praderas.min.y
        );
    }

    #[test]
    fn la_cadena_de_terrazas_no_se_rompe() {
        // Praderas -> Rompeolas -> terrazas de G-02 -> Aguas. Cada eslabon
        // tiene que solapar con el siguiente en `z` **y** en `y`: solapar
        // solo en planta dejaria dos mesetas flotando una sobre otra.
        let candidato = nivel_candidato();
        let conectado = nivel_conectado();

        let terrazas = huella(&conectado.scene, &masas_de_terraza(&candidato.scene));

        let cadena = [
            (
                "Praderas",
                huella_del_grupo(&conectado.scene, SpatialGroupId::Meadows),
            ),
            (
                "Rompeolas",
                huella_del_grupo(&conectado.scene, SpatialGroupId::Breakwater),
            ),
            ("terrazas G-02", terrazas),
            (
                "Aguas",
                huella_del_grupo(&conectado.scene, SpatialGroupId::FlyingWaters),
            ),
        ];

        for par in cadena.windows(2) {
            let (nombre_a, a) = &par[0];
            let (nombre_b, b) = &par[1];

            let en_z = solapan((a.min.z, a.max.z), (b.min.z, b.max.z));
            let en_y = solapan((a.min.y, a.max.y), (b.min.y, b.max.y));

            assert!(
                en_z > 0.0,
                "{nombre_a} y {nombre_b} no se tocan en z: {en_z:.2}"
            );
            assert!(
                en_y > 0.0,
                "{nombre_a} y {nombre_b} no se tocan en y: {en_y:.2}"
            );
        }
    }

    #[test]
    fn la_escala_conectada_se_deriva_de_la_geometria_movida() {
        let conectado = nivel_conectado();

        let medido = measure_scene_radius(&conectado.scene, conectado.anchors.orbit_center);
        assert_eq!(
            conectado.scale.scene_radius, medido,
            "scene_radius no corresponde con la geometria redistribuida"
        );

        assert_eq!(
            conectado.scale.orbit_radius,
            derive_orbit_radius(
                conectado.scale.scene_radius,
                conectado.scale.monolith_height
            ),
            "orbit_radius no corresponde con scene_radius y monolith_height"
        );

        // Y el ojo sale de la funcion real del proyecto, con el radio
        // derivado de esta escena.
        assert_eq!(
            conectado.anchors.hero_camera_anchor,
            eye_at_yaw(
                conectado.anchors.orbit_center,
                conectado.scale.orbit_radius,
                HERO_YAW_DEGREES
            ),
            "la toma hero no se derivo de la escala conectada"
        );

        // Las anclas que sí se movieron acompañan a su geometría.
        let candidato = nivel_candidato();
        assert_eq!(
            conectado.anchors.meadows_anchor,
            candidato.anchors.meadows_anchor + desplazamiento_praderas_conectado()
        );
        assert_eq!(
            conectado.anchors.breakwater_anchor,
            candidato.anchors.breakwater_anchor + desplazamiento_rompeolas_conectado()
        );
        assert_eq!(
            conectado.anchors.flying_waters_anchor,
            candidato.anchors.flying_waters_anchor + desplazamiento_aguas()
        );
        assert_eq!(
            conectado.anchors.boat_anchor,
            candidato.anchors.boat_anchor + desplazamiento_aguas()
        );

        // Y las que no, no.
        assert_eq!(
            conectado.anchors.orbit_center,
            candidato.anchors.orbit_center
        );
        assert_eq!(
            conectado.anchors.monolith_base_anchor,
            candidato.anchors.monolith_base_anchor
        );
    }

    #[test]
    fn el_nivel_conectado_es_determinista() {
        let uno = nivel_conectado();
        let dos = nivel_conectado();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
        assert_eq!(
            uno.anchors.hero_camera_anchor,
            dos.anchors.hero_camera_anchor
        );
    }

    #[test]
    fn los_niveles_previos_no_cambian_al_construir_el_conectado() {
        // `current`, `candidate` y `spacious` tienen que salir iguales que
        // antes de que existiera el cuarto nivel. Se comprueba generando y
        // contrastando: ningun hash del disco entra en un test.
        let previos = [nivel(), nivel_candidato(), nivel_espacioso()];

        let _conectado = nivel_conectado();

        let otra_vez = [nivel(), nivel_candidato(), nivel_espacioso()];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            assert_eq!(antes.scene.objects.len(), ahora.scene.objects.len());

            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
            assert_eq!(
                antes.anchors.hero_camera_anchor,
                ahora.anchors.hero_camera_anchor
            );
        }

        // Y el conectado no es el espacioso: si lo fuera, este test seria
        // una comprobacion vacia.
        let espacioso = nivel_espacioso();
        let conectado = nivel_conectado();
        let g02 = indices_del_grupo(&conectado, SpatialGroupId::ContinentBackground);

        assert!(
            g02.iter()
                .any(|&i| espacioso.scene.objects[i].primitive.bounds()
                    != conectado.scene.objects[i].primitive.bounds()),
            "el nivel conectado no toco ninguna masa de G-02: no arregla lo que la revision senalo"
        );
    }

    // ------------------------------------------------------------------
    // El labio costero: `refined`
    // ------------------------------------------------------------------

    /// Los ocho bloques de `A-11`, el borde roto.
    const LABIO: usize = 8;

    /// Margen mínimo entre Aguas Voladoras y el borde del plinto.
    const MARGEN_PLINTO: f32 = 0.10;

    /// Escalón mínimo entre las dos cotas del labio costero.
    ///
    /// Con los bloques entre `2.2` y `3.2` de alto, un escalón por debajo de
    /// esto no sería un escalón: sería ruido de la propia fila.
    const UMBRAL_COTA: f32 = 0.15;

    #[test]
    fn el_nivel_refinado_conserva_el_conteo_del_nivel() {
        let refinado = nivel_refinado();

        assert_eq!(
            refinado.scene.objects.len(),
            OBJETOS,
            "el labio costero cambio el conteo del nivel"
        );
    }

    #[test]
    fn el_plinto_refinado_mide_veintidos_por_diecinueve() {
        let conectado = nivel_conectado();
        let refinado = nivel_refinado();

        let indices = indices_del_grupo(&refinado, SpatialGroupId::Global);
        assert_eq!(indices.len(), 1, "G-01 deberia ser la unica pieza Global");

        assert_eq!(
            conectado.scene.objects[indices[0]].primitive.bounds(),
            refinado.scene.objects[indices[0]].primitive.bounds(),
            "el plinto cambio: refined no toca el suelo"
        );

        let tamano = tamano_de(&refinado.scene.objects[indices[0]].primitive.bounds());

        assert!(
            (tamano.x - PLINTO_X).abs() < EPS,
            "el plinto no mide 22 de ancho"
        );
        assert!(
            (tamano.z - PLINTO_Z).abs() < EPS,
            "el plinto no mide 19 de fondo"
        );
    }

    #[test]
    fn el_monolito_sigue_exacto_en_el_refinado() {
        let conectado = nivel_conectado();
        let refinado = nivel_refinado();

        for &i in &indices_del_grupo(&refinado, SpatialGroupId::Monolith) {
            assert_eq!(
                conectado.scene.objects[i].primitive.bounds(),
                refinado.scene.objects[i].primitive.bounds(),
                "la pieza {i} del Monolito se movio"
            );
        }

        assert_eq!(
            conectado.scale.monolith_height,
            refinado.scale.monolith_height
        );
    }

    #[test]
    fn el_volumen_de_agua_sigue_siendo_uno_y_sin_deformar() {
        let conectado = nivel_conectado();
        let refinado = nivel_refinado();

        let a01 = volumen_de_agua(&refinado.scene);

        // Sigue siendo **el mismo objeto**, ni partido ni reemplazado.
        assert_eq!(
            volumen_de_agua(&conectado.scene),
            a01,
            "A-01 cambio de indice"
        );
        assert_eq!(
            conectado.scene.objects[a01].primitive.bounds(),
            refinado.scene.objects[a01].primitive.bounds(),
            "A-01 se movio o se deformo"
        );

        // Y no hay un segundo volumen: nadie duplico la lamina de agua para
        // que se viera mas. Se mide por caja: ninguna otra pieza de Aguas
        // se le acerca en volumen.
        let caja = refinado.scene.objects[a01].primitive.bounds();
        let volumen = |c: &Aabb| {
            let t = tamano_de(c);
            t.x * t.y * t.z
        };
        let suyo = volumen(&caja);

        for &i in &indices_del_grupo(&refinado, SpatialGroupId::FlyingWaters) {
            if i == a01 {
                continue;
            }

            let otro = volumen(&refinado.scene.objects[i].primitive.bounds());

            assert!(
                otro < suyo * 0.5,
                "la pieza {i} de Aguas mide {otro:.2} contra los {suyo:.2} de A-01: \
                 hay dos volumenes de agua donde deberia haber uno"
            );
        }
    }

    #[test]
    fn solo_el_labio_costero_se_mueve_y_solo_por_traslacion() {
        let conectado = nivel_conectado();
        let refinado = nivel_refinado();

        let labio = labio_costero(&conectado.scene);
        assert_eq!(labio.len(), LABIO, "A-11 no tiene ocho bloques");

        // Los ocho comparten fondo y base: es lo que los identifica como
        // una fila, y lo que permite localizarlos sin escribir indices.
        let primero = conectado.scene.objects[labio[0]].primitive.bounds();
        for &i in &labio {
            let caja = conectado.scene.objects[i].primitive.bounds();

            assert!(
                (tamano_de(&caja).z - tamano_de(&primero).z).abs() < EPS,
                "el bloque {i} de A-11 no comparte fondo con la fila"
            );
            assert!(
                (caja.min.y - primero.min.y).abs() < EPS,
                "el bloque {i} de A-11 no arranca a la misma altura"
            );
        }

        let mut movidos = 0usize;

        for i in 0..OBJETOS {
            let a = &conectado.scene.objects[i];
            let b = &refinado.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            let antes = a.primitive.bounds();
            let ahora = b.primitive.bounds();

            assert!(
                (tamano_de(&ahora) - tamano_de(&antes)).magnitude() < EPS,
                "la pieza {i} cambio de tamano: refined solo traslada"
            );

            if antes != ahora {
                assert!(
                    labio.contains(&i),
                    "la pieza {i} se movio y no es del labio costero"
                );
                movidos += 1;
            }
        }

        assert_eq!(
            movidos, LABIO,
            "se movieron {movidos} piezas y el labio tiene {LABIO}"
        );
    }

    #[test]
    fn el_labio_costero_tiene_dos_cotas() {
        // La revision vio un muro negro continuo. Dos cotas quiere decir
        // dos niveles de techo que no se mezclan, el bajo ocupando el centro
        // de la fila —por donde se mira la bahia— y el alto los flancos.
        let refinado = nivel_refinado();
        let (exteriores, centrales) = cotas_del_labio(&refinado.scene);

        assert_eq!(exteriores.len(), LABIO / 2, "la cota alta no tiene cuatro");
        assert_eq!(centrales.len(), LABIO / 2, "la cota baja no tiene cuatro");

        let techos = |ids: &[usize]| -> (f32, f32) {
            let mut bajo = f32::MAX;
            let mut alto = f32::MIN;

            for &i in ids {
                let y = refinado.scene.objects[i].primitive.bounds().max.y;
                bajo = bajo.min(y);
                alto = alto.max(y);
            }

            (bajo, alto)
        };

        let (bajo_exterior, alto_exterior) = techos(&exteriores);
        let (_, alto_central) = techos(&centrales);

        // Las dos cotas no se mezclan: el bloque mas bajo del flanco sigue
        // por encima del mas alto del centro.
        assert!(
            bajo_exterior > alto_central,
            "el flanco mas bajo llega a {bajo_exterior:.2} y el centro mas alto \
             a {alto_central:.2}: los dos niveles se confunden"
        );

        // Y el escalon se mide: no vale un milimetro de diferencia.
        assert!(
            alto_exterior - alto_central >= UMBRAL_COTA,
            "el escalon mide {:.3} y el minimo es {UMBRAL_COTA}",
            alto_exterior - alto_central
        );

        // El centro ademas retrocede: la fila tiene relieve en z, no es un
        // muro plano con muescas.
        let z_medio = |ids: &[usize]| -> f32 {
            let caja = huella(&refinado.scene, ids);

            (caja.min.z + caja.max.z) * 0.5
        };

        assert!(
            z_medio(&centrales) < z_medio(&exteriores) - EPS,
            "la cota baja no retrocedio: {:.3} contra {:.3}",
            z_medio(&centrales),
            z_medio(&exteriores)
        );

        // Y la cota baja esta **entre** las altas: hay flanco a los dos
        // lados, que es lo que convierte el centro en una apertura y no en
        // un extremo desnudo.
        let caja_central = huella(&refinado.scene, &centrales);
        let centro_x = |i: usize| {
            let caja = refinado.scene.objects[i].primitive.bounds();

            (caja.min.x + caja.max.x) * 0.5
        };

        assert!(
            exteriores.iter().any(|&i| centro_x(i) < caja_central.min.x),
            "no hay flanco alto a la izquierda del centro"
        );
        assert!(
            exteriores.iter().any(|&i| centro_x(i) > caja_central.max.x),
            "no hay flanco alto a la derecha del centro"
        );
    }

    #[test]
    fn aguas_se_queda_dentro_del_plinto_con_margen() {
        let refinado = nivel_refinado();

        let aguas = huella_del_grupo(&refinado.scene, SpatialGroupId::FlyingWaters);
        let plinto = huella_del_grupo(&refinado.scene, SpatialGroupId::Global);

        for (nombre, holgura) in [
            ("-x", aguas.min.x - plinto.min.x),
            ("+x", plinto.max.x - aguas.max.x),
            ("-z", aguas.min.z - plinto.min.z),
            ("+z", plinto.max.z - aguas.max.z),
        ] {
            assert!(
                holgura >= MARGEN_PLINTO,
                "Aguas deja {holgura:.4} de margen en {nombre} y el minimo es {MARGEN_PLINTO}"
            );
        }
    }

    #[test]
    fn la_bahia_muestra_una_franja_de_agua_visible_y_conectada() {
        // Se mide con el **mismo rayo primario** que lanzaria el renderer,
        // por `Camera::ray_from_pixel` y `SceneAccel::intersect`. Una celda
        // cuenta solo si lo primero que toca es la cara de arriba de
        // `A-01`: ver la cara frontal del volumen no es ver agua, es ver
        // una losa oscura de canto, que es justo lo que la revision leyo
        // como cuenca.
        let conectado = nivel_conectado();
        let refinado = nivel_refinado();

        let antes = vista_del_agua(&conectado, &conectado.hero_camera());
        let ahora = vista_del_agua(&refinado, &refinado.hero_camera());

        assert!(
            ahora.visibles > 0,
            "no se ve nada de la superficie de agua desde la toma hero"
        );

        // Conectada: la mayor mancha se lleva la mitad de lo visible. Si el
        // agua saliera en astillas repartidas entre los bloques, esto no se
        // cumpliria.
        assert!(
            ahora.mayor_mancha * 2 >= ahora.visibles,
            "la mayor mancha de agua tiene {} celdas de {} visibles: sale fragmentada",
            ahora.mayor_mancha,
            ahora.visibles
        );

        // Y mejora lo que habia: las dos comparten resolucion de muestreo y
        // encuadre derivado, asi que la cifra es comparable.
        assert!(
            ahora.mayor_mancha > antes.mayor_mancha,
            "la franja de agua paso de {} a {} celdas: refined no abrio la bahia",
            antes.mayor_mancha,
            ahora.mayor_mancha
        );
    }

    #[test]
    fn la_cadena_de_terrazas_sigue_unida_en_el_refinado() {
        let candidato = nivel_candidato();
        let refinado = nivel_refinado();

        let terrazas = huella(&refinado.scene, &masas_de_terraza(&candidato.scene));

        let cadena = [
            (
                "Praderas",
                huella_del_grupo(&refinado.scene, SpatialGroupId::Meadows),
            ),
            (
                "Rompeolas",
                huella_del_grupo(&refinado.scene, SpatialGroupId::Breakwater),
            ),
            ("terrazas G-02", terrazas),
            (
                "Aguas",
                huella_del_grupo(&refinado.scene, SpatialGroupId::FlyingWaters),
            ),
        ];

        for par in cadena.windows(2) {
            let (nombre_a, a) = &par[0];
            let (nombre_b, b) = &par[1];

            assert!(
                solapan((a.min.z, a.max.z), (b.min.z, b.max.z)) > 0.0,
                "{nombre_a} y {nombre_b} no se tocan en z"
            );
            assert!(
                solapan((a.min.y, a.max.y), (b.min.y, b.max.y)) > 0.0,
                "{nombre_a} y {nombre_b} no se tocan en y"
            );
        }
    }

    #[test]
    fn la_escala_y_las_luces_refinadas_se_derivan_de_la_escena() {
        let candidato = nivel_candidato();
        let conectado = nivel_conectado();
        let refinado = nivel_refinado();

        assert_eq!(
            refinado.scale.scene_radius,
            measure_scene_radius(&refinado.scene, refinado.anchors.orbit_center),
            "scene_radius no se volvio a medir sobre la geometria refinada"
        );
        assert_eq!(
            refinado.scale.orbit_radius,
            derive_orbit_radius(refinado.scale.scene_radius, refinado.scale.monolith_height),
            "orbit_radius no corresponde con scene_radius y monolith_height"
        );
        assert_eq!(
            refinado.anchors.hero_camera_anchor,
            eye_at_yaw(
                refinado.anchors.orbit_center,
                refinado.scale.orbit_radius,
                HERO_YAW_DEGREES
            ),
            "la toma hero no se derivo de la escala refinada"
        );

        // Las luces se arman contra las anclas y la escala de **esta**
        // escena. Ninguna ancla de region se movio respecto de `connected`
        // —el labio se reordena dentro de Aguas—, asi que el rig tiene que
        // salir igual que el de `connected` y distinto del de `candidate`,
        // que todavia tiene la bahia sin abrir.
        let posiciones = |d: &Blockout| -> Vec<Vec3> {
            luces_del_diorama(&d.anchors, &d.scale)
                .iter()
                .map(|luz| luz.position)
                .collect()
        };

        assert!(
            !posiciones(&refinado).is_empty(),
            "el rig se quedo sin luces"
        );
        assert_eq!(
            posiciones(&refinado),
            posiciones(&conectado),
            "el rig cambio sin que se moviera ninguna ancla"
        );
        assert_ne!(
            posiciones(&refinado),
            posiciones(&candidato),
            "el rig no siguio a la bahia cuando se abrio al frente"
        );
    }

    #[test]
    fn el_nivel_refinado_es_determinista() {
        let uno = nivel_refinado();
        let dos = nivel_refinado();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
        assert_eq!(
            uno.anchors.hero_camera_anchor,
            dos.anchors.hero_camera_anchor
        );
    }

    #[test]
    fn los_cuatro_niveles_previos_no_cambian_al_construir_el_refinado() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
        ];

        let _refinado = nivel_refinado();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
            assert_eq!(
                antes.anchors.hero_camera_anchor,
                ahora.anchors.hero_camera_anchor
            );
        }
    }

    // ------------------------------------------------------------------
    // La boca asimétrica: `asymmetric`
    // ------------------------------------------------------------------

    /// Anchura mínima del canal. Por debajo de esto la boca deja de ser una
    /// boca y pasa a ser una junta entre dos bloques.
    const UMBRAL_CANAL: f32 = 1.00;

    /// Cuánto tiene que apartarse del eje el centro de la boca para que la
    /// asimetría se lea y no sea el ruido de dos anchuras distintas.
    const UMBRAL_ASIMETRIA: f32 = 0.60;

    /// Cuánto más retranqueado tiene que estar el flanco largo.
    const UMBRAL_RETRANQUEO: f32 = 0.30;

    /// Cuánto más largo tiene que ser el flanco largo.
    const UMBRAL_LARGO: f32 = 1.00;

    #[test]
    fn el_nivel_asimetrico_conserva_el_conteo_del_nivel() {
        let asimetrico = nivel_asimetrico();

        assert_eq!(
            asimetrico.scene.objects.len(),
            OBJETOS,
            "la boca asimetrica cambio el conteo del nivel"
        );
    }

    #[test]
    fn el_monolito_y_el_volumen_siguen_exactos_en_el_asimetrico() {
        let refinado = nivel_refinado();
        let asimetrico = nivel_asimetrico();

        for &i in &indices_del_grupo(&asimetrico, SpatialGroupId::Monolith) {
            assert_eq!(
                refinado.scene.objects[i].primitive.bounds(),
                asimetrico.scene.objects[i].primitive.bounds(),
                "la pieza {i} del Monolito se movio"
            );
        }

        let a01 = volumen_de_agua(&asimetrico.scene);

        assert_eq!(
            volumen_de_agua(&refinado.scene),
            a01,
            "A-01 cambio de indice"
        );
        assert_eq!(
            refinado.scene.objects[a01].primitive.bounds(),
            asimetrico.scene.objects[a01].primitive.bounds(),
            "A-01 se movio o se deformo"
        );

        // El plinto tampoco: la base sigue en 22 x 19.
        let plinto = indices_del_grupo(&asimetrico, SpatialGroupId::Global);
        let caja = asimetrico.scene.objects[plinto[0]].primitive.bounds();

        assert!((tamano_de(&caja).x - PLINTO_X).abs() < EPS);
        assert!((tamano_de(&caja).z - PLINTO_Z).abs() < EPS);
    }

    #[test]
    fn solo_el_labio_se_mueve_en_el_asimetrico() {
        let refinado = nivel_refinado();
        let asimetrico = nivel_asimetrico();

        let labio = labio_costero(&refinado.scene);
        let mut movidos = 0usize;

        for i in 0..OBJETOS {
            let a = &refinado.scene.objects[i];
            let b = &asimetrico.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            let antes = a.primitive.bounds();
            let ahora = b.primitive.bounds();

            assert!(
                (tamano_de(&ahora) - tamano_de(&antes)).magnitude() < EPS,
                "la pieza {i} cambio de tamano: asymmetric solo traslada"
            );

            if antes != ahora {
                assert!(
                    labio.contains(&i),
                    "la pieza {i} se movio y no es del labio costero"
                );
                movidos += 1;
            }
        }

        assert!(
            movidos > 0 && movidos <= LABIO,
            "se movieron {movidos} piezas y el labio tiene {LABIO}"
        );
    }

    #[test]
    fn la_boca_costera_es_asimetrica() {
        // `refined` dejo la boca centrada en el eje de la toma hero, con
        // cuatro bloques a cada lado. Aqui se rompe: un bloque cruza la
        // boca, el flanco largo se alarga y ademas retrocede.
        let refinado = nivel_refinado();
        let asimetrico = nivel_asimetrico();

        let boca_simetrica = boca_costera(&refinado.scene);
        let boca = boca_costera(&asimetrico.scene);

        assert_eq!(
            (boca_simetrica.izquierda.len(), boca_simetrica.derecha.len()),
            (LABIO / 2, LABIO / 2),
            "la boca de refined ya no era simetrica de partida"
        );
        assert_ne!(
            boca.izquierda.len(),
            boca.derecha.len(),
            "los dos flancos siguen teniendo el mismo numero de bloques"
        );

        let eje = |scene: &Scene| {
            let fila = huella(scene, &labio_costero(scene));

            (fila.min.x + fila.max.x) * 0.5
        };

        assert!(
            (boca_simetrica.centro() - eje(&refinado.scene)).abs() < UMBRAL_ASIMETRIA,
            "la boca de refined ya estaba descentrada {:.3}",
            (boca_simetrica.centro() - eje(&refinado.scene)).abs()
        );
        assert!(
            (boca.centro() - eje(&asimetrico.scene)).abs() >= UMBRAL_ASIMETRIA,
            "la boca se quedo a {:.3} del eje y el minimo es {UMBRAL_ASIMETRIA}",
            (boca.centro() - eje(&asimetrico.scene)).abs()
        );

        // Un flanco inequivocamente mas largo que el otro.
        let largo = |ids: &[usize]| {
            let caja = huella(&asimetrico.scene, ids);

            caja.max.x - caja.min.x
        };
        let (corto, extenso) = if boca.izquierda.len() < boca.derecha.len() {
            (&boca.izquierda, &boca.derecha)
        } else {
            (&boca.derecha, &boca.izquierda)
        };

        assert!(
            largo(extenso) >= largo(corto) + UMBRAL_LARGO,
            "el flanco largo mide {:.2} y el corto {:.2}",
            largo(extenso),
            largo(corto)
        );

        // Y ademas retranqueado: mas cerca de la bahia que el corto.
        let z_medio = |ids: &[usize]| {
            let caja = huella(&asimetrico.scene, ids);

            (caja.min.z + caja.max.z) * 0.5
        };

        assert!(
            z_medio(extenso) <= z_medio(corto) - UMBRAL_RETRANQUEO,
            "el flanco largo esta en z {:.2} y el corto en {:.2}: no retrocedio",
            z_medio(extenso),
            z_medio(corto)
        );
    }

    #[test]
    fn el_canal_sigue_abierto_en_el_asimetrico() {
        let refinado = nivel_refinado();
        let asimetrico = nivel_asimetrico();

        let boca = boca_costera(&asimetrico.scene);

        assert!(
            boca.ancho() >= UMBRAL_CANAL,
            "el canal mide {:.2} y el minimo es {UMBRAL_CANAL}",
            boca.ancho()
        );

        let antes = vista_del_agua(&refinado, &refinado.hero_camera());
        let ahora = vista_del_agua(&asimetrico, &asimetrico.hero_camera());

        assert!(
            ahora.visibles > 0,
            "no se ve lamina de agua desde la toma hero"
        );
        assert!(
            ahora.mayor_mancha * 2 >= ahora.visibles,
            "la lamina sale fragmentada: {} celdas de {} visibles",
            ahora.mayor_mancha,
            ahora.visibles
        );

        // Romper la simetria no puede costar el canal. Se admite que la
        // cifra baje un poco —los bloques cambian de sitio— pero no que se
        // hunda: el agua visible sigue siendo la de `refined`.
        assert!(
            ahora.mayor_mancha * 10 >= antes.mayor_mancha * 9,
            "la franja de agua paso de {} a {} celdas: la boca se cerro",
            antes.mayor_mancha,
            ahora.mayor_mancha
        );
    }

    #[test]
    fn aguas_se_queda_dentro_del_plinto_en_el_asimetrico() {
        let asimetrico = nivel_asimetrico();

        let aguas = huella_del_grupo(&asimetrico.scene, SpatialGroupId::FlyingWaters);
        let plinto = huella_del_grupo(&asimetrico.scene, SpatialGroupId::Global);

        for (nombre, holgura) in [
            ("-x", aguas.min.x - plinto.min.x),
            ("+x", plinto.max.x - aguas.max.x),
            ("-z", aguas.min.z - plinto.min.z),
            ("+z", plinto.max.z - aguas.max.z),
        ] {
            assert!(
                holgura >= MARGEN_PLINTO,
                "Aguas deja {holgura:.4} de margen en {nombre} y el minimo es {MARGEN_PLINTO}"
            );
        }
    }

    #[test]
    fn la_cadena_de_terrazas_sigue_unida_en_el_asimetrico() {
        let candidato = nivel_candidato();
        let asimetrico = nivel_asimetrico();

        let terrazas = huella(&asimetrico.scene, &masas_de_terraza(&candidato.scene));

        let cadena = [
            (
                "Praderas",
                huella_del_grupo(&asimetrico.scene, SpatialGroupId::Meadows),
            ),
            (
                "Rompeolas",
                huella_del_grupo(&asimetrico.scene, SpatialGroupId::Breakwater),
            ),
            ("terrazas G-02", terrazas),
            (
                "Aguas",
                huella_del_grupo(&asimetrico.scene, SpatialGroupId::FlyingWaters),
            ),
        ];

        for par in cadena.windows(2) {
            let (nombre_a, a) = &par[0];
            let (nombre_b, b) = &par[1];

            assert!(
                solapan((a.min.z, a.max.z), (b.min.z, b.max.z)) > 0.0,
                "{nombre_a} y {nombre_b} no se tocan en z"
            );
            assert!(
                solapan((a.min.y, a.max.y), (b.min.y, b.max.y)) > 0.0,
                "{nombre_a} y {nombre_b} no se tocan en y"
            );
        }
    }

    #[test]
    fn la_escala_asimetrica_se_deriva_de_la_escena() {
        let asimetrico = nivel_asimetrico();

        assert_eq!(
            asimetrico.scale.scene_radius,
            measure_scene_radius(&asimetrico.scene, asimetrico.anchors.orbit_center),
            "scene_radius no se volvio a medir"
        );
        assert_eq!(
            asimetrico.scale.orbit_radius,
            derive_orbit_radius(
                asimetrico.scale.scene_radius,
                asimetrico.scale.monolith_height
            ),
            "orbit_radius no corresponde con scene_radius y monolith_height"
        );
        assert_eq!(
            asimetrico.anchors.hero_camera_anchor,
            eye_at_yaw(
                asimetrico.anchors.orbit_center,
                asimetrico.scale.orbit_radius,
                HERO_YAW_DEGREES
            ),
            "la toma hero no se derivo de la escala asimetrica"
        );
    }

    #[test]
    fn el_nivel_asimetrico_es_determinista() {
        let uno = nivel_asimetrico();
        let dos = nivel_asimetrico();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
    }

    #[test]
    fn los_cinco_niveles_previos_no_cambian_al_construir_el_asimetrico() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
        ];

        let _asimetrico = nivel_asimetrico();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
            assert_eq!(
                antes.anchors.hero_camera_anchor,
                ahora.anchors.hero_camera_anchor
            );
        }
    }

    // ------------------------------------------------------------------
    // Tres territorios: `separated`
    // ------------------------------------------------------------------

    /// Claro mínimo exigible entre dos territorios, en planta.
    ///
    /// La composición los coloca a `1.00` exacto en el caso más apretado;
    /// el umbral deja holgura de coma flotante sin admitir un contacto.
    const CLARO_MINIMO: f32 = 0.90;

    #[test]
    fn el_nivel_separado_conserva_el_conteo_del_nivel() {
        let separado = nivel_separado();

        assert_eq!(
            separado.scene.objects.len(),
            OBJETOS,
            "la composicion separada cambio el conteo del nivel"
        );
    }

    #[test]
    fn el_plinto_separado_mide_veintidos_por_diecinueve() {
        let candidato = nivel_candidato();
        let separado = nivel_separado();

        let indices = indices_del_grupo(&separado, SpatialGroupId::Global);
        assert_eq!(indices.len(), 1, "G-01 deberia ser la unica pieza Global");

        let antes = candidato.scene.objects[indices[0]].primitive.bounds();
        let ahora = separado.scene.objects[indices[0]].primitive.bounds();
        let tamano = tamano_de(&ahora);

        assert!(
            (tamano.x - PLINTO_X).abs() < EPS,
            "el plinto mide {:.4} de ancho",
            tamano.x
        );
        assert!(
            (tamano.z - PLINTO_Z).abs() < EPS,
            "el plinto mide {:.4} de fondo",
            tamano.z
        );
        assert!(
            (tamano.y - tamano_de(&antes).y).abs() < EPS,
            "el plinto cambio de espesor"
        );
        assert!(
            (centro_de(&ahora) - centro_de(&antes)).magnitude() < EPS,
            "el plinto se movio de centro"
        );
    }

    #[test]
    fn el_monolito_y_el_volumen_siguen_exactos_en_el_separado() {
        let candidato = nivel_candidato();
        let separado = nivel_separado();

        for &i in &indices_del_grupo(&separado, SpatialGroupId::Monolith) {
            assert_eq!(
                candidato.scene.objects[i].primitive.bounds(),
                separado.scene.objects[i].primitive.bounds(),
                "la pieza {i} del Monolito se movio"
            );
        }
        assert_eq!(
            candidato.scale.monolith_height,
            separado.scale.monolith_height
        );

        // `A-01` viaja con su region, pero sigue siendo **un solo volumen**
        // y sigue teniendo su caja: se traslada, no se parte ni se estira.
        let a01 = volumen_de_agua(&separado.scene);
        assert_eq!(
            volumen_de_agua(&candidato.scene),
            a01,
            "A-01 cambio de indice"
        );

        let antes = candidato.scene.objects[a01].primitive.bounds();
        let ahora = separado.scene.objects[a01].primitive.bounds();

        assert!(
            (tamano_de(&ahora) - tamano_de(&antes)).magnitude() < EPS,
            "A-01 se deformo"
        );

        let volumen = |c: &Aabb| {
            let t = tamano_de(c);
            t.x * t.y * t.z
        };
        let suyo = volumen(&ahora);

        for &i in &indices_del_grupo(&separado, SpatialGroupId::FlyingWaters) {
            if i == a01 {
                continue;
            }

            assert!(
                volumen(&separado.scene.objects[i].primitive.bounds()) < suyo * 0.5,
                "la pieza {i} de Aguas se acerca al volumen de A-01"
            );
        }
    }

    #[test]
    fn las_regiones_del_separado_solo_se_trasladan() {
        let candidato = nivel_candidato();
        let separado = nivel_separado();

        let reparto = reparto_separado(&candidato.scene);
        let plinto = indices_del_grupo(&separado, SpatialGroupId::Global);

        for i in 0..OBJETOS {
            if plinto.contains(&i) {
                continue;
            }

            let a = &candidato.scene.objects[i];
            let b = &separado.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            let antes = a.primitive.bounds();
            let ahora = b.primitive.bounds();

            assert!(
                (tamano_de(&ahora) - tamano_de(&antes)).magnitude() < EPS,
                "la pieza {i} cambio de tamano: separated solo traslada"
            );

            // Cada pieza se mueve el vector de **su** region, y las demas no
            // se mueven en absoluto: el fondo continental y el Monolito se
            // quedan donde estaban.
            let esperado = match a.spatial_group {
                SpatialGroupId::Meadows => reparto.praderas,
                SpatialGroupId::Breakwater => reparto.rompeolas,
                SpatialGroupId::FlyingWaters => reparto.aguas,
                _ => Vec3::zeros(),
            };

            assert!(
                (centro_de(&ahora) - centro_de(&antes) - esperado).magnitude() < EPS,
                "la pieza {i} de {:?} no siguio el reparto de su region",
                a.spatial_group
            );
        }
    }

    #[test]
    fn las_tres_regiones_no_solapan_en_planta() {
        // Lo que pidio la correccion: territorio propio. Ninguna pareja de
        // huellas se toca en `XZ`, y el claro se mide.
        let separado = nivel_separado();

        let huellas = [
            (
                "Praderas",
                huella_del_grupo(&separado.scene, SpatialGroupId::Meadows),
            ),
            (
                "Rompeolas",
                huella_del_grupo(&separado.scene, SpatialGroupId::Breakwater),
            ),
            (
                "Aguas",
                huella_del_grupo(&separado.scene, SpatialGroupId::FlyingWaters),
            ),
        ];

        for a in 0..huellas.len() {
            for b in a + 1..huellas.len() {
                let claro = separacion_xz(&huellas[a].1, &huellas[b].1);

                assert!(
                    claro >= CLARO_MINIMO,
                    "{} y {} dejan {claro:.3} de claro y el minimo es {CLARO_MINIMO}",
                    huellas[a].0,
                    huellas[b].0
                );
            }
        }
    }

    #[test]
    fn el_monolito_queda_libre_y_con_corredor_desde_praderas() {
        let separado = nivel_separado();

        let monolito = huella_del_grupo(&separado.scene, SpatialGroupId::Monolith);

        for (nombre, grupo) in [
            ("Praderas", SpatialGroupId::Meadows),
            ("Rompeolas", SpatialGroupId::Breakwater),
            ("Aguas", SpatialGroupId::FlyingWaters),
        ] {
            let claro = separacion_xz(&monolito, &huella_del_grupo(&separado.scene, grupo));

            assert!(
                claro >= CLARO_MINIMO,
                "{nombre} deja {claro:.3} de claro al Monolito"
            );
        }

        // El corredor es **recto**: Praderas y el Monolito comparten banda
        // en `x`, asi que entre el borde de la meseta y la base del
        // Monolito hay una calle libre y no una diagonal.
        let praderas = huella_del_grupo(&separado.scene, SpatialGroupId::Meadows);

        assert!(
            praderas.min.x <= monolito.min.x && praderas.max.x >= monolito.max.x,
            "Praderas ocupa x [{:.2}, {:.2}] y el Monolito [{:.2}, {:.2}]: no hay corredor recto",
            praderas.min.x,
            praderas.max.x,
            monolito.min.x,
            monolito.max.x
        );
        assert!(
            praderas.max.z < monolito.min.z,
            "Praderas no quedo por detras del Monolito"
        );
    }

    #[test]
    fn aguas_se_queda_dentro_del_plinto_en_el_separado() {
        let separado = nivel_separado();

        let aguas = huella_del_grupo(&separado.scene, SpatialGroupId::FlyingWaters);
        let plinto = huella_del_grupo(&separado.scene, SpatialGroupId::Global);

        for (nombre, holgura) in [
            ("-x", aguas.min.x - plinto.min.x),
            ("+x", plinto.max.x - aguas.max.x),
            ("-z", aguas.min.z - plinto.min.z),
            ("+z", plinto.max.z - aguas.max.z),
        ] {
            assert!(
                holgura >= MARGEN_PLINTO,
                "Aguas deja {holgura:.4} de margen en {nombre} y el minimo es {MARGEN_PLINTO}"
            );
        }

        // Y sigue al frente: la bahia es lo mas cercano a la camara.
        for grupo in [SpatialGroupId::Meadows, SpatialGroupId::Breakwater] {
            assert!(
                aguas.max.z > huella_del_grupo(&separado.scene, grupo).max.z,
                "Aguas dejo de ser la region mas adelantada"
            );
        }
    }

    #[test]
    fn la_escala_y_las_luces_separadas_se_derivan_de_la_escena() {
        let candidato = nivel_candidato();
        let separado = nivel_separado();

        assert_eq!(
            separado.scale.scene_radius,
            measure_scene_radius(&separado.scene, separado.anchors.orbit_center),
            "scene_radius no se volvio a medir"
        );
        assert_eq!(
            separado.scale.orbit_radius,
            derive_orbit_radius(separado.scale.scene_radius, separado.scale.monolith_height),
            "orbit_radius no corresponde con scene_radius y monolith_height"
        );
        assert_eq!(
            separado.anchors.hero_camera_anchor,
            eye_at_yaw(
                separado.anchors.orbit_center,
                separado.scale.orbit_radius,
                HERO_YAW_DEGREES
            ),
            "la toma hero no se derivo de la escala separada"
        );

        let reparto = reparto_separado(&candidato.scene);

        assert_eq!(
            separado.anchors.meadows_anchor,
            candidato.anchors.meadows_anchor + reparto.praderas
        );
        assert_eq!(
            separado.anchors.breakwater_anchor,
            candidato.anchors.breakwater_anchor + reparto.rompeolas
        );
        assert_eq!(
            separado.anchors.flying_waters_anchor,
            candidato.anchors.flying_waters_anchor + reparto.aguas
        );
        assert_eq!(
            separado.anchors.boat_anchor,
            candidato.anchors.boat_anchor + reparto.aguas
        );
        assert_eq!(
            separado.anchors.orbit_center,
            candidato.anchors.orbit_center
        );

        // Las luces salen de esas anclas y de esta escala, y la bahia se
        // movio mucho: el rig no puede ser el del candidato.
        let posiciones = |d: &Blockout| -> Vec<Vec3> {
            luces_del_diorama(&d.anchors, &d.scale)
                .iter()
                .map(|luz| luz.position)
                .collect()
        };

        assert!(
            !posiciones(&separado).is_empty(),
            "el rig se quedo sin luces"
        );
        assert_ne!(
            posiciones(&separado),
            posiciones(&candidato),
            "el rig no siguio a las regiones"
        );
    }

    #[test]
    fn el_nivel_separado_es_determinista() {
        let uno = nivel_separado();
        let dos = nivel_separado();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
        assert_eq!(
            uno.anchors.hero_camera_anchor,
            dos.anchors.hero_camera_anchor
        );
    }

    #[test]
    fn los_seis_niveles_previos_no_cambian_al_construir_el_separado() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
        ];

        let _separado = nivel_separado();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
            assert_eq!(
                antes.anchors.hero_camera_anchor,
                ahora.anchors.hero_camera_anchor
            );
        }
    }

    #[test]
    fn la_separacion_en_planta_distingue_solape_de_claro() {
        // El instrumento antes que la medida: dos cajas que se solapan dan
        // negativo, dos que se rozan dan cero y dos separadas dan el hueco.
        let caja = |x0: f32, z0: f32, x1: f32, z1: f32| {
            Aabb::new(Vec3::new(x0, 0.0, z0), Vec3::new(x1, 1.0, z1))
        };

        let base = caja(0.0, 0.0, 2.0, 2.0);

        assert!(separacion_xz(&base, &caja(1.0, 1.0, 3.0, 3.0)) < 0.0);
        assert!(separacion_xz(&base, &caja(2.0, 0.0, 4.0, 2.0)).abs() < EPS);
        assert!((separacion_xz(&base, &caja(5.0, 0.0, 7.0, 2.0)) - 3.0).abs() < EPS);
        assert!((separacion_xz(&base, &caja(0.0, -4.0, 2.0, -1.0)) - 1.0).abs() < EPS);
    }

    // ------------------------------------------------------------------
    // Apoyo vertical: `grounded`
    // ------------------------------------------------------------------

    /// Las veintiocho piezas de `R-01`.
    const PILARES: usize = 28;

    /// Cuánto se admite que una pieza se hunda en su pedestal.
    ///
    /// Empotrar un poco es lo que garantiza contacto sin caras coplanares.
    /// Pasado este límite ya no es apoyo, es una pieza enterrada.
    const EMPOTRADO_MAXIMO: f32 = 0.60;

    #[test]
    fn el_nivel_apoyado_conserva_el_conteo_del_nivel() {
        let apoyado = nivel_apoyado();

        assert_eq!(
            apoyado.scene.objects.len(),
            OBJETOS,
            "el apoyo vertical cambio el conteo del nivel"
        );
    }

    #[test]
    fn el_plinto_el_monolito_y_el_volumen_siguen_exactos_en_el_apoyado() {
        let separado = nivel_separado();
        let apoyado = nivel_apoyado();

        for grupo in [SpatialGroupId::Global, SpatialGroupId::Monolith] {
            for &i in &indices_del_grupo(&apoyado, grupo) {
                assert_eq!(
                    separado.scene.objects[i].primitive.bounds(),
                    apoyado.scene.objects[i].primitive.bounds(),
                    "la pieza {i} de {grupo:?} se movio"
                );
            }
        }

        let a01 = volumen_de_agua(&apoyado.scene);
        assert_eq!(
            volumen_de_agua(&separado.scene),
            a01,
            "A-01 cambio de indice"
        );
        assert_eq!(
            separado.scene.objects[a01].primitive.bounds(),
            apoyado.scene.objects[a01].primitive.bounds(),
            "A-01 se movio o se deformo"
        );
    }

    #[test]
    fn praderas_y_aguas_no_se_mueven_en_el_apoyado() {
        let separado = nivel_separado();
        let apoyado = nivel_apoyado();

        for grupo in [SpatialGroupId::Meadows, SpatialGroupId::FlyingWaters] {
            for &i in &indices_del_grupo(&apoyado, grupo) {
                assert_eq!(
                    separado.scene.objects[i].primitive.bounds(),
                    apoyado.scene.objects[i].primitive.bounds(),
                    "la pieza {i} de {grupo:?} se movio y el encargo lo prohibe"
                );
            }
        }
    }

    #[test]
    fn el_apoyado_solo_traslada_el_rompeolas_y_su_pedestal() {
        let separado = nivel_separado();
        let apoyado = nivel_apoyado();

        let pedestal = masa_pedestal(&separado.scene);
        let rompeolas = indices_del_grupo(&apoyado, SpatialGroupId::Breakwater);

        for i in 0..OBJETOS {
            let a = &separado.scene.objects[i];
            let b = &apoyado.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            let antes = a.primitive.bounds();
            let ahora = b.primitive.bounds();

            assert!(
                (tamano_de(&ahora) - tamano_de(&antes)).magnitude() < EPS,
                "la pieza {i} cambio de tamano: grounded solo traslada"
            );

            if antes != ahora {
                assert!(
                    i == pedestal || rompeolas.contains(&i),
                    "la pieza {i} se movio y no es ni el Rompeolas ni su pedestal"
                );
            }
        }

        // El Rompeolas baja **en bloque**: si cada pieza bajara lo suyo, la
        // macroforma que la revision aprobo dejaria de ser la misma.
        let delta = |i: usize| {
            centro_de(&apoyado.scene.objects[i].primitive.bounds())
                - centro_de(&separado.scene.objects[i].primitive.bounds())
        };
        let comun = delta(rompeolas[0]);

        assert!(comun.y < 0.0, "el Rompeolas no bajo");
        assert!(
            comun.x.abs() < EPS && comun.z.abs() < EPS,
            "el Rompeolas se movio en planta y eso cambiaria su huella"
        );

        for &i in &rompeolas {
            assert!(
                (delta(i) - comun).magnitude() < EPS,
                "la pieza {i} del Rompeolas no bajo lo mismo que el resto"
            );
        }
    }

    #[test]
    fn cada_pilar_de_r01_tiene_pedestal_bajo_su_huella() {
        // El hallazgo humano: prismas que flotan. Un pedestal de verdad
        // **contiene** la huella de la pieza; que se toquen por una esquina
        // no sostiene nada.
        let apoyado = nivel_apoyado();
        let rompeolas = indices_del_grupo(&apoyado, SpatialGroupId::Breakwater);

        assert!(rompeolas.len() >= PILARES, "el Rompeolas perdio piezas");

        for &i in &rompeolas[..PILARES] {
            let caja = apoyado.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&apoyado.scene, i)
                .unwrap_or_else(|| panic!("el pilar {i} de R-01 no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo,
                "el pilar {i} arranca en {:.3} y su pedestal llega a {techo:.3}: flota {:.3}",
                caja.min.y,
                caja.min.y - techo
            );
            assert!(
                caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "el pilar {i} arranca en {:.3} y su pedestal llega a {techo:.3}: esta enterrado",
                caja.min.y
            );
        }
    }

    #[test]
    fn el_pedestal_se_apoya_en_el_plinto() {
        // Un pedestal que flota no arregla nada.
        let apoyado = nivel_apoyado();

        let i = masa_pedestal(&apoyado.scene);
        let losa = apoyado.scene.objects[i].primitive.bounds();
        let plinto = huella_del_grupo(&apoyado.scene, SpatialGroupId::Global);

        assert!(
            losa.min.y <= plinto.max.y,
            "la losa pedestal arranca en {:.3} y el plinto acaba en {:.3}",
            losa.min.y,
            plinto.max.y
        );
        assert!(
            losa.min.y >= plinto.min.y,
            "la losa pedestal atraviesa el plinto por debajo"
        );

        // Y se queda dentro del plinto en planta: un pedestal en voladizo
        // seria otro objeto flotando.
        assert!(
            losa.min.x >= plinto.min.x
                && losa.max.x <= plinto.max.x
                && losa.min.z >= plinto.min.z
                && losa.max.z <= plinto.max.z,
            "la losa pedestal se sale del plinto"
        );
    }

    #[test]
    fn las_tres_regiones_siguen_separadas_en_el_apoyado() {
        let separado = nivel_separado();
        let apoyado = nivel_apoyado();

        let huellas = [
            ("Praderas", SpatialGroupId::Meadows),
            ("Rompeolas", SpatialGroupId::Breakwater),
            ("Aguas", SpatialGroupId::FlyingWaters),
        ];

        // Las huellas en planta son **las mismas** que aprobo la revision:
        // el Rompeolas solo baja.
        for (nombre, grupo) in huellas {
            let antes = huella_del_grupo(&separado.scene, grupo);
            let ahora = huella_del_grupo(&apoyado.scene, grupo);

            assert!(
                (antes.min.x - ahora.min.x).abs() < EPS
                    && (antes.max.x - ahora.max.x).abs() < EPS
                    && (antes.min.z - ahora.min.z).abs() < EPS
                    && (antes.max.z - ahora.max.z).abs() < EPS,
                "{nombre} cambio de huella en planta"
            );
        }

        for a in 0..huellas.len() {
            for b in a + 1..huellas.len() {
                let claro = separacion_xz(
                    &huella_del_grupo(&apoyado.scene, huellas[a].1),
                    &huella_del_grupo(&apoyado.scene, huellas[b].1),
                );

                assert!(
                    claro >= CLARO_MINIMO,
                    "{} y {} dejan {claro:.3} de claro",
                    huellas[a].0,
                    huellas[b].0
                );
            }
        }
    }

    #[test]
    fn el_rompeolas_no_choca_con_praderas_ni_con_aguas() {
        // Bajar es seguro en planta, pero hay que decirlo en volumen: las
        // cajas de las tres regiones siguen sin cortarse en ningun eje.
        let apoyado = nivel_apoyado();

        let rompeolas = huella_del_grupo(&apoyado.scene, SpatialGroupId::Breakwater);

        for (nombre, grupo) in [
            ("Praderas", SpatialGroupId::Meadows),
            ("Aguas", SpatialGroupId::FlyingWaters),
        ] {
            let otra = huella_del_grupo(&apoyado.scene, grupo);

            let cortan = rompeolas.min.x < otra.max.x
                && rompeolas.max.x > otra.min.x
                && rompeolas.min.y < otra.max.y
                && rompeolas.max.y > otra.min.y
                && rompeolas.min.z < otra.max.z
                && rompeolas.max.z > otra.min.z;

            assert!(!cortan, "el Rompeolas y {nombre} se cortan");
        }
    }

    #[test]
    fn el_nivel_apoyado_es_determinista() {
        let uno = nivel_apoyado();
        let dos = nivel_apoyado();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
        assert_eq!(
            uno.anchors.hero_camera_anchor,
            dos.anchors.hero_camera_anchor
        );
    }

    #[test]
    fn la_escala_apoyada_se_deriva_de_la_escena() {
        let apoyado = nivel_apoyado();

        assert_eq!(
            apoyado.scale.scene_radius,
            measure_scene_radius(&apoyado.scene, apoyado.anchors.orbit_center),
            "scene_radius no se volvio a medir"
        );
        assert_eq!(
            apoyado.scale.orbit_radius,
            derive_orbit_radius(apoyado.scale.scene_radius, apoyado.scale.monolith_height),
            "orbit_radius no corresponde con scene_radius y monolith_height"
        );
        assert_eq!(
            apoyado.anchors.hero_camera_anchor,
            eye_at_yaw(
                apoyado.anchors.orbit_center,
                apoyado.scale.orbit_radius,
                HERO_YAW_DEGREES
            ),
            "la toma hero no se derivo de la escala apoyada"
        );
    }

    #[test]
    fn los_siete_niveles_previos_no_cambian_al_construir_el_apoyado() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
        ];

        let _apoyado = nivel_apoyado();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
            assert_eq!(
                antes.anchors.hero_camera_anchor,
                ahora.anchors.hero_camera_anchor
            );
        }
    }

    #[test]
    fn el_separado_dejaba_pilares_sin_pedestal() {
        // El hallazgo, escrito como test: en `separated` hay pilares de
        // `R-01` sin nada bajo su huella. Si algun dia esto dejara de ser
        // cierto, `grounded` habria dejado de hacer falta.
        let separado = nivel_separado();
        let rompeolas = indices_del_grupo(&separado, SpatialGroupId::Breakwater);

        let flotando = rompeolas[..PILARES]
            .iter()
            .filter(|&&i| {
                let caja = separado.scene.objects[i].primitive.bounds();

                match pedestal_bajo(&separado.scene, i) {
                    None => true,
                    Some(techo) => caja.min.y > techo,
                }
            })
            .count();

        assert!(
            flotando > 0,
            "separated ya apoyaba los {PILARES} pilares: grounded no corrige nada"
        );
    }

    // ------------------------------------------------------------------
    // La escalera irregular: `staircase`
    // ------------------------------------------------------------------

    /// Cuánto tiene que subir la escalera entre su tercio bajo y su tercio
    /// alto para que la subida se lea y no sea ruido.
    const UMBRAL_SUBIDA: f32 = 2.00;

    /// Cuánto tiene que correrse en `x` entre esos mismos tercios para que
    /// la diagonal se lea.
    const UMBRAL_DIAGONAL: f32 = 2.00;

    /// Cuántas inversiones locales de altura se exigen como mínimo.
    ///
    /// Sin ninguna, la subida sería una rampa peinada; es justo lo que la
    /// dirección pide evitar.
    const INVERSIONES_MINIMAS: usize = 4;

    /// Cuánto tiene que separarse la nube de pilares de su propia recta de
    /// regresión. Un peine se ajusta a una recta; una formación no.
    const DISPERSION_MINIMA: f32 = 0.40;

    fn tercios(valores: &[f32]) -> (f32, f32) {
        let n = valores.len() / 3;
        let media = |v: &[f32]| v.iter().sum::<f32>() / v.len() as f32;

        (media(&valores[..n]), media(&valores[valores.len() - n..]))
    }

    #[test]
    fn el_nivel_escalera_conserva_el_conteo_del_nivel() {
        let escalera = nivel_escalera();

        assert_eq!(
            escalera.scene.objects.len(),
            OBJETOS,
            "la escalera cambio el conteo del nivel"
        );
    }

    #[test]
    fn la_escalera_conserva_el_rompeolas_y_su_soporte() {
        let apoyado = nivel_apoyado();
        let escalera = nivel_escalera();

        let piezas = indices_del_grupo(&escalera, SpatialGroupId::Breakwater);
        assert_eq!(piezas.len(), 38, "el Rompeolas no tiene 38 piezas");

        // Solo se reorganizan los 28 pilares de `R-01`. `R-02`, `R-03` y la
        // losa que los sostiene se quedan exactamente donde estan.
        let pedestal = masa_pedestal(&apoyado.scene);

        for i in 0..OBJETOS {
            let a = &apoyado.scene.objects[i];
            let b = &escalera.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            if a.primitive.bounds() != b.primitive.bounds() {
                assert!(
                    piezas[..PILARES].contains(&i),
                    "la pieza {i} cambio y no es un pilar de R-01"
                );
            }
        }

        assert_eq!(
            apoyado.scene.objects[pedestal].primitive.bounds(),
            escalera.scene.objects[pedestal].primitive.bounds(),
            "la losa pedestal se movio"
        );
    }

    #[test]
    fn praderas_y_aguas_no_se_mueven_en_la_escalera() {
        let apoyado = nivel_apoyado();
        let escalera = nivel_escalera();

        for grupo in [SpatialGroupId::Meadows, SpatialGroupId::FlyingWaters] {
            for &i in &indices_del_grupo(&escalera, grupo) {
                assert_eq!(
                    apoyado.scene.objects[i].primitive.bounds(),
                    escalera.scene.objects[i].primitive.bounds(),
                    "la pieza {i} de {grupo:?} se movio y el encargo lo prohibe"
                );
            }
        }
    }

    #[test]
    fn cada_pilar_de_la_escalera_tiene_pedestal() {
        let escalera = nivel_escalera();
        let piezas = indices_del_grupo(&escalera, SpatialGroupId::Breakwater);

        for &i in &piezas[..PILARES] {
            let caja = escalera.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&escalera.scene, i)
                .unwrap_or_else(|| panic!("el pilar {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo,
                "el pilar {i} arranca en {:.3} y su pedestal llega a {techo:.3}: flota",
                caja.min.y
            );
            assert!(
                caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "el pilar {i} arranca en {:.3} sobre un pedestal de {techo:.3}: enterrado",
                caja.min.y
            );
        }
    }

    #[test]
    fn la_escalera_no_toca_praderas_ni_aguas() {
        let escalera = nivel_escalera();

        let huellas = [
            ("Praderas", SpatialGroupId::Meadows),
            ("Rompeolas", SpatialGroupId::Breakwater),
            ("Aguas", SpatialGroupId::FlyingWaters),
        ];

        for a in 0..huellas.len() {
            for b in a + 1..huellas.len() {
                let claro = separacion_xz(
                    &huella_del_grupo(&escalera.scene, huellas[a].1),
                    &huella_del_grupo(&escalera.scene, huellas[b].1),
                );

                assert!(
                    claro >= CLARO_MINIMO,
                    "{} y {} dejan {claro:.3} de claro",
                    huellas[a].0,
                    huellas[b].0
                );
            }
        }

        // Y ningun pilar suelto entra en la caja de Praderas, ni siquiera de
        // los que van por delante del resto.
        let praderas = huella_del_grupo(&escalera.scene, SpatialGroupId::Meadows);
        let piezas = indices_del_grupo(&escalera, SpatialGroupId::Breakwater);

        for &i in &piezas[..PILARES] {
            let caja = escalera.scene.objects[i].primitive.bounds();

            assert!(
                separacion_xz(&caja, &praderas) >= CLARO_MINIMO,
                "el pilar {i} se metio a {:.3} de Praderas",
                separacion_xz(&caja, &praderas)
            );
        }
    }

    #[test]
    fn la_escalera_apunta_a_la_izquierda_de_praderas() {
        let escalera = nivel_escalera();

        let praderas = huella_del_grupo(&escalera.scene, SpatialGroupId::Meadows);
        let orden = pilares_en_orden(&escalera.scene);
        let ultimo = escalera.scene.objects[*orden.last().expect("hay pilares")]
            .primitive
            .bounds();

        // El ultimo pilar llega hasta el claro y no mas: la subida se
        // detiene antes de la meseta.
        assert!(
            (ultimo.min.z - praderas.max.z) >= CLARO_MINIMO,
            "el ultimo pilar queda a {:.3} de Praderas",
            ultimo.min.z - praderas.max.z
        );
        assert!(
            (ultimo.min.z - praderas.max.z) <= CLARO_MINIMO * 3.0,
            "el ultimo pilar se queda a {:.3}: la subida no llega a Praderas",
            ultimo.min.z - praderas.max.z
        );

        // Y llega por la **izquierda**: su centro cae en la mitad izquierda
        // de la meseta o mas alla, nunca hacia Aguas Voladoras.
        let centro_ultimo = (ultimo.min.x + ultimo.max.x) * 0.5;
        let medio_praderas = (praderas.min.x + praderas.max.x) * 0.5;

        assert!(
            centro_ultimo < medio_praderas,
            "el ultimo pilar cae en x {centro_ultimo:.2} y el medio de Praderas es {medio_praderas:.2}"
        );

        let aguas = huella_del_grupo(&escalera.scene, SpatialGroupId::FlyingWaters);

        assert!(
            centro_ultimo < aguas.min.x,
            "la subida se fue hacia Aguas Voladoras"
        );
    }

    #[test]
    fn la_escalera_asciende_por_la_diagonal() {
        let escalera = nivel_escalera();
        let orden = pilares_en_orden(&escalera.scene);

        let techos: Vec<f32> = orden
            .iter()
            .map(|&i| escalera.scene.objects[i].primitive.bounds().max.y)
            .collect();
        let equis: Vec<f32> = orden
            .iter()
            .map(|&i| {
                let c = escalera.scene.objects[i].primitive.bounds();

                (c.min.x + c.max.x) * 0.5
            })
            .collect();

        let (techo_bajo, techo_alto) = tercios(&techos);
        let (x_bajo, x_alto) = tercios(&equis);

        assert!(
            techo_alto - techo_bajo >= UMBRAL_SUBIDA,
            "el tercio bajo remata en {techo_bajo:.2} y el alto en {techo_alto:.2}: no sube"
        );
        assert!(
            x_alto - x_bajo >= UMBRAL_DIAGONAL,
            "el tercio bajo esta en x {x_bajo:.2} y el alto en {x_alto:.2}: no hay diagonal"
        );
    }

    #[test]
    fn la_escalera_tiene_variacion_local() {
        // Sube de conjunto, pero no peldano a peldano: una rampa perfecta
        // seria una escalera humana, y la direccion pide geologia.
        let escalera = nivel_escalera();
        let orden = pilares_en_orden(&escalera.scene);

        let techos: Vec<f32> = orden
            .iter()
            .map(|&i| escalera.scene.objects[i].primitive.bounds().max.y)
            .collect();

        let inversiones = techos.windows(2).filter(|par| par[1] < par[0]).count();

        assert!(
            inversiones >= INVERSIONES_MINIMAS,
            "solo {inversiones} bajadas locales en {} pilares: la subida es una rampa",
            techos.len()
        );
        assert!(
            inversiones < techos.len() - 1,
            "todas las parejas bajan: eso no es una subida"
        );
    }

    #[test]
    fn las_huellas_de_la_escalera_son_irregulares() {
        let escalera = nivel_escalera();
        let orden = pilares_en_orden(&escalera.scene);
        let cajas: Vec<Aabb> = orden
            .iter()
            .map(|&i| escalera.scene.objects[i].primitive.bounds())
            .collect();

        // Huellas de tamano distinto.
        let areas: Vec<f32> = cajas
            .iter()
            .map(|c| (c.max.x - c.min.x) * (c.max.z - c.min.z))
            .collect();
        let minima = areas.iter().copied().fold(f32::MAX, f32::min);
        let maxima = areas.iter().copied().fold(f32::MIN, f32::max);

        assert!(
            maxima >= minima * 1.30,
            "las huellas van de {minima:.3} a {maxima:.3}: son todas iguales"
        );

        // Avances de tamano distinto: nada de paso constante.
        let avances: Vec<f32> = cajas
            .windows(2)
            .map(|par| {
                let a = (par[0].min.z + par[0].max.z) * 0.5;
                let b = (par[1].min.z + par[1].max.z) * 0.5;

                (a - b).abs()
            })
            .collect();
        let corto = avances.iter().copied().fold(f32::MAX, f32::min);
        let largo = avances.iter().copied().fold(f32::MIN, f32::max);

        assert!(
            largo >= corto * 1.50 + EPS,
            "los avances van de {corto:.3} a {largo:.3}: paso constante"
        );

        // Y la nube no se ajusta a una recta: un peine si lo haria.
        let n = cajas.len() as f32;
        let xs: Vec<f32> = cajas.iter().map(|c| (c.min.x + c.max.x) * 0.5).collect();
        let zs: Vec<f32> = cajas.iter().map(|c| -(c.min.z + c.max.z) * 0.5).collect();
        let media_x = xs.iter().sum::<f32>() / n;
        let media_z = zs.iter().sum::<f32>() / n;

        let sxz: f32 = xs
            .iter()
            .zip(&zs)
            .map(|(x, z)| (x - media_x) * (z - media_z))
            .sum();
        let szz: f32 = zs.iter().map(|z| (z - media_z) * (z - media_z)).sum();
        let pendiente = sxz / szz;

        let residuo: f32 = (xs
            .iter()
            .zip(&zs)
            .map(|(x, z)| {
                let e = x - (media_x + pendiente * (z - media_z));

                e * e
            })
            .sum::<f32>()
            / n)
            .sqrt();

        assert!(
            residuo >= DISPERSION_MINIMA,
            "los pilares se ajustan a una recta con residuo {residuo:.3}: parece un peine"
        );
    }

    #[test]
    fn el_nivel_escalera_es_determinista() {
        let uno = nivel_escalera();
        let dos = nivel_escalera();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
        assert_eq!(
            uno.anchors.hero_camera_anchor,
            dos.anchors.hero_camera_anchor
        );
    }

    #[test]
    fn la_escala_de_la_escalera_se_deriva_de_la_escena() {
        let escalera = nivel_escalera();

        assert_eq!(
            escalera.scale.scene_radius,
            measure_scene_radius(&escalera.scene, escalera.anchors.orbit_center),
            "scene_radius no se volvio a medir"
        );
        assert_eq!(
            escalera.scale.orbit_radius,
            derive_orbit_radius(escalera.scale.scene_radius, escalera.scale.monolith_height),
            "orbit_radius no corresponde con scene_radius y monolith_height"
        );
        assert_eq!(
            escalera.anchors.hero_camera_anchor,
            eye_at_yaw(
                escalera.anchors.orbit_center,
                escalera.scale.orbit_radius,
                HERO_YAW_DEGREES
            ),
            "la toma hero no se derivo de la escala de la escalera"
        );
    }

    #[test]
    fn los_ocho_niveles_previos_no_cambian_al_construir_la_escalera() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
        ];

        let _escalera = nivel_escalera();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }
    }

    // ------------------------------------------------------------------
    // Macizo, ascenso y barranco: `combined`
    // ------------------------------------------------------------------

    /// Cuántos prismas exige el macizo para leerse como masa y no como
    /// grupo de columnas sueltas.
    const MACIZO_MINIMO: usize = 12;

    /// Cuantas veces mas denso que el ascenso tiene que ser el macizo.
    ///
    /// Es lo que separa una masa de un recorrido. La cifra absoluta no dice
    /// nada por si sola —depende del ancho de los prismas—; la comparacion
    /// entre las dos partes del mismo reparto si.
    const DENSIDAD_RELATIVA: f32 = 2.00;

    /// Separación mínima entre dos racimos laterales para contarlos como
    /// dos.
    const RACIMO_APARTE: f32 = 1.50;

    /// Radio máximo de un racimo alrededor de su centro.
    const RACIMO_COMPACTO: f32 = 2.50;

    fn centro_xz(caja: &Aabb) -> (f32, f32) {
        (
            (caja.min.x + caja.max.x) * 0.5,
            (caja.min.z + caja.max.z) * 0.5,
        )
    }

    /// Agrupa por enlace simple: dos prismas del mismo racimo si sus centros
    /// distan menos que el umbral.
    fn racimos_por_cercania(centros: &[(f32, f32)], umbral: f32) -> Vec<Vec<usize>> {
        let mut grupo: Vec<Option<usize>> = vec![None; centros.len()];
        let mut racimos: Vec<Vec<usize>> = Vec::new();

        for i in 0..centros.len() {
            if grupo[i].is_some() {
                continue;
            }

            let id = racimos.len();
            racimos.push(Vec::new());

            let mut pila = vec![i];
            grupo[i] = Some(id);

            while let Some(k) = pila.pop() {
                racimos[id].push(k);

                for j in 0..centros.len() {
                    if grupo[j].is_some() {
                        continue;
                    }

                    let d = ((centros[k].0 - centros[j].0).powi(2)
                        + (centros[k].1 - centros[j].1).powi(2))
                    .sqrt();

                    if d < umbral {
                        grupo[j] = Some(id);
                        pila.push(j);
                    }
                }
            }
        }

        racimos
    }

    #[test]
    fn el_nivel_combinado_conserva_el_conteo_del_nivel() {
        let combinado = nivel_combinado();

        assert_eq!(
            combinado.scene.objects.len(),
            OBJETOS,
            "el combinado cambio el conteo del nivel"
        );
    }

    #[test]
    fn el_plinto_el_monolito_y_el_volumen_siguen_exactos_en_el_combinado() {
        let apoyado = nivel_apoyado();
        let combinado = nivel_combinado();

        for grupo in [SpatialGroupId::Global, SpatialGroupId::Monolith] {
            for &i in &indices_del_grupo(&combinado, grupo) {
                assert_eq!(
                    apoyado.scene.objects[i].primitive.bounds(),
                    combinado.scene.objects[i].primitive.bounds(),
                    "la pieza {i} de {grupo:?} se movio"
                );
            }
        }

        let a01 = volumen_de_agua(&combinado.scene);
        assert_eq!(
            volumen_de_agua(&apoyado.scene),
            a01,
            "A-01 cambio de indice"
        );
        assert_eq!(
            apoyado.scene.objects[a01].primitive.bounds(),
            combinado.scene.objects[a01].primitive.bounds(),
            "A-01 se movio o se deformo"
        );
    }

    #[test]
    fn el_combinado_solo_reorganiza_los_pilares() {
        let apoyado = nivel_apoyado();
        let combinado = nivel_combinado();

        let piezas = indices_del_grupo(&combinado, SpatialGroupId::Breakwater);
        assert_eq!(piezas.len(), 38, "el Rompeolas no tiene 38 piezas");

        for i in 0..OBJETOS {
            let a = &apoyado.scene.objects[i];
            let b = &combinado.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            if a.primitive.bounds() != b.primitive.bounds() {
                assert!(
                    piezas[..PILARES].contains(&i),
                    "la pieza {i} cambio y no es un pilar de R-01"
                );
            }
        }
    }

    #[test]
    fn praderas_y_aguas_no_se_mueven_en_el_combinado() {
        let apoyado = nivel_apoyado();
        let combinado = nivel_combinado();

        for grupo in [SpatialGroupId::Meadows, SpatialGroupId::FlyingWaters] {
            for &i in &indices_del_grupo(&combinado, grupo) {
                assert_eq!(
                    apoyado.scene.objects[i].primitive.bounds(),
                    combinado.scene.objects[i].primitive.bounds(),
                    "la pieza {i} de {grupo:?} se movio"
                );
            }
        }
    }

    #[test]
    fn cada_pilar_del_combinado_tiene_pedestal() {
        let combinado = nivel_combinado();
        let piezas = indices_del_grupo(&combinado, SpatialGroupId::Breakwater);

        for &i in &piezas[..PILARES] {
            let caja = combinado.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&combinado.scene, i)
                .unwrap_or_else(|| panic!("el pilar {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo,
                "el pilar {i} arranca en {:.3} y su pedestal llega a {techo:.3}: flota",
                caja.min.y
            );
            assert!(
                caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "el pilar {i} arranca en {:.3} sobre un pedestal de {techo:.3}: enterrado",
                caja.min.y
            );
        }
    }

    #[test]
    fn el_combinado_no_toca_praderas_ni_aguas() {
        let combinado = nivel_combinado();

        let huellas = [
            ("Praderas", SpatialGroupId::Meadows),
            ("Rompeolas", SpatialGroupId::Breakwater),
            ("Aguas", SpatialGroupId::FlyingWaters),
        ];

        for a in 0..huellas.len() {
            for b in a + 1..huellas.len() {
                let claro = separacion_xz(
                    &huella_del_grupo(&combinado.scene, huellas[a].1),
                    &huella_del_grupo(&combinado.scene, huellas[b].1),
                );

                assert!(
                    claro >= CLARO_MINIMO,
                    "{} y {} dejan {claro:.3} de claro",
                    huellas[a].0,
                    huellas[b].0
                );
            }
        }

        let praderas = huella_del_grupo(&combinado.scene, SpatialGroupId::Meadows);
        let piezas = indices_del_grupo(&combinado, SpatialGroupId::Breakwater);

        for &i in &piezas[..PILARES] {
            let caja = combinado.scene.objects[i].primitive.bounds();

            assert!(
                separacion_xz(&caja, &praderas) >= CLARO_MINIMO,
                "el pilar {i} se metio a {:.3} de Praderas",
                separacion_xz(&caja, &praderas)
            );
        }
    }

    #[test]
    fn el_macizo_basal_es_numeroso_y_compacto() {
        // La correccion humana: que no parezca que todos los prismas se
        // fueron de excursion. Tiene que quedar masa, y masa **apretada**:
        // en las referencias el bloque no tiene huecos entre columnas.
        let combinado = nivel_combinado();

        let macizo = macizo_basal(&combinado.scene);
        let ascenso = ascenso_combinado(&combinado.scene);

        assert!(
            macizo.len() >= MACIZO_MINIMO,
            "el macizo se quedo en {} prismas y el minimo es {MACIZO_MINIMO}",
            macizo.len()
        );

        let caja = huella(&combinado.scene, &macizo);
        let superficie = (caja.max.x - caja.min.x) * (caja.max.z - caja.min.z);

        // Compacidad, tal y como se ve en `im1`: los prismas se tocan. La
        // suma de sus huellas tiene que **cubrir** la caja del macizo; si
        // sobra caja, hay huecos entre columnas.
        let suma: f32 = macizo
            .iter()
            .map(|&i| {
                let c = combinado.scene.objects[i].primitive.bounds();

                (c.max.x - c.min.x) * (c.max.z - c.min.z)
            })
            .sum();

        assert!(
            suma >= superficie,
            "las huellas suman {suma:.2} y la caja del macizo mide {superficie:.2}: hay huecos"
        );

        // Y es mas denso que el ascenso por un margen claro: uno es masa y
        // el otro es recorrido.
        let densidad = macizo.len() as f32 / superficie;

        // Y es compacto **frente al ascenso**: el macizo es una mancha, la
        // subida es un recorrido.
        let caja_ascenso = huella(&combinado.scene, &ascenso);
        let superficie_ascenso =
            (caja_ascenso.max.x - caja_ascenso.min.x) * (caja_ascenso.max.z - caja_ascenso.min.z);

        assert!(
            superficie < superficie_ascenso,
            "el macizo ocupa {superficie:.2} y el ascenso {superficie_ascenso:.2}"
        );

        let densidad_ascenso = ascenso.len() as f32 / superficie_ascenso;

        assert!(
            densidad >= densidad_ascenso * DENSIDAD_RELATIVA,
            "el macizo tiene {densidad:.3} prismas por unidad y el ascenso {densidad_ascenso:.3}"
        );
    }

    #[test]
    fn el_ascenso_progresa_de_forma_irregular() {
        let combinado = nivel_combinado();
        let ascenso = ascenso_combinado(&combinado.scene);

        assert!(
            ascenso.len() >= 6,
            "el ascenso se quedo en {}",
            ascenso.len()
        );

        let techos: Vec<f32> = ascenso
            .iter()
            .map(|&i| combinado.scene.objects[i].primitive.bounds().max.y)
            .collect();
        let equis: Vec<f32> = ascenso
            .iter()
            .map(|&i| centro_xz(&combinado.scene.objects[i].primitive.bounds()).0)
            .collect();

        let (techo_bajo, techo_alto) = tercios(&techos);
        let (x_bajo, x_alto) = tercios(&equis);

        assert!(
            techo_alto - techo_bajo >= 1.50,
            "el tercio bajo remata en {techo_bajo:.2} y el alto en {techo_alto:.2}: no sube"
        );
        assert!(
            x_alto - x_bajo >= 1.00,
            "el tercio bajo esta en x {x_bajo:.2} y el alto en {x_alto:.2}: no hay diagonal"
        );

        // Irregular: el conjunto sube pero las parejas vecinas se cruzan.
        let inversiones = techos.windows(2).filter(|par| par[1] < par[0]).count();

        assert!(
            inversiones >= 2,
            "solo {inversiones} bajadas locales: el ascenso es una rampa"
        );
        assert!(inversiones < techos.len() - 1, "el ascenso solo baja");

        // Y los avances no son constantes.
        let avances: Vec<f32> = ascenso
            .windows(2)
            .map(|par| {
                let a = centro_xz(&combinado.scene.objects[par[0]].primitive.bounds()).1;
                let b = centro_xz(&combinado.scene.objects[par[1]].primitive.bounds()).1;

                (a - b).abs()
            })
            .collect();
        let corto = avances.iter().copied().fold(f32::MAX, f32::min);
        let largo = avances.iter().copied().fold(f32::MIN, f32::max);

        assert!(
            largo >= corto * 1.50 + EPS,
            "los avances van de {corto:.3} a {largo:.3}: paso constante"
        );
    }

    #[test]
    fn el_ascenso_se_detiene_antes_de_praderas_y_no_va_a_aguas() {
        let combinado = nivel_combinado();

        let praderas = huella_del_grupo(&combinado.scene, SpatialGroupId::Meadows);
        let aguas = huella_del_grupo(&combinado.scene, SpatialGroupId::FlyingWaters);
        let ascenso = ascenso_combinado(&combinado.scene);
        let ultimo = combinado.scene.objects[*ascenso.last().expect("hay ascenso")]
            .primitive
            .bounds();

        let claro = ultimo.min.z - praderas.max.z;

        assert!(
            claro >= CLARO_MINIMO,
            "el ultimo prisma queda a {claro:.3} de Praderas"
        );
        assert!(
            claro <= CLARO_MINIMO * 3.0,
            "el ultimo prisma queda a {claro:.3}: la subida no llega"
        );

        let centro_ultimo = centro_xz(&ultimo).0;

        assert!(
            centro_ultimo < (praderas.min.x + praderas.max.x) * 0.5,
            "el ascenso no llega por la izquierda de Praderas"
        );
        assert!(
            centro_ultimo < aguas.min.x,
            "el ascenso se fue hacia Aguas Voladoras"
        );
    }

    #[test]
    fn los_racimos_laterales_dibujan_un_barranco() {
        // De las referencias: una pared de columnas altas y, a su pie, un
        // abanico de columnas mucho mas bajas. Dos racimos, no un reparto.
        let combinado = nivel_combinado();

        let laterales = laterales_combinados(&combinado.scene);
        let macizo = macizo_basal(&combinado.scene);

        assert!(
            laterales.len() >= 4,
            "solo {} prismas laterales",
            laterales.len()
        );

        let centros: Vec<(f32, f32)> = laterales
            .iter()
            .map(|&i| centro_xz(&combinado.scene.objects[i].primitive.bounds()))
            .collect();
        let racimos = racimos_por_cercania(&centros, RACIMO_APARTE);

        assert_eq!(
            racimos.len(),
            2,
            "no hay dos racimos, hay {}",
            racimos.len()
        );

        for racimo in &racimos {
            assert!(racimo.len() >= 2, "un racimo se quedo en {}", racimo.len());

            for &a in racimo {
                for &b in racimo {
                    let d = ((centros[a].0 - centros[b].0).powi(2)
                        + (centros[a].1 - centros[b].1).powi(2))
                    .sqrt();

                    assert!(d <= RACIMO_COMPACTO, "un racimo se desparramo {d:.2}");
                }
            }
        }

        // Uno alto y otro bajo: eso es el barranco y su abanico.
        let techo_medio = |racimo: &[usize]| {
            racimo
                .iter()
                .map(|&k| {
                    combinado.scene.objects[laterales[k]]
                        .primitive
                        .bounds()
                        .max
                        .y
                })
                .sum::<f32>()
                / racimo.len() as f32
        };

        let mut alturas = [techo_medio(&racimos[0]), techo_medio(&racimos[1])];
        alturas.sort_by(|a, b| a.partial_cmp(b).expect("no hay NaN"));

        assert!(
            alturas[1] - alturas[0] >= 1.50,
            "los dos racimos rematan en {:.2} y {:.2}: no hay barranco",
            alturas[0],
            alturas[1]
        );

        // Y son **laterales**: quedan por delante del macizo, en el flanco
        // que mira a la camara, no repartidos por otras zonas.
        let z_lateral = centros.iter().map(|c| c.1).sum::<f32>() / centros.len() as f32;
        let caja_macizo = huella(&combinado.scene, &macizo);
        let z_macizo = (caja_macizo.min.z + caja_macizo.max.z) * 0.5;

        assert!(
            z_lateral > z_macizo + 1.00,
            "los racimos estan en z {z_lateral:.2} y el macizo en {z_macizo:.2}"
        );
    }

    #[test]
    fn el_nivel_combinado_es_determinista() {
        let uno = nivel_combinado();
        let dos = nivel_combinado();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
    }

    #[test]
    fn la_escala_del_combinado_se_deriva_de_la_escena() {
        let combinado = nivel_combinado();

        assert_eq!(
            combinado.scale.scene_radius,
            measure_scene_radius(&combinado.scene, combinado.anchors.orbit_center),
            "scene_radius no se volvio a medir"
        );
        assert_eq!(
            combinado.scale.orbit_radius,
            derive_orbit_radius(
                combinado.scale.scene_radius,
                combinado.scale.monolith_height
            ),
            "orbit_radius no corresponde con scene_radius y monolith_height"
        );
        assert_eq!(
            combinado.anchors.hero_camera_anchor,
            eye_at_yaw(
                combinado.anchors.orbit_center,
                combinado.scale.orbit_radius,
                HERO_YAW_DEGREES
            ),
            "la toma hero no se derivo de la escala del combinado"
        );
    }

    #[test]
    fn los_nueve_niveles_previos_no_cambian_al_construir_el_combinado() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
        ];

        let _combinado = nivel_combinado();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }
    }

    // ------------------------------------------------------------------
    // Extensión desde la cima: `grounded_ascent`
    // ------------------------------------------------------------------

    /// Cuántos prismas puede llevarse la extensión, como mucho.
    const CIMAS_MAXIMO: usize = 8;

    /// Qué parte de `R-01` tiene que quedarse en el macizo.
    const MACIZO_MAYORITARIO: usize = 20;

    /// El hueco libre mínimo que se exige en algún punto de la extensión.
    ///
    /// Sin un vacío de verdad, seis prismas seguidos a lo largo de una
    /// diagonal son un puente, y eso es justo lo que no se quiere.
    const VACIO_MINIMO: f32 = 0.80;

    #[test]
    fn el_ascenso_desde_grounded_conserva_el_conteo_del_nivel() {
        let nuevo = nivel_ascenso_desde_grounded();

        assert_eq!(
            nuevo.scene.objects.len(),
            OBJETOS,
            "la extension cambio el conteo del nivel"
        );
    }

    #[test]
    fn la_seleccion_son_los_prismas_mas_altos_y_son_pocos() {
        let apoyado = nivel_apoyado();
        let cimas = cimas_seleccionadas(&apoyado.scene);

        assert!(
            !cimas.is_empty() && cimas.len() <= CIMAS_MAXIMO,
            "la seleccion tiene {} prismas y el maximo es {CIMAS_MAXIMO}",
            cimas.len()
        );

        // Elegidos por techo medido, no por indice: cualquiera de los
        // elegidos remata mas alto que cualquiera de los que se quedan.
        let piezas = indices_del_grupo(&apoyado, SpatialGroupId::Breakwater);
        let techo = |i: usize| apoyado.scene.objects[i].primitive.bounds().max.y;

        let mas_bajo_elegido = cimas.iter().map(|&i| techo(i)).fold(f32::MAX, f32::min);
        let mas_alto_restante = piezas[..PILARES]
            .iter()
            .filter(|i| !cimas.contains(i))
            .map(|&i| techo(i))
            .fold(f32::MIN, f32::max);

        assert!(
            mas_bajo_elegido > mas_alto_restante,
            "el elegido mas bajo remata en {mas_bajo_elegido:.3} y el mas alto que se queda \
             en {mas_alto_restante:.3}: la seleccion no es por altura"
        );
    }

    #[test]
    fn grounded_queda_intacto_salvo_esa_seleccion() {
        let apoyado = nivel_apoyado();
        let nuevo = nivel_ascenso_desde_grounded();

        let cimas = cimas_seleccionadas(&apoyado.scene);

        for i in 0..OBJETOS {
            let a = &apoyado.scene.objects[i];
            let b = &nuevo.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            let antes = a.primitive.bounds();
            let ahora = b.primitive.bounds();

            if cimas.contains(&i) {
                // Se mueven, pero **enteros**: la extension traslada, no
                // reconstruye. Cada prisma conserva su tamano.
                assert!(
                    (tamano_de(&ahora) - tamano_de(&antes)).magnitude() < EPS,
                    "el prisma {i} cambio de tamano: la extension solo traslada"
                );
                assert!(
                    (centro_de(&ahora) - centro_de(&antes)).magnitude() > EPS,
                    "el prisma {i} esta en la seleccion y no se movio"
                );
            } else {
                assert_eq!(
                    antes, ahora,
                    "la pieza {i} cambio y no esta en la seleccion"
                );
            }
        }
    }

    #[test]
    fn el_macizo_sigue_mayoritario_y_compacto() {
        let apoyado = nivel_apoyado();
        let nuevo = nivel_ascenso_desde_grounded();

        let cimas = cimas_seleccionadas(&apoyado.scene);
        let piezas = indices_del_grupo(&nuevo, SpatialGroupId::Breakwater);
        let macizo: Vec<usize> = piezas[..PILARES]
            .iter()
            .copied()
            .filter(|i| !cimas.contains(i))
            .collect();

        assert!(
            macizo.len() >= MACIZO_MAYORITARIO,
            "el macizo se quedo en {} prismas de {PILARES}",
            macizo.len()
        );
        assert!(
            macizo.len() > cimas.len() * 2,
            "el macizo ({}) no es mayoritario frente a la extension ({})",
            macizo.len(),
            cimas.len()
        );

        // No se redistribuye: su huella sigue dentro de la que tenia en
        // grounded.
        let antes = huella(&apoyado.scene, &piezas[..PILARES]);
        let ahora = huella(&nuevo.scene, &macizo);

        assert!(
            ahora.min.x >= antes.min.x - EPS
                && ahora.max.x <= antes.max.x + EPS
                && ahora.min.z >= antes.min.z - EPS
                && ahora.max.z <= antes.max.z + EPS,
            "el macizo se salio de la huella que tenia en grounded"
        );

        // Y es una masa: cada prisma tiene otro a menos de dos anchos.
        let centro = |i: usize| centro_xz(&nuevo.scene.objects[i].primitive.bounds());
        let ancho = |i: usize| {
            let c = nuevo.scene.objects[i].primitive.bounds();

            c.max.x - c.min.x
        };
        let ancho_medio = macizo.iter().map(|&i| ancho(i)).sum::<f32>() / macizo.len() as f32;

        let vecino = |i: usize| {
            macizo
                .iter()
                .filter(|&&j| j != i)
                .map(|&j| {
                    let (ax, az) = centro(i);
                    let (bx, bz) = centro(j);

                    ((ax - bx).powi(2) + (az - bz).powi(2)).sqrt()
                })
                .fold(f32::MAX, f32::min)
        };
        let medio = macizo.iter().map(|&i| vecino(i)).sum::<f32>() / macizo.len() as f32;

        assert!(
            medio <= ancho_medio * 2.0,
            "el vecino medio del macizo esta a {medio:.2} y el ancho medio es {ancho_medio:.2}"
        );
    }

    #[test]
    fn cada_prisma_de_r01_sigue_apoyado_tras_la_extension() {
        let nuevo = nivel_ascenso_desde_grounded();
        let piezas = indices_del_grupo(&nuevo, SpatialGroupId::Breakwater);

        for &i in &piezas[..PILARES] {
            let caja = nuevo.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&nuevo.scene, i)
                .unwrap_or_else(|| panic!("el prisma {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo,
                "el prisma {i} arranca en {:.3} y su pedestal llega a {techo:.3}: flota",
                caja.min.y
            );
            assert!(
                caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "el prisma {i} arranca en {:.3} sobre un pedestal de {techo:.3}: enterrado",
                caja.min.y
            );
        }
    }

    #[test]
    fn la_extension_va_a_praderas_y_se_detiene_antes() {
        let apoyado = nivel_apoyado();
        let nuevo = nivel_ascenso_desde_grounded();

        let cimas = cimas_seleccionadas(&apoyado.scene);
        let praderas = huella_del_grupo(&nuevo.scene, SpatialGroupId::Meadows);
        let aguas = huella_del_grupo(&nuevo.scene, SpatialGroupId::FlyingWaters);

        let mut orden = cimas.clone();
        orden.sort_by(|a, b| {
            centro_xz(&nuevo.scene.objects[*b].primitive.bounds())
                .1
                .partial_cmp(&centro_xz(&nuevo.scene.objects[*a].primitive.bounds()).1)
                .expect("no hay NaN")
        });

        let ultimo = nuevo.scene.objects[*orden.last().expect("hay extension")]
            .primitive
            .bounds();
        let claro = ultimo.min.z - praderas.max.z;

        assert!(
            claro >= CLARO_MINIMO,
            "el ultimo prisma queda a {claro:.3} de Praderas"
        );
        assert!(
            claro <= CLARO_MINIMO * 3.0,
            "el ultimo prisma queda a {claro:.3}: la extension no llega"
        );

        let centro_ultimo = centro_xz(&ultimo).0;

        assert!(
            centro_ultimo < (praderas.min.x + praderas.max.x) * 0.5,
            "la extension no llega por la izquierda de Praderas"
        );
        assert!(
            centro_ultimo < aguas.min.x,
            "la extension se fue hacia Aguas Voladoras"
        );
    }

    #[test]
    fn la_extension_es_irregular_y_no_un_puente() {
        let apoyado = nivel_apoyado();
        let nuevo = nivel_ascenso_desde_grounded();

        let cimas = cimas_seleccionadas(&apoyado.scene);
        let mut orden = cimas.clone();
        orden.sort_by(|a, b| {
            centro_xz(&nuevo.scene.objects[*b].primitive.bounds())
                .1
                .partial_cmp(&centro_xz(&nuevo.scene.objects[*a].primitive.bounds()).1)
                .expect("no hay NaN")
        });

        let caja = |i: usize| nuevo.scene.objects[i].primitive.bounds();

        // Separaciones desiguales: algunos prismas van sueltos.
        let huecos: Vec<f32> = orden
            .windows(2)
            .map(|par| {
                let a = caja(par[0]);
                let b = caja(par[1]);

                separacion_xz(&a, &b)
            })
            .collect();

        let mayor = huecos.iter().copied().fold(f32::MIN, f32::max);
        let menor = huecos.iter().copied().fold(f32::MAX, f32::min);

        assert!(
            mayor >= VACIO_MINIMO,
            "el mayor hueco de la extension mide {mayor:.2}: es un puente"
        );
        assert!(
            mayor >= menor + VACIO_MINIMO,
            "los huecos van de {menor:.2} a {mayor:.2}: reparto regular"
        );

        // Y no van en fila: la nube no se ajusta a una recta.
        let n = orden.len() as f32;
        let xs: Vec<f32> = orden.iter().map(|&i| centro_xz(&caja(i)).0).collect();
        let zs: Vec<f32> = orden.iter().map(|&i| -centro_xz(&caja(i)).1).collect();
        let media_x = xs.iter().sum::<f32>() / n;
        let media_z = zs.iter().sum::<f32>() / n;
        let sxz: f32 = xs
            .iter()
            .zip(&zs)
            .map(|(x, z)| (x - media_x) * (z - media_z))
            .sum();
        let szz: f32 = zs.iter().map(|z| (z - media_z) * (z - media_z)).sum();
        let pendiente = sxz / szz;
        let residuo: f32 = (xs
            .iter()
            .zip(&zs)
            .map(|(x, z)| {
                let e = x - (media_x + pendiente * (z - media_z));

                e * e
            })
            .sum::<f32>()
            / n)
            .sqrt();

        assert!(
            residuo >= 0.30,
            "la extension se ajusta a una recta con residuo {residuo:.3}"
        );

        // Los techos no suben en escalon: el terreno bajo cada prisma manda,
        // y eso los desordena.
        let techos: Vec<f32> = orden.iter().map(|&i| caja(i).max.y).collect();
        let inversiones = techos.windows(2).filter(|par| par[1] < par[0]).count();

        assert!(
            inversiones >= 1 && inversiones < techos.len() - 1,
            "los techos de la extension van en {inversiones} bajadas: es una escalera regular"
        );
    }

    #[test]
    fn la_extension_no_toca_praderas_ni_aguas() {
        let nuevo = nivel_ascenso_desde_grounded();

        let huellas = [
            ("Praderas", SpatialGroupId::Meadows),
            ("Rompeolas", SpatialGroupId::Breakwater),
            ("Aguas", SpatialGroupId::FlyingWaters),
        ];

        for a in 0..huellas.len() {
            for b in a + 1..huellas.len() {
                let claro = separacion_xz(
                    &huella_del_grupo(&nuevo.scene, huellas[a].1),
                    &huella_del_grupo(&nuevo.scene, huellas[b].1),
                );

                assert!(
                    claro >= CLARO_MINIMO,
                    "{} y {} dejan {claro:.3} de claro",
                    huellas[a].0,
                    huellas[b].0
                );
            }
        }

        let praderas = huella_del_grupo(&nuevo.scene, SpatialGroupId::Meadows);
        let piezas = indices_del_grupo(&nuevo, SpatialGroupId::Breakwater);

        for &i in &piezas[..PILARES] {
            let caja = nuevo.scene.objects[i].primitive.bounds();

            assert!(
                separacion_xz(&caja, &praderas) >= CLARO_MINIMO,
                "el prisma {i} se metio a {:.3} de Praderas",
                separacion_xz(&caja, &praderas)
            );
        }
    }

    #[test]
    fn el_ascenso_desde_grounded_es_determinista() {
        let uno = nivel_ascenso_desde_grounded();
        let dos = nivel_ascenso_desde_grounded();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
        assert_eq!(
            uno.anchors.hero_camera_anchor,
            dos.anchors.hero_camera_anchor
        );
    }

    #[test]
    fn la_escala_del_ascenso_desde_grounded_se_deriva_de_la_escena() {
        let nuevo = nivel_ascenso_desde_grounded();

        assert_eq!(
            nuevo.scale.scene_radius,
            measure_scene_radius(&nuevo.scene, nuevo.anchors.orbit_center),
            "scene_radius no se volvio a medir"
        );
        assert_eq!(
            nuevo.scale.orbit_radius,
            derive_orbit_radius(nuevo.scale.scene_radius, nuevo.scale.monolith_height),
            "orbit_radius no corresponde con scene_radius y monolith_height"
        );
        assert_eq!(
            nuevo.anchors.hero_camera_anchor,
            eye_at_yaw(
                nuevo.anchors.orbit_center,
                nuevo.scale.orbit_radius,
                HERO_YAW_DEGREES
            ),
            "la toma hero no se derivo de la escala nueva"
        );
    }

    #[test]
    fn los_diez_niveles_previos_no_cambian_al_construir_la_extension() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
        ];

        let _nuevo = nivel_ascenso_desde_grounded();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }
    }

    // ------------------------------------------------------------------
    // Relleno de la cara −Z: `backfill`
    // ------------------------------------------------------------------

    /// Cuántos prismas puede llevarse el relleno, como mucho.
    const TRASEROS_MAXIMO: usize = 6;

    /// Cuánta abscisa tiene que ganar la cara `−Z` para que el relleno
    /// cuente como relleno.
    const COBERTURA_MINIMA: f32 = 2.00;

    #[test]
    fn el_relleno_trasero_conserva_el_conteo_del_nivel() {
        let relleno = nivel_relleno_trasero();

        assert_eq!(
            relleno.scene.objects.len(),
            OBJETOS,
            "el relleno cambio el conteo del nivel"
        );
    }

    #[test]
    fn la_seleccion_trasera_es_por_profundidad_y_es_pequena() {
        let previo = nivel_ascenso_desde_grounded();
        let traseros = traseros_seleccionados(&previo.scene);

        assert!(
            !traseros.is_empty() && traseros.len() <= TRASEROS_MAXIMO,
            "el relleno mueve {} prismas y el maximo es {TRASEROS_MAXIMO}",
            traseros.len()
        );

        // Elegidos por fondo medido: los que quedan mas lejos de la cara que
        // hay que poblar. Cualquiera de los elegidos esta mas atras que
        // cualquiera de los que se quedan.
        let piezas = indices_del_grupo(&previo, SpatialGroupId::Breakwater);
        let fondo = |i: usize| centro_xz(&previo.scene.objects[i].primitive.bounds()).1;

        let menos_hondo = traseros.iter().map(|&i| fondo(i)).fold(f32::MAX, f32::min);
        let mas_hondo = piezas[..PILARES]
            .iter()
            .filter(|i| !traseros.contains(i))
            .map(|&i| fondo(i))
            .fold(f32::MIN, f32::max);

        // Con `>=` y no con `>`: la formación es una retícula y los siete
        // prismas de la última fila comparten `z` **exacto**. Exigir orden
        // estricto sería exigir que la geometría no tuviera filas. Entre
        // empatados decide el índice, que es estable y determinista.
        assert!(
            menos_hondo >= mas_hondo,
            "el elegido mas adelantado esta en z {menos_hondo:.3} y el mas atrasado que se \
             queda en {mas_hondo:.3}: la seleccion no es por fondo"
        );

        // Y no pisa la extension: las dos subselecciones son disjuntas.
        let apoyado = nivel_apoyado();

        for &i in &cimas_seleccionadas(&apoyado.scene) {
            assert!(
                !traseros.contains(&i),
                "el prisma {i} esta en la extension y tambien en el relleno"
            );
        }
    }

    #[test]
    fn solo_una_subseleccion_de_r01_cambia_en_el_relleno() {
        let previo = nivel_ascenso_desde_grounded();
        let relleno = nivel_relleno_trasero();

        let traseros = traseros_seleccionados(&previo.scene);

        for i in 0..OBJETOS {
            let a = &previo.scene.objects[i];
            let b = &relleno.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            let antes = a.primitive.bounds();
            let ahora = b.primitive.bounds();

            if traseros.contains(&i) {
                assert!(
                    (tamano_de(&ahora) - tamano_de(&antes)).magnitude() < EPS,
                    "el prisma {i} cambio de tamano: el relleno solo traslada"
                );
                assert!(
                    (centro_de(&ahora) - centro_de(&antes)).magnitude() > EPS,
                    "el prisma {i} esta en la seleccion y no se movio"
                );
            } else {
                assert_eq!(
                    antes, ahora,
                    "la pieza {i} cambio y no esta en la seleccion"
                );
            }
        }
    }

    #[test]
    fn la_cara_menos_z_gana_cobertura() {
        // Lo que pidio la aclaracion: poblar la franja de `−Z` del
        // Rompeolas. Se mide como la abscisa que cubren los prismas que
        // pisan esa franja, antes y despues.
        let previo = nivel_ascenso_desde_grounded();
        let relleno = nivel_relleno_trasero();

        let franja = franja_trasera(&relleno.scene);
        let antes = cobertura_de_la_franja(&previo.scene, franja);
        let ahora = cobertura_de_la_franja(&relleno.scene, franja);

        assert!(
            ahora >= antes + COBERTURA_MINIMA,
            "la cara -Z cubria {antes:.2} y ahora cubre {ahora:.2}: no se lleno"
        );
    }

    #[test]
    fn el_relleno_se_queda_dentro_de_la_parcela() {
        // Quedarse en la losa es lo que garantiza a la vez el apoyo y el
        // claro: la parcela ya esta a mas de un claro de Praderas.
        let previo = nivel_ascenso_desde_grounded();
        let relleno = nivel_relleno_trasero();

        let traseros = traseros_seleccionados(&previo.scene);
        let parcela = relleno.scene.objects[masa_pedestal(&relleno.scene)]
            .primitive
            .bounds();

        for &i in &traseros {
            let caja = relleno.scene.objects[i].primitive.bounds();

            assert!(
                caja.min.x >= parcela.min.x
                    && caja.max.x <= parcela.max.x
                    && caja.min.z >= parcela.min.z
                    && caja.max.z <= parcela.max.z,
                "el prisma {i} se salio de la parcela del Rompeolas"
            );
        }
    }

    #[test]
    fn el_relleno_no_cruza_el_claro_ni_va_hacia_aguas() {
        let relleno = nivel_relleno_trasero();

        let huellas = [
            ("Praderas", SpatialGroupId::Meadows),
            ("Rompeolas", SpatialGroupId::Breakwater),
            ("Aguas", SpatialGroupId::FlyingWaters),
        ];

        for a in 0..huellas.len() {
            for b in a + 1..huellas.len() {
                let claro = separacion_xz(
                    &huella_del_grupo(&relleno.scene, huellas[a].1),
                    &huella_del_grupo(&relleno.scene, huellas[b].1),
                );

                assert!(
                    claro >= CLARO_MINIMO,
                    "{} y {} dejan {claro:.3} de claro",
                    huellas[a].0,
                    huellas[b].0
                );
            }
        }

        let praderas = huella_del_grupo(&relleno.scene, SpatialGroupId::Meadows);
        let aguas = huella_del_grupo(&relleno.scene, SpatialGroupId::FlyingWaters);
        let piezas = indices_del_grupo(&relleno, SpatialGroupId::Breakwater);

        for &i in &piezas[..PILARES] {
            let caja = relleno.scene.objects[i].primitive.bounds();

            assert!(
                separacion_xz(&caja, &praderas) >= CLARO_MINIMO,
                "el prisma {i} se metio a {:.3} de Praderas",
                separacion_xz(&caja, &praderas)
            );
            assert!(
                caja.max.x < aguas.min.x,
                "el prisma {i} avanzo hacia Aguas Voladoras"
            );
        }
    }

    #[test]
    fn cada_prisma_de_r01_sigue_apoyado_en_el_relleno() {
        let relleno = nivel_relleno_trasero();
        let piezas = indices_del_grupo(&relleno, SpatialGroupId::Breakwater);

        for &i in &piezas[..PILARES] {
            let caja = relleno.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&relleno.scene, i)
                .unwrap_or_else(|| panic!("el prisma {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo,
                "el prisma {i} arranca en {:.3} y su pedestal llega a {techo:.3}: flota",
                caja.min.y
            );
            assert!(
                caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "el prisma {i} arranca en {:.3} sobre un pedestal de {techo:.3}: enterrado",
                caja.min.y
            );
        }
    }

    #[test]
    fn praderas_aguas_monolito_y_volumen_siguen_exactos_en_el_relleno() {
        let previo = nivel_ascenso_desde_grounded();
        let relleno = nivel_relleno_trasero();

        for grupo in [
            SpatialGroupId::Meadows,
            SpatialGroupId::FlyingWaters,
            SpatialGroupId::Monolith,
            SpatialGroupId::Global,
            SpatialGroupId::ContinentBackground,
        ] {
            for &i in &indices_del_grupo(&relleno, grupo) {
                assert_eq!(
                    previo.scene.objects[i].primitive.bounds(),
                    relleno.scene.objects[i].primitive.bounds(),
                    "la pieza {i} de {grupo:?} se movio"
                );
            }
        }

        let a01 = volumen_de_agua(&relleno.scene);

        assert_eq!(volumen_de_agua(&previo.scene), a01, "A-01 cambio de indice");
    }

    #[test]
    fn el_nivel_relleno_trasero_es_determinista() {
        let uno = nivel_relleno_trasero();
        let dos = nivel_relleno_trasero();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
        assert_eq!(
            uno.anchors.hero_camera_anchor,
            dos.anchors.hero_camera_anchor
        );
    }

    #[test]
    fn la_escala_del_relleno_se_deriva_de_la_escena() {
        let relleno = nivel_relleno_trasero();

        assert_eq!(
            relleno.scale.scene_radius,
            measure_scene_radius(&relleno.scene, relleno.anchors.orbit_center),
            "scene_radius no se volvio a medir"
        );
        assert_eq!(
            relleno.scale.orbit_radius,
            derive_orbit_radius(relleno.scale.scene_radius, relleno.scale.monolith_height),
            "orbit_radius no corresponde con scene_radius y monolith_height"
        );
        assert_eq!(
            relleno.anchors.hero_camera_anchor,
            eye_at_yaw(
                relleno.anchors.orbit_center,
                relleno.scale.orbit_radius,
                HERO_YAW_DEGREES
            ),
            "la toma hero no se derivo de la escala del relleno"
        );
    }

    #[test]
    fn los_once_niveles_previos_no_cambian_al_construir_el_relleno() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
        ];

        let _relleno = nivel_relleno_trasero();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }
    }

    // ------------------------------------------------------------------
    // Escalones tectónicos tras el Monolito: `monolith_stair`
    // ------------------------------------------------------------------

    /// Cuántos prismas puede llevarse la ruta, como mucho.
    const RUTA_MAXIMO: usize = 8;

    /// Cuántos prismas tienen que alcanzar la banda que queda por detrás de
    /// la cara trasera del Monolito.
    const TRASEROS_EN_BANDA: usize = 3;

    /// Qué parte de `R-01` sigue siendo macizo.
    const MACIZO_DE_LA_RUTA: usize = 20;

    #[test]
    fn el_nivel_escalera_monolito_conserva_el_conteo() {
        let ruta = nivel_escalera_monolito();

        assert_eq!(
            ruta.scene.objects.len(),
            OBJETOS,
            "la escalera del Monolito cambio el conteo del nivel"
        );
    }

    #[test]
    fn el_monolito_el_plinto_y_el_volumen_siguen_exactos_en_la_ruta() {
        let apoyado = nivel_apoyado();
        let ruta = nivel_escalera_monolito();

        for grupo in [
            SpatialGroupId::Monolith,
            SpatialGroupId::Global,
            SpatialGroupId::ContinentBackground,
            SpatialGroupId::Meadows,
            SpatialGroupId::FlyingWaters,
        ] {
            for &i in &indices_del_grupo(&ruta, grupo) {
                assert_eq!(
                    apoyado.scene.objects[i].primitive.bounds(),
                    ruta.scene.objects[i].primitive.bounds(),
                    "la pieza {i} de {grupo:?} se movio"
                );
            }
        }

        assert_eq!(
            volumen_de_agua(&apoyado.scene),
            volumen_de_agua(&ruta.scene),
            "A-01 cambio de indice"
        );
    }

    #[test]
    fn solo_una_subseleccion_de_r01_cambia_en_la_ruta() {
        let apoyado = nivel_apoyado();
        let ruta = nivel_escalera_monolito();

        let elegidos = prismas_de_la_ruta(&apoyado.scene);

        assert!(
            !elegidos.is_empty() && elegidos.len() <= RUTA_MAXIMO,
            "la ruta mueve {} prismas y el maximo es {RUTA_MAXIMO}",
            elegidos.len()
        );

        for i in 0..OBJETOS {
            let a = &apoyado.scene.objects[i];
            let b = &ruta.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            let antes = a.primitive.bounds();
            let ahora = b.primitive.bounds();

            if elegidos.contains(&i) {
                assert!(
                    (tamano_de(&ahora) - tamano_de(&antes)).magnitude() < EPS,
                    "el prisma {i} cambio de tamano: la ruta solo traslada"
                );
                assert!(
                    (centro_de(&ahora) - centro_de(&antes)).magnitude() > EPS,
                    "el prisma {i} esta en la ruta y no se movio"
                );
            } else {
                assert_eq!(antes, ahora, "la pieza {i} cambio y no esta en la ruta");
            }
        }
    }

    #[test]
    fn el_macizo_sigue_mayoritario_tras_la_ruta() {
        let apoyado = nivel_apoyado();
        let ruta = nivel_escalera_monolito();

        let elegidos = prismas_de_la_ruta(&apoyado.scene);
        let piezas = indices_del_grupo(&ruta, SpatialGroupId::Breakwater);
        let macizo: Vec<usize> = piezas[..PILARES]
            .iter()
            .copied()
            .filter(|i| !elegidos.contains(i))
            .collect();

        assert!(
            macizo.len() >= MACIZO_DE_LA_RUTA,
            "el macizo se quedo en {} prismas de {PILARES}",
            macizo.len()
        );
        assert!(
            macizo.len() > elegidos.len() * 2,
            "el macizo ({}) no es mayoritario frente a la ruta ({})",
            macizo.len(),
            elegidos.len()
        );

        // Sigue donde estaba: no se redistribuye.
        let antes = huella(&apoyado.scene, &piezas[..PILARES]);
        let ahora = huella(&ruta.scene, &macizo);

        assert!(
            ahora.min.x >= antes.min.x - EPS
                && ahora.max.x <= antes.max.x + EPS
                && ahora.min.z >= antes.min.z - EPS
                && ahora.max.z <= antes.max.z + EPS,
            "el macizo se salio de la huella que tenia en grounded"
        );

        // Y es una masa: cada prisma tiene otro a menos de dos anchos.
        let centro = |i: usize| centro_xz(&ruta.scene.objects[i].primitive.bounds());
        let ancho_medio = macizo
            .iter()
            .map(|&i| {
                let c = ruta.scene.objects[i].primitive.bounds();

                c.max.x - c.min.x
            })
            .sum::<f32>()
            / macizo.len() as f32;
        let vecino = |i: usize| {
            macizo
                .iter()
                .filter(|&&j| j != i)
                .map(|&j| {
                    let (ax, az) = centro(i);
                    let (bx, bz) = centro(j);

                    ((ax - bx).powi(2) + (az - bz).powi(2)).sqrt()
                })
                .fold(f32::MAX, f32::min)
        };
        let medio = macizo.iter().map(|&i| vecino(i)).sum::<f32>() / macizo.len() as f32;

        assert!(
            medio <= ancho_medio * 2.0,
            "el vecino medio del macizo esta a {medio:.2} y el ancho medio es {ancho_medio:.2}"
        );
    }

    #[test]
    fn la_ruta_alcanza_la_banda_trasera_sin_tocar_el_monolito() {
        let apoyado = nivel_apoyado();
        let ruta = nivel_escalera_monolito();

        let elegidos = prismas_de_la_ruta(&apoyado.scene);
        let monolito = huella_del_grupo(&ruta.scene, SpatialGroupId::Monolith);

        // Ninguno corta la caja del Monolito, en ningun eje.
        for &i in &elegidos {
            let caja = ruta.scene.objects[i].primitive.bounds();

            let cortan = caja.min.x < monolito.max.x
                && caja.max.x > monolito.min.x
                && caja.min.y < monolito.max.y
                && caja.max.y > monolito.min.y
                && caja.min.z < monolito.max.z
                && caja.max.z > monolito.min.z;

            assert!(!cortan, "el prisma {i} se mete en la caja del Monolito");
            assert!(
                separacion_xz(&caja, &monolito) > 0.0,
                "el prisma {i} roza el Monolito en planta"
            );
        }

        // Y varios llegan **por detras** de su cara trasera.
        let detras = elegidos
            .iter()
            .filter(|&&i| ruta.scene.objects[i].primitive.bounds().min.z < monolito.min.z)
            .count();

        assert!(
            detras >= TRASEROS_EN_BANDA,
            "solo {detras} prismas pasan de la cara trasera del Monolito, y hacen falta \
             {TRASEROS_EN_BANDA}"
        );

        // Lo rodean de cerca: el que mas se adentra queda a menos de tres
        // unidades de su flanco, no perdido en el vacio.
        let mas_hondo = elegidos
            .iter()
            .map(|&i| ruta.scene.objects[i].primitive.bounds())
            .fold(None::<Aabb>, |mejor, caja| match mejor {
                Some(m) if m.min.z <= caja.min.z => Some(m),
                _ => Some(caja),
            })
            .expect("hay ruta");

        assert!(
            monolito.min.x - mas_hondo.max.x <= 3.0,
            "el prisma mas hondo queda a {:.2} del flanco del Monolito: no lo rodea",
            monolito.min.x - mas_hondo.max.x
        );
    }

    #[test]
    fn la_ruta_respeta_el_claro_y_no_va_a_aguas() {
        let ruta = nivel_escalera_monolito();

        let huellas = [
            ("Praderas", SpatialGroupId::Meadows),
            ("Rompeolas", SpatialGroupId::Breakwater),
            ("Aguas", SpatialGroupId::FlyingWaters),
        ];

        for a in 0..huellas.len() {
            for b in a + 1..huellas.len() {
                let claro = separacion_xz(
                    &huella_del_grupo(&ruta.scene, huellas[a].1),
                    &huella_del_grupo(&ruta.scene, huellas[b].1),
                );

                assert!(
                    claro >= CLARO_MINIMO,
                    "{} y {} dejan {claro:.3} de claro",
                    huellas[a].0,
                    huellas[b].0
                );
            }
        }

        let praderas = huella_del_grupo(&ruta.scene, SpatialGroupId::Meadows);
        let aguas = huella_del_grupo(&ruta.scene, SpatialGroupId::FlyingWaters);
        let piezas = indices_del_grupo(&ruta, SpatialGroupId::Breakwater);

        for &i in &piezas[..PILARES] {
            let caja = ruta.scene.objects[i].primitive.bounds();

            assert!(
                separacion_xz(&caja, &praderas) >= CLARO_MINIMO,
                "el prisma {i} se metio a {:.3} de Praderas",
                separacion_xz(&caja, &praderas)
            );
            assert!(
                caja.max.x < aguas.min.x,
                "el prisma {i} avanzo hacia Aguas Voladoras"
            );
        }
    }

    #[test]
    fn cada_prisma_sigue_apoyado_en_la_ruta() {
        let ruta = nivel_escalera_monolito();
        let piezas = indices_del_grupo(&ruta, SpatialGroupId::Breakwater);

        for &i in &piezas[..PILARES] {
            let caja = ruta.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&ruta.scene, i)
                .unwrap_or_else(|| panic!("el prisma {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo,
                "el prisma {i} arranca en {:.3} y su pedestal llega a {techo:.3}: flota",
                caja.min.y
            );
            assert!(
                caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "el prisma {i} arranca en {:.3} sobre un pedestal de {techo:.3}: enterrado",
                caja.min.y
            );
        }
    }

    #[test]
    fn la_ruta_es_irregular_y_escalonada() {
        let apoyado = nivel_apoyado();
        let ruta = nivel_escalera_monolito();

        let elegidos = prismas_de_la_ruta(&apoyado.scene);
        let mut orden = elegidos.clone();
        orden.sort_by(|a, b| {
            centro_xz(&ruta.scene.objects[*b].primitive.bounds())
                .1
                .partial_cmp(&centro_xz(&ruta.scene.objects[*a].primitive.bounds()).1)
                .expect("no hay NaN")
        });

        let caja = |i: usize| ruta.scene.objects[i].primitive.bounds();

        // Separaciones desiguales.
        let huecos: Vec<f32> = orden
            .windows(2)
            .map(|par| separacion_xz(&caja(par[0]), &caja(par[1])))
            .collect();
        let mayor = huecos.iter().copied().fold(f32::MIN, f32::max);
        let menor = huecos.iter().copied().fold(f32::MAX, f32::min);

        assert!(
            mayor >= menor + 0.60,
            "los huecos van de {menor:.2} a {mayor:.2}: reparto regular"
        );

        // Escalones: los techos no bajan en orden, porque el suelo bajo cada
        // prisma cambia de cota.
        let techos: Vec<f32> = orden.iter().map(|&i| caja(i).max.y).collect();
        let cotas: Vec<f32> = orden.iter().map(|&i| caja(i).min.y).collect();

        let distintas = cotas
            .iter()
            .map(|c| (c * 100.0).round() as i64)
            .collect::<std::collections::BTreeSet<_>>()
            .len();

        assert!(
            distintas >= 2,
            "los {} prismas de la ruta arrancan todos a la misma cota: no hay escalones",
            cotas.len()
        );

        let inversiones = techos.windows(2).filter(|par| par[1] > par[0]).count();

        assert!(
            inversiones >= 1 && inversiones < techos.len() - 1,
            "los techos de la ruta van en {inversiones} subidas: es una rampa"
        );
    }

    #[test]
    fn el_nivel_escalera_monolito_es_determinista() {
        let uno = nivel_escalera_monolito();
        let dos = nivel_escalera_monolito();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
    }

    #[test]
    fn la_escala_de_la_escalera_monolito_se_deriva_de_la_escena() {
        let ruta = nivel_escalera_monolito();

        assert_eq!(
            ruta.scale.scene_radius,
            measure_scene_radius(&ruta.scene, ruta.anchors.orbit_center),
            "scene_radius no se volvio a medir"
        );
        assert_eq!(
            ruta.scale.orbit_radius,
            derive_orbit_radius(ruta.scale.scene_radius, ruta.scale.monolith_height),
            "orbit_radius no corresponde con scene_radius y monolith_height"
        );
        assert_eq!(
            ruta.anchors.hero_camera_anchor,
            eye_at_yaw(
                ruta.anchors.orbit_center,
                ruta.scale.orbit_radius,
                HERO_YAW_DEGREES
            ),
            "la toma hero no se derivo de la escala de la ruta"
        );
    }

    #[test]
    fn los_doce_niveles_previos_no_cambian_al_construir_la_ruta() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
        ];

        let _ruta = nivel_escalera_monolito();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }
    }

    // ------------------------------------------------------------------
    // Rompeolas de borde: `edge_breakwater`
    // ------------------------------------------------------------------

    /// En cuántas bandas se parte el corredor para comprobar que la
    /// formación no lo deja a trozos.
    const BANDAS_DEL_BORDE: usize = 6;

    /// A qué distancia del Monolito se considera que un prisma está «cerca».
    const CERCA_DEL_MONOLITO: f32 = 2.50;

    /// Cuánto tienen que bajar de media los prismas de esa zona.
    const DESCENSO_MINIMO: f32 = 0.50;

    #[test]
    fn el_rompeolas_de_borde_conserva_el_conteo() {
        let borde = nivel_rompeolas_de_borde();

        assert_eq!(
            borde.scene.objects.len(),
            OBJETOS,
            "el rompeolas de borde cambio el conteo del nivel"
        );
    }

    #[test]
    fn el_monolito_el_plinto_y_el_volumen_siguen_exactos_en_el_borde() {
        let apoyado = nivel_apoyado();
        let borde = nivel_rompeolas_de_borde();

        for grupo in [
            SpatialGroupId::Monolith,
            SpatialGroupId::Global,
            SpatialGroupId::ContinentBackground,
            SpatialGroupId::Meadows,
            SpatialGroupId::FlyingWaters,
        ] {
            for &i in &indices_del_grupo(&borde, grupo) {
                assert_eq!(
                    apoyado.scene.objects[i].primitive.bounds(),
                    borde.scene.objects[i].primitive.bounds(),
                    "la pieza {i} de {grupo:?} se movio"
                );
            }
        }

        assert_eq!(
            volumen_de_agua(&apoyado.scene),
            volumen_de_agua(&borde.scene),
            "A-01 cambio de indice"
        );
    }

    #[test]
    fn solo_los_veintiocho_pilares_cambian_en_el_borde() {
        let apoyado = nivel_apoyado();
        let borde = nivel_rompeolas_de_borde();

        let piezas = indices_del_grupo(&borde, SpatialGroupId::Breakwater);
        assert_eq!(piezas.len(), 38, "el Rompeolas no tiene 38 piezas");

        for i in 0..OBJETOS {
            let a = &apoyado.scene.objects[i];
            let b = &borde.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            let antes = a.primitive.bounds();
            let ahora = b.primitive.bounds();

            assert!(
                (tamano_de(&ahora) - tamano_de(&antes)).magnitude() < EPS,
                "la pieza {i} cambio de tamano: el borde solo traslada"
            );

            if antes != ahora {
                assert!(
                    piezas[..PILARES].contains(&i),
                    "la pieza {i} cambio y no es un pilar de R-01"
                );
            }
        }

        // `R-02`, `R-03` y la losa se quedan como estaban.
        for &i in &piezas[PILARES..] {
            assert_eq!(
                apoyado.scene.objects[i].primitive.bounds(),
                borde.scene.objects[i].primitive.bounds(),
                "la pieza {i} de R-02/R-03 se movio"
            );
        }
    }

    #[test]
    fn el_borde_va_de_aguas_al_claro_de_praderas() {
        let borde = nivel_rompeolas_de_borde();

        let pilares = &indices_del_grupo(&borde, SpatialGroupId::Breakwater)[..PILARES];
        let formacion = huella(&borde.scene, pilares);
        let aguas = huella_del_grupo(&borde.scene, SpatialGroupId::FlyingWaters);
        let praderas = huella_del_grupo(&borde.scene, SpatialGroupId::Meadows);

        // Arranca junto a Aguas: su borde derecho queda a poco mas de un
        // claro de la bahia.
        let a_la_bahia = aguas.min.x - formacion.max.x;

        assert!(
            a_la_bahia >= CLARO_MINIMO,
            "la formacion queda a {a_la_bahia:.2} de Aguas: se le mete encima"
        );
        assert!(
            a_la_bahia <= CLARO_MINIMO * 2.0,
            "la formacion queda a {a_la_bahia:.2} de Aguas: no arranca junto a ella"
        );

        // Y llega hasta el claro de Praderas.
        let a_la_meseta = formacion.min.z - praderas.max.z;

        assert!(
            a_la_meseta >= CLARO_MINIMO,
            "la formacion queda a {a_la_meseta:.2} de Praderas"
        );
        assert!(
            a_la_meseta <= CLARO_MINIMO * 2.0,
            "la formacion queda a {a_la_meseta:.2} de Praderas: no llega al claro"
        );
    }

    #[test]
    fn el_borde_no_deja_huecos_a_lo_largo_del_corredor() {
        // Macroforma continua: el corredor se parte en bandas de `z` y
        // ninguna puede quedarse vacia. Una formacion con un tramo en blanco
        // no es un borde, son dos montones.
        let borde = nivel_rompeolas_de_borde();

        let pilares = &indices_del_grupo(&borde, SpatialGroupId::Breakwater)[..PILARES];
        let formacion = huella(&borde.scene, pilares);
        let fondo = formacion.max.z - formacion.min.z;
        let paso = fondo / BANDAS_DEL_BORDE as f32;

        let mut por_banda = [0usize; BANDAS_DEL_BORDE];

        for &i in pilares {
            let caja = borde.scene.objects[i].primitive.bounds();

            for (b, cuenta) in por_banda.iter_mut().enumerate() {
                let desde = formacion.min.z + paso * b as f32;
                let hasta = desde + paso;

                if caja.min.z < hasta && caja.max.z > desde {
                    *cuenta += 1;
                }
            }
        }

        for (b, cuenta) in por_banda.iter().enumerate() {
            assert!(
                *cuenta > 0,
                "la banda {b} del corredor esta vacia: la formacion no es continua"
            );
        }

        // Y los racimos son irregulares: unas bandas llevan mucho mas que
        // otras. Un reparto parejo seria una reja.
        let menos = por_banda.iter().copied().min().expect("hay bandas");
        let mas = por_banda.iter().copied().max().expect("hay bandas");

        assert!(
            mas >= menos * 2,
            "las bandas llevan entre {menos} y {mas} prismas: reparto parejo"
        );
    }

    #[test]
    fn los_pilares_junto_al_monolito_bajan() {
        // Cerca del Monolito la formacion se rompe y baja para no competir
        // con el.
        let borde = nivel_rompeolas_de_borde();

        let pilares = &indices_del_grupo(&borde, SpatialGroupId::Breakwater)[..PILARES];
        let monolito = huella_del_grupo(&borde.scene, SpatialGroupId::Monolith);

        let mut cerca: Vec<f32> = Vec::new();
        let mut lejos: Vec<f32> = Vec::new();

        for &i in pilares {
            let caja = borde.scene.objects[i].primitive.bounds();

            if separacion_xz(&caja, &monolito) <= CERCA_DEL_MONOLITO {
                cerca.push(caja.max.y);
            } else {
                lejos.push(caja.max.y);
            }
        }

        assert!(
            cerca.len() >= 3,
            "solo {} prismas quedan cerca del Monolito",
            cerca.len()
        );
        assert!(!lejos.is_empty(), "no hay macizo lateral con que comparar");

        let media = |v: &[f32]| v.iter().sum::<f32>() / v.len() as f32;
        let (junto, lateral) = (media(&cerca), media(&lejos));

        assert!(
            junto + DESCENSO_MINIMO <= lateral,
            "los de cerca rematan en {junto:.2} y el macizo lateral en {lateral:.2}: no bajan"
        );
    }

    #[test]
    fn cada_pilar_del_borde_tiene_apoyo() {
        let borde = nivel_rompeolas_de_borde();
        let piezas = indices_del_grupo(&borde, SpatialGroupId::Breakwater);

        for &i in &piezas[..PILARES] {
            let caja = borde.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&borde.scene, i)
                .unwrap_or_else(|| panic!("el prisma {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo,
                "el prisma {i} arranca en {:.3} y su pedestal llega a {techo:.3}: flota",
                caja.min.y
            );
            assert!(
                caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "el prisma {i} arranca en {:.3} sobre un pedestal de {techo:.3}: enterrado",
                caja.min.y
            );
        }
    }

    #[test]
    fn el_borde_no_toca_praderas_ni_aguas_ni_el_monolito() {
        let borde = nivel_rompeolas_de_borde();

        let huellas = [
            ("Praderas", SpatialGroupId::Meadows),
            ("Rompeolas", SpatialGroupId::Breakwater),
            ("Aguas", SpatialGroupId::FlyingWaters),
        ];

        for a in 0..huellas.len() {
            for b in a + 1..huellas.len() {
                let claro = separacion_xz(
                    &huella_del_grupo(&borde.scene, huellas[a].1),
                    &huella_del_grupo(&borde.scene, huellas[b].1),
                );

                assert!(
                    claro >= CLARO_MINIMO,
                    "{} y {} dejan {claro:.3} de claro",
                    huellas[a].0,
                    huellas[b].0
                );
            }
        }

        let praderas = huella_del_grupo(&borde.scene, SpatialGroupId::Meadows);
        let aguas = huella_del_grupo(&borde.scene, SpatialGroupId::FlyingWaters);
        let monolito = huella_del_grupo(&borde.scene, SpatialGroupId::Monolith);
        let piezas = indices_del_grupo(&borde, SpatialGroupId::Breakwater);

        for &i in &piezas[..PILARES] {
            let caja = borde.scene.objects[i].primitive.bounds();

            assert!(
                separacion_xz(&caja, &praderas) >= CLARO_MINIMO,
                "el prisma {i} se metio a {:.3} de Praderas",
                separacion_xz(&caja, &praderas)
            );
            assert!(
                separacion_xz(&caja, &aguas) >= CLARO_MINIMO,
                "el prisma {i} se metio a {:.3} de Aguas",
                separacion_xz(&caja, &aguas)
            );
            assert!(
                separacion_xz(&caja, &monolito) > 0.0,
                "el prisma {i} corta el Monolito en planta"
            );
        }
    }

    #[test]
    fn el_nivel_rompeolas_de_borde_es_determinista() {
        let uno = nivel_rompeolas_de_borde();
        let dos = nivel_rompeolas_de_borde();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
    }

    #[test]
    fn la_escala_del_borde_se_deriva_de_la_escena() {
        let borde = nivel_rompeolas_de_borde();

        assert_eq!(
            borde.scale.scene_radius,
            measure_scene_radius(&borde.scene, borde.anchors.orbit_center),
            "scene_radius no se volvio a medir"
        );
        assert_eq!(
            borde.scale.orbit_radius,
            derive_orbit_radius(borde.scale.scene_radius, borde.scale.monolith_height),
            "orbit_radius no corresponde con scene_radius y monolith_height"
        );
        assert_eq!(
            borde.anchors.hero_camera_anchor,
            eye_at_yaw(
                borde.anchors.orbit_center,
                borde.scale.orbit_radius,
                HERO_YAW_DEGREES
            ),
            "la toma hero no se derivo de la escala del borde"
        );
    }

    #[test]
    fn los_trece_niveles_previos_no_cambian_al_construir_el_borde() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
        ];

        let _borde = nivel_rompeolas_de_borde();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }
    }

    // ------------------------------------------------------------------
    // Terraza verde de borde: `green_shelf`
    // ------------------------------------------------------------------

    #[test]
    fn la_terraza_verde_conserva_el_conteo() {
        let terraza = nivel_terraza_verde_de_borde();

        assert_eq!(
            terraza.scene.objects.len(),
            OBJETOS,
            "la terraza cambio el conteo del nivel"
        );
    }

    #[test]
    fn los_treinta_y_ocho_prismas_quedan_exactos_en_la_terraza() {
        // `R-01`, `R-02` y `R-03` no se tocan: esto no redistribuye nada, sólo
        // le da suelo a lo que ya hay.
        let apoyado = nivel_apoyado();
        let terraza = nivel_terraza_verde_de_borde();

        for &i in &indices_del_grupo(&terraza, SpatialGroupId::Breakwater) {
            assert_eq!(
                apoyado.scene.objects[i].primitive.bounds(),
                terraza.scene.objects[i].primitive.bounds(),
                "la pieza {i} del Rompeolas se movio"
            );
        }
    }

    #[test]
    fn solo_la_losa_verde_cambia_en_la_terraza() {
        let apoyado = nivel_apoyado();
        let terraza = nivel_terraza_verde_de_borde();

        let losa = masa_pedestal(&apoyado.scene);

        assert_eq!(
            masa_pedestal(&terraza.scene),
            losa,
            "la masa de soporte dejo de ser la misma"
        );

        for i in 0..OBJETOS {
            let a = &apoyado.scene.objects[i];
            let b = &terraza.scene.objects[i];

            // Sigue siendo la misma pieza: mismo tipo, mismos materiales,
            // mismo grupo espacial y de revelacion. Solo cambia su caja.
            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            if i != losa {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "la pieza {i} cambio y no es la losa"
                );
            }
        }

        assert_eq!(
            terraza.scene.objects[losa].spatial_group,
            SpatialGroupId::ContinentBackground,
            "la losa dejo de ser una masa de G-02"
        );
    }

    #[test]
    fn la_terraza_crece_y_sigue_cubriendo_el_rompeolas() {
        let apoyado = nivel_apoyado();
        let terraza = nivel_terraza_verde_de_borde();

        let losa = masa_pedestal(&terraza.scene);
        let antes = apoyado.scene.objects[losa].primitive.bounds();
        let ahora = terraza.scene.objects[losa].primitive.bounds();

        // Se prolonga por el borde: gana fondo y no pierde nada.
        assert!(
            ahora.min.z < antes.min.z && ahora.max.z > antes.max.z,
            "la terraza no se prolongo: z [{:.2},{:.2}] -> [{:.2},{:.2}]",
            antes.min.z,
            antes.max.z,
            ahora.min.z,
            ahora.max.z
        );
        assert!(
            ahora.min.x <= antes.min.x + EPS && ahora.max.x >= antes.max.x - EPS,
            "la terraza se estrecho"
        );
        assert!(
            (ahora.min.y - antes.min.y).abs() < EPS && (ahora.max.y - antes.max.y).abs() < EPS,
            "la terraza cambio de espesor"
        );

        // Y cubre el Rompeolas entero en planta: es su pedestal.
        let rompeolas = huella_del_grupo(&terraza.scene, SpatialGroupId::Breakwater);

        assert!(
            ahora.min.x <= rompeolas.min.x
                && ahora.max.x >= rompeolas.max.x
                && ahora.min.z <= rompeolas.min.z
                && ahora.max.z >= rompeolas.max.z,
            "la terraza dejo de cubrir el Rompeolas"
        );
    }

    #[test]
    fn la_terraza_no_invade_nada() {
        let terraza = nivel_terraza_verde_de_borde();

        let losa = masa_pedestal(&terraza.scene);
        let caja = terraza.scene.objects[losa].primitive.bounds();
        let plinto = huella_del_grupo(&terraza.scene, SpatialGroupId::Global);

        // Dentro del plinto.
        assert!(
            caja.min.x >= plinto.min.x
                && caja.max.x <= plinto.max.x
                && caja.min.z >= plinto.min.z
                && caja.max.z <= plinto.max.z,
            "la terraza se sale del plinto"
        );

        // Sin cortar el Monolito, Praderas ni Aguas.
        for (nombre, grupo) in [
            ("el Monolito", SpatialGroupId::Monolith),
            ("Praderas", SpatialGroupId::Meadows),
            ("Aguas", SpatialGroupId::FlyingWaters),
        ] {
            let otra = huella_del_grupo(&terraza.scene, grupo);

            let cortan = caja.min.x < otra.max.x
                && caja.max.x > otra.min.x
                && caja.min.y < otra.max.y
                && caja.max.y > otra.min.y
                && caja.min.z < otra.max.z
                && caja.max.z > otra.min.z;

            assert!(!cortan, "la terraza corta {nombre}");
        }

        // Y a Praderas le deja el claro entero: la terraza no llega ni de
        // lejos, porque el Monolito la para mucho antes.
        let praderas = huella_del_grupo(&terraza.scene, SpatialGroupId::Meadows);

        assert!(
            caja.min.z - praderas.max.z >= CLARO_MINIMO,
            "la terraza deja {:.2} a Praderas",
            caja.min.z - praderas.max.z
        );
    }

    #[test]
    fn los_claros_entre_regiones_siguen_intactos_con_la_terraza() {
        let terraza = nivel_terraza_verde_de_borde();

        let huellas = [
            ("Praderas", SpatialGroupId::Meadows),
            ("Rompeolas", SpatialGroupId::Breakwater),
            ("Aguas", SpatialGroupId::FlyingWaters),
        ];

        for a in 0..huellas.len() {
            for b in a + 1..huellas.len() {
                let claro = separacion_xz(
                    &huella_del_grupo(&terraza.scene, huellas[a].1),
                    &huella_del_grupo(&terraza.scene, huellas[b].1),
                );

                assert!(
                    claro >= CLARO_MINIMO,
                    "{} y {} dejan {claro:.3} de claro",
                    huellas[a].0,
                    huellas[b].0
                );
            }
        }
    }

    #[test]
    fn cada_prisma_sigue_apoyado_sobre_la_terraza() {
        let terraza = nivel_terraza_verde_de_borde();
        let piezas = indices_del_grupo(&terraza, SpatialGroupId::Breakwater);

        for &i in &piezas[..PILARES] {
            let caja = terraza.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&terraza.scene, i)
                .unwrap_or_else(|| panic!("el prisma {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo,
                "el prisma {i} arranca en {:.3} y su pedestal llega a {techo:.3}: flota",
                caja.min.y
            );
            assert!(
                caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "el prisma {i} arranca en {:.3} sobre un pedestal de {techo:.3}: enterrado",
                caja.min.y
            );
        }
    }

    #[test]
    fn el_monolito_es_quien_para_la_terraza() {
        // El hallazgo, escrito como test: la terraza tiene que cubrir el
        // Rompeolas, y el Rompeolas llega hasta `x = 0.58`, dentro de la
        // banda del Monolito. Una caja unica que cubra eso no puede bajar de
        // la cara delantera del Monolito sin tragarselo. Si algun dia el
        // Monolito o el Rompeolas cambian de sitio, esto avisa.
        let terraza = nivel_terraza_verde_de_borde();

        let losa = masa_pedestal(&terraza.scene);
        let caja = terraza.scene.objects[losa].primitive.bounds();
        let monolito = huella_del_grupo(&terraza.scene, SpatialGroupId::Monolith);
        let rompeolas = huella_del_grupo(&terraza.scene, SpatialGroupId::Breakwater);

        assert!(
            rompeolas.max.x > monolito.min.x,
            "el Rompeolas ya no entra en la banda del Monolito: la terraza podria seguir"
        );
        assert!(
            caja.min.z > monolito.max.z,
            "la terraza paso de la cara delantera del Monolito"
        );
        assert!(
            caja.min.z - monolito.max.z < CLARO_MINIMO,
            "la terraza se quedo a {:.2} del Monolito: no llego a su tope",
            caja.min.z - monolito.max.z
        );
    }

    #[test]
    fn la_terraza_verde_es_determinista() {
        let uno = nivel_terraza_verde_de_borde();
        let dos = nivel_terraza_verde_de_borde();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
    }

    #[test]
    fn la_escala_de_la_terraza_se_deriva_de_la_escena() {
        let terraza = nivel_terraza_verde_de_borde();

        assert_eq!(
            terraza.scale.scene_radius,
            measure_scene_radius(&terraza.scene, terraza.anchors.orbit_center),
            "scene_radius no se volvio a medir"
        );
        assert_eq!(
            terraza.scale.orbit_radius,
            derive_orbit_radius(terraza.scale.scene_radius, terraza.scale.monolith_height),
            "orbit_radius no corresponde con scene_radius y monolith_height"
        );
    }

    #[test]
    fn los_catorce_niveles_previos_no_cambian_al_construir_la_terraza() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
        ];

        let _terraza = nivel_terraza_verde_de_borde();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }
    }

    // ------------------------------------------------------------------
    // Cadena de plataformas verdes: `platforms`
    // ------------------------------------------------------------------

    /// Cuántas plataformas forman la cadena.
    const PLATAFORMAS: usize = 3;

    /// Claro mínimo exigible entre dos plataformas.
    const CLARO_PLATAFORMAS: f32 = 0.50;

    #[test]
    fn las_plataformas_conservan_el_conteo() {
        let cadena = nivel_plataformas_de_borde();

        assert_eq!(
            cadena.scene.objects.len(),
            OBJETOS,
            "la cadena de plataformas cambio el conteo del nivel"
        );
    }

    #[test]
    fn hay_tres_plataformas_declaradas_y_solo_dos_cambian() {
        let terraza = nivel_terraza_verde_de_borde();
        let cadena = nivel_plataformas_de_borde();

        let plataformas = plataformas_de_borde(&terraza.scene);

        assert_eq!(
            plataformas.len(),
            PLATAFORMAS,
            "la cadena declara {} plataformas",
            plataformas.len()
        );
        assert_eq!(
            plataformas[0],
            masa_pedestal(&terraza.scene),
            "la principal no es la losa que ya sostenia el Rompeolas"
        );

        for i in 0..OBJETOS {
            let a = &terraza.scene.objects[i];
            let b = &cadena.scene.objects[i];

            // Siguen siendo las mismas piezas: tipo, materiales y grupos.
            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            if a.primitive.bounds() != b.primitive.bounds() {
                assert!(
                    plataformas[1..].contains(&i),
                    "la pieza {i} cambio y no es una de las dos plataformas nuevas"
                );
            }
        }

        // La principal se queda tal cual la dejo `green_shelf`.
        assert_eq!(
            terraza.scene.objects[plataformas[0]].primitive.bounds(),
            cadena.scene.objects[plataformas[0]].primitive.bounds(),
            "la plataforma principal se movio"
        );

        // Y las tres son masas del arco costero.
        for &i in &plataformas {
            assert_eq!(
                cadena.scene.objects[i].spatial_group,
                SpatialGroupId::ContinentBackground,
                "la plataforma {i} no es una masa de G-02"
            );
        }
    }

    #[test]
    fn la_principal_sigue_sosteniendo_el_rompeolas() {
        let terraza = nivel_terraza_verde_de_borde();
        let cadena = nivel_plataformas_de_borde();

        let principal = cadena.scene.objects[plataformas_de_borde(&terraza.scene)[0]]
            .primitive
            .bounds();
        let rompeolas = huella_del_grupo(&cadena.scene, SpatialGroupId::Breakwater);

        assert!(
            principal.min.x <= rompeolas.min.x
                && principal.max.x >= rompeolas.max.x
                && principal.min.z <= rompeolas.min.z
                && principal.max.z >= rompeolas.max.z,
            "la plataforma principal dejo de cubrir el Rompeolas"
        );

        // Y sigue siendo la que esta junto a Aguas.
        let aguas = huella_del_grupo(&cadena.scene, SpatialGroupId::FlyingWaters);

        assert!(
            aguas.min.x - principal.max.x > 0.0,
            "la principal toca Aguas"
        );
    }

    #[test]
    fn las_otras_dos_ocupan_el_corredor_hacia_praderas() {
        let terraza = nivel_terraza_verde_de_borde();
        let cadena = nivel_plataformas_de_borde();

        let plataformas = plataformas_de_borde(&terraza.scene);
        let principal = cadena.scene.objects[plataformas[0]].primitive.bounds();
        let praderas = huella_del_grupo(&cadena.scene, SpatialGroupId::Meadows);

        for &i in &plataformas[1..] {
            let caja = cadena.scene.objects[i].primitive.bounds();

            assert!(
                caja.max.z < principal.min.z,
                "la plataforma {i} no esta por delante de la principal hacia Praderas"
            );
            assert!(
                caja.min.z >= praderas.max.z + CLARO_MINIMO,
                "la plataforma {i} se mete en el claro de Praderas"
            );
        }

        // Entre las dos cubren buena parte del corredor: la mas adelantada
        // llega al claro de Praderas.
        let mas_adelantada = plataformas[1..]
            .iter()
            .map(|&i| cadena.scene.objects[i].primitive.bounds().min.z)
            .fold(f32::MAX, f32::min);

        assert!(
            mas_adelantada - praderas.max.z <= CLARO_MINIMO * 1.5,
            "la cadena se queda a {:.2} de Praderas: no recorre el corredor",
            mas_adelantada - praderas.max.z
        );
    }

    #[test]
    fn las_plataformas_estan_separadas_entre_si() {
        let terraza = nivel_terraza_verde_de_borde();
        let cadena = nivel_plataformas_de_borde();

        let plataformas = plataformas_de_borde(&terraza.scene);

        for a in 0..plataformas.len() {
            for b in a + 1..plataformas.len() {
                let claro = separacion_xz(
                    &cadena.scene.objects[plataformas[a]].primitive.bounds(),
                    &cadena.scene.objects[plataformas[b]].primitive.bounds(),
                );

                assert!(
                    claro >= CLARO_PLATAFORMAS,
                    "las plataformas {} y {} dejan {claro:.3} de claro: forman una placa",
                    plataformas[a],
                    plataformas[b]
                );
            }
        }
    }

    #[test]
    fn ninguna_plataforma_invade_ni_forma_placa_continua() {
        let terraza = nivel_terraza_verde_de_borde();
        let cadena = nivel_plataformas_de_borde();

        let plataformas = plataformas_de_borde(&terraza.scene);
        let plinto = huella_del_grupo(&cadena.scene, SpatialGroupId::Global);

        for &i in &plataformas {
            let caja = cadena.scene.objects[i].primitive.bounds();

            assert!(
                caja.min.x >= plinto.min.x
                    && caja.max.x <= plinto.max.x
                    && caja.min.z >= plinto.min.z
                    && caja.max.z <= plinto.max.z,
                "la plataforma {i} se sale del plinto"
            );

            for (nombre, grupo) in [
                ("el Monolito", SpatialGroupId::Monolith),
                ("Praderas", SpatialGroupId::Meadows),
                ("Aguas", SpatialGroupId::FlyingWaters),
            ] {
                let otra = huella_del_grupo(&cadena.scene, grupo);

                let cortan = caja.min.x < otra.max.x
                    && caja.max.x > otra.min.x
                    && caja.min.y < otra.max.y
                    && caja.max.y > otra.min.y
                    && caja.min.z < otra.max.z
                    && caja.max.z > otra.min.z;

                assert!(!cortan, "la plataforma {i} corta {nombre}");

                // Y ninguna plataforma se mete bajo dos regiones a la vez:
                // eso seria empezar a formar la placa continua.
                assert!(
                    separacion_xz(&caja, &otra) > 0.0,
                    "la plataforma {i} se solapa en planta con {nombre}"
                );
            }
        }
    }

    #[test]
    fn el_rompeolas_y_las_zonas_no_se_mueven_con_las_plataformas() {
        let terraza = nivel_terraza_verde_de_borde();
        let cadena = nivel_plataformas_de_borde();

        for grupo in [
            SpatialGroupId::Breakwater,
            SpatialGroupId::Monolith,
            SpatialGroupId::Meadows,
            SpatialGroupId::FlyingWaters,
            SpatialGroupId::Global,
        ] {
            for &i in &indices_del_grupo(&cadena, grupo) {
                assert_eq!(
                    terraza.scene.objects[i].primitive.bounds(),
                    cadena.scene.objects[i].primitive.bounds(),
                    "la pieza {i} de {grupo:?} se movio"
                );
            }
        }

        assert_eq!(
            volumen_de_agua(&terraza.scene),
            volumen_de_agua(&cadena.scene),
            "A-01 cambio de indice"
        );
    }

    #[test]
    fn cada_prisma_sigue_apoyado_con_las_plataformas() {
        let cadena = nivel_plataformas_de_borde();
        let piezas = indices_del_grupo(&cadena, SpatialGroupId::Breakwater);

        for &i in &piezas[..PILARES] {
            let caja = cadena.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&cadena.scene, i)
                .unwrap_or_else(|| panic!("el prisma {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo,
                "el prisma {i} arranca en {:.3} y su pedestal llega a {techo:.3}: flota",
                caja.min.y
            );
            assert!(
                caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "el prisma {i} arranca en {:.3} sobre un pedestal de {techo:.3}: enterrado",
                caja.min.y
            );
        }
    }

    #[test]
    fn las_plataformas_son_deterministas() {
        let uno = nivel_plataformas_de_borde();
        let dos = nivel_plataformas_de_borde();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
    }

    #[test]
    fn la_escala_de_las_plataformas_se_deriva_de_la_escena() {
        let cadena = nivel_plataformas_de_borde();

        assert_eq!(
            cadena.scale.scene_radius,
            measure_scene_radius(&cadena.scene, cadena.anchors.orbit_center),
            "scene_radius no se volvio a medir"
        );
        assert_eq!(
            cadena.scale.orbit_radius,
            derive_orbit_radius(cadena.scale.scene_radius, cadena.scale.monolith_height),
            "orbit_radius no corresponde con scene_radius y monolith_height"
        );
    }

    #[test]
    fn los_quince_niveles_previos_no_cambian_al_construir_las_plataformas() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
        ];

        let _cadena = nivel_plataformas_de_borde();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }
    }

    // ------------------------------------------------------------------
    // Plataformas finas y toma cenital: `platforms_thin`
    // ------------------------------------------------------------------

    /// Cuánto tiene que adelgazar como mínimo una cornisa.
    const ADELGAZAMIENTO_MINIMO: f32 = 0.30;

    #[test]
    fn las_plataformas_finas_conservan_el_conteo() {
        let finas = nivel_plataformas_finas();

        assert_eq!(
            finas.scene.objects.len(),
            OBJETOS,
            "las plataformas finas cambiaron el conteo del nivel"
        );
    }

    #[test]
    fn las_tres_plataformas_pierden_ancho_y_conservan_la_ruta() {
        let cadena = nivel_plataformas_de_borde();
        let finas = nivel_plataformas_finas();

        let plataformas = plataformas_de_borde(&nivel_terraza_verde_de_borde().scene);

        for (n, &i) in plataformas.iter().enumerate() {
            let antes = cadena.scene.objects[i].primitive.bounds();
            let ahora = finas.scene.objects[i].primitive.bounds();

            // Adelgazan en `x`, que es lo transversal al corredor.
            assert!(
                ahora.max.x - ahora.min.x < antes.max.x - antes.min.x,
                "la plataforma {n} no adelgazo: {:.2} -> {:.2}",
                antes.max.x - antes.min.x,
                ahora.max.x - ahora.min.x
            );

            // La ruta en `z` no se toca: mismos bordes, mismos claros.
            assert!(
                (ahora.min.z - antes.min.z).abs() < EPS && (ahora.max.z - antes.max.z).abs() < EPS,
                "la plataforma {n} cambio su tramo de corredor"
            );

            // Y el espesor tampoco.
            assert!(
                (ahora.min.y - antes.min.y).abs() < EPS && (ahora.max.y - antes.max.y).abs() < EPS,
                "la plataforma {n} cambio de espesor"
            );
        }

        // Las dos cornisas adelgazan de verdad, no un pelo.
        for &i in &plataformas[1..] {
            let antes = cadena.scene.objects[i].primitive.bounds();
            let ahora = finas.scene.objects[i].primitive.bounds();
            let razon = (ahora.max.x - ahora.min.x) / (antes.max.x - antes.min.x);

            assert!(
                razon <= 1.0 - ADELGAZAMIENTO_MINIMO,
                "la cornisa {i} se quedo en el {:.0} % de su ancho",
                razon * 100.0
            );
        }
    }

    #[test]
    fn la_principal_fina_sigue_cubriendo_el_rompeolas() {
        let finas = nivel_plataformas_finas();

        let plataformas = plataformas_de_borde(&nivel_terraza_verde_de_borde().scene);
        let principal = finas.scene.objects[plataformas[0]].primitive.bounds();
        let rompeolas = huella_del_grupo(&finas.scene, SpatialGroupId::Breakwater);

        assert!(
            principal.min.x <= rompeolas.min.x
                && principal.max.x >= rompeolas.max.x
                && principal.min.z <= rompeolas.min.z
                && principal.max.z >= rompeolas.max.z,
            "la principal adelgazo tanto que dejo de cubrir el Rompeolas"
        );
    }

    #[test]
    fn las_plataformas_finas_siguen_separadas_y_sin_invadir() {
        let finas = nivel_plataformas_finas();

        let plataformas = plataformas_de_borde(&nivel_terraza_verde_de_borde().scene);
        let plinto = huella_del_grupo(&finas.scene, SpatialGroupId::Global);

        for a in 0..plataformas.len() {
            for b in a + 1..plataformas.len() {
                let claro = separacion_xz(
                    &finas.scene.objects[plataformas[a]].primitive.bounds(),
                    &finas.scene.objects[plataformas[b]].primitive.bounds(),
                );

                assert!(
                    claro >= CLARO_PLATAFORMAS,
                    "las plataformas {} y {} dejan {claro:.3}",
                    plataformas[a],
                    plataformas[b]
                );
            }
        }

        for &i in &plataformas {
            let caja = finas.scene.objects[i].primitive.bounds();

            assert!(
                caja.min.x >= plinto.min.x
                    && caja.max.x <= plinto.max.x
                    && caja.min.z >= plinto.min.z
                    && caja.max.z <= plinto.max.z,
                "la plataforma {i} se sale del plinto"
            );

            for (nombre, grupo) in [
                ("el Monolito", SpatialGroupId::Monolith),
                ("Praderas", SpatialGroupId::Meadows),
                ("Aguas", SpatialGroupId::FlyingWaters),
            ] {
                assert!(
                    separacion_xz(&caja, &huella_del_grupo(&finas.scene, grupo)) > 0.0,
                    "la plataforma {i} se solapa con {nombre}"
                );
            }
        }
    }

    #[test]
    fn nada_mas_cambia_con_las_plataformas_finas() {
        let cadena = nivel_plataformas_de_borde();
        let finas = nivel_plataformas_finas();

        let plataformas = plataformas_de_borde(&nivel_terraza_verde_de_borde().scene);

        for i in 0..OBJETOS {
            let a = &cadena.scene.objects[i];
            let b = &finas.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            if a.primitive.bounds() != b.primitive.bounds() {
                assert!(
                    plataformas.contains(&i),
                    "la pieza {i} cambio y no es una plataforma"
                );
            }
        }

        assert_eq!(
            volumen_de_agua(&cadena.scene),
            volumen_de_agua(&finas.scene),
            "A-01 cambio de indice"
        );
    }

    #[test]
    fn cada_prisma_sigue_apoyado_con_las_plataformas_finas() {
        let finas = nivel_plataformas_finas();
        let piezas = indices_del_grupo(&finas, SpatialGroupId::Breakwater);

        for &i in &piezas[..PILARES] {
            let caja = finas.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&finas.scene, i)
                .unwrap_or_else(|| panic!("el prisma {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo,
                "el prisma {i} arranca en {:.3} y su pedestal llega a {techo:.3}: flota",
                caja.min.y
            );
            assert!(
                caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "el prisma {i} arranca en {:.3} sobre un pedestal de {techo:.3}: enterrado",
                caja.min.y
            );
        }
    }

    #[test]
    fn la_camara_cenital_es_derivada_y_determinista() {
        let finas = nivel_plataformas_finas();

        let hero = finas.hero_camera();
        let alto = camara_cenital(&finas);

        // El mismo centro y el mismo punto de mira que la toma hero: lo
        // unico que cambia es la elevacion.
        assert_eq!(alto.orbit_center, hero.orbit_center, "cambio el centro");
        assert_eq!(alto.look_at, hero.look_at, "cambio el punto de mira");
        assert_eq!(alto.vertical_fov, hero.vertical_fov, "cambio el campo");

        // Esta mas alta y mas cerca del eje.
        assert!(
            alto.eye.y > hero.eye.y,
            "la cenital no esta por encima: {:.2} contra {:.2}",
            alto.eye.y,
            hero.eye.y
        );

        let radio_horizontal = |c: &expedition33_continente_inacabado::camera::Camera| {
            ((c.eye.x - c.orbit_center.x).powi(2) + (c.eye.z - c.orbit_center.z).powi(2)).sqrt()
        };

        assert!(
            radio_horizontal(&alto) < radio_horizontal(&hero),
            "la cenital no se acerco al eje"
        );

        // Y esta sobre la esfera orbital derivada, al radio que mide la
        // escena: no es una camara inventada.
        let radio = (alto.eye - alto.orbit_center).magnitude();

        assert!(
            (radio - finas.scale.orbit_radius).abs() < 1.0e-3,
            "la cenital esta a {radio:.4} y la orbita derivada es {:.4}",
            finas.scale.orbit_radius
        );

        // Determinista.
        let otra = camara_cenital(&nivel_plataformas_finas());

        assert_eq!(alto.eye, otra.eye, "la cenital no es determinista");
        assert_eq!(alto.look_at, otra.look_at);
    }

    #[test]
    fn las_plataformas_finas_son_deterministas() {
        let uno = nivel_plataformas_finas();
        let dos = nivel_plataformas_finas();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
    }

    #[test]
    fn los_dieciseis_niveles_previos_no_cambian_al_afinar_las_plataformas() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
        ];

        let _finas = nivel_plataformas_finas();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }
    }

    // ------------------------------------------------------------------
    // Módulos de la parcela marcada: `marked_platforms`
    // ------------------------------------------------------------------

    /// Cuántas veces más profundo que ancho tiene que ser un módulo.
    ///
    /// La marca roja de `exp33/platforms_thin_top.png` es un rectángulo
    /// claramente vertical en la toma cenital: con el yaw hero, la vertical
    /// de la pantalla es el eje `z`. De ahí sale esta proporción.
    const ESBELTEZ_MINIMA: f32 = 2.00;

    #[test]
    fn las_plataformas_marcadas_conservan_el_conteo() {
        let marcadas = nivel_plataformas_marcadas();

        assert_eq!(
            marcadas.scene.objects.len(),
            OBJETOS,
            "las plataformas marcadas cambiaron el conteo del nivel"
        );
    }

    #[test]
    fn son_tres_plataformas_y_solo_ellas_cambian() {
        let apoyado = nivel_apoyado();
        let marcadas = nivel_plataformas_marcadas();

        let plataformas = plataformas_de_borde(&apoyado.scene);

        assert!(
            (2..=3).contains(&plataformas.len()),
            "la parcela declara {} plataformas",
            plataformas.len()
        );

        for i in 0..OBJETOS {
            let a = &apoyado.scene.objects[i];
            let b = &marcadas.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            if a.primitive.bounds() != b.primitive.bounds() {
                assert!(
                    plataformas.contains(&i),
                    "la pieza {i} cambio y no es una plataforma"
                );
            }
        }

        for &i in &plataformas {
            assert_eq!(
                marcadas.scene.objects[i].spatial_group,
                SpatialGroupId::ContinentBackground,
                "la plataforma {i} no es una masa de G-02"
            );
        }
    }

    #[test]
    fn los_modulos_son_estrechos_en_x_y_largos_en_z() {
        let apoyado = nivel_apoyado();
        let marcadas = nivel_plataformas_marcadas();

        // La principal queda fuera de esta regla a proposito: tiene que
        // cubrir el Rompeolas, que mide mas de ancho que de fondo. Los
        // modulos si siguen la forma de la marca.
        for &i in &plataformas_de_borde(&apoyado.scene)[1..] {
            let caja = marcadas.scene.objects[i].primitive.bounds();
            let ancho = caja.max.x - caja.min.x;
            let fondo = caja.max.z - caja.min.z;

            assert!(
                fondo >= ancho * ESBELTEZ_MINIMA,
                "el modulo {i} mide {ancho:.2} de ancho y {fondo:.2} de fondo: no es esbelto"
            );
        }
    }

    #[test]
    fn la_principal_marcada_sigue_soportando_el_macizo() {
        let apoyado = nivel_apoyado();
        let marcadas = nivel_plataformas_marcadas();

        let principal = marcadas.scene.objects[plataformas_de_borde(&apoyado.scene)[0]]
            .primitive
            .bounds();
        let rompeolas = huella_del_grupo(&marcadas.scene, SpatialGroupId::Breakwater);

        assert!(
            principal.min.x <= rompeolas.min.x
                && principal.max.x >= rompeolas.max.x
                && principal.min.z <= rompeolas.min.z
                && principal.max.z >= rompeolas.max.z,
            "la principal dejo de cubrir el Rompeolas"
        );
    }

    #[test]
    fn los_modulos_caen_en_el_corredor_marcado() {
        let apoyado = nivel_apoyado();
        let marcadas = nivel_plataformas_marcadas();

        let monolito = huella_del_grupo(&marcadas.scene, SpatialGroupId::Monolith);
        let praderas = huella_del_grupo(&marcadas.scene, SpatialGroupId::Meadows);
        let aguas = huella_del_grupo(&marcadas.scene, SpatialGroupId::FlyingWaters);

        for &i in &plataformas_de_borde(&apoyado.scene)[1..] {
            let caja = marcadas.scene.objects[i].primitive.bounds();

            // Al oeste del Monolito.
            assert!(
                caja.max.x < monolito.min.x,
                "el modulo {i} no esta al oeste del Monolito"
            );

            // Por delante de Praderas, con el claro entero.
            assert!(
                caja.min.z >= praderas.max.z + CLARO_MINIMO,
                "el modulo {i} se mete en el claro de Praderas"
            );

            // Al noroeste de Aguas: ni en su abscisa ni en su profundidad.
            assert!(
                caja.max.x < aguas.min.x && caja.max.z < aguas.min.z,
                "el modulo {i} no queda al noroeste de Aguas"
            );
        }
    }

    #[test]
    fn las_plataformas_marcadas_son_islas() {
        let apoyado = nivel_apoyado();
        let marcadas = nivel_plataformas_marcadas();

        let plataformas = plataformas_de_borde(&apoyado.scene);

        for a in 0..plataformas.len() {
            for b in a + 1..plataformas.len() {
                let claro = separacion_xz(
                    &marcadas.scene.objects[plataformas[a]].primitive.bounds(),
                    &marcadas.scene.objects[plataformas[b]].primitive.bounds(),
                );

                assert!(
                    claro >= CLARO_PLATAFORMAS,
                    "las plataformas {} y {} dejan {claro:.3}: forman una franja",
                    plataformas[a],
                    plataformas[b]
                );
            }
        }
    }

    #[test]
    fn las_plataformas_marcadas_no_invaden_nada() {
        let apoyado = nivel_apoyado();
        let marcadas = nivel_plataformas_marcadas();

        let plinto = huella_del_grupo(&marcadas.scene, SpatialGroupId::Global);

        for &i in &plataformas_de_borde(&apoyado.scene) {
            let caja = marcadas.scene.objects[i].primitive.bounds();

            assert!(
                caja.min.x >= plinto.min.x
                    && caja.max.x <= plinto.max.x
                    && caja.min.z >= plinto.min.z
                    && caja.max.z <= plinto.max.z,
                "la plataforma {i} se sale del plinto"
            );

            for (nombre, grupo) in [
                ("el Monolito", SpatialGroupId::Monolith),
                ("Praderas", SpatialGroupId::Meadows),
                ("Aguas", SpatialGroupId::FlyingWaters),
            ] {
                assert!(
                    separacion_xz(&caja, &huella_del_grupo(&marcadas.scene, grupo)) > 0.0,
                    "la plataforma {i} se solapa con {nombre}"
                );
            }
        }
    }

    #[test]
    fn el_rompeolas_y_las_zonas_no_se_mueven_con_las_marcadas() {
        let apoyado = nivel_apoyado();
        let marcadas = nivel_plataformas_marcadas();

        for grupo in [
            SpatialGroupId::Breakwater,
            SpatialGroupId::Monolith,
            SpatialGroupId::Meadows,
            SpatialGroupId::FlyingWaters,
            SpatialGroupId::Global,
        ] {
            for &i in &indices_del_grupo(&marcadas, grupo) {
                assert_eq!(
                    apoyado.scene.objects[i].primitive.bounds(),
                    marcadas.scene.objects[i].primitive.bounds(),
                    "la pieza {i} de {grupo:?} se movio"
                );
            }
        }

        assert_eq!(
            volumen_de_agua(&apoyado.scene),
            volumen_de_agua(&marcadas.scene),
            "A-01 cambio de indice"
        );
    }

    #[test]
    fn cada_prisma_sigue_apoyado_con_las_marcadas() {
        let marcadas = nivel_plataformas_marcadas();
        let piezas = indices_del_grupo(&marcadas, SpatialGroupId::Breakwater);

        for &i in &piezas[..PILARES] {
            let caja = marcadas.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&marcadas.scene, i)
                .unwrap_or_else(|| panic!("el prisma {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo,
                "el prisma {i} arranca en {:.3} y su pedestal llega a {techo:.3}: flota",
                caja.min.y
            );
            assert!(
                caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "el prisma {i} arranca en {:.3} sobre un pedestal de {techo:.3}: enterrado",
                caja.min.y
            );
        }
    }

    #[test]
    fn las_plataformas_marcadas_son_deterministas() {
        let uno = nivel_plataformas_marcadas();
        let dos = nivel_plataformas_marcadas();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
    }

    #[test]
    fn los_diecisiete_niveles_previos_no_cambian_con_las_marcadas() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
        ];

        let _marcadas = nivel_plataformas_marcadas();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }
    }

    // ------------------------------------------------------------------
    // Soporte modular: `modular_support`
    // ------------------------------------------------------------------

    /// Anchura por encima de la cual una plataforma deja de ser un módulo y
    /// pasa a ser una megaplataforma.
    ///
    /// La losa que `grounded` le dio al Rompeolas medía `12.00`. Un módulo
    /// de la parcela marcada no llega ni a la cuarta parte.
    const ANCHO_DE_MEGAPLATAFORMA: f32 = 3.50;

    /// Hueco mínimo entre dos módulos para que se vea el lienzo.
    const HUECO_VISIBLE: f32 = 1.00;

    #[test]
    fn el_soporte_modular_conserva_el_conteo() {
        let modular = nivel_soporte_modular();

        assert_eq!(
            modular.scene.objects.len(),
            OBJETOS,
            "el soporte modular cambio el conteo del nivel"
        );
    }

    #[test]
    fn ninguna_plataforma_es_una_megaplataforma() {
        let apoyado = nivel_apoyado();
        let modular = nivel_soporte_modular();

        let plataformas = plataformas_de_borde(&apoyado.scene);

        assert_eq!(plataformas.len(), 3, "no son tres plataformas");

        for &i in &plataformas {
            let caja = modular.scene.objects[i].primitive.bounds();
            let ancho = caja.max.x - caja.min.x;
            let fondo = caja.max.z - caja.min.z;

            assert!(
                ancho <= ANCHO_DE_MEGAPLATAFORMA,
                "la plataforma {i} mide {ancho:.2} de ancho: sigue siendo una megaplataforma"
            );
            assert!(
                fondo >= ancho * ESBELTEZ_MINIMA,
                "la plataforma {i} mide {ancho:.2} x {fondo:.2}: no es un modulo esbelto"
            );
        }
    }

    #[test]
    fn los_modulos_dejan_hueco_a_la_vista() {
        let apoyado = nivel_apoyado();
        let modular = nivel_soporte_modular();

        let plataformas = plataformas_de_borde(&apoyado.scene);

        for a in 0..plataformas.len() {
            for b in a + 1..plataformas.len() {
                let claro = separacion_xz(
                    &modular.scene.objects[plataformas[a]].primitive.bounds(),
                    &modular.scene.objects[plataformas[b]].primitive.bounds(),
                );

                assert!(
                    claro >= HUECO_VISIBLE,
                    "los modulos {} y {} dejan {claro:.3}: se leen como una placa",
                    plataformas[a],
                    plataformas[b]
                );
            }
        }
    }

    #[test]
    fn las_treinta_y_ocho_piezas_tienen_soporte_modular() {
        // Ahora no basta con `R-01`: si la losa deja de ser una losa, hay
        // que comprobar que tambien `R-02` y `R-03` se apoyan en algo.
        let modular = nivel_soporte_modular();
        let piezas = indices_del_grupo(&modular, SpatialGroupId::Breakwater);

        assert_eq!(piezas.len(), 38, "el Rompeolas no tiene 38 piezas");

        for &i in &piezas {
            let caja = modular.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&modular.scene, i)
                .unwrap_or_else(|| panic!("la pieza {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo,
                "la pieza {i} arranca en {:.3} y su pedestal llega a {techo:.3}: flota",
                caja.min.y
            );
            assert!(
                caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "la pieza {i} arranca en {:.3} sobre un pedestal de {techo:.3}: enterrada",
                caja.min.y
            );
        }
    }

    #[test]
    fn el_macizo_sigue_siendo_reconocible() {
        // Reconfigurar el soporte no puede desparramar la formacion: su
        // huella no crece y sus piezas siguen pegadas unas a otras.
        let apoyado = nivel_apoyado();
        let modular = nivel_soporte_modular();

        let antes = huella_del_grupo(&apoyado.scene, SpatialGroupId::Breakwater);
        let ahora = huella_del_grupo(&modular.scene, SpatialGroupId::Breakwater);

        assert!(
            ahora.min.x >= antes.min.x - EPS
                && ahora.max.x <= antes.max.x + EPS
                && ahora.min.z >= antes.min.z - EPS
                && ahora.max.z <= antes.max.z + EPS,
            "el Rompeolas se salio de su huella: [{:.2},{:.2}]x[{:.2},{:.2}] -> \
             [{:.2},{:.2}]x[{:.2},{:.2}]",
            antes.min.x,
            antes.max.x,
            antes.min.z,
            antes.max.z,
            ahora.min.x,
            ahora.max.x,
            ahora.min.z,
            ahora.max.z
        );

        let piezas = indices_del_grupo(&modular, SpatialGroupId::Breakwater);
        let pilares = &piezas[..PILARES];
        let centro = |i: usize| centro_xz(&modular.scene.objects[i].primitive.bounds());
        let ancho_medio = pilares
            .iter()
            .map(|&i| {
                let c = modular.scene.objects[i].primitive.bounds();

                c.max.x - c.min.x
            })
            .sum::<f32>()
            / pilares.len() as f32;
        let vecino = |i: usize| {
            pilares
                .iter()
                .filter(|&&j| j != i)
                .map(|&j| {
                    let (ax, az) = centro(i);
                    let (bx, bz) = centro(j);

                    ((ax - bx).powi(2) + (az - bz).powi(2)).sqrt()
                })
                .fold(f32::MAX, f32::min)
        };
        let medio = pilares.iter().map(|&i| vecino(i)).sum::<f32>() / pilares.len() as f32;

        assert!(
            medio <= ancho_medio * 2.0,
            "el vecino medio esta a {medio:.2} y el ancho medio es {ancho_medio:.2}: \
             la formacion se desparramo"
        );
    }

    #[test]
    fn el_soporte_modular_no_invade_nada() {
        let apoyado = nivel_apoyado();
        let modular = nivel_soporte_modular();

        let plinto = huella_del_grupo(&modular.scene, SpatialGroupId::Global);

        for &i in &plataformas_de_borde(&apoyado.scene) {
            let caja = modular.scene.objects[i].primitive.bounds();

            assert!(
                caja.min.x >= plinto.min.x
                    && caja.max.x <= plinto.max.x
                    && caja.min.z >= plinto.min.z
                    && caja.max.z <= plinto.max.z,
                "el modulo {i} se sale del plinto"
            );

            for (nombre, grupo) in [
                ("el Monolito", SpatialGroupId::Monolith),
                ("Praderas", SpatialGroupId::Meadows),
                ("Aguas", SpatialGroupId::FlyingWaters),
            ] {
                assert!(
                    separacion_xz(&caja, &huella_del_grupo(&modular.scene, grupo)) > 0.0,
                    "el modulo {i} se solapa con {nombre}"
                );
            }
        }

        let huellas = [
            ("Praderas", SpatialGroupId::Meadows),
            ("Rompeolas", SpatialGroupId::Breakwater),
            ("Aguas", SpatialGroupId::FlyingWaters),
        ];

        for a in 0..huellas.len() {
            for b in a + 1..huellas.len() {
                let claro = separacion_xz(
                    &huella_del_grupo(&modular.scene, huellas[a].1),
                    &huella_del_grupo(&modular.scene, huellas[b].1),
                );

                assert!(
                    claro >= CLARO_MINIMO,
                    "{} y {} dejan {claro:.3} de claro",
                    huellas[a].0,
                    huellas[b].0
                );
            }
        }
    }

    #[test]
    fn las_zonas_no_se_mueven_con_el_soporte_modular() {
        let apoyado = nivel_apoyado();
        let modular = nivel_soporte_modular();

        for grupo in [
            SpatialGroupId::Monolith,
            SpatialGroupId::Meadows,
            SpatialGroupId::FlyingWaters,
            SpatialGroupId::Global,
        ] {
            for &i in &indices_del_grupo(&modular, grupo) {
                assert_eq!(
                    apoyado.scene.objects[i].primitive.bounds(),
                    modular.scene.objects[i].primitive.bounds(),
                    "la pieza {i} de {grupo:?} se movio"
                );
            }
        }

        assert_eq!(
            volumen_de_agua(&apoyado.scene),
            volumen_de_agua(&modular.scene),
            "A-01 cambio de indice"
        );

        // Y todo conserva tipo, materiales y grupos.
        for i in 0..OBJETOS {
            let a = &apoyado.scene.objects[i];
            let b = &modular.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");
        }
    }

    #[test]
    fn las_piezas_del_rompeolas_solo_se_trasladan() {
        let apoyado = nivel_apoyado();
        let modular = nivel_soporte_modular();

        for &i in &indices_del_grupo(&modular, SpatialGroupId::Breakwater) {
            let antes = apoyado.scene.objects[i].primitive.bounds();
            let ahora = modular.scene.objects[i].primitive.bounds();

            assert!(
                (tamano_de(&ahora) - tamano_de(&antes)).magnitude() < EPS,
                "la pieza {i} cambio de tamano: el soporte modular solo traslada"
            );

            // Y no se mueven en `z`: lo unico que se ajusta es la abscisa,
            // para meterlas en un modulo, y la cota, para posarlas.
            assert!(
                (centro_de(&ahora).z - centro_de(&antes).z).abs() < EPS,
                "la pieza {i} cambio de profundidad"
            );
        }
    }

    #[test]
    fn el_soporte_modular_es_determinista() {
        let uno = nivel_soporte_modular();
        let dos = nivel_soporte_modular();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
    }

    #[test]
    fn los_dieciocho_niveles_previos_no_cambian_con_el_soporte_modular() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
            nivel_plataformas_marcadas(),
        ];

        let _modular = nivel_soporte_modular();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
            nivel_plataformas_marcadas(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }
    }

    // ------------------------------------------------------------------
    // Segunda isla verde: `second_island`
    // ------------------------------------------------------------------

    /// Área mínima en planta para que la isla sea territorio y no una
    /// cornisa.
    const AREA_DE_ISLA: f32 = 12.00;

    /// Lado mínimo de la isla, en cualquiera de los dos ejes.
    const LADO_DE_ISLA: f32 = 2.00;

    #[test]
    fn la_segunda_isla_conserva_el_conteo() {
        let isla = nivel_segunda_isla();

        assert_eq!(
            isla.scene.objects.len(),
            OBJETOS,
            "la segunda isla cambio el conteo del nivel"
        );
    }

    #[test]
    fn la_base_original_y_sus_prismas_quedan_exactos() {
        // Lo que la decision humana fija: `grounded` no se toca. Ni la losa
        // que sostiene el Rompeolas, ni una sola de sus treinta y ocho
        // piezas, ni el resto del diorama.
        let apoyado = nivel_apoyado();
        let isla = nivel_segunda_isla();

        let principal = masa_pedestal(&apoyado.scene);

        assert_eq!(
            apoyado.scene.objects[principal].primitive.bounds(),
            isla.scene.objects[principal].primitive.bounds(),
            "la plataforma principal cambio"
        );

        for grupo in [
            SpatialGroupId::Breakwater,
            SpatialGroupId::Monolith,
            SpatialGroupId::Meadows,
            SpatialGroupId::FlyingWaters,
            SpatialGroupId::Global,
        ] {
            for &i in &indices_del_grupo(&isla, grupo) {
                assert_eq!(
                    apoyado.scene.objects[i].primitive.bounds(),
                    isla.scene.objects[i].primitive.bounds(),
                    "la pieza {i} de {grupo:?} se movio"
                );
            }
        }

        assert_eq!(
            volumen_de_agua(&apoyado.scene),
            volumen_de_agua(&isla.scene),
            "A-01 cambio de indice"
        );
    }

    #[test]
    fn solo_una_masa_de_g02_cambia_y_no_es_la_principal() {
        let apoyado = nivel_apoyado();
        let isla = nivel_segunda_isla();

        let elegida = masa_de_la_segunda_isla(&apoyado.scene);

        assert_ne!(
            elegida,
            masa_pedestal(&apoyado.scene),
            "la isla reutilizo la propia plataforma principal"
        );
        assert_eq!(
            isla.scene.objects[elegida].spatial_group,
            SpatialGroupId::ContinentBackground,
            "la isla no es una masa de G-02"
        );

        for i in 0..OBJETOS {
            let a = &apoyado.scene.objects[i];
            let b = &isla.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            if a.primitive.bounds() != b.primitive.bounds() {
                assert_eq!(i, elegida, "la pieza {i} cambio y no es la isla");
            }
        }
    }

    #[test]
    fn la_isla_es_territorio_y_no_una_cornisa() {
        let apoyado = nivel_apoyado();
        let isla = nivel_segunda_isla();

        let caja = isla.scene.objects[masa_de_la_segunda_isla(&apoyado.scene)]
            .primitive
            .bounds();
        let ancho = caja.max.x - caja.min.x;
        let fondo = caja.max.z - caja.min.z;

        assert!(
            ancho * fondo >= AREA_DE_ISLA,
            "la isla ocupa {:.2} de superficie y el minimo es {AREA_DE_ISLA}",
            ancho * fondo
        );
        assert!(
            ancho >= LADO_DE_ISLA && fondo >= LADO_DE_ISLA,
            "la isla mide {ancho:.2} x {fondo:.2}: uno de sus lados no llega a {LADO_DE_ISLA}"
        );

        // Y es una sola pieza: no se parte en barras.
        assert!(
            matches!(
                isla.scene.objects[masa_de_la_segunda_isla(&apoyado.scene)].primitive,
                Primitive::Cuboid(_)
            ),
            "la isla dejo de ser un unico cuboide"
        );
    }

    #[test]
    fn la_isla_cae_en_la_parcela_marcada() {
        let apoyado = nivel_apoyado();
        let isla = nivel_segunda_isla();

        let caja = isla.scene.objects[masa_de_la_segunda_isla(&apoyado.scene)]
            .primitive
            .bounds();
        let principal = isla.scene.objects[masa_pedestal(&apoyado.scene)]
            .primitive
            .bounds();
        let monolito = huella_del_grupo(&isla.scene, SpatialGroupId::Monolith);
        let praderas = huella_del_grupo(&isla.scene, SpatialGroupId::Meadows);
        let aguas = huella_del_grupo(&isla.scene, SpatialGroupId::FlyingWaters);

        // Al oeste del Monolito.
        assert!(
            caja.max.x < monolito.min.x,
            "la isla no esta al oeste del Monolito"
        );

        // Entre la base original y Praderas.
        assert!(
            caja.max.z < principal.min.z,
            "la isla no esta por detras de la plataforma principal"
        );
        assert!(
            caja.min.z > praderas.max.z,
            "la isla no esta por delante de Praderas"
        );

        // Y lejos de Aguas.
        assert!(caja.max.x < aguas.min.x, "la isla se fue hacia Aguas");
    }

    #[test]
    fn la_isla_deja_claro_por_los_cuatro_lados() {
        let apoyado = nivel_apoyado();
        let isla = nivel_segunda_isla();

        let caja = isla.scene.objects[masa_de_la_segunda_isla(&apoyado.scene)]
            .primitive
            .bounds();
        let principal = isla.scene.objects[masa_pedestal(&apoyado.scene)]
            .primitive
            .bounds();

        for (nombre, otra) in [
            (
                "el Monolito",
                huella_del_grupo(&isla.scene, SpatialGroupId::Monolith),
            ),
            (
                "Praderas",
                huella_del_grupo(&isla.scene, SpatialGroupId::Meadows),
            ),
            (
                "Aguas",
                huella_del_grupo(&isla.scene, SpatialGroupId::FlyingWaters),
            ),
            ("la base principal", principal),
        ] {
            let claro = separacion_xz(&caja, &otra);

            assert!(claro >= CLARO_MINIMO, "la isla deja {claro:.3} a {nombre}");

            let cortan = caja.min.x < otra.max.x
                && caja.max.x > otra.min.x
                && caja.min.y < otra.max.y
                && caja.max.y > otra.min.y
                && caja.min.z < otra.max.z
                && caja.max.z > otra.min.z;

            assert!(!cortan, "la isla corta {nombre}");
        }

        // Dentro del plinto.
        let plinto = huella_del_grupo(&isla.scene, SpatialGroupId::Global);

        assert!(
            caja.min.x >= plinto.min.x
                && caja.max.x <= plinto.max.x
                && caja.min.z >= plinto.min.z
                && caja.max.z <= plinto.max.z,
            "la isla se sale del plinto"
        );
    }

    #[test]
    fn cada_prisma_sigue_apoyado_con_la_segunda_isla() {
        let isla = nivel_segunda_isla();
        let piezas = indices_del_grupo(&isla, SpatialGroupId::Breakwater);

        for &i in &piezas[..PILARES] {
            let caja = isla.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&isla.scene, i)
                .unwrap_or_else(|| panic!("el prisma {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo,
                "el prisma {i} arranca en {:.3} y su pedestal llega a {techo:.3}: flota",
                caja.min.y
            );
            assert!(
                caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "el prisma {i} arranca en {:.3} sobre un pedestal de {techo:.3}: enterrado",
                caja.min.y
            );
        }
    }

    #[test]
    fn la_segunda_isla_es_determinista() {
        let uno = nivel_segunda_isla();
        let dos = nivel_segunda_isla();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
    }

    #[test]
    fn los_diecinueve_niveles_previos_no_cambian_con_la_isla() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
            nivel_plataformas_marcadas(),
            nivel_soporte_modular(),
        ];

        let _isla = nivel_segunda_isla();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
            nivel_plataformas_marcadas(),
            nivel_soporte_modular(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }
    }

    // ------------------------------------------------------------------
    // Isla de borde: `edge_island`
    // ------------------------------------------------------------------

    #[test]
    fn la_isla_de_borde_conserva_el_conteo() {
        let borde = nivel_isla_de_borde();

        assert_eq!(
            borde.scene.objects.len(),
            OBJETOS,
            "la isla de borde cambio el conteo del nivel"
        );
    }

    #[test]
    fn solo_la_masa_de_la_isla_cambia_en_la_isla_de_borde() {
        let previo = nivel_segunda_isla();
        let borde = nivel_isla_de_borde();

        let elegida = masa_de_la_segunda_isla(&nivel_apoyado().scene);

        for i in 0..OBJETOS {
            let a = &previo.scene.objects[i];
            let b = &borde.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            if a.primitive.bounds() != b.primitive.bounds() {
                assert_eq!(i, elegida, "la pieza {i} cambio y no es la isla");
            }
        }
    }

    #[test]
    fn la_base_y_los_prismas_siguen_exactos_en_la_isla_de_borde() {
        let apoyado = nivel_apoyado();
        let borde = nivel_isla_de_borde();

        let principal = masa_pedestal(&apoyado.scene);

        assert_eq!(
            apoyado.scene.objects[principal].primitive.bounds(),
            borde.scene.objects[principal].primitive.bounds(),
            "la plataforma principal cambio"
        );

        for grupo in [
            SpatialGroupId::Breakwater,
            SpatialGroupId::Monolith,
            SpatialGroupId::Meadows,
            SpatialGroupId::FlyingWaters,
            SpatialGroupId::Global,
        ] {
            for &i in &indices_del_grupo(&borde, grupo) {
                assert_eq!(
                    apoyado.scene.objects[i].primitive.bounds(),
                    borde.scene.objects[i].primitive.bounds(),
                    "la pieza {i} de {grupo:?} se movio"
                );
            }
        }

        assert_eq!(
            volumen_de_agua(&apoyado.scene),
            volumen_de_agua(&borde.scene),
            "A-01 cambio de indice"
        );
    }

    #[test]
    fn la_isla_se_alarga_se_afina_y_se_corre_al_oeste() {
        let previo = nivel_segunda_isla();
        let borde = nivel_isla_de_borde();

        let elegida = masa_de_la_segunda_isla(&nivel_apoyado().scene);
        let antes = previo.scene.objects[elegida].primitive.bounds();
        let ahora = borde.scene.objects[elegida].primitive.bounds();

        let (ancho_antes, fondo_antes) = (antes.max.x - antes.min.x, antes.max.z - antes.min.z);
        let (ancho_ahora, fondo_ahora) = (ahora.max.x - ahora.min.x, ahora.max.z - ahora.min.z);

        assert!(
            fondo_ahora > fondo_antes,
            "el fondo paso de {fondo_antes:.2} a {fondo_ahora:.2}: no se alargo"
        );
        assert!(
            ancho_ahora < ancho_antes,
            "el ancho paso de {ancho_antes:.2} a {ancho_ahora:.2}: no se afino"
        );

        // Mas larga que ancha.
        assert!(
            fondo_ahora > ancho_ahora,
            "la isla mide {ancho_ahora:.2} x {fondo_ahora:.2}: sigue siendo mas ancha que larga"
        );

        // Se corrio hacia `-x`.
        assert!(
            centro_xz(&ahora).0 < centro_xz(&antes).0,
            "la isla no se movio hacia el oeste: {:.2} -> {:.2}",
            centro_xz(&antes).0,
            centro_xz(&ahora).0
        );

        // Y no perdio superficie util.
        assert!(
            ancho_ahora * fondo_ahora >= ancho_antes * fondo_antes,
            "la isla paso de {:.2} a {:.2} de superficie",
            ancho_antes * fondo_antes,
            ancho_ahora * fondo_ahora
        );
    }

    #[test]
    fn la_isla_de_borde_bordea_el_lienzo_por_el_oeste() {
        let borde = nivel_isla_de_borde();

        let elegida = masa_de_la_segunda_isla(&nivel_apoyado().scene);
        let caja = borde.scene.objects[elegida].primitive.bounds();
        let plinto = huella_del_grupo(&borde.scene, SpatialGroupId::Global);

        assert!(
            caja.min.x >= plinto.min.x
                && caja.max.x <= plinto.max.x
                && caja.min.z >= plinto.min.z
                && caja.max.z <= plinto.max.z,
            "la isla se sale del plinto"
        );

        // Pegada al canto oeste: lo que le sobra por ese lado es menos que
        // por el este.
        assert!(
            caja.min.x - plinto.min.x < plinto.max.x - caja.max.x,
            "la isla no esta arrimada al lado oeste del lienzo"
        );
        assert!(
            caja.min.x - plinto.min.x <= CLARO_MINIMO,
            "la isla deja {:.2} al canto oeste: no lo bordea",
            caja.min.x - plinto.min.x
        );
    }

    #[test]
    fn la_isla_de_borde_deja_claro_a_todo() {
        let apoyado = nivel_apoyado();
        let borde = nivel_isla_de_borde();

        let elegida = masa_de_la_segunda_isla(&apoyado.scene);
        let caja = borde.scene.objects[elegida].primitive.bounds();
        let principal = borde.scene.objects[masa_pedestal(&apoyado.scene)]
            .primitive
            .bounds();

        for (nombre, otra) in [
            ("la base principal", principal),
            (
                "el Monolito",
                huella_del_grupo(&borde.scene, SpatialGroupId::Monolith),
            ),
            (
                "Praderas",
                huella_del_grupo(&borde.scene, SpatialGroupId::Meadows),
            ),
            (
                "Aguas",
                huella_del_grupo(&borde.scene, SpatialGroupId::FlyingWaters),
            ),
            (
                "el Rompeolas",
                huella_del_grupo(&borde.scene, SpatialGroupId::Breakwater),
            ),
        ] {
            let claro = separacion_xz(&caja, &otra);

            assert!(claro >= CLARO_MINIMO, "la isla deja {claro:.3} a {nombre}");

            let cortan = caja.min.x < otra.max.x
                && caja.max.x > otra.min.x
                && caja.min.y < otra.max.y
                && caja.max.y > otra.min.y
                && caja.min.z < otra.max.z
                && caja.max.z > otra.min.z;

            assert!(!cortan, "la isla corta {nombre}");
        }

        // Y se alejo del Monolito respecto de donde estaba.
        let antes = nivel_segunda_isla().scene.objects[elegida]
            .primitive
            .bounds();
        let monolito = huella_del_grupo(&borde.scene, SpatialGroupId::Monolith);

        assert!(
            separacion_xz(&caja, &monolito) > separacion_xz(&antes, &monolito),
            "la isla no se alejo del Monolito"
        );
    }

    #[test]
    fn cada_prisma_sigue_apoyado_con_la_isla_de_borde() {
        let borde = nivel_isla_de_borde();
        let piezas = indices_del_grupo(&borde, SpatialGroupId::Breakwater);

        for &i in &piezas[..PILARES] {
            let caja = borde.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&borde.scene, i)
                .unwrap_or_else(|| panic!("el prisma {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo,
                "el prisma {i} arranca en {:.3} y su pedestal llega a {techo:.3}: flota",
                caja.min.y
            );
            assert!(
                caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "el prisma {i} arranca en {:.3} sobre un pedestal de {techo:.3}: enterrado",
                caja.min.y
            );
        }
    }

    #[test]
    fn la_isla_de_borde_es_determinista() {
        let uno = nivel_isla_de_borde();
        let dos = nivel_isla_de_borde();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
    }

    #[test]
    fn los_veinte_niveles_previos_no_cambian_con_la_isla_de_borde() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
            nivel_plataformas_marcadas(),
            nivel_soporte_modular(),
            nivel_segunda_isla(),
        ];

        let _borde = nivel_isla_de_borde();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
            nivel_plataformas_marcadas(),
            nivel_soporte_modular(),
            nivel_segunda_isla(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }
    }

    // ------------------------------------------------------------------
    // Isla de borde ancha: `edge_island_wide`
    // ------------------------------------------------------------------

    #[test]
    fn la_isla_ancha_conserva_el_conteo() {
        let ancha = nivel_isla_de_borde_ancha();

        assert_eq!(
            ancha.scene.objects.len(),
            OBJETOS,
            "la isla ancha cambio el conteo del nivel"
        );
    }

    #[test]
    fn solo_la_masa_de_la_isla_cambia_al_ensancharla() {
        let borde = nivel_isla_de_borde();
        let ancha = nivel_isla_de_borde_ancha();

        let elegida = masa_de_la_segunda_isla(&nivel_apoyado().scene);

        for i in 0..OBJETOS {
            let a = &borde.scene.objects[i];
            let b = &ancha.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            if a.primitive.bounds() != b.primitive.bounds() {
                assert_eq!(i, elegida, "la pieza {i} cambio y no es la isla");
            }
        }
    }

    #[test]
    fn la_base_y_los_prismas_siguen_exactos_en_la_isla_ancha() {
        let apoyado = nivel_apoyado();
        let ancha = nivel_isla_de_borde_ancha();

        let principal = masa_pedestal(&apoyado.scene);

        assert_eq!(
            apoyado.scene.objects[principal].primitive.bounds(),
            ancha.scene.objects[principal].primitive.bounds(),
            "la plataforma principal cambio"
        );

        for grupo in [
            SpatialGroupId::Breakwater,
            SpatialGroupId::Monolith,
            SpatialGroupId::Meadows,
            SpatialGroupId::FlyingWaters,
            SpatialGroupId::Global,
        ] {
            for &i in &indices_del_grupo(&ancha, grupo) {
                assert_eq!(
                    apoyado.scene.objects[i].primitive.bounds(),
                    ancha.scene.objects[i].primitive.bounds(),
                    "la pieza {i} de {grupo:?} se movio"
                );
            }
        }

        assert_eq!(
            volumen_de_agua(&apoyado.scene),
            volumen_de_agua(&ancha.scene),
            "A-01 cambio de indice"
        );
    }

    #[test]
    fn la_isla_solo_crece_hacia_mas_x() {
        let borde = nivel_isla_de_borde();
        let ancha = nivel_isla_de_borde_ancha();

        let elegida = masa_de_la_segunda_isla(&nivel_apoyado().scene);
        let antes = borde.scene.objects[elegida].primitive.bounds();
        let ahora = ancha.scene.objects[elegida].primitive.bounds();

        // El canto oeste, el tramo en `z` y el espesor no se tocan.
        assert!(
            (ahora.min.x - antes.min.x).abs() < EPS,
            "la isla se movio del canto oeste"
        );
        assert!(
            (ahora.min.z - antes.min.z).abs() < EPS && (ahora.max.z - antes.max.z).abs() < EPS,
            "la isla cambio su tramo en z"
        );
        assert!(
            (ahora.min.y - antes.min.y).abs() < EPS && (ahora.max.y - antes.max.y).abs() < EPS,
            "la isla cambio de espesor"
        );

        // Y crece por el este.
        assert!(
            ahora.max.x > antes.max.x,
            "la isla no se ensancho: x max {:.2} -> {:.2}",
            antes.max.x,
            ahora.max.x
        );

        let area = |c: &Aabb| (c.max.x - c.min.x) * (c.max.z - c.min.z);

        assert!(
            area(&ahora) > area(&antes),
            "la isla paso de {:.2} a {:.2} de superficie",
            area(&antes),
            area(&ahora)
        );
    }

    #[test]
    fn la_isla_ancha_se_para_justo_en_el_claro_de_praderas() {
        let ancha = nivel_isla_de_borde_ancha();

        let elegida = masa_de_la_segunda_isla(&nivel_apoyado().scene);
        let caja = ancha.scene.objects[elegida].primitive.bounds();
        let praderas = huella_del_grupo(&ancha.scene, SpatialGroupId::Meadows);

        let claro = praderas.min.x - caja.max.x;

        assert!(claro >= CLARO_MINIMO, "la isla deja {claro:.3} a Praderas");
        assert!(
            claro <= CLARO_MINIMO * 1.2,
            "la isla deja {claro:.3} a Praderas: no llego al maximo permitido"
        );
    }

    #[test]
    fn la_isla_ancha_no_invade_nada() {
        let apoyado = nivel_apoyado();
        let ancha = nivel_isla_de_borde_ancha();

        let elegida = masa_de_la_segunda_isla(&apoyado.scene);
        let caja = ancha.scene.objects[elegida].primitive.bounds();
        let principal = ancha.scene.objects[masa_pedestal(&apoyado.scene)]
            .primitive
            .bounds();
        let plinto = huella_del_grupo(&ancha.scene, SpatialGroupId::Global);

        assert!(
            caja.min.x >= plinto.min.x
                && caja.max.x <= plinto.max.x
                && caja.min.z >= plinto.min.z
                && caja.max.z <= plinto.max.z,
            "la isla se sale del plinto"
        );

        for (nombre, otra) in [
            ("la base principal", principal),
            (
                "el Monolito",
                huella_del_grupo(&ancha.scene, SpatialGroupId::Monolith),
            ),
            (
                "Praderas",
                huella_del_grupo(&ancha.scene, SpatialGroupId::Meadows),
            ),
            (
                "Aguas",
                huella_del_grupo(&ancha.scene, SpatialGroupId::FlyingWaters),
            ),
            (
                "el Rompeolas",
                huella_del_grupo(&ancha.scene, SpatialGroupId::Breakwater),
            ),
        ] {
            let claro = separacion_xz(&caja, &otra);

            assert!(claro >= CLARO_MINIMO, "la isla deja {claro:.3} a {nombre}");

            let cortan = caja.min.x < otra.max.x
                && caja.max.x > otra.min.x
                && caja.min.y < otra.max.y
                && caja.max.y > otra.min.y
                && caja.min.z < otra.max.z
                && caja.max.z > otra.min.z;

            assert!(!cortan, "la isla corta {nombre}");
        }
    }

    #[test]
    fn cada_prisma_sigue_apoyado_con_la_isla_ancha() {
        let ancha = nivel_isla_de_borde_ancha();
        let piezas = indices_del_grupo(&ancha, SpatialGroupId::Breakwater);

        for &i in &piezas[..PILARES] {
            let caja = ancha.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&ancha.scene, i)
                .unwrap_or_else(|| panic!("el prisma {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo,
                "el prisma {i} arranca en {:.3} y su pedestal llega a {techo:.3}: flota",
                caja.min.y
            );
            assert!(
                caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "el prisma {i} arranca en {:.3} sobre un pedestal de {techo:.3}: enterrado",
                caja.min.y
            );
        }
    }

    #[test]
    fn la_isla_ancha_es_determinista() {
        let uno = nivel_isla_de_borde_ancha();
        let dos = nivel_isla_de_borde_ancha();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
    }

    #[test]
    fn los_veintiun_niveles_previos_no_cambian_con_la_isla_ancha() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
            nivel_plataformas_marcadas(),
            nivel_soporte_modular(),
            nivel_segunda_isla(),
            nivel_isla_de_borde(),
        ];

        let _ancha = nivel_isla_de_borde_ancha();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
            nivel_plataformas_marcadas(),
            nivel_soporte_modular(),
            nivel_segunda_isla(),
            nivel_isla_de_borde(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }
    }

    // ------------------------------------------------------------------
    // Isla larga con escalera: `long_island_stair`
    // ------------------------------------------------------------------

    /// Cuántos prismas puede llevarse la escalera, como mucho.
    const ESCALERA_MAXIMO: usize = 8;

    /// Qué parte de `R-01` tiene que quedarse en el macizo.
    const MACIZO_DE_LA_ISLA: usize = 20;

    /// Hasta qué fracción de la altura del Monolito puede subir la escalera.
    const FRACCION_DEL_MONOLITO: f32 = 0.60;

    #[test]
    fn la_isla_larga_conserva_el_conteo() {
        let larga = nivel_isla_larga_con_escalera();

        assert_eq!(
            larga.scene.objects.len(),
            OBJETOS,
            "la isla larga cambio el conteo del nivel"
        );
    }

    #[test]
    fn la_isla_se_alarga_hacia_la_base_y_conserva_su_canto_oeste() {
        let ancha = nivel_isla_de_borde_ancha();
        let larga = nivel_isla_larga_con_escalera();

        let elegida = masa_de_la_segunda_isla(&nivel_apoyado().scene);
        let antes = ancha.scene.objects[elegida].primitive.bounds();
        let ahora = larga.scene.objects[elegida].primitive.bounds();

        assert!(
            ahora.max.z > antes.max.z,
            "la isla no se alargo: z max {:.2} -> {:.2}",
            antes.max.z,
            ahora.max.z
        );
        assert!(
            (ahora.min.z - antes.min.z).abs() < EPS,
            "la isla se movio de su canto sur"
        );
        assert!(
            (ahora.min.x - antes.min.x).abs() < EPS && (ahora.max.x - antes.max.x).abs() < EPS,
            "la isla cambio de anchura"
        );
    }

    #[test]
    fn el_solape_solo_es_con_la_base_principal() {
        let larga = nivel_isla_larga_con_escalera();

        let elegida = masa_de_la_segunda_isla(&nivel_apoyado().scene);
        let isla = larga.scene.objects[elegida].primitive.bounds();
        let principal = larga.scene.objects[masa_pedestal(&nivel_apoyado().scene)]
            .primitive
            .bounds();

        // Con la base **si** puede solapar: es lo que la decision autoriza.
        assert!(
            separacion_xz(&isla, &principal) < 0.0,
            "la isla no llego a tocar la base: el permiso de solape no se uso"
        );

        // Con nadie mas.
        for (nombre, grupo) in [
            ("el Monolito", SpatialGroupId::Monolith),
            ("Praderas", SpatialGroupId::Meadows),
            ("Aguas", SpatialGroupId::FlyingWaters),
        ] {
            let otra = huella_del_grupo(&larga.scene, grupo);

            assert!(
                separacion_xz(&isla, &otra) >= CLARO_MINIMO,
                "la isla deja {:.3} a {nombre}",
                separacion_xz(&isla, &otra)
            );

            let cortan = isla.min.x < otra.max.x
                && isla.max.x > otra.min.x
                && isla.min.y < otra.max.y
                && isla.max.y > otra.min.y
                && isla.min.z < otra.max.z
                && isla.max.z > otra.min.z;

            assert!(!cortan, "la isla corta {nombre}");
        }

        let plinto = huella_del_grupo(&larga.scene, SpatialGroupId::Global);

        assert!(
            isla.min.x >= plinto.min.x
                && isla.max.x <= plinto.max.x
                && isla.min.z >= plinto.min.z
                && isla.max.z <= plinto.max.z,
            "la isla se sale del plinto"
        );
    }

    #[test]
    fn la_escalera_es_pequena_y_el_macizo_mayoritario() {
        let apoyado = nivel_apoyado();
        let larga = nivel_isla_larga_con_escalera();

        let escalones = cimas_seleccionadas(&apoyado.scene);

        assert!(
            escalones.len() <= ESCALERA_MAXIMO,
            "la escalera se lleva {} prismas",
            escalones.len()
        );

        let piezas = indices_del_grupo(&larga, SpatialGroupId::Breakwater);
        let macizo: Vec<usize> = piezas[..PILARES]
            .iter()
            .copied()
            .filter(|i| !escalones.contains(i))
            .collect();

        assert!(
            macizo.len() >= MACIZO_DE_LA_ISLA,
            "el macizo se quedo en {} prismas",
            macizo.len()
        );

        // El macizo no se mueve de donde `grounded` lo dejo.
        for &i in &macizo {
            assert_eq!(
                apoyado.scene.objects[i].primitive.bounds(),
                larga.scene.objects[i].primitive.bounds(),
                "la pieza {i} del macizo se movio"
            );
        }

        // `R-02` y `R-03` tampoco.
        for &i in &piezas[PILARES..] {
            assert_eq!(
                apoyado.scene.objects[i].primitive.bounds(),
                larga.scene.objects[i].primitive.bounds(),
                "la pieza {i} de R-02/R-03 se movio"
            );
        }
    }

    #[test]
    fn la_escalera_se_apoya_en_la_isla() {
        let apoyado = nivel_apoyado();
        let larga = nivel_isla_larga_con_escalera();

        let elegida = masa_de_la_segunda_isla(&apoyado.scene);
        let isla = larga.scene.objects[elegida].primitive.bounds();

        for &i in &cimas_seleccionadas(&apoyado.scene) {
            let caja = larga.scene.objects[i].primitive.bounds();

            // La huella entera cae dentro de la isla.
            assert!(
                isla.min.x <= caja.min.x
                    && isla.max.x >= caja.max.x
                    && isla.min.z <= caja.min.z
                    && isla.max.z >= caja.max.z,
                "el escalon {i} no cabe en la isla"
            );

            // Y se posa en ella.
            let techo = pedestal_bajo(&larga.scene, i)
                .unwrap_or_else(|| panic!("el escalon {i} no tiene nada bajo su huella"));

            assert!(
                (techo - isla.max.y).abs() < EPS,
                "el escalon {i} se apoya en algo a {techo:.3} y la isla remata en {:.3}",
                isla.max.y
            );
            assert!(
                caja.min.y <= techo && caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "el escalon {i} arranca en {:.3} sobre un pedestal de {techo:.3}",
                caja.min.y
            );
        }

        // Y todo el Rompeolas sigue apoyado.
        for &i in &indices_del_grupo(&larga, SpatialGroupId::Breakwater)[..PILARES] {
            let caja = larga.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&larga.scene, i)
                .unwrap_or_else(|| panic!("la pieza {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo && caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "la pieza {i} arranca en {:.3} sobre un pedestal de {techo:.3}",
                caja.min.y
            );
        }
    }

    #[test]
    fn la_escalera_sube_de_forma_irregular_sin_competir_con_el_monolito() {
        let apoyado = nivel_apoyado();
        let larga = nivel_isla_larga_con_escalera();

        let mut orden = cimas_seleccionadas(&apoyado.scene);
        orden.sort_by(|a, b| {
            centro_xz(&larga.scene.objects[*a].primitive.bounds())
                .1
                .partial_cmp(&centro_xz(&larga.scene.objects[*b].primitive.bounds()).1)
                .expect("no hay NaN")
        });

        let techos: Vec<f32> = orden
            .iter()
            .map(|&i| larga.scene.objects[i].primitive.bounds().max.y)
            .collect();

        let n = techos.len() / 3;
        let media = |v: &[f32]| v.iter().sum::<f32>() / v.len() as f32;

        assert!(
            media(&techos[techos.len() - n..]) - media(&techos[..n]) >= 1.50,
            "el tercio bajo remata en {:.2} y el alto en {:.2}: no sube",
            media(&techos[..n]),
            media(&techos[techos.len() - n..])
        );

        let inversiones = techos.windows(2).filter(|par| par[1] < par[0]).count();

        assert!(
            inversiones >= 1 && inversiones < techos.len() - 1,
            "la escalera va en {inversiones} bajadas locales: es una rampa"
        );

        // No compite con el Monolito.
        let monolito = huella_del_grupo(&larga.scene, SpatialGroupId::Monolith);
        let cima = techos.iter().copied().fold(f32::MIN, f32::max);

        assert!(
            cima <= monolito.max.y * FRACCION_DEL_MONOLITO,
            "la escalera remata en {cima:.2} y el Monolito en {:.2}",
            monolito.max.y
        );
    }

    #[test]
    fn las_zonas_no_se_mueven_con_la_isla_larga() {
        let apoyado = nivel_apoyado();
        let larga = nivel_isla_larga_con_escalera();

        for grupo in [
            SpatialGroupId::Monolith,
            SpatialGroupId::Meadows,
            SpatialGroupId::FlyingWaters,
            SpatialGroupId::Global,
        ] {
            for &i in &indices_del_grupo(&larga, grupo) {
                assert_eq!(
                    apoyado.scene.objects[i].primitive.bounds(),
                    larga.scene.objects[i].primitive.bounds(),
                    "la pieza {i} de {grupo:?} se movio"
                );
            }
        }

        assert_eq!(
            apoyado.scene.objects[masa_pedestal(&apoyado.scene)]
                .primitive
                .bounds(),
            larga.scene.objects[masa_pedestal(&apoyado.scene)]
                .primitive
                .bounds(),
            "la base principal cambio"
        );

        for i in 0..OBJETOS {
            let a = &apoyado.scene.objects[i];
            let b = &larga.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");
        }
    }

    #[test]
    fn la_isla_larga_es_determinista() {
        let uno = nivel_isla_larga_con_escalera();
        let dos = nivel_isla_larga_con_escalera();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
    }

    #[test]
    fn los_veintidos_niveles_previos_no_cambian_con_la_isla_larga() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
            nivel_plataformas_marcadas(),
            nivel_soporte_modular(),
            nivel_segunda_isla(),
            nivel_isla_de_borde(),
            nivel_isla_de_borde_ancha(),
        ];

        let _larga = nivel_isla_larga_con_escalera();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
            nivel_plataformas_marcadas(),
            nivel_soporte_modular(),
            nivel_segunda_isla(),
            nivel_isla_de_borde(),
            nivel_isla_de_borde_ancha(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }
    }

    // ------------------------------------------------------------------
    // Terraza de transición: `connected_island_stair`
    // ------------------------------------------------------------------

    #[test]
    fn la_transicion_conserva_el_conteo() {
        let unida = nivel_isla_larga_con_transicion();

        assert_eq!(
            unida.scene.objects.len(),
            OBJETOS,
            "la transicion cambio el conteo del nivel"
        );
    }

    #[test]
    fn la_isla_la_escalera_y_el_macizo_quedan_intactos() {
        let larga = nivel_isla_larga_con_escalera();
        let unida = nivel_isla_larga_con_transicion();

        let conector = masa_de_la_transicion(&nivel_apoyado().scene);

        for i in 0..OBJETOS {
            let a = &larga.scene.objects[i];
            let b = &unida.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            if a.primitive.bounds() != b.primitive.bounds() {
                assert_eq!(i, conector, "la pieza {i} cambio y no es el conector");
            }
        }
    }

    #[test]
    fn el_conector_es_otra_masa_de_g02() {
        let apoyado = nivel_apoyado();
        let unida = nivel_isla_larga_con_transicion();

        let conector = masa_de_la_transicion(&apoyado.scene);

        assert_ne!(
            conector,
            masa_pedestal(&apoyado.scene),
            "el conector es la propia base"
        );
        assert_ne!(
            conector,
            masa_de_la_segunda_isla(&apoyado.scene),
            "el conector es la propia isla"
        );
        assert_eq!(
            unida.scene.objects[conector].spatial_group,
            SpatialGroupId::ContinentBackground,
            "el conector no es una masa de G-02"
        );
    }

    #[test]
    fn el_conector_encadena_la_base_con_la_isla() {
        // Continuidad geometrica: el conector **toca las dos**. Sin eso
        // seria una tercera pieza suelta, no una transicion.
        let apoyado = nivel_apoyado();
        let unida = nivel_isla_larga_con_transicion();

        let caja = unida.scene.objects[masa_de_la_transicion(&apoyado.scene)]
            .primitive
            .bounds();
        let base = unida.scene.objects[masa_pedestal(&apoyado.scene)]
            .primitive
            .bounds();
        let isla = unida.scene.objects[masa_de_la_segunda_isla(&apoyado.scene)]
            .primitive
            .bounds();

        for (nombre, otra) in [("la base", base), ("la isla", isla)] {
            assert!(
                separacion_xz(&caja, &otra) < 0.0,
                "el conector no llega a tocar {nombre}: {:.3}",
                separacion_xz(&caja, &otra)
            );
        }
    }

    #[test]
    fn el_conector_no_es_un_puente_regular() {
        // Otra cota y otra orientacion que las dos piezas que une: si
        // midiera y rematara igual, seria un trozo mas de la misma losa.
        let apoyado = nivel_apoyado();
        let unida = nivel_isla_larga_con_transicion();

        let caja = unida.scene.objects[masa_de_la_transicion(&apoyado.scene)]
            .primitive
            .bounds();
        let base = unida.scene.objects[masa_pedestal(&apoyado.scene)]
            .primitive
            .bounds();
        let isla = unida.scene.objects[masa_de_la_segunda_isla(&apoyado.scene)]
            .primitive
            .bounds();

        assert!(
            (caja.max.y - base.max.y).abs() > EPS && (caja.max.y - isla.max.y).abs() > EPS,
            "el conector remata a la misma cota que la base o la isla: {:.2}",
            caja.max.y
        );

        // La isla es larga en `z`; el conector, en `x`. Orientaciones
        // distintas.
        let alargado_en_z = |c: &Aabb| (c.max.z - c.min.z) > (c.max.x - c.min.x);

        assert!(alargado_en_z(&isla), "la isla dejo de ser larga en z");
        assert!(
            !alargado_en_z(&caja),
            "el conector tambien es largo en z: repite la forma de la isla"
        );
    }

    #[test]
    fn el_conector_no_invade_nada() {
        let apoyado = nivel_apoyado();
        let unida = nivel_isla_larga_con_transicion();

        let caja = unida.scene.objects[masa_de_la_transicion(&apoyado.scene)]
            .primitive
            .bounds();
        let plinto = huella_del_grupo(&unida.scene, SpatialGroupId::Global);

        assert!(
            caja.min.x >= plinto.min.x
                && caja.max.x <= plinto.max.x
                && caja.min.z >= plinto.min.z
                && caja.max.z <= plinto.max.z,
            "el conector se sale del plinto"
        );

        let corta = |otra: &Aabb| {
            caja.min.x < otra.max.x
                && caja.max.x > otra.min.x
                && caja.min.y < otra.max.y
                && caja.max.y > otra.min.y
                && caja.min.z < otra.max.z
                && caja.max.z > otra.min.z
        };

        for (nombre, grupo) in [
            ("el Monolito", SpatialGroupId::Monolith),
            ("Praderas", SpatialGroupId::Meadows),
            ("Aguas", SpatialGroupId::FlyingWaters),
        ] {
            assert!(
                !corta(&huella_del_grupo(&unida.scene, grupo)),
                "el conector corta {nombre}"
            );
        }

        // El Rompeolas se comprueba **pieza a pieza**: desde que seis de sus
        // prismas subieron a la isla, la caja del grupo abarca media escena
        // y decir que algo no la corta no significa nada.
        for &i in &indices_del_grupo(&unida, SpatialGroupId::Breakwater) {
            assert!(
                !corta(&unida.scene.objects[i].primitive.bounds()),
                "el conector corta la pieza {i} del Rompeolas"
            );
        }

        // Y no cierra el claro que la isla le deja a Praderas ni al
        // Monolito.
        for (nombre, grupo) in [
            ("el Monolito", SpatialGroupId::Monolith),
            ("Praderas", SpatialGroupId::Meadows),
            ("Aguas", SpatialGroupId::FlyingWaters),
        ] {
            let claro = separacion_xz(&caja, &huella_del_grupo(&unida.scene, grupo));

            assert!(
                claro >= CLARO_MINIMO,
                "el conector deja {claro:.3} a {nombre}"
            );
        }
    }

    #[test]
    fn cada_pieza_sigue_apoyada_con_la_transicion() {
        let unida = nivel_isla_larga_con_transicion();
        let piezas = indices_del_grupo(&unida, SpatialGroupId::Breakwater);

        for &i in &piezas[..PILARES] {
            let caja = unida.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&unida.scene, i)
                .unwrap_or_else(|| panic!("la pieza {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo && caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "la pieza {i} arranca en {:.3} sobre un pedestal de {techo:.3}",
                caja.min.y
            );
        }
    }

    #[test]
    fn la_transicion_es_determinista() {
        let uno = nivel_isla_larga_con_transicion();
        let dos = nivel_isla_larga_con_transicion();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
    }

    #[test]
    fn los_veintitres_niveles_previos_no_cambian_con_la_transicion() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
            nivel_plataformas_marcadas(),
            nivel_soporte_modular(),
            nivel_segunda_isla(),
            nivel_isla_de_borde(),
            nivel_isla_de_borde_ancha(),
            nivel_isla_larga_con_escalera(),
        ];

        let _unida = nivel_isla_larga_con_transicion();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
            nivel_plataformas_marcadas(),
            nivel_soporte_modular(),
            nivel_segunda_isla(),
            nivel_isla_de_borde(),
            nivel_isla_de_borde_ancha(),
            nivel_isla_larga_con_escalera(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }
    }

    // ------------------------------------------------------------------
    // Escalera en racimos: `clustered_stair`
    // ------------------------------------------------------------------

    /// A qué distancia dos escalones cuentan como del mismo racimo.
    const RACIMO_DE_ESCALONES: f32 = 2.20;

    #[test]
    fn la_escalera_agrupada_conserva_el_conteo() {
        let agrupada = nivel_escalera_agrupada();

        assert_eq!(
            agrupada.scene.objects.len(),
            OBJETOS,
            "la escalera agrupada cambio el conteo del nivel"
        );
    }

    #[test]
    fn solo_los_seis_escalones_cambian_al_agruparlos() {
        let unida = nivel_isla_larga_con_transicion();
        let agrupada = nivel_escalera_agrupada();

        let escalones = cimas_seleccionadas(&nivel_apoyado().scene);

        for i in 0..OBJETOS {
            let a = &unida.scene.objects[i];
            let b = &agrupada.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            if a.primitive.bounds() != b.primitive.bounds() {
                assert!(
                    escalones.contains(&i),
                    "la pieza {i} cambio y no es un escalon"
                );
            }
        }
    }

    #[test]
    fn las_tres_plataformas_quedan_exactas_bajo_la_escalera_agrupada() {
        let apoyado = nivel_apoyado();
        let unida = nivel_isla_larga_con_transicion();
        let agrupada = nivel_escalera_agrupada();

        for (nombre, i) in [
            ("la base", masa_pedestal(&apoyado.scene)),
            ("la isla", masa_de_la_segunda_isla(&apoyado.scene)),
            ("el conector", masa_de_la_transicion(&apoyado.scene)),
        ] {
            assert_eq!(
                unida.scene.objects[i].primitive.bounds(),
                agrupada.scene.objects[i].primitive.bounds(),
                "{nombre} cambio"
            );
        }
    }

    #[test]
    fn el_primer_racimo_pisa_la_terraza_conectora() {
        let apoyado = nivel_apoyado();
        let agrupada = nivel_escalera_agrupada();

        let conector = agrupada.scene.objects[masa_de_la_transicion(&apoyado.scene)]
            .primitive
            .bounds();

        let sobre_el_conector = cimas_seleccionadas(&apoyado.scene)
            .iter()
            .filter(|&&i| {
                let caja = agrupada.scene.objects[i].primitive.bounds();

                conector.min.x <= caja.min.x
                    && conector.max.x >= caja.max.x
                    && conector.min.z <= caja.min.z
                    && conector.max.z >= caja.max.z
            })
            .count();

        assert!(
            sobre_el_conector >= 2,
            "solo {sobre_el_conector} escalones caben en la terraza conectora: el primer \
             racimo no nace en ella"
        );

        // Y de verdad se apoyan en ella, no en lo que haya debajo.
        for &i in &cimas_seleccionadas(&apoyado.scene) {
            let caja = agrupada.scene.objects[i].primitive.bounds();

            let dentro = conector.min.x <= caja.min.x
                && conector.max.x >= caja.max.x
                && conector.min.z <= caja.min.z
                && conector.max.z >= caja.max.z;

            if dentro {
                let techo = pedestal_bajo(&agrupada.scene, i).expect("tiene pedestal");

                assert!(
                    (techo - conector.max.y).abs() < EPS,
                    "el escalon {i} esta sobre el conector pero se apoya a {techo:.3}"
                );
            }
        }
    }

    #[test]
    fn ningun_escalon_queda_aislado() {
        let apoyado = nivel_apoyado();
        let agrupada = nivel_escalera_agrupada();

        let escalones = cimas_seleccionadas(&apoyado.scene);
        let centros: Vec<(f32, f32)> = escalones
            .iter()
            .map(|&i| centro_xz(&agrupada.scene.objects[i].primitive.bounds()))
            .collect();

        for (k, &i) in escalones.iter().enumerate() {
            let vecino = centros
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != k)
                .map(|(_, c)| ((centros[k].0 - c.0).powi(2) + (centros[k].1 - c.1).powi(2)).sqrt())
                .fold(f32::MAX, f32::min);

            assert!(
                vecino <= RACIMO_DE_ESCALONES,
                "el escalon {i} tiene su vecino mas cercano a {vecino:.2}: es un poste suelto"
            );
        }

        // Y los racimos tienen mas de uno cada uno.
        let racimos = racimos_por_cercania(&centros, RACIMO_DE_ESCALONES);

        assert!(racimos.len() >= 2, "no hay racimos, hay {}", racimos.len());

        for racimo in &racimos {
            assert!(
                racimo.len() >= 2,
                "un racimo se quedo con {} escalon",
                racimo.len()
            );
        }
    }

    #[test]
    fn la_escalera_agrupada_sube_con_una_bajada() {
        let apoyado = nivel_apoyado();
        let agrupada = nivel_escalera_agrupada();

        let mut orden = cimas_seleccionadas(&apoyado.scene);
        orden.sort_by(|a, b| {
            centro_xz(&agrupada.scene.objects[*a].primitive.bounds())
                .1
                .partial_cmp(&centro_xz(&agrupada.scene.objects[*b].primitive.bounds()).1)
                .expect("no hay NaN")
        });

        let techos: Vec<f32> = orden
            .iter()
            .map(|&i| agrupada.scene.objects[i].primitive.bounds().max.y)
            .collect();

        let n = techos.len() / 3;
        let media = |v: &[f32]| v.iter().sum::<f32>() / v.len() as f32;

        assert!(
            media(&techos[techos.len() - n..]) - media(&techos[..n]) >= 1.50,
            "el tercio bajo remata en {:.2} y el alto en {:.2}: no sube",
            media(&techos[..n]),
            media(&techos[techos.len() - n..])
        );

        let bajadas = techos.windows(2).filter(|par| par[1] < par[0]).count();

        assert!(
            bajadas >= 1 && bajadas < techos.len() - 1,
            "la escalera va en {bajadas} bajadas locales"
        );

        // Sigue sin competir con el Monolito.
        let monolito = huella_del_grupo(&agrupada.scene, SpatialGroupId::Monolith);
        let cima = techos.iter().copied().fold(f32::MIN, f32::max);

        assert!(
            cima <= monolito.max.y * FRACCION_DEL_MONOLITO,
            "la escalera remata en {cima:.2} y el Monolito en {:.2}",
            monolito.max.y
        );
    }

    #[test]
    fn cada_escalon_agrupado_tiene_soporte_y_no_invade() {
        let apoyado = nivel_apoyado();
        let agrupada = nivel_escalera_agrupada();

        for &i in &indices_del_grupo(&agrupada, SpatialGroupId::Breakwater)[..PILARES] {
            let caja = agrupada.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&agrupada.scene, i)
                .unwrap_or_else(|| panic!("la pieza {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo && caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "la pieza {i} arranca en {:.3} sobre un pedestal de {techo:.3}",
                caja.min.y
            );
        }

        for &i in &cimas_seleccionadas(&apoyado.scene) {
            let caja = agrupada.scene.objects[i].primitive.bounds();

            for (nombre, grupo) in [
                ("el Monolito", SpatialGroupId::Monolith),
                ("Praderas", SpatialGroupId::Meadows),
                ("Aguas", SpatialGroupId::FlyingWaters),
            ] {
                let otra = huella_del_grupo(&agrupada.scene, grupo);

                assert!(
                    separacion_xz(&caja, &otra) >= CLARO_MINIMO,
                    "el escalon {i} deja {:.3} a {nombre}",
                    separacion_xz(&caja, &otra)
                );
            }
        }
    }

    #[test]
    fn la_escalera_agrupada_es_determinista() {
        let uno = nivel_escalera_agrupada();
        let dos = nivel_escalera_agrupada();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
    }

    #[test]
    fn los_veinticuatro_niveles_previos_no_cambian_con_la_escalera_agrupada() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
            nivel_plataformas_marcadas(),
            nivel_soporte_modular(),
            nivel_segunda_isla(),
            nivel_isla_de_borde(),
            nivel_isla_de_borde_ancha(),
            nivel_isla_larga_con_escalera(),
            nivel_isla_larga_con_transicion(),
        ];

        let _agrupada = nivel_escalera_agrupada();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
            nivel_plataformas_marcadas(),
            nivel_soporte_modular(),
            nivel_segunda_isla(),
            nivel_isla_de_borde(),
            nivel_isla_de_borde_ancha(),
            nivel_isla_larga_con_escalera(),
            nivel_isla_larga_con_transicion(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }
    }

    // ------------------------------------------------------------------
    // Montaña en la isla: `island_mountain`
    // ------------------------------------------------------------------

    /// Cuántos prismas tiene que conservar el macizo basal para que se siga
    /// leyendo como una masa y no como un resto.
    ///
    /// La mitad de `R-01`. Por debajo de eso la parcela del Rompeolas deja
    /// de tener un bloque y pasa a tener escombro.
    const MACIZO_BASAL_MINIMO: usize = 14;

    /// A qué distancia dos prismas del macizo cuentan como del mismo bloque.
    const RACIMO_DEL_MACIZO: f32 = 2.20;

    #[test]
    fn la_montana_en_isla_conserva_el_conteo() {
        let montanosa = nivel_montana_en_isla();

        assert_eq!(
            montanosa.scene.objects.len(),
            OBJETOS,
            "la montana en isla cambio el conteo del nivel"
        );
    }

    #[test]
    fn solo_los_doce_prismas_cambian_al_formar_la_montana() {
        let unida = nivel_isla_larga_con_transicion();
        let montanosa = nivel_montana_en_isla();

        let doce = prismas_de_la_montana(&nivel_apoyado().scene);

        for i in 0..OBJETOS {
            let a = &unida.scene.objects[i];
            let b = &montanosa.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            if a.primitive.bounds() != b.primitive.bounds() {
                assert!(
                    doce.contains(&i),
                    "la pieza {i} cambio y no es de la montana"
                );
            }
        }
    }

    #[test]
    fn las_tres_plataformas_quedan_exactas_bajo_la_montana() {
        let apoyado = nivel_apoyado();
        let unida = nivel_isla_larga_con_transicion();
        let montanosa = nivel_montana_en_isla();

        for (nombre, i) in [
            ("la base", masa_pedestal(&apoyado.scene)),
            ("la isla", masa_de_la_segunda_isla(&apoyado.scene)),
            ("el conector", masa_de_la_transicion(&apoyado.scene)),
        ] {
            assert_eq!(
                unida.scene.objects[i].primitive.bounds(),
                montanosa.scene.objects[i].primitive.bounds(),
                "{nombre} cambio"
            );
        }
    }

    #[test]
    fn la_montana_se_agrupa_en_tres_racimos_sin_postes() {
        let montanosa = nivel_montana_en_isla();
        let racimos = racimos_de_la_montana(&montanosa);

        assert_eq!(
            racimos.len(),
            3,
            "la montana salio en {} racimos y no en tres",
            racimos.len()
        );

        for racimo in &racimos {
            assert!(
                racimo.len() >= 2,
                "un racimo se quedo con {} prisma: es un poste suelto",
                racimo.len()
            );
        }
    }

    #[test]
    fn el_pie_es_mas_denso_que_el_centro_y_la_salida() {
        let [pie, centro, salida] = zonas_de_la_montana(&nivel_montana_en_isla());

        assert!(
            pie.len() > centro.len(),
            "el pie lleva {} prismas y el centro {}: no mengua",
            pie.len(),
            centro.len()
        );
        assert!(
            pie.len() > salida.len(),
            "el pie lleva {} prismas y la salida {}",
            pie.len(),
            salida.len()
        );
        assert!(
            salida.len() >= 2,
            "la salida se quedo con {} prisma",
            salida.len()
        );
    }

    #[test]
    fn la_altura_media_crece_del_pie_al_centro() {
        let montanosa = nivel_montana_en_isla();
        let [pie, centro, _salida] = zonas_de_la_montana(&montanosa);

        let medio = |zona: &[usize]| -> f32 {
            zona.iter()
                .map(|&i| montanosa.scene.objects[i].primitive.bounds().max.y)
                .sum::<f32>()
                / zona.len() as f32
        };

        assert!(
            medio(&centro) - medio(&pie) >= 2.00,
            "el pie remata de media en {:.2} y el centro en {:.2}: no gana altura",
            medio(&pie),
            medio(&centro)
        );
    }

    #[test]
    fn la_cima_cae_en_la_espina_de_la_isla() {
        let apoyado = nivel_apoyado();
        let montanosa = nivel_montana_en_isla();

        let [_pie, centro, _salida] = zonas_de_la_montana(&montanosa);
        let doce = prismas_de_la_montana(&apoyado.scene);
        let techo = |i: usize| montanosa.scene.objects[i].primitive.bounds().max.y;

        let cima = doce
            .iter()
            .copied()
            .fold(doce[0], |m, i| if techo(i) > techo(m) { i } else { m });

        assert!(
            centro.contains(&cima),
            "el prisma mas alto es el {cima} y no esta en el racimo central"
        );

        let isla = montanosa.scene.objects[masa_de_la_segunda_isla(&apoyado.scene)]
            .primitive
            .bounds();
        let (x, z) = centro_xz(&montanosa.scene.objects[cima].primitive.bounds());

        let fondo = isla.max.z - isla.min.z;

        assert!(
            z >= isla.min.z + fondo / 3.0 && z <= isla.max.z - fondo / 3.0,
            "la cima cae en z {z:.2}, fuera del tercio central [{:.2},{:.2}] de la isla",
            isla.min.z + fondo / 3.0,
            isla.max.z - fondo / 3.0
        );

        let ancho = isla.max.x - isla.min.x;

        assert!(
            x >= isla.min.x + ancho * 0.25 && x <= isla.max.x - ancho * 0.25,
            "la cima cae en x {x:.2}, pegada a un canto de la isla"
        );

        let monolito = huella_del_grupo(&montanosa.scene, SpatialGroupId::Monolith);

        assert!(
            techo(cima) <= monolito.max.y * FRACCION_DEL_MONOLITO,
            "la montana remata en {:.2} y el Monolito en {:.2}",
            techo(cima),
            monolito.max.y
        );
    }

    #[test]
    fn cada_prisma_de_la_montana_se_apoya_y_no_invade() {
        let apoyado = nivel_apoyado();
        let montanosa = nivel_montana_en_isla();

        for &i in &indices_del_grupo(&montanosa, SpatialGroupId::Breakwater)[..PILARES] {
            let caja = montanosa.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&montanosa.scene, i)
                .unwrap_or_else(|| panic!("la pieza {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo && caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "la pieza {i} arranca en {:.3} sobre un pedestal de {techo:.3}",
                caja.min.y
            );
        }

        // Y los doce que bajan se apoyan en la isla o en el conector, no en
        // cualquier cosa: su huella cabe entera en una de las dos.
        let isla = montanosa.scene.objects[masa_de_la_segunda_isla(&apoyado.scene)]
            .primitive
            .bounds();
        let conector = montanosa.scene.objects[masa_de_la_transicion(&apoyado.scene)]
            .primitive
            .bounds();

        let cabe = |caja: &Aabb, sobre: &Aabb| {
            sobre.min.x <= caja.min.x
                && sobre.max.x >= caja.max.x
                && sobre.min.z <= caja.min.z
                && sobre.max.z >= caja.max.z
        };

        for &i in &prismas_de_la_montana(&apoyado.scene) {
            let caja = montanosa.scene.objects[i].primitive.bounds();

            assert!(
                cabe(&caja, &isla) || cabe(&caja, &conector),
                "el prisma {i} no cae entero ni en la isla ni en el conector"
            );

            for (nombre, grupo) in [
                ("el Monolito", SpatialGroupId::Monolith),
                ("Praderas", SpatialGroupId::Meadows),
                ("Aguas", SpatialGroupId::FlyingWaters),
            ] {
                let otra = huella_del_grupo(&montanosa.scene, grupo);

                assert!(
                    separacion_xz(&caja, &otra) >= CLARO_MINIMO,
                    "el prisma {i} deja {:.3} a {nombre}",
                    separacion_xz(&caja, &otra)
                );
            }
        }
    }

    #[test]
    fn el_macizo_basal_sigue_siendo_reconocible() {
        let apoyado = nivel_apoyado();
        let unida = nivel_isla_larga_con_transicion();
        let montanosa = nivel_montana_en_isla();

        let macizo = macizo_de_la_montana(&apoyado.scene);
        let montana = prismas_de_la_montana(&apoyado.scene);

        assert!(
            macizo.len() >= MACIZO_BASAL_MINIMO,
            "el macizo se quedo con {} prismas",
            macizo.len()
        );
        assert!(
            macizo.len() > montana.len(),
            "bajan {} prismas y se quedan {}: el macizo deja de ser mayoria",
            montana.len(),
            macizo.len()
        );

        for &i in &macizo {
            assert_eq!(
                unida.scene.objects[i].primitive.bounds(),
                montanosa.scene.objects[i].primitive.bounds(),
                "el prisma {i} del macizo se movio"
            );
        }

        // Y sigue siendo un bloque, no piezas desperdigadas: su racimo mayor
        // guarda el minimo entero.
        let centros: Vec<(f32, f32)> = macizo
            .iter()
            .map(|&i| centro_xz(&montanosa.scene.objects[i].primitive.bounds()))
            .collect();

        let mayor = racimos_por_cercania(&centros, RACIMO_DEL_MACIZO)
            .iter()
            .map(|r| r.len())
            .max()
            .unwrap_or(0);

        assert!(
            mayor >= MACIZO_BASAL_MINIMO,
            "el macizo se deshizo: su bloque mayor tiene {mayor} prismas"
        );
    }

    #[test]
    fn la_montana_en_isla_es_determinista() {
        let uno = nivel_montana_en_isla();
        let dos = nivel_montana_en_isla();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
    }

    #[test]
    fn los_veinticinco_niveles_previos_no_cambian_con_la_montana() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
            nivel_plataformas_marcadas(),
            nivel_soporte_modular(),
            nivel_segunda_isla(),
            nivel_isla_de_borde(),
            nivel_isla_de_borde_ancha(),
            nivel_isla_larga_con_escalera(),
            nivel_isla_larga_con_transicion(),
            nivel_escalera_agrupada(),
        ];

        let _montanosa = nivel_montana_en_isla();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
            nivel_plataformas_marcadas(),
            nivel_soporte_modular(),
            nivel_segunda_isla(),
            nivel_isla_de_borde(),
            nivel_isla_de_borde_ancha(),
            nivel_isla_larga_con_escalera(),
            nivel_isla_larga_con_transicion(),
            nivel_escalera_agrupada(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }
    }

    // ------------------------------------------------------------------
    // Isla estilo spacious: `spacious_island`
    // ------------------------------------------------------------------

    /// Tolerancia al comparar las proporciones de la isla con las de
    /// `spacious`.
    const PROPORCION: f32 = 1.0e-3;

    /// Las masas de `G-02` que hacen de terraza sobre la isla, medidas: su
    /// huella cabe en la de la isla y rematan por encima de ella.
    fn terrazas_sobre_la_isla(diorama: &Blockout) -> Vec<usize> {
        let isla = diorama.scene.objects[masa_de_la_segunda_isla(&nivel_apoyado().scene)]
            .primitive
            .bounds();

        let mut terrazas: Vec<usize> =
            indices_del_grupo(diorama, SpatialGroupId::ContinentBackground)
                .into_iter()
                .filter(|&i| {
                    let c = diorama.scene.objects[i].primitive.bounds();

                    isla.min.x <= c.min.x
                        && isla.max.x >= c.max.x
                        && isla.min.z <= c.min.z
                        && isla.max.z >= c.max.z
                        && c.max.y > isla.max.y + EPS
                })
                .collect();

        // De mayor a menor huella: la primera es la terraza grande.
        let area = |i: usize| {
            let c = diorama.scene.objects[i].primitive.bounds();

            (c.max.x - c.min.x) * (c.max.z - c.min.z)
        };

        terrazas.sort_by(|a, b| area(*b).partial_cmp(&area(*a)).expect("no hay NaN"));

        terrazas
    }

    fn largo_y_corto(c: &Aabb) -> (f32, f32) {
        let x = c.max.x - c.min.x;
        let z = c.max.z - c.min.z;

        (x.max(z), x.min(z))
    }

    #[test]
    fn la_isla_estilo_spacious_conserva_el_conteo() {
        assert_eq!(
            nivel_isla_estilo_spacious().scene.objects.len(),
            OBJETOS,
            "la isla estilo spacious cambio el conteo del nivel"
        );
    }

    #[test]
    fn el_rompeolas_vuelve_exacto_a_grounded() {
        let apoyado = nivel_apoyado();
        let isla = nivel_isla_estilo_spacious();

        let piezas = indices_del_grupo(&apoyado, SpatialGroupId::Breakwater);

        assert_eq!(piezas.len(), SUSTITUIDOS + CONSERVADOS);
        assert_eq!(
            piezas,
            indices_del_grupo(&isla, SpatialGroupId::Breakwater),
            "el Rompeolas cambio de indices"
        );

        // R-01, R-02 y R-03: las treinta y ocho, caja y tipo.
        for &i in &piezas {
            let a = &apoyado.scene.objects[i];
            let b = &isla.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} del Rompeolas cambio de primitiva"
            );
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "la pieza {i} del Rompeolas no esta donde grounded la dejo"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
        }
    }

    #[test]
    fn monolito_aguas_praderas_y_base_siguen_exactos() {
        let apoyado = nivel_apoyado();
        let unida = nivel_isla_larga_con_transicion();
        let isla = nivel_isla_estilo_spacious();

        for grupo in [
            SpatialGroupId::Global,
            SpatialGroupId::Monolith,
            SpatialGroupId::Meadows,
            SpatialGroupId::FlyingWaters,
        ] {
            for &i in &indices_del_grupo(&apoyado, grupo) {
                assert_eq!(
                    apoyado.scene.objects[i].primitive.bounds(),
                    isla.scene.objects[i].primitive.bounds(),
                    "la pieza {i} de {grupo:?} cambio"
                );
            }
        }

        for (nombre, i) in [
            ("la base", masa_pedestal(&apoyado.scene)),
            ("la isla", masa_de_la_segunda_isla(&apoyado.scene)),
            ("el conector", masa_de_la_transicion(&apoyado.scene)),
        ] {
            assert_eq!(
                unida.scene.objects[i].primitive.bounds(),
                isla.scene.objects[i].primitive.bounds(),
                "{nombre} cambio respecto de connected_island_stair"
            );
        }
    }

    #[test]
    fn solo_cambian_el_rompeolas_restaurado_y_dos_masas_de_g02() {
        let apoyado = nivel_apoyado();
        let unida = nivel_isla_larga_con_transicion();
        let isla = nivel_isla_estilo_spacious();

        let rompeolas = indices_del_grupo(&apoyado, SpatialGroupId::Breakwater);
        let donantes = masas_tapadas(&unida.scene);
        let mut g02_movidas = 0usize;

        for i in 0..OBJETOS {
            let a = &unida.scene.objects[i];
            let b = &isla.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");

            if a.primitive.bounds() == b.primitive.bounds() {
                continue;
            }

            if rompeolas.contains(&i) {
                // Si se movio, es porque vuelve a grounded.
                assert_eq!(
                    b.primitive.bounds(),
                    apoyado.scene.objects[i].primitive.bounds(),
                    "la pieza {i} del Rompeolas se movio y no a grounded"
                );
            } else {
                assert!(
                    donantes.contains(&i),
                    "la pieza {i} cambio y no es ni del Rompeolas ni una masa tapada de G-02"
                );
                g02_movidas += 1;
            }
        }

        assert_eq!(g02_movidas, 2, "se movieron {g02_movidas} masas de G-02");
    }

    #[test]
    fn las_masas_reutilizadas_no_las_veia_nadie_desde_arriba() {
        let apoyado = nivel_apoyado();
        let unida = nivel_isla_larga_con_transicion();
        let donantes = masas_tapadas(&unida.scene);

        assert!(
            donantes.len() >= 2,
            "solo hay {} masas tapadas de G-02",
            donantes.len()
        );

        let intocables = [
            masa_pedestal(&apoyado.scene),
            masa_de_la_segunda_isla(&apoyado.scene),
            masa_de_la_transicion(&apoyado.scene),
        ];

        let impactos = impactos_por_objeto(&unida, &camara_cenital(&unida));

        for &i in &donantes {
            assert_eq!(
                unida.scene.objects[i].spatial_group,
                SpatialGroupId::ContinentBackground,
                "la masa {i} no es de G-02"
            );
            assert!(
                !intocables.contains(&i),
                "la masa {i} es la base, la isla o el conector"
            );
            assert_eq!(
                impactos[i], 0,
                "la masa {i} se veia desde arriba en connected_island_stair"
            );
        }
    }

    #[test]
    fn la_logica_de_spacious_se_lee_de_spacious() {
        let logica = logica_de_spacious(&nivel_espacioso().scene);

        for (nombre, r) in [
            ("largo de la terraza grande", logica.largo_t1),
            ("corto de la terraza grande", logica.corto_t1),
            ("largo de la terraza alta", logica.largo_t2),
            ("corto de la terraza alta", logica.corto_t2),
        ] {
            assert!(
                r > 0.20 && r < 0.95,
                "{nombre}: proporcion {r:.3}, no es una terraza sobre una masa"
            );
        }

        assert!(logica.salto_t1 > 0.0 && logica.salto_t2 > 0.0);
        assert!(
            logica.salto_t1 > logica.salto_t2,
            "spacious sube {:.2} y luego {:.2}: el salto grande va primero",
            logica.salto_t1,
            logica.salto_t2
        );
    }

    #[test]
    fn la_isla_sigue_la_composicion_de_spacious() {
        let apoyado = nivel_apoyado();
        let isla_spacious = nivel_isla_estilo_spacious();
        let logica = logica_de_spacious(&nivel_espacioso().scene);

        let masa = isla_spacious.scene.objects[masa_de_la_segunda_isla(&apoyado.scene)]
            .primitive
            .bounds();
        let terrazas = terrazas_sobre_la_isla(&isla_spacious);

        assert_eq!(
            terrazas.len(),
            2,
            "sobre la isla hay {} terrazas y no dos",
            terrazas.len()
        );

        let t1 = isla_spacious.scene.objects[terrazas[0]].primitive.bounds();
        let t2 = isla_spacious.scene.objects[terrazas[1]].primitive.bounds();

        // Masa principal reconocible: la isla sigue siendo la mayor.
        let area = |c: &Aabb| (c.max.x - c.min.x) * (c.max.z - c.min.z);

        assert!(
            area(&masa) > area(&t1) && area(&t1) > area(&t2),
            "las huellas no menguan: isla {:.2}, terraza {:.2}, alta {:.2}",
            area(&masa),
            area(&t1),
            area(&t2)
        );

        // Escalones anidados: la alta cabe en la grande.
        assert!(
            t1.min.x <= t2.min.x
                && t1.max.x >= t2.max.x
                && t1.min.z <= t2.min.z
                && t1.max.z >= t2.max.z,
            "la terraza alta no cabe en la grande"
        );

        // Las proporciones y los saltos, los de spacious.
        let (masa_l, masa_c) = largo_y_corto(&masa);
        let (t1_l, t1_c) = largo_y_corto(&t1);
        let (t2_l, t2_c) = largo_y_corto(&t2);

        for (nombre, medido, esperado) in [
            ("largo t1", t1_l / masa_l, logica.largo_t1),
            ("corto t1", t1_c / masa_c, logica.corto_t1),
            ("largo t2", t2_l / t1_l, logica.largo_t2),
            ("corto t2", t2_c / t1_c, logica.corto_t2),
            ("salto t1", t1.max.y - masa.max.y, logica.salto_t1),
            ("salto t2", t2.max.y - t1.max.y, logica.salto_t2),
        ] {
            assert!(
                (medido - esperado).abs() < PROPORCION,
                "{nombre}: la isla da {medido:.3} y spacious {esperado:.3}"
            );
        }

        // Y los ejes largos coinciden: la terraza sigue a la isla.
        assert_eq!(
            masa.max.z - masa.min.z >= masa.max.x - masa.min.x,
            t1.max.z - t1.min.z >= t1.max.x - t1.min.x,
            "la terraza grande va atravesada respecto de la isla"
        );
    }

    #[test]
    fn la_cima_de_la_isla_cae_en_su_espina() {
        let apoyado = nivel_apoyado();
        let isla_spacious = nivel_isla_estilo_spacious();

        let masa = isla_spacious.scene.objects[masa_de_la_segunda_isla(&apoyado.scene)]
            .primitive
            .bounds();
        let terrazas = terrazas_sobre_la_isla(&isla_spacious);
        let alta = terrazas
            .iter()
            .map(|&i| isla_spacious.scene.objects[i].primitive.bounds())
            .fold(masa, |m, c| if c.max.y > m.max.y { c } else { m });

        let (x, z) = centro_xz(&alta);
        let fondo = masa.max.z - masa.min.z;
        let ancho = masa.max.x - masa.min.x;

        assert!(
            z >= masa.min.z + fondo / 3.0 && z <= masa.max.z - fondo / 3.0,
            "la cima cae en z {z:.2}, fuera del tercio central de la isla"
        );
        assert!(
            x >= masa.min.x + ancho * 0.25 && x <= masa.max.x - ancho * 0.25,
            "la cima cae en x {x:.2}, pegada a un canto"
        );

        let monolito = huella_del_grupo(&isla_spacious.scene, SpatialGroupId::Monolith);

        assert!(alta.max.y < monolito.max.y * FRACCION_DEL_MONOLITO);
    }

    #[test]
    fn las_terrazas_se_apoyan_y_dejan_claro() {
        let apoyado = nivel_apoyado();
        let isla_spacious = nivel_isla_estilo_spacious();

        let masa = isla_spacious.scene.objects[masa_de_la_segunda_isla(&apoyado.scene)]
            .primitive
            .bounds();
        let terrazas = terrazas_sobre_la_isla(&isla_spacious);
        let t1 = isla_spacious.scene.objects[terrazas[0]].primitive.bounds();
        let t2 = isla_spacious.scene.objects[terrazas[1]].primitive.bounds();

        // Cada una se empotra en lo que la sostiene: ni flota ni se hunde de
        // mas. Sin caras coplanares con su soporte.
        for (nombre, terraza, soporte) in [("la grande", t1, masa), ("la alta", t2, t1)] {
            assert!(
                terraza.min.y < soporte.max.y && terraza.min.y >= soporte.max.y - EMPOTRADO_MAXIMO,
                "{nombre} arranca en {:.3} sobre un soporte de {:.3}",
                terraza.min.y,
                soporte.max.y
            );
            assert!(
                terraza.min.x > soporte.min.x
                    && terraza.max.x < soporte.max.x
                    && terraza.min.z > soporte.min.z
                    && terraza.max.z < soporte.max.z,
                "{nombre} toca el canto de su soporte"
            );
        }

        for &i in &terrazas {
            let caja = isla_spacious.scene.objects[i].primitive.bounds();

            for (nombre, otra) in [
                (
                    "el Monolito",
                    huella_del_grupo(&isla_spacious.scene, SpatialGroupId::Monolith),
                ),
                (
                    "Praderas",
                    huella_del_grupo(&isla_spacious.scene, SpatialGroupId::Meadows),
                ),
                (
                    "Aguas",
                    huella_del_grupo(&isla_spacious.scene, SpatialGroupId::FlyingWaters),
                ),
                (
                    "el Rompeolas",
                    huella_del_grupo(&isla_spacious.scene, SpatialGroupId::Breakwater),
                ),
            ] {
                assert!(
                    separacion_xz(&caja, &otra) >= CLARO_MINIMO,
                    "la terraza {i} deja {:.3} a {nombre}",
                    separacion_xz(&caja, &otra)
                );
            }
        }

        // Y el Rompeolas restaurado sigue apoyado, pieza a pieza.
        for &i in &indices_del_grupo(&isla_spacious, SpatialGroupId::Breakwater)[..PILARES] {
            let caja = isla_spacious.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&isla_spacious.scene, i)
                .unwrap_or_else(|| panic!("la pieza {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo && caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "la pieza {i} arranca en {:.3} sobre un pedestal de {techo:.3}",
                caja.min.y
            );
        }
    }

    #[test]
    fn la_isla_estilo_spacious_es_determinista() {
        let uno = nivel_isla_estilo_spacious();
        let dos = nivel_isla_estilo_spacious();

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
    }

    #[test]
    fn los_veintiseis_niveles_previos_no_cambian_con_la_isla_estilo_spacious() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
            nivel_plataformas_marcadas(),
            nivel_soporte_modular(),
            nivel_segunda_isla(),
            nivel_isla_de_borde(),
            nivel_isla_de_borde_ancha(),
            nivel_isla_larga_con_escalera(),
            nivel_isla_larga_con_transicion(),
            nivel_escalera_agrupada(),
            nivel_montana_en_isla(),
        ];

        let _isla = nivel_isla_estilo_spacious();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
            nivel_plataformas_marcadas(),
            nivel_soporte_modular(),
            nivel_segunda_isla(),
            nivel_isla_de_borde(),
            nivel_isla_de_borde_ancha(),
            nivel_isla_larga_con_escalera(),
            nivel_isla_larga_con_transicion(),
            nivel_escalera_agrupada(),
            nivel_montana_en_isla(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }
    }

    // ------------------------------------------------------------------
    // Segundo Rompeolas: `second_breakwater`
    // ------------------------------------------------------------------

    /// El conteo propio de este candidato: `154 - 2 + 10`.
    ///
    /// Es el único nivel del preview que cambia el conteo, y lo cambia por
    /// autorización explícita. Se escribe como la cuenta y no como el número
    /// para que el test diga de dónde sale.
    const OBJETOS_DEL_SEGUNDO_ROMPEOLAS: usize = OBJETOS - 2 + 10;

    /// Las piezas del Rompeolas original: `R-01`, `R-02` y `R-03`.
    const ROMPEOLAS_ORIGINAL: usize = SUSTITUIDOS + CONSERVADOS;

    /// Tolerancia al comparar medidas de la formación con las del primer
    /// Rompeolas.
    const MISMA_MEDIDA: f32 = 1.0e-3;

    fn cajas_del_grupo(diorama: &Blockout, grupo: SpatialGroupId) -> Vec<Aabb> {
        indices_del_grupo(diorama, grupo)
            .iter()
            .map(|&i| diorama.scene.objects[i].primitive.bounds())
            .collect()
    }

    /// Las filas de la formación, de sur a norte: prismas con el mismo
    /// centro en `z`.
    fn filas_de_la_formacion(diorama: &Blockout) -> Vec<Vec<usize>> {
        let mut piezas = segundo_rompeolas(&diorama.scene);
        let z = |i: usize| centro_xz(&diorama.scene.objects[i].primitive.bounds()).1;

        piezas.sort_by(|a, b| z(*a).partial_cmp(&z(*b)).expect("no hay NaN"));

        let mut filas: Vec<Vec<usize>> = Vec::new();

        for i in piezas {
            match filas.last_mut() {
                Some(fila) if (z(fila[0]) - z(i)).abs() < 0.05 => fila.push(i),
                _ => filas.push(vec![i]),
            }
        }

        filas
    }

    #[test]
    fn el_segundo_rompeolas_tiene_ciento_sesenta_y_dos_objetos() {
        assert_eq!(OBJETOS_DEL_SEGUNDO_ROMPEOLAS, 162);
        assert_eq!(
            nivel_segundo_rompeolas().scene.objects.len(),
            OBJETOS_DEL_SEGUNDO_ROMPEOLAS,
            "el segundo Rompeolas no queda en 154 - 2 + 10"
        );
    }

    #[test]
    fn se_retiran_exactamente_las_dos_masas_verdes_de_spacious_island() {
        let unida = nivel_isla_larga_con_transicion();
        let estilo = nivel_isla_estilo_spacious();
        let segundo = nivel_segundo_rompeolas();

        // Las dos que spacious_island convirtio en escalones: las unicas de
        // G-02 que cambian entre connected_island_stair y spacious_island.
        let escalones: Vec<usize> = indices_del_grupo(&unida, SpatialGroupId::ContinentBackground)
            .into_iter()
            .filter(|&i| {
                unida.scene.objects[i].primitive.bounds()
                    != estilo.scene.objects[i].primitive.bounds()
            })
            .collect();

        assert_eq!(escalones.len(), 2, "spacious_island no tenia dos escalones");

        // G-02 pierde esas dos y conserva el resto, en orden y exacto.
        let quedan: Vec<Aabb> = indices_del_grupo(&unida, SpatialGroupId::ContinentBackground)
            .into_iter()
            .filter(|i| !escalones.contains(i))
            .map(|i| unida.scene.objects[i].primitive.bounds())
            .collect();

        assert_eq!(
            cajas_del_grupo(&segundo, SpatialGroupId::ContinentBackground),
            quedan,
            "G-02 no es connected_island_stair menos las dos masas de los escalones"
        );

        // Y ninguna masa verde remata ya por encima de la isla.
        let isla = segundo.scene.objects[masa_de_la_segunda_isla(&nivel_apoyado().scene)]
            .primitive
            .bounds();

        for caja in cajas_del_grupo(&segundo, SpatialGroupId::ContinentBackground) {
            let dentro = isla.min.x <= caja.min.x
                && isla.max.x >= caja.max.x
                && isla.min.z <= caja.min.z
                && isla.max.z >= caja.max.z;

            assert!(
                !(dentro && caja.max.y > isla.max.y + EPS),
                "queda un escalon verde sobre la isla: {caja:?}"
            );
        }
    }

    #[test]
    fn el_rompeolas_original_queda_exacto() {
        let apoyado = nivel_apoyado();
        let segundo = nivel_segundo_rompeolas();

        let antes = indices_del_grupo(&apoyado, SpatialGroupId::Breakwater);
        let ahora = indices_del_grupo(&segundo, SpatialGroupId::Breakwater);

        assert_eq!(antes.len(), ROMPEOLAS_ORIGINAL);
        assert_eq!(ahora.len(), ROMPEOLAS_ORIGINAL + 10);

        for (&a, &b) in antes.iter().zip(&ahora[..ROMPEOLAS_ORIGINAL]) {
            let x = &apoyado.scene.objects[a];
            let y = &segundo.scene.objects[b];

            assert!(
                mismo_tipo(&x.primitive, &y.primitive),
                "la pieza {a} del Rompeolas original cambio de primitiva"
            );
            assert_eq!(
                x.primitive.bounds(),
                y.primitive.bounds(),
                "la pieza {a} del Rompeolas original no esta donde grounded la dejo"
            );
            assert_eq!(x.initial_material, y.initial_material, "pieza {a}");
            assert_eq!(x.final_material, y.final_material, "pieza {a}");
            assert_eq!(x.reveal_group, y.reveal_group, "pieza {a}");
        }
    }

    #[test]
    fn la_formacion_secundaria_son_diez_prismas_de_la_ruta() {
        let apoyado = nivel_apoyado();
        let segundo = nivel_segundo_rompeolas();

        let nuevos = segundo_rompeolas(&segundo.scene);

        assert_eq!(
            nuevos.len(),
            10,
            "la formacion tiene {} prismas",
            nuevos.len()
        );

        let r01 = &indices_del_grupo(&apoyado, SpatialGroupId::Breakwater)[..PILARES];
        let paleta_r01: Vec<_> = r01
            .iter()
            .map(|&i| {
                let o = &apoyado.scene.objects[i];

                (o.initial_material, o.final_material, o.reveal_group)
            })
            .collect();

        for &i in &nuevos {
            let o = &segundo.scene.objects[i];

            #[cfg(feature = "hex-prism")]
            assert!(
                matches!(o.primitive, Primitive::HexPrism(_)),
                "el prisma {i} no es HexPrism en la Ruta A"
            );
            #[cfg(not(feature = "hex-prism"))]
            assert!(
                matches!(o.primitive, Primitive::Cuboid(_)),
                "el prisma {i} no es Cuboid en la Ruta B"
            );

            assert_eq!(o.spatial_group, SpatialGroupId::Breakwater, "prisma {i}");
            assert!(
                paleta_r01.contains(&(o.initial_material, o.final_material, o.reveal_group)),
                "el prisma {i} no usa la paleta del primer Rompeolas"
            );
        }

        // Sin materiales ni texturas nuevos.
        assert_eq!(segundo.scene.palette.len(), apoyado.scene.palette.len());
        assert_eq!(segundo.scene.textures.len(), apoyado.scene.textures.len());
    }

    #[test]
    fn el_resto_de_la_escena_sigue_exacto() {
        let apoyado = nivel_apoyado();
        let unida = nivel_isla_larga_con_transicion();
        let segundo = nivel_segundo_rompeolas();

        for grupo in [
            SpatialGroupId::Global,
            SpatialGroupId::Monolith,
            SpatialGroupId::Meadows,
            SpatialGroupId::FlyingWaters,
        ] {
            assert_eq!(
                cajas_del_grupo(&apoyado, grupo),
                cajas_del_grupo(&segundo, grupo),
                "{grupo:?} cambio"
            );
        }

        // Base, isla y conector: los de connected_island_stair.
        for (nombre, i) in [
            ("la base", masa_pedestal(&apoyado.scene)),
            ("la isla", masa_de_la_segunda_isla(&apoyado.scene)),
            ("el conector", masa_de_la_transicion(&apoyado.scene)),
        ] {
            let caja = unida.scene.objects[i].primitive.bounds();

            assert!(
                cajas_del_grupo(&segundo, SpatialGroupId::ContinentBackground).contains(&caja),
                "{nombre} no sigue exacto"
            );
        }
    }

    #[test]
    fn cada_prisma_secundario_se_apoya_en_la_isla() {
        let segundo = nivel_segundo_rompeolas();
        let isla = segundo.scene.objects[masa_de_la_segunda_isla(&nivel_apoyado().scene)]
            .primitive
            .bounds();

        for &i in &segundo_rompeolas(&segundo.scene) {
            let caja = segundo.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&segundo.scene, i)
                .unwrap_or_else(|| panic!("el prisma {i} no tiene nada bajo su huella"));

            assert!(
                isla.min.x <= caja.min.x
                    && isla.max.x >= caja.max.x
                    && isla.min.z <= caja.min.z
                    && isla.max.z >= caja.max.z,
                "el prisma {i} no cae entero en la isla"
            );
            assert!(
                (techo - isla.max.y).abs() < EPS,
                "el prisma {i} se apoya a {techo:.3} y no en la isla"
            );
            assert!(
                caja.min.y < techo && caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "el prisma {i} arranca en {:.3} sobre un pedestal de {techo:.3}",
                caja.min.y
            );
        }

        // Y el Rompeolas original sigue apoyado, pieza a pieza.
        for &i in &indices_del_grupo(&segundo, SpatialGroupId::Breakwater)[..PILARES] {
            let caja = segundo.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&segundo.scene, i)
                .unwrap_or_else(|| panic!("la pieza {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo && caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "la pieza {i} arranca en {:.3} sobre un pedestal de {techo:.3}",
                caja.min.y
            );
        }
    }

    #[test]
    fn la_formacion_secundaria_es_seis_tres_uno() {
        let apoyado = nivel_apoyado();
        let segundo = nivel_segundo_rompeolas();

        let filas = filas_de_la_formacion(&segundo);
        let cuentas: Vec<usize> = filas.iter().map(|f| f.len()).collect();

        // De sur a norte: cima, racimo intermedio y las dos filas de la masa
        // basal.
        assert_eq!(cuentas, vec![1, 3, 3, 3], "las filas salen {cuentas:?}");

        let techo = |i: usize| segundo.scene.objects[i].primitive.bounds().max.y;
        let medio = |v: &[usize]| v.iter().map(|&i| techo(i)).sum::<f32>() / v.len() as f32;

        let cima = filas[0][0];
        let racimo = &filas[1];
        let basal: Vec<usize> = filas[2].iter().chain(&filas[3]).copied().collect();

        assert_eq!(basal.len(), 6);

        for &i in racimo.iter().chain(&basal) {
            assert!(
                techo(cima) > techo(i),
                "el prisma {i} remata en {:.2} y la cima en {:.2}",
                techo(i),
                techo(cima)
            );
        }

        assert!(
            medio(racimo) > medio(&basal),
            "el racimo remata de media en {:.2} y la masa basal en {:.2}",
            medio(racimo),
            medio(&basal)
        );

        // Es secundaria: su cima no pasa de la cresta del primer Rompeolas.
        let cresta = indices_del_grupo(&apoyado, SpatialGroupId::Breakwater)[..PILARES]
            .iter()
            .map(|&i| apoyado.scene.objects[i].primitive.bounds().max.y)
            .fold(f32::MIN, f32::max);

        assert!(
            techo(cima) < cresta,
            "la cima secundaria remata en {:.2} y la cresta original en {cresta:.2}",
            techo(cima)
        );

        // Y la cima cae en el tercio central de la isla.
        let isla = segundo.scene.objects[masa_de_la_segunda_isla(&apoyado.scene)]
            .primitive
            .bounds();
        let fondo = isla.max.z - isla.min.z;
        let z = centro_xz(&segundo.scene.objects[cima].primitive.bounds()).1;

        assert!(
            z >= isla.min.z + fondo / 3.0 && z <= isla.max.z - fondo / 3.0,
            "la cima cae en z {z:.2}, fuera del tercio central de la isla"
        );
    }

    #[test]
    fn la_formacion_secundaria_es_compacta() {
        let segundo = nivel_segundo_rompeolas();
        let centros: Vec<(f32, f32)> = segundo_rompeolas(&segundo.scene)
            .iter()
            .map(|&i| centro_xz(&segundo.scene.objects[i].primitive.bounds()))
            .collect();

        let racimos = racimos_por_cercania(&centros, RACIMO_DE_ESCALONES);

        assert_eq!(
            racimos.len(),
            1,
            "la formacion se parte en {} grupos",
            racimos.len()
        );

        for (k, c) in centros.iter().enumerate() {
            let vecino = centros
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != k)
                .map(|(_, o)| ((c.0 - o.0).powi(2) + (c.1 - o.1).powi(2)).sqrt())
                .fold(f32::MAX, f32::min);

            assert!(
                vecino <= 1.60,
                "el prisma {k} tiene su vecino a {vecino:.2}: es un poste"
            );
        }
    }

    #[test]
    fn la_formacion_secundaria_deja_claro() {
        let apoyado = nivel_apoyado();
        let segundo = nivel_segundo_rompeolas();

        let original = huella(
            &apoyado.scene,
            &indices_del_grupo(&apoyado, SpatialGroupId::Breakwater),
        );
        let conector = segundo.scene.objects[masa_de_la_transicion(&apoyado.scene)]
            .primitive
            .bounds();

        for &i in &segundo_rompeolas(&segundo.scene) {
            let caja = segundo.scene.objects[i].primitive.bounds();

            for (nombre, otra) in [
                (
                    "el Monolito",
                    huella_del_grupo(&segundo.scene, SpatialGroupId::Monolith),
                ),
                (
                    "Praderas",
                    huella_del_grupo(&segundo.scene, SpatialGroupId::Meadows),
                ),
                (
                    "Aguas",
                    huella_del_grupo(&segundo.scene, SpatialGroupId::FlyingWaters),
                ),
                ("el Rompeolas original", original),
            ] {
                assert!(
                    separacion_xz(&caja, &otra) >= CLARO_MINIMO,
                    "el prisma {i} deja {:.3} a {nombre}",
                    separacion_xz(&caja, &otra)
                );
            }

            assert!(
                separacion_xz(&caja, &conector) > 0.0,
                "el prisma {i} se mete en el conector"
            );
        }
    }

    #[test]
    fn la_formacion_reutiliza_el_lenguaje_del_primer_rompeolas() {
        let apoyado = nivel_apoyado();
        let segundo = nivel_segundo_rompeolas();

        let r01: Vec<Aabb> = indices_del_grupo(&apoyado, SpatialGroupId::Breakwater)[..PILARES]
            .iter()
            .map(|&i| apoyado.scene.objects[i].primitive.bounds())
            .collect();
        let nuevas: Vec<Aabb> = segundo_rompeolas(&segundo.scene)
            .iter()
            .map(|&i| segundo.scene.objects[i].primitive.bounds())
            .collect();

        // Los grosores son los de la macroformacion: cada ancho nuevo es uno
        // de los que ya tenia R-01.
        for caja in &nuevas {
            let ancho = caja.max.x - caja.min.x;

            assert!(
                r01.iter()
                    .any(|c| ((c.max.x - c.min.x) - ancho).abs() < MISMA_MEDIDA),
                "un prisma mide {ancho:.3} de ancho y ninguno de R-01 mide eso"
            );
        }

        // El paso de la reticula tambien: entre filas y dentro de una fila.
        let paso_x_r01 = (centro_xz(&r01[6]).0 - centro_xz(&r01[0]).0) / 6.0;
        let paso_z_r01 = centro_xz(&r01[7]).1 - centro_xz(&r01[0]).1;

        let filas = filas_de_la_formacion(&segundo);
        let c = |i: usize| centro_xz(&segundo.scene.objects[i].primitive.bounds());

        for par in filas.windows(2) {
            assert!(
                ((c(par[1][0]).1 - c(par[0][0]).1) - paso_z_r01).abs() < MISMA_MEDIDA,
                "las filas no van al paso de R-01"
            );
        }

        for fila in &filas {
            for par in fila.windows(2) {
                assert!(
                    ((c(par[1]).0 - c(par[0]).0).abs() - paso_x_r01).abs() < MISMA_MEDIDA,
                    "dentro de una fila los prismas no van al paso de R-01"
                );
            }
        }

        // Y no es una copia literal: ninguna caja coincide con una de R-01.
        for caja in &nuevas {
            assert!(!r01.contains(caja), "un prisma nuevo copia la caja de R-01");
        }
    }

    #[test]
    fn el_segundo_rompeolas_es_determinista() {
        let uno = nivel_segundo_rompeolas();
        let dos = nivel_segundo_rompeolas();

        assert_eq!(uno.scene.objects.len(), dos.scene.objects.len());

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
    }

    #[test]
    fn los_veintisiete_niveles_previos_no_cambian_con_el_segundo_rompeolas() {
        let previos = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
            nivel_plataformas_marcadas(),
            nivel_soporte_modular(),
            nivel_segunda_isla(),
            nivel_isla_de_borde(),
            nivel_isla_de_borde_ancha(),
            nivel_isla_larga_con_escalera(),
            nivel_isla_larga_con_transicion(),
            nivel_escalera_agrupada(),
            nivel_montana_en_isla(),
            nivel_isla_estilo_spacious(),
        ];

        let _segundo = nivel_segundo_rompeolas();

        let otra_vez = [
            nivel(),
            nivel_candidato(),
            nivel_espacioso(),
            nivel_conectado(),
            nivel_refinado(),
            nivel_asimetrico(),
            nivel_separado(),
            nivel_apoyado(),
            nivel_escalera(),
            nivel_combinado(),
            nivel_ascenso_desde_grounded(),
            nivel_relleno_trasero(),
            nivel_escalera_monolito(),
            nivel_rompeolas_de_borde(),
            nivel_terraza_verde_de_borde(),
            nivel_plataformas_de_borde(),
            nivel_plataformas_finas(),
            nivel_plataformas_marcadas(),
            nivel_soporte_modular(),
            nivel_segunda_isla(),
            nivel_isla_de_borde(),
            nivel_isla_de_borde_ancha(),
            nivel_isla_larga_con_escalera(),
            nivel_isla_larga_con_transicion(),
            nivel_escalera_agrupada(),
            nivel_montana_en_isla(),
            nivel_isla_estilo_spacious(),
        ];

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            assert_eq!(antes.scene.objects.len(), OBJETOS, "el nivel previo {n}");

            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }
    }

    // ------------------------------------------------------------------
    // Segundo Rompeolas invertido: `second_breakwater_turned`
    // ------------------------------------------------------------------

    /// Distancia en planta de cada prisma secundario a una caja, y su techo.
    fn distancias_y_techos(diorama: &Blockout, a: &Aabb) -> Vec<(usize, f32, f32)> {
        segundo_rompeolas(&diorama.scene)
            .into_iter()
            .map(|i| {
                let caja = diorama.scene.objects[i].primitive.bounds();

                (i, separacion_xz(&caja, a), caja.max.y)
            })
            .collect()
    }

    /// Covarianza entre la distancia y el techo: positiva si las piezas
    /// altas quedan lejos, negativa si quedan cerca.
    fn covarianza(v: &[(usize, f32, f32)]) -> f32 {
        let n = v.len() as f32;
        let md = v.iter().map(|p| p.1).sum::<f32>() / n;
        let mt = v.iter().map(|p| p.2).sum::<f32>() / n;

        v.iter().map(|p| (p.1 - md) * (p.2 - mt)).sum::<f32>() / n
    }

    #[test]
    fn el_segundo_rompeolas_invertido_sigue_en_ciento_sesenta_y_dos() {
        assert_eq!(
            nivel_segundo_rompeolas_invertido().scene.objects.len(),
            OBJETOS_DEL_SEGUNDO_ROMPEOLAS,
            "el segundo Rompeolas invertido no queda en 154 - 2 + 10"
        );
    }

    #[test]
    fn solo_se_mueven_los_diez_prismas_secundarios_al_invertir() {
        let segundo = nivel_segundo_rompeolas();
        let invertido = nivel_segundo_rompeolas_invertido();

        let nuevos = segundo_rompeolas(&segundo.scene);

        assert_eq!(nuevos, segundo_rompeolas(&invertido.scene));

        for i in 0..OBJETOS_DEL_SEGUNDO_ROMPEOLAS {
            let a = &segundo.scene.objects[i];
            let b = &invertido.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            let ca = a.primitive.bounds();
            let cb = b.primitive.bounds();

            if !nuevos.contains(&i) {
                assert_eq!(ca, cb, "la pieza {i} no es secundaria y se movio");
                continue;
            }

            // Mismo prisma: mismo ancho, misma altura, misma abscisa. Solo
            // cambia su sitio a lo largo del eje isla <-> Rompeolas.
            for (nombre, x, y) in [
                ("min.x", ca.min.x, cb.min.x),
                ("max.x", ca.max.x, cb.max.x),
                ("min.y", ca.min.y, cb.min.y),
                ("max.y", ca.max.y, cb.max.y),
                ("fondo", ca.max.z - ca.min.z, cb.max.z - cb.min.z),
            ] {
                assert!(
                    (x - y).abs() < MISMA_MEDIDA,
                    "el prisma {i} cambio de {nombre}: {x:.4} -> {y:.4}"
                );
            }
        }
    }

    #[test]
    fn la_jerarquia_se_invierte_hacia_el_rompeolas_y_la_base() {
        let apoyado = nivel_apoyado();
        let segundo = nivel_segundo_rompeolas();
        let invertido = nivel_segundo_rompeolas_invertido();

        let original = huella(
            &apoyado.scene,
            &indices_del_grupo(&apoyado, SpatialGroupId::Breakwater),
        );
        let base = apoyado.scene.objects[masa_pedestal(&apoyado.scene)]
            .primitive
            .bounds();

        for (nombre, destino) in [("el Rompeolas original", original), ("la base", base)] {
            let antes = distancias_y_techos(&segundo, &destino);
            let ahora = distancias_y_techos(&invertido, &destino);

            // Antes las altas quedaban lejos; ahora quedan cerca.
            assert!(
                covarianza(&antes) > 0.0,
                "en second_breakwater las altas ya estaban cerca de {nombre}"
            );
            assert!(
                covarianza(&ahora) < 0.0,
                "en el invertido las altas no quedan cerca de {nombre}: {:.3}",
                covarianza(&ahora)
            );

            // Y la cima es la pieza mas cercana.
            let cima = ahora
                .iter()
                .copied()
                .fold(ahora[0], |m, p| if p.2 > m.2 { p } else { m });
            let mas_cerca = ahora.iter().map(|p| p.1).fold(f32::MAX, f32::min);

            assert!(
                (cima.1 - mas_cerca).abs() < EPS,
                "la cima queda a {:.2} de {nombre} y la pieza mas cercana a {mas_cerca:.2}",
                cima.1
            );
        }
    }

    #[test]
    fn el_invertido_sigue_siendo_seis_tres_uno() {
        let invertido = nivel_segundo_rompeolas_invertido();

        let filas = filas_de_la_formacion(&invertido);
        let cuentas: Vec<usize> = filas.iter().map(|f| f.len()).collect();

        // De sur a norte: las dos filas de la masa basal, el racimo y la
        // cima, que ahora mira al Rompeolas.
        assert_eq!(cuentas, vec![3, 3, 3, 1], "las filas salen {cuentas:?}");

        let techo = |i: usize| invertido.scene.objects[i].primitive.bounds().max.y;
        let medio = |v: &[usize]| v.iter().map(|&i| techo(i)).sum::<f32>() / v.len() as f32;

        let cima = filas[3][0];
        let racimo = &filas[2];
        let basal: Vec<usize> = filas[0].iter().chain(&filas[1]).copied().collect();

        for &i in racimo.iter().chain(&basal) {
            assert!(techo(cima) > techo(i), "el prisma {i} pasa a la cima");
        }

        assert!(
            medio(racimo) > medio(&basal),
            "el racimo remata de media en {:.2} y la masa basal en {:.2}",
            medio(racimo),
            medio(&basal)
        );

        // La fila mas baja es la que mira al lado libre.
        assert!(
            medio(&filas[0]) < medio(&filas[1]),
            "la fila del sur no es la mas baja de la masa basal"
        );
    }

    #[test]
    fn el_invertido_se_apoya_y_deja_claro() {
        let apoyado = nivel_apoyado();
        let invertido = nivel_segundo_rompeolas_invertido();

        let isla = invertido.scene.objects[masa_de_la_segunda_isla(&apoyado.scene)]
            .primitive
            .bounds();
        let conector = invertido.scene.objects[masa_de_la_transicion(&apoyado.scene)]
            .primitive
            .bounds();
        let original = huella(
            &apoyado.scene,
            &indices_del_grupo(&apoyado, SpatialGroupId::Breakwater),
        );

        for &i in &segundo_rompeolas(&invertido.scene) {
            let caja = invertido.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&invertido.scene, i)
                .unwrap_or_else(|| panic!("el prisma {i} no tiene nada bajo su huella"));

            assert!(
                isla.min.x <= caja.min.x
                    && isla.max.x >= caja.max.x
                    && isla.min.z <= caja.min.z
                    && isla.max.z >= caja.max.z,
                "el prisma {i} no cae entero en la isla"
            );
            assert!(
                (techo - isla.max.y).abs() < EPS,
                "el prisma {i} se apoya a {techo:.3} y no en la isla"
            );
            assert!(
                caja.min.y < techo && caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "el prisma {i} arranca en {:.3} sobre un pedestal de {techo:.3}",
                caja.min.y
            );

            for (nombre, otra) in [
                (
                    "el Monolito",
                    huella_del_grupo(&invertido.scene, SpatialGroupId::Monolith),
                ),
                (
                    "Praderas",
                    huella_del_grupo(&invertido.scene, SpatialGroupId::Meadows),
                ),
                (
                    "Aguas",
                    huella_del_grupo(&invertido.scene, SpatialGroupId::FlyingWaters),
                ),
                ("el Rompeolas original", original),
            ] {
                assert!(
                    separacion_xz(&caja, &otra) >= CLARO_MINIMO,
                    "el prisma {i} deja {:.3} a {nombre}",
                    separacion_xz(&caja, &otra)
                );
            }

            assert!(
                separacion_xz(&caja, &conector) > 0.0,
                "el prisma {i} se mete en el conector"
            );
        }

        // Y el Rompeolas original sigue apoyado.
        for &i in &indices_del_grupo(&invertido, SpatialGroupId::Breakwater)[..PILARES] {
            let caja = invertido.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&invertido.scene, i)
                .unwrap_or_else(|| panic!("la pieza {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo && caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "la pieza {i} arranca en {:.3} sobre un pedestal de {techo:.3}",
                caja.min.y
            );
        }
    }

    #[test]
    fn el_invertido_conserva_el_lenguaje_y_la_compacidad() {
        let apoyado = nivel_apoyado();
        let invertido = nivel_segundo_rompeolas_invertido();

        let r01: Vec<Aabb> = indices_del_grupo(&apoyado, SpatialGroupId::Breakwater)[..PILARES]
            .iter()
            .map(|&i| apoyado.scene.objects[i].primitive.bounds())
            .collect();
        let paso_x_r01 = (centro_xz(&r01[6]).0 - centro_xz(&r01[0]).0) / 6.0;
        let paso_z_r01 = centro_xz(&r01[7]).1 - centro_xz(&r01[0]).1;

        let filas = filas_de_la_formacion(&invertido);
        let c = |i: usize| centro_xz(&invertido.scene.objects[i].primitive.bounds());

        for par in filas.windows(2) {
            assert!(
                ((c(par[1][0]).1 - c(par[0][0]).1) - paso_z_r01).abs() < MISMA_MEDIDA,
                "las filas no van al paso de R-01"
            );
        }

        for fila in &filas {
            for par in fila.windows(2) {
                assert!(
                    ((c(par[1]).0 - c(par[0]).0).abs() - paso_x_r01).abs() < MISMA_MEDIDA,
                    "dentro de una fila los prismas no van al paso de R-01"
                );
            }
        }

        let centros: Vec<(f32, f32)> = segundo_rompeolas(&invertido.scene)
            .iter()
            .map(|&i| c(i))
            .collect();

        assert_eq!(racimos_por_cercania(&centros, RACIMO_DE_ESCALONES).len(), 1);

        for (k, p) in centros.iter().enumerate() {
            let vecino = centros
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != k)
                .map(|(_, o)| ((p.0 - o.0).powi(2) + (p.1 - o.1).powi(2)).sqrt())
                .fold(f32::MAX, f32::min);

            assert!(
                vecino <= 1.60,
                "el prisma {k} tiene su vecino a {vecino:.2}"
            );
        }
    }

    #[test]
    fn el_segundo_rompeolas_invertido_es_determinista() {
        let uno = nivel_segundo_rompeolas_invertido();
        let dos = nivel_segundo_rompeolas_invertido();

        assert_eq!(uno.scene.objects.len(), dos.scene.objects.len());

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
    }

    #[test]
    fn los_veintiocho_niveles_previos_no_cambian_con_el_invertido() {
        let construir = || {
            [
                nivel(),
                nivel_candidato(),
                nivel_espacioso(),
                nivel_conectado(),
                nivel_refinado(),
                nivel_asimetrico(),
                nivel_separado(),
                nivel_apoyado(),
                nivel_escalera(),
                nivel_combinado(),
                nivel_ascenso_desde_grounded(),
                nivel_relleno_trasero(),
                nivel_escalera_monolito(),
                nivel_rompeolas_de_borde(),
                nivel_terraza_verde_de_borde(),
                nivel_plataformas_de_borde(),
                nivel_plataformas_finas(),
                nivel_plataformas_marcadas(),
                nivel_soporte_modular(),
                nivel_segunda_isla(),
                nivel_isla_de_borde(),
                nivel_isla_de_borde_ancha(),
                nivel_isla_larga_con_escalera(),
                nivel_isla_larga_con_transicion(),
                nivel_escalera_agrupada(),
                nivel_montana_en_isla(),
                nivel_isla_estilo_spacious(),
                nivel_segundo_rompeolas(),
            ]
        };

        let previos = construir();
        let _invertido = nivel_segundo_rompeolas_invertido();
        let otra_vez = construir();

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            assert_eq!(
                antes.scene.objects.len(),
                ahora.scene.objects.len(),
                "el nivel previo {n} cambio de conteo"
            );

            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }

        // El unico con otro conteo sigue siendo el segundo Rompeolas.
        assert_eq!(
            previos[previos.len() - 1].scene.objects.len(),
            OBJETOS_DEL_SEGUNDO_ROMPEOLAS
        );
    }

    // ------------------------------------------------------------------
    // Segundo Rompeolas escalonado: `second_breakwater_stair`
    // ------------------------------------------------------------------

    fn media(v: &[f32]) -> f32 {
        v.iter().sum::<f32>() / v.len() as f32
    }

    /// Por banda, de sur a norte: ancho medio, techo medio y distancia media
    /// a una caja.
    fn perfil_por_banda(diorama: &Blockout, a: &Aabb) -> Vec<(f32, f32, f32)> {
        filas_de_la_formacion(diorama)
            .iter()
            .map(|banda| {
                let cajas: Vec<Aabb> = banda
                    .iter()
                    .map(|&i| diorama.scene.objects[i].primitive.bounds())
                    .collect();

                (
                    media(&cajas.iter().map(|c| c.max.x - c.min.x).collect::<Vec<_>>()),
                    media(&cajas.iter().map(|c| c.max.y).collect::<Vec<_>>()),
                    media(
                        &cajas
                            .iter()
                            .map(|c| separacion_xz(c, a))
                            .collect::<Vec<_>>(),
                    ),
                )
            })
            .collect()
    }

    fn destinos_de_la_escalera() -> [(&'static str, Aabb); 2] {
        let apoyado = nivel_apoyado();

        [
            (
                "el Rompeolas original",
                huella(
                    &apoyado.scene,
                    &indices_del_grupo(&apoyado, SpatialGroupId::Breakwater),
                ),
            ),
            (
                "la base",
                apoyado.scene.objects[masa_pedestal(&apoyado.scene)]
                    .primitive
                    .bounds(),
            ),
        ]
    }

    #[test]
    fn el_segundo_rompeolas_escalonado_sigue_en_ciento_sesenta_y_dos() {
        assert_eq!(
            nivel_segundo_rompeolas_escalonado().scene.objects.len(),
            OBJETOS_DEL_SEGUNDO_ROMPEOLAS,
            "el escalonado no queda en 154 - 2 + 10"
        );
    }

    #[test]
    fn solo_los_diez_prismas_secundarios_cambian_al_escalonar() {
        let invertido = nivel_segundo_rompeolas_invertido();
        let escalonado = nivel_segundo_rompeolas_escalonado();

        let nuevos = segundo_rompeolas(&invertido.scene);

        assert_eq!(nuevos, segundo_rompeolas(&escalonado.scene));

        for i in 0..OBJETOS_DEL_SEGUNDO_ROMPEOLAS {
            let a = &invertido.scene.objects[i];
            let b = &escalonado.scene.objects[i];

            assert!(
                mismo_tipo(&a.primitive, &b.primitive),
                "la pieza {i} cambio de primitiva"
            );
            assert_eq!(a.initial_material, b.initial_material, "pieza {i}");
            assert_eq!(a.final_material, b.final_material, "pieza {i}");
            assert_eq!(a.spatial_group, b.spatial_group, "pieza {i}");
            assert_eq!(a.reveal_group, b.reveal_group, "pieza {i}");

            if !nuevos.contains(&i) {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "la pieza {i} no es secundaria y se movio"
                );
            }
        }

        // Base, isla, conector y el Rompeolas original: exactos a grounded
        // y a connected_island_stair.
        let apoyado = nivel_apoyado();

        for (&a, &b) in indices_del_grupo(&apoyado, SpatialGroupId::Breakwater)
            .iter()
            .zip(&indices_del_grupo(&escalonado, SpatialGroupId::Breakwater)[..ROMPEOLAS_ORIGINAL])
        {
            assert_eq!(
                apoyado.scene.objects[a].primitive.bounds(),
                escalonado.scene.objects[b].primitive.bounds(),
                "la pieza {a} del Rompeolas original cambio"
            );
        }
    }

    #[test]
    fn tamano_y_altura_crecen_por_banda_hacia_el_rompeolas() {
        let escalonado = nivel_segundo_rompeolas_escalonado();

        for (nombre, destino) in destinos_de_la_escalera() {
            let perfil = perfil_por_banda(&escalonado, &destino);

            assert!(
                perfil.len() >= 3,
                "la escalera tiene {} bandas y no al menos tres",
                perfil.len()
            );

            // De sur a norte: del extremo libre hacia el Rompeolas.
            for par in perfil.windows(2) {
                let (ancho_a, techo_a, lejos_a) = par[0];
                let (ancho_b, techo_b, lejos_b) = par[1];

                assert!(
                    lejos_b < lejos_a,
                    "las bandas no se acercan a {nombre}: {lejos_a:.2} -> {lejos_b:.2}"
                );
                assert!(
                    ancho_b > ancho_a,
                    "acercandose a {nombre} el ancho medio no crece: {ancho_a:.3} -> {ancho_b:.3}"
                );
                assert!(
                    techo_b > techo_a,
                    "acercandose a {nombre} el techo medio no crece: {techo_a:.2} -> {techo_b:.2}"
                );
            }
        }
    }

    #[test]
    fn la_escalera_se_orienta_hacia_el_rompeolas_original() {
        let escalonado = nivel_segundo_rompeolas_escalonado();

        for (nombre, destino) in destinos_de_la_escalera() {
            let piezas = distancias_y_techos(&escalonado, &destino);

            assert!(
                covarianza(&piezas) < 0.0,
                "las piezas altas no quedan del lado de {nombre}"
            );

            // La cima es la mas cercana y las mas pequenas, las mas lejanas.
            let cima = piezas
                .iter()
                .copied()
                .fold(piezas[0], |m, p| if p.2 > m.2 { p } else { m });
            let mas_cerca = piezas.iter().map(|p| p.1).fold(f32::MAX, f32::min);

            assert!(
                (cima.1 - mas_cerca).abs() < EPS,
                "la cima queda a {:.2} de {nombre} y la pieza mas cercana a {mas_cerca:.2}",
                cima.1
            );

            let ancho = |i: usize| {
                let c = escalonado.scene.objects[i].primitive.bounds();

                c.max.x - c.min.x
            };
            let mas_lejos = piezas.iter().map(|p| p.1).fold(f32::MIN, f32::max);
            let menor = piezas.iter().map(|p| ancho(p.0)).fold(f32::MAX, f32::min);

            for p in piezas.iter().filter(|p| (p.1 - mas_lejos).abs() < EPS) {
                assert!(
                    (ancho(p.0) - menor).abs() < MISMA_MEDIDA,
                    "la pieza mas lejana de {nombre} no es de las mas pequenas"
                );
            }
        }
    }

    #[test]
    fn la_escalera_es_continua_y_compacta() {
        let escalonado = nivel_segundo_rompeolas_escalonado();

        let centros: Vec<(f32, f32)> = segundo_rompeolas(&escalonado.scene)
            .iter()
            .map(|&i| centro_xz(&escalonado.scene.objects[i].primitive.bounds()))
            .collect();

        assert_eq!(
            racimos_por_cercania(&centros, RACIMO_DE_ESCALONES).len(),
            1,
            "la escalera se parte en grupos"
        );

        for (k, p) in centros.iter().enumerate() {
            let vecino = centros
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != k)
                .map(|(_, o)| ((p.0 - o.0).powi(2) + (p.1 - o.1).powi(2)).sqrt())
                .fold(f32::MAX, f32::min);

            assert!(
                vecino <= 1.60,
                "el prisma {k} tiene su vecino a {vecino:.2}"
            );
        }

        // Continua: cada banda toca la siguiente, sin un paso vacio entre
        // ellas.
        let filas = filas_de_la_formacion(&escalonado);
        let fondo = |banda: &[usize]| {
            let cajas: Vec<Aabb> = banda
                .iter()
                .map(|&i| escalonado.scene.objects[i].primitive.bounds())
                .collect();

            (
                cajas.iter().map(|c| c.min.z).fold(f32::MAX, f32::min),
                cajas.iter().map(|c| c.max.z).fold(f32::MIN, f32::max),
            )
        };

        for par in filas.windows(2) {
            let (_, norte_a) = fondo(&par[0]);
            let (sur_b, _) = fondo(&par[1]);

            assert!(
                sur_b <= norte_a,
                "entre dos bandas queda un hueco de {:.2}",
                sur_b - norte_a
            );
        }
    }

    #[test]
    fn la_escalera_se_apoya_y_deja_claro() {
        let apoyado = nivel_apoyado();
        let escalonado = nivel_segundo_rompeolas_escalonado();

        let isla = escalonado.scene.objects[masa_de_la_segunda_isla(&apoyado.scene)]
            .primitive
            .bounds();
        let conector = escalonado.scene.objects[masa_de_la_transicion(&apoyado.scene)]
            .primitive
            .bounds();
        let [(_, original), _] = destinos_de_la_escalera();

        for &i in &segundo_rompeolas(&escalonado.scene) {
            let caja = escalonado.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&escalonado.scene, i)
                .unwrap_or_else(|| panic!("el prisma {i} no tiene nada bajo su huella"));

            assert!(
                isla.min.x <= caja.min.x
                    && isla.max.x >= caja.max.x
                    && isla.min.z <= caja.min.z
                    && isla.max.z >= caja.max.z,
                "el prisma {i} no cae entero en la isla"
            );
            assert!(
                (techo - isla.max.y).abs() < EPS,
                "el prisma {i} se apoya a {techo:.3} y no en la isla"
            );
            assert!(
                caja.min.y < techo && caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "el prisma {i} arranca en {:.3} sobre un pedestal de {techo:.3}",
                caja.min.y
            );

            for (nombre, otra) in [
                (
                    "el Monolito",
                    huella_del_grupo(&escalonado.scene, SpatialGroupId::Monolith),
                ),
                (
                    "Praderas",
                    huella_del_grupo(&escalonado.scene, SpatialGroupId::Meadows),
                ),
                (
                    "Aguas",
                    huella_del_grupo(&escalonado.scene, SpatialGroupId::FlyingWaters),
                ),
                ("el Rompeolas original", original),
            ] {
                assert!(
                    separacion_xz(&caja, &otra) >= CLARO_MINIMO,
                    "el prisma {i} deja {:.3} a {nombre}",
                    separacion_xz(&caja, &otra)
                );
            }

            assert!(
                separacion_xz(&caja, &conector) > 0.0,
                "el prisma {i} se mete en el conector"
            );
        }

        for &i in &indices_del_grupo(&escalonado, SpatialGroupId::Breakwater)[..PILARES] {
            let caja = escalonado.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&escalonado.scene, i)
                .unwrap_or_else(|| panic!("la pieza {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo && caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "la pieza {i} arranca en {:.3} sobre un pedestal de {techo:.3}",
                caja.min.y
            );
        }
    }

    #[test]
    fn la_escalera_conserva_la_ruta_y_el_lenguaje() {
        let apoyado = nivel_apoyado();
        let escalonado = nivel_segundo_rompeolas_escalonado();

        let r01: Vec<Aabb> = indices_del_grupo(&apoyado, SpatialGroupId::Breakwater)[..PILARES]
            .iter()
            .map(|&i| apoyado.scene.objects[i].primitive.bounds())
            .collect();
        let paso_x_r01 = (centro_xz(&r01[6]).0 - centro_xz(&r01[0]).0) / 6.0;
        let paso_z_r01 = centro_xz(&r01[7]).1 - centro_xz(&r01[0]).1;

        for &i in &segundo_rompeolas(&escalonado.scene) {
            let o = &escalonado.scene.objects[i];
            let caja = o.primitive.bounds();

            #[cfg(feature = "hex-prism")]
            assert!(
                matches!(o.primitive, Primitive::HexPrism(_)),
                "el prisma {i} no es HexPrism en la Ruta A"
            );
            #[cfg(not(feature = "hex-prism"))]
            assert!(
                matches!(o.primitive, Primitive::Cuboid(_)),
                "el prisma {i} no es Cuboid en la Ruta B"
            );

            let ancho = caja.max.x - caja.min.x;

            assert!(
                r01.iter()
                    .any(|c| ((c.max.x - c.min.x) - ancho).abs() < MISMA_MEDIDA),
                "el prisma {i} mide {ancho:.3} y ningun grosor de R-01 mide eso"
            );
            assert!(!r01.contains(&caja), "el prisma {i} copia una caja de R-01");
        }

        let filas = filas_de_la_formacion(&escalonado);
        let c = |i: usize| centro_xz(&escalonado.scene.objects[i].primitive.bounds());

        for par in filas.windows(2) {
            assert!(
                ((c(par[1][0]).1 - c(par[0][0]).1) - paso_z_r01).abs() < MISMA_MEDIDA,
                "las bandas no van al paso de R-01"
            );
        }

        for fila in &filas {
            for par in fila.windows(2) {
                assert!(
                    ((c(par[1]).0 - c(par[0]).0).abs() - paso_x_r01).abs() < MISMA_MEDIDA,
                    "dentro de una banda los prismas no van al paso de R-01"
                );
            }
        }
    }

    #[test]
    fn el_segundo_rompeolas_escalonado_es_determinista() {
        let uno = nivel_segundo_rompeolas_escalonado();
        let dos = nivel_segundo_rompeolas_escalonado();

        assert_eq!(uno.scene.objects.len(), dos.scene.objects.len());

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
    }

    #[test]
    fn los_veintinueve_niveles_previos_no_cambian_con_el_escalonado() {
        let construir = || {
            [
                nivel(),
                nivel_candidato(),
                nivel_espacioso(),
                nivel_conectado(),
                nivel_refinado(),
                nivel_asimetrico(),
                nivel_separado(),
                nivel_apoyado(),
                nivel_escalera(),
                nivel_combinado(),
                nivel_ascenso_desde_grounded(),
                nivel_relleno_trasero(),
                nivel_escalera_monolito(),
                nivel_rompeolas_de_borde(),
                nivel_terraza_verde_de_borde(),
                nivel_plataformas_de_borde(),
                nivel_plataformas_finas(),
                nivel_plataformas_marcadas(),
                nivel_soporte_modular(),
                nivel_segunda_isla(),
                nivel_isla_de_borde(),
                nivel_isla_de_borde_ancha(),
                nivel_isla_larga_con_escalera(),
                nivel_isla_larga_con_transicion(),
                nivel_escalera_agrupada(),
                nivel_montana_en_isla(),
                nivel_isla_estilo_spacious(),
                nivel_segundo_rompeolas(),
                nivel_segundo_rompeolas_invertido(),
            ]
        };

        let previos = construir();
        let _escalonado = nivel_segundo_rompeolas_escalonado();
        let otra_vez = construir();

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            assert_eq!(
                antes.scene.objects.len(),
                ahora.scene.objects.len(),
                "el nivel previo {n} cambio de conteo"
            );

            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }
    }

    // ------------------------------------------------------------------
    // Segundo Rompeolas escalonado denso: `second_breakwater_stair_dense`
    // ------------------------------------------------------------------

    /// El conteo propio de este candidato: `154 - 2 + 16`.
    const OBJETOS_DEL_ESCALONADO_DENSO: usize = OBJETOS - 2 + 16;

    #[test]
    fn el_escalonado_denso_tiene_ciento_sesenta_y_ocho_objetos() {
        assert_eq!(OBJETOS_DEL_ESCALONADO_DENSO, 168);
        assert_eq!(
            nivel_segundo_rompeolas_escalonado_denso()
                .scene
                .objects
                .len(),
            OBJETOS_DEL_ESCALONADO_DENSO,
            "el escalonado denso no queda en 154 - 2 + 16"
        );
    }

    #[test]
    fn la_formacion_densa_son_dieciseis_prismas_de_la_ruta() {
        let apoyado = nivel_apoyado();
        let denso = nivel_segundo_rompeolas_escalonado_denso();

        let nuevos = segundo_rompeolas(&denso.scene);

        assert_eq!(
            nuevos.len(),
            16,
            "la formacion tiene {} prismas",
            nuevos.len()
        );

        let paleta_r01: Vec<_> = indices_del_grupo(&apoyado, SpatialGroupId::Breakwater)[..PILARES]
            .iter()
            .map(|&i| {
                let o = &apoyado.scene.objects[i];

                (o.initial_material, o.final_material, o.reveal_group)
            })
            .collect();

        for &i in &nuevos {
            let o = &denso.scene.objects[i];

            #[cfg(feature = "hex-prism")]
            assert!(
                matches!(o.primitive, Primitive::HexPrism(_)),
                "el prisma {i} no es HexPrism en la Ruta A"
            );
            #[cfg(not(feature = "hex-prism"))]
            assert!(
                matches!(o.primitive, Primitive::Cuboid(_)),
                "el prisma {i} no es Cuboid en la Ruta B"
            );

            assert_eq!(o.spatial_group, SpatialGroupId::Breakwater, "prisma {i}");
            assert!(
                paleta_r01.contains(&(o.initial_material, o.final_material, o.reveal_group)),
                "el prisma {i} no usa la paleta del primer Rompeolas"
            );
        }

        assert_eq!(denso.scene.palette.len(), apoyado.scene.palette.len());
        assert_eq!(denso.scene.textures.len(), apoyado.scene.textures.len());
    }

    #[test]
    fn solo_la_formacion_secundaria_cambia_en_el_denso() {
        let apoyado = nivel_apoyado();
        let escalonado = nivel_segundo_rompeolas_escalonado();
        let denso = nivel_segundo_rompeolas_escalonado_denso();

        for grupo in [
            SpatialGroupId::Global,
            SpatialGroupId::ContinentBackground,
            SpatialGroupId::Monolith,
            SpatialGroupId::Meadows,
            SpatialGroupId::FlyingWaters,
        ] {
            assert_eq!(
                cajas_del_grupo(&escalonado, grupo),
                cajas_del_grupo(&denso, grupo),
                "{grupo:?} cambio"
            );
        }

        // El Rompeolas original, exacto a grounded y delante de la formacion.
        let antes = indices_del_grupo(&apoyado, SpatialGroupId::Breakwater);
        let ahora = indices_del_grupo(&denso, SpatialGroupId::Breakwater);

        assert_eq!(ahora.len(), ROMPEOLAS_ORIGINAL + 16);

        for (&a, &b) in antes.iter().zip(&ahora[..ROMPEOLAS_ORIGINAL]) {
            let x = &apoyado.scene.objects[a];
            let y = &denso.scene.objects[b];

            assert!(mismo_tipo(&x.primitive, &y.primitive), "pieza {a}");
            assert_eq!(x.primitive.bounds(), y.primitive.bounds(), "pieza {a}");
            assert_eq!(x.initial_material, y.initial_material, "pieza {a}");
            assert_eq!(x.final_material, y.final_material, "pieza {a}");
        }

        // Y el grupo sigue contiguo: la formacion va justo detras.
        for par in ahora.windows(2) {
            assert_eq!(par[1], par[0] + 1, "el Rompeolas deja de ser contiguo");
        }
    }

    #[test]
    fn el_denso_se_acerca_al_rompeolas_original_sin_tocarlo() {
        let apoyado = nivel_apoyado();
        let denso = nivel_segundo_rompeolas_escalonado_denso();

        let piezas_r1: Vec<Aabb> = indices_del_grupo(&apoyado, SpatialGroupId::Breakwater)
            .iter()
            .map(|&i| apoyado.scene.objects[i].primitive.bounds())
            .collect();
        let [(_, original), _] = destinos_de_la_escalera();

        let claro = segundo_rompeolas(&denso.scene)
            .iter()
            .map(|&i| separacion_xz(&denso.scene.objects[i].primitive.bounds(), &original))
            .fold(f32::MAX, f32::min);

        assert!(
            (0.25..=0.60).contains(&claro),
            "el claro al Rompeolas original es {claro:.3}, fuera de 0.25..0.60"
        );

        // Mas cerca que la escalera de diez.
        let escalonado = nivel_segundo_rompeolas_escalonado();
        let claro_antes = segundo_rompeolas(&escalonado.scene)
            .iter()
            .map(|&i| separacion_xz(&escalonado.scene.objects[i].primitive.bounds(), &original))
            .fold(f32::MAX, f32::min);

        assert!(
            claro < claro_antes,
            "no se acerca: {claro_antes:.2} -> {claro:.2}"
        );

        // Sin solapar ninguna pieza del Rompeolas original.
        for &i in &segundo_rompeolas(&denso.scene) {
            let caja = denso.scene.objects[i].primitive.bounds();

            for (k, otra) in piezas_r1.iter().enumerate() {
                assert!(
                    separacion_xz(&caja, otra) > 0.0,
                    "el prisma {i} solapa la pieza {k} del Rompeolas original"
                );
            }
        }
    }

    #[test]
    fn el_denso_cubre_mas_isla_que_la_escalera_de_diez() {
        let escalonado = nivel_segundo_rompeolas_escalonado();
        let denso = nivel_segundo_rompeolas_escalonado_denso();

        let antes = cobertura_de_la_isla(&escalonado);
        let ahora = cobertura_de_la_isla(&denso);

        assert!(
            ahora > antes,
            "la formacion cubre {:.1} % de la isla y la de diez {:.1} %",
            ahora * 100.0,
            antes * 100.0
        );

        // Y el pie es mas ancho y mas poblado.
        let pie_antes = &filas_de_la_formacion(&escalonado)[0];
        let pie_ahora = &filas_de_la_formacion(&denso)[0];
        let ancho_de = |d: &Blockout, banda: &[usize]| {
            let cajas: Vec<Aabb> = banda
                .iter()
                .map(|&i| d.scene.objects[i].primitive.bounds())
                .collect();

            cajas.iter().map(|c| c.max.x).fold(f32::MIN, f32::max)
                - cajas.iter().map(|c| c.min.x).fold(f32::MAX, f32::min)
        };

        assert!(pie_ahora.len() > pie_antes.len());
        assert!(
            ancho_de(&denso, pie_ahora) > ancho_de(&escalonado, pie_antes),
            "el pie no se ensancha"
        );
    }

    #[test]
    fn las_bandas_densas_progresan_hacia_el_rompeolas() {
        let denso = nivel_segundo_rompeolas_escalonado_denso();

        let cuentas: Vec<usize> = filas_de_la_formacion(&denso)
            .iter()
            .map(|f| f.len())
            .collect();

        // Del extremo libre al Rompeolas.
        assert_eq!(cuentas, vec![6, 5, 3, 2], "las bandas salen {cuentas:?}");

        for (nombre, destino) in destinos_de_la_escalera() {
            let perfil = perfil_por_banda(&denso, &destino);

            for par in perfil.windows(2) {
                let (ancho_a, techo_a, lejos_a) = par[0];
                let (ancho_b, techo_b, lejos_b) = par[1];

                assert!(lejos_b < lejos_a, "las bandas no se acercan a {nombre}");
                assert!(
                    ancho_b > ancho_a,
                    "hacia {nombre} el ancho medio no crece: {ancho_a:.3} -> {ancho_b:.3}"
                );
                assert!(
                    techo_b > techo_a,
                    "hacia {nombre} el techo medio no crece: {techo_a:.2} -> {techo_b:.2}"
                );
            }

            assert!(
                covarianza(&distancias_y_techos(&denso, &destino)) < 0.0,
                "las piezas altas no quedan del lado de {nombre}"
            );
        }
    }

    #[test]
    fn la_cima_densa_es_alta_y_esta_junto_al_rompeolas() {
        let apoyado = nivel_apoyado();
        let escalonado = nivel_segundo_rompeolas_escalonado();
        let denso = nivel_segundo_rompeolas_escalonado_denso();

        let techo = |d: &Blockout, i: usize| d.scene.objects[i].primitive.bounds().max.y;
        let filas = filas_de_la_formacion(&denso);
        let arriba = filas.last().expect("hay bandas");

        let cima = segundo_rompeolas(&denso.scene)
            .into_iter()
            .fold(arriba[0], |m, i| {
                if techo(&denso, i) > techo(&denso, m) {
                    i
                } else {
                    m
                }
            });

        assert!(
            arriba.contains(&cima),
            "el prisma mas alto no esta en la banda junto al Rompeolas"
        );

        let cima_antes = segundo_rompeolas(&escalonado.scene)
            .into_iter()
            .map(|i| techo(&escalonado, i))
            .fold(f32::MIN, f32::max);

        assert!(
            techo(&denso, cima) >= cima_antes,
            "la cima baja: {cima_antes:.2} -> {:.2}",
            techo(&denso, cima)
        );

        let cresta = indices_del_grupo(&apoyado, SpatialGroupId::Breakwater)[..PILARES]
            .iter()
            .map(|&i| techo(&apoyado, i))
            .fold(f32::MIN, f32::max);

        assert!(
            techo(&denso, cima) < cresta,
            "la cima pasa de la cresta original"
        );
    }

    #[test]
    fn el_denso_es_compacto_y_continuo() {
        let denso = nivel_segundo_rompeolas_escalonado_denso();

        let centros: Vec<(f32, f32)> = segundo_rompeolas(&denso.scene)
            .iter()
            .map(|&i| centro_xz(&denso.scene.objects[i].primitive.bounds()))
            .collect();

        assert_eq!(racimos_por_cercania(&centros, RACIMO_DE_ESCALONES).len(), 1);

        for (k, p) in centros.iter().enumerate() {
            let vecino = centros
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != k)
                .map(|(_, o)| ((p.0 - o.0).powi(2) + (p.1 - o.1).powi(2)).sqrt())
                .fold(f32::MAX, f32::min);

            assert!(
                vecino <= 1.60,
                "el prisma {k} tiene su vecino a {vecino:.2}"
            );
        }

        let filas = filas_de_la_formacion(&denso);
        let tramo = |banda: &[usize]| {
            let cajas: Vec<Aabb> = banda
                .iter()
                .map(|&i| denso.scene.objects[i].primitive.bounds())
                .collect();

            (
                cajas.iter().map(|c| c.min.z).fold(f32::MAX, f32::min),
                cajas.iter().map(|c| c.max.z).fold(f32::MIN, f32::max),
            )
        };

        for par in filas.windows(2) {
            assert!(
                tramo(&par[1]).0 <= tramo(&par[0]).1,
                "entre dos bandas queda un hueco"
            );
        }

        // Las bandas van al paso de R-01 y dentro de una banda nunca se
        // separan mas que el: solo se aprietan donde la isla no da.
        let apoyado = nivel_apoyado();
        let r01: Vec<Aabb> = indices_del_grupo(&apoyado, SpatialGroupId::Breakwater)[..PILARES]
            .iter()
            .map(|&i| apoyado.scene.objects[i].primitive.bounds())
            .collect();
        let paso_x_r01 = (centro_xz(&r01[6]).0 - centro_xz(&r01[0]).0) / 6.0;
        let paso_z_r01 = centro_xz(&r01[7]).1 - centro_xz(&r01[0]).1;
        let c = |i: usize| centro_xz(&denso.scene.objects[i].primitive.bounds());

        for par in filas.windows(2) {
            assert!(((c(par[1][0]).1 - c(par[0][0]).1) - paso_z_r01).abs() < MISMA_MEDIDA);
        }

        for fila in &filas {
            for par in fila.windows(2) {
                assert!((c(par[1]).0 - c(par[0]).0).abs() <= paso_x_r01 + MISMA_MEDIDA);
            }
        }

        for &i in &segundo_rompeolas(&denso.scene) {
            let caja = denso.scene.objects[i].primitive.bounds();
            let ancho = caja.max.x - caja.min.x;

            assert!(
                r01.iter()
                    .any(|r| ((r.max.x - r.min.x) - ancho).abs() < MISMA_MEDIDA),
                "el prisma {i} mide {ancho:.3} y ningun grosor de R-01 mide eso"
            );
        }
    }

    #[test]
    fn el_denso_se_apoya_y_no_invade() {
        let apoyado = nivel_apoyado();
        let denso = nivel_segundo_rompeolas_escalonado_denso();

        let isla = denso.scene.objects[masa_de_la_segunda_isla(&apoyado.scene)]
            .primitive
            .bounds();
        let conector = denso.scene.objects[masa_de_la_transicion(&apoyado.scene)]
            .primitive
            .bounds();

        for &i in &segundo_rompeolas(&denso.scene) {
            let caja = denso.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&denso.scene, i)
                .unwrap_or_else(|| panic!("el prisma {i} no tiene nada bajo su huella"));

            assert!(
                isla.min.x <= caja.min.x
                    && isla.max.x >= caja.max.x
                    && isla.min.z <= caja.min.z
                    && isla.max.z >= caja.max.z,
                "el prisma {i} no cae entero en la isla"
            );
            assert!(
                (techo - isla.max.y).abs() < EPS,
                "el prisma {i} se apoya a {techo:.3} y no en la isla"
            );
            assert!(
                caja.min.y < techo && caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "el prisma {i} arranca en {:.3} sobre un pedestal de {techo:.3}",
                caja.min.y
            );

            for (nombre, otra) in [
                (
                    "el Monolito",
                    huella_del_grupo(&denso.scene, SpatialGroupId::Monolith),
                ),
                (
                    "Praderas",
                    huella_del_grupo(&denso.scene, SpatialGroupId::Meadows),
                ),
                (
                    "Aguas",
                    huella_del_grupo(&denso.scene, SpatialGroupId::FlyingWaters),
                ),
            ] {
                assert!(
                    separacion_xz(&caja, &otra) >= CLARO_MINIMO,
                    "el prisma {i} deja {:.3} a {nombre}",
                    separacion_xz(&caja, &otra)
                );
            }

            assert!(
                separacion_xz(&caja, &conector) > 0.0,
                "el prisma {i} se mete en el conector"
            );
        }

        for &i in &indices_del_grupo(&denso, SpatialGroupId::Breakwater)[..PILARES] {
            let caja = denso.scene.objects[i].primitive.bounds();
            let techo = pedestal_bajo(&denso.scene, i)
                .unwrap_or_else(|| panic!("la pieza {i} no tiene nada bajo su huella"));

            assert!(
                caja.min.y <= techo && caja.min.y >= techo - EMPOTRADO_MAXIMO,
                "la pieza {i} arranca en {:.3} sobre un pedestal de {techo:.3}",
                caja.min.y
            );
        }
    }

    #[test]
    fn el_escalonado_denso_es_determinista() {
        let uno = nivel_segundo_rompeolas_escalonado_denso();
        let dos = nivel_segundo_rompeolas_escalonado_denso();

        assert_eq!(uno.scene.objects.len(), dos.scene.objects.len());

        for (i, (a, b)) in uno.scene.objects.iter().zip(&dos.scene.objects).enumerate() {
            assert_eq!(
                a.primitive.bounds(),
                b.primitive.bounds(),
                "el objeto {i} no es determinista"
            );
        }

        assert_eq!(uno.scale.scene_radius, dos.scale.scene_radius);
        assert_eq!(uno.scale.orbit_radius, dos.scale.orbit_radius);
    }

    #[test]
    fn los_treinta_niveles_previos_no_cambian_con_el_denso() {
        let construir = || {
            [
                nivel(),
                nivel_candidato(),
                nivel_espacioso(),
                nivel_conectado(),
                nivel_refinado(),
                nivel_asimetrico(),
                nivel_separado(),
                nivel_apoyado(),
                nivel_escalera(),
                nivel_combinado(),
                nivel_ascenso_desde_grounded(),
                nivel_relleno_trasero(),
                nivel_escalera_monolito(),
                nivel_rompeolas_de_borde(),
                nivel_terraza_verde_de_borde(),
                nivel_plataformas_de_borde(),
                nivel_plataformas_finas(),
                nivel_plataformas_marcadas(),
                nivel_soporte_modular(),
                nivel_segunda_isla(),
                nivel_isla_de_borde(),
                nivel_isla_de_borde_ancha(),
                nivel_isla_larga_con_escalera(),
                nivel_isla_larga_con_transicion(),
                nivel_escalera_agrupada(),
                nivel_montana_en_isla(),
                nivel_isla_estilo_spacious(),
                nivel_segundo_rompeolas(),
                nivel_segundo_rompeolas_invertido(),
                nivel_segundo_rompeolas_escalonado(),
            ]
        };

        let previos = construir();
        let _denso = nivel_segundo_rompeolas_escalonado_denso();
        let otra_vez = construir();

        for (n, (antes, ahora)) in previos.iter().zip(&otra_vez).enumerate() {
            assert_eq!(
                antes.scene.objects.len(),
                ahora.scene.objects.len(),
                "el nivel previo {n} cambio de conteo"
            );

            for (i, (a, b)) in antes
                .scene
                .objects
                .iter()
                .zip(&ahora.scene.objects)
                .enumerate()
            {
                assert_eq!(
                    a.primitive.bounds(),
                    b.primitive.bounds(),
                    "el nivel previo {n} cambio en el objeto {i}"
                );
            }

            assert_eq!(antes.scale.scene_radius, ahora.scale.scene_radius);
            assert_eq!(antes.scale.orbit_radius, ahora.scale.orbit_radius);
        }

        // La escalera de diez sigue en 162.
        assert_eq!(
            previos[previos.len() - 1].scene.objects.len(),
            OBJETOS_DEL_SEGUNDO_ROMPEOLAS
        );
    }
}
