<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>LinearSvc</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-cls">Classifier</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">⚡ SVM</span>
      </div>
      <p class="ml-pg-tagline">LinearSVC — linear Support Vector Machine for classification via dual coordinate descent. / LinearSVC — Machine à vecteurs de support linéaire pour classification.</p>
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
svc = sp.LinearSVC(C=1.0)
svc.fit(X, y)
print(svc.score(X, y))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.LinearSvc</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_linear_svc` — aliases: `linear_svc`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.LinearSvc(C=1.0, max_iter=1000, tol=1e-4)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>C</code></td><td><code>float</code></td><td><code>1.0</code></td><td>Regularisation parameter (inverse margin).</td></tr>
<tr><td><code>max_iter</code></td><td><code>int</code></td><td><code>1000</code></td><td>Maximum iterations.</td></tr>
<tr><td><code>tol</code></td><td><code>float</code></td><td><code>1e-4</code></td><td>Convergence tolerance.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$\min_{w,b}\frac{1}{2}\|w\|^2 + C\sum_i \max(0, 1 - y_i(w^Tx_i + b))$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp
from sklearn.datasets import make_classification
X, y = make_classification(n_samples=500, n_features=8)
svc = sp.LinearSVC(C=1.0)
svc.fit(X, y)
print(svc.score(X, y))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_linear_svc` — alias : `linear_svc`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.LinearSvc(C=1.0, max_iter=1000, tol=1e-4)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>C</code></td><td><code>float</code></td><td><code>1.0</code></td><td>Paramètre de régularisation (inverse de la marge).</td></tr>
<tr><td><code>max_iter</code></td><td><code>int</code></td><td><code>1000</code></td><td>Nombre maximum d'itérations.</td></tr>
<tr><td><code>tol</code></td><td><code>float</code></td><td><code>1e-4</code></td><td>Tolérance de convergence.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$\min_{w,b}\frac{1}{2}\|w\|^2 + C\sum_i \max(0, 1 - y_i(w^Tx_i + b))$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp
from sklearn.datasets import make_classification
X, y = make_classification(n_samples=500, n_features=8)
svc = sp.LinearSVC(C=1.0)
svc.fit(X, y)
print(svc.score(X, y))
```

</div>

</div>

---

<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>LinearSvr</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-reg">Regressor</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">⚡ SVM</span>
      </div>
      <p class="ml-pg-tagline">LinearSVR — epsilon-insensitive linear Support Vector Regression. / LinearSVR — régression linéaire par vecteurs de support avec perte epsilon-insensible.</p>
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
y = X[:, 0] * 2 - X[:, 2] + np.random.randn(400) * 0.5
svr = sp.LinearSVR(C=1.0, epsilon=0.1)
svr.fit(X, y)
print(svr.score(X, y))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.LinearSvr</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_linear_svr` — aliases: `linear_svr`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.LinearSvr(C=1.0, epsilon=0.1, max_iter=1000, tol=1e-4)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>C</code></td><td><code>float</code></td><td><code>1.0</code></td><td>Regularisation parameter.</td></tr>
<tr><td><code>epsilon</code></td><td><code>float</code></td><td><code>0.1</code></td><td>Epsilon-tube width.</td></tr>
<tr><td><code>max_iter</code></td><td><code>int</code></td><td><code>1000</code></td><td>Maximum iterations.</td></tr>
<tr><td><code>tol</code></td><td><code>float</code></td><td><code>1e-4</code></td><td>Convergence tolerance.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$\min_{w,b}\frac{1}{2}\|w\|^2 + C\sum_i \max(0, |y_i - (w^Tx_i+b)| - \varepsilon)$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(400, 4)
y = X[:, 0] * 2 - X[:, 2] + np.random.randn(400) * 0.5
svr = sp.LinearSVR(C=1.0, epsilon=0.1)
svr.fit(X, y)
print(svr.score(X, y))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_linear_svr` — alias : `linear_svr`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.LinearSvr(C=1.0, epsilon=0.1, max_iter=1000, tol=1e-4)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>C</code></td><td><code>float</code></td><td><code>1.0</code></td><td>Paramètre de régularisation.</td></tr>
<tr><td><code>epsilon</code></td><td><code>float</code></td><td><code>0.1</code></td><td>Largeur du tube epsilon.</td></tr>
<tr><td><code>max_iter</code></td><td><code>int</code></td><td><code>1000</code></td><td>Nombre maximum d'itérations.</td></tr>
<tr><td><code>tol</code></td><td><code>float</code></td><td><code>1e-4</code></td><td>Tolérance de convergence.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `predictions`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$\min_{w,b}\frac{1}{2}\|w\|^2 + C\sum_i \max(0, |y_i - (w^Tx_i+b)| - \varepsilon)$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(400, 4)
y = X[:, 0] * 2 - X[:, 2] + np.random.randn(400) * 0.5
svr = sp.LinearSVR(C=1.0, epsilon=0.1)
svr.fit(X, y)
print(svr.score(X, y))
```

</div>

</div>
