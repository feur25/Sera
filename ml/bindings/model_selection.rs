use super::helpers::*;

#[crate::sera_doc(
    category = "Model Selection",
    en = "K-Fold split — returns train/test index arrays for k-fold cross-validation.",
    fr = "Division K-Fold — retourne les tableaux d'indices train/test pour la validation croisée k-fold.",
    file = "cv-splitters.md"
)]
#[crate::sera_alias("kfold_split")]
pub fn ml_kfold_split(input: &str) -> String {
    let v: serde_json::Value = serde_json::from_str(input).unwrap_or(serde_json::Value::Null);
    let n = ju(&v, "n", 0);
    let k = ju(&v, "k", 5);
    let seed = v.get("seed").and_then(|x| x.as_u64()).unwrap_or(42);
    let stratified = jb(&v, "stratified", false);
    let folds: Vec<(Vec<usize>, Vec<usize>)> = if stratified {
        let y = yi(&v);
        crate::ml::model_selection::split::stratified_kfold_indices(&y, k, seed)
    } else {
        crate::ml::model_selection::split::kfold_indices(n, k, seed)
    };
    let result: Vec<serde_json::Value> = folds
        .into_iter()
        .map(|(train, test)| serde_json::json!({"train": train, "test": test}))
        .collect();
    serde_json::to_string(&result).unwrap_or_default()
}

#[crate::sera_doc(
    category = "Model Selection",
    en = "cross_val_score — evaluates a Ridge model with k-fold cross-validation.",
    fr = "cross_val_score — evalue un modele Ridge par validation croisee k-fold.",
    file = "cv-splitters.md"
)]
#[crate::sera_alias("cross_val_score")]
pub fn ml_cross_val_score(input: &str) -> String {
    let (v, xf, n, p, _, _) = ml_parse(input);
    let k = ju(&v, "k", 5);
    let seed = v.get("seed").and_then(|x| x.as_u64()).unwrap_or(42);
    let is_cls = jb(&v, "classification", false);
    let scores: Vec<f64> = if is_cls {
        let y = yi(&v);
        let alpha = jf(&v, "alpha", 1.0);
        crate::ml::model_selection::cross_val::cross_val_score_cls(
            &xf,
            n,
            p,
            &y,
            k,
            seed,
            move |xtr, ntr, ptr, ytr, xte, nte| {
                let mut m = crate::ml::linear::ridge::RidgeClassifier::new(alpha);
                m.fit(xtr, ntr, ptr, ytr);
                m.predict(xte, nte, ptr)
            },
        )
    } else {
        let y = yf(&v);
        let alpha = jf(&v, "alpha", 1.0);
        crate::ml::model_selection::cross_val::cross_val_score_reg(
            &xf,
            n,
            p,
            &y,
            k,
            seed,
            move |xtr, ntr, ptr, ytr, xte, nte| {
                let mut m = crate::ml::linear::ridge::Ridge::new(alpha, true);
                m.fit(xtr, ntr, ptr, ytr);
                m.predict(xte, nte, ptr)
            },
        )
    };
    format!(
        r#"{{"scores":{},"mean":{}}}"#,
        serde_json::to_string(&scores).unwrap_or_default(),
        scores.iter().sum::<f64>() / scores.len().max(1) as f64
    )
}

#[crate::sera_doc(
    category = "Model Selection",
    en = "GridSearchCV — searches alpha values with k-fold validation.",
    fr = "GridSearchCV — cherche des valeurs alpha avec validation k-fold.",
    file = "grid-search.md"
)]
#[crate::sera_alias("grid_search_cv")]
pub fn ml_grid_search_cv(input: &str) -> String {
    let (v, xf, n, p, _, _) = ml_parse(input);
    let k = ju(&v, "k", 5);
    let seed = v.get("seed").and_then(|x| x.as_u64()).unwrap_or(42);
    let alphas: Vec<f64> = v
        .get("alphas")
        .and_then(|x| serde_json::from_value(x.clone()).ok())
        .unwrap_or_else(|| vec![0.01, 0.1, 1.0, 10.0]);
    let y = yf(&v);
    let fold_data =
        crate::ml::model_selection::grid_search::precompute_folds_reg(&xf, n, p, &y, k, seed);
    let mut best_alpha = alphas[0];
    let mut best_score = f64::NEG_INFINITY;
    for alpha in &alphas {
        let score: f64 = fold_data
            .iter()
            .map(|fd| crate::ml::model_selection::grid_search::gs_ridge_reg(fd, *alpha, true))
            .sum::<f64>()
            / fold_data.len() as f64;
        if score > best_score {
            best_score = score;
            best_alpha = *alpha;
        }
    }
    format!(
        r#"{{"best_alpha":{},"best_score":{}}}"#,
        best_alpha, best_score
    )
}
