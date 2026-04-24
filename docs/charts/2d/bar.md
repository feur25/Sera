# Bar Chart

<div class="lang-en">

## Signature

```python
sp.build_bar_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    color_hex: int = 0,
    orientation: str = "v",
    show_text: bool = False,
    color_groups: list[str] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    sort_order: str = "none",
    hover_json: str = "",
    legend_position: str = "right",
    palette: list[int] | None = None,
    series_names: list[str] | None = None,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.bar`, `sp.bar_chart`, `sp.bars`

---

## Description

Renders a vertical or horizontal bar chart.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title|
| `labels` | `list[str]` | required | Category labels|
| `values` | `list[float]` | required | Bar values|
| `color_hex` | `int` | `0` | Single bar color as hex int (e.g. `0xFF5733`)|
| `orientation` | `str` | `"v"` | `"v"` = vertical, `"h"` = horizontal |
| `show_text` | `bool` | `False` | Show value labels on bars|
| `color_groups` | `list[str] \| None` | `None` | Per-bar group name for coloring|
| `width` | `int` | `900` | Canvas width in pixels|
| `height` | `int` | `480` | Canvas height in pixels|
| `x_label` | `str` | `""` | X-axis label|
| `y_label` | `str` | `""` | Y-axis label|
| `gridlines` | `bool` | `False` | Show gridlines|
| `sort_order` | `str` | `"none"` | `"asc"`, `"desc"`, or `"none"` |
| `hover_json` | `str` | `""` | Custom hover tooltip JSON|
| `legend_position` | `str` | `"right"` | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `palette` | `list[int] \| None` | `None` | Custom color palette as list of hex ints |
| `background` | `str \| None` | `None` | Background color (e.g. `"#0f172a"`) or `None` = transparent |
| `no_x_axis` | `bool` | `False` | Hide X axis|
| `no_y_axis` | `bool` | `False` | Hide Y axis|

---

## Returns

`Chart` — object with `.html` property containing the full self-contained HTML.

---

## Examples

### Basic bar chart






<style>
.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}
.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155}
.sp-tb{padding:9px 22px;border:none;background:none;color:#64748b;cursor:pointer;font-size:13px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:#e2e8f0}
.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){hljs.highlightElement(c)})}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc code').forEach(function(c){hljs.highlightElement(c)})});
</script>
<div class="sp-tabs" id="bar">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('bar','bar-py',this)">Python</button><button class="sp-tb" onclick="spTab('bar','bar-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('bar','bar-ts',this)">TypeScript</button></div>
<div id="bar-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
labels = ["Jan", "Feb", "Mar", "Apr", "May"]
values = [1200.0, 1850.0, 2100.0, 1750.0, 2400.0]
logo = "https://raw.githubusercontent.com/feur25/seraplot-documentation/main/logo.png"
hover = sp.build_hover_json(labels, images=[logo] * len(labels))
chart = (
    sp.build_bar_chart(
        "Monthly Revenue",
        labels=labels,
        values=values,
        x_label="Month",
        y_label="Revenue (€)",
        gridlines=True,
        hover_json=hover,
    )
    .set_bg(None)
    .show_labels(position="top")
)</code></pre></div>
<div id="bar-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const labels = ["Jan", "Feb", "Mar", "Apr", "May"]
const values = [1200.0, 1850.0, 2100.0, 1750.0, 2400.0]
const logo = "https://raw.githubusercontent.com/feur25/seraplot-documentation/main/logo.png"
const hover = sp.build_hover_json(labels,
[logo])
const chart = (
    sp.build_bar_chart("Monthly Revenue",
labels,
{
    values: values,
    x_label: "Month",
    y_label: "Revenue (€)",
    gridlines: true,
    hover_json: hover
})
    .set_bg(null)
    .show_labels(position="top")
)</code></pre></div>
<div id="bar-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const labels: string[] = ["Jan", "Feb", "Mar", "Apr", "May"]
const values: number[] = [1200.0, 1850.0, 2100.0, 1750.0, 2400.0]
const logo: string = "https://raw.githubusercontent.com/feur25/seraplot-documentation/main/logo.png"
const hover = sp.build_hover_json(labels,
[logo])
const chart = (
    sp.build_bar_chart("Monthly Revenue",
labels,
{
    values: values,
    x_label: "Month",
    y_label: "Revenue (€)",
    gridlines: true,
    hover_json: hover
})
    .set_bg(null)
    .show_labels(position="top")
)</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/bar.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

### Colored groups

```python
chart = sp.build_bar_chart(
    "Products by Category",
    labels=["A1", "A2", "B1", "B2", "C1"],
    values=[10.0, 15.0, 8.0, 12.0, 20.0],
    color_groups=["Cat A", "Cat A", "Cat B", "Cat B", "Cat C"],
    legend_position="bottom",
)
```

### Dark background

```python
chart = sp.build_bar_chart(
    "Dark Theme",
    labels=["Q1", "Q2", "Q3", "Q4"],
    values=[300.0, 450.0, 380.0, 520.0],
    background="#0f172a",
    width=800,
    height=400,
)
```

---

## See also

- [Horizontal Bar](hbar.md) — `sp.build_hbar()`
- [Grouped Bar](grouped-bar.md) — `sp.build_grouped_bar()`
- [Stacked Bar](stacked-bar.md) — `sp.build_stacked_bar()`

</div>

<div class="lang-fr">

## Signature

```python
sp.build_bar_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    color_hex: int = 0,
    orientation: str = "v",
    show_text: bool = False,
    color_groups: list[str] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    sort_order: str = "none",
    hover_json: str = "",
    legend_position: str = "right",
    palette: list[int] | None = None,
    series_names: list[str] | None = None,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.bar`, `sp.bar_chart`, `sp.bars`

---

## Description

Affiche un graphique en barres vertical ou horizontal.

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Étiquettes des catégories |
| `values` | `list[float]` | requis | Valeurs des barres |
| `color_hex` | `int` | `0` | Couleur unique (ex. `0xFF5733`) |
| `orientation` | `str` | `"v"` | `"v"` = vertical, `"h"` = horizontal |
| `show_text` | `bool` | `False` | Afficher les valeurs sur les barres |
| `color_groups` | `list[str] \| None` | `None` | Groupe par barre pour la coloration |
| `width` | `int` | `900` | Largeur du canvas en pixels |
| `height` | `int` | `480` | Hauteur du canvas en pixels |
| `x_label` | `str` | `""` | Étiquette de l'axe X |
| `y_label` | `str` | `""` | Étiquette de l'axe Y |
| `gridlines` | `bool` | `False` | Afficher les lignes de grille |
| `sort_order` | `str` | `"none"` | `"asc"`, `"desc"` ou `"none"` |
| `hover_json` | `str` | `""` | JSON d'infobulle personnalisée |
| `legend_position` | `str` | `"right"` | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `palette` | `list[int] \| None` | `None` | Palette de couleurs personnalisée |
| `background` | `str \| None` | `None` | Couleur de fond ou `None` = transparent |
| `no_x_axis` | `bool` | `False` | Masquer l'axe X |
| `no_y_axis` | `bool` | `False` | Masquer l'axe Y |

---

## Retourne

`Chart` — objet avec la propriété `.html` contenant le HTML autonome complet.

---

## Exemples

### Graphique en barres simple

```python
import seraplot as sp

labels = ["Jan", "Fév", "Mar", "Avr", "Mai"]
values = [1200.0, 1850.0, 2100.0, 1750.0, 2400.0]

chart = (
    sp.build_bar_chart(
        "Chiffre d'affaires mensuel",
        labels=labels,
        values=values,
        x_label="Mois",
        y_label="Chiffre d'affaires (€)",
        gridlines=True,
    )
    .set_bg(None)
    .show_labels(position="top")
)
```

### Groupes colorés

```python
chart = sp.build_bar_chart(
    "Produits par catégorie",
    labels=["A1", "A2", "B1", "B2", "C1"],
    values=[10.0, 15.0, 8.0, 12.0, 20.0],
    color_groups=["Cat A", "Cat A", "Cat B", "Cat B", "Cat C"],
    legend_position="bottom",
)
```

---

## Voir aussi

- [Barres horizontales](hbar.md) — `sp.build_hbar()`
- [Barres groupées](grouped-bar.md) — `sp.build_grouped_bar()`
- [Barres empilées](stacked-bar.md) — `sp.build_stacked_bar()`

</div>
