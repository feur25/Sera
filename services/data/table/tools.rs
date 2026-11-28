use super::Table;
use crate::sera_doc_impl;
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[sera_doc_impl]
#[pymethods]
impl Table {
    fn pipe(&self, func: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        func.call1((self.clone(),)).map(|r| r.into())
    }

    #[sera_doc(
        name = "Table.to_multiline",
        category = "data_method", file = "canvas/table.md", en = "Extracts columns into sp.multiline() inputs.", fr = "Extrait des colonnes vers les entrees de sp.multiline().")]
    fn to_multiline(&self, py: Python<'_>, x_col: &str, y_cols: Vec<String>) -> PyResult<PyObject> {
        let x_labels = self.col_str(x_col)?;
        let mut series = Vec::with_capacity(y_cols.len());
        for col in &y_cols {
            series.push(self.col_f64(col)?);
        }
        let dict = PyDict::new_bound(py);
        dict.set_item("x_labels", x_labels)?;
        dict.set_item("series", series)?;
        dict.set_item("series_names", y_cols)?;
        Ok(dict.into())
    }

    #[pyo3(signature = (x_col, y_col, category_col = None))]
    #[sera_doc(
        name = "Table.to_scatter",
        category = "data_method", file = "canvas/table.md", en = "Extracts columns into sp.scatter() inputs.", fr = "Extrait des colonnes vers les entrees de sp.scatter().")]
    fn to_scatter(&self, py: Python<'_>, x_col: &str, y_col: &str, category_col: Option<&str>) -> PyResult<PyObject> {
        let x = self.col_f64(x_col)?;
        let y = self.col_f64(y_col)?;
        let dict = PyDict::new_bound(py);
        dict.set_item("x", x)?;
        dict.set_item("y", y)?;
        if let Some(cat) = category_col {
            dict.set_item("categories", self.col_str(cat)?)?;
        }
        Ok(dict.into())
    }
}
