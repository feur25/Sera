use super::super::series::{FxBuildHasher, Series};
use super::super::{SeraDFrame, SeraDFrame_};
use crate::core::dispatch::stats_par;
use crate::sera_doc_impl;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;
use std::sync::Arc;

#[sera_doc_impl]
#[pymethods]
impl SeraDFrame_ {
    #[sera_doc(
        name = "SeraDFrame.resample",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Buckets rows into fixed-width time windows (freq_seconds) over an epoch-second column, then aggregates one or more columns per window (sum/mean/min/max/count). One output row per non-empty window, sorted ascending by window start.",
        fr = "Regroupe les lignes en fenetres temporelles de largeur fixe (freq_seconds) sur une colonne secondes-epoch, puis agrege une ou plusieurs colonnes par fenetre (sum/mean/min/max/count). Une ligne en sortie par fenetre non-vide, triee par ordre croissant du debut de fenetre."
    )]
    pub(crate) fn resample(&self, time_col: &str, freq_seconds: f64, spec: &Bound<'_, PyDict>) -> PyResult<SeraDFrame_> {
        let freq = if freq_seconds <= 0.0 { 1.0 } else { freq_seconds };
        let times = self.inner.get(time_col)?.to_f64_vec();

        let buckets: Vec<i64> = times.iter().map(|&t| (t / freq).floor() as i64).collect();
        let mut bucket_order: Vec<i64> = buckets.clone();
        bucket_order.sort_unstable();
        bucket_order.dedup();

        let mut bucket_rows: HashMap<i64, Vec<usize>, FxBuildHasher> =
            HashMap::with_capacity_and_hasher(bucket_order.len(), FxBuildHasher::default());
        for (i, &b) in buckets.iter().enumerate() {
            bucket_rows.entry(b).or_insert_with(Vec::new).push(i);
        }

        let mut order = vec![time_col.to_string()];
        let mut columns: HashMap<String, Series> = HashMap::new();
        columns.insert(
            time_col.to_string(),
            Series::Num(Arc::new(bucket_order.iter().map(|&b| b as f64 * freq).collect())),
        );

        for (k, v) in spec.iter() {
            let col_name: String = k.extract()?;
            let agg_kind: String = v.extract()?;
            let vals = self.inner.get(&col_name)?.to_f64_vec();
            let out_name = format!("{}_{}", col_name, agg_kind);
            let mut out_vals = Vec::with_capacity(bucket_order.len());
            for b in &bucket_order {
                let sub: Vec<f64> = bucket_rows[b].iter().map(|&i| vals[i]).collect();
                let s = stats_par(&sub);
                out_vals.push(match agg_kind.as_str() {
                    "sum" => s.sum,
                    "min" => s.min,
                    "max" => s.max,
                    "count" => s.count as f64,
                    _ => s.mean(),
                });
            }
            order.push(out_name.clone());
            columns.insert(out_name, Series::Num(Arc::new(out_vals)));
        }

        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, columns, bucket_order.len())),
        })
    }
}
