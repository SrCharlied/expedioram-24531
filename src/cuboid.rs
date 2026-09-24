use crate::bounds::Aabb;
use crate::hit::{Hit, UvChart};
use crate::primitive::UvWorldScale;
use crate::ray::Ray;
use crate::ray_intersect::RayIntersect;
use crate::EPSILON;
use nalgebra_glm::{Vec2, Vec3};

/// Cuboide alineado a los ejes: la primitiva obligatoria del proyecto.
///
/// La distancia sale del slab test de `Aabb`. Lo que agrega este tipo es lo
/// que el AABB no necesita saber para acelerar pero el sombreado sí: qué
/// cara se tocó, hacia dónde mira y qué coordenada de textura corresponde.
#[derive(Debug, Clone, Copy)]
pub struct Cuboid {
    pub bounds: Aabb,
}

impl Cuboid {
    pub fn new(bounds: Aabb) -> Self {
        Cuboid { bounds }
    }

    /// Cuboide a partir de dos esquinas cualesquiera.
    pub fn from_corners(a: Vec3, b: Vec3) -> Self {
        Cuboid::new(Aabb::from_corners(a, b))
    }

    /// Cuboide centrado con un tamaño dado por eje. La escala no uniforme
    /// es lo normal en el diorama: pilares, tablones y masas de terreno son
    /// todos cajas estiradas.
    pub fn centrado(centro: Vec3, tamano: Vec3) -> Self {
        let medio = tamano * 0.5;
        Cuboid::new(Aabb::new(centro - medio, centro + medio))
    }

    /// Normal exterior de la cara perpendicular a `eje`.
    ///
    /// El signo sale de hacia dónde viaja el rayo: si avanza en `+eje`,
    /// entró por el plano `min` y esa cara mira hacia `-eje`.
    fn normal_de_cara(eje: usize, direccion_en_eje: f32, entrando: bool) -> Vec3 {
        let mut normal = Vec3::zeros();
        let hacia_adelante = direccion_en_eje > 0.0;

        // Al entrar se toca la cara opuesta al avance; al salir, la del
        // mismo lado hacia el que se avanza.
        normal[eje] = if hacia_adelante == entrando {
            -1.0
        } else {
            1.0
        };

        normal
    }

    /// Coordenada de textura dentro de la cara perpendicular a `eje`.
    ///
    /// Se usan los otros dos ejes como tangentes, normalizando la posición
    /// contra la extensión de la caja. Una cara degenerada —extensión cero—
    /// devuelve `0.0` en vez de dividir entre cero.
    fn uv_en_cara(&self, punto: &Vec3, eje: usize) -> Vec2 {
        let (eje_u, eje_v) = Cuboid::tangentes_de(eje);

        Vec2::new(self.fraccion(punto, eje_u), self.fraccion(punto, eje_v))
    }

    /// Carta de la cara perpendicular a `eje`, segun hacia donde mira.
    ///
    /// Seis caras, seis cartas, y las opuestas **no** pueden compartir: las
    /// dos caras de un mismo eje recorren identicas `uv` sobre los otros
    /// dos, asi que la coordenada sola no las distingue. Ver `UvChart`.
    ///
    /// El reparto —`+X`, `-X`, `+Y`, `-Y`, `+Z`, `-Z` en `1..=6`— es local
    /// a esta primitiva y solo significa algo junto al `object_index`.
    fn carta_de_cara(eje: usize, normal_exterior: &Vec3) -> UvChart {
        let hacia_positivo = normal_exterior[eje] > 0.0;

        UvChart::new(1 + 2 * eje as u8 + u8::from(!hacia_positivo))
    }

    /// Métrica mundo/`uv` de una de sus seis cartas.
    ///
    /// Las longitudes son las de los **mismos ejes tangentes** que usa
    /// `uv_en_cara`, y salen del mismo `match`: cara `X` recorre `Z` y `Y`,
    /// cara `Y` recorre `X` y `Z`, cara `Z` recorre `X` e `Y`. Si las dos
    /// tablas se separaran, el pincel se deformaría sobre una cara sin que
    /// nada más lo delatara.
    ///
    /// Las dos caras de un mismo eje comparten métrica —son del mismo
    /// tamaño— aunque sean cartas distintas: la carta separa *dónde* se
    /// pinta, no *cuánto* mide.
    ///
    /// Devuelve `None` para una carta que no sea suya y para una cara
    /// degenerada, que no tiene medida que dar.
    pub fn uv_world_scale(&self, chart: UvChart) -> Option<UvWorldScale> {
        // El reparto inverso de `carta_de_cara`: `1 + 2 * eje`, más uno si
        // mira al lado negativo.
        let eje = match chart.id() {
            1 | 2 => 0,
            3 | 4 => 1,
            5 | 6 => 2,
            _ => return None,
        };
        let (eje_u, eje_v) = Cuboid::tangentes_de(eje);

        UvWorldScale::new(self.extension(eje_u), self.extension(eje_v))
    }

    /// Los dos ejes que recorren `u` y `v` en la cara perpendicular a
    /// `eje`. Es la tabla que comparten `uv_en_cara` y `uv_world_scale`.
    fn tangentes_de(eje: usize) -> (usize, usize) {
        match eje {
            0 => (2, 1), // cara X: u recorre Z, v recorre Y
            1 => (0, 2), // cara Y: u recorre X, v recorre Z
            _ => (0, 1), // cara Z: u recorre X, v recorre Y
        }
    }

    /// Longitud de la caja en un eje.
    fn extension(&self, eje: usize) -> f32 {
        self.bounds.max[eje] - self.bounds.min[eje]
    }

    fn fraccion(&self, punto: &Vec3, eje: usize) -> f32 {
        let extension = self.extension(eje);

        if extension.abs() < f32::EPSILON {
            return 0.0;
        }

        ((punto[eje] - self.bounds.min[eje]) / extension).clamp(0.0, 1.0)
    }
}

impl RayIntersect for Cuboid {
    fn ray_intersect(&self, ray: &Ray) -> Option<Hit> {
        let intervalo = self.bounds.hit(ray, EPSILON, f32::INFINITY)?;

        // Un rayo que nace dentro del cuboide no tiene cara de entrada
        // visible: lo que va a tocar es la cara por la que sale. Es el caso
        // del rayo refractado que viaja dentro del volumen de agua.
        let desde_adentro = self.bounds.contiene(&ray.origin);

        let (t, eje) = if desde_adentro {
            (intervalo.t_exit, intervalo.exit_axis)
        } else {
            (intervalo.t_enter, intervalo.enter_axis)
        };

        if t <= EPSILON {
            return None;
        }

        let punto = ray.at(t);
        let normal = Cuboid::normal_de_cara(eje, ray.direction[eje], !desde_adentro);

        Some(
            Hit::new(ray, t, normal, self.uv_en_cara(&punto, eje))
                .on_chart(Cuboid::carta_de_cara(eje, &normal)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cubo_unitario() -> Cuboid {
        Cuboid::from_corners(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0))
    }

    /// Las seis caras, cada una con la dirección desde la que se le dispara
    /// y la normal exterior que debe reportar.
    const CARAS: [([f32; 3], [f32; 3]); 6] = [
        ([1.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        ([-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]),
        ([0.0, 1.0, 0.0], [0.0, 1.0, 0.0]),
        ([0.0, -1.0, 0.0], [0.0, -1.0, 0.0]),
        ([0.0, 0.0, 1.0], [0.0, 0.0, 1.0]),
        ([0.0, 0.0, -1.0], [0.0, 0.0, -1.0]),
    ];

    #[test]
    fn las_seis_caras_devuelven_su_normal() {
        for (desde, normal_esperada) in CARAS {
            let fuera = Vec3::new(desde[0], desde[1], desde[2]) * 5.0;
            let esperada = Vec3::new(normal_esperada[0], normal_esperada[1], normal_esperada[2]);

            let ray = Ray::new(fuera, -fuera.normalize());
            let hit = cubo_unitario()
                .ray_intersect(&ray)
                .unwrap_or_else(|| panic!("la cara {esperada:?} debe impactar"));

            assert_eq!(hit.normal, esperada, "cara {esperada:?}");
            assert!(hit.front_face, "cara {esperada:?} se toca desde afuera");
            assert!((hit.distance - 4.0).abs() < 1e-5, "cara {esperada:?}");
        }
    }

    #[test]
    fn uv_permanece_en_rango_unitario() {
        for (desde, _) in CARAS {
            let fuera = Vec3::new(desde[0], desde[1], desde[2]) * 5.0;
            let ray = Ray::new(fuera, -fuera.normalize());
            let hit = cubo_unitario().ray_intersect(&ray).expect("debe impactar");

            assert!((0.0..=1.0).contains(&hit.uv.x), "u = {}", hit.uv.x);
            assert!((0.0..=1.0).contains(&hit.uv.y), "v = {}", hit.uv.y);
        }
    }

    #[test]
    fn uv_recorre_la_cara_completa() {
        // Dos rayos hacia la cara +Z, uno cerca de la esquina inferior
        // izquierda y otro cerca de la superior derecha.
        let cubo = cubo_unitario();
        let direccion = Vec3::new(0.0, 0.0, -1.0);

        let bajo = cubo
            .ray_intersect(&Ray::new(Vec3::new(-0.9, -0.9, 5.0), direccion))
            .expect("debe impactar");
        let alto = cubo
            .ray_intersect(&Ray::new(Vec3::new(0.9, 0.9, 5.0), direccion))
            .expect("debe impactar");

        assert!(bajo.uv.x < 0.1 && bajo.uv.y < 0.1, "{:?}", bajo.uv);
        assert!(alto.uv.x > 0.9 && alto.uv.y > 0.9, "{:?}", alto.uv);
    }

    // --------------------------------------------- metrica mundo/uv

    /// Caja de `2 x 4 x 6`: las tres extensiones distintas, para que una
    /// confusion de ejes se vea en el numero y no solo en el signo.
    fn caja_desigual() -> Cuboid {
        Cuboid::centrado(Vec3::zeros(), Vec3::new(2.0, 4.0, 6.0))
    }

    /// Carta que reporta la cara alcanzada disparando desde `desde`.
    fn carta_desde(cubo: &Cuboid, desde: [f32; 3]) -> UvChart {
        let fuera = Vec3::new(desde[0], desde[1], desde[2]) * 20.0;
        let ray = Ray::new(fuera, -fuera.normalize());

        cubo.ray_intersect(&ray).expect("debe impactar").uv_chart
    }

    #[test]
    fn cada_cara_del_cuboide_reporta_las_longitudes_de_sus_tangentes() {
        // `uv_en_cara` fija que en la cara X `u` recorre Z y `v` recorre Y;
        // en la Y, `u` es X y `v` es Z; en la Z, `u` es X y `v` es Y. La
        // metrica tiene que decir exactamente esas longitudes.
        let cubo = caja_desigual();

        let esperado = [
            ([1.0, 0.0, 0.0], 6.0, 4.0),
            ([-1.0, 0.0, 0.0], 6.0, 4.0),
            ([0.0, 1.0, 0.0], 2.0, 6.0),
            ([0.0, -1.0, 0.0], 2.0, 6.0),
            ([0.0, 0.0, 1.0], 2.0, 4.0),
            ([0.0, 0.0, -1.0], 2.0, 4.0),
        ];

        for (desde, u, v) in esperado {
            let carta = carta_desde(&cubo, desde);
            let escala = cubo
                .uv_world_scale(carta)
                .unwrap_or_else(|| panic!("la cara {desde:?} deberia tener metrica"));

            assert!(
                (escala.u - u).abs() < 1e-5,
                "cara {desde:?}: u = {}",
                escala.u
            );
            assert!(
                (escala.v - v).abs() < 1e-5,
                "cara {desde:?}: v = {}",
                escala.v
            );
        }
    }

    #[test]
    fn una_carta_ajena_al_cuboide_no_tiene_metrica() {
        let cubo = caja_desigual();

        assert_eq!(cubo.uv_world_scale(UvChart::WHOLE), None);
        assert_eq!(cubo.uv_world_scale(UvChart::new(7)), None);
        assert_eq!(cubo.uv_world_scale(UvChart::new(200)), None);
    }

    #[test]
    fn una_caja_aplastada_no_inventa_una_escala() {
        // Extension nula en `Y`: las cuatro caras laterales tienen `v = 0`,
        // y una escala cero no es una metrica, es una division pendiente.
        let plana = Cuboid::centrado(Vec3::zeros(), Vec3::new(2.0, 0.0, 6.0));

        for carta in 1..=6u8 {
            let escala = plana.uv_world_scale(UvChart::new(carta));

            if let Some(escala) = escala {
                assert!(
                    escala.u > 0.0 && escala.v > 0.0,
                    "carta {carta}: {escala:?}"
                );
            }
        }

        // Las caras `+X` y `-X` son las cartas 1 y 2, y su `v` recorre `Y`.
        assert_eq!(plana.uv_world_scale(UvChart::new(1)), None);
        assert_eq!(plana.uv_world_scale(UvChart::new(2)), None);
    }

    #[test]
    fn dos_caras_opuestas_con_la_misma_uv_no_comparten_carta() {
        // Las caras `+X` y `-X` recorren las mismas `uv`: un punto y su
        // opuesto dan coordenadas identicas. Sin carta, una mascara
        // indexada por `uv` pintaria las dos a la vez, y la pintura
        // apareceria en la cara que el usuario no esta viendo.
        let cubo = cubo_unitario();

        let mas = cubo
            .ray_intersect(&Ray::new(
                Vec3::new(5.0, 0.3, 0.7),
                Vec3::new(-1.0, 0.0, 0.0),
            ))
            .expect("cara +X");
        let menos = cubo
            .ray_intersect(&Ray::new(
                Vec3::new(-5.0, 0.3, 0.7),
                Vec3::new(1.0, 0.0, 0.0),
            ))
            .expect("cara -X");

        assert_eq!(mas.uv, menos.uv, "el ejemplo exige uv identicas");
        assert_ne!(
            mas.uv_chart, menos.uv_chart,
            "dos caras opuestas no son la misma carta"
        );
    }

    #[test]
    fn las_seis_caras_dan_seis_cartas_distintas() {
        let mut vistas: Vec<UvChart> = Vec::new();

        for (desde, _) in CARAS {
            let fuera = Vec3::new(desde[0], desde[1], desde[2]) * 5.0;
            let hit = cubo_unitario()
                .ray_intersect(&Ray::new(fuera, -fuera.normalize()))
                .expect("debe impactar");

            assert!(
                !vistas.contains(&hit.uv_chart),
                "la cara {desde:?} repite la carta {:?}",
                hit.uv_chart
            );
            vistas.push(hit.uv_chart);
        }

        assert_eq!(vistas.len(), 6);
    }

    #[test]
    fn la_cara_de_salida_declara_su_propia_carta() {
        // Un rayo nacido dentro sale por `-Z`. Tiene que declarar la carta
        // de esa cara, no la de la que habria entrado.
        let cubo = cubo_unitario();
        let dentro = cubo
            .ray_intersect(&Ray::new(Vec3::zeros(), Vec3::new(0.0, 0.0, -1.0)))
            .expect("debe salir");
        let desde_fuera = cubo
            .ray_intersect(&Ray::new(
                Vec3::new(0.0, 0.0, -5.0),
                Vec3::new(0.0, 0.0, 1.0),
            ))
            .expect("cara -Z desde afuera");

        assert_eq!(dentro.uv_chart, desde_fuera.uv_chart);
    }

    #[test]
    fn rayo_interno_usa_la_cara_de_salida() {
        let ray = Ray::new(Vec3::zeros(), Vec3::new(0.0, 0.0, -1.0));
        let hit = cubo_unitario().ray_intersect(&ray).expect("debe salir");

        assert!((hit.distance - 1.0).abs() < 1e-5);
        assert!(!hit.front_face, "se toca desde adentro");
        // La normal exterior de la cara -Z es (0,0,-1); Hit la voltea para
        // dejarla contra el rayo, que viaja hacia -Z.
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn escala_no_uniforme_funciona() {
        // Caja de 2 x 4 x 6 centrada en el origen.
        let caja = Cuboid::centrado(Vec3::zeros(), Vec3::new(2.0, 4.0, 6.0));

        let por_x = caja
            .ray_intersect(&Ray::new(
                Vec3::new(5.0, 0.0, 0.0),
                Vec3::new(-1.0, 0.0, 0.0),
            ))
            .expect("debe impactar en X");
        let por_y = caja
            .ray_intersect(&Ray::new(
                Vec3::new(0.0, 5.0, 0.0),
                Vec3::new(0.0, -1.0, 0.0),
            ))
            .expect("debe impactar en Y");
        let por_z = caja
            .ray_intersect(&Ray::new(
                Vec3::new(0.0, 0.0, 5.0),
                Vec3::new(0.0, 0.0, -1.0),
            ))
            .expect("debe impactar en Z");

        assert!((por_x.distance - 4.0).abs() < 1e-5, "{}", por_x.distance);
        assert!((por_y.distance - 3.0).abs() < 1e-5, "{}", por_y.distance);
        assert!((por_z.distance - 2.0).abs() < 1e-5, "{}", por_z.distance);

        assert_eq!(por_x.normal, Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(por_y.normal, Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(por_z.normal, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn objeto_detras_de_la_camara_falla() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 1.0));

        assert!(cubo_unitario().ray_intersect(&ray).is_none());
    }

    #[test]
    fn rayo_que_pasa_de_lado_falla() {
        let ray = Ray::new(Vec3::new(3.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

        assert!(cubo_unitario().ray_intersect(&ray).is_none());
    }
}
