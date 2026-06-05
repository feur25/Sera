use super::helpers::*;

#[crate::sera_doc(
    category = "Tree-Based",
    en = "Decision tree classifier — CART with Gini/Entropy criterion, binned splits.",
    fr = "Arbre de décision classifieur — CART avec critère Gini/Entropie, splits binnés.",
    file = "decision-tree.md"
)]
#[crate::sera_alias("decision_tree_classifier", "dt_cls")]
pub fn ml_decision_tree_classifier(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let max_depth = ju(&v, "max_depth", usize::MAX);
    let min_samples_split = ju(&v, "min_samples_split", 2);
    let min_samples_leaf = ju(&v, "min_samples_leaf", 1);
    let mf = parse_max_features(&v);
    let max_features = match mf {
        crate::ml::tree::random_forest::MaxFeatures::Fixed(k) => Some(k),
        _ => None,
    };
    let criterion = parse_tree_criterion(js(&v, "criterion", "gini"));
    let mut model = crate::ml::tree::decision_tree::DecisionTreeClassifier::new(
        max_depth,
        min_samples_split,
        min_samples_leaf,
        max_features,
        criterion,
    );
    let y = yi(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{},"feature_importances":{},"classes":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default(),
        serde_json::to_string(&model.feature_importances).unwrap_or_default(),
        serde_json::to_string(&model.classes).unwrap_or_default()
    )
}

#[crate::sera_alias("decision_tree_regressor", "dt_reg")]
pub fn ml_decision_tree_regressor(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let max_depth = ju(&v, "max_depth", usize::MAX);
    let min_samples_split = ju(&v, "min_samples_split", 2);
    let min_samples_leaf = ju(&v, "min_samples_leaf", 1);
    let mf = parse_max_features(&v);
    let max_features = match mf {
        crate::ml::tree::random_forest::MaxFeatures::Fixed(k) => Some(k),
        _ => None,
    };
    let mut model = crate::ml::tree::decision_tree::DecisionTreeRegressor::new(
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
