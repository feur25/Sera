use rayon::prelude::*;
use crate::ml::linalg::splitmix64;

#[derive(Clone)]
struct INode {
    feature: i32,
    threshold: f64,
    left: i32,
    right: i32,
    size: u32,
    depth: u16,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct IsolationTree {
    nodes: Vec<INode>,
    height_limit: u16,
}

fn average_path_length(n: f64) -> f64 {
    if n <= 1.0 { return 0.0; }
    if n == 2.0 { return 1.0; }
    2.0 * ((n - 1.0).ln() + 0.5772156649015329) - 2.0 * (n - 1.0) / n
}

impl IsolationTree {
    fn build(x: &[f64], _n: usize, p: usize, sample_idx: &[u32], height_limit: u16, mut rng: u64) -> Self {
        let mut nodes: Vec<INode> = Vec::with_capacity(sample_idx.len() * 2);
        let stack: Vec<(Vec<u32>, u16)> = vec![(sample_idx.to_vec(), 0)];
        let mut node_starts: Vec<i32> = Vec::new();
        node_starts.push(0);

        fn build_rec(
            x: &[f64], p: usize, indices: Vec<u32>, depth: u16, height_limit: u16,
            nodes: &mut Vec<INode>, rng: &mut u64,
        ) -> i32 {
            let m = indices.len();
            if m <= 1 || depth >= height_limit {
                let id = nodes.len() as i32;
                nodes.push(INode { feature: -1, threshold: 0.0, left: -1, right: -1, size: m as u32, depth });
                return id;
            }
            *rng = splitmix64(*rng);
            let feat = (*rng as usize % p) as i32;
            let mut fmin = f64::INFINITY;
            let mut fmax = f64::NEG_INFINITY;
            for &idx in &indices {
                let v = x[idx as usize * p + feat as usize];
                if v < fmin { fmin = v; }
                if v > fmax { fmax = v; }
            }
            if fmin == fmax {
                let id = nodes.len() as i32;
                nodes.push(INode { feature: -1, threshold: 0.0, left: -1, right: -1, size: m as u32, depth });
                return id;
            }
            *rng = splitmix64(*rng);
            let frac = (*rng & 0x000FFFFFFFFFFFFF) as f64 / (1u64 << 52) as f64;
            let threshold = fmin + frac * (fmax - fmin);
            let mut left_idx = Vec::new();
            let mut right_idx = Vec::new();
            for &idx in &indices {
                let v = x[idx as usize * p + feat as usize];
                if v < threshold { left_idx.push(idx); } else { right_idx.push(idx); }
            }
            let id = nodes.len() as i32;
            nodes.push(INode { feature: feat, threshold, left: -1, right: -1, size: m as u32, depth });
            let l = build_rec(x, p, left_idx, depth + 1, height_limit, nodes, rng);
            let r = build_rec(x, p, right_idx, depth + 1, height_limit, nodes, rng);
            nodes[id as usize].left = l;
            nodes[id as usize].right = r;
            id
        }

        let _ = stack;
        let _ = node_starts;
        build_rec(x, p, sample_idx.to_vec(), 0, height_limit, &mut nodes, &mut rng);
        Self { nodes, height_limit }
    }

    fn path_length(&self, row: &[f64]) -> f64 {
        let mut node_id = 0i32;
        loop {
            let nd = &self.nodes[node_id as usize];
            if nd.feature < 0 {
                return nd.depth as f64 + average_path_length(nd.size as f64);
            }
            let v = row[nd.feature as usize];
            node_id = if v < nd.threshold { nd.left } else { nd.right };
        }
    }
}

#[crate::model(category = "Anomaly Detection", domain = "ml")]
pub struct IsolationForest {
    pub n_estimators: usize,
    pub max_samples: usize,
    pub contamination: f64,
    pub seed: u64,
    trees: Vec<IsolationTree>,
    psi: usize,
    pub threshold_: f64,
}

impl IsolationForest {
    pub fn new(n_estimators: usize, max_samples: usize, contamination: f64, seed: u64) -> Self {
        Self { n_estimators, max_samples, contamination, seed, trees: Vec::new(), psi: 0, threshold_: 0.0 }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize) {
        let psi = self.max_samples.min(n).max(2);
        self.psi = psi;
        let height_limit = (psi as f64).log2().ceil() as u16;
        let trees: Vec<IsolationTree> = (0..self.n_estimators).into_par_iter().map(|t| {
            let mut rng = self.seed.wrapping_add((t as u64).wrapping_mul(0x9E3779B97F4A7C15));
            let mut sample = vec![0u32; psi];
            for i in 0..psi {
                rng = splitmix64(rng);
                sample[i] = (rng as usize % n) as u32;
            }
            IsolationTree::build(x, n, p, &sample, height_limit, rng)
        }).collect();
        self.trees = trees;
        let scores = self.score_samples(x, n, p);
        let mut sorted = scores.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let k = ((self.contamination * n as f64).round() as usize).min(n.saturating_sub(1)).max(1);
        self.threshold_ = sorted[k - 1];
    }

    pub fn score_samples(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let c = average_path_length(self.psi as f64).max(1e-12);
        (0..n).into_par_iter().map(|i| {
            let row = &x[i * p..i * p + p];
            let avg: f64 = self.trees.iter().map(|t| t.path_length(row)).sum::<f64>() / self.trees.len() as f64;
            -2f64.powf(-avg / c)
        }).collect()
    }

    pub fn decision_function(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        self.score_samples(x, n, p).into_iter().map(|s| s - self.threshold_).collect()
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> {
        self.score_samples(x, n, p).into_iter()
            .map(|s| if s < self.threshold_ { -1 } else { 1 })
            .collect()
    }

    pub fn fit_predict(&mut self, x: &[f64], n: usize, p: usize) -> Vec<i32> {
        self.fit(x, n, p);
        self.predict(x, n, p)
    }
}


