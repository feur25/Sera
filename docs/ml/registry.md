<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title">Model Registry</h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-new">New in 2.4.34</span>
        <span class="ml-pg-tag ml-pg-tag-trx">Persistence</span>
      </div>
      <p class="ml-pg-tagline">Versioned model store — save, tag, promote and reload any trained estimator. Persists to <code>~/.seraplot/registry/</code>. / Registre de modèles versionné — sauvegarder, tagger, promouvoir et recharger tout estimateur entraîné.</p>
    </div>
    <div class="ml-pg-badges">
      <span class="ml-pg-badge ml-pg-badge-speed-hi">✓ local persist</span>
      <span class="ml-pg-badge ml-pg-badge-parity-hi">✓ JSON index</span>
    </div>
  </div>
  <div class="ml-pg-stats">
    <div class="ml-pg-stat">
      <span class="ml-pg-stat-lbl">Versions / model</span>
      <span class="ml-pg-stat-val ml-pg-stat-val-green">∞</span>
      <span class="ml-pg-stat-sub">auto-incremented</span>
    </div>
    <div class="ml-pg-stat">
      <span class="ml-pg-stat-lbl">Storage</span>
      <span class="ml-pg-stat-val ml-pg-stat-val-blue">JSON</span>
      <span class="ml-pg-stat-sub">~/.seraplot/registry/</span>
    </div>
    <div class="ml-pg-stat">
      <span class="ml-pg-stat-lbl">Tags / promote</span>
      <span class="ml-pg-stat-val ml-pg-stat-val-purple">✓</span>
      <span class="ml-pg-stat-sub">staging, prod, champion…</span>
    </div>
  </div>
</div>

<div class="ml-pg-feature-grid">
  <div class="ml-pg-feature-card">
    <div class="ml-pg-feature-card-icon">📦</div>
    <div class="ml-pg-feature-card-title">Versioning</div>
    <p class="ml-pg-feature-card-desc">Each call to <code>registry_register</code> creates a new version automatically. Get the latest or a specific version.</p>
  </div>
  <div class="ml-pg-feature-card">
    <div class="ml-pg-feature-card-icon">🏷️</div>
    <div class="ml-pg-feature-card-title">Tags & Promotion</div>
    <p class="ml-pg-feature-card-desc">Promote any version to a named tag (<code>prod</code>, <code>champion</code>, <code>staging</code>) and retrieve by tag.</p>
  </div>
  <div class="ml-pg-feature-card">
    <div class="ml-pg-feature-card-icon">📊</div>
    <div class="ml-pg-feature-card-title">Metrics</div>
    <p class="ml-pg-feature-card-desc">Attach any float metrics (<code>accuracy</code>, <code>rmse</code>, …) and params (<code>alpha</code>, <code>max_depth</code>, …) to each record.</p>
  </div>
  <div class="ml-pg-feature-card">
    <div class="ml-pg-feature-card-icon">💾</div>
    <div class="ml-pg-feature-card-title">Payload</div>
    <p class="ml-pg-feature-card-desc">Store any JSON-serialisable model payload (weights, config, pickle path). No size limit enforced.</p>
  </div>
</div>

<div class="ml-pg-qs">
  <div class="ml-pg-qs-header"><span class="ml-pg-qs-title">Quick start — EN</span></div>

```python
import seraplot as sp, json, numpy as np

X = np.random.randn(500, 5)
y = (X[:, 0] > 0).astype(int)

model = sp.RandomForestClassifier(n_estimators=50)
model.fit(X, y)
acc = model.score(X, y)

rec = sp.registry_register(
    name    = "fraud_detector",
    kind    = "classifier",
    payload = json.dumps({"n_estimators": 50}),
    params  = {"n_estimators": "50", "max_depth": "None"},
    metrics = {"accuracy": acc, "n_train": 500.0},
    tags    = ["baseline"],
)
print(f"Saved v{rec['version']}  acc={rec['metrics']['accuracy']:.4f}")

sp.registry_promote("fraud_detector", rec["version"], "prod")

champion = sp.registry_by_tag("fraud_detector", "prod")
print(f"Champion: v{champion['version']}  tags={champion['tags']}")

for entry in sp.registry_list():
    print(f"  {entry['name']} v{entry['version']}  {entry['tags']}")
```

</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">Functions</div>
<table class="ml-pg-table">
<thead><tr><th>Function</th><th>Signature</th><th>Returns</th></tr></thead>
<tbody>
<tr><td><code>registry_register</code></td><td><code>(name, kind, payload, params={}, metrics={}, tags=[])</code></td><td><span class="ml-pg-type ml-pg-type-none">dict</span></td></tr>
<tr><td><code>registry_get</code></td><td><code>(name, version=None)</code></td><td><span class="ml-pg-type ml-pg-type-none">dict | None</span></td></tr>
<tr><td><code>registry_list</code></td><td><code>()</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[dict]</span></td></tr>
<tr><td><code>registry_delete</code></td><td><code>(name, version=None)</code></td><td><span class="ml-pg-type ml-pg-type-int">int — deleted count</span></td></tr>
<tr><td><code>registry_promote</code></td><td><code>(name, version, tag)</code></td><td><span class="ml-pg-type ml-pg-type-bool">bool</span></td></tr>
<tr><td><code>registry_by_tag</code></td><td><code>(name, tag)</code></td><td><span class="ml-pg-type ml-pg-type-none">dict | None</span></td></tr>
<tr><td><code>registry_clear</code></td><td><code>()</code></td><td><span class="ml-pg-type ml-pg-type-int">int — deleted count</span></td></tr>
<tr><td><code>registry_dir</code></td><td><code>()</code></td><td><span class="ml-pg-type ml-pg-type-str">str — path</span></td></tr>
</tbody>
</table>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Record schema (returned dict)</div>
<table class="ml-pg-table">
<thead><tr><th>Field</th><th>Type</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>name</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td>Model name (lookup key)</td></tr>
<tr><td><code>version</code></td><td><span class="ml-pg-type ml-pg-type-int">int</span></td><td>Auto-incremented version number</td></tr>
<tr><td><code>kind</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td>Free-form type label (<code>"classifier"</code>, <code>"regressor"</code>, …)</td></tr>
<tr><td><code>created_ms</code></td><td><span class="ml-pg-type ml-pg-type-int">int</span></td><td>Unix timestamp in milliseconds</td></tr>
<tr><td><code>payload</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td>JSON string — model weights, pickle path, config</td></tr>
<tr><td><code>params</code></td><td><span class="ml-pg-type ml-pg-type-none">dict[str,str]</span></td><td>Hyperparameters (string values)</td></tr>
<tr><td><code>metrics</code></td><td><span class="ml-pg-type ml-pg-type-none">dict[str,float]</span></td><td>Evaluation metrics</td></tr>
<tr><td><code>tags</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[str]</span></td><td>Named tags — use <code>registry_promote</code> to add</td></tr>
</tbody>
</table>
</div>

<div class="ml-pg-note ml-pg-note-info">
  <span class="ml-pg-note-icon">ℹ️</span>
  <div>The registry persists to <strong><code>~/.seraplot/registry/index.json</code></strong>. Override the location with the environment variable <code>SERAPLOT_REGISTRY_DIR</code>.</div>
</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Fonctions</div>
<table class="ml-pg-table">
<thead><tr><th>Fonction</th><th>Signature</th><th>Retourne</th></tr></thead>
<tbody>
<tr><td><code>registry_register</code></td><td><code>(name, kind, payload, params={}, metrics={}, tags=[])</code></td><td><span class="ml-pg-type ml-pg-type-none">dict</span></td></tr>
<tr><td><code>registry_get</code></td><td><code>(name, version=None)</code></td><td><span class="ml-pg-type ml-pg-type-none">dict | None</span></td></tr>
<tr><td><code>registry_list</code></td><td><code>()</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[dict]</span></td></tr>
<tr><td><code>registry_delete</code></td><td><code>(name, version=None)</code></td><td><span class="ml-pg-type ml-pg-type-int">int — entrées supprimées</span></td></tr>
<tr><td><code>registry_promote</code></td><td><code>(name, version, tag)</code></td><td><span class="ml-pg-type ml-pg-type-bool">bool</span></td></tr>
<tr><td><code>registry_by_tag</code></td><td><code>(name, tag)</code></td><td><span class="ml-pg-type ml-pg-type-none">dict | None</span></td></tr>
<tr><td><code>registry_clear</code></td><td><code>()</code></td><td><span class="ml-pg-type ml-pg-type-int">int — entrées supprimées</span></td></tr>
<tr><td><code>registry_dir</code></td><td><code>()</code></td><td><span class="ml-pg-type ml-pg-type-str">str — chemin</span></td></tr>
</tbody>
</table>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Schéma d'un enregistrement (dict retourné)</div>
<table class="ml-pg-table">
<thead><tr><th>Champ</th><th>Type</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>name</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td>Nom du modèle (clé de recherche)</td></tr>
<tr><td><code>version</code></td><td><span class="ml-pg-type ml-pg-type-int">int</span></td><td>Numéro de version auto-incrémenté</td></tr>
<tr><td><code>kind</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td>Libellé libre (<code>"classifier"</code>, <code>"regressor"</code>, …)</td></tr>
<tr><td><code>created_ms</code></td><td><span class="ml-pg-type ml-pg-type-int">int</span></td><td>Timestamp Unix en millisecondes</td></tr>
<tr><td><code>payload</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td>JSON — poids du modèle, chemin pickle, config</td></tr>
<tr><td><code>params</code></td><td><span class="ml-pg-type ml-pg-type-none">dict[str,str]</span></td><td>Hyperparamètres (valeurs en chaîne)</td></tr>
<tr><td><code>metrics</code></td><td><span class="ml-pg-type ml-pg-type-none">dict[str,float]</span></td><td>Métriques d'évaluation</td></tr>
<tr><td><code>tags</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[str]</span></td><td>Tags nommés — utiliser <code>registry_promote</code> pour ajouter</td></tr>
</tbody>
</table>
</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div>Démarrage rapide : enregistrez vos modèles avec <code>registry_register</code>, promouvez en production avec <code>registry_promote(name, version, "prod")</code>, récupérez en <code>registry_by_tag(name, "prod")</code>.<br>
  Le registre persiste dans <strong><code>~/.seraplot/registry/index.json</code></strong>. Modifiez l'emplacement via la variable d'environnement <code>SERAPLOT_REGISTRY_DIR</code>.</div>
</div>

</div>
