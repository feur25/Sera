pub struct DataPoint<T: Copy> {
    pub value: T,
    pub label: String,
}

use crate::core::dispatch::{map_par, stats_par, std_dev_par};

impl<T: Copy> DataPoint<T> {
    #[inline]
    pub fn new(value: T, label: impl Into<String>) -> Self {
        Self { value, label: label.into() }
    }

    #[inline]
    pub fn map_value<U: Copy, F: Fn(T) -> U>(&self, f: F) -> DataPoint<U> {
        DataPoint { value: f(self.value), label: self.label.clone() }
    }
}

pub struct Dataset<T: Copy> {
    pub name: String,
    points: Vec<DataPoint<T>>,
}

impl<T: Copy> Dataset<T> {
    #[inline]
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), points: Vec::new() }
    }

    #[inline]
    pub fn with_capacity(name: impl Into<String>, cap: usize) -> Self {
        Self { name: name.into(), points: Vec::with_capacity(cap) }
    }

    #[inline]
    pub fn push(&mut self, value: T, label: impl Into<String>) {
        self.points.push(DataPoint::new(value, label));
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.points.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    #[inline]
    pub fn values(&self) -> impl Iterator<Item = T> + '_ {
        self.points.iter().map(|p| p.value)
    }

    #[inline]
    pub fn labels(&self) -> impl Iterator<Item = &str> + '_ {
        self.points.iter().map(|p| p.label.as_str())
    }

    #[inline]
    pub fn points(&self) -> &[DataPoint<T>] {
        &self.points
    }

    #[inline]
    pub fn map<U: Copy, F: Fn(T) -> U>(&self, f: F) -> Dataset<U> {
        let mut out = Dataset::with_capacity(self.name.clone(), self.points.len());
        for p in &self.points {
            out.push(f(p.value), p.label.as_str());
        }
        out
    }

    #[inline]
    pub fn filter<F: Fn(T) -> bool>(&self, f: F) -> Dataset<T> {
        let mut out = Dataset::new(self.name.clone());
        for p in &self.points {
            if f(p.value) {
                out.push(p.value, p.label.as_str());
            }
        }
        out
    }

    #[inline]
    pub fn into_values(self) -> Vec<T> {
        self.points.into_iter().map(|p| p.value).collect()
    }

    #[inline]
    pub fn into_labels(self) -> Vec<String> {
        self.points.into_iter().map(|p| p.label).collect()
    }

    #[inline]
    pub fn to_values_vec(&self) -> Vec<T> {
        self.points.iter().map(|p| p.value).collect()
    }

    #[inline]
    pub fn to_labels_vec(&self) -> Vec<String> {
        self.points.iter().map(|p| p.label.clone()).collect()
    }
}

impl Dataset<f64> {
    #[inline]
    pub fn min(&self) -> f64 {
        self.values().fold(f64::INFINITY, f64::min)
    }

    #[inline]
    pub fn max(&self) -> f64 {
        self.values().fold(f64::NEG_INFINITY, f64::max)
    }

    #[inline]
    pub fn sum(&self) -> f64 {
        self.values().sum()
    }

    #[inline]
    pub fn mean(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }
        self.sum() / self.points.len() as f64
    }

    #[inline]
    pub fn std_dev(&self) -> f64 {
        if self.points.len() < 2 {
            return 0.0;
        }
        let m = self.mean();
        let variance = self.values().map(|x| (x - m) * (x - m)).sum::<f64>()
            / (self.points.len() - 1) as f64;
        variance.sqrt()
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        let mn = self.min();
        let mx = self.max();
        let range = (mx - mn).max(1e-12);
        self.map(|v| (v - mn) / range)
    }

    #[inline]
    pub fn clamp(&self, lo: f64, hi: f64) -> Self {
        self.map(|v| v.clamp(lo, hi))
    }

    pub fn map_par<F: Fn(f64) -> f64 + Sync + Send>(&self, f: F) -> Dataset<f64> {
        let vals: Vec<f64> = map_par(
            self.points.as_slice(),
            move |p| f(p.value),
        );
        let mut out = Dataset::with_capacity(self.name.clone(), vals.len());
        for (v, p) in vals.into_iter().zip(self.points.iter()) {
            out.push(v, p.label.as_str());
        }
        out
    }

    pub fn par_stats(&self) -> DatasetStats {
        let vals: Vec<f64> = self.points.iter().map(|p| p.value).collect();
        let s = stats_par(&vals);
        let mean = s.mean();
        let std_dev = std_dev_par(&vals, mean);
        DatasetStats { min: s.min, max: s.max, mean, std_dev, sum: s.sum, count: s.count }
    }

    pub fn into_chunks(self, n: usize) -> Vec<Dataset<f64>> {
        let chunk_size = (self.points.len() + n.max(1) - 1) / n.max(1);
        if chunk_size == 0 { return vec![self]; }
        self.points
            .chunks(chunk_size)
            .map(|chunk| {
                let mut ds = Dataset::with_capacity(self.name.clone(), chunk.len());
                for p in chunk { ds.push(p.value, p.label.as_str()); }
                ds
            })
            .collect()
    }
}

pub struct DatasetStats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub std_dev: f64,
    pub sum: f64,
    pub count: usize,
}

impl From<Vec<f64>> for Dataset<f64> {
    #[inline]
    fn from(values: Vec<f64>) -> Self {
        let mut ds = Dataset::with_capacity("", values.len());
        for (i, v) in values.into_iter().enumerate() {
            ds.push(v, i.to_string());
        }
        ds
    }
}

impl From<Dataset<f64>> for Vec<f64> {
    #[inline]
    fn from(ds: Dataset<f64>) -> Self {
        ds.into_values()
    }
}

impl From<(Vec<f64>, Vec<String>)> for Dataset<f64> {
    #[inline]
    fn from((values, labels): (Vec<f64>, Vec<String>)) -> Self {
        let n = values.len().min(labels.len());
        let mut ds = Dataset::with_capacity("", n);
        for i in 0..n {
            ds.push(values[i], labels[i].as_str());
        }
        ds
    }
}

impl FromIterator<(f64, String)> for Dataset<f64> {
    fn from_iter<I: IntoIterator<Item = (f64, String)>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let (hint, _) = iter.size_hint();
        let mut ds = Dataset::with_capacity("", hint);
        for (v, l) in iter {
            ds.push(v, l);
        }
        ds
    }
}

impl<T: Copy + Default> Default for Dataset<T> {
    fn default() -> Self {
        Self::new("")
    }
}