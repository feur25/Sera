# Export (SVG / PNG / Data URL / HTML)

<div class="lang-en">

SeraPlot exposes a small, **language-universal** export layer. Every function
below is registered through the same `for_each_fn!` macro, so the exact same
behaviour is available from Python, JavaScript and the C-FFI.

The Python wrappers accept a `Chart` object directly (ergonomic sugar). The
JavaScript / FFI wrappers take a JSON string — the chart's `.html` field must
be passed inside `{"html": "..."}`.

---

## `sp.savefig(chart, path)`

Write the **complete HTML** of the chart to disk. The file is fully
self-contained: it embeds the SVG, its scripts and styles. It can be opened in
any browser, attached to an email, or served statically — no server, no CDN.

```python
import seraplot as sp

chart = sp.build_bar_chart("Revenue", ["Q1", "Q2"], [120, 180])
sp.savefig(chart, "revenue.html")
```

Errors are surfaced as `IOError`.

---

## `sp.export_svg(chart) -> str`

Returns the raw `<svg>...</svg>` block extracted from the chart. Useful for:

- Embedding the chart into a LaTeX / Word / Illustrator workflow
- Post-processing the geometry (CSS overrides, masks, custom filters)
- Server-side rasterisation with `cairosvg`, `resvg`, `librsvg`, etc.

```python
svg = sp.export_svg(chart)
with open("plot.svg", "w", encoding="utf-8") as f:
    f.write(svg)
```

Returns an empty string if no `<svg>` tag is found in the chart HTML.

---

## `sp.export_png(chart, path)`

Writes the SVG of the chart to disk under `path`. If `path` does not end in
`.svg`, the extension is rewritten to `.svg`. To convert to a true PNG raster,
pipe through any external converter — this keeps the binary small and avoids
shipping a 30 MB rasteriser as a hard dependency.

```python
sp.export_png(chart, "plot.svg")

import cairosvg
cairosvg.svg2png(url="plot.svg", write_to="plot.png", output_width=1920)
```

A `ValueError` is raised if the chart contains no `<svg>` block.

---

## `sp.export_data_url(chart) -> str`

Returns a `data:image/svg+xml;base64,...` URL that can be dropped directly into
an `<img src="...">`, a CSS `background-image: url(...)`, or any HTML email.

```python
url = sp.export_data_url(chart)
html = f'<img src="{url}" alt="Plot" />'
```

---

## `sp.chart_info(chart) -> str`

Returns a JSON string with structural metadata about the rendered chart. Handy
for logging, telemetry, or asserting chart complexity in unit tests.

```python
import json
info = json.loads(sp.chart_info(chart))
print(info)
```

```json
{"size": 18432, "paths": 12, "rects": 47, "circles": 0, "has_svg": true}
```

---

## JavaScript

```js
import * as sp from "seraplot";

const chart = sp.buildBarChart(JSON.stringify({
  title: "Revenue",
  labels: ["Q1", "Q2"],
  values: [120, 180],
}));

const svg     = sp.exportSvg(JSON.stringify({ html: chart }));
const dataUrl = sp.exportDataUrl(JSON.stringify({ html: chart }));
const written = sp.exportHtmlFile(JSON.stringify({ html: chart, path: "out.html" }));
const meta    = JSON.parse(sp.chartInfo(JSON.stringify({ html: chart })));
```

</div>

<div class="lang-fr">

SeraPlot expose une petite couche d'export **universelle entre langages**.
Chaque fonction ci-dessous est enregistrée via la même macro `for_each_fn!`, le
comportement est donc strictement identique depuis Python, JavaScript et le
C-FFI.

Les wrappers Python acceptent directement un objet `Chart` (sucre
ergonomique). Côté JavaScript / FFI, on passe une chaîne JSON — le champ
`.html` du chart doit être placé dans `{"html": "..."}`.

---

## `sp.savefig(chart, path)`

Écrit l'**intégralité du HTML** du chart sur disque. Le fichier est
auto-suffisant : il embarque le SVG, ses scripts et ses styles. Il s'ouvre
dans n'importe quel navigateur, peut être joint à un mail ou servi en
statique — aucun serveur, aucun CDN.

```python
import seraplot as sp

chart = sp.build_bar_chart("Chiffre d'affaires", ["T1", "T2"], [120, 180])
sp.savefig(chart, "revenue.html")
```

Les erreurs remontent en `IOError`.

---

## `sp.export_svg(chart) -> str`

Retourne le bloc `<svg>...</svg>` brut extrait du chart. Utile pour :

- Embarquer le chart dans un flux LaTeX / Word / Illustrator
- Post-traiter la géométrie (overrides CSS, masques, filtres custom)
- Rasteriser côté serveur avec `cairosvg`, `resvg`, `librsvg`, etc.

```python
svg = sp.export_svg(chart)
with open("plot.svg", "w", encoding="utf-8") as f:
    f.write(svg)
```

Retourne une chaîne vide si aucune balise `<svg>` n'est trouvée.

---

## `sp.export_png(chart, path)`

Écrit le SVG du chart sur disque sous `path`. Si `path` ne finit pas par
`.svg`, l'extension est réécrite en `.svg`. Pour obtenir un vrai PNG raster,
on passe par un convertisseur externe — cela garde le binaire compact et évite
d'embarquer un rasteriseur de 30 Mo en dépendance dure.

```python
sp.export_png(chart, "plot.svg")

import cairosvg
cairosvg.svg2png(url="plot.svg", write_to="plot.png", output_width=1920)
```

Un `ValueError` est levé si le chart ne contient aucun bloc `<svg>`.

---

## `sp.export_data_url(chart) -> str`

Retourne une URL `data:image/svg+xml;base64,...` directement injectable dans
un `<img src="...">`, un `background-image: url(...)` CSS, ou n'importe quel
mail HTML.

```python
url = sp.export_data_url(chart)
html = f'<img src="{url}" alt="Graphique" />'
```

---

## `sp.chart_info(chart) -> str`

Retourne une chaîne JSON contenant des métadonnées structurelles sur le chart
rendu. Pratique pour le logging, la télémétrie ou pour tester la complexité
d'un chart dans des tests unitaires.

```python
import json
info = json.loads(sp.chart_info(chart))
print(info)
```

```json
{"size": 18432, "paths": 12, "rects": 47, "circles": 0, "has_svg": true}
```

---

## JavaScript

```js
import * as sp from "seraplot";

const chart = sp.buildBarChart(JSON.stringify({
  title: "Chiffre d'affaires",
  labels: ["T1", "T2"],
  values: [120, 180],
}));

const svg     = sp.exportSvg(JSON.stringify({ html: chart }));
const dataUrl = sp.exportDataUrl(JSON.stringify({ html: chart }));
const written = sp.exportHtmlFile(JSON.stringify({ html: chart, path: "out.html" }));
const meta    = JSON.parse(sp.chartInfo(JSON.stringify({ html: chart })));
```

</div>
