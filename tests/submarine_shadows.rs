//! Validación de sombras submarinas — Tarea 5.6.
//!
//! Configuración controlada: agua presente, barco dentro, **`L-01` fuera** y
//! solo `L-02`, que está enlazada a Aguas Voladoras por light linking.
//!
//! Los cuatro criterios del plan, cada uno con su test:
//!
//! 1. El barco no está negro.
//! 2. El agua no bloquea sombras.
//! 3. Las rocas opacas sí producen sombra.
//! 4. El Monolito conserva su sombra de contacto.
//!
//! Vive como test y no solo como render de evidencia porque un criterio
//! cualitativo mirado a ojo no protege de una regresión. El render está en
//! `examples/submarine_shadows.rs`.

use expedition33_continente_inacabado::accel::{SceneAccel, TraversalStats};
use expedition33_continente_inacabado::color::Color;
use expedition33_continente_inacabado::light::{diorama as luces_del_diorama, PointLight};
use expedition33_continente_inacabado::ray::Ray;
use expedition33_continente_inacabado::renderer::{cast_ray, Shading};
use expedition33_continente_inacabado::reveal::RevealState;
use expedition33_continente_inacabado::scene::{Scene, SpatialGroupId};
use expedition33_continente_inacabado::scene_builder::Blockout;
use expedition33_continente_inacabado::scenes::flying_waters::{ancla_del_casco, caja_del_volumen};
use expedition33_continente_inacabado::scenes::{anclas_del_diorama, safe_level, WaterPreset};
use expedition33_continente_inacabado::EPSILON;
use nalgebra_glm::Vec3;

/// El nivel seguro con el volumen refractivo y su plataforma de luces.
fn nivel() -> (Blockout, Vec<PointLight>) {
    let diorama = safe_level(WaterPreset::RefractiveWater);
    let luces = luces_del_diorama(&diorama.anchors, &diorama.scale);

    (diorama, luces)
}

/// La luz del rig con ese identificador.
fn luz<'a>(luces: &'a [PointLight], id: &str) -> &'a PointLight {
    luces
        .iter()
        .find(|l| l.id == id)
        .unwrap_or_else(|| panic!("el rig deberia tener {id}"))
}

/// Configuración controlada: `L-01` fuera, solo `L-02`.
fn solo_l02(luces: &[PointLight]) -> Vec<PointLight> {
    vec![*luz(luces, "L-02")]
}

/// ¿Hay algo entre el punto y la luz?
///
/// Reproduce lo que hace el renderer: separa el origen de la superficie y
/// corta la búsqueda antes de la luz, para que un objeto **detrás** de ella
/// no proyecte una sombra que no existe. Respeta el light linking de
/// oclusores, que es lo que confina `L-02` a Aguas Voladoras.
fn en_sombra(scene: &Scene, accel: &SceneAccel, desde: Vec3, luz: &PointLight) -> bool {
    let hacia = luz.position - desde;
    let distancia = hacia.magnitude();
    let direccion = hacia / distancia;

    accel.occluded(
        scene,
        &Ray::new(desde + direccion * EPSILON, direccion),
        distancia - EPSILON,
        luz.occluder_groups,
        &mut TraversalStats::default(),
    )
}

fn brillo(color: Color) -> f32 {
    color.r + color.g + color.b
}

/// Punto sobre la cubierta central del barco.
///
/// La sección de cubierta es la del medio, a `+0.35` del centro del casco:
/// el mástil arranca a `-0.6`.
fn punto_de_la_cubierta() -> Vec3 {
    ancla_del_casco(anclas_del_diorama().flying_waters_anchor) + Vec3::new(0.35, 0.41, 0.0)
}

// --------------------------------------------------------- criterio 1

/// Índices de las doce piezas del casco, localizadas en la escena.
///
/// Ni desplazamientos copiados ni semillas: son los objetos de Aguas
/// Voladoras que caben dentro del volumen en planta y quedan **por encima
/// del lecho y del kelp**. El mástil, sus soportes, la cadena y el ancla se
/// filtran por esbeltos en planta.
fn piezas_del_casco(diorama: &Blockout) -> Vec<usize> {
    let (centro, tamano) = caja_del_volumen(anclas_del_diorama().flying_waters_anchor);
    let (minimo, maximo) = (centro - tamano * 0.5, centro + tamano * 0.5);

    diorama
        .scene
        .objects
        .iter()
        .enumerate()
        .filter(|(_, o)| o.spatial_group == SpatialGroupId::FlyingWaters)
        .filter(|(_, o)| {
            let caja = o.primitive.bounds();

            caja.min.x > minimo.x
                && caja.max.x < maximo.x
                && caja.min.z > minimo.z
                && caja.max.z < maximo.z
                && caja.min.y > minimo.y + 1.2
                && (caja.max.z - caja.min.z) > 0.5
        })
        .map(|(i, _)| i)
        .collect()
}

/// Rejilla de rayos verticales sobre la huella del casco, disparados desde
/// **justo por debajo de la superficie del agua**.
///
/// Ese origen no es casual. Dos alternativas que se probaron y no sirven:
///
/// - Desde encima del agua, el primer impacto es la superficie y lo que
///   vuelve es el color del casco ya pesado por `kt` y mezclado con el
///   reflejo del cielo. Mide la imagen final, no el sombreado del casco.
/// - Desde la cara superior de cada pieza, el rayo puede **nacer dentro** de
///   la pieza de encima: el casco está apilado —cuerpo, cubierta, costillas—
///   y ahí el cuboide devuelve su cara de salida, que mira hacia abajo y no
///   ve ninguna luz. Se mide una cara interna y sale negra con razón.
///
/// Arrancar bajo la superficie garantiza que lo primero que se toque sea una
/// cara **expuesta**, la misma que ve la cámara.
fn muestras_sobre_el_casco(diorama: &Blockout) -> Vec<(Vec3, usize)> {
    let base = anclas_del_diorama().flying_waters_anchor;
    let (centro, tamano) = caja_del_volumen(base);
    let superficie = centro.y + tamano.y * 0.5;

    let piezas = piezas_del_casco(diorama);
    let huella = piezas
        .iter()
        .map(|i| diorama.scene.objects[*i].primitive.bounds())
        .reduce(|a, b| a.union(&b))
        .expect("el casco tiene piezas");

    let mut muestras = Vec::new();

    for i in 0..24 {
        for j in 0..12 {
            let x = huella.min.x + (i as f32 + 0.5) / 24.0 * (huella.max.x - huella.min.x);
            let z = huella.min.z + (j as f32 + 0.5) / 12.0 * (huella.max.z - huella.min.z);

            let rayo = Ray::new(
                Vec3::new(x, superficie - 0.01, z),
                Vec3::new(0.0, -1.0, 0.0),
            );
            let Some(impacto) =
                diorama
                    .accel
                    .intersect(&diorama.scene, &rayo, &mut TraversalStats::default())
            else {
                continue;
            };

            if piezas.contains(&impacto.object_index) {
                muestras.push((rayo.origin, impacto.object_index));
            }
        }
    }

    muestras
}

#[test]
fn el_barco_no_esta_negro_con_solo_l02() {
    let (diorama, luces) = nivel();
    let confinada = solo_l02(&luces);
    let muestras = muestras_sobre_el_casco(&diorama);

    assert!(
        muestras.len() > 40,
        "solo {} rayos dieron en el casco",
        muestras.len()
    );

    // Se mide cara por cara y no en un punto: el pecio **se hace sombra a sí
    // mismo** —las tres costillas expuestas por la brecha tapan trozos de
    // cubierta—, así que un solo punto puede caer en penumbra y no dice nada
    // del barco. Eso salió midiendo, no suponiendo: con `L-01` sola, el
    // punto central de la cubierta queda tras la costilla de índice 117.
    let mut iluminadas = 0;
    let mut minimo_absoluto = f32::MAX;
    let mut suma_con_l02 = 0.0;
    let mut suma_ambiente = 0.0;

    for (origen, _) in &muestras {
        let rayo = Ray::new(*origen, Vec3::new(0.0, -1.0, 0.0));
        let trazar = |luces: &[PointLight]| {
            cast_ray(
                &rayo,
                &diorama.scene,
                &diorama.accel,
                luces,
                &RevealState::painted(),
                Shading::Material,
                &mut TraversalStats::default(),
            )
        };

        let a_oscuras = trazar(&[]);
        let con_l02 = trazar(&confinada);

        assert!(
            brillo(con_l02) >= brillo(a_oscuras) - 1e-6,
            "L-02 restó luz en {origen:?}: {con_l02} contra {a_oscuras}"
        );

        minimo_absoluto = minimo_absoluto.min(brillo(con_l02));
        suma_con_l02 += brillo(con_l02);
        suma_ambiente += brillo(a_oscuras);

        if brillo(con_l02) > brillo(a_oscuras) * 3.0 {
            iluminadas += 1;

            // Lo que llega es la luz fria de `L-02`, y eso **no** se puede
            // comprobar sobre el color final: la madera del casco es marron
            // y absorbe azul, asi que ni siquiera una luz azul pura la
            // vuelve azul. Lo que si es comprobable es la temperatura del
            // **aporte**, comparada con la del ambiente sobre la misma
            // superficie: si la luz fuera la calida de `L-01` la razon no
            // subiria.
            let aporte = Color::new(
                con_l02.r - a_oscuras.r,
                con_l02.g - a_oscuras.g,
                con_l02.b - a_oscuras.b,
            );

            assert!(
                aporte.r > 1e-6 && a_oscuras.r > 1e-6,
                "no hay con que comparar en {origen:?}"
            );
            assert!(
                aporte.b / aporte.r > (a_oscuras.b / a_oscuras.r) * 2.0,
                "el aporte en {origen:?} no es mas frio que el ambiente:                  azul/rojo {:.3} contra {:.3}",
                aporte.b / aporte.r,
                a_oscuras.b / a_oscuras.r
            );
        }
    }

    // Ninguna cara queda en cero. La mas apagada se apoya en el suelo de
    // ambiente —medido: `0.0084`, que es exactamente `albedo x AMBIENT`
    // sobre la madera del casco—, y esta ahi porque una costilla la tapa.
    // Que exista ese suelo es la razon de que `AMBIENT` no sea cero: sin
    // el, lo que una costilla tapa se veria negro absoluto y el pecio
    // perderia la silueta interior.
    assert!(
        minimo_absoluto > 0.005,
        "una cara del casco quedó en negro: {minimo_absoluto}"
    );

    // Y el barco en conjunto está claramente iluminado, no apenas por
    // encima del ambiente.
    assert!(
        suma_con_l02 > suma_ambiente * 3.0,
        "el barco en conjunto apenas supera el ambiente: {suma_con_l02} contra {suma_ambiente}"
    );

    // Y la mayoría está iluminada. No todas: lo que queda tras una costilla
    // está en su sombra, y es lo que hace que las costillas se lean.
    assert!(
        iluminadas * 2 > muestras.len(),
        "solo {iluminadas} de {} caras reciben L-02",
        muestras.len()
    );
}

// --------------------------------------------------------- criterio 2

#[test]
fn el_agua_no_bloquea_las_sombras_aunque_este_en_medio() {
    let (diorama, luces) = nivel();
    let l02 = luz(&luces, "L-02");
    let cubierta = punto_de_la_cubierta();

    // Primero, que el volumen **este** de verdad en medio: la superficie
    // del agua queda entre la cubierta y la luz. Sin esta comprobacion el
    // test pasaria por vacio si alguien moviera la luz bajo el agua.
    let (centro, tamano) = caja_del_volumen(anclas_del_diorama().flying_waters_anchor);
    let superficie = centro.y + tamano.y * 0.5;

    assert!(
        cubierta.y < superficie && superficie < l02.position.y,
        "la superficie del agua no esta entre la cubierta y L-02"
    );

    // Y aun asi la cubierta no esta en sombra: `A-01` lleva
    // `ShadowMode::Ignore` por decision del inventario.
    assert!(
        !en_sombra(&diorama.scene, &diorama.accel, cubierta, l02),
        "el volumen de agua proyecto sombra sobre el barco"
    );
}

// --------------------------------------------------------- criterio 3

/// Las seis rocas submarinas, localizadas por geometría y no por semilla.
///
/// Son las únicas primitivas de Aguas Voladoras que caben **enteras**
/// dentro del volumen de agua: el lecho es más ancho que él, el borde roto
/// lo atraviesa, y el barco asoma por arriba.
fn rocas_submarinas(diorama: &Blockout) -> Vec<(Vec3, f32)> {
    let (centro, tamano) = caja_del_volumen(anclas_del_diorama().flying_waters_anchor);
    let (minimo, maximo) = (centro - tamano * 0.5, centro + tamano * 0.5);
    let casco = ancla_del_casco(anclas_del_diorama().flying_waters_anchor);

    diorama
        .scene
        .objects
        .iter()
        .filter(|o| o.spatial_group == SpatialGroupId::FlyingWaters)
        .map(|o| o.primitive.bounds())
        .filter(|caja| {
            caja.min.x > minimo.x
                && caja.min.y > minimo.y
                && caja.min.z > minimo.z
                && caja.max.x < maximo.x
                && caja.max.y < maximo.y
                && caja.max.z < maximo.z
        })
        // Fuera el barco, la cadena y el ancla: quedan las rocas y el kelp.
        // Las rocas se apoyan en el lecho y son anchas; el kelp es una
        // fronda de 0.14 de lado y la cadena eslabones de 0.13.
        .filter(|caja| caja.max.x - caja.min.x > 0.30)
        // Y las que estan bajo el casco no sirven de oclusor limpio.
        .filter(|caja| (caja.min.x - casco.x).abs() > 1.2)
        .map(|caja| {
            let centro = (caja.min + caja.max) * 0.5;
            let radio = (caja.max - caja.min).magnitude() * 0.5;

            (centro, radio)
        })
        .collect()
}

#[test]
fn las_rocas_opacas_si_producen_sombra() {
    let (diorama, luces) = nivel();
    let l02 = luz(&luces, "L-02");
    let rocas = rocas_submarinas(&diorama);

    assert!(
        !rocas.is_empty(),
        "no se localizo ninguna roca submarina utilizable"
    );

    for (centro, radio) in &rocas {
        // Un punto inmediatamente **detras** de la roca, del lado opuesto a
        // la luz. Construirlo asi y no «sobre el lecho, bajo la roca»
        // garantiza que el segmento hacia la luz atraviese la roca: a esta
        // distancia la linea hacia `L-02` sube en diagonal, y un punto justo
        // debajo podria salirse por el costado.
        let hacia_luz = (l02.position - centro).normalize();
        let detras = centro - hacia_luz * (radio + 0.02);

        assert!(
            en_sombra(&diorama.scene, &diorama.accel, detras, l02),
            "una roca no proyecto sombra: centro {centro:?}, radio {radio}"
        );
    }
}

#[test]
fn el_lecho_tiene_sombras_y_tambien_luz() {
    // La comprobacion complementaria: que haya sombra no puede significar
    // que la bahia entera este en penumbra. Se muestrea el lecho en una
    // rejilla y se exige que la mezcla no sea uniforme en ninguno de los
    // dos sentidos.
    let (diorama, luces) = nivel();
    let l02 = luz(&luces, "L-02");
    let (centro, tamano) = caja_del_volumen(anclas_del_diorama().flying_waters_anchor);

    let lecho_y = centro.y - tamano.y * 0.5 + 0.36;
    let mut en_penumbra = 0;
    let mut total = 0;

    for i in 0..9 {
        for j in 0..9 {
            let x = centro.x + tamano.x * (i as f32 / 8.0 - 0.5) * 0.85;
            let z = centro.z + tamano.z * (j as f32 / 8.0 - 0.5) * 0.85;
            let punto = Vec3::new(x, lecho_y, z);

            total += 1;
            if en_sombra(&diorama.scene, &diorama.accel, punto, l02) {
                en_penumbra += 1;
            }
        }
    }

    assert!(
        en_penumbra > 0,
        "el lecho no tiene ni una sombra: {en_penumbra} de {total}"
    );
    assert!(
        en_penumbra < total,
        "el lecho esta enteramente en sombra: {en_penumbra} de {total}"
    );
}

// --------------------------------------------------------- criterio 4

#[test]
fn el_monolito_conserva_su_sombra_de_contacto() {
    // `L-01` está fuera de la configuración controlada de esta tarea, pero
    // la sombra de contacto del Monolito es un criterio suyo: es lo que
    // apoya el Monolito sobre el plinto en vez de dejarlo flotando. Se
    // valida contra `L-01`, que es la única luz con sombras que lo alcanza.
    let (diorama, luces) = nivel();
    let l01 = luz(&luces, "L-01");

    // Huella del Monolito, medida de la escena.
    let huella = diorama
        .scene
        .objects
        .iter()
        .filter(|o| o.spatial_group == SpatialGroupId::Monolith)
        .map(|o| o.primitive.bounds())
        .reduce(|a, b| a.union(&b))
        .expect("el Monolito tiene geometria");

    let (minimo, maximo) = (huella.min, huella.max);

    // Un punto al pie del Monolito, del lado contrario a la luz y a ras del
    // plinto. Lo que el rayo de sombra encuentre primero tiene que ser el
    // Monolito: eso es lo que hace la sombra de **contacto**, y no cualquier
    // sombra de otra masa del diorama.
    let base = Vec3::new(
        (minimo.x + maximo.x) * 0.5,
        0.0,
        (minimo.z + maximo.z) * 0.5,
    );
    let hacia_luz = l01.position - base;
    let horizontal = Vec3::new(hacia_luz.x, 0.0, hacia_luz.z).normalize();
    let media_huella = Vec3::new(maximo.x - minimo.x, 0.0, maximo.z - minimo.z).magnitude() * 0.5;

    let al_pie = base - horizontal * (media_huella + 0.05) + Vec3::new(0.0, 0.01, 0.0);

    assert!(
        en_sombra(&diorama.scene, &diorama.accel, al_pie, l01),
        "el Monolito no proyecta sombra de contacto en {al_pie:?}"
    );

    // Y quien la proyecta es el Monolito, no otra cosa.
    let direccion = (l01.position - al_pie).normalize();
    let rayo = Ray::new(al_pie + direccion * EPSILON, direccion);
    let impacto = diorama
        .accel
        .intersect(&diorama.scene, &rayo, &mut TraversalStats::default())
        .expect("algo tiene que interceptar el rayo de sombra");

    assert_eq!(
        diorama.scene.objects[impacto.object_index].spatial_group,
        SpatialGroupId::Monolith,
        "la sombra al pie del Monolito la proyecta otra masa"
    );
}
