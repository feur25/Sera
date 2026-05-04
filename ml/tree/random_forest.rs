use crate::ml::tree::decision_tree::{DecisionTreeClassifier, DecisionTreeRegressor, TreeCriterion, compute_bins, BinInfo};
use rayon::prelude::*;

pub struct RandomForestClassifier {
    pub n_estimators: usize,
    pub max_depth: usize,
    pub min_samples_split: usize,
    pub min_samples_leaf: usize,
    pub max_features: MaxFeatures,
    pub n_classes: usize,
    pub classes: Vec<i32>,
    trees: Vec<DecisionTreeClassifier>,
    pub feature_importances: Vec<f64>,
}

pub enum MaxFeatures { Sqrt, Log2, Fixed(usize), All }

impl RandomForestClassifier {
    pub fn new(n_estimators: usize, max_depth: usize, min_samples_split: usize, min_samples_leaf: usize, max_features: MaxFeatures) -> Self {
        Self {
            n_estimators, max_depth, min_samples_split, min_samples_leaf, max_features,
            n_classes: 0, classes: Vec::new(), trees: Vec::new(), feature_importances: Vec::new(),
        }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) {
        let mut classes: Vec<i32> = crate::ml::linalg::discover_classes(y);
        self.classes = classes.clone();
        self.n_classes = self.classes.len();

        let mf = match &self.max_features {
            MaxFeatures::Sqrt => (p as f64).sqrt().ceil() as usize,
            MaxFeatures::Log2 => (p as f64).log2().ceil().max(1.0) as usize,
            MaxFeatures::Fixed(v) => *v,
            MaxFeatures::All => p,
        };

        let master_bins = compute_bins(x, n, p);
        let trees: Vec<DecisionTreeClassifier> = (0..self.n_estimators).into_par_iter().map(|seed| {
            let mut rng = (seed as u64).wrapping_mul(0x9E3779B97F4A7C15).wrapping_add(0x123456789ABCDEF);
            let mut boot = vec![0u32; n];
            let mut yb = vec![0i32; n];
            for i in 0..n {
                rng ^= rng << 13; rng ^= rng >> 7; rng ^= rng << 17;
                let orig = rng as usize % n;
                boot[i] = orig as u32;
                yb[i] = y[orig];
            }
            let mut binned_boot = vec![0u8; n * p];
            for j in 0..p {
                let src = &master_bins.binned[j * n..(j + 1) * n];
                let dst = &mut binned_boot[j * n..(j + 1) * n];
                for (k, &bi) in boot.iter().enumerate() {
                    dst[k] = unsafe { *src.get_unchecked(bi as usize) };
                }
            }
            let bins = BinInfo { edges: master_bins.edges.clone(), n_bins: master_bins.n_bins.clone(), binned: binned_boot, p, n };
            let mut tree = DecisionTreeClassifier::new(self.max_depth, self.min_samples_split, self.min_samples_leaf, Some(mf), TreeCriterion::Gini);
            tree.need_dist = false;
            tree.fit_with_bins(&yb, &bins);
            tree
        }).collect();

        self.feature_importances = vec![0.0; p];
        for t in &trees {
            for (j, &v) in t.feature_importances.iter().enumerate() {
                self.feature_importances[j] += v;
            }
        }
        let inv = 1.0 / self.n_estimators as f64;
        for v in &mut self.feature_importances { *v *= inv; }

        self.trees = trees;
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> {
        let nc = self.n_classes;
        let all_preds: Vec<Vec<i32>> = self.trees.par_iter().map(|t| t.predict(x, n, p)).collect();
        let cmin = *self.classes.iter().min().unwrap_or(&0);
        let cmax = *self.classes.iter().max().unwrap_or(&0);
        let range = (cmax - cmin + 1) as usize;
        let mut cmap = vec![0usize; range];
        for (i, &c) in self.classes.iter().enumerate() { cmap[(c - cmin) as usize] = i; }
        let vote_one = |i: usize| -> i32 {
            let mut votes = [0u32; 64];
            for preds in &all_preds {
                unsafe { *votes.get_unchecked_mut(*cmap.get_unchecked((preds[i] - cmin) as usize)) += 1; }
            }
            let mut best = 0;
            let mut best_v = 0u32;
            for c in 0..nc { if votes[c] > best_v { best_v = votes[c]; best = c; } }
            self.classes[best]
        };
        if n >= 64 {
            (0..n).into_par_iter().map(vote_one).collect()
        } else {
            (0..n).map(vote_one).collect()
        }
    }

    pub fn predict_proba(&self, x: &[f64], n: usize, p: usize) -> Vec<Vec<f64>> {
        let nc = self.n_classes;
        let all_proba: Vec<Vec<Vec<f64>>> = self.trees.par_iter().map(|t| t.predict_proba(x, n, p)).collect();
        let inv = 1.0 / self.n_estimators as f64;
        let proba_one = |i: usize| -> Vec<f64> {
            let mut avg = vec![0.0; nc];
            for proba in &all_proba {
                for (j, &v) in proba[i].iter().enumerate() { avg[j] += v; }
            }
            for v in &mut avg { *v *= inv; }
            avg
        };
        if n >= 512 {
            (0..n).into_par_iter().map(proba_one).collect()
        } else {
            (0..n).map(proba_one).collect()
        }
    }
}

pub struct RandomForestRegressor {
    pub n_estimators: usize,
    pub max_depth: usize,
    pub min_samples_split: usize,
    pub min_samples_leaf: usize,
    pub max_features: MaxFeatures,
    trees: Vec<DecisionTreeRegressor>,
    pub feature_importances: Vec<f64>,
}

impl RandomForestRegressor {
    pub fn new(n_estimators: usize, max_depth: usize, min_samples_split: usize, min_samples_leaf: usize, max_features: MaxFeatures) -> Self {
        Self {
            n_estimators, max_depth, min_samples_split, min_samples_leaf, max_features,
            trees: Vec::new(), feature_importances: Vec::new(),
        }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) {
        let mf = match &self.max_features {
            MaxFeatures::Sqrt => (p as f64).sqrt().ceil() as usize,
            MaxFeatures::Log2 => (p as f64).log2().ceil().max(1.0) as usize,
            MaxFeatures::Fixed(v) => *v,
            MaxFeatures::All => p,
        };

        let master_bins = compute_bins(x, n, p);
        let trees: Vec<DecisionTreeRegressor> = (0..self.n_estimators).into_par_iter().map(|seed| {
            let mut rng = (seed as u64).wrapping_mul(0x9E3779B97F4A7C15).wrapping_add(0x123456789ABCDEF);
            let mut boot = vec![0u32; n];
            let mut yb = vec![0.0; n];
            for i in 0..n {
                rng ^= rng << 13; rng ^= rng >> 7; rng ^= rng << 17;
                let orig = rng as usize % n;
                boot[i] = orig as u32;
                yb[i] = y[orig];
            }
            let mut binned_boot = vec![0u8; n * p];
            for j in 0..p {
                let src = &master_bins.binned[j * n..(j + 1) * n];
                let dst = &mut binned_boot[j * n..(j + 1) * n];
                for (k, &bi) in boot.iter().enumerate() {
                    dst[k] = unsafe { *src.get_unchecked(bi as usize) };
                }
            }
            let bins = BinInfo { edges: master_bins.edges.clone(), n_bins: master_bins.n_bins.clone(), binned: binned_boot, p, n };
            let mut tree = DecisionTreeRegressor::new(self.max_depth, self.min_samples_split, self.min_samples_leaf, Some(mf));
            tree.fit_with_bins(&yb, &bins);
            tree
        }).collect();

        self.feature_importances = vec![0.0; p];
        for t in &trees {
            for (j, &v) in t.feature_importances.iter().enumerate() {
                self.feature_importances[j] += v;
            }
        }
        let inv = 1.0 / self.n_estimators as f64;
        for v in &mut self.feature_importances { *v *= inv; }

        self.trees = trees;
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let all_preds: Vec<Vec<f64>> = self.trees.par_iter().map(|t| t.predict(x, n, p)).collect();
        let inv = 1.0 / self.n_estimators as f64;
        if n >= 256 {
            (0..n).into_par_iter().map(|i| {
                let mut s = 0.0;
                for preds in &all_preds { s += unsafe { *preds.get_unchecked(i) }; }
                s * inv
            }).collect()
        } else {
            (0..n).map(|i| {
                let mut s = 0.0;
                for preds in &all_preds { s += unsafe { *preds.get_unchecked(i) }; }
                s * inv
            }).collect()
        }
    }
}

impl crate::ml::MlClassifier for RandomForestClassifier {
    fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) { self.fit(x, n, p, y); }
    fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> { self.predict(x, n, p) }
}

impl crate::ml::MlRegressor for RandomForestRegressor {
    fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) { self.fit(x, n, p, y); }
    fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> { self.predict(x, n, p) }
}
