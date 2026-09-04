//! Mide `interactive_frame_time`: el **peor cuadro de la transición**, que
//! es el número del que sale su duración.
//!
//! ```text
//! cargo run --release --example interactive_frame_time
//! ```
//!
//! # Por qué hace falta un probe propio
//!
//! De esta cifra depende `reveal_duration`, y de ella los quince cuadros de
//! la transición. Los tests prueban la aritmética **alrededor** del número;
//! ninguno prueba que el número siga siendo cierto.
//!
//! `interactive_probe` no sirve para esto: usa `InteriorVisible` en vez del
//! preset refractivo, mide una sola pasada en vez de una distribución, y no
//! separa los estados de revelación.
//!
//! # La corrección de la Tarea 7.1
//!
//! La versión anterior medía dos estados —`reveal 0.0` y `reveal 1.0`— y
//! tomaba el peor de los dos. El peor cuadro de la transición **no era
//! ninguno de los dos**, y la transición es exactamente lo que los quince
//! cuadros protegen.
//!
//! Dos mecanismos mueven el coste, y solo uno de ellos estaba medido:
//!
//! 1. **Los techos ópticos.** El material de lienzo no tiene techos, así
//!    que donde queda lienzo no se lanza un solo rayo secundario. En cuanto
//!    `t` despega, `kr` y `kt` pasan `SECONDARY_THRESHOLD` y el árbol de
//!    rayos completo ya está activo. Esto sí estaba medido: es la diferencia
//!    entre `reveal 0.0` y `reveal 1.0`, y es la grande —el lienzo cuesta
//!    dos terceras partes de lo demás—.
//! 2. **El doble muestreo.** `reveal::resolve` corta en `t <= 0` y en
//!    `t >= 1` con un único muestreo de textura; en cualquier `t` intermedio
//!    muestrea **las dos** y las mezcla. Esto no estaba medido en absoluto,
//!    porque los dos estados que se medían son justo los dos que lo evitan.
//!
//! El resultado del barrido es una **meseta**: cualquier estado con todo
//! revelado cuesta entre `1.3x` y `1.6x` el lienzo, y dentro de la meseta el
//! doble muestreo añade poco —del orden del `10 %` sobre el estado pintado,
//! con una dispersión de máquina del mismo orden—. La corrección importa
//! menos por la cifra que por dónde queda el margen: el cuadro que hay que
//! garantizar es intermedio, no final.
//!
//! Y una sorpresa que el barrido de las regiones no habría dado nunca: el
//! estado más caro es el del grupo **`Finale`**, no el de las regiones.
//! `Finale` son las diez masas del Monolito **y las diez del arco costero**,
//! que juntas ocupan casi todo el cuadro de la toma hero; mientras se barren
//! las regiones, ese grupo sigue en lienzo y se lleva su parte del cuadro
//! sin pagar un rayo secundario.
//!
//! # Qué se mide
//!
//! - `400 x 300`, el perfil interactivo.
//! - `RefractiveWater`, el preset canónico.
//! - Assets cargados, o aborta: sin texturas el doble muestreo no existe y
//!   la medición perdería justo lo que busca.
//! - Un barrido de la revelación con **las tres regiones a la vez**, que es
//!   el estado más caro que las regiones pueden alcanzar —tres teclas
//!   seguidas—, con el grupo `Finale` todavía en lienzo.
//! - Otro barrido del grupo `Finale` sobre las tres regiones ya pintadas.
//!   No es un añadido menor: `Finale` son las diez masas del Monolito **y
//!   las diez del arco costero**, que juntas ocupan casi todo el cuadro.
//! - El estado pintado completo, que es el que la calibración usaba antes.
//!
//! # Dos decisiones de método
//!
//! **Intercalado por repetición, no por bloque.** Los puntos no se miden
//! uno hasta agotarlo y luego el siguiente; se traza **un** cuadro de cada
//! punto por ronda. En esta máquina el estado térmico se mueve dentro de
//! una sola corrida: una primera versión que agotaba siete renders por
//! punto antes de pasar al siguiente daba diferencias entre puntos vecinos
//! de hasta un `19 %` que no reproducían, porque le cargaba el
//! calentamiento al punto que tocara. Con el intercalado por ronda, la
//! deriva le toca a todos por igual y la diferencia entre puntos vuelve a
//! ser atribuible al estado de revelación.
//!
//! **El punto peor se elige por el mínimo, no por la mediana.** El mínimo
//! es una estimación del suelo de coste; la mediana mezcla el coste con la
//! interferencia. El presupuesto que se reporta, en cambio, sale de la
//! **mediana** del peor punto: para comparar estados conviene el suelo, y
//! para dimensionar conviene el número conservador.
//!
//! Ni uno ni otro reproducen bien en esta máquina: entre corridas el suelo
//! del lienzo osciló entre `0.035` y `0.041 s`, un `16 %`. Por eso las
//! constantes del código **no** se eligen persiguiendo el máximo de una
//! corrida —ver `RevealState::worst_case`—: el barrido sirve para confirmar
//! la banda y para avisar si algún punto se sale de ella, no para nombrar un
//! ganador con cuatro decimales.

use std::path::PathBuf;
use std::time::Instant;

use expedition33_continente_inacabado::framebuffer::Framebuffer;
use expedition33_continente_inacabado::light::diorama as luces_del_diorama;
use expedition33_continente_inacabado::renderer::{render, InteractiveProfile, Shading};
use expedition33_continente_inacabado::reveal::{
    reveal_duration, RevealState, MINIMUM_REVEAL_FRAMES, REVEAL_DURATION_CEILING,
    WORST_CASE_PROGRESS,
};
use expedition33_continente_inacabado::scene::RevealGroup;
use expedition33_continente_inacabado::scene_builder::Blockout;
use expedition33_continente_inacabado::scenes::{safe_level_con, WaterPreset};

/// Cuadros por punto. Cada uno se traza en una ronda distinta, alternando
/// con los demás puntos. Ver la nota de método.
const MUESTRAS: usize = 30;

/// Puntos del barrido, con las tres regiones en el mismo progreso.
///
/// Los extremos siguen midiéndose: son la referencia contra la que se lee
/// cuánto cuesta el estado intermedio, y sin ellos la corrección no se
/// puede comprobar. `0.02` está para ver dónde despega el coste, que no es
/// gradual: los rayos secundarios se encienden en cuanto `t` deja de ser
/// cero.
const BARRIDO: [f32; 9] = [0.0, 0.02, 0.1, 0.25, 0.5, 0.75, 0.9, 0.98, 1.0];

/// Cuánto puede superar un punto al de calibración antes de que valga la
/// pena mirarlo.
///
/// No es una cifra de gusto: dentro de la meseta **el orden no reproduce**.
/// Dos corridas seguidas de este mismo ejemplo pusieron el máximo en
/// `finale 0.98` y en `finale 0.75`, con mínimos que se movieron un `3 %`
/// mientras el suelo del lienzo se movía un `16 %` entre corridas. Un aviso
/// que se disparara con esas diferencias solo enseñaría a ignorarlo.
const TOLERANCIA_MESETA: f64 = 0.05;

/// El mismo barrido para el grupo `Finale`, sobre las tres regiones ya
/// pintadas.
///
/// Hace falta un barrido propio porque `Finale` **no es solo el Monolito**:
/// son sus diez masas y las diez del arco costero, que juntas ocupan casi
/// todo el cuadro de la toma hero. Los extremos se omiten porque ya están
/// medidos con otro nombre: `finale 0.0` es `regiones 1.00` y `finale 1.0`
/// es el estado pintado completo.
const BARRIDO_FINALE: [f32; 7] = [0.02, 0.1, 0.25, 0.5, 0.75, 0.9, 0.98];

fn nivel_texturizado() -> Blockout {
    let raiz = PathBuf::from(".");

    match safe_level_con(WaterPreset::RefractiveWater, Some(&raiz)) {
        Ok(nivel) => nivel,
        Err(e) => {
            eprintln!("error: {e}");
            eprintln!("  esta medicion es la que fija la duracion de la revelacion");
            eprintln!("  y sin texturas mediria otra escena.");
            eprintln!("  generalos con: cargo run --release --bin generate_assets");
            std::process::exit(1);
        }
    }
}

/// Las tres regiones en el mismo progreso, el Monolito sin pintar.
///
/// El Monolito se queda en cero porque **no puede estar de otra forma**
/// mientras las regiones no llegan a uno: `activate` lo prohíbe y `advance`
/// solo lo arranca al completarse la tercera.
fn regiones_en(t: f32) -> RevealState {
    let mut estado = RevealState::unpainted();

    for grupo in RevealGroup::ALL {
        if grupo != RevealGroup::Finale {
            estado.set_progress(grupo, t);
        }
    }

    estado
}

/// El Monolito a medio revelar sobre las tres regiones ya pintadas.
fn finale_en(t: f32) -> RevealState {
    let mut estado = regiones_en(1.0);
    estado.set_progress(RevealGroup::Finale, t);

    estado
}

/// Un punto del barrido con su distribución de tiempos.
struct Punto {
    etiqueta: String,
    estado: RevealState,
    tiempos: Vec<f64>,
}

impl Punto {
    fn nuevo(etiqueta: String, estado: RevealState) -> Self {
        Punto {
            etiqueta,
            estado,
            tiempos: Vec::with_capacity(MUESTRAS),
        }
    }

    /// Mínimo, mediana y máximo.
    fn distribucion(&self) -> (f64, f64, f64) {
        let mut t = self.tiempos.clone();
        t.sort_by(|a, b| a.partial_cmp(b).expect("no hay NaN"));

        (t[0], t[t.len() / 2], t[t.len() - 1])
    }
}

fn main() {
    let diorama = nivel_texturizado();
    let luces = luces_del_diorama(&diorama.anchors, &diorama.scale);
    let camara = diorama.hero_camera();
    let perfil = InteractiveProfile::MEDIA;

    let mut puntos: Vec<Punto> = BARRIDO
        .iter()
        .map(|&t| Punto::nuevo(format!("regiones {t:.2}"), regiones_en(t)))
        .collect();
    puntos.extend(
        BARRIDO_FINALE
            .iter()
            .map(|&t| Punto::nuevo(format!("finale {t:.2}"), finale_en(t))),
    );
    puntos.push(Punto::nuevo(
        "todo pintado".to_string(),
        RevealState::painted(),
    ));

    println!("interactive_frame_time · el peor cuadro de la transicion\n");
    println!("  perfil      {} x {}", perfil.width, perfil.height);
    println!("  preset      safe-refractive-water");
    println!(
        "  escena      {} primitivas, {} texturas, {} luces",
        diorama.scene.objects.len(),
        diorama.scene.textures.len(),
        luces.len()
    );
    println!("  camara      toma hero");
    println!("  muestras    {MUESTRAS} por punto, intercaladas por ronda");
    println!("  comando     cargo run --release --example interactive_frame_time");

    let mut framebuffer = Framebuffer::new(perfil.width, perfil.height);

    for _ in 0..MUESTRAS {
        for punto in &mut puntos {
            let inicio = Instant::now();
            render(
                &mut framebuffer,
                &diorama.scene,
                &diorama.accel,
                &luces,
                &punto.estado,
                &camara,
                Shading::Material,
            );
            punto.tiempos.push(inicio.elapsed().as_secs_f64());
        }
    }

    let lienzo = puntos[0].distribucion();

    println!("\n  regiones t   = las tres regiones en t, grupo Finale en lienzo");
    println!("  finale t     = Finale en t sobre las tres regiones pintadas");
    println!("  todo pintado = los cuatro grupos en 1.0");
    println!(
        "\n  {:<16} {:>9} {:>9} {:>9} {:>11}",
        "estado", "minimo", "mediana", "maximo", "min/lienzo"
    );

    // El peor por **mínimo**, que es el estadístico que reproduce.
    let mut peor = 0usize;

    for (i, punto) in puntos.iter().enumerate() {
        let (minimo, mediana, maximo) = punto.distribucion();

        if minimo > puntos[peor].distribucion().0 {
            peor = i;
        }

        println!(
            "  {:<16} {minimo:>9.4} {mediana:>9.4} {maximo:>9.4} {:>10.2}x",
            punto.etiqueta,
            minimo / lienzo.0
        );
    }

    let (peor_min, peor_mediana, _) = puntos[peor].distribucion();

    // El estado pintado completo es el ultimo punto, y es la referencia que
    // importa: es lo que la calibracion medía antes de la Tarea 7.1.
    let pintado = puntos[puntos.len() - 1].distribucion();

    // Las bandas, que es lo que el barrido resuelve de verdad. Dentro de
    // cada una el progreso apenas mueve el coste; entre ellas hay un
    // escalón, y el escalón es el hallazgo.
    let banda = |prefijo: &str, salvo: &str| {
        let mins = puntos
            .iter()
            .filter(|p| p.etiqueta.starts_with(prefijo) && !p.etiqueta.ends_with(salvo))
            .map(|p| p.distribucion().0);

        (
            mins.clone().fold(f64::INFINITY, f64::min),
            mins.fold(0.0_f64, f64::max),
        )
    };

    // `regiones 0.00` es el lienzo y va aparte, no dentro de su banda.
    let (reg_baja, reg_alta) = banda("regiones", "0.00");
    let (fin_baja, fin_alta) = banda("finale", "nada");

    println!("\n  peor estado             {}", puntos[peor].etiqueta);
    println!("  lienzo                  {:.4} s   (minimo)", lienzo.0);
    println!(
        "  regiones reveladas      {reg_baja:.4} .. {reg_alta:.4} s   ({:.2}x .. {:.2}x)",
        reg_baja / lienzo.0,
        reg_alta / lienzo.0
    );
    println!(
        "  finale revelado         {fin_baja:.4} .. {fin_alta:.4} s   ({:.2}x .. {:.2}x)",
        fin_baja / lienzo.0,
        fin_alta / lienzo.0
    );
    println!(
        "  todo pintado            {:.4} s   ({:.2}x)",
        pintado.0,
        pintado.0 / lienzo.0
    );
    println!("  peor                    {peor_min:.4} s");
    println!(
        "  el peor cuesta          {:.2}x el lienzo y {:.2}x el pintado completo",
        peor_min / lienzo.0,
        peor_min / pintado.0
    );

    // La constante de calibración se comprueba contra la medición, no al
    // revés. Se compara por mínimos porque es como se eligió el peor punto.
    let calibrado = puntos
        .iter()
        .find(|p| p.estado == RevealState::worst_case())
        .expect("worst_case tiene que ser uno de los puntos del barrido")
        .distribucion();

    println!(
        "\n  calibracion de la ventana  RevealState::worst_case(), finale {WORST_CASE_PROGRESS:.2}"
    );

    if peor_min > calibrado.0 * (1.0 + TOLERANCIA_MESETA) {
        println!("  AVISO: hay un punto que se sale de la meseta por arriba.");
        println!(
            "  {:.4} s en {} contra {:.4} s en la constante.",
            peor_min, puntos[peor].etiqueta, calibrado.0
        );
        println!("  Revisar si aparecio un coste nuevo, y solo entonces mover la constante.");
    } else {
        println!(
            "  al dia: el punto mas caro le saca {:.1} % y la tolerancia son {:.0} %.",
            100.0 * (peor_min / calibrado.0 - 1.0),
            100.0 * TOLERANCIA_MESETA
        );
    }

    // Para dimensionar se usa la **mediana** del peor punto: el mínimo es
    // bueno para comparar estados y malo para prometer un tiempo.
    let medido = peor_mediana as f32;

    println!("\n  interactive_frame_time = {medido:.4} s   (mediana del peor estado)");

    match reveal_duration(medido) {
        Ok(duracion) => {
            let critico = REVEAL_DURATION_CEILING / MINIMUM_REVEAL_FRAMES;

            println!("  reveal_duration        = {duracion:.2} s");
            println!("  cuadros de transicion  = {:.0}", duracion / medido);
            println!(
                "  margen al critico      = {:.1}x   (critico {critico:.4} s)",
                critico / medido
            );
        }
        Err(fallo) => {
            println!(
                "  FALLA el gate de fluidez: quince cuadros exigen {:.2} s y el techo son {:.2} s",
                fallo.required, REVEAL_DURATION_CEILING
            );
            println!("  baja la resolucion del perfil en vez de alargar la animacion");
            std::process::exit(1);
        }
    }

    println!("\n  Registrar junto a la cifra: commit, fecha, hardware y toolchain.");
    println!("  Un tiempo solo significa algo junto a los que se midieron con el.");
}
