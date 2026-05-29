<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>KnnClassifier</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-cls">Classifier</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">🔍 Neighbors</span>
      </div>
      <p class="ml-pg-tagline">K-Nearest Neighbors classifier — majority vote among k nearest neighbors. / K plus proches voisins classifieur — vote majoritaire parmi les k voisins les plus proches.</p>
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
import seraplot as sp
from sklearn.datasets import load_iris
X, y = load_iris(return_X_y=True)
knn = sp.KNeighborsClassifier(n_neighbors=5)
knn.fit(X, y)
print(knn.score(X, y))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.KnnClassifier</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_knn_classifier` — aliases: `knn_classifier`, `knn_cls`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.KnnClassifier(n_neighbors=5, weights=uniform)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_neighbors</code></td><td><code>int</code></td><td><code>5</code></td><td>Number of nearest neighbors.</td></tr>
<tr><td><code>weights</code></td><td><code>str</code></td><td><code>uniform</code></td><td>Weighting: `uniform` or `distance`.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$\hat{y} = \arg\max_c \sum_{i \in \mathcal{N}_k(x)} w_i \mathbf{1}[y_i = c]$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp
from sklearn.datasets import load_iris
X, y = load_iris(return_X_y=True)
knn = sp.KNeighborsClassifier(n_neighbors=5)
knn.fit(X, y)
print(knn.score(X, y))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_knn_classifier` — alias : `knn_classifier`, `knn_cls`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.KnnClassifier(n_neighbors=5, weights=uniform)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_neighbors</code></td><td><code>int</code></td><td><code>5</code></td><td>Nombre de voisins les plus proches.</td></tr>
<tr><td><code>weights</code></td><td><code>str</code></td><td><code>uniform</code></td><td>Pondération : `uniform` ou `distance`.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$\hat{y} = \arg\max_c \sum_{i \in \mathcal{N}_k(x)} w_i \mathbf{1}[y_i = c]$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp
from sklearn.datasets import load_iris
X, y = load_iris(return_X_y=True)
knn = sp.KNeighborsClassifier(n_neighbors=5)
knn.fit(X, y)
print(knn.score(X, y))
```

</div>

</div>

---

<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>KnnRegressor</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-reg">Regressor</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">🔍 Neighbors</span>
      </div>
      <p class="ml-pg-tagline">K-Nearest Neighbors regressor — weighted average of k nearest neighbors. / K plus proches voisins régresseur — moyenne pondérée des k voisins les plus proches.</p>
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
X = np.random.randn(300, 3)
y = X[:, 0] ** 2 + X[:, 1] + np.random.randn(300) * 0.5
knn = sp.KNeighborsRegressor(n_neighbors=7, weights="distance")
knn.fit(X, y)
print(knn.score(X, y))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.KnnRegressor</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_knn_regressor` — aliases: `knn_regressor`, `knn_reg`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.KnnRegressor(n_neighbors=5, weights=uniform)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_neighbors</code></td><td><code>int</code></td><td><code>5</code></td><td>Number of nearest neighbors.</td></tr>
<tr><td><code>weights</code></td><td><code>str</code></td><td><code>uniform</code></td><td>Weighting: `uniform` or `distance`.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$\hat{y} = \frac{\sum_{i \in \mathcal{N}_k(x)} w_i y_i}{\sum_{i \in \mathcal{N}_k(x)} w_i}$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(300, 3)
y = X[:, 0] ** 2 + X[:, 1] + np.random.randn(300) * 0.5
knn = sp.KNeighborsRegressor(n_neighbors=7, weights="distance")
knn.fit(X, y)
print(knn.score(X, y))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_knn_regressor` — alias : `knn_regressor`, `knn_reg`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.KnnRegressor(n_neighbors=5, weights=uniform)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_neighbors</code></td><td><code>int</code></td><td><code>5</code></td><td>Nombre de voisins les plus proches.</td></tr>
<tr><td><code>weights</code></td><td><code>str</code></td><td><code>uniform</code></td><td>Pondération : `uniform` ou `distance`.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$\hat{y} = \frac{\sum_{i \in \mathcal{N}_k(x)} w_i y_i}{\sum_{i \in \mathcal{N}_k(x)} w_i}$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(300, 3)
y = X[:, 0] ** 2 + X[:, 1] + np.random.randn(300) * 0.5
knn = sp.KNeighborsRegressor(n_neighbors=7, weights="distance")
knn.fit(X, y)
print(knn.score(X, y))
```

</div>

</div>

---

<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>NearestCentroid</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-cls">Classifier</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">🔍 Neighbors</span>
      </div>
      <p class="ml-pg-tagline">Nearest Centroid classifier — assigns class by closest class mean. / Classificateur par centroïde le plus proche — assigne la classe par la moyenne de classe la plus proche.</p>
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
import seraplot as sp
from sklearn.datasets import load_iris
X, y = load_iris(return_X_y=True)
nc = sp.NearestCentroid()
nc.fit(X, y)
print(nc.score(X, y))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.NearestCentroid</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_nearest_centroid` — aliases: `nearest_centroid`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.NearestCentroid()
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<p><em>No constructor parameters.</em></p>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$\hat{y} = \arg\min_c \|x - \mu_c\|_2^2$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp
from sklearn.datasets import load_iris
X, y = load_iris(return_X_y=True)
nc = sp.NearestCentroid()
nc.fit(X, y)
print(nc.score(X, y))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_nearest_centroid` — alias : `nearest_centroid`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.NearestCentroid()
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<p><em>Aucun paramètre de constructeur.</em></p>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$\hat{y} = \arg\min_c \|x - \mu_c\|_2^2$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp
from sklearn.datasets import load_iris
X, y = load_iris(return_X_y=True)
nc = sp.NearestCentroid()
nc.fit(X, y)
print(nc.score(X, y))
```

</div>

</div>
