use super::helpers::*;

#[crate::sera_doc(category = "Anomaly Detection", en = "IsolationForest — anomaly detection via random partitioning trees.", fr = "IsolationForest — détection d'anomalies via arbres de partition aléatoire.", file = "isolation-forest.md")]
#[crate::sera_alias("isolation_forest")]
pub fn ml_isolation_forest(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let n_estimators = ju(&v, "n_estimators", 100);
    let max_samples = ju(&v, "max_samples", 256);
    let contamination = jf(&v, "contamination", 0.1);
    let seed = v.get("seed").and_then(|x| x.as_u64()).unwrap_or(42);
    let mut model = crate::ml::anomaly::isolation_forest::IsolationForest::new(n_estimators, max_samples, contamination, seed);
    if nt > 0 {
        model.fit(&xf, n, p);
        let preds = model.predict(&xtf, nt, p);
        format!(r#"{{"predictions":{}}}"#, serde_json::to_string(&preds).unwrap_or_default())
    } else {
        let preds = model.fit_predict(&xf, n, p);
        format!(r#"{{"predictions":{}}}"#, serde_json::to_string(&preds).unwrap_or_default())
    }
}
