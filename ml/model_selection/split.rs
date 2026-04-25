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

pub fn stratified_kfold_indices(y: &[i32], k: usize, seed: u64) -> Vec<(Vec<usize>, Vec<usize>)> {
    let mut classes: Vec<i32> = y.to_vec();
    classes.sort_unstable();
    classes.dedup();
    let mut class_indices: Vec<Vec<usize>> = classes.iter().map(|&c| {
        y.iter().enumerate().filter(|(_, &v)| v == c).map(|(i, _)| i).collect()
    }).collect();
    let mut rng = seed;
    for indices in &mut class_indices {
        for i in (1..indices.len()).rev() {
            rng = splitmix64(rng);
            let j = rng as usize % (i + 1);
            indices.swap(i, j);
        }
    }
    let mut folds: Vec<Vec<usize>> = (0..k).map(|_| Vec::new()).collect();
    for indices in &class_indices {
        for (i, &idx) in indices.iter().enumerate() {
            folds[i % k].push(idx);
        }
    }
    (0..k).map(|fi| {
        let test: Vec<usize> = folds[fi].clone();
        let train: Vec<usize> = (0..k)
            .filter(|&j| j != fi)
            .flat_map(|j| folds[j].iter().copied())
            .collect();
        (train, test)
    }).collect()
}

pub fn group_kfold_indices(groups: &[i32], k: usize) -> Vec<(Vec<usize>, Vec<usize>)> {
    let mut unique_groups: Vec<i32> = groups.to_vec();
    unique_groups.sort_unstable();
    unique_groups.dedup();
    let mut group_sizes: Vec<(i32, usize)> = unique_groups.iter().map(|&g| {
        (g, groups.iter().filter(|&&v| v == g).count())
    }).collect();
    group_sizes.sort_by(|a, b| b.1.cmp(&a.1));
    let kk = k.max(1).min(unique_groups.len().max(1));
    let mut fold_sizes = vec![0usize; kk];
    let mut group_to_fold: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
    for (g, sz) in group_sizes {
        let mut min_fi = 0usize;
        for fi in 1..kk {
            if fold_sizes[fi] < fold_sizes[min_fi] { min_fi = fi; }
        }
        fold_sizes[min_fi] += sz;
        group_to_fold.insert(g, min_fi);
    }
    (0..kk).map(|fi| {
        let mut test = Vec::new();
        let mut train = Vec::new();
        for (i, &g) in groups.iter().enumerate() {
            if group_to_fold.get(&g).copied().unwrap_or(0) == fi { test.push(i); } else { train.push(i); }
        }
        (train, test)
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
