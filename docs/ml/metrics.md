<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>MetricScore</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-util">Utility</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">📏 Metrics</span>
      </div>
      <p class="ml-pg-tagline">Compute a named metric score (accuracy, r2, f1, etc.) from predictions. / Calcule un score de métrique nommée (accuracy, r2, f1, etc.) à partir des prédictions.</p>
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
payload = {"y_true": [0,1,1,0,1], "y_pred": [0,1,0,0,1], "metric": "accuracy"}
res = json.loads(sp.ml_metric_score(json.dumps(payload)))
print(res["score"])
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.MetricScore</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_metric_score` — aliases: `metric_score`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.MetricScore(metric=accuracy)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>metric</code></td><td><code>str</code></td><td><code>accuracy</code></td><td>Metric name: `accuracy`, `r2`, `f1`, `precision`, `recall`, `mse`, `mae`.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `score`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, json
payload = {"y_true": [0,1,1,0,1], "y_pred": [0,1,0,0,1], "metric": "accuracy"}
res = json.loads(sp.ml_metric_score(json.dumps(payload)))
print(res["score"])
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_metric_score` — alias : `metric_score`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.MetricScore(metric=accuracy)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>metric</code></td><td><code>str</code></td><td><code>accuracy</code></td><td>Nom de la métrique : `accuracy`, `r2`, `f1`, `precision`, `recall`, `mse`, `mae`.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `score`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, json
payload = {"y_true": [0,1,1,0,1], "y_pred": [0,1,0,0,1], "metric": "accuracy"}
res = json.loads(sp.ml_metric_score(json.dumps(payload)))
print(res["score"])
```

</div>

</div>

---

<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>MetricCurve</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-util">Utility</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">📏 Metrics</span>
      </div>
      <p class="ml-pg-tagline">Compute a metric curve (ROC, PR, confusion matrix) from predictions. / Calcule une courbe de métrique (ROC, PR, matrice de confusion) à partir des prédictions.</p>
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
import numpy as np
y_true = (np.random.randn(100) > 0).astype(int).tolist()
y_score = np.random.rand(100).tolist()
res = json.loads(sp.ml_metric_curve(json.dumps({"y_true": y_true, "y_score": y_score, "curve": "roc"})))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.MetricCurve</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_metric_curve` — aliases: `metric_curve`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.MetricCurve(curve=roc)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>curve</code></td><td><code>str</code></td><td><code>roc</code></td><td>Curve type: `roc`, `pr`, `confusion`.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with curve data points.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, json
import numpy as np
y_true = (np.random.randn(100) > 0).astype(int).tolist()
y_score = np.random.rand(100).tolist()
res = json.loads(sp.ml_metric_curve(json.dumps({"y_true": y_true, "y_score": y_score, "curve": "roc"})))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_metric_curve` — alias : `metric_curve`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.MetricCurve(curve=roc)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>curve</code></td><td><code>str</code></td><td><code>roc</code></td><td>Type de courbe : `roc`, `pr`, `confusion`.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec les points de la courbe.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, json
import numpy as np
y_true = (np.random.randn(100) > 0).astype(int).tolist()
y_score = np.random.rand(100).tolist()
res = json.loads(sp.ml_metric_curve(json.dumps({"y_true": y_true, "y_score": y_score, "curve": "roc"})))
```

</div>

</div>
