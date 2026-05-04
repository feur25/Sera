<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>LinearRegression</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-reg">Regressor</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
      </div>
      <p class="ml-pg-tagline">Ordinary Least Squares — analytical Gram/Cholesky solver, parallel column-major Gram build, rayon-chunked reduction. / Moindres Carrés Ordinaires — solveur analytique Gram/Cholesky, construction Gram parallèle.</p>
    </div>
    <div class="ml-pg-badges">
      <span class="ml-pg-badge ml-pg-badge-speed-hi">⚡ ×4.0 at n=500k</span>
      <span class="ml-pg-badge ml-pg-badge-parity-hi">✓ parity 1.000</span>
    </div>
  </div>
  <div class="ml-pg-stats">
    <div class="ml-pg-stat">
      <span class="ml-pg-stat-lbl">Fit — n=500k</span>
      <span class="ml-pg-stat-val ml-pg-stat-val-green">×4.0</span>
      <span class="ml-pg-stat-sub">114ms vs 284ms sklearn</span>
    </div>
    <div class="ml-pg-stat">
      <span class="ml-pg-stat-lbl">Predict — n=500k</span>
      <span class="ml-pg-stat-val ml-pg-stat-val-blue">×0.55</span>
      <span class="ml-pg-stat-sub">predict overhead negligible</span>
    </div>
    <div class="ml-pg-stat">
      <span class="ml-pg-stat-lbl">Parity</span>
      <span class="ml-pg-stat-val ml-pg-stat-val-purple">1.000</span>
      <span class="ml-pg-stat-sub">identical to sklearn output</span>
    </div>
  </div>
</div>

<div class="ml-pg-qs">
  <div class="ml-pg-qs-header">
    <span class="ml-pg-qs-title">Quick start</span>
  </div>

```python
import seraplot as sp
import numpy as np

X = np.random.randn(1000, 5)
y = X @ [2.0, -1.0, 0.5, 0.3, -0.8] + np.random.randn(1000) * 0.1

model = sp.LinearRegression()
model.fit(X, y)
print(f"R²: {model.score(X, y):.6f}")
print(f"coef: {model.coef_}")
yhat = model.predict(X[:10])
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.LinearRegression</code> has the exact same API as <code>sklearn.linear_model.LinearRegression</code>. Swap the import, keep the rest.<br><strong>FR</strong> — Remplacement direct : <code>sp.LinearRegression</code> expose exactement la même API que sklearn. Changez l'import, gardez le reste.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">Signature</div>

```python
sp.LinearRegression(fit_intercept=True)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>
<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr>
  <td><code>fit_intercept</code></td>
  <td><span class="ml-pg-type ml-pg-type-bool">bool</span></td>
  <td><span class="ml-pg-default">True</span></td>
  <td>When <code>True</code>, fits a bias term alongside the coefficients.</td>
</tr>
</tbody>
</table>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Methods</div>
<table class="ml-pg-table">
<thead><tr><th>Method</th><th>Signature</th><th>Returns</th></tr></thead>
<tbody>
<tr><td><code>fit</code></td><td><code>fit(X, y)</code></td><td><span class="ml-pg-type ml-pg-type-none">self</span></td></tr>
<tr><td><code>predict</code></td><td><code>predict(X)</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[float]</span></td></tr>
<tr><td><code>score</code></td><td><code>score(X, y)</code></td><td><span class="ml-pg-type ml-pg-type-float">float — R²</span></td></tr>
<tr><td><code>get_params</code></td><td><code>get_params()</code></td><td><span class="ml-pg-type ml-pg-type-none">dict</span></td></tr>
<tr><td><code>set_params</code></td><td><code>set_params(**kw)</code></td><td><span class="ml-pg-type ml-pg-type-none">self</span></td></tr>
</tbody>
</table>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Attributes (after fit)</div>
<table class="ml-pg-table">
<thead><tr><th>Attribute</th><th>Type</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>coef_</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[float]</span></td><td>Fitted coefficients, shape $(p,)$</td></tr>
<tr><td><code>intercept_</code></td><td><span class="ml-pg-type ml-pg-type-float">float</span></td><td>Bias term (0.0 if <code>fit_intercept=False</code>)</td></tr>
<tr><td><code>n_features_in_</code></td><td><span class="ml-pg-type ml-pg-type-int">int</span></td><td>Number of input features</td></tr>
</tbody>
</table>
</div>

---

## Algorithmic Functioning

<div class="ml-pg-algo">

Ordinary Least Squares minimises the residual sum of squares:

<div>$$\hat{\beta} = \underset{\beta}{\arg\min}\ \|y - X\beta\|_2^2$$</div>

**Closed-form solution:**

<div>$$\hat{\beta} = (X^TX)^{-1}X^T y$$</div>

The Gram matrix $G = X^TX \in \mathbb{R}^{p \times p}$ is built in parallel with rayon 2048-row chunks, then factored via **Cholesky**. Falls back to **QR** if $G$ is not positive-definite.

**Score — coefficient of determination:**

<div>$$R^2 = 1 - \frac{\displaystyle\sum_{i=1}^n (y_i - \hat{y}_i)^2}{\displaystyle\sum_{i=1}^n (y_i - \bar{y})^2}$$</div>

</div>

---

## Big-data speed profile

<div class="ml-pg-speedbar"><span class="ml-pg-speedbar-label">n = 10,000</span><div class="ml-pg-speedbar-track"><div class="ml-pg-speedbar-fill" style="width:10%"></div></div><span class="ml-pg-speedbar-val">×0.41</span></div>
<div class="ml-pg-speedbar"><span class="ml-pg-speedbar-label">n = 50,000</span><div class="ml-pg-speedbar-track"><div class="ml-pg-speedbar-fill" style="width:52%"></div></div><span class="ml-pg-speedbar-val">×2.07</span></div>
<div class="ml-pg-speedbar"><span class="ml-pg-speedbar-label">n = 200,000</span><div class="ml-pg-speedbar-track"><div class="ml-pg-speedbar-fill" style="width:94%"></div></div><span class="ml-pg-speedbar-val">×3.75</span></div>
<div class="ml-pg-speedbar"><span class="ml-pg-speedbar-label">n = 500,000</span><div class="ml-pg-speedbar-track"><div class="ml-pg-speedbar-fill" style="width:100%"></div></div><span class="ml-pg-speedbar-val">×4.00</span></div>

<div class="ml-pg-note ml-pg-note-info" style="margin-top:14px">
  <span class="ml-pg-note-icon">ℹ️</span>
  <div>Speed improves with <em>n</em> — rayon's parallel Gram build amortises Python overhead. At small n (&lt;10k) sklearn's LAPACK call is near-instant and Rust startup cost dominates.</div>
</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Signature</div>

```python
sp.LinearRegression(fit_intercept=True)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>
<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr>
  <td><code>fit_intercept</code></td>
  <td><span class="ml-pg-type ml-pg-type-bool">bool</span></td>
  <td><span class="ml-pg-default">True</span></td>
  <td>Si <code>True</code>, ajuste un terme de biais en plus des coefficients.</td>
</tr>
</tbody>
</table>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Méthodes</div>
<table class="ml-pg-table">
<thead><tr><th>Méthode</th><th>Signature</th><th>Retourne</th></tr></thead>
<tbody>
<tr><td><code>fit</code></td><td><code>fit(X, y)</code></td><td><span class="ml-pg-type ml-pg-type-none">self</span></td></tr>
<tr><td><code>predict</code></td><td><code>predict(X)</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[float]</span></td></tr>
<tr><td><code>score</code></td><td><code>score(X, y)</code></td><td><span class="ml-pg-type ml-pg-type-float">float — R²</span></td></tr>
<tr><td><code>get_params</code></td><td><code>get_params()</code></td><td><span class="ml-pg-type ml-pg-type-none">dict</span></td></tr>
<tr><td><code>set_params</code></td><td><code>set_params(**kw)</code></td><td><span class="ml-pg-type ml-pg-type-none">self</span></td></tr>
</tbody>
</table>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Attributs (après fit)</div>
<table class="ml-pg-table">
<thead><tr><th>Attribut</th><th>Type</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>coef_</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[float]</span></td><td>Coefficients ajustés, forme $(p,)$</td></tr>
<tr><td><code>intercept_</code></td><td><span class="ml-pg-type ml-pg-type-float">float</span></td><td>Terme de biais (0.0 si <code>fit_intercept=False</code>)</td></tr>
<tr><td><code>n_features_in_</code></td><td><span class="ml-pg-type ml-pg-type-int">int</span></td><td>Nombre de variables d'entrée</td></tr>
</tbody>
</table>
</div>

---

## Fonctionnement algorithmique

<div class="ml-pg-algo">

La régression OLS minimise la somme des carrés des résidus :

<div>$$\hat{\beta} = \underset{\beta}{\arg\min}\ \|y - X\beta\|_2^2$$</div>

**Équation normale :**

<div>$$\hat{\beta} = (X^TX)^{-1}X^T y$$</div>

La matrice de Gram $G = X^TX$ est construite en parallèle via rayon (chunks de 2048 lignes), puis factorisée par **Cholesky**. Bascule sur **QR** si $G$ n'est pas définie positive.

**Score — coefficient de détermination :**

<div>$$R^2 = 1 - \frac{\displaystyle\sum_{i=1}^n (y_i - \hat{y}_i)^2}{\displaystyle\sum_{i=1}^n (y_i - \bar{y})^2}$$</div>

</div>

---

## Profil de vitesse gros volumes

<div class="ml-pg-speedbar"><span class="ml-pg-speedbar-label">n = 10 000</span><div class="ml-pg-speedbar-track"><div class="ml-pg-speedbar-fill" style="width:10%"></div></div><span class="ml-pg-speedbar-val">×0.41</span></div>
<div class="ml-pg-speedbar"><span class="ml-pg-speedbar-label">n = 50 000</span><div class="ml-pg-speedbar-track"><div class="ml-pg-speedbar-fill" style="width:52%"></div></div><span class="ml-pg-speedbar-val">×2.07</span></div>
<div class="ml-pg-speedbar"><span class="ml-pg-speedbar-label">n = 200 000</span><div class="ml-pg-speedbar-track"><div class="ml-pg-speedbar-fill" style="width:94%"></div></div><span class="ml-pg-speedbar-val">×3.75</span></div>
<div class="ml-pg-speedbar"><span class="ml-pg-speedbar-label">n = 500 000</span><div class="ml-pg-speedbar-track"><div class="ml-pg-speedbar-fill" style="width:100%"></div></div><span class="ml-pg-speedbar-val">×4.00</span></div>

<div class="ml-pg-note ml-pg-note-info" style="margin-top:14px">
  <span class="ml-pg-note-icon">ℹ️</span>
  <div>La vitesse s'améliore avec <em>n</em> — la construction parallèle de la Gram amortit l'overhead Python. Pour les petits <em>n</em> (&lt;10k), l'appel LAPACK de sklearn est quasi-instantané et le démarrage Rust domine.</div>
</div>

</div>

