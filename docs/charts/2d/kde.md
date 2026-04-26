# KDE Chart

<div class="lang-en">

## Signature

```python
sp.build_kde_chart(
    title: str,
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    bandwidth: float = 1.0,
    fill: bool = True,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "Density",
    gridlines: bool = True,
    palette: list[int] | None = None,
    series_names: list[str] | None = None,
) -> Chart
```

Aliases: `sp.kde`

---

## Description

A Kernel Density Estimation (KDE) chart produces a smooth, continuous probability density curve from sample data, offering a cleaner view of the underlying distribution shape than a histogram. The `bandwidth` parameter scales the Gaussian smoothing kernel — decrease it to reveal fine structure, increase it to emphasize the global shape. When `series_names` is provided, `values` is interpreted as a flat concatenation of equal-sized groups, and one density curve is drawn per group with distinct colors. SeraPlot computes the KDE entirely in Rust with no Python scipy dependency.

**Ideal for:**
- Probability density visualization for continuous variables
- Comparing distributional shapes of multiple groups on the same axes
- Replacing histograms when the number of bins is hard to choose objectively

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `values` | `list[float]` | required | Sample values (concatenated across all groups if multi-series) |
| `color_hex` | `int` | `0x6366F1` | Curve color for single-series mode |
| `bandwidth` | `float` | `1.0` | Gaussian kernel bandwidth scaling factor |
| `fill` | `bool` | `True` | Fill area under the density curve |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `"Density"` | Y-axis label |
| `gridlines` | `bool` | `True` | Show gridlines |
| `palette` | `list[int] \| None` | `None` | Colors for each group in multi-series mode |
| `series_names` | `list[str] \| None` | `None` | Group names; triggers multi-series mode when set |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="kde">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('kde','kde-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('kde','kde-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('kde','kde-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('kde','kde-r',this)">R</button>
<button class="sp-tb" onclick="spTab('kde','kde-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('kde','kde-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('kde','kde-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('kde','kde-cpp',this)">C++</button>
</div>
<div id="kde-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random

random.seed(7)
group_a = [random.gauss(45000, 8000) for _ in range(300)]
group_b = [random.gauss(65000, 12000) for _ in range(300)]
group_c = [random.gauss(90000, 15000) for _ in range(300)]

chart = sp.kde(
    title="Income Distribution by Education",
    values=group_a + group_b + group_c,
    series_names=["High School", "Bachelor's", "Master's+"],
    x_label="Annual Income (€)",
    bandwidth=1.2,
)
chart.show()</code></pre></div>
<div id="kde-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const chart = sp.kde({
  title: "Income Distribution by Education",
  values: [...groupA, ...groupB, ...groupC],
  seriesNames: ["High School", "Bachelor's", "Master's+"],
  xLabel: "Annual Income (€)",
  bandwidth: 1.2,
});
chart.show();</code></pre></div>
<div id="kde-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const chart = sp.kde({
  title: "Income Distribution by Education",
  values: [...groupA, ...groupB, ...groupC],
  seriesNames: ["High School", "Bachelor's", "Master's+"],
  xLabel: "Annual Income (€)",
  bandwidth: 1.2,
});
chart.show();</code></pre></div>
<div id="kde-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

set.seed(7)
groupe_a <- rnorm(300, mean = 45000, sd = 8000)
groupe_b <- rnorm(300, mean = 65000, sd = 12000)
groupe_c <- rnorm(300, mean = 90000, sd = 15000)

chart <- sp$kde(
  title        = "Income Distribution by Education",
  values       = c(groupe_a, groupe_b, groupe_c),
  series_names = c("High School", "Bachelor's", "Master's+"),
  x_label      = "Annual Income (€)",
  bandwidth    = 1.2
)
chart$show()</code></pre></div>
<div id="kde-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.ArrayList;
import java.util.List;

var all = new ArrayList<Double>();
all.addAll(groupA); all.addAll(groupB); all.addAll(groupC);

var chart = SeraPlot.kde()
    .title("Income Distribution by Education")
    .values(all)
    .seriesNames(List.of("High School", "Bachelor's", "Master's+"))
    .xLabel("Annual Income (€)")
    .bandwidth(1.2)
    .build();
chart.show();</code></pre></div>
<div id="kde-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Kde(
    title:       "Income Distribution by Education",
    values:      [..groupA, ..groupB, ..groupC],
    seriesNames: ["High School", "Bachelor's", "Master's+"],
    xLabel:      "Annual Income (€)",
    bandwidth:   1.2
);
chart.Show();</code></pre></div>
<div id="kde-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.kde(
  title        = "Income Distribution by Education",
  values       = groupA ++ groupB ++ groupC,
  series_names = List("High School", "Bachelor's", "Master's+"),
  x_label      = "Annual Income (€)",
  bandwidth    = 1.2
)
chart.show()</code></pre></div>
<div id="kde-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::kde({
    .title        = "Income Distribution by Education",
    .values       = all_values,
    .series_names = {"High School", "Bachelor's", "Master's+"},
    .x_label      = "Annual Income (€)",
    .bandwidth    = 1.2,
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/kde.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [Histogram](histogram.md) — `sp.build_histogram()`
- [Violin Chart](violin.md) — `sp.build_violin()`
- [Ridgeline Chart](ridgeline.md) — `sp.build_ridgeline_chart()`

</div>

<div class="lang-fr">

## Signature

```python
sp.build_kde_chart(
    title: str,
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    bandwidth: float = 1.0,
    fill: bool = True,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "Density",
    gridlines: bool = True,
    palette: list[int] | None = None,
    series_names: list[str] | None = None,
) -> Chart
```

Aliases: `sp.kde`

---

## Description

Un graphique d'estimation par noyau (KDE) produit une courbe de densité de probabilité lisse et continue à partir de données échantillonnées, offrant une meilleure vue de la forme de la distribution sous-jacente qu'un histogramme. Le paramètre `bandwidth` ajuste le noyau gaussien de lissage — diminuez-le pour révéler la structure fine, augmentez-le pour mettre en valeur la forme globale. Lorsque `series_names` est fourni, `values` est interprété comme une concaténation plate de groupes de taille égale, et une courbe de densité est tracée par groupe. SeraPlot calcule le KDE entièrement en Rust, sans dépendance Python à scipy.

**Idéal pour :**
- Visualisation de la densité de probabilité pour des variables continues
- Comparer les formes de distribution de plusieurs groupes sur les mêmes axes
- Remplacer les histogrammes quand le nombre de classes est difficile à choisir

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `values` | `list[float]` | requis | Valeurs échantillonnées (concaténées si multi-séries) |
| `color_hex` | `int` | `0x6366F1` | Couleur de la courbe en mode série unique |
| `bandwidth` | `float` | `1.0` | Facteur d'échelle du noyau gaussien |
| `fill` | `bool` | `True` | Remplir la surface sous la courbe |
| `width` | `int` | `900` | Largeur du canvas en pixels |
| `height` | `int` | `480` | Hauteur du canvas en pixels |
| `x_label` | `str` | `""` | Étiquette de l'axe X |
| `y_label` | `str` | `"Density"` | Étiquette de l'axe Y |
| `gridlines` | `bool` | `True` | Afficher les lignes de grille |
| `palette` | `list[int] \| None` | `None` | Couleurs pour chaque groupe en mode multi-séries |
| `series_names` | `list[str] \| None` | `None` | Noms des groupes ; déclenche le mode multi-séries |

---

## Retourne

`Chart`

---

<div class="sp-tabs" id="kde-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('kde-fr','kde-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('kde-fr','kde-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('kde-fr','kde-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('kde-fr','kde-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('kde-fr','kde-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('kde-fr','kde-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('kde-fr','kde-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('kde-fr','kde-fr-cpp',this)">C++</button>
</div>
<div id="kde-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
import random

random.seed(7)
groupe_a = [random.gauss(45000, 8000) for _ in range(300)]
groupe_b = [random.gauss(65000, 12000) for _ in range(300)]
groupe_c = [random.gauss(90000, 15000) for _ in range(300)]

chart = sp.kde(
    title="Distribution des revenus par niveau d'éducation",
    values=groupe_a + groupe_b + groupe_c,
    series_names=["Bac", "Licence", "Master+"],
    x_label="Revenu annuel (€)",
    bandwidth=1.2,
)
chart.show()</code></pre></div>
<div id="kde-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const chart = sp.kde({
  title: "Distribution des revenus par niveau d'éducation",
  values: [...groupeA, ...groupeB, ...groupeC],
  seriesNames: ["Bac", "Licence", "Master+"],
  xLabel: "Revenu annuel (€)",
  bandwidth: 1.2,
});
chart.show();</code></pre></div>
<div id="kde-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const chart = sp.kde({
  title: "Distribution des revenus par niveau d'éducation",
  values: [...groupeA, ...groupeB, ...groupeC],
  seriesNames: ["Bac", "Licence", "Master+"],
  xLabel: "Revenu annuel (€)",
  bandwidth: 1.2,
});
chart.show();</code></pre></div>
<div id="kde-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

set.seed(7)
groupe_a <- rnorm(300, mean = 45000, sd = 8000)
groupe_b <- rnorm(300, mean = 65000, sd = 12000)
groupe_c <- rnorm(300, mean = 90000, sd = 15000)

chart <- sp$kde(
  title        = "Distribution des revenus par niveau d'éducation",
  values       = c(groupe_a, groupe_b, groupe_c),
  series_names = c("Bac", "Licence", "Master+"),
  x_label      = "Revenu annuel (€)",
  bandwidth    = 1.2
)
chart$show()</code></pre></div>
<div id="kde-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.ArrayList;
import java.util.List;

var all = new ArrayList<Double>();
all.addAll(groupeA); all.addAll(groupeB); all.addAll(groupeC);

var chart = SeraPlot.kde()
    .title("Distribution des revenus par niveau d'éducation")
    .values(all)
    .seriesNames(List.of("Bac", "Licence", "Master+"))
    .xLabel("Revenu annuel (€)")
    .bandwidth(1.2)
    .build();
chart.show();</code></pre></div>
<div id="kde-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Kde(
    title:       "Distribution des revenus par niveau d'éducation",
    values:      [..groupeA, ..groupeB, ..groupeC],
    seriesNames: ["Bac", "Licence", "Master+"],
    xLabel:      "Revenu annuel (€)",
    bandwidth:   1.2
);
chart.Show();</code></pre></div>
<div id="kde-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.kde(
  title        = "Distribution des revenus par niveau d'éducation",
  values       = groupeA ++ groupeB ++ groupeC,
  series_names = List("Bac", "Licence", "Master+"),
  x_label      = "Revenu annuel (€)",
  bandwidth    = 1.2
)
chart.show()</code></pre></div>
<div id="kde-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::kde({
    .title        = "Distribution des revenus par niveau d'éducation",
    .values       = all_values,
    .series_names = {"Bac", "Licence", "Master+"},
    .x_label      = "Revenu annuel (€)",
    .bandwidth    = 1.2,
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/kde.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## Voir aussi

- [Histogramme](histogram.md) — `sp.build_histogram()`
- [Graphique en violon](violin.md) — `sp.build_violin()`
- [Graphique en arêtes](ridgeline.md) — `sp.build_ridgeline_chart()`

</div>
