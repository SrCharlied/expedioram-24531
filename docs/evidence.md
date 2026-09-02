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

## Hito 2 — Cámara final y blockout

**Fecha:** 1 de septiembre de 2026 (el plan asignaba del 5 al 8 de septiembre)  
**Estado:** cerrado. Blockout 1 aprobado visualmente.

### Parámetros de escala del Blockout 1

Medidos sobre la geometría construida, no elegidos. El binario headless los
imprime en cada corrida, así que esta tabla se copia sin transcribir a mano.

| Parámetro | Valor | Origen |
|---|---:|---|
| `scene_radius` | `12.0586` | medido: mayor distancia de `orbit_center` a una esquina de objeto |
| `monolith_height` | `12.0000` | medido: cima real de las masas del Monolito |
| `water_surface_y` | `1.9000` | cara superior del volumen de agua |
| `orbit_radius` | `28.6000` | derivado por bisección |
| `view_pitch` | `31.94°` | derivado del ojo y de `look_at` |
| Primitivas del blockout | `23` | cuboides grises |

`orbit_radius / scene_radius = 2.372`. Con `monolith_height / scene_radius ≈ 1.0`,
el inventario predice `2.38` para ese caso: la derivación en código y el
documento coinciden. El `2.2 × scene_radius` constante que traía la versión
anterior del inventario **habría recortado esta escena**.

Constantes de entrada: `eye_elevation = 35°`, `half_vertical_fov = 30°`,
`framing_margin = 2°`, `look_at_height_fraction = 0.15`.

### Vistas de validación

En `evidence/blockout/`, todas a `640 × 480`:

| Archivo | Cámara |
|---|---|
| `hero.png` | yaw `90°` — encara el borde roto |
| `90.png` | yaw `0°` |
| `180.png` | yaw `180°` |
| `270.png` | yaw `270°` |
| `corte_lateral.png` | yaw `0°`, elevación `8°` |
| `hero_normales.png` | toma hero con sombreado por normales |

El corte usa elevación baja a propósito: a `35°` la escena se lee en planta
y la relación Praderas-sobre-Rompeolas queda aplastada.

### Checklist del Blockout 1

La primera versión de la composición **falló** y se rehízo. Estado final:

| Criterio | Resultado |
|---|---|
| El Monolito permanece eje visual | Cumple — ~4 600 px en los cuatro ángulos |
| El borde roto encara la toma hero | Cumple — recorta la lámina de agua |
| Praderas está arriba | Cumple — meseta a `y ≈ 5.6` |
| Rompeolas sostiene la meseta | Cumple — pilares bajo su borde frontal |
| Aguas tiene espacio para barco y lecho | Cumple — lecho hundido más volumen de `1.2` |
| Nada esencial sale del frame | Cumple |
| `look_at` no manda el Continente al tercio inferior | Cumple — ocupa del 16 % al 77 % del alto |

**Por qué falló la primera versión.** La composición se había construido
plana, con el Monolito en `6.6` sobre un plinto de `22 × 20`. El inventario
describe una composición **vertical** —Praderas es una meseta alta,
Rompeolas la sostiene, Aguas ocupa el nivel bajo al frente— y esa
estratificación es la que hace legible al Monolito como eje. Al rehacerla,
la franja vertical ocupada pasó del 38 % al 60 % del frame.

### Dos correcciones que salieron de la validación

**`measure_scene_radius` medía sobre el AABB global.** La esquina de la
envolvente combina el máximo de los tres ejes aunque ningún objeto los
alcance a la vez; en una escena ancha y alta esa esquina cae en aire vacío.

| | Antes | Después |
|---|---:|---:|
| `scene_radius` | `16.51` | `12.06` |
| `orbit_radius` | `38.05` | `28.60` |

Un 27 % de distancia de más por medir donde no había geometría. Ahora
itera las esquinas de cada objeto.

**El borde roto estaba asignado al material del agua.** Compartía gris con
el volumen y quedaba invisible. El inventario le da `wet_basalt`: es
terreno en primer plano, y su función declarada es ocluir parcialmente la
cara frontal del agua. Con gris propio y altura por encima de
`water_surface_y` cumple esa función en vez de fundirse con ella.

### Decisión de encuadre: `LOOK_AT_HEIGHT_FRACTION` se conserva en `0.15`

La esfera de encuadre está centrada en `orbit_center`, que es la **base**
del Monolito. Se extiende por tanto tan abajo del suelo como arriba llega
el Monolito, y esa mitad inferior está siempre vacía: el diorama ocupa
alrededor del 16 % del frame.

**Es conservador, y se acepta durante el Hito 2.** En las imágenes reales
el Monolito no se recorta, el plinto no cae al borde, la escena está
razonablemente centrada, el Continente ocupa una porción suficiente del
frame y queda margen para fragmentos, skybox y efectos finales.

Subir `look_at` desplazaría la escena hacia abajo. Equilibraría algo el
espacio inferior, pero reduciría la sensación de elevación del Monolito:
es una **preferencia de encuadre, no una corrección estructural**.

Más adelante puede existir una cámara hero ligeramente ajustada respecto de
la órbita matemática. Esa es la vía prevista si el encuadre se quiere
afinar; convertir `LOOK_AT_HEIGHT_FRACTION` en otra ronda de cirugía no
está justificado.

### Gate del hito

- `cargo fmt -- --check` — sin diferencias
- `cargo clippy --all-targets -- -D warnings` — sin warnings
- `cargo test` — 66 tests, de los cuales 16 de cámara y 6 de `scene_builder`
- Blockout 1 **aprobado visualmente** en los cuatro ángulos

Los seis tests de `scene_builder` incluyen el que reproduce los dos valores
de referencia del inventario (`2.25 × S` y `2.38 × S`), que es lo que impide
que el código y el documento se desincronicen en silencio.

**Sin medir.** El Hito 2 tampoco produce números de rendimiento. Los
tiempos que imprime el binario headless están rotulados «informativo»: una
sola pasada, sin repeticiones ni registro de hardware. La primera medición
formal sigue siendo la del Hito 3.

**Nota de estado.** Al escribir esta entrada, las Tareas 2.1 a 2.5 están en
el árbol de trabajo sin commitear. Cuando se commiteen, anotar aquí el
árbol `src/` resultante — el árbol, no el commit, por lo explicado en el
apartado del preflight.

---

## Hito 3 — Checkpoint de rendimiento (Tarea 3.8)

**Fecha:** 1 de septiembre de 2026  
**Perfil:** `release` (`opt-level = 3`, sin LTO)
**Primera medición formal del proyecto.** Todo lo anterior estaba rotulado «informativo».

### Hardware y toolchain

| Dato | Valor |
|---|---|
| CPU | AMD Ryzen 7 6800H with Radeon Graphics |
| Núcleos | 8 físicos, 16 lógicos |
| Reloj máximo | 3201 MHz |
| RAM | 15.2 GB |
| Sistema | Windows 11 |
| `rustc` | 1.97.0 (`2d8144b78`) |
| Ejecución | monohilo; `rayon` no está habilitado |

### Resultados a 800 × 600, cinco repeticiones

| | `safe-interior-visible` | `safe-opaque-water` |
|---|---:|---:|
| Primitivas trazables | **159** | **160** |
| Grupos / clusters | 7 / 10 | 7 / 10 |
| Luces | 3, dos con sombra | 3, dos con sombra |
| Rayos primarios | 480 000 | 480 000 |
| Rayos de sombra | 98 561 | 104 713 |
| Pruebas de primitiva | 7 100 123 | 7 468 505 |
| Pruebas de bounds | 2 206 422 | 2 193 441 |
| Pruebas de primitiva por rayo | 12.27 | 12.77 |
| Tiempo mínimo | 0.0947 s | 0.0977 s |
| **Tiempo mediana** | **0.0956 s** | **0.0981 s** |
| Tiempo máximo | 0.0959 s | 0.0982 s |

El gate de rendimiento es `safe-interior-visible`. `safe-opaque-water` es
control visual y comparación de oclusión, nunca criterio de aprobación.

### Lo que aporta la aceleración

| Preset | Rayos | Pruebas sin jerarquía | Con jerarquía | Reducción |
|---|---:|---:|---:|---:|
| `safe-interior-visible` | 578 561 | 91 991 199 | 7 100 123 | **92.28 %** |
| `safe-opaque-water` | 584 713 | 93 554 080 | 7 468 505 | **92.02 %** |

La cota sin aceleración del inventario para el nivel seguro —76,8 millones
de pruebas primarias— queda confirmada: `480 000 × 159 = 76.3` millones,
más las de los rayos de sombra. La jerarquía evita nueve de cada diez.

### Por qué el preset opaco no puede aprobar rendimiento

El inventario lo advierte y la medición lo confirma, aunque no por donde
podría esperarse.

El agua opaca **no** abarata el recorrido: hace *más* pruebas de primitiva
(7.47 contra 7.10 millones), porque añade un cuboide grande que muchos
rayos atraviesan. Lo que oculta no es coste de intersección sino
**visibilidad**: las 44 primitivas del interior —casco, mástil, cadena,
ancla, kelp y rocas— dejan de verse.

El riesgo real llega con la óptica del Hito 5. En cuanto el agua sea
transparente, esas 44 primitivas volverán a la imagen y cada rayo que entre
al volumen generará además reflexión y refracción. Un tiempo medido hoy con
agua opaca serviría de línea base para una escena que va a costar bastante
más. Por eso el preset canónico es el que ya mira dentro de la bahía.

### Perfil interactivo (entrada de la Tarea 3.9)

| Resolución | Mediana | FPS implícitos |
|---|---:|---:|
| 800 × 600 | 0.0956 s | 10.5 |
| 400 × 300 | 0.0242 s | 41.3 |
| 320 × 240 | 0.0157 s | 63.7 |

**La mitigación 3.9 se dispara.** A 800 × 600 el loop va a 10.5 fps: eso es
latencia perceptible al orbitar, y los Hitos 4 a 6 hay que poder probarlos
de forma interactiva. A 400 × 300 el mismo trabajo baja a 0.0242 s, unos 41
fps, con la resolución final reservada para el cuadro en reposo.

**No se fija un objetivo de fps aquí.** El plan lo prohíbe expresamente
antes de medir, y esta es la primera medición: los números de arriba son el
punto de partida, no una meta.

### Perfil aplicado (Tarea 3.9)

La mitigación se ejecutó de inmediato, como manda su disparador. El perfil
elegido es **`MEDIA`, 400 × 300**, con `BAJA` (320 × 240) en reserva.

| Perfil | Trazado | Escalado | Total | Frente al cuadro final | Píxeles distintos |
|---|---:|---:|---:|---:|---:|
| `MEDIA` 400 × 300 | 0.0244 s | 0.0008 s | **0.0252 s** | 3.8× más rápido | 4.6 % |
| `BAJA` 320 × 240 | 0.0156 s | 0.0008 s | 0.0164 s | 5.9× más rápido | 5.4 % |
| Final 800 × 600 | — | — | 0.0960 s | — | — |

Se elige `MEDIA` porque 3.8× ya saca el loop de la zona pegajosa —de 10.5 a
unos 40 fps— y pierde menos detalle. `BAJA` apenas gana 0.009 s más, y ese
margen conviene guardarlo para cuando entren texturas y óptica.

El escalado cuesta **0.0008 s**, menos del 4 % del cuadro interactivo: es
irrelevante frente al trazado, que es lo que se quería comprobar antes de
adoptarlo.

Se usa **vecino más cercano** y no interpolación: el diorama es de caras
planas y aristas duras, y suavizar emborronaría precisamente los bordes que
dan la lectura de volumen.

**Comportamiento.** Mientras la cámara se mueve —y, desde la Tarea 6.4,
mientras una región se revela— se traza en el perfil y se escala. Al soltar
los controles se produce un único cuadro a resolución completa. Con todo
quieto se reutiliza el framebuffer sin volver a trazar.

**Coste de calendario.** Por la decisión cerrada del plan, los días que
consuma esta mitigación salen de la reserva del 25 al 27 de septiembre, no
de la calidad ni de los tests. El freeze del 28 no se mueve.

### Consecuencia para la Tarea 6.3

Con `interactive_frame_time = 0.0242 s` a 400 × 300, quince cuadros de
transición piden `0.363 s`, muy por debajo del piso de `1.5 s`. La
`reveal_duration` quedaría en el piso y el gate de fluidez pasa con holgura.
Es un cálculo provisional: la escena crecerá con texturas, reflexión y
refracción, así que hay que volver a medirlo en el Hito 6 y no heredar este
valor.

### Reproducir

```bash
cargo run --release --bin render_scene -- \
  --preset safe-interior-visible --width 800 --height 600 \
  --benchmark 5 --output evidence/performance/safe-interior-visible.png

cargo run --release --bin render_scene -- \
  --preset safe-opaque-water --width 800 --height 600 \
  --benchmark 5 --output evidence/performance/safe-opaque-water.png
```

Las imágenes quedan en `evidence/performance/`. Los contadores de rayos y
de pruebas los emite el propio binario: no son estimaciones.

---

## Pendientes de medición

Ninguna de estas filas puede completarse por estimación. Cada hito llena la suya.

| Hito | Qué se mide | Estado |
|---|---|---|
| 2 | `scene_radius` y `monolith_height` medidos en el blockout | **Registrado** |
| 2 | `orbit_radius` derivado por bisección, con `framing_margin` usado | **Registrado** |
| 3 | Benchmark `safe-interior-visible` (159 primitivas) — mín/mediana/máx | **Registrado** |
| 3 | Benchmark `safe-opaque-water` (160 primitivas) — control de oclusión | **Registrado** |
| 3 | `interactive_frame_time` del perfil interactivo | **Registrado** — perfil fijado en `MEDIA` (400 × 300) |
| 5 | Calibración de `L-02`: `distance_boat`, `range`, `intensity` | Pendiente |
| 6 | `reveal_duration` derivada de `interactive_frame_time` | Pendiente |
| 7 | Matriz de rendimiento por preset | Pendiente |
| 8 | Hardware de medición y tiempos finales en release | Pendiente |

**Regla.** Todos los benchmarks se ejecutan en release. El perfil `dev` de este proyecto lleva `opt-level = 3` heredado de la base académica, así que un tiempo medido en debug **parece** comparable a release y no lo es.
