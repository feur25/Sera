use std::sync::atomic::{AtomicUsize, Ordering};
use std::alloc::{GlobalAlloc, Layout};
use std::sync::Arc;

struct Arena {
    buffer: Vec<u8>,
    position: AtomicUsize,
    capacity: usize,
}

impl Arena {
    fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![0; capacity],
            position: AtomicUsize::new(0),
            capacity,
        }
    }

    #[inline(always)]
    fn alloc(&self, layout: Layout) -> Option<*mut u8> {
        let size = layout.size();
        let align = layout.align();
        
        let mut pos = self.position.load(Ordering::Relaxed);
        
        loop {
            let aligned_pos = (pos + align - 1) & !(align - 1);
            let new_pos = aligned_pos + size;
            
            if new_pos > self.capacity {
                return None;
            }
            
            match self.position.compare_exchange(
                pos,
                new_pos,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => return Some(unsafe { self.buffer.as_ptr().add(aligned_pos) as *mut u8 }),
                Err(actual_pos) => pos = actual_pos,
            }
        }
    }

    fn reset(&self) {
        self.position.store(0, Ordering::Release);
    }
}

pub struct SeraAllocator {
    arena: Arc<Arena>,
}

impl SeraAllocator {
    pub fn new(capacity: usize) -> Self {
        Self {
            arena: Arc::new(Arena::new(capacity)),
        }
    }

    pub fn reset(&self) {
        self.arena.reset();
    }
}

unsafe impl GlobalAlloc for SeraAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.arena
            .alloc(layout)
            .unwrap_or_else(|| std::alloc::System.alloc(layout))
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        std::alloc::System.dealloc(ptr, layout);
    }
}

pub struct RingBuffer<T> {
    buffer: std::sync::Arc<parking_lot::Mutex<Vec<T>>>,
    write_pos: AtomicUsize,
    read_pos: AtomicUsize,
    capacity: usize,
}

impl<T: Default + Clone> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        buffer.resize(capacity, T::default());
        
        Self {
            buffer: std::sync::Arc::new(parking_lot::Mutex::new(buffer)),
            write_pos: AtomicUsize::new(0),
            read_pos: AtomicUsize::new(0),
            capacity,
        }
    }

    #[inline(always)]
    pub fn push(&self, item: T) -> bool {
        let write = self.write_pos.load(Ordering::Relaxed);
        let next_write = (write + 1) % self.capacity;
        let read = self.read_pos.load(Ordering::Acquire);
        
        if next_write == read {
            return false;
        }
        
        let mut buf = self.buffer.lock();
        buf[write] = item;
        self.write_pos.store(next_write, Ordering::Release);
        true
    }

    #[inline(always)]
    pub fn pop(&self) -> Option<T>
    where
        T: Clone,
    {
        let read = self.read_pos.load(Ordering::Relaxed);
        let write = self.write_pos.load(Ordering::Acquire);
        
        if read == write {
            return None;
        }
        
        let mut buf = self.buffer.lock();
        let item = buf[read].clone();
        let next_read = (read + 1) % self.capacity;
        self.read_pos.store(next_read, Ordering::Release);
        Some(item)
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.read_pos.load(Ordering::Acquire) == self.write_pos.load(Ordering::Acquire)
    }

    pub fn reset(&self) {
        self.write_pos.store(0, Ordering::Release);
        self.read_pos.store(0, Ordering::Release);
    }
}


