//! Cielo del diorama: qué color devuelve un rayo que no toca nada.
//!
//! No es un objeto de la escena. No tiene geometría, no se intersecta y no
//! aparece en el presupuesto de 160 primitivas: es una función de la
//! **dirección** del rayo, evaluada solo cuando el recorrido falla todo lo
//! demás. Por eso también es el terminal correcto para un rayo que agota
//! `max_depth` en el Hito 5 —la decisión cerrada del plan— y no hay que
//! inventarle un color aparte.
//!
//! # Convención equirectangular
//!
//! Los dos panoramas se generan en `generate_assets` con esta convención, y
//! aquí se muestrea con la misma o el cielo saldría desplazado:
//!
//! | Coordenada | Significado |
//! |---|---|
//! | `u = 0.0` | azimut cero, mirando hacia `+X` |
//! | `u` creciente | gira de `+X` hacia `+Z`, el mismo sentido que el yaw |
//! | `v = 0.0` | nadir, recto hacia abajo |
//! | `v = 0.5` | horizonte |
//! | `v = 1.0` | cenit, recto hacia arriba |
//!
//! Que `u` siga el sentido del yaw de la cámara no es un detalle estético:
//! `eye_at` coloca el ojo en `(cos θ, ·, sin θ)`, así que un yaw de la
//! cámara se traduce directo a una franja del panorama y se puede razonar
//! sobre qué parte del cielo se está viendo. Ojo con el sentido de lectura:
//! `u` describe **hacia dónde viaja el rayo**, no dónde está la cámara, y
//! esas dos direcciones son opuestas. La cámara a yaw `θ` mira el cielo de
//! `u = (θ + 180°) / 360°`.

use crate::color::Color;
use crate::reveal::RevealState;
use crate::scene::{Scene, TextureId};
use nalgebra_glm::{Vec2, Vec3};
use std::f32::consts::{PI, TAU};

/// Color del cielo cuando la escena no tiene panoramas cargados.
///
/// Es el caso de los tests y de los presets sin texturas: un azul de noche
/// muy oscuro, que deja ver la silueta de la geometría sin competir con
/// ella. No es un color de relleno que oculte un asset ausente —cargar un
/// panorama que falta da error con su ruta—, sino el cielo declarado de una
/// escena que se construyó sin él a propósito.
pub const FALLBACK_COLOR: u32 = 0x040C24;

/// El cenit exacto (`v = 1.0`) no se puede muestrear tal cual.
///
/// Los panoramas envuelven en horizontal, así que su `WrapMode` es
/// `Repeat`; y bajo `Repeat`, `v = 1.0` envuelve a `0.0`. Un rayo que mira
/// recto hacia arriba tomaría el color del nadir: exactamente el opuesto.
/// Se recorta al último valor que sigue cayendo en la fila superior.
const CENIT_MUESTREABLE: f32 = 1.0 - f32::EPSILON;

/// Cómo se resuelve el cielo de una escena.
///
/// Las dos variantes existen porque el proyecto tiene dos modos legítimos
/// de correr: con los assets del Hito 4 cargados, y sin ellos. Modelarlo
/// como enum en vez de `Option<Skybox>` deja una sola ruta en el renderer
/// —el miss siempre pregunta al cielo— y ningún `if` de caso especial en el
/// camino caliente.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Skybox {
    /// Color plano. Es lo que usan los tests y los presets sin texturas.
    Flat(Color),
    /// Los dos panoramas equirectangulares, interpolados por el progreso
    /// global de la revelación: el cielo se pinta con el Continente.
    Panorama {
        /// Cielo sin pintar, el fondo del estado inicial.
        pale: TextureId,
        /// Cielo pintado, el fondo del estado final.
        painted: TextureId,
    },
}

impl Default for Skybox {
    fn default() -> Self {
        Skybox::Flat(Color::from_hex(FALLBACK_COLOR))
    }
}

/// Dirección de rayo a UV equirectangular.
///
/// Devuelve `u` en `0.0..1.0` y `v` en `0.0..=1.0`, según la convención del
/// encabezado del módulo. La dirección no necesita venir normalizada: se
/// normaliza aquí, porque un rayo reflejado o refractado puede llegar con
/// la magnitud algo desviada y el azimut es indiferente a la escala pero la
/// altura no.
pub fn direction_to_uv(direction: &Vec3) -> Vec2 {
    let largo = direction.magnitude();

    // Dirección degenerada —el vector cero, o un NaN heredado de una
    // operación anterior—. Devolver el horizonte al frente da un color
    // plausible; devolver un NaN indexaría la textura fuera de rango.
    if !largo.is_finite() || largo <= f32::EPSILON {
        return Vec2::new(0.0, 0.5);
    }

    let d = *direction / largo;

    // `rem_euclid` y no `%`: el azimut de una dirección con `z` negativa es
    // negativo, y con el resto de Rust se quedaría fuera de rango.
    let u = (d.z.atan2(d.x) / TAU).rem_euclid(1.0);

    // El recorte protege el `asin`: normalizar puede dejar la componente en
    // `1.0000001` por redondeo, y ahí `asin` devuelve NaN.
    let v = 0.5 + d.y.clamp(-1.0, 1.0).asin() / PI;

    Vec2::new(u, v)
}

impl Skybox {
    /// Color del cielo en la dirección dada.
    ///
    /// La escena llega como parámetro porque los panoramas viven en su
    /// tabla de texturas, igual que las de material: el cielo guarda dos
    /// índices y no dos imágenes de medio megabyte.
    pub fn sample(&self, scene: &Scene, direction: &Vec3, reveal: &RevealState) -> Color {
        match self {
            Skybox::Flat(color) => *color,
            Skybox::Panorama { pale, painted } => {
                let uv = direction_to_uv(direction);
                let (u, v) = (uv.x, uv.y.min(CENIT_MUESTREABLE));
                let t = reveal.global_progress();

                // Los dos extremos se atajan: son el caso común y se
                // ahorran un muestreo de panorama por rayo perdido, que en
                // este diorama son decenas de miles por cuadro.
                if t <= 0.0 {
                    scene.texture(*pale).sample(u, v)
                } else if t >= 1.0 {
                    scene.texture(*painted).sample(u, v)
                } else {
                    let sin_pintar = scene.texture(*pale).sample(u, v);
                    let pintado = scene.texture(*painted).sample(u, v);

                    // En lineal, como el resto del pipeline: es donde un
                    // punto medio se lee como un punto medio.
                    sin_pintar * (1.0 - t) + pintado * t
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::RevealGroup;
    use crate::scene_builder::eye_at_yaw;
    use crate::texture::Texture;

    fn assert_cerca(obtenido: f32, esperado: f32, que: &str) {
        assert!(
            (obtenido - esperado).abs() < 1e-5,
            "{que}: {obtenido} en vez de {esperado}"
        );
    }

    /// Panorama de dos por dos con un color por cuadrante, en orden de
    /// lectura: fila de arriba primero.
    ///
    /// Con `2 x 2` las dos mitades verticales son cielo y suelo, y las dos
    /// horizontales separan el azimut en dos.
    fn panorama_de_prueba(arriba: (Color, Color), abajo: (Color, Color)) -> Texture {
        Texture::from_pixels(2, 2, vec![arriba.0, arriba.1, abajo.0, abajo.1])
            .expect("2x2 con cuatro pixeles")
    }

    fn escena_con_panoramas() -> Scene {
        let mut scene = Scene::new();

        // Sin pintar: cielo blanco arriba, gris abajo.
        let pale = scene.add_texture(panorama_de_prueba(
            (Color::new(1.0, 1.0, 1.0), Color::new(1.0, 1.0, 1.0)),
            (Color::new(0.5, 0.5, 0.5), Color::new(0.5, 0.5, 0.5)),
        ));
        // Pintado: negro arriba, negro abajo. La distancia entre los dos
        // panoramas es lo que hace medible la interpolacion.
        let painted = scene.add_texture(panorama_de_prueba(
            (Color::black(), Color::black()),
            (Color::black(), Color::black()),
        ));

        scene.skybox = Skybox::Panorama { pale, painted };

        scene
    }

    fn progreso(valor: f32) -> RevealState {
        let mut reveal = RevealState::unpainted();
        for grupo in [
            RevealGroup::Meadows,
            RevealGroup::Breakwater,
            RevealGroup::FlyingWaters,
            RevealGroup::Finale,
        ] {
            reveal.set_progress(grupo, valor);
        }

        reveal
    }

    #[test]
    fn las_direcciones_cardinales_dan_la_uv_esperada() {
        // Azimut: u = 0 mirando a +X, y crece girando hacia +Z.
        for (direccion, u_esperada, nombre) in [
            (Vec3::new(1.0, 0.0, 0.0), 0.00, "+X"),
            (Vec3::new(0.0, 0.0, 1.0), 0.25, "+Z"),
            (Vec3::new(-1.0, 0.0, 0.0), 0.50, "-X"),
            (Vec3::new(0.0, 0.0, -1.0), 0.75, "-Z"),
        ] {
            let uv = direction_to_uv(&direccion);

            assert_cerca(uv.x, u_esperada, nombre);
            // Las cuatro estan en el horizonte.
            assert_cerca(uv.y, 0.5, nombre);
        }
    }

    #[test]
    fn el_cenit_y_el_nadir_estan_en_los_extremos_de_v() {
        let cenit = direction_to_uv(&Vec3::new(0.0, 1.0, 0.0));
        let nadir = direction_to_uv(&Vec3::new(0.0, -1.0, 0.0));

        assert_cerca(cenit.y, 1.0, "cenit");
        assert_cerca(nadir.y, 0.0, "nadir");
    }

    #[test]
    fn las_diagonales_a_45_grados_caen_a_mitad_de_camino() {
        // Elevacion de 45 grados: tanto altura como avance horizontal, y
        // `v` a mitad entre horizonte y cenit. Ojo con el vector: es la
        // altura contra la **magnitud horizontal**, no contra una sola
        // componente ya reducida.
        let arriba = direction_to_uv(&Vec3::new(1.0, 1.0, 0.0));
        assert_cerca(arriba.y, 0.75, "45 grados arriba");

        // Y hacia abajo, a mitad entre horizonte y nadir.
        let abajo = direction_to_uv(&Vec3::new(1.0, -1.0, 0.0));
        assert_cerca(abajo.y, 0.25, "45 grados abajo");

        // La misma comprobacion reparte el avance horizontal en dos ejes:
        // lo que decide `v` es la magnitud, no de que eje venga.
        let repartida = direction_to_uv(&Vec3::new(0.5_f32.sqrt(), 1.0, 0.5_f32.sqrt()));
        assert_cerca(repartida.y, 0.75, "45 grados en diagonal");

        // Azimut a 45 grados entre +X y +Z.
        let diagonal = direction_to_uv(&Vec3::new(1.0, 0.0, 1.0));
        assert_cerca(diagonal.x, 0.125, "azimut diagonal");
    }

    #[test]
    fn el_azimut_sigue_el_mismo_sentido_que_el_yaw_de_la_camara() {
        // La razon de elegir este sentido: un yaw de camara se traduce
        // directo a una franja del panorama. `eye_at_yaw` pone el ojo en
        // (cos, ., sin), asi que la direccion del centro al ojo tiene
        // azimut igual al yaw.
        let centro = Vec3::zeros();

        for yaw in [0.0_f32, 90.0, 180.0, 270.0, 35.0] {
            let ojo = eye_at_yaw(centro, 10.0, yaw);
            let uv = direction_to_uv(&(ojo - centro));

            assert_cerca(uv.x, yaw / 360.0, "yaw");
        }
    }

    #[test]
    fn la_uv_se_queda_en_rango_para_cualquier_direccion() {
        // Barrido sobre la esfera: ninguna direccion debe salirse, o el
        // muestreo indexaria la textura fuera de rango.
        for i in 0..37 {
            for j in 0..19 {
                let azimut = i as f32 / 36.0 * TAU;
                let altura = (j as f32 / 18.0 - 0.5) * PI;

                let direccion = Vec3::new(
                    altura.cos() * azimut.cos(),
                    altura.sin(),
                    altura.cos() * azimut.sin(),
                );
                let uv = direction_to_uv(&direccion);

                assert!((0.0..1.0).contains(&uv.x), "u fuera de rango: {}", uv.x);
                assert!((0.0..=1.0).contains(&uv.y), "v fuera de rango: {}", uv.y);
            }
        }
    }

    #[test]
    fn la_costura_del_azimut_es_continua() {
        // La junta esta en +X, donde u vuelve de 1 a 0. A un lado y otro el
        // panorama tiene que dar practicamente lo mismo, o se veria una
        // linea vertical en el cielo.
        let epsilon = 1e-3_f32;

        let antes = direction_to_uv(&Vec3::new(epsilon.cos(), 0.0, -epsilon.sin()));
        let despues = direction_to_uv(&Vec3::new(epsilon.cos(), 0.0, epsilon.sin()));

        assert!(antes.x > 0.999, "justo antes de la junta: {}", antes.x);
        assert!(despues.x < 0.001, "justo despues: {}", despues.x);
    }

    #[test]
    fn una_direccion_degenerada_devuelve_el_horizonte() {
        for degenerada in [
            Vec3::zeros(),
            Vec3::new(f32::NAN, 0.0, 0.0),
            Vec3::new(f32::INFINITY, 1.0, 0.0),
        ] {
            let uv = direction_to_uv(&degenerada);

            assert!(uv.x.is_finite() && uv.y.is_finite(), "UV no finita");
            assert_cerca(uv.y, 0.5, "horizonte");
        }
    }

    #[test]
    fn el_cenit_no_envuelve_al_nadir() {
        // El fallo que este test existe para atrapar: `v = 1.0` exacto bajo
        // `WrapMode::Repeat` envuelve a `0.0`, y un rayo que mira recto
        // hacia arriba tomaria el color del suelo.
        let mut scene = Scene::new();
        let blanco = Color::new(1.0, 1.0, 1.0);
        let negro = Color::black();

        let panorama = scene.add_texture(panorama_de_prueba(
            (blanco, blanco), // cielo
            (negro, negro),   // suelo
        ));
        scene.skybox = Skybox::Panorama {
            pale: panorama,
            painted: panorama,
        };

        let cenit = scene
            .skybox
            .sample(&scene, &Vec3::new(0.0, 1.0, 0.0), &progreso(0.0));
        let nadir = scene
            .skybox
            .sample(&scene, &Vec3::new(0.0, -1.0, 0.0), &progreso(0.0));

        assert_eq!(cenit, blanco, "el cenit tomo el color del suelo");
        assert_eq!(nadir, negro, "el nadir tomo el color del cielo");
    }

    #[test]
    fn sin_panoramas_el_cielo_es_el_color_de_respaldo() {
        let scene = Scene::new();

        assert_eq!(scene.skybox, Skybox::default());

        let color = scene
            .skybox
            .sample(&scene, &Vec3::new(0.0, 0.0, 1.0), &progreso(0.5));

        assert_eq!(color.to_hex(), FALLBACK_COLOR);
    }

    #[test]
    fn un_cielo_plano_ignora_la_direccion_y_el_progreso() {
        let scene = Scene::new();
        let plano = Skybox::Flat(Color::new(0.2, 0.4, 0.6));

        for direccion in [
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        ] {
            for t in [0.0, 0.5, 1.0] {
                assert_eq!(
                    plano.sample(&scene, &direccion, &progreso(t)),
                    Color::new(0.2, 0.4, 0.6)
                );
            }
        }
    }

    #[test]
    fn el_progreso_global_interpola_entre_los_dos_panoramas() {
        let scene = escena_con_panoramas();
        let cenit = Vec3::new(0.0, 1.0, 0.0);

        // Sin pintar: el panorama pale, que arriba es blanco.
        let inicial = scene.skybox.sample(&scene, &cenit, &progreso(0.0));
        assert_eq!(inicial, Color::new(1.0, 1.0, 1.0));

        // Pintado: el painted, que es negro.
        let final_ = scene.skybox.sample(&scene, &cenit, &progreso(1.0));
        assert_eq!(final_, Color::black());

        // El punto medio, a medio camino.
        let medio = scene.skybox.sample(&scene, &cenit, &progreso(0.5));
        assert_cerca(medio.r, 0.5, "punto medio");
    }

    #[test]
    fn la_interpolacion_del_cielo_es_monotona_y_acotada() {
        let scene = escena_con_panoramas();
        let cenit = Vec3::new(0.0, 1.0, 0.0);
        let mut anterior = f32::INFINITY;

        for paso in 0..=20 {
            let t = paso as f32 / 20.0;
            let color = scene.skybox.sample(&scene, &cenit, &progreso(t));

            assert!((0.0..=1.0).contains(&color.r), "fuera de rango en t = {t}");
            assert!(
                color.r <= anterior + 1e-6,
                "no decrecio de forma monotona en t = {t}"
            );
            anterior = color.r;
        }
    }

    #[test]
    fn el_cielo_avanza_con_el_progreso_global_y_no_con_un_solo_grupo() {
        // Pintar una sola region mueve el cielo un cuarto de camino: es lo
        // que hace que el fondo acompane al diorama en vez de saltar al
        // final de golpe.
        let scene = escena_con_panoramas();
        let cenit = Vec3::new(0.0, 1.0, 0.0);

        let mut una = RevealState::unpainted();
        una.set_progress(RevealGroup::Meadows, 1.0);

        let color = scene.skybox.sample(&scene, &cenit, &una);

        assert_cerca(color.r, 0.75, "una de cuatro regiones pintada");
    }
}
