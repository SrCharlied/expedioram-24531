use crate::ray::Ray;
use nalgebra_glm::Vec3;

/// Caja alineada a los ejes. Es la pieza que sostiene dos cosas distintas:
/// la geometría del cuboide y, más adelante, los bounds de cada grupo y
/// cluster de la estructura de aceleración.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

/// Tramo del rayo que queda dentro de la caja.
///
/// Además de las dos distancias guarda **por qué eje** entró y salió. El
/// cuboide necesita ese dato para saber qué cara tocó, y recalcularlo
/// después significaría repetir el mismo slab test dos veces.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RayInterval {
    pub t_enter: f32,
    pub t_exit: f32,
    pub enter_axis: usize,
    pub exit_axis: usize,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Aabb { min, max }
    }

    /// Construye la caja a partir de dos esquinas cualesquiera, ordenando
    /// cada eje. Evita el error de pasar las esquinas al revés y obtener una
    /// caja vacía que nunca impacta.
    pub fn from_corners(a: Vec3, b: Vec3) -> Self {
        Aabb {
            min: Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z)),
            max: Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z)),
        }
    }

    /// Caja mínima que contiene a las dos. Es la operación con la que la
    /// estructura de aceleración compone los bounds de cada cluster, cada
    /// grupo y la escena entera.
    pub fn union(&self, otra: &Aabb) -> Aabb {
        Aabb {
            min: Vec3::new(
                self.min.x.min(otra.min.x),
                self.min.y.min(otra.min.y),
                self.min.z.min(otra.min.z),
            ),
            max: Vec3::new(
                self.max.x.max(otra.max.x),
                self.max.y.max(otra.max.y),
                self.max.z.max(otra.max.z),
            ),
        }
    }

    pub fn centro(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    pub fn contiene(&self, punto: &Vec3) -> bool {
        (0..3).all(|eje| punto[eje] >= self.min[eje] && punto[eje] <= self.max[eje])
    }

    /// Slab test: interseca el rayo contra los tres pares de planos y se
    /// queda con la intersección de los tres intervalos.
    ///
    /// `t_min` y `t_max` acotan la búsqueda. Pasar `EPSILON` como `t_min` es
    /// lo que evita el autoimpacto de un rayo que nace sobre la superficie.
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayInterval> {
        let mut t_enter = t_min;
        let mut t_exit = t_max;
        let mut enter_axis = 0;
        let mut exit_axis = 0;

        for eje in 0..3 {
            let direccion = ray.direction[eje];
            let origen = ray.origin[eje];

            // Rayo paralelo a este par de planos. La división daría
            // infinito y, si el origen cae justo sobre un plano, un NaN que
            // se colaría silenciosamente por `max`/`min`. Se resuelve
            // aparte: o el origen ya está dentro de la franja y el eje no
            // impone restricción, o está fuera y no hay impacto posible.
            if direccion.abs() < f32::EPSILON {
                if origen < self.min[eje] || origen > self.max[eje] {
                    return None;
                }
                continue;
            }

            let inversa = 1.0 / direccion;
            let mut t0 = (self.min[eje] - origen) * inversa;
            let mut t1 = (self.max[eje] - origen) * inversa;

            // Con dirección negativa el plano `min` se alcanza después que
            // el `max`, así que el par llega invertido.
            if inversa < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > t_enter {
                t_enter = t0;
                enter_axis = eje;
            }
            if t1 < t_exit {
                t_exit = t1;
                exit_axis = eje;
            }

            if t_exit <= t_enter {
                return None;
            }
        }

        Some(RayInterval {
            t_enter,
            t_exit,
            enter_axis,
            exit_axis,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EPSILON;

    fn cubo_unitario() -> Aabb {
        Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0))
    }

    const T_MAX: f32 = f32::INFINITY;

    #[test]
    fn rayo_frontal_impacta() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));
        let intervalo = cubo_unitario()
            .hit(&ray, EPSILON, T_MAX)
            .expect("debe impactar");

        assert!((intervalo.t_enter - 4.0).abs() < 1e-5);
        assert!((intervalo.t_exit - 6.0).abs() < 1e-5);
    }

    #[test]
    fn intervalo_reporta_entrada_y_salida_ordenadas() {
        let ray = Ray::new(Vec3::new(-5.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let intervalo = cubo_unitario()
            .hit(&ray, EPSILON, T_MAX)
            .expect("debe impactar");

        assert!(intervalo.t_enter < intervalo.t_exit);
        assert_eq!(intervalo.enter_axis, 0);
        assert_eq!(intervalo.exit_axis, 0);
    }

    #[test]
    fn rayo_paralelo_por_fuera_falla() {
        // Viaja a lo largo de Z, pero desplazado en X más allá de la caja.
        let ray = Ray::new(Vec3::new(5.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

        assert!(cubo_unitario().hit(&ray, EPSILON, T_MAX).is_none());
    }

    #[test]
    fn rayo_paralelo_por_dentro_conserva_el_intervalo() {
        // Dirección exactamente paralela a X e Y: esos dos ejes no deben
        // imponer restricción ni producir NaN.
        let ray = Ray::new(Vec3::new(0.5, -0.25, 5.0), Vec3::new(0.0, 0.0, -1.0));
        let intervalo = cubo_unitario()
            .hit(&ray, EPSILON, T_MAX)
            .expect("debe impactar");

        assert!((intervalo.t_enter - 4.0).abs() < 1e-5);
        assert!((intervalo.t_exit - 6.0).abs() < 1e-5);
    }

    #[test]
    fn rayo_paralelo_rozando_el_plano_no_produce_nan() {
        // Origen exactamente sobre el plano x = 1. Es el caso que genera
        // 0 * infinito si el eje paralelo no se trata aparte.
        let ray = Ray::new(Vec3::new(1.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));
        let intervalo = cubo_unitario().hit(&ray, EPSILON, T_MAX);

        if let Some(intervalo) = intervalo {
            assert!(intervalo.t_enter.is_finite() && !intervalo.t_enter.is_nan());
            assert!(intervalo.t_exit.is_finite() && !intervalo.t_exit.is_nan());
        }
    }

    #[test]
    fn rayo_nacido_adentro_devuelve_salida_valida() {
        let ray = Ray::new(Vec3::zeros(), Vec3::new(0.0, 0.0, -1.0));
        let intervalo = cubo_unitario()
            .hit(&ray, EPSILON, T_MAX)
            .expect("debe reportar la salida");

        // El origen ya está dentro, así que la entrada quedó acotada por
        // t_min y lo útil es la salida.
        assert!((intervalo.t_enter - EPSILON).abs() < 1e-5);
        assert!((intervalo.t_exit - 1.0).abs() < 1e-5);
    }

    #[test]
    fn epsilon_evita_el_autoimpacto() {
        // Rayo que nace sobre la cara +Z y se aleja hacia adentro: sin
        // t_min la entrada saldría en 0.0 y el rayo se impactaría a sí mismo.
        let ray = Ray::new(Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, -1.0));
        let intervalo = cubo_unitario()
            .hit(&ray, EPSILON, T_MAX)
            .expect("debe impactar");

        assert!(
            intervalo.t_enter >= EPSILON,
            "t_enter {} quedo por debajo del epsilon",
            intervalo.t_enter
        );
    }

    #[test]
    fn caja_detras_del_origen_falla() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 1.0));

        assert!(cubo_unitario().hit(&ray, EPSILON, T_MAX).is_none());
    }

    #[test]
    fn from_corners_ordena_las_esquinas() {
        let caja = Aabb::from_corners(Vec3::new(1.0, 1.0, 1.0), Vec3::new(-1.0, -1.0, -1.0));

        assert_eq!(caja, cubo_unitario());
    }

    #[test]
    fn union_contiene_a_las_dos() {
        let a = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(0.0, 0.0, 0.0));
        let b = Aabb::new(Vec3::new(2.0, 3.0, 1.0), Vec3::new(4.0, 5.0, 2.0));
        let u = a.union(&b);

        assert_eq!(u.min, Vec3::new(-1.0, -1.0, -1.0));
        assert_eq!(u.max, Vec3::new(4.0, 5.0, 2.0));
        assert!(u.contiene(&a.min) && u.contiene(&a.max));
        assert!(u.contiene(&b.min) && u.contiene(&b.max));
    }

    #[test]
    fn la_union_es_conmutativa_e_idempotente() {
        let a = Aabb::new(Vec3::new(-2.0, 0.0, -1.0), Vec3::new(1.0, 2.0, 3.0));
        let b = Aabb::new(Vec3::new(0.0, -3.0, 0.0), Vec3::new(2.0, 1.0, 1.0));

        assert_eq!(a.union(&b), b.union(&a));
        assert_eq!(a.union(&a), a);
    }

    #[test]
    fn contiene_y_centro() {
        let caja = cubo_unitario();

        assert!(caja.contiene(&Vec3::zeros()));
        assert!(!caja.contiene(&Vec3::new(0.0, 2.0, 0.0)));
        assert_eq!(caja.centro(), Vec3::zeros());
    }
}
