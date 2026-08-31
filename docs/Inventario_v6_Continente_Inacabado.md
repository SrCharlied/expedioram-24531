# Inventario de escena v6 — *Expedition 33: El Continente Inacabado*

**Estado:** inventario cerrado para plan técnico; notas de calibración del blockout incorporadas; revelación alineada con `RevealState` (decisión cerrada, ver plan técnico `3.4`)  
**Blueprint de referencia:** `Expedition33_Blueprint_v2_2.svg`  
**Bitácora de referencia:** `Decisiones_Blueprint_v2_Expedition33.md`  
**Región estrella confirmada:** **Aguas Voladoras**  
**Orden de prioridad:** Aguas Voladoras → Acantilado Rompeolas → Praderas Primaverales

---

## 1. Objetivo

Traducir el blueprint espacial a una receta construible sin enumerar manualmente cientos de cubos.

La escena se divide en:

1. **Objetos hero:** piezas únicas que dominan la composición.
2. **Generadores:** conjuntos repetitivos creados mediante reglas y semilla fija.
3. **Masas simplificadas:** cuboides grandes que construyen la silueta general.

Este documento todavía no fija coordenadas mundiales definitivas. Las posiciones y escalas se validarán mediante un blockout gris antes de crear texturas o materiales finales.

### Decisión arquitectónica de revelación

Toda la geometría existe desde el arranque en su posición final. La revelación **no agrega, elimina ni transforma primitivas**: interpola el material visible entre `canvas_unpainted` y el material final mediante el progreso (`0.0–1.0`) del grupo de revelación al que pertenece cada objeto.

Ese progreso vive **centralizado** en `RevealState { progress_by_group: [f32; 4] }`, no dentro de cada objeto. El `SceneObject` es inmutable después de construir la escena.

La estructura de aceleración se construye una vez y permanece estática. El Monolito parece incompleto porque su grupo de revelación (`finale`) sigue en `0.0` y todas sus masas conservan el material de lienzo, no porque falte geometría.

### Unidad de conteo

Todos los campos `count_safe` y `count_target_max` cuentan **primitivas trazables reales** sometidas a intersección. Nunca cuentan una composición lógica como una sola pieza. Cada composición declara un techo interno explícito.

---

## 2. Prioridad de alcance

### Prioridad 1 — Aguas Voladoras

Debe alcanzar el nivel objetivo aunque las otras regiones permanezcan en nivel seguro.

Debe vender:

- Refracción.
- Reflexión.
- Transparencia.
- Profundidad.
- Barco suspendido.
- Borde roto del lienzo.
- Iluminación azul.

### Prioridad 2 — Acantilado Rompeolas

Debe conservar una silueta fuerte con pilares de alturas variables y roca húmeda. Los prismas hexagonales reales son una mejora, no una dependencia.

### Prioridad 3 — Praderas Primaverales

Debe comunicar color, altura y conexión con el Monolito mediante una implementación sencilla. Se amplía únicamente cuando Aguas y Rompeolas funcionan y cumplen rendimiento.

---

## 3. Niveles de alcance

### Nivel seguro

Versión mínima que debe verse terminada:

- 160 primitivas trazables estimadas.
- Tres regiones legibles.
- Aguas Voladoras completa en composición básica.
- Materiales diferenciados.
- Cámara orbital y vista hero funcionales.
- Sin microdetalle orgánico.

### Nivel objetivo

Versión recomendada si el rendimiento lo permite:

- Hasta 275 primitivas trazables.
- Mayor densidad concentrada en Aguas Voladoras.
- Fragmentos, ruinas, flores y vegetación adicional.
- Transiciones pictóricas más claras.

### Nivel soñador

Solo después de completar y medir el nivel objetivo:

- Pigmento nacarado inspirado en Borradores de Verso.
- Partículas.
- Cambio de skybox.
- Movimiento cinematográfico final.
- Detalles opcionales de fauna o vegetación.

El nivel soñador no puede retrasar los requisitos académicos.

---

## 4. Sistema de coordenadas y anclas

El origen global será la **base** del Monolito, sobre el terreno. La línea vertical que atraviesa esa base define el eje de yaw, pero el punto de encuadre se mantiene separado.

```text
scene_origin = monolith_base_anchor = orbit_center = (0, 0, 0)
look_at = monolith_base_anchor + (0, 0.15 × monolith_height, 0)
```

Anclas requeridas:

| ID | Propósito |
|---|---|
| `scene_origin` | Origen lógico de la escena |
| `monolith_base_anchor` | Base del Monolito sobre el terreno |
| `orbit_center` | Punto del eje vertical alrededor del cual se posiciona la cámara |
| `look_at` | Punto de encuadre, ligeramente por encima de la base |
| `meadows_anchor` | Meseta de Praderas |
| `breakwater_anchor` | Franja de pilares y sendero |
| `flying_waters_anchor` | Centro de la bahía sobre el plano horizontal de la superficie del agua (`y = water_surface_y`) |
| `palette_anchor` | Paleta sobre el plinto |
| `hero_camera_anchor` | Posición inicial de cámara |
| `broken_edge_anchor` | Centro del borde roto frontal |

### Parámetros globales de escala

| ID | Definición |
|---|---|
| `scene_radius` | Radio de la esfera que contiene la geometría visible del blockout, excluyendo skybox |
| `monolith_height` | Distancia vertical desde `monolith_base_anchor` hasta la parte superior del Monolito |
| `water_surface_y` | Altura mundial del plano horizontal de la superficie de Aguas Voladoras |
| `orbit_radius` | `2.2 × scene_radius` |

`scene_radius` reemplaza los nombres anteriores `S`, `visible_scene_radius` y “bounding sphere del blockout”. Existe un solo valor canónico. `monolith_height` se mide en el blockout y se almacena como parámetro de escena explícito.

Todo objeto de una región se expresa respecto de su ancla. Esto permite mover una región completa sin recalcular manualmente todas sus piezas.

---

## 5. Materiales

### Material inicial no puntuable

| ID | Uso | Características |
|---|---|---|
| `canvas_unpainted` | Estado inicial de toda la escena | Textura de lienzo, marfil, mate, opaco, no reflectivo |

### Cinco materiales finales puntuables

| # | ID | Uso | Propiedades principales |
|---:|---|---|---|
| 1 | `water` | Bahía y cascada | Transparente, Fresnel, `shadow_mode: ignore` |
| 2 | `wet_basalt` | Rompeolas y sendero | Specular local alto, sin rayo reflejado |
| 3 | `aged_wood` | Barco, mástil y partes del ancla | Mate, oscuro, reflectividad mínima |
| 4 | `meadow` | Césped, flores y árboles simplificados | Albedo colorido, specular bajo |
| 5 | `pictorial_crystal` | Monolito y fragmentos | Brillo y transparencia parcial; sombra decidida por entrada |

Los valores numéricos de albedo, specular, reflectividad, transparencia e índice de refracción se fijarán en el plan técnico y se validarán visualmente. `wet_basalt` tendrá `reflection_cap = 0.0`: su apariencia húmeda se logra con Blinn–Phong/Phong local (`specular_strength` y `shininess`) sin lanzar rayos secundarios.

---

## 6. Parámetros comunes del inventario

Cada entrada debe poder declarar:

```text
required              obligatorio en nivel seguro
casts_shadow          si el objeto participa en sombras opacas
receives_shadow       recibe iluminación y sombras
shadow_mode           opaque | ignore | attenuate
specular_strength     intensidad del brillo local, rango 0.0–1.0
shininess             exponente del brillo local
reflection_cap        techo del componente reflejado, rango 0.0–1.0
transmission_cap      techo del componente refractado, rango 0.0–1.0
ior                    índice de refracción; 1.0 cuando no aplica
reveal_group           uno de los cuatro grupos de revelación
spatial_group          grupo de aceleración
uv_scale               repetición de textura
```

### Grupos de revelación

**Decisión cerrada.** Existen exactamente cuatro grupos, y el progreso es un escalar por grupo. No hay `reveal_progress` ni `reveal_order` por objeto: la revelación es **uniforme dentro del grupo**.

| Índice | `reveal_group` | Entradas |
|---:|---|---|
| 0 | `meadows` | `P-01` … `P-08` |
| 1 | `breakwater` | `R-01` … `R-05` |
| 2 | `flying_waters` | `A-01` … `A-11` |
| 3 | `finale` | `G-01`, `G-03`, `G-05` |

Regla de derivación: salvo declaración explícita, `reveal_group` es el homónimo del `spatial_group` de la entrada. Las entradas globales se asignan a mano porque sus grupos de aceleración (`global`, `monolith`, `continent_background`, `interaction_props`) no son grupos de revelación.

- `G-01` (plinto) queda en `finale`, pero la asignación es inerte: su material inicial y final son ambos `canvas_unpainted`, así que nunca cambia de apariencia.
- `G-03` (Monolito) y `G-05` (fragmentos) se pintan juntos al entrar en `finale`.
- **Pendiente de decisión:** `G-02` (continente simplificado, final `meadow`) y `G-04` (paleta y pincel, final `pictorial_crystal`) todavía no tienen grupo asignado.

### Conservación de energía y Fresnel

`reflection_cap` y `transmission_cap` son techos, no contribuciones constantes. Para un impacto dieléctrico:

```text
F  = fresnel_schlick(cos_theta, ior)
kr = reflection_cap   × F
kt = transmission_cap × (1 - F)
kl = max(0, 1 - kr - kt)
```

El color base combina `kr × reflejo + kt × refracción + kl × local_diffuse`, por lo que `kr + kt + kl = 1`.

Para conservar el destello legible de las luces puntuales, el specular directo Blinn–Phong se suma **después** del reparto Fresnel:

```text
color = kr × reflection
      + kt × refraction
      + kl × local_diffuse
      + direct_specular
```

Esta es una aproximación didáctica deliberada, no un modelo físicamente perfecto. El resultado final se limita o tone-mapea para evitar sobreexposición. En `water`, `specular_strength` y `shininess` controlan ese highlight; no dependen de `kl`.

### Regla de sombras transparentes del MVP

La versión segura usa:

```text
shadow_mode: opaque   → bloquea completamente
shadow_mode: ignore   → el rayo continúa hasta la luz
```

`water` usa `shadow_mode: ignore`; no ocluye el barco ni los objetos submarinos. `pictorial_crystal` no fija un modo global: cada objeto decide si necesita sombra de contacto. El rayo de sombra siempre limita su búsqueda a `distance_to_light - epsilon`.

Una versión posterior puede añadir `shadow_mode: attenuate`. En ese caso, si `transmission_cap` representa transmisión, la visibilidad se multiplica por `transmission_cap` —posiblemente teñida por el albedo—, **no por `1 - transmission_cap`**. La atenuación requiere continuar buscando intersecciones y no se considera igual de barata que un any-hit opaco.

Por defecto:

- Objetos opacos comunes: `reflection_cap = 0.0`, `transmission_cap = 0.0`, `shadow_mode = opaque`.
- Solo agua y cristal pictórico tienen caps mayores que cero y generan rayos secundarios.
- `wet_basalt` usa brillo specular local, pero no genera rebotes.
- Detalles pequeños opcionales no proyectan sombras salvo que mejoren claramente la imagen.

---

# 7. Inventario global

## Presupuesto

| Elemento | Nivel seguro | Nivel objetivo máximo |
|---|---:|---:|
| Plinto | 1 | 1 |
| Continente simplificado | 10 | 14 |
| Monolito | 10 | 12 |
| Paleta y pincel | 6 | 6 |
| Fragmentos globales | 0 | 8 |
| Acentos sin geometría | 0 | 0 |
| **Subtotal seguro** | **27** | **Hasta 41** |

## G-01 · Plinto del lienzo

```yaml
id: global.plinth
category: hero_mass
primitive: cuboid
required: true
count_safe: 1
initial_material: canvas_unpainted
final_material: canvas_unpainted
spatial_group: global
```

**Intención:** sostener el diorama y comunicar físicamente que el Continente nace de un lienzo.

## G-02 · Continente simplificado

```yaml
id: global.simplified_continent
category: generator
primitive: cuboid
required: true
count_safe: 10
count_target_max: 14
initial_material: canvas_unpainted
final_material: meadow
spatial_group: continent_background
```

**Regla:** masas grandes, pocas terrazas, siluetas y costas. Sin materiales exclusivos ni microdetalle.

## G-03 · Monolito

```yaml
id: global.monolith
category: hero
primitive: cuboid_composition
required: true
count_safe: 10
count_target_max: 12
initial_material: canvas_unpainted
final_material: pictorial_crystal
casts_shadow: true
shadow_mode: opaque
reveal_group: finale
spatial_group: monolith
```

**Composición:** 10 primitivas trazables en nivel seguro y un máximo de 12. Evitar construirlo con cientos de piezas pequeñas.

**Revelación:** sus primitivas existen desde el inicio y **todas** conservan `canvas_unpainted` hasta que las tres regiones lleguen a `1.0`. En ese momento el grupo `finale` avanza y las doce primitivas interpolan hacia `pictorial_crystal` de forma uniforme. La geometría y sus bounds nunca cambian.

No hay escalonamiento interno: el Monolito parece incompleto porque el grupo `finale` está en `0.0`, no porque unas masas vayan por delante de otras.

## G-04 · Paleta y pincel

```yaml
id: global.palette_brush
category: hero
primitive: cuboid_composition
required: true
count_safe: 6
count_target_max: 6
initial_material: canvas_unpainted
final_material: pictorial_crystal
casts_shadow: true
shadow_mode: opaque
spatial_group: interaction_props
```

**Nota:** la paleta permanece sobre el plinto, fuera del terreno. Su geometría final dependerá del método de interacción.

## G-05 · Fragmentos pictóricos globales

```yaml
id: global.pictorial_fragments
category: generator
primitive: cuboid
required: false
count_safe: 0
count_target_max: 8
initial_material: canvas_unpainted
final_material: pictorial_crystal
casts_shadow: false
shadow_mode: ignore
seed: fixed
spatial_group: monolith
```

Los fragmentos son opcionales. Los efectos de pigmento o partículas que no participan en intersecciones no consumen presupuesto de primitivas.

---

# 8. Inventario — Praderas Primaverales

## Presupuesto

| Elemento | Nivel seguro | Nivel objetivo máximo |
|---|---:|---:|
| Masas de meseta | 6 | 8 |
| Frente de cascada | 8 | 10 |
| Superficies de césped | 4 | 6 |
| Árboles (primitivas) | 6 | 9 |
| Cascada | 1 | 2 |
| Ruinas opcionales | 0 | 6 |
| Grupos de flores | 12 | 18 |
| Rocas flotantes opcionales | 0 | 7 |
| **Subtotal seguro** | **37** | **Hasta 66** |

## P-01 · Meseta principal

```yaml
id: meadows.plateau
category: mass_generator
primitive: cuboid
required: true
count_safe: 6
count_target_max: 8
initial_material: canvas_unpainted
final_material: meadow
spatial_group: meadows
```

## P-02 · Frente de la cascada

```yaml
id: meadows.waterfall_cliff
category: mass_generator
primitive: cuboid
required: true
count_safe: 8
count_target_max: 10
initial_material: canvas_unpainted
final_material: wet_basalt
spatial_group: meadows
```

Este frente oscuro pertenece a la cascada de Praderas y no duplica el muro de contención de Rompeolas.

## P-03 · Superficies de césped

```yaml
id: meadows.grass_surfaces
category: generator
primitive: thin_cuboid
required: true
count_safe: 4
count_target_max: 6
initial_material: canvas_unpainted
final_material: meadow
uv_scale: repeated
spatial_group: meadows
```

## P-04 · Árboles simplificados

```yaml
id: meadows.trees
category: generator
primitive: cuboid_composition
required: true
count_safe: 6
count_target_max: 9
initial_material: canvas_unpainted
final_material: meadow
casts_shadow: true
spatial_group: meadows
```

Cada árbol utiliza como máximo tres primitivas trazables; el conteo representa primitivas, no árboles lógicos.

## P-05 · Cascada

```yaml
id: meadows.waterfall
category: hero
primitive: elongated_cuboid
required: true
count_safe: 1
count_target_max: 2
initial_material: canvas_unpainted
final_material: water
reflection_cap: 0.10
transmission_cap: 0.25
ior: 1.333
shadow_mode: ignore
spatial_group: meadows
```

**Restricción:** utilizar uno o dos volúmenes largos. No apilar numerosos cubos transparentes. Los pesos son provisionales, pero ya tienen semántica numérica explícita.

## P-06 · Ruinas

```yaml
id: meadows.ruins
category: generator
primitive: cuboid
required: false
count_safe: 0
count_target_max: 6
initial_material: canvas_unpainted
final_material: wet_basalt
spatial_group: meadows
```

## P-07 · Grupos de flores

```yaml
id: meadows.flower_clusters
category: generator
primitive: tiny_cuboid
required: true
count_safe: 12
count_target_max: 18
initial_material: canvas_unpainted
final_material: meadow
casts_shadow: false
seed: fixed
spatial_group: meadows
```

## P-08 · Rocas flotantes

```yaml
id: meadows.floating_rocks
category: generator
primitive: cuboid
required: false
count_safe: 0
count_target_max: 7
initial_material: canvas_unpainted
final_material: pictorial_crystal
casts_shadow: false
shadow_mode: ignore
seed: fixed
spatial_group: meadows
```

---

# 9. Inventario — Acantilado Rompeolas

## Presupuesto

| Elemento | Nivel seguro | Nivel objetivo máximo |
|---|---:|---:|
| Pilares principales | 28 | 42 |
| Segmentos de sendero | 6 | 8 |
| Masas de soporte | 4 | 6 |
| Árbol solitario opcional (primitivas) | 0 | 4 |
| Fragmentos flotantes opcionales | 0 | 5 |
| **Subtotal seguro** | **38** | **Hasta 65** |

## R-01 · Formación de pilares

```yaml
id: breakwater.basalt_formation
category: generator
required: true
count_safe: 28
count_target_max: 42
cluster_count_safe: 4
cluster_count_target: 4
cluster_partition: contiguous_arc_segments
primitives_per_cluster_safe: [7, 7, 7, 7]
primitives_per_cluster_target: [10, 10, 11, 11]
initial_material: canvas_unpainted
final_material: wet_basalt
seed: fixed
spatial_group: breakwater
```

### Ruta A — autorización del profesor

```yaml
primitive: hexagonal_prism
intersection: convex_planes
```

### Ruta B — solo cubos/cuboides

```yaml
primitive: vertical_cuboid
layout: offset_rows
visual_trick: basalt_top_texture
```

**Regla común:** seguir el borde cóncavo del arco, variar altura y mantener una franja de sendero legible.

**Partición espacial obligatoria:** R-01 produce cuatro `SpatialCluster`, uno por tramo contiguo del arco. Cada cluster calcula un AABB ajustado a su tramo, no a toda la formación. El nivel seguro usa 7 pilares por tramo; el objetivo usa 10–11. A-03 (casco) y A-07 (kelp) permanecen en un solo cluster porque son conjuntos compactos.

## R-02 · Sendero húmedo

```yaml
id: breakwater.wet_path
category: generator
primitive: thin_cuboid
required: true
count_safe: 6
count_target_max: 8
initial_material: canvas_unpainted
final_material: wet_basalt
specular_strength: 0.85
shininess: 96
reflection_cap: 0.0
transmission_cap: 0.0
spatial_group: breakwater
```

## R-03 · Masas de soporte

```yaml
id: breakwater.support_masses
category: mass_generator
primitive: cuboid
required: true
count_safe: 4
count_target_max: 6
initial_material: canvas_unpainted
final_material: wet_basalt
spatial_group: breakwater
```

## R-04 · Árbol solitario

```yaml
id: breakwater.solitary_tree
category: hero_accent
primitive: cuboid_composition
required: false
count_safe: 0
count_target_max: 4
initial_material: canvas_unpainted
final_material: meadow
casts_shadow: false
spatial_group: breakwater
```

El árbol es opcional y puede consumir como máximo cuatro primitivas trazables.

## R-05 · Fragmentos flotantes

```yaml
id: breakwater.fragments
category: generator
primitive: cuboid
required: false
count_safe: 0
count_target_max: 5
initial_material: canvas_unpainted
final_material: pictorial_crystal
casts_shadow: false
shadow_mode: ignore
seed: fixed
spatial_group: breakwater
```

---

# 10. Inventario — Aguas Voladoras

## Presupuesto

| Elemento | Nivel seguro | Nivel objetivo máximo |
|---|---:|---:|
| Volumen de agua | 1 | 1 |
| Masas del lecho | 5 | 8 |
| Casco del barco | 12 | 20 |
| Mástil y soportes | 3 | 5 |
| Segmentos de cadena | 8 | 12 |
| Ancla | 3 | 5 |
| Grupos de kelp | 12 | 20 |
| Rocas | 6 | 10 |
| Concha espiral opcional (primitivas) | 0 | 6 |
| Fragmentos submarinos opcionales | 0 | 6 |
| Borde roto | 8 | 10 |
| Acentos sin geometría | 0 | 0 |
| **Subtotal seguro** | **58** | **Hasta 103** |

## A-01 · Volumen de agua

```yaml
id: waters.main_volume
category: hero_volume
primitive: cuboid_or_bounded_volume
required: true
count_safe: 1
initial_material: canvas_unpainted
final_material: water
reflection_cap: 1.0
transmission_cap: 1.0
ior: 1.333
specular_strength: 0.18
shininess: 128
shadow_mode: ignore
spatial_group: flying_waters
```

**Requisitos:** volumen cerrado, normales orientadas correctamente, frontera aire–agua y control explícito del índice de refracción.

## A-02 · Masas del lecho

```yaml
id: waters.seafloor
category: mass_generator
primitive: cuboid
required: true
count_safe: 5
count_target_max: 8
initial_material: canvas_unpainted
final_material: wet_basalt
spatial_group: flying_waters
```

## A-03 · Casco del barco

```yaml
id: waters.ship_hull
category: hero
primitive: cuboid_composition
required: true
count_safe: 12
count_target_max: 20
initial_material: canvas_unpainted
final_material: aged_wood
casts_shadow: true
spatial_group: flying_waters
```

**Prioridad visual:** silueta rota y suspendida. No buscar precisión naval.

## A-04 · Mástil y soportes

```yaml
id: waters.ship_mast
category: hero
primitive: elongated_cuboid
required: true
count_safe: 3
count_target_max: 5
initial_material: canvas_unpainted
final_material: aged_wood
spatial_group: flying_waters
```

## A-05 · Cadena

```yaml
id: waters.anchor_chain
category: hero_generator
primitive: small_cuboid
required: true
count_safe: 8
count_target_max: 12
initial_material: canvas_unpainted
final_material: wet_basalt
spatial_group: flying_waters
```

Los segmentos siguen una línea o curva suave desde el barco hasta el ancla. No se modelan eslabones individuales orgánicos.

**Reuso de material:** la cadena es metal, pero reutiliza `wet_basalt` para conservar el límite de cinco materiales finales. Se distingue mediante escala UV, albedo gris y specular local; sigue teniendo `reflection_cap = 0.0`.

## A-06 · Ancla

```yaml
id: waters.anchor
category: hero
primitive: cuboid_composition
required: true
count_safe: 3
count_target_max: 5
initial_material: canvas_unpainted
final_material: wet_basalt
spatial_group: flying_waters
```

## A-07 · Kelp

```yaml
id: waters.kelp_clusters
category: generator
primitive: elongated_cuboid
required: true
count_safe: 12
count_target_max: 20
initial_material: canvas_unpainted
final_material: meadow
casts_shadow: false
seed: fixed
spatial_group: flying_waters
```

El material `meadow` reutiliza una textura vegetal con variación de tinte submarino; no se crea un sexto material final solo para el kelp.

## A-08 · Rocas submarinas

```yaml
id: waters.rocks
category: generator
primitive: cuboid
required: true
count_safe: 6
count_target_max: 10
initial_material: canvas_unpainted
final_material: wet_basalt
seed: fixed
spatial_group: flying_waters
```

## A-09 · Concha espiral opcional

```yaml
id: waters.spiral_shell
category: hero_accent
primitive: stepped_cuboid_composition
required: false
count_safe: 0
count_target_max: 6
initial_material: canvas_unpainted
final_material: pictorial_crystal
casts_shadow: true
shadow_mode: opaque
spatial_group: flying_waters
```

La concha opcional puede consumir como máximo seis primitivas trazables. No intentar una espiral geométricamente perfecta.

## A-10 · Fragmentos submarinos opcionales

```yaml
id: waters.fragments
category: generator
primitive: cuboid
required: false
count_safe: 0
count_target_max: 6
initial_material: canvas_unpainted
final_material: pictorial_crystal
casts_shadow: false
shadow_mode: ignore
seed: fixed
spatial_group: flying_waters
```

## A-11 · Borde roto

```yaml
id: waters.broken_edge
category: hero_generator
primitive: cuboid
required: true
count_safe: 8
count_target_max: 10
initial_material: canvas_unpainted
final_material: wet_basalt
reflection_cap: 0.0
transmission_cap: 0.0
seed: fixed
spatial_group: flying_waters
```

**Regla geométrica:** el volumen de agua continúa siendo un solo cuboide cerrado. El aspecto rasgado lo producen estos cuboides irregulares de terreno en primer plano, que ocluyen parcialmente la cara frontal del agua. No se intenta rasgar un AABB.

## A-12 · Acentos estrella sin geometría

Opcionales y añadidos únicamente después de medir rendimiento:

- Flores submarinas.
- Más vegetación.
- Segunda concha.
- Fragmentos pictóricos.
- Rayos de luz estilizados.
- Partículas sin geometría de intersección.

La raya nadando queda fuera del alcance inicial.

---

# 11. Cámara e iluminación (no consumen primitivas)

## Cámara orbital

La rama académica `origin/15-RT-03-ORBIT-CAMERA` usa `800 × 600`, aspecto `4:3`, FOV vertical de `π/3` (**60°**) y una cámara donde `center` cumple a la vez las funciones de órbita y mirada. El diorama separará conceptualmente `orbit_center` de `look_at`; esto requiere extender esa estructura durante la implementación.

```yaml
id: camera.hero_orbit
required: true
projection: perspective
resolution: [800, 600]
aspect_ratio: 4:3
vertical_fov_degrees: 60
orbit_center: monolith_base_anchor
look_at: monolith_base_anchor + [0, 0.15 * monolith_height, 0]
up: [0, 1, 0]
eye_elevation_degrees: 35
view_pitch_degrees: derived_from_eye_and_look_at
hero_yaw: faces_broken_edge
orbit_radius: 2.2 * scene_radius
zoom: modifies_orbit_radius
```

Con radio `R` y yaw `θ`, la posición elevada se obtiene mediante:

```text
horizontal_radius = 0.819152 × R
height            = 0.573576 × R
eye.x             = orbit_center.x + horizontal_radius × cos(θ)
eye.y             = orbit_center.y + height
eye.z             = orbit_center.z + horizontal_radius × sin(θ)
view_direction     = normalize(look_at - eye)
```

El valor absoluto de `scene_radius` se mide después de construir el blockout; de allí sale `R = orbit_radius`. `eye_elevation_degrees` describe la posición del ojo sobre la esfera orbital, **no** el pitch final de la vista. Como `look_at` está por encima de `orbit_center`, el pitch se deriva. Por ejemplo, si `monolith_height = 0.5 × scene_radius`, el pitch resulta aproximadamente `33.37°`, aunque la elevación orbital sea `35°`.

La cámara inicial heredada `(0, 0, 5)` sirve solo como referencia didáctica y no como posición final del diorama.

## Luces

Todas las posiciones y rangos usan el parámetro canónico `scene_radius`. Las luces puntuales comparten una atenuación estable respecto de la escala:

```text
attenuation(distance) = intensity / (1 + (distance / range)²)
```

### L-01 · Luz principal cálida

```yaml
id: light.key_warm
required: true
type: point
anchor: monolith_base_anchor
offset: [-0.8 * scene_radius, 1.2 * scene_radius, 0.6 * scene_radius]
color: warm_sun
intensity: 1.0
range: 2.5 * scene_radius
attenuation_model: normalized_quadratic
casts_shadows: true
purpose: separar terrazas, barco y pilares
```

### L-02 · Luz azul de Aguas Voladoras

```yaml
id: light.waters_blue
required: true
type: point
anchor: flying_waters_anchor
offset: [0.0, 0.15 * scene_radius, 0.10 * scene_radius]
color: cool_blue
intensity: 2.0
range: 0.20 * scene_radius
attenuation_model: normalized_quadratic
affected_groups: [flying_waters]
occluder_groups: [flying_waters]
calibration: provisional_until_blockout_4
casts_shadows: true
purpose: mantener legible el barco y permanecer confinada a Aguas Voladoras
```

`affected_groups` implementa *light linking*: antes de evaluar iluminación o lanzar el shadow ray, se comprueba que el receptor pertenezca a un grupo afectado. `occluder_groups` aplica el mismo filtro durante la sombra: L-02 solo puede ser bloqueada por geometría de `flying_waters`, por lo que Praderas no proyecta sombras de una luz que no la ilumina y el shadow ray no recorre los otros grupos.

Es una decisión artística intencional. Bajo distancias provisionales de `0.15 × scene_radius` al barco y `0.45 × scene_radius` a Praderas, la atenuación sin linking bajaría la contribución relativa de Praderas de `84.16%` a `25.77%`; el filtro de grupo la lleva efectivamente a cero fuera de Aguas.

### Calibración obligatoria de L-02 en Blockout 4

Los valores `intensity: 2.0` y `range: 0.20 × scene_radius` son iniciales, no definitivos. Con el modelo declarado:

| Rango | Iluminación a `0.15S` | Iluminación a `0.25S` | Caída relativa |
|---:|---:|---:|---:|
| `0.55S` | 1.8615 | 1.6575 | 10.96% |
| `0.20S` | 1.2800 | 0.7805 | 39.02% |

Medida por caída porcentual, la configuración estrecha es `3.56×` más sensible en este ejemplo, no cinco veces. La observación cualitativa permanece: un error de posición oscurece mucho más el barco.

Durante Blockout 4 se debe:

1. Medir la distancia real desde L-02 al centro visible del barco.
2. Medir la distancia al objeto obligatorio más lejano de Aguas Voladoras.
3. Elegir `range` para que ambos permanezcan legibles.
4. Elegir una contribución objetivo `E_boat` y recalcular:

```text
intensity = E_boat × (1 + (distance_boat / range)²)
```

5. Registrar los valores medidos; no heredar `2.0/0.20S` sin validación.

### L-03 · Acento del Monolito

```yaml
id: light.monolith_accent
required: false
type: point
anchor: monolith_base_anchor
offset: [0.0, 0.5 * scene_radius, -0.25 * scene_radius]
color: pictorial_cyan
intensity: 0.8
range: 0.4 * scene_radius
attenuation_model: normalized_quadratic
casts_shadows: false
purpose: activación final
```

El skybox o término ambiental no lanza rayos de sombra. Cada luz puntual obligatoria sí añade costo de sombras y debe incluirse en las mediciones.

---

# 12. Presupuesto consolidado

| Grupo | Nivel seguro | Nivel objetivo máximo |
|---|---:|---:|
| Global | 27 | 41 |
| Praderas Primaverales | 37 | 66 |
| Acantilado Rompeolas | 38 | 65 |
| Aguas Voladoras | 58 | 103 |
| **Total** | **160** | **275** |

A `800 × 600`, sin aceleración:

```text
Nivel seguro: 76,800,000 pruebas primarias por frame
Nivel objetivo: 132,000,000 pruebas primarias por frame
```

Con dos luces obligatorias que proyectan sombras, una cota de referencia simple —un impacto visible y dos shadow rays sin considerar early-out— es:

```text
Nivel seguro: 230,400,000 pruebas potenciales (primario + 2 sombras)
Nivel objetivo: 396,000,000 pruebas potenciales (primario + 2 sombras)
```

No es una medición de runtime: muchos rayos no impactan y las sombras opacas pueden terminar temprano. Tampoco incluye reflexión, refracción ni recorridos múltiples para una futura sombra atenuada. Por tanto, la aceleración es un requisito y no una optimización opcional.

---

# 13. Aceleración espacial elegida

La estructura del MVP queda cerrada como una jerarquía estática de tres niveles:

```text
scene_bounds
├── global
│   └── clusters por entrada
├── continent_background
│   └── clusters por entrada
├── meadows
│   └── clusters por generador/hero
├── breakwater
│   └── clusters por generador/hero
├── flying_waters
│   └── clusters por generador/hero
├── monolith
│   └── clusters por entrada
└── interaction_props
    └── clusters por entrada
```

Estructuras conceptuales:

```rust
SceneAccel {
    bounds,
    groups: Vec<SpatialGroup>,
}

SpatialGroup {
    id,
    bounds,
    clusters: Vec<SpatialCluster>,
}

SpatialCluster {
    id,
    bounds,
    object_indices: Vec<usize>,
}
```

Una entrada hero compacta produce normalmente un `SpatialCluster`. Un generador puede producir varios cuando su distribución es larga, curva o dispersa; R-01 produce obligatoriamente cuatro. La travesía es:

1. Probar `scene_bounds`.
2. Calcular `t_enter` para los bounds de cada región alcanzada.
3. Ordenar los grupos candidatos por `t_enter` ascendente.
4. Recorrerlos en ese orden y cortar cuando el impacto más cercano conocido sea anterior al `t_enter` del siguiente grupo.
5. Dentro de cada grupo, repetir el ordenamiento y poda con los clusters candidatos.
6. Solo entonces probar las primitivas del cluster.
7. Conservar el impacto válido más cercano.

Solo se ordenan los candidatos cuyo AABB fue alcanzado; son arreglos pequeños. Los bounds se calculan una vez después de generar la escena. El progreso de revelación no los invalida porque la geometría permanece estática y ningún objeto guarda estado mutable. Para sombras opacas se permite terminar al primer bloqueador dentro de `distance_to_light`; para `shadow_mode: ignore` se continúa la travesía.

Esta jerarquía es la solución requerida del MVP. Un BVH genérico queda como optimización posterior únicamente si las mediciones reales muestran que los clusters siguen siendo insuficientes; no condiciona la arquitectura inicial.

---

# 14. Orden de revelación

Las regiones pueden pintarse en cualquier orden durante el uso normal. Pintar solo modifica un `f32` en `RevealState` y la evaluación del material; ninguna primitiva aparece, desaparece o cambia sus bounds. Para la demostración se recomienda:

1. Praderas Primaverales.
2. Acantilado Rompeolas.
3. Aguas Voladoras.
4. Activación final del Monolito.

Aguas se reserva para el cierre porque concentra el cambio visual y técnico más fuerte.

Dentro de Aguas:

```text
pigmento azul
→ superficie y volumen de agua
→ silueta del barco
→ cadena y ancla
→ lecho y rocas
→ kelp y conchas
→ fragmentos pictóricos
→ iluminación final
```

**Alcance:** este orden es una **lectura artística**, no un comportamiento implementado. Con progreso único por grupo, todo `flying_waters` se revela de forma uniforme. Un escalonamiento por objeto requeriría `reveal_order` y estado por primitiva, ambos descartados por la decisión de `RevealState`. Queda como candidato explícito de nivel soñador.

---

# 15. Orden del blockout

## Blockout 1 — composición global

Construir únicamente:

- Plinto.
- Masas principales del arco costero.
- Tres anclas regionales.
- Bahía.
- Monolito.
- Cámara hero.

**Criterio de salida:** las tres regiones son legibles y permanecen dentro del encuadre durante la órbita.

## Blockout 2 — objetos hero

Agregar:

- Barco.
- Cadena y ancla.
- Formación principal de Rompeolas.
- Cascada.
- Árboles.
- Paleta y pincel.

**Criterio de salida:** las siluetas se reconocen sin texturas.

## Blockout 3 — densidad segura

Agregar los generadores hasta alcanzar el nivel seguro.

**Criterio de salida:** medir tiempo de render con rayos primarios y sombras antes de añadir reflexión o refracción.

## Blockout 4 — Aguas Voladoras

Agregar comportamiento óptico del agua y verificar el barco a través del volumen. Medir las distancias reales de L-02, recalcular `range/intensity` y comprobar que `affected_groups` y `occluder_groups` limitan iluminación y sombras a `flying_waters`.

**Criterio de salida:** la toma hero permite distinguir el barco y apreciar refracción sin perder completamente el reflejo de la superficie; los valores calibrados de L-02 quedan registrados.

---

# 16. Decisiones pendientes que no bloquean

| Pendiente | Ruta provisional |
|---|---|
| `reveal_group` de `G-02` y `G-04` | Decidir antes del Hito 6; no bloquea el blockout |
| Permiso para prisma hexagonal | Usar cuboides verticales hasta recibir respuesta |
| Coordenadas y escalas finales | Resolver mediante blockout |
| Valores numéricos restantes de materiales | Resolver en plan técnico y pruebas visuales |
| Densidad objetivo definitiva | Aumentar únicamente después de medir |

---

# 17. Definition of Done del inventario

El inventario queda listo para convertirse en plan técnico cuando:

- [x] Existe una región estrella confirmada.
- [x] Cada región tiene elementos obligatorios y opcionales.
- [x] Los objetos hero están identificados.
- [x] Los detalles repetitivos están definidos como generadores.
- [x] Existe una ruta con y sin prismas hexagonales.
- [x] El material inicial de lienzo está contabilizado.
- [x] La revelación conserva geometría estática y solo interpola materiales.
- [x] El progreso vive solo en `RevealState`, un `f32` por grupo, y los objetos son inmutables.
- [x] Existen exactamente cuatro grupos de revelación y la pintura es uniforme dentro de cada uno.
- [x] El borde roto tiene entrada y presupuesto propios.
- [x] Los conteos representan primitivas trazables y las composiciones tienen techo.
- [x] Las propiedades ópticas usan escalares definidos, no `limited`.
- [x] Fresnel produce pesos efectivos con conservación de energía.
- [x] El agua ignora sombras; el cristal decide `shadow_mode` por objeto.
- [x] El Monolito proyecta sombra opaca de contacto.
- [x] El specular directo del agua se suma después de Fresnel.
- [x] `scene_radius` y `monolith_height` son parámetros canónicos explícitos.
- [x] Las luces declaran intensidad, rango y atenuación.
- [x] L-02 está confinada a `flying_waters` mediante light linking.
- [x] Los oclusores de L-02 también se restringen a `flying_waters`.
- [x] `flying_waters_anchor` está fijada al centro de la superficie del agua.
- [x] `orbit_center` y `look_at` están separados.
- [x] Todos los `spatial_group` existen en el árbol de aceleración.
- [x] La cámara y las luces tienen entradas no geométricas.
- [x] Existe presupuesto seguro y objetivo.
- [x] La aceleración usa jerarquía estática escena→región→cluster→primitiva.
- [x] R-01 se divide en cuatro clusters contiguos a lo largo del arco.
- [x] Grupos y clusters candidatos se recorren por `t_enter` ascendente con poda.
- [ ] Las posiciones relativas se validaron en un blockout.
- [ ] La cámara hero se validó en geometría 3D.
- [ ] L-02 fue recalibrada con distancias medidas del blockout.
- [ ] El presupuesto se verificó mediante medición real del renderer.

Los últimos cuatro puntos pertenecen a la fase de implementación y no bloquean el plan técnico.
