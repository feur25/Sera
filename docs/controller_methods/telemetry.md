# Telemetry

SeraPlot collects anonymized performance metrics to help identify optimization opportunities and understand usage patterns. Telemetry is **disabled by default** and requires explicit activation.

## What Data is Collected

When enabled, the following metrics are automatically recorded for each method call:

| Metric | Type | Description | Examples |
|--------|------|-------------|----------|
| `ts` | timestamp | Unix timestamp when the event occurred | 1700000000 |
| `method` | string | Function or method name | "KMeans.fit", "plot", "scatter" |
| `duration_ms` | float | Execution time with decimal precision | 542.365, 0.113 |
| `version` | string | SeraPlot version | "2.7.0" |
| `os` | string | Operating system | "windows", "linux", "macos" |
| `arch` | string | CPU architecture | "x86_64", "aarch64" |
| `cpu_count` | integer | Number of available CPU cores | 8, 16, 32 |
| `ram_gb` | float | Total system RAM in GB | 16.0, 32.0 |
| `available_ram_gb` | float | Available RAM at measurement time | 8.5, 12.3 |
| `cpu_brand` | string | CPU model name | "Intel Core i7-9700", "AMD Ryzen 5" |
| `python_version` | string | Python interpreter version | "3.11.0", "3.10.5" |
| `data_count` | integer (optional) | Number of records/data points processed | 1000000, 50000 |
| `input_shape` | string (optional) | Input data dimensions | "1000x256", "500" |
| `output_shape` | string (optional) | Output data dimensions | "1000x64", "500" |
| `algorithm` | string (optional) | Specific algorithm name | "KMeans", "DBSCAN" |

## Privacy & Security

SeraPlot telemetry **does not collect**:
- User identity, credentials, or account information
- File names, file paths, or file contents
- Model parameters or trained weights
- Individual data values or samples
- Any personally identifiable information (PII)

Only aggregated, anonymized system and performance metrics are transmitted.

## How It Works

Telemetry events are collected locally in `~/.seraplot/telemetry.jsonl` and transmitted in background threads to avoid blocking user code. Events are deleted after successful transmission to GitHub.

The telemetry system:
1. Captures method execution time and system information
2. Serializes event as JSON
3. Spawns background thread for non-blocking transmission
4. Sends to GitHub Actions dispatcher endpoint
5. Falls back to local JSONL storage if transmission fails
6. Deletes local event after successful send

## Enabling Telemetry

By default, telemetry is **completely disabled**. To enable it, call:

```python
import seraplot as sp

sp.telemetry_consent(enabled=True)
```

This creates `~/.seraplot/consent.json` with your preference.

To disable again:

```python
sp.telemetry_consent(enabled=False)
```

## Accessing Raw Telemetry

To retrieve aggregated telemetry metrics programmatically:

```python
metrics = sp.get_metrics_summary()
print(metrics)
```

Returns aggregated statistics including percentiles (P50, P95, P99) for execution times.

## Raw Telemetry Module Source

The complete telemetry implementation is open-source. View the source code to verify data collection behavior:

```rust
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

const GITHUB_DISPATCH_URL: &str = "https://api.github.com/repos/feur25/seraplot/dispatches";

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

pub fn record(event: TelemetryEvent) {
    if !is_consent_given() {
        return;
    }

    let json_str = serde_json::to_string(&event).unwrap_or_default();
    let dir = seraplot_dir();
    let _ = std::fs::create_dir_all(&dir);

    let backup_path = dir.join("telemetry.jsonl");
    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&backup_path)
        .and_then(|mut file| {
            use std::io::Write;
            writeln!(file, "{}", json_str)
        });

    let system_info = get_system_info();
    std::thread::spawn(move || {
        let _ = try_send_event(&event, &system_info);
    });
}

pub fn is_consent_given() -> bool {
    let dir = seraplot_dir();
    let consent_file = dir.join("consent.json");
    if let Ok(content) = std::fs::read_to_string(&consent_file) {
        if let Ok(obj) = serde_json::from_str::<serde_json::Value>(&content) {
            return obj.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false);
        }
    }
    false
}

pub fn telemetry_consent(enabled: bool) {
    let dir = seraplot_dir();
    let _ = std::fs::create_dir_all(&dir);
    let consent_file = dir.join("consent.json");
    let content = serde_json::json!({"enabled": enabled}).to_string();
    let _ = std::fs::write(consent_file, content);
}

pub fn get_metrics_summary() -> serde_json::Value {
    let dir = seraplot_dir();
    let backup_path = dir.join("telemetry.jsonl");
    let mut durations = Vec::new();

    if let Ok(content) = std::fs::read_to_string(&backup_path) {
        for line in content.lines() {
            if let Ok(obj) = serde_json::from_str::<serde_json::Value>(line) {
                if let Some(d) = obj.get("duration_ms").and_then(|v| v.as_f64()) {
                    durations.push(d);
                }
            }
        }
    }

    durations.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    serde_json::json!({
        "count": durations.len(),
        "p50": get_percentile(&durations, 50.0),
        "p95": get_percentile(&durations, 95.0),
        "p99": get_percentile(&durations, 99.0),
    })
}

fn get_percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = ((p / 100.0) * sorted.len() as f64).ceil() as usize;
    sorted[idx.saturating_sub(1)]
}
```

The token constant has been removed from this documentation for security. The actual transmission uses GitHub's classic authentication header format.

## Questions?

For questions about telemetry or privacy, refer to [TELEMETRY.md](/TELEMETRY.md) in the main repository.
