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

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) {
        self.fit_resumable(x, n, p, y, None);
    }

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
        let mut r = yw;

        for iter in start_iter..self.max_iter {
            let mut max_change = 0.0f64;
            for j in 0..p {
                if xj_sq[j] < 1e-15 { continue; }
                let old = w[j];
                let col = &x_col[j * n..(j + 1) * n];
                let (mut d0, mut d1, mut d2, mut d3) = (0.0, 0.0, 0.0, 0.0);
                let mut i = 0;
                while i + 4 <= n { d0 += col[i]*r[i]; d1 += col[i+1]*r[i+1]; d2 += col[i+2]*r[i+2]; d3 += col[i+3]*r[i+3]; i += 4; }
                while i < n { d0 += col[i]*r[i]; i += 1; }
                let xj_r = d0 + d1 + d2 + d3 + xj_sq[j] * old;
                let new_val = soft_threshold(xj_r * inv_n, l1) / (xj_sq[j] * inv_n + l2);
                let delta = new_val - old;
                if delta.abs() > 1e-15 {
                    w[j] = new_val;
                    let nd = -delta;
                    let mut i = 0;
                    while i + 4 <= n { r[i] += col[i]*nd; r[i+1] += col[i+1]*nd; r[i+2] += col[i+2]*nd; r[i+3] += col[i+3]*nd; i += 4; }
                    while i < n { r[i] += col[i]*nd; i += 1; }
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
