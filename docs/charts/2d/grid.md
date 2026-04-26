# Grid Layout

<div class="lang-en">

## Signature

```python
sp.build_grid(
    charts: list[Chart],
    *,
    cols: int = 2,
    width: int = 1200,
    height: int = 800,
    background: str | None = None,
    gap: int = 12,
    title: str = "",
) -> Chart
```

Aliases: `sp.grid`

---

## Description

A grid layout places multiple `Chart` objects side by side in a responsive column grid. Charts fill left-to-right, top-to-bottom in the order they appear in `charts`. The `cols` parameter sets the number of columns; if `len(charts)` is not a multiple of `cols`, the last row is left-aligned with empty space on the right. Each cell gets an equal share of the total `width` and `height`, minus `gap` pixels between cells. The grid is ideal for dashboards and reports where multiple equal-importance charts need to be seen simultaneously.

**Ideal for:**
- Dashboards displaying several KPI charts at once
- Side-by-side comparisons of related charts (e.g., before/after, different regions)
- Report layouts combining diverse chart types in a structured grid

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `charts` | `list[Chart]` | required | Charts to place in the grid |
| `cols` | `int` | `2` | Number of columns |
| `width` | `int` | `1200` | Total canvas width in pixels |
| `height` | `int` | `800` | Total canvas height in pixels |
| `background` | `str \| None` | `None` | Background color or `None` = transparent |
| `gap` | `int` | `12` | Gap between cells in pixels |
| `title` | `str` | `""` | Optional grid header title |

---

## Returns

`Chart`

---

## Examples

### Q4 Analytics Dashboard

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="grid">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('grid','grid-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('grid','grid-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('grid','grid-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('grid','grid-r',this)">R</button>
<button class="sp-tb" onclick="spTab('grid','grid-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('grid','grid-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('grid','grid-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('grid','grid-cpp',this)">C++</button>
</div>
<div id="grid-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

bar  = sp.bar(
    title="Revenue by Region",
    labels=["EMEA", "APAC", "Americas", "LATAM"],
    values=[420, 380, 610, 195],
)
line = sp.line(
    title="MAU Trend",
    x=[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
    y=[12, 15, 14, 18, 22, 25, 24, 28, 31, 29, 34, 38],
)
pie  = sp.pie(
    title="Revenue by Product",
    labels=["Cloud", "SaaS", "Hardware", "Services"],
    values=[42, 31, 15, 12],
)
hist = sp.histogram(
    title="Deal Size Distribution",
    values=[12,45,23,67,34,89,55,42,78,30,61,48,95,27,53],
    bins=8,
)

dashboard = sp.grid(
    charts=[bar, line, pie, hist],
    cols=2,
    title="Q4 Analytics Dashboard",
    gap=16,
)
dashboard.show()</code></pre></div>
<div id="grid-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const bar  = sp.bar({ title: "Revenue by Region",
    labels: ["EMEA","APAC","Americas","LATAM"], values: [420,380,610,195] });
const line = sp.line({ title: "MAU Trend",
    x: [1,2,3,4,5,6,7,8,9,10,11,12],
    y: [12,15,14,18,22,25,24,28,31,29,34,38] });
const pie  = sp.pie({ title: "Revenue by Product",
    labels: ["Cloud","SaaS","Hardware","Services"], values: [42,31,15,12] });
const hist = sp.histogram({ title: "Deal Size Distribution",
    values: [12,45,23,67,34,89,55,42,78,30,61,48,95,27,53], bins: 8 });

const dashboard = sp.grid({
  charts: [bar, line, pie, hist],
  cols: 2,
  title: "Q4 Analytics Dashboard",
  gap: 16,
});
dashboard.show();</code></pre></div>
<div id="grid-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const bar  = sp.bar({ title: "Revenue by Region",
    labels: ["EMEA","APAC","Americas","LATAM"], values: [420,380,610,195] });
const line = sp.line({ title: "MAU Trend",
    x: [1,2,3,4,5,6,7,8,9,10,11,12],
    y: [12,15,14,18,22,25,24,28,31,29,34,38] });
const pie  = sp.pie({ title: "Revenue by Product",
    labels: ["Cloud","SaaS","Hardware","Services"], values: [42,31,15,12] });
const hist = sp.histogram({ title: "Deal Size Distribution",
    values: [12,45,23,67,34,89,55,42,78,30,61,48,95,27,53], bins: 8 });

const dashboard = sp.grid({
  charts: [bar, line, pie, hist],
  cols: 2,
  title: "Q4 Analytics Dashboard",
  gap: 16,
});
dashboard.show();</code></pre></div>
<div id="grid-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

bar  <- sp$bar(title="Revenue by Region",
               labels=c("EMEA","APAC","Americas","LATAM"), values=c(420,380,610,195))
line <- sp$line(title="MAU Trend",
                x=1:12, y=c(12,15,14,18,22,25,24,28,31,29,34,38))
pie  <- sp$pie(title="Revenue by Product",
               labels=c("Cloud","SaaS","Hardware","Services"), values=c(42,31,15,12))
hist <- sp$histogram(title="Deal Size Distribution",
                     values=c(12,45,23,67,34,89,55,42,78,30,61,48,95,27,53), bins=8L)

dashboard <- sp$grid(
  charts = list(bar, line, pie, hist),
  cols   = 2L,
  title  = "Q4 Analytics Dashboard",
  gap    = 16L
)
dashboard$show()</code></pre></div>
<div id="grid-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var bar  = SeraPlot.bar().title("Revenue by Region")
    .labels(List.of("EMEA","APAC","Americas","LATAM")).values(List.of(420.0,380.0,610.0,195.0)).build();
var line = SeraPlot.line().title("MAU Trend")
    .x(List.of(1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0))
    .y(List.of(12.0,15.0,14.0,18.0,22.0,25.0,24.0,28.0,31.0,29.0,34.0,38.0)).build();
var pie  = SeraPlot.pie().title("Revenue by Product")
    .labels(List.of("Cloud","SaaS","Hardware","Services")).values(List.of(42.0,31.0,15.0,12.0)).build();
var hist = SeraPlot.histogram().title("Deal Size Distribution")
    .values(List.of(12.0,45.0,23.0,67.0,34.0,89.0,55.0,42.0,78.0,30.0,61.0,48.0,95.0,27.0,53.0))
    .bins(8).build();

var dashboard = SeraPlot.grid()
    .charts(List.of(bar, line, pie, hist))
    .cols(2)
    .title("Q4 Analytics Dashboard")
    .gap(16)
    .build();
dashboard.show();</code></pre></div>
<div id="grid-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var bar  = Sp.Bar(title: "Revenue by Region",
    labels: ["EMEA","APAC","Americas","LATAM"], values: [420,380,610,195]);
var line = Sp.Line(title: "MAU Trend",
    x: [1,2,3,4,5,6,7,8,9,10,11,12],
    y: [12,15,14,18,22,25,24,28,31,29,34,38]);
var pie  = Sp.Pie(title: "Revenue by Product",
    labels: ["Cloud","SaaS","Hardware","Services"], values: [42,31,15,12]);
var hist = Sp.Histogram(title: "Deal Size Distribution",
    values: [12,45,23,67,34,89,55,42,78,30,61,48,95,27,53], bins: 8);

var dashboard = Sp.Grid(
    charts: [bar, line, pie, hist],
    cols:   2,
    title:  "Q4 Analytics Dashboard",
    gap:    16
);
dashboard.Show();</code></pre></div>
<div id="grid-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val bar  = sp.bar(title = "Revenue by Region",
  labels = List("EMEA","APAC","Americas","LATAM"), values = List(420.0,380.0,610.0,195.0))
val line = sp.line(title = "MAU Trend",
  x = (1 to 12).map(_.toDouble).toList,
  y = List(12,15,14,18,22,25,24,28,31,29,34,38).map(_.toDouble))
val pie  = sp.pie(title = "Revenue by Product",
  labels = List("Cloud","SaaS","Hardware","Services"), values = List(42.0,31.0,15.0,12.0))
val hist = sp.histogram(title = "Deal Size Distribution",
  values = List(12,45,23,67,34,89,55,42,78,30,61,48,95,27,53).map(_.toDouble), bins = 8)

val dashboard = sp.grid(
  charts = List(bar, line, pie, hist),
  cols   = 2,
  title  = "Q4 Analytics Dashboard",
  gap    = 16
)
dashboard.show()</code></pre></div>
<div id="grid-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto bar  = sp::bar({.title="Revenue by Region",
    .labels={"EMEA","APAC","Americas","LATAM"}, .values={420,380,610,195}});
auto line = sp::line({.title="MAU Trend",
    .x={1,2,3,4,5,6,7,8,9,10,11,12},
    .y={12,15,14,18,22,25,24,28,31,29,34,38}});
auto pie  = sp::pie({.title="Revenue by Product",
    .labels={"Cloud","SaaS","Hardware","Services"}, .values={42,31,15,12}});
auto hist = sp::histogram({.title="Deal Size Distribution",
    .values={12,45,23,67,34,89,55,42,78,30,61,48,95,27,53}, .bins=8});

auto dashboard = sp::grid({
    .charts = {bar, line, pie, hist},
    .cols   = 2,
    .title  = "Q4 Analytics Dashboard",
    .gap    = 16,
});
dashboard.show();</code></pre></div>
</div>

<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>
<iframe src="../../previews/grid.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>
</details>

### 3-column layout

```python
dashboard = sp.grid(
    charts=[c1, c2, c3, c4, c5, c6],
    cols=3,
    width=1800,
    height=1000,
    title="Operations Overview",
)
```

---

## See also

- [Slideshow](slideshow.md) — `sp.build_slideshow()`
- [Bar Chart](bar.md) — `sp.build_bar_chart()`
- [Line Chart](line.md) — `sp.build_line_chart()`

</div>

<div class="lang-fr">

## Signature

```python
sp.build_grid(
    charts: list[Chart],
    *,
    cols: int = 2,
    width: int = 1200,
    height: int = 800,
    background: str | None = None,
    gap: int = 12,
    title: str = "",
) -> Chart
```

Aliases: `sp.grid`

---

## Description

Une grille place plusieurs objets `Chart` côte à côte dans une grille en colonnes. Les graphiques remplissent de gauche à droite, de haut en bas, dans l'ordre d'apparition dans `charts`. Le paramètre `cols` définit le nombre de colonnes. Si `len(charts)` n'est pas un multiple de `cols`, la dernière ligne est alignée à gauche avec de l'espace vide à droite. Chaque cellule reçoit une part égale de la `width` et `height` totales, moins `gap` pixels entre les cellules.

**Idéal pour :**
- Tableaux de bord affichant plusieurs graphiques KPI simultanément
- Comparaisons côte à côte de graphiques liés (avant/après, différentes régions)
- Mises en page de rapport combinant des types de graphiques variés dans une grille structurée

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `charts` | `list[Chart]` | requis | Graphiques à placer dans la grille |
| `cols` | `int` | `2` | Nombre de colonnes |
| `width` | `int` | `1200` | Largeur totale du canvas en pixels |
| `height` | `int` | `800` | Hauteur totale du canvas en pixels |
| `background` | `str \| None` | `None` | Couleur de fond ou `None` = transparent |
| `gap` | `int` | `12` | Espace entre les cellules en pixels |
| `title` | `str` | `""` | Titre d'en-tête optionnel de la grille |

---

## Retourne

`Chart`

---

## Exemples

### Tableau de bord T4

<div class="sp-tabs" id="grid-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('grid-fr','grid-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('grid-fr','grid-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('grid-fr','grid-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('grid-fr','grid-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('grid-fr','grid-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('grid-fr','grid-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('grid-fr','grid-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('grid-fr','grid-fr-cpp',this)">C++</button>
</div>
<div id="grid-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

barre = sp.bar(
    title="Revenus par région",
    labels=["EMEA", "APAC", "Amériques", "LATAM"],
    values=[420, 380, 610, 195],
)
courbe = sp.line(
    title="Tendance UAM",
    x=[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
    y=[12, 15, 14, 18, 22, 25, 24, 28, 31, 29, 34, 38],
)
camembert = sp.pie(
    title="Revenus par produit",
    labels=["Cloud", "SaaS", "Matériel", "Services"],
    values=[42, 31, 15, 12],
)
histo = sp.histogram(
    title="Distribution des tailles de deal",
    values=[12,45,23,67,34,89,55,42,78,30,61,48,95,27,53],
    bins=8,
)

tableau = sp.grid(
    charts=[barre, courbe, camembert, histo],
    cols=2,
    title="Tableau de bord T4",
    gap=16,
)
tableau.show()</code></pre></div>
<div id="grid-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const barre     = sp.bar({ title: "Revenus par région",
    labels: ["EMEA","APAC","Amériques","LATAM"], values: [420,380,610,195] });
const courbe    = sp.line({ title: "Tendance UAM",
    x: [1,2,3,4,5,6,7,8,9,10,11,12],
    y: [12,15,14,18,22,25,24,28,31,29,34,38] });
const camembert = sp.pie({ title: "Revenus par produit",
    labels: ["Cloud","SaaS","Matériel","Services"], values: [42,31,15,12] });
const histo     = sp.histogram({ title: "Distribution des tailles de deal",
    values: [12,45,23,67,34,89,55,42,78,30,61,48,95,27,53], bins: 8 });

const tableau = sp.grid({
  charts: [barre, courbe, camembert, histo],
  cols: 2,
  title: "Tableau de bord T4",
  gap: 16,
});
tableau.show();</code></pre></div>
<div id="grid-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const barre     = sp.bar({ title: "Revenus par région",
    labels: ["EMEA","APAC","Amériques","LATAM"], values: [420,380,610,195] });
const courbe    = sp.line({ title: "Tendance UAM",
    x: [1,2,3,4,5,6,7,8,9,10,11,12],
    y: [12,15,14,18,22,25,24,28,31,29,34,38] });
const camembert = sp.pie({ title: "Revenus par produit",
    labels: ["Cloud","SaaS","Matériel","Services"], values: [42,31,15,12] });
const histo     = sp.histogram({ title: "Distribution des tailles de deal",
    values: [12,45,23,67,34,89,55,42,78,30,61,48,95,27,53], bins: 8 });

const tableau = sp.grid({
  charts: [barre, courbe, camembert, histo],
  cols: 2,
  title: "Tableau de bord T4",
  gap: 16,
});
tableau.show();</code></pre></div>
<div id="grid-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

barre     <- sp$bar(title="Revenus par région",
                    labels=c("EMEA","APAC","Amériques","LATAM"), values=c(420,380,610,195))
courbe    <- sp$line(title="Tendance UAM",
                     x=1:12, y=c(12,15,14,18,22,25,24,28,31,29,34,38))
camembert <- sp$pie(title="Revenus par produit",
                    labels=c("Cloud","SaaS","Matériel","Services"), values=c(42,31,15,12))
histo     <- sp$histogram(title="Distribution des tailles de deal",
                          values=c(12,45,23,67,34,89,55,42,78,30,61,48,95,27,53), bins=8L)

tableau <- sp$grid(
  charts = list(barre, courbe, camembert, histo),
  cols   = 2L,
  title  = "Tableau de bord T4",
  gap    = 16L
)
tableau$show()</code></pre></div>
<div id="grid-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var barre     = SeraPlot.bar().title("Revenus par région")
    .labels(List.of("EMEA","APAC","Amériques","LATAM")).values(List.of(420.0,380.0,610.0,195.0)).build();
var courbe    = SeraPlot.line().title("Tendance UAM")
    .x(List.of(1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0))
    .y(List.of(12.0,15.0,14.0,18.0,22.0,25.0,24.0,28.0,31.0,29.0,34.0,38.0)).build();
var camembert = SeraPlot.pie().title("Revenus par produit")
    .labels(List.of("Cloud","SaaS","Matériel","Services")).values(List.of(42.0,31.0,15.0,12.0)).build();
var histo     = SeraPlot.histogram().title("Distribution des tailles de deal")
    .values(List.of(12.0,45.0,23.0,67.0,34.0,89.0,55.0,42.0,78.0,30.0,61.0,48.0,95.0,27.0,53.0))
    .bins(8).build();

var tableau = SeraPlot.grid()
    .charts(List.of(barre, courbe, camembert, histo))
    .cols(2)
    .title("Tableau de bord T4")
    .gap(16)
    .build();
tableau.show();</code></pre></div>
<div id="grid-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var barre     = Sp.Bar(title: "Revenus par région",
    labels: ["EMEA","APAC","Amériques","LATAM"], values: [420,380,610,195]);
var courbe    = Sp.Line(title: "Tendance UAM",
    x: [1,2,3,4,5,6,7,8,9,10,11,12],
    y: [12,15,14,18,22,25,24,28,31,29,34,38]);
var camembert = Sp.Pie(title: "Revenus par produit",
    labels: ["Cloud","SaaS","Matériel","Services"], values: [42,31,15,12]);
var histo     = Sp.Histogram(title: "Distribution des tailles de deal",
    values: [12,45,23,67,34,89,55,42,78,30,61,48,95,27,53], bins: 8);

var tableau = Sp.Grid(
    charts: [barre, courbe, camembert, histo],
    cols:   2,
    title:  "Tableau de bord T4",
    gap:    16
);
tableau.Show();</code></pre></div>
<div id="grid-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val barre     = sp.bar(title = "Revenus par région",
  labels = List("EMEA","APAC","Amériques","LATAM"), values = List(420.0,380.0,610.0,195.0))
val courbe    = sp.line(title = "Tendance UAM",
  x = (1 to 12).map(_.toDouble).toList,
  y = List(12,15,14,18,22,25,24,28,31,29,34,38).map(_.toDouble))
val camembert = sp.pie(title = "Revenus par produit",
  labels = List("Cloud","SaaS","Matériel","Services"), values = List(42.0,31.0,15.0,12.0))
val histo     = sp.histogram(title = "Distribution des tailles de deal",
  values = List(12,45,23,67,34,89,55,42,78,30,61,48,95,27,53).map(_.toDouble), bins = 8)

val tableau = sp.grid(
  charts = List(barre, courbe, camembert, histo),
  cols   = 2,
  title  = "Tableau de bord T4",
  gap    = 16
)
tableau.show()</code></pre></div>
<div id="grid-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto barre     = sp::bar({.title="Revenus par région",
    .labels={"EMEA","APAC","Amériques","LATAM"}, .values={420,380,610,195}});
auto courbe    = sp::line({.title="Tendance UAM",
    .x={1,2,3,4,5,6,7,8,9,10,11,12},
    .y={12,15,14,18,22,25,24,28,31,29,34,38}});
auto camembert = sp::pie({.title="Revenus par produit",
    .labels={"Cloud","SaaS","Matériel","Services"}, .values={42,31,15,12}});
auto histo     = sp::histogram({.title="Distribution des tailles de deal",
    .values={12,45,23,67,34,89,55,42,78,30,61,48,95,27,53}, .bins=8});

auto tableau = sp::grid({
    .charts = {barre, courbe, camembert, histo},
    .cols   = 2,
    .title  = "Tableau de bord T4",
    .gap    = 16,
});
tableau.show();</code></pre></div>
</div>

---

## Voir aussi

- [Slideshow](slideshow.md) — `sp.build_slideshow()`
- [Graphique en barres](bar.md) — `sp.build_bar_chart()`
- [Courbe](line.md) — `sp.build_line_chart()`

</div>
