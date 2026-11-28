use pyo3::prelude::*;
use seraplot_macros::sera_doc;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

static COUNTERS: Mutex<Option<HashMap<(&'static str, u64), std::sync::Arc<AtomicU64>>>> = Mutex::new(None);

pub fn incr(metric: &'static str, hid: u64) {
    incr_by(metric, hid, 1);
}

pub fn incr_by(metric: &'static str, hid: u64, delta: u64) {
    let Ok(mut guard) = COUNTERS.lock() else { return };
    let map = guard.get_or_insert_with(HashMap::new);
    let counter = map.entry((metric, hid)).or_insert_with(|| std::sync::Arc::new(AtomicU64::new(0)));
    counter.fetch_add(delta, Ordering::Relaxed);
}

pub fn render_prometheus() -> String {
    let Ok(guard) = COUNTERS.lock() else { return String::new() };
    let Some(map) = guard.as_ref() else { return String::new() };
    let mut by_metric: HashMap<&str, Vec<(u64, u64)>> = HashMap::new();
    for ((metric, hid), counter) in map.iter() {
        by_metric.entry(*metric).or_default().push((*hid, counter.load(Ordering::Relaxed)));
    }
    let mut metric_names: Vec<&&str> = by_metric.keys().collect();
    metric_names.sort();
    let mut out = String::new();
    for name in metric_names {
        out.push_str(&format!("# TYPE {name} counter\n"));
        let mut entries = by_metric[name].clone();
        entries.sort_by_key(|(hid, _)| *hid);
        for (hid, value) in entries {
            out.push_str(&format!("{name}{{chart=\"{hid}\"}} {value}\n"));
        }
    }
    out
}

fn handle_connection(mut stream: std::net::TcpStream) {
    let mut buf = [0u8; 512];
    let _ = stream.read(&mut buf);
    super::health::touch("metrics", None);
    let body = render_prometheus();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain; version=0.0.4\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    let _ = stream.write_all(response.as_bytes());
}

pub fn serve_metrics_blocking(host: &str, port: u16) -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind((host, port))?;
    super::health::register("metrics", None, host, listener.local_addr().map(|a| a.port()).unwrap_or(port));
    for stream in listener.incoming().flatten() {
        handle_connection(stream);
    }
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (host = "127.0.0.1", port = 9187))]
#[sera_doc(
    category = "pulse_function",
    module = "health",
    file = "seraplot/pulse-pricing.md",
    en = "Starts a tiny standalone HTTP server exposing Pulse-wide counters (pushes, rate-limit rejections, anomalies, alerts sent) in Prometheus text exposition format at '/', for scraping. Separate from .serve()'s own server.",
    fr = "Démarre un petit serveur HTTP autonome exposant les compteurs Pulse (push, rejets de limitation de débit, anomalies, alertes envoyées) au format d'exposition texte Prometheus sur '/', prêt à être scrapé. Séparé du serveur de .serve().",
    param(name = "host", ty = "str", en = "Address to bind the metrics server on.", fr = "Adresse sur laquelle lier le serveur de métriques."),
    param(name = "port", ty = "int", en = "Port to bind the metrics server on.", fr = "Port sur lequel lier le serveur de métriques.")
)]
pub fn pulse_serve_metrics(host: &str, port: u16) -> PyResult<()> {
    let host = host.to_string();
    std::thread::spawn(move || {
        let _ = serve_metrics_blocking(&host, port);
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpStream;

    #[test]
    fn counters_start_at_zero_and_accumulate() {
        let hid = 3_000_001u64;
        incr("test_metric_a", hid);
        incr("test_metric_a", hid);
        incr_by("test_metric_a", hid, 3);
        let text = render_prometheus();
        assert!(text.contains(&format!("test_metric_a{{chart=\"{hid}\"}} 5")));
    }

    #[test]
    fn separate_hids_tracked_independently() {
        let a = 3_000_002u64;
        let b = 3_000_003u64;
        incr("test_metric_b", a);
        incr("test_metric_b", b);
        incr("test_metric_b", b);
        let text = render_prometheus();
        assert!(text.contains(&format!("test_metric_b{{chart=\"{a}\"}} 1")));
        assert!(text.contains(&format!("test_metric_b{{chart=\"{b}\"}} 2")));
    }

    #[test]
    fn output_has_type_header_per_metric() {
        let hid = 3_000_004u64;
        incr("test_metric_c", hid);
        let text = render_prometheus();
        assert!(text.contains("# TYPE test_metric_c counter"));
    }

    #[test]
    fn overhead_of_incr_is_negligible() {
        let hid = 3_000_005u64;
        let n = 1_000_000u32;
        let started = std::time::Instant::now();
        for _ in 0..n {
            incr("test_metric_perf", hid);
        }
        let elapsed = started.elapsed();
        println!("metrics::incr x{n}: {elapsed:?} ({:?}/call)", elapsed / n);
        assert!(elapsed.as_millis() < 1000, "incr() got too slow: {elapsed:?} for {n} calls");
    }

    #[test]
    fn real_http_server_serves_prometheus_text() {
        let hid = 3_000_006u64;
        incr("test_metric_http", hid);

        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind test server");
        let addr = listener.local_addr().unwrap();
        let handle = std::thread::spawn(move || {
            if let Ok((stream, _)) = listener.accept() {
                handle_connection(stream);
            }
        });

        let mut client = TcpStream::connect(addr).expect("connect to test server");
        client.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();
        let mut response = String::new();
        client.read_to_string(&mut response).unwrap();
        handle.join().unwrap();

        assert!(response.starts_with("HTTP/1.1 200 OK"));
        assert!(response.contains("Content-Type: text/plain"));
        assert!(response.contains(&format!("test_metric_http{{chart=\"{hid}\"}} 1")));
    }
}
