use nalgebra_glm::Vec3;
use std::f32::consts::PI;

/// Límite del pitch: un poco antes de los polos. Justo en el polo la
/// dirección de vista queda paralela a `up` y la base se vuelve
/// degenerada — el producto cruz da el vector cero y la imagen se rompe.
const PITCH_LIMIT: f32 = PI / 2.0 - 0.1;

/// Cámara descrita por tres vectores, la convención de `lookAt`: desde
/// dónde se ve, qué se ve y hacia dónde queda arriba.
pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Camera { eye, center, up }
    }

    /// Lleva un vector de coordenadas de cámara a coordenadas del mundo.
    ///
    /// Los rayos se generan siempre igual —hacia -Z, con la pantalla en el
    /// plano XY—, así que están en el sistema de la cámara. Este cambio de
    /// base los reexpresa en el sistema del mundo, que es donde viven los
    /// objetos.
    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = (self.center - self.eye).normalize();
        let right = forward.cross(&self.up).normalize();

        // El `up` que se recibe es una intención, no necesariamente
        // perpendicular a la vista. Recalcularlo con el producto cruz de
        // los otros dos garantiza que los tres ejes sean ortogonales.
        let up = right.cross(&forward).normalize();

        // La cámara ve hacia -Z, de ahí el signo del último término.
        let rotated = vector.x * right + vector.y * up - vector.z * forward;

        rotated.normalize()
    }

    /// Gira el ojo alrededor del centro conservando la distancia. El centro
    /// no se mueve: la cámara se desliza sobre una esfera imaginaria.
    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();

        // Coordenadas esféricas: el yaw es el ángulo alrededor del eje Y y
        // el pitch la altura sobre el plano XZ.
        let current_yaw = radius_vector.z.atan2(radius_vector.x);
        let radius_xz = (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);
        let new_pitch = (current_pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        self.eye = self.center
            + Vec3::new(
                radius * new_yaw.cos() * new_pitch.cos(),
                -radius * new_pitch.sin(),
                radius * new_yaw.sin() * new_pitch.cos(),
            );
    }
}
