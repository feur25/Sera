<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>GridSearchCv</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-util">Utility</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">⚙️ Model Selection</span>
      </div>
      <p class="ml-pg-tagline">Grid search with cross-validation — exhaustive hyperparameter search. / Recherche en grille avec validation croisée — recherche exhaustive d'hyperparamètres.</p>
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
y = X[:, 0] * 2 + np.random.randn(200) * 0.3
payload = {"X_train": X.tolist(), "y_train": y.tolist(), "param_grid": {"alpha": [0.1, 1.0, 10.0]}}
res = json.loads(sp.ml_grid_search_cv(json.dumps(payload)))
print(res["best_params"], res["best_score"])
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.GridSearchCv</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_grid_search_cv` — aliases: `grid_search_cv`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.GridSearchCv(param_grid={}, n_splits=5, scoring=auto)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>param_grid</code></td><td><code>dict</code></td><td><code>{}</code></td><td>Hyperparameter grid to search.</td></tr>
<tr><td><code>n_splits</code></td><td><code>int</code></td><td><code>5</code></td><td>Number of CV folds.</td></tr>
<tr><td><code>scoring</code></td><td><code>str</code></td><td><code>auto</code></td><td>Scoring metric.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `best_params`, `best_score`, `cv_results`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, json, numpy as np
X = np.random.randn(200, 5)
y = X[:, 0] * 2 + np.random.randn(200) * 0.3
payload = {"X_train": X.tolist(), "y_train": y.tolist(), "param_grid": {"alpha": [0.1, 1.0, 10.0]}}
res = json.loads(sp.ml_grid_search_cv(json.dumps(payload)))
print(res["best_params"], res["best_score"])
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_grid_search_cv` — alias : `grid_search_cv`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.GridSearchCv(param_grid={}, n_splits=5, scoring=auto)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>param_grid</code></td><td><code>dict</code></td><td><code>{}</code></td><td>Grille d'hyperparamètres à explorer.</td></tr>
<tr><td><code>n_splits</code></td><td><code>int</code></td><td><code>5</code></td><td>Nombre de plis CV.</td></tr>
<tr><td><code>scoring</code></td><td><code>str</code></td><td><code>auto</code></td><td>Métrique de scoring.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `best_params`, `best_score`, `cv_results`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, json, numpy as np
X = np.random.randn(200, 5)
y = X[:, 0] * 2 + np.random.randn(200) * 0.3
payload = {"X_train": X.tolist(), "y_train": y.tolist(), "param_grid": {"alpha": [0.1, 1.0, 10.0]}}
res = json.loads(sp.ml_grid_search_cv(json.dumps(payload)))
print(res["best_params"], res["best_score"])
```

</div>

</div>
