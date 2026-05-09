use rayon::prelude::*;
use crate::ml::linalg::splitmix64;
use crate::ml::metrics::regression::r2_score;
use crate::ml::metrics::classification::accuracy_score;

pub fn permutation_importance_reg<F>(
    x: &[f64], n: usize, p: usize, y: &[f64],
    n_repeats: usize, seed: u64,
    predict: F,
) -> (Vec<f64>, Vec<f64>)
where F: Fn(&[f64], usize, usize) -> Vec<f64> + Send + Sync
{
    let baseline = r2_score(y, &predict(x, n, p));
    let importances: Vec<(f64, f64)> = (0..p).into_par_iter().map(|j| {
        let mut diffs = Vec::with_capacity(n_repeats);
        for r in 0..n_repeats {
            let mut x_perm = x.to_vec();
            let mut rng = seed.wrapping_add((j as u64).wrapping_mul(0x9E3779B97F4A7C15)).wrapping_add(r as u64);
            for i in (1..n).rev() {
                rng = splitmix64(rng);
                let k = rng as usize % (i + 1);
                let a_idx = i * p + j;
                let b_idx = k * p + j;
                x_perm.swap(a_idx, b_idx);
            }
            let s = r2_score(y, &predict(&x_perm, n, p));
            diffs.push(baseline - s);
        }
        let mean = diffs.iter().sum::<f64>() / diffs.len() as f64;
        let var = diffs.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / diffs.len() as f64;
        (mean, var.sqrt())
    }).collect();
    let means: Vec<f64> = importances.iter().map(|(m, _)| *m).collect();
    let stds: Vec<f64> = importances.iter().map(|(_, s)| *s).collect();
    (means, stds)
}

pub fn permutation_importance_cls<F>(
    x: &[f64], n: usize, p: usize, y: &[i32],
    n_repeats: usize, seed: u64,
    predict: F,
) -> (Vec<f64>, Vec<f64>)
where F: Fn(&[f64], usize, usize) -> Vec<i32> + Send + Sync
{
    let baseline = accuracy_score(y, &predict(x, n, p));
    let importances: Vec<(f64, f64)> = (0..p).into_par_iter().map(|j| {
        let mut diffs = Vec::with_capacity(n_repeats);
        for r in 0..n_repeats {
            let mut x_perm = x.to_vec();
            let mut rng = seed.wrapping_add((j as u64).wrapping_mul(0x9E3779B97F4A7C15)).wrapping_add(r as u64);
            for i in (1..n).rev() {
                rng = splitmix64(rng);
                let k = rng as usize % (i + 1);
                let a_idx = i * p + j;
                let b_idx = k * p + j;
                x_perm.swap(a_idx, b_idx);
            }
            let s = accuracy_score(y, &predict(&x_perm, n, p));
            diffs.push(baseline - s);
        }
        let mean = diffs.iter().sum::<f64>() / diffs.len() as f64;
        let var = diffs.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / diffs.len() as f64;
        (mean, var.sqrt())
    }).collect();
    let means: Vec<f64> = importances.iter().map(|(m, _)| *m).collect();
    let stds: Vec<f64> = importances.iter().map(|(_, s)| *s).collect();
    (means, stds)
}


