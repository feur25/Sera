# Pie Chart

<div class="lang-en">

## Signature

```python
sp.build_pie_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_pct: bool = True,
    width: int = 700,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    hover_json: str | None = None,
    legend_position: str = "right",
) -> Chart
```

Aliases: `sp.pie`, `sp.pie_chart`

---

## Description

A pie chart divides a circle into proportional slices, where each slice's arc angle is proportional to its share of the total. It is most effective when comparing a small number of categories (3–6) that together form a meaningful whole, such as market share or budget allocation. SeraPlot automatically normalizes values to 100% so raw counts, percentages, or any numeric data can be passed directly. For hollow-center aesthetics, use the Donut chart variant.

**Ideal for:**
- Showing part-to-whole composition for 3–6 categories
- Displaying market share, budget splits, or survey results
- Presenting data for non-technical audiences who expect a familiar chart form

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | — | Chart title displayed at the top |
| `labels` | `list[str]` | — | Category name for each slice |
| `values` | `list[float]` | — | Numeric value for each slice; automatically normalized to 100% |
| `show_pct` | `bool` | `True` | Display percentage labels inside or beside each slice |
| `width` | `int` | `700` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `palette` | `list[int] \| None` | `None` | Custom slice colors as hex integers |
| `background` | `str \| None` | `None` | CSS background color override |
| `hover_json` | `str \| None` | `None` | JSON string for custom tooltip data |
| `legend_position` | `str` | `"right"` | Legend placement: `"top"`, `"right"`, `"bottom"`, or `"left"` |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="pie">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('pie','pie-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('pie','pie-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('pie','pie-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('pie','pie-r',this)">R</button>
<button class="sp-tb" onclick="spTab('pie','pie-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('pie','pie-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('pie','pie-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('pie','pie-cpp',this)">C++</button>
</div>
<div id="pie-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.pie(
    title="Market share Q4",
    labels=["SeraPlot", "Matplotlib", "Plotly", "Bokeh", "Other"],
    values=[35, 25, 20, 10, 10],
)
chart.show()</code></pre></div>
<div id="pie-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.pie({
  title: "Market share Q4",
  labels: ["SeraPlot", "Matplotlib", "Plotly", "Bokeh", "Other"],
  values: [35, 25, 20, 10, 10],
});
chart.show();</code></pre></div>
<div id="pie-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.pie({
  title: "Market share Q4",
  labels: ["SeraPlot", "Matplotlib", "Plotly", "Bokeh", "Other"],
  values: [35, 25, 20, 10, 10],
});
chart.show();</code></pre></div>
<div id="pie-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$pie(
  title = "Market share Q4",
  labels = c("SeraPlot", "Matplotlib", "Plotly", "Bokeh", "Other"),
  values = c(35, 25, 20, 10, 10)
)
chart$show()</code></pre></div>
<div id="pie-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.pie()
    .title("Market share Q4")
    .labels(List.of("SeraPlot", "Matplotlib", "Plotly", "Bokeh", "Other"))
    .values(List.of(35.0, 25.0, 20.0, 10.0, 10.0))
    .build();
chart.show();</code></pre></div>
<div id="pie-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Pie(
    title: "Market share Q4",
    labels: new[]{"SeraPlot", "Matplotlib", "Plotly", "Bokeh", "Other"},
    values: new[]{35.0, 25.0, 20.0, 10.0, 10.0}
);
chart.Show();</code></pre></div>
<div id="pie-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.pie(
  title = "Market share Q4",
  labels = List("SeraPlot", "Matplotlib", "Plotly", "Bokeh", "Other"),
  values = List(35.0, 25.0, 20.0, 10.0, 10.0)
)
chart.show()</code></pre></div>
<div id="pie-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::pie({
  .title  = "Market share Q4",
  .labels = {"SeraPlot", "Matplotlib", "Plotly", "Bokeh", "Other"},
  .values = {35.0, 25.0, 20.0, 10.0, 10.0}
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/pie.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [donut.md](donut.md) — Pie chart with a circular cutout at the center
- [sunburst.md](sunburst.md) — Hierarchical pie with nested rings
- [treemap.md](treemap.md) — Nested rectangles for part-to-whole hierarchies

</div>

<div class="lang-fr">

## Signature

```python
sp.build_pie_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_pct: bool = True,
    width: int = 700,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    hover_json: str | None = None,
    legend_position: str = "right",
) -> Chart
```

Aliases: `sp.pie`, `sp.pie_chart`

---

## Description

Un graphique en secteurs divise un cercle en tranches proportionnelles, où l'angle de chaque tranche est proportionnel à sa part du total. Il est plus efficace pour comparer un petit nombre de catégories (3 à 6) qui forment ensemble un tout significatif, comme des parts de marché ou une répartition budgétaire. SeraPlot normalise automatiquement les valeurs à 100 %, de sorte que des comptages bruts, des pourcentages ou toute donnée numérique peuvent être passés directement. Pour une esthétique avec trou central, utilisez le graphique en anneau.

**Idéal pour :**
- Afficher la composition parties-tout pour 3 à 6 catégories
- Visualiser des parts de marché, des répartitions budgétaires ou des résultats de sondage
- Présenter des données à un public non technique habitué à cette forme de graphique

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | — | Titre du graphique affiché en haut |
| `labels` | `list[str]` | — | Nom de catégorie pour chaque tranche |
| `values` | `list[float]` | — | Valeur numérique pour chaque tranche ; normalisée automatiquement à 100 % |
| `show_pct` | `bool` | `True` | Afficher les pourcentages à l'intérieur ou à côté de chaque tranche |
| `width` | `int` | `700` | Largeur du canevas en pixels |
| `height` | `int` | `480` | Hauteur du canevas en pixels |
| `palette` | `list[int] \| None` | `None` | Couleurs de tranche personnalisées en entiers hexadécimaux |
| `background` | `str \| None` | `None` | Couleur de fond CSS |
| `hover_json` | `str \| None` | `None` | JSON pour les données d'info-bulle personnalisées |
| `legend_position` | `str` | `"right"` | Position de la légende : `"top"`, `"right"`, `"bottom"` ou `"left"` |

---

## Retourne

`Chart`

---

<div class="sp-tabs" id="pie-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('pie-fr','pie-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('pie-fr','pie-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('pie-fr','pie-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('pie-fr','pie-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('pie-fr','pie-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('pie-fr','pie-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('pie-fr','pie-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('pie-fr','pie-fr-cpp',this)">C++</button>
</div>
<div id="pie-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.pie(
    title="Parts de marché T4",
    labels=["SeraPlot", "Matplotlib", "Plotly", "Bokeh", "Autres"],
    values=[35, 25, 20, 10, 10],
)
chart.show()</code></pre></div>
<div id="pie-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.pie({
  title: "Parts de marché T4",
  labels: ["SeraPlot", "Matplotlib", "Plotly", "Bokeh", "Autres"],
  values: [35, 25, 20, 10, 10],
});
chart.show();</code></pre></div>
<div id="pie-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.pie({
  title: "Parts de marché T4",
  labels: ["SeraPlot", "Matplotlib", "Plotly", "Bokeh", "Autres"],
  values: [35, 25, 20, 10, 10],
});
chart.show();</code></pre></div>
<div id="pie-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$pie(
  title = "Parts de marché T4",
  labels = c("SeraPlot", "Matplotlib", "Plotly", "Bokeh", "Autres"),
  values = c(35, 25, 20, 10, 10)
)
chart$show()</code></pre></div>
<div id="pie-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.pie()
    .title("Parts de marché T4")
    .labels(List.of("SeraPlot", "Matplotlib", "Plotly", "Bokeh", "Autres"))
    .values(List.of(35.0, 25.0, 20.0, 10.0, 10.0))
    .build();
chart.show();</code></pre></div>
<div id="pie-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Pie(
    title: "Parts de marché T4",
    labels: new[]{"SeraPlot", "Matplotlib", "Plotly", "Bokeh", "Autres"},
    values: new[]{35.0, 25.0, 20.0, 10.0, 10.0}
);
chart.Show();</code></pre></div>
<div id="pie-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.pie(
  title = "Parts de marché T4",
  labels = List("SeraPlot", "Matplotlib", "Plotly", "Bokeh", "Autres"),
  values = List(35.0, 25.0, 20.0, 10.0, 10.0)
)
chart.show()</code></pre></div>
<div id="pie-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::pie({
  .title  = "Parts de marché T4",
  .labels = {"SeraPlot", "Matplotlib", "Plotly", "Bokeh", "Autres"},
  .values = {35.0, 25.0, 20.0, 10.0, 10.0}
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/pie.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## Voir aussi

- [donut.md](donut.md) — Graphique en secteurs avec un trou central
- [sunburst.md](sunburst.md) — Secteurs hiérarchiques avec anneaux imbriqués
- [treemap.md](treemap.md) — Rectangles imbriqués pour les hiérarchies parties-tout

</div>
