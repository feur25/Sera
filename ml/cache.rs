use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};

pub struct CheckpointEntry {
    pub weights: Vec<f64>,
    pub iteration: usize,
}

static STORE: OnceLock<Mutex<HashMap<u64, CheckpointEntry>>> = OnceLock::new();

fn mem_store() -> &'static Mutex<HashMap<u64, CheckpointEntry>> {
    STORE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn tasks_dir() -> PathBuf {
    let base = std::env::var("SERAPLOT_TASKS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir().join("seraplot_tasks"));
    let _ = std::fs::create_dir_all(&base);
    base
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn gen_id() -> u64 {
    static CTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let c = CTR.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    now_ms().wrapping_mul(10000).wrapping_add(c % 10000)
}

static SESSION: OnceLock<u64> = OnceLock::new();

pub fn session_id() -> u64 {
    *SESSION.get_or_init(gen_id)
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct PartialState {
    pub weights: Vec<f64>,
    pub iteration: usize,
    pub combo_fold_scores: HashMap<String, Vec<f64>>,
    pub fold_scores: Vec<f64>,
    pub completed_folds: usize,
    pub halving_candidates: Vec<usize>,
    pub halving_resource: usize,
    pub halving_iteration: usize,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "status")]
pub enum TaskStatus {
    Running { progress: f32 },
    Completed,
    Failed { error: String },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TaskEntry {
    pub task_id: u64,
    pub task_type: String,
    pub status: TaskStatus,
    pub partial: PartialState,
    pub created_ms: u64,
    pub updated_ms: u64,
    #[serde(default)]
    pub fingerprint: u64,
    #[serde(default)]
    pub session: u64,
}

fn task_path(id: u64) -> PathBuf {
    tasks_dir().join(format!("{}.json", id))
}

fn result_path(id: u64) -> PathBuf {
    tasks_dir().join(format!("{}.result", id))
}

fn task_read(id: u64) -> Option<TaskEntry> {
    let bytes = std::fs::read(task_path(id)).ok()?;
    serde_json::from_slice(&bytes).ok()
}

fn task_write(entry: &TaskEntry) {
    if let Ok(json) = serde_json::to_vec(entry) {
        let _ = std::fs::write(task_path(entry.task_id), json);
    }
}

pub struct Fp(std::collections::hash_map::DefaultHasher);

impl Fp {
    pub fn new(name: &str) -> Self {
        let mut h = std::collections::hash_map::DefaultHasher::new();
        name.hash(&mut h);
        Self(h)
    }
    pub fn f64(mut self, v: f64) -> Self { v.to_bits().hash(&mut self.0); self }
    pub fn u64(mut self, v: u64) -> Self { v.hash(&mut self.0); self }
    pub fn usize(mut self, v: usize) -> Self { v.hash(&mut self.0); self }
    pub fn i64(mut self, v: i64) -> Self { v.hash(&mut self.0); self }
    pub fn str(mut self, v: &str) -> Self { v.hash(&mut self.0); self }
    pub fn bval(mut self, v: bool) -> Self { v.hash(&mut self.0); self }
    pub fn data(mut self, d: &[f64], n: usize, p: usize) -> Self {
        n.hash(&mut self.0);
        p.hash(&mut self.0);
        let len = d.len();
        for i in 0..4.min(len) { d[i].to_bits().hash(&mut self.0); }
        for i in len.saturating_sub(4)..len { d[i].to_bits().hash(&mut self.0); }
        self
    }
    pub fn labels(mut self, d: &[i32]) -> Self {
        d.len().hash(&mut self.0);
        for i in 0..4.min(d.len()) { d[i].hash(&mut self.0); }
        for i in d.len().saturating_sub(4)..d.len() { d[i].hash(&mut self.0); }
        self
    }
    pub fn targets(mut self, d: &[f64]) -> Self {
        d.len().hash(&mut self.0);
        for i in 0..4.min(d.len()) { d[i].to_bits().hash(&mut self.0); }
        for i in d.len().saturating_sub(4)..d.len() { d[i].to_bits().hash(&mut self.0); }
        self
    }
    pub fn strings(mut self, d: &[String]) -> Self {
        d.len().hash(&mut self.0);
        for i in 0..2.min(d.len()) { d[i].hash(&mut self.0); }
        for i in d.len().saturating_sub(2)..d.len() { d[i].hash(&mut self.0); }
        self
    }
    pub fn floats(mut self, d: &[f64]) -> Self {
        d.len().hash(&mut self.0);
        for i in 0..2.min(d.len()) { d[i].to_bits().hash(&mut self.0); }
        for i in d.len().saturating_sub(2)..d.len() { d[i].to_bits().hash(&mut self.0); }
        self
    }
    pub fn finish(self) -> u64 { self.0.finish() }
}

static FP_INDEX: OnceLock<Mutex<HashMap<u64, u64>>> = OnceLock::new();

fn fp_index() -> &'static Mutex<HashMap<u64, u64>> {
    FP_INDEX.get_or_init(|| {
        let mut map = HashMap::new();
        if let Ok(rd) = std::fs::read_dir(tasks_dir()) {
            for e in rd.filter_map(|e| e.ok()) {
                if e.path().extension().and_then(|x| x.to_str()) != Some("json") { continue; }
                if let Ok(bytes) = std::fs::read(e.path()) {
                    if let Ok(entry) = serde_json::from_slice::<TaskEntry>(&bytes) {
                        if entry.fingerprint != 0 {
                            map.insert(entry.fingerprint, entry.task_id);
                        }
                    }
                }
            }
        }
        Mutex::new(map)
    })
}

fn task_find_by_fp(fp: u64) -> Option<TaskEntry> {
    if fp == 0 { return None; }
    let idx = fp_index().lock().ok()?;
    let id = idx.get(&fp).copied()?;
    drop(idx);
    task_read(id)
}

fn task_create_with_fp(task_type: &str, fingerprint: u64) -> u64 {
    if fingerprint != 0 {
        if let Ok(mut idx) = fp_index().lock() {
            if let Some(old_id) = idx.remove(&fingerprint) {
                let _ = std::fs::remove_file(task_path(old_id));
                let _ = std::fs::remove_file(result_path(old_id));
            }
        }
    }
    let id = gen_id();
    task_write(&TaskEntry {
        task_id: id,
        task_type: task_type.to_string(),
        status: TaskStatus::Running { progress: 0.0 },
        partial: PartialState::default(),
        created_ms: now_ms(),
        updated_ms: now_ms(),
        fingerprint,
        session: session_id(),
    });
    if fingerprint != 0 {
        if let Ok(mut idx) = fp_index().lock() {
            idx.insert(fingerprint, id);
        }
    }
    id
}

pub struct TaskHandle {
    pub id: u64,
    completed: std::cell::Cell<bool>,
    partial_state: PartialState,
    has_result_file: bool,
}

impl TaskHandle {
    pub fn auto(name: &str, fingerprint: u64) -> Self {
        if let Some(entry) = task_find_by_fp(fingerprint) {
            if entry.session != session_id() {
                match &entry.status {
                    TaskStatus::Completed => {
                        return Self {
                            id: entry.task_id,
                            completed: std::cell::Cell::new(true),
                            partial_state: entry.partial,
                            has_result_file: result_path(entry.task_id).exists(),
                        };
                    }
                    TaskStatus::Running { .. } => {
                        return Self {
                            id: entry.task_id,
                            completed: std::cell::Cell::new(false),
                            partial_state: entry.partial,
                            has_result_file: false,
                        };
                    }
                    _ => {}
                }
            }
        }
        let id = task_create_with_fp(name, fingerprint);
        Self { id, completed: std::cell::Cell::new(false), partial_state: PartialState::default(), has_result_file: false }
    }

    pub fn done(&self) -> bool { self.completed.get() }
    pub fn partial(&self) -> &PartialState { &self.partial_state }

    pub fn cached_result(&self) -> Option<String> {
        if self.has_result_file { std::fs::read_to_string(result_path(self.id)).ok() } else { None }
    }

    pub fn update(&self, partial: &PartialState, progress: f32) {
        task_update(self.id, partial, progress);
    }

    pub fn complete(&self, partial: &PartialState) {
        task_complete(self.id, partial);
        self.completed.set(true);
    }

    pub fn complete_with_result(&self, partial: &PartialState, result: &str) {
        task_complete(self.id, partial);
        let _ = std::fs::write(result_path(self.id), result.as_bytes());
        self.completed.set(true);
    }

    pub fn complete_result(&self, result: &str) {
        task_complete(self.id, &PartialState::default());
        let _ = std::fs::write(result_path(self.id), result.as_bytes());
        self.completed.set(true);
    }
}

impl Drop for TaskHandle {
    fn drop(&mut self) {
        if !self.completed.get() {
            task_complete(self.id, &PartialState::default());
        }
    }
}

pub fn task_create(task_type: &str) -> u64 {
    task_create_with_fp(task_type, 0)
}

pub fn task_load(id: u64) -> Option<TaskEntry> {
    task_read(id)
}

pub fn task_is_running(id: u64) -> bool {
    task_read(id).map(|e| matches!(e.status, TaskStatus::Running { .. })).unwrap_or(false)
}

pub fn task_update(id: u64, partial: &PartialState, progress: f32) {
    if let Some(mut entry) = task_read(id) {
        entry.partial = partial.clone();
        entry.status = TaskStatus::Running { progress };
        entry.updated_ms = now_ms();
        task_write(&entry);
    }
}

pub fn task_complete(id: u64, partial: &PartialState) {
    if let Some(mut entry) = task_read(id) {
        entry.partial = partial.clone();
        entry.status = TaskStatus::Completed;
        entry.updated_ms = now_ms();
        task_write(&entry);
    }
}

pub fn task_fail(id: u64, error: &str) {
    if let Some(mut entry) = task_read(id) {
        entry.status = TaskStatus::Failed { error: error.to_string() };
        entry.updated_ms = now_ms();
        task_write(&entry);
    }
}

pub fn task_delete(id: u64) {
    if let Some(entry) = task_read(id) {
        if entry.fingerprint != 0 {
            if let Ok(mut idx) = fp_index().lock() {
                idx.remove(&entry.fingerprint);
            }
        }
    }
    let _ = std::fs::remove_file(task_path(id));
    let _ = std::fs::remove_file(result_path(id));
    if let Ok(mut s) = mem_store().lock() {
        s.remove(&id);
    }
}

pub fn task_list_all() -> Vec<TaskEntry> {
    let Ok(rd) = std::fs::read_dir(tasks_dir()) else { return Vec::new(); };
    rd.filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("json"))
        .filter_map(|e| {
            let bytes = std::fs::read(e.path()).ok()?;
            serde_json::from_slice(&bytes).ok()
        })
        .collect()
}

pub fn task_dir_path() -> String {
    tasks_dir().to_string_lossy().into_owned()
}

pub fn task_cleanup_old(max_age_ms: u64) -> usize {
    let now = now_ms();
    let mut count = 0;
    let Ok(rd) = std::fs::read_dir(tasks_dir()) else { return 0; };
    for e in rd.filter_map(|e| e.ok()) {
        let path = e.path();
        if path.extension().and_then(|x| x.to_str()) != Some("json") { continue; }
        let Ok(bytes) = std::fs::read(&path) else { continue; };
        let Ok(task) = serde_json::from_slice::<TaskEntry>(&bytes) else { continue; };
        if matches!(task.status, TaskStatus::Completed) && now.saturating_sub(task.updated_ms) > max_age_ms {
            let _ = std::fs::remove_file(&path);
            let _ = std::fs::remove_file(result_path(task.task_id));
            if task.fingerprint != 0 {
                if let Ok(mut idx) = fp_index().lock() {
                    idx.remove(&task.fingerprint);
                }
            }
            count += 1;
        }
    }
    count
}

pub fn checkpoint_save(id: u64, weights: &[f64], iteration: usize) {
    if let Ok(mut s) = mem_store().lock() {
        s.insert(id, CheckpointEntry { weights: weights.to_vec(), iteration });
    }
    let mut entry = task_read(id).unwrap_or_else(|| TaskEntry {
        task_id: id,
        task_type: "fit".to_string(),
        status: TaskStatus::Running { progress: 0.0 },
        partial: PartialState::default(),
        created_ms: now_ms(),
        updated_ms: now_ms(),
        fingerprint: 0,
        session: session_id(),
    });
    entry.partial.weights = weights.to_vec();
    entry.partial.iteration = iteration;
    entry.updated_ms = now_ms();
    task_write(&entry);
}

pub fn checkpoint_load(id: u64) -> Option<CheckpointEntry> {
    if let Ok(s) = mem_store().lock() {
        if let Some(e) = s.get(&id) {
            return Some(CheckpointEntry { weights: e.weights.clone(), iteration: e.iteration });
        }
    }
    let entry = task_read(id)?;
    if !entry.partial.weights.is_empty() {
        Some(CheckpointEntry { weights: entry.partial.weights, iteration: entry.partial.iteration })
    } else {
        None
    }
}

pub fn checkpoint_clear(id: u64) {
    if let Ok(mut s) = mem_store().lock() {
        s.remove(&id);
    }
}

pub fn checkpoint_list() -> Vec<u64> {
    let mut ids: Vec<u64> = mem_store().lock()
        .map(|s| s.keys().copied().collect())
        .unwrap_or_default();
    for e in task_list_all() {
        if !ids.contains(&e.task_id) { ids.push(e.task_id); }
    }
    ids
}

pub trait Resumable {
    type YType;
    fn fit_resumable(&mut self, x: &[f64], n: usize, p: usize, y: &[Self::YType], checkpoint_id: Option<u64>);
}
