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

use crate::camera::Camera;
use crate::ray::Ray;

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
}
