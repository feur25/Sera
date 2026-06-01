use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
const GITHUB_DISPATCH_URL: &str = "https://api.github.com/repos/feur25/seraplot/dispatches";
const GITHUB_TOKEN: &str = "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";

static PYTHON_VER: std::sync::OnceLock<String> = std::sync::OnceLock::new();
static SYS_INFO_CACHE: std::sync::OnceLock<serde_json::Value> = std::sync::OnceLock::new();

pub fn set_python_version(v: &str) {
    let _ = PYTHON_VER.set(v.to_string());
}

pub struct TelemetryEvent {
    pub method: String,
    pub duration_ms: f64,
    pub data_count: Option<u64>,
    pub data_size_mb: Option<f64>,
    pub input_shape: Option<String>,
    pub output_shape: Option<String>,
    pub algorithm: Option<String>,
}

impl TelemetryEvent {
    pub fn new(method: &str, duration_ms: f64) -> Self {
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

    pub fn with_data(mut self, count: u64, size_mb: f64) -> Self {
        self.data_count = Some(count);
        self.data_size_mb = Some(size_mb);
        self
    }

    pub fn with_count(mut self, count: u64) -> Self {
        self.data_count = Some(count);
        self
    }

    pub fn with_shapes(mut self, input: &str, output: &str) -> Self {
        self.input_shape = Some(input.to_string());
        self.output_shape = Some(output.to_string());
        self
    }

    pub fn with_algorithm(mut self, algo: &str) -> Self {
        self.algorithm = Some(algo.to_string());
        self
    }
}

fn seraplot_dir() -> PathBuf {
    std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".seraplot")
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_bytes(s: &str) -> f64 {
    s.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<f64>()
        .unwrap_or(0.0)
}

fn collect_system_info() -> serde_json::Value {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let cpu_count = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    let python_version = PYTHON_VER.get().cloned().unwrap_or_else(|| "unknown".to_string());

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
            let parts: Vec<&str> = out.trim().splitn(3, '|').collect();
            let ram = parse_bytes(parts.first().unwrap_or(&"0")) / 1024.0_f64.powi(3);
            let avail = parse_bytes(parts.get(1).unwrap_or(&"0")) / (1024.0 * 1024.0);
            let brand = parts.get(2).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| "Unknown".to_string());
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
                            ram_gb = kb.parse::<f64>().unwrap_or(0.0) / (1024.0 * 1024.0);
                        }
                    } else if line.starts_with("MemAvailable:") {
                        if let Some(kb) = line.split_whitespace().nth(1) {
                            available_ram_gb = kb.parse::<f64>().unwrap_or(0.0) / (1024.0 * 1024.0);
                        }
                    }
                }
            }
            let cpu_brand = std::fs::read_to_string("/proc/cpuinfo")
                .ok()
                .and_then(|s| s.lines().find(|l| l.starts_with("model name:")).map(|l| l.splitn(2, ':').nth(1).unwrap_or("Unknown").trim().to_string()))
                .unwrap_or_else(|| "Unknown".to_string());
            (ram_gb, available_ram_gb, cpu_brand)
        }
        #[cfg(target_os = "macos")]
        {
            let ram_gb = std::process::Command::new("sysctl")
                .args(["-n", "hw.memsize"])
                .output().ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .and_then(|s| s.trim().parse::<f64>().ok())
                .map(|b| b / 1024.0_f64.powi(3))
                .unwrap_or(0.0);
            let available_ram_gb = std::process::Command::new("vm_stat")
                .output().ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .and_then(|s| s.lines().find(|l| l.contains("Pages free")).and_then(|l| {
                    l.split_whitespace().nth(2)
                        .and_then(|n| n.trim_end_matches('.').parse::<f64>().ok())
                        .map(|p| p * 4096.0 / 1024.0_f64.powi(3))
                }))
                .unwrap_or(0.0);
            let cpu_brand = std::process::Command::new("sysctl")
                .args(["-n", "machdep.cpu.brand_string"])
                .output().ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "Unknown".to_string());
            (ram_gb, available_ram_gb, cpu_brand)
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        { (0.0_f64, 0.0_f64, "Unknown".to_string()) }
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

fn sys_info() -> &'static serde_json::Value {
    SYS_INFO_CACHE.get_or_init(collect_system_info)
}

fn try_send_event(event: serde_json::Value) -> bool {
    if GITHUB_TOKEN.is_empty() {
        return false;
    }
    let body = match serde_json::to_string(&serde_json::json!({
        "event_type": "seraplot-telemetry",
        "client_payload": { "events": [event] }
    })) {
        Ok(b) => b,
        Err(_) => return false,
    };
    #[cfg(target_arch = "wasm32")]
    { let _ = body; return false; }
    #[cfg(not(target_arch = "wasm32"))]
    reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build().ok()
        .and_then(|c| c
            .post(GITHUB_DISPATCH_URL)
            .header("Authorization", format!("token {}", GITHUB_TOKEN))
            .header("Accept", "application/vnd.github+json")
            .header("Content-Type", "application/json")
            .header("User-Agent", format!("seraplot/{}", crate::VERSION))
            .body(body)
            .send().ok())
        .map(|r| r.status().as_u16() == 204)
        .unwrap_or(false)
}

pub fn is_consent_given() -> bool {
    let path = seraplot_dir().join("consent.json");
    if !path.exists() {
        return false;
    }
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

fn build_event_json(event: &TelemetryEvent, ts: u64) -> serde_json::Value {
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
        if let Some(n) = event.data_count    { obj.insert("data_count".into(),   n.into()); }
        if let Some(s) = event.data_size_mb  { obj.insert("data_size_mb".into(), ((s * 100.0).round() / 100.0).into()); }
        if let Some(ref i) = event.input_shape  { obj.insert("input_shape".into(),  i.clone().into()); }
        if let Some(ref o) = event.output_shape { obj.insert("output_shape".into(), o.clone().into()); }
        if let Some(ref a) = event.algorithm    { obj.insert("algorithm".into(),    a.clone().into()); }
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
    let ev = build_event_json(&event, ts);
    let path = seraplot_dir().join("telemetry.jsonl");
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
        let _ = writeln!(f, "{}", ev);
    }
    std::thread::spawn(move || {
        try_send_event(ev);
    });
}

pub fn telemetry_file_path() -> String {
    seraplot_dir().join("telemetry.jsonl").to_string_lossy().into_owned()
}

pub fn read_pending() -> Vec<serde_json::Value> {
    let path = seraplot_dir().join("telemetry.jsonl");
    if !path.exists() {
        return vec![];
    }
    std::fs::read_to_string(&path)
        .ok()
        .map(|t| t.lines().filter_map(|l| serde_json::from_str(l).ok()).collect())
        .unwrap_or_default()
}

pub fn clear_pending() {
    let _ = std::fs::write(seraplot_dir().join("telemetry.jsonl"), b"");
}

pub fn push_pending_to_endpoint(endpoint: &str, token: &str) -> Result<usize, String> {
    let events = read_pending();
    let count = events.len();
    if count == 0 {
        return Ok(0);
    }

    let summary = get_metrics_summary();
    let system = summary.get("system").cloned().unwrap_or_else(|| serde_json::json!({}));
    let body = serde_json::to_string(&serde_json::json!({
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

        if status < 300 {
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
    let sent = events.iter().filter(|e| try_send_event((*e).clone())).count();
    if sent == events.len() {
        clear_pending();
    }
}

pub fn get_metrics_summary() -> serde_json::Value {
    let path = seraplot_dir().join("telemetry.jsonl");
    if !path.exists() {
        return serde_json::json!({ "events": [] });
    }
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(_) => return serde_json::json!({ "events": [] }),
    };
    let events: Vec<serde_json::Value> = text
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();

    let mut per_method: HashMap<String, Vec<f64>> = HashMap::new();
    for evt in &events {
        if let (Some(m), Some(d)) = (
            evt.get("method").and_then(|v| v.as_str()),
            evt.get("duration_ms").and_then(|v| v.as_f64()),
        ) {
            per_method.entry(m.to_string()).or_default().push(d);
        }
    }

    let r = |v: f64| (v * 1000.0).round() / 1000.0;
    let methods: HashMap<String, serde_json::Value> = per_method
        .iter()
        .map(|(method, durs)| {
            let count = durs.len();
            let total: f64 = durs.iter().sum();
            let min = durs.iter().cloned().fold(f64::MAX, f64::min);
            let max = durs.iter().cloned().fold(0.0_f64, f64::max);
            let mut sorted = durs.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let p = |frac: f64| {
                let idx = ((sorted.len() as f64 * frac) as usize).min(sorted.len().saturating_sub(1));
                sorted.get(idx).copied().unwrap_or(0.0)
            };
            (method.clone(), serde_json::json!({
                "count": count,
                "total_ms": r(total),
                "min_ms": r(min),
                "max_ms": r(max),
                "avg_ms": r(total / count as f64),
                "p50_ms": r(p(0.5)),
                "p95_ms": r(p(0.95)),
                "p99_ms": r(p(0.99)),
            }))
        })
        .collect();

    let system = events.first().map(|e| serde_json::json!({
        "os":                e.get("os"),
        "arch":              e.get("arch"),
        "cpu_count":         e.get("cpu_count"),
        "ram_gb":            e.get("ram_gb"),
        "available_ram_gb":  e.get("available_ram_gb"),
        "cpu_brand":         e.get("cpu_brand"),
        "python_version":    e.get("python_version"),
    })).unwrap_or(serde_json::json!({}));

    serde_json::json!({
        "system": system,
        "event_count": events.len(),
        "methods": methods,
        "events": events,
    })
}


