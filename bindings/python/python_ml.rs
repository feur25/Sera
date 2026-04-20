#![cfg(feature = "python")]
use pyo3::prelude::*;
use numpy::{PyReadonlyArray1, PyReadonlyArray2, PyUntypedArrayMethods, PyArrayMethods, IntoPyArray};
use pyo3::types::PyAny;
use rayon::prelude::*;

fn flat_to_np2d(py: Python<'_>, flat: Vec<f64>, n: usize, cols: usize) -> PyResult<PyObject> {
    if n == 0 || cols == 0 { return Ok(numpy::PyArray2::<f64>::zeros_bound(py, [0, 0], false).into_py(py)); }
    let arr = flat.into_pyarray_bound(py);
    Ok(arr.reshape([n, cols])?.into_py(py))
}

fn vv_to_np2d(py: Python<'_>, data: Vec<Vec<f64>>) -> PyResult<PyObject> {
    let n = data.len();
    if n == 0 { return Ok(numpy::PyArray2::<f64>::zeros_bound(py, [0, 0], false).into_py(py)); }
    let cols = data[0].len();
    let flat: Vec<f64> = data.into_iter().flat_map(|r| r).collect();
    flat_to_np2d(py, flat, n, cols)
}

fn extract_flat(x: &PyAny) -> PyResult<(Vec<f64>, usize, usize)> {
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
                let (xf, n, p) = extract_flat(x)?;
                self.inner.fit(&xf, n, p, &extract_targets(y)?);
                Ok(())
            }
            fn predict(&self, x: &PyAny) -> PyResult<Vec<f64>> {
                let (xf, n, p) = extract_flat(x)?;
                Ok(self.inner.predict(&xf, n, p))
            }
            fn score(&self, x: &PyAny, y: &PyAny) -> PyResult<f64> {
                let (xf, n, p) = extract_flat(x)?;
                let yt = extract_targets(y)?;
                let preds = self.inner.predict(&xf, n, p);
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
                let (xf, n, p) = extract_flat(x)?;
                self.inner.fit(&xf, n, p, &extract_labels(y)?);
                Ok(())
            }
            fn predict(&self, x: &PyAny) -> PyResult<Vec<i32>> {
                let (xf, n, p) = extract_flat(x)?;
                Ok(self.inner.predict(&xf, n, p))
            }
            fn score(&self, x: &PyAny, y: &PyAny) -> PyResult<f64> {
                let (xf, n, p) = extract_flat(x)?;
                let yl = extract_labels(y)?;
                let preds = self.inner.predict(&xf, n, p);
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
        let (xf, n, p) = extract_flat(x)?;
        self.inner.fit_resumable(&xf, n, p, &extract_targets(y)?, checkpoint_id);
        Ok(())
    }
    fn predict(&self, x: &PyAny) -> PyResult<Vec<f64>> {
        let (xf, n, p) = extract_flat(x)?;
        Ok(self.inner.predict(&xf, n, p))
    }
    fn score(&self, x: &PyAny, y: &PyAny) -> PyResult<f64> {
        let (xf, n, p) = extract_flat(x)?;
        let yt = extract_targets(y)?;
        let preds = self.inner.predict(&xf, n, p);
        Ok(crate::ml::linear::ols::r2_score_pub(&yt, &preds))
    }
    fn get_params(&self, _py: Python<'_>) -> PyResult<std::collections::HashMap<String, PyObject>> {
        let mut m = std::collections::HashMap::new();
        Python::with_gil(|py| {
            m.insert("alpha".to_string(), self.inner.alpha().into_py(py));
        });
        Ok(m)
    }
    fn set_params(&mut self, _params: std::collections::HashMap<String, PyObject>) -> PyResult<()> { Ok(()) }
    #[getter] fn coef_(&self) -> Vec<f64> { self.inner.coef().to_vec() }
    #[getter] fn intercept_(&self) -> f64 { self.inner.intercept() }
    #[getter] fn alpha_(&self) -> f64 { self.inner.alpha() }
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
        let (xf, n, p) = extract_flat(x)?;
        self.inner.fit_resumable(&xf, n, p, &extract_targets(y)?, checkpoint_id);
        Ok(())
    }
    fn predict(&self, x: &PyAny) -> PyResult<Vec<f64>> {
        let (xf, n, p) = extract_flat(x)?;
        Ok(self.inner.predict(&xf, n, p))
    }
    fn score(&self, x: &PyAny, y: &PyAny) -> PyResult<f64> {
        let (xf, n, p) = extract_flat(x)?;
        let yt = extract_targets(y)?;
        let preds = self.inner.predict(&xf, n, p);
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
    #[new]
    #[pyo3(signature = (c=1.0, max_iter=1000, tol=1e-4, fit_intercept=true))]
    fn py_new(c: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> Self {
        Self { inner: crate::ml::linear::logistic::LogisticRegression::new(c, max_iter, tol, fit_intercept) }
    }
    #[pyo3(signature = (x, y, checkpoint_id=None))]
    fn fit(&mut self, x: &PyAny, y: &PyAny, checkpoint_id: Option<u64>) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        self.inner.fit_resumable(&xf, n, p, &extract_labels(y)?, checkpoint_id);
        Ok(())
    }
    fn predict(&self, x: &PyAny) -> PyResult<Vec<i32>> {
        let (xf, n, p) = extract_flat(x)?;
        Ok(self.inner.predict(&xf, n, p))
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
        self.inner.fit_resumable(&xf, n, p, &extract_labels(y)?, checkpoint_id);
        Ok(())
    }
    fn predict(&self, x: &PyAny) -> PyResult<Vec<i32>> {
        let (xf, n, p) = extract_flat(x)?;
        Ok(self.inner.predict(&xf, n, p))
    }
    fn score(&self, x: &PyAny, y: &PyAny) -> PyResult<f64> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        let preds = self.inner.predict(&xf, n, p);
        Ok(crate::ml::metrics::classification::accuracy_score(&yl, &preds))
    }
    fn decision_function(&self, x: &PyAny) -> PyResult<Vec<f64>> {
        let (xf, n, p) = extract_flat(x)?;
        Ok(self.inner.decision_function(&xf, n, p))
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
        let (xf, n, p) = extract_flat(x)?;
        self.inner.fit_resumable(&xf, n, p, &extract_targets(y)?, checkpoint_id);
        Ok(())
    }
    fn predict(&self, x: &PyAny) -> PyResult<Vec<f64>> {
        let (xf, n, p) = extract_flat(x)?;
        Ok(self.inner.predict(&xf, n, p))
    }
    fn score(&self, x: &PyAny, y: &PyAny) -> PyResult<f64> {
        let (xf, n, p) = extract_flat(x)?;
        let yt = extract_targets(y)?;
        let preds = self.inner.predict(&xf, n, p);
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
impl_get_set_params!(PyKNeighborsClassifier, "n_neighbors" => inner.k, usize;);

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
impl_get_set_params!(PyKNeighborsRegressor, "n_neighbors" => inner.k, usize;);

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
    #[new]
    #[pyo3(signature = (c=1.0, max_iter=1000, tol=1e-4, fit_intercept=true))]
    fn py_new(c: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> Self {
        Self { inner: crate::ml::svm::svm::LinearSVC::with_fit_intercept(c, max_iter, tol, fit_intercept) }
    }
    fn decision_function(&self, x: &PyAny) -> PyResult<Vec<f64>> {
        let (xf, n, p) = extract_flat(x)?;
        Ok(self.inner.decision_function(&xf, n, p))
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
    #[new]
    #[pyo3(signature = (c=1.0, epsilon=0.1, max_iter=1000, tol=1e-4, fit_intercept=true))]
    fn py_new(c: f64, epsilon: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> Self {
        Self { inner: crate::ml::svm::svm::LinearSVR::with_fit_intercept(c, epsilon, max_iter, tol, fit_intercept) }
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
    #[getter] fn coef_(&self) -> Vec<f64> { self.inner.ridge.coef.clone() }
    #[getter] fn intercept_(&self) -> f64 { self.inner.ridge.intercept }
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
        let (xf, n, p) = extract_flat(x)?;
        self.n = n; self.p = p;
        self.inner.fit(&xf, n, p);
        Ok(())
    }
    fn transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        flat_to_np2d(py, self.inner.transform(&xf, n, p), n, p)
    }
    fn fit_transform(&mut self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        self.n = n; self.p = p;
        flat_to_np2d(py, self.inner.fit_transform(&xf, n, p), n, p)
    }
    fn inverse_transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        flat_to_np2d(py, self.inner.inverse_transform(&xf, n, p), n, p)
    }
    #[getter] fn mean_(&self) -> Vec<f64> { self.inner.mean.clone() }
    #[getter] fn scale_(&self) -> Vec<f64> { self.inner.scale.clone() }
    #[getter] fn with_mean_(&self) -> bool { self.inner.with_mean }
    #[getter] fn with_std_(&self) -> bool { self.inner.with_std }
    #[getter] fn var_(&self) -> Vec<f64> { self.inner.scale.iter().map(|s| s * s).collect() }
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
        self.inner.fit(&xf, n, p);
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
        self.inner.fit(&xf, n, p);
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
        self.inner.fit(&xf, n, p);
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
        let (xf, n, p) = extract_flat(x)?;
        self.inner.fit(&xf, n, p);
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
    fn inverse_transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, _p) = extract_flat(x)?;
        let flat = self.inner.inverse_transform(&xf, n);
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
        self.inner.fit(&xf, n, p);
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
pub fn accuracy_score(y_true: Vec<i32>, y_pred: Vec<i32>) -> f64 {
    crate::ml::metrics::classification::accuracy_score(&y_true, &y_pred)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn mean_squared_error(y_true: Vec<f64>, y_pred: Vec<f64>) -> f64 {
    crate::ml::metrics::regression::mean_squared_error(&y_true, &y_pred)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn mean_absolute_error(y_true: Vec<f64>, y_pred: Vec<f64>) -> f64 {
    crate::ml::metrics::regression::mean_absolute_error(&y_true, &y_pred)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn r2_score(y_true: Vec<f64>, y_pred: Vec<f64>) -> f64 {
    crate::ml::metrics::regression::r2_score(&y_true, &y_pred)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn classification_report(y_true: Vec<i32>, y_pred: Vec<i32>) -> String {
    crate::ml::metrics::classification::classification_report(&y_true, &y_pred)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred, average="weighted"))]
pub fn f1_score(y_true: Vec<i32>, y_pred: Vec<i32>, average: &str) -> f64 {
    let avg = match average {
        "macro" => crate::ml::metrics::classification::Average::Macro,
        "weighted" => crate::ml::metrics::classification::Average::Weighted,
        _ => {
            let classes = { let mut c: Vec<i32> = y_true.iter().chain(y_pred.iter()).copied().collect(); c.sort_unstable(); c.dedup(); c };
            crate::ml::metrics::classification::Average::Binary(*classes.last().unwrap_or(&1))
        }
    };
    crate::ml::metrics::classification::f1_score(&y_true, &y_pred, avg)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred, average="weighted"))]
pub fn precision_score(y_true: Vec<i32>, y_pred: Vec<i32>, average: &str) -> f64 {
    let avg = match average {
        "macro" => crate::ml::metrics::classification::Average::Macro,
        "weighted" => crate::ml::metrics::classification::Average::Weighted,
        _ => {
            let classes = { let mut c: Vec<i32> = y_true.iter().chain(y_pred.iter()).copied().collect(); c.sort_unstable(); c.dedup(); c };
            crate::ml::metrics::classification::Average::Binary(*classes.last().unwrap_or(&1))
        }
    };
    crate::ml::metrics::classification::precision_score(&y_true, &y_pred, avg)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred, average="weighted"))]
pub fn recall_score(y_true: Vec<i32>, y_pred: Vec<i32>, average: &str) -> f64 {
    let avg = match average {
        "macro" => crate::ml::metrics::classification::Average::Macro,
        "weighted" => crate::ml::metrics::classification::Average::Weighted,
        _ => {
            let classes = { let mut c: Vec<i32> = y_true.iter().chain(y_pred.iter()).copied().collect(); c.sort_unstable(); c.dedup(); c };
            crate::ml::metrics::classification::Average::Binary(*classes.last().unwrap_or(&1))
        }
    };
    crate::ml::metrics::classification::recall_score(&y_true, &y_pred, avg)
}

#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn confusion_matrix(y_true: Vec<i32>, y_pred: Vec<i32>) -> (Vec<i32>, Vec<usize>) {
    crate::ml::metrics::classification::confusion_matrix(&y_true, &y_pred)
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
#[pyo3(signature = (x, y, folds, c=1.0, max_iter=1000, tol=1e-4, fit_intercept=true))]
pub fn parallel_cv_cls(x: &PyAny, y: &PyAny, folds: Vec<(Vec<usize>, Vec<usize>)>, c: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> PyResult<Vec<f64>> {
    use rayon::prelude::*;
    let (xf, _n, p) = extract_flat(x)?;
    let yl = extract_labels(y)?;
    let scores: Vec<f64> = folds.into_par_iter().map(|(train_idx, test_idx)| {
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
    }).collect();
    Ok(scores)
}

#[pyfunction]
pub fn ml_checkpoint_clear(id: u64) { crate::ml::cache::checkpoint_clear(id); }

#[pyfunction]
pub fn ml_checkpoint_list() -> Vec<u64> { crate::ml::cache::checkpoint_list() }

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
    best_score: f64,
    best_params: std::collections::HashMap<String, String>,
    cv_results_mean: Vec<f64>,
    cv_results_std: Vec<f64>,
    fitted: bool,
}

#[pymethods]
impl PyGridSearchCV {
    #[new]
    #[pyo3(signature = (estimator, param_grid, cv=5, seed=42))]
    fn py_new(estimator: &str, param_grid: &pyo3::types::PyDict, cv: usize, seed: u64) -> PyResult<Self> {
        let (param_names, param_values) = parse_param_grid(param_grid)?;
        Ok(Self {
            estimator: estimator.to_string(),
            param_names,
            param_values,
            cv, seed,
            best_score: f64::NEG_INFINITY,
            best_params: std::collections::HashMap::new(),
            cv_results_mean: Vec::new(),
            cv_results_std: Vec::new(),
            fitted: false,
        })
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        use crate::ml::model_selection::grid_search::*;
        let (xf, n, p) = extract_flat(x)?;
        let param_sizes: Vec<usize> = self.param_values.iter().map(|v| v.len()).collect();
        let total = n_combinations(&param_sizes);
        let est = self.estimator.clone();
        let pn = self.param_names.clone();
        let pv = self.param_values.clone();
        let ps = param_sizes.clone();

        let folds = if is_classifier(&est) {
            let yl = extract_labels(y)?;
            precompute_folds_cls(&xf, n, p, &yl, self.cv, self.seed)
        } else {
            let yt = extract_targets(y)?;
            precompute_folds_reg(&xf, n, p, &yt, self.cv, self.seed)
        };

        let caches = compute_caches(&est, &folds, &pn, &pv);
        let result = grid_search_parallel_cached(total, &folds, &caches, |combo_idx, fold, cache| {
            eval_model_cached(&est, &pn, &pv, &ps, combo_idx, fold, cache)
        });

        self.best_score = result.best_score;
        self.best_params = build_best_params(&self.param_names, &self.param_values, &param_sizes, result.best_params_idx);
        self.cv_results_mean = result.cv_results;
        self.cv_results_std = result.cv_std;
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
    best_score: f64,
    best_params: std::collections::HashMap<String, String>,
    cv_results_mean: Vec<f64>,
    cv_results_std: Vec<f64>,
    fitted: bool,
}

#[pymethods]
impl PyRandomizedSearchCV {
    #[new]
    #[pyo3(signature = (estimator, param_distributions, n_iter=10, cv=5, seed=42))]
    fn py_new(estimator: &str, param_distributions: &pyo3::types::PyDict, n_iter: usize, cv: usize, seed: u64) -> PyResult<Self> {
        let (param_names, param_values) = parse_param_grid(param_distributions)?;
        Ok(Self {
            estimator: estimator.to_string(),
            param_names, param_values, n_iter, cv, seed,
            best_score: f64::NEG_INFINITY,
            best_params: std::collections::HashMap::new(),
            cv_results_mean: Vec::new(),
            cv_results_std: Vec::new(),
            fitted: false,
        })
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        use crate::ml::model_selection::grid_search::*;
        let (xf, n, p) = extract_flat(x)?;
        let param_sizes: Vec<usize> = self.param_values.iter().map(|v| v.len()).collect();
        let total = n_combinations(&param_sizes);
        let combo_indices = sample_indices(total, self.n_iter, self.seed);
        let est = self.estimator.clone();
        let pn = self.param_names.clone();
        let pv = self.param_values.clone();
        let ps = param_sizes.clone();

        let folds = if is_classifier(&est) {
            let yl = extract_labels(y)?;
            precompute_folds_cls(&xf, n, p, &yl, self.cv, self.seed)
        } else {
            let yt = extract_targets(y)?;
            precompute_folds_reg(&xf, n, p, &yt, self.cv, self.seed)
        };

        let caches = compute_caches(&est, &folds, &pn, &pv);
        let result = randomized_search_parallel_cached(&combo_indices, &folds, &caches, |combo_idx, fold, cache| {
            eval_model_cached(&est, &pn, &pv, &ps, combo_idx, fold, cache)
        });

        self.best_score = result.best_score;
        self.best_params = build_best_params(&self.param_names, &self.param_values, &param_sizes, result.best_params_idx);
        self.cv_results_mean = result.cv_results;
        self.cv_results_std = result.cv_std;
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
    best_score: f64,
    best_params: std::collections::HashMap<String, String>,
    n_iterations: usize,
    fitted: bool,
}

#[pymethods]
impl PyHalvingGridSearchCV {
    #[new]
    #[pyo3(signature = (estimator, param_grid, cv=5, factor=3, seed=42))]
    fn py_new(estimator: &str, param_grid: &pyo3::types::PyDict, cv: usize, factor: usize, seed: u64) -> PyResult<Self> {
        let (param_names, param_values) = parse_param_grid(param_grid)?;
        Ok(Self {
            estimator: estimator.to_string(),
            param_names, param_values, cv, factor: factor.max(2), seed,
            best_score: f64::NEG_INFINITY,
            best_params: std::collections::HashMap::new(),
            n_iterations: 0,
            fitted: false,
        })
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        use crate::ml::model_selection::grid_search::*;
        let (xf, n, p) = extract_flat(x)?;
        let param_sizes: Vec<usize> = self.param_values.iter().map(|v| v.len()).collect();
        let total = n_combinations(&param_sizes);
        let est = self.estimator.clone();
        let pn = self.param_names.clone();
        let pv = self.param_values.clone();
        let ps = param_sizes.clone();
        let fac = self.factor;

        let n_iters = (total as f64).log(fac as f64).ceil() as usize;
        let min_resources = (n as f64 / fac.pow(n_iters as u32) as f64).max(1.0) as usize;

        let full_folds = if is_classifier(&est) {
            let yl = extract_labels(y)?;
            precompute_folds_cls(&xf, n, p, &yl, self.cv, self.seed)
        } else {
            let yt = extract_targets(y)?;
            precompute_folds_reg(&xf, n, p, &yt, self.cv, self.seed)
        };

        let mut candidates: Vec<usize> = (0..total).collect();
        let mut resource = min_resources;
        let mut iteration = 0;

        while candidates.len() > 1 && resource <= n {
            let sub_folds: Vec<FoldData> = full_folds.iter()
                .map(|f| subsample_fold(f, resource))
                .collect();
            let caches = compute_caches(&est, &sub_folds, &pn, &pv);

            let scores: Vec<(usize, f64)> = candidates.par_iter().map(|&combo_idx| {
                let mean: f64 = sub_folds.iter().zip(caches.iter())
                    .map(|(fold, cache)| eval_model_cached(&est, &pn, &pv, &ps, combo_idx, fold, cache))
                    .sum::<f64>() / sub_folds.len() as f64;
                (combo_idx, mean)
            }).collect();

            let mut sorted = scores;
            sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            let keep = (candidates.len() / fac).max(1);
            candidates = sorted[..keep].iter().map(|&(idx, _)| idx).collect();

            resource = (resource * fac).min(n);
            iteration += 1;
        }

        let best_combo = candidates[0];
        let final_caches = compute_caches(&est, &full_folds, &pn, &pv);
        let best_score: f64 = full_folds.iter().zip(final_caches.iter())
            .map(|(fold, cache)| eval_model_cached(&est, &pn, &pv, &ps, best_combo, fold, cache))
            .sum::<f64>() / full_folds.len() as f64;

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
    best_score: f64,
    best_params: std::collections::HashMap<String, String>,
    n_iterations: usize,
    fitted: bool,
}

#[pymethods]
impl PyHalvingRandomSearchCV {
    #[new]
    #[pyo3(signature = (estimator, param_distributions, n_candidates=256, cv=5, factor=3, seed=42))]
    fn py_new(estimator: &str, param_distributions: &pyo3::types::PyDict, n_candidates: usize, cv: usize, factor: usize, seed: u64) -> PyResult<Self> {
        let (param_names, param_values) = parse_param_grid(param_distributions)?;
        Ok(Self {
            estimator: estimator.to_string(),
            param_names, param_values, n_candidates, cv, factor: factor.max(2), seed,
            best_score: f64::NEG_INFINITY,
            best_params: std::collections::HashMap::new(),
            n_iterations: 0,
            fitted: false,
        })
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        use crate::ml::model_selection::grid_search::*;
        let (xf, n, p) = extract_flat(x)?;
        let param_sizes: Vec<usize> = self.param_values.iter().map(|v| v.len()).collect();
        let total = n_combinations(&param_sizes);
        let est = self.estimator.clone();
        let pn = self.param_names.clone();
        let pv = self.param_values.clone();
        let ps = param_sizes.clone();
        let fac = self.factor;

        let full_folds = if is_classifier(&est) {
            let yl = extract_labels(y)?;
            precompute_folds_cls(&xf, n, p, &yl, self.cv, self.seed)
        } else {
            let yt = extract_targets(y)?;
            precompute_folds_reg(&xf, n, p, &yt, self.cv, self.seed)
        };

        let mut candidates = sample_indices(total, self.n_candidates, self.seed);
        let n_iters = (candidates.len() as f64).log(fac as f64).ceil() as usize;
        let min_resources = (n as f64 / fac.pow(n_iters as u32) as f64).max(1.0) as usize;
        let mut resource = min_resources;
        let mut iteration = 0;

        while candidates.len() > 1 && resource <= n {
            let sub_folds: Vec<FoldData> = full_folds.iter()
                .map(|f| subsample_fold(f, resource))
                .collect();
            let caches = compute_caches(&est, &sub_folds, &pn, &pv);

            let scores: Vec<(usize, f64)> = candidates.par_iter().map(|&combo_idx| {
                let mean: f64 = sub_folds.iter().zip(caches.iter())
                    .map(|(fold, cache)| eval_model_cached(&est, &pn, &pv, &ps, combo_idx, fold, cache))
                    .sum::<f64>() / sub_folds.len() as f64;
                (combo_idx, mean)
            }).collect();

            let mut sorted = scores;
            sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            let keep = (candidates.len() / fac).max(1);
            candidates = sorted[..keep].iter().map(|&(idx, _)| idx).collect();

            resource = (resource * fac).min(n);
            iteration += 1;
        }

        let best_combo = candidates[0];
        let final_caches = compute_caches(&est, &full_folds, &pn, &pv);
        let best_score: f64 = full_folds.iter().zip(final_caches.iter())
            .map(|(fold, cache)| eval_model_cached(&est, &pn, &pv, &ps, best_combo, fold, cache))
            .sum::<f64>() / full_folds.len() as f64;

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
    Ok(())
}
