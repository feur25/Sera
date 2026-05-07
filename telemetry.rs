use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

const GITHUB_DISPATCH_URL: &str = "https://api.github.com/repos/feur25/seraplot/dispatches";
const GITHUB_TOKEN: &str = "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";

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
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));
    home.join(".seraplot")
}

fn get_system_info() -> serde_json::Value {
    let os = std::env::consts::OS;
    let cpu_count = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let arch = std::env::consts::ARCH;
    let python_version = std::env::var("PYTHON_VERSION").unwrap_or_else(|_| "3.11".to_string());

    let (ram_gb, available_ram_gb, cpu_brand) = {
        #[cfg(target_os = "windows")]
        {
            let output = std::process::Command::new("systeminfo").output();
            let mut ram_gb = 0.0f64;
            let mut available_ram_gb = 0.0f64;
            let mut cpu_brand = String::new();
            if let Ok(o) = output {
                let stdout = String::from_utf8_lossy(&o.stdout);
                for line in stdout.lines() {
                    if line.contains("Total Physical Memory:") {
                        if let Some(val) = line.split(':').nth(1) {
                            let parts: Vec<&str> = val.split_whitespace().collect();
                            if let Some(num_str) = parts.first() {
                                let cleaned = num_str.replace(",", "");
                                if let Ok(mb) = cleaned.parse::<f64>() {
                                    ram_gb = mb / 1024.0;
                                }
                            }
                        }
                    }
                    if line.contains("Available Physical Memory:") {
                        if let Some(val) = line.split(':').nth(1) {
                            let parts: Vec<&str> = val.split_whitespace().collect();
                            if let Some(num_str) = parts.first() {
                                let cleaned = num_str.replace(",", "");
                                if let Ok(mb) = cleaned.parse::<f64>() {
                                    available_ram_gb = mb / 1024.0;
                                }
                            }
                        }
                    }
                    if line.contains("Processor(s):") && cpu_brand.is_empty() {
                        if let Some(val) = line.split(':').nth(1) {
                            cpu_brand = val.trim().to_string();
                        }
                    }
                }
            }
            if cpu_brand.is_empty() {
                cpu_brand = "Unknown".to_string();
            }
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
                            if let Ok(kb_val) = kb.parse::<f64>() {
                                ram_gb = kb_val / (1024.0 * 1024.0);
                            }
                        }
                    }
                    if line.starts_with("MemAvailable") {
                        if let Some(kb) = line.split_whitespace().nth(1) {
                            if let Ok(kb_val) = kb.parse::<f64>() {
                                available_ram_gb = kb_val / (1024.0 * 1024.0);
                            }
                        }
                    }
                }
            }
            let cpu_brand = std::fs::read_to_string("/proc/cpuinfo")
                .ok()
                .and_then(|s| s.lines().find(|l| l.starts_with("model name:")).map(|l| l.split(':').nth(1).unwrap_or("Unknown").trim().to_string()))
                .unwrap_or_else(|| "Unknown".to_string());
            (ram_gb, available_ram_gb, cpu_brand)
        }
        #[cfg(target_os = "macos")]
        {
            let mut ram_gb = 0.0f64;
            let mut available_ram_gb = 0.0f64;
            if let Ok(o) = std::process::Command::new("sysctl").args(&["-n", "hw.memsize"]).output() {
                if let Ok(s) = String::from_utf8(o.stdout) {
                    if let Ok(bytes) = s.trim().parse::<f64>() {
                        ram_gb = bytes / (1024.0_f64.powi(3));
                    }
                }
            }
            if let Ok(o) = std::process::Command::new("vm_stat").output() {
                if let Ok(s) = String::from_utf8(o.stdout) {
                    if let Some(line) = s.lines().find(|l| l.contains("Pages free")) {
                        if let Some(num_str) = line.split_whitespace().nth(2) {
                            let cleaned = num_str.trim_end_matches('.');
                            if let Ok(pages) = cleaned.parse::<f64>() {
                                available_ram_gb = (pages * 4096.0) / (1024.0_f64.powi(3));
                            }
                        }
                    }
                }
            }
            let cpu_brand = std::process::Command::new("sysctl")
                .args(&["-n", "machdep.cpu.brand_string"])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "Unknown".to_string());
            (ram_gb, available_ram_gb, cpu_brand)
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        { (0.0, 0.0, "Unknown".to_string()) }
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

fn try_send_event(event: serde_json::Value) -> bool {
    if GITHUB_TOKEN.is_empty() {
        return false;
    }
    let payload = serde_json::json!({
        "event_type": "seraplot-telemetry",
        "client_payload": {
            "events": [event]
        }
    });
    let body = match serde_json::to_string(&payload) {
        Ok(b) => b,
        Err(_) => return false,
    };
    let result = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build();
    if let Ok(client) = result {
        let auth_header = if GITHUB_TOKEN.starts_with("ghp_") {
            format!("token {}", GITHUB_TOKEN)
        } else {
            format!("Bearer {}", GITHUB_TOKEN)
        };
        client
            .post(GITHUB_DISPATCH_URL)
            .header("Authorization", auth_header)
            .header("Accept", "application/vnd.github+json")
            .header("Content-Type", "application/json")
            .header("User-Agent", format!("seraplot/{}", crate::VERSION))
            .body(body)
            .send()
            .map(|r| r.status().as_u16() == 204)
            .unwrap_or(false)
    } else {
        false
    }
}

pub fn is_consent_given() -> bool {
    let path = seraplot_dir().join("consent.json");
    if !path.exists() {
        return false;
    }
    if let Ok(text) = std::fs::read_to_string(&path) {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
            return v.get("enabled").and_then(|e| e.as_bool()).unwrap_or(false);
        }
    }
    false
}

pub fn set_consent(enabled: bool) {
    let dir = seraplot_dir();
    let _ = std::fs::create_dir_all(&dir);
    let json = format!(
        "{{\"enabled\":{enabled},\"version\":\"{}\"}}",
        crate::VERSION
    );
    let _ = std::fs::write(dir.join("consent.json"), json);
}

pub fn record(event: TelemetryEvent) {
    if !is_consent_given() {
        return;
    }
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let sys_info = get_system_info();
    let formatted_duration = format!("{:.3}", event.duration_ms);
    let duration_f64: f64 = formatted_duration.parse().unwrap_or(event.duration_ms);
    let mut event_json = serde_json::json!({
        "method": event.method,
        "duration_ms": duration_f64,
        "version": crate::VERSION,
        "ts": ts
    });
    if let Some(obj) = event_json.as_object_mut() {
        if let Some(info_obj) = sys_info.as_object() {
            for (k, v) in info_obj {
                obj.insert(k.clone(), v.clone());
            }
        }
        if let Some(dc) = event.data_count {
            obj.insert("data_count".to_string(), serde_json::json!(dc));
        }
        if let Some(ds) = event.data_size_mb {
            obj.insert("data_size_mb".to_string(), serde_json::json!((ds * 100.0).round() / 100.0));
        }
        if let Some(is_) = &event.input_shape {
            obj.insert("input_shape".to_string(), serde_json::json!(is_));
        }
        if let Some(os_) = &event.output_shape {
            obj.insert("output_shape".to_string(), serde_json::json!(os_));
        }
        if let Some(alg) = &event.algorithm {
            obj.insert("algorithm".to_string(), serde_json::json!(alg));
        }
    }
    std::thread::spawn(move || {
        if try_send_event(event_json.clone()) {
            return;
        }
        let line = format!("{}\n", event_json);
        let path = seraplot_dir().join("telemetry.jsonl");
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
            let _ = f.write_all(line.as_bytes());
            let _ = f.flush();
        }
    });
}

pub fn telemetry_file_path() -> String {
    seraplot_dir()
        .join("telemetry.jsonl")
        .to_string_lossy()
        .into_owned()
}

pub fn read_pending() -> Vec<serde_json::Value> {
    let path = seraplot_dir().join("telemetry.jsonl");
    if !path.exists() {
        return vec![];
    }
    if let Ok(text) = std::fs::read_to_string(&path) {
        text.lines()
            .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
            .collect()
    } else {
        vec![]
    }
}

pub fn clear_pending() {
    let path = seraplot_dir().join("telemetry.jsonl");
    let _ = std::fs::write(&path, b"");
}

pub fn flush_pending() {
    let events = read_pending();
    if events.is_empty() {
        return;
    }
    let mut sent = 0;
    for event in &events {
        if try_send_event(event.clone()) {
            sent += 1;
        }
    }
    if sent == events.len() {
        clear_pending();
    }
}

pub fn get_metrics_summary() -> serde_json::Value {
    let path = seraplot_dir().join("telemetry.jsonl");
    if !path.exists() {
        return serde_json::json!({ "events": [] });
    }
    if let Ok(text) = std::fs::read_to_string(&path) {
        let events: Vec<serde_json::Value> = text
            .lines()
            .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
            .collect();
        let mut methods = std::collections::HashMap::new();
        for evt in &events {
            if let Some(method) = evt.get("method").and_then(|m| m.as_str()) {
                let entry = methods.entry(method.to_string()).or_insert_with(|| {
                    serde_json::json!({ 
                        "count": 0, 
                        "total_ms": 0.0, 
                        "min_ms": f64::MAX, 
                        "max_ms": 0.0, 
                        "avg_ms": 0.0,
                        "p50_ms": 0.0,
                        "p95_ms": 0.0,
                        "p99_ms": 0.0
                    })
                });
                if let Some(dur) = evt.get("duration_ms").and_then(|d| d.as_f64()) {
                    let obj = entry.as_object_mut().unwrap();
                    let count = obj.get("count").and_then(|c| c.as_i64()).unwrap_or(0) + 1;
                    let total = obj.get("total_ms").and_then(|t| t.as_f64()).unwrap_or(0.0) + dur;
                    let min = obj.get("min_ms").and_then(|m| m.as_f64()).unwrap_or(f64::MAX).min(dur);
                    let max = obj.get("max_ms").and_then(|m| m.as_f64()).unwrap_or(0.0).max(dur);
                    obj.insert("count".to_string(), serde_json::json!(count));
                    obj.insert("total_ms".to_string(), serde_json::json!((total * 1000.0).round() / 1000.0));
                    obj.insert("min_ms".to_string(), serde_json::json!((min * 1000.0).round() / 1000.0));
                    obj.insert("max_ms".to_string(), serde_json::json!((max * 1000.0).round() / 1000.0));
                    obj.insert("avg_ms".to_string(), serde_json::json!(((total / count as f64) * 1000.0).round() / 1000.0));
                }
            }
        }
        let mut per_method_durations: std::collections::HashMap<String, Vec<f64>> = std::collections::HashMap::new();
        for evt in &events {
            if let Some(method) = evt.get("method").and_then(|m| m.as_str()) {
                if let Some(dur) = evt.get("duration_ms").and_then(|d| d.as_f64()) {
                    per_method_durations.entry(method.to_string()).or_insert_with(Vec::new).push(dur);
                }
            }
        }
        for (method, durs) in per_method_durations {
            if !durs.is_empty() && durs.len() > 1 {
                let mut sorted_durs = durs.clone();
                sorted_durs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let p50_idx = (sorted_durs.len() as f64 * 0.5) as usize;
                let p95_idx = (sorted_durs.len() as f64 * 0.95) as usize;
                let p99_idx = (sorted_durs.len() as f64 * 0.99) as usize;
                let p50 = sorted_durs.get(p50_idx).copied().unwrap_or(0.0);
                let p95 = sorted_durs.get(p95_idx.min(sorted_durs.len() - 1)).copied().unwrap_or(0.0);
                let p99 = sorted_durs.get(p99_idx.min(sorted_durs.len() - 1)).copied().unwrap_or(0.0);
                if let Some(m) = methods.get_mut(&method) {
                    if let Some(obj) = m.as_object_mut() {
                        obj.insert("p50_ms".to_string(), serde_json::json!((p50 * 1000.0).round() / 1000.0));
                        obj.insert("p95_ms".to_string(), serde_json::json!((p95 * 1000.0).round() / 1000.0));
                        obj.insert("p99_ms".to_string(), serde_json::json!((p99 * 1000.0).round() / 1000.0));
                    }
                }
            }
        }
        let sys_info = if !events.is_empty() {
            let first = &events[0];
            serde_json::json!({
                "os": first.get("os").cloned().unwrap_or(serde_json::json!(null)),
                "arch": first.get("arch").cloned().unwrap_or(serde_json::json!(null)),
                "cpu_count": first.get("cpu_count").cloned().unwrap_or(serde_json::json!(null)),
                "ram_gb": first.get("ram_gb").cloned().unwrap_or(serde_json::json!(null)),
                "available_ram_gb": first.get("available_ram_gb").cloned().unwrap_or(serde_json::json!(null)),
                "cpu_brand": first.get("cpu_brand").cloned().unwrap_or(serde_json::json!(null)),
                "python_version": first.get("python_version").cloned().unwrap_or(serde_json::json!(null))
            })
        } else {
            serde_json::json!({})
        };
        serde_json::json!({
            "system": sys_info,
            "event_count": events.len(),
            "methods": methods,
            "events": events
        })
    } else {
        serde_json::json!({ "events": [] })
    }
}
