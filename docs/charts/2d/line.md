# Line Chart

<div class="lang-en">

## Signature

```python
sp.build_line_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    show_points: bool = True,
    gridlines: bool = False,
    sort_order: str = "none",
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.line`, `sp.line_chart`

---

## Description

A line chart connects ordered data points with continuous line segments, making trends and time-series patterns immediately visible. It excels at showing how a single metric evolves over time — from stock prices to temperature readings to monthly sales. SeraPlot renders the line with anti-aliased SVG paths and optional point markers, with hover tooltips showing exact values at each position. For multiple series on the same axes, use `build_multiline_chart` instead.

**Ideal for:**
- Time series and trend analysis
- Monitoring a single KPI over time
- Detecting acceleration or deceleration in a metric

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | — | Chart title displayed at the top |
| `labels` | `list[str]` | — | Ordered category or date labels for the X axis |
| `values` | `list[float]` | — | Numeric Y values corresponding to each label |
| `color_hex` | `int` | `0x6366F1` | Line and point color as a hex integer |
| `show_points` | `bool` | `True` | Whether to draw a circle marker at each data point |
| `gridlines` | `bool` | `False` | Whether to draw horizontal gridlines |
| `sort_order` | `str` | `"none"` | Sort labels before rendering: `"asc"`, `"desc"`, or `"none"` |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `x_label` | `str` | `""` | Label for the X axis |
| `y_label` | `str` | `""` | Label for the Y axis |
| `no_y_axis` | `bool` | `False` | Hide the Y axis and its tick marks |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="line">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('line','line-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('line','line-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('line','line-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('line','line-r',this)">R</button>
<button class="sp-tb" onclick="spTab('line','line-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('line','line-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('line','line-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('line','line-cpp',this)">C++</button>
</div>
<div id="line-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.line(
    title="Temperature over 12 months",
    labels=["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"],
    values=[3.2, 4.1, 8.5, 13.2, 17.8, 22.1, 24.6, 24.0, 19.3, 13.7, 7.8, 4.2],
    gridlines=True,
    y_label="°C",
)
chart.show()</code></pre></div>
<div id="line-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.line({
  title: "Temperature over 12 months",
  labels: ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"],
  values: [3.2, 4.1, 8.5, 13.2, 17.8, 22.1, 24.6, 24.0, 19.3, 13.7, 7.8, 4.2],
  gridlines: true,
  yLabel: "°C",
});
chart.show();</code></pre></div>
<div id="line-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.line({
  title: "Temperature over 12 months",
  labels: ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"],
  values: [3.2, 4.1, 8.5, 13.2, 17.8, 22.1, 24.6, 24.0, 19.3, 13.7, 7.8, 4.2],
  gridlines: true,
  yLabel: "°C",
});
chart.show();</code></pre></div>
<div id="line-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$line(
  title = "Temperature over 12 months",
  labels = c("Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"),
  values = c(3.2, 4.1, 8.5, 13.2, 17.8, 22.1, 24.6, 24.0, 19.3, 13.7, 7.8, 4.2),
  gridlines = TRUE,
  y_label = "°C"
)
chart$show()</code></pre></div>
<div id="line-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.line()
    .title("Temperature over 12 months")
    .labels(List.of("Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"))
    .values(List.of(3.2, 4.1, 8.5, 13.2, 17.8, 22.1, 24.6, 24.0, 19.3, 13.7, 7.8, 4.2))
    .gridlines(true)
    .yLabel("°C")
    .build();
chart.show();</code></pre></div>
<div id="line-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Line(
    title: "Temperature over 12 months",
    labels: new[]{"Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"},
    values: new[]{3.2, 4.1, 8.5, 13.2, 17.8, 22.1, 24.6, 24.0, 19.3, 13.7, 7.8, 4.2},
    gridlines: true,
    yLabel: "°C"
);
chart.Show();</code></pre></div>
<div id="line-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.line(
  title = "Temperature over 12 months",
  labels = List("Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"),
  values = List(3.2, 4.1, 8.5, 13.2, 17.8, 22.1, 24.6, 24.0, 19.3, 13.7, 7.8, 4.2),
  gridlines = true,
  y_label = "°C"
)
chart.show()</code></pre></div>
<div id="line-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::line({
  .title     = "Temperature over 12 months",
  .labels    = {"Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"},
  .values    = {3.2, 4.1, 8.5, 13.2, 17.8, 22.1, 24.6, 24.0, 19.3, 13.7, 7.8, 4.2},
  .gridlines = true,
  .y_label   = "°C"
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/line.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [multiline.md](multiline.md) — Multiple series on the same axes
- [area.md](area.md) — Filled area under a line
- [scatter.md](scatter.md) — Points without connecting lines

</div>

<div class="lang-fr">

## Signature

```python
sp.build_line_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    show_points: bool = True,
    gridlines: bool = False,
    sort_order: str = "none",
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.line`, `sp.line_chart`

---

## Description

Un graphique en ligne relie des points de données ordonnés par des segments continus, rendant les tendances et les séries temporelles immédiatement visibles. Il excelle pour montrer l'évolution d'une seule métrique dans le temps — prix d'actions, relevés de température ou ventes mensuelles. SeraPlot trace la ligne avec des tracés SVG anti-crénelés et des marqueurs de points optionnels, avec des info-bulles affichant les valeurs exactes à chaque position. Pour plusieurs séries sur les mêmes axes, utilisez `build_multiline_chart`.

**Idéal pour :**
- L'analyse de séries temporelles et de tendances
- La surveillance d'un indicateur clé dans le temps
- La détection d'accélération ou de décélération d'une métrique

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | — | Titre du graphique affiché en haut |
| `labels` | `list[str]` | — | Labels de catégorie ou de date ordonnés pour l'axe X |
| `values` | `list[float]` | — | Valeurs Y numériques correspondant à chaque label |
| `color_hex` | `int` | `0x6366F1` | Couleur de la ligne et des points en entier hexadécimal |
| `show_points` | `bool` | `True` | Afficher un cercle marqueur à chaque point de données |
| `gridlines` | `bool` | `False` | Afficher des lignes de grille horizontales |
| `sort_order` | `str` | `"none"` | Trier les labels avant le rendu : `"asc"`, `"desc"` ou `"none"` |
| `width` | `int` | `900` | Largeur du canevas en pixels |
| `height` | `int` | `480` | Hauteur du canevas en pixels |
| `x_label` | `str` | `""` | Label de l'axe X |
| `y_label` | `str` | `""` | Label de l'axe Y |
| `no_y_axis` | `bool` | `False` | Masquer l'axe Y et ses graduations |

---

## Retourne

`Chart`

---

<div class="sp-tabs" id="line-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('line-fr','line-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('line-fr','line-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('line-fr','line-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('line-fr','line-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('line-fr','line-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('line-fr','line-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('line-fr','line-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('line-fr','line-fr-cpp',this)">C++</button>
</div>
<div id="line-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.line(
    title="Température sur 12 mois",
    labels=["Jan","Fév","Mar","Avr","Mai","Juin","Juil","Août","Sep","Oct","Nov","Déc"],
    values=[3.2, 4.1, 8.5, 13.2, 17.8, 22.1, 24.6, 24.0, 19.3, 13.7, 7.8, 4.2],
    gridlines=True,
    y_label="°C",
)
chart.show()</code></pre></div>
<div id="line-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.line({
  title: "Température sur 12 mois",
  labels: ["Jan","Fév","Mar","Avr","Mai","Juin","Juil","Août","Sep","Oct","Nov","Déc"],
  values: [3.2, 4.1, 8.5, 13.2, 17.8, 22.1, 24.6, 24.0, 19.3, 13.7, 7.8, 4.2],
  gridlines: true,
  yLabel: "°C",
});
chart.show();</code></pre></div>
<div id="line-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.line({
  title: "Température sur 12 mois",
  labels: ["Jan","Fév","Mar","Avr","Mai","Juin","Juil","Août","Sep","Oct","Nov","Déc"],
  values: [3.2, 4.1, 8.5, 13.2, 17.8, 22.1, 24.6, 24.0, 19.3, 13.7, 7.8, 4.2],
  gridlines: true,
  yLabel: "°C",
});
chart.show();</code></pre></div>
<div id="line-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$line(
  title = "Température sur 12 mois",
  labels = c("Jan","Fév","Mar","Avr","Mai","Juin","Juil","Août","Sep","Oct","Nov","Déc"),
  values = c(3.2, 4.1, 8.5, 13.2, 17.8, 22.1, 24.6, 24.0, 19.3, 13.7, 7.8, 4.2),
  gridlines = TRUE,
  y_label = "°C"
)
chart$show()</code></pre></div>
<div id="line-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.line()
    .title("Température sur 12 mois")
    .labels(List.of("Jan","Fév","Mar","Avr","Mai","Juin","Juil","Août","Sep","Oct","Nov","Déc"))
    .values(List.of(3.2, 4.1, 8.5, 13.2, 17.8, 22.1, 24.6, 24.0, 19.3, 13.7, 7.8, 4.2))
    .gridlines(true)
    .yLabel("°C")
    .build();
chart.show();</code></pre></div>
<div id="line-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Line(
    title: "Température sur 12 mois",
    labels: new[]{"Jan","Fév","Mar","Avr","Mai","Juin","Juil","Août","Sep","Oct","Nov","Déc"},
    values: new[]{3.2, 4.1, 8.5, 13.2, 17.8, 22.1, 24.6, 24.0, 19.3, 13.7, 7.8, 4.2},
    gridlines: true,
    yLabel: "°C"
);
chart.Show();</code></pre></div>
<div id="line-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.line(
  title = "Température sur 12 mois",
  labels = List("Jan","Fév","Mar","Avr","Mai","Juin","Juil","Août","Sep","Oct","Nov","Déc"),
  values = List(3.2, 4.1, 8.5, 13.2, 17.8, 22.1, 24.6, 24.0, 19.3, 13.7, 7.8, 4.2),
  gridlines = true,
  y_label = "°C"
)
chart.show()</code></pre></div>
<div id="line-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::line({
  .title     = "Température sur 12 mois",
  .labels    = {"Jan","Fév","Mar","Avr","Mai","Juin","Juil","Août","Sep","Oct","Nov","Déc"},
  .values    = {3.2, 4.1, 8.5, 13.2, 17.8, 22.1, 24.6, 24.0, 19.3, 13.7, 7.8, 4.2},
  .gridlines = true,
  .y_label   = "°C"
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/line.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## Voir aussi

- [multiline.md](multiline.md) — Plusieurs séries sur les mêmes axes
- [area.md](area.md) — Zone remplie sous une ligne
- [scatter.md](scatter.md) — Points sans lignes de connexion

</div>
