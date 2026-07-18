use super::groupby::GroupKeys;
use super::series::{f64_sort_key, row_hash, rows_equal, ColView, FxBuildHasher, PassThroughBuildHasher, PyCell, Series};
use super::{str_series, SeraDFrame, SeraDFrame_};
use crate::sera_doc_impl;
use pyo3::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

impl SeraDFrame_ {
    pub(super) fn filter_numeric(&self, name: &str, pred: impl Fn(f64) -> bool + Sync) -> PyResult<SeraDFrame_> {
        let col = self.inner.get(name)?;
        let owned;
        let vals: &[f64] = match col.as_f64_slice() {
            Some(v) => v,
            None => {
                owned = col.to_f64_vec();
                &owned
            }
        };
        let idx: Vec<usize> = vals
            .par_iter()
            .enumerate()
            .filter(|(_, v)| pred(**v))
            .map(|(i, _)| i)
            .collect();
        Ok(SeraDFrame_ {
            inner: Arc::new(self.inner.row_take(&idx)),
        })
    }
}

#[sera_doc_impl]
#[pymethods]
impl SeraDFrame_ {
    #[sera_doc(
        name = "SeraDFrame.filter_eq",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Keeps rows where a column equals a value.",
        fr = "Garde les lignes ou une colonne vaut une valeur.",
        aliases("where_eq")
    )]
    pub(crate) fn filter_eq(&self, name: &str, value: PyCell) -> PyResult<SeraDFrame_> {
        let col = self.inner.get(name)?;
        let target: Arc<str> = Arc::from(match value {
            PyCell::F(f) => {
                if f.fract().abs() < 1e-9 {
                    format!("{}", f as i64)
                } else {
                    f.to_string()
                }
            }
            PyCell::B(b) => b.to_string(),
            PyCell::S(s) => s,
        });
        let idx: Vec<usize> = match col.as_str_slice() {
            Some(v) => v
                .iter()
                .enumerate()
                .filter(|(_, s)| **s == target)
                .map(|(i, _)| i)
                .collect(),
            None => col
                .to_str_vec()
                .iter()
                .enumerate()
                .filter(|(_, s)| s.as_str() == &*target)
                .map(|(i, _)| i)
                .collect(),
        };
        Ok(SeraDFrame_ {
            inner: Arc::new(self.inner.row_take(&idx)),
        })
    }

    #[sera_doc(
        name = "SeraDFrame.filter_gt",
        category = "data_method", file = "canvas/dframe.md", en = "Keeps rows greater than a value.", fr = "Garde les lignes superieures a une valeur.")]
    fn filter_gt(&self, name: &str, value: f64) -> PyResult<SeraDFrame_> {
        self.filter_numeric(name, |v| v > value)
    }

    #[sera_doc(
        name = "SeraDFrame.filter_lt",
        category = "data_method", file = "canvas/dframe.md", en = "Keeps rows lesser than a value.", fr = "Garde les lignes inferieures a une valeur.")]
    fn filter_lt(&self, name: &str, value: f64) -> PyResult<SeraDFrame_> {
        self.filter_numeric(name, |v| v < value)
    }

    fn filter_ge(&self, name: &str, value: f64) -> PyResult<SeraDFrame_> {
        self.filter_numeric(name, |v| v >= value)
    }

    fn filter_le(&self, name: &str, value: f64) -> PyResult<SeraDFrame_> {
        self.filter_numeric(name, |v| v <= value)
    }

    #[sera_doc(
        name = "SeraDFrame.between",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Keeps rows within an inclusive numeric range.",
        fr = "Garde les lignes dans un intervalle numerique inclusif."
    )]
    fn between(&self, name: &str, lo: f64, hi: f64) -> PyResult<SeraDFrame_> {
        self.filter_numeric(name, |v| v >= lo && v <= hi)
    }

    #[sera_doc(
        name = "SeraDFrame.filter_in",
        category = "data_method", file = "canvas/dframe.md", en = "Keeps rows whose value is in a set.", fr = "Garde les lignes dont la valeur est dans un ensemble.")]
    fn filter_in(&self, name: &str, values: Vec<String>) -> PyResult<SeraDFrame_> {
        let col = self.inner.get(name)?;
        let wanted: std::collections::HashSet<String> = values.into_iter().collect();
        let owned;
        let strs: &[Arc<str>] = match col.as_str_slice() {
            Some(v) => v,
            None => {
                owned = col.to_str_vec().into_iter().map(Arc::from).collect::<Vec<_>>();
                &owned
            }
        };
        let idx: Vec<usize> = strs
            .par_iter()
            .enumerate()
            .filter(|(_, s)| wanted.contains(s.as_ref()))
            .map(|(i, _)| i)
            .collect();
        Ok(SeraDFrame_ {
            inner: Arc::new(self.inner.row_take(&idx)),
        })
    }

    #[sera_doc(
        name = "SeraDFrame.dropna",
        category = "data_method", file = "canvas/dframe.md", en = "Drops rows with missing values.", fr = "Supprime les lignes avec valeurs manquantes.")]
    fn dropna(&self, subset: Option<Vec<String>>) -> PyResult<SeraDFrame_> {
        let cols = subset.unwrap_or_else(|| self.inner.order.clone());
        let mut keep = vec![true; self.inner.nrows];
        for name in &cols {
            match self.inner.get(name)? {
                Series::Num(v) => {
                    for (i, x) in v.iter().enumerate() {
                        if x.is_nan() {
                            keep[i] = false;
                        }
                    }
                }
                Series::Str(v) => {
                    for (i, x) in v.iter().enumerate() {
                        if x.is_empty() {
                            keep[i] = false;
                        }
                    }
                }
                Series::Bool(_) => {}
            }
        }
        let idx: Vec<usize> = keep.iter().enumerate().filter(|(_, k)| **k).map(|(i, _)| i).collect();
        Ok(SeraDFrame_ {
            inner: Arc::new(self.inner.row_take(&idx)),
        })
    }

    #[sera_doc(
        name = "SeraDFrame.fillna",
        category = "data_method", file = "canvas/dframe.md", en = "Fills missing values with a constant.", fr = "Remplit les valeurs manquantes par une constante.")]
    fn fillna(&self, value: PyCell) -> SeraDFrame_ {
        let mut columns = HashMap::new();
        for name in &self.inner.order {
            let col = &self.inner.columns[name];
            let filled = match (col, &value) {
                (Series::Num(v), PyCell::F(f)) => {
                    Series::Num(Arc::new(v.iter().map(|x| if x.is_nan() { *f } else { *x }).collect()))
                }
                (Series::Str(v), PyCell::S(s)) => str_series(
                    v.iter()
                        .map(|x| if x.is_empty() { s.clone() } else { x.to_string() })
                        .collect(),
                ),
                _ => col.clone(),
            };
            columns.insert(name.clone(), filled);
        }
        SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(self.inner.order.clone(), columns, self.inner.nrows)),
        }
    }

    #[sera_doc(
        name = "SeraDFrame.ffill",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Forward-fills missing values with the last valid observation.",
        fr = "Propage vers l'avant la derniere valeur valide pour combler les manquants."
    )]
    fn ffill(&self, col: &str) -> PyResult<SeraDFrame_> {
        let filled = match self.inner.get(col)? {
            Series::Num(v) => {
                let mut out = v.as_ref().clone();
                let mut last = f64::NAN;
                for x in out.iter_mut() {
                    if x.is_nan() {
                        *x = last;
                    } else {
                        last = *x;
                    }
                }
                Series::Num(Arc::new(out))
            }
            Series::Str(v) => {
                let mut out: Vec<String> = v.iter().map(|s| s.to_string()).collect();
                let mut last = String::new();
                for x in out.iter_mut() {
                    if x.is_empty() {
                        *x = last.clone();
                    } else {
                        last = x.clone();
                    }
                }
                str_series(out)
            }
            other => other.clone(),
        };
        let mut columns = self.inner.columns.clone();
        columns.insert(col.to_string(), filled);
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(self.inner.order.clone(), columns, self.inner.nrows)),
        })
    }

    #[sera_doc(
        name = "SeraDFrame.bfill",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Back-fills missing values with the next valid observation.",
        fr = "Propage vers l'arriere la prochaine valeur valide pour combler les manquants."
    )]
    fn bfill(&self, col: &str) -> PyResult<SeraDFrame_> {
        let filled = match self.inner.get(col)? {
            Series::Num(v) => {
                let mut out = v.as_ref().clone();
                let mut next = f64::NAN;
                for x in out.iter_mut().rev() {
                    if x.is_nan() {
                        *x = next;
                    } else {
                        next = *x;
                    }
                }
                Series::Num(Arc::new(out))
            }
            Series::Str(v) => {
                let mut out: Vec<String> = v.iter().map(|s| s.to_string()).collect();
                let mut next = String::new();
                for x in out.iter_mut().rev() {
                    if x.is_empty() {
                        *x = next.clone();
                    } else {
                        next = x.clone();
                    }
                }
                str_series(out)
            }
            other => other.clone(),
        };
        let mut columns = self.inner.columns.clone();
        columns.insert(col.to_string(), filled);
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(self.inner.order.clone(), columns, self.inner.nrows)),
        })
    }

    #[sera_doc(
        name = "SeraDFrame.drop_duplicates",
        category = "data_method", file = "canvas/dframe.md", en = "Drops duplicate rows.", fr = "Supprime les lignes dupliquees.", aliases("dedupe"))]
    pub(crate) fn drop_duplicates(&self, subset: Option<Vec<String>>) -> PyResult<SeraDFrame_> {
        let cols = subset.unwrap_or_else(|| self.inner.order.clone());
        let views: Vec<ColView> = cols.iter().map(|n| self.inner.get(n).map(|c| c.as_view())).collect::<PyResult<Vec<_>>>()?;
        let mut buckets: HashMap<u64, Vec<usize>, PassThroughBuildHasher> = HashMap::default();
        let mut idx = Vec::new();
        for i in 0..self.inner.nrows {
            let h = row_hash(&views, i);
            let bucket = buckets.entry(h).or_default();
            if !bucket.iter().any(|&j| rows_equal(&views, i, j)) {
                bucket.push(i);
                idx.push(i);
            }
        }
        Ok(SeraDFrame_ {
            inner: Arc::new(self.inner.row_take(&idx)),
        })
    }

    #[pyo3(signature = (cols, ascending = true))]
    #[sera_doc(
        name = "SeraDFrame.sort_values",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Sorts rows by one or more columns, lexicographic tie-break.",
        fr = "Trie les lignes selon une ou plusieurs colonnes, egalites departagees lexicographiquement.",
        aliases("sort")
    )]
    pub(crate) fn sort_values(&self, cols: GroupKeys, ascending: bool) -> PyResult<SeraDFrame_> {
        let cols = cols.into_vec();
        let mut idx: Vec<usize> = (0..self.inner.nrows).collect();
        if cols.len() == 1 {
            let key: Vec<u64> = match self.inner.get(&cols[0])? {
                Series::Num(v) => v.par_iter().map(|&x| f64_sort_key(x)).collect(),
                Series::Bool(v) => v.par_iter().map(|&b| b as u64).collect(),
                Series::Str(v) => {
                    let mut uniq: Vec<&Arc<str>> = v.iter().collect();
                    uniq.sort_unstable();
                    uniq.dedup();
                    let rank: HashMap<Arc<str>, u64, FxBuildHasher> = uniq.into_iter().enumerate().map(|(i, s)| (s.clone(), i as u64)).collect();
                    v.par_iter().map(|s| rank[s]).collect()
                }
            };
            let mut keyed: Vec<(u64, usize)> = idx.par_iter().map(|&i| (key[i], i)).collect();
            keyed.par_sort_unstable_by_key(|&(k, _)| k);
            idx = keyed.into_iter().map(|(_, i)| i).collect();
        } else {
            let series_list: Vec<&Series> = cols.iter().map(|n| self.inner.get(n)).collect::<PyResult<Vec<_>>>()?;
            let key_matrix: Vec<Vec<u64>> = series_list
                .iter()
                .map(|s| match s {
                    Series::Num(v) => v.par_iter().map(|&x| f64_sort_key(x)).collect(),
                    Series::Bool(v) => v.par_iter().map(|&b| b as u64).collect(),
                    Series::Str(v) => {
                        let mut uniq: Vec<&Arc<str>> = v.iter().collect();
                        uniq.sort_unstable();
                        uniq.dedup();
                        let rank: HashMap<Arc<str>, u64, FxBuildHasher> = uniq.into_iter().enumerate().map(|(i, s)| (s.clone(), i as u64)).collect();
                        v.par_iter().map(|s| rank[s]).collect()
                    }
                })
                .collect();
            let mut keyed: Vec<(Vec<u64>, usize)> = idx
                .par_iter()
                .map(|&i| (key_matrix.iter().map(|col| col[i]).collect(), i))
                .collect();
            keyed.par_sort_unstable_by(|a, b| a.0.cmp(&b.0));
            idx = keyed.into_iter().map(|(_, i)| i).collect();
        }
        if !ascending {
            idx.reverse();
        }
        Ok(SeraDFrame_ {
            inner: Arc::new(self.inner.row_take(&idx)),
        })
    }
}
