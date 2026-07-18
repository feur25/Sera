use super::helpers::*;

#[crate::sera_doc(
    category = "Model Selection",
    en = "Permutation importance — feature importance by permuting each column and measuring score drop.",
    fr = "Importance par permutation — importance des features en permutant chaque colonne et mesurant la baisse de score.",
    file = "permutation-importance.md"
)]
#[crate::sera_alias("permutation_importance")]
#[crate::sera_builder]
pub fn ml_permutation_importance(input: &str) -> String {
    let (v, xf, n, p, _, _) = ml_parse(input);
    let n_repeats = ju(&v, "n_repeats", 5);
    let seed = v.get("seed").and_then(|x| x.as_u64()).unwrap_or(42);
    let is_cls = jb(&v, "classification", false);
    let alpha = jf(&v, "alpha", 1.0);
    let (means, stds) = if is_cls {
        let y = yi(&v);
        let mut m = crate::ml::linear::ridge::RidgeClassifier::new(alpha);
        m.fit(&xf, n, p, &y);
        crate::ml::model_selection::permutation::permutation_importance_cls(
            &xf,
            n,
            p,
            &y,
            n_repeats,
            seed,
            |x2, n2, p2| m.predict(x2, n2, p2),
        )
    } else {
        let y = yf(&v);
        let mut m = crate::ml::linear::ridge::Ridge::new(alpha, true);
        m.fit(&xf, n, p, &y);
        crate::ml::model_selection::permutation::permutation_importance_reg(
            &xf,
            n,
            p,
            &y,
            n_repeats,
            seed,
            |x2, n2, p2| m.predict(x2, n2, p2),
        )
    };
    format!(
        r#"{{"importances_mean":{},"importances_std":{}}}"#,
        serde_json::to_string(&means).unwrap_or_default(),
        serde_json::to_string(&stds).unwrap_or_default()
    )
}
