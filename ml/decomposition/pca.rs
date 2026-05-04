use crate::ml::linalg::{dot, svd_truncated, svd_randomized, col_means, symeig};
use rayon::prelude::*;

pub struct PCA {
    pub n_components: usize,
    pub components: Vec<f64>,
    pub explained_variance: Vec<f64>,
    pub explained_variance_ratio: Vec<f64>,
    pub singular_values: Vec<f64>,
    pub mean: Vec<f64>,
    pub whiten: bool,
    pub svd_solver: String,
    bias: Vec<f64>,
    p: usize,
}

impl PCA {
    pub fn new(n_components: usize) -> Self {
        Self {
            n_components, components: Vec::new(), explained_variance: Vec::new(),
            explained_variance_ratio: Vec::new(), singular_values: Vec::new(),
            mean: Vec::new(), whiten: false, svd_solver: "auto".to_string(),
            bias: Vec::new(), p: 0,
        }
    }

    pub fn with_options(n_components: usize, svd_solver: String, whiten: bool) -> Self {
        Self {
            n_components, components: Vec::new(), explained_variance: Vec::new(),
            explained_variance_ratio: Vec::new(), singular_values: Vec::new(),
            mean: Vec::new(), whiten, svd_solver,
            bias: Vec::new(), p: 0,
        }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize) {
        self.p = p;
        let k = self.n_components.min(n).min(p);

        self.mean = col_means(x, n, p);
        let nm1 = (n as f64 - 1.0).max(1.0);

        let use_randomized = match self.svd_solver.as_str() {
            "randomized" => true,
            "full" | "covariance_eigh" => false,
            _ => p >= 200 && k * 3 < p && n >= p * 4,
        };
        let use_covariance = !use_randomized && match self.svd_solver.as_str() {
            "full" => false,
            _ => n >= p * 4,
        };

        if use_randomized {
            let mut centered = vec![0.0; n * p];
            if n * p >= 100_000 {
                centered.par_chunks_mut(p).enumerate().for_each(|(i, row)| {
                    let xi = &x[i * p..i * p + p];
                    for j in 0..p { row[j] = xi[j] - self.mean[j]; }
                });
            } else {
                for i in 0..n {
                    for j in 0..p { centered[i * p + j] = x[i * p + j] - self.mean[j]; }
                }
            }
            let n_over = 10.min(p.saturating_sub(k));
            let n_piter = if n >= 10 * p { 4 } else { 6 };
            let (_u, s, vt) = svd_randomized(&centered, n, p, k, n_over, n_piter);
            self.components = vt;
            self.singular_values = s.clone();
            self.explained_variance = s.iter().map(|&sv| sv * sv / nm1).collect();
            let mut total_var = 0.0;
            if n * p >= 200_000 {
                total_var = centered.par_chunks(p).map(|row| {
                    let mut s = 0.0;
                    for &v in row { s += v * v; }
                    s
                }).sum::<f64>() / nm1;
            } else {
                for &v in &centered { total_var += v * v; }
                total_var /= nm1;
            }
            self.explained_variance_ratio = self.explained_variance.iter().map(|&v| v / total_var.max(1e-15)).collect();
        } else if use_covariance {
            let chunk = 4096usize.min(n);
            let xtx = if n >= 1024 {
                let nc = (n + chunk - 1) / chunk;
                let partials: Vec<Vec<f64>> = (0..nc).into_par_iter().map(|c| {
                    let s = c * chunk;
                    let e = (s + chunk).min(n);
                    let mut part = vec![0.0; p * p];
                    let mut r = s;
                    while r + 4 <= e {
                        let r0 = unsafe { x.get_unchecked(r * p..(r + 1) * p) };
                        let r1 = unsafe { x.get_unchecked((r + 1) * p..(r + 2) * p) };
                        let r2 = unsafe { x.get_unchecked((r + 2) * p..(r + 3) * p) };
                        let r3 = unsafe { x.get_unchecked((r + 3) * p..(r + 4) * p) };
                        for i in 0..p {
                            let (a0, a1, a2, a3) = unsafe {
                                (*r0.get_unchecked(i), *r1.get_unchecked(i),
                                 *r2.get_unchecked(i), *r3.get_unchecked(i))
                            };
                            let base = i * p;
                            for j in i..p {
                                unsafe {
                                    *part.get_unchecked_mut(base + j) +=
                                        a0 * *r0.get_unchecked(j) + a1 * *r1.get_unchecked(j) +
                                        a2 * *r2.get_unchecked(j) + a3 * *r3.get_unchecked(j);
                                }
                            }
                        }
                        r += 4;
                    }
                    while r < e {
                        let row = unsafe { x.get_unchecked(r * p..r * p + p) };
                        for i in 0..p {
                            let ai = unsafe { *row.get_unchecked(i) };
                            let base = i * p;
                            for j in i..p { unsafe { *part.get_unchecked_mut(base + j) += ai * *row.get_unchecked(j); } }
                        }
                        r += 1;
                    }
                    part
                }).collect();
                let mut xtx = vec![0.0; p * p];
                for part in &partials { for idx in 0..p * p { xtx[idx] += part[idx]; } }
                xtx
            } else {
                let mut xtx = vec![0.0; p * p];
                for r in 0..n {
                    let row = &x[r * p..r * p + p];
                    for i in 0..p {
                        let ai = row[i];
                        for j in i..p { xtx[i * p + j] += ai * row[j]; }
                    }
                }
                xtx
            };

            let inv_nm1 = 1.0 / nm1;
            let nf = n as f64;
            let mut cov = xtx;
            for i in 0..p {
                for j in i..p {
                    let c = (cov[i * p + j] - nf * self.mean[i] * self.mean[j]) * inv_nm1;
                    cov[i * p + j] = c;
                    cov[j * p + i] = c;
                }
            }

            let total_var: f64 = (0..p).map(|i| cov[i * p + i]).sum();
            let (evals, evecs) = symeig(&cov, p);
            let mut idx: Vec<usize> = (0..p).collect();
            idx.sort_unstable_by(|&a, &b| evals[b].partial_cmp(&evals[a]).unwrap_or(std::cmp::Ordering::Equal));
            let mut components = vec![0.0; k * p];
            let mut eigenvalues = vec![0.0; k];
            for c in 0..k {
                let ei = idx[c];
                for j in 0..p { components[c * p + j] = evecs[j * p + ei]; }
                eigenvalues[c] = evals[ei].max(0.0);
            }
            self.components = components;
            self.explained_variance = eigenvalues.clone();
            self.singular_values = eigenvalues.iter().map(|&e| (e * nm1).sqrt()).collect();
            self.explained_variance_ratio = eigenvalues.iter().map(|&v| v / total_var.max(1e-15)).collect();
        } else {
            let mut centered = vec![0.0; n * p];
            for i in 0..n {
                for j in 0..p { centered[i * p + j] = x[i * p + j] - self.mean[j]; }
            }
            let (_u, s, vt) = svd_truncated(&centered, n, p, k, 100);
            self.components = vt;
            self.singular_values = s.clone();
            self.explained_variance = s.iter().map(|&sv| sv * sv / nm1).collect();
            let total_var: f64 = centered.iter().map(|&v| v * v).sum::<f64>() / nm1;
            self.explained_variance_ratio = self.explained_variance.iter().map(|&v| v / total_var.max(1e-15)).collect();
        }

        self.bias = (0..k.min(self.components.len() / p.max(1))).map(|c| {
            dot(&self.mean, &self.components[c * p..(c + 1) * p])
        }).collect();
    }

    pub fn transform(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let k = self.n_components.min(self.singular_values.len());
        let mut out = vec![0.0; n * k];
        let comps = &self.components;
        let bias = &self.bias;
        let whiten_scale: Vec<f64> = if self.whiten {
            self.explained_variance.iter().map(|&v| 1.0 / v.max(1e-15).sqrt()).collect()
        } else {
            vec![1.0; k]
        };
        if n * p >= 200_000 {
            let grain = 512usize;
            out.par_chunks_mut(k * grain).enumerate().for_each(|(ci, block)| {
                let s = ci * grain;
                let rows = block.len() / k;
                for r in 0..rows {
                    let i = s + r;
                    let xi = &x[i * p..i * p + p];
                    for c in 0..k { block[r * k + c] = (dot(xi, &comps[c * p..(c + 1) * p]) - bias[c]) * whiten_scale[c]; }
                }
            });
        } else {
            for i in 0..n {
                let xi = &x[i * p..i * p + p];
                for c in 0..k { out[i * k + c] = (dot(xi, &comps[c * p..(c + 1) * p]) - bias[c]) * whiten_scale[c]; }
            }
        }
        out
    }

    pub fn fit_transform(&mut self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        self.fit(x, n, p);
        self.transform(x, n, p)
    }

    pub fn inverse_transform(&self, x_reduced: &[f64], n: usize) -> Vec<f64> {
        let k = self.n_components.min(self.singular_values.len());
        let p = self.p;
        let mut out = vec![0.0; n * p];
        for i in 0..n {
            for j in 0..p {
                let mut s = self.mean[j];
                for c in 0..k { s += x_reduced[i * k + c] * self.components[c * p + j]; }
                out[i * p + j] = s;
            }
        }
        out
    }
}

pub struct TruncatedSVD {
    pub n_components: usize,
    pub components: Vec<f64>,
    pub singular_values: Vec<f64>,
    pub explained_variance: Vec<f64>,
    pub explained_variance_ratio: Vec<f64>,
}

impl TruncatedSVD {
    pub fn new(n_components: usize) -> Self {
        Self {
            n_components, components: Vec::new(), singular_values: Vec::new(),
            explained_variance: Vec::new(), explained_variance_ratio: Vec::new(),
        }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize) {
        let k = self.n_components.min(n).min(p);
        let mut ata = vec![0.0f64; p * p];
        for i in 0..n {
            for a in 0..p {
                let xa = x[i * p + a];
                if xa == 0.0 { continue; }
                for b in 0..p { ata[a * p + b] += xa * x[i * p + b]; }
            }
        }
        let (evals, v) = crate::ml::linalg::symeig(&ata, p);
        let mut order: Vec<usize> = (0..p).collect();
        order.sort_by(|&i, &j| evals[j].partial_cmp(&evals[i]).unwrap_or(std::cmp::Ordering::Equal));
        let mut s = vec![0.0; k];
        let mut vt = vec![0.0; k * p];
        for c in 0..k {
            let idx = order[c];
            s[c] = evals[idx].max(0.0).sqrt();
            for j in 0..p { vt[c * p + j] = v[j * p + idx]; }
        }
        self.components = vt;
        self.singular_values = s.clone();
        let n_f = n as f64;
        self.explained_variance = s.iter().map(|&sv| sv * sv / (n_f - 1.0)).collect();
        let mut total_var = 0.0;
        for i in 0..n {
            for j in 0..p { total_var += x[i * p + j] * x[i * p + j]; }
        }
        total_var /= n_f - 1.0;
        self.explained_variance_ratio = self.explained_variance.iter().map(|&v| v / total_var.max(1e-15)).collect();
    }

    pub fn transform(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let k = self.n_components.min(self.singular_values.len());
        let mut out = vec![0.0; n * k];
        for i in 0..n {
            for c in 0..k {
                let mut s = 0.0;
                for j in 0..p { s += x[i * p + j] * self.components[c * p + j]; }
                out[i * k + c] = s;
            }
        }
        out
    }

    pub fn fit_transform(&mut self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        self.fit(x, n, p);
        self.transform(x, n, p)
    }
}
