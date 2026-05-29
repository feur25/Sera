use super::helpers::*;

#[crate::sera_doc(category = "Decomposition", en = "PCA — Principal Component Analysis via truncated SVD.", fr = "PCA — Analyse en Composantes Principales via SVD tronquée.", file = "decomposition.md")]
#[crate::sera_alias("pca")]
pub fn ml_pca(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let n_components = ju(&v, "n_components", p.min(2));
    let mut model = crate::ml::decomposition::pca::PCA::new(n_components);
    model.fit(&xf, n, p);
    let transformed = if nt > 0 { model.transform(&xtf, nt, p) } else { model.transform(&xf, n, p) };
    let out_n = if nt > 0 { nt } else { n };
    let rows: Vec<Vec<f64>> = (0..out_n).map(|i| transformed[i*n_components..(i+1)*n_components].to_vec()).collect();
    format!(r#"{{"components":{},"explained_variance_ratio":{}}}"#,
        serde_json::to_string(&rows).unwrap_or_default(),
        serde_json::to_string(&model.explained_variance_ratio).unwrap_or_default())
}

#[crate::sera_alias("truncated_svd")]
pub fn ml_truncated_svd(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let n_components = ju(&v, "n_components", p.min(2));
    let mut model = crate::ml::decomposition::pca::TruncatedSVD::new(n_components);
    model.fit(&xf, n, p);
    let transformed = if nt > 0 { model.transform(&xtf, nt, p) } else { model.transform(&xf, n, p) };
    let out_n = if nt > 0 { nt } else { n };
    let rows: Vec<Vec<f64>> = (0..out_n).map(|i| transformed[i*n_components..(i+1)*n_components].to_vec()).collect();
    format!(r#"{{"components":{}}}"#, serde_json::to_string(&rows).unwrap_or_default())
}
