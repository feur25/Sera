use rayon::prelude::*;

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

const BT_LEAF: usize = 40;
const TREE_THRESH: usize = 5000;
const TREE_MAX_DIM: usize = 12;

struct BtNode {
    radius: f64,
    left: u32,
    right: u32,
    start: u32,
    end: u32,
}

struct BallTree {
    nodes: Vec<BtNode>,
    centers: Vec<f64>,
    perm: Vec<u32>,
    perm_data: Vec<f64>,
    p: usize,
}

impl BallTree {
    fn build(data: &[f64], n: usize, p: usize) -> Self {
        let mut perm: Vec<u32> = (0..n as u32).collect();
        let est = 2 * n / BT_LEAF + 4;
        let mut nodes = Vec::with_capacity(est);
        let mut centers = Vec::with_capacity(est * p);
        Self::build_rec(data, &mut perm, 0, n, p, &mut nodes, &mut centers);
        let mut perm_data = vec![0.0f64; n * p];
        for (i, &idx) in perm.iter().enumerate() {
            perm_data[i * p..(i + 1) * p].copy_from_slice(&data[idx as usize * p..(idx as usize + 1) * p]);
        }
        BallTree { nodes, centers, perm, perm_data, p }
    }

    fn build_rec(data: &[f64], perm: &mut [u32], s: usize, e: usize, p: usize, nodes: &mut Vec<BtNode>, centers: &mut Vec<f64>) -> u32 {
        let ni = nodes.len() as u32;
        let count = e - s;
        let co = centers.len();
        centers.extend(std::iter::repeat(0.0).take(p));
        for i in s..e {
            let idx = perm[i] as usize;
            for j in 0..p { centers[co + j] += data[idx * p + j]; }
        }
        let inv = 1.0 / count as f64;
        for j in 0..p { centers[co + j] *= inv; }
        let mut radius_sq = 0.0f64;
        for i in s..e {
            let idx = perm[i] as usize;
            let mut d = 0.0;
            for j in 0..p { let diff = data[idx * p + j] - centers[co + j]; d += diff * diff; }
            if d > radius_sq { radius_sq = d; }
        }
        if count <= BT_LEAF {
            nodes.push(BtNode { radius: radius_sq.sqrt(), left: u32::MAX, right: u32::MAX, start: s as u32, end: e as u32 });
            return ni;
        }
        let mut best_dim = 0;
        let mut best_spread = 0.0f64;
        for j in 0..p {
            let mut lo = f64::MAX;
            let mut hi = f64::MIN;
            for i in s..e {
                let v = data[perm[i] as usize * p + j];
                if v < lo { lo = v; }
                if v > hi { hi = v; }
            }
            if hi - lo > best_spread { best_spread = hi - lo; best_dim = j; }
        }
        let mid = s + count / 2;
        perm[s..e].select_nth_unstable_by(mid - s, |&a, &b| {
            data[a as usize * p + best_dim].partial_cmp(&data[b as usize * p + best_dim]).unwrap()
        });
        nodes.push(BtNode { radius: radius_sq.sqrt(), left: 0, right: 0, start: s as u32, end: e as u32 });
        let left = Self::build_rec(data, perm, s, mid, p, nodes, centers);
        let right = Self::build_rec(data, perm, mid, e, p, nodes, centers);
        nodes[ni as usize].left = left;
        nodes[ni as usize].right = right;
        ni
    }

    #[inline]
    fn query_k(&self, q: &[f64], k: usize, heap: &mut Vec<(f64, u32)>) {
        heap.clear();
        self.query_rec(q, 0, k, heap);
    }

    fn query_rec(&self, q: &[f64], ni: u32, k: usize, heap: &mut Vec<(f64, u32)>) {
        let node = &self.nodes[ni as usize];
        let p = self.p;
        let co = ni as usize * p;
        let mut dc = 0.0;
        for j in 0..p { let d = q[j] - self.centers[co + j]; dc += d * d; }
        if heap.len() >= k {
            let bound = heap[0].0.sqrt() + node.radius;
            if dc >= bound * bound { return; }
        }
        if node.left == u32::MAX {
            for i in node.start..node.end {
                let i_us = i as usize;
                let d = dist_sq(q, unsafe { self.perm_data.get_unchecked(i_us * p..(i_us + 1) * p) }, p);
                if heap.len() < k {
                    heap.push((d, i));
                    if heap.len() == k { heap_build(heap); }
                } else if d < heap[0].0 {
                    heap[0] = (d, i);
                    heap_sift_down(heap, 0);
                }
            }
            return;
        }
        let lco = node.left as usize * p;
        let rco = node.right as usize * p;
        let mut dl = 0.0;
        let mut dr = 0.0;
        for j in 0..p {
            let ld = q[j] - self.centers[lco + j]; dl += ld * ld;
            let rd = q[j] - self.centers[rco + j]; dr += rd * rd;
        }
        if dl <= dr {
            self.query_rec(q, node.left, k, heap);
            self.query_rec(q, node.right, k, heap);
        } else {
            self.query_rec(q, node.right, k, heap);
            self.query_rec(q, node.left, k, heap);
        }
    }
}

#[inline]
fn heap_build(h: &mut [(f64, u32)]) {
    let n = h.len();
    for i in (0..n / 2).rev() { heap_sift_down(h, i); }
}

#[inline]
fn heap_sift_down(h: &mut [(f64, u32)], mut pos: usize) {
    let n = h.len();
    loop {
        let l = 2 * pos + 1;
        let r = 2 * pos + 2;
        let mut largest = pos;
        if l < n && h[l].0 > h[largest].0 { largest = l; }
        if r < n && h[r].0 > h[largest].0 { largest = r; }
        if largest == pos { break; }
        h.swap(pos, largest);
        pos = largest;
    }
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
    tree: Option<BallTree>,
}

impl KNeighborsClassifier {
    pub fn new(k: usize, weights: KnnWeights) -> Self {
        Self { k, weights, data: Vec::new(), labels: Vec::new(), label_idx: Vec::new(), n: 0, p: 0, classes: Vec::new(), tree: None }
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
        if n >= TREE_THRESH && p <= TREE_MAX_DIM {
            self.tree = Some(BallTree::build(x, n, p));
        } else {
            self.tree = None;
        }
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> {
        let k = self.k;
        let nc = self.classes.len();
        let use_dist = matches!(self.weights, KnnWeights::Distance);

        if let Some(ref tree) = self.tree {
            let predict_one = |i: usize| -> i32 {
                let xi = &x[i * p..(i + 1) * p];
                let mut heap: Vec<(f64, u32)> = Vec::with_capacity(k + 1);
                tree.query_k(xi, k, &mut heap);
                if use_dist {
                    let mut wts = [0.0f64; 256];
                    for &(d, idx) in heap.iter() {
                        let dist = d.sqrt().max(1e-10);
                        wts[self.label_idx[tree.perm[idx as usize] as usize] as usize] += 1.0 / dist;
                    }
                    let mut best = 0;
                    let mut best_w = 0.0;
                    for c in 0..nc { if wts[c] > best_w { best_w = wts[c]; best = c; } }
                    self.classes[best]
                } else {
                    let mut counts = [0u32; 256];
                    for &(_, idx) in heap.iter() {
                        counts[self.label_idx[tree.perm[idx as usize] as usize] as usize] += 1;
                    }
                    let mut best = 0;
                    let mut best_c = 0u32;
                    for c in 0..nc { if counts[c] > best_c { best_c = counts[c]; best = c; } }
                    self.classes[best]
                }
            };
            if n >= 64 {
                (0..n).into_par_iter().map(predict_one).collect()
            } else {
                (0..n).map(predict_one).collect()
            }
        } else {
            let p_ = self.p;
            let n_train = self.n;
            let predict_one = |i: usize| -> i32 {
                let xi = &x[i * p..(i + 1) * p];
                let mut heap: Vec<(f64, u32)> = Vec::with_capacity(k + 1);
                for j in 0..n_train {
                    let d = dist_sq(xi, unsafe { self.data.get_unchecked(j * p_..(j + 1) * p_) }, p_);
                    if heap.len() < k {
                        heap.push((d, j as u32));
                        if heap.len() == k { heap_build(&mut heap); }
                    } else if d < heap[0].0 {
                        heap[0] = (d, j as u32);
                        heap_sift_down(&mut heap, 0);
                    }
                }
                if use_dist {
                    let mut wts = [0.0f64; 256];
                    for &(d, idx) in heap.iter() {
                        let dist = d.sqrt().max(1e-10);
                        wts[self.label_idx[idx as usize] as usize] += 1.0 / dist;
                    }
                    let mut best = 0;
                    let mut best_w = 0.0;
                    for c in 0..nc { if wts[c] > best_w { best_w = wts[c]; best = c; } }
                    self.classes[best]
                } else {
                    let mut counts = [0u32; 256];
                    for &(_, idx) in heap.iter() {
                        counts[self.label_idx[idx as usize] as usize] += 1;
                    }
                    let mut best = 0;
                    let mut best_c = 0u32;
                    for c in 0..nc { if counts[c] > best_c { best_c = counts[c]; best = c; } }
                    self.classes[best]
                }
            };
            if n >= 64 {
                (0..n).into_par_iter().map(predict_one).collect()
            } else {
                (0..n).map(predict_one).collect()
            }
        }
    }

    pub fn predict_proba(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let k = self.k;
        let nc = self.classes.len();
        let inv = 1.0 / k as f64;
        let use_dist = matches!(self.weights, KnnWeights::Distance);

        if let Some(ref tree) = self.tree {
            let proba_one = |i: usize| -> Vec<f64> {
                let xi = &x[i * p..(i + 1) * p];
                let mut heap: Vec<(f64, u32)> = Vec::with_capacity(k + 1);
                tree.query_k(xi, k, &mut heap);
                let mut probs = vec![0.0; nc];
                if use_dist {
                    let mut wsum = 0.0;
                    for &(d, idx) in heap.iter() {
                        let dist = d.sqrt().max(1e-10);
                        let w = 1.0 / dist;
                        probs[self.label_idx[tree.perm[idx as usize] as usize] as usize] += w;
                        wsum += w;
                    }
                    let inv_w = 1.0 / wsum;
                    for v in probs.iter_mut() { *v *= inv_w; }
                } else {
                    for &(_, idx) in heap.iter() {
                        probs[self.label_idx[tree.perm[idx as usize] as usize] as usize] += inv;
                    }
                }
                probs
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
        } else {
            let p_ = self.p;
            let n_train = self.n;
            let proba_one = |i: usize| -> Vec<f64> {
                let xi = &x[i * p..(i + 1) * p];
                let mut heap: Vec<(f64, u32)> = Vec::with_capacity(k + 1);
                for j in 0..n_train {
                    let d = dist_sq(xi, unsafe { self.data.get_unchecked(j * p_..(j + 1) * p_) }, p_);
                    if heap.len() < k {
                        heap.push((d, j as u32));
                        if heap.len() == k { heap_build(&mut heap); }
                    } else if d < heap[0].0 {
                        heap[0] = (d, j as u32);
                        heap_sift_down(&mut heap, 0);
                    }
                }
                let mut probs = vec![0.0; nc];
                if use_dist {
                    let mut wsum = 0.0;
                    for &(d, idx) in heap.iter() {
                        let dist = d.sqrt().max(1e-10);
                        let w = 1.0 / dist;
                        probs[self.label_idx[idx as usize] as usize] += w;
                        wsum += w;
                    }
                    let inv_w = 1.0 / wsum;
                    for v in probs.iter_mut() { *v *= inv_w; }
                } else {
                    for &(_, idx) in heap.iter() { probs[self.label_idx[idx as usize] as usize] += inv; }
                }
                probs
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
}

pub struct KNeighborsRegressor {
    pub k: usize,
    pub weights: KnnWeights,
    data: Vec<f64>,
    targets: Vec<f64>,
    n: usize,
    p: usize,
    tree: Option<BallTree>,
}

impl KNeighborsRegressor {
    pub fn new(k: usize, weights: KnnWeights) -> Self {
        Self { k, weights, data: Vec::new(), targets: Vec::new(), n: 0, p: 0, tree: None }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) {
        self.data = x.to_vec();
        self.targets = y.to_vec();
        self.n = n;
        self.p = p;
        if n >= TREE_THRESH && p <= TREE_MAX_DIM {
            self.tree = Some(BallTree::build(x, n, p));
        } else {
            self.tree = None;
        }
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let k = self.k;
        let use_dist = matches!(self.weights, KnnWeights::Distance);

        if let Some(ref tree) = self.tree {
            let predict_one = |i: usize| -> f64 {
                let xi = &x[i * p..(i + 1) * p];
                let mut heap: Vec<(f64, u32)> = Vec::with_capacity(k + 1);
                tree.query_k(xi, k, &mut heap);
                if use_dist {
                    let mut wsum = 0.0;
                    let mut vsum = 0.0;
                    for &(d, idx) in heap.iter() {
                        let dist = d.sqrt().max(1e-10);
                        let w = 1.0 / dist;
                        vsum += w * self.targets[tree.perm[idx as usize] as usize];
                        wsum += w;
                    }
                    vsum / wsum
                } else {
                    let mut sum = 0.0;
                    for &(_, idx) in heap.iter() { sum += self.targets[tree.perm[idx as usize] as usize]; }
                    sum / k as f64
                }
            };
            if n >= 64 {
                (0..n).into_par_iter().map(predict_one).collect()
            } else {
                (0..n).map(predict_one).collect()
            }
        } else {
            let p_ = self.p;
            let n_train = self.n;
            let predict_one = |i: usize| -> f64 {
                let xi = &x[i * p..(i + 1) * p];
                let mut heap: Vec<(f64, u32)> = Vec::with_capacity(k + 1);
                let k_ = k.min(n_train);
                for j in 0..n_train {
                    let d = dist_sq(xi, unsafe { self.data.get_unchecked(j * p_..(j + 1) * p_) }, p_);
                    if heap.len() < k_ {
                        heap.push((d, j as u32));
                        if heap.len() == k_ { heap_build(&mut heap); }
                    } else if d < heap[0].0 {
                        heap[0] = (d, j as u32);
                        heap_sift_down(&mut heap, 0);
                    }
                }
                if use_dist {
                    let mut wsum = 0.0;
                    let mut vsum = 0.0;
                    for &(d, idx) in heap.iter() {
                        let dist = d.sqrt().max(1e-10);
                        let w = 1.0 / dist;
                        vsum += w * self.targets[idx as usize];
                        wsum += w;
                    }
                    vsum / wsum
                } else {
                    let mut sum = 0.0;
                    for &(_, idx) in heap.iter() { sum += self.targets[idx as usize]; }
                    sum / heap.len() as f64
                }
            };
            if n >= 64 {
                (0..n).into_par_iter().map(predict_one).collect()
            } else {
                (0..n).map(predict_one).collect()
            }
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

impl crate::ml::MlClassifier for KNeighborsClassifier {
    fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) { self.fit(x, n, p, y); }
    fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> { self.predict(x, n, p) }
}

impl crate::ml::MlRegressor for KNeighborsRegressor {
    fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) { self.fit(x, n, p, y); }
    fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> { self.predict(x, n, p) }
}
