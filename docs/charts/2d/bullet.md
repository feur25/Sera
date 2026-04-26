# Bullet Chart

<div class="lang-en">

## Signature

```python
sp.build_bullet(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    targets: list[float] | None = None,
    max_vals: list[float] | None = None,
    ranges: list[list[float]] | None = None,
    show_text: bool = True,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    background: str | None = None,
    palette: list[int] | None = None,
) -> Chart
```

Aliases: `sp.bullet`

---

## Description

A bullet chart follows the Stephen Few / Tufte design for information-dense KPI reporting. Each metric is rendered as a horizontal bar (the measured value) overlaid with a target marker (vertical tick) and up to three qualitative performance ranges (poor / satisfactory / good) shown as background bands. Multiple metrics are stacked vertically, making it possible to compare many KPIs in the space of a single chart. `ranges` is a list of lists, one sublist per metric, each containing up to three boundary values defining the performance bands.

**Ideal for:**
- Executive KPI dashboards with actual vs. target comparisons
- Sales performance tracking across multiple reps or regions
- Any scenario where multiple metrics need compact side-by-side display

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | — | Chart title displayed at the top |
| `labels` | `list[str]` | — | Name of each metric |
| `values` | `list[float]` | — | Measured value for each metric |
| `targets` | `list[float] \| None` | `None` | Target value per metric, shown as a vertical tick |
| `max_vals` | `list[float] \| None` | `None` | Maximum scale value per metric |
| `ranges` | `list[list[float]] \| None` | `None` | Performance band boundaries per metric (up to 3 values each) |
| `show_text` | `bool` | `True` | Display the numeric value at the end of each bar |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `x_label` | `str` | `""` | Label for the X axis |
| `background` | `str \| None` | `None` | CSS background color override |
| `palette` | `list[int] \| None` | `None` | Custom per-metric colors as hex integers |

---

## Returns

`Chart`

---

## Examples

### Q3 KPI performance

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="bullet">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bullet','bullet-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bullet','bullet-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bullet','bullet-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bullet','bullet-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bullet','bullet-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bullet','bullet-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bullet','bullet-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bullet','bullet-cpp',this)">C++</button>
</div>
<div id="bullet-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.bullet(
    title="Q3 KPI performance",
    labels=["Revenue", "Users", "NPS"],
    values=[820, 1450, 42],
    targets=[1000, 1800, 50],
    max_vals=[1200, 2000, 60],
    ranges=[
        [600, 800, 1200],   # Revenue bands
        [900, 1500, 2000],  # Users bands
        [30, 40, 60],       # NPS bands
    ],
)
chart.show()</code></pre></div>
<div id="bullet-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.bullet({
  title: "Q3 KPI performance",
  labels: ["Revenue", "Users", "NPS"],
  values: [820, 1450, 42],
  targets: [1000, 1800, 50],
  maxVals: [1200, 2000, 60],
  ranges: [
    [600, 800, 1200],
    [900, 1500, 2000],
    [30, 40, 60],
  ],
});
chart.show();</code></pre></div>
<div id="bullet-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.bullet({
  title: "Q3 KPI performance",
  labels: ["Revenue", "Users", "NPS"],
  values: [820, 1450, 42],
  targets: [1000, 1800, 50],
  maxVals: [1200, 2000, 60],
  ranges: [
    [600, 800, 1200],
    [900, 1500, 2000],
    [30, 40, 60],
  ],
});
chart.show();</code></pre></div>
<div id="bullet-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$bullet(
  title = "Q3 KPI performance",
  labels = c("Revenue", "Users", "NPS"),
  values = c(820, 1450, 42),
  targets = c(1000, 1800, 50),
  max_vals = c(1200, 2000, 60),
  ranges = list(
    c(600, 800, 1200),
    c(900, 1500, 2000),
    c(30, 40, 60)
  )
)
chart$show()</code></pre></div>
<div id="bullet-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bullet()
    .title("Q3 KPI performance")
    .labels(List.of("Revenue", "Users", "NPS"))
    .values(List.of(820.0, 1450.0, 42.0))
    .targets(List.of(1000.0, 1800.0, 50.0))
    .maxVals(List.of(1200.0, 2000.0, 60.0))
    .ranges(List.of(
        List.of(600.0, 800.0, 1200.0),
        List.of(900.0, 1500.0, 2000.0),
        List.of(30.0, 40.0, 60.0)
    ))
    .build();
chart.show();</code></pre></div>
<div id="bullet-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bullet(
    title: "Q3 KPI performance",
    labels: new[]{"Revenue", "Users", "NPS"},
    values: new[]{820.0, 1450.0, 42.0},
    targets: new[]{1000.0, 1800.0, 50.0},
    maxVals: new[]{1200.0, 2000.0, 60.0},
    ranges: new[]{
        new[]{600.0, 800.0, 1200.0},
        new[]{900.0, 1500.0, 2000.0},
        new[]{30.0, 40.0, 60.0},
    }
);
chart.Show();</code></pre></div>
<div id="bullet-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.bullet(
  title = "Q3 KPI performance",
  labels = List("Revenue", "Users", "NPS"),
  values = List(820, 1450, 42),
  targets = List(1000, 1800, 50),
  max_vals = List(1200, 2000, 60),
  ranges = List(
    List(600, 800, 1200),
    List(900, 1500, 2000),
    List(30, 40, 60)
  )
)
chart.show()</code></pre></div>
<div id="bullet-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bullet({
  .title    = "Q3 KPI performance",
  .labels   = {"Revenue", "Users", "NPS"},
  .values   = {820, 1450, 42},
  .targets  = {1000, 1800, 50},
  .max_vals = {1200, 2000, 60},
  .ranges   = {
    {600, 800, 1200},
    {900, 1500, 2000},
    {30, 40, 60}
  }
});
chart.show();</code></pre></div>
</div>

<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>
<iframe src="../../previews/bullet.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>
</details>

---

## See also

- [gauge.md](gauge.md) — Single-metric semicircular gauge
- [bar.md](bar.md) — Simple vertical bar chart
- [grid.md](grid.md) — Dashboard grid layout for multiple charts

</div>

<div class="lang-fr">

## Signature

```python
sp.build_bullet(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    targets: list[float] | None = None,
    max_vals: list[float] | None = None,
    ranges: list[list[float]] | None = None,
    show_text: bool = True,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    background: str | None = None,
    palette: list[int] | None = None,
) -> Chart
```

Aliases: `sp.bullet`

---

## Description

Un graphique en bullet suit le design Stephen Few / Tufte pour un reporting de KPI riche en informations. Chaque métrique est rendue comme une barre horizontale (la valeur mesurée) superposée à un marqueur cible (tiret vertical) et jusqu'à trois plages de performance qualitatives (insuffisant / satisfaisant / excellent) affichées comme des bandes de fond. Plusieurs métriques sont empilées verticalement, permettant de comparer de nombreux KPI dans l'espace d'un seul graphique. `ranges` est une liste de listes, une sous-liste par métrique, contenant jusqu'à trois valeurs limites définissant les bandes de performance.

**Idéal pour :**
- Tableaux de bord KPI exécutifs avec comparaisons réalisé vs cible
- Suivi des performances de vente sur plusieurs représentants ou régions
- Tout scénario nécessitant l'affichage compact de plusieurs métriques côte à côte

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | — | Titre du graphique affiché en haut |
| `labels` | `list[str]` | — | Nom de chaque métrique |
| `values` | `list[float]` | — | Valeur mesurée pour chaque métrique |
| `targets` | `list[float] \| None` | `None` | Valeur cible par métrique, affichée comme un tiret vertical |
| `max_vals` | `list[float] \| None` | `None` | Valeur maximale de l'échelle par métrique |
| `ranges` | `list[list[float]] \| None` | `None` | Limites de bandes de performance par métrique (jusqu'à 3 valeurs chacune) |
| `show_text` | `bool` | `True` | Afficher la valeur numérique à l'extrémité de chaque barre |
| `width` | `int` | `900` | Largeur du canevas en pixels |
| `height` | `int` | `480` | Hauteur du canevas en pixels |
| `x_label` | `str` | `""` | Label de l'axe X |
| `background` | `str \| None` | `None` | Couleur de fond CSS |
| `palette` | `list[int] \| None` | `None` | Couleurs personnalisées par métrique en entiers hexadécimaux |

---

## Retourne

`Chart`

---

## Exemples

### Performance des KPI T3

<div class="sp-tabs" id="bullet-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('bullet-fr','bullet-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('bullet-fr','bullet-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('bullet-fr','bullet-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('bullet-fr','bullet-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('bullet-fr','bullet-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('bullet-fr','bullet-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('bullet-fr','bullet-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('bullet-fr','bullet-fr-cpp',this)">C++</button>
</div>
<div id="bullet-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.bullet(
    title="Performance des KPI T3",
    labels=["Chiffre d'affaires", "Utilisateurs", "NPS"],
    values=[820, 1450, 42],
    targets=[1000, 1800, 50],
    max_vals=[1200, 2000, 60],
    ranges=[
        [600, 800, 1200],   # Bandes CA
        [900, 1500, 2000],  # Bandes Utilisateurs
        [30, 40, 60],       # Bandes NPS
    ],
)
chart.show()</code></pre></div>
<div id="bullet-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.bullet({
  title: "Performance des KPI T3",
  labels: ["Chiffre d'affaires", "Utilisateurs", "NPS"],
  values: [820, 1450, 42],
  targets: [1000, 1800, 50],
  maxVals: [1200, 2000, 60],
  ranges: [
    [600, 800, 1200],
    [900, 1500, 2000],
    [30, 40, 60],
  ],
});
chart.show();</code></pre></div>
<div id="bullet-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.bullet({
  title: "Performance des KPI T3",
  labels: ["Chiffre d'affaires", "Utilisateurs", "NPS"],
  values: [820, 1450, 42],
  targets: [1000, 1800, 50],
  maxVals: [1200, 2000, 60],
  ranges: [
    [600, 800, 1200],
    [900, 1500, 2000],
    [30, 40, 60],
  ],
});
chart.show();</code></pre></div>
<div id="bullet-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$bullet(
  title = "Performance des KPI T3",
  labels = c("Chiffre d'affaires", "Utilisateurs", "NPS"),
  values = c(820, 1450, 42),
  targets = c(1000, 1800, 50),
  max_vals = c(1200, 2000, 60),
  ranges = list(
    c(600, 800, 1200),
    c(900, 1500, 2000),
    c(30, 40, 60)
  )
)
chart$show()</code></pre></div>
<div id="bullet-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.bullet()
    .title("Performance des KPI T3")
    .labels(List.of("Chiffre d'affaires", "Utilisateurs", "NPS"))
    .values(List.of(820.0, 1450.0, 42.0))
    .targets(List.of(1000.0, 1800.0, 50.0))
    .maxVals(List.of(1200.0, 2000.0, 60.0))
    .ranges(List.of(
        List.of(600.0, 800.0, 1200.0),
        List.of(900.0, 1500.0, 2000.0),
        List.of(30.0, 40.0, 60.0)
    ))
    .build();
chart.show();</code></pre></div>
<div id="bullet-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Bullet(
    title: "Performance des KPI T3",
    labels: new[]{"Chiffre d'affaires", "Utilisateurs", "NPS"},
    values: new[]{820.0, 1450.0, 42.0},
    targets: new[]{1000.0, 1800.0, 50.0},
    maxVals: new[]{1200.0, 2000.0, 60.0},
    ranges: new[]{
        new[]{600.0, 800.0, 1200.0},
        new[]{900.0, 1500.0, 2000.0},
        new[]{30.0, 40.0, 60.0},
    }
);
chart.Show();</code></pre></div>
<div id="bullet-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.bullet(
  title = "Performance des KPI T3",
  labels = List("Chiffre d'affaires", "Utilisateurs", "NPS"),
  values = List(820, 1450, 42),
  targets = List(1000, 1800, 50),
  max_vals = List(1200, 2000, 60),
  ranges = List(
    List(600, 800, 1200),
    List(900, 1500, 2000),
    List(30, 40, 60)
  )
)
chart.show()</code></pre></div>
<div id="bullet-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::bullet({
  .title    = "Performance des KPI T3",
  .labels   = {"Chiffre d'affaires", "Utilisateurs", "NPS"},
  .values   = {820, 1450, 42},
  .targets  = {1000, 1800, 50},
  .max_vals = {1200, 2000, 60},
  .ranges   = {
    {600, 800, 1200},
    {900, 1500, 2000},
    {30, 40, 60}
  }
});
chart.show();</code></pre></div>
</div>

---

## Voir aussi

- [gauge.md](gauge.md) — Jauge semi-circulaire pour une seule métrique
- [bar.md](bar.md) — Graphique à barres verticales simple
- [grid.md](grid.md) — Grille de tableau de bord pour plusieurs graphiques

</div>
