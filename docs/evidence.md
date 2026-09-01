# Evidencia — El Continente Inacabado

Registro de verificaciones y mediciones del proyecto. Cada entrada dice **qué** se midió, **cómo** y **cuándo**. Nada se anota aquí por estimación: si un número no fue medido, la fila queda vacía y marcada como pendiente.

---

## Hito 0 — Base académica verificada

**Fecha:** 31 de agosto de 2026

| Dato | Valor |
|---|---|
| Repositorio de trabajo | `SrCharlied/expedioram-24531` |
| Rama de trabajo | `master` (no hay rama de proyecto separada) |
| Repositorio académico | `menene/cc2018-2026-02-10` |
| Rama base | `15-RT-03-ORBIT-CAMERA` |
| Commit base | `f3e553917077deba3529d9a97f39ea2b58341e84` |
| **Árbol `src/` verificado** | **`d77aad46c439f43ed5f06c2fd393bc25fa5bdf11`** |

**Método.** Comparación de contenido, no de hashes de commit: el historial local fue aplanado en `06a2b43 Init`, así que los commits no coinciden con los del curso y compararlos no diría nada.

**Resultado, medido antes de ejecutar la Tarea `0.4`:** los diez archivos de la base eran byte-idénticos entre el repositorio de trabajo y `f3e5539` — los seis de `src/`, más `Cargo.toml`, `Cargo.lock`, `README.md` y `.gitignore`. El árbol `src/` compartía hash en ambos lados.

Ese resultado es un **registro histórico con fecha**, no una propiedad permanente del repositorio. La Tarea `0.4` lo alteró deliberadamente:

| Archivo | ¿Sigue idéntico a `f3e5539`? | Por qué |
|---|---|---|
| `src/` (seis archivos) | Sí, hasta que empiece el Hito 1 | intacto |
| `README.md` | Sí, hasta la Tarea `8.4` | intacto |
| `.gitignore` | Sí | intacto |
| `Cargo.toml` | **No** | renombre del paquete en la Tarea `0.4` |
| `Cargo.lock` | **No** | regenerado por `cargo check` tras el renombre |

Por eso la *verificación extendida* de la Tarea `0.3` ya no sale vacía, y no debe interpretarse como una regresión. La comprobación que sigue siendo válida es la del árbol `src/`.

**Re-verificación posterior al Hito 1.** El Hito 1 modifica `src/`, así que la comprobación sobre `HEAD` dejará de dar `IDENTICO`. Eso es esperado. Como `src/` no cambió en ningún commit del Hito 0, el árbol `d77aad46…` identifica la base sin depender de ninguna rama ni commit concreto:

```bash
git rev-parse <cualquier-commit-del-hito-0>:src
# debe devolver d77aad46c439f43ed5f06c2fd393bc25fa5bdf11
```

**Nota sobre `upstream`.** El remoto académico es configuración local en `.git/config` y **no viaja con el repositorio**. Cada clon nuevo debe registrarlo antes de poder reproducir la verificación.

### Toolchain de la verificación

| Dato | Valor |
|---|---|
| `cargo` | 1.97.0 (`c980f4866`, 2026-06-30) |
| `rustc` | 1.97.0 (`2d8144b78`, 2026-07-07) |
| Sistema | Windows 11 |

`cargo check` compila limpio tras el renombre del paquete a `expedition33_continente_inacabado`.

---

## Contratos visuales — incompletos

`docs/design/` **no está completa**, y queda registrado aquí como exige la Tarea `0.4`.

| Fuente de verdad | Archivo | Estado |
|---:|---|---|
| 3 | `Inventario_v6_Continente_Inacabado.md` | Presente |
| 4 | `Expedition33_Blueprint_v2_2.svg` | **Falta** |
| 5 | `Decisiones_Blueprint_v2_Expedition33.md` | **Falta** |

Hasta que se incorporen, cualquier discrepancia de composición se resuelve contra el inventario, que es la fuente de verdad inmediatamente superior disponible.

---

## Pendientes de medición

Ninguna de estas filas puede completarse por estimación. Cada hito llena la suya.

| Hito | Qué se mide | Estado |
|---|---|---|
| 2 | `scene_radius` y `monolith_height` medidos en el blockout | Pendiente |
| 2 | `orbit_radius` derivado por bisección, con `framing_margin` usado | Pendiente |
| 3 | Benchmark `safe-interior-visible` (159 primitivas) — mín/mediana/máx | Pendiente |
| 3 | Benchmark `safe-opaque-water` (160 primitivas) — control de oclusión | Pendiente |
| 3 | `interactive_frame_time` del perfil interactivo | Pendiente |
| 5 | Calibración de `L-02`: `distance_boat`, `range`, `intensity` | Pendiente |
| 6 | `reveal_duration` derivada de `interactive_frame_time` | Pendiente |
| 7 | Matriz de rendimiento por preset | Pendiente |
| 8 | Hardware de medición y tiempos finales en release | Pendiente |

**Regla.** Todos los benchmarks se ejecutan en release. El perfil `dev` de este proyecto lleva `opt-level = 3` heredado de la base académica, así que un tiempo medido en debug **parece** comparable a release y no lo es.
