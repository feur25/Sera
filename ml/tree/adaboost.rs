use crate::ml::tree::decision_tree::{DecisionTreeClassifier, DecisionTreeRegressor, compute_bins, bin_data_with_edges, BinInfo};
use crate::ml::linalg::{discover_classes, weighted_bootstrap};

pub struct AdaBoostClassifier {
    pub n_estimators: usize,
    pub learning_rate: f64,
    pub max_depth: usize,
    pub classes: Vec<i32>,
    trees: Vec<DecisionTreeClassifier>,
    alphas: Vec<f64>,
}

impl AdaBoostClassifier {
    pub fn new(n_estimators: usize, learning_rate: f64, max_depth: usize) -> Self {
        Self {
            n_estimators, learning_rate, max_depth,
            classes: Vec::new(), trees: Vec::new(), alphas: Vec::new(),
        }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) {
        let mut classes: Vec<i32> = discover_classes(y);
        self.classes = classes.clone();
        let k = classes.len();
        let kf = k as f64;
        let eps = 1e-15f64;

        let y_idx: Vec<usize> = y.iter().map(|&v| classes.iter().position(|&c| c == v).unwrap()).collect();
        let mut weights = vec![1.0 / n as f64; n];
        self.trees.clear();
        self.alphas.clear();
        let mut rng = 0x123456789ABCDEFu64;
        let master_bins = compute_bins(x, n, p);

        for _ in 0..self.n_estimators {
            let mut tree = DecisionTreeClassifier::new(
                self.max_depth, 2, 1, None,
                crate::ml::tree::decision_tree::TreeCriterion::Gini,
            );

            let sample = weighted_bootstrap(n, &weights, &mut rng);
            let sampled_x: Vec<f64> = sample.iter().flat_map(|&i| x[i * p..(i + 1) * p].iter().copied()).collect();
            let sampled_y: Vec<i32> = sample.iter().map(|&i| y[i]).collect();
            let sn = sample.len();
            let binned = bin_data_with_edges(&sampled_x, sn, p, &master_bins.edges);
            let bins = BinInfo { edges: master_bins.edges.clone(), n_bins: master_bins.n_bins.clone(), binned, p, n: sn };
            tree.fit_with_bins(&sampled_y, &bins);

            let tree_nc = tree.n_classes();
            let inv_km1 = -1.0 / (kf - 1.0);
            for i in 0..n {
                let row = &x[i * p..(i + 1) * p];
                let dist = tree.leaf_dist(row);
                let dist_sum: f64 = dist.iter().sum();
                let smooth_total = dist_sum + kf;

                let mut score = 0.0;
                for ci in 0..k {
                    let cnt = if ci < tree_nc { dist[ci] } else { 0.0 };
                    let pk = (cnt + 1.0) / smooth_total;
                    let coding = if ci == y_idx[i] { 1.0 } else { inv_km1 };
                    score += coding * pk.ln();
                }
                weights[i] *= (-self.learning_rate * (kf - 1.0) / kf * score).exp();
            }
            let wsum: f64 = weights.iter().sum();
            if wsum <= 0.0 { break; }
            for w in weights.iter_mut() { *w /= wsum; }

            self.trees.push(tree);
        }
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> {
        let k = self.classes.len();
        let kf = k as f64;
        let eps = 1e-15f64;
        let mut scores = vec![0.0; n * k];
        for tree in &self.trees {
            let tree_nc = tree.n_classes();
            for i in 0..n {
                let row = &x[i * p..(i + 1) * p];
                let dist = tree.leaf_dist(row);
                let dist_sum: f64 = dist.iter().sum();
                let smooth_total = dist_sum + kf;

                let mut mean_log = 0.0;
                for ci in 0..k {
                    let cnt = if ci < tree_nc { dist[ci] } else { 0.0 };
                    let pk = (cnt + 1.0) / smooth_total;
                    mean_log += pk.ln();
                }
                mean_log /= kf;

                for ci in 0..k {
                    let cnt = if ci < tree_nc { dist[ci] } else { 0.0 };
                    let pk = (cnt + 1.0) / smooth_total;
                    scores[i * k + ci] += self.learning_rate * (kf - 1.0) * (pk.ln() - mean_log);
                }
            }
        }
        (0..n).map(|i| {
            let row = &scores[i * k..(i + 1) * k];
            let best = row.iter().enumerate().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()).map(|(i, _)| i).unwrap_or(0);
            self.classes[best]
        }).collect()
    }

    pub fn predict_proba(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let k = self.classes.len();
        let kf = k as f64;
        let mut scores = vec![0.0; n * k];
        for tree in &self.trees {
            let tree_nc = tree.n_classes();
            for i in 0..n {
                let row = &x[i * p..(i + 1) * p];
                let dist = tree.leaf_dist(row);
                let dist_sum: f64 = dist.iter().sum();
                let smooth_total = dist_sum + kf;
                let mut mean_log = 0.0;
                for ci in 0..k {
                    let cnt = if ci < tree_nc { dist[ci] } else { 0.0 };
                    let pk = (cnt + 1.0) / smooth_total;
                    mean_log += pk.ln();
                }
                mean_log /= kf;
                for ci in 0..k {
                    let cnt = if ci < tree_nc { dist[ci] } else { 0.0 };
                    let pk = (cnt + 1.0) / smooth_total;
                    scores[i * k + ci] += self.learning_rate * (kf - 1.0) * (pk.ln() - mean_log);
                }
            }
        }
        for i in 0..n {
            let row = &mut scores[i * k..(i + 1) * k];
            let mx = row.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let mut sum = 0.0;
            for v in row.iter_mut() {
                *v = (*v - mx).exp();
                sum += *v;
            }
            let inv = 1.0 / sum;
            for v in row.iter_mut() { *v *= inv; }
        }
        scores
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
            let sampled_x: Vec<f64> = sample.iter().flat_map(|&i| x[i * p..(i + 1) * p].iter().copied()).collect();
            let sampled_y: Vec<f64> = sample.iter().map(|&i| y[i]).collect();
            let sn = sample.len();

            let mut tree = DecisionTreeRegressor::new(self.max_depth, 2, 1, None);
            let binned = bin_data_with_edges(&sampled_x, sn, p, &master_bins_r.edges);
            let bins = BinInfo { edges: master_bins_r.edges.clone(), n_bins: master_bins_r.n_bins.clone(), binned, p, n: sn };
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
        (0..n).map(|i| {
            let mut s = 0.0;
            for (t, w) in self.weights.iter().enumerate() {
                s += w * all_preds[t][i];
            }
            s / wsum
        }).collect()
    }
}
