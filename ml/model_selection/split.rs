use crate::ml::linalg::splitmix64;

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

    for (out_i, &orig_i) in indices[..n_train].iter().enumerate() {
        x_train[out_i * p..(out_i + 1) * p].copy_from_slice(&x[orig_i * p..(orig_i + 1) * p]);
        y_train[out_i] = y[orig_i];
    }
    for (out_i, &orig_i) in indices[n_train..].iter().enumerate() {
        x_test[out_i * p..(out_i + 1) * p].copy_from_slice(&x[orig_i * p..(orig_i + 1) * p]);
        y_test[out_i] = y[orig_i];
    }
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

    for (out_i, &orig_i) in indices[..n_train].iter().enumerate() {
        x_train[out_i * p..(out_i + 1) * p].copy_from_slice(&x[orig_i * p..(orig_i + 1) * p]);
        y_train[out_i] = y[orig_i];
    }
    for (out_i, &orig_i) in indices[n_train..].iter().enumerate() {
        x_test[out_i * p..(out_i + 1) * p].copy_from_slice(&x[orig_i * p..(orig_i + 1) * p]);
        y_test[out_i] = y[orig_i];
    }
    (x_train, x_test, y_train, y_test)
}

pub fn kfold_indices(n: usize, k: usize, seed: u64) -> Vec<(Vec<usize>, Vec<usize>)> {
    let indices = shuffled_indices(n, seed);
    let fold_size = n / k;
    let mut folds = Vec::with_capacity(k);
    for i in 0..k {
        let start = i * fold_size;
        let end = if i == k - 1 { n } else { (i + 1) * fold_size };
        let test_idx: Vec<usize> = indices[start..end].to_vec();
        let train_idx: Vec<usize> = indices[..start].iter().chain(indices[end..].iter()).copied().collect();
        folds.push((train_idx, test_idx));
    }
    folds
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
