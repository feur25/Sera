<div align="center">

# SeraPlot

**High-performance data visualization library — Rust/Python**

[![PyPI](https://img.shields.io/pypi/v/seraplot)](https://pypi.org/project/seraplot/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/feur25/seraplot/blob/main/LICENSE)

</div>

SeraPlot is a **Rust-powered Python visualization library** that generates interactive HTML charts using inline JavaScript — no server required, no JavaScript dependencies, zero install friction.

**Bibliothèque de visualisation Python alimentée par Rust** qui génère des graphiques HTML interactifs avec du JavaScript embarqué — aucun serveur requis.

---

## Why SeraPlot? / Pourquoi SeraPlot ?

### Render time (Diabetes dataset, n=768, 40 runs)

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
