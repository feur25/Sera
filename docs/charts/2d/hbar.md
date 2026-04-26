# Horizontal Bar Chart

<div class="lang-en">

## Signature

```python
sp.build_hbar(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_text: bool = True,
    sort_order: str = "none",
    color_groups: list[str] | None = None,
    palette: list[int] | None = None,
    background: str | None = None,
    width: int = 900,
    height: int = 500,
    x_label: str = "",
    y_label: str = "",
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.hbar`, `sp.barh`, `sp.horizontal_bar`

---

## Description

A horizontal bar chart draws bars along the horizontal axis instead of the vertical axis, which is advantageous when category labels are long text strings that would otherwise overlap or require rotation. It is the canonical chart for ranked lists, leaderboards, and survey responses. By default SeraPlot shows value labels at the end of each bar (`show_text=True`), which eliminates the need for a separate axis scale when the labels themselves communicate magnitude. Sorting with `sort_order="desc"` produces a clean ranked view with minimal extra code.

**Ideal for:**
- Ranked lists and leaderboards with 5–30 items
- Comparing categories with long text labels
- Survey responses and NPS breakdowns

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | — | Chart title displayed at the top |
| `labels` | `list[str]` | — | Category label for each bar |
| `values` | `list[float]` | — | Numeric value for each bar |
| `show_text` | `bool` | `True` | Display the numeric value at the end of each bar |
| `sort_order` | `str` | `"none"` | Sort bars before rendering: `"asc"`, `"desc"`, or `"none"` |
| `color_groups` | `list[str] \| None` | `None` | Categorical group name per bar for auto-coloring |
| `palette` | `list[int] \| None` | `None` | Custom bar colors as hex integers |
| `background` | `str \| None` | `None` | CSS background color override |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `500` | Canvas height in pixels |
| `x_label` | `str` | `""` | Label for the X axis |
| `y_label` | `str` | `""` | Label for the Y axis |
| `no_y_axis` | `bool` | `False` | Hide the Y axis labels |

---

## Returns

`Chart`

---

## Examples

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="hbar">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hbar','hbar-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hbar','hbar-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hbar','hbar-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hbar','hbar-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hbar','hbar-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hbar','hbar-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hbar','hbar-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hbar','hbar-cpp',this)">C++</button>
</div>
<div id="hbar-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.hbar(
    title="Top 10 programming languages by popularity",
    labels=["Python","JavaScript","Java","C#","C++","TypeScript","Go","Rust","Kotlin","Swift"],
    values=[30.3, 22.5, 17.8, 12.4, 10.1, 8.9, 7.2, 6.5, 5.8, 4.9],
    sort_order="desc",
)
chart.show()</code></pre></div>
<div id="hbar-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.hbar({
  title: "Top 10 programming languages by popularity",
  labels: ["Python","JavaScript","Java","C#","C++","TypeScript","Go","Rust","Kotlin","Swift"],
  values: [30.3, 22.5, 17.8, 12.4, 10.1, 8.9, 7.2, 6.5, 5.8, 4.9],
  sortOrder: "desc",
});
chart.show();</code></pre></div>
<div id="hbar-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.hbar({
  title: "Top 10 programming languages by popularity",
  labels: ["Python","JavaScript","Java","C#","C++","TypeScript","Go","Rust","Kotlin","Swift"],
  values: [30.3, 22.5, 17.8, 12.4, 10.1, 8.9, 7.2, 6.5, 5.8, 4.9],
  sortOrder: "desc",
});
chart.show();</code></pre></div>
<div id="hbar-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$hbar(
  title = "Top 10 programming languages by popularity",
  labels = c("Python","JavaScript","Java","C#","C++","TypeScript","Go","Rust","Kotlin","Swift"),
  values = c(30.3, 22.5, 17.8, 12.4, 10.1, 8.9, 7.2, 6.5, 5.8, 4.9),
  sort_order = "desc"
)
chart$show()</code></pre></div>
<div id="hbar-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.hbar()
    .title("Top 10 programming languages by popularity")
    .labels(List.of("Python","JavaScript","Java","C#","C++","TypeScript","Go","Rust","Kotlin","Swift"))
    .values(List.of(30.3, 22.5, 17.8, 12.4, 10.1, 8.9, 7.2, 6.5, 5.8, 4.9))
    .sortOrder("desc")
    .build();
chart.show();</code></pre></div>
<div id="hbar-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Hbar(
    title: "Top 10 programming languages by popularity",
    labels: new[]{"Python","JavaScript","Java","C#","C++","TypeScript","Go","Rust","Kotlin","Swift"},
    values: new[]{30.3, 22.5, 17.8, 12.4, 10.1, 8.9, 7.2, 6.5, 5.8, 4.9},
    sortOrder: "desc"
);
chart.Show();</code></pre></div>
<div id="hbar-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.hbar(
  title = "Top 10 programming languages by popularity",
  labels = List("Python","JavaScript","Java","C#","C++","TypeScript","Go","Rust","Kotlin","Swift"),
  values = List(30.3, 22.5, 17.8, 12.4, 10.1, 8.9, 7.2, 6.5, 5.8, 4.9),
  sort_order = "desc"
)
chart.show()</code></pre></div>
<div id="hbar-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::hbar({
  .title      = "Top 10 programming languages by popularity",
  .labels     = {"Python","JavaScript","Java","C#","C++","TypeScript","Go","Rust","Kotlin","Swift"},
  .values     = {30.3, 22.5, 17.8, 12.4, 10.1, 8.9, 7.2, 6.5, 5.8, 4.9},
  .sort_order = "desc"
});
chart.show();</code></pre></div>
</div>

<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>
<iframe src="../../previews/hbar.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>
</details>

---

## See also

- [bar.md](bar.md) — Vertical bar chart
- [lollipop.md](lollipop.md) — Minimalist bar chart using dot-on-stick encoding
- [dumbbell.md](dumbbell.md) — Two-value comparison per category

</div>

<div class="lang-fr">

## Signature

```python
sp.build_hbar(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_text: bool = True,
    sort_order: str = "none",
    color_groups: list[str] | None = None,
    palette: list[int] | None = None,
    background: str | None = None,
    width: int = 900,
    height: int = 500,
    x_label: str = "",
    y_label: str = "",
    no_y_axis: bool = False,
) -> Chart
```

Aliases: `sp.hbar`, `sp.barh`, `sp.horizontal_bar`

---

## Description

Un graphique à barres horizontales trace les barres le long de l'axe horizontal plutôt que vertical, ce qui est avantageux lorsque les labels de catégories sont de longues chaînes de texte qui se chevaucheraient ou nécessiteraient une rotation. C'est le graphique canonique pour les classements, les tableaux de bord et les réponses à des sondages. Par défaut, SeraPlot affiche les valeurs numériques à l'extrémité de chaque barre (`show_text=True`), ce qui élimine le besoin d'une échelle d'axe séparée. Le tri avec `sort_order="desc"` produit une vue classée claire avec un minimum de code.

**Idéal pour :**
- Les classements et palmarès de 5 à 30 éléments
- La comparaison de catégories avec de longs libellés
- Les réponses à des sondages et les analyses NPS

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | — | Titre du graphique affiché en haut |
| `labels` | `list[str]` | — | Label de catégorie pour chaque barre |
| `values` | `list[float]` | — | Valeur numérique pour chaque barre |
| `show_text` | `bool` | `True` | Afficher la valeur numérique à l'extrémité de chaque barre |
| `sort_order` | `str` | `"none"` | Trier les barres avant le rendu : `"asc"`, `"desc"` ou `"none"` |
| `color_groups` | `list[str] \| None` | `None` | Nom de groupe catégoriel par barre pour la coloration automatique |
| `palette` | `list[int] \| None` | `None` | Couleurs de barres personnalisées en entiers hexadécimaux |
| `background` | `str \| None` | `None` | Couleur de fond CSS |
| `width` | `int` | `900` | Largeur du canevas en pixels |
| `height` | `int` | `500` | Hauteur du canevas en pixels |
| `x_label` | `str` | `""` | Label de l'axe X |
| `y_label` | `str` | `""` | Label de l'axe Y |
| `no_y_axis` | `bool` | `False` | Masquer les labels de l'axe Y |

---

## Retourne

`Chart`

---

## Exemples

<div class="sp-tabs" id="hbar-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('hbar-fr','hbar-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('hbar-fr','hbar-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('hbar-fr','hbar-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('hbar-fr','hbar-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('hbar-fr','hbar-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('hbar-fr','hbar-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('hbar-fr','hbar-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('hbar-fr','hbar-fr-cpp',this)">C++</button>
</div>
<div id="hbar-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.hbar(
    title="Top 10 des langages de programmation par popularité",
    labels=["Python","JavaScript","Java","C#","C++","TypeScript","Go","Rust","Kotlin","Swift"],
    values=[30.3, 22.5, 17.8, 12.4, 10.1, 8.9, 7.2, 6.5, 5.8, 4.9],
    sort_order="desc",
)
chart.show()</code></pre></div>
<div id="hbar-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.hbar({
  title: "Top 10 des langages de programmation par popularité",
  labels: ["Python","JavaScript","Java","C#","C++","TypeScript","Go","Rust","Kotlin","Swift"],
  values: [30.3, 22.5, 17.8, 12.4, 10.1, 8.9, 7.2, 6.5, 5.8, 4.9],
  sortOrder: "desc",
});
chart.show();</code></pre></div>
<div id="hbar-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.hbar({
  title: "Top 10 des langages de programmation par popularité",
  labels: ["Python","JavaScript","Java","C#","C++","TypeScript","Go","Rust","Kotlin","Swift"],
  values: [30.3, 22.5, 17.8, 12.4, 10.1, 8.9, 7.2, 6.5, 5.8, 4.9],
  sortOrder: "desc",
});
chart.show();</code></pre></div>
<div id="hbar-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$hbar(
  title = "Top 10 des langages de programmation par popularité",
  labels = c("Python","JavaScript","Java","C#","C++","TypeScript","Go","Rust","Kotlin","Swift"),
  values = c(30.3, 22.5, 17.8, 12.4, 10.1, 8.9, 7.2, 6.5, 5.8, 4.9),
  sort_order = "desc"
)
chart$show()</code></pre></div>
<div id="hbar-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.hbar()
    .title("Top 10 des langages de programmation par popularité")
    .labels(List.of("Python","JavaScript","Java","C#","C++","TypeScript","Go","Rust","Kotlin","Swift"))
    .values(List.of(30.3, 22.5, 17.8, 12.4, 10.1, 8.9, 7.2, 6.5, 5.8, 4.9))
    .sortOrder("desc")
    .build();
chart.show();</code></pre></div>
<div id="hbar-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Hbar(
    title: "Top 10 des langages de programmation par popularité",
    labels: new[]{"Python","JavaScript","Java","C#","C++","TypeScript","Go","Rust","Kotlin","Swift"},
    values: new[]{30.3, 22.5, 17.8, 12.4, 10.1, 8.9, 7.2, 6.5, 5.8, 4.9},
    sortOrder: "desc"
);
chart.Show();</code></pre></div>
<div id="hbar-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.hbar(
  title = "Top 10 des langages de programmation par popularité",
  labels = List("Python","JavaScript","Java","C#","C++","TypeScript","Go","Rust","Kotlin","Swift"),
  values = List(30.3, 22.5, 17.8, 12.4, 10.1, 8.9, 7.2, 6.5, 5.8, 4.9),
  sort_order = "desc"
)
chart.show()</code></pre></div>
<div id="hbar-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::hbar({
  .title      = "Top 10 des langages de programmation par popularité",
  .labels     = {"Python","JavaScript","Java","C#","C++","TypeScript","Go","Rust","Kotlin","Swift"},
  .values     = {30.3, 22.5, 17.8, 12.4, 10.1, 8.9, 7.2, 6.5, 5.8, 4.9},
  .sort_order = "desc"
});
chart.show();</code></pre></div>
</div>

---

## Voir aussi

- [bar.md](bar.md) — Graphique à barres verticales
- [lollipop.md](lollipop.md) — Graphique à barres minimaliste avec encodage point-sur-tige
- [dumbbell.md](dumbbell.md) — Comparaison de deux valeurs par catégorie

</div>
