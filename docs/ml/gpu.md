<div class="ml-pg-header">
  <div class="ml-pg-header-top">
    <div class="ml-pg-title-group">
      <h1 class="ml-pg-title">GPU Backend</h1>
      <div class="ml-pg-tags">
        <span class="ml-pg-tag ml-pg-tag-new">New in 2.4.34</span>
        <span class="ml-pg-tag ml-pg-tag-gpu">GPU</span>
      </div>
      <p class="ml-pg-tagline">Backend detection and selection for CUDA, Metal (Apple Silicon), ROCm and CPU. Auto-detection at runtime with zero configuration. / Détection et sélection du backend GPU/CPU — CUDA, Metal, ROCm détectés automatiquement.</p>
    </div>
    <div class="ml-pg-badges">
      <span class="ml-pg-badge ml-pg-badge-speed-hi">✓ CUDA detected</span>
      <span class="ml-pg-badge ml-pg-badge-parity-hi">✓ CPU always available</span>
    </div>
  </div>
  <div class="ml-pg-stats">
    <div class="ml-pg-stat">
      <span class="ml-pg-stat-lbl">Backends</span>
      <span class="ml-pg-stat-val ml-pg-stat-val-green">4</span>
      <span class="ml-pg-stat-sub">CPU · CUDA · Metal · ROCm</span>
    </div>
    <div class="ml-pg-stat">
      <span class="ml-pg-stat-lbl">Detection</span>
      <span class="ml-pg-stat-val ml-pg-stat-val-blue">Auto</span>
      <span class="ml-pg-stat-sub">env-var + path probing</span>
    </div>
    <div class="ml-pg-stat">
      <span class="ml-pg-stat-lbl">Config</span>
      <span class="ml-pg-stat-val ml-pg-stat-val-purple">Zero</span>
      <span class="ml-pg-stat-sub">no setup required</span>
    </div>
  </div>
</div>

<div class="ml-pg-note ml-pg-note-warn">
  <span class="ml-pg-note-icon">⚠️</span>
  <div><strong>EN — Current state:</strong> The GPU backend layer is an abstraction and detection API. SeraPlot <em>currently trains on CPU</em> using rayon multi-threading. Full CUDA/candle kernel dispatch is planned for 2.5.x. This API lets you prepare your code and detect hardware now.<br>
  <strong>FR — État actuel :</strong> Le backend GPU est une couche d'abstraction et de détection. SeraPlot <em>entraîne actuellement sur CPU</em> avec rayon. Le dispatch complet CUDA/candle est prévu pour 2.5.x. Cette API vous permet de préparer votre code et détecter le matériel dès maintenant.</div>
</div>

<div class="ml-pg-feature-grid">
  <div class="ml-pg-feature-card">
    <div class="ml-pg-feature-card-icon">🔍</div>
    <div class="ml-pg-feature-card-title">gpu_devices</div>
    <p class="ml-pg-feature-card-desc">List all detected devices with backend, name, memory and availability flag.</p>
  </div>
  <div class="ml-pg-feature-card">
    <div class="ml-pg-feature-card-icon">⚡</div>
    <div class="ml-pg-feature-card-title">gpu_set_backend</div>
    <p class="ml-pg-feature-card-desc">Explicitly select a backend. Pass <code>None</code> for auto-selection (picks the first available non-CPU backend).</p>
  </div>
  <div class="ml-pg-feature-card">
    <div class="ml-pg-feature-card-icon">📍</div>
    <div class="ml-pg-feature-card-title">gpu_active_backend</div>
    <p class="ml-pg-feature-card-desc">Returns the currently active backend name as a string.</p>
  </div>
  <div class="ml-pg-feature-card">
    <div class="ml-pg-feature-card-icon">🖥️</div>
    <div class="ml-pg-feature-card-title">Detection logic</div>
    <p class="ml-pg-feature-card-desc">CUDA: <code>CUDA_PATH</code> / <code>CUDA_HOME</code> env vars. Metal: macOS target. ROCm: <code>/opt/rocm</code> path probe.</p>
  </div>
</div>

<div class="ml-pg-qs">
  <div class="ml-pg-qs-header"><span class="ml-pg-qs-title">Quick start</span></div>

```python
import seraplot as sp

devices = sp.gpu_devices()
for d in devices:
    avail = "available" if d["available"] else "detected / unavailable"
    print(f"  [{d['backend']}] {d['name']}  {d['mem_mb']} MB  — {avail}")

active = sp.gpu_active_backend()
print(f"\nActive backend: {active}")

sp.gpu_set_backend("cpu")
print(f"Forced CPU: {sp.gpu_active_backend()}")

sp.gpu_set_backend(None)
print(f"Auto-select: {sp.gpu_active_backend()}")
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
<tr><td><code>gpu_devices</code></td><td><code>()</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[dict]</span></td></tr>
<tr><td><code>gpu_set_backend</code></td><td><code>(backend: str | None)</code></td><td><span class="ml-pg-type ml-pg-type-str">str — active backend</span></td></tr>
<tr><td><code>gpu_active_backend</code></td><td><code>()</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td></tr>
</tbody>
</table>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Device record schema</div>
<table class="ml-pg-table">
<thead><tr><th>Field</th><th>Type</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>backend</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td><code>"cpu"</code>, <code>"cuda"</code>, <code>"metal"</code>, <code>"rocm"</code></td></tr>
<tr><td><code>name</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td>Human-readable device name</td></tr>
<tr><td><code>mem_mb</code></td><td><span class="ml-pg-type ml-pg-type-int">int</span></td><td>Device memory in MB (0 for CPU)</td></tr>
<tr><td><code>available</code></td><td><span class="ml-pg-type ml-pg-type-bool">bool</span></td><td><code>True</code> if the device can be used for computation</td></tr>
</tbody>
</table>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Backend string values for gpu_set_backend</div>
<table class="ml-pg-table">
<thead><tr><th>Value</th><th>Platform</th><th>Detection method</th></tr></thead>
<tbody>
<tr><td><code>"cpu"</code></td><td>All</td><td>Always available</td></tr>
<tr><td><code>"cuda"</code></td><td>NVIDIA GPU</td><td><code>CUDA_PATH</code> or <code>CUDA_HOME</code> env var set</td></tr>
<tr><td><code>"metal"</code></td><td>Apple Silicon / macOS</td><td><code>cfg!(target_os = "macos")</code> at compile time</td></tr>
<tr><td><code>"rocm"</code></td><td>AMD GPU / Linux</td><td><code>/opt/rocm</code> directory exists</td></tr>
<tr><td><code>None</code></td><td>—</td><td>Auto-select: first non-CPU available backend</td></tr>
</tbody>
</table>
</div>

<div class="ml-pg-note ml-pg-note-info">
  <span class="ml-pg-note-icon">ℹ️</span>
  <div>Setting a backend does not automatically enable GPU kernels in the current version. It records your preference so that when kernel dispatch ships in 2.5.x, your code is already correct and ready.</div>
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
<tr><td><code>gpu_devices</code></td><td><code>()</code></td><td><span class="ml-pg-type ml-pg-type-arr">list[dict]</span></td></tr>
<tr><td><code>gpu_set_backend</code></td><td><code>(backend: str | None)</code></td><td><span class="ml-pg-type ml-pg-type-str">str — backend actif</span></td></tr>
<tr><td><code>gpu_active_backend</code></td><td><code>()</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td></tr>
</tbody>
</table>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Schéma d'un device</div>
<table class="ml-pg-table">
<thead><tr><th>Champ</th><th>Type</th><th>Description</th></tr></thead>
<tbody>
<tr><td><code>backend</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td><code>"cpu"</code>, <code>"cuda"</code>, <code>"metal"</code>, <code>"rocm"</code></td></tr>
<tr><td><code>name</code></td><td><span class="ml-pg-type ml-pg-type-str">str</span></td><td>Nom lisible du périphérique</td></tr>
<tr><td><code>mem_mb</code></td><td><span class="ml-pg-type ml-pg-type-int">int</span></td><td>Mémoire du périphérique en Mo (0 pour CPU)</td></tr>
<tr><td><code>available</code></td><td><span class="ml-pg-type ml-pg-type-bool">bool</span></td><td><code>True</code> si le périphérique peut être utilisé</td></tr>
</tbody>
</table>
</div>

<div class="ml-pg-section">
<div class="ml-pg-section-title">Valeurs de backend pour gpu_set_backend</div>
<table class="ml-pg-table">
<thead><tr><th>Valeur</th><th>Plateforme</th><th>Méthode de détection</th></tr></thead>
<tbody>
<tr><td><code>"cpu"</code></td><td>Toutes</td><td>Toujours disponible</td></tr>
<tr><td><code>"cuda"</code></td><td>GPU NVIDIA</td><td>Variable d'env <code>CUDA_PATH</code> ou <code>CUDA_HOME</code></td></tr>
<tr><td><code>"metal"</code></td><td>Apple Silicon / macOS</td><td><code>cfg!(target_os = "macos")</code> à la compilation</td></tr>
<tr><td><code>"rocm"</code></td><td>GPU AMD / Linux</td><td>Dossier <code>/opt/rocm</code> présent</td></tr>
<tr><td><code>None</code></td><td>—</td><td>Auto : premier backend non-CPU disponible</td></tr>
</tbody>
</table>
</div>

<div class="ml-pg-note ml-pg-note-info">
  <span class="ml-pg-note-icon">ℹ️</span>
  <div>Dans la version actuelle, définir un backend ne déclenche pas encore les kernels GPU. Cela enregistre votre préférence : quand le dispatch de kernels arrivera en 2.5.x, votre code sera déjà correct et prêt.</div>
</div>

</div>
