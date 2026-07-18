use super::derive::splitmix64;
use super::series::{FxBuildHasher, Series};
use super::{SeraDFrame, SeraDFrame_};
use crate::sera_doc_impl;
use pyo3::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

fn seed_or_time(seed: Option<u64>) -> u64 {
    seed.unwrap_or_else(|| {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x2545F4914F6CDD1D)
    })
}

fn reservoir_indices(total: usize, n: usize, seed: Option<u64>) -> Vec<usize> {
    let n = n.min(total);
    let mut state = seed_or_time(seed);
    let mut idx: Vec<usize> = (0..n).collect();
    for i in n..total {
        let j = (splitmix64(&mut state) as usize) % (i + 1);
        if j < n {
            idx[j] = i;
        }
    }
    idx.sort_unstable();
    idx
}

#[sera_doc_impl]
#[pymethods]
impl SeraDFrame_ {
    #[pyo3(signature = (frac, seed = None))]
    #[sera_doc(
        name = "SeraDFrame.sample_frac",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Random row sample sized by fraction (0.0-1.0) rather than a fixed count like sample().",
        fr = "Echantillon aleatoire de lignes dimensionne par fraction (0.0-1.0) plutot qu'un nombre fixe comme sample()."
    )]
    fn sample_frac(&self, frac: f64, seed: Option<u64>) -> SeraDFrame_ {
        let total = self.inner.nrows;
        let n = ((frac.clamp(0.0, 1.0) * total as f64).round() as usize).min(total);
        let idx = reservoir_indices(total, n, seed);
        SeraDFrame_ {
            inner: Arc::new(self.inner.row_take(&idx)),
        }
    }

    #[sera_doc(
        name = "SeraDFrame.chunks",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Splits the frame into a list of row-chunked frames of at most `size` rows each, for streaming/batched processing of datasets too large to transform in one pass.",
        fr = "Decoupe le frame en une liste de frames de `size` lignes maximum chacun, pour un traitement en flux/par lots de datasets trop volumineux pour etre transformes en une seule passe."
    )]
    fn chunks(&self, size: usize) -> Vec<SeraDFrame_> {
        let size = size.max(1);
        let total = self.inner.nrows;
        (0..total)
            .step_by(size)
            .map(|start| {
                let end = (start + size).min(total);
                let idx: Vec<usize> = (start..end).collect();
                SeraDFrame_ {
                    inner: Arc::new(self.inner.row_take(&idx)),
                }
            })
            .collect()
    }

    #[pyo3(signature = (col, q, max_samples = 200_000, seed = None))]
    #[sera_doc(
        name = "SeraDFrame.approx_quantile",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Approximate quantile via reservoir sampling: on columns larger than max_samples, computes the exact quantile of a fixed-size random sample instead of the full O(n) selection quantile() does on every call — a bounded-cost estimate for repeated quantile queries on very large columns.",
        fr = "Quantile approximatif par reservoir sampling : sur les colonnes plus grandes que max_samples, calcule le quantile exact d'un echantillon aleatoire de taille fixe plutot que la selection O(n) complete que fait quantile() a chaque appel — un cout borne pour des requetes de quantile repetees sur de tres grandes colonnes."
    )]
    fn approx_quantile(&self, col: &str, q: f64, max_samples: usize, seed: Option<u64>) -> PyResult<f64> {
        let vals: Vec<f64> = self.inner.get(col)?.to_f64_vec().into_iter().filter(|v| !v.is_nan()).collect();
        let n = vals.len();
        if n == 0 {
            return Ok(f64::NAN);
        }
        let mut sample = if n <= max_samples {
            vals
        } else {
            let idx = reservoir_indices(n, max_samples, seed);
            idx.into_iter().map(|i| vals[i]).collect()
        };
        let m = sample.len();
        if m == 1 {
            return Ok(sample[0]);
        }
        let q = q.clamp(0.0, 1.0);
        let pos = q * (m - 1) as f64;
        let lo = pos.floor() as usize;
        let hi = pos.ceil() as usize;
        let cmp = |a: &f64, b: &f64| a.partial_cmp(b).unwrap();
        sample.select_nth_unstable_by(lo, cmp);
        let lo_val = sample[lo];
        if hi == lo {
            return Ok(lo_val);
        }
        let (_, hi_val, _) = sample.select_nth_unstable_by(hi, cmp);
        let hi_val = *hi_val;
        let frac = pos - lo as f64;
        Ok(lo_val + (hi_val - lo_val) * frac)
    }

    #[sera_doc(
        name = "SeraDFrame.nunique_all",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Distinct-value count for every column at once, computed in parallel across columns — a one-call data-profiling summary instead of one nunique(col) per column.",
        fr = "Nombre de valeurs distinctes pour toutes les colonnes a la fois, calcule en parallele sur les colonnes — un resume de profilage en un seul appel plutot qu'un nunique(col) par colonne."
    )]
    fn nunique_all(&self) -> HashMap<String, usize> {
        self.inner
            .order
            .par_iter()
            .map(|name| {
                let series = &self.inner.columns[name];
                let count = match series {
                    Series::Num(v) => {
                        let mut seen: std::collections::HashSet<u64, FxBuildHasher> =
                            std::collections::HashSet::with_capacity_and_hasher(v.len(), FxBuildHasher::default());
                        for x in v.iter() {
                            seen.insert(x.to_bits());
                        }
                        seen.len()
                    }
                    Series::Str(v) => {
                        let seen: std::collections::HashSet<&str, FxBuildHasher> = v.iter().map(|s| s.as_ref()).collect();
                        seen.len()
                    }
                    Series::Bool(v) => {
                        let seen: std::collections::HashSet<bool, FxBuildHasher> = v.iter().copied().collect();
                        seen.len()
                    }
                };
                (name.clone(), count)
            })
            .collect()
    }

    #[pyo3(signature = (col, bins = 10))]
    #[sera_doc(
        name = "SeraDFrame.histogram",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Single-pass, fixed-bin-count histogram of a numeric column: returns a SeraDFrame with bin_low, bin_high and count columns. O(n) counting rather than sorting, so it stays cheap regardless of column size.",
        fr = "Histogramme a nombre de bins fixe, en une seule passe, d'une colonne numerique : retourne un SeraDFrame avec les colonnes bin_low, bin_high et count. Comptage O(n) plutot qu'un tri, donc le cout reste faible quelle que soit la taille de la colonne."
    )]
    fn histogram(&self, col: &str, bins: usize) -> PyResult<SeraDFrame_> {
        let vals: Vec<f64> = self.inner.get(col)?.to_f64_vec().into_iter().filter(|v| !v.is_nan()).collect();
        let bins = bins.max(1);
        if vals.is_empty() {
            let columns: HashMap<String, Series> = HashMap::from([
                ("bin_low".to_string(), Series::Num(Arc::new(Vec::new()))),
                ("bin_high".to_string(), Series::Num(Arc::new(Vec::new()))),
                ("count".to_string(), Series::Num(Arc::new(Vec::new()))),
            ]);
            return Ok(SeraDFrame_ {
                inner: Arc::new(SeraDFrame::from_parts(
                    vec!["bin_low".to_string(), "bin_high".to_string(), "count".to_string()],
                    columns,
                    0,
                )),
            });
        }
        let min_v = vals.par_iter().cloned().reduce(|| f64::INFINITY, f64::min);
        let max_v = vals.par_iter().cloned().reduce(|| f64::NEG_INFINITY, f64::max);
        let span = (max_v - min_v).max(1e-12);
        let width = span / bins as f64;
        let counts: Vec<f64> = {
            let mut acc = vec![0u64; bins];
            for v in &vals {
                let mut b = (((v - min_v) / width) as usize).min(bins - 1);
                if b >= bins {
                    b = bins - 1;
                }
                acc[b] += 1;
            }
            acc.into_iter().map(|c| c as f64).collect()
        };
        let bin_low: Vec<f64> = (0..bins).map(|i| min_v + i as f64 * width).collect();
        let bin_high: Vec<f64> = (0..bins).map(|i| min_v + (i + 1) as f64 * width).collect();
        let columns: HashMap<String, Series> = HashMap::from([
            ("bin_low".to_string(), Series::Num(Arc::new(bin_low))),
            ("bin_high".to_string(), Series::Num(Arc::new(bin_high))),
            ("count".to_string(), Series::Num(Arc::new(counts))),
        ]);
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(
                vec!["bin_low".to_string(), "bin_high".to_string(), "count".to_string()],
                columns,
                bins,
            )),
        })
    }
}
