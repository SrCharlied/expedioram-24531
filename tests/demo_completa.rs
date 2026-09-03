//! Gate del Hito 6: la demo completa, de lienzo a Monolito final.
//!
//! El criterio del plan es «sin recompilar», y eso es una afirmación sobre
//! el **estado en tiempo de ejecución**: todo el recorrido tiene que salir
//! de mover escalares y leer entrada, sin que nada dependa de una constante
//! elegida al compilar.
//!
//! Aquí se recorre entero por la API pública —la misma que usa la
//! ventana—, simulando el reloj. Lo que no se puede probar así es que el
//! ratón apunte donde el usuario cree, y para eso está la revisión visual;
//! todo lo demás sí.

use expedition33_continente_inacabado::accel::TraversalStats;
use expedition33_continente_inacabado::input::{demo_action, pick_region, DemoAction};
use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
use expedition33_continente_inacabado::renderer::{cast_ray, Shading};
use expedition33_continente_inacabado::reveal::{
    reveal_duration, reveal_speed, RevealPhase, RevealState, MINIMUM_REVEAL_FRAMES,
};
use expedition33_continente_inacabado::scene::RevealGroup;
use expedition33_continente_inacabado::scenes::{safe_level, WaterPreset};

const ANCHO: usize = 800;
const ALTO: usize = 600;

/// Tiempo por cuadro del perfil interactivo, medido en release.
const FRAME_TIME: f32 = 0.0490;

/// Aplica una tecla de la demo sobre el estado, como hace la ventana.
fn pulsar(tecla: char, reveal: &mut RevealState) {
    match demo_action(tecla) {
        Some(DemoAction::Paint(grupo)) => {
            reveal.activate(grupo);
        }
        Some(DemoAction::ResetCanvas) => *reveal = RevealState::unpainted(),
        // El reset de cámara no toca la revelación.
        Some(DemoAction::ResetCamera) | None => {}
    }
}

/// Avanza el reloj hasta que nada quede revelándose, contando cuadros.
///
/// Devuelve `(cuadros, segundos)`. Corta a los mil cuadros para que un
/// avance roto termine el test en vez de colgarlo.
fn dejar_correr(reveal: &mut RevealState, velocidad: f32) -> (usize, f32) {
    let mut cuadros = 0;
    let mut segundos = 0.0;

    while cuadros < 1000 {
        if !reveal.advance(FRAME_TIME, velocidad) {
            break;
        }

        cuadros += 1;
        segundos += FRAME_TIME;
    }

    (cuadros, segundos)
}

/// Avanza el reloj hasta que **ese grupo** quede pintado.
///
/// Hace falta aparte de `dejar_correr` por una razón que se ve midiendo: al
/// terminar la tercera región arranca el finale, así que «hasta que nada
/// quede revelándose» tarda dos duraciones y no una. Para cronometrar una
/// región hay que mirar esa región.
fn dejar_correr_hasta(
    reveal: &mut RevealState,
    velocidad: f32,
    grupo: RevealGroup,
) -> (usize, f32) {
    let mut cuadros = 0;
    let mut segundos = 0.0;

    while cuadros < 1000 && reveal.phase(grupo) == RevealPhase::Revealing {
        reveal.advance(FRAME_TIME, velocidad);

        cuadros += 1;
        segundos += FRAME_TIME;
    }

    (cuadros, segundos)
}

#[test]
fn la_demo_completa_va_de_lienzo_a_monolito() {
    let diorama = safe_level(WaterPreset::RefractiveWater);
    let duracion = reveal_duration(FRAME_TIME).expect("el perfil cabe");
    let velocidad = reveal_speed(duracion);

    let mut reveal = RevealState::unpainted();

    // Punto de partida: todo en lienzo, nada revelándose.
    for grupo in RevealGroup::ALL {
        assert_eq!(reveal.phase(grupo), RevealPhase::Unpainted, "{grupo:?}");
    }

    // Las tres regiones, por teclado, una tras otra.
    for (tecla, grupo) in [
        ('1', RevealGroup::Meadows),
        ('2', RevealGroup::Breakwater),
        ('3', RevealGroup::FlyingWaters),
    ] {
        pulsar(tecla, &mut reveal);

        assert_eq!(
            reveal.phase(grupo),
            RevealPhase::Revealing,
            "{tecla} no arranco {grupo:?}"
        );

        let (cuadros, segundos) = dejar_correr_hasta(&mut reveal, velocidad, grupo);

        assert_eq!(reveal.phase(grupo), RevealPhase::Painted, "{grupo:?}");

        // El criterio de aceptacion del plan, region por region.
        assert!(
            cuadros as f32 >= MINIMUM_REVEAL_FRAMES,
            "{grupo:?} se revelo en {cuadros} cuadros"
        );
        assert!(
            (segundos - duracion).abs() < FRAME_TIME * 2.0,
            "{grupo:?} tardo {segundos} s y la duracion es {duracion} s"
        );
    }

    // Y el finale, que nadie eligió: arranca solo al completarse las tres.
    // Un tick lo activa y otra duración lo pinta.
    reveal.advance(FRAME_TIME, velocidad);
    assert_eq!(
        reveal.phase(RevealGroup::Finale),
        RevealPhase::Revealing,
        "el Monolito no arranco al terminar el Continente"
    );

    let (cuadros, _) = dejar_correr_hasta(&mut reveal, velocidad, RevealGroup::Finale);

    assert_eq!(reveal.phase(RevealGroup::Finale), RevealPhase::Painted);
    assert!(
        cuadros as f32 >= MINIMUM_REVEAL_FRAMES,
        "el Monolito se revelo en {cuadros} cuadros"
    );

    assert!(reveal.all_regions_painted());
    assert_eq!(
        reveal.global_progress(),
        1.0,
        "el cielo no termino de pintarse"
    );

    // El diorama entero se ve pintado: el material resuelto de cada objeto
    // revelable coincide con su material final.
    for objeto in &diorama.scene.objects {
        if !objeto.is_revealable() {
            continue;
        }

        let progreso = reveal.progress(objeto.reveal_group);

        assert_eq!(progreso, 1.0, "un objeto quedo a medio pintar: {progreso}");
    }
}

#[test]
fn la_demo_se_puede_repetir_sin_recompilar() {
    // Es lo que el gate exige de verdad: que el recorrido completo salga de
    // mover estado, y que se pueda dar dos veces seguidas.
    let duracion = reveal_duration(FRAME_TIME).expect("el perfil cabe");
    let velocidad = reveal_speed(duracion);
    let mut reveal = RevealState::unpainted();

    for pase in 1..=2 {
        for tecla in ['1', '2', '3'] {
            pulsar(tecla, &mut reveal);
            dejar_correr(&mut reveal, velocidad);
        }

        assert_eq!(
            reveal.phase(RevealGroup::Finale),
            RevealPhase::Painted,
            "el pase {pase} no llego al Monolito"
        );

        // `L` devuelve al lienzo y deja todo listo para repetir.
        pulsar('L', &mut reveal);

        for grupo in RevealGroup::ALL {
            assert_eq!(
                reveal.phase(grupo),
                RevealPhase::Unpainted,
                "el pase {pase} no limpio {grupo:?}"
            );
        }
    }
}

#[test]
fn las_tres_regiones_pueden_revelarse_a_la_vez() {
    // Un presentador apurado pulsa 1, 2 y 3 seguidas. Las tres avanzan en
    // paralelo y el finale espera a la ultima, sin que ninguna se pierda.
    let duracion = reveal_duration(FRAME_TIME).expect("el perfil cabe");
    let velocidad = reveal_speed(duracion);
    let mut reveal = RevealState::unpainted();

    for tecla in ['1', '2', '3'] {
        pulsar(tecla, &mut reveal);
    }

    for grupo in [
        RevealGroup::Meadows,
        RevealGroup::Breakwater,
        RevealGroup::FlyingWaters,
    ] {
        assert_eq!(reveal.phase(grupo), RevealPhase::Revealing, "{grupo:?}");
    }

    let (cuadros, segundos) = dejar_correr(&mut reveal, velocidad);

    // En paralelo, las tres mas el finale tardan **dos** duraciones: una
    // para las regiones y otra para el Monolito, que arranca despues.
    assert!(
        (segundos - duracion * 2.0).abs() < FRAME_TIME * 3.0,
        "tardo {segundos} s y se esperaban {} s",
        duracion * 2.0
    );
    assert!(cuadros as f32 >= MINIMUM_REVEAL_FRAMES * 2.0);

    for grupo in RevealGroup::ALL {
        assert_eq!(reveal.phase(grupo), RevealPhase::Painted, "{grupo:?}");
    }
}

#[test]
fn el_clic_llega_a_las_mismas_regiones_que_el_teclado() {
    // Sin esto, el gate podria pasar por teclado y fallar con raton.
    let diorama = safe_level(WaterPreset::RefractiveWater);
    let camara = diorama.hero_camera();

    // Se recorre el cuadro y se recogen las regiones que el raton alcanza.
    let mut alcanzadas = Vec::new();

    for y in (0..ALTO).step_by(7) {
        for x in (0..ANCHO).step_by(7) {
            let cursor = (x as f32 + 0.5, y as f32 + 0.5);

            if let Some(grupo) =
                pick_region(&diorama.scene, &diorama.accel, &camara, cursor, ANCHO, ALTO)
            {
                if !alcanzadas.contains(&grupo) {
                    alcanzadas.push(grupo);
                }
            }
        }
    }

    for grupo in [
        RevealGroup::Meadows,
        RevealGroup::Breakwater,
        RevealGroup::FlyingWaters,
    ] {
        assert!(
            alcanzadas.contains(&grupo),
            "el raton no alcanza {grupo:?} desde la toma hero"
        );
    }
}

#[test]
fn el_diorama_cambia_de_verdad_entre_lienzo_y_pintado() {
    // El gate es visual, y esto es lo mas cerca que se puede llegar sin
    // ventana: los mismos rayos dan colores distintos en los dos extremos,
    // y ninguno queda en negro.
    let diorama = safe_level(WaterPreset::RefractiveWater);
    let luces = luces_del_diorama(&diorama.anchors, &diorama.scale);
    let camara = diorama.hero_camera();

    // Solo las muestras que **dan en la geometria**. La escena de los tests
    // se construye sin assets, asi que su cielo es un color plano que no
    // depende del progreso: incluir el fondo mediria sobre todo pixeles que
    // no pueden cambiar, y el diorama ocupa una fraccion del cuadro.
    let muestras: Vec<_> = (0..ALTO)
        .step_by(23)
        .flat_map(|y| (0..ANCHO).step_by(23).map(move |x| (x, y)))
        .filter(|(x, y)| {
            let rayo = camara.ray_from_pixel(*x, *y, ANCHO, ALTO);

            diorama
                .accel
                .intersect(&diorama.scene, &rayo, &mut TraversalStats::default())
                .is_some()
        })
        .collect();

    assert!(
        muestras.len() > 50,
        "solo {} muestras dan en el diorama",
        muestras.len()
    );

    let trazar = |reveal: &RevealState| {
        muestras
            .iter()
            .map(|(x, y)| {
                let rayo = camara.ray_from_pixel(*x, *y, ANCHO, ALTO);

                cast_ray(
                    &rayo,
                    &diorama.scene,
                    &diorama.accel,
                    &luces,
                    reveal,
                    Shading::Material,
                    &mut TraversalStats::default(),
                )
            })
            .collect::<Vec<_>>()
    };

    let lienzo = trazar(&RevealState::unpainted());
    let pintado = trazar(&RevealState::painted());

    assert_eq!(lienzo.len(), pintado.len());

    let distintos = lienzo
        .iter()
        .zip(&pintado)
        .filter(|(a, b)| a.to_hex() != b.to_hex())
        .count();

    assert!(
        distintos * 2 > lienzo.len(),
        "solo {distintos} de {} muestras del diorama cambian entre lienzo y pintado",
        lienzo.len()
    );

    for (indice, color) in pintado.iter().enumerate() {
        assert!(
            color.to_hex() & 0x00FF_FFFF != 0,
            "la muestra {indice} quedo en negro absoluto"
        );
    }
}
