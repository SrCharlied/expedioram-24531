//! Pincel visual: el gizmo que se dibuja sobre la superficie que se pinta.
//!
//! # Qué es y qué no es
//!
//! No es un sprite pegado al cursor ni tres líneas gruesas. Es un **sólido
//! facetado** —mango ahusado, virola de latón clara y un abanico de
//! cerdas afiladas— cuyos vértices viven en coordenadas de mundo, anclados al
//! punto de contacto, y que se proyectan con `Camera::project_to_pixels`. Por
//! eso se escorza al orbitar, crece al acercarse y se apoya sobre la cara que
//! toca.
//!
//! El mango se texturiza con la madera del pecio, que el diorama ya tiene
//! cargada: la herramienta está hecha del mismo material que el naufragio que
//! flota en Aguas Voladoras.
//!
//! # Es un overlay, y a propósito
//!
//! Se dibuja **encima** del cuadro ya trazado. No hay oclusión contra el
//! mundo: si el pincel queda detrás de una roca, se sigue viendo entero. Es
//! deliberado —es una herramienta, no un objeto de la obra— y es la razón de
//! que no entre en la escena: añadirlo como geometría obligaría a reconstruir
//! la jerarquía de aceleración en cada cuadro, porque el gizmo se mueve con el
//! ratón, y metería una primitiva en el camino más caliente del renderer para
//! algo que no proyecta sombra ni recibe luz.
//!
//! Lo que sí tiene es **profundidad propia**: un z-buffer local acotado a su
//! caja en pantalla, para que sus propias caras se tapen entre sí. Sin eso el
//! orden de dibujo decidiría qué se ve y el sólido se vería del revés según el
//! ángulo.

use crate::camera::Camera;
use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::texture::Texture;
use nalgebra_glm::{dot, Vec3};

/// Lados del prisma. Seis: suficientes para que el ahusado se lea como un
/// cuerpo redondeado y pocos para que las facetas se distingan, que es lo que
/// le da el aire tallado a mano.
const LADOS: usize = 6;

/// Cuánto sobresale o se retrae cada mechón, en múltiplos del tamaño
/// visual. Negativo es **más largo**.
///
/// La desigualdad radial no basta: mover el borde hacia dentro y hacia fuera
/// deja igualmente una arista limpia, y un poliedro con arista limpia se lee
/// como una cuña tallada, no como pelo.
///
/// Esto mueve cada lado **a lo largo del eje**, y el signo alterna según
/// dónde cae ese lado en la **tangente**, no según su índice. La diferencia
/// importa: con seis lados y la sección aplanada, los lados `1` y `5` caen
/// en la misma abscisa, y los `2` y `4` en otra, así que solo hay cuatro
/// posiciones distintas a lo ancho. Alternando por índice, los dos lados de
/// una misma abscisa se movían juntos y el contorno seguía siendo monótono.
/// Alternando por abscisa —largo, corto, largo, corto— unos mechones asoman
/// por debajo de sus vecinos y el borde se quiebra.
///
/// Retranquearlos todos hacia el mango tampoco sirve: los esconde detrás de
/// los que quedan largos y el contorno no cambia.
///
/// Los que sobresalen cruzan el plano de contacto. Es un overlay, así que se
/// dibujan igual, y lo que se ve es lo que hace un pincel de verdad al
/// apoyarse: unas cerdas se aplastan contra la superficie.
///
/// Fijos y no aleatorios, por lo mismo que `DESIGUAL`: el gizmo se redibuja
/// en cada cuadro y un desorden recalculado haría hervir la punta.
/// Los mechones de la cabeza: desplazamiento lateral, dónde acaba la punta
/// y medio ancho, los tres en múltiplos del tamaño visual.
///
/// Siete piezas planas y finas en vez de un manojo tubular. Cada una nace
/// **dentro de la virola**, en `NACIMIENTO_DE_LOS_MECHONES`, y baja hasta su
/// propia punta: los largos están escalonados, así que las puntas no se
/// alinean y la cabeza no vuelve a acabar en un filo.
///
/// El desplazamiento va en la tangente, que es el eje ancho de la sección
/// plana. La separación entre centros supera el doble del medio ancho, y ese
/// margen es el hueco que se ve entre puntas: sin él los mechones se
/// tocarían y volverían a leerse como un bloque.
///
/// # Por qué siete, y por qué casi pegados
///
/// Fueron cinco, más anchos y más separados, y la revisión los describió
/// como un tenedor: con tan pocas piezas se cuentan de un vistazo y el hueco
/// pesa tanto como el pelo. Siete a la mitad de ancho arreglaron el recuento
/// pero dejaron dos píxeles de aire entre puntas, y la siguiente revisión los
/// llamó peine.
///
/// La separación está ahora calibrada a **un píxel exacto** al tamaño
/// típico: los centros se acercaron de `0.31` a `0.299` y los mechones se
/// ensancharon de `0.08` a `0.102`, manteniendo el ancho total de la cabeza
/// en `2.00` para no mover la proporción con la virola.
///
/// Un píxel es el único valor que funciona, y por las dos razones opuestas.
/// Con dos o más, el aire entre puntas vuelve a leerse como púas. Con cero,
/// el hueco no sobrevive a un cambio de ángulo ni al escalado del perfil
/// interactivo, y la cabeza se funde en un bloque. Lo mide
/// `la_cabeza_es_un_manojo_denso_y_no_un_tenedor`.
///
/// Los largos varían poco y sin patrón: los dos extremos algo más cortos
/// —un pincel se gasta por los bordes— y el resto repartido. Uniformes darían
/// otra vez un filo; muy dispares, una púa suelta.
///
/// Los dos mechones de los bordes no siguen la retícula: están algo más
/// cerca de su vecino y algo más gruesos. No es un capricho de composición,
/// es lo que midió el framebuffer. Ahí se cruzan las dos puntas de alturas
/// más dispares, y con la separación regular el hueco se abría a dos píxeles
/// justo en el borde de la cabeza, que es donde más se mira.
///
/// Fijos y no aleatorios, por lo mismo que todo lo demás del gizmo: se
/// redibuja en cada cuadro y un desorden recalculado haría hervir la punta.
const MECHONES: [(f32, f32, f32); 7] = [
    (-0.872, 0.30, 0.124),
    (-0.600, 0.16, 0.117),
    (-0.300, 0.22, 0.115),
    (0.000, 0.08, 0.118),
    (0.300, 0.19, 0.116),
    (0.600, 0.13, 0.117),
    (0.872, 0.28, 0.124),
];

/// Dónde arrancan los mechones, en el avance del perfil.
///
/// Dentro del tramo de virola —que va de `PERFIL[1]` a `PERFIL[2]`—, de modo
/// que salen de ella y no de un cuello al aire. El metal los tapa por
/// delante, que es exactamente lo que hace una virola.
const NACIMIENTO_DE_LOS_MECHONES: f32 = 1.72;

/// Medio ancho de un mechón en la bitangente, como fracción del de la
/// tangente. Muy plano: son pelos peinados, no varillas.
const APLANADO_DEL_MECHON: f32 = 0.42;

/// Lados de cada mechón. Cuatro: una sección rectangular redondeada basta
/// para que tenga volumen, y multiplicar facetas por cinco piezas saldría
/// caro sin verse.
const LADOS_DEL_MECHON: usize = 4;

/// Los anillos del perfil: distancia a lo largo del eje y radio, los dos en
/// múltiplos del tamaño visual del pincel.
///
/// **Cuatro anillos, tres tramos.** Uno por material y ninguno más.
///
/// La versión anterior tenía diez anillos, con la virola partida en tres
/// franjas y dos bandas pulidas entre ellas. A `800 x 600` el gizmo entero
/// mide unas setenta filas de pantalla: repartidas entre nueve tramos, cada
/// franja de virola caía por debajo de la media docena de píxeles y el
/// detalle no se leía, se emborronaba. El problema no era que faltara
/// detalle, era que sobraba, y añadir más habría empeorado lo mismo.
///
/// Ahora la silueta se lee de un vistazo: el abanico se abre en el contacto
/// y ocupa **un tercio** del largo, la virola es una banda única de latón
/// claro que sobresale del cuello, y el mango se afila en una sola caída
/// continua.
///
/// # La sección es **aplanada**, no circular
///
/// El tercer número es la razón entre el eje fino y el eje ancho. Con una
/// sección circular —todo a `1.0`— el abanico se lee como un cono o una gema
/// por mucho que se ensanche: un sólido de revolución no parece un pincel de
/// pintor desde ningún ángulo.
///
/// Aquí la cabeza es **plana**: ancha en la tangente, fina en la
/// perpendicular. El mango recupera la sección redonda al llegar al remate,
/// que es como está tallado un pincel de verdad.
///
/// # La proporción de la cabeza
///
/// Una versión anterior abrió el labio a `2.30` buscando que el abanico se
/// viera. Se pasó: con la virola en `0.78`, la cabeza salía más de cuatro
/// veces más ancha y el pincel se leía como una pala, con la virola
/// convertida en una línea perdida entre las cerdas y el mango.
///
/// El labio está ahora en `1.14`, aproximadamente **una vez y media** la
/// virola, y el tramo es **corto**: una punta trapezoidal, no un abanico. La
/// medida está fijada en `la_cabeza_guarda_proporcion_con_la_virola`.
const PERFIL: [(f32, f32, f32); 4] = [
    (0.00, 1.14, 0.30),
    (1.35, 0.60, 0.36),
    (2.25, 0.74, 0.46),
    (7.40, 0.17, 0.92),
];

/// Qué material lleva cada tramo entre anillos consecutivos.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Material {
    /// Color activo: lo que el pincel va a dejar.
    Cerdas,
    /// Latón claro de la virola.
    Virola,
    /// Madera del pecio, texturizada.
    Mango,
}

/// Un material por tramo, en el orden de `PERFIL`.
///
/// El primer tramo ya **no se dibuja como prisma**: la cabeza son los cinco
/// mechones de `MECHONES`. El anillo sigue en el perfil porque marca el
/// punto de contacto y fija la caja en pantalla del conjunto.
const TRAMOS: [Material; PERFIL.len() - 1] = [Material::Cerdas, Material::Virola, Material::Mango];

/// Latón de la virola, en lineal.
///
/// Claro a propósito: es la única pieza que separa las cerdas del mango, y
/// tiene que distinguirse de los dos a la primera. Un latón envejecido y
/// oscuro se confundiría con la madera justo donde hace falta el corte.
const LATON: Color = Color {
    r: 0.86,
    g: 0.72,
    b: 0.38,
};

/// Realce de la costura de la virola. Sobrio: un paso, no un destello.
const LATON_PULIDO: Color = Color {
    r: 0.97,
    g: 0.88,
    b: 0.58,
};

/// Madera de reserva cuando no se presta ninguna textura.
const MADERA_PLANA: Color = Color {
    r: 0.19,
    g: 0.11,
    b: 0.06,
};

/// Ganancia de la madera del pecio.
///
/// El mismo valor que `scenes::flying_waters` aplica a `aged_wood` para el
/// casco: la herramienta está hecha de la madera del naufragio, y con la
/// ganancia del material base saldría casi negra sobre el diorama.
const GANANCIA_DE_LA_MADERA: f32 = 3.2;

/// Cuánto se inclina el eje del pincel hacia la cámara, contra la normal.
///
/// Ni `0` ni `1`. Con todo el peso en la normal, el pincel apuntaría hacia
/// afuera de la cara y sobre una pared se vería de canto. Con todo el peso en
/// la cámara, quedaría apuntando al ojo y se vería **de punta**, que es
/// justamente lo que hay que evitar sobre las caras frontales.
const SESGO_HACIA_CAMARA: f32 = 0.30;

/// Inclinación tangencial, perpendicular al eje y en el plano de pantalla.
///
/// Es lo que garantiza que el pincel nunca apunte exactamente al ojo: sobre
/// una cara vista de frente se ve escorzado en diagonal, con longitud, en vez
/// de reducirse a una rosca de anillos concéntricos.
const INCLINACION_TANGENCIAL: f32 = 0.34;

/// Límites del **tamaño visual** del gizmo, en píxeles de radio proyectado.
///
/// No son los límites de la huella: el pincel puede pintar un radio
/// minúsculo o enorme, y eso lo gobiernan las constantes de `main`. Estos
/// gobiernan cómo se **ve** la herramienta, y existen por dos razones
/// opuestas:
///
/// - Por abajo, un gizmo proporcional a una huella diminuta desaparecería, y
///   un pincel invisible es indistinguible de uno roto.
/// - Por arriba, una brocha gigante taparía media obra justo cuando se está
///   mirando lo que se pinta.
///
/// El mínimo no es «un píxel»: por debajo de unos nueve píxeles de radio la
/// virola deja de distinguirse del mango y el pincel se
/// lee como una astilla. Es el tamaño en el que la silueta sigue siendo
/// legible, no el tamaño en el que todavía hay algo pintado.
///
/// Entre los dos el gizmo sí es proporcional, así que `M` y `N` siguen
/// notándose.
const RADIO_VISUAL_MINIMO_PX: f32 = 11.0;
const RADIO_VISUAL_MAXIMO_PX: f32 = 34.0;

/// Dirección de la luz del overlay, en la base de la cámara.
///
/// Fija respecto del observador y no del mundo: el gizmo es una herramienta
/// en la mano, no un objeto de la escena, y una luz que girase con la órbita
/// lo dejaría a oscuras la mitad del tiempo.
const LUZ_DE_OVERLAY: Vec3 = Vec3::new(-0.45, 0.65, 0.62);

/// Suelo de iluminación de una faceta, para que ninguna caiga a negro.
const AMBIENTE_DEL_GIZMO: f32 = 0.34;

/// Un vértice del sólido, ya proyectado.
#[derive(Clone, Copy)]
struct Vertice {
    /// Píxel continuo.
    pantalla: (f32, f32),
    /// Distancia a la cámara, para el z-buffer local.
    profundidad: f32,
    /// Coordenada de textura, para el mango.
    uv: (f32, f32),
}

/// Dibuja el pincel sobre el framebuffer. Devuelve si llegó a dibujar algo.
///
/// `point` y `normal` son los del impacto —`input::PaintHit` los entrega tal
/// cual—, `world_radius` es el radio de mundo con el que se pinta, y
/// `bristle_color` el color de las cerdas en sRGB empaquetado `0x00RRGGBB`.
///
/// `wood` es la textura del mango, **prestada**: no se carga nada aquí ni se
/// modifica el asset. Sin ella el mango sale de un marrón plano, que es un
/// degradado peor pero no un fallo.
///
/// # No dibuja nada, y no toca el framebuffer, si
///
/// - Alguna entrada no es finita, la normal es nula o el radio no es positivo.
/// - El sólido entero cae fuera del cuadro o detrás de la cámara.
///
/// Un gizmo que asoma por el borde **sí** se dibuja, recortado: lo que se
/// exige es que al menos una cara tenga píxeles dentro.
pub fn draw_brush_gizmo(
    framebuffer: &mut Framebuffer,
    camera: &Camera,
    point: &Vec3,
    normal: &Vec3,
    world_radius: f32,
    bristle_color: u32,
    wood: Option<&Texture>,
) -> bool {
    let (ancho_px, alto_px) = (framebuffer.width, framebuffer.height);

    if ancho_px == 0 || alto_px == 0 {
        return false;
    }

    let Some(base) = base_del_pincel(camera, point, normal, world_radius, alto_px) else {
        return false;
    };
    let (eje, tangente, bitangente, escala) = base;

    let Some(anillos) = anillos_proyectados(
        camera,
        point,
        &eje,
        &tangente,
        &bitangente,
        escala,
        ancho_px,
        alto_px,
    ) else {
        return false;
    };

    let cerdas = desempaquetar(bristle_color);
    let luz = direccion_de_luz(camera);

    // Caja en pantalla del sólido, recortada al viewport: el z-buffer y el
    // recorrido se acotan a ella en vez de a la ventana entera.
    let Some(caja) = caja_en_pantalla(&anillos, ancho_px, alto_px) else {
        return false;
    };
    let (x0, y0, x1, y1) = caja;
    let (caja_ancho, caja_alto) = (x1 - x0 + 1, y1 - y0 + 1);

    // Un solo z-buffer para todo el gizmo: los mechones se tapan entre sí y
    // contra la virola con el mismo criterio que el resto de las piezas.
    let mut profundidad = vec![f32::INFINITY; caja_ancho * caja_alto];

    let mut pintado = false;

    // La cabeza, primero: cinco mechones planos que nacen en la virola.
    for (indice, (lateral, punta, medio_ancho)) in MECHONES.iter().enumerate() {
        let arranque = point + eje * (NACIMIENTO_DE_LOS_MECHONES * escala);
        let remate = point + eje * (punta * escala);
        let desvio = tangente * (lateral * escala);

        let seccion = |centro: Vec3, ancho: f32| -> Option<[Vertice; LADOS_DEL_MECHON]> {
            let mut anillo = [Vertice {
                pantalla: (0.0, 0.0),
                profundidad: 0.0,
                uv: (0.0, 0.0),
            }; LADOS_DEL_MECHON];

            for (lado, vertice) in anillo.iter_mut().enumerate() {
                let angulo = std::f32::consts::TAU * (lado as f32 + 0.5) / LADOS_DEL_MECHON as f32;
                let radial =
                    tangente * angulo.cos() + bitangente * (angulo.sin() * APLANADO_DEL_MECHON);
                let mundo = centro + radial * (ancho * escala);

                *vertice = Vertice {
                    pantalla: proyectar(camera, &mundo, ancho_px, alto_px)?,
                    profundidad: dot(&(mundo - camera.eye), &camera.forward()),
                    uv: (lado as f32 / LADOS_DEL_MECHON as f32, 0.0),
                };
            }

            Some(anillo)
        };

        // La base es algo más gruesa que la punta: un mechón se afila. Muy
        // poco, y por una razón medida: con un afilado marcado, la última
        // fila de cada punta sale tan estrecha que el hueco con su vecina se
        // abre a dos píxeles y reaparecen las púas justo en el borde, que es
        // donde más se miran.
        let (Some(base), Some(cabo)) = (
            seccion(arranque + desvio, medio_ancho * 1.06),
            seccion(remate + desvio, *medio_ancho),
        ) else {
            continue;
        };

        for lado in 0..LADOS_DEL_MECHON {
            let siguiente = (lado + 1) % LADOS_DEL_MECHON;
            let quad = [base[lado], base[siguiente], cabo[siguiente], cabo[lado]];

            let color = color_de_faceta(
                Material::Cerdas,
                lado + indice,
                0,
                &cerdas,
                &luz,
                camera,
                &eje,
                &tangente,
                &bitangente,
                APLANADO_DEL_MECHON,
            );

            for triangulo in [[quad[0], quad[1], quad[2]], [quad[0], quad[2], quad[3]]] {
                pintado |= triangulo_relleno(
                    framebuffer,
                    &mut profundidad,
                    caja,
                    &triangulo,
                    color,
                    Material::Cerdas,
                    0,
                    wood,
                );
            }
        }
    }

    // La virola y el mango, sin cambios: el tramo `0` del perfil ya lo
    // cubren los mechones.
    for (tramo, material) in TRAMOS.iter().enumerate().skip(1) {
        for lado in 0..LADOS {
            let siguiente = (lado + 1) % LADOS;
            let quad = [
                anillos[tramo][lado],
                anillos[tramo][siguiente],
                anillos[tramo + 1][siguiente],
                anillos[tramo + 1][lado],
            ];

            // El aplanado medio del tramo: la normal de la faceta depende de
            // cuánto esté comprimida la sección donde vive.
            let aplanado = (PERFIL[tramo].2 + PERFIL[tramo + 1].2) * 0.5;
            let color = color_de_faceta(
                *material,
                lado,
                tramo,
                &cerdas,
                &luz,
                camera,
                &eje,
                &tangente,
                &bitangente,
                aplanado,
            );

            // Dos triángulos por faceta, que es la forma de rasterizar un
            // cuadrilátero sin suponer que sus cuatro vértices son coplanares
            // en pantalla.
            for triangulo in [[quad[0], quad[1], quad[2]], [quad[0], quad[2], quad[3]]] {
                pintado |= triangulo_relleno(
                    framebuffer,
                    &mut profundidad,
                    caja,
                    &triangulo,
                    color,
                    *material,
                    tramo,
                    wood,
                );
            }
        }
    }

    pintado
}

/// La base local del pincel y su escala visual, o `None` si las entradas no
/// permiten construirla.
///
/// Devuelve `(eje, tangente, bitangente, escala)`. La escala ya lleva
/// aplicado el recorte de tamaño visual.
fn base_del_pincel(
    camera: &Camera,
    point: &Vec3,
    normal: &Vec3,
    world_radius: f32,
    alto: usize,
) -> Option<(Vec3, Vec3, Vec3, f32)> {
    if !finito(point) || !finito(normal) || !world_radius.is_finite() || world_radius <= 0.0 {
        return None;
    }

    let normal = normalizado(normal)?;
    let hacia_camara = normalizado(&(camera.eye - point))?;
    let derecha = normalizado(&camera.forward().cross(&camera.up))?;

    // El eje del mango: la normal levantada hacia el observador, y luego
    // ladeada en tangencial para que nunca coincida con la línea de vista.
    let eje = normal * (1.0 - SESGO_HACIA_CAMARA) + hacia_camara * SESGO_HACIA_CAMARA;
    let tangente_cruda = derecha - eje * dot(&derecha, &eje);
    let tangente_cruda = if tangente_cruda.magnitude() > 1e-4 {
        tangente_cruda
    } else {
        // El eje quedó alineado con la derecha de cámara: cualquier otra
        // referencia sirve para desempatar.
        camera.up - eje * dot(&camera.up, &eje)
    };
    let tangente_cruda = normalizado(&tangente_cruda)?;

    let eje = normalizado(&(eje + tangente_cruda * INCLINACION_TANGENCIAL))?;
    let tangente = normalizado(&(tangente_cruda - eje * dot(&tangente_cruda, &eje)))?;
    let bitangente = normalizado(&eje.cross(&tangente))?;

    // Tamaño visual: se mide cuántos píxeles ocupa un radio de mundo y se
    // recorta al rango legible. Ver `RADIO_VISUAL_MINIMO_PX`.
    let escala = escala_visual(camera, point, world_radius, alto)?;

    Some((eje, tangente, bitangente, escala))
}

/// Tamaño en mundo que hay que dar al gizmo para que su radio proyectado
/// caiga dentro del rango legible.
fn escala_visual(camera: &Camera, point: &Vec3, world_radius: f32, alto: usize) -> Option<f32> {
    let hacia = point - camera.eye;
    let profundidad = dot(&hacia, &camera.forward());

    if profundidad <= crate::EPSILON || !profundidad.is_finite() {
        return None;
    }

    let media_altura = (camera.vertical_fov / 2.0).tan() * profundidad;

    if media_altura <= 0.0 || !media_altura.is_finite() {
        return None;
    }

    // Píxeles por unidad de mundo a esa profundidad, medidos contra el
    // **viewport real**. Con una altura de referencia fija, el límite
    // mínimo valdría una cosa a 800 x 600 y otra en una ventana pequeña, y
    // el pincel se evaporaría en la segunda.
    let px_por_unidad = (alto as f32 / 2.0) / media_altura;
    let radio_px = world_radius * px_por_unidad;

    if !radio_px.is_finite() || radio_px <= 0.0 {
        return None;
    }

    let recortado = radio_px.clamp(RADIO_VISUAL_MINIMO_PX, RADIO_VISUAL_MAXIMO_PX);

    Some(recortado / px_por_unidad)
}

/// Los anillos del perfil, ya proyectados a pantalla.
///
/// `None` si ningún anillo se proyecta: el sólido está entero fuera o detrás.
#[allow(clippy::too_many_arguments)]
fn anillos_proyectados(
    camera: &Camera,
    point: &Vec3,
    eje: &Vec3,
    tangente: &Vec3,
    bitangente: &Vec3,
    escala: f32,
    ancho: usize,
    alto: usize,
) -> Option<Vec<[Vertice; LADOS]>> {
    let mut anillos = Vec::with_capacity(PERFIL.len());

    for (indice, (avance, radio, aplanado)) in PERFIL.iter().enumerate() {
        let centro = point + eje * (avance * escala);
        let mut anillo = [Vertice {
            pantalla: (0.0, 0.0),
            profundidad: 0.0,
            uv: (0.0, 0.0),
        }; LADOS];

        for (lado, vertice) in anillo.iter_mut().enumerate() {
            let angulo = std::f32::consts::TAU * lado as f32 / LADOS as f32;

            // El eje ancho va en la tangente y el fino en la bitangente,
            // recortado por `aplanado`. Es lo único que convierte el sólido
            // de revolución en una pala.
            let radial = tangente * angulo.cos() + bitangente * (angulo.sin() * aplanado);
            let mundo = centro + radial * (radio * escala);

            // Sin recorte al viewport: un vértice fuera del cuadro sigue
            // siendo un vértice válido del sólido, y descartarlo partiría las
            // facetas que asoman por el borde.
            let hacia = mundo - camera.eye;
            let profundidad = dot(&hacia, &camera.forward());

            if profundidad <= crate::EPSILON || !profundidad.is_finite() {
                return None;
            }

            let pantalla = proyectar(camera, &mundo, ancho, alto)?;

            *vertice = Vertice {
                pantalla,
                profundidad,
                uv: (
                    lado as f32 / LADOS as f32,
                    indice as f32 / (PERFIL.len() - 1) as f32,
                ),
            };
        }

        anillos.push(anillo);
    }

    Some(anillos)
}

/// Proyección a píxel continuo **sin** recortar al viewport.
///
/// `Camera::project_to_pixels` descarta lo que cae fuera, que es lo correcto
/// para señalar un punto pero no para rasterizar un sólido que asoma por el
/// borde. Aquí se reutiliza su resultado cuando cae dentro y se calcula la
/// misma fórmula cuando cae fuera, con la misma base.
fn proyectar(camera: &Camera, mundo: &Vec3, ancho: usize, alto: usize) -> Option<(f32, f32)> {
    if let Some(pixel) = camera.project_to_pixels(mundo, ancho, alto) {
        return Some(pixel);
    }

    let hacia = mundo - camera.eye;
    let forward = camera.forward();
    let derecha = normalizado(&forward.cross(&camera.up))?;
    let arriba = normalizado(&derecha.cross(&forward))?;
    let profundidad = dot(&hacia, &forward);

    if profundidad <= crate::EPSILON || !profundidad.is_finite() {
        return None;
    }

    let aspecto = ancho as f32 / alto as f32;
    let escala = (camera.vertical_fov / 2.0).tan();
    let sx = dot(&hacia, &derecha) / (profundidad * aspecto * escala);
    let sy = dot(&hacia, &arriba) / (profundidad * escala);

    let x = (sx + 1.0) * ancho as f32 / 2.0;
    let y = (1.0 - sy) * alto as f32 / 2.0;

    (x.is_finite() && y.is_finite()).then_some((x, y))
}

/// Caja de los anillos en pantalla, recortada al viewport.
fn caja_en_pantalla(
    anillos: &[[Vertice; LADOS]],
    ancho: usize,
    alto: usize,
) -> Option<(usize, usize, usize, usize)> {
    let (mut min_x, mut min_y) = (f32::INFINITY, f32::INFINITY);
    let (mut max_x, mut max_y) = (f32::NEG_INFINITY, f32::NEG_INFINITY);

    for anillo in anillos {
        for vertice in anillo {
            min_x = min_x.min(vertice.pantalla.0);
            max_x = max_x.max(vertice.pantalla.0);
            min_y = min_y.min(vertice.pantalla.1);
            max_y = max_y.max(vertice.pantalla.1);
        }
    }

    if !min_x.is_finite() || !max_x.is_finite() || !min_y.is_finite() || !max_y.is_finite() {
        return None;
    }

    // Entero fuera del cuadro: no hay nada que rasterizar.
    if max_x < 0.0 || max_y < 0.0 || min_x >= ancho as f32 || min_y >= alto as f32 {
        return None;
    }

    let x0 = min_x.floor().max(0.0) as usize;
    let y0 = min_y.floor().max(0.0) as usize;
    let x1 = (max_x.ceil() as isize).clamp(0, ancho as isize - 1) as usize;
    let y1 = (max_y.ceil() as isize).clamp(0, alto as isize - 1) as usize;

    (x0 <= x1 && y0 <= y1).then_some((x0, y0, x1, y1))
}

/// Dirección de la luz del overlay, llevada de la base de cámara al mundo.
fn direccion_de_luz(camera: &Camera) -> Vec3 {
    camera.basis_change(&LUZ_DE_OVERLAY)
}

/// Desempaqueta un color sRGB a lineal.
///
/// `from_srgb` y no un simple reparto en tres canales: el sombreado y la
/// mezcla del gizmo ocurren en lineal, y `Color::to_hex` vuelve a codificar
/// al escribir. Tratar el color empaquetado como si ya fuera lineal lo
/// aclararía dos veces, y el carmesí y el cian saldrían del mismo beige
/// lavado que el dorado.
fn desempaquetar(color: u32) -> Color {
    Color::from_srgb(
        ((color >> 16) & 0xFF) as f32 / 255.0,
        ((color >> 8) & 0xFF) as f32 / 255.0,
        (color & 0xFF) as f32 / 255.0,
    )
}

fn finito(v: &Vec3) -> bool {
    v.x.is_finite() && v.y.is_finite() && v.z.is_finite()
}

fn normalizado(v: &Vec3) -> Option<Vec3> {
    let largo = v.magnitude();

    (largo.is_finite() && largo > 1e-6).then(|| v / largo)
}

/// Color base de una faceta, ya sombreado.
///
/// El sombreado es un lambert de una sola luz fija más ambiente: suficiente
/// para que las seis caras del prisma se distingan y el cuerpo se lea como un
/// volumen, y lo bastante barato para ejecutarse una vez por faceta.
///
/// Las cerdas además varían de tono por lado y por altura: es la estría, y es
/// lo que impide que el abanico se vea como una pieza de plástico.
#[allow(clippy::too_many_arguments)]
fn color_de_faceta(
    material: Material,
    lado: usize,
    tramo: usize,
    cerdas: &Color,
    luz: &Vec3,
    camera: &Camera,
    eje: &Vec3,
    tangente: &Vec3,
    bitangente: &Vec3,
    aplanado: f32,
) -> Color {
    let _ = (camera, eje);

    // Normal de la faceta: la radial del prisma en el ángulo medio del lado.
    //
    // Sobre una sección aplanada la normal de una cara **no** es su radial:
    // el aplanado comprime el sólido en la bitangente, así que la normal se
    // inclina al revés, dividiendo por el mismo factor en vez de multiplicar.
    // Sin esa corrección las caras anchas de la pala se sombrearían como si
    // aún fueran las de un cilindro y el volumen se leería mal.
    let angulo = std::f32::consts::TAU * (lado as f32 + 0.5) / LADOS as f32;
    let aplanado = aplanado.max(1e-3);
    let normal = normalizado(&(tangente * angulo.cos() + bitangente * (angulo.sin() / aplanado)))
        .unwrap_or(*tangente);

    let difusa = dot(&normal, luz).max(0.0);
    let intensidad = AMBIENTE_DEL_GIZMO + (1.0 - AMBIENTE_DEL_GIZMO) * difusa;

    let base = match material {
        Material::Cerdas => {
            // Estrías: tres niveles de tono repartidos por lado, no dos. Con
            // dos alternando, el manojo se lee como un objeto bicolor; con
            // tres se lee como mechones de distinto grosor peinados desde la
            // virola hacia la punta.
            //
            // Pocas y de contraste corto a propósito: lo que se busca es
            // romper la superficie plana, no dibujar pelo a pelo.
            const ESTRIAS: [f32; 3] = [1.06, 0.88, 0.97];
            let veta = ESTRIAS[lado % ESTRIAS.len()];
            let altura = if tramo == 0 { 1.10 } else { 1.0 };

            *cerdas * (veta * altura)
        }
        // La virola no es un color plano con un brillo suelto: es una
        // rampa. Las caras recorren sombra, latón y realce según su lado,
        // que es como se ve un anillo metálico girando la luz alrededor. Con
        // un único brillo en una cara, el resalte se perdía en cuanto esa
        // cara quedaba fuera de vista.
        Material::Virola => {
            let vuelta = lado as f32 / LADOS as f32;
            let rampa = 0.52 + 0.48 * (std::f32::consts::TAU * vuelta).cos().mul_add(0.5, 0.5);

            LATON * rampa + LATON_PULIDO * (rampa * rampa * 0.35)
        }
        Material::Mango => MADERA_PLANA,
    };

    base * intensidad
}

/// Rasteriza un triángulo con z-buffer local. Devuelve si escribió algo.
#[allow(clippy::too_many_arguments)]
fn triangulo_relleno(
    framebuffer: &mut Framebuffer,
    profundidad: &mut [f32],
    caja: (usize, usize, usize, usize),
    triangulo: &[Vertice; 3],
    color: Color,
    material: Material,
    tramo: usize,
    wood: Option<&Texture>,
) -> bool {
    let (cx0, cy0, cx1, cy1) = caja;
    let caja_ancho = cx1 - cx0 + 1;

    let (ax, ay) = triangulo[0].pantalla;
    let (bx, by) = triangulo[1].pantalla;
    let (cx, cy) = triangulo[2].pantalla;

    let area = (bx - ax) * (cy - ay) - (by - ay) * (cx - ax);

    if area.abs() < 1e-6 || !area.is_finite() {
        return false;
    }

    // Caja del triángulo, ya dentro de la del sólido.
    let min_x = ax.min(bx).min(cx).floor().max(cx0 as f32) as usize;
    let max_x = (ax.max(bx).max(cx).ceil() as isize).clamp(cx0 as isize, cx1 as isize) as usize;
    let min_y = ay.min(by).min(cy).floor().max(cy0 as f32) as usize;
    let max_y = (ay.max(by).max(cy).ceil() as isize).clamp(cy0 as isize, cy1 as isize) as usize;

    if min_x > max_x || min_y > max_y {
        return false;
    }

    let mut pintado = false;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);

            // Baricéntricas, con el signo del área para no depender del
            // sentido en que quedó el triángulo tras proyectar.
            let w0 = ((bx - px) * (cy - py) - (by - py) * (cx - px)) / area;
            let w1 = ((cx - px) * (ay - py) - (cy - py) * (ax - px)) / area;
            let w2 = 1.0 - w0 - w1;

            if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                continue;
            }

            let z = w0 * triangulo[0].profundidad
                + w1 * triangulo[1].profundidad
                + w2 * triangulo[2].profundidad;

            let celda = (y - cy0) * caja_ancho + (x - cx0);

            if z >= profundidad[celda] {
                continue;
            }

            let u = w0 * triangulo[0].uv.0 + w1 * triangulo[1].uv.0 + w2 * triangulo[2].uv.0;
            let v = w0 * triangulo[0].uv.1 + w1 * triangulo[1].uv.1 + w2 * triangulo[2].uv.1;

            // Dónde cae el punto **dentro de su tramo**, de `0` en el anillo
            // de abajo a `1` en el de arriba.
            let local = (v * (PERFIL.len() - 1) as f32 - tramo as f32).clamp(0.0, 1.0);

            let final_ = match material {
                // Las dos costuras de la virola: no hay anillos extra que
                // las dibujen, así que se insinúan oscureciendo los bordes
                // del tramo. El exponente alto las deja finas, como una
                // junta, en vez de degradar la banda entera.
                Material::Virola => {
                    let borde = (2.0 * local - 1.0).abs();

                    color * (1.0 - 0.62 * borde.powi(6))
                }
                Material::Mango => {
                    // Sombra longitudinal: el mango es un cilindro, y lo que
                    // lo delata es una franja oscura recorriéndolo de punta a
                    // punta por el lado contrario a la luz.
                    let vuelta = std::f32::consts::TAU * u;
                    let cilindro = 0.58 + 0.42 * vuelta.cos().mul_add(0.5, 0.5);

                    // Marcas de la madera, discontinuas: la `v` se muestrea a
                    // un paso que no encaja con el perímetro, así que las
                    // vetas no se alinean en anillos ni en rayas paralelas.
                    let veta = match wood {
                        Some(tela) => {
                            tela.sample(u * 3.0 + v * 0.7, v * 4.3 + u * 0.4)
                                * GANANCIA_DE_LA_MADERA
                        }
                        None => MADERA_PLANA,
                    };

                    Color::new(
                        veta.r * color.r / MADERA_PLANA.r.max(1e-4),
                        veta.g * color.g / MADERA_PLANA.g.max(1e-4),
                        veta.b * color.b / MADERA_PLANA.b.max(1e-4),
                    ) * cilindro
                }
                Material::Cerdas => color,
            };

            profundidad[celda] = z;
            framebuffer.set_current_color(final_.to_hex());
            framebuffer.point(x, y);
            pintado = true;
        }
    }

    pintado
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera::DEFAULT_VERTICAL_FOV;
    use crate::color::Color;
    use crate::texture::Texture;

    const ANCHO: usize = 240;
    const ALTO: usize = 180;
    const CERDAS: u32 = 0x00E8C86A;

    fn camara_en(ojo: Vec3) -> Camera {
        Camera::new(
            ojo,
            Vec3::zeros(),
            Vec3::zeros(),
            Vec3::new(0.0, 1.0, 0.0),
            DEFAULT_VERTICAL_FOV,
        )
    }

    fn lienzo() -> Framebuffer {
        Framebuffer::new(ANCHO, ALTO)
    }

    /// Madera de prueba con vetas marcadas: dos tonos que se alternan por
    /// filas, para que muestrearla o no muestrearla se note.
    fn madera() -> Texture {
        let (w, h) = (8usize, 8usize);
        let pixeles = (0..w * h)
            .map(|i| {
                if (i / w) % 2 == 0 {
                    Color::new(0.38, 0.22, 0.11)
                } else {
                    Color::new(0.12, 0.07, 0.03)
                }
            })
            .collect();

        Texture::from_pixels(w, h, pixeles).expect("la madera de prueba es valida")
    }

    fn arriba() -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }

    /// Dibuja el gizmo de referencia sobre un lienzo limpio.
    fn pintar(madera: Option<&Texture>, punto: Vec3, radio: f32) -> Framebuffer {
        let mut framebuffer = lienzo();

        draw_brush_gizmo(
            &mut framebuffer,
            &camara_en(Vec3::new(0.0, 4.0, 9.0)),
            &punto,
            &arriba(),
            radio,
            CERDAS,
            madera,
        );

        framebuffer
    }

    fn pintados(framebuffer: &Framebuffer) -> usize {
        framebuffer.buffer.iter().filter(|p| **p != 0).count()
    }

    /// Cuántos colores distintos hay, sin contar el fondo.
    fn tonos(framebuffer: &Framebuffer) -> usize {
        let mut vistos: Vec<u32> = framebuffer
            .buffer
            .iter()
            .copied()
            .filter(|p| *p != 0)
            .collect();
        vistos.sort_unstable();
        vistos.dedup();

        vistos.len()
    }

    /// Anchura en píxeles de la fila `y`, contando de la primera a la
    /// última columna pintada.
    fn anchura_de_fila(framebuffer: &Framebuffer, y: usize) -> usize {
        let fila = &framebuffer.buffer[y * ANCHO..(y + 1) * ANCHO];
        let primera = fila.iter().position(|p| *p != 0);
        let ultima = fila.iter().rposition(|p| *p != 0);

        match (primera, ultima) {
            (Some(a), Some(b)) => b - a + 1,
            _ => 0,
        }
    }

    /// Filas con algo pintado, de arriba abajo.
    fn filas_pintadas(framebuffer: &Framebuffer) -> Vec<usize> {
        (0..ALTO)
            .filter(|y| anchura_de_fila(framebuffer, *y) > 0)
            .collect()
    }

    #[test]
    fn el_pincel_visible_pinta_y_deja_el_color_de_las_cerdas() {
        let framebuffer = pintar(Some(&madera()), Vec3::zeros(), 0.35);

        assert!(pintados(&framebuffer) > 0, "no pinto nada");

        // Las cerdas llevan el color activo, modulado por el sombreado de
        // cada faceta: no se exige el valor exacto, sino que su tono
        // domine sobre los otros canales igual que en el color pedido.
        let cerdas: Vec<u32> = framebuffer
            .buffer
            .iter()
            .copied()
            .filter(|p| {
                let (r, g, b) = (p >> 16 & 0xFF, p >> 8 & 0xFF, p & 0xFF);

                r > b && g > b && r >= g
            })
            .collect();

        assert!(
            cerdas.len() > 20,
            "no se reconoce el color de las cerdas: {} pixeles",
            cerdas.len()
        );
    }

    #[test]
    fn el_mango_usa_la_textura_de_madera() {
        // La misma escena con y sin tela tiene que dar imagenes distintas:
        // si la textura no se muestreara, el mango saldria de un color
        // plano y los dos buffers coincidirian.
        let con = pintar(Some(&madera()), Vec3::zeros(), 0.35);
        let sin = pintar(None, Vec3::zeros(), 0.35);

        assert!(pintados(&con) > 0 && pintados(&sin) > 0);
        assert_ne!(
            con.buffer, sin.buffer,
            "la textura de madera no cambio nada"
        );

        // Y con tela hay mas variedad de tono: la veta aporta valores que
        // un mango plano no tiene.
        assert!(
            tonos(&con) > tonos(&sin),
            "con madera {} tonos, sin madera {}",
            tonos(&con),
            tonos(&sin)
        );
    }

    #[test]
    fn las_facetas_no_son_todas_del_mismo_tono() {
        // Geometria facetada con sombreado local: cada cara recibe la luz
        // de overlay con un angulo distinto. Con tres lineas planas habria
        // tres colores y nada mas.
        let framebuffer = pintar(Some(&madera()), Vec3::zeros(), 0.35);

        assert!(
            tonos(&framebuffer) >= 10,
            "solo {} tonos: la silueta no esta facetada",
            tonos(&framebuffer)
        );
    }

    #[test]
    fn la_silueta_es_mas_ancha_en_las_cerdas_que_en_el_remate() {
        // Lo que hace que se lea como un pincel y no como un palo: el
        // abanico de cerdas abajo y el mango ahusado arriba.
        let framebuffer = pintar(Some(&madera()), Vec3::zeros(), 0.4);
        let filas = filas_pintadas(&framebuffer);

        assert!(filas.len() > 20, "la silueta es demasiado corta");

        let alto_de_todo = filas.len();
        let cerca_de_las_cerdas = filas[alto_de_todo - 1 - alto_de_todo / 10];
        let cerca_del_remate = filas[alto_de_todo / 10];

        let ancho_cerdas = anchura_de_fila(&framebuffer, cerca_de_las_cerdas);
        let ancho_remate = anchura_de_fila(&framebuffer, cerca_del_remate);

        assert!(
            ancho_cerdas > ancho_remate,
            "cerdas {ancho_cerdas} px contra remate {ancho_remate} px"
        );
    }

    #[test]
    fn un_gizmo_que_no_se_dibuja_no_toca_el_framebuffer() {
        let camara = camara_en(Vec3::new(0.0, 4.0, 9.0));
        let tela = madera();
        let limpio = lienzo();

        let casos: [(Vec3, Vec3, f32); 8] = [
            (Vec3::new(f32::NAN, 0.0, 0.0), arriba(), 0.35),
            (Vec3::zeros(), Vec3::new(f32::INFINITY, 0.0, 0.0), 0.35),
            (Vec3::zeros(), arriba(), f32::NAN),
            (Vec3::zeros(), arriba(), 0.0),
            (Vec3::zeros(), arriba(), -0.5),
            (Vec3::zeros(), Vec3::zeros(), 0.35),
            // Detras de la camara.
            (Vec3::new(0.0, 4.0, 60.0), arriba(), 0.35),
            // Tan a un lado que el gizmo entero se sale del cuadro.
            (Vec3::new(60.0, 0.0, 0.0), arriba(), 0.35),
        ];

        for (punto, normal, radio) in casos {
            let mut framebuffer = lienzo();

            let dibujo = draw_brush_gizmo(
                &mut framebuffer,
                &camara,
                &punto,
                &normal,
                radio,
                CERDAS,
                Some(&tela),
            );

            assert!(
                !dibujo,
                "{punto:?} / {normal:?} / {radio} no deberia dibujar"
            );
            assert_eq!(
                framebuffer.buffer, limpio.buffer,
                "{punto:?} / {normal:?} / {radio} toco el framebuffer"
            );
        }
    }

    #[test]
    fn el_gizmo_sigue_al_punto_y_no_se_queda_clavado() {
        let tela = madera();
        let izquierda = pintar(Some(&tela), Vec3::new(-1.5, 0.0, 0.0), 0.3);
        let derecha = pintar(Some(&tela), Vec3::new(1.5, 0.0, 0.0), 0.3);

        let centro = |f: &Framebuffer| {
            let mut suma = 0.0f32;
            let mut n = 0usize;

            for (i, pixel) in f.buffer.iter().enumerate() {
                if *pixel != 0 {
                    suma += (i % ANCHO) as f32;
                    n += 1;
                }
            }

            suma / n.max(1) as f32
        };

        assert!(
            centro(&derecha) - centro(&izquierda) > 10.0,
            "el gizmo no se movio con el punto"
        );
    }

    #[test]
    fn el_gizmo_se_mueve_al_mover_la_camara() {
        let tela = madera();
        // Lejos del eje de orbita: sobre el eje las dos camaras lo ven casi
        // en el mismo sitio y el test no probaria el paralaje.
        let punto = Vec3::new(2.4, 0.0, 0.0);

        let mut frente = lienzo();
        let mut ladeada = lienzo();

        assert!(draw_brush_gizmo(
            &mut frente,
            &camara_en(Vec3::new(0.0, 4.0, 9.0)),
            &punto,
            &arriba(),
            0.3,
            CERDAS,
            Some(&tela)
        ));
        assert!(draw_brush_gizmo(
            &mut ladeada,
            &camara_en(Vec3::new(6.0, 4.0, 7.0)),
            &punto,
            &arriba(),
            0.3,
            CERDAS,
            Some(&tela)
        ));

        assert_ne!(
            frente.buffer, ladeada.buffer,
            "el mismo punto bajo dos camaras dio la misma imagen"
        );
    }

    #[test]
    fn el_radio_cambia_el_tamano_dentro_de_los_limites_visuales() {
        let tela = madera();
        // Los dos radios **dentro** del rango visual: fuera de el los dos
        // quedarian recortados al mismo tamano y el test no probaria nada.
        let fino = pintar(Some(&tela), Vec3::zeros(), 0.70);
        let grueso = pintar(Some(&tela), Vec3::zeros(), 1.80);

        assert!(
            pintados(&grueso) > pintados(&fino),
            "el radio no cambio el tamano: {} contra {}",
            pintados(&fino),
            pintados(&grueso)
        );
    }

    #[test]
    fn un_radio_diminuto_sigue_dibujando_un_pincel_legible() {
        // El limite visual por abajo: la huella real puede ser minuscula,
        // pero la herramienta tiene que seguir viendose.
        let framebuffer = pintar(Some(&madera()), Vec3::zeros(), 0.002);

        assert!(
            pintados(&framebuffer) > 80,
            "el pincel se evaporo: {} pixeles",
            pintados(&framebuffer)
        );
    }

    #[test]
    fn un_radio_enorme_no_invade_el_cuadro_entero() {
        // Y el limite por arriba: pintar con una brocha gigante no puede
        // tapar la obra.
        let framebuffer = pintar(Some(&madera()), Vec3::zeros(), 40.0);
        let total = ANCHO * ALTO;

        assert!(
            pintados(&framebuffer) < total / 2,
            "el gizmo ocupo {} de {total} pixeles",
            pintados(&framebuffer)
        );
    }

    #[test]
    fn el_gizmo_no_escribe_fuera_del_framebuffer() {
        let camara = camara_en(Vec3::new(0.0, 4.0, 9.0));
        let tela = madera();

        for x in [-3.0f32, -2.0, 0.0, 2.0, 3.0] {
            let mut framebuffer = lienzo();

            draw_brush_gizmo(
                &mut framebuffer,
                &camara,
                &Vec3::new(x, 0.0, 0.0),
                &arriba(),
                0.3,
                CERDAS,
                Some(&tela),
            );

            assert_eq!(framebuffer.buffer.len(), ANCHO * ALTO);
        }
    }

    /// Madera de prueba **verde-gris**.
    ///
    /// No imita al asset real: su única razón de ser es que cada material
    /// del gizmo domine un canal distinto —cerdas el azul, latón el rojo,
    /// mango el verde—, de modo que clasificar un píxel sea exacto y no un
    /// umbral de luminancia que se rompa al retocar un color.
    fn madera_separable() -> Texture {
        let (w, h) = (8usize, 8usize);
        let pixeles = (0..w * h)
            .map(|i| {
                if (i / w) % 2 == 0 {
                    Color::new(0.10, 0.16, 0.12)
                } else {
                    Color::new(0.04, 0.07, 0.05)
                }
            })
            .collect();

        Texture::from_pixels(w, h, pixeles).expect("la madera de prueba es valida")
    }

    /// A qué material pertenece un píxel, por canal dominante.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum Zona {
        Cerdas,
        Virola,
        Mango,
    }

    fn zona_de(pixel: u32) -> Option<Zona> {
        if pixel == 0 {
            return None;
        }

        let (r, g, b) = (pixel >> 16 & 0xFF, pixel >> 8 & 0xFF, pixel & 0xFF);

        if b > r && b > g {
            Some(Zona::Cerdas)
        } else if r >= g && r > b {
            Some(Zona::Virola)
        } else if g > r && g >= b {
            Some(Zona::Mango)
        } else {
            None
        }
    }

    /// Por cada fila con pintura: qué material domina y su anchura de
    /// extremo a extremo.
    ///
    /// Anchura y no recuento de píxeles: lo que se juzga de una silueta es
    /// cuánto abarca, y un tramo con un hueco interior sigue siendo igual de
    /// ancho. La anchura se mide **dentro de la zona dominante**, porque en
    /// la fila donde dos piezas se tocan la extension total pertenece a las
    /// dos y no describe a ninguna.
    fn franjas(framebuffer: &Framebuffer, ancho: usize, alto: usize) -> Vec<(Zona, usize)> {
        let mut salida = Vec::new();

        for y in 0..alto {
            let (mut cerdas, mut virola, mut mango) = (0usize, 0usize, 0usize);
            let mut extremos = [(usize::MAX, 0usize); 3];

            for x in 0..ancho {
                let indice = match zona_de(framebuffer.buffer[y * ancho + x]) {
                    Some(Zona::Cerdas) => {
                        cerdas += 1;
                        0
                    }
                    Some(Zona::Virola) => {
                        virola += 1;
                        1
                    }
                    Some(Zona::Mango) => {
                        mango += 1;
                        2
                    }
                    None => continue,
                };

                extremos[indice].0 = extremos[indice].0.min(x);
                extremos[indice].1 = extremos[indice].1.max(x);
            }

            if cerdas + virola + mango == 0 {
                continue;
            }

            let (zona, indice) = if cerdas >= virola && cerdas >= mango {
                (Zona::Cerdas, 0)
            } else if virola >= mango {
                (Zona::Virola, 1)
            } else {
                (Zona::Mango, 2)
            };

            // La anchura es la de **esa** zona, no la de la fila entera: en
            // la frontera entre dos piezas la fila abarca las dos, y medirla
            // completa inflaria la mas estrecha hasta igualarlas.
            let (primera, ultima) = extremos[indice];

            salida.push((zona, ultima - primera + 1));
        }

        salida
    }

    #[test]
    fn el_abanico_de_cerdas_domina_la_punta_a_resolucion_de_ventana() {
        // La intención aprobada: silueta inmediata. A `800 x 600`, que es lo
        // que se presenta, el abanico tiene que ser **lo primero que se ve**
        // del pincel, no una puntita bajo una virola larga.
        //
        // Se mide sobre el cuadro real y no sobre las constantes: lo que
        // importa es cuánto ocupa cada material en pantalla después de
        // proyectar, recortar el tamaño visual y sombrear.
        const W: usize = 800;
        const H: usize = 600;

        let camara = Camera::new(
            Vec3::new(0.0, 12.0, 26.0),
            Vec3::zeros(),
            Vec3::zeros(),
            Vec3::new(0.0, 1.0, 0.0),
            DEFAULT_VERTICAL_FOV,
        );
        let mut framebuffer = Framebuffer::new(W, H);

        assert!(draw_brush_gizmo(
            &mut framebuffer,
            &camara,
            &Vec3::zeros(),
            &Vec3::new(0.0, 1.0, 0.0),
            0.30,
            0x0000C8FF,
            Some(&madera_separable()),
        ));

        let franjas = franjas(&framebuffer, W, H);
        let alto_total = franjas.len();

        assert!(alto_total > 30, "el gizmo mide {alto_total} filas");

        let filas_de = |zona: Zona| franjas.iter().filter(|(z, _)| *z == zona).count();

        let cerdas = filas_de(Zona::Cerdas);
        let virola = filas_de(Zona::Virola);
        let mango = filas_de(Zona::Mango);

        // El abanico ocupa una porcion reconocible del largo visible.
        //
        // Fue un tercio hasta que la revision visual rechazo esa version:
        // una cabeza tan larga y ancha se leia como una pala. Ahora la punta
        // es **corta y trapezoidal**, y lo que se exige es que siga siendo
        // una pieza distinguible, no que domine la silueta. La proporcion
        // buena la fija `la_cabeza_guarda_proporcion_con_la_virola`.
        assert!(
            cerdas * 7 >= alto_total,
            "las cerdas ocupan {cerdas} de {alto_total} filas; cerdas/virola/mango = {cerdas}/{virola}/{mango}"
        );

        // La proporcion de la cabeza **no** se comprueba aqui.
        //
        // Hubo una asercion de `cerdas` contra `mango`, pero el mango
        // arranca en el mismo anillo que la virola, asi que su anchura
        // maxima **es** la de la virola: medir contra el mango era medir por
        // segunda vez lo que ya fija
        // `la_cabeza_guarda_proporcion_con_la_virola`, y con un solo limite
        // en vez de una banda. Dos comprobaciones de lo mismo con umbrales
        // distintos no dan mas garantia; dan una ventana imposible en cuanto
        // los dos se acercan.
        //
        // La virola se ve, y es una sola banda entre los otros dos.
        assert!(virola >= 4, "la virola ocupa {virola} filas");

        let bloques = franjas
            .windows(2)
            .filter(|par| par[0].0 != par[1].0)
            .count()
            + 1;

        assert_eq!(
            bloques, 3,
            "el pincel cruza {bloques} zonas de material y deberian ser tres"
        );
    }

    #[test]
    fn el_abanico_es_mas_ancho_que_alto_como_un_pincel_plano() {
        // Lo que separa un pincel de pintor de una escoba: la cabeza es
        // **ancha y corta**, no un cono largo. Con una sección circular el
        // abanico sale tan alto como ancho y se lee como una gema o una
        // pala; con una sección aplanada se lee como lo que es.
        //
        // Se mide sobre el cuadro a `800 x 600`, que es lo que se presenta.
        const W: usize = 800;
        const H: usize = 600;

        let camara = Camera::new(
            Vec3::new(0.0, 12.0, 26.0),
            Vec3::zeros(),
            Vec3::zeros(),
            Vec3::new(0.0, 1.0, 0.0),
            DEFAULT_VERTICAL_FOV,
        );
        let mut framebuffer = Framebuffer::new(W, H);

        assert!(draw_brush_gizmo(
            &mut framebuffer,
            &camara,
            &Vec3::zeros(),
            &Vec3::new(0.0, 1.0, 0.0),
            0.30,
            0x0000C8FF,
            Some(&madera_separable()),
        ));

        let franjas = franjas(&framebuffer, W, H);
        let cabeza: Vec<usize> = franjas
            .iter()
            .filter(|(zona, _)| *zona == Zona::Cerdas)
            .map(|(_, ancho)| *ancho)
            .collect();

        assert!(!cabeza.is_empty(), "no se reconocio el abanico");

        let alto = cabeza.len();
        let ancho = cabeza.iter().copied().max().unwrap_or(0);

        assert!(
            ancho * 10 >= alto * 16,
            "el abanico mide {ancho} px de ancho por {alto} de alto: sigue siendo un cono"
        );
    }

    #[test]
    fn la_cabeza_guarda_proporcion_con_la_virola() {
        // El rechazo de la version anterior, dicho como medida: una cabeza
        // cuatro o cinco veces mas ancha que la virola no es un pincel
        // plano, es una pala, y deja la virola como una linea perdida entre
        // las cerdas y el mango.
        //
        // La proporcion de un pincel de pintor esta entre `1.4` y `2.0`: lo
        // bastante ancha para que la punta se lea, lo bastante contenida
        // para que la virola siga siendo una pieza y no un borde.
        const W: usize = 800;
        const H: usize = 600;

        let camara = Camera::new(
            Vec3::new(0.0, 12.0, 26.0),
            Vec3::zeros(),
            Vec3::zeros(),
            Vec3::new(0.0, 1.0, 0.0),
            DEFAULT_VERTICAL_FOV,
        );
        let mut framebuffer = Framebuffer::new(W, H);

        assert!(draw_brush_gizmo(
            &mut framebuffer,
            &camara,
            &Vec3::zeros(),
            &Vec3::new(0.0, 1.0, 0.0),
            0.30,
            0x0000C8FF,
            Some(&madera_separable()),
        ));

        let franjas = franjas(&framebuffer, W, H);
        let ancho_de = |zona: Zona| {
            franjas
                .iter()
                .filter(|(z, _)| *z == zona)
                .map(|(_, ancho)| *ancho)
                .max()
                .unwrap_or(0)
        };

        let cabeza = ancho_de(Zona::Cerdas);
        let virola = ancho_de(Zona::Virola);

        assert!(
            cabeza > 0 && virola > 0,
            "no se reconocieron las dos piezas"
        );

        // `1.4 <= cabeza / virola <= 2.0`, en aritmetica entera.
        assert!(
            cabeza * 10 >= virola * 14,
            "cabeza {cabeza} px y virola {virola} px: la punta no se lee"
        );
        assert!(
            cabeza * 10 <= virola * 20,
            "cabeza {cabeza} px contra virola {virola} px: es una pala, no un pincel"
        );
    }

    #[test]
    fn la_punta_es_compacta_y_no_acaba_en_un_filo_recto() {
        // El rechazo de la version anterior: la cabeza seguia leyendose como
        // una cuna solida. Dos cosas la delatan y las dos se miden aqui.
        //
        // La primera es la proporcion: una punta que casi dobla a la virola
        // es un triangulo, no unas cerdas. Se exige `<= 1.5`.
        //
        // La segunda es el borde. Un poliedro proyectado acaba en una arista
        // limpia, y eso es lo que hace que parezca tallado en madera en vez
        // de un manojo de pelo. Se mide el contorno inferior de la zona de
        // cerdas y se exige que **cambie de direccion** varias veces: un
        // filo recto, aunque este inclinado, avanza siempre hacia el mismo
        // lado.
        const W: usize = 800;
        const H: usize = 600;

        let camara = Camera::new(
            Vec3::new(0.0, 12.0, 26.0),
            Vec3::zeros(),
            Vec3::zeros(),
            Vec3::new(0.0, 1.0, 0.0),
            DEFAULT_VERTICAL_FOV,
        );
        let mut framebuffer = Framebuffer::new(W, H);

        assert!(draw_brush_gizmo(
            &mut framebuffer,
            &camara,
            &Vec3::zeros(),
            &Vec3::new(0.0, 1.0, 0.0),
            0.30,
            0x0000C8FF,
            Some(&madera_separable()),
        ));

        let franjas = franjas(&framebuffer, W, H);
        let ancho_de = |zona: Zona| {
            franjas
                .iter()
                .filter(|(z, _)| *z == zona)
                .map(|(_, ancho)| *ancho)
                .max()
                .unwrap_or(0)
        };

        let cabeza = ancho_de(Zona::Cerdas);
        let virola = ancho_de(Zona::Virola);

        assert!(
            cabeza > 0 && virola > 0,
            "no se reconocieron las dos piezas"
        );
        assert!(
            cabeza * 10 <= virola * 15,
            "punta {cabeza} px contra virola {virola} px: sigue siendo una cuna"
        );

        // Contorno inferior de las cerdas: por cada columna, la fila mas
        // baja con cerdas.
        let mut contorno: Vec<isize> = Vec::new();

        for x in 0..W {
            let mut fondo = None;

            for y in 0..H {
                if zona_de(framebuffer.buffer[y * W + x]) == Some(Zona::Cerdas) {
                    fondo = Some(y as isize);
                }
            }

            if let Some(y) = fondo {
                contorno.push(y);
            }
        }

        assert!(
            contorno.len() >= 8,
            "la punta solo abarca {} columnas",
            contorno.len()
        );

        let quiebros = contorno
            .windows(2)
            .map(|par| (par[1] - par[0]).signum())
            .filter(|paso| *paso != 0)
            .collect::<Vec<_>>()
            .windows(2)
            .filter(|par| par[0] != par[1])
            .count();

        // Dos, y no mas, porque dos es el techo de esta geometria: con seis
        // lados y la seccion aplanada solo hay **cuatro** abscisas distintas
        // a lo ancho, luego tres aristas en el contorno y como mucho dos
        // vertices donde pueda quebrarse. Pedir tres obligaria a anadir
        // facetas, que es justo lo que este rediseno no debe hacer.
        //
        // Un filo recto da cero: la version anterior daba exactamente eso.
        assert!(
            quiebros >= 2,
            "el borde de la punta cambia de direccion {quiebros} veces: es un filo recto"
        );
    }

    #[test]
    fn la_cabeza_son_varios_mechones_separados_y_no_un_bloque() {
        // La propuesta compuesta: la cabeza deja de ser un manojo tubular y
        // pasa a ser un puñado de mechones planos que nacen dentro de la
        // virola y se separan hacia la punta.
        //
        // La medida es directa: en alguna fila de la zona de cerdas tienen
        // que verse **cuatro tramos o mas**, con hueco entre ellos. Un
        // manojo macizo da exactamente uno, por ancho que sea.
        const W: usize = 800;
        const H: usize = 600;

        let camara = Camera::new(
            Vec3::new(0.0, 12.0, 26.0),
            Vec3::zeros(),
            Vec3::zeros(),
            Vec3::new(0.0, 1.0, 0.0),
            DEFAULT_VERTICAL_FOV,
        );
        let mut framebuffer = Framebuffer::new(W, H);

        assert!(draw_brush_gizmo(
            &mut framebuffer,
            &camara,
            &Vec3::zeros(),
            &Vec3::new(0.0, 1.0, 0.0),
            0.30,
            0x0000C8FF,
            Some(&madera_separable()),
        ));

        // Segmentos contiguos de cerdas en cada fila.
        let mut maximo = 0usize;

        for y in 0..H {
            let mut segmentos = 0usize;
            let mut dentro = false;

            for x in 0..W {
                let es_cerda = zona_de(framebuffer.buffer[y * W + x]) == Some(Zona::Cerdas);

                if es_cerda && !dentro {
                    segmentos += 1;
                }

                dentro = es_cerda;
            }

            maximo = maximo.max(segmentos);
        }

        assert!(
            maximo >= 4,
            "la fila mas dividida muestra {maximo} mechones: la cabeza sigue siendo un bloque"
        );
    }

    #[test]
    fn la_cabeza_es_un_manojo_denso_y_no_un_tenedor() {
        // El rechazo de la version anterior: cinco mechones anchos y muy
        // separados se leian como dientes. Lo que se pide es un manojo.
        //
        // Dos medidas sobre la misma fila, la mas dividida de la cabeza:
        //
        // - **Densidad de pelo**: seis mechones o mas. Con cinco se cuentan
        //   las piezas de un vistazo, que es justo lo que delata al tenedor.
        // - **Huecos contenidos**: ningun hueco puede ser mas ancho que el
        //   mechon mas ancho de esa fila. Separar mas devuelve los dientes,
        //   por muchos que sean.
        const W: usize = 800;
        const H: usize = 600;

        let camara = Camera::new(
            Vec3::new(0.0, 12.0, 26.0),
            Vec3::zeros(),
            Vec3::zeros(),
            Vec3::new(0.0, 1.0, 0.0),
            DEFAULT_VERTICAL_FOV,
        );
        let mut framebuffer = Framebuffer::new(W, H);

        assert!(draw_brush_gizmo(
            &mut framebuffer,
            &camara,
            &Vec3::zeros(),
            &Vec3::new(0.0, 1.0, 0.0),
            0.30,
            0x0000C8FF,
            Some(&madera_separable()),
        ));

        // Tramos de pelo y de hueco de cada fila, en orden.
        let tramos_de = |y: usize| -> (Vec<usize>, Vec<usize>) {
            let (mut pelo, mut huecos) = (Vec::new(), Vec::new());
            let (mut corriendo, mut dentro) = (0usize, false);
            let mut empezado = false;

            for x in 0..W {
                let es_cerda = zona_de(framebuffer.buffer[y * W + x]) == Some(Zona::Cerdas);

                if es_cerda {
                    empezado = true;
                }
                if !empezado {
                    continue;
                }

                if es_cerda == dentro {
                    corriendo += 1;
                } else {
                    if corriendo > 0 {
                        if dentro {
                            pelo.push(corriendo);
                        } else {
                            huecos.push(corriendo);
                        }
                    }
                    corriendo = 1;
                    dentro = es_cerda;
                }
            }

            if corriendo > 0 && dentro {
                pelo.push(corriendo);
            }

            (pelo, huecos)
        };

        // La fila mas dividida, desempatando por **cuanto pelo** tiene.
        //
        // Varias filas empatan a siete mechones, y entre ellas hay una
        // diferencia que importa: las mas bajas ya perdieron alguna punta
        // afilada y su aire es residuo, no separacion. Quedarse con la de
        // mayor cobertura elige la banda donde el manojo se ve entero, que
        // es la que describe la densidad. Sin el desempate, la eleccion
        // dependia del orden de recorrido.
        let mejor = (0..H)
            .max_by_key(|y| {
                let (pelo, _) = tramos_de(*y);

                (pelo.len(), pelo.iter().sum::<usize>())
            })
            .unwrap_or(0);
        let (pelo, huecos) = tramos_de(mejor);

        assert!(
            pelo.len() >= 6,
            "la fila mas poblada muestra {} mechones: falta densidad",
            pelo.len()
        );

        let hueco_mayor = huecos.iter().copied().max().unwrap_or(0);
        let pelo_mayor = pelo.iter().copied().max().unwrap_or(0);

        assert!(
            hueco_mayor <= pelo_mayor,
            "hueco de {hueco_mayor} px contra mechon de {pelo_mayor} px: son dientes"
        );

        // Y la separacion exacta: **un** pixel.
        //
        // Ni dos ni mas, porque a esta escala dos pixeles de aire entre
        // puntas ya se leen como las puas de un peine. Ni cero, porque un
        // hueco que no llega al pixel se fusiona en cuanto cambia el angulo
        // o la resolucion, y la cabeza vuelve a ser un bloque.
        assert!(
            hueco_mayor <= 1,
            "el hueco mayor entre puntas mide {hueco_mayor} px: parece un peine"
        );
        assert!(
            hueco_mayor >= 1,
            "no hay ningun hueco de un pixel entre las puntas: se fusionaron"
        );
    }
}
