use super::{Cell, Table};
use crate::sera_doc_impl;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

#[sera_doc_impl]
#[pymethods]
impl Table {
    #[new]
    fn new(columns: &Bound<'_, PyDict>) -> PyResult<Self> {
        let mut order = Vec::new();
        let mut map = HashMap::new();
        let mut nrows = 0usize;
        for (k, v) in columns.iter() {
            let key: String = k.extract()?;
            let vals: Vec<Cell> = v.extract()?;
            nrows = nrows.max(vals.len());
            order.push(key.clone());
            map.insert(key, vals);
        }
        Ok(Table::from_parts(order, map, nrows))
    }

    #[staticmethod]
    #[sera_doc(
        name = "Table.from_csv",
        category = "data_method", file = "canvas/table.md", en = "Reads a CSV into a Table.", fr = "Lit un CSV en Table.")]
    fn from_csv(path: &str) -> PyResult<Table> {
        let csv = crate::data::loader::CsvData::load(path)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        let mut order = Vec::new();
        let mut map = HashMap::new();
        for header in &csv.headers {
            let is_numeric = csv.numeric_columns.contains(header);
            let cells: Vec<Cell> = csv
                .rows
                .iter()
                .map(|row| {
                    let raw = row.get(header).cloned().unwrap_or_default();
                    if is_numeric {
                        Cell::Num(raw.parse().unwrap_or(f64::NAN))
                    } else {
                        Cell::Str(raw)
                    }
                })
                .collect();
            order.push(header.clone());
            map.insert(header.clone(), cells);
        }
        Ok(Table::from_parts(order, map, csv.len()))
    }

    #[sera_doc(
        name = "Table.columns",
        category = "data_method", file = "canvas/table.md", en = "Column names, in original order.", fr = "Noms des colonnes, dans l'ordre d'origine.")]
    fn columns(&self) -> Vec<String> {
        self.order.clone()
    }

    #[getter]
    fn nrows(&self) -> usize {
        self.nrows
    }

    fn column(&self, py: Python<'_>, name: &str) -> PyResult<Vec<PyObject>> {
        self.columns
            .get(name)
            .map(|c| c.iter().cloned().map(|v| v.into_py(py)).collect())
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err(format!("no column '{}'", name)))
    }

    fn column_f64(&self, name: &str) -> PyResult<Vec<f64>> {
        self.col_f64(name)
    }

    fn column_str(&self, name: &str) -> PyResult<Vec<String>> {
        self.col_str(name)
    }

    #[sera_doc(
        name = "Table.select",
        category = "data_method", file = "canvas/table.md", en = "A subset of columns.", fr = "Un sous-ensemble de colonnes.")]
    fn select(&self, names: Vec<String>) -> PyResult<Table> {
        let mut columns = HashMap::new();
        for name in &names {
            let col = self
                .columns
                .get(name)
                .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err(format!("no column '{}'", name)))?;
            columns.insert(name.clone(), col.clone());
        }
        Ok(Table::from_parts(names, columns, self.nrows))
    }

    #[sera_doc(
        name = "Table.head",
        category = "data_method", file = "canvas/table.md", en = "First n rows.", fr = "Les n premieres lignes.")]
    pub(crate) fn head(&self, n: usize) -> Table {
        let n = n.min(self.nrows);
        let mut columns = HashMap::new();
        for name in &self.order {
            columns.insert(name.clone(), self.columns[name][..n].to_vec());
        }
        Table::from_parts(self.order.clone(), columns, n)
    }

    fn to_records(&self, py: Python<'_>) -> Vec<PyObject> {
        (0..self.nrows)
            .map(|i| {
                let dict = PyDict::new_bound(py);
                for name in &self.order {
                    let _ = dict.set_item(name, self.columns[name][i].clone().into_py(py));
                }
                dict.into()
            })
            .collect()
    }
}
