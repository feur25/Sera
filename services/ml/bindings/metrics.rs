use crate::ml::metrics::*;
use serde::Deserialize;
use std::collections::HashMap;

struct MetricArgs {
    yt_i: Vec<i32>,
    yp_i: Vec<i32>,
    yt_f: Vec<f64>,
    yp_f: Vec<f64>,
    ys_f: Vec<f64>,
    avg: Average,
    pos: i32,
    beta: f64,
    alpha: f64,
    eps: f64,
    flat: Vec<f64>,
    labs: Vec<i32>,
    n: usize,
    p: usize,
    lt: Vec<i32>,
    lp: Vec<i32>,
}

lazy_static::lazy_static! {
    static ref METRIC_TABLE: HashMap<&'static str, fn(&MetricArgs) -> f64> = {
        let mut m: HashMap<&'static str, fn(&MetricArgs) -> f64> = HashMap::new();
        m.insert("accuracy_score", |a| accuracy_score(&a.yt_i, &a.yp_i));
        m.insert("balanced_accuracy_score", |a| balanced_accuracy_score(&a.yt_i, &a.yp_i));
        m.insert("precision_score", |a| precision_score(&a.yt_i, &a.yp_i, a.avg));
        m.insert("recall_score", |a| recall_score(&a.yt_i, &a.yp_i, a.avg));
        m.insert("f1_score", |a| f1_score(&a.yt_i, &a.yp_i, a.avg));
        m.insert("fbeta_score", |a| fbeta_score(&a.yt_i, &a.yp_i, a.beta, a.avg));
        m.insert("jaccard_score", |a| jaccard_score(&a.yt_i, &a.yp_i, a.pos));
        m.insert("matthews_corrcoef", |a| matthews_corrcoef(&a.yt_i, &a.yp_i));
        m.insert("cohen_kappa_score", |a| cohen_kappa_score(&a.yt_i, &a.yp_i));
        m.insert("hamming_loss", |a| hamming_loss(&a.yt_i, &a.yp_i));
        m.insert("zero_one_loss", |a| zero_one_loss(&a.yt_i, &a.yp_i));
        m.insert("binary_log_loss", |a| binary_log_loss(&a.yt_i, &a.yp_f, a.eps));
        m.insert("brier_score_loss", |a| brier_score_loss(&a.yt_i, &a.yp_f));
        m.insert("hinge_loss", |a| hinge_loss(&a.yt_i, &a.yp_f));
        m.insert("roc_auc_score", |a| roc_auc_score(&a.yt_i, &a.ys_f, a.pos));
        m.insert("average_precision_score", |a| average_precision_score(&a.yt_i, &a.ys_f, a.pos));
        m.insert("mean_squared_error", |a| mean_squared_error(&a.yt_f, &a.yp_f));
        m.insert("root_mean_squared_error", |a| root_mean_squared_error(&a.yt_f, &a.yp_f));
        m.insert("mean_absolute_error", |a| mean_absolute_error(&a.yt_f, &a.yp_f));
        m.insert("median_absolute_error", |a| median_absolute_error(&a.yt_f, &a.yp_f));
        m.insert("r2_score", |a| r2_score(&a.yt_f, &a.yp_f));
        m.insert("explained_variance_score", |a| explained_variance_score(&a.yt_f, &a.yp_f));
        m.insert("max_error", |a| max_error(&a.yt_f, &a.yp_f));
        m.insert("mean_absolute_percentage_error", |a| mean_absolute_percentage_error(&a.yt_f, &a.yp_f));
        m.insert("mean_squared_log_error", |a| mean_squared_log_error(&a.yt_f, &a.yp_f));
        m.insert("root_mean_squared_log_error", |a| root_mean_squared_log_error(&a.yt_f, &a.yp_f));
        m.insert("mean_pinball_loss", |a| mean_pinball_loss(&a.yt_f, &a.yp_f, a.alpha));
        m.insert("d2_absolute_error_score", |a| d2_absolute_error_score(&a.yt_f, &a.yp_f));
        m.insert("silhouette_score", |a| silhouette_score(&a.flat, &a.labs, a.n, a.p));
        m.insert("davies_bouldin_score", |a| davies_bouldin_score(&a.flat, &a.labs, a.n, a.p));
        m.insert("calinski_harabasz_score", |a| calinski_harabasz_score(&a.flat, &a.labs, a.n, a.p));
        m.insert("adjusted_rand_score", |a| adjusted_rand_score(&a.lt, &a.lp));
        m.insert("normalized_mutual_info_score", |a| normalized_mutual_info_score(&a.lt, &a.lp));
        m.insert("fowlkes_mallows_score", |a| fowlkes_mallows_score(&a.lt, &a.lp));
        m.insert("homogeneity_score", |a| homogeneity_score(&a.lt, &a.lp));
        m.insert("completeness_score", |a| completeness_score(&a.lt, &a.lp));
        m.insert("v_measure_score", |a| v_measure_score(&a.lt, &a.lp));
        m
    };
}

#[crate::sera_doc(
    category = "Metrics",
    en = "Compute a named metric score (accuracy, r2, f1, etc.) from predictions.",
    fr = "Calcule un score de métrique nommée (accuracy, r2, f1, etc.) à partir des prédictions.",
    file = "metrics.md"
)]
#[crate::sera_alias("metric_score", "ml_metric", "score_metric")]
#[crate::sera_builder]
pub fn ml_metric_score(input: &str) -> String {
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
    let sanitized = crate::plot::chart_input::sanitize_non_finite_json(input);
    let i: I = serde_json::from_str(&sanitized).unwrap_or_default();
    let name: String = i.name.unwrap_or_default();
    let to_i32 = |v: &[f64]| v.iter().map(|x| *x as i32).collect::<Vec<i32>>();
    let yt_f = i.y_true.unwrap_or_default();
    let yp_f = i.y_pred.unwrap_or_default();
    let ys_f = i.y_score.unwrap_or_default();
    let yt_i = to_i32(&yt_f);
    let yp_i = to_i32(&yp_f);
    let pos = i.pos_label.unwrap_or(1);
    let avg = match i.average.as_deref().unwrap_or("binary") {
        "macro" => Average::Macro,
        "weighted" => Average::Weighted,
        _ => Average::Binary(pos),
    };
    let rows = i.x.unwrap_or_default();
    let n = rows.len();
    let p = if n > 0 { rows[0].len() } else { 0 };
    let mut flat = Vec::with_capacity(n * p);
    for r in &rows {
        flat.extend_from_slice(&r[..p.min(r.len())]);
        if r.len() < p {
            flat.extend(std::iter::repeat(0.0).take(p - r.len()));
        }
    }
    let args = MetricArgs {
        yt_i,
        yp_i,
        yt_f,
        yp_f,
        ys_f,
        avg,
        pos,
        beta: i.beta.unwrap_or(1.0),
        alpha: i.alpha.unwrap_or(0.5),
        eps: i.eps.unwrap_or(1e-15),
        flat,
        labs: i.labels.unwrap_or_default(),
        n,
        p,
        lt: i.labels_true.unwrap_or_default(),
        lp: i.labels_pred.unwrap_or_default(),
    };
    match METRIC_TABLE.get(name.as_str()) {
        Some(f) => format!("{{\"value\":{}}}", f(&args)),
        None => format!("{{\"error\":\"unknown metric: {}\"}}", name),
    }
}

#[crate::sera_doc(
    category = "Metrics",
    en = "metric_curve — computes ROC or precision-recall curve arrays.",
    fr = "metric_curve — calcule les tableaux de courbe ROC ou precision-rappel.",
    file = "metrics.md"
)]
#[crate::sera_alias("metric_curve", "ml_curve", "pr_curve")]
#[crate::sera_builder]
pub fn ml_metric_curve(input: &str) -> String {
    use crate::ml::metrics::*;
    #[derive(Deserialize, Default)]
    struct I {
        name: Option<String>,
        y_true: Option<Vec<f64>>,
        y_score: Option<Vec<f64>>,
        pos_label: Option<i32>,
    }
    let sanitized = crate::plot::chart_input::sanitize_non_finite_json(input);
    let i: I = serde_json::from_str(&sanitized).unwrap_or_default();
    let yt: Vec<i32> = i
        .y_true
        .unwrap_or_default()
        .iter()
        .map(|v| *v as i32)
        .collect();
    let ys = i.y_score.unwrap_or_default();
    let pos = i.pos_label.unwrap_or(1);
    match i.name.as_deref().unwrap_or("") {
        "roc_curve" => {
            let (a, b, c) = roc_curve(&yt, &ys, pos);
            format!(
                "{{\"fpr\":{},\"tpr\":{},\"thresholds\":{}}}",
                serde_json::to_string(&a).unwrap_or_default(),
                serde_json::to_string(&b).unwrap_or_default(),
                serde_json::to_string(&c).unwrap_or_default()
            )
        }
        "precision_recall_curve" => {
            let (a, b, c) = precision_recall_curve(&yt, &ys, pos);
            format!(
                "{{\"precision\":{},\"recall\":{},\"thresholds\":{}}}",
                serde_json::to_string(&a).unwrap_or_default(),
                serde_json::to_string(&b).unwrap_or_default(),
                serde_json::to_string(&c).unwrap_or_default()
            )
        }
        n => format!("{{\"error\":\"unknown curve: {}\"}}", n),
    }
}
