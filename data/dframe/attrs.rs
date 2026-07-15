use super::series::Series;
use super::{SeraDFrame, SeraDFrame_};
use crate::sera_doc_impl;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[sera_doc_impl]
#[pymethods]
impl SeraDFrame_ {
    #[getter]
    #[sera_doc(
        name = "SeraDFrame.size",
        category = "data_method", file = "canvas/dframe.md", en = "Total cell count (rows times columns).", fr = "Nombre total de cellules (lignes fois colonnes).")]
    fn size(&self) -> usize {
        self.inner.nrows * self.inner.order.len()
    }

    #[getter]
    #[sera_doc(
        name = "SeraDFrame.ndim",
        category = "data_method", file = "canvas/dframe.md", en = "Always 2 for a SeraDFrame.", fr = "Toujours 2 pour un SeraDFrame.")]
    fn ndim(&self) -> usize {
        2
    }

    #[getter]
    #[sera_doc(
        name = "SeraDFrame.empty",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "True if the frame has no rows or no columns.",
        fr = "Vrai si le frame n'a aucune ligne ou aucune colonne."
    )]
    fn empty(&self) -> bool {
        self.inner.nrows == 0 || self.inner.order.is_empty()
    }

    #[sera_doc(
        name = "SeraDFrame.values",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Row-major list-of-lists view of every cell.",
        fr = "Vue ligne par ligne, liste de listes, de chaque cellule."
    )]
    fn values(&self, py: Python<'_>) -> Vec<PyObject> {
        (0..self.inner.nrows)
            .map(|i| {
                let row: Vec<PyObject> = self
                    .inner
                    .order
                    .iter()
                    .map(|name| match &self.inner.columns[name] {
                        Series::Num(v) => v[i].into_py(py),
                        Series::Str(v) => v[i].to_string().into_py(py),
                        Series::Bool(v) => v[i].into_py(py),
                    })
                    .collect();
                row.into_py(py)
            })
            .collect()
    }

    #[sera_doc(
        name = "SeraDFrame.drop",
        category = "data_method", file = "canvas/dframe.md", en = "Drops the given columns.", fr = "Supprime les colonnes donnees.")]
    fn drop(&self, columns: Vec<String>) -> SeraDFrame_ {
        let drop_set: std::collections::HashSet<&String> = columns.iter().collect();
        let order: Vec<String> = self.inner.order.iter().filter(|c| !drop_set.contains(c)).cloned().collect();
        let columns_map: HashMap<String, Series> = order.iter().map(|n| (n.clone(), self.inner.columns[n].clone())).collect();
        SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, columns_map, self.inner.nrows)),
        }
    }
}
