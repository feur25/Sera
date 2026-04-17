# Grouped Bar Chart

<div class="lang-en">

## Signature
    title: str,
    category_labels: list[str],
    series_values: list[float],
    *,
    show_values: bool = False,
    series_names: list[str] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    sort_order: str = "none",
    hover_json: str = "",
    legend_position: str = "right",
    palette: list[int] | None = None,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

---

## Description

Grouped bar chart for comparing multiple series across categories.

`series_values` must be a **flat list** of length `n_categories × n_series`, row-major (category-first).

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `category_labels` | `list[str]` | required | Category names on X axis |
| `series_values` | `list[float]` | required | Flat values: `[cat0_s0, cat0_s1, cat1_s0, cat1_s1, ...]` |
| `show_values` | `bool` | `False` | Show value labels |
| `series_names` | `list[str] \| None` | `None` | Series names for legend |
| `palette` | `list[int] \| None` | `None` | Custom color palette |

---

## Returns

`Chart`

---

## Examples






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
<div class="sp-tabs" id="grouped-bar">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('grouped-bar','grouped-bar-py',this)">Python</button><button class="sp-tb" onclick="spTab('grouped-bar','grouped-bar-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('grouped-bar','grouped-bar-ts',this)">TypeScript</button></div>
<div id="grouped-bar-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

categories = ["Q1", "Q2", "Q3", "Q4"]
values = [
    120.0, 90.0, 150.0,
    130.0, 110.0, 140.0,
    100.0, 95.0,  160.0,
    140.0, 120.0, 175.0,
]

logo = "https://raw.githubusercontent.com/feur25/seraplot-documentation/main/logo.png"
hover = sp.build_hover_json(categories * 3, images=[logo] * len(categories * 3))

chart = (
    sp.build_grouped_bar(
        "Quarterly Sales by Product",
        category_labels=categories,
        series_values=values,
        series_names=["Product A", "Product B", "Product C"],
        show_values=True,
        gridlines=True,
        legend_position="bottom",
        hover_json=hover,
    )
    .set_bg(None)
)</code></pre></div>
<div id="grouped-bar-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const categories = ["Q1", "Q2", "Q3", "Q4"]
const values = [
    120.0, 90.0, 150.0,
    130.0, 110.0, 140.0,
    100.0, 95.0,  160.0,
    140.0, 120.0, 175.0,
]

const logo = "https://raw.githubusercontent.com/feur25/seraplot-documentation/main/logo.png"
const hover = sp.build_hover_json(categories * 3,
[logo])

const chart = (
    sp.build_grouped_bar("Quarterly Sales by Product",
categories,
{
    series_values: values,
    series_names: ["Product A", "Product B", "Product C"],
    show_values: true,
    gridlines: true,
    legend_position: "bottom",
    hover_json: hover
})
    .set_bg(null)
)</code></pre></div>
<div id="grouped-bar-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const categories: string[] = ["Q1", "Q2", "Q3", "Q4"]
const values: number[] = [
    120.0, 90.0, 150.0,
    130.0, 110.0, 140.0,
    100.0, 95.0,  160.0,
    140.0, 120.0, 175.0,
]

const logo: string = "https://raw.githubusercontent.com/feur25/seraplot-documentation/main/logo.png"
const hover = sp.build_hover_json(categories * 3,
[logo])

const chart = (
    sp.build_grouped_bar("Quarterly Sales by Product",
categories,
{
    series_values: values,
    series_names: ["Product A", "Product B", "Product C"],
    show_values: true,
    gridlines: true,
    legend_position: "bottom",
    hover_json: hover
})
    .set_bg(null)
)</code></pre></div>
</div>


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/grouped-bar.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Stacked Bar](stacked-bar.md) — `sp.build_stacked_bar()`
- [Bar Chart](bar.md)

</div>

<div class="lang-fr">

## Signature

```python
sp.build_grouped_bar(
    title: str,
    category_labels: list[str],
    series_values: list[float],
    *,
    show_values: bool = False,
    series_names: list[str] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    sort_order: str = "none",
    hover_json: str = "",
    legend_position: str = "right",
    palette: list[int] | None = None,
    background: str | None = None,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

---

## Description

Graphique en barres groupées pour comparer plusieurs séries sur les mêmes catégories.

`series_values` doit être une **liste plate** de longueur `n_catégories × n_séries`, en ordre catégorie-major.

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `category_labels` | `list[str]` | requis | Noms des catégories sur l'axe X |
| `series_values` | `list[float]` | requis | Valeurs plates : `[cat0_s0, cat0_s1, cat1_s0, cat1_s1, ...]` |
| `show_values` | `bool` | `False` | Afficher les étiquettes de valeur |
| `series_names` | `list[str] \| None` | `None` | Noms des séries pour la légende |
| `palette` | `list[int] \| None` | `None` | Palette de couleurs personnalisée |
| `width` | `int` | `900` | Largeur du canvas |
| `height` | `int` | `480` | Hauteur du canvas |
| `legend_position` | `str` | `"right"` | Position de la légende |
| `gridlines` | `bool` | `False` | Lignes de grille |

---

## Retourne

`Chart`

---

## Exemples

```python
import seraplot as sp

categories = ["T1", "T2", "T3", "T4"]
valeurs = [
    120.0, 90.0, 150.0,
    130.0, 110.0, 140.0,
    100.0, 95.0,  160.0,
    140.0, 120.0, 175.0,
]

chart = (
    sp.build_grouped_bar(
        "Ventes trimestrielles par produit",
        category_labels=categories,
        series_values=valeurs,
        series_names=["Produit A", "Produit B", "Produit C"],
        show_values=True,
        gridlines=True,
        legend_position="bottom",
    )
    .set_bg(None)
)
```

---

## Voir aussi

- [Barres empilées](stacked-bar.md) — `sp.build_stacked_bar()`
- [Graphique en barres](bar.md)

</div>
