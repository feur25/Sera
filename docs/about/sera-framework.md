# Sera Framework

<style>
.prod-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: 12px; margin: 1.2em 0 2em; }
.prod-card { border: 1px solid rgba(100,116,139,.25); border-radius: 8px; padding: 18px 20px; }
.prod-card h3 { margin: 0 0 6px; font-size: 15px; font-weight: 700; border: none; }
.prod-card p { margin: 0 0 10px; font-size: .9em; line-height: 1.6; color: inherit; opacity: .8; }
.prod-tags { display: flex; flex-wrap: wrap; gap: 5px; }
.prod-tag { font-size: 10px; font-weight: 700; padding: 2px 7px; border-radius: 4px; border: 1px solid rgba(100,116,139,.25); opacity: .85; }
.soon { font-size: 10px; font-weight: 700; padding: 1px 6px; border-radius: 4px; border: 1px solid rgba(100,116,139,.3); opacity: .6; margin-left: 6px; vertical-align: middle; }

.algo-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(190px, 1fr)); gap: 10px; margin: 1em 0 2em; }
.algo-group { border: 1px solid rgba(100,116,139,.2); border-radius: 6px; padding: 14px 16px; }
.algo-group h4 { margin: 0 0 8px; font-size: 11px; font-weight: 800; letter-spacing: .07em; text-transform: uppercase; opacity: .6; border: none; }
.algo-group ul { margin: 0; padding: 0; list-style: none; }
.algo-group li { font-size: .88em; line-height: 1.7; opacity: .85; }
.algo-group li::before { content: "–"; margin-right: .4em; opacity: .4; }

.arch-table { width: 100%; border-collapse: collapse; font-size: .9em; margin: 1em 0 2em; }
.arch-table td { padding: 10px 14px; border-bottom: 1px solid rgba(100,116,139,.12); vertical-align: top; }
.arch-table tr:last-child td { border-bottom: none; }
.arch-table td:first-child { font-size: 11px; font-weight: 800; letter-spacing: .07em; text-transform: uppercase; opacity: .5; white-space: nowrap; width: 90px; }
.arch-chips { display: flex; flex-wrap: wrap; gap: 5px; }
.arch-chip { font-size: 11.5px; padding: 2px 8px; border-radius: 4px; border: 1px solid rgba(100,116,139,.22); font-weight: 600; }

.bench-table { width: 100%; border-collapse: collapse; font-size: .9em; margin: 1em 0 2em; }
.bench-table th { font-size: 11px; font-weight: 800; letter-spacing: .06em; text-transform: uppercase; opacity: .5; padding: 6px 10px; border-bottom: 1px solid rgba(100,116,139,.2); text-align: left; }
.bench-table td { padding: 8px 10px; border-bottom: 1px solid rgba(100,116,139,.08); font-size: .9em; }
.bench-table tr:last-child td { border-bottom: none; }
.bench-table td:last-child { font-weight: 700; text-align: right; }

.note { border-left: 3px solid rgba(100,116,139,.4); padding: .5em 0 .5em 1em; margin: 1.2em 0; font-size: .93em; line-height: 1.65; }
</style>

Sera is a Rust-native data toolkit covering visualization, machine learning, and (soon) DataFrames. It compiles to a single binary with no system dependencies and exposes a unified API to Python via PyO3 and to the browser via WebAssembly.

---

## Products

<div class="prod-grid">

<div class="prod-card">
<h3>SeraPlot</h3>
<p>High-performance chart library for Python, JavaScript, and WASM. Each chart renders to a self-contained HTML file — no CDN, no server, no runtime dependencies.</p>
<div class="prod-tags">
<span class="prod-tag">60+ chart types</span>
<span class="prod-tag">Python · JS · WASM</span>
<span class="prod-tag">Zero runtime deps</span>
<span class="prod-tag">~21 KB HTML output</span>
</div>
</div>

<div class="prod-card">
<h3>SeraML</h3>
<p>Machine learning library written in Rust with a scikit-learn-compatible API. Covers supervised learning, unsupervised learning, preprocessing, model selection, and ONNX export.</p>
<div class="prod-tags">
<span class="prod-tag">sklearn-compatible</span>
<span class="prod-tag">ONNX export</span>
<span class="prod-tag">GPU backend</span>
<span class="prod-tag">Rayon parallelism</span>
</div>
</div>

<div class="prod-card">
<h3>SeraDFrame <span class="soon">coming soon</span></h3>
<p>A Rust DataFrame engine with a pandas-compatible surface. Zero-copy, memory-mapped, with built-in SQL query support.</p>
<div class="prod-tags">
<span class="prod-tag">pandas-compatible</span>
<span class="prod-tag">Zero-copy</span>
<span class="prod-tag">SQL queries</span>
</div>
</div>

</div>

---

## SeraML — Algorithms

<div class="algo-grid">

<div class="algo-group">
<h4>Linear Models</h4>
<ul>
<li>OLS (Linear Regression)</li>
<li>Ridge</li>
<li>Lasso</li>
<li>ElasticNet</li>
<li>Logistic Regression</li>
<li>SGD Classifier</li>
</ul>
</div>

<div class="algo-group">
<h4>Trees & Ensembles</h4>
<ul>
<li>Decision Tree</li>
<li>Random Forest</li>
<li>AdaBoost</li>
<li>Gradient Boosting</li>
</ul>
</div>

<div class="algo-group">
<h4>Neighbors & SVM</h4>
<ul>
<li>K-Nearest Neighbors</li>
<li>Support Vector Machine</li>
</ul>
</div>

<div class="algo-group">
<h4>Naive Bayes</h4>
<ul>
<li>GaussianNB</li>
<li>BernoulliNB</li>
<li>MultinomialNB</li>
</ul>
</div>

<div class="algo-group">
<h4>Unsupervised</h4>
<ul>
<li>PCA</li>
<li>Isolation Forest</li>
</ul>
</div>

<div class="algo-group">
<h4>Preprocessing</h4>
<ul>
<li>Standard / MinMax / Robust scalers</li>
<li>Label / OneHot encoders</li>
<li>Feature transformers</li>
</ul>
</div>

<div class="algo-group">
<h4>Model Selection</h4>
<ul>
<li>Train / test split</li>
<li>Cross-validation</li>
<li>Grid search</li>
<li>Permutation test</li>
</ul>
</div>

<div class="algo-group">
<h4>Export & Registry</h4>
<ul>
<li>ONNX model export</li>
<li>Power BI integration</li>
<li>Tableau integration</li>
<li>Model registry</li>
</ul>
</div>

</div>

---

## Architecture

<table class="arch-table">
<tbody>
<tr>
<td>Python</td>
<td><div class="arch-chips"><span class="arch-chip">seraplot</span><span class="arch-chip">seraml</span><span class="arch-chip">seradframe</span><span class="arch-chip">PyO3</span></div></td>
</tr>
<tr>
<td>JS / WASM</td>
<td><div class="arch-chips"><span class="arch-chip">seraplot npm</span><span class="arch-chip">seraplot-web.js</span><span class="arch-chip">wasm-bindgen</span></div></td>
</tr>
<tr>
<td>Rust core</td>
<td><div class="arch-chips"><span class="arch-chip">core</span><span class="arch-chip">plot</span><span class="arch-chip">ml</span><span class="arch-chip">data</span><span class="arch-chip">html</span><span class="arch-chip">bindings</span><span class="arch-chip">telemetry</span><span class="arch-chip">viewer</span></div></td>
</tr>
<tr>
<td>Backends</td>
<td><div class="arch-chips"><span class="arch-chip">CPU (SIMD)</span><span class="arch-chip">CUDA</span><span class="arch-chip">Metal</span><span class="arch-chip">Rayon</span><span class="arch-chip">ONNX</span></div></td>
</tr>
</tbody>
</table>

---

## Performance

<table class="bench-table">
<thead>
<tr><th>Benchmark</th><th>Library</th><th>Speedup</th></tr>
</thead>
<tbody>
<tr><td>SeraPlot</td><td>Plotly</td><td>6 000×</td></tr>
<tr><td>SeraPlot</td><td>matplotlib</td><td>480×</td></tr>
<tr><td>SeraPlot</td><td>Seaborn</td><td>320×</td></tr>
<tr><td>SeraML KMeans</td><td>sklearn KMeans</td><td>686×</td></tr>
<tr><td>SeraML RF</td><td>sklearn RF</td><td>28×</td></tr>
<tr><td>SeraML LinReg</td><td>sklearn LinReg</td><td>12×</td></tr>
</tbody>
</table>

<div class="note">Benchmarks measured on a single machine. Results vary by dataset size, hardware, and configuration.</div>

---

---

# Framework Sera

Sera est un toolkit de données Rust-natif couvrant la visualisation, le machine learning et (bientôt) les DataFrames. Il compile en un binaire unique sans dépendances système et expose une API unifiée à Python via PyO3 et au navigateur via WebAssembly.

---

## Produits

<div class="prod-grid">

<div class="prod-card">
<h3>SeraPlot</h3>
<p>Bibliothèque de graphiques haute performance pour Python, JavaScript et WASM. Chaque graphique génère un fichier HTML autonome — sans CDN, sans serveur, sans dépendances runtime.</p>
<div class="prod-tags">
<span class="prod-tag">60+ graphiques</span>
<span class="prod-tag">Python · JS · WASM</span>
<span class="prod-tag">Zéro dépendance runtime</span>
<span class="prod-tag">HTML ~21 Ko</span>
</div>
</div>

<div class="prod-card">
<h3>SeraML</h3>
<p>Bibliothèque de machine learning écrite en Rust avec une API compatible scikit-learn. Couvre l'apprentissage supervisé, non-supervisé, le préprocessing, la sélection de modèles et l'export ONNX.</p>
<div class="prod-tags">
<span class="prod-tag">sklearn-compatible</span>
<span class="prod-tag">Export ONNX</span>
<span class="prod-tag">Backend GPU</span>
<span class="prod-tag">Parallélisme Rayon</span>
</div>
</div>

<div class="prod-card">
<h3>SeraDFrame <span class="soon">bientôt</span></h3>
<p>Moteur DataFrame Rust avec une surface pandas-compatible. Zéro copie, memory-mapped, avec support SQL intégré.</p>
<div class="prod-tags">
<span class="prod-tag">pandas-compatible</span>
<span class="prod-tag">Zéro copie</span>
<span class="prod-tag">Requêtes SQL</span>
</div>
</div>

</div>

---

## SeraML — Algorithmes

<div class="algo-grid">

<div class="algo-group">
<h4>Modèles linéaires</h4>
<ul>
<li>OLS (Régression linéaire)</li>
<li>Ridge</li>
<li>Lasso</li>
<li>ElasticNet</li>
<li>Régression logistique</li>
<li>SGD Classifier</li>
</ul>
</div>

<div class="algo-group">
<h4>Arbres & Ensembles</h4>
<ul>
<li>Arbre de décision</li>
<li>Random Forest</li>
<li>AdaBoost</li>
<li>Gradient Boosting</li>
</ul>
</div>

<div class="algo-group">
<h4>Voisins & SVM</h4>
<ul>
<li>K plus proches voisins</li>
<li>Support Vector Machine</li>
</ul>
</div>

<div class="algo-group">
<h4>Naive Bayes</h4>
<ul>
<li>GaussianNB</li>
<li>BernoulliNB</li>
<li>MultinomialNB</li>
</ul>
</div>

<div class="algo-group">
<h4>Non supervisé</h4>
<ul>
<li>PCA</li>
<li>Isolation Forest</li>
</ul>
</div>

<div class="algo-group">
<h4>Préprocessing</h4>
<ul>
<li>Scalers Standard / MinMax / Robust</li>
<li>Encodeurs Label / OneHot</li>
<li>Transformateurs de features</li>
</ul>
</div>

<div class="algo-group">
<h4>Sélection de modèles</h4>
<ul>
<li>Découpage train / test</li>
<li>Validation croisée</li>
<li>Grid search</li>
<li>Test de permutation</li>
</ul>
</div>

<div class="algo-group">
<h4>Export & Registre</h4>
<ul>
<li>Export ONNX</li>
<li>Intégration Power BI</li>
<li>Intégration Tableau</li>
<li>Registre de modèles</li>
</ul>
</div>

</div>

---

## Architecture

<table class="arch-table">
<tbody>
<tr>
<td>Python</td>
<td><div class="arch-chips"><span class="arch-chip">seraplot</span><span class="arch-chip">seraml</span><span class="arch-chip">seradframe</span><span class="arch-chip">PyO3</span></div></td>
</tr>
<tr>
<td>JS / WASM</td>
<td><div class="arch-chips"><span class="arch-chip">seraplot npm</span><span class="arch-chip">seraplot-web.js</span><span class="arch-chip">wasm-bindgen</span></div></td>
</tr>
<tr>
<td>Rust core</td>
<td><div class="arch-chips"><span class="arch-chip">core</span><span class="arch-chip">plot</span><span class="arch-chip">ml</span><span class="arch-chip">data</span><span class="arch-chip">html</span><span class="arch-chip">bindings</span><span class="arch-chip">telemetry</span><span class="arch-chip">viewer</span></div></td>
</tr>
<tr>
<td>Backends</td>
<td><div class="arch-chips"><span class="arch-chip">CPU (SIMD)</span><span class="arch-chip">CUDA</span><span class="arch-chip">Metal</span><span class="arch-chip">Rayon</span><span class="arch-chip">ONNX</span></div></td>
</tr>
</tbody>
</table>

---

## Performances

<table class="bench-table">
<thead>
<tr><th>Benchmark</th><th>Bibliothèque</th><th>Gain</th></tr>
</thead>
<tbody>
<tr><td>SeraPlot</td><td>Plotly</td><td>6 000×</td></tr>
<tr><td>SeraPlot</td><td>matplotlib</td><td>480×</td></tr>
<tr><td>SeraPlot</td><td>Seaborn</td><td>320×</td></tr>
<tr><td>SeraML KMeans</td><td>sklearn KMeans</td><td>686×</td></tr>
<tr><td>SeraML RF</td><td>sklearn RF</td><td>28×</td></tr>
<tr><td>SeraML LinReg</td><td>sklearn LinReg</td><td>12×</td></tr>
</tbody>
</table>

<div class="note">Benchmarks mesurés sur une seule machine. Les résultats varient selon la taille des données, le matériel et la configuration.</div>
