//! Estadística de las mediciones de tiempo.
//!
//! Vive en la librería y no en cada ejemplo por una razón concreta: la
//! primera versión de la matriz de la Tarea 7.1 calculó la «mediana» de
//! treinta muestras como `ordenadas[n / 2]`, que con `n` par no es la
//! mediana sino el mayor de los dos valores centrales. El sesgo es pequeño y
//! sistemático —siempre hacia arriba—, que es justo la clase de error que no
//! se ve en una tabla y sobrevive a la revisión.
//!
//! Duplicar tres líneas de aritmética en dos ejemplos es cómo se consigue
//! que dos tablas de la misma evidencia no cuadren. Aquí hay una definición,
//! y tests que la fijan.
//!
//! # Por qué hay un cociente emparejado
//!
//! Para comparar dos estados no basta con dividir sus medianas. Las dos
//! medianas se miden en la misma máquina, y esa máquina baja de frecuencia
//! mientras se mide: la deriva entra en las dos y el cociente hereda su
//! ruido. `median_ratio` divide **muestra contra muestra de la misma ronda**
//! y saca la mediana de los cocientes, que es una comparación pareada: lo
//! que le pase a la máquina en una ronda le pasa al numerador y al
//! denominador a la vez.

/// Mínimo, mediana y máximo de una distribución de tiempos.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Distribution {
    pub min: f64,
    pub median: f64,
    pub max: f64,
}

/// Resumen de una muestra de tiempos.
///
/// # Pánico
///
/// Con la muestra vacía. No hay resumen que devolver y un `Option` obligaría
/// a que cada llamador decidiera qué hacer con un caso que solo ocurre si el
/// bucle de medición no corrió.
pub fn summarize(samples: &[f64]) -> Distribution {
    assert!(!samples.is_empty(), "no hay muestras que resumir");

    let mut ordenadas = samples.to_vec();
    ordenadas.sort_by(|a, b| a.partial_cmp(b).expect("no hay NaN en un tiempo medido"));

    Distribution {
        min: ordenadas[0],
        median: median_of_sorted(&ordenadas),
        max: ordenadas[ordenadas.len() - 1],
    }
}

/// Mediana de una muestra **ya ordenada**.
///
/// Con `n` par es el promedio de los dos valores centrales, que es la
/// definición. Tomar el de arriba, que es lo que hacía la primera versión de
/// la matriz, sesga cada celda hacia el lado caro.
fn median_of_sorted(ordenadas: &[f64]) -> f64 {
    let n = ordenadas.len();

    if n % 2 == 1 {
        ordenadas[n / 2]
    } else {
        0.5 * (ordenadas[n / 2 - 1] + ordenadas[n / 2])
    }
}

/// Mediana de los cocientes muestra a muestra de `numerador` y
/// `denominador`.
///
/// Es una comparación **pareada**: el elemento `i` de las dos muestras tiene
/// que venir de la misma ronda de medición, y así la deriva térmica de esa
/// ronda se cancela en el cociente. Comparar `mediana(a) / mediana(b)` no lo
/// consigue, porque las dos medianas pueden estar dominadas por rondas
/// distintas.
///
/// # Pánico
///
/// Si las muestras no tienen el mismo tamaño: eso significaría que no están
/// emparejadas y el resultado no sería lo que dice el nombre.
pub fn median_ratio(numerador: &[f64], denominador: &[f64]) -> f64 {
    assert_eq!(
        numerador.len(),
        denominador.len(),
        "un cociente pareado exige el mismo numero de rondas en las dos muestras"
    );
    assert!(!numerador.is_empty(), "no hay muestras que comparar");

    let mut cocientes: Vec<f64> = numerador
        .iter()
        .zip(denominador)
        .map(|(a, b)| a / b)
        .collect();

    cocientes.sort_by(|a, b| a.partial_cmp(b).expect("no hay NaN en un cociente medido"));

    median_of_sorted(&cocientes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn la_mediana_impar_es_el_valor_central() {
        let d = summarize(&[3.0, 1.0, 2.0]);

        assert_eq!(d.min, 1.0);
        assert_eq!(d.median, 2.0);
        assert_eq!(d.max, 3.0);
    }

    #[test]
    fn la_mediana_par_promedia_los_dos_centrales() {
        // El bug que motivó este módulo: con `n` par, `ordenadas[n / 2]`
        // devuelve `3.0`, el mayor de los dos centrales, y no `2.5`.
        let d = summarize(&[1.0, 2.0, 3.0, 4.0]);

        assert_eq!(d.median, 2.5, "con n par la mediana promedia");
        assert_ne!(d.median, 3.0, "y no es el de arriba");
    }

    #[test]
    fn el_sesgo_de_la_mediana_par_va_siempre_hacia_arriba() {
        // No es un empate afortunado: para cualquier muestra par con los dos
        // centrales distintos, el índice `n / 2` es el mayor de los dos. Por
        // eso el error sobreestimaba sistemáticamente cada celda.
        for n in [2usize, 4, 6, 30] {
            let muestra: Vec<f64> = (0..n).map(|i| i as f64).collect();
            let d = summarize(&muestra);
            let de_arriba = muestra[n / 2];

            assert!(
                d.median < de_arriba,
                "con n = {n} la mediana correcta tiene que quedar por debajo del indice n/2"
            );
        }
    }

    #[test]
    fn una_sola_muestra_es_su_propio_resumen() {
        let d = summarize(&[0.05]);

        assert_eq!(d.min, 0.05);
        assert_eq!(d.median, 0.05);
        assert_eq!(d.max, 0.05);
    }

    #[test]
    fn el_cociente_pareado_cancela_una_deriva_comun() {
        // Dos estados: el segundo cuesta exactamente el doble que el
        // primero. La máquina se va frenando ronda a ronda, y la deriva
        // multiplica a los dos por igual.
        let deriva = [1.0, 1.4, 2.3, 0.8, 1.9];
        let barato: Vec<f64> = deriva.iter().map(|d| 0.01 * d).collect();
        let caro: Vec<f64> = deriva.iter().map(|d| 0.02 * d).collect();

        let pareado = median_ratio(&caro, &barato);

        assert!(
            (pareado - 2.0).abs() < 1e-12,
            "el cociente pareado tiene que dar exactamente 2, dio {pareado}"
        );
    }

    #[test]
    fn un_tiron_en_un_solo_estado_mueve_el_cociente_de_medianas() {
        // Cinco rondas. El estado caro cuesta un `25 %` más que el barato,
        // salvo en la primera ronda, donde sufre un tirón que lo triplica.
        //
        // El cociente de medianas cambia **de qué ronda** sale cada mediana
        // y se va a `1.33`. El pareado deja el tirón donde estaba —un
        // cociente de `3.0` entre cinco— y sigue diciendo `1.25`, que es lo
        // que cuesta el estado caro cuando la máquina no se atraganta.
        let barato = [0.010, 0.011, 0.012, 0.013, 0.014];
        let caro = [0.030, 0.014, 0.015, 0.016, 0.017];

        let de_medianas = summarize(&caro).median / summarize(&barato).median;
        let pareado = median_ratio(&caro, &barato);

        assert!((pareado - 1.25).abs() < 1e-12, "pareado dio {pareado}");
        assert!(
            (de_medianas - 4.0 / 3.0).abs() < 1e-12,
            "de medianas dio {de_medianas}"
        );
        assert!(
            de_medianas > pareado * 1.06,
            "el ejemplo tiene que separar los dos estimadores"
        );
    }

    #[test]
    #[should_panic(expected = "mismo numero de rondas")]
    fn un_cociente_sin_emparejar_no_se_calcula() {
        median_ratio(&[1.0, 2.0], &[1.0]);
    }
}
