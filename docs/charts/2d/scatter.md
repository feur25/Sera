# Scatter Chart

<div class="lang-en">

## Signature

```python
sp.build_scatter_chart(
    title: str,
    x_values: list[float],
    y_values: list[float],
    *,
    color_hex: int = 0,
    show_text: bool = False,
    labels: list[str] | None = None,
    sizes: list[float] | None = None,
    color_groups: list[str] | None = None,
    show_regression: bool = False,
    regression_type: str = "linear",
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    background: str | None = None,
) -> Chart
```

Aliases: `sp.scatter`, `sp.scatter_chart`

---

## Description

A scatter chart plots each observation as a point on a two-dimensional Cartesian plane, making correlations, clusters, and outliers instantly visible. Each point can encode additional dimensions through size (bubble effect) and color groups. The optional regression overlay — linear or polynomial — calculates the best-fit curve directly in the Rust engine without any external dependency. SeraPlot handles tens of thousands of points with no performance degradation.

**Ideal for:**
- Visualizing correlation between two continuous variables
- Detecting clusters and outliers in datasets
- Showing multi-dimensional relationships using size and color

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | — | Chart title displayed at the top |
| `x_values` | `list[float]` | — | X coordinate for each observation |
| `y_values` | `list[float]` | — | Y coordinate for each observation |
| `color_hex` | `int` | `0` | Point fill color as hex integer; `0` uses the theme default |
| `show_text` | `bool` | `False` | Render the point label next to each dot |
| `labels` | `list[str] \| None` | `None` | Optional text label for each point |
| `sizes` | `list[float] \| None` | `None` | Per-point radius multiplier for a bubble effect |
| `color_groups` | `list[str] \| None` | `None` | Categorical group name per point; groups are auto-colored |
| `show_regression` | `bool` | `False` | Overlay a best-fit regression curve |
| `regression_type` | `str` | `"linear"` | Regression algorithm: `"linear"` or `"polynomial"` |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `x_label` | `str` | `""` | Label for the X axis |
| `y_label` | `str` | `""` | Label for the Y axis |
| `gridlines` | `bool` | `False` | Draw horizontal and vertical gridlines |
| `background` | `str \| None` | `None` | CSS background color override (e.g. `"#1e293b"`) |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="scatter">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('scatter','scatter-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('scatter','scatter-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('scatter','scatter-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('scatter','scatter-r',this)">R</button>
<button class="sp-tb" onclick="spTab('scatter','scatter-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('scatter','scatter-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('scatter','scatter-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('scatter','scatter-cpp',this)">C++</button>
</div>
<div id="scatter-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.scatter(
    title="Height vs Weight",
    x_values=[160, 165, 170, 172, 175, 178, 180, 182, 185, 190],
    y_values=[55, 60, 68, 65, 72, 75, 80, 78, 85, 90],
    gridlines=True,
    x_label="Height (cm)",
    y_label="Weight (kg)",
    show_regression=True,
)
chart.show()</code></pre></div>
<div id="scatter-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.scatter({
  title: "Height vs Weight",
  xValues: [160, 165, 170, 172, 175, 178, 180, 182, 185, 190],
  yValues: [55, 60, 68, 65, 72, 75, 80, 78, 85, 90],
  gridlines: true,
  xLabel: "Height (cm)",
  yLabel: "Weight (kg)",
  showRegression: true,
});
chart.show();</code></pre></div>
<div id="scatter-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.scatter({
  title: "Height vs Weight",
  xValues: [160, 165, 170, 172, 175, 178, 180, 182, 185, 190],
  yValues: [55, 60, 68, 65, 72, 75, 80, 78, 85, 90],
  gridlines: true,
  xLabel: "Height (cm)",
  yLabel: "Weight (kg)",
  showRegression: true,
});
chart.show();</code></pre></div>
<div id="scatter-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$scatter(
  title = "Height vs Weight",
  x_values = c(160, 165, 170, 172, 175, 178, 180, 182, 185, 190),
  y_values = c(55, 60, 68, 65, 72, 75, 80, 78, 85, 90),
  gridlines = TRUE,
  x_label = "Height (cm)",
  y_label = "Weight (kg)",
  show_regression = TRUE
)
chart$show()</code></pre></div>
<div id="scatter-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.scatter()
    .title("Height vs Weight")
    .xValues(List.of(160.0, 165.0, 170.0, 172.0, 175.0, 178.0, 180.0, 182.0, 185.0, 190.0))
    .yValues(List.of(55.0, 60.0, 68.0, 65.0, 72.0, 75.0, 80.0, 78.0, 85.0, 90.0))
    .gridlines(true)
    .xLabel("Height (cm)")
    .yLabel("Weight (kg)")
    .showRegression(true)
    .build();
chart.show();</code></pre></div>
<div id="scatter-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Scatter(
    title: "Height vs Weight",
    xValues: new[]{160.0, 165.0, 170.0, 172.0, 175.0, 178.0, 180.0, 182.0, 185.0, 190.0},
    yValues: new[]{55.0, 60.0, 68.0, 65.0, 72.0, 75.0, 80.0, 78.0, 85.0, 90.0},
    gridlines: true,
    xLabel: "Height (cm)",
    yLabel: "Weight (kg)",
    showRegression: true
);
chart.Show();</code></pre></div>
<div id="scatter-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.scatter(
  title = "Height vs Weight",
  x_values = List(160.0, 165.0, 170.0, 172.0, 175.0, 178.0, 180.0, 182.0, 185.0, 190.0),
  y_values = List(55.0, 60.0, 68.0, 65.0, 72.0, 75.0, 80.0, 78.0, 85.0, 90.0),
  gridlines = true,
  x_label = "Height (cm)",
  y_label = "Weight (kg)",
  show_regression = true
)
chart.show()</code></pre></div>
<div id="scatter-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::scatter({
  .title           = "Height vs Weight",
  .x_values        = {160, 165, 170, 172, 175, 178, 180, 182, 185, 190},
  .y_values        = {55, 60, 68, 65, 72, 75, 80, 78, 85, 90},
  .gridlines       = true,
  .x_label         = "Height (cm)",
  .y_label         = "Weight (kg)",
  .show_regression = true
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/scatter.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [bubble.md](bubble.md) — Scatter with a third numeric dimension encoded as point size
- [line.md](line.md) — Ordered points connected by line segments
- [kde.md](kde.md) — Kernel density estimate for continuous distributions

</div>

<div class="lang-fr">

## Signature

```python
sp.build_scatter_chart(
    title: str,
    x_values: list[float],
    y_values: list[float],
    *,
    color_hex: int = 0,
    show_text: bool = False,
    labels: list[str] | None = None,
    sizes: list[float] | None = None,
    color_groups: list[str] | None = None,
    show_regression: bool = False,
    regression_type: str = "linear",
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = False,
    background: str | None = None,
) -> Chart
```

Aliases: `sp.scatter`, `sp.scatter_chart`

---

## Description

Un graphique de dispersion représente chaque observation comme un point sur un plan cartésien à deux dimensions, rendant les corrélations, les clusters et les valeurs aberrantes immédiatement visibles. Chaque point peut encoder des dimensions supplémentaires via la taille (effet bulle) et les groupes de couleurs. La superposition de régression optionnelle — linéaire ou polynomiale — calcule la courbe de meilleur ajustement directement dans le moteur Rust, sans dépendance externe. SeraPlot gère des dizaines de milliers de points sans dégradation des performances.

**Idéal pour :**
- Visualiser la corrélation entre deux variables continues
- Détecter les clusters et les valeurs aberrantes dans les jeux de données
- Montrer des relations multidimensionnelles via la taille et la couleur

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | — | Titre du graphique affiché en haut |
| `x_values` | `list[float]` | — | Coordonnée X de chaque observation |
| `y_values` | `list[float]` | — | Coordonnée Y de chaque observation |
| `color_hex` | `int` | `0` | Couleur des points en hexadécimal ; `0` utilise la valeur du thème |
| `show_text` | `bool` | `False` | Afficher le label du point à côté de chaque point |
| `labels` | `list[str] \| None` | `None` | Label textuel optionnel pour chaque point |
| `sizes` | `list[float] \| None` | `None` | Multiplicateur de rayon par point pour un effet bulle |
| `color_groups` | `list[str] \| None` | `None` | Nom de groupe catégoriel par point ; les groupes sont colorés automatiquement |
| `show_regression` | `bool` | `False` | Superposer une courbe de régression de meilleur ajustement |
| `regression_type` | `str` | `"linear"` | Algorithme de régression : `"linear"` ou `"polynomial"` |
| `width` | `int` | `900` | Largeur du canevas en pixels |
| `height` | `int` | `480` | Hauteur du canevas en pixels |
| `x_label` | `str` | `""` | Label de l'axe X |
| `y_label` | `str` | `""` | Label de l'axe Y |
| `gridlines` | `bool` | `False` | Afficher les lignes de grille horizontales et verticales |
| `background` | `str \| None` | `None` | Couleur de fond CSS (ex. `"#1e293b"`) |

---

## Retourne

`Chart`

---

<div class="sp-tabs" id="scatter-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('scatter-fr','scatter-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('scatter-fr','scatter-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('scatter-fr','scatter-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('scatter-fr','scatter-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('scatter-fr','scatter-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('scatter-fr','scatter-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('scatter-fr','scatter-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('scatter-fr','scatter-fr-cpp',this)">C++</button>
</div>
<div id="scatter-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.scatter(
    title="Taille vs Poids",
    x_values=[160, 165, 170, 172, 175, 178, 180, 182, 185, 190],
    y_values=[55, 60, 68, 65, 72, 75, 80, 78, 85, 90],
    gridlines=True,
    x_label="Taille (cm)",
    y_label="Poids (kg)",
    show_regression=True,
)
chart.show()</code></pre></div>
<div id="scatter-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.scatter({
  title: "Taille vs Poids",
  xValues: [160, 165, 170, 172, 175, 178, 180, 182, 185, 190],
  yValues: [55, 60, 68, 65, 72, 75, 80, 78, 85, 90],
  gridlines: true,
  xLabel: "Taille (cm)",
  yLabel: "Poids (kg)",
  showRegression: true,
});
chart.show();</code></pre></div>
<div id="scatter-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.scatter({
  title: "Taille vs Poids",
  xValues: [160, 165, 170, 172, 175, 178, 180, 182, 185, 190],
  yValues: [55, 60, 68, 65, 72, 75, 80, 78, 85, 90],
  gridlines: true,
  xLabel: "Taille (cm)",
  yLabel: "Poids (kg)",
  showRegression: true,
});
chart.show();</code></pre></div>
<div id="scatter-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$scatter(
  title = "Taille vs Poids",
  x_values = c(160, 165, 170, 172, 175, 178, 180, 182, 185, 190),
  y_values = c(55, 60, 68, 65, 72, 75, 80, 78, 85, 90),
  gridlines = TRUE,
  x_label = "Taille (cm)",
  y_label = "Poids (kg)",
  show_regression = TRUE
)
chart$show()</code></pre></div>
<div id="scatter-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.scatter()
    .title("Taille vs Poids")
    .xValues(List.of(160.0, 165.0, 170.0, 172.0, 175.0, 178.0, 180.0, 182.0, 185.0, 190.0))
    .yValues(List.of(55.0, 60.0, 68.0, 65.0, 72.0, 75.0, 80.0, 78.0, 85.0, 90.0))
    .gridlines(true)
    .xLabel("Taille (cm)")
    .yLabel("Poids (kg)")
    .showRegression(true)
    .build();
chart.show();</code></pre></div>
<div id="scatter-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Scatter(
    title: "Taille vs Poids",
    xValues: new[]{160.0, 165.0, 170.0, 172.0, 175.0, 178.0, 180.0, 182.0, 185.0, 190.0},
    yValues: new[]{55.0, 60.0, 68.0, 65.0, 72.0, 75.0, 80.0, 78.0, 85.0, 90.0},
    gridlines: true,
    xLabel: "Taille (cm)",
    yLabel: "Poids (kg)",
    showRegression: true
);
chart.Show();</code></pre></div>
<div id="scatter-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.scatter(
  title = "Taille vs Poids",
  x_values = List(160.0, 165.0, 170.0, 172.0, 175.0, 178.0, 180.0, 182.0, 185.0, 190.0),
  y_values = List(55.0, 60.0, 68.0, 65.0, 72.0, 75.0, 80.0, 78.0, 85.0, 90.0),
  gridlines = true,
  x_label = "Taille (cm)",
  y_label = "Poids (kg)",
  show_regression = true
)
chart.show()</code></pre></div>
<div id="scatter-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::scatter({
  .title           = "Taille vs Poids",
  .x_values        = {160, 165, 170, 172, 175, 178, 180, 182, 185, 190},
  .y_values        = {55, 60, 68, 65, 72, 75, 80, 78, 85, 90},
  .gridlines       = true,
  .x_label         = "Taille (cm)",
  .y_label         = "Poids (kg)",
  .show_regression = true
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/scatter.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## Voir aussi

- [bubble.md](bubble.md) — Nuage de points avec une troisième dimension encodée en taille
- [line.md](line.md) — Points ordonnés reliés par des segments de ligne
- [kde.md](kde.md) — Estimation par noyau de densité pour les distributions continues

</div>
