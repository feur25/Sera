<style>
.sera-hero{position:relative;padding:56px 44px 52px;border-radius:20px;background:#040810;border:1px solid rgba(99,102,241,.18);overflow:hidden;margin:1.2em 0 3em}
.sera-hero-glow{position:absolute;inset:0;background:radial-gradient(ellipse 80% 60% at 60% 20%,rgba(99,102,241,.13) 0%,transparent 65%),radial-gradient(ellipse 50% 80% at 20% 80%,rgba(34,211,238,.07) 0%,transparent 60%),radial-gradient(ellipse 40% 40% at 80% 70%,rgba(240,171,252,.06) 0%,transparent 55%);pointer-events:none}
.sera-hero-grid{position:absolute;inset:0;background-image:linear-gradient(rgba(99,102,241,.04) 1px,transparent 1px),linear-gradient(90deg,rgba(99,102,241,.04) 1px,transparent 1px);background-size:44px 44px;pointer-events:none;mask-image:radial-gradient(ellipse 100% 100% at 50% 50%,black 30%,transparent 100%)}
.sera-hero-inner{position:relative;z-index:1}
.sera-hero-logo{font-size:13px;font-weight:800;letter-spacing:.22em;color:#475569;text-transform:uppercase;margin-bottom:18px}
.sera-hero h1{margin:0 0 16px;font-size:46px;font-weight:900;letter-spacing:-.03em;line-height:1.05;border:none}
.sera-hero h1 .sera-s{background:linear-gradient(135deg,#a5b4fc 0%,#67e8f9 35%,#f0abfc 65%,#fb923c 100%);-webkit-background-clip:text;background-clip:text;color:transparent}
.sera-hero-sub{margin:0 0 32px;font-size:16.5px;color:#94a3b8;line-height:1.65;max-width:68ch}
.sera-products{display:grid;grid-template-columns:repeat(3,1fr);gap:14px;margin-bottom:32px}
@media(max-width:700px){.sera-products{grid-template-columns:1fr}}
.sera-prod{padding:20px 22px;border-radius:14px;border:1px solid transparent;transition:transform .2s,border-color .2s;cursor:default}
.sera-prod:hover{transform:translateY(-3px)}
.sera-prod-plot{background:linear-gradient(145deg,rgba(99,102,241,.12),rgba(99,102,241,.03));border-color:rgba(99,102,241,.22)}
.sera-prod-plot:hover{border-color:rgba(99,102,241,.5)}
.sera-prod-ml{background:linear-gradient(145deg,rgba(34,211,238,.1),rgba(34,211,238,.02));border-color:rgba(34,211,238,.2)}
.sera-prod-ml:hover{border-color:rgba(34,211,238,.45)}
.sera-prod-df{background:linear-gradient(145deg,rgba(240,171,252,.08),rgba(240,171,252,.02));border-color:rgba(240,171,252,.15);opacity:.6}
.sera-prod-icon{font-size:22px;margin-bottom:8px}
.sera-prod-name{font-size:15px;font-weight:800;margin:0 0 6px}
.sera-prod-plot .sera-prod-name{color:#a5b4fc}
.sera-prod-ml .sera-prod-name{color:#67e8f9}
.sera-prod-df .sera-prod-name{color:#e879f9}
.sera-prod-desc{font-size:12.5px;color:#64748b;line-height:1.55;margin:0}
.sera-prod-badge{display:inline-block;margin-top:10px;padding:2px 8px;border-radius:999px;font-size:10px;font-weight:700;letter-spacing:.07em;text-transform:uppercase}
.sera-badge-live{background:rgba(34,197,94,.12);color:#4ade80;border:1px solid rgba(34,197,94,.25)}
.sera-badge-soon{background:rgba(251,146,60,.1);color:#fb923c;border:1px solid rgba(251,146,60,.2)}
.sera-stats{display:flex;gap:28px;flex-wrap:wrap}
.sera-stat-num{font-size:22px;font-weight:900;letter-spacing:-.02em;background:linear-gradient(135deg,#a5b4fc,#67e8f9);-webkit-background-clip:text;background-clip:text;color:transparent;line-height:1}
.sera-stat-lbl{font-size:11px;color:#475569;font-weight:600;letter-spacing:.08em;text-transform:uppercase;margin-top:2px}
.sera-nav-cards{display:grid;grid-template-columns:repeat(auto-fit,minmax(220px,1fr));gap:14px;margin:2em 0}
.sera-nav-card{padding:22px 24px;background:#070d1c;border:1px solid #1a2540;border-radius:14px;transition:border-color .15s,transform .15s;text-decoration:none!important;display:block}
.sera-nav-card:hover{border-color:#3730a3;transform:translateY(-2px)}
.sera-nav-card h3{margin:0 0 6px;font-size:14px;font-weight:700;color:#a5b4fc;border:none}
.sera-nav-card p{margin:0;font-size:12.5px;color:#64748b;line-height:1.5}
.sera-nav-card .snc-tag{font-size:10px;font-weight:700;letter-spacing:.08em;text-transform:uppercase;padding:2px 7px;border-radius:999px;margin-bottom:10px;display:inline-block}
.snc-tag-plot{background:rgba(99,102,241,.14);color:#a5b4fc;border:1px solid rgba(99,102,241,.25)}
.snc-tag-ml{background:rgba(34,211,238,.1);color:#67e8f9;border:1px solid rgba(34,211,238,.2)}
.sera-section-title{font-size:11px;font-weight:700;letter-spacing:.18em;text-transform:uppercase;color:#475569;margin:2.4em 0 1em;padding-bottom:8px;border-bottom:1px solid #1a2540}
</style>

<div class="sera-hero">
<div class="sera-hero-glow"></div>
<div class="sera-hero-grid"></div>
<div class="sera-hero-inner">
<div class="sera-hero-logo">Sera Framework · Documentation</div>
<h1><span class="sera-s">Sera</span> — The Framework</h1>
<p class="sera-hero-sub">One Rust core. Three tools. Zero dependencies.<br>Visualize data, train models, and ship anywhere — at the speed of native code.</p>
<div class="sera-products">
<div class="sera-prod sera-prod-plot">
<div class="sera-prod-icon">📊</div>
<div class="sera-prod-name">SeraPlot</div>
<p class="sera-prod-desc">60+ chart types · WASM · Python · JS · 6 000× faster than Plotly</p>
<span class="sera-prod-badge sera-badge-live">Live · PyPI</span>
</div>
<div class="sera-prod sera-prod-ml">
<div class="sera-prod-icon">🧠</div>
<div class="sera-prod-name">SeraML</div>
<p class="sera-prod-desc">Drop-in sklearn replacement · GPU · Distributed · 2–686× faster</p>
<span class="sera-prod-badge sera-badge-live">Live · PyPI</span>
</div>
<div class="sera-prod sera-prod-df">
<div class="sera-prod-icon">📋</div>
<div class="sera-prod-name">SeraDFrame</div>
<p class="sera-prod-desc">Columnar data engine · lazy eval · zero-copy SeraPlot interop</p>
<span class="sera-prod-badge sera-badge-soon">Q4 2027</span>
</div>
</div>
<div class="sera-stats">
<div class="sera-stat"><div class="sera-stat-num">6 000×</div><div class="sera-stat-lbl">faster than Plotly</div></div>
<div class="sera-stat"><div class="sera-stat-num">686×</div><div class="sera-stat-lbl">faster sklearn ops</div></div>
<div class="sera-stat"><div class="sera-stat-num">60+</div><div class="sera-stat-lbl">chart types</div></div>
<div class="sera-stat"><div class="sera-stat-num">0</div><div class="sera-stat-lbl">runtime dependencies</div></div>
</div>
</div>
</div>

<div class="sera-section-title">SeraPlot — Visualization</div>
<div class="sera-nav-cards">
<a class="sera-nav-card" href="seraplot/introduction.html"><span class="snc-tag snc-tag-plot">SeraPlot</span><h3>Introduction</h3><p>What SeraPlot is, what it replaces, and why it is faster.</p></a>
<a class="sera-nav-card" href="seraplot/installation.html"><span class="snc-tag snc-tag-plot">SeraPlot</span><h3>Installation</h3><p>pip, npm, WASM, Rust crate — all install paths.</p></a>
<a class="sera-nav-card" href="seraplot/quickstart.html"><span class="snc-tag snc-tag-plot">SeraPlot</span><h3>Quick Start</h3><p>Your first chart in 2 lines. Live in-browser playground.</p></a>
<a class="sera-nav-card" href="charts/2d/bar.html"><span class="snc-tag snc-tag-plot">SeraPlot</span><h3>Chart Gallery</h3><p>Browse all 60+ chart types with live previews and code.</p></a>
</div>

<div class="sera-section-title">SeraML — Machine Learning</div>
<div class="sera-nav-cards">
<a class="sera-nav-card" href="seraml/introduction.html"><span class="snc-tag snc-tag-ml">SeraML</span><h3>Introduction</h3><p>Scikit-learn API, Rust engine, GPU-ready from day one.</p></a>
<a class="sera-nav-card" href="seraml/installation.html"><span class="snc-tag snc-tag-ml">SeraML</span><h3>Installation</h3><p>pip install and optional GPU backend setup.</p></a>
<a class="sera-nav-card" href="seraml/quickstart.html"><span class="snc-tag snc-tag-ml">SeraML</span><h3>Quick Start</h3><p>Train your first model and visualize predictions in minutes.</p></a>
<a class="sera-nav-card" href="ml/index.html"><span class="snc-tag snc-tag-ml">SeraML</span><h3>ML Reference</h3><p>All estimators, metrics, pipelines, and GPU backends.</p></a>
</div>