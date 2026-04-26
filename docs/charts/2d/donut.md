# Donut Chart

<div class="lang-en">

## Signature

```python
sp.build_donut_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_pct: bool = True,
    inner_radius_ratio: float = 0.55,
    width: int = 700,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    hover_json: str | None = None,
    legend_position: str = "right",
) -> Chart
```

Aliases: `sp.donut`, `sp.donut_chart`

---

## Description

A donut chart is a pie chart with a circular cutout at the center, reducing the visual weight and leaving space to display a summary metric (total, KPI name, or label) in the hole. The `inner_radius_ratio` parameter controls the size of the cutout: `0.0` produces a solid pie, `0.9` produces a thin ring. Donut charts carry the same part-to-whole semantics as pie charts while being more modern and less prone to area-distortion perception issues. They work best with 3–7 slices.

**Ideal for:**
- Part-to-whole breakdowns where you also want to highlight a key total in the center
- KPI dashboards and executive summaries
- A slightly more modern alternative to standard pie charts

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | — | Chart title displayed at the top |
| `labels` | `list[str]` | — | Category name for each slice |
| `values` | `list[float]` | — | Numeric value for each slice; automatically normalized to 100% |
| `show_pct` | `bool` | `True` | Display percentage labels beside each slice |
| `inner_radius_ratio` | `float` | `0.55` | Ratio of the inner hole radius to the outer radius (0.0–0.9) |
| `width` | `int` | `700` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `palette` | `list[int] \| None` | `None` | Custom slice colors as hex integers |
| `background` | `str \| None` | `None` | CSS background color override |
| `hover_json` | `str \| None` | `None` | JSON string for custom tooltip data |
| `legend_position` | `str` | `"right"` | Legend placement: `"top"`, `"right"`, `"bottom"`, or `"left"` |

---

## Returns

`Chart`

---

## Examples

### Revenue by product line

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="donut">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('donut','donut-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('donut','donut-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('donut','donut-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('donut','donut-r',this)">R</button>
<button class="sp-tb" onclick="spTab('donut','donut-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('donut','donut-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('donut','donut-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('donut','donut-cpp',this)">C++</button>
</div>
<div id="donut-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.donut(
    title="Revenue by product line",
    labels=["SaaS", "Professional Services", "Licensing", "Training"],
    values=[52, 24, 15, 9],
    inner_radius_ratio=0.6,
)
chart.show()</code></pre></div>
<div id="donut-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.donut({
  title: "Revenue by product line",
  labels: ["SaaS", "Professional Services", "Licensing", "Training"],
  values: [52, 24, 15, 9],
  innerRadiusRatio: 0.6,
});
chart.show();</code></pre></div>
<div id="donut-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.donut({
  title: "Revenue by product line",
  labels: ["SaaS", "Professional Services", "Licensing", "Training"],
  values: [52, 24, 15, 9],
  innerRadiusRatio: 0.6,
});
chart.show();</code></pre></div>
<div id="donut-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$donut(
  title = "Revenue by product line",
  labels = c("SaaS", "Professional Services", "Licensing", "Training"),
  values = c(52, 24, 15, 9),
  inner_radius_ratio = 0.6
)
chart$show()</code></pre></div>
<div id="donut-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.donut()
    .title("Revenue by product line")
    .labels(List.of("SaaS", "Professional Services", "Licensing", "Training"))
    .values(List.of(52.0, 24.0, 15.0, 9.0))
    .innerRadiusRatio(0.6)
    .build();
chart.show();</code></pre></div>
<div id="donut-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Donut(
    title: "Revenue by product line",
    labels: new[]{"SaaS", "Professional Services", "Licensing", "Training"},
    values: new[]{52.0, 24.0, 15.0, 9.0},
    innerRadiusRatio: 0.6
);
chart.Show();</code></pre></div>
<div id="donut-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.donut(
  title = "Revenue by product line",
  labels = List("SaaS", "Professional Services", "Licensing", "Training"),
  values = List(52.0, 24.0, 15.0, 9.0),
  inner_radius_ratio = 0.6
)
chart.show()</code></pre></div>
<div id="donut-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::donut({
  .title               = "Revenue by product line",
  .labels              = {"SaaS", "Professional Services", "Licensing", "Training"},
  .values              = {52.0, 24.0, 15.0, 9.0},
  .inner_radius_ratio  = 0.6
});
chart.show();</code></pre></div>
</div>

<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>
<iframe src="../../previews/donut.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>
</details>

---

## See also

- [pie.md](pie.md) — Solid pie chart without a center cutout
- [sunburst.md](sunburst.md) — Hierarchical donut with multiple nested rings

</div>

<div class="lang-fr">

## Signature

```python
sp.build_donut_chart(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_pct: bool = True,
    inner_radius_ratio: float = 0.55,
    width: int = 700,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    hover_json: str | None = None,
    legend_position: str = "right",
) -> Chart
```

Aliases: `sp.donut`, `sp.donut_chart`

---

## Description

Un graphique en anneau est un graphique en secteurs avec un trou circulaire au centre, réduisant le poids visuel et laissant de la place pour afficher une métrique récapitulative (total, nom d'indicateur ou label) dans le vide. Le paramètre `inner_radius_ratio` contrôle la taille du trou : `0.0` produit un secteur plein, `0.9` produit un anneau fin. Les graphiques en anneau ont la même sémantique parties-tout que les graphiques en secteurs, tout en étant plus modernes et moins sujets aux problèmes de perception liés à la distorsion des surfaces. Ils fonctionnent mieux avec 3 à 7 tranches.

**Idéal pour :**
- Les décompositions parties-tout lorsque vous souhaitez également mettre en évidence un total clé au centre
- Les tableaux de bord d'indicateurs et les rapports exécutifs
- Une alternative légèrement plus moderne aux graphiques en secteurs standard

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | — | Titre du graphique affiché en haut |
| `labels` | `list[str]` | — | Nom de catégorie pour chaque tranche |
| `values` | `list[float]` | — | Valeur numérique pour chaque tranche ; normalisée automatiquement à 100 % |
| `show_pct` | `bool` | `True` | Afficher les pourcentages à côté de chaque tranche |
| `inner_radius_ratio` | `float` | `0.55` | Rapport du rayon intérieur au rayon extérieur (0.0–0.9) |
| `width` | `int` | `700` | Largeur du canevas en pixels |
| `height` | `int` | `480` | Hauteur du canevas en pixels |
| `palette` | `list[int] \| None` | `None` | Couleurs de tranche personnalisées en entiers hexadécimaux |
| `background` | `str \| None` | `None` | Couleur de fond CSS |
| `hover_json` | `str \| None` | `None` | JSON pour les données d'info-bulle personnalisées |
| `legend_position` | `str` | `"right"` | Position de la légende : `"top"`, `"right"`, `"bottom"` ou `"left"` |

---

## Retourne

`Chart`

---

## Exemples

### Chiffre d'affaires par ligne de produit

<div class="sp-tabs" id="donut-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('donut-fr','donut-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('donut-fr','donut-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('donut-fr','donut-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('donut-fr','donut-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('donut-fr','donut-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('donut-fr','donut-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('donut-fr','donut-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('donut-fr','donut-fr-cpp',this)">C++</button>
</div>
<div id="donut-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.donut(
    title="Chiffre d'affaires par ligne de produit",
    labels=["SaaS", "Services professionnels", "Licences", "Formation"],
    values=[52, 24, 15, 9],
    inner_radius_ratio=0.6,
)
chart.show()</code></pre></div>
<div id="donut-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.donut({
  title: "Chiffre d'affaires par ligne de produit",
  labels: ["SaaS", "Services professionnels", "Licences", "Formation"],
  values: [52, 24, 15, 9],
  innerRadiusRatio: 0.6,
});
chart.show();</code></pre></div>
<div id="donut-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.donut({
  title: "Chiffre d'affaires par ligne de produit",
  labels: ["SaaS", "Services professionnels", "Licences", "Formation"],
  values: [52, 24, 15, 9],
  innerRadiusRatio: 0.6,
});
chart.show();</code></pre></div>
<div id="donut-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$donut(
  title = "Chiffre d'affaires par ligne de produit",
  labels = c("SaaS", "Services professionnels", "Licences", "Formation"),
  values = c(52, 24, 15, 9),
  inner_radius_ratio = 0.6
)
chart$show()</code></pre></div>
<div id="donut-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.donut()
    .title("Chiffre d'affaires par ligne de produit")
    .labels(List.of("SaaS", "Services professionnels", "Licences", "Formation"))
    .values(List.of(52.0, 24.0, 15.0, 9.0))
    .innerRadiusRatio(0.6)
    .build();
chart.show();</code></pre></div>
<div id="donut-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Donut(
    title: "Chiffre d'affaires par ligne de produit",
    labels: new[]{"SaaS", "Services professionnels", "Licences", "Formation"},
    values: new[]{52.0, 24.0, 15.0, 9.0},
    innerRadiusRatio: 0.6
);
chart.Show();</code></pre></div>
<div id="donut-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.donut(
  title = "Chiffre d'affaires par ligne de produit",
  labels = List("SaaS", "Services professionnels", "Licences", "Formation"),
  values = List(52.0, 24.0, 15.0, 9.0),
  inner_radius_ratio = 0.6
)
chart.show()</code></pre></div>
<div id="donut-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::donut({
  .title              = "Chiffre d'affaires par ligne de produit",
  .labels             = {"SaaS", "Services professionnels", "Licences", "Formation"},
  .values             = {52.0, 24.0, 15.0, 9.0},
  .inner_radius_ratio = 0.6
});
chart.show();</code></pre></div>
</div>

---

## Voir aussi

- [pie.md](pie.md) — Graphique en secteurs plein sans trou central
- [sunburst.md](sunburst.md) — Anneau hiérarchique avec plusieurs anneaux imbriqués

</div>
