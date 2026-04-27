# Violin Chart

<div class="lang-en">

## Signature

```python
sp.build_violin(
    title: str,
    categories: list[str],
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    bandwidth: float = 1.0,
) -> Chart
```

Aliases: `sp.violin`

---

## Description

A violin chart combines a box plot with a kernel density estimate (KDE) to show both summary statistics and the full probability distribution shape of each group. The mirrored symmetrical shape makes it easy to spot bimodal distributions, long tails, and concentration regions that a box plot would hide. `values` is a flat list concatenating all groups' samples; groups must have equal sample counts. `bandwidth` scales the KDE smoothing kernel — lower values reveal more detail, higher values produce smoother shapes.

**Ideal for:**
- Comparing full distribution shapes across groups
- Detecting bimodality or heavy tails invisible in box plots
- Scientific and academic data analysis requiring more than just quartiles

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | — | Chart title displayed at the top |
| `categories` | `list[str]` | — | Name of each group |
| `values` | `list[float]` | — | Flat list of all observations; must have equal samples per group |
| `color_hex` | `int` | `0x6366F1` | Single violin color when no palette is provided |
| `palette` | `list[int] \| None` | `None` | Per-group colors as hex integers |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `x_label` | `str` | `""` | Label for the X axis |
| `y_label` | `str` | `""` | Label for the Y axis |
| `gridlines` | `bool` | `True` | Draw horizontal gridlines |
| `bandwidth` | `float` | `1.0` | KDE bandwidth multiplier; lower = more detail, higher = smoother |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="violin">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('violin','violin-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('violin','violin-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('violin','violin-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('violin','violin-r',this)">R</button>
<button class="sp-tb" onclick="spTab('violin','violin-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('violin','violin-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('violin','violin-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('violin','violin-cpp',this)">C++</button>
</div>
<div id="violin-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.violin(
    title="Salary distributions by department",
    categories=["Engineering", "Marketing", "Sales"],
    values=[
        85000, 92000, 78000, 105000, 95000, 88000, 110000, 82000,  # Engineering
        55000, 62000, 58000,  65000, 60000, 57000,  63000, 59000,  # Marketing
        70000, 75000, 68000,  80000, 72000, 77000,  65000, 73000,  # Sales
    ],
    y_label="Annual Salary ($)",
)
chart.show()</code></pre></div>
<div id="violin-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const chart = sp.violin({
  title: "Salary distributions by department",
  categories: ["Engineering", "Marketing", "Sales"],
  values: [
    85000, 92000, 78000, 105000, 95000, 88000, 110000, 82000,
    55000, 62000, 58000,  65000, 60000, 57000,  63000, 59000,
    70000, 75000, 68000,  80000, 72000, 77000,  65000, 73000,
  ],
  yLabel: "Annual Salary ($)",
});
chart.show();</code></pre></div>
<div id="violin-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const chart = sp.violin({
  title: "Salary distributions by department",
  categories: ["Engineering", "Marketing", "Sales"],
  values: [
    85000, 92000, 78000, 105000, 95000, 88000, 110000, 82000,
    55000, 62000, 58000,  65000, 60000, 57000,  63000, 59000,
    70000, 75000, 68000,  80000, 72000, 77000,  65000, 73000,
  ],
  yLabel: "Annual Salary ($)",
});
chart.show();</code></pre></div>
<div id="violin-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$violin(
  title = "Salary distributions by department",
  categories = c("Engineering", "Marketing", "Sales"),
  values = c(
    85000, 92000, 78000, 105000, 95000, 88000, 110000, 82000,
    55000, 62000, 58000,  65000, 60000, 57000,  63000, 59000,
    70000, 75000, 68000,  80000, 72000, 77000,  65000, 73000
  ),
  y_label = "Annual Salary ($)"
)
chart$show()</code></pre></div>
<div id="violin-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
var chart = SeraPlot.violin()
    .title("Salary distributions by department")
    .categories(List.of("Engineering", "Marketing", "Sales"))
    .values(List.of(
        85000.0, 92000.0, 78000.0, 105000.0, 95000.0, 88000.0, 110000.0, 82000.0,
        55000.0, 62000.0, 58000.0,  65000.0, 60000.0, 57000.0,  63000.0, 59000.0,
        70000.0, 75000.0, 68000.0,  80000.0, 72000.0, 77000.0,  65000.0, 73000.0
    ))
    .yLabel("Annual Salary ($)")
    .build();
chart.show();</code></pre></div>
<div id="violin-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;
var chart = Sp.Violin(
    title: "Salary distributions by department",
    categories: new[]{"Engineering", "Marketing", "Sales"},
    values: new[]{
        85000, 92000, 78000, 105000, 95000, 88000, 110000, 82000,
        55000, 62000, 58000,  65000, 60000, 57000,  63000, 59000,
        70000, 75000, 68000,  80000, 72000, 77000,  65000, 73000,
    },
    yLabel: "Annual Salary ($)"
);
chart.Show();</code></pre></div>
<div id="violin-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._
val chart = sp.violin(
  title = "Salary distributions by department",
  categories = List("Engineering", "Marketing", "Sales"),
  values = List(
    85000, 92000, 78000, 105000, 95000, 88000, 110000, 82000,
    55000, 62000, 58000,  65000, 60000, 57000,  63000, 59000,
    70000, 75000, 68000,  80000, 72000, 77000,  65000, 73000
  ),
  y_label = "Annual Salary ($)"
)
chart.show()</code></pre></div>
<div id="violin-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;
auto chart = sp::violin({
  .title      = "Salary distributions by department",
  .categories = {"Engineering", "Marketing", "Sales"},
  .values     = {
    85000, 92000, 78000, 105000, 95000, 88000, 110000, 82000,
    55000, 62000, 58000,  65000, 60000, 57000,  63000, 59000,
    70000, 75000, 68000,  80000, 72000, 77000,  65000, 73000
  },
  .y_label    = "Annual Salary ($)"
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/violin.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [boxplot.md](boxplot.md) — Five-number summary without density estimate
- [kde.md](kde.md) — Standalone kernel density estimate
- [ridgeline.md](ridgeline.md) — Stacked KDE curves per group

</div>

<div class="lang-fr">

## Signature

```python
sp.build_violin(
    title: str,
    categories: list[str],
    values: list[float],
    *,
    color_hex: int = 0x6366F1,
    palette: list[int] | None = None,
    width: int = 900,
    height: int = 480,
    x_label: str = "",
    y_label: str = "",
    gridlines: bool = True,
    bandwidth: float = 1.0,
) -> Chart
```

Aliases: `sp.violin`

---

## Description

Un graphique en violon combine une boîte à moustaches avec une estimation par noyau de densité (KDE) pour montrer à la fois les statistiques récapitulatives et la forme complète de la distribution de probabilité de chaque groupe. La forme symétrique en miroir facilite la détection des distributions bimodales, des queues longues et des zones de concentration qu'une boîte à moustaches dissimulerait. `values` est une liste plate concaténant les échantillons de tous les groupes ; les groupes doivent avoir un nombre égal d'observations. `bandwidth` ajuste le noyau de lissage KDE — des valeurs faibles révèlent plus de détails, des valeurs élevées produisent des formes plus lisses.

**Idéal pour :**
- Comparer la forme complète des distributions entre groupes
- Détecter la bimodalité ou les queues lourdes invisibles dans les boîtes à moustaches
- L'analyse de données scientifiques et académiques nécessitant plus que de simples quartiles

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | — | Titre du graphique affiché en haut |
| `categories` | `list[str]` | — | Nom de chaque groupe |
| `values` | `list[float]` | — | Liste plate de toutes les observations ; doit avoir un nombre égal d'échantillons par groupe |
| `color_hex` | `int` | `0x6366F1` | Couleur de violon unique quand aucune palette n'est fournie |
| `palette` | `list[int] \| None` | `None` | Couleurs par groupe en entiers hexadécimaux |
| `width` | `int` | `900` | Largeur du canevas en pixels |
| `height` | `int` | `480` | Hauteur du canevas en pixels |
| `x_label` | `str` | `""` | Label de l'axe X |
| `y_label` | `str` | `""` | Label de l'axe Y |
| `gridlines` | `bool` | `True` | Afficher des lignes de grille horizontales |
| `bandwidth` | `float` | `1.0` | Multiplicateur de bande passante KDE ; faible = plus de détails, élevé = plus lisse |

---

## Retourne

`Chart`

---

<div class="sp-tabs" id="violin-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('violin-fr','violin-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('violin-fr','violin-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('violin-fr','violin-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('violin-fr','violin-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('violin-fr','violin-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('violin-fr','violin-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('violin-fr','violin-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('violin-fr','violin-fr-cpp',this)">C++</button>
</div>
<div id="violin-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.violin(
    title="Distributions salariales par département",
    categories=["Ingénierie", "Marketing", "Ventes"],
    values=[
        85000, 92000, 78000, 105000, 95000, 88000, 110000, 82000,  # Ingénierie
        55000, 62000, 58000,  65000, 60000, 57000,  63000, 59000,  # Marketing
        70000, 75000, 68000,  80000, 72000, 77000,  65000, 73000,  # Ventes
    ],
    y_label="Salaire annuel (€)",
)
chart.show()</code></pre></div>
<div id="violin-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const chart = sp.violin({
  title: "Distributions salariales par département",
  categories: ["Ingénierie", "Marketing", "Ventes"],
  values: [
    85000, 92000, 78000, 105000, 95000, 88000, 110000, 82000,
    55000, 62000, 58000,  65000, 60000, 57000,  63000, 59000,
    70000, 75000, 68000,  80000, 72000, 77000,  65000, 73000,
  ],
  yLabel: "Salaire annuel (€)",
});
chart.show();</code></pre></div>
<div id="violin-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const chart = sp.violin({
  title: "Distributions salariales par département",
  categories: ["Ingénierie", "Marketing", "Ventes"],
  values: [
    85000, 92000, 78000, 105000, 95000, 88000, 110000, 82000,
    55000, 62000, 58000,  65000, 60000, 57000,  63000, 59000,
    70000, 75000, 68000,  80000, 72000, 77000,  65000, 73000,
  ],
  yLabel: "Salaire annuel (€)",
});
chart.show();</code></pre></div>
<div id="violin-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)
chart <- sp$violin(
  title = "Distributions salariales par département",
  categories = c("Ingénierie", "Marketing", "Ventes"),
  values = c(
    85000, 92000, 78000, 105000, 95000, 88000, 110000, 82000,
    55000, 62000, 58000,  65000, 60000, 57000,  63000, 59000,
    70000, 75000, 68000,  80000, 72000, 77000,  65000, 73000
  ),
  y_label = "Salaire annuel (€)"
)
chart$show()</code></pre></div>
<div id="violin-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;
var chart = SeraPlot.violin()
    .title("Distributions salariales par département")
    .categories(List.of("Ingénierie", "Marketing", "Ventes"))
    .values(List.of(
        85000.0, 92000.0, 78000.0, 105000.0, 95000.0, 88000.0, 110000.0, 82000.0,
        55000.0, 62000.0, 58000.0,  65000.0, 60000.0, 57000.0,  63000.0, 59000.0,
        70000.0, 75000.0, 68000.0,  80000.0, 72000.0, 77000.0,  65000.0, 73000.0
    ))
    .yLabel("Salaire annuel (€)")
    .build();
chart.show();</code></pre></div>
<div id="violin-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;
var chart = Sp.Violin(
    title: "Distributions salariales par département",
    categories: new[]{"Ingénierie", "Marketing", "Ventes"},
    values: new[]{
        85000, 92000, 78000, 105000, 95000, 88000, 110000, 82000,
        55000, 62000, 58000,  65000, 60000, 57000,  63000, 59000,
        70000, 75000, 68000,  80000, 72000, 77000,  65000, 73000,
    },
    yLabel: "Salaire annuel (€)"
);
chart.Show();</code></pre></div>
<div id="violin-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._
val chart = sp.violin(
  title = "Distributions salariales par département",
  categories = List("Ingénierie", "Marketing", "Ventes"),
  values = List(
    85000, 92000, 78000, 105000, 95000, 88000, 110000, 82000,
    55000, 62000, 58000,  65000, 60000, 57000,  63000, 59000,
    70000, 75000, 68000,  80000, 72000, 77000,  65000, 73000
  ),
  y_label = "Salaire annuel (€)"
)
chart.show()</code></pre></div>
<div id="violin-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;
auto chart = sp::violin({
  .title      = "Distributions salariales par département",
  .categories = {"Ingénierie", "Marketing", "Ventes"},
  .values     = {
    85000, 92000, 78000, 105000, 95000, 88000, 110000, 82000,
    55000, 62000, 58000,  65000, 60000, 57000,  63000, 59000,
    70000, 75000, 68000,  80000, 72000, 77000,  65000, 73000
  },
  .y_label    = "Salaire annuel (€)"
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/violin.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## Voir aussi

- [boxplot.md](boxplot.md) — Résumé en cinq chiffres sans estimation de densité
- [kde.md](kde.md) — Estimation par noyau de densité autonome
- [ridgeline.md](ridgeline.md) — Courbes KDE empilées par groupe

</div>
