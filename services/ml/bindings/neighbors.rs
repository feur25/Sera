use super::helpers::*;

#[crate::sera_doc(
    category = "Neighbors",
    en = "K-Nearest Neighbors classifier — majority vote among k nearest neighbors.",
    fr = "K plus proches voisins classifieur — vote majoritaire parmi les k voisins les plus proches.",
    file = "knn.md"
)]
#[crate::sera_alias("knn_classifier", "knn_cls")]
#[crate::sera_builder]
pub fn ml_knn_classifier(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let k = ju(&v, "n_neighbors", 5);
    let weights = parse_knn_weights(js(&v, "weights", "uniform"));
    let mut model = crate::ml::neighbors::knn::KNeighborsClassifier::new(k, weights);
    let y = yi(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default()
    )
}

#[crate::sera_doc(
    category = "Neighbors",
    en = "K-Nearest Neighbors regressor — distance-based average among k nearest neighbors.",
    fr = "K plus proches voisins regresseur — moyenne par distance parmi les k voisins les plus proches.",
    file = "knn.md"
)]
#[crate::sera_alias("knn_regressor", "knn_reg")]
#[crate::sera_builder]
pub fn ml_knn_regressor(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let k = ju(&v, "n_neighbors", 5);
    let weights = parse_knn_weights(js(&v, "weights", "uniform"));
    let mut model = crate::ml::neighbors::knn::KNeighborsRegressor::new(k, weights);
    let y = yf(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default()
    )
}

#[crate::sera_doc(
    category = "Neighbors",
    en = "NearestCentroid — centroid classifier using class means.",
    fr = "NearestCentroid — classifieur par centroide utilisant les moyennes de classes.",
    file = "knn.md"
)]
#[crate::sera_alias("nearest_centroid")]
#[crate::sera_builder]
pub fn ml_nearest_centroid(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let mut model = crate::ml::neighbors::knn::NearestCentroid::new();
    let y = yi(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default()
    )
}
