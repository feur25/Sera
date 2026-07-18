use super::series::{PyCell, Series};
use super::{str_series, SeraDFrame, SeraDFrame_};
use crate::sera_doc_impl;
use pyo3::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

fn cell_to_string(cell: &PyCell) -> String {
    match cell {
        PyCell::F(f) => {
            if f.fract().abs() < 1e-9 {
                format!("{}", *f as i64)
            } else {
                f.to_string()
            }
        }
        PyCell::B(b) => b.to_string(),
        PyCell::S(s) => s.clone(),
    }
}

impl SeraDFrame_ {
    fn conditional_replace(&self, col: &str, cond_col: &str, other: PyCell, keep_when: bool) -> PyResult<SeraDFrame_> {
        let cond = self
            .inner
            .get(cond_col)?
            .as_bool_slice()
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("cond column must be boolean"))?;
        let series = self.inner.get(col)?;
        let replaced = match series {
            Series::Num(v) => {
                let other_f = match &other {
                    PyCell::F(f) => *f,
                    PyCell::B(b) => if *b { 1.0 } else { 0.0 },
                    PyCell::S(s) => s.parse().unwrap_or(f64::NAN),
                };
                Series::Num(Arc::new(
                    v.par_iter().zip(cond.par_iter()).map(|(x, &c)| if c == keep_when { *x } else { other_f }).collect(),
                ))
            }
            Series::Str(v) => {
                let other_s = cell_to_string(&other);
                Series::Str(Arc::new(
                    v.par_iter()
                        .zip(cond.par_iter())
                        .map(|(s, &c)| if c == keep_when { s.clone() } else { Arc::from(other_s.as_str()) })
                        .collect(),
                ))
            }
            Series::Bool(v) => {
                let other_b = match &other {
                    PyCell::B(b) => *b,
                    PyCell::F(f) => *f != 0.0,
                    PyCell::S(s) => s == "true",
                };
                Series::Bool(Arc::new(
                    v.par_iter().zip(cond.par_iter()).map(|(&x, &c)| if c == keep_when { x } else { other_b }).collect(),
                ))
            }
        };
        let mut columns = self.inner.columns.clone();
        columns.insert(col.to_string(), replaced);
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(self.inner.order.clone(), columns, self.inner.nrows)),
        })
    }
}

#[sera_doc_impl]
#[pymethods]
impl SeraDFrame_ {
    #[sera_doc(
        name = "SeraDFrame.where_col",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Keeps values where a boolean column is true, replaces the rest with a constant.",
        fr = "Garde les valeurs ou une colonne booleenne est vraie, remplace le reste par une constante."
    )]
    fn where_col(&self, col: &str, cond_col: &str, other: PyCell) -> PyResult<SeraDFrame_> {
        self.conditional_replace(col, cond_col, other, true)
    }

    #[sera_doc(
        name = "SeraDFrame.mask_col",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Replaces values where a boolean column is true, keeps the rest.",
        fr = "Remplace les valeurs ou une colonne booleenne est vraie, garde le reste."
    )]
    fn mask_col(&self, col: &str, cond_col: &str, other: PyCell) -> PyResult<SeraDFrame_> {
        self.conditional_replace(col, cond_col, other, false)
    }

    #[sera_doc(
        name = "SeraDFrame.abs",
        category = "data_method", file = "canvas/dframe.md", en = "Absolute value of every numeric column.", fr = "Valeur absolue de chaque colonne numerique.")]
    fn abs(&self) -> SeraDFrame_ {
        let columns: HashMap<String, Series> = self
            .inner
            .order
            .par_iter()
            .map(|name| {
                let s = match &self.inner.columns[name] {
                    Series::Num(v) => Series::Num(Arc::new(v.par_iter().map(|x| x.abs()).collect())),
                    other => other.clone(),
                };
                (name.clone(), s)
            })
            .collect();
        SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(self.inner.order.clone(), columns, self.inner.nrows)),
        }
    }

    #[sera_doc(
        name = "SeraDFrame.round",
        category = "data_method", file = "canvas/dframe.md", en = "Rounds every numeric column to n decimals.", fr = "Arrondit chaque colonne numerique a n decimales.")]
    fn round(&self, decimals: i32) -> SeraDFrame_ {
        let factor = 10f64.powi(decimals);
        let columns: HashMap<String, Series> = self
            .inner
            .order
            .par_iter()
            .map(|name| {
                let s = match &self.inner.columns[name] {
                    Series::Num(v) => Series::Num(Arc::new(v.par_iter().map(|x| (x * factor).round() / factor).collect())),
                    other => other.clone(),
                };
                (name.clone(), s)
            })
            .collect();
        SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(self.inner.order.clone(), columns, self.inner.nrows)),
        }
    }

    #[sera_doc(
        name = "SeraDFrame.replace",
        category = "data_method", file = "canvas/dframe.md", en = "Replaces a scalar value in a column.", fr = "Remplace une valeur scalaire dans une colonne.")]
    fn replace(&self, col: &str, old: PyCell, new: PyCell) -> PyResult<SeraDFrame_> {
        let series = self.inner.get(col)?;
        let old_str = cell_to_string(&old);
        let replaced = match series {
            Series::Num(v) => {
                let old_f = old_str.parse::<f64>().unwrap_or(f64::NAN);
                let new_f = match &new {
                    PyCell::F(f) => *f,
                    PyCell::B(b) => if *b { 1.0 } else { 0.0 },
                    PyCell::S(s) => s.parse().unwrap_or(f64::NAN),
                };
                Series::Num(Arc::new(v.par_iter().map(|x| if *x == old_f { new_f } else { *x }).collect()))
            }
            Series::Str(v) => {
                let new_str = cell_to_string(&new);
                Series::Str(Arc::new(
                    v.par_iter()
                        .map(|s| if s.as_ref() == old_str.as_str() { Arc::from(new_str.as_str()) } else { s.clone() })
                        .collect(),
                ))
            }
            Series::Bool(v) => {
                let old_b = old_str == "true";
                let new_b = match &new {
                    PyCell::B(b) => *b,
                    PyCell::F(f) => *f != 0.0,
                    PyCell::S(s) => s == "true",
                };
                Series::Bool(Arc::new(v.par_iter().map(|b| if *b == old_b { new_b } else { *b }).collect()))
            }
        };
        let mut columns = self.inner.columns.clone();
        columns.insert(col.to_string(), replaced);
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(self.inner.order.clone(), columns, self.inner.nrows)),
        })
    }

    #[sera_doc(
        name = "SeraDFrame.map_values",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Substitutes column values via a dict mapping.",
        fr = "Substitue les valeurs d'une colonne via un mapping dict.",
        aliases("map")
    )]
    pub(crate) fn map_values(&self, col: &str, mapping: HashMap<String, String>) -> PyResult<SeraDFrame_> {
        let series = self.inner.get(col)?;
        let strs = series.to_str_vec();
        let mapped: Vec<String> = strs.into_par_iter().map(|s| mapping.get(&s).cloned().unwrap_or(s)).collect();
        let mut columns = self.inner.columns.clone();
        columns.insert(col.to_string(), str_series(mapped));
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(self.inner.order.clone(), columns, self.inner.nrows)),
        })
    }
}
