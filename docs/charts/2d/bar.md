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

A bar chart encodes a single numeric variable as the length of rectangular bars drawn along a categorical axis. It is the most direct visual tool for comparing magnitudes across discrete categories and is universally understood by any audience. SeraPlot renders bar charts as fully self-contained HTML with hardware-accelerated SVG, delivering sub-millisecond frame times even with hundreds of bars. The `orientation` parameter switches between vertical (default) and horizontal layout without changing any other argument. Color groups let you assign a legend-backed color to each bar individually, turning the chart into a lightweight grouped or categorical view.

**Ideal for:**
- Comparing quantities across independent categories (sales by region, votes by candidate)
- Ranking items when combined with `sort_order="desc"`
- Highlighting categorical outliers with `color_groups` or a custom `palette`

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Category labels |
| `values` | `list[float]` | required | Bar values |
| `color_hex` | `int` | `0` | Single bar color as hex int (e.g. `0xFF5733`) |
| `orientation` | `str` | `"v"` | `"v"` = vertical, `"h"` = horizontal |
| `show_text` | `bool` | `False` | Show value labels on bars |
| `color_groups` | `list[str] \| None` | `None` | Per-bar group name for coloring |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `False` | Show gridlines |
| `sort_order` | `str` | `"none"` | `"asc"`, `"desc"`, or `"none"` |
| `hover_json` | `str` | `""` | Custom hover tooltip JSON |
| `legend_position` | `str` | `"right"` | `"right"`, `"left"`, `"top"`, `"bottom"` |
| `palette` | `list[int] \| None` | `None` | Custom color palette as list of hex ints |
| `background` | `str \| None` | `None` | Background color or `None` = transparent |
| `no_x_axis` | `bool` | `False` | Hide X axis |
| `no_y_axis` | `bool` | `False` | Hide Y axis |

---

## Returns

`Chart` — object with `.html` property containing the full self-contained HTML.

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="bar-basic">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bar-basic','bar-basic-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bar-basic','bar-basic-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bar-basic','bar-basic-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bar-basic','bar-basic-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bar-basic','bar-basic-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bar-basic','bar-basic-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bar-basic','bar-basic-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bar-basic','bar-basic-cpp',this)">C++</button>
</div>
<div id="bar-basic-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"]
revenue = [1200, 1850, 2100, 1750, 2400, 2800]
chart = sp.bar(
    title="Monthly Revenue",
    labels=months,
    values=revenue,
    y_label="Revenue (€)",
    gridlines=True,
    show_text=True,
)
chart.show()</code></pre></div>
<div id="bar-basic-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
const revenue = [1200, 1850, 2100, 1750, 2400, 2800];
const chart = sp.bar({
  title: "Monthly Revenue",
  labels: months,
  values: revenue,
  yLabel: "Revenue (€)",
  gridlines: true,
  showText: true,
});
chart.show();</code></pre></div>
<div id="bar-basic-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const months: string[] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
const revenue: number[] = [1200, 1850, 2100, 1750, 2400, 2800];
const chart = sp.bar({
  title: "Monthly Revenue",
  labels: months,
  values: revenue,
  yLabel: "Revenue (€)",
  gridlines: true,
  showText: true,
});
chart.show();</code></pre></div>
<div id="bar-basic-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
months <- c("Jan", "Feb", "Mar", "Apr", "May", "Jun")
revenue <- c(1200, 1850, 2100, 1750, 2400, 2800)
chart <- sp$bar(
  title = "Monthly Revenue",
  labels = months,
  values = revenue,
  y_label = "Revenue (€)",
  gridlines = TRUE,
  show_text = TRUE
)
chart$show()</code></pre></div>
<div id="bar-basic-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
var chart = SeraPlot.bar()
    .title("Monthly Revenue")
    .labels(List.of("Jan", "Feb", "Mar", "Apr", "May", "Jun"))
    .values(List.of(1200.0, 1850.0, 2100.0, 1750.0, 2400.0, 2800.0))
    .yLabel("Revenue (€)")
    .gridlines(true)
    .showText(true)
    .build();
chart.show();</code></pre></div>
<div id="bar-basic-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;
var chart = Sp.Bar(
    title: "Monthly Revenue",
    labels: ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
    values: [1200, 1850, 2100, 1750, 2400, 2800],
    yLabel: "Revenue (€)",
    gridlines: true,
    showText: true
);
chart.Show();</code></pre></div>
<div id="bar-basic-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp
val chart = sp.bar(
  title = "Monthly Revenue",
  labels = List("Jan", "Feb", "Mar", "Apr", "May", "Jun"),
  values = List(1200.0, 1850.0, 2100.0, 1750.0, 2400.0, 2800.0),
  y_label = "Revenue (€)",
  gridlines = true,
  show_text = true
)
chart.show()</code></pre></div>
<div id="bar-basic-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;
auto chart = sp::bar({
    .title    = "Monthly Revenue",
    .labels   = {"Jan", "Feb", "Mar", "Apr", "May", "Jun"},
    .values   = {1200.0, 1850.0, 2100.0, 1750.0, 2400.0, 2800.0},
    .y_label  = "Revenue (€)",
    .gridlines = true,
    .show_text = true,
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/bar.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [Horizontal Bar](hbar.md) — `sp.build_hbar()`
- [Grouped Bar](grouped-bar.md) — `sp.build_grouped_bar()`
- [Stacked Bar](stacked-bar.md) — `sp.build_stacked_bar()`
- [Lollipop](lollipop.md) — `sp.build_lollipop_chart()`

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

Un graphique en barres encode une variable numérique sous forme de barres rectangulaires dont la longueur est proportionnelle à la valeur. C'est l'outil visuel le plus direct pour comparer des magnitudes sur des catégories discrètes et il est universellement compris par tous les publics. SeraPlot produit des graphiques en barres sous forme de HTML autonome avec rendu SVG accéléré matériellement, offrant des temps de rendu inférieurs à la milliseconde même pour des centaines de barres. Le paramètre `orientation` bascule entre la disposition verticale (par défaut) et horizontale sans modifier aucun autre argument. Les groupes de couleurs permettent d'attribuer une couleur à chaque barre individuellement.

**Idéal pour :**
- Comparer des quantités entre catégories indépendantes (ventes par région, votes par candidat)
- Classer des éléments avec `sort_order="desc"`
- Mettre en évidence des valeurs atypiques avec `color_groups` ou une `palette` personnalisée

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
| `width` | `int` | `900` | Largeur en pixels |
| `height` | `int` | `480` | Hauteur en pixels |
| `x_label` | `str` | `""` | Étiquette de l'axe X |
| `y_label` | `str` | `""` | Étiquette de l'axe Y |
| `gridlines` | `bool` | `False` | Afficher les lignes de grille |
| `sort_order` | `str` | `"none"` | `"asc"`, `"desc"` ou `"none"` |
| `hover_json` | `str` | `""` | JSON d'infobulle personnalisée |
| `legend_position` | `str` | `"right"` | Position de la légende |
| `palette` | `list[int] \| None` | `None` | Palette de couleurs personnalisée |
| `background` | `str \| None` | `None` | Couleur de fond ou `None` = transparent |
| `no_x_axis` | `bool` | `False` | Masquer l'axe X |
| `no_y_axis` | `bool` | `False` | Masquer l'axe Y |

---

## Retourne

`Chart` — objet avec la propriété `.html` contenant le HTML autonome complet.

---

<div class="sp-tabs" id="bar-basic-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bar-basic-fr','bar-bfr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bar-basic-fr','bar-bfr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bar-basic-fr','bar-bfr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bar-basic-fr','bar-bfr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bar-basic-fr','bar-bfr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bar-basic-fr','bar-bfr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bar-basic-fr','bar-bfr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bar-basic-fr','bar-bfr-cpp',this)">C++</button>
</div>
<div id="bar-bfr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
mois = ["Jan", "Fév", "Mar", "Avr", "Mai", "Jun"]
ca   = [1200, 1850, 2100, 1750, 2400, 2800]
chart = sp.bar(
    title="Chiffre d'affaires mensuel",
    labels=mois,
    values=ca,
    y_label="CA (€)",
    gridlines=True,
    show_text=True,
)
chart.show()</code></pre></div>
<div id="bar-bfr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const mois = ["Jan", "Fév", "Mar", "Avr", "Mai", "Jun"];
const ca   = [1200, 1850, 2100, 1750, 2400, 2800];
const chart = sp.bar({
  title: "Chiffre d'affaires mensuel",
  labels: mois,
  values: ca,
  yLabel: "CA (€)",
  gridlines: true,
  showText: true,
});
chart.show();</code></pre></div>
<div id="bar-bfr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.bar({
  title: "Chiffre d'affaires mensuel",
  labels: ["Jan", "Fév", "Mar", "Avr", "Mai", "Jun"],
  values: [1200, 1850, 2100, 1750, 2400, 2800],
  yLabel: "CA (€)",
  gridlines: true,
  showText: true,
});
chart.show();</code></pre></div>
<div id="bar-bfr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$bar(
  title  = "Chiffre d'affaires mensuel",
  labels = c("Jan", "Fév", "Mar", "Avr", "Mai", "Jun"),
  values = c(1200, 1850, 2100, 1750, 2400, 2800),
  y_label = "CA (€)",
  gridlines = TRUE,
  show_text = TRUE
)
chart$show()</code></pre></div>
<div id="bar-bfr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
var chart = SeraPlot.bar()
    .title("Chiffre d'affaires mensuel")
    .labels(List.of("Jan", "Fév", "Mar", "Avr", "Mai", "Jun"))
    .values(List.of(1200.0, 1850.0, 2100.0, 1750.0, 2400.0, 2800.0))
    .yLabel("CA (€)")
    .gridlines(true)
    .showText(true)
    .build();
chart.show();</code></pre></div>
<div id="bar-bfr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;
var chart = Sp.Bar(
    title: "Chiffre d'affaires mensuel",
    labels: ["Jan", "Fév", "Mar", "Avr", "Mai", "Jun"],
    values: [1200, 1850, 2100, 1750, 2400, 2800],
    yLabel: "CA (€)",
    gridlines: true,
    showText: true
);
chart.Show();</code></pre></div>
<div id="bar-bfr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp
val chart = sp.bar(
  title  = "Chiffre d'affaires mensuel",
  labels = List("Jan", "Fév", "Mar", "Avr", "Mai", "Jun"),
  values = List(1200.0, 1850.0, 2100.0, 1750.0, 2400.0, 2800.0),
  y_label = "CA (€)",
  gridlines = true,
  show_text = true
)
chart.show()</code></pre></div>
<div id="bar-bfr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;
auto chart = sp::bar({
    .title     = "Chiffre d'affaires mensuel",
    .labels    = {"Jan", "Fév", "Mar", "Avr", "Mai", "Jun"},
    .values    = {1200.0, 1850.0, 2100.0, 1750.0, 2400.0, 2800.0},
    .y_label   = "CA (€)",
    .gridlines = true,
    .show_text = true,
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/bar.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

## Voir aussi

- [Barres horizontales](hbar.md) — `sp.build_hbar()`
- [Barres groupées](grouped-bar.md) — `sp.build_grouped_bar()`
- [Barres empilées](stacked-bar.md) — `sp.build_stacked_bar()`
- [Lollipop](lollipop.md) — `sp.build_lollipop_chart()`

</div>
