use super::adaptive_exec::{effective_chunk, effective_par_threshold, with_retry};
use super::hw_profile::hw;
use rayon::prelude::*;

#[inline]
pub fn map_par<T, U, F>(data: &[T], f: F) -> Vec<U>
where
    T: Sync,
    U: Send,
    F: Fn(&T) -> U + Sync + Send,
{
    with_retry(|| {
        if data.len() >= effective_par_threshold() {
            data.par_iter().map(&f).collect()
        } else {
            data.iter().map(&f).collect()
        }
    })
}

#[inline]
pub fn map_par_into<T, U, F>(data: &[T], out: &mut Vec<U>, f: F)
where
    T: Sync,
    U: Send,
    F: Fn(&T) -> U + Sync + Send,
{
    out.clear();
    if data.len() >= effective_par_threshold() {
        out.extend(data.par_iter().map(f).collect::<Vec<_>>());
    } else {
        out.extend(data.iter().map(f));
    }
}

#[inline]
pub fn reduce_par<T, U, F, C>(data: &[T], init: U, fold_fn: F, combine: C) -> U
where
    T: Sync,
    U: Send + Sync + Clone,
    F: Fn(U, &T) -> U + Sync + Send,
    C: Fn(U, U) -> U + Sync + Send,
{
    if data.len() >= effective_par_threshold() {
        data.par_iter()
            .fold(|| init.clone(), |acc, x| fold_fn(acc, x))
            .reduce(|| init.clone(), combine)
    } else {
        data.iter().fold(init, |acc, x| fold_fn(acc, x))
    }
}

#[inline]
pub fn for_each_chunks<T, F>(data: &[T], f: F)
where
    T: Sync,
    F: Fn(&[T]) + Sync + Send,
{
    let t_size = std::mem::size_of::<T>().max(1);
    let chunk = effective_chunk(hw().l2_chunk_elems * 8 / t_size).max(64);
    if data.len() >= effective_par_threshold() {
        data.par_chunks(chunk).for_each(f);
    } else {
        data.chunks(chunk).for_each(f);
    }
}

#[inline]
pub fn minmax_par(data: &[f64]) -> (f64, f64) {
    if data.is_empty() {
        return (0.0, 0.0);
    }
    let chunk = effective_chunk(hw().l2_chunk_elems);
    with_retry(|| {
        if data.len() >= effective_par_threshold() {
            data.par_chunks(chunk)
                .map(|c| {
                    c.iter()
                        .copied()
                        .fold((f64::INFINITY, f64::NEG_INFINITY), |(mn, mx), v| {
                            (mn.min(v), mx.max(v))
                        })
                })
                .reduce(
                    || (f64::INFINITY, f64::NEG_INFINITY),
                    |(amn, amx), (bmn, bmx)| (amn.min(bmn), amx.max(bmx)),
                )
        } else {
            data.iter()
                .copied()
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(mn, mx), v| {
                    (mn.min(v), mx.max(v))
                })
        }
    })
}

#[inline]
pub fn sum_par(data: &[f64]) -> f64 {
    if data.len() >= effective_par_threshold() {
        data.par_iter().copied().sum()
    } else {
        data.iter().copied().sum()
    }
}

pub struct SliceStats {
    pub min: f64,
    pub max: f64,
    pub sum: f64,
    pub count: usize,
}

impl SliceStats {
    #[inline]
    pub fn mean(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.sum / self.count as f64
        }
    }
}

#[inline]
pub fn stats_par(data: &[f64]) -> SliceStats {
    if data.is_empty() {
        return SliceStats {
            min: 0.0,
            max: 0.0,
            sum: 0.0,
            count: 0,
        };
    }
    let chunk = effective_chunk(hw().l2_chunk_elems);
    let (min, max, sum) = with_retry(|| {
        if data.len() >= effective_par_threshold() {
            data.par_chunks(chunk)
                .map(|c| {
                    let (mut mn, mut mx, mut s) = (f64::INFINITY, f64::NEG_INFINITY, 0.0f64);
                    for &v in c {
                        mn = mn.min(v);
                        mx = mx.max(v);
                        s += v;
                    }
                    (mn, mx, s)
                })
                .reduce(
                    || (f64::INFINITY, f64::NEG_INFINITY, 0.0),
                    |(amn, amx, as_), (bmn, bmx, bs)| (amn.min(bmn), amx.max(bmx), as_ + bs),
                )
        } else {
            let (mut mn, mut mx, mut s) = (f64::INFINITY, f64::NEG_INFINITY, 0.0f64);
            for &v in data {
                mn = mn.min(v);
                mx = mx.max(v);
                s += v;
            }
            (mn, mx, s)
        }
    });
    SliceStats {
        min,
        max,
        sum,
        count: data.len(),
    }
}

#[inline]
pub fn std_dev_par(data: &[f64], mean: f64) -> f64 {
    if data.len() < 2 {
        return 0.0;
    }
    let chunk = effective_chunk(hw().l2_chunk_elems);
    let sum_sq = if data.len() >= effective_par_threshold() {
        data.par_chunks(chunk)
            .map(|c| c.iter().map(|&x| (x - mean) * (x - mean)).sum::<f64>())
            .sum::<f64>()
    } else {
        data.iter().map(|&x| (x - mean) * (x - mean)).sum::<f64>()
    };
    (sum_sq / (data.len() - 1) as f64).sqrt()
}
