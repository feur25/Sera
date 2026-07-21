# Quick Start

<style>
.sp-tabs{border:1px solid var(--sp-border);border-radius:8px;overflow:hidden;margin:1em 0 1.6em}
.sp-tb{padding:9px 18px;border:none;background:none;color:var(--sp-text-muted);cursor:pointer;font-size:12.5px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

<div class="lang-en">

<div class="qs-hero">
<h2>Your first chart in under a minute</h2>
<p>SeraPlot is a unified plotting library that ships the same API to nine languages. Pick your stack, install one package, and render an interactive chart with three lines of code.</p>
<div class="qs-pills">
<span class="qs-pill">⚡ Native Rust core</span>
<span class="qs-pill">📊 60+ chart types</span>
<span class="qs-pill">🌐 9 languages</span>
<span class="qs-pill">🪶 Zero JS dependencies</span>
</div>
</div>

<div class="qs-step"><div class="qs-step-num">1</div><h3>Install</h3></div>

<div class="sp-tabs" id="qs-install">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('qs-install','qs-install-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('qs-install','qs-install-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('qs-install','qs-install-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('qs-install','qs-install-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('qs-install','qs-install-r',this)">R</button>
</div>
<div id="qs-install-py" class="sp-tc sp-on"><pre><code class="language-bash">pip install seraplot</code></pre></div>
<div id="qs-install-js" class="sp-tc"><pre><code class="language-bash">npm install seraplot</code></pre></div>
<div id="qs-install-ts" class="sp-tc"><pre><code class="language-bash">npm install seraplot
npm install --save-dev @types/seraplot</code></pre></div>
<div id="qs-install-rust" class="sp-tc"><pre><code class="language-bash">cargo add seraplot</code></pre></div>
<div id="qs-install-r" class="sp-tc"><pre><code class="language-r">install.packages("seraplot")</code></pre></div>
</div>

<div class="qs-step"><div class="qs-step-num">2</div><h3>Build a chart</h3></div>

<div class="sp-tabs" id="qs-build">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('qs-build','qs-build-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('qs-build','qs-build-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('qs-build','qs-build-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('qs-build','qs-build-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('qs-build','qs-build-r',this)">R</button>
</div>
<div id="qs-build-py" class="sp-tc sp-on"><pre><code class="language-python">import seraplot as sp

chart = sp.bar(
    title="Sales by Region",
    labels=["North", "South", "East", "West"],
    values=[120, 85, 200, 140],
    gridlines=True,
    show_text=True,
)
chart.show()</code></pre></div>
<div id="qs-build-js" class="sp-tc"><pre><code class="language-javascript">const sp = require("seraplot");

const chart = sp.bar({
  title: "Sales by Region",
  labels: ["North", "South", "East", "West"],
  values: [120, 85, 200, 140],
  gridlines: true,
  show_text: true,
});
chart.show();</code></pre></div>
<div id="qs-build-ts" class="sp-tc"><pre><code class="language-typescript">import * as sp from "seraplot";

const chart = sp.bar({
  title: "Sales by Region",
  labels: ["North", "South", "East", "West"],
  values: [120, 85, 200, 140],
  gridlines: true,
  show_text: true,
});
chart.show();</code></pre></div>
<div id="qs-build-rust" class="sp-tc"><pre><code class="language-rust">// Cargo.toml: [dependencies] seraplot = "2"
use seraplot::Chart;

fn main() {
    let chart = Chart::bar()
        .title("Sales by Region")
        .labels(["North", "South", "East", "West"])
        .values([120.0, 85.0, 200.0, 140.0])
        .gridlines(true)
        .show_text(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="qs-build-r" class="sp-tc"><pre><code class="language-r">library(seraplot)

chart <- sp_bar(
  title = "Sales by Region",
  labels = c("North", "South", "East", "West"),
  values = c(120, 85, 200, 140),
  gridlines = TRUE,
  show_text = TRUE
)
chart$show()</code></pre></div>
</div>

<div class="qs-tip"><strong>💡 Jupyter / notebook:</strong> the chart renders inline automatically — no <code>.show()</code> needed.</div>

<div class="qs-step"><div class="qs-step-num">3</div><h3>Save or export</h3></div>

<div class="sp-tabs" id="qs-save">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('qs-save','qs-save-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('qs-save','qs-save-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('qs-save','qs-save-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('qs-save','qs-save-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('qs-save','qs-save-r',this)">R</button>
</div>
<div id="qs-save-py" class="sp-tc sp-on"><pre><code class="language-python">chart.save("sales.html")    # standalone HTML
chart.save("sales.png")     # raster
chart.save("sales.svg")     # vector</code></pre></div>
<div id="qs-save-js" class="sp-tc"><pre><code class="language-javascript">chart.save("sales.html");
chart.save("sales.png");
chart.save("sales.svg");</code></pre></div>
<div id="qs-save-ts" class="sp-tc"><pre><code class="language-typescript">chart.save("sales.html");
chart.save("sales.png");
chart.save("sales.svg");</code></pre></div>
<div id="qs-save-rust" class="sp-tc"><pre><code class="language-rust">chart.save("sales.html")?;
chart.save("sales.png")?;
chart.save("sales.svg")?;</code></pre></div>
<div id="qs-save-r" class="sp-tc"><pre><code class="language-r">chart$save("sales.html")
chart$save("sales.png")
chart$save("sales.svg")</code></pre></div>
</div>

## Where to next?

<div class="qs-grid">
<div class="qs-card"><h4>📊 Chart catalog</h4><p>Browse <a href="../charts/2d/bar.html">bar</a>, <a href="../charts/2d/line.html">line</a>, <a href="../charts/2d/scatter.html">scatter</a>, heatmaps and 60+ more chart types — each with copy-paste examples in nine languages.</p></div>
<div class="qs-card"><h4>🤖 Machine learning</h4><p>Train, fit and visualise <a href="../ml/index.html">scikit-learn-style models</a> directly on chart data: regression, clustering, trees, SVM and more.</p></div>
<div class="qs-card"><h4>🎨 Themes &amp; palettes</h4><p>Customise colors with <a href="../config/palette.html">palettes</a>, <a href="../config/themes.html">themes</a> and per-chart <a href="../config/background.html">backgrounds</a>.</p></div>
<div class="qs-card"><h4>⚡ Streaming &amp; big data</h4><p>Render millions of points with <a href="../config/downsample.html">downsampling</a>, <a href="../config/streaming.html">live streaming</a> and <a href="../config/diff.html">diff updates</a>.</p></div>
</div>

</div>

<div class="lang-fr">

<div class="qs-hero">
<h2>Votre premier graphique en moins d'une minute</h2>
<p>SeraPlot est une bibliothèque graphique unifiée qui expose la même API dans neuf langages. Choisissez votre stack, installez un seul paquet, et générez un graphique interactif en trois lignes.</p>
<div class="qs-pills">
<span class="qs-pill">⚡ Cœur Rust natif</span>
<span class="qs-pill">📊 60+ types de graphiques</span>
<span class="qs-pill">🌐 9 langages</span>
<span class="qs-pill">🪶 Zéro dépendance JS</span>
</div>
</div>

<div class="qs-step"><div class="qs-step-num">1</div><h3>Installation</h3></div>

<div class="sp-tabs" id="qs-install-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('qs-install-fr','qs-install-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('qs-install-fr','qs-install-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('qs-install-fr','qs-install-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('qs-install-fr','qs-install-fr-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('qs-install-fr','qs-install-fr-r',this)">R</button>
</div>
<div id="qs-install-fr-py" class="sp-tc sp-on"><pre><code class="language-bash">pip install seraplot</code></pre></div>
<div id="qs-install-fr-js" class="sp-tc"><pre><code class="language-bash">npm install seraplot</code></pre></div>
<div id="qs-install-fr-ts" class="sp-tc"><pre><code class="language-bash">npm install seraplot
npm install --save-dev @types/seraplot</code></pre></div>
<div id="qs-install-fr-rust" class="sp-tc"><pre><code class="language-bash">cargo add seraplot</code></pre></div>
<div id="qs-install-fr-r" class="sp-tc"><pre><code class="language-r">install.packages("seraplot")</code></pre></div>
</div>

<div class="qs-step"><div class="qs-step-num">2</div><h3>Construire un graphique</h3></div>

<div class="sp-tabs" id="qs-build-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('qs-build-fr','qs-build-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('qs-build-fr','qs-build-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('qs-build-fr','qs-build-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('qs-build-fr','qs-build-fr-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('qs-build-fr','qs-build-fr-r',this)">R</button>
</div>
<div id="qs-build-fr-py" class="sp-tc sp-on"><pre><code class="language-python">import seraplot as sp

chart = sp.bar(
    title="Ventes par région",
    labels=["Nord", "Sud", "Est", "Ouest"],
    values=[120, 85, 200, 140],
    gridlines=True,
    show_text=True,
)
chart.show()</code></pre></div>
<div id="qs-build-fr-js" class="sp-tc"><pre><code class="language-javascript">const sp = require("seraplot");

const chart = sp.bar({
  title: "Ventes par région",
  labels: ["Nord", "Sud", "Est", "Ouest"],
  values: [120, 85, 200, 140],
  gridlines: true,
  show_text: true,
});
chart.show();</code></pre></div>
<div id="qs-build-fr-ts" class="sp-tc"><pre><code class="language-typescript">import * as sp from "seraplot";

const chart = sp.bar({
  title: "Ventes par région",
  labels: ["Nord", "Sud", "Est", "Ouest"],
  values: [120, 85, 200, 140],
  gridlines: true,
  show_text: true,
});
chart.show();</code></pre></div>
<div id="qs-build-fr-rust" class="sp-tc"><pre><code class="language-rust">// Cargo.toml: [dependencies] seraplot = "2"
use seraplot::Chart;

fn main() {
    let chart = Chart::bar()
        .title("Ventes par région")
        .labels(["Nord", "Sud", "Est", "Ouest"])
        .values([120.0, 85.0, 200.0, 140.0])
        .gridlines(true)
        .show_text(true)
        .build();
    chart.show();
}</code></pre></div>
<div id="qs-build-fr-r" class="sp-tc"><pre><code class="language-r">library(seraplot)

chart <- sp_bar(
  title = "Ventes par région",
  labels = c("Nord", "Sud", "Est", "Ouest"),
  values = c(120, 85, 200, 140),
  gridlines = TRUE,
  show_text = TRUE
)
chart$show()</code></pre></div>
</div>

<div class="qs-tip"><strong>💡 Jupyter / notebook :</strong> le graphique s'affiche automatiquement — pas besoin d'appeler <code>.show()</code>.</div>

<div class="qs-step"><div class="qs-step-num">3</div><h3>Enregistrer ou exporter</h3></div>

<div class="sp-tabs" id="qs-save-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('qs-save-fr','qs-save-fr-py',this)">Python</button>
<button class="sp-tb" onclick="spTab('qs-save-fr','qs-save-fr-js',this)">JavaScript</button>
<button class="sp-tb" onclick="spTab('qs-save-fr','qs-save-fr-ts',this)">TypeScript</button>
<button class="sp-tb" onclick="spTab('qs-save-fr','qs-save-fr-rust',this)">Rust</button>
<button class="sp-tb" onclick="spTab('qs-save-fr','qs-save-fr-r',this)">R</button>
</div>
<div id="qs-save-fr-py" class="sp-tc sp-on"><pre><code class="language-python">chart.save("ventes.html")   # HTML autonome
chart.save("ventes.png")    # raster
chart.save("ventes.svg")    # vectoriel</code></pre></div>
<div id="qs-save-fr-js" class="sp-tc"><pre><code class="language-javascript">chart.save("ventes.html");
chart.save("ventes.png");
chart.save("ventes.svg");</code></pre></div>
<div id="qs-save-fr-ts" class="sp-tc"><pre><code class="language-typescript">chart.save("ventes.html");
chart.save("ventes.png");
chart.save("ventes.svg");</code></pre></div>
<div id="qs-save-fr-rust" class="sp-tc"><pre><code class="language-rust">chart.save("ventes.html")?;
chart.save("ventes.png")?;
chart.save("ventes.svg")?;</code></pre></div>
<div id="qs-save-fr-r" class="sp-tc"><pre><code class="language-r">chart$save("ventes.html")
chart$save("ventes.png")
chart$save("ventes.svg")</code></pre></div>
</div>

## Et après ?

<div class="qs-grid">
<div class="qs-card"><h4>📊 Catalogue de graphiques</h4><p>Découvrez <a href="../charts/2d/bar.html">barres</a>, <a href="../charts/2d/line.html">lignes</a>, <a href="../charts/2d/scatter.html">nuages de points</a>, heatmaps et 60+ autres types — chacun avec des exemples copier-coller dans neuf langages.</p></div>
<div class="qs-card"><h4>🤖 Machine learning</h4><p>Entraînez et visualisez des <a href="../ml/index.html">modèles façon scikit-learn</a> directement sur vos données : régression, clustering, arbres, SVM…</p></div>
<div class="qs-card"><h4>🎨 Thèmes &amp; palettes</h4><p>Personnalisez les couleurs avec <a href="../config/palette.html">palettes</a>, <a href="../config/themes.html">thèmes</a> et <a href="../config/background.html">arrière-plans</a> par graphique.</p></div>
<div class="qs-card"><h4>⚡ Streaming &amp; big data</h4><p>Affichez des millions de points grâce au <a href="../config/downsample.html">sous-échantillonnage</a>, au <a href="../config/streaming.html">streaming live</a> et aux <a href="../config/diff.html">mises à jour diff</a>.</p></div>
</div>

</div>
