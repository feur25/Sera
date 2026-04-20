use rayon::prelude::*;
use crate::ml::linalg::splitmix64;
use crate::ml::model_selection::split::kfold_indices;
use crate::ml::{MlRegressor, MlClassifier};
pub struct FoldData {
    pub x_train: Vec<f64>,
    pub x_test: Vec<f64>,
    pub y_train_f: Vec<f64>,
    pub y_test_f: Vec<f64>,
    pub y_train_i: Vec<i32>,
    pub y_test_i: Vec<i32>,
    pub n_train: usize,
    pub n_test: usize,
    pub p: usize,
}

pub fn precompute_folds_reg(
    x: &[f64], n: usize, p: usize, y: &[f64], cv: usize, seed: u64,
) -> Vec<FoldData> {
    let folds = kfold_indices(n, cv, seed);
    folds.into_iter().map(|(train_idx, test_idx)| {
        let n_train = train_idx.len();
        let n_test = test_idx.len();
        let mut x_train = vec![0.0; n_train * p];
        let mut x_test = vec![0.0; n_test * p];
        let mut y_train = vec![0.0; n_train];
        let mut y_test = vec![0.0; n_test];
        for (i, &idx) in train_idx.iter().enumerate() {
            x_train[i * p..(i + 1) * p].copy_from_slice(&x[idx * p..(idx + 1) * p]);
            y_train[i] = y[idx];
        }
        for (i, &idx) in test_idx.iter().enumerate() {
            x_test[i * p..(i + 1) * p].copy_from_slice(&x[idx * p..(idx + 1) * p]);
            y_test[i] = y[idx];
        }
        FoldData {
            x_train, x_test,
            y_train_f: y_train, y_test_f: y_test,
            y_train_i: Vec::new(), y_test_i: Vec::new(),
            n_train, n_test, p,
        }
    }).collect()
}

pub fn precompute_folds_cls(
    x: &[f64], n: usize, p: usize, y: &[i32], cv: usize, seed: u64,
) -> Vec<FoldData> {
    let folds = stratified_kfold_indices(y, cv, seed);
    folds.into_iter().map(|(train_idx, test_idx)| {
        let n_train = train_idx.len();
        let n_test = test_idx.len();
        let mut x_train = vec![0.0; n_train * p];
        let mut x_test = vec![0.0; n_test * p];
        let mut y_train_i = vec![0i32; n_train];
        let mut y_test_i = vec![0i32; n_test];
        for (i, &idx) in train_idx.iter().enumerate() {
            x_train[i * p..(i + 1) * p].copy_from_slice(&x[idx * p..(idx + 1) * p]);
            y_train_i[i] = y[idx];
        }
        for (i, &idx) in test_idx.iter().enumerate() {
            x_test[i * p..(i + 1) * p].copy_from_slice(&x[idx * p..(idx + 1) * p]);
            y_test_i[i] = y[idx];
        }
        FoldData {
            x_train, x_test,
            y_train_f: Vec::new(), y_test_f: Vec::new(),
            y_train_i, y_test_i,
            n_train, n_test, p,
        }
    }).collect()
}

fn stratified_kfold_indices(y: &[i32], k: usize, seed: u64) -> Vec<(Vec<usize>, Vec<usize>)> {
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

#[derive(Clone, Debug)]
pub struct GridSearchResult {
    pub best_score: f64,
    pub best_params_idx: usize,
    pub cv_results: Vec<f64>,
    pub cv_std: Vec<f64>,
    pub all_scores: Vec<Vec<f64>>,
}

pub fn grid_search_parallel<F>(
    n_combos: usize,
    folds: &[FoldData],
    eval_fn: F,
) -> GridSearchResult
where
    F: Fn(usize, &FoldData) -> f64 + Send + Sync,
{
    let all_scores: Vec<Vec<f64>> = (0..n_combos)
        .into_par_iter()
        .map(|combo_idx| {
            folds.iter().map(|fold| eval_fn(combo_idx, fold)).collect::<Vec<f64>>()
        })
        .collect();

    finalise_results(all_scores, None)
}

pub fn randomized_search_parallel<F>(
    combo_indices: &[usize],
    folds: &[FoldData],
    eval_fn: F,
) -> GridSearchResult
where
    F: Fn(usize, &FoldData) -> f64 + Send + Sync,
{
    let all_scores: Vec<Vec<f64>> = combo_indices
        .par_iter()
        .map(|&combo_idx| {
            folds.iter().map(|fold| eval_fn(combo_idx, fold)).collect::<Vec<f64>>()
        })
        .collect();

    finalise_results(all_scores, Some(combo_indices))
}

fn finalise_results(all_scores: Vec<Vec<f64>>, combo_indices: Option<&[usize]>) -> GridSearchResult {
    let cv_results: Vec<f64> = all_scores.iter().map(|scores| {
        scores.iter().sum::<f64>() / scores.len() as f64
    }).collect();

    let cv_std: Vec<f64> = all_scores.iter().zip(cv_results.iter()).map(|(scores, &mean)| {
        let var = scores.iter().map(|&s| (s - mean) * (s - mean)).sum::<f64>() / scores.len() as f64;
        var.sqrt()
    }).collect();

    let (best_local_idx, &best_score) = cv_results.iter().enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or((0, &f64::NEG_INFINITY));

    let best_params_idx = match combo_indices {
        Some(ci) => ci[best_local_idx.min(ci.len().saturating_sub(1))],
        None => best_local_idx,
    };

    GridSearchResult { best_score, best_params_idx, cv_results, cv_std, all_scores }
}

pub fn sample_indices(total: usize, n_iter: usize, seed: u64) -> Vec<usize> {
    if n_iter >= total {
        return (0..total).collect();
    }
    let mut indices = Vec::with_capacity(n_iter);
    let mut seen = std::collections::HashSet::with_capacity(n_iter);
    let mut rng = seed;
    while indices.len() < n_iter {
        rng = splitmix64(rng);
        let idx = rng as usize % total;
        if seen.insert(idx) {
            indices.push(idx);
        }
    }
    indices
}

pub fn gs_ridge_reg(fold: &FoldData, alpha: f64, fit_intercept: bool) -> f64 {
    let mut m = crate::ml::linear::ridge::Ridge::new(alpha, fit_intercept);
    m.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_f);
    let preds = m.predict(&fold.x_test, fold.n_test, fold.p);
    crate::ml::metrics::regression::r2_score(&fold.y_test_f, &preds)
}

pub fn gs_lasso_reg(fold: &FoldData, alpha: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> f64 {
    let mut m = crate::ml::linear::lasso::Lasso::new(alpha, max_iter, tol, fit_intercept);
    m.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_f);
    let preds = m.predict(&fold.x_test, fold.n_test, fold.p);
    crate::ml::metrics::regression::r2_score(&fold.y_test_f, &preds)
}

pub fn gs_elasticnet_reg(fold: &FoldData, alpha: f64, l1_ratio: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> f64 {
    let mut m = crate::ml::linear::elastic_net::ElasticNet::new(alpha, l1_ratio, max_iter, tol, fit_intercept);
    m.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_f);
    let preds = m.predict(&fold.x_test, fold.n_test, fold.p);
    crate::ml::metrics::regression::r2_score(&fold.y_test_f, &preds)
}

pub fn gs_linear_reg(fold: &FoldData, fit_intercept: bool) -> f64 {
    let mut m = crate::ml::linear::ols::LinearRegression::new(fit_intercept);
    m.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_f);
    let preds = m.predict(&fold.x_test, fold.n_test, fold.p);
    crate::ml::metrics::regression::r2_score(&fold.y_test_f, &preds)
}

pub fn gs_logistic_cls(fold: &FoldData, c: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> f64 {
    let mut m = crate::ml::linear::logistic::LogisticRegression::new(c, max_iter, tol, fit_intercept);
    m.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_i);
    let preds = m.predict(&fold.x_test, fold.n_test, fold.p);
    crate::ml::metrics::classification::accuracy_score(&fold.y_test_i, &preds)
}

pub fn gs_ridge_cls(fold: &FoldData, alpha: f64) -> f64 {
    let mut m = crate::ml::linear::ridge::RidgeClassifier::new(alpha);
    m.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_i);
    let preds = m.predict(&fold.x_test, fold.n_test, fold.p);
    crate::ml::metrics::classification::accuracy_score(&fold.y_test_i, &preds)
}

pub fn gs_sgd_cls(fold: &FoldData, loss: crate::ml::linear::sgd::SGDLoss, alpha: f64, max_iter: usize, tol: f64, lr: f64, fit_intercept: bool) -> f64 {
    let mut m = crate::ml::linear::sgd::SGDClassifier::new(loss, alpha, max_iter, tol, lr, fit_intercept);
    m.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_i);
    let preds = m.predict(&fold.x_test, fold.n_test, fold.p);
    crate::ml::metrics::classification::accuracy_score(&fold.y_test_i, &preds)
}

pub fn gs_sgd_reg(fold: &FoldData, alpha: f64, max_iter: usize, tol: f64, lr: f64, fit_intercept: bool) -> f64 {
    let mut m = crate::ml::linear::sgd::SGDRegressor::new(alpha, max_iter, tol, lr, fit_intercept);
    m.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_f);
    let preds = m.predict(&fold.x_test, fold.n_test, fold.p);
    crate::ml::metrics::regression::r2_score(&fold.y_test_f, &preds)
}

pub fn gs_dt_cls(fold: &FoldData, max_depth: usize, min_samples_split: usize, min_samples_leaf: usize, max_features: Option<usize>, criterion: crate::ml::tree::decision_tree::TreeCriterion) -> f64 {
    let mut m = crate::ml::tree::decision_tree::DecisionTreeClassifier::new(max_depth, min_samples_split, min_samples_leaf, max_features, criterion);
    m.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_i);
    let preds = m.predict(&fold.x_test, fold.n_test, fold.p);
    crate::ml::metrics::classification::accuracy_score(&fold.y_test_i, &preds)
}

pub fn gs_dt_reg(fold: &FoldData, max_depth: usize, min_samples_split: usize, min_samples_leaf: usize, max_features: Option<usize>) -> f64 {
    let mut m = crate::ml::tree::decision_tree::DecisionTreeRegressor::new(max_depth, min_samples_split, min_samples_leaf, max_features);
    m.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_f);
    let preds = m.predict(&fold.x_test, fold.n_test, fold.p);
    crate::ml::metrics::regression::r2_score(&fold.y_test_f, &preds)
}

pub fn gs_rf_cls(fold: &FoldData, n_estimators: usize, max_depth: usize, min_samples_split: usize, min_samples_leaf: usize, max_features: crate::ml::tree::random_forest::MaxFeatures) -> f64 {
    let mut m = crate::ml::tree::random_forest::RandomForestClassifier::new(n_estimators, max_depth, min_samples_split, min_samples_leaf, max_features);
    m.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_i);
    let preds = m.predict(&fold.x_test, fold.n_test, fold.p);
    crate::ml::metrics::classification::accuracy_score(&fold.y_test_i, &preds)
}

pub fn gs_rf_reg(fold: &FoldData, n_estimators: usize, max_depth: usize, min_samples_split: usize, min_samples_leaf: usize, max_features: crate::ml::tree::random_forest::MaxFeatures) -> f64 {
    let mut m = crate::ml::tree::random_forest::RandomForestRegressor::new(n_estimators, max_depth, min_samples_split, min_samples_leaf, max_features);
    m.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_f);
    let preds = m.predict(&fold.x_test, fold.n_test, fold.p);
    crate::ml::metrics::regression::r2_score(&fold.y_test_f, &preds)
}

pub fn gs_knn_cls(fold: &FoldData, k: usize, weights: crate::ml::neighbors::knn::KnnWeights) -> f64 {
    let mut m = crate::ml::neighbors::knn::KNeighborsClassifier::new(k, weights);
    m.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_i);
    let preds = m.predict(&fold.x_test, fold.n_test, fold.p);
    crate::ml::metrics::classification::accuracy_score(&fold.y_test_i, &preds)
}

pub fn gs_knn_reg(fold: &FoldData, k: usize, weights: crate::ml::neighbors::knn::KnnWeights) -> f64 {
    let mut m = crate::ml::neighbors::knn::KNeighborsRegressor::new(k, weights);
    m.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_f);
    let preds = m.predict(&fold.x_test, fold.n_test, fold.p);
    crate::ml::metrics::regression::r2_score(&fold.y_test_f, &preds)
}

pub fn gs_gb_cls(fold: &FoldData, n_estimators: usize, learning_rate: f64, max_depth: usize, min_samples_split: usize, min_samples_leaf: usize) -> f64 {
    let mut m = crate::ml::tree::gradient_boosting::GradientBoostingClassifier::new(n_estimators, learning_rate, max_depth, min_samples_split, min_samples_leaf);
    m.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_i);
    let preds = m.predict(&fold.x_test, fold.n_test, fold.p);
    crate::ml::metrics::classification::accuracy_score(&fold.y_test_i, &preds)
}

pub fn gs_gb_reg(fold: &FoldData, n_estimators: usize, learning_rate: f64, max_depth: usize, min_samples_split: usize, min_samples_leaf: usize) -> f64 {
    let mut m = crate::ml::tree::gradient_boosting::GradientBoostingRegressor::new(n_estimators, learning_rate, max_depth, min_samples_split, min_samples_leaf);
    m.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_f);
    let preds = m.predict(&fold.x_test, fold.n_test, fold.p);
    crate::ml::metrics::regression::r2_score(&fold.y_test_f, &preds)
}

pub fn gs_svc(fold: &FoldData, c: f64, max_iter: usize, tol: f64) -> f64 {
    let mut m = crate::ml::svm::svm::LinearSVC::new(c, max_iter, tol);
    m.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_i);
    let preds = m.predict(&fold.x_test, fold.n_test, fold.p);
    crate::ml::metrics::classification::accuracy_score(&fold.y_test_i, &preds)
}

pub fn gs_svr(fold: &FoldData, c: f64, epsilon: f64, max_iter: usize, tol: f64) -> f64 {
    let mut m = crate::ml::svm::svm::LinearSVR::new(c, epsilon, max_iter, tol);
    m.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_f);
    let preds = m.predict(&fold.x_test, fold.n_test, fold.p);
    crate::ml::metrics::regression::r2_score(&fold.y_test_f, &preds)
}

pub fn gs_gnb(fold: &FoldData) -> f64 {
    let mut m = crate::ml::naive_bayes::gaussian::GaussianNB::new();
    m.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_i);
    let preds = m.predict(&fold.x_test, fold.n_test, fold.p);
    crate::ml::metrics::classification::accuracy_score(&fold.y_test_i, &preds)
}

pub fn gs_adaboost_cls(fold: &FoldData, n_estimators: usize, learning_rate: f64, max_depth: usize) -> f64 {
    let mut m = crate::ml::tree::adaboost::AdaBoostClassifier::new(n_estimators, learning_rate, max_depth);
    m.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_i);
    let preds = m.predict(&fold.x_test, fold.n_test, fold.p);
    crate::ml::metrics::classification::accuracy_score(&fold.y_test_i, &preds)
}

pub fn gs_adaboost_reg(fold: &FoldData, n_estimators: usize, learning_rate: f64, max_depth: usize) -> f64 {
    let mut m = crate::ml::tree::adaboost::AdaBoostRegressor::new(n_estimators, learning_rate, max_depth);
    m.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_f);
    let preds = m.predict(&fold.x_test, fold.n_test, fold.p);
    crate::ml::metrics::regression::r2_score(&fold.y_test_f, &preds)
}

pub fn decode_combo(combo_idx: usize, sizes: &[usize]) -> Vec<usize> {
    let mut indices = Vec::with_capacity(sizes.len());
    let mut remaining = combo_idx;
    for &size in sizes {
        indices.push(remaining % size);
        remaining /= size;
    }
    indices
}

pub fn n_combinations(sizes: &[usize]) -> usize {
    sizes.iter().product()
}

pub fn is_classifier(estimator: &str) -> bool {
    matches!(estimator,
        "LogisticRegression" | "RidgeClassifier" | "SGDClassifier" |
        "DecisionTreeClassifier" | "RandomForestClassifier" |
        "GradientBoostingClassifier" | "AdaBoostClassifier" |
        "KNeighborsClassifier" | "GaussianNB" | "LinearSVC"
    )
}

fn gf(names: &[String], vals: &[String], key: &str, default: f64) -> f64 {
    names.iter().position(|n| n == key).and_then(|i| vals[i].parse().ok()).unwrap_or(default)
}
fn gu(names: &[String], vals: &[String], key: &str, default: usize) -> usize {
    names.iter().position(|n| n == key).and_then(|i| vals[i].parse::<f64>().ok().map(|v| v as usize)).unwrap_or(default)
}
fn gb(names: &[String], vals: &[String], key: &str, default: bool) -> bool {
    names.iter().position(|n| n == key).map(|i| vals[i] == "true" || vals[i] == "True" || vals[i] == "1").unwrap_or(default)
}
fn gs<'a>(names: &[String], vals: &'a [String], key: &str, default: &'a str) -> &'a str {
    names.iter().position(|n| n == key).map(|i| vals[i].as_str()).unwrap_or(default)
}

pub fn eval_model(
    estimator: &str,
    param_names: &[String],
    param_values: &[Vec<String>],
    param_sizes: &[usize],
    combo_idx: usize,
    fold: &FoldData,
) -> f64 {
    let indices = decode_combo(combo_idx, param_sizes);
    let vals: Vec<String> = indices.iter().enumerate()
        .map(|(i, &idx)| param_values[i][idx].clone())
        .collect();
    let n = param_names;
    let v = &vals;

    match estimator {
        "Ridge" => gs_ridge_reg(fold, gf(n, v, "alpha", 1.0), gb(n, v, "fit_intercept", true)),
        "Lasso" => gs_lasso_reg(fold, gf(n, v, "alpha", 1.0), gu(n, v, "max_iter", 1000), gf(n, v, "tol", 1e-4), gb(n, v, "fit_intercept", true)),
        "ElasticNet" => gs_elasticnet_reg(fold, gf(n, v, "alpha", 1.0), gf(n, v, "l1_ratio", 0.5), gu(n, v, "max_iter", 1000), gf(n, v, "tol", 1e-4), gb(n, v, "fit_intercept", true)),
        "LinearRegression" => gs_linear_reg(fold, gb(n, v, "fit_intercept", true)),
        "LogisticRegression" => gs_logistic_cls(fold, gf(n, v, "C", 1.0), gu(n, v, "max_iter", 100), gf(n, v, "tol", 1e-4), gb(n, v, "fit_intercept", true)),
        "RidgeClassifier" => gs_ridge_cls(fold, gf(n, v, "alpha", 1.0)),
        "SGDClassifier" => {
            let loss = match gs(n, v, "loss", "hinge") {
                "log" | "log_loss" => crate::ml::linear::sgd::SGDLoss::Log,
                "modified_huber" => crate::ml::linear::sgd::SGDLoss::ModifiedHuber,
                "squared_hinge" => crate::ml::linear::sgd::SGDLoss::SquaredHinge,
                _ => crate::ml::linear::sgd::SGDLoss::Hinge,
            };
            gs_sgd_cls(fold, loss, gf(n, v, "alpha", 0.0001), gu(n, v, "max_iter", 1000), gf(n, v, "tol", 1e-3), gf(n, v, "eta0", 0.01), gb(n, v, "fit_intercept", true))
        }
        "SGDRegressor" => gs_sgd_reg(fold, gf(n, v, "alpha", 0.0001), gu(n, v, "max_iter", 1000), gf(n, v, "tol", 1e-3), gf(n, v, "eta0", 0.01), gb(n, v, "fit_intercept", true)),
        "DecisionTreeClassifier" => {
            let criterion = if gs(n, v, "criterion", "gini") == "entropy" { crate::ml::tree::decision_tree::TreeCriterion::Entropy } else { crate::ml::tree::decision_tree::TreeCriterion::Gini };
            let mf = n.iter().position(|k| k == "max_features").and_then(|i| vals[i].parse::<usize>().ok());
            gs_dt_cls(fold, gu(n, v, "max_depth", 10), gu(n, v, "min_samples_split", 2), gu(n, v, "min_samples_leaf", 1), mf, criterion)
        }
        "DecisionTreeRegressor" => {
            let mf = n.iter().position(|k| k == "max_features").and_then(|i| vals[i].parse::<usize>().ok());
            gs_dt_reg(fold, gu(n, v, "max_depth", 10), gu(n, v, "min_samples_split", 2), gu(n, v, "min_samples_leaf", 1), mf)
        }
        "RandomForestClassifier" => {
            let mf = match gs(n, v, "max_features", "sqrt") {
                "log2" => crate::ml::tree::random_forest::MaxFeatures::Log2,
                "all" | "none" => crate::ml::tree::random_forest::MaxFeatures::All,
                s => s.parse::<usize>().map(crate::ml::tree::random_forest::MaxFeatures::Fixed).unwrap_or(crate::ml::tree::random_forest::MaxFeatures::Sqrt),
            };
            gs_rf_cls(fold, gu(n, v, "n_estimators", 100), gu(n, v, "max_depth", 10), gu(n, v, "min_samples_split", 2), gu(n, v, "min_samples_leaf", 1), mf)
        }
        "RandomForestRegressor" => {
            let mf = match gs(n, v, "max_features", "sqrt") {
                "log2" => crate::ml::tree::random_forest::MaxFeatures::Log2,
                "all" | "none" => crate::ml::tree::random_forest::MaxFeatures::All,
                s => s.parse::<usize>().map(crate::ml::tree::random_forest::MaxFeatures::Fixed).unwrap_or(crate::ml::tree::random_forest::MaxFeatures::Sqrt),
            };
            gs_rf_reg(fold, gu(n, v, "n_estimators", 100), gu(n, v, "max_depth", 10), gu(n, v, "min_samples_split", 2), gu(n, v, "min_samples_leaf", 1), mf)
        }
        "GradientBoostingClassifier" => gs_gb_cls(fold, gu(n, v, "n_estimators", 100), gf(n, v, "learning_rate", 0.1), gu(n, v, "max_depth", 3), gu(n, v, "min_samples_split", 2), gu(n, v, "min_samples_leaf", 1)),
        "GradientBoostingRegressor" => gs_gb_reg(fold, gu(n, v, "n_estimators", 100), gf(n, v, "learning_rate", 0.1), gu(n, v, "max_depth", 3), gu(n, v, "min_samples_split", 2), gu(n, v, "min_samples_leaf", 1)),
        "AdaBoostClassifier" => gs_adaboost_cls(fold, gu(n, v, "n_estimators", 50), gf(n, v, "learning_rate", 1.0), gu(n, v, "max_depth", 1)),
        "AdaBoostRegressor" => gs_adaboost_reg(fold, gu(n, v, "n_estimators", 50), gf(n, v, "learning_rate", 1.0), gu(n, v, "max_depth", 1)),
        "KNeighborsClassifier" => {
            let w = if gs(n, v, "weights", "uniform") == "distance" { crate::ml::neighbors::knn::KnnWeights::Distance } else { crate::ml::neighbors::knn::KnnWeights::Uniform };
            gs_knn_cls(fold, gu(n, v, "n_neighbors", 5), w)
        }
        "KNeighborsRegressor" => {
            let w = if gs(n, v, "weights", "uniform") == "distance" { crate::ml::neighbors::knn::KnnWeights::Distance } else { crate::ml::neighbors::knn::KnnWeights::Uniform };
            gs_knn_reg(fold, gu(n, v, "n_neighbors", 5), w)
        }
        "GaussianNB" => gs_gnb(fold),
        "LinearSVC" => gs_svc(fold, gf(n, v, "C", 1.0), gu(n, v, "max_iter", 1000), gf(n, v, "tol", 1e-4)),
        "LinearSVR" => gs_svr(fold, gf(n, v, "C", 1.0), gf(n, v, "epsilon", 0.1), gu(n, v, "max_iter", 1000), gf(n, v, "tol", 1e-4)),
        _ => f64::NEG_INFINITY,
    }
}

pub fn subsample_fold(fold: &FoldData, n_sub: usize) -> FoldData {
    let n = n_sub.min(fold.n_train);
    FoldData {
        x_train: fold.x_train[..n * fold.p].to_vec(),
        x_test: fold.x_test.clone(),
        y_train_f: if fold.y_train_f.is_empty() { Vec::new() } else { fold.y_train_f[..n].to_vec() },
        y_test_f: fold.y_test_f.clone(),
        y_train_i: if fold.y_train_i.is_empty() { Vec::new() } else { fold.y_train_i[..n].to_vec() },
        y_test_i: fold.y_test_i.clone(),
        n_train: n,
        n_test: fold.n_test,
        p: fold.p,
    }
}
