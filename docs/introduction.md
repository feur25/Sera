<div align="center">

# SeraPlot

**High-performance data visualization — Rust core, Python API**

[![PyPI](https://img.shields.io/pypi/v/seraplot)](https://pypi.org/project/seraplot/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/feur25/seraplot/blob/main/LICENSE)

</div>

SeraPlot is a Python visualization library with a Rust rendering engine. You call Python functions, SeraPlot returns a self-contained HTML file — 20 to 90 KB, no CDN, no external JavaScript, works offline.

The entire render pipeline runs in compiled Rust. There is no Python overhead on the hot path, no JavaScript runtime at build time, and no network call at display time. The result: render times measured in **microseconds**, not milliseconds.

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

Raw numbers (µs, log scale would make SeraPlot invisible):

| Chart | SeraPlot | Plotly figure | Plotly →HTML | Matplotlib |
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

Average speedup vs Plotly →HTML: **~3,500×**. The KDE worst case is still 753×. Ridgeline does not exist in Plotly or Matplotlib.

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

Matplotlib outputs PNG/SVG/PDF (50–500 KB) — not self-contained HTML.

---

## What SeraPlot actually is

SeraPlot is **not a wrapper** around Plotly, Chart.js, D3, or any JavaScript library. It is a standalone renderer written in Rust that produces HTML with its own minimal embedded JavaScript.

Each chart type has its own focused JS implementation. A Pie chart gets Pie JS. A Bar chart gets Bar JS. Nothing else is bundled. This is why the output is 20 KB instead of 4.7 MB.

The Python API is a thin binding layer — it validates inputs, calls into the Rust library, and returns an object whose `.to_html()` method gives you a complete standalone HTML string.

**Concrete consequences of this architecture:**

- No internet connection at render time — no CDN calls, no external fonts, no remote scripts
- Render latency in microseconds — you can generate thousands of charts per second in a pipeline
- Output files small enough to embed in emails, check into Git, or serve from a static file host
- Zero Python dependency conflicts — SeraPlot has no required Python dependencies at all
- 55+ chart types including 17 WebGL 3D charts and a Rust-native DBSCAN that runs 600× faster than scikit-learn

---

## When to use SeraPlot vs the alternatives

**Use SeraPlot when you need HTML output and throughput matters.**

Report generators, CI pipelines, batch chart exports, Jupyter notebooks, email dashboards — anywhere you are producing embeddable HTML and either volume or file size is a constraint.

**Use Plotly when you need a live interactive web application.** Plotly has click events, Dash server integration, cross-chart filtering, and a full JavaScript API for custom behaviors. SeraPlot charts have hover tooltips but are not a frontend framework.

**Use Matplotlib for publication figures.** PDF output with LaTeX math rendering, precise layout control, and the widest academic toolchain support (seaborn, statsmodels, scikit-learn plot utilities all target it).

SeraPlot is not a stripped-down Plotly. It is a different tool built around a different constraint: **maximum throughput, minimum output size, zero runtime dependencies**.

---

## Navigation

- **[Installation](getting-started/installation.md)** — `pip install seraplot`
- **[Quick Start](getting-started/quickstart.md)** — first chart in 3 lines
- **[2D Charts](charts/2d/bar.md)** — 33 chart types
- **[3D Charts](charts/3d/scatter3d.md)** — 17 chart types, WebGL GPU renderer
- **[Machine Learning](ml/dbscan.md)** — DBSCAN up to 600× faster than scikit-learn
- **[API Reference](api/index.md)** — complete function index


Benchmarked against Plotly (figure object), Plotly (→HTML), and Matplotlib on the same machine, same dataset.

| Chart | SeraPlot | Plotly figure | Plotly →HTML | Matplotlib |
|-------|----------|--------------|-------------|------------|
| Pie | **4.2 µs** | 725 µs | 33,416 µs | 15,085 µs |
| Bar | **2.8 µs** | 658 µs | 18,166 µs | 13,596 µs |
| Grouped Bar | **5.0 µs** | 558 µs | 17,981 µs | 17,445 µs |
| Histogram | **12.4 µs** | 2,496 µs | 32,762 µs | 37,973 µs |
| Scatter | **17.0 µs** | 3,916 µs | 21,615 µs | 14,141 µs |
| Violin | **16.7 µs** | 2,616 µs | 21,347 µs | 21,211 µs |
| Box Plot | **18.4 µs** | 2,329 µs | 21,799 µs | 15,590 µs |
| KDE | **26.3 µs** | 2,981 µs | 19,807 µs | 40,108 µs |
| Radar | **11.8 µs** | 962 µs | 17,679 µs | 20,942 µs |
| Lollipop | **6.3 µs** | 8,382 µs | 25,096 µs | 9,072 µs |
| Candlestick | **8.8 µs** | 1,478 µs | 17,934 µs | N/A |
| Ridgeline | **88.8 µs** | N/A | N/A | N/A |

Average speedup vs Plotly →HTML: **~3,500×**. The worst case (Ridgeline) has no competitor — it does not exist in Plotly or Matplotlib.

### Output file size

| Chart | SeraPlot | Plotly | Ratio |
|-------|----------|--------|-------|
| Pie | 19 KB | 4,733 KB | Plotly 246× larger |
| Bar | 21 KB | 4,733 KB | Plotly 225× larger |
| Scatter | 39 KB | 4,740 KB | Plotly 121× larger |
| Violin | 21 KB | 4,737 KB | Plotly 227× larger |
| Radar | 23 KB | 4,733 KB | Plotly 205× larger |

Matplotlib outputs PNG/SVG/PDF (50–500 KB) — not self-contained HTML.

### Speedup vs Plotly →HTML

| Chart | SeraPlot | Speedup |
|-------|----------|---------|
| Pie | 4.2 µs | **7,956×** |
| Bar | 2.8 µs | **6,488×** |
| Grouped Bar | 5.0 µs | **3,596×** |
| Candlestick | 8.8 µs | **2,038×** |
| Lollipop | 6.3 µs | **3,983×** |
| Radar | 11.8 µs | **1,498×** |
| KDE | 26.3 µs | **753×** |

---

## What this means in practice

**Plotly** is mature, has large ecosystem support, and excels at interactivity (built-in zoom, click events, full JS control). Choose it when you need fine-grained widget behavior or Dash integration.

**Matplotlib** is the standard for publication figures (PDF, SVG, LaTeX). Choose it when you need print-quality static output.

**SeraPlot** is the right choice when:
- you output HTML (dashboards, emails, embedded reports, Jupyter notebooks)
- render time matters (real-time dashboards, CI pipelines, large batch exports)
- file size matters (sharing charts in emails or embedding thousands of them)
- you want zero-dependency self-contained HTML (no CDN, no internet required)
- you need chart types that do not exist in other libraries (Ridgeline, Dumbbell, Slope, Bullet, Globe 3D, Slideshow, GPU scatter 3D)

SeraPlot is not trying to replace Plotly for interactive web apps. It is a different tool for a different constraint: **maximum throughput, minimum size, maximum portability**.

---

## Navigation

- **[Installation](getting-started/installation.md)** — Install SeraPlot from PyPI
- **[Quick Start](getting-started/quickstart.md)** — Create your first chart in 3 lines
- **[2D Charts](charts/2d/bar.md)** — 33 chart types
- **[3D Charts](charts/3d/scatter3d.md)** — 17 chart types with WebGL GPU renderer
- **[Machine Learning](ml/dbscan.md)** — DBSCAN up to 600× faster than scikit-learn
- **[API Reference](api/index.md)** — Complete function index
