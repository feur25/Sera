use serde::Deserialize;

#[crate::sera_doc(category = "Metrics", en = "Compute a named metric score (accuracy, r2, f1, etc.) from predictions.", fr = "Calcule un score de métrique nommée (accuracy, r2, f1, etc.) à partir des prédictions.", file = "metrics.md")]
#[crate::sera_alias("metric_score", "ml_metric", "score_metric")]
pub fn ml_metric_score(input: &str) -> String {
    use crate::ml::metrics::*;
    #[derive(Deserialize, Default)]
    struct I {
        name: Option<String>,
        y_true: Option<Vec<f64>>,
        y_pred: Option<Vec<f64>>,
        y_score: Option<Vec<f64>>,
        labels: Option<Vec<i32>>,
        labels_true: Option<Vec<i32>>,
        labels_pred: Option<Vec<i32>>,
        x: Option<Vec<Vec<f64>>>,
        average: Option<String>,
        pos_label: Option<i32>,
        beta: Option<f64>,
        alpha: Option<f64>,
        eps: Option<f64>,
    }
    let i: I = serde_json::from_str(input).unwrap_or_default();
    let name: String = i.name.unwrap_or_default();
    let to_i32 = |v: &[f64]| v.iter().map(|x| *x as i32).collect::<Vec<i32>>();
    let yt_f = i.y_true.clone().unwrap_or_default();
    let yp_f = i.y_pred.clone().unwrap_or_default();
    let ys_f = i.y_score.clone().unwrap_or_default();
    let yt_i = to_i32(&yt_f);
    let yp_i = to_i32(&yp_f);
    let pos = i.pos_label.unwrap_or(1);
    let avg = match i.average.as_deref().unwrap_or("binary") {
        "macro" => Average::Macro,
        "weighted" => Average::Weighted,
        _ => Average::Binary(pos),
    };
    let value: f64 = match name.as_str() {
        "accuracy_score" => accuracy_score(&yt_i, &yp_i),
        "balanced_accuracy_score" => balanced_accuracy_score(&yt_i, &yp_i),
        "precision_score" => precision_score(&yt_i, &yp_i, avg),
        "recall_score" => recall_score(&yt_i, &yp_i, avg),
        "f1_score" => f1_score(&yt_i, &yp_i, avg),
        "fbeta_score" => fbeta_score(&yt_i, &yp_i, i.beta.unwrap_or(1.0), avg),
        "jaccard_score" => jaccard_score(&yt_i, &yp_i, pos),
        "matthews_corrcoef" => matthews_corrcoef(&yt_i, &yp_i),
        "cohen_kappa_score" => cohen_kappa_score(&yt_i, &yp_i),
        "hamming_loss" => hamming_loss(&yt_i, &yp_i),
        "zero_one_loss" => zero_one_loss(&yt_i, &yp_i),
        "binary_log_loss" => binary_log_loss(&yt_i, &yp_f, i.eps.unwrap_or(1e-15)),
        "brier_score_loss" => brier_score_loss(&yt_i, &yp_f),
        "hinge_loss" => hinge_loss(&yt_i, &yp_f),
        "roc_auc_score" => roc_auc_score(&yt_i, &ys_f, pos),
        "average_precision_score" => average_precision_score(&yt_i, &ys_f, pos),
        "mean_squared_error" => mean_squared_error(&yt_f, &yp_f),
        "root_mean_squared_error" => root_mean_squared_error(&yt_f, &yp_f),
        "mean_absolute_error" => mean_absolute_error(&yt_f, &yp_f),
        "median_absolute_error" => median_absolute_error(&yt_f, &yp_f),
        "r2_score" => r2_score(&yt_f, &yp_f),
        "explained_variance_score" => explained_variance_score(&yt_f, &yp_f),
        "max_error" => max_error(&yt_f, &yp_f),
        "mean_absolute_percentage_error" => mean_absolute_percentage_error(&yt_f, &yp_f),
        "mean_squared_log_error" => mean_squared_log_error(&yt_f, &yp_f),
        "root_mean_squared_log_error" => root_mean_squared_log_error(&yt_f, &yp_f),
        "mean_pinball_loss" => mean_pinball_loss(&yt_f, &yp_f, i.alpha.unwrap_or(0.5)),
        "d2_absolute_error_score" => d2_absolute_error_score(&yt_f, &yp_f),
        "silhouette_score" | "davies_bouldin_score" | "calinski_harabasz_score" => {
            let rows = i.x.clone().unwrap_or_default();
            let n = rows.len();
            let p = if n > 0 { rows[0].len() } else { 0 };
            let mut flat = Vec::with_capacity(n * p);
            for r in &rows { flat.extend_from_slice(&r[..p.min(r.len())]); if r.len() < p { flat.extend(std::iter::repeat(0.0).take(p - r.len())); } }
            let labs = i.labels.clone().unwrap_or_default();
            match name.as_str() {
                "silhouette_score" => silhouette_score(&flat, &labs, n, p),
                "davies_bouldin_score" => davies_bouldin_score(&flat, &labs, n, p),
                _ => calinski_harabasz_score(&flat, &labs, n, p),
            }
        }
        "adjusted_rand_score" | "normalized_mutual_info_score" | "fowlkes_mallows_score"
        | "homogeneity_score" | "completeness_score" | "v_measure_score" => {
            let lt = i.labels_true.clone().unwrap_or_default();
            let lp = i.labels_pred.clone().unwrap_or_default();
            match name.as_str() {
                "adjusted_rand_score" => adjusted_rand_score(&lt, &lp),
                "normalized_mutual_info_score" => normalized_mutual_info_score(&lt, &lp),
                "fowlkes_mallows_score" => fowlkes_mallows_score(&lt, &lp),
                "homogeneity_score" => homogeneity_score(&lt, &lp),
                "completeness_score" => completeness_score(&lt, &lp),
                _ => v_measure_score(&lt, &lp),
            }
        }
        _ => f64::NAN,
    };
    if value.is_nan() { format!("{{\"error\":\"unknown metric: {}\"}}", name) }
    else { format!("{{\"value\":{}}}", value) }
}

#[crate::sera_alias("metric_curve", "ml_curve", "pr_curve")]
pub fn ml_metric_curve(input: &str) -> String {
    use crate::ml::metrics::*;
    #[derive(Deserialize, Default)]
    struct I {
        name: Option<String>,
        y_true: Option<Vec<f64>>,
        y_score: Option<Vec<f64>>,
        pos_label: Option<i32>,
    }
    let i: I = serde_json::from_str(input).unwrap_or_default();
    let yt: Vec<i32> = i.y_true.unwrap_or_default().iter().map(|v| *v as i32).collect();
    let ys = i.y_score.unwrap_or_default();
    let pos = i.pos_label.unwrap_or(1);
    match i.name.as_deref().unwrap_or("") {
        "roc_curve" => {
            let (a, b, c) = roc_curve(&yt, &ys, pos);
            format!("{{\"fpr\":{},\"tpr\":{},\"thresholds\":{}}}",
                serde_json::to_string(&a).unwrap_or_default(),
                serde_json::to_string(&b).unwrap_or_default(),
                serde_json::to_string(&c).unwrap_or_default())
        }
        "precision_recall_curve" => {
            let (a, b, c) = precision_recall_curve(&yt, &ys, pos);
            format!("{{\"precision\":{},\"recall\":{},\"thresholds\":{}}}",
                serde_json::to_string(&a).unwrap_or_default(),
                serde_json::to_string(&b).unwrap_or_default(),
                serde_json::to_string(&c).unwrap_or_default())
        }
        n => format!("{{\"error\":\"unknown curve: {}\"}}", n),
    }
}
