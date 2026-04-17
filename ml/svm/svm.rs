use crate::ml::linalg::{dot, splitmix64};
use rayon::prelude::*;

pub struct LinearSVC {
    pub c: f64,
    pub max_iter: usize,
    pub tol: f64,
    pub classes: Vec<i32>,
    coefs: Vec<Vec<f64>>,
    intercepts: Vec<f64>,
    p: usize,
}

impl LinearSVC {
    pub fn new(c: f64, max_iter: usize, tol: f64) -> Self {
        Self { c, max_iter, tol, classes: Vec::new(), coefs: Vec::new(), intercepts: Vec::new(), p: 0 }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) {
        self.p = p;
        let mut cls = y.to_vec();
        cls.sort_unstable();
        cls.dedup();
        self.classes = cls.clone();

        if self.classes.len() == 2 {
            let labels: Vec<f64> = y.iter().map(|&yi| if yi == self.classes[1] { 1.0 } else { -1.0 }).collect();
            let (w, b) = train_linear_svm(x, n, p, &labels, self.c, self.max_iter, self.tol);
            self.coefs = vec![w];
            self.intercepts = vec![b];
        } else {
            let c = self.c;
            let mi = self.max_iter;
            let tol = self.tol;
            let results: Vec<(Vec<f64>, f64)> = cls.par_iter().map(|&ci| {
                let labels: Vec<f64> = y.iter().map(|&yi| if yi == ci { 1.0 } else { -1.0 }).collect();
                train_linear_svm(x, n, p, &labels, c, mi, tol)
            }).collect();
            self.coefs = results.iter().map(|r| r.0.clone()).collect();
            self.intercepts = results.iter().map(|r| r.1).collect();
        }
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> {
        let predict_one = |i: usize| -> i32 {
            let xi = &x[i * p..(i + 1) * p];
            if self.classes.len() == 2 {
                let s = dot(xi, &self.coefs[0]) + self.intercepts[0];
                if s >= 0.0 { self.classes[1] } else { self.classes[0] }
            } else {
                let mut best = 0;
                let mut best_s = f64::NEG_INFINITY;
                for (c, w) in self.coefs.iter().enumerate() {
                    let s = dot(xi, w) + self.intercepts[c];
                    if s > best_s { best_s = s; best = c; }
                }
                self.classes[best]
            }
        };
        if n >= 512 {
            (0..n).into_par_iter().map(predict_one).collect()
        } else {
            (0..n).map(predict_one).collect()
        }
    }

    pub fn decision_function(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        if self.classes.len() == 2 {
            (0..n).map(|i| {
                let xi = &x[i * p..(i + 1) * p];
                dot(xi, &self.coefs[0]) + self.intercepts[0]
            }).collect()
        } else {
            let k = self.classes.len();
            let mut out = vec![0.0; n * k];
            for i in 0..n {
                let xi = &x[i * p..(i + 1) * p];
                for c in 0..k {
                    out[i * k + c] = dot(xi, &self.coefs[c]) + self.intercepts[c];
                }
            }
            out
        }
    }

    pub fn coef(&self) -> &[Vec<f64>] { &self.coefs }
    pub fn intercept(&self) -> &[f64] { &self.intercepts }
}

pub struct LinearSVR {
    pub c: f64,
    pub epsilon: f64,
    pub max_iter: usize,
    pub tol: f64,
    w: Vec<f64>,
    b: f64,
}

impl LinearSVR {
    pub fn new(c: f64, epsilon: f64, max_iter: usize, tol: f64) -> Self {
        Self { c, epsilon, max_iter, tol, w: Vec::new(), b: 0.0 }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) {
        let c = self.c;
        let eps = self.epsilon;
        let mut w = vec![0.0; p];
        let mut b = 0.0;
        let mut ap = vec![0.0; n];
        let mut an = vec![0.0; n];

        let qd: Vec<f64> = (0..n).map(|i| {
            let xi = unsafe { x.get_unchecked(i * p..(i + 1) * p) };
            let mut s = 1.0;
            let mut k = 0;
            while k + 4 <= p {
                s += xi[k]*xi[k] + xi[k+1]*xi[k+1] + xi[k+2]*xi[k+2] + xi[k+3]*xi[k+3];
                k += 4;
            }
            while k < p { s += xi[k]*xi[k]; k += 1; }
            s
        }).collect();

        let mut rng = 0xCAFEBABEu64;
        let mut idx: Vec<usize> = (0..n).collect();
        let mut init_pv = 0.0f64;

        for iter in 0..self.max_iter {
            for i in (1..n).rev() {
                rng = splitmix64(rng);
                idx.swap(i, rng as usize % (i + 1));
            }

            let mut max_pv = 0.0f64;

            for &i in &idx {
                let xi = unsafe { x.get_unchecked(i * p..(i + 1) * p) };
                let pred = dot(xi, &w) + b;

                {
                    let g = pred - y[i] + eps;
                    let ai = ap[i];
                    let pg = if ai <= 0.0 { g.min(0.0) } else if ai >= c { g.max(0.0) } else { g };
                    let a = pg.abs();
                    if a > max_pv { max_pv = a; }
                    if a > 1e-12 {
                        let na = (ai - g / unsafe { *qd.get_unchecked(i) }).clamp(0.0, c);
                        let d = na - ai;
                        ap[i] = na;
                        for j in 0..p { unsafe { *w.get_unchecked_mut(j) += d * *xi.get_unchecked(j); } }
                        b += d;
                    }
                }

                let pred2 = dot(xi, &w) + b;

                {
                    let g = y[i] - pred2 + eps;
                    let ai = an[i];
                    let pg = if ai <= 0.0 { g.min(0.0) } else if ai >= c { g.max(0.0) } else { g };
                    let a = pg.abs();
                    if a > max_pv { max_pv = a; }
                    if a > 1e-12 {
                        let na = (ai - g / unsafe { *qd.get_unchecked(i) }).clamp(0.0, c);
                        let d = na - ai;
                        an[i] = na;
                        for j in 0..p { unsafe { *w.get_unchecked_mut(j) -= d * *xi.get_unchecked(j); } }
                        b -= d;
                    }
                }
            }

            if iter == 0 { init_pv = max_pv; if init_pv < 1e-15 { break; } }
            if max_pv < self.tol * init_pv { break; }
        }
        self.w = w;
        self.b = b;
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        if n >= 512 {
            (0..n).into_par_iter().map(|i| {
                dot(unsafe { x.get_unchecked(i * p..(i + 1) * p) }, &self.w) + self.b
            }).collect()
        } else {
            (0..n).map(|i| {
                dot(&x[i * p..(i + 1) * p], &self.w) + self.b
            }).collect()
        }
    }

    pub fn coef(&self) -> &[f64] { &self.w }
    pub fn intercept(&self) -> f64 { self.b }
}

fn train_linear_svm(x: &[f64], n: usize, p: usize, y: &[f64], c: f64, max_iter: usize, tol: f64) -> (Vec<f64>, f64) {
    let mut w = vec![0.0; p];
    let mut b = 0.0;
    let mut alpha = vec![0.0; n];
    let inv_2c = 0.5 / c;
    let mut qd = vec![0.0; n];
    for i in 0..n {
        let xi = &x[i * p..(i + 1) * p];
        let mut s = 1.0 + inv_2c;
        for j in 0..p { s += xi[j] * xi[j]; }
        qd[i] = s;
    }
    let mut init_pv = 0.0f64;
    let mut rng = 0xDEADCAFEu64;
    let mut idx: Vec<usize> = (0..n).collect();
    for iter in 0..max_iter {
        for i in (1..n).rev() {
            rng ^= rng << 13; rng ^= rng >> 7; rng ^= rng << 17;
            idx.swap(i, rng as usize % (i + 1));
        }
        let mut max_pv = 0.0f64;
        for &i in &idx {
            let xi = unsafe { x.get_unchecked(i * p..(i + 1) * p) };
            let mut dp = b;
            for j in 0..p { dp += unsafe { *w.get_unchecked(j) * *xi.get_unchecked(j) }; }
            let yi = unsafe { *y.get_unchecked(i) };
            let ai = unsafe { *alpha.get_unchecked(i) };
            let gi = yi * dp + ai * inv_2c - 1.0;
            let pg = if ai <= 0.0 { gi.min(0.0) } else { gi };
            let apg = pg.abs();
            if apg > max_pv { max_pv = apg; }
            if apg < 1e-12 { continue; }
            let new_ai = (ai - gi / unsafe { *qd.get_unchecked(i) }).max(0.0);
            unsafe { *alpha.get_unchecked_mut(i) = new_ai; }
            let d = (new_ai - ai) * yi;
            for j in 0..p { unsafe { *w.get_unchecked_mut(j) += d * *xi.get_unchecked(j); } }
            b += d;
        }
        if iter == 0 { init_pv = max_pv; if init_pv < 1e-15 { break; } }
        if max_pv < tol * init_pv { break; }
    }
    (w, b)
}
