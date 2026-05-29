<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>Lasso</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-reg">Regressor</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">📈 Linear</span>
      </div>
      <p class="ml-pg-tagline">Lasso regression — L1-penalised OLS via coordinate descent. / Régression Lasso — OLS pénalisée L1 par descente de coordonnées.</p>
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
X = np.random.randn(200, 10)
y = X[:, 0] * 2 + X[:, 3] * -1.5 + np.random.randn(200) * 0.3
model = sp.Lasso(alpha=0.1)
model.fit(X, y)
print([f"{c:.3f}" for c in model.coef_])
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.Lasso</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_lasso` — aliases: `lasso`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.Lasso(alpha=1.0, max_iter=1000, tol=1e-4, fit_intercept=true)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>alpha</code></td><td><code>float</code></td><td><code>1.0</code></td><td>L1 regularisation strength.</td></tr>
<tr><td><code>max_iter</code></td><td><code>int</code></td><td><code>1000</code></td><td>Maximum iterations.</td></tr>
<tr><td><code>tol</code></td><td><code>float</code></td><td><code>1e-4</code></td><td>Convergence tolerance.</td></tr>
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

$$\hat{\beta} = \arg\min_{\beta}\|y - X\beta\|_2^2 + \alpha\|\beta\|_1$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(200, 10)
y = X[:, 0] * 2 + X[:, 3] * -1.5 + np.random.randn(200) * 0.3
model = sp.Lasso(alpha=0.1)
model.fit(X, y)
print([f"{c:.3f}" for c in model.coef_])
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_lasso` — alias : `lasso`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.Lasso(alpha=1.0, max_iter=1000, tol=1e-4, fit_intercept=true)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>alpha</code></td><td><code>float</code></td><td><code>1.0</code></td><td>Force de régularisation L1.</td></tr>
<tr><td><code>max_iter</code></td><td><code>int</code></td><td><code>1000</code></td><td>Nombre maximum d'itérations.</td></tr>
<tr><td><code>tol</code></td><td><code>float</code></td><td><code>1e-4</code></td><td>Tolérance de convergence.</td></tr>
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

$$\hat{\beta} = \arg\min_{\beta}\|y - X\beta\|_2^2 + \alpha\|\beta\|_1$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(200, 10)
y = X[:, 0] * 2 + X[:, 3] * -1.5 + np.random.randn(200) * 0.3
model = sp.Lasso(alpha=0.1)
model.fit(X, y)
print([f"{c:.3f}" for c in model.coef_])
```

</div>

</div>
