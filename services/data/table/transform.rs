use super::{Cell, Table};
use crate::sera_doc_impl;
use pyo3::prelude::*;

#[sera_doc_impl]
#[pymethods]
impl Table {
    #[pyo3(signature = (name, op, left, right))]
    #[sera_doc(
        name = "Table.with_column",
        category = "data_method", file = "canvas/table.md", en = "Adds a computed column (add/sub/mul/div).", fr = "Ajoute une colonne calculee (add/sub/mul/div).")]
    fn with_column(&self, name: &str, op: &str, left: &str, right: &Bound<'_, PyAny>) -> PyResult<Table> {
        let apply: fn(f64, f64) -> f64 = match op {
            "add" => |a, b| a + b,
            "sub" => |a, b| a - b,
            "mul" => |a, b| a * b,
            "div" => |a, b| if b != 0.0 { a / b } else { 0.0 },
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "Table.with_column: unknown op '{op}', expected one of add/sub/mul/div"
                )))
            }
        };
        let lhs = self.col_f64(left)?;
        let rhs: Vec<f64> = if let Ok(col_name) = right.extract::<String>() {
            if let Ok(col) = self.col_f64(&col_name) {
                col
            } else {
                let scalar: f64 = right.extract()?;
                vec![scalar; self.nrows]
            }
        } else {
            let scalar: f64 = right.extract()?;
            vec![scalar; self.nrows]
        };
        let computed: Vec<Cell> = lhs
            .iter()
            .zip(rhs.iter())
            .map(|(a, b)| Cell::Num(apply(*a, *b)))
            .collect();
        let mut columns = self.columns.clone();
        let mut order = self.order.clone();
        if !columns.contains_key(name) {
            order.push(name.to_string());
        }
        columns.insert(name.to_string(), computed);
        Ok(Table::from_parts(order, columns, self.nrows))
    }

    #[sera_doc(
        name = "Table.rolling_mean",
        category = "data_method", file = "canvas/table.md", en = "Trailing moving average.", fr = "Moyenne mobile arriere.")]
    fn rolling_mean(&self, col: &str, window: usize) -> PyResult<Table> {
        let vals = self.col_f64(col)?;
        let w = window.max(1);
        let out: Vec<Cell> = (0..vals.len())
            .map(|i| {
                let start = i.saturating_sub(w - 1);
                let slice = &vals[start..=i];
                Cell::Num(slice.iter().sum::<f64>() / slice.len() as f64)
            })
            .collect();
        let mut columns = self.columns.clone();
        let mut order = self.order.clone();
        let name = format!("{}_rolling{}", col, w);
        if !columns.contains_key(&name) {
            order.push(name.clone());
        }
        columns.insert(name, out);
        Ok(Table::from_parts(order, columns, self.nrows))
    }

    #[sera_doc(
        name = "Table.cumsum",
        category = "data_method", file = "canvas/table.md", en = "Running total.", fr = "Total cumule.")]
    fn cumsum(&self, col: &str) -> PyResult<Table> {
        let vals = self.col_f64(col)?;
        let mut acc = 0.0;
        let out: Vec<Cell> = vals
            .iter()
            .map(|v| {
                acc += v;
                Cell::Num(acc)
            })
            .collect();
        let mut columns = self.columns.clone();
        let mut order = self.order.clone();
        let name = format!("{}_cumsum", col);
        if !columns.contains_key(&name) {
            order.push(name.clone());
        }
        columns.insert(name, out);
        Ok(Table::from_parts(order, columns, self.nrows))
    }

    #[sera_doc(
        name = "Table.pct_change",
        category = "data_method", file = "canvas/table.md", en = "Row-over-row percent change.", fr = "Variation en pourcentage ligne a ligne.")]
    fn pct_change(&self, col: &str) -> PyResult<Table> {
        let vals = self.col_f64(col)?;
        let out: Vec<Cell> = (0..vals.len())
            .map(|i| {
                if i == 0 || vals[i - 1] == 0.0 {
                    Cell::Num(0.0)
                } else {
                    Cell::Num((vals[i] - vals[i - 1]) / vals[i - 1])
                }
            })
            .collect();
        let mut columns = self.columns.clone();
        let mut order = self.order.clone();
        let name = format!("{}_pct_change", col);
        if !columns.contains_key(&name) {
            order.push(name.clone());
        }
        columns.insert(name, out);
        Ok(Table::from_parts(order, columns, self.nrows))
    }

    #[pyo3(signature = (col, desc = false))]
    #[sera_doc(
        name = "Table.rank",
        category = "data_method", file = "canvas/table.md", en = "1-based rank.", fr = "Rang (base 1).")]
    fn rank(&self, col: &str, desc: bool) -> PyResult<Table> {
        let vals = self.col_f64(col)?;
        let mut idx: Vec<usize> = (0..vals.len()).collect();
        idx.sort_by(|&a, &b| vals[a].partial_cmp(&vals[b]).unwrap());
        if desc {
            idx.reverse();
        }
        let mut ranks = vec![0.0f64; vals.len()];
        for (r, &i) in idx.iter().enumerate() {
            ranks[i] = (r + 1) as f64;
        }
        let mut columns = self.columns.clone();
        let mut order = self.order.clone();
        let name = format!("{}_rank", col);
        if !columns.contains_key(&name) {
            order.push(name.clone());
        }
        columns.insert(name, ranks.into_iter().map(Cell::Num).collect());
        Ok(Table::from_parts(order, columns, self.nrows))
    }

    #[sera_doc(
        name = "Table.zscore",
        category = "data_method", file = "canvas/table.md", en = "(x - mean) / std.", fr = "(x - moyenne) / ecart-type.")]
    fn zscore(&self, col: &str) -> PyResult<Table> {
        let vals = self.col_f64(col)?;
        let n = vals.len().max(1) as f64;
        let mean = vals.iter().sum::<f64>() / n;
        let variance = vals.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / n;
        let std = variance.sqrt().max(1e-12);
        let scored: Vec<Cell> = vals.iter().map(|v| Cell::Num((v - mean) / std)).collect();
        let mut columns = self.columns.clone();
        let mut order = self.order.clone();
        let name = format!("{}_zscore", col);
        if !columns.contains_key(&name) {
            order.push(name.clone());
        }
        columns.insert(name, scored);
        Ok(Table::from_parts(order, columns, self.nrows))
    }
}
