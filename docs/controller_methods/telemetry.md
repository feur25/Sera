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
.tm-mini-code{display:block;width:100%;min-height:138px;margin:0;padding:16px 18px;border:0;resize:vertical;background:#08101d;color:#dfe7f2;font:12.5px/1.65 Consolas,Monaco,monospace;white-space:pre;overflow:auto}
.tm-source-code{display:block;width:100%;min-height:620px;margin:0;padding:16px 18px;border:1px solid #263346;border-top:0;border-radius:0 0 8px 8px;resize:vertical;background:#0d1117;color:#d7dee9;font:12px/1.65 Consolas,Monaco,monospace;white-space:pre;overflow:auto}
.tm-editor{border:1px solid #263346;border-bottom:0;border-radius:8px 8px 0 0;background:#0d1117;overflow:hidden;box-shadow:0 16px 40px -28px #000}
.tm-editor-top{display:flex;align-items:center;gap:10px;height:34px;padding:0 12px;background:#181f2a;border-bottom:1px solid #263346}
.tm-dots{display:flex;gap:6px}
.tm-dot{width:10px;height:10px;border-radius:50%;background:#5b6575}
.tm-dot:nth-child(1){background:#d35f5f}.tm-dot:nth-child(2){background:#caa65a}.tm-dot:nth-child(3){background:#5aaf78}
.tm-tab{align-self:stretch;display:flex;align-items:center;padding:0 12px;border-left:1px solid #263346;border-right:1px solid #263346;background:#0d1117;color:#d7dee9;font-size:12px;font-weight:700}


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
<textarea class="tm-mini-code" readonly spellcheck="false">import seraplot as sp

sp.telemetry_consent(enabled=True)   # opt in
sp.telemetry_consent(enabled=False)  # opt out

metrics = sp.get_metrics_summary()
print(metrics)</textarea>
</div>

<div class="tm-section">Implementation Preview</div>
<div class="tm-editor">
<div class="tm-editor-top"><div class="tm-dots"><span class="tm-dot"></span><span class="tm-dot"></span><span class="tm-dot"></span></div><div class="tm-tab">telemetry.rs</div></div>
</div>

<textarea class="tm-source-code" readonly spellcheck="false">use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(not(target_arch = "wasm32"))]
const GITHUB_DISPATCH_URL: &amp;str = "https://api.github.com/repos/feur25/seraplot/dispatches";
#[cfg(not(target_arch = "wasm32"))]
const GITHUB_TOKEN_ENV: &amp;str = "SERAPLOT_GITHUB_TOKEN";

static PYTHON_VER: std::sync::OnceLock&lt;String&gt; = std::sync::OnceLock::new();
static SYS_INFO_CACHE: std::sync::OnceLock&lt;serde_json::Value&gt; = std::sync::OnceLock::new();
#[cfg(not(target_arch = "wasm32"))]
static GITHUB_TOKEN_CACHE: std::sync::OnceLock&lt;Option&lt;String&gt;&gt; = std::sync::OnceLock::new();

pub fn set_python_version(v: &amp;str) {
    let _ = PYTHON_VER.set(v.to_string());
}

pub struct TelemetryEvent {
    pub method: String,
    pub duration_ms: f64,
    pub data_count: Option&lt;u64&gt;,
    pub data_size_mb: Option&lt;f64&gt;,
    pub input_shape: Option&lt;String&gt;,
    pub output_shape: Option&lt;String&gt;,
    pub algorithm: Option&lt;String&gt;,
}

impl TelemetryEvent {
    pub fn new(method: &amp;str, duration_ms: f64) -&gt; Self {
        Self {
            method: method.to_string(),
            duration_ms,
            data_count: None,
            data_size_mb: None,
            input_shape: None,
            output_shape: None,
            algorithm: None,
        }
    }

    pub fn with_data(mut self, count: u64, size_mb: f64) -&gt; Self {
        self.data_count = Some(count);
        self.data_size_mb = Some(size_mb);
        self
    }

    pub fn with_count(mut self, count: u64) -&gt; Self {
        self.data_count = Some(count);
        self
    }

    pub fn with_shapes(mut self, input: &amp;str, output: &amp;str) -&gt; Self {
        self.input_shape = Some(input.to_string());
        self.output_shape = Some(output.to_string());
        self
    }

    pub fn with_algorithm(mut self, algo: &amp;str) -&gt; Self {
        self.algorithm = Some(algo.to_string());
        self
    }
}

fn seraplot_dir() -&gt; PathBuf {
    std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".seraplot")
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_bytes(s: &amp;str) -&gt; f64 {
    s.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::&lt;String&gt;()
        .parse::&lt;f64&gt;()
        .unwrap_or(0.0)
}

fn collect_system_info() -&gt; serde_json::Value {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let cpu_count = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let python_version = PYTHON_VER
        .get()
        .cloned()
        .unwrap_or_else(|| "unknown".to_string());

    let (ram_gb, available_ram_gb, cpu_brand) = {
        #[cfg(target_os = "windows")]
        {
            let ps = r#"try{$c=Get-CimInstance Win32_ComputerSystem;$o=Get-CimInstance Win32_OperatingSystem;$p=Get-CimInstance Win32_Processor|Select-Object -First 1;Write-Output "$($c.TotalPhysicalMemory)|$($o.FreePhysicalMemory)|$($p.Name)"}catch{Write-Output '0|0|Unknown'}"#;
            let out = std::process::Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", ps])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .unwrap_or_default();
            let parts: Vec&lt;&amp;str&gt; = out.trim().splitn(3, '|').collect();
            let ram = parse_bytes(parts.first().unwrap_or(&amp;"0")) / 1024.0_f64.powi(3);
            let avail = parse_bytes(parts.get(1).unwrap_or(&amp;"0")) / (1024.0 * 1024.0);
            let brand = parts
                .get(2)
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "Unknown".to_string());
            (ram, avail, brand)
        }
        #[cfg(target_os = "linux")]
        {
            let mut ram_gb = 0.0f64;
            let mut available_ram_gb = 0.0f64;
            if let Ok(s) = std::fs::read_to_string("/proc/meminfo") {
                for line in s.lines() {
                    if line.starts_with("MemTotal:") {
                        if let Some(kb) = line.split_whitespace().nth(1) {
                            ram_gb = kb.parse::&lt;f64&gt;().unwrap_or(0.0) / (1024.0 * 1024.0);
                        }
                    } else if line.starts_with("MemAvailable:") {
                        if let Some(kb) = line.split_whitespace().nth(1) {
                            available_ram_gb = kb.parse::&lt;f64&gt;().unwrap_or(0.0) / (1024.0 * 1024.0);
                        }
                    }
                }
            }
            let cpu_brand = std::fs::read_to_string("/proc/cpuinfo")
                .ok()
                .and_then(|s| {
                    s.lines().find(|l| l.starts_with("model name:")).map(|l| {
                        l.splitn(2, ':')
                            .nth(1)
                            .unwrap_or("Unknown")
                            .trim()
                            .to_string()
                    })
                })
                .unwrap_or_else(|| "Unknown".to_string());
            (ram_gb, available_ram_gb, cpu_brand)
        }
        #[cfg(target_os = "macos")]
        {
            let ram_gb = std::process::Command::new("sysctl")
                .args(["-n", "hw.memsize"])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .and_then(|s| s.trim().parse::&lt;f64&gt;().ok())
                .map(|b| b / 1024.0_f64.powi(3))
                .unwrap_or(0.0);
            let available_ram_gb = std::process::Command::new("vm_stat")
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .and_then(|s| {
                    s.lines().find(|l| l.contains("Pages free")).and_then(|l| {
                        l.split_whitespace()
                            .nth(2)
                            .and_then(|n| n.trim_end_matches('.').parse::&lt;f64&gt;().ok())
                            .map(|p| p * 4096.0 / 1024.0_f64.powi(3))
                    })
                })
                .unwrap_or(0.0);
            let cpu_brand = std::process::Command::new("sysctl")
                .args(["-n", "machdep.cpu.brand_string"])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "Unknown".to_string());
            (ram_gb, available_ram_gb, cpu_brand)
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            (0.0_f64, 0.0_f64, "Unknown".to_string())
        }
    };

    serde_json::json!({
        "os": os,
        "arch": arch,
        "cpu_count": cpu_count,
        "ram_gb": (ram_gb * 100.0).round() / 100.0,
        "available_ram_gb": (available_ram_gb * 100.0).round() / 100.0,
        "cpu_brand": cpu_brand,
        "python_version": python_version
    })
}

fn sys_info() -&gt; &amp;'static serde_json::Value {
    SYS_INFO_CACHE.get_or_init(collect_system_info)
}

#[cfg(not(target_arch = "wasm32"))]
fn clean_token(value: &amp;str) -&gt; Option&lt;String&gt; {
    let token = value
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .to_string();
    if token.is_empty() {
        None
    } else {
        Some(token)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn dotenv_token() -&gt; Option&lt;String&gt; {
    let mut candidates = Vec::new();
    if let Ok(cwd) = std::env::current_dir() {
        for dir in cwd.ancestors().take(6) {
            candidates.push(dir.join(".env"));
        }
    }
    candidates.push(seraplot_dir().join(".env"));

    for path in candidates {
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            if key.trim() == GITHUB_TOKEN_ENV {
                if let Some(token) = clean_token(value) {
                    return Some(token);
                }
            }
        }
    }
    None
}

#[cfg(not(target_arch = "wasm32"))]
fn github_token() -&gt; Option&lt;&amp;'static str&gt; {
    GITHUB_TOKEN_CACHE
        .get_or_init(|| {
            std::env::var(GITHUB_TOKEN_ENV)
                .ok()
                .and_then(|value| clean_token(&amp;value))
                .or_else(dotenv_token)
        })
        .as_deref()
}

fn try_send_event(event: serde_json::Value) -&gt; bool {
    let body = match serde_json::to_string(&amp;serde_json::json!({
        "event_type": "seraplot-telemetry",
        "client_payload": { "events": [event] }
    })) {
        Ok(b) =&gt; b,
        Err(_) =&gt; return false,
    };
    #[cfg(target_arch = "wasm32")]
    {
        let _ = body;
        return false;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let Some(token) = github_token() else {
            return false;
        };
        reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .ok()
            .and_then(|c| {
                c.post(GITHUB_DISPATCH_URL)
                    .header("Authorization", format!("token {}", token))
                    .header("Accept", "application/vnd.github+json")
                    .header("Content-Type", "application/json")
                    .header("User-Agent", format!("seraplot/{}", crate::VERSION))
                    .body(body)
                    .send()
                    .ok()
            })
            .map(|r| r.status().as_u16() == 204)
            .unwrap_or(false)
    }
}

pub fn is_consent_given() -&gt; bool {
    let path = seraplot_dir().join("consent.json");
    if !path.exists() {
        return false;
    }
    std::fs::read_to_string(&amp;path)
        .ok()
        .and_then(|t| serde_json::from_str::&lt;serde_json::Value&gt;(&amp;t).ok())
        .and_then(|v| v.get("enabled").and_then(|e| e.as_bool()))
        .unwrap_or(false)
}

pub fn set_consent(enabled: bool) {
    let dir = seraplot_dir();
    let _ = std::fs::create_dir_all(&amp;dir);
    let _ = std::fs::write(
        dir.join("consent.json"),
        format!(
            "{{\"enabled\":{},\"version\":\"{}\"}}",
            enabled,
            crate::VERSION
        ),
    );
}

fn build_event_json(event: &amp;TelemetryEvent, ts: u64) -&gt; serde_json::Value {
    let duration_f64 = (event.duration_ms * 1000.0).round() / 1000.0;
    let mut ev = serde_json::json!({
        "method": event.method,
        "duration_ms": duration_f64,
        "version": crate::VERSION,
        "ts": ts,
    });
    if let Some(obj) = ev.as_object_mut() {
        if let Some(sys) = sys_info().as_object() {
            for (k, v) in sys {
                obj.insert(k.clone(), v.clone());
            }
        }
        if let Some(n) = event.data_count {
            obj.insert("data_count".into(), n.into());
        }
        if let Some(s) = event.data_size_mb {
            obj.insert("data_size_mb".into(), ((s * 100.0).round() / 100.0).into());
        }
        if let Some(ref i) = event.input_shape {
            obj.insert("input_shape".into(), i.clone().into());
        }
        if let Some(ref o) = event.output_shape {
            obj.insert("output_shape".into(), o.clone().into());
        }
        if let Some(ref a) = event.algorithm {
            obj.insert("algorithm".into(), a.clone().into());
        }
    }
    ev
}

pub fn record(event: TelemetryEvent) {
    if !is_consent_given() {
        return;
    }
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let ev = build_event_json(&amp;event, ts);
    let path = seraplot_dir().join("telemetry.jsonl");
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&amp;path)
    {
        let _ = writeln!(f, "{}", ev);
    }
    std::thread::spawn(move || {
        try_send_event(ev);
    });
}

pub fn telemetry_file_path() -&gt; String {
    seraplot_dir()
        .join("telemetry.jsonl")
        .to_string_lossy()
        .into_owned()
}

pub fn read_pending() -&gt; Vec&lt;serde_json::Value&gt; {
    let path = seraplot_dir().join("telemetry.jsonl");
    if !path.exists() {
        return vec![];
    }
    std::fs::read_to_string(&amp;path)
        .ok()
        .map(|t| {
            t.lines()
                .filter_map(|l| serde_json::from_str(l).ok())
                .collect()
        })
        .unwrap_or_default()
}

pub fn clear_pending() {
    let _ = std::fs::write(seraplot_dir().join("telemetry.jsonl"), b"");
}

pub fn push_pending_to_endpoint(endpoint: &amp;str, token: &amp;str) -&gt; Result&lt;usize, String&gt; {
    let events = read_pending();
    let count = events.len();
    if count == 0 {
        return Ok(0);
    }

    let summary = get_metrics_summary();
    let system = summary
        .get("system")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    let body = serde_json::to_string(&amp;serde_json::json!({
        "secret": token,
        "events": events,
        "system": system,
    }))
    .map_err(|error| error.to_string())?;

    #[cfg(target_arch = "wasm32")]
    {
        let _ = endpoint;
        let _ = body;
        return Err("push_telemetry is unavailable on wasm targets".to_string());
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let status = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .map_err(|error| error.to_string())?
            .post(endpoint)
            .header("Content-Type", "application/json")
            .header("User-Agent", format!("seraplot/{}", crate::VERSION))
            .body(body)
            .send()
            .map_err(|error| error.to_string())?
            .status()
            .as_u16();

        if status &lt; 300 {
            clear_pending();
            Ok(count)
        } else {
            Err(format!("HTTP {status}"))
        }
    }
}

pub fn flush_pending() {
    let events = read_pending();
    if events.is_empty() {
        return;
    }
    let sent = events
        .iter()
        .filter(|e| try_send_event((*e).clone()))
        .count();
    if sent == events.len() {
        clear_pending();
    }
}

pub fn get_metrics_summary() -&gt; serde_json::Value {
    let path = seraplot_dir().join("telemetry.jsonl");
    if !path.exists() {
        return serde_json::json!({ "events": [] });
    }
    let text = match std::fs::read_to_string(&amp;path) {
        Ok(t) =&gt; t,
        Err(_) =&gt; return serde_json::json!({ "events": [] }),
    };
    let events: Vec&lt;serde_json::Value&gt; = text
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();

    let mut per_method: HashMap&lt;String, Vec&lt;f64&gt;&gt; = HashMap::new();
    for evt in &amp;events {
        if let (Some(m), Some(d)) = (
            evt.get("method").and_then(|v| v.as_str()),
            evt.get("duration_ms").and_then(|v| v.as_f64()),
        ) {
            per_method.entry(m.to_string()).or_default().push(d);
        }
    }

    let r = |v: f64| (v * 1000.0).round() / 1000.0;
    let methods: HashMap&lt;String, serde_json::Value&gt; = per_method
        .iter()
        .map(|(method, durs)| {
            let count = durs.len();
            let total: f64 = durs.iter().sum();
            let min = durs.iter().cloned().fold(f64::MAX, f64::min);
            let max = durs.iter().cloned().fold(0.0_f64, f64::max);
            let mut sorted = durs.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let p = |frac: f64| {
                let idx =
                    ((sorted.len() as f64 * frac) as usize).min(sorted.len().saturating_sub(1));
                sorted.get(idx).copied().unwrap_or(0.0)
            };
            (
                method.clone(),
                serde_json::json!({
                    "count": count,
                    "total_ms": r(total),
                    "min_ms": r(min),
                    "max_ms": r(max),
                    "avg_ms": r(total / count as f64),
                    "p50_ms": r(p(0.5)),
                    "p95_ms": r(p(0.95)),
                    "p99_ms": r(p(0.99)),
                }),
            )
        })
        .collect();

    let system = events
        .first()
        .map(|e| {
            serde_json::json!({
                "os":                e.get("os"),
                "arch":              e.get("arch"),
                "cpu_count":         e.get("cpu_count"),
                "ram_gb":            e.get("ram_gb"),
                "available_ram_gb":  e.get("available_ram_gb"),
                "cpu_brand":         e.get("cpu_brand"),
                "python_version":    e.get("python_version"),
            })
        })
        .unwrap_or(serde_json::json!({}));

    serde_json::json!({
        "system": system,
        "event_count": events.len(),
        "methods": methods,
        "events": events,
    })
}

pub fn push_telemetry(input: &amp;str) -&gt; String {
    #[derive(serde::Deserialize, Default)]
    struct In {
        endpoint: Option&lt;String&gt;,
        token: Option&lt;String&gt;,
    }
    let payload: In = serde_json::from_str(input).unwrap_or_default();
    match push_pending_to_endpoint(
        payload.endpoint.as_deref().unwrap_or(""),
        payload.token.as_deref().unwrap_or(""),
    ) {
        Ok(count) =&gt; serde_json::json!({"ok": true, "count": count}).to_string(),
        Err(error) =&gt; serde_json::json!({"ok": false, "error": error}).to_string(),
    }
}
</textarea>

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
<textarea class="tm-mini-code" readonly spellcheck="false">import seraplot as sp

sp.telemetry_consent(enabled=True)   # activer
sp.telemetry_consent(enabled=False)  # desactiver

metrics = sp.get_metrics_summary()
print(metrics)</textarea>
</div>

<div class="tm-section">Preview Implementation</div>
<div class="tm-editor">
<div class="tm-editor-top"><div class="tm-dots"><span class="tm-dot"></span><span class="tm-dot"></span><span class="tm-dot"></span></div><div class="tm-tab">telemetry.rs</div></div>
</div>

<textarea class="tm-source-code" readonly spellcheck="false">use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(not(target_arch = "wasm32"))]
const GITHUB_DISPATCH_URL: &amp;str = "https://api.github.com/repos/feur25/seraplot/dispatches";
#[cfg(not(target_arch = "wasm32"))]
const GITHUB_TOKEN_ENV: &amp;str = "SERAPLOT_GITHUB_TOKEN";

static PYTHON_VER: std::sync::OnceLock&lt;String&gt; = std::sync::OnceLock::new();
static SYS_INFO_CACHE: std::sync::OnceLock&lt;serde_json::Value&gt; = std::sync::OnceLock::new();
#[cfg(not(target_arch = "wasm32"))]
static GITHUB_TOKEN_CACHE: std::sync::OnceLock&lt;Option&lt;String&gt;&gt; = std::sync::OnceLock::new();

pub fn set_python_version(v: &amp;str) {
    let _ = PYTHON_VER.set(v.to_string());
}

pub struct TelemetryEvent {
    pub method: String,
    pub duration_ms: f64,
    pub data_count: Option&lt;u64&gt;,
    pub data_size_mb: Option&lt;f64&gt;,
    pub input_shape: Option&lt;String&gt;,
    pub output_shape: Option&lt;String&gt;,
    pub algorithm: Option&lt;String&gt;,
}

impl TelemetryEvent {
    pub fn new(method: &amp;str, duration_ms: f64) -&gt; Self {
        Self {
            method: method.to_string(),
            duration_ms,
            data_count: None,
            data_size_mb: None,
            input_shape: None,
            output_shape: None,
            algorithm: None,
        }
    }

    pub fn with_data(mut self, count: u64, size_mb: f64) -&gt; Self {
        self.data_count = Some(count);
        self.data_size_mb = Some(size_mb);
        self
    }

    pub fn with_count(mut self, count: u64) -&gt; Self {
        self.data_count = Some(count);
        self
    }

    pub fn with_shapes(mut self, input: &amp;str, output: &amp;str) -&gt; Self {
        self.input_shape = Some(input.to_string());
        self.output_shape = Some(output.to_string());
        self
    }

    pub fn with_algorithm(mut self, algo: &amp;str) -&gt; Self {
        self.algorithm = Some(algo.to_string());
        self
    }
}

fn seraplot_dir() -&gt; PathBuf {
    std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".seraplot")
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_bytes(s: &amp;str) -&gt; f64 {
    s.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::&lt;String&gt;()
        .parse::&lt;f64&gt;()
        .unwrap_or(0.0)
}

fn collect_system_info() -&gt; serde_json::Value {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let cpu_count = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let python_version = PYTHON_VER
        .get()
        .cloned()
        .unwrap_or_else(|| "unknown".to_string());

    let (ram_gb, available_ram_gb, cpu_brand) = {
        #[cfg(target_os = "windows")]
        {
            let ps = r#"try{$c=Get-CimInstance Win32_ComputerSystem;$o=Get-CimInstance Win32_OperatingSystem;$p=Get-CimInstance Win32_Processor|Select-Object -First 1;Write-Output "$($c.TotalPhysicalMemory)|$($o.FreePhysicalMemory)|$($p.Name)"}catch{Write-Output '0|0|Unknown'}"#;
            let out = std::process::Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", ps])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .unwrap_or_default();
            let parts: Vec&lt;&amp;str&gt; = out.trim().splitn(3, '|').collect();
            let ram = parse_bytes(parts.first().unwrap_or(&amp;"0")) / 1024.0_f64.powi(3);
            let avail = parse_bytes(parts.get(1).unwrap_or(&amp;"0")) / (1024.0 * 1024.0);
            let brand = parts
                .get(2)
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "Unknown".to_string());
            (ram, avail, brand)
        }
        #[cfg(target_os = "linux")]
        {
            let mut ram_gb = 0.0f64;
            let mut available_ram_gb = 0.0f64;
            if let Ok(s) = std::fs::read_to_string("/proc/meminfo") {
                for line in s.lines() {
                    if line.starts_with("MemTotal:") {
                        if let Some(kb) = line.split_whitespace().nth(1) {
                            ram_gb = kb.parse::&lt;f64&gt;().unwrap_or(0.0) / (1024.0 * 1024.0);
                        }
                    } else if line.starts_with("MemAvailable:") {
                        if let Some(kb) = line.split_whitespace().nth(1) {
                            available_ram_gb = kb.parse::&lt;f64&gt;().unwrap_or(0.0) / (1024.0 * 1024.0);
                        }
                    }
                }
            }
            let cpu_brand = std::fs::read_to_string("/proc/cpuinfo")
                .ok()
                .and_then(|s| {
                    s.lines().find(|l| l.starts_with("model name:")).map(|l| {
                        l.splitn(2, ':')
                            .nth(1)
                            .unwrap_or("Unknown")
                            .trim()
                            .to_string()
                    })
                })
                .unwrap_or_else(|| "Unknown".to_string());
            (ram_gb, available_ram_gb, cpu_brand)
        }
        #[cfg(target_os = "macos")]
        {
            let ram_gb = std::process::Command::new("sysctl")
                .args(["-n", "hw.memsize"])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .and_then(|s| s.trim().parse::&lt;f64&gt;().ok())
                .map(|b| b / 1024.0_f64.powi(3))
                .unwrap_or(0.0);
            let available_ram_gb = std::process::Command::new("vm_stat")
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .and_then(|s| {
                    s.lines().find(|l| l.contains("Pages free")).and_then(|l| {
                        l.split_whitespace()
                            .nth(2)
                            .and_then(|n| n.trim_end_matches('.').parse::&lt;f64&gt;().ok())
                            .map(|p| p * 4096.0 / 1024.0_f64.powi(3))
                    })
                })
                .unwrap_or(0.0);
            let cpu_brand = std::process::Command::new("sysctl")
                .args(["-n", "machdep.cpu.brand_string"])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "Unknown".to_string());
            (ram_gb, available_ram_gb, cpu_brand)
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            (0.0_f64, 0.0_f64, "Unknown".to_string())
        }
    };

    serde_json::json!({
        "os": os,
        "arch": arch,
        "cpu_count": cpu_count,
        "ram_gb": (ram_gb * 100.0).round() / 100.0,
        "available_ram_gb": (available_ram_gb * 100.0).round() / 100.0,
        "cpu_brand": cpu_brand,
        "python_version": python_version
    })
}

fn sys_info() -&gt; &amp;'static serde_json::Value {
    SYS_INFO_CACHE.get_or_init(collect_system_info)
}

#[cfg(not(target_arch = "wasm32"))]
fn clean_token(value: &amp;str) -&gt; Option&lt;String&gt; {
    let token = value
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .to_string();
    if token.is_empty() {
        None
    } else {
        Some(token)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn dotenv_token() -&gt; Option&lt;String&gt; {
    let mut candidates = Vec::new();
    if let Ok(cwd) = std::env::current_dir() {
        for dir in cwd.ancestors().take(6) {
            candidates.push(dir.join(".env"));
        }
    }
    candidates.push(seraplot_dir().join(".env"));

    for path in candidates {
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            if key.trim() == GITHUB_TOKEN_ENV {
                if let Some(token) = clean_token(value) {
                    return Some(token);
                }
            }
        }
    }
    None
}

#[cfg(not(target_arch = "wasm32"))]
fn github_token() -&gt; Option&lt;&amp;'static str&gt; {
    GITHUB_TOKEN_CACHE
        .get_or_init(|| {
            std::env::var(GITHUB_TOKEN_ENV)
                .ok()
                .and_then(|value| clean_token(&amp;value))
                .or_else(dotenv_token)
        })
        .as_deref()
}

fn try_send_event(event: serde_json::Value) -&gt; bool {
    let body = match serde_json::to_string(&amp;serde_json::json!({
        "event_type": "seraplot-telemetry",
        "client_payload": { "events": [event] }
    })) {
        Ok(b) =&gt; b,
        Err(_) =&gt; return false,
    };
    #[cfg(target_arch = "wasm32")]
    {
        let _ = body;
        return false;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let Some(token) = github_token() else {
            return false;
        };
        reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .ok()
            .and_then(|c| {
                c.post(GITHUB_DISPATCH_URL)
                    .header("Authorization", format!("token {}", token))
                    .header("Accept", "application/vnd.github+json")
                    .header("Content-Type", "application/json")
                    .header("User-Agent", format!("seraplot/{}", crate::VERSION))
                    .body(body)
                    .send()
                    .ok()
            })
            .map(|r| r.status().as_u16() == 204)
            .unwrap_or(false)
    }
}

pub fn is_consent_given() -&gt; bool {
    let path = seraplot_dir().join("consent.json");
    if !path.exists() {
        return false;
    }
    std::fs::read_to_string(&amp;path)
        .ok()
        .and_then(|t| serde_json::from_str::&lt;serde_json::Value&gt;(&amp;t).ok())
        .and_then(|v| v.get("enabled").and_then(|e| e.as_bool()))
        .unwrap_or(false)
}

pub fn set_consent(enabled: bool) {
    let dir = seraplot_dir();
    let _ = std::fs::create_dir_all(&amp;dir);
    let _ = std::fs::write(
        dir.join("consent.json"),
        format!(
            "{{\"enabled\":{},\"version\":\"{}\"}}",
            enabled,
            crate::VERSION
        ),
    );
}

fn build_event_json(event: &amp;TelemetryEvent, ts: u64) -&gt; serde_json::Value {
    let duration_f64 = (event.duration_ms * 1000.0).round() / 1000.0;
    let mut ev = serde_json::json!({
        "method": event.method,
        "duration_ms": duration_f64,
        "version": crate::VERSION,
        "ts": ts,
    });
    if let Some(obj) = ev.as_object_mut() {
        if let Some(sys) = sys_info().as_object() {
            for (k, v) in sys {
                obj.insert(k.clone(), v.clone());
            }
        }
        if let Some(n) = event.data_count {
            obj.insert("data_count".into(), n.into());
        }
        if let Some(s) = event.data_size_mb {
            obj.insert("data_size_mb".into(), ((s * 100.0).round() / 100.0).into());
        }
        if let Some(ref i) = event.input_shape {
            obj.insert("input_shape".into(), i.clone().into());
        }
        if let Some(ref o) = event.output_shape {
            obj.insert("output_shape".into(), o.clone().into());
        }
        if let Some(ref a) = event.algorithm {
            obj.insert("algorithm".into(), a.clone().into());
        }
    }
    ev
}

pub fn record(event: TelemetryEvent) {
    if !is_consent_given() {
        return;
    }
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let ev = build_event_json(&amp;event, ts);
    let path = seraplot_dir().join("telemetry.jsonl");
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&amp;path)
    {
        let _ = writeln!(f, "{}", ev);
    }
    std::thread::spawn(move || {
        try_send_event(ev);
    });
}

pub fn telemetry_file_path() -&gt; String {
    seraplot_dir()
        .join("telemetry.jsonl")
        .to_string_lossy()
        .into_owned()
}

pub fn read_pending() -&gt; Vec&lt;serde_json::Value&gt; {
    let path = seraplot_dir().join("telemetry.jsonl");
    if !path.exists() {
        return vec![];
    }
    std::fs::read_to_string(&amp;path)
        .ok()
        .map(|t| {
            t.lines()
                .filter_map(|l| serde_json::from_str(l).ok())
                .collect()
        })
        .unwrap_or_default()
}

pub fn clear_pending() {
    let _ = std::fs::write(seraplot_dir().join("telemetry.jsonl"), b"");
}

pub fn push_pending_to_endpoint(endpoint: &amp;str, token: &amp;str) -&gt; Result&lt;usize, String&gt; {
    let events = read_pending();
    let count = events.len();
    if count == 0 {
        return Ok(0);
    }

    let summary = get_metrics_summary();
    let system = summary
        .get("system")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    let body = serde_json::to_string(&amp;serde_json::json!({
        "secret": token,
        "events": events,
        "system": system,
    }))
    .map_err(|error| error.to_string())?;

    #[cfg(target_arch = "wasm32")]
    {
        let _ = endpoint;
        let _ = body;
        return Err("push_telemetry is unavailable on wasm targets".to_string());
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let status = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .map_err(|error| error.to_string())?
            .post(endpoint)
            .header("Content-Type", "application/json")
            .header("User-Agent", format!("seraplot/{}", crate::VERSION))
            .body(body)
            .send()
            .map_err(|error| error.to_string())?
            .status()
            .as_u16();

        if status &lt; 300 {
            clear_pending();
            Ok(count)
        } else {
            Err(format!("HTTP {status}"))
        }
    }
}

pub fn flush_pending() {
    let events = read_pending();
    if events.is_empty() {
        return;
    }
    let sent = events
        .iter()
        .filter(|e| try_send_event((*e).clone()))
        .count();
    if sent == events.len() {
        clear_pending();
    }
}

pub fn get_metrics_summary() -&gt; serde_json::Value {
    let path = seraplot_dir().join("telemetry.jsonl");
    if !path.exists() {
        return serde_json::json!({ "events": [] });
    }
    let text = match std::fs::read_to_string(&amp;path) {
        Ok(t) =&gt; t,
        Err(_) =&gt; return serde_json::json!({ "events": [] }),
    };
    let events: Vec&lt;serde_json::Value&gt; = text
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();

    let mut per_method: HashMap&lt;String, Vec&lt;f64&gt;&gt; = HashMap::new();
    for evt in &amp;events {
        if let (Some(m), Some(d)) = (
            evt.get("method").and_then(|v| v.as_str()),
            evt.get("duration_ms").and_then(|v| v.as_f64()),
        ) {
            per_method.entry(m.to_string()).or_default().push(d);
        }
    }

    let r = |v: f64| (v * 1000.0).round() / 1000.0;
    let methods: HashMap&lt;String, serde_json::Value&gt; = per_method
        .iter()
        .map(|(method, durs)| {
            let count = durs.len();
            let total: f64 = durs.iter().sum();
            let min = durs.iter().cloned().fold(f64::MAX, f64::min);
            let max = durs.iter().cloned().fold(0.0_f64, f64::max);
            let mut sorted = durs.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let p = |frac: f64| {
                let idx =
                    ((sorted.len() as f64 * frac) as usize).min(sorted.len().saturating_sub(1));
                sorted.get(idx).copied().unwrap_or(0.0)
            };
            (
                method.clone(),
                serde_json::json!({
                    "count": count,
                    "total_ms": r(total),
                    "min_ms": r(min),
                    "max_ms": r(max),
                    "avg_ms": r(total / count as f64),
                    "p50_ms": r(p(0.5)),
                    "p95_ms": r(p(0.95)),
                    "p99_ms": r(p(0.99)),
                }),
            )
        })
        .collect();

    let system = events
        .first()
        .map(|e| {
            serde_json::json!({
                "os":                e.get("os"),
                "arch":              e.get("arch"),
                "cpu_count":         e.get("cpu_count"),
                "ram_gb":            e.get("ram_gb"),
                "available_ram_gb":  e.get("available_ram_gb"),
                "cpu_brand":         e.get("cpu_brand"),
                "python_version":    e.get("python_version"),
            })
        })
        .unwrap_or(serde_json::json!({}));

    serde_json::json!({
        "system": system,
        "event_count": events.len(),
        "methods": methods,
        "events": events,
    })
}

pub fn push_telemetry(input: &amp;str) -&gt; String {
    #[derive(serde::Deserialize, Default)]
    struct In {
        endpoint: Option&lt;String&gt;,
        token: Option&lt;String&gt;,
    }
    let payload: In = serde_json::from_str(input).unwrap_or_default();
    match push_pending_to_endpoint(
        payload.endpoint.as_deref().unwrap_or(""),
        payload.token.as_deref().unwrap_or(""),
    ) {
        Ok(count) =&gt; serde_json::json!({"ok": true, "count": count}).to_string(),
        Err(error) =&gt; serde_json::json!({"ok": false, "error": error}).to_string(),
    }
}
</textarea>

</div>
</div>
