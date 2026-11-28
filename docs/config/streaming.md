# Live Streaming

<div class="lang-en">

`LiveStream` is a bounded ring-buffer accumulator that turns a series of
`(x, y)` samples arriving over time into a continuously redrawn chart. Memory
stays constant whatever the duration of the stream, since older samples are
dropped past `max_points`.

What makes it smooth in a notebook: `push()` / `extend()` / `clear()` don't
just re-render — they repaint the **same** Jupyter output cell in place via
IPython's `display_id` mechanism (`display()` once, then `update_display()`
on every following call). No new cells get appended, no scroll-jump, no
clear-then-flash; the chart updates where it already is.

---

## Constructor

```python
sp.LiveStream(
    kind: str = "line",        # "line" or "scatter"
    title: str = "",
    max_points: int = 500,      # ring buffer size
    color_hex: int = 0x6366F1,
    width: int = 900,
    height: int = 420,
)
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `kind` | `str` | `"line"` | Chart kind. `"line"` or `"scatter"`. |
| `title` | `str` | `""` | Chart title rendered on every update. |
| `max_points` | `int` | `500` | Maximum samples kept in the buffer. Older samples are dropped from the head. |
| `color_hex` | `int` | `0x6366F1` | Series color as a 24-bit RGB integer. |
| `width` | `int` | `900` | Canvas width in pixels. |
| `height` | `int` | `420` | Canvas height in pixels. |

---

## Methods

| Method | Effect |
|--------|--------|
| `push(x, y)` | Append a single sample, then redraw in place. |
| `extend(xs, ys)` | Append two lists in lock-step, then redraw in place. |
| `clear()` | Empty the buffer, then redraw the (now empty) chart in place. |
| `render() -> Chart` | Render the current buffer to a standalone `Chart`, without touching the live display. |
| `n` (getter) | Current sample count. |
| `html` (getter) | Current chart HTML as a string. |

Every mutating call enforces the `max_points` cap by dropping the oldest
samples first — so the buffer is **always** bounded.

---

## Example: live IoT sensor dashboard in Jupyter

```python
import seraplot as sp
import time, random

stream = sp.LiveStream(kind="line", title="Temperature sensor (°C)", max_points=60)

value = 21.0
for tick in range(300):
    value += random.uniform(-0.4, 0.4)
    stream.push(tick, value)
    time.sleep(0.05)
```

Run this in a single notebook cell: the chart appears once, then updates
smoothly in place on every `push()` — no flicker, no new output cells.

See `v2/examples/iot_live_dashboard.py` for a fuller multi-sensor simulation.

</div>

<div class="lang-fr">

`LiveStream` est un accumulateur à buffer circulaire borné qui transforme une
série d'échantillons `(x, y)` arrivant dans le temps en un graphique redessiné
en continu. La mémoire reste constante quelle que soit la durée du flux,
puisque les échantillons les plus anciens sont supprimés au-delà de
`max_points`.

Ce qui le rend fluide dans un notebook : `push()` / `extend()` / `clear()` ne
font pas que re-rendre — ils repeignent la **même** cellule de sortie Jupyter
sur place via le mécanisme `display_id` d'IPython (`display()` une fois, puis
`update_display()` à chaque appel suivant). Aucune nouvelle cellule n'est
ajoutée, pas de saut de défilement, pas de clear-puis-flash ; le graphique se
met à jour là où il est déjà.

---

## Constructeur

```python
sp.LiveStream(
    kind: str = "line",        # "line" ou "scatter"
    title: str = "",
    max_points: int = 500,      # taille du buffer circulaire
    color_hex: int = 0x6366F1,
    width: int = 900,
    height: int = 420,
)
```

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `kind` | `str` | `"line"` | Type de graphique. `"line"` ou `"scatter"`. |
| `title` | `str` | `""` | Titre rendu à chaque mise à jour. |
| `max_points` | `int` | `500` | Nombre maximum d'échantillons gardés. Les plus anciens sont supprimés en tête. |
| `color_hex` | `int` | `0x6366F1` | Couleur de la série en entier RGB 24 bits. |
| `width` | `int` | `900` | Largeur du canvas en pixels. |
| `height` | `int` | `420` | Hauteur du canvas en pixels. |

---

## Méthodes

| Méthode | Effet |
|---------|-------|
| `push(x, y)` | Ajoute un échantillon unique, puis redessine sur place. |
| `extend(xs, ys)` | Ajoute deux listes en parallèle, puis redessine sur place. |
| `clear()` | Vide le buffer, puis redessine le graphique (désormais vide) sur place. |
| `render() -> Chart` | Rend le buffer courant sous forme de `Chart` autonome, sans toucher à l'affichage live. |
| `n` (getter) | Nombre d'échantillons courant. |
| `html` (getter) | HTML courant du graphique sous forme de chaîne. |

Chaque appel mutant applique la limite `max_points` en supprimant d'abord les
échantillons les plus anciens — le buffer est donc **toujours** borné.

---

## Exemple : tableau de bord IoT live dans Jupyter

```python
import seraplot as sp
import time, random

stream = sp.LiveStream(kind="line", title="Capteur de température (°C)", max_points=60)

value = 21.0
for tick in range(300):
    value += random.uniform(-0.4, 0.4)
    stream.push(tick, value)
    time.sleep(0.05)
```

Exécute ceci dans une seule cellule de notebook : le graphique apparaît une
fois, puis se met à jour de manière fluide sur place à chaque `push()` — sans
scintillement, sans nouvelle cellule de sortie.

Voir `v2/examples/iot_live_dashboard.py` pour une simulation multi-capteurs
plus complète.

</div>
