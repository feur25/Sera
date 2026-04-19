use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub struct CheckpointEntry {
    pub weights: Vec<f64>,
    pub iteration: usize,
}

static STORE: OnceLock<Mutex<HashMap<u64, CheckpointEntry>>> = OnceLock::new();

fn store() -> &'static Mutex<HashMap<u64, CheckpointEntry>> {
    STORE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn checkpoint_save(id: u64, weights: &[f64], iteration: usize) {
    if let Ok(mut s) = store().lock() {
        s.insert(id, CheckpointEntry { weights: weights.to_vec(), iteration });
    }
}

pub fn checkpoint_load(id: u64) -> Option<CheckpointEntry> {
    if let Ok(s) = store().lock() {
        s.get(&id).map(|e| CheckpointEntry { weights: e.weights.clone(), iteration: e.iteration })
    } else {
        None
    }
}

pub fn checkpoint_clear(id: u64) {
    if let Ok(mut s) = store().lock() {
        s.remove(&id);
    }
}

pub fn checkpoint_list() -> Vec<u64> {
    if let Ok(s) = store().lock() {
        s.keys().copied().collect()
    } else {
        Vec::new()
    }
}

pub trait Resumable {
    type YType;
    fn fit_resumable(&mut self, x: &[f64], n: usize, p: usize, y: &[Self::YType], checkpoint_id: Option<u64>);
}
