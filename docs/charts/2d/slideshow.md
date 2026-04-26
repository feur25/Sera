# Slideshow

<div class="lang-en">

## Signature

```python
sp.build_slideshow(
    charts: list[Chart],
    *,
    title: str = "",
    width: int = 1000,
    height: int = 600,
    background: str | None = None,
    autoplay: bool = False,
    interval_ms: int = 3000,
) -> Chart
```

Aliases: `sp.slideshow`

---

## Description

A slideshow wraps multiple `Chart` objects into a single interactive carousel with Prev/Next navigation buttons. All charts are pre-rendered inline in the output HTML, so the slideshow works fully offline without any external dependencies. When `autoplay` is `True`, slides advance automatically every `interval_ms` milliseconds and loop back to the first after the last. The slideshow is ideal for presentations, dashboards, and reports where multiple charts share equal importance and should be viewed sequentially.

**Ideal for:**
- Business reviews and presentations requiring multiple charts in a single embed
- Report sections where charts follow a narrative sequence
- Dashboard demos cycling through key metrics automatically

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `charts` | `list[Chart]` | required | Ordered list of charts to display |
| `title` | `str` | `""` | Slideshow header title |
| `width` | `int` | `1000` | Canvas width in pixels |
| `height` | `int` | `600` | Canvas height in pixels |
| `background` | `str \| None` | `None` | Background color or `None` = transparent |
| `autoplay` | `bool` | `False` | Auto-advance slides |
| `interval_ms` | `int` | `3000` | Milliseconds between auto-advances |

---

## Returns

`Chart`

---

## Examples

### Q4 2024 Business Review

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>

<div class="sp-tabs" id="slideshow">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('slideshow','slideshow-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('slideshow','slideshow-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('slideshow','slideshow-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('slideshow','slideshow-r',this)">R</button>
<button class="sp-tb" onclick="spTab('slideshow','slideshow-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('slideshow','slideshow-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('slideshow','slideshow-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('slideshow','slideshow-cpp',this)">C++</button>
</div>
<div id="slideshow-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

bar = sp.bar(
    title="Q4 Revenue by Region",
    labels=["EMEA", "APAC", "Americas", "LATAM"],
    values=[420, 380, 610, 195],
)
line = sp.line(
    title="Monthly Active Users",
    x=[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
    y=[12, 15, 14, 18, 22, 25, 24, 28, 31, 29, 34, 38],
    x_label="Month",
    y_label="MAU (K)",
)
donut = sp.donut(
    title="Revenue by Product",
    labels=["Cloud", "SaaS", "Hardware", "Services"],
    values=[42, 31, 15, 12],
)

show = sp.slideshow(
    charts=[bar, line, donut],
    title="Q4 2024 Business Review",
    autoplay=False,
)
show.show()</code></pre></div>
<div id="slideshow-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const bar   = sp.bar({ title: "Q4 Revenue by Region",
    labels: ["EMEA","APAC","Americas","LATAM"], values: [420,380,610,195] });
const line  = sp.line({ title: "Monthly Active Users",
    x: [1,2,3,4,5,6,7,8,9,10,11,12],
    y: [12,15,14,18,22,25,24,28,31,29,34,38] });
const donut = sp.donut({ title: "Revenue by Product",
    labels: ["Cloud","SaaS","Hardware","Services"], values: [42,31,15,12] });

const show = sp.slideshow({
  charts: [bar, line, donut],
  title: "Q4 2024 Business Review",
  autoplay: false,
});
show.show();</code></pre></div>
<div id="slideshow-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const bar   = sp.bar({ title: "Q4 Revenue by Region",
    labels: ["EMEA","APAC","Americas","LATAM"], values: [420,380,610,195] });
const line  = sp.line({ title: "Monthly Active Users",
    x: [1,2,3,4,5,6,7,8,9,10,11,12],
    y: [12,15,14,18,22,25,24,28,31,29,34,38] });
const donut = sp.donut({ title: "Revenue by Product",
    labels: ["Cloud","SaaS","Hardware","Services"], values: [42,31,15,12] });

const show = sp.slideshow({
  charts: [bar, line, donut],
  title: "Q4 2024 Business Review",
  autoplay: false,
});
show.show();</code></pre></div>
<div id="slideshow-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

bar   <- sp$bar(title="Q4 Revenue by Region",
                labels=c("EMEA","APAC","Americas","LATAM"), values=c(420,380,610,195))
line  <- sp$line(title="Monthly Active Users",
                 x=1:12, y=c(12,15,14,18,22,25,24,28,31,29,34,38))
donut <- sp$donut(title="Revenue by Product",
                  labels=c("Cloud","SaaS","Hardware","Services"), values=c(42,31,15,12))

show <- sp$slideshow(
  charts  = list(bar, line, donut),
  title   = "Q4 2024 Business Review",
  autoplay = FALSE
)
show$show()</code></pre></div>
<div id="slideshow-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var bar   = SeraPlot.bar().title("Q4 Revenue by Region")
    .labels(List.of("EMEA","APAC","Americas","LATAM")).values(List.of(420.0,380.0,610.0,195.0)).build();
var line  = SeraPlot.line().title("Monthly Active Users")
    .x(List.of(1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0))
    .y(List.of(12.0,15.0,14.0,18.0,22.0,25.0,24.0,28.0,31.0,29.0,34.0,38.0)).build();
var donut = SeraPlot.donut().title("Revenue by Product")
    .labels(List.of("Cloud","SaaS","Hardware","Services")).values(List.of(42.0,31.0,15.0,12.0)).build();

var show = SeraPlot.slideshow()
    .charts(List.of(bar, line, donut))
    .title("Q4 2024 Business Review")
    .autoplay(false)
    .build();
show.show();</code></pre></div>
<div id="slideshow-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var bar   = Sp.Bar(title: "Q4 Revenue by Region",
    labels: ["EMEA","APAC","Americas","LATAM"], values: [420,380,610,195]);
var line  = Sp.Line(title: "Monthly Active Users",
    x: [1,2,3,4,5,6,7,8,9,10,11,12],
    y: [12,15,14,18,22,25,24,28,31,29,34,38]);
var donut = Sp.Donut(title: "Revenue by Product",
    labels: ["Cloud","SaaS","Hardware","Services"], values: [42,31,15,12]);

var show = Sp.Slideshow(
    charts:   [bar, line, donut],
    title:    "Q4 2024 Business Review",
    autoplay: false
);
show.Show();</code></pre></div>
<div id="slideshow-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val bar   = sp.bar(title = "Q4 Revenue by Region",
  labels = List("EMEA","APAC","Americas","LATAM"), values = List(420.0,380.0,610.0,195.0))
val line  = sp.line(title = "Monthly Active Users",
  x = (1 to 12).map(_.toDouble).toList,
  y = List(12,15,14,18,22,25,24,28,31,29,34,38).map(_.toDouble))
val donut = sp.donut(title = "Revenue by Product",
  labels = List("Cloud","SaaS","Hardware","Services"), values = List(42.0,31.0,15.0,12.0))

val show = sp.slideshow(
  charts   = List(bar, line, donut),
  title    = "Q4 2024 Business Review",
  autoplay = false
)
show.show()</code></pre></div>
<div id="slideshow-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto bar   = sp::bar({.title="Q4 Revenue by Region",
    .labels={"EMEA","APAC","Americas","LATAM"}, .values={420,380,610,195}});
auto line  = sp::line({.title="Monthly Active Users",
    .x={1,2,3,4,5,6,7,8,9,10,11,12},
    .y={12,15,14,18,22,25,24,28,31,29,34,38}});
auto donut = sp::donut({.title="Revenue by Product",
    .labels={"Cloud","SaaS","Hardware","Services"}, .values={42,31,15,12}});

auto show = sp::slideshow({
    .charts   = {bar, line, donut},
    .title    = "Q4 2024 Business Review",
    .autoplay = false,
});
show.show();</code></pre></div>
</div>

<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>
<iframe src="../../previews/slideshow.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>
</details>

### Autoplay kiosk mode

```python
show = sp.slideshow(
    charts=[c1, c2, c3, c4, c5],
    title="Operations Dashboard",
    autoplay=True,
    interval_ms=5000,
    width=1280,
    height=720,
)
show.save("kiosk.html")
```

---

## See also

- [Grid Layout](grid.md) — `sp.build_grid()`

</div>

<div class="lang-fr">

## Signature

```python
sp.build_slideshow(
    charts: list[Chart],
    *,
    title: str = "",
    width: int = 1000,
    height: int = 600,
    background: str | None = None,
    autoplay: bool = False,
    interval_ms: int = 3000,
) -> Chart
```

Aliases: `sp.slideshow`

---

## Description

Un slideshow enveloppe plusieurs objets `Chart` dans un seul carousel interactif avec des boutons de navigation Précédent/Suivant. Tous les graphiques sont pré-rendus en ligne dans le HTML de sortie, le slideshow fonctionne donc entièrement hors ligne. Quand `autoplay` est `True`, les diapositives avancent automatiquement toutes les `interval_ms` millisecondes et rebouclent au début après la dernière.

**Idéal pour :**
- Bilans d'activité et présentations nécessitant plusieurs graphiques dans un seul embed
- Sections de rapport où les graphiques suivent une séquence narrative
- Démonstrations de tableau de bord faisant défiler automatiquement les indicateurs clés

---

## Paramètres

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `charts` | `list[Chart]` | requis | Liste ordonnée des graphiques à afficher |
| `title` | `str` | `""` | Titre d'en-tête du slideshow |
| `width` | `int` | `1000` | Largeur du canvas en pixels |
| `height` | `int` | `600` | Hauteur du canvas en pixels |
| `background` | `str \| None` | `None` | Couleur de fond ou `None` = transparent |
| `autoplay` | `bool` | `False` | Avance automatique des diapositives |
| `interval_ms` | `int` | `3000` | Millisecondes entre les avancements automatiques |

---

## Retourne

`Chart`

---

## Exemples

### Bilan T4 2024

<div class="sp-tabs" id="slideshow-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('slideshow-fr','slideshow-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('slideshow-fr','slideshow-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('slideshow-fr','slideshow-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('slideshow-fr','slideshow-fr-r',this)">R</button>
<button class="sp-tb" onclick="spTab('slideshow-fr','slideshow-fr-java',this)">Java</button>
<button class="sp-tb" onclick="spTab('slideshow-fr','slideshow-fr-cs',this)">C#</button>
<button class="sp-tb" onclick="spTab('slideshow-fr','slideshow-fr-scala',this)">Scala</button>
<button class="sp-tb" onclick="spTab('slideshow-fr','slideshow-fr-cpp',this)">C++</button>
</div>
<div id="slideshow-fr-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp

barre = sp.bar(
    title="Revenus T4 par région",
    labels=["EMEA", "APAC", "Amériques", "LATAM"],
    values=[420, 380, 610, 195],
)
courbe = sp.line(
    title="Utilisateurs actifs mensuels",
    x=[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
    y=[12, 15, 14, 18, 22, 25, 24, 28, 31, 29, 34, 38],
    x_label="Mois",
    y_label="UAM (K)",
)
donut = sp.donut(
    title="Revenus par produit",
    labels=["Cloud", "SaaS", "Matériel", "Services"],
    values=[42, 31, 15, 12],
)

show = sp.slideshow(
    charts=[barre, courbe, donut],
    title="Bilan T4 2024",
    autoplay=False,
)
show.show()</code></pre></div>
<div id="slideshow-fr-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require("seraplot");

const barre  = sp.bar({ title: "Revenus T4 par région",
    labels: ["EMEA","APAC","Amériques","LATAM"], values: [420,380,610,195] });
const courbe = sp.line({ title: "Utilisateurs actifs mensuels",
    x: [1,2,3,4,5,6,7,8,9,10,11,12],
    y: [12,15,14,18,22,25,24,28,31,29,34,38] });
const donut  = sp.donut({ title: "Revenus par produit",
    labels: ["Cloud","SaaS","Matériel","Services"], values: [42,31,15,12] });

const show = sp.slideshow({
  charts: [barre, courbe, donut],
  title: "Bilan T4 2024",
  autoplay: false,
});
show.show();</code></pre></div>
<div id="slideshow-fr-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from "seraplot";

const barre  = sp.bar({ title: "Revenus T4 par région",
    labels: ["EMEA","APAC","Amériques","LATAM"], values: [420,380,610,195] });
const courbe = sp.line({ title: "Utilisateurs actifs mensuels",
    x: [1,2,3,4,5,6,7,8,9,10,11,12],
    y: [12,15,14,18,22,25,24,28,31,29,34,38] });
const donut  = sp.donut({ title: "Revenus par produit",
    labels: ["Cloud","SaaS","Matériel","Services"], values: [42,31,15,12] });

const show = sp.slideshow({
  charts: [barre, courbe, donut],
  title: "Bilan T4 2024",
  autoplay: false,
});
show.show();</code></pre></div>
<div id="slideshow-fr-r" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-r">library(seraplot)

barre  <- sp$bar(title="Revenus T4 par région",
                 labels=c("EMEA","APAC","Amériques","LATAM"), values=c(420,380,610,195))
courbe <- sp$line(title="Utilisateurs actifs mensuels",
                  x=1:12, y=c(12,15,14,18,22,25,24,28,31,29,34,38))
donut  <- sp$donut(title="Revenus par produit",
                   labels=c("Cloud","SaaS","Matériel","Services"), values=c(42,31,15,12))

show <- sp$slideshow(
  charts   = list(barre, courbe, donut),
  title    = "Bilan T4 2024",
  autoplay = FALSE
)
show$show()</code></pre></div>
<div id="slideshow-fr-java" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-java">import io.seraplot.SeraPlot;
import java.util.List;

var barre  = SeraPlot.bar().title("Revenus T4 par région")
    .labels(List.of("EMEA","APAC","Amériques","LATAM")).values(List.of(420.0,380.0,610.0,195.0)).build();
var courbe = SeraPlot.line().title("Utilisateurs actifs mensuels")
    .x(List.of(1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0))
    .y(List.of(12.0,15.0,14.0,18.0,22.0,25.0,24.0,28.0,31.0,29.0,34.0,38.0)).build();
var donut  = SeraPlot.donut().title("Revenus par produit")
    .labels(List.of("Cloud","SaaS","Matériel","Services")).values(List.of(42.0,31.0,15.0,12.0)).build();

var show = SeraPlot.slideshow()
    .charts(List.of(barre, courbe, donut))
    .title("Bilan T4 2024")
    .autoplay(false)
    .build();
show.show();</code></pre></div>
<div id="slideshow-fr-cs" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-csharp">using SeraPlot;

var barre  = Sp.Bar(title: "Revenus T4 par région",
    labels: ["EMEA","APAC","Amériques","LATAM"], values: [420,380,610,195]);
var courbe = Sp.Line(title: "Utilisateurs actifs mensuels",
    x: [1,2,3,4,5,6,7,8,9,10,11,12],
    y: [12,15,14,18,22,25,24,28,31,29,34,38]);
var donut  = Sp.Donut(title: "Revenus par produit",
    labels: ["Cloud","SaaS","Matériel","Services"], values: [42,31,15,12]);

var show = Sp.Slideshow(
    charts:   [barre, courbe, donut],
    title:    "Bilan T4 2024",
    autoplay: false
);
show.Show();</code></pre></div>
<div id="slideshow-fr-scala" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-scala">import seraplot.sp

val barre  = sp.bar(title = "Revenus T4 par région",
  labels = List("EMEA","APAC","Amériques","LATAM"), values = List(420.0,380.0,610.0,195.0))
val courbe = sp.line(title = "Utilisateurs actifs mensuels",
  x = (1 to 12).map(_.toDouble).toList,
  y = List(12,15,14,18,22,25,24,28,31,29,34,38).map(_.toDouble))
val donut  = sp.donut(title = "Revenus par produit",
  labels = List("Cloud","SaaS","Matériel","Services"), values = List(42.0,31.0,15.0,12.0))

val show = sp.slideshow(
  charts   = List(barre, courbe, donut),
  title    = "Bilan T4 2024",
  autoplay = false
)
show.show()</code></pre></div>
<div id="slideshow-fr-cpp" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-cpp">#include &lt;seraplot/seraplot.hpp&gt;

auto barre  = sp::bar({.title="Revenus T4 par région",
    .labels={"EMEA","APAC","Amériques","LATAM"}, .values={420,380,610,195}});
auto courbe = sp::line({.title="Utilisateurs actifs mensuels",
    .x={1,2,3,4,5,6,7,8,9,10,11,12},
    .y={12,15,14,18,22,25,24,28,31,29,34,38}});
auto donut  = sp::donut({.title="Revenus par produit",
    .labels={"Cloud","SaaS","Matériel","Services"}, .values={42,31,15,12}});

auto show = sp::slideshow({
    .charts   = {barre, courbe, donut},
    .title    = "Bilan T4 2024",
    .autoplay = false,
});
show.show();</code></pre></div>
</div>

---

## Voir aussi

- [Grille de graphiques](grid.md) — `sp.build_grid()`

</div>
