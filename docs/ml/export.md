<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title">PowerBI &amp; Tableau Export</h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-new">New in 2.4.34</span>
        <span class="ml-pg-tag ml-pg-tag-trx">Export</span>
      </div>
      <p class="ml-pg-tagline">Export predictions and feature matrices directly to PowerBI Push dataset JSON or Tableau TDS/CSV — native format, no extra tooling. / Export les prédictions et matrices de features en JSON PowerBI ou TDS/CSV Tableau.</p>
    </div>
    <div class="ml-pg-badges">
      <span class="ml-pg-badge ml-pg-badge-speed-hi">✓ PowerBI JSON</span>
      <span class="ml-pg-badge ml-pg-badge-parity-hi">✓ Tableau TDS + CSV</span>
    </div>
  </div>
  <div class="ml-pg-stats">
    <div class="ml-pg-stat">
      <span class="ml-pg-stat-lbl">PowerBI format</span>
      <span class="ml-pg-stat-val ml-pg-stat-val-green">Push</span>
      <span class="ml-pg-stat-sub">dataset API compatible</span>
    </div>
    <div class="ml-pg-stat">
      <span class="ml-pg-stat-lbl">Tableau format</span>
      <span class="ml-pg-stat-val ml-pg-stat-val-blue">TDS + CSV</span>
      <span class="ml-pg-stat-sub">data source + extract</span>
    </div>
    <div class="ml-pg-stat">
      <span class="ml-pg-stat-lbl">Columns</span>
      <span class="ml-pg-stat-val ml-pg-stat-val-purple">X + y + ŷ</span>
      <span class="ml-pg-stat-sub">feature, target, pred</span>
    </div>
  </div>
</div>

<div class="ml-pg-feature-grid">
  <div class="ml-pg-feature-card">
    <div class="ml-pg-feature-card-icon">📊</div>
    <div class="ml-pg-feature-card-title">export_powerbi</div>
    <p class="ml-pg-feature-card-desc">Returns a JSON string representing a PowerBI Push dataset. Paste into the PowerBI REST API or save to <code>.json</code>.</p>
  </div>
  <div class="ml-pg-feature-card">
    <div class="ml-pg-feature-card-icon">📋</div>
    <div class="ml-pg-feature-card-title">export_tableau_tds</div>
    <p class="ml-pg-feature-card-desc">Returns a Tableau Data Source XML (<code>.tds</code>). Defines all columns — open in Tableau Desktop to connect instantly.</p>
  </div>
  <div class="ml-pg-feature-card">
    <div class="ml-pg-feature-card-icon">📁</div>
    <div class="ml-pg-feature-card-title">export_tableau_csv</div>
    <p class="ml-pg-feature-card-desc">Returns a CSV string with a header row. Includes feature columns, optional target and optional predictions.</p>
  </div>
  <div class="ml-pg-feature-card">
    <div class="ml-pg-feature-card-icon">🔗</div>
    <div class="ml-pg-feature-card-title">Combine with Registry</div>
    <p class="ml-pg-feature-card-desc">Export predictions from a registry-loaded model payload and share reproducible exports alongside versioned models.</p>
  </div>
</div>

<div class="ml-pg-qs">
  <div class="ml-pg-qs-header"><span class="ml-pg-qs-title">Quick start</span></div>

```python
import seraplot as sp, numpy as np

X = np.random.randn(200, 3)
y = X[:, 0] * 2 + np.random.randn(200) * 0.1

model = sp.LinearRegression()
model.fit(X, y)
yhat = model.predict(X)

pbi_json = sp.export_powerbi(
    name   = "HousePrice",
    table  = "Predictions",
    X      = X,
    y      = list(y),
    y_pred = list(yhat),
)
with open("powerbi_dataset.json", "w") as f:
    f.write(pbi_json)

tds_xml = sp.export_tableau_tds(
    name   = "HousePrice",
    X      = X,
    y      = list(y),
    y_pred = list(yhat),
)
with open("house_price.tds", "w") as f:
    f.write(tds_xml)

csv_str = sp.export_tableau_csv(
    name   = "HousePrice",
    X      = X,
    y      = list(y),
    y_pred = list(yhat),
)
with open("house_price.csv", "w") as f:
    f.write(csv_str)
```

</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">export_powerbi</div>
<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>name</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td>—</td><td>Dataset name (appears in PowerBI)</td></tr>
<tr><td><code>table</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td>—</td><td>Table name inside the dataset</td></tr>
<tr><td><code>X</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[list[float]]</span></td><td>—</td><td>Feature matrix, shape <em>(n, p)</em></td></tr>
<tr><td><code>y</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[float] | None</span></td><td><span class="ml-pg-default">None</span></td><td>Target values (optional)</td></tr>
<tr><td><code>y_pred</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[float] | None</span></td><td><span class="ml-pg-default">None</span></td><td>Predicted values (optional)</td></tr>
</tbody>
</table>
<div class="ml-pg-ret">Returns → <code>str</code> — PowerBI Push dataset JSON</div>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">export_tableau_tds</div>
<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>name</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td>—</td><td>Data source name in Tableau</td></tr>
<tr><td><code>X</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[list[float]]</span></td><td>—</td><td>Feature matrix</td></tr>
<tr><td><code>y</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[float] | None</span></td><td><span class="ml-pg-default">None</span></td><td>Target values (optional)</td></tr>
<tr><td><code>y_pred</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[float] | None</span></td><td><span class="ml-pg-default">None</span></td><td>Predicted values (optional)</td></tr>
</tbody>
</table>
<div class="ml-pg-ret">Returns → <code>str</code> — Tableau TDS XML</div>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">export_tableau_csv</div>
<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>name</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td>—</td><td>Used as comment in header row</td></tr>
<tr><td><code>X</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[list[float]]</span></td><td>—</td><td>Feature matrix</td></tr>
<tr><td><code>y</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[float] | None</span></td><td><span class="ml-pg-default">None</span></td><td>Target values (optional)</td></tr>
<tr><td><code>y_pred</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[float] | None</span></td><td><span class="ml-pg-default">None</span></td><td>Predicted values (optional)</td></tr>
</tbody>
</table>
<div class="ml-pg-ret">Returns → <code>str</code> — CSV with header row (<code>feat_0, feat_1, …, target, prediction</code>)</div>
</div>

<div class="ml-pg-note ml-pg-note-info">
  <span class="ml-pg-note-icon">ℹ️</span>
  <div>Column names are auto-generated: <code>feat_0</code> … <code>feat_{p-1}</code>, <code>target</code>, <code>prediction</code>. All functions return strings — write them to disk or POST them to the relevant API.</div>
</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">export_powerbi</div>
<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>name</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td>—</td><td>Nom du dataset (visible dans PowerBI)</td></tr>
<tr><td><code>table</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td>—</td><td>Nom de la table dans le dataset</td></tr>
<tr><td><code>X</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[list[float]]</span></td><td>—</td><td>Matrice de features, forme <em>(n, p)</em></td></tr>
<tr><td><code>y</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[float] | None</span></td><td><span class="ml-pg-default">None</span></td><td>Valeurs cibles (optionnel)</td></tr>
<tr><td><code>y_pred</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[float] | None</span></td><td><span class="ml-pg-default">None</span></td><td>Prédictions (optionnel)</td></tr>
</tbody>
</table>
<div class="ml-pg-ret">Retourne → <code>str</code> — JSON Push dataset PowerBI</div>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">export_tableau_tds</div>
<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>name</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td>—</td><td>Nom de la source de données dans Tableau</td></tr>
<tr><td><code>X</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[list[float]]</span></td><td>—</td><td>Matrice de features</td></tr>
<tr><td><code>y</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[float] | None</span></td><td><span class="ml-pg-default">None</span></td><td>Valeurs cibles (optionnel)</td></tr>
<tr><td><code>y_pred</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[float] | None</span></td><td><span class="ml-pg-default">None</span></td><td>Prédictions (optionnel)</td></tr>
</tbody>
</table>
<div class="ml-pg-ret">Retourne → <code>str</code> — XML Tableau TDS</div>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">export_tableau_csv</div>
<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>name</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td>—</td><td>Utilisé en commentaire dans la ligne d'en-tête</td></tr>
<tr><td><code>X</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[list[float]]</span></td><td>—</td><td>Matrice de features</td></tr>
<tr><td><code>y</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[float] | None</span></td><td><span class="ml-pg-default">None</span></td><td>Valeurs cibles (optionnel)</td></tr>
<tr><td><code>y_pred</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[float] | None</span></td><td><span class="ml-pg-default">None</span></td><td>Prédictions (optionnel)</td></tr>
</tbody>
</table>
<div class="ml-pg-ret">Retourne → <code>str</code> — CSV avec en-tête (<code>feat_0, feat_1, …, target, prediction</code>)</div>
</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div>Les colonnes sont nommées automatiquement : <code>feat_0</code> … <code>feat_{p-1}</code>, <code>target</code>, <code>prediction</code>. Toutes les fonctions retournent des chaînes de caractères — écrivez-les sur disque ou envoyez-les à l'API concernée via HTTP POST.</div>
</div>

</div>
