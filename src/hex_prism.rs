//! Prisma hexagonal regular vertical. Ruta A de la Tarea 7.3.
//!
//! # Por qué vertical y sin rotación libre
//!
//! Los pilares del Rompeolas son verticales por concepto. Una orientación
//! arbitraria obligaría a llevar el rayo a espacio local con una matriz y su
//! inversa en el camino más caliente del renderer, para algo que la escena no
//! pide. Si algún día hace falta girarlos, lo que entra es un `yaw` escalar en
//! XZ, no una transformación general.
//!
//! # La arista mira a `+X`
//!
//! El hexágono lleva una **arista** perpendicular a `+X`, no un vértice. Eso
//! fija todo lo demás: las seis normales laterales están en `k · 60°`, la
//! distancia de cada plano al centro es la **apotema**, y los vértices caen en
//! `30° + k · 60°` a distancia del circunradio.

use crate::bounds::Aabb;
use crate::hit::Hit;
use crate::ray::Ray;
use crate::ray_intersect::RayIntersect;
use crate::EPSILON;
use nalgebra_glm::{dot, Vec2, Vec3};

/// Prisma hexagonal regular de eje vertical.
#[derive(Debug, Clone, Copy)]
pub struct HexPrism {
    centro: Vec3,
    /// Circunradio: del centro a un vértice.
    radio: f32,
    /// Media altura en `Y`.
    media_altura: f32,
    /// Precalculado como en `Cuboid`: la jerarquía lo pide por objeto en
    /// cada construcción del árbol.
    bounds: Aabb,
}

/// Cuántas caras laterales tiene. Se escribe una vez y se usa en todas
/// partes: un `6` suelto en un bucle y otro en una división es como se
/// desincroniza una geometría.
pub const LADOS: usize = 6;

impl HexPrism {
    /// Prisma centrado, con circunradio y altura total.
    pub fn new(centro: Vec3, radio: f32, altura: f32) -> Self {
        let media_altura = altura * 0.5;
        let apotema = Self::apotema_de(radio);

        // Ajustado, no el circunradio en los dos ejes: con la arista hacia
        // `+X` el alcance en X es la apotema. Sigue conteniendo la forma, y
        // deja pasar menos rayos al test exacto.
        let medio = Vec3::new(apotema, media_altura, radio);

        HexPrism {
            centro,
            radio,
            media_altura,
            bounds: Aabb::new(centro - medio, centro + medio),
        }
    }

    /// Apotema: del centro al punto medio de una arista.
    fn apotema_de(radio: f32) -> f32 {
        radio * (std::f32::consts::PI / 6.0).cos()
    }

    /// UV del impacto, segun la cara que se toco.
    ///
    /// # Lateral: distancia sobre el perimetro
    ///
    /// `u` es cuanto se ha recorrido dando la vuelta al prisma, normalizado
    /// por el perimetro. En un hexagono regular el lado mide lo mismo que el
    /// circunradio, asi que la cara `k` ocupa el tramo `[k/6, (k+1)/6]` y su
    /// centro cae en `(k + 0.5) / 6`.
    ///
    /// Se hace asi y no con seis mapeos independientes porque la textura
    /// tiene que **dar la vuelta**: con mapeos por cara, cada arista seria
    /// una costura vertical.
    ///
    /// `v` es la fraccion en `Y`, igual que la cara lateral de un cuboide.
    ///
    /// # Tapas: proyeccion en XZ
    ///
    /// Normalizada al diametro y centrada, que es lo que hace `Cuboid` en su
    /// cara `Y`. Con la arista hacia `+X`, `u` no llega a tocar `0` ni `1`
    /// —la forma no alcanza el borde de su caja en ese eje— y `v` si.
    fn uv_en_cara(&self, punto: &Vec3, cara: usize) -> Vec2 {
        let relativo = punto - self.centro;

        if cara >= LADOS {
            let u = relativo.x / (2.0 * self.radio) + 0.5;
            let v = relativo.z / (2.0 * self.radio) + 0.5;

            return Vec2::new(u.clamp(0.0, 1.0), v.clamp(0.0, 1.0));
        }

        let angulo = std::f32::consts::TAU * cara as f32 / LADOS as f32;
        let tangente = Vec3::new(-angulo.sin(), 0.0, angulo.cos());

        // `s` va de `-lado/2` a `+lado/2`, y el lado de un hexagono regular
        // es su circunradio.
        let s = dot(&relativo, &tangente) / self.radio;
        let u = (cara as f32 + s + 0.5) / LADOS as f32;
        let v = (relativo.y + self.media_altura) / (2.0 * self.media_altura);

        Vec2::new(u.clamp(0.0, 1.0), v.clamp(0.0, 1.0))
    }

    pub fn bounds(&self) -> Aabb {
        self.bounds
    }

    /// Normal exterior de la cara lateral `k`.
    fn normal_lateral(k: usize) -> Vec3 {
        let angulo = std::f32::consts::TAU * k as f32 / LADOS as f32;

        Vec3::new(angulo.cos(), 0.0, angulo.sin())
    }
}

/// Los ocho semiespacios, como `(normal exterior, distancia al centro)`.
///
/// Un AABB son tres pares de planos y `Aabb::hit` los resuelve como slabs
/// alineados a ejes; esto es lo mismo con cuatro pares, dos de ellos
/// oblicuos. Por eso no se puede reutilizar `Aabb::hit`: su intervalo viene
/// indexado por eje, y aquí las caras no son ejes.
impl HexPrism {
    fn planos(&self) -> [(Vec3, f32); LADOS + 2] {
        let apotema = Self::apotema_de(self.radio);
        let mut planos = [(Vec3::zeros(), 0.0); LADOS + 2];

        for (k, plano) in planos.iter_mut().enumerate().take(LADOS) {
            *plano = (Self::normal_lateral(k), apotema);
        }

        planos[LADOS] = (Vec3::new(0.0, 1.0, 0.0), self.media_altura);
        planos[LADOS + 1] = (Vec3::new(0.0, -1.0, 0.0), self.media_altura);

        planos
    }
}

impl RayIntersect for HexPrism {
    fn ray_intersect(&self, ray: &Ray) -> Option<Hit> {
        let relativo = ray.origin - self.centro;

        let mut t_enter = f32::NEG_INFINITY;
        let mut t_exit = f32::INFINITY;
        let mut cara_enter = 0usize;
        let mut cara_exit = 0usize;

        let planos = self.planos();

        for (i, (normal, distancia)) in planos.iter().enumerate() {
            let denominador = dot(&ray.direction, normal);
            let holgura = distancia - dot(&relativo, normal);

            // Paralelo al plano: no hay `t` que calcular. Solo importa de
            // qué lado está el origen. Dividir aquí daría un infinito con
            // signo que contaminaría el intervalo.
            if denominador.abs() < f32::EPSILON {
                if holgura < 0.0 {
                    return None;
                }

                continue;
            }

            let t = holgura / denominador;

            if denominador < 0.0 {
                // El rayo entra por este plano.
                if t > t_enter {
                    t_enter = t;
                    cara_enter = i;
                }
            } else if t < t_exit {
                t_exit = t;
                cara_exit = i;
            }

            if t_enter > t_exit {
                return None;
            }
        }

        // Nacido dentro: lo que se va a tocar es la cara de salida. Es el
        // caso del rayo refractado, y la política es la misma que la de
        // `Cuboid`.
        let desde_adentro = t_enter < EPSILON;

        let (t, cara) = if desde_adentro {
            (t_exit, cara_exit)
        } else {
            (t_enter, cara_enter)
        };

        if t <= EPSILON || !t.is_finite() {
            return None;
        }

        let punto = ray.at(t);
        let (normal, _) = planos[cara];

        Some(Hit::new(ray, t, normal, self.uv_en_cara(&punto, cara)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hit::Hit;
    use crate::ray::Ray;
    use crate::ray_intersect::RayIntersect;
    use nalgebra_glm::Vec3;

    const CENTRO: Vec3 = Vec3::new(1.0, 2.0, -3.0);
    const RADIO: f32 = 0.8;
    const ALTURA: f32 = 3.0;

    fn prisma() -> HexPrism {
        HexPrism::new(CENTRO, RADIO, ALTURA)
    }

    #[test]
    fn el_aabb_ajusta_apotema_en_x_y_circunradio_en_z() {
        // El hexagono lleva una **arista** perpendicular a `+X`, asi que su
        // alcance en X es la apotema y en Z el circunradio. Un AABB con el
        // circunradio en los dos ejes tambien seria valido, pero dejaria
        // pasar al test exacto rayos que no pueden tocar nada.
        let caja = prisma().bounds();
        let apotema = RADIO * (30.0_f32).to_radians().cos();

        assert!((caja.min.x - (CENTRO.x - apotema)).abs() < 1e-5, "{caja:?}");
        assert!((caja.max.x - (CENTRO.x + apotema)).abs() < 1e-5, "{caja:?}");
        assert!((caja.min.z - (CENTRO.z - RADIO)).abs() < 1e-5, "{caja:?}");
        assert!((caja.max.z - (CENTRO.z + RADIO)).abs() < 1e-5, "{caja:?}");
        assert!(
            (caja.min.y - (CENTRO.y - ALTURA * 0.5)).abs() < 1e-5,
            "{caja:?}"
        );
        assert!(
            (caja.max.y - (CENTRO.y + ALTURA * 0.5)).abs() < 1e-5,
            "{caja:?}"
        );
    }

    #[test]
    fn el_aabb_contiene_los_seis_vertices_y_las_dos_tapas() {
        // La condicion que la jerarquia de aceleracion necesita: si un punto
        // del solido cayera fuera de la caja, el recorrido podria podar un
        // objeto que si se toca.
        let caja = prisma().bounds();

        for k in 0..6 {
            let angulo = (30.0 + 60.0 * k as f32).to_radians();

            for lado in [-1.0_f32, 1.0] {
                let vertice = CENTRO
                    + Vec3::new(
                        RADIO * angulo.cos(),
                        lado * ALTURA * 0.5,
                        RADIO * angulo.sin(),
                    );

                assert!(caja.contiene(&vertice), "vertice {k} {lado}: {vertice:?}");
            }
        }
    }

    /// Rayo que apunta al centro desde `origen`.
    fn hacia_el_centro(origen: Vec3) -> Ray {
        Ray::new(origen, (CENTRO - origen).normalize())
    }

    #[test]
    fn un_impacto_lateral_devuelve_la_normal_de_su_cara() {
        // Cara `k = 0`: la arista perpendicular a `+X`. Se dispara de frente
        // a la altura del centro, asi que el impacto cae en el plano de la
        // apotema y no en un vertice.
        let apotema = RADIO * (30.0_f32).to_radians().cos();
        let ray = hacia_el_centro(CENTRO + Vec3::new(5.0, 0.0, 0.0));

        let hit = prisma().ray_intersect(&ray).expect("la cara +X se toca");

        assert!((hit.distance - (5.0 - apotema)).abs() < 1e-4, "{hit:?}");
        assert!(
            (hit.normal - Vec3::new(1.0, 0.0, 0.0)).magnitude() < 1e-5,
            "{hit:?}"
        );
        assert!(hit.front_face);
    }

    #[test]
    fn un_impacto_en_la_tapa_devuelve_la_normal_vertical() {
        let ray = hacia_el_centro(CENTRO + Vec3::new(0.0, 5.0, 0.0));

        let hit = prisma().ray_intersect(&ray).expect("la tapa se toca");

        assert!(
            (hit.distance - (5.0 - ALTURA * 0.5)).abs() < 1e-4,
            "{hit:?}"
        );
        assert!(
            (hit.normal - Vec3::new(0.0, 1.0, 0.0)).magnitude() < 1e-5,
            "{hit:?}"
        );
        assert!(hit.front_face);
    }

    #[test]
    fn un_rayo_paralelo_y_fuera_no_toca_nada() {
        // Paralelo a la tapa y por encima de ella: el denominador de ese
        // plano es cero y el codigo tiene que descartar sin dividir.
        let fuera = Ray::new(
            CENTRO + Vec3::new(-5.0, ALTURA, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        );

        assert!(prisma().ray_intersect(&fuera).is_none());

        // Y paralelo a una cara lateral, por fuera de su plano.
        let apotema = RADIO * (30.0_f32).to_radians().cos();
        let lateral = Ray::new(
            CENTRO + Vec3::new(apotema + 0.5, 0.0, -5.0),
            Vec3::new(0.0, 0.0, 1.0),
        );

        assert!(prisma().ray_intersect(&lateral).is_none());
    }

    #[test]
    fn un_rayo_paralelo_y_dentro_si_atraviesa() {
        // El mismo caso de denominador cero, pero con el origen dentro de la
        // banda: ese plano no acota y el rayo tiene que seguir probandose
        // contra los demas. Descartarlo seria un agujero en el solido.
        let dentro = Ray::new(CENTRO + Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));

        assert!(prisma().ray_intersect(&dentro).is_some());
    }

    #[test]
    fn un_rayo_nacido_dentro_sale_por_la_cara_de_salida() {
        // Es el caso del rayo refractado. La normal exterior de la cara de
        // salida apunta en el sentido de la marcha, asi que `Hit` la voltea
        // y marca `front_face` falso: es lo que Fresnel necesita para elegir
        // la razon de indices.
        let ray = Ray::new(CENTRO, Vec3::new(1.0, 0.0, 0.0));
        let apotema = RADIO * (30.0_f32).to_radians().cos();

        let hit = prisma().ray_intersect(&ray).expect("sale por la cara +X");

        assert!((hit.distance - apotema).abs() < 1e-4, "{hit:?}");
        assert!(!hit.front_face, "se toca desde dentro");
        assert!(
            (hit.normal - Vec3::new(-1.0, 0.0, 0.0)).magnitude() < 1e-5,
            "{hit:?}"
        );
    }

    #[test]
    fn las_ocho_caras_devuelven_su_normal() {
        // Las seis laterales y las dos tapas, cada una disparada de frente.
        let apotema = RADIO * (30.0_f32).to_radians().cos();
        let mut caras: Vec<Vec3> = (0..LADOS)
            .map(|k| {
                let angulo = std::f32::consts::TAU * k as f32 / LADOS as f32;
                Vec3::new(angulo.cos(), 0.0, angulo.sin())
            })
            .collect();
        caras.push(Vec3::new(0.0, 1.0, 0.0));
        caras.push(Vec3::new(0.0, -1.0, 0.0));

        for normal in caras {
            let ray = hacia_el_centro(CENTRO + normal * 5.0);
            let hit = prisma()
                .ray_intersect(&ray)
                .unwrap_or_else(|| panic!("la cara {normal:?} debe impactar"));

            assert!(
                (hit.normal - normal).magnitude() < 1e-4,
                "cara {normal:?} devolvio {:?}",
                hit.normal
            );
            assert!(hit.front_face, "cara {normal:?} se toca desde afuera");

            // Distancia esperada: apotema para las laterales, media altura
            // para las tapas.
            let esperada = if normal.y.abs() > 0.5 {
                5.0 - ALTURA * 0.5
            } else {
                5.0 - apotema
            };
            assert!((hit.distance - esperada).abs() < 1e-4, "cara {normal:?}");
        }
    }

    /// Punto sobre la cara `k`, a `s` del centro de la cara y a la altura
    /// del centro del prisma.
    fn punto_en_cara(k: usize, s: f32) -> Vec3 {
        let angulo = std::f32::consts::TAU * k as f32 / LADOS as f32;
        let normal = Vec3::new(angulo.cos(), 0.0, angulo.sin());
        let tangente = Vec3::new(-angulo.sin(), 0.0, angulo.cos());
        let apotema = RADIO * (30.0_f32).to_radians().cos();

        CENTRO + normal * apotema + tangente * s
    }

    /// Dispara perpendicularmente contra ese punto, desde fuera.
    fn disparar_a(k: usize, s: f32) -> Hit {
        let angulo = std::f32::consts::TAU * k as f32 / LADOS as f32;
        let normal = Vec3::new(angulo.cos(), 0.0, angulo.sin());
        let destino = punto_en_cara(k, s);

        let ray = Ray::new(destino + normal * 3.0, -normal);

        prisma()
            .ray_intersect(&ray)
            .expect("el punto esta en la cara")
    }

    #[test]
    fn el_uv_lateral_recorre_el_perimetro() {
        // `u` es distancia recorrida sobre el perimetro, normalizada. El
        // centro de la cara `k` cae por tanto en `(k + 0.5) / 6`, y eso es
        // lo que hace que la textura de la vuelta sin saltar de cara en
        // cara.
        for k in 0..LADOS {
            let hit = disparar_a(k, 0.0);
            let esperado = (k as f32 + 0.5) / LADOS as f32;

            assert!(
                (hit.uv.x - esperado).abs() < 1e-4,
                "cara {k}: u = {} y se esperaba {esperado}",
                hit.uv.x
            );
            assert!((hit.uv.y - 0.5).abs() < 1e-4, "cara {k}: v = {}", hit.uv.y);
        }
    }

    #[test]
    fn el_uv_lateral_es_continuo_en_la_arista() {
        // Las dos caras que comparten el vertice de `30 grados`, cada una
        // mirada desde su lado. Si `u` saltara ahi, la textura mostraria una
        // costura vertical en cada arista del prisma.
        let delta = RADIO * 1e-3;
        let desde_la_cara_0 = disparar_a(0, RADIO * 0.5 - delta);
        let desde_la_cara_1 = disparar_a(1, -RADIO * 0.5 + delta);

        let salto = (desde_la_cara_0.uv.x - desde_la_cara_1.uv.x).abs();

        assert!(
            salto < 1e-3,
            "la arista salta {salto}: {} contra {}",
            desde_la_cara_0.uv.x,
            desde_la_cara_1.uv.x
        );
    }

    #[test]
    fn el_uv_de_las_tapas_cae_en_cero_uno() {
        // La tapa se mapea por proyeccion en XZ normalizada al diametro.
        // Con la arista hacia `+X`, `u` no llega a los extremos y `v` si;
        // lo que se exige es que ninguno se salga.
        let apotema = RADIO * (30.0_f32).to_radians().cos();

        for (dx, dz) in [
            (0.0_f32, 0.0_f32),
            (apotema * 0.9, 0.0),
            (-apotema * 0.9, 0.0),
            (0.0, RADIO * 0.9),
            (0.0, -RADIO * 0.9),
        ] {
            let destino = CENTRO + Vec3::new(dx, ALTURA * 0.5, dz);
            let ray = Ray::new(
                destino + Vec3::new(0.0, 3.0, 0.0),
                Vec3::new(0.0, -1.0, 0.0),
            );
            let hit = prisma().ray_intersect(&ray).expect("la tapa se toca");

            assert!(
                (0.0..=1.0).contains(&hit.uv.x) && (0.0..=1.0).contains(&hit.uv.y),
                "tapa en ({dx}, {dz}): uv = {:?}",
                hit.uv
            );
        }

        // Y el centro de la tapa cae en el centro de la textura.
        let ray = Ray::new(CENTRO + Vec3::new(0.0, 3.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        let hit = prisma().ray_intersect(&ray).expect("la tapa se toca");

        assert!((hit.uv.x - 0.5).abs() < 1e-4, "{:?}", hit.uv);
        assert!((hit.uv.y - 0.5).abs() < 1e-4, "{:?}", hit.uv);
    }
}
