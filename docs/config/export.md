# Export (SVG / PNG / HTML)

<div class="lang-en">

Export is exposed two ways: as `Chart` methods in Python (verified below against
a real build), and as standalone `for_each_fn!`-registered functions callable
from JavaScript / the C-FFI. The two are not a 1:1 mirror of each other today —
`chart_info` is the one function that is genuinely callable from both sides.

---

## `chart.save(path)` / `chart.export_html(path)`

Write the **complete HTML** of the chart to disk. The file is fully
self-contained: it embeds the SVG, its scripts and styles. It can be opened in
any browser, attached to an email, or served statically — no server, no CDN.

```python
import seraplot as sp

chart = sp.bar(title="Revenue", labels=["Q1", "Q2"], values=[120, 180])
chart.save("revenue.html")
```

---

## `chart.export_svg(path)`

Extracts the `<svg>...</svg>` block from the chart and writes it to `path`.
Useful for embedding into a LaTeX / Word / Illustrator workflow, or
post-processing the geometry (CSS overrides, masks, custom filters).

```python
chart.export_svg("plot.svg")
```

---

## `chart.export_png(path)`

Rasterizes the chart to PNG via `cairosvg`. Requires `pip install cairosvg` —
raises a clear error naming the missing dependency if it is not installed,
rather than failing silently.

```python
chart.export_png("plot.png")
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
{"size": 25488, "paths": 0, "rects": 3, "circles": 0, "has_svg": true}
```

---

## JavaScript

Only `buildBarChart` (and the equivalent `build*` function per chart family)
and `exportHtmlFile` are registered on the JS/WASM side today — there is no
JS equivalent of `export_svg`/`export_png`/`chart_info` yet.

```js
import * as sp from "seraplot";

const chart = sp.buildBarChart(JSON.stringify({
  title: "Revenue",
  labels: ["Q1", "Q2"],
  values: [120, 180],
}));

const written = sp.exportHtmlFile(JSON.stringify({ html: chart, path: "out.html" }));
```

</div>

<div class="lang-fr">

L'export est exposé de deux façons : comme méthodes sur `Chart` en Python
(vérifié ci-dessous contre un vrai build), et comme fonctions autonomes
enregistrées via `for_each_fn!`, appelables depuis JavaScript / le C-FFI. Les
deux ne sont pas un miroir 1:1 aujourd'hui — `chart_info` est la seule
fonction réellement appelable des deux côtés.

---

## `chart.save(path)` / `chart.export_html(path)`

Écrit l'**intégralité du HTML** du chart sur disque. Le fichier est
auto-suffisant : il embarque le SVG, ses scripts et ses styles. Il s'ouvre
dans n'importe quel navigateur, peut être joint à un mail ou servi en
statique — aucun serveur, aucun CDN.

```python
import seraplot as sp

chart = sp.bar(title="Chiffre d'affaires", labels=["T1", "T2"], values=[120, 180])
chart.save("revenue.html")
```

---

## `chart.export_svg(path)`

Extrait le bloc `<svg>...</svg>` du chart et l'écrit sous `path`. Utile pour
l'intégrer dans un flux LaTeX / Word / Illustrator, ou post-traiter la
géométrie (overrides CSS, masques, filtres custom).

```python
chart.export_svg("plot.svg")
```

---

## `chart.export_png(path)`

Rasterise le chart en PNG via `cairosvg`. Nécessite `pip install cairosvg` —
lève une erreur claire nommant la dépendance manquante plutôt que d'échouer
silencieusement.

```python
chart.export_png("plot.png")
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
{"size": 25488, "paths": 0, "rects": 3, "circles": 0, "has_svg": true}
```

---

## JavaScript

Seuls `buildBarChart` (et l'équivalent `build*` par famille de chart) et
`exportHtmlFile` sont enregistrés côté JS/WASM aujourd'hui — pas encore
d'équivalent JS pour `export_svg`/`export_png`/`chart_info`.

```js
import * as sp from "seraplot";

const chart = sp.buildBarChart(JSON.stringify({
  title: "Chiffre d'affaires",
  labels: ["T1", "T2"],
  values: [120, 180],
}));

const written = sp.exportHtmlFile(JSON.stringify({ html: chart, path: "out.html" }));
```

</div>
