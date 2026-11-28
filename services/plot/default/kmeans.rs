#![allow(dead_code)]

use crate::bindings::utils::simd_ops::find_minmax;
use crate::html::hover::{html_id, html_prefix, html_suffix};
use crate::plot::parse_all;
use crate::plot::statistical::common::{hex6, palette_color, push_b, push_f2, push_i};
use base64::Engine as _;
use rayon::prelude::*;
use std::collections::HashSet;

crate::chart_config!(KMeansConfig, 1000, 580;
    struct {
        pub x_values: &'a [f64],
        pub y_values: &'a [f64],
        pub k: usize,
        pub max_iter: usize,
        pub tol: f64,
        pub mini_batch: bool,
        pub batch_size: usize,
        pub palette: &'a [u32],
    }
    defaults {
        x_values: &[],
        y_values: &[],
        k: 3,
        max_iter: 300,
        tol: 1e-4,
        mini_batch: false,
        batch_size: 1000,
        palette: &[],
    }
);

#[inline(always)]
fn sq_dist_2d(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    let dx = ax - bx;
    let dy = ay - by;
    dx * dx + dy * dy
}

#[inline(always)]
pub fn sq_dist_nd(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len().min(b.len());
    let mut d = 0.0f64;
    let mut i = 0;
    while i + 4 <= n {
        let (d0, d1) = (a[i] - b[i], a[i + 1] - b[i + 1]);
        let (d2, d3) = (a[i + 2] - b[i + 2], a[i + 3] - b[i + 3]);
        d += d0 * d0 + d1 * d1 + d2 * d2 + d3 * d3;
        i += 4;
    }
    while i < n {
        let di = a[i] - b[i];
        d += di * di;
        i += 1;
    }
    d
}

fn xorshift64(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

/// SplitMix64 — much better statistical quality than xorshift64.
/// Used for k-means++ D²-weighted sampling where PRNG quality matters.
fn splitmix64(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

fn kmeans_pp_seed(s: &[f64], k: usize, seed: u64) -> Vec<f64> {
    let n = s.len();
    let k = k.min(n);
    let mut rng = seed.wrapping_add(n as u64).wrapping_mul(0xDEADBEEF_u64);
    let first = (splitmix64(&mut rng) as usize) % n;
    let mut cx = vec![s[first]];
    let mut dists = vec![f64::INFINITY; n];
    for _ in 1..k {
        let lc = *cx.last().unwrap();
        let mut total = 0.0f64;
        for i in 0..n {
            let dx = s[i] - lc;
            let d = dx * dx;
            if d < dists[i] {
                dists[i] = d;
            }
            total += dists[i];
        }
        if total <= 0.0 {
            break;
        }
        let mut target = (splitmix64(&mut rng) as f64 / u64::MAX as f64) * total;
        let mut chosen = n - 1;
        for (i, &d) in dists.iter().enumerate() {
            target -= d;
            if target <= 0.0 {
                chosen = i;
                break;
            }
        }
        cx.push(s[chosen]);
    }
    cx
}

fn kmeans_pp_2d(x: &[f64], y: &[f64], k: usize, seed: u64) -> (Vec<f64>, Vec<f64>) {
    let n = x.len();
    let k = k.min(n);
    let mut rng = seed.wrapping_add(n as u64).wrapping_mul(0xDEADBEEF_u64);
    let first = (splitmix64(&mut rng) as usize) % n;
    let mut cx = vec![x[first]];
    let mut cy = vec![y[first]];
    let mut dists = vec![f64::INFINITY; n];
    for _ in 1..k {
        let (lx, ly) = (*cx.last().unwrap(), *cy.last().unwrap());
        let mut total = 0.0f64;
        for i in 0..n {
            let d = sq_dist_2d(x[i], y[i], lx, ly);
            if d < dists[i] {
                dists[i] = d;
            }
            total += dists[i];
        }
        if total <= 0.0 {
            break;
        }
        let mut target = (splitmix64(&mut rng) as f64 / u64::MAX as f64) * total;
        let mut chosen = n - 1;
        for (i, &d) in dists.iter().enumerate() {
            target -= d;
            if target <= 0.0 {
                chosen = i;
                break;
            }
        }
        cx.push(x[chosen]);
        cy.push(y[chosen]);
    }
    (cx, cy)
}

pub fn kmeans_pp_flat(data: &[f64], n: usize, dims: usize, k: usize, seed: u64) -> Vec<f64> {
    let k = k.min(n);
    if k == 0 {
        return Vec::new();
    }
    let n_trials = if k >= 100 {
        2
    } else {
        2 + (k as f64).ln() as usize
    };
    let mut rng = seed.wrapping_add(n as u64).wrapping_mul(0xCAFEBABE_u64);
    let first = (splitmix64(&mut rng) as usize) % n;
    let mut centroids = Vec::with_capacity(k * dims);
    centroids.extend_from_slice(&data[first * dims..(first + 1) * dims]);
    let mut dists = vec![f64::MAX; n];
    for ki in 1..k {
        let prev = &centroids[(ki - 1) * dims..ki * dims];
        let mut total = 0.0f64;
        for i in 0..n {
            let d = sq_dist_flat(&data[i * dims..(i + 1) * dims], prev);
            if d < dists[i] {
                dists[i] = d;
            }
            total += dists[i];
        }
        if total <= 0.0 {
            for kj in ki..k {
                let fallback = (kj * 7 + 3) % n;
                centroids.extend_from_slice(&data[fallback * dims..(fallback + 1) * dims]);
            }
            break;
        }
        // Sample n_trials candidates, pick the one with lowest resulting potential
        let mut best_idx = n - 1;
        let mut best_potential = f64::INFINITY;
        for _ in 0..n_trials {
            let mut target = (splitmix64(&mut rng) as f64 / u64::MAX as f64) * total;
            let mut chosen = n - 1;
            for (i, &d) in dists.iter().enumerate() {
                target -= d;
                if target <= 0.0 {
                    chosen = i;
                    break;
                }
            }
            let cand = &data[chosen * dims..(chosen + 1) * dims];
            let potential: f64 = (0..n)
                .map(|i| dists[i].min(sq_dist_flat(&data[i * dims..(i + 1) * dims], cand)))
                .sum();
            if potential < best_potential {
                best_potential = potential;
                best_idx = chosen;
            }
        }
        centroids.extend_from_slice(&data[best_idx * dims..(best_idx + 1) * dims]);
    }
    centroids
}

/// Parallel k-means++ init with trial candidates (matching sklearn's improved k-means++).
/// For each centroid, samples `2+ln(k)` candidates and picks the one that minimises total potential.
fn kmeans_pp_flat_par(data: &[f64], n: usize, dims: usize, k: usize, seed: u64) -> Vec<f64> {
    let k = k.min(n);
    if k == 0 {
        return Vec::new();
    }
    let n_trials = if k >= 100 {
        2
    } else {
        2 + (k as f64).ln() as usize
    };
    let mut rng = seed.wrapping_add(n as u64).wrapping_mul(0xCAFEBABE_u64);
    let first = (splitmix64(&mut rng) as usize) % n;
    let mut centroids = Vec::with_capacity(k * dims);
    centroids.extend_from_slice(&data[first * dims..(first + 1) * dims]);
    let mut dists = vec![f64::MAX; n];
    for ki in 1..k {
        let prev = centroids[(ki - 1) * dims..ki * dims].to_vec();
        let total: f64 = dists
            .par_chunks_mut(HAMERLY_CHUNK)
            .zip(data.par_chunks(HAMERLY_CHUNK * dims))
            .map(|(dist_c, data_c)| {
                let mut chunk_sum = 0.0f64;
                for i in 0..dist_c.len() {
                    let d = sq_dist_flat(&data_c[i * dims..(i + 1) * dims], &prev);
                    if d < dist_c[i] {
                        dist_c[i] = d;
                    }
                    chunk_sum += dist_c[i];
                }
                chunk_sum
            })
            .sum();
        if total <= 0.0 {
            for kj in ki..k {
                let fallback = (kj * 7 + 3) % n;
                centroids.extend_from_slice(&data[fallback * dims..(fallback + 1) * dims]);
            }
            break;
        }
        // Trial candidates — pick the one with lowest resulting potential (parallel)
        let mut best_idx = n - 1;
        let mut best_potential = f64::INFINITY;
        for _ in 0..n_trials {
            let mut target = (splitmix64(&mut rng) as f64 / u64::MAX as f64) * total;
            let mut chosen = n - 1;
            for (i, &d) in dists.iter().enumerate() {
                target -= d;
                if target <= 0.0 {
                    chosen = i;
                    break;
                }
            }
            let cand = data[chosen * dims..(chosen + 1) * dims].to_vec();
            let potential: f64 = dists
                .par_chunks(HAMERLY_CHUNK)
                .zip(data.par_chunks(HAMERLY_CHUNK * dims))
                .map(|(dist_c, data_c)| {
                    dist_c
                        .iter()
                        .enumerate()
                        .map(|(i, &d)| {
                            d.min(sq_dist_flat(&data_c[i * dims..(i + 1) * dims], &cand))
                        })
                        .sum::<f64>()
                })
                .sum();
            if potential < best_potential {
                best_potential = potential;
                best_idx = chosen;
            }
        }
        centroids.extend_from_slice(&data[best_idx * dims..(best_idx + 1) * dims]);
    }
    centroids
}

#[inline(always)]
pub fn sq_dist_flat(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len().min(b.len());
    let mut d = 0.0f64;
    let mut i = 0;
    while i + 4 <= n {
        let (d0, d1) = (a[i] - b[i], a[i + 1] - b[i + 1]);
        let (d2, d3) = (a[i + 2] - b[i + 2], a[i + 3] - b[i + 3]);
        d += d0 * d0 + d1 * d1 + d2 * d2 + d3 * d3;
        i += 4;
    }
    while i < n {
        let di = a[i] - b[i];
        d += di * di;
        i += 1;
    }
    d
}

fn kmeans_assign_sequential(
    data: &[f64],
    n: usize,
    dims: usize,
    k: usize,
    centroids: &[f64],
    labels: &mut [i32],
) -> f64 {
    let mut inertia = 0.0f64;
    for i in 0..n {
        let pt = &data[i * dims..(i + 1) * dims];
        let (mut bc, mut bd) = (0usize, f64::INFINITY);
        for ki in 0..k {
            let d = sq_dist_flat(pt, &centroids[ki * dims..(ki + 1) * dims]);
            if d < bd {
                bd = d;
                bc = ki;
            }
        }
        labels[i] = bc as i32;
        inertia += bd;
    }
    inertia
}

fn kmeans_update_centroids(
    data: &[f64],
    n: usize,
    dims: usize,
    k: usize,
    labels: &[i32],
    centroids: &mut Vec<f64>,
) {
    let mut sums = vec![0.0f64; k * dims];
    let mut cnt = vec![0u32; k];
    for i in 0..n {
        let ci = labels[i] as usize;
        cnt[ci] += 1;
        for d in 0..dims {
            sums[ci * dims + d] += data[i * dims + d];
        }
    }
    *centroids = vec![0.0f64; k * dims];
    for ki in 0..k {
        if cnt[ki] > 0 {
            let c = cnt[ki] as f64;
            for d in 0..dims {
                centroids[ki * dims + d] = sums[ki * dims + d] / c;
            }
        } else {
            let fallback = (ki * 7 + 3) % n;
            centroids[ki * dims..(ki + 1) * dims]
                .copy_from_slice(&data[fallback * dims..(fallback + 1) * dims]);
        }
    }
}

#[inline(always)]
fn eucl_dist_flat(a: &[f64], b: &[f64]) -> f64 {
    sq_dist_flat(a, b).sqrt()
}

fn nearest_two(pt: &[f64], centroids: &[f64], k: usize, dims: usize) -> (usize, f64, f64) {
    let (mut bc, mut bd1, mut bd2) = (0usize, f64::INFINITY, f64::INFINITY);
    for ki in 0..k {
        let d = sq_dist_flat(pt, &centroids[ki * dims..(ki + 1) * dims]);
        if d < bd1 {
            bd2 = bd1;
            bd1 = d;
            bc = ki;
        } else if d < bd2 {
            bd2 = d;
        }
    }
    (bc, bd1.sqrt(), bd2.sqrt())
}

fn nearest_two_from_curr(
    pt: &[f64],
    centroids: &[f64],
    k: usize,
    dims: usize,
    curr: usize,
    curr_d: f64,
) -> (usize, f64, f64) {
    let curr_sq = curr_d * curr_d;
    let (mut bc, mut bd1, mut bd2) = (curr, curr_sq, f64::INFINITY);
    for ki in 0..k {
        if ki == curr {
            continue;
        }
        let d = sq_dist_flat(pt, &centroids[ki * dims..(ki + 1) * dims]);
        if d < bd1 {
            bd2 = bd1;
            bd1 = d;
            bc = ki;
        } else if d < bd2 {
            bd2 = d;
        }
    }
    (bc, bd1.sqrt(), bd2.sqrt())
}

fn half_nearest_centroid_dists(centroids: &[f64], k: usize, dims: usize) -> Vec<f64> {
    (0..k)
        .map(|j| {
            let cj = &centroids[j * dims..(j + 1) * dims];
            (0..k)
                .filter(|&l| l != j)
                .map(|l| eucl_dist_flat(cj, &centroids[l * dims..(l + 1) * dims]))
                .fold(f64::INFINITY, f64::min)
                * 0.5
        })
        .collect()
}

fn update_centroids_par(
    data: &[f64],
    _n: usize,
    dims: usize,
    k: usize,
    labels: &[i32],
    old: &[f64],
) -> Vec<f64> {
    // Chunked fold: one accumulator per HAMERLY_CHUNK-sized task rather than per element,
    // avoiding thousands of small Vec allocations and reducing Rayon task overhead.
    let (sums, cnts) = labels
        .par_chunks(HAMERLY_CHUNK)
        .zip(data.par_chunks(HAMERLY_CHUNK * dims))
        .fold(
            || (vec![0.0f64; k * dims], vec![0u32; k]),
            |(mut s, mut c), (lab_c, data_c)| {
                for i in 0..lab_c.len() {
                    let ci = lab_c[i] as usize;
                    c[ci] += 1;
                    for d in 0..dims {
                        s[ci * dims + d] += data_c[i * dims + d];
                    }
                }
                (s, c)
            },
        )
        .reduce(
            || (vec![0.0f64; k * dims], vec![0u32; k]),
            |(mut a_s, mut a_c), (b_s, b_c)| {
                for i in 0..k * dims {
                    a_s[i] += b_s[i];
                }
                for j in 0..k {
                    a_c[j] += b_c[j];
                }
                (a_s, a_c)
            },
        );
    let mut out = vec![0.0f64; k * dims];
    for ki in 0..k {
        if cnts[ki] > 0 {
            let c = cnts[ki] as f64;
            for d in 0..dims {
                out[ki * dims + d] = sums[ki * dims + d] / c;
            }
        } else {
            out[ki * dims..(ki + 1) * dims].copy_from_slice(&old[ki * dims..(ki + 1) * dims]);
        }
    }
    out
}

fn update_centroids_seq(
    data: &[f64],
    n: usize,
    dims: usize,
    k: usize,
    labels: &[i32],
    old: &[f64],
) -> Vec<f64> {
    let mut sums = vec![0.0f64; k * dims];
    let mut cnts = vec![0u32; k];
    for i in 0..n {
        let ci = labels[i] as usize;
        cnts[ci] += 1;
        for d in 0..dims {
            sums[ci * dims + d] += data[i * dims + d];
        }
    }
    let mut out = vec![0.0f64; k * dims];
    for ki in 0..k {
        if cnts[ki] > 0 {
            let c = cnts[ki] as f64;
            for d in 0..dims {
                out[ki * dims + d] = sums[ki * dims + d] / c;
            }
        } else {
            out[ki * dims..(ki + 1) * dims].copy_from_slice(&old[ki * dims..(ki + 1) * dims]);
        }
    }
    out
}

const PAR_THRESHOLD: usize = 40_000;
const HAMERLY_CHUNK: usize = 16_384;
const CORESET_THRESHOLD: usize = 50_000;
const K_BLAS: usize = 9999;
const BLAS_CHUNK: usize = 4096;

std::thread_local! {
    static BLAS_BUF: std::cell::RefCell<Vec<f64>> = std::cell::RefCell::new(Vec::new());
}

fn blas_assign_par(
    x: &[f64],
    _n: usize,
    p: usize,
    k: usize,
    centroids: &[f64],
    x_norms_sq: &[f64],
    c_norms: &[f64],
    labels: &mut [i32],
) -> f64 {
    labels
        .par_chunks_mut(BLAS_CHUNK)
        .zip(x_norms_sq.par_chunks(BLAS_CHUNK))
        .enumerate()
        .map(|(ci, (lab_chunk, xn_chunk))| {
            let s = ci * BLAS_CHUNK;
            let cn = lab_chunk.len();
            let x_chunk = &x[s * p..(s + cn) * p];
            BLAS_BUF.with(|cell| {
                let mut buf = cell.borrow_mut();
                let needed = cn * k;
                if buf.len() < needed {
                    buf.resize(needed, 0.0);
                }
                unsafe {
                    matrixmultiply::dgemm(
                        cn,
                        p,
                        k,
                        1.0,
                        x_chunk.as_ptr(),
                        p as isize,
                        1,
                        centroids.as_ptr(),
                        1,
                        p as isize,
                        0.0,
                        buf.as_mut_ptr(),
                        k as isize,
                        1,
                    );
                }
                let mut chunk_inertia = 0.0f64;
                for i in 0..cn {
                    let xn = unsafe { *xn_chunk.get_unchecked(i) };
                    let row = unsafe { buf.get_unchecked(i * k..(i + 1) * k) };
                    let mut best = 0usize;
                    let mut best_d = xn - 2.0 * unsafe { *row.get_unchecked(0) }
                        + unsafe { *c_norms.get_unchecked(0) };
                    for j in 1..k {
                        let d = xn - 2.0 * unsafe { *row.get_unchecked(j) }
                            + unsafe { *c_norms.get_unchecked(j) };
                        if d < best_d {
                            best_d = d;
                            best = j;
                        }
                    }
                    unsafe {
                        *lab_chunk.get_unchecked_mut(i) = best as i32;
                    }
                    chunk_inertia += best_d.max(0.0);
                }
                chunk_inertia
            })
        })
        .sum()
}

fn blas_assign_seq(
    x: &[f64],
    n: usize,
    p: usize,
    k: usize,
    centroids: &[f64],
    x_norms_sq: &[f64],
    c_norms: &[f64],
    labels: &mut [i32],
) -> f64 {
    let n_chunks = (n + BLAS_CHUNK - 1) / BLAS_CHUNK;
    let mut inertia = 0.0f64;
    BLAS_BUF.with(|cell| {
        let mut buf = cell.borrow_mut();
        for ci in 0..n_chunks {
            let s = ci * BLAS_CHUNK;
            let e = (s + BLAS_CHUNK).min(n);
            let cn = e - s;
            let needed = cn * k;
            if buf.len() < needed {
                buf.resize(needed, 0.0);
            }
            let x_chunk = &x[s * p..e * p];
            unsafe {
                matrixmultiply::dgemm(
                    cn,
                    p,
                    k,
                    1.0,
                    x_chunk.as_ptr(),
                    p as isize,
                    1,
                    centroids.as_ptr(),
                    1,
                    p as isize,
                    0.0,
                    buf.as_mut_ptr(),
                    k as isize,
                    1,
                );
            }
            for i in 0..cn {
                let xn = unsafe { *x_norms_sq.get_unchecked(s + i) };
                let row = unsafe { buf.get_unchecked(i * k..(i + 1) * k) };
                let mut best = 0usize;
                let mut best_d = xn - 2.0 * unsafe { *row.get_unchecked(0) }
                    + unsafe { *c_norms.get_unchecked(0) };
                for j in 1..k {
                    let d = xn - 2.0 * unsafe { *row.get_unchecked(j) }
                        + unsafe { *c_norms.get_unchecked(j) };
                    if d < best_d {
                        best_d = d;
                        best = j;
                    }
                }
                unsafe {
                    *labels.get_unchecked_mut(s + i) = best as i32;
                }
                inertia += best_d.max(0.0);
            }
        }
    });
    inertia
}

/// Single-seed Hamerly k-means on the full data. This is the workhorse.
/// When `force_seq` is true, all internal work is sequential (no Rayon).
/// Use force_seq=true when this function is called inside a parallel
/// multi-init loop to avoid nested Rayon contention.
fn kmeans_core_flat_seeded(
    data: &[f64],
    n: usize,
    dims: usize,
    k: usize,
    max_iter: usize,
    seed: u64,
    force_seq: bool,
    tol: f64,
) -> (Vec<i32>, Vec<f64>, f64) {
    let k = k.min(n);
    let par = !force_seq && n >= PAR_THRESHOLD;

    let mut centroids = if par {
        kmeans_pp_flat_par(data, n, dims, k, seed)
    } else {
        kmeans_pp_flat(data, n, dims, k, seed)
    };

    if k >= K_BLAS {
        let x_norms_sq: Vec<f64> = if par {
            data.par_chunks(dims)
                .map(|row| row.iter().map(|&v| v * v).sum::<f64>())
                .collect()
        } else {
            data.chunks(dims)
                .map(|row| row.iter().map(|&v| v * v).sum::<f64>())
                .collect()
        };
        let mut labels = vec![0i32; n];
        let mut c_norms: Vec<f64> = (0..k)
            .map(|j| {
                centroids[j * dims..(j + 1) * dims]
                    .iter()
                    .map(|&v| v * v)
                    .sum::<f64>()
            })
            .collect();
        let mut inertia = if par {
            blas_assign_par(
                data,
                n,
                dims,
                k,
                &centroids,
                &x_norms_sq,
                &c_norms,
                &mut labels,
            )
        } else {
            blas_assign_seq(
                data,
                n,
                dims,
                k,
                &centroids,
                &x_norms_sq,
                &c_norms,
                &mut labels,
            )
        };
        for _ in 0..max_iter {
            let old = centroids.clone();
            centroids = if par {
                update_centroids_par(data, n, dims, k, &labels, &old)
            } else {
                update_centroids_seq(data, n, dims, k, &labels, &old)
            };
            let drift_sq: f64 = (0..k)
                .map(|j| {
                    sq_dist_flat(
                        &centroids[j * dims..(j + 1) * dims],
                        &old[j * dims..(j + 1) * dims],
                    )
                })
                .sum();
            if drift_sq <= tol {
                break;
            }
            for j in 0..k {
                c_norms[j] = centroids[j * dims..(j + 1) * dims]
                    .iter()
                    .map(|&v| v * v)
                    .sum();
            }
            inertia = if par {
                blas_assign_par(
                    data,
                    n,
                    dims,
                    k,
                    &centroids,
                    &x_norms_sq,
                    &c_norms,
                    &mut labels,
                )
            } else {
                blas_assign_seq(
                    data,
                    n,
                    dims,
                    k,
                    &centroids,
                    &x_norms_sq,
                    &c_norms,
                    &mut labels,
                )
            };
        }
        return (labels, centroids, inertia);
    }

    let mut labels = vec![0i32; n];
    let mut upper = vec![0.0f64; n];
    let mut lower = vec![0.0f64; n];

    if par {
        labels
            .par_chunks_mut(HAMERLY_CHUNK)
            .zip(upper.par_chunks_mut(HAMERLY_CHUNK))
            .zip(lower.par_chunks_mut(HAMERLY_CHUNK))
            .zip(data.par_chunks(HAMERLY_CHUNK * dims))
            .for_each(|(((lab_c, u_c), l_c), data_c)| {
                for i in 0..lab_c.len() {
                    let (bc, bd1, bd2) =
                        nearest_two(&data_c[i * dims..(i + 1) * dims], &centroids, k, dims);
                    lab_c[i] = bc as i32;
                    u_c[i] = bd1;
                    l_c[i] = bd2;
                }
            });
    } else {
        for i in 0..n {
            let (bc, bd1, bd2) = nearest_two(&data[i * dims..(i + 1) * dims], &centroids, k, dims);
            labels[i] = bc as i32;
            upper[i] = bd1;
            lower[i] = bd2;
        }
    }

    for _ in 0..max_iter {
        let old = centroids.clone();
        centroids = if par {
            update_centroids_par(data, n, dims, k, &labels, &old)
        } else {
            update_centroids_seq(data, n, dims, k, &labels, &old)
        };

        let drifts: Vec<f64> = (0..k)
            .map(|j| {
                eucl_dist_flat(
                    &old[j * dims..(j + 1) * dims],
                    &centroids[j * dims..(j + 1) * dims],
                )
            })
            .collect();
        let max_drift = drifts.iter().cloned().fold(0.0f64, f64::max);
        if max_drift == 0.0 {
            break;
        }

        // Convergence check: sum of squared centroid drifts (matches sklearn's tol)
        let drift_sq: f64 = drifts.iter().map(|d| d * d).sum();
        if drift_sq <= tol {
            break;
        }

        if par {
            labels
                .par_iter()
                .zip(upper.par_iter_mut())
                .zip(lower.par_iter_mut())
                .for_each(|((&lab, u), l)| {
                    *u += drifts[lab as usize];
                    *l = (*l - max_drift).max(0.0);
                });
        } else {
            for i in 0..n {
                upper[i] += drifts[labels[i] as usize];
                lower[i] = (lower[i] - max_drift).max(0.0);
            }
        }

        let s = half_nearest_centroid_dists(&centroids, k, dims);

        if par {
            labels
                .par_chunks_mut(HAMERLY_CHUNK)
                .zip(upper.par_chunks_mut(HAMERLY_CHUNK))
                .zip(lower.par_chunks_mut(HAMERLY_CHUNK))
                .zip(data.par_chunks(HAMERLY_CHUNK * dims))
                .for_each(|(((lab_c, u_c), l_c), data_c)| {
                    for i in 0..lab_c.len() {
                        let curr = lab_c[i] as usize;
                        let m = s[curr].max(l_c[i]);
                        if u_c[i] <= m {
                            continue;
                        }
                        let pt = &data_c[i * dims..(i + 1) * dims];
                        u_c[i] = eucl_dist_flat(pt, &centroids[curr * dims..(curr + 1) * dims]);
                        if u_c[i] <= m {
                            continue;
                        }
                        let (bc, bd1, bd2) =
                            nearest_two_from_curr(pt, &centroids, k, dims, curr, u_c[i]);
                        lab_c[i] = bc as i32;
                        u_c[i] = bd1;
                        l_c[i] = bd2;
                    }
                });
        } else {
            for i in 0..n {
                let curr = labels[i] as usize;
                let m = s[curr].max(lower[i]);
                if upper[i] <= m {
                    continue;
                }
                let pt = &data[i * dims..(i + 1) * dims];
                upper[i] = eucl_dist_flat(pt, &centroids[curr * dims..(curr + 1) * dims]);
                if upper[i] <= m {
                    continue;
                }
                let (bc, bd1, bd2) = nearest_two_from_curr(pt, &centroids, k, dims, curr, upper[i]);
                labels[i] = bc as i32;
                upper[i] = bd1;
                lower[i] = bd2;
            }
        }
    }

    let inertia: f64 = if par {
        labels
            .par_chunks(HAMERLY_CHUNK)
            .zip(data.par_chunks(HAMERLY_CHUNK * dims))
            .map(|(lab_c, data_c)| {
                lab_c
                    .iter()
                    .enumerate()
                    .map(|(i, &lab)| {
                        sq_dist_flat(
                            &data_c[i * dims..(i + 1) * dims],
                            &centroids[lab as usize * dims..(lab as usize + 1) * dims],
                        )
                    })
                    .sum::<f64>()
            })
            .sum()
    } else {
        (0..n)
            .map(|i| {
                sq_dist_flat(
                    &data[i * dims..(i + 1) * dims],
                    &centroids[labels[i] as usize * dims..(labels[i] as usize + 1) * dims],
                )
            })
            .sum()
    };

    (labels, centroids, inertia)
}

/// Refine given centroids on `data` using Hamerly for at most `max_iter` iterations,
/// then compute final labels + inertia.  Used after coreset selection to polish on full data.
fn kmeans_refine_flat(
    data: &[f64],
    n: usize,
    dims: usize,
    k: usize,
    max_iter: usize,
    centroids_in: &[f64],
    tol: f64,
) -> (Vec<i32>, Vec<f64>, f64) {
    let par = n >= PAR_THRESHOLD;
    let mut centroids = centroids_in.to_vec();

    if k >= K_BLAS {
        let x_norms_sq: Vec<f64> = data
            .par_chunks(dims)
            .map(|row| row.iter().map(|&v| v * v).sum::<f64>())
            .collect();
        let mut labels = vec![0i32; n];
        let mut c_norms: Vec<f64> = (0..k)
            .map(|j| {
                centroids[j * dims..(j + 1) * dims]
                    .iter()
                    .map(|&v| v * v)
                    .sum::<f64>()
            })
            .collect();
        let mut inertia = blas_assign_par(
            data,
            n,
            dims,
            k,
            &centroids,
            &x_norms_sq,
            &c_norms,
            &mut labels,
        );
        for _ in 0..max_iter {
            let old = centroids.clone();
            centroids = update_centroids_par(data, n, dims, k, &labels, &old);
            let drift_sq: f64 = (0..k)
                .map(|j| {
                    sq_dist_flat(
                        &centroids[j * dims..(j + 1) * dims],
                        &old[j * dims..(j + 1) * dims],
                    )
                })
                .sum();
            if drift_sq <= tol {
                break;
            }
            for j in 0..k {
                c_norms[j] = centroids[j * dims..(j + 1) * dims]
                    .iter()
                    .map(|&v| v * v)
                    .sum();
            }
            inertia = blas_assign_par(
                data,
                n,
                dims,
                k,
                &centroids,
                &x_norms_sq,
                &c_norms,
                &mut labels,
            );
        }
        return (labels, centroids, inertia);
    }

    let mut labels = vec![0i32; n];
    let mut upper = vec![0.0f64; n];
    let mut lower = vec![0.0f64; n];

    // Initial full assignment from the given centroids
    labels
        .par_chunks_mut(HAMERLY_CHUNK)
        .zip(upper.par_chunks_mut(HAMERLY_CHUNK))
        .zip(lower.par_chunks_mut(HAMERLY_CHUNK))
        .zip(data.par_chunks(HAMERLY_CHUNK * dims))
        .for_each(|(((lab_c, u_c), l_c), data_c)| {
            for i in 0..lab_c.len() {
                let (bc, bd1, bd2) =
                    nearest_two(&data_c[i * dims..(i + 1) * dims], &centroids, k, dims);
                lab_c[i] = bc as i32;
                u_c[i] = bd1;
                l_c[i] = bd2;
            }
        });

    for _ in 0..max_iter {
        let old = centroids.clone();
        centroids = if par {
            update_centroids_par(data, n, dims, k, &labels, &old)
        } else {
            update_centroids_seq(data, n, dims, k, &labels, &old)
        };

        let drifts: Vec<f64> = (0..k)
            .map(|j| {
                eucl_dist_flat(
                    &old[j * dims..(j + 1) * dims],
                    &centroids[j * dims..(j + 1) * dims],
                )
            })
            .collect();
        let max_drift = drifts.iter().cloned().fold(0.0f64, f64::max);
        if max_drift == 0.0 {
            break;
        }

        // Convergence: sum of squared centroid drifts (same criterion as sklearn)
        let drift_sq: f64 = drifts.iter().map(|d| d * d).sum();
        if drift_sq <= tol {
            break;
        }

        labels
            .par_iter()
            .zip(upper.par_iter_mut())
            .zip(lower.par_iter_mut())
            .for_each(|((&lab, u), l)| {
                *u += drifts[lab as usize];
                *l = (*l - max_drift).max(0.0);
            });

        let s = half_nearest_centroid_dists(&centroids, k, dims);

        labels
            .par_chunks_mut(HAMERLY_CHUNK)
            .zip(upper.par_chunks_mut(HAMERLY_CHUNK))
            .zip(lower.par_chunks_mut(HAMERLY_CHUNK))
            .zip(data.par_chunks(HAMERLY_CHUNK * dims))
            .for_each(|(((lab_c, u_c), l_c), data_c)| {
                for i in 0..lab_c.len() {
                    let curr = lab_c[i] as usize;
                    let m = s[curr].max(l_c[i]);
                    if u_c[i] <= m {
                        continue;
                    }
                    let pt = &data_c[i * dims..(i + 1) * dims];
                    u_c[i] = eucl_dist_flat(pt, &centroids[curr * dims..(curr + 1) * dims]);
                    if u_c[i] <= m {
                        continue;
                    }
                    let (bc, bd1, bd2) =
                        nearest_two_from_curr(pt, &centroids, k, dims, curr, u_c[i]);
                    lab_c[i] = bc as i32;
                    u_c[i] = bd1;
                    l_c[i] = bd2;
                }
            });
    }

    let inertia: f64 = labels
        .par_chunks(HAMERLY_CHUNK)
        .zip(data.par_chunks(HAMERLY_CHUNK * dims))
        .map(|(lab_c, data_c)| {
            lab_c
                .iter()
                .enumerate()
                .map(|(i, &lab)| {
                    sq_dist_flat(
                        &data_c[i * dims..(i + 1) * dims],
                        &centroids[lab as usize * dims..(lab as usize + 1) * dims],
                    )
                })
                .sum::<f64>()
        })
        .sum();

    (labels, centroids, inertia)
}

/// Public entry-point for k-means with n_init support.
///
/// For n_init == 1: delegates to the single-seed function.
///
/// For n_init > 1 and **large n (>= CORESET_THRESHOLD)**:
///   1. Builds a jittered coreset of n/5 points (min 10k).
///   2. Runs n_init Hamerly k-means on the coreset **in parallel** (force_seq per run).
///   3. Picks the init with lowest coreset inertia.
///   4. Refines those centroids on full data until Hamerly convergence.
///
/// For n_init > 1 and small n: runs n_init seeds (force_seq) via par_iter, keeps best.
pub fn kmeans_core_flat(
    data: &[f64],
    n: usize,
    dims: usize,
    k: usize,
    max_iter: usize,
    tol: f64,
) -> (Vec<i32>, Vec<f64>, f64) {
    kmeans_core_flat_ninit(data, n, dims, k, max_iter, tol, 1)
}

pub fn kmeans_core_flat_ninit(
    data: &[f64],
    n: usize,
    dims: usize,
    k: usize,
    max_iter: usize,
    tol: f64,
    n_init: usize,
) -> (Vec<i32>, Vec<f64>, f64) {
    if n == 0 || k == 0 || dims == 0 {
        return (Vec::new(), Vec::new(), 0.0);
    }
    let k = k.min(n);
    let n_init = n_init.max(1);

    let base_seed = data[0]
        .to_bits()
        .wrapping_add(data[(n / 2) * dims].to_bits())
        .wrapping_add(n as u64)
        .wrapping_mul(0xCAFEBABE_u64);

    // Scale tol by mean feature variance — matches sklearn's tolerance criterion:
    // sklearn uses tol_effective = mean(var(X)) * tol_user so that the convergence
    // threshold is data-scale-independent. Without this, our tol=1e-4 is orders of
    // magnitude tighter than sklearn's on high-variance data, causing us to run all
    // max_iter iterations while sklearn converges in 10-20 iterations.
    let mean_var: f64 = {
        let mut s_d = vec![0.0f64; dims];
        let mut sq_d = vec![0.0f64; dims];
        for i in 0..n {
            let row = &data[i * dims..(i + 1) * dims];
            for d in 0..dims {
                s_d[d] += row[d];
                sq_d[d] += row[d] * row[d];
            }
        }
        (0..dims)
            .map(|d| {
                let m = s_d[d] / n as f64;
                sq_d[d] / n as f64 - m * m
            })
            .sum::<f64>()
            / dims as f64
    };
    let tol = tol * mean_var.max(1.0);

    // ── Single init: direct full-data Hamerly (internal parallelism ok) ──
    if n_init == 1 {
        return kmeans_core_flat_seeded(data, n, dims, k, max_iter, base_seed, false, tol);
    }

    // ── Multi-init with coreset acceleration for large n ──
    if n >= CORESET_THRESHOLD {
        // 1. Build jittered coreset — dynamic size scaled to n
        let cs_n = if k >= 100 {
            (n / 20).max(5_000)
        } else {
            (n / 5).max(10_000)
        }
        .min(n);
        let stride = n / cs_n;
        let mut rng_cs = base_seed.wrapping_mul(0xDEAD1234_u64);
        let mut coreset = Vec::with_capacity(cs_n * dims);
        for i in 0..cs_n {
            let offset = (splitmix64(&mut rng_cs) as usize) % stride;
            let idx = i * stride + offset;
            coreset.extend_from_slice(&data[idx * dims..(idx + 1) * dims]);
        }

        // 2. Run n_init k-means on coreset in parallel — force_seq=true avoids
        //    nested Rayon contention (outer par_iter provides all parallelism).
        //    For k<100 the coreset is large (n/5) but converges in <30 iters with k++ init;
        //    cap at 50 to avoid wasting time. For k>=100 the coreset is small (n/20) and
        //    may need more iterations to give good init for the full refine.
        let cs_max_iter = if k < 100 { max_iter.min(50) } else { max_iter };
        let best_centroids: Vec<f64> = (0..n_init)
            .into_par_iter()
            .map(|idx| {
                let seed = base_seed.wrapping_add(idx as u64 * 0x9E3779B97F4A7C15);
                kmeans_core_flat_seeded(&coreset, cs_n, dims, k, cs_max_iter, seed, true, tol)
            })
            .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap()
            .1;

        // 3. Refine on full data with Hamerly until convergence
        return kmeans_refine_flat(data, n, dims, k, max_iter, &best_centroids, tol);
    }

    // ── Multi-init on full data for smaller n ──
    // force_seq=true — each init runs sequentially, outer par_iter parallelises
    (0..n_init)
        .into_par_iter()
        .map(|idx| {
            let seed = base_seed.wrapping_add(idx as u64 * 0x9E3779B97F4A7C15);
            kmeans_core_flat_seeded(data, n, dims, k, max_iter, seed, true, tol)
        })
        .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap()
}

pub fn minibatch_kmeans_core_flat(
    data: &[f64],
    n: usize,
    dims: usize,
    k: usize,
    max_iter: usize,
    batch_size: usize,
) -> (Vec<i32>, Vec<f64>, f64) {
    if n == 0 || k == 0 || dims == 0 {
        return (Vec::new(), Vec::new(), 0.0);
    }
    let k = k.min(n);
    let batch = batch_size.min(n);
    // Use subsample init for large n — the previous O(n*k) sequential scan was the
    // dominant cost (e.g. ~150ms at n=1.79M k=15), making mini-batch SLOWER than sklearn.
    let seed = data[0]
        .to_bits()
        .wrapping_add(n as u64)
        .wrapping_mul(0xCAFE1234_u64);
    let mut centroids = if n >= CORESET_THRESHOLD {
        let sample_n = 50_000usize.min(n);
        let stride = n / sample_n;
        let mut sample = Vec::with_capacity(sample_n * dims);
        for i in 0..sample_n {
            sample.extend_from_slice(&data[i * stride * dims..(i * stride + 1) * dims]);
        }
        kmeans_pp_flat(&sample, sample_n, dims, k, seed)
    } else {
        kmeans_pp_flat(data, n, dims, k, seed)
    };
    let mut counts = vec![1u32; k];
    let mut rng = seed;
    for _ in 0..max_iter {
        let s = (splitmix64(&mut rng) as usize) % (n - batch + 1).max(1);
        for i in s..(s + batch).min(n) {
            let pt = &data[i * dims..(i + 1) * dims];
            let (mut bc, mut bd) = (0usize, f64::INFINITY);
            for ki in 0..k {
                let d = sq_dist_flat(pt, &centroids[ki * dims..(ki + 1) * dims]);
                if d < bd {
                    bd = d;
                    bc = ki;
                }
            }
            counts[bc] += 1;
            let lr = 1.0 / counts[bc] as f64;
            for d in 0..dims {
                centroids[bc * dims + d] += lr * (pt[d] - centroids[bc * dims + d]);
            }
        }
    }
    // Single parallel pass: assign labels AND accumulate inertia in one sweep.
    // Previously two passes (separate label assign + separate inertia with par_iter/par_chunks(dims)).
    // Chunked approach avoids per-element Rayon task overhead and halves memory traffic.
    let mut labels = vec![0i32; n];
    let inertia: f64 = labels
        .par_chunks_mut(HAMERLY_CHUNK)
        .zip(data.par_chunks(HAMERLY_CHUNK * dims))
        .map(|(lab_c, data_c)| {
            let mut local = 0.0f64;
            for i in 0..lab_c.len() {
                let pt = &data_c[i * dims..(i + 1) * dims];
                let (mut bc, mut bd) = (0usize, f64::INFINITY);
                for ki in 0..k {
                    let d = sq_dist_flat(pt, &centroids[ki * dims..(ki + 1) * dims]);
                    if d < bd {
                        bd = d;
                        bc = ki;
                    }
                }
                lab_c[i] = bc as i32;
                local += bd;
            }
            local
        })
        .sum();
    (labels, centroids, inertia)
}

pub fn kmeans_core_nd(
    data: &[Vec<f64>],
    k: usize,
    max_iter: usize,
    tol: f64,
) -> (Vec<i32>, Vec<Vec<f64>>, f64) {
    let n = data.len();
    if n == 0 || k == 0 {
        return (Vec::new(), Vec::new(), 0.0);
    }
    let dims = data[0].len();
    let mut flat = vec![0.0f64; n * dims];
    for (i, row) in data.iter().enumerate() {
        flat[i * dims..(i + 1) * dims].copy_from_slice(row);
    }
    let (labels, flat_c, inertia) = kmeans_core_flat(&flat, n, dims, k, max_iter, tol);
    let centroids: Vec<Vec<f64>> = (0..k)
        .map(|ki| flat_c[ki * dims..(ki + 1) * dims].to_vec())
        .collect();
    (labels, centroids, inertia)
}

pub fn minibatch_kmeans_core_nd(
    data: &[Vec<f64>],
    k: usize,
    max_iter: usize,
    batch_size: usize,
) -> (Vec<i32>, Vec<Vec<f64>>, f64) {
    let n = data.len();
    if n == 0 || k == 0 {
        return (Vec::new(), Vec::new(), 0.0);
    }
    let dims = data[0].len();
    let mut flat = vec![0.0f64; n * dims];
    for (i, row) in data.iter().enumerate() {
        flat[i * dims..(i + 1) * dims].copy_from_slice(row);
    }
    let (labels, flat_c, inertia) =
        minibatch_kmeans_core_flat(&flat, n, dims, k, max_iter, batch_size);
    let centroids: Vec<Vec<f64>> = (0..k)
        .map(|ki| flat_c[ki * dims..(ki + 1) * dims].to_vec())
        .collect();
    (labels, centroids, inertia)
}
pub fn kmeans_core_2d(
    x: &[f64],
    y: &[f64],
    k: usize,
    max_iter: usize,
    tol: f64,
) -> (Vec<i32>, Vec<f64>, Vec<f64>, f64) {
    let n = x.len().min(y.len());
    if n == 0 || k == 0 {
        return (Vec::new(), Vec::new(), Vec::new(), 0.0);
    }
    let mut flat = vec![0.0f64; n * 2];
    for i in 0..n {
        flat[i * 2] = x[i];
        flat[i * 2 + 1] = y[i];
    }
    let (labels, flat_c, inertia) = kmeans_core_flat(&flat, n, 2, k, max_iter, tol);
    let cx: Vec<f64> = (0..flat_c.len() / 2).map(|ki| flat_c[ki * 2]).collect();
    let cy: Vec<f64> = (0..flat_c.len() / 2).map(|ki| flat_c[ki * 2 + 1]).collect();
    (labels, cx, cy, inertia)
}

pub fn minibatch_kmeans_core_2d(
    x: &[f64],
    y: &[f64],
    k: usize,
    max_iter: usize,
    batch_size: usize,
) -> (Vec<i32>, Vec<f64>, Vec<f64>, f64) {
    let n = x.len().min(y.len());
    if n == 0 || k == 0 {
        return (Vec::new(), Vec::new(), Vec::new(), 0.0);
    }
    let mut flat = vec![0.0f64; n * 2];
    for i in 0..n {
        flat[i * 2] = x[i];
        flat[i * 2 + 1] = y[i];
    }
    let (labels, flat_c, inertia) =
        minibatch_kmeans_core_flat(&flat, n, 2, k, max_iter, batch_size);
    let cx: Vec<f64> = (0..flat_c.len() / 2).map(|ki| flat_c[ki * 2]).collect();
    let cy: Vec<f64> = (0..flat_c.len() / 2).map(|ki| flat_c[ki * 2 + 1]).collect();
    (labels, cx, cy, inertia)
}

fn push_js_str(buf: &mut Vec<u8>, s: &str) {
    for b in s.bytes() {
        match b {
            b'\'' => {
                buf.push(b'\\');
                buf.push(b'\'');
            }
            b'\\' => {
                buf.push(b'\\');
                buf.push(b'\\');
            }
            b'\n' => {
                buf.push(b'\\');
                buf.push(b'n');
            }
            _ => buf.push(b),
        }
    }
}

fn b64enc(data: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(data)
}

pub fn render_kmeans_html(cfg: &KMeansConfig) -> String {
    let n = cfg.x_values.len().min(cfg.y_values.len());
    if n == 0 || cfg.k == 0 {
        return String::new();
    }

    let (labels, cx, cy, inertia) = if cfg.mini_batch || n > 100_000 {
        minibatch_kmeans_core_2d(
            cfg.x_values,
            cfg.y_values,
            cfg.k,
            cfg.max_iter,
            cfg.batch_size,
        )
    } else {
        kmeans_core_2d(cfg.x_values, cfg.y_values, cfg.k, cfg.max_iter, cfg.tol)
    };
    let k = cx.len();

    let (min_x, max_x) = find_minmax(cfg.x_values);
    let (min_y, max_y) = find_minmax(cfg.y_values);
    let (rx, ry) = ((max_x - min_x).max(1e-12), (max_y - min_y).max(1e-12));
    let (pad_l, pad_t, pad_b, leg_w) = (56i32, 36i32, 48i32, 170i32);
    let (pw, ph) = (cfg.width - pad_l - 20 - leg_w, cfg.height - pad_t - pad_b);
    let (irx, iry) = (pw as f64 / rx, ph as f64 / ry);

    let colors: Vec<String> = (0..k)
        .map(|i| {
            let hx = hex6(palette_color(cfg.palette, i));
            format!(
                "#{}{}{}{}{}{}",
                hx[0] as char,
                hx[1] as char,
                hx[2] as char,
                hx[3] as char,
                hx[4] as char,
                hx[5] as char
            )
        })
        .collect();

    let dedup = n > 50_000;
    let mut seen: HashSet<u64> = if dedup {
        HashSet::with_capacity((pw * ph) as usize)
    } else {
        HashSet::new()
    };
    let mut raw: Vec<Vec<(i16, i16)>> = vec![Vec::new(); k];
    for i in 0..n {
        let gi = (labels[i] as usize).min(k.saturating_sub(1));
        let px = ((cfg.x_values[i] - min_x) * irx).clamp(0.0, (pw - 1) as f64) as i16;
        let py = (ph as f64 - (cfg.y_values[i] - min_y) * iry).clamp(0.0, (ph - 1) as f64) as i16;
        if dedup {
            let key = ((gi as u64) << 40) | ((px as u64 & 0xFFFFF) << 20) | (py as u64 & 0xFFFFF);
            if !seen.insert(key) {
                continue;
            }
        }
        raw[gi].push((px, py));
    }
    let gd: Vec<String> = raw
        .iter()
        .map(|pts| {
            let mut v = Vec::with_capacity(pts.len() * 4);
            for &(px, py) in pts {
                v.extend_from_slice(&px.to_le_bytes());
                v.extend_from_slice(&py.to_le_bytes());
            }
            b64enc(&v)
        })
        .collect();
    let cpx: Vec<i32> = cx
        .iter()
        .map(|&v| pad_l + ((v - min_x) * irx).clamp(0.0, (pw - 1) as f64) as i32)
        .collect();
    let cpy: Vec<i32> = cy
        .iter()
        .map(|&v| pad_t + (ph as f64 - (v - min_y) * iry).clamp(0.0, (ph - 1) as f64) as i32)
        .collect();

    let hid = html_id();
    let cv = format!("spcv{hid}");
    let tip = format!("sptip{hid}");
    let mut buf = Vec::<u8>::with_capacity(n / 4 + 14_000);
    let lg = pad_l + pw + 12;
    let lt = pad_t + 8;

    html_prefix(&mut buf, cfg.title, hid);
    push_b(
        &mut buf,
        b"<div style=\"position:relative;display:inline-block\">",
    );
    push_b(&mut buf, b"<canvas id=\"");
    buf.extend_from_slice(cv.as_bytes());
    push_b(&mut buf, b"\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\" style=\"display:block\"></canvas><div id=\"");
    buf.extend_from_slice(tip.as_bytes());
    push_b(&mut buf, b"\" style=\"position:absolute;pointer-events:none;opacity:0;transition:opacity .15s;background:#0b0e18;color:#f1f5f9;font:12px -apple-system,Arial,sans-serif;border-radius:8px;padding:6px 10px;white-space:nowrap;box-shadow:0 4px 16px rgba(0,0,0,.4);z-index:10\"></div></div><script>(function(){");
    push_b(&mut buf, b"var cv=document.getElementById('");
    buf.extend_from_slice(cv.as_bytes());
    push_b(
        &mut buf,
        b"'),ctx=cv.getContext('2d'),tip=document.getElementById('",
    );
    buf.extend_from_slice(tip.as_bytes());
    push_b(&mut buf, b"');");
    push_b(&mut buf, b"var pL=");
    push_i(&mut buf, pad_l);
    push_b(&mut buf, b",pT=");
    push_i(&mut buf, pad_t);
    push_b(&mut buf, b",pW=");
    push_i(&mut buf, pw);
    push_b(&mut buf, b",pH=");
    push_i(&mut buf, ph);
    push_b(&mut buf, b",W=");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b",H=");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b",minX=");
    push_f2(&mut buf, min_x);
    push_b(&mut buf, b",minY=");
    push_f2(&mut buf, min_y);
    push_b(&mut buf, b",rX=");
    push_f2(&mut buf, rx);
    push_b(&mut buf, b",rY=");
    push_f2(&mut buf, ry);
    push_b(&mut buf, b",GL=");
    buf.extend_from_slice(if cfg.gridlines { b"1" } else { b"0" });
    push_b(&mut buf, b";");
    push_b(&mut buf, b"var hidden={};");
    push_b(&mut buf, b"function b64(s){var b=atob(s),n=b.length,a=new Int16Array(n/2);for(var i=0;i<n;i+=2)a[i/2]=b.charCodeAt(i)|(b.charCodeAt(i+1)<<8);return a;}");
    push_b(&mut buf, b"var GD=[");
    for (i, d) in gd.iter().enumerate() {
        if i > 0 {
            buf.push(b',');
        }
        buf.push(b'\'');
        buf.extend_from_slice(d.as_bytes());
        buf.push(b'\'');
    }
    push_b(&mut buf, b"],GC=[");
    for (i, c) in colors.iter().enumerate() {
        if i > 0 {
            buf.push(b',');
        }
        buf.push(b'\'');
        push_js_str(&mut buf, c);
        buf.push(b'\'');
    }
    push_b(&mut buf, b"],GN=[");
    for i in 0..k {
        if i > 0 {
            buf.push(b',');
        }
        buf.push(b'\'');
        push_js_str(&mut buf, &format!("Cluster {}", i + 1));
        buf.push(b'\'');
    }
    push_b(&mut buf, b"],CPX=[");
    for (i, &px) in cpx.iter().enumerate() {
        if i > 0 {
            buf.push(b',');
        }
        push_i(&mut buf, px);
    }
    push_b(&mut buf, b"],CPY=[");
    for (i, &py) in cpy.iter().enumerate() {
        if i > 0 {
            buf.push(b',');
        }
        push_i(&mut buf, py);
    }
    push_b(&mut buf, b"];");
    push_b(
        &mut buf,
        b"function draw(){ctx.fillStyle='#fff';ctx.fillRect(0,0,W,H);",
    );
    push_b(&mut buf, b"if(GL){ctx.strokeStyle='#e2e8f0';ctx.lineWidth=0.5;for(var i=1;i<=5;i++){var gy=pT+Math.round((1-i/5)*pH);ctx.beginPath();ctx.moveTo(pL,gy);ctx.lineTo(pL+pW,gy);ctx.stroke();}}");
    push_b(&mut buf, b"ctx.strokeStyle='#cbd5e1';ctx.lineWidth=1;ctx.beginPath();ctx.moveTo(pL,pT);ctx.lineTo(pL,pT+pH);ctx.stroke();ctx.beginPath();ctx.moveTo(pL,pT+pH);ctx.lineTo(pL+pW,pT+pH);ctx.stroke();");
    push_b(&mut buf, b"ctx.fillStyle='#6b7280';ctx.font='9px Arial';ctx.textAlign='end';for(var i=0;i<=5;i++){var f=i/5,yp=pT+Math.round((1-f)*pH),yv=minY+f*rY;ctx.fillText(yv>=1000?Math.round(yv)+'':yv.toFixed(2),pL-4,yp+3);}");
    push_b(&mut buf, b"ctx.textAlign='center';for(var i=0;i<=5;i++){var f=i/5,xp=pL+Math.round(f*pW),xv=minX+f*rX;ctx.fillText(xv>=1000?Math.round(xv)+'':xv.toFixed(2),xp,pT+pH+14);}");
    if !cfg.y_label.is_empty() {
        push_b(&mut buf, b"ctx.save();ctx.translate(14,pT+pH/2);ctx.rotate(-Math.PI/2);ctx.font='11px Arial';ctx.fillStyle='#374151';ctx.textAlign='center';ctx.fillText('");
        push_js_str(&mut buf, cfg.y_label);
        push_b(&mut buf, b"',0,0);ctx.restore();");
    }
    if !cfg.x_label.is_empty() {
        push_b(
            &mut buf,
            b"ctx.font='11px Arial';ctx.fillStyle='#374151';ctx.textAlign='center';ctx.fillText('",
        );
        push_js_str(&mut buf, cfg.x_label);
        push_b(&mut buf, b"',pL+pW/2,H-4);");
    }
    if !cfg.title.is_empty() {
        push_b(&mut buf, b"ctx.font='700 14px -apple-system,Arial,sans-serif';ctx.fillStyle='#1a202c';ctx.textAlign='center';ctx.fillText('");
        push_js_str(&mut buf, cfg.title);
        push_b(&mut buf, b"',W/2,22);");
    }
    push_b(&mut buf, b"for(var gi=0;gi<GD.length;gi++){if(hidden[gi])continue;var a=b64(GD[gi]);ctx.fillStyle=GC[gi]||GC[0];for(var i=0;i<a.length;i+=2)ctx.fillRect(pL+a[i],pT+a[i+1],2,2);}");
    push_b(&mut buf, b"for(var ci=0;ci<CPX.length;ci++){if(hidden[ci])continue;var px=CPX[ci],py=CPY[ci],cl=GC[ci]||'#333',s=9;ctx.strokeStyle='#fff';ctx.lineWidth=3.5;ctx.beginPath();ctx.moveTo(px-s,py);ctx.lineTo(px+s,py);ctx.moveTo(px,py-s);ctx.lineTo(px,py+s);ctx.stroke();ctx.strokeStyle=cl;ctx.lineWidth=2;ctx.beginPath();ctx.moveTo(px-s,py);ctx.lineTo(px+s,py);ctx.moveTo(px,py-s);ctx.lineTo(px,py+s);ctx.stroke();}");
    push_b(&mut buf, b"ctx.textAlign='left';ctx.font='11px Arial';");
    for gi in 0..k {
        let ly = lt + gi as i32 * 22;
        push_b(&mut buf, b"if(!hidden[");
        push_i(&mut buf, gi as i32);
        push_b(
            &mut buf,
            b"]){ctx.globalAlpha=1;}else{ctx.globalAlpha=0.28;}",
        );
        push_b(&mut buf, b"ctx.fillStyle='");
        push_js_str(&mut buf, &colors[gi]);
        push_b(&mut buf, b"';ctx.beginPath();ctx.arc(");
        push_i(&mut buf, lg + 6);
        buf.push(b',');
        push_i(&mut buf, ly + 6);
        push_b(&mut buf, b",6,0,2*Math.PI);ctx.fill();ctx.globalAlpha=1;ctx.fillStyle='#374151';ctx.fillText(GN[");
        push_i(&mut buf, gi as i32);
        push_b(&mut buf, b"]||'',");
        push_i(&mut buf, lg + 17);
        buf.push(b',');
        push_i(&mut buf, ly + 11);
        push_b(&mut buf, b");");
    }
    push_b(&mut buf, b"ctx.globalAlpha=1;ctx.fillStyle='#6b7280';ctx.font='10px Arial';ctx.textAlign='left';ctx.fillText('");
    push_js_str(&mut buf, &format!("inertia: {inertia:.0}"));
    push_b(&mut buf, b"',pL+4,pT+pH-8);}draw();");
    push_b(&mut buf, b"cv.addEventListener('click',function(e){var r=cv.getBoundingClientRect(),mx=e.clientX-r.left,my=e.clientY-r.top;");
    push_b(&mut buf, b"for(var gi=0;gi<");
    push_i(&mut buf, k as i32);
    push_b(&mut buf, b";gi++){var ly=");
    push_i(&mut buf, lt);
    push_b(&mut buf, b"+gi*22;if(mx>=");
    push_i(&mut buf, lg);
    push_b(&mut buf, b"&&mx<=");
    push_i(&mut buf, lg + 150);
    push_b(
        &mut buf,
        b"&&my>=ly&&my<=ly+18){hidden[gi]=!hidden[gi];draw();return;}}});",
    );
    push_b(&mut buf, b"var allPts=[];for(var gi=0;gi<GD.length;gi++){var a=b64(GD[gi]);for(var i=0;i<a.length;i+=2)allPts.push([gi,pL+a[i],pT+a[i+1]]);}");
    push_b(&mut buf, b"cv.addEventListener('mousemove',function(e){var r=cv.getBoundingClientRect(),mx=e.clientX-r.left,my=e.clientY-r.top;if(mx<pL||mx>pL+pW||my<pT||my>pT+pH){tip.style.opacity=0;return;}var best=null,bd=1e9;for(var i=0;i<allPts.length;i++){var p=allPts[i];if(hidden[p[0]])continue;var dx=p[1]-mx,dy=p[2]-my,d=dx*dx+dy*dy;if(d<bd){bd=d;best=p;}}if(!best||bd>400){tip.style.opacity=0;return;}var xv=(best[1]-pL)/pW*rX+minX,yv=(1-(best[2]-pT)/pH)*rY+minY;var gn=GN[best[0]]?'<br><span style=\"color:#94a3b8\">'+GN[best[0]]+'</span>':'';tip.innerHTML='<b>x:</b> '+xv.toFixed(2)+'&nbsp;&nbsp;<b>y:</b> '+yv.toFixed(2)+gn;var tx=best[1]+12,ty=best[2]-28;if(tx+160>W)tx=best[1]-170;if(ty<0)ty=best[2]+8;tip.style.left=tx+'px';tip.style.top=ty+'px';tip.style.opacity=1;});");
    push_b(
        &mut buf,
        b"cv.addEventListener('mouseleave',function(){tip.style.opacity=0;});})();</script>",
    );
    html_suffix(&mut buf, hid, "[]");
    unsafe { String::from_utf8_unchecked(buf) }
}

#[crate::sera_alias("kmeans", "kmeans_chart")]
#[crate::sera_builder]
pub fn build_kmeans_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x = a.x.unwrap_or_default();
    let y = a.y.unwrap_or_default();
    let k = o.k.unwrap_or(3);
    let max_iter = o.max_iter.unwrap_or(300);
    let pal = o.pal();
    let bg_str = o.bg_str();
    let html = crate::plot::default::render_kmeans_html(&crate::plot::default::KMeansConfig {
        title,
        x_values: &x,
        y_values: &y,
        k,
        max_iter,
        tol: 1e-4,
        mini_batch: false,
        batch_size: 1000,
        width: o.w(1000),
        height: o.h(580),
        x_label: &o.xl(),
        y_label: &o.yl(),
        gridlines: o.grid(),
        palette: &pal,
        ..Default::default()
    });
    let h = crate::html::hover::apply_opts(html, bg_str.as_deref(), !o.no_x(), !o.no_y());
    crate::apply_global_color_bindings(h)
}

#[cfg(test)]
mod kmeans_pp_degenerate_tests {
    use super::*;

    #[test]
    fn kmeans_pp_flat_returns_k_centroids_even_when_all_points_are_identical() {
        let data = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let k = 3;
        let dims = 2;
        let n = 5;
        let centroids = kmeans_pp_flat(&data, n, dims, k, 42);
        assert_eq!(centroids.len(), k * dims);
    }

    #[test]
    fn kmeans_pp_flat_par_returns_k_centroids_even_when_all_points_are_identical() {
        let data = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let k = 3;
        let dims = 2;
        let n = 5;
        let centroids = kmeans_pp_flat_par(&data, n, dims, k, 42);
        assert_eq!(centroids.len(), k * dims);
    }

    #[test]
    fn build_kmeans_chart_does_not_panic_on_identical_points() {
        let html = build_kmeans_chart(
            r#"{"title":"t","x":[0.0,0.0,0.0,0.0,0.0],"y":[0.0,0.0,0.0,0.0,0.0],"k":2}"#,
        );
        assert!(!html.is_empty());
    }
}
