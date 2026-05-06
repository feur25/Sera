use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn seraplot_dir() -> PathBuf {
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));
    home.join(".seraplot")
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
    let cpu_count = {
        #[cfg(not(target_os = "unknown"))]
        {
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1)
        }
        #[cfg(target_os = "unknown")]
        { 1usize }
    };
    let line = format!(
        "{{\"method\":\"{method}\",\"duration_ms\":{duration_ms},\"version\":\"{ver}\",\"cpu_count\":{cpu_count},\"ts\":{ts}}}\n",
        ver = crate::VERSION
    );
    let path = seraplot_dir().join("telemetry.jsonl");
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
        let _ = f.write_all(line.as_bytes());
    }
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
