use super::series::Series;
use super::{SeraDFrame, SeraDFrame_};
use crate::core::dispatch::{stats_par, std_dev_par};
use crate::sera_doc_impl;
use pyo3::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

pub(super) fn cov_pair(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len().min(y.len());
    if n < 2 {
        return 0.0;
    }
    let x = &x[..n];
    let y = &y[..n];
    let mx = stats_par(x).mean();
    let my = stats_par(y).mean();
    let cov: f64 = x.par_iter().zip(y.par_iter()).map(|(xi, yi)| (xi - mx) * (yi - my)).sum();
    cov / (n as f64 - 1.0)
}

pub(super) fn corr_pair(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len().min(y.len());
    if n < 2 {
        return 0.0;
    }
    let x = &x[..n];
    let y = &y[..n];
    let mx = stats_par(x).mean();
    let my = stats_par(y).mean();
    let (cov, vx, vy) = x
        .par_iter()
        .zip(y.par_iter())
        .map(|(xi, yi)| {
            let dx = xi - mx;
            let dy = yi - my;
            (dx * dy, dx * dx, dy * dy)
        })
        .reduce(|| (0.0, 0.0, 0.0), |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2));
    let denom = (vx * vy).sqrt();
    if denom.abs() < 1e-12 {
        0.0
    } else {
        cov / denom
    }
}

#[sera_doc_impl]
#[pymethods]
impl SeraDFrame_ {
    #[sera_doc(
        name = "SeraDFrame.unique",
        category = "data_method", file = "canvas/dframe.md", en = "Unique values of a column.", fr = "Valeurs uniques d'une colonne.")]
    fn unique(&self, name: &str) -> PyResult<Vec<String>> {
        let col = self.inner.get(name)?;
        let owned;
        let vals: &[Arc<str>] = match col.as_str_slice() {
            Some(v) => v,
            None => {
                owned = col.to_str_vec().into_iter().map(Arc::from).collect::<Vec<_>>();
                &owned
            }
        };
        let mut seen = std::collections::HashSet::new();
        let mut out = Vec::new();
        for v in vals {
            if seen.insert(v.clone()) {
                out.push(v.to_string());
            }
        }
        Ok(out)
    }

    fn nunique(&self, name: &str) -> PyResult<usize> {
        Ok(self.unique(name)?.len())
    }

    #[sera_doc(
        name = "SeraDFrame.value_counts",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Counts occurrences per unique value.",
        fr = "Compte les occurrences par valeur unique.",
        aliases("counts")
    )]
    pub(crate) fn value_counts(&self, name: &str) -> PyResult<SeraDFrame_> {
        let col = self.inner.get(name)?;
        let owned;
        let vals: &[Arc<str>] = match col.as_str_slice() {
            Some(v) => v,
            None => {
                owned = col.to_str_vec().into_iter().map(Arc::from).collect::<Vec<_>>();
                &owned
            }
        };
        let mut counts: HashMap<Arc<str>, f64> = HashMap::new();
        let mut order_keys: Vec<String> = Vec::new();
        for v in vals {
            match counts.get_mut(v) {
                Some(c) => *c += 1.0,
                None => {
                    order_keys.push(v.to_string());
                    counts.insert(v.clone(), 1.0);
                }
            }
        }
        let counts: HashMap<String, f64> = counts.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
        order_keys.sort_by(|a, b| counts[b].partial_cmp(&counts[a]).unwrap());
        let mut columns = HashMap::new();
        columns.insert(name.to_string(), super::str_series(order_keys.clone()));
        columns.insert(
            "count".to_string(),
            Series::Num(Arc::new(order_keys.iter().map(|k| counts[k]).collect())),
        );
        let n = order_keys.len();
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(
                vec![name.to_string(), "count".to_string()],
                columns,
                n,
            )),
        })
    }

    #[sera_doc(
        name = "SeraDFrame.corr",
        category = "data_method", file = "canvas/dframe.md", en = "Pearson correlation between two columns.", fr = "Correlation de Pearson entre deux colonnes.")]
    fn corr(&self, a: &str, b: &str) -> PyResult<f64> {
        let xcol = self.inner.get(a)?;
        let ycol = self.inner.get(b)?;
        let xowned;
        let yowned;
        let x: &[f64] = match xcol.as_f64_slice() {
            Some(v) => v,
            None => {
                xowned = xcol.to_f64_vec();
                &xowned
            }
        };
        let y: &[f64] = match ycol.as_f64_slice() {
            Some(v) => v,
            None => {
                yowned = ycol.to_f64_vec();
                &yowned
            }
        };
        Ok(corr_pair(x, y))
    }

    #[sera_doc(
        name = "SeraDFrame.corr_matrix",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Full pairwise Pearson correlation matrix across numeric columns.",
        fr = "Matrice complete de correlation de Pearson entre colonnes numeriques."
    )]
    fn corr_matrix(&self) -> SeraDFrame_ {
        let numeric = self.inner.numeric_columns();
        let borrowed: Vec<&[f64]> = numeric.iter().map(|n| self.inner.columns[n].as_f64_slice().unwrap()).collect();
        let n = numeric.len();
        let pairs: Vec<(usize, usize)> = (0..n).flat_map(|i| ((i + 1)..n).map(move |j| (i, j))).collect();
        let results: Vec<((usize, usize), f64)> = if pairs.len() < 8 {
            pairs.iter().map(|&(i, j)| ((i, j), corr_pair(borrowed[i], borrowed[j]))).collect()
        } else {
            pairs.par_iter().map(|&(i, j)| ((i, j), corr_pair(borrowed[i], borrowed[j]))).collect()
        };
        let mut matrix = vec![vec![0.0; n]; n];
        for row in matrix.iter_mut().enumerate() {
            row.1[row.0] = 1.0;
        }
        for ((i, j), c) in results {
            matrix[i][j] = c;
            matrix[j][i] = c;
        }
        let mut order = vec!["column".to_string()];
        order.extend(numeric.iter().cloned());
        let mut columns = HashMap::new();
        columns.insert("column".to_string(), super::str_series(numeric.clone()));
        for (j, name) in numeric.iter().enumerate() {
            let col_vals: Vec<f64> = (0..n).map(|i| matrix[i][j]).collect();
            columns.insert(name.clone(), Series::Num(Arc::new(col_vals)));
        }
        SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, columns, n)),
        }
    }

    #[sera_doc(
        name = "SeraDFrame.cov",
        category = "data_method", file = "canvas/dframe.md", en = "Sample covariance between two columns.", fr = "Covariance d'echantillon entre deux colonnes.")]
    fn cov(&self, a: &str, b: &str) -> PyResult<f64> {
        let xcol = self.inner.get(a)?;
        let ycol = self.inner.get(b)?;
        let xowned;
        let yowned;
        let x: &[f64] = match xcol.as_f64_slice() {
            Some(v) => v,
            None => {
                xowned = xcol.to_f64_vec();
                &xowned
            }
        };
        let y: &[f64] = match ycol.as_f64_slice() {
            Some(v) => v,
            None => {
                yowned = ycol.to_f64_vec();
                &yowned
            }
        };
        Ok(cov_pair(x, y))
    }

    #[sera_doc(
        name = "SeraDFrame.corrwith",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Pearson correlation of each shared numeric column against another SeraDFrame.",
        fr = "Correlation de Pearson de chaque colonne numerique partagee avec un autre SeraDFrame."
    )]
    fn corrwith(&self, other: &SeraDFrame_) -> SeraDFrame_ {
        let common: Vec<String> = self
            .inner
            .numeric_columns()
            .into_iter()
            .filter(|c| matches!(other.inner.columns.get(c), Some(Series::Num(_))))
            .collect();
        let corrs: Vec<f64> = common
            .par_iter()
            .map(|c| {
                let x = self.inner.columns[c].to_f64_vec();
                let y = other.inner.columns[c].to_f64_vec();
                corr_pair(&x, &y)
            })
            .collect();
        let mut columns = HashMap::new();
        columns.insert("column".to_string(), super::str_series(common.clone()));
        columns.insert("corr".to_string(), Series::Num(Arc::new(corrs)));
        let n = common.len();
        SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(vec!["column".to_string(), "corr".to_string()], columns, n)),
        }
    }

    #[sera_doc(
        name = "SeraDFrame.mode",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Most frequent value(s) of a column, ties included.",
        fr = "Valeur(s) la/les plus frequente(s) d'une colonne, egalites incluses."
    )]
    fn mode(&self, col: &str) -> PyResult<Vec<String>> {
        let col = self.inner.get(col)?;
        let owned;
        let vals: &[Arc<str>] = match col.as_str_slice() {
            Some(v) => v,
            None => {
                owned = col.to_str_vec().into_iter().map(Arc::from).collect::<Vec<_>>();
                &owned
            }
        };
        let mut counts: HashMap<Arc<str>, u32> = HashMap::new();
        let mut order: Vec<Arc<str>> = Vec::new();
        for v in vals {
            match counts.get_mut(v) {
                Some(c) => *c += 1,
                None => {
                    order.push(v.clone());
                    counts.insert(v.clone(), 1);
                }
            }
        }
        let max_count = counts.values().copied().max().unwrap_or(0);
        Ok(order.into_iter().filter(|v| counts[v] == max_count).map(|v| v.to_string()).collect())
    }

    #[sera_doc(
        name = "SeraDFrame.sem",
        category = "data_method", file = "canvas/dframe.md", en = "Standard error of the mean of a numeric column.", fr = "Erreur standard de la moyenne d'une colonne numerique.")]
    fn sem(&self, col: &str) -> PyResult<f64> {
        let vals: Vec<f64> = self.inner.get(col)?.to_f64_vec().into_iter().filter(|v| !v.is_nan()).collect();
        let n = vals.len();
        if n < 2 {
            return Ok(0.0);
        }
        let mean = stats_par(&vals).mean();
        let std = std_dev_par(&vals, mean);
        Ok(std / (n as f64).sqrt())
    }

    #[sera_doc(
        name = "SeraDFrame.skew",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Sample-adjusted (Fisher-Pearson) skewness of a numeric column.",
        fr = "Asymetrie (Fisher-Pearson, ajustee echantillon) d'une colonne numerique."
    )]
    fn skew(&self, col: &str) -> PyResult<f64> {
        let vals: Vec<f64> = self.inner.get(col)?.to_f64_vec().into_iter().filter(|v| !v.is_nan()).collect();
        let n = vals.len();
        if n < 3 {
            return Ok(0.0);
        }
        let nf = n as f64;
        let mean = stats_par(&vals).mean();
        let (m2, m3) = vals
            .par_iter()
            .map(|v| {
                let d = v - mean;
                (d * d, d * d * d)
            })
            .reduce(|| (0.0, 0.0), |a, b| (a.0 + b.0, a.1 + b.1));
        let m2 = m2 / nf;
        let m3 = m3 / nf;
        if m2 <= 0.0 {
            return Ok(0.0);
        }
        let g1 = m3 / m2.powf(1.5);
        Ok((nf * (nf - 1.0)).sqrt() / (nf - 2.0) * g1)
    }

    #[sera_doc(
        name = "SeraDFrame.kurt",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Sample-adjusted excess kurtosis of a numeric column.",
        fr = "Kurtosis en exces (ajustee echantillon) d'une colonne numerique.",
        aliases("kurtosis")
    )]
    pub(crate) fn kurt(&self, col: &str) -> PyResult<f64> {
        let vals: Vec<f64> = self.inner.get(col)?.to_f64_vec().into_iter().filter(|v| !v.is_nan()).collect();
        let n = vals.len();
        if n < 4 {
            return Ok(0.0);
        }
        let nf = n as f64;
        let mean = stats_par(&vals).mean();
        let (m2, m4) = vals
            .par_iter()
            .map(|v| {
                let d = v - mean;
                let d2 = d * d;
                (d2, d2 * d2)
            })
            .reduce(|| (0.0, 0.0), |a, b| (a.0 + b.0, a.1 + b.1));
        let m2 = m2 / nf;
        let m4 = m4 / nf;
        if m2 <= 0.0 {
            return Ok(0.0);
        }
        let g2 = m4 / (m2 * m2) - 3.0;
        Ok(((nf - 1.0) / ((nf - 2.0) * (nf - 3.0))) * ((nf + 1.0) * g2 + 6.0))
    }

    #[sera_doc(
        name = "SeraDFrame.any",
        category = "data_method", file = "canvas/dframe.md", en = "True if any value in a column is truthy.", fr = "Vrai si une valeur de la colonne est vraie.")]
    fn any(&self, col: &str) -> PyResult<bool> {
        let series = self.inner.get(col)?;
        Ok(match series {
            Series::Bool(v) => v.par_iter().any(|b| *b),
            Series::Num(v) => v.par_iter().any(|x| *x != 0.0),
            Series::Str(v) => v.par_iter().any(|s| !s.is_empty()),
        })
    }

    #[sera_doc(
        name = "SeraDFrame.all",
        category = "data_method", file = "canvas/dframe.md", en = "True if every value in a column is truthy.", fr = "Vrai si toutes les valeurs de la colonne sont vraies.")]
    fn all(&self, col: &str) -> PyResult<bool> {
        let series = self.inner.get(col)?;
        Ok(match series {
            Series::Bool(v) => v.par_iter().all(|b| *b),
            Series::Num(v) => v.par_iter().all(|x| *x != 0.0),
            Series::Str(v) => v.par_iter().all(|s| !s.is_empty()),
        })
    }

    #[sera_doc(
        name = "SeraDFrame.get_dummies",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "One-hot encodes a categorical column into {col}_{category} flags.",
        fr = "Encode one-hot une colonne categorielle en indicateurs {col}_{categorie}."
    )]
    fn get_dummies(&self, col: &str) -> PyResult<SeraDFrame_> {
        let series = self.inner.get(col)?;
        let owned;
        let vals: &[Arc<str>] = match series.as_str_slice() {
            Some(v) => v,
            None => {
                owned = series.to_str_vec().into_iter().map(Arc::from).collect::<Vec<_>>();
                &owned
            }
        };
        let mut categories: Vec<Arc<str>> = Vec::new();
        let mut code_of: HashMap<Arc<str>, u32> = HashMap::new();
        for v in vals {
            if !code_of.contains_key(v) {
                code_of.insert(v.clone(), categories.len() as u32);
                categories.push(v.clone());
            }
        }
        let codes: Vec<u32> = vals.par_iter().map(|v| code_of[v]).collect();
        let mut order = self.inner.order.clone();
        let mut columns = self.inner.columns.clone();
        for (j, cat) in categories.iter().enumerate() {
            let name = format!("{}_{}", col, cat);
            let j = j as u32;
            let flags: Vec<f64> = codes.par_iter().map(|&c| if c == j { 1.0 } else { 0.0 }).collect();
            if !columns.contains_key(&name) {
                order.push(name.clone());
            }
            columns.insert(name, Series::Num(Arc::new(flags)));
        }
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, columns, self.inner.nrows)),
        })
    }

    #[sera_doc(
        name = "SeraDFrame.describe",
        category = "data_method", file = "canvas/dframe.md", en = "Per-column count/mean/min/max/std.", fr = "Count/mean/min/max/std par colonne.")]
    fn describe(&self) -> SeraDFrame_ {
        let numeric = self.inner.numeric_columns();
        let computed: Vec<(String, f64, f64, f64, f64, f64)> = numeric
            .par_iter()
            .filter_map(|col| {
                let vals: Vec<f64> = match &self.inner.columns[col] {
                    Series::Num(v) => v.iter().copied().filter(|x| !x.is_nan()).collect(),
                    _ => return None,
                };
                if vals.is_empty() {
                    return None;
                }
                let s = stats_par(&vals);
                let mean = s.mean();
                let std = std_dev_par(&vals, mean);
                Some((col.clone(), s.count as f64, mean, s.min, s.max, std))
            })
            .collect();
        let mut names = Vec::new();
        let mut counts = Vec::new();
        let mut means = Vec::new();
        let mut mins = Vec::new();
        let mut maxs = Vec::new();
        let mut stds = Vec::new();
        for (name, count, mean, min, max, std) in computed {
            names.push(name);
            counts.push(count);
            means.push(mean);
            mins.push(min);
            maxs.push(max);
            stds.push(std);
        }
        let mut columns = HashMap::new();
        columns.insert("column".to_string(), super::str_series(names.clone()));
        columns.insert("count".to_string(), Series::Num(Arc::new(counts)));
        columns.insert("mean".to_string(), Series::Num(Arc::new(means)));
        columns.insert("min".to_string(), Series::Num(Arc::new(mins)));
        columns.insert("max".to_string(), Series::Num(Arc::new(maxs)));
        columns.insert("std".to_string(), Series::Num(Arc::new(stds)));
        let n = names.len();
        SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(
                vec![
                    "column".to_string(),
                    "count".to_string(),
                    "mean".to_string(),
                    "min".to_string(),
                    "max".to_string(),
                    "std".to_string(),
                ],
                columns,
                n,
            )),
        }
    }
}
