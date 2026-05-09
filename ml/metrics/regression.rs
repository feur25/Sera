pub fn mean_squared_error(y_true: &[f64], y_pred: &[f64]) -> f64 {
    let n = y_true.len().min(y_pred.len());
    if n == 0 { return 0.0; }
    let mut s = 0.0;
    for i in 0..n { let d = y_true[i] - y_pred[i]; s += d * d; }
    s / n as f64
}

pub fn mean_absolute_error(y_true: &[f64], y_pred: &[f64]) -> f64 {
    let n = y_true.len().min(y_pred.len());
    if n == 0 { return 0.0; }
    let mut s = 0.0;
    for i in 0..n { s += (y_true[i] - y_pred[i]).abs(); }
    s / n as f64
}

pub fn r2_score(y_true: &[f64], y_pred: &[f64]) -> f64 {
    let n = y_true.len().min(y_pred.len());
    if n == 0 { return 0.0; }
    let mean = y_true.iter().sum::<f64>() / n as f64;
    let ss_res: f64 = (0..n).map(|i| { let d = y_true[i] - y_pred[i]; d * d }).sum();
    let ss_tot: f64 = (0..n).map(|i| { let d = y_true[i] - mean; d * d }).sum();
    if ss_tot < 1e-15 { return 0.0; }
    1.0 - ss_res / ss_tot
}

pub fn root_mean_squared_error(y_true: &[f64], y_pred: &[f64]) -> f64 {
    mean_squared_error(y_true, y_pred).sqrt()
}

pub fn mean_absolute_percentage_error(y_true: &[f64], y_pred: &[f64]) -> f64 {
    let n = y_true.len().min(y_pred.len());
    if n == 0 { return 0.0; }
    let eps = f64::EPSILON;
    let mut s = 0.0;
    for i in 0..n {
        let denom = y_true[i].abs().max(eps);
        s += ((y_true[i] - y_pred[i]) / denom).abs();
    }
    s / n as f64
}

pub fn explained_variance_score(y_true: &[f64], y_pred: &[f64]) -> f64 {
    let n = y_true.len().min(y_pred.len());
    if n == 0 { return 0.0; }
    let residuals: Vec<f64> = (0..n).map(|i| y_true[i] - y_pred[i]).collect();
    let mean_res = residuals.iter().sum::<f64>() / n as f64;
    let var_res: f64 = residuals.iter().map(|&r| (r - mean_res).powi(2)).sum::<f64>() / n as f64;
    let mean_y = y_true.iter().sum::<f64>() / n as f64;
    let var_y: f64 = y_true.iter().map(|&y| (y - mean_y).powi(2)).sum::<f64>() / n as f64;
    if var_y < 1e-15 { return 0.0; }
    1.0 - var_res / var_y
}

pub fn max_error(y_true: &[f64], y_pred: &[f64]) -> f64 {
    y_true.iter().zip(y_pred.iter()).map(|(t, p)| (t - p).abs()).fold(0.0f64, f64::max)
}

pub fn median_absolute_error(y_true: &[f64], y_pred: &[f64]) -> f64 {
    let n = y_true.len().min(y_pred.len());
    if n == 0 { return 0.0; }
    let mut a: Vec<f64> = (0..n).map(|i| (y_true[i] - y_pred[i]).abs()).collect();
    a.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    if n % 2 == 1 { a[n / 2] } else { (a[n / 2 - 1] + a[n / 2]) * 0.5 }
}

pub fn mean_squared_log_error(y_true: &[f64], y_pred: &[f64]) -> f64 {
    let n = y_true.len().min(y_pred.len());
    if n == 0 { return 0.0; }
    let mut s = 0.0;
    for i in 0..n {
        if y_true[i] < 0.0 || y_pred[i] < 0.0 { continue; }
        let d = (y_pred[i] + 1.0).ln() - (y_true[i] + 1.0).ln();
        s += d * d;
    }
    s / n as f64
}

pub fn root_mean_squared_log_error(y_true: &[f64], y_pred: &[f64]) -> f64 {
    mean_squared_log_error(y_true, y_pred).sqrt()
}

pub fn mean_pinball_loss(y_true: &[f64], y_pred: &[f64], alpha: f64) -> f64 {
    let n = y_true.len().min(y_pred.len());
    if n == 0 { return 0.0; }
    let mut s = 0.0;
    for i in 0..n {
        let d = y_true[i] - y_pred[i];
        s += if d >= 0.0 { alpha * d } else { (alpha - 1.0) * d };
    }
    s / n as f64
}

pub fn d2_absolute_error_score(y_true: &[f64], y_pred: &[f64]) -> f64 {
    let n = y_true.len();
    if n == 0 { return 0.0; }
    let mut sorted: Vec<f64> = y_true.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let median = if n % 2 == 1 { sorted[n / 2] } else { (sorted[n / 2 - 1] + sorted[n / 2]) * 0.5 };
    let num = mean_absolute_error(y_true, y_pred);
    let denom: f64 = y_true.iter().map(|y| (y - median).abs()).sum::<f64>() / n as f64;
    if denom < 1e-15 { 0.0 } else { 1.0 - num / denom }
}


