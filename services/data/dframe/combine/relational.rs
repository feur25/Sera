use super::super::groupby::GroupKeys;
use super::super::series::{column_from_pyobjects, row_hash, rows_equal_cross, ColView, PassThroughBuildHasher, Series};
use super::super::{SeraDFrame, SeraDFrame_};
use crate::sera_doc_impl;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[sera_doc_impl]
#[pymethods]
impl SeraDFrame_ {
    #[sera_doc(
        name = "SeraDFrame.apply",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Calls a Python function per value in a column.",
        fr = "Appelle une fonction Python par valeur d'une colonne."
    )]
    fn apply(&self, name: &str, func: &Bound<'_, PyAny>) -> PyResult<SeraDFrame_> {
        let col = self.inner.get(name)?;
        let out: Vec<Bound<'_, PyAny>> = match col {
            Series::Num(v) => v.iter().map(|x| func.call1((*x,))).collect::<PyResult<Vec<_>>>()?,
            Series::Str(v) => v
                .iter()
                .map(|x| func.call1((x.to_string(),)))
                .collect::<PyResult<Vec<_>>>()?,
            Series::Bool(v) => v.iter().map(|x| func.call1((*x,))).collect::<PyResult<Vec<_>>>()?,
        };
        let series = column_from_pyobjects(out);
        let mut columns = self.inner.columns.clone();
        columns.insert(name.to_string(), series);
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(self.inner.order.clone(), columns, self.inner.nrows)),
        })
    }

    #[sera_doc(
        name = "SeraDFrame.applymap",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Calls a Python function on every cell of every column.",
        fr = "Appelle une fonction Python sur chaque cellule de chaque colonne."
    )]
    fn applymap(&self, func: &Bound<'_, PyAny>) -> PyResult<SeraDFrame_> {
        let mut columns = HashMap::new();
        for name in &self.inner.order {
            let col = &self.inner.columns[name];
            let out: Vec<Bound<'_, PyAny>> = match col {
                Series::Num(v) => v.iter().map(|x| func.call1((*x,))).collect::<PyResult<Vec<_>>>()?,
                Series::Str(v) => v
                    .iter()
                    .map(|x| func.call1((x.to_string(),)))
                    .collect::<PyResult<Vec<_>>>()?,
                Series::Bool(v) => v.iter().map(|x| func.call1((*x,))).collect::<PyResult<Vec<_>>>()?,
            };
            columns.insert(name.clone(), column_from_pyobjects(out));
        }
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(self.inner.order.clone(), columns, self.inner.nrows)),
        })
    }

    #[pyo3(signature = (other, on, how = "inner"))]
    #[sera_doc(
        name = "SeraDFrame.merge",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Relational join with another SeraDFrame on one or more keys.",
        fr = "Jointure relationnelle avec un autre SeraDFrame sur une ou plusieurs cles.",
        aliases("join")
    )]
    pub(crate) fn merge(&self, other: &SeraDFrame_, on: GroupKeys, how: &str) -> PyResult<SeraDFrame_> {
        let on = on.into_vec();
        let left_views: Vec<ColView> = on.iter().map(|n| self.inner.get(n).map(|c| c.as_view())).collect::<PyResult<Vec<_>>>()?;
        let right_views: Vec<ColView> = on.iter().map(|n| other.inner.get(n).map(|c| c.as_view())).collect::<PyResult<Vec<_>>>()?;

        let mut right_index: HashMap<u64, Vec<usize>, PassThroughBuildHasher> = HashMap::default();
        for j in 0..other.inner.nrows {
            let h = row_hash(&right_views, j);
            right_index.entry(h).or_default().push(j);
        }

        let left_others: Vec<String> = self.inner.order.iter().filter(|c| !on.contains(c)).cloned().collect();
        let right_others: Vec<String> = other.inner.order.iter().filter(|c| !on.contains(c)).cloned().collect();
        let mut rename: HashMap<String, String> = HashMap::new();
        for n in &right_others {
            if left_others.contains(n) {
                rename.insert(n.clone(), format!("right_{}", n));
            }
        }
        let mut order = on.clone();
        order.extend(left_others.iter().cloned());
        order.extend(right_others.iter().map(|n| rename.get(n).cloned().unwrap_or_else(|| n.clone())));

        let mut li = Vec::new();
        let mut ri: Vec<Option<usize>> = Vec::new();
        for i in 0..self.inner.nrows {
            let h = row_hash(&left_views, i);
            let matches: Vec<usize> = right_index
                .get(&h)
                .map(|bucket| bucket.iter().copied().filter(|&j| rows_equal_cross(&left_views, &right_views, i, j)).collect())
                .unwrap_or_default();
            if matches.is_empty() {
                if how == "left" {
                    li.push(i);
                    ri.push(None);
                }
                continue;
            }
            for j in matches {
                li.push(i);
                ri.push(Some(j));
            }
        }

        let mut columns = HashMap::new();
        for name in &on {
            columns.insert(name.clone(), self.inner.columns[name].take(&li));
        }
        for name in &left_others {
            columns.insert(name.clone(), self.inner.columns[name].take(&li));
        }
        for name in &right_others {
            let out_name = rename.get(name).cloned().unwrap_or_else(|| name.clone());
            let src = &other.inner.columns[name];
            let vals = match src {
                Series::Num(v) => Series::Num(Arc::new(ri.iter().map(|r| r.map(|i| v[i]).unwrap_or(f64::NAN)).collect())),
                Series::Str(v) => Series::Str(Arc::new(ri.iter().map(|r| r.map(|i| v[i].clone()).unwrap_or_default()).collect())),
                Series::Bool(v) => Series::Bool(Arc::new(ri.iter().map(|r| r.map(|i| v[i]).unwrap_or(false)).collect())),
            };
            columns.insert(out_name, vals);
        }
        let n = li.len();
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, columns, n)),
        })
    }

    #[sera_doc(
        name = "SeraDFrame.concat",
        category = "data_method", file = "canvas/dframe.md", en = "Vertical union with another SeraDFrame.", fr = "Union verticale avec un autre SeraDFrame.")]
    fn concat(&self, other: &SeraDFrame_) -> SeraDFrame_ {
        let mut order = self.inner.order.clone();
        for n in &other.inner.order {
            if !order.contains(n) {
                order.push(n.clone());
            }
        }
        let columns: HashMap<String, Series> = order
            .par_iter()
            .map(|name| {
                let left = self.inner.columns.get(name);
                let right = other.inner.columns.get(name);
                let merged = match (left, right) {
                    (Some(Series::Num(l)), Some(r)) => {
                        let mut v = l.as_ref().clone();
                        v.extend(r.to_f64_vec());
                        Series::Num(Arc::new(v))
                    }
                    (Some(Series::Bool(l)), Some(Series::Bool(r))) => {
                        let mut v = l.as_ref().clone();
                        v.extend(r.iter().copied());
                        Series::Bool(Arc::new(v))
                    }
                    (Some(Series::Str(l)), Some(Series::Str(r))) => {
                        let mut v = l.as_ref().clone();
                        v.extend(r.iter().cloned());
                        Series::Str(Arc::new(v))
                    }
                    (Some(l), None) => {
                        let mut v = l.to_str_vec();
                        v.extend(vec![String::new(); other.inner.nrows]);
                        super::super::str_series(v)
                    }
                    (None, Some(r)) => {
                        let mut v = vec![String::new(); self.inner.nrows];
                        v.extend(r.to_str_vec());
                        super::super::str_series(v)
                    }
                    (Some(l), Some(r)) => {
                        let mut v = l.to_str_vec();
                        v.extend(r.to_str_vec());
                        super::super::str_series(v)
                    }
                    (None, None) => Series::Str(Arc::new(vec![])),
                };
                (name.clone(), merged)
            })
            .collect();
        SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, columns, self.inner.nrows + other.inner.nrows)),
        }
    }

    #[sera_doc(
        name = "SeraDFrame.to_csv",
        category = "data_method", file = "canvas/dframe.md", en = "Writes to a CSV file.", fr = "Ecrit dans un fichier CSV.")]
    fn to_csv(&self, path: &str) -> PyResult<()> {
        let mut wtr = csv::Writer::from_path(path).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        wtr.write_record(&self.inner.order)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        let cols: Vec<Vec<String>> = self.inner.order.iter().map(|n| self.inner.columns[n].to_str_vec()).collect();
        for i in 0..self.inner.nrows {
            let row: Vec<&str> = cols.iter().map(|c| c[i].as_str()).collect();
            wtr.write_record(&row).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        }
        wtr.flush().map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    #[pyo3(signature = (orient = "records"))]
    #[sera_doc(
        name = "SeraDFrame.to_json",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Serializes to a JSON string. orient='records' (default) is a list of row objects; orient='columns' is a dict of column name to value list.",
        fr = "Serialise en chaine JSON. orient='records' (defaut) est une liste d'objets par ligne ; orient='columns' est un dict nom de colonne vers liste de valeurs."
    )]
    fn to_json(&self, orient: &str) -> PyResult<String> {
        let value = if orient == "columns" {
            let mut obj = serde_json::Map::new();
            for name in &self.inner.order {
                let series = &self.inner.columns[name];
                let arr: Vec<serde_json::Value> = match series {
                    Series::Num(v) => v.iter().map(|x| serde_json::json!(x)).collect(),
                    Series::Str(v) => v.iter().map(|x| serde_json::json!(x.as_ref())).collect(),
                    Series::Bool(v) => v.iter().map(|x| serde_json::json!(x)).collect(),
                };
                obj.insert(name.clone(), serde_json::Value::Array(arr));
            }
            serde_json::Value::Object(obj)
        } else {
            let rows: Vec<serde_json::Value> = (0..self.inner.nrows)
                .map(|i| {
                    let mut obj = serde_json::Map::new();
                    for name in &self.inner.order {
                        let v = match &self.inner.columns[name] {
                            Series::Num(c) => serde_json::json!(c[i]),
                            Series::Str(c) => serde_json::json!(c[i].as_ref()),
                            Series::Bool(c) => serde_json::json!(c[i]),
                        };
                        obj.insert(name.clone(), v);
                    }
                    serde_json::Value::Object(obj)
                })
                .collect();
            serde_json::Value::Array(rows)
        };
        serde_json::to_string(&value).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    #[sera_doc(
        name = "SeraDFrame.to_records",
        category = "data_method", file = "canvas/dframe.md", en = "Row-oriented list of dicts.", fr = "Vue orientee ligne, liste de dicts.", aliases("to_dicts"))]
    pub(crate) fn to_records(&self, py: Python<'_>) -> Vec<PyObject> {
        (0..self.inner.nrows)
            .map(|i| {
                let dict = PyDict::new_bound(py);
                for name in &self.inner.order {
                    let v = match &self.inner.columns[name] {
                        Series::Num(c) => c[i].into_py(py),
                        Series::Str(c) => c[i].to_string().into_py(py),
                        Series::Bool(c) => c[i].into_py(py),
                    };
                    let _ = dict.set_item(name, v);
                }
                dict.into()
            })
            .collect()
    }
}
