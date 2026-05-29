<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>AdaboostClassifier</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-cls">Classifier</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">🌲 Tree-Based</span>
      </div>
      <p class="ml-pg-tagline">AdaBoost classifier — adaptive boosting with weighted decision stumps. / AdaBoost classifieur — boosting adaptatif avec stumps de décision pondérés.</p>
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
X, y = make_classification(n_samples=500, n_features=8)
ada = sp.AdaBoostClassifier(n_estimators=50)
ada.fit(X, y)
print(ada.score(X, y))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.AdaboostClassifier</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_adaboost_classifier` — aliases: `adaboost_classifier`, `ada_cls`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.AdaboostClassifier(n_estimators=50, learning_rate=1.0, max_depth=1)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_estimators</code></td><td><code>int</code></td><td><code>50</code></td><td>Number of weak learners.</td></tr>
<tr><td><code>learning_rate</code></td><td><code>float</code></td><td><code>1.0</code></td><td>Shrinkage of each weak learner.</td></tr>
<tr><td><code>max_depth</code></td><td><code>int</code></td><td><code>1</code></td><td>Depth of each base tree.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$F(x) = \text{sign}\left(\sum_{m=1}^{M} \alpha_m h_m(x)\right)$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp
from sklearn.datasets import make_classification
X, y = make_classification(n_samples=500, n_features=8)
ada = sp.AdaBoostClassifier(n_estimators=50)
ada.fit(X, y)
print(ada.score(X, y))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_adaboost_classifier` — alias : `adaboost_classifier`, `ada_cls`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.AdaboostClassifier(n_estimators=50, learning_rate=1.0, max_depth=1)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_estimators</code></td><td><code>int</code></td><td><code>50</code></td><td>Nombre d'apprenants faibles.</td></tr>
<tr><td><code>learning_rate</code></td><td><code>float</code></td><td><code>1.0</code></td><td>Shrinkage de chaque apprenant faible.</td></tr>
<tr><td><code>max_depth</code></td><td><code>int</code></td><td><code>1</code></td><td>Profondeur de chaque arbre de base.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$F(x) = \text{sign}\left(\sum_{m=1}^{M} \alpha_m h_m(x)\right)$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp
from sklearn.datasets import make_classification
X, y = make_classification(n_samples=500, n_features=8)
ada = sp.AdaBoostClassifier(n_estimators=50)
ada.fit(X, y)
print(ada.score(X, y))
```

</div>

</div>

---

<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>AdaboostRegressor</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-reg">Regressor</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">🌲 Tree-Based</span>
      </div>
      <p class="ml-pg-tagline">AdaBoost regressor — adaptive boosting with median-weighted aggregation. / AdaBoost régresseur — boosting adaptatif avec agrégation pondérée par la médiane.</p>
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
y = X[:, 0] ** 2 + np.random.randn(400) * 0.3
ada = sp.AdaBoostRegressor(n_estimators=50, learning_rate=0.8)
ada.fit(X, y)
print(ada.score(X, y))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.AdaboostRegressor</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_adaboost_regressor` — aliases: `adaboost_regressor`, `ada_reg`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.AdaboostRegressor(n_estimators=50, learning_rate=1.0, max_depth=3)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_estimators</code></td><td><code>int</code></td><td><code>50</code></td><td>Number of weak learners.</td></tr>
<tr><td><code>learning_rate</code></td><td><code>float</code></td><td><code>1.0</code></td><td>Shrinkage of each weak learner.</td></tr>
<tr><td><code>max_depth</code></td><td><code>int</code></td><td><code>3</code></td><td>Depth of each base tree.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$\hat{y} = \text{weighted median}\{h_m(x), w_m\}_{m=1}^{M}$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(400, 4)
y = X[:, 0] ** 2 + np.random.randn(400) * 0.3
ada = sp.AdaBoostRegressor(n_estimators=50, learning_rate=0.8)
ada.fit(X, y)
print(ada.score(X, y))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_adaboost_regressor` — alias : `adaboost_regressor`, `ada_reg`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.AdaboostRegressor(n_estimators=50, learning_rate=1.0, max_depth=3)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_estimators</code></td><td><code>int</code></td><td><code>50</code></td><td>Nombre d'apprenants faibles.</td></tr>
<tr><td><code>learning_rate</code></td><td><code>float</code></td><td><code>1.0</code></td><td>Shrinkage de chaque apprenant faible.</td></tr>
<tr><td><code>max_depth</code></td><td><code>int</code></td><td><code>3</code></td><td>Profondeur de chaque arbre de base.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$\hat{y} = \text{médiane pondérée}\{h_m(x), w_m\}_{m=1}^{M}$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(400, 4)
y = X[:, 0] ** 2 + np.random.randn(400) * 0.3
ada = sp.AdaBoostRegressor(n_estimators=50, learning_rate=0.8)
ada.fit(X, y)
print(ada.score(X, y))
```

</div>

</div>
