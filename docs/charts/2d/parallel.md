# Parallel Coordinates

<div class="lang-en">

## Signature

```python
sp.build_parallel(
    title: str,
    axes: list[str],
    series: list[list[float]],
    *,
    series_names: list[str] | None = None,
    color_groups: list[str] | None = None,
    palette: list[int] | None = None,
    width: int = 1000,
    height: int = 480,
    background: str | None = None,
    line_opacity: float = 0.6,
) -> Chart
```

Aliases: `sp.parallel`

---

## Description

Parallel coordinates place each dimension on its own vertical axis, with observations drawn as polylines connecting their values across all axes. The resulting visual immediately reveals correlations (parallel lines), inverse correlations (crossing lines), clusters (bundles of similar lines), and outliers (isolated crossing lines). Each inner list in `series` must have length equal to `len(axes)`. Color groups paint different observations in different colors, turning the chart into a multi-class pattern explorer. `line_opacity` controls visual density for large datasets.

**Ideal for:**
- Exploring high-dimensional datasets (5–15 variables) for patterns and clusters
- Comparing multiple entities (products, countries, patients) across many attributes simultaneously
- Preliminary EDA before dimensionality reduction or clustering

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `axes` | `list[str]` | required | Axis names (one per dimension) |
| `series` | `list[list[float]]` | required | Each inner list is one observation with `len(axes)` values |
| `series_names` | `list[str] \| None` | `None` | Name for each observation (used in tooltips) |
| `color_groups` | `list[str] \| None` | `None` | Group label per observation for color coding |
| `palette` | `list[int] \| None` | `None` | Colors for each distinct color group |
| `width` | `int` | `1000` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `background` | `str \| None` | `None` | Background color or `None` = transparent |
| `line_opacity` | `float` | `0.6` | Line opacity (0.0–1.0) |

---

## Returns

`Chart`

---

## Examples

### Iris dataset — 4 measurements

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="parallel">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('parallel','parallel-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('parallel','parallel-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('parallel','parallel-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('parallel','parallel-r',this)">R</button>
<button class="sp-tb" onclick="spTab('parallel','parallel-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('parallel','parallel-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('parallel','parallel-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('parallel','parallel-cpp',this)">C++</button>
</div>
<div id="parallel-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

# 12 Iris observations (4 per species), 4 features each
sepal_l = [5.1,4.9,4.7,7.0,6.4,6.9,6.3,5.8,7.1,5.8,5.7,6.2]
sepal_w = [3.5,3.0,3.2,3.2,3.2,3.1,3.3,2.7,3.0,2.8,2.8,2.9]
petal_l = [1.4,1.4,1.3,4.7,4.5,4.9,6.0,5.1,5.9,5.1,4.5,4.3]
petal_w = [0.2,0.2,0.2,1.4,1.5,1.5,2.5,1.9,2.1,1.8,1.3,1.3]
species = ["Setosa"]*3 + ["Versicolor"]*3 + ["Virginica"]*3 + ["Versicolor"]*3

chart = sp.parallel(
    title="Iris Dataset — Parallel Coordinates",
    axes=["Sepal L", "Sepal W", "Petal L", "Petal W"],
    series=[[sepal_l[i], sepal_w[i], petal_l[i], petal_w[i]] for i in range(12)],
    color_groups=species,
    line_opacity=0.7,
)
chart.show()</code></pre></div>
<div id="parallel-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const sepalL = [5.1,4.9,4.7,7.0,6.4,6.9,6.3,5.8,7.1,5.8,5.7,6.2];
const sepalW = [3.5,3.0,3.2,3.2,3.2,3.1,3.3,2.7,3.0,2.8,2.8,2.9];
const petalL = [1.4,1.4,1.3,4.7,4.5,4.9,6.0,5.1,5.9,5.1,4.5,4.3];
const petalW = [0.2,0.2,0.2,1.4,1.5,1.5,2.5,1.9,2.1,1.8,1.3,1.3];
const species = [...Array(3).fill("Setosa"), ...Array(3).fill("Versicolor"),
                 ...Array(3).fill("Virginica"), ...Array(3).fill("Versicolor")];

const chart = sp.parallel({
  title: "Iris Dataset — Parallel Coordinates",
  axes: ["Sepal L", "Sepal W", "Petal L", "Petal W"],
  series: sepalL.map((_, i) => [sepalL[i], sepalW[i], petalL[i], petalW[i]]),
  colorGroups: species,
  lineOpacity: 0.7,
});
chart.show();</code></pre></div>
<div id="parallel-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const sepalL: number[] = [5.1,4.9,4.7,7.0,6.4,6.9,6.3,5.8,7.1,5.8,5.7,6.2];
const sepalW: number[] = [3.5,3.0,3.2,3.2,3.2,3.1,3.3,2.7,3.0,2.8,2.8,2.9];
const petalL: number[] = [1.4,1.4,1.3,4.7,4.5,4.9,6.0,5.1,5.9,5.1,4.5,4.3];
const petalW: number[] = [0.2,0.2,0.2,1.4,1.5,1.5,2.5,1.9,2.1,1.8,1.3,1.3];
const species: string[] = [...Array(3).fill("Setosa"), ...Array(3).fill("Versicolor"),
                           ...Array(3).fill("Virginica"), ...Array(3).fill("Versicolor")];

const chart = sp.parallel({
  title: "Iris Dataset — Parallel Coordinates",
  axes: ["Sepal L", "Sepal W", "Petal L", "Petal W"],
  series: sepalL.map((_, i) => [sepalL[i], sepalW[i], petalL[i], petalW[i]]),
  colorGroups: species,
  lineOpacity: 0.7,
});
chart.show();</code></pre></div>
<div id="parallel-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

sepal_l <- c(5.1,4.9,4.7,7.0,6.4,6.9,6.3,5.8,7.1,5.8,5.7,6.2)
sepal_w <- c(3.5,3.0,3.2,3.2,3.2,3.1,3.3,2.7,3.0,2.8,2.8,2.9)
petal_l <- c(1.4,1.4,1.3,4.7,4.5,4.9,6.0,5.1,5.9,5.1,4.5,4.3)
petal_w <- c(0.2,0.2,0.2,1.4,1.5,1.5,2.5,1.9,2.1,1.8,1.3,1.3)
especes <- c(rep("Setosa",3), rep("Versicolor",3), rep("Virginica",3), rep("Versicolor",3))

chart <- sp$parallel(
  title        = "Iris Dataset — Coordonnées parallèles",
  axes         = c("Sepal L", "Sepal W", "Petal L", "Petal W"),
  series       = lapply(1:12, function(i) c(sepal_l[i], sepal_w[i], petal_l[i], petal_w[i])),
  color_groups = especes,
  line_opacity = 0.7
)
chart$show()</code></pre></div>
<div id="parallel-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var series = List.of(
    List.of(5.1,3.5,1.4,0.2), List.of(4.9,3.0,1.4,0.2), List.of(4.7,3.2,1.3,0.2),
    List.of(7.0,3.2,4.7,1.4), List.of(6.4,3.2,4.5,1.5), List.of(6.9,3.1,4.9,1.5),
    List.of(6.3,3.3,6.0,2.5), List.of(5.8,2.7,5.1,1.9), List.of(7.1,3.0,5.9,2.1),
    List.of(5.8,2.8,5.1,1.8), List.of(5.7,2.8,4.5,1.3), List.of(6.2,2.9,4.3,1.3)
);
var colorGroups = List.of("Setosa","Setosa","Setosa","Versicolor","Versicolor","Versicolor",
    "Virginica","Virginica","Virginica","Versicolor","Versicolor","Versicolor");

var chart = SeraPlot.parallel()
    .title("Iris Dataset — Parallel Coordinates")
    .axes(List.of("Sepal L", "Sepal W", "Petal L", "Petal W"))
    .series(series)
    .colorGroups(colorGroups)
    .lineOpacity(0.7)
    .build();
chart.show();</code></pre></div>
<div id="parallel-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

double[][] series = [
    [5.1,3.5,1.4,0.2], [4.9,3.0,1.4,0.2], [4.7,3.2,1.3,0.2],
    [7.0,3.2,4.7,1.4], [6.4,3.2,4.5,1.5], [6.9,3.1,4.9,1.5],
    [6.3,3.3,6.0,2.5], [5.8,2.7,5.1,1.9], [7.1,3.0,5.9,2.1],
    [5.8,2.8,5.1,1.8], [5.7,2.8,4.5,1.3], [6.2,2.9,4.3,1.3],
];
var chart = Sp.Parallel(
    title:       "Iris Dataset — Parallel Coordinates",
    axes:        ["Sepal L", "Sepal W", "Petal L", "Petal W"],
    series:      series,
    colorGroups: ["Setosa","Setosa","Setosa","Versicolor","Versicolor","Versicolor",
                  "Virginica","Virginica","Virginica","Versicolor","Versicolor","Versicolor"],
    lineOpacity: 0.7
);
chart.Show();</code></pre></div>
<div id="parallel-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val sepalL = List(5.1,4.9,4.7,7.0,6.4,6.9,6.3,5.8,7.1,5.8,5.7,6.2)
val sepalW = List(3.5,3.0,3.2,3.2,3.2,3.1,3.3,2.7,3.0,2.8,2.8,2.9)
val petalL = List(1.4,1.4,1.3,4.7,4.5,4.9,6.0,5.1,5.9,5.1,4.5,4.3)
val petalW = List(0.2,0.2,0.2,1.4,1.5,1.5,2.5,1.9,2.1,1.8,1.3,1.3)
val especes = List.fill(3)("Setosa") ++ List.fill(3)("Versicolor") ++
              List.fill(3)("Virginica") ++ List.fill(3)("Versicolor")

val chart = sp.parallel(
  title        = "Iris Dataset — Parallel Coordinates",
  axes         = List("Sepal L", "Sepal W", "Petal L", "Petal W"),
  series       = (0 until 12).map(i => List(sepalL(i), sepalW(i), petalL(i), petalW(i))).toList,
  color_groups = especes,
  line_opacity = 0.7
)
chart.show()</code></pre></div>
<div id="parallel-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::parallel({
    .title        = "Iris Dataset — Parallel Coordinates",
    .axes         = {"Sepal L", "Sepal W", "Petal L", "Petal W"},
    .series       = {
        {5.1,3.5,1.4,0.2}, {4.9,3.0,1.4,0.2}, {4.7,3.2,1.3,0.2},
        {7.0,3.2,4.7,1.4}, {6.4,3.2,4.5,1.5}, {6.9,3.1,4.9,1.5},
        {6.3,3.3,6.0,2.5}, {5.8,2.7,5.1,1.9}, {7.1,3.0,5.9,2.1},
        {5.8,2.8,5.1,1.8}, {5.7,2.8,4.5,1.3}, {6.2,2.9,4.3,1.3},
    },
    .color_groups = {"Setosa","Setosa","Setosa","Versicolor","Versicolor","Versicolor",
                     "Virginica","Virginica","Virginica","Versicolor","Versicolor","Versicolor"},
    .line_opacity = 0.7f,
});
chart.show();</code></pre></div>
</div>

<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>
<iframe src="../../previews/parallel.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>
</details>

### Without color groups

```python
chart = sp.parallel(
    title="Car Performance Metrics",
    axes=["MPG", "Cylinders", "Displacement", "Horsepower", "Weight"],
    series=car_data,
    line_opacity=0.4,
)
```

---

## See also

- [Scatter Chart](scatter.md) — `sp.build_scatter_chart()`
- [Heatmap](heatmap.md) — `sp.build_heatmap()`
- [Radar Chart](radar.md) — `sp.build_radar_chart()`

</div>

<div class="lang-fr">

## Signature

```python
sp.build_parallel(
    title: str,
    axes: list[str],
    series: list[list[float]],
    *,
    series_names: list[str] | None = None,
    color_groups: list[str] | None = None,
    palette: list[int] | None = None,
    width: int = 1000,
    height: int = 480,
    background: str | None = None,
    line_opacity: float = 0.6,
) -> Chart
```

Aliases: `sp.parallel`

---

## Description

Les coordonnées parallèles placent chaque dimension sur son propre axe vertical, avec les observations tracées comme des polylignes reliant leurs valeurs sur tous les axes. Le visuel révèle immédiatement les corrélations (lignes parallèles), les corrélations inverses (lignes croisées), les clusters (paquets de lignes similaires) et les valeurs aberrantes. Chaque liste interne de `series` doit avoir une longueur égale à `len(axes)`. Les groupes de couleurs colorient différentes observations, transformant le graphique en explorateur de motifs multi-classes.

**Idéal pour :**
- Explorer des jeux de données à haute dimension (5–15 variables) à la recherche de motifs et clusters
- Comparer plusieurs entités (produits, pays, patients) sur de nombreux attributs simultanément
- EDA préliminaire avant réduction de dimensionnalité ou clustering

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `axes` | `list[str]` | requis | Noms des axes (un par dimension) |
| `series` | `list[list[float]]` | requis | Chaque liste interne est une observation avec `len(axes)` valeurs |
| `series_names` | `list[str] \| None` | `None` | Nom de chaque observation (pour les infobulles) |
| `color_groups` | `list[str] \| None` | `None` | Étiquette de groupe par observation pour le codage couleur |
| `palette` | `list[int] \| None` | `None` | Couleurs pour chaque groupe distinct |
| `width` | `int` | `1000` | Largeur du canvas en pixels |
| `height` | `int` | `480` | Hauteur du canvas en pixels |
| `background` | `str \| None` | `None` | Couleur de fond ou `None` = transparent |
| `line_opacity` | `float` | `0.6` | Opacité des lignes (0.0–1.0) |

---

## Retourne

`Chart`

---

## Exemples

### Jeu de données Iris — 4 mesures

<div class="sp-tabs" id="parallel-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('parallel-fr','parallel-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('parallel-fr','parallel-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('parallel-fr','parallel-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('parallel-fr','parallel-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('parallel-fr','parallel-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('parallel-fr','parallel-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('parallel-fr','parallel-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('parallel-fr','parallel-fr-cpp',this)">C++</button>
</div>
<div id="parallel-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

long_sep = [5.1,4.9,4.7,7.0,6.4,6.9,6.3,5.8,7.1,5.8,5.7,6.2]
larg_sep = [3.5,3.0,3.2,3.2,3.2,3.1,3.3,2.7,3.0,2.8,2.8,2.9]
long_pet = [1.4,1.4,1.3,4.7,4.5,4.9,6.0,5.1,5.9,5.1,4.5,4.3]
larg_pet = [0.2,0.2,0.2,1.4,1.5,1.5,2.5,1.9,2.1,1.8,1.3,1.3]
especes  = ["Setosa"]*3 + ["Versicolor"]*3 + ["Virginica"]*3 + ["Versicolor"]*3

chart = sp.parallel(
    title="Iris — Coordonnées parallèles",
    axes=["Long. Sép.", "Larg. Sép.", "Long. Pét.", "Larg. Pét."],
    series=[[long_sep[i], larg_sep[i], long_pet[i], larg_pet[i]] for i in range(12)],
    color_groups=especes,
    line_opacity=0.7,
)
chart.show()</code></pre></div>
<div id="parallel-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const chart = sp.parallel({
  title: "Iris — Coordonnées parallèles",
  axes: ["Long. Sép.", "Larg. Sép.", "Long. Pét.", "Larg. Pét."],
  series: longSep.map((_, i) => [longSep[i], largSep[i], longPet[i], largPet[i]]),
  colorGroups: especes,
  lineOpacity: 0.7,
});
chart.show();</code></pre></div>
<div id="parallel-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const chart = sp.parallel({
  title: "Iris — Coordonnées parallèles",
  axes: ["Long. Sép.", "Larg. Sép.", "Long. Pét.", "Larg. Pét."],
  series: longSep.map((_, i) => [longSep[i], largSep[i], longPet[i], largPet[i]]),
  colorGroups: especes,
  lineOpacity: 0.7,
});
chart.show();</code></pre></div>
<div id="parallel-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

long_sep <- c(5.1,4.9,4.7,7.0,6.4,6.9,6.3,5.8,7.1,5.8,5.7,6.2)
larg_sep <- c(3.5,3.0,3.2,3.2,3.2,3.1,3.3,2.7,3.0,2.8,2.8,2.9)
long_pet <- c(1.4,1.4,1.3,4.7,4.5,4.9,6.0,5.1,5.9,5.1,4.5,4.3)
larg_pet <- c(0.2,0.2,0.2,1.4,1.5,1.5,2.5,1.9,2.1,1.8,1.3,1.3)
especes  <- c(rep("Setosa",3),rep("Versicolor",3),rep("Virginica",3),rep("Versicolor",3))

chart <- sp$parallel(
  title        = "Iris — Coordonnées parallèles",
  axes         = c("Long. Sép.", "Larg. Sép.", "Long. Pét.", "Larg. Pét."),
  series       = lapply(1:12, function(i) c(long_sep[i], larg_sep[i], long_pet[i], larg_pet[i])),
  color_groups = especes,
  line_opacity = 0.7
)
chart$show()</code></pre></div>
<div id="parallel-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.parallel()
    .title("Iris — Coordonnées parallèles")
    .axes(List.of("Long. Sép.", "Larg. Sép.", "Long. Pét.", "Larg. Pét."))
    .series(series)
    .colorGroups(especes)
    .lineOpacity(0.7)
    .build();
chart.show();</code></pre></div>
<div id="parallel-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Parallel(
    title:       "Iris — Coordonnées parallèles",
    axes:        ["Long. Sép.", "Larg. Sép.", "Long. Pét.", "Larg. Pét."],
    series:      series,
    colorGroups: especes,
    lineOpacity: 0.7
);
chart.Show();</code></pre></div>
<div id="parallel-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.parallel(
  title        = "Iris — Coordonnées parallèles",
  axes         = List("Long. Sép.", "Larg. Sép.", "Long. Pét.", "Larg. Pét."),
  series       = series,
  color_groups = especes,
  line_opacity = 0.7
)
chart.show()</code></pre></div>
<div id="parallel-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::parallel({
    .title        = "Iris — Coordonnées parallèles",
    .axes         = {"Long. Sép.", "Larg. Sép.", "Long. Pét.", "Larg. Pét."},
    .series       = series,
    .color_groups = especes,
    .line_opacity = 0.7f,
});
chart.show();</code></pre></div>
</div>

---

## Voir aussi

- [Nuage de points](scatter.md) — `sp.build_scatter_chart()`
- [Carte de chaleur](heatmap.md) — `sp.build_heatmap()`
- [Graphique radar](radar.md) — `sp.build_radar_chart()`

</div>
