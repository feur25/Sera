# Grouped Bar Chart

<div class="lang-en">

## Signature

```python
sp.build_grouped_bar(
    title: str,
    category_labels: list[str],
    series_values: list[float],
    *,
    show_values: bool = False,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    legend_position: str = "right",
    gridlines: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.grouped_bar`

---

## Description

A grouped bar chart places bars for multiple series side by side within each category, enabling direct visual comparison of values across both categories and series simultaneously. `series_values` is a flat list in row-major order: `[cat0_s0, cat0_s1, cat1_s0, cat1_s1, ...]` — the total length must equal `len(category_labels) × n_series`. SeraPlot infers the number of series from `series_names` if provided, otherwise from `len(series_values) / len(category_labels)`. Grouped bar charts are the counterpart to stacked bar charts; use grouped when absolute values matter more than composition.

**Ideal for:**
- Comparing multiple product lines, regions, or periods side by side per category
- A/B testing results across user segments or multiple metrics
- Quarterly comparisons across several KPIs in a single compact view

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `category_labels` | `list[str]` | required | Category labels (X axis) |
| `series_values` | `list[float]` | required | Flat row-major values: `[cat0_s0, cat0_s1, cat1_s0, ...]` |
| `show_values` | `bool` | `False` | Show value labels on bars |
| `series_names` | `list[str] \| None` | `None` | Series names for the legend |
| `palette` | `list[int] \| None` | `None` | Custom color palette as list of hex ints |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `legend_position` | `str` | `"right"` | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `gridlines` | `bool` | `False` | Show gridlines |
| `no_y_axis` | `bool` | `False` | Hide Y axis |

---

## Returns

`Chart`

---

## Examples

### Quarterly sales by region

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="grouped-bar">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('grouped-bar','grouped-bar-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('grouped-bar','grouped-bar-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('grouped-bar','grouped-bar-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('grouped-bar','grouped-bar-r',this)">R</button>
<button class="sp-tb" onclick="spTab('grouped-bar','grouped-bar-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('grouped-bar','grouped-bar-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('grouped-bar','grouped-bar-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('grouped-bar','grouped-bar-cpp',this)">C++</button>
</div>
<div id="grouped-bar-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

quarters = ["Q1", "Q2", "Q3", "Q4"]
# Flat: [q1_north, q1_south, q1_east, q2_north, q2_south, q2_east, ...]
values = [
    320, 280, 350,
    380, 310, 400,
    420, 360, 450,
    480, 390, 510,
]

chart = sp.grouped_bar(
    title="Quarterly Sales by Region",
    category_labels=quarters,
    series_values=values,
    series_names=["North", "South", "East"],
    legend_position="top",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="grouped-bar-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const quarters = ["Q1", "Q2", "Q3", "Q4"];
const values = [
  320, 280, 350,
  380, 310, 400,
  420, 360, 450,
  480, 390, 510,
];

const chart = sp.groupedBar({
  title: "Quarterly Sales by Region",
  categoryLabels: quarters,
  seriesValues: values,
  seriesNames: ["North", "South", "East"],
  legendPosition: "top",
  gridlines: true,
});
chart.show();</code></pre></div>
<div id="grouped-bar-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const quarters: string[] = ["Q1", "Q2", "Q3", "Q4"];
const values: number[] = [
  320, 280, 350,
  380, 310, 400,
  420, 360, 450,
  480, 390, 510,
];

const chart = sp.groupedBar({
  title: "Quarterly Sales by Region",
  categoryLabels: quarters,
  seriesValues: values,
  seriesNames: ["North", "South", "East"],
  legendPosition: "top",
  gridlines: true,
});
chart.show();</code></pre></div>
<div id="grouped-bar-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

quarters <- c("Q1", "Q2", "Q3", "Q4")
values <- c(
  320, 280, 350,
  380, 310, 400,
  420, 360, 450,
  480, 390, 510
)

chart <- sp$grouped_bar(
  title            = "Quarterly Sales by Region",
  category_labels  = quarters,
  series_values    = values,
  series_names     = c("North", "South", "East"),
  legend_position  = "top",
  gridlines        = TRUE
)
chart$show()</code></pre></div>
<div id="grouped-bar-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.groupedBar()
    .title("Quarterly Sales by Region")
    .categoryLabels(List.of("Q1", "Q2", "Q3", "Q4"))
    .seriesValues(List.of(
        320.0, 280.0, 350.0,
        380.0, 310.0, 400.0,
        420.0, 360.0, 450.0,
        480.0, 390.0, 510.0
    ))
    .seriesNames(List.of("North", "South", "East"))
    .legendPosition("top")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="grouped-bar-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.GroupedBar(
    title:          "Quarterly Sales by Region",
    categoryLabels: ["Q1", "Q2", "Q3", "Q4"],
    seriesValues:   [320, 280, 350, 380, 310, 400, 420, 360, 450, 480, 390, 510],
    seriesNames:    ["North", "South", "East"],
    legendPosition: "top",
    gridlines:      true
);
chart.Show();</code></pre></div>
<div id="grouped-bar-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.grouped_bar(
  title           = "Quarterly Sales by Region",
  category_labels = List("Q1", "Q2", "Q3", "Q4"),
  series_values   = List(
    320.0, 280.0, 350.0,
    380.0, 310.0, 400.0,
    420.0, 360.0, 450.0,
    480.0, 390.0, 510.0
  ),
  series_names    = List("North", "South", "East"),
  legend_position = "top",
  gridlines       = true
)
chart.show()</code></pre></div>
<div id="grouped-bar-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::grouped_bar({
    .title           = "Quarterly Sales by Region",
    .category_labels = {"Q1", "Q2", "Q3", "Q4"},
    .series_values   = {
        320.0, 280.0, 350.0,
        380.0, 310.0, 400.0,
        420.0, 360.0, 450.0,
        480.0, 390.0, 510.0
    },
    .series_names    = {"North", "South", "East"},
    .legend_position = "top",
    .gridlines       = true,
});
chart.show();</code></pre></div>
</div>

<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>
<iframe src="../../previews/grouped-bar.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>
</details>

### Show values with custom palette

```python
chart = sp.grouped_bar(
    title="Browser Market Share by Year",
    category_labels=["2021", "2022", "2023"],
    series_values=[65, 19, 10, 68, 18, 9, 70, 16, 8],
    series_names=["Chrome", "Safari", "Firefox"],
    palette=[0x4285f4, 0x34a853, 0xff6d00],
    show_values=True,
    legend_position="bottom",
)
```

---

## See also

- [Bar Chart](bar.md) — `sp.build_bar_chart()`
- [Stacked Bar](stacked-bar.md) — `sp.build_stacked_bar()`
- [Multiline Chart](multiline.md) — `sp.build_multiline()`

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
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    legend_position: str = "right",
    gridlines: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.grouped_bar`

---

## Description

Un graphique en barres groupées place les barres de plusieurs séries côte à côte au sein de chaque catégorie, permettant une comparaison visuelle directe des valeurs entre catégories et séries simultanément. `series_values` est une liste plate en ordre ligne-majeur : `[cat0_s0, cat0_s1, cat1_s0, cat1_s1, ...]` — la longueur totale doit être égale à `len(category_labels) × n_séries`. SeraPlot déduit le nombre de séries depuis `series_names` si fourni, sinon depuis `len(series_values) / len(category_labels)`. Les barres groupées sont le pendant des barres empilées ; utilisez le groupé quand les valeurs absolues comptent plus que la composition.

**Idéal pour :**
- Comparer plusieurs lignes de produits, régions ou périodes côte à côte par catégorie
- Résultats de tests A/B sur plusieurs segments d'utilisateurs
- Comparaisons trimestrielles de plusieurs indicateurs clés en une seule vue

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `category_labels` | `list[str]` | requis | Étiquettes des catégories (axe X) |
| `series_values` | `list[float]` | requis | Valeurs en ordre ligne-majeur : `[cat0_s0, cat0_s1, cat1_s0, ...]` |
| `show_values` | `bool` | `False` | Afficher les valeurs sur les barres |
| `series_names` | `list[str] \| None` | `None` | Noms des séries pour la légende |
| `palette` | `list[int] \| None` | `None` | Palette de couleurs personnalisée |
| `width` | `int` | `900` | Largeur du canvas en pixels |
| `height` | `int` | `480` | Hauteur du canvas en pixels |
| `legend_position` | `str` | `"right"` | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `gridlines` | `bool` | `False` | Afficher les lignes de grille |
| `no_y_axis` | `bool` | `False` | Masquer l'axe Y |

---

## Retourne

`Chart`

---

## Exemples

### Ventes trimestrielles par région

<div class="sp-tabs" id="grouped-bar-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('grouped-bar-fr','grouped-bar-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('grouped-bar-fr','grouped-bar-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('grouped-bar-fr','grouped-bar-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('grouped-bar-fr','grouped-bar-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('grouped-bar-fr','grouped-bar-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('grouped-bar-fr','grouped-bar-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('grouped-bar-fr','grouped-bar-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('grouped-bar-fr','grouped-bar-fr-cpp',this)">C++</button>
</div>
<div id="grouped-bar-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

trimestres = ["T1", "T2", "T3", "T4"]
valeurs = [
    320, 280, 350,
    380, 310, 400,
    420, 360, 450,
    480, 390, 510,
]

chart = sp.grouped_bar(
    title="Ventes trimestrielles par région",
    category_labels=trimestres,
    series_values=valeurs,
    series_names=["Nord", "Sud", "Est"],
    legend_position="top",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="grouped-bar-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const trimestres = ["T1", "T2", "T3", "T4"];
const valeurs = [
  320, 280, 350,
  380, 310, 400,
  420, 360, 450,
  480, 390, 510,
];

const chart = sp.groupedBar({
  title: "Ventes trimestrielles par région",
  categoryLabels: trimestres,
  seriesValues: valeurs,
  seriesNames: ["Nord", "Sud", "Est"],
  legendPosition: "top",
  gridlines: true,
});
chart.show();</code></pre></div>
<div id="grouped-bar-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const trimestres: string[] = ["T1", "T2", "T3", "T4"];
const valeurs: number[] = [
  320, 280, 350,
  380, 310, 400,
  420, 360, 450,
  480, 390, 510,
];

const chart = sp.groupedBar({
  title: "Ventes trimestrielles par région",
  categoryLabels: trimestres,
  seriesValues: valeurs,
  seriesNames: ["Nord", "Sud", "Est"],
  legendPosition: "top",
  gridlines: true,
});
chart.show();</code></pre></div>
<div id="grouped-bar-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

trimestres <- c("T1", "T2", "T3", "T4")
valeurs <- c(
  320, 280, 350,
  380, 310, 400,
  420, 360, 450,
  480, 390, 510
)

chart <- sp$grouped_bar(
  title           = "Ventes trimestrielles par région",
  category_labels = trimestres,
  series_values   = valeurs,
  series_names    = c("Nord", "Sud", "Est"),
  legend_position = "top",
  gridlines       = TRUE
)
chart$show()</code></pre></div>
<div id="grouped-bar-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.groupedBar()
    .title("Ventes trimestrielles par région")
    .categoryLabels(List.of("T1", "T2", "T3", "T4"))
    .seriesValues(List.of(
        320.0, 280.0, 350.0,
        380.0, 310.0, 400.0,
        420.0, 360.0, 450.0,
        480.0, 390.0, 510.0
    ))
    .seriesNames(List.of("Nord", "Sud", "Est"))
    .legendPosition("top")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="grouped-bar-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.GroupedBar(
    title:          "Ventes trimestrielles par région",
    categoryLabels: ["T1", "T2", "T3", "T4"],
    seriesValues:   [320, 280, 350, 380, 310, 400, 420, 360, 450, 480, 390, 510],
    seriesNames:    ["Nord", "Sud", "Est"],
    legendPosition: "top",
    gridlines:      true
);
chart.Show();</code></pre></div>
<div id="grouped-bar-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.grouped_bar(
  title           = "Ventes trimestrielles par région",
  category_labels = List("T1", "T2", "T3", "T4"),
  series_values   = List(
    320.0, 280.0, 350.0,
    380.0, 310.0, 400.0,
    420.0, 360.0, 450.0,
    480.0, 390.0, 510.0
  ),
  series_names    = List("Nord", "Sud", "Est"),
  legend_position = "top",
  gridlines       = true
)
chart.show()</code></pre></div>
<div id="grouped-bar-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::grouped_bar({
    .title           = "Ventes trimestrielles par région",
    .category_labels = {"T1", "T2", "T3", "T4"},
    .series_values   = {
        320.0, 280.0, 350.0,
        380.0, 310.0, 400.0,
        420.0, 360.0, 450.0,
        480.0, 390.0, 510.0
    },
    .series_names    = {"Nord", "Sud", "Est"},
    .legend_position = "top",
    .gridlines       = true,
});
chart.show();</code></pre></div>
</div>

---

## Voir aussi

- [Graphique en barres](bar.md) — `sp.build_bar_chart()`
- [Barres empilées](stacked-bar.md) — `sp.build_stacked_bar()`
- [Graphique multiligne](multiline.md) — `sp.build_multiline()`

</div>
