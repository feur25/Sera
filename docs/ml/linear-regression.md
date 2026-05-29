<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>LinearRegression</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-reg">Regressor</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">📈 Linear</span>
      </div>
      <p class="ml-pg-tagline">Ordinary Least Squares linear regression — analytical Gram/Cholesky solver. / Régression linéaire par moindres carrés ordinaires — solveur analytique Gram/Cholesky.</p>
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
X = np.random.randn(500, 4)
y = X @ [2, -1, 0.5, 1.5] + np.random.randn(500) * 0.3
model = sp.LinearRegression()
model.fit(X, y)
print(model.coef_, model.intercept_, model.score(X, y))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.LinearRegression</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_linear_regression` — aliases: `linear_regression`, `linreg`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.LinearRegression(fit_intercept=true)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
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

$$\hat{\beta} = (X^T X)^{-1} X^T y$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(500, 4)
y = X @ [2, -1, 0.5, 1.5] + np.random.randn(500) * 0.3
model = sp.LinearRegression()
model.fit(X, y)
print(model.coef_, model.intercept_, model.score(X, y))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_linear_regression` — alias : `linear_regression`, `linreg`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.LinearRegression(fit_intercept=true)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
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

$$\hat{\beta} = (X^T X)^{-1} X^T y$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(500, 4)
y = X @ [2, -1, 0.5, 1.5] + np.random.randn(500) * 0.3
model = sp.LinearRegression()
model.fit(X, y)
print(model.coef_, model.intercept_, model.score(X, y))
```

</div>

</div>
