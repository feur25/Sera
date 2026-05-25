<style>
.sml-hero{position:relative;padding:40px 44px;border-radius:18px;background:linear-gradient(140deg,#050c1a 0%,#071620 45%,#0c1c20 80%,#040d10 100%);border:1px solid rgba(34,211,238,.18);overflow:hidden;margin:1.4em 0 2.2em;box-shadow:0 24px 60px -20px rgba(0,0,0,.8)}
.sml-hero::before{content:"";position:absolute;top:-30%;right:-8%;width:55%;height:180%;background:radial-gradient(ellipse,rgba(34,211,238,.1) 0%,transparent 65%);pointer-events:none}
.sml-hero h1{margin:0 0 10px;font-size:38px;background:linear-gradient(135deg,#67e8f9 0%,#a5f3fc 50%,#22d3ee 100%);-webkit-background-clip:text;background-clip:text;color:transparent;font-weight:900;letter-spacing:-.03em;border:none}
.sml-hero p{margin:0;color:#94a3b8;font-size:15px;line-height:1.65;max-width:72ch}
.sml-badges{display:flex;gap:8px;flex-wrap:wrap;margin-top:14px}
.sml-badge{padding:3px 10px;background:rgba(34,211,238,.1);border:1px solid rgba(34,211,238,.2);border-radius:999px;font-size:11px;font-weight:700;color:#a5f3fc;letter-spacing:.04em}

.sml-grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(260px,1fr));gap:14px;margin:2em 0}
.sml-card{padding:20px 22px;background:#050d10;border:1px solid #0d2028;border-radius:12px;transition:border-color .15s,transform .15s}
.sml-card:hover{border-color:rgba(34,211,238,.35);transform:translateY(-2px)}
.sml-card h3{margin:0 0 8px;font-size:13px;color:#67e8f9;font-weight:700;letter-spacing:.06em;text-transform:uppercase;border:none}
.sml-card p{margin:0;color:#64748b;font-size:13px;line-height:1.55}

.sml-bench{margin:2em 0;background:#050d10;border:1px solid #0d2028;border-radius:12px;overflow:hidden}
.sml-bench-row{display:flex;align-items:center;gap:12px;padding:10px 18px;border-bottom:1px solid #0d2028}
.sml-bench-row:last-child{border-bottom:none}
.sml-bench-name{width:220px;flex-shrink:0;font-size:12.5px;color:#94a3b8;font-weight:600}
.sml-bench-bar-wrap{flex:1;height:8px;background:#0d2028;border-radius:999px;overflow:hidden}
.sml-bench-bar{height:100%;border-radius:999px;background:linear-gradient(90deg,#22d3ee,#0891b2)}
.sml-bench-x{font-size:11px;font-weight:700;color:#67e8f9;white-space:nowrap;min-width:48px;text-align:right}
</style>

<div class="sml-hero">
<h1>SeraML</h1>
<p>A drop-in replacement for scikit-learn, written in Rust.<br>Same API. Same patterns. 2× to 686× faster — with GPU and distributed backends built in.</p>
<div class="sml-badges">
<span class="sml-badge">Drop-in sklearn replacement</span>
<span class="sml-badge">2–686× faster</span>
<span class="sml-badge">GPU backend</span>
<span class="sml-badge">Distributed training</span>
<span class="sml-badge">PyPI</span>
</div>
</div>

## What SeraML is

SeraML is the machine learning component of the Sera framework. It exposes a `sklearn`-compatible API — `fit`, `predict`, `score`, `transform` — but the implementation runs in Rust via PyO3. The Python overhead is negligible.

<div class="sml-grid">
<div class="sml-card"><h3>Same API as sklearn</h3><p>Replace <code>from sklearn.linear_model import LinearRegression</code> with <code>from seraml.linear_model import LinearRegression</code> — the rest of your code is unchanged.</p></div>
<div class="sml-card"><h3>GPU backend</h3><p>Matrix operations fall through to a CUDA or Metal backend when a GPU is available. No code change required.</p></div>
<div class="sml-card"><h3>Distributed training</h3><p>Partition large datasets across cores or machines using the built-in distributed backend — no Spark or Dask dependency.</p></div>
<div class="sml-card"><h3>Model registry</h3><p>Save, version, and load trained models with a single call. Export to ONNX, PowerBI, and Tableau.</p></div>
</div>

## Performance vs scikit-learn

<div class="sml-bench">
<div class="sml-bench-row"><div class="sml-bench-name">KMeans (10k pts)</div><div class="sml-bench-bar-wrap"><div class="sml-bench-bar" style="width:99%"></div></div><div class="sml-bench-x">686×</div></div>
<div class="sml-bench-row"><div class="sml-bench-name">RandomForest fit</div><div class="sml-bench-bar-wrap"><div class="sml-bench-bar" style="width:82%"></div></div><div class="sml-bench-x">28×</div></div>
<div class="sml-bench-row"><div class="sml-bench-name">GradientBoosting</div><div class="sml-bench-bar-wrap"><div class="sml-bench-bar" style="width:65%"></div></div><div class="sml-bench-x">18×</div></div>
<div class="sml-bench-row"><div class="sml-bench-name">LinearRegression</div><div class="sml-bench-bar-wrap"><div class="sml-bench-bar" style="width:50%"></div></div><div class="sml-bench-x">12×</div></div>
<div class="sml-bench-row"><div class="sml-bench-name">PCA (50k×100)</div><div class="sml-bench-bar-wrap"><div class="sml-bench-bar" style="width:38%"></div></div><div class="sml-bench-x">8×</div></div>
<div class="sml-bench-row"><div class="sml-bench-name">StandardScaler</div><div class="sml-bench-bar-wrap"><div class="sml-bench-bar" style="width:28%"></div></div><div class="sml-bench-x">4×</div></div>
</div>

## What is covered

| Category | Estimators |
|---|---|
| Linear Models | LinearRegression, Ridge, Lasso, ElasticNet, LogisticRegression, SGD |
| Tree-based | DecisionTree, RandomForest, GradientBoosting, AdaBoost |
| Clustering | KMeans, DBSCAN |
| Neighbors | KNN, NearestCentroid |
| Naive Bayes | GaussianNB, MultinomialNB, BernoulliNB |
| SVM | LinearSVC, LinearSVR |
| Preprocessing | StandardScaler, MinMaxScaler, RobustScaler, MaxAbsScaler, Normalizer |
| Decomposition | PCA, TruncatedSVD |
| Model Selection | GridSearchCV, RandomizedSearchCV, KFold, StratifiedKFold |
| Anomaly | IsolationForest |
| Metrics | Full classification, regression, and clustering suites |

→ [Installation →](installation.html)  
→ [Quick Start →](quickstart.html)  
→ [ML Reference →](../ml/index.html)
