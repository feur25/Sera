use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(not(target_arch = "wasm32"))]
const GITHUB_DISPATCH_URL: &str = "https://api.github.com/repos/feur25/seraplot/dispatches";
#[cfg(not(target_arch = "wasm32"))]
const GITHUB_TOKEN_ENV: &str = "SERAPLOT_GITHUB_TOKEN";

const CONSENT_FILE: &str = "consent.json";
const TELEMETRY_FILE: &str = "telemetry.jsonl";
const UNKNOWN: &str = "Unknown";

static PYTHON_VER: std::sync::OnceLock<String> = std::sync::OnceLock::new();
static SYS_INFO_CACHE: std::sync::OnceLock<serde_json::Value> = std::sync::OnceLock::new();
#[cfg(not(target_arch = "wasm32"))]
static GITHUB_TOKEN_CACHE: std::sync::OnceLock<Option<String>> = std::sync::OnceLock::new();

pub fn set_python_version(v: &str) {
    let _ = PYTHON_VER.set(v.to_string());
}
#[derive(Clone, Debug)]
pub struct TelemetryEvent {
    pub method: String,
    pub duration_ms: f64,
    pub data_count: Option<u64>,
    pub data_size_mb: Option<f64>,
    pub input_shape: Option<String>,
    pub output_shape: Option<String>,
    pub algorithm: Option<String>,
}

#[derive(Clone, Debug)]
pub struct TelemetryEventBuilder {
    event: TelemetryEvent,
}

impl TelemetryEvent {
    pub fn new(method: &str, duration_ms: f64) -> Self {
        Self::builder(method, duration_ms).build()
    }

    pub fn builder(method: &str, duration_ms: f64) -> TelemetryEventBuilder {
        TelemetryEventBuilder {
            event: Self {
                method: method.to_string(),
                duration_ms,
                data_count: None,
                data_size_mb: None,
                input_shape: None,
                output_shape: None,
                algorithm: None,
            },
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

impl TelemetryEventBuilder {
    pub fn data(mut self, count: u64, size_mb: f64) -> Self {
        self.event.data_count = Some(count);
        self.event.data_size_mb = Some(size_mb);
        self
    }

    pub fn count(mut self, count: u64) -> Self {
        self.event.data_count = Some(count);
        self
    }

    pub fn shapes(mut self, input: &str, output: &str) -> Self {
        self.event.input_shape = Some(input.to_string());
        self.event.output_shape = Some(output.to_string());
        self
    }

    pub fn algorithm(mut self, algo: &str) -> Self {
        self.event.algorithm = Some(algo.to_string());
        self
    }

    pub fn build(self) -> TelemetryEvent {
        self.event
    }
}

fn round_to(value: f64, scale: f64) -> f64 {
    (value * scale).round() / scale
}

fn round2(value: f64) -> f64 {
    round_to(value, 100.0)
}

fn round3(value: f64) -> f64 {
    round_to(value, 1000.0)
}

fn seraplot_dir() -> PathBuf {
    std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".seraplot")
}

fn telemetry_path() -> PathBuf {
    seraplot_dir().join(TELEMETRY_FILE)
}

fn consent_path() -> PathBuf {
    seraplot_dir().join(CONSENT_FILE)
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_bytes(s: &str) -> f64 {
    s.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<f64>()
        .unwrap_or(0.0)
}

#[cfg(target_os = "windows")]
fn platform_system_info() -> (f64, f64, String) {
    let ps = r#"try{$c=Get-CimInstance Win32_ComputerSystem;$o=Get-CimInstance Win32_OperatingSystem;$p=Get-CimInstance Win32_Processor|Select-Object -First 1;Write-Output "$($c.TotalPhysicalMemory)|$($o.FreePhysicalMemory)|$($p.Name)"}catch{Write-Output '0|0|Unknown'}"#;
    let out = std::process::Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", ps])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default();
    let parts: Vec<&str> = out.trim().splitn(3, '|').collect();
    let ram_gb = parse_bytes(parts.first().unwrap_or(&"0")) / 1024.0_f64.powi(3);
    let available_ram_gb = parse_bytes(parts.get(1).unwrap_or(&"0")) / (1024.0 * 1024.0);
    let cpu_brand = parts
        .get(2)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| UNKNOWN.to_string());
    (ram_gb, available_ram_gb, cpu_brand)
}

#[cfg(target_os = "linux")]
fn meminfo_gb(text: &str, key: &str) -> f64 {
    text.lines()
        .find(|line| line.starts_with(key))
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|kb| kb.parse::<f64>().ok())
        .map(|kb| kb / (1024.0 * 1024.0))
        .unwrap_or(0.0)
}

#[cfg(target_os = "linux")]
fn platform_system_info() -> (f64, f64, String) {
    let meminfo = std::fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let ram_gb = meminfo_gb(&meminfo, "MemTotal:");
    let available_ram_gb = meminfo_gb(&meminfo, "MemAvailable:");
    let cpu_brand = std::fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|text| {
            text.lines()
                .find(|line| line.starts_with("model name:"))
                .and_then(|line| line.split_once(':'))
                .map(|(_, brand)| brand.trim().to_string())
        })
        .filter(|brand| !brand.is_empty())
        .unwrap_or_else(|| UNKNOWN.to_string());
    (ram_gb, available_ram_gb, cpu_brand)
}

#[cfg(target_os = "macos")]
fn command_stdout(command: &str, args: &[&str]) -> Option<String> {
    std::process::Command::new(command)
        .args(args)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
}

#[cfg(target_os = "macos")]
fn platform_system_info() -> (f64, f64, String) {
    let ram_gb = command_stdout("sysctl", &["-n", "hw.memsize"])
        .and_then(|s| s.trim().parse::<f64>().ok())
        .map(|bytes| bytes / 1024.0_f64.powi(3))
        .unwrap_or(0.0);
    let available_ram_gb = command_stdout("vm_stat", &[])
        .and_then(|text| {
            text.lines()
                .find(|line| line.contains("Pages free"))
                .and_then(|line| line.split_whitespace().nth(2))
                .and_then(|pages| pages.trim_end_matches('.').parse::<f64>().ok())
                .map(|pages| pages * 4096.0 / 1024.0_f64.powi(3))
        })
        .unwrap_or(0.0);
    let cpu_brand = command_stdout("sysctl", &["-n", "machdep.cpu.brand_string"])
        .map(|s| s.trim().to_string())
        .filter(|brand| !brand.is_empty())
        .unwrap_or_else(|| UNKNOWN.to_string());
    (ram_gb, available_ram_gb, cpu_brand)
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn platform_system_info() -> (f64, f64, String) {
    (0.0, 0.0, UNKNOWN.to_string())
}

fn collect_system_info() -> serde_json::Value {
    let (ram_gb, available_ram_gb, cpu_brand) = platform_system_info();
    serde_json::json!({
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "cpu_count": std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1),
        "ram_gb": round2(ram_gb),
        "available_ram_gb": round2(available_ram_gb),
        "cpu_brand": cpu_brand,
        "python_version": PYTHON_VER.get().cloned().unwrap_or_else(|| "unknown".to_string())
    })
}

fn sys_info() -> &'static serde_json::Value {
    SYS_INFO_CACHE.get_or_init(collect_system_info)
}

#[cfg(not(target_arch = "wasm32"))]
fn clean_token(value: &str) -> Option<String> {
    let token = value
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .to_string();
    (!token.is_empty()).then_some(token)
}

#[cfg(not(target_arch = "wasm32"))]
fn dotenv_candidates() -> Vec<PathBuf> {
    let cwd_candidates = std::env::current_dir()
        .ok()
        .into_iter()
        .flat_map(|cwd| {
            cwd.ancestors()
                .take(6)
                .map(Path::to_path_buf)
                .collect::<Vec<_>>()
        })
        .map(|dir| dir.join(".env"));
    cwd_candidates
        .chain([seraplot_dir().join(".env")])
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
fn token_from_dotenv(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok().and_then(|text| {
        text.lines().find_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                return None;
            }
            let (key, value) = line.split_once('=')?;
            (key.trim() == GITHUB_TOKEN_ENV)
                .then(|| clean_token(value))
                .flatten()
        })
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn dotenv_token() -> Option<String> {
    dotenv_candidates()
        .iter()
        .find_map(|path| token_from_dotenv(path))
}

#[cfg(not(target_arch = "wasm32"))]
fn github_token() -> Option<&'static str> {
    GITHUB_TOKEN_CACHE
        .get_or_init(|| {
            std::env::var(GITHUB_TOKEN_ENV)
                .ok()
                .and_then(|value| clean_token(&value))
                .or_else(dotenv_token)
        })
        .as_deref()
}

fn github_dispatch_body(event: serde_json::Value) -> Option<String> {
    serde_json::to_string(&serde_json::json!({
        "event_type": "seraplot-telemetry",
        "client_payload": { "events": [event] }
    }))
    .ok()
}

fn try_send_event(event: serde_json::Value) -> bool {
    let Some(body) = github_dispatch_body(event) else {
        return false;
    };

    #[cfg(target_arch = "wasm32")]
    {
        let _ = body;
        false
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
            .and_then(|client| {
                client
                    .post(GITHUB_DISPATCH_URL)
                    .header("Authorization", format!("token {}", token))
                    .header("Accept", "application/vnd.github+json")
                    .header("Content-Type", "application/json")
                    .header("User-Agent", format!("seraplot/{}", crate::VERSION))
                    .body(body)
                    .send()
                    .ok()
            })
            .map(|response| response.status().as_u16() == 204)
            .unwrap_or(false)
    }
}

pub fn is_consent_given() -> bool {
    let path = consent_path();
    if !path.exists() {
        return false;
    }
    std::fs::read_to_string(path)
        .ok()
        .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
        .and_then(|value| value.get("enabled").and_then(|enabled| enabled.as_bool()))
        .unwrap_or(false)
}

pub fn set_consent(enabled: bool) {
    let dir = seraplot_dir();
    let payload = serde_json::json!({
        "enabled": enabled,
        "version": crate::VERSION
    });
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(dir.join(CONSENT_FILE), payload.to_string());
}

fn event_optional_fields(event: &TelemetryEvent) -> Vec<(&'static str, serde_json::Value)> {
    [
        event.data_count.map(|value| ("data_count", value.into())),
        event
            .data_size_mb
            .map(|value| ("data_size_mb", round2(value).into())),
        event
            .input_shape
            .as_ref()
            .map(|value| ("input_shape", value.clone().into())),
        event
            .output_shape
            .as_ref()
            .map(|value| ("output_shape", value.clone().into())),
        event
            .algorithm
            .as_ref()
            .map(|value| ("algorithm", value.clone().into())),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn build_event_json(event: &TelemetryEvent, ts: u64) -> serde_json::Value {
    let mut fields = serde_json::Map::from_iter([
        ("method".to_string(), event.method.clone().into()),
        ("duration_ms".to_string(), round3(event.duration_ms).into()),
        ("version".to_string(), crate::VERSION.into()),
        ("ts".to_string(), ts.into()),
    ]);

    if let Some(system) = sys_info().as_object() {
        fields.extend(
            system
                .iter()
                .map(|(key, value)| (key.clone(), value.clone())),
        );
    }

    fields.extend(
        event_optional_fields(event)
            .into_iter()
            .map(|(key, value)| (key.to_string(), value)),
    );

    serde_json::Value::Object(fields)
}

pub fn record(event: TelemetryEvent) {
    if !is_consent_given() {
        return;
    }

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    let event_json = build_event_json(&event, ts);

    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(telemetry_path())
    {
        let _ = writeln!(file, "{}", event_json);
    }

    std::thread::spawn(move || {
        try_send_event(event_json);
    });
}

pub fn telemetry_file_path() -> String {
    telemetry_path().to_string_lossy().into_owned()
}

pub fn read_pending() -> Vec<serde_json::Value> {
    std::fs::read_to_string(telemetry_path())
        .ok()
        .map(|text| {
            text.lines()
                .filter_map(|line| serde_json::from_str(line).ok())
                .collect()
        })
        .unwrap_or_default()
}

pub fn clear_pending() {
    let _ = std::fs::write(telemetry_path(), b"");
}

fn endpoint_body(events: &[serde_json::Value], token: &str) -> Result<String, String> {
    let system = get_metrics_summary()
        .get("system")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    serde_json::to_string(&serde_json::json!({
        "secret": token,
        "events": events,
        "system": system,
    }))
    .map_err(|error| error.to_string())
}

pub fn push_pending_to_endpoint(endpoint: &str, token: &str) -> Result<usize, String> {
    let events = read_pending();
    let count = events.len();
    if count == 0 {
        return Ok(0);
    }

    let body = endpoint_body(&events, token)?;

    #[cfg(target_arch = "wasm32")]
    {
        let _ = endpoint;
        let _ = body;
        Err("push_telemetry is unavailable on wasm targets".to_string())
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

    let sent = events
        .iter()
        .filter(|event| try_send_event((*event).clone()))
        .count();

    if sent == events.len() {
        clear_pending();
    }
}

fn percentile(sorted: &[f64], fraction: f64) -> f64 {
    let idx = ((sorted.len() as f64 * fraction) as usize).min(sorted.len().saturating_sub(1));
    sorted.get(idx).copied().unwrap_or(0.0)
}

fn method_summary(durations: &[f64]) -> serde_json::Value {
    let count = durations.len();
    let total = durations.iter().sum::<f64>();
    let min = durations.iter().copied().fold(f64::MAX, f64::min);
    let max = durations.iter().copied().fold(0.0_f64, f64::max);
    let mut sorted = durations.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    serde_json::json!({
        "count": count,
        "total_ms": round3(total),
        "min_ms": round3(min),
        "max_ms": round3(max),
        "avg_ms": round3(total / count as f64),
        "p50_ms": round3(percentile(&sorted, 0.5)),
        "p95_ms": round3(percentile(&sorted, 0.95)),
        "p99_ms": round3(percentile(&sorted, 0.99)),
    })
}

fn methods_summary(events: &[serde_json::Value]) -> HashMap<String, serde_json::Value> {
    let mut durations_by_method: HashMap<String, Vec<f64>> = HashMap::new();

    for event in events {
        let method_and_duration = event.get("method").and_then(|method| method.as_str()).zip(
            event
                .get("duration_ms")
                .and_then(|duration| duration.as_f64()),
        );

        if let Some((method, duration)) = method_and_duration {
            durations_by_method
                .entry(method.to_string())
                .or_default()
                .push(duration);
        }
    }

    durations_by_method
        .into_iter()
        .map(|(method, durations)| (method, method_summary(&durations)))
        .collect()
}

fn system_summary(events: &[serde_json::Value]) -> serde_json::Value {
    const SYSTEM_KEYS: [&str; 7] = [
        "os",
        "arch",
        "cpu_count",
        "ram_gb",
        "available_ram_gb",
        "cpu_brand",
        "python_version",
    ];

    events
        .first()
        .map(|event| {
            serde_json::Value::Object(serde_json::Map::from_iter(SYSTEM_KEYS.iter().map(|key| {
                (
                    (*key).to_string(),
                    event.get(*key).cloned().unwrap_or_default(),
                )
            })))
        })
        .unwrap_or_else(|| serde_json::json!({}))
}

pub fn get_metrics_summary() -> serde_json::Value {
    let events = read_pending();
    if events.is_empty() {
        return serde_json::json!({ "events": [] });
    }

    serde_json::json!({
        "system": system_summary(&events),
        "event_count": events.len(),
        "methods": methods_summary(&events),
        "events": events,
    })
}

#[crate::sera_register(custom)]
pub fn push_telemetry(input: &str) -> String {
    #[derive(serde::Deserialize, Default)]
    struct Input {
        endpoint: Option<String>,
        token: Option<String>,
    }

    let payload: Input = serde_json::from_str(input).unwrap_or_default();
    let result = push_pending_to_endpoint(
        payload.endpoint.as_deref().unwrap_or(""),
        payload.token.as_deref().unwrap_or(""),
    );

    match result {
        Ok(count) => serde_json::json!({"ok": true, "count": count}).to_string(),
        Err(error) => serde_json::json!({"ok": false, "error": error}).to_string(),
    }
}
