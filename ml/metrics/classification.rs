pub fn accuracy_score(y_true: &[i32], y_pred: &[i32]) -> f64 {
    let n = y_true.len().min(y_pred.len());
    if n == 0 { return 0.0; }
    let correct = y_true.iter().zip(y_pred.iter()).filter(|(a, b)| a == b).count();
    correct as f64 / n as f64
}

pub fn precision_score(y_true: &[i32], y_pred: &[i32], average: Average) -> f64 {
    match average {
        Average::Binary(pos) => {
            let tp = y_true.iter().zip(y_pred.iter()).filter(|(&t, &p)| t == pos && p == pos).count();
            let fp = y_true.iter().zip(y_pred.iter()).filter(|(&t, &p)| t != pos && p == pos).count();
            if tp + fp == 0 { 0.0 } else { tp as f64 / (tp + fp) as f64 }
        }
        Average::Macro => {
            let classes = unique_classes(y_true, y_pred);
            let sum: f64 = classes.iter().map(|&c| precision_score(y_true, y_pred, Average::Binary(c))).sum();
            sum / classes.len().max(1) as f64
        }
        Average::Weighted => {
            let classes = unique_classes(y_true, y_pred);
            let n = y_true.len() as f64;
            classes.iter().map(|&c| {
                let support = y_true.iter().filter(|&&t| t == c).count() as f64;
                precision_score(y_true, y_pred, Average::Binary(c)) * support / n
            }).sum()
        }
    }
}

pub fn recall_score(y_true: &[i32], y_pred: &[i32], average: Average) -> f64 {
    match average {
        Average::Binary(pos) => {
            let tp = y_true.iter().zip(y_pred.iter()).filter(|(&t, &p)| t == pos && p == pos).count();
            let fn_ = y_true.iter().zip(y_pred.iter()).filter(|(&t, &p)| t == pos && p != pos).count();
            if tp + fn_ == 0 { 0.0 } else { tp as f64 / (tp + fn_) as f64 }
        }
        Average::Macro => {
            let classes = unique_classes(y_true, y_pred);
            let sum: f64 = classes.iter().map(|&c| recall_score(y_true, y_pred, Average::Binary(c))).sum();
            sum / classes.len().max(1) as f64
        }
        Average::Weighted => {
            let classes = unique_classes(y_true, y_pred);
            let n = y_true.len() as f64;
            classes.iter().map(|&c| {
                let support = y_true.iter().filter(|&&t| t == c).count() as f64;
                recall_score(y_true, y_pred, Average::Binary(c)) * support / n
            }).sum()
        }
    }
}

pub fn f1_score(y_true: &[i32], y_pred: &[i32], average: Average) -> f64 {
    match average {
        Average::Binary(pos) => {
            let p = precision_score(y_true, y_pred, Average::Binary(pos));
            let r = recall_score(y_true, y_pred, Average::Binary(pos));
            if p + r < 1e-15 { 0.0 } else { 2.0 * p * r / (p + r) }
        }
        Average::Macro => {
            let classes = unique_classes(y_true, y_pred);
            let sum: f64 = classes.iter().map(|&c| f1_score(y_true, y_pred, Average::Binary(c))).sum();
            sum / classes.len().max(1) as f64
        }
        Average::Weighted => {
            let classes = unique_classes(y_true, y_pred);
            let n = y_true.len() as f64;
            classes.iter().map(|&c| {
                let support = y_true.iter().filter(|&&t| t == c).count() as f64;
                f1_score(y_true, y_pred, Average::Binary(c)) * support / n
            }).sum()
        }
    }
}

pub fn confusion_matrix(y_true: &[i32], y_pred: &[i32]) -> (Vec<i32>, Vec<usize>) {
    let classes = unique_classes(y_true, y_pred);
    let k = classes.len();
    let mut mat = vec![0usize; k * k];
    for (&t, &p) in y_true.iter().zip(y_pred.iter()) {
        if let (Some(ti), Some(pi)) = (classes.iter().position(|&c| c == t), classes.iter().position(|&c| c == p)) {
            mat[ti * k + pi] += 1;
        }
    }
    (classes, mat)
}

pub fn classification_report(y_true: &[i32], y_pred: &[i32]) -> String {
    let classes = unique_classes(y_true, y_pred);
    let mut out = String::from("              precision    recall  f1-score   support\n\n");
    for &c in &classes {
        let p = precision_score(y_true, y_pred, Average::Binary(c));
        let r = recall_score(y_true, y_pred, Average::Binary(c));
        let f1 = f1_score(y_true, y_pred, Average::Binary(c));
        let support = y_true.iter().filter(|&&t| t == c).count();
        out += &format!("{:>12}    {:>6.4}    {:>6.4}    {:>6.4}    {:>5}\n", c, p, r, f1, support);
    }
    let acc = accuracy_score(y_true, y_pred);
    let n = y_true.len();
    out += &format!("\n    accuracy                        {:>6.4}    {:>5}\n", acc, n);
    out += &format!("   macro avg    {:>6.4}    {:>6.4}    {:>6.4}    {:>5}\n",
        precision_score(y_true, y_pred, Average::Macro),
        recall_score(y_true, y_pred, Average::Macro),
        f1_score(y_true, y_pred, Average::Macro), n);
    out += &format!("weighted avg    {:>6.4}    {:>6.4}    {:>6.4}    {:>5}\n",
        precision_score(y_true, y_pred, Average::Weighted),
        recall_score(y_true, y_pred, Average::Weighted),
        f1_score(y_true, y_pred, Average::Weighted), n);
    out
}

#[derive(Clone, Copy)]
pub enum Average {
    Binary(i32),
    Macro,
    Weighted,
}

fn unique_classes(y_true: &[i32], y_pred: &[i32]) -> Vec<i32> {
    let mut c: Vec<i32> = y_true.iter().chain(y_pred.iter()).copied().collect();
    c.sort_unstable();
    c.dedup();
    c
}

pub fn balanced_accuracy_score(y_true: &[i32], y_pred: &[i32]) -> f64 {
    let classes = unique_classes(y_true, y_pred);
    if classes.is_empty() { return 0.0; }
    let mut acc = 0.0;
    let mut k = 0usize;
    for &c in &classes {
        let total = y_true.iter().filter(|&&t| t == c).count();
        if total == 0 { continue; }
        let correct = y_true.iter().zip(y_pred.iter()).filter(|(&t, &p)| t == c && p == c).count();
        acc += correct as f64 / total as f64;
        k += 1;
    }
    if k == 0 { 0.0 } else { acc / k as f64 }
}

pub fn matthews_corrcoef(y_true: &[i32], y_pred: &[i32]) -> f64 {
    let classes = unique_classes(y_true, y_pred);
    let k = classes.len();
    let n = y_true.len().min(y_pred.len()) as f64;
    if n == 0.0 || k < 2 { return 0.0; }
    let mut t = vec![0.0f64; k];
    let mut p = vec![0.0f64; k];
    let mut c = 0.0f64;
    for i in 0..n as usize {
        let ti = classes.iter().position(|&v| v == y_true[i]);
        let pi = classes.iter().position(|&v| v == y_pred[i]);
        if let (Some(a), Some(b)) = (ti, pi) {
            t[a] += 1.0;
            p[b] += 1.0;
            if a == b { c += 1.0; }
        }
    }
    let s: f64 = t.iter().zip(p.iter()).map(|(a, b)| a * b).sum();
    let tt: f64 = t.iter().map(|v| v * v).sum();
    let pp: f64 = p.iter().map(|v| v * v).sum();
    let denom = ((n * n - tt) * (n * n - pp)).sqrt();
    if denom < 1e-15 { 0.0 } else { (c * n - s) / denom }
}

pub fn cohen_kappa_score(y_true: &[i32], y_pred: &[i32]) -> f64 {
    let (classes, mat) = confusion_matrix(y_true, y_pred);
    let k = classes.len();
    let n: f64 = mat.iter().sum::<usize>() as f64;
    if n == 0.0 || k == 0 { return 0.0; }
    let po: f64 = (0..k).map(|i| mat[i * k + i] as f64).sum::<f64>() / n;
    let mut row_sum = vec![0.0f64; k];
    let mut col_sum = vec![0.0f64; k];
    for i in 0..k {
        for j in 0..k {
            row_sum[i] += mat[i * k + j] as f64;
            col_sum[j] += mat[i * k + j] as f64;
        }
    }
    let pe: f64 = (0..k).map(|i| (row_sum[i] / n) * (col_sum[i] / n)).sum();
    if (1.0 - pe).abs() < 1e-15 { 0.0 } else { (po - pe) / (1.0 - pe) }
}

pub fn hamming_loss(y_true: &[i32], y_pred: &[i32]) -> f64 {
    let n = y_true.len().min(y_pred.len());
    if n == 0 { return 0.0; }
    let mis = y_true.iter().zip(y_pred.iter()).filter(|(a, b)| a != b).count();
    mis as f64 / n as f64
}

pub fn zero_one_loss(y_true: &[i32], y_pred: &[i32]) -> f64 {
    1.0 - accuracy_score(y_true, y_pred)
}

pub fn jaccard_score(y_true: &[i32], y_pred: &[i32], pos_label: i32) -> f64 {
    let tp = y_true.iter().zip(y_pred.iter()).filter(|(&t, &p)| t == pos_label && p == pos_label).count();
    let fp = y_true.iter().zip(y_pred.iter()).filter(|(&t, &p)| t != pos_label && p == pos_label).count();
    let fn_ = y_true.iter().zip(y_pred.iter()).filter(|(&t, &p)| t == pos_label && p != pos_label).count();
    let denom = tp + fp + fn_;
    if denom == 0 { 0.0 } else { tp as f64 / denom as f64 }
}

pub fn fbeta_score(y_true: &[i32], y_pred: &[i32], beta: f64, average: Average) -> f64 {
    match average {
        Average::Binary(pos) => {
            let p = precision_score(y_true, y_pred, Average::Binary(pos));
            let r = recall_score(y_true, y_pred, Average::Binary(pos));
            let b2 = beta * beta;
            let denom = b2 * p + r;
            if denom < 1e-15 { 0.0 } else { (1.0 + b2) * p * r / denom }
        }
        Average::Macro => {
            let classes = unique_classes(y_true, y_pred);
            let s: f64 = classes.iter().map(|&c| fbeta_score(y_true, y_pred, beta, Average::Binary(c))).sum();
            s / classes.len().max(1) as f64
        }
        Average::Weighted => {
            let classes = unique_classes(y_true, y_pred);
            let n = y_true.len() as f64;
            classes.iter().map(|&c| {
                let support = y_true.iter().filter(|&&t| t == c).count() as f64;
                fbeta_score(y_true, y_pred, beta, Average::Binary(c)) * support / n
            }).sum()
        }
    }
}

pub fn log_loss(y_true: &[i32], y_proba: &[f64], n: usize, k: usize, eps: f64) -> f64 {
    if n == 0 || k == 0 { return 0.0; }
    let mut s = 0.0;
    for i in 0..n {
        let cls = y_true[i] as usize;
        if cls >= k { continue; }
        let p = y_proba[i * k + cls].max(eps).min(1.0 - eps);
        s -= p.ln();
    }
    s / n as f64
}

pub fn binary_log_loss(y_true: &[i32], y_score: &[f64], eps: f64) -> f64 {
    let n = y_true.len().min(y_score.len());
    if n == 0 { return 0.0; }
    let mut s = 0.0;
    for i in 0..n {
        let p = y_score[i].max(eps).min(1.0 - eps);
        let y = y_true[i] as f64;
        s -= y * p.ln() + (1.0 - y) * (1.0 - p).ln();
    }
    s / n as f64
}

pub fn brier_score_loss(y_true: &[i32], y_score: &[f64]) -> f64 {
    let n = y_true.len().min(y_score.len());
    if n == 0 { return 0.0; }
    let mut s = 0.0;
    for i in 0..n {
        let d = y_score[i] - y_true[i] as f64;
        s += d * d;
    }
    s / n as f64
}

pub fn hinge_loss(y_true: &[i32], pred_decision: &[f64]) -> f64 {
    let n = y_true.len().min(pred_decision.len());
    if n == 0 { return 0.0; }
    let mut s = 0.0;
    for i in 0..n {
        let y = if y_true[i] > 0 { 1.0 } else { -1.0 };
        let m = (1.0 - y * pred_decision[i]).max(0.0);
        s += m;
    }
    s / n as f64
}

pub fn roc_curve(y_true: &[i32], y_score: &[f64], pos_label: i32) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let n = y_true.len().min(y_score.len());
    if n == 0 { return (vec![], vec![], vec![]); }
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_unstable_by(|&a, &b| y_score[b].partial_cmp(&y_score[a]).unwrap_or(std::cmp::Ordering::Equal));
    let p_total = y_true.iter().filter(|&&t| t == pos_label).count() as f64;
    let n_total = (n as f64) - p_total;
    if p_total == 0.0 || n_total == 0.0 { return (vec![0.0, 1.0], vec![0.0, 1.0], vec![]); }
    let mut tpr = Vec::with_capacity(n + 1);
    let mut fpr = Vec::with_capacity(n + 1);
    let mut th = Vec::with_capacity(n + 1);
    let mut tp = 0.0;
    let mut fp = 0.0;
    let mut prev = f64::INFINITY;
    fpr.push(0.0);
    tpr.push(0.0);
    for &i in &idx {
        let s = y_score[i];
        if s != prev {
            fpr.push(fp / n_total);
            tpr.push(tp / p_total);
            th.push(s);
            prev = s;
        }
        if y_true[i] == pos_label { tp += 1.0; } else { fp += 1.0; }
    }
    fpr.push(1.0);
    tpr.push(1.0);
    (fpr, tpr, th)
}

pub fn roc_auc_score(y_true: &[i32], y_score: &[f64], pos_label: i32) -> f64 {
    let (fpr, tpr, _) = roc_curve(y_true, y_score, pos_label);
    let mut a = 0.0;
    for i in 1..fpr.len() {
        a += (fpr[i] - fpr[i - 1]) * (tpr[i] + tpr[i - 1]) * 0.5;
    }
    a
}

pub fn precision_recall_curve(y_true: &[i32], y_score: &[f64], pos_label: i32) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let n = y_true.len().min(y_score.len());
    if n == 0 { return (vec![], vec![], vec![]); }
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_unstable_by(|&a, &b| y_score[b].partial_cmp(&y_score[a]).unwrap_or(std::cmp::Ordering::Equal));
    let p_total = y_true.iter().filter(|&&t| t == pos_label).count() as f64;
    if p_total == 0.0 { return (vec![1.0], vec![0.0], vec![]); }
    let mut prec = Vec::with_capacity(n + 1);
    let mut rec = Vec::with_capacity(n + 1);
    let mut th = Vec::with_capacity(n);
    let mut tp = 0.0;
    let mut fp = 0.0;
    for &i in &idx {
        if y_true[i] == pos_label { tp += 1.0; } else { fp += 1.0; }
        let p = if tp + fp == 0.0 { 1.0 } else { tp / (tp + fp) };
        let r = tp / p_total;
        prec.push(p);
        rec.push(r);
        th.push(y_score[i]);
    }
    prec.push(1.0);
    rec.push(0.0);
    (prec, rec, th)
}

pub fn average_precision_score(y_true: &[i32], y_score: &[f64], pos_label: i32) -> f64 {
    let (prec, rec, _) = precision_recall_curve(y_true, y_score, pos_label);
    let mut a = 0.0;
    for i in 1..rec.len() {
        a += (rec[i - 1] - rec[i]) * prec[i - 1];
    }
    a
}
