use std::collections::HashMap;

pub trait Index<K, V> {
    fn insert(&mut self, key: K, value: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn contains(&self, key: &K) -> bool;
    fn len(&self) -> usize;
}

pub struct TraceIndex {
    traces: HashMap<String, usize>,
}

impl TraceIndex {
    pub fn new() -> Self {
        Self {
            traces: HashMap::new(),
        }
    }

    pub fn register(&mut self, id: String, index: usize) {
        self.traces.insert(id, index);
    }

    pub fn get_index(&self, id: &str) -> Option<usize> {
        self.traces.get(id).copied()
    }

    pub fn contains(&self, id: &str) -> bool {
        self.traces.contains_key(id)
    }

    pub fn len(&self) -> usize {
        self.traces.len()
    }
}

impl Default for TraceIndex {
    fn default() -> Self {
        Self::new()
    }
}


