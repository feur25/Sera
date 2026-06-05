pub struct BitSet {
    words: Vec<u64>,
    capacity: usize,
}

impl BitSet {
    #[inline(always)]
    pub fn new(size: usize) -> Self {
        let words = (size + 63) / 64;
        Self {
            words: vec![!0u64; words],
            capacity: size,
        }
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    #[inline(always)]
    pub fn set(&mut self, index: usize) {
        let word_idx = index / 64;
        let bit_idx = index % 64;
        if word_idx < self.words.len() {
            self.words[word_idx] |= 1u64 << bit_idx;
        }
    }

    #[inline(always)]
    pub fn clear(&mut self, index: usize) {
        let word_idx = index / 64;
        let bit_idx = index % 64;
        if word_idx < self.words.len() {
            self.words[word_idx] &= !(1u64 << bit_idx);
        }
    }

    #[inline(always)]
    pub fn get(&self, index: usize) -> bool {
        let word_idx = index / 64;
        let bit_idx = index % 64;
        if word_idx < self.words.len() {
            (self.words[word_idx] >> bit_idx) & 1 == 1
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn count(&self) -> usize {
        self.words.iter().map(|w| w.count_ones() as usize).sum()
    }

    pub fn clear_all(&mut self) {
        for word in &mut self.words {
            *word = 0;
        }
    }

    pub fn set_all(&mut self) {
        for word in &mut self.words {
            *word = !0u64;
        }
    }

    #[inline(always)]
    pub fn iter_set(&self) -> impl Iterator<Item = usize> + '_ {
        self.words.iter().enumerate().flat_map(|(word_idx, &word)| {
            (0..64).filter_map(move |bit_idx| {
                if (word >> bit_idx) & 1 == 1 {
                    Some(word_idx * 64 + bit_idx)
                } else {
                    None
                }
            })
        })
    }
}

pub struct FastLookupTable {
    table: Vec<f32>,
}

impl FastLookupTable {
    pub fn new(size: usize) -> Self {
        Self {
            table: vec![0.0; size],
        }
    }

    #[inline(always)]
    pub fn get(&self, idx: usize) -> f32 {
        *self.table.get(idx).unwrap_or(&0.0)
    }

    #[inline(always)]
    pub fn set(&mut self, idx: usize, val: f32) {
        if idx < self.table.len() {
            self.table[idx] = val;
        }
    }

    pub fn populate_from_fn<F>(&mut self, f: F)
    where
        F: Fn(usize) -> f32,
    {
        for i in 0..self.table.len() {
            self.table[i] = f(i);
        }
    }
}

#[repr(align(64))]
pub struct CacheAlignedData<T> {
    data: T,
}

impl<T> CacheAlignedData<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }

    #[inline(always)]
    pub fn get(&self) -> &T {
        &self.data
    }

    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.data
    }

    #[inline(always)]
    pub fn into_inner(self) -> T {
        self.data
    }
}
