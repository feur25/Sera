use serde::Deserialize;

#[crate::sera_doc(
    category = "Clustering",
    en = "DBSCAN — density-based spatial clustering, no preset number of clusters.",
    fr = "DBSCAN — clustering spatial basé sur la densité, sans nombre de clusters prédéfini.",
    file = "dbscan.md"
)]
#[crate::sera_alias(
    "dbscan_fit_predict",
    "ml_dbscan",
    "DBSCAN_fit_predict",
    "cluster_dbscan"
)]
pub fn ml_dbscan_fit_predict(input: &str) -> String {
    #[derive(Deserialize, Default)]
    struct I {
        data: Option<Vec<Vec<f64>>>,
        eps: Option<f64>,
        min_samples: Option<usize>,
    }
    let i: I = serde_json::from_str(input).unwrap_or_default();
    let data = i.data.unwrap_or_default();
    let eps = i.eps.unwrap_or(0.5);
    let min_samples = i.min_samples.unwrap_or(5);
    let (labels, n_clusters) =
        crate::plot::default::scatter::dbscan_core_nd(&data, eps, min_samples);
    let n_noise = labels.iter().filter(|&&l| l < 0).count();
    format!(
        "{{\"labels\":{},\"n_clusters\":{},\"n_noise\":{}}}",
        serde_json::to_string(&labels).unwrap_or_default(),
        n_clusters,
        n_noise
    )
}

#[crate::sera_alias(
    "kmeans_fit_predict",
    "ml_kmeans",
    "KMeans_fit_predict",
    "cluster_kmeans"
)]
pub fn ml_kmeans_fit_predict(input: &str) -> String {
    #[derive(Deserialize, Default)]
    struct I {
        data: Option<Vec<Vec<f64>>>,
        k: Option<usize>,
        max_iter: Option<usize>,
        n_init: Option<usize>,
    }
    let i: I = serde_json::from_str(input).unwrap_or_default();
    let rows = i.data.unwrap_or_default();
    let k = i.k.unwrap_or(3);
    let max_iter = i.max_iter.unwrap_or(300);
    let n_init = i.n_init.unwrap_or(10);
    let n = rows.len();
    if n == 0 {
        return "{\"labels\":[],\"inertia\":0.0}".to_string();
    }
    let dims = rows[0].len();
    let mut flat = vec![0.0f64; n * dims];
    for (ri, row) in rows.iter().enumerate() {
        let len = row.len().min(dims);
        flat[ri * dims..ri * dims + len].copy_from_slice(&row[..len]);
    }
    let (labels, _, inertia) =
        crate::plot::default::kmeans_core_flat_ninit(&flat, n, dims, k, max_iter, 1e-4, n_init);
    format!(
        "{{\"labels\":{},\"inertia\":{:.6}}}",
        serde_json::to_string(&labels).unwrap_or_default(),
        inertia
    )
}
