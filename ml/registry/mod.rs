use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ModelRecord {
    pub name: String,
    pub version: u32,
    pub kind: String,
    pub created_ms: u64,
    pub metrics: HashMap<String, f64>,
    pub params: HashMap<String, String>,
    pub payload: String,
    pub tags: Vec<String>,
}

#[derive(Default, Serialize, Deserialize)]
struct RegistryIndex {
    entries: HashMap<String, Vec<ModelRecord>>,
}

static MEM: OnceLock<Mutex<RegistryIndex>> = OnceLock::new();

fn store() -> &'static Mutex<RegistryIndex> {
    MEM.get_or_init(|| Mutex::new(load_disk()))
}

pub fn registry_dir() -> PathBuf {
    let base = std::env::var("SERAPLOT_REGISTRY_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs_home()
                .unwrap_or_else(std::env::temp_dir)
                .join(".seraplot")
                .join("registry")
        });
    let _ = std::fs::create_dir_all(&base);
    base
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
}

fn index_path() -> PathBuf { registry_dir().join("index.json") }

fn load_disk() -> RegistryIndex {
    let p = index_path();
    if let Ok(s) = std::fs::read_to_string(&p) {
        serde_json::from_str(&s).unwrap_or_default()
    } else {
        RegistryIndex::default()
    }
}

fn flush_disk(idx: &RegistryIndex) {
    if let Ok(s) = serde_json::to_string(idx) {
        let _ = std::fs::write(index_path(), s);
    }
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

pub fn register(name: &str, kind: &str, payload: &str, params: HashMap<String, String>, metrics: HashMap<String, f64>, tags: Vec<String>) -> ModelRecord {
    let mut g = store().lock().unwrap();
    let v = g.entries.get(name).map(|l| l.iter().map(|r| r.version).max().unwrap_or(0) + 1).unwrap_or(1);
    let rec = ModelRecord {
        name: name.to_string(),
        version: v,
        kind: kind.to_string(),
        created_ms: now_ms(),
        metrics,
        params,
        payload: payload.to_string(),
        tags,
    };
    g.entries.entry(name.to_string()).or_default().push(rec.clone());
    flush_disk(&g);
    rec
}

pub fn get(name: &str, version: Option<u32>) -> Option<ModelRecord> {
    let g = store().lock().unwrap();
    let list = g.entries.get(name)?;
    match version {
        Some(v) => list.iter().find(|r| r.version == v).cloned(),
        None => list.iter().max_by_key(|r| r.version).cloned(),
    }
}

pub fn list() -> Vec<ModelRecord> {
    let g = store().lock().unwrap();
    let mut all: Vec<ModelRecord> = g.entries.values().flat_map(|v| v.iter().cloned()).collect();
    all.sort_by(|a, b| b.created_ms.cmp(&a.created_ms));
    all
}

pub fn delete(name: &str, version: Option<u32>) -> usize {
    let mut g = store().lock().unwrap();
    let removed = if let Some(v) = version {
        if let Some(list) = g.entries.get_mut(name) {
            let before = list.len();
            list.retain(|r| r.version != v);
            before - list.len()
        } else { 0 }
    } else {
        g.entries.remove(name).map(|l| l.len()).unwrap_or(0)
    };
    flush_disk(&g);
    removed
}

pub fn clear_all() -> usize {
    let mut g = store().lock().unwrap();
    let n: usize = g.entries.values().map(|v| v.len()).sum();
    g.entries.clear();
    flush_disk(&g);
    n
}

pub fn promote(name: &str, version: u32, tag: &str) -> bool {
    let mut g = store().lock().unwrap();
    if let Some(list) = g.entries.get_mut(name) {
        for r in list.iter_mut() {
            if r.version == version {
                if !r.tags.iter().any(|t| t == tag) { r.tags.push(tag.to_string()); }
                flush_disk(&g);
                return true;
            }
        }
    }
    false
}

pub fn by_tag(name: &str, tag: &str) -> Option<ModelRecord> {
    let g = store().lock().unwrap();
    g.entries.get(name)?.iter().filter(|r| r.tags.iter().any(|t| t == tag)).max_by_key(|r| r.version).cloned()
}


