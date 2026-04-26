# Bubble Chart

<div class="lang-en">

## Signature

```python
sp.build_bubble(
    title: str,
    x_values: list[float],
    y_values: list[float],
    sizes: list[float],
    *,
    labels: list[str] | None = None,
    color_groups: list[str] | None = None,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    background: str | None = None,
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.bubble`

---

## Description

A bubble chart extends a scatter plot with a third numeric dimension represented as bubble **area** (not radius), following best practices for perceptual accuracy. Each point is defined by its X position, Y position, and size; optional string labels are shown on hover, and optional `color_groups` partition bubbles into named series with automatic palette assignment. SeraPlot normalizes raw size values to a visually pleasing pixel range automatically.

**Ideal for:**
- Three-variable relationships (e.g., GDP × life expectancy × population)
- Portfolio analysis (risk × return × allocation weight)
- Market segmentation (revenue × growth × market share)

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | — | Chart title displayed at the top |
| `x_values` | `list[float]` | — | Horizontal position of each bubble |
| `y_values` | `list[float]` | — | Vertical position of each bubble |
| `sizes` | `list[float]` | — | Raw size values mapped to bubble area |
| `labels` | `list[str] \| None` | `None` | Per-point label shown on hover |
| `color_groups` | `list[str] \| None` | `None` | Group name per point for color partitioning |
| `color_hex` | `int` | `0x6366F1` | Fallback color when no palette or groups are provided |
| `palette` | `list[int] \| None` | `None` | Custom group colors as hex integers |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `x_label` | `str` | `""` | Label for the X axis |
| `y_label` | `str` | `""` | Label for the Y axis |
| `gridlines` | `bool` | `True` | Draw gridlines |
| `background` | `str \| None` | `None` | CSS background color override |
| `hover_json` | `str \| None` | `None` | JSON string with extra tooltip metadata |

---

## Returns

`Chart`

---

## Examples

### Countries: GDP vs Life expectancy

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="bubble">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bubble','bubble-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bubble','bubble-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bubble','bubble-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bubble','bubble-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bubble','bubble-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bubble','bubble-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bubble','bubble-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bubble','bubble-cpp',this)">C++</button>
</div>
<div id="bubble-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.bubble(
    title="Countries: GDP vs Life expectancy",
    x_values=[2.5, 5.0, 12.0, 18.5, 25.0, 3.2, 8.0],
    y_values=[72, 75, 80, 82, 79, 68, 77],
    sizes=[500, 1200, 3000, 4500, 8000, 800, 2100],
    labels=["Brazil", "Mexico", "Germany", "Japan", "USA", "Nigeria", "Turkey"],
    color_groups=["Americas", "Americas", "Europe", "Asia", "Americas", "Africa", "Europe"],
    x_label="GDP per capita (k$)",
    y_label="Life expectancy (years)",
)
chart.show()</code></pre></div>
<div id="bubble-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.bubble({
  title: "Countries: GDP vs Life expectancy",
  xValues: [2.5, 5.0, 12.0, 18.5, 25.0, 3.2, 8.0],
  yValues: [72, 75, 80, 82, 79, 68, 77],
  sizes: [500, 1200, 3000, 4500, 8000, 800, 2100],
  labels: ["Brazil", "Mexico", "Germany", "Japan", "USA", "Nigeria", "Turkey"],
  colorGroups: ["Americas", "Americas", "Europe", "Asia", "Americas", "Africa", "Europe"],
  xLabel: "GDP per capita (k$)",
  yLabel: "Life expectancy (years)",
});
chart.show();</code></pre></div>
<div id="bubble-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.bubble({
  title: "Countries: GDP vs Life expectancy",
  xValues: [2.5, 5.0, 12.0, 18.5, 25.0, 3.2, 8.0],
  yValues: [72, 75, 80, 82, 79, 68, 77],
  sizes: [500, 1200, 3000, 4500, 8000, 800, 2100],
  labels: ["Brazil", "Mexico", "Germany", "Japan", "USA", "Nigeria", "Turkey"],
  colorGroups: ["Americas", "Americas", "Europe", "Asia", "Americas", "Africa", "Europe"],
  xLabel: "GDP per capita (k$)",
  yLabel: "Life expectancy (years)",
});
chart.show();</code></pre></div>
<div id="bubble-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$bubble(
  title = "Countries: GDP vs Life expectancy",
  x_values = c(2.5, 5.0, 12.0, 18.5, 25.0, 3.2, 8.0),
  y_values = c(72, 75, 80, 82, 79, 68, 77),
  sizes = c(500, 1200, 3000, 4500, 8000, 800, 2100),
  labels = c("Brazil", "Mexico", "Germany", "Japan", "USA", "Nigeria", "Turkey"),
  color_groups = c("Americas", "Americas", "Europe", "Asia", "Americas", "Africa", "Europe"),
  x_label = "GDP per capita (k$)",
  y_label = "Life expectancy (years)"
)
chart$show()</code></pre></div>
<div id="bubble-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bubble()
    .title("Countries: GDP vs Life expectancy")
    .xValues(List.of(2.5, 5.0, 12.0, 18.5, 25.0, 3.2, 8.0))
    .yValues(List.of(72.0, 75.0, 80.0, 82.0, 79.0, 68.0, 77.0))
    .sizes(List.of(500.0, 1200.0, 3000.0, 4500.0, 8000.0, 800.0, 2100.0))
    .labels(List.of("Brazil", "Mexico", "Germany", "Japan", "USA", "Nigeria", "Turkey"))
    .colorGroups(List.of("Americas", "Americas", "Europe", "Asia", "Americas", "Africa", "Europe"))
    .xLabel("GDP per capita (k$)")
    .yLabel("Life expectancy (years)")
    .build();
chart.show();</code></pre></div>
<div id="bubble-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bubble(
    title: "Countries: GDP vs Life expectancy",
    xValues: new[]{2.5, 5.0, 12.0, 18.5, 25.0, 3.2, 8.0},
    yValues: new[]{72.0, 75.0, 80.0, 82.0, 79.0, 68.0, 77.0},
    sizes: new[]{500.0, 1200.0, 3000.0, 4500.0, 8000.0, 800.0, 2100.0},
    labels: new[]{"Brazil", "Mexico", "Germany", "Japan", "USA", "Nigeria", "Turkey"},
    colorGroups: new[]{"Americas", "Americas", "Europe", "Asia", "Americas", "Africa", "Europe"},
    xLabel: "GDP per capita (k$)",
    yLabel: "Life expectancy (years)"
);
chart.Show();</code></pre></div>
<div id="bubble-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.bubble(
  title = "Countries: GDP vs Life expectancy",
  x_values = List(2.5, 5.0, 12.0, 18.5, 25.0, 3.2, 8.0),
  y_values = List(72, 75, 80, 82, 79, 68, 77),
  sizes = List(500, 1200, 3000, 4500, 8000, 800, 2100),
  labels = List("Brazil", "Mexico", "Germany", "Japan", "USA", "Nigeria", "Turkey"),
  color_groups = List("Americas", "Americas", "Europe", "Asia", "Americas", "Africa", "Europe"),
  x_label = "GDP per capita (k$)",
  y_label = "Life expectancy (years)"
)
chart.show()</code></pre></div>
<div id="bubble-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bubble({
  .title        = "Countries: GDP vs Life expectancy",
  .x_values     = {2.5, 5.0, 12.0, 18.5, 25.0, 3.2, 8.0},
  .y_values     = {72, 75, 80, 82, 79, 68, 77},
  .sizes        = {500, 1200, 3000, 4500, 8000, 800, 2100},
  .labels       = {"Brazil", "Mexico", "Germany", "Japan", "USA", "Nigeria", "Turkey"},
  .color_groups = {"Americas", "Americas", "Europe", "Asia", "Americas", "Africa", "Europe"},
  .x_label      = "GDP per capita (k$)",
  .y_label      = "Life expectancy (years)"
});
chart.show();</code></pre></div>
</div>

<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>
<iframe src="../../previews/bubble.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>
</details>

---

## See also

- [scatter.md](scatter.md) — Two-variable scatter without size encoding
- [parallel.md](parallel.md) — High-dimensional data across parallel axes

</div>

<div class="lang-fr">

## Signature

```python
sp.build_bubble(
    title: str,
    x_values: list[float],
    y_values: list[float],
    sizes: list[float],
    *,
    labels: list[str] | None = None,
    color_groups: list[str] | None = None,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    background: str | None = None,
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.bubble`

---

## Description

Un graphique à bulles étend un nuage de points avec une troisième dimension numérique représentée comme une **aire** de bulle (et non un rayon), conformément aux meilleures pratiques de précision perceptive. Chaque point est défini par sa position X, sa position Y et sa taille ; les labels textuels optionnels sont affichés au survol, et les `color_groups` optionnels partitionnent les bulles en séries nommées avec attribution automatique de palette. SeraPlot normalise les valeurs de taille brutes vers une plage de pixels visuellement agréable automatiquement.

**Idéal pour :**
- Relations à trois variables (ex. PIB × espérance de vie × population)
- Analyse de portefeuille (risque × rendement × poids d'allocation)
- Segmentation de marché (revenus × croissance × part de marché)

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | — | Titre du graphique affiché en haut |
| `x_values` | `list[float]` | — | Position horizontale de chaque bulle |
| `y_values` | `list[float]` | — | Position verticale de chaque bulle |
| `sizes` | `list[float]` | — | Valeurs brutes de taille mappées à l'aire des bulles |
| `labels` | `list[str] \| None` | `None` | Label par point affiché au survol |
| `color_groups` | `list[str] \| None` | `None` | Nom de groupe par point pour le partitionnement des couleurs |
| `color_hex` | `int` | `0x6366F1` | Couleur de secours quand aucune palette ou groupe n'est fourni |
| `palette` | `list[int] \| None` | `None` | Couleurs de groupe personnalisées en entiers hexadécimaux |
| `width` | `int` | `900` | Largeur du canevas en pixels |
| `height` | `int` | `480` | Hauteur du canevas en pixels |
| `x_label` | `str` | `""` | Label de l'axe X |
| `y_label` | `str` | `""` | Label de l'axe Y |
| `gridlines` | `bool` | `True` | Afficher des lignes de grille |
| `background` | `str \| None` | `None` | Couleur de fond CSS |
| `hover_json` | `str \| None` | `None` | Chaîne JSON avec des métadonnées supplémentaires pour les infobulles |

---

## Retourne

`Chart`

---

## Exemples

### Pays : PIB vs Espérance de vie

<div class="sp-tabs" id="bubble-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bubble-fr','bubble-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bubble-fr','bubble-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bubble-fr','bubble-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bubble-fr','bubble-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bubble-fr','bubble-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bubble-fr','bubble-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bubble-fr','bubble-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bubble-fr','bubble-fr-cpp',this)">C++</button>
</div>
<div id="bubble-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.bubble(
    title="Pays : PIB vs Espérance de vie",
    x_values=[2.5, 5.0, 12.0, 18.5, 25.0, 3.2, 8.0],
    y_values=[72, 75, 80, 82, 79, 68, 77],
    sizes=[500, 1200, 3000, 4500, 8000, 800, 2100],
    labels=["Brésil", "Mexique", "Allemagne", "Japon", "États-Unis", "Nigeria", "Turquie"],
    color_groups=["Amériques", "Amériques", "Europe", "Asie", "Amériques", "Afrique", "Europe"],
    x_label="PIB par habitant (k$)",
    y_label="Espérance de vie (années)",
)
chart.show()</code></pre></div>
<div id="bubble-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.bubble({
  title: "Pays : PIB vs Espérance de vie",
  xValues: [2.5, 5.0, 12.0, 18.5, 25.0, 3.2, 8.0],
  yValues: [72, 75, 80, 82, 79, 68, 77],
  sizes: [500, 1200, 3000, 4500, 8000, 800, 2100],
  labels: ["Brésil", "Mexique", "Allemagne", "Japon", "États-Unis", "Nigeria", "Turquie"],
  colorGroups: ["Amériques", "Amériques", "Europe", "Asie", "Amériques", "Afrique", "Europe"],
  xLabel: "PIB par habitant (k$)",
  yLabel: "Espérance de vie (années)",
});
chart.show();</code></pre></div>
<div id="bubble-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.bubble({
  title: "Pays : PIB vs Espérance de vie",
  xValues: [2.5, 5.0, 12.0, 18.5, 25.0, 3.2, 8.0],
  yValues: [72, 75, 80, 82, 79, 68, 77],
  sizes: [500, 1200, 3000, 4500, 8000, 800, 2100],
  labels: ["Brésil", "Mexique", "Allemagne", "Japon", "États-Unis", "Nigeria", "Turquie"],
  colorGroups: ["Amériques", "Amériques", "Europe", "Asie", "Amériques", "Afrique", "Europe"],
  xLabel: "PIB par habitant (k$)",
  yLabel: "Espérance de vie (années)",
});
chart.show();</code></pre></div>
<div id="bubble-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$bubble(
  title = "Pays : PIB vs Espérance de vie",
  x_values = c(2.5, 5.0, 12.0, 18.5, 25.0, 3.2, 8.0),
  y_values = c(72, 75, 80, 82, 79, 68, 77),
  sizes = c(500, 1200, 3000, 4500, 8000, 800, 2100),
  labels = c("Brésil", "Mexique", "Allemagne", "Japon", "États-Unis", "Nigeria", "Turquie"),
  color_groups = c("Amériques", "Amériques", "Europe", "Asie", "Amériques", "Afrique", "Europe"),
  x_label = "PIB par habitant (k$)",
  y_label = "Espérance de vie (années)"
)
chart$show()</code></pre></div>
<div id="bubble-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bubble()
    .title("Pays : PIB vs Espérance de vie")
    .xValues(List.of(2.5, 5.0, 12.0, 18.5, 25.0, 3.2, 8.0))
    .yValues(List.of(72.0, 75.0, 80.0, 82.0, 79.0, 68.0, 77.0))
    .sizes(List.of(500.0, 1200.0, 3000.0, 4500.0, 8000.0, 800.0, 2100.0))
    .labels(List.of("Brésil", "Mexique", "Allemagne", "Japon", "États-Unis", "Nigeria", "Turquie"))
    .colorGroups(List.of("Amériques", "Amériques", "Europe", "Asie", "Amériques", "Afrique", "Europe"))
    .xLabel("PIB par habitant (k$)")
    .yLabel("Espérance de vie (années)")
    .build();
chart.show();</code></pre></div>
<div id="bubble-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bubble(
    title: "Pays : PIB vs Espérance de vie",
    xValues: new[]{2.5, 5.0, 12.0, 18.5, 25.0, 3.2, 8.0},
    yValues: new[]{72.0, 75.0, 80.0, 82.0, 79.0, 68.0, 77.0},
    sizes: new[]{500.0, 1200.0, 3000.0, 4500.0, 8000.0, 800.0, 2100.0},
    labels: new[]{"Brésil", "Mexique", "Allemagne", "Japon", "États-Unis", "Nigeria", "Turquie"},
    colorGroups: new[]{"Amériques", "Amériques", "Europe", "Asie", "Amériques", "Afrique", "Europe"},
    xLabel: "PIB par habitant (k$)",
    yLabel: "Espérance de vie (années)"
);
chart.Show();</code></pre></div>
<div id="bubble-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.bubble(
  title = "Pays : PIB vs Espérance de vie",
  x_values = List(2.5, 5.0, 12.0, 18.5, 25.0, 3.2, 8.0),
  y_values = List(72, 75, 80, 82, 79, 68, 77),
  sizes = List(500, 1200, 3000, 4500, 8000, 800, 2100),
  labels = List("Brésil", "Mexique", "Allemagne", "Japon", "États-Unis", "Nigeria", "Turquie"),
  color_groups = List("Amériques", "Amériques", "Europe", "Asie", "Amériques", "Afrique", "Europe"),
  x_label = "PIB par habitant (k$)",
  y_label = "Espérance de vie (années)"
)
chart.show()</code></pre></div>
<div id="bubble-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bubble({
  .title        = "Pays : PIB vs Espérance de vie",
  .x_values     = {2.5, 5.0, 12.0, 18.5, 25.0, 3.2, 8.0},
  .y_values     = {72, 75, 80, 82, 79, 68, 77},
  .sizes        = {500, 1200, 3000, 4500, 8000, 800, 2100},
  .labels       = {"Brésil", "Mexique", "Allemagne", "Japon", "États-Unis", "Nigeria", "Turquie"},
  .color_groups = {"Amériques", "Amériques", "Europe", "Asie", "Amériques", "Afrique", "Europe"},
  .x_label      = "PIB par habitant (k$)",
  .y_label      = "Espérance de vie (années)"
});
chart.show();</code></pre></div>
</div>

---

## Voir aussi

- [scatter.md](scatter.md) — Nuage de points à deux variables sans encodage de taille
- [parallel.md](parallel.md) — Données haute dimension sur des axes parallèles

</div>
