<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>RandomForestClassifier</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-cls">Classifier</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">🌲 Tree-Based</span>
      </div>
      <p class="ml-pg-tagline">Random Forest classifier — bagging of CART trees with feature subsampling. / Random Forest classifieur — bagging d'arbres CART avec sous-échantillonnage de features.</p>
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
rf = sp.RandomForestClassifier(n_estimators=100, max_depth=6)
rf.fit(X, y)
print(rf.score(X, y), rf.feature_importances_)
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.RandomForestClassifier</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_random_forest_classifier` — aliases: `random_forest_classifier`, `rf_cls`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.RandomForestClassifier(n_estimators=100, max_depth=∞, min_samples_split=2, min_samples_leaf=1, max_features=sqrt)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_estimators</code></td><td><code>int</code></td><td><code>100</code></td><td>Number of trees.</td></tr>
<tr><td><code>max_depth</code></td><td><code>int</code></td><td><code>∞</code></td><td>Maximum tree depth.</td></tr>
<tr><td><code>min_samples_split</code></td><td><code>int</code></td><td><code>2</code></td><td>Min samples to split a node.</td></tr>
<tr><td><code>min_samples_leaf</code></td><td><code>int</code></td><td><code>1</code></td><td>Min samples in a leaf.</td></tr>
<tr><td><code>max_features</code></td><td><code>str</code></td><td><code>sqrt</code></td><td>Features per split: `sqrt`, `log2`, `all`, or int.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `predictions`, `feature_importances`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$\hat{y} = \text{majority}\{h_b(x)\}_{b=1}^{B}$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp
from sklearn.datasets import load_iris
X, y = load_iris(return_X_y=True)
rf = sp.RandomForestClassifier(n_estimators=100, max_depth=6)
rf.fit(X, y)
print(rf.score(X, y), rf.feature_importances_)
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_random_forest_classifier` — alias : `random_forest_classifier`, `rf_cls`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.RandomForestClassifier(n_estimators=100, max_depth=∞, min_samples_split=2, min_samples_leaf=1, max_features=sqrt)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_estimators</code></td><td><code>int</code></td><td><code>100</code></td><td>Nombre d'arbres.</td></tr>
<tr><td><code>max_depth</code></td><td><code>int</code></td><td><code>∞</code></td><td>Profondeur maximale.</td></tr>
<tr><td><code>min_samples_split</code></td><td><code>int</code></td><td><code>2</code></td><td>Min d'échantillons pour diviser.</td></tr>
<tr><td><code>min_samples_leaf</code></td><td><code>int</code></td><td><code>1</code></td><td>Min d'échantillons en feuille.</td></tr>
<tr><td><code>max_features</code></td><td><code>str</code></td><td><code>sqrt</code></td><td>Features par split : `sqrt`, `log2`, `all`, ou int.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `predictions`, `feature_importances`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$\hat{y} = \text{majority}\{h_b(x)\}_{b=1}^{B}$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp
from sklearn.datasets import load_iris
X, y = load_iris(return_X_y=True)
rf = sp.RandomForestClassifier(n_estimators=100, max_depth=6)
rf.fit(X, y)
print(rf.score(X, y), rf.feature_importances_)
```

</div>

</div>

---

<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>RandomForestRegressor</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-reg">Regressor</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">🌲 Tree-Based</span>
      </div>
      <p class="ml-pg-tagline">Random Forest regressor — averaged ensemble of CART trees. / Random Forest régresseur — ensemble moyenné d'arbres CART.</p>
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
X = np.random.randn(500, 6)
y = X[:, 0] ** 2 + X[:, 1] * X[:, 2] + np.random.randn(500) * 0.3
rf = sp.RandomForestRegressor(n_estimators=50, max_depth=8)
rf.fit(X, y)
print(rf.score(X, y))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.RandomForestRegressor</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_random_forest_regressor` — aliases: `random_forest_regressor`, `rf_reg`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.RandomForestRegressor(n_estimators=100, max_depth=∞, min_samples_split=2, min_samples_leaf=1, max_features=sqrt)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_estimators</code></td><td><code>int</code></td><td><code>100</code></td><td>Number of trees.</td></tr>
<tr><td><code>max_depth</code></td><td><code>int</code></td><td><code>∞</code></td><td>Maximum tree depth.</td></tr>
<tr><td><code>min_samples_split</code></td><td><code>int</code></td><td><code>2</code></td><td>Min samples to split.</td></tr>
<tr><td><code>min_samples_leaf</code></td><td><code>int</code></td><td><code>1</code></td><td>Min samples in leaf.</td></tr>
<tr><td><code>max_features</code></td><td><code>str</code></td><td><code>sqrt</code></td><td>Features per split.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `predictions`, `feature_importances`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$\hat{y} = \frac{1}{B}\sum_{b=1}^{B} h_b(x)$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(500, 6)
y = X[:, 0] ** 2 + X[:, 1] * X[:, 2] + np.random.randn(500) * 0.3
rf = sp.RandomForestRegressor(n_estimators=50, max_depth=8)
rf.fit(X, y)
print(rf.score(X, y))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_random_forest_regressor` — alias : `random_forest_regressor`, `rf_reg`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.RandomForestRegressor(n_estimators=100, max_depth=∞, min_samples_split=2, min_samples_leaf=1, max_features=sqrt)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_estimators</code></td><td><code>int</code></td><td><code>100</code></td><td>Nombre d'arbres.</td></tr>
<tr><td><code>max_depth</code></td><td><code>int</code></td><td><code>∞</code></td><td>Profondeur maximale.</td></tr>
<tr><td><code>min_samples_split</code></td><td><code>int</code></td><td><code>2</code></td><td>Min d'échantillons pour diviser.</td></tr>
<tr><td><code>min_samples_leaf</code></td><td><code>int</code></td><td><code>1</code></td><td>Min d'échantillons en feuille.</td></tr>
<tr><td><code>max_features</code></td><td><code>str</code></td><td><code>sqrt</code></td><td>Features par split.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `predictions`, `feature_importances`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$\hat{y} = \frac{1}{B}\sum_{b=1}^{B} h_b(x)$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(500, 6)
y = X[:, 0] ** 2 + X[:, 1] * X[:, 2] + np.random.randn(500) * 0.3
rf = sp.RandomForestRegressor(n_estimators=50, max_depth=8)
rf.fit(X, y)
print(rf.score(X, y))
```

</div>

</div>
