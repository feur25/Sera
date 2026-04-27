# Histogram Overlay

<div class="lang-en">

## Signature

```python
sp.build_histogram_overlay(
    title: str,
    values: list[float],
    overlay_values: list[float],
    *,
    bins: int = 20,
    series_names: list[str] | None = None,
    color_hex: int = 0,
    overlay_color_hex: int = 0,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.histogram_overlay`

---

## Description

A histogram overlay renders two distributions as semi-transparent histograms on the same axes, enabling direct visual comparison of their shapes, centers, and spreads. Both histograms share the same bin boundaries computed from the merged dataset, ensuring fair comparison. Colors are independently configurable; by default the engine picks two distinct palette colors. This chart is the primary tool for A/B test distribution comparison and before/after analysis at scale.

**Ideal for:**
- A/B test outcome distributions (control vs variant)
- Before/after intervention comparisons on continuous metrics
- Validating that two samples are drawn from the same underlying distribution

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `values` | `list[float]` | required | First distribution values |
| `overlay_values` | `list[float]` | required | Second distribution values |
| `bins` | `int` | `20` | Number of bins (shared between both distributions) |
| `series_names` | `list[str] \| None` | `None` | Names for the two series; defaults to `["A", "B"]` |
| `color_hex` | `int` | `0` | Color of first histogram; `0` = auto |
| `overlay_color_hex` | `int` | `0` | Color of overlay histogram; `0` = auto |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `no_y_axis` | `bool` | `False` | Hide Y axis |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="hist-overlay">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hist-overlay','hist-overlay-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hist-overlay','hist-overlay-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hist-overlay','hist-overlay-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hist-overlay','hist-overlay-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hist-overlay','hist-overlay-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hist-overlay','hist-overlay-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hist-overlay','hist-overlay-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hist-overlay','hist-overlay-cpp',this)">C++</button>
</div>
<div id="hist-overlay-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(42)
control = [random.gauss(62, 8) for _ in range(500)]
variant = [random.gauss(67, 9) for _ in range(500)]
chart = sp.histogram_overlay(
    title="A/B Test — Time on Page (seconds)",
    values=control,
    overlay_values=variant,
    bins=30,
    series_names=["Control", "Variant B"],
    x_label="Seconds",
)
chart.show()</code></pre></div>
<div id="hist-overlay-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.histogramOverlay({
  title: "A/B Test — Time on Page (seconds)",
  values: control,
  overlayValues: variant,
  bins: 30,
  seriesNames: ["Control", "Variant B"],
  xLabel: "Seconds",
});
chart.show();</code></pre></div>
<div id="hist-overlay-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.histogramOverlay({
  title: "A/B Test — Time on Page (seconds)",
  values: control,
  overlayValues: variant,
  bins: 30,
  seriesNames: ["Control", "Variant B"],
  xLabel: "Seconds",
});
chart.show();</code></pre></div>
<div id="hist-overlay-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
set.seed(42)
controle <- rnorm(500, mean = 62, sd = 8)
variante <- rnorm(500, mean = 67, sd = 9)
chart <- sp$histogram_overlay(
  title          = "Test A/B — Temps sur la page (secondes)",
  values         = controle,
  overlay_values = variante,
  bins           = 30,
  series_names   = c("Control", "Variant B"),
  x_label        = "Seconds"
)
chart$show()</code></pre></div>
<div id="hist-overlay-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
var chart = SeraPlot.histogramOverlay()
    .title("A/B Test — Time on Page (seconds)")
    .values(control)
    .overlayValues(variant)
    .bins(30)
    .seriesNames(List.of("Control", "Variant B"))
    .xLabel("Seconds")
    .build();
chart.show();</code></pre></div>
<div id="hist-overlay-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;
var chart = Sp.HistogramOverlay(
    title:         "A/B Test — Time on Page (seconds)",
    values:        control,
    overlayValues: variant,
    bins:          30,
    seriesNames:   ["Control", "Variant B"],
    xLabel:        "Seconds"
);
chart.Show();</code></pre></div>
<div id="hist-overlay-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp
val chart = sp.histogram_overlay(
  title          = "A/B Test — Time on Page (seconds)",
  values         = control,
  overlay_values = variant,
  bins           = 30,
  series_names   = List("Control", "Variant B"),
  x_label        = "Seconds"
)
chart.show()</code></pre></div>
<div id="hist-overlay-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;
auto chart = sp::histogram_overlay({
    .title          = "A/B Test — Time on Page (seconds)",
    .values         = control,
    .overlay_values = variant,
    .bins           = 30,
    .series_names   = {"Control", "Variant B"},
    .x_label        = "Seconds",
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/histogram-overlay.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [Histogram](histogram.md) — `sp.build_histogram()`
- [KDE Chart](kde.md) — `sp.build_kde_chart()`
- [Violin Chart](violin.md) — `sp.build_violin()`

</div>

<div class="lang-fr">

## Signature

```python
sp.build_histogram_overlay(
    title: str,
    values: list[float],
    overlay_values: list[float],
    *,
    bins: int = 20,
    series_names: list[str] | None = None,
    color_hex: int = 0,
    overlay_color_hex: int = 0,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.histogram_overlay`

---

## Description

Un histogramme superposé affiche deux distributions sous forme d'histogrammes semi-transparents sur les mêmes axes, permettant une comparaison visuelle directe de leurs formes, centres et dispersions. Les deux histogrammes partagent les mêmes bornes de classes calculées à partir du jeu de données fusionné, garantissant une comparaison équitable. Les couleurs sont configurables indépendamment ; par défaut, le moteur choisit deux couleurs de palette distinctes. Ce graphique est l'outil principal de comparaison de distributions pour les tests A/B.

**Idéal pour :**
- Distributions des résultats de tests A/B (contrôle vs variante)
- Comparaisons avant/après une intervention sur des métriques continues
- Vérifier que deux échantillons proviennent de la même distribution sous-jacente

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `values` | `list[float]` | requis | Valeurs de la première distribution |
| `overlay_values` | `list[float]` | requis | Valeurs de la deuxième distribution |
| `bins` | `int` | `20` | Nombre de classes (partagé entre les deux distributions) |
| `series_names` | `list[str] \| None` | `None` | Noms des deux séries ; par défaut `["A", "B"]` |
| `color_hex` | `int` | `0` | Couleur du premier histogramme ; `0` = auto |
| `overlay_color_hex` | `int` | `0` | Couleur de l'histogramme superposé ; `0` = auto |
| `width` | `int` | `900` | Largeur du canvas en pixels |
| `height` | `int` | `480` | Hauteur du canvas en pixels |
| `x_label` | `str` | `""` | Étiquette de l'axe X |
| `y_label` | `str` | `""` | Étiquette de l'axe Y |
| `no_y_axis` | `bool` | `False` | Masquer l'axe Y |

---

## Retourne

`Chart`

---

<div class="sp-tabs" id="hist-overlay-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hist-overlay-fr','hist-overlay-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hist-overlay-fr','hist-overlay-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hist-overlay-fr','hist-overlay-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hist-overlay-fr','hist-overlay-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hist-overlay-fr','hist-overlay-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hist-overlay-fr','hist-overlay-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hist-overlay-fr','hist-overlay-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hist-overlay-fr','hist-overlay-fr-cpp',this)">C++</button>
</div>
<div id="hist-overlay-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(42)
controle = [random.gauss(62, 8) for _ in range(500)]
variante = [random.gauss(67, 9) for _ in range(500)]
chart = sp.histogram_overlay(
    title="Test A/B — Durée sur la page (secondes)",
    values=controle,
    overlay_values=variante,
    bins=30,
    series_names=["Contrôle", "Variante B"],
    x_label="Secondes",
)
chart.show()</code></pre></div>
<div id="hist-overlay-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.histogramOverlay({
  title: "Test A/B — Durée sur la page (secondes)",
  values: controle,
  overlayValues: variante,
  bins: 30,
  seriesNames: ["Contrôle", "Variante B"],
  xLabel: "Secondes",
});
chart.show();</code></pre></div>
<div id="hist-overlay-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.histogramOverlay({
  title: "Test A/B — Durée sur la page (secondes)",
  values: controle,
  overlayValues: variante,
  bins: 30,
  seriesNames: ["Contrôle", "Variante B"],
  xLabel: "Secondes",
});
chart.show();</code></pre></div>
<div id="hist-overlay-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
set.seed(42)
controle <- rnorm(500, mean = 62, sd = 8)
variante <- rnorm(500, mean = 67, sd = 9)
chart <- sp$histogram_overlay(
  title          = "Test A/B — Durée sur la page (secondes)",
  values         = controle,
  overlay_values = variante,
  bins           = 30,
  series_names   = c("Contrôle", "Variante B"),
  x_label        = "Secondes"
)
chart$show()</code></pre></div>
<div id="hist-overlay-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
var chart = SeraPlot.histogramOverlay()
    .title("Test A/B — Durée sur la page (secondes)")
    .values(controle)
    .overlayValues(variante)
    .bins(30)
    .seriesNames(List.of("Contrôle", "Variante B"))
    .xLabel("Secondes")
    .build();
chart.show();</code></pre></div>
<div id="hist-overlay-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;
var chart = Sp.HistogramOverlay(
    title:         "Test A/B — Durée sur la page (secondes)",
    values:        controle,
    overlayValues: variante,
    bins:          30,
    seriesNames:   ["Contrôle", "Variante B"],
    xLabel:        "Secondes"
);
chart.Show();</code></pre></div>
<div id="hist-overlay-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp
val chart = sp.histogram_overlay(
  title          = "Test A/B — Durée sur la page (secondes)",
  values         = controle,
  overlay_values = variante,
  bins           = 30,
  series_names   = List("Contrôle", "Variante B"),
  x_label        = "Secondes"
)
chart.show()</code></pre></div>
<div id="hist-overlay-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;
auto chart = sp::histogram_overlay({
    .title          = "Test A/B — Durée sur la page (secondes)",
    .values         = controle,
    .overlay_values = variante,
    .bins           = 30,
    .series_names   = {"Contrôle", "Variante B"},
    .x_label        = "Secondes",
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/histogram-overlay.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## Voir aussi

- [Histogramme](histogram.md) — `sp.build_histogram()`
- [Courbe de densité (KDE)](kde.md) — `sp.build_kde_chart()`
- [Graphique en violon](violin.md) — `sp.build_violin()`

</div>
