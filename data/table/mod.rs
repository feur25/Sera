mod agg;
mod construct;
mod filter;
mod tools;
mod transform;

use pyo3::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub enum Cell {
    Num(f64),
    Str(String),
    Bool(bool),
}

impl Cell {
    pub(crate) fn as_f64(&self) -> Option<f64> {
        match self {
            Cell::Num(n) => Some(*n),
            Cell::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
            Cell::Str(s) => s.parse().ok(),
        }
    }

    pub(crate) fn as_str(&self) -> String {
        match self {
            Cell::Num(n) => {
                if n.fract().abs() < 1e-9 {
                    format!("{}", *n as i64)
                } else {
                    n.to_string()
                }
            }
            Cell::Str(s) => s.clone(),
            Cell::Bool(b) => b.to_string(),
        }
    }

    pub(crate) fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Cell::Num(n) => n.into_py(py),
            Cell::Str(s) => s.into_py(py),
            Cell::Bool(b) => b.into_py(py),
        }
    }
}

impl<'py> FromPyObject<'py> for Cell {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(b) = ob.extract::<bool>() {
            return Ok(Cell::Bool(b));
        }
        if let Ok(f) = ob.extract::<f64>() {
            return Ok(Cell::Num(f));
        }
        if let Ok(s) = ob.extract::<String>() {
            return Ok(Cell::Str(s));
        }
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Table cells must be int, float, str or bool",
        ))
    }
}

pub(crate) fn agg_values(vals: &[f64], agg: &str) -> f64 {
    if vals.is_empty() {
        return 0.0;
    }
    match agg {
        "mean" | "avg" => vals.iter().sum::<f64>() / vals.len() as f64,
        "count" => vals.len() as f64,
        "min" => vals.iter().copied().fold(f64::INFINITY, f64::min),
        "max" => vals.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        "median" => {
            let mut sorted = vals.to_vec();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mid = sorted.len() / 2;
            if sorted.len() % 2 == 0 {
                (sorted[mid - 1] + sorted[mid]) / 2.0
            } else {
                sorted[mid]
            }
        }
        _ => vals.iter().sum::<f64>(),
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Table {
    pub(crate) order: Vec<String>,
    pub(crate) columns: HashMap<String, Vec<Cell>>,
    pub(crate) nrows: usize,
}

inventory::submit! {
    crate::doc_registry::FnDoc {
        name: "Table",
        category: "data_method",
        file: "canvas/table.md",
        en: "Small columnar data-shaping utility built from a dict of columns.",
        fr: "Petit outil de mise en forme de donnees en colonnes construit depuis un dict.",
        params: &[],
        aliases: &[],
    }
}

impl Table {
    pub(crate) fn from_parts(order: Vec<String>, columns: HashMap<String, Vec<Cell>>, nrows: usize) -> Self {
        Table { order, columns, nrows }
    }

    pub(crate) fn col_f64(&self, name: &str) -> PyResult<Vec<f64>> {
        self.columns
            .get(name)
            .map(|c| c.iter().map(|v| v.as_f64().unwrap_or(0.0)).collect())
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err(format!("no column '{}'", name)))
    }

    pub(crate) fn col_str(&self, name: &str) -> PyResult<Vec<String>> {
        self.columns
            .get(name)
            .map(|c| c.iter().map(|v| v.as_str()).collect())
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err(format!("no column '{}'", name)))
    }

    pub(crate) fn row_mask(&self, mask: &[bool]) -> Table {
        let mut columns = HashMap::new();
        for name in &self.order {
            let src = &self.columns[name];
            let filtered: Vec<Cell> = src
                .iter()
                .zip(mask.iter())
                .filter(|(_, keep)| **keep)
                .map(|(c, _)| c.clone())
                .collect();
            columns.insert(name.clone(), filtered);
        }
        let nrows = mask.iter().filter(|k| **k).count();
        Table::from_parts(self.order.clone(), columns, nrows)
    }
}

#[pymethods]
impl Table {
    fn __len__(&self) -> usize {
        self.nrows
    }

    fn __repr__(&self) -> String {
        format!("Table({} rows x {} cols)", self.nrows, self.order.len())
    }
}
