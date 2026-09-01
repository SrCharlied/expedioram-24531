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

## Contratos visuales — completos

`docs/design/` contiene las tres fuentes de verdad de diseño.

| Fuente de verdad | Archivo | Estado |
|---:|---|---|
| 3 | `Inventario_v6_Continente_Inacabado.md` | Presente |
| 4 | `Expedition33_Blueprint_v2_2.svg` | Presente — 2400 × 1540 |
| 5 | `Decisiones_Blueprint_v2_Expedition33.md` | Presente |

**Discrepancia menor detectada.** La bitácora declara documentar `Expedition33_Blueprint_v2.svg`, mientras que el archivo versionado es `Expedition33_Blueprint_v2_2.svg`. Ambos miden 2400 × 1540. Queda anotado por trazabilidad; no se corrigió la bitácora porque es un registro histórico y su edición corresponde a quien la escribió.

---

## Preflight del Hito 1

**Fecha:** 31 de agosto de 2026  
**Commit:** `847d0e3dc0cf1ee10f5b5e585093aa9abfcb0953`

**Base heredada:** árbol `d77aad46c439f43ed5f06c2fd393bc25fa5bdf11`

La base académica presentaba:

- Dos diferencias de `cargo fmt --check`, en `src/camera.rs:49` y `src/main.rs:68`.
- Un warning `clippy::wrong_self_convention` en `Color::to_hex`, que toma `&self` siendo `Color` un tipo `Copy`.

Ambos bloqueaban los gates que el plan exige desde el Hito 1, donde `clippy` corre con `-D warnings`. Se corrigieron en un commit aislado antes de modificar arquitectura o comportamiento.

Cambios exactos, sin efecto sobre el comportamiento:

| Archivo | Cambio |
|---|---|
| `src/camera.rs` | rustfmt parte la línea larga de `radius_xz` |
| `src/main.rs` | rustfmt une la llamada a `set_current_color` |
| `src/color.rs` | `Color::to_hex` toma `self` por valor |

Comandos ejecutados, los cuatro en verde tras el commit:

- `cargo fmt -- --check` — sin diferencias
- `cargo clippy --all-targets -- -D warnings` — sin warnings
- `cargo test` — 0 tests, compila
- `cargo check` — compila

**Árbol `src/` tras la normalización:** `3fe8a4a8091bfc13582c791ba91c96c5fd95ae60`

Desde este commit, `src/` diverge de `f3e5539` **por formato, no por lógica**. Es la frontera entre la base del curso y el código del proyecto: para comparar contra la base se usa cualquier commit anterior a este, cuyo árbol `src/` sigue siendo `d77aad46…`.

**Por qué el ancla de registro es el árbol y no el commit.** Este apartado citaba originalmente el commit `8287bb8…`; una reescritura posterior del historial lo dejó huérfano y esa referencia quedó rota. Los hashes de árbol no dependen de la identidad del commit —se derivan del contenido—, así que `d77aad46…` y `3fe8a4a…` sobrevivieron sin cambio. Cuando haya que señalar un estado del código, citar el árbol; el commit es una comodidad que puede caducar.

---

## Hito 1 — Núcleo matemático testeable

**Fecha:** 1 de septiembre de 2026 (el plan asignaba hasta el 4 de septiembre)  
**Árbol `src/` al cierre:** `2aa3202aebdbedd31466a3298ae4e11530b5d7db`

Un commit por tarea, en orden:

| Tarea | Commit | Qué introdujo |
|---|---|---|
| 1.1 | `3b84522` | `lib.rs` y `renderer.rs`; `main.rs` queda con ventana, input y presentación |
| 1.2 | `46f0769` | `Color` en `f32` lineal, con el recorte una sola vez en `to_hex` |
| 1.3 | `c757df9` | `Ray`, `Hit` con `front_face` y `object_index`, `EPSILON` canónico |
| 1.4 | `4d720a9` | `Aabb` y el slab test, con el eje paralelo tratado aparte |
| 1.5 | `190108b` | `Cuboid` con normal y UV por cara |
| 1.6 | `707451c` | `Primitive`, `Scene`, `SceneObject`; se elimina `sphere.rs` |

**Gate del hito, los cuatro en verde:**

- `cargo fmt -- --check` — sin diferencias
- `cargo clippy --all-targets -- -D warnings` — sin warnings
- `cargo test` — 43 tests en 4 targets (38 unitarios, 5 de integración)
- `cargo build --release` — compila

El render headless del gate vive en `tests/render_smoke.rs`: renderiza `32 × 24` sin abrir ventana y comprueba que haya píxeles de cubo y de fondo, que el material se resuelva por `object_index` y que ningún color salga NaN. La comprobación de NaN se hace sobre `Color` y no sobre el framebuffer, porque al empacar a `u32` un NaN ya se habría convertido en un entero cualquiera.

**Sin medir.** El Hito 1 no produce números de rendimiento y no debe intentar producirlos: no hay luces, ni escena de tamaño real, ni aceleración. La primera medición legítima es la del Hito 3.

**Verificación visual pendiente.** `cargo run` abre ventana y no se puede automatizar aquí. Debe mostrar un cuboide coloreado por normales, con cada cara de un color distinto y estable al orbitar.

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
