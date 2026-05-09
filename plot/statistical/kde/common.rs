use super::config::KdeConfig;
use crate::plot::statistical::common::{sort_indices, sorted};

pub fn scott_bw(vals: &[f64]) -> f64 {
    let n = vals.len();
    if n < 2 { return 1.0; }
    let mean = vals.iter().sum::<f64>() / n as f64;
    let var = vals.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1) as f64;
    1.06 * var.sqrt() * (n as f64).powf(-0.2)
}

pub fn kde_eval(vals: &[f64], x: f64, bw: f64) -> f64 {
    if vals.is_empty() || bw <= 0.0 { return 0.0; }
    let inv_n_bw = 1.0 / (bw * vals.len() as f64);
    let c = 0.3989422804014327 * inv_n_bw;
    let mut sum = 0.0f64;
    for &v in vals {
        let u = (x - v) / bw;
        let uu = u * u;
        if uu < 16.0 { sum += c * (-0.5 * uu).exp(); }
    }
    sum
}

pub fn ordered_series(cfg: &KdeConfig) -> Vec<(String, Vec<f64>)> {
    let n_ser = cfg.series.len();
    if cfg.sort_order != "none" && !cfg.sort_order.is_empty() && n_ser > 1 {
        let means: Vec<f64> = cfg.series.iter().map(|(_, v)| if v.is_empty() { 0.0 } else { v.iter().sum::<f64>() / v.len() as f64 }).collect();
        let names: Vec<String> = cfg.series.iter().map(|(n, _)| n.clone()).collect();
        sorted(&sort_indices(n_ser, &means, &names, cfg.sort_order), cfg.series)
    } else {
        cfg.series.to_vec()
    }
}

pub fn x_range(series: &[(String, Vec<f64>)]) -> Option<(f64, f64)> {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for (_, vals) in series {
        for &v in vals {
            if v.is_finite() {
                if v < min { min = v; }
                if v > max { max = v; }
            }
        }
    }
    if !min.is_finite() { return None; }
    let r = (max - min).max(1e-12);
    let pad = r * 0.12;
    Some((min - pad, max + pad))
}

pub fn build_xs(x0: f64, x1: f64, n: usize) -> Vec<f64> {
    let n = n.max(2);
    let r = x1 - x0;
    (0..n).map(|i| x0 + r * i as f64 / (n - 1) as f64).collect()
}

pub fn sample_subset(vals: &[f64], cap: usize) -> (Vec<f64>, f64) {
    if vals.len() <= cap {
        (vals.to_vec(), 1.0)
    } else {
        let step = vals.len() / cap;
        let s: Vec<f64> = vals.iter().step_by(step).copied().collect();
        (s, step as f64)
    }
}

pub fn build_curve(vals: &[f64], xs: &[f64], bw_override: f64) -> Vec<f64> {
    let bw = if bw_override > 0.0 { bw_override } else { scott_bw(vals).max(1e-12) };
    let (sub, scale) = sample_subset(vals, 50);
    xs.iter().map(|&x| kde_eval(&sub, x, bw) * scale).collect()
}

pub fn integrate(curve: &[f64], xs: &[f64]) -> f64 {
    if curve.len() < 2 { return 0.0; }
    let mut a = 0.0f64;
    for i in 1..curve.len() {
        let dx = xs[i] - xs[i - 1];
        a += 0.5 * (curve[i] + curve[i - 1]) * dx;
    }
    a.max(1e-12)
}

pub fn cumulative(curve: &[f64], xs: &[f64]) -> Vec<f64> {
    let mut out = vec![0.0f64; curve.len()];
    for i in 1..curve.len() {
        let dx = xs[i] - xs[i - 1];
        out[i] = out[i - 1] + 0.5 * (curve[i] + curve[i - 1]) * dx;
    }
    out
}

pub fn histogram_normalized(vals: &[f64], x0: f64, x1: f64, n_bins: usize) -> (Vec<f64>, f64) {
    let n = n_bins.max(5);
    let mut counts = vec![0u64; n];
    let r = (x1 - x0).max(1e-12);
    let inv = n as f64 / r;
    for &v in vals {
        if !v.is_finite() { continue; }
        let i = ((v - x0) * inv) as i64;
        if i >= 0 && (i as usize) < n {
            counts[i as usize] += 1;
        }
    }
    let total = vals.len().max(1) as f64;
    let bw = r / n as f64;
    let densities: Vec<f64> = counts.iter().map(|&c| c as f64 / (total * bw)).collect();
    let max = densities.iter().copied().fold(0.0f64, f64::max);
    (densities, max)
}


