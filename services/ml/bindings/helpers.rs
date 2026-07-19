pub fn parse_x2d(v: &serde_json::Value, key: &str, alias: &str) -> Vec<Vec<f64>> {
    let raw = v.get(key).or_else(|| v.get(alias));
    if let Some(x) = raw {
        if let Ok(mat) = serde_json::from_value::<Vec<Vec<f64>>>(x.clone()) {
            return mat;
        }
        if let Ok(flat) = serde_json::from_value::<Vec<f64>>(x.clone()) {
            return flat.into_iter().map(|v| vec![v]).collect();
        }
    }
    vec![]
}

pub fn ml_parse(input: &str) -> (serde_json::Value, Vec<f64>, usize, usize, Vec<f64>, usize) {
    let v: serde_json::Value = serde_json::from_str(input).unwrap_or(serde_json::Value::Null);
    let xt = parse_x2d(&v, "x_train", "x");
    let xtest = parse_x2d(&v, "x_test", "test_x");
    let n = xt.len();
    let p = xt.first().map_or(0, |r| r.len());
    let xf: Vec<f64> = xt.into_iter().flatten().collect();
    let nt = xtest.len();
    let xtf: Vec<f64> = xtest.into_iter().flatten().collect();
    (v, xf, n, p, xtf, nt)
}

pub fn jf(v: &serde_json::Value, k: &str, d: f64) -> f64 {
    v.get(k).and_then(|x| x.as_f64()).unwrap_or(d)
}
pub fn ju(v: &serde_json::Value, k: &str, d: usize) -> usize {
    v.get(k)
        .and_then(|x| x.as_u64())
        .map(|x| x as usize)
        .unwrap_or(d)
}
pub fn jb(v: &serde_json::Value, k: &str, d: bool) -> bool {
    v.get(k).and_then(|x| x.as_bool()).unwrap_or(d)
}
pub fn js<'a>(v: &'a serde_json::Value, k: &str, d: &'a str) -> &'a str {
    v.get(k).and_then(|x| x.as_str()).unwrap_or(d)
}
pub fn yf(v: &serde_json::Value) -> Vec<f64> {
    match v.get("y_train").or_else(|| v.get("y")) {
        Some(x) => serde_json::from_value(x.clone()).unwrap_or_else(|e| {
            eprintln!("seraplot ml: y_train/y present but failed to parse as numeric labels: {e}");
            Vec::new()
        }),
        None => Vec::new(),
    }
}
pub fn yi(v: &serde_json::Value) -> Vec<i32> {
    match v.get("y_train").or_else(|| v.get("y")) {
        Some(x) => serde_json::from_value(x.clone()).unwrap_or_else(|e| {
            eprintln!("seraplot ml: y_train/y present but failed to parse as integer labels: {e}");
            Vec::new()
        }),
        None => Vec::new(),
    }
}

pub fn parse_max_features(v: &serde_json::Value) -> crate::ml::tree::random_forest::MaxFeatures {
    use crate::ml::tree::random_forest::MaxFeatures as MF;
    match v.get("max_features") {
        Some(serde_json::Value::String(s)) => match s.as_str() {
            "log2" => MF::Log2,
            "all" => MF::All,
            "none" => MF::All,
            _ => MF::Sqrt,
        },
        Some(serde_json::Value::Number(n)) => MF::Fixed(n.as_u64().unwrap_or(0) as usize),
        _ => MF::Sqrt,
    }
}

pub fn parse_knn_weights(s: &str) -> crate::ml::neighbors::knn::KnnWeights {
    use crate::ml::neighbors::knn::KnnWeights as W;
    match s {
        "distance" => W::Distance,
        _ => W::Uniform,
    }
}

pub fn parse_tree_criterion(s: &str) -> crate::ml::tree::decision_tree::TreeCriterion {
    use crate::ml::tree::decision_tree::TreeCriterion as C;
    match s {
        "entropy" => C::Entropy,
        "mse" => C::MSE,
        _ => C::Gini,
    }
}

pub fn pred_x<'a>(xf: &'a [f64], xtf: &'a [f64], nt: usize) -> (&'a [f64], usize) {
    if nt > 0 {
        (xtf, nt)
    } else {
        (xf, 0)
    }
}

pub fn pred_n(n: usize, nt: usize) -> usize {
    if nt > 0 {
        nt
    } else {
        n
    }
}

pub fn flat_to_json_str(data: &[f64], n: usize, cols: usize) -> String {
    let mut rows: Vec<Vec<f64>> = Vec::with_capacity(n);
    for r in 0..n {
        rows.push(data[r * cols..(r + 1) * cols].to_vec());
    }
    serde_json::to_string(&rows).unwrap_or_default()
}
