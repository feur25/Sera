use crate::ml::linalg::{dot, sigmoid, col_means, col_std};
use rayon::prelude::*;

pub struct SGDClassifier {
    pub coef: Vec<f64>,
    pub intercept: f64,
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

impl SGDClassifier {
    pub fn new(loss: SGDLoss, alpha: f64, max_iter: usize, tol: f64, learning_rate: f64, fit_intercept: bool) -> Self {
        Self {
            coef: Vec::new(), intercept: 0.0, alpha, max_iter, tol,
            loss, learning_rate, fit_intercept, n_iter: 0, classes: Vec::new(),
        }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) {
        let mut classes: Vec<i32> = y.to_vec();
        classes.sort_unstable();
        classes.dedup();
        self.classes = classes;

        let yf: Vec<f64> = y.iter().map(|&v| if v == self.classes[self.classes.len() - 1] { 1.0 } else { -1.0 }).collect();
        let mut w = vec![0.0; p];
        let mut b = 0.0;
        let mut rng = 0xDEADBEEFCAFEu64;
        let alpha_eta0 = self.alpha * self.learning_rate;
        let mut global_t = 0usize;
        let mut best_loss = f64::MAX;
        let mut no_change = 0usize;
        let use_par = n >= 8192;
        let block_size = 4096usize.min(n);
        let n_blocks = (n + block_size - 1) / block_size;
        let mut block_order: Vec<usize> = (0..n_blocks).collect();
        let loss_fn = self.loss;
        let alpha = self.alpha;

        for epoch in 0..self.max_iter {
            for i in (1..n_blocks).rev() {
                rng ^= rng << 13; rng ^= rng >> 7; rng ^= rng << 17;
                let j = rng as usize % (i + 1);
                block_order.swap(i, j);
            }

            let eta = self.learning_rate / (1.0 + alpha_eta0 * global_t as f64);
            global_t += n;
            let mut total_loss = 0.0;

            if use_par {
                for &bi in &block_order {
                    let start = bi * block_size;
                    let end = (start + block_size).min(n);
                    let bs = end - start;
                    let w_ref = &w;
                    let b_val = b;
                    let chunk_sz = 256usize.max(bs / (rayon::current_num_threads() * 2));
                    let (gw, gb, bloss) = (start..end).into_par_iter()
                        .with_min_len(chunk_sz)
                        .fold(|| (vec![0.0; p], 0.0f64, 0.0f64),
                            |(mut gw, mut gb, mut loss), idx| {
                                let row = &x[idx * p..(idx + 1) * p];
                                let yi = yf[idx];
                                let z = dot(row, w_ref) + b_val;
                                let (dl, lv) = match loss_fn {
                                    SGDLoss::Hinge => { let m = yi * z; if m < 1.0 { (-yi, (1.0 - m).max(0.0)) } else { (0.0, 0.0) } }
                                    SGDLoss::Log => { let p = sigmoid(yi * z); (-yi * (1.0 - p), -(p.max(1e-15).ln())) }
                                    SGDLoss::ModifiedHuber => { let m = yi * z; if m < -1.0 { (-4.0 * yi, (1.0 - m) * (1.0 - m)) } else if m < 1.0 { (-2.0 * yi * (1.0 - m), (1.0 - m) * (1.0 - m)) } else { (0.0, 0.0) } }
                                    SGDLoss::SquaredHinge => { let m = yi * z; if m < 1.0 { (-2.0 * yi * (1.0 - m), (1.0 - m) * (1.0 - m)) } else { (0.0, 0.0) } }
                                };
                                for j in 0..p { gw[j] += dl * row[j]; }
                                gb += dl;
                                loss += lv;
                                (gw, gb, loss)
                            })
                        .reduce(|| (vec![0.0; p], 0.0, 0.0),
                            |(mut a, ab, al), (bv, bb, bl)| {
                                for j in 0..p { a[j] += bv[j]; }
                                (a, ab + bb, al + bl)
                            });
                    total_loss += bloss;
                    let inv_bs = eta / bs as f64;
                    for j in 0..p { w[j] -= inv_bs * gw[j] + eta * alpha * w[j]; }
                    if self.fit_intercept { b -= inv_bs * gb; }
                }
            } else {
                let mut indices: Vec<usize> = (0..n).collect();
                for i in (1..n).rev() {
                    rng ^= rng << 13; rng ^= rng >> 7; rng ^= rng << 17;
                    let j = rng as usize % (i + 1);
                    indices.swap(i, j);
                }
                for &idx in &indices {
                    let row = &x[idx * p..(idx + 1) * p];
                    let yi = yf[idx];
                    let z = dot(row, &w) + b;
                    let (dloss, loss_val) = match loss_fn {
                        SGDLoss::Hinge => { let m = yi * z; if m < 1.0 { (-yi, (1.0 - m).max(0.0)) } else { (0.0, 0.0) } }
                        SGDLoss::Log => { let p = sigmoid(yi * z); (-yi * (1.0 - p), -(p.max(1e-15).ln())) }
                        SGDLoss::ModifiedHuber => { let m = yi * z; if m < -1.0 { (-4.0 * yi, (1.0 - m) * (1.0 - m)) } else if m < 1.0 { (-2.0 * yi * (1.0 - m), (1.0 - m) * (1.0 - m)) } else { (0.0, 0.0) } }
                        SGDLoss::SquaredHinge => { let m = yi * z; if m < 1.0 { (-2.0 * yi * (1.0 - m), (1.0 - m) * (1.0 - m)) } else { (0.0, 0.0) } }
                    };
                    total_loss += loss_val;
                    for j in 0..p { w[j] -= eta * (dloss * row[j] + alpha * w[j]); }
                    if self.fit_intercept { b -= eta * dloss; }
                }
            }

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
        }

        self.coef = w;
        self.intercept = b;
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> {
        let pos = self.classes[self.classes.len() - 1];
        let neg = self.classes[0];
        let coef = &self.coef;
        let b = self.intercept;
        if n >= 512 {
            (0..n).into_par_iter().map(|i| {
                if dot(&x[i * p..(i + 1) * p], coef) + b >= 0.0 { pos } else { neg }
            }).collect()
        } else {
            (0..n).map(|i| {
                if dot(&x[i * p..(i + 1) * p], coef) + b >= 0.0 { pos } else { neg }
            }).collect()
        }
    }

    pub fn decision_function(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let coef = &self.coef;
        let b = self.intercept;
        if n >= 512 {
            (0..n).into_par_iter().map(|i| dot(&x[i * p..(i + 1) * p], coef) + b).collect()
        } else {
            (0..n).map(|i| dot(&x[i * p..(i + 1) * p], coef) + b).collect()
        }
    }
}

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
        let mut w = vec![0.0; p];
        let mut b = 0.0;
        let mut rng = 0xDEADBEEFCAFEu64;
        let alpha_eta0 = self.alpha * self.learning_rate;
        let mut global_t = 0usize;
        let mut best_loss = f64::MAX;
        let mut no_change = 0usize;
        let use_par = n >= 8192;
        let block_size = 4096usize.min(n);
        let n_blocks = (n + block_size - 1) / block_size;
        let mut block_order: Vec<usize> = (0..n_blocks).collect();
        let alpha = self.alpha;
        let loss_fn = self.loss;
        let epsilon = self.epsilon;

        for epoch in 0..self.max_iter {
            for i in (1..n_blocks).rev() {
                rng ^= rng << 13; rng ^= rng >> 7; rng ^= rng << 17;
                let j = rng as usize % (i + 1);
                block_order.swap(i, j);
            }

            let eta = self.learning_rate / (1.0 + alpha_eta0 * global_t as f64);
            global_t += n;
            let mut total_loss = 0.0;

            if use_par {
                for &bi in &block_order {
                    let start = bi * block_size;
                    let end = (start + block_size).min(n);
                    let bs = end - start;
                    let w_ref = &w;
                    let b_val = b;
                    let chunk_sz = 256usize.max(bs / (rayon::current_num_threads() * 2));
                    let (gw, gb, bloss) = (start..end).into_par_iter()
                        .with_min_len(chunk_sz)
                        .fold(|| (vec![0.0; p], 0.0f64, 0.0f64),
                            |(mut gw, mut gb, mut loss), idx| {
                                let row = &x[idx * p..(idx + 1) * p];
                                let pred = dot(row, w_ref) + b_val;
                                let err = pred - y[idx];
                                let (grad, lv) = match loss_fn {
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
                                for j in 0..p { gw[j] += grad * row[j]; }
                                gb += grad;
                                loss += lv;
                                (gw, gb, loss)
                            })
                        .reduce(|| (vec![0.0; p], 0.0, 0.0),
                            |(mut a, ab, al), (bv, bb, bl)| {
                                for j in 0..p { a[j] += bv[j]; }
                                (a, ab + bb, al + bl)
                            });
                    total_loss += bloss;
                    let inv_bs = eta / bs as f64;
                    for j in 0..p { w[j] -= inv_bs * gw[j] + eta * alpha * w[j]; }
                    if self.fit_intercept { b -= inv_bs * gb; }
                }
            } else {
                let mut indices: Vec<usize> = (0..n).collect();
                for i in (1..n).rev() {
                    rng ^= rng << 13; rng ^= rng >> 7; rng ^= rng << 17;
                    let j = rng as usize % (i + 1);
                    indices.swap(i, j);
                }
                for &idx in &indices {
                    let row = &x[idx * p..(idx + 1) * p];
                    let pred = dot(row, &w) + b;
                    let err = pred - y[idx];
                    let (grad, lv) = match loss_fn {
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
                    total_loss += lv;
                    for j in 0..p { w[j] -= eta * (grad * row[j] + alpha * w[j]); }
                    if self.fit_intercept { b -= eta * grad; }
                }
            }

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
        }

        self.coef = w;
        self.intercept = b;
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let coef = &self.coef;
        let b = self.intercept;
        if n >= 512 {
            (0..n).into_par_iter().map(|i| dot(&x[i * p..(i + 1) * p], coef) + b).collect()
        } else {
            (0..n).map(|i| dot(&x[i * p..(i + 1) * p], coef) + b).collect()
        }
    }
}
