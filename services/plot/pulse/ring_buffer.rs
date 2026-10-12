pub(crate) struct RingBuffer<T> {
    buf: Vec<T>,
    cap: usize,
    write: usize,
    len: usize,
}

#[allow(dead_code)]
impl<T> RingBuffer<T> {
    pub fn with_capacity(cap: usize) -> Self {
        let cap = cap.max(1);
        RingBuffer { buf: Vec::with_capacity(cap), cap, write: 0, len: 0 }
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.cap
    }

    pub fn push(&mut self, value: T) -> usize {
        let slot = self.write;
        if self.buf.len() < self.cap {
            self.buf.push(value);
        } else {
            self.buf[slot] = value;
        }
        self.write = (self.write + 1) % self.cap;
        self.len = (self.len + 1).min(self.cap);
        slot
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let start = if self.is_full() { self.write } else { 0 };
        let n = self.len;
        let cap = self.cap;
        (0..n).map(move |i| &self.buf[(start + i) % cap])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_fresh_buffer_is_empty_and_reports_the_requested_capacity() {
        let rb: RingBuffer<i32> = RingBuffer::with_capacity(4);
        assert_eq!(rb.capacity(), 4);
        assert_eq!(rb.len(), 0);
        assert!(rb.is_empty());
        assert!(!rb.is_full());
    }

    #[test]
    fn zero_capacity_is_clamped_to_one_instead_of_panicking() {
        let mut rb: RingBuffer<i32> = RingBuffer::with_capacity(0);
        assert_eq!(rb.capacity(), 1);
        rb.push(1);
        rb.push(2);
        assert_eq!(rb.len(), 1);
        assert_eq!(rb.iter().copied().collect::<Vec<_>>(), vec![2]);
    }

    #[test]
    fn pushes_below_capacity_are_kept_in_insertion_order() {
        let mut rb = RingBuffer::with_capacity(5);
        for v in [10, 20, 30] {
            rb.push(v);
        }
        assert_eq!(rb.len(), 3);
        assert!(!rb.is_full());
        assert_eq!(rb.iter().copied().collect::<Vec<_>>(), vec![10, 20, 30]);
    }

    #[test]
    fn pushes_beyond_capacity_overwrite_the_oldest_entry_first() {
        let mut rb = RingBuffer::with_capacity(3);
        for v in [1, 2, 3, 4, 5] {
            rb.push(v);
        }
        assert!(rb.is_full());
        assert_eq!(rb.len(), 3);
        assert_eq!(rb.iter().copied().collect::<Vec<_>>(), vec![3, 4, 5]);
    }

    #[test]
    fn push_returns_the_wrapped_slot_index_it_wrote_to() {
        let mut rb: RingBuffer<i32> = RingBuffer::with_capacity(3);
        let slots: Vec<usize> = (0..7).map(|i| rb.push(i)).collect();
        assert_eq!(slots, vec![0, 1, 2, 0, 1, 2, 0]);
    }

    #[test]
    fn never_reallocates_past_its_initial_capacity() {
        let mut rb: RingBuffer<u8> = RingBuffer::with_capacity(16);
        for i in 0..10_000u32 {
            rb.push(i as u8);
        }
        assert_eq!(rb.buf.capacity(), 16);
        assert_eq!(rb.len(), 16);
    }

    #[test]
    fn a_large_ring_buffer_sustains_a_high_push_throughput() {
        let cap = 100_000usize;
        let mut rb: RingBuffer<f32> = RingBuffer::with_capacity(cap);
        let n = 5_000_000usize;
        let start = std::time::Instant::now();
        for i in 0..n {
            rb.push(i as f32);
        }
        let elapsed = start.elapsed();
        let per_sec = n as f64 / elapsed.as_secs_f64();
        println!("ring buffer perf: {n} pushes into a {cap}-capacity buffer in {elapsed:?} -> {per_sec:.0} pushes/sec");
        assert!(rb.is_full());
        assert_eq!(rb.len(), cap);
        assert!(per_sec > 1_000_000.0, "expected well over 1M pushes/sec for a plain overwrite loop, got {per_sec:.0}/sec");
    }
}
