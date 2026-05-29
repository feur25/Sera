<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>DbscanFitPredict</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-clu">Clusterer</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">🔮 Clustering</span>
      </div>
      <p class="ml-pg-tagline">DBSCAN — density-based spatial clustering, no preset number of clusters. / DBSCAN — clustering spatial basé sur la densité, sans nombre de clusters prédéfini.</p>
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
X, _ = make_blobs(n_samples=300, centers=4, cluster_std=0.6)
result = sp.DBSCAN(eps=0.8, min_samples=5).fit_predict(X)
print(result)
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.DbscanFitPredict</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_dbscan_fit_predict` — aliases: `dbscan_fit_predict`, `ml_dbscan`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.DbscanFitPredict(eps=0.5, min_samples=5)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>eps</code></td><td><code>float</code></td><td><code>0.5</code></td><td>Neighbourhood radius.</td></tr>
<tr><td><code>min_samples</code></td><td><code>int</code></td><td><code>5</code></td><td>Minimum points to form a core point.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `labels` (−1 = noise), `n_clusters`, `n_noise`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

A point $p$ is a **core point** if $|\mathcal{N}_{\varepsilon}(p)| \geq m$. Clusters are connected components of core points.

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
from sklearn.datasets import make_blobs
X, _ = make_blobs(n_samples=300, centers=4, cluster_std=0.6)
result = sp.DBSCAN(eps=0.8, min_samples=5).fit_predict(X)
print(result)
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_dbscan_fit_predict` — alias : `dbscan_fit_predict`, `ml_dbscan`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.DbscanFitPredict(eps=0.5, min_samples=5)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>eps</code></td><td><code>float</code></td><td><code>0.5</code></td><td>Rayon de voisinage.</td></tr>
<tr><td><code>min_samples</code></td><td><code>int</code></td><td><code>5</code></td><td>Minimum de points pour former un point noyau.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `labels` (−1 = bruit), `n_clusters`, `n_noise`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

Un point $p$ est un **point noyau** si $|\mathcal{N}_{\varepsilon}(p)| \geq m$. Les clusters sont des composantes connexes de points noyaux.

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
from sklearn.datasets import make_blobs
X, _ = make_blobs(n_samples=300, centers=4, cluster_std=0.6)
result = sp.DBSCAN(eps=0.8, min_samples=5).fit_predict(X)
print(result)
```

</div>

</div>
