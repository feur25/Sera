use super::super::series::{f64_sort_key, Series};
use super::super::{str_series, SeraDFrame, SeraDFrame_};
use crate::sera_doc_impl;
use pyo3::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[sera_doc_impl]
#[pymethods]
impl SeraDFrame_ {
    #[sera_doc(
        name = "SeraDFrame.melt",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Unpivots id/value columns into long variable/value rows.",
        fr = "Depivote des colonnes id/value en lignes longues variable/value.",
        aliases("unpivot")
    )]
    pub(crate) fn melt(&self, id_vars: Vec<String>, value_vars: Vec<String>) -> PyResult<SeraDFrame_> {
        let nrows = self.inner.nrows;
        let n_out = nrows * value_vars.len();
        let all_numeric = value_vars.iter().all(|v| matches!(self.inner.columns.get(v), Some(Series::Num(_))));

        let mut variable: Vec<Arc<str>> = Vec::with_capacity(n_out);
        for vname in &value_vars {
            let arc_name: Arc<str> = Arc::from(vname.as_str());
            variable.extend(std::iter::repeat(arc_name).take(nrows));
        }

        let mut id_series: Vec<Series> = Vec::with_capacity(id_vars.len());
        for name in &id_vars {
            let col = self.inner.get(name)?;
            let repeated = match col {
                Series::Num(v) => {
                    let mut out = Vec::with_capacity(n_out);
                    for _ in 0..value_vars.len() {
                        out.extend_from_slice(v);
                    }
                    Series::Num(Arc::new(out))
                }
                Series::Str(v) => {
                    let mut out = Vec::with_capacity(n_out);
                    for _ in 0..value_vars.len() {
                        out.extend_from_slice(v);
                    }
                    Series::Str(Arc::new(out))
                }
                Series::Bool(v) => {
                    let mut out = Vec::with_capacity(n_out);
                    for _ in 0..value_vars.len() {
                        out.extend_from_slice(v);
                    }
                    Series::Bool(Arc::new(out))
                }
            };
            id_series.push(repeated);
        }

        let mut value_str: Vec<String> = Vec::new();
        let mut value_num: Vec<f64> = Vec::new();
        for vname in &value_vars {
            let col = self.inner.get(vname)?;
            if all_numeric {
                value_num.extend(col.to_f64_vec());
            } else {
                value_str.extend(col.to_str_vec());
            }
        }

        let mut order = id_vars.clone();
        order.push("variable".to_string());
        order.push("value".to_string());
        let mut columns = HashMap::new();
        for (name, series) in id_vars.iter().zip(id_series.into_iter()) {
            columns.insert(name.clone(), series);
        }
        columns.insert("variable".to_string(), Series::Str(Arc::new(variable)));
        if all_numeric {
            columns.insert("value".to_string(), Series::Num(Arc::new(value_num)));
        } else {
            columns.insert("value".to_string(), str_series(value_str));
        }
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, columns, n_out)),
        })
    }

    #[pyo3(signature = (col, q = 4))]
    #[sera_doc(
        name = "SeraDFrame.qcut",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Adds {col}_qbin, equal-frequency quantile bin index.",
        fr = "Ajoute {col}_qbin, index de bin par quantile a frequence egale."
    )]
    fn qcut(&self, col: &str, q: usize) -> PyResult<SeraDFrame_> {
        let vals = self.inner.get(col)?.to_f64_vec();
        let n = vals.len();
        let q = q.max(1);
        let mut keyed: Vec<(u64, usize)> = vals.par_iter().enumerate().map(|(i, &x)| (f64_sort_key(x), i)).collect();
        keyed.par_sort_unstable_by_key(|&(k, _)| k);
        let mut bins = vec![0.0; n];
        for (rank, &(_, i)) in keyed.iter().enumerate() {
            let bin = (rank * q / n.max(1)).min(q - 1);
            bins[i] = bin as f64;
        }
        self.push_derived(col, "qbin", Series::Num(Arc::new(bins)))
    }

    #[sera_doc(
        name = "SeraDFrame.interpolate",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Fills NaN gaps in a column by linear interpolation.",
        fr = "Comble les trous NaN d'une colonne par interpolation lineaire."
    )]
    fn interpolate(&self, col: &str) -> PyResult<SeraDFrame_> {
        let vals = self.inner.get(col)?.to_f64_vec();
        let n = vals.len();
        let mut out = vals.clone();
        let mut i = 0;
        while i < n {
            if out[i].is_nan() {
                let start = if i == 0 { None } else { Some((i - 1, out[i - 1])) };
                let mut j = i;
                while j < n && out[j].is_nan() {
                    j += 1;
                }
                let end = if j < n { Some((j, out[j])) } else { None };
                match (start, end) {
                    (Some((si, sv)), Some((ei, ev))) => {
                        for k in i..j {
                            let t = (k - si) as f64 / (ei - si) as f64;
                            out[k] = sv + (ev - sv) * t;
                        }
                    }
                    (None, Some((_, ev))) => {
                        for k in i..j {
                            out[k] = ev;
                        }
                    }
                    (Some((_, sv)), None) => {
                        for k in i..j {
                            out[k] = sv;
                        }
                    }
                    (None, None) => {}
                }
                i = j;
            } else {
                i += 1;
            }
        }
        let mut columns = self.inner.columns.clone();
        columns.insert(col.to_string(), Series::Num(Arc::new(out)));
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(self.inner.order.clone(), columns, self.inner.nrows)),
        })
    }
}
