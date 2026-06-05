# Telemetry

<style>
.tm-doc{max-width:1080px;margin:0 auto 3rem;color:#c9d4e3}
.tm-board{margin:1.2em 0 1.6em;border:1px solid #2b3749;border-radius:8px;background:#0b1220;overflow:hidden}
.tm-board-head{display:grid;grid-template-columns:1fr 240px;gap:22px;padding:22px 24px;border-bottom:1px solid #2b3749;background:#101827}
.tm-kicker{margin:0 0 8px;color:#93a4bb;font-size:11px;font-weight:800;letter-spacing:.13em;text-transform:uppercase}
.tm-title{margin:0;border:0;color:#f1f5f9;font-size:34px;font-weight:800;letter-spacing:0;line-height:1.16}
.tm-lead{margin:14px 0 0;max-width:760px;color:#b7c3d4;font-size:15px;line-height:1.7}
.tm-state{align-self:start;border:1px solid #334155;border-radius:6px;background:#0b1220}
.tm-state-row{display:grid;grid-template-columns:86px 1fr;border-bottom:1px solid #263346;font-size:12px}
.tm-state-row:last-child{border-bottom:0}
.tm-state-k{padding:8px 10px;color:#8798ad;background:#111c2e;font-weight:750}
.tm-state-v{padding:8px 10px;color:#d8e1ee;font-weight:650}
.tm-note{padding:14px 24px;border-bottom:1px solid #2b3749;color:#b7c3d4;font-size:14px;line-height:1.65}
.tm-note strong{color:#f1f5f9}
.tm-strip{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));border-bottom:1px solid #2b3749}
.tm-strip-item{padding:14px 16px;border-right:1px solid #2b3749;color:#d8e1ee;font-size:13px;font-weight:750}
.tm-strip-item:last-child{border-right:0}
.tm-strip-item span{display:block;margin-top:4px;color:#8798ad;font-size:11px;font-weight:800;letter-spacing:.08em;text-transform:uppercase}
.tm-section{margin:2em 0 .85em;color:#e2e8f0;font-size:13px;font-weight:800;letter-spacing:.1em;text-transform:uppercase}
.tm-table-wrap{overflow-x:auto;border:1px solid #2b3749;border-radius:8px;background:#0b1220}
.tm-table{width:100%;border-collapse:collapse;font-size:13px}
.tm-table th{padding:10px 12px;border-bottom:1px solid #2b3749;background:#101827;color:#cbd5e1;text-align:left;font-size:11px;font-weight:800;letter-spacing:.08em;text-transform:uppercase;white-space:nowrap}
.tm-table td{padding:12px;border-bottom:1px solid #223044;color:#b7c3d4;line-height:1.55;vertical-align:top}
.tm-table tr:last-child td{border-bottom:0}
.tm-table tbody tr:hover td{background:#101827}
.tm-table code{padding:2px 7px;border:1px solid #344258;border-radius:5px;background:#101a2b;color:#dfe7f2;font-size:11.5px}
.tm-pill{display:inline-block;margin-left:6px;padding:1px 6px;border-radius:5px;font-size:10px;font-weight:800;letter-spacing:.04em}
.tm-core{border:1px solid #384a66;background:#121f33;color:#b9c6d8}
.tm-opt{border:1px solid #514332;background:#20180f;color:#e0bd83}
.tm-grid{display:grid;grid-template-columns:1fr 1fr;gap:14px}
.tm-panel{border:1px solid #2b3749;border-radius:8px;background:#0b1220;overflow:hidden}
.tm-panel h2{margin:0;padding:11px 14px;border:0;border-bottom:1px solid #2b3749;background:#101827;color:#e2e8f0;font-size:13px;font-weight:800;letter-spacing:.08em;text-transform:uppercase}
.tm-panel-body{padding:14px}
.tm-checks{margin:0;padding:0;list-style:none}
.tm-checks li{padding:10px 0;border-bottom:1px solid #223044;color:#b7c3d4;font-size:13px;line-height:1.55}
.tm-checks li:last-child{border-bottom:0}
.tm-checks b{display:inline-block;min-width:64px;color:#f1f5f9}
.tm-steps{display:flex;flex-direction:column;gap:8px}
.tm-step{display:grid;grid-template-columns:32px 1fr;gap:12px;padding:10px 12px;border:1px solid #223044;border-radius:6px;background:#08101d}
.tm-step-n{display:flex;align-items:center;justify-content:center;width:24px;height:24px;border:1px solid #344258;border-radius:5px;background:#101a2b;color:#dfe7f2;font-size:11px;font-weight:800}
.tm-step strong{display:block;margin:0 0 3px;color:#f1f5f9;font-size:13px}
.tm-step span{color:#b7c3d4;font-size:12.5px;line-height:1.55}
.tm-codebox{border:1px solid #2b3749;border-radius:8px;background:#0b1220;overflow:hidden}
.tm-codebox-title{padding:9px 13px;border-bottom:1px solid #2b3749;background:#101827;color:#cbd5e1;font-size:11px;font-weight:800;letter-spacing:.08em;text-transform:uppercase}
.tm-codebox pre{margin:0;padding:16px 18px;background:#08101d;overflow-x:auto}
.tm-codebox code{color:#dfe7f2;font-size:12.5px;line-height:1.65}
.tm-editor{border:1px solid #263346;border-radius:8px;background:#0d1117;overflow:hidden;box-shadow:0 16px 40px -28px #000}
.tm-editor-top{display:flex;align-items:center;gap:10px;height:34px;padding:0 12px;background:#181f2a;border-bottom:1px solid #263346}
.tm-dots{display:flex;gap:6px}
.tm-dot{width:10px;height:10px;border-radius:50%;background:#5b6575}
.tm-dot:nth-child(1){background:#d35f5f}.tm-dot:nth-child(2){background:#caa65a}.tm-dot:nth-child(3){background:#5aaf78}
.tm-tab{align-self:stretch;display:flex;align-items:center;padding:0 12px;border-left:1px solid #263346;border-right:1px solid #263346;background:#0d1117;color:#d7dee9;font-size:12px;font-weight:700}
.tm-editor-main{display:grid;grid-template-columns:42px 1fr;max-height:520px;overflow:auto;background:#0d1117}
.tm-lines{padding:14px 0;background:#101722;color:#64748b;text-align:right;font:12px/1.65 Consolas,Monaco,monospace;user-select:none}
.tm-lines span{display:block;padding:0 10px}
.tm-code{margin:0;padding:14px 18px;background:#0d1117;color:#d7dee9;font:12px/1.65 Consolas,Monaco,monospace;white-space:pre;overflow:visible}
.tm-cmt{color:#7f8ea3}.tm-kw{color:#c792ea}.tm-fn{color:#82aaff}.tm-str{color:#c3e88d}.tm-ty{color:#ffcb6b}.tm-num{color:#f78c6c}
@media(max-width:860px){
  .tm-board-head,.tm-grid{grid-template-columns:1fr}
  .tm-strip{grid-template-columns:repeat(2,minmax(0,1fr))}
  .tm-strip-item:nth-child(2){border-right:0}
  .tm-strip-item:nth-child(-n+2){border-bottom:1px solid #2b3749}
}
</style>

<div class="lang-en">
<div class="tm-doc">

<div class="tm-board">
<div class="tm-board-head">
<div>
<p class="tm-kicker">Telemetry reference</p>
<h1 class="tm-title">Optional runtime metrics, documented plainly.</h1>
<p class="tm-lead">SeraPlot telemetry records coarse performance events only after explicit consent. This page shows the contract: collected fields, privacy boundaries, API calls, and the real implementation shape.</p>
</div>
<div class="tm-state">
<div class="tm-state-row"><div class="tm-state-k">Default</div><div class="tm-state-v">Off</div></div>
<div class="tm-state-row"><div class="tm-state-k">Consent</div><div class="tm-state-v">Required</div></div>
<div class="tm-state-row"><div class="tm-state-k">Local file</div><div class="tm-state-v">JSONL</div></div>
<div class="tm-state-row"><div class="tm-state-k">Threading</div><div class="tm-state-v">Background</div></div>
</div>
</div>
<div class="tm-note"><strong>Storage path:</strong> consent is stored in <code>~/.seraplot/consent.json</code>; failed sends are kept in <code>~/.seraplot/telemetry.jsonl</code>.</div>
<div class="tm-strip">
<div class="tm-strip-item">Opt-in only<span>User controls activation</span></div>
<div class="tm-strip-item">No raw data<span>No samples or files</span></div>
<div class="tm-strip-item">Non-blocking<span>Background dispatch</span></div>
<div class="tm-strip-item">Auditable<span>Rust source below</span></div>
</div>
</div>

<div class="tm-section">Collected Fields</div>
<div class="tm-table-wrap">
<table class="tm-table">
<thead><tr><th>Metric</th><th>Type</th><th>Description</th><th>Example</th></tr></thead>
<tbody>
<tr><td><code>ts</code><span class="tm-pill tm-core">CORE</span></td><td>timestamp</td><td>Unix timestamp for the event.</td><td>1746615600</td></tr>
<tr><td><code>method</code><span class="tm-pill tm-core">CORE</span></td><td>string</td><td>Function or method name.</td><td>"scatter"</td></tr>
<tr><td><code>duration_ms</code><span class="tm-pill tm-core">CORE</span></td><td>float</td><td>Execution time rounded to milliseconds.</td><td>0.113</td></tr>
<tr><td><code>version</code><span class="tm-pill tm-core">CORE</span></td><td>string</td><td>Installed SeraPlot version.</td><td>"2.7.0"</td></tr>
<tr><td><code>os</code>, <code>arch</code><span class="tm-pill tm-core">CORE</span></td><td>string</td><td>Operating system and CPU architecture.</td><td>"windows", "x86_64"</td></tr>
<tr><td><code>cpu_count</code>, <code>ram_gb</code><span class="tm-pill tm-core">CORE</span></td><td>number</td><td>Basic system capacity information.</td><td>16, 32.0</td></tr>
<tr><td><code>data_count</code><span class="tm-pill tm-opt">OPT</span></td><td>integer</td><td>Number of records processed, when available.</td><td>1000000</td></tr>
<tr><td><code>input_shape</code>, <code>output_shape</code><span class="tm-pill tm-opt">OPT</span></td><td>string</td><td>Data dimensions supplied by the call site.</td><td>"1000x256"</td></tr>
<tr><td><code>algorithm</code><span class="tm-pill tm-opt">OPT</span></td><td>string</td><td>Algorithm name for ML-related events.</td><td>"KMeans"</td></tr>
</tbody>
</table>
</div>

<div class="tm-section">Privacy And Flow</div>
<div class="tm-grid">
<div class="tm-panel">
<h2>Never collected</h2>
<div class="tm-panel-body">
<ul class="tm-checks">
<li><b>No</b> user identity, credentials, or account information.</li>
<li><b>No</b> file names, file contents, or raw data values.</li>
<li><b>No</b> model weights, parameters, or training samples.</li>
<li><b>No</b> IP address, geolocation, or personal identifiers in the payload.</li>
</ul>
</div>
</div>
<div class="tm-panel">
<h2>Runtime flow</h2>
<div class="tm-panel-body">
<div class="tm-steps">
<div class="tm-step"><div class="tm-step-n">1</div><div><strong>Capture</strong><span>Method name, duration, version, and system summary are collected when the call completes.</span></div></div>
<div class="tm-step"><div class="tm-step-n">2</div><div><strong>Serialize</strong><span>The event is written as compact JSON.</span></div></div>
<div class="tm-step"><div class="tm-step-n">3</div><div><strong>Dispatch</strong><span>A background thread sends the payload when possible.</span></div></div>
<div class="tm-step"><div class="tm-step-n">4</div><div><strong>Fallback</strong><span>If sending fails, the event remains in the local JSONL file.</span></div></div>
</div>
</div>
</div>
</div>

<div class="tm-section">API</div>
<div class="tm-codebox"><div class="tm-codebox-title">Enable, disable, inspect</div>
<pre><code>import seraplot as sp

sp.telemetry_consent(enabled=True)   # opt in
sp.telemetry_consent(enabled=False)  # opt out

metrics = sp.get_metrics_summary()
print(metrics)</code></pre>
</div>

<div class="tm-section">Implementation Preview</div>
<div class="tm-editor">
<div class="tm-editor-top"><div class="tm-dots"><span class="tm-dot"></span><span class="tm-dot"></span><span class="tm-dot"></span></div><div class="tm-tab">telemetry.rs</div></div>
<div class="tm-editor-main">
<div class="tm-lines"><span>1</span><span>2</span><span>3</span><span>4</span><span>5</span><span>6</span><span>7</span><span>8</span><span>9</span><span>10</span><span>11</span><span>12</span><span>13</span><span>14</span><span>15</span><span>16</span><span>17</span><span>18</span><span>19</span><span>20</span><span>21</span><span>22</span><span>23</span><span>24</span><span>25</span><span>26</span><span>27</span><span>28</span><span>29</span><span>30</span></div>
<pre class="tm-code"><span class="tm-kw">pub struct</span> <span class="tm-ty">TelemetryEvent</span> {
    <span class="tm-kw">pub</span> method: <span class="tm-ty">String</span>,
    <span class="tm-kw">pub</span> duration_ms: <span class="tm-ty">f64</span>,
    <span class="tm-kw">pub</span> data_count: <span class="tm-ty">Option</span>&lt;<span class="tm-ty">u64</span>&gt;,
    <span class="tm-kw">pub</span> data_size_mb: <span class="tm-ty">Option</span>&lt;<span class="tm-ty">f64</span>&gt;,
    <span class="tm-kw">pub</span> input_shape: <span class="tm-ty">Option</span>&lt;<span class="tm-ty">String</span>&gt;,
    <span class="tm-kw">pub</span> output_shape: <span class="tm-ty">Option</span>&lt;<span class="tm-ty">String</span>&gt;,
    <span class="tm-kw">pub</span> algorithm: <span class="tm-ty">Option</span>&lt;<span class="tm-ty">String</span>&gt;,
}

<span class="tm-kw">pub fn</span> <span class="tm-fn">is_consent_given</span>() -&gt; <span class="tm-ty">bool</span> {
    <span class="tm-kw">let</span> path = <span class="tm-fn">seraplot_dir</span>().join(<span class="tm-str">"consent.json"</span>);
    <span class="tm-kw">if</span> !path.exists() { <span class="tm-kw">return</span> <span class="tm-kw">false</span>; }
    std::fs::read_to_string(&amp;path).ok()
        .and_then(|t| serde_json::from_str::&lt;serde_json::Value&gt;(&amp;t).ok())
        .and_then(|v| v.get(<span class="tm-str">"enabled"</span>).and_then(|e| e.as_bool()))
        .unwrap_or(<span class="tm-kw">false</span>)
}

<span class="tm-kw">pub fn</span> <span class="tm-fn">record</span>(event: <span class="tm-ty">TelemetryEvent</span>) {
    <span class="tm-kw">if</span> !<span class="tm-fn">is_consent_given</span>() { <span class="tm-kw">return</span>; }
    <span class="tm-kw">let</span> ts = <span class="tm-ty">SystemTime</span>::now()
        .duration_since(<span class="tm-ty">UNIX_EPOCH</span>).map(|d| d.as_secs()).unwrap_or(<span class="tm-num">0</span>);
    <span class="tm-kw">let</span> ev = <span class="tm-fn">build_event_json</span>(&amp;event, ts);
    <span class="tm-cmt">// token and transport details intentionally omitted in docs</span>
    std::thread::spawn(<span class="tm-kw">move</span> || { <span class="tm-fn">try_send_event</span>(ev); });
}</pre>
</div>
</div>

</div>
</div>

---

<div class="lang-fr">
<div class="tm-doc">

<div class="tm-board">
<div class="tm-board-head">
<div>
<p class="tm-kicker">Reference telemetrie</p>
<h1 class="tm-title">Mesures runtime optionnelles, documentees clairement.</h1>
<p class="tm-lead">La telemetrie SeraPlot enregistre des evenements de performance seulement apres consentement explicite. Cette page montre le contrat : champs collectes, limites de confidentialite, appels API et forme reelle de l'implementation.</p>
</div>
<div class="tm-state">
<div class="tm-state-row"><div class="tm-state-k">Defaut</div><div class="tm-state-v">Off</div></div>
<div class="tm-state-row"><div class="tm-state-k">Consent.</div><div class="tm-state-v">Requis</div></div>
<div class="tm-state-row"><div class="tm-state-k">Fichier</div><div class="tm-state-v">JSONL</div></div>
<div class="tm-state-row"><div class="tm-state-k">Thread</div><div class="tm-state-v">Arriere-plan</div></div>
</div>
</div>
<div class="tm-note"><strong>Chemins :</strong> le consentement est stocke dans <code>~/.seraplot/consent.json</code>; les envois echoues restent dans <code>~/.seraplot/telemetry.jsonl</code>.</div>
<div class="tm-strip">
<div class="tm-strip-item">Opt-in seulement<span>Activation utilisateur</span></div>
<div class="tm-strip-item">Pas de donnees brutes<span>Pas de fichiers ni samples</span></div>
<div class="tm-strip-item">Non bloquant<span>Envoi en arriere-plan</span></div>
<div class="tm-strip-item">Auditable<span>Source Rust ci-dessous</span></div>
</div>
</div>

<div class="tm-section">Champs Collectes</div>
<div class="tm-table-wrap">
<table class="tm-table">
<thead><tr><th>Metrique</th><th>Type</th><th>Description</th><th>Exemple</th></tr></thead>
<tbody>
<tr><td><code>ts</code><span class="tm-pill tm-core">CORE</span></td><td>timestamp</td><td>Horodatage Unix de l'evenement.</td><td>1746615600</td></tr>
<tr><td><code>method</code><span class="tm-pill tm-core">CORE</span></td><td>string</td><td>Nom de la fonction ou methode.</td><td>"scatter"</td></tr>
<tr><td><code>duration_ms</code><span class="tm-pill tm-core">CORE</span></td><td>float</td><td>Temps d'execution arrondi en millisecondes.</td><td>0.113</td></tr>
<tr><td><code>version</code><span class="tm-pill tm-core">CORE</span></td><td>string</td><td>Version de SeraPlot installee.</td><td>"2.7.0"</td></tr>
<tr><td><code>os</code>, <code>arch</code><span class="tm-pill tm-core">CORE</span></td><td>string</td><td>Systeme et architecture CPU.</td><td>"windows", "x86_64"</td></tr>
<tr><td><code>cpu_count</code>, <code>ram_gb</code><span class="tm-pill tm-core">CORE</span></td><td>number</td><td>Informations systeme generales.</td><td>16, 32.0</td></tr>
<tr><td><code>data_count</code><span class="tm-pill tm-opt">OPT</span></td><td>integer</td><td>Nombre d'enregistrements traites, si disponible.</td><td>1000000</td></tr>
<tr><td><code>input_shape</code>, <code>output_shape</code><span class="tm-pill tm-opt">OPT</span></td><td>string</td><td>Dimensions fournies par le call site.</td><td>"1000x256"</td></tr>
<tr><td><code>algorithm</code><span class="tm-pill tm-opt">OPT</span></td><td>string</td><td>Nom d'algorithme pour les evenements ML.</td><td>"KMeans"</td></tr>
</tbody>
</table>
</div>

<div class="tm-section">Confidentialite Et Flux</div>
<div class="tm-grid">
<div class="tm-panel">
<h2>Jamais collecte</h2>
<div class="tm-panel-body">
<ul class="tm-checks">
<li><b>Non</b> identite, credentials ou informations de compte.</li>
<li><b>Non</b> noms de fichiers, contenus ou valeurs brutes.</li>
<li><b>Non</b> poids de modeles, parametres ou samples d'entrainement.</li>
<li><b>Non</b> adresse IP, geolocalisation ou identifiants personnels dans le payload.</li>
</ul>
</div>
</div>
<div class="tm-panel">
<h2>Flux runtime</h2>
<div class="tm-panel-body">
<div class="tm-steps">
<div class="tm-step"><div class="tm-step-n">1</div><div><strong>Capture</strong><span>Methode, duree, version et resume systeme a la fin de l'appel.</span></div></div>
<div class="tm-step"><div class="tm-step-n">2</div><div><strong>Serialisation</strong><span>L'evenement est ecrit en JSON compact.</span></div></div>
<div class="tm-step"><div class="tm-step-n">3</div><div><strong>Envoi</strong><span>Un thread en arriere-plan envoie le payload si possible.</span></div></div>
<div class="tm-step"><div class="tm-step-n">4</div><div><strong>Repli</strong><span>Si l'envoi echoue, l'evenement reste dans le fichier JSONL local.</span></div></div>
</div>
</div>
</div>
</div>

<div class="tm-section">API</div>
<div class="tm-codebox"><div class="tm-codebox-title">Activer, desactiver, inspecter</div>
<pre><code>import seraplot as sp

sp.telemetry_consent(enabled=True)   # activer
sp.telemetry_consent(enabled=False)  # desactiver

metrics = sp.get_metrics_summary()
print(metrics)</code></pre>
</div>

<div class="tm-section">Preview Implementation</div>
<div class="tm-editor">
<div class="tm-editor-top"><div class="tm-dots"><span class="tm-dot"></span><span class="tm-dot"></span><span class="tm-dot"></span></div><div class="tm-tab">telemetry.rs</div></div>
<div class="tm-editor-main">
<div class="tm-lines"><span>1</span><span>2</span><span>3</span><span>4</span><span>5</span><span>6</span><span>7</span><span>8</span><span>9</span><span>10</span><span>11</span><span>12</span><span>13</span><span>14</span><span>15</span><span>16</span><span>17</span><span>18</span><span>19</span><span>20</span><span>21</span><span>22</span><span>23</span><span>24</span><span>25</span><span>26</span><span>27</span><span>28</span><span>29</span><span>30</span></div>
<pre class="tm-code"><span class="tm-kw">pub struct</span> <span class="tm-ty">TelemetryEvent</span> {
    <span class="tm-kw">pub</span> method: <span class="tm-ty">String</span>,
    <span class="tm-kw">pub</span> duration_ms: <span class="tm-ty">f64</span>,
    <span class="tm-kw">pub</span> data_count: <span class="tm-ty">Option</span>&lt;<span class="tm-ty">u64</span>&gt;,
    <span class="tm-kw">pub</span> data_size_mb: <span class="tm-ty">Option</span>&lt;<span class="tm-ty">f64</span>&gt;,
    <span class="tm-kw">pub</span> input_shape: <span class="tm-ty">Option</span>&lt;<span class="tm-ty">String</span>&gt;,
    <span class="tm-kw">pub</span> output_shape: <span class="tm-ty">Option</span>&lt;<span class="tm-ty">String</span>&gt;,
    <span class="tm-kw">pub</span> algorithm: <span class="tm-ty">Option</span>&lt;<span class="tm-ty">String</span>&gt;,
}

<span class="tm-kw">pub fn</span> <span class="tm-fn">is_consent_given</span>() -&gt; <span class="tm-ty">bool</span> {
    <span class="tm-kw">let</span> path = <span class="tm-fn">seraplot_dir</span>().join(<span class="tm-str">"consent.json"</span>);
    <span class="tm-kw">if</span> !path.exists() { <span class="tm-kw">return</span> <span class="tm-kw">false</span>; }
    std::fs::read_to_string(&amp;path).ok()
        .and_then(|t| serde_json::from_str::&lt;serde_json::Value&gt;(&amp;t).ok())
        .and_then(|v| v.get(<span class="tm-str">"enabled"</span>).and_then(|e| e.as_bool()))
        .unwrap_or(<span class="tm-kw">false</span>)
}

<span class="tm-kw">pub fn</span> <span class="tm-fn">record</span>(event: <span class="tm-ty">TelemetryEvent</span>) {
    <span class="tm-kw">if</span> !<span class="tm-fn">is_consent_given</span>() { <span class="tm-kw">return</span>; }
    <span class="tm-kw">let</span> ts = <span class="tm-ty">SystemTime</span>::now()
        .duration_since(<span class="tm-ty">UNIX_EPOCH</span>).map(|d| d.as_secs()).unwrap_or(<span class="tm-num">0</span>);
    <span class="tm-kw">let</span> ev = <span class="tm-fn">build_event_json</span>(&amp;event, ts);
    <span class="tm-cmt">// token and transport details intentionally omitted in docs</span>
    std::thread::spawn(<span class="tm-kw">move</span> || { <span class="tm-fn">try_send_event</span>(ev); });
}</pre>
</div>
</div>

</div>
</div>
