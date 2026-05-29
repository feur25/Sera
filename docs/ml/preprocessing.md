<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>StandardScaler</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-trx">Transformer</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">🔧 Preprocessing</span>
      </div>
      <p class="ml-pg-tagline">StandardScaler — zero-mean unit-variance standardisation. / StandardScaler — standardisation à moyenne nulle et variance unitaire.</p>
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
X = np.random.randn(200, 4) * [3, 0.5, 10, 1]
scaler = sp.StandardScaler()
scaler.fit(X)
Xt = scaler.transform(X)
print(Xt.mean(0), Xt.std(0))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.StandardScaler</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_standard_scaler` — aliases: `standard_scaler`, `standardize`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.StandardScaler()
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<p><em>No constructor parameters.</em></p>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `transformed` matrix.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$x' = \frac{x - \mu}{\sigma}$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(200, 4) * [3, 0.5, 10, 1]
scaler = sp.StandardScaler()
scaler.fit(X)
Xt = scaler.transform(X)
print(Xt.mean(0), Xt.std(0))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_standard_scaler` — alias : `standard_scaler`, `standardize`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.StandardScaler()
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<p><em>Aucun paramètre de constructeur.</em></p>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec matrice `transformed`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$x' = \frac{x - \mu}{\sigma}$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(200, 4) * [3, 0.5, 10, 1]
scaler = sp.StandardScaler()
scaler.fit(X)
Xt = scaler.transform(X)
print(Xt.mean(0), Xt.std(0))
```

</div>

</div>

---

<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>MinmaxScaler</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-trx">Transformer</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">🔧 Preprocessing</span>
      </div>
      <p class="ml-pg-tagline">MinMaxScaler — scale features to [0, 1] range. / MinMaxScaler — mise à l'échelle des features dans l'intervalle [0, 1].</p>
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
X = np.random.randn(200, 3) * 100
scaler = sp.MinMaxScaler()
Xt = scaler.fit_transform(X)
print(Xt.min(), Xt.max())
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.MinmaxScaler</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_minmax_scaler` — aliases: `minmax_scaler`, `min_max_scaler`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.MinmaxScaler()
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<p><em>No constructor parameters.</em></p>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `transformed` matrix.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$x' = \frac{x - x_{\min}}{x_{\max} - x_{\min}}$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(200, 3) * 100
scaler = sp.MinMaxScaler()
Xt = scaler.fit_transform(X)
print(Xt.min(), Xt.max())
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_minmax_scaler` — alias : `minmax_scaler`, `min_max_scaler`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.MinmaxScaler()
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<p><em>Aucun paramètre de constructeur.</em></p>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec matrice `transformed`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$x' = \frac{x - x_{\min}}{x_{\max} - x_{\min}}$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(200, 3) * 100
scaler = sp.MinMaxScaler()
Xt = scaler.fit_transform(X)
print(Xt.min(), Xt.max())
```

</div>

</div>

---

<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>RobustScaler</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-trx">Transformer</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">🔧 Preprocessing</span>
      </div>
      <p class="ml-pg-tagline">RobustScaler — scale using median and IQR, robust to outliers. / RobustScaler — mise à l'échelle par médiane et IQR, robuste aux valeurs aberrantes.</p>
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
X = np.random.randn(300, 4)
X[0, :] = 100
scaler = sp.RobustScaler()
Xt = scaler.fit_transform(X)
print(Xt.mean(0))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.RobustScaler</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_robust_scaler` — aliases: `robust_scaler`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.RobustScaler()
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<p><em>No constructor parameters.</em></p>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `transformed` matrix.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$x' = \frac{x - \text{median}}{\text{IQR}}$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(300, 4)
X[0, :] = 100
scaler = sp.RobustScaler()
Xt = scaler.fit_transform(X)
print(Xt.mean(0))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_robust_scaler` — alias : `robust_scaler`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.RobustScaler()
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<p><em>Aucun paramètre de constructeur.</em></p>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec matrice `transformed`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$x' = \frac{x - \text{médiane}}{\text{IQR}}$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(300, 4)
X[0, :] = 100
scaler = sp.RobustScaler()
Xt = scaler.fit_transform(X)
print(Xt.mean(0))
```

</div>

</div>

---

<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>FitTransform</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-trx">Transformer</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">🔧 Preprocessing</span>
      </div>
      <p class="ml-pg-tagline">Unified fit_transform — dispatches to StandardScaler/MinMaxScaler/RobustScaler by name. / fit_transform unifié — dispatche vers StandardScaler/MinMaxScaler/RobustScaler par nom.</p>
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
import json
X = np.random.randn(100, 3) * 5
res = json.loads(sp.ml_fit_transform(json.dumps({"X_train": X.tolist(), "scaler": "minmax"})))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.FitTransform</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_fit_transform` — aliases: `fit_transform`, `preprocess`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.FitTransform(scaler=standard)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>scaler</code></td><td><code>str</code></td><td><code>standard</code></td><td>Scaler type: `standard`, `minmax`, `robust`.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `transformed` matrix.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
import json
X = np.random.randn(100, 3) * 5
res = json.loads(sp.ml_fit_transform(json.dumps({"X_train": X.tolist(), "scaler": "minmax"})))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_fit_transform` — alias : `fit_transform`, `preprocess`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.FitTransform(scaler=standard)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>scaler</code></td><td><code>str</code></td><td><code>standard</code></td><td>Type de scaler : `standard`, `minmax`, `robust`.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec matrice `transformed`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
import json
X = np.random.randn(100, 3) * 5
res = json.loads(sp.ml_fit_transform(json.dumps({"X_train": X.tolist(), "scaler": "minmax"})))
```

</div>

</div>
