# Stacked Bar Chart

<div class="lang-en">

## Signature

```python
sp.build_stacked_bar(
    title: str,
    category_labels: list[str],
    series_values: list[float],
    *,
    show_values: bool = False,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    legend_position: str = "right",
    gridlines: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.stacked_bar`

---

## Description

A stacked bar chart divides each bar into colored segments representing sub-series contributions to a total. `series_values` is a flat list laid out in row-major order: all values for series 1 first, then all values for series 2, etc. If there are `n` categories and `k` series, `series_values` has length `n × k`. The chart is ideal for seeing both the total magnitude and the part-to-whole composition in a single view. Use `no_y_axis` with `show_values` for a cleaner self-labelled layout.

**Ideal for:**
- Tracking total and component breakdown simultaneously (revenue by product line over years)
- Part-to-whole composition across categories
- Budget allocation or resource distribution comparisons

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `category_labels` | `list[str]` | required | X-axis category labels |
| `series_values` | `list[float]` | required | Flat row-major values: all series 1 values, then series 2, etc. |
| `show_values` | `bool` | `False` | Show numeric labels inside each segment |
| `series_names` | `list[str] \| None` | `None` | Legend labels per series |
| `palette` | `list[int] \| None` | `None` | Colors per series |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `legend_position` | `str` | `"right"` | Legend position: `"right"`, `"bottom"`, `"top"`, `"none"` |
| `gridlines` | `bool` | `False` | Show horizontal gridlines |
| `no_y_axis` | `bool` | `False` | Hide Y-axis ticks and labels |

---

## Returns

`Chart`

---

## Examples

### Annual revenue by product line

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="stacked-bar">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('stacked-bar','stacked-bar-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('stacked-bar','stacked-bar-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('stacked-bar','stacked-bar-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('stacked-bar','stacked-bar-r',this)">R</button>
<button class="sp-tb" onclick="spTab('stacked-bar','stacked-bar-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('stacked-bar','stacked-bar-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('stacked-bar','stacked-bar-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('stacked-bar','stacked-bar-cpp',this)">C++</button>
</div>
<div id="stacked-bar-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

# 4 years × 3 product lines (row-major: Cloud first, then SaaS, then Hardware)
chart = sp.stacked_bar(
    title="Annual Revenue by Product Line (K€)",
    category_labels=["2021", "2022", "2023", "2024"],
    series_values=[
        # Cloud
        320, 410, 530, 680,
        # SaaS
        210, 260, 310, 390,
        # Hardware
        150, 140, 130, 115,
    ],
    series_names=["Cloud", "SaaS", "Hardware"],
    legend_position="right",
)
chart.show()</code></pre></div>
<div id="stacked-bar-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const chart = sp.stackedBar({
  title: "Annual Revenue by Product Line (K€)",
  categoryLabels: ["2021", "2022", "2023", "2024"],
  seriesValues: [
    320, 410, 530, 680,  // Cloud
    210, 260, 310, 390,  // SaaS
    150, 140, 130, 115,  // Hardware
  ],
  seriesNames: ["Cloud", "SaaS", "Hardware"],
  legendPosition: "right",
});
chart.show();</code></pre></div>
<div id="stacked-bar-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const chart = sp.stackedBar({
  title: "Annual Revenue by Product Line (K€)",
  categoryLabels: ["2021", "2022", "2023", "2024"],
  seriesValues: [
    320, 410, 530, 680,  // Cloud
    210, 260, 310, 390,  // SaaS
    150, 140, 130, 115,  // Hardware
  ],
  seriesNames: ["Cloud", "SaaS", "Hardware"],
  legendPosition: "right",
});
chart.show();</code></pre></div>
<div id="stacked-bar-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$stacked_bar(
  title            = "Annual Revenue by Product Line (K€)",
  category_labels  = c("2021", "2022", "2023", "2024"),
  series_values    = c(
    320, 410, 530, 680,   # Cloud
    210, 260, 310, 390,   # SaaS
    150, 140, 130, 115    # Hardware
  ),
  series_names     = c("Cloud", "SaaS", "Hardware"),
  legend_position  = "right"
)
chart$show()</code></pre></div>
<div id="stacked-bar-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.stackedBar()
    .title("Annual Revenue by Product Line (K€)")
    .categoryLabels(List.of("2021", "2022", "2023", "2024"))
    .seriesValues(List.of(
        320.0, 410.0, 530.0, 680.0,
        210.0, 260.0, 310.0, 390.0,
        150.0, 140.0, 130.0, 115.0
    ))
    .seriesNames(List.of("Cloud", "SaaS", "Hardware"))
    .legendPosition("right")
    .build();
chart.show();</code></pre></div>
<div id="stacked-bar-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.StackedBar(
    title:          "Annual Revenue by Product Line (K€)",
    categoryLabels: ["2021", "2022", "2023", "2024"],
    seriesValues:   [320, 410, 530, 680, 210, 260, 310, 390, 150, 140, 130, 115],
    seriesNames:    ["Cloud", "SaaS", "Hardware"],
    legendPosition: "right"
);
chart.Show();</code></pre></div>
<div id="stacked-bar-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.stacked_bar(
  title           = "Annual Revenue by Product Line (K€)",
  category_labels = List("2021", "2022", "2023", "2024"),
  series_values   = List(
    320.0, 410.0, 530.0, 680.0,
    210.0, 260.0, 310.0, 390.0,
    150.0, 140.0, 130.0, 115.0
  ),
  series_names    = List("Cloud", "SaaS", "Hardware"),
  legend_position = "right"
)
chart.show()</code></pre></div>
<div id="stacked-bar-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::stacked_bar({
    .title           = "Annual Revenue by Product Line (K€)",
    .category_labels = {"2021", "2022", "2023", "2024"},
    .series_values   = {
        320.0, 410.0, 530.0, 680.0,
        210.0, 260.0, 310.0, 390.0,
        150.0, 140.0, 130.0, 115.0,
    },
    .series_names    = {"Cloud", "SaaS", "Hardware"},
    .legend_position = "right",
});
chart.show();</code></pre></div>
</div>

<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>
<iframe src="../../previews/stacked-bar.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>
</details>

### With value labels and no Y-axis

```python
chart = sp.stacked_bar(
    title="Survey Results by Region",
    category_labels=["North", "South", "East", "West"],
    series_values=[42,38,51,29, 35,30,27,41, 23,32,22,30],
    series_names=["Agree", "Neutral", "Disagree"],
    show_values=True,
    no_y_axis=True,
    legend_position="bottom",
)
```

---

## See also

- [Grouped Bar Chart](grouped-bar.md) — `sp.build_grouped_bar()`
- [Bar Chart](bar.md) — `sp.build_bar_chart()`
- [Area Chart](area.md) — `sp.build_area_chart()`

</div>

<div class="lang-fr">

## Signature

```python
sp.build_stacked_bar(
    title: str,
    category_labels: list[str],
    series_values: list[float],
    *,
    show_values: bool = False,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    legend_position: str = "right",
    gridlines: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.stacked_bar`

---

## Description

Un graphique en barres empilées divise chaque barre en segments colorés représentant les contributions de sous-séries à un total. `series_values` est une liste plate en ordre row-major : toutes les valeurs de la série 1 en premier, puis toutes les valeurs de la série 2, etc. Si on a `n` catégories et `k` séries, `series_values` a une longueur de `n × k`. Le graphique est idéal pour voir à la fois la magnitude totale et la composition partie-à-tout en une seule vue.

**Idéal pour :**
- Suivre simultanément le total et la décomposition par composant (revenus par ligne de produits sur plusieurs années)
- Comparaisons de composition partie-à-tout entre catégories
- Comparaisons d'allocation budgétaire ou de distribution de ressources

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `category_labels` | `list[str]` | requis | Étiquettes des catégories de l'axe X |
| `series_values` | `list[float]` | requis | Valeurs row-major : toutes les valeurs de la série 1, puis série 2, etc. |
| `show_values` | `bool` | `False` | Afficher les étiquettes numériques dans chaque segment |
| `series_names` | `list[str] \| None` | `None` | Étiquettes de légende par série |
| `palette` | `list[int] \| None` | `None` | Couleurs par série |
| `width` | `int` | `900` | Largeur du canvas en pixels |
| `height` | `int` | `480` | Hauteur du canvas en pixels |
| `legend_position` | `str` | `"right"` | Position de la légende : `"right"`, `"bottom"`, `"top"`, `"none"` |
| `gridlines` | `bool` | `False` | Afficher les lignes de grille horizontales |
| `no_y_axis` | `bool` | `False` | Masquer les graduations et étiquettes de l'axe Y |

---

## Retourne

`Chart`

---

## Exemples

### Revenus annuels par ligne de produits

<div class="sp-tabs" id="stacked-bar-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('stacked-bar-fr','stacked-bar-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('stacked-bar-fr','stacked-bar-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('stacked-bar-fr','stacked-bar-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('stacked-bar-fr','stacked-bar-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('stacked-bar-fr','stacked-bar-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('stacked-bar-fr','stacked-bar-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('stacked-bar-fr','stacked-bar-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('stacked-bar-fr','stacked-bar-fr-cpp',this)">C++</button>
</div>
<div id="stacked-bar-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.stacked_bar(
    title="Revenus annuels par ligne de produits (K€)",
    category_labels=["2021", "2022", "2023", "2024"],
    series_values=[
        # Cloud
        320, 410, 530, 680,
        # SaaS
        210, 260, 310, 390,
        # Matériel
        150, 140, 130, 115,
    ],
    series_names=["Cloud", "SaaS", "Matériel"],
    legend_position="right",
)
chart.show()</code></pre></div>
<div id="stacked-bar-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const chart = sp.stackedBar({
  title: "Revenus annuels par ligne de produits (K€)",
  categoryLabels: ["2021", "2022", "2023", "2024"],
  seriesValues: [320, 410, 530, 680, 210, 260, 310, 390, 150, 140, 130, 115],
  seriesNames: ["Cloud", "SaaS", "Matériel"],
  legendPosition: "right",
});
chart.show();</code></pre></div>
<div id="stacked-bar-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const chart = sp.stackedBar({
  title: "Revenus annuels par ligne de produits (K€)",
  categoryLabels: ["2021", "2022", "2023", "2024"],
  seriesValues: [320, 410, 530, 680, 210, 260, 310, 390, 150, 140, 130, 115],
  seriesNames: ["Cloud", "SaaS", "Matériel"],
  legendPosition: "right",
});
chart.show();</code></pre></div>
<div id="stacked-bar-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$stacked_bar(
  title           = "Revenus annuels par ligne de produits (K€)",
  category_labels = c("2021", "2022", "2023", "2024"),
  series_values   = c(320, 410, 530, 680, 210, 260, 310, 390, 150, 140, 130, 115),
  series_names    = c("Cloud", "SaaS", "Matériel"),
  legend_position = "right"
)
chart$show()</code></pre></div>
<div id="stacked-bar-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.stackedBar()
    .title("Revenus annuels par ligne de produits (K€)")
    .categoryLabels(List.of("2021", "2022", "2023", "2024"))
    .seriesValues(List.of(
        320.0, 410.0, 530.0, 680.0,
        210.0, 260.0, 310.0, 390.0,
        150.0, 140.0, 130.0, 115.0
    ))
    .seriesNames(List.of("Cloud", "SaaS", "Matériel"))
    .legendPosition("right")
    .build();
chart.show();</code></pre></div>
<div id="stacked-bar-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.StackedBar(
    title:          "Revenus annuels par ligne de produits (K€)",
    categoryLabels: ["2021", "2022", "2023", "2024"],
    seriesValues:   [320, 410, 530, 680, 210, 260, 310, 390, 150, 140, 130, 115],
    seriesNames:    ["Cloud", "SaaS", "Matériel"],
    legendPosition: "right"
);
chart.Show();</code></pre></div>
<div id="stacked-bar-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.stacked_bar(
  title           = "Revenus annuels par ligne de produits (K€)",
  category_labels = List("2021", "2022", "2023", "2024"),
  series_values   = List(320.0, 410.0, 530.0, 680.0, 210.0, 260.0, 310.0, 390.0, 150.0, 140.0, 130.0, 115.0),
  series_names    = List("Cloud", "SaaS", "Matériel"),
  legend_position = "right"
)
chart.show()</code></pre></div>
<div id="stacked-bar-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::stacked_bar({
    .title           = "Revenus annuels par ligne de produits (K€)",
    .category_labels = {"2021", "2022", "2023", "2024"},
    .series_values   = {320.0, 410.0, 530.0, 680.0, 210.0, 260.0, 310.0, 390.0, 150.0, 140.0, 130.0, 115.0},
    .series_names    = {"Cloud", "SaaS", "Matériel"},
    .legend_position = "right",
});
chart.show();</code></pre></div>
</div>

---

## Voir aussi

- [Barres groupées](grouped-bar.md) — `sp.build_grouped_bar()`
- [Graphique en barres](bar.md) — `sp.build_bar_chart()`
- [Graphique en aires](area.md) — `sp.build_area_chart()`

</div>
