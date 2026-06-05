use super::helpers::*;
use serde::Deserialize;

#[crate::sera_alias("fit_transform", "ml_transform", "preprocess_fit_transform")]
pub fn ml_fit_transform(input: &str) -> String {
    use crate::ml::preprocessing::scalers::*;
    use crate::ml::preprocessing::transformers::*;
    #[derive(Deserialize, Default)]
    struct I {
        name: Option<String>,
        data: Option<Vec<Vec<f64>>>,
        strategy: Option<String>,
        fill_value: Option<f64>,
        degree: Option<usize>,
        interaction_only: Option<bool>,
        include_bias: Option<bool>,
        n_bins: Option<usize>,
        method: Option<String>,
        n_quantiles: Option<usize>,
        output_distribution: Option<String>,
    }
    let i: I = serde_json::from_str(input).unwrap_or_default();
    let rows = i.data.unwrap_or_default();
    let n = rows.len();
    let p = if n > 0 { rows[0].len() } else { 0 };
    let mut flat = Vec::with_capacity(n * p);
    for r in &rows {
        flat.extend_from_slice(&r[..p.min(r.len())]);
        if r.len() < p {
            flat.extend(std::iter::repeat(0.0).take(p - r.len()));
        }
    }
    let (out, cols, extra): (Vec<f64>, usize, String) = match i.name.as_deref().unwrap_or("") {
        "SimpleImputer" => {
            let mut t = SimpleImputer::new(
                i.strategy.as_deref().unwrap_or("mean"),
                i.fill_value.unwrap_or(0.0),
            );
            t.fit(&flat, n, p);
            let o = t.transform(&flat, n, p);
            (
                o,
                p,
                format!(
                    ",\"statistics\":{}",
                    serde_json::to_string(&t.statistics).unwrap_or_default()
                ),
            )
        }
        "PolynomialFeatures" => {
            let mut t = PolynomialFeatures::new(
                i.degree.unwrap_or(2),
                i.interaction_only.unwrap_or(false),
                i.include_bias.unwrap_or(true),
            );
            t.fit(&flat, n, p);
            let o = t.transform(&flat, n, p);
            let nf = t.n_output_features();
            (o, nf, format!(",\"n_features_out\":{}", nf))
        }
        "KBinsDiscretizer" => {
            let mut t = KBinsDiscretizer::new(
                i.n_bins.unwrap_or(5),
                i.strategy.as_deref().unwrap_or("quantile"),
            );
            t.fit(&flat, n, p);
            let o = t.transform(&flat, n, p);
            (o, p, String::new())
        }
        "PowerTransformer" => {
            let mut t = PowerTransformer::new(i.method.as_deref().unwrap_or("yeo-johnson"));
            t.fit(&flat, n, p);
            let o = t.transform(&flat, n, p);
            (
                o,
                p,
                format!(
                    ",\"lambdas\":{}",
                    serde_json::to_string(&t.lambdas).unwrap_or_default()
                ),
            )
        }
        "QuantileTransformer" => {
            let mut t = QuantileTransformer::new(
                i.n_quantiles.unwrap_or(1000),
                i.output_distribution.as_deref().unwrap_or("uniform"),
            );
            t.fit(&flat, n, p);
            let o = t.transform(&flat, n, p);
            (o, p, String::new())
        }
        "StandardScaler" => {
            let mut t = StandardScaler::new(true, true);
            t.fit(&flat, n, p);
            let o = t.transform(&flat, n, p);
            (o, p, String::new())
        }
        "MinMaxScaler" => {
            let mut t = MinMaxScaler::new((0.0, 1.0));
            t.fit(&flat, n, p);
            let o = t.transform(&flat, n, p);
            (o, p, String::new())
        }
        "RobustScaler" => {
            let mut t = RobustScaler::new(true, true);
            t.fit(&flat, n, p);
            let o = t.transform(&flat, n, p);
            (o, p, String::new())
        }
        "MaxAbsScaler" => {
            let mut t = MaxAbsScaler::new();
            t.fit(&flat, n, p);
            let o = t.transform(&flat, n, p);
            (o, p, String::new())
        }
        n_ => return format!("{{\"error\":\"unknown transformer: {}}}", n_),
    };
    let data_str = flat_to_json_str(&out, n, cols);
    format!(
        "{{\"data\":{},\"n\":{},\"cols\":{}{}}}",
        data_str, n, cols, extra
    )
}

#[crate::sera_doc(
    category = "Preprocessing",
    en = "StandardScaler — zero-mean unit-variance standardisation.",
    fr = "StandardScaler — standardisation à moyenne nulle et variance unitaire.",
    file = "preprocessing.md"
)]
#[crate::sera_alias("standard_scaler", "ml_standard_scaler", "StandardScaler")]
pub fn ml_standard_scaler(input: &str) -> String {
    use crate::ml::preprocessing::scalers::StandardScaler;
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let with_mean = jb(&v, "with_mean", true);
    let with_std = jb(&v, "with_std", true);
    let mut s = StandardScaler::new(with_mean, with_std);
    s.fit(&xf, n, p);
    let (px, pn) = if nt > 0 {
        (xtf.as_slice(), nt)
    } else {
        (xf.as_slice(), n)
    };
    let out = s.transform(px, pn, p);
    let data_str = flat_to_json_str(&out, pn, p);
    format!(
        "{{\"data\":{},\"mean\":{},\"scale\":{}}}",
        data_str,
        serde_json::to_string(&s.mean).unwrap_or_default(),
        serde_json::to_string(&s.scale).unwrap_or_default()
    )
}

#[crate::sera_alias("minmax_scaler", "ml_minmax_scaler", "MinMaxScaler")]
pub fn ml_minmax_scaler(input: &str) -> String {
    use crate::ml::preprocessing::scalers::MinMaxScaler;
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let fmin = jf(&v, "feature_range_min", 0.0);
    let fmax = jf(&v, "feature_range_max", 1.0);
    let mut s = MinMaxScaler::new((fmin, fmax));
    s.fit(&xf, n, p);
    let (px, pn) = if nt > 0 {
        (xtf.as_slice(), nt)
    } else {
        (xf.as_slice(), n)
    };
    let out = s.transform(px, pn, p);
    let data_str = flat_to_json_str(&out, pn, p);
    format!("{{\"data\":{}}}", data_str)
}

#[crate::sera_alias("robust_scaler", "ml_robust_scaler", "RobustScaler")]
pub fn ml_robust_scaler(input: &str) -> String {
    use crate::ml::preprocessing::scalers::RobustScaler;
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let with_centering = jb(&v, "with_centering", true);
    let with_scaling = jb(&v, "with_scaling", true);
    let mut s = RobustScaler::new(with_centering, with_scaling);
    s.fit(&xf, n, p);
    let (px, pn) = if nt > 0 {
        (xtf.as_slice(), nt)
    } else {
        (xf.as_slice(), n)
    };
    let out = s.transform(px, pn, p);
    let data_str = flat_to_json_str(&out, pn, p);
    format!("{{\"data\":{}}}", data_str)
}
