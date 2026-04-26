# Treemap

<div class="lang-en">

## Signature

```python
sp.build_treemap(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    parents: list[str] | None = None,
    width: int = 900,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.treemap`

---

## Description

A treemap tiles rectangular cells whose area is proportional to `values`, using the squarified layout algorithm to maximize the aspect ratio of each tile. When `parents` is provided, tiles are nested inside their parent rectangles, forming a two-level hierarchy with colored group backgrounds. Font size inside each tile scales with tile area so labels are always legible at their given size. SeraPlot's Rust renderer handles layouts of thousands of nodes in under a millisecond.

**Ideal for:**
- Part-to-whole comparisons where area encoding conveys magnitude directly
- Navigating large hierarchies (file systems, org structures, market cap breakdowns)
- Comparing relative weights across many categories simultaneously

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Tile labels |
| `values` | `list[float]` | required | Tile values (determines area) |
| `parents` | `list[str] \| None` | `None` | Parent label per tile for nested layout |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `palette` | `list[int] \| None` | `None` | Colors per root group |
| `background` | `str \| None` | `None` | Background color or `None` = transparent |
| `hover_json` | `str \| None` | `None` | JSON string with extra per-tile tooltip data |

---

## Returns

`Chart`

---

## Examples

### S&P 500 sector weights

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="treemap">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('treemap','treemap-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('treemap','treemap-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('treemap','treemap-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('treemap','treemap-r',this)">R</button>
<button class="sp-tb" onclick="spTab('treemap','treemap-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('treemap','treemap-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('treemap','treemap-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('treemap','treemap-cpp',this)">C++</button>
</div>
<div id="treemap-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.treemap(
    title="S&P 500 Sector Weights (%)",
    labels=["Information Technology", "Health Care", "Financials",
            "Consumer Discretionary", "Industrials", "Communication Services",
            "Consumer Staples", "Energy", "Utilities",
            "Real Estate", "Materials"],
    values=[29.0, 12.5, 13.2, 10.7, 8.5, 8.8, 6.0, 4.0, 2.5, 2.4, 2.4],
)
chart.show()</code></pre></div>
<div id="treemap-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const chart = sp.treemap({
  title: "S&P 500 Sector Weights (%)",
  labels: ["Information Technology","Health Care","Financials",
           "Consumer Discretionary","Industrials","Communication Services",
           "Consumer Staples","Energy","Utilities","Real Estate","Materials"],
  values: [29.0,12.5,13.2,10.7,8.5,8.8,6.0,4.0,2.5,2.4,2.4],
});
chart.show();</code></pre></div>
<div id="treemap-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const chart = sp.treemap({
  title: "S&P 500 Sector Weights (%)",
  labels: ["Information Technology","Health Care","Financials",
           "Consumer Discretionary","Industrials","Communication Services",
           "Consumer Staples","Energy","Utilities","Real Estate","Materials"],
  values: [29.0,12.5,13.2,10.7,8.5,8.8,6.0,4.0,2.5,2.4,2.4],
});
chart.show();</code></pre></div>
<div id="treemap-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$treemap(
  title  = "S&P 500 Sector Weights (%)",
  labels = c("Information Technology","Health Care","Financials",
             "Consumer Discretionary","Industrials","Communication Services",
             "Consumer Staples","Energy","Utilities","Real Estate","Materials"),
  values = c(29.0,12.5,13.2,10.7,8.5,8.8,6.0,4.0,2.5,2.4,2.4)
)
chart$show()</code></pre></div>
<div id="treemap-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.treemap()
    .title("S&P 500 Sector Weights (%)")
    .labels(List.of("Information Technology","Health Care","Financials",
                    "Consumer Discretionary","Industrials","Communication Services",
                    "Consumer Staples","Energy","Utilities","Real Estate","Materials"))
    .values(List.of(29.0,12.5,13.2,10.7,8.5,8.8,6.0,4.0,2.5,2.4,2.4))
    .build();
chart.show();</code></pre></div>
<div id="treemap-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Treemap(
    title:  "S&P 500 Sector Weights (%)",
    labels: ["Information Technology","Health Care","Financials",
             "Consumer Discretionary","Industrials","Communication Services",
             "Consumer Staples","Energy","Utilities","Real Estate","Materials"],
    values: [29.0,12.5,13.2,10.7,8.5,8.8,6.0,4.0,2.5,2.4,2.4]
);
chart.Show();</code></pre></div>
<div id="treemap-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.treemap(
  title  = "S&P 500 Sector Weights (%)",
  labels = List("Information Technology","Health Care","Financials",
                "Consumer Discretionary","Industrials","Communication Services",
                "Consumer Staples","Energy","Utilities","Real Estate","Materials"),
  values = List(29.0,12.5,13.2,10.7,8.5,8.8,6.0,4.0,2.5,2.4,2.4)
)
chart.show()</code></pre></div>
<div id="treemap-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::treemap({
    .title  = "S&P 500 Sector Weights (%)",
    .labels = {"Information Technology","Health Care","Financials",
               "Consumer Discretionary","Industrials","Communication Services",
               "Consumer Staples","Energy","Utilities","Real Estate","Materials"},
    .values = {29.0,12.5,13.2,10.7,8.5,8.8,6.0,4.0,2.5,2.4,2.4},
});
chart.show();</code></pre></div>
</div>

<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>
<iframe src="../../previews/treemap.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>
</details>

### Nested layout with parents

```python
chart = sp.treemap(
    title="Codebase Size by Module (LOC)",
    labels=["Core","UI","API","Parser","Lexer","Router","Handlers","Auth"],
    values=[0,    0,   0,  4200,  1800,  2100,   3500,   1200],
    parents=["","","","Core","Core","API","API","API"],
)
```

---

## See also

- [Sunburst Chart](sunburst.md) — `sp.build_sunburst()`
- [Pie Chart](pie.md) — `sp.build_pie_chart()`
- [Bar Chart](bar.md) — `sp.build_bar_chart()`

</div>

<div class="lang-fr">

## Signature

```python
sp.build_treemap(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    parents: list[str] | None = None,
    width: int = 900,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.treemap`

---

## Description

Un treemap dispose des cellules rectangulaires dont l'aire est proportionnelle aux `values`, en utilisant l'algorithme de mise en page squarifiée pour maximiser le rapport d'aspect de chaque tuile. Quand `parents` est fourni, les tuiles sont imbriquées dans leurs rectangles parents, formant une hiérarchie à deux niveaux. La taille de police dans chaque tuile s'adapte à l'aire de la tuile. Le moteur Rust de SeraPlot gère des mises en page de milliers de nœuds en moins d'une milliseconde.

**Idéal pour :**
- Comparaisons partie-à-tout où l'encodage par aire transmet directement la magnitude
- Navigation dans de grandes hiérarchies (systèmes de fichiers, structures org, répartitions de capitalisation boursière)
- Comparer les poids relatifs de nombreuses catégories simultanément

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Étiquettes des tuiles |
| `values` | `list[float]` | requis | Valeurs des tuiles (détermine l'aire) |
| `parents` | `list[str] \| None` | `None` | Étiquette du parent par tuile pour la mise en page imbriquée |
| `width` | `int` | `900` | Largeur du canvas en pixels |
| `height` | `int` | `480` | Hauteur du canvas en pixels |
| `palette` | `list[int] \| None` | `None` | Couleurs par groupe racine |
| `background` | `str \| None` | `None` | Couleur de fond ou `None` = transparent |
| `hover_json` | `str \| None` | `None` | Chaîne JSON avec des données d'infobulle supplémentaires par tuile |

---

## Retourne

`Chart`

---

## Exemples

### Pondérations sectorielles S&P 500

<div class="sp-tabs" id="treemap-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('treemap-fr','treemap-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('treemap-fr','treemap-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('treemap-fr','treemap-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('treemap-fr','treemap-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('treemap-fr','treemap-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('treemap-fr','treemap-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('treemap-fr','treemap-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('treemap-fr','treemap-fr-cpp',this)">C++</button>
</div>
<div id="treemap-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.treemap(
    title="Pondérations sectorielles S&P 500 (%)",
    labels=["Technologies", "Santé", "Finance",
            "Conso. discrétion.", "Industrie", "Communication",
            "Conso. courante", "Énergie", "Services publics",
            "Immobilier", "Matériaux"],
    values=[29.0, 12.5, 13.2, 10.7, 8.5, 8.8, 6.0, 4.0, 2.5, 2.4, 2.4],
)
chart.show()</code></pre></div>
<div id="treemap-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const chart = sp.treemap({
  title: "Pondérations sectorielles S&P 500 (%)",
  labels: ["Technologies","Santé","Finance",
           "Conso. discrétion.","Industrie","Communication",
           "Conso. courante","Énergie","Services publics","Immobilier","Matériaux"],
  values: [29.0,12.5,13.2,10.7,8.5,8.8,6.0,4.0,2.5,2.4,2.4],
});
chart.show();</code></pre></div>
<div id="treemap-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const chart = sp.treemap({
  title: "Pondérations sectorielles S&P 500 (%)",
  labels: ["Technologies","Santé","Finance",
           "Conso. discrétion.","Industrie","Communication",
           "Conso. courante","Énergie","Services publics","Immobilier","Matériaux"],
  values: [29.0,12.5,13.2,10.7,8.5,8.8,6.0,4.0,2.5,2.4,2.4],
});
chart.show();</code></pre></div>
<div id="treemap-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$treemap(
  title  = "Pondérations sectorielles S&P 500 (%)",
  labels = c("Technologies","Santé","Finance",
             "Conso. discrétion.","Industrie","Communication",
             "Conso. courante","Énergie","Services publics","Immobilier","Matériaux"),
  values = c(29.0,12.5,13.2,10.7,8.5,8.8,6.0,4.0,2.5,2.4,2.4)
)
chart$show()</code></pre></div>
<div id="treemap-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.treemap()
    .title("Pondérations sectorielles S&P 500 (%)")
    .labels(List.of("Technologies","Santé","Finance",
                    "Conso. discrétion.","Industrie","Communication",
                    "Conso. courante","Énergie","Services publics","Immobilier","Matériaux"))
    .values(List.of(29.0,12.5,13.2,10.7,8.5,8.8,6.0,4.0,2.5,2.4,2.4))
    .build();
chart.show();</code></pre></div>
<div id="treemap-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Treemap(
    title:  "Pondérations sectorielles S&P 500 (%)",
    labels: ["Technologies","Santé","Finance",
             "Conso. discrétion.","Industrie","Communication",
             "Conso. courante","Énergie","Services publics","Immobilier","Matériaux"],
    values: [29.0,12.5,13.2,10.7,8.5,8.8,6.0,4.0,2.5,2.4,2.4]
);
chart.Show();</code></pre></div>
<div id="treemap-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.treemap(
  title  = "Pondérations sectorielles S&P 500 (%)",
  labels = List("Technologies","Santé","Finance",
                "Conso. discrétion.","Industrie","Communication",
                "Conso. courante","Énergie","Services publics","Immobilier","Matériaux"),
  values = List(29.0,12.5,13.2,10.7,8.5,8.8,6.0,4.0,2.5,2.4,2.4)
)
chart.show()</code></pre></div>
<div id="treemap-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::treemap({
    .title  = "Pondérations sectorielles S&P 500 (%)",
    .labels = {"Technologies","Santé","Finance",
               "Conso. discrétion.","Industrie","Communication",
               "Conso. courante","Énergie","Services publics","Immobilier","Matériaux"},
    .values = {29.0,12.5,13.2,10.7,8.5,8.8,6.0,4.0,2.5,2.4,2.4},
});
chart.show();</code></pre></div>
</div>

---

## Voir aussi

- [Sunburst](sunburst.md) — `sp.build_sunburst()`
- [Camembert](pie.md) — `sp.build_pie_chart()`
- [Graphique en barres](bar.md) — `sp.build_bar_chart()`

</div>
