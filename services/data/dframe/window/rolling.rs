use super::super::series::Series;
use super::super::SeraDFrame_;
use crate::sera_doc_impl;
use pyo3::prelude::*;
use rayon::prelude::*;
use std::sync::Arc;

impl SeraDFrame_ {
    fn rolling_apply(&self, col: &str, window: usize, suffix: &str, f: impl Fn(&[f64]) -> f64 + Sync) -> PyResult<SeraDFrame_> {
        let vals = self.inner.get(col)?.to_f64_vec();
        let n = vals.len();
        let window = window.max(1);
        let out: Vec<f64> = (0..n)
            .into_par_iter()
            .map(|i| {
                if i + 1 < window {
                    f64::NAN
                } else {
                    f(&vals[i + 1 - window..=i])
                }
            })
            .collect();
        self.push_derived(col, suffix, Series::Num(Arc::new(out)))
    }
}

#[sera_doc_impl]
#[pymethods]
impl SeraDFrame_ {
    #[sera_doc(
        name = "SeraDFrame.rolling_mean",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Adds {col}_rollN_mean, a trailing rolling mean of window N.",
        fr = "Ajoute {col}_rollN_mean, une moyenne glissante de fenetre N."
    )]
    fn rolling_mean(&self, col: &str, window: usize) -> PyResult<SeraDFrame_> {
        self.rolling_apply(col, window, &format!("roll{}_mean", window), |w| w.iter().sum::<f64>() / w.len() as f64)
    }

    #[sera_doc(
        name = "SeraDFrame.rolling_sum",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Adds {col}_rollN_sum, a trailing rolling sum of window N.",
        fr = "Ajoute {col}_rollN_sum, une somme glissante de fenetre N."
    )]
    fn rolling_sum(&self, col: &str, window: usize) -> PyResult<SeraDFrame_> {
        self.rolling_apply(col, window, &format!("roll{}_sum", window), |w| w.iter().sum())
    }

    #[sera_doc(
        name = "SeraDFrame.rolling_min",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Adds {col}_rollN_min, a trailing rolling minimum of window N.",
        fr = "Ajoute {col}_rollN_min, un minimum glissant de fenetre N."
    )]
    fn rolling_min(&self, col: &str, window: usize) -> PyResult<SeraDFrame_> {
        self.rolling_apply(col, window, &format!("roll{}_min", window), |w| {
            w.iter().cloned().fold(f64::INFINITY, f64::min)
        })
    }

    #[sera_doc(
        name = "SeraDFrame.rolling_max",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Adds {col}_rollN_max, a trailing rolling maximum of window N.",
        fr = "Ajoute {col}_rollN_max, un maximum glissant de fenetre N."
    )]
    fn rolling_max(&self, col: &str, window: usize) -> PyResult<SeraDFrame_> {
        self.rolling_apply(col, window, &format!("roll{}_max", window), |w| {
            w.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
        })
    }

    #[sera_doc(
        name = "SeraDFrame.rolling_std",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Adds {col}_rollN_std, a trailing rolling std of window N.",
        fr = "Ajoute {col}_rollN_std, un ecart-type glissant de fenetre N."
    )]
    fn rolling_std(&self, col: &str, window: usize) -> PyResult<SeraDFrame_> {
        self.rolling_apply(col, window, &format!("roll{}_std", window), |w| {
            let n = w.len() as f64;
            let mean = w.iter().sum::<f64>() / n;
            (w.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / (n - 1.0).max(1.0)).sqrt()
        })
    }

    #[sera_doc(
        name = "SeraDFrame.expanding_mean",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Adds {col}_expanding_mean, a running mean from row 0.",
        fr = "Ajoute {col}_expanding_mean, une moyenne cumulee depuis la ligne 0."
    )]
    fn expanding_mean(&self, col: &str) -> PyResult<SeraDFrame_> {
        let vals = self.inner.get(col)?.to_f64_vec();
        let mut sum = 0.0;
        let out: Vec<f64> = vals
            .iter()
            .enumerate()
            .map(|(i, v)| {
                sum += v;
                sum / (i as f64 + 1.0)
            })
            .collect();
        self.push_derived(col, "expanding_mean", Series::Num(Arc::new(out)))
    }

    #[sera_doc(
        name = "SeraDFrame.expanding_sum",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Adds {col}_expanding_sum, a running total from row 0.",
        fr = "Ajoute {col}_expanding_sum, une somme cumulee depuis la ligne 0."
    )]
    fn expanding_sum(&self, col: &str) -> PyResult<SeraDFrame_> {
        let vals = self.inner.get(col)?.to_f64_vec();
        let mut sum = 0.0;
        let out: Vec<f64> = vals
            .iter()
            .map(|v| {
                sum += v;
                sum
            })
            .collect();
        self.push_derived(col, "expanding_sum", Series::Num(Arc::new(out)))
    }

    #[sera_doc(
        name = "SeraDFrame.expanding_std",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Adds {col}_expanding_std, a running std (Welford) from row 0.",
        fr = "Ajoute {col}_expanding_std, un ecart-type cumule (Welford) depuis la ligne 0."
    )]
    fn expanding_std(&self, col: &str) -> PyResult<SeraDFrame_> {
        let vals = self.inner.get(col)?.to_f64_vec();
        let mut mean = 0.0;
        let mut m2 = 0.0;
        let mut count = 0.0;
        let out: Vec<f64> = vals
            .iter()
            .map(|&v| {
                count += 1.0;
                let delta = v - mean;
                mean += delta / count;
                let delta2 = v - mean;
                m2 += delta * delta2;
                if count < 2.0 {
                    0.0
                } else {
                    (m2 / (count - 1.0)).sqrt()
                }
            })
            .collect();
        self.push_derived(col, "expanding_std", Series::Num(Arc::new(out)))
    }

    #[sera_doc(
        name = "SeraDFrame.ewm_mean",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Adds {col}_ewm_mean, an exponentially weighted moving average (adjust=False semantics).",
        fr = "Ajoute {col}_ewm_mean, une moyenne mobile ponderee exponentiellement (semantique adjust=False)."
    )]
    fn ewm_mean(&self, col: &str, alpha: f64) -> PyResult<SeraDFrame_> {
        let vals = self.inner.get(col)?.to_f64_vec();
        let mut prev = f64::NAN;
        let out: Vec<f64> = vals
            .iter()
            .map(|&v| {
                prev = if prev.is_nan() { v } else { alpha * v + (1.0 - alpha) * prev };
                prev
            })
            .collect();
        self.push_derived(col, "ewm_mean", Series::Num(Arc::new(out)))
    }
}
