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
pub struct GramCache {
    xtx_c: Vec<f64>,
    xty_c: Vec<f64>,
    gram_s: Vec<f64>,
    xty_s: Vec<f64>,
    xj_sq: Vec<f64>,
    x_mean: Vec<f64>,
    x_std: Vec<f64>,
    y_mean: f64,
    n: usize,
    p: usize,
}

pub struct KnnDistCache {
    nn: Vec<Vec<(f64, usize)>>,
    max_k: usize,
}

pub struct IrlsCache {
    xt: Vec<f64>,
    y_bin: Vec<f64>,
    classes: [i32; 2],
    is_binary: bool,
}

pub enum ModelCache {
    Gram(GramCache),
    Knn(KnnDistCache),
    Irls(IrlsCache),
    None,
}

pub fn compute_gram_cache(fold: &FoldData) -> GramCache {
    let n = fold.n_train;
    let p = fold.p;
    let x = &fold.x_train;
    let y = &fold.y_train_f;
    let nf = n as f64;
    let inv = 1.0 / nf;

    let mut x_mean = vec![0.0; p];
    let mut y_mean = 0.0;
    for i in 0..n {
        for j in 0..p { x_mean[j] += x[i * p + j]; }
        y_mean += y[i];
    }
    for j in 0..p { x_mean[j] *= inv; }
    y_mean *= inv;

    let mut x_var = vec![0.0; p];
    for i in 0..n {
        for j in 0..p { let d = x[i * p + j] - x_mean[j]; x_var[j] += d * d; }
    }
    let mut x_std = vec![0.0; p];
    for j in 0..p {
        x_std[j] = (x_var[j] * inv).sqrt();
        if x_std[j] < 1e-15 { x_std[j] = 1.0; }
    }

    let mut xtx = vec![0.0; p * p];
    let mut xty = vec![0.0; p];
    for i in 0..n {
        let row = &x[i * p..(i + 1) * p];
        let yi = y[i];
        for ii in 0..p {
            let ai = row[ii];
            for j in ii..p { xtx[ii * p + j] += ai * row[j]; }
            xty[ii] += ai * yi;
        }
    }
    for i in 0..p { for j in (i + 1)..p { xtx[j * p + i] = xtx[i * p + j]; } }

    let mut xtx_c = xtx;
    for i in 0..p {
        for j in 0..p { xtx_c[i * p + j] -= nf * x_mean[i] * x_mean[j]; }
    }
    let mut xty_c = xty;
    for j in 0..p { xty_c[j] -= nf * x_mean[j] * y_mean; }

    let mut gram_s = xtx_c.clone();
    for i in 0..p {
        for j in 0..p { gram_s[i * p + j] /= x_std[i] * x_std[j]; }
    }
    let mut xty_s = xty_c.clone();
    for j in 0..p { xty_s[j] /= x_std[j]; }
    let xj_sq: Vec<f64> = (0..p).map(|j| gram_s[j * p + j]).collect();

    GramCache { xtx_c, xty_c, gram_s, xty_s, xj_sq, x_mean, x_std, y_mean, n, p }
}

pub fn compute_knn_dist_cache(fold: &FoldData, max_k: usize) -> KnnDistCache {
    let is_cls = !fold.y_train_i.is_empty();
    if is_cls {
        let mut model = crate::ml::neighbors::knn::KNeighborsClassifier::new(max_k, crate::ml::neighbors::knn::KnnWeights::Uniform);
        model.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_i);
        let nn = model.kneighbors(&fold.x_test, fold.n_test, fold.p);
        KnnDistCache { nn, max_k }
    } else {
        let mut model = crate::ml::neighbors::knn::KNeighborsRegressor::new(max_k, crate::ml::neighbors::knn::KnnWeights::Uniform);
        model.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_f);
        let nn = model.kneighbors(&fold.x_test, fold.n_test, fold.p);
        KnnDistCache { nn, max_k }
    }
}

pub fn needs_gram_cache(estimator: &str) -> bool {
    matches!(estimator, "Ridge" | "Lasso" | "ElasticNet" | "LinearRegression")
}

pub fn needs_knn_cache(estimator: &str) -> bool {
    matches!(estimator, "KNeighborsClassifier" | "KNeighborsRegressor")
}

pub fn needs_irls_cache(estimator: &str) -> bool {
    matches!(estimator, "LogisticRegression")
}

pub fn compute_irls_cache(fold: &FoldData) -> IrlsCache {
    let n = fold.n_train;
    let p = fold.p;
    let y = &fold.y_train_i;
    let mut classes_vec = y.to_vec();
    classes_vec.sort_unstable();
    classes_vec.dedup();
    let is_binary = classes_vec.len() == 2;
    let classes = if is_binary { [classes_vec[0], classes_vec[1]] } else { [0, 0] };
    let y_bin = if is_binary {
        y.iter().map(|&v| if v == classes[1] { 1.0 } else { 0.0 }).collect()
    } else {
        Vec::new()
    };
    let mut xt = vec![0.0; p * n];
    for i in 0..n {
        for j in 0..p {
            xt[j * n + i] = fold.x_train[i * p + j];
        }
    }
    IrlsCache { xt, y_bin, classes, is_binary }
}

pub fn compute_caches(
    estimator: &str, folds: &[FoldData],
    param_names: &[String], param_values: &[Vec<String>],
) -> Vec<ModelCache> {
    if needs_gram_cache(estimator) {
        folds.iter().map(|f| ModelCache::Gram(compute_gram_cache(f))).collect()
    } else if needs_knn_cache(estimator) {
        let max_k = param_names.iter().position(|n| n == "n_neighbors")
            .and_then(|i| param_values[i].iter().filter_map(|v| v.parse::<usize>().ok()).max())
            .unwrap_or(10);
        folds.iter().map(|f| ModelCache::Knn(compute_knn_dist_cache(f, max_k))).collect()
    } else if needs_irls_cache(estimator) {
        folds.iter().map(|f| ModelCache::Irls(compute_irls_cache(f))).collect()
    } else {
        folds.iter().map(|_| ModelCache::None).collect()
    }
}

fn predict_and_r2(fold: &FoldData, coef: &[f64], intercept: f64) -> f64 {
    let n = fold.n_test;
    let p = fold.p;
    let y = &fold.y_test_f;
    let y_mean = y.iter().sum::<f64>() / n as f64;
    let (mut ss_res, mut ss_tot) = (0.0, 0.0);
    for i in 0..n {
        let mut pred = intercept;
        for j in 0..p { pred += fold.x_test[i * p + j] * coef[j]; }
        let e = y[i] - pred; ss_res += e * e;
        let d = y[i] - y_mean; ss_tot += d * d;
    }
    if ss_tot < 1e-15 { 0.0 } else { 1.0 - ss_res / ss_tot }
}

#[inline]
fn soft_thresh(x: f64, lam: f64) -> f64 {
    if x > lam { x - lam } else if x < -lam { x + lam } else { 0.0 }
}

fn gs_ridge_gram(fold: &FoldData, g: &GramCache, alpha: f64, fit_intercept: bool) -> f64 {
    let p = g.p;
    if fit_intercept {
        let mut a = g.xtx_c.clone();
        for j in 0..p { a[j * p + j] += alpha; }
        let w = crate::ml::linalg::solve_spd(&a, p, &g.xty_c).unwrap_or_else(|| vec![0.0; p]);
        let b = g.y_mean - crate::ml::linalg::dot(&w, &g.x_mean);
        predict_and_r2(fold, &w, b)
    } else {
        let nf = g.n as f64;
        let mut a = g.xtx_c.clone();
        for i in 0..p { for j in 0..p { a[i * p + j] += nf * g.x_mean[i] * g.x_mean[j]; } a[i * p + i] += alpha; }
        let mut b = g.xty_c.clone();
        for j in 0..p { b[j] += nf * g.x_mean[j] * g.y_mean; }
        let w = crate::ml::linalg::solve_spd(&a, p, &b).unwrap_or_else(|| vec![0.0; p]);
        predict_and_r2(fold, &w, 0.0)
    }
}

fn gs_elasticnet_gram_eval(fold: &FoldData, gc: &GramCache, alpha: f64, l1_ratio: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> f64 {
    let p = gc.p;
    let l1 = alpha * l1_ratio;
    let l2 = alpha * (1.0 - l1_ratio);
    let inv_n = 1.0 / gc.n as f64;

    if fit_intercept {
        let g = &gc.gram_s;
        let b = &gc.xty_s;
        let xj_sq = &gc.xj_sq;
        let mut w = vec![0.0; p];
        for _ in 0..max_iter {
            let mut mc = 0.0f64;
            for j in 0..p {
                if xj_sq[j] < 1e-15 { continue; }
                let old = w[j];
                let mut rho = b[j];
                for k in 0..p { rho -= g[j * p + k] * w[k]; }
                rho += xj_sq[j] * old;
                let nv = soft_thresh(rho * inv_n, l1) / (xj_sq[j] * inv_n + l2);
                w[j] = nv;
                mc = mc.max((nv - old).abs());
            }
            if mc < tol { break; }
        }
        let coef: Vec<f64> = (0..p).map(|j| w[j] / gc.x_std[j]).collect();
        let intercept = gc.y_mean - crate::ml::linalg::dot(&coef, &gc.x_mean);
        predict_and_r2(fold, &coef, intercept)
    } else {
        let nf = gc.n as f64;
        let mut g = gc.xtx_c.clone();
        for i in 0..p { for j in 0..p { g[i * p + j] += nf * gc.x_mean[i] * gc.x_mean[j]; } }
        let mut bv = gc.xty_c.clone();
        for j in 0..p { bv[j] += nf * gc.x_mean[j] * gc.y_mean; }
        let xsq: Vec<f64> = (0..p).map(|j| g[j * p + j]).collect();
        let mut w = vec![0.0; p];
        for _ in 0..max_iter {
            let mut mc = 0.0f64;
            for j in 0..p {
                if xsq[j] < 1e-15 { continue; }
                let old = w[j];
                let mut rho = bv[j];
                for k in 0..p { rho -= g[j * p + k] * w[k]; }
                rho += xsq[j] * old;
                let nv = soft_thresh(rho * inv_n, l1) / (xsq[j] * inv_n + l2);
                w[j] = nv;
                mc = mc.max((nv - old).abs());
            }
            if mc < tol { break; }
        }
        predict_and_r2(fold, &w, 0.0)
    }
}

fn gs_lasso_gram_eval(fold: &FoldData, gc: &GramCache, alpha: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> f64 {
    gs_elasticnet_gram_eval(fold, gc, alpha, 1.0, max_iter, tol, fit_intercept)
}

fn gs_linear_gram_eval(fold: &FoldData, gc: &GramCache, fit_intercept: bool) -> f64 {
    gs_ridge_gram(fold, gc, 0.0, fit_intercept)
}

fn gs_knn_cached_cls(fold: &FoldData, cache: &KnnDistCache, k: usize, weights: crate::ml::neighbors::knn::KnnWeights) -> f64 {
    let n_test = fold.n_test;
    let y_train = &fold.y_train_i;
    let y_test = &fold.y_test_i;
    let use_dist = matches!(weights, crate::ml::neighbors::knn::KnnWeights::Distance);
    let mut correct = 0usize;
    for i in 0..n_test {
        let nn = &cache.nn[i][..k.min(cache.nn[i].len())];
        let pred = if use_dist {
            let mut best_cls = y_train[nn[0].1];
            let mut best_w = 0.0f64;
            let mut wts = [0.0f64; 256];
            let mut cls_map: Vec<i32> = Vec::new();
            for &(d_sq, idx) in nn {
                let label = y_train[idx];
                let ci = match cls_map.iter().position(|&c| c == label) {
                    Some(p) => p, None => { cls_map.push(label); cls_map.len() - 1 }
                };
                let w = 1.0 / d_sq.sqrt().max(1e-10);
                wts[ci] += w;
                if wts[ci] > best_w { best_w = wts[ci]; best_cls = label; }
            }
            best_cls
        } else {
            let mut best_cls = y_train[nn[0].1];
            let mut best_c = 0u32;
            let mut counts = [0u32; 256];
            let mut cls_map: Vec<i32> = Vec::new();
            for &(_, idx) in nn {
                let label = y_train[idx];
                let ci = match cls_map.iter().position(|&c| c == label) {
                    Some(p) => p, None => { cls_map.push(label); cls_map.len() - 1 }
                };
                counts[ci] += 1;
                if counts[ci] > best_c { best_c = counts[ci]; best_cls = label; }
            }
            best_cls
        };
        if pred == y_test[i] { correct += 1; }
    }
    correct as f64 / n_test as f64
}

fn gs_knn_cached_reg(fold: &FoldData, cache: &KnnDistCache, k: usize, weights: crate::ml::neighbors::knn::KnnWeights) -> f64 {
    let n_test = fold.n_test;
    let y_train = &fold.y_train_f;
    let y_test = &fold.y_test_f;
    let use_dist = matches!(weights, crate::ml::neighbors::knn::KnnWeights::Distance);
    let y_mean = y_test.iter().sum::<f64>() / n_test as f64;
    let (mut ss_res, mut ss_tot) = (0.0, 0.0);
    for i in 0..n_test {
        let nn = &cache.nn[i][..k.min(cache.nn[i].len())];
        let pred = if use_dist {
            let (mut ws, mut vs) = (0.0, 0.0);
            for &(d_sq, idx) in nn { let w = 1.0 / d_sq.sqrt().max(1e-10); vs += w * y_train[idx]; ws += w; }
            vs / ws
        } else {
            nn.iter().map(|&(_, idx)| y_train[idx]).sum::<f64>() / nn.len() as f64
        };
        let e = y_test[i] - pred; ss_res += e * e;
        let d = y_test[i] - y_mean; ss_tot += d * d;
    }
    if ss_tot < 1e-15 { 0.0 } else { 1.0 - ss_res / ss_tot }
}

fn gs_logistic_irls(fold: &FoldData, c: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> f64 {
    let n = fold.n_train;
    let p = fold.p;
    let x = &fold.x_train;
    let y = &fold.y_train_i;
    let lambda = 1.0 / c;

    let mut classes = y.to_vec();
    classes.sort_unstable();
    classes.dedup();
    if classes.len() != 2 {
        return gs_logistic_cls(fold, c, max_iter, tol, fit_intercept);
    }

    let y_bin: Vec<f64> = y.iter().map(|&v| if v == classes[1] { 1.0 } else { 0.0 }).collect();
    let dim = if fit_intercept { p + 1 } else { p };
    let inv_n = 1.0 / n as f64;
    let mut w = vec![0.0; dim];

    for _ in 0..max_iter {
        let mut probs = vec![0.0; n];
        for i in 0..n {
            let mut z = if fit_intercept { w[p] } else { 0.0 };
            for j in 0..p { z += x[i * p + j] * w[j]; }
            z = z.clamp(-500.0, 500.0);
            probs[i] = (1.0 / (1.0 + (-z).exp())).clamp(1e-12, 1.0 - 1e-12);
        }

        let mut hess = vec![0.0; dim * dim];
        let mut grad = vec![0.0; dim];
        for k in 0..n {
            let wk = probs[k] * (1.0 - probs[k]);
            let dk = probs[k] - y_bin[k];
            for i in 0..p {
                let xi = x[k * p + i];
                grad[i] += xi * dk;
                for j in i..p { hess[i * dim + j] += xi * x[k * p + j] * wk; }
                if fit_intercept { hess[i * dim + p] += xi * wk; }
            }
            if fit_intercept { grad[p] += dk; hess[p * dim + p] += wk; }
        }
        for i in 0..dim {
            grad[i] *= inv_n;
            for j in i..dim { hess[i * dim + j] *= inv_n; hess[j * dim + i] = hess[i * dim + j]; }
        }
        for i in 0..p { grad[i] += lambda * w[i]; hess[i * dim + i] += lambda; }

        match crate::ml::linalg::solve_spd(&hess, dim, &grad) {
            Some(d) => {
                let mut mc = 0.0f64;
                for i in 0..dim { w[i] -= d[i]; mc = mc.max(d[i].abs()); }
                if mc < tol { break; }
            }
            None => break,
        }
    }

    let n_test = fold.n_test;
    let y_test = &fold.y_test_i;
    let mut correct = 0usize;
    for i in 0..n_test {
        let mut z = if fit_intercept { w[p] } else { 0.0 };
        for j in 0..p { z += fold.x_test[i * p + j] * w[j]; }
        let pred = if z >= 0.0 { classes[1] } else { classes[0] };
        if pred == y_test[i] { correct += 1; }
    }
    correct as f64 / n_test as f64
}

fn gs_logistic_irls_cached(fold: &FoldData, ic: &IrlsCache, c: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> f64 {
    if !ic.is_binary {
        return gs_logistic_cls(fold, c, max_iter, tol, fit_intercept);
    }
    let n = fold.n_train;
    let p = fold.p;
    let xt = &ic.xt;
    let y_bin = &ic.y_bin;
    let lambda = 1.0 / c;
    let dim = if fit_intercept { p + 1 } else { p };
    let inv_n = 1.0 / n as f64;
    let mut w = vec![0.0; dim];
    let mut probs = vec![0.0; n];
    let mut dk = vec![0.0; n];
    let mut wk = vec![0.0; n];

    for _ in 0..max_iter {
        if fit_intercept { probs.fill(w[p]); } else { probs.fill(0.0); }
        for j in 0..p {
            let xj = &xt[j * n..(j + 1) * n];
            let wj = w[j];
            for i in 0..n { probs[i] += xj[i] * wj; }
        }
        for i in 0..n {
            let z = probs[i].clamp(-500.0, 500.0);
            let s = (1.0 / (1.0 + (-z).exp())).clamp(1e-12, 1.0 - 1e-12);
            dk[i] = s - y_bin[i];
            wk[i] = s * (1.0 - s);
        }

        let mut hess = vec![0.0; dim * dim];
        let mut grad = vec![0.0; dim];
        for i in 0..p {
            let xi = &xt[i * n..(i + 1) * n];
            let mut g = 0.0;
            for k in 0..n { g += xi[k] * dk[k]; }
            grad[i] = g * inv_n + lambda * w[i];
            for j in i..p {
                let xj = &xt[j * n..(j + 1) * n];
                let mut s = 0.0;
                for k in 0..n { s += xi[k] * xj[k] * wk[k]; }
                let v = s * inv_n + if i == j { lambda } else { 0.0 };
                hess[i * dim + j] = v;
                hess[j * dim + i] = v;
            }
            if fit_intercept {
                let mut s = 0.0;
                for k in 0..n { s += xi[k] * wk[k]; }
                hess[i * dim + p] = s * inv_n;
                hess[p * dim + i] = s * inv_n;
            }
        }
        if fit_intercept {
            let mut g = 0.0;
            let mut s = 0.0;
            for k in 0..n { g += dk[k]; s += wk[k]; }
            grad[p] = g * inv_n;
            hess[p * dim + p] = s * inv_n;
        }

        match crate::ml::linalg::solve_spd(&hess, dim, &grad) {
            Some(d) => {
                let mut mc = 0.0f64;
                for i in 0..dim { w[i] -= d[i]; mc = mc.max(d[i].abs()); }
                if mc < tol { break; }
            }
            None => break,
        }
    }

    let n_test = fold.n_test;
    let y_test = &fold.y_test_i;
    let mut correct = 0usize;
    for i in 0..n_test {
        let mut z = if fit_intercept { w[p] } else { 0.0 };
        for j in 0..p { z += fold.x_test[i * p + j] * w[j]; }
        let pred = if z >= 0.0 { ic.classes[1] } else { ic.classes[0] };
        if pred == y_test[i] { correct += 1; }
    }
    correct as f64 / n_test as f64
}

pub fn eval_model_cached(
    estimator: &str,
    param_names: &[String],
    param_values: &[Vec<String>],
    param_sizes: &[usize],
    combo_idx: usize,
    fold: &FoldData,
    cache: &ModelCache,
) -> f64 {
    let indices = decode_combo(combo_idx, param_sizes);
    let vals: Vec<String> = indices.iter().enumerate()
        .map(|(i, &idx)| param_values[i][idx].clone()).collect();
    let n = param_names;
    let v = &vals;
    match (estimator, cache) {
        ("Ridge", ModelCache::Gram(g)) =>
            gs_ridge_gram(fold, g, gf(n, v, "alpha", 1.0), gb(n, v, "fit_intercept", true)),
        ("Lasso", ModelCache::Gram(g)) =>
            gs_lasso_gram_eval(fold, g, gf(n, v, "alpha", 1.0), gu(n, v, "max_iter", 1000), gf(n, v, "tol", 1e-4), gb(n, v, "fit_intercept", true)),
        ("ElasticNet", ModelCache::Gram(g)) =>
            gs_elasticnet_gram_eval(fold, g, gf(n, v, "alpha", 1.0), gf(n, v, "l1_ratio", 0.5), gu(n, v, "max_iter", 1000), gf(n, v, "tol", 1e-4), gb(n, v, "fit_intercept", true)),
        ("LinearRegression", ModelCache::Gram(g)) =>
            gs_linear_gram_eval(fold, g, gb(n, v, "fit_intercept", true)),
        ("KNeighborsClassifier", ModelCache::Knn(kc)) => {
            let wt = if gs(n, v, "weights", "uniform") == "distance" { crate::ml::neighbors::knn::KnnWeights::Distance } else { crate::ml::neighbors::knn::KnnWeights::Uniform };
            gs_knn_cached_cls(fold, kc, gu(n, v, "n_neighbors", 5), wt)
        }
        ("KNeighborsRegressor", ModelCache::Knn(kc)) => {
            let wt = if gs(n, v, "weights", "uniform") == "distance" { crate::ml::neighbors::knn::KnnWeights::Distance } else { crate::ml::neighbors::knn::KnnWeights::Uniform };
            gs_knn_cached_reg(fold, kc, gu(n, v, "n_neighbors", 5), wt)
        }
        ("LogisticRegression", ModelCache::Irls(ic)) =>
            gs_logistic_irls_cached(fold, ic, gf(n, v, "C", 1.0), gu(n, v, "max_iter", 100), gf(n, v, "tol", 1e-4), gb(n, v, "fit_intercept", true)),
        ("LogisticRegression", _) =>
            gs_logistic_irls(fold, gf(n, v, "C", 1.0), gu(n, v, "max_iter", 100), gf(n, v, "tol", 1e-4), gb(n, v, "fit_intercept", true)),
        _ => eval_model(estimator, param_names, param_values, param_sizes, combo_idx, fold),
    }
}

pub fn grid_search_parallel_cached<F>(
    n_combos: usize,
    folds: &[FoldData],
    caches: &[ModelCache],
    eval_fn: F,
) -> GridSearchResult
where F: Fn(usize, &FoldData, &ModelCache) -> f64 + Send + Sync,
{
    let all_scores: Vec<Vec<f64>> = (0..n_combos)
        .into_par_iter()
        .map(|ci| folds.iter().zip(caches.iter())
            .map(|(fold, cache)| eval_fn(ci, fold, cache))
            .collect::<Vec<f64>>())
        .collect();
    finalise_results(all_scores, None)
}

pub fn randomized_search_parallel_cached<F>(
    combo_indices: &[usize],
    folds: &[FoldData],
    caches: &[ModelCache],
    eval_fn: F,
) -> GridSearchResult
where F: Fn(usize, &FoldData, &ModelCache) -> f64 + Send + Sync,
{
    let all_scores: Vec<Vec<f64>> = combo_indices
        .par_iter()
        .map(|&ci| folds.iter().zip(caches.iter())
            .map(|(fold, cache)| eval_fn(ci, fold, cache))
            .collect::<Vec<f64>>())
        .collect();
    finalise_results(all_scores, Some(combo_indices))
}

pub fn grid_search_parallel_cached_resumable<F>(
    total: usize,
    folds: &[FoldData],
    caches: &[ModelCache],
    task_id: Option<u64>,
    eval_fn: F,
) -> GridSearchResult
where F: Fn(usize, &FoldData, &ModelCache) -> f64 + Send + Sync,
{
    use crate::ml::cache::{task_load, task_update, task_complete, PartialState};
    let mut partial = task_id
        .and_then(|id| task_load(id))
        .filter(|e| matches!(e.status, crate::ml::cache::TaskStatus::Running { .. }))
        .map(|e| e.partial)
        .unwrap_or_default();
    let done: std::collections::HashSet<usize> = partial.combo_fold_scores.keys()
        .filter_map(|k| k.parse().ok())
        .collect();
    let remaining: Vec<usize> = (0..total).filter(|i| !done.contains(i)).collect();
    let n_folds = folds.len();
    let batch_size = if task_id.is_some() { (remaining.len() / 8 + 1).min(50).max(1) } else { remaining.len().max(1) };
    for batch in remaining.chunks(batch_size) {
        let scores: Vec<(usize, Vec<f64>)> = batch.par_iter().map(|&ci| {
            let fs: Vec<f64> = folds.iter().zip(caches.iter())
                .map(|(fold, cache)| eval_fn(ci, fold, cache))
                .collect();
            (ci, fs)
        }).collect();
        for (ci, fs) in scores { partial.combo_fold_scores.insert(ci.to_string(), fs); }
        if let Some(id) = task_id {
            task_update(id, &partial, partial.combo_fold_scores.len() as f32 / total as f32);
        }
    }
    if let Some(id) = task_id { task_complete(id, &partial); }
    let all_scores: Vec<Vec<f64>> = (0..total)
        .map(|i| partial.combo_fold_scores.get(&i.to_string())
            .cloned()
            .unwrap_or_else(|| vec![f64::NEG_INFINITY; n_folds]))
        .collect();
    finalise_results(all_scores, None)
}

pub fn randomized_search_parallel_cached_resumable<F>(
    combo_indices: &[usize],
    folds: &[FoldData],
    caches: &[ModelCache],
    task_id: Option<u64>,
    eval_fn: F,
) -> GridSearchResult
where F: Fn(usize, &FoldData, &ModelCache) -> f64 + Send + Sync,
{
    use crate::ml::cache::{task_load, task_update, task_complete, PartialState};
    let mut partial = task_id
        .and_then(|id| task_load(id))
        .filter(|e| matches!(e.status, crate::ml::cache::TaskStatus::Running { .. }))
        .map(|e| e.partial)
        .unwrap_or_default();
    let done: std::collections::HashSet<usize> = partial.combo_fold_scores.keys()
        .filter_map(|k| k.parse().ok())
        .collect();
    let remaining: Vec<usize> = combo_indices.iter().copied().filter(|i| !done.contains(i)).collect();
    let total = combo_indices.len();
    let n_folds = folds.len();
    let batch_size = if task_id.is_some() { (remaining.len() / 8 + 1).min(50).max(1) } else { remaining.len().max(1) };
    for batch in remaining.chunks(batch_size) {
        let scores: Vec<(usize, Vec<f64>)> = batch.par_iter().map(|&ci| {
            let fs: Vec<f64> = folds.iter().zip(caches.iter())
                .map(|(fold, cache)| eval_fn(ci, fold, cache))
                .collect();
            (ci, fs)
        }).collect();
        for (ci, fs) in scores { partial.combo_fold_scores.insert(ci.to_string(), fs); }
        if let Some(id) = task_id {
            task_update(id, &partial, partial.combo_fold_scores.len() as f32 / total as f32);
        }
    }
    if let Some(id) = task_id { task_complete(id, &partial); }
    let all_scores: Vec<Vec<f64>> = combo_indices.iter()
        .map(|i| partial.combo_fold_scores.get(&i.to_string())
            .cloned()
            .unwrap_or_else(|| vec![f64::NEG_INFINITY; n_folds]))
        .collect();
    finalise_results(all_scores, Some(combo_indices))
}

fn score_reg(y_true: &[f64], y_pred: &[f64], scoring: &str) -> f64 {
    match scoring {
        "neg_mean_squared_error" => -crate::ml::metrics::regression::mean_squared_error(y_true, y_pred),
        "neg_mean_absolute_error" => -crate::ml::metrics::regression::mean_absolute_error(y_true, y_pred),
        _ => crate::ml::metrics::regression::r2_score(y_true, y_pred),
    }
}

fn score_cls(y_true: &[i32], y_pred: &[i32], scoring: &str) -> f64 {
    match scoring {
        "f1" | "f1_weighted" => crate::ml::metrics::classification::f1_score(y_true, y_pred, crate::ml::metrics::classification::Average::Weighted),
        "f1_macro" => crate::ml::metrics::classification::f1_score(y_true, y_pred, crate::ml::metrics::classification::Average::Macro),
        "precision" | "precision_weighted" => crate::ml::metrics::classification::precision_score(y_true, y_pred, crate::ml::metrics::classification::Average::Weighted),
        "precision_macro" => crate::ml::metrics::classification::precision_score(y_true, y_pred, crate::ml::metrics::classification::Average::Macro),
        "recall" | "recall_weighted" => crate::ml::metrics::classification::recall_score(y_true, y_pred, crate::ml::metrics::classification::Average::Weighted),
        "recall_macro" => crate::ml::metrics::classification::recall_score(y_true, y_pred, crate::ml::metrics::classification::Average::Macro),
        _ => crate::ml::metrics::classification::accuracy_score(y_true, y_pred),
    }
}

fn is_default_scoring(scoring: &str) -> bool {
    matches!(scoring, "" | "auto" | "r2" | "accuracy")
}

fn eval_model_generic_reg(fold: &FoldData, model: &mut dyn crate::ml::MlRegressor, scoring: &str) -> f64 {
    model.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_f);
    let preds = model.predict(&fold.x_test, fold.n_test, fold.p);
    score_reg(&fold.y_test_f, &preds, scoring)
}

fn eval_model_generic_cls(fold: &FoldData, model: &mut dyn crate::ml::MlClassifier, scoring: &str) -> f64 {
    model.fit(&fold.x_train, fold.n_train, fold.p, &fold.y_train_i);
    let preds = model.predict(&fold.x_test, fold.n_test, fold.p);
    score_cls(&fold.y_test_i, &preds, scoring)
}

pub fn eval_model_scored(
    estimator: &str,
    param_names: &[String],
    param_values: &[Vec<String>],
    param_sizes: &[usize],
    combo_idx: usize,
    fold: &FoldData,
    cache: &ModelCache,
    scoring: &str,
) -> f64 {
    if is_default_scoring(scoring) {
        return eval_model_cached(estimator, param_names, param_values, param_sizes, combo_idx, fold, cache);
    }
    let indices = decode_combo(combo_idx, param_sizes);
    let vals: Vec<String> = indices.iter().enumerate()
        .map(|(i, &idx)| param_values[i][idx].clone()).collect();
    let n = param_names;
    let v = &vals;
    match estimator {
        "Ridge" => {
            let mut m = crate::ml::linear::ridge::Ridge::new(gf(n, v, "alpha", 1.0), gb(n, v, "fit_intercept", true));
            eval_model_generic_reg(fold, &mut m, scoring)
        }
        "Lasso" => {
            let mut m = crate::ml::linear::lasso::Lasso::new(gf(n, v, "alpha", 1.0), gu(n, v, "max_iter", 1000), gf(n, v, "tol", 1e-4), gb(n, v, "fit_intercept", true));
            eval_model_generic_reg(fold, &mut m, scoring)
        }
        "ElasticNet" => {
            let mut m = crate::ml::linear::elastic_net::ElasticNet::new(gf(n, v, "alpha", 1.0), gf(n, v, "l1_ratio", 0.5), gu(n, v, "max_iter", 1000), gf(n, v, "tol", 1e-4), gb(n, v, "fit_intercept", true));
            eval_model_generic_reg(fold, &mut m, scoring)
        }
        "LinearRegression" => {
            let mut m = crate::ml::linear::ols::LinearRegression::new(gb(n, v, "fit_intercept", true));
            eval_model_generic_reg(fold, &mut m, scoring)
        }
        "LogisticRegression" => {
            let mut m = crate::ml::linear::logistic::LogisticRegression::new(gf(n, v, "C", 1.0), gu(n, v, "max_iter", 100), gf(n, v, "tol", 1e-4), gb(n, v, "fit_intercept", true));
            eval_model_generic_cls(fold, &mut m, scoring)
        }
        "RidgeClassifier" => {
            let mut m = crate::ml::linear::ridge::RidgeClassifier::new(gf(n, v, "alpha", 1.0));
            eval_model_generic_cls(fold, &mut m, scoring)
        }
        "SGDClassifier" => {
            let loss = match gs(n, v, "loss", "hinge") {
                "log" | "log_loss" => crate::ml::linear::sgd::SGDLoss::Log,
                "modified_huber" => crate::ml::linear::sgd::SGDLoss::ModifiedHuber,
                "squared_hinge" => crate::ml::linear::sgd::SGDLoss::SquaredHinge,
                _ => crate::ml::linear::sgd::SGDLoss::Hinge,
            };
            let mut m = crate::ml::linear::sgd::SGDClassifier::new(loss, gf(n, v, "alpha", 0.0001), gu(n, v, "max_iter", 1000), gf(n, v, "tol", 1e-3), gf(n, v, "eta0", 0.01), gb(n, v, "fit_intercept", true));
            eval_model_generic_cls(fold, &mut m, scoring)
        }
        "SGDRegressor" => {
            let mut m = crate::ml::linear::sgd::SGDRegressor::new(gf(n, v, "alpha", 0.0001), gu(n, v, "max_iter", 1000), gf(n, v, "tol", 1e-3), gf(n, v, "eta0", 0.01), gb(n, v, "fit_intercept", true));
            eval_model_generic_reg(fold, &mut m, scoring)
        }
        "DecisionTreeClassifier" => {
            let criterion = if gs(n, v, "criterion", "gini") == "entropy" { crate::ml::tree::decision_tree::TreeCriterion::Entropy } else { crate::ml::tree::decision_tree::TreeCriterion::Gini };
            let mf = n.iter().position(|k| k == "max_features").and_then(|i| vals[i].parse::<usize>().ok());
            let mut m = crate::ml::tree::decision_tree::DecisionTreeClassifier::new(gu(n, v, "max_depth", 10), gu(n, v, "min_samples_split", 2), gu(n, v, "min_samples_leaf", 1), mf, criterion);
            eval_model_generic_cls(fold, &mut m, scoring)
        }
        "DecisionTreeRegressor" => {
            let mf = n.iter().position(|k| k == "max_features").and_then(|i| vals[i].parse::<usize>().ok());
            let mut m = crate::ml::tree::decision_tree::DecisionTreeRegressor::new(gu(n, v, "max_depth", 10), gu(n, v, "min_samples_split", 2), gu(n, v, "min_samples_leaf", 1), mf);
            eval_model_generic_reg(fold, &mut m, scoring)
        }
        "RandomForestClassifier" => {
            let mf = match gs(n, v, "max_features", "sqrt") {
                "log2" => crate::ml::tree::random_forest::MaxFeatures::Log2,
                "all" | "none" => crate::ml::tree::random_forest::MaxFeatures::All,
                s => s.parse::<usize>().map(crate::ml::tree::random_forest::MaxFeatures::Fixed).unwrap_or(crate::ml::tree::random_forest::MaxFeatures::Sqrt),
            };
            let mut m = crate::ml::tree::random_forest::RandomForestClassifier::new(gu(n, v, "n_estimators", 100), gu(n, v, "max_depth", 10), gu(n, v, "min_samples_split", 2), gu(n, v, "min_samples_leaf", 1), mf);
            eval_model_generic_cls(fold, &mut m, scoring)
        }
        "RandomForestRegressor" => {
            let mf = match gs(n, v, "max_features", "sqrt") {
                "log2" => crate::ml::tree::random_forest::MaxFeatures::Log2,
                "all" | "none" => crate::ml::tree::random_forest::MaxFeatures::All,
                s => s.parse::<usize>().map(crate::ml::tree::random_forest::MaxFeatures::Fixed).unwrap_or(crate::ml::tree::random_forest::MaxFeatures::Sqrt),
            };
            let mut m = crate::ml::tree::random_forest::RandomForestRegressor::new(gu(n, v, "n_estimators", 100), gu(n, v, "max_depth", 10), gu(n, v, "min_samples_split", 2), gu(n, v, "min_samples_leaf", 1), mf);
            eval_model_generic_reg(fold, &mut m, scoring)
        }
        "GradientBoostingClassifier" => {
            let mut m = crate::ml::tree::gradient_boosting::GradientBoostingClassifier::new(gu(n, v, "n_estimators", 100), gf(n, v, "learning_rate", 0.1), gu(n, v, "max_depth", 3), gu(n, v, "min_samples_split", 2), gu(n, v, "min_samples_leaf", 1));
            eval_model_generic_cls(fold, &mut m, scoring)
        }
        "GradientBoostingRegressor" => {
            let mut m = crate::ml::tree::gradient_boosting::GradientBoostingRegressor::new(gu(n, v, "n_estimators", 100), gf(n, v, "learning_rate", 0.1), gu(n, v, "max_depth", 3), gu(n, v, "min_samples_split", 2), gu(n, v, "min_samples_leaf", 1));
            eval_model_generic_reg(fold, &mut m, scoring)
        }
        "AdaBoostClassifier" => {
            let mut m = crate::ml::tree::adaboost::AdaBoostClassifier::new(gu(n, v, "n_estimators", 50), gf(n, v, "learning_rate", 1.0), gu(n, v, "max_depth", 1));
            eval_model_generic_cls(fold, &mut m, scoring)
        }
        "AdaBoostRegressor" => {
            let mut m = crate::ml::tree::adaboost::AdaBoostRegressor::new(gu(n, v, "n_estimators", 50), gf(n, v, "learning_rate", 1.0), gu(n, v, "max_depth", 1));
            eval_model_generic_reg(fold, &mut m, scoring)
        }
        "KNeighborsClassifier" => {
            let w = if gs(n, v, "weights", "uniform") == "distance" { crate::ml::neighbors::knn::KnnWeights::Distance } else { crate::ml::neighbors::knn::KnnWeights::Uniform };
            let mut m = crate::ml::neighbors::knn::KNeighborsClassifier::new(gu(n, v, "n_neighbors", 5), w);
            eval_model_generic_cls(fold, &mut m, scoring)
        }
        "KNeighborsRegressor" => {
            let w = if gs(n, v, "weights", "uniform") == "distance" { crate::ml::neighbors::knn::KnnWeights::Distance } else { crate::ml::neighbors::knn::KnnWeights::Uniform };
            let mut m = crate::ml::neighbors::knn::KNeighborsRegressor::new(gu(n, v, "n_neighbors", 5), w);
            eval_model_generic_reg(fold, &mut m, scoring)
        }
        "GaussianNB" => {
            let mut m = crate::ml::naive_bayes::gaussian::GaussianNB::new();
            eval_model_generic_cls(fold, &mut m, scoring)
        }
        "LinearSVC" => {
            let mut m = crate::ml::svm::svm::LinearSVC::new(gf(n, v, "C", 1.0), gu(n, v, "max_iter", 1000), gf(n, v, "tol", 1e-4));
            eval_model_generic_cls(fold, &mut m, scoring)
        }
        "LinearSVR" => {
            let mut m = crate::ml::svm::svm::LinearSVR::new(gf(n, v, "C", 1.0), gf(n, v, "epsilon", 0.1), gu(n, v, "max_iter", 1000), gf(n, v, "tol", 1e-4));
            eval_model_generic_reg(fold, &mut m, scoring)
        }
        _ => f64::NEG_INFINITY,
    }
}
