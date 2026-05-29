<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>DecisionTreeClassifier</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-cls">Classifier</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">🌲 Tree-Based</span>
      </div>
      <p class="ml-pg-tagline">Decision tree classifier — CART with Gini/Entropy criterion, binned splits. / Arbre de décision classifieur — CART avec critère Gini/Entropie, splits binnés.</p>
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
from sklearn.datasets import load_iris
X, y = load_iris(return_X_y=True)
tree = sp.DecisionTreeClassifier(max_depth=4)
tree.fit(X, y)
print(f"Accuracy: {tree.score(X, y):.3f}")
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.DecisionTreeClassifier</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_decision_tree_classifier` — aliases: `decision_tree_classifier`, `dt_cls`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.DecisionTreeClassifier(max_depth=∞, min_samples_split=2, min_samples_leaf=1, max_features=null, criterion=gini)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>max_depth</code></td><td><code>int</code></td><td><code>∞</code></td><td>Maximum tree depth.</td></tr>
<tr><td><code>min_samples_split</code></td><td><code>int</code></td><td><code>2</code></td><td>Minimum samples to split a node.</td></tr>
<tr><td><code>min_samples_leaf</code></td><td><code>int</code></td><td><code>1</code></td><td>Minimum samples in a leaf.</td></tr>
<tr><td><code>max_features</code></td><td><code>int|str</code></td><td><code>null</code></td><td>Max features per split (int or `sqrt`/`log2`).</td></tr>
<tr><td><code>criterion</code></td><td><code>str</code></td><td><code>gini</code></td><td>Split criterion: `gini` or `entropy`.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `predictions`, `feature_importances`, `classes`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$\text{Gini}(t) = 1 - \sum_{k} p_k^2$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
from sklearn.datasets import load_iris
X, y = load_iris(return_X_y=True)
tree = sp.DecisionTreeClassifier(max_depth=4)
tree.fit(X, y)
print(f"Accuracy: {tree.score(X, y):.3f}")
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_decision_tree_classifier` — alias : `decision_tree_classifier`, `dt_cls`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.DecisionTreeClassifier(max_depth=∞, min_samples_split=2, min_samples_leaf=1, max_features=null, criterion=gini)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>max_depth</code></td><td><code>int</code></td><td><code>∞</code></td><td>Profondeur maximale de l'arbre.</td></tr>
<tr><td><code>min_samples_split</code></td><td><code>int</code></td><td><code>2</code></td><td>Minimum d'échantillons pour diviser un nœud.</td></tr>
<tr><td><code>min_samples_leaf</code></td><td><code>int</code></td><td><code>1</code></td><td>Minimum d'échantillons dans une feuille.</td></tr>
<tr><td><code>max_features</code></td><td><code>int|str</code></td><td><code>null</code></td><td>Max features par split (int ou `sqrt`/`log2`).</td></tr>
<tr><td><code>criterion</code></td><td><code>str</code></td><td><code>gini</code></td><td>Critère de split : `gini` ou `entropy`.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `predictions`, `feature_importances`, `classes`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$\text{Gini}(t) = 1 - \sum_{k} p_k^2$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
from sklearn.datasets import load_iris
X, y = load_iris(return_X_y=True)
tree = sp.DecisionTreeClassifier(max_depth=4)
tree.fit(X, y)
print(f"Précision : {tree.score(X, y):.3f}")
```

</div>

</div>

---

<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>DecisionTreeRegressor</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-reg">Regressor</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">🌲 Tree-Based</span>
      </div>
      <p class="ml-pg-tagline">Decision tree regressor — CART with MSE variance reduction, binned splits. / Arbre de décision régresseur — CART avec réduction de variance MSE, splits binnés.</p>
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
X = np.random.randn(400, 4)
y = X[:, 0] ** 2 + X[:, 1] - X[:, 2] + np.random.randn(400) * 0.5
tree = sp.DecisionTreeRegressor(max_depth=5)
tree.fit(X, y)
print(tree.score(X, y))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.DecisionTreeRegressor</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_decision_tree_regressor` — aliases: `decision_tree_regressor`, `dt_reg`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.DecisionTreeRegressor(max_depth=∞, min_samples_split=2, min_samples_leaf=1, max_features=null)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>max_depth</code></td><td><code>int</code></td><td><code>∞</code></td><td>Maximum tree depth.</td></tr>
<tr><td><code>min_samples_split</code></td><td><code>int</code></td><td><code>2</code></td><td>Minimum samples to split a node.</td></tr>
<tr><td><code>min_samples_leaf</code></td><td><code>int</code></td><td><code>1</code></td><td>Minimum samples in a leaf.</td></tr>
<tr><td><code>max_features</code></td><td><code>int|str</code></td><td><code>null</code></td><td>Max features per split.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `predictions`, `feature_importances`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$\text{MSE}(t) = \frac{1}{n_t}\sum_{i \in t}(y_i - \bar{y}_t)^2$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(400, 4)
y = X[:, 0] ** 2 + X[:, 1] - X[:, 2] + np.random.randn(400) * 0.5
tree = sp.DecisionTreeRegressor(max_depth=5)
tree.fit(X, y)
print(tree.score(X, y))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_decision_tree_regressor` — alias : `decision_tree_regressor`, `dt_reg`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.DecisionTreeRegressor(max_depth=∞, min_samples_split=2, min_samples_leaf=1, max_features=null)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>max_depth</code></td><td><code>int</code></td><td><code>∞</code></td><td>Profondeur maximale de l'arbre.</td></tr>
<tr><td><code>min_samples_split</code></td><td><code>int</code></td><td><code>2</code></td><td>Minimum d'échantillons pour diviser un nœud.</td></tr>
<tr><td><code>min_samples_leaf</code></td><td><code>int</code></td><td><code>1</code></td><td>Minimum d'échantillons dans une feuille.</td></tr>
<tr><td><code>max_features</code></td><td><code>int|str</code></td><td><code>null</code></td><td>Max features par split.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `predictions`, `feature_importances`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$\text{MSE}(t) = \frac{1}{n_t}\sum_{i \in t}(y_i - \bar{y}_t)^2$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(400, 4)
y = X[:, 0] ** 2 + X[:, 1] - X[:, 2] + np.random.randn(400) * 0.5
tree = sp.DecisionTreeRegressor(max_depth=5)
tree.fit(X, y)
print(tree.score(X, y))
```

</div>

</div>
