<style>
.sf-doc{max-width:1080px;margin:0 auto 3rem;color:#c9d4e3}
.sf-board{margin:1.2em 0 1.6em;border:1px solid #2b3749;border-radius:8px;background:#0b1220;overflow:hidden}
.sf-board-head{display:grid;grid-template-columns:1fr auto;gap:20px;padding:22px 24px;border-bottom:1px solid #2b3749;background:#101827}
.sf-kicker{margin:0 0 8px;color:#93a4bb;font-size:11px;font-weight:800;letter-spacing:.13em;text-transform:uppercase}
.sf-title{margin:0;border:0;color:#f1f5f9;font-size:34px;font-weight:800;letter-spacing:0;line-height:1.16}
.sf-lead{margin:14px 0 0;max-width:760px;color:#b7c3d4;font-size:15px;line-height:1.7}
.sf-ref{align-self:start;min-width:190px;border:1px solid #334155;border-radius:6px;background:#0b1220}
.sf-ref-row{display:grid;grid-template-columns:82px 1fr;border-bottom:1px solid #263346;font-size:12px}
.sf-ref-row:last-child{border-bottom:0}
.sf-ref-k{padding:8px 10px;color:#8798ad;background:#111c2e;font-weight:750}
.sf-ref-v{padding:8px 10px;color:#d8e1ee;font-weight:650}
.sf-note{padding:14px 24px;border-bottom:1px solid #2b3749;color:#b7c3d4;font-size:14px;line-height:1.65}
.sf-note strong{color:#f1f5f9}
.sf-stats{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));border-bottom:1px solid #2b3749}
.sf-stat{padding:15px 18px;border-right:1px solid #2b3749;background:#0b1220}
.sf-stat:last-child{border-right:0}
.sf-stat-n{display:block;color:#f1f5f9;font-size:23px;font-weight:800;line-height:1.1}
.sf-stat-l{display:block;margin-top:5px;color:#8798ad;font-size:11px;font-weight:800;letter-spacing:.08em;text-transform:uppercase}
.sf-section{margin:2em 0 .85em;color:#e2e8f0;font-size:13px;font-weight:800;letter-spacing:.1em;text-transform:uppercase}
.sf-table-wrap{overflow-x:auto;border:1px solid #2b3749;border-radius:8px;background:#0b1220}
.sf-table{width:100%;border-collapse:collapse;font-size:13px}
.sf-table th{padding:10px 12px;border-bottom:1px solid #2b3749;background:#101827;color:#cbd5e1;text-align:left;font-size:11px;font-weight:800;letter-spacing:.08em;text-transform:uppercase;white-space:nowrap}
.sf-table td{padding:12px;border-bottom:1px solid #223044;color:#b7c3d4;line-height:1.55;vertical-align:top}
.sf-table tr:last-child td{border-bottom:0}
.sf-table tbody tr:hover td{background:#101827}
.sf-table strong{color:#f1f5f9}
.sf-tag-row{display:flex;flex-wrap:wrap;gap:6px}
.sf-tag{padding:3px 8px;border:1px solid #344258;border-radius:5px;background:#101a2b;color:#c5d1df;font-size:11px;font-weight:700;white-space:nowrap}
.sf-grid{display:grid;grid-template-columns:1.1fr .9fr;gap:14px}
.sf-panel{border:1px solid #2b3749;border-radius:8px;background:#0b1220;overflow:hidden}
.sf-panel h2{margin:0;padding:11px 14px;border:0;border-bottom:1px solid #2b3749;background:#101827;color:#e2e8f0;font-size:13px;font-weight:800;letter-spacing:.08em;text-transform:uppercase}
.sf-list{margin:0;padding:0;list-style:none}
.sf-list li{display:grid;grid-template-columns:120px 1fr;gap:12px;padding:12px 14px;border-bottom:1px solid #223044;font-size:13px;line-height:1.55}
.sf-list li:last-child{border-bottom:0}
.sf-list b{color:#f1f5f9}
.sf-list span{color:#b7c3d4}
.sf-flow{display:flex;flex-direction:column;gap:8px;padding:14px}
.sf-flow-row{display:grid;grid-template-columns:112px 1fr;gap:12px;padding:10px 12px;border:1px solid #223044;border-radius:6px;background:#08101d}
.sf-flow-k{color:#93a4bb;font-size:11px;font-weight:800;letter-spacing:.08em;text-transform:uppercase}
.sf-flow-v{display:flex;flex-wrap:wrap;gap:6px}
.sf-chip{padding:3px 8px;border:1px solid #344258;border-radius:5px;background:#101a2b;color:#c5d1df;font-size:11px;font-weight:700}
@media(max-width:820px){
  .sf-board-head,.sf-grid{grid-template-columns:1fr}
  .sf-stats{grid-template-columns:repeat(2,minmax(0,1fr))}
  .sf-stat:nth-child(2){border-right:0}
  .sf-stat:nth-child(-n+2){border-bottom:1px solid #2b3749}
  .sf-list li,.sf-flow-row{grid-template-columns:1fr}
}
</style>

<div class="sf-doc">

<div class="sf-board">
<div class="sf-board-head">
<div>
<p class="sf-kicker">Sera Framework</p>
<h1 class="sf-title">Rust-native data tooling, documented as a stack.</h1>
<p class="sf-lead">Sera groups plotting, machine learning, and DataFrame work behind one compact architecture. This page is a reference board: what each layer owns, where it runs, and how the pieces fit together.</p>
</div>
<div class="sf-ref">
<div class="sf-ref-row"><div class="sf-ref-k">Scope</div><div class="sf-ref-v">Data stack</div></div>
<div class="sf-ref-row"><div class="sf-ref-k">Core</div><div class="sf-ref-v">Rust</div></div>
<div class="sf-ref-row"><div class="sf-ref-k">Bindings</div><div class="sf-ref-v">Python / JS / WASM</div></div>
<div class="sf-ref-row"><div class="sf-ref-k">Runtime</div><div class="sf-ref-v">Local-first</div></div>
</div>
</div>
<div class="sf-note"><strong>Design goal:</strong> keep the public API familiar while moving heavy rendering, training, and data handling into a small native core.</div>
<div class="sf-stats">
<div class="sf-stat"><span class="sf-stat-n">6 000x</span><span class="sf-stat-l">vs Plotly</span></div>
<div class="sf-stat"><span class="sf-stat-n">686x</span><span class="sf-stat-l">vs sklearn</span></div>
<div class="sf-stat"><span class="sf-stat-n">60+</span><span class="sf-stat-l">chart types</span></div>
<div class="sf-stat"><span class="sf-stat-n">0</span><span class="sf-stat-l">system deps</span></div>
</div>
</div>

<div class="sf-section">Product Matrix</div>
<div class="sf-table-wrap">
<table class="sf-table">
<thead><tr><th>Product</th><th>Role</th><th>Main surface</th><th>Best for</th><th>Status</th></tr></thead>
<tbody>
<tr><td><strong>SeraPlot</strong></td><td>Chart rendering and export.</td><td><div class="sf-tag-row"><span class="sf-tag">Python</span><span class="sf-tag">JavaScript</span><span class="sf-tag">WASM</span></div></td><td>Self-contained interactive charts, reports, previews, and docs.</td><td><span class="sf-tag">Active</span></td></tr>
<tr><td><strong>SeraML</strong></td><td>Native ML algorithms with a familiar fit/predict workflow.</td><td><div class="sf-tag-row"><span class="sf-tag">Python</span><span class="sf-tag">Rust core</span><span class="sf-tag">ONNX</span></div></td><td>Fast local experiments, model scoring, and portable exports.</td><td><span class="sf-tag">Active</span></td></tr>
<tr><td><strong>SeraDFrame</strong></td><td>Planned DataFrame layer for tabular analysis.</td><td><div class="sf-tag-row"><span class="sf-tag">pandas-like</span><span class="sf-tag">SQL</span><span class="sf-tag">zero-copy</span></div></td><td>Memory-mapped data work and bridge code between plotting and ML.</td><td><span class="sf-tag">Planned</span></td></tr>
</tbody>
</table>
</div>

<div class="sf-section">Architecture Notes</div>
<div class="sf-grid">
<div class="sf-panel">
<h2>Layer responsibilities</h2>
<ul class="sf-list">
<li><b>Bindings</b><span>Expose a familiar API in Python and JavaScript while keeping the implementation shared.</span></li>
<li><b>Rust core</b><span>Owns rendering, algorithms, model state, export paths, and performance-sensitive loops.</span></li>
<li><b>Backends</b><span>Use CPU parallelism today and leave clear room for CUDA, Metal, ONNX, and WASM-specific paths.</span></li>
<li><b>Artifacts</b><span>Prefer portable outputs: standalone HTML, model summaries, registries, and serializable state.</span></li>
</ul>
</div>
<div class="sf-panel">
<h2>Stack map</h2>
<div class="sf-flow">
<div class="sf-flow-row"><div class="sf-flow-k">Python</div><div class="sf-flow-v"><span class="sf-chip">seraplot</span><span class="sf-chip">seraml</span><span class="sf-chip">seradframe</span></div></div>
<div class="sf-flow-row"><div class="sf-flow-k">JS / WASM</div><div class="sf-flow-v"><span class="sf-chip">npm package</span><span class="sf-chip">seraplot-web.js</span><span class="sf-chip">wasm-bindgen</span></div></div>
<div class="sf-flow-row"><div class="sf-flow-k">Rust</div><div class="sf-flow-v"><span class="sf-chip">chart core</span><span class="sf-chip">ML core</span><span class="sf-chip">PyO3</span></div></div>
<div class="sf-flow-row"><div class="sf-flow-k">Runtime</div><div class="sf-flow-v"><span class="sf-chip">CPU SIMD</span><span class="sf-chip">Rayon</span><span class="sf-chip">WASM</span><span class="sf-chip">ONNX</span></div></div>
</div>
</div>
</div>

<div class="sf-section">Performance Reference</div>
<div class="sf-table-wrap">
<table class="sf-table">
<thead><tr><th>Comparison</th><th>Reported ratio</th><th>What it signals</th></tr></thead>
<tbody>
<tr><td>SeraPlot vs Plotly</td><td><strong>6 000x</strong></td><td>Native rendering and compact export path for chart generation.</td></tr>
<tr><td>SeraPlot vs matplotlib</td><td><strong>480x</strong></td><td>Lower overhead for common chart construction paths.</td></tr>
<tr><td>SeraPlot vs Seaborn</td><td><strong>320x</strong></td><td>Less wrapper overhead and direct SVG/HTML output.</td></tr>
<tr><td>SeraML vs sklearn KMeans</td><td><strong>686x</strong></td><td>Optimized native loops for iterative workloads.</td></tr>
<tr><td>SeraML vs sklearn Random Forest</td><td><strong>28x</strong></td><td>Parallel training/scoring potential in tree workloads.</td></tr>
</tbody>
</table>
</div>

</div>
