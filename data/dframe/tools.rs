use super::series::Series;
use super::{SeraDFrame, SeraDFrame_, SeraDFrameGroupBy};
use crate::sera_doc_impl;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;
use std::sync::Arc;

#[sera_doc_impl]
#[pymethods]
impl SeraDFrame_ {
    #[pyo3(name = "sort", signature = (cols, ascending = true))]
    fn sort_alias(&self, cols: super::groupby::GroupKeys, ascending: bool) -> PyResult<SeraDFrame_> {
        self.sort_values(cols, ascending)
    }

    #[pyo3(name = "group_by")]
    fn group_by_alias(&self, keys: super::groupby::GroupKeys) -> SeraDFrameGroupBy {
        self.groupby(keys)
    }

    #[pyo3(name = "dedupe", signature = (subset = None))]
    fn dedupe_alias(&self, subset: Option<Vec<String>>) -> PyResult<SeraDFrame_> {
        self.drop_duplicates(subset)
    }

    #[pyo3(name = "counts")]
    fn counts_alias(&self, name: &str) -> PyResult<SeraDFrame_> {
        self.value_counts(name)
    }

    #[pyo3(name = "where_eq")]
    fn where_eq_alias(&self, name: &str, value: super::series::PyCell) -> PyResult<SeraDFrame_> {
        self.filter_eq(name, value)
    }

    #[pyo3(name = "join", signature = (other, on, how = "inner"))]
    fn join_alias(&self, other: &SeraDFrame_, on: super::groupby::GroupKeys, how: &str) -> PyResult<SeraDFrame_> {
        self.merge(other, on, how)
    }

    #[pyo3(name = "unpivot")]
    fn unpivot_alias(&self, id_vars: Vec<String>, value_vars: Vec<String>) -> PyResult<SeraDFrame_> {
        self.melt(id_vars, value_vars)
    }

    #[pyo3(name = "is_null")]
    fn is_null_alias(&self) -> SeraDFrame_ {
        self.isna()
    }

    #[pyo3(name = "is_not_null")]
    fn is_not_null_alias(&self) -> SeraDFrame_ {
        self.notna()
    }

    #[pyo3(name = "isnull")]
    fn isnull_alias(&self) -> SeraDFrame_ {
        self.isna()
    }

    #[pyo3(name = "notnull")]
    fn notnull_alias(&self) -> SeraDFrame_ {
        self.notna()
    }

    #[pyo3(name = "kurtosis")]
    fn kurtosis_alias(&self, col: &str) -> PyResult<f64> {
        self.kurt(col)
    }

    #[pyo3(name = "aggregate")]
    fn aggregate_alias(&self, spec: &Bound<'_, PyDict>) -> PyResult<SeraDFrame_> {
        self.agg(spec)
    }

    #[pyo3(name = "T")]
    fn t_alias(&self) -> PyResult<SeraDFrame_> {
        self.transpose(None)
    }

    #[pyo3(name = "is_in")]
    fn is_in_alias(&self, col: &str, values: Vec<String>) -> PyResult<Vec<bool>> {
        self.isin(col, values)
    }

    #[pyo3(name = "is_duplicated", signature = (subset = None))]
    fn is_duplicated_alias(&self, subset: Option<Vec<String>>) -> PyResult<Vec<bool>> {
        self.duplicated(subset)
    }

    #[pyo3(name = "top_k")]
    fn top_k_alias(&self, col: &str, n: usize) -> PyResult<SeraDFrame_> {
        self.nlargest(col, n)
    }

    #[pyo3(name = "bottom_k")]
    fn bottom_k_alias(&self, col: &str, n: usize) -> PyResult<SeraDFrame_> {
        self.nsmallest(col, n)
    }

    #[pyo3(name = "to_dicts")]
    fn to_dicts_alias(&self, py: Python<'_>) -> Vec<PyObject> {
        self.to_records(py)
    }

    #[pyo3(name = "estimated_size")]
    fn estimated_size_alias(&self) -> HashMap<String, u64> {
        self.memory_usage()
    }

    #[sera_doc(
        name = "SeraDFrame.pipe",
        category = "data_method", file = "canvas/dframe.md", en = "Passes self into a Python function, chainable.", fr = "Passe self dans une fonction Python, chainable.")]
    fn pipe(&self, func: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        func.call1((self.clone(),)).map(|r| r.into())
    }

    #[sera_doc(
        name = "SeraDFrame.clip",
        category = "data_method", file = "canvas/dframe.md", en = "Clamps a numeric column between two bounds.", fr = "Borne une colonne numerique entre deux limites.")]
    fn clip(&self, col: &str, lo: f64, hi: f64) -> PyResult<SeraDFrame_> {
        let vals = self.inner.get(col)?.to_f64_vec();
        let clipped: Vec<f64> = vals.iter().map(|v| v.clamp(lo, hi)).collect();
        let mut columns = self.inner.columns.clone();
        columns.insert(col.to_string(), Series::Num(Arc::new(clipped)));
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(self.inner.order.clone(), columns, self.inner.nrows)),
        })
    }
}
