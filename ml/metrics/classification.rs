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
