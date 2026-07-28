SeraPlot — High-Performance Data Visualization Framework

**SeraPlot v2.8.0+** is a Rust-native charting library exposed to Python as a single compiled extension — no JavaScript charting library, no Node toolchain, no external rendering service. One `pip install` gets you 60+ chart types across 2D, 3D, and maps, a small canvas drawing API for custom compositions, and `sp.App` — a reactive dashboard server built directly into the core, no Flask or Dash required.

Every chart renders to a single self-contained HTML file — no CDN, no backend, works offline or embedded anywhere a browser can open a file.

> **Documentation:** https://feur25.github.io/Sera/getting-started/intro-seraplot.html

---

## Installation

**Python** (PyPI — wheel for CPython 3.8+, Windows/Linux/macOS)

```bash
pip install seraplot
```

Other package managers:

```bash
conda install -c conda-forge seraplot
uv pip install seraplot
```

**JavaScript/WebAssembly** (npm)

```bash
npm install seraplot
```

**Quickstart**

```python
import seraplot as sp

chart = sp.bar("Revenue by Product", labels=["A", "B", "C"], values=[42, 68, 35])
chart.save("chart.html")
```

Open `chart.html` in any browser — no server, no build step.

---

### Gallery — Chart Types

#### 2D Charts
| | | | |
|:---:|:---:|:---:|:---:|
| ![2D-1](https://raw.githubusercontent.com/feur25/seraplot-documentation/main/2d/1.png) | ![2D-2](https://raw.githubusercontent.com/feur25/seraplot-documentation/main/2d/2.png) | ![2D-3](https://raw.githubusercontent.com/feur25/seraplot-documentation/main/2d/3.png) | ![2D-4](https://raw.githubusercontent.com/feur25/seraplot-documentation/main/2d/4.png) |
| ![2D-5](https://raw.githubusercontent.com/feur25/seraplot-documentation/main/2d/5.png) | ![2D-6](https://raw.githubusercontent.com/feur25/seraplot-documentation/main/2d/6.png) | ![2D-7](https://raw.githubusercontent.com/feur25/seraplot-documentation/main/2d/7.png) | ![2D-8](https://raw.githubusercontent.com/feur25/seraplot-documentation/main/2d/8.png) |

#### 3D Charts  
| | | | |
|:---:|:---:|:---:|:---:|
| ![3D-1](https://raw.githubusercontent.com/feur25/seraplot-documentation/main/3d/1.png) | ![3D-2](https://raw.githubusercontent.com/feur25/seraplot-documentation/main/3d/2.png) | ![3D-3](https://raw.githubusercontent.com/feur25/seraplot-documentation/main/3d/3.png) | ![3D-4](https://raw.githubusercontent.com/feur25/seraplot-documentation/main/3d/4.png) |
| ![3D-5](https://raw.githubusercontent.com/feur25/seraplot-documentation/main/3d/5.png) | ![3D-6](https://raw.githubusercontent.com/feur25/seraplot-documentation/main/3d/6.png) | ![3D-7](https://raw.githubusercontent.com/feur25/seraplot-documentation/main/3d/7.png) | ![3D-8](https://raw.githubusercontent.com/feur25/seraplot-documentation/main/3d/8.png) |

---
<p align="center">
  <a href="https://feur25.github.io/Sera/getting-started/intro-seraplot.html"><img src="https://i.ibb.co/WpF73657/logo-banner.png" alt="Seraplot Banner" border="0"></a>
</p>
