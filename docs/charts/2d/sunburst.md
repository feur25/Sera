# Sunburst Chart

<div class="lang-en">

## Signature

```python
sp.build_sunburst(
    title: str,
    labels: list[str],
    parents: list[str],
    values: list[float],
    *,
    width: int = 700,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.sunburst`

---

## Description

A sunburst chart displays hierarchical data as concentric rings. Each ring represents one level of the hierarchy, with segments sized proportionally to their `values`. The tree structure is defined by parallel `labels` and `parents` lists: a node whose `parents` entry is `""` is a root node (inner ring), while other nodes reference their parent by label. SeraPlot uses an O(n) arena allocator for the tree so even deep hierarchies render instantly. Hovering a segment shows its full path from root to leaf.

**Ideal for:**
- Visualizing nested categorical hierarchies (org charts, file systems, budget breakdowns)
- Showing both individual segment sizes and total subtree proportions
- Exploring drill-down relationships in a single compact view

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Node names (must be unique) |
| `parents` | `list[str]` | required | Parent label per node; `""` marks root nodes |
| `values` | `list[float]` | required | Leaf values; non-leaf values are summed automatically |
| `width` | `int` | `700` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `palette` | `list[int] \| None` | `None` | Colors for root-level segments (inherited by children) |
| `background` | `str \| None` | `None` | Background color or `None` = transparent |
| `hover_json` | `str \| None` | `None` | JSON string with extra per-node tooltip data |

---

## Returns

`Chart`

---

## Examples

### Budget breakdown by department

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="sunburst">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('sunburst','sunburst-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('sunburst','sunburst-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('sunburst','sunburst-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('sunburst','sunburst-r',this)">R</button>
<button class="sp-tb" onclick="spTab('sunburst','sunburst-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('sunburst','sunburst-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('sunburst','sunburst-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('sunburst','sunburst-cpp',this)">C++</button>
</div>
<div id="sunburst-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.sunburst(
    title="Budget Breakdown by Department",
    labels=["Engineering", "Marketing", "Sales",
            "Backend",  "Frontend", "DevOps",
            "SEO", "Ads",
            "Enterprise", "SMB"],
    parents=["", "", "",
             "Engineering", "Engineering", "Engineering",
             "Marketing", "Marketing",
             "Sales", "Sales"],
    values=[0, 0, 0,
            420, 310, 180,
            95, 210,
            380, 260],
)
chart.show()</code></pre></div>
<div id="sunburst-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const chart = sp.sunburst({
  title: "Budget Breakdown by Department",
  labels:  ["Engineering","Marketing","Sales",
            "Backend","Frontend","DevOps",
            "SEO","Ads","Enterprise","SMB"],
  parents: ["","","",
            "Engineering","Engineering","Engineering",
            "Marketing","Marketing","Sales","Sales"],
  values:  [0,0,0, 420,310,180, 95,210, 380,260],
});
chart.show();</code></pre></div>
<div id="sunburst-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const chart = sp.sunburst({
  title: "Budget Breakdown by Department",
  labels:  ["Engineering","Marketing","Sales",
            "Backend","Frontend","DevOps",
            "SEO","Ads","Enterprise","SMB"],
  parents: ["","","",
            "Engineering","Engineering","Engineering",
            "Marketing","Marketing","Sales","Sales"],
  values:  [0,0,0, 420,310,180, 95,210, 380,260],
});
chart.show();</code></pre></div>
<div id="sunburst-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$sunburst(
  title   = "Budget Breakdown by Department",
  labels  = c("Engineering","Marketing","Sales",
              "Backend","Frontend","DevOps",
              "SEO","Ads","Enterprise","SMB"),
  parents = c("","","",
              "Engineering","Engineering","Engineering",
              "Marketing","Marketing","Sales","Sales"),
  values  = c(0,0,0, 420,310,180, 95,210, 380,260)
)
chart$show()</code></pre></div>
<div id="sunburst-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.sunburst()
    .title("Budget Breakdown by Department")
    .labels(List.of("Engineering","Marketing","Sales",
                    "Backend","Frontend","DevOps",
                    "SEO","Ads","Enterprise","SMB"))
    .parents(List.of("","","",
                     "Engineering","Engineering","Engineering",
                     "Marketing","Marketing","Sales","Sales"))
    .values(List.of(0.0,0.0,0.0, 420.0,310.0,180.0, 95.0,210.0, 380.0,260.0))
    .build();
chart.show();</code></pre></div>
<div id="sunburst-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Sunburst(
    title:   "Budget Breakdown by Department",
    labels:  ["Engineering","Marketing","Sales",
              "Backend","Frontend","DevOps",
              "SEO","Ads","Enterprise","SMB"],
    parents: ["","","",
              "Engineering","Engineering","Engineering",
              "Marketing","Marketing","Sales","Sales"],
    values:  [0,0,0, 420,310,180, 95,210, 380,260]
);
chart.Show();</code></pre></div>
<div id="sunburst-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.sunburst(
  title   = "Budget Breakdown by Department",
  labels  = List("Engineering","Marketing","Sales",
                 "Backend","Frontend","DevOps",
                 "SEO","Ads","Enterprise","SMB"),
  parents = List("","","",
                 "Engineering","Engineering","Engineering",
                 "Marketing","Marketing","Sales","Sales"),
  values  = List(0.0,0.0,0.0, 420.0,310.0,180.0, 95.0,210.0, 380.0,260.0)
)
chart.show()</code></pre></div>
<div id="sunburst-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::sunburst({
    .title   = "Budget Breakdown by Department",
    .labels  = {"Engineering","Marketing","Sales",
                "Backend","Frontend","DevOps",
                "SEO","Ads","Enterprise","SMB"},
    .parents = {"","","",
                "Engineering","Engineering","Engineering",
                "Marketing","Marketing","Sales","Sales"},
    .values  = {0.0,0.0,0.0, 420.0,310.0,180.0, 95.0,210.0, 380.0,260.0},
});
chart.show();</code></pre></div>
</div>

<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>
<iframe src="../../previews/sunburst.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>
</details>

---

## See also

- [Treemap](treemap.md) — `sp.build_treemap()`
- [Pie Chart](pie.md) — `sp.build_pie_chart()`
- [Donut Chart](donut.md) — `sp.build_donut_chart()`

</div>

<div class="lang-fr">

## Signature

```python
sp.build_sunburst(
    title: str,
    labels: list[str],
    parents: list[str],
    values: list[float],
    *,
    width: int = 700,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.sunburst`

---

## Description

Un graphique sunburst affiche des données hiérarchiques sous forme d'anneaux concentriques. Chaque anneau représente un niveau de la hiérarchie, avec des segments dimensionnés proportionnellement à leurs `values`. La structure arborescente est définie par les listes parallèles `labels` et `parents` : un nœud dont l'entrée `parents` est `""` est un nœud racine (anneau intérieur), tandis que les autres nœuds référencent leur parent par étiquette. SeraPlot utilise un allocateur d'arène O(n) pour l'arbre.

**Idéal pour :**
- Visualiser des hiérarchies catégorielles imbriquées (organigrammes, systèmes de fichiers, décompositions budgétaires)
- Afficher à la fois les tailles individuelles des segments et les proportions de sous-arbres
- Explorer des relations de drill-down dans une vue compacte

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Noms des nœuds (doivent être uniques) |
| `parents` | `list[str]` | requis | Étiquette du parent par nœud ; `""` marque les nœuds racines |
| `values` | `list[float]` | requis | Valeurs des feuilles ; les valeurs non-feuilles sont sommées automatiquement |
| `width` | `int` | `700` | Largeur du canvas en pixels |
| `height` | `int` | `480` | Hauteur du canvas en pixels |
| `palette` | `list[int] \| None` | `None` | Couleurs pour les segments de niveau racine (héritées par les enfants) |
| `background` | `str \| None` | `None` | Couleur de fond ou `None` = transparent |
| `hover_json` | `str \| None` | `None` | Chaîne JSON avec des données d'infobulle supplémentaires par nœud |

---

## Retourne

`Chart`

---

## Exemples

### Répartition du budget par département

<div class="sp-tabs" id="sunburst-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('sunburst-fr','sunburst-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('sunburst-fr','sunburst-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('sunburst-fr','sunburst-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('sunburst-fr','sunburst-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('sunburst-fr','sunburst-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('sunburst-fr','sunburst-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('sunburst-fr','sunburst-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('sunburst-fr','sunburst-fr-cpp',this)">C++</button>
</div>
<div id="sunburst-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.sunburst(
    title="Répartition du budget par département",
    labels=["Ingénierie", "Marketing", "Ventes",
            "Backend", "Frontend", "DevOps",
            "SEO", "Publicité",
            "Grands comptes", "PME"],
    parents=["", "", "",
             "Ingénierie", "Ingénierie", "Ingénierie",
             "Marketing", "Marketing",
             "Ventes", "Ventes"],
    values=[0, 0, 0,
            420, 310, 180,
            95, 210,
            380, 260],
)
chart.show()</code></pre></div>
<div id="sunburst-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const chart = sp.sunburst({
  title: "Répartition du budget par département",
  labels:  ["Ingénierie","Marketing","Ventes",
            "Backend","Frontend","DevOps",
            "SEO","Publicité","Grands comptes","PME"],
  parents: ["","","",
            "Ingénierie","Ingénierie","Ingénierie",
            "Marketing","Marketing","Ventes","Ventes"],
  values:  [0,0,0, 420,310,180, 95,210, 380,260],
});
chart.show();</code></pre></div>
<div id="sunburst-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const chart = sp.sunburst({
  title: "Répartition du budget par département",
  labels:  ["Ingénierie","Marketing","Ventes",
            "Backend","Frontend","DevOps",
            "SEO","Publicité","Grands comptes","PME"],
  parents: ["","","",
            "Ingénierie","Ingénierie","Ingénierie",
            "Marketing","Marketing","Ventes","Ventes"],
  values:  [0,0,0, 420,310,180, 95,210, 380,260],
});
chart.show();</code></pre></div>
<div id="sunburst-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$sunburst(
  title   = "Répartition du budget par département",
  labels  = c("Ingénierie","Marketing","Ventes",
              "Backend","Frontend","DevOps",
              "SEO","Publicité","Grands comptes","PME"),
  parents = c("","","",
              "Ingénierie","Ingénierie","Ingénierie",
              "Marketing","Marketing","Ventes","Ventes"),
  values  = c(0,0,0, 420,310,180, 95,210, 380,260)
)
chart$show()</code></pre></div>
<div id="sunburst-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.sunburst()
    .title("Répartition du budget par département")
    .labels(List.of("Ingénierie","Marketing","Ventes",
                    "Backend","Frontend","DevOps",
                    "SEO","Publicité","Grands comptes","PME"))
    .parents(List.of("","","",
                     "Ingénierie","Ingénierie","Ingénierie",
                     "Marketing","Marketing","Ventes","Ventes"))
    .values(List.of(0.0,0.0,0.0, 420.0,310.0,180.0, 95.0,210.0, 380.0,260.0))
    .build();
chart.show();</code></pre></div>
<div id="sunburst-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Sunburst(
    title:   "Répartition du budget par département",
    labels:  ["Ingénierie","Marketing","Ventes",
              "Backend","Frontend","DevOps",
              "SEO","Publicité","Grands comptes","PME"],
    parents: ["","","",
              "Ingénierie","Ingénierie","Ingénierie",
              "Marketing","Marketing","Ventes","Ventes"],
    values:  [0,0,0, 420,310,180, 95,210, 380,260]
);
chart.Show();</code></pre></div>
<div id="sunburst-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.sunburst(
  title   = "Répartition du budget par département",
  labels  = List("Ingénierie","Marketing","Ventes",
                 "Backend","Frontend","DevOps",
                 "SEO","Publicité","Grands comptes","PME"),
  parents = List("","","",
                 "Ingénierie","Ingénierie","Ingénierie",
                 "Marketing","Marketing","Ventes","Ventes"),
  values  = List(0.0,0.0,0.0, 420.0,310.0,180.0, 95.0,210.0, 380.0,260.0)
)
chart.show()</code></pre></div>
<div id="sunburst-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::sunburst({
    .title   = "Répartition du budget par département",
    .labels  = {"Ingénierie","Marketing","Ventes",
                "Backend","Frontend","DevOps",
                "SEO","Publicité","Grands comptes","PME"},
    .parents = {"","","",
                "Ingénierie","Ingénierie","Ingénierie",
                "Marketing","Marketing","Ventes","Ventes"},
    .values  = {0.0,0.0,0.0, 420.0,310.0,180.0, 95.0,210.0, 380.0,260.0},
});
chart.show();</code></pre></div>
</div>

---

## Voir aussi

- [Treemap](treemap.md) — `sp.build_treemap()`
- [Camembert](pie.md) — `sp.build_pie_chart()`
- [Donut](donut.md) — `sp.build_donut_chart()`

</div>
