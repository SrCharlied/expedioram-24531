# 15 — Raytracing: Cámara orbital

Tercera etapa de la fase de **Raytracing** del curso **cc2018 – Gráficas por Computadora** (UVG). Hasta aquí la cámara estuvo clavada en el origen viendo hacia −Z, y la única forma de ver la escena desde otro lado era mover las esferas. Esta etapa separa las dos cosas: los objetos se quedan donde están y lo que se mueve es el punto desde el cual se mira.

## Objetivo

- Describir la cámara con `eye`, `center` y `up`.
- Construir una base ortonormal a partir de esos tres vectores.
- Convertir la dirección del rayo de coordenadas de cámara a coordenadas del mundo.
- Orbitar el ojo alrededor del centro con el teclado.
- Hacer explícito el campo de visión.

## Controles

| Tecla | Acción |
| ----- | ------ |
| `←` | Orbitar hacia la izquierda |
| `→` | Orbitar hacia la derecha |
| `↑` | Subir sobre la escena |
| `↓` | Bajar bajo la escena |
| `Escape` | Salir |

## Dos sistemas de coordenadas

El generador de rayos de las etapas anteriores solo sabe hacer una cosa: repartir direcciones sobre un rectángulo en el plano XY, apuntando hacia −Z. Esa es una descripción cómoda y no conviene perderla — el mapeo de píxel a dirección no debería depender de hacia dónde está viendo la cámara.

La salida son entonces dos sistemas distintos:

- **Coordenadas de cámara**: el ojo en el origen, la vista hacia −Z, la pantalla en XY. Aquí nacen todos los rayos, siempre igual.
- **Coordenadas del mundo**: donde están las esferas, y donde la cámara es un punto cualquiera viendo en una dirección cualquiera.

El puente entre ambos es un **cambio de base**. El rayo se genera en el primero y se reexpresa en el segundo justo antes de lanzarlo.

## La base de la cámara

Los tres vectores que describen la cámara son la convención de `lookAt`, la misma de OpenGL:

| Vector | Qué es |
| --- | --- |
| `eye` | dónde está la cámara |
| `center` | qué punto está viendo |
| `up` | hacia dónde queda «arriba» |

De ahí salen los tres ejes:

```rust
let forward = (self.center - self.eye).normalize();
let right = forward.cross(&self.up).normalize();
let up = right.cross(&forward).normalize();
```

Hay un detalle en la tercera línea que es fácil pasar por alto: el `up` que se calcula **no es** el `up` que se recibió. El que se recibe es una intención —«arriba es hacia allá»— y no tiene por qué ser perpendicular a la dirección de vista; en cuanto la cámara se eleva sobre la escena, deja de serlo. Recalcularlo como el producto cruz de los otros dos garantiza que los tres ejes queden mutuamente perpendiculares, que es lo que hace que la imagen no salga sesgada.

Eso también explica el límite del pitch. Si la cámara llegara justo encima de la escena, `forward` sería paralelo a `up`, su producto cruz sería el vector cero, y normalizar el cero da `NaN`: la imagen se rompe. Detenerse una décima de radián antes del polo evita el caso degenerado.

Con la base lista, el cambio de base es una combinación lineal:

```rust
let rotated = vector.x * right + vector.y * up - vector.z * forward;
```

El signo negativo del último término es la misma convención de siempre: en coordenadas de cámara se ve hacia −Z, así que una `z` negativa tiene que salir hacia **adelante** en el mundo.

Nótese que el cambio de base solo rota — no traslada. La posición entra por otro lado: el rayo ya no sale del origen sino de `camera.eye`.

## Orbitar

`orbit` mueve el ojo sobre una esfera imaginaria centrada en `center`, sin cambiar el radio. Es más fácil en **coordenadas esféricas**: el vector que va del centro al ojo se descompone en radio, yaw (el ángulo alrededor del eje Y) y pitch (la altura sobre el plano XZ), se le suman los incrementos, y se rearma el vector.

```
yaw   = atan2(z, x)
pitch = atan2(−y, √(x² + z²))
```

El radio se calcula pero no se toca, y por eso la cámara nunca se acerca ni se aleja: gira alrededor de la escena a distancia fija. El yaw da la vuelta completa con el módulo `2π`; el pitch se recorta contra los polos.

Girar alrededor de la escena es también la manera más directa de comprobar la prueba de profundidad de la etapa anterior: la esfera azul se ve entera desde el frente, y a un cuarto de vuelta la de marfil le tapa poco más de la mitad.

## El campo de visión, ahora explícito

Las etapas 13 y 14 ponían el plano de proyección a una unidad de distancia con la pantalla de −1 a 1, lo que fijaba el campo de visión en 90 grados sin decirlo. Ahora el FOV es un parámetro y el plano se escala a partir de él:

```rust
let perspective_scale = (FOV / 2.0).tan();
```

Es la relación inversa de antes: con el plano a distancia 1, la media altura de la ventana es `tan(FOV/2)`. Con `FOV = π/3` (60 grados) esa media altura es 0.577, más angosta que la de 90 grados — la escena se ve más de cerca, como con un lente más largo.

La corrección de aspecto sigue aplicándose solo a la horizontal, así que el FOV declarado es el **vertical** y el horizontal sale más ancho en la misma proporción que la ventana.

## Renderizar solo cuando hace falta

Con la cámara móvil vuelve el problema que la etapa 13 había esquivado: la imagen ya no se puede calcular una sola vez. Pero tampoco hace falta recalcularla 60 veces por segundo cuando nadie está tocando el teclado — la escena es estática y la cámara quieta da exactamente el mismo resultado.

```rust
if camera_moved {
    render(&mut framebuffer, &objects, &camera);
    camera_moved = false;
}
```

La bandera se enciende con cada tecla de órbita y con el primer cuadro. El ciclo de la ventana sigue corriendo a su ritmo y presentando el buffer; lo que se ahorra son los 480 000 rayos de los cuadros en los que nada cambió.

## Estructura

```
.
├── Cargo.toml            # Manifiesto del proyecto (minifb, nalgebra-glm)
├── Cargo.lock            # Versiones exactas de las dependencias
└── src
    ├── main.rs           # Generación de rayos, campo de visión y ciclo de eventos
    ├── camera.rs         # Base de la cámara, cambio de base y órbita
    ├── framebuffer.rs    # Buffer de píxeles en memoria
    ├── color.rs          # Color por canales, suma y escalado
    ├── ray_intersect.rs  # Material, Intersect y el trait común a los objetos
    └── sphere.rs         # Esfera, solución de la cuadrática y normal
```

## Cómo correr

1. Clonar el repositorio y cambiar a esta rama:
    ```bash
    git clone https://github.com/menene/cc2018-2026-02-10.git
    cd cc2018-2026-02-10
    git checkout 15-RT-03-ORBIT-CAMERA
    ```

2. Compilar y ejecutar:
    ```bash
    cargo run
    ```

3. Se abren tres esferas alrededor del origen. Con las flechas se gira alrededor de ellas: a un cuarto de vuelta la de marfil tapa poco más de la mitad de la azul, y con `↑` se llega a verlas desde arriba. Vale la pena cambiar `FOV` para ver cómo se abre y se cierra el encuadre, y sustituir el `up` recalculado por el `up` recibido para ver la imagen sesgarse en cuanto la cámara se eleva. Cerrar con `Escape` o con el botón de cerrar de la ventana.

## Recursos

- [Rust Programming Language](https://www.rust-lang.org/)
- [minifb](https://docs.rs/minifb/)
- [nalgebra-glm](https://docs.rs/nalgebra-glm/)
- [Change of basis](https://en.wikipedia.org/wiki/Change_of_basis)
- [Spherical coordinate system](https://en.wikipedia.org/wiki/Spherical_coordinate_system)
- [`gluLookAt` — la convención eye / center / up](https://registry.khronos.org/OpenGL-Refpages/gl2.1/xhtml/gluLookAt.xml)
- [Scratchapixel — Placing a Camera: the LookAt Function](https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/lookat-function/framing-lookat-function.html)

## Procedencia de los assets

Los ocho assets de `assets/` se **generan dentro del proyecto**. No hay
imágenes descargadas, así que no hay licencias de terceros que rastrear ni
acreditar.

```bash
cargo run --release --bin generate_assets
```

| Archivo | Tamaño | Semilla |
|---|---|---|
| `assets/textures/canvas.png` | 256 × 256 | `0x0CA1_7A50` |
| `assets/textures/water.png` | 256 × 256 | `0x0A90_A900`, alabeo `0x0A90_0AA0` |
| `assets/textures/wet_basalt.png` | 256 × 256 | `0x0BA5_A170`, motas `0x5A17_0501` |
| `assets/textures/aged_wood.png` | 256 × 256 | `0x0DE0_71BA`, fibra `0xF1B0_4400` |
| `assets/textures/meadow.png` | 256 × 256 | `0x6BA5_5A00`, brotes `0xB007_E500` |
| `assets/textures/pictorial_crystal.png` | 256 × 256 | `0xC157_A100` |
| `assets/skybox/pale.png` | 1024 × 512 | `0x0A1E_0001` |
| `assets/skybox/painted.png` | 1024 × 512 | `0x0A17_7ED0` |

**Algoritmo.** Ruido de valor sobre una rejilla, sumado en octavas con
lacunaridad 2 y ganancia 0.5. Los índices de la rejilla se envuelven con
`rem_euclid`, así que el ruido es **periódico por construcción** y las
texturas repiten sin costura visible. Encima de esa base, cada material
aplica su propio patrón: trama y urdimbre para el lienzo, ondas deformadas
por ruido para el agua, anillos distorsionados para la madera, escalones
para las facetas del cristal.

Con semilla fija, regenerar desde un clon limpio produce bytes idénticos.
Los PNG se versionan porque son los que carga el renderer; el generador se
conserva como su fuente reproducible. Está permitido retocarlos después,
pero si alguno se sustituye por una imagen de origen externo hay que guardar
su licencia y atribución aquí.

**Espacio de color.** Los PNG están en sRGB, que es lo que espera cualquier
visor. El renderer los decodifica a lineal al cargarlos y vuelve a codificar
a sRGB al escribir el píxel; el cálculo de iluminación, reflexión y
refracción ocurre siempre en lineal.

**Panoramas del skybox.** Equirectangulares y cubren la esfera completa:
`v = 0` es el nadir, `v = 0.5` el horizonte y `v = 1` el cenit. Cubrir la
esfera entera es necesario porque la cámara orbita a 35° de elevación
mirando hacia abajo, así que hay rayos que fallan la geometría viajando por
debajo del horizonte.
