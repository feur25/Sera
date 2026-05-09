use rayon::prelude::*;

fn pair_dist(x: &[f64], i: usize, j: usize, p: usize) -> f64 {
    let mut s = 0.0;
    for k in 0..p {
        let d = x[i * p + k] - x[j * p + k];
        s += d * d;
    }
    s.sqrt()
}

pub fn silhouette_score(x: &[f64], labels: &[i32], n: usize, p: usize) -> f64 {
    if n < 2 { return 0.0; }
    let mut classes: Vec<i32> = labels.to_vec();
    classes.sort_unstable();
    classes.dedup();
    if classes.len() < 2 { return 0.0; }
    let buckets: Vec<Vec<usize>> = classes.iter().map(|&c| {
        labels.iter().enumerate().filter(|(_, &l)| l == c).map(|(i, _)| i).collect()
    }).collect();
    let scores: Vec<f64> = (0..n).into_par_iter().map(|i| {
        let li = labels[i];
        let own_idx = classes.iter().position(|&c| c == li).unwrap();
        let own = &buckets[own_idx];
        if own.len() < 2 { return 0.0; }
        let a: f64 = own.iter().filter(|&&j| j != i).map(|&j| pair_dist(x, i, j, p)).sum::<f64>() / (own.len() - 1) as f64;
        let mut b = f64::INFINITY;
        for (k, b_idx) in buckets.iter().enumerate() {
            if k == own_idx || b_idx.is_empty() { continue; }
            let mean: f64 = b_idx.iter().map(|&j| pair_dist(x, i, j, p)).sum::<f64>() / b_idx.len() as f64;
            if mean < b { b = mean; }
        }
        if b.is_infinite() { return 0.0; }
        let m = a.max(b);
        if m < 1e-15 { 0.0 } else { (b - a) / m }
    }).collect();
    scores.iter().sum::<f64>() / n as f64
}

pub fn davies_bouldin_score(x: &[f64], labels: &[i32], n: usize, p: usize) -> f64 {
    let mut classes: Vec<i32> = labels.to_vec();
    classes.sort_unstable();
    classes.dedup();
    let k = classes.len();
    if k < 2 { return 0.0; }
    let mut centroids = vec![0.0f64; k * p];
    let mut counts = vec![0usize; k];
    for i in 0..n {
        let ci = classes.iter().position(|&c| c == labels[i]).unwrap();
        for j in 0..p { centroids[ci * p + j] += x[i * p + j]; }
        counts[ci] += 1;
    }
    for c in 0..k {
        if counts[c] == 0 { continue; }
        for j in 0..p { centroids[c * p + j] /= counts[c] as f64; }
    }
    let mut s = vec![0.0f64; k];
    for i in 0..n {
        let ci = classes.iter().position(|&c| c == labels[i]).unwrap();
        let mut d = 0.0;
        for j in 0..p {
            let v = x[i * p + j] - centroids[ci * p + j];
            d += v * v;
        }
        s[ci] += d.sqrt();
    }
    for c in 0..k { if counts[c] > 0 { s[c] /= counts[c] as f64; } }
    let mut acc = 0.0;
    for i in 0..k {
        let mut best = 0.0f64;
        for j in 0..k {
            if i == j { continue; }
            let mut m = 0.0;
            for d in 0..p {
                let v = centroids[i * p + d] - centroids[j * p + d];
                m += v * v;
            }
            let m = m.sqrt();
            if m < 1e-15 { continue; }
            let r = (s[i] + s[j]) / m;
            if r > best { best = r; }
        }
        acc += best;
    }
    acc / k as f64
}

pub fn calinski_harabasz_score(x: &[f64], labels: &[i32], n: usize, p: usize) -> f64 {
    let mut classes: Vec<i32> = labels.to_vec();
    classes.sort_unstable();
    classes.dedup();
    let k = classes.len();
    if k < 2 || n <= k { return 0.0; }
    let mut centroids = vec![0.0f64; k * p];
    let mut counts = vec![0usize; k];
    let mut overall = vec![0.0f64; p];
    for i in 0..n {
        let ci = classes.iter().position(|&c| c == labels[i]).unwrap();
        for j in 0..p {
            centroids[ci * p + j] += x[i * p + j];
            overall[j] += x[i * p + j];
        }
        counts[ci] += 1;
    }
    for j in 0..p { overall[j] /= n as f64; }
    for c in 0..k {
        if counts[c] == 0 { continue; }
        for j in 0..p { centroids[c * p + j] /= counts[c] as f64; }
    }
    let mut bcss = 0.0;
    for c in 0..k {
        let mut d = 0.0;
        for j in 0..p {
            let v = centroids[c * p + j] - overall[j];
            d += v * v;
        }
        bcss += counts[c] as f64 * d;
    }
    let mut wcss = 0.0;
    for i in 0..n {
        let ci = classes.iter().position(|&c| c == labels[i]).unwrap();
        for j in 0..p {
            let v = x[i * p + j] - centroids[ci * p + j];
            wcss += v * v;
        }
    }
    if wcss < 1e-15 { return 0.0; }
    (bcss / (k - 1) as f64) / (wcss / (n - k) as f64)
}

pub fn adjusted_rand_score(labels_true: &[i32], labels_pred: &[i32]) -> f64 {
    let n = labels_true.len().min(labels_pred.len());
    if n < 2 { return 0.0; }
    let mut t: Vec<i32> = labels_true.to_vec(); t.sort_unstable(); t.dedup();
    let mut p: Vec<i32> = labels_pred.to_vec(); p.sort_unstable(); p.dedup();
    let r = t.len();
    let c = p.len();
    let mut mat = vec![0u64; r * c];
    for i in 0..n {
        let a = t.iter().position(|&v| v == labels_true[i]).unwrap();
        let b = p.iter().position(|&v| v == labels_pred[i]).unwrap();
        mat[a * c + b] += 1;
    }
    let comb2 = |x: u64| -> f64 { if x < 2 { 0.0 } else { (x * (x - 1) / 2) as f64 } };
    let row_sums: Vec<u64> = (0..r).map(|i| (0..c).map(|j| mat[i * c + j]).sum()).collect();
    let col_sums: Vec<u64> = (0..c).map(|j| (0..r).map(|i| mat[i * c + j]).sum()).collect();
    let sum_comb: f64 = mat.iter().map(|&v| comb2(v)).sum();
    let sum_a: f64 = row_sums.iter().map(|&v| comb2(v)).sum();
    let sum_b: f64 = col_sums.iter().map(|&v| comb2(v)).sum();
    let total = comb2(n as u64);
    let expected = sum_a * sum_b / total.max(1e-15);
    let max_idx = 0.5 * (sum_a + sum_b);
    if (max_idx - expected).abs() < 1e-15 { 0.0 } else { (sum_comb - expected) / (max_idx - expected) }
}

pub fn normalized_mutual_info_score(labels_true: &[i32], labels_pred: &[i32]) -> f64 {
    let n = labels_true.len().min(labels_pred.len());
    if n == 0 { return 0.0; }
    let mut t: Vec<i32> = labels_true.to_vec(); t.sort_unstable(); t.dedup();
    let mut p: Vec<i32> = labels_pred.to_vec(); p.sort_unstable(); p.dedup();
    let r = t.len();
    let c = p.len();
    let mut mat = vec![0u64; r * c];
    let mut ru = vec![0u64; r];
    let mut cu = vec![0u64; c];
    for i in 0..n {
        let a = t.iter().position(|&v| v == labels_true[i]).unwrap();
        let b = p.iter().position(|&v| v == labels_pred[i]).unwrap();
        mat[a * c + b] += 1;
        ru[a] += 1;
        cu[b] += 1;
    }
    let nf = n as f64;
    let mut mi = 0.0;
    for i in 0..r {
        for j in 0..c {
            let nij = mat[i * c + j] as f64;
            if nij == 0.0 { continue; }
            mi += (nij / nf) * (nij * nf / (ru[i] as f64 * cu[j] as f64)).ln();
        }
    }
    let h = |counts: &[u64]| -> f64 {
        let mut s = 0.0;
        for &v in counts {
            if v == 0 { continue; }
            let p = v as f64 / nf;
            s -= p * p.ln();
        }
        s
    };
    let ht = h(&ru);
    let hp = h(&cu);
    let denom = (ht * hp).sqrt();
    if denom < 1e-15 { 0.0 } else { mi / denom }
}

pub fn fowlkes_mallows_score(labels_true: &[i32], labels_pred: &[i32]) -> f64 {
    let n = labels_true.len().min(labels_pred.len());
    if n < 2 { return 0.0; }
    let mut tp = 0u64;
    let mut fp = 0u64;
    let mut fn_ = 0u64;
    for i in 0..n {
        for j in (i + 1)..n {
            let same_t = labels_true[i] == labels_true[j];
            let same_p = labels_pred[i] == labels_pred[j];
            match (same_t, same_p) {
                (true, true) => tp += 1,
                (false, true) => fp += 1,
                (true, false) => fn_ += 1,
                _ => {}
            }
        }
    }
    if tp == 0 { return 0.0; }
    let denom = ((tp + fp) as f64 * (tp + fn_) as f64).sqrt();
    if denom < 1e-15 { 0.0 } else { tp as f64 / denom }
}

pub fn homogeneity_score(labels_true: &[i32], labels_pred: &[i32]) -> f64 {
    nmi_homogeneity(labels_true, labels_pred).0
}

pub fn completeness_score(labels_true: &[i32], labels_pred: &[i32]) -> f64 {
    nmi_homogeneity(labels_true, labels_pred).1
}

pub fn v_measure_score(labels_true: &[i32], labels_pred: &[i32]) -> f64 {
    let (h, c) = nmi_homogeneity(labels_true, labels_pred);
    if h + c < 1e-15 { 0.0 } else { 2.0 * h * c / (h + c) }
}

fn nmi_homogeneity(labels_true: &[i32], labels_pred: &[i32]) -> (f64, f64) {
    let n = labels_true.len().min(labels_pred.len());
    if n == 0 { return (0.0, 0.0); }
    let mut t: Vec<i32> = labels_true.to_vec(); t.sort_unstable(); t.dedup();
    let mut p: Vec<i32> = labels_pred.to_vec(); p.sort_unstable(); p.dedup();
    let r = t.len();
    let c = p.len();
    let mut mat = vec![0u64; r * c];
    let mut ru = vec![0u64; r];
    let mut cu = vec![0u64; c];
    for i in 0..n {
        let a = t.iter().position(|&v| v == labels_true[i]).unwrap();
        let b = p.iter().position(|&v| v == labels_pred[i]).unwrap();
        mat[a * c + b] += 1;
        ru[a] += 1;
        cu[b] += 1;
    }
    let nf = n as f64;
    let h = |counts: &[u64]| -> f64 {
        let mut s = 0.0;
        for &v in counts {
            if v == 0 { continue; }
            let p = v as f64 / nf;
            s -= p * p.ln();
        }
        s
    };
    let ht = h(&ru);
    let hp = h(&cu);
    let mut mi = 0.0;
    for i in 0..r {
        for j in 0..c {
            let nij = mat[i * c + j] as f64;
            if nij == 0.0 { continue; }
            mi += (nij / nf) * (nij * nf / (ru[i] as f64 * cu[j] as f64)).ln();
        }
    }
    let homo = if ht < 1e-15 { 1.0 } else { mi / ht };
    let comp = if hp < 1e-15 { 1.0 } else { mi / hp };
    (homo, comp)
}


