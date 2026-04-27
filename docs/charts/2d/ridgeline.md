# Ridgeline Chart

<div class="lang-en">

## Signature

```python
sp.build_ridgeline_chart(
    title: str,
    categories: list[str],
    values: list[float],
    *,
    bandwidth: float = 1.0,
    overlap: float = 0.5,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    gridlines: bool = False,
) -> Chart
```

Aliases: `sp.ridgeline`

---

## Description

A ridgeline chart (joyplot) stacks KDE density curves for multiple categories along a shared X axis, creating a mountain-ridge appearance that is both aesthetically striking and analytically powerful. `values` is a flat list where all category samples are concatenated in order; each category must have the same number of values (`len(values) / len(categories)`). The `overlap` parameter controls vertical spacing: 0 means no overlap (full separation), 1 means categories completely stack on top of each other. SeraPlot computes all KDE curves simultaneously in parallel threads.

**Ideal for:**
- Comparing distributional shapes across many categories (hourly traffic by day, income by country)
- Revealing bimodality or shifts across ordered groups
- Creating publication-ready distribution comparisons with minimal visual noise

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `categories` | `list[str]` | required | Category labels (one per density ridge) |
| `values` | `list[float]` | required | All sample values concatenated; length = `len(categories) × n` |
| `bandwidth` | `float` | `1.0` | Gaussian kernel bandwidth scaling factor |
| `overlap` | `float` | `0.5` | Ridge overlap ratio (0 = full separation, 1 = full overlap) |
| `color_hex` | `int` | `0x6366F1` | Base fill color; `palette` overrides this |
| `palette` | `list[int] \| None` | `None` | Per-category colors |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `x_label` | `str` | `""` | X-axis label |
| `gridlines` | `bool` | `False` | Show vertical gridlines |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="ridgeline">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('ridgeline','ridgeline-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('ridgeline','ridgeline-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('ridgeline','ridgeline-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('ridgeline','ridgeline-r',this)">R</button>
<button class="sp-tb" onclick="spTab('ridgeline','ridgeline-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('ridgeline','ridgeline-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('ridgeline','ridgeline-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('ridgeline','ridgeline-cpp',this)">C++</button>
</div>
<div id="ridgeline-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(99)
months = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"]
means  = [5, 7, 11, 15, 19, 23, 26, 25, 21, 16, 10, 6]
values = []
for m in means:
    values.extend([random.gauss(m, 3) for _ in range(100)])
chart = sp.ridgeline(
    title="Daily Temperature Distribution by Month (°C)",
    categories=months,
    values=values,
    bandwidth=1.1,
    overlap=0.6,
    x_label="Temperature (°C)",
)
chart.show()</code></pre></div>
<div id="ridgeline-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const months = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];
const chart = sp.ridgeline({
  title: "Daily Temperature Distribution by Month (°C)",
  categories: months,
  values: values,
  bandwidth: 1.1,
  overlap: 0.6,
  xLabel: "Temperature (°C)",
});
chart.show();</code></pre></div>
<div id="ridgeline-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const months: string[] = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];
const chart = sp.ridgeline({
  title: "Daily Temperature Distribution by Month (°C)",
  categories: months,
  values: values,
  bandwidth: 1.1,
  overlap: 0.6,
  xLabel: "Temperature (°C)",
});
chart.show();</code></pre></div>
<div id="ridgeline-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
set.seed(99)
months <- c("Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec")
means  <- c(5, 7, 11, 15, 19, 23, 26, 25, 21, 16, 10, 6)
values <- unlist(lapply(means, function(m) rnorm(100, mean = m, sd = 3)))
chart <- sp$ridgeline(
  title      = "Daily Temperature Distribution by Month (°C)",
  categories = months,
  values     = values,
  bandwidth  = 1.1,
  overlap    = 0.6,
  x_label    = "Temperature (°C)"
)
chart$show()</code></pre></div>
<div id="ridgeline-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
var chart = SeraPlot.ridgeline()
    .title("Daily Temperature Distribution by Month (°C)")
    .categories(List.of("Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"))
    .values(values)
    .bandwidth(1.1)
    .overlap(0.6)
    .xLabel("Temperature (°C)")
    .build();
chart.show();</code></pre></div>
<div id="ridgeline-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;
var chart = Sp.Ridgeline(
    title:      "Daily Temperature Distribution by Month (°C)",
    categories: ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"],
    values:     values,
    bandwidth:  1.1,
    overlap:    0.6,
    xLabel:     "Temperature (°C)"
);
chart.Show();</code></pre></div>
<div id="ridgeline-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp
val chart = sp.ridgeline(
  title      = "Daily Temperature Distribution by Month (°C)",
  categories = List("Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"),
  values     = values,
  bandwidth  = 1.1,
  overlap    = 0.6,
  x_label    = "Temperature (°C)"
)
chart.show()</code></pre></div>
<div id="ridgeline-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;
auto chart = sp::ridgeline({
    .title      = "Daily Temperature Distribution by Month (°C)",
    .categories = {"Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"},
    .values     = values,
    .bandwidth  = 1.1,
    .overlap    = 0.6,
    .x_label    = "Temperature (°C)",
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/ridgeline.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [KDE Chart](kde.md) — `sp.build_kde_chart()`
- [Violin Chart](violin.md) — `sp.build_violin()`
- [Histogram](histogram.md) — `sp.build_histogram()`

</div>

<div class="lang-fr">

## Signature

```python
sp.build_ridgeline_chart(
    title: str,
    categories: list[str],
    values: list[float],
    *,
    bandwidth: float = 1.0,
    overlap: float = 0.5,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    gridlines: bool = False,
) -> Chart
```

Aliases: `sp.ridgeline`

---

## Description

Un graphique en arêtes (ridgeline chart ou joyplot) empile des courbes de densité KDE pour plusieurs catégories le long d'un axe X commun, créant un effet de chaîne de montagnes à la fois esthétique et analytiquement puissant. `values` est une liste plate où tous les échantillons de catégories sont concaténés dans l'ordre ; chaque catégorie doit avoir le même nombre de valeurs (`len(values) / len(categories)`). Le paramètre `overlap` contrôle l'espacement vertical : 0 = pas de chevauchement, 1 = chevauchement complet. SeraPlot calcule toutes les courbes KDE simultanément dans des threads parallèles.

**Idéal pour :**
- Comparer les formes de distribution sur de nombreuses catégories (trafic horaire par jour, revenus par pays)
- Révéler la bimodalité ou les décalages entre groupes ordonnés
- Créer des comparaisons de distributions prêtes à publier avec un bruit visuel minimal

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `categories` | `list[str]` | requis | Étiquettes des catégories (une crête par catégorie) |
| `values` | `list[float]` | requis | Toutes les valeurs concaténées ; longueur = `len(categories) × n` |
| `bandwidth` | `float` | `1.0` | Facteur d'échelle du noyau gaussien |
| `overlap` | `float` | `0.5` | Ratio de chevauchement des crêtes (0 = séparation, 1 = plein chevauchement) |
| `color_hex` | `int` | `0x6366F1` | Couleur de remplissage de base ; `palette` l'écrase |
| `palette` | `list[int] \| None` | `None` | Couleurs par catégorie |
| `width` | `int` | `900` | Largeur du canvas en pixels |
| `height` | `int` | `480` | Hauteur du canvas en pixels |
| `x_label` | `str` | `""` | Étiquette de l'axe X |
| `gridlines` | `bool` | `False` | Afficher les lignes de grille verticales |

---

## Retourne

`Chart`

---

<div class="sp-tabs" id="ridgeline-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('ridgeline-fr','ridgeline-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('ridgeline-fr','ridgeline-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('ridgeline-fr','ridgeline-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('ridgeline-fr','ridgeline-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('ridgeline-fr','ridgeline-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('ridgeline-fr','ridgeline-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('ridgeline-fr','ridgeline-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('ridgeline-fr','ridgeline-fr-cpp',this)">C++</button>
</div>
<div id="ridgeline-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random
random.seed(99)
mois   = ["Jan","Fév","Mar","Avr","Mai","Juin","Juil","Août","Sep","Oct","Nov","Déc"]
moyens = [5, 7, 11, 15, 19, 23, 26, 25, 21, 16, 10, 6]
valeurs = []
for m in moyens:
    valeurs.extend([random.gauss(m, 3) for _ in range(100)])
chart = sp.ridgeline(
    title="Distribution des températures par mois (°C)",
    categories=mois,
    values=valeurs,
    bandwidth=1.1,
    overlap=0.6,
    x_label="Température (°C)",
)
chart.show()</code></pre></div>
<div id="ridgeline-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const mois = ["Jan","Fév","Mar","Avr","Mai","Juin","Juil","Août","Sep","Oct","Nov","Déc"];
const chart = sp.ridgeline({
  title: "Distribution des températures par mois (°C)",
  categories: mois,
  values: valeurs,
  bandwidth: 1.1,
  overlap: 0.6,
  xLabel: "Température (°C)",
});
chart.show();</code></pre></div>
<div id="ridgeline-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const mois: string[] = ["Jan","Fév","Mar","Avr","Mai","Juin","Juil","Août","Sep","Oct","Nov","Déc"];
const chart = sp.ridgeline({
  title: "Distribution des températures par mois (°C)",
  categories: mois,
  values: valeurs,
  bandwidth: 1.1,
  overlap: 0.6,
  xLabel: "Température (°C)",
});
chart.show();</code></pre></div>
<div id="ridgeline-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
set.seed(99)
mois   <- c("Jan","Fév","Mar","Avr","Mai","Juin","Juil","Août","Sep","Oct","Nov","Déc")
moyens <- c(5, 7, 11, 15, 19, 23, 26, 25, 21, 16, 10, 6)
valeurs <- unlist(lapply(moyens, function(m) rnorm(100, mean = m, sd = 3)))
chart <- sp$ridgeline(
  title      = "Distribution des températures par mois (°C)",
  categories = mois,
  values     = valeurs,
  bandwidth  = 1.1,
  overlap    = 0.6,
  x_label    = "Température (°C)"
)
chart$show()</code></pre></div>
<div id="ridgeline-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
var chart = SeraPlot.ridgeline()
    .title("Distribution des températures par mois (°C)")
    .categories(List.of("Jan","Fév","Mar","Avr","Mai","Juin","Juil","Août","Sep","Oct","Nov","Déc"))
    .values(valeurs)
    .bandwidth(1.1)
    .overlap(0.6)
    .xLabel("Température (°C)")
    .build();
chart.show();</code></pre></div>
<div id="ridgeline-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;
var chart = Sp.Ridgeline(
    title:      "Distribution des températures par mois (°C)",
    categories: ["Jan","Fév","Mar","Avr","Mai","Juin","Juil","Août","Sep","Oct","Nov","Déc"],
    values:     valeurs,
    bandwidth:  1.1,
    overlap:    0.6,
    xLabel:     "Température (°C)"
);
chart.Show();</code></pre></div>
<div id="ridgeline-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp
val chart = sp.ridgeline(
  title      = "Distribution des températures par mois (°C)",
  categories = List("Jan","Fév","Mar","Avr","Mai","Juin","Juil","Août","Sep","Oct","Nov","Déc"),
  values     = valeurs,
  bandwidth  = 1.1,
  overlap    = 0.6,
  x_label    = "Température (°C)"
)
chart.show()</code></pre></div>
<div id="ridgeline-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;
auto chart = sp::ridgeline({
    .title      = "Distribution des températures par mois (°C)",
    .categories = {"Jan","Fév","Mar","Avr","Mai","Juin","Juil","Août","Sep","Oct","Nov","Déc"},
    .values     = valeurs,
    .bandwidth  = 1.1,
    .overlap    = 0.6,
    .x_label    = "Température (°C)",
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/ridgeline.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## Voir aussi

- [Courbe de densité (KDE)](kde.md) — `sp.build_kde_chart()`
- [Graphique en violon](violin.md) — `sp.build_violin()`
- [Histogramme](histogram.md) — `sp.build_histogram()`

</div>
