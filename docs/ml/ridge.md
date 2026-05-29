<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>Ridge</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-reg">Regressor</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">📈 Linear</span>
      </div>
      <p class="ml-pg-tagline">Ridge regression — L2-penalised OLS with Cholesky solver. / Régression Ridge — OLS pénalisée L2 avec solveur Cholesky.</p>
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
X = np.random.randn(300, 5)
y = X @ np.array([1, -2, 0.5, 1.5, -0.8]) + np.random.randn(300)
reg = sp.Ridge(alpha=0.5)
reg.fit(X, y)
print(f"R²: {reg.score(X, y):.4f}")
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.Ridge</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_ridge` — aliases: `ridge`, `ridge_regression`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.Ridge(alpha=1.0, fit_intercept=true)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>alpha</code></td><td><code>float</code></td><td><code>1.0</code></td><td>L2 regularisation strength.</td></tr>
<tr><td><code>fit_intercept</code></td><td><code>bool</code></td><td><code>true</code></td><td>Fit an intercept term.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `predictions`, `coef`, `intercept`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$\hat{\beta} = (X^TX + \alpha I)^{-1}X^Ty$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(300, 5)
y = X @ np.array([1, -2, 0.5, 1.5, -0.8]) + np.random.randn(300)
reg = sp.Ridge(alpha=0.5)
reg.fit(X, y)
print(f"R²: {reg.score(X, y):.4f}")
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_ridge` — alias : `ridge`, `ridge_regression`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.Ridge(alpha=1.0, fit_intercept=true)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>alpha</code></td><td><code>float</code></td><td><code>1.0</code></td><td>Force de régularisation L2.</td></tr>
<tr><td><code>fit_intercept</code></td><td><code>bool</code></td><td><code>true</code></td><td>Ajuster un terme d'intercept.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `predictions`, `coef`, `intercept`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$\hat{\beta} = (X^TX + \alpha I)^{-1}X^Ty$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(300, 5)
y = X @ [1, -2, 0.5, 1.5, -0.8] + np.random.randn(300)
reg = sp.Ridge(alpha=0.5)
reg.fit(X, y)
print(f"R² : {reg.score(X, y):.4f}")
```

</div>

</div>

---

<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>RidgeClassifier</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-cls">Classifier</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">📈 Linear</span>
      </div>
      <p class="ml-pg-tagline">Ridge classifier — one-vs-rest Ridge regression mapped to class labels. / Classificateur Ridge — régression Ridge one-vs-rest mappée en labels.</p>
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
X = np.random.randn(300, 5)
y = (X[:, 0] - X[:, 2] > 0).astype(int)
clf = sp.RidgeClassifier(alpha=1.0)
clf.fit(X, y)
print(f"Accuracy: {clf.score(X, y):.3f}")
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.RidgeClassifier</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_ridge_classifier` — aliases: `ridge_classifier`, `ridge_cls`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.RidgeClassifier(alpha=1.0)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>alpha</code></td><td><code>float</code></td><td><code>1.0</code></td><td>L2 regularisation strength.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$\hat{y} = \text{sign}(X\hat{\beta}), \quad \hat{\beta} = (X^TX + \alpha I)^{-1}X^TY$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(300, 5)
y = (X[:, 0] - X[:, 2] > 0).astype(int)
clf = sp.RidgeClassifier(alpha=1.0)
clf.fit(X, y)
print(f"Accuracy: {clf.score(X, y):.3f}")
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_ridge_classifier` — alias : `ridge_classifier`, `ridge_cls`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.RidgeClassifier(alpha=1.0)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>alpha</code></td><td><code>float</code></td><td><code>1.0</code></td><td>Force de régularisation L2.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$\hat{y} = \text{sign}(X\hat{\beta}), \quad \hat{\beta} = (X^TX + \alpha I)^{-1}X^TY$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(300, 5)
y = (X[:, 0] - X[:, 2] > 0).astype(int)
clf = sp.RidgeClassifier(alpha=1.0)
clf.fit(X, y)
print(f"Précision : {clf.score(X, y):.3f}")
```

</div>

</div>
