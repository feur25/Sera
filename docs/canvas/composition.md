# Canvas Composition

<div class="lang-en">

`Canvas` is SeraPlot's composition layer: a free-form surface where you place
multiple independently-built `Chart` objects, images, shapes and text, wire
them together, and export the whole thing as one `Chart`. It is the tool for
building dashboards, annotated stories and multi-panel figures that a single
chart function can't produce on its own.

```python
import seraplot as sp

bar = sp.grouped_bar("", labels=[...], values=[...], series_names=[...])
cv = sp.canvas(1280, 800, "#0a0a0f")
bars_ref = cv.place(bar, 60, 60, 560, 420, name="revenue")
chart = cv.build()
```

Every placement/drawing method accepts an optional `name` keyword. A named
element gets a `data-sp-name="..."` attribute in the rendered HTML, which is
what makes it addressable by every other method below (`nudge`, `resize`,
`style`, `script`, `group`, `link`, `refill`, dev-mode dragging, ...). Naming
is free and has no runtime cost — name anything you might want to touch again.

---

## Constructing a canvas

```python
cv = sp.canvas(width: int, height: int, bg: str = "#0a0a0f")
```

---

## Placing charts and images

| Method | Effect |
|--------|--------|
| `place(chart, x, y, w, h, rotation=0, opacity=1, clip="", group="", name="")` | Places a `Chart` on the canvas at `(x, y)` sized `w×h`. Returns a `chart_ref` (`int`) used by `pin`/`connect`/`attach_*`. |
| `image(src, x, y, w, h, rotation=0, opacity=1, clip="", group="", name="")` | Places a PNG/JPEG/GIF/WebP/SVG image. `src` is a local file path (read and base64-embedded), a `data:` URI, or an `http(s)://` URL. |
| `slot(name, x, y, w, h)` | Reserves a named region without placing anything yet. |
| `fill(slot_name, chart, ...)` | Places a chart using a previously-declared slot's geometry. |
| `grid(x, y, w, h, rows, cols, gap_x=0, gap_y=0)` | Declares a `rows × cols` grid of slots named `cell_{row}_{col}` and returns the list of names in row-major order — no manual coordinate math for dashboards. |
| `refill(name, chart) -> bool` | Replaces a **named, already-placed** chart's content in place — same position, size, rotation, style. Use this (not `place`/`fill` again) whenever you're refreshing a panel with new data. |

`clip` accepts `"circle"`, `"diamond"`, `"hex"`, `"tri"`, `"pentagon"` for
non-rectangular chart/image framing.

Calling `place`/`fill` again with a **name that's already taken by a placed
chart** updates that chart in place instead of stacking a duplicate — so
`fill(slot, new_chart, name="panel")` is safe to call repeatedly.

---

## Micro-tools: shared conventions

`text`, `line`, `curve`, `connector`, `circle`, `ring`, `rect`, `polygon`,
`path`, `arrow`, `annotate` and `gradient` are the low-level drawing
primitives everything else in `Canvas` (including `voronoi()`, below) is
built from. They draw directly on the canvas SVG, and share two keywords:

- `layer="fg"|"bg"` — `"bg"` renders under every `place()`d chart/image,
  `"fg"` (the default) renders on top. Use `"bg"` for backdrops, card
  panels and watermark-style decoration; `"fg"` for annotations, callouts
  and anything that should sit above the data.
- `name=""` — makes the element addressable afterward by `nudge`, `resize`,
  `style`, `script`, `group`, `link`, and draggable in `dev()` mode. Naming
  is free; name anything you might want to touch again.

None of them require a `Chart` — a canvas full of nothing but these
primitives is a valid way to hand-draw a diagram SeraPlot has no dedicated
chart function for (see the closing example on this page).

---

## Lines, curves & connectors

Four ways to draw *between* points — pick based on how many points and how
rigid the shape between them needs to be.

### `line` — straight segment

```python
cv.line(x1, y1, x2, y2, color="#ffffff", width=1.5, dash="", opacity=1.0,
         cap="round", layer="fg", hover_group="", name="")
```

| Parameter | Type | Default | Description |
|---|---|---|---|
| `x1, y1, x2, y2` | `float` | required | Endpoints, in canvas pixels |
| `color` | `str` | `"#ffffff"` | Stroke color |
| `width` | `float` | `1.5` | Stroke width |
| `dash` | `str` | `""` | SVG `stroke-dasharray`, e.g. `"6,4"` for a dashed line |
| `opacity` | `float` | `1.0` | 0–1 |
| `cap` | `str` | `"round"` | Line-end style: `"round"`, `"butt"`, or `"square"` |
| `hover_group` | `str` | `""` | Adds an invisible wide hit-area alongside the (often thin) visible stroke, so the line reacts to hover as part of a `link()` group |
| `name` | `str` | `""` | Addressable name |

The plain straight-line primitive — axes, dividers, guide lines, or one leg
of a hand-built diagram.

### `curve` — smooth line through N points

```python
cv.curve(points, color="#ffffff", width=1.5, opacity=1.0, tension=1.0,
          fill="none", layer="fg", name="")
```

| Parameter | Type | Default | Description |
|---|---|---|---|
| `points` | `list[[x, y]]` | required | Three or more waypoints the curve passes through |
| `tension` | `float` | `1.0` | Catmull-Rom tension: `0` collapses to straight segments between points, `1` is a standard smooth spline, higher values pull the curve into more pronounced bulges past each point |
| `fill` | `str` | `"none"` | Fills the area under the curve when set — a hand-drawn area-chart look |
| *(color / width / opacity / layer / name — same as `line`)* | | | |

Unlike `connector` (below), which always routes between exactly two points,
`curve` interpolates through an arbitrary polyline of waypoints. Reach for
it for hand-drawn trend lines, sparkline-style decoration, or free-form
organic strokes that aren't tied to a `Chart`'s own axes:

```python
cv.curve([[40, 300], [140, 120], [260, 260], [400, 80]],
          color="#22c55e", width=3, tension=0.8, name="trend-doodle")
```

### `connector` — S-curve between two points

```python
cv.connector(x1, y1, x2, y2, color="#ffffff", width=1.5, opacity=1.0,
              bend=0.5, layer="fg", name="")
```

| Parameter | Type | Default | Description |
|---|---|---|---|
| `bend` | `float` | `0.5` | Fraction along the dominant axis (whichever of dx/dy is larger) where the bezier control points sit — `0.5` gives a symmetric S-curve; values toward `0` or `1` skew the curve's midpoint toward one end |
| *(others — same as `line`)* | | | |

The "flowchart wire" primitive: one cubic bezier that always produces a
clean S- or L-shaped route between two points, regardless of their relative
position. Use it to link two `place()`d panels or two named elements
without hand-computing control points — `connect()` (under **Connecting
two charts**, below) draws the exact same curve, but reads its endpoints
from registered *pins* instead of raw coordinates.

### `arrow` — directional line with an arrowhead

```python
cv.arrow(x1, y1, x2, y2, color="#ffffff", width=1.5, head_size=4.0,
          opacity=1.0, layer="fg", name="")
```

| Parameter | Type | Default | Description |
|---|---|---|---|
| `head_size` | `float` | `4.0` | Arrowhead size in px — the marker scales with this, not with `width` |
| *(others — same as `line`)* | | | |

A `line` with an SVG `<marker>` arrowhead baked onto its end, for pointing
*at* something rather than just connecting two things.

---

## Shapes

Five ways to fill or stroke a region, from most constrained to most free-form.

### `circle` / `ring`

```python
cv.circle(cx, cy, r, fill="none", stroke="#ffffff", stroke_width=1.5,
           opacity=1.0, layer="fg", hover_group="", name="")
cv.ring(cx, cy, inner_r, outer_r, fill="#ffffff", stroke="none",
         stroke_width=0.0, opacity=1.0, layer="fg", name="")
```

`ring` is a donut: the filled region strictly between `inner_r` and
`outer_r`, built from two arcs combined with `fill-rule="evenodd"` rather
than a solid `<circle>`. Use it for radial progress rings, avatar frames,
or halo highlights — anywhere `circle`'s solid disc would cover whatever
sits underneath it.

### `rect`

```python
cv.rect(x, y, w, h, fill="none", stroke="#ffffff", stroke_width=1.5,
         rx=0.0, opacity=1.0, rotation=0.0, layer="fg", name="")
```

`rx` rounds the corners; `rotation` (degrees) spins the rect around its own
center. The two together cover card backgrounds, chips/badges, and simple
category-key swatches.

### `polygon`

```python
cv.polygon(points, fill="none", stroke="#ffffff", stroke_width=1.5,
            opacity=1.0, layer="fg", name="")
```

A closed shape through an arbitrary `list[[x, y]]` of vertices — the
primitive `voronoi()` itself is built from (each cell it returns is one
`polygon()` call under the hood). Use it directly for triangular/diamond
markers, custom badge shapes, or any closed region `rect`/`circle` can't
express.

### `path`

```python
cv.path(d, fill="none", stroke="#ffffff", stroke_width=1.5, opacity=1.0,
          layer="fg", name="")
```

The escape hatch: `d` is a raw SVG path-data string (`"M ... L ... A ... Z"`)
for shapes none of the other primitives cover — logos, icons, arcs with a
specific sweep, or geometry computed by your own code. This is exactly how
[`icicle()`](../charts/2d/icicle.md)'s `"radial"` variant draws its annular
sectors internally: hand-built `M`/`A`/`L`/`Z` strings, no separate arc
primitive needed.

---

## Text & annotations

### `text`

```python
cv.text(content, x, y, size=24.0, color="#ffffff", weight="normal",
          anchor="start", rotation=0.0, letter_spacing=0.0,
          font="sans-serif", opacity=1.0, layer="fg", name="")
```

`anchor` is the SVG text-anchor (`"start"`, `"middle"`, `"end"`) relative
to `(x, y)` — `"middle"` centers a title over a panel, `"end"` right-aligns
a value next to an axis.

### `annotate` — leader-line label

```python
cv.annotate(text, ax, ay, tx, ty, color="#ffffff", size=13.0,
             line_dash="", line_width=1.0, bg="", layer="fg", name="")
```

| Parameter | Type | Default | Description |
|---|---|---|---|
| `ax, ay` | `float` | required | The point being annotated — where the leader line starts |
| `tx, ty` | `float` | required | Where the text itself sits — where the leader line ends |
| `text` | `str` | required | Supports `\n` for multi-line labels |
| `line_dash` | `str` | `""` | Dash pattern for the leader line, e.g. `"4,3"` |
| `bg` | `str` | `""` | Background color behind the text; `""`/`"none"` draws no box |

Unlike a plain `text()` + `line()` pair, `annotate()` auto-routes a clean
two-segment elbow between `(ax, ay)` and `(tx, ty)` (picking the elbow
point from whichever axis has the larger offset) and sizes its own
background box to fit the text. The tool for "this specific point,
labeled, with a callout line" — a bar's peak, a scatter outlier.
`annotate_at()` (under **Connecting two charts**, below) is the
pin-aware version of this same primitive, for labeling a point inside a
`place()`d chart instead of a raw canvas coordinate.

---

## Color: `gradient`

```python
cv.gradient(id, from_color, to_color, x1=0.0, y1=0.0, x2=1.0, y2=0.0)
```

Registers an SVG `linearGradient` definition — `x1/y1/x2/y2` live in the
`0..1` objectBoundingBox space, so `(0,0)→(1,0)` is left-to-right and
`(0,0)→(0,1)` is top-to-bottom. It draws nothing by itself; call it once,
then reference `fill=f"url(#{id})"` on any subsequent `rect`/`circle`/
`polygon`/`path`:

```python
cv.gradient("card-glow", "#6366f1", "#0a0a0f", x1=0, y1=0, x2=0, y2=1)
cv.rect(40, 40, 300, 200, fill="url(#card-glow)", rx=18, name="card")
```

---

## Composing micro-tools: a radial dial

None of the primitives above need a `Chart` at all — a canvas built only
from them is a fully valid way to hand-draw a widget SeraPlot has no
dedicated chart function for. `ring` only ever draws a *complete* annulus,
so a partial-sweep progress dial needs `path` with a hand-computed SVG arc
— exactly the "escape hatch" role described above:

```python
import math
import seraplot as sp

def arc_path(cx, cy, r, pct):
    start = -math.pi / 2
    end = start + 2 * math.pi * pct
    x1, y1 = cx + r * math.cos(start), cy + r * math.sin(start)
    x2, y2 = cx + r * math.cos(end), cy + r * math.sin(end)
    large_arc = 1 if pct > 0.5 else 0
    return f"M {x1:.2f},{y1:.2f} A {r},{r} 0 {large_arc},1 {x2:.2f},{y2:.2f}"

cv = sp.Canvas(300, 300, bg="#0a0a0f")
cv.gradient("dial-g", "#6366f1", "#22d3ee", x1=0, y1=0, x2=1, y2=1)
cv.ring(150, 150, 100, 112, fill="#1e293b", name="track")
cv.path(arc_path(150, 150, 106, 0.72), fill="none", stroke="url(#dial-g)",
         stroke_width=12, name="progress")
cv.text("72%", 150, 158, size=34, color="#f8fafc", weight="800",
          anchor="middle", name="pct-label")
chart = cv.build()
```

`ring` draws the static background track, `path` draws the live progress
arc on top of it (stroked with the `gradient` defined a line earlier), and
`text` centers the number — three primitives from three different sections
above, one small self-contained gauge.

---

## Organic layouts: Voronoi

`voronoi(sites, x, y, w, h, fills=None, stroke=..., stroke_width=..., opacity=...)`
computes a bounded Voronoi diagram — one cell per site, each cell the region
closer to that site than to any other — and adds every cell to the canvas as
a `polygon()` in one call, returning their element indices for later
addressing (hover groups, `derive()`, etc.). This is the same organic,
cell-based layout technique behind editorial data-journalism pieces like
[Nadieh Bremer's Highly Hazardous Pesticides](https://www.visualcinnamon.com/portfolio/highly-hazardous-pesticides/)
— no off-the-shelf "voronoi chart type" needed, just the primitive.

```python
import random
cv = sp.Canvas(900, 540)
sites = [[random.uniform(30, 870), random.uniform(30, 510)] for _ in range(22)]
palette = ["#6366f1", "#ec4899", "#22c55e", "#f59e0b", "#06b6d4", "#8b5cf6", "#ef4444"]
fills = [palette[i % len(palette)] for i in range(len(sites))]
cv.voronoi(sites, 0, 0, 900, 540, fills=fills, stroke="#0d1117", stroke_width=2, opacity=0.88)
```

<iframe src="../previews/canvas-voronoi.html" style="width:100%;height:420px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

Cell size follows site density automatically — cluster sites tightly to
shrink their cells, useful for a treemap-like "one cell per record, colored
by category, sized by local density" layout without a separate packing
algorithm. Implemented natively (iterative half-plane clipping against
every other site, no external geometry crate) rather than pulled in as a
dependency.

---

## Custom CSS / JS

| Method | Effect |
|--------|--------|
| `style(name, css)` | Injects `[data-sp-name="name"]{ css }` into the canvas's `<style>`. Pass `name=""` to inject a raw, unscoped CSS block (e.g. `@keyframes`). |
| `script(js)` | Appends a raw `<script>js</script>` before `</body>` — full manual control for users who want to hand-write interactivity. |

---

## Groups and inter-plot linking

Two different mechanisms, both driven by element names:

**`group(group_name, member_names)` / `move_group(group_name, dx, dy)`** —
moves several named elements together as a rigid unit. `nudge(name, dx, dy)`
and `resize(name, dw, dh)` do the same for a single element. Pins registered
on a chart before a move/resize are shifted along with it automatically.

**`link(group_name, member_names) -> int`** — ties elements **across
different panels** into one hover group: hovering *any* linked element (a
`Chart`, `Rect`, `Text` or `Circle`) glows/pulses all the others in the same
group. Returns how many of the given names were actually linkable (`Line`,
`RawPath` and other pure decoration types don't currently support it).

```python
cv.link("story", ["revenue_chart", "trend_chart", "kpi_card"])
```

---

## Connecting two charts (pins)

Pins are named anchor points registered *inside* a placed chart's coordinate
space, in canvas pixel coordinates. `connect()`/`annotate_at()` read pins to
draw a line or label between (or on top of) charts.

| Method | Effect |
|--------|--------|
| `pin(chart_ref, name, local_x, local_y)` | Registers a pin at a chart-local pixel coordinate. |
| `pin_frac(chart_ref, name, fx, fy)` | Registers a pin at a fractional position (`0..1`) of the chart's native size. |
| `pin_xy(chart_ref, name) -> (x, y) | None` | Reads back a pin's canvas coordinates. |
| `attach_bar(chart_ref, values, chart_w, chart_h, ...)` | Auto-registers `bar:{i}:top/center/bottom/left/right` pins by reading the actual rendered bar rectangles. |
| `attach_scatter(chart_ref, x_vals, y_vals, labels, chart_w, chart_h, ...)` | Auto-registers `point:{i}` (and named) pins from the data's projected positions. |
| `connect(from_ref, from_name, to_ref, to_name, ...)` | Draws a curved connector between two pins, possibly on two different charts. |
| `annotate_at(chart_ref, pin_name, text, ...)` | Draws a leader-line label pointing at a pin. |

**Pins go stale when the geometry they were computed from changes.**
`refill()` on a chart clears its pins (so you don't silently connect to
coordinates that belonged to the old content) — re-pin after refilling if you
still need them. `nudge`/`resize`/`move_group`, on the other hand, *do* shift
existing pins automatically, since the underlying content hasn't changed.

---

## Reusable skeletons: template & derive

```python
skeleton = base_canvas.template()   # strip Chart/Image elements, keep everything else
dashboard = skeleton.derive()       # deep-clone a fresh instance to fill in
dashboard.fill("main", my_chart, name="panel")
```

`template()` returns a canvas with all `place()`d charts and `image()`s
removed but every decorative element (cards, gradients, titles, slots,
groups, custom CSS/JS) intact — the reusable "class". `derive()` deep-clones
*any* canvas (templated or not) into an independent instance — the
"constructor call". Build your branded skeleton once, `derive()` + `fill()`
it per dataset/variant instead of repeating layout code.

---

## Persistence

| Method | Effect |
|--------|--------|
| `save(path)` | Serializes the full canvas state (elements, pins, groups, slots, custom CSS/JS) to JSON. |
| `sp.canvas_load(path) -> Canvas` | Rebuilds a canvas from a saved JSON file. |
| `sp.canvas_save_named(cv, name) -> str` | Saves under `~/.seraplot/canvas/{name}.json` and updates an `index.json` manifest. |
| `sp.canvas_load_named(name) -> Canvas` | Loads back via that manifest. |
| `to_json() -> str` | The raw JSON string, if you want to manage storage yourself. |

This is what lets a generated dashboard survive closing and reopening the
app: `cv.save(...)` once, `sp.canvas_load(...)` next session reconstructs an
identical canvas — positions, links, styling, everything.

---

## Interactive dev mode

```python
cv.dev()
```

Renders the canvas with a floating panel: drag any named element to move it,
drag the corner handle on charts/images to resize them, hover shows the
element's name and its linked group (if any). The panel's **Copy Python**
button generates the equivalent `cv.nudge(...)`/`cv.resize(...)` calls;
**Download JSON** exports the same deltas to a file that `apply_deltas_json()`
can replay headlessly (`cv.apply_deltas_json(open(path).read())`) — the
route from interactive tweaking to a reproducible script.

</div>

<div class="lang-fr">

`Canvas` est la couche de composition de SeraPlot : une surface libre où l'on
place plusieurs `Chart` construits indépendamment, des images, des formes et
du texte, où on les relie entre eux, puis on exporte le tout comme un seul
`Chart`. C'est l'outil pour construire des dashboards, des histoires
annotées et des figures multi-panneaux qu'une seule fonction de chart ne
peut pas produire seule.

```python
import seraplot as sp

bar = sp.grouped_bar("", labels=[...], values=[...], series_names=[...])
cv = sp.canvas(1280, 800, "#0a0a0f")
bars_ref = cv.place(bar, 60, 60, 560, 420, name="revenue")
chart = cv.build()
```

Chaque méthode de placement/dessin accepte un mot-clé `name` optionnel. Un
élément nommé reçoit un attribut `data-sp-name="..."` dans le HTML généré,
ce qui le rend adressable par toutes les autres méthodes ci-dessous
(`nudge`, `resize`, `style`, `script`, `group`, `link`, `refill`, le
glisser-déposer du mode dev, ...). Nommer est gratuit et sans coût
d'exécution — nommez tout ce que vous pourriez vouloir retoucher.

---

## Créer un canvas

```python
cv = sp.canvas(width: int, height: int, bg: str = "#0a0a0f")
```

---

## Placer des charts et des images

| Méthode | Effet |
|--------|--------|
| `place(chart, x, y, w, h, rotation=0, opacity=1, clip="", group="", name="")` | Place un `Chart` sur le canvas à `(x, y)` de taille `w×h`. Renvoie un `chart_ref` (`int`) utilisé par `pin`/`connect`/`attach_*`. |
| `image(src, x, y, w, h, rotation=0, opacity=1, clip="", group="", name="")` | Place une image PNG/JPEG/GIF/WebP/SVG. `src` est un chemin de fichier local (lu et encodé en base64), une URI `data:`, ou une URL `http(s)://`. |
| `slot(name, x, y, w, h)` | Réserve une région nommée sans encore rien y placer. |
| `fill(slot_name, chart, ...)` | Place un chart en utilisant la géométrie d'un slot déclaré au préalable. |
| `grid(x, y, w, h, rows, cols, gap_x=0, gap_y=0)` | Déclare une grille `rows × cols` de slots nommés `cell_{row}_{col}` et renvoie la liste des noms en ordre ligne par ligne — plus de calcul de coordonnées à la main pour un dashboard. |
| `refill(name, chart) -> bool` | Remplace le contenu d'un chart **nommé et déjà placé**, en conservant position, taille, rotation, style. À utiliser (au lieu de rappeler `place`/`fill`) chaque fois que vous rafraîchissez un panneau avec de nouvelles données. |

`clip` accepte `"circle"`, `"diamond"`, `"hex"`, `"tri"`, `"pentagon"` pour
un cadrage non rectangulaire du chart/de l'image.

Rappeler `place`/`fill` avec un **nom déjà pris par un chart placé** met à
jour ce chart en place au lieu d'empiler un doublon — `fill(slot, new_chart,
name="panel")` peut donc être rappelé sans risque.

---

## Micro-outils : conventions communes

`text`, `line`, `curve`, `connector`, `circle`, `ring`, `rect`, `polygon`,
`path`, `arrow`, `annotate` et `gradient` sont les primitives de dessin bas
niveau à partir desquelles tout le reste de `Canvas` (y compris `voronoi()`,
plus bas) est construit. Elles dessinent directement sur le SVG du canvas,
et partagent deux mots-clés :

- `layer="fg"|"bg"` — `"bg"` s'affiche sous chaque chart/image `place()`é,
  `"fg"` (par défaut) s'affiche au-dessus. Utilisez `"bg"` pour les fonds,
  panneaux-cartes et décorations façon filigrane ; `"fg"` pour les
  annotations, callouts et tout ce qui doit rester au-dessus des données.
- `name=""` — rend l'élément adressable ensuite par `nudge`, `resize`,
  `style`, `script`, `group`, `link`, et déplaçable au glisser-déposer en
  mode `dev()`. Nommer est gratuit ; nommez tout ce que vous pourriez
  vouloir retoucher.

Aucune de ces primitives ne nécessite un `Chart` — un canvas ne contenant
que ces primitives est une façon parfaitement valide de dessiner à la main
un diagramme pour lequel SeraPlot n'a pas de fonction de chart dédiée (voir
l'exemple de clôture de cette page).

---

## Lignes, courbes & connecteurs

Quatre façons de dessiner *entre* des points — le choix dépend du nombre de
points et de la rigidité voulue pour la forme qui les relie.

### `line` — segment droit

```python
cv.line(x1, y1, x2, y2, color="#ffffff", width=1.5, dash="", opacity=1.0,
         cap="round", layer="fg", hover_group="", name="")
```

| Paramètre | Type | Défaut | Description |
|---|---|---|---|
| `x1, y1, x2, y2` | `float` | requis | Extrémités, en pixels canvas |
| `color` | `str` | `"#ffffff"` | Couleur du trait |
| `width` | `float` | `1.5` | Épaisseur du trait |
| `dash` | `str` | `""` | `stroke-dasharray` SVG, ex. `"6,4"` pour un trait pointillé |
| `opacity` | `float` | `1.0` | 0–1 |
| `cap` | `str` | `"round"` | Style d'extrémité : `"round"`, `"butt"`, ou `"square"` |
| `hover_group` | `str` | `""` | Ajoute une zone de survol invisible plus large que le trait visible (souvent fin), pour que la ligne réagisse au survol en tant que membre d'un groupe `link()` |
| `name` | `str` | `""` | Nom adressable |

La primitive ligne droite la plus simple — axes, séparateurs, lignes
guides, ou un segment d'un diagramme construit à la main.

### `curve` — ligne lissée passant par N points

```python
cv.curve(points, color="#ffffff", width=1.5, opacity=1.0, tension=1.0,
          fill="none", layer="fg", name="")
```

| Paramètre | Type | Défaut | Description |
|---|---|---|---|
| `points` | `list[[x, y]]` | requis | Trois points ou plus par lesquels la courbe passe |
| `tension` | `float` | `1.0` | Tension Catmull-Rom : `0` réduit la courbe à des segments droits entre les points, `1` donne une spline lissée standard, des valeurs plus hautes accentuent les bombements après chaque point |
| `fill` | `str` | `"none"` | Remplit la zone sous la courbe si défini — effet aire dessinée à la main |
| *(color / width / opacity / layer / name — comme `line`)* | | | |

Contrairement à `connector` (ci-dessous), qui relie toujours exactement
deux points, `curve` interpole à travers une polyligne arbitraire de
points de passage. À utiliser pour des lignes de tendance dessinées à la
main, des décorations façon sparkline, ou des traits organiques libres non
liés aux axes d'un `Chart` :

```python
cv.curve([[40, 300], [140, 120], [260, 260], [400, 80]],
          color="#22c55e", width=3, tension=0.8, name="trend-doodle")
```

### `connector` — courbe en S entre deux points

```python
cv.connector(x1, y1, x2, y2, color="#ffffff", width=1.5, opacity=1.0,
              bend=0.5, layer="fg", name="")
```

| Paramètre | Type | Défaut | Description |
|---|---|---|---|
| `bend` | `float` | `0.5` | Fraction, le long de l'axe dominant (celui de dx/dy le plus grand), où se placent les points de contrôle de la bézier — `0.5` donne une courbe en S symétrique ; des valeurs vers `0` ou `1` décalent le milieu de la courbe vers une extrémité |
| *(autres — comme `line`)* | | | |

La primitive « fil de flowchart » : une seule bézier cubique qui produit
toujours un tracé propre en S ou en L entre deux points, quelle que soit
leur position relative. À utiliser pour relier deux panneaux `place()`és
ou deux éléments nommés sans calculer les points de contrôle à la main —
`connect()` (sous **Connecter deux charts**, plus bas) trace exactement la
même courbe, mais lit ses extrémités depuis des *pins* enregistrés plutôt
que des coordonnées brutes.

### `arrow` — ligne directionnelle avec pointe de flèche

```python
cv.arrow(x1, y1, x2, y2, color="#ffffff", width=1.5, head_size=4.0,
          opacity=1.0, layer="fg", name="")
```

| Paramètre | Type | Défaut | Description |
|---|---|---|---|
| `head_size` | `float` | `4.0` | Taille de la pointe en px — le marqueur suit cette valeur, pas `width` |
| *(autres — comme `line`)* | | | |

Une `line` avec une pointe de flèche SVG (`<marker>`) ajoutée à son
extrémité, pour pointer *vers* quelque chose plutôt que simplement relier
deux points.

---

## Formes

Cinq façons de remplir ou tracer une région, de la plus contrainte à la
plus libre.

### `circle` / `ring`

```python
cv.circle(cx, cy, r, fill="none", stroke="#ffffff", stroke_width=1.5,
           opacity=1.0, layer="fg", hover_group="", name="")
cv.ring(cx, cy, inner_r, outer_r, fill="#ffffff", stroke="none",
         stroke_width=0.0, opacity=1.0, layer="fg", name="")
```

`ring` est un anneau (donut) : la région remplie strictement entre
`inner_r` et `outer_r`, construite à partir de deux arcs combinés avec
`fill-rule="evenodd"` plutôt qu'un `<circle>` plein. À utiliser pour des
anneaux de progression radiale, des cadres d'avatar, ou des halos de mise
en valeur — partout où le disque plein de `circle` masquerait ce qu'il y a
en dessous.

### `rect`

```python
cv.rect(x, y, w, h, fill="none", stroke="#ffffff", stroke_width=1.5,
         rx=0.0, opacity=1.0, rotation=0.0, layer="fg", name="")
```

`rx` arrondit les coins ; `rotation` (en degrés) fait pivoter le rectangle
autour de son propre centre. Les deux ensemble couvrent les fonds de
carte, badges/puces, et échantillons de légende de catégorie.

### `polygon`

```python
cv.polygon(points, fill="none", stroke="#ffffff", stroke_width=1.5,
            opacity=1.0, layer="fg", name="")
```

Une forme fermée à travers une liste arbitraire `list[[x, y]]` de
sommets — la primitive à partir de laquelle `voronoi()` elle-même est
construite (chaque cellule qu'elle renvoie est un appel `polygon()` en
coulisses). À utiliser directement pour des marqueurs triangulaires/en
losange, des formes de badge personnalisées, ou toute région fermée que
`rect`/`circle` ne peuvent pas exprimer.

### `path`

```python
cv.path(d, fill="none", stroke="#ffffff", stroke_width=1.5, opacity=1.0,
          layer="fg", name="")
```

L'échappatoire : `d` est une chaîne de données de chemin SVG brute
(`"M ... L ... A ... Z"`) pour les formes qu'aucune autre primitive ne
couvre — logos, icônes, arcs avec un balayage spécifique, ou géométrie
calculée par votre propre code. C'est exactement ainsi que la variante
`"radial"` d'[`icicle()`](../charts/2d/icicle.md) dessine ses secteurs
annulaires en interne : des chaînes `M`/`A`/`L`/`Z` construites à la main,
sans primitive d'arc séparée.

---

## Texte & annotations

### `text`

```python
cv.text(content, x, y, size=24.0, color="#ffffff", weight="normal",
          anchor="start", rotation=0.0, letter_spacing=0.0,
          font="sans-serif", opacity=1.0, layer="fg", name="")
```

`anchor` est le text-anchor SVG (`"start"`, `"middle"`, `"end"`) relatif à
`(x, y)` — `"middle"` centre un titre au-dessus d'un panneau, `"end"`
aligne une valeur à droite le long d'un axe.

### `annotate` — étiquette avec ligne de rappel

```python
cv.annotate(text, ax, ay, tx, ty, color="#ffffff", size=13.0,
             line_dash="", line_width=1.0, bg="", layer="fg", name="")
```

| Paramètre | Type | Défaut | Description |
|---|---|---|---|
| `ax, ay` | `float` | requis | Le point annoté — où commence la ligne de rappel |
| `tx, ty` | `float` | requis | Où se trouve le texte lui-même — où finit la ligne de rappel |
| `text` | `str` | requis | Supporte `\n` pour des étiquettes multi-lignes |
| `line_dash` | `str` | `""` | Motif de tirets pour la ligne de rappel, ex. `"4,3"` |
| `bg` | `str` | `""` | Couleur de fond derrière le texte ; `""`/`"none"` ne dessine aucun cadre |

Contrairement à une paire `text()` + `line()`, `annotate()` route
automatiquement un coude propre en deux segments entre `(ax, ay)` et
`(tx, ty)` (le point de coude choisi selon l'axe ayant le plus grand
écart) et dimensionne son propre cadre de fond pour s'ajuster au texte.
L'outil pour « ce point précis, étiqueté, avec une ligne d'appel » — le
pic d'une barre, un point aberrant sur un scatter. `annotate_at()` (sous
**Connecter deux charts**, plus bas) est la version « pin-aware » de cette
même primitive, pour étiqueter un point à l'intérieur d'un chart `place()`é
plutôt qu'une coordonnée canvas brute.

---

## Couleur : `gradient`

```python
cv.gradient(id, from_color, to_color, x1=0.0, y1=0.0, x2=1.0, y2=0.0)
```

Enregistre une définition `linearGradient` SVG — `x1/y1/x2/y2` vivent dans
l'espace objectBoundingBox `0..1`, donc `(0,0)→(1,0)` va de gauche à
droite et `(0,0)→(0,1)` de haut en bas. Ne dessine rien par elle-même ;
appelez-la une fois, puis référencez `fill=f"url(#{id})"` sur n'importe
quel `rect`/`circle`/`polygon`/`path` suivant :

```python
cv.gradient("card-glow", "#6366f1", "#0a0a0f", x1=0, y1=0, x2=0, y2=1)
cv.rect(40, 40, 300, 200, fill="url(#card-glow)", rx=18, name="card")
```

---

## Composer les micro-outils : un cadran radial

Aucune des primitives ci-dessus n'a besoin d'un `Chart` — un canvas
construit uniquement à partir d'elles est une façon parfaitement valide de
dessiner à la main un widget pour lequel SeraPlot n'a pas de fonction de
chart dédiée. `ring` ne dessine jamais qu'un anneau *complet*, donc un
cadran de progression à balayage partiel nécessite `path` avec un arc SVG
calculé à la main — exactement le rôle d'« échappatoire » décrit plus haut :

```python
import math
import seraplot as sp

def arc_path(cx, cy, r, pct):
    start = -math.pi / 2
    end = start + 2 * math.pi * pct
    x1, y1 = cx + r * math.cos(start), cy + r * math.sin(start)
    x2, y2 = cx + r * math.cos(end), cy + r * math.sin(end)
    large_arc = 1 if pct > 0.5 else 0
    return f"M {x1:.2f},{y1:.2f} A {r},{r} 0 {large_arc},1 {x2:.2f},{y2:.2f}"

cv = sp.Canvas(300, 300, bg="#0a0a0f")
cv.gradient("dial-g", "#6366f1", "#22d3ee", x1=0, y1=0, x2=1, y2=1)
cv.ring(150, 150, 100, 112, fill="#1e293b", name="track")
cv.path(arc_path(150, 150, 106, 0.72), fill="none", stroke="url(#dial-g)",
         stroke_width=12, name="progress")
cv.text("72%", 150, 158, size=34, color="#f8fafc", weight="800",
          anchor="middle", name="pct-label")
chart = cv.build()
```

`ring` dessine la piste de fond statique, `path` dessine par-dessus l'arc
de progression réel (tracé avec le `gradient` défini juste avant), et
`text` centre le nombre — trois primitives issues de trois sections
différentes ci-dessus, une seule jauge autonome.

---

## Mises en page organiques : Voronoi

`voronoi(sites, x, y, w, h, fills=None, stroke=..., stroke_width=..., opacity=...)`
calcule un diagramme de Voronoi borné — une cellule par site, chaque cellule
étant la région plus proche de ce site que de tout autre — et ajoute chaque
cellule au canvas comme un `polygon()` en un seul appel, en renvoyant leurs
indices d'éléments pour un adressage ultérieur (groupes de survol,
`derive()`, etc.). C'est la même technique de mise en page organique par
cellules derrière des pièces éditoriales de data-journalisme comme
[Highly Hazardous Pesticides de Nadieh Bremer](https://www.visualcinnamon.com/portfolio/highly-hazardous-pesticides/)
— pas besoin d'un "type de chart voronoi" tout fait, juste la primitive.

```python
import random
cv = sp.Canvas(900, 540)
sites = [[random.uniform(30, 870), random.uniform(30, 510)] for _ in range(22)]
palette = ["#6366f1", "#ec4899", "#22c55e", "#f59e0b", "#06b6d4", "#8b5cf6", "#ef4444"]
fills = [palette[i % len(palette)] for i in range(len(sites))]
cv.voronoi(sites, 0, 0, 900, 540, fills=fills, stroke="#0d1117", stroke_width=2, opacity=0.88)
```

<iframe src="../previews/canvas-voronoi.html" style="width:100%;height:420px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

La taille des cellules suit automatiquement la densité des sites — resserrer
des sites rétrécit leurs cellules, utile pour une mise en page façon treemap
("une cellule par enregistrement, colorée par catégorie, dimensionnée par
densité locale") sans algorithme de packing séparé. Implémenté nativement
(découpage itératif par demi-plans contre chaque autre site, aucune
dépendance de géométrie externe).

---

## CSS / JS custom

| Méthode | Effet |
|--------|--------|
| `style(name, css)` | Injecte `[data-sp-name="name"]{ css }` dans le `<style>` du canvas. Passer `name=""` pour injecter un bloc CSS brut non scopé (ex. `@keyframes`). |
| `script(js)` | Ajoute un `<script>js</script>` brut avant `</body>` — contrôle manuel complet pour qui veut écrire son interactivité à la main. |

---

## Groupes et liaison inter-plot

Deux mécanismes distincts, tous deux pilotés par les noms d'éléments :

**`group(group_name, member_names)` / `move_group(group_name, dx, dy)`** —
déplace plusieurs éléments nommés ensemble comme un bloc rigide. `nudge(name,
dx, dy)` et `resize(name, dw, dh)` font pareil pour un seul élément. Les
pins enregistrés sur un chart avant un déplacement/redimensionnement sont
automatiquement décalés avec lui.

**`link(group_name, member_names) -> int`** — relie des éléments **à
travers des panneaux différents** en un seul groupe de survol : survoler
*n'importe quel* élément lié (`Chart`, `Rect`, `Text` ou `Circle`) fait
briller/pulser tous les autres du même groupe. Renvoie le nombre de noms
effectivement liables (`Line`, `RawPath` et autres types purement
décoratifs ne le supportent pas encore).

```python
cv.link("story", ["revenue_chart", "trend_chart", "kpi_card"])
```

---

## Connecter deux charts (pins)

Les pins sont des points d'ancrage nommés enregistrés *dans* l'espace de
coordonnées d'un chart placé, en coordonnées pixel du canvas.
`connect()`/`annotate_at()` lisent les pins pour tracer une ligne ou une
étiquette entre (ou par-dessus) des charts.

| Méthode | Effet |
|--------|--------|
| `pin(chart_ref, name, local_x, local_y)` | Enregistre un pin à une coordonnée pixel locale au chart. |
| `pin_frac(chart_ref, name, fx, fy)` | Enregistre un pin à une position fractionnaire (`0..1`) de la taille native du chart. |
| `pin_xy(chart_ref, name) -> (x, y) | None` | Relit les coordonnées canvas d'un pin. |
| `attach_bar(chart_ref, values, chart_w, chart_h, ...)` | Enregistre automatiquement les pins `bar:{i}:top/center/bottom/left/right` en lisant les rectangles de barres réellement rendus. |
| `attach_scatter(chart_ref, x_vals, y_vals, labels, chart_w, chart_h, ...)` | Enregistre automatiquement les pins `point:{i}` (et nommés) à partir des positions projetées des données. |
| `connect(from_ref, from_name, to_ref, to_name, ...)` | Trace un connecteur courbe entre deux pins, éventuellement sur deux charts différents. |
| `annotate_at(chart_ref, pin_name, text, ...)` | Trace une étiquette avec ligne de rappel pointant vers un pin. |

**Les pins deviennent obsolètes quand la géométrie qui les a produits
change.** `refill()` sur un chart efface ses pins (pour ne pas connecter
silencieusement vers des coordonnées appartenant à l'ancien contenu) —
re-pinnez après un refill si vous en avez encore besoin. `nudge`/`resize`/
`move_group`, en revanche, décalent bien les pins existants automatiquement,
puisque le contenu sous-jacent n'a pas changé.

---

## Squelettes réutilisables : template & derive

```python
skeleton = base_canvas.template()   # retire les Chart/Image, garde le reste
dashboard = skeleton.derive()       # clone profond d'une instance prête à remplir
dashboard.fill("main", my_chart, name="panel")
```

`template()` renvoie un canvas dont tous les charts `place()`és et images
`image()`ées sont retirés, mais où chaque élément décoratif (cartes,
dégradés, titres, slots, groupes, CSS/JS custom) reste intact — la "classe"
réutilisable. `derive()` clone en profondeur **n'importe quel** canvas
(templatisé ou non) en une instance indépendante — "l'instanciation".
Construisez votre squelette de marque une fois, puis `derive()` + `fill()`
par jeu de données/variante au lieu de répéter le code de mise en page.

---

## Persistance

| Méthode | Effet |
|--------|--------|
| `save(path)` | Sérialise tout l'état du canvas (éléments, pins, groupes, slots, CSS/JS custom) en JSON. |
| `sp.canvas_load(path) -> Canvas` | Reconstruit un canvas depuis un fichier JSON sauvegardé. |
| `sp.canvas_save_named(cv, name) -> str` | Sauvegarde sous `~/.seraplot/canvas/{name}.json` et met à jour un manifeste `index.json`. |
| `sp.canvas_load_named(name) -> Canvas` | Recharge via ce manifeste. |
| `to_json() -> str` | La chaîne JSON brute, pour gérer soi-même le stockage. |

C'est ce qui permet à un dashboard généré de survivre à la fermeture et à la
réouverture de l'application : `cv.save(...)` une fois, `sp.canvas_load(...)`
à la session suivante reconstruit un canvas identique — positions, liens,
style, tout.

---

## Mode dev interactif

```python
cv.dev()
```

Rend le canvas avec un panneau flottant : glissez n'importe quel élément
nommé pour le déplacer, glissez la poignée en coin des charts/images pour
les redimensionner, le survol affiche le nom de l'élément et son groupe lié
(le cas échéant). Le bouton **Copy Python** du panneau génère les appels
`cv.nudge(...)`/`cv.resize(...)` équivalents ; **Download JSON** exporte les
mêmes deltas dans un fichier que `apply_deltas_json()` peut rejouer sans
interface (`cv.apply_deltas_json(open(path).read())`) — le chemin entre
ajustement interactif et script reproductible.

</div>
