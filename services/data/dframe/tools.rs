use super::series::Series;
use super::{SeraDFrame, SeraDFrame_, SeraDFrameGroupBy};
use crate::sera_doc_impl;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[sera_doc_impl]
#[pymethods]
impl SeraDFrame_ {
    #[pyo3(name = "group_by")]
    fn group_by_alias(&self, keys: super::groupby::GroupKeys) -> SeraDFrameGroupBy {
        self.groupby(keys)
    }

    #[pyo3(name = "kurtosis")]
    fn kurtosis_alias(&self, col: &str) -> PyResult<f64> {
        self.kurt(col)
    }

    #[pyo3(name = "is_in")]
    fn is_in_alias(&self, col: &str, values: Vec<String>) -> PyResult<Vec<bool>> {
        self.isin(col, values)
    }

    #[pyo3(name = "is_duplicated", signature = (subset = None))]
    fn is_duplicated_alias(&self, subset: Option<Vec<String>>) -> PyResult<Vec<bool>> {
        self.duplicated(subset)
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
