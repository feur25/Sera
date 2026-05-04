<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title">Distributed Training &amp; Cloud Planner</h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-new">New in 2.4.34</span>
        <span class="ml-pg-tag ml-pg-tag-dist">Distributed</span>
        <span class="ml-pg-tag ml-pg-tag-gpu">Cloud</span>
      </div>
      <p class="ml-pg-tagline">Ray-style scatter/gather primitives backed by rayon + auto-scaling planner for datasets &gt;1M rows. Zero infra, pure Python. / Primitives scatter/gather style Ray via rayon + planificateur auto-scaling pour jeux de données &gt;1M lignes.</p>
    </div>
    <div class="ml-pg-badges">
      <span class="ml-pg-badge ml-pg-badge-speed-hi">✓ WorkerPool</span>
      <span class="ml-pg-badge ml-pg-badge-parity-hi">✓ cloud_plan</span>
    </div>
  </div>
  <div class="ml-pg-stats">
    <div class="ml-pg-stat">
      <span class="ml-pg-stat-lbl">Max workers</span>
      <span class="ml-pg-stat-val ml-pg-stat-val-green">auto</span>
      <span class="ml-pg-stat-sub">0 = rayon thread count</span>
    </div>
    <div class="ml-pg-stat">
      <span class="ml-pg-stat-lbl">Scatter / gather</span>
      <span class="ml-pg-stat-val ml-pg-stat-val-blue">✓</span>
      <span class="ml-pg-stat-sub">allreduce_mean / sum</span>
    </div>
    <div class="ml-pg-stat">
      <span class="ml-pg-stat-lbl">Planner</span>
      <span class="ml-pg-stat-val ml-pg-stat-val-purple">✓</span>
      <span class="ml-pg-stat-sub">in_memory / chunked / streamed</span>
    </div>
  </div>
</div>

<div class="ml-pg-feature-grid">
  <div class="ml-pg-feature-card">
    <div class="ml-pg-feature-card-icon">🔀</div>
    <div class="ml-pg-feature-card-title">WorkerPool</div>
    <p class="ml-pg-feature-card-desc">Create a pool, scatter rows into shards, train per shard in parallel, then allreduce results back.</p>
  </div>
  <div class="ml-pg-feature-card">
    <div class="ml-pg-feature-card-icon">📐</div>
    <div class="ml-pg-feature-card-title">cloud_plan</div>
    <p class="ml-pg-feature-card-desc">Given n_rows × n_cols and a memory budget, produces the optimal strategy: <code>in_memory</code>, <code>chunked</code> or <code>streamed</code>.</p>
  </div>
  <div class="ml-pg-feature-card">
    <div class="ml-pg-feature-card-icon">📡</div>
    <div class="ml-pg-feature-card-title">cloud_resources</div>
    <p class="ml-pg-feature-card-desc">Runtime snapshot: CPU threads, active backend, OS, architecture, registry path.</p>
  </div>
  <div class="ml-pg-feature-card">
    <div class="ml-pg-feature-card-icon">📂</div>
    <div class="ml-pg-feature-card-title">cloud_count_rows</div>
    <p class="ml-pg-feature-card-desc">Streaming CSV row count — constant memory regardless of file size. Useful before calling <code>cloud_plan</code>.</p>
  </div>
</div>

<div class="ml-pg-qs">
  <div class="ml-pg-qs-header"><span class="ml-pg-qs-title">Quick start — WorkerPool</span></div>

```python
import seraplot as sp, numpy as np

n_rows = 10_000
X = np.random.randn(n_rows, 5)
y = X[:, 0] * 2 + np.random.randn(n_rows) * 0.1

wp = sp.WorkerPool(n_workers=0)
print(f"Pool: {wp.n_workers} workers")

handle = wp.scatter(n_rows)
shards = wp.shards(handle)

coefs = []
for shard in shards:
    Xi = X[shard["start"]:shard["end"]]
    yi = y[shard["start"]:shard["end"]]
    model = sp.LinearRegression()
    model.fit(Xi, yi)
    coefs.append(model.coef_)

mean_coef = wp.allreduce_mean(coefs)
print(f"Aggregated coef: {mean_coef}")

wp.release(handle)
```

</div>

<div class="ml-pg-qs">
  <div class="ml-pg-qs-header"><span class="ml-pg-qs-title">Quick start — Cloud planner</span></div>

```python
import seraplot as sp

plan = sp.cloud_plan(n_rows=5_000_000, n_cols=50, mem_budget_mb=4096)
print(f"Strategy    : {plan['strategy']}")
print(f"Workers     : {plan['recommended_workers']}")
print(f"Chunk rows  : {plan['recommended_chunk_rows']}")
print(f"Chunks      : {plan['n_chunks']}")
print(f"Est. seconds: {plan['estimated_seconds']:.1f}s")

resources = sp.cloud_resources()
print(f"\nCPU threads : {resources['cpu_threads']}")
print(f"Backend     : {resources['backend']}")
print(f"OS          : {resources['os']} / {resources['arch']}")
```

</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">WorkerPool</div>

```python
wp = sp.WorkerPool(n_workers=0)
```

<table class="ml-pg-table">
<thead><tr><th>Method / Attr</th><th>Signature</th><th>Returns</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_workers</code></td><td>property</td><td><span class="ml-pg-type ml-pg-type-int">int</span></td><td>Actual number of workers (resolved from <code>n_workers=0</code> = rayon thread count)</td></tr>
<tr><td><code>scatter</code></td><td><code>(n_rows) -> int</code></td><td><span class="ml-pg-type ml-pg-type-int">handle</span></td><td>Partition <em>n_rows</em> rows into shards, return opaque handle</td></tr>
<tr><td><code>shards</code></td><td><code>(handle) -> list[dict]</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[dict]</span></td><td>Get shard list for handle: <code>[{"id":0,"start":0,"end":625}, …]</code></td></tr>
<tr><td><code>release</code></td><td><code>(handle)</code></td><td><span class="ml-pg-type ml-pg-type-none">None</span></td><td>Free memory associated with the scatter handle</td></tr>
<tr><td><code>allreduce_mean</code></td><td><code>(vecs: list[list[float]]) -> list[float]</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[float]</span></td><td>Element-wise mean across all vectors (same length)</td></tr>
<tr><td><code>allreduce_sum</code></td><td><code>(vecs: list[list[float]]) -> list[float]</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[float]</span></td><td>Element-wise sum across all vectors</td></tr>
</tbody>
</table>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">cloud_plan</div>
<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_rows</code></td><td><span class="ml-pg-type ml-pg-type-int">int</span></td><td>—</td><td>Number of dataset rows</td></tr>
<tr><td><code>n_cols</code></td><td><span class="ml-pg-type ml-pg-type-int">int</span></td><td>—</td><td>Number of feature columns</td></tr>
<tr><td><code>mem_budget_mb</code></td><td><span class="ml-pg-type ml-pg-type-int">int</span></td><td><span class="ml-pg-default">2048</span></td><td>Available RAM budget in MB</td></tr>
</tbody>
</table>
<div class="ml-pg-ret">Returns → <code>dict</code> with keys: <code>n_rows</code>, <code>n_cols</code>, <code>bytes_total</code>, <code>mem_budget_mb</code>, <code>recommended_workers</code>, <code>recommended_chunk_rows</code>, <code>n_chunks</code>, <code>estimated_seconds</code>, <code>strategy</code></div>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">cloud_resources</div>
<div class="ml-pg-ret">Returns → <code>dict</code> — <code>cpu_threads</code> (int), <code>backend</code> (str), <code>os</code> (str), <code>arch</code> (str), <code>registry_dir</code> (str), <code>tasks_dir</code> (str)</div>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">cloud_count_rows</div>
<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>path</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td>—</td><td>Path to CSV file</td></tr>
<tr><td><code>chunk_rows</code></td><td><span class="ml-pg-type ml-pg-type-int">int</span></td><td><span class="ml-pg-default">100000</span></td><td>Internal read buffer size</td></tr>
<tr><td><code>has_header</code></td><td><span class="ml-pg-type ml-pg-type-bool">bool</span></td><td><span class="ml-pg-default">True</span></td><td>Skip first line as header</td></tr>
<tr><td><code>delimiter</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td><span class="ml-pg-default">","</span></td><td>CSV delimiter</td></tr>
</tbody>
</table>
<div class="ml-pg-ret">Returns → <code>int</code> — number of data rows (header excluded if <code>has_header=True</code>)</div>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Strategy reference (cloud_plan)</div>
<table class="ml-pg-table">
<thead><tr><th>Strategy</th><th>Condition</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>in_memory</code></td><td>Dataset fits in budget</td><td>Load all at once, use full parallel training</td></tr>
<tr><td><code>chunked</code></td><td>Dataset &gt; budget, &lt;2× budget per chunk</td><td>Process in chunks, aggregate results per chunk</td></tr>
<tr><td><code>streamed</code></td><td>Dataset &gt;&gt; budget</td><td>Stream rows one chunk at a time, online aggregation</td></tr>
</tbody>
</table>
</div>

<div class="ml-pg-note ml-pg-note-info">
  <span class="ml-pg-note-icon">ℹ️</span>
  <div>Call <code>cloud_count_rows(path)</code> on your CSV file, then pass the result to <code>cloud_plan</code> to get the optimal strategy <em>before</em> loading any data into memory.</div>
</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">WorkerPool</div>

```python
wp = sp.WorkerPool(n_workers=0)
```

<table class="ml-pg-table">
<thead><tr><th>Méthode / Attr</th><th>Signature</th><th>Retourne</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_workers</code></td><td>propriété</td><td><span class="ml-pg-type ml-pg-type-int">int</span></td><td>Nombre réel de workers (0 = nombre de threads rayon)</td></tr>
<tr><td><code>scatter</code></td><td><code>(n_rows) -> int</code></td><td><span class="ml-pg-type ml-pg-type-int">handle</span></td><td>Partitionne <em>n_rows</em> lignes en shards, retourne un handle opaque</td></tr>
<tr><td><code>shards</code></td><td><code>(handle) -> list[dict]</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[dict]</span></td><td>Récupère les shards : <code>[{"id":0,"start":0,"end":625}, …]</code></td></tr>
<tr><td><code>release</code></td><td><code>(handle)</code></td><td><span class="ml-pg-type ml-pg-type-none">None</span></td><td>Libère la mémoire associée au handle scatter</td></tr>
<tr><td><code>allreduce_mean</code></td><td><code>(vecs)</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[float]</span></td><td>Moyenne élément par élément sur tous les vecteurs</td></tr>
<tr><td><code>allreduce_sum</code></td><td><code>(vecs)</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[float]</span></td><td>Somme élément par élément sur tous les vecteurs</td></tr>
</tbody>
</table>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">cloud_plan</div>
<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_rows</code></td><td><span class="ml-pg-type ml-pg-type-int">int</span></td><td>—</td><td>Nombre de lignes du jeu de données</td></tr>
<tr><td><code>n_cols</code></td><td><span class="ml-pg-type ml-pg-type-int">int</span></td><td>—</td><td>Nombre de colonnes de features</td></tr>
<tr><td><code>mem_budget_mb</code></td><td><span class="ml-pg-type ml-pg-type-int">int</span></td><td><span class="ml-pg-default">2048</span></td><td>Budget RAM disponible en Mo</td></tr>
</tbody>
</table>
<div class="ml-pg-ret">Retourne → <code>dict</code> avec les clés : <code>n_rows</code>, <code>n_cols</code>, <code>bytes_total</code>, <code>mem_budget_mb</code>, <code>recommended_workers</code>, <code>recommended_chunk_rows</code>, <code>n_chunks</code>, <code>estimated_seconds</code>, <code>strategy</code></div>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">cloud_resources</div>
<div class="ml-pg-ret">Retourne → <code>dict</code> — <code>cpu_threads</code> (int), <code>backend</code> (str), <code>os</code> (str), <code>arch</code> (str), <code>registry_dir</code> (str), <code>tasks_dir</code> (str)</div>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">cloud_count_rows</div>
<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>path</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td>—</td><td>Chemin vers le fichier CSV</td></tr>
<tr><td><code>chunk_rows</code></td><td><span class="ml-pg-type ml-pg-type-int">int</span></td><td><span class="ml-pg-default">100000</span></td><td>Taille du buffer de lecture interne</td></tr>
<tr><td><code>has_header</code></td><td><span class="ml-pg-type ml-pg-type-bool">bool</span></td><td><span class="ml-pg-default">True</span></td><td>Ignorer la première ligne (en-tête)</td></tr>
<tr><td><code>delimiter</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td><span class="ml-pg-default">","</span></td><td>Séparateur CSV</td></tr>
</tbody>
</table>
<div class="ml-pg-ret">Retourne → <code>int</code> — nombre de lignes de données (en-tête exclu si <code>has_header=True</code>)</div>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Référence des stratégies (cloud_plan)</div>
<table class="ml-pg-table">
<thead><tr><th>Stratégie</th><th>Condition</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>in_memory</code></td><td>Dataset tient dans le budget</td><td>Chargement complet, entraînement parallèle total</td></tr>
<tr><td><code>chunked</code></td><td>Dataset &gt; budget, &lt;2× budget par chunk</td><td>Traitement en chunks, agrégation des résultats</td></tr>
<tr><td><code>streamed</code></td><td>Dataset &gt;&gt; budget</td><td>Streaming ligne par ligne, agrégation en ligne</td></tr>
</tbody>
</table>
</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div>Appelez <code>cloud_count_rows(chemin)</code> sur votre CSV, puis passez le résultat à <code>cloud_plan</code> pour obtenir la stratégie optimale <em>avant</em> de charger la moindre donnée en mémoire.</div>
</div>

</div>
