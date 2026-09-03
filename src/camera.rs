use crate::ray::Ray;
use nalgebra_glm::{normalize, Vec3};
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

    /// Acerca o aleja el ojo modificando el radio orbital.
    ///
    /// `delta` es un cambio de distancia con signo: negativo acerca. La
    /// dirección no se toca —el ojo se desliza sobre el mismo rayo que sale
    /// de `orbit_center`—, así que hacer zoom nunca reencuadra la escena.
    ///
    /// El resultado se recorta a `min_radius..=max_radius`. Sin ese recorte
    /// se rompen dos cosas: acercarse demasiado mete la cámara dentro del
    /// diorama y empieza a trazar desde el interior de la geometría, y
    /// alejarse demasiado deja el Continente reducido a un punto.
    pub fn zoom(&mut self, delta: f32) {
        let radius_vector = self.eye - self.orbit_center;
        let radius = radius_vector.magnitude();

        // Con el ojo exactamente sobre el eje no hay dirección que
        // conservar. No es un estado alcanzable orbitando, pero tampoco hay
        // que dividir entre cero por él.
        if radius < f32::EPSILON {
            return;
        }

        let nuevo = (radius + delta).clamp(self.min_radius, self.max_radius);

        self.eye = self.orbit_center + radius_vector * (nuevo / radius);
    }

    /// Rayo primario que atraviesa el centro del píxel `(x, y)`.
    ///
    /// Vive en la cámara y no en el renderer porque el picking del Hito 6
    /// tiene que convertir la posición del cursor con **esta misma**
    /// proyección: si el render y el picking calcularan la dirección por
    /// separado, un clic terminaría apuntando a un píxel distinto del que
    /// se ve en pantalla.
    ///
    /// El `+ 0.5` muestrea el centro del píxel y no su borde. Un píxel es un
    /// área y el renderer la representa por su centro; un cursor es un
    /// punto, y de eso se encarga `ray_from_cursor`. Las dos rutas comparten
    /// la proyección y solo difieren en cómo llegan a las coordenadas de
    /// pantalla, que es la diferencia correcta.
    pub fn ray_from_pixel(&self, x: usize, y: usize, width: usize, height: usize) -> Ray {
        let screen_x = (2.0 * (x as f32 + 0.5)) / width as f32 - 1.0;
        let screen_y = -(2.0 * (y as f32 + 0.5)) / height as f32 + 1.0;

        self.ray_from_screen(screen_x, screen_y, width, height)
    }

    /// Rayo primario que pasa por la posición **continua** del cursor.
    ///
    /// `cursor` va en píxeles de ventana, con el origen arriba a la
    /// izquierda, que es como lo entrega `minifb`. No se redondea a píxel:
    /// el centro exacto de la ventana produce el rayo central exacto, lo que
    /// con una resolución par no ocurriría al redondear —no hay píxel
    /// central en 800 × 600—.
    ///
    /// El tamaño que se pasa es el de la **ventana**, no el del perfil
    /// interactivo. Da lo mismo en la práctica, y conviene saber por qué: el
    /// escalado por vecino más cercano preserva las coordenadas de pantalla,
    /// así que un píxel de ventana y su píxel de perfil correspondiente
    /// caen en la misma coordenada normalizada.
    pub fn ray_from_cursor(&self, cursor: (f32, f32), width: usize, height: usize) -> Ray {
        let screen_x = (2.0 * cursor.0) / width as f32 - 1.0;
        let screen_y = -(2.0 * cursor.1) / height as f32 + 1.0;

        self.ray_from_screen(screen_x, screen_y, width, height)
    }

    /// La proyección, compartida por el render y el picking.
    ///
    /// `screen_x` y `screen_y` van de `-1` a `1`, con la `y` ya invertida
    /// respecto del orden de filas de la imagen.
    ///
    /// Es la única implementación de la perspectiva del proyecto. Extraerla
    /// es lo que hace **cierta** la promesa de que un clic apunta al píxel
    /// que se ve: no hay una segunda fórmula que pueda desviarse.
    fn ray_from_screen(&self, screen_x: f32, screen_y: f32, width: usize, height: usize) -> Ray {
        let aspect_ratio = width as f32 / height as f32;

        // Media altura del plano de proyección, que está a una unidad de la
        // cámara. Abrir el campo de visión ensancha el plano.
        let perspective_scale = (self.vertical_fov / 2.0).tan();

        // El rayo nace en coordenadas de cámara —viendo hacia -Z— y el
        // cambio de base lo lleva al mundo, donde están los objetos.
        let direccion = normalize(&Vec3::new(
            screen_x * aspect_ratio * perspective_scale,
            screen_y * perspective_scale,
            -1.0,
        ));

        Ray::new(self.eye, self.basis_change(&direccion))
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

    /// Ángulo entre dos direcciones, en radianes.
    fn angulo(a: Vec3, b: Vec3) -> f32 {
        dot(&a, &b).clamp(-1.0, 1.0).acos()
    }

    // Dimensiones impares: existe un píxel exactamente central.
    const ANCHO: usize = 33;
    const ALTO: usize = 25;

    #[test]
    fn el_pixel_central_apunta_al_look_at() {
        let camera = camara_del_diorama();
        let ray = camera.ray_from_pixel(ANCHO / 2, ALTO / 2, ANCHO, ALTO);

        assert_eq!(ray.origin, camera.eye);
        assert!(
            (ray.direction - camera.forward()).magnitude() < 1e-5,
            "{:?} contra {:?}",
            ray.direction,
            camera.forward()
        );
    }

    #[test]
    fn las_esquinas_respetan_el_fov_vertical() {
        let camera = camara_del_diorama();

        let arriba = camera.ray_from_pixel(ANCHO / 2, 0, ANCHO, ALTO);
        let abajo = camera.ray_from_pixel(ANCHO / 2, ALTO - 1, ANCHO, ALTO);

        // Los píxeles extremos se muestrean en su centro, así que abarcan
        // (1 - 1/alto) del campo de visión, no el 100 %.
        let fraccion = 1.0 - 1.0 / ALTO as f32;
        let esperado = 2.0 * (fraccion * (DEFAULT_VERTICAL_FOV / 2.0).tan()).atan();
        let medido = angulo(arriba.direction, abajo.direction);

        assert!(
            (medido - esperado).abs() < 1e-4,
            "vertical {} contra {}",
            medido.to_degrees(),
            esperado.to_degrees()
        );
    }

    #[test]
    fn las_esquinas_respetan_el_aspecto() {
        let camera = camara_del_diorama();

        let izquierda = camera.ray_from_pixel(0, ALTO / 2, ANCHO, ALTO);
        let derecha = camera.ray_from_pixel(ANCHO - 1, ALTO / 2, ANCHO, ALTO);

        let aspect_ratio = ANCHO as f32 / ALTO as f32;
        let fraccion = 1.0 - 1.0 / ANCHO as f32;
        let esperado = 2.0 * (fraccion * aspect_ratio * (DEFAULT_VERTICAL_FOV / 2.0).tan()).atan();
        let horizontal = angulo(izquierda.direction, derecha.direction);

        assert!(
            (horizontal - esperado).abs() < 1e-4,
            "horizontal {} contra {}",
            horizontal.to_degrees(),
            esperado.to_degrees()
        );

        // El aspecto se aplica a la horizontal: en un frame apaisado el
        // campo horizontal tiene que ser el mayor de los dos.
        let vertical = angulo(
            camera.ray_from_pixel(ANCHO / 2, 0, ANCHO, ALTO).direction,
            camera
                .ray_from_pixel(ANCHO / 2, ALTO - 1, ANCHO, ALTO)
                .direction,
        );

        assert!(
            horizontal > vertical,
            "el aspecto se aplico al eje equivocado"
        );
    }

    #[test]
    fn la_fila_cero_esta_arriba() {
        // Cámara horizontal: el píxel de la fila 0 debe mirar por encima
        // del central, no por debajo.
        let camera = Camera::new(
            Vec3::new(0.0, 0.0, 5.0),
            Vec3::zeros(),
            Vec3::zeros(),
            Vec3::new(0.0, 1.0, 0.0),
            DEFAULT_VERTICAL_FOV,
        );

        let arriba = camera.ray_from_pixel(ANCHO / 2, 0, ANCHO, ALTO);
        let centro = camera.ray_from_pixel(ANCHO / 2, ALTO / 2, ANCHO, ALTO);

        assert!(
            arriba.direction.y > centro.direction.y,
            "la fila 0 apunto hacia abajo"
        );
    }

    #[test]
    fn el_zoom_conserva_la_direccion_y_cambia_el_radio() {
        let mut camera = camara_del_diorama();
        let direccion = (camera.eye - camera.orbit_center).normalize();
        let radio = camera.radius();

        camera.zoom(-1.5);

        assert!((camera.radius() - (radio - 1.5)).abs() < 1e-5);
        assert!(
            ((camera.eye - camera.orbit_center).normalize() - direccion).magnitude() < 1e-5,
            "el zoom reencuadro la escena"
        );

        // Ni el eje de órbita ni el encuadre se mueven.
        assert_eq!(camera.orbit_center, Vec3::zeros());
        assert_eq!(camera.look_at, Vec3::new(0.0, 0.75, 0.0));
    }

    #[test]
    fn los_clamps_impiden_entrar_al_diorama_o_perderse() {
        let mut camera = camara_del_diorama();

        camera.zoom(-1_000.0);
        assert!(
            (camera.radius() - camera.min_radius).abs() < 1e-4,
            "radio {} deberia haberse detenido en {}",
            camera.radius(),
            camera.min_radius
        );
        assert!(camera.radius() > 0.0, "la camara colapso sobre el eje");

        camera.zoom(1_000.0);
        assert!(
            (camera.radius() - camera.max_radius).abs() < 1e-4,
            "radio {} deberia haberse detenido en {}",
            camera.radius(),
            camera.max_radius
        );
    }

    #[test]
    fn el_zoom_reencuadra_el_pitch_pero_sigue_apuntando_al_look_at() {
        // Consecuencia de separar órbita y encuadre: con `look_at` a otra
        // altura que `orbit_center`, acercarse cambia el ángulo bajo el que
        // se ve el punto de encuadre. El pitch de la vista está acoplado al
        // radio y no puede elegirse independientemente del zoom.
        let mut camera = camara_del_diorama();
        let pitch_lejos = camera.view_pitch();

        camera.zoom(-3.0);

        assert!(
            (camera.view_pitch() - pitch_lejos).abs() > 1e-3,
            "el pitch deberia cambiar al acercarse"
        );

        // Lo que sí se conserva: el rayo central sigue apuntando al
        // encuadre. `ray_from_pixel` y `forward` no pueden divergir.
        let central = camera
            .ray_from_pixel(ANCHO / 2, ALTO / 2, ANCHO, ALTO)
            .direction;

        assert!((central - camera.forward()).magnitude() < 1e-5);
    }

    #[test]
    fn sin_separacion_el_zoom_no_toca_el_pitch() {
        // Caso de control: con el encuadre sobre el eje de órbita, el
        // acoplamiento desaparece.
        let mut camera = Camera::new(
            Vec3::new(0.0, 3.0, 4.0),
            Vec3::zeros(),
            Vec3::zeros(),
            Vec3::new(0.0, 1.0, 0.0),
            DEFAULT_VERTICAL_FOV,
        );
        let pitch_lejos = camera.view_pitch();

        camera.zoom(-2.0);

        assert!((camera.view_pitch() - pitch_lejos).abs() < 1e-5);
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

    // ------------------------------------------------- rayo del cursor

    #[test]
    fn el_cursor_en_el_centro_de_la_ventana_da_el_rayo_central() {
        // Con resolucion par no hay pixel central, asi que esto solo puede
        // salir exacto porque el cursor no se redondea a pixel.
        let camara = camara_del_diorama();
        let (ancho, alto) = (800, 600);

        let central = camara.ray_from_cursor((ancho as f32 / 2.0, alto as f32 / 2.0), ancho, alto);

        // El rayo central apunta al look_at, que es la propiedad que el
        // Hito 2 ya fijo para el render.
        let hacia_look_at = (camara.look_at - camara.eye).normalize();
        let desvio = (central.direction - hacia_look_at).magnitude();

        assert!(desvio < 1e-6, "el rayo central desvio {desvio} del look_at");
    }

    #[test]
    fn el_cursor_en_el_centro_de_un_pixel_da_el_rayo_de_ese_pixel() {
        // La forma fuerte de «la misma funcion que el renderer»: para el
        // centro de cualquier pixel, las dos rutas coinciden bit a bit
        // dentro de la tolerancia. Cubre de una vez el aspect ratio y el
        // campo de vision, porque los dos entran en la misma proyeccion.
        let camara = camara_del_diorama();

        for (ancho, alto) in [(800, 600), (400, 300), (37, 91)] {
            for (x, y) in [(0, 0), (1, 2), (ancho / 2, alto / 2), (ancho - 1, alto - 1)] {
                let del_pixel = camara.ray_from_pixel(x, y, ancho, alto);
                let del_cursor =
                    camara.ray_from_cursor((x as f32 + 0.5, y as f32 + 0.5), ancho, alto);

                let desvio = (del_pixel.direction - del_cursor.direction).magnitude();

                assert!(
                    desvio < 1e-6,
                    "en {ancho}x{alto} el pixel ({x}, {y}) desvio {desvio}"
                );
                assert_eq!(del_pixel.origin, del_cursor.origin);
            }
        }
    }

    #[test]
    fn el_cursor_respeta_el_campo_de_vision_vertical() {
        // El borde superior de la ventana tiene que caer justo a medio FOV
        // del rayo central. Es la comprobacion de que el picking hereda el
        // encuadre del render y no otro.
        let camara = camara_del_diorama();
        let (ancho, alto) = (800, 600);

        let central = camara.ray_from_cursor((400.0, 300.0), ancho, alto);
        let arriba = camara.ray_from_cursor((400.0, 0.0), ancho, alto);

        let angulo = dot(&central.direction, &arriba.direction).acos();
        let medio_fov = camara.vertical_fov / 2.0;

        assert!(
            (angulo - medio_fov).abs() < 1e-4,
            "el borde superior esta a {angulo} rad y medio FOV es {medio_fov}"
        );
    }

    #[test]
    fn el_cursor_respeta_el_aspect_ratio() {
        // El borde lateral abre mas que el superior, y exactamente en la
        // proporcion del cuadro: es lo que significa el aspect ratio.
        let camara = camara_del_diorama();
        let (ancho, alto) = (800, 600);

        let central = camara.ray_from_cursor((400.0, 300.0), ancho, alto);
        let lado = camara.ray_from_cursor((0.0, 300.0), ancho, alto);
        let arriba = camara.ray_from_cursor((400.0, 0.0), ancho, alto);

        // Comparando tangentes, que es donde la relacion es exacta; los
        // angulos no escalan linealmente.
        let tan_lado = dot(&central.direction, &lado.direction).acos().tan();
        let tan_arriba = dot(&central.direction, &arriba.direction).acos().tan();

        let razon = tan_lado / tan_arriba;
        let esperada = ancho as f32 / alto as f32;

        assert!(
            (razon - esperada).abs() < 1e-4,
            "la razon de tangentes es {razon} y el aspect ratio {esperada}"
        );
    }

    #[test]
    fn el_cursor_fuera_del_cuadro_sigue_dando_un_rayo_valido() {
        // La geometria no tiene por que rechazar nada: extrapolar mas alla
        // del borde es una direccion perfectamente definida. Quien decide
        // que un clic fuera de la ventana no cuenta es `input`.
        let camara = camara_del_diorama();

        for cursor in [(-50.0, 300.0), (900.0, 300.0), (400.0, -10.0)] {
            let rayo = camara.ray_from_cursor(cursor, 800, 600);

            assert!((rayo.direction.magnitude() - 1.0).abs() < 1e-6);
        }
    }
}
