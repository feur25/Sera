use super::series::{row_hash, rows_equal, ColView, PassThroughBuildHasher, Series};
use super::{SeraDFrame, SeraDFrame_};
use crate::sera_doc_impl;
use pyo3::prelude::*;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[sera_doc_impl]
#[pymethods]
impl SeraDFrame_ {
    #[sera_doc(
        name = "SeraDFrame.isna",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Per-cell missing-value boolean mask.",
        fr = "Masque booleen par cellule des valeurs manquantes.",
        aliases("is_null", "isnull")
    )]
    pub(crate) fn isna(&self) -> SeraDFrame_ {
        let columns: HashMap<String, Series> = self
            .inner
            .order
            .par_iter()
            .map(|name| {
                let mask = match &self.inner.columns[name] {
                    Series::Num(v) => v.par_iter().map(|x| x.is_nan()).collect(),
                    Series::Str(v) => v.par_iter().map(|s| s.is_empty()).collect(),
                    Series::Bool(v) => vec![false; v.len()],
                };
                (name.clone(), Series::Bool(Arc::new(mask)))
            })
            .collect();
        SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(self.inner.order.clone(), columns, self.inner.nrows)),
        }
    }

    #[sera_doc(
        name = "SeraDFrame.notna",
        category = "data_method", file = "canvas/dframe.md", en = "Inverse of isna.", fr = "Inverse de isna.", aliases("is_not_null", "notnull"))]
    pub(crate) fn notna(&self) -> SeraDFrame_ {
        let na = self.isna();
        let columns: HashMap<String, Series> = na
            .inner
            .order
            .par_iter()
            .map(|name| {
                let inv: Vec<bool> = match &na.inner.columns[name] {
                    Series::Bool(v) => v.par_iter().map(|b| !b).collect(),
                    _ => unreachable!(),
                };
                (name.clone(), Series::Bool(Arc::new(inv)))
            })
            .collect();
        SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(na.inner.order.clone(), columns, na.inner.nrows)),
        }
    }

    #[pyo3(signature = (subset = None))]
    #[sera_doc(
        name = "SeraDFrame.duplicated",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Boolean mask of rows that repeat an earlier row.",
        fr = "Masque booleen des lignes qui repetent une ligne precedente.",
        aliases("is_duplicated")
    )]
    pub(crate) fn duplicated(&self, subset: Option<Vec<String>>) -> PyResult<Vec<bool>> {
        let cols = subset.unwrap_or_else(|| self.inner.order.clone());
        let views: Vec<ColView> = cols.iter().map(|n| self.inner.get(n).map(|s| s.as_view())).collect::<PyResult<Vec<_>>>()?;
        let mut buckets: HashMap<u64, Vec<usize>, PassThroughBuildHasher> = HashMap::default();
        let mut out = vec![false; self.inner.nrows];
        for i in 0..self.inner.nrows {
            let h = row_hash(&views, i);
            let bucket = buckets.entry(h).or_default();
            if bucket.iter().any(|&j| rows_equal(&views, i, j)) {
                out[i] = true;
            } else {
                bucket.push(i);
            }
        }
        Ok(out)
    }

    #[sera_doc(
        name = "SeraDFrame.equals",
        category = "data_method", file = "canvas/dframe.md", en = "Structural equality with another SeraDFrame.", fr = "Egalite structurelle avec un autre SeraDFrame.")]
    fn equals(&self, other: &SeraDFrame_) -> bool {
        if self.inner.order != other.inner.order || self.inner.nrows != other.inner.nrows {
            return false;
        }
        self.inner.order.iter().all(|name| match (&self.inner.columns[name], &other.inner.columns[name]) {
            (Series::Num(a), Series::Num(b)) => a.iter().zip(b.iter()).all(|(x, y)| x == y || (x.is_nan() && y.is_nan())),
            (Series::Bool(a), Series::Bool(b)) => a == b,
            (a, b) => a.to_str_vec() == b.to_str_vec(),
        })
    }

    #[sera_doc(
        name = "SeraDFrame.isin",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Boolean mask of rows whose value is in a set.",
        fr = "Masque booleen des lignes dont la valeur est dans un ensemble.",
        aliases("is_in")
    )]
    pub(crate) fn isin(&self, col: &str, values: Vec<String>) -> PyResult<Vec<bool>> {
        let target = self.inner.get(col)?;
        let wanted: HashSet<String> = values.into_iter().collect();
        let owned;
        let strs: &[Arc<str>] = match target.as_str_slice() {
            Some(v) => v,
            None => {
                owned = target.to_str_vec().into_iter().map(Arc::from).collect::<Vec<_>>();
                &owned
            }
        };
        Ok(strs.par_iter().map(|s| wanted.contains(s.as_ref())).collect())
    }
}
