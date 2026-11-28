use crate::sera_doc_impl;
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pyclass(name = "DatasetStats", module = "seraplot")]
pub struct PyDatasetStats {
    #[pyo3(get)]
    pub min: f64,
    #[pyo3(get)]
    pub max: f64,
    #[pyo3(get)]
    pub mean: f64,
    #[pyo3(get)]
    pub std_dev: f64,
    #[pyo3(get)]
    pub sum: f64,
    #[pyo3(get)]
    pub count: usize,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyDatasetStats {
    fn __repr__(&self) -> String {
        format!(
            "DatasetStats(min={:.4}, max={:.4}, mean={:.4}, std_dev={:.4}, count={})",
            self.min, self.max, self.mean, self.std_dev, self.count
        )
    }
}

#[cfg(feature = "python")]
#[pyclass(name = "Dataset", module = "seraplot")]
pub struct PyDataset {
    inner: crate::data::Dataset<f64>,
}

#[cfg(feature = "python")]
#[sera_doc_impl]
#[pymethods]
impl PyDataset {
    #[staticmethod]
    #[sera_doc(
        category = "data",
        file = "api/dataset.md",
        en = "Creates a Dataset from a Python list of float values.",
        fr = "Crée un Dataset à partir d'une liste Python de valeurs float.",
        param(
            name = "values",
            ty = "list[float]",
            en = "List of numeric values.",
            fr = "Liste de valeurs numériques."
        )
    )]
    #[pyo3(signature = (values, labels=None))]
    fn from_list(values: Vec<f64>, labels: Option<Vec<String>>) -> Self {
        let mut ds = crate::data::Dataset::with_capacity("dataset", values.len());
        for (i, v) in values.iter().enumerate() {
            let lbl = labels
                .as_ref()
                .and_then(|l| l.get(i))
                .map(|s| s.as_str())
                .unwrap_or("")
                .to_string();
            ds.push(*v, lbl);
        }
        PyDataset { inner: ds }
    }

    #[sera_doc(
        category = "data",
        file = "api/dataset.md",
        en = "Computes descriptive statistics in parallel: min, max, mean, variance, std_dev, sum, count.",
        fr = "Calcule des statistiques descriptives en parallèle: min, max, mean, variance, std_dev, sum, count."
    )]
    fn par_stats(&self) -> PyDatasetStats {
        let s = self.inner.par_stats();
        PyDatasetStats {
            min: s.min,
            max: s.max,
            mean: s.mean,
            std_dev: s.std_dev,
            sum: s.sum,
            count: s.count,
        }
    }

    #[sera_doc(
        category = "data",
        file = "api/dataset.md",
        en = "Splits the dataset into n equal-sized chunks and returns them as a list of Dataset objects.",
        fr = "Divise le dataset en n morceaux de taille égale et les retourne sous forme de liste d'objets Dataset.",
        param(
            name = "n",
            ty = "int",
            en = "Number of chunks to split into.",
            fr = "Nombre de morceaux en lesquels diviser."
        )
    )]
    fn into_chunks(&self, n: usize) -> Vec<PyDataset> {
        let vals: Vec<f64> = self.inner.values().collect();
        let labels: Vec<String> = self.inner.labels().map(|s| s.to_string()).collect();
        if n == 0 || vals.is_empty() {
            return vec![];
        }
        let chunk_size = (vals.len() + n - 1) / n;
        vals.chunks(chunk_size)
            .enumerate()
            .map(|(ci, chunk)| {
                let lbl_slice = &labels[ci * chunk_size..ci * chunk_size + chunk.len()];
                let mut ds =
                    crate::data::Dataset::with_capacity(&format!("chunk_{ci}"), chunk.len());
                for (v, l) in chunk.iter().zip(lbl_slice.iter()) {
                    ds.push(*v, l.as_str());
                }
                PyDataset { inner: ds }
            })
            .collect()
    }

    fn values(&self) -> Vec<f64> {
        self.inner.values().collect()
    }

    fn labels(&self) -> Vec<String> {
        self.inner.labels().map(|s| s.to_string()).collect()
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }

    fn __repr__(&self) -> String {
        format!(
            "Dataset(name={:?}, len={})",
            self.inner.name,
            self.inner.len()
        )
    }
}
