//! Entrada del usuario: de una posición de cursor a un rayo de mundo.
//!
//! # Por qué existe aparte de `camera`
//!
//! La cámara resuelve **geometría** y no toma decisiones: extrapolar más
//! allá del borde del cuadro es una dirección perfectamente definida, y
//! `Camera::ray_from_cursor` la devuelve sin objetar. Que un clic afuera de
//! la ventana no deba contar es una decisión de **política de entrada**, y
//! vive aquí.
//!
//! # Por qué no aparece `minifb`
//!
//! Este módulo trabaja con `(f32, f32)` crudos, no con tipos de la librería
//! de ventana. La razón es la misma que sostiene toda la librería: lo que
//! está en `lib.rs` tiene que poder probarse sin abrir una ventana. El
//! binario lee el cursor de `minifb` y pasa el par de números; el picking se
//! prueba con `cargo test`.
//!
//! # El cuadro presentado
//!
//! Un clic se resuelve contra `PresentedFrame`: la cámara **y la resolución
//! fuente** con las que se trazó lo que está en pantalla.
//!
//! Una versión anterior de este módulo afirmaba que el escalado por vecino
//! más cercano preserva las coordenadas normalizadas y que por eso daba lo
//! mismo usar el tamaño de la ventana. **Es falso.** El escalado trunca:
//! a `800 × 600` sobre un perfil de `400 × 300`, los píxeles de ventana `2k`
//! y `2k + 1` muestran los dos el píxel fuente `k`, y el centro de ese
//! píxel fuente está a media unidad de fuente —un píxel de ventana— de la
//! coordenada continua del cursor.
//!
//! La consecuencia es que un clic podía elegir lo que había un píxel al
//! lado de lo que el usuario veía. Se resuelve pasando por el mismo mapeo
//! que dibuja, `framebuffer::source_pixel`, y trazando el rayo del **píxel
//! fuente**: el mismo que el renderer trazó para ese punto de la imagen.

use crate::accel::{SceneAccel, TraversalStats};
use crate::camera::Camera;
use crate::framebuffer::source_pixel;
use crate::hit::UvChart;
use crate::primitive::UvWorldScale;
use crate::ray::Ray;
use crate::scene::{RevealGroup, Scene};
use nalgebra_glm::{Vec2, Vec3};

/// La imagen que el usuario está mirando: con qué cámara y a qué resolución
/// se trazó, y en qué ventana se presentó.
///
/// Existe porque las tres cosas pueden diferir de las actuales:
///
/// - La **cámara** avanza antes de que se lea el clic. Las teclas de órbita
///   se procesan primero en el mismo cuadro, así que sosteniendo una flecha
///   mientras se pincha, el rayo apuntaría a una escena que todavía no se ha
///   mostrado.
/// - La **resolución fuente** es la del perfil interactivo mientras algo se
///   mueve, y la de la ventana en reposo. El clic tiene que resolverse
///   contra la que produjo la imagen.
#[derive(Debug, Clone, Copy)]
pub struct PresentedFrame {
    /// Cámara con la que se trazó el cuadro.
    pub camera: Camera,
    /// Resolución a la que se **trazó**: el perfil interactivo o la ventana.
    pub source: (usize, usize),
    /// Resolución de la ventana en la que se presentó.
    pub window: (usize, usize),
}

impl PresentedFrame {
    /// Un cuadro trazado a la resolución de la ventana, sin escalado.
    pub fn full(camera: Camera, window: (usize, usize)) -> Self {
        PresentedFrame {
            camera,
            source: window,
            window,
        }
    }

    /// Píxel **fuente** que se muestra bajo ese cursor, o `None` si el
    /// cursor no señala el cuadro.
    ///
    /// Se rechazan dos cosas, y ninguna es paranoia:
    ///
    /// - **Fuera de la ventana.** `minifb` entrega la última posición
    ///   conocida del puntero incluso cuando salió del área de dibujo, así
    ///   que un clic registrado ahí apuntaría a geometría que el usuario no
    ///   está viendo.
    /// - **No finito.** Un `NaN` produciría una dirección `NaN`, y un rayo
    ///   así no falla: recorre la escena, no impacta nada y devuelve cielo.
    ///   Un picking que «no encuentra nada» es indistinguible de un clic en
    ///   el vacío, y ese es justo el error que no se quiere depurar mirando
    ///   la pantalla.
    ///
    /// El borde derecho e inferior quedan **excluidos**: un cursor en
    /// `x = 800` de una ventana de `800` está una columna más allá del
    /// último píxel, igual que el índice `800` de un arreglo de `800`.
    pub fn source_pixel_at(&self, cursor: (f32, f32)) -> Option<(usize, usize)> {
        if !cursor.0.is_finite() || !cursor.1.is_finite() {
            return None;
        }

        if cursor.0 < 0.0 || cursor.1 < 0.0 {
            return None;
        }

        let (ancho, alto) = self.window;

        if cursor.0 >= ancho as f32 || cursor.1 >= alto as f32 {
            return None;
        }

        // El mismo mapeo que dibuja. Ver `framebuffer::source_pixel`.
        Some((
            source_pixel(cursor.0 as usize, ancho, self.source.0),
            source_pixel(cursor.1 as usize, alto, self.source.1),
        ))
    }

    /// El rayo que el renderer trazó para el píxel bajo ese cursor.
    ///
    /// No es el rayo de la posición continua del cursor: es el del **píxel
    /// fuente**, que es la unidad que el usuario ve como un solo punto de
    /// color. Con el perfil interactivo activo, los cuatro píxeles de
    /// ventana de un bloque `2 × 2` devuelven el mismo rayo, porque muestran
    /// el mismo píxel.
    pub fn ray_under_cursor(&self, cursor: (f32, f32)) -> Option<Ray> {
        let (x, y) = self.source_pixel_at(cursor)?;

        Some(
            self.camera
                .ray_from_pixel(x, y, self.source.0, self.source.1),
        )
    }
}

/// Región que selecciona un clic, o `None` si no selecciona ninguna.
///
/// El impacto devuelve un **grupo de revelación** y nada más fino. No hay
/// pintado por vóxel ni por cara: el clic elige una de las tres regiones y
/// la región entera se revela. Esa granularidad no es una simplificación
/// perezosa, es la que sostiene toda la arquitectura del proyecto —un
/// escalar por grupo, la geometría inmutable, la jerarquía de aceleración
/// construida una sola vez—.
///
/// Devuelve `None` en tres casos, todos legítimos:
///
/// - El cursor no señala el cuadro. Lo resuelve
///   `PresentedFrame::source_pixel_at`.
/// - El rayo no toca nada: un clic en el cielo.
/// - Lo que toca no se pinta. El plinto ocupa toda la base del diorama y
///   comparte grupo con el Monolito por tipado; sin este filtro, pincharlo
///   activaría el finale. Ver `Scene::paintable_group`.
pub fn pick_region(
    scene: &Scene,
    accel: &SceneAccel,
    frame: &PresentedFrame,
    cursor: (f32, f32),
) -> Option<RevealGroup> {
    pick_paint(scene, accel, frame, cursor).map(|objetivo| objetivo.reveal_group)
}

/// Superficie que un clic elige, con el detalle que necesita una brocha.
///
/// `pick_region` contesta **qué región** se pinta; esto contesta además
/// **qué objeto y en qué punto de su superficie**, que es lo que hace falta
/// para estampar un trazo en vez de revelar la región entera.
///
/// Es un dato, no un identificador persistente: el `object_index` vale
/// mientras la escena no cambie, y la escena de este proyecto se construye
/// una sola vez.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PaintHit {
    /// Objeto elegido, dentro de `Scene::objects`.
    pub object_index: usize,
    /// Punto de su superficie, en las `uv` de la cara impactada.
    pub uv: Vec2,
    /// El punto del impacto, en coordenadas de mundo.
    ///
    /// Viene tal cual del `Hit`. Lo necesita cualquier cosa que quiera
    /// dibujarse **sobre** la superficie —un gizmo de pincel, por ejemplo—,
    /// que tiene que saber dónde está antes de proyectarlo a la pantalla.
    pub point: Vec3,
    /// Normal de la superficie en ese punto, ya orientada contra el rayo.
    ///
    /// También viene tal cual del `Hit`, con el lado ya resuelto por
    /// `Hit::new`. No se reclasifica aquí: deducir la cara a partir de la
    /// normal sería reconstruir a ojo algo que la primitiva ya sabía.
    pub normal: Vec3,
    /// Sobre qué carta se midió esa `uv`.
    ///
    /// Sin esto, `uv` es ambigua dentro del propio objeto: las caras `+X` y
    /// `-X` de un cuboide recorren las mismas coordenadas. El par
    /// `(object_index, uv_chart)` es lo que identifica un trozo pintable;
    /// `uv` dice dónde dentro de él. Ver `UvChart`.
    pub uv_chart: UvChart,
    /// Cuánto mundo recorre cada coordenada `uv` de esa carta.
    ///
    /// La entrega la primitiva, que es la única que lo sabe con exactitud.
    /// Sirve para que un pincel pueda convertir un radio en unidades de
    /// mundo a semiejes `uv` y dibujar un trazo redondo sobre una cara
    /// alargada. Ver `primitive::UvWorldScale`.
    pub uv_world_scale: UvWorldScale,
    /// Región a la que pertenece.
    ///
    /// Puede ser `Finale`: `pick_artistic` acepta el Monolito y el plinto,
    /// que pertenecen a ese grupo. Lo que **no** puede es venir de
    /// `pick_paint`, que aplica la política de entrega y los excluye. El
    /// tipo no distingue de qué picker salió, así que quien use este campo
    /// para revelar tiene que haberlo pedido por el picker correcto.
    pub reveal_group: RevealGroup,
}

/// Superficie de pintura bajo el cursor, o `None` si no hay ninguna
/// elegible.
///
/// La política de elegibilidad es **la misma** que la de `pick_region`, y lo
/// es porque esta función es su única implementación: cursor válido, el rayo
/// tiene que impactar algo, y lo impactado tiene que pintarse —ni inerte ni
/// del finale, ver `Scene::paintable_group`—.
///
/// Que las dos respuestas salgan del mismo sitio no es limpieza: si
/// divergieran, un clic revelaría una región y la brocha pintaría sobre
/// otra, y el síntoma aparecería como un fallo de pintado y no de picking.
pub fn pick_paint(
    scene: &Scene,
    accel: &SceneAccel,
    frame: &PresentedFrame,
    cursor: (f32, f32),
) -> Option<PaintHit> {
    pick_con(scene, accel, frame, cursor, Scene::paintable_group)
}

/// Superficie bajo el cursor según la política **artística**, o `None`.
///
/// Acepta cualquier superficie del diorama y rechaza solo la paleta y el
/// pincel físicos. Ver `Scene::artistic_group` para el porqué de cada parte.
///
/// Comparte con `pick_paint` todo lo demás —el cursor, el rayo, el impacto,
/// la carta `uv`— y se diferencia únicamente en la política. Es lo que
/// permite afirmar que donde las dos aceptan entregan lo mismo, y que
/// ampliar el objetivo no cambia dónde cae un trazo.
pub fn pick_artistic(
    scene: &Scene,
    accel: &SceneAccel,
    frame: &PresentedFrame,
    cursor: (f32, f32),
) -> Option<PaintHit> {
    pick_con(scene, accel, frame, cursor, Scene::artistic_group)
}

/// El picking, con la política como parámetro.
///
/// Las dos públicas son esta con una política distinta, y no dos cuerpos
/// parecidos: si divergieran, un trazo caería en un punto con una y en otro
/// con la otra, y el síntoma sería pintura desplazada al cambiar de modo.
fn pick_con(
    scene: &Scene,
    accel: &SceneAccel,
    frame: &PresentedFrame,
    cursor: (f32, f32),
    politica: impl Fn(&Scene, usize) -> Option<RevealGroup>,
) -> Option<PaintHit> {
    let ray = frame.ray_under_cursor(cursor)?;

    // Un rayo por clic: los contadores de recorrido no se llevan a ninguna
    // parte, así que se descartan aquí en vez de obligar al llamador a
    // cargar un acumulador que no va a leer.
    let hit = accel.intersect(scene, &ray, &mut TraversalStats::default())?;
    let reveal_group = politica(scene, hit.object_index)?;

    // Sin métrica no hay superficie pintable utilizable: un pincel no
    // podría dimensionarse sobre ella. Solo ocurre con geometría degenerada
    // o con una carta que la primitiva no reconozca, y en los dos casos
    // devolver `None` es más honesto que entregar un objetivo a medias.
    let uv_world_scale = scene
        .objects
        .get(hit.object_index)?
        .primitive
        .uv_world_scale(hit.uv_chart)?;

    Some(PaintHit {
        object_index: hit.object_index,
        uv: hit.uv,
        point: hit.point,
        normal: hit.normal,
        uv_chart: hit.uv_chart,
        uv_world_scale,
        reveal_group,
    })
}

/// Las tres regiones del fallback de teclado, en el orden del plan.
///
/// El ratón es la interacción principal. Esto existe porque una
/// presentación no puede depender de acertar un clic sobre una bahía que
/// ocupa el `2.4 %` del cuadro: con las teclas `1`, `2` y `3` la
/// demostración es reproducible aunque el puntero falle.
pub const DEMO_REGIONS: [RevealGroup; 3] = [
    RevealGroup::Meadows,
    RevealGroup::Breakwater,
    RevealGroup::FlyingWaters,
];

/// Región del fallback asociada a un dígito, o `None` si ese dígito no
/// corresponde a ninguna.
///
/// `Finale` queda fuera a propósito y no por olvido: el Monolito no es una
/// región que se elija, es la consecuencia de haber pintado las tres. Ver
/// `RevealState::all_regions_painted`.
pub fn demo_region(digit: u8) -> Option<RevealGroup> {
    DEMO_REGIONS.get(digit.checked_sub(1)? as usize).copied()
}

/// Lo que el teclado de presentación puede pedir.
///
/// Las tres acciones se declaran juntas porque juntas son la garantía de
/// que la demo se puede dar sin ratón: pintar cada región, volver al lienzo
/// para repetirla, y recuperar el encuadre si la órbita se fue a un ángulo
/// desde el que no se ve nada.
///
/// `ResetCamera` **no lleva el encuadre**. Se limita a decir «restaurá», y
/// el encuadre lo aporta la escena: `Blockout::hero_preset`. El plan lo
/// exige así para que los tres puntos del blueprint no queden clavados en
/// el módulo de entrada, donde se desincronizarían del blockout en el
/// primer ajuste de composición.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DemoAction {
    /// Empezar a pintar una región.
    Paint(RevealGroup),
    /// Volver al lienzo para repetir la demostración.
    ResetCanvas,
    /// Recuperar el encuadre hero de la escena.
    ResetCamera,
}

/// Acción asociada a una tecla de la demo, o `None` si esa tecla no hace
/// nada.
///
/// Se recibe el carácter y no un tipo de `minifb`, por la misma razón que
/// el resto del módulo: la superficie de teclado se prueba sin abrir una
/// ventana.
pub fn demo_action(key: char) -> Option<DemoAction> {
    match key {
        '1' | '2' | '3' => {
            let digito = key as u8 - b'0';

            demo_region(digito).map(DemoAction::Paint)
        }
        'l' | 'L' => Some(DemoAction::ResetCanvas),
        'r' | 'R' => Some(DemoAction::ResetCamera),
        _ => None,
    }
}

/// Lo que un cuadro de entrada le pide al estado, ya resuelto.
///
/// El ciclo de la ventana puede recibir varias acciones en el **mismo**
/// cuadro: un clic y una tecla, o `1`, `2` y `3` en el mismo sondeo. Con
/// una sola variable `elegida` el último ganaba y los otros se perdían en
/// silencio, y un `ResetCanvas` no impedía que una selección del mismo
/// cuadro se aplicara **después** sobre el lienzo recién reiniciado: la
/// consola decía «reiniciado» y el estado tenía una región revelándose.
///
/// Reducir primero y aplicar después arregla las dos cosas, y deja la
/// política probable sin abrir una ventana.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FrameIntent {
    /// Grupos a activar, sin repetir. Cuatro caben siempre: son todos los
    /// que existen.
    paint: [Option<RevealGroup>; RevealGroup::COUNT],
    /// Volver al lienzo. **Domina** sobre `paint` del mismo cuadro.
    pub reset_canvas: bool,
    /// Restaurar el encuadre hero. Convive con todo lo demás: mover la
    /// cámara y pintar son cosas independientes.
    pub reset_camera: bool,
}

impl FrameIntent {
    /// Reduce las acciones de un cuadro a una intención.
    pub fn from_actions(actions: impl IntoIterator<Item = DemoAction>) -> Self {
        let mut intent = FrameIntent::default();

        for action in actions {
            match action {
                DemoAction::Paint(grupo) => intent.add_paint(grupo),
                DemoAction::ResetCanvas => intent.reset_canvas = true,
                DemoAction::ResetCamera => intent.reset_camera = true,
            }
        }

        intent
    }

    /// Añade un grupo a pintar, sin duplicarlo.
    pub fn add_paint(&mut self, group: RevealGroup) {
        if self.paint.contains(&Some(group)) {
            return;
        }

        if let Some(hueco) = self.paint.iter_mut().find(|g| g.is_none()) {
            *hueco = Some(group);
        }
    }

    /// Los grupos a pintar **después** de aplicar la precedencia.
    ///
    /// Con `reset_canvas` la lista queda vacía: volver al lienzo y pintar en
    /// el mismo cuadro es contradictorio, y de las dos lecturas la que
    /// respeta lo que el usuario acaba de pedir es reiniciar. La otra deja
    /// el estado y la consola diciendo cosas distintas.
    pub fn paints(&self) -> impl Iterator<Item = RevealGroup> + '_ {
        let vacio = self.reset_canvas;

        self.paint.iter().flatten().copied().filter(move |_| !vacio)
    }

    /// ¿Este cuadro no pide nada?
    pub fn is_empty(&self) -> bool {
        !self.reset_canvas && !self.reset_camera && self.paint.iter().all(Option::is_none)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera::DEFAULT_VERTICAL_FOV;
    use nalgebra_glm::Vec3;

    const ANCHO: usize = 800;
    const ALTO: usize = 600;

    fn camara() -> Camera {
        Camera::new(
            Vec3::new(0.0, 6.0, 12.0),
            Vec3::zeros(),
            Vec3::new(0.0, 1.8, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            DEFAULT_VERTICAL_FOV,
        )
    }

    /// Cuadro presentado a resolución completa.
    fn completo() -> PresentedFrame {
        PresentedFrame::full(camara(), (ANCHO, ALTO))
    }

    /// Cuadro presentado desde el perfil interactivo: se trazó a la mitad de
    /// lado y se escaló al doble.
    fn escalado() -> PresentedFrame {
        PresentedFrame {
            camera: camara(),
            source: (ANCHO / 2, ALTO / 2),
            window: (ANCHO, ALTO),
        }
    }

    #[test]
    fn un_cursor_dentro_del_cuadro_da_rayo() {
        let cuadro = completo();

        for cursor in [(0.0, 0.0), (400.0, 300.0), (799.9, 599.9)] {
            let rayo = cuadro
                .ray_under_cursor(cursor)
                .unwrap_or_else(|| panic!("{cursor:?} esta dentro del cuadro"));

            assert!((rayo.direction.magnitude() - 1.0).abs() < 1e-6);
            assert_eq!(rayo.origin, cuadro.camera.eye);
        }
    }

    #[test]
    fn un_cursor_fuera_del_cuadro_no_da_rayo() {
        let cuadro = completo();

        for cursor in [
            (-0.1, 300.0),
            (400.0, -0.1),
            (800.0, 300.0),
            (400.0, 600.0),
            (1e6, 1e6),
        ] {
            assert!(
                cuadro.ray_under_cursor(cursor).is_none(),
                "{cursor:?} deberia quedar fuera"
            );
        }
    }

    #[test]
    fn un_cursor_no_finito_no_da_rayo() {
        // Sin este filtro el rayo saldria con direccion NaN, recorreria la
        // escena sin impactar nada y devolveria cielo: un picking roto
        // indistinguible de un clic en el vacio.
        let cuadro = completo();

        for cursor in [
            (f32::NAN, 300.0),
            (400.0, f32::NAN),
            (f32::INFINITY, 300.0),
            (400.0, f32::NEG_INFINITY),
        ] {
            assert!(cuadro.ray_under_cursor(cursor).is_none());
        }
    }

    #[test]
    fn el_borde_derecho_e_inferior_estan_excluidos() {
        // Igual que el indice de un arreglo: el ultimo pixel de 800 es el
        // 799, y `x = 800` esta una columna mas alla.
        let cuadro = completo();

        assert!(cuadro.ray_under_cursor((799.999, 300.0)).is_some());
        assert!(cuadro.ray_under_cursor((800.0, 300.0)).is_none());
        assert!(cuadro.ray_under_cursor((400.0, 599.999)).is_some());
        assert!(cuadro.ray_under_cursor((400.0, 600.0)).is_none());
    }

    #[test]
    fn el_rayo_del_cursor_es_el_mismo_que_traza_el_renderer() {
        // La promesa del Hito 6 dicha en un test: el rayo del picking y el
        // del render son **el mismo**, no aproximadamente el mismo. A
        // resolucion completa la igualdad es exacta, porque las dos rutas
        // llaman a `ray_from_pixel` con los mismos argumentos.
        let cuadro = completo();

        for (x, y) in [(0, 0), (13, 41), (400, 300), (799, 599)] {
            let del_render = cuadro.camera.ray_from_pixel(x, y, ANCHO, ALTO);

            // Cualquier punto **dentro** del pixel, no solo su centro: el
            // picking trunca al pixel, asi que las cuatro esquinas y el
            // centro dan lo mismo.
            for (dx, dy) in [(0.0, 0.0), (0.5, 0.5), (0.99, 0.01), (0.01, 0.99)] {
                let del_cursor = cuadro
                    .ray_under_cursor((x as f32 + dx, y as f32 + dy))
                    .expect("dentro del cuadro");

                assert_eq!(
                    del_render.direction, del_cursor.direction,
                    "el pixel ({x}, {y}) con offset ({dx}, {dy})"
                );
            }
        }
    }

    #[test]
    fn una_ventana_degenerada_no_divide_entre_cero_ni_con_escalado() {
        let cuadro = PresentedFrame::full(camara(), (0, 600));

        assert!(cuadro.ray_under_cursor((0.0, 0.0)).is_none());
    }

    // ------------------------------------- el cuadro escalado

    #[test]
    fn un_bloque_de_dos_por_dos_elige_el_mismo_pixel_fuente() {
        // Con el perfil interactivo a la mitad de lado, cada pixel fuente
        // ocupa un bloque de 2 x 2 en la ventana. El usuario ve ese bloque
        // como **un** punto de color, asi que los cuatro tienen que elegir
        // lo mismo.
        let cuadro = escalado();

        for (bx, by) in [(0, 0), (7, 13), (199, 149), (399, 299)] {
            let esperado = (bx, by);

            for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                let cursor = ((bx * 2 + dx) as f32 + 0.5, (by * 2 + dy) as f32 + 0.5);

                assert_eq!(
                    cuadro.source_pixel_at(cursor),
                    Some(esperado),
                    "el bloque ({bx}, {by}) difiere en el offset ({dx}, {dy})"
                );
            }
        }
    }

    #[test]
    fn el_bloque_de_dos_por_dos_tambien_comparte_el_rayo() {
        // Y no solo el indice: el rayo es identico, que es lo que hace que
        // el picking elija lo mismo en los cuatro.
        let cuadro = escalado();

        for (bx, by) in [(0, 0), (100, 75), (399, 299)] {
            let referencia = cuadro
                .camera
                .ray_from_pixel(bx, by, cuadro.source.0, cuadro.source.1);

            for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                let cursor = ((bx * 2 + dx) as f32 + 0.5, (by * 2 + dy) as f32 + 0.5);
                let rayo = cuadro.ray_under_cursor(cursor).expect("dentro");

                assert_eq!(rayo.direction, referencia.direction);
            }
        }
    }

    #[test]
    fn el_rayo_escalado_es_el_que_trazo_el_perfil_interactivo() {
        // La promesa completa: el clic resuelve contra el rayo que el
        // renderer trazo **a la resolucion del perfil**, no contra el que
        // habria trazado a resolucion de ventana. Los dos difieren.
        let cuadro = escalado();
        let cursor = (401.0, 301.0);

        let (sx, sy) = cuadro.source_pixel_at(cursor).expect("dentro");
        let del_perfil = cuadro
            .camera
            .ray_from_pixel(sx, sy, cuadro.source.0, cuadro.source.1);
        let del_cursor = cuadro.ray_under_cursor(cursor).expect("dentro");

        assert_eq!(del_cursor.direction, del_perfil.direction);

        // Y no coincide con el de la ventana: si coincidiera, el tipo no
        // haria falta. La diferencia es de medio pixel fuente.
        let de_ventana =
            cuadro
                .camera
                .ray_from_pixel(cursor.0 as usize, cursor.1 as usize, ANCHO, ALTO);
        let desvio = (del_cursor.direction - de_ventana.direction).magnitude();

        assert!(
            desvio > 1e-5,
            "las dos resoluciones dan el mismo rayo: el escalado no importaria"
        );
    }

    #[test]
    fn el_pixel_fuente_del_picking_es_el_que_dibuja_el_escalado() {
        // El mapeo es uno solo, y esto lo comprueba contra el propio
        // `blit_upscaled`: se pinta un patron en el borrador, se escala, y
        // el color bajo el cursor tiene que ser el del pixel que el picking
        // dice que hay ahi.
        use crate::framebuffer::Framebuffer;

        let (fuente_ancho, fuente_alto) = (7, 5);
        let (ventana_ancho, ventana_alto) = (31, 17);

        let mut fuente = Framebuffer::new(fuente_ancho, fuente_alto);
        for y in 0..fuente_alto {
            for x in 0..fuente_ancho {
                fuente.set_current_color((y * fuente_ancho + x) as u32 + 1);
                fuente.point(x, y);
            }
        }

        let mut ventana = Framebuffer::new(ventana_ancho, ventana_alto);
        ventana.blit_upscaled(&fuente);

        let cuadro = PresentedFrame {
            camera: camara(),
            source: (fuente_ancho, fuente_alto),
            window: (ventana_ancho, ventana_alto),
        };

        for y in 0..ventana_alto {
            for x in 0..ventana_ancho {
                let cursor = (x as f32 + 0.5, y as f32 + 0.5);
                let (sx, sy) = cuadro.source_pixel_at(cursor).expect("dentro");

                assert_eq!(
                    ventana.buffer[y * ventana_ancho + x],
                    fuente.buffer[sy * fuente_ancho + sx],
                    "el pixel de ventana ({x}, {y}) muestra otro pixel fuente"
                );
            }
        }
    }

    // ------------------------------------------------- picking de region

    use crate::accel::SceneAccel;
    use crate::color::Color;
    use crate::cuboid::Cuboid;
    use crate::material::Material;
    use crate::scene::{RevealGroup, Scene, SceneObject, SpatialGroupId};

    /// Escena con una losa por región, apiladas en profundidad para que un
    /// clic en distintas zonas del cuadro caiga en distintas regiones.
    ///
    /// La cuarta losa es **inerte**: mismo material inicial y final, como el
    /// plinto del diorama.
    fn escena_de_regiones() -> (Scene, SceneAccel, Camera) {
        let mut scene = Scene::new();
        let lienzo = scene.add_material(Material::new(Color::new(0.9, 0.87, 0.79)));
        let pintado = scene.add_material(Material::new(Color::new(0.3, 0.5, 0.2)));

        let mut poner = |centro: Vec3, grupo: RevealGroup, inicial| {
            scene.add_object(SceneObject {
                primitive: Cuboid::centrado(centro, Vec3::new(1.8, 1.8, 1.8)).into(),
                initial_material: inicial,
                final_material: pintado,
                spatial_group: SpatialGroupId::Global,
                reveal_group: grupo,
            });
        };

        poner(Vec3::new(-4.0, 0.0, 0.0), RevealGroup::Meadows, lienzo);
        poner(Vec3::new(0.0, 0.0, 0.0), RevealGroup::Breakwater, lienzo);
        poner(Vec3::new(4.0, 0.0, 0.0), RevealGroup::FlyingWaters, lienzo);
        // Inerte: nace y muere pintada, como el plinto.
        poner(Vec3::new(0.0, -4.0, 0.0), RevealGroup::Finale, pintado);

        let accel = SceneAccel::build(&scene).expect("hay geometria");
        let camara = Camera::new(
            Vec3::new(0.0, 0.0, 14.0),
            Vec3::zeros(),
            Vec3::zeros(),
            Vec3::new(0.0, 1.0, 0.0),
            DEFAULT_VERTICAL_FOV,
        );

        (scene, accel, camara)
    }

    /// Cursor que apunta al centro de una losa, resolviendo su proyeccion
    /// por busqueda: evita clavar coordenadas de pantalla a mano.
    fn cursor_sobre(camara: &Camera, objetivo: Vec3) -> (f32, f32) {
        let mut mejor = (0.0, 0.0);
        let mut minimo = f32::MAX;

        for i in 0..ANCHO {
            for j in 0..ALTO {
                let cursor = (i as f32 + 0.5, j as f32 + 0.5);
                let rayo = camara.ray_from_cursor(cursor, ANCHO, ALTO);
                let hacia = (objetivo - rayo.origin).normalize();
                let desvio = (rayo.direction - hacia).magnitude();

                if desvio < minimo {
                    minimo = desvio;
                    mejor = cursor;
                }
            }
        }

        mejor
    }

    #[test]
    fn un_clic_sobre_cada_region_devuelve_su_grupo() {
        let (scene, accel, camara) = escena_de_regiones();

        for (centro, esperado) in [
            (Vec3::new(-4.0, 0.0, 0.0), RevealGroup::Meadows),
            (Vec3::new(0.0, 0.0, 0.0), RevealGroup::Breakwater),
            (Vec3::new(4.0, 0.0, 0.0), RevealGroup::FlyingWaters),
        ] {
            let cursor = cursor_sobre(&camara, centro);
            let grupo = pick_region(
                &scene,
                &accel,
                &PresentedFrame::full(camara, (ANCHO, ALTO)),
                cursor,
            );

            assert_eq!(grupo, Some(esperado), "el clic en {cursor:?} eligio mal");
        }
    }

    #[test]
    fn un_clic_en_el_cielo_no_elige_region() {
        let (scene, accel, camara) = escena_de_regiones();

        // Esquina superior izquierda: por encima y a un lado de las losas.
        assert_eq!(
            pick_region(
                &scene,
                &accel,
                &PresentedFrame::full(camara, (ANCHO, ALTO)),
                (2.0, 2.0)
            ),
            None
        );
    }

    #[test]
    fn un_clic_sobre_una_entrada_inerte_no_elige_nada() {
        // El caso que este filtro existe para atrapar: el plinto ocupa toda
        // la base del diorama y comparte grupo con el Monolito por tipado.
        // Sin el filtro, pincharlo activaria el finale.
        let (scene, accel, camara) = escena_de_regiones();
        let cursor = cursor_sobre(&camara, Vec3::new(0.0, -4.0, 0.0));

        // El rayo si toca algo: lo que no hace es elegir region.
        let cuadro = PresentedFrame::full(camara, (ANCHO, ALTO));
        let rayo = cuadro.ray_under_cursor(cursor).expect("cursor dentro");
        assert!(accel
            .intersect(&scene, &rayo, &mut TraversalStats::default())
            .is_some());

        assert_eq!(
            pick_region(
                &scene,
                &accel,
                &PresentedFrame::full(camara, (ANCHO, ALTO)),
                cursor
            ),
            None
        );
    }

    #[test]
    fn un_clic_fuera_de_la_ventana_no_elige_region() {
        let (scene, accel, camara) = escena_de_regiones();

        for cursor in [(-1.0, 300.0), (800.0, 300.0), (f32::NAN, 300.0)] {
            assert_eq!(
                pick_region(
                    &scene,
                    &accel,
                    &PresentedFrame::full(camara, (ANCHO, ALTO)),
                    cursor
                ),
                None
            );
        }
    }

    #[test]
    fn el_picking_usa_el_mismo_rayo_que_el_render() {
        // Un clic en el centro de un pixel elige lo mismo que el renderer
        // dibuja en ese pixel. Es la promesa del Hito 6, comprobada contra
        // la escena y no solo contra la direccion del rayo.
        let (scene, accel, camara) = escena_de_regiones();

        for (x, y) in [(200, 300), (400, 300), (600, 300), (10, 10)] {
            let del_render = camara.ray_from_pixel(x, y, ANCHO, ALTO);
            let dibuja = accel
                .intersect(&scene, &del_render, &mut TraversalStats::default())
                .and_then(|h| scene.paintable_group(h.object_index));

            let elige = pick_region(
                &scene,
                &accel,
                &PresentedFrame::full(camara, (ANCHO, ALTO)),
                (x as f32 + 0.5, y as f32 + 0.5),
            );

            assert_eq!(elige, dibuja, "el pixel ({x}, {y}) discrepa");
        }
    }

    // --------------------------------------- picking detallado de pintura

    #[test]
    fn un_clic_detallado_entrega_objeto_uv_y_grupo() {
        let (scene, accel, camara) = escena_de_regiones();
        let cuadro = PresentedFrame::full(camara, (ANCHO, ALTO));

        for (indice, centro, esperado) in [
            (0usize, Vec3::new(-4.0, 0.0, 0.0), RevealGroup::Meadows),
            (1, Vec3::new(0.0, 0.0, 0.0), RevealGroup::Breakwater),
            (2, Vec3::new(4.0, 0.0, 0.0), RevealGroup::FlyingWaters),
        ] {
            let cursor = cursor_sobre(&camara, centro);
            let objetivo: PaintHit = pick_paint(&scene, &accel, &cuadro, cursor)
                .unwrap_or_else(|| panic!("el clic en {cursor:?} deberia elegir una superficie"));

            assert_eq!(objetivo.object_index, indice, "eligio otro objeto");
            assert_eq!(objetivo.reveal_group, esperado);
            assert!(
                (0.0..=1.0).contains(&objetivo.uv.x) && (0.0..=1.0).contains(&objetivo.uv.y),
                "la uv {:?} tiene que caer dentro de la cara",
                objetivo.uv
            );
        }
    }

    #[test]
    fn la_uv_detallada_apunta_al_punto_que_se_pincho() {
        // No basta con que este en rango: una uv constante, o con los ejes
        // intercambiados, tambien lo estaria. La losa central se mira de
        // frente, asi que un clic a su centro cae al centro de su cara.
        let (scene, accel, camara) = escena_de_regiones();
        let cuadro = PresentedFrame::full(camara, (ANCHO, ALTO));
        let cursor = cursor_sobre(&camara, Vec3::zeros());

        let objetivo = pick_paint(&scene, &accel, &cuadro, cursor).expect("da en la losa central");

        assert!(
            (objetivo.uv.x - 0.5).abs() < 0.05 && (objetivo.uv.y - 0.5).abs() < 0.05,
            "el centro de la cara deberia dar una uv proxima a (0.5, 0.5), dio {:?}",
            objetivo.uv
        );
    }

    #[test]
    fn el_picking_detallado_rechaza_lo_inerte_y_el_finale() {
        // Las dos exclusiones de `Scene::paintable_group`, ahora sobre el
        // impacto detallado: el plinto nace pintado, y el Monolito no se
        // elige aunque sea revelable.
        let (scene, accel, camara) = escena_de_regiones();
        let cuadro = PresentedFrame::full(camara, (ANCHO, ALTO));
        let cursor = cursor_sobre(&camara, Vec3::new(0.0, -4.0, 0.0));
        let rayo = cuadro.ray_under_cursor(cursor).expect("cursor dentro");

        assert!(
            accel
                .intersect(&scene, &rayo, &mut TraversalStats::default())
                .is_some(),
            "el rayo si toca la losa inerte: lo que no hace es elegirla"
        );
        assert_eq!(pick_paint(&scene, &accel, &cuadro, cursor), None);

        let (finale, accel_finale, camara_finale) = escena_del_finale();
        let cuadro_finale = PresentedFrame::full(camara_finale, (ANCHO, ALTO));

        assert!(
            finale.objects[0].is_revealable(),
            "el monolito de prueba es revelable"
        );
        assert_eq!(
            pick_paint(&finale, &accel_finale, &cuadro_finale, (400.0, 300.0)),
            None,
            "un objeto del finale no es un objetivo de pintura"
        );
    }

    #[test]
    fn el_picking_detallado_ignora_el_cielo_y_el_cursor_fuera() {
        let (scene, accel, camara) = escena_de_regiones();
        let cuadro = PresentedFrame::full(camara, (ANCHO, ALTO));

        // Cielo, los cuatro rechazos de cursor, y el no finito.
        for cursor in [
            (2.0, 2.0),
            (-1.0, 300.0),
            (400.0, -1.0),
            (800.0, 300.0),
            (400.0, 600.0),
            (f32::NAN, 300.0),
        ] {
            assert_eq!(
                pick_paint(&scene, &accel, &cuadro, cursor),
                None,
                "{cursor:?} no deberia elegir nada"
            );
        }
    }

    #[test]
    fn pick_region_es_el_grupo_del_picking_detallado() {
        // `pick_region` no es una segunda implementacion de la misma
        // politica: es la misma eleccion contada mas corta. Si las dos
        // pudieran discrepar, un clic pintaria una region y la brocha otra.
        let (scene, accel, camara) = escena_de_regiones();
        let cuadro = PresentedFrame::full(camara, (ANCHO, ALTO));
        let mut elegidas = 0;

        for centro in [
            Vec3::new(-4.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, -4.0, 0.0),
        ] {
            let cursor = cursor_sobre(&camara, centro);
            let detallado = pick_paint(&scene, &accel, &cuadro, cursor);

            if detallado.is_some() {
                elegidas += 1;
            }

            assert_eq!(
                pick_region(&scene, &accel, &cuadro, cursor),
                detallado.map(|objetivo| objetivo.reveal_group),
                "discrepan sobre {cursor:?}"
            );
        }

        assert_eq!(elegidas, 3, "tres de las cuatro losas son elegibles");

        for cursor in [(2.0, 2.0), (-1.0, 300.0), (f32::NAN, 300.0)] {
            assert_eq!(
                pick_region(&scene, &accel, &cuadro, cursor),
                pick_paint(&scene, &accel, &cuadro, cursor).map(|objetivo| objetivo.reveal_group),
                "discrepan sobre {cursor:?}"
            );
        }
    }

    #[test]
    fn el_picking_conserva_la_carta_que_dio_la_primitiva() {
        // La carta no se reconstruye en el consumidor: se transporta. Se
        // compara contra el impacto crudo del mismo rayo, no contra un
        // numero escrito a mano, para que el test no fije el reparto
        // interno de `Cuboid` sino la fidelidad del transporte.
        let (scene, accel, camara) = escena_de_regiones();
        let cuadro = PresentedFrame::full(camara, (ANCHO, ALTO));

        for centro in [
            Vec3::new(-4.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
        ] {
            let cursor = cursor_sobre(&camara, centro);
            let rayo = cuadro.ray_under_cursor(cursor).expect("cursor dentro");
            let crudo = accel
                .intersect(&scene, &rayo, &mut TraversalStats::default())
                .expect("el rayo da en una losa");
            let objetivo = pick_paint(&scene, &accel, &cuadro, cursor).expect("losa elegible");

            assert_eq!(
                objetivo.uv_chart, crudo.uv_chart,
                "la carta se perdio o se altero en el camino"
            );
            assert_eq!(objetivo.uv, crudo.uv);
            assert_eq!(objetivo.object_index, crudo.object_index);
        }
    }

    #[test]
    fn dos_caras_del_mismo_objeto_se_distinguen_por_la_carta() {
        // El caso que da sentido al campo: el mismo objeto, la misma `uv`,
        // dos caras opuestas. Sin carta serian el mismo punto pintable.
        //
        // Se mira desde dos camaras simetricas contra la losa central, asi
        // que no depende de que objeto quede en que indice.
        let (scene, accel, _) = escena_de_regiones();
        let cuadro_de = |desde: Vec3| {
            PresentedFrame::full(
                Camera::new(
                    desde,
                    Vec3::zeros(),
                    Vec3::zeros(),
                    Vec3::new(0.0, 1.0, 0.0),
                    DEFAULT_VERTICAL_FOV,
                ),
                (ANCHO, ALTO),
            )
        };

        let centro = (ANCHO as f32 / 2.0, ALTO as f32 / 2.0);
        let frente = pick_paint(
            &scene,
            &accel,
            &cuadro_de(Vec3::new(0.0, 0.0, 14.0)),
            centro,
        )
        .expect("cara +Z de la losa central");
        let espalda = pick_paint(
            &scene,
            &accel,
            &cuadro_de(Vec3::new(0.0, 0.0, -14.0)),
            centro,
        )
        .expect("cara -Z de la misma losa");

        assert_eq!(
            frente.object_index, espalda.object_index,
            "las dos vistas tienen que dar en la misma losa"
        );
        assert_eq!(frente.reveal_group, espalda.reveal_group);
        assert_ne!(
            frente.uv_chart, espalda.uv_chart,
            "dos caras opuestas del mismo objeto no son el mismo trozo pintable"
        );
    }

    /// Impacto crudo del mismo rayo, para comparar sin reimplementar nada.
    fn crudo_bajo(
        scene: &Scene,
        accel: &SceneAccel,
        cuadro: &PresentedFrame,
        cursor: (f32, f32),
    ) -> crate::hit::Hit {
        let rayo = cuadro.ray_under_cursor(cursor).expect("cursor dentro");

        accel
            .intersect(scene, &rayo, &mut TraversalStats::default())
            .expect("el rayo impacta")
    }

    #[test]
    fn el_picking_entrega_el_punto_y_la_normal_del_impacto() {
        // El gizmo necesita saber **donde** y **hacia donde** esta la
        // superficie. Los dos datos ya venian en el `Hit`: se transportan,
        // no se recalculan ni se reclasifica el lado.
        let (scene, accel, camara) = escena_de_regiones();
        let cuadro = PresentedFrame::full(camara, (ANCHO, ALTO));

        for centro in [
            Vec3::new(-4.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
        ] {
            let cursor = cursor_sobre(&camara, centro);
            let objetivo = pick_paint(&scene, &accel, &cuadro, cursor).expect("region elegible");
            let crudo = crudo_bajo(&scene, &accel, &cuadro, cursor);

            assert_eq!(objetivo.point, crudo.point, "el punto no se transporto");
            assert_eq!(objetivo.normal, crudo.normal, "la normal no se transporto");

            // Y lo que ya entregaba sigue igual.
            assert_eq!(objetivo.object_index, crudo.object_index);
            assert_eq!(objetivo.uv, crudo.uv);
            assert_eq!(objetivo.uv_chart, crudo.uv_chart);
        }
    }

    #[test]
    fn el_picking_artistico_tambien_entrega_punto_y_normal() {
        let (scene, accel, camara) = escena_artistica();
        let cuadro = PresentedFrame::full(camara, (ANCHO, ALTO));
        let cursor = cursor_sobre(&camara, Vec3::new(2.0, 0.0, 0.0));

        let objetivo = pick_artistic(&scene, &accel, &cuadro, cursor).expect("el monolito");
        let crudo = crudo_bajo(&scene, &accel, &cuadro, cursor);

        assert_eq!(objetivo.point, crudo.point);
        assert_eq!(objetivo.normal, crudo.normal);
        assert_eq!(
            objetivo.reveal_group,
            RevealGroup::Finale,
            "la politica no cambia"
        );
    }

    #[test]
    fn el_picking_entrega_la_metrica_de_la_primitiva() {
        // La metrica viaja desde la primitiva, no se reconstruye aqui. Se
        // compara contra la que da el propio objeto para la carta del
        // impacto, de modo que el test no fija el reparto interno.
        let (scene, accel, camara) = escena_de_regiones();
        let cuadro = PresentedFrame::full(camara, (ANCHO, ALTO));

        for centro in [
            Vec3::new(-4.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
        ] {
            let cursor = cursor_sobre(&camara, centro);
            let objetivo = pick_paint(&scene, &accel, &cuadro, cursor).expect("region elegible");
            let esperada = scene.objects[objetivo.object_index]
                .primitive
                .uv_world_scale(objetivo.uv_chart)
                .expect("la losa tiene metrica");

            assert_eq!(objetivo.uv_world_scale, esperada);

            // Las losas son cubos de lado 1.8: las seis caras miden lo
            // mismo en los dos ejes.
            assert!((objetivo.uv_world_scale.u - 1.8).abs() < 1e-5);
            assert!((objetivo.uv_world_scale.v - 1.8).abs() < 1e-5);
        }
    }

    #[test]
    fn el_picking_artistico_tambien_entrega_la_metrica() {
        let (scene, accel, camara) = escena_artistica();
        let cuadro = PresentedFrame::full(camara, (ANCHO, ALTO));
        let cursor = cursor_sobre(&camara, Vec3::new(-2.0, 0.0, 0.0));

        let objetivo = pick_artistic(&scene, &accel, &cuadro, cursor).expect("el plinto inerte");
        let esperada = scene.objects[objetivo.object_index]
            .primitive
            .uv_world_scale(objetivo.uv_chart)
            .expect("tiene metrica");

        assert_eq!(objetivo.uv_world_scale, esperada);
    }

    // --------------------------------------------- picking artistico

    /// Cuatro losas separadas en `X`, una por caso de politica.
    ///
    /// Los indices son los que fija este fixture, no los del diorama real.
    fn escena_artistica() -> (Scene, SceneAccel, Camera) {
        let mut scene = Scene::new();
        let lienzo = scene.add_material(Material::new(Color::new(0.9, 0.87, 0.79)));
        let pintado = scene.add_material(Material::new(Color::new(0.3, 0.5, 0.2)));

        let mut poner = |x: f32, grupo_espacial, grupo, inicial| {
            scene.add_object(SceneObject {
                primitive: Cuboid::centrado(Vec3::new(x, 0.0, 0.0), Vec3::new(1.8, 1.8, 1.8))
                    .into(),
                initial_material: inicial,
                final_material: pintado,
                spatial_group: grupo_espacial,
                reveal_group: grupo,
            });
        };

        // 0: region normal. 1: plinto inerte del finale. 2: monolito
        // revelable del finale. 3: la paleta fisica.
        poner(-6.0, SpatialGroupId::Meadows, RevealGroup::Meadows, lienzo);
        poner(-2.0, SpatialGroupId::Global, RevealGroup::Finale, pintado);
        poner(2.0, SpatialGroupId::Monolith, RevealGroup::Finale, lienzo);
        poner(
            6.0,
            SpatialGroupId::InteractionProps,
            RevealGroup::Meadows,
            lienzo,
        );

        let accel = SceneAccel::build(&scene).expect("hay geometria");
        let camara = Camera::new(
            Vec3::new(0.0, 0.0, 18.0),
            Vec3::zeros(),
            Vec3::zeros(),
            Vec3::new(0.0, 1.0, 0.0),
            DEFAULT_VERTICAL_FOV,
        );

        (scene, accel, camara)
    }

    #[test]
    fn el_picker_artistico_acepta_lo_inerte_y_el_finale() {
        let (scene, accel, camara) = escena_artistica();
        let cuadro = PresentedFrame::full(camara, (ANCHO, ALTO));

        for (indice, x, grupo) in [
            (0usize, -6.0f32, RevealGroup::Meadows),
            (1, -2.0, RevealGroup::Finale),
            (2, 2.0, RevealGroup::Finale),
        ] {
            let cursor = cursor_sobre(&camara, Vec3::new(x, 0.0, 0.0));
            let objetivo = pick_artistic(&scene, &accel, &cuadro, cursor)
                .unwrap_or_else(|| panic!("la losa {indice} deberia aceptarse"));

            assert_eq!(objetivo.object_index, indice);
            assert_eq!(objetivo.reveal_group, grupo);
            assert!(
                (0.0..=1.0).contains(&objetivo.uv.x) && (0.0..=1.0).contains(&objetivo.uv.y),
                "uv fuera de la cara: {:?}",
                objetivo.uv
            );

            // La carta es la que dio la primitiva, no una reconstruida.
            let rayo = cuadro.ray_under_cursor(cursor).expect("cursor dentro");
            let crudo = accel
                .intersect(&scene, &rayo, &mut TraversalStats::default())
                .expect("el rayo impacta");

            assert_eq!(objetivo.uv_chart, crudo.uv_chart);
            assert_eq!(objetivo.uv, crudo.uv);
        }
    }

    #[test]
    fn el_picker_artistico_rechaza_la_paleta_y_el_pincel() {
        // El rayo **si** toca la losa de atrezo: lo que no hace es
        // ofrecerla como superficie pintable.
        let (scene, accel, camara) = escena_artistica();
        let cuadro = PresentedFrame::full(camara, (ANCHO, ALTO));
        let cursor = cursor_sobre(&camara, Vec3::new(6.0, 0.0, 0.0));
        let rayo = cuadro.ray_under_cursor(cursor).expect("cursor dentro");

        let crudo = accel
            .intersect(&scene, &rayo, &mut TraversalStats::default())
            .expect("el rayo toca el atrezo");
        assert_eq!(
            crudo.object_index, 3,
            "el ejemplo tiene que dar en la paleta"
        );

        assert_eq!(pick_artistic(&scene, &accel, &cuadro, cursor), None);
    }

    #[test]
    fn el_picker_de_entrega_sigue_excluyendo_lo_inerte_y_el_finale() {
        // La misma escena por la otra politica: el contrato de entrega no
        // se ha movido.
        let (scene, accel, camara) = escena_artistica();
        let cuadro = PresentedFrame::full(camara, (ANCHO, ALTO));

        for x in [-2.0f32, 2.0] {
            let cursor = cursor_sobre(&camara, Vec3::new(x, 0.0, 0.0));

            assert!(
                pick_artistic(&scene, &accel, &cuadro, cursor).is_some(),
                "el artistico si la acepta"
            );
            assert_eq!(
                pick_paint(&scene, &accel, &cuadro, cursor),
                None,
                "y el de entrega no"
            );
        }
    }

    #[test]
    fn el_picker_artistico_ignora_el_cielo_y_el_cursor_fuera() {
        let (scene, accel, camara) = escena_artistica();
        let cuadro = PresentedFrame::full(camara, (ANCHO, ALTO));

        for cursor in [
            (2.0, 2.0),
            (-1.0, 300.0),
            (800.0, 300.0),
            (400.0, 600.0),
            (f32::NAN, 300.0),
        ] {
            assert_eq!(
                pick_artistic(&scene, &accel, &cuadro, cursor),
                None,
                "{cursor:?} no deberia elegir nada"
            );
        }
    }

    #[test]
    fn los_dos_pickers_coinciden_sobre_las_tres_regiones() {
        // Donde las dos politicas aceptan, tienen que entregar lo mismo:
        // el modo artistico amplia el objetivo, no cambia el resultado.
        let (scene, accel, camara) = escena_de_regiones();
        let cuadro = PresentedFrame::full(camara, (ANCHO, ALTO));

        for centro in [
            Vec3::new(-4.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
        ] {
            let cursor = cursor_sobre(&camara, centro);
            let entrega = pick_paint(&scene, &accel, &cuadro, cursor).expect("region elegible");
            let artistico =
                pick_artistic(&scene, &accel, &cuadro, cursor).expect("tambien para el artistico");

            assert_eq!(entrega, artistico, "los dos pickers discrepan");
        }
    }

    // ------------------------------------------------- fallback de teclado

    #[test]
    fn el_fallback_mapea_los_tres_digitos_del_plan() {
        assert_eq!(demo_region(1), Some(RevealGroup::Meadows));
        assert_eq!(demo_region(2), Some(RevealGroup::Breakwater));
        assert_eq!(demo_region(3), Some(RevealGroup::FlyingWaters));
    }

    #[test]
    fn el_fallback_no_incluye_el_finale_ni_desborda() {
        // `Finale` fuera a proposito: el Monolito no se elige, es la
        // consecuencia de haber pintado las tres regiones.
        assert_eq!(demo_region(4), None);
        assert_eq!(demo_region(0), None, "el cero no debe restar por debajo");
        assert_eq!(demo_region(255), None);

        assert!(!DEMO_REGIONS.contains(&RevealGroup::Finale));
        assert_eq!(DEMO_REGIONS.len(), 3);
    }

    #[test]
    fn el_fallback_cubre_las_mismas_regiones_que_el_picking() {
        // Las dos rutas tienen que poder llegar a lo mismo, o la
        // presentacion con teclado no seria equivalente a la del raton.
        let (scene, accel, camara) = escena_de_regiones();

        for (indice, grupo) in DEMO_REGIONS.iter().enumerate() {
            let digito = indice as u8 + 1;
            assert_eq!(demo_region(digito), Some(*grupo));
        }

        let centros = [
            Vec3::new(-4.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
        ];

        for (centro, grupo) in centros.iter().zip(&DEMO_REGIONS) {
            let cursor = cursor_sobre(&camara, *centro);

            assert_eq!(
                pick_region(
                    &scene,
                    &accel,
                    &PresentedFrame::full(camara, (ANCHO, ALTO)),
                    cursor
                ),
                Some(*grupo)
            );
        }
    }

    // ------------------------------------------------- superficie de teclado

    #[test]
    fn las_teclas_de_la_demo_hacen_lo_que_anuncia_la_ventana() {
        assert_eq!(
            demo_action('1'),
            Some(DemoAction::Paint(RevealGroup::Meadows))
        );
        assert_eq!(
            demo_action('2'),
            Some(DemoAction::Paint(RevealGroup::Breakwater))
        );
        assert_eq!(
            demo_action('3'),
            Some(DemoAction::Paint(RevealGroup::FlyingWaters))
        );
        assert_eq!(demo_action('L'), Some(DemoAction::ResetCanvas));
        assert_eq!(demo_action('R'), Some(DemoAction::ResetCamera));
    }

    #[test]
    fn las_teclas_no_distinguen_mayusculas() {
        assert_eq!(demo_action('l'), demo_action('L'));
        assert_eq!(demo_action('r'), demo_action('R'));
    }

    #[test]
    fn ninguna_otra_tecla_hace_nada() {
        for tecla in ['0', '4', '9', 'a', 'W', 'S', ' ', 'ñ'] {
            assert_eq!(
                demo_action(tecla),
                None,
                "la tecla {tecla} no deberia actuar"
            );
        }
    }

    #[test]
    fn las_dos_teclas_de_reset_no_se_pisan() {
        // La 6.2 uso `R` para volver al lienzo y la 6.5 la reserva para la
        // camara. Este test es el que impide que vuelvan a colisionar.
        assert_ne!(demo_action('L'), demo_action('R'));
        assert_eq!(demo_action('L'), Some(DemoAction::ResetCanvas));
        assert_eq!(demo_action('R'), Some(DemoAction::ResetCamera));
    }

    #[test]
    fn reset_camera_no_transporta_ningun_encuadre() {
        // La accion solo dice «restaura»: los tres puntos los aporta la
        // escena. Si `ResetCamera` llevara un `CameraPreset`, el encuadre
        // del blueprint acabaria escrito en algun sitio de `input`, que es
        // justo lo que el plan prohibe.
        assert_eq!(
            std::mem::size_of::<DemoAction>(),
            std::mem::size_of::<Option<RevealGroup>>(),
            "DemoAction crecio: alguien le colgo datos"
        );
    }

    #[test]
    fn la_superficie_de_teclado_cubre_las_tres_regiones() {
        // Sin raton se tiene que poder llegar a la demo completa.
        let alcanzables: Vec<RevealGroup> = ['1', '2', '3']
            .iter()
            .filter_map(|k| match demo_action(*k) {
                Some(DemoAction::Paint(grupo)) => Some(grupo),
                _ => None,
            })
            .collect();

        assert_eq!(alcanzables, DEMO_REGIONS.to_vec());
    }

    /// Escena de un solo objeto **revelable y del finale**: el caso que la
    /// capa de picking tiene que rechazar, y no por inerte.
    fn escena_del_finale() -> (Scene, SceneAccel, Camera) {
        let mut scene = Scene::new();
        let lienzo = scene.add_material(Material::new(Color::new(0.9, 0.87, 0.79)));
        let cristal = scene.add_material(Material::new(Color::new(0.62, 0.86, 0.92)));

        scene.add_object(SceneObject {
            primitive: Cuboid::centrado(Vec3::zeros(), Vec3::new(2.0, 6.0, 2.0)).into(),
            initial_material: lienzo,
            final_material: cristal,
            spatial_group: SpatialGroupId::Monolith,
            reveal_group: RevealGroup::Finale,
        });

        let accel = SceneAccel::build(&scene).expect("hay geometria");
        let camara = Camera::new(
            Vec3::new(0.0, 0.0, 14.0),
            Vec3::zeros(),
            Vec3::zeros(),
            Vec3::new(0.0, 1.0, 0.0),
            DEFAULT_VERTICAL_FOV,
        );

        (scene, accel, camara)
    }

    #[test]
    fn un_clic_sobre_algo_del_finale_no_selecciona_nada() {
        // El Monolito no es una region que se elija: es la consecuencia de
        // haber pintado las tres. Un clic sobre el lo adelantaria, y la
        // condicion que gobierna el climax quedaria en manos de donde
        // apunte el puntero.
        let (scene, accel, camara) = escena_del_finale();

        let objeto = scene.objects[0];
        assert!(
            objeto.is_revealable(),
            "el objeto de prueba debe ser revelable"
        );

        // El rayo si da en el Monolito.
        let centro = (400.0, 300.0);
        let cuadro = PresentedFrame::full(camara, (ANCHO, ALTO));
        let rayo = cuadro.ray_under_cursor(centro).expect("cursor dentro");
        assert!(accel
            .intersect(&scene, &rayo, &mut TraversalStats::default())
            .is_some());

        // Y aun asi no selecciona region.
        assert_eq!(
            pick_region(
                &scene,
                &accel,
                &PresentedFrame::full(camara, (ANCHO, ALTO)),
                centro
            ),
            None
        );
    }

    // ------------------------------------------------- reducer del cuadro

    #[test]
    fn reset_canvas_domina_sobre_pintar_en_el_mismo_cuadro() {
        // El fallo que este reducer existe para cerrar: `L` reemplazaba el
        // estado y la seleccion del mismo cuadro se aplicaba despues, sobre
        // el lienzo recien reiniciado. La consola decia «reiniciado» y una
        // region quedaba revelandose.
        let intent = FrameIntent::from_actions([
            DemoAction::Paint(RevealGroup::Meadows),
            DemoAction::ResetCanvas,
        ]);

        assert!(intent.reset_canvas);
        assert_eq!(intent.paints().count(), 0, "el pintado sobrevivio al reset");

        // Y en el orden contrario da lo mismo: la precedencia no depende de
        // que tecla se sondeo antes.
        let alreves = FrameIntent::from_actions([
            DemoAction::ResetCanvas,
            DemoAction::Paint(RevealGroup::Meadows),
        ]);

        assert_eq!(alreves, intent);
    }

    #[test]
    fn varias_selecciones_del_mismo_cuadro_se_conservan_todas() {
        // Con una sola variable, `1 + 2 + 3` en el mismo sondeo dejaba solo
        // la ultima.
        let intent = FrameIntent::from_actions([
            DemoAction::Paint(RevealGroup::Meadows),
            DemoAction::Paint(RevealGroup::Breakwater),
            DemoAction::Paint(RevealGroup::FlyingWaters),
        ]);

        let grupos: Vec<RevealGroup> = intent.paints().collect();

        assert_eq!(grupos.len(), 3, "se perdio alguna seleccion");
        assert_eq!(grupos, DEMO_REGIONS.to_vec());
    }

    #[test]
    fn una_seleccion_repetida_no_se_duplica() {
        // Un clic y una tecla sobre la misma region son una sola cosa.
        let intent = FrameIntent::from_actions([
            DemoAction::Paint(RevealGroup::Breakwater),
            DemoAction::Paint(RevealGroup::Breakwater),
        ]);

        assert_eq!(intent.paints().count(), 1);
    }

    #[test]
    fn reset_camera_convive_con_todo() {
        // Mover la camara y pintar son independientes: `R` no cancela nada
        // ni nada lo cancela a el.
        let intent = FrameIntent::from_actions([
            DemoAction::Paint(RevealGroup::Meadows),
            DemoAction::ResetCamera,
        ]);

        assert!(intent.reset_camera);
        assert_eq!(intent.paints().count(), 1);

        // Incluso junto a un reset de lienzo, que si borra el pintado.
        let con_lienzo = FrameIntent::from_actions([
            DemoAction::Paint(RevealGroup::Meadows),
            DemoAction::ResetCanvas,
            DemoAction::ResetCamera,
        ]);

        assert!(con_lienzo.reset_camera, "el reset de camara sobrevive");
        assert!(con_lienzo.reset_canvas);
        assert_eq!(con_lienzo.paints().count(), 0);
    }

    #[test]
    fn un_cuadro_sin_entrada_no_pide_nada() {
        let intent = FrameIntent::from_actions([]);

        assert!(intent.is_empty());
        assert_eq!(intent.paints().count(), 0);
        assert!(!intent.reset_canvas);
        assert!(!intent.reset_camera);

        // Y cualquier accion lo deja de estar.
        assert!(!FrameIntent::from_actions([DemoAction::ResetCamera]).is_empty());
    }

    #[test]
    fn el_reducer_no_desborda_con_mas_acciones_que_grupos() {
        // Cuatro huecos, y el finale no llega por picking; aun asi el
        // reducer no debe indexar fuera de rango si alguien insiste.
        let muchas: Vec<DemoAction> = (0..50)
            .map(|i| DemoAction::Paint(RevealGroup::ALL[i % RevealGroup::COUNT]))
            .collect();

        let intent = FrameIntent::from_actions(muchas);

        assert_eq!(intent.paints().count(), RevealGroup::COUNT);
    }
}
