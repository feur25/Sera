<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>GradientBoostingClassifier</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-cls">Classifier</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">🌲 Tree-Based</span>
      </div>
      <p class="ml-pg-tagline">Gradient Boosting classifier — sequential additive model with log-loss. / Gradient Boosting classifieur — modèle additif séquentiel avec log-loss.</p>
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
from sklearn.datasets import make_classification
X, y = make_classification(n_samples=500, n_features=10)
gb = sp.GradientBoostingClassifier(n_estimators=100, learning_rate=0.1)
gb.fit(X, y)
print(gb.score(X, y))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.GradientBoostingClassifier</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_gradient_boosting_classifier` — aliases: `gradient_boosting_classifier`, `gb_cls`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.GradientBoostingClassifier(n_estimators=100, learning_rate=0.1, max_depth=3, min_samples_split=2, min_samples_leaf=1)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_estimators</code></td><td><code>int</code></td><td><code>100</code></td><td>Number of boosting rounds.</td></tr>
<tr><td><code>learning_rate</code></td><td><code>float</code></td><td><code>0.1</code></td><td>Shrinkage applied to each tree.</td></tr>
<tr><td><code>max_depth</code></td><td><code>int</code></td><td><code>3</code></td><td>Maximum tree depth.</td></tr>
<tr><td><code>min_samples_split</code></td><td><code>int</code></td><td><code>2</code></td><td>Min samples to split.</td></tr>
<tr><td><code>min_samples_leaf</code></td><td><code>int</code></td><td><code>1</code></td><td>Min samples in leaf.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$F_m(x) = F_{m-1}(x) + \eta \cdot h_m(x)$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp
from sklearn.datasets import make_classification
X, y = make_classification(n_samples=500, n_features=10)
gb = sp.GradientBoostingClassifier(n_estimators=100, learning_rate=0.1)
gb.fit(X, y)
print(gb.score(X, y))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_gradient_boosting_classifier` — alias : `gradient_boosting_classifier`, `gb_cls`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.GradientBoostingClassifier(n_estimators=100, learning_rate=0.1, max_depth=3, min_samples_split=2, min_samples_leaf=1)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_estimators</code></td><td><code>int</code></td><td><code>100</code></td><td>Nombre de tours de boosting.</td></tr>
<tr><td><code>learning_rate</code></td><td><code>float</code></td><td><code>0.1</code></td><td>Shrinkage appliqué à chaque arbre.</td></tr>
<tr><td><code>max_depth</code></td><td><code>int</code></td><td><code>3</code></td><td>Profondeur maximale de chaque arbre.</td></tr>
<tr><td><code>min_samples_split</code></td><td><code>int</code></td><td><code>2</code></td><td>Min d'échantillons pour diviser.</td></tr>
<tr><td><code>min_samples_leaf</code></td><td><code>int</code></td><td><code>1</code></td><td>Min d'échantillons en feuille.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$F_m(x) = F_{m-1}(x) + \eta \cdot h_m(x)$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp
from sklearn.datasets import make_classification
X, y = make_classification(n_samples=500, n_features=10)
gb = sp.GradientBoostingClassifier(n_estimators=100, learning_rate=0.1)
gb.fit(X, y)
print(gb.score(X, y))
```

</div>

</div>

---

<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>GradientBoostingRegressor</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-reg">Regressor</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">🌲 Tree-Based</span>
      </div>
      <p class="ml-pg-tagline">Gradient Boosting regressor — sequential additive model minimising MSE. / Gradient Boosting régresseur — modèle additif séquentiel minimisant la MSE.</p>
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
X = np.random.randn(400, 5)
y = X[:, 0] ** 2 - X[:, 1] + np.random.randn(400) * 0.3
gb = sp.GradientBoostingRegressor(n_estimators=100, learning_rate=0.05)
gb.fit(X, y)
print(gb.score(X, y))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.GradientBoostingRegressor</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_gradient_boosting_regressor` — aliases: `gradient_boosting_regressor`, `gb_reg`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.GradientBoostingRegressor(n_estimators=100, learning_rate=0.1, max_depth=3, min_samples_split=2, min_samples_leaf=1)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_estimators</code></td><td><code>int</code></td><td><code>100</code></td><td>Number of boosting rounds.</td></tr>
<tr><td><code>learning_rate</code></td><td><code>float</code></td><td><code>0.1</code></td><td>Shrinkage applied to each tree.</td></tr>
<tr><td><code>max_depth</code></td><td><code>int</code></td><td><code>3</code></td><td>Maximum tree depth.</td></tr>
<tr><td><code>min_samples_split</code></td><td><code>int</code></td><td><code>2</code></td><td>Min samples to split.</td></tr>
<tr><td><code>min_samples_leaf</code></td><td><code>int</code></td><td><code>1</code></td><td>Min samples in leaf.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$F_m(x) = F_{m-1}(x) + \eta \cdot h_m(x), \quad h_m = \arg\min_h \sum_i(r_i - h(x_i))^2$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(400, 5)
y = X[:, 0] ** 2 - X[:, 1] + np.random.randn(400) * 0.3
gb = sp.GradientBoostingRegressor(n_estimators=100, learning_rate=0.05)
gb.fit(X, y)
print(gb.score(X, y))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_gradient_boosting_regressor` — alias : `gradient_boosting_regressor`, `gb_reg`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.GradientBoostingRegressor(n_estimators=100, learning_rate=0.1, max_depth=3, min_samples_split=2, min_samples_leaf=1)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_estimators</code></td><td><code>int</code></td><td><code>100</code></td><td>Nombre de tours de boosting.</td></tr>
<tr><td><code>learning_rate</code></td><td><code>float</code></td><td><code>0.1</code></td><td>Shrinkage appliqué à chaque arbre.</td></tr>
<tr><td><code>max_depth</code></td><td><code>int</code></td><td><code>3</code></td><td>Profondeur maximale.</td></tr>
<tr><td><code>min_samples_split</code></td><td><code>int</code></td><td><code>2</code></td><td>Min d'échantillons pour diviser.</td></tr>
<tr><td><code>min_samples_leaf</code></td><td><code>int</code></td><td><code>1</code></td><td>Min d'échantillons en feuille.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$F_m(x) = F_{m-1}(x) + \eta \cdot h_m(x), \quad r_i = y_i - F_{m-1}(x_i)$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(400, 5)
y = X[:, 0] ** 2 - X[:, 1] + np.random.randn(400) * 0.3
gb = sp.GradientBoostingRegressor(n_estimators=100, learning_rate=0.05)
gb.fit(X, y)
print(gb.score(X, y))
```

</div>

</div>
