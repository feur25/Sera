use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct MemoryBlock {
    data: Vec<u8>,
    capacity: usize,
}

impl MemoryBlock {
    #[inline]
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        self.data.clear();
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    #[inline]
    pub fn push_bytes(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

pub struct MemoryPool {
    blocks: Vec<UnsafeCell<MemoryBlock>>,
    current: AtomicUsize,
}

impl MemoryPool {
    pub fn new(block_count: usize, block_size: usize) -> Arc<Self> {
        let mut blocks = Vec::with_capacity(block_count);
        for _ in 0..block_count {
            blocks.push(UnsafeCell::new(MemoryBlock::new(block_size)));
        }
        Arc::new(Self {
            blocks,
            current: AtomicUsize::new(0),
        })
    }

    #[inline]
    pub fn acquire(&self) -> Option<&UnsafeCell<MemoryBlock>> {
        let idx = self.current.fetch_add(1, Ordering::Relaxed) % self.blocks.len();
        self.blocks.get(idx)
    }

    pub fn cycle(&self) {
        self.current.store(0, Ordering::Release);
    }
}

unsafe impl Send for MemoryPool {}
unsafe impl Sync for MemoryPool {}

pub struct RingBuffer {
    buffer: Vec<u8>,
    read_pos: usize,
    write_pos: usize,
    capacity: usize,
}

impl RingBuffer {
    #[inline]
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![0u8; capacity],
            read_pos: 0,
            write_pos: 0,
            capacity,
        }
    }

    #[inline]
    pub fn write(&mut self, data: &[u8]) -> bool {
        let len = data.len();
        if self.available_write() < len {
            return false;
        }
        
        for &byte in data {
            self.buffer[self.write_pos] = byte;
            self.write_pos = (self.write_pos + 1) % self.capacity;
        }
        true
    }

    #[inline]
    pub fn read(&mut self, len: usize) -> Option<Vec<u8>> {
        if self.available_read() < len {
            return None;
        }
        
        let mut result = Vec::with_capacity(len);
        for _ in 0..len {
            result.push(self.buffer[self.read_pos]);
            self.read_pos = (self.read_pos + 1) % self.capacity;
        }
        Some(result)
    }

    #[inline]
    pub fn available_write(&self) -> usize {
        if self.write_pos >= self.read_pos {
            self.capacity - (self.write_pos - self.read_pos)
        } else {
            self.read_pos - self.write_pos
        }
    }

    #[inline]
    pub fn available_read(&self) -> usize {
        if self.write_pos >= self.read_pos {
            self.write_pos - self.read_pos
        } else {
            self.capacity - (self.read_pos - self.write_pos)
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.read_pos = 0;
        self.write_pos = 0;
    }
}

pub struct StackBuffer<const N: usize> {
    data: [u8; N],
    len: usize,
}

impl<const N: usize> StackBuffer<N> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            data: [0u8; N],
            len: 0,
        }
    }

    #[inline]
    pub fn push(&mut self, byte: u8) {
        if self.len < N {
            self.data[self.len] = byte;
            self.len += 1;
        }
    }

    #[inline]
    pub fn push_slice(&mut self, slice: &[u8]) {
        for &byte in slice {
            self.push(byte);
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.len]
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn reset(&mut self) {
        self.len = 0;
    }

    #[inline]
    pub fn to_vec(&self) -> Vec<u8> {
        self.as_slice().to_vec()
    }
}

pub struct CompactBuffer {
    data: Vec<u8>,
    chunks: Vec<(usize, usize)>,
}

impl CompactBuffer {
    #[inline]
    pub fn new(estimated_size: usize) -> Self {
        Self {
            data: Vec::with_capacity(estimated_size),
            chunks: Vec::new(),
        }
    }

    #[inline]
    pub fn write_chunk(&mut self, bytes: &[u8]) {
        let start = self.data.len();
        self.data.extend_from_slice(bytes);
        let end = self.data.len();
        self.chunks.push((start, end));
    }

    #[inline]
    pub fn get_chunk(&self, idx: usize) -> Option<&[u8]> {
        self.chunks.get(idx).map(|(start, end)| &self.data[*start..*end])
    }

    #[inline]
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    #[inline]
    pub fn as_vec(&self) -> &Vec<u8> {
        &self.data
    }

    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.data
    }

    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
        self.chunks.clear();
    }
}


