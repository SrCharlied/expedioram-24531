//! La cuenca del Monolito en la entrega: la composición aprobada.
//!
//! Es la variante `flooded_blue_raised` del preview
//! (`examples/monolith_basin_preview.rs`), aprobada por Charlie como default.
//! Este módulo es su versión de producción: el mismo resultado, sin el arnés
//! de comparación ni las variantes descartadas. Un test del preview
//! comprueba con los assets reales que la entrega que sale de aquí es la
//! escena aprobada —geometría, materiales efectivos, texturas texel a texel,
//! jerarquía y renders—.
//!
//! # Qué hace
//!
//! `aplicar`, sobre la entrega ya compuesta y con `A-01` todavía dentro:
//!
//! 1. **Alarga las cuatro caídas exteriores de Praderas** hasta lo que cada
//!    una tiene debajo, en su misma planta, y las hunde `EMPOTRADO` en ello:
//!    tres llegan al plinto y la cuarta a la masa de fondo que tiene debajo.
//!    Las dos interiores ya desembocan y no se tocan. Ver `prolongar_caidas`.
//! 2. **Mide la base libre**: el techo del plinto menos su `BORDE` y menos la
//!    planta de toda pieza que haya sobre él —masas, islas, Rompeolas,
//!    Aguas, la meseta flotante y lo que cuelga de ella, el Monolito—, con
//!    `HOLGURA`, o `HOLGURA_CAIDA` para las caídas. La reparte en
//!    rectángulos de interiores disjuntos y descarta las astillas, salvo la
//!    desembocadura de una caída. Ver `huella_libre`.
//! 3. **Añade al final** `CELDAS` volúmenes de agua de techo común `NIVEL`
//!    y `LECHOS` piezas de lecho, retiradas `RETRANQUEO_DEL_LECHO` del borde
//!    del agua. Grupo espacial `Global`, revelación `Meadows`, nacen en el
//!    lienzo como las caídas.
//!
//! Se mide en tiempo de ejecución, así que vale para la Ruta A y la B, cuyas
//! masas del Rompeolas quedan `0.15` más al fondo en la B.
//!
//! # Materiales
//!
//! Un agua y un fondo por pieza de lecho, añadidos al final de la paleta;
//! ningún material ni textura compartidos se modifican.
//!
//! - **El agua** es el `A-01` del preset con `BRILLO` y el tinte cian marino
//!   `TINTE`. En la entrega refractiva —la aprobada— conserva sus techos
//!   `0.9 / 0.9`, su `ior 1.333`, su sombra y su textura. En el control
//!   `OpaqueWater` sigue al control y queda sin óptica, como `A-01`.
//! - **El fondo**: cada pieza de lecho lleva su material opaco y su textura
//!   procedural, muestreada **en coordenadas de mundo** sobre un único campo
//!   de piedras azul pizarra (`campo_de_roca`) a `TEXELS_POR_UNIDAD`. La `uv`
//!   de un cuboide va normalizada por cara: una textura compartida se
//!   estiraría distinto en cada pieza.
//!
//! # El presupuesto
//!
//! `168 + 19 + 18 = 205`. Es experimental en rendimiento: ver la evidencia.
//!
//! # El límite conocido
//!
//! El renderer decide entrar o salir del agua por `front_face`, sin pila de
//! medios: el rayo refractado que llega a la cara que comparten dos celdas
//! «sale» al aire. Con `NIVEL` le pasa a menos del `2 %` de los píxeles de
//! agua; más agua lo empeora. Lo acota un test.

use crate::bounds::Aabb;
use crate::color::Color;
use crate::cuboid::Cuboid;
use crate::material::Material;
use crate::scene::{MaterialId, RevealGroup, Scene, SceneObject, SpatialGroupId};
use crate::texture::Texture;
use nalgebra_glm::Vec3;

// ---------------------------------------------------------------------
// Constantes aprobadas
// ---------------------------------------------------------------------

/// Celdas de agua y piezas de lecho de la cuenca aprobada.
pub const CELDAS: usize = 19;
pub const LECHOS: usize = 18;
/// Celdas de agua y piezas de lecho del relleno bajo Praderas.
pub const RELLENO_AGUA: usize = 3;
pub const RELLENO_LECHO: usize = 6;
/// Celdas de agua y piezas de lecho del relleno tras las masas.
pub const TRASERO_AGUA: usize = 2;
pub const TRASERO_LECHO: usize = 2;
/// Hueco vertical mínimo, sobre el agua, que deja una pieza elevada de
/// Praderas para que la cuenca pueda pasar por debajo.
pub const HUECO_BAJO_PRADERAS: f32 = 0.50;

/// Lo que una pieza se hunde en su apoyo: separa caras coplanares.
pub const EMPOTRADO: f32 = 0.04;
/// Techo común del agua sobre el plinto.
pub const NIVEL: f32 = 0.30;
/// Holgura en planta del agua a las piezas opacas.
pub const HOLGURA: f32 = 0.03;
/// Holgura a las caídas: sin contacto, pero lo bastante cerca para que
/// desemboquen.
pub const HOLGURA_CAIDA: f32 = 0.02;
/// Lo que el agua deja seco junto al canto del plinto.
pub const BORDE: f32 = 0.05;
/// Lo que el lecho se retira del borde del agua.
pub const RETRANQUEO_DEL_LECHO: f32 = 0.02;
/// Celdas más estrechas o más pequeñas que esto son astillas.
pub const ANCHO_MINIMO: f32 = 0.25;
pub const AREA_MINIMA: f32 = 0.30;
/// Los trozos de lecho más pequeños se omiten.
pub const AREA_MINIMA_DEL_LECHO: f32 = 0.25;
/// Lado mínimo de una celda que recibe una caída.
pub const ANCHO_DE_DESEMBOCADURA: f32 = 0.10;
/// El lecho, entre estas dos cotas sobre el plinto.
pub const LECHO: (f32, f32) = (-0.02, 0.03);
/// Densidad de las texturas del lecho, en texels por unidad de mundo.
pub const TEXELS_POR_UNIDAD: f32 = 24.0;

/// Tinte del agua, en sRGB: un cian marino claro.
pub const TINTE: [f32; 3] = [0.62, 0.88, 1.00];
/// Brillo especular directo del agua: `(fuerza, exponente)`.
pub const BRILLO: (f32, f32) = (0.22, 40.0);

/// Lado de las piedras del fondo, en unidades de mundo.
const PIEDRA: f32 = 0.42;
/// Los tres tonos del fondo, en sRGB.
const GRIETA: [f32; 3] = [0.17, 0.27, 0.37];
const PIEDRA_OSCURA: [f32; 3] = [0.28, 0.42, 0.55];
const PIEDRA_CLARA: [f32; 3] = [0.40, 0.57, 0.69];

/// Lo que se le pide a una pieza para contar como apoyo de una caída:
/// cubrir al menos la mitad de su planta.
const APOYO_MINIMO: f32 = 0.5;
const EPS: f32 = 1.0e-4;

// ---------------------------------------------------------------------
// El fondo de piedra
// ---------------------------------------------------------------------

fn azar(x: i32, z: i32, semilla: u32) -> f32 {
    let mut h =
        semilla ^ (x as u32).wrapping_mul(0x27D4_EB2D) ^ (z as u32).wrapping_mul(0x1656_67B1);
    h ^= h >> 15;
    h = h.wrapping_mul(0x2C1B_3C6D);
    h ^= h >> 12;
    h = h.wrapping_mul(0x297A_2D39);
    h ^= h >> 15;
    (h >> 8) as f32 / (1u32 << 24) as f32
}

/// Ruido de valor suave en el plano, de periodo `escala`.
fn ruido_de_valor(x: f32, z: f32, escala: f32, semilla: u32) -> f32 {
    let (u, v) = (x / escala, z / escala);
    let (x0, z0) = (u.floor(), v.floor());
    let suave = |t: f32| t * t * (3.0 - 2.0 * t);
    let (sx, sz) = (suave(u - x0), suave(v - z0));
    let en = |dx: i32, dz: i32| azar(x0 as i32 + dx, z0 as i32 + dz, semilla);
    let a = en(0, 0) + (en(1, 0) - en(0, 0)) * sx;
    let b = en(0, 1) + (en(1, 1) - en(0, 1)) * sx;
    a + (b - a) * sz
}

fn mezclar(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    let t = t.clamp(0.0, 1.0);
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

/// El fondo rocoso, en coordenadas de mundo: piedras de Voronoi de unos
/// `PIEDRA` de lado con un tono cada una, grietas de fuerza variable y
/// manchas amplias.
pub fn campo_de_roca(x: f32, z: f32) -> Color {
    let (u, v) = (x / PIEDRA, z / PIEDRA);
    let (cu, cv) = (u.floor() as i32, v.floor() as i32);
    let (mut d1, mut d2, mut tono) = (f32::MAX, f32::MAX, 0.0);
    for dz in -1..=1 {
        for dx in -1..=1 {
            let (gx, gz) = (cu + dx, cv + dz);
            let px = gx as f32 + 0.15 + 0.7 * azar(gx, gz, 0xB10E_0001);
            let pz = gz as f32 + 0.15 + 0.7 * azar(gx, gz, 0xB10E_0002);
            let d = ((u - px).powi(2) + (v - pz).powi(2)).sqrt();
            if d < d1 {
                d2 = d1;
                d1 = d;
                tono = azar(gx, gz, 0xB10E_0003);
            } else if d < d2 {
                d2 = d;
            }
        }
    }

    let mancha = 0.6 * ruido_de_valor(x, z, 1.7, 0xB10E_0004)
        + 0.4 * ruido_de_valor(x, z, 0.55, 0xB10E_0005);
    let piedra = mezclar(
        PIEDRA_OSCURA,
        PIEDRA_CLARA,
        0.15 + 0.5 * tono + 0.35 * mancha,
    );
    let borde = ((d2 - d1) / 0.09).clamp(0.0, 1.0);
    let fuerza = 0.35 + 0.65 * ruido_de_valor(x, z, 1.1, 0xB10E_0006);
    let grieta = (1.0 - borde * borde * (3.0 - 2.0 * borde)) * fuerza;
    let [r, g, b] = mezclar(piedra, GRIETA, grieta);
    Color::from_srgb(r, g, b)
}

/// La textura de una pieza de lecho: `campo_de_roca` en el centro de cada
/// texel de su cara superior. Esa cara recorre `x` en `u` y `z` en `v`, y la
/// fila `0` de la textura es `v = 1`.
fn textura_de_fondo(b: &Aabb) -> Texture {
    let (w, h) = (b.max.x - b.min.x, b.max.z - b.min.z);
    let ancho = ((w * TEXELS_POR_UNIDAD).round() as usize).max(2);
    let alto = ((h * TEXELS_POR_UNIDAD).round() as usize).max(2);
    let mut pixeles = Vec::with_capacity(ancho * alto);
    for fila in 0..alto {
        let v = 1.0 - (fila as f32 + 0.5) / alto as f32;
        for col in 0..ancho {
            let u = (col as f32 + 0.5) / ancho as f32;
            pixeles.push(campo_de_roca(b.min.x + u * w, b.min.z + v * h));
        }
    }
    Texture::from_pixels(ancho, alto, pixeles).expect("dimensiones validas")
}

// ---------------------------------------------------------------------
// Las caídas
// ---------------------------------------------------------------------

/// Índices de escena de las seis caídas de Praderas: sus piezas de agua, en
/// orden de índice.
pub fn caidas(scene: &Scene) -> Vec<usize> {
    let caidas: Vec<usize> = (0..scene.objects.len())
        .filter(|&i| es_caida(scene, i))
        .collect();
    assert_eq!(caidas.len(), 6, "Praderas tiene seis caidas de agua");
    caidas
}

/// ¿Es `j` una caída de Praderas? El cristal del Monolito también refracta,
/// pero es un bloque y lleva la holgura de los bloques.
fn es_caida(scene: &Scene, j: usize) -> bool {
    scene.objects[j].spatial_group == SpatialGroupId::Meadows
        && scene.material(scene.objects[j].final_material).ior > 1.0
}

/// Fracción de la planta de `a` que cubre la de `b`.
fn cubierta(a: &Aabb, b: &Aabb) -> f32 {
    let dx = (a.max.x.min(b.max.x) - a.min.x.max(b.min.x)).max(0.0);
    let dz = (a.max.z.min(b.max.z) - a.min.z.max(b.min.z)).max(0.0);
    dx * dz / ((a.max.x - a.min.x) * (a.max.z - a.min.z))
}

/// Alarga cada caída que termina en el aire hasta la superficie que tiene
/// debajo, en su misma planta, y la hunde `EMPOTRADO` en ella. No se fuerza
/// un plano común; una caída que ya apoya no se toca.
fn prolongar_caidas(scene: &mut Scene) {
    for i in caidas(scene) {
        let b = scene.objects[i].primitive.bounds();
        let opacas =
            |j: usize| j != i && scene.material(scene.objects[j].final_material).ior <= 1.0;
        let apoya = (0..scene.objects.len()).filter(|&j| opacas(j)).any(|j| {
            let o = scene.objects[j].primitive.bounds();
            cubierta(&b, &o) >= APOYO_MINIMO && o.min.y - EPS <= b.min.y && b.min.y <= o.max.y + EPS
        });
        if apoya {
            continue;
        }

        let techo = (0..scene.objects.len())
            .filter(|&j| opacas(j))
            .filter_map(|j| {
                let o = scene.objects[j].primitive.bounds();
                (cubierta(&b, &o) >= APOYO_MINIMO && o.max.y <= b.min.y + EPS).then_some(o.max.y)
            })
            .max_by(f32::total_cmp)
            .expect("bajo cada caida hay al menos el plinto");

        let pie = techo - EMPOTRADO;
        scene.objects[i].primitive =
            Cuboid::new(Aabb::new(Vec3::new(b.min.x, pie, b.min.z), b.max)).into();
    }
}

// ---------------------------------------------------------------------
// La huella
// ---------------------------------------------------------------------

/// Una planta `[x0, z0, x1, z1]`.
type Planta = [f32; 4];

fn area(p: &Planta) -> f32 {
    (p[2] - p[0]) * (p[3] - p[1])
}

/// Cortes ordenados y sin repetir.
fn cortes(valores: impl Iterator<Item = f32>) -> Vec<f32> {
    let mut v: Vec<f32> = valores.collect();
    v.sort_by(f32::total_cmp);
    v.dedup();
    v
}

/// Fusiona las celdas marcadas de una rejilla en rectángulos de interiores
/// disjuntos, el mayor primero; a igual área gana el primero encontrado.
fn fusionar(xs: &[f32], zs: &[f32], marcada: &[Vec<bool>]) -> Vec<Planta> {
    let (nx, nz) = (xs.len() - 1, zs.len() - 1);
    let mut libre: Vec<Vec<bool>> = marcada.to_vec();
    let mut rects = Vec::new();

    loop {
        let mut racha = vec![vec![0usize; nz + 1]; nx];
        for (i, columna) in racha.iter_mut().enumerate() {
            for k in (0..nz).rev() {
                columna[k] = if libre[i][k] { columna[k + 1] + 1 } else { 0 };
            }
        }

        let mut mejor: Option<(f32, usize, usize, usize, usize)> = None;
        for i in 0..nx {
            for k in 0..nz {
                let mut alto = usize::MAX;
                for (j, columna) in racha.iter().enumerate().skip(i) {
                    alto = alto.min(columna[k]);
                    if alto == 0 {
                        break;
                    }
                    let a = (xs[j + 1] - xs[i]) * (zs[k + alto] - zs[k]);
                    if mejor.is_none_or(|m| a > m.0) {
                        mejor = Some((a, i, k, j, k + alto - 1));
                    }
                }
            }
        }

        let Some((_, i, k, j, q)) = mejor else {
            break;
        };
        for columna in &mut libre[i..=j] {
            for celda in &mut columna[k..=q] {
                *celda = false;
            }
        }
        rects.push([xs[i], zs[k], xs[j + 1], zs[q + 1]]);
    }

    rects
}

/// Las celdas de agua: la base libre del plinto `plinto`, repartida.
fn huella_libre(scene: &Scene, plinto: usize) -> Vec<Planta> {
    let p = scene.objects[plinto].primitive.bounds();
    let base = [
        p.min.x + BORDE,
        p.min.z + BORDE,
        p.max.x - BORDE,
        p.max.z - BORDE,
    ];

    let todos: Vec<Planta> = (0..scene.objects.len())
        .filter(|&j| j != plinto)
        .map(|j| {
            let b = scene.objects[j].primitive.bounds();
            let h = if es_caida(scene, j) {
                HOLGURA_CAIDA
            } else {
                HOLGURA
            };
            [b.min.x - h, b.min.z - h, b.max.x + h, b.max.z + h]
        })
        .collect();

    // Una planta contenida en otra no cambia la huella, pero sus cortes
    // partirían las celdas vecinas; de dos iguales se queda la primera.
    let contiene =
        |a: &Planta, b: &Planta| a[0] <= b[0] && a[1] <= b[1] && b[2] <= a[2] && b[3] <= a[3];
    let obstaculos: Vec<Planta> = (0..todos.len())
        .filter(|&i| {
            !(0..todos.len()).any(|j| {
                j != i && contiene(&todos[j], &todos[i]) && (todos[i] != todos[j] || j < i)
            })
        })
        .map(|i| todos[i])
        .collect();

    let recorte = |v: f32, a: f32, b: f32| v.clamp(a, b);
    let xs = cortes(
        [base[0], base[2]]
            .into_iter()
            .chain(obstaculos.iter().flat_map(|o| [o[0], o[2]]))
            .map(|v| recorte(v, base[0], base[2])),
    );
    let zs = cortes(
        [base[1], base[3]]
            .into_iter()
            .chain(obstaculos.iter().flat_map(|o| [o[1], o[3]]))
            .map(|v| recorte(v, base[1], base[3])),
    );

    let libre: Vec<Vec<bool>> = (0..xs.len() - 1)
        .map(|i| {
            let cx = 0.5 * (xs[i] + xs[i + 1]);
            (0..zs.len() - 1)
                .map(|k| {
                    let cz = 0.5 * (zs[k] + zs[k + 1]);
                    !obstaculos
                        .iter()
                        .any(|o| o[0] < cx && cx < o[2] && o[1] < cz && cz < o[3])
                })
                .collect()
        })
        .collect();

    // Las caídas que ya llegan al plinto: el agua que tienen delante es su
    // desembocadura y se conserva aunque sea estrecha.
    let piso = p.max.y;
    let receptoras: Vec<Aabb> = caidas(scene)
        .into_iter()
        .map(|i| scene.objects[i].primitive.bounds())
        .filter(|b| b.min.y < piso)
        .collect();
    let desemboca = |r: &Planta| {
        receptoras.iter().any(|b| {
            let hueco = r[1] - b.max.z;
            hueco > 0.0
                && hueco <= HOLGURA_CAIDA + 1.0e-4
                && r[0] < b.max.x
                && b.min.x < r[2]
                && (r[2] - r[0]).min(r[3] - r[1]) >= ANCHO_DE_DESEMBOCADURA
        })
    };

    fusionar(&xs, &zs, &libre)
        .into_iter()
        .filter(|r| {
            (r[2] - r[0]).min(r[3] - r[1]) >= ANCHO_MINIMO && area(r) >= AREA_MINIMA || desemboca(r)
        })
        .collect()
}

/// El lecho: la unión de las celdas retirada `RETRANQUEO_DEL_LECHO` hacia
/// dentro, en rectángulos de interiores disjuntos.
fn lechos(celdas: &[Planta]) -> Vec<Planta> {
    let r = RETRANQUEO_DEL_LECHO;
    let gx = cortes(celdas.iter().flat_map(|c| [c[0], c[2]]));
    let gz = cortes(celdas.iter().flat_map(|c| [c[1], c[3]]));
    let union: Vec<Vec<bool>> = (0..gx.len() - 1)
        .map(|i| {
            let cx = 0.5 * (gx[i] + gx[i + 1]);
            (0..gz.len() - 1)
                .map(|k| {
                    let cz = 0.5 * (gz[k] + gz[k + 1]);
                    celdas
                        .iter()
                        .any(|c| c[0] < cx && cx < c[2] && c[1] < cz && cz < c[3])
                })
                .collect()
        })
        .collect();

    let cubierto = |a0: f32, a1: f32, b0: f32, b1: f32| {
        let is: Vec<usize> = (0..gx.len() - 1)
            .filter(|&i| gx[i + 1] > a0 && gx[i] < a1)
            .collect();
        let ks: Vec<usize> = (0..gz.len() - 1)
            .filter(|&k| gz[k + 1] > b0 && gz[k] < b1)
            .collect();
        a0 >= gx[0]
            && a1 <= gx[gx.len() - 1]
            && b0 >= gz[0]
            && b1 <= gz[gz.len() - 1]
            && is.iter().all(|&i| ks.iter().all(|&k| union[i][k]))
    };

    let xs = cortes(celdas.iter().flat_map(|c| [c[0] + r, c[2] - r, c[0], c[2]]));
    let zs = cortes(celdas.iter().flat_map(|c| [c[1] + r, c[3] - r, c[1], c[3]]));
    let marcada: Vec<Vec<bool>> = (0..xs.len() - 1)
        .map(|i| {
            (0..zs.len() - 1)
                .map(|k| cubierto(xs[i] - r, xs[i + 1] + r, zs[k] - r, zs[k + 1] + r))
                .collect()
        })
        .collect();

    fusionar(&xs, &zs, &marcada)
        .into_iter()
        .filter(|p| (p[2] - p[0]).min(p[3] - p[1]) >= 0.05 && area(p) >= AREA_MINIMA_DEL_LECHO)
        .collect()
}

// ---------------------------------------------------------------------
// Relleno bajo Praderas
// ---------------------------------------------------------------------

/// Lo que la región bajo Praderas se extiende más allá de la planta de sus
/// piezas: alcanza a las celdas vecinas sin dejar costura seca.
const ALCANCE: f32 = 0.10;

/// ¿Es `j` una pieza de Praderas que vuela por encima de la cuenca, con
/// sitio para el agua y el lecho debajo?
fn elevada_de_praderas(scene: &Scene, j: usize, piso: f32) -> bool {
    scene.objects[j].spatial_group == SpatialGroupId::Meadows
        && scene.objects[j].primitive.bounds().min.y >= piso + NIVEL + HUECO_BAJO_PRADERAS
}

/// Las plantas del relleno bajo Praderas: `(agua, lecho)`.
///
/// La base libre de `aplicar` excluye en planta toda pieza a cualquier
/// altura, y la meseta de Praderas vuela a `4.2`: bajo ella quedaba lienzo
/// seco. Aquí se mide la base libre **solo bajo las piezas elevadas de
/// Praderas**, dejando de excluirlas, pero no a nada más: las masas de fondo
/// que flotan bajo la meseta, las caídas que llegan al suelo y las celdas y
/// lechos ya puestos siguen fuera. Las celdas ya puestas no se tocan; las
/// nuevas las tocan por su borde.
fn plantas_bajo_praderas(
    scene: &Scene,
    plinto: usize,
    previas: &[usize],
    franja: (f32, f32),
) -> (Vec<Planta>, Vec<Planta>) {
    let p = scene.objects[plinto].primitive.bounds();
    let piso = p.max.y;
    let base = [
        p.min.x + BORDE,
        p.min.z + BORDE,
        p.max.x - BORDE,
        p.max.z - BORDE,
    ];
    let planta = |j: usize| {
        let b = scene.objects[j].primitive.bounds();
        [b.min.x, b.min.z, b.max.x, b.max.z]
    };
    let agua_previa: Vec<Planta> = previas
        .iter()
        .filter(|&&j| scene.objects[j].primitive.bounds().max.y > piso + LECHO.1 + EPS)
        .map(|&j| planta(j))
        .collect();
    let lecho_previo: Vec<Planta> = previas
        .iter()
        .filter(|&&j| scene.objects[j].primitive.bounds().max.y <= piso + LECHO.1 + EPS)
        .map(|&j| planta(j))
        .collect();

    let bajo: Vec<Planta> = (0..scene.objects.len())
        .filter(|&j| j != plinto && elevada_de_praderas(scene, j, piso))
        .map(planta)
        .map(|[x0, z0, x1, z1]| [x0 - ALCANCE, z0 - ALCANCE, x1 + ALCANCE, z1 + ALCANCE])
        .filter(|r| r[3] > franja.0 && r[1] < franja.1)
        .map(|[x0, z0, x1, z1]| [x0, z0.max(franja.0), x1, z1.min(franja.1)])
        .collect();
    let mut obstaculos: Vec<Planta> = (0..scene.objects.len())
        .filter(|&j| j != plinto && !previas.contains(&j) && !elevada_de_praderas(scene, j, piso))
        .map(|j| {
            let h = if es_caida(scene, j) {
                HOLGURA_CAIDA
            } else {
                HOLGURA
            };
            let [x0, z0, x1, z1] = planta(j);
            [x0 - h, z0 - h, x1 + h, z1 + h]
        })
        .collect();
    obstaculos.extend(agua_previa.iter().copied());

    let recorte = |v: f32, a: f32, b: f32| v.clamp(a, b);
    let xs = cortes(
        [base[0], base[2]]
            .into_iter()
            .chain(obstaculos.iter().chain(&bajo).flat_map(|o| [o[0], o[2]]))
            .map(|v| recorte(v, base[0], base[2])),
    );
    let zs = cortes(
        [base[1], base[3]]
            .into_iter()
            .chain(obstaculos.iter().chain(&bajo).flat_map(|o| [o[1], o[3]]))
            .map(|v| recorte(v, base[1], base[3])),
    );
    let dentro = |r: &Planta, x: f32, z: f32| r[0] < x && x < r[2] && r[1] < z && z < r[3];
    let libre: Vec<Vec<bool>> = (0..xs.len() - 1)
        .map(|i| {
            let cx = 0.5 * (xs[i] + xs[i + 1]);
            (0..zs.len() - 1)
                .map(|k| {
                    let cz = 0.5 * (zs[k] + zs[k + 1]);
                    bajo.iter().any(|r| dentro(r, cx, cz))
                        && !obstaculos.iter().any(|o| dentro(o, cx, cz))
                })
                .collect()
        })
        .collect();
    let agua: Vec<Planta> = fusionar(&xs, &zs, &libre)
        .into_iter()
        .filter(|r| (r[2] - r[0]).min(r[3] - r[1]) >= ANCHO_MINIMO && area(r) >= AREA_MINIMA)
        .collect();

    // El lecho nuevo: la unión de **todas** las celdas retirada
    // `RETRANQUEO_DEL_LECHO`, sin pisar el lecho previo y solo junto a las
    // celdas nuevas. Así cubre también el reborde que el lecho previo dejó
    // en la frontera con ellas y no queda costura sin fondo.
    let r = RETRANQUEO_DEL_LECHO;
    let todas: Vec<Planta> = agua_previa.iter().chain(&agua).copied().collect();
    let cubierto = |a0: f32, a1: f32, b0: f32, b1: f32| {
        // Muestreo exacto sobre la rejilla de las celdas: el cuadrado entero
        // dentro de la unión.
        let gx = cortes(todas.iter().flat_map(|c| [c[0], c[2]]));
        let gz = cortes(todas.iter().flat_map(|c| [c[1], c[3]]));
        (0..gx.len() - 1)
            .filter(|&i| gx[i + 1] > a0 && gx[i] < a1)
            .all(|i| {
                (0..gz.len() - 1)
                    .filter(|&k| gz[k + 1] > b0 && gz[k] < b1)
                    .all(|k| {
                        let (cx, cz) = (0.5 * (gx[i] + gx[i + 1]), 0.5 * (gz[k] + gz[k + 1]));
                        todas.iter().any(|c| dentro(c, cx, cz))
                    })
            })
            && a0 >= base[0]
            && a1 <= base[2]
            && b0 >= base[1]
            && b1 <= base[3]
    };
    let cerca: Vec<Planta> = agua
        .iter()
        .map(|c| [c[0] - r, c[1] - r, c[2] + r, c[3] + r])
        .collect();
    let xs = cortes(
        todas
            .iter()
            .flat_map(|c| [c[0] + r, c[2] - r, c[0], c[2]])
            .chain(lecho_previo.iter().flat_map(|l| [l[0], l[2]]))
            .chain(cerca.iter().flat_map(|c| [c[0], c[2]])),
    );
    let zs = cortes(
        todas
            .iter()
            .flat_map(|c| [c[1] + r, c[3] - r, c[1], c[3]])
            .chain(lecho_previo.iter().flat_map(|l| [l[1], l[3]]))
            .chain(cerca.iter().flat_map(|c| [c[1], c[3]])),
    );
    let marcada: Vec<Vec<bool>> = (0..xs.len() - 1)
        .map(|i| {
            let cx = 0.5 * (xs[i] + xs[i + 1]);
            (0..zs.len() - 1)
                .map(|k| {
                    let cz = 0.5 * (zs[k] + zs[k + 1]);
                    cerca.iter().any(|c| dentro(c, cx, cz))
                        && !lecho_previo.iter().any(|l| dentro(l, cx, cz))
                        && cubierto(xs[i] - r, xs[i + 1] + r, zs[k] - r, zs[k + 1] + r)
                })
                .collect()
        })
        .collect();
    let lecho: Vec<Planta> = fusionar(&xs, &zs, &marcada)
        .into_iter()
        .filter(|p| (p[2] - p[0]).min(p[3] - p[1]) >= 0.01 && area(p) >= 0.005)
        .collect();

    (agua, lecho)
}

// ---------------------------------------------------------------------
// API
// ---------------------------------------------------------------------

/// Compone la cuenca sobre una entrega.
///
/// Hay que llamarla con el volumen `A-01` todavía dentro —antes de
/// retirarlo para `InteriorVisible`— y antes de medir la escala y construir
/// la jerarquía. Añade sus piezas al final del vector: agua y después lecho.
pub fn aplicar(scene: &mut Scene) {
    prolongar_caidas(scene);

    let globales: Vec<usize> = (0..scene.objects.len())
        .filter(|&i| scene.objects[i].spatial_group == SpatialGroupId::Global)
        .collect();
    assert_eq!(globales.len(), 1, "G-01 es la unica pieza global");
    let plinto = globales[0];
    let celdas = huella_libre(scene, plinto);
    let fondos = lechos(&celdas);

    // El agua deriva del `A-01` del preset: el último de Aguas.
    let a01 = (0..scene.objects.len())
        .rfind(|&i| scene.objects[i].spatial_group == SpatialGroupId::FlyingWaters)
        .expect("A-01 va al final de Aguas");
    let base = scene.material(scene.objects[a01].final_material);
    assert!(
        base.ior > 1.0,
        "la ultima pieza de Aguas es el volumen A-01"
    );
    let [r, g, b] = TINTE;
    let agua = scene.add_material(
        base.with_specular(BRILLO.0, BRILLO.1)
            .with_tint(Color::from_srgb(r, g, b)),
    );

    let lienzo = scene.objects[caidas(scene)[0]].initial_material;
    let piso = scene.objects[plinto].primitive.bounds().max.y;
    let pieza = |p: &Planta, abajo: f32, arriba: f32, material: MaterialId| SceneObject {
        primitive: Cuboid::new(Aabb::new(
            Vec3::new(p[0], piso + abajo, p[1]),
            Vec3::new(p[2], piso + arriba, p[3]),
        ))
        .into(),
        initial_material: lienzo,
        final_material: material,
        spatial_group: SpatialGroupId::Global,
        reveal_group: RevealGroup::Meadows,
    };

    for c in &celdas {
        scene.objects.push(pieza(c, -EMPOTRADO, NIVEL, agua));
    }
    for f in &fondos {
        let caja = Aabb::new(
            Vec3::new(f[0], piso + LECHO.0, f[1]),
            Vec3::new(f[2], piso + LECHO.1, f[3]),
        );
        let tex = scene.add_texture(textura_de_fondo(&caja));
        let fondo = scene.add_material(
            Material::new(Color::new(1.0, 1.0, 1.0))
                .with_texture(tex)
                .with_uv_scale(1.0)
                .with_specular(0.06, 10.0),
        );
        scene.objects.push(pieza(f, LECHO.0, LECHO.1, fondo));
    }
}

/// Rellena la cuenca bajo la meseta de Praderas, delante de las masas que
/// flotan bajo ella: la franja que `aplicar` dejaba seca.
///
/// Se llama después de `aplicar` y, como ella, con `A-01` todavía dentro.
/// No toca ninguna pieza existente: añade al final `RELLENO_AGUA` celdas con
/// el **mismo** material de agua de la cuenca —mismo nivel, color, brillo y
/// óptica, también por preset— y `RELLENO_LECHO` piezas de lecho, cada una
/// con su fondo muestreado del mismo campo de piedra en coordenadas de mundo.
pub fn rellenar_bajo_praderas(scene: &mut Scene) {
    rellenar_tramo(scene, Tramo::Delante);
}

/// Rellena la cuenca bajo la meseta de Praderas **detrás** de las masas que
/// flotan bajo ella: la franja que se ve desde una órbita trasera, entre las
/// masas y la franja de agua del fondo del plinto.
///
/// Se llama después de `rellenar_bajo_praderas`, con `A-01` dentro, y como
/// ella solo añade piezas: `TRASERO_AGUA` celdas con el mismo material de
/// agua y `TRASERO_LECHO` piezas de lecho con su fondo del mismo campo de
/// piedra. Las masas siguen excluidas: el agua no pasa bajo ellas.
pub fn rellenar_tras_las_masas(scene: &mut Scene) {
    rellenar_tramo(scene, Tramo::Detras);
}

/// Qué lado de las masas que flotan bajo la meseta rellena una pasada.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Tramo {
    /// Delante del fondo de las masas, hacia el Monolito.
    Delante,
    /// Detrás de ellas, hacia el fondo del plinto.
    Detras,
}

fn rellenar_tramo(scene: &mut Scene, tramo: Tramo) {
    let plinto = (0..scene.objects.len())
        .find(|&i| scene.objects[i].spatial_group == SpatialGroupId::Global)
        .expect("G-01 es la primera pieza global");
    let de_la_cuenca = |i: usize| {
        scene.objects[i].spatial_group == SpatialGroupId::Global
            && scene.objects[i].reveal_group == RevealGroup::Meadows
    };
    let previas: Vec<usize> = (0..scene.objects.len())
        .filter(|&i| de_la_cuenca(i))
        .collect();
    let piso = scene.objects[plinto].primitive.bounds().max.y;
    let una_celda = *previas
        .iter()
        .find(|&&i| scene.objects[i].primitive.bounds().max.y > piso + LECHO.1 + EPS)
        .expect("la cuenca se aplica antes que su relleno");
    let agua = scene.objects[una_celda].final_material;
    let lienzo = scene.objects[una_celda].initial_material;

    // El frente: delante del fondo de las masas que flotan con su centro bajo
    // la meseta de Praderas.
    let bajo: Vec<Aabb> = (0..scene.objects.len())
        .filter(|&j| j != plinto && elevada_de_praderas(scene, j, piso))
        .map(|j| scene.objects[j].primitive.bounds())
        .collect();
    let frente = (0..scene.objects.len())
        .filter(|&j| {
            let o = &scene.objects[j];
            let b = o.primitive.bounds();
            let (cx, cz) = (0.5 * (b.min.x + b.max.x), 0.5 * (b.min.z + b.max.z));
            o.spatial_group != SpatialGroupId::Meadows
                && !de_la_cuenca(j)
                && j != plinto
                && b.min.y > piso + EPS
                && bajo
                    .iter()
                    .any(|m| m.min.x < cx && cx < m.max.x && m.min.z < cz && cz < m.max.z)
        })
        .map(|j| scene.objects[j].primitive.bounds().min.z)
        .max_by(f32::total_cmp)
        .expect("bajo la meseta flotan las masas de fondo");

    let franja = match tramo {
        Tramo::Delante => (frente, f32::INFINITY),
        Tramo::Detras => (f32::NEG_INFINITY, frente),
    };
    let (celdas, fondos) = plantas_bajo_praderas(scene, plinto, &previas, franja);
    let pieza = |p: &Planta, abajo: f32, arriba: f32, material: MaterialId| SceneObject {
        primitive: Cuboid::new(Aabb::new(
            Vec3::new(p[0], piso + abajo, p[1]),
            Vec3::new(p[2], piso + arriba, p[3]),
        ))
        .into(),
        initial_material: lienzo,
        final_material: material,
        spatial_group: SpatialGroupId::Global,
        reveal_group: RevealGroup::Meadows,
    };
    for c in &celdas {
        scene.objects.push(pieza(c, -EMPOTRADO, NIVEL, agua));
    }
    for f in &fondos {
        let caja = Aabb::new(
            Vec3::new(f[0], piso + LECHO.0, f[1]),
            Vec3::new(f[2], piso + LECHO.1, f[3]),
        );
        let tex = scene.add_texture(textura_de_fondo(&caja));
        let fondo = scene.add_material(
            Material::new(Color::new(1.0, 1.0, 1.0))
                .with_texture(tex)
                .with_uv_scale(1.0)
                .with_specular(0.06, 10.0),
        );
        scene.objects.push(pieza(f, LECHO.0, LECHO.1, fondo));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounds::Aabb;
    use crate::camera::Camera;
    use crate::material::ShadowMode;
    use crate::optics::refracted_ray;
    use crate::scene::{RevealGroup, Scene, SpatialGroupId};
    use crate::scene_builder::{measure_scene_radius, Blockout, HERO_YAW_DEGREES};
    use crate::scenes::{
        delivery_level_previo_cuenca, delivery_level_previo_monolito,
        delivery_level_previo_relleno, delivery_level_previo_trasero, WaterPreset, DELIVERY,
    };

    const EPS: f32 = 1.0e-4;
    const PRESETS: [WaterPreset; 3] = [
        WaterPreset::RefractiveWater,
        WaterPreset::OpaqueWater,
        WaterPreset::InteriorVisible,
    ];

    /// La cuenca aprobada, la etapa que miden los tests de la cuenca: desde
    /// el relleno bajo Praderas, `delivery_level` añade además sus piezas,
    /// que miden los tests del relleno.
    fn entrega() -> Blockout {
        delivery_level_previo_relleno(WaterPreset::RefractiveWater)
    }

    fn previa() -> Blockout {
        delivery_level_previo_cuenca(WaterPreset::RefractiveWater)
    }

    fn caja(s: &Scene, i: usize) -> Aabb {
        s.objects[i].primitive.bounds()
    }

    fn solapan(a: &Aabb, b: &Aabb) -> bool {
        a.min.x < b.max.x - EPS
            && b.min.x < a.max.x - EPS
            && a.min.y < b.max.y - EPS
            && b.min.y < a.max.y - EPS
            && a.min.z < b.max.z - EPS
            && b.min.z < a.max.z - EPS
    }

    /// Separación en planta por ejes: la mayor de las dos holguras.
    fn separacion(a: &Aabb, b: &Aabb) -> f32 {
        let dx = (b.min.x - a.max.x).max(a.min.x - b.max.x);
        let dz = (b.min.z - a.max.z).max(a.min.z - b.max.z);
        dx.max(dz)
    }

    /// Las piezas de la cuenca de una entrega: `(agua, lecho)`, las últimas
    /// del vector, en ese orden.
    fn cuenca(s: &Scene) -> (Vec<usize>, Vec<usize>) {
        let opaca = |i: usize| {
            s.material(s.objects[i].final_material)
                .albedo_texture
                .is_some()
                && s.objects[i].spatial_group == SpatialGroupId::Global
                && s.objects[i].reveal_group == RevealGroup::Meadows
        };
        let nuevas: Vec<usize> = (0..s.objects.len())
            .filter(|&i| {
                s.objects[i].spatial_group == SpatialGroupId::Global
                    && s.objects[i].reveal_group == RevealGroup::Meadows
            })
            .collect();
        let lechos: Vec<usize> = nuevas
            .iter()
            .copied()
            .filter(|&i| caja(s, i).max.y < LECHO.1 + EPS && opaca(i))
            .collect();
        let agua: Vec<usize> = nuevas
            .iter()
            .copied()
            .filter(|i| !lechos.contains(i))
            .collect();
        // Ningún test de la cuenca puede pasar en vacío.
        assert_eq!(
            (agua.len(), lechos.len()),
            (CELDAS, LECHOS),
            "la entrega no trae su cuenca"
        );
        (agua, lechos)
    }

    #[test]
    fn la_cuenca_suma_19_de_agua_y_18_de_lecho_en_los_tres_presets() {
        for water in PRESETS {
            let d = delivery_level_previo_relleno(water);
            let sin_volumen = usize::from(water == WaterPreset::InteriorVisible);
            let (agua, lechos) = cuenca(&d.scene);
            assert_eq!((agua.len(), lechos.len()), (CELDAS, LECHOS), "{water:?}");
            assert_eq!(
                d.scene.objects.len(),
                DELIVERY.total() + CELDAS + LECHOS - sin_volumen,
                "{water:?}"
            );
            // Las de la cuenca van al final: agua y después lecho.
            let n = d.scene.objects.len();
            let esperado: Vec<usize> = (n - CELDAS - LECHOS..n).collect();
            assert_eq!([agua, lechos].concat(), esperado, "{water:?}");
        }
        assert_eq!(DELIVERY.total() + CELDAS + LECHOS, 205);
    }

    /// Fuera de la cuenca y del pie de las cuatro caídas exteriores, la
    /// entrega es su línea base byte a byte: objetos, paleta compartida y
    /// texturas compartidas, en los tres presets.
    #[test]
    fn fuera_de_la_cuenca_la_entrega_es_la_previa_byte_a_byte() {
        for water in PRESETS {
            let a = delivery_level_previo_cuenca(water);
            let b = delivery_level_previo_relleno(water);
            let caidas: Vec<usize> = caidas(&b.scene);
            let mut alargadas = 0;
            for (i, (x, y)) in a.scene.objects.iter().zip(&b.scene.objects).enumerate() {
                if caidas[..4].contains(&i) {
                    let (p, q) = (x.primitive.bounds(), y.primitive.bounds());
                    assert_eq!(p.max, q.max, "{water:?}: caida {i}");
                    assert_eq!((p.min.x, p.min.z), (q.min.x, q.min.z));
                    assert!(q.min.y < p.min.y, "{water:?}: la caida {i} no baja");
                    assert_eq!(
                        (
                            x.initial_material,
                            x.final_material,
                            x.spatial_group,
                            x.reveal_group
                        ),
                        (
                            y.initial_material,
                            y.final_material,
                            y.spatial_group,
                            y.reveal_group
                        )
                    );
                    alargadas += 1;
                } else {
                    assert_eq!(format!("{x:?}"), format!("{y:?}"), "{water:?}: objeto {i}");
                }
            }
            assert_eq!(alargadas, 4, "{water:?}");
            assert_eq!(b.scene.palette.len(), a.scene.palette.len() + 1 + LECHOS);
            for (i, (m, n)) in a.scene.palette.iter().zip(&b.scene.palette).enumerate() {
                assert_eq!(
                    format!("{m:?}"),
                    format!("{n:?}"),
                    "{water:?}: material {i}"
                );
            }
            assert_eq!(b.scene.textures.len(), a.scene.textures.len() + LECHOS);
            for (i, (t, u)) in a.scene.textures.iter().zip(&b.scene.textures).enumerate() {
                assert_eq!(
                    (t.width(), t.height(), t.peak()),
                    (u.width(), u.height(), u.peak()),
                    "{i}"
                );
            }
            assert_eq!(format!("{:?}", a.anchors), format!("{:?}", b.anchors));
            assert_eq!(format!("{:?}", a.scale), format!("{:?}", b.scale));
            assert_eq!(
                format!("{:?}", a.scene.skybox),
                format!("{:?}", b.scene.skybox)
            );
        }
    }

    /// El agua de la cuenca deriva del `A-01` **del preset**: en la entrega
    /// refractiva —la aprobada— es el agua de `A-01` con techos `0.9 / 0.9`,
    /// `ior 1.333` y sombra `Ignore`; en el control opaco sigue al control,
    /// sin óptica. `InteriorVisible` compone con el volumen dentro y lleva el
    /// agua refractiva.
    #[test]
    fn el_agua_de_la_cuenca_sigue_al_preset() {
        for water in PRESETS {
            let d = delivery_level_previo_relleno(water);
            let s = &d.scene;
            let (agua, lechos) = cuenca(s);
            let m = s.material(s.objects[agua[0]].final_material);
            for &i in &agua {
                assert_eq!(
                    s.objects[i].final_material,
                    s.objects[agua[0]].final_material
                );
            }
            let [r, g, b] = TINTE;
            assert_eq!(
                format!("{:?}", m.albedo),
                format!("{:?}", crate::color::Color::from_srgb(r, g, b))
            );
            assert_eq!((m.specular_strength, m.shininess), BRILLO);
            match water {
                WaterPreset::OpaqueWater => {
                    assert_eq!((m.reflection_cap, m.transmission_cap), (0.0, 0.0));
                }
                _ => {
                    assert_eq!(
                        (m.reflection_cap, m.transmission_cap, m.ior),
                        (0.9, 0.9, 1.333)
                    );
                }
            }
            assert_eq!(m.shadow_mode, ShadowMode::Ignore);
            for &i in &lechos {
                let l = s.material(s.objects[i].final_material);
                assert_eq!(
                    (l.reflection_cap, l.transmission_cap, l.ior),
                    (0.0, 0.0, 1.0)
                );
                assert_eq!(l.shadow_mode, ShadowMode::Opaque);
                assert!(l.albedo_texture.is_some());
                for (j, o) in s.objects.iter().enumerate() {
                    if j != i {
                        assert_ne!(o.final_material, s.objects[i].final_material);
                    }
                }
            }
            for o in &s.objects[..s.objects.len() - CELDAS - LECHOS] {
                assert!(!agua
                    .iter()
                    .any(|&i| s.objects[i].final_material == o.final_material));
            }
        }
    }

    /// Agua de altura común apoyada en el plinto, lecho somero bajo ella,
    /// y ninguna intersección en volumen con nada salvo el plinto y su
    /// lecho. Las caídas, la excepción controlada: sin contacto.
    #[test]
    fn la_cuenca_apoya_en_la_base_y_no_corta_nada() {
        let d = entrega();
        let s = &d.scene;
        let (agua, lechos) = cuenca(s);
        let piso = caja(s, 0).max.y;
        for &i in &agua {
            let c = caja(s, i);
            assert!((c.min.y - (piso - EMPOTRADO)).abs() < EPS);
            assert!((c.max.y - (piso + NIVEL)).abs() < EPS);
            for j in 1..s.objects.len() {
                if j == i || lechos.contains(&j) {
                    continue;
                }
                assert!(!solapan(&c, &caja(s, j)), "el agua {i} corta {j}");
                if s.objects[j].spatial_group == SpatialGroupId::Meadows
                    && s.material(s.objects[j].final_material).ior > 1.0
                {
                    assert!(
                        separacion(&c, &caja(s, j)) > 0.0,
                        "el agua {i} toca la caida {j}"
                    );
                }
            }
        }
        for &i in &lechos {
            let b = caja(s, i);
            assert!(b.min.y < piso && piso < b.max.y && b.max.y < piso + NIVEL);
            assert!(agua.iter().any(|&a| {
                let c = caja(s, a);
                c.min.x <= b.min.x && b.max.x <= c.max.x || c.min.z <= b.min.z && b.max.z <= c.max.z
            }));
            for j in 1..s.objects.len() {
                if j != i && !agua.contains(&j) {
                    assert!(!solapan(&b, &caja(s, j)), "el lecho {i} corta {j}");
                }
            }
        }
    }

    /// Los bloques vecinos del agua asoman al menos `0.10`, el Monolito no
    /// cambia, y la cuenca no se une a Aguas Voladoras.
    #[test]
    fn los_bloques_emergen_y_aguas_queda_aparte() {
        let p = previa();
        let d = entrega();
        let s = &d.scene;
        let (agua, lechos) = cuenca(s);
        let techo = caja(s, 0).max.y + NIVEL;
        for &i in &agua {
            for j in 1..s.objects.len() {
                if agua.contains(&j) || lechos.contains(&j) {
                    continue;
                }
                if separacion(&caja(s, i), &caja(s, j)) <= 0.10 {
                    assert!(
                        caja(s, j).max.y >= techo + 0.10,
                        "{j} no emerge junto a {i}"
                    );
                }
            }
        }
        let monolito: Vec<usize> = (0..s.objects.len())
            .filter(|&i| s.objects[i].spatial_group == SpatialGroupId::Monolith)
            .collect();
        for &i in &monolito {
            assert_eq!(
                format!("{:?}", p.scene.objects[i]),
                format!("{:?}", s.objects[i])
            );
        }
        let aguas: Vec<usize> = (0..s.objects.len())
            .filter(|&i| s.objects[i].spatial_group == SpatialGroupId::FlyingWaters)
            .collect();
        let a01 = caja(s, *aguas.last().unwrap());
        for &i in &agua {
            for &j in &aguas {
                assert!(separacion(&caja(s, i), &caja(s, j)) >= HOLGURA - EPS);
            }
            assert!(separacion(&caja(s, i), &a01) >= 0.2);
        }
        assert!(
            a01.max.y - techo > 2.0,
            "sin conexion con la superficie de Aguas"
        );
    }

    /// Cada caída exterior baja a la superficie que tiene debajo; las tres
    /// que llegan al plinto desembocan en el agua por todo su frente.
    #[test]
    fn las_caidas_desembocan_donde_la_geometria_lo_permite() {
        let p = previa();
        let d = entrega();
        let s = &d.scene;
        let (agua, _) = cuenca(s);
        let todas = caidas(s);
        let piso = caja(s, 0).max.y;
        let mut al_plinto = 0;
        for &i in &todas[..4] {
            let b = caja(s, i);
            assert!(b.min.y < caja(&p.scene, i).min.y);
            if b.min.y >= piso {
                // La cuarta: sobre la masa de fondo que tiene debajo.
                assert!(
                    (b.min.y - (1.70 - EMPOTRADO)).abs() < 1e-3,
                    "{i}: {}",
                    b.min.y
                );
                continue;
            }
            al_plinto += 1;
            assert!((b.min.y - (piso - EMPOTRADO)).abs() < EPS);
            let mut tramos: Vec<(f32, f32)> = agua
                .iter()
                .map(|&a| caja(s, a))
                .filter(|c| {
                    let hueco = c.min.z - b.max.z;
                    hueco > 0.0 && hueco <= 0.03
                })
                .map(|c| (c.min.x, c.max.x))
                .collect();
            tramos.sort_by(|x, y| x.0.total_cmp(&y.0));
            let mut hasta = b.min.x;
            for (x0, x1) in tramos {
                if x0 <= hasta + EPS {
                    hasta = hasta.max(x1);
                }
            }
            assert!(hasta >= b.max.x - EPS, "la caida {i} no desemboca entera");
        }
        assert_eq!(al_plinto, 3);
        for &i in &todas[4..] {
            assert_eq!(
                format!("{:?}", p.scene.objects[i]),
                format!("{:?}", s.objects[i])
            );
        }
    }

    /// La escala medida y las cámaras no se mueven.
    #[test]
    fn la_escala_y_las_camaras_no_cambian() {
        let p = previa();
        let d = entrega();
        assert_eq!(
            measure_scene_radius(&d.scene, d.anchors.orbit_center),
            p.scale.scene_radius
        );
        assert_eq!(
            format!("{:?}", p.hero_camera()),
            format!("{:?}", d.hero_camera())
        );
        assert_eq!(
            format!("{:?}", p.camera_at(HERO_YAW_DEGREES, 78.0)),
            format!("{:?}", d.camera_at(HERO_YAW_DEGREES, 78.0))
        );
    }

    fn camaras(d: &Blockout) -> [Camera; 3] {
        let mira = nalgebra_glm::Vec3::new(0.0, 0.3, -0.6);
        [
            d.hero_camera(),
            d.camera_at(HERO_YAW_DEGREES, 78.0),
            Camera::new(
                nalgebra_glm::Vec3::new(1.5, 11.0, 7.5),
                mira,
                mira,
                nalgebra_glm::Vec3::new(0.0, 1.0, 0.0),
                45f32.to_radians(),
            ),
        ]
    }

    #[test]
    fn la_jerarquia_coincide_con_el_oraculo_lineal() {
        let d = entrega();
        let (w, h) = (120, 90);
        for camara in camaras(&d) {
            for p in 0..w * h {
                let rayo = camara.ray_from_pixel(p % w, p / w, w, h);
                let mut stats = Default::default();
                assert_eq!(
                    d.accel
                        .intersect(&d.scene, &rayo, &mut stats)
                        .map(|x| x.object_index),
                    d.scene.intersect(&rayo).map(|x| x.object_index)
                );
            }
        }
    }

    /// El artefacto conocido del renderer, acotado: el rayo refractado que
    /// llega a la cara que comparten dos celdas «sale» al aire. Se queda en
    /// `<= 2 %` de los píxeles de agua en las tres tomas.
    #[test]
    fn las_costuras_entre_celdas_quedan_acotadas() {
        let d = entrega();
        let s = &d.scene;
        let (agua, _) = cuenca(s);
        let plantas: Vec<Aabb> = agua.iter().map(|&i| caja(s, i)).collect();
        let compartida = |x: f32, z: f32| {
            plantas
                .iter()
                .filter(|c| c.min.x <= x && x <= c.max.x && c.min.z <= z && z <= c.max.z)
                .count()
                >= 2
        };
        let (w, h) = (400, 300);
        for camara in camaras(&d) {
            let (mut n, mut costura) = (0usize, 0usize);
            for p in 0..w * h {
                let rayo = camara.ray_from_pixel(p % w, p / w, w, h);
                let mut st = Default::default();
                let Some(hit) = d.accel.intersect(s, &rayo, &mut st) else {
                    continue;
                };
                if !agua.contains(&hit.object_index) || !hit.front_face || hit.normal.y < 0.9 {
                    continue;
                }
                n += 1;
                let Some(dentro) = refracted_ray(&hit, &rayo.direction, 1.333) else {
                    continue;
                };
                if let Some(x) = d.accel.intersect(s, &dentro, &mut st) {
                    if agua.contains(&x.object_index)
                        && x.normal.y.abs() < 0.5
                        && compartida(x.point.x, x.point.z)
                    {
                        costura += 1;
                    }
                }
            }
            assert!(n > 500);
            assert!(costura as f32 / n as f32 <= 0.02, "{costura} de {n}");
        }
    }

    #[test]
    fn la_cuenca_es_determinista() {
        let (a, b) = (entrega(), entrega());
        assert_eq!(
            format!("{:?}", a.scene.objects),
            format!("{:?}", b.scene.objects)
        );
        assert_eq!(
            format!("{:?}", a.scene.palette),
            format!("{:?}", b.scene.palette)
        );
    }

    /// FNV-1a de 64 bits sobre las piezas de la cuenca y las caídas, sus
    /// materiales efectivos y las texturas del lecho, texel a texel, sin
    /// assets. Se fijó **después** de que el test del preview comprobara, con
    /// los assets reales, que la entrega es `flooded_blue_raised` byte a byte.
    fn huella(d: &Blockout, previa: &Scene) -> u64 {
        let s = &d.scene;
        let mut h: u64 = 0xCBF2_9CE4_8422_2325;
        let mut mezclar = |texto: String| {
            for b in texto.bytes() {
                h ^= u64::from(b);
                h = h.wrapping_mul(0x0100_0000_01B3);
            }
        };
        for i in previa.objects.len()..s.objects.len() {
            mezclar(format!("{:?}", s.objects[i].primitive));
            mezclar(format!("{:?}", s.material(s.objects[i].final_material)));
        }
        for i in caidas(s) {
            mezclar(format!("{:?}", s.objects[i]));
        }
        for t in &s.textures[previa.textures.len()..] {
            for y in 0..t.height() {
                for x in 0..t.width() {
                    let u = (x as f32 + 0.5) / t.width() as f32;
                    let v = (y as f32 + 0.5) / t.height() as f32;
                    mezclar(format!("{:?}", t.sample(u, v)));
                }
            }
        }
        h
    }

    /// Una huella por ruta: sin `hex-prism`, el Rompeolas es de cuboides y
    /// su losa y sus dos masas vecinas quedan `0.15` más al fondo, así que la
    /// base libre y sus celdas son otras. Las dos se fijaron después de que
    /// la equivalencia con assets reales del preview pasara en su ruta; el
    /// pincel artístico no cambia la geometría y comparte la de la Ruta B.
    #[cfg(feature = "hex-prism")]
    const HUELLA_DE_LA_CUENCA: u64 = 9_815_729_660_935_996_773;
    #[cfg(not(feature = "hex-prism"))]
    const HUELLA_DE_LA_CUENCA: u64 = 10_362_225_630_250_966_656;

    #[test]
    fn la_huella_de_la_cuenca_queda_fijada() {
        let p = previa();
        assert_eq!(huella(&entrega(), &p.scene), HUELLA_DE_LA_CUENCA);
    }

    // ================================================== relleno bajo Praderas

    fn aprobada() -> Blockout {
        delivery_level_previo_relleno(WaterPreset::RefractiveWater)
    }

    /// La entrega con el relleno delantero, la etapa que miden los tests
    /// del relleno: desde el relleno tras las masas, `delivery_level` añade
    /// además sus piezas, que miden los tests del relleno trasero.
    fn con_relleno() -> Blockout {
        delivery_level_previo_trasero(WaterPreset::RefractiveWater)
    }

    /// Las piezas del relleno: lo que la entrega añade detrás de las 205 de
    /// la cuenca aprobada, `(agua, lecho)`. Exige las nueve.
    fn relleno(d: &Blockout, previa: &Blockout) -> (Vec<usize>, Vec<usize>) {
        let s = &d.scene;
        let piso = caja(s, 0).max.y;
        let nuevas: Vec<usize> = (previa.scene.objects.len()..s.objects.len()).collect();
        let (agua, lecho): (Vec<usize>, Vec<usize>) = nuevas
            .iter()
            .partition(|&&i| caja(s, i).max.y > piso + LECHO.1 + EPS);
        assert_eq!(
            (agua.len(), lecho.len()),
            (RELLENO_AGUA, RELLENO_LECHO),
            "la entrega no trae su relleno"
        );
        (agua, lecho)
    }

    /// ¿Hay algo, salvo el plinto y lo que vuela sobre la cuenca en Praderas,
    /// ocupando el volumen del agua sobre el punto `(x, z)`?
    fn ocupado(d: &Blockout, x: f32, z: f32, salvo: &[usize]) -> bool {
        let s = &d.scene;
        let piso = caja(s, 0).max.y;
        (1..s.objects.len())
            .filter(|j| !salvo.contains(j))
            .any(|j| {
                let b = caja(s, j);
                let h = HOLGURA;
                b.min.x - h < x
                    && x < b.max.x + h
                    && b.min.z - h < z
                    && z < b.max.z + h
                    && b.min.y < piso + NIVEL + HUECO_BAJO_PRADERAS
                    && b.max.y > piso - EMPOTRADO
            })
    }

    /// La franja de la captura: bajo la meseta de Praderas, delante de las
    /// masas de fondo que flotan bajo ella. Puntos al paso de `0.05`.
    fn franja(d: &Blockout) -> Vec<(f32, f32)> {
        let s = &d.scene;
        let meseta = caja(
            s,
            (0..s.objects.len())
                .find(|&i| s.objects[i].spatial_group == SpatialGroupId::Meadows)
                .unwrap(),
        );
        let mut v = Vec::new();
        let mut x = meseta.min.x + 0.025;
        while x < meseta.max.x {
            let mut z = -6.0 + 0.025;
            while z < meseta.max.z {
                v.push((x, z));
                z += 0.05;
            }
            x += 0.05;
        }
        v
    }

    /// RED/GREEN del hueco: en la cuenca aprobada, la franja libre bajo
    /// Praderas está **seca**; en la entrega, el agua cubre al menos el
    /// `90 %` de lo que está libre en volumen.
    #[test]
    fn el_hueco_bajo_praderas_estaba_seco_y_queda_lleno() {
        let p = aprobada();
        let d = con_relleno();
        let s = &d.scene;
        let (agua_vieja, lecho_viejo) = cuenca(&p.scene);
        let (agua, lecho) = relleno(&d, &p);
        let piso = caja(s, 0).max.y;
        let elevadas: Vec<usize> = (0..s.objects.len())
            .filter(|&j| {
                s.objects[j].spatial_group == SpatialGroupId::Meadows
                    && caja(s, j).min.y >= piso + NIVEL + HUECO_BAJO_PRADERAS
            })
            .collect();
        let todas_viejas: Vec<usize> = agua_vieja.iter().chain(&lecho_viejo).copied().collect();
        let mojada = |celdas: &[usize], x: f32, z: f32| {
            celdas.iter().any(|&i| {
                let c = caja(s, i);
                c.min.x <= x && x <= c.max.x && c.min.z <= z && z <= c.max.z
            })
        };
        let (mut libres, mut secas_antes, mut cubiertas) = (0usize, 0usize, 0usize);
        for (x, z) in franja(&d) {
            let salvo: Vec<usize> = elevadas
                .iter()
                .chain(&todas_viejas)
                .chain(&agua)
                .chain(&lecho)
                .copied()
                .collect();
            if ocupado(&d, x, z, &salvo) {
                continue;
            }
            libres += 1;
            secas_antes += usize::from(!mojada(&agua_vieja, x, z));
            cubiertas += usize::from(mojada(&agua, x, z));
        }
        assert!(libres > 1000, "la franja libre es real: {libres} muestras");
        assert_eq!(secas_antes, libres, "antes del relleno estaba seca");
        let f = cubiertas as f32 / libres as f32;
        assert!(f >= 0.90, "el relleno cubre {f}");
    }

    /// Las 205 piezas de la cuenca aprobada quedan exactas, y su paleta y
    /// sus texturas delante intactas; el relleno solo se añade, en los tres
    /// presets, con el agua de la cuenca y un fondo nuevo por pieza de lecho.
    #[test]
    fn el_relleno_solo_se_anade_tras_la_cuenca_aprobada() {
        for water in PRESETS {
            let p = delivery_level_previo_relleno(water);
            let d = delivery_level_previo_trasero(water);
            let sin_volumen = usize::from(water == WaterPreset::InteriorVisible);
            assert_eq!(
                d.scene.objects.len(),
                DELIVERY.total() + CELDAS + LECHOS + RELLENO_AGUA + RELLENO_LECHO - sin_volumen
            );
            for (i, (x, y)) in p.scene.objects.iter().zip(&d.scene.objects).enumerate() {
                assert_eq!(format!("{x:?}"), format!("{y:?}"), "{water:?}: objeto {i}");
            }
            assert_eq!(d.scene.palette.len(), p.scene.palette.len() + RELLENO_LECHO);
            for (i, (m, n)) in p.scene.palette.iter().zip(&d.scene.palette).enumerate() {
                assert_eq!(format!("{m:?}"), format!("{n:?}"), "material {i}");
            }
            assert_eq!(
                d.scene.textures.len(),
                p.scene.textures.len() + RELLENO_LECHO
            );
            for (i, (t, u)) in p.scene.textures.iter().zip(&d.scene.textures).enumerate() {
                assert_eq!(
                    (t.width(), t.height(), t.peak()),
                    (u.width(), u.height(), u.peak()),
                    "{i}"
                );
            }
            let (agua_vieja, _) = cuenca(&p.scene);
            let (agua, lecho) = relleno(&d, &p);
            let material_del_agua = p.scene.objects[agua_vieja[0]].final_material;
            for &i in &agua {
                assert_eq!(
                    d.scene.objects[i].final_material, material_del_agua,
                    "{water:?}"
                );
                assert_eq!(
                    d.scene.objects[i].initial_material,
                    p.scene.objects[agua_vieja[0]].initial_material
                );
                assert_eq!(d.scene.objects[i].spatial_group, SpatialGroupId::Global);
                assert_eq!(d.scene.objects[i].reveal_group, RevealGroup::Meadows);
            }
            for &i in &lecho {
                let id = d.scene.objects[i].final_material;
                assert!(id.0 >= p.scene.palette.len());
                let l = d.scene.material(id);
                assert_eq!(
                    (l.reflection_cap, l.transmission_cap, l.ior),
                    (0.0, 0.0, 1.0)
                );
                assert!(l.albedo_texture.unwrap().0 >= p.scene.textures.len());
                for (j, o) in d.scene.objects.iter().enumerate() {
                    if j != i {
                        assert_ne!(o.final_material, id, "fondo compartido");
                    }
                }
            }
            assert_eq!(format!("{:?}", p.anchors), format!("{:?}", d.anchors));
            assert_eq!(format!("{:?}", p.scale), format!("{:?}", d.scale));
        }
    }

    /// Mismo nivel y apoyo, sin cortar nada en volumen —ni las celdas
    /// previas, ni las masas que flotan bajo la meseta, ni las caídas— y con
    /// la meseta y lo que cuelga de ella muy por encima del agua.
    #[test]
    fn el_relleno_apoya_y_no_corta_nada() {
        let p = aprobada();
        let d = con_relleno();
        let s = &d.scene;
        let (agua, lecho) = relleno(&d, &p);
        let piso = caja(s, 0).max.y;
        for &i in &agua {
            let c = caja(s, i);
            assert!((c.min.y - (piso - EMPOTRADO)).abs() < EPS);
            assert!((c.max.y - (piso + NIVEL)).abs() < EPS);
            for j in 1..s.objects.len() {
                if j == i || lecho.contains(&j) {
                    continue;
                }
                let o = caja(s, j);
                assert!(!solapan(&c, &o), "el agua nueva {i} corta {j}");
                if s.objects[j].spatial_group == SpatialGroupId::Meadows {
                    let refractiva = s.material(s.objects[j].final_material).ior > 1.0;
                    if o.min.y < piso + NIVEL + HUECO_BAJO_PRADERAS {
                        assert!(refractiva, "{j} no es una caida y baja hasta el agua");
                        assert!(separacion(&c, &o) > 0.0, "el agua {i} toca la caida {j}");
                    } else if c.min.x < o.max.x
                        && o.min.x < c.max.x
                        && c.min.z < o.max.z
                        && o.min.z < c.max.z
                    {
                        assert!(
                            o.min.y - c.max.y >= HUECO_BAJO_PRADERAS - EPS,
                            "{j} sobre el agua {i}"
                        );
                    }
                }
            }
        }
        for &i in &lecho {
            let b = caja(s, i);
            assert!(
                (b.min.y - (piso + LECHO.0)).abs() < EPS
                    && (b.max.y - (piso + LECHO.1)).abs() < EPS
            );
            for j in 1..s.objects.len() {
                let o = caja(s, j);
                let es_agua = s.objects[j].spatial_group == SpatialGroupId::Global
                    && o.max.y > piso + LECHO.1 + EPS;
                if j != i && !es_agua {
                    assert!(!solapan(&b, &o), "el lecho nuevo {i} corta {j}");
                }
            }
        }
    }

    /// Continuidad: cada celda nueva comparte un tramo de borde con una
    /// celda previa o con otra nueva; y el lecho nuevo queda bajo agua, a
    /// `RETRANQUEO_DEL_LECHO` de su borde exterior.
    #[test]
    fn el_relleno_continua_el_agua_y_su_lecho() {
        let p = aprobada();
        let d = con_relleno();
        let s = &d.scene;
        let (agua_vieja, _) = cuenca(&p.scene);
        let (agua, lecho) = relleno(&d, &p);
        let todas: Vec<Aabb> = agua_vieja
            .iter()
            .chain(&agua)
            .map(|&i| caja(s, i))
            .collect();
        let tocan = |a: &Aabb, b: &Aabb| {
            let tramo_z = a.max.z.min(b.max.z) - a.min.z.max(b.min.z);
            let tramo_x = a.max.x.min(b.max.x) - a.min.x.max(b.min.x);
            ((a.max.x - b.min.x).abs() < EPS || (b.max.x - a.min.x).abs() < EPS) && tramo_z > 0.05
                || ((a.max.z - b.min.z).abs() < EPS || (b.max.z - a.min.z).abs() < EPS)
                    && tramo_x > 0.05
        };
        for &i in &agua {
            let c = caja(s, i);
            assert!(
                todas.iter().any(|o| *o != c && tocan(&c, o)),
                "la celda nueva {i} queda suelta"
            );
        }
        let m = RETRANQUEO_DEL_LECHO * 0.99;
        let mojada = |x: f32, z: f32| {
            todas
                .iter()
                .any(|c| c.min.x <= x && x <= c.max.x && c.min.z <= z && z <= c.max.z)
        };
        for &i in &lecho {
            let b = caja(s, i);
            for (x, z) in [
                (b.min.x, b.min.z),
                (b.min.x, b.max.z),
                (b.max.x, b.min.z),
                (b.max.x, b.max.z),
            ] {
                for (dx, dz) in [(-m, -m), (-m, m), (m, -m), (m, m)] {
                    assert!(mojada(x + dx, z + dz), "lecho {i} sin agua en ({x}, {z})");
                }
            }
        }
    }

    /// El fondo nuevo es el mismo campo de piedra, en coordenadas de mundo y
    /// a `TEXELS_POR_UNIDAD`: continúa el lecho previo sin costura ni
    /// estiramiento.
    #[test]
    fn el_fondo_nuevo_muestrea_el_mismo_campo_de_mundo() {
        let p = aprobada();
        let d = con_relleno();
        let s = &d.scene;
        let (_, lecho) = relleno(&d, &p);
        for &i in &lecho {
            let b = caja(s, i);
            let t = s.texture(
                s.material(s.objects[i].final_material)
                    .albedo_texture
                    .unwrap(),
            );
            let (w, h) = (b.max.x - b.min.x, b.max.z - b.min.z);
            assert_eq!(t.width(), ((w * TEXELS_POR_UNIDAD).round() as usize).max(2));
            assert_eq!(
                t.height(),
                ((h * TEXELS_POR_UNIDAD).round() as usize).max(2)
            );
            for (u, v) in [(0.1, 0.13), (0.5, 0.52), (0.93, 0.31)] {
                let tx = ((u * t.width() as f32) as usize).min(t.width() - 1);
                let fila = (((1.0 - v) * t.height() as f32) as usize).min(t.height() - 1);
                let x = b.min.x + (tx as f32 + 0.5) / t.width() as f32 * w;
                let z = b.min.z + (1.0 - (fila as f32 + 0.5) / t.height() as f32) * h;
                assert_eq!(
                    format!("{:?}", t.sample(u, v)),
                    format!("{:?}", campo_de_roca(x, z))
                );
            }
        }
    }

    #[test]
    fn el_relleno_conserva_la_jerarquia_lineal_y_el_determinismo() {
        let d = con_relleno();
        let (w, h) = (120, 90);
        let mira = nalgebra_glm::Vec3::new(0.0, 0.6, -3.6);
        let bajo = Camera::new(
            nalgebra_glm::Vec3::new(-1.0, 2.2, 3.5),
            mira,
            mira,
            nalgebra_glm::Vec3::new(0.0, 1.0, 0.0),
            50f32.to_radians(),
        );
        for camara in camaras(&d).into_iter().chain([bajo]) {
            for p in 0..w * h {
                let rayo = camara.ray_from_pixel(p % w, p / w, w, h);
                let mut stats = Default::default();
                assert_eq!(
                    d.accel
                        .intersect(&d.scene, &rayo, &mut stats)
                        .map(|x| x.object_index),
                    d.scene.intersect(&rayo).map(|x| x.object_index)
                );
            }
        }
        let otra = con_relleno();
        assert_eq!(
            format!("{:?}", d.scene.objects),
            format!("{:?}", otra.scene.objects)
        );
    }

    /// FNV-1a de las piezas del relleno, sus materiales efectivos y sus
    /// texturas, sin assets. Se fijó después de la equivalencia con assets
    /// reales del preview en las dos rutas.
    fn huella_del_relleno(d: &Blockout, previa: &Scene) -> u64 {
        let s = &d.scene;
        let mut h: u64 = 0xCBF2_9CE4_8422_2325;
        let mut mezclar = |texto: String| {
            for b in texto.bytes() {
                h ^= u64::from(b);
                h = h.wrapping_mul(0x0100_0000_01B3);
            }
        };
        for i in previa.objects.len()..s.objects.len() {
            mezclar(format!("{:?}", s.objects[i].primitive));
            mezclar(format!("{:?}", s.material(s.objects[i].final_material)));
        }
        for t in &s.textures[previa.textures.len()..] {
            for y in 0..t.height() {
                for x in 0..t.width() {
                    let u = (x as f32 + 0.5) / t.width() as f32;
                    let v = (y as f32 + 0.5) / t.height() as f32;
                    mezclar(format!("{:?}", t.sample(u, v)));
                }
            }
        }
        h
    }

    /// La misma en las dos rutas: lo que cambia en la Ruta B —el Rompeolas y
    /// sus masas— queda lejos de Praderas.
    const HUELLA_DEL_RELLENO: u64 = 13_192_632_095_342_143_467;

    #[test]
    fn la_huella_del_relleno_queda_fijada() {
        let p = aprobada();
        assert_eq!(
            huella_del_relleno(&con_relleno(), &p.scene),
            HUELLA_DEL_RELLENO
        );
    }

    /// Las costuras con el relleno, por toma. El rayo refractado que llega a
    /// una cara compartida entre celdas «sale» al aire: es el artefacto
    /// conocido del renderer. En la hero y en la cenital —las tomas de
    /// juicio— sigue en `<= 2 %`. En el encuadre cercano del Monolito sube de
    /// `1.59 %` a `2.12 %`, sobre todo porque el canto trasero de una celda
    /// previa pasa a ser cara compartida con una nueva; se acota en
    /// `<= 2.5 %`, aceptado por Charlie y documentado. El test de la cuenca
    /// aprobada conserva su `<= 2 %` en las tres.
    #[test]
    fn las_costuras_siguen_acotadas_con_el_relleno() {
        let d = con_relleno();
        let s = &d.scene;
        let piso = caja(s, 0).max.y;
        let agua: Vec<usize> = (0..s.objects.len())
            .filter(|&i| {
                s.objects[i].spatial_group == SpatialGroupId::Global
                    && s.objects[i].reveal_group == RevealGroup::Meadows
                    && caja(s, i).max.y > piso + LECHO.1 + EPS
            })
            .collect();
        assert_eq!(agua.len(), CELDAS + RELLENO_AGUA);
        let plantas: Vec<Aabb> = agua.iter().map(|&i| caja(s, i)).collect();
        let compartida = |x: f32, z: f32| {
            plantas
                .iter()
                .filter(|c| c.min.x <= x && x <= c.max.x && c.min.z <= z && z <= c.max.z)
                .count()
                >= 2
        };
        let (w, h) = (400, 300);
        for (camara, tope) in camaras(&d).into_iter().zip([0.02, 0.02, 0.025]) {
            let (mut n, mut costura) = (0usize, 0usize);
            for p in 0..w * h {
                let rayo = camara.ray_from_pixel(p % w, p / w, w, h);
                let mut st = Default::default();
                let Some(hit) = d.accel.intersect(s, &rayo, &mut st) else {
                    continue;
                };
                if !agua.contains(&hit.object_index) || !hit.front_face || hit.normal.y < 0.9 {
                    continue;
                }
                n += 1;
                let Some(dentro) = refracted_ray(&hit, &rayo.direction, 1.333) else {
                    continue;
                };
                if let Some(x) = d.accel.intersect(s, &dentro, &mut st) {
                    if agua.contains(&x.object_index)
                        && x.normal.y.abs() < 0.5
                        && compartida(x.point.x, x.point.z)
                    {
                        costura += 1;
                    }
                }
            }
            assert!(n > 500);
            assert!(costura as f32 / n as f32 <= tope, "{costura} de {n}");
        }
    }

    // ============================================ relleno tras las masas

    /// La entrega con los dos rellenos, la etapa que se mide aquí. Desde
    /// el Monolito de carbón, `delivery_level` añade además su placa al
    /// final; esta etapa es `delivery_level_previo_monolito`.
    fn con_trasero() -> Blockout {
        delivery_level_previo_monolito(WaterPreset::RefractiveWater)
    }

    /// Las piezas del relleno trasero: lo que la entrega añade detrás de las
    /// 214 del relleno delantero, `(agua, lecho)`. Exige las cuatro.
    fn trasero(d: &Blockout, previa: &Blockout) -> (Vec<usize>, Vec<usize>) {
        let s = &d.scene;
        let piso = caja(s, 0).max.y;
        let nuevas: Vec<usize> = (previa.scene.objects.len()..s.objects.len()).collect();
        let (agua, lecho): (Vec<usize>, Vec<usize>) = nuevas
            .iter()
            .partition(|&&i| caja(s, i).max.y > piso + LECHO.1 + EPS);
        assert_eq!(
            (agua.len(), lecho.len()),
            (TRASERO_AGUA, TRASERO_LECHO),
            "la entrega no trae su relleno trasero"
        );
        (agua, lecho)
    }

    /// Píxeles, desde las órbitas que la ventana alcanza —los cuatro lados a
    /// `20°` y `35°`—, cuyo primer impacto es el techo **seco** del plinto
    /// bajo la meseta de Praderas y detrás de las masas que flotan bajo ella.
    /// No cuenta el reborde de holgura junto a cualquier pieza ni la ranura
    /// de `0.14` entre las dos masas: son astillas de diseño, más estrechas
    /// que `ANCHO_MINIMO`.
    fn plinto_seco_visible_tras_las_masas(d: &Blockout) -> Vec<(f32, f32, usize)> {
        let s = &d.scene;
        let meseta = caja(
            s,
            (0..s.objects.len())
                .find(|&i| s.objects[i].spatial_group == SpatialGroupId::Meadows)
                .unwrap(),
        );
        let piso = caja(s, 0).max.y;
        let cerca_de_una_pieza = |x: f32, z: f32| {
            (1..s.objects.len()).any(|j| {
                let b = caja(s, j);
                b.min.y < piso + NIVEL + HUECO_BAJO_PRADERAS
                    && b.min.x - 0.05 < x
                    && x < b.max.x + 0.05
                    && b.min.z - 0.05 < z
                    && z < b.max.z + 0.05
            })
        };
        let (w, h) = (400, 300);
        let mut vistas = Vec::new();
        for giro in [0.0f32, 90.0, 180.0, 270.0] {
            for elevacion in [20.0f32, 35.0] {
                let camara = d.camera_at(HERO_YAW_DEGREES + giro, elevacion);
                let mut n = 0;
                for p in 0..w * h {
                    let rayo = camara.ray_from_pixel(p % w, p / w, w, h);
                    let mut st = Default::default();
                    let Some(hit) = d.accel.intersect(s, &rayo, &mut st) else {
                        continue;
                    };
                    let (x, z) = (hit.point.x, hit.point.z);
                    if hit.object_index == 0
                        && hit.normal.y > 0.9
                        && meseta.min.x < x
                        && x < meseta.max.x
                        && meseta.min.z < z
                        && z < -6.0
                        && !(-0.1..=0.1).contains(&x)
                        && !cerca_de_una_pieza(x, z)
                    {
                        n += 1;
                    }
                }
                vistas.push((giro, elevacion, n));
            }
        }
        vistas
    }

    /// RED/GREEN de la franja que vio Charlie desde una órbita trasera: con
    /// el relleno delantero solo, el plinto seco tras las masas se ve desde
    /// detrás; con el trasero, desde ninguna órbita.
    #[test]
    fn la_franja_tras_las_masas_no_se_ve_seca_desde_ninguna_orbita() {
        let antes = plinto_seco_visible_tras_las_masas(&con_relleno());
        let ahora = plinto_seco_visible_tras_las_masas(&con_trasero());
        let desde_atras = antes
            .iter()
            .find(|v| v.0 == 180.0 && v.1 == 35.0)
            .unwrap()
            .2;
        assert!(
            desde_atras > 500,
            "la sonda ve la franja con el relleno delantero: {desde_atras} px"
        );
        for (a, b) in antes.iter().zip(&ahora) {
            assert_eq!(
                b.2, 0,
                "giro {} elevacion {}: {} px secos ({} antes)",
                b.0, b.1, b.2, a.2
            );
        }
    }

    /// El relleno trasero solo se añade tras las 214, en los tres presets,
    /// con el agua de la cuenca y un fondo nuevo por pieza de lecho; no pasa
    /// bajo las masas ni corta nada; y continúa el agua existente.
    #[test]
    fn el_relleno_trasero_solo_se_anade_y_no_corta_nada() {
        for water in PRESETS {
            let p = delivery_level_previo_trasero(water);
            let d = delivery_level_previo_monolito(water);
            let sin_volumen = usize::from(water == WaterPreset::InteriorVisible);
            assert_eq!(
                d.scene.objects.len(),
                DELIVERY.total()
                    + CELDAS
                    + LECHOS
                    + RELLENO_AGUA
                    + RELLENO_LECHO
                    + TRASERO_AGUA
                    + TRASERO_LECHO
                    - sin_volumen
            );
            for (i, (x, y)) in p.scene.objects.iter().zip(&d.scene.objects).enumerate() {
                assert_eq!(format!("{x:?}"), format!("{y:?}"), "{water:?}: objeto {i}");
            }
            assert_eq!(d.scene.palette.len(), p.scene.palette.len() + TRASERO_LECHO);
            for (m, n) in p.scene.palette.iter().zip(&d.scene.palette) {
                assert_eq!(format!("{m:?}"), format!("{n:?}"));
            }
            assert_eq!(
                d.scene.textures.len(),
                p.scene.textures.len() + TRASERO_LECHO
            );
            let (agua_vieja, _) = cuenca(&delivery_level_previo_relleno(water).scene);
            let (agua, lecho) = trasero(&d, &p);
            let s = &d.scene;
            let piso = caja(s, 0).max.y;
            let material = p.scene.objects[agua_vieja[0]].final_material;
            let todas: Vec<Aabb> = (0..s.objects.len())
                .filter(|&i| {
                    s.objects[i].spatial_group == SpatialGroupId::Global
                        && s.objects[i].reveal_group == RevealGroup::Meadows
                        && caja(s, i).max.y > piso + LECHO.1 + EPS
                })
                .map(|i| caja(s, i))
                .collect();
            for &i in &agua {
                let c = caja(s, i);
                assert_eq!(s.objects[i].final_material, material);
                assert!((c.min.y - (piso - EMPOTRADO)).abs() < EPS);
                assert!((c.max.y - (piso + NIVEL)).abs() < EPS);
                for j in 1..s.objects.len() {
                    if j == i || lecho.contains(&j) {
                        continue;
                    }
                    assert!(
                        !solapan(&c, &caja(s, j)),
                        "{water:?}: el agua {i} corta {j}"
                    );
                }
                let toca = todas.iter().any(|o| {
                    *o != c
                        && (((o.max.x - c.min.x).abs() < EPS || (c.max.x - o.min.x).abs() < EPS)
                            && o.max.z.min(c.max.z) - o.min.z.max(c.min.z) > 0.05
                            || ((o.max.z - c.min.z).abs() < EPS || (c.max.z - o.min.z).abs() < EPS)
                                && o.max.x.min(c.max.x) - o.min.x.max(c.min.x) > 0.05)
                });
                assert!(toca, "{water:?}: la celda {i} queda suelta");
            }
            for &i in &lecho {
                let b = caja(s, i);
                let l = s.material(s.objects[i].final_material);
                assert_eq!(
                    (l.reflection_cap, l.transmission_cap, l.ior),
                    (0.0, 0.0, 1.0)
                );
                assert!(s.objects[i].final_material.0 >= p.scene.palette.len());
                for j in 1..s.objects.len() {
                    let o = caja(s, j);
                    let es_agua = s.objects[j].spatial_group == SpatialGroupId::Global
                        && o.max.y > piso + LECHO.1 + EPS;
                    if j != i && !es_agua {
                        assert!(!solapan(&b, &o), "{water:?}: el lecho {i} corta {j}");
                    }
                }
            }
        }
    }

    /// Las costuras con los dos rellenos, por toma, con el mismo criterio que
    /// el relleno delantero.
    #[test]
    fn las_costuras_siguen_acotadas_con_el_relleno_trasero() {
        let d = con_trasero();
        let s = &d.scene;
        let piso = caja(s, 0).max.y;
        let agua: Vec<usize> = (0..s.objects.len())
            .filter(|&i| {
                s.objects[i].spatial_group == SpatialGroupId::Global
                    && s.objects[i].reveal_group == RevealGroup::Meadows
                    && caja(s, i).max.y > piso + LECHO.1 + EPS
            })
            .collect();
        assert_eq!(agua.len(), CELDAS + RELLENO_AGUA + TRASERO_AGUA);
        let plantas: Vec<Aabb> = agua.iter().map(|&i| caja(s, i)).collect();
        let compartida = |x: f32, z: f32| {
            plantas
                .iter()
                .filter(|c| c.min.x <= x && x <= c.max.x && c.min.z <= z && z <= c.max.z)
                .count()
                >= 2
        };
        let (w, h) = (400, 300);
        for (camara, tope) in camaras(&d).into_iter().zip([0.02, 0.02, 0.025]) {
            let (mut n, mut costura) = (0usize, 0usize);
            for p in 0..w * h {
                let rayo = camara.ray_from_pixel(p % w, p / w, w, h);
                let mut st = Default::default();
                let Some(hit) = d.accel.intersect(s, &rayo, &mut st) else {
                    continue;
                };
                if !agua.contains(&hit.object_index) || !hit.front_face || hit.normal.y < 0.9 {
                    continue;
                }
                n += 1;
                let Some(dentro) = refracted_ray(&hit, &rayo.direction, 1.333) else {
                    continue;
                };
                if let Some(x) = d.accel.intersect(s, &dentro, &mut st) {
                    if agua.contains(&x.object_index)
                        && x.normal.y.abs() < 0.5
                        && compartida(x.point.x, x.point.z)
                    {
                        costura += 1;
                    }
                }
            }
            assert!(n > 500);
            assert!(costura as f32 / n as f32 <= tope, "{costura} de {n}");
        }
    }

    #[test]
    fn el_relleno_trasero_conserva_la_jerarquia_lineal_y_el_determinismo() {
        let d = con_trasero();
        let (w, h) = (120, 90);
        let mut todas: Vec<Camera> = camaras(&d).to_vec();
        for giro in [90.0f32, 180.0, 270.0] {
            todas.push(d.camera_at(HERO_YAW_DEGREES + giro, 35.0));
        }
        for camara in todas {
            for p in 0..w * h {
                let rayo = camara.ray_from_pixel(p % w, p / w, w, h);
                let mut stats = Default::default();
                assert_eq!(
                    d.accel
                        .intersect(&d.scene, &rayo, &mut stats)
                        .map(|x| x.object_index),
                    d.scene.intersect(&rayo).map(|x| x.object_index)
                );
            }
        }
        assert_eq!(
            format!("{:?}", d.scene.objects),
            format!("{:?}", con_trasero().scene.objects)
        );
    }

    /// La misma en las dos rutas, medida con los dos rellenos.
    const HUELLA_DEL_TRASERO: u64 = 9_682_438_666_188_897_094;

    #[test]
    fn la_huella_del_relleno_trasero_queda_fijada() {
        let p = con_relleno();
        assert_eq!(
            huella_del_relleno(&con_trasero(), &p.scene),
            HUELLA_DEL_TRASERO
        );
    }
}
