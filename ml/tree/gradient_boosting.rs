use crate::ml::tree::decision_tree::{DecisionTreeRegressor, compute_bins};
use rayon::prelude::*;

pub struct GradientBoostingClassifier {
    pub n_estimators: usize,
    pub learning_rate: f64,
    pub max_depth: usize,
    pub min_samples_split: usize,
    pub min_samples_leaf: usize,
    pub classes: Vec<i32>,
    trees: Vec<Vec<DecisionTreeRegressor>>,
    base_scores: Vec<f64>,
}

impl GradientBoostingClassifier {
    pub fn new(n_estimators: usize, learning_rate: f64, max_depth: usize, min_samples_split: usize, min_samples_leaf: usize) -> Self {
        Self {
            n_estimators, learning_rate, max_depth, min_samples_split, min_samples_leaf,
            classes: Vec::new(), trees: Vec::new(), base_scores: Vec::new(),
        }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) {
        let classes = crate::ml::linalg::discover_classes(y);
        self.classes = classes.clone();
        let k = self.classes.len();

        let mut scores = vec![0.0; n * k];
        self.base_scores = vec![0.0; k];
        for &yi in y {
            let pos = self.classes.iter().position(|&c| c == yi).unwrap();
            self.base_scores[pos] += 1.0;
        }
        for j in 0..k {
            self.base_scores[j] = (self.base_scores[j] / n as f64).max(1e-15).ln();
        }
        for i in 0..n {
            for j in 0..k { scores[i * k + j] = self.base_scores[j]; }
        }

        self.trees = vec![Vec::new(); k];
        let classes = &self.classes;

        let bins = compute_bins(x, n, p);
        for _ in 0..self.n_estimators {
            let probs = softmax_rows(&scores, n, k);

            let new_trees: Vec<(DecisionTreeRegressor, Vec<f64>)> = (0..k).into_par_iter().map(|c| {
                let mut residuals = vec![0.0; n];
                let mut hessians = vec![0.0; n];
                for i in 0..n {
                    let target = if y[i] == classes[c] { 1.0 } else { 0.0 };
                    let pc = probs[i * k + c];
                    residuals[i] = target - pc;
                    hessians[i] = (pc * (1.0 - pc)).max(1e-7);
                }
                let mut tree = DecisionTreeRegressor::new(self.max_depth, self.min_samples_split, self.min_samples_leaf, None);
                tree.fit_with_bins(&residuals, &bins);
                tree.update_leaves_newton(x, n, p, &residuals, &hessians);
                let preds = tree.predict(x, n, p);
                (tree, preds)
            }).collect();

            for (c, (tree, preds)) in new_trees.into_iter().enumerate() {
                for i in 0..n {
                    scores[i * k + c] += self.learning_rate * preds[i];
                }
                self.trees[c].push(tree);
            }
        }
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> {
        let scores = self.decision_function(x, n, p);
        let k = self.classes.len();
        (0..n).map(|i| {
            let row = &scores[i * k..(i + 1) * k];
            let best = row.iter().enumerate().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()).map(|(i, _)| i).unwrap_or(0);
            self.classes[best]
        }).collect()
    }

    pub fn decision_function(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let k = self.classes.len();
        let mut scores = vec![0.0; n * k];
        for i in 0..n {
            for j in 0..k { scores[i * k + j] = self.base_scores[j]; }
        }
        for c in 0..k {
            for tree in &self.trees[c] {
                let preds = tree.predict(x, n, p);
                for i in 0..n { scores[i * k + c] += self.learning_rate * preds[i]; }
            }
        }
        scores
    }

    pub fn predict_proba(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let scores = self.decision_function(x, n, p);
        let k = self.classes.len();
        let mut out = scores;
        for i in 0..n {
            let row = &mut out[i * k..(i + 1) * k];
            let mx = row.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let mut sum = 0.0;
            for v in row.iter_mut() {
                *v = (*v - mx).exp();
                sum += *v;
            }
            let inv = 1.0 / sum;
            for v in row.iter_mut() { *v *= inv; }
        }
        out
    }
}

pub struct GradientBoostingRegressor {
    pub n_estimators: usize,
    pub learning_rate: f64,
    pub max_depth: usize,
    pub min_samples_split: usize,
    pub min_samples_leaf: usize,
    trees: Vec<DecisionTreeRegressor>,
    base_score: f64,
}

impl GradientBoostingRegressor {
    pub fn new(n_estimators: usize, learning_rate: f64, max_depth: usize, min_samples_split: usize, min_samples_leaf: usize) -> Self {
        Self {
            n_estimators, learning_rate, max_depth, min_samples_split, min_samples_leaf,
            trees: Vec::new(), base_score: 0.0,
        }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) {
        self.base_score = y.iter().sum::<f64>() / n as f64;
        let mut preds = vec![self.base_score; n];
        self.trees.clear();

        let bins = compute_bins(x, n, p);
        for _ in 0..self.n_estimators {
            let residuals: Vec<f64> = (0..n).map(|i| y[i] - preds[i]).collect();
            let mut tree = DecisionTreeRegressor::new(self.max_depth, self.min_samples_split, self.min_samples_leaf, None);
            tree.fit_with_bins(&residuals, &bins);
            let tp = tree.predict(x, n, p);
            for i in 0..n { preds[i] += self.learning_rate * tp[i]; }
            self.trees.push(tree);
        }
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let lr = self.learning_rate;
        let base = self.base_score;
        let predict_one = |i: usize| -> f64 {
            let xi = &x[i * p..(i + 1) * p];
            let mut s = base;
            for tree in &self.trees { s += lr * tree.predict_single(xi); }
            s
        };
        if n >= 256 {
            (0..n).into_par_iter().map(predict_one).collect()
        } else {
            (0..n).map(predict_one).collect()
        }
    }
}

fn softmax_rows(scores: &[f64], n: usize, k: usize) -> Vec<f64> {
    let mut out = vec![0.0; n * k];
    for i in 0..n {
        let row = &scores[i * k..(i + 1) * k];
        let max_s = row.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let mut sum = 0.0;
        for j in 0..k {
            out[i * k + j] = (row[j] - max_s).exp();
            sum += out[i * k + j];
        }
        for j in 0..k { out[i * k + j] /= sum; }
    }
    out
}
