# Histogram

<div class="lang-en">

## Signature

```python
sp.build_histogram(
    title: str,
    values: list[float],
    *,
    bins: int = 20,
    show_counts: bool = False,
    color_hex: int = 0,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.hist`, `sp.histogram`

---

## Description

A histogram groups a continuous numeric variable into equal-width bins and draws a bar whose height represents the count (or frequency) of observations falling in that interval. It is the primary tool for understanding the shape of a distribution — symmetry, skewness, modality, and outliers. SeraPlot computes bin boundaries entirely in Rust for maximum speed, making it practical even for datasets with millions of rows. The `bins` parameter controls resolution; start with 20 and increase for larger datasets.

**Ideal for:**
- Visualizing the distribution shape of a numeric variable
- Identifying skewness, multimodality, and outliers in data
- Comparing a distribution to an expected shape before modelling

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | — | Chart title displayed at the top |
| `values` | `list[float]` | — | The raw numeric observations to bin |
| `bins` | `int` | `20` | Number of equal-width bins to create |
| `show_counts` | `bool` | `False` | Display the count value on top of each bar |
| `color_hex` | `int` | `0` | Bar fill color as hex integer; `0` uses the theme default |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `x_label` | `str` | `""` | Label for the X axis |
| `y_label` | `str` | `""` | Label for the Y axis |
| `no_y_axis` | `bool` | `False` | Hide the Y axis and its tick marks |

---

## Returns

`Chart`

---

## Examples

### Distribution of exam scores

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="histogram">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('histogram','histogram-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('histogram','histogram-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('histogram','histogram-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('histogram','histogram-r',this)">R</button>
<button class="sp-tb" onclick="spTab('histogram','histogram-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('histogram','histogram-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('histogram','histogram-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('histogram','histogram-cpp',this)">C++</button>
</div>
<div id="histogram-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random

random.seed(42)
values = [random.gauss(70, 10) for _ in range(500)]

chart = sp.hist(
    title="Distribution of exam scores",
    values=values,
    bins=25,
    x_label="Score",
    show_counts=True,
)
chart.show()</code></pre></div>
<div id="histogram-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

// Box-Muller transform to generate normally distributed values
function randn(mean, std) {
  const u1 = Math.random(), u2 = Math.random();
  return mean + std * Math.sqrt(-2 * Math.log(u1)) * Math.cos(2 * Math.PI * u2);
}
const values = Array.from({ length: 500 }, () => randn(70, 10));

const chart = sp.hist({
  title: "Distribution of exam scores",
  values: values,
  bins: 25,
  xLabel: "Score",
  showCounts: true,
});
chart.show();</code></pre></div>
<div id="histogram-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

function randn(mean: number, std: number): number {
  const u1 = Math.random(), u2 = Math.random();
  return mean + std * Math.sqrt(-2 * Math.log(u1)) * Math.cos(2 * Math.PI * u2);
}
const values: number[] = Array.from({ length: 500 }, () => randn(70, 10));

const chart = sp.hist({
  title: "Distribution of exam scores",
  values: values,
  bins: 25,
  xLabel: "Score",
  showCounts: true,
});
chart.show();</code></pre></div>
<div id="histogram-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

set.seed(42)
values <- rnorm(500, mean = 70, sd = 10)

chart <- sp$hist(
  title = "Distribution of exam scores",
  values = values,
  bins = 25,
  x_label = "Score",
  show_counts = TRUE
)
chart$show()</code></pre></div>
<div id="histogram-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
import java.util.Random;
import java.util.stream.Collectors;
import java.util.stream.Stream;

var rng = new Random(42);
var values = Stream.generate(() -> 70.0 + 10.0 * rng.nextGaussian())
    .limit(500)
    .collect(Collectors.toList());

var chart = SeraPlot.hist()
    .title("Distribution of exam scores")
    .values(values)
    .bins(25)
    .xLabel("Score")
    .showCounts(true)
    .build();
chart.show();</code></pre></div>
<div id="histogram-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var rng = new Random(42);
var values = Enumerable.Range(0, 500).Select(_ => {
    double u1 = rng.NextDouble(), u2 = rng.NextDouble();
    return 70.0 + 10.0 * Math.Sqrt(-2.0 * Math.Log(u1)) * Math.Cos(2 * Math.PI * u2);
}).ToList();

var chart = Sp.Hist(
    title: "Distribution of exam scores",
    values: values,
    bins: 25,
    xLabel: "Score",
    showCounts: true
);
chart.Show();</code></pre></div>
<div id="histogram-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val rng = new scala.util.Random(42)
val values = List.fill(500)(70.0 + 10.0 * rng.nextGaussian())

val chart = sp.hist(
  title = "Distribution of exam scores",
  values = values,
  bins = 25,
  x_label = "Score",
  show_counts = true
)
chart.show()</code></pre></div>
<div id="histogram-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;
#include &lt;random&gt;
#include &lt;vector&gt;

std::mt19937 rng(42);
std::normal_distribution&lt;double&gt; dist(70.0, 10.0);
std::vector&lt;double&gt; values(500);
std::generate(values.begin(), values.end(), [&]{ return dist(rng); });

auto chart = sp::hist({
  .title       = "Distribution of exam scores",
  .values      = values,
  .bins        = 25,
  .x_label     = "Score",
  .show_counts = true
});
chart.show();</code></pre></div>
</div>

<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>
<iframe src="../../previews/histogram.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>
</details>

---

## See also

- [kde.md](kde.md) — Smooth kernel density estimate curve
- [violin.md](violin.md) — Per-group distribution including KDE
- [boxplot.md](boxplot.md) — Five-number summary per category

</div>

<div class="lang-fr">

## Signature

```python
sp.build_histogram(
    title: str,
    values: list[float],
    *,
    bins: int = 20,
    show_counts: bool = False,
    color_hex: int = 0,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.hist`, `sp.histogram`

---

## Description

Un histogramme regroupe une variable numérique continue en intervalles de largeur égale et trace une barre dont la hauteur représente le nombre d'observations tombant dans cet intervalle. C'est l'outil principal pour comprendre la forme d'une distribution — symétrie, asymétrie, modalité et valeurs aberrantes. SeraPlot calcule les limites des intervalles entièrement en Rust pour une vitesse maximale, ce qui le rend pratique même pour des jeux de données de plusieurs millions de lignes. Le paramètre `bins` contrôle la résolution ; commencez par 20 et augmentez pour les jeux de données plus grands.

**Idéal pour :**
- Visualiser la forme de la distribution d'une variable numérique
- Identifier l'asymétrie, la multimodalité et les valeurs aberrantes dans les données
- Comparer une distribution à une forme attendue avant la modélisation

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | — | Titre du graphique affiché en haut |
| `values` | `list[float]` | — | Les observations numériques brutes à discrétiser |
| `bins` | `int` | `20` | Nombre d'intervalles de largeur égale à créer |
| `show_counts` | `bool` | `False` | Afficher la valeur de comptage au-dessus de chaque barre |
| `color_hex` | `int` | `0` | Couleur de remplissage des barres en hexadécimal ; `0` utilise la valeur du thème |
| `width` | `int` | `900` | Largeur du canevas en pixels |
| `height` | `int` | `480` | Hauteur du canevas en pixels |
| `x_label` | `str` | `""` | Label de l'axe X |
| `y_label` | `str` | `""` | Label de l'axe Y |
| `no_y_axis` | `bool` | `False` | Masquer l'axe Y et ses graduations |

---

## Retourne

`Chart`

---

## Exemples

### Distribution des notes d'examen

<div class="sp-tabs" id="histogram-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('histogram-fr','histogram-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('histogram-fr','histogram-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('histogram-fr','histogram-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('histogram-fr','histogram-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('histogram-fr','histogram-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('histogram-fr','histogram-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('histogram-fr','histogram-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('histogram-fr','histogram-fr-cpp',this)">C++</button>
</div>
<div id="histogram-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random

random.seed(42)
valeurs = [random.gauss(70, 10) for _ in range(500)]

chart = sp.hist(
    title="Distribution des notes d'examen",
    values=valeurs,
    bins=25,
    x_label="Note",
    show_counts=True,
)
chart.show()</code></pre></div>
<div id="histogram-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

function randn(mean, std) {
  const u1 = Math.random(), u2 = Math.random();
  return mean + std * Math.sqrt(-2 * Math.log(u1)) * Math.cos(2 * Math.PI * u2);
}
const valeurs = Array.from({ length: 500 }, () => randn(70, 10));

const chart = sp.hist({
  title: "Distribution des notes d'examen",
  values: valeurs,
  bins: 25,
  xLabel: "Note",
  showCounts: true,
});
chart.show();</code></pre></div>
<div id="histogram-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

function randn(mean: number, std: number): number {
  const u1 = Math.random(), u2 = Math.random();
  return mean + std * Math.sqrt(-2 * Math.log(u1)) * Math.cos(2 * Math.PI * u2);
}
const valeurs: number[] = Array.from({ length: 500 }, () => randn(70, 10));

const chart = sp.hist({
  title: "Distribution des notes d'examen",
  values: valeurs,
  bins: 25,
  xLabel: "Note",
  showCounts: true,
});
chart.show();</code></pre></div>
<div id="histogram-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

set.seed(42)
valeurs <- rnorm(500, mean = 70, sd = 10)

chart <- sp$hist(
  title = "Distribution des notes d'examen",
  values = valeurs,
  bins = 25,
  x_label = "Note",
  show_counts = TRUE
)
chart$show()</code></pre></div>
<div id="histogram-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.Random;
import java.util.stream.Collectors;
import java.util.stream.Stream;

var rng = new Random(42);
var valeurs = Stream.generate(() -> 70.0 + 10.0 * rng.nextGaussian())
    .limit(500)
    .collect(Collectors.toList());

var chart = SeraPlot.hist()
    .title("Distribution des notes d'examen")
    .values(valeurs)
    .bins(25)
    .xLabel("Note")
    .showCounts(true)
    .build();
chart.show();</code></pre></div>
<div id="histogram-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var rng = new Random(42);
var valeurs = Enumerable.Range(0, 500).Select(_ => {
    double u1 = rng.NextDouble(), u2 = rng.NextDouble();
    return 70.0 + 10.0 * Math.Sqrt(-2.0 * Math.Log(u1)) * Math.Cos(2 * Math.PI * u2);
}).ToList();

var chart = Sp.Hist(
    title: "Distribution des notes d'examen",
    values: valeurs,
    bins: 25,
    xLabel: "Note",
    showCounts: true
);
chart.Show();</code></pre></div>
<div id="histogram-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val rng = new scala.util.Random(42)
val valeurs = List.fill(500)(70.0 + 10.0 * rng.nextGaussian())

val chart = sp.hist(
  title = "Distribution des notes d'examen",
  values = valeurs,
  bins = 25,
  x_label = "Note",
  show_counts = true
)
chart.show()</code></pre></div>
<div id="histogram-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;
#include &lt;random&gt;
#include &lt;vector&gt;

std::mt19937 rng(42);
std::normal_distribution&lt;double&gt; dist(70.0, 10.0);
std::vector&lt;double&gt; valeurs(500);
std::generate(valeurs.begin(), valeurs.end(), [&]{ return dist(rng); });

auto chart = sp::hist({
  .title       = "Distribution des notes d'examen",
  .values      = valeurs,
  .bins        = 25,
  .x_label     = "Note",
  .show_counts = true
});
chart.show();</code></pre></div>
</div>

---

## Voir aussi

- [kde.md](kde.md) — Courbe d'estimation par noyau de densité
- [violin.md](violin.md) — Distribution par groupe incluant une KDE
- [boxplot.md](boxplot.md) — Résumé en cinq chiffres par catégorie

</div>
