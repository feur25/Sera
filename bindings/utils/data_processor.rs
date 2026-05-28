use crate::core::hw_profile::hw;

pub struct DataProcessor {
    chunk_size: usize,
}

impl DataProcessor {
    #[inline]
    pub fn new(chunk_size: usize) -> Self {
        Self { chunk_size }
    }

    #[inline]
    pub fn adaptive() -> Self {
        Self { chunk_size: hw().l2_chunk_elems }
    }

    #[inline(always)]
    pub fn find_max_chunk(chunk: &[f64]) -> f64 {
        chunk.iter().copied().fold(f64::NEG_INFINITY, f64::max)
    }

    #[inline(always)]
    pub fn find_min_chunk(chunk: &[f64]) -> f64 {
        chunk.iter().copied().fold(f64::INFINITY, f64::min)
    }

    pub fn compute_bounds(&self, values: &[f64]) -> (f64, f64) {
        let mut max = f64::NEG_INFINITY;
        let mut min = f64::INFINITY;

        for chunk in values.chunks(self.chunk_size) {
            let local_max = Self::find_max_chunk(chunk);
            let local_min = Self::find_min_chunk(chunk);
            
            if local_max > max { max = local_max; }
            if local_min < min { min = local_min; }
        }

        (min, max)
    }

    pub fn normalize_vec(&self, values: &[f64], max_val: f64) -> Vec<f32> {
        let mut result = Vec::with_capacity(values.len());
        
        if max_val > 0.0 {
            for &v in values {
                result.push((v / max_val) as f32);
            }
        }
        
        result
    }
}


