use super::SeraDFrame_;
use crate::core::dispatch::{stats_par, std_dev_par};
use crate::sera_doc_impl;
use pyo3::prelude::*;
use rayon::prelude::*;

fn drop_nan(vals: &[f64]) -> Vec<f64> {
    vals.par_iter().copied().filter(|x| !x.is_nan()).collect()
}

fn nan_safe_sum_count_slow(vals: &[f64]) -> (f64, usize) {
    let threads = rayon::current_num_threads().max(1);
    if vals.len() < 50_000 {
        let mut sum = 0.0;
        let mut count = 0usize;
        for &v in vals {
            if !v.is_nan() {
                sum += v;
                count += 1;
            }
        }
        return (sum, count);
    }
    let chunk = (vals.len() / threads).max(4096);
    vals.par_chunks(chunk)
        .map(|c| {
            let mut sum = 0.0;
            let mut count = 0usize;
            for &v in c {
                if !v.is_nan() {
                    sum += v;
                    count += 1;
                }
            }
            (sum, count)
        })
        .reduce(|| (0.0, 0usize), |a, b| (a.0 + b.0, a.1 + b.1))
}

fn nan_safe_sum_count(vals: &[f64]) -> (f64, usize) {
    if vals.len() < 50_000 {
        let sum: f64 = vals.iter().sum();
        return if sum.is_nan() && vals.iter().any(|v| v.is_nan()) {
            nan_safe_sum_count_slow(vals)
        } else {
            (sum, vals.len())
        };
    }
    let threads = rayon::current_num_threads().max(1);
    let chunk = (vals.len() / threads).max(4096);
    let sum: f64 = vals.par_chunks(chunk).map(|c| c.iter().sum::<f64>()).sum();
    if sum.is_nan() && vals.par_iter().any(|v| v.is_nan()) {
        nan_safe_sum_count_slow(vals)
    } else {
        (sum, vals.len())
    }
}

impl SeraDFrame_ {
    fn numeric_slice<'a>(&'a self, col: &str, owned: &'a mut Vec<f64>) -> PyResult<&'a [f64]> {
        let series = self.inner.get(col)?;
        Ok(match series.as_f64_slice() {
            Some(v) => v,
            None => {
                *owned = series.to_f64_vec();
                owned.as_slice()
            }
        })
    }

    fn numeric_clean(&self, col: &str) -> PyResult<Vec<f64>> {
        let mut owned = Vec::new();
        let vals = self.numeric_slice(col, &mut owned)?;
        Ok(drop_nan(vals))
    }
}

#[sera_doc_impl]
#[pymethods]
impl SeraDFrame_ {
    #[sera_doc(
        name = "SeraDFrame.sum",
        category = "data_method", file = "canvas/dframe.md", en = "Sum of a numeric column.", fr = "Somme d'une colonne numerique.")]
    fn sum(&self, col: &str) -> PyResult<f64> {
        let mut owned = Vec::new();
        let vals = self.numeric_slice(col, &mut owned)?;
        Ok(nan_safe_sum_count(vals).0)
    }

    #[sera_doc(
        name = "SeraDFrame.mean",
        category = "data_method", file = "canvas/dframe.md", en = "Mean of a numeric column.", fr = "Moyenne d'une colonne numerique.")]
    fn mean(&self, col: &str) -> PyResult<f64> {
        let mut owned = Vec::new();
        let vals = self.numeric_slice(col, &mut owned)?;
        let (sum, count) = nan_safe_sum_count(vals);
        Ok(if count == 0 { f64::NAN } else { sum / count as f64 })
    }

    #[sera_doc(
        name = "SeraDFrame.min",
        category = "data_method", file = "canvas/dframe.md", en = "Minimum of a numeric column.", fr = "Minimum d'une colonne numerique.")]
    fn min(&self, col: &str) -> PyResult<f64> {
        Ok(stats_par(&self.numeric_clean(col)?).min)
    }

    #[sera_doc(
        name = "SeraDFrame.max",
        category = "data_method", file = "canvas/dframe.md", en = "Maximum of a numeric column.", fr = "Maximum d'une colonne numerique.")]
    fn max(&self, col: &str) -> PyResult<f64> {
        Ok(stats_par(&self.numeric_clean(col)?).max)
    }

    #[sera_doc(
        name = "SeraDFrame.std",
        category = "data_method", file = "canvas/dframe.md", en = "Standard deviation of a numeric column.", fr = "Ecart-type d'une colonne numerique.")]
    fn std(&self, col: &str) -> PyResult<f64> {
        let vals = self.numeric_clean(col)?;
        let mean = stats_par(&vals).mean();
        Ok(std_dev_par(&vals, mean))
    }

    #[sera_doc(
        name = "SeraDFrame.var",
        category = "data_method", file = "canvas/dframe.md", en = "Sample variance of a numeric column.", fr = "Variance echantillon d'une colonne numerique.")]
    fn var(&self, col: &str) -> PyResult<f64> {
        let vals = self.numeric_clean(col)?;
        let mean = stats_par(&vals).mean();
        let sd = std_dev_par(&vals, mean);
        Ok(sd * sd)
    }

    #[sera_doc(
        name = "SeraDFrame.median",
        category = "data_method", file = "canvas/dframe.md", en = "Median of a numeric column via O(n) selection.", fr = "Mediane d'une colonne numerique via selection O(n).")]
    fn median(&self, col: &str) -> PyResult<f64> {
        let mut vals = self.numeric_clean(col)?;
        let n = vals.len();
        if n == 0 {
            return Ok(f64::NAN);
        }
        let mid = n / 2;
        vals.select_nth_unstable_by(mid, |a, b| a.partial_cmp(b).unwrap());
        if n % 2 == 1 {
            return Ok(vals[mid]);
        }
        let hi = vals[mid];
        let lo = vals[..mid].iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        Ok((hi + lo) / 2.0)
    }

    #[sera_doc(
        name = "SeraDFrame.quantile",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Arbitrary quantile of a numeric column via O(n) selection and linear interpolation.",
        fr = "Quantile arbitraire d'une colonne numerique via selection O(n) et interpolation lineaire."
    )]
    fn quantile(&self, col: &str, q: f64) -> PyResult<f64> {
        let mut vals = self.numeric_clean(col)?;
        let n = vals.len();
        if n == 0 {
            return Ok(f64::NAN);
        }
        if n == 1 {
            return Ok(vals[0]);
        }
        let q = q.clamp(0.0, 1.0);
        let pos = q * (n - 1) as f64;
        let lo = pos.floor() as usize;
        let hi = pos.ceil() as usize;
        let cmp = |a: &f64, b: &f64| a.partial_cmp(b).unwrap();
        vals.select_nth_unstable_by(lo, cmp);
        let lo_val = vals[lo];
        if hi == lo {
            return Ok(lo_val);
        }
        vals.select_nth_unstable_by(hi, cmp);
        let hi_val = vals[hi];
        let frac = pos - lo as f64;
        Ok(lo_val + (hi_val - lo_val) * frac)
    }

    #[sera_doc(
        name = "SeraDFrame.idxmax",
        category = "data_method", file = "canvas/dframe.md", en = "Row index of the maximum value.", fr = "Index de ligne de la valeur maximale.")]
    fn idxmax(&self, col: &str) -> PyResult<usize> {
        let series = self.inner.get(col)?;
        let owned;
        let vals: &[f64] = match series.as_f64_slice() {
            Some(v) => v,
            None => {
                owned = series.to_f64_vec();
                &owned
            }
        };
        let mut best: Option<usize> = None;
        for (i, v) in vals.iter().enumerate() {
            if v.is_nan() {
                continue;
            }
            best = match best {
                Some(b) if vals[b] >= *v => Some(b),
                _ => Some(i),
            };
        }
        best.ok_or_else(|| pyo3::exceptions::PyValueError::new_err("no non-NaN values"))
    }

    #[sera_doc(
        name = "SeraDFrame.idxmin",
        category = "data_method", file = "canvas/dframe.md", en = "Row index of the minimum value.", fr = "Index de ligne de la valeur minimale.")]
    fn idxmin(&self, col: &str) -> PyResult<usize> {
        let series = self.inner.get(col)?;
        let owned;
        let vals: &[f64] = match series.as_f64_slice() {
            Some(v) => v,
            None => {
                owned = series.to_f64_vec();
                &owned
            }
        };
        let mut best: Option<usize> = None;
        for (i, v) in vals.iter().enumerate() {
            if v.is_nan() {
                continue;
            }
            best = match best {
                Some(b) if vals[b] <= *v => Some(b),
                _ => Some(i),
            };
        }
        best.ok_or_else(|| pyo3::exceptions::PyValueError::new_err("no non-NaN values"))
    }
}
