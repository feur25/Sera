#[pyclass(module = "seraplot", name = "DBSCAN")]
pub struct DbscanModel(DbscanState);

#[pymethods]
impl DbscanModel {
    #[new]
    #[pyo3(signature = (eps=0.5, min_samples=5))]
    pub fn py_new(eps: f64, min_samples: usize) -> Self {
        DbscanModel(DbscanState::new(eps, min_samples))
    }
    #[pyo3(signature = (x))]
    pub fn fit(&mut self, x: &Bound<'_, PyAny>) -> PyResult<()> {
        let t = std::time::Instant::now();
        with_flat(x, |xf, n, d| {
            let fp = crate::ml::cache::Fp::new("DBSCAN.fit").f64(self.0.eps).usize(self.0.min_samples).usize(n).usize(d).finish();
            let _h = crate::ml::cache::TaskHandle::auto("DBSCAN.fit", fp);
            self.0.fit_flat(xf, n, d);
            _h.complete(&crate::ml::cache::PartialState::default());
        })?;
        crate::telemetry::record(crate::telemetry::TelemetryEvent::new("DBSCAN.fit", t.elapsed().as_secs_f64() * 1000.0).with_algorithm("DBSCAN"));
        Ok(())
    }
    #[pyo3(signature = (x))]
    pub fn fit_predict(&mut self, py: Python<'_>, x: &Bound<'_, PyAny>) -> PyResult<pyo3::PyObject> {
        self.fit(x)?;
        Ok(vec_i32_to_np(py, self.0.labels.clone()))
    }
    #[getter] pub fn labels_(&self) -> Vec<i32> { self.0.labels.clone() }
    #[getter] pub fn n_clusters_(&self) -> usize { self.0.n_clusters }
    #[getter] pub fn n_noise_(&self) -> usize { self.0.n_noise }
    #[getter] pub fn eps(&self) -> f64 { self.0.eps }
    #[getter] pub fn min_samples(&self) -> usize { self.0.min_samples }
    fn doc(&self) -> &'static str { crate::bindings::commands::docs::DOC_DBSCAN_MODEL }
    fn __repr__(&self) -> String {
        format!("DBSCAN(eps={}, min_samples={}) -> {} clusters, {} noise",
            self.0.eps, self.0.min_samples, self.0.n_clusters, self.0.n_noise)
    }
}

#[pyclass(module = "seraplot", name = "KMeans")]
pub struct KMeansModel(KMeansState);

#[pymethods]
impl KMeansModel {
    #[new]
    #[pyo3(signature = (k=3, max_iter=300, tol=1e-4, mini_batch=false, batch_size=1000, n_init=10, n_clusters=None, random_state=None))]
    pub fn py_new(k: usize, max_iter: usize, tol: f64, mini_batch: bool, batch_size: usize, n_init: usize, n_clusters: Option<usize>, random_state: Option<i64>) -> Self {
        let _ = random_state;
        let kk = n_clusters.unwrap_or(k);
        KMeansModel(KMeansState::new(kk, max_iter, tol, mini_batch, batch_size, n_init))
    }
    #[pyo3(signature = (x))]
    pub fn fit(&mut self, x: &Bound<'_, PyAny>) -> PyResult<()> {
        let t = std::time::Instant::now();
        with_flat(x, |xf, n, d| {
            let fp = crate::ml::cache::Fp::new("KMeans.fit").usize(self.0.k).usize(self.0.max_iter).bval(self.0.mini_batch).data(xf, n, d).finish();
            let _h = crate::ml::cache::TaskHandle::auto("KMeans.fit", fp);
            self.0.fit_flat(xf, n, d);
            _h.complete(&crate::ml::cache::PartialState::default());
        })?;
        crate::telemetry::record(crate::telemetry::TelemetryEvent::new("KMeans.fit", t.elapsed().as_secs_f64() * 1000.0).with_algorithm("KMeans"));
        Ok(())
    }
    #[pyo3(signature = (x))]
    pub fn fit_predict(&mut self, py: Python<'_>, x: &Bound<'_, PyAny>) -> PyResult<pyo3::PyObject> {
        self.fit(x)?;
        Ok(vec_i32_to_np(py, self.0.labels.clone()))
    }
    #[pyo3(signature = (x))]
    pub fn transform(&self, py: Python<'_>, x: &Bound<'_, PyAny>) -> PyResult<pyo3::PyObject> {
        let (flat, n, dims) = extract_flat(x)?;
        vv_to_np2d(py, self.0.transform_flat(&flat, n, dims))
    }
    #[pyo3(signature = (x))]
    pub fn predict(&self, py: Python<'_>, x: &Bound<'_, PyAny>) -> PyResult<pyo3::PyObject> {
        use numpy::PyReadonlyArray2;
        if let Ok(arr) = x.extract::<PyReadonlyArray2<f64>>() {
            let view = arr.as_array();
            let shape = view.shape();
            let (n, dims) = (shape[0], shape[1]);
            if view.is_standard_layout() {
                let s = view.as_slice().unwrap();
                return Ok(vec_i32_to_np(py, self.0.predict_flat(s, n, dims)));
            }
        }
        let (flat, n, dims) = extract_flat(x)?;
        Ok(vec_i32_to_np(py, self.0.predict_flat(&flat, n, dims)))
    }
    #[getter] pub fn labels_(&self) -> Vec<i32> { self.0.labels.clone() }
    #[getter] pub fn centroids_(&self) -> Vec<Vec<f64>> { self.0.centroids.clone() }
    #[getter] pub fn cluster_centers_(&self) -> Vec<Vec<f64>> { self.0.centroids.clone() }
    #[getter] pub fn inertia_(&self) -> f64 { self.0.inertia }
    #[getter] pub fn n_iter_(&self) -> usize { self.0.n_iter }
    #[getter] pub fn n_clusters(&self) -> usize { self.0.centroids.len() }
    #[getter] pub fn n_clusters_(&self) -> usize { self.0.centroids.len() }
    #[getter] pub fn k(&self) -> usize { self.0.k }
    fn doc(&self) -> &'static str { crate::bindings::commands::docs::DOC_KMEANS_MODEL }
    fn __repr__(&self) -> String {
        format!("KMeans(k={}, max_iter={}, n_init={}, mini_batch={}) -> inertia={:.2}",
            self.0.k, self.0.max_iter, self.0.n_init, self.0.mini_batch, self.0.inertia)
    }
}

