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
    let mut s = 0.0;
    for i in 0..n {
        if y_true[i].abs() > 1e-15 {
            s += ((y_true[i] - y_pred[i]) / y_true[i]).abs();
        }
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
