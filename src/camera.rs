use nalgebra_glm::Vec3;
use std::f32::consts::PI;

/// Límite del pitch: un poco antes de los polos. Justo en el polo la
/// dirección de vista queda paralela a `up` y la base se vuelve
/// degenerada — el producto cruz da el vector cero y la imagen se rompe.
const PITCH_LIMIT: f32 = PI / 2.0 - 0.1;

/// Campo de visión vertical heredado de la rama académica: 60 grados.
pub const DEFAULT_VERTICAL_FOV: f32 = PI / 3.0;

/// Cámara orbital con el eje de giro separado del punto de encuadre.
///
/// La versión académica tenía un solo `center` que hacía las dos cosas a la
/// vez: era el punto alrededor del cual giraba la cámara **y** el punto al
/// que miraba. Para el diorama eso no alcanza. El eje de giro tiene que ser
/// la vertical que atraviesa la base del Monolito —es lo que hace que la
/// órbita se sienta anclada al suelo—, pero mirar a la base deja medio
/// encuadre lleno de terreno. El encuadre apunta más arriba,
/// `0.15 × monolith_height` sobre la base.
///
/// Separarlos tiene una consecuencia que conviene tener presente: el pitch
/// de la vista deja de ser igual a la elevación del ojo sobre la esfera
/// orbital. La elevación se elige; el pitch se deriva.
pub struct Camera {
    pub eye: Vec3,
    /// Punto del eje vertical alrededor del cual gira el ojo.
    pub orbit_center: Vec3,
    /// Punto al que apunta la vista. No tiene por qué coincidir con
    /// `orbit_center`.
    pub look_at: Vec3,
    pub up: Vec3,
    pub vertical_fov: f32,
    pub min_radius: f32,
    pub max_radius: f32,
}

impl Camera {
    /// Los límites de radio arrancan derivados del radio inicial. Son
    /// provisionales: el Blockout 1 mide `scene_radius` y la Tarea 2.5 los
    /// fija contra ese valor.
    const MIN_RADIUS_FACTOR: f32 = 0.35;
    const MAX_RADIUS_FACTOR: f32 = 3.0;

    pub fn new(eye: Vec3, orbit_center: Vec3, look_at: Vec3, up: Vec3, vertical_fov: f32) -> Self {
        let radius = (eye - orbit_center).magnitude();

        Camera {
            eye,
            orbit_center,
            look_at,
            up,
            vertical_fov,
            min_radius: radius * Self::MIN_RADIUS_FACTOR,
            max_radius: radius * Self::MAX_RADIUS_FACTOR,
        }
    }

    /// Fija los límites del zoom explícitamente, una vez medidos.
    pub fn with_radius_limits(mut self, min_radius: f32, max_radius: f32) -> Self {
        self.min_radius = min_radius;
        self.max_radius = max_radius;

        self
    }

    /// Distancia del ojo al eje de órbita. Es la magnitud que el zoom
    /// modifica y que la órbita debe conservar.
    pub fn radius(&self) -> f32 {
        (self.eye - self.orbit_center).magnitude()
    }

    /// Dirección normalizada del ojo hacia el punto de encuadre.
    pub fn forward(&self) -> Vec3 {
        (self.look_at - self.eye).normalize()
    }

    /// Pitch de la vista, en radianes bajo la horizontal.
    ///
    /// Es **derivado**, no un parámetro: sale de dónde quedó el ojo y de
    /// dónde está `look_at`. Con el encuadre por encima del eje de órbita,
    /// este valor es menor que la elevación del ojo.
    pub fn view_pitch(&self) -> f32 {
        let hacia = self.look_at - self.eye;
        let horizontal = (hacia.x * hacia.x + hacia.z * hacia.z).sqrt();

        (-hacia.y).atan2(horizontal)
    }

    /// Lleva un vector de coordenadas de cámara a coordenadas del mundo.
    ///
    /// Los rayos se generan siempre igual —hacia -Z, con la pantalla en el
    /// plano XY—, así que están en el sistema de la cámara. Este cambio de
    /// base los reexpresa en el sistema del mundo, que es donde viven los
    /// objetos. El eje de vista sale de `look_at`, no de `orbit_center`.
    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = self.forward();
        let right = forward.cross(&self.up).normalize();

        // El `up` que se recibe es una intención, no necesariamente
        // perpendicular a la vista. Recalcularlo con el producto cruz de
        // los otros dos garantiza que los tres ejes sean ortogonales.
        let up = right.cross(&forward).normalize();

        // La cámara ve hacia -Z, de ahí el signo del último término.
        let rotated = vector.x * right + vector.y * up - vector.z * forward;

        rotated.normalize()
    }

    /// Gira el ojo alrededor de `orbit_center` conservando la distancia.
    ///
    /// `look_at` no participa: mover el encuadre reapunta la vista pero no
    /// desplaza el eje de giro ni cambia el radio.
    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let radius_vector = self.eye - self.orbit_center;
        let radius = radius_vector.magnitude();

        // Coordenadas esféricas: el yaw es el ángulo alrededor del eje Y y
        // el pitch la altura sobre el plano XZ.
        let current_yaw = radius_vector.z.atan2(radius_vector.x);
        let radius_xz =
            (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);
        let new_pitch = (current_pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        self.eye = self.orbit_center
            + Vec3::new(
                radius * new_yaw.cos() * new_pitch.cos(),
                -radius * new_pitch.sin(),
                radius * new_yaw.sin() * new_pitch.cos(),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra_glm::dot;

    /// Cámara con el encuadre por encima del eje de órbita, que es la
    /// configuración del diorama.
    fn camara_del_diorama() -> Camera {
        Camera::new(
            Vec3::new(0.0, 0.0, 5.0),
            Vec3::zeros(),
            Vec3::new(0.0, 0.75, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            DEFAULT_VERTICAL_FOV,
        )
    }

    #[test]
    fn la_orbita_conserva_el_radio_respecto_al_eje() {
        let mut camera = camara_del_diorama();
        let radio_inicial = camera.radius();

        for _ in 0..12 {
            camera.orbit(PI / 7.0, PI / 11.0);

            assert!(
                (camera.radius() - radio_inicial).abs() < 1e-4,
                "radio {} contra {}",
                camera.radius(),
                radio_inicial
            );
        }
    }

    #[test]
    fn basis_change_apunta_al_look_at() {
        let camera = camara_del_diorama();

        // El rayo central nace hacia -Z en coordenadas de cámara; al
        // llevarlo al mundo debe coincidir con la dirección al encuadre.
        let central = camera.basis_change(&Vec3::new(0.0, 0.0, -1.0));
        let al_encuadre = camera.forward();

        assert!(
            (central - al_encuadre).magnitude() < 1e-5,
            "central {central:?} contra {al_encuadre:?}"
        );

        // Y no al eje de órbita, que está más abajo.
        let al_eje = (camera.orbit_center - camera.eye).normalize();
        assert!(
            (central - al_eje).magnitude() > 1e-3,
            "el eje de vista se confundio con el de orbita"
        );
    }

    #[test]
    fn mover_el_encuadre_no_mueve_el_eje_orbital() {
        let mut con_encuadre_bajo = camara_del_diorama();
        let mut con_encuadre_alto = camara_del_diorama();
        con_encuadre_alto.look_at = Vec3::new(0.0, 3.0, 0.0);

        con_encuadre_bajo.orbit(PI / 5.0, PI / 9.0);
        con_encuadre_alto.orbit(PI / 5.0, PI / 9.0);

        // Misma órbita: el ojo termina exactamente en el mismo lugar.
        assert!(
            (con_encuadre_bajo.eye - con_encuadre_alto.eye).magnitude() < 1e-5,
            "{:?} contra {:?}",
            con_encuadre_bajo.eye,
            con_encuadre_alto.eye
        );
        assert!((con_encuadre_bajo.radius() - con_encuadre_alto.radius()).abs() < 1e-5);

        // Pero la vista sí cambió: apuntan a alturas distintas.
        assert!(
            (con_encuadre_bajo.forward() - con_encuadre_alto.forward()).magnitude() > 1e-3,
            "el encuadre no reapunto la vista"
        );
    }

    #[test]
    fn el_pitch_orbital_no_cruza_los_polos() {
        let mut camera = camara_del_diorama();

        // Empujar mucho más allá del polo en ambos sentidos.
        for delta in [PI, -2.0 * PI] {
            camera.orbit(0.0, delta);

            let radius_vector = camera.eye - camera.orbit_center;
            let horizontal =
                (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();

            // Si el pitch hubiera llegado al polo, la componente horizontal
            // sería cero y la base de cámara quedaría degenerada.
            assert!(horizontal > 1e-3, "componente horizontal {horizontal}");

            // Y la base sigue siendo utilizable.
            let central = camera.basis_change(&Vec3::new(0.0, 0.0, -1.0));
            assert!(central.magnitude().is_finite());
            assert!((central.magnitude() - 1.0).abs() < 1e-4);
        }
    }

    #[test]
    fn el_pitch_de_vista_es_menor_que_la_elevacion_del_ojo() {
        // Ojo elevado 35° sobre el eje, encuadre por encima del eje: la
        // vista queda menos inclinada que la posición del ojo. Es la
        // consecuencia de separar órbita y encuadre.
        let elevacion: f32 = 35.0_f32.to_radians();
        let radio = 5.0_f32;

        let camera = Camera::new(
            Vec3::new(radio * elevacion.cos(), radio * elevacion.sin(), 0.0),
            Vec3::zeros(),
            Vec3::new(0.0, 0.75, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            DEFAULT_VERTICAL_FOV,
        );

        let pitch = camera.view_pitch();

        assert!(pitch > 0.0, "la vista deberia mirar hacia abajo");
        assert!(
            pitch < elevacion,
            "pitch {} deberia ser menor que la elevacion {}",
            pitch.to_degrees(),
            elevacion.to_degrees()
        );
    }

    #[test]
    fn sin_separacion_el_pitch_iguala_la_elevacion() {
        // Caso de control: con look_at sobre el eje de órbita, el
        // comportamiento colapsa al de la cámara académica.
        let elevacion: f32 = 35.0_f32.to_radians();
        let radio = 5.0_f32;

        let camera = Camera::new(
            Vec3::new(radio * elevacion.cos(), radio * elevacion.sin(), 0.0),
            Vec3::zeros(),
            Vec3::zeros(),
            Vec3::new(0.0, 1.0, 0.0),
            DEFAULT_VERTICAL_FOV,
        );

        assert!((camera.view_pitch() - elevacion).abs() < 1e-5);
    }

    #[test]
    fn los_limites_de_radio_se_derivan_y_se_pueden_fijar() {
        let camera = camara_del_diorama();

        assert!(camera.min_radius < camera.radius());
        assert!(camera.max_radius > camera.radius());

        let fijada = camara_del_diorama().with_radius_limits(2.0, 9.0);

        assert_eq!(fijada.min_radius, 2.0);
        assert_eq!(fijada.max_radius, 9.0);
    }

    #[test]
    fn la_base_de_camara_es_ortonormal() {
        let camera = camara_del_diorama();

        let derecha = camera.basis_change(&Vec3::new(1.0, 0.0, 0.0));
        let arriba = camera.basis_change(&Vec3::new(0.0, 1.0, 0.0));
        let adelante = camera.basis_change(&Vec3::new(0.0, 0.0, -1.0));

        for vector in [derecha, arriba, adelante] {
            assert!((vector.magnitude() - 1.0).abs() < 1e-5);
        }

        assert!(dot(&derecha, &arriba).abs() < 1e-5);
        assert!(dot(&derecha, &adelante).abs() < 1e-5);
        assert!(dot(&arriba, &adelante).abs() < 1e-5);
    }
}
