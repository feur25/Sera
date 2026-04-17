use rayon::prelude::*;
use std::cell::RefCell;

thread_local! {
    static KNN_BUF: RefCell<Vec<(f64, u32)>> = RefCell::new(Vec::new());
}

#[inline(always)]
fn dist_sq(a: &[f64], b: &[f64], p: usize) -> f64 {
    let (mut s0, mut s1, mut s2, mut s3) = (0.0, 0.0, 0.0, 0.0);
    let mut i = 0;
    while i + 4 <= p {
        let d0 = unsafe { *a.get_unchecked(i) - *b.get_unchecked(i) };
        let d1 = unsafe { *a.get_unchecked(i+1) - *b.get_unchecked(i+1) };
        let d2 = unsafe { *a.get_unchecked(i+2) - *b.get_unchecked(i+2) };
        let d3 = unsafe { *a.get_unchecked(i+3) - *b.get_unchecked(i+3) };
        s0 += d0*d0; s1 += d1*d1; s2 += d2*d2; s3 += d3*d3;
        i += 4;
    }
    while i < p { let d = unsafe { *a.get_unchecked(i) - *b.get_unchecked(i) }; s0 += d*d; i += 1; }
    s0 + s1 + s2 + s3
}

#[derive(Clone, Copy)]
pub enum KnnWeights { Uniform, Distance }

pub struct KNeighborsClassifier {
    pub k: usize,
    pub weights: KnnWeights,
    data: Vec<f64>,
    labels: Vec<i32>,
    label_idx: Vec<u8>,
    n: usize,
    p: usize,
    pub classes: Vec<i32>,
}

impl KNeighborsClassifier {
    pub fn new(k: usize, weights: KnnWeights) -> Self {
        Self { k, weights, data: Vec::new(), labels: Vec::new(), label_idx: Vec::new(), n: 0, p: 0, classes: Vec::new() }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) {
        self.data = x.to_vec();
        self.labels = y.to_vec();
        self.n = n;
        self.p = p;
        let mut c = y.to_vec();
        c.sort_unstable();
        c.dedup();
        self.classes = c;
        self.label_idx = y.iter().map(|&v| self.classes.iter().position(|&c| c == v).unwrap() as u8).collect();
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> {
        let k = self.k;
        let p_ = self.p;
        let n_train = self.n;
        let nc = self.classes.len();
        let use_dist = matches!(self.weights, KnnWeights::Distance);
        let predict_one = |i: usize| -> i32 {
            let xi = &x[i * p..(i + 1) * p];
            KNN_BUF.with(|buf| {
                let mut buf = buf.borrow_mut();
                if buf.len() < n_train { buf.resize(n_train, (0.0, 0)); }
                for j in 0..n_train {
                    let d = dist_sq(xi, unsafe { self.data.get_unchecked(j * p_..(j + 1) * p_) }, p_);
                    buf[j] = (d, j as u32);
                }
                buf[..n_train].select_nth_unstable_by(k - 1, |a, b| a.0.partial_cmp(&b.0).unwrap());
                if use_dist {
                    let mut wts = [0.0f64; 256];
                    for t in 0..k {
                        let d = buf[t].0.sqrt().max(1e-10);
                        let ci = unsafe { *self.label_idx.get_unchecked(buf[t].1 as usize) } as usize;
                        unsafe { *wts.get_unchecked_mut(ci) += 1.0 / d; }
                    }
                    let mut best = 0;
                    let mut best_w = 0.0;
                    for c in 0..nc { if wts[c] > best_w { best_w = wts[c]; best = c; } }
                    self.classes[best]
                } else {
                    let mut counts = [0u32; 256];
                    for t in 0..k { unsafe { *counts.get_unchecked_mut(*self.label_idx.get_unchecked(buf[t].1 as usize) as usize) += 1; } }
                    let mut best = 0;
                    let mut best_c = 0u32;
                    for c in 0..nc { if counts[c] > best_c { best_c = counts[c]; best = c; } }
                    self.classes[best]
                }
            })
        };
        if n >= 64 {
            (0..n).into_par_iter().map(predict_one).collect()
        } else {
            (0..n).map(predict_one).collect()
        }
    }

    pub fn predict_proba(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let k = self.k;
        let p_ = self.p;
        let n_train = self.n;
        let nc = self.classes.len();
        let inv = 1.0 / k as f64;
        let use_dist = matches!(self.weights, KnnWeights::Distance);
        let proba_one = |i: usize| -> Vec<f64> {
            let xi = &x[i * p..(i + 1) * p];
            KNN_BUF.with(|buf| {
                let mut buf = buf.borrow_mut();
                if buf.len() < n_train { buf.resize(n_train, (0.0, 0)); }
                for j in 0..n_train {
                    let d = dist_sq(xi, unsafe { self.data.get_unchecked(j * p_..(j + 1) * p_) }, p_);
                    buf[j] = (d, j as u32);
                }
                buf[..n_train].select_nth_unstable_by(k - 1, |a, b| a.0.partial_cmp(&b.0).unwrap());
                let mut probs = vec![0.0; nc];
                if use_dist {
                    let mut wsum = 0.0;
                    for t in 0..k {
                        let d = buf[t].0.sqrt().max(1e-10);
                        let w = 1.0 / d;
                        probs[self.label_idx[buf[t].1 as usize] as usize] += w;
                        wsum += w;
                    }
                    let inv_w = 1.0 / wsum;
                    for v in probs.iter_mut() { *v *= inv_w; }
                } else {
                    for t in 0..k { probs[self.label_idx[buf[t].1 as usize] as usize] += inv; }
                }
                probs
            })
        };
        if n >= 64 {
            let results: Vec<Vec<f64>> = (0..n).into_par_iter().map(proba_one).collect();
            let mut out = vec![0.0; n * nc];
            for (i, probs) in results.into_iter().enumerate() {
                out[i * nc..(i + 1) * nc].copy_from_slice(&probs);
            }
            out
        } else {
            let mut out = vec![0.0; n * nc];
            for i in 0..n {
                let probs = proba_one(i);
                out[i * nc..(i + 1) * nc].copy_from_slice(&probs);
            }
            out
        }
    }
}

pub struct KNeighborsRegressor {
    pub k: usize,
    pub weights: KnnWeights,
    data: Vec<f64>,
    targets: Vec<f64>,
    n: usize,
    p: usize,
}

impl KNeighborsRegressor {
    pub fn new(k: usize, weights: KnnWeights) -> Self {
        Self { k, weights, data: Vec::new(), targets: Vec::new(), n: 0, p: 0 }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) {
        self.data = x.to_vec();
        self.targets = y.to_vec();
        self.n = n;
        self.p = p;
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let k = self.k;
        let p_ = self.p;
        let n_train = self.n;
        let use_dist = matches!(self.weights, KnnWeights::Distance);
        let predict_one = |i: usize| -> f64 {
            let xi = &x[i * p..(i + 1) * p];
            KNN_BUF.with(|buf| {
                let mut buf = buf.borrow_mut();
                if buf.len() < n_train { buf.resize(n_train, (0.0, 0)); }
                for j in 0..n_train {
                    let d = dist_sq(xi, unsafe { self.data.get_unchecked(j * p_..(j + 1) * p_) }, p_);
                    buf[j] = (d, j as u32);
                }
                let k_ = k.min(n_train);
                buf[..n_train].select_nth_unstable_by(k_ - 1, |a, b| a.0.partial_cmp(&b.0).unwrap());
                if use_dist {
                    let mut wsum = 0.0;
                    let mut vsum = 0.0;
                    for t in 0..k_ {
                        let d = buf[t].0.sqrt().max(1e-10);
                        let w = 1.0 / d;
                        vsum += w * self.targets[buf[t].1 as usize];
                        wsum += w;
                    }
                    vsum / wsum
                } else {
                    let mut sum = 0.0;
                    for t in 0..k_ { sum += self.targets[buf[t].1 as usize]; }
                    sum / k_ as f64
                }
            })
        };
        if n >= 64 {
            (0..n).into_par_iter().map(predict_one).collect()
        } else {
            (0..n).map(predict_one).collect()
        }
    }
}

pub struct NearestCentroid {
    centroids: Vec<f64>,
    pub classes: Vec<i32>,
    p: usize,
}

impl NearestCentroid {
    pub fn new() -> Self {
        Self { centroids: Vec::new(), classes: Vec::new(), p: 0 }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) {
        self.p = p;
        let mut cls = y.to_vec();
        cls.sort_unstable();
        cls.dedup();
        self.classes = cls;
        let k = self.classes.len();
        self.centroids = vec![0.0; k * p];
        let mut counts = vec![0usize; k];
        for i in 0..n {
            let pos = self.classes.iter().position(|&c| c == y[i]).unwrap();
            counts[pos] += 1;
            for j in 0..p { self.centroids[pos * p + j] += x[i * p + j]; }
        }
        for c in 0..k {
            if counts[c] > 0 {
                let inv = 1.0 / counts[c] as f64;
                for j in 0..p { self.centroids[c * p + j] *= inv; }
            }
        }
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> {
        let predict_one = |i: usize| -> i32 {
            let xi = &x[i * p..(i + 1) * p];
            let mut best = 0;
            let mut best_d = f64::MAX;
            for (c, _) in self.classes.iter().enumerate() {
                let d = dist_sq(xi, &self.centroids[c * p..(c + 1) * p], p);
                if d < best_d { best_d = d; best = c; }
            }
            self.classes[best]
        };
        if n >= 256 {
            (0..n).into_par_iter().map(predict_one).collect()
        } else {
            (0..n).map(predict_one).collect()
        }
    }
}
