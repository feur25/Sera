use std::cell::RefCell;

struct PooledBuf {
    bufs: Vec<String>,
}

impl PooledBuf {
    fn new() -> Self {
        Self {
            bufs: Vec::with_capacity(8),
        }
    }

    fn acquire(&mut self, capacity: usize) -> String {
        if let Some(mut s) = self.bufs.pop() {
            s.clear();
            if s.capacity() < capacity {
                s.reserve(capacity - s.capacity());
            }
            s
        } else {
            String::with_capacity(capacity)
        }
    }

    fn release(&mut self, s: String) {
        if self.bufs.len() < 8 && s.capacity() <= 4 * 1024 * 1024 {
            self.bufs.push(s);
        }
    }
}

thread_local! {
    static RENDER_POOL: RefCell<PooledBuf> = RefCell::new(PooledBuf::new());
}

#[inline]
pub fn acquire_buf(capacity: usize) -> String {
    RENDER_POOL.with(|p| p.borrow_mut().acquire(capacity))
}

#[inline]
pub fn release_buf(s: String) {
    RENDER_POOL.with(|p| p.borrow_mut().release(s));
}

#[inline]
pub fn with_buf<R, F: FnOnce(&mut String) -> R>(capacity: usize, f: F) -> String {
    let mut buf = acquire_buf(capacity);
    f(&mut buf);
    buf
}

pub struct RenderArena {
    svg: String,
    json: String,
}

impl RenderArena {
    #[inline]
    pub fn new(svg_cap: usize, json_cap: usize) -> Self {
        Self {
            svg: acquire_buf(svg_cap),
            json: acquire_buf(json_cap),
        }
    }

    #[inline]
    pub fn svg_buf(&mut self) -> &mut String {
        &mut self.svg
    }

    #[inline]
    pub fn json_buf(&mut self) -> &mut String {
        &mut self.json
    }

    #[inline]
    pub fn take_svg(&mut self) -> String {
        std::mem::replace(&mut self.svg, acquire_buf(0))
    }

    #[inline]
    pub fn take_json(&mut self) -> String {
        std::mem::replace(&mut self.json, acquire_buf(0))
    }
}

impl Drop for RenderArena {
    fn drop(&mut self) {
        let svg = std::mem::take(&mut self.svg);
        let json = std::mem::take(&mut self.json);
        release_buf(svg);
        release_buf(json);
    }
}
