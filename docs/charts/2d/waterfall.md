# Waterfall Chart

<div class="lang-en">

## Signature

```python
sp.build_waterfall(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_text: bool = True,
    color_pos: int = 0x22c55e,
    color_neg: int = 0xef4444,
    color_total: int = 0x6366f1,
    width: int = 900,
    height: int = 480,
    y_label: str = "",
    gridlines: bool = True,
) -> Chart
```

Aliases: `sp.waterfall`

---

## Description

A waterfall chart shows cumulative contributions of sequential positive and negative values, visualized as floating bars that start where the previous bar ended. This makes it the standard chart for P&L decomposition and bridge analysis. Positive values are rendered in green, negative in red, and totals in indigo — all three colors are configurable. Pass `0` as the last value to automatically compute and display the running total as a total bar.

**Ideal for:**
- P&L, EBITDA bridge, and income statement decomposition
- Variance analysis (budget vs. actual)
- Cash flow waterfall (opening → inflows → outflows → closing)

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | — | Chart title displayed at the top |
| `labels` | `list[str]` | — | Label for each bar |
| `values` | `list[float]` | — | Incremental value per bar; `0` triggers automatic total computation |
| `show_text` | `bool` | `True` | Display the numeric value above each bar |
| `color_pos` | `int` | `0x22c55e` | Color for positive increments (default: green) |
| `color_neg` | `int` | `0xef4444` | Color for negative increments (default: red) |
| `color_total` | `int` | `0x6366f1` | Color for total bars (default: indigo) |
| `width` | `int` | `900` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `y_label` | `str` | `""` | Label for the Y axis |
| `gridlines` | `bool` | `True` | Draw horizontal gridlines |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="waterfall">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('waterfall','waterfall-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('waterfall','waterfall-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('waterfall','waterfall-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('waterfall','waterfall-r',this)">R</button>
<button class="sp-tb" onclick="spTab('waterfall','waterfall-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('waterfall','waterfall-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('waterfall','waterfall-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('waterfall','waterfall-cpp',this)">C++</button>
</div>
<div id="waterfall-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.waterfall(
    title="P&L breakdown",
    labels=["Revenue", "COGS", "R&D", "Sales", "G&A", "EBIT"],
    values=[10000, -4200, -1200, -1800, -600, 0],
    y_label="Amount ($k)",
)
chart.show()</code></pre></div>
<div id="waterfall-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.waterfall({
  title: "P&L breakdown",
  labels: ["Revenue", "COGS", "R&D", "Sales", "G&A", "EBIT"],
  values: [10000, -4200, -1200, -1800, -600, 0],
  yLabel: "Amount ($k)",
});
chart.show();</code></pre></div>
<div id="waterfall-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.waterfall({
  title: "P&L breakdown",
  labels: ["Revenue", "COGS", "R&D", "Sales", "G&A", "EBIT"],
  values: [10000, -4200, -1200, -1800, -600, 0],
  yLabel: "Amount ($k)",
});
chart.show();</code></pre></div>
<div id="waterfall-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$waterfall(
  title = "P&L breakdown",
  labels = c("Revenue", "COGS", "R&D", "Sales", "G&A", "EBIT"),
  values = c(10000, -4200, -1200, -1800, -600, 0),
  y_label = "Amount ($k)"
)
chart$show()</code></pre></div>
<div id="waterfall-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.waterfall()
    .title("P&L breakdown")
    .labels(List.of("Revenue", "COGS", "R&D", "Sales", "G&A", "EBIT"))
    .values(List.of(10000.0, -4200.0, -1200.0, -1800.0, -600.0, 0.0))
    .yLabel("Amount ($k)")
    .build();
chart.show();</code></pre></div>
<div id="waterfall-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Waterfall(
    title: "P&L breakdown",
    labels: new[]{"Revenue", "COGS", "R&D", "Sales", "G&A", "EBIT"},
    values: new[]{10000.0, -4200.0, -1200.0, -1800.0, -600.0, 0.0},
    yLabel: "Amount ($k)"
);
chart.Show();</code></pre></div>
<div id="waterfall-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.waterfall(
  title = "P&L breakdown",
  labels = List("Revenue", "COGS", "R&D", "Sales", "G&A", "EBIT"),
  values = List(10000, -4200, -1200, -1800, -600, 0),
  y_label = "Amount ($k)"
)
chart.show()</code></pre></div>
<div id="waterfall-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::waterfall({
  .title   = "P&L breakdown",
  .labels  = {"Revenue", "COGS", "R&D", "Sales", "G&A", "EBIT"},
  .values  = {10000, -4200, -1200, -1800, -600, 0},
  .y_label = "Amount ($k)"
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/waterfall.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [bar.md](bar.md) — Simple vertical bar chart
- [stacked-bar.md](stacked-bar.md) — Stacked contributions per category

</div>

<div class="lang-fr">

## Signature

```python
sp.build_waterfall(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_text: bool = True,
    color_pos: int = 0x22c55e,
    color_neg: int = 0xef4444,
    color_total: int = 0x6366f1,
    width: int = 900,
    height: int = 480,
    y_label: str = "",
    gridlines: bool = True,
) -> Chart
```

Aliases: `sp.waterfall`

---

## Description

Un graphique en cascade affiche les contributions cumulées de valeurs positives et négatives séquentielles, visualisées comme des barres flottantes qui démarrent là où la barre précédente s'est terminée. C'est le graphique standard pour la décomposition des résultats et l'analyse bridge. Les valeurs positives sont rendues en vert, les négatives en rouge et les totaux en indigo — les trois couleurs sont configurables. Passer `0` comme dernière valeur calcule et affiche automatiquement le total cumulé comme barre de total.

**Idéal pour :**
- Décomposition de P&L, bridge EBITDA et compte de résultat
- Analyse des écarts (budget vs réalisé)
- Cascade de trésorerie (ouverture → entrées → sorties → clôture)

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | — | Titre du graphique affiché en haut |
| `labels` | `list[str]` | — | Label pour chaque barre |
| `values` | `list[float]` | — | Valeur incrémentale par barre ; `0` déclenche le calcul automatique du total |
| `show_text` | `bool` | `True` | Afficher la valeur numérique au-dessus de chaque barre |
| `color_pos` | `int` | `0x22c55e` | Couleur pour les incréments positifs (défaut : vert) |
| `color_neg` | `int` | `0xef4444` | Couleur pour les incréments négatifs (défaut : rouge) |
| `color_total` | `int` | `0x6366f1` | Couleur pour les barres de total (défaut : indigo) |
| `width` | `int` | `900` | Largeur du canevas en pixels |
| `height` | `int` | `480` | Hauteur du canevas en pixels |
| `y_label` | `str` | `""` | Label de l'axe Y |
| `gridlines` | `bool` | `True` | Afficher des lignes de grille horizontales |

---

## Retourne

`Chart`

---

<div class="sp-tabs" id="waterfall-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('waterfall-fr','waterfall-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('waterfall-fr','waterfall-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('waterfall-fr','waterfall-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('waterfall-fr','waterfall-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('waterfall-fr','waterfall-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('waterfall-fr','waterfall-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('waterfall-fr','waterfall-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('waterfall-fr','waterfall-fr-cpp',this)">C++</button>
</div>
<div id="waterfall-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.waterfall(
    title="Décomposition du compte de résultat",
    labels=["Chiffre d'affaires", "Coût des ventes", "R&D", "Ventes", "G&A", "EBIT"],
    values=[10000, -4200, -1200, -1800, -600, 0],
    y_label="Montant (k€)",
)
chart.show()</code></pre></div>
<div id="waterfall-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.waterfall({
  title: "Décomposition du compte de résultat",
  labels: ["Chiffre d'affaires", "Coût des ventes", "R&D", "Ventes", "G&A", "EBIT"],
  values: [10000, -4200, -1200, -1800, -600, 0],
  yLabel: "Montant (k€)",
});
chart.show();</code></pre></div>
<div id="waterfall-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.waterfall({
  title: "Décomposition du compte de résultat",
  labels: ["Chiffre d'affaires", "Coût des ventes", "R&D", "Ventes", "G&A", "EBIT"],
  values: [10000, -4200, -1200, -1800, -600, 0],
  yLabel: "Montant (k€)",
});
chart.show();</code></pre></div>
<div id="waterfall-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$waterfall(
  title = "Décomposition du compte de résultat",
  labels = c("Chiffre d'affaires", "Coût des ventes", "R&D", "Ventes", "G&A", "EBIT"),
  values = c(10000, -4200, -1200, -1800, -600, 0),
  y_label = "Montant (k€)"
)
chart$show()</code></pre></div>
<div id="waterfall-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.waterfall()
    .title("Décomposition du compte de résultat")
    .labels(List.of("Chiffre d'affaires", "Coût des ventes", "R&D", "Ventes", "G&A", "EBIT"))
    .values(List.of(10000.0, -4200.0, -1200.0, -1800.0, -600.0, 0.0))
    .yLabel("Montant (k€)")
    .build();
chart.show();</code></pre></div>
<div id="waterfall-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Waterfall(
    title: "Décomposition du compte de résultat",
    labels: new[]{"Chiffre d'affaires", "Coût des ventes", "R&D", "Ventes", "G&A", "EBIT"},
    values: new[]{10000.0, -4200.0, -1200.0, -1800.0, -600.0, 0.0},
    yLabel: "Montant (k€)"
);
chart.Show();</code></pre></div>
<div id="waterfall-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.waterfall(
  title = "Décomposition du compte de résultat",
  labels = List("Chiffre d'affaires", "Coût des ventes", "R&D", "Ventes", "G&A", "EBIT"),
  values = List(10000, -4200, -1200, -1800, -600, 0),
  y_label = "Montant (k€)"
)
chart.show()</code></pre></div>
<div id="waterfall-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::waterfall({
  .title   = "Décomposition du compte de résultat",
  .labels  = {"Chiffre d'affaires", "Coût des ventes", "R&D", "Ventes", "G&A", "EBIT"},
  .values  = {10000, -4200, -1200, -1800, -600, 0},
  .y_label = "Montant (k€)"
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/waterfall.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## Voir aussi

- [bar.md](bar.md) — Graphique à barres verticales simple
- [stacked-bar.md](stacked-bar.md) — Contributions empilées par catégorie

</div>
