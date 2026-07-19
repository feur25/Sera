use crate::core::hw_profile::hw;
use rayon::prelude::*;

fn agg_sum(vals: &[f64]) -> f64 {
    vals.iter().sum()
}
fn agg_min(vals: &[f64]) -> f64 {
    vals.iter().copied().fold(f64::INFINITY, f64::min)
}
fn agg_max(vals: &[f64]) -> f64 {
    vals.iter().copied().fold(f64::NEG_INFINITY, f64::max)
}
fn agg_first(vals: &[f64]) -> f64 {
    vals[0]
}
fn agg_last(vals: &[f64]) -> f64 {
    vals[vals.len() - 1]
}
fn agg_median(vals: &[f64]) -> f64 {
    let mut sorted: Vec<f64> = vals.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    sorted[sorted.len() / 2]
}
fn agg_mean(vals: &[f64]) -> f64 {
    vals.iter().sum::<f64>() / vals.len() as f64
}

const AGGREGATORS: &[(&str, fn(&[f64]) -> f64)] = &[
    ("sum", agg_sum),
    ("min", agg_min),
    ("max", agg_max),
    ("first", agg_first),
    ("last", agg_last),
    ("median", agg_median),
];

fn agg_bucket(vals: &[f64], agg: &str) -> f64 {
    AGGREGATORS
        .iter()
        .find(|(k, _)| *k == agg)
        .map(|(_, f)| f(vals))
        .unwrap_or_else(|| agg_mean(vals))
}

pub fn bucket_downsample(x: &[f64], y: &[f64], max_points: usize, agg: &str) -> (Vec<f64>, Vec<f64>) {
    let n = x.len().min(y.len());
    if n == 0 || max_points == 0 || n <= max_points {
        return (x[..n].to_vec(), y[..n].to_vec());
    }
    let chunk = (n + max_points - 1) / max_points;
    let x_chunks: Vec<&[f64]> = x[..n].chunks(chunk).collect();
    let y_chunks: Vec<&[f64]> = y[..n].chunks(chunk).collect();
    x_chunks
        .par_iter()
        .zip(y_chunks.par_iter())
        .map(|(xs, ys)| (xs[xs.len() / 2], agg_bucket(ys, agg)))
        .unzip()
}

pub fn bucket_downsample_multi(
    x: &[f64],
    ys: &[&[f64]],
    max_points: usize,
    agg: &str,
) -> (Vec<f64>, Vec<Vec<f64>>) {
    let n = x.len();
    if n == 0 || max_points == 0 || n <= max_points {
        return (x.to_vec(), ys.iter().map(|y| y.to_vec()).collect());
    }
    let chunk = (n + max_points - 1) / max_points;
    let out_x: Vec<f64> = x.chunks(chunk).map(|xs| xs[xs.len() / 2]).collect();
    let out_ys: Vec<Vec<f64>> = ys
        .par_iter()
        .map(|y| y.chunks(chunk).map(|ys| agg_bucket(ys, agg)).collect())
        .collect();
    (out_x, out_ys)
}

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
        Self {
            chunk_size: hw().l2_chunk_elems,
        }
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

            if local_max > max {
                max = local_max;
            }
            if local_min < min {
                min = local_min;
            }
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
