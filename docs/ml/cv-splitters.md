<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>KfoldSplit</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-util">Utility</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">⚙️ Model Selection</span>
      </div>
      <p class="ml-pg-tagline">K-Fold split — returns train/test index arrays for k-fold cross-validation. / Division K-Fold — retourne les tableaux d'indices train/test pour la validation croisée k-fold.</p>
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
X = np.random.randn(100, 4)
res = json.loads(sp.ml_kfold_split(json.dumps({"X_train": X.tolist(), "n_splits": 5})))
print(len(res["folds"]))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.KfoldSplit</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_kfold_split` — aliases: `kfold_split`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.KfoldSplit(n_splits=5, stratified=false, seed=42)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_splits</code></td><td><code>int</code></td><td><code>5</code></td><td>Number of folds.</td></tr>
<tr><td><code>stratified</code></td><td><code>bool</code></td><td><code>false</code></td><td>Use stratified splits (requires y_train).</td></tr>
<tr><td><code>seed</code></td><td><code>int</code></td><td><code>42</code></td><td>Random seed.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `folds` array, each fold has `train_indices`, `test_indices`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, json, numpy as np
X = np.random.randn(100, 4)
res = json.loads(sp.ml_kfold_split(json.dumps({"X_train": X.tolist(), "n_splits": 5})))
print(len(res["folds"]))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_kfold_split` — alias : `kfold_split`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.KfoldSplit(n_splits=5, stratified=false, seed=42)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_splits</code></td><td><code>int</code></td><td><code>5</code></td><td>Nombre de plis.</td></tr>
<tr><td><code>stratified</code></td><td><code>bool</code></td><td><code>false</code></td><td>Utiliser des splits stratifiés (nécessite y_train).</td></tr>
<tr><td><code>seed</code></td><td><code>int</code></td><td><code>42</code></td><td>Graine aléatoire.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec tableau `folds`, chaque pli a `train_indices`, `test_indices`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, json, numpy as np
X = np.random.randn(100, 4)
res = json.loads(sp.ml_kfold_split(json.dumps({"X_train": X.tolist(), "n_splits": 5})))
print(len(res["folds"]))
```

</div>

</div>

---

<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>CrossValScore</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-util">Utility</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">⚙️ Model Selection</span>
      </div>
      <p class="ml-pg-tagline">Cross-validation score — evaluates model performance across k folds. / Score de validation croisée — évalue la performance du modèle sur k plis.</p>
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
y = X @ [1, -1, 0.5, 0, 1] + np.random.randn(200) * 0.3
payload = {"X_train": X.tolist(), "y_train": y.tolist(), "model": "ridge", "n_splits": 5}
res = json.loads(sp.ml_cross_val_score(json.dumps(payload)))
print(res["scores"], res["mean_score"])
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.CrossValScore</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_cross_val_score` — aliases: `cross_val_score`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.CrossValScore(model=ridge, n_splits=5, scoring=auto)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>model</code></td><td><code>str</code></td><td><code>ridge</code></td><td>Model name for evaluation.</td></tr>
<tr><td><code>n_splits</code></td><td><code>int</code></td><td><code>5</code></td><td>Number of folds.</td></tr>
<tr><td><code>scoring</code></td><td><code>str</code></td><td><code>auto</code></td><td>Score metric.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `scores` array and `mean_score`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$\text{CV}(k) = \frac{1}{k}\sum_{i=1}^{k} s_i$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, json, numpy as np
X = np.random.randn(200, 5)
y = X @ [1, -1, 0.5, 0, 1] + np.random.randn(200) * 0.3
payload = {"X_train": X.tolist(), "y_train": y.tolist(), "model": "ridge", "n_splits": 5}
res = json.loads(sp.ml_cross_val_score(json.dumps(payload)))
print(res["scores"], res["mean_score"])
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_cross_val_score` — alias : `cross_val_score`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.CrossValScore(model=ridge, n_splits=5, scoring=auto)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>model</code></td><td><code>str</code></td><td><code>ridge</code></td><td>Nom du modèle à évaluer.</td></tr>
<tr><td><code>n_splits</code></td><td><code>int</code></td><td><code>5</code></td><td>Nombre de plis.</td></tr>
<tr><td><code>scoring</code></td><td><code>str</code></td><td><code>auto</code></td><td>Métrique de score.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec tableau `scores` et `mean_score`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$\text{CV}(k) = \frac{1}{k}\sum_{i=1}^{k} s_i$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, json, numpy as np
X = np.random.randn(200, 5)
y = X @ [1, -1, 0.5, 0, 1] + np.random.randn(200) * 0.3
payload = {"X_train": X.tolist(), "y_train": y.tolist(), "model": "ridge", "n_splits": 5}
res = json.loads(sp.ml_cross_val_score(json.dumps(payload)))
print(res["scores"], res["mean_score"])
```

</div>

</div>
