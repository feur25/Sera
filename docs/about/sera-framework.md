<style>
.sf-page{max-width:1040px;margin:0 auto 3rem}
.sf-hero{margin:1.2em 0 2em;padding:32px 0 24px;border-bottom:1px solid #243044}
.sf-kicker{margin:0 0 10px;color:#8aa0bd;font-size:12px;font-weight:700;letter-spacing:.12em;text-transform:uppercase}
.sf-title{margin:0 0 14px;border:0;color:#f1f5f9;font-size:clamp(32px,4vw,48px);font-weight:800;letter-spacing:0;line-height:1.12}
.sf-lead{max-width:760px;margin:0;color:#b6c2d2;font-size:16px;line-height:1.7}
.sf-stats{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));gap:12px;margin:28px 0 0}
.sf-stat{padding:14px 16px;border:1px solid #263346;border-radius:8px;background:#0b1220}
.sf-stat-n{display:block;color:#e8eef7;font-size:22px;font-weight:800;line-height:1.1}
.sf-stat-l{display:block;margin-top:5px;color:#8190a5;font-size:11px;font-weight:700;letter-spacing:.08em;text-transform:uppercase}
.sf-section{margin:2.4em 0 .9em;padding-bottom:8px;border-bottom:1px solid #243044;color:#d7dee9;font-size:12px;font-weight:800;letter-spacing:.12em;text-transform:uppercase}
.sf-products{display:grid;grid-template-columns:repeat(auto-fit,minmax(260px,1fr));gap:14px;margin:1em 0 2.2em}
.sf-card{padding:20px;border:1px solid #263346;border-radius:8px;background:#0b1220}
.sf-card h2{margin:0 0 8px;border:0;color:#f1f5f9;font-size:18px;letter-spacing:0}
.sf-card p{margin:0 0 14px;color:#aab8ca;font-size:13.5px;line-height:1.65}
.sf-tags{display:flex;flex-wrap:wrap;gap:6px}
.sf-tag{padding:3px 8px;border:1px solid #33445c;border-radius:6px;background:#101a2b;color:#b9c6d8;font-size:11px;font-weight:650}
.sf-bench{display:flex;flex-direction:column;gap:10px;margin:1em 0 2em}
.sf-bench-row{display:grid;grid-template-columns:minmax(150px,220px) 1fr 64px;gap:14px;align-items:center}
.sf-bench-lbl{color:#aab8ca;font-size:13px;font-weight:650}
.sf-bench-track{height:8px;border-radius:999px;background:#172235;overflow:hidden}
.sf-bench-fill{height:100%;border-radius:999px;background:#6f8fbd}
.sf-bench-val{color:#d7dee9;font-size:12px;font-weight:800;text-align:right;font-variant-numeric:tabular-nums}
.sf-arch{padding:18px;border:1px solid #263346;border-radius:8px;background:#0b1220}
.sf-arch-layers{display:flex;flex-direction:column;gap:8px}
.sf-arch-layer{display:grid;grid-template-columns:95px 1fr;gap:12px;align-items:start;padding:12px;border:1px solid #1e2a3c;border-radius:7px;background:#08101d}
.sf-arch-label{color:#c7d2e0;font-size:11px;font-weight:800;letter-spacing:.08em;text-transform:uppercase}
.sf-arch-items{display:flex;flex-wrap:wrap;gap:7px}
.sf-chip{padding:3px 9px;border:1px solid #33445c;border-radius:6px;color:#b9c6d8;background:#101a2b;font-size:11px;font-weight:650}
@media(max-width:760px){
  .sf-stats{grid-template-columns:repeat(2,minmax(0,1fr))}
  .sf-bench-row{grid-template-columns:1fr 56px;gap:8px}
  .sf-bench-track{grid-column:1 / -1;grid-row:2}
  .sf-arch-layer{grid-template-columns:1fr}
}
</style>

<div class="sf-page">

<div class="sf-hero">
<p class="sf-kicker">Sera Framework</p>
<h1 class="sf-title">A compact Rust-native stack for data work.</h1>
<p class="sf-lead">Sera brings visualization, machine learning, and DataFrame tooling into one coherent API. The framework is designed for small binaries, local-first workflows, and bindings that feel natural from Python, JavaScript, and WASM.</p>
<div class="sf-stats">
<div class="sf-stat"><span class="sf-stat-n">6 000x</span><span class="sf-stat-l">vs Plotly</span></div>
<div class="sf-stat"><span class="sf-stat-n">686x</span><span class="sf-stat-l">vs sklearn</span></div>
<div class="sf-stat"><span class="sf-stat-n">60+</span><span class="sf-stat-l">chart types</span></div>
<div class="sf-stat"><span class="sf-stat-n">0</span><span class="sf-stat-l">system deps</span></div>
</div>
</div>

<div class="sf-section">Products</div>

<div class="sf-products">
<div class="sf-card">
<h2>SeraPlot</h2>
<p>High-performance data visualization for Python, JavaScript, and WASM. Each chart can ship as a self-contained HTML file without a CDN or server runtime.</p>
<div class="sf-tags"><span class="sf-tag">60+ charts</span><span class="sf-tag">Small HTML export</span><span class="sf-tag">WASM ready</span></div>
</div>
<div class="sf-card">
<h2>SeraML</h2>
<p>A Rust machine-learning layer with a familiar fit, predict, and score workflow. It targets fast local experimentation and production-friendly exports.</p>
<div class="sf-tags"><span class="sf-tag">sklearn-style API</span><span class="sf-tag">Fast training</span><span class="sf-tag">ONNX export</span></div>
</div>
<div class="sf-card">
<h2>SeraDFrame</h2>
<p>A planned DataFrame engine inspired by Polars and pandas, focused on zero-copy data handling, memory mapping, and SQL-friendly analysis.</p>
<div class="sf-tags"><span class="sf-tag">pandas surface</span><span class="sf-tag">Zero-copy</span><span class="sf-tag">Coming soon</span></div>
</div>
</div>

<div class="sf-section">Performance</div>

<div class="sf-bench">
<div class="sf-bench-row"><div class="sf-bench-lbl">SeraPlot vs Plotly</div><div class="sf-bench-track"><div class="sf-bench-fill" style="width:99%"></div></div><div class="sf-bench-val">6 000x</div></div>
<div class="sf-bench-row"><div class="sf-bench-lbl">SeraPlot vs matplotlib</div><div class="sf-bench-track"><div class="sf-bench-fill" style="width:78%"></div></div><div class="sf-bench-val">480x</div></div>
<div class="sf-bench-row"><div class="sf-bench-lbl">SeraPlot vs Seaborn</div><div class="sf-bench-track"><div class="sf-bench-fill" style="width:68%"></div></div><div class="sf-bench-val">320x</div></div>
<div class="sf-bench-row"><div class="sf-bench-lbl">SeraML vs sklearn KMeans</div><div class="sf-bench-track"><div class="sf-bench-fill" style="width:99%"></div></div><div class="sf-bench-val">686x</div></div>
<div class="sf-bench-row"><div class="sf-bench-lbl">SeraML vs sklearn RF</div><div class="sf-bench-track"><div class="sf-bench-fill" style="width:55%"></div></div><div class="sf-bench-val">28x</div></div>
<div class="sf-bench-row"><div class="sf-bench-lbl">SeraML vs sklearn LinReg</div><div class="sf-bench-track"><div class="sf-bench-fill" style="width:32%"></div></div><div class="sf-bench-val">12x</div></div>
</div>

<div class="sf-section">Architecture</div>

<div class="sf-arch">
<div class="sf-arch-layers">
<div class="sf-arch-layer"><div class="sf-arch-label">Python</div><div class="sf-arch-items"><span class="sf-chip">seraplot</span><span class="sf-chip">seraml</span><span class="sf-chip">seradframe</span></div></div>
<div class="sf-arch-layer"><div class="sf-arch-label">JS / WASM</div><div class="sf-arch-items"><span class="sf-chip">seraplot npm</span><span class="sf-chip">seraplot-web.js</span><span class="sf-chip">wasm-bindgen</span></div></div>
<div class="sf-arch-layer"><div class="sf-arch-label">Rust core</div><div class="sf-arch-items"><span class="sf-chip">seraplot crate</span><span class="sf-chip">seraml crate</span><span class="sf-chip">seradframe crate</span><span class="sf-chip">PyO3</span></div></div>
<div class="sf-arch-layer"><div class="sf-arch-label">Backends</div><div class="sf-arch-items"><span class="sf-chip">CPU SIMD</span><span class="sf-chip">CUDA</span><span class="sf-chip">Metal</span><span class="sf-chip">Rayon</span><span class="sf-chip">ONNX</span></div></div>
</div>
</div>

</div>
