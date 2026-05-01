pub fn compute_bins(values: &[f64], n_bins: usize) -> (Vec<u64>, Vec<f64>) {
    if values.is_empty() { return (vec![], vec![]); }
    let (min, max) = crate::bindings::utils::simd_ops::find_minmax(values);
    if !min.is_finite() || !max.is_finite() { return (vec![], vec![]); }
    let n = if n_bins == 0 {
        ((values.len() as f64).log2().ceil() as usize + 1).clamp(5, 128)
    } else {
        n_bins.clamp(2, 512)
    };
    let range = (max - min).max(1e-12);
    let step = range / n as f64;
    let inv_step = 1.0 / step;
    let mut edges = Vec::with_capacity(n + 1);
    for i in 0..=n { edges.push(min + i as f64 * step); }
    let mut counts = vec![0u64; n];
    for &v in values {
        if !v.is_finite() { continue; }
        let idx = ((v - min) * inv_step) as usize;
        counts[idx.min(n - 1)] += 1;
    }
    (counts, edges)
}

pub fn bin_to_edges(values: &[f64], edges: &[f64]) -> (Vec<u64>, Vec<f64>) {
    let n = edges.len().saturating_sub(1);
    let mut counts = vec![0u64; n];
    if n == 0 { return (counts, edges.to_vec()); }
    let min = edges[0];
    let max = *edges.last().unwrap();
    let range = (max - min).max(1e-12);
    let step = range / n as f64;
    for &v in values.iter() {
        if !v.is_finite() { continue; }
        let idx = ((v - min) / step) as usize;
        counts[idx.min(n - 1)] += 1;
    }
    (counts, edges.to_vec())
}

pub fn group_indices(categories: &[String], n: usize) -> (Vec<String>, Vec<usize>) {
    let mut order: Vec<String> = Vec::new();
    let mut idx: Vec<usize> = vec![0; n];
    for i in 0..n {
        let c = if i < categories.len() { categories[i].as_str() } else { "" };
        let pos = order.iter().position(|x| x == c);
        let p = match pos { Some(p) => p, None => { order.push(c.to_string()); order.len() - 1 } };
        idx[i] = p;
    }
    (order, idx)
}
