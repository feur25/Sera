use super::{agg_values, Cell, Table};
use crate::sera_doc_impl;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

#[sera_doc_impl]
#[pymethods]
impl Table {
    #[pyo3(signature = (group_col, value_col, agg = "sum"))]
    #[sera_doc(
        name = "Table.groupby_agg",
        category = "data_method", file = "canvas/table.md", en = "Groups by a column, aggregates another.", fr = "Groupe par une colonne, agrege une autre.", aliases("group_by_agg"))]
    pub(crate) fn groupby_agg(&self, group_col: &str, value_col: &str, agg: &str) -> PyResult<Table> {
        let groups = self.col_str(group_col)?;
        let values = self.col_f64(value_col)?;
        let mut order_keys: Vec<String> = Vec::new();
        let mut buckets: HashMap<String, Vec<f64>> = HashMap::new();
        for (g, v) in groups.into_iter().zip(values.into_iter()) {
            if !buckets.contains_key(&g) {
                order_keys.push(g.clone());
            }
            buckets.entry(g).or_default().push(v);
        }
        let group_cells: Vec<Cell> = order_keys.iter().map(|k| Cell::Str(k.clone())).collect();
        let value_cells: Vec<Cell> = order_keys
            .iter()
            .map(|k| Cell::Num(agg_values(&buckets[k], agg)))
            .collect();
        let mut columns = HashMap::new();
        columns.insert(group_col.to_string(), group_cells);
        columns.insert(value_col.to_string(), value_cells);
        let n = order_keys.len();
        Ok(Table::from_parts(
            vec![group_col.to_string(), value_col.to_string()],
            columns,
            n,
        ))
    }

    #[pyo3(signature = (index_col, columns_col, values_col, agg = "sum"))]
    #[sera_doc(
        name = "Table.pivot",
        category = "data_method", file = "canvas/table.md", en = "Reshapes long data to wide.", fr = "Passe du format long au format large.")]
    fn pivot(&self, index_col: &str, columns_col: &str, values_col: &str, agg: &str) -> PyResult<Table> {
        let idx_vals = self.col_str(index_col)?;
        let col_vals = self.col_str(columns_col)?;
        let val_vals = self.col_f64(values_col)?;

        let mut idx_order: Vec<String> = Vec::new();
        let mut col_order: Vec<String> = Vec::new();
        let mut cells: HashMap<(String, String), Vec<f64>> = HashMap::new();

        for i in 0..self.nrows {
            let iv = idx_vals[i].clone();
            let cv = col_vals[i].clone();
            if !idx_order.contains(&iv) {
                idx_order.push(iv.clone());
            }
            if !col_order.contains(&cv) {
                col_order.push(cv.clone());
            }
            cells.entry((iv, cv)).or_default().push(val_vals[i]);
        }

        let mut columns = HashMap::new();
        columns.insert(
            index_col.to_string(),
            idx_order.iter().map(|v| Cell::Str(v.clone())).collect(),
        );
        for series in &col_order {
            let series_vals: Vec<Cell> = idx_order
                .iter()
                .map(|iv| {
                    let key = (iv.clone(), series.clone());
                    let agg_val = cells.get(&key).map(|v| agg_values(v, agg)).unwrap_or(0.0);
                    Cell::Num(agg_val)
                })
                .collect();
            columns.insert(series.clone(), series_vals);
        }

        let mut order = vec![index_col.to_string()];
        order.extend(col_order);
        let n = idx_order.len();
        Ok(Table::from_parts(order, columns, n))
    }

    #[pyo3(signature = (index_col, columns_col, values_col, agg = "sum"))]
    #[sera_doc(
        name = "Table.to_grouped_bar",
        category = "data_method", file = "canvas/table.md", en = "Pivots and flattens into sp.grouped_bar() inputs.", fr = "Pivote et aplatit vers les entrees de sp.grouped_bar().")]
    fn to_grouped_bar(
        &self,
        py: Python<'_>,
        index_col: &str,
        columns_col: &str,
        values_col: &str,
        agg: &str,
    ) -> PyResult<PyObject> {
        let pivoted = self.pivot(index_col, columns_col, values_col, agg)?;
        let category_labels = pivoted.col_str(index_col)?;
        let series_names: Vec<String> = pivoted
            .order
            .iter()
            .filter(|c| c.as_str() != index_col)
            .cloned()
            .collect();
        let n_cats = category_labels.len();
        let mut flat = vec![0.0f64; series_names.len() * n_cats];
        for (si, series) in series_names.iter().enumerate() {
            let vals = pivoted.col_f64(series)?;
            for (ci, v) in vals.into_iter().enumerate() {
                flat[si * n_cats + ci] = v;
            }
        }
        let dict = PyDict::new_bound(py);
        dict.set_item("category_labels", category_labels)?;
        dict.set_item("values", flat)?;
        dict.set_item("series_names", series_names)?;
        Ok(dict.into())
    }

    #[sera_doc(
        name = "Table.describe",
        category = "data_method", file = "canvas/table.md", en = "count/mean/min/max/std per numeric column.", fr = "count/mean/min/max/std par colonne numerique.")]
    fn describe(&self) -> Table {
        let mut names = Vec::new();
        let mut counts = Vec::new();
        let mut means = Vec::new();
        let mut mins = Vec::new();
        let mut maxs = Vec::new();
        let mut stds = Vec::new();
        for col in &self.order {
            let cells = &self.columns[col];
            let is_numeric = cells.iter().all(|c| matches!(c, Cell::Num(_) | Cell::Bool(_)));
            if !is_numeric {
                continue;
            }
            let vals: Vec<f64> = cells.iter().filter_map(|c| c.as_f64()).collect();
            if vals.is_empty() {
                continue;
            }
            let n = vals.len() as f64;
            let mean = vals.iter().sum::<f64>() / n;
            let variance = vals.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / n;
            names.push(col.clone());
            counts.push(vals.len() as f64);
            means.push(mean);
            mins.push(vals.iter().copied().fold(f64::INFINITY, f64::min));
            maxs.push(vals.iter().copied().fold(f64::NEG_INFINITY, f64::max));
            stds.push(variance.sqrt());
        }
        let mut columns = HashMap::new();
        columns.insert("column".to_string(), names.iter().map(|s| Cell::Str(s.clone())).collect());
        columns.insert("count".to_string(), counts.into_iter().map(Cell::Num).collect());
        columns.insert("mean".to_string(), means.into_iter().map(Cell::Num).collect());
        columns.insert("min".to_string(), mins.into_iter().map(Cell::Num).collect());
        columns.insert("max".to_string(), maxs.into_iter().map(Cell::Num).collect());
        columns.insert("std".to_string(), stds.into_iter().map(Cell::Num).collect());
        let n = names.len();
        Table::from_parts(
            vec![
                "column".to_string(),
                "count".to_string(),
                "mean".to_string(),
                "min".to_string(),
                "max".to_string(),
                "std".to_string(),
            ],
            columns,
            n,
        )
    }

    #[sera_doc(
        name = "Table.concat",
        category = "data_method", file = "canvas/table.md", en = "Vertical union of two tables.", fr = "Union verticale de deux tables.")]
    fn concat(&self, other: &Table) -> Table {
        let mut order = self.order.clone();
        for name in &other.order {
            if !order.contains(name) {
                order.push(name.clone());
            }
        }
        let mut columns = HashMap::new();
        for name in &order {
            let self_col = self.columns.get(name);
            let other_col = other.columns.get(name);
            let mut merged = self_col
                .cloned()
                .unwrap_or_else(|| vec![Cell::default_for(other_col); self.nrows]);
            let other_vals = other_col
                .cloned()
                .unwrap_or_else(|| vec![Cell::default_for(self_col); other.nrows]);
            merged.extend(other_vals);
            columns.insert(name.clone(), merged);
        }
        Table::from_parts(order, columns, self.nrows + other.nrows)
    }

    #[pyo3(signature = (other, on, how = "inner"))]
    #[sera_doc(
        name = "Table.join",
        category = "data_method", file = "canvas/table.md", en = "Relational join on a key column.", fr = "Jointure relationnelle sur une colonne cle.")]
    fn join(&self, other: &Table, on: &str, how: &str) -> PyResult<Table> {
        let left_keys = self.col_str(on)?;
        let right_keys = other.col_str(on)?;
        let mut right_index: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, k) in right_keys.iter().enumerate() {
            right_index.entry(k.clone()).or_default().push(i);
        }

        let left_others: Vec<String> = self.order.iter().filter(|c| c.as_str() != on).cloned().collect();
        let mut right_others: Vec<String> = other.order.iter().filter(|c| c.as_str() != on).cloned().collect();
        let mut right_rename: HashMap<String, String> = HashMap::new();
        for name in right_others.iter_mut() {
            if left_others.contains(name) {
                let renamed = format!("right_{}", name);
                right_rename.insert(name.clone(), renamed);
            }
        }

        let mut order = vec![on.to_string()];
        order.extend(left_others.iter().cloned());
        order.extend(right_others.iter().map(|n| right_rename.get(n).cloned().unwrap_or_else(|| n.clone())));

        let mut columns: HashMap<String, Vec<Cell>> = HashMap::new();
        for name in &order {
            columns.insert(name.clone(), Vec::new());
        }

        let mut nrows = 0usize;
        for (li, lk) in left_keys.iter().enumerate() {
            let matches = right_index.get(lk).cloned().unwrap_or_default();
            if matches.is_empty() {
                if how == "left" {
                    columns.get_mut(on).unwrap().push(Cell::Str(lk.clone()));
                    for name in &left_others {
                        columns.get_mut(name).unwrap().push(self.columns[name][li].clone());
                    }
                    for name in &right_others {
                        let out_name = right_rename.get(name).cloned().unwrap_or_else(|| name.clone());
                        let fill = Cell::default_for(other.columns.get(name));
                        columns.get_mut(&out_name).unwrap().push(fill);
                    }
                    nrows += 1;
                }
                continue;
            }
            for ri in matches {
                columns.get_mut(on).unwrap().push(Cell::Str(lk.clone()));
                for name in &left_others {
                    columns.get_mut(name).unwrap().push(self.columns[name][li].clone());
                }
                for name in &right_others {
                    let out_name = right_rename.get(name).cloned().unwrap_or_else(|| name.clone());
                    columns.get_mut(&out_name).unwrap().push(other.columns[name][ri].clone());
                }
                nrows += 1;
            }
        }

        Ok(Table::from_parts(order, columns, nrows))
    }
}
