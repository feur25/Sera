mod attrs;
mod bigdata;
mod builder;
mod combine;
mod construct;
mod datetime;
mod derive;
mod elementwise;
mod filter;
mod groupby;
mod mask;
mod query;
mod reduce;
mod relational;
mod reshape;
mod rolling;
mod series;
mod stats;
mod strings;
mod tools;

pub use builder::DFrameBuilder;
pub use groupby::SeraDFrameGroupBy;
pub use series::Series;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct SeraDFrame {
    pub(crate) order: Vec<String>,
    pub(crate) columns: HashMap<String, Series>,
    pub(crate) nrows: usize,
}

impl SeraDFrame {
    pub(crate) fn from_parts(order: Vec<String>, columns: HashMap<String, Series>, nrows: usize) -> Self {
        SeraDFrame { order, columns, nrows }
    }

    pub(crate) fn get(&self, name: &str) -> PyResult<&Series> {
        self.columns
            .get(name)
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err(format!("no column '{}'", name)))
    }

    pub(crate) fn numeric_columns(&self) -> Vec<String> {
        self.order
            .iter()
            .filter(|c| matches!(self.columns.get(*c), Some(Series::Num(_))))
            .cloned()
            .collect()
    }

    pub(crate) fn row_take(&self, idx: &[usize]) -> SeraDFrame {
        let columns: HashMap<String, Series> = self
            .order
            .par_iter()
            .map(|name| (name.clone(), self.columns[name].take(idx)))
            .collect();
        SeraDFrame::from_parts(self.order.clone(), columns, idx.len())
    }

    pub(crate) fn dict_to_frame(dict: &Bound<'_, PyDict>) -> PyResult<SeraDFrame> {
        let mut order = Vec::new();
        let mut columns = HashMap::new();
        let mut nrows = 0usize;
        for (k, v) in dict.iter() {
            let key: String = k.extract()?;
            let items: Vec<Bound<'_, PyAny>> = v.extract()?;
            nrows = nrows.max(items.len());
            let series = series::column_from_pyobjects(items);
            order.push(key.clone());
            columns.insert(key, series);
        }
        Ok(SeraDFrame::from_parts(order, columns, nrows))
    }
}

#[pyclass(name = "SeraDFrame")]
#[derive(Clone)]
pub struct SeraDFrame_ {
    pub(crate) inner: Arc<SeraDFrame>,
}

inventory::submit! {
    crate::doc_registry::FnDoc {
        name: "SeraDFrame",
        category: "data_method",
        file: "canvas/dframe.md",
        en: "Columnar Rust-native dataframe built from a dict of columns.",
        fr: "Dataframe colonnaire natif Rust construit depuis un dict de colonnes.",
        params: &[],
        aliases: &[],
    }
}

fn str_arc(s: String) -> Arc<str> {
    Arc::from(s)
}

pub(crate) fn str_series(vals: Vec<String>) -> Series {
    Series::Str(Arc::new(vals.into_iter().map(str_arc).collect()))
}
