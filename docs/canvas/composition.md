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

## Shapes and text

`text`, `line`, `curve`, `connector`, `circle`, `ring`, `rect`, `polygon`,
`path`, `arrow`, `annotate`, `gradient` draw directly on the canvas SVG layers
(`layer="bg"` renders under the placed charts, `layer="fg"` — the default —
renders on top). Every one of them accepts `name=""` for later addressing.

```python
cv.rect(40, 40, 300, 200, fill="#171724", stroke="rgba(148,163,184,.15)",
        rx=18, layer="bg", name="card")
cv.text("Revenue", 60, 70, size=16, color="#f8fafc", weight="700", name="title")
```

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

## Formes et texte

`text`, `line`, `curve`, `connector`, `circle`, `ring`, `rect`, `polygon`,
`path`, `arrow`, `annotate`, `gradient` dessinent directement sur les
couches SVG du canvas (`layer="bg"` sous les charts placés, `layer="fg"` —
par défaut — au-dessus). Chacune accepte `name=""` pour un adressage
ultérieur.

```python
cv.rect(40, 40, 300, 200, fill="#171724", stroke="rgba(148,163,184,.15)",
        rx=18, layer="bg", name="card")
cv.text("Revenue", 60, 70, size=16, color="#f8fafc", weight="700", name="title")
```

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
