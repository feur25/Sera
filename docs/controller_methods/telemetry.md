# Telemetry

<style>
/* ── Telemetry page ─────────────────────────────────────────────────────── */
.tm-hero{margin:1.6em 0 2em;padding:28px 30px;border-radius:14px;background:linear-gradient(135deg,#0a0f1c 0%,#0f1f2e 60%,#0a2340 100%);border:1px solid rgba(56,189,248,.2);box-shadow:0 18px 50px -12px rgba(14,165,233,.15),inset 0 1px 0 rgba(255,255,255,.04);position:relative;overflow:hidden}
.tm-hero::before{content:"";position:absolute;top:-40%;right:-10%;width:60%;height:180%;background:radial-gradient(ellipse at center,rgba(56,189,248,.12) 0%,transparent 65%);pointer-events:none}
.tm-hero h2{margin:0 0 8px;font-size:22px;color:#f0f9ff;font-weight:700;letter-spacing:-.01em;border:none}
.tm-hero p{margin:0;color:#94a3b8;font-size:14px;line-height:1.6;max-width:64ch}
.tm-pills{display:flex;gap:8px;flex-wrap:wrap;margin-top:14px;position:relative;z-index:1}
.tm-pill{padding:4px 11px;background:rgba(14,165,233,.12);border:1px solid rgba(56,189,248,.25);border-radius:999px;font-size:11px;font-weight:600;color:#7dd3fc;letter-spacing:.04em}

/* ── Status badge ── */
.tm-status{display:inline-flex;align-items:center;gap:7px;padding:6px 14px;background:#0f172a;border:1px solid #1e293b;border-radius:7px;font-size:12px;color:#64748b;font-weight:600;margin:1.2em 0 .4em;user-select:none}
.tm-dot{width:8px;height:8px;border-radius:50%;background:#ef4444;box-shadow:0 0 6px #ef4444aa}
.tm-status.active .tm-dot{background:#22c55e;box-shadow:0 0 6px #22c55eaa}

/* ── Metrics table ── */
.tm-table-wrap{overflow-x:auto;margin:1em 0 1.6em;border-radius:10px;border:1px solid #1e293b;background:#080d18}
.tm-table{width:100%;border-collapse:collapse;font-size:12.5px}
.tm-table thead tr{background:#0d1520}
.tm-table th{padding:10px 14px;text-align:left;color:#475569;font-weight:700;font-size:11px;letter-spacing:.07em;text-transform:uppercase;border-bottom:1px solid #1e293b;white-space:nowrap}
.tm-table td{padding:9px 14px;color:#94a3b8;border-bottom:1px solid #0f1a27;vertical-align:top;line-height:1.5}
.tm-table tr:last-child td{border-bottom:none}
.tm-table tr:hover td{background:#0a1220}
.tm-table td:first-child code{background:#0f172a;border:1px solid #334155;color:#38bdf8;padding:2px 7px;border-radius:5px;font-size:11.5px;white-space:nowrap}
.tm-table td:nth-child(2){color:#64748b;font-size:11.5px;white-space:nowrap}
.tm-opt{background:rgba(251,146,60,.08);border:1px solid rgba(251,146,60,.2);color:#fb923c;font-size:10px;font-weight:700;padding:1px 6px;border-radius:4px;letter-spacing:.04em;margin-left:4px;vertical-align:middle}
.tm-core{background:rgba(99,102,241,.08);border:1px solid rgba(99,102,241,.2);color:#818cf8;font-size:10px;font-weight:700;padding:1px 6px;border-radius:4px;letter-spacing:.04em;margin-left:4px;vertical-align:middle}

/* ── Privacy list ── */
.tm-privacy{list-style:none;padding:0;margin:.8em 0 1.2em;display:grid;grid-template-columns:repeat(auto-fit,minmax(220px,1fr));gap:8px}
.tm-privacy li{display:flex;align-items:center;gap:9px;background:#080d18;border:1px solid #1e293b;border-radius:8px;padding:9px 13px;color:#64748b;font-size:13px}
.tm-privacy li::before{content:"✕";flex-shrink:0;width:20px;height:20px;border-radius:50%;background:rgba(239,68,68,.1);border:1px solid rgba(239,68,68,.2);color:#f87171;font-size:10px;font-weight:700;display:flex;align-items:center;justify-content:center;text-align:center;line-height:20px}

/* ── Steps ── */
.tm-steps{counter-reset:step;padding:0;margin:1em 0 1.6em;display:flex;flex-direction:column;gap:0}
.tm-step{display:flex;gap:14px;padding:14px 0;border-bottom:1px solid #0f1a27;align-items:flex-start}
.tm-step:last-child{border-bottom:none}
.tm-step-n{flex-shrink:0;width:28px;height:28px;border-radius:50%;background:linear-gradient(135deg,#0ea5e9,#0369a1);color:#fff;font-weight:700;font-size:13px;display:flex;align-items:center;justify-content:center;box-shadow:0 3px 10px -3px rgba(14,165,233,.5);margin-top:1px}
.tm-step-body{flex:1}
.tm-step-body strong{color:#e2e8f0;font-size:13.5px;display:block;margin-bottom:3px}
.tm-step-body span{color:#64748b;font-size:12.5px;line-height:1.55}

/* ── Code cell (scrollable source) ── */
.tm-source{border:1px solid #1e2d45;border-radius:12px;overflow:hidden;margin:1.4em 0 2em;background:#080c16}
.tm-source-hdr{display:flex;align-items:center;justify-content:space-between;padding:10px 16px;background:#0a1120;border-bottom:1px solid #1e2d45}
.tm-source-title{display:flex;align-items:center;gap:8px;font-size:12px;font-weight:700;color:#38bdf8;letter-spacing:.06em;text-transform:uppercase}
.tm-source-title svg{opacity:.7}
.tm-source-dots{display:flex;gap:5px}
.tm-source-dot{width:11px;height:11px;border-radius:50%}
.tm-scroll{max-height:460px;overflow-y:auto;overflow-x:auto;scrollbar-width:thin;scrollbar-color:#1e3a5f #080c16}
.tm-scroll::-webkit-scrollbar{width:6px;height:6px}
.tm-scroll::-webkit-scrollbar-track{background:#080c16}
.tm-scroll::-webkit-scrollbar-thumb{background:#1e3a5f;border-radius:3px}
.tm-scroll pre{margin:0;border-radius:0;padding:18px 20px;background:#080c16}
.tm-scroll pre code{font-size:12px;line-height:1.65;color:#cdd9e5;background:none;padding:0}

/* ── API call blocks ── */
.tm-api{margin:1em 0 1.6em}
.tm-api-call{background:#080d18;border:1px solid #1e293b;border-radius:8px;overflow:hidden;margin-bottom:10px}
.tm-api-call pre{margin:0;padding:14px 16px;background:transparent}
.tm-api-call pre code{font-size:12.5px;line-height:1.55}

/* ── Note box ── */
.tm-note{display:flex;align-items:flex-start;gap:10px;margin:1em 0;padding:12px 16px;background:rgba(14,165,233,.05);border:1px solid rgba(14,165,233,.2);border-radius:8px;font-size:13px;color:#94a3b8;line-height:1.55}
.tm-note-ico{flex-shrink:0;font-size:16px;margin-top:1px}
.tm-note strong{color:#7dd3fc}

/* tabs reuse from global */
.sp-tabs{border:1px solid #1e2d45;border-radius:8px;overflow:hidden;margin:1em 0 1.6em}
.sp-tab-btns{display:flex;background:#0a1120;border-bottom:1px solid #1e2d45}
.sp-tb{padding:9px 18px;border:none;background:none;color:#475569;cursor:pointer;font-size:12.5px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:#e2e8f0}
.sp-tb.sp-act{color:#38bdf8;border-bottom-color:#38bdf8}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}
.sp-tc pre{margin:0;border-radius:0;padding:14px 16px;overflow-x:auto;background:#080c16}
.sp-tc pre code{font-size:12.5px;line-height:1.55;color:#cbd5e1;background:none;padding:0}
</style>

<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act')}
</script>

<div class="lang-en">

<div class="tm-hero">
<h2>Telemetry — Performance Insights</h2>
<p>SeraPlot optionally collects anonymized performance metrics to identify optimization opportunities and understand usage patterns across environments.</p>
<div class="tm-pills">
<span class="tm-pill">🔒 Opt-in only</span>
<span class="tm-pill">🚫 No personal data</span>
<span class="tm-pill">⚡ Non-blocking</span>
<span class="tm-pill">📊 Open-source</span>
</div>
</div>

<div class="tm-status">
<span class="tm-dot"></span>
Disabled by default — requires explicit activation
</div>

## What Data is Collected

<div class="tm-table-wrap">
<table class="tm-table">
<thead>
<tr><th>Metric</th><th>Type</th><th>Description</th><th>Example</th></tr>
</thead>
<tbody>
<tr><td><code>ts</code> <span class="tm-core">CORE</span></td><td>timestamp</td><td>Unix timestamp of the event</td><td>1746615600</td></tr>
<tr><td><code>method</code> <span class="tm-core">CORE</span></td><td>string</td><td>Function or method name called</td><td>"KMeans.fit", "scatter"</td></tr>
<tr><td><code>duration_ms</code> <span class="tm-core">CORE</span></td><td>float</td><td>Execution time with sub-ms precision</td><td>542.365, 0.113</td></tr>
<tr><td><code>version</code> <span class="tm-core">CORE</span></td><td>string</td><td>SeraPlot version installed</td><td>"2.7.0"</td></tr>
<tr><td><code>os</code> <span class="tm-core">CORE</span></td><td>string</td><td>Operating system</td><td>"windows", "linux"</td></tr>
<tr><td><code>arch</code> <span class="tm-core">CORE</span></td><td>string</td><td>CPU architecture</td><td>"x86_64", "aarch64"</td></tr>
<tr><td><code>cpu_count</code> <span class="tm-core">CORE</span></td><td>integer</td><td>Number of logical CPU cores</td><td>8, 16</td></tr>
<tr><td><code>ram_gb</code> <span class="tm-core">CORE</span></td><td>float</td><td>Total system RAM</td><td>16.0, 32.0</td></tr>
<tr><td><code>available_ram_gb</code> <span class="tm-core">CORE</span></td><td>float</td><td>Available RAM at call time</td><td>8.5, 12.3</td></tr>
<tr><td><code>cpu_brand</code> <span class="tm-core">CORE</span></td><td>string</td><td>CPU model name</td><td>"Intel Core i7-9700"</td></tr>
<tr><td><code>python_version</code> <span class="tm-core">CORE</span></td><td>string</td><td>Python interpreter version</td><td>"3.11.0"</td></tr>
<tr><td><code>data_count</code> <span class="tm-opt">OPT</span></td><td>integer</td><td>Number of records/data points processed</td><td>1000000, 50000</td></tr>
<tr><td><code>data_size_mb</code> <span class="tm-opt">OPT</span></td><td>float</td><td>Approximate input data size in MB</td><td>12.4, 0.8</td></tr>
<tr><td><code>input_shape</code> <span class="tm-opt">OPT</span></td><td>string</td><td>Input data dimensions</td><td>"1000x256"</td></tr>
<tr><td><code>output_shape</code> <span class="tm-opt">OPT</span></td><td>string</td><td>Output data dimensions</td><td>"1000x64"</td></tr>
<tr><td><code>algorithm</code> <span class="tm-opt">OPT</span></td><td>string</td><td>Specific algorithm used</td><td>"KMeans", "DBSCAN"</td></tr>
</tbody>
</table>
</div>

## Privacy & Security

SeraPlot telemetry **never** collects:

<ul class="tm-privacy">
<li>User identity or credentials</li>
<li>File names or file contents</li>
<li>Model weights or parameters</li>
<li>Individual data values or samples</li>
<li>IP addresses or geolocation</li>
<li>Any personally identifiable information</li>
</ul>

<div class="tm-note"><span class="tm-note-ico">ℹ️</span><div>Events are sent in a <strong>background thread</strong> and fall back to local <code>~/.seraplot/telemetry.jsonl</code> if transmission fails. No data is held longer than needed.</div></div>

## How It Works

<div class="tm-steps">
<div class="tm-step"><div class="tm-step-n">1</div><div class="tm-step-body"><strong>Capture</strong><span>Method name, execution time, and system information are recorded at call completion.</span></div></div>
<div class="tm-step"><div class="tm-step-n">2</div><div class="tm-step-body"><strong>Serialize</strong><span>Event is serialized as a compact JSON object with all available fields.</span></div></div>
<div class="tm-step"><div class="tm-step-n">3</div><div class="tm-step-body"><strong>Dispatch</strong><span>A background thread sends the payload to a GitHub Actions dispatcher endpoint.</span></div></div>
<div class="tm-step"><div class="tm-step-n">4</div><div class="tm-step-body"><strong>Fallback</strong><span>If transmission fails, the event is written to a local JSONL file for later retry.</span></div></div>
<div class="tm-step"><div class="tm-step-n">5</div><div class="tm-step-body"><strong>Clear</strong><span>Successfully transmitted events are removed from local storage immediately.</span></div></div>
</div>

## Enable / Disable

<div class="tm-api">
<div class="tm-api-call">

```python
import seraplot as sp

sp.telemetry_consent(enabled=True)   # opt in
sp.telemetry_consent(enabled=False)  # opt out
```

</div>
</div>

This writes `~/.seraplot/consent.json`. Telemetry stays disabled until you call this explicitly.

## Metrics Summary

```python
metrics = sp.get_metrics_summary()
print(metrics)
# {"count": 42, "p50": 12.3, "p95": 310.7, "p99": 892.1, ...}
```

## Source Code

The full telemetry module is open-source. Review it to verify exactly what is collected:

<div class="tm-source">
<div class="tm-source-hdr">
<span class="tm-source-title">
<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/></svg>
telemetry.rs
</span>
<div class="tm-source-dots">
<div class="tm-source-dot" style="background:#ef4444"></div>
<div class="tm-source-dot" style="background:#f59e0b"></div>
<div class="tm-source-dot" style="background:#22c55e"></div>
</div>
</div>
<div class="tm-scroll">

```rust
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

// ── GitHub dispatch endpoint (public repository) ─────────────────────────
const GITHUB_DISPATCH_URL: &str =
    "https://api.github.com/repos/feur25/seraplot/dispatches";

// ── Event structure ───────────────────────────────────────────────────────
pub struct TelemetryEvent {
    pub method:       String,
    pub duration_ms:  f64,
    pub data_count:   Option<u64>,
    pub data_size_mb: Option<f64>,
    pub input_shape:  Option<String>,
    pub output_shape: Option<String>,
    pub algorithm:    Option<String>,
}

impl TelemetryEvent {
    pub fn new(method: &str, duration_ms: f64) -> Self {
        Self {
            method: method.to_string(),
            duration_ms,
            data_count:   None,
            data_size_mb: None,
            input_shape:  None,
            output_shape: None,
            algorithm:    None,
        }
    }

    pub fn with_data(mut self, count: u64, size_mb: f64) -> Self {
        self.data_count   = Some(count);
        self.data_size_mb = Some(size_mb);
        self
    }

    pub fn with_shapes(mut self, input: &str, output: &str) -> Self {
        self.input_shape  = Some(input.to_string());
        self.output_shape = Some(output.to_string());
        self
    }

    pub fn with_algorithm(mut self, algo: &str) -> Self {
        self.algorithm = Some(algo.to_string());
        self
    }
}

// ── Consent gate ─────────────────────────────────────────────────────────
pub fn is_consent_given() -> bool {
    let path = seraplot_dir().join("consent.json");
    if let Ok(text) = std::fs::read_to_string(&path) {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
            return v.get("enabled")
                .and_then(|e| e.as_bool())
                .unwrap_or(false);
        }
    }
    false
}

pub fn set_consent(enabled: bool) {
    let dir = seraplot_dir();
    let _ = std::fs::create_dir_all(&dir);
    let json = format!(
        r#"{{"enabled":{enabled},"version":"{}"}}"#,
        crate::VERSION
    );
    let _ = std::fs::write(dir.join("consent.json"), json);
}

// ── Record an event ───────────────────────────────────────────────────────
pub fn record(event: TelemetryEvent) {
    if !is_consent_given() { return; }

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let sys_info   = get_system_info();
    let duration_f = format!("{:.3}", event.duration_ms)
        .parse::<f64>()
        .unwrap_or(event.duration_ms);

    let mut ev = serde_json::json!({
        "method":      event.method,
        "duration_ms": duration_f,
        "version":     crate::VERSION,
        "ts":          ts,
    });

    // Merge system info fields
    if let (Some(obj), Some(sys)) = (ev.as_object_mut(), sys_info.as_object()) {
        for (k, v) in sys { obj.insert(k.clone(), v.clone()); }
        if let Some(n) = event.data_count   { obj.insert("data_count".into(),   n.into()); }
        if let Some(s) = event.data_size_mb { obj.insert("data_size_mb".into(), ((s*100.0).round()/100.0).into()); }
        if let Some(i) = &event.input_shape  { obj.insert("input_shape".into(),  i.clone().into()); }
        if let Some(o) = &event.output_shape { obj.insert("output_shape".into(), o.clone().into()); }
        if let Some(a) = &event.algorithm    { obj.insert("algorithm".into(),    a.clone().into()); }
    }

    // Non-blocking background send
    std::thread::spawn(move || {
        if try_send_event(ev.clone()) { return; }
        // Fallback: local JSONL
        let path = seraplot_dir().join("telemetry.jsonl");
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true).append(true).open(&path)
        {
            let _ = writeln!(f, "{}", ev);
        }
    });
}
```

</div>
</div>

</div>

---

<div class="lang-fr">

<div class="tm-hero">
<h2>Télémétrie — Analyse des performances</h2>
<p>SeraPlot collecte optionnellement des métriques de performance anonymisées pour identifier les opportunités d'optimisation et comprendre les patterns d'utilisation.</p>
<div class="tm-pills">
<span class="tm-pill">🔒 Opt-in uniquement</span>
<span class="tm-pill">🚫 Aucune donnée personnelle</span>
<span class="tm-pill">⚡ Non-bloquant</span>
<span class="tm-pill">📊 Open-source</span>
</div>
</div>

<div class="tm-status">
<span class="tm-dot"></span>
Désactivée par défaut — nécessite une activation explicite
</div>

## Données collectées

<div class="tm-table-wrap">
<table class="tm-table">
<thead>
<tr><th>Métrique</th><th>Type</th><th>Description</th><th>Exemple</th></tr>
</thead>
<tbody>
<tr><td><code>ts</code> <span class="tm-core">CORE</span></td><td>timestamp</td><td>Horodatage Unix de l'événement</td><td>1746615600</td></tr>
<tr><td><code>method</code> <span class="tm-core">CORE</span></td><td>string</td><td>Nom de la fonction ou méthode appelée</td><td>"KMeans.fit", "scatter"</td></tr>
<tr><td><code>duration_ms</code> <span class="tm-core">CORE</span></td><td>float</td><td>Temps d'exécution avec précision sous-ms</td><td>542.365, 0.113</td></tr>
<tr><td><code>version</code> <span class="tm-core">CORE</span></td><td>string</td><td>Version de SeraPlot installée</td><td>"2.7.0"</td></tr>
<tr><td><code>os</code> <span class="tm-core">CORE</span></td><td>string</td><td>Système d'exploitation</td><td>"windows", "linux"</td></tr>
<tr><td><code>arch</code> <span class="tm-core">CORE</span></td><td>string</td><td>Architecture du processeur</td><td>"x86_64", "aarch64"</td></tr>
<tr><td><code>cpu_count</code> <span class="tm-core">CORE</span></td><td>integer</td><td>Nombre de cœurs logiques</td><td>8, 16</td></tr>
<tr><td><code>ram_gb</code> <span class="tm-core">CORE</span></td><td>float</td><td>RAM totale du système</td><td>16.0, 32.0</td></tr>
<tr><td><code>available_ram_gb</code> <span class="tm-core">CORE</span></td><td>float</td><td>RAM disponible au moment de l'appel</td><td>8.5, 12.3</td></tr>
<tr><td><code>cpu_brand</code> <span class="tm-core">CORE</span></td><td>string</td><td>Modèle du processeur</td><td>"Intel Core i7-9700"</td></tr>
<tr><td><code>python_version</code> <span class="tm-core">CORE</span></td><td>string</td><td>Version de l'interpréteur Python</td><td>"3.11.0"</td></tr>
<tr><td><code>data_count</code> <span class="tm-opt">OPT</span></td><td>integer</td><td>Nombre d'enregistrements traités</td><td>1000000, 50000</td></tr>
<tr><td><code>data_size_mb</code> <span class="tm-opt">OPT</span></td><td>float</td><td>Taille approximative des données en Mo</td><td>12.4, 0.8</td></tr>
<tr><td><code>input_shape</code> <span class="tm-opt">OPT</span></td><td>string</td><td>Dimensions des données d'entrée</td><td>"1000x256"</td></tr>
<tr><td><code>output_shape</code> <span class="tm-opt">OPT</span></td><td>string</td><td>Dimensions des données de sortie</td><td>"1000x64"</td></tr>
<tr><td><code>algorithm</code> <span class="tm-opt">OPT</span></td><td>string</td><td>Algorithme spécifique utilisé</td><td>"KMeans", "DBSCAN"</td></tr>
</tbody>
</table>
</div>

## Confidentialité & Sécurité

La télémétrie SeraPlot ne collecte **jamais** :

<ul class="tm-privacy">
<li>Identité ou credentials utilisateur</li>
<li>Noms ou contenus de fichiers</li>
<li>Poids ou paramètres de modèles</li>
<li>Valeurs individuelles ou échantillons</li>
<li>Adresses IP ou géolocalisation</li>
<li>Toute information personnelle identifiable</li>
</ul>

<div class="tm-note"><span class="tm-note-ico">ℹ️</span><div>Les événements sont envoyés dans un <strong>thread en arrière-plan</strong> et redirigés vers <code>~/.seraplot/telemetry.jsonl</code> en cas d'échec. Aucune donnée n'est conservée plus longtemps que nécessaire.</div></div>

## Fonctionnement

<div class="tm-steps">
<div class="tm-step"><div class="tm-step-n">1</div><div class="tm-step-body"><strong>Capture</strong><span>Le nom de la méthode, le temps d'exécution et les informations système sont enregistrés à la fin de l'appel.</span></div></div>
<div class="tm-step"><div class="tm-step-n">2</div><div class="tm-step-body"><strong>Sérialisation</strong><span>L'événement est sérialisé en JSON compact avec tous les champs disponibles.</span></div></div>
<div class="tm-step"><div class="tm-step-n">3</div><div class="tm-step-body"><strong>Envoi</strong><span>Un thread en arrière-plan envoie le payload vers un endpoint GitHub Actions dispatcher.</span></div></div>
<div class="tm-step"><div class="tm-step-n">4</div><div class="tm-step-body"><strong>Repli</strong><span>En cas d'échec, l'événement est écrit dans un fichier JSONL local pour une retransmission ultérieure.</span></div></div>
<div class="tm-step"><div class="tm-step-n">5</div><div class="tm-step-body"><strong>Nettoyage</strong><span>Les événements transmis avec succès sont immédiatement supprimés du stockage local.</span></div></div>
</div>

## Activer / Désactiver

<div class="tm-api">
<div class="tm-api-call">

```python
import seraplot as sp

sp.telemetry_consent(enabled=True)   # activer
sp.telemetry_consent(enabled=False)  # désactiver
```

</div>
</div>

Cela crée `~/.seraplot/consent.json`. La télémétrie reste désactivée jusqu'à un appel explicite.

## Résumé des métriques

```python
metrics = sp.get_metrics_summary()
print(metrics)
# {"count": 42, "p50": 12.3, "p95": 310.7, "p99": 892.1, ...}
```

## Code source

Le module de télémétrie complet est open-source. Vérifiez-le pour confirmer ce qui est collecté :

<div class="tm-source">
<div class="tm-source-hdr">
<span class="tm-source-title">
<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/></svg>
telemetry.rs
</span>
<div class="tm-source-dots">
<div class="tm-source-dot" style="background:#ef4444"></div>
<div class="tm-source-dot" style="background:#f59e0b"></div>
<div class="tm-source-dot" style="background:#22c55e"></div>
</div>
</div>
<div class="tm-scroll">

```rust
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

const GITHUB_DISPATCH_URL: &str =
    "https://api.github.com/repos/feur25/seraplot/dispatches";
// Jeton chargé depuis l'environnement de build — absent du code source
const GITHUB_TOKEN: &str = env!("SERAPLOT_TOKEN", "<non-défini>");

static PYTHON_VER: std::sync::OnceLock<String> = std::sync::OnceLock::new();

pub fn set_python_version(v: &str) {
    let _ = PYTHON_VER.set(v.to_string());
}

// ── Événement ────────────────────────────────────────────────────────────
pub struct TelemetryEvent {
    pub method:       String,
    pub duration_ms:  f64,
    pub data_count:   Option<u64>,
    pub data_size_mb: Option<f64>,
    pub input_shape:  Option<String>,
    pub output_shape: Option<String>,
    pub algorithm:    Option<String>,
}

impl TelemetryEvent {
    pub fn new(method: &str, duration_ms: f64) -> Self {
        Self {
            method: method.to_string(),
            duration_ms,
            data_count: None, data_size_mb: None,
            input_shape: None, output_shape: None, algorithm: None,
        }
    }
    pub fn with_data(mut self, count: u64, size_mb: f64) -> Self {
        self.data_count = Some(count); self.data_size_mb = Some(size_mb); self
    }
    pub fn with_shapes(mut self, input: &str, output: &str) -> Self {
        self.input_shape = Some(input.to_string());
        self.output_shape = Some(output.to_string()); self
    }
    pub fn with_algorithm(mut self, algo: &str) -> Self {
        self.algorithm = Some(algo.to_string()); self
    }
}

// ── Répertoire de données ─────────────────────────────────────────────────
fn seraplot_dir() -> PathBuf {
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));
    home.join(".seraplot")
}

// ── Infos système (exécuté dans le thread d'arrière-plan) ─────────────────
fn get_system_info() -> serde_json::Value {
    let os        = std::env::consts::OS;
    let arch      = std::env::consts::ARCH;
    let cpu_count = std::thread::available_parallelism()
        .map(|n| n.get()).unwrap_or(1);
    let python_version = PYTHON_VER.get().cloned()
        .unwrap_or_else(|| "unknown".to_string());

    let (ram_gb, available_ram_gb, cpu_brand) = {
        #[cfg(target_os = "windows")]
        {
            // Appel PowerShell unique — indépendant de la locale
            let ps = r#"try{\
$c=Get-CimInstance Win32_ComputerSystem;\
$o=Get-CimInstance Win32_OperatingSystem;\
$p=(Get-CimInstance Win32_Processor|Select-Object -First 1);\
Write-Output("{0}|{1}|{2}"-f $c.TotalPhysicalMemory,\
$o.FreePhysicalMemory,$p.Name)\
}catch{Write-Output '0|0|Unknown'}"#;
            let out = std::process::Command::new("powershell")
                .args(&["-NoProfile", "-NonInteractive", "-Command", ps])
                .output().ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .unwrap_or_default();
            let parts: Vec<&str> = out.trim().split('|').collect();
            // TotalPhysicalMemory → octets; FreePhysicalMemory → Ko
            let ram_gb = parts.get(0)
                .and_then(|s| s.trim().parse::<f64>().ok()).unwrap_or(0.0)
                / (1024.0_f64 * 1024.0 * 1024.0);
            let available_ram_gb = parts.get(1)
                .and_then(|s| s.trim().parse::<f64>().ok()).unwrap_or(0.0)
                / (1024.0 * 1024.0);
            let cpu_brand = parts.get(2)
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "Unknown".to_string());
            (ram_gb, available_ram_gb, cpu_brand)
        }
        #[cfg(target_os = "linux")]
        {
            let mut ram_gb = 0.0f64;
            let mut available_ram_gb = 0.0f64;
            if let Ok(s) = std::fs::read_to_string("/proc/meminfo") {
                for line in s.lines() {
                    if line.starts_with("MemTotal") {
                        if let Some(kb) = line.split_whitespace().nth(1) {
                            if let Ok(v) = kb.parse::<f64>() { ram_gb = v / (1024.0*1024.0); }
                        }
                    }
                    if line.starts_with("MemAvailable") {
                        if let Some(kb) = line.split_whitespace().nth(1) {
                            if let Ok(v) = kb.parse::<f64>() { available_ram_gb = v / (1024.0*1024.0); }
                        }
                    }
                }
            }
            let cpu_brand = std::fs::read_to_string("/proc/cpuinfo").ok()
                .and_then(|s| s.lines().find(|l| l.starts_with("model name:"))
                    .map(|l| l.split(':').nth(1).unwrap_or("Unknown").trim().to_string()))
                .unwrap_or_else(|| "Unknown".to_string());
            (ram_gb, available_ram_gb, cpu_brand)
        }
        #[cfg(target_os = "macos")]
        {
            let mut ram_gb = 0.0f64;
            let mut available_ram_gb = 0.0f64;
            if let Ok(o) = std::process::Command::new("sysctl")
                .args(&["-n", "hw.memsize"]).output() {
                if let Ok(s) = String::from_utf8(o.stdout) {
                    if let Ok(b) = s.trim().parse::<f64>() { ram_gb = b / 1024.0_f64.powi(3); }
                }
            }
            let cpu_brand = std::process::Command::new("sysctl")
                .args(&["-n", "machdep.cpu.brand_string"]).output().ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "Unknown".to_string());
            (ram_gb, available_ram_gb, cpu_brand)
        }
        #[cfg(not(any(target_os="windows",target_os="linux",target_os="macos")))]
        { (0.0, 0.0, "Unknown".to_string()) }
    };
    serde_json::json!({
        "os": os, "arch": arch, "cpu_count": cpu_count,
        "ram_gb": (ram_gb*100.0).round()/100.0,
        "available_ram_gb": (available_ram_gb*100.0).round()/100.0,
        "cpu_brand": cpu_brand, "python_version": python_version
    })
}

// ── Envoi HTTP ────────────────────────────────────────────────────────────
fn try_send_event(event: serde_json::Value) -> bool {
    if GITHUB_TOKEN.is_empty() { return false; }
    let body = serde_json::json!({
        "event_type": "seraplot-telemetry",
        "client_payload": { "events": [event] }
    }).to_string();
    reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build().ok()
        .and_then(|c| c
            .post(GITHUB_DISPATCH_URL)
            .header("Authorization", format!("token {}", GITHUB_TOKEN))
            .header("Accept", "application/vnd.github+json")
            .header("Content-Type", "application/json")
            .header("User-Agent", format!("seraplot/{}", crate::VERSION))
            .body(body).send().ok())
        .map(|r| r.status().as_u16() == 204)
        .unwrap_or(false)
}

// ── Consentement ──────────────────────────────────────────────────────────
pub fn is_consent_given() -> bool {
    let path = seraplot_dir().join("consent.json");
    if !path.exists() { return false; }
    std::fs::read_to_string(&path).ok()
        .and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
        .and_then(|v| v.get("enabled").and_then(|e| e.as_bool()))
        .unwrap_or(false)
}

pub fn set_consent(enabled: bool) {
    let dir = seraplot_dir();
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(
        dir.join("consent.json"),
        format!("{{\"enabled\":{},\"version\":\"{}\"}}", enabled, crate::VERSION),
    );
}

// ── Enregistrement — infos système dans le thread d'arrière-plan ──────────
pub fn record(event: TelemetryEvent) {
    if !is_consent_given() { return; }
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let duration_f64: f64 = format!("{:.3}", event.duration_ms)
        .parse().unwrap_or(event.duration_ms);
    std::thread::spawn(move || {
        let sys_info = get_system_info();   // ← requêtes OS ici
        let mut ev = serde_json::json!({
            "method": event.method, "duration_ms": duration_f64,
            "version": crate::VERSION, "ts": ts
        });
        if let Some(obj) = ev.as_object_mut() {
            if let Some(info) = sys_info.as_object() {
                for (k, v) in info { obj.insert(k.clone(), v.clone()); }
            }
            if let Some(n) = event.data_count   { obj.insert("data_count".into(),   n.into()); }
            if let Some(s) = event.data_size_mb { obj.insert("data_size_mb".into(), ((s*100.0).round()/100.0).into()); }
            if let Some(ref i) = event.input_shape  { obj.insert("input_shape".into(),  i.clone().into()); }
            if let Some(ref o) = event.output_shape { obj.insert("output_shape".into(), o.clone().into()); }
            if let Some(ref a) = event.algorithm    { obj.insert("algorithm".into(),    a.clone().into()); }
        }
        if try_send_event(ev.clone()) { return; }
        // Repli : JSONL local
        let path = seraplot_dir().join("telemetry.jsonl");
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true).append(true).open(&path) {
            let _ = writeln!(f, "{}", ev);
        }
    });
}

// ── Utilitaires ───────────────────────────────────────────────────────────
pub fn read_pending() -> Vec<serde_json::Value> {
    let path = seraplot_dir().join("telemetry.jsonl");
    if !path.exists() { return vec![]; }
    std::fs::read_to_string(&path).ok()
        .map(|t| t.lines().filter_map(|l| serde_json::from_str(l).ok()).collect())
        .unwrap_or_default()
}

pub fn clear_pending() {
    let _ = std::fs::write(seraplot_dir().join("telemetry.jsonl"), b"");
}

pub fn flush_pending() {
    let events = read_pending();
    if events.is_empty() { return; }
    let sent = events.iter().filter(|e| try_send_event((*e).clone())).count();
    if sent == events.len() { clear_pending(); }
}

pub fn get_metrics_summary() -> serde_json::Value {
    let events: Vec<serde_json::Value> = std::fs::read_to_string(
        seraplot_dir().join("telemetry.jsonl")).unwrap_or_default()
        .lines().filter_map(|l| serde_json::from_str(l).ok()).collect();
    let mut per_method: HashMap<String, Vec<f64>> = HashMap::new();
    for ev in &events {
        if let (Some(m), Some(d)) = (
            ev.get("method").and_then(|v| v.as_str()),
            ev.get("duration_ms").and_then(|v| v.as_f64()),
        ) { per_method.entry(m.to_string()).or_default().push(d); }
    }
    let methods: serde_json::Value = per_method.into_iter().map(|(m, mut durs)| {
        durs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let n = durs.len() as f64;
        let avg = durs.iter().sum::<f64>() / n;
        let p = |pct: f64| durs[((pct/100.0)*n) as usize .min(durs.len()-1)];
        (m, serde_json::json!({
            "count": durs.len(), "avg_ms": (avg*1000.0).round()/1000.0,
            "min_ms": durs.first().copied().unwrap_or(0.0),
            "max_ms": durs.last().copied().unwrap_or(0.0),
            "p50_ms": p(50.0), "p95_ms": p(95.0), "p99_ms": p(99.0)
        }))
    }).collect();
    serde_json::json!({
        "event_count": events.len(), "methods": methods, "events": events
    })
}
```

</div>
</div>

</div>

