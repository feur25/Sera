use parking_lot::RwLock;
use std::sync::Arc;

pub struct RingBuffer<T: Copy, const N: usize> {
    buf: [T; N],
    head: usize,
    len: usize,
}

impl<T: Copy + Default, const N: usize> RingBuffer<T, N> {
    #[inline]
    pub fn new() -> Self {
        Self {
            buf: [T::default(); N],
            head: 0,
            len: 0,
        }
    }

    #[inline]
    pub fn push(&mut self, val: T) {
        if self.len < N {
            self.buf[(self.head + self.len) % N] = val;
            self.len += 1;
        } else {
            self.buf[self.head] = val;
            self.head = (self.head + 1) % N;
        }
    }

    #[inline]
    pub fn get(&self, idx: usize) -> Option<T> {
        if idx >= self.len {
            return None;
        }
        Some(self.buf[(self.head + idx) % N])
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.len == N
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        (0..self.len).map(move |i| self.buf[(self.head + i) % N])
    }

    #[inline]
    pub fn clear(&mut self) {
        self.head = 0;
        self.len = 0;
    }
}

impl<T: Copy + Default + std::iter::Sum, const N: usize> RingBuffer<T, N> {
    #[inline]
    pub fn sum(&self) -> T {
        self.iter().sum()
    }
}

impl<const N: usize> RingBuffer<f64, N> {
    #[inline]
    pub fn mean(&self) -> f64 {
        if self.len == 0 {
            return 0.0;
        }
        self.iter().sum::<f64>() / self.len as f64
    }

    #[inline]
    pub fn std_dev(&self) -> f64 {
        if self.len < 2 {
            return 0.0;
        }
        let m = self.mean();
        let v = self.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (self.len - 1) as f64;
        v.sqrt()
    }

    #[inline]
    pub fn min(&self) -> f64 {
        self.iter().fold(f64::INFINITY, f64::min)
    }

    #[inline]
    pub fn max(&self) -> f64 {
        self.iter().fold(f64::NEG_INFINITY, f64::max)
    }
}

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
