use crate::ml::tree::decision_tree::{DecisionTreeClassifier, DecisionTreeRegressor, compute_bins, bin_data_with_edges, BinInfo};
use crate::ml::linalg::{discover_classes, weighted_bootstrap};
use rayon::prelude::*;

#[inline(always)]
fn gini_w(cnts: &[f64], total: f64) -> f64 {
    if total <= 0.0 { return 0.0; }
    let inv = 1.0 / total;
    let mut s = 1.0;
    for &c in cnts { let p = c * inv; s -= p * p; }
    s
}

struct Stump {
    feature: usize,
    threshold: f64,
    left_cls: i32,
    right_cls: i32,
}

impl Stump {
    #[inline(always)]
    fn predict_single(&self, row: &[f64]) -> i32 {
        if row[self.feature] <= self.threshold { self.left_cls } else { self.right_cls }
    }
}

enum WeakLearner {
    Stump(Stump),
    Tree(DecisionTreeClassifier),
}

impl WeakLearner {
    #[inline(always)]
    fn predict_single(&self, row: &[f64]) -> i32 {
        match self {
            WeakLearner::Stump(s) => s.predict_single(row),
            WeakLearner::Tree(t) => t.predict_single(row),
        }
    }
}

fn scan_stump_feature(
    feat: usize, x: &[f64], n: usize, p: usize, y_cls: &[u8], nc: usize,
    weights: &[f64], sorted_idx: &[usize], total_cwts: &[f64], total_w: f64, parent_gini: f64,
) -> (f64, f64, usize, usize) {
    let mut lwts = vec![0.0f64; nc];
    let mut rwts = vec![0.0f64; nc];
    rwts.copy_from_slice(total_cwts);
    let mut wl = 0.0f64;
    let mut bg = -1.0f64;
    let mut bthr = f64::INFINITY;
    let mut blci = 0usize;
    let mut brci = 0usize;
    let mut i = 0usize;
    while i < n {
        let si_i = unsafe { *sorted_idx.get_unchecked(i) };
        let xv = unsafe { *x.get_unchecked(si_i * p + feat) };
        while i < n {
            let si_i = unsafe { *sorted_idx.get_unchecked(i) };
            if unsafe { *x.get_unchecked(si_i * p + feat) } != xv { break; }
            let ci = unsafe { *y_cls.get_unchecked(si_i) as usize };
            let w = unsafe { *weights.get_unchecked(si_i) };
            unsafe { *lwts.get_unchecked_mut(ci) += w; *rwts.get_unchecked_mut(ci) -= w; }
            wl += w;
            i += 1;
        }
        if i >= n { break; }
        let wr = total_w - wl;
        if wl < 1e-15 || wr < 1e-15 { continue; }
        let gl = gini_w(&lwts, wl);
        let gr = gini_w(&rwts, wr);
        let gain = parent_gini - (wl / total_w) * gl - (wr / total_w) * gr;
        if gain > bg {
            bg = gain;
            let next_x = unsafe { *x.get_unchecked(sorted_idx.get_unchecked(i) * p + feat) };
            bthr = (xv + next_x) * 0.5;
            blci = lwts.iter().enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(ci, _)| ci).unwrap_or(0);
            brci = rwts.iter().enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(ci, _)| ci).unwrap_or(0);
        }
    }
    (bg, bthr, blci, brci)
}

fn fit_stump_exact(
    x: &[f64], n: usize, p: usize, y_cls: &[u8], nc: usize,
    classes: &[i32], weights: &[f64], sorted_idx: &[Vec<usize>],
) -> Stump {
    let mut total_cwts = vec![0.0f64; nc];
    let mut total_w = 0.0f64;
    for i in 0..n {
        unsafe {
            *total_cwts.get_unchecked_mut(*y_cls.get_unchecked(i) as usize) += *weights.get_unchecked(i);
            total_w += *weights.get_unchecked(i);
        }
    }
    let parent_gini = gini_w(&total_cwts, total_w);

    let results: Vec<(f64, f64, usize, usize)> = sorted_idx.par_iter().enumerate().map(|(feat, si)| {
        scan_stump_feature(feat, x, n, p, y_cls, nc, weights, si, &total_cwts, total_w, parent_gini)
    }).collect();

    let mut best_gain = -1.0f64;
    let mut best_feat = 0usize;
    let mut best_thr = f64::INFINITY;
    let mut best_left_ci = 0usize;
    let mut best_right_ci = 0usize;
    for (feat, (bg, bthr, blci, brci)) in results.into_iter().enumerate() {
        if bg > best_gain {
            best_gain = bg;
            best_feat = feat;
            best_thr = bthr;
            best_left_ci = blci;
            best_right_ci = brci;
        }
    }

    Stump {
        feature: best_feat,
        threshold: best_thr,
        left_cls: classes[best_left_ci],
        right_cls: classes[best_right_ci],
    }
}

pub struct AdaBoostClassifier {
    pub n_estimators: usize,
    pub learning_rate: f64,
    pub max_depth: usize,
    pub classes: Vec<i32>,
    learners: Vec<WeakLearner>,
    alphas: Vec<f64>,
}

impl AdaBoostClassifier {
    pub fn new(n_estimators: usize, learning_rate: f64, max_depth: usize) -> Self {
        Self {
            n_estimators, learning_rate, max_depth,
            classes: Vec::new(), learners: Vec::new(), alphas: Vec::new(),
        }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) {
        let classes: Vec<i32> = discover_classes(y);
        self.classes = classes.clone();
        let k = classes.len();
        let kf = k as f64;
        let cmin = *classes.iter().min().unwrap_or(&0);
        let cmax = *classes.iter().max().unwrap_or(&0);
        let crange = (cmax - cmin + 1) as usize;
        let mut cmap = vec![0u8; crange];
        for (i, &c) in classes.iter().enumerate() { cmap[(c - cmin) as usize] = i as u8; }
        let y_cls: Vec<u8> = y.iter().map(|&v| cmap[(v - cmin) as usize]).collect();

        let mut weights = vec![1.0f64 / n as f64; n];
        self.learners.clear();
        self.alphas.clear();

        if self.max_depth == 1 {
            let sorted_idx: Vec<Vec<usize>> = (0..p).into_par_iter().map(|j| {
                let mut idx: Vec<usize> = (0..n).collect();
                idx.sort_unstable_by(|&a, &b| {
                    x[a * p + j].partial_cmp(&x[b * p + j]).unwrap_or(std::cmp::Ordering::Equal)
                });
                idx
            }).collect();

            for _ in 0..self.n_estimators {
                let stump = fit_stump_exact(x, n, p, &y_cls, k, &classes, &weights, &sorted_idx);
                let preds: Vec<i32> = (0..n).map(|i| stump.predict_single(&x[i * p..(i + 1) * p])).collect();

                let mut err = 0.0f64;
                for i in 0..n {
                    if preds[i] != y[i] { err += weights[i]; }
                }

                if err >= 1.0 - 1.0 / kf { break; }
                if err <= 1e-15 {
                    self.alphas.push(self.learning_rate * (((1.0 - 1e-15) / 1e-15f64).ln() + (kf - 1.0).ln()));
                    self.learners.push(WeakLearner::Stump(stump));
                    break;
                }

                let alpha = self.learning_rate * (((1.0 - err) / err).ln() + (kf - 1.0).ln());
                if alpha <= 0.0 { break; }
                let exp_alpha = alpha.exp();
                for i in 0..n {
                    if preds[i] != y[i] { weights[i] *= exp_alpha; }
                }
                let wsum: f64 = weights.iter().sum();
                if wsum <= 0.0 { break; }
                let inv_wsum = 1.0 / wsum;
                for w in weights.iter_mut() { *w *= inv_wsum; }

                self.alphas.push(alpha);
                self.learners.push(WeakLearner::Stump(stump));
            }
        } else {
            let master_bins = compute_bins(x, n, p);
            for _ in 0..self.n_estimators {
                let mut tree = DecisionTreeClassifier::new(
                    self.max_depth, 2, 1, None,
                    crate::ml::tree::decision_tree::TreeCriterion::Gini,
                );
                tree.fit_weighted(y, &master_bins, &weights);

                let preds: Vec<i32> = if n >= 256 {
                    (0..n).into_par_iter().map(|i| tree.predict_single(&x[i * p..(i + 1) * p])).collect()
                } else {
                    (0..n).map(|i| tree.predict_single(&x[i * p..(i + 1) * p])).collect()
                };

                let mut err = 0.0f64;
                for i in 0..n { if preds[i] != y[i] { err += weights[i]; } }

                if err >= 1.0 - 1.0 / kf { break; }
                if err <= 1e-15 {
                    self.alphas.push(self.learning_rate * (((1.0 - 1e-15) / 1e-15f64).ln() + (kf - 1.0).ln()));
                    self.learners.push(WeakLearner::Tree(tree));
                    break;
                }

                let alpha = self.learning_rate * (((1.0 - err) / err).ln() + (kf - 1.0).ln());
                if alpha <= 0.0 { break; }
                let exp_alpha = alpha.exp();
                for i in 0..n { if preds[i] != y[i] { weights[i] *= exp_alpha; } }
                let wsum: f64 = weights.iter().sum();
                if wsum <= 0.0 { break; }
                let inv_wsum = 1.0 / wsum;
                for w in weights.iter_mut() { *w *= inv_wsum; }

                self.alphas.push(alpha);
                self.learners.push(WeakLearner::Tree(tree));
            }
        }
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> {
        let k = self.classes.len();
        let learners = &self.learners;
        let alphas = &self.alphas;
        let classes = &self.classes;
        let cmin = *classes.iter().min().unwrap_or(&0);
        let crange = (*classes.iter().max().unwrap_or(&0) - cmin + 1) as usize;
        let mut cmap = vec![k; crange];
        for (i, &c) in classes.iter().enumerate() { cmap[(c - cmin) as usize] = i; }

        let predict_one = |i: usize| -> i32 {
            let row = &x[i * p..(i + 1) * p];
            let mut scores = vec![0.0f64; k];
            for (t, lrn) in learners.iter().enumerate() {
                let pred = lrn.predict_single(row);
                let ci = cmap[((pred - cmin) as usize).min(crange - 1)];
                if ci < k { scores[ci] += alphas[t]; }
            }
            let best = scores.iter().enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(i, _)| i).unwrap_or(0);
            classes[best]
        };

        if n >= 64 {
            (0..n).into_par_iter().map(predict_one).collect()
        } else {
            (0..n).map(predict_one).collect()
        }
    }

    pub fn predict_proba(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let k = self.classes.len();
        let learners = &self.learners;
        let alphas = &self.alphas;
        let classes = &self.classes;
        let cmin = *classes.iter().min().unwrap_or(&0);
        let crange = (*classes.iter().max().unwrap_or(&0) - cmin + 1) as usize;
        let mut cmap = vec![k; crange];
        for (i, &c) in classes.iter().enumerate() { cmap[(c - cmin) as usize] = i; }

        let compute_row = |i: usize| -> Vec<f64> {
            let row_x = &x[i * p..(i + 1) * p];
            let mut scores = vec![0.0f64; k];
            for (t, lrn) in learners.iter().enumerate() {
                let pred = lrn.predict_single(row_x);
                let ci = cmap[((pred - cmin) as usize).min(crange - 1)];
                if ci < k { scores[ci] += alphas[t]; }
            }
            let mx = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let mut sum = 0.0;
            for v in scores.iter_mut() { *v = (*v - mx).exp(); sum += *v; }
            let inv = 1.0 / sum;
            for v in scores.iter_mut() { *v *= inv; }
            scores
        };

        if n >= 64 {
            (0..n).into_par_iter().flat_map(compute_row).collect()
        } else {
            (0..n).flat_map(compute_row).collect()
        }
    }
}

pub struct AdaBoostRegressor {
    pub n_estimators: usize,
    pub learning_rate: f64,
    pub max_depth: usize,
    trees: Vec<DecisionTreeRegressor>,
    weights: Vec<f64>,
}

impl AdaBoostRegressor {
    pub fn new(n_estimators: usize, learning_rate: f64, max_depth: usize) -> Self {
        Self {
            n_estimators, learning_rate, max_depth,
            trees: Vec::new(), weights: Vec::new(),
        }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) {
        let mut sample_weights = vec![1.0 / n as f64; n];
        self.trees.clear();
        self.weights.clear();
        let mut rng = 0xCAFEBABE12345678u64;
        let master_bins_r = compute_bins(x, n, p);

        for _ in 0..self.n_estimators {
            let sample = weighted_bootstrap(n, &sample_weights, &mut rng);
            let sampled_y: Vec<f64> = sample.iter().map(|&i| y[i]).collect();
            let sn = sample.len();
            let mut binned_boot = vec![0u8; sn * p];
            for j in 0..p {
                let src = &master_bins_r.binned[j * n..(j + 1) * n];
                for (k, &bi) in sample.iter().enumerate() {
                    binned_boot[j * sn + k] = unsafe { *src.get_unchecked(bi) };
                }
            }
            let bins = BinInfo { edges: master_bins_r.edges.clone(), n_bins: master_bins_r.n_bins.clone(), binned: binned_boot, p, n: sn };
            let mut tree = DecisionTreeRegressor::new(self.max_depth, 2, 1, None);
            tree.fit_with_bins(&sampled_y, &bins);

            let preds = tree.predict(x, n, p);
            let max_err = (0..n).map(|i| (y[i] - preds[i]).abs()).fold(0.0f64, f64::max);
            if max_err < 1e-15 {
                self.trees.push(tree);
                self.weights.push(1.0);
                break;
            }

            let losses: Vec<f64> = (0..n).map(|i| (y[i] - preds[i]).abs() / max_err).collect();
            let avg_loss: f64 = losses.iter().zip(sample_weights.iter()).map(|(l, w)| l * w).sum();
            if avg_loss >= 0.5 { break; }

            let beta = avg_loss / (1.0 - avg_loss);
            for i in 0..n {
                sample_weights[i] *= beta.powf(self.learning_rate * (1.0 - losses[i]));
            }
            let wsum: f64 = sample_weights.iter().sum();
            for w in sample_weights.iter_mut() { *w /= wsum; }

            let tw = (1.0 / beta).ln();
            self.trees.push(tree);
            self.weights.push(tw);
        }
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        if self.trees.is_empty() { return vec![0.0; n]; }
        let all_preds: Vec<Vec<f64>> = self.trees.iter().map(|t| t.predict(x, n, p)).collect();
        let wsum: f64 = self.weights.iter().sum();
        let inv_wsum = 1.0 / wsum;
        let weights = &self.weights;
        if n >= 64 {
            (0..n).into_par_iter().map(|i| {
                let mut s = 0.0;
                for (t, w) in weights.iter().enumerate() { s += w * all_preds[t][i]; }
                s * inv_wsum
            }).collect()
        } else {
            (0..n).map(|i| {
                let mut s = 0.0;
                for (t, w) in weights.iter().enumerate() { s += w * all_preds[t][i]; }
                s * inv_wsum
            }).collect()
        }
    }
}

impl crate::ml::MlClassifier for AdaBoostClassifier {
    fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) { self.fit(x, n, p, y); }
    fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> { self.predict(x, n, p) }
}

impl crate::ml::MlRegressor for AdaBoostRegressor {
    fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) { self.fit(x, n, p, y); }
    fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> { self.predict(x, n, p) }
}
