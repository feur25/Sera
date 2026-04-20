use rayon::prelude::*;
use crate::ml::model_selection::split::kfold_indices;
use crate::ml::metrics::classification::accuracy_score;
use crate::ml::metrics::regression::r2_score;

pub fn cross_val_score_cls<F>(
    x: &[f64], n: usize, p: usize, y: &[i32],
    k: usize, seed: u64,
    train_predict: F,
) -> Vec<f64>
where F: Fn(&[f64], usize, usize, &[i32], &[f64], usize) -> Vec<i32> + Send + Sync
{
    let folds = kfold_indices(n, k, seed);
    folds.into_par_iter().map(|(train_idx, test_idx)| {
        let n_train = train_idx.len();
        let n_test = test_idx.len();
        let mut x_train = vec![0.0; n_train * p];
        let mut y_train = vec![0i32; n_train];
        let mut x_test = vec![0.0; n_test * p];
        let mut y_test = vec![0i32; n_test];

        for (i, &idx) in train_idx.iter().enumerate() {
            x_train[i * p..(i + 1) * p].copy_from_slice(&x[idx * p..(idx + 1) * p]);
            y_train[i] = y[idx];
        }
        for (i, &idx) in test_idx.iter().enumerate() {
            x_test[i * p..(i + 1) * p].copy_from_slice(&x[idx * p..(idx + 1) * p]);
            y_test[i] = y[idx];
        }

        let preds = train_predict(&x_train, n_train, p, &y_train, &x_test, n_test);
        accuracy_score(&y_test, &preds)
    }).collect()
}

pub fn cross_val_score_reg<F>(
    x: &[f64], n: usize, p: usize, y: &[f64],
    k: usize, seed: u64,
    train_predict: F,
) -> Vec<f64>
where F: Fn(&[f64], usize, usize, &[f64], &[f64], usize) -> Vec<f64> + Send + Sync
{
    let folds = kfold_indices(n, k, seed);
    folds.into_par_iter().map(|(train_idx, test_idx)| {
        let n_train = train_idx.len();
        let n_test = test_idx.len();
        let mut x_train = vec![0.0; n_train * p];
        let mut y_train = vec![0.0; n_train];
        let mut x_test = vec![0.0; n_test * p];
        let mut y_test = vec![0.0; n_test];

        for (i, &idx) in train_idx.iter().enumerate() {
            x_train[i * p..(i + 1) * p].copy_from_slice(&x[idx * p..(idx + 1) * p]);
            y_train[i] = y[idx];
        }
        for (i, &idx) in test_idx.iter().enumerate() {
            x_test[i * p..(i + 1) * p].copy_from_slice(&x[idx * p..(idx + 1) * p]);
            y_test[i] = y[idx];
        }

        let preds = train_predict(&x_train, n_train, p, &y_train, &x_test, n_test);
        r2_score(&y_test, &preds)
    }).collect()
}
