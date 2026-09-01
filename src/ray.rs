use nalgebra_glm::Vec3;

/// Un rayo: de dónde sale y hacia dónde va.
///
/// Antes el origen y la dirección viajaban como dos parámetros sueltos en
/// cada firma. Empaquetarlos evita el error de pasarlos invertidos y da un
/// lugar natural a `at`, que es la operación que todos los tests de
/// intersección necesitan.
///
/// La dirección se asume normalizada: es lo que hace que el parámetro `t`
/// sea una distancia real y no un múltiplo arbitrario.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    /// Punto del rayo a distancia `t` del origen.
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_vec_close(actual: Vec3, expected: Vec3) {
        let delta = (actual - expected).magnitude();
        assert!(delta < 1e-5, "esperado {expected:?}, obtenido {actual:?}");
    }

    #[test]
    fn at_cero_devuelve_el_origen() {
        let ray = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 0.0, -1.0));

        assert_vec_close(ray.at(0.0), Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn at_dos_avanza_dos_unidades() {
        let ray = Ray::new(Vec3::zeros(), Vec3::new(0.0, 0.0, -1.0));

        assert_vec_close(ray.at(2.0), Vec3::new(0.0, 0.0, -2.0));
    }

    #[test]
    fn con_direccion_normalizada_t_es_una_distancia() {
        // Dirección diagonal normalizada: avanzar t=5 debe alejarse
        // exactamente 5 unidades del origen, no 5 veces el vector crudo.
        let direccion = Vec3::new(1.0, 1.0, 0.0).normalize();
        let ray = Ray::new(Vec3::zeros(), direccion);

        let recorrido = (ray.at(5.0) - ray.origin).magnitude();

        assert!((recorrido - 5.0).abs() < 1e-5, "recorrido {recorrido}");
    }
}
