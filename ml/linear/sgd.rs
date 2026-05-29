use crate::ml::linalg::{dot, sigmoid};
use rayon::prelude::*;

#[crate::model(category = "Linear", domain = "ml")]
pub struct SGDClassifier {
    pub coef: Vec<f64>,
    pub intercept: f64,
    pub multi_coef: Vec<Vec<f64>>,
    pub multi_intercept: Vec<f64>,
    pub alpha: f64,
    pub max_iter: usize,
    pub tol: f64,
    pub loss: SGDLoss,
    pub learning_rate: f64,
    pub fit_intercept: bool,
    pub n_iter: usize,
    pub classes: Vec<i32>,
}

#[derive(Clone, Copy)]
pub enum SGDLoss {
    Hinge,
    Log,
    ModifiedHuber,
    SquaredHinge,
}

#[derive(Clone, Copy)]
pub enum SGDRegLoss {
    SquaredError,
    Huber,
    EpsilonInsensitive,
}

#[inline(always)]
fn sgd_cls_dloss(loss: SGDLoss, yi: f64, z: f64) -> (f64, f64) {
    match loss {
        SGDLoss::Hinge => { let m = yi * z; if m < 1.0 { (-yi, (1.0 - m).max(0.0)) } else { (0.0, 0.0) } }
        SGDLoss::Log => { let p = sigmoid(yi * z); (-yi * (1.0 - p), -(p.max(1e-15).ln())) }
        SGDLoss::ModifiedHuber => { let m = yi * z; if m < -1.0 { (-4.0 * yi, (1.0 - m) * (1.0 - m)) } else if m < 1.0 { (-2.0 * yi * (1.0 - m), (1.0 - m) * (1.0 - m)) } else { (0.0, 0.0) } }
        SGDLoss::SquaredHinge => { let m = yi * z; if m < 1.0 { (-2.0 * yi * (1.0 - m), (1.0 - m) * (1.0 - m)) } else { (0.0, 0.0) } }
    }
}

fn train_one_binary(
    x: &[f64], n: usize, p: usize, yf: &[f64],
    loss: SGDLoss, alpha: f64, max_iter: usize, tol: f64,
    lr: f64, fit_intercept: bool, seed: u64, checkpoint_id: Option<u64>,
) -> (Vec<f64>, f64, usize) {
    let mut w = vec![0.0; p];
    let mut b = 0.0;

    let start_epoch = if let Some(id) = checkpoint_id {
        if let Some(entry) = crate::ml::cache::checkpoint_load(id) {
            if entry.weights.len() == p + 1 {
                w.copy_from_slice(&entry.weights[..p]);
                b = entry.weights[p];
            }
            entry.iteration.min(max_iter)
        } else { 0 }
    } else { 0 };

    let mut rng = seed;
    let alpha_lr = alpha * lr;
    let mut global_t = start_epoch * n;
    let mut best_loss = f64::MAX;
    let mut no_change = 0usize;
    let mut n_iter = 0;
    let block_sz = 1024usize.min(n);
    let n_blocks = (n + block_sz - 1) / block_sz;
    let mut block_order: Vec<usize> = (0..n_blocks).collect();
    for epoch in start_epoch..max_iter {
        for i in (1..n_blocks).rev() {
            rng ^= rng << 13; rng ^= rng >> 7; rng ^= rng << 17;
            block_order.swap(i, rng as usize % (i + 1));
        }
        let eta = lr / (1.0 + alpha_lr * global_t as f64);
        global_t += n;
        let mut total_loss = 0.0;
        for &bi in &block_order {
            let start = bi * block_sz;
            let end = (start + block_sz).min(n);
            for idx in start..end {
                let row = &x[idx * p..(idx + 1) * p];
                let z = dot(row, &w) + b;
                let (dl, lv) = sgd_cls_dloss(loss, yf[idx], z);
                total_loss += lv;
                for j in 0..p { w[j] -= eta * (dl * row[j] + alpha * w[j]); }
                if fit_intercept { b -= eta * dl; }
            }
        }
        n_iter = epoch + 1;
        let avg = total_loss / n as f64;
        if avg < tol { break; }
        if avg < best_loss - tol { best_loss = avg; no_change = 0; }
        else { no_change += 1; if no_change >= 5 { break; } }
        if let Some(id) = checkpoint_id {
            if (epoch + 1) % 10 == 0 {
                let mut wb = w.clone(); wb.push(b);
                crate::ml::cache::checkpoint_save(id, &wb, epoch + 1);
            }
        }
    }
    if let Some(id) = checkpoint_id { crate::ml::cache::checkpoint_clear(id); }
    (w, b, n_iter)
}

impl SGDClassifier {
    pub fn new(loss: SGDLoss, alpha: f64, max_iter: usize, tol: f64, learning_rate: f64, fit_intercept: bool) -> Self {
        Self {
            coef: Vec::new(), intercept: 0.0,
            multi_coef: Vec::new(), multi_intercept: Vec::new(),
            alpha, max_iter, tol,
            loss, learning_rate, fit_intercept, n_iter: 0, classes: Vec::new(),
        }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) {
        self.fit_resumable(x, n, p, y, None);
    }

    pub fn fit_resumable(&mut self, x: &[f64], n: usize, p: usize, y: &[i32], checkpoint_id: Option<u64>) {
        let mut classes: Vec<i32> = y.to_vec();
        classes.sort_unstable();
        classes.dedup();
        self.classes = classes;

        if self.classes.len() > 2 {
            let k = self.classes.len();
            let loss = self.loss;
            let alpha = self.alpha;
            let mi = self.max_iter;
            let tol = self.tol;
            let lr = self.learning_rate;
            let fi = self.fit_intercept;
            let cls = &self.classes;
            let results: Vec<(Vec<f64>, f64, usize)> = (0..k).into_par_iter().map(|ci| {
                let yf: Vec<f64> = y.iter().map(|&v| if v == cls[ci] { 1.0 } else { -1.0 }).collect();
                train_one_binary(x, n, p, &yf, loss, alpha, mi, tol, lr, fi,
                    0xDEADBEEFCAFEu64.wrapping_add(ci as u64 * 0x9E3779B97F4A7C15), None)
            }).collect();
            self.multi_coef = results.iter().map(|(w, _, _)| w.clone()).collect();
            self.multi_intercept = results.iter().map(|(_, b, _)| *b).collect();
            self.n_iter = results.iter().map(|(_, _, ni)| *ni).max().unwrap_or(0);
            self.coef.clear();
            self.intercept = 0.0;
            return;
        }
        self.multi_coef.clear();
        self.multi_intercept.clear();

        let yf: Vec<f64> = y.iter().map(|&v| if v == self.classes[self.classes.len() - 1] { 1.0 } else { -1.0 }).collect();
        let (w, b, ni) = train_one_binary(x, n, p, &yf, self.loss, self.alpha, self.max_iter, self.tol, self.learning_rate, self.fit_intercept, 0xDEADBEEFCAFEu64, checkpoint_id);
        self.coef = w;
        self.intercept = b;
        self.n_iter = ni;
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> {
        if !self.multi_coef.is_empty() {
            let k = self.classes.len();
            let mc = &self.multi_coef;
            let mi = &self.multi_intercept;
            let cls = &self.classes;
            if n >= 256 {
                (0..n).into_par_iter().map(|i| {
                    let row = &x[i * p..(i + 1) * p];
                    let mut best = f64::NEG_INFINITY;
                    let mut bc = cls[0];
                    for ci in 0..k {
                        let s = dot(row, &mc[ci]) + mi[ci];
                        if s > best { best = s; bc = cls[ci]; }
                    }
                    bc
                }).collect()
            } else {
                (0..n).map(|i| {
                    let row = &x[i * p..(i + 1) * p];
                    let mut best = f64::NEG_INFINITY;
                    let mut bc = cls[0];
                    for ci in 0..k {
                        let s = dot(row, &mc[ci]) + mi[ci];
                        if s > best { best = s; bc = cls[ci]; }
                    }
                    bc
                }).collect()
            }
        } else {
            let pos = self.classes[self.classes.len() - 1];
            let neg = self.classes[0];
            let coef = &self.coef;
            let b = self.intercept;
            if n >= 256 {
                (0..n).into_par_iter().map(|i| {
                    if dot(&x[i * p..(i + 1) * p], coef) + b >= 0.0 { pos } else { neg }
                }).collect()
            } else {
                (0..n).map(|i| {
                    if dot(&x[i * p..(i + 1) * p], coef) + b >= 0.0 { pos } else { neg }
                }).collect()
            }
        }
    }

    pub fn decision_function(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        if !self.multi_coef.is_empty() {
            let k = self.classes.len();
            let mc = &self.multi_coef;
            let mi = &self.multi_intercept;
            let mut out = vec![0.0; n * k];
            for i in 0..n {
                let row = &x[i * p..(i + 1) * p];
                for ci in 0..k { out[i * k + ci] = dot(row, &mc[ci]) + mi[ci]; }
            }
            out
        } else {
            let coef = &self.coef;
            let b = self.intercept;
            if n >= 256 {
                (0..n).into_par_iter().map(|i| dot(&x[i * p..(i + 1) * p], coef) + b).collect()
            } else {
                (0..n).map(|i| dot(&x[i * p..(i + 1) * p], coef) + b).collect()
            }
        }
    }
}

#[crate::model(category = "Linear", domain = "ml")]
pub struct SGDRegressor {
    pub coef: Vec<f64>,
    pub intercept: f64,
    pub alpha: f64,
    pub max_iter: usize,
    pub tol: f64,
    pub learning_rate: f64,
    pub fit_intercept: bool,
    pub n_iter: usize,
    pub loss: SGDRegLoss,
    pub epsilon: f64,
}

impl SGDRegressor {
    pub fn new(alpha: f64, max_iter: usize, tol: f64, learning_rate: f64, fit_intercept: bool) -> Self {
        Self { coef: Vec::new(), intercept: 0.0, alpha, max_iter, tol, learning_rate, fit_intercept, n_iter: 0, loss: SGDRegLoss::SquaredError, epsilon: 0.1 }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) {
        self.fit_resumable(x, n, p, y, None);
    }

    pub fn fit_resumable(&mut self, x: &[f64], n: usize, p: usize, y: &[f64], checkpoint_id: Option<u64>) {
        let mut w = vec![0.0; p];
        let mut b = 0.0;

        let start_epoch = if let Some(id) = checkpoint_id {
            if let Some(entry) = crate::ml::cache::checkpoint_load(id) {
                if entry.weights.len() == p + 1 {
                    w.copy_from_slice(&entry.weights[..p]);
                    b = entry.weights[p];
                }
                entry.iteration.min(self.max_iter)
            } else { 0 }
        } else { 0 };

        let alpha = self.alpha;
        let loss_fn = self.loss;
        let epsilon = self.epsilon;
        let lr = self.learning_rate;
        let fit_intercept = self.fit_intercept;
        let mut best_loss = f64::MAX;
        let mut no_change = 0usize;
        let mut global_t = 1usize + start_epoch * n;

        let use_par = n >= 100_000;
        let batch = 1024usize.min(n);
        let n_batches = (n + batch - 1) / batch;

        for epoch in start_epoch..self.max_iter {
            let eta = lr / (global_t as f64).sqrt().sqrt();
            let inv_n = 1.0 / n as f64;
            let total_loss = if use_par {
                let (gw, gb, lv) = (0..n_batches).into_par_iter().map(|bi| {
                    let s = bi * batch;
                    let e = (s + batch).min(n);
                    let mut lgw = vec![0.0f64; p];
                    let mut lgb = 0.0f64;
                    let mut lloss = 0.0f64;
                    for idx in s..e {
                        let row = &x[idx * p..(idx + 1) * p];
                        let pred = dot(row, &w) + b;
                        let err = pred - y[idx];
                        let (grad, lvi) = match loss_fn {
                            SGDRegLoss::SquaredError => (err, err * err),
                            SGDRegLoss::Huber => {
                                if err.abs() <= epsilon { (err, 0.5 * err * err) }
                                else { (epsilon * err.signum(), epsilon * (err.abs() - 0.5 * epsilon)) }
                            }
                            SGDRegLoss::EpsilonInsensitive => {
                                let a = err.abs();
                                if a <= epsilon { (0.0, 0.0) }
                                else { (err.signum(), a - epsilon) }
                            }
                        };
                        lloss += lvi;
                        for j in 0..p { lgw[j] += grad * row[j]; }
                        lgb += grad;
                    }
                    (lgw, lgb, lloss)
                }).reduce(|| (vec![0.0; p], 0.0, 0.0), |mut a, b2| {
                    for j in 0..p { a.0[j] += b2.0[j]; }
                    a.1 += b2.1; a.2 += b2.2;
                    a
                });
                for j in 0..p { w[j] -= eta * (gw[j] * inv_n + alpha * w[j]); }
                if fit_intercept { b -= eta * gb * inv_n; }
                lv
            } else {
                let mut lloss = 0.0f64;
                let mut t = global_t;
                for idx in 0..n {
                    let row = unsafe { x.get_unchecked(idx * p..(idx + 1) * p) };
                    let mut pred = b;
                    for j in 0..p { pred += unsafe { *row.get_unchecked(j) * *w.get_unchecked(j) }; }
                    let err = pred - unsafe { *y.get_unchecked(idx) };
                    let (grad, lvi) = match loss_fn {
                        SGDRegLoss::SquaredError => (err, err * err),
                        SGDRegLoss::Huber => {
                            if err.abs() <= epsilon { (err, 0.5 * err * err) }
                            else { (epsilon * err.signum(), epsilon * (err.abs() - 0.5 * epsilon)) }
                        }
                        SGDRegLoss::EpsilonInsensitive => {
                            let a = err.abs();
                            if a <= epsilon { (0.0, 0.0) }
                            else { (err.signum(), a - epsilon) }
                        }
                    };
                    lloss += lvi;
                    let eta_t = lr / (t as f64).sqrt().sqrt();
                    let shrink = 1.0 - eta_t * alpha;
                    for j in 0..p {
                        unsafe {
                            let wj = w.get_unchecked_mut(j);
                            *wj = *wj * shrink - eta_t * grad * *row.get_unchecked(j);
                        }
                    }
                    if fit_intercept { b -= eta_t * grad; }
                    t += 1;
                }
                let _ = eta;
                let _ = inv_n;
                lloss
            };
            global_t += n;

            self.n_iter = epoch + 1;
            let avg_loss = total_loss / n as f64;
            if avg_loss < self.tol { break; }
            if avg_loss < best_loss - self.tol {
                best_loss = avg_loss;
                no_change = 0;
            } else {
                no_change += 1;
                if no_change >= 5 { break; }
            }
            if let Some(id) = checkpoint_id {
                if (epoch + 1) % 10 == 0 {
                    let mut wb = w.clone(); wb.push(b);
                    crate::ml::cache::checkpoint_save(id, &wb, epoch + 1);
                }
            }
        }
        if let Some(id) = checkpoint_id { crate::ml::cache::checkpoint_clear(id); }

        self.coef = w;
        self.intercept = b;
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let coef = &self.coef;
        let b = self.intercept;
        if n == 0 || p == 0 || coef.is_empty() { return vec![b; n]; }
        if n * p >= 8192 {
            let mut out = vec![0.0f64; n];
            unsafe {
                matrixmultiply::dgemm(
                    n, p, 1,
                    1.0,
                    x.as_ptr(), p as isize, 1,
                    coef.as_ptr(), 1, 1,
                    0.0,
                    out.as_mut_ptr(), 1, 1,
                );
            }
            if b != 0.0 { for v in out.iter_mut() { *v += b; } }
            out
        } else {
            (0..n).map(|i| dot(&x[i * p..(i + 1) * p], coef) + b).collect()
        }
    }
}

impl crate::ml::MlClassifier for SGDClassifier {
    fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) { self.fit(x, n, p, y); }
    fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> { self.predict(x, n, p) }
}

impl crate::ml::MlRegressor for SGDRegressor {
    fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) { self.fit(x, n, p, y); }
    fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> { self.predict(x, n, p) }
}


