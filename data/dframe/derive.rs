use super::series::{column_from_pyobjects, Series};
use super::{SeraDFrame, SeraDFrame_};
use crate::core::dispatch::stats_par;
use crate::sera_doc_impl;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

#[sera_doc_impl]
#[pymethods]
impl SeraDFrame_ {
    #[sera_doc(
        name = "SeraDFrame.with_columns",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Batch-adds or replaces several columns in one pass.",
        fr = "Ajoute ou remplace plusieurs colonnes en un seul passage."
    )]
    fn with_columns(&self, cols: &Bound<'_, PyDict>) -> PyResult<SeraDFrame_> {
        let mut order = self.inner.order.clone();
        let mut columns = self.inner.columns.clone();
        let mut nrows = self.inner.nrows;
        for (k, v) in cols.iter() {
            let name: String = k.extract()?;
            let items: Vec<Bound<'_, PyAny>> = v.extract()?;
            let series = column_from_pyobjects(items);
            nrows = nrows.max(series.len());
            if !columns.contains_key(&name) {
                order.push(name.clone());
            }
            columns.insert(name, series);
        }
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, columns, nrows)),
        })
    }

    #[sera_doc(
        name = "SeraDFrame.diff",
        category = "data_method", file = "canvas/dframe.md", en = "Adds {col}_diff, the row-over-row difference.", fr = "Ajoute {col}_diff, la difference ligne a ligne.")]
    fn diff(&self, col: &str) -> PyResult<SeraDFrame_> {
        let vals = self.inner.get(col)?.to_f64_vec();
        let out: Vec<f64> = (0..vals.len())
            .into_par_iter()
            .map(|i| if i == 0 { 0.0 } else { vals[i] - vals[i - 1] })
            .collect();
        self.push_derived(col, "diff", Series::Num(Arc::new(out)))
    }

    #[sera_doc(
        name = "SeraDFrame.normalize",
        category = "data_method", file = "canvas/dframe.md", en = "Adds {col}_norm, min-max scaled to 0..1.", fr = "Ajoute {col}_norm, mis a l'echelle min-max entre 0 et 1.")]
    fn normalize(&self, col: &str) -> PyResult<SeraDFrame_> {
        let vals = self.inner.get(col)?.to_f64_vec();
        let s = stats_par(&vals);
        let range = (s.max - s.min).max(1e-12);
        let out: Vec<f64> = vals.par_iter().map(|v| (v - s.min) / range).collect();
        self.push_derived(col, "norm", Series::Num(Arc::new(out)))
    }

    #[pyo3(signature = (col, n_bins = 10))]
    #[sera_doc(
        name = "SeraDFrame.bin",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Adds {col}_bin, an equal-width bin index in 0..n_bins.",
        fr = "Ajoute {col}_bin, un index de bin a largeur egale entre 0 et n_bins."
    )]
    fn bin(&self, col: &str, n_bins: usize) -> PyResult<SeraDFrame_> {
        let vals = self.inner.get(col)?.to_f64_vec();
        let s = stats_par(&vals);
        let range = (s.max - s.min).max(1e-12);
        let n_bins = n_bins.max(1);
        let n_bins_f = n_bins as f64;
        let out: Vec<f64> = vals
            .par_iter()
            .map(|v| (((v - s.min) / range) * n_bins_f).floor().clamp(0.0, n_bins_f - 1.0))
            .collect();
        self.push_derived(col, "bin", Series::Num(Arc::new(out)))
    }

    #[sera_doc(
        name = "SeraDFrame.nlargest",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Top-n rows by column value via O(n) partial selection, no full sort.",
        fr = "Les n plus grandes lignes selon une colonne via selection partielle O(n), sans tri complet.",
        aliases("top_k")
    )]
    pub(crate) fn nlargest(&self, col: &str, n: usize) -> PyResult<SeraDFrame_> {
        self.select_extreme(col, n, true)
    }

    #[sera_doc(
        name = "SeraDFrame.nsmallest",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Bottom-n rows by column value via O(n) partial selection, no full sort.",
        fr = "Les n plus petites lignes selon une colonne via selection partielle O(n), sans tri complet.",
        aliases("bottom_k")
    )]
    pub(crate) fn nsmallest(&self, col: &str, n: usize) -> PyResult<SeraDFrame_> {
        self.select_extreme(col, n, false)
    }

    #[pyo3(signature = (col, periods = 1))]
    #[sera_doc(
        name = "SeraDFrame.shift",
        category = "data_method", file = "canvas/dframe.md", en = "Adds {col}_shiftN, values shifted by N periods.", fr = "Ajoute {col}_shiftN, valeurs decalees de N periodes.")]
    fn shift(&self, col: &str, periods: i64) -> PyResult<SeraDFrame_> {
        let vals = self.inner.get(col)?.to_f64_vec();
        let n = vals.len() as i64;
        let out: Vec<f64> = (0..n)
            .into_par_iter()
            .map(|i| {
                let src = i - periods;
                if src >= 0 && src < n {
                    vals[src as usize]
                } else {
                    f64::NAN
                }
            })
            .collect();
        self.push_derived(col, &format!("shift{}", periods), Series::Num(Arc::new(out)))
    }

    #[sera_doc(
        name = "SeraDFrame.pct_change",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Adds {col}_pct_change, row-over-row percent change.",
        fr = "Ajoute {col}_pct_change, variation en pourcentage ligne a ligne."
    )]
    fn pct_change(&self, col: &str) -> PyResult<SeraDFrame_> {
        let vals = self.inner.get(col)?.to_f64_vec();
        let out: Vec<f64> = (0..vals.len())
            .into_par_iter()
            .map(|i| if i == 0 || vals[i - 1] == 0.0 { f64::NAN } else { (vals[i] - vals[i - 1]) / vals[i - 1] })
            .collect();
        self.push_derived(col, "pct_change", Series::Num(Arc::new(out)))
    }

    #[sera_doc(
        name = "SeraDFrame.cumsum",
        category = "data_method", file = "canvas/dframe.md", en = "Adds {col}_cumsum, running total.", fr = "Ajoute {col}_cumsum, somme cumulee.")]
    fn cumsum(&self, col: &str) -> PyResult<SeraDFrame_> {
        let vals = self.inner.get(col)?.to_f64_vec();
        let mut acc = 0.0;
        let out: Vec<f64> = vals
            .iter()
            .map(|v| {
                if v.is_nan() {
                    return f64::NAN;
                }
                acc += v;
                acc
            })
            .collect();
        self.push_derived(col, "cumsum", Series::Num(Arc::new(out)))
    }

    #[sera_doc(
        name = "SeraDFrame.cumprod",
        category = "data_method", file = "canvas/dframe.md", en = "Adds {col}_cumprod, running product.", fr = "Ajoute {col}_cumprod, produit cumule.")]
    fn cumprod(&self, col: &str) -> PyResult<SeraDFrame_> {
        let vals = self.inner.get(col)?.to_f64_vec();
        let mut acc = 1.0;
        let out: Vec<f64> = vals
            .iter()
            .map(|v| {
                if v.is_nan() {
                    return f64::NAN;
                }
                acc *= v;
                acc
            })
            .collect();
        self.push_derived(col, "cumprod", Series::Num(Arc::new(out)))
    }

    #[sera_doc(
        name = "SeraDFrame.cummax",
        category = "data_method", file = "canvas/dframe.md", en = "Adds {col}_cummax, running maximum.", fr = "Ajoute {col}_cummax, maximum cumule.")]
    fn cummax(&self, col: &str) -> PyResult<SeraDFrame_> {
        let vals = self.inner.get(col)?.to_f64_vec();
        let mut acc = f64::NEG_INFINITY;
        let out: Vec<f64> = vals
            .iter()
            .map(|v| {
                if v.is_nan() {
                    return f64::NAN;
                }
                acc = acc.max(*v);
                acc
            })
            .collect();
        self.push_derived(col, "cummax", Series::Num(Arc::new(out)))
    }

    #[sera_doc(
        name = "SeraDFrame.cummin",
        category = "data_method", file = "canvas/dframe.md", en = "Adds {col}_cummin, running minimum.", fr = "Ajoute {col}_cummin, minimum cumule.")]
    fn cummin(&self, col: &str) -> PyResult<SeraDFrame_> {
        let vals = self.inner.get(col)?.to_f64_vec();
        let mut acc = f64::INFINITY;
        let out: Vec<f64> = vals
            .iter()
            .map(|v| {
                if v.is_nan() {
                    return f64::NAN;
                }
                acc = acc.min(*v);
                acc
            })
            .collect();
        self.push_derived(col, "cummin", Series::Num(Arc::new(out)))
    }

    #[sera_doc(
        name = "SeraDFrame.rank",
        category = "data_method", file = "canvas/dframe.md", en = "Adds {col}_rank, average rank with ties.", fr = "Ajoute {col}_rank, rang moyen en cas d'egalite.")]
    fn rank(&self, col: &str) -> PyResult<SeraDFrame_> {
        let vals = self.inner.get(col)?.to_f64_vec();
        let mut idx: Vec<usize> = (0..vals.len()).filter(|&i| !vals[i].is_nan()).collect();
        idx.sort_by(|&a, &b| vals[a].partial_cmp(&vals[b]).unwrap());
        let mut ranks = vec![f64::NAN; vals.len()];
        let mut i = 0;
        while i < idx.len() {
            let mut j = i;
            while j + 1 < idx.len() && vals[idx[j + 1]] == vals[idx[i]] {
                j += 1;
            }
            let avg_rank = ((i + 1) + (j + 1)) as f64 / 2.0;
            for k in i..=j {
                ranks[idx[k]] = avg_rank;
            }
            i = j + 1;
        }
        self.push_derived(col, "rank", Series::Num(Arc::new(ranks)))
    }

    #[sera_doc(
        name = "SeraDFrame.astype",
        category = "data_method", file = "canvas/dframe.md", en = "Casts a column to float/str/bool.", fr = "Convertit une colonne en float/str/bool.")]
    fn astype(&self, col: &str, dtype: &str) -> PyResult<SeraDFrame_> {
        let series = self.inner.get(col)?;
        let converted = match dtype {
            "float" | "float64" | "num" | "numeric" => Series::Num(Arc::new(series.to_f64_vec())),
            "str" | "object" | "string" => super::str_series(series.to_str_vec()),
            "bool" | "boolean" => Series::Bool(Arc::new(series.to_f64_vec().iter().map(|v| *v != 0.0).collect())),
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "unsupported dtype '{}', expected float/str/bool",
                    dtype
                )))
            }
        };
        let mut columns = self.inner.columns.clone();
        columns.insert(col.to_string(), converted);
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(self.inner.order.clone(), columns, self.inner.nrows)),
        })
    }

    #[pyo3(signature = (n, seed = None))]
    #[sera_doc(
        name = "SeraDFrame.sample",
        category = "data_method", file = "canvas/dframe.md", en = "Random row sample via reservoir sampling.", fr = "Echantillon aleatoire de lignes par reservoir sampling.")]
    fn sample(&self, n: usize, seed: Option<u64>) -> SeraDFrame_ {
        let total = self.inner.nrows;
        let n = n.min(total);
        let mut state = seed.unwrap_or_else(|| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0x2545F4914F6CDD1D)
        });
        let mut idx: Vec<usize> = (0..n).collect();
        for i in n..total {
            let j = (splitmix64(&mut state) as usize) % (i + 1);
            if j < n {
                idx[j] = i;
            }
        }
        idx.sort_unstable();
        SeraDFrame_ {
            inner: Arc::new(self.inner.row_take(&idx)),
        }
    }

    #[sera_doc(
        name = "SeraDFrame.agg",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Whole-frame aggregation into a single-row SeraDFrame.",
        fr = "Agregation de tout le frame en un SeraDFrame d'une ligne.",
        aliases("aggregate")
    )]
    pub(crate) fn agg(&self, spec: &Bound<'_, PyDict>) -> PyResult<SeraDFrame_> {
        let mut order = Vec::new();
        let mut columns = HashMap::new();
        for (k, v) in spec.iter() {
            let col_name: String = k.extract()?;
            let agg_kind: String = v.extract()?;
            let vals = self.inner.get(&col_name)?.to_f64_vec();
            let s = stats_par(&vals);
            let result = match agg_kind.as_str() {
                "sum" => s.sum,
                "min" => s.min,
                "max" => s.max,
                "count" => s.count as f64,
                _ => s.mean(),
            };
            order.push(col_name.clone());
            columns.insert(col_name, Series::Num(Arc::new(vec![result])));
        }
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, columns, 1)),
        })
    }
}

impl SeraDFrame_ {
    fn select_extreme(&self, col: &str, n: usize, largest: bool) -> PyResult<SeraDFrame_> {
        let vals = self.inner.get(col)?.to_f64_vec();
        let total = vals.len();
        let n = n.min(total);
        let mut idx: Vec<usize> = (0..total).collect();
        let cmp = |a: &usize, b: &usize| {
            if largest {
                vals[*b].partial_cmp(&vals[*a]).unwrap()
            } else {
                vals[*a].partial_cmp(&vals[*b]).unwrap()
            }
        };
        if n > 0 && n < total {
            idx.select_nth_unstable_by(n - 1, cmp);
        }
        idx.truncate(n);
        idx.sort_by(cmp);
        Ok(SeraDFrame_ {
            inner: Arc::new(self.inner.row_take(&idx)),
        })
    }

    pub(super) fn push_derived(&self, col: &str, suffix: &str, series: Series) -> PyResult<SeraDFrame_> {
        let name = format!("{}_{}", col, suffix);
        let mut order = self.inner.order.clone();
        let mut columns = self.inner.columns.clone();
        if !columns.contains_key(&name) {
            order.push(name.clone());
        }
        columns.insert(name, series);
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, columns, self.inner.nrows)),
        })
    }
}
