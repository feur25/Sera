<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>KmeansFitPredict</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-clu">Clusterer</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">🔮 Clustering</span>
      </div>
      <p class="ml-pg-tagline">K-Means — Lloyd's algorithm with k-means++ initialisation. / K-Means — algorithme de Lloyd avec initialisation k-means++.</p>
    </div>
    <div class="ml-pg-badges">
      <span class="ml-pg-badge ml-pg-badge-speed-hi">⚡ Rust-native</span>
      <span class="ml-pg-badge ml-pg-badge-parity-hi">✓ sklearn parity</span>
    </div>
  </div>
</div>

<div class="ml-pg-qs">
  <div class="ml-pg-qs-header">
    <span class="ml-pg-qs-title">Quick start — Python</span>
  </div>

```python
import seraplot as sp, numpy as np
from sklearn.datasets import make_blobs
X, _ = make_blobs(n_samples=500, centers=4)
result = sp.KMeans(k=4, max_iter=300).fit_predict(X)
print(result)
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.KmeansFitPredict</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_kmeans_fit_predict` — aliases: `kmeans_fit_predict`, `ml_kmeans`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.KmeansFitPredict(k=3, max_iter=300, n_init=10)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>k</code></td><td><code>int</code></td><td><code>3</code></td><td>Number of clusters.</td></tr>
<tr><td><code>max_iter</code></td><td><code>int</code></td><td><code>300</code></td><td>Maximum iterations.</td></tr>
<tr><td><code>n_init</code></td><td><code>int</code></td><td><code>10</code></td><td>Number of random restarts.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `labels`, `inertia`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$J = \sum_{k=1}^{K}\sum_{i \in C_k}\|x_i - \mu_k\|_2^2$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
from sklearn.datasets import make_blobs
X, _ = make_blobs(n_samples=500, centers=4)
result = sp.KMeans(k=4, max_iter=300).fit_predict(X)
print(result)
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_kmeans_fit_predict` — alias : `kmeans_fit_predict`, `ml_kmeans`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.KmeansFitPredict(k=3, max_iter=300, n_init=10)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>k</code></td><td><code>int</code></td><td><code>3</code></td><td>Nombre de clusters.</td></tr>
<tr><td><code>max_iter</code></td><td><code>int</code></td><td><code>300</code></td><td>Nombre maximum d'itérations.</td></tr>
<tr><td><code>n_init</code></td><td><code>int</code></td><td><code>10</code></td><td>Nombre de redémarrages aléatoires.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `labels`, `inertia`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$J = \sum_{k=1}^{K}\sum_{i \in C_k}\|x_i - \mu_k\|_2^2$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
from sklearn.datasets import make_blobs
X, _ = make_blobs(n_samples=500, centers=4)
result = sp.KMeans(k=4, max_iter=300).fit_predict(X)
print(result)
```

</div>

</div>
