<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>PermutationImportance</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-util">Utility</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">⚙️ Model Selection</span>
      </div>
      <p class="ml-pg-tagline">Permutation importance — feature importance by permuting each column and measuring score drop. / Importance par permutation — importance des features en permutant chaque colonne et mesurant la baisse de score.</p>
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
import seraplot as sp, json, numpy as np
X = np.random.randn(200, 5)
y = X @ [2, -1, 0, 0.5, 1.2] + np.random.randn(200) * 0.3
model = sp.Ridge(alpha=0.5)
model.fit(X, y)
imp = sp.permutation_importance(model, X, y, n_repeats=5)
print(imp.importances_mean_)
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.PermutationImportance</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_permutation_importance` — aliases: `permutation_importance`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.PermutationImportance(n_repeats=10, scoring=auto)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_repeats</code></td><td><code>int</code></td><td><code>10</code></td><td>Number of permutations per feature.</td></tr>
<tr><td><code>scoring</code></td><td><code>str</code></td><td><code>auto</code></td><td>Scoring metric: auto-detects cls/reg.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `importances_mean`, `importances_std` per feature.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$\text{imp}_j = \bar{s} - \frac{1}{K}\sum_{k=1}^{K} s(\text{perm}_k(X_j))$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, json, numpy as np
X = np.random.randn(200, 5)
y = X @ [2, -1, 0, 0.5, 1.2] + np.random.randn(200) * 0.3
model = sp.Ridge(alpha=0.5)
model.fit(X, y)
imp = sp.permutation_importance(model, X, y, n_repeats=5)
print(imp.importances_mean_)
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_permutation_importance` — alias : `permutation_importance`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.PermutationImportance(n_repeats=10, scoring=auto)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_repeats</code></td><td><code>int</code></td><td><code>10</code></td><td>Nombre de permutations par feature.</td></tr>
<tr><td><code>scoring</code></td><td><code>str</code></td><td><code>auto</code></td><td>Métrique de scoring : auto-détecte cls/reg.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `importances_mean`, `importances_std` par feature.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$\text{imp}_j = \bar{s} - \frac{1}{K}\sum_{k=1}^{K} s(\text{perm}_k(X_j))$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(200, 5)
y = X @ [2, -1, 0, 0.5, 1.2] + np.random.randn(200) * 0.3
model = sp.Ridge(alpha=0.5)
model.fit(X, y)
imp = sp.permutation_importance(model, X, y, n_repeats=5)
print(imp.importances_mean_)
```

</div>

</div>
