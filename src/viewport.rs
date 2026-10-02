//! Dónde cae el cuadro dentro de una ventana redimensionable, y el camino
//! de vuelta del puntero.
//!
//! La ventana abre a `1200 x 900` y el usuario la puede estirar, pero el
//! cuadro se sigue trazando a `800 x 600` —o al perfil interactivo mientras
//! algo se mueve— y `minifb` lo escala al presentarlo con
//! `ScaleMode::AspectRatioStretch`: conserva `4 : 3` y rellena el sobrante
//! con bandas. Agrandar la ventana no cuesta un rayo más.
//!
//! El problema es el puntero. `minifb 0.26` entrega `get_mouse_pos` en
//! píxeles **de cliente de la ventana**, no del buffer —su backend de
//! Windows lo dice en `// TODO: Needs to be fixed with resize support`—, así
//! que con la ventana estirada un clic caería en otro sitio del diorama, de
//! la paleta o de la bocina. Este módulo replica el encuadre que calcula
//! `minifb` al pintar y lo invierte.
//!
//! # De dónde sale la fórmula
//!
//! De `minifb-0.26.0/src/os/windows/mod.rs`, rama `WM_PAINT`,
//! `ScaleMode::AspectRatioStretch`: los cocientes de aspecto en `f32`, el
//! lado escalado truncado a `i32` y el desplazamiento por división entera
//! `(nuevo - ventana) / -2`. Se copian las mismas operaciones, truncados
//! incluidos, así que el **encuadre** —dónde empieza y acaba la imagen, y
//! dónde están las bandas— es exactamente el de `minifb`. Dentro de él se
//! supone que `StretchDIBits` amplía por réplica (`floor`); si GDI muestreara
//! por centro de píxel, el puntero podría caer a un píxel del buffer del que
//! se ve. Es el backend de Windows, que es donde se entrega la obra; en otro
//! sistema el cálculo de `minifb` podría diferir.

/// Tamaño de cliente con el que abre la ventana: `1.5` veces el cuadro.
pub const VENTANA_INICIAL: (usize, usize) = (1200, 900);

/// Rectángulo, en píxeles de cliente de la ventana, donde se presenta el
/// cuadro. Puede empezar fuera del cliente si la ventana es más estrecha
/// que el cuadro escalado; con `AspectRatioStretch` no ocurre.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Encuadre {
    pub x: i32,
    pub y: i32,
    pub ancho: i32,
    pub alto: i32,
}

/// El encuadre que usa `minifb` para presentar un buffer de `buffer` en una
/// ventana de cliente `ventana`. `None` si alguno de los dos es vacío —una
/// ventana minimizada informa `0 x 0`—.
pub fn encuadre(ventana: (usize, usize), buffer: (usize, usize)) -> Option<Encuadre> {
    if ventana.0 == 0 || ventana.1 == 0 || buffer.0 == 0 || buffer.1 == 0 {
        return None;
    }

    // Mismas operaciones y mismos tipos que `minifb`: ver el módulo.
    let (ancho_buffer, alto_buffer) = (buffer.0 as i32, buffer.1 as i32);
    let (ancho_ventana, alto_ventana) = (ventana.0 as i32, ventana.1 as i32);
    let aspecto_buffer = ancho_buffer as f32 / alto_buffer as f32;
    let aspecto_ventana = ancho_ventana as f32 / alto_ventana as f32;

    let e = if aspecto_buffer > aspecto_ventana {
        let alto = (ancho_ventana as f32 / aspecto_buffer) as i32;
        Encuadre {
            x: 0,
            y: (alto - alto_ventana) / -2,
            ancho: ancho_ventana,
            alto,
        }
    } else {
        let ancho = (alto_ventana as f32 * aspecto_buffer) as i32;
        Encuadre {
            x: (ancho - ancho_ventana) / -2,
            y: 0,
            ancho,
            alto: alto_ventana,
        }
    };

    (e.ancho > 0 && e.alto > 0).then_some(e)
}

/// Del puntero en píxeles de cliente al punto del **buffer** que se ve
/// debajo, en las mismas unidades que esperan `PresentedFrame`, la paleta y
/// la bocina. `None` en las bandas, fuera de la ventana o con un puntero no
/// finito: ahí no se ve el cuadro, así que un clic no apunta a nada de él.
pub fn al_buffer(
    cursor: (f32, f32),
    ventana: (usize, usize),
    buffer: (usize, usize),
) -> Option<(f32, f32)> {
    if !cursor.0.is_finite() || !cursor.1.is_finite() {
        return None;
    }
    if cursor.0 < 0.0
        || cursor.1 < 0.0
        || cursor.0 >= ventana.0 as f32
        || cursor.1 >= ventana.1 as f32
    {
        return None;
    }

    let e = encuadre(ventana, buffer)?;

    // `StretchDIBits` amplía por réplica: el píxel de cliente `d` muestra el
    // píxel del buffer `floor((d - origen) * lado_buffer / lado_encuadre)`.
    // El mapeo continuo da ese mismo píxel al truncarse, y a tamaño natural
    // es la identidad exacta.
    let x = (cursor.0 - e.x as f32) * buffer.0 as f32 / e.ancho as f32;
    let y = (cursor.1 - e.y as f32) * buffer.1 as f32 / e.alto as f32;

    let dentro = x >= 0.0 && y >= 0.0 && x < buffer.0 as f32 && y < buffer.1 as f32;

    dentro.then_some((x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    const CUADRO: (usize, usize) = (800, 600);

    /// La fórmula de `minifb`, escrita aparte y paso a paso como en su
    /// `WM_PAINT`, para comparar contra ella en un barrido de tamaños.
    fn referencia_minifb(ventana: (usize, usize), buffer: (usize, usize)) -> (i32, i32, i32, i32) {
        let (buffer_width, buffer_height) = (buffer.0 as i32, buffer.1 as i32);
        let (window_width, window_height) = (ventana.0 as i32, ventana.1 as i32);
        let mut new_height = window_height;
        let mut new_width = window_width;
        let mut x_offset = 0;
        let mut y_offset = 0;
        let buffer_aspect = buffer_width as f32 / buffer_height as f32;
        let win_aspect = window_width as f32 / window_height as f32;
        if buffer_aspect > win_aspect {
            new_height = (window_width as f32 / buffer_aspect) as i32;
            y_offset = (new_height - window_height) / -2;
        } else {
            new_width = (window_height as f32 * buffer_aspect) as i32;
            x_offset = (new_width - window_width) / -2;
        }
        (x_offset, y_offset, new_width, new_height)
    }

    #[test]
    fn la_ventana_inicial_es_el_cuadro_ampliado_sin_bandas() {
        assert_eq!(VENTANA_INICIAL, (1200, 900));
        assert_eq!(
            encuadre(VENTANA_INICIAL, CUADRO),
            Some(Encuadre {
                x: 0,
                y: 0,
                ancho: 1200,
                alto: 900
            })
        );
    }

    #[test]
    fn a_tamano_natural_el_puntero_no_se_transforma() {
        for cursor in [(0.0, 0.0), (399.0, 299.0), (799.0, 599.0), (123.0, 456.0)] {
            assert_eq!(al_buffer(cursor, CUADRO, CUADRO), Some(cursor));
        }
    }

    #[test]
    fn a_1200x900_centro_y_esquinas_caen_en_su_sitio() {
        let v = VENTANA_INICIAL;
        assert_eq!(al_buffer((600.0, 450.0), v, CUADRO), Some((400.0, 300.0)));
        assert_eq!(al_buffer((0.0, 0.0), v, CUADRO), Some((0.0, 0.0)));
        let (x, y) = al_buffer((1199.0, 899.0), v, CUADRO).unwrap();
        assert!(
            x < 800.0 && y < 600.0 && x >= 799.0 && y >= 599.0,
            "{x} {y}"
        );
        assert_eq!(al_buffer((1200.0, 450.0), v, CUADRO), None);
        assert_eq!(al_buffer((600.0, 900.0), v, CUADRO), None);
        assert_eq!(al_buffer((-1.0, 450.0), v, CUADRO), None);
    }

    /// Ventana más ancha que `4 : 3`: bandas a los lados, como las pinta
    /// `minifb`, y el puntero sobre ellas no señala nada.
    #[test]
    fn una_ventana_ancha_tiene_bandas_laterales_que_no_apuntan_a_nada() {
        let v = (1600, 900);
        let e = encuadre(v, CUADRO).unwrap();
        assert_eq!(
            e,
            Encuadre {
                x: 200,
                y: 0,
                ancho: 1200,
                alto: 900
            }
        );
        assert_eq!(al_buffer((100.0, 450.0), v, CUADRO), None);
        assert_eq!(al_buffer((199.0, 450.0), v, CUADRO), None);
        assert_eq!(al_buffer((200.0, 0.0), v, CUADRO), Some((0.0, 0.0)));
        assert_eq!(al_buffer((800.0, 450.0), v, CUADRO), Some((400.0, 300.0)));
        assert!(al_buffer((1399.0, 899.0), v, CUADRO).is_some());
        assert_eq!(al_buffer((1400.0, 450.0), v, CUADRO), None);
    }

    /// Ventana más alta: bandas arriba y abajo. `800 / 1.3333334` en `f32`
    /// redondea a `600.0`, así que aquí el truncado de `minifb` no recorta.
    #[test]
    fn una_ventana_alta_tiene_bandas_arriba_y_abajo() {
        let v = (800, 900);
        let e = encuadre(v, CUADRO).unwrap();
        assert_eq!(
            e,
            Encuadre {
                x: 0,
                y: 150,
                ancho: 800,
                alto: 600
            }
        );
        assert_eq!(al_buffer((400.0, 149.0), v, CUADRO), None);
        assert_eq!(al_buffer((400.0, 150.0), v, CUADRO), Some((400.0, 0.0)));
        assert_eq!(al_buffer((400.0, 749.0), v, CUADRO), Some((400.0, 599.0)));
        assert_eq!(al_buffer((400.0, 750.0), v, CUADRO), None);
    }

    /// Un tamaño donde el truncado sí recorta: `1000 / 1.3333334` da
    /// `749.99994` y `minifb` se queda con `749`.
    #[test]
    fn el_truncado_de_minifb_se_respeta_cuando_recorta() {
        let v = (1000, 1000);
        let e = encuadre(v, CUADRO).unwrap();
        assert_eq!((1000.0f32 / (800.0f32 / 600.0f32)) as i32, e.alto);
        assert_eq!(e.x, 0);
        assert_eq!(e.y, (e.alto - 1000) / -2);
        assert_eq!(al_buffer((500.0, (e.y - 1) as f32), v, CUADRO), None);
        assert!(al_buffer((500.0, (e.y + e.alto - 1) as f32), v, CUADRO).is_some());
        assert_eq!(al_buffer((500.0, (e.y + e.alto) as f32), v, CUADRO), None);
    }

    #[test]
    fn una_ventana_minimizada_o_un_puntero_no_finito_no_apuntan_a_nada() {
        assert_eq!(encuadre((0, 0), CUADRO), None);
        assert_eq!(encuadre((1200, 0), CUADRO), None);
        assert_eq!(encuadre(VENTANA_INICIAL, (0, 600)), None);
        assert_eq!(al_buffer((10.0, 10.0), (0, 0), CUADRO), None);
        assert_eq!(al_buffer((f32::NAN, 10.0), VENTANA_INICIAL, CUADRO), None);
        assert_eq!(
            al_buffer((10.0, f32::INFINITY), VENTANA_INICIAL, CUADRO),
            None
        );
    }

    /// Barrido: el encuadre es el de `minifb` en cualquier tamaño, y todo
    /// píxel de cliente dentro de él cae en un píxel del buffer; todo píxel
    /// fuera, en ninguno. Es la garantía de que la imagen y el puntero no se
    /// separan al redimensionar.
    #[test]
    fn en_cualquier_tamano_el_encuadre_es_el_de_minifb_y_el_puntero_lo_respeta() {
        let mut tamanos = Vec::new();
        for ancho in (1..=2000).step_by(37) {
            for alto in (1..=1500).step_by(41) {
                tamanos.push((ancho, alto));
            }
        }
        tamanos.extend([
            (801, 600),
            (800, 601),
            (1201, 900),
            (1199, 900),
            (1, 1),
            (4, 3),
        ]);
        for v in tamanos {
            let (x, y, w, h) = referencia_minifb(v, CUADRO);
            let e = encuadre(v, CUADRO);
            if w <= 0 || h <= 0 {
                assert_eq!(e, None, "{v:?}");
                continue;
            }
            assert_eq!(
                e,
                Some(Encuadre {
                    x,
                    y,
                    ancho: w,
                    alto: h
                }),
                "{v:?}"
            );
            // Esquinas del encuadre, justo dentro y justo fuera.
            let dentro = [
                (x, y),
                (x + w - 1, y),
                (x, y + h - 1),
                (x + w - 1, y + h - 1),
            ];
            for (cx, cy) in dentro {
                if cx < 0 || cy < 0 || cx >= v.0 as i32 || cy >= v.1 as i32 {
                    continue;
                }
                let p = al_buffer((cx as f32, cy as f32), v, CUADRO)
                    .unwrap_or_else(|| panic!("{v:?} {cx} {cy}"));
                assert!(
                    p.0 >= 0.0 && p.0 < 800.0 && p.1 >= 0.0 && p.1 < 600.0,
                    "{v:?} {p:?}"
                );
            }
            for (cx, cy) in [(x - 1, y), (x, y - 1), (x + w, y), (x, y + h)] {
                assert_eq!(
                    al_buffer((cx as f32, cy as f32), v, CUADRO),
                    None,
                    "{v:?} {cx} {cy}"
                );
            }
        }
    }

    /// El mapeo es monótono y proporcional: el centro del encuadre cae en
    /// el centro del cuadro en todo tamaño con imagen de al menos `4 x 3`.
    #[test]
    fn el_centro_del_encuadre_es_el_centro_del_cuadro() {
        for v in [
            (1200, 900),
            (1600, 900),
            (800, 900),
            (1000, 1000),
            (1920, 1017),
            (640, 480),
        ] {
            let e = encuadre(v, CUADRO).unwrap();
            let c = (
                e.x as f32 + e.ancho as f32 / 2.0,
                e.y as f32 + e.alto as f32 / 2.0,
            );
            let p = al_buffer(c, v, CUADRO).unwrap();
            assert!(
                (p.0 - 400.0).abs() < 1e-3 && (p.1 - 300.0).abs() < 1e-3,
                "{v:?} {p:?}"
            );
        }
    }

    /// La cadena completa del clic en la ventana estirada: del cliente al
    /// buffer con `al_buffer`, y del buffer al píxel **fuente** con
    /// `PresentedFrame`, en los dos perfiles —el cuadro final a `800 x 600`
    /// y el interactivo de la ventana—. El píxel fuente es el que la imagen
    /// muestra bajo el puntero.
    #[test]
    fn el_picking_escalado_llega_al_pixel_fuente_en_los_dos_perfiles() {
        use crate::camera::Camera;
        use crate::input::PresentedFrame;
        use crate::renderer::InteractiveProfile;
        use nalgebra_glm::Vec3;

        let camara = Camera::new(
            Vec3::new(0.0, 5.0, 20.0),
            Vec3::zeros(),
            Vec3::zeros(),
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
        );
        let perfil = InteractiveProfile::default();
        let final_ = PresentedFrame::full(camara, CUADRO);
        let interactivo = PresentedFrame {
            camera: camara,
            source: (perfil.width, perfil.height),
            window: CUADRO,
        };
        for v in [
            (1200, 900),
            (1600, 900),
            (800, 900),
            (1000, 1000),
            (800, 600),
        ] {
            let e = encuadre(v, CUADRO).unwrap();
            for (fx, fy) in [(0.0, 0.0), (0.5, 0.5), (0.999, 0.999), (0.25, 0.75)] {
                // El píxel de cliente que cae en esa fracción de la imagen.
                let cliente = (
                    (e.x as f32 + fx * e.ancho as f32).floor(),
                    (e.y as f32 + fy * e.alto as f32).floor(),
                );
                let b = al_buffer(cliente, v, CUADRO).unwrap();
                for (cuadro, (sw, sh)) in [
                    (final_, CUADRO),
                    (interactivo, (perfil.width, perfil.height)),
                ] {
                    let (px, py) = cuadro.source_pixel_at(b).unwrap();
                    // Mismo píxel fuente que el de la fracción del cliente.
                    let esperado_x =
                        ((cliente.0 - e.x as f32) * sw as f32 / e.ancho as f32) as usize;
                    let esperado_y =
                        ((cliente.1 - e.y as f32) * sh as f32 / e.alto as f32) as usize;
                    assert!(
                        px == esperado_x && py == esperado_y,
                        "{v:?} {cliente:?} {px} {py} {esperado_x} {esperado_y}"
                    );
                    assert!(cuadro.ray_under_cursor(b).is_some());
                }
            }
            // En las bandas no hay rayo: el clic no llega al picking.
            if e.x > 0 {
                assert_eq!(al_buffer((0.0, 10.0), v, CUADRO), None);
            }
        }
    }
}
