# Dumbbell Chart

<div class="lang-en">

## Signature

```python
sp.build_dumbbell(
    title: str,
    labels: list[str],
    values_start: list[float],
    values_end: list[float],
    *,
    show_text: bool = True,
    color_start: int = 0x6366f1,
    color_end: int = 0xf43f5e,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    gridlines: bool = True,
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.dumbbell`

---

## Description

A dumbbell chart (also called dot plot or gap chart) draws a horizontal line connecting two data points per category, making the magnitude of change between two states immediately visible. The left endpoint (`values_start`) is colored differently from the right endpoint (`values_end`), creating a two-tone dumbbell shape. It is superior to paired bar charts for comparing before/after data because the visual gap is the primary encoding rather than bar area. Labels on both endpoints are shown by default via `show_text=True`.

**Ideal for:**
- Before/after comparisons (pre vs post treatment, 2020 vs 2024 values)
- Showing the range and direction of change per category simultaneously
- Ranking categories by the size of their gap

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Category labels (one per dumbbell) |
| `values_start` | `list[float]` | required | Left endpoint values |
| `values_end` | `list[float]` | required | Right endpoint values |
| `show_text` | `bool` | `True` | Show value labels on both endpoints |
| `color_start` | `int` | `0x6366f1` | Color of the start (left) dot |
| `color_end` | `int` | `0xf43f5e` | Color of the end (right) dot |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `x_label` | `str` | `""` | X-axis label |
| `gridlines` | `bool` | `True` | Show gridlines |
| `hover_json` | `str \| None` | `None` | Custom hover tooltip JSON |

---

## Returns

`Chart`

---

## Examples

### Life expectancy 1990 vs 2023

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="dumbbell">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('dumbbell','dumbbell-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('dumbbell','dumbbell-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('dumbbell','dumbbell-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('dumbbell','dumbbell-r',this)">R</button>
<button class="sp-tb" onclick="spTab('dumbbell','dumbbell-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('dumbbell','dumbbell-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('dumbbell','dumbbell-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('dumbbell','dumbbell-cpp',this)">C++</button>
</div>
<div id="dumbbell-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

countries = ["Japan", "Australia", "Germany", "USA", "Brazil", "China", "India"]
le_1990 = [78.9, 77.1, 75.3, 75.3, 66.6, 69.3, 58.3]
le_2023 = [84.3, 83.5, 81.2, 78.8, 75.9, 77.4, 70.8]

chart = sp.dumbbell(
    title="Life Expectancy 1990 vs 2023",
    labels=countries,
    values_start=le_1990,
    values_end=le_2023,
    x_label="Years",
)
chart.show()</code></pre></div>
<div id="dumbbell-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const countries = ["Japan", "Australia", "Germany", "USA", "Brazil", "China", "India"];
const le1990 = [78.9, 77.1, 75.3, 75.3, 66.6, 69.3, 58.3];
const le2023 = [84.3, 83.5, 81.2, 78.8, 75.9, 77.4, 70.8];

const chart = sp.dumbbell({
  title: "Life Expectancy 1990 vs 2023",
  labels: countries,
  valuesStart: le1990,
  valuesEnd: le2023,
  xLabel: "Years",
});
chart.show();</code></pre></div>
<div id="dumbbell-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const countries: string[] = ["Japan", "Australia", "Germany", "USA", "Brazil", "China", "India"];
const le1990: number[] = [78.9, 77.1, 75.3, 75.3, 66.6, 69.3, 58.3];
const le2023: number[] = [84.3, 83.5, 81.2, 78.8, 75.9, 77.4, 70.8];

const chart = sp.dumbbell({
  title: "Life Expectancy 1990 vs 2023",
  labels: countries,
  valuesStart: le1990,
  valuesEnd: le2023,
  xLabel: "Years",
});
chart.show();</code></pre></div>
<div id="dumbbell-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

pays    <- c("Japan", "Australia", "Germany", "USA", "Brazil", "China", "India")
le_1990 <- c(78.9, 77.1, 75.3, 75.3, 66.6, 69.3, 58.3)
le_2023 <- c(84.3, 83.5, 81.2, 78.8, 75.9, 77.4, 70.8)

chart <- sp$dumbbell(
  title        = "Life Expectancy 1990 vs 2023",
  labels       = pays,
  values_start = le_1990,
  values_end   = le_2023,
  x_label      = "Years"
)
chart$show()</code></pre></div>
<div id="dumbbell-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.dumbbell()
    .title("Life Expectancy 1990 vs 2023")
    .labels(List.of("Japan", "Australia", "Germany", "USA", "Brazil", "China", "India"))
    .valuesStart(List.of(78.9, 77.1, 75.3, 75.3, 66.6, 69.3, 58.3))
    .valuesEnd(List.of(84.3, 83.5, 81.2, 78.8, 75.9, 77.4, 70.8))
    .xLabel("Years")
    .build();
chart.show();</code></pre></div>
<div id="dumbbell-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Dumbbell(
    title:       "Life Expectancy 1990 vs 2023",
    labels:      ["Japan", "Australia", "Germany", "USA", "Brazil", "China", "India"],
    valuesStart: [78.9, 77.1, 75.3, 75.3, 66.6, 69.3, 58.3],
    valuesEnd:   [84.3, 83.5, 81.2, 78.8, 75.9, 77.4, 70.8],
    xLabel:      "Years"
);
chart.Show();</code></pre></div>
<div id="dumbbell-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.dumbbell(
  title        = "Life Expectancy 1990 vs 2023",
  labels       = List("Japan", "Australia", "Germany", "USA", "Brazil", "China", "India"),
  values_start = List(78.9, 77.1, 75.3, 75.3, 66.6, 69.3, 58.3),
  values_end   = List(84.3, 83.5, 81.2, 78.8, 75.9, 77.4, 70.8),
  x_label      = "Years"
)
chart.show()</code></pre></div>
<div id="dumbbell-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::dumbbell({
    .title        = "Life Expectancy 1990 vs 2023",
    .labels       = {"Japan", "Australia", "Germany", "USA", "Brazil", "China", "India"},
    .values_start = {78.9, 77.1, 75.3, 75.3, 66.6, 69.3, 58.3},
    .values_end   = {84.3, 83.5, 81.2, 78.8, 75.9, 77.4, 70.8},
    .x_label      = "Years",
});
chart.show();</code></pre></div>
</div>

<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>
<iframe src="../../previews/dumbbell.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>
</details>

### Custom endpoint colors

```python
chart = sp.dumbbell(
    title="Test Scores: Before vs After Training",
    labels=["Alice", "Bob", "Carol", "Dave", "Eve"],
    values_start=[62, 54, 71, 48, 65],
    values_end=[78, 74, 85, 70, 82],
    color_start=0xf59e0b,
    color_end=0x10b981,
    x_label="Score",
)
```

---

## See also

- [Bar Chart](bar.md) — `sp.build_bar_chart()`
- [Slope Chart](slope.md) — `sp.build_slope()`
- [Horizontal Bar](hbar.md) — `sp.build_hbar()`

</div>

<div class="lang-fr">

## Signature

```python
sp.build_dumbbell(
    title: str,
    labels: list[str],
    values_start: list[float],
    values_end: list[float],
    *,
    show_text: bool = True,
    color_start: int = 0x6366f1,
    color_end: int = 0xf43f5e,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    gridlines: bool = True,
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.dumbbell`

---

## Description

Un graphique haltère (dumbbell chart, aussi appelé dot plot ou gap chart) trace une ligne horizontale reliant deux points de données par catégorie, rendant immédiatement visible l'amplitude du changement entre deux états. L'extrémité gauche (`values_start`) est colorée différemment de l'extrémité droite (`values_end`), créant une forme en haltère bicolore. Il est supérieur aux graphiques en barres appariées pour les comparaisons avant/après car l'écart visuel est l'encodage principal. Les étiquettes sur les deux extrémités sont affichées par défaut via `show_text=True`.

**Idéal pour :**
- Les comparaisons avant/après (pré vs post-traitement, 2020 vs 2024)
- Montrer l'amplitude et la direction du changement par catégorie
- Classer les catégories par taille d'écart

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Étiquettes des catégories (un haltère par entrée) |
| `values_start` | `list[float]` | requis | Valeurs de l'extrémité gauche |
| `values_end` | `list[float]` | requis | Valeurs de l'extrémité droite |
| `show_text` | `bool` | `True` | Afficher les étiquettes sur les deux extrémités |
| `color_start` | `int` | `0x6366f1` | Couleur du point de départ (gauche) |
| `color_end` | `int` | `0xf43f5e` | Couleur du point d'arrivée (droite) |
| `width` | `int` | `900` | Largeur du canvas en pixels |
| `height` | `int` | `480` | Hauteur du canvas en pixels |
| `x_label` | `str` | `""` | Étiquette de l'axe X |
| `gridlines` | `bool` | `True` | Afficher les lignes de grille |
| `hover_json` | `str \| None` | `None` | JSON d'infobulle personnalisée |

---

## Retourne

`Chart`

---

## Exemples

### Espérance de vie 1990 vs 2023

<div class="sp-tabs" id="dumbbell-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('dumbbell-fr','dumbbell-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr','dumbbell-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr','dumbbell-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr','dumbbell-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr','dumbbell-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr','dumbbell-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr','dumbbell-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('dumbbell-fr','dumbbell-fr-cpp',this)">C++</button>
</div>
<div id="dumbbell-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

pays    = ["Japon", "Australie", "Allemagne", "États-Unis", "Brésil", "Chine", "Inde"]
esp_1990 = [78.9, 77.1, 75.3, 75.3, 66.6, 69.3, 58.3]
esp_2023 = [84.3, 83.5, 81.2, 78.8, 75.9, 77.4, 70.8]

chart = sp.dumbbell(
    title="Espérance de vie 1990 vs 2023",
    labels=pays,
    values_start=esp_1990,
    values_end=esp_2023,
    x_label="Années",
)
chart.show()</code></pre></div>
<div id="dumbbell-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const pays    = ["Japon", "Australie", "Allemagne", "États-Unis", "Brésil", "Chine", "Inde"];
const esp1990 = [78.9, 77.1, 75.3, 75.3, 66.6, 69.3, 58.3];
const esp2023 = [84.3, 83.5, 81.2, 78.8, 75.9, 77.4, 70.8];

const chart = sp.dumbbell({
  title: "Espérance de vie 1990 vs 2023",
  labels: pays,
  valuesStart: esp1990,
  valuesEnd: esp2023,
  xLabel: "Années",
});
chart.show();</code></pre></div>
<div id="dumbbell-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const pays: string[]    = ["Japon", "Australie", "Allemagne", "États-Unis", "Brésil", "Chine", "Inde"];
const esp1990: number[] = [78.9, 77.1, 75.3, 75.3, 66.6, 69.3, 58.3];
const esp2023: number[] = [84.3, 83.5, 81.2, 78.8, 75.9, 77.4, 70.8];

const chart = sp.dumbbell({
  title: "Espérance de vie 1990 vs 2023",
  labels: pays,
  valuesStart: esp1990,
  valuesEnd: esp2023,
  xLabel: "Années",
});
chart.show();</code></pre></div>
<div id="dumbbell-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

pays     <- c("Japon", "Australie", "Allemagne", "États-Unis", "Brésil", "Chine", "Inde")
esp_1990 <- c(78.9, 77.1, 75.3, 75.3, 66.6, 69.3, 58.3)
esp_2023 <- c(84.3, 83.5, 81.2, 78.8, 75.9, 77.4, 70.8)

chart <- sp$dumbbell(
  title        = "Espérance de vie 1990 vs 2023",
  labels       = pays,
  values_start = esp_1990,
  values_end   = esp_2023,
  x_label      = "Années"
)
chart$show()</code></pre></div>
<div id="dumbbell-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.dumbbell()
    .title("Espérance de vie 1990 vs 2023")
    .labels(List.of("Japon", "Australie", "Allemagne", "États-Unis", "Brésil", "Chine", "Inde"))
    .valuesStart(List.of(78.9, 77.1, 75.3, 75.3, 66.6, 69.3, 58.3))
    .valuesEnd(List.of(84.3, 83.5, 81.2, 78.8, 75.9, 77.4, 70.8))
    .xLabel("Années")
    .build();
chart.show();</code></pre></div>
<div id="dumbbell-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Dumbbell(
    title:       "Espérance de vie 1990 vs 2023",
    labels:      ["Japon", "Australie", "Allemagne", "États-Unis", "Brésil", "Chine", "Inde"],
    valuesStart: [78.9, 77.1, 75.3, 75.3, 66.6, 69.3, 58.3],
    valuesEnd:   [84.3, 83.5, 81.2, 78.8, 75.9, 77.4, 70.8],
    xLabel:      "Années"
);
chart.Show();</code></pre></div>
<div id="dumbbell-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.dumbbell(
  title        = "Espérance de vie 1990 vs 2023",
  labels       = List("Japon", "Australie", "Allemagne", "États-Unis", "Brésil", "Chine", "Inde"),
  values_start = List(78.9, 77.1, 75.3, 75.3, 66.6, 69.3, 58.3),
  values_end   = List(84.3, 83.5, 81.2, 78.8, 75.9, 77.4, 70.8),
  x_label      = "Années"
)
chart.show()</code></pre></div>
<div id="dumbbell-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::dumbbell({
    .title        = "Espérance de vie 1990 vs 2023",
    .labels       = {"Japon", "Australie", "Allemagne", "États-Unis", "Brésil", "Chine", "Inde"},
    .values_start = {78.9, 77.1, 75.3, 75.3, 66.6, 69.3, 58.3},
    .values_end   = {84.3, 83.5, 81.2, 78.8, 75.9, 77.4, 70.8},
    .x_label      = "Années",
});
chart.show();</code></pre></div>
</div>

---

## Voir aussi

- [Graphique en barres](bar.md) — `sp.build_bar_chart()`
- [Graphique de pente](slope.md) — `sp.build_slope()`
- [Barres horizontales](hbar.md) — `sp.build_hbar()`

</div>
