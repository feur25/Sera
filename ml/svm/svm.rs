use crate::ml::linalg::{dot, splitmix64, solve_spd};
use rayon::prelude::*;

#[crate::model(category = "SVM", domain = "ml")]
pub struct LinearSVC {
    pub c: f64,
    pub max_iter: usize,
    pub tol: f64,
    pub fit_intercept: bool,
    pub classes: Vec<i32>,
    coefs: Vec<Vec<f64>>,
    intercepts: Vec<f64>,
    p: usize,
}

impl LinearSVC {
    pub fn new(c: f64, max_iter: usize, tol: f64) -> Self {
        Self { c, max_iter, tol, fit_intercept: true, classes: Vec::new(), coefs: Vec::new(), intercepts: Vec::new(), p: 0 }
    }

    pub fn with_fit_intercept(c: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> Self {
        Self { c, max_iter, tol, fit_intercept, classes: Vec::new(), coefs: Vec::new(), intercepts: Vec::new(), p: 0 }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) {
        self.p = p;
        let mut cls = y.to_vec();
        cls.sort_unstable();
        cls.dedup();
        self.classes = cls.clone();

        if self.classes.len() == 2 {
            let labels: Vec<f64> = y.iter().map(|&yi| if yi == self.classes[1] { 1.0 } else { -1.0 }).collect();
            let (w, b) = train_linear_svm(x, n, p, &labels, self.c, self.max_iter, self.tol, self.fit_intercept, true);
            self.coefs = vec![w];
            self.intercepts = vec![b];
        } else {
            let c = self.c;
            let mi = self.max_iter;
            let tol = self.tol;
            let fi = self.fit_intercept;
            let results: Vec<(Vec<f64>, f64)> = cls.par_iter().map(|&ci| {
                let labels: Vec<f64> = y.iter().map(|&yi| if yi == ci { 1.0 } else { -1.0 }).collect();
                train_linear_svm(x, n, p, &labels, c, mi, tol, fi, false)
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

#[crate::model(category = "SVM", domain = "ml")]
pub struct LinearSVR {
    pub c: f64,
    pub epsilon: f64,
    pub max_iter: usize,
    pub tol: f64,
    pub fit_intercept: bool,
    w: Vec<f64>,
    b: f64,
}

impl LinearSVR {
    pub fn new(c: f64, epsilon: f64, max_iter: usize, tol: f64) -> Self {
        Self { c, epsilon, max_iter, tol, fit_intercept: true, w: Vec::new(), b: 0.0 }
    }

    pub fn with_fit_intercept(c: f64, epsilon: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> Self {
        Self { c, epsilon, max_iter, tol, fit_intercept, w: Vec::new(), b: 0.0 }
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
        let mut shuf_x = vec![0.0; n * p];
        let mut shuf_y = vec![0.0; n];
        let mut shuf_qd = vec![0.0; n];

        for iter in 0..self.max_iter {
            for i in (1..n).rev() {
                rng = splitmix64(rng);
                idx.swap(i, rng as usize % (i + 1));
            }
            for (si, &oi) in idx.iter().enumerate() {
                shuf_x[si * p..(si + 1) * p].copy_from_slice(&x[oi * p..(oi + 1) * p]);
                shuf_y[si] = y[oi];
                shuf_qd[si] = qd[oi];
            }

            let mut max_pv = 0.0f64;

            for si in 0..n {
                let oi = idx[si];
                let xi = unsafe { shuf_x.get_unchecked(si * p..(si + 1) * p) };
                let pred = dot(xi, &w) + b;

                {
                    let g = pred - shuf_y[si] + eps;
                    let ai = ap[oi];
                    let pg = if ai <= 0.0 { g.min(0.0) } else if ai >= c { g.max(0.0) } else { g };
                    let a = pg.abs();
                    if a > max_pv { max_pv = a; }
                    if a > 1e-12 {
                        let na = (ai - g / shuf_qd[si]).clamp(0.0, c);
                        let d = na - ai;
                        ap[oi] = na;
                        for j in 0..p { unsafe { *w.get_unchecked_mut(j) += d * *xi.get_unchecked(j); } }
                        if self.fit_intercept { b += d; }
                    }
                }

                let pred2 = dot(xi, &w) + b;

                {
                    let g = shuf_y[si] - pred2 + eps;
                    let ai = an[oi];
                    let pg = if ai <= 0.0 { g.min(0.0) } else if ai >= c { g.max(0.0) } else { g };
                    let a = pg.abs();
                    if a > max_pv { max_pv = a; }
                    if a > 1e-12 {
                        let na = (ai - g / shuf_qd[si]).clamp(0.0, c);
                        let d = na - ai;
                        an[oi] = na;
                        for j in 0..p { unsafe { *w.get_unchecked_mut(j) -= d * *xi.get_unchecked(j); } }
                        if self.fit_intercept { b -= d; }
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

impl crate::ml::MlClassifier for LinearSVC {
    fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) { self.fit(x, n, p, y); }
    fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> { self.predict(x, n, p) }
}

impl crate::ml::MlRegressor for LinearSVR {
    fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) { self.fit(x, n, p, y); }
    fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> { self.predict(x, n, p) }
}

fn train_linear_svm(x: &[f64], n: usize, p: usize, y: &[f64], c: f64, max_iter: usize, tol: f64, fit_intercept: bool, parallel_inner: bool) -> (Vec<f64>, f64) {
    let pb = if fit_intercept { p + 1 } else { p };
    let mut w = vec![0.0; pb];
    let mut gnorm_init = 0.0f64;

    for iter in 0..max_iter.min(100) {
        let (mut grad, mut hess) = if parallel_inner && n >= 4096 {
            let ww = &w;
            (0..n).into_par_iter().fold(
                || (vec![0.0; pb], vec![0.0; pb * pb]),
                |(mut g, mut h), i| {
                    let xi = &x[i * p..(i + 1) * p];
                    let yi = y[i];
                    let mut dp = if fit_intercept { ww[p] } else { 0.0 };
                    for j in 0..p { dp += ww[j] * xi[j]; }
                    let m = yi * dp;
                    if m < 1.0 {
                        let loss = 1.0 - m;
                        let gc = -2.0 * c * loss * yi;
                        for j in 0..p { g[j] += gc * xi[j]; }
                        if fit_intercept { g[p] += gc; }
                        let hc = 2.0 * c;
                        for j in 0..p {
                            let xj = xi[j] * hc;
                            for k in j..p { h[j * pb + k] += xj * xi[k]; }
                            if fit_intercept { h[j * pb + p] += xj; }
                        }
                        if fit_intercept { h[p * pb + p] += hc; }
                    }
                    (g, h)
                },
            ).reduce(
                || (vec![0.0; pb], vec![0.0; pb * pb]),
                |(mut g1, mut h1), (g2, h2)| {
                    for j in 0..pb { g1[j] += g2[j]; }
                    for j in 0..(pb * pb) { h1[j] += h2[j]; }
                    (g1, h1)
                },
            )
        } else {
            let mut g = vec![0.0; pb];
            let mut h = vec![0.0; pb * pb];
            for i in 0..n {
                let xi = &x[i * p..(i + 1) * p];
                let yi = y[i];
                let mut dp = if fit_intercept { w[p] } else { 0.0 };
                for j in 0..p { dp += w[j] * xi[j]; }
                let m = yi * dp;
                if m < 1.0 {
                    let loss = 1.0 - m;
                    let gc = -2.0 * c * loss * yi;
                    for j in 0..p { g[j] += gc * xi[j]; }
                    if fit_intercept { g[p] += gc; }
                    let hc = 2.0 * c;
                    for j in 0..p {
                        let xj = xi[j] * hc;
                        for k in j..p { h[j * pb + k] += xj * xi[k]; }
                        if fit_intercept { h[j * pb + p] += xj; }
                    }
                    if fit_intercept { h[p * pb + p] += hc; }
                }
            }
            (g, h)
        };

        for j in 0..p {
            grad[j] += w[j];
            hess[j * pb + j] += 1.0;
        }
        if fit_intercept { hess[p * pb + p] += 1e-8; }
        for j in 0..pb { for k in (j + 1)..pb { hess[k * pb + j] = hess[j * pb + k]; } }

        let gnorm: f64 = grad.iter().map(|g| g * g).sum::<f64>().sqrt();
        if iter == 0 { gnorm_init = gnorm; }
        if gnorm < tol * gnorm_init.max(1.0) { break; }

        let neg_grad: Vec<f64> = grad.iter().map(|g| -g).collect();
        match solve_spd(&hess, pb, &neg_grad) {
            Some(d) => { for j in 0..pb { w[j] += d[j]; } }
            None => break,
        }
    }

    let b = if fit_intercept { let bv = w[p]; w.truncate(p); bv } else { w.truncate(p); 0.0 };
    (w, b)
}


