use crate::Chart;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use seraplot_macros::{sera_doc, sera_doc_impl};
use std::collections::VecDeque;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex};

const CAPACITY: usize = 256;

struct ServiceEntry {
    kind: &'static str,
    hid: Option<u64>,
    host: String,
    port: u16,
    started_at_ms: i64,
    last_activity_at_ms: AtomicI64,
}

struct Registry {
    entries: VecDeque<Arc<ServiceEntry>>,
}

static REGISTRY: Mutex<Option<Registry>> = Mutex::new(None);

fn push_bounded(entries: &mut VecDeque<Arc<ServiceEntry>>, entry: Arc<ServiceEntry>) {
    entries.push_back(entry);
    while entries.len() > CAPACITY {
        entries.pop_front();
    }
}

pub fn register(kind: &'static str, hid: Option<u64>, host: &str, port: u16) {
    let Ok(mut guard) = REGISTRY.lock() else { return };
    let reg = guard.get_or_insert_with(|| Registry { entries: VecDeque::new() });
    let now = crate::plot::now_ms();
    push_bounded(
        &mut reg.entries,
        Arc::new(ServiceEntry {
            kind,
            hid,
            host: host.to_string(),
            port,
            started_at_ms: now,
            last_activity_at_ms: AtomicI64::new(now),
        }),
    );
}

pub fn touch(kind: &str, hid: Option<u64>) {
    let Ok(guard) = REGISTRY.lock() else { return };
    let Some(reg) = guard.as_ref() else { return };
    let now = crate::plot::now_ms();
    for entry in reg.entries.iter().rev() {
        if entry.kind == kind && entry.hid == hid {
            entry.last_activity_at_ms.store(now, Ordering::Relaxed);
            break;
        }
    }
}

struct Snapshot {
    kind: &'static str,
    hid: Option<u64>,
    host: String,
    port: u16,
    started_at_ms: i64,
    last_activity_at_ms: i64,
}

fn snapshot() -> Vec<Snapshot> {
    let Ok(guard) = REGISTRY.lock() else { return Vec::new() };
    let Some(reg) = guard.as_ref() else { return Vec::new() };
    reg.entries
        .iter()
        .map(|e| Snapshot {
            kind: e.kind,
            hid: e.hid,
            host: e.host.clone(),
            port: e.port,
            started_at_ms: e.started_at_ms,
            last_activity_at_ms: e.last_activity_at_ms.load(Ordering::Relaxed),
        })
        .collect()
}

pub(crate) fn is_active(kind: &str, hid: u64) -> bool {
    let Ok(guard) = REGISTRY.lock() else { return false };
    let Some(reg) = guard.as_ref() else { return false };
    reg.entries.iter().any(|e| e.kind == kind && e.hid == Some(hid))
}

fn render_json() -> String {
    let now = crate::plot::now_ms();
    let items: Vec<serde_json::Value> = snapshot()
        .iter()
        .map(|e| {
            serde_json::json!({
                "kind": e.kind,
                "hid": e.hid,
                "host": e.host,
                "port": e.port,
                "started_at_ms": e.started_at_ms,
                "uptime_ms": (now - e.started_at_ms).max(0),
                "last_activity_at_ms": e.last_activity_at_ms,
                "idle_ms": (now - e.last_activity_at_ms).max(0),
            })
        })
        .collect();
    serde_json::json!({ "now_ms": now, "services": items }).to_string()
}

fn handle_connection(mut stream: std::net::TcpStream) {
    let mut buf = [0u8; 512];
    let _ = stream.read(&mut buf);
    let body = render_json();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    let _ = stream.write_all(response.as_bytes());
}

pub fn serve_health_blocking(host: &str, port: u16) -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind((host, port))?;
    register("health", None, host, listener.local_addr().map(|a| a.port()).unwrap_or(port));
    for stream in listener.incoming().flatten() {
        touch("health", None);
        handle_connection(stream);
    }
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (host = "127.0.0.1", port = 9188))]
#[sera_doc(
    category = "pulse_function",
    module = "health",
    file = "seraplot/pulse-pricing.md",
    en = "Starts a tiny standalone HTTP server exposing '/' as a JSON health check: every serve()/enable_remote_push()/pulse_serve_metrics() instance currently running in this process, each with its host, port, chart id (if any), uptime, and time since last activity - so the monitoring tool itself can be monitored. Separate from .serve()'s own server.",
    fr = "Démarre un petit serveur HTTP autonome exposant '/' en JSON de santé : chaque instance serve()/enable_remote_push()/pulse_serve_metrics() actuellement active dans ce process, avec son hôte, son port, l'id du chart (s'il y en a un), sa durée de vie et le temps depuis sa dernière activité — pour que l'outil de monitoring soit lui-même monitorable. Séparé du serveur de .serve().",
    param(name = "host", ty = "str", en = "Address to bind the health server on.", fr = "Adresse sur laquelle lier le serveur de santé."),
    param(name = "port", ty = "int", en = "Port to bind the health server on.", fr = "Port sur lequel lier le serveur de santé.")
)]
pub fn pulse_serve_health(host: &str, port: u16) -> PyResult<()> {
    let host = host.to_string();
    std::thread::spawn(move || {
        let _ = serve_health_blocking(&host, port);
    });
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction]
#[sera_doc(
    category = "pulse_function",
    module = "health",
    file = "seraplot/pulse-pricing.md",
    en = "Returns the same data as pulse_serve_health()'s endpoint, synchronously and without starting a server - every serve()/enable_remote_push()/pulse_serve_metrics() instance currently running in this process, with uptime and time since last activity.",
    fr = "Retourne les mêmes données que le point d'entrée de pulse_serve_health(), en synchrone et sans démarrer de serveur — chaque instance serve()/enable_remote_push()/pulse_serve_metrics() actuellement active dans ce process, avec sa durée de vie et le temps depuis sa dernière activité."
)]
pub fn pulse_health(py: Python<'_>) -> PyResult<Py<PyDict>> {
    let now = crate::plot::now_ms();
    let dict = PyDict::new_bound(py);
    dict.set_item("now_ms", now)?;
    let list = PyList::empty_bound(py);
    for e in snapshot() {
        let item = PyDict::new_bound(py);
        item.set_item("kind", e.kind)?;
        item.set_item("hid", e.hid)?;
        item.set_item("host", e.host)?;
        item.set_item("port", e.port)?;
        item.set_item("started_at_ms", e.started_at_ms)?;
        item.set_item("uptime_ms", (now - e.started_at_ms).max(0))?;
        item.set_item("last_activity_at_ms", e.last_activity_at_ms)?;
        item.set_item("idle_ms", (now - e.last_activity_at_ms).max(0))?;
        list.append(item)?;
    }
    dict.set_item("services", list)?;
    Ok(dict.unbind())
}

#[cfg(feature = "python")]
#[sera_doc_impl]
#[pyo3::pymethods]
impl Chart {
    #[sera_doc(
        category = "chart_method",
        module = "streaming",
        file = "seraplot/pulse-pricing.md",
        en = "Reports whether serve() is currently running for this chart in this process: True right after a successful serve() call, False before it. There is currently no stop_serve(), so once True this stays True for the rest of the process's life.",
        fr = "Indique si serve() est actuellement actif pour ce chart dans ce processus : True juste après un appel serve() réussi, False avant. Il n'existe pas encore de stop_serve(), donc une fois à True cela le reste pour le reste de la vie du processus."
    )]
    fn is_serving(&self) -> PyResult<bool> {
        let hid = super::require_hid(&self.html)?;
        Ok(is_active("serve", hid))
    }

    #[sera_doc(
        category = "chart_method",
        module = "remote",
        file = "seraplot/pulse-pricing.md",
        en = "Reports whether enable_remote_push() is currently running for this chart in this process - the same one-way flag as is_serving(), since there is no disable_remote_push() either yet.",
        fr = "Indique si enable_remote_push() est actuellement actif pour ce chart dans ce processus — le même indicateur à sens unique que is_serving(), puisqu'il n'existe pas non plus de disable_remote_push() pour l'instant."
    )]
    fn is_remote_push_enabled(&self) -> PyResult<bool> {
        let hid = super::require_hid(&self.html)?;
        Ok(is_active("remote_push", hid))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_snapshot_round_trip() {
        register("test_kind_a", Some(1), "127.0.0.1", 9999);
        let rows = snapshot();
        let found = rows.iter().find(|r| r.kind == "test_kind_a" && r.hid == Some(1));
        assert!(found.is_some(), "a registered service should appear in the snapshot");
        let entry = found.unwrap();
        assert_eq!(entry.host, "127.0.0.1");
        assert_eq!(entry.port, 9999);
        assert!(entry.started_at_ms > 0);
        assert_eq!(entry.started_at_ms, entry.last_activity_at_ms, "last_activity should start equal to started_at before any touch()");
    }

    #[test]
    fn touch_updates_only_the_matching_service() {
        register("test_kind_b", Some(2), "127.0.0.1", 9998);
        register("test_kind_b", Some(3), "127.0.0.1", 9997);
        std::thread::sleep(std::time::Duration::from_millis(5));
        touch("test_kind_b", Some(2));

        let rows = snapshot();
        let entry2 = rows.iter().rev().find(|r| r.kind == "test_kind_b" && r.hid == Some(2)).unwrap();
        let entry3 = rows.iter().rev().find(|r| r.kind == "test_kind_b" && r.hid == Some(3)).unwrap();
        assert!(entry2.last_activity_at_ms > entry2.started_at_ms, "touched service should have a later last_activity");
        assert_eq!(entry3.last_activity_at_ms, entry3.started_at_ms, "untouched service should be unaffected");
    }

    #[test]
    fn touch_on_a_repeated_kind_updates_the_most_recent_matching_entry() {
        register("test_kind_c", Some(4), "127.0.0.1", 9000);
        std::thread::sleep(std::time::Duration::from_millis(5));
        register("test_kind_c", Some(4), "127.0.0.1", 9001);
        std::thread::sleep(std::time::Duration::from_millis(5));
        touch("test_kind_c", Some(4));

        let rows = snapshot();
        let matching: Vec<_> = rows.iter().filter(|r| r.kind == "test_kind_c" && r.hid == Some(4)).collect();
        assert_eq!(matching.len(), 2);
        let newest = matching.iter().max_by_key(|r| r.started_at_ms).unwrap();
        let oldest = matching.iter().min_by_key(|r| r.started_at_ms).unwrap();
        assert!(newest.last_activity_at_ms > newest.started_at_ms, "the most recently registered matching entry should receive the touch");
        assert_eq!(oldest.last_activity_at_ms, oldest.started_at_ms, "the older entry with the same kind/hid should not be touched");
    }

    #[test]
    fn eviction_logic_is_bounded_and_keeps_the_newest_entries() {
        let mut entries: VecDeque<Arc<ServiceEntry>> = VecDeque::new();
        for i in 0..(CAPACITY + 100) {
            let entry = Arc::new(ServiceEntry {
                kind: "test_kind_bound",
                hid: Some(i as u64),
                host: "127.0.0.1".to_string(),
                port: 8000,
                started_at_ms: 0,
                last_activity_at_ms: AtomicI64::new(0),
            });
            push_bounded(&mut entries, entry);
        }
        assert_eq!(entries.len(), CAPACITY, "tested directly against a local queue, not the shared global REGISTRY other tests in this file also use, so a flood this size can't evict their entries");
        assert_eq!(entries.back().unwrap().hid, Some((CAPACITY + 99) as u64), "the newest entry should always survive eviction");
        assert_eq!(entries.front().unwrap().hid, Some(100), "the oldest 100 entries should have been evicted first (FIFO)");
    }

    #[test]
    fn json_rendering_includes_registered_services() {
        register("test_kind_json", Some(77), "0.0.0.0", 8123);
        let body = render_json();
        assert!(body.contains("test_kind_json"));
        assert!(body.contains("8123"));
        assert!(body.contains("\"now_ms\""));
    }

    #[test]
    fn real_http_server_serves_json_health() {
        let hid = 88u64;
        register("test_kind_http", Some(hid), "127.0.0.1", 7654);

        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind test server");
        let addr = listener.local_addr().unwrap();
        let handle = std::thread::spawn(move || {
            if let Ok((stream, _)) = listener.accept() {
                handle_connection(stream);
            }
        });

        let mut client = std::net::TcpStream::connect(addr).expect("connect to test server");
        client.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();
        let mut response = String::new();
        client.read_to_string(&mut response).unwrap();
        handle.join().unwrap();

        assert!(response.starts_with("HTTP/1.1 200 OK"));
        assert!(response.contains("Content-Type: application/json"));
        assert!(response.contains("test_kind_http"));
    }

    #[test]
    fn is_active_matches_only_the_exact_kind_and_hid() {
        register("test_kind_active", Some(501), "127.0.0.1", 6001);
        assert!(is_active("test_kind_active", 501));
        assert!(!is_active("test_kind_active", 502), "a different hid with the same kind should not match");
        assert!(!is_active("other_kind", 501), "a different kind with the same hid should not match");
        assert!(!is_active("test_kind_active_unregistered", 999), "an hid that was never registered should not match");
    }

    #[test]
    fn overhead_of_is_active_is_negligible() {
        register("test_kind_is_active_perf", Some(4242), "127.0.0.1", 4321);
        let n = 100_000u32;
        let started = std::time::Instant::now();
        for _ in 0..n {
            is_active("test_kind_is_active_perf", 4242);
        }
        let elapsed = started.elapsed();
        println!("health::is_active x{n}: {elapsed:?} ({:?}/call)", elapsed / n);
        assert!(elapsed.as_millis() < 500, "is_active() got too slow: {elapsed:?} for {n} calls");
    }

    #[test]
    fn overhead_of_touch_is_negligible() {
        register("test_kind_perf", Some(999), "127.0.0.1", 1234);
        let n = 100_000u32;
        let started = std::time::Instant::now();
        for _ in 0..n {
            touch("test_kind_perf", Some(999));
        }
        let elapsed = started.elapsed();
        println!("health::touch x{n}: {elapsed:?} ({:?}/call)", elapsed / n);
        assert!(elapsed.as_millis() < 500, "touch() got too slow: {elapsed:?} for {n} calls");
    }
}
