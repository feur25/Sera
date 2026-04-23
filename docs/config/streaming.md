# Live Streaming

<div class="lang-en">

`LiveStream` is a tiny, allocation-aware accumulator that turns a series of
`(x, y)` samples arriving over time into a fully-rendered SeraPlot `Chart`.
It maintains a bounded ring buffer (the `max_points` window) so memory is
constant whatever the duration of the stream.

The same engine is exposed in JavaScript as `chartAppend(...)`, but Python
gets the dedicated stateful class because it's by far the most common
language for live data pipelines (sensors, sockets, message buses, ML training
loops, etc.).

---

## Constructor

```python
sp.LiveStream(
    kind: str = "line",        # "line" or "scatter"
    title: str = "",
    max_points: int = 500,     # ring buffer size
    color_hex: int = 0x6366F1,
    width: int = 900,
    height: int = 420,
)
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `kind` | `str` | `"line"` | Chart kind. `"line"` (categorical X) or `"scatter"` (numeric X). |
| `title` | `str` | `""` | Chart title rendered on every `render()`. |
| `max_points` | `int` | `500` | Maximum samples kept in the buffer. Older samples are dropped from the head. |
| `color_hex` | `int` | `0x6366F1` | Series colour as a 24-bit RGB integer. |
| `width` | `int` | `900` | Canvas width in pixels. |
| `height` | `int` | `420` | Canvas height in pixels. |

---

## Methods

| Method | Effect |
|--------|--------|
| `push(x, y)` | Append a single sample. |
| `extend(xs, ys)` | Append two lists in lock-step. |
| `render() -> Chart` | Re-render the current buffer and return a fresh `Chart`. |
| `clear()` | Empty the buffer. |
| `n` (getter) | Current sample count. |

Every mutating call enforces the `max_points` cap by dropping the oldest
samples — so the buffer is **always** bounded.

---

## Example: Jupyter live plot

```python
import seraplot as sp
import time, random
from IPython.display import display, clear_output

stream = sp.LiveStream(kind="line", title="Sensor", max_points=200)

for t in range(2000):
    stream.push(t, 50 + 10 * random.gauss(0, 1))
    if t % 20 == 0:
        clear_output(wait=True)
        display(stream.render())
        time.sleep(0.05)
```

---

## Example: WebSocket / Kafka feed

```python
async def consume(ws):
    stream = sp.LiveStream(kind="scatter", max_points=10_000)
    async for msg in ws:
        x, y = parse(msg)
        stream.push(x, y)
        if stream.n % 250 == 0:
            broadcast_chart(stream.render())
```

---

## Underlying primitive (universal)

Internally `LiveStream.render()` calls the universal `chart_append` function.
The same primitive is reachable from JavaScript and the C-FFI:

```js
import * as sp from "seraplot";

let prev_x = [], prev_y = [];

function tick(x, y) {
  const out = JSON.parse(sp.chartAppend(JSON.stringify({
    kind: "line",
    title: "Sensor",
    x: [x], y: [y],
    prev_x, prev_y,
    max_points: 500,
  })));
  prev_x = out.x;
  prev_y = out.y;
  document.getElementById("plot").innerHTML = out.html;
}
```

The JSON payload `{kind, x, y, prev_x, prev_y, max_points, title, color_hex,
width, height}` is the contract — the same shape applies in any host
language.

</div>

<div class="lang-fr">

`LiveStream` est un petit accumulateur économe en allocation qui transforme
une série d'échantillons `(x, y)` arrivant dans le temps en un `Chart`
SeraPlot entièrement rendu. Il maintient un buffer circulaire borné (la
fenêtre `max_points`), donc la mémoire reste constante quelle que soit la
durée du flux.

Le même moteur est exposé en JavaScript via `chartAppend(...)`, mais Python
obtient une classe à état dédiée parce que c'est de loin le langage le plus
utilisé pour les pipelines de données live (capteurs, sockets, bus de
messages, boucles d'entraînement ML, etc.).

---

## Constructeur

```python
sp.LiveStream(
    kind: str = "line",        # "line" ou "scatter"
    title: str = "",
    max_points: int = 500,     # taille du buffer circulaire
    color_hex: int = 0x6366F1,
    width: int = 900,
    height: int = 420,
)
```

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `kind` | `str` | `"line"` | Type de graphique. `"line"` (X catégoriel) ou `"scatter"` (X numérique). |
| `title` | `str` | `""` | Titre rendu à chaque `render()`. |
| `max_points` | `int` | `500` | Nombre maximum d'échantillons gardés. Les plus anciens sont supprimés en tête. |
| `color_hex` | `int` | `0x6366F1` | Couleur de la série en entier RGB 24 bits. |
| `width` | `int` | `900` | Largeur du canvas en pixels. |
| `height` | `int` | `420` | Hauteur du canvas en pixels. |

---

## Méthodes

| Méthode | Effet |
|---------|-------|
| `push(x, y)` | Ajoute un échantillon unique. |
| `extend(xs, ys)` | Ajoute deux listes en parallèle. |
| `render() -> Chart` | Re-rend le buffer courant et retourne un nouveau `Chart`. |
| `clear()` | Vide le buffer. |
| `n` (getter) | Nombre d'échantillons courant. |

Chaque appel mutant applique la limite `max_points` en supprimant les
échantillons les plus anciens — le buffer est donc **toujours** borné.

---

## Exemple : plot live dans Jupyter

```python
import seraplot as sp
import time, random
from IPython.display import display, clear_output

stream = sp.LiveStream(kind="line", title="Capteur", max_points=200)

for t in range(2000):
    stream.push(t, 50 + 10 * random.gauss(0, 1))
    if t % 20 == 0:
        clear_output(wait=True)
        display(stream.render())
        time.sleep(0.05)
```

---

## Exemple : flux WebSocket / Kafka

```python
async def consume(ws):
    stream = sp.LiveStream(kind="scatter", max_points=10_000)
    async for msg in ws:
        x, y = parse(msg)
        stream.push(x, y)
        if stream.n % 250 == 0:
            broadcast_chart(stream.render())
```

---

## Primitive sous-jacente (universelle)

En interne, `LiveStream.render()` appelle la fonction universelle
`chart_append`. La même primitive est accessible depuis JavaScript et le
C-FFI :

```js
import * as sp from "seraplot";

let prev_x = [], prev_y = [];

function tick(x, y) {
  const out = JSON.parse(sp.chartAppend(JSON.stringify({
    kind: "line",
    title: "Capteur",
    x: [x], y: [y],
    prev_x, prev_y,
    max_points: 500,
  })));
  prev_x = out.x;
  prev_y = out.y;
  document.getElementById("plot").innerHTML = out.html;
}
```

Le payload JSON `{kind, x, y, prev_x, prev_y, max_points, title, color_hex,
width, height}` est le contrat — la même forme s'applique dans n'importe
quelle langue hôte.

</div>
