# Lollipop Chart

<div class="lang-en">

## Signature

```python
sp.build_lollipop_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    orientation: str = "v",
    show_text: bool = False,
    sort_order: str = "none",
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.lollipop`

---

## Description

A lollipop chart is a cleaner, lower ink-density alternative to a bar chart: each category is represented by a thin stem line topped with a filled circle instead of a filled rectangle. This reduces visual clutter while preserving all quantitative information, making it a popular choice in modern data journalism and presentation-quality dashboards. The `orientation` parameter switches between vertical lollipops (default) and horizontal ones — horizontal works best for ranked lists with long category labels. `sort_order` lets you instantly produce a sorted ranking.

**Ideal for:**
- Ranking comparisons where minimal visual weight is desired
- Replacing bar charts in polished, presentation-ready designs
- Showing sparse data where most values are small but a few stand out

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Category labels |
| `values` | `list[float]` | required | Values for each lollipop |
| `color_hex` | `int` | `0x6366F1` | Uniform color for stems and dots |
| `palette` | `list[int] \| None` | `None` | Per-item color palette |
| `orientation` | `str` | `"v"` | `"v"` = vertical, `"h"` = horizontal |
| `show_text` | `bool` | `False` | Show value labels on each dot |
| `sort_order` | `str` | `"none"` | `"asc"`, `"desc"`, or `"none"` |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `x_label` | `str` | `""` | X-axis label |
| `y_label` | `str` | `""` | Y-axis label |
| `gridlines` | `bool` | `True` | Show gridlines |
| `no_y_axis` | `bool` | `False` | Hide Y axis |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="lollipop">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('lollipop','lollipop-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('lollipop','lollipop-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('lollipop','lollipop-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('lollipop','lollipop-r',this)">R</button>
<button class="sp-tb" onclick="spTab('lollipop','lollipop-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('lollipop','lollipop-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('lollipop','lollipop-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('lollipop','lollipop-cpp',this)">C++</button>
</div>
<div id="lollipop-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

frameworks = ["React", "Vue", "Angular", "Svelte", "Solid", "Lit", "Qwik"]
stars_k = [223, 47, 95, 80, 32, 18, 21]

chart = sp.lollipop(
    title="Frontend Framework Stars (GitHub, K)",
    labels=frameworks,
    values=stars_k,
    sort_order="desc",
    show_text=True,
    y_label="Stars (K)",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="lollipop-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const frameworks = ["React", "Vue", "Angular", "Svelte", "Solid", "Lit", "Qwik"];
const starsK = [223, 47, 95, 80, 32, 18, 21];

const chart = sp.lollipop({
  title: "Frontend Framework Stars (GitHub, K)",
  labels: frameworks,
  values: starsK,
  sortOrder: "desc",
  showText: true,
  yLabel: "Stars (K)",
  gridlines: true,
});
chart.show();</code></pre></div>
<div id="lollipop-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const frameworks: string[] = ["React", "Vue", "Angular", "Svelte", "Solid", "Lit", "Qwik"];
const starsK: number[] = [223, 47, 95, 80, 32, 18, 21];

const chart = sp.lollipop({
  title: "Frontend Framework Stars (GitHub, K)",
  labels: frameworks,
  values: starsK,
  sortOrder: "desc",
  showText: true,
  yLabel: "Stars (K)",
  gridlines: true,
});
chart.show();</code></pre></div>
<div id="lollipop-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

frameworks <- c("React", "Vue", "Angular", "Svelte", "Solid", "Lit", "Qwik")
stars_k    <- c(223, 47, 95, 80, 32, 18, 21)

chart <- sp$lollipop(
  title      = "Frontend Framework Stars (GitHub, K)",
  labels     = frameworks,
  values     = stars_k,
  sort_order = "desc",
  show_text  = TRUE,
  y_label    = "Stars (K)",
  gridlines  = TRUE
)
chart$show()</code></pre></div>
<div id="lollipop-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.lollipop()
    .title("Frontend Framework Stars (GitHub, K)")
    .labels(List.of("React", "Vue", "Angular", "Svelte", "Solid", "Lit", "Qwik"))
    .values(List.of(223.0, 47.0, 95.0, 80.0, 32.0, 18.0, 21.0))
    .sortOrder("desc")
    .showText(true)
    .yLabel("Stars (K)")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="lollipop-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Lollipop(
    title:     "Frontend Framework Stars (GitHub, K)",
    labels:    ["React", "Vue", "Angular", "Svelte", "Solid", "Lit", "Qwik"],
    values:    [223, 47, 95, 80, 32, 18, 21],
    sortOrder: "desc",
    showText:  true,
    yLabel:    "Stars (K)",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="lollipop-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.lollipop(
  title      = "Frontend Framework Stars (GitHub, K)",
  labels     = List("React", "Vue", "Angular", "Svelte", "Solid", "Lit", "Qwik"),
  values     = List(223.0, 47.0, 95.0, 80.0, 32.0, 18.0, 21.0),
  sort_order = "desc",
  show_text  = true,
  y_label    = "Stars (K)",
  gridlines  = true
)
chart.show()</code></pre></div>
<div id="lollipop-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::lollipop({
    .title      = "Frontend Framework Stars (GitHub, K)",
    .labels     = {"React", "Vue", "Angular", "Svelte", "Solid", "Lit", "Qwik"},
    .values     = {223.0, 47.0, 95.0, 80.0, 32.0, 18.0, 21.0},
    .sort_order = "desc",
    .show_text  = true,
    .y_label    = "Stars (K)",
    .gridlines  = true,
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/lollipop.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [Bar Chart](bar.md) — `sp.build_bar_chart()`
- [Horizontal Bar](hbar.md) — `sp.build_hbar()`
- [Dumbbell Chart](dumbbell.md) — `sp.build_dumbbell()`

</div>

<div class="lang-fr">

## Signature

```python
sp.build_lollipop_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    orientation: str = "v",
    show_text: bool = False,
    sort_order: str = "none",
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.lollipop`

---

## Description

Un graphique en sucette (lollipop chart) est une alternative plus épurée au graphique en barres : chaque catégorie est représentée par une fine ligne-tige surmontée d'un cercle plein au lieu d'un rectangle. Cela réduit l'encombrement visuel tout en conservant toutes les informations quantitatives, ce qui en fait un choix populaire dans le journalisme de données moderne. Le paramètre `orientation` bascule entre les sucettes verticales (par défaut) et horizontales — l'horizontal convient mieux aux listes classées avec de longues étiquettes. `sort_order` permet de produire instantanément un classement trié.

**Idéal pour :**
- Comparaisons de classements avec un poids visuel minimal
- Remplacement des barres dans les designs de présentation soignés
- Données éparses où la plupart des valeurs sont faibles mais quelques-unes se démarquent

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Étiquettes des catégories |
| `values` | `list[float]` | requis | Valeurs pour chaque sucette |
| `color_hex` | `int` | `0x6366F1` | Couleur uniforme pour les tiges et les points |
| `palette` | `list[int] \| None` | `None` | Palette de couleurs par élément |
| `orientation` | `str` | `"v"` | `"v"` = vertical, `"h"` = horizontal |
| `show_text` | `bool` | `False` | Afficher les étiquettes de valeur sur chaque point |
| `sort_order` | `str` | `"none"` | `"asc"`, `"desc"` ou `"none"` |
| `width` | `int` | `900` | Largeur du canvas en pixels |
| `height` | `int` | `480` | Hauteur du canvas en pixels |
| `x_label` | `str` | `""` | Étiquette de l'axe X |
| `y_label` | `str` | `""` | Étiquette de l'axe Y |
| `gridlines` | `bool` | `True` | Afficher les lignes de grille |
| `no_y_axis` | `bool` | `False` | Masquer l'axe Y |

---

## Retourne

`Chart`

---

<div class="sp-tabs" id="lollipop-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('lollipop-fr','lollipop-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('lollipop-fr','lollipop-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('lollipop-fr','lollipop-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('lollipop-fr','lollipop-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('lollipop-fr','lollipop-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('lollipop-fr','lollipop-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('lollipop-fr','lollipop-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('lollipop-fr','lollipop-fr-cpp',this)">C++</button>
</div>
<div id="lollipop-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

frameworks = ["React", "Vue", "Angular", "Svelte", "Solid", "Lit", "Qwik"]
etoiles_k  = [223, 47, 95, 80, 32, 18, 21]

chart = sp.lollipop(
    title="Stars GitHub des frameworks frontend (K)",
    labels=frameworks,
    values=etoiles_k,
    sort_order="desc",
    show_text=True,
    y_label="Stars (K)",
    gridlines=True,
)
chart.show()</code></pre></div>
<div id="lollipop-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const frameworks = ["React", "Vue", "Angular", "Svelte", "Solid", "Lit", "Qwik"];
const etoilesK   = [223, 47, 95, 80, 32, 18, 21];

const chart = sp.lollipop({
  title: "Stars GitHub des frameworks frontend (K)",
  labels: frameworks,
  values: etoilesK,
  sortOrder: "desc",
  showText: true,
  yLabel: "Stars (K)",
  gridlines: true,
});
chart.show();</code></pre></div>
<div id="lollipop-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const frameworks: string[] = ["React", "Vue", "Angular", "Svelte", "Solid", "Lit", "Qwik"];
const etoilesK: number[]   = [223, 47, 95, 80, 32, 18, 21];

const chart = sp.lollipop({
  title: "Stars GitHub des frameworks frontend (K)",
  labels: frameworks,
  values: etoilesK,
  sortOrder: "desc",
  showText: true,
  yLabel: "Stars (K)",
  gridlines: true,
});
chart.show();</code></pre></div>
<div id="lollipop-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

frameworks <- c("React", "Vue", "Angular", "Svelte", "Solid", "Lit", "Qwik")
etoiles_k  <- c(223, 47, 95, 80, 32, 18, 21)

chart <- sp$lollipop(
  title      = "Stars GitHub des frameworks frontend (K)",
  labels     = frameworks,
  values     = etoiles_k,
  sort_order = "desc",
  show_text  = TRUE,
  y_label    = "Stars (K)",
  gridlines  = TRUE
)
chart$show()</code></pre></div>
<div id="lollipop-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.lollipop()
    .title("Stars GitHub des frameworks frontend (K)")
    .labels(List.of("React", "Vue", "Angular", "Svelte", "Solid", "Lit", "Qwik"))
    .values(List.of(223.0, 47.0, 95.0, 80.0, 32.0, 18.0, 21.0))
    .sortOrder("desc")
    .showText(true)
    .yLabel("Stars (K)")
    .gridlines(true)
    .build();
chart.show();</code></pre></div>
<div id="lollipop-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Lollipop(
    title:     "Stars GitHub des frameworks frontend (K)",
    labels:    ["React", "Vue", "Angular", "Svelte", "Solid", "Lit", "Qwik"],
    values:    [223, 47, 95, 80, 32, 18, 21],
    sortOrder: "desc",
    showText:  true,
    yLabel:    "Stars (K)",
    gridlines: true
);
chart.Show();</code></pre></div>
<div id="lollipop-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.lollipop(
  title      = "Stars GitHub des frameworks frontend (K)",
  labels     = List("React", "Vue", "Angular", "Svelte", "Solid", "Lit", "Qwik"),
  values     = List(223.0, 47.0, 95.0, 80.0, 32.0, 18.0, 21.0),
  sort_order = "desc",
  show_text  = true,
  y_label    = "Stars (K)",
  gridlines  = true
)
chart.show()</code></pre></div>
<div id="lollipop-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::lollipop({
    .title      = "Stars GitHub des frameworks frontend (K)",
    .labels     = {"React", "Vue", "Angular", "Svelte", "Solid", "Lit", "Qwik"},
    .values     = {223.0, 47.0, 95.0, 80.0, 32.0, 18.0, 21.0},
    .sort_order = "desc",
    .show_text  = true,
    .y_label    = "Stars (K)",
    .gridlines  = true,
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/lollipop.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## Voir aussi

- [Graphique en barres](bar.md) — `sp.build_bar_chart()`
- [Barres horizontales](hbar.md) — `sp.build_hbar()`
- [Graphique haltère](dumbbell.md) — `sp.build_dumbbell()`

</div>
