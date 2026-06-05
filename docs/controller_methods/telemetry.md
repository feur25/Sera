# Telemetry

<style>
.tm-page{max-width:1040px;margin:0 auto 3rem}
.tm-hero{margin:1.2em 0 1.8em;padding:30px 0 22px;border-bottom:1px solid #243044}
.tm-kicker{margin:0 0 10px;color:#8aa0bd;font-size:12px;font-weight:800;letter-spacing:.12em;text-transform:uppercase}
.tm-title{margin:0 0 12px;border:0;color:#f1f5f9;font-size:clamp(30px,4vw,44px);font-weight:800;letter-spacing:0;line-height:1.12}
.tm-lead{max-width:760px;margin:0;color:#b6c2d2;font-size:15.5px;line-height:1.7}
.tm-badges{display:flex;flex-wrap:wrap;gap:7px;margin-top:18px}
.tm-badge{padding:4px 9px;border:1px solid #33445c;border-radius:6px;background:#101a2b;color:#b9c6d8;font-size:11px;font-weight:650}
.tm-status{display:inline-flex;align-items:center;gap:8px;margin:0 0 1.2em;padding:7px 11px;border:1px solid #28543c;border-radius:7px;background:#0b1a13;color:#9bd7ad;font-size:12.5px;font-weight:650}
.tm-dot{width:8px;height:8px;border-radius:50%;background:#57b879}
.tm-section{margin:2em 0 .85em;padding-bottom:8px;border-bottom:1px solid #243044;color:#d7dee9;font-size:12px;font-weight:800;letter-spacing:.12em;text-transform:uppercase}
.tm-table-wrap{overflow-x:auto;margin:1em 0 1.8em;border:1px solid #263346;border-radius:8px;background:#0b1220}
.tm-table{width:100%;border-collapse:collapse;font-size:12.5px}
.tm-table th{padding:10px 12px;border-bottom:1px solid #263346;background:#101a2b;color:#c7d2e0;text-align:left;font-size:11px;font-weight:800;letter-spacing:.06em;text-transform:uppercase;white-space:nowrap}
.tm-table td{padding:10px 12px;border-bottom:1px solid #1e2a3c;color:#b6c2d2;line-height:1.55;vertical-align:top}
.tm-table tr:last-child td{border-bottom:0}
.tm-table tbody tr:hover td{background:#101827}
.tm-table code{padding:2px 7px;border:1px solid #33445c;border-radius:5px;background:#101a2b;color:#d7dee9;font-size:11.5px}
.tm-pill{display:inline-block;margin-left:6px;padding:1px 6px;border-radius:5px;font-size:10px;font-weight:800;letter-spacing:.04em}
.tm-core{border:1px solid #384a66;background:#121f33;color:#b9c6d8}
.tm-opt{border:1px solid #514332;background:#20180f;color:#e0bd83}
.tm-grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(230px,1fr));gap:10px;margin:1em 0 1.8em}
.tm-card{padding:13px 14px;border:1px solid #263346;border-radius:8px;background:#0b1220;color:#b6c2d2;font-size:13px;line-height:1.55}
.tm-card strong{display:block;margin-bottom:4px;color:#e8eef7;font-size:13.5px}
.tm-no{border-color:#3b2d35;background:#141018;color:#c8b8c0}
.tm-no::before{content:"No";display:inline-block;margin-right:8px;color:#e19aa8;font-size:11px;font-weight:800;text-transform:uppercase}
.tm-steps{display:flex;flex-direction:column;gap:10px;margin:1em 0 1.8em}
.tm-step{display:grid;grid-template-columns:34px 1fr;gap:12px;align-items:start}
.tm-step-n{display:flex;align-items:center;justify-content:center;width:28px;height:28px;border:1px solid #33445c;border-radius:7px;background:#101a2b;color:#d7dee9;font-size:12px;font-weight:800}
.tm-step strong{display:block;margin:0 0 3px;color:#e8eef7;font-size:13.5px}
.tm-step span{color:#aab8ca;font-size:13px;line-height:1.6}
.tm-note{margin:1.2em 0;padding:13px 15px;border:1px solid #2b4561;border-radius:8px;background:#0b1624;color:#b9c6d8;font-size:13.5px;line-height:1.6}
.tm-note strong{color:#e8eef7}
.tm-code{margin:1em 0 1.8em;border:1px solid #263346;border-radius:8px;overflow:hidden;background:#0b1220}
.tm-code-title{padding:9px 13px;border-bottom:1px solid #263346;background:#101a2b;color:#c7d2e0;font-size:11px;font-weight:800;letter-spacing:.08em;text-transform:uppercase}
.tm-code pre{margin:0;padding:16px 18px;background:#08101d;overflow-x:auto}
.tm-code code{color:#d7dee9;font-size:12.5px;line-height:1.65}
@media(max-width:760px){
  .tm-hero{padding:22px 0 18px}
  .tm-table th,.tm-table td{padding:9px 10px}
  .tm-step{grid-template-columns:30px 1fr}
}
</style>

<div class="lang-en">
<div class="tm-page">

<div class="tm-hero">
<p class="tm-kicker">Privacy-first telemetry</p>
<h1 class="tm-title">Optional performance metrics for SeraPlot.</h1>
<p class="tm-lead">SeraPlot can record anonymized runtime metrics to help identify slow paths, environment issues, and optimization opportunities. Telemetry is disabled until the user explicitly opts in.</p>
<div class="tm-badges"><span class="tm-badge">Opt-in only</span><span class="tm-badge">No personal data</span><span class="tm-badge">Local fallback</span><span class="tm-badge">Open source</span></div>
</div>

<div class="tm-status"><span class="tm-dot"></span>Disabled by default</div>

<div class="tm-section">Collected Fields</div>
<div class="tm-table-wrap">
<table class="tm-table">
<thead><tr><th>Metric</th><th>Type</th><th>Description</th><th>Example</th></tr></thead>
<tbody>
<tr><td><code>ts</code><span class="tm-pill tm-core">CORE</span></td><td>timestamp</td><td>Unix timestamp for the event.</td><td>1746615600</td></tr>
<tr><td><code>method</code><span class="tm-pill tm-core">CORE</span></td><td>string</td><td>Function or method name.</td><td>"scatter"</td></tr>
<tr><td><code>duration_ms</code><span class="tm-pill tm-core">CORE</span></td><td>float</td><td>Execution time with sub-ms precision.</td><td>0.113</td></tr>
<tr><td><code>version</code><span class="tm-pill tm-core">CORE</span></td><td>string</td><td>Installed SeraPlot version.</td><td>"2.7.0"</td></tr>
<tr><td><code>os</code>, <code>arch</code><span class="tm-pill tm-core">CORE</span></td><td>string</td><td>Operating system and CPU architecture.</td><td>"windows", "x86_64"</td></tr>
<tr><td><code>cpu_count</code>, <code>ram_gb</code><span class="tm-pill tm-core">CORE</span></td><td>number</td><td>Basic system capacity information.</td><td>16, 32.0</td></tr>
<tr><td><code>data_count</code><span class="tm-pill tm-opt">OPT</span></td><td>integer</td><td>Number of records processed, when available.</td><td>1000000</td></tr>
<tr><td><code>input_shape</code>, <code>output_shape</code><span class="tm-pill tm-opt">OPT</span></td><td>string</td><td>Data dimensions, when supplied by the call site.</td><td>"1000x256"</td></tr>
<tr><td><code>algorithm</code><span class="tm-pill tm-opt">OPT</span></td><td>string</td><td>Algorithm name for ML-related events.</td><td>"KMeans"</td></tr>
</tbody>
</table>
</div>

<div class="tm-section">Privacy</div>
<p>SeraPlot telemetry never collects:</p>
<div class="tm-grid">
<div class="tm-card tm-no">User identity, credentials, or account information</div>
<div class="tm-card tm-no">File names, file contents, or raw data values</div>
<div class="tm-card tm-no">Model weights, parameters, or training samples</div>
<div class="tm-card tm-no">IP addresses, geolocation, or personal identifiers</div>
</div>
<div class="tm-note"><strong>Storage:</strong> failed sends are written to <code>~/.seraplot/telemetry.jsonl</code> and successfully transmitted events are cleared from local storage.</div>

<div class="tm-section">Workflow</div>
<div class="tm-steps">
<div class="tm-step"><div class="tm-step-n">1</div><div><strong>Capture</strong><span>Method name, duration, version, and system summary are collected when the call completes.</span></div></div>
<div class="tm-step"><div class="tm-step-n">2</div><div><strong>Serialize</strong><span>The event is written as a compact JSON object.</span></div></div>
<div class="tm-step"><div class="tm-step-n">3</div><div><strong>Dispatch</strong><span>A background thread sends the payload without blocking the chart or ML call.</span></div></div>
<div class="tm-step"><div class="tm-step-n">4</div><div><strong>Fallback</strong><span>If sending fails, the event stays local for a later retry.</span></div></div>
</div>

<div class="tm-section">API</div>
<div class="tm-code"><div class="tm-code-title">Enable or disable</div>

```python
import seraplot as sp

sp.telemetry_consent(enabled=True)   # opt in
sp.telemetry_consent(enabled=False)  # opt out
```

</div>
<p>This writes <code>~/.seraplot/consent.json</code>. Telemetry remains disabled until this call is made explicitly.</p>

<div class="tm-code"><div class="tm-code-title">Local metrics summary</div>

```python
metrics = sp.get_metrics_summary()
print(metrics)
```

</div>

</div>
</div>

---

<div class="lang-fr">
<div class="tm-page">

<div class="tm-hero">
<p class="tm-kicker">Telemetrie respectueuse</p>
<h1 class="tm-title">Mesures de performance optionnelles pour SeraPlot.</h1>
<p class="tm-lead">SeraPlot peut enregistrer des metriques anonymisees pour identifier les appels lents, les problemes d'environnement et les pistes d'optimisation. La telemetrie reste desactivee tant que l'utilisateur ne l'active pas explicitement.</p>
<div class="tm-badges"><span class="tm-badge">Opt-in uniquement</span><span class="tm-badge">Aucune donnee personnelle</span><span class="tm-badge">Repli local</span><span class="tm-badge">Open source</span></div>
</div>

<div class="tm-status"><span class="tm-dot"></span>Desactivee par defaut</div>

<div class="tm-section">Champs collectes</div>
<div class="tm-table-wrap">
<table class="tm-table">
<thead><tr><th>Metrique</th><th>Type</th><th>Description</th><th>Exemple</th></tr></thead>
<tbody>
<tr><td><code>ts</code><span class="tm-pill tm-core">CORE</span></td><td>timestamp</td><td>Horodatage Unix de l'evenement.</td><td>1746615600</td></tr>
<tr><td><code>method</code><span class="tm-pill tm-core">CORE</span></td><td>string</td><td>Nom de la fonction ou methode appelee.</td><td>"scatter"</td></tr>
<tr><td><code>duration_ms</code><span class="tm-pill tm-core">CORE</span></td><td>float</td><td>Temps d'execution avec precision sous-ms.</td><td>0.113</td></tr>
<tr><td><code>version</code><span class="tm-pill tm-core">CORE</span></td><td>string</td><td>Version de SeraPlot installee.</td><td>"2.7.0"</td></tr>
<tr><td><code>os</code>, <code>arch</code><span class="tm-pill tm-core">CORE</span></td><td>string</td><td>Systeme d'exploitation et architecture CPU.</td><td>"windows", "x86_64"</td></tr>
<tr><td><code>cpu_count</code>, <code>ram_gb</code><span class="tm-pill tm-core">CORE</span></td><td>number</td><td>Informations systeme generales.</td><td>16, 32.0</td></tr>
<tr><td><code>data_count</code><span class="tm-pill tm-opt">OPT</span></td><td>integer</td><td>Nombre d'enregistrements traites si disponible.</td><td>1000000</td></tr>
<tr><td><code>input_shape</code>, <code>output_shape</code><span class="tm-pill tm-opt">OPT</span></td><td>string</td><td>Dimensions des donnees si fournies par l'appel.</td><td>"1000x256"</td></tr>
<tr><td><code>algorithm</code><span class="tm-pill tm-opt">OPT</span></td><td>string</td><td>Nom de l'algorithme pour les evenements ML.</td><td>"KMeans"</td></tr>
</tbody>
</table>
</div>

<div class="tm-section">Confidentialite</div>
<p>La telemetrie SeraPlot ne collecte jamais :</p>
<div class="tm-grid">
<div class="tm-card tm-no">Identite utilisateur, credentials ou informations de compte</div>
<div class="tm-card tm-no">Noms de fichiers, contenus de fichiers ou valeurs brutes</div>
<div class="tm-card tm-no">Poids de modeles, parametres ou echantillons d'entrainement</div>
<div class="tm-card tm-no">Adresses IP, geolocalisation ou identifiants personnels</div>
</div>
<div class="tm-note"><strong>Stockage :</strong> les envois echoues sont ecrits dans <code>~/.seraplot/telemetry.jsonl</code> et les evenements transmis sont supprimes du stockage local.</div>

<div class="tm-section">Fonctionnement</div>
<div class="tm-steps">
<div class="tm-step"><div class="tm-step-n">1</div><div><strong>Capture</strong><span>Le nom de la methode, la duree, la version et un resume systeme sont collectes a la fin de l'appel.</span></div></div>
<div class="tm-step"><div class="tm-step-n">2</div><div><strong>Serialisation</strong><span>L'evenement est ecrit sous forme d'objet JSON compact.</span></div></div>
<div class="tm-step"><div class="tm-step-n">3</div><div><strong>Envoi</strong><span>Un thread en arriere-plan envoie le payload sans bloquer l'appel.</span></div></div>
<div class="tm-step"><div class="tm-step-n">4</div><div><strong>Repli</strong><span>Si l'envoi echoue, l'evenement reste local pour une tentative ulterieure.</span></div></div>
</div>

<div class="tm-section">API</div>
<div class="tm-code"><div class="tm-code-title">Activer ou desactiver</div>

```python
import seraplot as sp

sp.telemetry_consent(enabled=True)   # activer
sp.telemetry_consent(enabled=False)  # desactiver
```

</div>
<p>Cela ecrit <code>~/.seraplot/consent.json</code>. La telemetrie reste desactivee tant que cet appel n'est pas fait explicitement.</p>

<div class="tm-code"><div class="tm-code-title">Resume local des metriques</div>

```python
metrics = sp.get_metrics_summary()
print(metrics)
```

</div>

</div>
</div>
