use rayon::prelude::*;
use std::cell::RefCell;

const MAX_BINS: usize = 256;
const MAX_NC: usize = 64;

thread_local! {
    static CLS_WORK: RefCell<Vec<u32>> = RefCell::new(vec![0u32; MAX_BINS * MAX_NC]);
    static CLS_WWORK: RefCell<Vec<f64>> = RefCell::new(vec![0.0; MAX_BINS * MAX_NC]);
    static REG_WORK: RefCell<(Vec<f64>, Vec<f64>, Vec<u32>)> = RefCell::new((
        vec![0.0f64; MAX_BINS], vec![0.0f64; MAX_BINS], vec![0u32; MAX_BINS]
    ));
    static PART_BUF: RefCell<Vec<usize>> = RefCell::new(Vec::new());
}

#[derive(Clone)]
pub enum TreeCriterion {
    Gini,
    Entropy,
    MSE,
}

#[allow(dead_code)]
#[derive(Clone)]
enum NodeKind {
    Leaf {
        value: f64,
        class: i32,
        dist: Vec<f64>,
    },
    Split {
        feature: usize,
        threshold: f64,
        left: usize,
        right: usize,
    },
}

#[allow(dead_code)]
#[derive(Clone)]
struct TreeNode {
    kind: NodeKind,
    n_samples: usize,
}

pub struct BinInfo {
    pub edges: Vec<Vec<f64>>,
    pub n_bins: Vec<usize>,
    pub binned: Vec<u8>,
    pub p: usize,
    pub n: usize,
}

pub fn compute_bins(x: &[f64], n: usize, p: usize) -> BinInfo {
    let (edges, n_bins): (Vec<Vec<f64>>, Vec<usize>) = (0..p)
        .into_par_iter()
        .map(|j| {
            let mut col: Vec<f64> = (0..n).map(|i| x[i * p + j]).collect();
            col.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            col.dedup();
            let nb = col.len().min(MAX_BINS);
            if nb <= 1 {
                return (vec![f64::MAX], 1);
            }
            let step = col.len() as f64 / nb as f64;
            let mut e = Vec::with_capacity(nb);
            for b in 1..nb {
                let idx = (b as f64 * step) as usize;
                let idx = idx.min(col.len() - 1);
                e.push((col[idx - 1] + col[idx]) * 0.5);
            }
            e.push(f64::MAX);
            let actual_bins = e.len();
            (e, actual_bins)
        })
        .unzip();

    let mut binned = vec![0u8; n * p];
    let edges_ref = &edges;
    if p >= 4 {
        binned.par_chunks_mut(n).enumerate().for_each(|(j, col)| {
            let ej = &edges_ref[j];
            for i in 0..n {
                col[i] = ej.partition_point(|&e| e < x[i * p + j]) as u8;
            }
        });
    } else {
        for j in 0..p {
            let col = &mut binned[j * n..(j + 1) * n];
            for i in 0..n {
                col[i] = edges[j].partition_point(|&e| e < x[i * p + j]) as u8;
            }
        }
    }
    BinInfo {
        edges,
        n_bins,
        binned,
        p,
        n,
    }
}

pub fn bin_data_with_edges(x: &[f64], n: usize, p: usize, edges: &[Vec<f64>]) -> Vec<u8> {
    let mut binned = vec![0u8; n * p];
    if p >= 4 {
        binned.par_chunks_mut(n).enumerate().for_each(|(j, col)| {
            for i in 0..n {
                col[i] = edges[j].partition_point(|&e| e < x[i * p + j]) as u8;
            }
        });
    } else {
        for j in 0..p {
            let col = &mut binned[j * n..(j + 1) * n];
            for i in 0..n {
                col[i] = edges[j].partition_point(|&e| e < x[i * p + j]) as u8;
            }
        }
    }
    binned
}

#[crate::model(category = "Tree-Based", domain = "ml")]
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
    pub need_dist: bool,
}

impl DecisionTreeClassifier {
    pub fn new(
        max_depth: usize,
        min_samples_split: usize,
        min_samples_leaf: usize,
        max_features: Option<usize>,
        criterion: TreeCriterion,
    ) -> Self {
        Self {
            max_depth,
            min_samples_split,
            min_samples_leaf,
            max_features,
            criterion,
            n_classes: 0,
            classes: Vec::new(),
            nodes: Vec::new(),
            feature_importances: Vec::new(),
            need_dist: true,
        }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) {
        let bins = compute_bins(x, n, p);
        self.fit_with_bins(y, &bins);
    }

    pub fn fit_with_presort(&mut self, x: &[f64], n: usize, p: usize, y: &[i32], _presort: bool) {
        self.fit(x, n, p, y);
    }

    pub fn fit_with_bins(&mut self, y: &[i32], bins: &BinInfo) {
        let n = bins.n;
        let p = bins.p;
        let classes = crate::ml::linalg::discover_classes(y);
        self.classes = classes;
        self.n_classes = self.classes.len();

        let cmin = *self.classes.iter().min().unwrap_or(&0);
        let cmax = *self.classes.iter().max().unwrap_or(&0);
        let crange = (cmax - cmin + 1) as usize;
        let mut cmap = vec![0u8; crange];
        for (i, &c) in self.classes.iter().enumerate() {
            cmap[(c - cmin) as usize] = i as u8;
        }
        let y_idx: Vec<u8> = y.iter().map(|&v| cmap[(v - cmin) as usize]).collect();
        self.nodes.clear();
        self.feature_importances = vec![0.0; p];

        let mut rng = 0x123456789ABCDEFu64;
        let mut indices: Vec<usize> = (0..n).collect();
        self.build_node(bins, &y_idx, &mut indices, 0, &mut rng);

        let sum: f64 = self.feature_importances.iter().sum();
        if sum > 0.0 {
            for v in &mut self.feature_importances {
                *v /= sum;
            }
        }
    }

    fn build_node(
        &mut self,
        bins: &BinInfo,
        y: &[u8],
        indices: &mut [usize],
        depth: usize,
        rng: &mut u64,
    ) -> usize {
        let nc = self.n_classes;
        let n = indices.len();
        let p = bins.p;
        let bn = bins.n;
        let idx = self.nodes.len();
        self.nodes.push(TreeNode {
            kind: NodeKind::Leaf {
                value: 0.0,
                class: 0,
                dist: Vec::new(),
            },
            n_samples: n,
        });

        let mut cnts = [0u32; MAX_NC];
        for &i in indices.iter() {
            unsafe {
                *cnts.get_unchecked_mut(*y.get_unchecked(i) as usize) += 1;
            }
        }
        let majority = cnts[..nc]
            .iter()
            .enumerate()
            .max_by_key(|(_, &v)| v)
            .map(|(i, _)| i)
            .unwrap_or(0);

        if depth >= self.max_depth
            || n < self.min_samples_split
            || cnts[..nc].iter().filter(|&&v| v > 0).count() <= 1
        {
            let dist: Vec<f64> = if self.need_dist {
                cnts[..nc].iter().map(|&v| v as f64).collect()
            } else {
                Vec::new()
            };
            self.nodes[idx].kind = NodeKind::Leaf {
                value: cnts[majority] as f64,
                class: self.classes[majority],
                dist,
            };
            return idx;
        }

        let max_f = self.max_features.unwrap_or(p);
        let mut feats_arr = [0usize; 64];
        let features: &[usize] = if max_f >= p {
            for i in 0..p {
                feats_arr[i] = i;
            }
            &feats_arr[..p]
        } else {
            let mut len = 0;
            while len < max_f {
                *rng ^= *rng << 13;
                *rng ^= *rng >> 7;
                *rng ^= *rng << 17;
                let f = *rng as usize % p;
                if !feats_arr[..len].contains(&f) {
                    feats_arr[len] = f;
                    len += 1;
                }
            }
            &feats_arr[..max_f]
        };

        let parent_impurity = gini_from_counts(&cnts[..nc], n);
        let mut best_gain = 0.0;
        let mut best_feat = 0;
        let mut best_bin = 0usize;
        let min_sl = self.min_samples_leaf;

        let scan_feat = |feat: usize| -> (f64, usize) {
            let nb = unsafe { *bins.n_bins.get_unchecked(feat) };
            CLS_WORK.with(|ws| {
                let mut ws = ws.borrow_mut();
                let len = nb * nc;
                ws[..len].fill(0);
                let col_off = feat * bn;
                for &i in indices.iter() {
                    let b = unsafe { *bins.binned.get_unchecked(col_off + i) } as usize;
                    let ci = unsafe { *y.get_unchecked(i) } as usize;
                    unsafe {
                        *ws.get_unchecked_mut(b * nc + ci) += 1;
                    }
                }
                let mut lcnts = [0u32; MAX_NC];
                let mut rcnts = cnts;
                let mut nl = 0usize;
                let mut bg = 0.0f64;
                let mut bb = 0usize;
                for b in 0..nb.saturating_sub(1) {
                    let row = unsafe { ws.get_unchecked(b * nc..(b + 1) * nc) };
                    let bn_row: u32 = row.iter().sum();
                    if bn_row == 0 {
                        continue;
                    }
                    for c in 0..nc {
                        lcnts[c] += row[c];
                        rcnts[c] -= row[c];
                    }
                    nl += bn_row as usize;
                    let nr = n - nl;
                    if nl < min_sl || nr < min_sl {
                        continue;
                    }
                    let il = gini_from_counts(&lcnts[..nc], nl);
                    let ir = gini_from_counts(&rcnts[..nc], nr);
                    let gain =
                        parent_impurity - (nl as f64 / n as f64) * il - (nr as f64 / n as f64) * ir;
                    if gain > bg {
                        bg = gain;
                        bb = b;
                    }
                }
                (bg, bb)
            })
        };

        if features.len() >= 8 && n >= 1000 {
            let results: Vec<(f64, usize, usize)> = features
                .par_iter()
                .map(|&feat| {
                    let (g, b) = scan_feat(feat);
                    (g, feat, b)
                })
                .collect();
            for (g, f, b) in results {
                if g > best_gain {
                    best_gain = g;
                    best_feat = f;
                    best_bin = b;
                }
            }
        } else {
            for &feat in features.iter() {
                let (g, b) = scan_feat(feat);
                if g > best_gain {
                    best_gain = g;
                    best_feat = feat;
                    best_bin = b;
                }
            }
        }

        if best_gain <= 0.0 {
            let dist: Vec<f64> = if self.need_dist {
                cnts[..nc].iter().map(|&v| v as f64).collect()
            } else {
                Vec::new()
            };
            self.nodes[idx].kind = NodeKind::Leaf {
                value: cnts[majority] as f64,
                class: self.classes[majority],
                dist,
            };
            return idx;
        }

        let best_thr = bins.edges[best_feat][best_bin];
        self.feature_importances[best_feat] += best_gain * n as f64;

        let split_bin = best_bin as u8;
        let col = &bins.binned[best_feat * bn..];
        let mut lo = 0usize;
        let mut hi = n;
        while lo < hi {
            if unsafe { *col.get_unchecked(indices[lo]) } <= split_bin {
                lo += 1;
            } else {
                hi -= 1;
                indices.swap(lo, hi);
            }
        }

        let (left_sl, right_sl) = indices.split_at_mut(lo);
        let left = self.build_node(bins, y, left_sl, depth + 1, rng);
        let right = self.build_node(bins, y, right_sl, depth + 1, rng);
        self.nodes[idx].kind = NodeKind::Split {
            feature: best_feat,
            threshold: best_thr,
            left,
            right,
        };
        idx
    }

    pub fn fit_weighted(&mut self, y: &[i32], bins: &BinInfo, weights: &[f64]) {
        let n = bins.n;
        let p = bins.p;
        let classes = crate::ml::linalg::discover_classes(y);
        self.classes = classes;
        self.n_classes = self.classes.len();
        let cmin = *self.classes.iter().min().unwrap_or(&0);
        let cmax = *self.classes.iter().max().unwrap_or(&0);
        let crange = (cmax - cmin + 1) as usize;
        let mut cmap = vec![0u8; crange];
        for (i, &c) in self.classes.iter().enumerate() {
            cmap[(c - cmin) as usize] = i as u8;
        }
        let y_idx: Vec<u8> = y.iter().map(|&v| cmap[(v - cmin) as usize]).collect();
        self.nodes.clear();
        self.feature_importances = vec![0.0; p];
        let mut rng = 0x123456789ABCDEFu64;
        let mut indices: Vec<usize> = (0..n).collect();
        self.build_node_weighted(bins, &y_idx, weights, &mut indices, 0, &mut rng);
        let sum: f64 = self.feature_importances.iter().sum();
        if sum > 0.0 {
            for v in &mut self.feature_importances {
                *v /= sum;
            }
        }
    }

    fn build_node_weighted(
        &mut self,
        bins: &BinInfo,
        y: &[u8],
        weights: &[f64],
        indices: &mut [usize],
        depth: usize,
        rng: &mut u64,
    ) -> usize {
        let nc = self.n_classes;
        let n = indices.len();
        let p = bins.p;
        let bn = bins.n;
        let idx = self.nodes.len();
        self.nodes.push(TreeNode {
            kind: NodeKind::Leaf {
                value: 0.0,
                class: 0,
                dist: Vec::new(),
            },
            n_samples: n,
        });

        let mut cwts = [0.0f64; MAX_NC];
        let mut total_w = 0.0f64;
        for &i in indices.iter() {
            let w = unsafe { *weights.get_unchecked(i) };
            unsafe {
                *cwts.get_unchecked_mut(*y.get_unchecked(i) as usize) += w;
            }
            total_w += w;
        }
        let majority = cwts[..nc]
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);

        if depth >= self.max_depth
            || n < self.min_samples_split
            || cwts[..nc].iter().filter(|&&v| v > 1e-15).count() <= 1
        {
            let dist: Vec<f64> = cwts[..nc].to_vec();
            self.nodes[idx].kind = NodeKind::Leaf {
                value: cwts[majority],
                class: self.classes[majority],
                dist,
            };
            return idx;
        }

        let max_f = self.max_features.unwrap_or(p);
        let mut feats_arr = [0usize; 64];
        let features: &[usize] = if max_f >= p {
            for i in 0..p {
                feats_arr[i] = i;
            }
            &feats_arr[..p]
        } else {
            let mut len = 0;
            while len < max_f {
                *rng ^= *rng << 13;
                *rng ^= *rng >> 7;
                *rng ^= *rng << 17;
                let f = *rng as usize % p;
                if !feats_arr[..len].contains(&f) {
                    feats_arr[len] = f;
                    len += 1;
                }
            }
            &feats_arr[..max_f]
        };

        let parent_impurity = gini_weighted(&cwts[..nc], total_w);
        let mut best_gain = 0.0;
        let mut best_feat = 0;
        let mut best_bin = 0usize;

        let scan_feat = |feat: usize| -> (f64, usize) {
            let nb = unsafe { *bins.n_bins.get_unchecked(feat) };
            CLS_WWORK.with(|ws| {
                let mut ws = ws.borrow_mut();
                let len = nb * nc;
                ws[..len].fill(0.0);
                let col_off = feat * bn;
                for &i in indices.iter() {
                    let b = unsafe { *bins.binned.get_unchecked(col_off + i) } as usize;
                    let ci = unsafe { *y.get_unchecked(i) } as usize;
                    let w = unsafe { *weights.get_unchecked(i) };
                    unsafe {
                        *ws.get_unchecked_mut(b * nc + ci) += w;
                    }
                }
                let mut lwts = [0.0f64; MAX_NC];
                let mut rwts = [0.0f64; MAX_NC];
                rwts[..nc].copy_from_slice(&cwts[..nc]);
                let mut wl = 0.0f64;
                let mut bg = 0.0f64;
                let mut bb = 0usize;
                for b in 0..nb.saturating_sub(1) {
                    let row = unsafe { ws.get_unchecked(b * nc..(b + 1) * nc) };
                    let bw: f64 = row[..nc].iter().sum();
                    if bw <= 0.0 {
                        continue;
                    }
                    for c in 0..nc {
                        lwts[c] += row[c];
                        rwts[c] -= row[c];
                    }
                    wl += bw;
                    let wr = total_w - wl;
                    if wl < 1e-15 || wr < 1e-15 {
                        continue;
                    }
                    let il = gini_weighted(&lwts[..nc], wl);
                    let ir = gini_weighted(&rwts[..nc], wr);
                    let gain = parent_impurity - (wl / total_w) * il - (wr / total_w) * ir;
                    if gain > bg {
                        bg = gain;
                        bb = b;
                    }
                }
                (bg, bb)
            })
        };

        if features.len() >= 8 && n >= 1000 {
            let results: Vec<(f64, usize, usize)> = features
                .par_iter()
                .map(|&feat| {
                    let (g, b) = scan_feat(feat);
                    (g, feat, b)
                })
                .collect();
            for (g, f, b) in results {
                if g > best_gain {
                    best_gain = g;
                    best_feat = f;
                    best_bin = b;
                }
            }
        } else {
            for &feat in features.iter() {
                let (g, b) = scan_feat(feat);
                if g > best_gain {
                    best_gain = g;
                    best_feat = feat;
                    best_bin = b;
                }
            }
        }

        if best_gain <= 0.0 {
            let dist: Vec<f64> = cwts[..nc].to_vec();
            self.nodes[idx].kind = NodeKind::Leaf {
                value: cwts[majority],
                class: self.classes[majority],
                dist,
            };
            return idx;
        }

        let best_thr = bins.edges[best_feat][best_bin];
        self.feature_importances[best_feat] += best_gain * total_w;

        let split_bin = best_bin as u8;
        let col = &bins.binned[best_feat * bn..];
        let mut lo = 0usize;
        let mut hi = n;
        while lo < hi {
            if unsafe { *col.get_unchecked(indices[lo]) } <= split_bin {
                lo += 1;
            } else {
                hi -= 1;
                indices.swap(lo, hi);
            }
        }

        let (left_sl, right_sl) = indices.split_at_mut(lo);
        let left = self.build_node_weighted(bins, y, weights, left_sl, depth + 1, rng);
        let right = self.build_node_weighted(bins, y, weights, right_sl, depth + 1, rng);
        self.nodes[idx].kind = NodeKind::Split {
            feature: best_feat,
            threshold: best_thr,
            left,
            right,
        };
        idx
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> {
        if n >= 256 {
            (0..n)
                .into_par_iter()
                .map(|i| {
                    let row = &x[i * p..(i + 1) * p];
                    self.predict_single(row)
                })
                .collect()
        } else {
            (0..n)
                .map(|i| {
                    let row = &x[i * p..(i + 1) * p];
                    self.predict_single(row)
                })
                .collect()
        }
    }

    pub fn predict_proba(&self, x: &[f64], n: usize, p: usize) -> Vec<Vec<f64>> {
        (0..n)
            .map(|i| {
                let row = &x[i * p..(i + 1) * p];
                self.predict_proba_single(row)
            })
            .collect()
    }

    pub fn predict_single(&self, row: &[f64]) -> i32 {
        let mut node = 0;
        loop {
            match &self.nodes[node].kind {
                NodeKind::Leaf { class, .. } => return *class,
                NodeKind::Split {
                    feature,
                    threshold,
                    left,
                    right,
                } => {
                    node = if row[*feature] <= *threshold {
                        *left
                    } else {
                        *right
                    };
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
                    return if sum > 0.0 {
                        dist.iter().map(|&v| v / sum).collect()
                    } else {
                        dist.clone()
                    };
                }
                NodeKind::Split {
                    feature,
                    threshold,
                    left,
                    right,
                } => {
                    node = if row[*feature] <= *threshold {
                        *left
                    } else {
                        *right
                    };
                }
            }
        }
    }

    pub fn leaf_dist(&self, row: &[f64]) -> &[f64] {
        let mut node = 0;
        loop {
            match &self.nodes[node].kind {
                NodeKind::Leaf { dist, .. } => return dist,
                NodeKind::Split {
                    feature,
                    threshold,
                    left,
                    right,
                } => {
                    node = if row[*feature] <= *threshold {
                        *left
                    } else {
                        *right
                    };
                }
            }
        }
    }

    pub fn n_classes(&self) -> usize {
        self.classes.len()
    }
}

#[derive(Clone)]
#[crate::model(category = "Tree-Based", domain = "ml")]
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
    Leaf {
        value: f64,
    },
    Split {
        feature: usize,
        threshold: f64,
        left: usize,
        right: usize,
    },
}

#[allow(dead_code)]
#[derive(Clone)]
struct TreeNodeReg {
    kind: TreeNodeRegKind,
    n_samples: usize,
}

impl DecisionTreeRegressor {
    pub fn new(
        max_depth: usize,
        min_samples_split: usize,
        min_samples_leaf: usize,
        max_features: Option<usize>,
    ) -> Self {
        Self {
            max_depth,
            min_samples_split,
            min_samples_leaf,
            max_features,
            nodes: Vec::new(),
            feature_importances: Vec::new(),
        }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) {
        let bins = compute_bins(x, n, p);
        self.fit_with_bins(y, &bins);
    }

    pub fn fit_with_presort(&mut self, x: &[f64], n: usize, p: usize, y: &[f64], _presort: bool) {
        self.fit(x, n, p, y);
    }

    pub fn fit_with_bins(&mut self, y: &[f64], bins: &BinInfo) {
        let n = bins.n;
        let p = bins.p;
        let mut indices: Vec<usize> = (0..n).collect();
        self.nodes.clear();
        self.feature_importances = vec![0.0; p];
        let mut rng = 0x123456789ABCDEFu64;
        self.build_node(bins, y, &mut indices, 0, &mut rng);
        let sum: f64 = self.feature_importances.iter().sum();
        if sum > 0.0 {
            for v in &mut self.feature_importances {
                *v /= sum;
            }
        }
    }

    fn build_node(
        &mut self,
        bins: &BinInfo,
        y: &[f64],
        indices: &mut [usize],
        depth: usize,
        rng: &mut u64,
    ) -> usize {
        let n = indices.len();
        let p = bins.p;
        let bn = bins.n;
        let idx = self.nodes.len();
        let (sum_y, sum_y2) = indices.iter().fold((0.0f64, 0.0f64), |(s, s2), &i| {
            let yi = unsafe { *y.get_unchecked(i) };
            (s + yi, s2 + yi * yi)
        });
        let mean_y = sum_y / n as f64;
        self.nodes.push(TreeNodeReg {
            kind: TreeNodeRegKind::Leaf { value: mean_y },
            n_samples: n,
        });

        if depth >= self.max_depth || n < self.min_samples_split {
            return idx;
        }
        let parent_mse = sum_y2 / n as f64 - mean_y * mean_y;
        if parent_mse < 1e-15 {
            return idx;
        }

        let max_f = self.max_features.unwrap_or(p);
        let mut feats_arr = [0usize; 64];
        let features: &[usize] = if max_f >= p {
            for i in 0..p {
                feats_arr[i] = i;
            }
            &feats_arr[..p]
        } else {
            let mut len = 0;
            while len < max_f {
                *rng ^= *rng << 13;
                *rng ^= *rng >> 7;
                *rng ^= *rng << 17;
                let f = *rng as usize % p;
                if !feats_arr[..len].contains(&f) {
                    feats_arr[len] = f;
                    len += 1;
                }
            }
            &feats_arr[..max_f]
        };

        let mut best_gain = 0.0;
        let mut best_feat = 0;
        let mut best_bin = 0usize;
        let min_sl = self.min_samples_leaf;
        let indices_ro: &[usize] = &*indices;

        let scan_feat = |feat: usize| -> (f64, usize) {
            let nb = unsafe { *bins.n_bins.get_unchecked(feat) };
            REG_WORK.with(|ws| {
                let mut ws = ws.borrow_mut();
                let (bin_sum, bin_sq, bin_cnt) = &mut *ws;
                bin_sum[..nb].fill(0.0);
                bin_sq[..nb].fill(0.0);
                bin_cnt[..nb].fill(0);
                let col_off = feat * bn;
                for &i in indices_ro.iter() {
                    let b = unsafe { *bins.binned.get_unchecked(col_off + i) } as usize;
                    let yi = unsafe { *y.get_unchecked(i) };
                    unsafe {
                        *bin_sum.get_unchecked_mut(b) += yi;
                        *bin_sq.get_unchecked_mut(b) += yi * yi;
                        *bin_cnt.get_unchecked_mut(b) += 1;
                    }
                }
                let mut sl = 0.0f64;
                let mut ssl = 0.0f64;
                let mut nl = 0usize;
                let mut bg = 0.0f64;
                let mut bb = 0usize;
                for b in 0..nb.saturating_sub(1) {
                    sl += bin_sum[b];
                    ssl += bin_sq[b];
                    nl += bin_cnt[b] as usize;
                    if nl == 0 {
                        continue;
                    }
                    let nr = n - nl;
                    if nl < min_sl || nr < min_sl {
                        continue;
                    }
                    let sr = sum_y - sl;
                    let ssr = sum_y2 - ssl;
                    let ml = sl / nl as f64;
                    let mr = sr / nr as f64;
                    let gl = ssl / nl as f64 - ml * ml;
                    let gr = ssr / nr as f64 - mr * mr;
                    let gain =
                        parent_mse - (nl as f64 / n as f64) * gl - (nr as f64 / n as f64) * gr;
                    if gain > bg {
                        bg = gain;
                        bb = b;
                    }
                }
                (bg, bb)
            })
        };

        if features.len() >= 8 && n >= 1000 {
            let results: Vec<(f64, usize, usize)> = features
                .par_iter()
                .map(|&feat| {
                    let (g, b) = scan_feat(feat);
                    (g, feat, b)
                })
                .collect();
            for (g, f, b) in results {
                if g > best_gain {
                    best_gain = g;
                    best_feat = f;
                    best_bin = b;
                }
            }
        } else {
            for &feat in features.iter() {
                let (g, b) = scan_feat(feat);
                if g > best_gain {
                    best_gain = g;
                    best_feat = feat;
                    best_bin = b;
                }
            }
        }

        if best_gain <= 0.0 {
            return idx;
        }

        let best_thr = bins.edges[best_feat][best_bin];
        self.feature_importances[best_feat] += best_gain * n as f64;

        let split_bin = best_bin as u8;
        let col = &bins.binned[best_feat * bn..];
        let mut lo = 0usize;
        let mut hi = n;
        while lo < hi {
            if unsafe { *col.get_unchecked(indices[lo]) } <= split_bin {
                lo += 1;
            } else {
                hi -= 1;
                indices.swap(lo, hi);
            }
        }

        let (left_sl, right_sl) = indices.split_at_mut(lo);
        let left = self.build_node(bins, y, left_sl, depth + 1, rng);
        let right = self.build_node(bins, y, right_sl, depth + 1, rng);
        self.nodes[idx].kind = TreeNodeRegKind::Split {
            feature: best_feat,
            threshold: best_thr,
            left,
            right,
        };
        idx
    }

    pub fn predict_single(&self, row: &[f64]) -> f64 {
        let mut node = 0;
        loop {
            match &self.nodes[node].kind {
                TreeNodeRegKind::Leaf { value } => return *value,
                TreeNodeRegKind::Split {
                    feature,
                    threshold,
                    left,
                    right,
                } => {
                    node = if row[*feature] <= *threshold {
                        *left
                    } else {
                        *right
                    };
                }
            }
        }
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        if n >= 256 {
            (0..n)
                .into_par_iter()
                .map(|i| self.predict_single(&x[i * p..(i + 1) * p]))
                .collect()
        } else {
            (0..n)
                .map(|i| self.predict_single(&x[i * p..(i + 1) * p]))
                .collect()
        }
    }

    pub fn update_leaves_newton(
        &mut self,
        x: &[f64],
        n: usize,
        p: usize,
        grad: &[f64],
        hess: &[f64],
    ) {
        let nn = self.nodes.len();
        let mut gs = vec![0.0; nn];
        let mut hs = vec![0.0; nn];
        for i in 0..n {
            let leaf = self.leaf_of(&x[i * p..(i + 1) * p]);
            gs[leaf] += grad[i];
            hs[leaf] += hess[i];
        }
        for idx in 0..nn {
            if let TreeNodeRegKind::Leaf { ref mut value } = self.nodes[idx].kind {
                if hs[idx] > 1e-10 {
                    *value = gs[idx] / hs[idx];
                }
            }
        }
    }

    fn leaf_of(&self, row: &[f64]) -> usize {
        let mut node = 0;
        loop {
            match &self.nodes[node].kind {
                TreeNodeRegKind::Leaf { .. } => return node,
                TreeNodeRegKind::Split {
                    feature,
                    threshold,
                    left,
                    right,
                } => {
                    node = if row[*feature] <= *threshold {
                        *left
                    } else {
                        *right
                    };
                }
            }
        }
    }
}

impl crate::ml::MlClassifier for DecisionTreeClassifier {
    fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) {
        self.fit(x, n, p, y);
    }
    fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> {
        self.predict(x, n, p)
    }
}

impl crate::ml::MlRegressor for DecisionTreeRegressor {
    fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) {
        self.fit(x, n, p, y);
    }
    fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        self.predict(x, n, p)
    }
}

#[inline(always)]
fn gini_from_counts(cnts: &[u32], n: usize) -> f64 {
    let inv = 1.0 / n as f64;
    let mut s = 1.0;
    for &c in cnts {
        let p = c as f64 * inv;
        s -= p * p;
    }
    s
}

#[inline(always)]
fn gini_weighted(cnts: &[f64], total: f64) -> f64 {
    if total <= 0.0 {
        return 0.0;
    }
    let inv = 1.0 / total;
    let mut s = 1.0;
    for &c in cnts {
        let p = c * inv;
        s -= p * p;
    }
    s
}
