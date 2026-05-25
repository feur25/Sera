<style>
.sml-inst-hero{padding:24px 28px;border-radius:14px;background:linear-gradient(135deg,#050c1a,#071620,#0c1c20);border:1px solid rgba(34,211,238,.2);margin:1.4em 0 2em}
.sml-inst-hero h2{margin:0 0 6px;font-size:20px;color:#67e8f9;font-weight:800;border:none}
.sml-inst-hero p{margin:0;color:#94a3b8;font-size:13.5px;line-height:1.55}
.sml-inst-sw{border:1px solid #0d2028;border-radius:10px;overflow:hidden;margin:1.5em 0}
.sml-inst-tabs{display:flex;background:#050d10;border-bottom:1px solid #0d2028;padding:0 6px}
.sml-inst-tab{padding:11px 20px;border:none;background:none;color:#64748b;font-size:13px;font-weight:600;cursor:pointer;border-bottom:2px solid transparent;margin-bottom:-1px;transition:color .15s,border-color .15s;white-space:nowrap}
.sml-inst-tab:hover{color:#e2e8f0}
.sml-inst-tab.on{color:#22d3ee;border-bottom-color:#22d3ee}
.sml-inst-body{padding:20px 22px;background:#040d10}
.sml-inst-pane{display:none}
.sml-inst-pane.on{display:block}
.sml-inst-pane pre{margin:0 0 10px;border-radius:7px!important;background:#0a1520!important}
.sml-inst-pane pre:last-of-type{margin-bottom:0}
.sml-inst-desc{font-size:13px;color:#94a3b8;margin:0 0 10px;line-height:1.6}
.sml-note{display:flex;align-items:flex-start;gap:8px;margin-top:12px;background:#050d10;border:1px solid #0d2028;border-radius:7px;padding:10px 13px;font-size:12px;color:#64748b;line-height:1.55}
.sml-note strong{color:#67e8f9}
.sml-req-grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(200px,1fr));gap:12px;margin:1.5em 0}
.sml-req-card{background:#050d10;border:1px solid #0d2028;border-radius:10px;padding:16px 18px}
.sml-req-card strong{display:block;font-size:13px;color:#67e8f9;margin-bottom:5px}
.sml-req-card span{font-size:12px;color:#64748b;line-height:1.55}
</style>
<script>
function smlInstTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sml-inst-pane').forEach(function(e){e.classList.remove('on')});r.querySelectorAll('.sml-inst-tab').forEach(function(b){b.classList.remove('on')});document.getElementById(id).classList.add('on');btn.classList.add('on')}
</script>

<div class="sml-inst-hero">
<h2>Install SeraML</h2>
<p>One command. No C compiler, no BLAS, no system dependencies.</p>
</div>

## Requirements

<div class="sml-req-grid">
<div class="sml-req-card"><strong>Python</strong><span>3.8 or later (3.11+ recommended)</span></div>
<div class="sml-req-card"><strong>OS</strong><span>Windows, macOS, Linux (x86_64 and arm64)</span></div>
<div class="sml-req-card"><strong>GPU (optional)</strong><span>CUDA 11.8+ or Metal (Apple Silicon)</span></div>
<div class="sml-req-card"><strong>RAM</strong><span>No minimum — scales with your dataset</span></div>
</div>

## Install

<div class="sml-inst-sw" id="sml-install">
<div class="sml-inst-tabs">
<button class="sml-inst-tab on" onclick="smlInstTab('sml-install','sml-i-pip',this)">pip</button>
<button class="sml-inst-tab" onclick="smlInstTab('sml-install','sml-i-uv',this)">uv</button>
<button class="sml-inst-tab" onclick="smlInstTab('sml-install','sml-i-conda',this)">conda</button>
<button class="sml-inst-tab" onclick="smlInstTab('sml-install','sml-i-gpu',this)">GPU backend</button>
</div>
<div class="sml-inst-body">
<div id="sml-i-pip" class="sml-inst-pane on">
<p class="sml-inst-desc">Standard install — CPU backend, all estimators included.</p>

```bash
pip install seraml
```

</div>
<div id="sml-i-uv" class="sml-inst-pane">

```bash
uv pip install seraml
```

</div>
<div id="sml-i-conda" class="sml-inst-pane">

```bash
conda install -c conda-forge seraml
```

</div>
<div id="sml-i-gpu" class="sml-inst-pane">
<p class="sml-inst-desc">Install with the optional GPU backend (requires CUDA 11.8+ or Metal).</p>

```bash
pip install "seraml[gpu]"
```

<div class="sml-note"><strong>Note:</strong> The GPU backend is auto-detected at runtime. If no GPU is found, SeraML falls back to the CPU backend silently.</div>
</div>
</div>
</div>

## Verify

```python
import seraml
print(seraml.__version__)
```

## Optional: use alongside SeraPlot

SeraML integrates directly with SeraPlot for visualizing predictions:

```bash
pip install seraplot seraml
```

```python
import seraplot as sp
from seraml.linear_model import LinearRegression

model = LinearRegression()
model.fit(X_train, y_train)
preds = model.predict(X_test)

sp.scatter("Predictions vs Actual", values=list(y_test), series=[list(preds)]).show()
```

→ [Quick Start →](quickstart.html)
