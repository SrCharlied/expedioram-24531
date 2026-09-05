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

> **Cambiado en la Tarea 7.1.** `BAJA` dejó de ser la reserva y pasó a ser
> el perfil de serie: `MEDIA` no aguanta el peor encuadre alcanzable con
> margen. Ver «La mitigación, medida y aplicada».

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

## Cierre del Hito 3

**Fecha:** 1 de septiembre de 2026  
**Commit:** `9dfaa5b`  
**Árbol `src/`:** `43ce5b1ccea9e752e0b94e5730e816c563bdcee3`

La Tarea 3.9 quedó en `d382aca`; el commit de cierre es posterior porque
incluye dos arreglos que salieron después: el `default-run` de `Cargo.toml`
y el cambio de la ventana al nivel seguro.

### Gate

- `cargo fmt -- --check` — OK, sin diferencias
- `cargo clippy --all-targets -- -D warnings` — OK, sin warnings
- `cargo test` — 160 tests en 5 targets: 154 unitarios y 6 de integración, 0 fallos
- `cargo build --release` — OK
- `cargo run` — arranca y se mantiene en ejecución

### Tarea 3.10 — `rayon`

**Estado: evaluada, no activada.**

Su disparador dice evaluarla *solo si la resolución interactiva y la
aceleración estática no bastan*, y no es el caso:

- La aceleración estática evita alrededor del **92 %** de las pruebas de
  primitiva.
- El perfil `MEDIA` entrega unos **40 fps** durante el movimiento.
- Al detenerse se produce un **único** cuadro final de unos **96 ms**.

Añadir paralelismo ahora traería una dependencia, una feature y un camino de
código nuevo sin resolver ningún problema observado.

`rayon` queda como **reserva de los Hitos 5 a 7**.

### Disparador futuro de `rayon`

Reconsiderar `rayon` si, después de incorporar reflexión y refracción, el
perfil `MEDIA` deja de ser suficientemente cómodo **y** el perfil `BAJA`
tampoco permite desarrollar o verificar la interacción con fluidez.

**No se fija un umbral numérico** —ni 30 ni 60 fps— a propósito: el plan ya
decidió medir antes de imponer metas, y poner una cifra ahora sería
exactamente la clase de objetivo inventado que ese criterio prohíbe. La
condición es cualitativa por diseño, y la medición que la resuelva será la
del Hito 5, cuando el volumen de agua empiece a generar rayos secundarios.

### Un fallo detectado al cerrar

`cargo run` a secas dejó de funcionar en la Tarea 2.3, al crear
`src/bin/render_scene.rs`: con dos binarios en el paquete, Cargo se niega a
elegir. Se arregló con `default-run` en `Cargo.toml`.

Pasó tres tareas inadvertido porque **ninguno de los cuatro gates ejecuta
`cargo run`**: `fmt`, `clippy`, `test` y `build` no tocan ese camino. Habría
reventado en la Tarea 8.6, que verifica ese comando exacto en un clon
limpio.

A partir de aquí, la comprobación de que el binario de ventana arranca se
hace en cada cierre de tarea, con un timeout. Sigue siendo necesaria la
verificación visual humana: que el proceso viva no dice qué se ve.

---

## Hito 4 — Texturas, materiales y skybox

### Skybox: la mitad del panorama que sí se ve

Al integrar el muestreo equirectangular en la Tarea 4.5 se midió algo que
la autoría de los panoramas, en la Tarea 4.2, había supuesto al revés.

#### La medición

La cámara hero orbita a `35°` de elevación, con la vista inclinada `31.94°`
hacia abajo y medio FOV vertical de `30°`. Los rayos que fallan la
geometría salen del ojo entre unos `−62°` y `−2°` de elevación, medidos
sobre el eje vertical del cuadro:

| Borde del cuadro | Elevación del rayo | `v` muestreada |
|---|---|---|
| Superior | `≈ −1.9°` | `0.489` |
| Centro | `≈ −31.9°` | `0.323` |
| Inferior | `≈ −61.9°` | `0.156` |

Es decir que **todo el fondo de la toma que se presenta está por debajo del
horizonte**: `v` nunca alcanza `0.5`, y el cenit azul profundo del panorama
pintado no aparece en ningún píxel de la vista hero.

Cubrir la esfera completa —lo que ya exigía la Tarea 4.2— era necesario,
pero insuficiente: la mitad inferior se había resuelto como relleno neutro,
con el argumento de que «ahí no hay cielo que describir». Resulta que ahí
está todo el cielo que se ve.

#### La corrección

Solo el hemisferio inferior de `skybox_painted`. Franja cálida breve —unos
`5°`— que deja legible dónde está el horizonte, tránsito por malva, e
índigo profundo como color dominante, cerrando en el `0x040C24` que el
proyecto eligió como fondo en el Hito 1.

Medido sobre los renders de `evidence/hito4/`, promediando la esquina
inferior izquierda —cielo puro— y la franja superior:

| Estado | Franja superior antes | Franja superior después | Fondo bajo antes | Fondo bajo después |
|---|---|---|---|---|
| `reveal 0.66` | `#C6B29E` | `#BAAAA0` | `#9B8F83` | `#848184` |
| `reveal 1.00` | `#AA8A6B` | `#93766E` | `#6C584B` | `#16214F` |

El estado final pasa de un café uniforme a índigo. El estado intermedio
pasa de beige cálido —`r − b = 24`— a gris neutro frío —`b − r = 0`—, y esa
neutralidad no es un residuo: interpolar marfil contra índigo **en lineal**
da un punto medio desaturado por construcción, y el marfil pesa mucho más
en lineal de lo que su aspecto sugiere. Se registra como consecuencia
conocida y no como defecto pendiente.

El panorama pálido queda **intacto**: `assets/skybox/pale.png` es
byte-idéntico, igual que las seis texturas de material, y las semillas del
generador no cambiaron. El único asset modificado es
`assets/skybox/painted.png`.

#### Qué queda amarrado con tests

La calibración vive en cinco tests del generador, para que no se pierda en
un retoque posterior: el fondo de la toma hero es azul y no cálido a `−15`,
`−30`, `−45` y `−60` grados; la franja cálida sigue viva al ras del
horizonte y ya se apagó diez grados más abajo; el nadir cierra cerca del
azul de noche del proyecto; y el hemisferio inferior no es plano —un telón
de un solo color se leería como un error de render—.

El muestreo tiene los suyos aparte, en `src/skybox.rs`: direcciones
cardinales contra UV esperada, el azimut atado al yaw de la cámara vía
`eye_at_yaw`, la continuidad de la costura en `+X`, y el cenit exacto, que
bajo `WrapMode::Repeat` envolvería a `v = 0` y devolvería el color del
suelo a un rayo que mira recto hacia arriba.

### Gate del Hito 4

Dos criterios: render safe con lienzo y render final con cinco materiales
claramente distinguibles, y todos los assets cargando desde un clon limpio.

**Preset del gate: `safe-opaque-water`.** El agua es uno de los cinco
materiales, y en `safe-interior-visible` el volumen no existe todavía —lo
construye la Tarea 5.4—, así que la bahía queda como basalto sin iluminar.
El preset opaco es el que puede mostrar los seis materiales a la vez. Sigue
sin servir para aprobar rendimiento, por la razón ya registrada en el Hito
3: oculta 44 primitivas del interior.

| Render | Estado | Archivo |
|---|---|---|
| Lienzo | `reveal 0.0` | `evidence/hito4/gate_lienzo.png` |
| Materiales | `reveal 1.0` | `evidence/hito4/gate_materiales.png` |
| Materiales, sin luces | `reveal 1.0`, shading `albedo` | `evidence/hito4/gate_materiales_albedo.png` |

`800 × 600`, preset `safe-opaque-water`, 160 primitivas, 8 texturas
cargadas. La progresión completa de la revelación está en
`reveal_000/033/066/100.png`, a `420 × 315`.

#### Los seis materiales, medidos

Tono medio de cada textura, promediando los bytes del PNG. Se promedia en
sRGB y no en lineal porque aquí no se suma energía: se estima el tono que el
ojo compara al ver dos materiales uno al lado del otro.

| Material | Tono medio | Descripción |
|---|---|---|
| `canvas` | `#DFD7C3` | marfil |
| `water` | `#3D7BA2` | azul medio |
| `wet_basalt` | `#4C5058` | azul grisáceo oscuro |
| `aged_wood` | `#583C26` | marrón cálido |
| `meadow` | `#507C39` | verde |
| `pictorial_crystal` | `#90C1CF` | cian pálido |

Separación del par más cercano, en distancia L1 sobre bytes `0..255`:

| Par | L1 |
|---|---|
| `wet_basalt` / `meadow` | `79.4` |
| `wet_basalt` / `aged_wood` | `83.0` |
| `aged_wood` / `meadow` | `91.6` |
| `canvas` / `pictorial_crystal` | `113.1` |

Los quince pares quedan por encima de `60`, unos 20 puntos por canal. El
umbral vive en un test del generador, así que un retoque de textura que
acerque dos materiales falla antes de llegar a un render.

Un segundo test fija que el **lienzo sea el más claro de los seis**, con
`633` de brillo sumado contra `544` del segundo. Si un material final
saliera más claro que el lienzo, pintarlo se leería como aclarar en vez de
como pintar.

#### El estado sin pintar, comprobado del lado de la escena

En `reveal 0.0` lo único que no es lienzo son las **seis piezas de `G-04`**
—la paleta y el pincel—, y son inertes: la herramienta con la que se pinta
no se pinta a sí misma. El test recorre los objetos de los dos presets y lo
verifica contra el material del plinto.

Ese test existe por un fallo real: el nivel seguro se construyó en la Tarea
3.7, antes de que existiera la revelación, con el mismo material en los dos
extremos de cada objeto. La interpolación funcionaba y no se veía nada.

#### Clon limpio

```text
git clone . <temporal>
cd <temporal>
render_scene --preset safe-opaque-water --width 800 --height 600 --reveal 1.0
```

Ocho assets versionados, ocho texturas cargadas, y el PNG resultante
**byte-idéntico** al del árbol de trabajo. La raíz de assets es el
directorio actual, así que el binario ejecutado dentro del clon lee los
archivos del clon y no los del repositorio original.

El camino de error también se verificó, escondiendo un asset en el clon:

```text
error: no existe la textura .\assets/skybox/painted.png
  genera los assets con: cargo run --release --bin generate_assets
```

Código de salida `1`, la ruta en el mensaje y ninguna sustitución
silenciosa. Es lo que el plan exige: un asset ausente se descubre al
arrancar, no mirando la imagen final.

#### Checklist

| Criterio | Estado |
|---|---|
| Render safe con lienzo | **Cumple** |
| Cinco materiales finales distinguibles, más el lienzo | **Cumple** — quince pares por encima del umbral |
| Todos los assets cargan desde un clon limpio | **Cumple** — render byte-idéntico |
| Asset ausente da error con su ruta | **Cumple** |

#### Lo que queda abierto y no es del Hito 4

En `safe-interior-visible` el interior de la bahía se ve **negro**: sin
volumen de agua no hay superficie que devuelva luz, y el basalto interior
solo recibe ambiente. Lo resuelven la Tarea 5.4 —volumen cerrado y borde
roto— y la 5.7 —calibración de `L-02`—. Se registra aquí para que no se
confunda con un defecto de materiales.

Los tiempos de estos renders son informativos, de una sola pasada
—`0.17 s` a `0.25 s` a `800 × 600`—. Los benchmarks del proyecto se hacen
con repeticiones y sobre `safe-interior-visible`, como en el Hito 3.

---

## Hito 5 — Reflexión, refracción y Aguas Voladoras

### Tarea 5.4 — el volumen cerrado, y un preset que había dejado de ser lo que decía

Al insertar el volumen con óptica real salió a la luz una deriva: el preset
`safe-opaque-water` **ya no era opaco**.

`Palette::registrar` le dio al agua sus techos reales —`0.9 / 0.9`,
`ior 1.333`— en el Hito 4, y el preset «opaco» insertaba `paleta.water` tal
cual. Mientras `cast_ray` ignoraba los techos eso daba lo mismo. Al llegar
la recursión de la Tarea 5.3, el control de oclusión empezó a refractar y
dejó de ocultar las 44 primitivas del interior, que es su única razón de
existir.

**Los tiempos del Hito 3 no quedan invalidados**: se midieron antes de que
existiera la recursión, así que en ese momento el volumen era opaco de
hecho. Lo que ya no reproduce esos números es volver a correr ese preset
hoy sin la corrección.

La corrección: el control deriva del agua y le quita **solo** la óptica
—techos a cero—, conservando albedo, textura, escala UV, specular y
`ShadowMode::Ignore`. Un control tiene que diferenciarse de la escena real
en exactamente una cosa. Conservar `Ignore` no es descuido: el inventario
prohíbe que `A-01` bloquee sombras, y cambiarlo rompería la comparación con
el Hito 3.

#### Los tres presets

| Preset | Primitivas | Qué mide |
|---|---:|---|
| `safe-refractive-water` | 160 | **el canónico** desde 5.4: el volumen real con óptica |
| `safe-interior-visible` | 159 | el interior sin el coste de la refracción; referencia del Hito 3 |
| `safe-opaque-water` | 160 | control de oclusión, con los techos en cero |

Medido a `800 × 600`, `reveal 1.0`, cinco repeticiones:

| Preset | Mediana | Reflejados | Refractados |
|---|---:|---:|---:|
| `safe-refractive-water` | `0.2887 s` | `48 199` | `40 850` |
| `safe-interior-visible` | `0.2223 s` | `28 936` | `26 068` |
| `safe-opaque-water` | `0.2343 s` | `26 084` | `23 407` |

**Estos tiempos son de una sesión más lenta.** Salieron un `45 %` por
encima de los de la Tarea 5.8, con los mismos conteos de rayos. Siguen
siendo comparables **entre sí** —el volumen cuesta un `30 %` sobre la
referencia—, pero no con los del gate. Ver *Procedencia de las tres
sesiones* en la Tarea 5.8.

El volumen añade un **30 %** sobre la referencia sin refracción, y casi
duplica los rayos secundarios.

La comparación con el Hito 3 se hace en la Tarea 5.8, con cifras de una sola
sesión: **`2.09×`**. El `3.0×` que decía esta sección mezclaba dos sesiones
distintas.

Nótese que los contadores **no son del agua**: el cristal pictórico también
transmite —`transmission_cap = 0.25`— y el Monolito ocupa buena parte del
cuadro. Por eso el control opaco sigue marcando 23 407 refracciones. Lo que
se puede atribuir al volumen es la diferencia.

#### El volumen no se rasga

Una sola primitiva, comprobado en los dos presets que lo insertan. Y desde
la Tarea 5.3 esto dejó de ser una preferencia de presupuesto: **cada
frontera cuesta un nivel de recursión**. Un volumen partido en tres losas
gastaría los tres niveles de `MAX_DEPTH` solo en atravesarse, y el interior
de la bahía terminaría en cielo antes de llegar al barco.

#### Oclusión de la cara frontal, medida

Los ocho cuboides de terreno de `A-11` cubren el **88.7 %** de la cara
frontal del volumen, muestreado sobre una rejilla de `120 × 120`. Los ocho
cruzan el plano de la cara: quedan mitad delante y mitad dentro.

El 11 % descubierto no está repartido: se concentra en el **borde superior**,
donde los bloques más bajos —`2.22`, `2.25`, `2.48` y `2.55` de alto contra
una cara que llega a `2.6`— dejan asomar el filo del agua. Eso es lo que
produce el aspecto rasgado en vez de una caja limpia.

El test no puede identificar el borde por «lo que sobresale de la cara»:
la masa principal del lecho mide `5.4` de fondo contra los `5.0` del
volumen y también asoma. Construye el borde solo.

#### Lo que este render todavía no resuelve

El interior de la bahía sigue oscuro y el borde roto se lee casi negro por
su cara frontal. Es lo esperado: reciben solo ambiente. Los resuelven la
Tarea 5.6 —validación de sombras submarinas— y la 5.7 —calibración de
`L-02`—. El render de `evidence/hito5/safe-refractive-water.png` se guarda
como el antes de esa calibración.

### Tarea 5.5 — barco, cadena y ancla

Presupuesto verificado **entrada por entrada** y no solo en el total: un
casco que se pase de largo y un mástil que se quede corto se cancelarían en
la suma.

| Entrada | Primitivas |
|---|---:|
| `A-03` casco | 12 |
| `A-04` mástil y soportes | 3 |
| `A-05` cadena | 8 |
| `A-06` ancla | 3 |

#### El metal de la cadena era una promesa del comentario

El doc de `A-05` decía que la cadena reutiliza `wet_basalt` «distinguido por
escala UV, albedo gris y specular local», como pide el inventario para no
crear un sexto material final. El código pasaba `paleta.wet_basalt` tal
cual: la cadena y el ancla eran roca del acantilado.

Ahora existe `metal_reusado`, que deriva del basalto y cambia tres cosas:

| Propiedad | Basalto | Metal |
|---|---:|---:|
| Factor de tinte | — | `(0.70, 0.78, 1.00)` lineal |
| Escala UV | `3.0` | `12.0` |
| `shininess` | `96` | `220` |
| `specular_strength` | `0.85` | `0.80` |
| `reflection_cap` | `0.0` | `0.0` |

Color resultante, que no es el factor sino el producto:

| | Con texturas | Sin texturas |
|---|---|---|
| Roca | `#4C5058` | `#42454C` |
| Metal | `#404758` | `#373D4C` |

Acero oscuro y frío, no gris claro: el factor solo puede atenuar. La
proporción azul/rojo sube de `1.38` en la roca a `1.93` en el metal, y eso
es lo que se lee como acero y no como piedra.

La distinción **no es un brillo más fuerte**. `wet_basalt` ya viene con
`specular_strength = 0.85` porque la roca mojada brilla mucho, y competir
en fuerza no distinguiría nada. Lo que separa al metal es el **tamaño del
lóbulo**: un punto de luz pequeño e intenso en vez de un brillo extendido.

La escala UV de `12.0` sale del tamaño de la pieza: los eslabones miden
`0.13`, y con la escala `3.0` del acantilado la textura no alcanzaría a
repetir ni una vez sobre una cara.

`reflection_cap` se queda en cero, heredado. Son once primitivas pequeñas
**dentro** del volumen de agua, y cada una reflejando costaría un nivel de
los tres de `MAX_DEPTH`, justo donde el rayo ya gastó dos en entrar.

#### La silueta, medida

Lo que hace legible al pecio no es el detalle sino tres decisiones de forma,
y las tres están ahora amarradas por tests:

- El cuerpo **se estrecha hacia proa**: el ancho de las cinco secciones
  decrece de forma monótona.
- El casco es más de tres veces más largo que ancho.
- La popa es la pieza más alta.

Y una que descubrí midiendo: **la popa rompe la superficie del agua**, a
`2.76` contra los `2.6` del volumen. No estaba escrito en ninguna parte.
Una sola pieza de doce, y mejora la lectura: un pecio escorado con la popa
levantada se lee mucho mejor que un casco enteramente sumergido. Queda
registrado como intencional, con un test que permite una o dos piezas
asomando y ninguna más.

El mástil sobresale `2.05` unidades sobre la superficie. Es el ancla visual
del barco: con la bahía en penumbra, es lo único que se lee sin buscar.

En planta, nada del barco se sale del volumen; en altura, solo el mástil y
la popa, a propósito.

#### Lo que sigue faltando

El casco bajo el agua se lee **débil**: recibe solo ambiente, y la
superficie del agua encima refleja el cielo. La cadena y el ancla no se
distinguen a resolución de presentación. Los resuelven la Tarea 5.6
—sombras submarinas— y la 5.7 —calibración de `L-02`, la luz enlazada a
Aguas Voladoras cuyo objetivo declarado es el barco—.

#### Un hallazgo fuera de alcance

`A-07` (kelp) tiene el mismo defecto que tenía `A-05`: su doc dice
«reutiliza `meadow` con tinte submarino» y el código pasa `paleta.meadow`
sin teñir. No se tocó, porque es otra entrada del inventario y su tinte es
una decisión visual que corresponde aprobar aparte. **Corregida a continuación**, con
autorización explícita.

### `A-07` kelp — la misma corrección, y una trampa del tinte

Autorizada aparte, antes de la Tarea 5.6. El kelp tenía el defecto idéntico
al de la cadena: su doc prometía «`meadow` con tinte submarino» y el código
pasaba `paleta.meadow` sin teñir.

El tinte **corta el rojo** y conserva verde y azul, con factor
`(0.30, 0.85, 1.00)` en lineal. No es una preferencia de paleta: es lo que
hace el agua, que absorbe primero las longitudes de onda largas. A un metro
de profundidad lo primero que se pierde es el rojo, y a un césped al que se
le quita el rojo se le ve submarino sin necesidad de un sexto material.

| | Con texturas | Sin texturas |
|---|---|---|
| Pradera | `#507C39` | `#4C853D` |
| Kelp | `#2B7339` | `#297B3D` |

Se le suma `ShadowMode::Ignore`. Son doce frondas delgadas dentro de la
bahía: sombras duras proyectadas por doce palos motearían el lecho con un
patrón que nadie lee como sombra de kelp, y costarían un rayo de sombra por
fronda y por luz para producirlo.

Dentro de la bahía quedan **45 primitivas que sí proyectan sombra** de las
58: el volumen y las doce frondas son las dos excepciones. Lo que da
profundidad al lecho son las rocas y el barco.

#### La trampa: `with_tint` reemplaza, no multiplica

El primer intento usó un tinte absoluto y el test lo tumbó: el kelp salía
**más claro** que la pradera en vez de más oscuro.

`Material::with_texture` pone el albedo en blanco a propósito, para no
oscurecer dos veces. Sobre un material así, reemplazar el albedo equivale a
multiplicar la muestra y todo funciona. Pero sobre un material de color
plano, reemplazarlo sustituye el color entero y el «tinte» deja de teñir.

Y el proyecto corre en los dos modos: con assets, y con `--no-textures`,
que es como corren **todos los tests**. Un tinte absoluto daba dos
materiales distintos según hubiera texturas cargadas.

La corrección es un helper de tres líneas, `tenir(material, factor)`, que
multiplica el albedo que el material ya tiene. Da el mismo color efectivo en
los dos modos, y hay un test que lo comprueba muestreando una textura de un
píxel contra un color plano equivalente. Los dos materiales derivados de
Aguas Voladoras usan ese único idioma.

El factor va en **lineal** a propósito: es una atenuación de energía por
canal, no un color elegido a ojo.

---

### Tarea 5.6 — validación de sombras submarinas

Configuración controlada: agua presente, barco dentro, y cuatro
plataformas de luces sobre **la misma escena**, cambiando solo el rig.

Los cuatro criterios viven como tests en `tests/submarine_shadows.rs`, no
solo como render: un criterio cualitativo mirado a ojo no protege de una
regresión. El render y los números salen de
`cargo run --release --example submarine_shadows`.

#### Criterio 1 — el barco no está negro

Medido sobre **242 rayos** que dan en las doce piezas del casco, no sobre un
punto. Brillo sumado de los tres canales:

| Luces | Mínimo | Media | Máximo | Caras iluminadas |
|---|---:|---:|---:|---:|
| Rig completo | `0.0163` | `0.2064` | `0.4400` | 197 / 242 |
| Solo `L-02` | `0.0054` | `0.1054` | `0.2496` | 178 / 242 |
| Solo `L-01` | `0.0056` | `0.0984` | `0.2317` | 142 / 242 |
| Sin luces | `0.0054` | `0.0104` | `0.0161` | 0 / 242 |

«Iluminada» significa que supera tres veces su propio ambiente. Con `L-02`
sola, la media del casco es **10.1 veces** el ambiente.

`L-02` alcanza el 73.6 % de las caras y `L-01` el 58.7 %: se complementan,
y juntas llegan al 81.4 %. No al 100 %, y eso es correcto —ver abajo—.

#### Tres cosas que la medición corrigió

**El pecio se hace sombra a sí mismo.** Con `L-01` sola, el punto central de
la cubierta queda **negro**. El oclusor es el objeto 117: una de las tres
costillas expuestas por la brecha del casco. No es un defecto: es lo que
hace que las costillas se lean como volumen. Un criterio medido en un solo
punto habría dado un falso negativo.

**Una cara «superior» puede estar dentro de otra pieza.** El casco está
apilado —cuerpo, cubierta, costillas—, así que un rayo lanzado desde la cara
de arriba de una pieza nace **dentro** de la de encima, y el cuboide
devuelve entonces su cara de salida: una superficie que mira hacia abajo y
no ve ninguna luz. Sale negra con razón. Los rayos se disparan por eso desde
justo debajo de la superficie del agua, que garantiza tocar una cara
expuesta, la misma que ve la cámara.

**El azul de `L-02` no vuelve azul a la madera.** El casco es marrón y
absorbe azul: ni una luz azul pura lo pone azul, y el color final llega a
tener más rojo que azul porque el ambiente marrón pesa. Lo comprobable no es
el color final sino la **temperatura del aporte**: la razón azul/rojo de lo
que `L-02` añade es más del doble que la del ambiente sobre la misma
superficie. Si la luz fuera la cálida de `L-01`, esa razón no subiría.

La cara más apagada del casco queda en `0.0084`, que es exactamente
`albedo × AMBIENT` sobre la madera. Está ahí porque una costilla la tapa, y
que exista ese suelo es la razón de que `AMBIENT` no sea cero: sin él, lo
que una costilla tapa se vería negro absoluto y el pecio perdería su
silueta interior.

#### Criterio 2 — el agua no bloquea sombras

El test comprueba **primero** que el volumen esté de verdad en medio: la
superficie del agua, en `y = 2.60`, queda entre la cubierta (`y = 2.46`) y
`L-02` (`y = 4.41`). Sin esa comprobación el criterio pasaría por vacío si
alguien moviera la luz bajo el agua.

Y aun con el volumen en medio, la cubierta no está en sombra:
`ShadowMode::Ignore` de `A-01` funciona en la escena real, no solo en el
test sintético del Hito 3.

#### Criterio 3 — las rocas opacas sí producen sombra

Las seis rocas se localizan **por geometría y no por semilla**: son las
únicas primitivas de Aguas Voladoras que caben enteras dentro del volumen
—el lecho es más ancho, el borde roto lo atraviesa, el barco asoma— y son
anchas, lo que descarta kelp y eslabones.

Para cada una, el punto de prueba se construye inmediatamente **detrás** de
la roca, del lado opuesto a la luz, y no «sobre el lecho, bajo la roca»: a
esta distancia la línea hacia `L-02` sube en diagonal, y un punto justo
debajo podría salirse por el costado y dar un falso negativo.

El test complementario muestrea el lecho en una rejilla de `9 × 9` y exige
que haya sombra **y** luz: que existan sombras no puede significar que la
bahía entera esté en penumbra.

#### Criterio 4 — el Monolito conserva su sombra de contacto

Se valida contra `L-01`, que es la única luz con sombras que lo alcanza.
La huella del Monolito se mide de la escena, y el punto de prueba va al pie,
del lado contrario a la luz y a ras del plinto.

La comprobación no se queda en «hay sombra»: se traza el rayo de sombra y se
verifica que **lo primero que encuentra pertenece al grupo `Monolith`**. Eso
es lo que distingue una sombra de contacto de cualquier otra sombra del
diorama cayendo en el mismo sitio.

#### Los renders

Cuatro imágenes a `800 × 600` en `evidence/hito5/`:

| Archivo | Rayos de sombra |
|---|---:|
| `sombras-rig-completo.png` | `174 427` |
| `sombras-solo-l02.png` | `44 106` |
| `sombras-solo-l01.png` | `130 321` |
| `sombras-sin-luces.png` | `0` |

El render con solo `L-02` es además la evidencia visual del **light
linking**: el resto del diorama cae a ambiente mientras la bahía queda
iluminada. `L-02` afecta y ocluye únicamente a `FlyingWaters`, y se ve.

### Tarea 5.7 — calibración de `L-02` con el blockout real

Los cinco pasos que exige el inventario. `cargo run --release --example calibrate_l02`
imprime el barrido completo; aquí quedan los valores.

#### 1 y 2 · las distancias medidas

| Magnitud | Valor | En múltiplos de `S` |
|---|---:|---:|
| `scene_radius` | `12.0586` | `1.000 S` |
| `distance_boat` | `2.3179` | `0.192 S` |
| `distance_farthest` (centro) | `5.1560` | `0.428 S` |
| `distance_farthest` (esquina) | `7.5026` | `0.622 S` |

`distance_boat` se mide al **centro visible** del barco, que es lo que pide
el inventario, y no al centro de la caja del casco. La diferencia importa:
el centro visible está `0.34` más arriba y `0.22` más a popa, porque la
cubierta y la popa aportan casi toda la superficie expuesta y el centro de
la caja cae por debajo de todas ellas. Se obtiene promediando los `242`
puntos donde los rayos de la rejilla de la Tarea 5.6 tocan el casco.

Todas las entradas de Aguas Voladoras del nivel seguro son obligatorias
—`A-09` y `A-10` son las opcionales y valen cero primitivas—, así que el
objeto más lejano se busca sobre las 58.

#### 3 · el alcance elegido

Barrido de la razón entre lo que recibe el objeto más lejano y lo que recibe
el barco:

| `range` | `= S ×` | Lejano / barco |
|---:|---:|---:|
| `1.8088` | `0.15` | `29.0 %` |
| `2.4117` | `0.20` | `34.5 %` ← heredado |
| `3.0147` | `0.25` | `40.5 %` |
| **`3.6176`** | **`0.30`** | **`46.5 %`** ← elegido |
| `4.8234` | `0.40` | `57.4 %` |
| `6.6322` | `0.55` | `69.9 %` |
| `9.6469` | `0.80` | `82.3 %` |

**`range = 0.30 S`.** El fondo de la bahía conserva casi la mitad de la
iluminación del barco: baja lo justo para que la bahía tenga profundidad, y
no tanto como para que su fondo desaparezca.

El argumento original para un alcance estrecho era evitar que el azul se
derramara fuera de Aguas Voladoras, y de eso ya se encarga el **light
linking**, que lo lleva a cero exacto. La atenuación solo tiene que modelar
la caída *dentro* de la bahía, así que ensancharla no cuesta nada fuera.

#### 4 · `E_boat` y la intensidad derivada

Barrido de `E_boat` contra el brillo del casco, medido sobre las mismas 242
caras y expresado en byte sRGB, que es la escala en la que se juzga si algo
se lee:

| `E_boat` | `intensity` a `0.30 S` | Media del casco | Byte medio | Byte máximo |
|---:|---:|---:|---:|---:|
| `1.00` | `1.4106` | `0.0940` | `50` | `87` |
| `1.50` | `2.1158` | `0.1367` | `60` | `105` |
| **`2.00`** | **`2.8211`** | **`0.1794`** | **`69`** | **`120`** |
| `2.50` | `3.5264` | `0.2222` | `77` | `133` |
| `3.00` | `4.2317` | `0.2649` | `84` | `144` |
| `4.00` | `5.6422` | `0.3503` | `96` | `164` |

**`E_boat = 2.0`**, que da `intensity = 2.8211`. Es el único número
artístico de la calibración; todo lo demás se deriva de él y de la
geometría. Los valores heredados equivalían a `E_boat ≈ 1.04`, con el casco
en el byte `50`: una mancha oscura.

`2.50` y `3.00` quedan registrados como las alternativas si el gate de la
Tarea 5.8 pide más presencia. Subir de ahí empieza a desbalancear la bahía
contra el resto del diorama, que solo tiene `L-01`.

**La intensidad se deriva, no se escribe.** `l02_intensity` invierte el
modelo de atenuación:

```text
intensity = E_boat × (1 + (distance_boat / range)²)
```

Si la composición se mueve, la intensidad la sigue sola y la contribución
sobre el barco se mantiene. Es la misma razón por la que `orbit_radius` se
deriva del encuadre en vez de escribirse.

#### 5 · antes y después

Sobre las 242 caras visibles del casco, brillo sumado de los tres canales:

| Luces | Antes (media) | Después (media) | Antes (máx) | Después (máx) |
|---|---:|---:|---:|---:|
| Solo `L-02` | `0.1054` | `0.1955` | `0.2496` | `0.4484` |
| Rig completo | `0.2064` | `0.2964` | `0.4400` | `0.6392` |

El casco recibe **1.85 veces** más luz de `L-02`. El mínimo no cambia
—`0.0054` en las dos—: las caras que una costilla tapa siguen apoyadas en el
suelo de ambiente, que es lo correcto, porque no reciben luz directa ni
antes ni después.

Renders a `800 × 600`, mismo encuadre y mismo preset:

| | Archivo |
|---|---|
| Antes | `evidence/hito5/l02-antes.png` |
| Después | `evidence/hito5/l02-despues.png` |

Los dos salen de `cargo run --release --example calibrate_l02`, que arma el
rig heredado —`intensity 2.0`, `range 0.20 S`— junto al calibrado y renderiza
los dos con la misma cámara.

Los dos renders se toman con la geometría **actual**, así que aíslan el
cambio de `L-02` y nada más: la muesca del borde y la ganancia del pecio
están en las dos imágenes.

Que los genere el propio ejemplo no es comodidad: el «antes» **no se puede
reproducir de otra forma**, porque los valores heredados no viven en ninguna
parte del código. La primera versión de esta evidencia apuntó el antes a
`safe-refractive-water.png`, y una remedición posterior lo sobrescribió con
la escena ya calibrada. El antes se había perdido y hubo que reconstruirlo.

#### Una trampa del ancla que hubo que evitar

`flying_waters_anchor` **cambia** entre la construcción de la escena y el
armado de las luces: se construye con `y = 0` y el nivel seguro la reescribe
a la altura de la superficie del agua. Derivar la posición del barco de ella
desde `light::diorama` habría dado un objetivo desplazado en toda la altura
de la bahía, y la intensidad calibrada contra un punto que no es el barco.

Por eso el centro visible entra como ancla propia, `boat_anchor`, calculada
desde el ancla **base**. El blockout, que no tiene barco, la apunta al
centro de la bahía.

#### Qué queda amarrado con tests

- La atenuación de `L-02` sobre `boat_anchor` es exactamente `E_boat`.
- La intensidad derivada **sube si el barco se aleja**, y vale `E_boat`
  exacto a distancia cero. Un `range` no positivo da cero en vez de dividir
  entre cero.
- El alcance calibrado deja el objeto más lejano en el `46.5 %`, y el
  heredado lo dejaba en el `34.5 %`.
- El desplazamiento medido del centro visible **se vuelve a medir** contra
  la escena, con tolerancia `0.02`. Un número medido a mano se
  desincroniza en el primer ajuste de composición, y de esa posición sale
  la intensidad de la luz.

Las dos tablas ilustrativas del inventario —la de `0.55S` contra `0.20S` y
la cifra del `25.77 %` que justifica el light linking— siguen comprobadas
con luces sintéticas, así que la calibración no las invalida: documentan el
modelo, no el rig.

### Tarea 5.8 — gate de Aguas Voladoras

Toma hero a `800 x 600`, preset `safe-refractive-water`, `reveal 1.0`.
`cargo run --release --example gate_flying_waters` mide los seis criterios;
la imagen es `evidence/hito5/gate-hero.png`.

| Criterio | Veredicto |
|---|---|
| 1 · La superficie devuelve skybox | **Cumple** |
| 2 · El borde frontal permite ver el barco | **Cumple** |
| 3 · El highlight del agua se ve | **Cumple** |
| 4 · Barco, cadena y ancla legibles | **Cumple** |
| 5 · Ni acné severo ni negro total | **Cumple** |
| 6 · Tiempo en release registrado | **Registrado** |

El criterio 4 lo aprobó la revisión visual sobre
`evidence/hito5/gate-hero.png`, después de las tres correcciones de más
abajo. Queda anotado que una versión anterior de esta sección lo marcó como
cumplido **antes** de esa revisión: un criterio de legibilidad lo cierra
quien mira la imagen, no quien la mide.

#### 6 · el tiempo

Siete repeticiones de cada preset, **corridas seguidas en una sola sesión**
para que sean comparables entre sí:

| Preset | Mediana | Primitivas |
|---|---:|---:|
| `safe-refractive-water` | `0.1999 s` | 160 |
| `safe-interior-visible` | `0.1603 s` | 159 |
| `safe-opaque-water` | `0.1592 s` | 160 |

El volumen refractivo cuesta un **25 %** sobre la referencia sin refracción
—`0.1999 / 0.1603 = 1.247`—. `480 000` rayos primarios, `173 744` de sombra,
`47 917` reflejados y `40 730` refractados.

##### Procedencia de las tres sesiones

Hay tres tandas de medición del mismo trabajo en esta evidencia, y la
diferencia entre ellas es la máquina, no la escena:

| Sesión | `refractive` | `interior-visible` | Razón |
|---|---:|---:|---:|
| Tarea 5.4 | `0.2887 s` | `0.2223 s` | `1.30` |
| Primera del gate | `0.1930 s` | `0.1611 s` | `1.20` |
| Definitiva del gate | `0.1999 s` | `0.1603 s` | `1.25` |

Los conteos de rayos son idénticos en las tres, así que la escena era la
misma; la sesión de la 5.4 corrió con la máquina un `45 %` más lenta. **Solo
las razones son comparables entre sesiones**, y las tres coinciden en que el
volumen cuesta entre un `20 %` y un `30 %`.

Por lo mismo, la comparación con el Hito 3 —`0.0956 s` sin óptica ni
texturas— vale como orden de magnitud y no como factor exacto: `2.09×` con
las cifras de esta sesión. La primera versión de esta evidencia dijo `3.0×`
mezclando dos sesiones, que es exactamente el error que la regla del Hito 3
advierte.

No se fija un umbral de fps, por la decisión del Hito 3 de medir antes de
imponer metas. Ninguna de las cinco mitigaciones del plan hizo falta: la
profundidad sigue en `3`, las 58 primitivas de Aguas están completas y el
cristal conserva su reflexión.

#### 1 · la superficie devuelve skybox

Medido apagando el cielo: se trazan las muestras de la superficie con el
panorama real y con un cielo negro plano.

La superficie ocupa `11 317` píxeles y se **submuestrea uno de cada siete**,
`1 617` muestras. Las cifras de abajo son de la muestra, no del total:

| | |
|---|---:|
| Muestras que cambian al apagar el cielo | `972 / 1 617` |
| Aporte medio del cielo | `0.0091` |
| Aporte máximo | `0.0797` |

El `60 %` de las muestras cambia, así que el reflejo llega al píxel. Es
**sutil por geometría**: la cámara mira el agua a unos `58°` de su normal,
donde Schlick da `F ≈ 0.043` y el techo deja `kr ≈ 0.039`. El máximo, nueve
veces la media, cae en las muestras rasantes del borde lejano, que es
exactamente donde Fresnel sube.

#### 2 · el borde frontal permite ver el barco

Recorrido completo del cuadro, sin submuestrear:

| Qué alcanza el rayo primario | Píxeles |
|---|---:|
| Superficie del agua | `11 317`  (2.36 % del cuadro) |
| Borde roto | `11 571` |
| Casco, directo | `147` |
| Casco, a través de la superficie | `1 327` |
| Cadena y ancla, a través | `167` |
| Lecho, kelp, rocas y caras internas | `9 823` |
| Refractados que terminan en cielo | `0` |

El casco suma **`1 474` píxeles** visibles y el borde roto no lo tapa. Que
ningún rayo refractado termine en cielo confirma que `max_depth = 3` alcanza
para cruzar el volumen: el interior siempre encuentra geometría.

#### 3 · el highlight del agua

Medido apagando el specular del material de agua, sobre las mismas `1 617`
muestras:

| | |
|---|---:|
| Muestras con aporte especular apreciable | `227 / 1 617` |
| Aporte máximo | `0.2068` |

Un `14 %` de las muestras lleva highlight, con un máximo fuerte. El specular
sobrevive porque **no entra en el reparto de Fresnel** —se suma después—;
con `kl = 0.1` habría quedado al diez por ciento. Ver la Tarea 5.3.

#### 5 · limpieza

| | |
|---|---:|
| Píxeles en negro absoluto | `0` |
| Píxeles aislados más oscuros que **todos** sus vecinos | `6`  (`0.0012 %`) |

Seis píxeles sobre 480 000 no es acné: es el borde de una arista. El
criterio de detección exige que el píxel sea menos de la mitad de luminoso
que su vecino **más oscuro**, lo que descarta los bordes de sombra
legítimos, que tienen vecinos oscuros a un lado.

#### 4 · legibilidad — el criterio que costó tres intentos

| Parte | Píxeles | Mín | Media | Máx |
|---|---:|---:|---:|---:|
| Superficie del agua | `11 317` | `0.0639` | `0.3560` | `0.9683` |
| Casco visible | `1 474` | `0.1909` | **`0.4524`** | `0.7370` |
| Cadena y ancla | `167` | `0.1921` | **`0.3774`** | `0.6657` |
| Borde roto | `11 571` | `0.0272` | `0.1293` | `0.9717` |

El contraste **no se mide contra la media global de la superficie**. Esa
media incluye el fondo de la bahía y los highlights del borde lejano, que no
son «el agua que rodea al casco». Se mide contra el anillo de píxeles de
superficie a menos de seis píxeles del casco, excluyendo los que muestran el
casco **a través** del agua —esos son casco, no entorno, y contarlos hacía
que el entorno subiera junto con la ganancia y el contraste no se moviera—:

| | |
|---|---:|
| Casco | `0.4524` |
| Agua que lo rodea, `1 363` píxeles de anillo | `0.3605` |
| **Contraste** | **`1.25`** |

Con la media global el contraste habría salido `1.27`; con el anillo mal
construido, `1.02`. El número honesto es el del anillo limpio.

Color medio en bytes sRGB, que muestra por dónde se lee además del brillo:

| | R | G | B | Rojo / azul |
|---|---:|---:|---:|---:|
| Casco | `127.6` | `112.2` | `110.2` | `1.16` |
| Agua que lo rodea | `71.7` | `95.5` | `116.6` | `0.61` |

El casco es más claro **y** más cálido que el agua. Las dos cosas suman.

#### Las tres correcciones que hicieron falta

**Uno · el casco necesitaba una ganancia, no un tinte.** El idioma `tenir`
solo puede quitar. `ganancia_local` es su contraria, y su techo sale de la
textura: el albedo efectivo es `albedo × muestra`, así que la ganancia puede
subir el albedo hasta `1 / pico`. Con textura el albedo pasa de uno, que ahí
no es una reflectancia sino un factor sobre una muestra oscura.

El techo va **por canal**. Con un solo escalar las dos rutas —con textura y
sin ella— dejan de coincidir en cuanto el recorte muerde, y se vio al subir
la ganancia: un test reportó `0.8` contra `0.625` en el canal verde. La
madera del pecio tiene el rojo a `0.171` y el azul a `0.017`; un techo común
los trataría igual.

`GANANCIA_DEL_PECIO = 3.2`, contra un techo de `5.83`. El barrido:

| Ganancia | Casco | Entorno | Contraste |
|---:|---:|---:|---:|
| `1.8` | `0.3752` | `0.3579` | `1.05` |
| `2.6` | `0.4217` | `0.3595` | `1.17` |
| **`3.2`** | **`0.4524`** | **`0.3605`** | **`1.25`** |
| `3.8` | `0.4805` | `0.3615` | `1.33` |

Se eligió `3.2` y no `3.8` porque a `3.8` el máximo del casco llega a `0.85`
y empieza a quemarse. El entorno se mantiene plano en el barrido, que es la
señal de que la ganancia hace lo que dice.

**Dos · el engrosamiento alcanza a las tres piezas del ancla.**
`GROSOR_METAL = 0.22` sustituye los `0.13` del eslabón y los `0.10`–`0.12`
de la caña, los brazos y el arganeo. Sube el grosor y no el largo.

**Tres · la muesca del borde roto.** Y esta es la que resolvió el problema
de verdad. Engrosar llevó la cadena de `30` a `61` píxeles, no a los `86`
que predecía el área, y el diagnóstico por número de objeto encontró la
causa: **el bloque `5` del borde roto, de `3.05` de alto, tapaba nueve de
las once piezas**, y el `6` la restante.

Las ocho alturas del borde salen del generador con semilla fija. La
corrección **conserva el multiconjunto** y solo reordena a qué bloque le
toca cuál, de modo que las dos más bajas caigan frente a la cadena. El borde
tiene exactamente las mismas ocho alturas y la misma silueta rasgada; lo que
cambia es que la abertura queda donde hay algo que mirar.

Con la muesca los bloques de enfrente bajaron a `2.22`, y la línea de visión
de los tramos centrales pasaba a `2.20`: dos centésimas por debajo. Tensar
la cadena —comba de `0.35` a `0.20`— los levantó lo justo, sin mover sus
extremos ni despegar el ancla del lecho, que eran las otras dos palancas y
las dos más invasivas.

Resultado sobre la cadena y el ancla: `30 → 61 → 167` píxeles, y la
luminancia media de `0.2354` a `0.3774`, por encima del agua que las rodea.

#### Visibilidad al orbitar

| `yaw` | Casco | Cadena y ancla |
|---:|---:|---:|
| `45°` | `1 201` | `154` |
| `90°` (hero) | `1 474` | **`167`** |
| `135°` | `1 236` | `98` |
| `180°` | `793` | `22` |
| `225°` | `111` | `1` |
| `270°` | `6` | `24` |
| `315°` | `643` | `50` |

La muesca invirtió la relación: antes la cadena se veía mejor a `yaw 45°`
que en la hero —`137` contra `61`—, y ahora la hero es la mejor vista de las
siete. Los cuartos de atrás siguen tapados por el Monolito y las Praderas,
que es correcto: la bahía está al frente.

#### Cierre del Hito 5

Gates finales, con el árbol en el estado de esta sección:

```text
cargo fmt -- --check                        OK
cargo clippy --all-targets -- -D warnings   0 avisos
cargo test                                  306 tests, 0 fallos
cargo build --release                       OK
cargo run                                   arranca, 160 primitivas, 8 texturas
```

##### Los generadores de evidencia abortan de verdad

Abortaban si no **cargaban** los assets, pero no si no **escribían** la
imagen: imprimían el error del `save_png` y terminaban con código `0`. Un
guion que los invocara los daba por buenos sin evidencia producida.

Los tres pasan ahora por un `guardar` que aborta con código `1` y la ruta en
el mensaje. Comprobado bloqueando el directorio de destino:

| Ejemplo | Assets ausentes | Fallo al escribir |
|---|---:|---:|
| `gate_flying_waters` | `1` | `1` |
| `calibrate_l02` | `1` | `1` |
| `submarine_shadows` | `1` | `1` |

Reparto de los 306: `276` de librería, `16` del generador de assets, `8` de
humo del render y `6` de sombras submarinas.

Dos imágenes se **eliminaron** en la limpieza de cierre:
`gate-hero-corregido.png`, byte a byte igual a `gate-hero.png`, y
`l02-calibrada.png`, que el antes/después reemplazó semánticamente.
Conservarlas invitaba a que documentación futura volviera a apuntar al
nombre equivocado, que es justo lo que le pasó al «antes» de la Tarea 5.7.

`safe-refractive-water.png` se conserva aunque hoy coincida con la toma
hero: cumple una función distinta —es el preset canónico actual, no un
artefacto histórico de un antes/después—.

Las diez imágenes del hito son:

| Archivo | Qué es |
|---|---|
| `gate-hero.png` | la toma del gate, criterio 4 aprobado sobre ella |
| `safe-refractive-water.png` | el preset canónico |
| `safe-interior-visible.png` | referencia sin refracción |
| `safe-opaque-water.png` | control de oclusión |
| `l02-antes.png` / `l02-despues.png` | la calibración de `L-02` |
| `sombras-*.png` (cuatro) | las plataformas de luces de la Tarea 5.6 |

**Hito 5 cerrado.** Las ocho tareas, las tres correcciones de legibilidad y
los cuatro puntos de la limpieza de cierre: criterio 4 aprobado, guardado de
evidencia fatal, la métrica de `chain_placement` renombrada a lo que de
verdad mide, y los dos PNG redundantes eliminados.

##### Una métrica que decía otra cosa

`chain_placement` reportaba «piezas visibles» y daba `0 de 11` justo donde
el gate contaba `167` píxeles de cadena. No era contradicción: mide si el
segmento del ojo hasta el **centro** de cada pieza está libre, y un centro
puede estar tapado mientras varias caras de la misma pieza se ven. Los
primeros eslabones se ocultan el centro entre ellos, que es precisamente lo
que hace una cadena vista de canto.

Quedó renombrada a **centros de pieza con línea de visión libre**, con la
advertencia de que no es un gate de legibilidad. El gate es el recorrido
rasterizado de `gate_flying_waters`, que cuenta superficie visible píxel por
píxel.

---

## Hito 6 — Picking, paleta y revelación

### La proyección quedó en un solo sitio

El picking usa **la misma función** que el render, y eso ahora es literal:
`Camera::ray_from_screen` es la única implementación de la perspectiva del
proyecto, y `ray_from_pixel` y `ray_from_cursor` la comparten. Sin una
segunda fórmula no hay nada que pueda desviarse.

Difieren en una cosa, a propósito: un píxel es un **área** y el renderer la
representa por su centro; un cursor es un **punto**. Redondear el cursor a
píxel rompería el criterio del plan, porque con `800 × 600` no hay píxel
central y solo una posición continua da el rayo central exacto.

El test que amarra la equivalencia compara las dos rutas en el centro de
varios píxeles, en tres resoluciones incluida una impar, con desvío por
debajo de `1e-6`. Cubre a la vez el aspect ratio y el campo de visión,
porque los dos entran en la misma proyección.

Y una corrección que llegó al cerrar el hito: **el picking no usa la
posición continua del cursor**. Una versión anterior de esta sección afirmó
que el escalado por vecino más cercano preserva las coordenadas
normalizadas, y que por eso daba lo mismo resolver el clic contra el tamaño
de la ventana. Es falso: el escalado **trunca**. Con el perfil interactivo a
`400 × 300` sobre una ventana de `800 × 600`, los píxeles de ventana `2k` y
`2k + 1` muestran los dos el píxel fuente `k`, y el centro de ese píxel está
a media unidad de fuente —un píxel de ventana— de donde está el cursor. Un
clic podía elegir lo que había un píxel al lado de lo que el usuario veía.

La corrección es `input::PresentedFrame`, que guarda **la cámara y la
resolución fuente** con las que se trazó lo que está en pantalla, y trunca
al píxel fuente con `framebuffer::source_pixel`, el mismo mapeo que dibuja.
El rayo del picking pasa a ser, literalmente, el que el renderer trazó para
ese punto de la imagen.

Los tests que lo fijan:

| Test | Qué comprueba |
|---|---|
| `un_bloque_de_dos_por_dos_elige_el_mismo_pixel_fuente` | los cuatro píxeles de ventana de un bloque devuelven el mismo índice |
| `el_bloque_de_dos_por_dos_tambien_comparte_el_rayo` | y el mismo rayo, que es lo que hace que elijan lo mismo |
| `el_rayo_escalado_es_el_que_trazo_el_perfil_interactivo` | y **difiere** del de la ventana: si coincidiera, el tipo no haría falta |
| `el_pixel_fuente_del_picking_es_el_que_dibuja_el_escalado` | contra `blit_upscaled` real, con un patrón de `7 × 5` escalado a `31 × 17` |
| `el_rayo_del_cursor_es_el_mismo_que_traza_el_renderer` | igualdad **exacta**, y desde las cuatro esquinas del píxel, no solo su centro |

El último de la lista dejó de comparar con tolerancia: a resolución completa
las dos rutas llaman a `ray_from_pixel` con los mismos argumentos, así que
la igualdad es exacta y el test lo exige.

### Un `Option` que evita un bug de composición

`Scene::paintable_group` devuelve `None` para las entradas **inertes**.
Sin ese filtro hay un fallo real y difícil de ver: el plinto ocupa toda la
base del diorama y comparte grupo con el Monolito **por tipado**, así que un
clic en la base activaría el finale. El plinto es lienzo y nunca se pinta.

La firma también es lo que hace cumplir el «no pintar por vóxel» del plan:
de un clic se obtiene un grupo, no un objeto, no una cara y no una
coordenada de textura.

### La fase se deriva, y eso tiene un precio declarado

`RevealPhase` no se guarda en ninguna parte. La consecuencia es que un grupo
en cero exacto es **indistinguible** de uno que nadie tocó, así que
`activate` tiene que sacarlo de cero: un empujón de `1e-4`, que a la
velocidad del proyecto son `0.15 ms` de animación. Queda documentado como lo
que es —el precio de no duplicar el estado de activación— y no como un
número mágico.

### La duración se midió otra vez, no se heredó

El `interactive_frame_time` del Hito 3 se midió **antes de la óptica**:

| Perfil `400 × 300` | Tiempo por cuadro |
|---|---:|
| Hito 3, sin óptica | `0.0242 s` |
| Hoy, `reveal 1.0` | **`0.0490 s`** |
| Hoy, `reveal 0.0` | `0.0347 s` |

Y de paso un hallazgo: **el cuadro se encarece durante la propia
transición**. En lienzo los techos del agua están interpolados desde `0/0`,
así que `kl = 1` y no se lanza un solo rayo secundario. La derivación toma
el extremo pintado, que es el caso peor.

> **Corregido en la Tarea 7.1.** La primera frase se sostiene; la última es
> falsa. El extremo pintado **no** es el caso peor: entre los dos extremos
> `resolve` muestrea las dos texturas en vez de una, y los dos estados que
> se midieron aquí son justo los dos que evitan ese coste. El peor cuadro
> es intermedio y cuesta un `7 %` más que el pintado completo.
>
> Y falta lo más grande: esto se midió **solo en la toma hero**. El zoom más
> cercano que el usuario alcanza cuesta `3.6` veces eso, y ahí los quince
> cuadros no caben en el techo de cuatro segundos. Ver «Tres cosas que la
> medición no estaba midiendo».

```text
reveal_duration = clamp(15 x 0.0490, 1.5, 4.0) = 1.5 s   (piso)
reveal_speed    = 0.667 por segundo
```

El crítico está en `0.267 s` por cuadro: **5.4 veces de margen**.
`reveal_duration` devuelve `Result` y no un valor recortado, así que si el
perfil no cupiera la ventana **aborta** con qué hacer, en vez de alargar la
animación, que es lo que el plan prohíbe.

### El terminal que completaba la transición de golpe

El guardián del avance era `delta_seconds > 0.0`, y un `inf` lo pasa: la
transición entera se completaba en un cuadro, que es exactamente el corte
que los quince cuadros existen para evitar. Ahora se exige **finitud**.
Perder un cuadro por un reloj que hipó es mejor que eso.

### El finale arranca solo

Al completarse las tres regiones, el tick activa `Finale`. Es la regla del
inventario, y sin ella el Monolito era **inalcanzable**: nada lo pintaba
nunca. La condición se deriva del propio estado, y `all_regions_painted`
deja fuera al finale para que no se cumpla a sí misma.

### Dirty rendering: sostenido contra instantáneo

Las cuatro fuentes del plan tienen cada una su booleano, y de ese desglose
salió una distinción que no era obvia. Un cambio **sostenido** va a seguir
cambiando el cuadro siguiente —órbita, zoom, transición— y dibuja al perfil
interactivo. Un cambio **instantáneo** ya terminó, así que gastarle un
cuadro de baja resolución no aporta nada: va directo al cuadro final.

La política salió del ciclo a `renderer::plan_frame`, con su tabla de tres
filas probada, incluida la secuencia completa de una interacción: dos
cuadros moviéndose, uno final, y reposo.

Y el descanso de `16 ms` se ejecutaba **todos** los cuadros, también durante
la animación: a `0.0490 s` por cuadro eso bajaba la transición de treinta
cuadros a veintitrés. Ahora duerme solo en reposo.

### El encuadre hero vive en la escena

`Blockout::hero_preset` se **deriva** de `hero_camera`, no en paralelo, así
que las dos no pueden discrepar. Los tres puntos salen de las anclas
medidas. La parte de producción de `input.rs` no menciona `Vec3` ni una sola
vez: cero constantes de cámara, que es lo que el plan exigía.

`CameraPreset` lleva el **encuadre** y nada más. Los límites de radio son de
la escena medida y el FOV es del proyecto, así que `restore` no los toca.

`DemoAction` declara la superficie de teclado completa en un sitio probable
sin ventana, y `ResetCamera` **no transporta el encuadre**: solo dice
«restaurá». Hay un test que vigila el tamaño del enum para que nadie le
cuelgue un preset y acabe escribiendo el blueprint dentro de `input`.

### Gate del Hito 6

«Demo completa desde lienzo hasta Monolito final sin recompilar».

El criterio «sin recompilar» es una afirmación sobre el **estado en tiempo de
ejecución**, y como tal se puede verificar sin ventana:
`tests/demo_completa.rs` recorre el trayecto entero por la API pública —la
misma que usa la ventana— simulando el reloj.

| Comprobación | Resultado |
|---|---|
| Lienzo → tres regiones → Monolito | **Pasa** |
| Cada región tarda la duración derivada | `1.5 s`, ±2 cuadros |
| Cada región usa al menos quince cuadros | **31 cuadros** |
| La demo se puede dar **dos veces** seguidas con `L` | **Pasa** |
| Las tres regiones pueden revelarse en paralelo | dos duraciones en total |
| El ratón alcanza las tres regiones desde la hero | **Pasa** |
| Lienzo y pintado difieren en la mayoría del diorama | **Pasa** |
| Ningún píxel del estado final en negro absoluto | **Pasa** |

Los `31` cuadros por región son el doble del mínimo exigido.

#### Línea de tiempo, en imágenes

`cargo run --release --example demo_timeline` reproduce la presentación con
el mismo reloj y la misma velocidad derivada que la ventana, y guarda un
render en cada hito:

| Archivo | Momento |
|---|---|
| `evidence/hito6/0-lienzo.png` | el diorama sin pintar |
| `1-praderas-a-medias.png` | cuadro 15 de la primera transición |
| `2-praderas.png` | Praderas listas |
| `3-rompeolas.png` | Rompeolas listo |
| `4-aguas.png` | Aguas Voladoras lista |
| `6-monolito.png` | el Monolito, que arrancó solo |

#### Lo que el recorrido humano añade

Dos cosas que ningún test alcanza, y que son la revisión visual pendiente:

- Que el **ratón apunte donde el usuario cree**. La equivalencia con el
  render está probada, pero que el clic caiga sobre la región que se
  pretendía es percepción.
- Que la transición **se lea como animación**. Los quince cuadros son el
  criterio, y treinta y uno lo duplican, pero si se lee fluido lo dice el
  ojo.

Lista para ese recorrido: abrir la ventana, pulsar `1`, `2` y `3` viendo
cada transición; comprobar que el Monolito se pinta solo al terminar la
tercera; pulsar `L` y repetirlo todo; orbitar hasta perder el encuadre y
pulsar `R`; y hacer clic directamente sobre las praderas, el acantilado y la
bahía.

De la auditoría se añadió una sola comprobación más: hacer clic **sobre el
Monolito**, que no debe pintar nada. La carrera entre `L` y una selección
del mismo cuadro no entra en la lista manual —ver la decisión cerrada sobre
el despintado por región—.

#### Hito 6 cerrado

El recorrido se ejecutó y aprobó: las tres regiones se pintan por teclado y
por clic, el Monolito arranca solo al completarse la tercera, `L` permite
repetir la demostración, `R` recupera el encuadre, y un clic sobre el
Monolito no pinta nada.

Con eso quedan cerradas las cinco tareas del hito, los tres bloqueos de
integración que encontró la auditoría, los seis textos obsoletos y el
recorrido humano. La demo va de lienzo a Monolito final sin recompilar.

---

#### Auditoría del gate y correcciones

El gate no cerró en el primer intento. Una revisión externa encontró tres
bloqueos de integración y varios claims sobredichos; todos quedaron
corregidos, y los que eran bugs llevan test.

##### Bloqueo 1 · el picking podía elegir el finale

`paintable_group` filtraba las entradas inertes pero no el grupo `Finale`,
así que un clic sobre el Monolito —que **es** revelable— lo adelantaba. Con
eso se perdía el clímax del diorama y la condición que lo gobierna quedaba
en manos de dónde apuntara el puntero.

La corrección es **defensa en dos capas**:

1. `Scene::paintable_group` rechaza `Finale`. El ratón solo alcanza las tres
   regiones.
2. `RevealState::activate` rechaza `Finale` mientras las tres regiones no
   estén pintadas. Una regla que solo vive en la política de entrada se la
   salta el siguiente llamador, y esta es la invariante del inventario.

El autoarranque no se ve afectado: `advance` llama a `activate(Finale)`
justo después de comprobar `all_regions_painted`.

Los tests nuevos usan **conjuntos** y no tres `contains`. La diferencia
importa: tres comprobaciones de pertenencia habrían pasado igual con
`Finale` en la lista.

| Test | Qué fija |
|---|---|
| `los_grupos_alcanzables_por_raton_son_exactamente_las_tres_regiones` | igualdad de conjuntos sobre el cuadro completo |
| `el_monolito_ocupa_pixeles_y_aun_asi_no_es_seleccionable` | que el anterior no pase por vacío: hay más de 20 muestras que dan en el finale |
| `activate_del_finale_falla_hasta_que_las_tres_regiones_esten_pintadas` | la segunda capa, con cero, una y dos regiones |
| `una_region_a_medias_no_habilita_el_finale` | pintadas, no en curso |
| `un_clic_sobre_algo_del_finale_no_selecciona_nada` | con un objeto revelable del finale, no inerte |

##### Bloqueo 2 · `L` reiniciaba y volvía a pintar en el mismo cuadro

El ciclo guardaba la selección en un solo `Option`, y `L` reemplazaba el
estado **antes** de que esa selección se aplicara. Resultado: la consola
decía «reiniciado al lienzo» y el estado tenía una región revelándose.

El mismo `Option` hacía que varias selecciones del mismo cuadro —un clic y
una tecla, o `1`, `2` y `3` en el mismo sondeo— se pisaran y solo
sobreviviera la última.

La corrección es `input::FrameIntent`, un reducer de las acciones del cuadro
que se prueba **sin abrir una ventana**: recoge todo, reduce, y el ciclo
aplica después. `ResetCanvas` domina sobre `Paint`, `ResetCamera` convive
con todo, y los `Paint` distintos se conservan sin duplicar.

El test que afirmaba probar la revelación paralela activaba tres grupos
directamente y no atravesaba esta integración; ahora la política tiene sus
seis tests propios.

##### Bloqueo 3 · un tirón finito convertía la transición en corte

`advance` rechazaba `NaN` e infinito, pero aceptaba cualquier delta finito.
Si la ventana se suspende, se arrastra o se bloquea más de la duración de la
revelación, el cuadro siguiente trae un delta perfectamente válido que lleva
el progreso de casi cero a uno **de golpe**: exactamente el corte que los
quince cuadros existen para evitar. La medición de rendimiento protege el
caso normal, no los tirones.

`MAX_PROGRESS_PER_TICK = 1 / 15` es el paso de un cuadro del criterio. Un
tirón deja de completar la transición y solo consume un cuadro.

Lo que se sacrifica queda declarado: la sincronía estricta con el reloj de
pared **durante una suspensión**. Tras un bloqueo largo la transición
termina más tarde de lo que diría el reloj, y es la elección deliberada,
porque la alternativa es que la animación haya ocurrido mientras la ventana
estaba congelada. Lo que el criterio protege es que se vea.

El paso normal es `0.0286`, muy por debajo del tope de `0.0667`: el caso
corriente no lo toca, y hay un test que lo comprueba.

##### El picking apuntaba a un encuadre no presentado

El clic se resolvía contra la cámara **actual**, y las teclas de órbita se
procesan antes en el mismo cuadro. Sosteniendo una flecha mientras se
pincha, la cámara ya había rotado y el rayo apuntaba a una escena que
todavía no se había mostrado. La diferencia es de un cuadro, y es justo el
cuadro en el que el usuario decidió dónde pinchar.

Ahora el ciclo guarda `camara_presentada` —la cámara con la que se dibujó lo
que está en pantalla— y el picking usa esa. `Camera` pasó a `Copy` para
poder hacerlo sin razonar sobre préstamos en el camino de la entrada.

##### La cifra de la duración ya no se hereda

`0.0490` estaba escrito en cuatro sitios y de él dependía `reveal_duration`.
Los tests probaban la aritmética **alrededor** de esa cifra; ninguno probaba
que siguiera siendo cierta. Y una cifra escrita al compilar es la de la
máquina de quien la escribió: en un equipo más lento la animación se
quedaría corta de cuadros sin que nada avisara, y el gate de fluidez no
llegaría a dispararse nunca.

Dos cambios:

- **La ventana se autocalibra.** Traza tres cuadros al arrancar —el primero
  paga el calentamiento de cachés— y deriva la duración de la mediana. El
  gate de fluidez pasa a evaluarse en la máquina que corre.
- **Existe un probe que rederiva la cifra registrada**:
  `cargo run --release --example interactive_frame_time`.

`interactive_probe` no servía: usa `InteriorVisible` en vez del preset
refractivo, mide una sola pasada en vez de una distribución, y no separa el
lienzo del estado pintado.

Medición registrada, con su procedencia:

| | |
|---|---|
| Perfil | `400 × 300`, `InteractiveProfile::MEDIA` |
| Preset | `safe-refractive-water`, 160 primitivas, 8 texturas |
| Repeticiones | 15 por estado |
| `reveal 0.0` | mín `0.0350`, mediana `0.0363`, máx `0.0397` |
| `reveal 1.0` | mín `0.0501`, mediana **`0.0524`**, máx `0.0703` |
| `interactive_frame_time` | `0.0524 s`, la peor de las dos medianas |
| `reveal_duration` | `1.50 s` (piso) |
| Cuadros de transición | `28.6` |
| Margen al crítico | `5.1×` |
| Commit | `6402f3f` |
| Fecha | 3 de septiembre de 2026 |
| Hardware | AMD Ryzen 7 6800H |
| Toolchain | `rustc 1.97.0` |

Se toma el **peor** de los dos estados: el lienzo no lanza rayos secundarios
y sale más barato, así que garantizar los quince cuadros con su tiempo
dejaría el final de la transición sin margen.

##### Un tiempo por cuadro nulo o negativo ahora es error

`reveal_duration(0.0)` y `reveal_duration(-1.0)` devolvían `Ok(1.5)`, con el
argumento de que «no hay quince cuadros que garantizar si el cuadro no
cuesta nada». El argumento estaba mal: un cuadro no cuesta cero segundos
nunca, así que un cero es una medición **inválida**, y una medición inválida
que produce una duración de aspecto razonable es peor que una que falla.

##### Claims corregidos

**`demo_timeline` no es end-to-end.** Su descripción decía «simula el
recorrido de la presentación», y no lo hace: no pasa por `demo_action` ni
por `pick_region`, no procesa eventos de `minifb`, no ejerce el antirrebote
del botón, no usa `plan_frame` ni el perfil interactivo —renderiza siempre a
`800 × 600`— y no prueba `L` ni `R`. Se redescribió como lo que es: **un
generador de estados visuales representativos de la línea temporal**.

**«Ningún píxel negro» estaba sobredicho.** El test usa la escena sin
assets, submuestrea cada 23 píxeles y solo considera rayos que impactan
geometría. La afirmación correcta es que **ninguna muestra geométrica del
subconjunto evaluado** queda en negro. El recuento exhaustivo del cuadro
completo vive en `examples/gate_flying_waters`.

**Faltaba el intermedio del Monolito.** La secuencia demostraba extremos
coherentes pero no que el finale se interpola. Se añadió
`5-monolito-a-medias.png`: el Monolito en marfil pálido a medio camino del
cristal, con las tres regiones ya pintadas.

##### Hallazgos menores documentados

- **Identidad por índice.** `Hit::object_index` es la posición en
  `Scene::objects`, y `SceneAccel` guarda esas mismas posiciones. Insertar,
  borrar u ordenar tras construir la aceleración las desincronizaría en
  silencio. La invariante se sostiene por construcción —todo el llenado
  ocurre antes de `build`— y ahora está escrita en el doc de `Scene`, junto
  con la corrección estructural que haría falta si alguna vez hay que mutar
  la escena en caliente.
- **Empates geométricos.** Dos superficies a distancia exacta dejan ganar al
  primero recorrido, por el orden de `SpatialGroupId::ALL` y de inserción.
  Es determinista y estable entre corridas, que es lo que el proyecto
  necesita; no es geométricamente significativo. Queda como política escrita
  en `accel`.
- **Dimensiones cero.** Las funciones públicas de cámara dividen por
  `width` y `height`. `ray_under_cursor` lo cubre rechazando todo cursor con
  rango vacío, y la precondición quedó documentada.

#### Decisión cerrada — no hay despintado por región

`L` devuelve los **cuatro** grupos al lienzo de golpe. No existe una tecla
ni una función para despintar una región suelta, y no se añade.

La razón no es de esfuerzo —sería `set_progress(grupo, 0.0)` y una tecla—
sino de estado: despintar una región **después** del finale dejaría al
Monolito pintado sobre un Continente incompleto, que contradice la regla del
inventario. Resolverlo obligaría a decidir si despintar una región despinta
también el finale, y con eso a inventar un estado que la narrativa de la
obra no contempla: el arco va de lienzo a Monolito, y `L` ya permite
repetirlo entero.

Consecuencia para el recorrido de verificación: la comprobación de «`L` y
una región en el mismo cuadro» **sale de la lista manual**. Los dos pulsos
tendrían que caer en el mismo sondeo de unos `50 ms`, así que no se puede
provocar a mano con fiabilidad. La cubre `FrameIntent` con un test
determinista, en los dos órdenes de sondeo.

#### Seis textos obsoletos, corregidos

Al cerrar el hito quedaban afirmaciones que el propio avance del proyecto
había dejado falsas. No son cosméticas: un comentario que miente sobre una
medición es peor que no tener comentario, porque el siguiente lector lo
cree.

| Dónde | Decía | Ahora |
|---|---|---|
| `camera::ray_from_cursor` | que daba lo mismo el tamaño de ventana o de perfil | que **no** lo da, y que para picking hay que usar `PresentedFrame` |
| `main.rs`, carga de escena | que el volumen no se inserta «hasta que el Hito 5 traiga refracción» | el preset refractivo, que es el canónico desde la 5.4 |
| `main.rs`, perfil | `~0.096 s` a `800 × 600` | `~0.20 s`, remedido con óptica |
| `InteractiveProfile` | `0.0956 s` a resolución completa, de la Tarea 3.8 | `0.2005 s`, con la nota de que las de la 3.8 eran de antes de la refracción |
| `InteractiveProfile::MEDIA` y `BAJA` | `0.0242 s` y `0.0157 s` | `0.0518 s` y `0.0340 s`, con procedencia y el comando que las rederiva |
| `RevealState` y `optics` | que la fase, el avance y la recursión «llegan» en tareas futuras | que ya están, y dónde |

Las tres cifras nuevas del perfil se midieron juntas: preset
`safe-refractive-water`, `reveal 1.0`, mediana de quince repeticiones,
commit `afa92c2`, 3 de septiembre de 2026, Ryzen 7 6800H, rustc 1.97.0.

#### Gates registrados

```text
cargo fmt -- --check                        OK
cargo clippy --all-targets -- -D warnings   0 avisos
cargo test                                  380 tests, 0 fallos
cargo build --release                       OK
cargo run                                   autocalibra y anuncia los controles
```

Reparto de los 380: `344` de librería, `16` del generador de assets, `8` de
humo del render, `6` de sombras submarinas y `6` de la demo completa.

---

## Hito 7 — Reserva técnica

### Tarea 7.1 — Tres cosas que la medición no estaba midiendo

La revelación se anima con quince cuadros como mínimo, y esos quince salen
de dividir una duración por un tiempo por cuadro **medido**. La medición que
había medía dos estados —`reveal 0.0` y `reveal 1.0`— en la toma hero, y se
quedaba con el peor de los dos. Le faltaban tres cosas, y la tercera cambia
la conclusión del hito.

#### 1 · El estado intermedio

```rust
// reveal::resolve
let albedo = if t <= 0.0 {
    scene.albedo_at(&inicial, uv)          // un muestreo
} else if t >= 1.0 {
    scene.albedo_at(&final_, uv)           // un muestreo
} else {
    let a = scene.albedo_at(&inicial, uv); // dos, y una mezcla
    let b = scene.albedo_at(&final_, uv);
    a * (1.0 - t) + b * t
};
```

Los dos atajos existen porque los extremos son el caso común. Y los dos
estados que se medían eran, exactamente, los dos que toman un atajo.

El barrido de diecisiete estados resuelve **tres bandas**, no un máximo:

| Banda | Coste | Qué la separa de la anterior |
|---|---:|---|
| lienzo | `1.00x` | — |
| regiones reveladas | `1.30x .. 1.44x` | los techos ópticos: sin ellos no hay rayos secundarios |
| `Finale` revelado | `1.41x .. 1.58x` | veinte primitivas más que dejan de ser lienzo, y casi todo el cuadro |
| todo pintado | `1.47x` | — (queda **dentro** de la banda alta, no encima) |

Dentro de cada banda el progreso apenas mueve el coste. El escalón entre
bandas es el hallazgo, y el estado más caro —`finale 0.50`, `0.0735 s`—
cuesta un `7 %` más que el pintado completo y un `58 %` más que el lienzo.

#### 2 · `Finale` no es solo el Monolito

Buscando el peor cuadro empecé por las tres regiones a la vez —tres teclas
seguidas— y resultó no ser el peor. El peor es el del grupo `Finale`, y la
razón está en `scenes::continent::globales`: `RevealGroup::Finale` cubre las
diez masas del Monolito **y las diez del arco costero**. Juntas ocupan casi
todo el cuadro de la toma hero.

Mientras se barren las regiones ese grupo sigue en lienzo, y se lleva su
parte del cuadro sin pagar un solo rayo secundario. Por eso el barrido de
las regiones se queda corto, y por eso el peor cuadro de la demo es el
**clímax**: el Continente pintado y el Monolito revelándose sobre él. Toda
demostración completa pasa por ahí.

#### 3 · La cámara, que es la que manda

Las dos primeras correcciones mueven el coste un `7 %`. La tercera lo
multiplica por tres y medio.

`worst_case()` medido en las **diez cámaras alcanzables** —la hero, la
órbita completa en pasos de `45°` y los dos extremos del zoom—:

| Cámara | Mediana | vs hero (pareado) | 2os rayos |
|---|---:|---:|---:|
| hero | `0.0739` | `1.00x` | `22 512` |
| `yaw +45` | `0.0770` | `1.04x` | `23 557` |
| `yaw +315` | `0.0800` | `1.03x` | `24 390` |
| `yaw +90` | `0.0621` | `0.85x` | `17 848` |
| `yaw +135` | `0.0510` | `0.65x` | `8 535` |
| `yaw +180` | `0.0482` | `0.66x` | `10 550` |
| `yaw +225` | `0.0686` | `0.87x` | `20 218` |
| `yaw +270` | `0.0664` | `0.84x` | `19 750` |
| **`zoom cerca`** | **`0.2721`** | **`3.6x`** | **`107 084`** |
| `zoom lejos` | `0.0398` | `0.54x` | `6 871` |

El zoom más cercano que el usuario puede alcanzar con dos teclas cuesta
`3.6` veces la toma hero, y los conteos dicen por qué: acercarse llena la
pantalla de bahía refractiva y los rayos secundarios pasan de veintidós mil
a ciento siete mil.

**Y ahí el gate de fluidez falla.** Quince cuadros a `0.2721 s` exigen
`4.08 s` y el techo son `4.0 s`. No es un margen estrecho: es el otro lado
del límite.

| Corrida | `zoom cerca` | 15 cuadros | Gate |
|---|---:|---:|---|
| 1 | `0.2496` | `3.74 s` | pasa |
| 2 | `0.2718` | `4.08 s` | **falla** |
| 3 | `0.2901` | `4.35 s` | **falla** |
| 4 | `0.2530` | `3.80 s` | pasa |
| 5 | `0.2648` | `3.97 s` | pasa |
| 6 | `0.2721` | `4.08 s` | **falla** |

Falla en la mitad de las corridas. Un veredicto que oscila con el estado
térmico de la máquina no es «pasa»: es un margen que no existe.

### Tarea 7.1 — Seis correcciones de instrumento

La primera versión de esta tarea llegó a la conclusión contraria —`3.5x` de
reserva— y no por mala suerte: por seis defectos del instrumento, todos en
la misma dirección.

| Defecto | Por qué sesga | Corrección |
|---|---|---|
| Orden fijo dentro de la ronda | la primera fila paga el arranque de **cada** ronda; favorece sistemáticamente a las primeras | el orden rota: en la ronda `r` empieza el punto `r % n` |
| `ordenadas[n / 2]` con `n` par | no es la mediana, es el mayor de los dos centrales: sesgo pequeño y siempre hacia arriba | `stats::summarize`, y quince rondas, que es impar |
| Cociente de medianas | las dos medianas pueden salir de rondas distintas, y un tirón en una las descoloca | `stats::median_ratio`: cociente ronda contra ronda |
| Una sola cámara | el encuadre resultó ser el término dominante, y estaba fuera de la medición | `Blockout::measurement_cameras`, diez encuadres alcanzables |
| Procedencia por bloque | un solo párrafo cubría corridas distintas de builds distintos | cada tabla dice su corrida, su comando y que el árbol está sin commitear |
| Atribución por el nombre de la fila | «las texturas del estado pintado» era una hipótesis contada como medición | columna de rayos secundarios en cada celda |

La primera y la segunda las pidió la revisión; la tercera también. La
cuarta es la que cambió el resultado.

Sobre la **atribución**: el primer escalón de la matriz no es «las
texturas». Al pasar de lienzo a pintado se encienden a la vez las texturas y
los techos ópticos de los materiales finales, y los conteos dicen cuál pesa:
los rayos secundarios por cuadro pasan de `986` a `13 809`. Es la óptica. El
único escalón que sí es muestreo de textura es el último, que añade `262`
rayos y un `4 %` a `9 %` de tiempo.

Y una afirmación que había que retirar entera: escribí que «la primitiva
número ciento sesenta cuesta más que las ciento cincuenta y nueve anteriores
juntas». Es falsa por aritmética —el escalón mide un `23 %` a `26 %`, no un
`100 %`— y confundía el conteo con lo que el volumen enciende.

#### Lo que el instrumento sigue sin poder decir

Cuál es el **máximo global**. Dentro de la banda alta las diferencias son
menores que la dispersión de la máquina: seis corridas pusieron el máximo en
`finale 0.98`, `0.75`, `0.98`, `0.90`, `0.98` y `0.50`, con medianas
separadas menos de un `5 %`. Por eso `RevealState::worst_case()` es un
**representante** de la banda alta, elegido por la estructura de la escena
—el único estado alcanzable en que nada queda en lienzo y un grupo grande
está a medio muestrear—, y no un máximo demostrado. El aviso del ejemplo
tolera un `15 %` antes de sugerir mover la constante, que es lo que se movió
entre corridas.

Como control de reproducibilidad, el estado de calibración se mide en las
dos fases —es `finale 0.50` en la primera y `hero` en la segunda—. Las dos
medidas quedaron a `+1.1 %`, lo que dice que el instrumento ya es consistente
dentro de una corrida aunque no lo sea entre ellas.

### Tarea 7.1 — La matriz

> **Superada por «La mitigación, medida y aplicada».** Las cifras de abajo
> son de antes de recortar el zoom y bajar el perfil, y de una rejilla de
> diez cámaras que no cruzaba la elevación. Se conservan porque son el
> punto de partida contra el que se lee la mitigación; para la matriz
> vigente, ir a la sección final.

El plan nombra cinco presets sin definirlos. Cada fila declara su definición
en las dos dimensiones que mueven el coste:

| Nombre del plan | Volumen | Revelación |
|---|---|---|
| `safe-canvas` | sin volumen, `159` | lienzo |
| `safe-painted` | sin volumen, `159` | pintado |
| `safe-water` | refractivo, `160` | pintado |
| `safe-revealing` | refractivo, `160` | `worst_case()` |
| `target-water` | nivel objetivo | pintado |

Están ordenadas para que cada fila añada **un** cambio sobre la anterior.
Release, quince rondas intercaladas y rotadas, toma hero:

| Preset | Prim. | `800 × 600` | `400 × 300` | 2os rayos |
|---|---:|---:|---:|---:|
| `safe-canvas` | 159 | `0.1815` | `0.0512` | `986` |
| `safe-painted` | 159 | `0.2155` | `0.0603` | `13 809` |
| `safe-water` | 160 | `0.2634` | `0.0755` | `22 250` |
| `safe-revealing` | 160 | `0.2759` | `0.0804` | `22 512` |
| `target-water` | — | Pendiente | Pendiente | — |

`target-water` no se mide porque **el nivel objetivo no existe**: es la
Tarea 7.2, y decidir si se construye es justo para lo que sirve esta matriz.
La fila se imprime como pendiente en vez de omitirse, para que se vea que
falta y no que no hacía falta.

Los escalones, con cociente pareado y el cambio de rayos al lado:

| Escalón | `400 × 300` | `800 × 600` | 2os rayos |
|---|---:|---:|---:|
| lienzo → materiales pintados | `+17.1 %` | `+17.5 %` | `+12 823` |
| volumen refractivo | `+26.4 %` | `+22.8 %` | `+8 441` |
| doble muestreo de la transición | `+8.8 %` | `+4.5 %` | `+262` |

Los conteos de rayos son lo que convierte la tabla en una explicación: los
dos primeros escalones compran rayos y el tercero no compra ninguno.

### Tarea 7.1 — El presupuesto, y lo que autoriza

> **Superada por «La mitigación, medida y aplicada».** El presupuesto de
> abajo se midió sin el eje de elevación, y por eso su peor encuadre no era
> el peor: con la rejilla completa el gate fallaba por un `63 %`, no por un
> `2 %`. La reserva vigente es `2.1x`, después de la mitigación.

El único gate que puede fallar es el de fluidez, y solo mira el perfil
interactivo: el cuadro final se produce una vez al soltar los controles y
nadie lo anima. El crítico es `4.0 s / 15 = 0.2667 s` por cuadro.

| Perfil | Encuadre | Peor cuadro | 15 cuadros | Reserva | Gate |
|---|---|---:|---:|---:|---|
| `MEDIA` `400 × 300` | hero | `0.0804` | `1.21 s` | `3.3x` | pasa |
| `MEDIA` `400 × 300` | `zoom cerca` | `0.2721` | `4.08 s` | `1.0x` | **falla** |
| `BAJA` `320 × 240` | hero | `0.0512` | `0.77 s` | `5.2x` | pasa |
| `BAJA` `320 × 240` | `zoom cerca` | `0.1843` | `2.76 s` | `1.4x` | pasa |

**La reserva de `3.3x` es real y es solo del encuadre hero.** En el peor
encuadre alcanzable con el perfil por defecto no hay reserva ninguna: el
gate falla en la mitad de las corridas. La cifra de `3.5x` que registró la
primera versión de esta tarea era eso —la reserva de un encuadre—
presentada como la reserva del proyecto.

Tres cosas que la matriz obliga a escribir junto a las cifras:

1. **La reserva está en tiempo, no en primitivas.** El coste no es lineal en
   el conteo: la jerarquía poda el `92 %` de los tests. Traducir tiempo a
   densidad exige medir el nivel objetivo, no dividir.
2. **Lo que se compra con densidad son rayos.** El `zoom cerca` cuesta `3.6`
   veces la hero con la **misma** geometría, solo por poner más bahía en
   pantalla. Cualquier detalle dentro de la bahía se paga a través de los
   rayos refractados, y ese es el sitio donde el presupuesto ya está agotado.
3. **`BAJA` es hoy la única opción que aguanta el peor encuadre**, y con
   `1.4x`, que tampoco es holgura. Bajar el perfil por defecto o recortar el
   rango de zoom son decisiones de presentación —cambian lo que el usuario
   ve—, así que quedan **planteadas y no aplicadas**.

#### Qué queda autorizado

No la densidad objetivo completa. Con el perfil actual el margen en el peor
encuadre es cero, así que añadir geometría dentro de la bahía empeoraría un
gate que ya falla la mitad de las veces.

Sí queda viable **probar densidad en Aguas de forma incremental**, y con una
condición previa que la propia medición señala: resolver el perfil o el
rango de zoom. Con `BAJA` hay `1.4x` en el peor encuadre, que da para lotes
pequeños con remedición entre lotes. Sin resolverlo, el primer lote se mide
contra un gate que ya no se cumple.

#### Qué cambió en el código

| Dónde | Cambio |
|---|---|
| `src/stats.rs` | nuevo: `summarize` y `median_ratio`, con siete tests |
| `Blockout::measurement_cameras` | nuevo: las diez cámaras alcanzables, con dos tests |
| `reveal::WORST_CASE_PROGRESS` | nueva: el progreso del `Finale` en el peor estado |
| `RevealState::worst_case()` | nuevo: `painted()` con el `Finale` a medio revelar |
| `main.rs`, autocalibración | calibra con `worst_case()` y ya no con `painted()` |
| `examples/interactive_frame_time.rs` | tres fases: estados, cámaras y el cruce de perfiles |
| `examples/performance_matrix.rs` | nuevo: la matriz, con conteos de rayos |
| `tests/demo_completa.rs`, `examples/demo_timeline.rs` | `FRAME_TIME`: `0.0524` → `0.0820` → `0.0736`, ahora con el encuadre declarado |

Cinco tests amarran lo que no debe volver a moverse solo, y ninguno mide
tiempo:

| Test | Qué comprueba |
|---|---|
| `el_peor_estado_no_tiene_nada_en_lienzo` | ningún grupo en `Unpainted`: donde hay lienzo no hay rayos secundarios |
| `el_peor_estado_tiene_el_finale_a_medio_revelar` | `Finale` en `Revealing` y las regiones en `Painted`, que es lo único que permite el doble muestreo |
| `el_peor_estado_es_alcanzable_por_la_demo` | se llega a él con `activate` y `advance`, sin tocar `set_progress` |
| `las_camaras_de_medicion_cubren_la_orbita_y_los_dos_extremos_del_zoom` | diez cámaras, ocho a radio orbital y dos pegadas a los límites medidos |
| `las_camaras_de_medicion_miran_todas_al_mismo_encuadre` | comparar cámaras no puede estar comparando también dos `look_at` |

Y en `stats`, los dos que fijan la estadística: que la mediana par promedia
los dos centrales, y que el cociente pareado cancela una deriva común que el
cociente de medianas no cancela.

#### Procedencia

Las dos tablas del barrido y las tres de la matriz salen de **dos corridas**
del 4 de septiembre de 2026, una por ejemplo, sobre el mismo binario:

```text
cargo run --release --example interactive_frame_time    (fases 1, 2 y 3)
cargo run --release --example performance_matrix        (la matriz)
```

Árbol de trabajo **sin commitear** sobre `2c7960a`; Ryzen 7 6800H; rustc
1.97.0; release. La tabla de las seis corridas de `zoom cerca` recoge las
corridas de desarrollo del mismo día, sobre el mismo árbol salvo las
correcciones del instrumento que se fueron aplicando: las tres primeras son
de antes de la rotación del orden.

Cuando este árbol se commitee, el hash de esta sección hay que sustituirlo
por el del commit que lo contenga. Citar un commit que no contiene las
cifras es lo que hacía la primera versión de esta sección.

Los valores absolutos siguen sin ser comparables entre bloques: cada bloque
deja la máquina más caliente para el siguiente, y el suelo del lienzo se
movió un `16 %` entre corridas del mismo día. Lo comparable es lo de dentro
de un bloque, que es donde está el intercalado y la rotación.

#### Gates registrados

```text
cargo fmt -- --check                        OK
cargo clippy --all-targets -- -D warnings   0 avisos
cargo test                                  392 tests, 0 fallos
cargo build --release                       OK
cargo run --release --example performance_matrix   codigo 1: el gate falla
```

Reparto de los 392: `356` de librería, `16` del generador de assets, `8` de
humo del render, `6` de sombras submarinas y `6` de la demo completa.

Que la matriz salga con código distinto de cero es deliberado: el gate de
fluidez falla en el peor encuadre alcanzable, y un generador de evidencia
que termina en éxito mientras un gate falla es exactamente el fallo
silencioso que el Hito 5 corrigió.

---

### Tarea 7.1 — La mitigación, medida y aplicada

La revisión pidió cerrar el hueco antes de tocar una sola primitiva, y
empezó por un claim mío que era falso.

#### La elevación sí era alcanzable

Escribí en `measurement_cameras` que «la elevación no se barre: la ventana no
la expone». `main.rs` mapea `Key::Up` y `Key::Down` a
`Camera::orbit(0, ∓ROTATION_SPEED)`, con el pitch recortado a `±84.3°`. La
elevación es tan alcanzable como el yaw, y mirar el diorama desde arriba es
justo lo que pone la bahía refractiva entera en pantalla.

La rejilla pasó de diez cámaras a **cuarenta y ocho**: cuatro cuadrantes de
yaw × cuatro elevaciones × tres radios. Y el eje que faltaba escondía un caso
mucho peor que el que había encontrado:

| Encuadre | Mediana | vs hero |
|---|---:|---:|
| `y+0 e+35 cerca` (el «zoom cerca» de antes) | `0.4342` | `3.56x` |
| `y+0 e+84 cerca` | `0.4018` | `3.22x` |
| `y+90 e+84 cerca` | `0.4014` | `3.02x` |
| hero | `0.1241` | `1.00x` |

Quince cuadros a `0.4342 s` exigen **`6.51 s`** contra un techo de `4.0 s`. No
era un margen estrecho: el gate fallaba por un `63 %`.

Un detalle que sirve de control: `y+0 e-84 cerca` —mirando desde debajo del
plinto— traza **cero** rayos secundarios. La bahía no se ve desde ahí, y el
coste lo dice.

#### Las dos palancas, medidas juntas

El gate se corrige bajando la resolución del perfil o recortando el zoom, y
las dos son decisiones de presentación. Medirlas por separado habría dado dos
respuestas parciales, así que se cruzaron sobre la dirección más cara:

| Radio mínimo | `MEDIA` `400 × 300` | `BAJA` `320 × 240` |
|---|---:|---:|
| `1.2 S` | `0.3994` — **falla** | `0.2467` — `1.08x` |
| `1.7 S` | `0.1867` — `1.43x` | `0.1336` — `2.00x` |
| **`1.8 S`** | `0.1807` — `1.48x` | **`0.1124` — `2.34x`** |
| `1.9 S` | `0.1598` — `1.67x` | `0.1063` — `2.51x` |

El margen se exige contra el crítico de `0.2667 s`, y el umbral para
recomendar es `1.30x`. Sale de la dispersión de la propia máquina: entre
corridas del mismo día la mediana del peor encuadre se movió un `16 %`, así
que una combinación que solo pasara por un `10 %` estaría dentro del ruido.
`BAJA · 1.2 S` pasa por `1.08x`, que es exactamente eso.

**Aplicado: `InteractiveProfile::default() = BAJA` y `MIN_RADIUS_FACTOR =
1.8`.** `MEDIA · 1.8 S` también cumple, con `1.48x`; se eligió `BAJA` porque
`2.34x` sobrevive a un día malo y `1.48x` no necesariamente. Lo que se paga es
resolución **mientras algo se mueve**: al soltar los controles el cuadro final
sigue siendo `800 × 600`.

La comparación visual está en `evidence/hito7/`, generada con
`cargo run --release --example profile_preview`: los dos perfiles ya
**ampliados** con el mismo `blit_upscaled` que dibuja la ventana, junto al
cuadro final de referencia, en la toma hero y en el peor encuadre. Ampliados y
no a su tamaño a propósito: un PNG de `320 × 240` visto al `100 %` parecería
más nítido que el de `800 × 600`, que es lo contrario de lo que pasa en
pantalla.

#### La ventana calibraba en el encuadre equivocado

Con la mitigación aplicada el banco pasaba, y el programa seguía sin cumplir.
`main.rs` calibraba con `RevealState::worst_case()` en la **toma hero**, que
da `1.5 s` de duración; si el usuario se acercaba durante la transición, el
cuadro subía a `0.126 s` y la animación entregaba doce cuadros. El gate se
cumplía en la medición y no en la obra.

La duración se fija **una sola vez** al arrancar, así que tiene que salir del
peor encuadre alcanzable y no del que se presenta. `Blockout::worst_case_camera()`
—cenital, pegada al radio mínimo— es con la que calibra ahora. Cuesta `0.4 s`
de arranque y a cambio los quince cuadros se cumplen en cualquier sitio al que
el usuario pueda llegar. Es el mismo intercambio que ya hacía
`RevealState::worst_case` con el estado.

Y eso destapó un fallo de contabilidad en `tests/demo_completa.rs`: metía un
`advance` extra «para activar el Monolito» y empezaba a contar después, así
que el Monolito recibía sus quince cuadros y el contador veía catorce. No se
notó mientras `reveal_duration` estuvo pegada a su piso de `1.5 s`, que daba
veinte cuadros y de sobra. Al calibrar con el peor encuadre la duración dejó
de estar en el piso, el margen pasó a ser exactamente cero y el cuadro perdido
apareció como un fallo. El `advance` extra sobraba: la activación del finale
cae **dentro** de la llamada que completa la última región.

#### Las otras tres correcciones de instrumento

| Qué | Antes | Ahora |
|---|---|---|
| Selección del peor punto | por el mínimo | por la **mediana**: el mínimo es el mejor cuadro que se llegó a ver, y de un presupuesto interesa el coste típico |
| Referencia de los cocientes | `camaras[0]`, asumiendo que la hero es la primera | `hero_index()`, porque en la rejilla no lo es |
| Procedencia | árbol sin commitear sobre `2c7960a` | árbol de esta tarea sobre `20e0f37` |

#### La matriz, reejecutada

Release, quince rondas intercaladas y rotadas, perfil de serie `320 × 240`:

| Preset | Prim. | `800 × 600` | `320 × 240` | 2os rayos |
|---|---:|---:|---:|---:|
| `safe-canvas` | 159 | `0.2897` | `0.0476` | `597` |
| `safe-painted` | 159 | `0.3439` | `0.0579` | `8 819` |
| `safe-water` | 160 | `0.4463` | `0.0715` | `14 124` |
| `safe-revealing` | 160 | `0.4508` | `0.0717` | `14 355` |
| `target-water` | — | Pendiente | Pendiente | — |

| Escalón | `320 × 240` | `800 × 600` | 2os rayos |
|---|---:|---:|---:|
| lienzo → materiales pintados | `+17.3 %` | `+18.7 %` | `+8 222` |
| volumen refractivo | `+20.1 %` | `+24.8 %` | `+5 305` |
| doble muestreo de la transición | `+5.7 %` | `+4.5 %` | `+231` |

```text
peor cuadro interactivo    0.1259 s   (y+0 e+84 cerca)
critico del gate           0.2667 s
reserva                    2.1x en tiempo
reveal_duration            1.89 s     (15 cuadros)
```

**La reserva es `2.1x`, y ahora es del peor encuadre alcanzable y no de uno
solo.** Es la primera cifra de esta tarea que significa lo que dice.

#### Estado

| Tarea | Estado |
|---|---|
| 7.1 | instrumento suficiente para detectar la regresión; **no** es una caracterización completa del rango interactivo —la rejilla son cuarenta y ocho puntos de un espacio continuo— |
| 7.2 | **no autorizada**. Con `2.1x` hay margen para probar densidad en Aguas de forma incremental, con remedición entre lotes; no para la densidad objetivo completa |
| 7.3 | no activada. El permiso para hexágonos sigue detrás de mitigación y densidad |

Lo que sigue abierto y hay que decir en voz alta: el peor cuadro **medido** es
el peor de una rejilla, no el peor del espacio. Entre dos celdas hay
encuadres sin medir, y la única defensa es que el coste varía de forma suave
con los tres ejes —lo que la propia rejilla muestra— y que el aviso del
ejemplo se dispara si alguna corrida encuentra un punto fuera de banda.

#### Procedencia

Todas las cifras de esta sección salen de dos corridas del 4 de septiembre de
2026 sobre el mismo binario:

```text
cargo run --release --example interactive_frame_time    (fases 1, 2 y 3)
cargo run --release --example performance_matrix        (la matriz)
cargo run --release --example profile_preview           (los seis PNG)
```

Árbol de la Tarea 7.1 sobre `20e0f37`; Ryzen 7 6800H; rustc 1.97.0; release.

Las tablas de encuadres de esta sección y las de la sección anterior **no son
comparables entre sí en valor absoluto**: la rejilla de cuarenta y ocho
cámaras sostiene mucha más carga que el barrido de diez, y la máquina baja de
frecuencia. En la misma corrida en que la hero costaba `0.0743 s` con la
rejilla vieja, con la nueva costaba `0.1241 s`. Lo que reproduce son los
cocientes dentro de una corrida, y son los cocientes los que sostienen todas
las conclusiones de arriba.

#### Gates registrados

```text
cargo fmt -- --check                        OK
cargo clippy --all-targets -- -D warnings   0 avisos
cargo test                                  396 tests, 0 fallos
cargo build --release                       OK
cargo run --release --example interactive_frame_time   codigo 0: el gate pasa
cargo run --release --example performance_matrix       codigo 0: el gate pasa
cargo run                                   320 x 240, 15 cuadros, 0.0972 s
```

Reparto de los 396: `360` de librería, `16` del generador de assets, `8` de
humo del render, `6` de sombras submarinas y `6` de la demo completa.

---

## Pendientes de medición

Ninguna de estas filas puede completarse por estimación. Cada hito llena la suya.

| Hito | Qué se mide | Estado |
|---|---|---|
| 2 | `scene_radius` y `monolith_height` medidos en el blockout | **Registrado** |
| 2 | `orbit_radius` derivado por bisección, con `framing_margin` usado | **Registrado** |
| 3 | Benchmark `safe-interior-visible` (159 primitivas) — mín/mediana/máx | **Registrado** |
| 3 | Benchmark `safe-opaque-water` (160 primitivas) — control de oclusión | **Registrado** |
| 3 | `interactive_frame_time` del perfil interactivo | **Registrado** — perfil fijado en `MEDIA` (400 × 300); movido a `BAJA` (320 × 240) en la 7.1 |
| 5 | Calibración de `L-02`: `distance_boat`, `range`, `intensity` | **Registrado** — `0.192 S`, `0.30 S`, `2.8211` derivada |
| 6 | `reveal_duration` derivada de `interactive_frame_time` | **Registrado** — corregido dos veces en la 7.1: `0.0736 s` en el peor estado **de la toma hero**, `1.5 s` de duración, 20 cuadros; la ventana se autocalibra |
| 7 | Matriz de rendimiento por preset | **Registrado** — cuatro presets en dos resoluciones y diez cámaras, con conteos de rayos; `target-water` pendiente de que exista el nivel objetivo |
| 7 | Peor estado de revelación | **Registrado** — `Finale` a medio revelar sobre el Continente pintado, `1.58x` el lienzo |
| 7 | Peor encuadre alcanzable | **Registrado** — rejilla de 48 cámaras; el peor es la vista cenital al radio mínimo, `3.2x` la toma hero |
| 7 | Perfil interactivo por defecto | **Cerrado** — `BAJA` `320 × 240`, medido contra `MEDIA` en las dos palancas |
| 7 | Límite de zoom | **Cerrado** — `min_radius = 1.8 S`, elegido con `2.34x` de margen sobre el crítico |
| 7 | Encuadre de calibración de la ventana | **Registrado** — `worst_case_camera()`, no la toma hero: la duración se fija una vez y el usuario puede acercarse después |
| 7 | Caracterización completa del rango interactivo | **Abierto** — la rejilla son 48 puntos de un espacio continuo |
| 8 | Hardware de medición y tiempos finales en release | Pendiente |

**Regla.** Todos los benchmarks se ejecutan en release. El perfil `dev` de este proyecto lleva `opt-level = 3` heredado de la base académica, así que un tiempo medido en debug **parece** comparable a release y no lo es.
