<style>
@keyframes mesh-float{0%{transform:translate(0,0) rotate(0deg)}25%{transform:translate(-8px,12px) rotate(.5deg)}50%{transform:translate(6px,-8px) rotate(-.3deg)}75%{transform:translate(-4px,6px) rotate(.2deg)}100%{transform:translate(0,0) rotate(0deg)}}
@keyframes slide-up{from{opacity:0;transform:translateY(28px)}to{opacity:1;transform:translateY(0)}}
@keyframes bar-grow{from{width:0}to{width:var(--w)}}

.sf-bg{position:relative;overflow:hidden;border-radius:22px;background:#030810;border:1px solid rgba(99,102,241,.2);margin:1.4em 0 3em;min-height:400px;box-shadow:0 40px 100px -30px rgba(0,0,0,.95)}
.sf-canvas{position:absolute;inset:0;pointer-events:none;overflow:hidden}
.sf-blob{position:absolute;border-radius:50%;filter:blur(80px);animation:mesh-float 12s ease-in-out infinite}
.sf-blob-1{width:500px;height:500px;top:-15%;left:-10%;background:radial-gradient(circle,rgba(99,102,241,.18) 0%,transparent 70%)}
.sf-blob-2{width:400px;height:400px;bottom:-20%;right:-8%;background:radial-gradient(circle,rgba(34,211,238,.12) 0%,transparent 70%);animation-delay:-4s}
.sf-blob-3{width:320px;height:320px;top:30%;left:40%;background:radial-gradient(circle,rgba(232,121,249,.08) 0%,transparent 70%);animation-delay:-8s}
.sf-hero-content{position:relative;z-index:2;padding:56px 52px 44px;animation:slide-up .6s cubic-bezier(.22,.78,.45,1) both}
.sf-eyebrow{font-size:11px;font-weight:800;letter-spacing:.16em;text-transform:uppercase;color:#6366f1;margin-bottom:14px;display:flex;align-items:center;gap:8px}
.sf-eyebrow::before{content:"";display:inline-block;width:28px;height:2px;background:#6366f1;border-radius:2px}
.sf-h1{font-size:clamp(38px,5vw,60px);font-weight:900;letter-spacing:-.04em;line-height:1.08;margin:0 0 18px;border:none;background:linear-gradient(120deg,#ffffff 0%,#a5b4fc 35%,#67e8f9 65%,#f0abfc 100%);-webkit-background-clip:text;background-clip:text;color:transparent}
.sf-sub{font-size:16px;color:#94a3b8;line-height:1.7;max-width:68ch;margin:0}
.sf-stats{display:flex;flex-wrap:wrap;gap:28px;margin-top:36px}
.sf-stat{display:flex;flex-direction:column}
.sf-stat-n{font-size:26px;font-weight:900;letter-spacing:-.03em;background:linear-gradient(135deg,#a5b4fc,#67e8f9);-webkit-background-clip:text;background-clip:text;color:transparent}
.sf-stat-l{font-size:11px;color:#475569;font-weight:600;letter-spacing:.06em;text-transform:uppercase;margin-top:2px}
.sf-products{display:grid;grid-template-columns:repeat(auto-fit,minmax(260px,1fr));gap:16px;margin:2.2em 0}
.sf-prod{position:relative;overflow:hidden;padding:28px 26px;border-radius:16px;background:#050b18;border:1px solid transparent;transition:border-color .2s,transform .2s,box-shadow .2s}
.sf-prod:hover{transform:translateY(-4px)}
.sf-prod-plot{border-color:rgba(99,102,241,.35)}
.sf-prod-plot:hover{border-color:rgba(99,102,241,.65);box-shadow:0 16px 48px -16px rgba(99,102,241,.3)}
.sf-prod-ml{border-color:rgba(34,211,238,.25)}
.sf-prod-ml:hover{border-color:rgba(34,211,238,.55);box-shadow:0 16px 48px -16px rgba(34,211,238,.2)}
.sf-prod-df{border-color:rgba(232,121,249,.2)}
.sf-prod-df:hover{border-color:rgba(232,121,249,.45);box-shadow:0 16px 48px -16px rgba(232,121,249,.2)}
.sf-prod-icon{width:42px;height:42px;border-radius:10px;margin-bottom:16px;display:flex;align-items:center;justify-content:center;font-size:20px}
.sf-prod-plot .sf-prod-icon{background:rgba(99,102,241,.15);border:1px solid rgba(99,102,241,.25)}
.sf-prod-ml .sf-prod-icon{background:rgba(34,211,238,.1);border:1px solid rgba(34,211,238,.2)}
.sf-prod-df .sf-prod-icon{background:rgba(232,121,249,.1);border:1px solid rgba(232,121,249,.2)}
.sf-prod-name{font-size:17px;font-weight:800;letter-spacing:-.02em;margin:0 0 7px;border:none}
.sf-prod-plot .sf-prod-name{color:#a5b4fc}
.sf-prod-ml .sf-prod-name{color:#67e8f9}
.sf-prod-df .sf-prod-name{color:#f0abfc}
.sf-prod-desc{font-size:13px;color:#475569;line-height:1.6;margin:0 0 14px}
.sf-prod-pills{display:flex;flex-wrap:wrap;gap:6px}
.sf-pill{font-size:10.5px;font-weight:700;padding:3px 8px;border-radius:999px}
.sf-prod-plot .sf-pill{background:rgba(99,102,241,.12);color:#c7d2fe;border:1px solid rgba(99,102,241,.18)}
.sf-prod-ml .sf-pill{background:rgba(34,211,238,.08);color:#a5f3fc;border:1px solid rgba(34,211,238,.15)}
.sf-prod-df .sf-pill{background:rgba(232,121,249,.08);color:#f5d0fe;border:1px solid rgba(232,121,249,.15)}
.sf-section-head{font-size:12px;font-weight:800;letter-spacing:.12em;text-transform:uppercase;color:#475569;margin:2.4em 0 1em;padding-bottom:8px;border-bottom:1px solid #0d1426}
.sf-bench{margin:1em 0 2em;display:flex;flex-direction:column;gap:8px}
.sf-bench-row{display:flex;align-items:center;gap:14px}
.sf-bench-lbl{width:200px;flex-shrink:0;font-size:12.5px;color:#94a3b8;font-weight:600}
.sf-bench-track{flex:1;height:10px;background:#0a1020;border-radius:999px;overflow:hidden}
.sf-bench-fill{height:100%;border-radius:999px;animation:bar-grow .8s cubic-bezier(.22,.78,.45,1) both}
.sf-bench-fill-a{background:linear-gradient(90deg,#6366f1,#a5b4fc)}
.sf-bench-fill-b{background:linear-gradient(90deg,#22d3ee,#67e8f9)}
.sf-bench-val{font-size:11px;font-weight:700;min-width:44px;text-align:right}
.sf-bench-row:nth-child(-n+3) .sf-bench-val{color:#a5b4fc}
.sf-bench-row:nth-child(n+4) .sf-bench-val{color:#67e8f9}
.sf-arch-box{margin:1em 0 2em;padding:28px 32px;background:#050b18;border:1px solid #0d1a2e;border-radius:16px}
.sf-arch-layers{display:flex;flex-direction:column;gap:10px}
.sf-arch-layer{display:flex;align-items:center;gap:14px;padding:14px 16px;border-radius:10px;background:#030810;border:1px solid #0d1426}
.sf-arch-layer-label{font-size:11px;font-weight:700;letter-spacing:.08em;text-transform:uppercase;width:80px;flex-shrink:0}
.sf-arch-layer-items{display:flex;flex-wrap:wrap;gap:7px}
.sf-arch-chip{font-size:11px;font-weight:600;padding:3px 10px;border-radius:6px;background:#0a1520}
.sf-arch-l1 .sf-arch-layer-label{color:#a5b4fc}
.sf-arch-l1 .sf-arch-chip{color:#a5b4fc;border:1px solid rgba(99,102,241,.22)}
.sf-arch-l2 .sf-arch-layer-label{color:#67e8f9}
.sf-arch-l2 .sf-arch-chip{color:#67e8f9;border:1px solid rgba(34,211,238,.18)}
.sf-arch-l3 .sf-arch-layer-label{color:#f0abfc}
.sf-arch-l3 .sf-arch-chip{color:#f0abfc;border:1px solid rgba(232,121,249,.18)}
.sf-arch-l4 .sf-arch-layer-label{color:#64748b}
.sf-arch-l4 .sf-arch-chip{color:#64748b;border:1px solid #1a2540}
.sf-divider{height:1px;background:linear-gradient(90deg,transparent,rgba(99,102,241,.25),rgba(34,211,238,.15),transparent);margin:2.4em 0}
.sf-cta-row{display:flex;gap:12px;flex-wrap:wrap;margin:2em 0}
.sf-cta{display:inline-flex;align-items:center;gap:7px;padding:12px 24px;border-radius:10px;font-size:13.5px;font-weight:700;text-decoration:none!important;transition:transform .15s,box-shadow .15s}
.sf-cta:hover{transform:translateY(-2px)}
.sf-cta-primary{background:linear-gradient(135deg,#6366f1,#4f46e5);color:#fff!important;box-shadow:0 8px 24px -8px rgba(99,102,241,.5)}
.sf-cta-primary:hover{box-shadow:0 12px 32px -8px rgba(99,102,241,.6)}
.sf-cta-secondary{background:#0a1020;border:1px solid #1a2540;color:#94a3b8!important}
.sf-cta-secondary:hover{border-color:#6366f1;color:#a5b4fc!important}
</style>

<div class="sf-bg">
<div class="sf-canvas">
<div class="sf-blob sf-blob-1"></div>
<div class="sf-blob sf-blob-2"></div>
<div class="sf-blob sf-blob-3"></div>
<svg viewBox="0 0 900 400" preserveAspectRatio="xMidYMid slice" xmlns="http://www.w3.org/2000/svg" opacity=".07"><defs><pattern id="gr" width="40" height="40" patternUnits="userSpaceOnUse"><path d="M 40 0 L 0 0 0 40" fill="none" stroke="#6366f1" stroke-width=".5"/></pattern></defs><rect width="100%" height="100%" fill="url(#gr)"/></svg>
</div>
<div class="sf-hero-content">
<div class="sf-eyebrow">Sera Framework</div>
<h1 class="sf-h1">One framework.<br>Every data need.</h1>
<p class="sf-sub">Sera is a Rust-native data toolkit that replaces entire Python ecosystems — visualization, machine learning, and DataFrames — in a single coherent API that ships as a 5 MB binary.</p>
<div class="sf-stats">
<div class="sf-stat"><span class="sf-stat-n">6 000×</span><span class="sf-stat-l">vs Plotly</span></div>
<div class="sf-stat"><span class="sf-stat-n">686×</span><span class="sf-stat-l">vs sklearn</span></div>
<div class="sf-stat"><span class="sf-stat-n">60+</span><span class="sf-stat-l">chart types</span></div>
<div class="sf-stat"><span class="sf-stat-n">0</span><span class="sf-stat-l">system deps</span></div>
</div>
</div>
</div>

<div class="sf-section-head">The products</div>

<div class="sf-products">
<div class="sf-prod sf-prod-plot">
<div class="sf-prod-icon">📊</div>
<div class="sf-prod-name">SeraPlot</div>
<p class="sf-prod-desc">High-performance data visualization for Python, JavaScript, and WASM. Ships a complete chart as a self-contained 21 KB HTML — no CDN, no server.</p>
<div class="sf-prod-pills"><span class="sf-pill">60+ charts</span><span class="sf-pill">6 000× Plotly</span><span class="sf-pill">Zero deps</span><span class="sf-pill">WASM ready</span></div>
</div>
<div class="sf-prod sf-prod-ml">
<div class="sf-prod-icon">🧠</div>
<div class="sf-prod-name">SeraML</div>
<p class="sf-prod-desc">Drop-in replacement for scikit-learn written in Rust. Same fit/predict/score API. GPU and distributed backends. Model registry with ONNX export.</p>
<div class="sf-prod-pills"><span class="sf-pill">sklearn API</span><span class="sf-pill">686× faster</span><span class="sf-pill">GPU backend</span><span class="sf-pill">ONNX export</span></div>
</div>
<div class="sf-prod sf-prod-df">
<div class="sf-prod-icon">🗂️</div>
<div class="sf-prod-name">SeraDFrame</div>
<p class="sf-prod-desc">A Polars-inspired DataFrame engine with a pandas-compatible surface. Zero-copy, memory-mapped, with SQL query support built in.</p>
<div class="sf-prod-pills"><span class="sf-pill">pandas API</span><span class="sf-pill">Zero-copy</span><span class="sf-pill">SQL queries</span><span class="sf-pill">Coming soon</span></div>
</div>
</div>

<div class="sf-divider"></div>
<div class="sf-section-head">Performance</div>

<div class="sf-bench">
<div class="sf-bench-row"><div class="sf-bench-lbl">SeraPlot vs Plotly</div><div class="sf-bench-track"><div class="sf-bench-fill sf-bench-fill-a" style="width:99%"></div></div><div class="sf-bench-val">6 000×</div></div>
<div class="sf-bench-row"><div class="sf-bench-lbl">SeraPlot vs matplotlib</div><div class="sf-bench-track"><div class="sf-bench-fill sf-bench-fill-a" style="width:78%"></div></div><div class="sf-bench-val">480×</div></div>
<div class="sf-bench-row"><div class="sf-bench-lbl">SeraPlot vs Seaborn</div><div class="sf-bench-track"><div class="sf-bench-fill sf-bench-fill-a" style="width:68%"></div></div><div class="sf-bench-val">320×</div></div>
<div class="sf-bench-row"><div class="sf-bench-lbl">SeraML vs sklearn KMeans</div><div class="sf-bench-track"><div class="sf-bench-fill sf-bench-fill-b" style="width:99%"></div></div><div class="sf-bench-val">686×</div></div>
<div class="sf-bench-row"><div class="sf-bench-lbl">SeraML vs sklearn RF</div><div class="sf-bench-track"><div class="sf-bench-fill sf-bench-fill-b" style="width:55%"></div></div><div class="sf-bench-val">28×</div></div>
<div class="sf-bench-row"><div class="sf-bench-lbl">SeraML vs sklearn LinReg</div><div class="sf-bench-track"><div class="sf-bench-fill sf-bench-fill-b" style="width:32%"></div></div><div class="sf-bench-val">12×</div></div>
</div>

<div class="sf-divider"></div>
<div class="sf-section-head">Architecture</div>

<div class="sf-arch-box">
<div class="sf-arch-layers">
<div class="sf-arch-layer sf-arch-l1"><div class="sf-arch-layer-label">Python</div><div class="sf-arch-layer-items"><span class="sf-arch-chip">seraplot</span><span class="sf-arch-chip">seraml</span><span class="sf-arch-chip">seradframe</span></div></div>
<div class="sf-arch-layer sf-arch-l2"><div class="sf-arch-layer-label">JS / WASM</div><div class="sf-arch-layer-items"><span class="sf-arch-chip">seraplot npm</span><span class="sf-arch-chip">seraplot-web.js</span><span class="sf-arch-chip">wasm-bindgen</span></div></div>
<div class="sf-arch-layer sf-arch-l3"><div class="sf-arch-layer-label">Rust core</div><div class="sf-arch-layer-items"><span class="sf-arch-chip">seraplot crate</span><span class="sf-arch-chip">seraml crate</span><span class="sf-arch-chip">seradframe crate</span><span class="sf-arch-chip">PyO3</span></div></div>
<div class="sf-arch-layer sf-arch-l4"><div class="sf-arch-layer-label">Backends</div><div class="sf-arch-layer-items"><span class="sf-arch-chip">CPU (SIMD)</span><span class="sf-arch-chip">CUDA</span><span class="sf-arch-chip">Metal</span><span class="sf-arch-chip">Rayon</span><span class="sf-arch-chip">ONNX</span></div></div>
</div>
</div>

<div class="sf-divider"></div>

<div class="sf-cta-row">
<a href="../seraplot/introduction.html" class="sf-cta sf-cta-primary">Get started with SeraPlot →</a>
<a href="../seraml/introduction.html" class="sf-cta sf-cta-secondary">Explore SeraML →</a>
<a href="../about/support.html" class="sf-cta sf-cta-secondary">Support & Contact →</a>
</div>
