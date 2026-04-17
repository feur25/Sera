pub struct MultinomialNB {
    pub classes: Vec<i32>,
    log_priors: Vec<f64>,
    log_probs: Vec<f64>,
    pub alpha: f64,
    p: usize,
}

impl MultinomialNB {
    pub fn new(alpha: f64) -> Self {
        Self { classes: Vec::new(), log_priors: Vec::new(), log_probs: Vec::new(), alpha, p: 0 }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) {
        self.p = p;
        let mut cls = y.to_vec();
        cls.sort_unstable();
        cls.dedup();
        self.classes = cls;
        let k = self.classes.len();

        let mut feature_counts = vec![0.0; k * p];
        let mut class_counts = vec![0.0; k];

        for i in 0..n {
            let c = self.classes.iter().position(|&v| v == y[i]).unwrap();
            class_counts[c] += 1.0;
            for j in 0..p { feature_counts[c * p + j] += x[i * p + j]; }
        }

        self.log_priors = class_counts.iter().map(|&c| (c / n as f64).ln()).collect();

        self.log_probs = vec![0.0; k * p];
        for c in 0..k {
            let total: f64 = (0..p).map(|j| feature_counts[c * p + j]).sum::<f64>() + self.alpha * p as f64;
            for j in 0..p {
                self.log_probs[c * p + j] = ((feature_counts[c * p + j] + self.alpha) / total).ln();
            }
        }
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> {
        (0..n).map(|i| {
            let xi = &x[i * p..(i + 1) * p];
            let mut best = 0;
            let mut best_s = f64::NEG_INFINITY;
            for c in 0..self.classes.len() {
                let mut s = self.log_priors[c];
                for j in 0..p { s += xi[j] * self.log_probs[c * p + j]; }
                if s > best_s { best_s = s; best = c; }
            }
            self.classes[best]
        }).collect()
    }

    pub fn predict_proba(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let k = self.classes.len();
        let mut out = vec![0.0; n * k];
        for i in 0..n {
            let xi = &x[i * p..(i + 1) * p];
            let scores: Vec<f64> = (0..k).map(|c| {
                let mut s = self.log_priors[c];
                for j in 0..p { s += xi[j] * self.log_probs[c * p + j]; }
                s
            }).collect();
            let max_s = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let mut sum = 0.0;
            for c in 0..k { out[i * k + c] = (scores[c] - max_s).exp(); sum += out[i * k + c]; }
            for c in 0..k { out[i * k + c] /= sum; }
        }
        out
    }
}
