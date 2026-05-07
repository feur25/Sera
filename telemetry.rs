use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const GITHUB_DISPATCH_URL: &str = "https://api.github.com/repos/seraplots/seraplot/dispatches";
const GITHUB_TOKEN: &str = "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";

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

    let (ram_gb, cpu_brand) = {
        #[cfg(target_os = "windows")]
        {
            let output = std::process::Command::new("systeminfo").output();
            let mut ram_gb = 0u64;
            let mut cpu_brand = "Unknown".to_string();
            if let Ok(o) = output {
                let stdout = String::from_utf8_lossy(&o.stdout);
                for line in stdout.lines() {
                    if line.contains("Total Physical Memory:") {
                        if let Some(val) = line.split(':').nth(1) {
                            let kb_str = val.split_whitespace().next().unwrap_or("0").replace(",", "");
                            if let Ok(kb) = kb_str.parse::<u64>() {
                                ram_gb = kb / (1024 * 1024);
                            }
                        }
                    }
                    if line.contains("Processor(s):") {
                        if let Some(val) = line.split(':').nth(1) {
                            cpu_brand = val.trim().to_string();
                        }
                    }
                }
            }
            (ram_gb, cpu_brand)
        }
        #[cfg(target_os = "linux")]
        {
            let mut ram_gb = 0u64;
            if let Ok(s) = std::fs::read_to_string("/proc/meminfo") {
                if let Some(line) = s.lines().find(|l| l.starts_with("MemTotal")) {
                    if let Some(kb) = line.split_whitespace().nth(1) {
                        if let Ok(kb_val) = kb.parse::<u64>() {
                            ram_gb = kb_val / (1024 * 1024);
                        }
                    }
                }
            }
            let cpu_brand = std::fs::read_to_string("/proc/cpuinfo")
                .ok()
                .and_then(|s| s.lines().find(|l| l.starts_with("model name:")).map(|l| l.to_string()))
                .unwrap_or_else(|| "Unknown".to_string());
            (ram_gb, cpu_brand)
        }
        #[cfg(target_os = "macos")]
        {
            let mut ram_gb = 0u64;
            if let Ok(o) = std::process::Command::new("sysctl").args(&["-n", "hw.memsize"]).output() {
                if let Ok(s) = String::from_utf8(o.stdout) {
                    if let Ok(bytes) = s.trim().parse::<u64>() {
                        ram_gb = bytes / (1024_u64.pow(3));
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
            (ram_gb, cpu_brand)
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        { (0u64, "Unknown".to_string()) }
    };

    serde_json::json!({
        "os": os,
        "cpu_count": cpu_count,
        "ram_gb": ram_gb,
        "cpu_brand": cpu_brand
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
        client
            .post(GITHUB_DISPATCH_URL)
            .header("Authorization", format!("Bearer {}", GITHUB_TOKEN))
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

pub fn record(method: &str, duration_ms: u64) {
    if !is_consent_given() {
        return;
    }
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let sys_info = get_system_info();
    let mut event = serde_json::json!({
        "method": method,
        "duration_ms": duration_ms,
        "version": crate::VERSION,
        "ts": ts
    });
    if let Some(obj) = event.as_object_mut() {
        if let Some(info_obj) = sys_info.as_object() {
            for (k, v) in info_obj {
                obj.insert(k.clone(), v.clone());
            }
        }
    }
    std::thread::spawn(move || {
        if try_send_event(event.clone()) {
            return;
        }
        let line = format!("{}\n", event);
        let path = seraplot_dir().join("telemetry.jsonl");
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
            let _ = f.write_all(line.as_bytes());
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
                    serde_json::json!({ "count": 0, "total_ms": 0i64, "min_ms": i64::MAX, "max_ms": 0i64, "avg_ms": 0.0 })
                });
                if let Some(dur) = evt.get("duration_ms").and_then(|d| d.as_i64()) {
                    let obj = entry.as_object_mut().unwrap();
                    let count = obj.get("count").and_then(|c| c.as_i64()).unwrap_or(0) + 1;
                    let total = obj.get("total_ms").and_then(|t| t.as_i64()).unwrap_or(0) + dur;
                    let min = obj.get("min_ms").and_then(|m| m.as_i64()).unwrap_or(i64::MAX).min(dur);
                    let max = obj.get("max_ms").and_then(|m| m.as_i64()).unwrap_or(0).max(dur);
                    obj.insert("count".to_string(), serde_json::json!(count));
                    obj.insert("total_ms".to_string(), serde_json::json!(total));
                    obj.insert("min_ms".to_string(), serde_json::json!(min));
                    obj.insert("max_ms".to_string(), serde_json::json!(max));
                    obj.insert("avg_ms".to_string(), serde_json::json!(total as f64 / count as f64));
                }
            }
        }
        let sys_info = if !events.is_empty() {
            let first = &events[0];
            serde_json::json!({
                "os": first.get("os").cloned().unwrap_or(serde_json::json!(null)),
                "cpu_count": first.get("cpu_count").cloned().unwrap_or(serde_json::json!(null)),
                "ram_gb": first.get("ram_gb").cloned().unwrap_or(serde_json::json!(null)),
                "cpu_brand": first.get("cpu_brand").cloned().unwrap_or(serde_json::json!(null))
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
