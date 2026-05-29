<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>SgdClassifier</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-cls">Classifier</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">📈 Linear</span>
      </div>
      <p class="ml-pg-tagline">SGDClassifier — stochastic gradient descent for linear classifiers. / SGDClassifier — descente de gradient stochastique pour classifieurs linéaires.</p>
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
X = np.random.randn(1000, 5)
y = (X[:, 0] > 0).astype(int)
clf = sp.SGDClassifier(loss="hinge", alpha=1e-4)
clf.fit(X, y)
print(clf.score(X, y))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.SgdClassifier</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_sgd_classifier` — aliases: `sgd_classifier`, `sgd_cls`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.SgdClassifier(loss=hinge, alpha=0.0001, max_iter=1000, tol=1e-3, eta0=1.0, fit_intercept=true)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>loss</code></td><td><code>str</code></td><td><code>hinge</code></td><td>Loss: `hinge`, `log`, `modified_huber`, `squared_hinge`.</td></tr>
<tr><td><code>alpha</code></td><td><code>float</code></td><td><code>0.0001</code></td><td>Regularisation multiplier.</td></tr>
<tr><td><code>max_iter</code></td><td><code>int</code></td><td><code>1000</code></td><td>Maximum passes over the data.</td></tr>
<tr><td><code>tol</code></td><td><code>float</code></td><td><code>1e-3</code></td><td>Convergence tolerance.</td></tr>
<tr><td><code>eta0</code></td><td><code>float</code></td><td><code>1.0</code></td><td>Initial learning rate.</td></tr>
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

$$\beta_{t+1} = \beta_t - \eta_t \nabla_{\beta} L(y_i, x_i^T\beta_t)$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(1000, 5)
y = (X[:, 0] > 0).astype(int)
clf = sp.SGDClassifier(loss="hinge", alpha=1e-4)
clf.fit(X, y)
print(clf.score(X, y))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_sgd_classifier` — alias : `sgd_classifier`, `sgd_cls`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.SgdClassifier(loss=hinge, alpha=0.0001, max_iter=1000, tol=1e-3, eta0=1.0, fit_intercept=true)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>loss</code></td><td><code>str</code></td><td><code>hinge</code></td><td>Perte : `hinge`, `log`, `modified_huber`, `squared_hinge`.</td></tr>
<tr><td><code>alpha</code></td><td><code>float</code></td><td><code>0.0001</code></td><td>Multiplicateur de régularisation.</td></tr>
<tr><td><code>max_iter</code></td><td><code>int</code></td><td><code>1000</code></td><td>Nombre maximum de passes sur les données.</td></tr>
<tr><td><code>tol</code></td><td><code>float</code></td><td><code>1e-3</code></td><td>Tolérance de convergence.</td></tr>
<tr><td><code>eta0</code></td><td><code>float</code></td><td><code>1.0</code></td><td>Taux d'apprentissage initial.</td></tr>
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

$$\beta_{t+1} = \beta_t - \eta_t \nabla_{\beta} L(y_i, x_i^T\beta_t)$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(1000, 5)
y = (X[:, 0] > 0).astype(int)
clf = sp.SGDClassifier(loss="hinge", alpha=1e-4)
clf.fit(X, y)
print(clf.score(X, y))
```

</div>

</div>

---

<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>SgdRegressor</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-reg">Regressor</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">📈 Linear</span>
      </div>
      <p class="ml-pg-tagline">SGDRegressor — stochastic gradient descent for linear regressors. / SGDRegressor — descente de gradient stochastique pour régresseurs linéaires.</p>
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
X = np.random.randn(1000, 3)
y = X @ [1.5, -0.5, 2.0] + np.random.randn(1000) * 0.5
reg = sp.SGDRegressor(alpha=1e-4, max_iter=500)
reg.fit(X, y)
print(reg.score(X, y))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.SgdRegressor</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_sgd_regressor` — aliases: `sgd_regressor`, `sgd_reg`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.SgdRegressor(alpha=0.0001, max_iter=1000, tol=1e-3, eta0=0.1, fit_intercept=true)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>alpha</code></td><td><code>float</code></td><td><code>0.0001</code></td><td>Regularisation multiplier.</td></tr>
<tr><td><code>max_iter</code></td><td><code>int</code></td><td><code>1000</code></td><td>Maximum passes.</td></tr>
<tr><td><code>tol</code></td><td><code>float</code></td><td><code>1e-3</code></td><td>Convergence tolerance.</td></tr>
<tr><td><code>eta0</code></td><td><code>float</code></td><td><code>0.1</code></td><td>Initial learning rate.</td></tr>
<tr><td><code>fit_intercept</code></td><td><code>bool</code></td><td><code>true</code></td><td>Fit an intercept.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `predictions`, `coef`, `intercept`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$\beta_{t+1} = \beta_t - \eta_t \cdot 2(\hat{y}_i - y_i) x_i$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(1000, 3)
y = X @ [1.5, -0.5, 2.0] + np.random.randn(1000) * 0.5
reg = sp.SGDRegressor(alpha=1e-4, max_iter=500)
reg.fit(X, y)
print(reg.score(X, y))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_sgd_regressor` — alias : `sgd_regressor`, `sgd_reg`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.SgdRegressor(alpha=0.0001, max_iter=1000, tol=1e-3, eta0=0.1, fit_intercept=true)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>alpha</code></td><td><code>float</code></td><td><code>0.0001</code></td><td>Multiplicateur de régularisation.</td></tr>
<tr><td><code>max_iter</code></td><td><code>int</code></td><td><code>1000</code></td><td>Nombre maximum de passes.</td></tr>
<tr><td><code>tol</code></td><td><code>float</code></td><td><code>1e-3</code></td><td>Tolérance de convergence.</td></tr>
<tr><td><code>eta0</code></td><td><code>float</code></td><td><code>0.1</code></td><td>Taux d'apprentissage initial.</td></tr>
<tr><td><code>fit_intercept</code></td><td><code>bool</code></td><td><code>true</code></td><td>Ajuster un intercept.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `predictions`, `coef`, `intercept`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$\beta_{t+1} = \beta_t - \eta_t \cdot 2(\hat{y}_i - y_i) x_i$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(1000, 3)
y = X @ [1.5, -0.5, 2.0] + np.random.randn(1000) * 0.5
reg = sp.SGDRegressor(alpha=1e-4, max_iter=500)
reg.fit(X, y)
print(reg.score(X, y))
```

</div>

</div>
