# Telemetry

<style>
.tm-doc{max-width:1040px;margin:0 auto 3rem;color:#c9d4e3}
.tm-head{padding:0 0 18px;margin:1.2em 0 1.8em;border-bottom:1px solid #2b3749}
.tm-kicker{margin:0 0 8px;color:#8fa1b8;font-size:11px;font-weight:800;letter-spacing:.13em;text-transform:uppercase}
.tm-title{margin:0;border:0;color:#f1f5f9;font-size:32px;font-weight:800;letter-spacing:0;line-height:1.18}
.tm-lead{max-width:840px;margin:12px 0 0;color:#b7c3d4;font-size:15px;line-height:1.75}
.tm-meta{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));margin-top:18px;border:1px solid #2b3749;border-radius:8px;overflow:hidden;background:#0b1220}
.tm-meta div{padding:12px 14px;border-right:1px solid #2b3749;color:#e2e8f0;font-size:13px;font-weight:750}.tm-meta div:last-child{border-right:0}.tm-meta span{display:block;margin-top:4px;color:#8798ad;font-size:11px;font-weight:800;letter-spacing:.08em;text-transform:uppercase}
.tm-layout{display:grid;grid-template-columns:minmax(0,1fr) 280px;gap:18px;align-items:start}
.tm-copy{color:#b7c3d4;font-size:14.5px;line-height:1.75}.tm-copy p{margin:0 0 13px}.tm-copy strong{color:#f1f5f9}
.tm-section{margin:2.1em 0 .85em;color:#e2e8f0;font-size:13px;font-weight:800;letter-spacing:.1em;text-transform:uppercase}
.tm-table-wrap{overflow-x:auto;border:1px solid #2b3749;border-radius:8px;background:#0b1220;margin:12px 0 18px}.tm-table{width:100%;border-collapse:collapse;font-size:13px}.tm-table th{padding:10px 12px;border-bottom:1px solid #2b3749;background:#101827;color:#cbd5e1;text-align:left;font-size:11px;font-weight:800;letter-spacing:.08em;text-transform:uppercase;white-space:nowrap}.tm-table td{padding:11px 12px;border-bottom:1px solid #223044;color:#b7c3d4;line-height:1.6;vertical-align:top}.tm-table tr:last-child td{border-bottom:0}.tm-table code,.tm-inline-code{padding:2px 7px;border:1px solid #344258;border-radius:5px;background:#101a2b;color:#dfe7f2;font-size:11.5px}
.tm-pill{display:inline-block;margin-left:6px;padding:1px 6px;border-radius:5px;font-size:10px;font-weight:800;letter-spacing:.04em}.tm-core{border:1px solid #384a66;background:#121f33;color:#b9c6d8}.tm-opt{border:1px solid #514332;background:#20180f;color:#e0bd83}
.tm-grid{display:grid;grid-template-columns:1fr 1fr;gap:14px}.tm-panel{border:1px solid #2b3749;border-radius:8px;background:#0b1220;overflow:hidden;margin-bottom:16px}.tm-panel-title{padding:10px 14px;border-bottom:1px solid #2b3749;background:#101827;color:#e2e8f0;font-size:12px;font-weight:800;letter-spacing:.08em;text-transform:uppercase}.tm-panel-body{padding:14px;color:#b7c3d4;font-size:13.5px;line-height:1.65}.tm-panel-body p{margin:0 0 10px}.tm-panel-body p:last-child{margin-bottom:0}
.tm-list{margin:0;padding:0;list-style:none}.tm-list li{padding:9px 0;border-bottom:1px solid #223044;color:#b7c3d4}.tm-list li:last-child{border-bottom:0}.tm-list b{display:inline-block;min-width:70px;color:#f1f5f9}
.tm-steps{display:flex;flex-direction:column;gap:8px}.tm-step{display:grid;grid-template-columns:30px 1fr;gap:10px;padding:10px 12px;border:1px solid #223044;border-radius:6px;background:#08101d}.tm-step-n{display:flex;align-items:center;justify-content:center;width:22px;height:22px;border:1px solid #344258;border-radius:5px;background:#101a2b;color:#dfe7f2;font-size:11px;font-weight:800}.tm-step strong{display:block;margin-bottom:3px;color:#f1f5f9}.tm-step span{font-size:12.5px;color:#b7c3d4;line-height:1.55}
.tm-codebox{border:1px solid #2b3749;border-radius:8px;background:#0b1220;overflow:hidden}.tm-codebox-title{padding:9px 13px;border-bottom:1px solid #2b3749;background:#101827;color:#cbd5e1;font-size:11px;font-weight:800;letter-spacing:.08em;text-transform:uppercase}.tm-code-sample{margin:0;padding:16px 18px;background:#08101d;overflow:auto;font:12.5px/1.65 Consolas,Monaco,monospace;color:#dfe7f2;white-space:pre}
.tm-side-table{width:100%;border-collapse:collapse;font-size:12.5px}.tm-side-table td{padding:8px 10px;border-bottom:1px solid #223044;color:#b7c3d4}.tm-side-table tr:last-child td{border-bottom:0}.tm-side-table td:first-child{width:84px;color:#8798ad;background:#111c2e;font-weight:750}
.tm-editor-frame{border:1px solid #263346;border-bottom:0;border-radius:8px 8px 0 0;background:#181f2a;box-shadow:0 16px 40px -28px #000;margin-top:12px}.tm-editor-top{display:flex;align-items:center;gap:10px;height:38px;padding:0 12px}.tm-dots{display:flex;gap:6px}.tm-dot{width:10px;height:10px;border-radius:50%;background:#5b6575}.tm-dot:nth-child(1){background:#d35f5f}.tm-dot:nth-child(2){background:#caa65a}.tm-dot:nth-child(3){background:#5aaf78}.tm-tab{align-self:stretch;display:flex;align-items:center;padding:0 12px;border-left:1px solid #263346;border-right:1px solid #263346;background:#0d1117;color:#d7dee9;font-size:12px;font-weight:800}.tm-editor-meta{margin-left:auto;color:#7f8ea3;font-size:11px;font-weight:700}.tm-editor-frame+pre{margin-top:0!important;max-height:660px;overflow:auto;border:1px solid #263346;border-radius:0 0 8px 8px;background:#0d1117!important}.tm-editor-frame+pre code{font-size:12px;line-height:1.65}
@media(max-width:900px){.tm-layout,.tm-grid,.tm-meta{grid-template-columns:1fr}.tm-meta div{border-right:0;border-bottom:1px solid #2b3749}.tm-meta div:last-child{border-bottom:0}}
</style>

<div class="lang-en">
<div class="tm-doc">
<div class="tm-head">
<p class="tm-kicker">Telemetry reference</p>
<h1 class="tm-title">Runtime metrics, consent, and the real implementation.</h1>
<p class="tm-lead">Telemetry is a small opt-in subsystem. It records coarse runtime metrics, writes failed sends locally, and dispatches in the background only when a token is configured.</p>
<div class="tm-meta"><div>Off<span>Default state</span></div><div>Consent file<span>~/.seraplot</span></div><div>JSONL<span>Local queue</span></div><div>Env token<span>No hardcoded secret</span></div></div>
</div>
<div class="tm-layout"><div class="tm-main">
<div class="tm-copy">
<p>The implementation is intentionally boring: consent gates collection, events are serialized as JSON, and failed sends stay in a local queue. The GitHub dispatch token is read from <code>SERAPLOT_GITHUB_TOKEN</code> or a local git-ignored <code>.env</code> file.</p>
<p>The payload is built from runtime metadata and optional call-site context. It does not include user files, raw values, credentials, model weights, or personally identifying account data.</p>
</div>

<div class="tm-section">Collected Fields</div>
<div class="tm-table-wrap"><table class="tm-table"><thead><tr><th>Metric</th><th>Type</th><th>Description</th><th>Example</th></tr></thead><tbody>
<tr><td><code>ts</code><span class="tm-pill tm-core">CORE</span></td><td>timestamp</td><td>Unix timestamp for the event.</td><td>1746615600</td></tr>
<tr><td><code>method</code><span class="tm-pill tm-core">CORE</span></td><td>string</td><td>Function or method name.</td><td>"scatter"</td></tr>
<tr><td><code>duration_ms</code><span class="tm-pill tm-core">CORE</span></td><td>float</td><td>Execution time rounded to milliseconds.</td><td>0.113</td></tr>
<tr><td><code>version</code><span class="tm-pill tm-core">CORE</span></td><td>string</td><td>Installed SeraPlot version.</td><td>"2.7.10"</td></tr>
<tr><td><code>os</code>, <code>arch</code><span class="tm-pill tm-core">CORE</span></td><td>string</td><td>Operating system and CPU architecture.</td><td>"windows", "x86_64"</td></tr>
<tr><td><code>cpu_count</code>, <code>ram_gb</code><span class="tm-pill tm-core">CORE</span></td><td>number</td><td>Basic system capacity information.</td><td>16, 32.0</td></tr>
<tr><td><code>data_count</code><span class="tm-pill tm-opt">OPT</span></td><td>integer</td><td>Number of records processed, when available.</td><td>1000000</td></tr>
<tr><td><code>input_shape</code>, <code>output_shape</code><span class="tm-pill tm-opt">OPT</span></td><td>string</td><td>Data dimensions supplied by the call site.</td><td>"1000x256"</td></tr>
<tr><td><code>algorithm</code><span class="tm-pill tm-opt">OPT</span></td><td>string</td><td>Algorithm name for ML-related events.</td><td>"KMeans"</td></tr>
</tbody></table></div>

<div class="tm-section">Privacy And Flow</div>
<div class="tm-grid"><div class="tm-panel"><div class="tm-panel-title">Never collected</div><div class="tm-panel-body"><ul class="tm-list"><li><b>No</b> user identity, credentials, or account data.</li><li><b>No</b> file names, file contents, or raw values.</li><li><b>No</b> model weights, parameters, or samples.</li><li><b>No</b> IP address or geolocation in the payload.</li></ul></div></div><div class="tm-panel"><div class="tm-panel-title">Runtime flow</div><div class="tm-panel-body"><div class="tm-steps"><div class="tm-step"><div class="tm-step-n">1</div><div><strong>Capture</strong><span>Method, duration, version, and system summary are recorded after a call.</span></div></div><div class="tm-step"><div class="tm-step-n">2</div><div><strong>Queue</strong><span>The event is appended to <span class="tm-inline-code">telemetry.jsonl</span>.</span></div></div><div class="tm-step"><div class="tm-step-n">3</div><div><strong>Dispatch</strong><span>A background thread sends the payload if a token exists.</span></div></div><div class="tm-step"><div class="tm-step-n">4</div><div><strong>Flush</strong><span>Successfully sent pending events are cleared.</span></div></div></div></div></div></div>

<div class="tm-section">API</div>
<div class="tm-codebox"><div class="tm-codebox-title">Enable, disable, inspect</div><div class="tm-code-sample">import seraplot as sp

sp.telemetry_consent(enabled=True)   # opt in
sp.telemetry_consent(enabled=False)  # opt out

metrics = sp.get_metrics_summary()
print(metrics)</div></div>

</div><aside><div class="tm-panel"><div class="tm-panel-title">Configuration</div><div class="tm-panel-body"><table class="tm-side-table"><tr><td>Token</td><td><code>SERAPLOT_GITHUB_TOKEN</code></td></tr><tr><td>Fallback</td><td><code>.env</code></td></tr><tr><td>Consent</td><td><code>consent.json</code></td></tr><tr><td>Queue</td><td><code>telemetry.jsonl</code></td></tr></table></div></div><div class="tm-panel"><div class="tm-panel-title">Operational note</div><div class="tm-panel-body"><p>The GitHub token is never stored in Rust source. Local development can use a git-ignored <code>.env</code> file; production should set the environment variable directly.</p></div></div></aside></div></div></div>

<div class="lang-fr">
<div class="tm-doc">
<div class="tm-head">
<p class="tm-kicker">Reference telemetry</p>
<h1 class="tm-title">Metriques runtime, consentement et implementation reelle.</h1>
<p class="tm-lead">La telemetry est un petit sous-systeme opt-in. Elle enregistre des metriques runtime generales, conserve les envois echoues en local et dispatch en arriere-plan seulement si un token est configure.</p>
<div class="tm-meta"><div>Off<span>Etat par defaut</span></div><div>Consentement<span>~/.seraplot</span></div><div>JSONL<span>Queue locale</span></div><div>Token env<span>Pas de secret hardcode</span></div></div>
</div>
<div class="tm-layout"><div class="tm-main">
<div class="tm-copy">
<p>L'implementation reste volontairement simple : le consentement controle la collecte, les evenements sont serialises en JSON, et les envois echoues restent dans une queue locale. Le token GitHub dispatch est lu depuis <code>SERAPLOT_GITHUB_TOKEN</code> ou depuis un fichier local <code>.env</code> ignore par git.</p>
<p>Le payload contient des metadonnees runtime et du contexte optionnel fourni par le call site. Il n'inclut pas les fichiers utilisateur, les valeurs brutes, les credentials, les poids de modele ou les donnees de compte identifiantes.</p>
</div>

<div class="tm-section">Champs Collectes</div>
<div class="tm-table-wrap"><table class="tm-table"><thead><tr><th>Metrique</th><th>Type</th><th>Description</th><th>Exemple</th></tr></thead><tbody>
<tr><td><code>ts</code><span class="tm-pill tm-core">CORE</span></td><td>timestamp</td><td>Horodatage Unix de l'evenement.</td><td>1746615600</td></tr>
<tr><td><code>method</code><span class="tm-pill tm-core">CORE</span></td><td>string</td><td>Nom de la fonction ou methode.</td><td>"scatter"</td></tr>
<tr><td><code>duration_ms</code><span class="tm-pill tm-core">CORE</span></td><td>float</td><td>Temps d'execution arrondi en millisecondes.</td><td>0.113</td></tr>
<tr><td><code>version</code><span class="tm-pill tm-core">CORE</span></td><td>string</td><td>Version de SeraPlot installee.</td><td>"2.7.10"</td></tr>
<tr><td><code>os</code>, <code>arch</code><span class="tm-pill tm-core">CORE</span></td><td>string</td><td>Systeme et architecture CPU.</td><td>"windows", "x86_64"</td></tr>
<tr><td><code>cpu_count</code>, <code>ram_gb</code><span class="tm-pill tm-core">CORE</span></td><td>number</td><td>Informations systeme generales.</td><td>16, 32.0</td></tr>
<tr><td><code>data_count</code><span class="tm-pill tm-opt">OPT</span></td><td>integer</td><td>Nombre d'enregistrements traites si disponible.</td><td>1000000</td></tr>
<tr><td><code>input_shape</code>, <code>output_shape</code><span class="tm-pill tm-opt">OPT</span></td><td>string</td><td>Dimensions fournies par le call site.</td><td>"1000x256"</td></tr>
<tr><td><code>algorithm</code><span class="tm-pill tm-opt">OPT</span></td><td>string</td><td>Nom d'algorithme pour les evenements ML.</td><td>"KMeans"</td></tr>
</tbody></table></div>

<div class="tm-section">Confidentialite Et Flux</div>
<div class="tm-grid"><div class="tm-panel"><div class="tm-panel-title">Jamais collecte</div><div class="tm-panel-body"><ul class="tm-list"><li><b>Non</b> identite utilisateur, credentials ou donnees de compte.</li><li><b>Non</b> noms de fichiers, contenus ou valeurs brutes.</li><li><b>Non</b> poids de modele, parametres ou samples.</li><li><b>Non</b> adresse IP ou geolocalisation dans le payload.</li></ul></div></div><div class="tm-panel"><div class="tm-panel-title">Flux runtime</div><div class="tm-panel-body"><div class="tm-steps"><div class="tm-step"><div class="tm-step-n">1</div><div><strong>Capture</strong><span>Methode, duree, version et resume systeme sont enregistres apres un appel.</span></div></div><div class="tm-step"><div class="tm-step-n">2</div><div><strong>Queue</strong><span>L'evenement est ajoute a <span class="tm-inline-code">telemetry.jsonl</span>.</span></div></div><div class="tm-step"><div class="tm-step-n">3</div><div><strong>Dispatch</strong><span>Un thread en arriere-plan envoie le payload si un token existe.</span></div></div><div class="tm-step"><div class="tm-step-n">4</div><div><strong>Flush</strong><span>Les evenements envoyes avec succes sont supprimes de la queue.</span></div></div></div></div></div></div>

<div class="tm-section">API</div>
<div class="tm-codebox"><div class="tm-codebox-title">Activer, desactiver, inspecter</div><div class="tm-code-sample">import seraplot as sp

sp.telemetry_consent(enabled=True)   # activer
sp.telemetry_consent(enabled=False)  # desactiver

metrics = sp.get_metrics_summary()
print(metrics)</div></div>

</div><aside><div class="tm-panel"><div class="tm-panel-title">Configuration</div><div class="tm-panel-body"><table class="tm-side-table"><tr><td>Token</td><td><code>SERAPLOT_GITHUB_TOKEN</code></td></tr><tr><td>Fallback</td><td><code>.env</code></td></tr><tr><td>Consent.</td><td><code>consent.json</code></td></tr><tr><td>File</td><td><code>telemetry.jsonl</code></td></tr></table></div></div><div class="tm-panel"><div class="tm-panel-title">Note operationnelle</div><div class="tm-panel-body"><p>Le token GitHub n'est jamais stocke dans le source Rust. En local, un fichier <code>.env</code> ignore par git peut etre utilise; en production, il faut definir la variable d'environnement.</p></div></div></aside></div></div></div>

<div class="tm-section">telemetry.rs</div>
<div class="tm-editor-frame"><div class="tm-editor-top"><div class="tm-dots"><span class="tm-dot"></span><span class="tm-dot"></span><span class="tm-dot"></span></div><div class="tm-tab">telemetry.rs</div><div class="tm-editor-meta">real src/telemetry.rs</div></div></div>

```rust
{{#include ../../telemetry.rs}}
```
