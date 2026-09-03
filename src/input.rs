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
//! El tamaño que se pasa es el de la **ventana**, el mismo con el que se
//! presenta el framebuffer. El escalado por vecino más cercano del perfil
//! interactivo preserva las coordenadas normalizadas, así que un píxel de
//! ventana y su píxel de perfil caen en la misma coordenada de pantalla y no
//! hay ambigüedad entre los dos tamaños.

use crate::accel::{SceneAccel, TraversalStats};
use crate::camera::Camera;
use crate::ray::Ray;
use crate::scene::{RevealGroup, Scene};

/// Rayo bajo el cursor, o `None` si ese cursor no señala el cuadro.
///
/// Se rechazan dos cosas, y ninguna es paranoia:
///
/// - **Fuera de la ventana.** `minifb` entrega la última posición conocida
///   del puntero incluso cuando salió del área de dibujo, así que un clic
///   registrado ahí apuntaría a geometría que el usuario no está viendo.
/// - **No finito.** Un `NaN` propagado desde el sistema de ventanas
///   produciría una dirección `NaN`, y un rayo así no falla: recorre la
///   escena, no impacta nada y devuelve cielo. Un picking que «no encuentra
///   nada» es indistinguible de un clic en el vacío, y ese es justo el
///   error que no se quiere depurar mirando la pantalla.
///
/// El borde derecho e inferior quedan **excluidos**: un cursor en `x = 800`
/// de una ventana de `800` está una columna más allá del último píxel, igual
/// que el índice `800` de un arreglo de `800`.
pub fn ray_under_cursor(
    camera: &Camera,
    cursor: (f32, f32),
    width: usize,
    height: usize,
) -> Option<Ray> {
    if !cursor.0.is_finite() || !cursor.1.is_finite() {
        return None;
    }

    if cursor.0 < 0.0 || cursor.1 < 0.0 {
        return None;
    }

    if cursor.0 >= width as f32 || cursor.1 >= height as f32 {
        return None;
    }

    Some(camera.ray_from_cursor(cursor, width, height))
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
/// - El cursor no señala el cuadro. Lo resuelve `ray_under_cursor`.
/// - El rayo no toca nada: un clic en el cielo.
/// - Lo que toca no se pinta. El plinto ocupa toda la base del diorama y
///   comparte grupo con el Monolito por tipado; sin este filtro, pincharlo
///   activaría el finale. Ver `Scene::paintable_group`.
pub fn pick_region(
    scene: &Scene,
    accel: &SceneAccel,
    camera: &Camera,
    cursor: (f32, f32),
    width: usize,
    height: usize,
) -> Option<RevealGroup> {
    let ray = ray_under_cursor(camera, cursor, width, height)?;

    // Un rayo por clic: los contadores de recorrido no se llevan a ninguna
    // parte, así que se descartan aquí en vez de obligar al llamador a
    // cargar un acumulador que no va a leer.
    let hit = accel.intersect(scene, &ray, &mut TraversalStats::default())?;

    scene.paintable_group(hit.object_index)
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

    #[test]
    fn un_cursor_dentro_del_cuadro_da_rayo() {
        let camara = camara();

        for cursor in [(0.0, 0.0), (400.0, 300.0), (799.9, 599.9)] {
            let rayo = ray_under_cursor(&camara, cursor, ANCHO, ALTO)
                .unwrap_or_else(|| panic!("{cursor:?} esta dentro del cuadro"));

            assert!((rayo.direction.magnitude() - 1.0).abs() < 1e-6);
            assert_eq!(rayo.origin, camara.eye);
        }
    }

    #[test]
    fn un_cursor_fuera_del_cuadro_no_da_rayo() {
        let camara = camara();

        for cursor in [
            (-0.1, 300.0),
            (400.0, -0.1),
            (800.0, 300.0),
            (400.0, 600.0),
            (1e6, 1e6),
        ] {
            assert!(
                ray_under_cursor(&camara, cursor, ANCHO, ALTO).is_none(),
                "{cursor:?} deberia quedar fuera"
            );
        }
    }

    #[test]
    fn un_cursor_no_finito_no_da_rayo() {
        // Sin este filtro el rayo saldria con direccion NaN, recorreria la
        // escena sin impactar nada y devolveria cielo: un picking roto
        // indistinguible de un clic en el vacio.
        let camara = camara();

        for cursor in [
            (f32::NAN, 300.0),
            (400.0, f32::NAN),
            (f32::INFINITY, 300.0),
            (400.0, f32::NEG_INFINITY),
        ] {
            assert!(ray_under_cursor(&camara, cursor, ANCHO, ALTO).is_none());
        }
    }

    #[test]
    fn el_borde_derecho_e_inferior_estan_excluidos() {
        // Igual que el indice de un arreglo: el ultimo pixel de 800 es el
        // 799, y `x = 800` esta una columna mas alla.
        let camara = camara();

        assert!(ray_under_cursor(&camara, (799.999, 300.0), ANCHO, ALTO).is_some());
        assert!(ray_under_cursor(&camara, (800.0, 300.0), ANCHO, ALTO).is_none());
        assert!(ray_under_cursor(&camara, (400.0, 599.999), ANCHO, ALTO).is_some());
        assert!(ray_under_cursor(&camara, (400.0, 600.0), ANCHO, ALTO).is_none());
    }

    #[test]
    fn el_rayo_del_cursor_es_el_mismo_que_traza_el_renderer() {
        // La promesa del Hito 6 dicha en un test: para el centro de un
        // pixel, el rayo del picking y el del render coinciden. Se comprueba
        // aqui ademas de en `camera` porque es la capa que el binario llama.
        let camara = camara();

        for (x, y) in [(0, 0), (13, 41), (400, 300), (799, 599)] {
            let del_render = camara.ray_from_pixel(x, y, ANCHO, ALTO);
            let del_cursor =
                ray_under_cursor(&camara, (x as f32 + 0.5, y as f32 + 0.5), ANCHO, ALTO)
                    .expect("el centro de un pixel esta dentro del cuadro");

            let desvio = (del_render.direction - del_cursor.direction).magnitude();

            assert!(desvio < 1e-6, "el pixel ({x}, {y}) desvio {desvio}");
        }
    }

    #[test]
    fn una_ventana_degenerada_no_divide_entre_cero() {
        // No es alcanzable con `minifb`, pero el rango vacio hace que todo
        // cursor quede fuera, y eso resuelve el caso sin dividir.
        let camara = camara();

        assert!(ray_under_cursor(&camara, (0.0, 0.0), 0, 600).is_none());
        assert!(ray_under_cursor(&camara, (0.0, 0.0), 800, 0).is_none());
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
            let grupo = pick_region(&scene, &accel, &camara, cursor, ANCHO, ALTO);

            assert_eq!(grupo, Some(esperado), "el clic en {cursor:?} eligio mal");
        }
    }

    #[test]
    fn un_clic_en_el_cielo_no_elige_region() {
        let (scene, accel, camara) = escena_de_regiones();

        // Esquina superior izquierda: por encima y a un lado de las losas.
        assert_eq!(
            pick_region(&scene, &accel, &camara, (2.0, 2.0), ANCHO, ALTO),
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
        let rayo = ray_under_cursor(&camara, cursor, ANCHO, ALTO).expect("cursor dentro");
        assert!(accel
            .intersect(&scene, &rayo, &mut TraversalStats::default())
            .is_some());

        assert_eq!(
            pick_region(&scene, &accel, &camara, cursor, ANCHO, ALTO),
            None
        );
    }

    #[test]
    fn un_clic_fuera_de_la_ventana_no_elige_region() {
        let (scene, accel, camara) = escena_de_regiones();

        for cursor in [(-1.0, 300.0), (800.0, 300.0), (f32::NAN, 300.0)] {
            assert_eq!(
                pick_region(&scene, &accel, &camara, cursor, ANCHO, ALTO),
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
                &camara,
                (x as f32 + 0.5, y as f32 + 0.5),
                ANCHO,
                ALTO,
            );

            assert_eq!(elige, dibuja, "el pixel ({x}, {y}) discrepa");
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
                pick_region(&scene, &accel, &camara, cursor, ANCHO, ALTO),
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
}
