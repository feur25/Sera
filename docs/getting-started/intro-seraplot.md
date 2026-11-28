<div align="center">

# SeraPlot

**Plot anything. Train anything. Ship anywhere.**

*A Rust-native engine for visualization, machine learning, and zero-friction delivery — independently benchmarked at 12×-116× faster than Plotly and Matplotlib, 200× smaller, zero dependencies.*

[![PyPI](https://img.shields.io/pypi/v/seraplot)](https://pypi.org/project/seraplot/)
[![npm](https://img.shields.io/npm/v/seraplot)](https://www.npmjs.com/package/seraplot)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/feur25/seraplot/blob/main/LICENSE)

</div>

<div class="lang-en">

## Seraplot, More than a charting library

SeraPlot is a complete data toolkit written in Rust. The same engine powers your visualizations, your machine learning, and the way you ship results to other people.

| Pillar | What you get |
|---|---|
| **Plot** | 57 chart types — 33 in 2D, 17 in 3D (WebGL), 2 maps. Built-in themes, palettes, animation, zoom, crosshair, export. |
| **Train** | Scikit-learn-compatible ML in Rust — DBSCAN, K-Means, RandomForest, GradientBoosting, SVM, PCA, GridSearchCV, train/test split. 1.3× to 686× faster. |
| **Stream & scale** | Live updates, downsampling for millions of points, drift detection, AutoML, diff mode, facet grids. |
| **Ship** | Self-contained 21 KB HTML — no CDN, no backend, works offline, by email, in S3, in PDFs, in Notion, in air-gapped CI. |
| **Integrate** | Python, JavaScript, TypeScript, Rust. Drop-in `seraplot.matplotlib as plt` migration. Pandas / NumPy native. |
| **Author** | Native VS Code extension — live preview, gallery, theme studio, snippets, auto-detection of `labels` / `values` from your code. |
| **Persist & export** | Save to HTML, PNG, SVG, PDF, pickle. Re-load trained ML models. CSP-safe output. |
| **Stay accessible** | A11y-tagged SVG, semantic HTML, keyboard navigation, locale-aware number formatting. |

> **One library replaces:** matplotlib + plotly + dash + streamlit + seaborn + parts of scikit-learn — with one `pip install` and zero runtime dependencies.

---

## Same chart — three libraries

<style>
.sp-tabs{border:1px solid var(--sp-border);border-radius:8px;overflow:hidden;margin:1.5em 0}
.sp-tab-btns{display:flex;background:var(--sp-surface);border-bottom:1px solid var(--sp-border);overflow-x:auto}
.sp-tb{padding:9px 22px;border:none;background:none;color:var(--sp-text-muted);cursor:pointer;font-size:13px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap;flex:0 0 auto}
.sp-tc pre{margin:0;border-radius:0;overflow-x:auto;overflow-y:hidden;padding:14px 16px;box-sizing:border-box}
</style>

<div class="sp-tabs" id="g1">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('g1','g1a',this)">SeraPlot — 2 lines</button>
<button class="sp-tb" onclick="spTab('g1','g1b',this)">Plotly — 4 lines</button>
<button class="sp-tb" onclick="spTab('g1','g1c',this)">Matplotlib — 7 lines</button>
</div>
<div id="g1a" class="sp-tc sp-on"><pre><code class="language-python">import seraplot as sp
sp.bar("Revenue by Product", labels, values).save("chart.html")</code></pre></div>
<div id="g1b" class="sp-tc"><pre><code class="language-python">import plotly.express as px
fig = px.bar(x=labels, y=values, title="Revenue by Product")
fig.update_layout(template="plotly_white")
fig.write_html("chart.html")</code></pre></div>
<div id="g1c" class="sp-tc"><pre><code class="language-python">import matplotlib.pyplot as plt
fig, ax = plt.subplots(figsize=(9, 5))
ax.bar(labels, values, color="#6366f1")
ax.set_title("Revenue by Product")
ax.set_ylabel("Revenue")
plt.tight_layout()
plt.savefig("chart.png")</code></pre></div>
</div>

|               |  SeraPlot | Plotly | Matplotlib |
| ------------- | :-------: | :----: | :--------: |
| Lines of code |   **2**   |    4   |      7     |
| Output        |    HTML   |  HTML  |     PNG    |
| File size     | **21 KB** | 4.7 MB |   ~150 KB  |
| Interactive   |     ✅     |    ✅   |      ❌     |
| Dependencies  |   **0**   |   6+   |     3+     |
| 1-line migration | ✅     |    —   |      —     |

---

## Why SeraPlot?

**Fun facts, independently verified** — full raw data and the scripts to reproduce every one of them live in [`tools/benchmarks/`](https://github.com/feur25/Sera/tree/main/tools/benchmarks):

- A dashboard that builds 500 bar charts in a row finishes before Matplotlib has drawn its 30th — Matplotlib didn't complete a single 500-build sustained loop on categorical charts within a 90-second cap, in any of 3 tested environments.
- One `pip install seraplot` replaces Matplotlib + Plotly + Dash + Streamlit + Seaborn and part of scikit-learn — one engine, zero runtime dependencies, no CDN, no backend.
- SeraPlot isn't faster at everything. At very large datasets (1,000,000+ points), Matplotlib actually wins on a couple of chart types — we measure that too and say so, because a benchmark you can't contradict isn't a benchmark.
- SeraPlot is also available across JS/TS, Rust, Python, C#, and C++, with the same command names in every language.

### Why SeraPlot, and not Plotly or Matplotlib?

Plotly and Matplotlib are mature, excellent tools — SeraPlot isn't trying to replace them for every job. Reach for SeraPlot when:

- You're shipping charts to people who don't control their environment — email, S3, Notion, air-gapped networks, PDFs. A self-contained HTML file a few tens of KB large just works, without a multi-megabyte bundle or a static image.
- You're building something that renders a lot of charts fast — a live dashboard, a report generator, a CLI tool — where the gap is widest.
- You want one engine for plotting, basic ML, and dataframe work instead of stitching three ecosystems together, with a chunk-resume system for recovering from a crash and multiple aliases for the same method (e.g. `sp.build_bar_chart` / `sp.bar_chart` / `sp.bar` / `sp.bars`).
- You're fine being an early adopter: SeraPlot is a one-person project, and its benchmark claims get republished with full raw data every time they're challenged — including the two real bugs found and fixed while building this very page ([#29](https://github.com/feur25/Sera/issues/29), [#30](https://github.com/feur25/Sera/issues/30)).

---

## 1000 charts. Measured.

Same code, same random data, same machine. Full HTML output timed.

<div class="sp-tabs" id="g-bench">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('g-bench','gb-sp',this)">SeraPlot — 44 ms</button>
<button class="sp-tb" onclick="spTab('g-bench','gb-pl',this)">Plotly — 18.4 s</button>
<button class="sp-tb" onclick="spTab('g-bench','gb-mp',this)">Matplotlib — 31.8 s</button>
</div>
<div id="gb-sp" class="sp-tc sp-on">
<pre><code class="language-python">import seraplot as sp
categories = ["Electronics", "Clothing", "Food", "Books", "Sports", "Toys", "Health", "Auto"]
data = [...]  # 1000 pre-generated lists
for i in range(1000):
    sp.bar(f"Report #{i+1}", labels=categories, values=data[i]).html</code></pre>
<p style="padding:10px 16px;margin:0;color:#6366f1;font-weight:700">1000 charts in 44 ms — 44 µs/chart</p>
<iframe src="../previews/bench-seraplot.html" style="width:100%;height:480px;border:none;border-top:1px solid #334155" loading="lazy"></iframe>
</div>
<div id="gb-pl" class="sp-tc">
<pre><code class="language-python">import plotly.graph_objects as go
categories = ["Electronics", "Clothing", "Food", "Books", "Sports", "Toys", "Health", "Auto"]
data = [...]  # same 1000 pre-generated lists
for i in range(1000):
    fig = go.Figure(data=[go.Bar(x=categories, y=data[i])])
    fig.update_layout(title=f"Report #{i+1}", template="plotly_dark")
    fig.to_html(full_html=True, include_plotlyjs="cdn")</code></pre>
<p style="padding:10px 16px;margin:0;color:#ef4444;font-weight:700">1000 charts in 18,368 ms — 421× slower</p>
<iframe src="../previews/bench-plotly.html" style="width:100%;height:480px;border:none;border-top:1px solid #334155" loading="lazy"></iframe>
</div>
<div id="gb-mp" class="sp-tc">
<pre><code class="language-python">import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
categories = ["Electronics", "Clothing", "Food", "Books", "Sports", "Toys", "Health", "Auto"]
data = [...]  # same 1000 pre-generated lists
for i in range(1000):
    fig, ax = plt.subplots(figsize=(9, 5))
    ax.bar(categories, data[i])
    ax.set_title(f"Report #{i+1}")
    fig.savefig(f"chart_{i}.png")
    plt.close()</code></pre>
<p style="padding:10px 16px;margin:0;color:#ef4444;font-weight:700">1000 charts in 31,833 ms — 730× slower</p>
<iframe src="../previews/bench-matplotlib.html" style="width:100%;height:480px;border:none;border-top:1px solid #334155" loading="lazy"></iframe>
</div>
</div>

Measured on the same machine, same data, same loop, re-run and re-checked for this page -- not carried over from an older release. Full methodology and raw numbers in [`tools/benchmarks/`](https://github.com/feur25/Sera/tree/main/tools/benchmarks).

| Scale | SeraPlot | Plotly | Matplotlib |
|-------|:--------:|:------:|:----------:|
| 1,000 charts | **44 ms** | 18.4 s | 31.8 s |
| 10,000 charts | **~440 ms** | ~3 min | ~5.3 min |
| 100,000 charts | **~4.4 s** | ~31 min | ~53 min |

---

## Render core speed

**Benchmark: 10,548 cold-process trials + 1,620 warm-loop sweeps (810,000 chart builds), 6 chart types, 6 sizes (10 to 1,000,000), 3 Docker resource tiers, against Plotly and Matplotlib. Full pipeline timed end to end — import, build, and HTML serialization, for every tool, on the same footing. Raw CSVs, environment specs, and the scripts to reproduce every number below are checked into [`tools/benchmarks/`](https://github.com/feur25/Sera/tree/main/tools/benchmarks).**

An earlier version of this table compared SeraPlot's object-creation time against Plotly's full HTML-serialization time while claiming to exclude serialization on both sides — that inflated the multipliers shown here by roughly 46×. It's been replaced with the numbers below, which compare the same thing on both sides.

<iframe src="../previews/bench-cold-speed.html" style="width:100%;height:480px;border:none;border-top:1px solid #334155" loading="lazy"></iframe>

| Chart type | vs Plotly | vs Matplotlib |
|-------|:--------:|:--------:|
| Bar | **22×** | **116×** |
| Line | **23×** | **59×** |
| Scatter | **22×** | **61×** |
| Histogram | **12×** | **41×** |
| Pie | **14×** | **45×** |
| Box Plot | **13×** | **39×** |

Median time across every size and every resource tier, one fresh process per call. Building this benchmark also surfaced a real regression: `bar`/`line`/`scatter` were paying a one-time ~100ms numpy-import tax on every cold process (now fixed), which is why the numbers above are roughly 25× faster than they were a version earlier for those three types:

<iframe src="../previews/bench-before-after.html" style="width:100%;height:420px;border:none;border-top:1px solid #334155" loading="lazy"></iframe>

In a long-running process that builds many charts in a row (a dashboard, an app), import cost is paid once and the gap widens further — on categorical charts specifically, Matplotlib didn't finish the warm-loop sweep at all within its timeout, which is where "thousands of times faster" comes from. Full breakdown, methodology, and reproduction instructions in [`tools/benchmarks/README.md`](https://github.com/feur25/Sera/blob/main/tools/benchmarks/README.md).

---

## Output file size

Plotly embeds its entire JavaScript bundle in every HTML file.
SeraPlot only includes the JS needed for that specific chart type.

<div style="font-family:monospace;margin:1.2em 0">
<div style="display:flex;align-items:center;gap:8px;margin:6px 0">
  <span style="min-width:90px;font-size:.85em">Pie</span>
  <div style="display:flex;gap:3px;align-items:center">
    <div style="background:#6366f1;width:5px;height:16px;border-radius:2px" title="SeraPlot 19 KB"></div>
    <div style="background:#ef4444;width:300px;height:16px;border-radius:2px" title="Plotly 4,733 KB"></div>
  </div>
  <span style="font-size:.8em;color:#94a3b8">19 KB vs 4,733 KB — <b style="color:#ef4444">246×</b></span>
</div>
<div style="display:flex;align-items:center;gap:8px;margin:6px 0">
  <span style="min-width:90px;font-size:.85em">Bar</span>
  <div style="display:flex;gap:3px;align-items:center">
    <div style="background:#6366f1;width:5px;height:16px;border-radius:2px"></div>
    <div style="background:#ef4444;width:274px;height:16px;border-radius:2px"></div>
  </div>
  <span style="font-size:.8em;color:#94a3b8">21 KB vs 4,733 KB — <b style="color:#ef4444">225×</b></span>
</div>
<div style="display:flex;align-items:center;gap:8px;margin:6px 0">
  <span style="min-width:90px;font-size:.85em">Scatter</span>
  <div style="display:flex;gap:3px;align-items:center">
    <div style="background:#6366f1;width:5px;height:16px;border-radius:2px"></div>
    <div style="background:#ef4444;width:148px;height:16px;border-radius:2px"></div>
  </div>
  <span style="font-size:.8em;color:#94a3b8">39 KB vs 4,740 KB — <b style="color:#ef4444">121×</b></span>
</div>
<div style="display:flex;align-items:center;gap:8px;margin:6px 0">
  <span style="min-width:90px;font-size:.85em">Radar</span>
  <div style="display:flex;gap:3px;align-items:center">
    <div style="background:#6366f1;width:5px;height:16px;border-radius:2px"></div>
    <div style="background:#ef4444;width:250px;height:16px;border-radius:2px"></div>
  </div>
  <span style="font-size:.8em;color:#94a3b8">23 KB vs 4,733 KB — <b style="color:#ef4444">205×</b></span>
</div>
</div>

Matplotlib outputs PNG/SVG/PDF (50-500 KB) — not interactive HTML.

---

## What SeraPlot actually is

SeraPlot is not a wrapper around Plotly, Chart.js, or D3.

It is a **Rust-native rendering engine** that generates minimal HTML + JS per chart.
A Pie chart gets Pie JS. A Bar chart gets Bar JS. Nothing else is bundled.

That's why the output is 20 KB instead of 4.7 MB.

---

## One line migration

```python
import seraplot.matplotlib as plt
```

Everything else stays the same.
`plt.bar()`, `plt.scatter()`, `plt.hist()`, `plt.show()`, `plt.savefig()` — unchanged.

---

## Deploy from an API

<div class="sp-tabs" id="g2">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('g2','g2a',this)">SeraPlot — 7 lines</button>
<button class="sp-tb" onclick="spTab('g2','g2b',this)">Plotly — 10 lines</button>
<button class="sp-tb" onclick="spTab('g2','g2c',this)">Matplotlib — 14 lines</button>
</div>
<div id="g2a" class="sp-tc sp-on"><pre><code class="language-python">from fastapi import FastAPI
import seraplot as sp
app = FastAPI()
@app.get("/chart")
def revenue_chart():
    return sp.bar("Revenue", labels, values).html</code></pre></div>
<div id="g2b" class="sp-tc"><pre><code class="language-python">from fastapi import FastAPI
from fastapi.responses import HTMLResponse
import plotly.express as px
app = FastAPI()
@app.get("/chart", response_class=HTMLResponse)
def revenue_chart():
    fig = px.bar(x=labels, y=values, title="Revenue")
    return fig.to_html(full_html=True)</code></pre></div>
<div id="g2c" class="sp-tc"><pre><code class="language-python">from fastapi import FastAPI
from fastapi.responses import FileResponse
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import tempfile
app = FastAPI()
@app.get("/chart")
def revenue_chart():
    fig, ax = plt.subplots(figsize=(9, 5))
    ax.bar(labels, values)
    ax.set_title("Revenue")
    path = tempfile.mktemp(suffix=".png")
    plt.savefig(path)
    plt.close()
    return FileResponse(path, media_type="image/png")</code></pre></div>
</div>

<script>
function spTab(g, id, btn) {
  var root = document.getElementById(g);
  root.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on');});
  root.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act');});
  document.getElementById(id).classList.add('sp-on');
  btn.classList.add('sp-act');
  if (window.hljs) document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c);}catch(e){}});
}
document.addEventListener('DOMContentLoaded', function() {
  if (window.hljs) document.querySelectorAll('.sp-tc code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c);}catch(e){}});
});
</script>

Plotly returns 4.7 MB per request. Matplotlib requires disk I/O and returns a static PNG.
SeraPlot returns 21 KB of interactive HTML directly from RAM.

---

## Everything SeraPlot does

- **57 chart types** — every 2D chart has a 3D WebGL variant
- **Drop-in matplotlib API** — `import seraplot.matplotlib as plt`
- **Pandas & NumPy native** — pass DataFrames directly
- **7 built-in themes** — dark, light, scientific, apple, notion, minimal, neon
- **Global config** — `sp.config()` sets font, zoom, crosshair, animation across all charts
- **Zero dependencies** — pure Rust renderer
- **200× smaller files** — no bundled JS runtime
- **Multi-language** — Python, JavaScript/TypeScript (npm), Rust, R, Scala, C#, C++, Java
- **DBSCAN up to 600× faster** than scikit-learn
- **Native Machine learning** — some ml methods is include by default in this tool
- **Works everywhere** — Python ≥ 3.8, any OS

---

## See it in action

<figure class="sp-video-embed">

<video controls preload="metadata" poster="../media/video/tuto_doc_poster.jpg" playsinline>
<source src="../media/video/tuto_doc.mp4" type="video/mp4">
</video>

<figcaption>A walkthrough of the SeraPlot documentation site</figcaption>

</figure>

## Navigation

- **[Installation](installation.md)** — `pip install seraplot`
- **[Quick Start](quickstart.md)** — first chart in 3 lines
- **[Chart Methods](chart-methods.md)** — all universal Chart methods
- **[2D Charts](../charts/2d/bar.md)** — 33 chart types
- **[3D Charts](../charts/3d/scatter3d.md)** — 17 chart types, WebGL GPU renderer
- **[Machine Learning](../ml/dbscan.md)** — DBSCAN up to 600x faster than scikit-learn

</div>

<div class="lang-fr">

## Seraplot - Bien plus qu'une bibliothèque de graphiques

> **« Tout tracer. Tout entraîner. Partout déployer. »**

SeraPlot est une boîte à outils data complète écrite en Rust. Le même moteur propulse vos visualisations, votre machine learning, et la façon dont vous livrez les résultats à vos collègues.

| Pilier | Ce que vous obtenez |
|---|---|
| **Tracer** | 57 types de graphiques — 33 en 2D, 17 en 3D (WebGL), 2 cartes. Thèmes intégrés, palettes, animations, zoom, crosshair, export. |
| **Entraîner** | ML compatible scikit-learn en Rust — DBSCAN, K-Means, RandomForest, GradientBoosting, SVM, PCA, GridSearchCV, train/test split. De 1,3× à 686× plus rapide. |
| **Streamer & passer à l'échelle** | Mises à jour en direct, downsampling pour des millions de points, détection de drift, AutoML, mode diff, grilles de facettes. |
| **Déployer** | HTML autonome de 21 Ko — pas de CDN, pas de backend, fonctionne hors-ligne, par e-mail, sur S3, dans des PDF, dans Notion, en CI isolée. |
| **Intégrer** | Python, JavaScript, TypeScript, Rust. Migration directe `seraplot.matplotlib as plt`. Pandas / NumPy nativement. |
| **Coder** | Extension VS Code native — aperçu en direct, galerie, studio de thèmes, snippets, détection automatique des `labels` / `values` depuis votre code. |
| **Persister & exporter** | Export HTML, PNG, SVG, PDF, pickle. Rechargement des modèles ML entraînés. Sortie compatible CSP. |
| **Rester accessible** | SVG balisé a11y, HTML sémantique, navigation clavier, formatage numérique localisé. |

> **Une seule librairie remplace :** matplotlib + plotly + dash + streamlit + seaborn + une partie de scikit-learn — avec un seul `pip install` et zéro dépendance d'exécution.

---

## Même graphique — trois bibliothèques

<div class="sp-tabs" id="g1fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('g1fr','g1fra',this)">SeraPlot — 2 lignes</button>
<button class="sp-tb" onclick="spTab('g1fr','g1frb',this)">Plotly — 4 lignes</button>
<button class="sp-tb" onclick="spTab('g1fr','g1frc',this)">Matplotlib — 7 lignes</button>
</div>
<div id="g1fra" class="sp-tc sp-on"><pre><code class="language-python">import seraplot as sp
sp.bar("Chiffre d'affaires par produit", labels, values).save("chart.html")</code></pre></div>
<div id="g1frb" class="sp-tc"><pre><code class="language-python">import plotly.express as px
fig = px.bar(x=labels, y=values, title="Chiffre d'affaires par produit")
fig.update_layout(template="plotly_white")
fig.write_html("chart.html")</code></pre></div>
<div id="g1frc" class="sp-tc"><pre><code class="language-python">import matplotlib.pyplot as plt
fig, ax = plt.subplots(figsize=(9, 5))
ax.bar(labels, values, color="#6366f1")
ax.set_title("Chiffre d'affaires par produit")
ax.set_ylabel("Chiffre d'affaires")
plt.tight_layout()
plt.savefig("chart.png")</code></pre></div>
</div>

|                    |  SeraPlot | Plotly | Matplotlib |
| ------------------ | :-------: | :----: | :--------: |
| Lignes de code     |   **2**   |    4   |      7     |
| Sortie             |    HTML   |  HTML  |     PNG    |
| Taille fichier     | **21 Ko** | 4,7 Mo |  ~150 Ko   |
| Interactif         |     ✅    |    ✅  |      ❌    |
| Dépendances        |   **0**   |   6+   |     3+     |
| Migration 1 ligne  |    ✅     |    —   |      —     |

---

## Pourquoi SeraPlot ?

**Fun facts, vérifiés indépendamment** — données brutes et scripts de reproduction complets dans [`tools/benchmarks/`](https://github.com/feur25/Sera/tree/main/tools/benchmarks) :

- Un dashboard qui construit 500 graphiques en barres d'affilée a terminé avant que Matplotlib n'ait dessiné le 30e — Matplotlib n'a jamais terminé une seule boucle soutenue de 500 constructions sur des graphiques catégoriels dans les 90 secondes imparties, dans aucun des 3 environnements testés.
- Un seul `pip install seraplot` remplace Matplotlib + Plotly + Dash + Streamlit + Seaborn et une partie de scikit-learn — un seul moteur, zéro dépendance d'exécution, pas de CDN, pas de backend.
- SeraPlot n'est pas plus rapide sur tout. Sur de très gros volumes (1 000 000+ points), Matplotlib l'emporte réellement sur deux types de graphiques — on le mesure aussi et on le dit, parce qu'un benchmark qu'on ne peut pas contredire n'en est pas un.
- SeraPlot est aussi disponible en JS/TS, Rust, Python, C# et C++, avec les mêmes noms de commandes dans chaque langage.

### Pourquoi SeraPlot, et pas Plotly ou Matplotlib ?

Plotly et Matplotlib sont des outils matures et excellents — SeraPlot ne cherche pas à les remplacer pour tous les usages. Préférez SeraPlot quand :

- Vous livrez des graphiques à des gens qui ne contrôlent pas leur environnement — email, S3, Notion, réseaux isolés, PDF. Un fichier HTML autonome de quelques dizaines de Ko fonctionne tout simplement, sans bundle de plusieurs mégaoctets ni image statique.
- Vous construisez quelque chose qui génère beaucoup de graphiques rapidement — un dashboard en direct, un générateur de rapports, un outil CLI — là où l'écart est le plus large.
- Vous voulez un seul moteur pour la visualisation, le ML de base et le travail sur dataframes plutôt que d'assembler trois écosystèmes, avec un système de reprise par chunks en cas de crash et plusieurs alias pour une même méthode (ex : `sp.build_bar_chart` / `sp.bar_chart` / `sp.bar` / `sp.bars`).
- Vous acceptez d'être un early adopter : SeraPlot est un projet développé par une seule personne, et ses affirmations de performance sont republiées avec toutes les données brutes à chaque fois qu'elles sont contestées — y compris les deux vrais bugs trouvés et corrigés en construisant cette page même ([#29](https://github.com/feur25/Sera/issues/29), [#30](https://github.com/feur25/Sera/issues/30)).

---

## 1 000 graphiques. Mesurés.

Même code, mêmes données aléatoires, même machine. Sortie HTML complète chronométrée.

<div class="sp-tabs" id="g-bench-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('g-bench-fr','gb-sp-fr',this)">SeraPlot — 44 ms</button>
<button class="sp-tb" onclick="spTab('g-bench-fr','gb-pl-fr',this)">Plotly — 18,4 s</button>
<button class="sp-tb" onclick="spTab('g-bench-fr','gb-mp-fr',this)">Matplotlib — 31,8 s</button>
</div>
<div id="gb-sp-fr" class="sp-tc sp-on">
<pre><code class="language-python">import seraplot as sp
categories = ["Électronique", "Vêtements", "Alimentation", "Livres", "Sport", "Jouets", "Santé", "Auto"]
data = [...]
for i in range(1000):
    sp.bar(f"Rapport #{i+1}", labels=categories, values=data[i]).html</code></pre>
<p style="padding:10px 16px;margin:0;color:#6366f1;font-weight:700">1 000 graphiques en 44 ms — 44 µs/graphique</p>
<iframe src="../previews/bench-seraplot.html" style="width:100%;height:480px;border:none;border-top:1px solid #334155" loading="lazy"></iframe>
</div>
<div id="gb-pl-fr" class="sp-tc">
<pre><code class="language-python">import plotly.graph_objects as go
categories = ["Électronique", "Vêtements", "Alimentation", "Livres", "Sport", "Jouets", "Santé", "Auto"]
data = [...]
for i in range(1000):
    fig = go.Figure(data=[go.Bar(x=categories, y=data[i])])
    fig.update_layout(title=f"Rapport #{i+1}", template="plotly_dark")
    fig.to_html(full_html=True, include_plotlyjs="cdn")</code></pre>
<p style="padding:10px 16px;margin:0;color:#ef4444;font-weight:700">1 000 graphiques en 18 368 ms — 421× plus lent</p>
<iframe src="../previews/bench-plotly.html" style="width:100%;height:480px;border:none;border-top:1px solid #334155" loading="lazy"></iframe>
</div>
<div id="gb-mp-fr" class="sp-tc">
<pre><code class="language-python">import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
categories = ["Électronique", "Vêtements", "Alimentation", "Livres", "Sport", "Jouets", "Santé", "Auto"]
data = [...]
for i in range(1000):
    fig, ax = plt.subplots(figsize=(9, 5))
    ax.bar(categories, data[i])
    ax.set_title(f"Rapport #{i+1}")
    fig.savefig(f"chart_{i}.png")
    plt.close()</code></pre>
<p style="padding:10px 16px;margin:0;color:#ef4444;font-weight:700">1 000 graphiques en 31 833 ms — 730× plus lent</p>
<iframe src="../previews/bench-matplotlib.html" style="width:100%;height:480px;border:none;border-top:1px solid #334155" loading="lazy"></iframe>
</div>
</div>

Mesuré sur la même machine, les mêmes données, la même boucle, rejoué et revérifié pour cette page -- pas repris d'une ancienne version. Méthodologie complète et chiffres bruts dans [`tools/benchmarks/`](https://github.com/feur25/Sera/tree/main/tools/benchmarks).

| Échelle | SeraPlot | Plotly | Matplotlib |
|---------|:--------:|:------:|:----------:|
| 1 000 graphiques | **44 ms** | 18,4 s | 31,8 s |
| 10 000 graphiques | **~440 ms** | ~3 min | ~5,3 min |
| 100 000 graphiques | **~4,4 s** | ~31 min | ~53 min |

---

## Vitesse du moteur de rendu

**Benchmark : 10 548 essais a froid + 1 620 boucles chaudes (810 000 graphiques construits), 6 types de graphique, 6 tailles (10 a 1 000 000), 3 paliers de ressources Docker, face a Plotly et Matplotlib. Pipeline complet chronometre de bout en bout — import, construction et serialisation HTML, pour chaque outil, sur un pied d'egalite. CSV bruts, specs d'environnement et scripts de reproduction dans [`tools/benchmarks/`](https://github.com/feur25/Sera/tree/main/tools/benchmarks).**

Une version precedente de ce tableau comparait le temps de construction pure de SeraPlot au temps de serialisation HTML complete de Plotly, tout en affirmant exclure la serialisation des deux cotes — ce qui inflait les multiplicateurs d'environ 46×. Remplace par les chiffres ci-dessous, qui comparent la meme chose des deux cotes.

<iframe src="../previews/bench-cold-speed.html" style="width:100%;height:480px;border:none;border-top:1px solid #334155" loading="lazy"></iframe>

| Type de graphique | vs Plotly | vs Matplotlib |
|-------|:--------:|:--------:|
| Barres | **22×** | **116×** |
| Ligne | **23×** | **59×** |
| Nuage de points | **22×** | **61×** |
| Histogramme | **12×** | **41×** |
| Camembert | **14×** | **45×** |
| Boîte à moustaches | **13×** | **39×** |

Mediane sur toutes les tailles et tous les paliers de ressources, un process neuf par appel. La construction de ce benchmark a aussi mis a jour une vraie regression : `bar`/`line`/`scatter` payaient une taxe ponctuelle d'import numpy (~100ms) a chaque process froid (desormais corrigee), d'ou les chiffres ci-dessus environ 25× plus rapides qu'une version precedente pour ces trois types :

<iframe src="../previews/bench-before-after.html" style="width:100%;height:420px;border:none;border-top:1px solid #334155" loading="lazy"></iframe>

Dans un process qui construit beaucoup de graphiques d'affilee (un dashboard, une appli), le cout d'import n'est paye qu'une fois et l'ecart se creuse encore — sur les graphiques categoriels specifiquement, Matplotlib n'a meme pas termine le sweep chaud dans son delai, d'ou le "des milliers de fois plus rapide". Detail complet, methodologie et instructions de reproduction dans [`tools/benchmarks/README.md`](https://github.com/feur25/Sera/blob/main/tools/benchmarks/README.md).

---

## Taille des fichiers de sortie

Plotly embarque tout son bundle JavaScript dans chaque fichier HTML.
SeraPlot n'inclut que le JS nécessaire au type de graphique spécifique.

<div style="font-family:monospace;margin:1.2em 0">
<div style="display:flex;align-items:center;gap:8px;margin:6px 0">
  <span style="min-width:90px;font-size:.85em">Camembert</span>
  <div style="display:flex;gap:3px;align-items:center">
    <div style="background:#6366f1;width:5px;height:16px;border-radius:2px"></div>
    <div style="background:#ef4444;width:300px;height:16px;border-radius:2px"></div>
  </div>
  <span style="font-size:.8em;color:#94a3b8">19 Ko vs 4 733 Ko — <b style="color:#ef4444">246×</b></span>
</div>
<div style="display:flex;align-items:center;gap:8px;margin:6px 0">
  <span style="min-width:90px;font-size:.85em">Barres</span>
  <div style="display:flex;gap:3px;align-items:center">
    <div style="background:#6366f1;width:5px;height:16px;border-radius:2px"></div>
    <div style="background:#ef4444;width:274px;height:16px;border-radius:2px"></div>
  </div>
  <span style="font-size:.8em;color:#94a3b8">21 Ko vs 4 733 Ko — <b style="color:#ef4444">225×</b></span>
</div>
<div style="display:flex;align-items:center;gap:8px;margin:6px 0">
  <span style="min-width:90px;font-size:.85em">Nuage de points</span>
  <div style="display:flex;gap:3px;align-items:center">
    <div style="background:#6366f1;width:5px;height:16px;border-radius:2px"></div>
    <div style="background:#ef4444;width:148px;height:16px;border-radius:2px"></div>
  </div>
  <span style="font-size:.8em;color:#94a3b8">39 Ko vs 4 740 Ko — <b style="color:#ef4444">121×</b></span>
</div>
<div style="display:flex;align-items:center;gap:8px;margin:6px 0">
  <span style="min-width:90px;font-size:.85em">Radar</span>
  <div style="display:flex;gap:3px;align-items:center">
    <div style="background:#6366f1;width:5px;height:16px;border-radius:2px"></div>
    <div style="background:#ef4444;width:250px;height:16px;border-radius:2px"></div>
  </div>
  <span style="font-size:.8em;color:#94a3b8">23 Ko vs 4 733 Ko — <b style="color:#ef4444">205×</b></span>
</div>
</div>

Matplotlib produit du PNG/SVG/PDF (50–500 Ko) — pas du HTML interactif.

---

## Ce qu'est réellement SeraPlot

SeraPlot n'est pas un wrapper autour de Plotly, Chart.js ou D3.

C'est un **moteur de rendu natif Rust** qui génère du HTML + JS minimal par graphique.
chaque chart reçoit soit js dédiée. Rien d'autre n'est embarqué.

C'est pour ça que la sortie fait 20 Ko au lieu de 4,7 Mo.

---

## Migration en une ligne

```python
import seraplot.matplotlib as plt
```

Tout le reste reste identique.
`plt.bar()`, `plt.scatter()`, `plt.hist()`, `plt.show()`, `plt.savefig()` — inchangés.

---

## Déployer depuis une API

<div class="sp-tabs" id="g2fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('g2fr','g2fra',this)">SeraPlot — 7 lignes</button>
<button class="sp-tb" onclick="spTab('g2fr','g2frb',this)">Plotly — 10 lignes</button>
<button class="sp-tb" onclick="spTab('g2fr','g2frc',this)">Matplotlib — 14 lignes</button>
</div>
<div id="g2fra" class="sp-tc sp-on"><pre><code class="language-python">from fastapi import FastAPI
import seraplot as sp
app = FastAPI()
@app.get("/chart")
def revenue_chart():
    return sp.bar("Chiffre d'affaires", labels, values).html</code></pre></div>
<div id="g2frb" class="sp-tc"><pre><code class="language-python">from fastapi import FastAPI
from fastapi.responses import HTMLResponse
import plotly.express as px
app = FastAPI()
@app.get("/chart", response_class=HTMLResponse)
def revenue_chart():
    fig = px.bar(x=labels, y=values, title="Chiffre d'affaires")
    return fig.to_html(full_html=True)</code></pre></div>
<div id="g2frc" class="sp-tc"><pre><code class="language-python">from fastapi import FastAPI
from fastapi.responses import FileResponse
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import tempfile
app = FastAPI()
@app.get("/chart")
def revenue_chart():
    fig, ax = plt.subplots(figsize=(9, 5))
    ax.bar(labels, values)
    ax.set_title("Chiffre d'affaires")
    path = tempfile.mktemp(suffix=".png")
    plt.savefig(path)
    plt.close()
    return FileResponse(path, media_type="image/png")</code></pre></div>
</div>

Plotly retourne 4,7 Mo par requête. Matplotlib nécessite des I/O disque et retourne un PNG statique.
SeraPlot retourne 21 Ko de HTML interactif directement depuis la RAM.

---

## Tout ce que SeraPlot fait

- **57 types de graphiques** — chaque graphique 2D a une variante 3D WebGL
- **API matplotlib drop-in** — `import seraplot.matplotlib as plt`
- **Pandas & NumPy natifs** — passez les DataFrames directement
- **7 thèmes intégrés** — dark, light, scientific, apple, notion, minimal, neon
- **Configuration globale** — `sp.config()` définit la police, le zoom, le réticule, l'animation pour tous les graphiques
- **Zéro dépendance** — moteur de rendu Rust pur
- **Fichiers 200× plus petits** — pas de runtime JS embarqué
- **Multi-langage** — Python, JavaScript/TypeScript (npm), Rust, R, Scala, C#, C++, Java
- **DBSCAN jusqu'à 600× plus rapide** que scikit-learn
- **Machine learning natif** — Plusieurs methods ml, sont déjà existante dans le framework
- **Fonctionne partout** — Python ≥ 3.8, tout OS

---

## À voir en action

<figure class="sp-video-embed">

<video controls preload="metadata" poster="../media/video/tuto_doc_poster.jpg" playsinline>
<source src="../media/video/tuto_doc.mp4" type="video/mp4">
</video>

<figcaption>Une visite guidée du site de documentation SeraPlot</figcaption>

</figure>

## Navigation

- **[Installation](installation.md)** — `pip install seraplot`
- **[Démarrage rapide](quickstart.md)** — premier graphique en 3 lignes
- **[Méthodes des graphiques](chart-methods.md)** — toutes les méthodes universelles Chart
- **[Graphiques 2D](../charts/2d/bar.md)** — 33 types
- **[Graphiques 3D](../charts/3d/scatter3d.md)** — 17 types, rendu GPU WebGL
- **[Machine Learning](../ml/dbscan.md)** — DBSCAN jusqu'à 600× plus rapide que scikit-learn

</div>
