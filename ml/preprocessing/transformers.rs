#[crate::model(category = "Preprocessing", domain = "ml")]
pub struct SimpleImputer {
    pub strategy: String,
    pub fill_value: f64,
    pub statistics: Vec<f64>,
}

impl SimpleImputer {
    pub fn new(strategy: &str, fill_value: f64) -> Self {
        Self { strategy: strategy.to_string(), fill_value, statistics: Vec::new() }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize) {
        self.statistics = vec![0.0; p];
        for j in 0..p {
            let col: Vec<f64> = (0..n).map(|i| x[i * p + j]).filter(|v| v.is_finite()).collect();
            self.statistics[j] = match self.strategy.as_str() {
                "mean" => if col.is_empty() { 0.0 } else { col.iter().sum::<f64>() / col.len() as f64 },
                "median" => {
                    let mut c = col.clone();
                    c.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                    if c.is_empty() { 0.0 } else if c.len() % 2 == 1 { c[c.len() / 2] } else { (c[c.len() / 2 - 1] + c[c.len() / 2]) * 0.5 }
                }
                "most_frequent" => {
                    if col.is_empty() { 0.0 } else {
                        let mut counts: std::collections::HashMap<u64, (f64, usize)> = std::collections::HashMap::new();
                        for &v in &col {
                            let k = v.to_bits();
                            counts.entry(k).and_modify(|e| e.1 += 1).or_insert((v, 1));
                        }
                        counts.into_iter().max_by_key(|&(_, (_, c))| c).map(|(_, (v, _))| v).unwrap_or(0.0)
                    }
                }
                _ => self.fill_value,
            };
        }
    }

    pub fn transform(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let mut out = x.to_vec();
        for i in 0..n {
            for j in 0..p {
                let v = out[i * p + j];
                if !v.is_finite() {
                    out[i * p + j] = self.statistics.get(j).copied().unwrap_or(self.fill_value);
                }
            }
        }
        out
    }

    pub fn fit_transform(&mut self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        self.fit(x, n, p);
        self.transform(x, n, p)
    }
}

#[crate::model(category = "Preprocessing", domain = "ml")]
pub struct PolynomialFeatures {
    pub degree: usize,
    pub interaction_only: bool,
    pub include_bias: bool,
    pub n_input_features: usize,
    pub powers: Vec<Vec<usize>>,
}

impl PolynomialFeatures {
    pub fn new(degree: usize, interaction_only: bool, include_bias: bool) -> Self {
        Self { degree, interaction_only, include_bias, n_input_features: 0, powers: Vec::new() }
    }

    fn enumerate(&mut self, p: usize) {
        self.n_input_features = p;
        self.powers.clear();
        if self.include_bias { self.powers.push(vec![0; p]); }
        for d in 1..=self.degree {
            self.combos(p, d, &mut vec![0usize; p], 0, 0);
        }
    }

    fn combos(&mut self, p: usize, remaining: usize, current: &mut Vec<usize>, start: usize, depth: usize) {
        if depth == remaining {
            let row: Vec<usize> = (0..p).map(|i| current.iter().filter(|&&v| v == i).count()).collect();
            if self.interaction_only && row.iter().any(|&c| c > 1) { return; }
            self.powers.push(row);
            return;
        }
        for j in start..p {
            current[depth] = j;
            self.combos(p, remaining, current, j, depth + 1);
        }
    }

    pub fn fit(&mut self, _x: &[f64], _n: usize, p: usize) {
        self.enumerate(p);
    }

    pub fn transform(&self, x: &[f64], n: usize, _p: usize) -> Vec<f64> {
        use rayon::prelude::*;
        let cols = self.powers.len();
        let mut out = vec![0.0f64; n * cols];
        let p_in = self.n_input_features;
        let powers = &self.powers;
        let active: Vec<Vec<(usize, usize)>> = powers.iter().map(|pw| {
            pw.iter().enumerate().filter_map(|(j, &e)| if e > 0 { Some((j, e)) } else { None }).collect()
        }).collect();
        let work = |i: usize, row: &mut [f64]| {
            let xi = &x[i * p_in..(i + 1) * p_in];
            for c in 0..cols {
                let mut v = 1.0f64;
                for &(j, e) in &active[c] {
                    let xj = xi[j];
                    match e {
                        1 => v *= xj,
                        2 => v *= xj * xj,
                        3 => { let s = xj * xj; v *= s * xj; }
                        _ => { let mut t = 1.0; for _ in 0..e { t *= xj; } v *= t; }
                    }
                }
                row[c] = v;
            }
        };
        if n >= 1024 {
            out.par_chunks_mut(cols).enumerate().for_each(|(i, row)| work(i, row));
        } else {
            for i in 0..n {
                let row = &mut out[i * cols..(i + 1) * cols];
                work(i, row);
            }
        }
        out
    }

    pub fn fit_transform(&mut self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        self.fit(x, n, p);
        self.transform(x, n, p)
    }

    pub fn n_output_features(&self) -> usize { self.powers.len() }
}

#[crate::model(category = "Preprocessing", domain = "ml")]
pub struct KBinsDiscretizer {
    pub n_bins: usize,
    pub strategy: String,
    pub bin_edges: Vec<Vec<f64>>,
}

impl KBinsDiscretizer {
    pub fn new(n_bins: usize, strategy: &str) -> Self {
        Self { n_bins, strategy: strategy.to_string(), bin_edges: Vec::new() }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize) {
        self.bin_edges.clear();
        for j in 0..p {
            let mut col: Vec<f64> = (0..n).map(|i| x[i * p + j]).filter(|v| v.is_finite()).collect();
            col.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            if col.is_empty() { self.bin_edges.push(vec![0.0; self.n_bins + 1]); continue; }
            let edges = match self.strategy.as_str() {
                "uniform" => {
                    let lo = col[0];
                    let hi = col[col.len() - 1];
                    let step = (hi - lo) / self.n_bins as f64;
                    (0..=self.n_bins).map(|k| lo + step * k as f64).collect()
                }
                "quantile" => {
                    (0..=self.n_bins).map(|k| {
                        let q = k as f64 / self.n_bins as f64;
                        let idx = ((col.len() - 1) as f64 * q).round() as usize;
                        col[idx]
                    }).collect()
                }
                _ => {
                    let lo = col[0];
                    let hi = col[col.len() - 1];
                    let step = (hi - lo) / self.n_bins as f64;
                    (0..=self.n_bins).map(|k| lo + step * k as f64).collect()
                }
            };
            self.bin_edges.push(edges);
        }
    }

    pub fn transform(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let mut out = vec![0.0f64; n * p];
        for i in 0..n {
            for j in 0..p {
                let v = x[i * p + j];
                let edges = &self.bin_edges[j];
                let mut bin = 0usize;
                for k in 1..edges.len() - 1 {
                    if v >= edges[k] { bin = k; }
                }
                out[i * p + j] = bin as f64;
            }
        }
        out
    }

    pub fn fit_transform(&mut self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        self.fit(x, n, p);
        self.transform(x, n, p)
    }
}

#[crate::model(category = "Preprocessing", domain = "ml")]
pub struct PowerTransformer {
    pub method: String,
    pub lambdas: Vec<f64>,
}

impl PowerTransformer {
    pub fn new(method: &str) -> Self {
        Self { method: method.to_string(), lambdas: Vec::new() }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize) {
        self.lambdas = vec![1.0; p];
        for j in 0..p {
            let col: Vec<f64> = (0..n).map(|i| x[i * p + j]).collect();
            self.lambdas[j] = optimize_lambda(&col, &self.method);
        }
    }

    pub fn transform(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let mut out = vec![0.0f64; n * p];
        for i in 0..n {
            for j in 0..p {
                out[i * p + j] = apply_power(x[i * p + j], self.lambdas[j], &self.method);
            }
        }
        out
    }

    pub fn fit_transform(&mut self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        self.fit(x, n, p);
        self.transform(x, n, p)
    }
}

fn apply_power(v: f64, lambda: f64, method: &str) -> f64 {
    match method {
        "yeo-johnson" => {
            if v >= 0.0 {
                if (lambda).abs() < 1e-10 { (v + 1.0).ln() } else { ((v + 1.0).powf(lambda) - 1.0) / lambda }
            } else {
                if ((lambda - 2.0)).abs() < 1e-10 { -(-v + 1.0).ln() } else { -((-v + 1.0).powf(2.0 - lambda) - 1.0) / (2.0 - lambda) }
            }
        }
        "box-cox" => {
            if v <= 0.0 { 0.0 } else if lambda.abs() < 1e-10 { v.ln() } else { (v.powf(lambda) - 1.0) / lambda }
        }
        _ => v,
    }
}

fn optimize_lambda(col: &[f64], method: &str) -> f64 {
    let candidates: Vec<f64> = (-20..=20).map(|i| i as f64 * 0.1).collect();
    let mut best = 1.0;
    let mut best_var = f64::INFINITY;
    for &lam in &candidates {
        let t: Vec<f64> = col.iter().map(|&v| apply_power(v, lam, method)).collect();
        let m = t.iter().sum::<f64>() / t.len() as f64;
        let v = t.iter().map(|&x| (x - m).powi(2)).sum::<f64>() / t.len() as f64;
        if v < best_var && v.is_finite() { best_var = v; best = lam; }
    }
    best
}

#[crate::model(category = "Preprocessing", domain = "ml")]
pub struct QuantileTransformer {
    pub n_quantiles: usize,
    pub output_distribution: String,
    pub quantiles: Vec<Vec<f64>>,
}

impl QuantileTransformer {
    pub fn new(n_quantiles: usize, output_distribution: &str) -> Self {
        Self { n_quantiles, output_distribution: output_distribution.to_string(), quantiles: Vec::new() }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize) {
        use rayon::prelude::*;
        self.quantiles = (0..p).into_par_iter().map(|j| {
            let mut col: Vec<f64> = (0..n).map(|i| x[i * p + j]).collect();
            col.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            (0..self.n_quantiles).map(|k| {
                let f = k as f64 / (self.n_quantiles - 1).max(1) as f64;
                let idx = ((col.len() - 1) as f64 * f).round() as usize;
                col[idx.min(col.len() - 1)]
            }).collect()
        }).collect();
    }

    pub fn transform(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        use rayon::prelude::*;
        let mut out = vec![0.0f64; n * p];
        let qs = &self.quantiles;
        let dist = self.output_distribution.as_str();
        if n * p >= 50_000 {
            out.par_chunks_mut(p).enumerate().for_each(|(i, row)| {
                for j in 0..p {
                    let v = x[i * p + j];
                    let q = &qs[j];
                    let idx = q.partition_point(|&qv| qv <= v);
                    let r = idx as f64 / q.len().max(1) as f64;
                    row[j] = if dist == "normal" { norm_ppf(r.max(1e-7).min(1.0 - 1e-7)) } else { r };
                }
            });
        } else {
            for i in 0..n {
                for j in 0..p {
                    let v = x[i * p + j];
                    let q = &qs[j];
                    let idx = q.partition_point(|&qv| qv <= v);
                    let r = idx as f64 / q.len().max(1) as f64;
                    out[i * p + j] = if dist == "normal" { norm_ppf(r.max(1e-7).min(1.0 - 1e-7)) } else { r };
                }
            }
        }
        out
    }

    pub fn fit_transform(&mut self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        self.fit(x, n, p);
        self.transform(x, n, p)
    }
}

fn norm_ppf(p: f64) -> f64 {
    let a = [-3.969683028665376e+01, 2.209460984245205e+02, -2.759285104469687e+02, 1.383577518672690e+02, -3.066479806614716e+01, 2.506628277459239e+00];
    let b = [-5.447609879822406e+01, 1.615858368580409e+02, -1.556989798598866e+02, 6.680131188771972e+01, -1.328068155288572e+01];
    let c = [-7.784894002430293e-03, -3.223964580411365e-01, -2.400758277161838e+00, -2.549732539343734e+00, 4.374664141464968e+00, 2.938163982698783e+00];
    let d = [7.784695709041462e-03, 3.224671290700398e-01, 2.445134137142996e+00, 3.754408661907416e+00];
    let plow = 0.02425;
    let phigh = 1.0 - plow;
    if p < plow {
        let q = (-2.0 * p.ln()).sqrt();
        (((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5]) / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0)
    } else if p <= phigh {
        let q = p - 0.5;
        let r = q * q;
        (((((a[0] * r + a[1]) * r + a[2]) * r + a[3]) * r + a[4]) * r + a[5]) * q / (((((b[0] * r + b[1]) * r + b[2]) * r + b[3]) * r + b[4]) * r + 1.0)
    } else {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        -(((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5]) / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0)
    }
}


