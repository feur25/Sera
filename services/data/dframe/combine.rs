use super::series::{FxBuildHasher, Series};
use super::{SeraDFrame, SeraDFrame_};
use crate::sera_doc_impl;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[sera_doc_impl]
#[pymethods]
impl SeraDFrame_ {
    #[sera_doc(
        name = "SeraDFrame.align",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Aligns two SeraDFrames onto the union of their columns, filling gaps.",
        fr = "Aligne deux SeraDFrames sur l'union de leurs colonnes, comble les trous."
    )]
    fn align(&self, other: &SeraDFrame_) -> (SeraDFrame_, SeraDFrame_) {
        let mut order = self.inner.order.clone();
        for c in &other.inner.order {
            if !order.contains(c) {
                order.push(c.clone());
            }
        }
        let build = |frame: &SeraDFrame_| -> SeraDFrame_ {
            let mut columns = HashMap::new();
            for name in &order {
                let s = frame
                    .inner
                    .columns
                    .get(name)
                    .cloned()
                    .unwrap_or_else(|| Series::Num(Arc::new(vec![f64::NAN; frame.inner.nrows])));
                columns.insert(name.clone(), s);
            }
            SeraDFrame_ {
                inner: Arc::new(SeraDFrame::from_parts(order.clone(), columns, frame.inner.nrows)),
            }
        };
        (build(self), build(other))
    }

    #[sera_doc(
        name = "SeraDFrame.update",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Overwrites shared columns with the other frame's non-missing values, row by row.",
        fr = "Remplace les colonnes partagees par les valeurs non manquantes de l'autre frame, ligne par ligne."
    )]
    fn update(&self, other: &SeraDFrame_) -> SeraDFrame_ {
        let n = self.inner.nrows.min(other.inner.nrows);
        let mut columns = self.inner.columns.clone();
        for name in &self.inner.order {
            if let Some(other_col) = other.inner.columns.get(name) {
                let merged = match (&self.inner.columns[name], other_col) {
                    (Series::Num(a), Series::Num(b)) => {
                        let mut out = a.as_ref().clone();
                        for i in 0..n {
                            if !b[i].is_nan() {
                                out[i] = b[i];
                            }
                        }
                        Series::Num(Arc::new(out))
                    }
                    (Series::Str(a), Series::Str(b)) => {
                        let mut out = a.as_ref().clone();
                        for i in 0..n {
                            if !b[i].is_empty() {
                                out[i] = b[i].clone();
                            }
                        }
                        Series::Str(Arc::new(out))
                    }
                    (self_col, _) => self_col.clone(),
                };
                columns.insert(name.clone(), merged);
            }
        }
        SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(self.inner.order.clone(), columns, self.inner.nrows)),
        }
    }

    #[sera_doc(
        name = "SeraDFrame.combine_first",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Fills this frame's missing values from the other frame, row by row.",
        fr = "Comble les valeurs manquantes de ce frame avec l'autre frame, ligne par ligne."
    )]
    fn combine_first(&self, other: &SeraDFrame_) -> SeraDFrame_ {
        let n = self.inner.nrows.min(other.inner.nrows);
        let mut columns = self.inner.columns.clone();
        for name in &self.inner.order {
            if let Some(other_col) = other.inner.columns.get(name) {
                let merged = match (&self.inner.columns[name], other_col) {
                    (Series::Num(a), Series::Num(b)) => {
                        let mut out = a.as_ref().clone();
                        for i in 0..n {
                            if out[i].is_nan() {
                                out[i] = b[i];
                            }
                        }
                        Series::Num(Arc::new(out))
                    }
                    (Series::Str(a), Series::Str(b)) => {
                        let mut out = a.as_ref().clone();
                        for i in 0..n {
                            if out[i].is_empty() {
                                out[i] = b[i].clone();
                            }
                        }
                        Series::Str(Arc::new(out))
                    }
                    (self_col, _) => self_col.clone(),
                };
                columns.insert(name.clone(), merged);
            }
        }
        SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(self.inner.order.clone(), columns, self.inner.nrows)),
        }
    }

    #[sera_doc(
        name = "SeraDFrame.compare",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Boolean mask of shared columns showing where two SeraDFrames differ.",
        fr = "Masque booleen des colonnes partagees indiquant ou deux SeraDFrames different."
    )]
    fn compare(&self, other: &SeraDFrame_) -> SeraDFrame_ {
        let n = self.inner.nrows.min(other.inner.nrows);
        let shared: Vec<String> = self.inner.order.iter().filter(|c| other.inner.columns.contains_key(*c)).cloned().collect();
        let mut columns = HashMap::new();
        for name in &shared {
            let a = &self.inner.columns[name];
            let b = &other.inner.columns[name];
            let diff: Vec<bool> = (0..n)
                .map(|i| match (a, b) {
                    (Series::Num(x), Series::Num(y)) => !(x[i] == y[i] || (x[i].is_nan() && y[i].is_nan())),
                    (Series::Bool(x), Series::Bool(y)) => x[i] != y[i],
                    _ => a.value_str(i) != b.value_str(i),
                })
                .collect();
            columns.insert(name.clone(), Series::Bool(Arc::new(diff)));
        }
        SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(shared, columns, n)),
        }
    }

    #[sera_doc(
        name = "SeraDFrame.factorize",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Encodes a column as (integer codes, unique values).",
        fr = "Encode une colonne en (codes entiers, valeurs uniques)."
    )]
    fn factorize(&self, col: &str) -> PyResult<(Vec<i64>, Vec<String>)> {
        let series = self.inner.get(col)?;
        let owned;
        let vals: &[Arc<str>] = match series.as_str_slice() {
            Some(v) => v,
            None => {
                owned = series.to_str_vec().into_iter().map(Arc::from).collect::<Vec<_>>();
                &owned
            }
        };
        let mut code_of: HashMap<Arc<str>, i64, FxBuildHasher> = HashMap::default();
        let mut uniques: Vec<String> = Vec::new();
        let codes: Vec<i64> = vals
            .iter()
            .map(|v| match code_of.get(v) {
                Some(&c) => c,
                None => {
                    let c = uniques.len() as i64;
                    uniques.push(v.to_string());
                    code_of.insert(v.clone(), c);
                    c
                }
            })
            .collect();
        Ok((codes, uniques))
    }

    #[sera_doc(
        name = "SeraDFrame.crosstab",
        category = "data_method", file = "canvas/dframe.md", en = "Frequency cross-tabulation of two columns.", fr = "Tableau croise de frequences de deux colonnes.")]
    fn crosstab(&self, row_col: &str, col_col: &str) -> PyResult<SeraDFrame_> {
        let rows = self.inner.get(row_col)?.to_str_vec();
        let cols = self.inner.get(col_col)?.to_str_vec();
        let mut row_uniques: Vec<String> = Vec::new();
        let mut row_index: HashMap<String, usize, FxBuildHasher> = HashMap::default();
        let mut col_uniques: Vec<String> = Vec::new();
        let mut col_index: HashMap<String, usize, FxBuildHasher> = HashMap::default();
        for r in &rows {
            if !row_index.contains_key(r) {
                row_index.insert(r.clone(), row_uniques.len());
                row_uniques.push(r.clone());
            }
        }
        for c in &cols {
            if !col_index.contains_key(c) {
                col_index.insert(c.clone(), col_uniques.len());
                col_uniques.push(c.clone());
            }
        }
        let ncols = col_uniques.len();
        let mut counts = vec![0.0; row_uniques.len() * ncols];
        for (r, c) in rows.iter().zip(cols.iter()) {
            let ri = row_index[r];
            let ci = col_index[c];
            counts[ri * ncols + ci] += 1.0;
        }
        let mut order = vec![row_col.to_string()];
        order.extend(col_uniques.iter().cloned());
        let mut columns = HashMap::new();
        columns.insert(row_col.to_string(), super::str_series(row_uniques.clone()));
        for (ci, cname) in col_uniques.iter().enumerate() {
            let col_vals: Vec<f64> = (0..row_uniques.len()).map(|ri| counts[ri * ncols + ci]).collect();
            columns.insert(cname.clone(), Series::Num(Arc::new(col_vals)));
        }
        let n = row_uniques.len();
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, columns, n)),
        })
    }

    #[pyo3(signature = (index, columns, values, agg = "mean"))]
    #[sera_doc(
        name = "SeraDFrame.pivot",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Reshapes long data into wide form: unique `index` values become rows, unique `columns` values become new columns, cells hold `values` aggregated by `agg` ('mean', 'sum', 'count', 'min', or 'max'). The inverse of melt()/unpivot().",
        fr = "Transforme des donnees longues en format large : les valeurs uniques de `index` deviennent des lignes, celles de `columns` deviennent de nouvelles colonnes, les cellules contiennent `values` agregees par `agg` ('mean', 'sum', 'count', 'min' ou 'max'). L'inverse de melt()/unpivot()."
    )]
    fn pivot(&self, index: &str, columns: &str, values: &str, agg: &str) -> PyResult<SeraDFrame_> {
        let idx_vals = self.inner.get(index)?.to_str_vec();
        let col_vals = self.inner.get(columns)?.to_str_vec();
        let val_vals = self.inner.get(values)?.to_f64_vec();
        let mut row_uniques: Vec<String> = Vec::new();
        let mut row_index: HashMap<String, usize, FxBuildHasher> = HashMap::default();
        let mut col_uniques: Vec<String> = Vec::new();
        let mut col_index: HashMap<String, usize, FxBuildHasher> = HashMap::default();
        for r in &idx_vals {
            if !row_index.contains_key(r) {
                row_index.insert(r.clone(), row_uniques.len());
                row_uniques.push(r.clone());
            }
        }
        for c in &col_vals {
            if !col_index.contains_key(c) {
                col_index.insert(c.clone(), col_uniques.len());
                col_uniques.push(c.clone());
            }
        }
        let ncols = col_uniques.len();
        let mut sums = vec![0.0; row_uniques.len() * ncols];
        let mut counts = vec![0.0; row_uniques.len() * ncols];
        let mut mins = vec![f64::INFINITY; row_uniques.len() * ncols];
        let mut maxs = vec![f64::NEG_INFINITY; row_uniques.len() * ncols];
        for ((r, c), v) in idx_vals.iter().zip(col_vals.iter()).zip(val_vals.iter()) {
            let ri = row_index[r];
            let ci = col_index[c];
            let cell = ri * ncols + ci;
            sums[cell] += v;
            counts[cell] += 1.0;
            if *v < mins[cell] {
                mins[cell] = *v;
            }
            if *v > maxs[cell] {
                maxs[cell] = *v;
            }
        }
        let mut order = vec![index.to_string()];
        order.extend(col_uniques.iter().cloned());
        let mut out_columns = HashMap::new();
        out_columns.insert(index.to_string(), super::str_series(row_uniques.clone()));
        for (ci, cname) in col_uniques.iter().enumerate() {
            let col_out: Vec<f64> = (0..row_uniques.len())
                .map(|ri| {
                    let cell = ri * ncols + ci;
                    let n = counts[cell];
                    if n == 0.0 {
                        return f64::NAN;
                    }
                    match agg {
                        "sum" => sums[cell],
                        "count" => n,
                        "min" => mins[cell],
                        "max" => maxs[cell],
                        _ => sums[cell] / n,
                    }
                })
                .collect();
            out_columns.insert(cname.clone(), Series::Num(Arc::new(col_out)));
        }
        let n = row_uniques.len();
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, out_columns, n)),
        })
    }
}
