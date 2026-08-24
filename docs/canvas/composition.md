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

The hand-rolled `arc_path` above is exactly what `arc`/`wedge` below do
internally — reach for them first; `path` stays the escape hatch for shapes
neither one covers.

---

## Radial drawing: `arc`, `wedge`, `ribbon`, `polar`

`arc`, `wedge`, `ribbon`, `polar` and `radial_gradient` share one angle
convention: **degrees, 0° at the top, increasing clockwise** — the same
convention pie/donut/gauge charts already use, so radial compositions read
the same way.

| Method | Effect |
|--------|--------|
| `arc(cx, cy, r, start_deg, end_deg, color="#ffffff", width=1.5, opacity=1, cap="round", layer="fg", name="")` | A stroked circular arc — spokes, progress rings, radial tick marks. |
| `wedge(cx, cy, r_inner, r_outer, start_deg, end_deg, fill="#ffffff", stroke="none", stroke_width=0, opacity=1, layer="fg", group="", name="")` | A filled donut segment; `r_inner=0` collapses it to a pie slice. The building block for radial bar charts — one wedge per bar, `r_outer` mapped to the value. `group` (or a later `link()` call by `name`) makes it join a hover-glow group like `circle`/`rect`/`text` can. |
| `ribbon(cx, cy, r, a_start, a_end, b_start, b_end, fill="#ffffff", opacity=0.7, layer="fg", name="")` | A curved band connecting two arc spans on the same circle through its center — chord-diagram-style links between categories. |
| `polar(cx, cy, r, deg) -> (x, y)` | Pure coordinate math, no drawing — converts a radial position to `(x, y)` so you can place *any* other primitive (`text`, `circle`, `line`, a placed `Chart`) at a computed angle instead of hand-deriving trig every time. |
| `radial_gradient(id, from_color, to_color, cx=0.5, cy=0.5, r=0.5)` | A radial counterpart to `gradient` — reference it the same way, `fill="url(#id)"`, for glows and center-out fades. |

```python
cv = sp.canvas(600, 600, "#0a0a12")
cx, cy = 300, 300
cv.radial_gradient("glow", "#312e81", "#0a0a12", r=0.75)
cv.circle(cx, cy, 260, fill="url(#glow)", name="glow-bg")

values = [8, 15, 6, 22, 11, 18, 4, 13]
n = len(values)
for i, v in enumerate(values):
    a0 = i * 360 / n + 2
    a1 = (i + 1) * 360 / n - 2
    cv.wedge(cx, cy, 60, 60 + v * 7, a0, a1,
             fill=f"hsl({i * 360 // n}, 70%, 60%)", name=f"bar-{i}")
    lx, ly = cv.polar(cx, cy, 60 + v * 7 + 16, (a0 + a1) / 2)
    cv.text(str(v), lx, ly, size=11, color="#f8fafc", anchor="middle", name=f"lbl-{i}")

cv.ribbon(cx, cy, 58, 10, 30, 190, 210, fill="#a78bfa", opacity=0.35, name="link-a")
chart = cv.build()
```

This is the same shape as the radial pieces at
[visualcinnamon.com](https://www.visualcinnamon.com) — a center, a set of
wedges swept around it, labels placed with `polar`, and ribbons crossing
between spans. Nothing here is a dedicated "radial bar chart" type; it's the
five primitives above composed by hand, the same way the rest of `Canvas`
works — including mixing in a placed `Chart` at a `polar`-computed position
if the story calls for it.

---

## Radial gallery: spiral, sunburst, network

Three more shapes built from the same handful of primitives — no new API
below this line, just `polar`, `curve`, `wedge` and `connector` combined
differently each time.

**A spiral** — each point's radius grows with its index instead of staying
fixed, the technique behind timeline-as-spiral pieces like *Searching for
Birds*:

```python
cx, cy = 300, 300
n = 60
cv = sp.canvas(600, 600, "#0a0a12")
pts = []
for i in range(n):
    deg = i * 12
    r = 20 + i * 4.2
    pts.append(list(cv.polar(cx, cy, r, deg)))
cv.curve(pts, color="#a78bfa", width=2, tension=0.8, name="spiral")
for i in range(0, n, 4):
    x, y = pts[i]
    cv.circle(x, y, 3 + (i / n) * 5, fill="#22d3ee", name=f"pt-{i}")
chart = cv.build()
```

**A sunburst** — two rings of `wedge`, the outer ring's spans computed from
the inner ring's proportions instead of an even split, giving a hierarchical
part-of-a-part-of-a-whole read:

```python
cx, cy = 300, 300
cv = sp.canvas(600, 600, "#0a0a12")
groups = [("Frontend", 40), ("Backend", 35), ("Data", 25)]
subgroups = {
    "Frontend": [("React", 20), ("CSS", 12), ("A11y", 8)],
    "Backend": [("API", 18), ("Auth", 10), ("Jobs", 7)],
    "Data": [("ETL", 14), ("ML", 11)],
}

total = sum(v for _, v in groups)
cursor = 0.0
for name, v in groups:
    span = v / total * 360
    cv.wedge(cx, cy, 60, 130, cursor, cursor + span - 2, fill="#6366f1", name=f"inner-{name}")
    lx, ly = cv.polar(cx, cy, 95, cursor + span / 2)
    cv.text(name, lx, ly, size=11, color="#fff", anchor="middle", name=f"inner-lbl-{name}")

    sub_total = sum(sv for _, sv in subgroups[name])
    sub_cursor = cursor
    for sname, sv in subgroups[name]:
        sub_span = sv / sub_total * span
        cv.wedge(cx, cy, 135, 200, sub_cursor, sub_cursor + sub_span - 1,
                 fill="#22d3ee", opacity=0.85, name=f"outer-{sname}")
        lx, ly = cv.polar(cx, cy, 168, sub_cursor + sub_span / 2)
        cv.text(sname, lx, ly, size=9, color="#0a0a12", anchor="middle", name=f"outer-lbl-{sname}")
        sub_cursor += sub_span
    cursor += span
chart = cv.build()
```

**A radial network** — nodes placed on a circle with `polar`, random pairs
joined with `connector`'s `bend` for a soft curve instead of a straight
chord; the same "relationships between things" story `ribbon` tells, drawn
node-and-edge instead of band-and-arc:

```python
import random

cx, cy = 300, 300
n = 14
cv = sp.canvas(600, 600, "#0a0a12")
cv.radial_gradient("net-glow", "#1e1b4b", "#0a0a12", r=0.85)
cv.circle(cx, cy, 280, fill="url(#net-glow)", name="bg")

nodes = [cv.polar(cx, cy, 220, i * 360 / n) for i in range(n)]

edges = set()
while len(edges) < 22:
    a, b = random.sample(range(n), 2)
    edges.add((min(a, b), max(a, b)))

for a, b in edges:
    ax, ay = nodes[a]
    bx, by = nodes[b]
    cv.connector(ax, ay, bx, by, color="#4c1d95", width=1, opacity=0.5, bend=0.15, name=f"edge-{a}-{b}")

for i, (x, y) in enumerate(nodes):
    cv.circle(x, y, 8, fill="#a78bfa", stroke="#0a0a12", stroke_width=2, name=f"node-{i}")
chart = cv.build()
```

---

## Real-world composition: a RéciTAC-style network

A radial network diagram organizing a research program's disciplines,
universities, actions and impact outcomes around a dense central network of
"stories" and "people". None of it needs a dedicated "network chart" type:
`wedge`+`polar` build the outer capsule-segment rings — angular width
proportional to story count per discipline, so the ring itself carries data
instead of just decorating — `polygon`+`rect` build the hexagon impact
clusters and their pill-shaped outcome labels, `connector` draws every curved
edge, and `link()` ties related elements — a hexagon cluster and every story
it touches, a university and its disciplines, an action's color segments —
into shared hover-glow, dim-the-rest groups (hovering any grouped element now
fades everything outside its group to near-transparent, a `Canvas`-level
generalization of `Chart.hover_family()`'s dim-the-rest effect, which only
natively works on hierarchical/flow charts like icicle and sankey). The story
nodes themselves are five real, `place()`d `sp.bubble()` charts — one per
discipline, arranged in a rosette so each contributes its own color-clustered
mass to a shared dense core instead of reading as one undifferentiated blob —
rather than hand-drawn circles. Splitting the hairball this way also makes it
addressable by `link()`: hovering a discipline's ring segment now dims every
*other* discipline's cluster, isolating just its own. Each sub-chart's actual
rendered positions are read back out of its own SVG (by `data-idx`, since
`bubble()` reorders its DOM by category) so every connector line lands
exactly on a real dot, with a fully transparent same-position `circle()` as
the `link()` target `bubble()`'s own iframe can't expose directly, and
`.no_hover()` disables each chart's own baked-in double-click zoom, which
would otherwise shift a dot's position inside its iframe independently of the
canvas-level connector lines drawn to its build-time coordinates. Three more
real charts get `place()`d straight into the
canvas's empty corners and lower edge rather than boxed on top of it: a
`sp.bubble()` "story constellation" stripped of axes/background and
circle-clipped so it reads as one more circular motif, a `sp.histogram()`
"people per story" tilted into its own corner, and a `sp.barh()` "stories per
discipline" sitting low and centered — all three summarizing the same dataset
the diagram encodes, from different angles. Composition isn't an alternative
to SeraPlot's chart functions, it's a way to combine them in one scene.

The full, runnable version (with the synthetic dataset, the university/action
hover groups, and both embedded charts) lives at
[`notebook/canva/recitac_remake.ipynb`](https://github.com/feur25/Sera/blob/main/notebook/canva/recitac_remake.ipynb).
The trimmed sketch below shows the core technique — outer ring, one hexagon
cluster, a handful of hairball nodes:

```python
import random
import seraplot as sp

def tangent_rot(angle):
    r = (angle - 90) % 360
    return r - 180 if 90 < r < 270 else r

W = H = 1700
CX = CY = W / 2
DISCIPLINES = [("Social Science", "#f59e0b"), ("Health", "#16a34a"), ("Engineering", "#38bdf8")]

cv = sp.Canvas(W, H, "#ffffff")
cv.radial_gradient("glow", "#fef9f0", "#ffffff", cx=0.5, cy=0.58, r=0.6)
cv.circle(CX, CY, 640, fill="url(#glow)", layer="bg")

R = 700
span = 120 / len(DISCIPLINES)
for i, (name, color) in enumerate(DISCIPLINES):
    a0, a1 = 200 + i * span, 200 + (i + 1) * span - 3
    cv.wedge(CX, CY, R - 9, R + 9, a0, a1, fill=color, name=f"disc-{i}")
    lx, ly = cv.polar(CX, CY, R - 26, (a0 + a1) / 2)
    cv.text(name, lx, ly, size=11, anchor="middle", rotation=tangent_rot((a0 + a1) / 2))

def hexagon(cx, cy, r):
    return [list(cv.polar(cx, cy, r, k * 60)) for k in range(6)]

hx, hy = CX - 190, CY - 240
cv.polygon(hexagon(hx, hy, 125), fill="#ecfdf5", stroke="#22c55e", stroke_width=2.5, name="hex-TRUST")
cv.rect(hx - 60, hy - 20, 108, 52, fill="#22c55e", rx=14, name="outcome-TRUST-0")
cv.text("Trust in\nreciprocity", hx - 6, hy + 6, size=10.5, color="#fff", anchor="middle")

stories = []
for i in range(12):
    ang, rad = random.uniform(0, 360), random.uniform(60, 300)
    x, y = cv.polar(CX, CY, rad, ang)
    color = random.choice(DISCIPLINES)[1]
    cv.circle(x, y, 10, fill=color, stroke="#fff", stroke_width=1.5, name=f"story-{i}")
    cv.connector(x, y, hx, hy, color=color, width=0.8, opacity=0.15, bend=0.3)
    stories.append(f"story-{i}")

cv.link("impact-TRUST", ["hex-TRUST", "outcome-TRUST-0"] + stories)

bubbles = sp.bubble(
    categories=[random.choice(DISCIPLINES)[0] for _ in range(20)],
    x_values=[random.uniform(0, 100) for _ in range(20)],
    y_values=[random.uniform(0, 10) for _ in range(20)],
    sizes=[random.uniform(10, 40) for _ in range(20)],
    palette=[int(c.lstrip("#"), 16) for _, c in DISCIPLINES],
    width=700, height=700,
).no_axes().no_background().gridlines(False)
cv.place(bubbles, 20, 100, 280, 280, clip="circle", name="panel-bubbles")

chart = cv.build().zoom()
```

<div class="sp-preview-frame"><iframe src="../previews/canvas-recitac.html?v=6c71919e" style="width:100%;height:640px;border:none;border-radius:8px;display:block;background:#ffffff" loading="lazy"></iframe></div>

The dataset is synthetic — the point is the *composition pattern*, not a
literal port of Nadieh's real research-program data (which comes from a
private Google Sheet). Everything scales with the data: add a discipline and
every ring/legend/embedded chart picks it up automatically; add a story and a
new node, a new set of connectors, and a new hover-group member appear on the
next `build()`.

**`.zoom()`** on the built `Chart` (called above, `cv.build().zoom()`) turns
on mouse-wheel/pinch zoom and drag-to-pan for the whole composition — useful
once a canvas is dense enough that hovering individual hexagons or story
nodes benefits from zooming in first.

**A tip for very large canvases**: `chart` on its own (or `chart.show()`)
sizes its inline `<iframe>` via CSS `aspect-ratio`, which some notebook
frontends resolve unreliably for big square canvases like this one (1700×1700),
silently cropping the output instead of shrinking it to fit. `chart.save(path)`
plus `IPython.display.IFrame(src=path, width=..., height=...)` sidesteps that
by reserving an explicit pixel size upfront — the canvas's own internal
viewport-fit script then scales the full composition down to whatever that
turns out to be, so you always see it in full.

---

## A constellation of real charts: satellites on dashed leaders

A different composition shape than RéciTAC's rings-and-hairball: one central
`sp.scatter()` plotting every model's mean quality against its mean
emissions, and one small satellite `sp.scatter()` per model — its own raw
event cloud, `no_axes()`/`no_title()`/`hide_grid()`/`no_legend()`/
`no_background()`'d down to just the dots — arranged in a ring around it.
Each satellite connects back to its model's exact point on the central plot
with a dashed `line()`.

The satellite-to-point link uses `hover_group` directly on `line()`/`circle()`/
`place()` instead of a separate `link()` call: pass the same `hover_group="sat-3"`
string to every element that should glow together, and they're linked from the
moment they're created — no `name=` + follow-up `cv.link(...)` pass needed.
Both mechanisms end up in the same place; `hover_group` is the one-line version
when you know the group at creation time.

```python
import math
import random
import seraplot as sp

random.seed(3)
MODELS = [("Titan-7B", "#6366f1"), ("Mixtral-Sparse", "#7c3aed"),
          ("Codex-T5", "#0891b2"), ("VisionSpeak-VL", "#0ea5e9"),
          ("DiffuGen-2", "#d97706"), ("BertCore", "#059669")]

means = [(random.uniform(55, 95), random.uniform(3.8, 5.2)) for _ in MODELS]
central = sp.scatter(
    "Quality vs log(emissions)",
    x=[m[0] for m in means], y=[m[1] for m in means],
    labels=[name for name, _ in MODELS], groups=[name for name, _ in MODELS],
    x_label="Quality score", y_label="log10 gCO2e", width=520, height=400,
)

CW = CH = 900
CX = CY = 450
R = 340
cv = sp.Canvas(CW, CH, "#ffffff")
cv.place(central, CX - 260, CY - 200, 520, 400)

for i, (name, color) in enumerate(MODELS):
    ang = math.radians(-90 + i * 360 / len(MODELS))
    mx, my = CX + R * math.cos(ang), CY + R * math.sin(ang)
    px = CX - 260 + 60 + (means[i][0] - 55) / 40 * 400
    py = CY - 200 + 30 + (1 - (means[i][1] - 3.8) / 1.4) * 330
    hg = f"sat-{i}"

    cv.line(px, py, mx, my, color=color, width=1.2, dash="2 5", hover_group=hg)

    mini = sp.scatter(
        "", x=[random.gauss(0, 1) for _ in range(30)],
        y=[random.gauss(0, 1) for _ in range(30)],
        labels=[name] * 30, groups=[name] * 30, palette=[int(color[1:], 16)],
        width=150, height=110,
    ).no_axes().no_title().hide_grid().no_legend().no_background().no_hover()
    cv.place(mini, mx - 75, my - 55, 150, 110, group=hg)
    cv.text(name, mx, my - 62, size=9, color=color, anchor="middle", weight="bold")

chart = cv.build()
```

<div class="sp-preview-frame"><iframe src="../previews/canvas-constellation.html?v=5bd7e57b" style="width:100%;height:640px;border:none;border-radius:8px;display:block;background:#ffffff" loading="lazy"></iframe></div>

The dataset for the live preview above is synthetic too — recreated from the
same `ai_story` data-story project's `constellation.py`, whose original CSVs
no longer exist. 14 architectures, not 6, and each satellite's event cloud is
real per-model data (up to 60 events) rather than a Gaussian stand-in — the
technique is identical, just with more rows and a second sidecar dataset
joined in by `model_id`.

---

## Composing real charts: a mission-control dashboard

`place()` embeds a full `Chart` — not just a primitive — inside a canvas,
which means canvas composition isn't limited to hand-drawn shapes: real
`sp.line()`, `sp.bar()`, `sp.gauge()`, `sp.area()`, `sp.donut()`,
`sp.barh()` panels can sit framed, connected, and annotated by the exact
same primitives used everywhere else on this page. The part that actually
sells "dashboard" over "charts in boxes" is the same trick as the RéciTAC
network above: a **shared center every core panel connects to**. One
glowing hub, four color-matched spokes (`connector` + a `circle` anchor at
each end), each spoke tinted to match the panel it comes from via the
chart-level chainable methods (`palette()`, `gridlines()`,
`width()`/`height()`, `title_color()` — see
[Chart Methods](../getting-started/chart-methods.md)) applied before
`place()`. Beyond the four hub panels, a KPI ribbon with its own inline
`sp.line()` sparklines sits above the grid, two more real charts flank it
on the right on a subtler `hover_group`-linked connection, and a full-width
throughput panel closes the composition at the bottom:

```python
import random
import seraplot as sp

random.seed(3)

W, H = 1950, 1380
cv = sp.Canvas(W, H)

cv.radial_gradient("dashBg", "#1a2140", "#04050a", cx=0.5, cy=0.42, r=1.0)
cv.rect(0, 0, W, H, fill="url(#dashBg)", layer="bg")
cv.radial_gradient("hubGlow", "#22d3ee", "#04050a", cx=0.5, cy=0.5, r=0.5)

cv.text("Mission Control", 48, 56, size=30, color="#f8fafc", weight="800")
cv.text("Every panel wired into one live hub — sp.Canvas place() + connectors + real SeraPlot charts",
         48, 84, size=13, color="#64748b")

cv.text("updated 2s ago", W - 60, 50, size=11, color="#475569", anchor="end")
cv.circle(W - 260, 47, 5, fill="#22c55e", name="live-dot")
cv.circle(W - 260, 47, 5, fill="none", stroke="#22c55e", stroke_width=1.5, opacity=0.6, name="live-pulse")
cv.text("LIVE", W - 246, 52, size=12, color="#22c55e", weight="700", letter_spacing=1.5)

KPI = [
    ("ACTIVE USERS", "12,940", "+6.1%", "#6366f1", [820, 860, 901, 934, 990, 1120, 1180, 1290, 1330, 1320]),
    ("MRR", "$184.2k", "+3.4%", "#22d3ee", [140, 148, 152, 149, 158, 165, 170, 176, 180, 184]),
    ("UPTIME", "99.982%", "+0.02%", "#f59e0b", [99.9, 99.91, 99.95, 99.93, 99.96, 99.97, 99.98, 99.97, 99.98, 99.982]),
    ("OPEN INCIDENTS", "5", "-2 today", "#f472b6", [9, 8, 8, 7, 6, 7, 6, 6, 5, 5]),
]
KPI_Y = 118
KPI_W, KPI_H = 340, 92
for i, (label, value, delta, color, series) in enumerate(KPI):
    kx = 48 + i * (KPI_W + 20)
    cv.rect(kx, KPI_Y, KPI_W, KPI_H, fill="#0b1022", stroke="rgba(255,255,255,.07)",
            stroke_width=1, rx=14, layer="bg", name=f"kpi-{i}")
    cv.rect(kx, KPI_Y, 4, KPI_H, fill=color, rx=2, layer="bg")
    cv.text(label, kx + 20, KPI_Y + 26, size=10.5, color="#64748b", weight="700", letter_spacing=1.2)
    cv.text(value, kx + 20, KPI_Y + 58, size=24, color="#f8fafc", weight="800")
    cv.text(delta, kx + 20, KPI_Y + 78, size=11.5, color=color, weight="600")

    spark = sp.line(labels=[str(j) for j in range(len(series))], values=series,
                     color_hex=int(color.lstrip("#"), 16), width=150, height=64) \
        .no_axes().no_title().hide_grid().no_legend().no_background().no_hover()
    cv.place(spark, kx + KPI_W - 168, KPI_Y + 16, 150, 64, name=f"kpi-spark-{i}")

GRID_Y0 = 250
PW, PH = 660, 320
GAP_X, GAP_Y = 100, 80
COL0, COL1 = 60, 60 + PW + GAP_X
ROW0, ROW1 = GRID_Y0, GRID_Y0 + PH + GAP_Y
HX, HY, HR = (COL0 + PW + GAP_X / 2), (ROW0 + PH + GAP_Y / 2), 92

PALETTE = [0x6366f1, 0x22d3ee, 0xf59e0b, 0xf472b6]
HEX = [f"#{c:06x}" for c in PALETTE]
PANELS = [("trend", COL0, ROW0, HEX[0]), ("revenue", COL1, ROW0, HEX[1]),
          ("health", COL0, ROW1, HEX[2]), ("incidents", COL1, ROW1, HEX[3])]

def panel_frame(x, y, w, h, color, name):
    cv.rect(x - 16, y - 16, w + 32, h + 32, fill="#0b1022", stroke="rgba(255,255,255,.06)",
            stroke_width=1, rx=18, layer="bg", name=name)
    cv.rect(x - 16, y - 16, w + 32, 4, fill=color, rx=2, layer="bg")

for name, x, y, color in PANELS:
    panel_frame(x, y, PW, PH, color, f"panel-{name}")

trend = sp.line(labels=["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"],
                 values=[820, 932, 901, 934, 1290, 1330, 1320],
                 ).width(PW).height(PH).palette(PALETTE).gridlines(False).background("#0b1022")

revenue = sp.bar(labels=["Core", "Cloud", "API", "Mobile", "Support"],
                  values=[420, 680, 310, 240, 150], title="Revenue by Segment",
                  ).width(PW).height(PH).palette(PALETTE).gridlines(False).background("#0b1022").title_color("#e2e8f0")

health = sp.gauge(value=87).width(PW).height(PH).background("#0b1022")

incidents = sp.area(labels=["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"], values=[5, 3, 6, 2, 4, 1, 2],
                     title="Open Incidents",
                     ).width(PW).height(PH).palette([PALETTE[3]]).gridlines(False).background("#0b1022").title_color("#e2e8f0")

cv.place(trend, COL0, ROW0, PW, PH, name="chart-trend")
cv.place(revenue, COL1, ROW0, PW, PH, name="chart-revenue")
cv.place(health, COL0, ROW1, PW, PH, name="chart-health")
cv.place(incidents, COL1, ROW1, PW, PH, name="chart-incidents")
cv.text("Weekly Active Users", COL0 + 20, ROW0 + 26, size=13, color="#e2e8f0", weight="700")
cv.text("System Health", COL0 + 20, ROW1 + 26, size=13, color="#e2e8f0", weight="700")

ANCHORS = {
    "trend": (COL0 + PW, ROW0 + PH / 2),
    "revenue": (COL1, ROW0 + PH / 2),
    "health": (COL0 + PW, ROW1 + PH / 2),
    "incidents": (COL1, ROW1 + PH / 2),
}
for name, x, y, color in PANELS:
    ax, ay = ANCHORS[name]
    cv.connector(ax, ay, HX, HY, color=color, width=2, opacity=0.55, bend=0.28, name=f"spoke-{name}")
    cv.circle(ax, ay, 6, fill=color, stroke="#04050a", stroke_width=2, name=f"anchor-{name}")
    cv.circle(ax, ay, 11, fill="none", stroke=color, stroke_width=1, opacity=0.4)

cv.circle(HX, HY, HR + 34, fill="url(#hubGlow)", opacity=0.35)
cv.ring(HX, HY, HR + 18, HR + 22, fill="#22d3ee", opacity=0.25)
cv.ring(HX, HY, HR + 6, HR + 9, fill="#22d3ee", opacity=0.45)

start = 0.0
for name, _, _, color in PANELS:
    end = start + 90
    cv.wedge(HX, HY, HR - 10, HR - 4, start, end, fill=color, opacity=0.85)
    start = end

cv.circle(HX, HY, HR - 14, fill="#0b1022", stroke="#f8fafc", stroke_width=2)
cv.text("87", HX, HY - 4, size=40, color="#22d3ee", weight="800", anchor="middle")
cv.text("SYSTEM SCORE", HX, HY + 22, size=10, color="#64748b", anchor="middle", letter_spacing=1.5)

cv.link("hub-cluster", ["anchor-trend", "anchor-revenue", "anchor-health", "anchor-incidents"])

cv.annotate("Trending up 61% since Monday", COL0 + 190, ROW0 + 170, COL0 + 40, ROW0 + PH + 55,
            color="#94a3b8", size=12, line_dash="4,3", bg="#0b1022")
cv.annotate("5 open, 2 critical", COL1 + 460, ROW1 + 90, COL1 + 40, ROW1 + PH + 55,
            color="#94a3b8", size=12, line_dash="4,3", bg="#0b1022")

SIDE_X = COL1 + PW + GAP_X
SIDE_W = W - SIDE_X - 60
panel_frame(SIDE_X, ROW0, SIDE_W, PH, "#a78bfa", "panel-region")
panel_frame(SIDE_X, ROW1, SIDE_W, PH, "#34d399", "panel-latency")

region = sp.donut(labels=["NA", "EU", "APAC", "LATAM"], values=[40, 28, 20, 12],
                   title="Regional Split", width=SIDE_W, height=PH,
                   palette=[0x6366f1, 0x22d3ee, 0xf59e0b, 0xf472b6]) \
    .background("#0b1022").title_color("#e2e8f0")

latency = sp.barh(labels=["p50", "p90", "p95", "p99"], values=[42, 118, 210, 480],
                   title="Latency (ms)", width=SIDE_W, height=PH,
                   palette=[0x34d399]) \
    .gridlines(False).background("#0b1022").title_color("#e2e8f0")

cv.place(region, SIDE_X, ROW0, SIDE_W, PH, group="side-link", name="panel-region-chart")
cv.place(latency, SIDE_X, ROW1, SIDE_W, PH, group="side-link", name="panel-latency-chart")

SIDE_LINK = [(ROW0, "#a78bfa"), (ROW1, "#34d399")]
for row_y, color in SIDE_LINK:
    ax, ay = COL1 + PW, row_y + PH / 2
    bx, by = SIDE_X, row_y + PH / 2
    cv.line(ax, ay, bx, by, color="#94a3b8", width=1, dash="2 5", hover_group="side-link")
    cv.circle(ax, ay, 5, fill=color, stroke="#04050a", stroke_width=2)
    cv.circle(bx, by, 5, fill=color, stroke="#04050a", stroke_width=2)

STRIP_Y = ROW1 + PH + GAP_Y
STRIP_H = H - STRIP_Y - 60
panel_frame(60, STRIP_Y, W - 120, STRIP_H, "#22d3ee", "panel-throughput")

hours = [f"{h:02d}:00" for h in range(0, 24, 2)]
throughput = [random.randint(800, 2600) for _ in hours]
throughput_chart = sp.bar(labels=hours, values=throughput, title="Requests/sec — last 24h",
                           width=W - 120, height=STRIP_H,
                           color_hex=0x22d3ee) \
    .gridlines(False).background("#0b1022").title_color("#e2e8f0")
cv.place(throughput_chart, 60, STRIP_Y, W - 120, STRIP_H, name="panel-throughput-chart")

chart = cv.build()
```

<div class="sp-preview-frame"><iframe src="../previews/canvas-dashboard.html?v=0e335555" style="width:100%;height:680px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe></div>

The hub itself is a small radial gauge in disguise — four `wedge` slices
(one per panel, in that panel's color) inside two `ring` pulse tracks,
built from exactly the same primitives as the "Composing micro-tools"
dial and the RéciTAC donut rings above. Reusing one visual language across
every worked example on this page is the actual point: primitives don't
know or care whether they're drawing a progress dial, a discipline ring,
or a dashboard hub.

`panel_frame()` draws a rounded card plus a thin color-matched top border
— that accent color is the same one used for that panel's spoke and
`palette()`, so the eye connects "this line is orange" to "this panel is
orange" to "this spoke is orange" without a legend. The one easy-to-miss
detail behind all of it: any decoration meant to sit *behind* a placed
chart (background fill, panel frames) needs `layer="bg"` explicitly —
`rect()`'s default `layer="fg"` renders **on top of** placed charts by
design (so connectors and callouts can cross over them), which will
silently hide a panel's contents if the panel itself is drawn on the
foreground layer.

Two things needed fixing to get here from the original four-panel
version. First, `title_color()`: `sp.bar()`/`sp.area()`/`sp.donut()`
happily took an explicit title color, but `sp.line()`/`sp.gauge()`
rendered their title in a barely visible default shade regardless —
worked around by skipping the chart's own `title=` entirely for those two
and drawing the panel name as a plain `cv.text()` at the same position
instead, which also matches the KPI tiles' hand-drawn labels. Second, a
chart's own y-axis tick generator reserves space proportional to the
plot's own *requested* pixel size, not its `place()`d footprint — the
bottom throughput strip's tick labels overlapped badly until its
`sp.bar()` was requested at the strip's full native size (`width=W-120,
height=STRIP_H`) rather than an undersized box meant to be stretched.

The two right-hand panels (`Regional Split`, `Latency (ms)`) aren't wired
into the hub — a fifth and sixth spoke would have overloaded the one
visual idea the hub is supposed to communicate — but they're not
orphaned either: a single `hover_group="side-link"` on both panels and
their connecting line links them exactly like the RéciTAC satellites, so
hovering either panel dims everything outside that pair via the same
`Canvas`-level dim-the-rest hover behavior described above.

---

## Organic layouts: Voronoi

`voronoi(sites, x, y, w, h, fills=None, stroke=..., stroke_width=..., opacity=...)`
computes a bounded Voronoi diagram — one cell per site, each cell the region
closer to that site than to any other — and adds every cell to the canvas as
a `polygon()` in one call, returning their element indices for later
addressing (hover groups, `derive()`, etc.).

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
`Chart`, `Rect`, `Text`, `Circle`, `Wedge` or `Polygon`) glows/pulses all the
others in the same group. Returns how many of the given names were actually
linkable (`Line`, `RawPath` and other pure decoration types don't currently
support it). `circle(...)` and `polygon(...)` also accept a `hover_group=`
kwarg to join a group right at creation time, without a separate `link()`
call — both paths stamp the same `data-sp-grp`/`data-group` attributes, so
grouped circles/polygons are also picked up by any chart's own chainable
`.group_hover_opacity(dim)` (see [Chart Methods](../getting-started/chart-methods.md)),
letting a single hover dim every non-matching mark on the canvas while the
native glow/pulse handles the matching ones.

```python
cv.link("story", ["revenue_chart", "trend_chart", "kpi_card"])
cv.circle(120, 80, 6, fill="#2dd4bf", hover_group="alice", name="c1")
```

### `frieze` / `timeline` / `chronology` — inter-plot chronological layout

Three names for the same primitive: lay out `labels` in a boustrophedon
(snake) grid — left-to-right, then right-to-left on the next row, and so on —
connected by straight segments within a row and an S-curve at each row wrap,
with one enclosing ring and label per cell. Returns the `(x, y, ring_radius)`
anchor of every cell, so any other chart or shape can be drawn/placed right
on top of it — a natural fit for stringing several single-group
`circle_pack(variant="swarm")` clusters, or any other small chart, along a
real chronology instead of relying on a variant's own built-in layout.

```python
anchors = cv.frieze(week_labels, weights=week_counts, cols=8,
                     cell_w=280, cell_h=280, ring_color="#7dd3fc")
for (x, y, r), commits_in_week in zip(anchors, weeks):
    ...
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

Le `arc_path` codé à la main ci-dessus, c'est exactement ce que font
`arc`/`wedge` en interne — utilisez-les en premier ; `path` reste
l'échappatoire pour les formes qu'aucun des deux ne couvre.

---

## Dessin radial : `arc`, `wedge`, `ribbon`, `polar`

`arc`, `wedge`, `ribbon`, `polar` et `radial_gradient` partagent une
convention d'angle : **degrés, 0° en haut, croissant dans le sens horaire**
— la même convention que les charts pie/donut/gauge, pour que les
compositions radiales se lisent de la même façon.

| Méthode | Effet |
|--------|--------|
| `arc(cx, cy, r, start_deg, end_deg, color="#ffffff", width=1.5, opacity=1, cap="round", layer="fg", name="")` | Un arc de cercle tracé — rayons, anneaux de progression, graduations radiales. |
| `wedge(cx, cy, r_inner, r_outer, start_deg, end_deg, fill="#ffffff", stroke="none", stroke_width=0, opacity=1, layer="fg", group="", name="")` | Un segment d'anneau rempli ; `r_inner=0` le réduit à une part de camembert. La brique de base des barres radiales — une wedge par barre, `r_outer` mappé sur la valeur. `group` (ou un appel `link()` ultérieur par `name`) le fait rejoindre un groupe de survol comme `circle`/`rect`/`text`. |
| `ribbon(cx, cy, r, a_start, a_end, b_start, b_end, fill="#ffffff", opacity=0.7, layer="fg", name="")` | Une bande courbe reliant deux plages d'arc sur le même cercle en passant par son centre — liens façon chord diagram entre catégories. |
| `polar(cx, cy, r, deg) -> (x, y)` | Du calcul de coordonnées pur, sans dessin — convertit une position radiale en `(x, y)` pour placer n'importe quelle autre primitive (`text`, `circle`, `line`, un `Chart` placé) à un angle calculé, sans refaire la trigonométrie à la main. |
| `radial_gradient(id, from_color, to_color, cx=0.5, cy=0.5, r=0.5)` | Le pendant radial de `gradient` — se référence pareil, `fill="url(#id)"`, pour des lueurs et fondus centre-vers-bord. |

```python
cv = sp.canvas(600, 600, "#0a0a12")
cx, cy = 300, 300
cv.radial_gradient("glow", "#312e81", "#0a0a12", r=0.75)
cv.circle(cx, cy, 260, fill="url(#glow)", name="glow-bg")

values = [8, 15, 6, 22, 11, 18, 4, 13]
n = len(values)
for i, v in enumerate(values):
    a0 = i * 360 / n + 2
    a1 = (i + 1) * 360 / n - 2
    cv.wedge(cx, cy, 60, 60 + v * 7, a0, a1,
             fill=f"hsl({i * 360 // n}, 70%, 60%)", name=f"bar-{i}")
    lx, ly = cv.polar(cx, cy, 60 + v * 7 + 16, (a0 + a1) / 2)
    cv.text(str(v), lx, ly, size=11, color="#f8fafc", anchor="middle", name=f"lbl-{i}")

cv.ribbon(cx, cy, 58, 10, 30, 190, 210, fill="#a78bfa", opacity=0.35, name="link-a")
chart = cv.build()
```

C'est la même construction que les pièces radiales de
[visualcinnamon.com](https://www.visualcinnamon.com) — un centre, des
wedges disposées tout autour, des labels placés avec `polar`, et des
ribbons qui traversent entre les plages. Rien ici n'est un type
« radial bar chart » dédié ; ce sont les cinq primitives ci-dessus
composées à la main, de la même façon que le reste de `Canvas` —
y compris en mélangeant un `Chart` placé à une position calculée par
`polar` si l'histoire le demande.

---

## Galerie radiale : spirale, sunburst, réseau

Trois formes de plus construites à partir des mêmes primitives — aucune
nouvelle API en dessous de cette ligne, juste `polar`, `curve`, `wedge` et
`connector` combinés différemment à chaque fois.

**Une spirale** — le rayon de chaque point grandit avec son index au lieu
de rester fixe, la technique derrière les pièces façon timeline-en-spirale
comme *Searching for Birds* :

```python
cx, cy = 300, 300
n = 60
cv = sp.canvas(600, 600, "#0a0a12")
pts = []
for i in range(n):
    deg = i * 12
    r = 20 + i * 4.2
    pts.append(list(cv.polar(cx, cy, r, deg)))
cv.curve(pts, color="#a78bfa", width=2, tension=0.8, name="spiral")
for i in range(0, n, 4):
    x, y = pts[i]
    cv.circle(x, y, 3 + (i / n) * 5, fill="#22d3ee", name=f"pt-{i}")
chart = cv.build()
```

**Un sunburst** — deux anneaux de `wedge`, les plages de l'anneau extérieur
calculées à partir des proportions de l'anneau intérieur plutôt qu'un
partage égal, pour une lecture hiérarchique « partie d'une partie d'un
tout » :

```python
cx, cy = 300, 300
cv = sp.canvas(600, 600, "#0a0a12")
groups = [("Frontend", 40), ("Backend", 35), ("Data", 25)]
subgroups = {
    "Frontend": [("React", 20), ("CSS", 12), ("A11y", 8)],
    "Backend": [("API", 18), ("Auth", 10), ("Jobs", 7)],
    "Data": [("ETL", 14), ("ML", 11)],
}

total = sum(v for _, v in groups)
cursor = 0.0
for name, v in groups:
    span = v / total * 360
    cv.wedge(cx, cy, 60, 130, cursor, cursor + span - 2, fill="#6366f1", name=f"inner-{name}")
    lx, ly = cv.polar(cx, cy, 95, cursor + span / 2)
    cv.text(name, lx, ly, size=11, color="#fff", anchor="middle", name=f"inner-lbl-{name}")

    sub_total = sum(sv for _, sv in subgroups[name])
    sub_cursor = cursor
    for sname, sv in subgroups[name]:
        sub_span = sv / sub_total * span
        cv.wedge(cx, cy, 135, 200, sub_cursor, sub_cursor + sub_span - 1,
                 fill="#22d3ee", opacity=0.85, name=f"outer-{sname}")
        lx, ly = cv.polar(cx, cy, 168, sub_cursor + sub_span / 2)
        cv.text(sname, lx, ly, size=9, color="#0a0a12", anchor="middle", name=f"outer-lbl-{sname}")
        sub_cursor += sub_span
    cursor += span
chart = cv.build()
```

**Un réseau radial** — des nœuds placés sur un cercle avec `polar`, des
paires aléatoires reliées via le `bend` de `connector` pour une courbe
douce plutôt qu'une corde droite ; la même histoire de « relations entre
les choses » que raconte `ribbon`, dessinée nœuds-et-arêtes plutôt que
bandes-et-arcs :

```python
import random

cx, cy = 300, 300
n = 14
cv = sp.canvas(600, 600, "#0a0a12")
cv.radial_gradient("net-glow", "#1e1b4b", "#0a0a12", r=0.85)
cv.circle(cx, cy, 280, fill="url(#net-glow)", name="bg")

nodes = [cv.polar(cx, cy, 220, i * 360 / n) for i in range(n)]

edges = set()
while len(edges) < 22:
    a, b = random.sample(range(n), 2)
    edges.add((min(a, b), max(a, b)))

for a, b in edges:
    ax, ay = nodes[a]
    bx, by = nodes[b]
    cv.connector(ax, ay, bx, by, color="#4c1d95", width=1, opacity=0.5, bend=0.15, name=f"edge-{a}-{b}")

for i, (x, y) in enumerate(nodes):
    cv.circle(x, y, 8, fill="#a78bfa", stroke="#0a0a12", stroke_width=2, name=f"node-{i}")
chart = cv.build()
```

---

## Composition réelle : un réseau façon RéciTAC

Un diagramme réseau radial qui organise les disciplines, les
universités, les actions et les résultats d'impact d'un programme de
recherche autour d'un réseau central dense de « stories » et de
« personnes ». Rien de tout cela ne nécessite un type « graphique réseau »
dédié : `wedge`+`polar` construisent les anneaux extérieurs en capsules —
largeur angulaire proportionnelle au nombre de stories par discipline,
donc l'anneau lui-même porte de la donnée au lieu de seulement décorer —
`polygon`+`rect` construisent les clusters hexagonaux d'impact et leurs
étiquettes en pilule, `connector` trace chaque arête courbe, et `link()`
relie les éléments connexes — un cluster hexagonal et chaque story qui le
touche, une université et ses disciplines, les segments de couleur d'une
action — en groupes de survol partagés qui estompent aussi tout le reste
(survoler un élément d'un groupe fait maintenant tomber l'opacité de tout
ce qui n'en fait pas partie — une généralisation au niveau `Canvas` de
l'effet d'atténuation de `Chart.hover_family()`, qui ne fonctionne
nativement que sur les charts hiérarchiques/flux comme icicle et sankey).
Les nœuds story sont eux-mêmes cinq vrais charts `sp.bubble()` `place()`és —
un par discipline, disposés en rosace pour que chacun apporte sa propre
masse colorée à un noyau dense commun au lieu de former un seul bloc
indifférencié — plutôt que des cercles dessinés à la main. Découper le
hairball ainsi le rend aussi adressable par `link()` : survoler le segment
d'anneau d'une discipline estompe maintenant le cluster de chaque *autre*
discipline, isolant le sien. Les positions réellement rendues de chaque
sous-chart sont relues depuis son propre SVG (via `data-idx`, car
`bubble()` réordonne son DOM par catégorie) pour que chaque ligne de
connexion tombe exactement sur un vrai point, avec un `circle()`
transparent à la même position comme cible de `link()`, ce que l'iframe
propre de `bubble()` ne peut pas exposer directement, et `.no_hover()`
désactive le zoom par double-clic intégré à chaque chart, qui sinon
déplacerait un point à l'intérieur de son iframe indépendamment des lignes
de connexion tracées au niveau du canvas vers ses coordonnées de
construction. Trois autres vrais charts sont `place()`és directement dans
les coins vides et le bas du canvas plutôt qu'encadrés par-dessus : un
`sp.bubble()` « constellation de stories » débarrassé de ses axes/fond et
rogné en cercle, un `sp.histogram()` « personnes par story » incliné dans
son coin, et un `sp.barh()` « stories par discipline » posé bas et centré —
les trois résumant le même jeu de données que le diagramme encode, sous des
angles différents. La composition n'est pas une alternative aux fonctions
de chart de SeraPlot, c'est un moyen de les combiner dans une seule scène.

La version complète et exécutable (avec le jeu de données synthétique, les
groupes de survol université/action, et les deux charts intégrés) se
trouve dans
[`notebook/canva/recitac_remake.ipynb`](https://github.com/feur25/Sera/blob/main/notebook/canva/recitac_remake.ipynb).
L'esquisse simplifiée ci-dessous montre la technique de base — anneau
extérieur, un cluster hexagonal, une poignée de nœuds du réseau central :

```python
import random
import seraplot as sp

def tangent_rot(angle):
    r = (angle - 90) % 360
    return r - 180 if 90 < r < 270 else r

W = H = 1700
CX = CY = W / 2
DISCIPLINES = [("Social Science", "#f59e0b"), ("Health", "#16a34a"), ("Engineering", "#38bdf8")]

cv = sp.Canvas(W, H, "#ffffff")
cv.radial_gradient("glow", "#fef9f0", "#ffffff", cx=0.5, cy=0.58, r=0.6)
cv.circle(CX, CY, 640, fill="url(#glow)", layer="bg")

R = 700
span = 120 / len(DISCIPLINES)
for i, (name, color) in enumerate(DISCIPLINES):
    a0, a1 = 200 + i * span, 200 + (i + 1) * span - 3
    cv.wedge(CX, CY, R - 9, R + 9, a0, a1, fill=color, name=f"disc-{i}")
    lx, ly = cv.polar(CX, CY, R - 26, (a0 + a1) / 2)
    cv.text(name, lx, ly, size=11, anchor="middle", rotation=tangent_rot((a0 + a1) / 2))

def hexagon(cx, cy, r):
    return [list(cv.polar(cx, cy, r, k * 60)) for k in range(6)]

hx, hy = CX - 190, CY - 240
cv.polygon(hexagon(hx, hy, 125), fill="#ecfdf5", stroke="#22c55e", stroke_width=2.5, name="hex-TRUST")
cv.rect(hx - 60, hy - 20, 108, 52, fill="#22c55e", rx=14, name="outcome-TRUST-0")
cv.text("Trust in\nreciprocity", hx - 6, hy + 6, size=10.5, color="#fff", anchor="middle")

stories = []
for i in range(12):
    ang, rad = random.uniform(0, 360), random.uniform(60, 300)
    x, y = cv.polar(CX, CY, rad, ang)
    color = random.choice(DISCIPLINES)[1]
    cv.circle(x, y, 10, fill=color, stroke="#fff", stroke_width=1.5, name=f"story-{i}")
    cv.connector(x, y, hx, hy, color=color, width=0.8, opacity=0.15, bend=0.3)
    stories.append(f"story-{i}")

cv.link("impact-TRUST", ["hex-TRUST", "outcome-TRUST-0"] + stories)

bubbles = sp.bubble(
    categories=[random.choice(DISCIPLINES)[0] for _ in range(20)],
    x_values=[random.uniform(0, 100) for _ in range(20)],
    y_values=[random.uniform(0, 10) for _ in range(20)],
    sizes=[random.uniform(10, 40) for _ in range(20)],
    palette=[int(c.lstrip("#"), 16) for _, c in DISCIPLINES],
    width=700, height=700,
).no_axes().no_background().gridlines(False)
cv.place(bubbles, 20, 100, 280, 280, clip="circle", name="panel-bubbles")

chart = cv.build().zoom()
```

<div class="sp-preview-frame"><iframe src="../previews/canvas-recitac.html?v=6c71919e" style="width:100%;height:640px;border:none;border-radius:8px;display:block;background:#ffffff" loading="lazy"></iframe></div>

Le jeu de données est synthétique — l'intérêt est le *motif de
composition*, pas un portage littéral des vraies données du programme de
recherche de Nadieh (issues d'une feuille Google Sheets privée). Tout
s'adapte aux données : ajouter une discipline et chaque anneau/légende/
chart intégré la prend en compte automatiquement ; ajouter une story fait
apparaître un nouveau nœud, un nouveau jeu de connecteurs, et un nouveau
membre de groupe de survol au prochain `build()`.

**`.zoom()`** sur le `Chart` construit (appelé ci-dessus,
`cv.build().zoom()`) active le zoom molette/pincement et le glisser-déposer
pour toute la composition — utile dès qu'un canvas est assez dense pour que
survoler un hexagone ou un nœud individuel bénéficie d'un zoom préalable.

**Une astuce pour les très grands canvas** : `chart` seul (ou
`chart.show()`) dimensionne son `<iframe>` en ligne via `aspect-ratio` CSS,
que certains frontends de notebook résolvent de façon peu fiable pour de
grands canvas carrés comme celui-ci (1700×1700), rognant silencieusement
le rendu au lieu de le réduire pour qu'il tienne. `chart.save(path)`
combiné à `IPython.display.IFrame(src=path, width=..., height=...)`
contourne ce problème en réservant une taille en pixels explicite dès le
départ — le script interne de mise à l'échelle du canvas réduit alors la
composition complète à cette taille, donc vous la voyez toujours en
intégralité.

---

## Une constellation de vrais charts : satellites sur des lignes en pointillés

Une forme de composition différente des anneaux et du réseau central de
RéciTAC : un `sp.scatter()` central traçant la qualité moyenne de chaque
modèle contre ses émissions moyennes, et un petit `sp.scatter()` satellite
par modèle — son propre nuage d'événements bruts, réduit à de simples points
via `no_axes()`/`no_title()`/`hide_grid()`/`no_legend()`/`no_background()` —
disposés en anneau autour. Chaque satellite se relie à son point exact sur
le graphique central par une `line()` en pointillés.

Le lien satellite-point utilise `hover_group` directement sur `line()`/
`circle()`/`place()` plutôt qu'un appel `link()` séparé : passez la même
chaîne `hover_group="sat-3"` à chaque élément qui doit briller ensemble, et
ils sont liés dès leur création — pas besoin de `name=` puis d'un appel
`cv.link(...)` après coup. Les deux mécanismes aboutissent au même résultat ;
`hover_group` est la version en une ligne quand vous connaissez le groupe
dès la création.

```python
import math
import random
import seraplot as sp

random.seed(3)
MODELS = [("Titan-7B", "#6366f1"), ("Mixtral-Sparse", "#7c3aed"),
          ("Codex-T5", "#0891b2"), ("VisionSpeak-VL", "#0ea5e9"),
          ("DiffuGen-2", "#d97706"), ("BertCore", "#059669")]

means = [(random.uniform(55, 95), random.uniform(3.8, 5.2)) for _ in MODELS]
central = sp.scatter(
    "Quality vs log(emissions)",
    x=[m[0] for m in means], y=[m[1] for m in means],
    labels=[name for name, _ in MODELS], groups=[name for name, _ in MODELS],
    x_label="Quality score", y_label="log10 gCO2e", width=520, height=400,
)

CW = CH = 900
CX = CY = 450
R = 340
cv = sp.Canvas(CW, CH, "#ffffff")
cv.place(central, CX - 260, CY - 200, 520, 400)

for i, (name, color) in enumerate(MODELS):
    ang = math.radians(-90 + i * 360 / len(MODELS))
    mx, my = CX + R * math.cos(ang), CY + R * math.sin(ang)
    px = CX - 260 + 60 + (means[i][0] - 55) / 40 * 400
    py = CY - 200 + 30 + (1 - (means[i][1] - 3.8) / 1.4) * 330
    hg = f"sat-{i}"

    cv.line(px, py, mx, my, color=color, width=1.2, dash="2 5", hover_group=hg)

    mini = sp.scatter(
        "", x=[random.gauss(0, 1) for _ in range(30)],
        y=[random.gauss(0, 1) for _ in range(30)],
        labels=[name] * 30, groups=[name] * 30, palette=[int(color[1:], 16)],
        width=150, height=110,
    ).no_axes().no_title().hide_grid().no_legend().no_background().no_hover()
    cv.place(mini, mx - 75, my - 55, 150, 110, group=hg)
    cv.text(name, mx, my - 62, size=9, color=color, anchor="middle", weight="bold")

chart = cv.build()
```

<div class="sp-preview-frame"><iframe src="../previews/canvas-constellation.html?v=5bd7e57b" style="width:100%;height:640px;border:none;border-radius:8px;display:block;background:#ffffff" loading="lazy"></iframe></div>

Le jeu de données de l'aperçu ci-dessus est également synthétique — recréé
à partir du même projet de data-story `ai_story`, dont le fichier
`constellation.py` référençait des CSV qui n'existent plus. 14
architectures au lieu de 6, et le nuage d'événements de chaque satellite
provient de vraies données par modèle (jusqu'à 60 événements) plutôt que
d'une approximation gaussienne — la technique est identique, juste avec
plus de lignes et un second jeu de données jointu par `model_id`.

---

## Composer de vrais charts : un tableau de bord mission-control

`place()` intègre un `Chart` complet — pas seulement une primitive — dans
un canvas, ce qui signifie que la composition canvas ne se limite pas aux
formes dessinées à la main : de vrais panneaux `sp.line()`, `sp.bar()`,
`sp.gauge()`, `sp.area()`, `sp.donut()`, `sp.barh()` peuvent être encadrés,
connectés et annotés par les mêmes primitives que partout ailleurs sur
cette page. Ce qui fait vraiment la différence entre "tableau de bord" et
"charts dans des boîtes", c'est la même astuce que le réseau RéciTAC
ci-dessus : un **centre partagé auquel chaque panneau central se
connecte**. Un hub lumineux, 4 rayons colorés (`connector` + une ancre
`circle` à chaque bout), chaque rayon teinté pour correspondre à son
panneau via les méthodes chainables de niveau chart (`palette()`,
`gridlines()`, `width()`/`height()`, `title_color()` — voir
[Méthodes de graphique](../getting-started/chart-methods.md)) appliquées
avant `place()`. Au-delà des quatre panneaux du hub, un ruban de KPI avec
ses propres mini-graphiques `sp.line()` en ligne surplombe la grille, deux
vrais charts supplémentaires l'encadrent à droite via une connexion plus
subtile en `hover_group`, et un panneau de débit pleine largeur referme la
composition en bas :

```python
import random
import seraplot as sp

random.seed(3)

W, H = 1950, 1380
cv = sp.Canvas(W, H)

cv.radial_gradient("dashBg", "#1a2140", "#04050a", cx=0.5, cy=0.42, r=1.0)
cv.rect(0, 0, W, H, fill="url(#dashBg)", layer="bg")
cv.radial_gradient("hubGlow", "#22d3ee", "#04050a", cx=0.5, cy=0.5, r=0.5)

cv.text("Mission Control", 48, 56, size=30, color="#f8fafc", weight="800")
cv.text("Chaque panneau relié à un seul hub vivant — sp.Canvas place() + connecteurs + de vrais charts SeraPlot",
         48, 84, size=13, color="#64748b")

cv.text("mis à jour il y a 2s", W - 60, 50, size=11, color="#475569", anchor="end")
cv.circle(W - 260, 47, 5, fill="#22c55e", name="live-dot")
cv.circle(W - 260, 47, 5, fill="none", stroke="#22c55e", stroke_width=1.5, opacity=0.6, name="live-pulse")
cv.text("LIVE", W - 246, 52, size=12, color="#22c55e", weight="700", letter_spacing=1.5)

KPI = [
    ("UTILISATEURS ACTIFS", "12 940", "+6.1%", "#6366f1", [820, 860, 901, 934, 990, 1120, 1180, 1290, 1330, 1320]),
    ("MRR", "184.2k $", "+3.4%", "#22d3ee", [140, 148, 152, 149, 158, 165, 170, 176, 180, 184]),
    ("DISPONIBILITÉ", "99.982%", "+0.02%", "#f59e0b", [99.9, 99.91, 99.95, 99.93, 99.96, 99.97, 99.98, 99.97, 99.98, 99.982]),
    ("INCIDENTS OUVERTS", "5", "-2 aujourd'hui", "#f472b6", [9, 8, 8, 7, 6, 7, 6, 6, 5, 5]),
]
KPI_Y = 118
KPI_W, KPI_H = 340, 92
for i, (label, value, delta, color, series) in enumerate(KPI):
    kx = 48 + i * (KPI_W + 20)
    cv.rect(kx, KPI_Y, KPI_W, KPI_H, fill="#0b1022", stroke="rgba(255,255,255,.07)",
            stroke_width=1, rx=14, layer="bg", name=f"kpi-{i}")
    cv.rect(kx, KPI_Y, 4, KPI_H, fill=color, rx=2, layer="bg")
    cv.text(label, kx + 20, KPI_Y + 26, size=10.5, color="#64748b", weight="700", letter_spacing=1.2)
    cv.text(value, kx + 20, KPI_Y + 58, size=24, color="#f8fafc", weight="800")
    cv.text(delta, kx + 20, KPI_Y + 78, size=11.5, color=color, weight="600")

    spark = sp.line(labels=[str(j) for j in range(len(series))], values=series,
                     color_hex=int(color.lstrip("#"), 16), width=150, height=64) \
        .no_axes().no_title().hide_grid().no_legend().no_background().no_hover()
    cv.place(spark, kx + KPI_W - 168, KPI_Y + 16, 150, 64, name=f"kpi-spark-{i}")

GRID_Y0 = 250
PW, PH = 660, 320
GAP_X, GAP_Y = 100, 80
COL0, COL1 = 60, 60 + PW + GAP_X
ROW0, ROW1 = GRID_Y0, GRID_Y0 + PH + GAP_Y
HX, HY, HR = (COL0 + PW + GAP_X / 2), (ROW0 + PH + GAP_Y / 2), 92

PALETTE = [0x6366f1, 0x22d3ee, 0xf59e0b, 0xf472b6]
HEX = [f"#{c:06x}" for c in PALETTE]
PANELS = [("trend", COL0, ROW0, HEX[0]), ("revenue", COL1, ROW0, HEX[1]),
          ("health", COL0, ROW1, HEX[2]), ("incidents", COL1, ROW1, HEX[3])]

def panel_frame(x, y, w, h, color, name):
    cv.rect(x - 16, y - 16, w + 32, h + 32, fill="#0b1022", stroke="rgba(255,255,255,.06)",
            stroke_width=1, rx=18, layer="bg", name=name)
    cv.rect(x - 16, y - 16, w + 32, 4, fill=color, rx=2, layer="bg")

for name, x, y, color in PANELS:
    panel_frame(x, y, PW, PH, color, f"panel-{name}")

trend = sp.line(labels=["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"],
                 values=[820, 932, 901, 934, 1290, 1330, 1320],
                 ).width(PW).height(PH).palette(PALETTE).gridlines(False).background("#0b1022")

revenue = sp.bar(labels=["Core", "Cloud", "API", "Mobile", "Support"],
                  values=[420, 680, 310, 240, 150], title="Revenue by Segment",
                  ).width(PW).height(PH).palette(PALETTE).gridlines(False).background("#0b1022").title_color("#e2e8f0")

health = sp.gauge(value=87).width(PW).height(PH).background("#0b1022")

incidents = sp.area(labels=["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"], values=[5, 3, 6, 2, 4, 1, 2],
                     title="Open Incidents",
                     ).width(PW).height(PH).palette([PALETTE[3]]).gridlines(False).background("#0b1022").title_color("#e2e8f0")

cv.place(trend, COL0, ROW0, PW, PH, name="chart-trend")
cv.place(revenue, COL1, ROW0, PW, PH, name="chart-revenue")
cv.place(health, COL0, ROW1, PW, PH, name="chart-health")
cv.place(incidents, COL1, ROW1, PW, PH, name="chart-incidents")
cv.text("Weekly Active Users", COL0 + 20, ROW0 + 26, size=13, color="#e2e8f0", weight="700")
cv.text("System Health", COL0 + 20, ROW1 + 26, size=13, color="#e2e8f0", weight="700")

ANCHORS = {
    "trend": (COL0 + PW, ROW0 + PH / 2),
    "revenue": (COL1, ROW0 + PH / 2),
    "health": (COL0 + PW, ROW1 + PH / 2),
    "incidents": (COL1, ROW1 + PH / 2),
}
for name, x, y, color in PANELS:
    ax, ay = ANCHORS[name]
    cv.connector(ax, ay, HX, HY, color=color, width=2, opacity=0.55, bend=0.28, name=f"spoke-{name}")
    cv.circle(ax, ay, 6, fill=color, stroke="#04050a", stroke_width=2, name=f"anchor-{name}")
    cv.circle(ax, ay, 11, fill="none", stroke=color, stroke_width=1, opacity=0.4)

cv.circle(HX, HY, HR + 34, fill="url(#hubGlow)", opacity=0.35)
cv.ring(HX, HY, HR + 18, HR + 22, fill="#22d3ee", opacity=0.25)
cv.ring(HX, HY, HR + 6, HR + 9, fill="#22d3ee", opacity=0.45)

start = 0.0
for name, _, _, color in PANELS:
    end = start + 90
    cv.wedge(HX, HY, HR - 10, HR - 4, start, end, fill=color, opacity=0.85)
    start = end

cv.circle(HX, HY, HR - 14, fill="#0b1022", stroke="#f8fafc", stroke_width=2)
cv.text("87", HX, HY - 4, size=40, color="#22d3ee", weight="800", anchor="middle")
cv.text("SYSTEM SCORE", HX, HY + 22, size=10, color="#64748b", anchor="middle", letter_spacing=1.5)

cv.link("hub-cluster", ["anchor-trend", "anchor-revenue", "anchor-health", "anchor-incidents"])

cv.annotate("Trending up 61% since Monday", COL0 + 190, ROW0 + 170, COL0 + 40, ROW0 + PH + 55,
            color="#94a3b8", size=12, line_dash="4,3", bg="#0b1022")
cv.annotate("5 open, 2 critical", COL1 + 460, ROW1 + 90, COL1 + 40, ROW1 + PH + 55,
            color="#94a3b8", size=12, line_dash="4,3", bg="#0b1022")

SIDE_X = COL1 + PW + GAP_X
SIDE_W = W - SIDE_X - 60
panel_frame(SIDE_X, ROW0, SIDE_W, PH, "#a78bfa", "panel-region")
panel_frame(SIDE_X, ROW1, SIDE_W, PH, "#34d399", "panel-latency")

region = sp.donut(labels=["NA", "EU", "APAC", "LATAM"], values=[40, 28, 20, 12],
                   title="Regional Split", width=SIDE_W, height=PH,
                   palette=[0x6366f1, 0x22d3ee, 0xf59e0b, 0xf472b6]) \
    .background("#0b1022").title_color("#e2e8f0")

latency = sp.barh(labels=["p50", "p90", "p95", "p99"], values=[42, 118, 210, 480],
                   title="Latency (ms)", width=SIDE_W, height=PH,
                   palette=[0x34d399]) \
    .gridlines(False).background("#0b1022").title_color("#e2e8f0")

cv.place(region, SIDE_X, ROW0, SIDE_W, PH, group="side-link", name="panel-region-chart")
cv.place(latency, SIDE_X, ROW1, SIDE_W, PH, group="side-link", name="panel-latency-chart")

SIDE_LINK = [(ROW0, "#a78bfa"), (ROW1, "#34d399")]
for row_y, color in SIDE_LINK:
    ax, ay = COL1 + PW, row_y + PH / 2
    bx, by = SIDE_X, row_y + PH / 2
    cv.line(ax, ay, bx, by, color="#94a3b8", width=1, dash="2 5", hover_group="side-link")
    cv.circle(ax, ay, 5, fill=color, stroke="#04050a", stroke_width=2)
    cv.circle(bx, by, 5, fill=color, stroke="#04050a", stroke_width=2)

STRIP_Y = ROW1 + PH + GAP_Y
STRIP_H = H - STRIP_Y - 60
panel_frame(60, STRIP_Y, W - 120, STRIP_H, "#22d3ee", "panel-throughput")

hours = [f"{h:02d}:00" for h in range(0, 24, 2)]
throughput = [random.randint(800, 2600) for _ in hours]
throughput_chart = sp.bar(labels=hours, values=throughput, title="Requests/sec — last 24h",
                           width=W - 120, height=STRIP_H,
                           color_hex=0x22d3ee) \
    .gridlines(False).background("#0b1022").title_color("#e2e8f0")
cv.place(throughput_chart, 60, STRIP_Y, W - 120, STRIP_H, name="panel-throughput-chart")

chart = cv.build()
```

<div class="sp-preview-frame"><iframe src="../previews/canvas-dashboard.html?v=0e335555" style="width:100%;height:680px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe></div>

Le hub lui-même est une petite jauge radiale déguisée — 4 parts de `wedge`
(une par panneau, dans sa couleur) à l'intérieur de deux pistes `ring`
pulsées, construites avec exactement les mêmes primitives que le cadran
"Composer les micro-outils" et les anneaux donut RéciTAC ci-dessus.
Réutiliser un seul langage visuel sur chaque exemple travaillé de cette
page est tout l'intérêt : les primitives ne savent pas et ne se soucient
pas de dessiner un cadran de progression, un anneau de discipline, ou un
hub de tableau de bord.

`panel_frame()` dessine une carte arrondie plus une fine bordure supérieure
teintée — cette couleur d'accent est la même utilisée pour le rayon de ce
panneau et son `palette()`, pour que l'œil relie "cette ligne est orange"
à "ce panneau est orange" à "ce rayon est orange" sans légende. Le détail
facile à manquer derrière tout ça : toute décoration censée se trouver
*derrière* un chart placé (fond, cadres de panneau) a besoin de
`layer="bg"` explicitement — le `layer="fg"` par défaut de `rect()` se
dessine **au-dessus** des charts placés par conception (pour que
connecteurs et annotations puissent les traverser), ce qui masquera
silencieusement le contenu d'un panneau si celui-ci est dessiné sur la
couche de premier plan.

Deux choses ont dû être corrigées pour arriver ici depuis la version à
quatre panneaux d'origine. D'abord, `title_color()` : `sp.bar()`,
`sp.area()` et `sp.donut()` acceptaient volontiers une couleur de titre
explicite, mais `sp.line()`/`sp.gauge()` rendaient leur titre dans une
teinte par défaut à peine visible quoi qu'il arrive — contourné en
sautant complètement le `title=` propre au chart pour ces deux-là et en
dessinant le nom du panneau comme un simple `cv.text()` à la même
position, ce qui correspond aussi aux étiquettes dessinées à la main des
tuiles KPI. Ensuite, le générateur de graduations Y d'un chart réserve un
espace proportionnel à la taille en pixels *demandée* du plot, pas à son
empreinte `place()`e — les étiquettes de graduation du bandeau de débit du
bas se chevauchaient sérieusement jusqu'à ce que son `sp.bar()` soit
demandé à la taille native complète du bandeau (`width=W-120,
height=STRIP_H`) plutôt qu'une boîte sous-dimensionnée censée être étirée.

Les deux panneaux de droite (`Regional Split`, `Latency (ms)`) ne sont pas
câblés dans le hub — un cinquième et sixième rayon auraient surchargé
l'unique idée visuelle que le hub est censé communiquer — mais ils ne sont
pas orphelins pour autant : un seul `hover_group="side-link"` sur les deux
panneaux et leur ligne de connexion les relie exactement comme les
satellites RéciTAC, si bien que survoler l'un ou l'autre estompe tout ce
qui est en dehors de cette paire via le même comportement de survol
`Canvas` décrit plus haut.

---

## Mises en page organiques : Voronoi

`voronoi(sites, x, y, w, h, fills=None, stroke=..., stroke_width=..., opacity=...)`
calcule un diagramme de Voronoi borné — une cellule par site, chaque cellule
étant la région plus proche de ce site que de tout autre — et ajoute chaque
cellule au canvas comme un `polygon()` en un seul appel, en renvoyant leurs
indices d'éléments pour un adressage ultérieur (groupes de survol,
`derive()`, etc.).

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
*n'importe quel* élément lié (`Chart`, `Rect`, `Text`, `Circle`, `Wedge` ou
`Polygon`) fait briller/pulser tous les autres du même groupe. Renvoie le
nombre de noms effectivement liables (`Line`, `RawPath` et autres types
purement décoratifs ne le supportent pas encore). `circle(...)` et
`polygon(...)` acceptent aussi un paramètre `hover_group=` pour rejoindre un
groupe dès la création, sans appel `link()` séparé — les deux chemins posent
les mêmes attributs `data-sp-grp`/`data-group`, si bien que les cercles/
polygones groupés sont aussi pris en compte par la méthode chainable
`.group_hover_opacity(dim)` de n'importe quel chart (voir
[Méthodes de Chart](../getting-started/chart-methods.md)) — un seul survol
peut ainsi assombrir toutes les marques hors groupe sur le canvas pendant
que le halo/pulsation natif s'occupe de celles qui correspondent.

```python
cv.link("story", ["revenue_chart", "trend_chart", "kpi_card"])
cv.circle(120, 80, 6, fill="#2dd4bf", hover_group="alice", name="c1")
```

### `frieze` / `timeline` / `chronology` — disposition chronologique inter-plot

Trois noms pour le même outil : dispose `labels` en grille en serpentin
(boustrophédon) — de gauche à droite, puis de droite à gauche à la ligne
suivante, etc. — reliés par des segments droits au sein d'une ligne et par
une courbe en S à chaque retour à la ligne, avec un anneau englobant et une
étiquette par cellule. Renvoie l'ancre `(x, y, rayon_anneau)` de chaque
cellule, pour dessiner ou placer n'importe quel autre chart ou forme
directement dessus — idéal pour enfiler plusieurs amas
`circle_pack(variant="swarm")` mono-groupe, ou tout autre petit chart, sur
une vraie chronologie plutôt que de dépendre de la mise en page intégrée
d'un variant.

```python
anchors = cv.frieze(week_labels, weights=week_counts, cols=8,
                     cell_w=280, cell_h=280, ring_color="#7dd3fc")
for (x, y, r), commits_in_week in zip(anchors, weeks):
    ...
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
