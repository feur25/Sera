use crate::ml::linalg::{dot, col_means, col_std, soft_threshold, mat_vec};
use rayon::prelude::*;

pub struct ElasticNet {
    pub coef: Vec<f64>,
    pub intercept: f64,
    pub alpha: f64,
    pub l1_ratio: f64,
    pub max_iter: usize,
    pub tol: f64,
    pub fit_intercept: bool,
    pub n_iter: usize,
}

impl ElasticNet {
    pub fn new(alpha: f64, l1_ratio: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> Self {
        Self { coef: Vec::new(), intercept: 0.0, alpha, l1_ratio, max_iter, tol, fit_intercept, n_iter: 0 }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) { self.fit_resumable(x, n, p, y, None); }

    pub fn fit_resumable(&mut self, x: &[f64], n: usize, p: usize, y: &[f64], checkpoint_id: Option<u64>) {
        let l1 = self.alpha * self.l1_ratio;
        let l2 = self.alpha * (1.0 - self.l1_ratio);
        let inv_n = 1.0 / n as f64;
        let (x_col, yw, means, stds, y_mean) = if self.fit_intercept {
            let m = col_means(x, n, p);
            let s = col_std(x, n, p, &m);
            let ym = y.iter().sum::<f64>() * inv_n;
            let mut xc = vec![0.0; n * p];
            let mut yn = vec![0.0; n];
            if n * p >= 100_000 {
                xc.par_chunks_mut(n).enumerate().for_each(|(j, col)| {
                    let inv_s = 1.0 / s[j];
                    let mj = m[j];
                    for i in 0..n { col[i] = (x[i * p + j] - mj) * inv_s; }
                });
            } else {
                for j in 0..p {
                    let inv_s = 1.0 / s[j];
                    let mj = m[j];
                    let col = &mut xc[j * n..(j + 1) * n];
                    for i in 0..n { col[i] = (x[i * p + j] - mj) * inv_s; }
                }
            }
            for i in 0..n { yn[i] = y[i] - ym; }
            (xc, yn, m, s, ym)
        } else {
            let mut xc = vec![0.0; n * p];
            if n * p >= 100_000 {
                xc.par_chunks_mut(n).enumerate().for_each(|(j, col)| {
                    for i in 0..n { col[i] = x[i * p + j]; }
                });
            } else {
                for j in 0..p {
                    let col = &mut xc[j * n..(j + 1) * n];
                    for i in 0..n { col[i] = x[i * p + j]; }
                }
            }
            (xc, y.to_vec(), vec![0.0; p], vec![1.0; p], 0.0)
        };

        let xj_sq: Vec<f64> = (0..p).map(|j| {
            let col = &x_col[j * n..(j + 1) * n];
            let (mut s0, mut s1, mut s2, mut s3) = (0.0, 0.0, 0.0, 0.0);
            let mut i = 0;
            while i + 4 <= n { s0 += col[i]*col[i]; s1 += col[i+1]*col[i+1]; s2 += col[i+2]*col[i+2]; s3 += col[i+3]*col[i+3]; i += 4; }
            while i < n { s0 += col[i]*col[i]; i += 1; }
            s0 + s1 + s2 + s3
        }).collect();

        let mut w = vec![0.0; p];
        let start_iter = if let Some(id) = checkpoint_id {
            if let Some(entry) = crate::ml::cache::checkpoint_load(id) {
                if entry.weights.len() == p { w.copy_from_slice(&entry.weights); }
                entry.iteration.min(self.max_iter)
            } else { 0 }
        } else { 0 };

        let use_gram = p <= 256 && (n as u128) * (p as u128) >= 200_000;
        if use_gram {
            let mut gram = vec![0.0f64; p * p];
            let mut q = vec![0.0f64; p];
            let chunk_n = 4096usize.min(n).max(1);
            let nc = (n + chunk_n - 1) / chunk_n;
            let partials: Vec<(Vec<f64>, Vec<f64>)> = (0..nc).into_par_iter().map(|ci| {
                let s = ci * chunk_n;
                let e = (s + chunk_n).min(n);
                let mut pg = vec![0.0f64; p * p];
                let mut pq = vec![0.0f64; p];
                for k in 0..p {
                    let ck = &x_col[k * n + s..k * n + e];
                    for k2 in k..p {
                        let ck2 = &x_col[k2 * n + s..k2 * n + e];
                        let len = e - s;
                        let (mut a0, mut a1, mut a2, mut a3) = (0.0, 0.0, 0.0, 0.0);
                        let mut i = 0;
                        while i + 4 <= len { a0 += ck[i]*ck2[i]; a1 += ck[i+1]*ck2[i+1]; a2 += ck[i+2]*ck2[i+2]; a3 += ck[i+3]*ck2[i+3]; i += 4; }
                        while i < len { a0 += ck[i]*ck2[i]; i += 1; }
                        pg[k*p+k2] += a0+a1+a2+a3;
                    }
                    let yc = &yw[s..e];
                    let len = e - s;
                    let (mut a0, mut a1, mut a2, mut a3) = (0.0, 0.0, 0.0, 0.0);
                    let mut i = 0;
                    while i + 4 <= len { a0 += ck[i]*yc[i]; a1 += ck[i+1]*yc[i+1]; a2 += ck[i+2]*yc[i+2]; a3 += ck[i+3]*yc[i+3]; i += 4; }
                    while i < len { a0 += ck[i]*yc[i]; i += 1; }
                    pq[k] += a0+a1+a2+a3;
                }
                (pg, pq)
            }).collect();
            for (pg, pq) in &partials {
                for k in 0..p { for k2 in k..p { gram[k*p+k2] += pg[k*p+k2]; } q[k] += pq[k]; }
            }
            for k in 0..p { for k2 in k+1..p { gram[k2*p+k] = gram[k*p+k2]; } }
            for iter in start_iter..self.max_iter {
                let mut max_change = 0.0f64;
                for j in 0..p {
                    let gjj = gram[j*p+j];
                    if gjj < 1e-15 { continue; }
                    let old = w[j];
                    let row = &gram[j*p..(j+1)*p];
                    let mut s = 0.0f64;
                    for k in 0..p { s += row[k] * w[k]; }
                    let xj_r = q[j] - s + gjj * old;
                    let new_val = soft_threshold(xj_r * inv_n, l1) / (gjj * inv_n + l2);
                    let delta = new_val - old;
                    if delta.abs() > 1e-15 {
                        w[j] = new_val;
                        max_change = max_change.max(delta.abs());
                    }
                }
                self.n_iter = iter + 1;
                if max_change < self.tol { break; }
                if let Some(id) = checkpoint_id {
                    if (iter + 1) % 20 == 0 { crate::ml::cache::checkpoint_save(id, &w, iter + 1); }
                }
            }
            if let Some(id) = checkpoint_id { crate::ml::cache::checkpoint_clear(id); }
            if self.fit_intercept {
                self.coef = (0..p).map(|j| w[j] / stds[j]).collect();
                self.intercept = y_mean - dot(&self.coef, &means);
            } else {
                self.coef = w;
                self.intercept = 0.0;
            }
            return;
        }

        let mut r = yw;
        let par_threshold = 50_000usize;
        let chunk = ((n + 7) / 8).max(8192);
        for iter in start_iter..self.max_iter {
            let mut max_change = 0.0f64;
            for j in 0..p {
                if xj_sq[j] < 1e-15 { continue; }
                let old = w[j];
                let col = &x_col[j * n..(j + 1) * n];
                let xj_r = if n >= par_threshold {
                    let s: f64 = col.par_chunks(chunk).zip(r.par_chunks(chunk)).map(|(c, rr)| {
                        let (mut s0, mut s1, mut s2, mut s3) = (0.0, 0.0, 0.0, 0.0);
                        let len = c.len();
                        let mut i = 0;
                        while i + 4 <= len { s0 += c[i]*rr[i]; s1 += c[i+1]*rr[i+1]; s2 += c[i+2]*rr[i+2]; s3 += c[i+3]*rr[i+3]; i += 4; }
                        while i < len { s0 += c[i]*rr[i]; i += 1; }
                        s0 + s1 + s2 + s3
                    }).sum();
                    s + xj_sq[j] * old
                } else {
                    let (mut d0, mut d1, mut d2, mut d3) = (0.0, 0.0, 0.0, 0.0);
                    let mut i = 0;
                    while i + 4 <= n { d0 += col[i]*r[i]; d1 += col[i+1]*r[i+1]; d2 += col[i+2]*r[i+2]; d3 += col[i+3]*r[i+3]; i += 4; }
                    while i < n { d0 += col[i]*r[i]; i += 1; }
                    d0 + d1 + d2 + d3 + xj_sq[j] * old
                };
                let new_val = soft_threshold(xj_r * inv_n, l1) / (xj_sq[j] * inv_n + l2);
                let delta = new_val - old;
                if delta.abs() > 1e-15 {
                    w[j] = new_val;
                    let nd = -delta;
                    if n >= par_threshold {
                        r.par_chunks_mut(chunk).zip(col.par_chunks(chunk)).for_each(|(rr, c)| {
                            let len = rr.len();
                            let mut i = 0;
                            while i + 4 <= len { rr[i] += c[i]*nd; rr[i+1] += c[i+1]*nd; rr[i+2] += c[i+2]*nd; rr[i+3] += c[i+3]*nd; i += 4; }
                            while i < len { rr[i] += c[i]*nd; i += 1; }
                        });
                    } else {
                        let mut i = 0;
                        while i + 4 <= n { r[i] += col[i]*nd; r[i+1] += col[i+1]*nd; r[i+2] += col[i+2]*nd; r[i+3] += col[i+3]*nd; i += 4; }
                        while i < n { r[i] += col[i]*nd; i += 1; }
                    }
                    max_change = max_change.max(delta.abs());
                }
            }
            self.n_iter = iter + 1;
            if max_change < self.tol { break; }
            if let Some(id) = checkpoint_id {
                if (iter + 1) % 20 == 0 { crate::ml::cache::checkpoint_save(id, &w, iter + 1); }
            }
        }
        if let Some(id) = checkpoint_id { crate::ml::cache::checkpoint_clear(id); }

        if self.fit_intercept {
            self.coef = (0..p).map(|j| w[j] / stds[j]).collect();
            self.intercept = y_mean - dot(&self.coef, &means);
        } else {
            self.coef = w;
            self.intercept = 0.0;
        }
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let mut out = vec![0.0; n];
        mat_vec(x, n, p, &self.coef, &mut out);
        let b = self.intercept;
        for v in out.iter_mut() { *v += b; }
        out
    }
}

impl crate::ml::MlRegressor for ElasticNet {
    fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) { self.fit(x, n, p, y); }
    fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> { self.predict(x, n, p) }
}


