<style>
.sf-doc{max-width:980px;margin:0 auto 3rem;color:#c9d4e3}
.sf-head{padding:0 0 18px;margin:1.2em 0 1.8em;border-bottom:1px solid #2b3749}
.sf-kicker{margin:0 0 8px;color:#8fa1b8;font-size:11px;font-weight:800;letter-spacing:.13em;text-transform:uppercase}
.sf-title{margin:0;border:0;color:#f1f5f9;font-size:32px;font-weight:800;letter-spacing:0;line-height:1.18}
.sf-lead{max-width:820px;margin:12px 0 0;color:#b7c3d4;font-size:15px;line-height:1.75}
.sf-meta{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));margin-top:18px;border:1px solid #2b3749;border-radius:8px;overflow:hidden;background:#0b1220}
.sf-meta div{padding:12px 14px;border-right:1px solid #2b3749;color:#e2e8f0;font-size:13px;font-weight:750}
.sf-meta div:last-child{border-right:0}.sf-meta span{display:block;margin-top:4px;color:#8798ad;font-size:11px;font-weight:800;letter-spacing:.08em;text-transform:uppercase}
.sf-section{margin:2.2em 0 .85em;color:#e2e8f0;font-size:13px;font-weight:800;letter-spacing:.1em;text-transform:uppercase}
.sf-copy{max-width:860px;color:#b7c3d4;font-size:14.5px;line-height:1.75}.sf-copy p{margin:0 0 13px}.sf-copy strong{color:#f1f5f9}
.sf-note{margin:16px 0;padding:14px 16px;border:1px solid #2b3749;border-left:4px solid #64748b;border-radius:8px;background:#0b1220;color:#c5d1df;font-size:14px;line-height:1.7}
.sf-table-wrap{overflow-x:auto;border:1px solid #2b3749;border-radius:8px;background:#0b1220;margin:12px 0 18px}
.sf-table{width:100%;border-collapse:collapse;font-size:13px}.sf-table th{padding:10px 12px;border-bottom:1px solid #2b3749;background:#101827;color:#cbd5e1;text-align:left;font-size:11px;font-weight:800;letter-spacing:.08em;text-transform:uppercase;white-space:nowrap}.sf-table td{padding:12px;border-bottom:1px solid #223044;color:#b7c3d4;line-height:1.6;vertical-align:top}.sf-table tr:last-child td{border-bottom:0}.sf-table strong{color:#f1f5f9}
.sf-tag-row{display:flex;flex-wrap:wrap;gap:6px}.sf-tag{padding:3px 8px;border:1px solid #344258;border-radius:5px;background:#101a2b;color:#c5d1df;font-size:11px;font-weight:700;white-space:nowrap}
.sf-grid{display:grid;grid-template-columns:1fr 1fr;gap:14px;margin-top:12px}.sf-panel{border:1px solid #2b3749;border-radius:8px;background:#0b1220;overflow:hidden}.sf-panel h2{margin:0;padding:11px 14px;border:0;border-bottom:1px solid #2b3749;background:#101827;color:#e2e8f0;font-size:12px;font-weight:800;letter-spacing:.08em;text-transform:uppercase}.sf-panel ul{margin:0;padding:4px 16px 10px 30px}.sf-panel li{padding:8px 0;color:#b7c3d4;font-size:13.5px;line-height:1.6}.sf-panel li::marker{color:#8798ad}
.sf-flow{display:grid;grid-template-columns:150px 1fr;gap:0;border:1px solid #2b3749;border-radius:8px;background:#0b1220;overflow:hidden}.sf-flow div{padding:11px 13px;border-bottom:1px solid #223044;color:#b7c3d4;font-size:13px;line-height:1.55}.sf-flow div:nth-child(odd){background:#101827;color:#e2e8f0;font-weight:750}.sf-flow div:nth-last-child(-n+2){border-bottom:0}
@media(max-width:820px){.sf-meta,.sf-grid,.sf-flow{grid-template-columns:1fr}.sf-meta div{border-right:0;border-bottom:1px solid #2b3749}.sf-meta div:last-child{border-bottom:0}.sf-flow div:nth-last-child(2){border-bottom:1px solid #223044}}
</style>

<div class="lang-en">
<div class="sf-doc">
<div class="sf-head">
<p class="sf-kicker">Sera Framework</p>
<h1 class="sf-title">A native core for plotting, learning, and tabular data.</h1>
<p class="sf-lead">Sera is the shared architecture behind SeraPlot, SeraML, and the planned SeraDFrame layer. The goal is simple: keep the public API familiar, while moving expensive work into a Rust core that can be reused from Python, JavaScript, and WebAssembly.</p>
<div class="sf-meta"><div>Rust<span>Core runtime</span></div><div>Python / JS<span>Bindings</span></div><div>Local first<span>Default mode</span></div><div>Portable<span>Artifacts</span></div></div>
</div>

<div class="sf-copy">
<p>Sera is not a separate product that replaces the libraries above it. It is the internal stack that lets each library share the same rendering, algorithm, serialization, and export logic. That keeps the user-facing APIs small while avoiding duplicated implementations between languages.</p>
<p>The framework is designed for documentation, notebooks, local reports, and developer tools where a chart, a model, or a data operation should be fast to produce and easy to ship as a standalone artifact.</p>
</div>

<div class="sf-section">Project Layers</div>
<div class="sf-table-wrap">
<table class="sf-table">
<thead><tr><th>Layer</th><th>Responsibility</th><th>Typical outputs</th></tr></thead>
<tbody>
<tr><td><strong>SeraPlot</strong></td><td>Chart construction, rendering, export, and interactive document previews.</td><td><div class="sf-tag-row"><span class="sf-tag">HTML</span><span class="sf-tag">SVG</span><span class="sf-tag">WASM</span><span class="sf-tag">Docs</span></div></td></tr>
<tr><td><strong>SeraML</strong></td><td>Native machine-learning algorithms with a familiar fit, predict, and export workflow.</td><td><div class="sf-tag-row"><span class="sf-tag">Model state</span><span class="sf-tag">Metrics</span><span class="sf-tag">ONNX</span></div></td></tr>
<tr><td><strong>SeraDFrame</strong></td><td>Planned tabular layer for loading, transforming, and passing data between plotting and ML.</td><td><div class="sf-tag-row"><span class="sf-tag">Tables</span><span class="sf-tag">SQL-like ops</span><span class="sf-tag">Zero-copy paths</span></div></td></tr>
</tbody>
</table>
</div>

<div class="sf-section">How The Stack Fits</div>
<div class="sf-copy">
<p>The language bindings stay close to the conventions users already know. Python code can feel like a plotting or ML library, JavaScript can focus on browser integration, and the shared Rust core remains responsible for the performance-sensitive work.</p>
</div>
<div class="sf-flow">
<div>Public API</div><div>Python functions, JavaScript modules, and future notebook or WASM entry points.</div>
<div>Bindings</div><div>Thin adapters that validate inputs, convert data, and call the shared core.</div>
<div>Rust core</div><div>Rendering, algorithms, model state, telemetry hooks, exports, and serialization.</div>
<div>Artifacts</div><div>Standalone documents, chart payloads, model summaries, and portable runtime data.</div>
</div>

<div class="sf-section">Design Notes</div>
<div class="sf-grid">
<div class="sf-panel">
<h2>What Sera optimizes for</h2>
<ul>
<li>Fast local execution without requiring a service backend.</li>
<li>Small output files that can be embedded in docs or shared as reports.</li>
<li>Predictable APIs across Python, JavaScript, Rust, and WebAssembly.</li>
<li>Clear boundaries between user data, generated artifacts, and optional telemetry.</li>
</ul>
</div>
<div class="sf-panel">
<h2>What stays explicit</h2>
<ul>
<li>Telemetry remains opt-in and documented separately.</li>
<li>Native acceleration is used where it helps, without hiding the data flow.</li>
<li>Benchmarks should be read as implementation signals, not as universal guarantees.</li>
<li>Experimental surfaces are marked before they become stable API.</li>
</ul>
</div>
</div>

<div class="sf-section">Performance Reference</div>
<div class="sf-copy">
<p>The published ratios describe benchmarked paths where the native implementation removes heavy wrapper overhead. Real-world gains depend on data size, chart type, hardware, and export target.</p>
</div>
<div class="sf-table-wrap">
<table class="sf-table">
<thead><tr><th>Comparison</th><th>Reported ratio</th><th>Interpretation</th></tr></thead>
<tbody>
<tr><td>SeraPlot vs Plotly</td><td><strong>6 000x</strong></td><td>Compact native rendering path for chart generation and export.</td></tr>
<tr><td>SeraPlot vs matplotlib</td><td><strong>480x</strong></td><td>Lower overhead for common chart construction paths.</td></tr>
<tr><td>SeraPlot vs Seaborn</td><td><strong>320x</strong></td><td>Less wrapper work before producing the final artifact.</td></tr>
<tr><td>SeraML vs sklearn KMeans</td><td><strong>686x</strong></td><td>Optimized native loops for iterative workloads.</td></tr>
<tr><td>SeraML vs sklearn Random Forest</td><td><strong>28x</strong></td><td>Parallel training and scoring potential in tree workloads.</td></tr>
</tbody>
</table>
</div>
<div class="sf-note"><strong>Documentation note:</strong> this page intentionally avoids marketing buttons. It is meant to explain the architecture first, then point readers to the dedicated product pages from the sidebar.</div>
</div>
</div>

<div class="lang-fr">
<div class="sf-doc">
<div class="sf-head">
<p class="sf-kicker">Sera Framework</p>
<h1 class="sf-title">Un coeur natif pour les graphiques, le machine learning et les donnees tabulaires.</h1>
<p class="sf-lead">Sera est l'architecture partagee derriere SeraPlot, SeraML et la future couche SeraDFrame. L'objectif est simple : garder une API familiere, tout en placant le travail couteux dans un coeur Rust reutilisable depuis Python, JavaScript et WebAssembly.</p>
<div class="sf-meta"><div>Rust<span>Coeur runtime</span></div><div>Python / JS<span>Bindings</span></div><div>Local first<span>Mode par defaut</span></div><div>Portable<span>Artifacts</span></div></div>
</div>

<div class="sf-copy">
<p>Sera n'est pas un produit separe qui remplace les bibliotheques au-dessus de lui. C'est la pile interne qui permet a chaque bibliotheque de partager le meme rendu, les memes algorithmes, la meme serialisation et les memes exports. L'API reste plus petite, et l'implementation n'est pas dupliquee entre les langages.</p>
<p>Le framework vise les documentations, notebooks, rapports locaux et outils developpeur ou un graphique, un modele ou une operation de donnees doit etre produit rapidement puis partage comme artifact autonome.</p>
</div>

<div class="sf-section">Couches Du Projet</div>
<div class="sf-table-wrap">
<table class="sf-table">
<thead><tr><th>Couche</th><th>Responsabilite</th><th>Sorties typiques</th></tr></thead>
<tbody>
<tr><td><strong>SeraPlot</strong></td><td>Construction de graphiques, rendu, export et previews interactives dans la documentation.</td><td><div class="sf-tag-row"><span class="sf-tag">HTML</span><span class="sf-tag">SVG</span><span class="sf-tag">WASM</span><span class="sf-tag">Docs</span></div></td></tr>
<tr><td><strong>SeraML</strong></td><td>Algorithmes de machine learning natifs avec un workflow familier fit, predict et export.</td><td><div class="sf-tag-row"><span class="sf-tag">Etat modele</span><span class="sf-tag">Metriques</span><span class="sf-tag">ONNX</span></div></td></tr>
<tr><td><strong>SeraDFrame</strong></td><td>Couche tabulaire prevue pour charger, transformer et faire circuler les donnees entre plotting et ML.</td><td><div class="sf-tag-row"><span class="sf-tag">Tables</span><span class="sf-tag">Ops SQL-like</span><span class="sf-tag">Zero-copy</span></div></td></tr>
</tbody>
</table>
</div>

<div class="sf-section">Organisation De La Pile</div>
<div class="sf-copy">
<p>Les bindings restent proches des conventions que les utilisateurs connaissent deja. Python peut ressembler a une bibliotheque de plotting ou de ML, JavaScript peut se concentrer sur l'integration navigateur, et le coeur Rust gere le travail sensible aux performances.</p>
</div>
<div class="sf-flow">
<div>API publique</div><div>Fonctions Python, modules JavaScript et futurs points d'entree notebook ou WASM.</div>
<div>Bindings</div><div>Adaptateurs fins qui valident les entrees, convertissent les donnees et appellent le coeur partage.</div>
<div>Coeur Rust</div><div>Rendu, algorithmes, etat modele, hooks telemetry, exports et serialisation.</div>
<div>Artifacts</div><div>Documents autonomes, payloads de graphiques, resumes de modeles et donnees runtime portables.</div>
</div>

<div class="sf-section">Notes De Conception</div>
<div class="sf-grid">
<div class="sf-panel">
<h2>Ce que Sera optimise</h2>
<ul>
<li>Execution locale rapide sans backend obligatoire.</li>
<li>Fichiers de sortie compacts, faciles a integrer dans une doc ou un rapport.</li>
<li>APIs previsibles entre Python, JavaScript, Rust et WebAssembly.</li>
<li>Frontieres claires entre donnees utilisateur, artifacts generes et telemetry optionnelle.</li>
</ul>
</div>
<div class="sf-panel">
<h2>Ce qui reste explicite</h2>
<ul>
<li>La telemetry reste opt-in et documentee sur une page separee.</li>
<li>L'acceleration native est utilisee quand elle aide, sans masquer le flux de donnees.</li>
<li>Les benchmarks doivent etre lus comme des signaux d'implementation, pas comme des garanties universelles.</li>
<li>Les surfaces experimentales sont indiquees avant de devenir une API stable.</li>
</ul>
</div>
</div>

<div class="sf-section">Reference Performance</div>
<div class="sf-copy">
<p>Les ratios publies decrivent des chemins benchmarkes ou l'implementation native retire beaucoup d'overhead. Les gains reels dependent de la taille des donnees, du type de graphique, du materiel et de la cible d'export.</p>
</div>
<div class="sf-table-wrap">
<table class="sf-table">
<thead><tr><th>Comparaison</th><th>Ratio annonce</th><th>Lecture conseillee</th></tr></thead>
<tbody>
<tr><td>SeraPlot vs Plotly</td><td><strong>6 000x</strong></td><td>Chemin de rendu natif compact pour generer et exporter les graphiques.</td></tr>
<tr><td>SeraPlot vs matplotlib</td><td><strong>480x</strong></td><td>Moins d'overhead sur les chemins courants de construction de graphiques.</td></tr>
<tr><td>SeraPlot vs Seaborn</td><td><strong>320x</strong></td><td>Moins de couches wrapper avant de produire l'artifact final.</td></tr>
<tr><td>SeraML vs sklearn KMeans</td><td><strong>686x</strong></td><td>Boucles natives optimisees pour les workloads iteratifs.</td></tr>
<tr><td>SeraML vs sklearn Random Forest</td><td><strong>28x</strong></td><td>Potentiel de parallelisation pour l'entrainement et le scoring des arbres.</td></tr>
</tbody>
</table>
</div>
<div class="sf-note"><strong>Note documentation :</strong> cette page evite volontairement les boutons marketing. Elle explique d'abord l'architecture, puis laisse la sidebar guider vers les pages produit dediees.</div>
</div>
</div>
