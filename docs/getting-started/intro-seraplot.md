<div align="center">

# SeraPlot

**Plot anything. Train anything. Ship anywhere.**

*A Rust-native engine for visualization, machine learning, and zero-friction delivery — 6,000× faster than Plotly, 200× smaller, zero dependencies.*

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
.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}
.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;overflow-x:auto}
.sp-tb{padding:9px 22px;border:none;background:none;color:#64748b;cursor:pointer;font-size:13px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap;flex:0 0 auto}
.sp-tb:hover{color:#e2e8f0}
.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}
.sp-tc pre{margin:0;border-radius:0;overflow-x:auto;overflow-y:hidden;padding:14px 16px;box-sizing:border-box}
.sp-tc code{display:block;line-height:1.45}
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

## Why Seraplot?

As you’ve probably understood by now, Seraplot is a tool designed to be extremely customizable, while also being much faster and more resource-efficient than existing solutions. It also provides a wide range of helpful features, such as the Seraplot extension for VSCode, which allows you to generate plots or ML methods very quickly and live, between each save of your scripts.

In addition, Seraplot is available across multiple languages such as: JS/TS, C (C# & C++), Java, Rust, Python, R & Scala. The main goal is to be highly accessible: from one language to another, the commands remain the same for greater simplicity.

In summary, Seraplot is a much more practical and independent tool that enables the generation of 2D & 3D plots, while also aiming to provide machine learning-related methods that you will find throughout the documentation. More surprises await you, such as the ability to choose different themes, a chunk system in case of crashes to resume from the error point, and even multiple aliases to use the same method (e.g., sp.build_bar_chart / sp.bar_chart / sp.bar / sp.bars).

---

## 1000 charts. Measured.

Same code, same random data, same machine. Full HTML output timed.

<div class="sp-tabs" id="g-bench">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('g-bench','gb-sp',this)">SeraPlot — 6 ms</button>
<button class="sp-tb" onclick="spTab('g-bench','gb-pl',this)">Plotly — 37 s</button>
<button class="sp-tb" onclick="spTab('g-bench','gb-mp',this)">Matplotlib — 60 s</button>
</div>
<div id="gb-sp" class="sp-tc sp-on">
<pre><code class="language-python">import seraplot as sp
categories = ["Electronics", "Clothing", "Food", "Books", "Sports", "Toys", "Health", "Auto"]
data = [...]  # 1000 pre-generated lists
for i in range(1000):
    sp.bar(f"Report #{i+1}", categories, data[i]).html</code></pre>
<p style="padding:10px 16px;margin:0;color:#6366f1;font-weight:700">1000 charts in 6 ms — 6 µs/chart</p>
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
<p style="padding:10px 16px;margin:0;color:#ef4444;font-weight:700">1000 charts in 37,023 ms — 6,170× slower</p>
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
<p style="padding:10px 16px;margin:0;color:#ef4444;font-weight:700">1000 charts in 60,352 ms — 10,058× slower</p>
<iframe src="../previews/bench-matplotlib.html" style="width:100%;height:480px;border:none;border-top:1px solid #334155" loading="lazy"></iframe>
</div>
</div>

| Scale | SeraPlot | Plotly | Matplotlib |
|-------|:--------:|:------:|:----------:|
| 1,000 charts | **6 ms** | 37 s | 60 s |
| 10,000 charts | **~60 ms** | ~6 min | ~10 min |
| 100,000 charts | **~600 ms** | ~1 h | ~1.7 h |

---

## Render core speed

**Benchmark: Diabetes dataset (n=768, 40 runs). Rust render time — chart object creation, not full HTML serialization.**

<div style="font-family:monospace;margin:1.2em 0">
<div style="display:flex;align-items:center;gap:12px;margin:6px 0">
  <span style="min-width:130px;font-size:.85em">Pie</span>
  <div style="background:#6366f1;width:300px;height:16px;border-radius:3px"></div>
  <b style="color:#6366f1;font-size:.85em">7,956×</b>
</div>
<div style="display:flex;align-items:center;gap:12px;margin:6px 0">
  <span style="min-width:130px;font-size:.85em">Bar</span>
  <div style="background:#6366f1;width:245px;height:16px;border-radius:3px"></div>
  <b style="color:#6366f1;font-size:.85em">6,488×</b>
</div>
<div style="display:flex;align-items:center;gap:12px;margin:6px 0">
  <span style="min-width:130px;font-size:.85em">Lollipop</span>
  <div style="background:#6366f1;width:150px;height:16px;border-radius:3px"></div>
  <b style="color:#6366f1;font-size:.85em">3,983×</b>
</div>
<div style="display:flex;align-items:center;gap:12px;margin:6px 0">
  <span style="min-width:130px;font-size:.85em">Grouped Bar</span>
  <div style="background:#6366f1;width:136px;height:16px;border-radius:3px"></div>
  <b style="color:#6366f1;font-size:.85em">3,596×</b>
</div>
<div style="display:flex;align-items:center;gap:12px;margin:6px 0">
  <span style="min-width:130px;font-size:.85em">Candlestick</span>
  <div style="background:#6366f1;width:77px;height:16px;border-radius:3px"></div>
  <b style="color:#6366f1;font-size:.85em">2,038×</b>
</div>
<div style="display:flex;align-items:center;gap:12px;margin:6px 0">
  <span style="min-width:130px;font-size:.85em">Radar</span>
  <div style="background:#6366f1;width:56px;height:16px;border-radius:3px"></div>
  <b style="color:#6366f1;font-size:.85em">1,498×</b>
</div>
<div style="display:flex;align-items:center;gap:12px;margin:6px 0">
  <span style="min-width:130px;font-size:.85em">KDE</span>
  <div style="background:#6366f1;width:28px;height:16px;border-radius:3px"></div>
  <b style="color:#6366f1;font-size:.85em">753×</b>
</div>
</div>

| Chart | SeraPlot | Plotly figure | Plotly → HTML | Matplotlib |
|-------|----------|--------------|-------------|------------|
| Pie | **4.2** | 725 | 33,416 | 15,085 |
| Bar | **2.8** | 658 | 18,166 | 13,596 |
| Grouped Bar | **5.0** | 558 | 17,981 | 17,445 |
| Histogram | **12.4** | 2,496 | 32,762 | 37,973 |
| Scatter | **17.0** | 3,916 | 21,615 | 14,141 |
| Violin | **16.7** | 2,616 | 21,347 | 21,211 |
| Box Plot | **18.4** | 2,329 | 21,799 | 15,590 |
| KDE | **26.3** | 2,981 | 19,807 | 40,108 |
| Radar | **11.8** | 962 | 17,679 | 20,942 |
| Lollipop | **6.3** | 8,382 | 25,096 | 9,072 |
| Candlestick | **8.8** | 1,478 | 17,934 | N/A |
| Ridgeline | **88.8** | N/A | N/A | N/A |

All times in µs.

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

## Pourquoi Seraplot ?

Comme vous l’aurez compris en arrivant jusqu’ici, Seraplot est un outil qui a pour objectif d’être extrêmement personnalisable, mais aussi beaucoup plus rapide et moins gourmand que ce qui existe déjà, en plus de proposer tout un panel d’aides, comme l’extension Seraplot dans VSCode, qui vous permettra de générer des plots ou des méthodes ML très rapidement et en live, entre chaque sauvegarde de vos scripts.

En plus de cela, Seraplot se voit distribué dans différents langages comme : JS/TS, C (C# & C++), Java, Rust, Python, R & Scala. L’objectif étant vraiment d’être ultra accessible : d’un langage à un autre, les commandes restent les mêmes pour plus de simplicité.

Pour résumer, Seraplot est un outil beaucoup plus pratique de ce qui existe déjà et complétement indépendant, en plus de compilé différente fonctionnalitée. En autre il permet la génération de plots 2D & 3D, mais aussi qui tend à proposer des méthodes liées au ML, que vous pourrez retrouver au cours de votre documentation. D’autres surprises vous attendent, que ce soit la possibilité de choisir différents thèmes, ou bien le système de chunks en cas de crash pour reprendre au point d’erreur, ou encore pour n'en citer que un dernier, le fait d’avoir différents alias pour utiliser une même méthode (ex : sp.build_bar_chart / sp.bar_chart / sp.bar / sp.bars).

---

## 1 000 graphiques. Mesurés.

Même code, mêmes données aléatoires, même machine. Sortie HTML complète chronométrée.

<div class="sp-tabs" id="g-bench-fr">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('g-bench-fr','gb-sp-fr',this)">SeraPlot — 6 ms</button>
<button class="sp-tb" onclick="spTab('g-bench-fr','gb-pl-fr',this)">Plotly — 37 s</button>
<button class="sp-tb" onclick="spTab('g-bench-fr','gb-mp-fr',this)">Matplotlib — 60 s</button>
</div>
<div id="gb-sp-fr" class="sp-tc sp-on">
<pre><code class="language-python">import seraplot as sp
categories = ["Électronique", "Vêtements", "Alimentation", "Livres", "Sport", "Jouets", "Santé", "Auto"]
data = [...]
for i in range(1000):
    sp.bar(f"Rapport #{i+1}", categories, data[i]).html</code></pre>
<p style="padding:10px 16px;margin:0;color:#6366f1;font-weight:700">1 000 graphiques en 6 ms — 6 µs/graphique</p>
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
<p style="padding:10px 16px;margin:0;color:#ef4444;font-weight:700">1 000 graphiques en 37 023 ms — 6 170× plus lent</p>
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
<p style="padding:10px 16px;margin:0;color:#ef4444;font-weight:700">1 000 graphiques en 60 352 ms — 10 058× plus lent</p>
<iframe src="../previews/bench-matplotlib.html" style="width:100%;height:480px;border:none;border-top:1px solid #334155" loading="lazy"></iframe>
</div>
</div>

| Échelle | SeraPlot | Plotly | Matplotlib |
|---------|:--------:|:------:|:----------:|
| 1 000 graphiques | **6 ms** | 37 s | 60 s |
| 10 000 graphiques | **~60 ms** | ~6 min | ~10 min |
| 100 000 graphiques | **~600 ms** | ~1 h | ~1,7 h |

---

## Vitesse du moteur de rendu

**Benchmark : dataset Diabetes (n=768, 40 itérations). Temps de rendu Rust — création de l'objet graphique, pas la sérialisation HTML complète.**

<div style="font-family:monospace;margin:1.2em 0">
<div style="display:flex;align-items:center;gap:12px;margin:6px 0">
  <span style="min-width:130px;font-size:.85em">Camembert</span>
  <div style="background:#6366f1;width:300px;height:16px;border-radius:3px"></div>
  <b style="color:#6366f1;font-size:.85em">7 956×</b>
</div>
<div style="display:flex;align-items:center;gap:12px;margin:6px 0">
  <span style="min-width:130px;font-size:.85em">Barres</span>
  <div style="background:#6366f1;width:245px;height:16px;border-radius:3px"></div>
  <b style="color:#6366f1;font-size:.85em">6 488×</b>
</div>
<div style="display:flex;align-items:center;gap:12px;margin:6px 0">
  <span style="min-width:130px;font-size:.85em">Sucette</span>
  <div style="background:#6366f1;width:150px;height:16px;border-radius:3px"></div>
  <b style="color:#6366f1;font-size:.85em">3 983×</b>
</div>
<div style="display:flex;align-items:center;gap:12px;margin:6px 0">
  <span style="min-width:130px;font-size:.85em">Barres groupées</span>
  <div style="background:#6366f1;width:136px;height:16px;border-radius:3px"></div>
  <b style="color:#6366f1;font-size:.85em">3 596×</b>
</div>
<div style="display:flex;align-items:center;gap:12px;margin:6px 0">
  <span style="min-width:130px;font-size:.85em">Bougie</span>
  <div style="background:#6366f1;width:77px;height:16px;border-radius:3px"></div>
  <b style="color:#6366f1;font-size:.85em">2 038×</b>
</div>
<div style="display:flex;align-items:center;gap:12px;margin:6px 0">
  <span style="min-width:130px;font-size:.85em">Radar</span>
  <div style="background:#6366f1;width:56px;height:16px;border-radius:3px"></div>
  <b style="color:#6366f1;font-size:.85em">1 498×</b>
</div>
<div style="display:flex;align-items:center;gap:12px;margin:6px 0">
  <span style="min-width:130px;font-size:.85em">KDE</span>
  <div style="background:#6366f1;width:28px;height:16px;border-radius:3px"></div>
  <b style="color:#6366f1;font-size:.85em">753×</b>
</div>
</div>

| Graphique | SeraPlot | Plotly figure | Plotly → HTML | Matplotlib |
|-----------|----------|--------------|-------------|------------|
| Camembert | **4,2** | 725 | 33 416 | 15 085 |
| Barres | **2,8** | 658 | 18 166 | 13 596 |
| Barres groupées | **5,0** | 558 | 17 981 | 17 445 |
| Histogramme | **12,4** | 2 496 | 32 762 | 37 973 |
| Nuage de points | **17,0** | 3 916 | 21 615 | 14 141 |
| Violon | **16,7** | 2 616 | 21 347 | 21 211 |
| Boîte à moustaches | **18,4** | 2 329 | 21 799 | 15 590 |
| KDE | **26,3** | 2 981 | 19 807 | 40 108 |
| Radar | **11,8** | 962 | 17 679 | 20 942 |
| Sucette | **6,3** | 8 382 | 25 096 | 9 072 |
| Bougie | **8,8** | 1 478 | 17 934 | N/A |
| Ridgeline | **88,8** | N/A | N/A | N/A |

Toutes les valeurs en µs.

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

## Navigation

- **[Installation](installation.md)** — `pip install seraplot`
- **[Démarrage rapide](quickstart.md)** — premier graphique en 3 lignes
- **[Méthodes des graphiques](chart-methods.md)** — toutes les méthodes universelles Chart
- **[Graphiques 2D](../charts/2d/bar.md)** — 33 types
- **[Graphiques 3D](../charts/3d/scatter3d.md)** — 17 types, rendu GPU WebGL
- **[Machine Learning](../ml/dbscan.md)** — DBSCAN jusqu'à 600× plus rapide que scikit-learn

</div>
