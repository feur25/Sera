<div align="center">

# SeraPlot

**Rust-native charting. 30× faster than Plotly. Zero dependencies.**

[![PyPI](https://img.shields.io/pypi/v/seraplot)](https://pypi.org/project/seraplot/)
[![npm](https://img.shields.io/npm/v/seraplot)](https://www.npmjs.com/package/seraplot)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/feur25/seraplot/blob/main/LICENSE)

</div>

```bash
pip install seraplot
```

---

## The switch

```python
import matplotlib.pyplot as plt

import seraplot.matplotlib as plt
```

- 30× faster than Plotly (measured end-to-end)
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

1,000 charts in 710 ms. Not 22 seconds. Not 73 seconds.

### Offline-first apps

No CDN. No JS framework. No internet at render time. Works air-gapped.

### APIs that return charts

Return HTML directly from a FastAPI endpoint. 21 KB in the response body, no temp files.

---

## Same chart — three libraries

```python
import seraplot as sp

sp.bar("Revenue by Product", labels, values).save("chart.html")
```

```python
import plotly.express as px

fig = px.bar(x=labels, y=values, title="Revenue by Product")
fig.update_layout(template="plotly_white")
fig.write_html("chart.html")
```

```python
import matplotlib.pyplot as plt

fig, ax = plt.subplots(figsize=(9, 5))
ax.bar(labels, values, color="#6366f1")
ax.set_title("Revenue by Product")
ax.set_ylabel("Revenue")
plt.tight_layout()
plt.savefig("chart.png")
```

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

### SeraPlot

```python
import seraplot as sp
import random

categories = ["Electronics", "Clothing", "Food", "Books", "Sports", "Toys", "Health", "Auto"]

for i in range(1000):
    random.seed(i)
    values = [random.randint(10, 100) for _ in categories]
    sp.bar(f"Report #{i+1}", categories, values).html
```

**1000 charts in 710 ms**

<iframe src="previews/bench-seraplot.html" style="width:100%;height:540px;border:1px solid #334155;border-radius:8px" loading="lazy"></iframe>

### Plotly

```python
import plotly.graph_objects as go
import random

categories = ["Electronics", "Clothing", "Food", "Books", "Sports", "Toys", "Health", "Auto"]

for i in range(1000):
    random.seed(i)
    values = [random.randint(10, 100) for _ in categories]
    fig = go.Figure(data=[go.Bar(x=categories, y=values)])
    fig.update_layout(title=f"Report #{i+1}", template="plotly_dark")
    fig.to_html(full_html=True)
```

**1000 charts in 21,688 ms — 30× slower**

<iframe src="previews/bench-plotly.html" style="width:100%;height:540px;border:1px solid #334155;border-radius:8px" loading="lazy"></iframe>

### Matplotlib

```python
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import random

categories = ["Electronics", "Clothing", "Food", "Books", "Sports", "Toys", "Health", "Auto"]

for i in range(1000):
    random.seed(i)
    values = [random.randint(10, 100) for _ in categories]
    fig, ax = plt.subplots(figsize=(9, 5))
    ax.bar(categories, values)
    ax.set_title(f"Report #{i+1}")
    fig.savefig(f"chart_{i}.png")
    plt.close()
```

**1000 charts in 72,705 ms — 102× slower**

<iframe src="previews/bench-matplotlib.html" style="width:100%;height:540px;border:1px solid #334155;border-radius:8px" loading="lazy"></iframe>

| Scale | SeraPlot | Plotly | Matplotlib |
|-------|:--------:|:------:|:----------:|
| 1,000 charts | **710 ms** | 22 s | 73 s |
| 10,000 charts | **~7 s** | ~4 min | ~12 min |
| 100,000 charts | **~71 s** | ~36 min | ~2 h |

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

```python
from fastapi import FastAPI
import seraplot as sp

app = FastAPI()

@app.get("/chart")
def revenue_chart():
    return sp.bar("Revenue", labels, values).html
```

```python
from fastapi import FastAPI
from fastapi.responses import HTMLResponse
import plotly.express as px

app = FastAPI()

@app.get("/chart", response_class=HTMLResponse)
def revenue_chart():
    fig = px.bar(x=labels, y=values, title="Revenue")
    return fig.to_html(full_html=True)
```

```python
from fastapi import FastAPI
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
    return FileResponse(path, media_type="image/png")
```

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