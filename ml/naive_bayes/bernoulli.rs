use rayon::prelude::*;

pub struct BernoulliNB {
    pub classes: Vec<i32>,
    log_priors: Vec<f64>,
    log_probs: Vec<f64>,
    log_neg_probs: Vec<f64>,
    pub alpha: f64,
    pub binarize: f64,
    p: usize,
}

impl BernoulliNB {
    pub fn new(alpha: f64, binarize: f64) -> Self {
        Self {
            classes: Vec::new(), log_priors: Vec::new(), log_probs: Vec::new(),
            log_neg_probs: Vec::new(), alpha, binarize, p: 0,
        }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) {
        self.p = p;
        let mut cls = y.to_vec();
        cls.sort_unstable();
        cls.dedup();
        self.classes = cls;
        let k = self.classes.len();
        let kp = k * p;
        let binarize = self.binarize;

        let cmin = *self.classes.iter().min().unwrap();
        let cmax = *self.classes.iter().max().unwrap();
        let crange = (cmax - cmin + 1) as usize;
        let mut cmap = vec![0u8; crange];
        for (i, &c) in self.classes.iter().enumerate() { cmap[(c - cmin) as usize] = i as u8; }
        let class_map: Vec<usize> = y.iter().map(|&v| cmap[(v - cmin) as usize] as usize).collect();

        let chunk = 4096usize;
        let (feature_counts_f, class_counts_v) = if n >= 8192 {
            let nc = (n + chunk - 1) / chunk;
            (0..nc).into_par_iter().fold(
                || (vec![0.0f64; kp], vec![0.0f64; k]),
                |(mut fc, mut cc), ci| {
                    let start = ci * chunk;
                    let end = (start + chunk).min(n);
                    for i in start..end {
                        let c = class_map[i];
                        cc[c] += 1.0;
                        for j in 0..p {
                            if x[i * p + j] > binarize { fc[c * p + j] += 1.0; }
                        }
                    }
                    (fc, cc)
                }
            ).reduce(
                || (vec![0.0f64; kp], vec![0.0f64; k]),
                |(mut a, mut b), (a2, b2)| {
                    for i in 0..kp { a[i] += a2[i]; }
                    for i in 0..k { b[i] += b2[i]; }
                    (a, b)
                }
            )
        } else {
            let mut fc = vec![0.0f64; kp];
            let mut cc = vec![0.0f64; k];
            for i in 0..n {
                let c = class_map[i];
                cc[c] += 1.0;
                for j in 0..p { if x[i * p + j] > binarize { fc[c * p + j] += 1.0; } }
            }
            (fc, cc)
        };

        let feature_counts = feature_counts_f;
        let class_counts = class_counts_v;

        self.log_priors = class_counts.iter().map(|&c| (c / n as f64).ln()).collect();

        self.log_probs = vec![0.0; k * p];
        self.log_neg_probs = vec![0.0; k * p];
        for c in 0..k {
            for j in 0..p {
                let prob = (feature_counts[c * p + j] + self.alpha) / (class_counts[c] + 2.0 * self.alpha);
                self.log_probs[c * p + j] = prob.ln();
                self.log_neg_probs[c * p + j] = (1.0 - prob).ln();
            }
        }
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> {
        let predict_one = |i: usize| -> i32 {
            let xi = &x[i * p..(i + 1) * p];
            let mut best = 0;
            let mut best_s = f64::NEG_INFINITY;
            for c in 0..self.classes.len() {
                let mut s = self.log_priors[c];
                for j in 0..p {
                    if xi[j] > self.binarize { s += self.log_probs[c * p + j]; }
                    else { s += self.log_neg_probs[c * p + j]; }
                }
                if s > best_s { best_s = s; best = c; }
            }
            self.classes[best]
        };
        if n >= 512 {
            (0..n).into_par_iter().map(predict_one).collect()
        } else {
            (0..n).map(predict_one).collect()
        }
    }

    pub fn predict_proba(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let k = self.classes.len();
        let compute_row = |i: usize| -> Vec<f64> {
            let xi = &x[i * p..(i + 1) * p];
            let scores: Vec<f64> = (0..k).map(|c| {
                let mut s = self.log_priors[c];
                for j in 0..p {
                    if xi[j] > self.binarize { s += self.log_probs[c * p + j]; }
                    else { s += self.log_neg_probs[c * p + j]; }
                }
                s
            }).collect();
            let max_s = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let mut row = vec![0.0; k];
            let mut sum = 0.0;
            for c in 0..k { row[c] = (scores[c] - max_s).exp(); sum += row[c]; }
            let inv = 1.0 / sum;
            for v in row.iter_mut() { *v *= inv; }
            row
        };
        if n >= 512 {
            (0..n).into_par_iter().flat_map(compute_row).collect()
        } else {
            (0..n).flat_map(compute_row).collect()
        }
    }
}

impl crate::ml::MlClassifier for BernoulliNB {
    fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) { self.fit(x, n, p, y); }
    fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> { self.predict(x, n, p) }
}


