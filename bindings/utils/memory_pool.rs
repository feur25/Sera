use std::sync::Arc;
use parking_lot::RwLock;

pub struct StringInterner {
    strings: Arc<RwLock<Vec<Arc<str>>>>,
    lookup: Arc<RwLock<std::collections::HashMap<String, usize>>>,
}

impl StringInterner {
    pub fn new() -> Self {
        Self {
            strings: Arc::new(RwLock::new(Vec::with_capacity(10000))),
            lookup: Arc::new(RwLock::new(std::collections::HashMap::with_capacity(10000))),
        }
    }

    #[inline]
    pub fn intern(&self, s: &str) -> Arc<str> {
        let mut lookup = self.lookup.write();
        if let Some(&idx) = lookup.get(s) {
            return Arc::clone(&self.strings.read()[idx]);
        }
        
        let mut strings = self.strings.write();
        let interned: Arc<str> = Arc::from(s);
        lookup.insert(s.to_string(), strings.len());
        strings.push(Arc::clone(&interned));
        interned
    }
}

pub struct MemoryPool {
    json_buf: parking_lot::Mutex<String>,
    svg_buf: parking_lot::Mutex<String>,
}

impl MemoryPool {
    pub fn new() -> Self {
        Self {
            json_buf: parking_lot::Mutex::new(String::with_capacity(1024 * 1024)),
            svg_buf: parking_lot::Mutex::new(String::with_capacity(512 * 1024)),
        }
    }

    #[inline]
    pub fn take_json(&self) -> String {
        let mut buf = self.json_buf.lock();
        std::mem::take(&mut *buf)
    }

    #[inline]
    pub fn return_json(&self, mut buf: String) {
        buf.clear();
        let mut stored = self.json_buf.lock();
        *stored = buf;
    }

    #[inline]
    pub fn take_svg(&self) -> String {
        let mut buf = self.svg_buf.lock();
        std::mem::take(&mut *buf)
    }

    #[inline]
    pub fn return_svg(&self, mut buf: String) {
        buf.clear();
        let mut stored = self.svg_buf.lock();
        *stored = buf;
    }
}


