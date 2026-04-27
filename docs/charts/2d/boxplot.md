# Box Plot

<div class="lang-en">

## Signature

```python
sp.build_boxplot(
    title: str,
    category_labels: list[str],
    values: list[float],
    *,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    background: str | None = None,
    gridlines: bool = True,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.boxplot`, `sp.box_plot`

---

## Description

A box plot (box-and-whisker chart) summarizes the statistical distribution of a numeric variable per category through five summary statistics: minimum, Q1, median, Q3, and maximum. Outliers beyond 1.5×IQR are plotted as individual points. `values` is a flat list that concatenates all category samples in order; all categories must have the same number of values. SeraPlot computes quantiles in pure Rust without NumPy or R dependencies, making it usable in any environment.

**Ideal for:**
- Comparing statistical distributions across groups (A/B tests, cohorts, conditions)
- Detecting outliers and skew in continuous data per category
- Validating that groups have similar variance before statistical tests

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | — | Chart title displayed at the top |
| `category_labels` | `list[str]` | — | Name of each category group |
| `values` | `list[float]` | — | Flat list of all observations; must contain equal samples per category |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `x_label` | `str` | `""` | Label for the X axis |
| `y_label` | `str` | `""` | Label for the Y axis |
| `color_hex` | `int` | `0x6366F1` | Single box color when no palette is provided |
| `palette` | `list[int] \| None` | `None` | Per-category colors as hex integers |
| `background` | `str \| None` | `None` | CSS background color override |
| `gridlines` | `bool` | `True` | Draw horizontal gridlines |
| `no_x_axis` | `bool` | `False` | Hide the X axis labels |
| `no_y_axis` | `bool` | `False` | Hide the Y axis and its ticks |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="boxplot">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('boxplot','boxplot-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('boxplot','boxplot-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('boxplot','boxplot-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('boxplot','boxplot-r',this)">R</button>
<button class="sp-tb" onclick="spTab('boxplot','boxplot-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('boxplot','boxplot-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('boxplot','boxplot-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('boxplot','boxplot-cpp',this)">C++</button>
</div>
<div id="boxplot-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.boxplot(
    title="Test scores by class",
    category_labels=["Class A", "Class B", "Class C", "Class D"],
    values=[
        72, 75, 68, 80, 85, 71, 66, 78, 82, 74,  # Class A
        65, 70, 73, 68, 72, 69, 71, 75, 67, 70,  # Class B
        85, 88, 90, 82, 87, 91, 84, 89, 86, 83,  # Class C
        55, 62, 58, 60, 65, 57, 61, 59, 63, 56,  # Class D
    ],
    y_label="Score",
)
chart.show()</code></pre></div>
<div id="boxplot-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.boxplot({
  title: "Test scores by class",
  categoryLabels: ["Class A", "Class B", "Class C", "Class D"],
  values: [
    72, 75, 68, 80, 85, 71, 66, 78, 82, 74,
    65, 70, 73, 68, 72, 69, 71, 75, 67, 70,
    85, 88, 90, 82, 87, 91, 84, 89, 86, 83,
    55, 62, 58, 60, 65, 57, 61, 59, 63, 56,
  ],
  yLabel: "Score",
});
chart.show();</code></pre></div>
<div id="boxplot-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.boxplot({
  title: "Test scores by class",
  categoryLabels: ["Class A", "Class B", "Class C", "Class D"],
  values: [
    72, 75, 68, 80, 85, 71, 66, 78, 82, 74,
    65, 70, 73, 68, 72, 69, 71, 75, 67, 70,
    85, 88, 90, 82, 87, 91, 84, 89, 86, 83,
    55, 62, 58, 60, 65, 57, 61, 59, 63, 56,
  ],
  yLabel: "Score",
});
chart.show();</code></pre></div>
<div id="boxplot-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$boxplot(
  title = "Test scores by class",
  category_labels = c("Class A", "Class B", "Class C", "Class D"),
  values = c(
    72, 75, 68, 80, 85, 71, 66, 78, 82, 74,
    65, 70, 73, 68, 72, 69, 71, 75, 67, 70,
    85, 88, 90, 82, 87, 91, 84, 89, 86, 83,
    55, 62, 58, 60, 65, 57, 61, 59, 63, 56
  ),
  y_label = "Score"
)
chart$show()</code></pre></div>
<div id="boxplot-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.boxplot()
    .title("Test scores by class")
    .categoryLabels(List.of("Class A", "Class B", "Class C", "Class D"))
    .values(List.of(
        72.0, 75.0, 68.0, 80.0, 85.0, 71.0, 66.0, 78.0, 82.0, 74.0,
        65.0, 70.0, 73.0, 68.0, 72.0, 69.0, 71.0, 75.0, 67.0, 70.0,
        85.0, 88.0, 90.0, 82.0, 87.0, 91.0, 84.0, 89.0, 86.0, 83.0,
        55.0, 62.0, 58.0, 60.0, 65.0, 57.0, 61.0, 59.0, 63.0, 56.0
    ))
    .yLabel("Score")
    .build();
chart.show();</code></pre></div>
<div id="boxplot-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Boxplot(
    title: "Test scores by class",
    categoryLabels: new[]{"Class A", "Class B", "Class C", "Class D"},
    values: new[]{
        72, 75, 68, 80, 85, 71, 66, 78, 82, 74,
        65, 70, 73, 68, 72, 69, 71, 75, 67, 70,
        85, 88, 90, 82, 87, 91, 84, 89, 86, 83,
        55, 62, 58, 60, 65, 57, 61, 59, 63, 56,
    },
    yLabel: "Score"
);
chart.Show();</code></pre></div>
<div id="boxplot-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.boxplot(
  title = "Test scores by class",
  category_labels = List("Class A", "Class B", "Class C", "Class D"),
  values = List(
    72, 75, 68, 80, 85, 71, 66, 78, 82, 74,
    65, 70, 73, 68, 72, 69, 71, 75, 67, 70,
    85, 88, 90, 82, 87, 91, 84, 89, 86, 83,
    55, 62, 58, 60, 65, 57, 61, 59, 63, 56
  ),
  y_label = "Score"
)
chart.show()</code></pre></div>
<div id="boxplot-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::boxplot({
  .title            = "Test scores by class",
  .category_labels  = {"Class A", "Class B", "Class C", "Class D"},
  .values           = {
    72, 75, 68, 80, 85, 71, 66, 78, 82, 74,
    65, 70, 73, 68, 72, 69, 71, 75, 67, 70,
    85, 88, 90, 82, 87, 91, 84, 89, 86, 83,
    55, 62, 58, 60, 65, 57, 61, 59, 63, 56
  },
  .y_label          = "Score"
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/boxplot.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [violin.md](violin.md) — Box plot combined with a kernel density estimate
- [histogram.md](histogram.md) — Distribution of a single variable
- [kde.md](kde.md) — Smooth density curve

</div>

<div class="lang-fr">

## Signature

```python
sp.build_boxplot(
    title: str,
    category_labels: list[str],
    values: list[float],
    *,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    background: str | None = None,
    gridlines: bool = True,
    no_x_axis: bool = False,
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.boxplot`, `sp.box_plot`

---

## Description

Un diagramme en boîte (boîte à moustaches) résume la distribution statistique d'une variable numérique par catégorie à travers cinq statistiques récapitulatives : minimum, Q1, médiane, Q3 et maximum. Les valeurs aberrantes au-delà de 1,5×IQR sont tracées comme des points individuels. `values` est une liste plate qui concatène tous les échantillons de catégories dans l'ordre ; toutes les catégories doivent avoir le même nombre de valeurs. SeraPlot calcule les quantiles en Rust pur sans dépendances NumPy ou R.

**Idéal pour :**
- Comparer les distributions statistiques entre groupes (tests A/B, cohortes, conditions)
- Détecter les valeurs aberrantes et l'asymétrie dans les données continues par catégorie
- Valider que les groupes ont une variance similaire avant des tests statistiques

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | — | Titre du graphique affiché en haut |
| `category_labels` | `list[str]` | — | Nom de chaque groupe de catégorie |
| `values` | `list[float]` | — | Liste plate de toutes les observations ; doit contenir un nombre égal d'échantillons par catégorie |
| `width` | `int` | `900` | Largeur du canevas en pixels |
| `height` | `int` | `480` | Hauteur du canevas en pixels |
| `x_label` | `str` | `""` | Label de l'axe X |
| `y_label` | `str` | `""` | Label de l'axe Y |
| `color_hex` | `int` | `0x6366F1` | Couleur de boîte unique quand aucune palette n'est fournie |
| `palette` | `list[int] \| None` | `None` | Couleurs par catégorie en entiers hexadécimaux |
| `background` | `str \| None` | `None` | Couleur de fond CSS |
| `gridlines` | `bool` | `True` | Afficher des lignes de grille horizontales |
| `no_x_axis` | `bool` | `False` | Masquer les labels de l'axe X |
| `no_y_axis` | `bool` | `False` | Masquer l'axe Y et ses graduations |

---

## Retourne

`Chart`

---

<div class="sp-tabs" id="boxplot-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('boxplot-fr','boxplot-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('boxplot-fr','boxplot-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('boxplot-fr','boxplot-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('boxplot-fr','boxplot-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('boxplot-fr','boxplot-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('boxplot-fr','boxplot-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('boxplot-fr','boxplot-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('boxplot-fr','boxplot-fr-cpp',this)">C++</button>
</div>
<div id="boxplot-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.boxplot(
    title="Notes d'examen par classe",
    category_labels=["Classe A", "Classe B", "Classe C", "Classe D"],
    values=[
        72, 75, 68, 80, 85, 71, 66, 78, 82, 74,  # Classe A
        65, 70, 73, 68, 72, 69, 71, 75, 67, 70,  # Classe B
        85, 88, 90, 82, 87, 91, 84, 89, 86, 83,  # Classe C
        55, 62, 58, 60, 65, 57, 61, 59, 63, 56,  # Classe D
    ],
    y_label="Note",
)
chart.show()</code></pre></div>
<div id="boxplot-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.boxplot({
  title: "Notes d'examen par classe",
  categoryLabels: ["Classe A", "Classe B", "Classe C", "Classe D"],
  values: [
    72, 75, 68, 80, 85, 71, 66, 78, 82, 74,
    65, 70, 73, 68, 72, 69, 71, 75, 67, 70,
    85, 88, 90, 82, 87, 91, 84, 89, 86, 83,
    55, 62, 58, 60, 65, 57, 61, 59, 63, 56,
  ],
  yLabel: "Note",
});
chart.show();</code></pre></div>
<div id="boxplot-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.boxplot({
  title: "Notes d'examen par classe",
  categoryLabels: ["Classe A", "Classe B", "Classe C", "Classe D"],
  values: [
    72, 75, 68, 80, 85, 71, 66, 78, 82, 74,
    65, 70, 73, 68, 72, 69, 71, 75, 67, 70,
    85, 88, 90, 82, 87, 91, 84, 89, 86, 83,
    55, 62, 58, 60, 65, 57, 61, 59, 63, 56,
  ],
  yLabel: "Note",
});
chart.show();</code></pre></div>
<div id="boxplot-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$boxplot(
  title = "Notes d'examen par classe",
  category_labels = c("Classe A", "Classe B", "Classe C", "Classe D"),
  values = c(
    72, 75, 68, 80, 85, 71, 66, 78, 82, 74,
    65, 70, 73, 68, 72, 69, 71, 75, 67, 70,
    85, 88, 90, 82, 87, 91, 84, 89, 86, 83,
    55, 62, 58, 60, 65, 57, 61, 59, 63, 56
  ),
  y_label = "Note"
)
chart$show()</code></pre></div>
<div id="boxplot-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.boxplot()
    .title("Notes d'examen par classe")
    .categoryLabels(List.of("Classe A", "Classe B", "Classe C", "Classe D"))
    .values(List.of(
        72.0, 75.0, 68.0, 80.0, 85.0, 71.0, 66.0, 78.0, 82.0, 74.0,
        65.0, 70.0, 73.0, 68.0, 72.0, 69.0, 71.0, 75.0, 67.0, 70.0,
        85.0, 88.0, 90.0, 82.0, 87.0, 91.0, 84.0, 89.0, 86.0, 83.0,
        55.0, 62.0, 58.0, 60.0, 65.0, 57.0, 61.0, 59.0, 63.0, 56.0
    ))
    .yLabel("Note")
    .build();
chart.show();</code></pre></div>
<div id="boxplot-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Boxplot(
    title: "Notes d'examen par classe",
    categoryLabels: new[]{"Classe A", "Classe B", "Classe C", "Classe D"},
    values: new[]{
        72, 75, 68, 80, 85, 71, 66, 78, 82, 74,
        65, 70, 73, 68, 72, 69, 71, 75, 67, 70,
        85, 88, 90, 82, 87, 91, 84, 89, 86, 83,
        55, 62, 58, 60, 65, 57, 61, 59, 63, 56,
    },
    yLabel: "Note"
);
chart.Show();</code></pre></div>
<div id="boxplot-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.boxplot(
  title = "Notes d'examen par classe",
  category_labels = List("Classe A", "Classe B", "Classe C", "Classe D"),
  values = List(
    72, 75, 68, 80, 85, 71, 66, 78, 82, 74,
    65, 70, 73, 68, 72, 69, 71, 75, 67, 70,
    85, 88, 90, 82, 87, 91, 84, 89, 86, 83,
    55, 62, 58, 60, 65, 57, 61, 59, 63, 56
  ),
  y_label = "Note"
)
chart.show()</code></pre></div>
<div id="boxplot-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::boxplot({
  .title           = "Notes d'examen par classe",
  .category_labels = {"Classe A", "Classe B", "Classe C", "Classe D"},
  .values          = {
    72, 75, 68, 80, 85, 71, 66, 78, 82, 74,
    65, 70, 73, 68, 72, 69, 71, 75, 67, 70,
    85, 88, 90, 82, 87, 91, 84, 89, 86, 83,
    55, 62, 58, 60, 65, 57, 61, 59, 63, 56
  },
  .y_label         = "Note"
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/boxplot.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## Voir aussi

- [violin.md](violin.md) — Boîte à moustaches combinée avec une estimation par noyau de densité
- [histogram.md](histogram.md) — Distribution d'une seule variable
- [kde.md](kde.md) — Courbe de densité lisse

</div>
