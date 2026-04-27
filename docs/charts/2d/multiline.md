# Multi-Line Chart

<div class="lang-en">

## Signature

```python
sp.build_multiline_chart(
    title: str,
    x_labels: list[str],
    series_values: list[list[float]],
    *,
    show_points: bool = True,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    legend_position: str = "top",
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.multiline`

---

## Description

A multi-line chart overlays multiple line series on the same axes, enabling direct comparison of how several metrics evolve in parallel over the same time dimension. Each inner list in `series_values` must have the same length as `x_labels`. Series are distinguished by color from the default palette (or a custom one) and labeled in the legend. Unlike `build_line_chart`, this function is optimized for 2–10 simultaneous series; beyond that, consider a ridgeline or heatmap.

**Ideal for:**
- Comparing multiple KPIs or products over the same period
- Visualizing A/B test outcomes over time
- Overlaying actual vs forecast vs target lines

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | — | Chart title displayed at the top |
| `x_labels` | `list[str]` | — | Ordered X axis labels shared by all series |
| `series_values` | `list[list[float]]` | — | One inner list per series; all must match the length of `x_labels` |
| `show_points` | `bool` | `True` | Draw circle markers at each data point |
| `series_names` | `list[str] \| None` | `None` | Legend label for each series |
| `palette` | `list[int] \| None` | `None` | Custom series colors as hex integers |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `x_label` | `str` | `""` | Label for the X axis |
| `y_label` | `str` | `""` | Label for the Y axis |
| `gridlines` | `bool` | `True` | Draw horizontal gridlines |
| `legend_position` | `str` | `"top"` | Legend placement: `"top"`, `"right"`, `"bottom"`, or `"left"` |
| `hover_json` | `str \| None` | `None` | JSON string for custom tooltip data |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="multiline">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('multiline','multiline-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('multiline','multiline-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('multiline','multiline-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('multiline','multiline-r',this)">R</button>
<button class="sp-tb" onclick="spTab('multiline','multiline-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('multiline','multiline-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('multiline','multiline-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('multiline','multiline-cpp',this)">C++</button>
</div>
<div id="multiline-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.multiline(
    title="Monthly revenue by product line",
    x_labels=["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
    series_values=[
        [1200, 1400, 1600, 1500, 1800, 2100],
        [800,  850,  900,  1050, 1100, 1300],
        [500,  520,  580,  610,  650,  700],
    ],
    series_names=["Product A", "Product B", "Product C"],
)
chart.show()</code></pre></div>
<div id="multiline-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.multiline({
  title: "Monthly revenue by product line",
  xLabels: ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
  seriesValues: [
    [1200, 1400, 1600, 1500, 1800, 2100],
    [800,  850,  900,  1050, 1100, 1300],
    [500,  520,  580,  610,  650,  700],
  ],
  seriesNames: ["Product A", "Product B", "Product C"],
});
chart.show();</code></pre></div>
<div id="multiline-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.multiline({
  title: "Monthly revenue by product line",
  xLabels: ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
  seriesValues: [
    [1200, 1400, 1600, 1500, 1800, 2100],
    [800,  850,  900,  1050, 1100, 1300],
    [500,  520,  580,  610,  650,  700],
  ],
  seriesNames: ["Product A", "Product B", "Product C"],
});
chart.show();</code></pre></div>
<div id="multiline-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$multiline(
  title = "Monthly revenue by product line",
  x_labels = c("Jan", "Feb", "Mar", "Apr", "May", "Jun"),
  series_values = list(
    c(1200, 1400, 1600, 1500, 1800, 2100),
    c(800,  850,  900,  1050, 1100, 1300),
    c(500,  520,  580,  610,  650,  700)
  ),
  series_names = c("Product A", "Product B", "Product C")
)
chart$show()</code></pre></div>
<div id="multiline-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.multiline()
    .title("Monthly revenue by product line")
    .xLabels(List.of("Jan", "Feb", "Mar", "Apr", "May", "Jun"))
    .seriesValues(List.of(
        List.of(1200.0, 1400.0, 1600.0, 1500.0, 1800.0, 2100.0),
        List.of(800.0,  850.0,  900.0,  1050.0, 1100.0, 1300.0),
        List.of(500.0,  520.0,  580.0,  610.0,  650.0,  700.0)
    ))
    .seriesNames(List.of("Product A", "Product B", "Product C"))
    .build();
chart.show();</code></pre></div>
<div id="multiline-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Multiline(
    title: "Monthly revenue by product line",
    xLabels: new[]{"Jan", "Feb", "Mar", "Apr", "May", "Jun"},
    seriesValues: new[]{
        new[]{1200.0, 1400.0, 1600.0, 1500.0, 1800.0, 2100.0},
        new[]{800.0,  850.0,  900.0,  1050.0, 1100.0, 1300.0},
        new[]{500.0,  520.0,  580.0,  610.0,  650.0,  700.0},
    },
    seriesNames: new[]{"Product A", "Product B", "Product C"}
);
chart.Show();</code></pre></div>
<div id="multiline-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.multiline(
  title = "Monthly revenue by product line",
  x_labels = List("Jan", "Feb", "Mar", "Apr", "May", "Jun"),
  series_values = List(
    List(1200.0, 1400.0, 1600.0, 1500.0, 1800.0, 2100.0),
    List(800.0,  850.0,  900.0,  1050.0, 1100.0, 1300.0),
    List(500.0,  520.0,  580.0,  610.0,  650.0,  700.0)
  ),
  series_names = List("Product A", "Product B", "Product C")
)
chart.show()</code></pre></div>
<div id="multiline-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::multiline({
  .title         = "Monthly revenue by product line",
  .x_labels      = {"Jan", "Feb", "Mar", "Apr", "May", "Jun"},
  .series_values = {
    {1200, 1400, 1600, 1500, 1800, 2100},
    {800,  850,  900,  1050, 1100, 1300},
    {500,  520,  580,  610,  650,  700}
  },
  .series_names  = {"Product A", "Product B", "Product C"}
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/multiline.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [line.md](line.md) — Single series line chart
- [area.md](area.md) — Multi-series with filled areas

</div>

<div class="lang-fr">

## Signature

```python
sp.build_multiline_chart(
    title: str,
    x_labels: list[str],
    series_values: list[list[float]],
    *,
    show_points: bool = True,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    legend_position: str = "top",
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.multiline`

---

## Description

Un graphique multi-courbes superpose plusieurs séries de lignes sur les mêmes axes, permettant la comparaison directe de l'évolution de plusieurs métriques en parallèle sur la même dimension temporelle. Chaque liste interne de `series_values` doit avoir la même longueur que `x_labels`. Les séries sont distinguées par les couleurs de la palette par défaut (ou d'une palette personnalisée) et étiquetées dans la légende. Contrairement à `build_line_chart`, cette fonction est optimisée pour 2 à 10 séries simultanées ; au-delà, envisagez un ridgeline ou une carte thermique.

**Idéal pour :**
- Comparer plusieurs indicateurs ou produits sur la même période
- Visualiser les résultats de tests A/B dans le temps
- Superposer les lignes réel, prévision et objectif

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | — | Titre du graphique affiché en haut |
| `x_labels` | `list[str]` | — | Labels de l'axe X partagés par toutes les séries |
| `series_values` | `list[list[float]]` | — | Une liste interne par série ; toutes doivent correspondre à la longueur de `x_labels` |
| `show_points` | `bool` | `True` | Afficher des marqueurs circulaires à chaque point de données |
| `series_names` | `list[str] \| None` | `None` | Label de légende pour chaque série |
| `palette` | `list[int] \| None` | `None` | Couleurs de série personnalisées en entiers hexadécimaux |
| `width` | `int` | `900` | Largeur du canevas en pixels |
| `height` | `int` | `480` | Hauteur du canevas en pixels |
| `x_label` | `str` | `""` | Label de l'axe X |
| `y_label` | `str` | `""` | Label de l'axe Y |
| `gridlines` | `bool` | `True` | Afficher des lignes de grille horizontales |
| `legend_position` | `str` | `"top"` | Position de la légende : `"top"`, `"right"`, `"bottom"` ou `"left"` |
| `hover_json` | `str \| None` | `None` | JSON pour les données d'info-bulle personnalisées |

---

## Retourne

`Chart`

---

<div class="sp-tabs" id="multiline-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('multiline-fr','multiline-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('multiline-fr','multiline-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('multiline-fr','multiline-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('multiline-fr','multiline-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('multiline-fr','multiline-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('multiline-fr','multiline-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('multiline-fr','multiline-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('multiline-fr','multiline-fr-cpp',this)">C++</button>
</div>
<div id="multiline-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.multiline(
    title="Chiffre d'affaires mensuel par ligne de produit",
    x_labels=["Jan", "Fév", "Mar", "Avr", "Mai", "Juin"],
    series_values=[
        [1200, 1400, 1600, 1500, 1800, 2100],
        [800,  850,  900,  1050, 1100, 1300],
        [500,  520,  580,  610,  650,  700],
    ],
    series_names=["Produit A", "Produit B", "Produit C"],
)
chart.show()</code></pre></div>
<div id="multiline-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.multiline({
  title: "Chiffre d'affaires mensuel par ligne de produit",
  xLabels: ["Jan", "Fév", "Mar", "Avr", "Mai", "Juin"],
  seriesValues: [
    [1200, 1400, 1600, 1500, 1800, 2100],
    [800,  850,  900,  1050, 1100, 1300],
    [500,  520,  580,  610,  650,  700],
  ],
  seriesNames: ["Produit A", "Produit B", "Produit C"],
});
chart.show();</code></pre></div>
<div id="multiline-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.multiline({
  title: "Chiffre d'affaires mensuel par ligne de produit",
  xLabels: ["Jan", "Fév", "Mar", "Avr", "Mai", "Juin"],
  seriesValues: [
    [1200, 1400, 1600, 1500, 1800, 2100],
    [800,  850,  900,  1050, 1100, 1300],
    [500,  520,  580,  610,  650,  700],
  ],
  seriesNames: ["Produit A", "Produit B", "Produit C"],
});
chart.show();</code></pre></div>
<div id="multiline-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$multiline(
  title = "Chiffre d'affaires mensuel par ligne de produit",
  x_labels = c("Jan", "Fév", "Mar", "Avr", "Mai", "Juin"),
  series_values = list(
    c(1200, 1400, 1600, 1500, 1800, 2100),
    c(800,  850,  900,  1050, 1100, 1300),
    c(500,  520,  580,  610,  650,  700)
  ),
  series_names = c("Produit A", "Produit B", "Produit C")
)
chart$show()</code></pre></div>
<div id="multiline-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.multiline()
    .title("Chiffre d'affaires mensuel par ligne de produit")
    .xLabels(List.of("Jan", "Fév", "Mar", "Avr", "Mai", "Juin"))
    .seriesValues(List.of(
        List.of(1200.0, 1400.0, 1600.0, 1500.0, 1800.0, 2100.0),
        List.of(800.0,  850.0,  900.0,  1050.0, 1100.0, 1300.0),
        List.of(500.0,  520.0,  580.0,  610.0,  650.0,  700.0)
    ))
    .seriesNames(List.of("Produit A", "Produit B", "Produit C"))
    .build();
chart.show();</code></pre></div>
<div id="multiline-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Multiline(
    title: "Chiffre d'affaires mensuel par ligne de produit",
    xLabels: new[]{"Jan", "Fév", "Mar", "Avr", "Mai", "Juin"},
    seriesValues: new[]{
        new[]{1200.0, 1400.0, 1600.0, 1500.0, 1800.0, 2100.0},
        new[]{800.0,  850.0,  900.0,  1050.0, 1100.0, 1300.0},
        new[]{500.0,  520.0,  580.0,  610.0,  650.0,  700.0},
    },
    seriesNames: new[]{"Produit A", "Produit B", "Produit C"}
);
chart.Show();</code></pre></div>
<div id="multiline-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.multiline(
  title = "Chiffre d'affaires mensuel par ligne de produit",
  x_labels = List("Jan", "Fév", "Mar", "Avr", "Mai", "Juin"),
  series_values = List(
    List(1200.0, 1400.0, 1600.0, 1500.0, 1800.0, 2100.0),
    List(800.0,  850.0,  900.0,  1050.0, 1100.0, 1300.0),
    List(500.0,  520.0,  580.0,  610.0,  650.0,  700.0)
  ),
  series_names = List("Produit A", "Produit B", "Produit C")
)
chart.show()</code></pre></div>
<div id="multiline-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::multiline({
  .title         = "Chiffre d'affaires mensuel par ligne de produit",
  .x_labels      = {"Jan", "Fév", "Mar", "Avr", "Mai", "Juin"},
  .series_values = {
    {1200, 1400, 1600, 1500, 1800, 2100},
    {800,  850,  900,  1050, 1100, 1300},
    {500,  520,  580,  610,  650,  700}
  },
  .series_names  = {"Produit A", "Produit B", "Produit C"}
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/multiline.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## Voir aussi

- [line.md](line.md) — Graphique en ligne à série unique
- [area.md](area.md) — Multi-séries avec zones remplies

</div>
