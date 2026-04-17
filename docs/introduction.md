<div align="center">

# SeraPlot

**Rust-native charting. 6,000× faster than Plotly. Zero dependencies.**

[![PyPI](https://img.shields.io/pypi/v/seraplot)](https://pypi.org/project/seraplot/)
[![npm](https://img.shields.io/npm/v/seraplot)](https://www.npmjs.com/package/seraplot)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/feur25/seraplot/blob/main/LICENSE)

</div>

<div class="lang-en">

```bash
pip install seraplot
```

---

## The switch

```python
import matplotlib.pyplot as plt

import seraplot.matplotlib as plt
```

- 6,000× faster than Plotly (measured, 1000 bar charts)
- 200× smaller files
- interactive HTML by default
- zero Python dependencies

---

## The moment it clicks

```python
import seraplot as sp

sp.scatter("1M points", list(range(1_000_000)), [x**0.5 for x in range(1_000_000)]).show()
```

**1 million points. Interactive. In Jupyter.**

No config. No backend. No waiting.

---

## Why people actually switch

People don't switch because of benchmarks.
They switch because something becomes **possible**.

### Dashboards without a backend

Each chart is a self-contained HTML file. Drop it anywhere — S3, email, Git, Notion.
No server, no Dash, no Streamlit.

### Charts by email

21 KB interactive HTML vs Plotly's 4.7 MB static blob.

### CI pipelines at scale

1,000 charts in 6 ms. Not 37 seconds. Not 60 seconds.

### Offline-first apps

No CDN. No JS framework. No internet at render time. Works air-gapped.

### APIs that return charts

Return HTML directly from a FastAPI endpoint. 21 KB in the response body, no temp files.

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
<iframe src="previews/bench-seraplot.html" style="width:100%;height:480px;border:none;border-top:1px solid #334155" loading="lazy"></iframe>
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
<iframe src="previews/bench-plotly.html" style="width:100%;height:480px;border:none;border-top:1px solid #334155" loading="lazy"></iframe>
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
<iframe src="previews/bench-matplotlib.html" style="width:100%;height:480px;border:none;border-top:1px solid #334155" loading="lazy"></iframe>
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

## 7 themes

```python
sp.theme("dark")
sp.theme("apple")
sp.theme("notion")
sp.theme("scientific")
sp.theme("neon")
sp.theme("minimal")
sp.theme("light")

sp.reset_theme()
```

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
  if (window.hljs) document.getElementById(id).querySelectorAll('code').forEach(function(c){hljs.highlightElement(c);});
}
document.addEventListener('DOMContentLoaded', function() {
  if (window.hljs) document.querySelectorAll('.sp-tc code').forEach(function(c){hljs.highlightElement(c);});
});
</script>

Plotly returns 4.7 MB per request. Matplotlib requires disk I/O and returns a static PNG.
SeraPlot returns 21 KB of interactive HTML directly from RAM.

---

## When you might not need it

- quick one-off scripts where render time doesn't matter
- heavy JS customization (plotly.js ecosystem)
- your team only reads static PNG reports

SeraPlot is built for production throughput.
If all you need is 3 charts in a notebook, matplotlib works fine.

---

## Global config

Set once, every chart inherits:

```python
sp.config(
    font="Inter",
    font_size=14,
    title_size=22,
    crosshair=True,
    zoom=True,
    animation=True,
    export_button=True,
    responsive=True,
    border_radius=12,
    margin=16,
    opacity=0.85,
    background="#0f172a",
    palette=[0x818CF8, 0xFB7185, 0x34D399],
    gridlines=True,
)

sp.bar("Revenue", labels, values)   # inherits everything
sp.line("Trend", dates, values)     # same config
sp.scatter("Clusters", x, y)        # same config
```

Per-chart override with method chaining:

```python
sp.bar("Revenue", labels, values).font("Roboto").zoom(False)
```

Reset everything:

```python
sp.reset_config()
```

| Parameter | Type | Effect |
|-----------|------|--------|
| `font` | str | Font family for all text |
| `font_size` | int | Base font size (px) |
| `title_size` | int | Title font size (px) |
| `crosshair` | bool | Crosshair lines on hover |
| `zoom` | bool | Mouse wheel zoom + pan |
| `animation` | bool | Fade-in animation on elements |
| `animation_duration` | int | Animation duration (ms) |
| `export_button` | bool | Download button on each chart |
| `responsive` | bool | Auto-resize to container width |
| `border_radius` | int | Chart container border radius (px) |
| `margin` | int | Chart container padding (px) |
| `opacity` | float | Element opacity (0.0–1.0) |
| `background` | str | Background color |
| `palette` | list[int] | Color palette (hex ints) |
| `gridlines` | bool | Show grid lines |
| `locale` | str | Number formatting locale |
| `thousands_sep` | str | Thousands separator character |
| `tooltip` | str | Tooltip mode |

### Chain methods (per-chart)

| Method | Effect |
|--------|--------|
| `.font("Inter")` | Override font family |
| `.title_size(22)` | Override title font size |
| `.set_font_size(14)` | Override base font size |
| `.crosshair()` | Enable crosshair |
| `.zoom()` | Enable zoom + pan |
| `.animate(300)` | Enable animation (ms) |
| `.export_button()` | Add download button |
| `.responsive()` | Auto-resize |
| `.border_radius(12)` | Set border radius |
| `.set_opacity(0.85)` | Set element opacity |
| `.set_margin(16)` | Set chart padding |
| `.set_bg("#0f172a")` | Set background color |
| `.inject_css("...")` | Inject custom CSS |
| `.inject_js("...")` | Inject custom JS |

---

## Native machine learning

Rust-native DBSCAN with KD-tree spatial indexing:

| Points | scikit-learn | SeraPlot DBSCAN | Factor |
|--------|-------------|-----------------|--------|
| 1,000 | 3.2 ms | 0.18 ms | **18×** |
| 10,000 | 54 ms | 1.1 ms | **49×** |
| 100,000 | 1,340 ms | 8.4 ms | **160×** |
| 500,000 | 21,000 ms | 38 ms | **553×** |

---

## Everything SeraPlot does

- **57 chart types** — every 2D chart has a 3D WebGL variant
- **Drop-in matplotlib API** — `import seraplot.matplotlib as plt`
- **Pandas & NumPy native** — pass DataFrames directly
- **7 built-in themes** — dark, light, scientific, apple, notion, minimal, neon
- **Global config** — `sp.config()` sets font, zoom, crosshair, animation across all charts
- **Zero dependencies** — pure Rust renderer
- **200× smaller files** — no bundled JS runtime
- **Multi-language** — Python, JavaScript/TypeScript (npm), Rust
- **DBSCAN up to 600× faster** than scikit-learn
- **Works everywhere** — Python ≥ 3.8, any OS

---

## Navigation

- **[Installation](getting-started/installation.md)** — `pip install seraplot`
- **[Quick Start](getting-started/quickstart.md)** — first chart in 3 lines
- **[Chart Methods](getting-started/chart-methods.md)** — all universal Chart methods
- **[2D Charts](charts/2d/bar.md)** — 33 chart types
- **[3D Charts](charts/3d/scatter3d.md)** — 17 chart types, WebGL GPU renderer
- **[Machine Learning](ml/dbscan.md)** — DBSCAN up to 600x faster than scikit-learn
- **[API Reference](api/index.md)** — complete function index

</div>

<div class="lang-fr">

# SeraPlot

Visualisation de données native Rust. **6 000× plus rapide que Plotly. Zéro dépendance.**

```bash
pip install seraplot
```

## Le déclencheur
```python
import seraplot as sp

sp.scatter("1M points", list(range(1_000_000)), [x**0.5 for x in range(1_000_000)]).show()
```

**1 million de points. Interactif. Dans Jupyter.**

Aucune configuration. Aucun backend. Aucune attente.

---

## Pourquoi les gens changent vraiment

Les gens ne changent pas à cause des benchmarks. Ils changent parce que quelque chose devient **possible**.

### Tableaux de bord sans backend
Chaque graphique est un fichier HTML autonome. Déposez-le où vous voulez — S3, e-mail, Git, Notion. Pas de serveur, pas de Dash, pas de Streamlit.

### Graphiques par e-mail
21 Ko de HTML interactif contre 4,7 Mo de blob statique Plotly.

### Pipelines CI à grande échelle
1 000 graphiques en 6 ms. Pas 37 secondes. Pas 60 secondes.

### Applications hors ligne
Pas de CDN. Pas de framework JS. Pas d'internet au rendu. Fonctionne en environnement isolé.

### APIs qui retournent des graphiques
Retournez du HTML directement depuis un endpoint FastAPI. 21 Ko dans le corps de la réponse, pas de fichiers temporaires.

---

## Même graphique — trois bibliothèques

|               | SeraPlot  | Plotly | Matplotlib |
| ------------- | :-------: | :----: | :--------: |
| Lignes de code | **2**    |    4   |      7     |
| Sortie         | HTML      |  HTML  |     PNG    |
| Taille fichier | **21 Ko** | 4,7 Mo |   ~150 Ko  |
| Interactif     | ✅        |   ✅   |      ❌     |
| Dépendances    | **0**     |   6+   |     3+     |
| Migration 1 ligne | ✅     |   —    |      —     |

---

## 1 000 graphiques. Mesurés.

Même code, mêmes données aléatoires, même machine. Sortie HTML complète chronométrée.

| Échelle | SeraPlot | Plotly | Matplotlib |
|---------|:--------:|:------:|:----------:|
| 1 000 graphiques | **6 ms** | 37 s | 60 s |
| 10 000 graphiques | **~60 ms** | ~6 min | ~10 min |
| 100 000 graphiques | **~600 ms** | ~1 h | ~1,7 h |

---

## Vitesse du moteur de rendu

**Benchmark : dataset Diabetes (n=768, 40 itérations). Temps de rendu Rust — création de l'objet graphique, pas la sérialisation HTML complète.**

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

Plotly embarque tout son bundle JavaScript dans chaque fichier HTML. SeraPlot n'inclut que le JS nécessaire au type de graphique spécifique.

| Graphique | SeraPlot | Plotly | Ratio |
|-----------|:--------:|:------:|:-----:|
| Camembert | 19 Ko | 4 733 Ko | **246×** |
| Barres | 21 Ko | 4 733 Ko | **225×** |
| Nuage de points | 39 Ko | 4 740 Ko | **121×** |
| Radar | 23 Ko | 4 733 Ko | **205×** |

Matplotlib produit du PNG/SVG/PDF (50–500 Ko) — pas du HTML interactif.

---

## Ce qu'est réellement SeraPlot

SeraPlot n'est pas un wrapper autour de Plotly, Chart.js, ou D3.

C'est un **moteur de rendu natif Rust** qui génère du HTML + JS minimal par graphique. Un camembert reçoit le JS camembert. Un graphique en barres reçoit le JS barres. Rien d'autre n'est inclus.

C'est pourquoi la sortie fait 20 Ko au lieu de 4,7 Mo.

---

## Configuration globale

Définir une fois, tous les graphiques héritent :

```python
sp.config(
    font="Inter",
    font_size=14,
    title_size=22,
    crosshair=True,
    zoom=True,
    animation=True,
    export_button=True,
    responsive=True,
    border_radius=12,
    margin=16,
    opacity=0.85,
    background="#0f172a",
    palette=[0x818CF8, 0xFB7185, 0x34D399],
    gridlines=True,
)

sp.bar("Revenus", labels, values)   # hérite de tout
sp.line("Tendance", dates, values)  # même config
sp.scatter("Clusters", x, y)        # même config
```

Surcharge par graphique avec chaînage de méthodes :

```python
sp.bar("Revenus", labels, values).font("Roboto").zoom(False)
```

Réinitialiser tout :

```python
sp.reset_config()
```

| Paramètre | Type | Effet |
|-----------|------|-------|
| `font` | str | Police pour tout le texte |
| `font_size` | int | Taille de base (px) |
| `title_size` | int | Taille du titre (px) |
| `crosshair` | bool | Réticule au survol |
| `zoom` | bool | Zoom molette + panoramique |
| `animation` | bool | Animation d'apparition |
| `animation_duration` | int | Durée d'animation (ms) |
| `export_button` | bool | Bouton de téléchargement |
| `responsive` | bool | Redimensionnement automatique |
| `border_radius` | int | Rayon des coins du conteneur (px) |
| `margin` | int | Marges internes (px) |
| `opacity` | float | Opacité des éléments (0,0–1,0) |
| `background` | str | Couleur de fond |
| `palette` | list[int] | Palette de couleurs (entiers hex) |
| `gridlines` | bool | Afficher les lignes de grille |
| `locale` | str | Locale pour le formatage des nombres |
| `thousands_sep` | str | Séparateur des milliers |
| `tooltip` | str | Mode infobulle |

### Méthodes de chaînage (par graphique)

| Méthode | Effet |
|---------|-------|
| `.font("Inter")` | Surcharger la police |
| `.title_size(22)` | Surcharger la taille du titre |
| `.set_font_size(14)` | Surcharger la taille du texte |
| `.crosshair()` | Activer le réticule |
| `.zoom()` | Activer le zoom + panoramique |
| `.animate(300)` | Activer l'animation (ms) |
| `.export_button()` | Ajouter un bouton de téléchargement |
| `.responsive()` | Redimensionnement automatique |
| `.border_radius(12)` | Définir le rayon des coins |
| `.set_opacity(0.85)` | Définir l'opacité des éléments |
| `.set_margin(16)` | Définir les marges internes |
| `.set_bg("#0f172a")` | Définir la couleur de fond |
| `.inject_css("...")` | Injecter du CSS personnalisé |
| `.inject_js("...")` | Injecter du JS personnalisé |

---

## 7 thèmes

```python
sp.theme("dark")
sp.theme("apple")
sp.theme("notion")
sp.theme("scientific")
sp.theme("neon")
sp.theme("minimal")
sp.theme("light")

sp.reset_theme()
```

---

## Machine Learning natif

DBSCAN natif Rust avec indexation spatiale KD-tree :

| Points | scikit-learn | SeraPlot DBSCAN | Facteur |
|--------|-------------|-----------------|---------|
| 1 000 | 3,2 ms | 0,18 ms | **18×** |
| 10 000 | 54 ms | 1,1 ms | **49×** |
| 100 000 | 1 340 ms | 8,4 ms | **160×** |
| 500 000 | 21 000 ms | 38 ms | **553×** |

---

## Déployer depuis une API

```python
from fastapi import FastAPI
import seraplot as sp
app = FastAPI()

@app.get("/chart")
def revenue_chart():
    return sp.bar("Revenus", labels, values).html
```

Plotly retourne 4,7 Mo par requête. Matplotlib nécessite des E/S disque et retourne un PNG statique. SeraPlot retourne 21 Ko de HTML interactif directement depuis la RAM.

---

## Migration en une ligne

```python
import seraplot.matplotlib as plt
```

Tout le reste reste identique. `plt.bar()`, `plt.scatter()`, `plt.hist()`, `plt.show()`, `plt.savefig()` — inchangés.

---

## Tout ce que fait SeraPlot

- **57 types de graphiques** — chaque graphique 2D a une variante 3D WebGL
- **API matplotlib compatible** — `import seraplot.matplotlib as plt`
- **Pandas & NumPy natifs** — passez des DataFrames directement
- **7 thèmes intégrés** — dark, light, scientific, apple, notion, minimal, neon
- **Configuration globale** — `sp.config()` définit police, zoom, réticule, animation pour tous
- **Zéro dépendance** — moteur de rendu Rust pur
- **200× plus petits** — pas de runtime JS inclus
- **DBSCAN jusqu'à 600× plus rapide** que scikit-learn
- **Fonctionne partout** — Python ≥ 3.8, tout OS

---

## Navigation

- **[Installation](getting-started/installation.md)** — `pip install seraplot`
- **[Démarrage rapide](getting-started/quickstart.md)** — premier graphique en 3 lignes
- **[Méthodes du graphique](getting-started/chart-methods.md)** — toutes les méthodes universelles
- **[Graphiques 2D](charts/2d/bar.md)** — 33 types
- **[Graphiques 3D](charts/3d/scatter3d.md)** — 17 types, rendu WebGL GPU
- **[Machine Learning](ml/dbscan.md)** — DBSCAN jusqu'à 600× plus rapide que scikit-learn
- **[Référence API](api/index.md)** — index complet des fonctions

</div>
```python
import seraplot as sp

sp.scatter("1M points", list(range(1_000_000)), [x**0.5 for x in range(1_000_000)]).show()
```

**1 million de points. Interactif. Dans Jupyter.**

Aucune configuration. Aucun backend. Aucune attente.

---

## Pourquoi les gens changent vraiment

#### Tableaux de bord sans backend
Chaque graphique est un fichier HTML autonome. Déposez-le où vous voulez — S3, e-mail, Git, Notion. Pas de serveur, pas de Dash, pas de Streamlit.

#### Graphiques par e-mail
21 Ko de HTML interactif contre 4,7 Mo de blob statique Plotly.

#### Pipelines CI à grande échelle
1 000 graphiques en 6 ms. Pas 37 secondes. Pas 60 secondes.

#### Applications hors ligne
Pas de CDN. Pas de framework JS. Pas d'internet au rendu. Fonctionne en environnement isolé.

#### APIs qui retournent des graphiques
Retournez du HTML directement depuis un endpoint FastAPI. 21 Ko dans le corps de la réponse, pas de fichiers temporaires.

---

## Même graphique — trois bibliothèques

| | SeraPlot | Plotly | Matplotlib |
|---|:---:|:---:|:---:|
| Lignes de code | **2** | 4 | 7 |
| Sortie | HTML | HTML | PNG |
| Taille du fichier | **21 Ko** | 4,7 Mo | ~150 Ko |
| Interactif | ✅ | ✅ | ❌ |
| Dépendances | **0** | 6+ | 3+ |
| Migration 1 ligne | ✅ | — | — |

---

## 1 000 graphiques. Mesurés.

Même code, mêmes données aléatoires, même machine. Sortie HTML complète chronométrée.

| Échelle | SeraPlot | Plotly | Matplotlib |
|-------|:--------:|:------:|:----------:|
| 1 000 graphiques | **6 ms** | 37 s | 60 s |
| 10 000 graphiques | **~60 ms** | ~6 min | ~10 min |
| 100 000 graphiques | **~600 ms** | ~1 h | ~1,7 h |

---

## Taille des fichiers de sortie

Plotly embarque tout son bundle JavaScript dans chaque fichier HTML. SeraPlot n'inclut que le JS nécessaire au type de graphique spécifique.

| Graphique | SeraPlot | Plotly | Ratio |
|-----------|:--------:|:------:|:-----:|
| Camembert | 19 Ko | 4 733 Ko | 246× |
| Barres | 21 Ko | 4 733 Ko | 225× |
| Nuage de points | 39 Ko | 4 740 Ko | 121× |
| Radar | 23 Ko | 4 733 Ko | 205× |

---

## Configuration globale

Définir une fois, tous les graphiques héritent :

```python
sp.config(
    font="Inter",
    font_size=14,
    title_size=22,
    crosshair=True,
    zoom=True,
    animation=True,
    export_button=True,
    responsive=True,
    border_radius=12,
    margin=16,
    opacity=0.85,
    background="#0f172a",
    palette=[0x818CF8, 0xFB7185, 0x34D399],
    gridlines=True,
)

sp.bar("Revenus", labels, values)   # hérite de tout
sp.line("Tendance", dates, values)  # même config
sp.scatter("Clusters", x, y)        # même config
```

---

## 7 thèmes

```python
sp.theme("dark")
sp.theme("apple")
sp.theme("notion")
sp.theme("scientific")
sp.theme("neon")
sp.theme("minimal")
sp.theme("light")

sp.reset_theme()
```

---

## Migration en une ligne

```python
import seraplot.matplotlib as plt
```

Tout le reste reste identique. `plt.bar()`, `plt.scatter()`, `plt.hist()`, `plt.show()`, `plt.savefig()` — inchangés.

---

## Par où commencer

- **[Installation](getting-started/installation.md)**
- **[Démarrage rapide](getting-started/quickstart.md)**
- **[Graphiques 2D](charts/2d/bar.md)** — 33 types
- **[Graphiques 3D](charts/3d/scatter3d.md)** — 17 types, rendu WebGL GPU
- **[Machine Learning](ml/dbscan.md)** — DBSCAN jusqu'à 600× plus rapide que scikit-learn
- **[Référence API](api/index.md)** — index complet des fonctions

</div>
