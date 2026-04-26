# Slope Chart

<div class="lang-en">

## Signature

```python
sp.build_slope(
    title: str,
    labels: list[str],
    values_left: list[float],
    values_right: list[float],
    *,
    left_label: str = "Before",
    right_label: str = "After",
    show_text: bool = True,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    width: int = 600,
    height: int = 480,
    background: str | None = None,
) -> Chart
```

Aliases: `sp.slope`

---

## Description

A slope chart draws one line per entity between two vertical axes, highlighting change between exactly two time points or conditions. Unlike a line chart (which uses a continuous X axis), slope charts put emphasis on individual trajectories and their rank changes. Endpoint labels are shown by default via `show_text`. Best suited for 5–20 entities; more entities benefit from reduced `color_hex` opacity or grouped coloring. Ascending slopes (positive change) and descending slopes (negative change) are immediately distinguishable by direction.

**Ideal for:**
- Showing before/after changes for a moderate number of entities
- Illustrating rank reordering (e.g., country HDI rankings across two years)
- Comparing two conditions, policies, or time periods side by side

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Entity names (one label per line) |
| `values_left` | `list[float]` | required | Left-axis value per entity |
| `values_right` | `list[float]` | required | Right-axis value per entity |
| `left_label` | `str` | `"Before"` | Left-axis column header |
| `right_label` | `str` | `"After"` | Right-axis column header |
| `show_text` | `bool` | `True` | Show value labels at both endpoints |
| `color_hex` | `int` | `0x6366F1` | Single color for all lines; `palette` overrides this |
| `palette` | `list[int] \| None` | `None` | Per-entity colors |
| `width` | `int` | `600` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `background` | `str \| None` | `None` | Background color or `None` = transparent |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="slope">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('slope','slope-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('slope','slope-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('slope','slope-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('slope','slope-r',this)">R</button>
<button class="sp-tb" onclick="spTab('slope','slope-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('slope','slope-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('slope','slope-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('slope','slope-cpp',this)">C++</button>
</div>
<div id="slope-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.slope(
    title="Human Development Index: 2010 vs 2022",
    labels=["Norway", "Switzerland", "Iceland", "Hong Kong", "Australia",
            "Denmark", "Sweden", "Ireland", "Germany", "Netherlands"],
    values_left= [0.941, 0.934, 0.917, 0.898, 0.929, 0.920, 0.916, 0.908, 0.919, 0.921],
    values_right=[0.966, 0.962, 0.959, 0.956, 0.946, 0.952, 0.952, 0.950, 0.950, 0.946],
    left_label="2010",
    right_label="2022",
)
chart.show()</code></pre></div>
<div id="slope-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const chart = sp.slope({
  title: "Human Development Index: 2010 vs 2022",
  labels: ["Norway","Switzerland","Iceland","Hong Kong","Australia",
           "Denmark","Sweden","Ireland","Germany","Netherlands"],
  valuesLeft:  [0.941,0.934,0.917,0.898,0.929,0.920,0.916,0.908,0.919,0.921],
  valuesRight: [0.966,0.962,0.959,0.956,0.946,0.952,0.952,0.950,0.950,0.946],
  leftLabel:   "2010",
  rightLabel:  "2022",
});
chart.show();</code></pre></div>
<div id="slope-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const chart = sp.slope({
  title: "Human Development Index: 2010 vs 2022",
  labels: ["Norway","Switzerland","Iceland","Hong Kong","Australia",
           "Denmark","Sweden","Ireland","Germany","Netherlands"],
  valuesLeft:  [0.941,0.934,0.917,0.898,0.929,0.920,0.916,0.908,0.919,0.921],
  valuesRight: [0.966,0.962,0.959,0.956,0.946,0.952,0.952,0.950,0.950,0.946],
  leftLabel:   "2010",
  rightLabel:  "2022",
});
chart.show();</code></pre></div>
<div id="slope-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$slope(
  title        = "Human Development Index: 2010 vs 2022",
  labels       = c("Norway","Switzerland","Iceland","Hong Kong","Australia",
                   "Denmark","Sweden","Ireland","Germany","Netherlands"),
  values_left  = c(0.941,0.934,0.917,0.898,0.929,0.920,0.916,0.908,0.919,0.921),
  values_right = c(0.966,0.962,0.959,0.956,0.946,0.952,0.952,0.950,0.950,0.946),
  left_label   = "2010",
  right_label  = "2022"
)
chart$show()</code></pre></div>
<div id="slope-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.slope()
    .title("Human Development Index: 2010 vs 2022")
    .labels(List.of("Norway","Switzerland","Iceland","Hong Kong","Australia",
                    "Denmark","Sweden","Ireland","Germany","Netherlands"))
    .valuesLeft(List.of(0.941,0.934,0.917,0.898,0.929,0.920,0.916,0.908,0.919,0.921))
    .valuesRight(List.of(0.966,0.962,0.959,0.956,0.946,0.952,0.952,0.950,0.950,0.946))
    .leftLabel("2010")
    .rightLabel("2022")
    .build();
chart.show();</code></pre></div>
<div id="slope-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Slope(
    title:       "Human Development Index: 2010 vs 2022",
    labels:      ["Norway","Switzerland","Iceland","Hong Kong","Australia",
                  "Denmark","Sweden","Ireland","Germany","Netherlands"],
    valuesLeft:  [0.941,0.934,0.917,0.898,0.929,0.920,0.916,0.908,0.919,0.921],
    valuesRight: [0.966,0.962,0.959,0.956,0.946,0.952,0.952,0.950,0.950,0.946],
    leftLabel:   "2010",
    rightLabel:  "2022"
);
chart.Show();</code></pre></div>
<div id="slope-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.slope(
  title        = "Human Development Index: 2010 vs 2022",
  labels       = List("Norway","Switzerland","Iceland","Hong Kong","Australia",
                      "Denmark","Sweden","Ireland","Germany","Netherlands"),
  values_left  = List(0.941,0.934,0.917,0.898,0.929,0.920,0.916,0.908,0.919,0.921),
  values_right = List(0.966,0.962,0.959,0.956,0.946,0.952,0.952,0.950,0.950,0.946),
  left_label   = "2010",
  right_label  = "2022"
)
chart.show()</code></pre></div>
<div id="slope-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::slope({
    .title        = "Human Development Index: 2010 vs 2022",
    .labels       = {"Norway","Switzerland","Iceland","Hong Kong","Australia",
                     "Denmark","Sweden","Ireland","Germany","Netherlands"},
    .values_left  = {0.941,0.934,0.917,0.898,0.929,0.920,0.916,0.908,0.919,0.921},
    .values_right = {0.966,0.962,0.959,0.956,0.946,0.952,0.952,0.950,0.950,0.946},
    .left_label   = "2010",
    .right_label  = "2022",
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/slope.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [Dumbbell Chart](dumbbell.md) — `sp.build_dumbbell()`
- [Line Chart](line.md) — `sp.build_line_chart()`
- [Multi-Line Chart](multiline.md) — `sp.build_multiline()`

</div>

<div class="lang-fr">

## Signature

```python
sp.build_slope(
    title: str,
    labels: list[str],
    values_left: list[float],
    values_right: list[float],
    *,
    left_label: str = "Before",
    right_label: str = "After",
    show_text: bool = True,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    width: int = 600,
    height: int = 480,
    background: str | None = None,
) -> Chart
```

Aliases: `sp.slope`

---

## Description

Un graphique en pentes trace une ligne par entité entre deux axes verticaux, mettant en évidence le changement entre exactement deux instants ou conditions. Contrairement à un graphique en courbes (qui utilise un axe X continu), les graphiques en pentes mettent l'accent sur les trajectoires individuelles et leurs changements de rang. Les étiquettes de terminaison sont affichées par défaut via `show_text`. Idéal pour 5 à 20 entités ; au-delà, réduire l'opacité ou utiliser des couleurs groupées.

**Idéal pour :**
- Afficher les changements avant/après pour un nombre modéré d'entités
- Illustrer le réordonnancement des rangs (ex. classements IDH par pays sur deux années)
- Comparer deux conditions, politiques ou périodes côte à côte

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Noms des entités (une étiquette par ligne) |
| `values_left` | `list[float]` | requis | Valeur de l'axe gauche par entité |
| `values_right` | `list[float]` | requis | Valeur de l'axe droit par entité |
| `left_label` | `str` | `"Before"` | En-tête de la colonne gauche |
| `right_label` | `str` | `"After"` | En-tête de la colonne droite |
| `show_text` | `bool` | `True` | Afficher les étiquettes de valeur aux deux extrémités |
| `color_hex` | `int` | `0x6366F1` | Couleur unique pour toutes les lignes ; `palette` écrase ce paramètre |
| `palette` | `list[int] \| None` | `None` | Couleurs par entité |
| `width` | `int` | `600` | Largeur du canvas en pixels |
| `height` | `int` | `480` | Hauteur du canvas en pixels |
| `background` | `str \| None` | `None` | Couleur de fond ou `None` = transparent |

---

## Retourne

`Chart`

---

<div class="sp-tabs" id="slope-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('slope-fr','slope-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('slope-fr','slope-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('slope-fr','slope-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('slope-fr','slope-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('slope-fr','slope-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('slope-fr','slope-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('slope-fr','slope-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('slope-fr','slope-fr-cpp',this)">C++</button>
</div>
<div id="slope-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.slope(
    title="Indice de développement humain : 2010 vs 2022",
    labels=["Norvège", "Suisse", "Islande", "Hong Kong", "Australie",
            "Danemark", "Suède", "Irlande", "Allemagne", "Pays-Bas"],
    values_left= [0.941, 0.934, 0.917, 0.898, 0.929, 0.920, 0.916, 0.908, 0.919, 0.921],
    values_right=[0.966, 0.962, 0.959, 0.956, 0.946, 0.952, 0.952, 0.950, 0.950, 0.946],
    left_label="2010",
    right_label="2022",
)
chart.show()</code></pre></div>
<div id="slope-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const chart = sp.slope({
  title: "Indice de développement humain : 2010 vs 2022",
  labels: ["Norvège","Suisse","Islande","Hong Kong","Australie",
           "Danemark","Suède","Irlande","Allemagne","Pays-Bas"],
  valuesLeft:  [0.941,0.934,0.917,0.898,0.929,0.920,0.916,0.908,0.919,0.921],
  valuesRight: [0.966,0.962,0.959,0.956,0.946,0.952,0.952,0.950,0.950,0.946],
  leftLabel:   "2010",
  rightLabel:  "2022",
});
chart.show();</code></pre></div>
<div id="slope-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const chart = sp.slope({
  title: "Indice de développement humain : 2010 vs 2022",
  labels: ["Norvège","Suisse","Islande","Hong Kong","Australie",
           "Danemark","Suède","Irlande","Allemagne","Pays-Bas"],
  valuesLeft:  [0.941,0.934,0.917,0.898,0.929,0.920,0.916,0.908,0.919,0.921],
  valuesRight: [0.966,0.962,0.959,0.956,0.946,0.952,0.952,0.950,0.950,0.946],
  leftLabel:   "2010",
  rightLabel:  "2022",
});
chart.show();</code></pre></div>
<div id="slope-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$slope(
  title        = "Indice de développement humain : 2010 vs 2022",
  labels       = c("Norvège","Suisse","Islande","Hong Kong","Australie",
                   "Danemark","Suède","Irlande","Allemagne","Pays-Bas"),
  values_left  = c(0.941,0.934,0.917,0.898,0.929,0.920,0.916,0.908,0.919,0.921),
  values_right = c(0.966,0.962,0.959,0.956,0.946,0.952,0.952,0.950,0.950,0.946),
  left_label   = "2010",
  right_label  = "2022"
)
chart$show()</code></pre></div>
<div id="slope-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.slope()
    .title("Indice de développement humain : 2010 vs 2022")
    .labels(List.of("Norvège","Suisse","Islande","Hong Kong","Australie",
                    "Danemark","Suède","Irlande","Allemagne","Pays-Bas"))
    .valuesLeft(List.of(0.941,0.934,0.917,0.898,0.929,0.920,0.916,0.908,0.919,0.921))
    .valuesRight(List.of(0.966,0.962,0.959,0.956,0.946,0.952,0.952,0.950,0.950,0.946))
    .leftLabel("2010")
    .rightLabel("2022")
    .build();
chart.show();</code></pre></div>
<div id="slope-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Slope(
    title:       "Indice de développement humain : 2010 vs 2022",
    labels:      ["Norvège","Suisse","Islande","Hong Kong","Australie",
                  "Danemark","Suède","Irlande","Allemagne","Pays-Bas"],
    valuesLeft:  [0.941,0.934,0.917,0.898,0.929,0.920,0.916,0.908,0.919,0.921],
    valuesRight: [0.966,0.962,0.959,0.956,0.946,0.952,0.952,0.950,0.950,0.946],
    leftLabel:   "2010",
    rightLabel:  "2022"
);
chart.Show();</code></pre></div>
<div id="slope-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val chart = sp.slope(
  title        = "Indice de développement humain : 2010 vs 2022",
  labels       = List("Norvège","Suisse","Islande","Hong Kong","Australie",
                      "Danemark","Suède","Irlande","Allemagne","Pays-Bas"),
  values_left  = List(0.941,0.934,0.917,0.898,0.929,0.920,0.916,0.908,0.919,0.921),
  values_right = List(0.966,0.962,0.959,0.956,0.946,0.952,0.952,0.950,0.950,0.946),
  left_label   = "2010",
  right_label  = "2022"
)
chart.show()</code></pre></div>
<div id="slope-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::slope({
    .title        = "Indice de développement humain : 2010 vs 2022",
    .labels       = {"Norvège","Suisse","Islande","Hong Kong","Australie",
                     "Danemark","Suède","Irlande","Allemagne","Pays-Bas"},
    .values_left  = {0.941,0.934,0.917,0.898,0.929,0.920,0.916,0.908,0.919,0.921},
    .values_right = {0.966,0.962,0.959,0.956,0.946,0.952,0.952,0.950,0.950,0.946},
    .left_label   = "2010",
    .right_label  = "2022",
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/slope.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## Voir aussi

- [Graphique haltère](dumbbell.md) — `sp.build_dumbbell()`
- [Courbe](line.md) — `sp.build_line_chart()`
- [Multi-courbes](multiline.md) — `sp.build_multiline()`

</div>
