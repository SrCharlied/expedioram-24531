//! Óptica de los rayos secundarios: reflexión, refracción y de qué lado
//! sale cada uno.
//!
//! Solo geometría de direcciones. El reparto de energía —Schlick, `kr`,
//! `kt`, `kl`— llega en la Tarea 5.2, y la recursión en la 5.3. Separarlo
//! así deja este módulo comprobable contra la ley de Snell sin escena, sin
//! materiales y sin luces.
//!
//! # Convención de vectores
//!
//! Las tres funciones comparten la misma, y es la que trae `Hit`:
//!
//! - `incident` **apunta hacia la superficie**, en el sentido en que viaja
//!   el rayo. No es el vector «hacia el ojo».
//! - `normal` está orientada **contra** el rayo incidente, es decir hacia
//!   el lado desde el que se golpeó. `Hit::new` ya la voltea y recuerda en
//!   `front_face` si tuvo que hacerlo.
//!
//! Con esa convención `dot(incident, normal)` es siempre negativo, y el
//! rayo reflejado sale del lado de `+normal` mientras el refractado sale
//! del lado de `−normal`. Toda la aritmética de abajo depende de eso.

use crate::hit::Hit;
use crate::ray::Ray;
use crate::EPSILON;
use nalgebra_glm::{dot, Vec3};

/// Índice de refracción del aire. El diorama flota en vacío, así que todo
/// lo que no es un dieléctrico se trata como aire.
pub const IOR_AIRE: f32 = 1.0;

/// Dirección reflejada especularmente.
///
/// `d − 2 (d · n) n`: se invierte la componente del rayo paralela a la
/// normal y se conserva la tangencial. De ahí sale, sin más, que el ángulo
/// de salida iguale al de entrada.
///
/// Con `incident` unitario el resultado también lo es. No se normaliza a la
/// salida a propósito: normalizar por si acaso escondería un llamador que
/// pasa una dirección sin normalizar, y ese error hay que verlo.
pub fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - normal * (2.0 * dot(incident, normal))
}

/// Razón de índices `n_incidente / n_transmitido` para este impacto.
///
/// Es el único lugar donde el lado importa, y por eso está separado: el
/// mismo material da dos razones distintas según se entre o se salga, y
/// equivocarla no rompe nada visiblemente —el rayo simplemente se desvía
/// hacia el lado contrario— hasta que alguien mira el agua de cerca.
///
/// Al entrar, el rayo viene del aire y va al medio: `1 / ior`. Al salir es
/// al revés: `ior / 1`.
pub fn eta_for(front_face: bool, ior: f32) -> f32 {
    if front_face {
        IOR_AIRE / ior
    } else {
        ior / IOR_AIRE
    }
}

/// Dirección refractada, o `None` si hay **reflexión total interna**.
///
/// `eta` es `n_incidente / n_transmitido`, lo que devuelve `eta_for`.
///
/// De la ley de Snell, `sin θt = eta · sin θi`. Cuando ese seno pasaría de
/// uno no existe ángulo transmitido: el rayo no puede salir del medio y se
/// refleja entero hacia adentro. Devolver `None` en vez de un vector
/// cualquiera obliga a quien llama a decidir qué hace —reflejar— en vez de
/// arrastrar un `NaN` desde la raíz cuadrada de un número negativo.
///
/// Solo ocurre saliendo de un medio más denso: con `eta < 1` el seno
/// transmitido siempre cabe.
pub fn refract(incident: &Vec3, normal: &Vec3, eta: f32) -> Option<Vec3> {
    // Positivo por la convención del módulo: la normal mira hacia el rayo.
    let cos_entrada = -dot(incident, normal);

    // `sin² θt = eta² · sin² θi`, con `sin² θi = 1 − cos² θi`.
    let sin2_salida = eta * eta * (1.0 - cos_entrada * cos_entrada);

    if sin2_salida > 1.0 {
        return None;
    }

    let cos_salida = (1.0 - sin2_salida).sqrt();

    Some(incident * eta + normal * (eta * cos_entrada - cos_salida))
}

/// Ángulo crítico de un medio, en grados, o `None` si no tiene.
///
/// El ángulo desde el que un rayo que **sale** del medio ya no puede
/// hacerlo. Existe solo para `ior > 1`; el aire no tiene ángulo crítico
/// porque nunca es el medio más denso de los dos.
///
/// No lo usa el renderer: está para que la calibración de Aguas Voladoras
/// pueda razonar sobre qué parte de la superficie va a actuar como espejo.
pub fn critical_angle_degrees(ior: f32) -> Option<f32> {
    if ior <= IOR_AIRE {
        return None;
    }

    Some((IOR_AIRE / ior).asin().to_degrees())
}

/// Rayo reflejado, ya despegado de la superficie.
///
/// Sale por el lado de `+normal`, el mismo por el que entró, así que el
/// origen se desplaza en ese sentido. Sin ese desplazamiento el rayo vuelve
/// a impactar el punto del que sale por error de redondeo: el mismo acné
/// que ya obligó a desplazar los rayos de sombra.
pub fn reflected_ray(hit: &Hit, incident: &Vec3) -> Ray {
    Ray::new(
        hit.point + hit.normal * EPSILON,
        reflect(incident, &hit.normal),
    )
}

/// Rayo refractado, ya despegado de la superficie, o `None` en reflexión
/// total interna.
///
/// Aquí está la asimetría que importa: el refractado **cruza** la
/// superficie, así que su origen se desplaza hacia `−normal` y no hacia
/// `+normal`. Desplazarlo del lado equivocado lo deja del lado de entrada,
/// donde vuelve a intersectar la misma cara de inmediato; en el volumen
/// cerrado de Aguas eso se ve como una superficie que no deja pasar nada.
pub fn refracted_ray(hit: &Hit, incident: &Vec3, ior: f32) -> Option<Ray> {
    let direction = refract(incident, &hit.normal, eta_for(hit.front_face, ior))?;

    Some(Ray::new(hit.point - hit.normal * EPSILON, direction))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra_glm::Vec2;

    /// Índice del agua y del cristal pictórico, según el inventario.
    const IOR_AGUA: f32 = 1.333;

    fn arriba() -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }

    fn assert_vec_cerca(obtenido: Vec3, esperado: Vec3, que: &str) {
        let delta = (obtenido - esperado).magnitude();
        assert!(
            delta < 1e-5,
            "{que}: {obtenido:?} en vez de {esperado:?} (delta {delta})"
        );
    }

    /// Dirección que baja hacia el plano `y = 0` con el ángulo dado
    /// respecto de la normal vertical.
    fn incidencia(grados: f32) -> Vec3 {
        let t = grados.to_radians();

        Vec3::new(t.sin(), -t.cos(), 0.0)
    }

    /// Ángulo entre una dirección y la normal, en grados.
    fn angulo_con_la_normal(direccion: &Vec3, normal: &Vec3) -> f32 {
        dot(&direccion.normalize(), normal)
            .abs()
            .acos()
            .to_degrees()
    }

    /// Impacto sintético sobre el plano `y = 0`, golpeado desde arriba.
    fn impacto_desde_arriba(direccion: Vec3) -> Hit {
        let ray = Ray::new(Vec3::new(0.0, 1.0, 0.0), direccion);

        Hit::new(&ray, 1.0 / direccion.y.abs(), arriba(), Vec2::zeros())
    }

    /// El mismo plano, golpeado desde abajo: es el rayo que sale del agua.
    fn impacto_desde_abajo(direccion: Vec3) -> Hit {
        let ray = Ray::new(Vec3::new(0.0, -1.0, 0.0), direccion);

        Hit::new(&ray, 1.0 / direccion.y.abs(), arriba(), Vec2::zeros())
    }

    // ------------------------------------------------------- reflexión

    #[test]
    fn la_reflexion_sobre_una_normal_vertical_invierte_lo_vertical() {
        // Cae a 45 grados hacia +X y sube a 45 grados hacia +X: la
        // componente tangencial se conserva, la normal se invierte.
        let raiz = 0.5_f32.sqrt();

        let reflejado = reflect(&Vec3::new(raiz, -raiz, 0.0), &arriba());

        assert_vec_cerca(reflejado, Vec3::new(raiz, raiz, 0.0), "reflexion a 45");
    }

    #[test]
    fn un_rayo_perpendicular_vuelve_por_donde_vino() {
        let reflejado = reflect(&-arriba(), &arriba());

        assert_vec_cerca(reflejado, arriba(), "incidencia normal");
    }

    #[test]
    fn la_reflexion_conserva_el_angulo_y_la_magnitud() {
        for grados in [0.0_f32, 5.0, 15.0, 30.0, 45.0, 60.0, 75.0, 89.0] {
            let entrada = incidencia(grados);
            let salida = reflect(&entrada, &arriba());

            assert!(
                (salida.magnitude() - 1.0).abs() < 1e-5,
                "a {grados} grados la magnitud se fue a {}",
                salida.magnitude()
            );

            let angulo_salida = angulo_con_la_normal(&salida, &arriba());
            assert!(
                (angulo_salida - grados).abs() < 1e-3,
                "a {grados} grados de entrada salio a {angulo_salida}"
            );

            // Y sigue en el plano de incidencia, sin girar de lado.
            assert!(salida.z.abs() < 1e-6, "la reflexion salio del plano");
        }
    }

    // ------------------------------------------------------ refracción

    #[test]
    fn refraccion_aire_agua_acerca_el_rayo_a_la_normal() {
        let eta = eta_for(true, IOR_AGUA);

        for grados in [10.0_f32, 30.0, 45.0, 60.0, 80.0] {
            let entrada = incidencia(grados);
            let salida = refract(&entrada, &arriba(), eta).expect("entrando nunca hay TIR");

            let angulo_salida = angulo_con_la_normal(&salida, &arriba());

            // Se acerca a la normal: el agua es el medio mas denso.
            assert!(
                angulo_salida < grados,
                "a {grados} grados no se acerco: salio a {angulo_salida}"
            );

            // Y lo hace exactamente donde dice Snell.
            let esperado = (grados.to_radians().sin() / IOR_AGUA).asin().to_degrees();
            assert!(
                (angulo_salida - esperado).abs() < 1e-3,
                "Snell pide {esperado} y salio {angulo_salida}"
            );

            // Cruza la superficie: sigue bajando.
            assert!(salida.y < 0.0, "el refractado no cruzo la superficie");
            assert!((salida.magnitude() - 1.0).abs() < 1e-5);
        }
    }

    #[test]
    fn refraccion_agua_aire_aleja_el_rayo_de_la_normal() {
        let eta = eta_for(false, IOR_AGUA);

        // Por debajo del angulo critico, que para el agua ronda los 48.6.
        for grados in [10.0_f32, 30.0, 45.0, 48.0] {
            // Sube hacia la superficie desde dentro del agua.
            let entrada = -incidencia(grados);
            // La normal mira hacia el rayo, o sea hacia abajo.
            let salida = refract(&entrada, &-arriba(), eta).expect("por debajo del critico sale");

            let angulo_salida = angulo_con_la_normal(&salida, &arriba());

            assert!(
                angulo_salida > grados,
                "a {grados} grados no se alejo: salio a {angulo_salida}"
            );

            let esperado = (grados.to_radians().sin() * IOR_AGUA).asin().to_degrees();
            assert!(
                (angulo_salida - esperado).abs() < 1e-3,
                "Snell pide {esperado} y salio {angulo_salida}"
            );

            assert!(salida.y > 0.0, "el refractado no salio del agua");
        }
    }

    #[test]
    fn la_refraccion_perpendicular_no_desvia_el_rayo() {
        for (front, eta) in [
            (true, eta_for(true, IOR_AGUA)),
            (false, eta_for(false, IOR_AGUA)),
        ] {
            let normal = if front { arriba() } else { -arriba() };
            let entrada = -normal;

            let salida = refract(&entrada, &normal, eta).expect("perpendicular siempre pasa");

            assert_vec_cerca(salida, entrada, "incidencia normal");
        }
    }

    #[test]
    fn un_ior_de_uno_no_desvia_nada() {
        // Sin salto de indice no hay refraccion: el rayo sigue derecho.
        for grados in [0.0_f32, 25.0, 60.0, 85.0] {
            let entrada = incidencia(grados);
            let salida = refract(&entrada, &arriba(), eta_for(true, IOR_AIRE))
                .expect("sin salto de indice no hay TIR");

            assert_vec_cerca(salida, entrada, "ior 1.0");
        }
    }

    #[test]
    fn refractar_y_volver_devuelve_la_direccion_original() {
        // Entrar al agua por una cara y salir por otra paralela tiene que
        // devolver la direccion de partida. Es el invariante que atrapa un
        // eta invertido: con la razon al reves el rayo se desvia dos veces
        // hacia el mismo lado en vez de compensarse.
        for grados in [10.0_f32, 30.0, 55.0, 75.0] {
            let original = incidencia(grados);

            let dentro = refract(&original, &arriba(), eta_for(true, IOR_AGUA)).expect("entra");
            // Cara paralela vista desde dentro: la normal mira hacia el
            // rayo, o sea hacia arriba.
            let fuera = refract(&dentro, &arriba(), eta_for(false, IOR_AGUA)).expect("sale");

            assert_vec_cerca(fuera, original, "ida y vuelta");
        }
    }

    // --------------------------------------------- reflexión total interna

    #[test]
    fn la_reflexion_total_interna_ocurre_pasado_el_angulo_critico() {
        let eta = eta_for(false, IOR_AGUA);
        let critico = critical_angle_degrees(IOR_AGUA).expect("el agua tiene critico");

        assert!(
            (critico - 48.607).abs() < 0.01,
            "el critico del agua salio {critico}"
        );

        // Justo por debajo pasa.
        let entrada = -incidencia(critico - 0.5);
        assert!(
            refract(&entrada, &-arriba(), eta).is_some(),
            "por debajo del critico el rayo tiene que salir"
        );

        // Justo por encima, no.
        let entrada = -incidencia(critico + 0.5);
        assert!(
            refract(&entrada, &-arriba(), eta).is_none(),
            "pasado el critico tiene que haber reflexion total"
        );

        // Y sigue sin poder salir en todo el resto del rango.
        for grados in [60.0_f32, 75.0, 89.0] {
            let entrada = -incidencia(grados);

            assert!(
                refract(&entrada, &-arriba(), eta).is_none(),
                "a {grados} grados deberia haber reflexion total"
            );
        }
    }

    #[test]
    fn no_hay_reflexion_total_al_entrar_en_un_medio_mas_denso() {
        // Barrido completo: entrando, el seno transmitido siempre cabe.
        let eta = eta_for(true, IOR_AGUA);

        for paso in 0..=89 {
            let entrada = incidencia(paso as f32);

            assert!(
                refract(&entrada, &arriba(), eta).is_some(),
                "a {paso} grados de entrada no deberia haber TIR"
            );
        }
    }

    #[test]
    fn el_aire_no_tiene_angulo_critico() {
        assert_eq!(critical_angle_degrees(IOR_AIRE), None);
        assert_eq!(critical_angle_degrees(0.5), None);
        assert!(critical_angle_degrees(1.5).is_some());
    }

    // ------------------------------------------------- lado del origen

    #[test]
    fn el_origen_del_reflejado_queda_del_lado_de_entrada() {
        let direccion = incidencia(35.0);
        let hit = impacto_desde_arriba(direccion);

        let rayo = reflected_ray(&hit, &direccion);

        // El plano es `y = 0` y se golpeo desde arriba: el reflejado tiene
        // que arrancar por encima y subir.
        assert!(
            rayo.origin.y > 0.0,
            "el origen quedo dentro: {}",
            rayo.origin
        );
        assert!((rayo.origin.y - EPSILON).abs() < 1e-6);
        assert!(rayo.direction.y > 0.0, "el reflejado no sube");
    }

    #[test]
    fn el_origen_del_refractado_queda_del_otro_lado() {
        let direccion = incidencia(35.0);
        let hit = impacto_desde_arriba(direccion);

        let rayo = refracted_ray(&hit, &direccion, IOR_AGUA).expect("entrando pasa");

        // Cruzo la superficie: arranca por debajo y sigue bajando.
        assert!(rayo.origin.y < 0.0, "el origen quedo del lado de entrada");
        assert!((rayo.origin.y + EPSILON).abs() < 1e-6);
        assert!(rayo.direction.y < 0.0, "el refractado no cruzo");
    }

    #[test]
    fn los_dos_secundarios_salen_a_lados_opuestos_de_la_superficie() {
        // La propiedad que importa, dicha una sola vez: reflejado y
        // refractado nunca arrancan del mismo lado.
        for grados in [5.0_f32, 40.0, 80.0] {
            let direccion = incidencia(grados);
            let hit = impacto_desde_arriba(direccion);

            let reflejado = reflected_ray(&hit, &direccion);
            let refractado = refracted_ray(&hit, &direccion, IOR_AGUA).expect("entrando pasa");

            assert!(
                reflejado.origin.y * refractado.origin.y < 0.0,
                "a {grados} grados los dos origenes quedaron del mismo lado"
            );
        }
    }

    #[test]
    fn saliendo_del_agua_los_lados_se_invierten() {
        // Mismo plano, golpeado desde abajo. `Hit` voltea la normal, asi
        // que «el lado de entrada» ahora es el de abajo y el epsilon tiene
        // que seguirlo.
        let direccion = -incidencia(20.0);
        let hit = impacto_desde_abajo(direccion);

        assert!(!hit.front_face, "golpeado desde abajo es cara interna");

        let reflejado = reflected_ray(&hit, &direccion);
        let refractado = refracted_ray(&hit, &direccion, IOR_AGUA).expect("20 grados sale");

        assert!(reflejado.origin.y < 0.0, "el reflejado no volvio al agua");
        assert!(reflejado.direction.y < 0.0);

        assert!(refractado.origin.y > 0.0, "el refractado no salio del agua");
        assert!(refractado.direction.y > 0.0);
    }

    #[test]
    fn la_reflexion_total_interna_no_produce_rayo_refractado() {
        // Rasante desde dentro del agua: no hay salida.
        let direccion = -incidencia(70.0);
        let hit = impacto_desde_abajo(direccion);

        assert!(refracted_ray(&hit, &direccion, IOR_AGUA).is_none());

        // Pero el reflejado siempre existe, y es lo que la Tarea 5.3 usará
        // en su lugar: una superficie en TIR se comporta como espejo.
        let reflejado = reflected_ray(&hit, &direccion);
        assert!(reflejado.direction.y < 0.0, "el espejo no devolvio el rayo");
    }

    #[test]
    fn eta_depende_del_lado_por_el_que_se_golpea() {
        let entrando = eta_for(true, IOR_AGUA);
        let saliendo = eta_for(false, IOR_AGUA);

        assert!((entrando - 1.0 / IOR_AGUA).abs() < 1e-6);
        assert!((saliendo - IOR_AGUA).abs() < 1e-6);
        // Son recíprocos: es lo que hace que la ida y vuelta se compense.
        assert!((entrando * saliendo - 1.0).abs() < 1e-6);
    }

    #[test]
    fn el_epsilon_es_el_mismo_que_usan_las_sombras() {
        // Un segundo epsilon divergente es exactamente el defecto que
        // `crate::EPSILON` existe para evitar.
        let direccion = incidencia(45.0);
        let hit = impacto_desde_arriba(direccion);

        let reflejado = reflected_ray(&hit, &direccion);
        let separacion = (reflejado.origin - hit.point).magnitude();

        assert!(
            (separacion - EPSILON).abs() < 1e-7,
            "separacion {separacion}"
        );
    }
}
