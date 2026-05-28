use rayon::prelude::*;
use crate::core::hw_profile::hw;

#[inline(always)]
pub fn splitmix64(state: u64) -> u64 {
    let mut z = state.wrapping_add(0x9E3779B97F4A7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

#[inline(always)]
pub fn dot(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len().min(b.len());
    let (mut s0, mut s1, mut s2, mut s3) = (0.0, 0.0, 0.0, 0.0);
    let (mut s4, mut s5, mut s6, mut s7) = (0.0, 0.0, 0.0, 0.0);
    let mut i = 0;
    while i + 8 <= n {
        s0 += a[i] * b[i];
        s1 += a[i + 1] * b[i + 1];
        s2 += a[i + 2] * b[i + 2];
        s3 += a[i + 3] * b[i + 3];
        s4 += a[i + 4] * b[i + 4];
        s5 += a[i + 5] * b[i + 5];
        s6 += a[i + 6] * b[i + 6];
        s7 += a[i + 7] * b[i + 7];
        i += 8;
    }
    while i < n { s0 += a[i] * b[i]; i += 1; }
    (s0 + s1 + s2 + s3) + (s4 + s5 + s6 + s7)
}

pub fn mat_vec(a: &[f64], rows: usize, cols: usize, x: &[f64], out: &mut [f64]) {
    if rows == 0 || cols == 0 { for v in out.iter_mut() { *v = 0.0; } return; }
    if rows >= hw().par_threshold {
        out.par_chunks_mut(2048).enumerate().for_each(|(ci, chunk)| {
            let r0 = ci * 2048;
            for (k, v) in chunk.iter_mut().enumerate() {
                let i = r0 + k;
                let row = unsafe { a.get_unchecked(i * cols..(i + 1) * cols) };
                *v = dot(row, x);
            }
        });
        return;
    }
    for i in 0..rows {
        out[i] = dot(unsafe { a.get_unchecked(i * cols..(i + 1) * cols) }, x);
    }
}

pub fn mat_t_vec(a: &[f64], rows: usize, cols: usize, x: &[f64], out: &mut [f64]) {
    if rows == 0 || cols == 0 { for v in out.iter_mut() { *v = 0.0; } return; }
    if rows * cols < 4096 {
        for j in 0..cols { out[j] = 0.0; }
        for i in 0..rows {
            let xi = x[i];
            let row = &a[i * cols..(i + 1) * cols];
            for j in 0..cols { out[j] += row[j] * xi; }
        }
        return;
    }
    unsafe {
        matrixmultiply::dgemm(
            cols, rows, 1,
            1.0,
            a.as_ptr(), 1, cols as isize,
            x.as_ptr(), 1, 1,
            0.0,
            out.as_mut_ptr(), 1, 1,
        );
    }
}

pub fn mat_t_mat(a: &[f64], n: usize, p: usize, out: &mut [f64]) {
    for v in out[..p * p].iter_mut() { *v = 0.0; }
    if n == 0 || p == 0 { return; }
    if p <= 64 || n * p * p < hw().par_threshold {
        if n >= hw().par_threshold / 32 {
            let chunk = 2048usize.min(n);
            let nc = (n + chunk - 1) / chunk;
            let partials: Vec<Vec<f64>> = (0..nc).into_par_iter().map(|c| {
                let s = c * chunk;
                let e = (s + chunk).min(n);
                let mut part = vec![0.0; p * p];
                for r in s..e {
                    let row = &a[r * p..r * p + p];
                    for i in 0..p {
                        let ai = row[i];
                        for j in i..p { part[i * p + j] += ai * row[j]; }
                    }
                }
                part
            }).collect();
            for part in &partials {
                for i in 0..p { for j in i..p { out[i * p + j] += part[i * p + j]; } }
            }
        } else {
            for r in 0..n {
                let row = &a[r * p..r * p + p];
                for i in 0..p {
                    let ai = row[i];
                    for j in i..p { out[i * p + j] += ai * row[j]; }
                }
            }
        }
        for i in 0..p { for j in (i + 1)..p { out[j * p + i] = out[i * p + j]; } }
        return;
    }
    unsafe {
        matrixmultiply::dgemm(
            p, n, p,
            1.0,
            a.as_ptr(), 1, p as isize,
            a.as_ptr(), p as isize, 1,
            0.0,
            out.as_mut_ptr(), p as isize, 1,
        );
    }
}

pub fn mat_t_y(a: &[f64], n: usize, p: usize, y: &[f64], out: &mut [f64]) {
    for j in 0..p { out[j] = 0.0; }
    if n == 0 || p == 0 { return; }
    if n * p < 4096 {
        for i in 0..n {
            let yi = y[i];
            let row = &a[i * p..(i + 1) * p];
            for j in 0..p { out[j] += row[j] * yi; }
        }
        return;
    }
    unsafe {
        matrixmultiply::dgemm(
            p, n, 1,
            1.0,
            a.as_ptr(), 1, p as isize,
            y.as_ptr(), 1, 1,
            0.0,
            out.as_mut_ptr(), 1, 1,
        );
    }
}

pub fn cholesky(a: &[f64], n: usize) -> Option<Vec<f64>> {
    let mut l = vec![0.0; n * n];
    for j in 0..n {
        let mut s = 0.0;
        for k in 0..j { s += l[j * n + k] * l[j * n + k]; }
        let diag = a[j * n + j] - s;
        if diag <= 0.0 { return None; }
        l[j * n + j] = diag.sqrt();
        for i in (j + 1)..n {
            let mut s2 = 0.0;
            for k in 0..j { s2 += l[i * n + k] * l[j * n + k]; }
            l[i * n + j] = (a[i * n + j] - s2) / l[j * n + j];
        }
    }
    Some(l)
}

pub fn cholesky_solve(l: &[f64], n: usize, b: &[f64], x: &mut [f64]) {
    let mut y = vec![0.0; n];
    for i in 0..n {
        let mut s = b[i];
        for k in 0..i { s -= l[i * n + k] * y[k]; }
        y[i] = s / l[i * n + i];
    }
    for i in (0..n).rev() {
        let mut s = y[i];
        for k in (i + 1)..n { s -= l[k * n + i] * x[k]; }
        x[i] = s / l[i * n + i];
    }
}

pub fn solve_spd(ata: &[f64], p: usize, atb: &[f64]) -> Option<Vec<f64>> {
    let l = cholesky(ata, p)?;
    let mut x = vec![0.0; p];
    cholesky_solve(&l, p, atb, &mut x);
    Some(x)
}

pub fn qr_economy(a: &[f64], n: usize, p: usize) -> (Vec<f64>, Vec<f64>) {
    let mut q = a.to_vec();
    let mut r = vec![0.0; p * p];
    for j in 0..p {
        for i in 0..j {
            let mut d = 0.0;
            for row in 0..n { d += q[row * p + i] * q[row * p + j]; }
            r[i * p + j] = d;
            for row in 0..n { q[row * p + j] -= d * q[row * p + i]; }
        }
        let mut norm = 0.0;
        for row in 0..n { norm += q[row * p + j] * q[row * p + j]; }
        norm = norm.sqrt();
        r[j * p + j] = norm;
        if norm > 1e-15 {
            let inv = 1.0 / norm;
            for row in 0..n { q[row * p + j] *= inv; }
        }
    }
    (q, r)
}

pub fn back_sub(r: &[f64], p: usize, b: &[f64], x: &mut [f64]) {
    for i in (0..p).rev() {
        let mut s = b[i];
        for j in (i + 1)..p { s -= r[i * p + j] * x[j]; }
        x[i] = if r[i * p + i].abs() > 1e-15 { s / r[i * p + i] } else { 0.0 };
    }
}

pub fn qr_solve(a: &[f64], n: usize, p: usize, y: &[f64]) -> Vec<f64> {
    let (q, r) = qr_economy(a, n, p);
    let mut qtb = vec![0.0; p];
    for j in 0..p {
        let mut s = 0.0;
        for i in 0..n { s += q[i * p + j] * y[i]; }
        qtb[j] = s;
    }
    let mut x = vec![0.0; p];
    back_sub(&r, p, &qtb, &mut x);
    x
}

pub fn svd_truncated(a: &[f64], n: usize, p: usize, k: usize, max_iter: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let kk = k.min(n).min(p);
    let mut u = vec![0.0; n * kk];
    let mut s = vec![0.0; kk];
    let mut vt = vec![0.0; kk * p];
    let mut rng = 0x123456789ABCDEFu64;
    for comp in 0..kk {
        let mut v = vec![0.0; p];
        for j in 0..p {
            rng ^= rng << 13; rng ^= rng >> 7; rng ^= rng << 17;
            v[j] = (rng as f64) / (u64::MAX as f64) - 0.5;
        }
        for _ in 0..max_iter {
            let mut u_vec = vec![0.0; n];
            mat_vec(a, n, p, &v, &mut u_vec);
            for prev in 0..comp {
                let mut d = 0.0;
                for i in 0..n { d += u_vec[i] * u[i * kk + prev]; }
                for i in 0..n { u_vec[i] -= d * u[i * kk + prev]; }
            }
            let norm_u = u_vec.iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm_u > 1e-15 {
                let inv = 1.0 / norm_u;
                for x in &mut u_vec { *x *= inv; }
            }
            let mut v_new = vec![0.0; p];
            mat_t_vec(a, n, p, &u_vec, &mut v_new);
            for prev in 0..comp {
                let mut d = 0.0;
                for j in 0..p { d += v_new[j] * vt[prev * p + j]; }
                for j in 0..p { v_new[j] -= d * vt[prev * p + j]; }
            }
            let sigma = v_new.iter().map(|x| x * x).sum::<f64>().sqrt();
            if sigma > 1e-15 {
                let inv = 1.0 / sigma;
                for x in &mut v_new { *x *= inv; }
            }
            let diff: f64 = v.iter().zip(v_new.iter()).map(|(a, b)| (a - b).abs()).sum();
            v = v_new;
            if diff < 1e-10 { break; }
        }
        let mut u_vec = vec![0.0; n];
        mat_vec(a, n, p, &v, &mut u_vec);
        let sigma = u_vec.iter().map(|x| x * x).sum::<f64>().sqrt();
        if sigma > 1e-15 {
            let inv = 1.0 / sigma;
            for x in &mut u_vec { *x *= inv; }
        }
        s[comp] = sigma;
        for i in 0..n { u[i * kk + comp] = u_vec[i]; }
        for j in 0..p { vt[comp * p + j] = v[j]; }
    }
    (u, s, vt)
}

pub fn symeig(a_in: &[f64], p: usize) -> (Vec<f64>, Vec<f64>) {
    let mut a = a_in.to_vec();
    let mut v = vec![0.0; p * p];
    for i in 0..p { v[i * p + i] = 1.0; }
    for _ in 0..100 {
        let mut max_off = 0.0f64;
        for i in 0..p { for j in (i + 1)..p { let x = a[i * p + j].abs(); if x > max_off { max_off = x; } } }
        if max_off < 1e-14 { break; }
        for i in 0..p {
            for j in (i + 1)..p {
                let aij = a[i * p + j];
                if aij.abs() < 1e-15 { continue; }
                let theta = (a[j * p + j] - a[i * p + i]) / (2.0 * aij);
                let t = if theta >= 0.0 { 1.0 / (theta + (1.0 + theta * theta).sqrt()) }
                        else { -1.0 / (-theta + (1.0 + theta * theta).sqrt()) };
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = t * c;
                let aii = a[i * p + i]; let ajj = a[j * p + j];
                a[i * p + i] = aii - t * aij;
                a[j * p + j] = ajj + t * aij;
                a[i * p + j] = 0.0; a[j * p + i] = 0.0;
                for k in 0..p {
                    if k != i && k != j {
                        let aki = a[k * p + i]; let akj = a[k * p + j];
                        a[k * p + i] = c * aki - s * akj; a[i * p + k] = a[k * p + i];
                        a[k * p + j] = s * aki + c * akj; a[j * p + k] = a[k * p + j];
                    }
                }
                for k in 0..p {
                    let vki = v[k * p + i]; let vkj = v[k * p + j];
                    v[k * p + i] = c * vki - s * vkj;
                    v[k * p + j] = s * vki + c * vkj;
                }
            }
        }
    }
    let evals: Vec<f64> = (0..p).map(|i| a[i * p + i]).collect();
    (evals, v)
}

pub fn soft_threshold(x: f64, lambda: f64) -> f64 {
    if x > lambda { x - lambda }
    else if x < -lambda { x + lambda }
    else { 0.0 }
}

pub fn col_means(a: &[f64], n: usize, p: usize) -> Vec<f64> {
    let inv = 1.0 / n as f64;
    if n * p >= 100_000 {
        let chunk = 4096usize.min(n);
        let nc = (n + chunk - 1) / chunk;
        let partials: Vec<Vec<f64>> = (0..nc).into_par_iter().map(|c| {
            let s = c * chunk;
            let e = (s + chunk).min(n);
            let mut part = vec![0.0; p];
            for i in s..e {
                for j in 0..p { part[j] += a[i * p + j]; }
            }
            part
        }).collect();
        let mut m = vec![0.0; p];
        for part in &partials { for j in 0..p { m[j] += part[j]; } }
        for j in 0..p { m[j] *= inv; }
        m
    } else {
        let mut m = vec![0.0; p];
        for i in 0..n {
            for j in 0..p { m[j] += a[i * p + j]; }
        }
        for j in 0..p { m[j] *= inv; }
        m
    }
}

pub fn col_std(a: &[f64], n: usize, p: usize, means: &[f64]) -> Vec<f64> {
    let inv = 1.0 / n as f64;
    if n * p >= 100_000 {
        let chunk = 4096usize.min(n);
        let nc = (n + chunk - 1) / chunk;
        let partials: Vec<Vec<f64>> = (0..nc).into_par_iter().map(|c| {
            let s = c * chunk;
            let e = (s + chunk).min(n);
            let mut part = vec![0.0; p];
            for i in s..e {
                for j in 0..p { let d = a[i * p + j] - means[j]; part[j] += d * d; }
            }
            part
        }).collect();
        let mut v = vec![0.0; p];
        for part in &partials { for j in 0..p { v[j] += part[j]; } }
        for j in 0..p { v[j] = (v[j] * inv).sqrt().max(1e-15); }
        v
    } else {
        let mut v = vec![0.0; p];
        for i in 0..n {
            for j in 0..p { let d = a[i * p + j] - means[j]; v[j] += d * d; }
        }
        for j in 0..p { v[j] = (v[j] * inv).sqrt().max(1e-15); }
        v
    }
}

#[inline(always)]
pub fn sigmoid(x: f64) -> f64 {
    if x >= 0.0 { 1.0 / (1.0 + (-x).exp()) }
    else { let e = x.exp(); e / (1.0 + e) }
}

pub fn argmax(a: &[f64]) -> usize {
    let mut bi = 0;
    let mut bv = f64::NEG_INFINITY;
    for (i, &v) in a.iter().enumerate() {
        if v > bv { bv = v; bi = i; }
    }
    bi
}

pub fn argsort(a: &[f64]) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..a.len()).collect();
    idx.sort_unstable_by(|&i, &j| a[i].partial_cmp(&a[j]).unwrap_or(std::cmp::Ordering::Equal));
    idx
}

pub fn mat_mul(a: &[f64], m: usize, k: usize, b: &[f64], bn: usize, c: &mut [f64]) {
    if m == 0 || k == 0 || bn == 0 {
        for v in c[..m * bn].iter_mut() { *v = 0.0; }
        return;
    }
    if m * k * bn < 4096 {
        for v in c[..m * bn].iter_mut() { *v = 0.0; }
        for i in 0..m {
            let arow = &a[i * k..i * k + k];
            let crow = &mut c[i * bn..i * bn + bn];
            for t in 0..k {
                let ait = arow[t];
                let brow = &b[t * bn..t * bn + bn];
                for j in 0..bn { crow[j] += ait * brow[j]; }
            }
        }
        return;
    }
    unsafe {
        matrixmultiply::dgemm(
            m, k, bn,
            1.0,
            a.as_ptr(), k as isize, 1,
            b.as_ptr(), bn as isize, 1,
            0.0,
            c.as_mut_ptr(), bn as isize, 1,
        );
    }
}

pub fn mat_t_mul(a: &[f64], m: usize, k: usize, b: &[f64], bn: usize, c: &mut [f64]) {
    if m == 0 || k == 0 || bn == 0 {
        for v in c[..k * bn].iter_mut() { *v = 0.0; }
        return;
    }
    if m * k * bn < 4096 {
        for v in c[..k * bn].iter_mut() { *v = 0.0; }
        for r in 0..m {
            let arow = &a[r * k..r * k + k];
            let brow = &b[r * bn..r * bn + bn];
            for i in 0..k {
                let ai = arow[i];
                let crow = &mut c[i * bn..i * bn + bn];
                for j in 0..bn { crow[j] += ai * brow[j]; }
            }
        }
        return;
    }
    unsafe {
        matrixmultiply::dgemm(
            k, m, bn,
            1.0,
            a.as_ptr(), 1, k as isize,
            b.as_ptr(), bn as isize, 1,
            0.0,
            c.as_mut_ptr(), bn as isize, 1,
        );
    }
}

pub fn svd_randomized(a: &[f64], n: usize, p: usize, k: usize, n_over: usize, n_piter: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let kk = k.min(n).min(p);
    let l = (kk + n_over).min(n).min(p);
    let mut rng = 0x9E3779B97F4A7C15u64;
    let mut omega = vec![0.0; p * l];
    let mut idx = 0;
    while idx + 1 < p * l {
        rng = splitmix64(rng);
        let u1 = ((rng >> 11) as f64 / (1u64 << 53) as f64).max(1e-300);
        rng = splitmix64(rng);
        let u2 = (rng >> 11) as f64 / (1u64 << 53) as f64;
        let r = (-2.0 * u1.ln()).sqrt();
        let th = 2.0 * std::f64::consts::PI * u2;
        omega[idx] = r * th.cos();
        omega[idx + 1] = r * th.sin();
        idx += 2;
    }
    if idx < p * l {
        rng = splitmix64(rng);
        omega[idx] = ((rng >> 11) as f64 / (1u64 << 53) as f64) - 0.5;
    }
    let mut y = vec![0.0; n * l];
    mat_mul(a, n, p, &omega, l, &mut y);
    for _ in 0..n_piter {
        let (q, _) = qr_economy(&y, n, l);
        let mut z = vec![0.0; p * l];
        mat_t_mul(a, n, p, &q, l, &mut z);
        let (q2, _) = qr_economy(&z, p, l);
        mat_mul(a, n, p, &q2, l, &mut y);
    }
    let (q, _) = qr_economy(&y, n, l);
    let mut b = vec![0.0; l * p];
    mat_t_mul(&q, n, l, a, p, &mut b);
    let (ub, s, vt) = svd_truncated(&b, l, p, kk, 100);
    let mut u = vec![0.0; n * kk];
    mat_mul(&q, n, l, &ub, kk, &mut u);
    (u, s, vt)
}

pub fn discover_classes(y: &[i32]) -> Vec<i32> {
    let mut c: Vec<i32> = y.to_vec();
    c.sort_unstable();
    c.dedup();
    c
}

pub fn weighted_bootstrap(n: usize, weights: &[f64], rng: &mut u64) -> Vec<usize> {
    let total: f64 = weights.iter().sum();
    let scale = n as f64 / total;
    let mut prob: Vec<f64> = weights.iter().map(|&w| w * scale).collect();
    let mut alias: Vec<usize> = (0..n).collect();
    let mut small: Vec<usize> = Vec::with_capacity(n);
    let mut large: Vec<usize> = Vec::with_capacity(n);
    for i in 0..n {
        if prob[i] < 1.0 { small.push(i); } else { large.push(i); }
    }
    while !small.is_empty() && !large.is_empty() {
        let s = small.pop().unwrap();
        let l = *large.last().unwrap();
        alias[s] = l;
        prob[l] -= 1.0 - prob[s];
        if prob[l] < 1.0 { large.pop(); small.push(l); }
    }
    let mut indices = Vec::with_capacity(n);
    for _ in 0..n {
        *rng = splitmix64(*rng);
        let u = *rng;
        let i = ((u >> 33) as usize).wrapping_mul(n) >> 31;
        let i = i.min(n - 1);
        let frac = (u & 0x1FFFFF) as f64 * (1.0 / 0x200000 as f64);
        indices.push(if frac < unsafe { *prob.get_unchecked(i) } { i } else { unsafe { *alias.get_unchecked(i) } });
    }
    indices
}


