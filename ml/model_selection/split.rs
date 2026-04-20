use crate::ml::linalg::splitmix64;
use rayon::prelude::*;

pub fn train_test_split_cls(
    x: &[f64], n: usize, p: usize, y: &[i32], test_size: f64, seed: u64,
) -> (Vec<f64>, Vec<f64>, Vec<i32>, Vec<i32>) {
    let indices = shuffled_indices(n, seed);
    let split = ((1.0 - test_size) * n as f64).round() as usize;
    let n_train = split.max(1).min(n - 1);
    let n_test = n - n_train;

    let mut x_train = vec![0.0; n_train * p];
    let mut x_test = vec![0.0; n_test * p];
    let mut y_train = vec![0i32; n_train];
    let mut y_test = vec![0i32; n_test];

    scatter_rows(x, p, &indices[..n_train], &mut x_train, &mut y_train, y);
    scatter_rows(x, p, &indices[n_train..], &mut x_test, &mut y_test, y);

    (x_train, x_test, y_train, y_test)
}

pub fn train_test_split_reg(
    x: &[f64], n: usize, p: usize, y: &[f64], test_size: f64, seed: u64,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let indices = shuffled_indices(n, seed);
    let split = ((1.0 - test_size) * n as f64).round() as usize;
    let n_train = split.max(1).min(n - 1);
    let n_test = n - n_train;

    let mut x_train = vec![0.0; n_train * p];
    let mut x_test = vec![0.0; n_test * p];
    let mut y_train = vec![0.0; n_train];
    let mut y_test = vec![0.0; n_test];

    scatter_rows_f(x, p, &indices[..n_train], &mut x_train, &mut y_train, y);
    scatter_rows_f(x, p, &indices[n_train..], &mut x_test, &mut y_test, y);

    (x_train, x_test, y_train, y_test)
}

pub fn kfold_indices(n: usize, k: usize, seed: u64) -> Vec<(Vec<usize>, Vec<usize>)> {
    let indices = shuffled_indices(n, seed);
    let fold_size = n / k;
    (0..k).map(|i| {
        let start = i * fold_size;
        let end = if i == k - 1 { n } else { (i + 1) * fold_size };
        let test_idx: Vec<usize> = indices[start..end].to_vec();
        let train_idx: Vec<usize> = indices[..start].iter().chain(indices[end..].iter()).copied().collect();
        (train_idx, test_idx)
    }).collect()
}

#[inline]
fn scatter_rows(x: &[f64], p: usize, idx: &[usize], x_out: &mut [f64], y_out: &mut [i32], y: &[i32]) {
    if idx.len() * p > 200_000 {
        x_out.par_chunks_mut(p).enumerate().for_each(|(i, dst)| {
            let orig = idx[i];
            dst.copy_from_slice(&x[orig * p..(orig + 1) * p]);
        });
        y_out.par_iter_mut().enumerate().for_each(|(i, dst)| { *dst = y[idx[i]]; });
    } else {
        for (i, &orig) in idx.iter().enumerate() {
            x_out[i * p..(i + 1) * p].copy_from_slice(&x[orig * p..(orig + 1) * p]);
            y_out[i] = y[orig];
        }
    }
}

#[inline]
fn scatter_rows_f(x: &[f64], p: usize, idx: &[usize], x_out: &mut [f64], y_out: &mut [f64], y: &[f64]) {
    if idx.len() * p > 200_000 {
        x_out.par_chunks_mut(p).enumerate().for_each(|(i, dst)| {
            let orig = idx[i];
            dst.copy_from_slice(&x[orig * p..(orig + 1) * p]);
        });
        y_out.par_iter_mut().enumerate().for_each(|(i, dst)| { *dst = y[idx[i]]; });
    } else {
        for (i, &orig) in idx.iter().enumerate() {
            x_out[i * p..(i + 1) * p].copy_from_slice(&x[orig * p..(orig + 1) * p]);
            y_out[i] = y[orig];
        }
    }
}

fn shuffled_indices(n: usize, seed: u64) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..n).collect();
    let mut rng = seed;
    for i in (1..n).rev() {
        rng = splitmix64(rng);
        let j = rng as usize % (i + 1);
        indices.swap(i, j);
    }
    indices
}
