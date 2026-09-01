# Bitácora de decisiones — Blueprint v2

**Proyecto:** Proyecto 2 · Diorama con raytracing · *Expedition 33: El Continente Inacabado*
**Artefacto que documenta:** `Expedition33_Blueprint_v2.svg` (2400 × 1540)
**Estado:** blueprint visual cerrado. Todavía sin inventario de objetos ni implementación en Rust.

---

## 1. Por qué existe la v2

El blueprint v1 fallaba dos criterios del punto 16 del brief:

- **Criterio 3** — no se podía responder cómo se conectan espacialmente las regiones. Eran tres elipses traslapadas sin jerarquía de altura.
- **Criterio 10** — no decía nada del encuadre ni de cómo se ve la escena desde otros ángulos.

Además leía como diagrama corporativo, no como concepto artístico, lo cual contradice el punto 11 (dirección visual).

Lo único rescatable del v1 fue la idea del corte lateral.

---

## 2. Decisiones confirmadas por el usuario

### D-01 · Aguas Voladoras se resuelve como bahía de borde roto

Se compararon tres composiciones:

| Opción | Ventaja | Por qué se descartó |
|---|---|---|
| **A** · pecera lateral | Refracción visible siempre desde la cámara hero | Una pared de vidrio vertical y plana es el caso más fácil de refracción: el rayo entra casi perpendicular y se desvía poco. Se ve como ventana sucia. Además rompe la ficción del mapa-diorama y colapsa al rotar 90° |
| **B** · bahía oculta | Coherente desde cualquier ángulo | Con cámara elevada en ángulo rasante, Fresnel devuelve mayoritariamente **reflexión**. Se ve cielo, no barco. Se pierde el efecto justo en la toma de presentación |
| **C** · bahía de borde roto | **Elegida** | — |

**Solución adoptada:** el agua se hunde en la tierra como bahía normal, pero el terreno del extremo frontal está arrancado con corte irregular. Ese borde roto funciona como pared de corte.

Se obtienen los dos efectos simultáneos: superficie horizontal reflectiva desde arriba, y pared abierta que da refracción lateral hacia el barco.

**Justificación narrativa:** un lienzo inacabado tiene bordes rotos. La ficción paga la técnica en vez de pelearse con ella.

### D-02 · Silueta del continente: arco costero largo

Tomado de la referencia aérea, donde el continente lee como arco costero con bandas de terreno encadenadas a lo largo de la costa, no como isla compacta ni como línea recta.

Resuelve el conflicto del punto 5 del brief entre "mapa visto desde arriba" y "composición vertical apilada": el arco se lee como mapa en planta y como escalera de tres peldaños en corte.

### D-03 · Monolito en el centro exacto

Sinergia no prevista al proponerlo: si la cámara es orbital, el Monolito se vuelve el **eje de rotación**. El arco costero pasa por delante y por detrás de él en cada vuelta, así que la composición se sostiene en los 360°.

Consecuencia geométrica: el radio de órbita queda definido por la geometría del plinto, no a ojo.

---

## 3. Decisiones tomadas al trazar el blueprint

### D-04 · Rompeolas es el muro de contención, no una zona vecina

El basalto se dibuja como franja pegada al borde cóncavo del arco, sosteniendo físicamente la meseta de Praderas. Justificación estructural, no decorativa.

Efecto secundario: permite que una sola línea de corte atraviese Pradera → Rompeolas → Agua en orden, sin doblarse.

### D-05 · El barco va suspendido, no hundido en el fondo

Observado en la referencia: el barco cuelga de una cadena larga que baja a un ancla en el lecho. No descansa en el fondo.

Es el mejor elemento del proyecto para raytracing: un objeto suspendido dentro del volumen de agua obliga al rayo a refractar dos veces antes de llegar al casco. Sale gratis conceptualmente y es fiel a la referencia.

### D-06 · La cascada es el conector físico entre nivel 3 y nivel 1

No es adorno. Cae del borde sur de la meseta hacia la bahía y explica visualmente la diferencia de altura. En cubos es una columna de cubos transparentes: barato y vende mucho.

### D-07 · La mitad derecha del arco no recibe detalle

Zona etiquetada "continente simplificado": masas grandes, costas, niebla, siluetas. Cero cubos texturizados propios. Es lo que permite que el mapa se lea completo dentro del presupuesto.

### D-08 · El corte A–A′ es esquemático, en un plano que no toca el Monolito

El Monolito queda al este de la línea de corte. En el corte lateral se dibuja punteado y etiquetado "detrás del plano de corte", que es lo correcto en dibujo técnico.

**Revisable:** si se prefiere que el corte lo atraviese, hay que mover la línea y perder la lectura limpia de las tres terrazas.

### D-09 · El borde roto encara a la cámara inicial

Único punto del diseño donde la orientación de cámara y la geometría están acopladas: **si se mueve la cámara hero, el borde roto se mueve con ella.**

### D-10 · Presupuesto de cubos como restricción de composición

Cada cubo se prueba contra cada rayo. A 800 × 600 sin estructura de aceleración, cada cubo cuesta ~480,000 tests rayo-AABB por frame.

| Zona | Cubos |
|---|---|
| Praderas | 100 |
| Rompeolas | 90 |
| Aguas Voladoras | 110 |
| Monolito | 40 |
| Relleno y fragmentos | 60 |
| **Total** | **≈ 400** |

Techo duro: 500. El Monolito se construye por masas grandes (8–12 cubos escalados), no por cientos de cubos pequeños. Fragmentos flotantes: máximo 20.

Un blueprint que dibuje 2,000 cubos es un blueprint que miente.

---

## 4. Observado vs. interpretado en las referencias

Separación exigida por el punto 14 del brief.

**Observado directamente:**

- Barco suspendido de cadena con ancla en el fondo; vegetación vertical tipo kelp; conchas espirales gigantes; una raya nadando; rayos de luz volumétricos; flores rosadas y violetas en el lecho.
- Praderas: repisa elevada con flores rosadas, ruinas de mampostería, cascadas por acantilado oscuro, fragmentos rocosos flotando, niebla en el valle.
- Rompeolas: columnas hexagonales de basalto oscuro, sendero mojado con specular fuerte, sol bajo, cubos flotando, árbol solitario diminuto, paleta casi monocroma.
- Vista aérea: arco costero, bandas encadenadas (basalto → bosque verde → bosque rojo → dorado), vetas de luz en el suelo, verticales monumentales al fondo.

**Correcciones sobre supuestos previos:**

- En la referencia de Praderas, el elemento brillante con energía celeste es **un árbol seco**, no el Monolito. El Monolito no aparece con claridad en ninguna de las seis referencias, así que su silueta es interpretación propia.
- Las imágenes 4 y 5 son el mismo archivo duplicado.

---

## 5. Geometría clave del SVG

Coordenadas dentro del panel 1 (vista superior), útiles si hay que editar a mano:

| Elemento | Posición |
|---|---|
| Plinto (lienzo) | x 162–1454 · y 290–900 |
| Monolito | centro (810, 590) — centro exacto del plinto |
| Órbita de cámara | círculo r = 330 centrado en el Monolito |
| Cámara inicial | (640, 873), sobre la órbita · elevación ≈ 35° |
| Praderas | centro ≈ (588, 420) |
| Rompeolas | franja siguiendo el borde (400,472) → (690,762) |
| Aguas Voladoras | concavidad, centro ≈ (500, 680) |
| Borde roto | de (640, 822) a (440, 772) |
| Paleta y pincel | (240, 700), sobre el plinto, fuera del terreno |
| Corte A–A′ | de A (400, 770) a A′ (700, 330) |

---

## 6. Materiales

| # | Material | Uso | Comportamiento |
|---|---|---|---|
| 1 | Agua | Aguas Voladoras | Transparencia + reflexión + refracción |
| 2 | Roca húmeda | Rompeolas | Specular alto, reflectividad moderada |
| 3 | Madera envejecida | Barco | Mate, oscura, casi sin reflejo |
| 4 | Césped y flores | Praderas | Albedo colorido, specular bajo |
| 5 | Cristal pictórico | Monolito y fragmentos | Brillo, transparencia parcial |

---

## 7. Pendientes

| # | Pendiente | Bloquea |
|---|---|---|
| P-01 | Respuesta del profesor sobre el prisma hexagonal como primitiva propia | Nada. Ruta A (hexágonos reales) y Ruta B (cuboides verticales con textura) producen la misma silueta |
| P-02 | Marcar región por región qué es obligatorio y qué es opcional | Criterio 9 del punto 16, respondido solo parcialmente |
| P-03 | Validar D-08 (Monolito fuera del plano de corte) | Nada crítico |

---

## 8. Siguiente paso

**Inventario de objetos:** lista de cubos por región con posición, escala y material asignado. Es el documento que se traduce directo a Rust.

Después de eso viene el plan técnico del raytracer.
