# Heatmap

<div class="lang-en">

## Signature

```python
sp.build_heatmap(
    title: str,
    labels: list[str],
    flat_matrix: list[float],
    *,
    show_values: bool = True,
    color_low: int = 0,
    color_mid: int = 0,
    color_high: int = 0,
    col_labels: list[str] | None = None,
    width: int = 900,
    height: int = 480,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.heatmap`

---

## Description

A heatmap encodes the magnitude of a two-dimensional matrix as color intensity, enabling instant pattern recognition across rows and columns. `flat_matrix` must contain `n_rows × n_cols` values in row-major order, where `n_rows = len(labels)`. Column labels default to the same list as row labels (correlation matrix case) but can be overridden with `col_labels`. SeraPlot automatically normalizes values to [0, 1] for color mapping, applying a three-stop gradient from `color_low` through `color_mid` to `color_high`.

**Ideal for:**
- Visualizing correlation matrices and confusion matrices
- Comparing metrics across two categorical dimensions
- Showing time-of-day or day-of-week usage patterns

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | — | Chart title displayed at the top |
| `labels` | `list[str]` | — | Row labels; also used as column labels unless `col_labels` is provided |
| `flat_matrix` | `list[float]` | — | Matrix values in row-major order; length must equal `len(labels) × len(col_labels)` |
| `show_values` | `bool` | `True` | Display the numeric value inside each cell |
| `color_low` | `int` | `0` | Color for the minimum value (hex integer; `0` uses theme default) |
| `color_mid` | `int` | `0` | Color for the midpoint value (hex integer; `0` uses theme default) |
| `color_high` | `int` | `0` | Color for the maximum value (hex integer; `0` uses theme default) |
| `col_labels` | `list[str] \| None` | `None` | Column labels; defaults to the same list as `labels` |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `no_x_axis` | `bool` | `False` | Hide the X axis labels |
| `no_y_axis` | `bool` | `False` | Hide the Y axis labels |

---

## Returns

`Chart`

---

## Examples

### Correlation matrix

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="heatmap">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('heatmap','heatmap-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('heatmap','heatmap-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('heatmap','heatmap-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('heatmap','heatmap-r',this)">R</button>
<button class="sp-tb" onclick="spTab('heatmap','heatmap-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('heatmap','heatmap-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('heatmap','heatmap-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('heatmap','heatmap-cpp',this)">C++</button>
</div>
<div id="heatmap-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.heatmap(
    title="Correlation matrix",
    labels=["Price", "Volume", "Volatility", "Return"],
    flat_matrix=[
        1.0,  0.3, -0.2,  0.5,
        0.3,  1.0,  0.1,  0.2,
       -0.2,  0.1,  1.0, -0.3,
        0.5,  0.2, -0.3,  1.0,
    ],
)
chart.show()</code></pre></div>
<div id="heatmap-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.heatmap({
  title: "Correlation matrix",
  labels: ["Price", "Volume", "Volatility", "Return"],
  flatMatrix: [
     1.0,  0.3, -0.2,  0.5,
     0.3,  1.0,  0.1,  0.2,
    -0.2,  0.1,  1.0, -0.3,
     0.5,  0.2, -0.3,  1.0,
  ],
});
chart.show();</code></pre></div>
<div id="heatmap-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.heatmap({
  title: "Correlation matrix",
  labels: ["Price", "Volume", "Volatility", "Return"],
  flatMatrix: [
     1.0,  0.3, -0.2,  0.5,
     0.3,  1.0,  0.1,  0.2,
    -0.2,  0.1,  1.0, -0.3,
     0.5,  0.2, -0.3,  1.0,
  ],
});
chart.show();</code></pre></div>
<div id="heatmap-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$heatmap(
  title = "Correlation matrix",
  labels = c("Price", "Volume", "Volatility", "Return"),
  flat_matrix = c(
     1.0,  0.3, -0.2,  0.5,
     0.3,  1.0,  0.1,  0.2,
    -0.2,  0.1,  1.0, -0.3,
     0.5,  0.2, -0.3,  1.0
  )
)
chart$show()</code></pre></div>
<div id="heatmap-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.heatmap()
    .title("Correlation matrix")
    .labels(List.of("Price", "Volume", "Volatility", "Return"))
    .flatMatrix(List.of(
         1.0,  0.3, -0.2,  0.5,
         0.3,  1.0,  0.1,  0.2,
        -0.2,  0.1,  1.0, -0.3,
         0.5,  0.2, -0.3,  1.0
    ))
    .build();
chart.show();</code></pre></div>
<div id="heatmap-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Heatmap(
    title: "Correlation matrix",
    labels: new[]{"Price", "Volume", "Volatility", "Return"},
    flatMatrix: new[]{
         1.0,  0.3, -0.2,  0.5,
         0.3,  1.0,  0.1,  0.2,
        -0.2,  0.1,  1.0, -0.3,
         0.5,  0.2, -0.3,  1.0
    }
);
chart.Show();</code></pre></div>
<div id="heatmap-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.heatmap(
  title = "Correlation matrix",
  labels = List("Price", "Volume", "Volatility", "Return"),
  flat_matrix = List(
     1.0,  0.3, -0.2,  0.5,
     0.3,  1.0,  0.1,  0.2,
    -0.2,  0.1,  1.0, -0.3,
     0.5,  0.2, -0.3,  1.0
  )
)
chart.show()</code></pre></div>
<div id="heatmap-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::heatmap({
  .title       = "Correlation matrix",
  .labels      = {"Price", "Volume", "Volatility", "Return"},
  .flat_matrix = {
     1.0,  0.3, -0.2,  0.5,
     0.3,  1.0,  0.1,  0.2,
    -0.2,  0.1,  1.0, -0.3,
     0.5,  0.2, -0.3,  1.0
  }
});
chart.show();</code></pre></div>
</div>

<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>
<iframe src="../../previews/heatmap.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>
</details>

---

## See also

- [parallel.md](parallel.md) — Parallel coordinates for high-dimensional data
- [scatter.md](scatter.md) — Pairwise scatter for two variables

</div>

<div class="lang-fr">

## Signature

```python
sp.build_heatmap(
    title: str,
    labels: list[str],
    flat_matrix: list[float],
    *,
    show_values: bool = True,
    color_low: int = 0,
    color_mid: int = 0,
    color_high: int = 0,
    col_labels: list[str] | None = None,
    width: int = 900,
    height: int = 480,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.heatmap`

---

## Description

Une carte thermique encode la magnitude d'une matrice bidimensionnelle sous forme d'intensité de couleur, permettant une reconnaissance instantanée des motifs entre lignes et colonnes. `flat_matrix` doit contenir `n_lignes × n_colonnes` valeurs dans l'ordre ligne par ligne, où `n_lignes = len(labels)`. Les labels de colonnes correspondent par défaut à la même liste que les labels de lignes (cas de la matrice de corrélation) mais peuvent être remplacés par `col_labels`. SeraPlot normalise automatiquement les valeurs vers [0, 1] pour le mappage de couleurs, appliquant un dégradé en trois points de `color_low` à `color_mid` puis `color_high`.

**Idéal pour :**
- Visualiser les matrices de corrélation et de confusion
- Comparer des métriques sur deux dimensions catégorielles
- Afficher les patterns d'utilisation par heure de la journée ou jour de la semaine

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | — | Titre du graphique affiché en haut |
| `labels` | `list[str]` | — | Labels de lignes ; utilisés aussi comme labels de colonnes sauf si `col_labels` est fourni |
| `flat_matrix` | `list[float]` | — | Valeurs de la matrice en ordre ligne par ligne ; longueur = `len(labels) × len(col_labels)` |
| `show_values` | `bool` | `True` | Afficher la valeur numérique à l'intérieur de chaque cellule |
| `color_low` | `int` | `0` | Couleur pour la valeur minimale (hexadécimal ; `0` utilise le thème) |
| `color_mid` | `int` | `0` | Couleur pour la valeur médiane (hexadécimal ; `0` utilise le thème) |
| `color_high` | `int` | `0` | Couleur pour la valeur maximale (hexadécimal ; `0` utilise le thème) |
| `col_labels` | `list[str] \| None` | `None` | Labels de colonnes ; par défaut identiques aux `labels` |
| `width` | `int` | `900` | Largeur du canevas en pixels |
| `height` | `int` | `480` | Hauteur du canevas en pixels |
| `no_x_axis` | `bool` | `False` | Masquer les labels de l'axe X |
| `no_y_axis` | `bool` | `False` | Masquer les labels de l'axe Y |

---

## Retourne

`Chart`

---

## Exemples

### Matrice de corrélation

<div class="sp-tabs" id="heatmap-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('heatmap-fr','heatmap-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('heatmap-fr','heatmap-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('heatmap-fr','heatmap-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('heatmap-fr','heatmap-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('heatmap-fr','heatmap-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('heatmap-fr','heatmap-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('heatmap-fr','heatmap-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('heatmap-fr','heatmap-fr-cpp',this)">C++</button>
</div>
<div id="heatmap-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.heatmap(
    title="Matrice de corrélation",
    labels=["Prix", "Volume", "Volatilité", "Rendement"],
    flat_matrix=[
        1.0,  0.3, -0.2,  0.5,
        0.3,  1.0,  0.1,  0.2,
       -0.2,  0.1,  1.0, -0.3,
        0.5,  0.2, -0.3,  1.0,
    ],
)
chart.show()</code></pre></div>
<div id="heatmap-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.heatmap({
  title: "Matrice de corrélation",
  labels: ["Prix", "Volume", "Volatilité", "Rendement"],
  flatMatrix: [
     1.0,  0.3, -0.2,  0.5,
     0.3,  1.0,  0.1,  0.2,
    -0.2,  0.1,  1.0, -0.3,
     0.5,  0.2, -0.3,  1.0,
  ],
});
chart.show();</code></pre></div>
<div id="heatmap-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.heatmap({
  title: "Matrice de corrélation",
  labels: ["Prix", "Volume", "Volatilité", "Rendement"],
  flatMatrix: [
     1.0,  0.3, -0.2,  0.5,
     0.3,  1.0,  0.1,  0.2,
    -0.2,  0.1,  1.0, -0.3,
     0.5,  0.2, -0.3,  1.0,
  ],
});
chart.show();</code></pre></div>
<div id="heatmap-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$heatmap(
  title = "Matrice de corrélation",
  labels = c("Prix", "Volume", "Volatilité", "Rendement"),
  flat_matrix = c(
     1.0,  0.3, -0.2,  0.5,
     0.3,  1.0,  0.1,  0.2,
    -0.2,  0.1,  1.0, -0.3,
     0.5,  0.2, -0.3,  1.0
  )
)
chart$show()</code></pre></div>
<div id="heatmap-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.heatmap()
    .title("Matrice de corrélation")
    .labels(List.of("Prix", "Volume", "Volatilité", "Rendement"))
    .flatMatrix(List.of(
         1.0,  0.3, -0.2,  0.5,
         0.3,  1.0,  0.1,  0.2,
        -0.2,  0.1,  1.0, -0.3,
         0.5,  0.2, -0.3,  1.0
    ))
    .build();
chart.show();</code></pre></div>
<div id="heatmap-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Heatmap(
    title: "Matrice de corrélation",
    labels: new[]{"Prix", "Volume", "Volatilité", "Rendement"},
    flatMatrix: new[]{
         1.0,  0.3, -0.2,  0.5,
         0.3,  1.0,  0.1,  0.2,
        -0.2,  0.1,  1.0, -0.3,
         0.5,  0.2, -0.3,  1.0
    }
);
chart.Show();</code></pre></div>
<div id="heatmap-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.heatmap(
  title = "Matrice de corrélation",
  labels = List("Prix", "Volume", "Volatilité", "Rendement"),
  flat_matrix = List(
     1.0,  0.3, -0.2,  0.5,
     0.3,  1.0,  0.1,  0.2,
    -0.2,  0.1,  1.0, -0.3,
     0.5,  0.2, -0.3,  1.0
  )
)
chart.show()</code></pre></div>
<div id="heatmap-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::heatmap({
  .title       = "Matrice de corrélation",
  .labels      = {"Prix", "Volume", "Volatilité", "Rendement"},
  .flat_matrix = {
     1.0,  0.3, -0.2,  0.5,
     0.3,  1.0,  0.1,  0.2,
    -0.2,  0.1,  1.0, -0.3,
     0.5,  0.2, -0.3,  1.0
  }
});
chart.show();</code></pre></div>
</div>

---

## Voir aussi

- [parallel.md](parallel.md) — Coordonnées parallèles pour les données haute dimension
- [scatter.md](scatter.md) — Nuage de points pour deux variables

</div>
