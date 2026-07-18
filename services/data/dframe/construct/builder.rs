use super::super::series::column_from_pyobjects;
use super::super::{SeraDFrame, SeraDFrame_, Series};
use pyo3::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[pyclass]
pub struct DFrameBuilder {
    order: Vec<String>,
    columns: HashMap<String, Series>,
}

inventory::submit! {
    crate::doc_registry::FnDoc {
        name: "DFrameBuilder",
        category: "data_method",
        file: "canvas/dframe.md",
        en: "Chainable column-by-column dataframe builder.",
        fr: "Constructeur de dataframe chainable colonne par colonne.",
        params: &[],
        aliases: &[],
    }
}

#[pymethods]
impl DFrameBuilder {
    #[new]
    fn new() -> Self {
        DFrameBuilder {
            order: Vec::new(),
            columns: HashMap::new(),
        }
    }

    fn column<'py>(mut slf: PyRefMut<'py, Self>, name: &str, values: &Bound<'py, PyAny>) -> PyResult<PyRefMut<'py, Self>> {
        let items: Vec<Bound<'py, PyAny>> = values.extract()?;
        let series = column_from_pyobjects(items);
        if !slf.columns.contains_key(name) {
            slf.order.push(name.to_string());
        }
        slf.columns.insert(name.to_string(), series);
        Ok(slf)
    }

    fn build(&self) -> SeraDFrame_ {
        let nrows = self.columns.values().map(|c| c.len()).max().unwrap_or(0);
        SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(self.order.clone(), self.columns.clone(), nrows)),
        }
    }
}
