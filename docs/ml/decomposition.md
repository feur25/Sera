<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>Pca</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-trx">Transformer</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">🔬 Decomposition</span>
      </div>
      <p class="ml-pg-tagline">PCA — Principal Component Analysis via truncated SVD. / PCA — Analyse en Composantes Principales via SVD tronquée.</p>
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
X = np.random.randn(300, 10)
pca = sp.PCA(n_components=3)
pca.fit(X)
Xt = pca.transform(X)
print(Xt.shape, pca.explained_variance_ratio_)
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.Pca</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_pca` — aliases: `pca`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.Pca(n_components=2)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_components</code></td><td><code>int</code></td><td><code>2</code></td><td>Number of principal components.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `transformed` (n×k matrix), `explained_variance_ratio`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$X_{\text{proj}} = X W_k, \quad W_k = \text{top-}k\text{ right singular vectors of } \tilde{X}$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(300, 10)
pca = sp.PCA(n_components=3)
pca.fit(X)
Xt = pca.transform(X)
print(Xt.shape, pca.explained_variance_ratio_)
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_pca` — alias : `pca`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.Pca(n_components=2)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_components</code></td><td><code>int</code></td><td><code>2</code></td><td>Nombre de composantes principales.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `transformed` (matrice n×k), `explained_variance_ratio`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$X_{\text{proj}} = X W_k, \quad W_k = k\text{ premiers vecteurs singuliers droits de } \tilde{X}$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(300, 10)
pca = sp.PCA(n_components=3)
pca.fit(X)
Xt = pca.transform(X)
print(Xt.shape, pca.explained_variance_ratio_)
```

</div>

</div>

---

<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>TruncatedSvd</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-trx">Transformer</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">🔬 Decomposition</span>
      </div>
      <p class="ml-pg-tagline">TruncatedSVD — truncated Singular Value Decomposition (no centering, sparse-friendly). / TruncatedSVD — Décomposition en Valeurs Singulières tronquée (sans centrage, compatible sparse).</p>
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
X = np.abs(np.random.randn(200, 15))
svd = sp.TruncatedSVD(n_components=5)
svd.fit(X)
Xt = svd.transform(X)
print(Xt.shape)
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.TruncatedSvd</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_truncated_svd` — aliases: `truncated_svd`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.TruncatedSvd(n_components=2)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_components</code></td><td><code>int</code></td><td><code>2</code></td><td>Number of components to keep.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `transformed`, `explained_variance_ratio`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$X \approx U_k \Sigma_k V_k^T$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
X = np.abs(np.random.randn(200, 15))
svd = sp.TruncatedSVD(n_components=5)
svd.fit(X)
Xt = svd.transform(X)
print(Xt.shape)
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_truncated_svd` — alias : `truncated_svd`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.TruncatedSvd(n_components=2)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_components</code></td><td><code>int</code></td><td><code>2</code></td><td>Nombre de composantes à conserver.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `transformed`, `explained_variance_ratio`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$X \approx U_k \Sigma_k V_k^T$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
X = np.abs(np.random.randn(200, 15))
svd = sp.TruncatedSVD(n_components=5)
svd.fit(X)
Xt = svd.transform(X)
print(Xt.shape)
```

</div>

</div>
