# Funnel Chart

<div class="lang-en">

## Signature

```python
sp.build_funnel(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_text: bool = True,
    width: int = 700,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.funnel`

---

## Description

A funnel chart visualizes progressive reduction through ordered stages, where each bar is centered and its width is proportional to its value relative to the first stage. SeraPlot automatically computes and displays the percentage retained at each stage relative to the first stage. The chart is symmetric, making it immediately clear where the largest drop-offs occur in a sequential process.

**Ideal for:**
- Sales conversion funnels (visitors → leads → opportunities → closed)
- Marketing campaign pipelines
- Process efficiency analysis with ordered stages

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | — | Chart title displayed at the top |
| `labels` | `list[str]` | — | Stage name for each bar |
| `values` | `list[float]` | — | Count or quantity at each stage; must be monotonically decreasing |
| `show_text` | `bool` | `True` | Display value and percentage on each bar |
| `width` | `int` | `700` | Canvas width in pixels |
| `height` | `int` | `480` | Canvas height in pixels |
| `palette` | `list[int] \| None` | `None` | Per-stage colors as hex integers |
| `background` | `str \| None` | `None` | CSS background color override |
| `hover_json` | `str \| None` | `None` | JSON string with extra tooltip metadata |

---

## Returns

`Chart`

---

## Examples

### Sales conversion funnel

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="funnel">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('funnel','funnel-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('funnel','funnel-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('funnel','funnel-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('funnel','funnel-r',this)">R</button>
<button class="sp-tb" onclick="spTab('funnel','funnel-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('funnel','funnel-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('funnel','funnel-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('funnel','funnel-cpp',this)">C++</button>
</div>
<div id="funnel-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.funnel(
    title="Sales conversion funnel",
    labels=["Visitors", "Leads", "Opportunities", "Proposals", "Closed Won"],
    values=[10000, 3200, 1400, 580, 210],
)
chart.show()</code></pre></div>
<div id="funnel-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.funnel({
  title: "Sales conversion funnel",
  labels: ["Visitors", "Leads", "Opportunities", "Proposals", "Closed Won"],
  values: [10000, 3200, 1400, 580, 210],
});
chart.show();</code></pre></div>
<div id="funnel-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.funnel({
  title: "Sales conversion funnel",
  labels: ["Visitors", "Leads", "Opportunities", "Proposals", "Closed Won"],
  values: [10000, 3200, 1400, 580, 210],
});
chart.show();</code></pre></div>
<div id="funnel-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$funnel(
  title = "Sales conversion funnel",
  labels = c("Visitors", "Leads", "Opportunities", "Proposals", "Closed Won"),
  values = c(10000, 3200, 1400, 580, 210)
)
chart$show()</code></pre></div>
<div id="funnel-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.funnel()
    .title("Sales conversion funnel")
    .labels(List.of("Visitors", "Leads", "Opportunities", "Proposals", "Closed Won"))
    .values(List.of(10000.0, 3200.0, 1400.0, 580.0, 210.0))
    .build();
chart.show();</code></pre></div>
<div id="funnel-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Funnel(
    title: "Sales conversion funnel",
    labels: new[]{"Visitors", "Leads", "Opportunities", "Proposals", "Closed Won"},
    values: new[]{10000.0, 3200.0, 1400.0, 580.0, 210.0}
);
chart.Show();</code></pre></div>
<div id="funnel-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.funnel(
  title = "Sales conversion funnel",
  labels = List("Visitors", "Leads", "Opportunities", "Proposals", "Closed Won"),
  values = List(10000, 3200, 1400, 580, 210)
)
chart.show()</code></pre></div>
<div id="funnel-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::funnel({
  .title  = "Sales conversion funnel",
  .labels = {"Visitors", "Leads", "Opportunities", "Proposals", "Closed Won"},
  .values = {10000, 3200, 1400, 580, 210}
});
chart.show();</code></pre></div>
</div>

<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>
<iframe src="../../previews/funnel.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>
</details>

---

## See also

- [waterfall.md](waterfall.md) — Incremental contributions with floating bars
- [bar.md](bar.md) — Simple vertical bar chart

</div>

<div class="lang-fr">

## Signature

```python
sp.build_funnel(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    show_text: bool = True,
    width: int = 700,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    hover_json: str | None = None,
) -> Chart
```

Aliases: `sp.funnel`

---

## Description

Un graphique en entonnoir visualise la réduction progressive à travers des étapes ordonnées, où chaque barre est centrée et sa largeur est proportionnelle à sa valeur par rapport à la première étape. SeraPlot calcule et affiche automatiquement le pourcentage retenu à chaque étape par rapport à la première. Le graphique est symétrique, rendant immédiatement visible les plus grandes chutes dans un processus séquentiel.

**Idéal pour :**
- Entonnoirs de conversion des ventes (visiteurs → leads → opportunités → clients)
- Pipelines de campagnes marketing
- Analyse d'efficacité de processus avec des étapes ordonnées

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | — | Titre du graphique affiché en haut |
| `labels` | `list[str]` | — | Nom de l'étape pour chaque barre |
| `values` | `list[float]` | — | Quantité à chaque étape ; doit être monotoniquement décroissant |
| `show_text` | `bool` | `True` | Afficher la valeur et le pourcentage sur chaque barre |
| `width` | `int` | `700` | Largeur du canevas en pixels |
| `height` | `int` | `480` | Hauteur du canevas en pixels |
| `palette` | `list[int] \| None` | `None` | Couleurs par étape en entiers hexadécimaux |
| `background` | `str \| None` | `None` | Couleur de fond CSS |
| `hover_json` | `str \| None` | `None` | Chaîne JSON avec des métadonnées supplémentaires pour les infobulles |

---

## Retourne

`Chart`

---

## Exemples

### Entonnoir de conversion des ventes

<div class="sp-tabs" id="funnel-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('funnel-fr','funnel-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('funnel-fr','funnel-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('funnel-fr','funnel-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('funnel-fr','funnel-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('funnel-fr','funnel-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('funnel-fr','funnel-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('funnel-fr','funnel-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('funnel-fr','funnel-fr-cpp',this)">C++</button>
</div>
<div id="funnel-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

chart = sp.funnel(
    title="Entonnoir de conversion des ventes",
    labels=["Visiteurs", "Leads", "Opportunités", "Propositions", "Clients gagnés"],
    values=[10000, 3200, 1400, 580, 210],
)
chart.show()</code></pre></div>
<div id="funnel-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');

const chart = sp.funnel({
  title: "Entonnoir de conversion des ventes",
  labels: ["Visiteurs", "Leads", "Opportunités", "Propositions", "Clients gagnés"],
  values: [10000, 3200, 1400, 580, 210],
});
chart.show();</code></pre></div>
<div id="funnel-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';

const chart = sp.funnel({
  title: "Entonnoir de conversion des ventes",
  labels: ["Visiteurs", "Leads", "Opportunités", "Propositions", "Clients gagnés"],
  values: [10000, 3200, 1400, 580, 210],
});
chart.show();</code></pre></div>
<div id="funnel-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

chart <- sp$funnel(
  title = "Entonnoir de conversion des ventes",
  labels = c("Visiteurs", "Leads", "Opportunités", "Propositions", "Clients gagnés"),
  values = c(10000, 3200, 1400, 580, 210)
)
chart$show()</code></pre></div>
<div id="funnel-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var chart = SeraPlot.funnel()
    .title("Entonnoir de conversion des ventes")
    .labels(List.of("Visiteurs", "Leads", "Opportunités", "Propositions", "Clients gagnés"))
    .values(List.of(10000.0, 3200.0, 1400.0, 580.0, 210.0))
    .build();
chart.show();</code></pre></div>
<div id="funnel-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var chart = Sp.Funnel(
    title: "Entonnoir de conversion des ventes",
    labels: new[]{"Visiteurs", "Leads", "Opportunités", "Propositions", "Clients gagnés"},
    values: new[]{10000.0, 3200.0, 1400.0, 580.0, 210.0}
);
chart.Show();</code></pre></div>
<div id="funnel-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import io.seraplot._

val chart = sp.funnel(
  title = "Entonnoir de conversion des ventes",
  labels = List("Visiteurs", "Leads", "Opportunités", "Propositions", "Clients gagnés"),
  values = List(10000, 3200, 1400, 580, 210)
)
chart.show()</code></pre></div>
<div id="funnel-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto chart = sp::funnel({
  .title  = "Entonnoir de conversion des ventes",
  .labels = {"Visiteurs", "Leads", "Opportunités", "Propositions", "Clients gagnés"},
  .values = {10000, 3200, 1400, 580, 210}
});
chart.show();</code></pre></div>
</div>

---

## Voir aussi

- [waterfall.md](waterfall.md) — Contributions incrémentales avec des barres flottantes
- [bar.md](bar.md) — Graphique à barres verticales simple

</div>
