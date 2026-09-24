//! Máscara de cobertura pintada sobre una superficie.
//!
//! `BrushMask` guarda **cuánto** se ha pintado cada celda de una superficie,
//! normalizado en `0.0..=1.0`, y nada más. No sabe de colores, de texturas ni
//! de materiales: es el sustrato que las herramientas futuras —revelado por
//! pincel y pigmento— consultarán para decidir qué mostrar en cada punto.
//!
//! Separar la cobertura del color es deliberado. El revelado actual es un
//! `f32` por grupo, global y sin forma; una máscara por superficie es lo que
//! permite que dos herramientas distintas compartan la misma geometría de
//! trazo sin acordar una paleta.
//!
//! `BrushMasks` guarda una máscara **por superficie pintable**, y una
//! superficie es el par `(objeto, carta UV)`: ver `SurfaceKey`. Las caras
//! `+X` y `-X` de un cuboide recorren las mismas `uv`, así que sin la carta
//! serían la misma máscara y pintar una mancharía la otra.
//!
//! `PigmentMasks` es la otra herramienta sobre la misma geometría: en vez
//! de *cuánto* se ha pintado guarda *de qué color*, con su propia alfa. Las
//! dos comparten el rasterizador —la misma cápsula, el mismo borde duro— y
//! se diferencian solo en qué escriben en cada celda cubierta.
//!
//! El pigmento se aplica plano o **desde una textura**. En el segundo caso
//! la textura se muestrea celda a celda y el color queda estampado: la capa
//! guarda colores, nunca una referencia ni un identificador, así que la
//! textura de origen puede desaparecer sin que lo pintado cambie.
//!
//! El módulo no sabe de `RevealState`, materiales ni texturas: solo de
//! cobertura y de color lineal.

use crate::color::Color;
use crate::hit::UvChart;
use crate::texture::Texture;
use std::collections::HashMap;

/// Cobertura pintada sobre una superficie, en `0.0..=1.0` por celda.
///
/// La superficie se direcciona en `uv`: `0.0..=1.0` en los dos ejes, con
/// `v = 0` **abajo**, que es la convención que ya usan las texturas del
/// proyecto. Fuera de ese rango no hay superficie, y la máscara **no
/// envuelve**: una `uv` fuera de rango no pinta al otro lado ni se consulta
/// como si estuviera dentro.
///
/// La cobertura se decide en el **centro de cada celda** y el borde del
/// trazo es duro, sin degradado. Es la misma decisión que el muestreo por
/// vecino más cercano del resto del renderer: el diorama es de aristas
/// definidas, y un borde duro es además exactamente reproducible, que es lo
/// que permite fijarlo con tests.
#[derive(Debug, Clone)]
pub struct BrushMask {
    width: usize,
    height: usize,
    cobertura: Vec<f32>,
}

impl BrushMask {
    /// Máscara vacía de `width` × `height` celdas.
    ///
    /// # Pánico
    ///
    /// Con cualquier dimensión nula. Una máscara sin celdas no tiene nada
    /// que consultar y cualquier `uv` caería en un hueco: es un error de
    /// programación, no una entrada del usuario, y fallar en el momento
    /// cuesta menos que devolver una máscara que nunca pinta.
    pub fn new(width: usize, height: usize) -> Self {
        assert!(
            width > 0 && height > 0,
            "una mascara necesita dimension no nula, se pidio {width} x {height}"
        );

        BrushMask {
            width,
            height,
            cobertura: vec![0.0; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Cobertura de una celda concreta.
    ///
    /// # Pánico
    ///
    /// Con índices fuera de la rejilla. A diferencia de una `uv`, que puede
    /// venir de una intersección y se trata como dato, un índice de celda lo
    /// escribe quien llama.
    pub fn pixel(&self, x: usize, y: usize) -> f32 {
        assert!(
            x < self.width && y < self.height,
            "la celda ({x}, {y}) cae fuera de la mascara de {} x {}",
            self.width,
            self.height
        );

        self.cobertura[y * self.width + x]
    }

    /// Cobertura en una coordenada `uv`.
    ///
    /// Fuera de `0.0..=1.0`, o con una coordenada no finita, devuelve `0.0`:
    /// ahí no hay superficie pintada porque no hay superficie.
    pub fn sample(&self, u: f32, v: f32) -> f32 {
        match self.celda(u, v) {
            Some((x, y)) => self.cobertura[y * self.width + x],
            None => 0.0,
        }
    }

    /// Estampa un disco de radio `radio` —en unidades `uv`— centrado en
    /// `(u, v)`, sumando `opacidad` a la cobertura que ya hubiera.
    ///
    /// Es el trazo degenerado: un segmento cuyos dos extremos coinciden.
    pub fn stamp(&mut self, u: f32, v: f32, radio: f32, opacidad: f32) {
        self.stroke(u, v, u, v, radio, opacidad);
    }

    /// Traza un segmento continuo de `(u0, v0)` a `(u1, v1)` con grosor
    /// `radio`, sumando `opacidad` a la cobertura que ya hubiera.
    ///
    /// La figura es la cápsula de radio `radio` alrededor del segmento, y se
    /// resuelve por **distancia al segmento**, no encadenando sellos. La
    /// diferencia importa: una ristra de sellos solapados aplicaría la
    /// opacidad varias veces a la misma celda, y un trazo suave saldría
    /// saturado según lo fino que fuera el paso.
    ///
    /// Es un no-op silencioso si algún argumento no es finito, si un extremo
    /// cae fuera de `0.0..=1.0`, o si el radio o la opacidad no pueden
    /// pintar nada. Estas entradas llegarán de una intersección y de un
    /// arrastre del ratón, así que son datos a validar y no errores de
    /// programación.
    ///
    /// Es el caso redondo de `stroke_ellipse`, no una implementación
    /// aparte: delegar es lo que garantiza que las dos figuras no puedan
    /// separarse con el tiempo.
    pub fn stroke(&mut self, u0: f32, v0: f32, u1: f32, v1: f32, radio: f32, opacidad: f32) {
        self.stroke_ellipse(u0, v0, u1, v1, radio, radio, opacidad);
    }

    /// Estampa una elipse de semiejes `radio_u` y `radio_v`, en unidades
    /// `uv`, centrada en `(u, v)`.
    ///
    /// Es el trazo elíptico degenerado, igual que `stamp` lo es de `stroke`.
    pub fn stamp_ellipse(&mut self, u: f32, v: f32, radio_u: f32, radio_v: f32, opacidad: f32) {
        self.stroke_ellipse(u, v, u, v, radio_u, radio_v, opacidad);
    }

    /// Traza un segmento con grosor **elíptico**: `radio_u` en `u` y
    /// `radio_v` en `v`.
    ///
    /// La figura es la cápsula de radio uno alrededor del segmento en el
    /// espacio `uv` **normalizado por los dos radios**, que es una cápsula
    /// elíptica de verdad y no un círculo corregido a ojo: sus dos tapas y
    /// su grosor se estiran por igual, y con `radio_u == radio_v` vuelve a
    /// ser exactamente la circular.
    ///
    /// Sirve para lo que `stroke` no puede: una cara cuya `uv` cubre una
    /// superficie mucho más ancha que alta necesita semiejes distintos para
    /// que el trazo se vea redondo sobre la pieza. Ese cálculo no vive aquí
    /// —esta máscara no sabe cuánto mide su superficie— pero la geometría ya
    /// está disponible para quien lo haga.
    ///
    /// Rechaza lo mismo que `stroke`, y además **cualquiera** de los dos
    /// radios que no sea finito y estrictamente positivo.
    #[allow(clippy::too_many_arguments)]
    pub fn stroke_ellipse(
        &mut self,
        u0: f32,
        v0: f32,
        u1: f32,
        v1: f32,
        radio_u: f32,
        radio_v: f32,
        opacidad: f32,
    ) {
        let Some(opacidad) = opacidad_efectiva(u0, v0, u1, v1, radio_u, radio_v, opacidad) else {
            return;
        };

        let (ancho, alto) = (self.width, self.height);
        let cobertura = &mut self.cobertura;

        por_celda_cubierta(ancho, alto, u0, v0, u1, v1, radio_u, radio_v, |x, y| {
            let celda = &mut cobertura[y * ancho + x];
            *celda = (*celda + opacidad).min(1.0);
        });
    }

    /// Celda que contiene una `uv`, o `None` si no hay superficie ahí.
    ///
    /// `u = 1.0` pertenece a la última columna y no envuelve a la primera:
    /// la máscara cubre una superficie, no un mosaico repetido.
    fn celda(&self, u: f32, v: f32) -> Option<(usize, usize)> {
        if !(0.0..=1.0).contains(&u) || !(0.0..=1.0).contains(&v) {
            return None;
        }

        let x = ((u * self.width as f32) as usize).min(self.width - 1);
        let y = ((v * self.height as f32) as usize).min(self.height - 1);

        Some((x, y))
    }
}

/// Recorre las celdas que cubre la cápsula de un trazo.
///
/// Es el **único** rasterizador del módulo. `BrushMask` y `PigmentMasks`
/// dibujan la misma figura y se diferencian solo en qué escriben en cada
/// celda, así que la geometría se define aquí una vez: si hubiera dos
/// copias, un trazo de revelado y uno de color acabarían dejando siluetas
/// distintas sobre la misma superficie, y la diferencia se vería antes en la
/// pantalla que en un test.
///
/// La figura es la cápsula de radio **uno** alrededor del segmento en el
/// espacio `uv` normalizado por `radio_u` y `radio_v`, resuelta por
/// distancia y con borde duro: una celda está dentro o fuera, decidida en su
/// centro. Con los dos radios iguales esa normalización es un escalado
/// uniforme y la figura es la circular de siempre.
///
/// Normalizar antes de proyectar, y no después, es lo que hace que la
/// cápsula sea elíptica de verdad: el punto más cercano del segmento a una
/// celda depende de la métrica, así que proyectar en `uv` crudo y estirar
/// luego daría una figura distinta cerca de las tapas.
///
/// `aplicar` recibe las coordenadas `(x, y)` de cada celda cubierta, sin
/// repetir ninguna.
///
/// Entrega la celda y no su índice porque el pigmento de textura necesita
/// saber **dónde** está para muestrear en su centro; quien solo quiera
/// escribir hace la multiplicación, que cuesta lo mismo que la división que
/// haría falta al revés.
///
/// No valida: eso lo hace `opacidad_efectiva` antes de llegar aquí.
#[allow(clippy::too_many_arguments)]
fn por_celda_cubierta(
    width: usize,
    height: usize,
    u0: f32,
    v0: f32,
    u1: f32,
    v1: f32,
    radio_u: f32,
    radio_v: f32,
    mut aplicar: impl FnMut(usize, usize),
) {
    let ancho = width as f32;
    let alto = height as f32;

    // Solo las celdas que la cápsula puede alcanzar, y cada eje con su
    // radio. Sin esto habría que recorrer la máscara entera por cada trazo.
    let franja = |bajo: f32, alto_: f32, celdas: f32, radio: f32| {
        let primera = ((bajo - radio) * celdas).floor().clamp(0.0, celdas - 1.0) as usize;
        let ultima = ((alto_ + radio) * celdas).floor().clamp(0.0, celdas - 1.0) as usize;
        primera..=ultima
    };

    // El segmento en el espacio normalizado. Ahí la figura es un círculo de
    // radio uno, y toda la aritmética vuelve a ser la del caso redondo.
    let (dx, dy) = ((u1 - u0) / radio_u, (v1 - v0) / radio_v);
    let largo2 = dx * dx + dy * dy;

    for y in franja(v0.min(v1), v0.max(v1), alto, radio_v) {
        let cv = (y as f32 + 0.5) / alto;
        let pv = (cv - v0) / radio_v;

        for x in franja(u0.min(u1), u0.max(u1), ancho, radio_u) {
            let cu = (x as f32 + 0.5) / ancho;
            let pu = (cu - u0) / radio_u;

            // Proyección del centro de la celda sobre el segmento, recortada
            // a sus extremos: con `t` fuera de `0..1` el punto más cercano es
            // una de las dos tapas.
            let t = if largo2 > 0.0 {
                ((pu * dx + pv * dy) / largo2).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let (du, dv) = (pu - dx * t, pv - dy * t);

            if du * du + dv * dv <= 1.0 {
                aplicar(x, y);
            }
        }
    }
}

/// Opacidad ya recortada de un trazo, o `None` si la entrada no puede
/// pintar nada.
///
/// Es la **única** definición de entrada válida del módulo. Vive suelta y no
/// dentro de `BrushMask` porque `BrushMasks` la necesita **antes** de decidir
/// si abre una capa: si validara después, un clic fallido dejaría una máscara
/// vacía por cada superficie que el puntero rozara.
///
/// Rechaza lo mismo que documenta `BrushMask::stroke`: extremos fuera de
/// `0.0..=1.0` —lo que incluye los no finitos, que no pertenecen a ningún
/// rango—, y un radio o una opacidad que no sean finitos y positivos. Los
/// dos radios se validan por separado: uno nulo aplastaría la elipse y el
/// otro dividiría entre cero al normalizar.
fn opacidad_efectiva(
    u0: f32,
    v0: f32,
    u1: f32,
    v1: f32,
    radio_u: f32,
    radio_v: f32,
    opacidad: f32,
) -> Option<f32> {
    let en_rango = |c: f32| (0.0..=1.0).contains(&c);
    let radio_util = |r: f32| r.is_finite() && r > 0.0;

    if !en_rango(u0) || !en_rango(v0) || !en_rango(u1) || !en_rango(v1) {
        return None;
    }
    if !radio_util(radio_u) || !radio_util(radio_v) {
        return None;
    }
    if !opacidad.is_finite() || opacidad <= 0.0 {
        return None;
    }

    Some(opacidad.min(1.0))
}

/// Qué superficie se pinta: un objeto de la escena y una de sus cartas `uv`.
///
/// Las dos mitades son necesarias y ninguna sobra:
///
/// - Sin el **objeto**, los veintiocho pilares del Rompeolas compartirían
///   máscara, porque comparten la carta lateral.
/// - Sin la **carta**, las caras `+X` y `-X` de un mismo cuboide compartirían
///   máscara, porque recorren idénticas `uv`.
///
/// Hereda de `UvChart` su alcance: vale mientras la escena no cambie de
/// composición, y la de este proyecto se construye una sola vez.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfaceKey {
    pub object_index: usize,
    pub uv_chart: UvChart,
}

impl SurfaceKey {
    pub const fn new(object_index: usize, uv_chart: UvChart) -> Self {
        SurfaceKey {
            object_index,
            uv_chart,
        }
    }
}

/// Una `BrushMask` por superficie pintada, todas a la misma resolución.
///
/// # Por qué perezoso
///
/// Reservar una máscara por superficie de antemano costaría una rejilla
/// entera por cada cara de cada objeto del diorama, casi toda en cero para
/// siempre: el pincel toca una fracción de lo que se ve. Aquí una capa
/// aparece cuando algo se pinta sobre ella, y **solo** entonces. Consultar no
/// crea, y una entrada que no puede pintar tampoco: si no fuera así, el
/// puntero fallando sobre el cielo iría dejando capas vacías detrás.
#[derive(Debug, Clone)]
pub struct BrushMasks {
    /// Máscara vacía que se clona al abrir una capa. Guardarla en vez de un
    /// par de enteros deja la validación de dimensiones en `BrushMask::new`,
    /// que es donde ya estaba.
    plantilla: BrushMask,
    capas: HashMap<SurfaceKey, BrushMask>,
}

impl BrushMasks {
    /// Contenedor vacío cuyas capas serán de `width` × `height`.
    ///
    /// # Pánico
    ///
    /// Con cualquier dimensión nula, por la misma razón y en el mismo sitio
    /// que `BrushMask::new`.
    pub fn new(width: usize, height: usize) -> Self {
        BrushMasks {
            plantilla: BrushMask::new(width, height),
            capas: HashMap::new(),
        }
    }

    pub fn width(&self) -> usize {
        self.plantilla.width()
    }

    pub fn height(&self) -> usize {
        self.plantilla.height()
    }

    /// Cuántas superficies tienen capa abierta.
    pub fn len(&self) -> usize {
        self.capas.len()
    }

    /// ¿No se ha pintado nada todavía?
    pub fn is_empty(&self) -> bool {
        self.capas.is_empty()
    }

    /// Máscara de una superficie, o `None` si nunca se pintó.
    ///
    /// Consultar **no** abre capa.
    pub fn mask(&self, key: SurfaceKey) -> Option<&BrushMask> {
        self.capas.get(&key)
    }

    /// Cobertura en un punto de una superficie.
    ///
    /// `0.0` si esa superficie no tiene capa, que es lo mismo que decir que
    /// nadie la ha pintado. El llamador no tiene que distinguir los dos
    /// casos: sin pintar y pintado a cero se ven igual.
    pub fn sample(&self, key: SurfaceKey, u: f32, v: f32) -> f32 {
        match self.capas.get(&key) {
            Some(capa) => capa.sample(u, v),
            None => 0.0,
        }
    }

    /// Estampa un disco sobre una superficie. Ver `BrushMask::stamp`.
    pub fn stamp(&mut self, key: SurfaceKey, u: f32, v: f32, radio: f32, opacidad: f32) {
        self.stroke(key, u, v, u, v, radio, opacidad);
    }

    /// Estampa una elipse sobre una superficie. Ver
    /// `BrushMask::stamp_ellipse`.
    #[allow(clippy::too_many_arguments)]
    pub fn stamp_ellipse(
        &mut self,
        key: SurfaceKey,
        u: f32,
        v: f32,
        radio_u: f32,
        radio_v: f32,
        opacidad: f32,
    ) {
        self.stroke_ellipse(key, u, v, u, v, radio_u, radio_v, opacidad);
    }

    /// Traza un segmento de grosor elíptico sobre una superficie.
    ///
    /// Misma elección de capa y misma asignación perezosa que `stroke`; la
    /// geometría es la de `BrushMask::stroke_ellipse`, entera y sin
    /// repetirla aquí.
    #[allow(clippy::too_many_arguments)]
    pub fn stroke_ellipse(
        &mut self,
        key: SurfaceKey,
        u0: f32,
        v0: f32,
        u1: f32,
        v1: f32,
        radio_u: f32,
        radio_v: f32,
        opacidad: f32,
    ) {
        if opacidad_efectiva(u0, v0, u1, v1, radio_u, radio_v, opacidad).is_none() {
            return;
        }

        let BrushMasks { plantilla, capas } = self;

        capas
            .entry(key)
            .or_insert_with(|| plantilla.clone())
            .stroke_ellipse(u0, v0, u1, v1, radio_u, radio_v, opacidad);
    }

    /// Traza un segmento sobre una superficie. Ver `BrushMask::stroke`.
    ///
    /// El rasterizado es el de `BrushMask`, entero y sin repetirlo aquí. Lo
    /// que añade este método es elegir la capa y abrirla solo si la entrada
    /// podía pintar.
    #[allow(clippy::too_many_arguments)]
    pub fn stroke(
        &mut self,
        key: SurfaceKey,
        u0: f32,
        v0: f32,
        u1: f32,
        v1: f32,
        radio: f32,
        opacidad: f32,
    ) {
        self.stroke_ellipse(key, u0, v0, u1, v1, radio, radio, opacidad);
    }

    /// Borra todas las capas.
    ///
    /// Suelta las máscaras en vez de rellenarlas de ceros: el estado
    /// «nadie ha pintado esto» es la ausencia de capa, y dejar rejillas
    /// vacías detrás contradiría la asignación perezosa.
    pub fn clear(&mut self) {
        self.capas.clear();
    }
}

/// Lo que hay pintado en un punto: un color lineal y cuánto cubre.
///
/// El color **no** está premultiplicado por `coverage`. Se guarda así
/// porque es lo que un consumidor querrá leer —«de qué color es esto»— sin
/// tener que deshacer una división, y porque con `coverage` en cero no
/// habría color que recuperar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pigment {
    /// Color en espacio **lineal**, que es donde mezclar tiene sentido.
    pub color: Color,
    /// Cuánto cubre, en `0.0..=1.0`. Es la alfa de la composición.
    pub coverage: f32,
}

impl Pigment {
    /// Celda sin pintar: sin cobertura y sin color que mostrar.
    const VACIO: Pigment = Pigment {
        color: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        },
        coverage: 0.0,
    };

    /// Compone una pincelada **encima** de lo que ya hubiera, `source-over`.
    ///
    /// La alfa resultante es `as + ad · (1 - as)` y el color el promedio de
    /// los dos pesado por lo que aporta cada uno. Con `as = 1` reemplaza, y
    /// con `as = 0` no tocaría nada —pero ese caso ni llega, porque
    /// `opacidad_efectiva` lo rechaza antes—.
    fn sobre(self, color: Color, alfa: f32) -> Pigment {
        let resto = self.coverage * (1.0 - alfa);
        let cobertura = alfa + resto;

        // El divisor solo puede ser cero si `alfa` lo era, y entonces no
        // habría pincelada. Se comprueba igualmente: una división entre
        // cero aquí envenenaría la celda para siempre.
        if cobertura <= 0.0 {
            return Pigment::VACIO;
        }

        Pigment {
            color: (color * alfa + self.color * resto) * (1.0 / cobertura),
            coverage: cobertura.min(1.0),
        }
    }
}

/// Una capa de pigmento: color y cobertura por celda.
#[derive(Debug, Clone)]
struct PigmentLayer {
    width: usize,
    height: usize,
    celdas: Vec<Pigment>,
}

impl PigmentLayer {
    fn new(width: usize, height: usize) -> Self {
        assert!(
            width > 0 && height > 0,
            "una capa de pigmento necesita dimension no nula, se pidio {width} x {height}"
        );

        PigmentLayer {
            width,
            height,
            celdas: vec![Pigment::VACIO; width * height],
        }
    }

    /// Pigmento en una `uv`, o `None` fuera de la superficie.
    ///
    /// El mapeo de `uv` a celda es el de `BrushMask::celda`, con la misma
    /// política: no envuelve, y una coordenada fuera de rango o no finita no
    /// pertenece a ninguna celda.
    fn sample(&self, u: f32, v: f32) -> Option<Pigment> {
        if !(0.0..=1.0).contains(&u) || !(0.0..=1.0).contains(&v) {
            return None;
        }

        let x = ((u * self.width as f32) as usize).min(self.width - 1);
        let y = ((v * self.height as f32) as usize).min(self.height - 1);

        Some(self.celdas[y * self.width + x])
    }

    /// Aplica una pincelada ya validada, con semiejes.
    #[allow(clippy::too_many_arguments)]
    fn stroke(
        &mut self,
        u0: f32,
        v0: f32,
        u1: f32,
        v1: f32,
        radio_u: f32,
        radio_v: f32,
        color: Color,
        alfa: f32,
    ) {
        let (ancho, alto) = (self.width, self.height);
        let celdas = &mut self.celdas;

        por_celda_cubierta(ancho, alto, u0, v0, u1, v1, radio_u, radio_v, |x, y| {
            let i = y * ancho + x;
            celdas[i] = celdas[i].sobre(color, alfa);
        });
    }

    /// Aplica una pincelada de textura ya validada.
    ///
    /// Cada celda cubierta muestrea la textura en **su propio centro**,
    /// escalado, y compone ese color como cualquier otro pigmento. Muestrear
    /// por celda y no una vez por pincelada es lo que hace que un trazo
    /// ancho muestre la tela y no un color plano sacado de su punto medio.
    ///
    /// La repetición o el recorte fuera de `0..1` los decide el `WrapMode`
    /// de la propia textura, dentro de `Texture::sample`. Aquí no se
    /// reimplementa ninguna de las dos políticas: la textura ya sabe cuál es
    /// la suya, y duplicarla sería otra copia que mantener sincronizada.
    #[allow(clippy::too_many_arguments)]
    fn stroke_texture(
        &mut self,
        u0: f32,
        v0: f32,
        u1: f32,
        v1: f32,
        radio_u: f32,
        radio_v: f32,
        texture: &Texture,
        escala: f32,
        alfa: f32,
    ) {
        let (ancho, alto) = (self.width, self.height);
        let celdas = &mut self.celdas;

        por_celda_cubierta(ancho, alto, u0, v0, u1, v1, radio_u, radio_v, |x, y| {
            let cu = (x as f32 + 0.5) / ancho as f32;
            let cv = (y as f32 + 0.5) / alto as f32;
            let color = texture.sample(cu * escala, cv * escala);
            let i = y * ancho + x;

            celdas[i] = celdas[i].sobre(color, alfa);
        });
    }
}

/// ¿Es un color que se puede componer?
///
/// Un canal no finito no se recupera: se propaga por cada mezcla posterior
/// y deja la celda envenenada para el resto de la sesión. Es la misma razón
/// por la que `opacidad_efectiva` rechaza una opacidad no finita.
fn color_componible(color: &Color) -> bool {
    color.r.is_finite() && color.g.is_finite() && color.b.is_finite()
}

/// Pigmento plano por superficie: una capa de color por `SurfaceKey`.
///
/// Comparte con `BrushMasks` la resolución fija, la asignación perezosa y el
/// rasterizador. Lo que cambia es qué guarda cada celda: aquí un color
/// lineal con su alfa, compuesto en **orden de pincelada**.
#[derive(Debug, Clone)]
pub struct PigmentMasks {
    plantilla: PigmentLayer,
    capas: HashMap<SurfaceKey, PigmentLayer>,
}

impl PigmentMasks {
    /// Contenedor vacío cuyas capas serán de `width` × `height`.
    ///
    /// # Pánico
    ///
    /// Con cualquier dimensión nula, por la misma razón que `BrushMask`.
    pub fn new(width: usize, height: usize) -> Self {
        PigmentMasks {
            plantilla: PigmentLayer::new(width, height),
            capas: HashMap::new(),
        }
    }

    pub fn width(&self) -> usize {
        self.plantilla.width
    }

    pub fn height(&self) -> usize {
        self.plantilla.height
    }

    /// Cuántas superficies tienen pigmento.
    pub fn len(&self) -> usize {
        self.capas.len()
    }

    /// ¿No se ha pintado nada todavía?
    pub fn is_empty(&self) -> bool {
        self.capas.is_empty()
    }

    /// Pigmento visible en un punto de una superficie.
    ///
    /// `None` si esa superficie no tiene capa, si la `uv` cae fuera, o si
    /// ahí no hay nada cubierto. Los tres casos significan lo mismo para
    /// quien pregunta —no hay color que mostrar—, y distinguirlos obligaría
    /// a cada llamador a decidir qué hacer con una diferencia que no puede
    /// ver.
    pub fn sample(&self, key: SurfaceKey, u: f32, v: f32) -> Option<Pigment> {
        let visible = self.capas.get(&key)?.sample(u, v)?;

        (visible.coverage > 0.0).then_some(visible)
    }

    /// Estampa un disco de pigmento. Ver `PigmentMasks::stroke`.
    pub fn stamp(&mut self, key: SurfaceKey, u: f32, v: f32, radio: f32, color: Color, alfa: f32) {
        self.stroke(key, u, v, u, v, radio, color, alfa);
    }

    /// Estampa una elipse de pigmento. Ver `PigmentMasks::stroke_ellipse`.
    #[allow(clippy::too_many_arguments)]
    pub fn stamp_ellipse(
        &mut self,
        key: SurfaceKey,
        u: f32,
        v: f32,
        radio_u: f32,
        radio_v: f32,
        color: Color,
        alfa: f32,
    ) {
        self.stroke_ellipse(key, u, v, u, v, radio_u, radio_v, color, alfa);
    }

    /// Traza un segmento de pigmento con grosor elíptico.
    ///
    /// Misma composición `source-over` y misma asignación perezosa que
    /// `stroke`; lo único que cambia es la figura.
    #[allow(clippy::too_many_arguments)]
    pub fn stroke_ellipse(
        &mut self,
        key: SurfaceKey,
        u0: f32,
        v0: f32,
        u1: f32,
        v1: f32,
        radio_u: f32,
        radio_v: f32,
        color: Color,
        alfa: f32,
    ) {
        let Some(alfa) = opacidad_efectiva(u0, v0, u1, v1, radio_u, radio_v, alfa) else {
            return;
        };

        if !color_componible(&color) {
            return;
        }

        let PigmentMasks { plantilla, capas } = self;

        capas
            .entry(key)
            .or_insert_with(|| plantilla.clone())
            .stroke(u0, v0, u1, v1, radio_u, radio_v, color, alfa);
    }

    /// Traza un segmento de pigmento sobre una superficie.
    ///
    /// La figura es la de `BrushMask::stroke` —la misma cápsula, el mismo
    /// borde duro, la misma política de `uv`— y cada celda cubierta recibe
    /// la pincelada **una vez**. Lo que cambia es que en vez de sumar
    /// cobertura se compone color sobre lo que ya hubiera.
    ///
    /// Es un no-op silencioso con las mismas entradas que rechaza
    /// `BrushMasks::stroke`, más un color con algún canal no finito. Y como
    /// allí, un no-op **no abre capa**.
    #[allow(clippy::too_many_arguments)]
    pub fn stroke(
        &mut self,
        key: SurfaceKey,
        u0: f32,
        v0: f32,
        u1: f32,
        v1: f32,
        radio: f32,
        color: Color,
        alfa: f32,
    ) {
        self.stroke_ellipse(key, u0, v0, u1, v1, radio, radio, color, alfa);
    }

    /// Estampa un disco de textura. Ver `PigmentMasks::stroke_texture`.
    #[allow(clippy::too_many_arguments)]
    pub fn stamp_texture(
        &mut self,
        key: SurfaceKey,
        u: f32,
        v: f32,
        radio: f32,
        texture: &Texture,
        escala: f32,
        alfa: f32,
    ) {
        self.stroke_texture(key, u, v, u, v, radio, texture, escala, alfa);
    }

    /// Traza un segmento de textura sobre una superficie.
    ///
    /// Misma figura, misma continuidad y misma asignación perezosa que
    /// `PigmentMasks::stroke`; lo que cambia es de dónde sale el color de
    /// cada celda. El resultado queda **estampado**: la capa guarda los
    /// colores muestreados, no la textura ni una referencia a ella, así que
    /// lo pintado no cambia si la textura se descarta después, y la textura
    /// de origen no se toca.
    ///
    /// `escala` multiplica las `uv` antes de muestrear: mayor que uno
    /// aprieta el motivo y lo repite —o lo recorta— según el `WrapMode` de
    /// la textura.
    ///
    /// Es un no-op silencioso con las mismas entradas que rechaza
    /// `PigmentMasks::stroke`, más una escala que no sea finita y
    /// estrictamente positiva: con cero o negativa el muestreo no
    /// significaría nada, y con infinito o `NaN` la coordenada dejaría de
    /// serlo. Y como allí, un no-op **no abre capa**.
    #[allow(clippy::too_many_arguments)]
    pub fn stroke_texture(
        &mut self,
        key: SurfaceKey,
        u0: f32,
        v0: f32,
        u1: f32,
        v1: f32,
        radio: f32,
        texture: &Texture,
        escala: f32,
        alfa: f32,
    ) {
        self.stroke_texture_ellipse(key, u0, v0, u1, v1, radio, radio, texture, escala, alfa);
    }

    /// Estampa una elipse de textura. Ver
    /// `PigmentMasks::stroke_texture_ellipse`.
    #[allow(clippy::too_many_arguments)]
    pub fn stamp_texture_ellipse(
        &mut self,
        key: SurfaceKey,
        u: f32,
        v: f32,
        radio_u: f32,
        radio_v: f32,
        texture: &Texture,
        escala: f32,
        alfa: f32,
    ) {
        self.stroke_texture_ellipse(key, u, v, u, v, radio_u, radio_v, texture, escala, alfa);
    }

    /// Traza un segmento de textura con grosor elíptico.
    ///
    /// La figura es la de `BrushMask::stroke_ellipse` y el muestreo es el de
    /// `Texture::sample`, los dos sin tocar. Rechaza lo mismo que
    /// `stroke_ellipse` más una escala que no sea finita y estrictamente
    /// positiva, y un no-op **no abre capa**.
    #[allow(clippy::too_many_arguments)]
    pub fn stroke_texture_ellipse(
        &mut self,
        key: SurfaceKey,
        u0: f32,
        v0: f32,
        u1: f32,
        v1: f32,
        radio_u: f32,
        radio_v: f32,
        texture: &Texture,
        escala: f32,
        alfa: f32,
    ) {
        let Some(alfa) = opacidad_efectiva(u0, v0, u1, v1, radio_u, radio_v, alfa) else {
            return;
        };

        if !escala.is_finite() || escala <= 0.0 {
            return;
        }

        let PigmentMasks { plantilla, capas } = self;

        capas
            .entry(key)
            .or_insert_with(|| plantilla.clone())
            .stroke_texture(u0, v0, u1, v1, radio_u, radio_v, texture, escala, alfa);
    }

    /// Borra todo el pigmento.
    pub fn clear(&mut self) {
        self.capas.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::texture::WrapMode;

    /// Lado de la máscara de trabajo. Bastante grande para que un disco de
    /// radio pequeño cubra varios píxeles y deje esquinas lejanas intactas.
    const LADO: usize = 32;

    fn mascara() -> BrushMask {
        BrushMask::new(LADO, LADO)
    }

    /// Cobertura máxima de toda la máscara. Sirve para afirmar que una
    /// operación fue un no-op: si el máximo sigue en cero, nadie pintó.
    fn maximo(mascara: &BrushMask) -> f32 {
        let mut mayor = 0.0f32;
        for y in 0..mascara.height() {
            for x in 0..mascara.width() {
                mayor = mayor.max(mascara.pixel(x, y));
            }
        }
        mayor
    }

    #[test]
    fn una_mascara_nueva_esta_vacia_y_conserva_sus_dimensiones() {
        let m = BrushMask::new(7, 3);

        assert_eq!(m.width(), 7);
        assert_eq!(m.height(), 3);
        assert_eq!(
            maximo(&m),
            0.0,
            "una mascara recien creada no tiene pintura"
        );
        assert_eq!(m.sample(0.5, 0.5), 0.0);
    }

    #[test]
    #[should_panic(expected = "dimension")]
    fn una_dimension_cero_no_es_una_mascara() {
        // No hay celda que consultar y toda `uv` caería en un hueco. Es un
        // error de programación, no una entrada del usuario: falla ruidoso.
        BrushMask::new(0, 4);
    }

    #[test]
    #[should_panic(expected = "fuera de la mascara")]
    fn consultar_un_pixel_inexistente_es_un_error_de_programacion() {
        let m = BrushMask::new(4, 4);

        m.pixel(4, 0);
    }

    #[test]
    fn el_sello_central_pinta_el_centro_y_no_la_esquina() {
        let mut m = mascara();

        m.stamp(0.5, 0.5, 0.1, 1.0);

        assert_eq!(m.sample(0.5, 0.5), 1.0, "el centro del sello queda pintado");
        assert_eq!(
            m.sample(0.02, 0.02),
            0.0,
            "una esquina lejana no se contamina"
        );
        assert_eq!(m.sample(0.98, 0.98), 0.0);
    }

    #[test]
    fn la_opacidad_se_acumula_y_se_satura_en_uno() {
        let mut m = mascara();

        m.stamp(0.5, 0.5, 0.1, 0.4);
        assert!((m.sample(0.5, 0.5) - 0.4).abs() < 1e-6);

        m.stamp(0.5, 0.5, 0.1, 0.4);
        assert!(
            (m.sample(0.5, 0.5) - 0.8).abs() < 1e-6,
            "dos pasadas suman, dio {}",
            m.sample(0.5, 0.5)
        );

        m.stamp(0.5, 0.5, 0.1, 0.4);
        assert_eq!(
            m.sample(0.5, 0.5),
            1.0,
            "la tercera satura exactamente en uno, no en 1.2"
        );

        assert_eq!(maximo(&m), 1.0, "ninguna celda se pasa de uno");
    }

    #[test]
    fn una_opacidad_mayor_que_uno_no_desborda_la_mascara() {
        let mut m = mascara();

        m.stamp(0.5, 0.5, 0.1, 4.0);

        assert_eq!(m.sample(0.5, 0.5), 1.0);
        assert_eq!(maximo(&m), 1.0);
    }

    #[test]
    fn el_trazo_deja_cobertura_continua_en_todo_su_tramo() {
        let mut m = mascara();
        let (u0, v0) = (0.2f32, 0.5f32);
        let (u1, v1) = (0.8f32, 0.5f32);

        m.stroke(u0, v0, u1, v1, 0.05, 1.0);

        // Cien puntos sobre el segmento: si el trazo tuviera huecos, alguno
        // caería en uno. Es la diferencia entre un segmento y una ristra de
        // sellos sueltos.
        for paso in 0..=100 {
            let t = paso as f32 / 100.0;
            let u = u0 + (u1 - u0) * t;
            let v = v0 + (v1 - v0) * t;

            assert_eq!(
                m.sample(u, v),
                1.0,
                "el trazo tiene un hueco en t = {t} (uv {u}, {v})"
            );
        }

        assert_eq!(m.sample(0.5, 0.05), 0.0, "el trazo no se sale de su banda");
        assert_eq!(m.sample(0.05, 0.5), 0.0, "ni se pasa del extremo inicial");
        assert_eq!(m.sample(0.95, 0.5), 0.0, "ni del final");
    }

    #[test]
    fn el_trazo_no_acumula_dos_veces_sobre_la_misma_celda() {
        // Un trazo es **una** pasada de pincel. Si se implementara como
        // sellos solapados a lo largo del segmento, cada celda recibiría la
        // opacidad varias veces y un trazo suave saldría saturado.
        let mut m = mascara();

        m.stroke(0.2, 0.5, 0.8, 0.5, 0.05, 0.5);

        assert!(
            (m.sample(0.5, 0.5) - 0.5).abs() < 1e-6,
            "una pasada de opacidad 0.5 deja 0.5, dio {}",
            m.sample(0.5, 0.5)
        );
        assert!(
            maximo(&m) <= 0.5 + 1e-6,
            "ninguna celda recibio dos pasadas"
        );
    }

    #[test]
    fn un_trazo_degenerado_equivale_a_un_sello() {
        let mut trazo = mascara();
        let mut sello = mascara();

        trazo.stroke(0.4, 0.6, 0.4, 0.6, 0.08, 0.7);
        sello.stamp(0.4, 0.6, 0.08, 0.7);

        for y in 0..LADO {
            for x in 0..LADO {
                assert_eq!(
                    trazo.pixel(x, y),
                    sello.pixel(x, y),
                    "difieren en ({x}, {y})"
                );
            }
        }
    }

    #[test]
    fn la_celda_y_la_uv_nombran_el_mismo_sitio_con_v_cero_abajo() {
        // Radio menor que media celda y centrado en el centro exacto de una:
        // solo esa puede quedar pintada. Fija dos cosas a la vez, el mapeo
        // `uv` -> celda y que la fila 0 es la de `v` pequeña, que es la
        // convención del resto del proyecto.
        let mut m = BrushMask::new(4, 4);
        let (x, y) = (3usize, 0usize);
        let u = (x as f32 + 0.5) / 4.0;
        let v = (y as f32 + 0.5) / 4.0;

        m.stamp(u, v, 0.05, 1.0);

        assert_eq!(m.pixel(x, y), 1.0, "la celda del centro del sello");
        assert_eq!(m.sample(u, v), 1.0, "y la misma consultada por uv");

        for celda_y in 0..4 {
            for celda_x in 0..4 {
                if (celda_x, celda_y) != (x, y) {
                    assert_eq!(
                        m.pixel(celda_x, celda_y),
                        0.0,
                        "({celda_x}, {celda_y}) no deberia tener pintura"
                    );
                }
            }
        }
    }

    #[test]
    fn las_uv_fuera_de_rango_no_pintan_ni_envuelven() {
        let mut m = mascara();

        m.stamp(1.5, 0.5, 0.2, 1.0);
        m.stamp(-0.2, 0.5, 0.2, 1.0);
        m.stamp(0.5, 1.001, 0.2, 1.0);
        m.stroke(1.4, 0.5, 1.6, 0.5, 0.2, 1.0);

        assert_eq!(
            maximo(&m),
            0.0,
            "fuera de 0..1 no hay superficie: ni se pinta ni se envuelve al otro borde"
        );
    }

    #[test]
    fn consultar_fuera_de_rango_devuelve_cero() {
        let mut m = mascara();
        m.stamp(0.5, 0.5, 0.6, 1.0);

        assert_eq!(m.sample(-0.01, 0.5), 0.0);
        assert_eq!(m.sample(1.01, 0.5), 0.0);
        assert_eq!(m.sample(0.5, -0.01), 0.0);
        assert_eq!(m.sample(0.5, 1.01), 0.0);
        assert_eq!(m.sample(f32::NAN, 0.5), 0.0);
        assert_eq!(m.sample(0.5, f32::INFINITY), 0.0);
    }

    #[test]
    fn las_entradas_no_finitas_son_no_ops_seguras() {
        let mut m = mascara();
        let raros = [f32::NAN, f32::INFINITY, f32::NEG_INFINITY];

        for raro in raros {
            m.stamp(raro, 0.5, 0.1, 1.0);
            m.stamp(0.5, raro, 0.1, 1.0);
            m.stamp(0.5, 0.5, raro, 1.0);
            m.stamp(0.5, 0.5, 0.1, raro);
            m.stroke(raro, 0.5, 0.8, 0.5, 0.1, 1.0);
            m.stroke(0.2, 0.5, 0.8, raro, 0.1, 1.0);
            m.stroke(0.2, 0.5, 0.8, 0.5, raro, 1.0);
            m.stroke(0.2, 0.5, 0.8, 0.5, 0.1, raro);
        }

        assert_eq!(maximo(&m), 0.0, "ninguna entrada no finita pinto nada");
    }

    #[test]
    fn un_radio_o_una_opacidad_sin_efecto_no_pintan() {
        let mut m = mascara();

        m.stamp(0.5, 0.5, 0.0, 1.0);
        m.stamp(0.5, 0.5, -0.3, 1.0);
        m.stamp(0.5, 0.5, 0.1, 0.0);
        m.stamp(0.5, 0.5, 0.1, -1.0);
        m.stroke(0.2, 0.5, 0.8, 0.5, -0.1, 1.0);
        m.stroke(0.2, 0.5, 0.8, 0.5, 0.1, 0.0);

        assert_eq!(maximo(&m), 0.0);
    }

    #[test]
    fn la_misma_secuencia_produce_la_misma_mascara() {
        // Determinismo: no hay azar, no hay orden de iteración observable y
        // no hay estado compartido. Dos máscaras que reciben lo mismo son
        // indistinguibles.
        let guion = |m: &mut BrushMask| {
            m.stamp(0.31, 0.72, 0.13, 0.35);
            m.stroke(0.1, 0.1, 0.9, 0.85, 0.07, 0.6);
            m.stamp(0.5, 0.5, 0.2, 0.9);
        };

        let mut primera = mascara();
        let mut segunda = mascara();
        guion(&mut primera);
        guion(&mut segunda);

        for y in 0..LADO {
            for x in 0..LADO {
                assert_eq!(
                    primera.pixel(x, y),
                    segunda.pixel(x, y),
                    "difieren en ({x}, {y})"
                );
            }
        }
        assert!(maximo(&primera) > 0.0, "el guion tiene que pintar algo");
    }

    // ------------------------------------------------ capsula eliptica

    /// Lado de las mascaras de los tests de elipse. Mas fino que `LADO`
    /// para que una extension de `0.05` uv se pueda contar en celdas.
    const FINO: usize = 64;

    /// Cuantas celdas seguidas cubre una fila, y cuantas una columna, por
    /// el centro de la mascara.
    fn extension(mascara: &BrushMask) -> (usize, usize) {
        let medio = FINO / 2;
        let horizontal = (0..FINO).filter(|x| mascara.pixel(*x, medio) > 0.0).count();
        let vertical = (0..FINO).filter(|y| mascara.pixel(medio, *y) > 0.0).count();

        (horizontal, vertical)
    }

    #[test]
    fn una_elipse_de_radios_iguales_es_el_circulo_de_siempre() {
        // No «parecida»: la misma. El circulo delega en la elipse, asi que
        // no hay dos geometrias que puedan separarse con el tiempo.
        let trazos = [
            (0.5f32, 0.5f32, 0.5f32, 0.5f32, 0.13f32),
            (0.2, 0.5, 0.8, 0.5, 0.05),
            (0.1, 0.12, 0.88, 0.9, 0.07),
        ];

        for (u0, v0, u1, v1, radio) in trazos {
            let mut circular = BrushMask::new(FINO, FINO);
            let mut eliptica = BrushMask::new(FINO, FINO);

            circular.stroke(u0, v0, u1, v1, radio, 0.7);
            eliptica.stroke_ellipse(u0, v0, u1, v1, radio, radio, 0.7);

            for y in 0..FINO {
                for x in 0..FINO {
                    assert_eq!(
                        circular.pixel(x, y),
                        eliptica.pixel(x, y),
                        "el trazo {u0},{v0}..{u1},{v1} difiere en ({x}, {y})"
                    );
                }
            }
        }
    }

    #[test]
    fn un_sello_ancho_en_u_se_extiende_mas_en_horizontal() {
        let mut mascara = BrushMask::new(FINO, FINO);

        mascara.stamp_ellipse(0.5, 0.5, 0.20, 0.05, 1.0);

        let (horizontal, vertical) = extension(&mascara);

        assert!(
            horizontal > vertical * 3,
            "horizontal {horizontal} contra vertical {vertical}"
        );

        // Y las extensiones son las que dicen los radios, no una redonda
        // de compromiso: `2 r` de ancho en cada eje.
        let esperado_h = (2.0 * 0.20 * FINO as f32) as usize;
        let esperado_v = (2.0 * 0.05 * FINO as f32) as usize;

        assert!(
            horizontal.abs_diff(esperado_h) <= 2,
            "horizontal {horizontal}, se esperaba cerca de {esperado_h}"
        );
        assert!(
            vertical.abs_diff(esperado_v) <= 2,
            "vertical {vertical}, se esperaba cerca de {esperado_v}"
        );

        // La anisotropia, dicha sin contar celdas: a la misma distancia del
        // centro, dentro por el eje largo y fuera por el corto.
        assert!(mascara.sample(0.65, 0.5) > 0.0, "dentro por el eje ancho");
        assert_eq!(mascara.sample(0.5, 0.65), 0.0, "fuera por el eje fino");
    }

    #[test]
    fn un_sello_alto_en_v_se_extiende_mas_en_vertical() {
        let mut mascara = BrushMask::new(FINO, FINO);

        mascara.stamp_ellipse(0.5, 0.5, 0.05, 0.20, 1.0);

        let (horizontal, vertical) = extension(&mascara);

        assert!(
            vertical > horizontal * 3,
            "vertical {vertical} contra horizontal {horizontal}"
        );
        assert!(mascara.sample(0.5, 0.65) > 0.0, "dentro por el eje alto");
        assert_eq!(mascara.sample(0.65, 0.5), 0.0, "fuera por el eje fino");
    }

    #[test]
    fn el_trazo_eliptico_es_continuo_y_conserva_su_grosor() {
        let mut mascara = BrushMask::new(FINO, FINO);
        let (u0, v0) = (0.2f32, 0.5f32);
        let (u1, v1) = (0.8f32, 0.5f32);

        mascara.stroke_ellipse(u0, v0, u1, v1, 0.03, 0.10, 1.0);

        for paso in 0..=100 {
            let t = paso as f32 / 100.0;
            let u = u0 + (u1 - u0) * t;

            assert_eq!(mascara.sample(u, v0), 1.0, "hueco en t = {t}");
        }

        // El grosor perpendicular al trazo lo manda `radio_v`: a `0.08` del
        // eje sigue dentro, y a `0.12` ya no.
        assert!(mascara.sample(0.5, 0.58) > 0.0, "dentro del grosor");
        assert_eq!(mascara.sample(0.5, 0.63), 0.0, "fuera del grosor");
    }

    #[test]
    fn una_elipse_con_entrada_invalida_no_pinta() {
        let mut mascara = BrushMask::new(FINO, FINO);
        let raros = [0.0f32, -0.3, f32::NAN, f32::INFINITY];

        for raro in raros {
            mascara.stamp_ellipse(0.5, 0.5, raro, 0.1, 1.0);
            mascara.stamp_ellipse(0.5, 0.5, 0.1, raro, 1.0);
            mascara.stroke_ellipse(0.2, 0.5, 0.8, 0.5, raro, 0.1, 1.0);
            mascara.stroke_ellipse(0.2, 0.5, 0.8, 0.5, 0.1, raro, 1.0);
        }

        // Y lo de siempre: uv fuera de rango, no finitas y opacidad nula.
        mascara.stamp_ellipse(1.5, 0.5, 0.1, 0.1, 1.0);
        mascara.stamp_ellipse(f32::NAN, 0.5, 0.1, 0.1, 1.0);
        mascara.stamp_ellipse(0.5, 0.5, 0.1, 0.1, 0.0);
        mascara.stamp_ellipse(0.5, 0.5, 0.1, 0.1, f32::NAN);
        mascara.stroke_ellipse(0.2, 0.5, 1.4, 0.5, 0.1, 0.1, 1.0);

        for y in 0..FINO {
            for x in 0..FINO {
                assert_eq!(mascara.pixel(x, y), 0.0, "({x}, {y}) se pinto");
            }
        }
    }

    #[test]
    fn un_sello_eliptico_equivale_a_su_trazo_degenerado() {
        let mut sello = BrushMask::new(FINO, FINO);
        let mut trazo = BrushMask::new(FINO, FINO);

        sello.stamp_ellipse(0.4, 0.6, 0.12, 0.04, 0.8);
        trazo.stroke_ellipse(0.4, 0.6, 0.4, 0.6, 0.12, 0.04, 0.8);

        for y in 0..FINO {
            for x in 0..FINO {
                assert_eq!(sello.pixel(x, y), trazo.pixel(x, y), "({x}, {y})");
            }
        }
    }

    // ------------------------------------ una mascara por superficie

    const CLAVE_A: SurfaceKey = SurfaceKey::new(7, UvChart::new(1));

    /// Misma carta, otro objeto.
    const OTRO_OBJETO: SurfaceKey = SurfaceKey::new(9, UvChart::new(1));

    /// Mismo objeto, otra carta: la cara opuesta de un cuboide.
    const OTRA_CARTA: SurfaceKey = SurfaceKey::new(7, UvChart::new(2));

    fn capas() -> BrushMasks {
        BrushMasks::new(LADO, LADO)
    }

    #[test]
    fn una_superficie_sin_pintar_no_tiene_cobertura_ni_capa() {
        // Consultar no crea. Es lo que permite preguntar por cada impacto
        // del renderer sin que mirar el diorama lo llene de capas vacias.
        let capas = capas();

        assert_eq!(capas.sample(CLAVE_A, 0.5, 0.5), 0.0);
        assert!(
            capas.mask(CLAVE_A).is_none(),
            "consultar no debe crear capa"
        );
        assert!(capas.is_empty());
        assert_eq!(capas.len(), 0);
    }

    #[test]
    fn la_misma_clave_acumula_en_la_misma_capa() {
        let mut capas = capas();

        capas.stamp(CLAVE_A, 0.5, 0.5, 0.1, 0.4);
        capas.stamp(CLAVE_A, 0.5, 0.5, 0.1, 0.4);

        assert!(
            (capas.sample(CLAVE_A, 0.5, 0.5) - 0.8).abs() < 1e-6,
            "dio {}",
            capas.sample(CLAVE_A, 0.5, 0.5)
        );
        assert_eq!(capas.len(), 1, "las dos pasadas son la misma superficie");
    }

    #[test]
    fn dos_cartas_del_mismo_objeto_son_independientes() {
        // El caso que motiva la clave: las caras `+X` y `-X` de un cuboide
        // recorren las mismas `uv`. Pintar una no puede manchar la otra.
        let mut capas = capas();

        capas.stamp(CLAVE_A, 0.5, 0.5, 0.1, 1.0);

        assert_eq!(capas.sample(CLAVE_A, 0.5, 0.5), 1.0);
        assert_eq!(
            capas.sample(OTRA_CARTA, 0.5, 0.5),
            0.0,
            "la otra carta del mismo objeto no se pinta"
        );
        assert_eq!(capas.len(), 1);
    }

    #[test]
    fn dos_objetos_con_la_misma_carta_son_independientes() {
        // Y la otra mitad de la clave: veintiocho pilares comparten la
        // carta lateral, y son veintiocho superficies distintas.
        let mut capas = capas();

        capas.stamp(CLAVE_A, 0.5, 0.5, 0.1, 1.0);

        assert_eq!(capas.sample(CLAVE_A, 0.5, 0.5), 1.0);
        assert_eq!(capas.sample(OTRO_OBJETO, 0.5, 0.5), 0.0);
        assert_eq!(capas.len(), 1);
    }

    #[test]
    fn una_entrada_invalida_no_deja_capa_observable() {
        // Un clic que no cae en superficie no puede costar una capa: seria
        // una fuga silenciosa proporcional a cuanto falle el puntero.
        let mut capas = capas();

        capas.stamp(CLAVE_A, 1.5, 0.5, 0.1, 1.0);
        capas.stamp(CLAVE_A, f32::NAN, 0.5, 0.1, 1.0);
        capas.stamp(CLAVE_A, 0.5, 0.5, 0.0, 1.0);
        capas.stamp(CLAVE_A, 0.5, 0.5, 0.1, 0.0);
        capas.stroke(CLAVE_A, 0.2, 0.5, 1.4, 0.5, 0.1, 1.0);
        capas.stroke(CLAVE_A, 0.2, 0.5, 0.8, 0.5, f32::INFINITY, 1.0);

        assert!(capas.is_empty(), "ninguna entrada invalida abre una capa");
        assert!(capas.mask(CLAVE_A).is_none());
        assert_eq!(capas.sample(CLAVE_A, 0.5, 0.5), 0.0);
    }

    #[test]
    fn clear_vuelve_todo_a_cero() {
        let mut capas = capas();

        capas.stamp(CLAVE_A, 0.5, 0.5, 0.2, 1.0);
        capas.stamp(OTRA_CARTA, 0.5, 0.5, 0.2, 1.0);
        capas.stamp(OTRO_OBJETO, 0.5, 0.5, 0.2, 1.0);
        assert_eq!(capas.len(), 3);

        capas.clear();

        assert!(capas.is_empty());
        for clave in [CLAVE_A, OTRA_CARTA, OTRO_OBJETO] {
            assert_eq!(capas.sample(clave, 0.5, 0.5), 0.0);
            assert!(capas.mask(clave).is_none());
        }
    }

    #[test]
    fn el_trazo_de_una_clave_es_continuo_y_no_toca_a_las_demas() {
        // El rasterizador es el de `BrushMask`: lo que este test comprueba
        // es que el contenedor lo aplica entero sobre **una** capa.
        let mut capas = capas();
        let (u0, v0) = (0.2f32, 0.5f32);
        let (u1, v1) = (0.8f32, 0.5f32);

        capas.stroke(CLAVE_A, u0, v0, u1, v1, 0.05, 1.0);

        for paso in 0..=100 {
            let t = paso as f32 / 100.0;
            let u = u0 + (u1 - u0) * t;
            let v = v0 + (v1 - v0) * t;

            assert_eq!(capas.sample(CLAVE_A, u, v), 1.0, "hueco en t = {t}");
            assert_eq!(capas.sample(OTRA_CARTA, u, v), 0.0);
            assert_eq!(capas.sample(OTRO_OBJETO, u, v), 0.0);
        }

        assert_eq!(capas.len(), 1);
    }

    #[test]
    fn las_capas_heredan_la_resolucion_del_contenedor() {
        let mut capas = BrushMasks::new(16, 4);

        assert_eq!(capas.width(), 16);
        assert_eq!(capas.height(), 4);

        capas.stamp(CLAVE_A, 0.5, 0.5, 0.1, 1.0);
        let capa = capas.mask(CLAVE_A).expect("la capa existe");

        assert_eq!(capa.width(), 16);
        assert_eq!(capa.height(), 4);
    }

    #[test]
    fn una_clave_es_el_objeto_y_la_carta() {
        assert_eq!(CLAVE_A, SurfaceKey::new(7, UvChart::new(1)));
        assert_ne!(CLAVE_A, OTRA_CARTA, "misma pieza, otra cara");
        assert_ne!(CLAVE_A, OTRO_OBJETO, "misma cara, otra pieza");
    }

    // ------------------------------------------- pigmento por superficie

    const ROJO: Color = Color {
        r: 0.8,
        g: 0.1,
        b: 0.1,
    };
    const AZUL: Color = Color {
        r: 0.1,
        g: 0.2,
        b: 0.7,
    };

    fn pigmentos() -> PigmentMasks {
        PigmentMasks::new(LADO, LADO)
    }

    fn cerca(a: Color, b: Color) -> bool {
        (a.r - b.r).abs() < 1e-5 && (a.g - b.g).abs() < 1e-5 && (a.b - b.b).abs() < 1e-5
    }

    #[test]
    fn un_pigmento_nuevo_no_tiene_nada_que_mostrar() {
        let pigmentos = pigmentos();

        assert_eq!(pigmentos.sample(CLAVE_A, 0.5, 0.5), None);
        assert!(pigmentos.is_empty());
        assert_eq!(pigmentos.len(), 0);
        assert_eq!(pigmentos.width(), LADO);
        assert_eq!(pigmentos.height(), LADO);
    }

    #[test]
    fn una_pasada_opaca_deja_el_color_exacto() {
        let mut pigmentos = pigmentos();

        pigmentos.stamp(CLAVE_A, 0.5, 0.5, 0.1, ROJO, 1.0);

        let visible = pigmentos.sample(CLAVE_A, 0.5, 0.5).expect("hay pigmento");

        assert_eq!(visible.color, ROJO, "una pasada opaca no mezcla con nada");
        assert_eq!(visible.coverage, 1.0);
        assert_eq!(pigmentos.len(), 1);
    }

    #[test]
    fn una_pasada_opaca_reemplaza_a_la_anterior() {
        let mut pigmentos = pigmentos();

        pigmentos.stamp(CLAVE_A, 0.5, 0.5, 0.1, AZUL, 1.0);
        pigmentos.stamp(CLAVE_A, 0.5, 0.5, 0.1, ROJO, 1.0);

        let visible = pigmentos.sample(CLAVE_A, 0.5, 0.5).expect("hay pigmento");

        assert_eq!(
            visible.color, ROJO,
            "la ultima pincelada tapa a la anterior"
        );
        assert_eq!(visible.coverage, 1.0);
    }

    #[test]
    fn dos_pasadas_semitransparentes_se_componen_en_lineal() {
        // `source-over` sin premultiplicar: sobre azul a `0.5` cae rojo a
        // `0.5`, asi que la cobertura sube a `0.75` y el color queda a dos
        // tercios de rojo.
        let mut pigmentos = pigmentos();

        pigmentos.stamp(CLAVE_A, 0.5, 0.5, 0.1, AZUL, 0.5);
        pigmentos.stamp(CLAVE_A, 0.5, 0.5, 0.1, ROJO, 0.5);

        let visible = pigmentos.sample(CLAVE_A, 0.5, 0.5).expect("hay pigmento");
        let esperado = (ROJO * 0.5 + AZUL * 0.25) * (1.0 / 0.75);

        assert!(
            (visible.coverage - 0.75).abs() < 1e-6,
            "alpha dio {}",
            visible.coverage
        );
        assert!(
            cerca(visible.color, esperado),
            "color dio {:?} y se esperaba {esperado:?}",
            visible.color
        );
    }

    #[test]
    fn la_cobertura_del_pigmento_se_queda_en_cero_uno() {
        let mut pigmentos = pigmentos();

        for _ in 0..8 {
            pigmentos.stamp(CLAVE_A, 0.5, 0.5, 0.1, ROJO, 0.6);
        }

        let visible = pigmentos.sample(CLAVE_A, 0.5, 0.5).expect("hay pigmento");

        assert!(
            (0.0..=1.0).contains(&visible.coverage),
            "alpha se salio: {}",
            visible.coverage
        );
        assert!(
            visible.coverage > 0.99,
            "ocho pasadas deberian casi saturar"
        );
    }

    #[test]
    fn el_trazo_de_pigmento_es_continuo() {
        let mut pigmentos = pigmentos();
        let (u0, v0) = (0.2f32, 0.5f32);
        let (u1, v1) = (0.8f32, 0.5f32);

        pigmentos.stroke(CLAVE_A, u0, v0, u1, v1, 0.05, ROJO, 1.0);

        for paso in 0..=100 {
            let t = paso as f32 / 100.0;
            let u = u0 + (u1 - u0) * t;
            let v = v0 + (v1 - v0) * t;

            let visible = pigmentos
                .sample(CLAVE_A, u, v)
                .unwrap_or_else(|| panic!("hueco en t = {t}"));

            assert_eq!(visible.color, ROJO);
            assert_eq!(visible.coverage, 1.0);
        }

        assert_eq!(pigmentos.sample(CLAVE_A, 0.5, 0.05), None, "no se sale");
    }

    #[test]
    fn el_pigmento_de_una_clave_no_mancha_a_las_demas() {
        let mut pigmentos = pigmentos();

        pigmentos.stamp(CLAVE_A, 0.5, 0.5, 0.2, ROJO, 1.0);

        assert!(pigmentos.sample(CLAVE_A, 0.5, 0.5).is_some());
        assert_eq!(
            pigmentos.sample(OTRA_CARTA, 0.5, 0.5),
            None,
            "otra carta del mismo objeto"
        );
        assert_eq!(
            pigmentos.sample(OTRO_OBJETO, 0.5, 0.5),
            None,
            "otro objeto con la misma carta"
        );
        assert_eq!(pigmentos.len(), 1);
    }

    #[test]
    fn una_entrada_invalida_no_deja_pigmento_ni_capa() {
        let mut pigmentos = pigmentos();
        let roto = Color::new(f32::NAN, 0.5, 0.5);

        pigmentos.stamp(CLAVE_A, 1.5, 0.5, 0.1, ROJO, 1.0);
        pigmentos.stamp(CLAVE_A, f32::NAN, 0.5, 0.1, ROJO, 1.0);
        pigmentos.stamp(CLAVE_A, 0.5, 0.5, 0.0, ROJO, 1.0);
        pigmentos.stamp(CLAVE_A, 0.5, 0.5, 0.1, ROJO, 0.0);
        pigmentos.stamp(CLAVE_A, 0.5, 0.5, 0.1, ROJO, f32::NAN);
        pigmentos.stamp(CLAVE_A, 0.5, 0.5, 0.1, roto, 1.0);
        pigmentos.stroke(CLAVE_A, 0.2, 0.5, 1.4, 0.5, 0.1, ROJO, 1.0);
        pigmentos.stroke(CLAVE_A, 0.2, 0.5, 0.8, 0.5, f32::INFINITY, ROJO, 1.0);

        assert!(pigmentos.is_empty(), "ninguna entrada invalida abre capa");
        assert_eq!(pigmentos.sample(CLAVE_A, 0.5, 0.5), None);
    }

    #[test]
    fn clear_borra_todo_el_pigmento() {
        let mut pigmentos = pigmentos();

        pigmentos.stamp(CLAVE_A, 0.5, 0.5, 0.2, ROJO, 1.0);
        pigmentos.stamp(OTRA_CARTA, 0.5, 0.5, 0.2, AZUL, 1.0);
        assert_eq!(pigmentos.len(), 2);

        pigmentos.clear();

        assert!(pigmentos.is_empty());
        assert_eq!(pigmentos.sample(CLAVE_A, 0.5, 0.5), None);
        assert_eq!(pigmentos.sample(OTRA_CARTA, 0.5, 0.5), None);
    }

    #[test]
    fn el_pigmento_cubre_exactamente_lo_mismo_que_la_mascara() {
        // La geometria es una sola: misma capsula, mismo borde duro, mismo
        // criterio de celda. Si divergieran, un trazo de revelado y uno de
        // color dejarian siluetas distintas sobre la misma superficie.
        let mut mascara = BrushMask::new(LADO, LADO);
        let mut pigmentos = pigmentos();

        let trazos = [
            (0.2f32, 0.5f32, 0.8f32, 0.5f32, 0.05f32),
            (0.1, 0.1, 0.9, 0.85, 0.07),
            (0.5, 0.5, 0.5, 0.5, 0.13),
        ];

        for (u0, v0, u1, v1, radio) in trazos {
            mascara.stroke(u0, v0, u1, v1, radio, 1.0);
            pigmentos.stroke(CLAVE_A, u0, v0, u1, v1, radio, ROJO, 1.0);
        }

        for y in 0..LADO {
            for x in 0..LADO {
                let u = (x as f32 + 0.5) / LADO as f32;
                let v = (y as f32 + 0.5) / LADO as f32;

                let cubierta = mascara.sample(u, v) > 0.0;
                let pintada = pigmentos.sample(CLAVE_A, u, v).is_some();

                assert_eq!(cubierta, pintada, "la celda ({x}, {y}) difiere");
            }
        }

        assert!(
            mascara.sample(0.5, 0.5) > 0.0,
            "el guion tiene que cubrir algo"
        );
    }

    // ---------------------------------------------- pigmento de textura

    /// Textura `2 x 2` con cuatro colores bien distintos.
    ///
    /// Los pixeles llegan en orden de lectura de imagen —fila 0 arriba—,
    /// que es lo que documenta `Texture::from_pixels`. Quien decide como se
    /// leen despues es `Texture::sample`, y estos tests no lo reimplementan:
    /// comparan contra el.
    fn tela() -> Texture {
        Texture::from_pixels(
            2,
            2,
            vec![
                Color::new(0.9, 0.1, 0.1),
                Color::new(0.1, 0.9, 0.1),
                Color::new(0.1, 0.1, 0.9),
                Color::new(0.9, 0.9, 0.1),
            ],
        )
        .expect("la tela de prueba es valida")
    }

    /// Centro `uv` de una celda de una rejilla de `lado x lado`.
    fn centro(lado: usize, x: usize, y: usize) -> (f32, f32) {
        (
            (x as f32 + 0.5) / lado as f32,
            (y as f32 + 0.5) / lado as f32,
        )
    }

    #[test]
    fn una_textura_se_estampa_celda_a_celda_con_v_cero_abajo() {
        // Cada celda toma el color que la textura da en **su** centro. Si
        // la orientacion se invirtiera, los dos cuadrantes de arriba
        // cambiarian con los de abajo y este test lo veria.
        let tela = tela();
        let mut pigmentos = PigmentMasks::new(2, 2);

        pigmentos.stamp_texture(CLAVE_A, 0.5, 0.5, 2.0, &tela, 1.0, 1.0);

        let mut vistos = Vec::new();

        for y in 0..2 {
            for x in 0..2 {
                let (u, v) = centro(2, x, y);
                let celda = pigmentos.sample(CLAVE_A, u, v).expect("celda pintada");

                assert_eq!(
                    celda.color,
                    tela.sample(u, v),
                    "la celda ({x}, {y}) no tomo el color de su centro"
                );
                assert_eq!(celda.coverage, 1.0);
                vistos.push(celda.color);
            }
        }

        // Los cuatro distintos: ni se colapsan ni se repiten por una
        // orientacion degenerada.
        for i in 0..vistos.len() {
            for j in (i + 1)..vistos.len() {
                assert_ne!(vistos[i], vistos[j], "las celdas {i} y {j} coinciden");
            }
        }

        // Y `v = 0` es abajo: la fila `y = 0` es la que la textura sirve
        // para `v` pequena.
        assert_eq!(vistos[0], tela.sample(0.25, 0.25));
        assert_ne!(vistos[0], tela.sample(0.25, 0.75));
    }

    #[test]
    fn una_escala_mayor_que_uno_repite_segun_la_textura() {
        // La repeticion la decide `WrapMode` dentro de `Texture`; aqui solo
        // se comprueba que la escala llega y que no se reimplemento nada.
        let tela = tela();
        let mut pigmentos = PigmentMasks::new(4, 4);

        pigmentos.stamp_texture(CLAVE_A, 0.5, 0.5, 2.0, &tela, 2.0, 1.0);

        for y in 0..4 {
            for x in 0..4 {
                let (u, v) = centro(4, x, y);
                let celda = pigmentos.sample(CLAVE_A, u, v).expect("celda pintada");

                assert_eq!(celda.color, tela.sample(u * 2.0, v * 2.0), "({x}, {y})");
            }
        }

        // Con `Repeat` y escala dos, la primera celda y la tercera caen en
        // el mismo punto de la tela.
        let primera = pigmentos.sample(CLAVE_A, centro(4, 0, 0).0, centro(4, 0, 0).1);
        let tercera = pigmentos.sample(CLAVE_A, centro(4, 2, 0).0, centro(4, 2, 0).1);

        assert_eq!(primera, tercera, "la escala tiene que repetir la tela");
    }

    #[test]
    fn una_textura_clamp_se_recorta_en_vez_de_repetir() {
        let repetida = tela();
        let recortada = tela().with_wrap(WrapMode::Clamp);

        let mut con_repeat = PigmentMasks::new(4, 4);
        let mut con_clamp = PigmentMasks::new(4, 4);

        con_repeat.stamp_texture(CLAVE_A, 0.5, 0.5, 2.0, &repetida, 2.0, 1.0);
        con_clamp.stamp_texture(CLAVE_A, 0.5, 0.5, 2.0, &recortada, 2.0, 1.0);

        for y in 0..4 {
            for x in 0..4 {
                let (u, v) = centro(4, x, y);

                assert_eq!(
                    con_clamp
                        .sample(CLAVE_A, u, v)
                        .expect("celda pintada")
                        .color,
                    recortada.sample(u * 2.0, v * 2.0),
                    "({x}, {y}) no respeta el clamp de la textura"
                );
            }
        }

        // Y las dos politicas dan resultados distintos en alguna celda, o
        // el bucle de arriba no estaria probando nada. No se fija cual: con
        // una tela de cuatro colores hay celdas donde `repeat` y `clamp`
        // caen por casualidad en el mismo pixel.
        let discrepan = (0..4).any(|y| {
            (0..4).any(|x| {
                let (u, v) = centro(4, x, y);

                con_repeat.sample(CLAVE_A, u, v) != con_clamp.sample(CLAVE_A, u, v)
            })
        });

        assert!(
            discrepan,
            "repeat y clamp tendrian que diferir en alguna celda"
        );
    }

    #[test]
    fn dos_pasadas_de_textura_se_componen_como_el_pigmento_plano() {
        let tela = tela();
        let mut pigmentos = PigmentMasks::new(2, 2);
        let (u, v) = centro(2, 0, 0);

        pigmentos.stamp(CLAVE_A, u, v, 0.1, AZUL, 1.0);
        pigmentos.stamp_texture(CLAVE_A, u, v, 0.1, &tela, 1.0, 0.5);

        let celda = pigmentos.sample(CLAVE_A, u, v).expect("celda pintada");
        let encima = tela.sample(u, v);
        let esperado = encima * 0.5 + AZUL * 0.5;

        assert!(
            cerca(celda.color, esperado),
            "dio {:?} y se esperaba {esperado:?}",
            celda.color
        );
        assert_eq!(celda.coverage, 1.0);
    }

    #[test]
    fn el_trazo_de_textura_deja_la_misma_huella_que_la_mascara() {
        let tela = tela();
        let mut mascara = BrushMask::new(LADO, LADO);
        let mut pigmentos = PigmentMasks::new(LADO, LADO);

        for (u0, v0, u1, v1, radio) in [
            (0.2f32, 0.5f32, 0.8f32, 0.5f32, 0.05f32),
            (0.1, 0.15, 0.85, 0.9, 0.06),
        ] {
            mascara.stroke(u0, v0, u1, v1, radio, 1.0);
            pigmentos.stroke_texture(CLAVE_A, u0, v0, u1, v1, radio, &tela, 3.0, 1.0);
        }

        for y in 0..LADO {
            for x in 0..LADO {
                let (u, v) = centro(LADO, x, y);

                assert_eq!(
                    mascara.sample(u, v) > 0.0,
                    pigmentos.sample(CLAVE_A, u, v).is_some(),
                    "la celda ({x}, {y}) difiere"
                );
            }
        }

        // Continuo sobre el segmento, no una ristra de sellos.
        for paso in 0..=100 {
            let t = paso as f32 / 100.0;
            let u = 0.2 + 0.6 * t;

            assert!(
                pigmentos.sample(CLAVE_A, u, 0.5).is_some(),
                "hueco en t = {t}"
            );
        }
    }

    #[test]
    fn el_trazo_de_textura_no_mancha_otras_claves() {
        let tela = tela();
        let mut pigmentos = pigmentos();

        pigmentos.stroke_texture(CLAVE_A, 0.2, 0.5, 0.8, 0.5, 0.1, &tela, 1.0, 1.0);

        assert!(pigmentos.sample(CLAVE_A, 0.5, 0.5).is_some());
        assert_eq!(pigmentos.sample(OTRA_CARTA, 0.5, 0.5), None);
        assert_eq!(pigmentos.sample(OTRO_OBJETO, 0.5, 0.5), None);
        assert_eq!(pigmentos.len(), 1);
    }

    #[test]
    fn una_textura_con_entrada_invalida_no_crea_capa() {
        let tela = tela();
        let mut pigmentos = pigmentos();

        // Escala no finita, nula o negativa.
        for escala in [0.0f32, -1.0, f32::NAN, f32::INFINITY] {
            pigmentos.stamp_texture(CLAVE_A, 0.5, 0.5, 0.1, &tela, escala, 1.0);
        }

        // Alfa invalido.
        for alfa in [0.0f32, -0.5, f32::NAN] {
            pigmentos.stamp_texture(CLAVE_A, 0.5, 0.5, 0.1, &tela, 1.0, alfa);
        }

        // `uv` fuera de la superficie y radio imposible.
        pigmentos.stamp_texture(CLAVE_A, 1.5, 0.5, 0.1, &tela, 1.0, 1.0);
        pigmentos.stamp_texture(CLAVE_A, f32::NAN, 0.5, 0.1, &tela, 1.0, 1.0);
        pigmentos.stamp_texture(CLAVE_A, 0.5, 0.5, 0.0, &tela, 1.0, 1.0);
        pigmentos.stroke_texture(CLAVE_A, 0.2, 0.5, 1.4, 0.5, 0.1, &tela, 1.0, 1.0);

        assert!(pigmentos.is_empty(), "ninguna entrada invalida abre capa");
        assert_eq!(pigmentos.sample(CLAVE_A, 0.5, 0.5), None);
    }

    // ------------------------------ elipse en los contenedores

    /// Los centros de celda de una rejilla `FINO x FINO`.
    fn centros() -> impl Iterator<Item = (usize, usize, f32, f32)> {
        (0..FINO).flat_map(|y| {
            (0..FINO).map(move |x| {
                let (u, v) = centro(FINO, x, y);

                (x, y, u, v)
            })
        })
    }

    #[test]
    fn el_revelado_circular_equivale_al_eliptico_de_radios_iguales() {
        let mut circular = BrushMasks::new(FINO, FINO);
        let mut eliptica = BrushMasks::new(FINO, FINO);

        circular.stroke(CLAVE_A, 0.2, 0.4, 0.75, 0.7, 0.06, 0.6);
        eliptica.stroke_ellipse(CLAVE_A, 0.2, 0.4, 0.75, 0.7, 0.06, 0.06, 0.6);

        circular.stamp(CLAVE_A, 0.3, 0.8, 0.05, 0.5);
        eliptica.stamp_ellipse(CLAVE_A, 0.3, 0.8, 0.05, 0.05, 0.5);

        for (x, y, u, v) in centros() {
            assert_eq!(
                circular.sample(CLAVE_A, u, v),
                eliptica.sample(CLAVE_A, u, v),
                "difieren en ({x}, {y})"
            );
        }

        assert_eq!(circular.len(), 1);
        assert_eq!(eliptica.len(), 1);
    }

    #[test]
    fn el_pigmento_plano_circular_equivale_al_eliptico_de_radios_iguales() {
        let mut circular = PigmentMasks::new(FINO, FINO);
        let mut eliptica = PigmentMasks::new(FINO, FINO);

        circular.stroke(CLAVE_A, 0.2, 0.4, 0.75, 0.7, 0.06, ROJO, 0.6);
        eliptica.stroke_ellipse(CLAVE_A, 0.2, 0.4, 0.75, 0.7, 0.06, 0.06, ROJO, 0.6);

        circular.stamp(CLAVE_A, 0.3, 0.8, 0.05, AZUL, 1.0);
        eliptica.stamp_ellipse(CLAVE_A, 0.3, 0.8, 0.05, 0.05, AZUL, 1.0);

        for (x, y, u, v) in centros() {
            assert_eq!(
                circular.sample(CLAVE_A, u, v),
                eliptica.sample(CLAVE_A, u, v),
                "difieren en ({x}, {y})"
            );
        }
    }

    #[test]
    fn la_textura_circular_equivale_a_la_eliptica_de_radios_iguales() {
        let tela = tela();
        let mut circular = PigmentMasks::new(FINO, FINO);
        let mut eliptica = PigmentMasks::new(FINO, FINO);

        circular.stroke_texture(CLAVE_A, 0.2, 0.4, 0.75, 0.7, 0.06, &tela, 2.0, 0.6);
        eliptica.stroke_texture_ellipse(CLAVE_A, 0.2, 0.4, 0.75, 0.7, 0.06, 0.06, &tela, 2.0, 0.6);

        circular.stamp_texture(CLAVE_A, 0.3, 0.8, 0.05, &tela, 2.0, 1.0);
        eliptica.stamp_texture_ellipse(CLAVE_A, 0.3, 0.8, 0.05, 0.05, &tela, 2.0, 1.0);

        for (x, y, u, v) in centros() {
            assert_eq!(
                circular.sample(CLAVE_A, u, v),
                eliptica.sample(CLAVE_A, u, v),
                "difieren en ({x}, {y})"
            );
        }
    }

    #[test]
    fn los_semiejes_estiran_cada_capa_en_su_eje() {
        let tela = tela();
        let mut revelado = BrushMasks::new(FINO, FINO);
        let mut plano = PigmentMasks::new(FINO, FINO);
        let mut textura = PigmentMasks::new(FINO, FINO);

        revelado.stamp_ellipse(CLAVE_A, 0.5, 0.5, 0.20, 0.05, 1.0);
        plano.stamp_ellipse(CLAVE_A, 0.5, 0.5, 0.20, 0.05, ROJO, 1.0);
        textura.stamp_texture_ellipse(CLAVE_A, 0.5, 0.5, 0.20, 0.05, &tela, 1.0, 1.0);

        // Ancho en `u`: dentro por el eje largo, fuera por el corto.
        assert!(revelado.sample(CLAVE_A, 0.65, 0.5) > 0.0);
        assert_eq!(revelado.sample(CLAVE_A, 0.5, 0.65), 0.0);

        assert!(plano.sample(CLAVE_A, 0.65, 0.5).is_some());
        assert_eq!(plano.sample(CLAVE_A, 0.5, 0.65), None);

        assert!(textura.sample(CLAVE_A, 0.65, 0.5).is_some());
        assert_eq!(textura.sample(CLAVE_A, 0.5, 0.65), None);

        // Y el simetrico: alto en `v`.
        let mut alto = BrushMasks::new(FINO, FINO);
        alto.stamp_ellipse(CLAVE_A, 0.5, 0.5, 0.05, 0.20, 1.0);

        assert!(alto.sample(CLAVE_A, 0.5, 0.65) > 0.0);
        assert_eq!(alto.sample(CLAVE_A, 0.65, 0.5), 0.0);
    }

    #[test]
    fn el_trazo_eliptico_de_cada_capa_es_continuo() {
        let tela = tela();
        let mut revelado = BrushMasks::new(FINO, FINO);
        let mut plano = PigmentMasks::new(FINO, FINO);
        let mut textura = PigmentMasks::new(FINO, FINO);
        let (u0, u1, v) = (0.2f32, 0.8f32, 0.5f32);

        revelado.stroke_ellipse(CLAVE_A, u0, v, u1, v, 0.03, 0.10, 1.0);
        plano.stroke_ellipse(CLAVE_A, u0, v, u1, v, 0.03, 0.10, ROJO, 1.0);
        textura.stroke_texture_ellipse(CLAVE_A, u0, v, u1, v, 0.03, 0.10, &tela, 1.0, 1.0);

        for paso in 0..=100 {
            let t = paso as f32 / 100.0;
            let u = u0 + (u1 - u0) * t;

            assert_eq!(revelado.sample(CLAVE_A, u, v), 1.0, "revelado, t = {t}");
            assert!(plano.sample(CLAVE_A, u, v).is_some(), "plano, t = {t}");
            assert!(textura.sample(CLAVE_A, u, v).is_some(), "textura, t = {t}");
        }
    }

    #[test]
    fn las_elipses_respetan_las_claves() {
        let tela = tela();
        let mut revelado = BrushMasks::new(FINO, FINO);
        let mut plano = PigmentMasks::new(FINO, FINO);
        let mut textura = PigmentMasks::new(FINO, FINO);

        revelado.stamp_ellipse(CLAVE_A, 0.5, 0.5, 0.2, 0.1, 1.0);
        plano.stamp_ellipse(CLAVE_A, 0.5, 0.5, 0.2, 0.1, ROJO, 1.0);
        textura.stamp_texture_ellipse(CLAVE_A, 0.5, 0.5, 0.2, 0.1, &tela, 1.0, 1.0);

        for clave in [OTRA_CARTA, OTRO_OBJETO] {
            assert_eq!(revelado.sample(clave, 0.5, 0.5), 0.0, "{clave:?}");
            assert_eq!(plano.sample(clave, 0.5, 0.5), None, "{clave:?}");
            assert_eq!(textura.sample(clave, 0.5, 0.5), None, "{clave:?}");
        }

        assert_eq!(revelado.len(), 1);
        assert_eq!(plano.len(), 1);
        assert_eq!(textura.len(), 1);
    }

    #[test]
    fn una_elipse_invalida_no_abre_capa_en_ningun_contenedor() {
        let tela = tela();
        let mut revelado = BrushMasks::new(FINO, FINO);
        let mut plano = PigmentMasks::new(FINO, FINO);
        let mut textura = PigmentMasks::new(FINO, FINO);

        for raro in [0.0f32, -0.3, f32::NAN, f32::INFINITY] {
            // Radio `u` malo, radio `v` malo, y los dos bien con alfa mala.
            revelado.stamp_ellipse(CLAVE_A, 0.5, 0.5, raro, 0.1, 1.0);
            revelado.stamp_ellipse(CLAVE_A, 0.5, 0.5, 0.1, raro, 1.0);
            revelado.stroke_ellipse(CLAVE_A, 0.2, 0.5, 0.8, 0.5, 0.1, raro, 1.0);

            plano.stamp_ellipse(CLAVE_A, 0.5, 0.5, raro, 0.1, ROJO, 1.0);
            plano.stamp_ellipse(CLAVE_A, 0.5, 0.5, 0.1, raro, ROJO, 1.0);
            plano.stroke_ellipse(CLAVE_A, 0.2, 0.5, 0.8, 0.5, 0.1, raro, ROJO, 1.0);

            textura.stamp_texture_ellipse(CLAVE_A, 0.5, 0.5, raro, 0.1, &tela, 1.0, 1.0);
            textura.stamp_texture_ellipse(CLAVE_A, 0.5, 0.5, 0.1, raro, &tela, 1.0, 1.0);
            // Y la escala, que solo la textura tiene.
            textura.stamp_texture_ellipse(CLAVE_A, 0.5, 0.5, 0.1, 0.1, &tela, raro, 1.0);
        }

        // `uv` fuera de rango con radios validos.
        revelado.stamp_ellipse(CLAVE_A, 1.5, 0.5, 0.1, 0.1, 1.0);
        plano.stamp_ellipse(CLAVE_A, 1.5, 0.5, 0.1, 0.1, ROJO, 1.0);
        textura.stamp_texture_ellipse(CLAVE_A, 1.5, 0.5, 0.1, 0.1, &tela, 1.0, 1.0);

        assert!(revelado.is_empty(), "el revelado abrio capa");
        assert!(plano.is_empty(), "el pigmento plano abrio capa");
        assert!(textura.is_empty(), "la textura abrio capa");
    }

    #[test]
    fn una_mascara_no_cuadrada_admite_toda_su_superficie() {
        // El disco se mide en `uv`, así que en una máscara alargada es una
        // elipse en celdas. Lo que no puede pasar es que una fila o columna
        // quede inalcanzable por un mapeo mal escalado.
        let mut m = BrushMask::new(16, 4);

        m.stamp(0.5, 0.5, 2.0, 1.0);

        for y in 0..m.height() {
            for x in 0..m.width() {
                assert_eq!(m.pixel(x, y), 1.0, "({x}, {y}) quedo sin alcanzar");
            }
        }
    }
}
