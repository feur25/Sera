# Gauge Chart

<div class="lang-en">

## Signature

```python
sp.build_gauge(
    title: str,
    value: float,
    *,
    min_val: float = 0.0,
    max_val: float = 100.0,
    thresholds: list[float] | None = None,
    threshold_colors: list[int] | None = None,
    color_hex: int = 0x6366F1,
    width: int = 500,
    height: int = 350,
    show_value: bool = True,
    label: str = "",
) -> Chart
```

Aliases: `sp.gauge`

---

## Description

A gauge chart renders a semicircular speedometer-style arc with a needle pointing to a single numeric value. Colored threshold zones divide the arc into named regions (e.g., safe / warning / danger), making it ideal for dashboards and KPI displays. The chart is rendered as pure SVG with smooth decimal needle positioning. `thresholds` defines the boundary values between color zones; `threshold_colors` provides one color per zone in order (there must be `len(thresholds) + 1` colors).

> **Note:** `value` is a singular `float`, not a list — unlike most other SeraPlot chart functions.

**Ideal for:**
- Real-time system metrics (CPU, memory, latency)
- KPI gauges on executive dashboards
- Progress toward a single target value

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | — | Chart title displayed at the top |
| `value` | `float` | — | Current value to display; must be between `min_val` and `max_val` |
| `min_val` | `float` | `0.0` | Minimum value of the scale |
| `max_val` | `float` | `100.0` | Maximum value of the scale |
| `thresholds` | `list[float] \| None` | `None` | Boundary values between color zones |
| `threshold_colors` | `list[int] \| None` | `None` | Colors per zone; must have `len(thresholds) + 1` entries |
| `color_hex` | `int` | `0x6366F1` | Fallback arc color when no thresholds are defined |
| `width` | `int` | `500` | Canvas width in pixels |
| `height` | `int` | `350` | Canvas height in pixels |
| `show_value` | `bool` | `True` | Display the numeric value below the needle |
| `label` | `str` | `""` | Unit label appended to the displayed value (e.g. `"%"`) |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="gauge">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('gauge','gauge-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('gauge','gauge-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('gauge','gauge-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('gauge','gauge-r',this)">R</button>
<button class="sp-tb" onclick="spTab('gauge','gauge-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('gauge','gauge-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('gauge','gauge-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('gauge','gauge-cpp',this)">C++</button>
</div>
<div id="gauge-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.gauge(
    title="Server CPU load",
    value=72.5,
    min_val=0,
    max_val=100,
    thresholds=[60, 80],
    threshold_colors=[0x22c55e, 0xf59e0b, 0xef4444],
    label="%",
    show_value=True,
)
chart.show()</code></pre></div>
<div id="gauge-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.gauge({
  title: "Server CPU load",
  value: 72.5,
  minVal: 0,
  maxVal: 100,
  thresholds: [60, 80],
  thresholdColors: [0x22c55e, 0xf59e0b, 0xef4444],
  label: "%",
  showValue: true,
});
chart.show();</code></pre></div>
<div id="gauge-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.gauge({
  title: "Server CPU load",
  value: 72.5,
  minVal: 0,
  maxVal: 100,
  thresholds: [60, 80],
  thresholdColors: [0x22c55e, 0xf59e0b, 0xef4444],
  label: "%",
  showValue: true,
});
chart.show();</code></pre></div>
<div id="gauge-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$gauge(
  title = "Server CPU load",
  value = 72.5,
  min_val = 0,
  max_val = 100,
  thresholds = c(60, 80),
  threshold_colors = c(0x22c55e, 0xf59e0b, 0xef4444),
  label = "%",
  show_value = TRUE
)
chart$show()</code></pre></div>
<div id="gauge-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.gauge()
    .title("Server CPU load")
    .value(72.5)
    .minVal(0.0)
    .maxVal(100.0)
    .thresholds(List.of(60.0, 80.0))
    .thresholdColors(List.of(0x22c55e, 0xf59e0b, 0xef4444))
    .label("%")
    .showValue(true)
    .build();
chart.show();</code></pre></div>
<div id="gauge-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Gauge(
    title: "Server CPU load",
    value: 72.5,
    minVal: 0,
    maxVal: 100,
    thresholds: new[]{60.0, 80.0},
    thresholdColors: new[]{0x22c55e, 0xf59e0b, 0xef4444},
    label: "%",
    showValue: true
);
chart.Show();</code></pre></div>
<div id="gauge-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.gauge(
  title = "Server CPU load",
  value = 72.5,
  min_val = 0,
  max_val = 100,
  thresholds = List(60.0, 80.0),
  threshold_colors = List(0x22c55e, 0xf59e0b, 0xef4444),
  label = "%",
  show_value = true
)
chart.show()</code></pre></div>
<div id="gauge-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::gauge({
  .title             = "Server CPU load",
  .value             = 72.5,
  .min_val           = 0,
  .max_val           = 100,
  .thresholds        = {60, 80},
  .threshold_colors  = {0x22c55e, 0xf59e0b, 0xef4444},
  .label             = "%",
  .show_value        = true
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/gauge.html" style="width:100%;height:400px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [bullet.md](bullet.md) — Multiple KPIs with target markers and performance ranges
- [grid.md](grid.md) — Dashboard grid layout for multiple charts

</div>

<div class="lang-fr">

## Signature

```python
sp.build_gauge(
    title: str,
    value: float,
    *,
    min_val: float = 0.0,
    max_val: float = 100.0,
    thresholds: list[float] | None = None,
    threshold_colors: list[int] | None = None,
    color_hex: int = 0x6366F1,
    width: int = 500,
    height: int = 350,
    show_value: bool = True,
    label: str = "",
) -> Chart
```

Aliases: `sp.gauge`

---

## Description

Un graphique de jauge affiche un arc en demi-cercle de style compteur de vitesse avec une aiguille pointant vers une valeur numérique unique. Des zones de couleur seuil divisent l'arc en régions nommées (ex. sûr / avertissement / danger), le rendant idéal pour les tableaux de bord et les affichages de KPI. Le graphique est rendu en SVG pur avec un positionnement précis de l'aiguille. `thresholds` définit les valeurs limites entre les zones de couleur ; `threshold_colors` fournit une couleur par zone dans l'ordre (il doit y avoir `len(thresholds) + 1` couleurs).

> **Remarque :** `value` est un `float` unique, pas une liste — contrairement à la plupart des autres fonctions de graphique SeraPlot.

**Idéal pour :**
- Métriques système en temps réel (CPU, mémoire, latence)
- Jauges KPI sur des tableaux de bord exécutifs
- Progression vers une valeur cible unique

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | — | Titre du graphique affiché en haut |
| `value` | `float` | — | Valeur actuelle à afficher ; doit être entre `min_val` et `max_val` |
| `min_val` | `float` | `0.0` | Valeur minimale de l'échelle |
| `max_val` | `float` | `100.0` | Valeur maximale de l'échelle |
| `thresholds` | `list[float] \| None` | `None` | Valeurs limites entre les zones de couleur |
| `threshold_colors` | `list[int] \| None` | `None` | Couleurs par zone ; doit avoir `len(thresholds) + 1` entrées |
| `color_hex` | `int` | `0x6366F1` | Couleur de l'arc de secours quand aucun seuil n'est défini |
| `width` | `int` | `500` | Largeur du canevas en pixels |
| `height` | `int` | `350` | Hauteur du canevas en pixels |
| `show_value` | `bool` | `True` | Afficher la valeur numérique sous l'aiguille |
| `label` | `str` | `""` | Unité ajoutée à la valeur affichée (ex. `"%"`) |

---

## Retourne

`Chart`

---

<div class="sp-tabs" id="gauge-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('gauge-fr','gauge-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('gauge-fr','gauge-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('gauge-fr','gauge-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('gauge-fr','gauge-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('gauge-fr','gauge-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('gauge-fr','gauge-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('gauge-fr','gauge-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('gauge-fr','gauge-fr-cpp',this)">C++</button>
</div>
<div id="gauge-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.gauge(
    title="Charge CPU du serveur",
    value=72.5,
    min_val=0,
    max_val=100,
    thresholds=[60, 80],
    threshold_colors=[0x22c55e, 0xf59e0b, 0xef4444],
    label="%",
    show_value=True,
)
chart.show()</code></pre></div>
<div id="gauge-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.gauge({
  title: "Charge CPU du serveur",
  value: 72.5,
  minVal: 0,
  maxVal: 100,
  thresholds: [60, 80],
  thresholdColors: [0x22c55e, 0xf59e0b, 0xef4444],
  label: "%",
  showValue: true,
});
chart.show();</code></pre></div>
<div id="gauge-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.gauge({
  title: "Charge CPU du serveur",
  value: 72.5,
  minVal: 0,
  maxVal: 100,
  thresholds: [60, 80],
  thresholdColors: [0x22c55e, 0xf59e0b, 0xef4444],
  label: "%",
  showValue: true,
});
chart.show();</code></pre></div>
<div id="gauge-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$gauge(
  title = "Charge CPU du serveur",
  value = 72.5,
  min_val = 0,
  max_val = 100,
  thresholds = c(60, 80),
  threshold_colors = c(0x22c55e, 0xf59e0b, 0xef4444),
  label = "%",
  show_value = TRUE
)
chart$show()</code></pre></div>
<div id="gauge-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.gauge()
    .title("Charge CPU du serveur")
    .value(72.5)
    .minVal(0.0)
    .maxVal(100.0)
    .thresholds(List.of(60.0, 80.0))
    .thresholdColors(List.of(0x22c55e, 0xf59e0b, 0xef4444))
    .label("%")
    .showValue(true)
    .build();
chart.show();</code></pre></div>
<div id="gauge-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Gauge(
    title: "Charge CPU du serveur",
    value: 72.5,
    minVal: 0,
    maxVal: 100,
    thresholds: new[]{60.0, 80.0},
    thresholdColors: new[]{0x22c55e, 0xf59e0b, 0xef4444},
    label: "%",
    showValue: true
);
chart.Show();</code></pre></div>
<div id="gauge-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.gauge(
  title = "Charge CPU du serveur",
  value = 72.5,
  min_val = 0,
  max_val = 100,
  thresholds = List(60.0, 80.0),
  threshold_colors = List(0x22c55e, 0xf59e0b, 0xef4444),
  label = "%",
  show_value = true
)
chart.show()</code></pre></div>
<div id="gauge-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::gauge({
  .title            = "Charge CPU du serveur",
  .value            = 72.5,
  .min_val          = 0,
  .max_val          = 100,
  .thresholds       = {60, 80},
  .threshold_colors = {0x22c55e, 0xf59e0b, 0xef4444},
  .label            = "%",
  .show_value       = true
});
chart.show();</code></pre></div>
</div>

<iframe src="../../previews/gauge.html" style="width:100%;height:400px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## Voir aussi

- [bullet.md](bullet.md) — Plusieurs KPI avec marqueurs de cibles et plages de performance
- [grid.md](grid.md) — Grille de tableau de bord pour plusieurs graphiques

</div>
