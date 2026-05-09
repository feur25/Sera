use std::sync::{Arc, OnceLock};
use super::memory_pool::{StringInterner, MemoryPool};

static INTERNER: OnceLock<StringInterner> = OnceLock::new();
static POOL: OnceLock<MemoryPool> = OnceLock::new();

#[inline]
pub fn get_interner() -> &'static StringInterner {
    INTERNER.get_or_init(StringInterner::new)
}

#[inline]
pub fn get_pool() -> &'static MemoryPool {
    POOL.get_or_init(MemoryPool::new)
}

pub struct CompactChartState {
    labels_interned: Vec<Arc<str>>,
    values: Vec<f64>,
}

impl CompactChartState {
    #[inline]
    pub fn new(cap: usize) -> Self {
        Self {
            labels_interned: Vec::with_capacity(cap),
            values: Vec::with_capacity(cap),
        }
    }

    #[inline]
    pub fn add_point(&mut self, label: String, value: f64) {
        let interned = get_interner().intern(&label);
        self.labels_interned.push(interned);
        self.values.push(value);
    }

    #[inline]
    pub fn to_minimal_json(&self) -> String {
        let mut json = get_pool().take_json();
        json.reserve(self.labels_interned.len() * 30 + self.values.len() * 15);
        
        json.push_str("{\"l\":[");
        for (i, label) in self.labels_interned.iter().enumerate() {
            if i > 0 { json.push(','); }
            json.push('"');
            for &ch in label.as_bytes() {
                match ch {
                    b'"' => json.push_str("\\\""),
                    b'\\' => json.push_str("\\\\"),
                    b'\n' => json.push_str("\\n"),
                    b'\r' => json.push_str("\\r"),
                    b'\t' => json.push_str("\\t"),
                    ch if ch < 32 => {
                        use std::fmt::Write;
                        let _ = write!(json, "\\u{:04x}", ch as u32);
                    },
                    ch => json.push(ch as char),
                }
            }
            json.push('"');
        }
        
        json.push_str("],\"v\":[");
        for (i, &v) in self.values.iter().enumerate() {
            if i > 0 { json.push(','); }
            use std::fmt::Write;
            let _ = write!(json, "{}", v);
        }
        json.push_str("]}");
        
        json
    }

    #[inline]
    pub fn memory_used(&self) -> usize {
        std::mem::size_of::<Self>() 
            + self.labels_interned.len() * std::mem::size_of::<Arc<str>>()
            + self.values.len() * std::mem::size_of::<f64>()
    }
}


