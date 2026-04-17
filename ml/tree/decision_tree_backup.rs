use rayon::prelude::*;

#[derive(Clone)]
pub enum TreeCriterion { Gini, Entropy, MSE }

#[derive(Clone)]
enum NodeKind {
    Leaf { value: f64, class: i32, dist: Vec<f64> },
    Split { feature: usize, threshold: f64, left: usize, right: usize },
}

#[derive(Clone)]
struct TreeNode {
    kind: NodeKind,
    n_samples: usize,
}

pub struct DecisionTreeClassifier {
    pub max_depth: usize,
    pub min_samples_split: usize,
    pub min_samples_leaf: usize,
    pub max_features: Option<usize>,
    pub criterion: TreeCriterion,
    pub n_classes: usize,
    pub classes: Vec<i32>,
    nodes: Vec<TreeNode>,
    pub feature_importances: Vec<f64>,
}

impl DecisionTreeClassifier {
    pub fn new(max_depth: usize, min_samples_split: usize, min_samples_leaf: usize, max_features: Option<usize>, criterion: TreeCriterion) -> Self {
        Self {
            max_depth, min_samples_split, min_samples_leaf, max_features,
            criterion, n_classes: 0, classes: Vec::new(), nodes: Vec::new(),
            feature_importances: Vec::new(),
        }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) {
        self.fit_with_presort(x, n, p, y, n >= 5000);
    }

    pub fn fit_with_presort(&mut self, x: &[f64], n: usize, p: usize, y: &[i32], presort: bool) {
        let mut classes: Vec<i32> = y.to_vec();
        classes.sort_unstable();
        classes.dedup();
        self.classes = classes;
        self.n_classes = self.classes.len();

        let y_idx: Vec<usize> = y.iter().map(|&v| self.classes.iter().position(|&c| c == v).unwrap()).collect();
        let sorted_cols: Vec<Vec<usize>> = if presort {
            (0..p).into_par_iter().map(|feat| {
                let mut order: Vec<usize> = (0..n).collect();
                order.sort_unstable_by(|&a, &b| x[a * p + feat].partial_cmp(&x[b * p + feat]).unwrap());
                order
            }).collect()
        } else {
            Vec::new()
        };
        let indices: Vec<usize> = (0..n).collect();
        self.nodes.clear();
        self.feature_importances = vec![0.0; p];

        let mut rng = 0x123456789ABCDEFu64;
        self.build_node(x, p, &y_idx, &indices, 0, &mut rng, &sorted_cols, n);

        let sum: f64 = self.feature_importances.iter().sum();
        if sum > 0.0 {
            for v in &mut self.feature_importances { *v /= sum; }
        }
    }

    fn build_node(&mut self, x: &[f64], p: usize, y: &[usize], indices: &[usize], depth: usize, rng: &mut u64, sorted_cols: &[Vec<usize>], orig_n: usize) -> usize {
        let nc = self.n_classes;
        let n = indices.len();
        let idx = self.nodes.len();
        self.nodes.push(TreeNode { kind: NodeKind::Leaf { value: 0.0, class: 0, dist: Vec::new() }, n_samples: n });

        let mut dist = vec![0.0; nc];
        for &i in indices { dist[y[i]] += 1.0; }
        let majority = dist.iter().enumerate().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()).map(|(i, _)| i).unwrap_or(0);

        if depth >= self.max_depth || n < self.min_samples_split || dist.iter().filter(|&&v| v > 0.0).count() <= 1 {
            self.nodes[idx].kind = NodeKind::Leaf { value: dist[majority], class: self.classes[majority], dist };
            return idx;
        }

        let max_f = self.max_features.unwrap_or(p);
        let features: Vec<usize> = if max_f >= p {
            (0..p).collect()
        } else {
            let mut fs = Vec::with_capacity(max_f);
            while fs.len() < max_f {
                *rng ^= *rng << 13; *rng ^= *rng >> 7; *rng ^= *rng << 17;
                let f = *rng as usize % p;
                if !fs.contains(&f) { fs.push(f); }
            }
            fs
        };

        let parent_impurity = impurity_from_dist(&dist, n, &self.criterion);
        let mut best_gain = 0.0;
        let mut best_feat = 0;
        let mut best_thr = 0.0;

        let criterion = &self.criterion;
        let min_sl = self.min_samples_leaf;
        if !sorted_cols.is_empty() {
            let full_n = orig_n;
            let in_node: Vec<bool> = if full_n > n {
                let mut v = vec![false; full_n];
                for &i in indices { v[i] = true; }
                v
            } else {
                vec![true; n]
            };

            if features.len() >= 4 {
                let results: Vec<(f64, usize, f64)> = features.par_iter().map(|&feat| {
                    let vals: Vec<(f64, usize)> = sorted_cols[feat].iter()
                        .filter(|&&i| in_node[i])
                        .map(|&i| (x[i * p + feat], y[i])).collect();
                    let vn = vals.len();
                    let mut left_dist = vec![0.0; nc];
                    let mut right_dist = dist.clone();
                    let mut nl = 0usize;
                    let mut bg = 0.0;
                    let mut bt = 0.0;
                    for s in 0..vn.saturating_sub(1) {
                        left_dist[vals[s].1] += 1.0;
                        right_dist[vals[s].1] -= 1.0;
                        nl += 1;
                        let nr = vn - nl;
                        if nl < min_sl || nr < min_sl { continue; }
                        if (vals[s].0 - vals[s + 1].0).abs() < 1e-15 { continue; }
                        let il = impurity_from_dist(&left_dist, nl, criterion);
                        let ir = impurity_from_dist(&right_dist, nr, criterion);
                        let gain = parent_impurity - (nl as f64 / vn as f64) * il - (nr as f64 / vn as f64) * ir;
                        if gain > bg { bg = gain; bt = (vals[s].0 + vals[s + 1].0) * 0.5; }
                    }
                    (bg, feat, bt)
                }).collect();
                for (g, f, t) in results {
                    if g > best_gain { best_gain = g; best_feat = f; best_thr = t; }
                }
            } else {
                for &feat in &features {
                    let vals: Vec<(f64, usize)> = sorted_cols[feat].iter()
                        .filter(|&&i| in_node[i])
                        .map(|&i| (x[i * p + feat], y[i])).collect();
                    let vn = vals.len();
                    let mut left_dist = vec![0.0; nc];
                    let mut right_dist = dist.clone();
                    let mut nl = 0usize;
                    for s in 0..vn.saturating_sub(1) {
                        left_dist[vals[s].1] += 1.0;
                        right_dist[vals[s].1] -= 1.0;
                        nl += 1;
                        let nr = vn - nl;
                        if nl < self.min_samples_leaf || nr < self.min_samples_leaf { continue; }
                        if (vals[s].0 - vals[s + 1].0).abs() < 1e-15 { continue; }
                        let il = impurity_from_dist(&left_dist, nl, &self.criterion);
                        let ir = impurity_from_dist(&right_dist, nr, &self.criterion);
                        let gain = parent_impurity - (nl as f64 / vn as f64) * il - (nr as f64 / vn as f64) * ir;
                        if gain > best_gain { best_gain = gain; best_feat = feat; best_thr = (vals[s].0 + vals[s + 1].0) * 0.5; }
                    }
                }
            }
        } else {
            let scan_feat = |feat: usize| -> (f64, f64) {
                let mut vals: Vec<(f64, usize)> = indices.iter().map(|&i| (x[i * p + feat], y[i])).collect();
                vals.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                let vn = vals.len();
                let mut left_dist = vec![0.0; nc];
                let mut right_dist = dist.clone();
                let mut nl = 0usize;
                let mut bg = 0.0f64;
                let mut bt = 0.0f64;
                for s in 0..vn.saturating_sub(1) {
                    left_dist[vals[s].1] += 1.0;
                    right_dist[vals[s].1] -= 1.0;
                    nl += 1;
                    let nr = vn - nl;
                    if nl < min_sl || nr < min_sl { continue; }
                    if (vals[s].0 - vals[s + 1].0).abs() < 1e-15 { continue; }
                    let il = impurity_from_dist(&left_dist, nl, criterion);
                    let ir = impurity_from_dist(&right_dist, nr, criterion);
                    let gain = parent_impurity - (nl as f64 / vn as f64) * il - (nr as f64 / vn as f64) * ir;
                    if gain > bg { bg = gain; bt = (vals[s].0 + vals[s + 1].0) * 0.5; }
                }
                (bg, bt)
            };
            for &feat in &features {
                let (g, t) = scan_feat(feat);
                if g > best_gain { best_gain = g; best_feat = feat; best_thr = t; }
            }
        }

        if best_gain <= 0.0 {
            self.nodes[idx].kind = NodeKind::Leaf { value: dist[majority], class: self.classes[majority], dist };
            return idx;
        }

        self.feature_importances[best_feat] += best_gain * n as f64;

        let mut left_idx = Vec::new();
        let mut right_idx = Vec::new();
        for &i in indices {
            if x[i * p + best_feat] <= best_thr { left_idx.push(i); }
            else { right_idx.push(i); }
        }

        let left = self.build_node(x, p, y, &left_idx, depth + 1, rng, sorted_cols, orig_n);
        let right = self.build_node(x, p, y, &right_idx, depth + 1, rng, sorted_cols, orig_n);
        self.nodes[idx].kind = NodeKind::Split { feature: best_feat, threshold: best_thr, left, right };
        idx
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> {
        if n >= 1024 {
            (0..n).into_par_iter().map(|i| {
                let row = &x[i * p..(i + 1) * p];
                self.predict_single(row)
            }).collect()
        } else {
            (0..n).map(|i| {
                let row = &x[i * p..(i + 1) * p];
                self.predict_single(row)
            }).collect()
        }
    }

    pub fn predict_proba(&self, x: &[f64], n: usize, p: usize) -> Vec<Vec<f64>> {
        (0..n).map(|i| {
            let row = &x[i * p..(i + 1) * p];
            self.predict_proba_single(row)
        }).collect()
    }

    fn predict_single(&self, row: &[f64]) -> i32 {
        let mut node = 0;
        loop {
            match &self.nodes[node].kind {
                NodeKind::Leaf { class, .. } => return *class,
                NodeKind::Split { feature, threshold, left, right } => {
                    node = if row[*feature] <= *threshold { *left } else { *right };
                }
            }
        }
    }

    fn predict_proba_single(&self, row: &[f64]) -> Vec<f64> {
        let mut node = 0;
        loop {
            match &self.nodes[node].kind {
                NodeKind::Leaf { dist, .. } => {
                    let sum: f64 = dist.iter().sum();
                    return if sum > 0.0 { dist.iter().map(|&v| v / sum).collect() } else { dist.clone() };
                }
                NodeKind::Split { feature, threshold, left, right } => {
                    node = if row[*feature] <= *threshold { *left } else { *right };
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct DecisionTreeRegressor {
    pub max_depth: usize,
    pub min_samples_split: usize,
    pub min_samples_leaf: usize,
    pub max_features: Option<usize>,
    nodes: Vec<TreeNodeReg>,
    pub feature_importances: Vec<f64>,
}

#[derive(Clone)]
enum TreeNodeRegKind {
    Leaf { value: f64 },
    Split { feature: usize, threshold: f64, left: usize, right: usize },
}

#[derive(Clone)]
struct TreeNodeReg {
    kind: TreeNodeRegKind,
    n_samples: usize,
}

impl DecisionTreeRegressor {
    pub fn new(max_depth: usize, min_samples_split: usize, min_samples_leaf: usize, max_features: Option<usize>) -> Self {
        Self { max_depth, min_samples_split, min_samples_leaf, max_features, nodes: Vec::new(), feature_importances: Vec::new() }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) {
        self.fit_with_presort(x, n, p, y, n >= 5000);
    }

    pub fn fit_with_presort(&mut self, x: &[f64], n: usize, p: usize, y: &[f64], presort: bool) {
        let indices: Vec<usize> = (0..n).collect();
        self.nodes.clear();
        self.feature_importances = vec![0.0; p];
        let sorted_cols: Vec<Vec<usize>> = if presort {
            (0..p).into_par_iter().map(|feat| {
                let mut order: Vec<usize> = (0..n).collect();
                order.sort_unstable_by(|&a, &b| x[a * p + feat].partial_cmp(&x[b * p + feat]).unwrap());
                order
            }).collect()
        } else { Vec::new() };
        let mut rng = 0x123456789ABCDEFu64;
        self.build_node(x, p, y, &indices, 0, &mut rng, &sorted_cols, n);
        let sum: f64 = self.feature_importances.iter().sum();
        if sum > 0.0 { for v in &mut self.feature_importances { *v /= sum; } }
    }

    fn build_node(&mut self, x: &[f64], p: usize, y: &[f64], indices: &[usize], depth: usize, rng: &mut u64, sorted_cols: &[Vec<usize>], orig_n: usize) -> usize {
        let n = indices.len();
        let idx = self.nodes.len();
        let mean_y: f64 = indices.iter().map(|&i| y[i]).sum::<f64>() / n as f64;
        self.nodes.push(TreeNodeReg { kind: TreeNodeRegKind::Leaf { value: mean_y }, n_samples: n });

        if depth >= self.max_depth || n < self.min_samples_split { return idx; }

        let parent_mse: f64 = indices.iter().map(|&i| { let d = y[i] - mean_y; d * d }).sum::<f64>() / n as f64;

        let max_f = self.max_features.unwrap_or(p);
        let features: Vec<usize> = if max_f >= p {
            (0..p).collect()
        } else {
            let mut fs = Vec::with_capacity(max_f);
            while fs.len() < max_f {
                *rng ^= *rng << 13; *rng ^= *rng >> 7; *rng ^= *rng << 17;
                let f = *rng as usize % p;
                if !fs.contains(&f) { fs.push(f); }
            }
            fs
        };

        let mut best_gain = 0.0;
        let mut best_feat = 0;
        let mut best_thr = 0.0;
        let min_sl = self.min_samples_leaf;

        let scan_feat_reg = |feat: usize, vals: &[(f64, f64)]| -> (f64, f64) {
            let vn = vals.len();
            let mut sl = 0.0f64;
            let mut ssl = 0.0f64;
            let total_sum: f64 = vals.iter().map(|v| v.1).sum();
            let total_sq: f64 = vals.iter().map(|v| v.1 * v.1).sum();
            let mut nl = 0usize;
            let mut bg = 0.0f64;
            let mut bt = 0.0f64;
            for s in 0..vn.saturating_sub(1) {
                sl += vals[s].1;
                ssl += vals[s].1 * vals[s].1;
                nl += 1;
                let nr = vn - nl;
                if nl < min_sl || nr < min_sl { continue; }
                if (vals[s].0 - vals[s + 1].0).abs() < 1e-15 { continue; }
                let sr = total_sum - sl;
                let ssr = total_sq - ssl;
                let mse_l = ssl / nl as f64 - (sl / nl as f64) * (sl / nl as f64);
                let mse_r = ssr / nr as f64 - (sr / nr as f64) * (sr / nr as f64);
                let gain = parent_mse - (nl as f64 / vn as f64) * mse_l - (nr as f64 / vn as f64) * mse_r;
                if gain > bg { bg = gain; bt = (vals[s].0 + vals[s + 1].0) * 0.5; }
            }
            (bg, bt)
        };

        if !sorted_cols.is_empty() {
            let in_node: Vec<bool> = if orig_n > n {
                let mut v = vec![false; orig_n];
                for &i in indices { v[i] = true; }
                v
            } else { vec![true; n] };
            for &feat in &features {
                let vals: Vec<(f64, f64)> = sorted_cols[feat].iter()
                    .filter(|&&i| in_node[i])
                    .map(|&i| (x[i * p + feat], y[i])).collect();
                let (g, t) = scan_feat_reg(feat, &vals);
                if g > best_gain { best_gain = g; best_feat = feat; best_thr = t; }
            }
        } else {
            for &feat in &features {
                let mut vals: Vec<(f64, f64)> = indices.iter().map(|&i| (x[i * p + feat], y[i])).collect();
                vals.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                let (g, t) = scan_feat_reg(feat, &vals);
                if g > best_gain { best_gain = g; best_feat = feat; best_thr = t; }
            }
        }

        if best_gain <= 0.0 { return idx; }

        self.feature_importances[best_feat] += best_gain * n as f64;

        let mut left_idx = Vec::new();
        let mut right_idx = Vec::new();
        for &i in indices {
            if x[i * p + best_feat] <= best_thr { left_idx.push(i); }
            else { right_idx.push(i); }
        }

        let left = self.build_node(x, p, y, &left_idx, depth + 1, rng, sorted_cols, orig_n);
        let right = self.build_node(x, p, y, &right_idx, depth + 1, rng, sorted_cols, orig_n);
        self.nodes[idx].kind = TreeNodeRegKind::Split { feature: best_feat, threshold: best_thr, left, right };
        idx
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        if n >= 1024 {
            (0..n).into_par_iter().map(|i| {
                let row = &x[i * p..(i + 1) * p];
                let mut node = 0;
                loop {
                    match &self.nodes[node].kind {
                        TreeNodeRegKind::Leaf { value } => return *value,
                        TreeNodeRegKind::Split { feature, threshold, left, right } => {
                            node = if row[*feature] <= *threshold { *left } else { *right };
                        }
                    }
                }
            }).collect()
        } else {
            (0..n).map(|i| {
                let row = &x[i * p..(i + 1) * p];
                let mut node = 0;
                loop {
                    match &self.nodes[node].kind {
                        TreeNodeRegKind::Leaf { value } => return *value,
                        TreeNodeRegKind::Split { feature, threshold, left, right } => {
                            node = if row[*feature] <= *threshold { *left } else { *right };
                        }
                    }
                }
            }).collect()
        }
    }
}

fn impurity_from_dist(dist: &[f64], n: usize, criterion: &TreeCriterion) -> f64 {
    let inv = 1.0 / n as f64;
    match criterion {
        TreeCriterion::Gini => {
            let mut s = 1.0;
            for &d in dist { let p = d * inv; s -= p * p; }
            s
        }
        TreeCriterion::Entropy => {
            let mut s = 0.0;
            for &d in dist {
                if d > 0.0 { let p = d * inv; s -= p * p.ln(); }
            }
            s
        }
        TreeCriterion::MSE => 0.0,
    }
}
