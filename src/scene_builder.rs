//! Anclas, parámetros de escala y la cámara derivada del encuadre.
//!
//! Todo objeto del diorama se expresa respecto de un ancla, no en
//! coordenadas absolutas: eso permite mover una región completa sin
//! recalcular sus piezas una por una.
//!
//! Los parámetros de escala no se eligen, se **miden** sobre la geometría
//! ya construida. `scene_radius` sale de la envolvente, `monolith_height`
//! de la altura real del Monolito, y `orbit_radius` se deriva de ambos.

use crate::camera::Camera;
use crate::scene::Scene;
use nalgebra_glm::Vec3;

/// Elevación del ojo sobre la esfera orbital. **No** es el pitch de la
/// vista: con el encuadre por encima del eje, el pitch resulta menor.
pub const EYE_ELEVATION_DEGREES: f32 = 35.0;

/// Yaw de la toma hero. Encara el borde roto de Aguas Voladoras, que en
/// este blockout mira hacia +Z.
pub const HERO_YAW_DEGREES: f32 = 90.0;

/// Mitad del campo de visión vertical, en grados.
pub const HALF_VERTICAL_FOV_DEGREES: f32 = 30.0;

/// Holgura angular que se le exige al encuadre por encima de lo
/// estrictamente necesario. Sin margen, la esfera envolvente toca
/// exactamente el borde del frame y cualquier ajuste posterior la recorta.
pub const FRAMING_MARGIN_DEGREES: f32 = 2.0;

/// Altura del encuadre sobre la base del Monolito, como fracción de su
/// altura. Mirar a la base deja medio frame lleno de terreno.
pub const LOOK_AT_HEIGHT_FRACTION: f32 = 0.15;

/// Puntos de referencia de la escena. Cada región cuelga del suyo.
#[derive(Debug, Clone, Copy)]
pub struct SceneAnchors {
    pub scene_origin: Vec3,
    pub monolith_base_anchor: Vec3,
    pub orbit_center: Vec3,
    pub look_at: Vec3,
    pub meadows_anchor: Vec3,
    pub breakwater_anchor: Vec3,
    /// Centro de la bahía sobre el plano de la superficie del agua.
    pub flying_waters_anchor: Vec3,
    pub palette_anchor: Vec3,
    pub hero_camera_anchor: Vec3,
    pub broken_edge_anchor: Vec3,
}

/// Parámetros globales de escala. Todos medidos, ninguno inventado.
#[derive(Debug, Clone, Copy)]
pub struct SceneScale {
    /// Radio de la esfera centrada en `orbit_center` que contiene toda la
    /// geometría visible.
    pub scene_radius: f32,
    /// Distancia vertical de la base del Monolito a su punto más alto.
    pub monolith_height: f32,
    /// Altura mundial del plano de la superficie de Aguas Voladoras.
    pub water_surface_y: f32,
    /// Derivado del encuadre por bisección. Ver `derive_orbit_radius`.
    pub orbit_radius: f32,
}

/// Menor radio orbital que mantiene toda la escena dentro del encuadre.
///
/// No es una constante. Un valor fijo falla porque `look_at` está por
/// encima de `orbit_center`: el eje de vista no pasa por el centro de la
/// esfera envolvente, y ese desvío crece con `monolith_height`.
///
/// ```text
/// h     = look_at.y - orbit_center.y
/// alpha = asin(scene_radius / R)            radio angular de la esfera
/// beta  = phi - atan2(R·sin phi - h, R·cos phi)   desvio del eje de vista
///
/// orbit_radius = min R : alpha(R) + beta(R) <= half_fov - margen
/// ```
///
/// Ambos términos decrecen monótonamente con `R`, así que una bisección
/// converge sin casos especiales. El resultado se redondea hacia arriba a
/// dos decimales, nunca hacia abajo: redondear a la baja devolvería un
/// radio que ya no cumple la condición.
pub fn derive_orbit_radius(scene_radius: f32, monolith_height: f32) -> f32 {
    let phi = EYE_ELEVATION_DEGREES.to_radians();
    let objetivo = (HALF_VERTICAL_FOV_DEGREES - FRAMING_MARGIN_DEGREES).to_radians();
    let h = LOOK_AT_HEIGHT_FRACTION * monolith_height;

    let requerido = |radio: f32| -> f32 {
        let alpha = (scene_radius / radio).clamp(-1.0, 1.0).asin();
        let beta = phi - (radio * phi.sin() - h).atan2(radio * phi.cos());

        alpha + beta
    };

    // La cota inferior arranca apenas fuera de la esfera envolvente, donde
    // alpha ya vale casi 90 grados y la condición no se cumple; la superior
    // es holgada de sobra.
    let mut bajo = scene_radius * 1.01;
    let mut alto = scene_radius * 8.0;

    for _ in 0..64 {
        let medio = 0.5 * (bajo + alto);
        if requerido(medio) <= objetivo {
            alto = medio;
        } else {
            bajo = medio;
        }
    }

    (alto * 100.0).ceil() / 100.0
}

/// Radio de la esfera centrada en `centro` que contiene toda la geometría.
///
/// Se mide sobre las esquinas de la envolvente, no sobre los centros de los
/// objetos: un cuboide grande cuyo centro está cerca puede tener una
/// esquina muy lejos.
pub fn measure_scene_radius(scene: &Scene, centro: Vec3) -> f32 {
    let Some(caja) = scene.bounds() else {
        return 0.0;
    };

    let mut maximo: f32 = 0.0;
    for i in 0..8 {
        let esquina = Vec3::new(
            if i & 1 == 0 { caja.min.x } else { caja.max.x },
            if i & 2 == 0 { caja.min.y } else { caja.max.y },
            if i & 4 == 0 { caja.min.z } else { caja.max.z },
        );
        maximo = maximo.max((esquina - centro).magnitude());
    }

    maximo
}

/// Posición del ojo sobre la esfera orbital, para un yaw dado.
pub fn eye_at_yaw(orbit_center: Vec3, orbit_radius: f32, yaw_degrees: f32) -> Vec3 {
    let phi = EYE_ELEVATION_DEGREES.to_radians();
    let theta = yaw_degrees.to_radians();

    let horizontal = orbit_radius * phi.cos();
    let altura = orbit_radius * phi.sin();

    orbit_center + Vec3::new(horizontal * theta.cos(), altura, horizontal * theta.sin())
}

/// El blockout completo: geometría, anclas y escala medida.
pub struct Blockout {
    pub scene: Scene,
    pub anchors: SceneAnchors,
    pub scale: SceneScale,
}

impl Blockout {
    /// Cámara del diorama para un yaw dado, ya con los límites de zoom
    /// atados a la escala medida.
    ///
    /// `min_radius` deja la cámara fuera de la esfera envolvente: acercarse
    /// más la metería dentro de la geometría y empezaría a trazar desde el
    /// interior de las masas.
    pub fn camera_at_yaw(&self, yaw_degrees: f32) -> Camera {
        let eye = eye_at_yaw(
            self.anchors.orbit_center,
            self.scale.orbit_radius,
            yaw_degrees,
        );

        Camera::new(
            eye,
            self.anchors.orbit_center,
            self.anchors.look_at,
            Vec3::new(0.0, 1.0, 0.0),
            (2.0 * HALF_VERTICAL_FOV_DEGREES).to_radians(),
        )
        .with_radius_limits(self.scale.scene_radius * 1.2, self.scale.scene_radius * 4.0)
    }

    /// Toma hero: la que encara el borde roto.
    pub fn hero_camera(&self) -> Camera {
        self.camera_at_yaw(HERO_YAW_DEGREES)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn el_radio_derivado_reproduce_los_valores_del_inventario() {
        // Los dos casos de referencia documentados en el inventario, con
        // scene_radius = 1 para leerlos como multiplos de S.
        assert_eq!(derive_orbit_radius(1.0, 0.5), 2.25);
        assert_eq!(derive_orbit_radius(1.0, 1.0), 2.38);
    }

    #[test]
    fn el_radio_derivado_cumple_la_condicion_de_encuadre() {
        let phi = EYE_ELEVATION_DEGREES.to_radians();
        let objetivo = (HALF_VERTICAL_FOV_DEGREES - FRAMING_MARGIN_DEGREES).to_radians();

        for altura in [0.2_f32, 0.5, 1.0, 1.5, 2.0] {
            let radio = derive_orbit_radius(1.0, altura);
            let h = LOOK_AT_HEIGHT_FRACTION * altura;

            let alpha = (1.0_f32 / radio).asin();
            let beta = phi - (radio * phi.sin() - h).atan2(radio * phi.cos());

            assert!(
                alpha + beta <= objetivo + 1e-4,
                "altura {altura}: {} excede {}",
                (alpha + beta).to_degrees(),
                objetivo.to_degrees()
            );
        }
    }

    #[test]
    fn un_monolito_mas_alto_exige_mas_radio() {
        // Es la razon de ser de la derivacion: el desvio del eje de vista
        // crece con la altura del encuadre.
        let bajo = derive_orbit_radius(1.0, 0.5);
        let alto = derive_orbit_radius(1.0, 1.5);

        assert!(alto > bajo, "{alto} deberia superar a {bajo}");
        // Y el valor fijo que traia el inventario se queda corto.
        assert!(alto > 2.2, "el 2.2 constante no habria alcanzado");
    }

    #[test]
    fn el_radio_escala_de_forma_lineal_con_la_escena() {
        // Duplicar la escena y el Monolito duplica el radio: la condicion
        // solo depende de las proporciones.
        let unitario = derive_orbit_radius(1.0, 0.5);
        let doble = derive_orbit_radius(2.0, 1.0);

        assert!(
            (doble - 2.0 * unitario).abs() < 0.02,
            "{doble} contra {unitario}"
        );
    }

    #[test]
    fn el_ojo_se_coloca_a_la_elevacion_pedida() {
        let centro = Vec3::new(0.0, 0.0, 0.0);
        let radio = 10.0;

        for yaw in [0.0_f32, 90.0, 180.0, 270.0] {
            let eye = eye_at_yaw(centro, radio, yaw);

            assert!((eye.magnitude() - radio).abs() < 1e-4, "yaw {yaw}");

            let elevacion = (eye.y / radio).asin().to_degrees();
            assert!(
                (elevacion - EYE_ELEVATION_DEGREES).abs() < 1e-3,
                "yaw {yaw}: elevacion {elevacion}"
            );
        }
    }

    #[test]
    fn el_yaw_hero_coloca_el_ojo_hacia_z_positivo() {
        let eye = eye_at_yaw(Vec3::zeros(), 10.0, HERO_YAW_DEGREES);

        assert!(eye.z > 0.0, "la toma hero debe encarar el borde roto");
        assert!(eye.x.abs() < 1e-4);
    }
}
