use crate::ml::linalg::{dot, svd_truncated, svd_randomized, col_means, symeig};
use rayon::prelude::*;

#[crate::model(category = "Decomposition", domain = "ml")]
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
            let mut cov = vec![0.0; p * p];
            crate::ml::linalg::mat_t_mat(x, n, p, &mut cov);
            let inv_nm1 = 1.0 / nm1;
            let nf = n as f64;
            for i in 0..p {
                for j in 0..p {
                    cov[i * p + j] = (cov[i * p + j] - nf * self.mean[i] * self.mean[j]) * inv_nm1;
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
        if n == 0 || k == 0 { return out; }
        let bias = &self.bias;
        let whiten_scale: Vec<f64> = if self.whiten {
            self.explained_variance.iter().map(|&v| 1.0 / v.max(1e-15).sqrt()).collect()
        } else {
            vec![1.0; k]
        };
        let comps = &self.components;
        if n * p * k < 200_000 {
            for i in 0..n {
                let xi = &x[i * p..i * p + p];
                for c in 0..k { out[i * k + c] = (dot(xi, &comps[c * p..(c + 1) * p]) - bias[c]) * whiten_scale[c]; }
            }
            return out;
        }
        unsafe {
            matrixmultiply::dgemm(
                n, p, k,
                1.0,
                x.as_ptr(), p as isize, 1,
                self.components.as_ptr(), 1, p as isize,
                0.0,
                out.as_mut_ptr(), k as isize, 1,
            );
        }
        let need_post = self.whiten || bias.iter().any(|&b| b != 0.0);
        if need_post {
            out.par_chunks_mut(k).for_each(|row| {
                for c in 0..k { row[c] = (row[c] - bias[c]) * whiten_scale[c]; }
            });
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
        if n == 0 || p == 0 || k == 0 {
            for i in 0..n { for j in 0..p { out[i * p + j] = self.mean[j]; } }
            return out;
        }
        crate::ml::linalg::mat_mul(x_reduced, n, k, &self.components, p, &mut out);
        out.par_chunks_mut(p).for_each(|row| {
            for j in 0..p { row[j] += self.mean[j]; }
        });
        out
    }
}

#[crate::model(category = "Decomposition", domain = "ml")]
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
        crate::ml::linalg::mat_t_mat(x, n, p, &mut ata);
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
        let total_var = if n * p >= 100_000 {
            x.par_chunks(p).map(|row| row.iter().map(|&v| v * v).sum::<f64>()).sum::<f64>() / (n_f - 1.0)
        } else {
            x.iter().map(|&v| v * v).sum::<f64>() / (n_f - 1.0)
        };
        self.explained_variance_ratio = self.explained_variance.iter().map(|&v| v / total_var.max(1e-15)).collect();
    }

    pub fn transform(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let k = self.n_components.min(self.singular_values.len());
        let mut out = vec![0.0; n * k];
        if n == 0 || k == 0 { return out; }
        if n * p * k < 200_000 {
            for i in 0..n {
                for c in 0..k {
                    let mut s = 0.0;
                    for j in 0..p { s += x[i * p + j] * self.components[c * p + j]; }
                    out[i * k + c] = s;
                }
            }
            return out;
        }
        unsafe {
            matrixmultiply::dgemm(
                n, p, k,
                1.0,
                x.as_ptr(), p as isize, 1,
                self.components.as_ptr(), 1, p as isize,
                0.0,
                out.as_mut_ptr(), k as isize, 1,
            );
        }
        out
    }

    pub fn fit_transform(&mut self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        self.fit(x, n, p);
        self.transform(x, n, p)
    }
}


