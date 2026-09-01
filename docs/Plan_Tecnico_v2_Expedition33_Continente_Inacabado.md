# Plan técnico de implementación — *Expedition 33: El Continente Inacabado*

> **Para Hermes/Claude:** ejecutar este plan por tareas y verificar cada gate antes de continuar. No saltar directamente al agua, las texturas finales o la escena completa.

**Objetivo:** construir en Rust un diorama raytraced orbital inspirado en *Clair Obscur: Expedition 33*, donde Praderas Primaverales, Acantilado Rompeolas y Aguas Voladoras pasan de lienzo inacabado a materiales finales mediante pintura; Aguas Voladoras es la región estrella.

**Arquitectura:** partir de la rama académica `15-RT-03-ORBIT-CAMERA`, separar el renderer en una librería testeable y mantener `main.rs` como ciclo de ventana/input. Toda la geometría permanece estática; la revelación interpola materiales. La aceleración usa una jerarquía estática `escena → región → cluster → primitiva`, con recorridos ordenados por `t_enter`.

**Stack:** Rust 2021, `minifb 0.26`, `nalgebra-glm 0.18`, `image 0.25` con PNG; `rayon` solo si una medición posterior lo justifica. No usar motores ni crates que implementen raytracing, escenas, BVH, materiales o geometría 3D.

**Fecha del plan:** 31 de agosto de 2026  
**Entrega:** 1 de octubre de 2026  
**Ventana disponible:** 31 días calendario  
**Revisión:** v2.2 — cierra el audit completo: `RevealState` como fuente única, Hito 7 como reserva técnica, duración por frames medidos, `orbit_radius` derivado del encuadre, caps de agua `0.9/0.9`, `max_depth = 3` con terminal en skybox y `shadow_mode` como único campo de sombras  
**Base verificada:** `upstream/15-RT-03-ORBIT-CAMERA` @ `f3e553917077deba3529d9a97f39ea2b58341e84`  
**Verificación realizada:** el árbol `src/` del repositorio de trabajo y el de `f3e5539` comparten hash `d77aad46c439f43ed5f06c2fd393bc25fa5bdf11`; los diez archivos de la base son byte-idénticos

---

## 0. Fuentes de verdad

Orden de prioridad cuando dos documentos discrepen:

1. Enunciado/rúbrica oficial del curso.
2. Respuesta del profesor sobre primitivas adicionales.
3. `Inventario_v6_Continente_Inacabado.md`.
4. `Expedition33_Blueprint_v2_2.svg`.
5. `Decisiones_Blueprint_v2_Expedition33.md`, con las correcciones técnicas posteriores.
6. Referencias visuales.

Archivos de diseño que deben copiarse al repositorio final:

```text
docs/design/Inventario_v6_Continente_Inacabado.md
docs/design/Expedition33_Blueprint_v2_2.svg
docs/design/Decisiones_Blueprint_v2_Expedition33.md
```

**Estado actual:** el inventario ya está en el repositorio, junto con este plan, bajo `docs/`. El SVG del blueprint y la bitácora de decisiones **todavía no existen en el repositorio**; son las fuentes de verdad `4` y `5` y hay que incorporarlas en la Tarea `0.5`. Hasta entonces, cualquier discrepancia de composición se resuelve contra el inventario.

La v6 manda sobre los presupuestos y políticas ópticas. El SVG manda sobre composición, no sobre implementación.

---

## 1. Estado real de la base académica

### Ya existe

- Ventana `800 × 600` con `minifb`.
- Framebuffer de `u32`.
- Rayos primarios por píxel.
- FOV vertical de 60°.
- `Camera { eye, center, up }`.
- Cambio de base cámara→mundo.
- Órbita yaw/pitch con flechas.
- Render solo cuando la cámara cambia.
- `RayIntersect`.
- Intersección con esfera.
- Selección del impacto más cercano.
- `Material` con color difuso.

### Falta

- Cubos/cuboides y UV.
- Tipos heterogéneos de primitivas.
- Color flotante para iluminación recursiva.
- Texturas.
- Luces, atenuación y sombras.
- Sombras transparentes.
- Reflexión, refracción, Fresnel e IOR.
- Skybox.
- Cámara con `orbit_center` separado de `look_at`.
- Zoom.
- Escena, IDs de material y grupos.
- Aceleración espacial.
- Generadores.
- Blockout del Continente.
- Picking con el pincel.
- Revelación por material.
- Render headless, benchmarks y tests.
- README final, evidencia y video.

### Limitación de esta planificación

El host donde se escribió este documento no tiene `rustc` ni `cargo`. Por tanto, **no se afirma que ningún comando de Rust haya pasado aquí**: los comandos de compilación y tests deben ejecutarse en la máquina del equipo.

Lo que sí quedó verificado por comparación real de contenido, no por inspección visual:

- El remoto académico responde y su rama `15-RT-03-ORBIT-CAMERA` apunta a `f3e5539…`.
- Los seis archivos de `src/`, más `Cargo.toml`, `Cargo.lock`, `README.md` y `.gitignore`, son byte-idénticos entre el repositorio de trabajo y esa rama.
- El árbol `src/` comparte hash en ambos lados, así que la base no fue modificada antes de empezar.

---

## 2. Estructura final prevista

```text
.
├── Cargo.toml
├── Cargo.lock
├── README.md
├── assets/
│   ├── textures/
│   │   ├── canvas.png
│   │   ├── water.png
│   │   ├── wet_basalt.png
│   │   ├── aged_wood.png
│   │   ├── meadow.png
│   │   └── pictorial_crystal.png
│   └── skybox/
│       ├── pale.png
│       └── painted.png
├── docs/
│   ├── architecture.md
│   ├── controls.md
│   ├── evidence.md
│   └── design/
│       ├── Inventario_v6_Continente_Inacabado.md
│       ├── Expedition33_Blueprint_v2_2.svg
│       └── Decisiones_Blueprint_v2_Expedition33.md
├── evidence/
│   ├── blockout/
│   ├── renders/
│   └── performance/
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── color.rs
│   ├── framebuffer.rs
│   ├── camera.rs
│   ├── ray.rs
│   ├── hit.rs
│   ├── material.rs
│   ├── texture.rs
│   ├── primitive.rs
│   ├── cuboid.rs
│   ├── hex_prism.rs              # solo Ruta A autorizada
│   ├── bounds.rs
│   ├── accel.rs
│   ├── light.rs
│   ├── optics.rs
│   ├── skybox.rs
│   ├── renderer.rs
│   ├── input.rs
│   ├── reveal.rs
│   ├── scene.rs
│   ├── scene_builder.rs
│   ├── scenes/
│   │   ├── mod.rs
│   │   ├── continent.rs
│   │   ├── meadows.rs
│   │   ├── breakwater.rs
│   │   └── flying_waters.rs
│   └── bin/
│       └── render_scene.rs
└── tests/
    ├── render_smoke.rs
    ├── scene_budget.rs
    └── reveal_static_geometry.rs
```

No crear todos estos archivos el primer día. Cada tarea introduce únicamente lo que necesita el siguiente gate.

---

## 3. Contratos arquitectónicos

### 3.1 Color

Usar color lineal flotante durante el render:

```rust
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
```

Rango normal de trabajo `0.0..1.0`; permitir valores mayores antes de clamp/tone mapping. Convertir a `u32` únicamente al escribir el framebuffer.

### 3.2 Intersección

`Hit` debe contener como mínimo:

```rust
pub struct Hit {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
    pub front_face: bool,
    pub object_index: usize,
}
```

`Hit` no contiene `material_index`: durante la revelación no existe un único material, sino `initial_material`, `final_material` y el progreso del grupo al que pertenece el objeto. El renderer resuelve esos datos mediante `object_index` y una consulta a `RevealState` (ver `3.4`). No copiar `Material` ni estados de revelación dentro de cada impacto.

### 3.3 Primitivas

Usar un enum para evitar `Vec<Box<dyn Trait>>` en el camino caliente:

```rust
pub enum Primitive {
    Cuboid(Cuboid),
    #[cfg(feature = "hex-prism")]
    HexPrism(HexPrism),
}
```

Ruta B con cuboides es la implementación obligatoria hasta que el profesor autorice Ruta A.

### 3.4 Revelación — `RevealState` es la única fuente de progreso

**Decisión cerrada.** El progreso de pintura **no** vive en el objeto. `SceneObject` es completamente inmutable después de construir la escena:

```rust
pub struct SceneObject {
    pub primitive: Primitive,
    pub initial_material: MaterialId,
    pub final_material: MaterialId,
    pub spatial_group: SpatialGroupId,
    pub reveal_group: RevealGroup,
}
```

El progreso vive centralizado, un escalar por grupo:

```rust
pub struct RevealState {
    progress_by_group: [f32; 4],
}
```

Los cuatro grupos son exactamente:

| Índice | `RevealGroup` | Cubre |
|---:|---|---|
| 0 | `Meadows` | Praderas Primaverales |
| 1 | `Breakwater` | Acantilado Rompeolas |
| 2 | `FlyingWaters` | Aguas Voladoras |
| 3 | `Finale` | Monolito, fragmentos, continente simplificado, plinto y paleta |

El renderer resuelve el material en el punto de sombreado:

```rust
let progress = reveal_state.progress(object.reveal_group);
```

Consecuencias que esta decisión garantiza:

- No existen 160 copias mutables del progreso.
- Un clic modifica un solo `f32`, no un recorrido sobre la escena.
- Los objetos permanecen completamente estáticos; la aceleración nunca se reconstruye ni se invalida.
- El test de clic repetido tiene una única fuente que observar.

Las cinco entradas globales (`G-01` … `G-05`) van a `Finale`. Dos son **inertes**: `G-01` (plinto) nace y muere en `canvas_unpainted`, y `G-04` (paleta y pincel) nace ya en `pictorial_crystal` — la herramienta con la que se pinta no puede estar sin pintar. Ambas necesitan grupo por tipado, no por comportamiento.

**Consecuencia aceptada:** la revelación es *uniforme dentro del grupo*. No hay escalonamiento por objeto (`reveal_order` queda fuera del MVP). El Monolito se pinta de una sola vez al entrar en `Finale`, y el orden interno de Aguas descrito en el inventario es una lectura artística, no un comportamiento implementado.

La transformación y los bounds nunca cambian durante la pintura.

### 3.5 Óptica

```text
F  = Schlick(cos_theta, ior)
kr = reflection_cap   × F
kt = transmission_cap × (1 - F)
kl = max(0, 1 - kr - kt)
```

Sumar un specular directo moderado después del reparto Fresnel y clamp/tone-mapear el resultado.

**Decisión cerrada — caps del agua `0.9 / 0.9`.** Con `1.0 / 1.0` el reparto da `kl = 0` y el albedo del agua nunca contribuye: la textura y su `uv_scale` quedarían muertos. Con `0.9 / 0.9`, `kl = 0.1` constante, independiente del ángulo, y es lo que porta el color propio del agua.

**Decisión cerrada — `max_depth = 3` inicial.** No `2`. Un rayo primario que entra al volumen cerrado de Aguas gasta un nivel al refractar en la cara frontal; si no impacta el barco, gasta el segundo en la cara interna trasera y necesita el tercero para salir hacia el lecho, las rocas o el skybox. Con `max_depth = 2` todo lo que está *detrás* del volumen se pierde. Bajar a `2` solo si la medición lo exige, y registrando qué se pierde.

**Decisión cerrada — recursión agotada devuelve skybox.** Nunca negro. Un rayo que llega al límite de profundidad se resuelve muestreando el skybox en su dirección actual, igual que un miss. Con `kl = 0.1` no hay color local suficiente para disimular un terminal negro: se vería como manchas oscuras dentro del agua.

### 3.6 Sombras

```rust
pub enum ShadowMode {
    Opaque,
    Ignore,
    Attenuate, // no requerida para MVP
}
```

- Agua: `Ignore`.
- Monolito/paleta/concha: `Opaque`.
- Fragmentos pequeños: `Ignore`.
- Limitar rayos a `distance_to_light - epsilon`.

**Decisión cerrada — `casts_shadow` no existe.** `ShadowMode` es el único campo de sombras del objeto; el antiguo `casts_shadow` en falso es hoy `ShadowMode::Ignore`. Tener dos banderas para la misma responsabilidad garantizaba que el renderer eligiera mal. El campo `casts_shadows` de las **luces** es distinto y se conserva: dice si la luz genera shadow rays.

**Decisión cerrada — `shadow_mode` no se interpola.** El modo del `final_material` rige durante **toda** la revelación, incluido `progress = 0.0`. El agua no bloquea sombras ni siquiera mientras se ve como lienzo. Si se interpolara, el barco parpadearía entre iluminado y negro justo durante la transición estrella.

### 3.7 Aceleración

```text
SceneAccel
→ SpatialGroup
→ SpatialCluster
→ primitive indices
```

Ordenar candidatos alcanzados por `t_enter`. Podar cuando `closest_t < next_t_enter`.

R-01 produce cuatro clusters:

```text
Seguro:   [7, 7, 7, 7]
Objetivo: [10, 10, 11, 11]
```

### 3.8 Cámara

Separar:

```rust
orbit_center
look_at
eye
up
```

`eye_elevation_degrees = 35`; el pitch visual es derivado. Zoom modifica radio orbital con clamps.

**Decisión cerrada — `orbit_radius` se deriva del encuadre.** No es la constante `2.2 × scene_radius`. Como `look_at` está por encima de `orbit_center`, el eje de vista no pasa por el centro de la esfera envolvente, y ese desvío crece con `monolith_height`:

```text
h     = look_at.y - orbit_center.y
alpha = asin(scene_radius / R)
beta  = φ - atan2(R·sin φ - h, R·cos φ)          φ = 35°

orbit_radius = min R  tal que  alpha(R) + beta(R) ≤ 30° - 2°
```

Ambos términos decrecen monótonamente con `R`; una bisección sobre `[1.01, 8.0] × scene_radius` basta. Valores de referencia:

| `monolith_height` | `orbit_radius` | Con el antiguo `2.2 × S` |
|---|---:|:--|
| `0.5 × scene_radius` | `2.25 × S` | `28.67°` — cabe, margen `1.33°` |
| `1.0 × scene_radius` | `2.38 × S` | `30.36°` — **recorta la escena** |

`min_radius` y `max_radius` del zoom se anclan a este valor derivado, no a una constante. Ver el inventario, sección *Derivación de `orbit_radius`*.

### 3.9 L-02

```yaml
affected_groups: [flying_waters]
occluder_groups: [flying_waters]
calibration: provisional_until_blockout_4
```

No congelar `intensity = 2.0` y `range = 0.20 × scene_radius` sin medir el blockout.

---

# 4. Calendario y gates

| Fechas | Hito | Gate obligatorio |
|---|---|---|
| 31 ago–4 sep | Fundamento testeable y cubo | Un cubo se intersecta, ilumina y renderiza headless |
| 5–8 sep | Cámara final y blockout global | Composición legible en 0°, 90°, 180° y 270° |
| 9–12 sep | Aceleración, luces, sombras y mitigación condicional | Interior de Aguas medido sin A-01; loop usable antes de continuar |
| 13–16 sep | Texturas, skybox y materiales | Cinco materiales + lienzo visibles |
| 17–21 sep | Reflexión, refracción y Aguas | Barco visible a través del agua en hero view |
| 22–24 sep | Picking y revelación | Tres regiones pintables; Monolito final |
| 25–27 sep | **Reserva técnica** | Mitigaciones pagadas y regresiones corregidas; densidad solo si sobra ventana |
| 28 sep | Freeze de features | No entran features nuevas |
| 29–30 sep | README, video, evidencia y entrega | Build release limpio y artefactos finales |
| 1 oct | Margen de entrega | Solo contingencia |

Si un gate falla, se arregla antes de continuar. No compensar un renderer roto con más arte.

---

# 5. Plan de implementación por tareas

## Hito 0 — Preparar repositorio final

**Estado al escribir esta revisión.** El repositorio de trabajo ya existe y no hay que fabricarlo: `origin` apunta a `SrCharlied/expedioram-24531`, la rama es `master`, el árbol está limpio y el código de `src/` coincide con la base académica. Las tareas `0.1` a `0.4` documentan cómo se llegó ahí y cómo lo reproduce otro integrante; solo `0.5` queda pendiente de ejecutar.

El repositorio académico **no** es el remoto de entrega. Se registra aparte, como `upstream`, para poder consultar la base y verificar contra ella — nunca para hacer push.

### Tarea 0.1 — Clonar el repositorio de trabajo

**Objetivo:** partir del repositorio del equipo, que ya es el entregable.

```bash
git clone https://github.com/SrCharlied/expedioram-24531.git
cd expedioram-24531
```

**Verificación:**

```bash
git remote get-url origin      # https://github.com/SrCharlied/expedioram-24531.git
git status --short --branch    # working tree limpio, master...origin/master
```

Reglas:

- `origin` es el remoto de entrega desde el primer momento.
- No incluir tokens en URLs, archivos ni historial.
- Confirmar que otro integrante puede abrir el remoto.

**Commit:** ninguno; clonar no produce commit.

### Tarea 0.2 — Registrar el repositorio académico como `upstream`

**Objetivo:** poder consultar y verificar contra la base del curso sin mezclar remotos.

```bash
git remote add upstream https://github.com/menene/cc2018-2026-02-10.git
git fetch upstream 15-RT-03-ORBIT-CAMERA
```

**Verificación:**

```bash
git remote -v
git rev-parse upstream/15-RT-03-ORBIT-CAMERA
```

Esperado:

```text
origin    https://github.com/SrCharlied/expedioram-24531.git (fetch)
origin    https://github.com/SrCharlied/expedioram-24531.git (push)
upstream  https://github.com/menene/cc2018-2026-02-10.git (fetch)
upstream  https://github.com/menene/cc2018-2026-02-10.git (push)

f3e553917077deba3529d9a97f39ea2b58341e84
```

Reglas:

- **Nunca** hacer push a `upstream`. Es solo lectura.
- No fusionar `upstream` dentro del proyecto: la base ya está en el historial local.

### Tarea 0.3 — Verificar que el código coincide con `f3e5539`

**Objetivo:** probar por contenido —no por confianza— que la base no fue alterada antes de empezar.

El historial local fue aplanado (`06a2b43 Init`), así que los hashes de commit no coinciden con los del curso y compararlos no diría nada. Lo que sí es comparable es el **contenido**.

Verificación fuerte, un solo comando:

```bash
test "$(git rev-parse HEAD:src)" = "$(git rev-parse upstream/15-RT-03-ORBIT-CAMERA:src)" \
  && echo "src/ IDENTICO a f3e5539" || echo "src/ DIFIERE"
```

Esperado: `src/ IDENTICO a f3e5539`, con hash de árbol `d77aad46c439f43ed5f06c2fd393bc25fa5bdf11`.

Verificación extendida, incluyendo los archivos de configuración:

```bash
git diff --stat upstream/15-RT-03-ORBIT-CAMERA HEAD -- \
  src Cargo.toml Cargo.lock README.md .gitignore
```

Esperado: salida vacía. Cualquier línea aquí es una modificación involuntaria de la base y hay que resolverla antes de seguir.

La raíz completa **sí** difiere, y debe hacerlo: el repositorio de trabajo agrega `docs/`, que no existe en la rama académica. Comparar `HEAD^{tree}` contra el árbol de `f3e5539` no es una verificación válida.

**Gate:** `src/` idéntico y diff de configuración vacío antes de escribir la primera línea de código nuevo.

### Tarea 0.4 — Crear `proyecto2/continente-inacabado` desde el `master` verificado

**Objetivo:** aislar el trabajo del proyecto sin perder el `master` que quedó verificado contra la base.

```bash
git switch -c proyecto2/continente-inacabado
git push -u origin proyecto2/continente-inacabado
```

**Verificación:**

```bash
git status --short --branch
git ls-remote --heads origin proyecto2/continente-inacabado
```

Reglas:

- `master` queda como referencia de la base verificada; no se le hace commit de proyecto.
- Todo el trabajo de los Hitos 1–8 vive en `proyecto2/continente-inacabado`.
- El primer push existe en el remoto antes de escribir código.

**Commit:** no crear commit solo por cambiar de rama.

### Tarea 0.5 — Renombrar el paquete y organizar los contratos visuales

**Modificar:** `Cargo.toml`  
**Mover:** `docs/Inventario_v6_Continente_Inacabado.md` → `docs/design/`  
**Mover:** `docs/Plan_Tecnico_v2_Expedition33_Continente_Inacabado.md` → `docs/design/`  
**Incorporar:** `docs/design/Expedition33_Blueprint_v2_2.svg`  
**Incorporar:** `docs/design/Decisiones_Blueprint_v2_Expedition33.md`

Cambiar el paquete a:

```toml
[package]
name = "expedition33_continente_inacabado"
version = "0.1.0"
edition = "2021"
```

Conservar inicialmente:

```toml
nalgebra-glm = "0.18.0"
minifb = "0.26.0"
```

Agregar después `image`, no antes de necesitar texturas.

Organizar los contratos visuales con `git mv`, para que el historial siga los archivos:

```bash
mkdir -p docs/design
git mv docs/Inventario_v6_Continente_Inacabado.md docs/design/
git mv docs/Plan_Tecnico_v2_Expedition33_Continente_Inacabado.md docs/design/
```

**Pendiente real:** el SVG del blueprint y la bitácora de decisiones no están en el repositorio. Son fuentes de verdad `4` y `5` de la sección `0`. Copiarlos a `docs/design/` en esta tarea; si todavía no existen en forma de archivo, anotarlo explícitamente en `docs/evidence.md` en lugar de dejar la carpeta incompleta en silencio.

Sobre `[profile.dev] opt-level = 3`, que viene de la base: **conservarlo** —hace usable el loop interactivo durante el desarrollo— pero documentar en el README que aquí el perfil debug está optimizado. De lo contrario, un tiempo medido en debug parecerá comparable a release y no lo es. Todos los benchmarks del plan se ejecutan en release, sin excepción.

**Verificación:**

```bash
cargo check
git status --short
```

**Commit sugerido:**

```text
Prepara base del Continente Inacabado
```

---

## Hito 1 — Núcleo matemático testeable

### Tarea 1.1 — Separar librería y binario

**Crear:** `src/lib.rs`  
**Modificar:** `src/main.rs`

Mover declaraciones de módulos públicos a `lib.rs`. `main.rs` debe importar la librería del paquete y conservar únicamente ventana, input y presentación del buffer.

**Test inicial:** ejecutar tests existentes; actualmente se esperan cero tests, pero la compilación debe conservarse.

```bash
cargo test
cargo run
```

**Commit:** `Separa renderer en librería testeable`

### Tarea 1.2 — Convertir Color a flotante

**Modificar:** `src/color.rs`

Agregar tests unitarios para:

- Construcción en `0.0..1.0`.
- Suma sin pérdida prematura.
- Multiplicación escalar.
- Clamp al convertir a `u32`.
- Ausencia de wrap-around.

**RED:** escribir tests esperando `Color::new(1.0, 0.5, 0.0)`.

```bash
cargo test color -- --nocapture
```

Esperado antes de implementar: fallo de tipos/API.

**GREEN:** implementar operaciones flotantes y `to_hex()`.

**Commit:** `Convierte color a representación flotante`

### Tarea 1.3 — Introducir Ray y Hit

**Crear:** `src/ray.rs`  
**Crear:** `src/hit.rs`  
**Modificar:** `src/ray_intersect.rs`  
**Modificar:** `src/sphere.rs`

`Ray` encapsula origen, dirección y `at(t)`. `Hit` reemplaza `Intersect` y orienta la normal mediante `front_face`.

Tests:

- `Ray::at(0)` devuelve origen.
- `Ray::at(2)` avanza dos unidades.
- Normal frontal apunta contra el rayo.
- Normal interna se invierte y `front_face = false`.

**Commit:** `Normaliza contrato de rayos e impactos`

### Tarea 1.4 — Implementar AABB y slab test

**Crear:** `src/bounds.rs`

Tests obligatorios:

- Rayo frontal impacta.
- Rayo paralelo fuera falla.
- Rayo paralelo dentro conserva intervalo.
- Rayo nacido dentro devuelve salida válida.
- Intervalos producen `t_enter` y `t_exit`.
- Epsilon evita autoimpacto.

API mínima:

```rust
pub struct Aabb { pub min: Vec3, pub max: Vec3 }
pub struct RayInterval { pub t_enter: f32, pub t_exit: f32 }
```

**Gate:** `cargo test bounds` en verde.

**Commit:** `Agrega intersección robusta con AABB`

### Tarea 1.5 — Implementar Cuboid con normal y UV

**Crear:** `src/cuboid.rs`

El cuboide usa `Aabb` para distancia, identifica la cara de entrada/salida y calcula UV por cara.

Tests:

- Las seis caras devuelven la normal correcta.
- UV permanece en `0.0..1.0`.
- Rayo interno usa cara de salida.
- Escala no uniforme funciona.
- Objeto detrás de cámara falla.

**Gate visual:** sustituir temporalmente las tres esferas por un cuboide y producir una imagen coloreada por normales.

**Commit:** `Implementa cuboides con normales y UV`

### Tarea 1.6 — Generalizar Primitive y SceneObject

**Crear:** `src/primitive.rs`  
**Crear:** `src/scene.rs`  
**Modificar:** `src/renderer.rs` o extraer `cast_ray` desde `main.rs`

Introducir:

```rust
pub enum Primitive { Cuboid(Cuboid) }
pub struct SceneObject {
    pub primitive: Primitive,
    pub initial_material: MaterialId,
    pub final_material: MaterialId,
    pub spatial_group: SpatialGroupId,
    pub reveal_group: RevealGroup,
}
```

`SceneObject` **no** lleva `reveal_progress`: ese estado vive en `RevealState` por grupo (ver `3.4`). Una vez construida la escena, el objeto es inmutable.

Eliminar la firma `objects: &[Sphere]`.

Test: una escena con dos cuboides devuelve el más cercano aunque el lejano esté primero.

**Gate Hito 1:**

```bash
cargo fmt -- --check
cargo test
cargo clippy --all-targets -- -D warnings
```

Además, render headless pequeño o captura manual de un cubo correctamente sombreable.

---

## Hito 2 — Cámara final y blockout

### Tarea 2.1 — Separar orbit_center y look_at

**Modificar:** `src/camera.rs`

Nueva cámara:

```rust
pub struct Camera {
    pub eye: Vec3,
    pub orbit_center: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,
    pub vertical_fov: f32,
    pub min_radius: f32,
    pub max_radius: f32,
}
```

Tests:

- Órbita conserva radio respecto a `orbit_center`.
- `basis_change` apunta al `look_at`.
- Mover `look_at.y` no mueve el eje orbital.
- Pitch orbital no cruza polos.

**Commit:** `Separa órbita y encuadre de cámara`

### Tarea 2.2 — Agregar zoom y ray_from_pixel

**Modificar:** `src/camera.rs`

Agregar:

```rust
zoom(delta)
ray_from_pixel(x, y, width, height)
```

Tests:

- Píxel central apunta a `look_at`.
- Esquinas respetan FOV/aspecto.
- Zoom conserva dirección y cambia radio.
- Clamps impiden entrar al diorama o alejarse demasiado.
- `orbit_radius` derivado contiene la esfera envolvente: `alpha + beta ≤ 28°`.
- Con `monolith_height = 1.0 × scene_radius` el radio derivado supera `2.2 × scene_radius`.

Controles previstos:

```text
Flechas: órbita
Rueda o W/S: zoom
R: volver a cámara hero
Escape: salir
```

### Tarea 2.3 — Crear render headless

**Crear:** `src/renderer.rs`  
**Crear:** `src/bin/render_scene.rs`

El binario debe poder renderizar sin abrir ventana:

```bash
cargo run --release --bin render_scene -- \
  --preset blockout \
  --width 800 --height 600 \
  --output evidence/blockout/hero.png
```

No usar `clap`; parseo pequeño con `std::env::args` es suficiente.

Tests/smoke:

- Render `32 × 24` termina.
- No produce NaN.
- Al menos un píxel no es background.
- Guarda PNG válido cuando `image` se agregue.

En esta tarea agregar:

```toml
image = { version = "0.25", default-features = false, features = ["png"] }
```

### Tarea 2.4 — Definir anclas y parámetros de escena

**Crear:** `src/scene_builder.rs`  
**Crear:** `src/scenes/mod.rs`  
**Crear:** `src/scenes/continent.rs`

Declarar:

```text
monolith_base_anchor
orbit_center
look_at
meadows_anchor
breakwater_anchor
flying_waters_anchor = centro de superficie
broken_edge_anchor
scene_radius
monolith_height
water_surface_y
orbit_radius = derivado por bisección, no constante
```

`orbit_radius` se calcula al terminar el blockout, cuando `scene_radius` y `monolith_height` ya están medidos. Registrar el valor resultante junto con las dos medidas de entrada.

No fijar detalle todavía. Construir únicamente plinto, masas del arco, bahía, tres regiones y Monolito con cuboides grises.

### Tarea 2.5 — Validar Blockout 1

Renderizar:

```text
hero
90 grados
180 grados
270 grados
lateral/corte aproximado
```

Guardar en `evidence/blockout/`.

Checklist:

- Monolito permanece eje visual.
- Borde roto encara hero view.
- Praderas está arriba.
- Rompeolas sostiene la meseta.
- Aguas tiene espacio para barco y lecho.
- Nada esencial sale del frame.
- `look_at` no manda el Continente al tercio inferior.

**No continuar si falla.** Ajustar anclas, no materiales.

**Gate Hito 2:** blockout aprobado visualmente en cuatro ángulos y tests de cámara verdes.

---

## Hito 3 — Aceleración, luces y sombras opacas

### Tarea 3.1 — Implementar SpatialGroup y SpatialCluster

**Crear:** `src/accel.rs`

Estructuras según inventario v6. Los bounds se calculan a partir de primitivas después de generar escena.

Tests:

- Bounds de cluster contienen todas sus primitivas.
- Bounds de grupo contienen todos sus clusters.
- Material reveal no modifica bounds.
- Grupo fallado evita probar primitivas.

### Tarea 3.2 — Ordenar candidatos por t_enter

**Modificar:** `src/accel.rs`

Tests con contador instrumentado:

- Grupo cercano se visita antes que lejano.
- `closest_t < next_t_enter` poda el grupo lejano.
- Resultado coincide con brute force.
- Shadow any-hit termina en primer opaco antes de la luz.

Conservar una función brute-force solo bajo `#[cfg(test)]` para comparar resultados.

### Tarea 3.3 — Implementar generador R-01 en cuatro clusters

**Crear:** `src/scenes/breakwater.rs`

Ruta B obligatoria: cuboides verticales. Semilla determinista propia; no añadir `rand`.

Tests:

```text
safe   → 28 pilares, clusters [7,7,7,7]
target → 42 pilares, clusters [10,10,11,11]
```

Verificar que cada cluster cubra un tramo contiguo del arco y que no exista un AABB único para toda la formación.

### Tarea 3.4 — Implementar PointLight y atenuación

**Crear:** `src/light.rs`

```text
attenuation = intensity / (1 + (distance / range)^2)
```

Tests con valores conocidos:

- Intensidad en distancia 0.
- Mitad relativa cuando `distance = range`.
- L-02 ignora receptores fuera de `flying_waters`.
- L-02 solo consulta oclusores de `flying_waters`.

### Tarea 3.5 — Implementar diffuse + Blinn–Phong

**Crear:** `src/material.rs`  
**Modificar:** `src/renderer.rs`

Separar:

```text
ambient
lambert diffuse
direct specular
```

`wet_basalt` usa specular alto con `reflection_cap = 0`.

Tests:

- Superficie frente a luz recibe máximo diffuse.
- Superficie opuesta recibe cero diffuse.
- Shininess alto estrecha highlight.
- Atenuación reduce contribución.

### Tarea 3.6 — Implementar sombras y ShadowMode

**Modificar:** `src/renderer.rs`  
**Modificar:** `src/material.rs`

Tests:

- Opaco entre punto y luz bloquea.
- Opaco detrás de la luz no bloquea.
- Agua `Ignore` no bloquea.
- Monolito `Opaque` sí bloquea.
- L-02 no consulta Praderas como oclusor.
- Epsilon evita acne básico.

### Tarea 3.7 — Construir nivel seguro y presets tempranos

**Crear:**

```text
src/scenes/meadows.rs
src/scenes/flying_waters.rs
```

Agregar primitivas seguras sin reflexión/refracción todavía:

```text
Global 27
Praderas 37
Rompeolas 38
Aguas 58
Total 160 (incluye A-01)
```

A-01 debe admitir dos presets tempranos:

1. `safe-interior-visible`: no insertar el volumen de agua como primitiva intersectable; puede mostrarse únicamente como wireframe/debug marker. Quedan `159` primitivas trazables y los rayos alcanzan barco, mástil, cadena, ancla, kelp, rocas y lecho.
2. `safe-opaque-water`: insertar A-01 como cuboide azul opaco. Conserva las `160` primitivas y sirve para validar el volumen/composición, pero **no** para aprobar rendimiento porque oculta aproximadamente 44 primitivas interiores.

No crear todavía una falsa transparencia sin óptica. El preset canónico de benchmark temprano es `safe-interior-visible`.

### Tarea 3.8 — Medir checkpoint sin agua y control opaco

Ejecutar ambos presets en release al menos tres veces:

```bash
cargo run --release --bin render_scene -- \
  --preset safe-interior-visible \
  --output evidence/performance/safe-interior-visible.png \
  --benchmark 3

cargo run --release --bin render_scene -- \
  --preset safe-opaque-water \
  --output evidence/performance/safe-opaque-water.png \
  --benchmark 3
```

Registrar por preset en `docs/evidence.md`:

- CPU.
- Resolución.
- Primitivas trazables (`159` sin A-01; `160` con A-01).
- Grupos/clusters.
- Luces con sombra.
- Rays y primitive tests instrumentados, si están disponibles.
- Tiempo mínimo/mediana/máximo.
- Explicación de por qué el preset opaco poda el interior.

El gate de rendimiento usa `safe-interior-visible`; `safe-opaque-water` es un control visual y una comparación de oclusión. No fijar FPS antes de medir.

### Tarea 3.9 — Mitigación condicional: resolución interactiva

**Disparador:** ejecutar inmediatamente si la Tarea 3.8 muestra latencia molesta al orbitar o si la revelación futura no podría animarse fluidamente.

**Modificar:** `src/renderer.rs`, `src/main.rs`, `src/framebuffer.rs`

Mientras cámara o revelación cambia, renderizar en un perfil interno medido —comenzar con `400 × 300` y permitir `320 × 240` si hace falta— y escalar a `800 × 600`. Al quedar quieta, producir el frame final a `800 × 600`.

No esperar al Hito 7: los siguientes hitos deben poder probarse interactivamente.

### Tarea 3.10 — Mitigación condicional: evaluar rayon

**Disparador:** evaluar solo si resolución interactiva y aceleración estática no bastan.

```toml
[features]
parallel = ["dep:rayon"]

rayon = { version = "1.10", optional = true }
```

Paralelizar por filas/píxeles sin compartir mutablemente el framebuffer. Comparar serial vs. paralelo en release, con el mismo preset y al menos cinco repeticiones. Conservarlo únicamente si mejora de forma consistente y no compromete estabilidad.

### Coste de calendario de las mitigaciones

**Decisión cerrada.** Si `3.9` o `3.10` se activan, el tiempo sale de la reserva, no de la calidad:

- No se eliminan tests.
- No se acepta un loop inutilizable como "suficiente".
- El Hito 3 puede consumir uno o dos días adicionales.
- Los hitos siguientes se desplazan **en bloque**.
- Esos días salen de la ventana del **25–27 de septiembre**.
- El freeze del **28 de septiembre permanece fijo**.

En otras palabras, el Hito 7 deja de ser "tres días garantizados para añadir cosas bonitas" y pasa a ser la reserva técnica del calendario.

**Gate Hito 3:** benchmark interior registrado y loop suficientemente usable para desarrollar los Hitos 4–6; si fue necesario, la mitigación queda implementada aquí.

---

## Hito 4 — Texturas, materiales y skybox

### Tarea 4.1 — Implementar Texture y muestreo UV

**Crear:** `src/texture.rs`

Tests:

- Carga PNG.
- Clamp/repeat según modo.
- Muestreo de cuatro esquinas conocido.
- Textura ausente devuelve error claro, no color silencioso.

### Tarea 4.2 — Generar y versionar seis texturas mínimas

**Crear:** `src/bin/generate_assets.rs`  
**Crear y versionar:** `assets/textures/*.png` y bases de `assets/skybox/*.png`

Reservar el **13 de septiembre** para producir y revisar assets. El generador usa `image` y una semilla fija; crea PNG reales, repetibles y transportables. Como mínimo genera bases para `canvas`, `wet_basalt`, `meadow`, agua, madera, cristal y los dos panoramas. Se permite retocar después, pero el proyecto debe poder regenerar una base válida sin descargar archivos externos.

Comando previsto:

```bash
cargo run --release --bin generate_assets
```

Los PNG generados se commitean porque son los assets de textura que carga el renderer; el generador también se conserva como fuente reproducible.

Materiales:

1. Canvas no puntuable.
2. Water.
3. Wet basalt.
4. Aged wood.
5. Meadow.
6. Pictorial crystal.

El máximo puntuable sigue siendo cinco finales; canvas es estado inicial.

Registrar en README que los assets base fueron generados dentro del proyecto, con algoritmo/semilla. Si alguno se reemplaza o retoca con una fuente externa, guardar licencia y atribución. No usar imágenes sin procedencia.

### Tarea 4.3 — Implementar Material completo

**Modificar:** `src/material.rs`

Campos mínimos:

```rust
albedo_texture
specular_strength
shininess
reflection_cap
transmission_cap
ior
shadow_mode
uv_scale
```

Tests de defaults y límites `0.0..1.0`.

### Tarea 4.4 — Implementar material reveal

**Crear:** `src/reveal.rs`  
**Modificar:** `src/renderer.rs`

Interpolar canvas→final usando el progreso del grupo (`reveal_state.progress(object.reveal_group)`). No mezclar transforms. El material no guarda estado de revelación.

Tests:

- Progress 0 produce canvas.
- Progress 1 produce material final.
- Progress intermedio mezcla de forma estable.
- Cambiar progress no modifica bounds ni conteo de objetos.
- `shadow_mode` NO se interpola: el agua reporta `Ignore` también en `progress = 0.0`.
- `G-01` y `G-04` producen el mismo material en todo el rango de progress.

### Tarea 4.5 — Implementar skybox equirectangular

**Crear:** `src/skybox.rs`  
**Usar/refinar:** `assets/skybox/pale.png` y `painted.png`, generados inicialmente en 4.2

Muestrear dirección de rayo→UV equirectangular. Interpolar skyboxes según progreso global. No presupuestar aquí la autoría desde cero de dos panoramas: esta tarea implementa muestreo y ajuste visual sobre las bases ya generadas.

Tests:

- Direcciones cardinales producen UV esperada.
- Miss ray devuelve skybox, no color fijo.

**Asignación mínima del Hito 4:**

```text
13 sep: generador + ocho assets base (6 texturas, 2 panoramas)
14 sep: carga/muestreo UV + validación de tiling
15 sep: Material + reveal de materiales
16 sep: skybox + integración + gate
```

**Gate Hito 4:** render safe con lienzo y render final con cinco materiales claramente distinguibles; todos los assets cargan desde un clon limpio.

---

## Hito 5 — Reflexión, refracción y Aguas Voladoras

### Tarea 5.1 — Implementar reflect y refract

**Crear:** `src/optics.rs`

Tests:

- Reflexión sobre normal vertical.
- Refracción aire→agua.
- Refracción agua→aire.
- Total internal reflection.
- Origen secundario usa epsilon correcto según lado.

### Tarea 5.2 — Implementar Schlick y caps

**Modificar:** `src/optics.rs`

Tests:

- Fresnel normal coincide aproximadamente con `R0`.
- Fresnel aumenta hacia grazing.
- `kr + kt + kl = 1` dentro de tolerancia.
- Ningún par de caps genera más de 100% de energía.
- Caps `0.9/0.9` producen `kl = 0.1` exacto en todo el rango de ángulos.

### Tarea 5.3 — Agregar cast_ray recursivo limitado

**Modificar:** `src/renderer.rs`

**Profundidad máxima inicial: `3`.** Cruzar el volumen cerrado de Aguas cuesta dos niveles (entrada y salida); el tercero es el que permite ver el lecho y las rocas detrás del agua. Bajar a `2` solo si la medición lo exige, documentando qué se pierde.

**Recursión agotada devuelve skybox**, nunca negro ni un color fijo.

Orden:

1. Impacto.
2. Iluminación directa.
3. Specular directo.
4. Rayo reflejado si `kr > threshold`.
5. Rayo refractado si `kt > threshold`.
6. Clamp/tone mapping.

Tests:

- Depth 0 no recurre.
- Mirror simple refleja skybox.
- Agua deja ver objeto interior.
- Resultado permanece finito.
- Rayo que agota `max_depth` devuelve la muestra de skybox de su dirección, no negro.
- Con `max_depth = 3`, un rayo que atraviesa el volumen alcanza el lecho al otro lado.
- Con `max_depth = 2`, ese mismo rayo termina en skybox y no en negro.

### Tarea 5.4 — Construir volumen cerrado y borde roto

**Modificar:** `src/scenes/flying_waters.rs`

Implementar:

- Un volumen de agua cerrado.
- Ocho cuboides seguros de borde roto; máximo diez.
- El volumen no se “rasga”.
- Los cuboides de terreno ocluyen parcialmente la cara frontal.

Test de presupuesto y test visual headless.

### Tarea 5.5 — Construir barco, cadena y ancla

**Modificar:** `src/scenes/flying_waters.rs`

Presupuesto seguro:

```text
casco 12
mástil 3
cadena 8
ancla 3
```

El barco se prioriza por silueta. La cadena reutiliza `wet_basalt` con tinte/UV metálico y cero reflexión recursiva.

### Tarea 5.6 — Validar sombras submarinas

Render controlado con:

- Agua presente.
- Barco dentro.
- L-01 fuera.
- L-02 enlazada a Aguas.

Criterios:

- El barco no está negro.
- Agua no bloquea shadows.
- Rocas opacas sí producen sombra.
- Monolito conserva sombra de contacto.

### Tarea 5.7 — Calibrar L-02 con blockout real

Medir:

```text
distance_boat
distance_farthest_required_water_object
scene_radius
```

Elegir range y derivar:

```text
intensity = E_boat × (1 + (distance_boat / range)^2)
```

Actualizar valores en código, `Inventario` copiado o `docs/evidence.md`, y capturar comparación antes/después.

### Tarea 5.8 — Gate Aguas Voladoras

Render hero `800 × 600`:

- Superficie devuelve skybox.
- Borde frontal permite ver barco.
- Highlight del agua visible.
- Barco, cadena y ancla legibles.
- No hay acne severo ni negro total.
- Tiempo release registrado.

Si falla rendimiento:

1. Mantener 58 primitivas seguras.
2. Bajar profundidad recursiva.
3. Desactivar reflexión del cristal opcional.
4. Render interactivo a menor resolución.
5. No recortar el barco ni el borde roto.

---

## Hito 6 — Picking, paleta y revelación

### Tarea 6.1 — Extraer ray_from_cursor

**Modificar:** `src/camera.rs`  
**Crear:** `src/input.rs`

Transformar cursor de minifb a rayo de mundo usando la misma función que el renderer.

Tests:

- Centro de ventana coincide con rayo central.
- Aspect ratio y FOV coinciden con render.

### Tarea 6.2 — Implementar picking de región

**Modificar:** `src/input.rs`  
**Modificar:** `src/scene.rs`

El impacto devuelve `RevealGroup`. No pintar por vóxel ni modificar textura libremente.

Agregar fallback de demo:

```text
1: Praderas
2: Rompeolas
3: Aguas
```

El mouse es la interacción principal; teclado garantiza presentación fiable.

### Tarea 6.3 — Implementar RevealState y temporización

**Modificar:** `src/reveal.rs`

**Decisión cerrada — `RevealState` almacena únicamente `[f32; 4]`.** La fase no se guarda: se **deriva** del escalar, para no reintroducir el estado duplicado que esta decisión eliminó.

```rust
pub enum RevealPhase { Unpainted, Revealing, Painted }

impl RevealState {
    pub fn phase(&self, g: RevealGroup) -> RevealPhase {
        match self.progress(g) {
            p if p <= 0.0 => RevealPhase::Unpainted,
            p if p >= 1.0 => RevealPhase::Painted,
            _             => RevealPhase::Revealing,
        }
    }
}
```

`RevealPhase` es una vista de solo lectura. Nada la escribe ni la persiste.

#### Duración derivada de frames medidos

**Decisión cerrada.** La duración no se elige por gusto: se deriva del `interactive_frame_time` medido en el Hito 3, con piso y techo explícitos.

```text
reveal_duration = clamp(15 × interactive_frame_time, 1.5, 4.0)
reveal_speed    = 1.0 / reveal_duration
```

El objetivo es garantizar **al menos quince frames** de transición, para que se lea como animación y no como corte. El piso de `1.5 s` evita que una máquina rápida produzca un parpadeo.

**Matiz del techo:** si `interactive_frame_time > 0.267 s`, quince frames ya no caben en cuatro segundos. En ese caso el techo **no** se levanta: falla el gate.

Criterio operativo:

1. Medir `interactive_frame_time` en el perfil interactivo del Hito 3.
2. Calcular la duración requerida para quince frames.
3. Aplicar piso de `1.5 s`.
4. Si supera `4.0 s`, el preset interactivo **falla el gate de fluidez**.
5. Bajar resolución o mejorar rendimiento; no alargar indefinidamente la animación.
6. Repetir la medición.

| `interactive_frame_time` | 15 frames requieren | `reveal_duration` |
|---:|---:|:--|
| `0.05 s` | `0.75 s` | `1.5 s` (piso) |
| `0.10 s` | `1.50 s` | `1.5 s` |
| `0.20 s` | `3.00 s` | `3.0 s` |
| `0.30 s` | `4.50 s` | **falla el gate; bajar resolución** |

Al activar una región, avanzar con tiempo real:

```text
progress = min(1.0, progress + delta_seconds * reveal_speed)
```

No avanzar por cantidad de frames: una máquina lenta debe terminar la transición en aproximadamente el mismo tiempo de pared. Los quince frames son el **criterio de aceptación**, no el mecanismo de avance.

Así la transición no se evalúa por "se siente más o menos bien", sino por una medida concreta y defendible. Registrar el `interactive_frame_time` medido y la duración resultante en `docs/evidence.md`.

Monolito comienza el finale únicamente cuando las tres regiones llegan a 1.0.

Tests:

- Una región se revela sin afectar otras.
- Monolito no inicia antes de las tres.
- Repetir click no reinicia ni duplica progreso.
- Geometría y accel permanecen idénticas.
- Dos secuencias con distintos frame times alcanzan progreso equivalente para el mismo tiempo acumulado.
- `SceneObject` no expone ningún campo mutable de progreso; el estado completo cabe en `[f32; 4]`.
- `reveal_duration` respeta el piso `1.5` y el techo `4.0`.
- Un `interactive_frame_time` mayor que `0.267 s` reporta fallo de gate, no una duración mayor que `4.0`.

### Tarea 6.4 — Integrar dirty rendering

**Modificar:** `src/main.rs`

Renderizar cuando:

- Cámara cambia.
- Zoom cambia.
- Reveal progress avanza.
- Región seleccionada cambia.

Todo frame con cámara en movimiento o con algún grupo en `RevealPhase::Revealing` (derivada, ver `6.3`) usa el perfil de resolución interactiva definido/medido en Hito 3; al terminar el movimiento o la transición se renderiza un frame final `800 × 600`. Cuando nada cambia, reutilizar framebuffer como la rama del profesor.

### Tarea 6.5 — Añadir cámara hero/reset

**Modificar:** `src/input.rs` y `src/camera.rs`

Tecla `R` restaura eye/orbit/look_at del blueprint. Guardar preset hero en escena, no hardcodearlo en input.

**Gate Hito 6:** demo completa desde lienzo hasta Monolito final sin recompilar.

---

## Hito 7 — Reserva técnica, densidad y Ruta A opcional

Esta ventana (25–27 sep) es la **reserva técnica del calendario**, no tiempo garantizado para detalle. Prioridad estricta:

1. Pagar las mitigaciones tempranas que se hayan activado en el Hito 3.
2. Corregir regresiones.
3. Agregar densidad objetivo.
4. Implementar el prisma hexagonal opcional.

Si las mitigaciones consumen dos días, **el prisma hexagonal y el detalle objetivo son los primeros candidatos a desaparecer**. Los tests del núcleo no se negocian y el freeze del 28 de septiembre no se mueve.

### Tarea 7.1 — Ejecutar matriz de rendimiento

Presets:

```text
safe-canvas
safe-painted
safe-water
safe-revealing
target-water
```

Medir en release. No comparar debug con release.

### Tarea 7.2 — Agregar detalle objetivo por prioridad

Orden estricto:

1. Aguas hasta máximo 103.
2. Rompeolas hasta máximo 65.
3. Praderas hasta máximo 66.
4. Global hasta máximo 41.

Después de cada incremento, repetir benchmark hero y órbita.

Detenerse si empeora interacción o pone en riesgo entrega.

### Tarea 7.3 — Ruta A de prisma hexagonal, solo con autorización

**Crear:** `src/hex_prism.rs`  
**Modificar:** `src/primitive.rs`, `Cargo.toml`

Implementar como intersección con ocho semiespacios/planos convexos:

```text
6 laterales + tapa superior + tapa inferior
```

Tests:

- Impacto lateral.
- Impacto superior.
- Rayo paralelo.
- Normal de cada cara.
- UV lateral y tapa.
- Resultado visual equivalente a Ruta B.

Feature:

```toml
[features]
hex-prism = []
```

Si el profesor no responde o dice que no: no crear este archivo; Ruta B es final.

---

## Hito 8 — Freeze, documentación y entrega

### Tarea 8.1 — Congelar features el 28 de septiembre

Crear lista de bugs únicamente. No agregar:

- Fauna.
- Personajes.
- Movimiento libre.
- Geometría orgánica.
- Más regiones.
- Caustics reales.
- Malla OBJ.
- Postprocesado complejo.

### Tarea 8.2 — Ejecutar quality gates

```bash
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```

Si Ruta A no existe, omitir feature `hex-prism`; no dejar feature rota.

### Tarea 8.3 — Generar evidencia final

Renderizar y guardar:

```text
hero_canvas.png
hero_meadows.png
hero_breakwater.png
hero_waters.png
hero_final.png
angle_90.png
angle_180.png
angle_270.png
```

Registrar hardware, preset, primitivas, luces, profundidad y tiempo.

### Tarea 8.4 — Completar README

**Modificar:** `README.md`

Contenido obligatorio:

- Concepto y vínculo con Expedition 33.
- Captura hero.
- Controles.
- Arquitectura del raytracer.
- Cubos/prismas propios.
- Materiales.
- Reflexión y refracción.
- Aceleración.
- Cómo compilar y ejecutar.
- Cómo correr tests.
- Decisiones de alcance.
- Procedencia reproducible de assets; créditos/licencias de cualquier fuente externa.
- Hardware de medición.
- Limitaciones conocidas.

### Tarea 8.5 — Grabar video

Guion recomendado:

1. Continente en lienzo.
2. Órbita breve.
3. Pintar Praderas.
4. Pintar Rompeolas.
5. Pintar Aguas Voladoras.
6. Acercarse al borde roto y mostrar barco/refracción.
7. Activación del Monolito.
8. Vista final y créditos.

Ensayar la secuencia con teclas fallback antes de grabar.

### Tarea 8.6 — Verificación limpia

En una copia limpia:

```bash
git clone <url-final>
cd <repo>
cargo test
cargo build --release
cargo run --release
```

Verificar que assets usen rutas relativas y no dependan de archivos fuera del repositorio.

### Tarea 8.7 — Estado de entrega

Confirmar:

- Repositorio público/privado según curso.
- Rama correcta.
- README visible.
- Video accesible.
- Sin `.env`, tokens o binarios enormes.
- `target/` ignorado.
- Tag/release si se solicita.

---

# 6. Tests mínimos obligatorios

## Geometría

- AABB frontal, paralelo, interior y detrás.
- Seis normales de cuboide.
- UV por cara.
- Prisma hexagonal solo si autorizado.

## Cámara

- Radio orbital constante.
- `look_at` separado.
- Píxel central.
- FOV/aspecto.
- Zoom clamped.
- `orbit_radius` derivado contiene la esfera envolvente con margen de `2°`.

## Aceleración

- Resultado igual a brute force.
- Orden `t_enter`.
- Poda de grupos/clusters.
- R-01 `28/42` y `4` clusters.

## Iluminación

- Diffuse y specular.
- Atenuación.
- Shadow distance cap.
- Agua ignore.
- Monolito opaque.
- Linking de receptores y oclusores.
- `shadow_mode` del material final rige en todo el rango de revelación.

## Óptica

- Reflect.
- Refract.
- TIR.
- Schlick.
- Energía, con `kl = 0.1` para caps `0.9/0.9`.
- Recursión limitada a `max_depth = 3`.
- Terminal de profundidad agotada es skybox.

## Revelación

- `0/1/intermedio`.
- Independencia de regiones.
- Finale del Monolito.
- Bounds estáticos.
- Progreso basado en `delta_seconds`, no en cantidad de frames.
- Transición usa resolución interactiva y termina con frame final completo.
- `RevealState` es la única fuente de progreso: `[f32; 4]`, sin estado por objeto.
- `reveal_duration` clamped a `[1.5, 4.0]` y gate de fluidez sobre `0.267 s/frame`.

## Integración

- Presupuesto safe exactamente `160`.
- Render `32 × 24` sin NaN.
- PNG headless válido.
- Preset sin A-01 expone y mide el interior de Aguas (`159` trazables).
- Preset opaco conserva `160` primitivas y documenta la poda.
- `Hit` resuelve materiales por `object_index`, sin `material_index`.
- Barco visible bajo agua.

---

# 7. Presupuesto y recortes

## Nivel seguro — obligatorio

```text
Global       27
Praderas     37
Rompeolas    38
Aguas        58
Total       160
```

## Nivel objetivo — condicionado por medición

```text
Global       41
Praderas     66
Rompeolas    65
Aguas       103
Total       275
```

## Orden de recorte si falta tiempo

1. Eliminar acentos soñadores y partículas.
2. Mantener Praderas en safe.
3. Mantener Rompeolas en safe.
4. Reducir fragmentos/cristal opcional.
5. Mantener Aguas safe completa.
6. Mantener barco, agua, borde roto, Monolito, órbita y zoom.

Nunca recortar primero:

- Barco.
- Borde roto.
- Agua reflectiva/refractiva.
- Pilares de Rompeolas.
- Cascada principal.
- Monolito.
- Cámara orbital.
- Skybox.
- Cinco materiales finales.

---

# 8. Riesgos y respuestas

| Riesgo | Señal temprana | Respuesta |
|---|---|---|
| Benchmark temprano demasiado optimista | Agua opaca poda 44 primitivas interiores | Gate con `safe-interior-visible`; opaco solo como control |
| Cientos de objetos congelan cámara | Render safe lento en Hito 3 | Aplicar low-res/rayon condicional en Hito 3, antes de continuar |
| Agua vuelve negro el barco | Shadow test cruza volumen | `ShadowMode::Ignore` para agua |
| Agua sale blanca | Se suman dos recursiones completas | Caps + Fresnel + clamp |
| Agua no parece agua | Sin highlight puntual | Specular directo después de Fresnel |
| L-02 contamina Praderas | Tinte azul fuera de bahía | `affected_groups` y `occluder_groups` |
| L-02 oscurece barco | Distancia real distinta | Recalibrar range/intensity en Blockout 4 |
| Monolito flota | Sin sombra de contacto | `shadow_mode: Opaque` |
| Rompeolas acelera mal | AABB largo lleno de aire | Cuatro clusters contiguos |
| Cámara encuadra cielo | Anchor usado como centro alto | Base/orbit/look_at separados |
| Escena recortada en la órbita | `orbit_radius` constante con Monolito alto | Radio derivado por bisección con margen de `2°` |
| Manchas negras dentro del agua | Rayo agota `max_depth` y no hay color local | `max_depth = 3` y terminal en skybox |
| Textura del agua invisible | Caps `1.0/1.0` dejan `kl = 0` | Caps `0.9/0.9` ⇒ `kl = 0.1` |
| Sombras incoherentes al pintar | `shadow_mode` interpolado | Modo del material final durante toda la revelación |
| Profesor niega hexágonos | Ruta A bloqueada | Ruta B con cuboides/textura |
| Assets faltan o consumen demasiado tiempo | Archivos manuales/paths absolutos | Generador determinista + PNG versionados + clean clone |
| Scope crece | Fauna/personajes/región nueva | Freeze y lista explícita de no objetivos |

---

# 9. Definition of Done

El proyecto está terminado cuando:

- [ ] Parte del commit orbital documentado.
- [ ] Compila en release desde clon limpio.
- [ ] Todos los tests pasan.
- [ ] `fmt` y `clippy` pasan.
- [ ] Usa cubos/cuboides propios; prisma solo si autorizado.
- [ ] Tiene cinco materiales finales texturizados más canvas inicial.
- [ ] Tiene iluminación diffuse/specular y sombras.
- [ ] Tiene reflexión y refracción visibles.
- [ ] Agua no bloquea iluminación submarina.
- [ ] Los caps del agua son `0.9 / 0.9` y el albedo contribuye con `kl = 0.1`.
- [ ] `max_depth = 3` y la recursión agotada devuelve skybox, nunca negro.
- [ ] `orbit_radius` se derivó del encuadre y quedó registrado con sus dos medidas de entrada.
- [ ] `ShadowMode` es el único campo de sombras del objeto; `casts_shadow` no existe.
- [ ] `RevealPhase` se deriva del `f32`; no se almacena.
- [ ] Monolito proyecta sombra.
- [ ] Skybox funciona.
- [ ] Cámara orbita y hace zoom.
- [ ] Picking/revelación funciona con fallback de teclado.
- [ ] Las tres regiones se revelan.
- [ ] Aguas Voladoras es legible y protagonista.
- [ ] Monolito se activa al final.
- [ ] Nivel safe respeta 160 primitivas.
- [ ] Rendimiento está medido, no supuesto.
- [ ] El progreso de revelación vive solo en `RevealState`; `SceneObject` es inmutable.
- [ ] `reveal_duration` se derivó de `interactive_frame_time` medido y quedó registrada.
- [ ] README explica arquitectura, controles y ejecución.
- [ ] Assets generados documentan algoritmo/semilla; cualquier fuente externa tiene créditos/licencia.
- [ ] Video demuestra la secuencia completa.
- [ ] Repositorio no contiene secretos ni assets externos rotos.

---

# 10. Primer bloque de trabajo recomendado

No comenzar creando la escena completa. El primer bloque implementable es:

```text
1. Crear repo/branch desde f3e5539 y conectar el remoto final.
2. Separar lib/main.
3. Convertir Color a f32.
4. Crear Ray/Hit.
5. Implementar AABB.
6. Implementar un Cuboid.
7. Escribir tests de seis caras.
8. Renderizar un cubo por normales.
```

**Criterio para avanzar:** `cargo test`, `cargo clippy` y una imagen headless de un cubo correcto. Hasta entonces no se toca agua, barco, texturas finales ni generadores.
