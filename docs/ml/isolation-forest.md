<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title"><code>IsolationForest</code></h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-anom">Detector</span>
        <span class="ml-pg-tag ml-pg-tag-trx">sklearn-compatible</span>
        <span class="ml-pg-tag ml-pg-tag-cat">🚨 Anomaly Detection</span>
      </div>
      <p class="ml-pg-tagline">IsolationForest — anomaly detection via random partitioning trees. / IsolationForest — détection d'anomalies via arbres de partition aléatoire.</p>
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
X[0] = [10, 10, 10, 10]
iso = sp.IsolationForest(n_estimators=100, contamination=0.05)
iso.fit(X)
print(iso.predict(X[:5]))
```

</div>

<div class="ml-pg-note ml-pg-note-tip">
  <span class="ml-pg-note-icon">💡</span>
  <div><strong>EN</strong> — Drop-in replacement: <code>sp.IsolationForest</code> has the same API as sklearn.<br><strong>FR</strong> — Remplacement direct : même API que sklearn, changez l'import.</div>
</div>

---

<div class="lang-en">

## API Reference

<div class="ml-pg-section">
<div class="ml-pg-section-title">JSON function name</div>

`ml_isolation_forest` — aliases: `isolation_forest`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Python class</div>

```python
sp.IsolationForest(n_estimators=100, max_samples=256, contamination=0.1, seed=42)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Constructor Parameters</div>

<table class="ml-pg-table">
<thead><tr><th>Parameter</th><th>Type</th><th>Default</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_estimators</code></td><td><code>int</code></td><td><code>100</code></td><td>Number of isolation trees.</td></tr>
<tr><td><code>max_samples</code></td><td><code>int</code></td><td><code>256</code></td><td>Max samples per tree.</td></tr>
<tr><td><code>contamination</code></td><td><code>float</code></td><td><code>0.1</code></td><td>Expected fraction of outliers.</td></tr>
<tr><td><code>seed</code></td><td><code>int</code></td><td><code>42</code></td><td>Random seed.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Returns</div>

JSON with `labels` (1 = inlier, −1 = outlier), `scores`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithm</div>

$$s(x) = 2^{-E[h(x)]/c(n)}, \quad c(n) = 2H_{n-1} - \frac{2(n-1)}{n}$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Example</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(300, 4)
X[0] = [10, 10, 10, 10]
iso = sp.IsolationForest(n_estimators=100, contamination=0.05)
iso.fit(X)
print(iso.predict(X[:5]))
```

</div>

</div>

---

<div class="lang-fr">

## Référence API

<div class="ml-pg-section">
<div class="ml-pg-section-title">Nom de fonction JSON</div>

`ml_isolation_forest` — alias : `isolation_forest`

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Classe Python</div>

```python
sp.IsolationForest(n_estimators=100, max_samples=256, contamination=0.1, seed=42)
```

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Paramètres du constructeur</div>

<table class="ml-pg-table">
<thead><tr><th>Paramètre</th><th>Type</th><th>Défaut</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>n_estimators</code></td><td><code>int</code></td><td><code>100</code></td><td>Nombre d'arbres d'isolation.</td></tr>
<tr><td><code>max_samples</code></td><td><code>int</code></td><td><code>256</code></td><td>Max échantillons par arbre.</td></tr>
<tr><td><code>contamination</code></td><td><code>float</code></td><td><code>0.1</code></td><td>Fraction attendue de valeurs aberrantes.</td></tr>
<tr><td><code>seed</code></td><td><code>int</code></td><td><code>42</code></td><td>Graine aléatoire.</td></tr>
</tbody>
</table>

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Retourne</div>

JSON avec `labels` (1 = normal, −1 = anomalie), `scores`.

</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Algorithme</div>

$$s(x) = 2^{-E[h(x)]/c(n)}, \quad c(n) = 2H_{n-1} - \frac{2(n-1)}{n}$$

</div>
<div class="ml-pg-section">
<div class="ml-pg-section-title">Exemple</div>

```python
import seraplot as sp, numpy as np
X = np.random.randn(300, 4)
X[0] = [10, 10, 10, 10]
iso = sp.IsolationForest(n_estimators=100, contamination=0.05)
iso.fit(X)
print(iso.predict(X[:5]))
```

</div>

</div>
