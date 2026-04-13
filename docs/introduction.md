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

| Feature | SeraPlot | Matplotlib | Plotly |
|---------|----------|------------|--------|
| DBSCAN speed (100k pts) | **0.05s** | — | ~30s |
| HTML output | ✅ | ❌ | ✅ |
| No JS dependencies | ✅ | — | ❌ |
| Jupyter auto-display | ✅ | ✅ | ✅ |
| 3D WebGL (GPU) | ✅ | ❌ | ✅ |

---

## Navigation

- **[Installation](getting-started/installation.md)** — Install SeraPlot from PyPI
- **[Quick Start](getting-started/quickstart.md)** — Create your first chart in 3 lines
- **[2D Charts](charts/2d/bar.md)** — 33 chart types
- **[3D Charts](charts/3d/scatter3d.md)** — 17 chart types with WebGL renderer
- **[Machine Learning](ml/dbscan.md)** — DBSCAN × 600 faster than sklearn
- **[API Reference](api/index.md)** — Complete function index
