use super::helpers::*;

#[crate::sera_doc(
    category = "Tree-Based",
    en = "Random Forest classifier — bagging of CART trees with feature subsampling.",
    fr = "Random Forest classifieur — bagging d'arbres CART avec sous-échantillonnage de features.",
    file = "random-forest.md"
)]
#[crate::sera_alias("random_forest_classifier", "rf_cls")]
pub fn ml_random_forest_classifier(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let n_estimators = ju(&v, "n_estimators", 100);
    let max_depth = ju(&v, "max_depth", usize::MAX);
    let min_samples_split = ju(&v, "min_samples_split", 2);
    let min_samples_leaf = ju(&v, "min_samples_leaf", 1);
    let max_features = parse_max_features(&v);
    let mut model = crate::ml::tree::random_forest::RandomForestClassifier::new(
        n_estimators,
        max_depth,
        min_samples_split,
        min_samples_leaf,
        max_features,
    );
    let y = yi(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{},"feature_importances":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default(),
        serde_json::to_string(&model.feature_importances).unwrap_or_default()
    )
}

#[crate::sera_alias("random_forest_regressor", "rf_reg")]
pub fn ml_random_forest_regressor(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let n_estimators = ju(&v, "n_estimators", 100);
    let max_depth = ju(&v, "max_depth", usize::MAX);
    let min_samples_split = ju(&v, "min_samples_split", 2);
    let min_samples_leaf = ju(&v, "min_samples_leaf", 1);
    let max_features = parse_max_features(&v);
    let mut model = crate::ml::tree::random_forest::RandomForestRegressor::new(
        n_estimators,
        max_depth,
        min_samples_split,
        min_samples_leaf,
        max_features,
    );
    let y = yf(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{},"feature_importances":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default(),
        serde_json::to_string(&model.feature_importances).unwrap_or_default()
    )
}

#[crate::sera_alias("gradient_boosting_classifier", "gb_cls")]
pub fn ml_gradient_boosting_classifier(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let n_estimators = ju(&v, "n_estimators", 100);
    let learning_rate = jf(&v, "learning_rate", 0.1);
    let max_depth = ju(&v, "max_depth", 3);
    let min_samples_split = ju(&v, "min_samples_split", 2);
    let min_samples_leaf = ju(&v, "min_samples_leaf", 1);
    let mut model = crate::ml::tree::gradient_boosting::GradientBoostingClassifier::new(
        n_estimators,
        learning_rate,
        max_depth,
        min_samples_split,
        min_samples_leaf,
    );
    let y = yi(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default()
    )
}

#[crate::sera_alias("gradient_boosting_regressor", "gb_reg")]
pub fn ml_gradient_boosting_regressor(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let n_estimators = ju(&v, "n_estimators", 100);
    let learning_rate = jf(&v, "learning_rate", 0.1);
    let max_depth = ju(&v, "max_depth", 3);
    let min_samples_split = ju(&v, "min_samples_split", 2);
    let min_samples_leaf = ju(&v, "min_samples_leaf", 1);
    let mut model = crate::ml::tree::gradient_boosting::GradientBoostingRegressor::new(
        n_estimators,
        learning_rate,
        max_depth,
        min_samples_split,
        min_samples_leaf,
    );
    let y = yf(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default()
    )
}

#[crate::sera_alias("adaboost_classifier", "ada_cls")]
pub fn ml_adaboost_classifier(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let n_estimators = ju(&v, "n_estimators", 50);
    let learning_rate = jf(&v, "learning_rate", 1.0);
    let max_depth = ju(&v, "max_depth", 1);
    let mut model =
        crate::ml::tree::adaboost::AdaBoostClassifier::new(n_estimators, learning_rate, max_depth);
    let y = yi(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default()
    )
}

#[crate::sera_alias("adaboost_regressor", "ada_reg")]
pub fn ml_adaboost_regressor(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let n_estimators = ju(&v, "n_estimators", 50);
    let learning_rate = jf(&v, "learning_rate", 1.0);
    let max_depth = ju(&v, "max_depth", 3);
    let mut model =
        crate::ml::tree::adaboost::AdaBoostRegressor::new(n_estimators, learning_rate, max_depth);
    let y = yf(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default()
    )
}
