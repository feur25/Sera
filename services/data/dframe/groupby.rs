use super::series::{row_hash, rows_equal, ColView, FxBuildHasher, PassThroughBuildHasher, Series};
use super::{SeraDFrame, SeraDFrame_};
use crate::sera_doc_impl;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(FromPyObject)]
pub enum GroupKeys {
    One(String),
    Many(Vec<String>),
}

impl GroupKeys {
    pub fn into_vec(self) -> Vec<String> {
        match self {
            GroupKeys::One(s) => vec![s],
            GroupKeys::Many(v) => v,
        }
    }
}

#[pyclass]
pub struct SeraDFrameGroupBy {
    frame: Arc<SeraDFrame>,
    group_cols: Vec<String>,
}

#[sera_doc_impl]
#[pymethods]
impl SeraDFrame_ {
    #[sera_doc(
        name = "SeraDFrame.groupby",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Groups rows by one or more columns for aggregation.",
        fr = "Groupe les lignes par une ou plusieurs colonnes pour agregation.",
        aliases("group_by")
    )]
    pub(crate) fn groupby(&self, keys: GroupKeys) -> SeraDFrameGroupBy {
        SeraDFrameGroupBy {
            frame: self.inner.clone(),
            group_cols: keys.into_vec(),
        }
    }
}

struct GroupAcc {
    sum: Vec<f64>,
    min: Vec<f64>,
    max: Vec<f64>,
    count: Vec<u32>,
}

impl GroupAcc {
    fn new(n: usize) -> Self {
        GroupAcc {
            sum: vec![0.0; n],
            min: vec![f64::INFINITY; n],
            max: vec![f64::NEG_INFINITY; n],
            count: vec![0; n],
        }
    }

    fn absorb(&mut self, vals: &[f64], group_id: &[u32]) {
        for (&v, &g) in vals.iter().zip(group_id.iter()) {
            if v.is_nan() {
                continue;
            }
            let g = g as usize;
            self.sum[g] += v;
            if v < self.min[g] {
                self.min[g] = v;
            }
            if v > self.max[g] {
                self.max[g] = v;
            }
            self.count[g] += 1;
        }
    }

    fn merge(mut self, other: GroupAcc) -> GroupAcc {
        for g in 0..self.sum.len() {
            self.sum[g] += other.sum[g];
            if other.min[g] < self.min[g] {
                self.min[g] = other.min[g];
            }
            if other.max[g] > self.max[g] {
                self.max[g] = other.max[g];
            }
            self.count[g] += other.count[g];
        }
        self
    }

    fn resolve(&self, agg: &str) -> Vec<f64> {
        (0..self.sum.len())
            .map(|g| {
                let min = if self.min[g].is_finite() { self.min[g] } else { 0.0 };
                let max = if self.max[g].is_finite() { self.max[g] } else { 0.0 };
                crate::core::dispatch::resolve_agg(agg, self.sum[g], min, max, self.count[g] as f64)
            })
            .collect()
    }
}

fn accumulate(vals: &[f64], group_id: &[u32], n_groups: usize) -> GroupAcc {
    let threads = rayon::current_num_threads().max(1);
    let chunk_size = (vals.len() / threads).max(4096).min(vals.len().max(1));
    vals.par_chunks(chunk_size)
        .zip(group_id.par_chunks(chunk_size))
        .map(|(vchunk, gchunk)| {
            let mut acc = GroupAcc::new(n_groups);
            acc.absorb(vchunk, gchunk);
            acc
        })
        .reduce(|| GroupAcc::new(n_groups), GroupAcc::merge)
}

impl SeraDFrameGroupBy {
    fn groups_single(&self, name: &str) -> PyResult<(Vec<Arc<str>>, Vec<u32>)> {
        let col = self.frame.get(name)?;
        let owned;
        let keys: &[Arc<str>] = match col.as_str_slice() {
            Some(v) => v,
            None => {
                owned = col.to_str_vec().into_iter().map(Arc::from).collect::<Vec<_>>();
                &owned
            }
        };
        let mut order: Vec<Arc<str>> = Vec::new();
        let mut lookup: HashMap<Arc<str>, u32, FxBuildHasher> =
            HashMap::with_capacity_and_hasher(64, FxBuildHasher::default());
        let mut group_id = Vec::with_capacity(keys.len());
        for k in keys {
            let id = match lookup.get(k) {
                Some(&id) => id,
                None => {
                    let id = order.len() as u32;
                    order.push(k.clone());
                    lookup.insert(k.clone(), id);
                    id
                }
            };
            group_id.push(id);
        }
        Ok((order, group_id))
    }

    fn groups_multi(&self) -> PyResult<(Vec<Vec<Arc<str>>>, Vec<u32>)> {
        let views: Vec<ColView> = self.group_cols.iter().map(|n| self.frame.get(n).map(|s| s.as_view())).collect::<PyResult<Vec<_>>>()?;
        let nrows = self.frame.nrows;
        let mut buckets: HashMap<u64, Vec<u32>, PassThroughBuildHasher> = HashMap::default();
        let mut order_rows: Vec<usize> = Vec::new();
        let mut group_id = vec![0u32; nrows];
        for i in 0..nrows {
            let h = row_hash(&views, i);
            let bucket = buckets.entry(h).or_default();
            let mut found = None;
            for &gid in bucket.iter() {
                if rows_equal(&views, i, order_rows[gid as usize]) {
                    found = Some(gid);
                    break;
                }
            }
            let gid = match found {
                Some(g) => g,
                None => {
                    let g = order_rows.len() as u32;
                    order_rows.push(i);
                    bucket.push(g);
                    g
                }
            };
            group_id[i] = gid;
        }
        let key_cols: Vec<&Series> = self.group_cols.iter().map(|n| self.frame.get(n)).collect::<PyResult<Vec<_>>>()?;
        let order_keys: Vec<Vec<Arc<str>>> = key_cols
            .iter()
            .map(|c| order_rows.iter().map(|&r| Arc::from(c.value_str(r).as_str())).collect())
            .collect();
        Ok((order_keys, group_id))
    }

    fn groups(&self) -> PyResult<(Vec<Vec<Arc<str>>>, Vec<u32>)> {
        if self.group_cols.len() == 1 {
            let (order, group_id) = self.groups_single(&self.group_cols[0])?;
            return Ok((vec![order], group_id));
        }
        self.groups_multi()
    }

    fn agg_one_slow(vals: &[f64], agg: &str) -> f64 {
        let clean: Vec<f64> = vals.iter().copied().filter(|v| !v.is_nan()).collect();
        if clean.is_empty() {
            return 0.0;
        }
        let n = clean.len() as f64;
        let mean = clean.iter().sum::<f64>() / n;
        match agg {
            "std" => (clean.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / n).sqrt(),
            _ => mean,
        }
    }

    fn insert_key_columns(&self, columns: &mut HashMap<String, Series>, order_keys: &[Vec<Arc<str>>]) {
        for (name, keys) in self.group_cols.iter().zip(order_keys.iter()) {
            columns.insert(name.clone(), Series::Str(Arc::new(keys.clone())));
        }
    }

    fn agg_all_kind(&self, agg: &str) -> PyResult<SeraDFrame_> {
        let (order_keys, group_id) = self.groups()?;
        let n_groups = order_keys[0].len();
        let numeric = self.frame.numeric_columns();
        let mut columns = HashMap::new();
        self.insert_key_columns(&mut columns, &order_keys);
        for col in &numeric {
            if self.group_cols.contains(col) {
                continue;
            }
            let vals = match &self.frame.columns[col] {
                Series::Num(v) => v,
                _ => continue,
            };
            let acc = accumulate(vals, &group_id, n_groups);
            columns.insert(col.clone(), Series::Num(Arc::new(acc.resolve(agg))));
        }
        let mut order = self.group_cols.clone();
        order.extend(numeric.into_iter().filter(|c| !self.group_cols.contains(c)));
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, columns, n_groups)),
        })
    }
}

#[sera_doc_impl]
#[pymethods]
impl SeraDFrameGroupBy {
    #[sera_doc(
        name = "SeraDFrameGroupBy.agg",
        category = "data_method", file = "canvas/dframe.md", en = "Per-column aggregation spec.", fr = "Specification d'agregation par colonne.")]
    fn agg(&self, spec: &Bound<'_, PyDict>) -> PyResult<SeraDFrame_> {
        let (order_keys, group_id) = self.groups()?;
        let n_groups = order_keys[0].len();
        let mut columns = HashMap::new();
        self.insert_key_columns(&mut columns, &order_keys);
        let mut order = self.group_cols.clone();
        for (k, v) in spec.iter() {
            let col_name: String = k.extract()?;
            let agg_kind: String = v.extract()?;
            let col = self.frame.get(&col_name)?;
            let owned;
            let vals: &[f64] = match col.as_f64_slice() {
                Some(v) => v,
                None => {
                    owned = col.to_f64_vec();
                    &owned
                }
            };
            let agged = if agg_kind == "std" {
                let mut buckets: Vec<Vec<f64>> = vec![Vec::new(); n_groups];
                for (&v, &g) in vals.iter().zip(group_id.iter()) {
                    buckets[g as usize].push(v);
                }
                buckets.iter().map(|b| Self::agg_one_slow(b, "std")).collect()
            } else {
                accumulate(vals, &group_id, n_groups).resolve(&agg_kind)
            };
            order.push(col_name.clone());
            columns.insert(col_name, Series::Num(Arc::new(agged)));
        }
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, columns, n_groups)),
        })
    }

    #[sera_doc(
        name = "SeraDFrameGroupBy.mean",
        category = "data_method", file = "canvas/dframe.md", en = "Mean of every numeric column per group.", fr = "Moyenne de chaque colonne numerique par groupe.")]
    fn mean(&self) -> PyResult<SeraDFrame_> {
        self.agg_all_kind("mean")
    }

    #[sera_doc(
        name = "SeraDFrameGroupBy.sum",
        category = "data_method", file = "canvas/dframe.md", en = "Sum of every numeric column per group.", fr = "Somme de chaque colonne numerique par groupe.")]
    fn sum(&self) -> PyResult<SeraDFrame_> {
        self.agg_all_kind("sum")
    }

    fn min(&self) -> PyResult<SeraDFrame_> {
        self.agg_all_kind("min")
    }

    fn max(&self) -> PyResult<SeraDFrame_> {
        self.agg_all_kind("max")
    }

    fn count(&self) -> PyResult<SeraDFrame_> {
        self.agg_all_kind("count")
    }

    #[sera_doc(
        name = "SeraDFrameGroupBy.size",
        category = "data_method", file = "canvas/dframe.md", en = "Row count per group.", fr = "Nombre de lignes par groupe.")]
    fn size(&self) -> PyResult<SeraDFrame_> {
        let (order_keys, group_id) = self.groups()?;
        let n_groups = order_keys[0].len();
        let mut sizes = vec![0.0f64; n_groups];
        for &g in &group_id {
            sizes[g as usize] += 1.0;
        }
        let mut columns = HashMap::new();
        self.insert_key_columns(&mut columns, &order_keys);
        columns.insert("size".to_string(), Series::Num(Arc::new(sizes)));
        let mut order = self.group_cols.clone();
        order.push("size".to_string());
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, columns, n_groups)),
        })
    }

    #[sera_doc(
        name = "SeraDFrameGroupBy.transform",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Broadcasts a per-group aggregate back onto every original row.",
        fr = "Diffuse une agregation par groupe sur chaque ligne d'origine."
    )]
    fn transform(&self, col: &str, agg: &str) -> PyResult<SeraDFrame_> {
        let (order_keys, group_id) = self.groups()?;
        let n_groups = order_keys[0].len();
        let vals = self.frame.get(col)?.to_f64_vec();
        let group_vals = if agg == "std" {
            let mut buckets: Vec<Vec<f64>> = vec![Vec::new(); n_groups];
            for (&v, &g) in vals.iter().zip(group_id.iter()) {
                buckets[g as usize].push(v);
            }
            buckets.iter().map(|b| Self::agg_one_slow(b, "std")).collect::<Vec<f64>>()
        } else {
            accumulate(&vals, &group_id, n_groups).resolve(agg)
        };
        let broadcast: Vec<f64> = group_id.iter().map(|&g| group_vals[g as usize]).collect();
        let name = format!("{}_{}", col, agg);
        let mut order = self.frame.order.clone();
        let mut columns = self.frame.columns.clone();
        if !columns.contains_key(&name) {
            order.push(name.clone());
        }
        columns.insert(name, Series::Num(Arc::new(broadcast)));
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, columns, self.frame.nrows)),
        })
    }

    #[sera_doc(
        name = "SeraDFrameGroupBy.head",
        category = "data_method", file = "canvas/dframe.md", en = "First n rows of every group.", fr = "Les n premieres lignes de chaque groupe.")]
    fn head(&self, n: usize) -> PyResult<SeraDFrame_> {
        let (order_keys, group_id) = self.groups()?;
        let n_groups = order_keys[0].len();
        let mut counts = vec![0usize; n_groups];
        let mut idx = Vec::new();
        for (i, &g) in group_id.iter().enumerate() {
            let g = g as usize;
            if counts[g] < n {
                idx.push(i);
                counts[g] += 1;
            }
        }
        Ok(SeraDFrame_ {
            inner: Arc::new(self.frame.row_take(&idx)),
        })
    }

    #[sera_doc(
        name = "SeraDFrameGroupBy.tail",
        category = "data_method", file = "canvas/dframe.md", en = "Last n rows of every group.", fr = "Les n dernieres lignes de chaque groupe.")]
    fn tail(&self, n: usize) -> PyResult<SeraDFrame_> {
        let (order_keys, group_id) = self.groups()?;
        let n_groups = order_keys[0].len();
        let mut per_group: Vec<Vec<usize>> = vec![Vec::new(); n_groups];
        for (i, &g) in group_id.iter().enumerate() {
            per_group[g as usize].push(i);
        }
        let mut idx = Vec::new();
        for grp in &per_group {
            let start = grp.len().saturating_sub(n);
            idx.extend(&grp[start..]);
        }
        idx.sort_unstable();
        Ok(SeraDFrame_ {
            inner: Arc::new(self.frame.row_take(&idx)),
        })
    }

    #[sera_doc(
        name = "SeraDFrameGroupBy.nth",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "The nth row of every group (negative indexes from the end).",
        fr = "La n-ieme ligne de chaque groupe (index negatif depuis la fin)."
    )]
    fn nth(&self, n: i64) -> PyResult<SeraDFrame_> {
        let (order_keys, group_id) = self.groups()?;
        let n_groups = order_keys[0].len();
        let mut per_group: Vec<Vec<usize>> = vec![Vec::new(); n_groups];
        for (i, &g) in group_id.iter().enumerate() {
            per_group[g as usize].push(i);
        }
        let mut idx = Vec::new();
        for grp in &per_group {
            let len = grp.len() as i64;
            let target = if n >= 0 { n } else { len + n };
            if target >= 0 && target < len {
                idx.push(grp[target as usize]);
            }
        }
        idx.sort_unstable();
        Ok(SeraDFrame_ {
            inner: Arc::new(self.frame.row_take(&idx)),
        })
    }
}
