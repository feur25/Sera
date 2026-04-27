# Area Chart

<div class="lang-en">

## Signature

```python
sp.build_area_chart(
    title: str,
    x_labels: list[str],
    series_values: list[list[float]],
    *,
    stacked: bool = False,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    background: str | None = None,
    legend_position: str = "top",
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.area`, `sp.area_chart`

---

## Description

An area chart is a line chart where the region between the line and the horizontal axis is filled with a semi-transparent color, emphasizing the total magnitude of values over time. When `stacked=True`, multiple series are layered on top of each other so their cumulative total fills the canvas — ideal for part-to-whole analysis. Each inner list in `series_values` represents one series; all inner lists must have the same length as `x_labels`. SeraPlot automatically assigns colors from the default palette or from a custom one.

**Ideal for:**
- Showing total volume changing over time
- Visualizing part-to-whole composition across time with stacked mode
- Comparing two or three series with overlapping fills

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | — | Chart title displayed at the top |
| `x_labels` | `list[str]` | — | Ordered X axis labels shared by all series |
| `series_values` | `list[list[float]]` | — | One inner list per series; all must have the same length as `x_labels` |
| `stacked` | `bool` | `False` | Stack series cumulatively instead of overlapping |
| `series_names` | `list[str] \| None` | `None` | Legend label for each series |
| `palette` | `list[int] \| None` | `None` | Custom series colors as hex integers |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `x_label` | `str` | `""` | Label for the X axis |
| `y_label` | `str` | `""` | Label for the Y axis |
| `gridlines` | `bool` | `True` | Draw horizontal gridlines |
| `background` | `str \| None` | `None` | CSS background color override |
| `legend_position` | `str` | `"top"` | Legend placement: `"top"`, `"right"`, `"bottom"`, or `"left"` |
| `hover_json` | `str \| None` | `None` | JSON string for custom tooltip data |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="area">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('area','area-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('area','area-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('area','area-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('area','area-r',this)">R</button>
<button class="sp-tb" onclick="spTab('area','area-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('area','area-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('area','area-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('area','area-cpp',this)">C++</button>
</div>
<div id="area-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.area(
    title="Website traffic by source",
    x_labels=["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
    series_values=[
        [1200, 1400, 1300, 1600, 1800, 2100],
        [800,  900,  950,  1100, 1200, 1400],
    ],
    series_names=["Organic", "Paid"],
)
chart.show()</code></pre></div>
<div id="area-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.area({
  title: "Website traffic by source",
  xLabels: ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
  seriesValues: [
    [1200, 1400, 1300, 1600, 1800, 2100],
    [800,  900,  950,  1100, 1200, 1400],
  ],
  seriesNames: ["Organic", "Paid"],
});
chart.show();</code></pre></div>
<div id="area-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.area({
  title: "Website traffic by source",
  xLabels: ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
  seriesValues: [
    [1200, 1400, 1300, 1600, 1800, 2100],
    [800,  900,  950,  1100, 1200, 1400],
  ],
  seriesNames: ["Organic", "Paid"],
});
chart.show();</code></pre></div>
<div id="area-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$area(
  title = "Website traffic by source",
  x_labels = c("Jan", "Feb", "Mar", "Apr", "May", "Jun"),
  series_values = list(
    c(1200, 1400, 1300, 1600, 1800, 2100),
    c(800,  900,  950,  1100, 1200, 1400)
  ),
  series_names = c("Organic", "Paid")
)
chart$show()</code></pre></div>
<div id="area-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.area()
    .title("Website traffic by source")
    .xLabels(List.of("Jan", "Feb", "Mar", "Apr", "May", "Jun"))
    .seriesValues(List.of(
        List.of(1200.0, 1400.0, 1300.0, 1600.0, 1800.0, 2100.0),
        List.of(800.0,  900.0,  950.0,  1100.0, 1200.0, 1400.0)
    ))
    .seriesNames(List.of("Organic", "Paid"))
    .build();
chart.show();</code></pre></div>
<div id="area-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Area(
    title: "Website traffic by source",
    xLabels: new[]{"Jan", "Feb", "Mar", "Apr", "May", "Jun"},
    seriesValues: new[]{
        new[]{1200.0, 1400.0, 1300.0, 1600.0, 1800.0, 2100.0},
        new[]{800.0,  900.0,  950.0,  1100.0, 1200.0, 1400.0},
    },
    seriesNames: new[]{"Organic", "Paid"}
);
chart.Show();</code></pre></div>
<div id="area-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.area(
  title = "Website traffic by source",
  x_labels = List("Jan", "Feb", "Mar", "Apr", "May", "Jun"),
  series_values = List(
    List(1200.0, 1400.0, 1300.0, 1600.0, 1800.0, 2100.0),
    List(800.0,  900.0,  950.0,  1100.0, 1200.0, 1400.0)
  ),
  series_names = List("Organic", "Paid")
)
chart.show()</code></pre></div>
<div id="area-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::area({
  .title         = "Website traffic by source",
  .x_labels      = {"Jan", "Feb", "Mar", "Apr", "May", "Jun"},
  .series_values = {
    {1200, 1400, 1300, 1600, 1800, 2100},
    {800,  900,  950,  1100, 1200, 1400}
  },
  .series_names  = {"Organic", "Paid"}
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/area.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [line.md](line.md) — Unfilled line chart for a single series
- [multiline.md](multiline.md) — Multiple lines without area fills
- [waterfall.md](waterfall.md) — Incremental contribution chart

</div>

<div class="lang-fr">

## Signature

```python
sp.build_area_chart(
    title: str,
    x_labels: list[str],
    series_values: list[list[float]],
    *,
    stacked: bool = False,
    series_names: list[str] | None = None,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    background: str | None = None,
    legend_position: str = "top",
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.area`, `sp.area_chart`

---

## Description

Un graphique de zone est un graphique en ligne dont la région entre la courbe et l'axe horizontal est remplie d'une couleur semi-transparente, mettant en valeur la magnitude totale des valeurs dans le temps. Lorsque `stacked=True`, plusieurs séries sont superposées de sorte que leur total cumulatif remplit le canevas — idéal pour l'analyse parties-tout. Chaque liste interne de `series_values` représente une série ; toutes les listes internes doivent avoir la même longueur que `x_labels`. SeraPlot assigne automatiquement les couleurs de la palette par défaut ou d'une palette personnalisée.

**Idéal pour :**
- Montrer l'évolution du volume total dans le temps
- Visualiser la composition parties-tout dans le temps avec le mode empilé
- Comparer deux ou trois séries avec des zones superposées

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | — | Titre du graphique affiché en haut |
| `x_labels` | `list[str]` | — | Labels de l'axe X partagés par toutes les séries |
| `series_values` | `list[list[float]]` | — | Une liste interne par série ; toutes doivent avoir la même longueur que `x_labels` |
| `stacked` | `bool` | `False` | Empiler les séries de façon cumulative au lieu de les superposer |
| `series_names` | `list[str] \| None` | `None` | Label de légende pour chaque série |
| `palette` | `list[int] \| None` | `None` | Couleurs de série personnalisées en entiers hexadécimaux |
| `width` | `int` | `900` | Largeur du canevas en pixels |
| `height` | `int` | `480` | Hauteur du canevas en pixels |
| `x_label` | `str` | `""` | Label de l'axe X |
| `y_label` | `str` | `""` | Label de l'axe Y |
| `gridlines` | `bool` | `True` | Afficher des lignes de grille horizontales |
| `background` | `str \| None` | `None` | Couleur de fond CSS |
| `legend_position` | `str` | `"top"` | Position de la légende : `"top"`, `"right"`, `"bottom"` ou `"left"` |
| `hover_json` | `str \| None` | `None` | JSON pour les données d'info-bulle personnalisées |

---

## Retourne

`Chart`

---

<div class="sp-tabs" id="area-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('area-fr','area-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('area-fr','area-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('area-fr','area-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('area-fr','area-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('area-fr','area-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('area-fr','area-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('area-fr','area-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('area-fr','area-fr-cpp',this)">C++</button>
</div>
<div id="area-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.area(
    title="Trafic web par source",
    x_labels=["Jan", "Fév", "Mar", "Avr", "Mai", "Juin"],
    series_values=[
        [1200, 1400, 1300, 1600, 1800, 2100],
        [800,  900,  950,  1100, 1200, 1400],
    ],
    series_names=["Organique", "Payant"],
)
chart.show()</code></pre></div>
<div id="area-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.area({
  title: "Trafic web par source",
  xLabels: ["Jan", "Fév", "Mar", "Avr", "Mai", "Juin"],
  seriesValues: [
    [1200, 1400, 1300, 1600, 1800, 2100],
    [800,  900,  950,  1100, 1200, 1400],
  ],
  seriesNames: ["Organique", "Payant"],
});
chart.show();</code></pre></div>
<div id="area-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.area({
  title: "Trafic web par source",
  xLabels: ["Jan", "Fév", "Mar", "Avr", "Mai", "Juin"],
  seriesValues: [
    [1200, 1400, 1300, 1600, 1800, 2100],
    [800,  900,  950,  1100, 1200, 1400],
  ],
  seriesNames: ["Organique", "Payant"],
});
chart.show();</code></pre></div>
<div id="area-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$area(
  title = "Trafic web par source",
  x_labels = c("Jan", "Fév", "Mar", "Avr", "Mai", "Juin"),
  series_values = list(
    c(1200, 1400, 1300, 1600, 1800, 2100),
    c(800,  900,  950,  1100, 1200, 1400)
  ),
  series_names = c("Organique", "Payant")
)
chart$show()</code></pre></div>
<div id="area-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.area()
    .title("Trafic web par source")
    .xLabels(List.of("Jan", "Fév", "Mar", "Avr", "Mai", "Juin"))
    .seriesValues(List.of(
        List.of(1200.0, 1400.0, 1300.0, 1600.0, 1800.0, 2100.0),
        List.of(800.0,  900.0,  950.0,  1100.0, 1200.0, 1400.0)
    ))
    .seriesNames(List.of("Organique", "Payant"))
    .build();
chart.show();</code></pre></div>
<div id="area-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Area(
    title: "Trafic web par source",
    xLabels: new[]{"Jan", "Fév", "Mar", "Avr", "Mai", "Juin"},
    seriesValues: new[]{
        new[]{1200.0, 1400.0, 1300.0, 1600.0, 1800.0, 2100.0},
        new[]{800.0,  900.0,  950.0,  1100.0, 1200.0, 1400.0},
    },
    seriesNames: new[]{"Organique", "Payant"}
);
chart.Show();</code></pre></div>
<div id="area-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.area(
  title = "Trafic web par source",
  x_labels = List("Jan", "Fév", "Mar", "Avr", "Mai", "Juin"),
  series_values = List(
    List(1200.0, 1400.0, 1300.0, 1600.0, 1800.0, 2100.0),
    List(800.0,  900.0,  950.0,  1100.0, 1200.0, 1400.0)
  ),
  series_names = List("Organique", "Payant")
)
chart.show()</code></pre></div>
<div id="area-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::area({
  .title         = "Trafic web par source",
  .x_labels      = {"Jan", "Fév", "Mar", "Avr", "Mai", "Juin"},
  .series_values = {
    {1200, 1400, 1300, 1600, 1800, 2100},
    {800,  900,  950,  1100, 1200, 1400}
  },
  .series_names  = {"Organique", "Payant"}
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/area.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## Voir aussi

- [line.md](line.md) — Graphique en ligne sans remplissage pour une seule série
- [multiline.md](multiline.md) — Plusieurs lignes sans zone remplie
- [waterfall.md](waterfall.md) — Graphique à contribution incrémentale

</div>
