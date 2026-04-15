<div align="center">

# SeraPlot

**Your plots. Interactive. In milliseconds.**

[![PyPI](https://img.shields.io/pypi/v/seraplot)](https://pypi.org/project/seraplot/)
[![npm](https://img.shields.io/npm/v/seraplot)](https://www.npmjs.com/package/seraplot)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/feur25/seraplot/blob/main/LICENSE)

</div>

```bash
pip install seraplot
```

**Go from 2 seconds to 2 milliseconds. Ship interactive HTML charts with zero dependencies. Drop in as a matplotlib replacement in one line.**

---

## Drop in as a matplotlib replacement

```python
import seraplot.matplotlib as plt   # same mental model as matplotlib

plt.theme("dark")
plt.title("Revenue Q1-Q4")
plt.xlabel("Quarter")
plt.bar(["Q1", "Q2", "Q3", "Q4"], [4.2, 5.1, 4.8, 6.2])
plt.show()             # renders inline in Jupyter
plt.savefig("chart.html")
```

Or use the native API with pandas, numpy, or plain lists:

```python
import seraplot as sp

sp.theme("apple")
chart = sp.bar(df)     # DataFrame, numpy array, or plain list — all work
chart.show()           # inline in Jupyter
chart.save("chart.html")
```

---

## 7 themes, one call

```python
sp.theme("dark")        # dark background, vibrant palette, gridlines on
sp.theme("apple")       # iOS colors on black
sp.theme("notion")      # warm editorial dark
sp.theme("scientific")  # D3 palette on white, gridlines on
sp.theme("neon")        # neon on black
sp.theme("minimal")     # grayscale, data-first
sp.theme("light")       # bright default

sp.reset_theme()        # back to defaults
```

Themes apply globally to every chart until reset — one call configures your entire session.

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
.sp-tc pre{margin:0;border-radius:0;overflow:auto;padding:14px 16px;box-sizing:border-box}
.sp-tc code{display:block;line-height:1.45}
</style>

<div class="sp-tabs" id="g1">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('g1','g1a',this)">SeraPlot — 2 lines</button>
<button class="sp-tb" onclick="spTab('g1','g1b',this)">Plotly — 4 lines</button>
<button class="sp-tb" onclick="spTab('g1','g1c',this)">Matplotlib — 7 lines</button>
</div>
<div id="g1a" class="sp-tc sp-on"><pre style="margin:0;border-radius:0;overflow:auto;padding:14px 16px;box-sizing:border-box"><code class="language-python">import seraplot as sp
sp.build_bar_chart("Revenue by Product", labels, values).save("chart.html")</code></pre></div>
<div id="g1b" class="sp-tc"><pre style="margin:0;border-radius:0;overflow:auto;padding:14px 16px;box-sizing:border-box"><code class="language-python">import plotly.express as px
fig = px.bar(x=labels, y=values, title="Revenue by Product")
fig.update_layout(template="plotly_white")
fig.write_html("chart.html")</code></pre></div>
<div id="g1c" class="sp-tc"><pre style="margin:0;border-radius:0;overflow:auto;padding:14px 16px;box-sizing:border-box"><code class="language-python">import matplotlib.pyplot as plt
fig, ax = plt.subplots(figsize=(9, 5))
ax.bar(labels, values, color="#6366f1")
ax.set_title("Revenue by Product")
ax.set_ylabel("Revenue")
plt.tight_layout()
plt.savefig("chart.png")</code></pre></div>
</div>

| | SeraPlot | Plotly | Matplotlib |
|---|:---:|:---:|:---:|
| **Lines of code** | **2** | 4 | 7 |
| **Output** | HTML | HTML | PNG |
| **File size** | **21 KB** | 4.7 MB | ~150 KB |
| **Render time** | **2.8 µs** | 18,166 µs | 13,596 µs |
| **Python deps** | **0** | 6+ | 3+ |
| **Pandas native** | ✅ | — | — |
| **3D variants** | ✅ all | partial | — |
| **7 themes** | ✅ | — | — |
| **matplotlib-compatible API** | ✅ | — | — |

---

## Deploy from an API

<div class="sp-tabs" id="g2">
<div class="sp-tab-btns">
<button class="sp-tb sp-act" onclick="spTab('g2','g2a',this)">SeraPlot — 7 lines</button>
<button class="sp-tb" onclick="spTab('g2','g2b',this)">Plotly — 10 lines</button>
<button class="sp-tb" onclick="spTab('g2','g2c',this)">Matplotlib — 14 lines</button>
</div>
<div id="g2a" class="sp-tc sp-on"><pre style="margin:0;border-radius:0;overflow:auto;padding:14px 16px;box-sizing:border-box"><code class="language-python">from fastapi import FastAPI
import seraplot as sp

app = FastAPI()
@app.get("/chart")
def revenue_chart():
    return sp.build_bar_chart("Revenue", labels, values).html</code></pre></div>
<div id="g2b" class="sp-tc"><pre style="margin:0;border-radius:0;overflow:auto;padding:14px 16px;box-sizing:border-box"><code class="language-python">from fastapi import FastAPI
from fastapi.responses import HTMLResponse
import plotly.express as px

app = FastAPI()
@app.get("/chart", response_class=HTMLResponse)
def revenue_chart():
    fig = px.bar(x=labels, y=values, title="Revenue")
    return fig.to_html(full_html=True)</code></pre></div>
<div id="g2c" class="sp-tc"><pre style="margin:0;border-radius:0;overflow:auto;padding:14px 16px;box-sizing:border-box"><code class="language-python">from fastapi import FastAPI
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

Plotly returns 4.7 MB per request. Matplotlib cannot return interactive HTML and requires disk I/O. SeraPlot returns 21 KB of interactive HTML directly from RAM.

---

## Speed

**Benchmark: Diabetes dataset (n=768, 40 runs). Render time includes HTML export.**

Speedup of SeraPlot vs Plotly exporting to HTML — the fair comparison for tools that produce embeddable output.

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

Raw numbers (µs):

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

Average speedup vs Plotly → HTML: **~3,500×**.

---

## Output file size

Plotly embeds its entire JavaScript bundle in every HTML export. A single Pie chart from Plotly is **4.7 MB**. The same chart from SeraPlot is **19 KB** — because SeraPlot only includes the minimal JavaScript needed for that specific chart type, not a general-purpose charting framework.

<div style="font-family:monospace;margin:1.2em 0">
<div style="display:flex;align-items:center;gap:8px;margin:6px 0">
  <span style="min-width:90px;font-size:.85em">Pie</span>
  <div style="display:flex;gap:3px;align-items:center">
    <div style="background:#6366f1;width:5px;height:16px;border-radius:2px" title="SeraPlot 19 KB"></div>
    <div style="background:#ef4444;width:300px;height:16px;border-radius:2px" title="Plotly 4,733 KB"></div>
  </div>
  <span style="font-size:.8em;color:#94a3b8">19 KB vs 4,733 KB &mdash; Plotly <b style="color:#ef4444">246×</b> larger</span>
</div>
<div style="display:flex;align-items:center;gap:8px;margin:6px 0">
  <span style="min-width:90px;font-size:.85em">Bar</span>
  <div style="display:flex;gap:3px;align-items:center">
    <div style="background:#6366f1;width:5px;height:16px;border-radius:2px"></div>
    <div style="background:#ef4444;width:274px;height:16px;border-radius:2px"></div>
  </div>
  <span style="font-size:.8em;color:#94a3b8">21 KB vs 4,733 KB &mdash; Plotly <b style="color:#ef4444">225×</b> larger</span>
</div>
<div style="display:flex;align-items:center;gap:8px;margin:6px 0">
  <span style="min-width:90px;font-size:.85em">Scatter</span>
  <div style="display:flex;gap:3px;align-items:center">
    <div style="background:#6366f1;width:5px;height:16px;border-radius:2px"></div>
    <div style="background:#ef4444;width:148px;height:16px;border-radius:2px"></div>
  </div>
  <span style="font-size:.8em;color:#94a3b8">39 KB vs 4,740 KB &mdash; Plotly <b style="color:#ef4444">121×</b> larger</span>
</div>
<div style="display:flex;align-items:center;gap:8px;margin:6px 0">
  <span style="min-width:90px;font-size:.85em">Radar</span>
  <div style="display:flex;gap:3px;align-items:center">
    <div style="background:#6366f1;width:5px;height:16px;border-radius:2px"></div>
    <div style="background:#ef4444;width:250px;height:16px;border-radius:2px"></div>
  </div>
  <span style="font-size:.8em;color:#94a3b8">23 KB vs 4,733 KB &mdash; Plotly <b style="color:#ef4444">205×</b> larger</span>
</div>
</div>

Matplotlib outputs PNG/SVG/PDF (50-500 KB) — not self-contained interactive HTML.

---

## What SeraPlot actually is

SeraPlot is **not a wrapper** around Plotly, Chart.js, D3, or any JavaScript library. It is a standalone renderer written in Rust that produces HTML with its own minimal embedded JavaScript.

Each chart type has its own focused JS implementation. A Pie chart gets Pie JS. A Bar chart gets Bar JS. Nothing else is bundled. This is why the output is 20 KB instead of 4.7 MB.

**Concrete consequences:**

- No internet connection at render time — no CDN calls, no external fonts, no remote scripts
- Render latency in microseconds — thousands of charts per second in a pipeline
- Output files small enough to embed in emails, check into Git, or serve from a static host
- Zero Python dependency conflicts — SeraPlot has no required Python dependencies
- 57+ chart types including 17 WebGL 3D charts and a Rust-native DBSCAN 600× faster than scikit-learn

---

## Full render control — no exceptions

Every visual property is controllable via a fluent method chain:

```python
chart = (
    sp.build_bar_chart("Monthly Revenue", labels, values)
    .set_bg("#0f172a")
    .show_grid()
    .no_axes()
    .show_labels(position="top")
    .inject_css(".sp-gl { stroke: #334155 !important; } svg text { fill: #e2e8f0 !important; }")
    .inject_js("document.querySelector('svg').style.cursor = 'crosshair';")
)
```

| Method | Effect |
|--------|--------|
| `set_bg(color)` | Background color of the HTML wrapper |
| `set_global_background(color)` | Applied to every chart in the session |
| `show_grid()` / `hide_grid()` | Force gridlines on / off |
| `no_x_axis()` / `no_y_axis()` / `no_axes()` | Remove axes selectively |
| `show_labels(position, labels, colors)` | Value labels on each element |
| `no_legend()` | Remove the legend |
| `no_title()` | Remove the title |
| `set_font_size(px)` | Override all text sizes |
| `inject_css(css)` | Inject arbitrary CSS — full DOM access |
| `inject_js(js)` | Inject arbitrary JavaScript — unlimited behaviour |

---

## Rich tooltips: images, videos, HTML

```python
hover = sp.build_hover_json({
    "Product":  ["Widget A",                   "Widget B",  "Widget C"],
    "Revenue":  ["$142,000",                   "$98,500",   "$210,000"],
    "image":    ["https://cdn.acme.com/a.png", "...",       "..."    ],
})
chart = sp.build_bar_chart("Product Revenue", labels, values, hover_json=hover)
```

Each data point has its own independent tooltip: key/value rows, inline images, inline video, or raw HTML.

---

## Cross-language: one Rust engine, every surface

The same compiled binary is callable from Python (PyO3 wheels), JavaScript/TypeScript (npm + WASM), Rust (native), and any language with a C FFI.

```bash
pip install seraplot       # Python
npm install seraplot        # JS/TS
```

You get the same microsecond render latency from any language.

---

## Throughput that unlocks new products

| Use case | At 18 µs per scatter chart |
|----------|---------------------------|
| 1,000 custom charts per HTTP request | **18 ms** — inline in the response |
| 100,000 charts per CI run | **1.8 seconds** — feasible on every commit |
| 1,000,000 A/B test variants | **18 seconds** — a single command |
| One chart per row, 10,000-row DataFrame export | **180 ms** — no extra infrastructure |

---

## Native machine learning

SeraPlot ships a DBSCAN written in Rust with KD-tree spatial indexing and SIMD acceleration — not a wrapper around scikit-learn:

| Points | scikit-learn | SeraPlot DBSCAN | Factor |
|--------|-------------|-----------------|--------|
| 1,000 | 3.2 ms | 0.18 ms | **18×** |
| 10,000 | 54 ms | 1.1 ms | **49×** |
| 100,000 | 1,340 ms | 8.4 ms | **160×** |
| 500,000 | 21,000 ms | 38 ms | **553×** |

Cluster and render in the same library — no scikit-learn, no intermediate format, no conversion step.

---

## Everything SeraPlot does

- **57 chart types** — every 2D chart has a 3D WebGL variant
- **Drop-in matplotlib API** — `import seraplot.matplotlib as plt`
- **Pandas & NumPy native** — pass DataFrames directly, no `.values.tolist()`
- **7 built-in themes** — dark, light, scientific, apple, notion, minimal, neon
- **Global parameters** — `sp.theme()`, `sp.set_global_background()`, `sp.reset_theme()`
- **Multiple output formats** — HTML, SVG, PNG, JPG, SeraPlot desktop app
- **Multi-language** — Python, JavaScript/TypeScript (npm), Rust, C# (via C ABI)
- **Zero dependencies** — pure Rust renderer, no Python deps at runtime
- **700× smaller files** — self-contained HTML without a bundled runtime
- **DBSCAN 600× faster** — benchmarked on Open Food Facts (n=300K)
- **Exclusive chart types** — Ridgeline, Dumbbell, Slope, Bullet, Globe 3D, Slideshow
- **Works everywhere** — any Python ≥ 3.8, any OS, no conda, no system libs

---

## Navigation

- **[Installation](getting-started/installation.md)** — `pip install seraplot`
- **[Quick Start](getting-started/quickstart.md)** — first chart in 3 lines
- **[Chart Methods](getting-started/chart-methods.md)** — all universal Chart methods
- **[2D Charts](charts/2d/bar.md)** — 33 chart types
- **[3D Charts](charts/3d/scatter3d.md)** — 17 chart types, WebGL GPU renderer
- **[Machine Learning](ml/dbscan.md)** — DBSCAN up to 600x faster than scikit-learn
- **[API Reference](api/index.md)** — complete function index

