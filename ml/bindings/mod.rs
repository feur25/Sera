pub mod anomaly;
pub mod clustering;
pub mod decomposition;
pub mod ensemble;
pub mod helpers;
pub mod importance;
pub mod linear;
pub mod metrics;
pub mod model_selection;
pub mod naive_bayes;
pub mod neighbors;
pub mod persistence;
pub mod preprocessing;
pub mod svm;
pub mod tree;

pub use anomaly::*;
pub use clustering::*;
pub use decomposition::*;
pub use ensemble::*;
pub use importance::*;
pub use linear::*;
pub use metrics::*;
pub use model_selection::*;
pub use naive_bayes::*;
pub use neighbors::*;
pub use persistence::*;
pub use preprocessing::*;
pub use svm::*;
pub use tree::*;

#[cfg(feature = "python")]
mod python_bindings {
    use pyo3::prelude::*;
    use numpy::{PyReadonlyArray1, PyReadonlyArray2, PyUntypedArrayMethods, PyArrayMethods, IntoPyArray};
    use pyo3::types::PyAny;
    use rayon::prelude::*;
    use super::*;

    // Original Python bindings (kept intact) start here

macro_rules! impl_doc {
    ($name:ident, $doc:expr) => {
        #[pymethods]
        impl $name {
            fn doc(&self) -> &'static str {
                $doc
            }
        }
    };
}

pub fn flat_to_np2d(py: Python<'_>, flat: Vec<f64>, n: usize, cols: usize) -> PyResult<PyObject> {
    if n == 0 || cols == 0 { return Ok(numpy::PyArray2::<f64>::zeros_bound(py, [0, 0], false).into_py(py)); }
    let arr = flat.into_pyarray_bound(py);
    Ok(arr.reshape([n, cols])?.into_py(py))
}

pub fn vv_to_np2d(py: Python<'_>, data: Vec<Vec<f64>>) -> PyResult<PyObject> {
    let n = data.len();
    if n == 0 { return Ok(numpy::PyArray2::<f64>::zeros_bound(py, [0, 0], false).into_py(py)); }
    let cols = data[0].len();
    let flat: Vec<f64> = data.into_iter().flat_map(|r| r).collect();
    flat_to_np2d(py, flat, n, cols)
}

pub fn vec_f64_to_np(py: Python<'_>, v: Vec<f64>) -> PyObject {
    v.into_pyarray_bound(py).into_py(py)
}

pub fn vec_i32_to_np(py: Python<'_>, v: Vec<i32>) -> PyObject {
    v.into_pyarray_bound(py).into_py(py)
}

pub fn extract_flat(x: &PyAny) -> PyResult<(Vec<f64>, usize, usize)> {
    if let Ok(arr) = x.extract::<PyReadonlyArray2<f64>>() {
        let shape = arr.shape();
        let (n, p) = (shape[0], shape[1]);
        let view = arr.as_array();
        if view.is_standard_layout() { return Ok((view.as_slice().unwrap().to_vec(), n, p)); }
        let c = view.as_standard_layout();
        return Ok((c.as_slice().unwrap().to_vec(), n, p));
    }
    if let Ok(arr) = x.extract::<PyReadonlyArray2<f32>>() {
        let shape = arr.shape();
        let (n, p) = (shape[0], shape[1]);
        let view = arr.as_array();
        if view.is_standard_layout() { return Ok((view.as_slice().unwrap().iter().map(|&v| v as f64).collect(), n, p)); }
        let c = view.as_standard_layout();
        return Ok((c.as_slice().unwrap().iter().map(|&v| v as f64).collect(), n, p));
    }
    let rows: Vec<Vec<f64>> = x.extract()?;
    let n = rows.len();
    if n == 0 { return Ok((Vec::new(), 0, 0)); }
    let p = rows[0].len();
    let mut flat = vec![0.0f64; n * p];
    for (i, row) in rows.iter().enumerate() { flat[i * p..(i + 1) * p].copy_from_slice(row); }
    Ok((flat, n, p))
}

pub fn with_flat<R>(x: &PyAny, f: impl FnOnce(&[f64], usize, usize) -> R) -> PyResult<R> {
    if let Ok(arr) = x.extract::<PyReadonlyArray2<f64>>() {
        let shape = arr.shape();
        let (n, p) = (shape[0], shape[1]);
        let view = arr.as_array();
        if view.is_standard_layout() {
            return Ok(f(view.as_slice().unwrap(), n, p));
        }
        let c = view.as_standard_layout();
        return Ok(f(c.as_slice().unwrap(), n, p));
    }
    if let Ok(arr) = x.extract::<PyReadonlyArray2<f32>>() {
        let shape = arr.shape();
        let (n, p) = (shape[0], shape[1]);
        let view = arr.as_array();
        let src: &[f32] = if view.is_standard_layout() { view.as_slice().unwrap() } else {
            let c = view.as_standard_layout();
            let s = c.as_slice().unwrap();
            let mut v = vec![0.0f64; n * p];
            v.par_chunks_mut(8192).enumerate().for_each(|(ci, chunk)| {
                let s0 = ci * 8192;
                for (k, d) in chunk.iter_mut().enumerate() { *d = unsafe { *s.get_unchecked(s0 + k) } as f64; }
            });
            return Ok(f(&v, n, p));
        };
        let mut v = vec![0.0f64; n * p];
        v.par_chunks_mut(8192).enumerate().for_each(|(ci, chunk)| {
            let s0 = ci * 8192;
            for (k, d) in chunk.iter_mut().enumerate() { *d = unsafe { *src.get_unchecked(s0 + k) } as f64; }
        });
        return Ok(f(&v, n, p));
    }
    let (v, n, p) = extract_flat(x)?;
    Ok(f(&v, n, p))
}

fn extract_labels(y: &PyAny) -> PyResult<Vec<i32>> {
    if let Ok(arr) = y.extract::<PyReadonlyArray1<i64>>() { return Ok(arr.as_slice().unwrap().iter().map(|&v| v as i32).collect()); }
    if let Ok(arr) = y.extract::<PyReadonlyArray1<i32>>() { return Ok(arr.as_slice().unwrap().to_vec()); }
    if let Ok(arr) = y.extract::<PyReadonlyArray1<f64>>() { return Ok(arr.as_slice().unwrap().iter().map(|&v| v as i32).collect()); }
    y.extract::<Vec<i32>>()
}

fn extract_targets(y: &PyAny) -> PyResult<Vec<f64>> {
    if let Ok(arr) = y.extract::<PyReadonlyArray1<f64>>() { return Ok(arr.as_slice().unwrap().to_vec()); }
    if let Ok(arr) = y.extract::<PyReadonlyArray1<f32>>() { return Ok(arr.as_slice().unwrap().iter().map(|&v| v as f64).collect()); }
    if let Ok(arr) = y.extract::<PyReadonlyArray1<i64>>() { return Ok(arr.as_slice().unwrap().iter().map(|&v| v as f64).collect()); }
    y.extract::<Vec<f64>>()
}

pub fn extract_estimator_name(est: &PyAny) -> PyResult<String> {
    if let Ok(s) = est.extract::<String>() { return Ok(s); }
    if let Ok(t) = est.getattr("__class__") {
        if let Ok(n) = t.getattr("__name__") {
            if let Ok(s) = n.extract::<String>() { return Ok(s); }
        }
    }
    Err(pyo3::exceptions::PyTypeError::new_err("estimator must be a string name or seraplot estimator instance"))
}

macro_rules! impl_get_set_params {
    ($name:ident, $( $param:expr => $($path:ident).+ , $t:ty );+ $(;)?) => {
        #[pymethods]
        impl $name {
            fn get_params(&self, _py: Python<'_>) -> PyResult<std::collections::HashMap<String, PyObject>> {
                let mut m = std::collections::HashMap::new();
                Python::with_gil(|py| { $( m.insert($param.to_string(), (self.$($path).+).into_py(py)); )+ });
                Ok(m)
            }
            fn set_params(&mut self, params: std::collections::HashMap<String, PyObject>) -> PyResult<()> {
                Python::with_gil(|py| { $( if let Some(v) = params.get($param) { self.$($path).+ = v.extract::<$t>(py).unwrap_or(self.$($path).+); } )+ });
                Ok(())
            }
        }
    };
}

macro_rules! impl_reg_fit_predict_score {
    ($name:ident) => {
        #[pymethods]
        impl $name {
            fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
                let yt = extract_targets(y)?;
                with_flat(x, |xf, n, p| {
                    let fp = crate::ml::cache::Fp::new(concat!(stringify!($name), ".fit"))
                        .str(&self.__repr__()).data(xf, n, p).targets(&yt).finish();
                    let _h = crate::ml::cache::TaskHandle::auto(concat!(stringify!($name), ".fit"), fp);
                    self.inner.fit(xf, n, p, &yt);
                    _h.complete(&crate::ml::cache::PartialState::default());
                })?;
                Ok(())
            }
            fn predict(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
                let v = with_flat(x, |xf, n, p| self.inner.predict(xf, n, p))?;
                Ok(vec_f64_to_np(py, v))
            }
            fn score(&self, x: &PyAny, y: &PyAny) -> PyResult<f64> {
                let yt = extract_targets(y)?;
                let preds = with_flat(x, |xf, n, p| self.inner.predict(xf, n, p))?;
                Ok(crate::ml::metrics::regression::r2_score(&yt, &preds))
            }
        }
    };
}

macro_rules! impl_cls_fit_predict_score {
    ($name:ident) => {
        #[pymethods]
        impl $name {
            fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
                let yl = extract_labels(y)?;
                with_flat(x, |xf, n, p| {
                    let fp = crate::ml::cache::Fp::new(concat!(stringify!($name), ".fit"))
                        .str(&self.__repr__()).data(xf, n, p).labels(&yl).finish();
                    let _h = crate::ml::cache::TaskHandle::auto(concat!(stringify!($name), ".fit"), fp);
                    self.inner.fit(xf, n, p, &yl);
                    _h.complete(&crate::ml::cache::PartialState::default());
                })?;
                Ok(())
            }
            fn predict(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
                let v = with_flat(x, |xf, n, p| self.inner.predict(xf, n, p))?;
                Ok(vec_i32_to_np(py, v))
            }
            fn score(&self, x: &PyAny, y: &PyAny) -> PyResult<f64> {
                let yl = extract_labels(y)?;
                let preds = with_flat(x, |xf, n, p| self.inner.predict(xf, n, p))?;
                Ok(crate::ml::metrics::classification::accuracy_score(&yl, &preds))
            }
        }
    };
}

#[pyclass(module = "seraplot", name = "LinearRegression")]
pub struct PyLinearRegression { inner: crate::ml::linear::ols::LinearRegression }

impl_reg_fit_predict_score!(PyLinearRegression);
impl_get_set_params!(PyLinearRegression, "fit_intercept" => inner.fit_intercept, bool;);

#[pymethods]
impl PyLinearRegression {
    #[new]
    #[pyo3(signature = (fit_intercept=true))]
    fn py_new(fit_intercept: bool) -> Self { Self { inner: crate::ml::linear::ols::LinearRegression::new(fit_intercept) } }
    #[getter] fn coef_(&self) -> Vec<f64> { self.inner.coef.clone() }
    #[getter] fn intercept_(&self) -> f64 { self.inner.intercept }
    #[getter] fn fit_intercept_(&self) -> bool { self.inner.fit_intercept }
    #[getter] fn n_features_in_(&self) -> usize { self.inner.coef.len() }
    fn __repr__(&self) -> String { "LinearRegression()".to_string() }
}

#[pyclass(module = "seraplot", name = "Ridge")]
pub struct PyRidge { inner: crate::ml::linear::ridge::Ridge }

impl_reg_fit_predict_score!(PyRidge);
impl_get_set_params!(PyRidge, "alpha" => inner.alpha, f64; "fit_intercept" => inner.fit_intercept, bool;);

#[pymethods]
impl PyRidge {
    #[new]
    #[pyo3(signature = (alpha=1.0, fit_intercept=true))]
    fn py_new(alpha: f64, fit_intercept: bool) -> Self { Self { inner: crate::ml::linear::ridge::Ridge::new(alpha, fit_intercept) } }
    #[getter] fn coef_(&self) -> Vec<f64> { self.inner.coef.clone() }
    #[getter] fn intercept_(&self) -> f64 { self.inner.intercept }
    #[getter] fn alpha_(&self) -> f64 { self.inner.alpha }
    #[getter] fn fit_intercept_(&self) -> bool { self.inner.fit_intercept }
    fn __repr__(&self) -> String { format!("Ridge(alpha={})", self.inner.alpha) }
}

#[pyclass(module = "seraplot", name = "Lasso")]
pub struct PyLasso { inner: crate::ml::linear::lasso::Lasso }

#[pymethods]
impl PyLasso {
    #[new]
    #[pyo3(signature = (alpha=1.0, max_iter=1000, tol=1e-4, fit_intercept=true))]
    fn py_new(alpha: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> Self {
        Self { inner: crate::ml::linear::lasso::Lasso::new(alpha, max_iter, tol, fit_intercept) }
    }
    #[pyo3(signature = (x, y, checkpoint_id=None))]
    fn fit(&mut self, x: &PyAny, y: &PyAny, checkpoint_id: Option<u64>) -> PyResult<()> {
        let yt = extract_targets(y)?;
        with_flat(x, |xf, n, p| self.inner.fit_resumable(xf, n, p, &yt, checkpoint_id))?;
        Ok(())
    }
    fn predict(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let v = with_flat(x, |xf, n, p| self.inner.predict(xf, n, p))?;
        Ok(vec_f64_to_np(py, v))
    }
    fn score(&self, x: &PyAny, y: &PyAny) -> PyResult<f64> {
        let yt = extract_targets(y)?;
        let preds = with_flat(x, |xf, n, p| self.inner.predict(xf, n, p))?;
        Ok(crate::ml::linear::ols::r2_score_pub(&yt, &preds))
    }
    fn get_params(&self, _py: Python<'_>) -> PyResult<std::collections::HashMap<String, PyObject>> {
        let mut m = std::collections::HashMap::new();
        Python::with_gil(|py| {
            m.insert("alpha".to_string(), self.inner.alpha().into_py(py));
            m.insert("max_iter".to_string(), self.inner.max_iter().into_py(py));
            m.insert("tol".to_string(), self.inner.tol().into_py(py));
            m.insert("fit_intercept".to_string(), self.inner.fit_intercept().into_py(py));
        });
        Ok(m)
    }
    fn set_params(&mut self, params: std::collections::HashMap<String, PyObject>) -> PyResult<()> {
        Python::with_gil(|py| {
            if let Some(v) = params.get("alpha") { if let Ok(val) = v.extract::<f64>(py) { self.inner.set_alpha(val); } }
            if let Some(v) = params.get("max_iter") { if let Ok(val) = v.extract::<usize>(py) { self.inner.set_max_iter(val); } }
            if let Some(v) = params.get("tol") { if let Ok(val) = v.extract::<f64>(py) { self.inner.set_tol(val); } }
            if let Some(v) = params.get("fit_intercept") { if let Ok(val) = v.extract::<bool>(py) { self.inner.set_fit_intercept(val); } }
        });
        Ok(())
    }
    #[getter] fn coef_(&self) -> Vec<f64> { self.inner.coef().to_vec() }
    #[getter] fn intercept_(&self) -> f64 { self.inner.intercept() }
    #[getter] fn alpha_(&self) -> f64 { self.inner.alpha() }
    #[getter] fn max_iter_(&self) -> usize { self.inner.max_iter() }
    #[getter] fn tol_(&self) -> f64 { self.inner.tol() }
    #[getter] fn fit_intercept_(&self) -> bool { self.inner.fit_intercept() }
    #[getter] fn n_iter_(&self) -> usize { self.inner.n_iter() }
    fn __repr__(&self) -> String { format!("Lasso(alpha={})", self.inner.alpha()) }
}

#[pyclass(module = "seraplot", name = "ElasticNet")]
pub struct PyElasticNet { inner: crate::ml::linear::elastic_net::ElasticNet }

impl_get_set_params!(PyElasticNet,
    "alpha" => inner.alpha, f64;
    "l1_ratio" => inner.l1_ratio, f64;
    "max_iter" => inner.max_iter, usize;
    "tol" => inner.tol, f64;
    "fit_intercept" => inner.fit_intercept, bool;
);

#[pymethods]
impl PyElasticNet {
    #[new]
    #[pyo3(signature = (alpha=1.0, l1_ratio=0.5, max_iter=1000, tol=1e-4, fit_intercept=true))]
    fn py_new(alpha: f64, l1_ratio: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> Self {
        Self { inner: crate::ml::linear::elastic_net::ElasticNet::new(alpha, l1_ratio, max_iter, tol, fit_intercept) }
    }
    #[pyo3(signature = (x, y, checkpoint_id=None))]
    fn fit(&mut self, x: &PyAny, y: &PyAny, checkpoint_id: Option<u64>) -> PyResult<()> {
        let yt = extract_targets(y)?;
        with_flat(x, |xf, n, p| {
            let fp = crate::ml::cache::Fp::new("ElasticNet.fit").str(&self.__repr__()).data(xf, n, p).targets(&yt).finish();
            let _h = crate::ml::cache::TaskHandle::auto("ElasticNet.fit", fp);
            let ckpt = checkpoint_id.or(Some(_h.id));
            self.inner.fit_resumable(xf, n, p, &yt, ckpt);
            _h.complete(&crate::ml::cache::PartialState::default());
        })?;
        Ok(())
    }
    fn predict(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let v = with_flat(x, |xf, n, p| self.inner.predict(xf, n, p))?;
        Ok(vec_f64_to_np(py, v))
    }
    fn score(&self, x: &PyAny, y: &PyAny) -> PyResult<f64> {
        let yt = extract_targets(y)?;
        let preds = with_flat(x, |xf, n, p| self.inner.predict(xf, n, p))?;
        Ok(crate::ml::linear::ols::r2_score_pub(&yt, &preds))
    }
    #[getter] fn coef_(&self) -> Vec<f64> { self.inner.coef.clone() }
    #[getter] fn intercept_(&self) -> f64 { self.inner.intercept }
    #[getter] fn n_iter_(&self) -> usize { self.inner.n_iter }
    #[getter] fn alpha_(&self) -> f64 { self.inner.alpha }
    #[getter] fn l1_ratio_(&self) -> f64 { self.inner.l1_ratio }
    #[getter] fn max_iter_(&self) -> usize { self.inner.max_iter }
    #[getter] fn tol_(&self) -> f64 { self.inner.tol }
    #[getter] fn fit_intercept_(&self) -> bool { self.inner.fit_intercept }
    fn __repr__(&self) -> String { format!("ElasticNet(alpha={}, l1_ratio={})", self.inner.alpha, self.inner.l1_ratio) }
}

#[pyclass(module = "seraplot", name = "LogisticRegression")]
pub struct PyLogisticRegression { inner: crate::ml::linear::logistic::LogisticRegression }

impl_get_set_params!(PyLogisticRegression,
    "C" => inner.c, f64;
    "max_iter" => inner.max_iter, usize;
    "tol" => inner.tol, f64;
    "fit_intercept" => inner.fit_intercept, bool;
);

#[pymethods]
impl PyLogisticRegression {
    #[allow(non_snake_case)]
    #[new]
    #[pyo3(signature = (C=1.0, max_iter=1000, tol=1e-4, fit_intercept=true))]
    fn py_new(C: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> Self {
        Self { inner: crate::ml::linear::logistic::LogisticRegression::new(C, max_iter, tol, fit_intercept) }
    }
    #[pyo3(signature = (x, y, checkpoint_id=None))]
    fn fit(&mut self, x: &PyAny, y: &PyAny, checkpoint_id: Option<u64>) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        let fp = crate::ml::cache::Fp::new("LogisticRegression.fit").str(&self.__repr__()).data(&xf, n, p).labels(&yl).finish();
        let _h = crate::ml::cache::TaskHandle::auto("LogisticRegression.fit", fp);
        let ckpt = checkpoint_id.or(Some(_h.id));
        self.inner.fit_resumable(&xf, n, p, &yl, ckpt);
        _h.complete(&crate::ml::cache::PartialState::default());
        Ok(())
    }
    fn predict(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        Ok(vec_i32_to_np(py, self.inner.predict(&xf, n, p)))
    }
    fn predict_proba(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        vv_to_np2d(py, self.inner.predict_proba(&xf, n, p))
    }
    fn score(&self, x: &PyAny, y: &PyAny) -> PyResult<f64> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        let preds = self.inner.predict(&xf, n, p);
        Ok(crate::ml::metrics::classification::accuracy_score(&yl, &preds))
    }
    #[getter] fn classes_(&self) -> Vec<i32> { self.inner.classes.clone() }
    #[getter] fn coef_(&self, py: Python<'_>) -> PyResult<PyObject> {
        if self.inner.multi_coef.is_empty() { Ok(self.inner.coef.clone().into_pyarray_bound(py).into_py(py)) }
        else { vv_to_np2d(py, self.inner.multi_coef.clone()) }
    }
    #[getter] fn intercept_(&self, py: Python<'_>) -> PyResult<PyObject> {
        if self.inner.multi_intercept.is_empty() { Ok(self.inner.intercept.into_py(py)) }
        else { Ok(self.inner.multi_intercept.clone().into_pyarray_bound(py).into_py(py)) }
    }
    #[getter] fn n_iter_(&self) -> usize { self.inner.n_iter }
    #[getter] fn C_(&self) -> f64 { self.inner.c }
    #[getter] fn max_iter_(&self) -> usize { self.inner.max_iter }
    #[getter] fn tol_(&self) -> f64 { self.inner.tol }
    #[getter] fn fit_intercept_(&self) -> bool { self.inner.fit_intercept }
    fn __repr__(&self) -> String { format!("LogisticRegression(C={})", self.inner.c) }
}

#[pyclass(module = "seraplot", name = "SGDClassifier")]
pub struct PySGDClassifier { inner: crate::ml::linear::sgd::SGDClassifier }

#[pymethods]
impl PySGDClassifier {
    #[new]
    #[pyo3(signature = (loss="hinge", alpha=0.0001, max_iter=1000, tol=1e-3, fit_intercept=true, eta0=1.0))]
    fn py_new(loss: &str, alpha: f64, max_iter: usize, tol: f64, fit_intercept: bool, eta0: f64) -> Self {
        let l = match loss {
            "log" | "log_loss" => crate::ml::linear::sgd::SGDLoss::Log,
            "modified_huber" => crate::ml::linear::sgd::SGDLoss::ModifiedHuber,
            "squared_hinge" => crate::ml::linear::sgd::SGDLoss::SquaredHinge,
            _ => crate::ml::linear::sgd::SGDLoss::Hinge,
        };
        Self { inner: crate::ml::linear::sgd::SGDClassifier::new(l, alpha, max_iter, tol, eta0, fit_intercept) }
    }
    #[pyo3(signature = (x, y, checkpoint_id=None))]
    fn fit(&mut self, x: &PyAny, y: &PyAny, checkpoint_id: Option<u64>) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        let fp = crate::ml::cache::Fp::new("SGDClassifier.fit").str(&self.__repr__()).data(&xf, n, p).labels(&yl).finish();
        let _h = crate::ml::cache::TaskHandle::auto("SGDClassifier.fit", fp);
        let ckpt = checkpoint_id.or(Some(_h.id));
        self.inner.fit_resumable(&xf, n, p, &yl, ckpt);
        _h.complete(&crate::ml::cache::PartialState::default());
        Ok(())
    }
    fn predict(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        Ok(vec_i32_to_np(py, self.inner.predict(&xf, n, p)))
    }
    fn score(&self, x: &PyAny, y: &PyAny) -> PyResult<f64> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        let preds = self.inner.predict(&xf, n, p);
        Ok(crate::ml::metrics::classification::accuracy_score(&yl, &preds))
    }
    fn decision_function(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        Ok(vec_f64_to_np(py, self.inner.decision_function(&xf, n, p)))
    }
    fn get_params(&self, _py: Python<'_>) -> PyResult<std::collections::HashMap<String, PyObject>> {
        let mut m = std::collections::HashMap::new();
        Python::with_gil(|py| {
            m.insert("loss".to_string(), self.loss_().into_py(py));
            m.insert("alpha".to_string(), self.inner.alpha.into_py(py));
            m.insert("max_iter".to_string(), self.inner.max_iter.into_py(py));
            m.insert("tol".to_string(), self.inner.tol.into_py(py));
            m.insert("fit_intercept".to_string(), self.inner.fit_intercept.into_py(py));
            m.insert("eta0".to_string(), self.inner.learning_rate.into_py(py));
        });
        Ok(m)
    }
    fn set_params(&mut self, params: std::collections::HashMap<String, PyObject>) -> PyResult<()> {
        Python::with_gil(|py| {
            if let Some(v) = params.get("alpha") { self.inner.alpha = v.extract::<f64>(py).unwrap_or(self.inner.alpha); }
            if let Some(v) = params.get("max_iter") { self.inner.max_iter = v.extract::<usize>(py).unwrap_or(self.inner.max_iter); }
            if let Some(v) = params.get("tol") { self.inner.tol = v.extract::<f64>(py).unwrap_or(self.inner.tol); }
            if let Some(v) = params.get("eta0") { self.inner.learning_rate = v.extract::<f64>(py).unwrap_or(self.inner.learning_rate); }
        });
        Ok(())
    }
    #[getter] fn coef_(&self) -> Vec<f64> { self.inner.coef.clone() }
    #[getter] fn intercept_(&self) -> f64 { self.inner.intercept }
    #[getter] fn classes_(&self) -> Vec<i32> { self.inner.classes.clone() }
    #[getter] fn n_iter_(&self) -> usize { self.inner.n_iter }
    #[getter] fn alpha_(&self) -> f64 { self.inner.alpha }
    #[getter] fn max_iter_(&self) -> usize { self.inner.max_iter }
    #[getter] fn tol_(&self) -> f64 { self.inner.tol }
    #[getter] fn fit_intercept_(&self) -> bool { self.inner.fit_intercept }
    #[getter] fn eta0_(&self) -> f64 { self.inner.learning_rate }
    #[getter] fn loss_(&self) -> &str {
        match self.inner.loss {
            crate::ml::linear::sgd::SGDLoss::Hinge => "hinge",
            crate::ml::linear::sgd::SGDLoss::Log => "log_loss",
            crate::ml::linear::sgd::SGDLoss::ModifiedHuber => "modified_huber",
            crate::ml::linear::sgd::SGDLoss::SquaredHinge => "squared_hinge",
        }
    }
    fn __repr__(&self) -> String { "SGDClassifier()".to_string() }
}

#[pyclass(module = "seraplot", name = "SGDRegressor")]
pub struct PySGDRegressor { inner: crate::ml::linear::sgd::SGDRegressor }

#[pymethods]
impl PySGDRegressor {
    #[new]
    #[pyo3(signature = (loss="squared_error", alpha=0.0001, max_iter=1000, tol=1e-3, fit_intercept=true, eta0=0.01, epsilon=0.1))]
    fn py_new(loss: &str, alpha: f64, max_iter: usize, tol: f64, fit_intercept: bool, eta0: f64, epsilon: f64) -> Self {
        let l = match loss {
            "huber" => crate::ml::linear::sgd::SGDRegLoss::Huber,
            "epsilon_insensitive" => crate::ml::linear::sgd::SGDRegLoss::EpsilonInsensitive,
            _ => crate::ml::linear::sgd::SGDRegLoss::SquaredError,
        };
        let mut inner = crate::ml::linear::sgd::SGDRegressor::new(alpha, max_iter, tol, eta0, fit_intercept);
        inner.loss = l;
        inner.epsilon = epsilon;
        Self { inner }
    }
    #[pyo3(signature = (x, y, checkpoint_id=None))]
    fn fit(&mut self, x: &PyAny, y: &PyAny, checkpoint_id: Option<u64>) -> PyResult<()> {
        let yt = extract_targets(y)?;
        with_flat(x, |xf, n, p| {
            let fp = crate::ml::cache::Fp::new("SGDRegressor.fit").str(&self.__repr__()).data(xf, n, p).targets(&yt).finish();
            let _h = crate::ml::cache::TaskHandle::auto("SGDRegressor.fit", fp);
            let ckpt = checkpoint_id.or(Some(_h.id));
            self.inner.fit_resumable(xf, n, p, &yt, ckpt);
            _h.complete(&crate::ml::cache::PartialState::default());
        })?;
        Ok(())
    }
    fn predict(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let v = with_flat(x, |xf, n, p| self.inner.predict(xf, n, p))?;
        Ok(vec_f64_to_np(py, v))
    }
    fn score(&self, x: &PyAny, y: &PyAny) -> PyResult<f64> {
        let yt = extract_targets(y)?;
        let preds = with_flat(x, |xf, n, p| self.inner.predict(xf, n, p))?;
        Ok(crate::ml::metrics::regression::r2_score(&yt, &preds))
    }
    fn get_params(&self, _py: Python<'_>) -> PyResult<std::collections::HashMap<String, PyObject>> {
        let mut m = std::collections::HashMap::new();
        Python::with_gil(|py| {
            m.insert("loss".to_string(), self.loss_().into_py(py));
            m.insert("alpha".to_string(), self.inner.alpha.into_py(py));
            m.insert("max_iter".to_string(), self.inner.max_iter.into_py(py));
            m.insert("tol".to_string(), self.inner.tol.into_py(py));
            m.insert("fit_intercept".to_string(), self.inner.fit_intercept.into_py(py));
            m.insert("eta0".to_string(), self.inner.learning_rate.into_py(py));
            m.insert("epsilon".to_string(), self.inner.epsilon.into_py(py));
        });
        Ok(m)
    }
    fn set_params(&mut self, params: std::collections::HashMap<String, PyObject>) -> PyResult<()> {
        Python::with_gil(|py| {
            if let Some(v) = params.get("alpha") { self.inner.alpha = v.extract::<f64>(py).unwrap_or(self.inner.alpha); }
            if let Some(v) = params.get("max_iter") { self.inner.max_iter = v.extract::<usize>(py).unwrap_or(self.inner.max_iter); }
            if let Some(v) = params.get("tol") { self.inner.tol = v.extract::<f64>(py).unwrap_or(self.inner.tol); }
            if let Some(v) = params.get("eta0") { self.inner.learning_rate = v.extract::<f64>(py).unwrap_or(self.inner.learning_rate); }
            if let Some(v) = params.get("epsilon") { self.inner.epsilon = v.extract::<f64>(py).unwrap_or(self.inner.epsilon); }
        });
        Ok(())
    }
    #[getter] fn coef_(&self) -> Vec<f64> { self.inner.coef.clone() }
    #[getter] fn intercept_(&self) -> f64 { self.inner.intercept }
    #[getter] fn n_iter_(&self) -> usize { self.inner.n_iter }
    #[getter] fn alpha_(&self) -> f64 { self.inner.alpha }
    #[getter] fn max_iter_(&self) -> usize { self.inner.max_iter }
    #[getter] fn tol_(&self) -> f64 { self.inner.tol }
    #[getter] fn fit_intercept_(&self) -> bool { self.inner.fit_intercept }
    #[getter] fn eta0_(&self) -> f64 { self.inner.learning_rate }
    #[getter] fn epsilon_(&self) -> f64 { self.inner.epsilon }
    #[getter] fn loss_(&self) -> &str {
        match self.inner.loss {
            crate::ml::linear::sgd::SGDRegLoss::SquaredError => "squared_error",
            crate::ml::linear::sgd::SGDRegLoss::Huber => "huber",
            crate::ml::linear::sgd::SGDRegLoss::EpsilonInsensitive => "epsilon_insensitive",
        }
    }
    fn __repr__(&self) -> String { "SGDRegressor()".to_string() }
}

#[pyclass(module = "seraplot", name = "DecisionTreeClassifier")]
pub struct PyDecisionTreeClassifier { inner: crate::ml::tree::decision_tree::DecisionTreeClassifier }

impl_cls_fit_predict_score!(PyDecisionTreeClassifier);
impl_get_set_params!(PyDecisionTreeClassifier,
    "max_depth" => inner.max_depth, usize;
    "min_samples_split" => inner.min_samples_split, usize;
    "min_samples_leaf" => inner.min_samples_leaf, usize;
);

#[pymethods]
impl PyDecisionTreeClassifier {
    #[new]
    #[pyo3(signature = (max_depth=10, min_samples_split=2, min_samples_leaf=1, max_features=None, criterion="gini"))]
    fn py_new(max_depth: usize, min_samples_split: usize, min_samples_leaf: usize, max_features: Option<usize>, criterion: &str) -> Self {
        let c = if criterion == "entropy" { crate::ml::tree::decision_tree::TreeCriterion::Entropy } else { crate::ml::tree::decision_tree::TreeCriterion::Gini };
        Self { inner: crate::ml::tree::decision_tree::DecisionTreeClassifier::new(max_depth, min_samples_split, min_samples_leaf, max_features, c) }
    }
    fn predict_proba(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        vv_to_np2d(py, self.inner.predict_proba(&xf, n, p))
    }
    #[getter] fn feature_importances_(&self) -> Vec<f64> { self.inner.feature_importances.clone() }
    #[getter] fn classes_(&self) -> Vec<i32> { self.inner.classes.clone() }
    #[getter] fn max_depth_(&self) -> usize { self.inner.max_depth }
    #[getter] fn min_samples_split_(&self) -> usize { self.inner.min_samples_split }
    #[getter] fn min_samples_leaf_(&self) -> usize { self.inner.min_samples_leaf }
    #[getter] fn max_features_(&self) -> Option<usize> { self.inner.max_features }
    #[getter] fn criterion_(&self) -> &str {
        match self.inner.criterion {
            crate::ml::tree::decision_tree::TreeCriterion::Entropy => "entropy",
            _ => "gini",
        }
    }
    fn __repr__(&self) -> String { format!("DecisionTreeClassifier(max_depth={})", self.inner.max_depth) }
}

#[pyclass(module = "seraplot", name = "DecisionTreeRegressor")]
pub struct PyDecisionTreeRegressor { inner: crate::ml::tree::decision_tree::DecisionTreeRegressor }

impl_reg_fit_predict_score!(PyDecisionTreeRegressor);
impl_get_set_params!(PyDecisionTreeRegressor,
    "max_depth" => inner.max_depth, usize;
    "min_samples_split" => inner.min_samples_split, usize;
    "min_samples_leaf" => inner.min_samples_leaf, usize;
);

#[pymethods]
impl PyDecisionTreeRegressor {
    #[new]
    #[pyo3(signature = (max_depth=10, min_samples_split=2, min_samples_leaf=1, max_features=None))]
    fn py_new(max_depth: usize, min_samples_split: usize, min_samples_leaf: usize, max_features: Option<usize>) -> Self {
        Self { inner: crate::ml::tree::decision_tree::DecisionTreeRegressor::new(max_depth, min_samples_split, min_samples_leaf, max_features) }
    }
    #[getter] fn feature_importances_(&self) -> Vec<f64> { self.inner.feature_importances.clone() }
    #[getter] fn max_depth_(&self) -> usize { self.inner.max_depth }
    #[getter] fn min_samples_split_(&self) -> usize { self.inner.min_samples_split }
    #[getter] fn min_samples_leaf_(&self) -> usize { self.inner.min_samples_leaf }
    #[getter] fn max_features_(&self) -> Option<usize> { self.inner.max_features }
    fn __repr__(&self) -> String { format!("DecisionTreeRegressor(max_depth={})", self.inner.max_depth) }
}

#[pyclass(module = "seraplot", name = "RandomForestClassifier")]
pub struct PyRandomForestClassifier { inner: crate::ml::tree::random_forest::RandomForestClassifier }

impl_cls_fit_predict_score!(PyRandomForestClassifier);
impl_get_set_params!(PyRandomForestClassifier,
    "n_estimators" => inner.n_estimators, usize;
    "max_depth" => inner.max_depth, usize;
    "min_samples_split" => inner.min_samples_split, usize;
    "min_samples_leaf" => inner.min_samples_leaf, usize;
);

#[pymethods]
impl PyRandomForestClassifier {
    #[new]
    #[pyo3(signature = (n_estimators=100, max_depth=10, min_samples_split=2, min_samples_leaf=1, max_features="sqrt"))]
    fn py_new(n_estimators: usize, max_depth: usize, min_samples_split: usize, min_samples_leaf: usize, max_features: &str) -> Self {
        let mf = match max_features {
            "log2" => crate::ml::tree::random_forest::MaxFeatures::Log2,
            "all" | "none" => crate::ml::tree::random_forest::MaxFeatures::All,
            _ => crate::ml::tree::random_forest::MaxFeatures::Sqrt,
        };
        Self { inner: crate::ml::tree::random_forest::RandomForestClassifier::new(n_estimators, max_depth, min_samples_split, min_samples_leaf, mf) }
    }
    fn predict_proba(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        vv_to_np2d(py, self.inner.predict_proba(&xf, n, p))
    }
    #[getter] fn feature_importances_(&self) -> Vec<f64> { self.inner.feature_importances.clone() }
    #[getter] fn classes_(&self) -> Vec<i32> { self.inner.classes.clone() }
    #[getter] fn n_estimators_(&self) -> usize { self.inner.n_estimators }
    #[getter] fn max_depth_(&self) -> usize { self.inner.max_depth }
    #[getter] fn min_samples_split_(&self) -> usize { self.inner.min_samples_split }
    #[getter] fn min_samples_leaf_(&self) -> usize { self.inner.min_samples_leaf }
    #[getter] fn max_features_(&self) -> String {
        match self.inner.max_features {
            crate::ml::tree::random_forest::MaxFeatures::Sqrt => "sqrt".to_string(),
            crate::ml::tree::random_forest::MaxFeatures::Log2 => "log2".to_string(),
            crate::ml::tree::random_forest::MaxFeatures::All => "all".to_string(),
            crate::ml::tree::random_forest::MaxFeatures::Fixed(n) => n.to_string(),
        }
    }
    fn __repr__(&self) -> String { format!("RandomForestClassifier(n_estimators={})", self.inner.n_estimators) }
}

#[pyclass(module = "seraplot", name = "RandomForestRegressor")]
pub struct PyRandomForestRegressor { inner: crate::ml::tree::random_forest::RandomForestRegressor }

impl_reg_fit_predict_score!(PyRandomForestRegressor);
impl_get_set_params!(PyRandomForestRegressor,
    "n_estimators" => inner.n_estimators, usize;
    "max_depth" => inner.max_depth, usize;
    "min_samples_split" => inner.min_samples_split, usize;
    "min_samples_leaf" => inner.min_samples_leaf, usize;
);

#[pymethods]
impl PyRandomForestRegressor {
    #[new]
    #[pyo3(signature = (n_estimators=100, max_depth=10, min_samples_split=2, min_samples_leaf=1, max_features="sqrt"))]
    fn py_new(n_estimators: usize, max_depth: usize, min_samples_split: usize, min_samples_leaf: usize, max_features: &str) -> Self {
        let mf = match max_features {
            "log2" => crate::ml::tree::random_forest::MaxFeatures::Log2,
            "all" | "none" => crate::ml::tree::random_forest::MaxFeatures::All,
            _ => crate::ml::tree::random_forest::MaxFeatures::Sqrt,
        };
        Self { inner: crate::ml::tree::random_forest::RandomForestRegressor::new(n_estimators, max_depth, min_samples_split, min_samples_leaf, mf) }
    }
    #[getter] fn feature_importances_(&self) -> Vec<f64> { self.inner.feature_importances.clone() }
    #[getter] fn n_estimators_(&self) -> usize { self.inner.n_estimators }
    #[getter] fn max_depth_(&self) -> usize { self.inner.max_depth }
    #[getter] fn min_samples_split_(&self) -> usize { self.inner.min_samples_split }
    #[getter] fn min_samples_leaf_(&self) -> usize { self.inner.min_samples_leaf }
    #[getter] fn max_features_(&self) -> String {
        match self.inner.max_features {
            crate::ml::tree::random_forest::MaxFeatures::Sqrt => "sqrt".to_string(),
            crate::ml::tree::random_forest::MaxFeatures::Log2 => "log2".to_string(),
            crate::ml::tree::random_forest::MaxFeatures::All => "all".to_string(),
            crate::ml::tree::random_forest::MaxFeatures::Fixed(n) => n.to_string(),
        }
    }
    fn __repr__(&self) -> String { format!("RandomForestRegressor(n_estimators={})", self.inner.n_estimators) }
}

#[pyclass(module = "seraplot", name = "GradientBoostingClassifier")]
pub struct PyGradientBoostingClassifier { inner: crate::ml::tree::gradient_boosting::GradientBoostingClassifier }

impl_cls_fit_predict_score!(PyGradientBoostingClassifier);
impl_get_set_params!(PyGradientBoostingClassifier,
    "n_estimators" => inner.n_estimators, usize;
    "learning_rate" => inner.learning_rate, f64;
    "max_depth" => inner.max_depth, usize;
);

#[pymethods]
impl PyGradientBoostingClassifier {
    #[new]
    #[pyo3(signature = (n_estimators=100, learning_rate=0.1, max_depth=3, min_samples_split=2, min_samples_leaf=1))]
    fn py_new(n_estimators: usize, learning_rate: f64, max_depth: usize, min_samples_split: usize, min_samples_leaf: usize) -> Self {
        Self { inner: crate::ml::tree::gradient_boosting::GradientBoostingClassifier::new(n_estimators, learning_rate, max_depth, min_samples_split, min_samples_leaf) }
    }
    fn predict_proba(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let flat = self.inner.predict_proba(&xf, n, p);
        let k = self.inner.classes.len();
        flat_to_np2d(py, flat, n, k)
    }
    #[getter] fn classes_(&self) -> Vec<i32> { self.inner.classes.clone() }
    #[getter] fn n_estimators_(&self) -> usize { self.inner.n_estimators }
    #[getter] fn learning_rate_(&self) -> f64 { self.inner.learning_rate }
    #[getter] fn max_depth_(&self) -> usize { self.inner.max_depth }
    #[getter] fn min_samples_split_(&self) -> usize { self.inner.min_samples_split }
    #[getter] fn min_samples_leaf_(&self) -> usize { self.inner.min_samples_leaf }
    fn __repr__(&self) -> String { format!("GradientBoostingClassifier(n_estimators={}, lr={})", self.inner.n_estimators, self.inner.learning_rate) }
}

#[pyclass(module = "seraplot", name = "GradientBoostingRegressor")]
pub struct PyGradientBoostingRegressor { inner: crate::ml::tree::gradient_boosting::GradientBoostingRegressor }

impl_reg_fit_predict_score!(PyGradientBoostingRegressor);
impl_get_set_params!(PyGradientBoostingRegressor,
    "n_estimators" => inner.n_estimators, usize;
    "learning_rate" => inner.learning_rate, f64;
    "max_depth" => inner.max_depth, usize;
);

#[pymethods]
impl PyGradientBoostingRegressor {
    #[new]
    #[pyo3(signature = (n_estimators=100, learning_rate=0.1, max_depth=3, min_samples_split=2, min_samples_leaf=1))]
    fn py_new(n_estimators: usize, learning_rate: f64, max_depth: usize, min_samples_split: usize, min_samples_leaf: usize) -> Self {
        Self { inner: crate::ml::tree::gradient_boosting::GradientBoostingRegressor::new(n_estimators, learning_rate, max_depth, min_samples_split, min_samples_leaf) }
    }
    #[getter] fn n_estimators_(&self) -> usize { self.inner.n_estimators }
    #[getter] fn learning_rate_(&self) -> f64 { self.inner.learning_rate }
    #[getter] fn max_depth_(&self) -> usize { self.inner.max_depth }
    #[getter] fn min_samples_split_(&self) -> usize { self.inner.min_samples_split }
    #[getter] fn min_samples_leaf_(&self) -> usize { self.inner.min_samples_leaf }
    fn __repr__(&self) -> String { format!("GradientBoostingRegressor(n_estimators={}, lr={})", self.inner.n_estimators, self.inner.learning_rate) }
}

#[pyclass(module = "seraplot", name = "AdaBoostClassifier")]
pub struct PyAdaBoostClassifier { inner: crate::ml::tree::adaboost::AdaBoostClassifier }

impl_cls_fit_predict_score!(PyAdaBoostClassifier);
impl_get_set_params!(PyAdaBoostClassifier,
    "n_estimators" => inner.n_estimators, usize;
    "learning_rate" => inner.learning_rate, f64;
    "max_depth" => inner.max_depth, usize;
);

#[pymethods]
impl PyAdaBoostClassifier {
    #[new]
    #[pyo3(signature = (n_estimators=50, learning_rate=1.0, max_depth=1))]
    fn py_new(n_estimators: usize, learning_rate: f64, max_depth: usize) -> Self {
        Self { inner: crate::ml::tree::adaboost::AdaBoostClassifier::new(n_estimators, learning_rate, max_depth) }
    }
    fn predict_proba(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let flat = self.inner.predict_proba(&xf, n, p);
        let k = self.inner.classes.len();
        flat_to_np2d(py, flat, n, k)
    }
    #[getter] fn classes_(&self) -> Vec<i32> { self.inner.classes.clone() }
    #[getter] fn n_estimators_(&self) -> usize { self.inner.n_estimators }
    #[getter] fn learning_rate_(&self) -> f64 { self.inner.learning_rate }
    #[getter] fn max_depth_(&self) -> usize { self.inner.max_depth }
    fn __repr__(&self) -> String { format!("AdaBoostClassifier(n_estimators={})", self.inner.n_estimators) }
}

#[pyclass(module = "seraplot", name = "AdaBoostRegressor")]
pub struct PyAdaBoostRegressor { inner: crate::ml::tree::adaboost::AdaBoostRegressor }

impl_reg_fit_predict_score!(PyAdaBoostRegressor);
impl_get_set_params!(PyAdaBoostRegressor,
    "n_estimators" => inner.n_estimators, usize;
    "learning_rate" => inner.learning_rate, f64;
    "max_depth" => inner.max_depth, usize;
);

#[pymethods]
impl PyAdaBoostRegressor {
    #[new]
    #[pyo3(signature = (n_estimators=50, learning_rate=1.0, max_depth=3))]
    fn py_new(n_estimators: usize, learning_rate: f64, max_depth: usize) -> Self {
        Self { inner: crate::ml::tree::adaboost::AdaBoostRegressor::new(n_estimators, learning_rate, max_depth) }
    }
    #[getter] fn n_estimators_(&self) -> usize { self.inner.n_estimators }
    #[getter] fn learning_rate_(&self) -> f64 { self.inner.learning_rate }
    #[getter] fn max_depth_(&self) -> usize { self.inner.max_depth }
    fn __repr__(&self) -> String { format!("AdaBoostRegressor(n_estimators={})", self.inner.n_estimators) }
}

#[pyclass(module = "seraplot", name = "KNeighborsClassifier")]
pub struct PyKNeighborsClassifier { inner: crate::ml::neighbors::knn::KNeighborsClassifier }

impl_cls_fit_predict_score!(PyKNeighborsClassifier);

#[pymethods]
impl PyKNeighborsClassifier {
    fn get_params(&self, _py: Python<'_>) -> PyResult<std::collections::HashMap<String, PyObject>> {
        let mut m = std::collections::HashMap::new();
        Python::with_gil(|py| {
            m.insert("n_neighbors".to_string(), self.inner.k.into_py(py));
            m.insert("weights".to_string(), self.weights_().into_py(py));
        });
        Ok(m)
    }
    fn set_params(&mut self, params: std::collections::HashMap<String, PyObject>) -> PyResult<()> {
        Python::with_gil(|py| {
            if let Some(v) = params.get("n_neighbors") { self.inner.k = v.extract::<usize>(py).unwrap_or(self.inner.k); }
            if let Some(v) = params.get("weights") {
                if let Ok(s) = v.extract::<String>(py) {
                    self.inner.weights = match s.as_str() { "distance" => crate::ml::neighbors::knn::KnnWeights::Distance, _ => crate::ml::neighbors::knn::KnnWeights::Uniform };
                }
            }
        });
        Ok(())
    }
}

#[pymethods]
impl PyKNeighborsClassifier {
    #[new]
    #[pyo3(signature = (n_neighbors=5, weights="uniform"))]
    fn py_new(n_neighbors: usize, weights: &str) -> Self {
        let w = match weights { "distance" => crate::ml::neighbors::knn::KnnWeights::Distance, _ => crate::ml::neighbors::knn::KnnWeights::Uniform };
        Self { inner: crate::ml::neighbors::knn::KNeighborsClassifier::new(n_neighbors, w) }
    }
    fn predict_proba(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let flat = self.inner.predict_proba(&xf, n, p);
        let k = self.inner.classes.len();
        flat_to_np2d(py, flat, n, k)
    }
    #[getter] fn classes_(&self) -> Vec<i32> { self.inner.classes.clone() }
    #[getter] fn n_neighbors_(&self) -> usize { self.inner.k }
    #[getter] fn weights_(&self) -> &str {
        match self.inner.weights { crate::ml::neighbors::knn::KnnWeights::Uniform => "uniform", crate::ml::neighbors::knn::KnnWeights::Distance => "distance" }
    }
    fn __repr__(&self) -> String { format!("KNeighborsClassifier(n_neighbors={})", self.inner.k) }
}

#[pyclass(module = "seraplot", name = "KNeighborsRegressor")]
pub struct PyKNeighborsRegressor { inner: crate::ml::neighbors::knn::KNeighborsRegressor }

impl_reg_fit_predict_score!(PyKNeighborsRegressor);

#[pymethods]
impl PyKNeighborsRegressor {
    fn get_params(&self, _py: Python<'_>) -> PyResult<std::collections::HashMap<String, PyObject>> {
        let mut m = std::collections::HashMap::new();
        Python::with_gil(|py| {
            m.insert("n_neighbors".to_string(), self.inner.k.into_py(py));
            m.insert("weights".to_string(), self.weights_().into_py(py));
        });
        Ok(m)
    }
    fn set_params(&mut self, params: std::collections::HashMap<String, PyObject>) -> PyResult<()> {
        Python::with_gil(|py| {
            if let Some(v) = params.get("n_neighbors") { self.inner.k = v.extract::<usize>(py).unwrap_or(self.inner.k); }
            if let Some(v) = params.get("weights") {
                if let Ok(s) = v.extract::<String>(py) {
                    self.inner.weights = match s.as_str() { "distance" => crate::ml::neighbors::knn::KnnWeights::Distance, _ => crate::ml::neighbors::knn::KnnWeights::Uniform };
                }
            }
        });
        Ok(())
    }
}

#[pymethods]
impl PyKNeighborsRegressor {
    #[new]
    #[pyo3(signature = (n_neighbors=5, weights="uniform"))]
    fn py_new(n_neighbors: usize, weights: &str) -> Self {
        let w = match weights { "distance" => crate::ml::neighbors::knn::KnnWeights::Distance, _ => crate::ml::neighbors::knn::KnnWeights::Uniform };
        Self { inner: crate::ml::neighbors::knn::KNeighborsRegressor::new(n_neighbors, w) }
    }
    #[getter] fn n_neighbors_(&self) -> usize { self.inner.k }
    #[getter] fn weights_(&self) -> &str {
        match self.inner.weights { crate::ml::neighbors::knn::KnnWeights::Uniform => "uniform", crate::ml::neighbors::knn::KnnWeights::Distance => "distance" }
    }
    fn __repr__(&self) -> String { format!("KNeighborsRegressor(n_neighbors={})", self.inner.k) }
}

#[pyclass(module = "seraplot", name = "NearestCentroid")]
pub struct PyNearestCentroid { inner: crate::ml::neighbors::knn::NearestCentroid }

impl_cls_fit_predict_score!(PyNearestCentroid);

#[pymethods]
impl PyNearestCentroid {
    #[new]
    fn py_new() -> Self { Self { inner: crate::ml::neighbors::knn::NearestCentroid::new() } }
    fn get_params(&self, _py: Python<'_>) -> PyResult<std::collections::HashMap<String, PyObject>> { Ok(std::collections::HashMap::new()) }
    fn set_params(&mut self, _params: std::collections::HashMap<String, PyObject>) -> PyResult<()> { Ok(()) }
    #[getter] fn classes_(&self) -> Vec<i32> { self.inner.classes.clone() }
    fn __repr__(&self) -> String { "NearestCentroid()".to_string() }
}

#[pyclass(module = "seraplot", name = "GaussianNB")]
pub struct PyGaussianNB { inner: crate::ml::naive_bayes::gaussian::GaussianNB }

impl_cls_fit_predict_score!(PyGaussianNB);
impl_get_set_params!(PyGaussianNB, "var_smoothing" => inner.var_smoothing, f64;);

#[pymethods]
impl PyGaussianNB {
    #[new]
    #[pyo3(signature = (var_smoothing=1e-9))]
    fn py_new(var_smoothing: f64) -> Self { Self { inner: crate::ml::naive_bayes::gaussian::GaussianNB::with_var_smoothing(var_smoothing) } }
    fn predict_proba(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let flat = self.inner.predict_proba(&xf, n, p);
        let k = self.inner.classes.len();
        flat_to_np2d(py, flat, n, k)
    }
    #[getter] fn classes_(&self) -> Vec<i32> { self.inner.classes.clone() }
    #[getter] fn var_smoothing_(&self) -> f64 { self.inner.var_smoothing }
    #[getter] fn theta_(&self) -> Vec<Vec<f64>> {
        let k = self.inner.classes.len();
        let p = self.inner.n_features();
        (0..k).map(|c| self.inner.theta()[c * p..(c + 1) * p].to_vec()).collect()
    }
    #[getter] fn var_(&self) -> Vec<Vec<f64>> {
        let k = self.inner.classes.len();
        let p = self.inner.n_features();
        (0..k).map(|c| self.inner.var()[c * p..(c + 1) * p].to_vec()).collect()
    }
    #[getter] fn class_prior_(&self) -> Vec<f64> { self.inner.class_prior().to_vec() }
    fn __repr__(&self) -> String { "GaussianNB()".to_string() }
}

#[pyclass(module = "seraplot", name = "MultinomialNB")]
pub struct PyMultinomialNB { inner: crate::ml::naive_bayes::multinomial::MultinomialNB }

impl_cls_fit_predict_score!(PyMultinomialNB);
impl_get_set_params!(PyMultinomialNB, "alpha" => inner.alpha, f64;);

#[pymethods]
impl PyMultinomialNB {
    #[new]
    #[pyo3(signature = (alpha=1.0))]
    fn py_new(alpha: f64) -> Self { Self { inner: crate::ml::naive_bayes::multinomial::MultinomialNB::new(alpha) } }
    fn predict_proba(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let flat = self.inner.predict_proba(&xf, n, p);
        let k = self.inner.classes.len();
        flat_to_np2d(py, flat, n, k)
    }
    #[getter] fn classes_(&self) -> Vec<i32> { self.inner.classes.clone() }
    #[getter] fn alpha_(&self) -> f64 { self.inner.alpha }
    fn __repr__(&self) -> String { format!("MultinomialNB(alpha={})", self.inner.alpha) }
}

#[pyclass(module = "seraplot", name = "BernoulliNB")]
pub struct PyBernoulliNB { inner: crate::ml::naive_bayes::bernoulli::BernoulliNB }

impl_cls_fit_predict_score!(PyBernoulliNB);
impl_get_set_params!(PyBernoulliNB, "alpha" => inner.alpha, f64; "binarize" => inner.binarize, f64;);

#[pymethods]
impl PyBernoulliNB {
    #[new]
    #[pyo3(signature = (alpha=1.0, binarize=0.0))]
    fn py_new(alpha: f64, binarize: f64) -> Self { Self { inner: crate::ml::naive_bayes::bernoulli::BernoulliNB::new(alpha, binarize) } }
    fn predict_proba(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let flat = self.inner.predict_proba(&xf, n, p);
        let k = self.inner.classes.len();
        flat_to_np2d(py, flat, n, k)
    }
    #[getter] fn classes_(&self) -> Vec<i32> { self.inner.classes.clone() }
    #[getter] fn alpha_(&self) -> f64 { self.inner.alpha }
    #[getter] fn binarize_(&self) -> f64 { self.inner.binarize }
    fn __repr__(&self) -> String { format!("BernoulliNB(alpha={})", self.inner.alpha) }
}

#[pyclass(module = "seraplot", name = "LinearSVC")]
pub struct PyLinearSVC { inner: crate::ml::svm::svm::LinearSVC }

impl_cls_fit_predict_score!(PyLinearSVC);
impl_get_set_params!(PyLinearSVC,
    "C" => inner.c, f64;
    "max_iter" => inner.max_iter, usize;
    "tol" => inner.tol, f64;
    "fit_intercept" => inner.fit_intercept, bool;
);

#[pymethods]
impl PyLinearSVC {
    #[allow(non_snake_case)]
    #[new]
    #[pyo3(signature = (C=1.0, max_iter=1000, tol=1e-4, fit_intercept=true))]
    fn py_new(C: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> Self {
        Self { inner: crate::ml::svm::svm::LinearSVC::with_fit_intercept(C, max_iter, tol, fit_intercept) }
    }
    fn decision_function(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        Ok(vec_f64_to_np(py, self.inner.decision_function(&xf, n, p)))
    }
    #[getter] fn coef_(&self) -> Vec<Vec<f64>> { self.inner.coef().to_vec() }
    #[getter] fn intercept_(&self) -> Vec<f64> { self.inner.intercept().to_vec() }
    #[getter] fn classes_(&self) -> Vec<i32> { self.inner.classes.clone() }
    #[getter] fn C_(&self) -> f64 { self.inner.c }
    #[getter] fn max_iter_(&self) -> usize { self.inner.max_iter }
    #[getter] fn tol_(&self) -> f64 { self.inner.tol }
    #[getter] fn fit_intercept_(&self) -> bool { self.inner.fit_intercept }
    fn __repr__(&self) -> String { format!("LinearSVC(C={})", self.inner.c) }
}

#[pyclass(module = "seraplot", name = "LinearSVR")]
pub struct PyLinearSVR { inner: crate::ml::svm::svm::LinearSVR }

impl_reg_fit_predict_score!(PyLinearSVR);
impl_get_set_params!(PyLinearSVR,
    "C" => inner.c, f64;
    "epsilon" => inner.epsilon, f64;
    "max_iter" => inner.max_iter, usize;
    "tol" => inner.tol, f64;
    "fit_intercept" => inner.fit_intercept, bool;
);

#[pymethods]
impl PyLinearSVR {
    #[allow(non_snake_case)]
    #[new]
    #[pyo3(signature = (C=1.0, epsilon=0.1, max_iter=1000, tol=1e-4, fit_intercept=true))]
    fn py_new(C: f64, epsilon: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> Self {
        Self { inner: crate::ml::svm::svm::LinearSVR::with_fit_intercept(C, epsilon, max_iter, tol, fit_intercept) }
    }
    #[getter] fn coef_(&self) -> Vec<f64> { self.inner.coef().to_vec() }
    #[getter] fn intercept_(&self) -> f64 { self.inner.intercept() }
    #[getter] fn C_(&self) -> f64 { self.inner.c }
    #[getter] fn epsilon_(&self) -> f64 { self.inner.epsilon }
    #[getter] fn max_iter_(&self) -> usize { self.inner.max_iter }
    #[getter] fn tol_(&self) -> f64 { self.inner.tol }
    #[getter] fn fit_intercept_(&self) -> bool { self.inner.fit_intercept }
    fn __repr__(&self) -> String { format!("LinearSVR(C={})", self.inner.c) }
}

#[pyclass(module = "seraplot", name = "RidgeClassifier")]
pub struct PyRidgeClassifier { inner: crate::ml::linear::ridge::RidgeClassifier }

impl_cls_fit_predict_score!(PyRidgeClassifier);
impl_get_set_params!(PyRidgeClassifier, "alpha" => inner.ridge.alpha, f64;);

#[pymethods]
impl PyRidgeClassifier {
    #[new]
    #[pyo3(signature = (alpha=1.0))]
    fn py_new(alpha: f64) -> Self { Self { inner: crate::ml::linear::ridge::RidgeClassifier::new(alpha) } }
    #[getter] fn coef_(&self, py: Python<'_>) -> PyResult<PyObject> {
        if self.inner.coefs.len() <= 1 {
            let v = self.inner.coefs.first().cloned().unwrap_or_default();
            Ok(v.into_pyarray_bound(py).into_py(py))
        } else {
            vv_to_np2d(py, self.inner.coefs.clone())
        }
    }
    #[getter] fn intercept_(&self, py: Python<'_>) -> PyResult<PyObject> {
        if self.inner.intercepts.len() <= 1 {
            Ok(self.inner.intercepts.first().copied().unwrap_or(0.0).into_py(py))
        } else {
            Ok(self.inner.intercepts.clone().into_pyarray_bound(py).into_py(py))
        }
    }
    #[getter] fn classes_(&self) -> Vec<i32> { self.inner.classes.clone() }
    fn __repr__(&self) -> String { format!("RidgeClassifier(alpha={})", self.inner.ridge.alpha) }
}

#[pyclass(module = "seraplot", name = "StandardScaler")]
pub struct PyStandardScaler { inner: crate::ml::preprocessing::scalers::StandardScaler, n: usize, p: usize }

#[pymethods]
impl PyStandardScaler {
    #[new]
    #[pyo3(signature = (with_mean=true, with_std=true))]
    fn py_new(with_mean: bool, with_std: bool) -> Self {
        Self { inner: crate::ml::preprocessing::scalers::StandardScaler::new(with_mean, with_std), n: 0, p: 0 }
    }
    fn fit(&mut self, x: &PyAny) -> PyResult<()> {
        with_flat(x, |xf, n, p| {
            let fp = crate::ml::cache::Fp::new("StandardScaler.fit").data(xf, n, p).finish();
            let _h = crate::ml::cache::TaskHandle::auto("StandardScaler.fit", fp);
            self.n = n; self.p = p;
            self.inner.fit(xf, n, p);
            _h.complete(&crate::ml::cache::PartialState::default());
        })?;
        Ok(())
    }
    fn partial_fit(&mut self, x: &PyAny) -> PyResult<()> {
        with_flat(x, |xf, n, p| {
            self.inner.partial_fit(xf, n, p);
            self.n = self.inner.n_samples_seen as usize; self.p = p;
        })?;
        Ok(())
    }
    fn transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (out, n, p) = with_flat(x, |xf, n, p| (self.inner.transform(xf, n, p), n, p))?;
        flat_to_np2d(py, out, n, p)
    }
    fn fit_transform(&mut self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (out, n, p) = with_flat(x, |xf, n, p| {
            self.n = n; self.p = p;
            (self.inner.fit_transform(xf, n, p), n, p)
        })?;
        flat_to_np2d(py, out, n, p)
    }
    fn inverse_transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (out, n, p) = with_flat(x, |xf, n, p| (self.inner.inverse_transform(xf, n, p), n, p))?;
        flat_to_np2d(py, out, n, p)
    }
    #[getter] fn mean_(&self) -> Vec<f64> { self.inner.mean.clone() }
    #[getter] fn scale_(&self) -> Vec<f64> { self.inner.scale.clone() }
    #[getter] fn with_mean_(&self) -> bool { self.inner.with_mean }
    #[getter] fn with_std_(&self) -> bool { self.inner.with_std }
    #[getter] fn var_(&self) -> Vec<f64> { self.inner.scale.iter().map(|s| s * s).collect() }
    #[getter] fn n_samples_seen_(&self) -> u64 { self.inner.n_samples_seen }
    fn __repr__(&self) -> String { "StandardScaler()".to_string() }
}

#[pyclass(module = "seraplot", name = "MinMaxScaler")]
pub struct PyMinMaxScaler { inner: crate::ml::preprocessing::scalers::MinMaxScaler }

#[pymethods]
impl PyMinMaxScaler {
    #[new]
    #[pyo3(signature = (feature_range_min=0.0, feature_range_max=1.0))]
    fn py_new(feature_range_min: f64, feature_range_max: f64) -> Self {
        Self { inner: crate::ml::preprocessing::scalers::MinMaxScaler::new((feature_range_min, feature_range_max)) }
    }
    fn fit(&mut self, x: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let fp = crate::ml::cache::Fp::new("MinMaxScaler.fit").data(&xf, n, p).finish();
        let _h = crate::ml::cache::TaskHandle::auto("MinMaxScaler.fit", fp);
        self.inner.fit(&xf, n, p);
        _h.complete(&crate::ml::cache::PartialState::default());
        Ok(())
    }
    fn partial_fit(&mut self, x: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        self.inner.partial_fit(&xf, n, p);
        Ok(())
    }
    fn transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        flat_to_np2d(py, self.inner.transform(&xf, n, p), n, p)
    }
    fn fit_transform(&mut self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        flat_to_np2d(py, self.inner.fit_transform(&xf, n, p), n, p)
    }
    fn inverse_transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        flat_to_np2d(py, self.inner.inverse_transform(&xf, n, p), n, p)
    }
    #[getter] fn data_min_(&self) -> Vec<f64> { self.inner.min.clone() }
    #[getter] fn data_range_(&self) -> Vec<f64> { self.inner.range.clone() }
    #[getter] fn feature_range_(&self) -> (f64, f64) { self.inner.feature_range }
    #[getter] fn scale_(&self) -> Vec<f64> {
        let (a, b) = self.inner.feature_range;
        let span = b - a;
        self.inner.range.iter().map(|r| if *r != 0.0 { span / r } else { 0.0 }).collect()
    }
    fn __repr__(&self) -> String { "MinMaxScaler()".to_string() }
}

#[pyclass(module = "seraplot", name = "RobustScaler")]
pub struct PyRobustScaler { inner: crate::ml::preprocessing::scalers::RobustScaler }

#[pymethods]
impl PyRobustScaler {
    #[new]
    #[pyo3(signature = (with_centering=true, with_scaling=true, quantile_range=(25.0, 75.0)))]
    fn py_new(with_centering: bool, with_scaling: bool, quantile_range: (f64, f64)) -> Self {
        Self { inner: crate::ml::preprocessing::scalers::RobustScaler::with_quantile_range(with_centering, with_scaling, quantile_range) }
    }
    fn fit(&mut self, x: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let fp = crate::ml::cache::Fp::new("RobustScaler.fit").data(&xf, n, p).finish();
        let _h = crate::ml::cache::TaskHandle::auto("RobustScaler.fit", fp);
        self.inner.fit(&xf, n, p);
        _h.complete(&crate::ml::cache::PartialState::default());
        Ok(())
    }
    fn transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        flat_to_np2d(py, self.inner.transform(&xf, n, p), n, p)
    }
    fn fit_transform(&mut self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        flat_to_np2d(py, self.inner.fit_transform(&xf, n, p), n, p)
    }
    #[getter] fn center_(&self) -> Vec<f64> { self.inner.center.clone() }
    #[getter] fn scale_(&self) -> Vec<f64> { self.inner.scale.clone() }
    #[getter] fn with_centering_(&self) -> bool { self.inner.with_centering }
    #[getter] fn with_scaling_(&self) -> bool { self.inner.with_scaling }
    #[getter] fn quantile_range_(&self) -> (f64, f64) { self.inner.quantile_range }
    fn __repr__(&self) -> String { "RobustScaler()".to_string() }
}

#[pyclass(module = "seraplot", name = "MaxAbsScaler")]
pub struct PyMaxAbsScaler { inner: crate::ml::preprocessing::scalers::MaxAbsScaler }

#[pymethods]
impl PyMaxAbsScaler {
    #[new]
    fn py_new() -> Self { Self { inner: crate::ml::preprocessing::scalers::MaxAbsScaler::new() } }
    fn fit(&mut self, x: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let fp = crate::ml::cache::Fp::new("MaxAbsScaler.fit").data(&xf, n, p).finish();
        let _h = crate::ml::cache::TaskHandle::auto("MaxAbsScaler.fit", fp);
        self.inner.fit(&xf, n, p);
        _h.complete(&crate::ml::cache::PartialState::default());
        Ok(())
    }
    fn partial_fit(&mut self, x: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        self.inner.partial_fit(&xf, n, p);
        Ok(())
    }
    fn transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        flat_to_np2d(py, self.inner.transform(&xf, n, p), n, p)
    }
    fn fit_transform(&mut self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        flat_to_np2d(py, self.inner.fit_transform(&xf, n, p), n, p)
    }
    #[getter] fn max_abs_(&self) -> Vec<f64> { self.inner.max_abs.clone() }
    fn __repr__(&self) -> String { "MaxAbsScaler()".to_string() }
}

#[pyclass(module = "seraplot", name = "Normalizer")]
pub struct PyNormalizer { inner: crate::ml::preprocessing::scalers::Normalizer }

#[pymethods]
impl PyNormalizer {
    #[new]
    #[pyo3(signature = (norm="l2"))]
    fn py_new(norm: &str) -> Self {
        let n = match norm {
            "l1" => crate::ml::preprocessing::scalers::NormType::L1,
            "max" => crate::ml::preprocessing::scalers::NormType::Max,
            _ => crate::ml::preprocessing::scalers::NormType::L2,
        };
        Self { inner: crate::ml::preprocessing::scalers::Normalizer::new(n) }
    }
    fn transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        flat_to_np2d(py, self.inner.transform(&xf, n, p), n, p)
    }
    fn fit_transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        flat_to_np2d(py, self.inner.transform(&xf, n, p), n, p)
    }
    fn __repr__(&self) -> String { "Normalizer()".to_string() }
}

#[pyclass(module = "seraplot", name = "PCA")]
pub struct PyPCA { inner: crate::ml::decomposition::pca::PCA }

#[pymethods]
impl PyPCA {
    #[new]
    #[pyo3(signature = (n_components=2, svd_solver="auto", whiten=false))]
    fn py_new(n_components: usize, svd_solver: &str, whiten: bool) -> Self {
        Self { inner: crate::ml::decomposition::pca::PCA::with_options(n_components, svd_solver.to_string(), whiten) }
    }
    fn fit(&mut self, x: &PyAny) -> PyResult<()> {
        with_flat(x, |xf, n, p| {
            let fp = crate::ml::cache::Fp::new("PCA.fit").usize(self.inner.n_components).data(xf, n, p).finish();
            let _h = crate::ml::cache::TaskHandle::auto("PCA.fit", fp);
            self.inner.fit(xf, n, p);
            _h.complete(&crate::ml::cache::PartialState::default());
        })?;
        Ok(())
    }
    fn transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (flat, n) = with_flat(x, |xf, nn, pp| (self.inner.transform(xf, nn, pp), nn))?;
        let k = self.inner.n_components.min(self.inner.singular_values.len());
        flat_to_np2d(py, flat, n, k)
    }
    fn fit_transform(&mut self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (flat, n) = with_flat(x, |xf, nn, pp| (self.inner.fit_transform(xf, nn, pp), nn))?;
        let k = self.inner.n_components.min(self.inner.singular_values.len());
        flat_to_np2d(py, flat, n, k)
    }
    fn inverse_transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (flat, n) = with_flat(x, |xf, nn, _pp| (self.inner.inverse_transform(xf, nn), nn))?;
        let p = self.inner.mean.len();
        flat_to_np2d(py, flat, n, p)
    }
    #[getter] fn components_(&self) -> Vec<Vec<f64>> {
        let p = self.inner.mean.len();
        let k = self.inner.n_components.min(self.inner.singular_values.len());
        (0..k).map(|c| self.inner.components[c * p..(c + 1) * p].to_vec()).collect()
    }
    #[getter] fn explained_variance_(&self) -> Vec<f64> { self.inner.explained_variance.clone() }
    #[getter] fn explained_variance_ratio_(&self) -> Vec<f64> { self.inner.explained_variance_ratio.clone() }
    #[getter] fn singular_values_(&self) -> Vec<f64> { self.inner.singular_values.clone() }
    #[getter] fn mean_(&self) -> Vec<f64> { self.inner.mean.clone() }
    #[getter] fn n_components_(&self) -> usize { self.inner.n_components }
    #[getter] fn whiten_(&self) -> bool { self.inner.whiten }
    #[getter] fn svd_solver_(&self) -> String { self.inner.svd_solver.clone() }
    fn __repr__(&self) -> String { format!("PCA(n_components={})", self.inner.n_components) }
}

#[pyclass(module = "seraplot", name = "TruncatedSVD")]
pub struct PyTruncatedSVD { inner: crate::ml::decomposition::pca::TruncatedSVD }

#[pymethods]
impl PyTruncatedSVD {
    #[new]
    #[pyo3(signature = (n_components=2))]
    fn py_new(n_components: usize) -> Self { Self { inner: crate::ml::decomposition::pca::TruncatedSVD::new(n_components) } }
    fn fit(&mut self, x: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let fp = crate::ml::cache::Fp::new("TruncatedSVD.fit").usize(self.inner.n_components).data(&xf, n, p).finish();
        let _h = crate::ml::cache::TaskHandle::auto("TruncatedSVD.fit", fp);
        self.inner.fit(&xf, n, p);
        _h.complete(&crate::ml::cache::PartialState::default());
        Ok(())
    }
    fn transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let flat = self.inner.transform(&xf, n, p);
        let k = self.inner.n_components.min(self.inner.singular_values.len());
        flat_to_np2d(py, flat, n, k)
    }
    fn fit_transform(&mut self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let flat = self.inner.fit_transform(&xf, n, p);
        let k = self.inner.n_components.min(self.inner.singular_values.len());
        flat_to_np2d(py, flat, n, k)
    }
    #[getter] fn components_(&self) -> Vec<Vec<f64>> {
        let comp_len = self.inner.components.len();
        let k = self.inner.n_components.min(self.inner.singular_values.len());
        if k == 0 { return vec![]; }
        let p = comp_len / k;
        (0..k).map(|c| self.inner.components[c * p..(c + 1) * p].to_vec()).collect()
    }
    #[getter] fn explained_variance_(&self) -> Vec<f64> { self.inner.explained_variance.clone() }
    #[getter] fn explained_variance_ratio_(&self) -> Vec<f64> { self.inner.explained_variance_ratio.clone() }
    #[getter] fn singular_values_(&self) -> Vec<f64> { self.inner.singular_values.clone() }
    #[getter] fn n_components_(&self) -> usize { self.inner.n_components }
    fn __repr__(&self) -> String { format!("TruncatedSVD(n_components={})", self.inner.n_components) }
}

#[pyclass(module = "seraplot", name = "LabelEncoder")]
pub struct PyLabelEncoder { inner: crate::ml::preprocessing::encoders::LabelEncoder }

#[pymethods]
impl PyLabelEncoder {
    #[new]
    fn py_new() -> Self { Self { inner: crate::ml::preprocessing::encoders::LabelEncoder::new() } }
    fn fit(&mut self, y: Vec<String>) { self.inner.fit(&y); }
    fn transform(&self, y: Vec<String>) -> Vec<i32> { self.inner.transform(&y) }
    fn fit_transform(&mut self, py: Python<'_>, y: &PyAny) -> PyResult<PyObject> {
        use pyo3::types::PyDict;
        let np = py.import("numpy")?;
        let kw = PyDict::new(py);
        kw.set_item("return_inverse", true)?;
        let result: &PyAny = np.call_method("unique", (y,), Some(kw))?;
        let unique: Vec<String> = result.get_item(0)?.call_method0("tolist")?.extract()?;
        self.inner.fit(&unique);
        let inv_any: &PyAny = result.get_item(1)?;
        Ok(inv_any.call_method1("astype", (np.getattr("int32")?,))?.into_py(py))
    }
    fn inverse_transform(&self, y: Vec<i32>) -> Vec<String> { self.inner.inverse_transform(&y) }
    #[getter] fn classes_(&self) -> Vec<String> { self.inner.classes.clone() }
    fn __repr__(&self) -> String { "LabelEncoder()".to_string() }
}

#[pyclass(module = "seraplot", name = "StratifiedKFold")]
pub struct PyStratifiedKFold { n_splits: usize, shuffle: bool, random_state: u64 }

#[pymethods]
impl PyStratifiedKFold {
    #[new]
    #[pyo3(signature = (n_splits=5, shuffle=false, random_state=0))]
    fn py_new(n_splits: usize, shuffle: bool, random_state: u64) -> Self { Self { n_splits, shuffle, random_state } }
    fn split(&self, _x: &PyAny, y: &PyAny) -> PyResult<Vec<(Vec<usize>, Vec<usize>)>> {
        let yl = extract_labels(y)?;
        let k = self.n_splits;
        let mut classes: Vec<i32> = yl.clone();
        classes.sort_unstable(); classes.dedup();
        let mut class_indices: Vec<Vec<usize>> = classes.iter().map(|&c| {
            yl.iter().enumerate().filter(|(_, &v)| v == c).map(|(i, _)| i).collect()
        }).collect();
        if self.shuffle {
            let mut rng = if self.random_state == 0 { 0xDEADBEEFCAFEu64 } else { self.random_state };
            for indices in &mut class_indices {
                for i in (1..indices.len()).rev() {
                    rng ^= rng << 13; rng ^= rng >> 7; rng ^= rng << 17;
                    let j = rng as usize % (i + 1);
                    indices.swap(i, j);
                }
            }
        }
        let mut folds: Vec<Vec<usize>> = (0..k).map(|_| Vec::new()).collect();
        for indices in &class_indices {
            for (i, &idx) in indices.iter().enumerate() { folds[i % k].push(idx); }
        }
        Ok((0..k).map(|fi| {
            let test: Vec<usize> = folds[fi].clone();
            let mut train: Vec<usize> = (0..k).filter(|&j| j != fi).flat_map(|j| folds[j].iter().copied()).collect();
            train.sort_unstable();
            (train, test)
        }).collect())
    }
    fn get_n_splits(&self, _x: Option<&PyAny>, _y: Option<&PyAny>, _groups: Option<&PyAny>) -> usize { self.n_splits }
    #[getter] fn n_splits_(&self) -> usize { self.n_splits }
    #[getter] fn shuffle_(&self) -> bool { self.shuffle }
    #[getter] fn random_state_(&self) -> u64 { self.random_state }
    fn __repr__(&self) -> String { format!("StratifiedKFold(n_splits={})", self.n_splits) }
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn accuracy_score(y_true: &PyAny, y_pred: &PyAny) -> PyResult<f64> {
    let yt = extract_labels(y_true)?; let yp = extract_labels(y_pred)?;
    Ok(crate::ml::metrics::classification::accuracy_score(&yt, &yp))
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn mean_squared_error(y_true: &PyAny, y_pred: &PyAny) -> PyResult<f64> {
    let yt = extract_targets(y_true)?; let yp = extract_targets(y_pred)?;
    Ok(crate::ml::metrics::regression::mean_squared_error(&yt, &yp))
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn mean_absolute_error(y_true: &PyAny, y_pred: &PyAny) -> PyResult<f64> {
    let yt = extract_targets(y_true)?; let yp = extract_targets(y_pred)?;
    Ok(crate::ml::metrics::regression::mean_absolute_error(&yt, &yp))
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn r2_score(y_true: &PyAny, y_pred: &PyAny) -> PyResult<f64> {
    let yt = extract_targets(y_true)?; let yp = extract_targets(y_pred)?;
    Ok(crate::ml::metrics::regression::r2_score(&yt, &yp))
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn root_mean_squared_error(y_true: &PyAny, y_pred: &PyAny) -> PyResult<f64> {
    let yt = extract_targets(y_true)?; let yp = extract_targets(y_pred)?;
    Ok(crate::ml::metrics::regression::root_mean_squared_error(&yt, &yp))
}

#[pyfunction]
#[pyo3(signature = (estimator, x, y, *, cv=5, scoring="auto", seed=42))]
pub fn cross_val_score(estimator: &PyAny, x: &PyAny, y: &PyAny, cv: usize, scoring: &str, seed: u64) -> PyResult<Vec<f64>> {
    use crate::ml::model_selection::grid_search::*;
    let est_name = extract_estimator_name(estimator)?;
    let estimator = est_name.as_str();
    let (xf, n, p) = extract_flat(x)?;
    let pn: Vec<String> = Vec::new();
    let pv: Vec<Vec<String>> = Vec::new();
    let ps: Vec<usize> = Vec::new();
    if is_classifier(estimator) {
        let yl = extract_labels(y)?;
        let folds = precompute_folds_cls(&xf, n, p, &yl, cv, seed);
        let caches = compute_caches(estimator, &folds, &pn, &pv);
        Ok(folds.iter().zip(caches.iter()).map(|(fold, cache)| {
            eval_model_scored(estimator, &pn, &pv, &ps, 0, fold, cache, scoring)
        }).collect())
    } else {
        let yt = extract_targets(y)?;
        let folds = precompute_folds_reg(&xf, n, p, &yt, cv, seed);
        let caches = compute_caches(estimator, &folds, &pn, &pv);
        Ok(folds.iter().zip(caches.iter()).map(|(fold, cache)| {
            eval_model_scored(estimator, &pn, &pv, &ps, 0, fold, cache, scoring)
        }).collect())
    }
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn classification_report(y_true: &PyAny, y_pred: &PyAny) -> PyResult<String> {
    let yt = extract_labels(y_true)?; let yp = extract_labels(y_pred)?;
    Ok(crate::ml::metrics::classification::classification_report(&yt, &yp))
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred, average="weighted"))]
pub fn f1_score(y_true: &PyAny, y_pred: &PyAny, average: &str) -> PyResult<f64> {
    let y_true = extract_labels(y_true)?; let y_pred = extract_labels(y_pred)?;
    let avg = match average {
        "macro" => crate::ml::metrics::classification::Average::Macro,
        "weighted" => crate::ml::metrics::classification::Average::Weighted,
        _ => {
            let classes = { let mut c: Vec<i32> = y_true.iter().chain(y_pred.iter()).copied().collect(); c.sort_unstable(); c.dedup(); c };
            crate::ml::metrics::classification::Average::Binary(*classes.last().unwrap_or(&1))
        }
    };
    Ok(crate::ml::metrics::classification::f1_score(&y_true, &y_pred, avg))
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred, average="weighted"))]
pub fn precision_score(y_true: &PyAny, y_pred: &PyAny, average: &str) -> PyResult<f64> {
    let y_true = extract_labels(y_true)?; let y_pred = extract_labels(y_pred)?;
    let avg = match average {
        "macro" => crate::ml::metrics::classification::Average::Macro,
        "weighted" => crate::ml::metrics::classification::Average::Weighted,
        _ => {
            let classes = { let mut c: Vec<i32> = y_true.iter().chain(y_pred.iter()).copied().collect(); c.sort_unstable(); c.dedup(); c };
            crate::ml::metrics::classification::Average::Binary(*classes.last().unwrap_or(&1))
        }
    };
    Ok(crate::ml::metrics::classification::precision_score(&y_true, &y_pred, avg))
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred, average="weighted"))]
pub fn recall_score(y_true: &PyAny, y_pred: &PyAny, average: &str) -> PyResult<f64> {
    let y_true = extract_labels(y_true)?; let y_pred = extract_labels(y_pred)?;
    let avg = match average {
        "macro" => crate::ml::metrics::classification::Average::Macro,
        "weighted" => crate::ml::metrics::classification::Average::Weighted,
        _ => {
            let classes = { let mut c: Vec<i32> = y_true.iter().chain(y_pred.iter()).copied().collect(); c.sort_unstable(); c.dedup(); c };
            crate::ml::metrics::classification::Average::Binary(*classes.last().unwrap_or(&1))
        }
    };
    Ok(crate::ml::metrics::classification::recall_score(&y_true, &y_pred, avg))
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn confusion_matrix(y_true: &PyAny, y_pred: &PyAny) -> PyResult<(Vec<i32>, Vec<usize>)> {
    let yt = extract_labels(y_true)?; let yp = extract_labels(y_pred)?;
    Ok(crate::ml::metrics::classification::confusion_matrix(&yt, &yp))
}

#[pyfunction]
#[pyo3(signature = (x, y, test_size=0.2, random_state=42))]
pub fn train_test_split(py: Python<'_>, x: &PyAny, y: &PyAny, test_size: f64, random_state: u64) -> PyResult<(PyObject, PyObject, Vec<f64>, Vec<f64>)> {
    let (xf, n, p) = extract_flat(x)?;
    let yt = extract_targets(y)?;
    let (x_train, x_test, y_train, y_test) = crate::ml::model_selection::split::train_test_split_reg(&xf, n, p, &yt, test_size, random_state);
    let n_train = y_train.len();
    let n_test = y_test.len();
    let xt = flat_to_np2d(py, x_train, n_train, p)?;
    let xte = flat_to_np2d(py, x_test, n_test, p)?;
    Ok((xt, xte, y_train, y_test))
}

#[pyfunction]
#[pyo3(signature = (x, y, folds, c=1.0, max_iter=1000, tol=1e-4, fit_intercept=true, task_id=None))]
pub fn parallel_cv_cls(x: &PyAny, y: &PyAny, folds: Vec<(Vec<usize>, Vec<usize>)>, c: f64, max_iter: usize, tol: f64, fit_intercept: bool, task_id: Option<u64>) -> PyResult<Vec<f64>> {
    use rayon::prelude::*;
    use crate::ml::cache::{task_load, task_update, task_complete, PartialState, TaskStatus};
    let (xf, _n, p) = extract_flat(x)?;
    let yl = extract_labels(y)?;
    let n_folds = folds.len();
    let fp = crate::ml::cache::Fp::new("parallel_cv_cls")
        .f64(c).usize(max_iter).f64(tol).bval(fit_intercept)
        .data(&xf, _n, p).labels(&yl).usize(n_folds).finish();
    let _h = crate::ml::cache::TaskHandle::auto("parallel_cv_cls", fp);
    let effective_id = task_id.or(Some(_h.id));
    let run_fold = |(train_idx, test_idx): (Vec<usize>, Vec<usize>)| -> f64 {
        let n_tr = train_idx.len();
        let n_te = test_idx.len();
        let mut x_tr = vec![0.0f64; n_tr * p];
        let mut y_tr = vec![0i32; n_tr];
        let mut x_te = vec![0.0f64; n_te * p];
        let mut y_te = vec![0i32; n_te];
        for (i, &idx) in train_idx.iter().enumerate() {
            x_tr[i * p..(i + 1) * p].copy_from_slice(&xf[idx * p..(idx + 1) * p]);
            y_tr[i] = yl[idx];
        }
        for (i, &idx) in test_idx.iter().enumerate() {
            x_te[i * p..(i + 1) * p].copy_from_slice(&xf[idx * p..(idx + 1) * p]);
            y_te[i] = yl[idx];
        }
        let mut lr = crate::ml::linear::logistic::LogisticRegression::new(c, max_iter, tol, fit_intercept);
        lr.fit(&x_tr, n_tr, p, &y_tr);
        let preds = lr.predict(&x_te, n_te, p);
        crate::ml::metrics::classification::accuracy_score(&y_te, &preds)
    };
    if let Some(id) = effective_id {
        let mut partial = task_load(id)
            .filter(|e| matches!(e.status, TaskStatus::Running { .. }))
            .map(|e| e.partial)
            .unwrap_or_default();
        if partial.completed_folds >= n_folds && !partial.fold_scores.is_empty() {
            _h.complete(&partial);
            return Ok(partial.fold_scores);
        }
        for fold in folds.into_iter().skip(partial.completed_folds) {
            partial.fold_scores.push(run_fold(fold));
            partial.completed_folds += 1;
            task_update(id, &partial, partial.completed_folds as f32 / n_folds as f32);
        }
        task_complete(id, &partial);
        _h.complete(&partial);
        return Ok(partial.fold_scores);
    }
    let scores: Vec<f64> = folds.into_par_iter().map(run_fold).collect();
    _h.complete(&PartialState::default());
    Ok(scores)
}

#[pyfunction]
pub fn ml_checkpoint_clear(id: u64) { crate::ml::cache::checkpoint_clear(id); }

#[pyfunction]
pub fn ml_checkpoint_list() -> Vec<u64> { crate::ml::cache::checkpoint_list() }

#[pyfunction]
pub fn task_new(task_type: &str) -> u64 {
    crate::ml::cache::task_create(task_type)
}

#[pyfunction]
pub fn task_info(py: Python<'_>, task_id: u64) -> PyResult<Option<PyObject>> {
    let Some(entry) = crate::ml::cache::task_load(task_id) else { return Ok(None); };
    let d = pyo3::types::PyDict::new_bound(py);
    d.set_item("task_id", entry.task_id)?;
    d.set_item("task_type", &entry.task_type)?;
    d.set_item("created_ms", entry.created_ms)?;
    d.set_item("updated_ms", entry.updated_ms)?;
    match &entry.status {
        crate::ml::cache::TaskStatus::Running { progress } => {
            d.set_item("status", "running")?;
            d.set_item("progress", *progress)?;
        }
        crate::ml::cache::TaskStatus::Completed => {
            d.set_item("status", "completed")?;
            d.set_item("progress", 1.0f32)?;
        }
        crate::ml::cache::TaskStatus::Failed { error } => {
            d.set_item("status", "failed")?;
            d.set_item("error", error.as_str())?;
        }
    }
    Ok(Some(d.into()))
}

#[pyfunction]
pub fn task_list(py: Python<'_>) -> PyResult<PyObject> {
    let entries = crate::ml::cache::task_list_all();
    let list = pyo3::types::PyList::empty_bound(py);
    for entry in entries {
        let d = pyo3::types::PyDict::new_bound(py);
        d.set_item("task_id", entry.task_id)?;
        d.set_item("task_type", &entry.task_type)?;
        d.set_item("created_ms", entry.created_ms)?;
        d.set_item("updated_ms", entry.updated_ms)?;
        match &entry.status {
            crate::ml::cache::TaskStatus::Running { progress } => {
                d.set_item("status", "running")?;
                d.set_item("progress", *progress)?;
            }
            crate::ml::cache::TaskStatus::Completed => {
                d.set_item("status", "completed")?;
                d.set_item("progress", 1.0f32)?;
            }
            crate::ml::cache::TaskStatus::Failed { error } => {
                d.set_item("status", "failed")?;
                d.set_item("error", error.as_str())?;
            }
        }
        list.append(d.into_py(py))?;
    }
    Ok(list.into())
}

#[pyfunction]
pub fn task_delete(task_id: u64) {
    crate::ml::cache::task_delete(task_id);
}

#[pyfunction]
pub fn task_dir() -> String {
    crate::ml::cache::task_dir_path()
}

#[pyfunction]
#[pyo3(signature = (max_age_hours=24))]
pub fn task_cleanup(max_age_hours: u64) -> usize {
    crate::ml::cache::task_cleanup_old(max_age_hours.saturating_mul(3_600_000))
}

fn parse_param_grid(param_grid: &pyo3::types::PyDict) -> PyResult<(Vec<String>, Vec<Vec<String>>)> {
    let mut param_names = Vec::new();
    let mut param_values = Vec::new();
    for (key, val) in param_grid.iter() {
        let name: String = key.extract()?;
        let py_list: Vec<&PyAny> = val.extract()?;
        let mut values = Vec::new();
        for item in py_list {
            if item.is_instance_of::<pyo3::types::PyBool>() {
                let b: bool = item.extract()?;
                values.push(if b { "true".to_string() } else { "false".to_string() });
            } else if let Ok(s) = item.extract::<String>() {
                values.push(s);
            } else if let Ok(f) = item.extract::<f64>() {
                values.push(f.to_string());
            } else {
                values.push(item.str()?.to_str()?.to_string());
            }
        }
        param_names.push(name);
        param_values.push(values);
    }
    Ok((param_names, param_values))
}

fn build_best_params(
    param_names: &[String],
    param_values: &[Vec<String>],
    param_sizes: &[usize],
    best_idx: usize,
) -> std::collections::HashMap<String, String> {
    let indices = crate::ml::model_selection::grid_search::decode_combo(best_idx, param_sizes);
    param_names.iter().enumerate()
        .map(|(i, name)| (name.clone(), param_values[i][indices[i]].clone()))
        .collect()
}

fn build_all_params(
    param_names: &[String],
    param_values: &[Vec<String>],
    param_sizes: &[usize],
    n_combos: usize,
) -> Vec<std::collections::HashMap<String, String>> {
    (0..n_combos).map(|idx| build_best_params(param_names, param_values, param_sizes, idx)).collect()
}

#[pyclass(module = "seraplot", name = "GridSearchCV")]
pub struct PyGridSearchCV {
    estimator: String,
    param_names: Vec<String>,
    param_values: Vec<Vec<String>>,
    cv: usize,
    seed: u64,
    scoring: String,
    best_score: f64,
    best_params: std::collections::HashMap<String, String>,
    cv_results_mean: Vec<f64>,
    cv_results_std: Vec<f64>,
    fitted: bool,
}

#[pymethods]
impl PyGridSearchCV {
    #[new]
    #[pyo3(signature = (estimator, param_grid, cv=5, seed=42, scoring="auto"))]
    fn py_new(estimator: &PyAny, param_grid: &pyo3::types::PyDict, cv: usize, seed: u64, scoring: &str) -> PyResult<Self> {
        let est_name = extract_estimator_name(estimator)?;
        let (param_names, param_values) = parse_param_grid(param_grid)?;
        Ok(Self {
            estimator: est_name,
            param_names,
            param_values,
            cv, seed,
            scoring: scoring.to_string(),
            best_score: f64::NEG_INFINITY,
            best_params: std::collections::HashMap::new(),
            cv_results_mean: Vec::new(),
            cv_results_std: Vec::new(),
            fitted: false,
        })
    }

    #[pyo3(signature = (x, y, task_id=None))]
    fn fit(&mut self, x: &PyAny, y: &PyAny, task_id: Option<u64>) -> PyResult<()> {
        use crate::ml::model_selection::grid_search::*;
        let (xf, n, p) = extract_flat(x)?;
        let fp = crate::ml::cache::Fp::new("GridSearchCV.fit")
            .str(&self.estimator).usize(self.cv).u64(self.seed)
            .str(&self.scoring).data(&xf, n, p).finish();
        let _h = crate::ml::cache::TaskHandle::auto("GridSearchCV.fit", fp);
        let effective_id = task_id.or(Some(_h.id));
        let param_sizes: Vec<usize> = self.param_values.iter().map(|v| v.len()).collect();
        let total = n_combinations(&param_sizes);
        let est = self.estimator.clone();
        let pn = self.param_names.clone();
        let pv = self.param_values.clone();
        let ps = param_sizes.clone();
        let sc = self.scoring.clone();
        let folds = if is_classifier(&est) {
            let yl = extract_labels(y)?;
            precompute_folds_cls(&xf, n, p, &yl, self.cv, self.seed)
        } else {
            let yt = extract_targets(y)?;
            precompute_folds_reg(&xf, n, p, &yt, self.cv, self.seed)
        };
        let caches = compute_caches(&est, &folds, &pn, &pv);
        let result = grid_search_parallel_cached_resumable(total, &folds, &caches, effective_id, |combo_idx, fold, cache| {
            eval_model_scored(&est, &pn, &pv, &ps, combo_idx, fold, cache, &sc)
        });
        self.best_score = result.best_score;
        self.best_params = build_best_params(&self.param_names, &self.param_values, &param_sizes, result.best_params_idx);
        self.cv_results_mean = result.cv_results;
        self.cv_results_std = result.cv_std;
        self.fitted = true;
        _h.complete(&crate::ml::cache::PartialState::default());
        Ok(())
    }

    #[getter]
    fn best_score_(&self) -> f64 { self.best_score }

    #[getter]
    fn best_params_(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new_bound(py);
        for (k, v) in &self.best_params {
            if let Ok(f) = v.parse::<f64>() {
                if v.contains('.') { dict.set_item(k, f)?; }
                else { dict.set_item(k, f as i64)?; }
            } else if v == "true" || v == "True" { dict.set_item(k, true)?; }
            else if v == "false" || v == "False" { dict.set_item(k, false)?; }
            else { dict.set_item(k, v)?; }
        }
        Ok(dict.into())
    }

    #[getter]
    fn cv_results_(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new_bound(py);
        dict.set_item("mean_test_score", self.cv_results_mean.clone())?;
        dict.set_item("std_test_score", self.cv_results_std.clone())?;
        let param_sizes: Vec<usize> = self.param_values.iter().map(|v| v.len()).collect();
        let total = crate::ml::model_selection::grid_search::n_combinations(&param_sizes);
        let params = build_all_params(&self.param_names, &self.param_values, &param_sizes, total);
        let py_params: Vec<PyObject> = params.iter().map(|p| {
            let d = pyo3::types::PyDict::new_bound(py);
            for (k, v) in p { let _ = d.set_item(k, v); }
            d.into()
        }).collect();
        dict.set_item("params", py_params)?;
        let mut ranks: Vec<(usize, f64)> = self.cv_results_mean.iter().enumerate().map(|(i, &s)| (i, s)).collect();
        ranks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let mut rank_arr = vec![0usize; ranks.len()];
        for (rank, (idx, _)) in ranks.iter().enumerate() { rank_arr[*idx] = rank + 1; }
        dict.set_item("rank_test_score", rank_arr)?;
        Ok(dict.into())
    }
}

#[pyclass(module = "seraplot", name = "RandomizedSearchCV")]
pub struct PyRandomizedSearchCV {
    estimator: String,
    param_names: Vec<String>,
    param_values: Vec<Vec<String>>,
    n_iter: usize,
    cv: usize,
    seed: u64,
    scoring: String,
    best_score: f64,
    best_params: std::collections::HashMap<String, String>,
    cv_results_mean: Vec<f64>,
    cv_results_std: Vec<f64>,
    fitted: bool,
}

#[pymethods]
impl PyRandomizedSearchCV {
    #[new]
    #[pyo3(signature = (estimator, param_distributions, n_iter=10, cv=5, seed=42, scoring="auto"))]
    fn py_new(estimator: &PyAny, param_distributions: &pyo3::types::PyDict, n_iter: usize, cv: usize, seed: u64, scoring: &str) -> PyResult<Self> {
        let est_name = extract_estimator_name(estimator)?;
        let (param_names, param_values) = parse_param_grid(param_distributions)?;
        Ok(Self {
            estimator: est_name,
            param_names, param_values, n_iter, cv, seed,
            scoring: scoring.to_string(),
            best_score: f64::NEG_INFINITY,
            best_params: std::collections::HashMap::new(),
            cv_results_mean: Vec::new(),
            cv_results_std: Vec::new(),
            fitted: false,
        })
    }

    #[pyo3(signature = (x, y, task_id=None))]
    fn fit(&mut self, x: &PyAny, y: &PyAny, task_id: Option<u64>) -> PyResult<()> {
        use crate::ml::model_selection::grid_search::*;
        let (xf, n, p) = extract_flat(x)?;
        let fp = crate::ml::cache::Fp::new("RandomizedSearchCV.fit")
            .str(&self.estimator).usize(self.cv).u64(self.seed).usize(self.n_iter)
            .str(&self.scoring).data(&xf, n, p).finish();
        let _h = crate::ml::cache::TaskHandle::auto("RandomizedSearchCV.fit", fp);
        let effective_id = task_id.or(Some(_h.id));
        let param_sizes: Vec<usize> = self.param_values.iter().map(|v| v.len()).collect();
        let total = n_combinations(&param_sizes);
        let combo_indices = sample_indices(total, self.n_iter, self.seed);
        let est = self.estimator.clone();
        let pn = self.param_names.clone();
        let pv = self.param_values.clone();
        let ps = param_sizes.clone();
        let sc = self.scoring.clone();
        let folds = if is_classifier(&est) {
            let yl = extract_labels(y)?;
            precompute_folds_cls(&xf, n, p, &yl, self.cv, self.seed)
        } else {
            let yt = extract_targets(y)?;
            precompute_folds_reg(&xf, n, p, &yt, self.cv, self.seed)
        };
        let caches = compute_caches(&est, &folds, &pn, &pv);
        let result = randomized_search_parallel_cached_resumable(&combo_indices, &folds, &caches, effective_id, |combo_idx, fold, cache| {
            eval_model_scored(&est, &pn, &pv, &ps, combo_idx, fold, cache, &sc)
        });
        self.best_score = result.best_score;
        self.best_params = build_best_params(&self.param_names, &self.param_values, &param_sizes, result.best_params_idx);
        self.cv_results_mean = result.cv_results;
        self.cv_results_std = result.cv_std;
        self.fitted = true;
        _h.complete(&crate::ml::cache::PartialState::default());
        Ok(())
    }

    #[getter]
    fn best_score_(&self) -> f64 { self.best_score }

    #[getter]
    fn best_params_(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new_bound(py);
        for (k, v) in &self.best_params {
            if let Ok(f) = v.parse::<f64>() {
                if v.contains('.') { dict.set_item(k, f)?; }
                else { dict.set_item(k, f as i64)?; }
            } else if v == "true" || v == "True" { dict.set_item(k, true)?; }
            else if v == "false" || v == "False" { dict.set_item(k, false)?; }
            else { dict.set_item(k, v)?; }
        }
        Ok(dict.into())
    }

    #[getter]
    fn cv_results_(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new_bound(py);
        dict.set_item("mean_test_score", self.cv_results_mean.clone())?;
        dict.set_item("std_test_score", self.cv_results_std.clone())?;
        Ok(dict.into())
    }
}

#[pyclass(module = "seraplot", name = "HalvingGridSearchCV")]
pub struct PyHalvingGridSearchCV {
    estimator: String,
    param_names: Vec<String>,
    param_values: Vec<Vec<String>>,
    cv: usize,
    factor: usize,
    seed: u64,
    scoring: String,
    best_score: f64,
    best_params: std::collections::HashMap<String, String>,
    n_iterations: usize,
    fitted: bool,
}

#[pymethods]
impl PyHalvingGridSearchCV {
    #[new]
    #[pyo3(signature = (estimator, param_grid, cv=5, factor=3, seed=42, scoring="auto"))]
    fn py_new(estimator: &PyAny, param_grid: &pyo3::types::PyDict, cv: usize, factor: usize, seed: u64, scoring: &str) -> PyResult<Self> {
        let est_name = extract_estimator_name(estimator)?;
        let (param_names, param_values) = parse_param_grid(param_grid)?;
        Ok(Self {
            estimator: est_name,
            param_names, param_values, cv, factor: factor.max(2), seed,
            scoring: scoring.to_string(),
            best_score: f64::NEG_INFINITY,
            best_params: std::collections::HashMap::new(),
            n_iterations: 0,
            fitted: false,
        })
    }

    #[pyo3(signature = (x, y, task_id=None))]
    fn fit(&mut self, x: &PyAny, y: &PyAny, task_id: Option<u64>) -> PyResult<()> {
        use crate::ml::model_selection::grid_search::*;
        use crate::ml::cache::{task_load, task_update, task_complete, PartialState, TaskStatus};
        let (xf, n, p) = extract_flat(x)?;
        let fp = crate::ml::cache::Fp::new("HalvingGridSearchCV.fit")
            .str(&self.estimator).usize(self.cv).u64(self.seed).usize(self.factor)
            .str(&self.scoring).data(&xf, n, p).finish();
        let _h = crate::ml::cache::TaskHandle::auto("HalvingGridSearchCV.fit", fp);
        let effective_id = task_id.or(Some(_h.id));
        let param_sizes: Vec<usize> = self.param_values.iter().map(|v| v.len()).collect();
        let total = n_combinations(&param_sizes);
        let est = self.estimator.clone();
        let pn = self.param_names.clone();
        let pv = self.param_values.clone();
        let ps = param_sizes.clone();
        let fac = self.factor;
        let sc = self.scoring.clone();
        let n_iters = (total as f64).log(fac as f64).ceil() as usize;
        let min_resources = (n as f64 / fac.pow(n_iters as u32) as f64).max(1.0) as usize;
        let full_folds = if is_classifier(&est) {
            let yl = extract_labels(y)?;
            precompute_folds_cls(&xf, n, p, &yl, self.cv, self.seed)
        } else {
            let yt = extract_targets(y)?;
            precompute_folds_reg(&xf, n, p, &yt, self.cv, self.seed)
        };
        let saved = effective_id
            .and_then(|id| task_load(id))
            .filter(|e| matches!(e.status, TaskStatus::Running { .. }))
            .map(|e| e.partial);
        let (mut candidates, mut resource, mut iteration) = match saved {
            Some(ref p) if !p.halving_candidates.is_empty() =>
                (p.halving_candidates.clone(), p.halving_resource, p.halving_iteration),
            _ => ((0..total).collect::<Vec<_>>(), min_resources, 0),
        };
        while candidates.len() > 1 && resource <= n {
            let sub_folds: Vec<FoldData> = full_folds.iter().map(|f| subsample_fold(f, resource)).collect();
            let caches = compute_caches(&est, &sub_folds, &pn, &pv);
            let scores: Vec<(usize, f64)> = candidates.par_iter().map(|&combo_idx| {
                let mean: f64 = sub_folds.iter().zip(caches.iter())
                    .map(|(fold, cache)| eval_model_scored(&est, &pn, &pv, &ps, combo_idx, fold, cache, &sc))
                    .sum::<f64>() / sub_folds.len() as f64;
                (combo_idx, mean)
            }).collect();
            let mut sorted = scores;
            sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            let keep = (candidates.len() / fac).max(1);
            candidates = sorted[..keep].iter().map(|&(idx, _)| idx).collect();
            resource = (resource * fac).min(n);
            iteration += 1;
            if let Some(id) = effective_id {
                let partial = PartialState { halving_candidates: candidates.clone(), halving_resource: resource, halving_iteration: iteration, ..PartialState::default() };
                task_update(id, &partial, iteration as f32 / n_iters.max(1) as f32);
            }
        }
        let best_combo = candidates[0];
        let final_caches = compute_caches(&est, &full_folds, &pn, &pv);
        let best_score: f64 = full_folds.iter().zip(final_caches.iter())
            .map(|(fold, cache)| eval_model_scored(&est, &pn, &pv, &ps, best_combo, fold, cache, &sc))
            .sum::<f64>() / full_folds.len() as f64;
        if let Some(id) = effective_id { task_complete(id, &PartialState::default()); }
        _h.complete(&PartialState::default());
        self.best_score = best_score;
        self.best_params = build_best_params(&self.param_names, &self.param_values, &param_sizes, best_combo);
        self.n_iterations = iteration;
        self.fitted = true;
        Ok(())
    }

    #[getter]
    fn best_score_(&self) -> f64 { self.best_score }

    #[getter]
    fn best_params_(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new_bound(py);
        for (k, v) in &self.best_params {
            if let Ok(f) = v.parse::<f64>() {
                if v.contains('.') { dict.set_item(k, f)?; }
                else { dict.set_item(k, f as i64)?; }
            } else if v == "true" || v == "True" { dict.set_item(k, true)?; }
            else if v == "false" || v == "False" { dict.set_item(k, false)?; }
            else { dict.set_item(k, v)?; }
        }
        Ok(dict.into())
    }

    #[getter]
    fn n_iterations_(&self) -> usize { self.n_iterations }
}

#[pyclass(module = "seraplot", name = "HalvingRandomSearchCV")]
pub struct PyHalvingRandomSearchCV {
    estimator: String,
    param_names: Vec<String>,
    param_values: Vec<Vec<String>>,
    n_candidates: usize,
    cv: usize,
    factor: usize,
    seed: u64,
    scoring: String,
    best_score: f64,
    best_params: std::collections::HashMap<String, String>,
    n_iterations: usize,
    fitted: bool,
}

#[pymethods]
impl PyHalvingRandomSearchCV {
    #[new]
    #[pyo3(signature = (estimator, param_distributions, n_candidates=256, cv=5, factor=3, seed=42, scoring="auto"))]
    fn py_new(estimator: &PyAny, param_distributions: &pyo3::types::PyDict, n_candidates: usize, cv: usize, factor: usize, seed: u64, scoring: &str) -> PyResult<Self> {
        let est_name = extract_estimator_name(estimator)?;
        let (param_names, param_values) = parse_param_grid(param_distributions)?;
        Ok(Self {
            estimator: est_name,
            param_names, param_values, n_candidates, cv, factor: factor.max(2), seed,
            scoring: scoring.to_string(),
            best_score: f64::NEG_INFINITY,
            best_params: std::collections::HashMap::new(),
            n_iterations: 0,
            fitted: false,
        })
    }

    #[pyo3(signature = (x, y, task_id=None))]
    fn fit(&mut self, x: &PyAny, y: &PyAny, task_id: Option<u64>) -> PyResult<()> {
        use crate::ml::model_selection::grid_search::*;
        use crate::ml::cache::{task_load, task_update, task_complete, PartialState, TaskStatus};
        let (xf, n, p) = extract_flat(x)?;
        let fp = crate::ml::cache::Fp::new("HalvingRandomSearchCV.fit")
            .str(&self.estimator).usize(self.cv).u64(self.seed).usize(self.factor)
            .usize(self.n_candidates).str(&self.scoring).data(&xf, n, p).finish();
        let _h = crate::ml::cache::TaskHandle::auto("HalvingRandomSearchCV.fit", fp);
        let effective_id = task_id.or(Some(_h.id));
        let param_sizes: Vec<usize> = self.param_values.iter().map(|v| v.len()).collect();
        let total = n_combinations(&param_sizes);
        let est = self.estimator.clone();
        let pn = self.param_names.clone();
        let pv = self.param_values.clone();
        let ps = param_sizes.clone();
        let fac = self.factor;
        let sc = self.scoring.clone();
        let full_folds = if is_classifier(&est) {
            let yl = extract_labels(y)?;
            precompute_folds_cls(&xf, n, p, &yl, self.cv, self.seed)
        } else {
            let yt = extract_targets(y)?;
            precompute_folds_reg(&xf, n, p, &yt, self.cv, self.seed)
        };
        let saved = effective_id
            .and_then(|id| task_load(id))
            .filter(|e| matches!(e.status, TaskStatus::Running { .. }))
            .map(|e| e.partial);
        let initial_candidates = sample_indices(total, self.n_candidates, self.seed);
        let n_iters = (initial_candidates.len() as f64).log(fac as f64).ceil() as usize;
        let min_resources = (n as f64 / fac.pow(n_iters as u32) as f64).max(1.0) as usize;
        let (mut candidates, mut resource, mut iteration) = match saved {
            Some(ref p) if !p.halving_candidates.is_empty() =>
                (p.halving_candidates.clone(), p.halving_resource, p.halving_iteration),
            _ => (initial_candidates, min_resources, 0),
        };
        while candidates.len() > 1 && resource <= n {
            let sub_folds: Vec<FoldData> = full_folds.iter().map(|f| subsample_fold(f, resource)).collect();
            let caches = compute_caches(&est, &sub_folds, &pn, &pv);
            let scores: Vec<(usize, f64)> = candidates.par_iter().map(|&combo_idx| {
                let mean: f64 = sub_folds.iter().zip(caches.iter())
                    .map(|(fold, cache)| eval_model_scored(&est, &pn, &pv, &ps, combo_idx, fold, cache, &sc))
                    .sum::<f64>() / sub_folds.len() as f64;
                (combo_idx, mean)
            }).collect();
            let mut sorted = scores;
            sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            let keep = (candidates.len() / fac).max(1);
            candidates = sorted[..keep].iter().map(|&(idx, _)| idx).collect();
            resource = (resource * fac).min(n);
            iteration += 1;
            if let Some(id) = effective_id {
                let partial = PartialState { halving_candidates: candidates.clone(), halving_resource: resource, halving_iteration: iteration, ..PartialState::default() };
                task_update(id, &partial, iteration as f32 / n_iters.max(1) as f32);
            }
        }
        let best_combo = candidates[0];
        let final_caches = compute_caches(&est, &full_folds, &pn, &pv);
        let best_score: f64 = full_folds.iter().zip(final_caches.iter())
            .map(|(fold, cache)| eval_model_scored(&est, &pn, &pv, &ps, best_combo, fold, cache, &sc))
            .sum::<f64>() / full_folds.len() as f64;
        if let Some(id) = effective_id { task_complete(id, &PartialState::default()); }
        _h.complete(&PartialState::default());
        self.best_score = best_score;
        self.best_params = build_best_params(&self.param_names, &self.param_values, &param_sizes, best_combo);
        self.n_iterations = iteration;
        self.fitted = true;
        Ok(())
    }

    #[getter]
    fn best_score_(&self) -> f64 { self.best_score }

    #[getter]
    fn best_params_(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new_bound(py);
        for (k, v) in &self.best_params {
            if let Ok(f) = v.parse::<f64>() {
                if v.contains('.') { dict.set_item(k, f)?; }
                else { dict.set_item(k, f as i64)?; }
            } else if v == "true" || v == "True" { dict.set_item(k, true)?; }
            else if v == "false" || v == "False" { dict.set_item(k, false)?; }
            else { dict.set_item(k, v)?; }
        }
        Ok(dict.into())
    }

    #[getter]
    fn n_iterations_(&self) -> usize { self.n_iterations }
}

#[pyfunction]
fn ml_registry(py: Python<'_>) -> PyResult<PyObject> {
    let dict = pyo3::types::PyDict::new_bound(py);
    dict.set_item("regression", vec!["LinearRegression", "Ridge", "Lasso", "ElasticNet", "SGDRegressor", "DecisionTreeRegressor", "RandomForestRegressor", "GradientBoostingRegressor", "AdaBoostRegressor", "KNeighborsRegressor", "LinearSVR"])?;
    dict.set_item("classification", vec!["LogisticRegression", "SGDClassifier", "DecisionTreeClassifier", "RandomForestClassifier", "GradientBoostingClassifier", "AdaBoostClassifier", "KNeighborsClassifier", "NearestCentroid", "GaussianNB", "MultinomialNB", "BernoulliNB", "LinearSVC", "RidgeClassifier"])?;
    dict.set_item("preprocessing", vec!["StandardScaler", "MinMaxScaler", "RobustScaler", "MaxAbsScaler", "Normalizer", "LabelEncoder"])?;
    dict.set_item("dimensionality_reduction", vec!["PCA", "TruncatedSVD"])?;
    dict.set_item("model_selection", vec!["train_test_split", "StratifiedKFold", "GridSearchCV", "RandomizedSearchCV", "HalvingGridSearchCV", "HalvingRandomSearchCV"])?;
    dict.set_item("metrics", vec!["accuracy_score", "mean_squared_error", "mean_absolute_error", "r2_score", "f1_score", "precision_score", "recall_score", "confusion_matrix", "classification_report"])?;
    Ok(dict.into())
}

impl_doc!(PyLinearRegression, crate::bindings::commands::docs::DOC_LINEAR_REGRESSION);
impl_doc!(PyRidge, crate::bindings::commands::docs::DOC_RIDGE);
impl_doc!(PyLasso, crate::bindings::commands::docs::DOC_LASSO);
impl_doc!(PyElasticNet, crate::bindings::commands::docs::DOC_ELASTIC_NET);
impl_doc!(PyLogisticRegression, crate::bindings::commands::docs::DOC_LOGISTIC_REGRESSION);
impl_doc!(PySGDClassifier, crate::bindings::commands::docs::DOC_SGD_CLASSIFIER);
impl_doc!(PySGDRegressor, crate::bindings::commands::docs::DOC_SGD_REGRESSOR);
impl_doc!(PyDecisionTreeClassifier, crate::bindings::commands::docs::DOC_DECISION_TREE_CLASSIFIER);
impl_doc!(PyDecisionTreeRegressor, crate::bindings::commands::docs::DOC_DECISION_TREE_REGRESSOR);
impl_doc!(PyRandomForestClassifier, crate::bindings::commands::docs::DOC_RANDOM_FOREST_CLASSIFIER);
impl_doc!(PyRandomForestRegressor, crate::bindings::commands::docs::DOC_RANDOM_FOREST_REGRESSOR);
impl_doc!(PyGradientBoostingClassifier, crate::bindings::commands::docs::DOC_GRADIENT_BOOSTING_CLASSIFIER);
impl_doc!(PyGradientBoostingRegressor, crate::bindings::commands::docs::DOC_GRADIENT_BOOSTING_REGRESSOR);
impl_doc!(PyAdaBoostClassifier, crate::bindings::commands::docs::DOC_ADABOOST_CLASSIFIER);
impl_doc!(PyAdaBoostRegressor, crate::bindings::commands::docs::DOC_ADABOOST_REGRESSOR);
impl_doc!(PyKNeighborsClassifier, crate::bindings::commands::docs::DOC_KNEIGHBORS_CLASSIFIER);
impl_doc!(PyKNeighborsRegressor, crate::bindings::commands::docs::DOC_KNEIGHBORS_REGRESSOR);
impl_doc!(PyNearestCentroid, crate::bindings::commands::docs::DOC_NEAREST_CENTROID);
impl_doc!(PyGaussianNB, crate::bindings::commands::docs::DOC_GAUSSIAN_NB);
impl_doc!(PyMultinomialNB, crate::bindings::commands::docs::DOC_MULTINOMIAL_NB);
impl_doc!(PyBernoulliNB, crate::bindings::commands::docs::DOC_BERNOULLI_NB);
impl_doc!(PyLinearSVC, crate::bindings::commands::docs::DOC_LINEAR_SVC);
impl_doc!(PyLinearSVR, crate::bindings::commands::docs::DOC_LINEAR_SVR);
impl_doc!(PyRidgeClassifier, crate::bindings::commands::docs::DOC_RIDGE_CLASSIFIER);

#[pyclass(module = "seraplot", name = "OneHotEncoder")]
pub struct PyOneHotEncoder { inner: crate::ml::preprocessing::encoders::OneHotEncoder, p: usize }

#[pymethods]
impl PyOneHotEncoder {
    #[new]
    fn py_new() -> Self { Self { inner: crate::ml::preprocessing::encoders::OneHotEncoder::new(), p: 0 } }
    fn fit(&mut self, x: Vec<Vec<String>>) -> PyResult<()> {
        let n = x.len();
        let p = if n == 0 { 0 } else { x[0].len() };
        self.p = p;
        let flat: Vec<String> = x.into_iter().flatten().collect();
        self.inner.fit(&flat, n, p);
        Ok(())
    }
    fn partial_fit(&mut self, x: Vec<Vec<String>>) -> PyResult<()> {
        let n = x.len();
        let p = if n == 0 { 0 } else { x[0].len() };
        if self.p == 0 { self.p = p; }
        let flat: Vec<String> = x.into_iter().flatten().collect();
        self.inner.partial_fit(&flat, n, p);
        Ok(())
    }
    fn transform(&self, py: Python<'_>, x: Vec<Vec<String>>) -> PyResult<PyObject> {
        let n = x.len();
        let flat: Vec<String> = x.into_iter().flatten().collect();
        let out = self.inner.transform(&flat, n, self.p);
        flat_to_np2d(py, out, n, self.inner.n_output_features())
    }
    fn fit_transform(&mut self, py: Python<'_>, x: Vec<Vec<String>>) -> PyResult<PyObject> {
        self.fit(x.clone())?;
        self.transform(py, x)
    }
    #[getter] fn categories_(&self) -> Vec<Vec<String>> { self.inner.categories.clone() }
    #[getter] fn n_features_out_(&self) -> usize { self.inner.n_output_features() }
    fn __repr__(&self) -> String { "OneHotEncoder()".to_string() }
}

#[pyclass(module = "seraplot", name = "OrdinalEncoder")]
pub struct PyOrdinalEncoder { inner: crate::ml::preprocessing::encoders::OrdinalEncoder, p: usize }

#[pymethods]
impl PyOrdinalEncoder {
    #[new]
    fn py_new() -> Self { Self { inner: crate::ml::preprocessing::encoders::OrdinalEncoder::new(), p: 0 } }
    fn fit(&mut self, x: Vec<Vec<String>>) -> PyResult<()> {
        let n = x.len();
        let p = if n == 0 { 0 } else { x[0].len() };
        self.p = p;
        let flat: Vec<String> = x.into_iter().flatten().collect();
        self.inner.fit(&flat, n, p);
        Ok(())
    }
    fn partial_fit(&mut self, x: Vec<Vec<String>>) -> PyResult<()> {
        let n = x.len();
        let p = if n == 0 { 0 } else { x[0].len() };
        if self.p == 0 { self.p = p; }
        let flat: Vec<String> = x.into_iter().flatten().collect();
        self.inner.partial_fit(&flat, n, p);
        Ok(())
    }
    fn transform(&self, py: Python<'_>, x: Vec<Vec<String>>) -> PyResult<PyObject> {
        let n = x.len();
        let flat: Vec<String> = x.into_iter().flatten().collect();
        let out = self.inner.transform(&flat, n, self.p);
        flat_to_np2d(py, out, n, self.p)
    }
    fn fit_transform(&mut self, py: Python<'_>, x: Vec<Vec<String>>) -> PyResult<PyObject> {
        self.fit(x.clone())?;
        self.transform(py, x)
    }
    #[getter] fn categories_(&self) -> Vec<Vec<String>> { self.inner.categories.clone() }
    fn __repr__(&self) -> String { "OrdinalEncoder()".to_string() }
}

#[pyclass(module = "seraplot", name = "SimpleImputer")]
pub struct PySimpleImputer { inner: crate::ml::preprocessing::transformers::SimpleImputer }

#[pymethods]
impl PySimpleImputer {
    #[new]
    #[pyo3(signature = (strategy="mean", fill_value=0.0))]
    fn py_new(strategy: &str, fill_value: f64) -> Self {
        Self { inner: crate::ml::preprocessing::transformers::SimpleImputer::new(strategy, fill_value) }
    }
    fn fit(&mut self, x: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        self.inner.fit(&xf, n, p);
        Ok(())
    }
    fn transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let out = self.inner.transform(&xf, n, p);
        flat_to_np2d(py, out, n, p)
    }
    fn fit_transform(&mut self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let out = self.inner.fit_transform(&xf, n, p);
        flat_to_np2d(py, out, n, p)
    }
    #[getter] fn statistics_(&self) -> Vec<f64> { self.inner.statistics.clone() }
    #[getter] fn strategy_(&self) -> String { self.inner.strategy.clone() }
    fn __repr__(&self) -> String { format!("SimpleImputer(strategy='{}')", self.inner.strategy) }
}

#[pyclass(module = "seraplot", name = "PolynomialFeatures")]
pub struct PyPolynomialFeatures { inner: crate::ml::preprocessing::transformers::PolynomialFeatures }

#[pymethods]
impl PyPolynomialFeatures {
    #[new]
    #[pyo3(signature = (degree=2, interaction_only=false, include_bias=true))]
    fn py_new(degree: usize, interaction_only: bool, include_bias: bool) -> Self {
        Self { inner: crate::ml::preprocessing::transformers::PolynomialFeatures::new(degree, interaction_only, include_bias) }
    }
    fn fit(&mut self, x: &PyAny) -> PyResult<()> {
        with_flat(x, |xf, n, p| self.inner.fit(xf, n, p))?;
        Ok(())
    }
    fn transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (out, n) = with_flat(x, |xf, nn, pp| (self.inner.transform(xf, nn, pp), nn))?;
        let cols = self.inner.n_output_features();
        flat_to_np2d(py, out, n, cols)
    }
    fn fit_transform(&mut self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (out, n) = with_flat(x, |xf, nn, pp| (self.inner.fit_transform(xf, nn, pp), nn))?;
        let cols = self.inner.n_output_features();
        flat_to_np2d(py, out, n, cols)
    }
    #[getter] fn n_output_features_(&self) -> usize { self.inner.n_output_features() }
    #[getter] fn degree_(&self) -> usize { self.inner.degree }
    fn __repr__(&self) -> String { format!("PolynomialFeatures(degree={})", self.inner.degree) }
}

#[pyclass(module = "seraplot", name = "KBinsDiscretizer")]
pub struct PyKBinsDiscretizer { inner: crate::ml::preprocessing::transformers::KBinsDiscretizer }

#[pymethods]
impl PyKBinsDiscretizer {
    #[new]
    #[pyo3(signature = (n_bins=5, strategy="quantile"))]
    fn py_new(n_bins: usize, strategy: &str) -> Self {
        Self { inner: crate::ml::preprocessing::transformers::KBinsDiscretizer::new(n_bins, strategy) }
    }
    fn fit(&mut self, x: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        self.inner.fit(&xf, n, p);
        Ok(())
    }
    fn transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let out = self.inner.transform(&xf, n, p);
        flat_to_np2d(py, out, n, p)
    }
    fn fit_transform(&mut self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let out = self.inner.fit_transform(&xf, n, p);
        flat_to_np2d(py, out, n, p)
    }
    #[getter] fn n_bins_(&self) -> usize { self.inner.n_bins }
    fn __repr__(&self) -> String { format!("KBinsDiscretizer(n_bins={}, strategy='{}')", self.inner.n_bins, self.inner.strategy) }
}

#[pyclass(module = "seraplot", name = "PowerTransformer")]
pub struct PyPowerTransformer { inner: crate::ml::preprocessing::transformers::PowerTransformer }

#[pymethods]
impl PyPowerTransformer {
    #[new]
    #[pyo3(signature = (method="yeo-johnson"))]
    fn py_new(method: &str) -> Self {
        Self { inner: crate::ml::preprocessing::transformers::PowerTransformer::new(method) }
    }
    fn fit(&mut self, x: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        self.inner.fit(&xf, n, p);
        Ok(())
    }
    fn transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let out = self.inner.transform(&xf, n, p);
        flat_to_np2d(py, out, n, p)
    }
    fn fit_transform(&mut self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let out = self.inner.fit_transform(&xf, n, p);
        flat_to_np2d(py, out, n, p)
    }
    #[getter] fn lambdas_(&self) -> Vec<f64> { self.inner.lambdas.clone() }
    fn __repr__(&self) -> String { format!("PowerTransformer(method='{}')", self.inner.method) }
}

#[pyclass(module = "seraplot", name = "QuantileTransformer")]
pub struct PyQuantileTransformer { inner: crate::ml::preprocessing::transformers::QuantileTransformer }

#[pymethods]
impl PyQuantileTransformer {
    #[new]
    #[pyo3(signature = (n_quantiles=1000, output_distribution="uniform"))]
    fn py_new(n_quantiles: usize, output_distribution: &str) -> Self {
        Self { inner: crate::ml::preprocessing::transformers::QuantileTransformer::new(n_quantiles, output_distribution) }
    }
    fn fit(&mut self, x: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        self.inner.fit(&xf, n, p);
        Ok(())
    }
    fn transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let out = self.inner.transform(&xf, n, p);
        flat_to_np2d(py, out, n, p)
    }
    fn fit_transform(&mut self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let out = self.inner.fit_transform(&xf, n, p);
        flat_to_np2d(py, out, n, p)
    }
    fn __repr__(&self) -> String { format!("QuantileTransformer(n_quantiles={})", self.inner.n_quantiles) }
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn balanced_accuracy_score(y_true: Vec<i32>, y_pred: Vec<i32>) -> f64 {
    crate::ml::metrics::classification::balanced_accuracy_score(&y_true, &y_pred)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn matthews_corrcoef(y_true: Vec<i32>, y_pred: Vec<i32>) -> f64 {
    crate::ml::metrics::classification::matthews_corrcoef(&y_true, &y_pred)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn cohen_kappa_score(y_true: Vec<i32>, y_pred: Vec<i32>) -> f64 {
    crate::ml::metrics::classification::cohen_kappa_score(&y_true, &y_pred)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn hamming_loss(y_true: Vec<i32>, y_pred: Vec<i32>) -> f64 {
    crate::ml::metrics::classification::hamming_loss(&y_true, &y_pred)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn zero_one_loss(y_true: Vec<i32>, y_pred: Vec<i32>) -> f64 {
    crate::ml::metrics::classification::zero_one_loss(&y_true, &y_pred)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred, pos_label=1))]
pub fn jaccard_score(y_true: Vec<i32>, y_pred: Vec<i32>, pos_label: i32) -> f64 {
    crate::ml::metrics::classification::jaccard_score(&y_true, &y_pred, pos_label)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred, beta=1.0, average="weighted"))]
pub fn fbeta_score(y_true: Vec<i32>, y_pred: Vec<i32>, beta: f64, average: &str) -> f64 {
    let avg = match average {
        "macro" => crate::ml::metrics::classification::Average::Macro,
        "weighted" => crate::ml::metrics::classification::Average::Weighted,
        _ => {
            let mut c: Vec<i32> = y_true.iter().chain(y_pred.iter()).copied().collect();
            c.sort_unstable(); c.dedup();
            crate::ml::metrics::classification::Average::Binary(*c.last().unwrap_or(&1))
        }
    };
    crate::ml::metrics::classification::fbeta_score(&y_true, &y_pred, beta, avg)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_proba, eps=1e-15))]
pub fn log_loss(y_true: Vec<i32>, y_proba: &PyAny, eps: f64) -> PyResult<f64> {
    let (flat, n, k) = extract_flat(y_proba)?;
    Ok(crate::ml::metrics::classification::log_loss(&y_true, &flat, n, k, eps))
}

#[pyfunction]
#[pyo3(signature = (y_true, y_score, eps=1e-15))]
pub fn binary_log_loss(y_true: Vec<i32>, y_score: Vec<f64>, eps: f64) -> f64 {
    crate::ml::metrics::classification::binary_log_loss(&y_true, &y_score, eps)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_score))]
pub fn brier_score_loss(y_true: Vec<i32>, y_score: Vec<f64>) -> f64 {
    crate::ml::metrics::classification::brier_score_loss(&y_true, &y_score)
}

#[pyfunction]
#[pyo3(signature = (y_true, pred_decision))]
pub fn hinge_loss(y_true: Vec<i32>, pred_decision: Vec<f64>) -> f64 {
    crate::ml::metrics::classification::hinge_loss(&y_true, &pred_decision)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_score, pos_label=1))]
pub fn roc_curve(y_true: Vec<i32>, y_score: Vec<f64>, pos_label: i32) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    crate::ml::metrics::classification::roc_curve(&y_true, &y_score, pos_label)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_score, pos_label=1))]
pub fn roc_auc_score(y_true: Vec<i32>, y_score: Vec<f64>, pos_label: i32) -> f64 {
    crate::ml::metrics::classification::roc_auc_score(&y_true, &y_score, pos_label)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_score, pos_label=1))]
pub fn precision_recall_curve(y_true: Vec<i32>, y_score: Vec<f64>, pos_label: i32) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    crate::ml::metrics::classification::precision_recall_curve(&y_true, &y_score, pos_label)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_score, pos_label=1))]
pub fn average_precision_score(y_true: Vec<i32>, y_score: Vec<f64>, pos_label: i32) -> f64 {
    crate::ml::metrics::classification::average_precision_score(&y_true, &y_score, pos_label)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn median_absolute_error(y_true: Vec<f64>, y_pred: Vec<f64>) -> f64 {
    crate::ml::metrics::regression::median_absolute_error(&y_true, &y_pred)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn mean_squared_log_error(y_true: &PyAny, y_pred: &PyAny) -> PyResult<f64> {
    let yt = extract_targets(y_true)?; let yp = extract_targets(y_pred)?;
    Ok(crate::ml::metrics::regression::mean_squared_log_error(&yt, &yp))
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn root_mean_squared_log_error(y_true: Vec<f64>, y_pred: Vec<f64>) -> f64 {
    crate::ml::metrics::regression::root_mean_squared_log_error(&y_true, &y_pred)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred, alpha=0.5))]
pub fn mean_pinball_loss(y_true: Vec<f64>, y_pred: Vec<f64>, alpha: f64) -> f64 {
    crate::ml::metrics::regression::mean_pinball_loss(&y_true, &y_pred, alpha)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn mean_absolute_percentage_error(y_true: Vec<f64>, y_pred: Vec<f64>) -> f64 {
    crate::ml::metrics::regression::mean_absolute_percentage_error(&y_true, &y_pred)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn explained_variance_score(y_true: Vec<f64>, y_pred: Vec<f64>) -> f64 {
    crate::ml::metrics::regression::explained_variance_score(&y_true, &y_pred)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn max_error(y_true: Vec<f64>, y_pred: Vec<f64>) -> f64 {
    crate::ml::metrics::regression::max_error(&y_true, &y_pred)
}

#[pyfunction]
#[pyo3(signature = (x, labels))]
pub fn silhouette_score(x: &PyAny, labels: Vec<i32>) -> PyResult<f64> {
    let (xf, n, p) = extract_flat(x)?;
    Ok(crate::ml::metrics::clustering::silhouette_score(&xf, &labels, n, p))
}

#[pyfunction]
#[pyo3(signature = (x, labels))]
pub fn davies_bouldin_score(x: &PyAny, labels: Vec<i32>) -> PyResult<f64> {
    let (xf, n, p) = extract_flat(x)?;
    Ok(crate::ml::metrics::clustering::davies_bouldin_score(&xf, &labels, n, p))
}

#[pyfunction]
#[pyo3(signature = (x, labels))]
pub fn calinski_harabasz_score(x: &PyAny, labels: Vec<i32>) -> PyResult<f64> {
    let (xf, n, p) = extract_flat(x)?;
    Ok(crate::ml::metrics::clustering::calinski_harabasz_score(&xf, &labels, n, p))
}

#[pyfunction]
#[pyo3(signature = (labels_true, labels_pred))]
pub fn adjusted_rand_score(labels_true: Vec<i32>, labels_pred: Vec<i32>) -> f64 {
    crate::ml::metrics::clustering::adjusted_rand_score(&labels_true, &labels_pred)
}

#[pyfunction]
#[pyo3(signature = (labels_true, labels_pred))]
pub fn normalized_mutual_info_score(labels_true: Vec<i32>, labels_pred: Vec<i32>) -> f64 {
    crate::ml::metrics::clustering::normalized_mutual_info_score(&labels_true, &labels_pred)
}

#[pyfunction]
#[pyo3(signature = (labels_true, labels_pred))]
pub fn fowlkes_mallows_score(labels_true: Vec<i32>, labels_pred: Vec<i32>) -> f64 {
    crate::ml::metrics::clustering::fowlkes_mallows_score(&labels_true, &labels_pred)
}

#[pyfunction]
#[pyo3(signature = (labels_true, labels_pred))]
pub fn homogeneity_score(labels_true: Vec<i32>, labels_pred: Vec<i32>) -> f64 {
    crate::ml::metrics::clustering::homogeneity_score(&labels_true, &labels_pred)
}

#[pyfunction]
#[pyo3(signature = (labels_true, labels_pred))]
pub fn completeness_score(labels_true: Vec<i32>, labels_pred: Vec<i32>) -> f64 {
    crate::ml::metrics::clustering::completeness_score(&labels_true, &labels_pred)
}

#[pyfunction]
#[pyo3(signature = (labels_true, labels_pred))]
pub fn v_measure_score(labels_true: Vec<i32>, labels_pred: Vec<i32>) -> f64 {
    crate::ml::metrics::clustering::v_measure_score(&labels_true, &labels_pred)
}

#[pyfunction]
#[pyo3(signature = (name, kind, payload, params=None, metrics=None, tags=None))]
fn registry_register(py: Python<'_>, name: &str, kind: &str, payload: &str, params: Option<std::collections::HashMap<String, String>>, metrics: Option<std::collections::HashMap<String, f64>>, tags: Option<Vec<String>>) -> PyResult<PyObject> {
    let rec = crate::ml::registry::register(name, kind, payload, params.unwrap_or_default(), metrics.unwrap_or_default(), tags.unwrap_or_default());
    record_to_py(py, &rec)
}

#[pyfunction]
#[pyo3(signature = (name, version=None))]
fn registry_get(py: Python<'_>, name: &str, version: Option<u32>) -> PyResult<PyObject> {
    match crate::ml::registry::get(name, version) {
        Some(r) => record_to_py(py, &r),
        None => Ok(py.None()),
    }
}

#[pyfunction]
fn registry_list(py: Python<'_>) -> PyResult<PyObject> {
    let recs = crate::ml::registry::list();
    let list = pyo3::types::PyList::empty_bound(py);
    for r in &recs { list.append(record_to_py(py, r)?)?; }
    Ok(list.into())
}

#[pyfunction]
#[pyo3(signature = (name, version=None))]
fn registry_delete(name: &str, version: Option<u32>) -> usize {
    crate::ml::registry::delete(name, version)
}

#[pyfunction]
fn registry_clear() -> usize { crate::ml::registry::clear_all() }

#[pyfunction]
fn registry_promote(name: &str, version: u32, tag: &str) -> bool { crate::ml::registry::promote(name, version, tag) }

#[pyfunction]
fn registry_by_tag(py: Python<'_>, name: &str, tag: &str) -> PyResult<PyObject> {
    match crate::ml::registry::by_tag(name, tag) {
        Some(r) => record_to_py(py, &r),
        None => Ok(py.None()),
    }
}

#[pyfunction]
fn registry_dir() -> String { crate::ml::registry::registry_dir().to_string_lossy().to_string() }

fn record_to_py(py: Python<'_>, r: &crate::ml::registry::ModelRecord) -> PyResult<PyObject> {
    let d = pyo3::types::PyDict::new_bound(py);
    d.set_item("name", &r.name)?;
    d.set_item("version", r.version)?;
    d.set_item("kind", &r.kind)?;
    d.set_item("created_ms", r.created_ms)?;
    d.set_item("payload", &r.payload)?;
    d.set_item("tags", r.tags.clone())?;
    let p = pyo3::types::PyDict::new_bound(py);
    for (k, v) in &r.params { p.set_item(k, v)?; }
    d.set_item("params", p)?;
    let mt = pyo3::types::PyDict::new_bound(py);
    for (k, v) in &r.metrics { mt.set_item(k, *v)?; }
    d.set_item("metrics", mt)?;
    Ok(d.into())
}

#[pyfunction]
#[pyo3(signature = (name, table, x, y=None, y_pred=None))]
fn export_powerbi(name: &str, table: &str, x: &PyAny, y: Option<&PyAny>, y_pred: Option<&PyAny>) -> PyResult<String> {
    let (xv, n, p) = extract_flat(x)?;
    let yv = if let Some(yy) = y { Some(extract_targets(yy)?) } else { None };
    let yp = if let Some(yy) = y_pred { Some(extract_targets(yy)?) } else { None };
    let cols = crate::ml::export::powerbi::columns_from_features(p, yv.is_some(), yp.is_some());
    let rows = crate::ml::export::powerbi::rows_from_matrix(&xv, n, p, yv.as_deref(), yp.as_deref());
    let ds = crate::ml::export::powerbi::build_dataset(name, table, &cols, rows);
    Ok(crate::ml::export::powerbi::to_json(&ds))
}

#[pyfunction]
#[pyo3(signature = (name, x, y=None, y_pred=None))]
fn export_tableau_tds(name: &str, x: &PyAny, y: Option<&PyAny>, y_pred: Option<&PyAny>) -> PyResult<String> {
    let (xv, n, p) = extract_flat(x)?;
    let yv = if let Some(yy) = y { Some(extract_targets(yy)?) } else { None };
    let yp = if let Some(yy) = y_pred { Some(extract_targets(yy)?) } else { None };
    let cols = crate::ml::export::tableau::columns_from_features(p, yv.is_some(), yp.is_some());
    let rows = crate::ml::export::tableau::rows_from_matrix(&xv, n, p, yv.as_deref(), yp.as_deref());
    let d = crate::ml::export::tableau::TdsDescriptor { name: name.into(), columns: cols, rows, row_strings: vec![], use_strings: false };
    Ok(crate::ml::export::tableau::to_tds_xml(&d))
}

#[pyfunction]
#[pyo3(signature = (name, x, y=None, y_pred=None))]
fn export_tableau_csv(name: &str, x: &PyAny, y: Option<&PyAny>, y_pred: Option<&PyAny>) -> PyResult<String> {
    let (xv, n, p) = extract_flat(x)?;
    let yv = if let Some(yy) = y { Some(extract_targets(yy)?) } else { None };
    let yp = if let Some(yy) = y_pred { Some(extract_targets(yy)?) } else { None };
    let cols = crate::ml::export::tableau::columns_from_features(p, yv.is_some(), yp.is_some());
    let rows = crate::ml::export::tableau::rows_from_matrix(&xv, n, p, yv.as_deref(), yp.as_deref());
    let d = crate::ml::export::tableau::TdsDescriptor { name: name.into(), columns: cols, rows, row_strings: vec![], use_strings: false };
    Ok(crate::ml::export::tableau::to_csv(&d))
}

#[pyfunction]
fn gpu_devices(py: Python<'_>) -> PyResult<PyObject> {
    let list = pyo3::types::PyList::empty_bound(py);
    for d in crate::ml::gpu::detect_devices() {
        let dd = pyo3::types::PyDict::new_bound(py);
        dd.set_item("backend", d.backend.name())?;
        dd.set_item("name", d.name)?;
        dd.set_item("mem_mb", d.mem_mb)?;
        dd.set_item("available", d.available)?;
        list.append(dd)?;
    }
    Ok(list.into())
}

#[pyfunction]
#[pyo3(signature = (backend=None))]
fn gpu_set_backend(backend: Option<&str>) -> String {
    let b = crate::ml::gpu::Backend::from_str(backend.unwrap_or("auto"));
    let dev = crate::ml::gpu::select(b);
    crate::ml::gpu::set_active(dev.backend);
    dev.backend.name().to_string()
}

#[pyfunction]
fn gpu_active_backend() -> String { crate::ml::gpu::active().name().to_string() }

#[pyfunction]
#[pyo3(signature = (n_rows, n_cols, mem_budget_mb=2048))]
fn cloud_plan(py: Python<'_>, n_rows: usize, n_cols: usize, mem_budget_mb: u64) -> PyResult<PyObject> {
    let p = crate::cloud::planner::plan(n_rows, n_cols, mem_budget_mb);
    let d = pyo3::types::PyDict::new_bound(py);
    d.set_item("n_rows", p.n_rows)?;
    d.set_item("n_cols", p.n_cols)?;
    d.set_item("bytes_total", p.bytes_total)?;
    d.set_item("mem_budget_mb", p.mem_budget_mb)?;
    d.set_item("recommended_workers", p.recommended_workers)?;
    d.set_item("recommended_chunk_rows", p.recommended_chunk_rows)?;
    d.set_item("n_chunks", p.n_chunks)?;
    d.set_item("estimated_seconds", p.estimated_seconds)?;
    d.set_item("strategy", p.strategy)?;
    Ok(d.into())
}

#[pyfunction]
fn cloud_resources(py: Python<'_>) -> PyResult<PyObject> {
    let r = crate::cloud::profile::current();
    let d = pyo3::types::PyDict::new_bound(py);
    d.set_item("cpu_threads", r.cpu_threads)?;
    d.set_item("backend", r.backend)?;
    d.set_item("os", r.os)?;
    d.set_item("arch", r.arch)?;
    d.set_item("registry_dir", r.registry_dir)?;
    d.set_item("tasks_dir", r.tasks_dir)?;
    Ok(d.into())
}

#[pyfunction]
#[pyo3(signature = (path, chunk_rows=100000, has_header=true, delimiter=","))]
fn cloud_count_rows(path: &str, chunk_rows: usize, has_header: bool, delimiter: &str) -> PyResult<usize> {
    let _ = (chunk_rows, delimiter);
    crate::cloud::chunker::count_rows(path, has_header).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
}

#[pyclass(module = "seraplot", name = "WorkerPool")]
pub struct PyWorkerPool { inner: std::sync::Arc<crate::ml::distributed::WorkerPool> }

#[pymethods]
impl PyWorkerPool {
    #[new]
    #[pyo3(signature = (n_workers=0))]
    fn py_new(n_workers: usize) -> Self { Self { inner: crate::ml::distributed::WorkerPool::new(n_workers) } }
    #[getter] fn n_workers(&self) -> usize { self.inner.n_workers }
    fn scatter(&self, n_rows: usize) -> u64 { self.inner.scatter(n_rows) }
    fn shards(&self, py: Python<'_>, handle: u64) -> PyResult<PyObject> {
        let list = pyo3::types::PyList::empty_bound(py);
        for s in self.inner.shards_for(handle) {
            let d = pyo3::types::PyDict::new_bound(py);
            d.set_item("id", s.id)?; d.set_item("start", s.start)?; d.set_item("end", s.end)?;
            list.append(d)?;
        }
        Ok(list.into())
    }
    fn release(&self, handle: u64) { self.inner.release(handle) }
    fn allreduce_mean(&self, vecs: Vec<Vec<f64>>) -> Vec<f64> { self.inner.allreduce_mean(vecs) }
    fn allreduce_sum(&self, vecs: Vec<Vec<f64>>) -> Vec<f64> { self.inner.allreduce_sum(vecs) }
}

pub fn register_ml_classes(m: &PyModule) -> PyResult<()> {
    m.add_class::<PyLinearRegression>()?;
    m.add_class::<PyRidge>()?;
    m.add_class::<PyLasso>()?;
    m.add_class::<PyElasticNet>()?;
    m.add_class::<PySGDRegressor>()?;
    m.add_class::<PyDecisionTreeRegressor>()?;
    m.add_class::<PyRandomForestRegressor>()?;
    m.add_class::<PyGradientBoostingRegressor>()?;
    m.add_class::<PyAdaBoostRegressor>()?;
    m.add_class::<PyKNeighborsRegressor>()?;
    m.add_class::<PyLinearSVR>()?;
    m.add_class::<PyLogisticRegression>()?;
    m.add_class::<PySGDClassifier>()?;
    m.add_class::<PyDecisionTreeClassifier>()?;
    m.add_class::<PyRandomForestClassifier>()?;
    m.add_class::<PyGradientBoostingClassifier>()?;
    m.add_class::<PyAdaBoostClassifier>()?;
    m.add_class::<PyKNeighborsClassifier>()?;
    m.add_class::<PyNearestCentroid>()?;
    m.add_class::<PyGaussianNB>()?;
    m.add_class::<PyMultinomialNB>()?;
    m.add_class::<PyBernoulliNB>()?;
    m.add_class::<PyLinearSVC>()?;
    m.add_class::<PyRidgeClassifier>()?;
    m.add_class::<PyStandardScaler>()?;
    m.add_class::<PyMinMaxScaler>()?;
    m.add_class::<PyRobustScaler>()?;
    m.add_class::<PyMaxAbsScaler>()?;
    m.add_class::<PyNormalizer>()?;
    m.add_class::<PyLabelEncoder>()?;
    m.add_class::<PyPCA>()?;
    m.add_class::<PyTruncatedSVD>()?;
    m.add_class::<PyStratifiedKFold>()?;
    m.add_class::<PyGridSearchCV>()?;
    m.add_class::<PyRandomizedSearchCV>()?;
    m.add_class::<PyHalvingGridSearchCV>()?;
    m.add_class::<PyHalvingRandomSearchCV>()?;
    m.add_function(wrap_pyfunction!(accuracy_score, m)?)?;
    m.add_function(wrap_pyfunction!(mean_squared_error, m)?)?;
    m.add_function(wrap_pyfunction!(mean_absolute_error, m)?)?;
    m.add_function(wrap_pyfunction!(r2_score, m)?)?;
    m.add_function(wrap_pyfunction!(root_mean_squared_error, m)?)?;
    m.add_function(wrap_pyfunction!(cross_val_score, m)?)?;
    m.add_function(wrap_pyfunction!(classification_report, m)?)?;
    m.add_function(wrap_pyfunction!(f1_score, m)?)?;
    m.add_function(wrap_pyfunction!(precision_score, m)?)?;
    m.add_function(wrap_pyfunction!(recall_score, m)?)?;
    m.add_function(wrap_pyfunction!(confusion_matrix, m)?)?;
    m.add_function(wrap_pyfunction!(train_test_split, m)?)?;
    m.add_function(wrap_pyfunction!(ml_registry, m)?)?;
    m.add_function(wrap_pyfunction!(parallel_cv_cls, m)?)?;
    m.add_function(wrap_pyfunction!(ml_checkpoint_clear, m)?)?;
    m.add_function(wrap_pyfunction!(ml_checkpoint_list, m)?)?;
    m.add_function(wrap_pyfunction!(task_new, m)?)?;
    m.add_function(wrap_pyfunction!(task_info, m)?)?;
    m.add_function(wrap_pyfunction!(task_list, m)?)?;
    m.add_function(wrap_pyfunction!(task_delete, m)?)?;
    m.add_function(wrap_pyfunction!(task_dir, m)?)?;
    m.add_function(wrap_pyfunction!(task_cleanup, m)?)?;
    m.add_class::<PyOneHotEncoder>()?;
    m.add_class::<PyOrdinalEncoder>()?;
    m.add_class::<PySimpleImputer>()?;
    m.add_class::<PyPolynomialFeatures>()?;
    m.add_class::<PyKBinsDiscretizer>()?;
    m.add_class::<PyPowerTransformer>()?;
    m.add_class::<PyQuantileTransformer>()?;
    m.add_function(wrap_pyfunction!(balanced_accuracy_score, m)?)?;
    m.add_function(wrap_pyfunction!(matthews_corrcoef, m)?)?;
    m.add_function(wrap_pyfunction!(cohen_kappa_score, m)?)?;
    m.add_function(wrap_pyfunction!(hamming_loss, m)?)?;
    m.add_function(wrap_pyfunction!(zero_one_loss, m)?)?;
    m.add_function(wrap_pyfunction!(jaccard_score, m)?)?;
    m.add_function(wrap_pyfunction!(fbeta_score, m)?)?;
    m.add_function(wrap_pyfunction!(log_loss, m)?)?;
    m.add_function(wrap_pyfunction!(binary_log_loss, m)?)?;
    m.add_function(wrap_pyfunction!(brier_score_loss, m)?)?;
    m.add_function(wrap_pyfunction!(hinge_loss, m)?)?;
    m.add_function(wrap_pyfunction!(roc_curve, m)?)?;
    m.add_function(wrap_pyfunction!(roc_auc_score, m)?)?;
    m.add_function(wrap_pyfunction!(precision_recall_curve, m)?)?;
    m.add_function(wrap_pyfunction!(average_precision_score, m)?)?;
    m.add_function(wrap_pyfunction!(median_absolute_error, m)?)?;
    m.add_function(wrap_pyfunction!(mean_squared_log_error, m)?)?;
    m.add_function(wrap_pyfunction!(root_mean_squared_log_error, m)?)?;
    m.add_function(wrap_pyfunction!(mean_pinball_loss, m)?)?;
    m.add_function(wrap_pyfunction!(mean_absolute_percentage_error, m)?)?;
    m.add_function(wrap_pyfunction!(explained_variance_score, m)?)?;
    m.add_function(wrap_pyfunction!(max_error, m)?)?;
    m.add_function(wrap_pyfunction!(silhouette_score, m)?)?;
    m.add_function(wrap_pyfunction!(davies_bouldin_score, m)?)?;
    m.add_function(wrap_pyfunction!(calinski_harabasz_score, m)?)?;
    m.add_function(wrap_pyfunction!(adjusted_rand_score, m)?)?;
    m.add_function(wrap_pyfunction!(normalized_mutual_info_score, m)?)?;
    m.add_function(wrap_pyfunction!(fowlkes_mallows_score, m)?)?;
    m.add_function(wrap_pyfunction!(homogeneity_score, m)?)?;
    m.add_function(wrap_pyfunction!(completeness_score, m)?)?;
    m.add_function(wrap_pyfunction!(v_measure_score, m)?)?;
    m.add_function(wrap_pyfunction!(registry_register, m)?)?;
    m.add_function(wrap_pyfunction!(registry_get, m)?)?;
    m.add_function(wrap_pyfunction!(registry_list, m)?)?;
    m.add_function(wrap_pyfunction!(registry_delete, m)?)?;
    m.add_function(wrap_pyfunction!(registry_clear, m)?)?;
    m.add_function(wrap_pyfunction!(registry_promote, m)?)?;
    m.add_function(wrap_pyfunction!(registry_by_tag, m)?)?;
    m.add_function(wrap_pyfunction!(registry_dir, m)?)?;
    m.add_function(wrap_pyfunction!(export_powerbi, m)?)?;
    m.add_function(wrap_pyfunction!(export_tableau_tds, m)?)?;
    m.add_function(wrap_pyfunction!(export_tableau_csv, m)?)?;
    m.add_function(wrap_pyfunction!(gpu_devices, m)?)?;
    m.add_function(wrap_pyfunction!(gpu_set_backend, m)?)?;
    m.add_function(wrap_pyfunction!(gpu_active_backend, m)?)?;
    m.add_function(wrap_pyfunction!(cloud_plan, m)?)?;
    m.add_function(wrap_pyfunction!(cloud_resources, m)?)?;
    m.add_function(wrap_pyfunction!(cloud_count_rows, m)?)?;
    m.add_class::<PyWorkerPool>()?;
    Ok(())
}

pub fn register_full_ml(m: &PyModule) -> PyResult<()> {
    register_ml_classes(m)
}

} // end mod python_bindings

#[cfg(feature = "python")]
pub use python_bindings::*;

