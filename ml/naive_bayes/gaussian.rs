use std::f64::consts::PI;
use rayon::prelude::*;

pub struct GaussianNB {
    pub classes: Vec<i32>,
    pub var_smoothing: f64,
    means: Vec<f64>,
    vars: Vec<f64>,
    priors: Vec<f64>,
    log_prior: Vec<f64>,
    log_norm: Vec<f64>,
    inv_2var: Vec<f64>,
    p: usize,
}

impl GaussianNB {
    pub fn new() -> Self {
        Self { classes: Vec::new(), var_smoothing: 1e-9, means: Vec::new(), vars: Vec::new(), priors: Vec::new(),
               log_prior: Vec::new(), log_norm: Vec::new(), inv_2var: Vec::new(), p: 0 }
    }

    pub fn with_var_smoothing(var_smoothing: f64) -> Self {
        Self { classes: Vec::new(), var_smoothing, means: Vec::new(), vars: Vec::new(), priors: Vec::new(),
               log_prior: Vec::new(), log_norm: Vec::new(), inv_2var: Vec::new(), p: 0 }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) {
        self.p = p;
        let classes = crate::ml::linalg::discover_classes(y);
        self.classes = classes;
        let k = self.classes.len();
        let kp = k * p;

        let cmin = *self.classes.iter().min().unwrap();
        let cmax = *self.classes.iter().max().unwrap();
        let crange = (cmax - cmin + 1) as usize;
        let mut cmap = vec![0u8; crange];
        for (i, &c) in self.classes.iter().enumerate() { cmap[(c - cmin) as usize] = i as u8; }

        let mut sums = vec![0.0f64; kp];
        let mut sq_sums = vec![0.0f64; kp];
        let mut counts = vec![0usize; k];
        for i in 0..n {
            let c = cmap[(y[i] - cmin) as usize] as usize;
            counts[c] += 1;
            let row = &x[i * p..(i + 1) * p];
            let base = c * p;
            let mut j = 0;
            while j + 4 <= p {
                let v0 = row[j]; let v1 = row[j+1]; let v2 = row[j+2]; let v3 = row[j+3];
                sums[base+j] += v0; sums[base+j+1] += v1; sums[base+j+2] += v2; sums[base+j+3] += v3;
                sq_sums[base+j] += v0*v0; sq_sums[base+j+1] += v1*v1; sq_sums[base+j+2] += v2*v2; sq_sums[base+j+3] += v3*v3;
                j += 4;
            }
            while j < p { let v = row[j]; sums[base+j] += v; sq_sums[base+j] += v*v; j += 1; }
        }

        self.means = vec![0.0; kp];
        self.vars = vec![0.0; kp];
        self.priors = vec![0.0; k];
        for c in 0..k {
            let inv = 1.0 / counts[c].max(1) as f64;
            for j in 0..p { self.means[c * p + j] = sums[c * p + j] * inv; }
            self.priors[c] = counts[c] as f64 / n as f64;
        }
        for c in 0..k {
            let inv = 1.0 / counts[c].max(1) as f64;
            for j in 0..p {
                let m = self.means[c * p + j];
                self.vars[c * p + j] = (sq_sums[c * p + j] * inv - m * m).max(0.0) + self.var_smoothing;
            }
        }

        self.log_prior = self.priors.iter().map(|&p| p.ln()).collect();
        self.log_norm = vec![0.0; kp];
        self.inv_2var = vec![0.0; kp];
        for c in 0..k {
            for j in 0..p {
                let v = self.vars[c * p + j];
                self.log_norm[c * p + j] = -0.5 * (2.0 * PI * v).ln();
                self.inv_2var[c * p + j] = -0.5 / v;
            }
        }
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> {
        let predict_one = |i: usize| -> i32 {
            let xi = &x[i * p..(i + 1) * p];
            let mut best = 0;
            let mut best_ll = f64::NEG_INFINITY;
            for c in 0..self.classes.len() {
                let ll = self.log_likelihood_fast(c, xi);
                if ll > best_ll { best_ll = ll; best = c; }
            }
            self.classes[best]
        };
        if n >= 128 {
            (0..n).into_par_iter().map(predict_one).collect()
        } else {
            (0..n).map(predict_one).collect()
        }
    }

    pub fn predict_proba(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let k = self.classes.len();
        let mut out = vec![0.0; n * k];
        for i in 0..n {
            let xi = &x[i * p..(i + 1) * p];
            let mut max_ll = f64::NEG_INFINITY;
            for c in 0..k {
                let ll = self.log_likelihood_fast(c, xi);
                out[i * k + c] = ll;
                if ll > max_ll { max_ll = ll; }
            }
            let mut sum = 0.0;
            for c in 0..k {
                out[i * k + c] = (out[i * k + c] - max_ll).exp();
                sum += out[i * k + c];
            }
            let inv = 1.0 / sum;
            for c in 0..k { out[i * k + c] *= inv; }
        }
        out
    }

    #[inline(always)]
    fn log_likelihood_fast(&self, c: usize, xi: &[f64]) -> f64 {
        let mut ll = self.log_prior[c];
        let base = c * self.p;
        let mut i = 0;
        let p = self.p;
        while i + 4 <= p {
            let d0 = xi[i] - self.means[base + i];
            let d1 = xi[i + 1] - self.means[base + i + 1];
            let d2 = xi[i + 2] - self.means[base + i + 2];
            let d3 = xi[i + 3] - self.means[base + i + 3];
            ll += self.log_norm[base + i] + self.inv_2var[base + i] * d0 * d0;
            ll += self.log_norm[base + i + 1] + self.inv_2var[base + i + 1] * d1 * d1;
            ll += self.log_norm[base + i + 2] + self.inv_2var[base + i + 2] * d2 * d2;
            ll += self.log_norm[base + i + 3] + self.inv_2var[base + i + 3] * d3 * d3;
            i += 4;
        }
        while i < p {
            let d = xi[i] - self.means[base + i];
            ll += self.log_norm[base + i] + self.inv_2var[base + i] * d * d;
            i += 1;
        }
        ll
    }

    pub fn theta(&self) -> &[f64] { &self.means }
    pub fn var(&self) -> &[f64] { &self.vars }
    pub fn class_prior(&self) -> &[f64] { &self.priors }
    pub fn n_features(&self) -> usize { self.p }
}

impl crate::ml::MlClassifier for GaussianNB {
    fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) { self.fit(x, n, p, y); }
    fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> { self.predict(x, n, p) }
}
