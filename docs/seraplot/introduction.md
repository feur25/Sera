<div align="center">

<style>
.spi-hero{margin:1.4em 0 2.2em;padding:36px 40px;border-radius:18px;background:linear-gradient(140deg,#050c1a 0%,#0d1426 45%,#131c35 80%,#0a1020 100%);border:1px solid rgba(99,102,241,.22);position:relative;overflow:hidden;box-shadow:0 24px 60px -20px rgba(0,0,0,.8)}
.spi-hero::before{content:"";position:absolute;top:-30%;right:-8%;width:55%;height:180%;background:radial-gradient(ellipse,rgba(99,102,241,.14) 0%,transparent 65%);pointer-events:none}
.spi-hero h1{margin:0 0 10px;font-size:38px;background:linear-gradient(135deg,#a5b4fc 0%,#67e8f9 50%,#f0abfc 100%);-webkit-background-clip:text;background-clip:text;color:transparent;font-weight:900;letter-spacing:-.03em;border:none}
.spi-hero p{margin:0;color:#94a3b8;font-size:15px;line-height:1.65;max-width:72ch}
.spi-badges{display:flex;gap:8px;flex-wrap:wrap;margin-top:14px}
.spi-badge{padding:3px 10px;background:rgba(99,102,241,.12);border:1px solid rgba(165,180,252,.22);border-radius:999px;font-size:11px;font-weight:700;color:#c7d2fe;letter-spacing:.04em}

.spi-grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(260px,1fr));gap:14px;margin:2em 0}
.spi-card{padding:20px 22px;background:#070d1c;border:1px solid #1a2540;border-radius:12px;transition:border-color .15s,transform .15s}
.spi-card:hover{border-color:#3730a3;transform:translateY(-2px)}
.spi-card h3{margin:0 0 8px;font-size:13px;color:#a5b4fc;font-weight:700;letter-spacing:.06em;text-transform:uppercase;border:none}
.spi-card p{margin:0;color:#64748b;font-size:13px;line-height:1.55}

.spi-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}
.spi-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;overflow-x:auto}
.spi-tb{padding:9px 22px;border:none;background:none;color:#64748b;cursor:pointer;font-size:13px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.spi-tb:hover{color:#e2e8f0}
.spi-tb.spi-act{color:#6366f1;border-bottom-color:#6366f1}
.spi-tc{display:none}
.spi-tc.spi-on{display:block}
.spi-tc pre{margin:0;border-radius:0;overflow-x:auto;padding:14px 16px}
</style>

<script>
function spiTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.spi-tc').forEach(function(e){e.classList.remove('spi-on')});r.querySelectorAll('.spi-tb').forEach(function(b){b.classList.remove('spi-act')});document.getElementById(id).classList.add('spi-on');btn.classList.add('spi-act')}
</script>

<div class="spi-hero">
<h1>SeraPlot</h1>
<p>High-performance data visualization in Python, JavaScript, and WASM.<br>60+ chart types, zero dependencies, ships as a self-contained 21 KB HTML.</p>
<div class="spi-badges">
<span class="spi-badge">6 000× faster than Plotly</span>
<span class="spi-badge">60+ chart types</span>
<span class="spi-badge">Zero runtime deps</span>
<span class="spi-badge">Python · JS · WASM · Rust</span>
</div>
</div>

</div>

## What SeraPlot replaces

SeraPlot is a complete data toolkit. One `pip install` replaces:

| You used to need | SeraPlot replaces with |
|---|---|
| matplotlib | `sp.line()`, `sp.bar()`, `sp.scatter()`, `sp.histogram()` — drop-in |
| Plotly | Same API, 6 000× faster, no CDN, offline output |
| Seaborn | `sp.kde()`, `sp.violin()`, `sp.ridgeline()`, `sp.boxplot()` |
| Streamlit / Dash | `sp.show()` opens in browser — no server, no install |
| Chart.js / D3 | `seraplot` npm package or WASM drop-in |

## Why it's faster

SeraPlot is written in Rust. The Python bindings use PyO3 — there is no Python in the critical path. SVG and HTML are rendered in memory by the Rust engine and returned as a string.

<div class="spi-grid">
<div class="spi-card"><h3>Zero dependency output</h3><p>Every chart exports to a single self-contained HTML file — no CDN, no JavaScript imports, works in email and offline environments.</p></div>
<div class="spi-card"><h3>21 KB per chart</h3><p>200× smaller than Plotly. Suitable for CI artifacts, Notion pages, S3 buckets, and PDFs.</p></div>
<div class="spi-card"><h3>Pandas / NumPy native</h3><p>Pass a DataFrame column or a numpy array directly. No conversion needed.</p></div>
<div class="spi-card"><h3>Live in the browser</h3><p>Compiled to WebAssembly — every chart page in this documentation runs the real engine.</p></div>
</div>

## Same chart — three libraries

<div class="spi-tabs" id="sp1">
<div class="spi-tab-btns">
<button class="spi-tb spi-act" onclick="spiTab('sp1','sp1a',this)">SeraPlot — 2 lines</button>
<button class="spi-tb" onclick="spiTab('sp1','sp1b',this)">Plotly — 4 lines</button>
<button class="spi-tb" onclick="spiTab('sp1','sp1c',this)">Matplotlib — 7 lines</button>
</div>
<div id="sp1a" class="spi-tc spi-on">

```python
import seraplot as sp
sp.bar("Sales", labels=["Q1","Q2","Q3","Q4"], values=[120,145,98,162]).show()
```

</div>
<div id="sp1b" class="spi-tc">

```python
import plotly.graph_objects as go
fig = go.Figure(go.Bar(x=["Q1","Q2","Q3","Q4"], y=[120,145,98,162]))
fig.update_layout(title="Sales")
fig.show()
```

</div>
<div id="sp1c" class="spi-tc">

```python
import matplotlib.pyplot as plt
labels = ["Q1","Q2","Q3","Q4"]
values = [120,145,98,162]
fig, ax = plt.subplots()
ax.bar(labels, values)
ax.set_title("Sales")
plt.show()
```

</div>
</div>

## Supported platforms

| Platform | How |
|---|---|
| Python | `pip install seraplot` |
| JavaScript / TypeScript | `npm install seraplot` |
| Browser (WASM) | `<script src="seraplot-web.js">` |
| Rust | `seraplot = "2"` in Cargo.toml |

→ [Installation →](installation.html)  
→ [Quick Start →](quickstart.html)
