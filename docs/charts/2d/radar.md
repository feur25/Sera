# Radar Chart

<div class="lang-en">

## Signature

```python
sp.build_radar_chart(
    title: str,
    axes: list[str],
    series: list[list[float]],
    *,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    fill_opacity: float = 0.25,
    width: int = 600,
    height: int = 500,
    background: str | None = None,
    max_val: float | None = None,
) -> Chart
```

Aliases: `sp.radar`, `sp.radar_chart`

---

## Description

A radar chart (spider chart) maps each dimension to a radial axis emanating from a common center, with observations drawn as filled polygons. Multiple series are overlaid as semi-transparent polygons, making visual comparison of profiles (rather than individual values) the primary focus. All axes use the same scale — either auto-computed from the maximum across all series, or set uniformly via `max_val`. Radar charts work best with 4–8 axes; beyond 8 the polygon shape becomes too complex to read.

**Ideal for:**
- Profiling entities across multiple attributes (player stats, product features, skill assessments)
- Comparing two or three alternatives across the same fixed set of criteria
- Product benchmarking and competitive analysis

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `axes` | `list[str]` | required | Axis names (3–12 recommended) |
| `series` | `list[list[float]]` | required | Each inner list has one value per axis |
| `series_names` | `list[str] \| None` | `None` | Legend labels for each polygon |
| `palette` | `list[int] \| None` | `None` | Colors for each series |
| `fill_opacity` | `float` | `0.25` | Opacity of the polygon fill (0.0–1.0) |
| `width` | `int` | `600` | Canvas width in pixels |
| `height` | `int` | `500` | Canvas height in pixels |
| `background` | `str \| None` | `None` | Background color or `None` = transparent |
| `max_val` | `float \| None` | `None` | Shared maximum for all axes; auto if `None` |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="radar">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('radar','radar-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('radar','radar-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('radar','radar-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('radar','radar-r',this)">R</button>
<button class="sp-tb" onclick="spTab('radar','radar-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('radar','radar-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('radar','radar-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('radar','radar-cpp',this)">C++</button>
</div>
<div id="radar-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.radar(
    title="Player Performance Comparison",
    axes=["Speed", "Strength", "Stamina", "Agility", "Accuracy", "IQ"],
    series=[
        [85, 72, 90, 88, 76, 82],  # Player A
        [70, 90, 75, 65, 88, 78],  # Player B
    ],
    series_names=["Player A", "Player B"],
    fill_opacity=0.3,
)
chart.show()</code></pre></div>
<div id="radar-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.radar({
  title: "Player Performance Comparison",
  axes: ["Speed", "Strength", "Stamina", "Agility", "Accuracy", "IQ"],
  series: [
    [85, 72, 90, 88, 76, 82],
    [70, 90, 75, 65, 88, 78],
  ],
  seriesNames: ["Player A", "Player B"],
  fillOpacity: 0.3,
});
chart.show();</code></pre></div>
<div id="radar-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.radar({
  title: "Player Performance Comparison",
  axes: ["Speed", "Strength", "Stamina", "Agility", "Accuracy", "IQ"],
  series: [
    [85, 72, 90, 88, 76, 82],
    [70, 90, 75, 65, 88, 78],
  ],
  seriesNames: ["Player A", "Player B"],
  fillOpacity: 0.3,
});
chart.show();</code></pre></div>
<div id="radar-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$radar(
  title        = "Player Performance Comparison",
  axes         = c("Speed", "Strength", "Stamina", "Agility", "Accuracy", "IQ"),
  series       = list(
    c(85, 72, 90, 88, 76, 82),
    c(70, 90, 75, 65, 88, 78)
  ),
  series_names = c("Player A", "Player B"),
  fill_opacity = 0.3
)
chart$show()</code></pre></div>
<div id="radar-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
var chart = SeraPlot.radar()
    .title("Player Performance Comparison")
    .axes(List.of("Speed", "Strength", "Stamina", "Agility", "Accuracy", "IQ"))
    .series(List.of(
        List.of(85.0, 72.0, 90.0, 88.0, 76.0, 82.0),
        List.of(70.0, 90.0, 75.0, 65.0, 88.0, 78.0)
    ))
    .seriesNames(List.of("Player A", "Player B"))
    .fillOpacity(0.3)
    .build();
chart.show();</code></pre></div>
<div id="radar-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;
var chart = Sp.Radar(
    title:       "Player Performance Comparison",
    axes:        ["Speed", "Strength", "Stamina", "Agility", "Accuracy", "IQ"],
    series:      [[85, 72, 90, 88, 76, 82], [70, 90, 75, 65, 88, 78]],
    seriesNames: ["Player A", "Player B"],
    fillOpacity: 0.3
);
chart.Show();</code></pre></div>
<div id="radar-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp
val chart = sp.radar(
  title        = "Player Performance Comparison",
  axes         = List("Speed", "Strength", "Stamina", "Agility", "Accuracy", "IQ"),
  series       = List(
    List(85.0, 72.0, 90.0, 88.0, 76.0, 82.0),
    List(70.0, 90.0, 75.0, 65.0, 88.0, 78.0)
  ),
  series_names = List("Player A", "Player B"),
  fill_opacity = 0.3
)
chart.show()</code></pre></div>
<div id="radar-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;
auto chart = sp::radar({
    .title        = "Player Performance Comparison",
    .axes         = {"Speed", "Strength", "Stamina", "Agility", "Accuracy", "IQ"},
    .series       = {
        {85.0, 72.0, 90.0, 88.0, 76.0, 82.0},
        {70.0, 90.0, 75.0, 65.0, 88.0, 78.0},
    },
    .series_names = {"Player A", "Player B"},
    .fill_opacity = 0.3f,
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/radar.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [Parallel Coordinates](parallel.md) — `sp.build_parallel()`
- [Bar Chart](bar.md) — `sp.build_bar_chart()`
- [Heatmap](heatmap.md) — `sp.build_heatmap()`

</div>

<div class="lang-fr">

## Signature

```python
sp.build_radar_chart(
    title: str,
    axes: list[str],
    series: list[list[float]],
    *,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    fill_opacity: float = 0.25,
    width: int = 600,
    height: int = 500,
    background: str | None = None,
    max_val: float | None = None,
) -> Chart
```

Aliases: `sp.radar`, `sp.radar_chart`

---

## Description

Un graphique radar (graphique en araignée) associe chaque dimension à un axe radial émanant d'un centre commun, avec les observations tracées comme des polygones remplis. Plusieurs séries sont superposées sous forme de polygones semi-transparents, faisant de la comparaison de profils (plutôt que de valeurs individuelles) l'objectif principal. Tous les axes utilisent la même échelle — soit calculée automatiquement à partir du maximum de toutes les séries, soit définie uniformément via `max_val`. Les graphiques radar fonctionnent mieux avec 4 à 8 axes.

**Idéal pour :**
- Profiler des entités sur plusieurs attributs (statistiques de joueurs, fonctionnalités de produits)
- Comparer deux ou trois alternatives sur le même ensemble fixe de critères
- Benchmarking de produits et analyse concurrentielle

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `axes` | `list[str]` | requis | Noms des axes (3 à 12 recommandé) |
| `series` | `list[list[float]]` | requis | Chaque liste interne a une valeur par axe |
| `series_names` | `list[str] \| None` | `None` | Étiquettes de légende pour chaque polygone |
| `palette` | `list[int] \| None` | `None` | Couleurs pour chaque série |
| `fill_opacity` | `float` | `0.25` | Opacité du remplissage du polygone (0.0–1.0) |
| `width` | `int` | `600` | Largeur du canvas en pixels |
| `height` | `int` | `500` | Hauteur du canvas en pixels |
| `background` | `str \| None` | `None` | Couleur de fond ou `None` = transparent |
| `max_val` | `float \| None` | `None` | Maximum commun pour tous les axes ; auto si `None` |

---

## Retourne

`Chart`

---

<div class="sp-tabs" id="radar-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('radar-fr','radar-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('radar-fr','radar-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('radar-fr','radar-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('radar-fr','radar-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('radar-fr','radar-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('radar-fr','radar-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('radar-fr','radar-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('radar-fr','radar-fr-cpp',this)">C++</button>
</div>
<div id="radar-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.radar(
    title="Comparaison des performances des joueurs",
    axes=["Vitesse", "Force", "Endurance", "Agilité", "Précision", "QI"],
    series=[
        [85, 72, 90, 88, 76, 82],  # Joueur A
        [70, 90, 75, 65, 88, 78],  # Joueur B
    ],
    series_names=["Joueur A", "Joueur B"],
    fill_opacity=0.3,
)
chart.show()</code></pre></div>
<div id="radar-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");
const chart = sp.radar({
  title: "Comparaison des performances des joueurs",
  axes: ["Vitesse", "Force", "Endurance", "Agilité", "Précision", "QI"],
  series: [
    [85, 72, 90, 88, 76, 82],
    [70, 90, 75, 65, 88, 78],
  ],
  seriesNames: ["Joueur A", "Joueur B"],
  fillOpacity: 0.3,
});
chart.show();</code></pre></div>
<div id="radar-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";
const chart = sp.radar({
  title: "Comparaison des performances des joueurs",
  axes: ["Vitesse", "Force", "Endurance", "Agilité", "Précision", "QI"],
  series: [
    [85, 72, 90, 88, 76, 82],
    [70, 90, 75, 65, 88, 78],
  ],
  seriesNames: ["Joueur A", "Joueur B"],
  fillOpacity: 0.3,
});
chart.show();</code></pre></div>
<div id="radar-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$radar(
  title        = "Comparaison des performances des joueurs",
  axes         = c("Vitesse", "Force", "Endurance", "Agilité", "Précision", "QI"),
  series       = list(
    c(85, 72, 90, 88, 76, 82),
    c(70, 90, 75, 65, 88, 78)
  ),
  series_names = c("Joueur A", "Joueur B"),
  fill_opacity = 0.3
)
chart$show()</code></pre></div>
<div id="radar-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
var chart = SeraPlot.radar()
    .title("Comparaison des performances des joueurs")
    .axes(List.of("Vitesse", "Force", "Endurance", "Agilité", "Précision", "QI"))
    .series(List.of(
        List.of(85.0, 72.0, 90.0, 88.0, 76.0, 82.0),
        List.of(70.0, 90.0, 75.0, 65.0, 88.0, 78.0)
    ))
    .seriesNames(List.of("Joueur A", "Joueur B"))
    .fillOpacity(0.3)
    .build();
chart.show();</code></pre></div>
<div id="radar-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;
var chart = Sp.Radar(
    title:       "Comparaison des performances des joueurs",
    axes:        ["Vitesse", "Force", "Endurance", "Agilité", "Précision", "QI"],
    series:      [[85, 72, 90, 88, 76, 82], [70, 90, 75, 65, 88, 78]],
    seriesNames: ["Joueur A", "Joueur B"],
    fillOpacity: 0.3
);
chart.Show();</code></pre></div>
<div id="radar-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp
val chart = sp.radar(
  title        = "Comparaison des performances des joueurs",
  axes         = List("Vitesse", "Force", "Endurance", "Agilité", "Précision", "QI"),
  series       = List(
    List(85.0, 72.0, 90.0, 88.0, 76.0, 82.0),
    List(70.0, 90.0, 75.0, 65.0, 88.0, 78.0)
  ),
  series_names = List("Joueur A", "Joueur B"),
  fill_opacity = 0.3
)
chart.show()</code></pre></div>
<div id="radar-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;
auto chart = sp::radar({
    .title        = "Comparaison des performances des joueurs",
    .axes         = {"Vitesse", "Force", "Endurance", "Agilité", "Précision", "QI"},
    .series       = {
        {85.0, 72.0, 90.0, 88.0, 76.0, 82.0},
        {70.0, 90.0, 75.0, 65.0, 88.0, 78.0},
    },
    .series_names = {"Joueur A", "Joueur B"},
    .fill_opacity = 0.3f,
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/radar.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## Voir aussi

- [Coordonnées parallèles](parallel.md) — `sp.build_parallel()`
- [Graphique en barres](bar.md) — `sp.build_bar_chart()`
- [Carte de chaleur](heatmap.md) — `sp.build_heatmap()`

</div>
