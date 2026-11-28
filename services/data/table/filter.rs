use super::{Cell, Table};
use crate::sera_doc_impl;
use pyo3::prelude::*;
use std::collections::HashMap;

#[sera_doc_impl]
#[pymethods]
impl Table {
    #[sera_doc(
        name = "Table.filter_eq",
        category = "data_method", file = "canvas/table.md", en = "Rows where a column equals a value.", fr = "Lignes ou une colonne vaut une valeur.")]
    fn filter_eq(&self, name: &str, value: Cell) -> PyResult<Table> {
        let col = self
            .columns
            .get(name)
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err(format!("no column '{}'", name)))?;
        let target = value.as_str();
        let mask: Vec<bool> = col.iter().map(|c| c.as_str() == target).collect();
        Ok(self.row_mask(&mask))
    }

    #[sera_doc(
        name = "Table.filter_gt",
        category = "data_method", file = "canvas/table.md", en = "Rows above a numeric value.", fr = "Lignes au-dessus d'une valeur numerique.")]
    fn filter_gt(&self, name: &str, value: f64) -> PyResult<Table> {
        let vals = self.col_f64(name)?;
        let mask: Vec<bool> = vals.iter().map(|v| *v > value).collect();
        Ok(self.row_mask(&mask))
    }

    fn filter_lt(&self, name: &str, value: f64) -> PyResult<Table> {
        let vals = self.col_f64(name)?;
        let mask: Vec<bool> = vals.iter().map(|v| *v < value).collect();
        Ok(self.row_mask(&mask))
    }

    fn filter_ge(&self, name: &str, value: f64) -> PyResult<Table> {
        let vals = self.col_f64(name)?;
        let mask: Vec<bool> = vals.iter().map(|v| *v >= value).collect();
        Ok(self.row_mask(&mask))
    }

    fn filter_le(&self, name: &str, value: f64) -> PyResult<Table> {
        let vals = self.col_f64(name)?;
        let mask: Vec<bool> = vals.iter().map(|v| *v <= value).collect();
        Ok(self.row_mask(&mask))
    }

    fn filter_in(&self, name: &str, values: Vec<String>) -> PyResult<Table> {
        let col = self.col_str(name)?;
        let mask: Vec<bool> = col.iter().map(|v| values.contains(v)).collect();
        Ok(self.row_mask(&mask))
    }

    #[pyo3(signature = (name, desc = false))]
    #[sera_doc(
        name = "Table.sort_by",
        category = "data_method", file = "canvas/table.md", en = "Sorted copy by a column.", fr = "Copie triee selon une colonne.", aliases("sort"))]
    pub(crate) fn sort_by(&self, name: &str, desc: bool) -> PyResult<Table> {
        let vals = self.col_f64(name).ok();
        let mut idx: Vec<usize> = (0..self.nrows).collect();
        if let Some(nums) = vals {
            idx.sort_by(|&a, &b| nums[a].partial_cmp(&nums[b]).unwrap());
        } else {
            let strs = self.col_str(name)?;
            idx.sort_by(|&a, &b| strs[a].cmp(&strs[b]));
        }
        if desc {
            idx.reverse();
        }
        let mut columns = HashMap::new();
        for cname in &self.order {
            let src = &self.columns[cname];
            let reordered: Vec<Cell> = idx.iter().map(|&i| src[i].clone()).collect();
            columns.insert(cname.clone(), reordered);
        }
        Ok(Table::from_parts(self.order.clone(), columns, self.nrows))
    }

    #[pyo3(signature = (col, n, desc = true))]
    #[sera_doc(
        name = "Table.top_n",
        category = "data_method", file = "canvas/table.md", en = "Shortcut for sort_by(desc).head(n).", fr = "Raccourci pour sort_by(desc).head(n).")]
    fn top_n(&self, col: &str, n: usize, desc: bool) -> PyResult<Table> {
        self.sort_by(col, desc).map(|t| t.head(n))
    }
}
