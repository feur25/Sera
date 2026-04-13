# Installation

## Requirements / Prérequis

- Python **3.8+**
- pip 21+

SeraPlot ships as a compiled Rust extension (`.pyd` / `.so`) bundled in the wheel. There is **no compiler required** on the user side — the binary is pre-built for each platform.

---

## From PyPI / Depuis PyPI

```bash
pip install seraplot
```

```bash
pip install seraplot==2.3.47
```

```bash
pip install --upgrade seraplot
```

---

## Why the install is this simple

Most Python charting libraries have a long dependency chain. Plotly pulls in `tenacity`, `packaging`, and a 4+ MB JavaScript bundle. Bokeh requires `tornado`, `pillow`, `numpy`, and a full Node.js build step to customize. Matplotlib requires a C compiler and FreeType on some platforms.

SeraPlot has **zero required Python dependencies**. The Rust extension is entirely self-contained — the HTML output embeds its own JavaScript inline and does not load anything from a CDN. This means:

- charts work offline, in air-gapped environments, in emails, in PDF exports via browser print
- the install takes under a second on any supported platform
- there is no version conflict between your `numpy`, `pandas`, or `scipy` versions and SeraPlot
- wheels are available for Windows, Linux, and macOS on PyPI — no compilation needed

For comparison, `pip install plotly` downloads ~15 MB. `pip install seraplot` downloads ~2 MB.

---

## Jupyter / Notebook

SeraPlot displays charts **automatically** in Jupyter without any `display()` call.

SeraPlot affiche les graphiques **automatiquement** dans Jupyter sans appel `display()`.

```python
import seraplot as sp

chart = sp.build_bar_chart("Sales", labels=["Q1", "Q2", "Q3"], values=[120, 145, 98])
```

The chart renders inline. No `chart.show()`, no `plt.show()`, no `display(chart)`.

Auto-display can be disabled if you are building charts in batch without a notebook:

```python
sp.set_auto_display(False)
```

---

## Optional dependencies

For ML comparisons or loading data in notebooks:

```bash
pip install numpy scikit-learn matplotlib
```

These are only needed if you want to compare SeraPlot against scikit-learn's DBSCAN or load CSV data via pandas. They are never required by SeraPlot itself.

---

## Build from source / Compiler depuis les sources

```bash
git clone https://github.com/feur25/seraplot.git
cd seraplot
pip install maturin
maturin develop --features python
```
