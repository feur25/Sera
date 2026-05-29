<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>SaveModel</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-util">Utility</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">📦 Registry</span>
      </div>
      <p class="ml-pg-tagline">Save model to the in-memory registry with name, version, and metadata. / Sauvegarder le modèle dans le registre en mémoire avec nom, version et métadonnées.</p>
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
X = np.random.randn(100, 3)
y = X[:, 0] * 2 + np.random.randn(100) * 0.3
model = sp.Ridge(alpha=0.5)
model.fit(X, y)
res = json.loads(sp.ml_save_model(json.dumps({"name": "my_ridge", "kind": "ridge"})))
print(res["version"])
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.SaveModel</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_save_model` — aliases: `save_model`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.SaveModel(name=required, kind=required)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>name</code></td><td><code>str</code></td><td><code>required</code></td><td>Model name.</td></tr>
<tr><td><code>kind</code></td><td><code>str</code></td><td><code>required</code></td><td>Model type (e.g. `ridge`, `random_forest`).</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with saved `ModelRecord` (id, name, version, created_at).

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, json, numpy as np
X = np.random.randn(100, 3)
y = X[:, 0] * 2 + np.random.randn(100) * 0.3
model = sp.Ridge(alpha=0.5)
model.fit(X, y)
res = json.loads(sp.ml_save_model(json.dumps({"name": "my_ridge", "kind": "ridge"})))
print(res["version"])
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_save_model` — alias : `save_model`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.SaveModel(name=required, kind=required)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>name</code></td><td><code>str</code></td><td><code>required</code></td><td>Nom du modèle.</td></tr>
<tr><td><code>kind</code></td><td><code>str</code></td><td><code>required</code></td><td>Type de modèle (ex. `ridge`, `random_forest`).</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `ModelRecord` sauvegardé (id, name, version, created_at).

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, json, numpy as np
X = np.random.randn(100, 3)
y = X[:, 0] * 2 + np.random.randn(100) * 0.3
model = sp.Ridge(alpha=0.5)
model.fit(X, y)
res = json.loads(sp.ml_save_model(json.dumps({"name": "my_ridge", "kind": "ridge"})))
print(res["version"])
```

</div>

</div>

---

<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>LoadModel</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-util">Utility</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">📦 Registry</span>
      </div>
      <p class="ml-pg-tagline">Load a model from the registry by name and optional version. / Charger un modèle depuis le registre par nom et version optionnelle.</p>
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
import seraplot as sp, json
res = json.loads(sp.ml_load_model(json.dumps({"name": "my_ridge"})))
print(res)
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.LoadModel</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_load_model` — aliases: `load_model`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.LoadModel(name=required, version=null)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>name</code></td><td><code>str</code></td><td><code>required</code></td><td>Model name.</td></tr>
<tr><td><code>version</code></td><td><code>int</code></td><td><code>null</code></td><td>Version to load (latest if omitted).</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `ModelRecord` or `null` if not found.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, json
res = json.loads(sp.ml_load_model(json.dumps({"name": "my_ridge"})))
print(res)
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_load_model` — alias : `load_model`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.LoadModel(name=required, version=null)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>name</code></td><td><code>str</code></td><td><code>required</code></td><td>Nom du modèle.</td></tr>
<tr><td><code>version</code></td><td><code>int</code></td><td><code>null</code></td><td>Version à charger (dernière si omis).</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `ModelRecord` ou `null` si non trouvé.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, json
res = json.loads(sp.ml_load_model(json.dumps({"name": "my_ridge"})))
print(res)
```

</div>

</div>
