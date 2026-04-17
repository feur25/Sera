#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use numpy::{PyReadonlyArray1, PyReadonlyArray2, PyUntypedArrayMethods, PyArrayMethods, IntoPyArray};

#[cfg(feature = "python")]
use pyo3::types::PyAny;

#[cfg(feature = "python")]
fn flat_to_np2d(py: Python<'_>, flat: Vec<f64>, n: usize, cols: usize) -> PyResult<PyObject> {
    if n == 0 || cols == 0 { return Ok(numpy::PyArray2::<f64>::zeros_bound(py, [0, 0], false).into_py(py)); }
    let arr = flat.into_pyarray_bound(py);
    Ok(arr.reshape([n, cols])?.into_py(py))
}

#[cfg(feature = "python")]
fn vv_to_np2d(py: Python<'_>, data: Vec<Vec<f64>>) -> PyResult<PyObject> {
    let n = data.len();
    if n == 0 { return Ok(numpy::PyArray2::<f64>::zeros_bound(py, [0, 0], false).into_py(py)); }
    let cols = data[0].len();
    let flat: Vec<f64> = data.into_iter().flat_map(|r| r).collect();
    flat_to_np2d(py, flat, n, cols)
}

#[cfg(feature = "python")]
fn extract_flat(x: &PyAny) -> PyResult<(Vec<f64>, usize, usize)> {
    if let Ok(arr) = x.extract::<PyReadonlyArray2<f64>>() {
        let shape = arr.shape();
        let (n, p) = (shape[0], shape[1]);
        let view = arr.as_array();
        if view.is_standard_layout() {
            return Ok((view.as_slice().unwrap().to_vec(), n, p));
        }
        let c = view.as_standard_layout();
        return Ok((c.as_slice().unwrap().to_vec(), n, p));
    }
    if let Ok(arr) = x.extract::<PyReadonlyArray2<f32>>() {
        let shape = arr.shape();
        let (n, p) = (shape[0], shape[1]);
        let view = arr.as_array();
        if view.is_standard_layout() {
            return Ok((view.as_slice().unwrap().iter().map(|&v| v as f64).collect(), n, p));
        }
        let c = view.as_standard_layout();
        return Ok((c.as_slice().unwrap().iter().map(|&v| v as f64).collect(), n, p));
    }
    let rows: Vec<Vec<f64>> = x.extract()?;
    let n = rows.len();
    if n == 0 { return Ok((Vec::new(), 0, 0)); }
    let p = rows[0].len();
    let mut flat = vec![0.0f64; n * p];
    for (i, row) in rows.iter().enumerate() {
        flat[i * p..(i + 1) * p].copy_from_slice(row);
    }
    Ok((flat, n, p))
}

#[cfg(feature = "python")]
fn extract_labels(y: &PyAny) -> PyResult<Vec<i32>> {
    if let Ok(arr) = y.extract::<PyReadonlyArray1<i64>>() {
        return Ok(arr.as_slice().unwrap().iter().map(|&v| v as i32).collect());
    }
    if let Ok(arr) = y.extract::<PyReadonlyArray1<i32>>() {
        return Ok(arr.as_slice().unwrap().to_vec());
    }
    if let Ok(arr) = y.extract::<PyReadonlyArray1<f64>>() {
        return Ok(arr.as_slice().unwrap().iter().map(|&v| v as i32).collect());
    }
    y.extract::<Vec<i32>>()
}

#[cfg(feature = "python")]
fn extract_targets(y: &PyAny) -> PyResult<Vec<f64>> {
    if let Ok(arr) = y.extract::<PyReadonlyArray1<f64>>() {
        return Ok(arr.as_slice().unwrap().to_vec());
    }
    if let Ok(arr) = y.extract::<PyReadonlyArray1<f32>>() {
        return Ok(arr.as_slice().unwrap().iter().map(|&v| v as f64).collect());
    }
    if let Ok(arr) = y.extract::<PyReadonlyArray1<i64>>() {
        return Ok(arr.as_slice().unwrap().iter().map(|&v| v as f64).collect());
    }
    y.extract::<Vec<f64>>()
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "LinearRegression")]
pub struct PyLinearRegression {
    inner: crate::ml::linear::ols::LinearRegression,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyLinearRegression {
    #[new]
    #[pyo3(signature = (fit_intercept=true))]
    fn py_new(fit_intercept: bool) -> Self {
        Self { inner: crate::ml::linear::ols::LinearRegression::new(fit_intercept) }
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yt = extract_targets(y)?;
        self.inner.fit(&xf, n, p, &yt);
        Ok(())
    }

    fn predict(&self, x: &PyAny) -> PyResult<Vec<f64>> {
        let (xf, n, p) = extract_flat(x)?;
        Ok(self.inner.predict(&xf, n, p))
    }

    fn score(&self, x: &PyAny, y: &PyAny) -> PyResult<f64> {
        let (xf, n, p) = extract_flat(x)?;
        let yt = extract_targets(y)?;
        Ok(self.inner.score(&xf, n, p, &yt))
    }

    #[getter] fn coef_(&self) -> Vec<f64> { self.inner.coef.clone() }
    #[getter] fn intercept_(&self) -> f64 { self.inner.intercept }
    #[getter] fn fit_intercept_(&self) -> bool { self.inner.fit_intercept }
    #[getter] fn n_features_in_(&self) -> usize { self.inner.coef.len() }

    fn __repr__(&self) -> String { "LinearRegression()".to_string() }
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "Ridge")]
pub struct PyRidge {
    inner: crate::ml::linear::ridge::Ridge,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyRidge {
    #[new]
    #[pyo3(signature = (alpha=1.0, fit_intercept=true))]
    fn py_new(alpha: f64, fit_intercept: bool) -> Self {
        Self { inner: crate::ml::linear::ridge::Ridge::new(alpha, fit_intercept) }
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yt = extract_targets(y)?;
        self.inner.fit(&xf, n, p, &yt);
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
    #[getter] fn alpha_(&self) -> f64 { self.inner.alpha }
    #[getter] fn fit_intercept_(&self) -> bool { self.inner.fit_intercept }

    fn __repr__(&self) -> String { format!("Ridge(alpha={})", self.inner.alpha) }
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "Lasso")]
pub struct PyLasso {
    inner: crate::ml::linear::lasso::Lasso,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyLasso {
    #[new]
    #[pyo3(signature = (alpha=1.0, max_iter=1000, tol=1e-4, fit_intercept=true))]
    fn py_new(alpha: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> Self {
        Self { inner: crate::ml::linear::lasso::Lasso::new(alpha, max_iter, tol, fit_intercept) }
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yt = extract_targets(y)?;
        self.inner.fit(&xf, n, p, &yt);
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

    #[getter] fn coef_(&self) -> Vec<f64> { self.inner.coef().to_vec() }
    #[getter] fn intercept_(&self) -> f64 { self.inner.intercept() }
    #[getter] fn alpha_(&self) -> f64 { self.inner.alpha() }
    #[getter] fn n_iter_(&self) -> usize { self.inner.n_iter() }

    fn __repr__(&self) -> String { format!("Lasso(alpha={})", self.inner.alpha()) }
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "ElasticNet")]
pub struct PyElasticNet {
    inner: crate::ml::linear::elastic_net::ElasticNet,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyElasticNet {
    #[new]
    #[pyo3(signature = (alpha=1.0, l1_ratio=0.5, max_iter=1000, tol=1e-4, fit_intercept=true))]
    fn py_new(alpha: f64, l1_ratio: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> Self {
        Self { inner: crate::ml::linear::elastic_net::ElasticNet::new(alpha, l1_ratio, max_iter, tol, fit_intercept) }
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yt = extract_targets(y)?;
        self.inner.fit(&xf, n, p, &yt);
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

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "LogisticRegression")]
pub struct PyLogisticRegression {
    inner: crate::ml::linear::logistic::LogisticRegression,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyLogisticRegression {
    #[new]
    #[pyo3(signature = (c=1.0, max_iter=1000, tol=1e-4, fit_intercept=true))]
    fn py_new(c: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> Self {
        Self { inner: crate::ml::linear::logistic::LogisticRegression::new(c, max_iter, tol, fit_intercept) }
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        self.inner.fit(&xf, n, p, &yl);
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
        if self.inner.multi_coef.is_empty() {
            Ok(self.inner.coef.clone().into_pyarray_bound(py).into_py(py))
        } else {
            vv_to_np2d(py, self.inner.multi_coef.clone())
        }
    }
    #[getter] fn intercept_(&self, py: Python<'_>) -> PyResult<PyObject> {
        if self.inner.multi_intercept.is_empty() {
            Ok(self.inner.intercept.into_py(py))
        } else {
            Ok(self.inner.multi_intercept.clone().into_pyarray_bound(py).into_py(py))
        }
    }
    #[getter] fn n_iter_(&self) -> usize { self.inner.n_iter }
    #[getter] fn C_(&self) -> f64 { self.inner.c }
    #[getter] fn max_iter_(&self) -> usize { self.inner.max_iter }
    #[getter] fn tol_(&self) -> f64 { self.inner.tol }
    #[getter] fn fit_intercept_(&self) -> bool { self.inner.fit_intercept }

    fn __repr__(&self) -> String { format!("LogisticRegression(C={})", self.inner.c) }
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "SGDClassifier")]
pub struct PySGDClassifier {
    inner: crate::ml::linear::sgd::SGDClassifier,
}

#[cfg(feature = "python")]
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

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        self.inner.fit(&xf, n, p, &yl);
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

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "SGDRegressor")]
pub struct PySGDRegressor {
    inner: crate::ml::linear::sgd::SGDRegressor,
}

#[cfg(feature = "python")]
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

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yt = extract_targets(y)?;
        self.inner.fit(&xf, n, p, &yt);
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

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "DecisionTreeClassifier")]
pub struct PyDecisionTreeClassifier {
    inner: crate::ml::tree::decision_tree::DecisionTreeClassifier,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyDecisionTreeClassifier {
    #[new]
    #[pyo3(signature = (max_depth=10, min_samples_split=2, min_samples_leaf=1, max_features=None, criterion="gini"))]
    fn py_new(max_depth: usize, min_samples_split: usize, min_samples_leaf: usize, max_features: Option<usize>, criterion: &str) -> Self {
        let c = if criterion == "entropy" {
            crate::ml::tree::decision_tree::TreeCriterion::Entropy
        } else {
            crate::ml::tree::decision_tree::TreeCriterion::Gini
        };
        Self { inner: crate::ml::tree::decision_tree::DecisionTreeClassifier::new(max_depth, min_samples_split, min_samples_leaf, max_features, c) }
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        self.inner.fit(&xf, n, p, &yl);
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

    #[getter] fn feature_importances_(&self) -> Vec<f64> { self.inner.feature_importances.clone() }
    #[getter] fn classes_(&self) -> Vec<i32> { self.inner.classes.clone() }
    #[getter] fn max_depth_(&self) -> usize { self.inner.max_depth }
    #[getter] fn min_samples_split_(&self) -> usize { self.inner.min_samples_split }
    #[getter] fn min_samples_leaf_(&self) -> usize { self.inner.min_samples_leaf }
    #[getter] fn max_features_(&self) -> Option<usize> { self.inner.max_features }
    #[getter] fn criterion_(&self) -> &str {
        match self.inner.criterion {
            crate::ml::tree::decision_tree::TreeCriterion::Gini => "gini",
            crate::ml::tree::decision_tree::TreeCriterion::Entropy => "entropy",
            crate::ml::tree::decision_tree::TreeCriterion::MSE => "gini",
        }
    }

    fn __repr__(&self) -> String { format!("DecisionTreeClassifier(max_depth={})", self.inner.max_depth) }
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "DecisionTreeRegressor")]
pub struct PyDecisionTreeRegressor {
    inner: crate::ml::tree::decision_tree::DecisionTreeRegressor,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyDecisionTreeRegressor {
    #[new]
    #[pyo3(signature = (max_depth=10, min_samples_split=2, min_samples_leaf=1, max_features=None))]
    fn py_new(max_depth: usize, min_samples_split: usize, min_samples_leaf: usize, max_features: Option<usize>) -> Self {
        Self { inner: crate::ml::tree::decision_tree::DecisionTreeRegressor::new(max_depth, min_samples_split, min_samples_leaf, max_features) }
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yt = extract_targets(y)?;
        self.inner.fit(&xf, n, p, &yt);
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

    #[getter] fn feature_importances_(&self) -> Vec<f64> { self.inner.feature_importances.clone() }
    #[getter] fn max_depth_(&self) -> usize { self.inner.max_depth }
    #[getter] fn min_samples_split_(&self) -> usize { self.inner.min_samples_split }
    #[getter] fn min_samples_leaf_(&self) -> usize { self.inner.min_samples_leaf }
    #[getter] fn max_features_(&self) -> Option<usize> { self.inner.max_features }

    fn __repr__(&self) -> String { format!("DecisionTreeRegressor(max_depth={})", self.inner.max_depth) }
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "RandomForestClassifier")]
pub struct PyRandomForestClassifier {
    inner: crate::ml::tree::random_forest::RandomForestClassifier,
}

#[cfg(feature = "python")]
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

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        self.inner.fit(&xf, n, p, &yl);
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

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "RandomForestRegressor")]
pub struct PyRandomForestRegressor {
    inner: crate::ml::tree::random_forest::RandomForestRegressor,
}

#[cfg(feature = "python")]
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

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yt = extract_targets(y)?;
        self.inner.fit(&xf, n, p, &yt);
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

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "GradientBoostingClassifier")]
pub struct PyGradientBoostingClassifier {
    inner: crate::ml::tree::gradient_boosting::GradientBoostingClassifier,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyGradientBoostingClassifier {
    #[new]
    #[pyo3(signature = (n_estimators=100, learning_rate=0.1, max_depth=3, min_samples_split=2, min_samples_leaf=1))]
    fn py_new(n_estimators: usize, learning_rate: f64, max_depth: usize, min_samples_split: usize, min_samples_leaf: usize) -> Self {
        Self { inner: crate::ml::tree::gradient_boosting::GradientBoostingClassifier::new(n_estimators, learning_rate, max_depth, min_samples_split, min_samples_leaf) }
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        self.inner.fit(&xf, n, p, &yl);
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

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "GradientBoostingRegressor")]
pub struct PyGradientBoostingRegressor {
    inner: crate::ml::tree::gradient_boosting::GradientBoostingRegressor,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyGradientBoostingRegressor {
    #[new]
    #[pyo3(signature = (n_estimators=100, learning_rate=0.1, max_depth=3, min_samples_split=2, min_samples_leaf=1))]
    fn py_new(n_estimators: usize, learning_rate: f64, max_depth: usize, min_samples_split: usize, min_samples_leaf: usize) -> Self {
        Self { inner: crate::ml::tree::gradient_boosting::GradientBoostingRegressor::new(n_estimators, learning_rate, max_depth, min_samples_split, min_samples_leaf) }
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yt = extract_targets(y)?;
        self.inner.fit(&xf, n, p, &yt);
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

    #[getter] fn n_estimators_(&self) -> usize { self.inner.n_estimators }
    #[getter] fn learning_rate_(&self) -> f64 { self.inner.learning_rate }
    #[getter] fn max_depth_(&self) -> usize { self.inner.max_depth }
    #[getter] fn min_samples_split_(&self) -> usize { self.inner.min_samples_split }
    #[getter] fn min_samples_leaf_(&self) -> usize { self.inner.min_samples_leaf }

    fn __repr__(&self) -> String { format!("GradientBoostingRegressor(n_estimators={}, lr={})", self.inner.n_estimators, self.inner.learning_rate) }
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "AdaBoostClassifier")]
pub struct PyAdaBoostClassifier {
    inner: crate::ml::tree::adaboost::AdaBoostClassifier,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyAdaBoostClassifier {
    #[new]
    #[pyo3(signature = (n_estimators=50, learning_rate=1.0, max_depth=1))]
    fn py_new(n_estimators: usize, learning_rate: f64, max_depth: usize) -> Self {
        Self { inner: crate::ml::tree::adaboost::AdaBoostClassifier::new(n_estimators, learning_rate, max_depth) }
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        self.inner.fit(&xf, n, p, &yl);
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

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "AdaBoostRegressor")]
pub struct PyAdaBoostRegressor {
    inner: crate::ml::tree::adaboost::AdaBoostRegressor,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyAdaBoostRegressor {
    #[new]
    #[pyo3(signature = (n_estimators=50, learning_rate=1.0, max_depth=3))]
    fn py_new(n_estimators: usize, learning_rate: f64, max_depth: usize) -> Self {
        Self { inner: crate::ml::tree::adaboost::AdaBoostRegressor::new(n_estimators, learning_rate, max_depth) }
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yt = extract_targets(y)?;
        self.inner.fit(&xf, n, p, &yt);
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

    #[getter] fn n_estimators_(&self) -> usize { self.inner.n_estimators }
    #[getter] fn learning_rate_(&self) -> f64 { self.inner.learning_rate }
    #[getter] fn max_depth_(&self) -> usize { self.inner.max_depth }

    fn __repr__(&self) -> String { format!("AdaBoostRegressor(n_estimators={})", self.inner.n_estimators) }
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "KNeighborsClassifier")]
pub struct PyKNeighborsClassifier {
    inner: crate::ml::neighbors::knn::KNeighborsClassifier,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyKNeighborsClassifier {
    #[new]
    #[pyo3(signature = (n_neighbors=5, weights="uniform"))]
    fn py_new(n_neighbors: usize, weights: &str) -> Self {
        let w = match weights { "distance" => crate::ml::neighbors::knn::KnnWeights::Distance, _ => crate::ml::neighbors::knn::KnnWeights::Uniform };
        Self { inner: crate::ml::neighbors::knn::KNeighborsClassifier::new(n_neighbors, w) }
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        self.inner.fit(&xf, n, p, &yl);
        Ok(())
    }

    fn predict(&self, x: &PyAny) -> PyResult<Vec<i32>> {
        let (xf, n, p) = extract_flat(x)?;
        Ok(self.inner.predict(&xf, n, p))
    }

    fn predict_proba(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let flat = self.inner.predict_proba(&xf, n, p);
        let k = self.inner.classes.len();
        flat_to_np2d(py, flat, n, k)
    }

    fn score(&self, x: &PyAny, y: &PyAny) -> PyResult<f64> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        let preds = self.inner.predict(&xf, n, p);
        Ok(crate::ml::metrics::classification::accuracy_score(&yl, &preds))
    }

    #[getter] fn classes_(&self) -> Vec<i32> { self.inner.classes.clone() }
    #[getter] fn n_neighbors_(&self) -> usize { self.inner.k }
    #[getter] fn weights_(&self) -> &str {
        match self.inner.weights {
            crate::ml::neighbors::knn::KnnWeights::Uniform => "uniform",
            crate::ml::neighbors::knn::KnnWeights::Distance => "distance",
        }
    }

    fn __repr__(&self) -> String { format!("KNeighborsClassifier(n_neighbors={})", self.inner.k) }
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "KNeighborsRegressor")]
pub struct PyKNeighborsRegressor {
    inner: crate::ml::neighbors::knn::KNeighborsRegressor,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyKNeighborsRegressor {
    #[new]
    #[pyo3(signature = (n_neighbors=5, weights="uniform"))]
    fn py_new(n_neighbors: usize, weights: &str) -> Self {
        let w = match weights { "distance" => crate::ml::neighbors::knn::KnnWeights::Distance, _ => crate::ml::neighbors::knn::KnnWeights::Uniform };
        Self { inner: crate::ml::neighbors::knn::KNeighborsRegressor::new(n_neighbors, w) }
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yt = extract_targets(y)?;
        self.inner.fit(&xf, n, p, &yt);
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

    #[getter] fn n_neighbors_(&self) -> usize { self.inner.k }
    #[getter] fn weights_(&self) -> &str {
        match self.inner.weights {
            crate::ml::neighbors::knn::KnnWeights::Uniform => "uniform",
            crate::ml::neighbors::knn::KnnWeights::Distance => "distance",
        }
    }

    fn __repr__(&self) -> String { format!("KNeighborsRegressor(n_neighbors={})", self.inner.k) }
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "NearestCentroid")]
pub struct PyNearestCentroid {
    inner: crate::ml::neighbors::knn::NearestCentroid,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyNearestCentroid {
    #[new]
    fn py_new() -> Self {
        Self { inner: crate::ml::neighbors::knn::NearestCentroid::new() }
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        self.inner.fit(&xf, n, p, &yl);
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

    #[getter] fn classes_(&self) -> Vec<i32> { self.inner.classes.clone() }

    fn __repr__(&self) -> String { "NearestCentroid()".to_string() }
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "GaussianNB")]
pub struct PyGaussianNB {
    inner: crate::ml::naive_bayes::gaussian::GaussianNB,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyGaussianNB {
    #[new]
    #[pyo3(signature = (var_smoothing=1e-9))]
    fn py_new(var_smoothing: f64) -> Self {
        Self { inner: crate::ml::naive_bayes::gaussian::GaussianNB::with_var_smoothing(var_smoothing) }
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        self.inner.fit(&xf, n, p, &yl);
        Ok(())
    }

    fn predict(&self, x: &PyAny) -> PyResult<Vec<i32>> {
        let (xf, n, p) = extract_flat(x)?;
        Ok(self.inner.predict(&xf, n, p))
    }

    fn predict_proba(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let flat = self.inner.predict_proba(&xf, n, p);
        let k = self.inner.classes.len();
        flat_to_np2d(py, flat, n, k)
    }

    fn score(&self, x: &PyAny, y: &PyAny) -> PyResult<f64> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        let preds = self.inner.predict(&xf, n, p);
        Ok(crate::ml::metrics::classification::accuracy_score(&yl, &preds))
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

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "MultinomialNB")]
pub struct PyMultinomialNB {
    inner: crate::ml::naive_bayes::multinomial::MultinomialNB,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyMultinomialNB {
    #[new]
    #[pyo3(signature = (alpha=1.0))]
    fn py_new(alpha: f64) -> Self {
        Self { inner: crate::ml::naive_bayes::multinomial::MultinomialNB::new(alpha) }
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        self.inner.fit(&xf, n, p, &yl);
        Ok(())
    }

    fn predict(&self, x: &PyAny) -> PyResult<Vec<i32>> {
        let (xf, n, p) = extract_flat(x)?;
        Ok(self.inner.predict(&xf, n, p))
    }

    fn predict_proba(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let flat = self.inner.predict_proba(&xf, n, p);
        let k = self.inner.classes.len();
        flat_to_np2d(py, flat, n, k)
    }

    fn score(&self, x: &PyAny, y: &PyAny) -> PyResult<f64> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        let preds = self.inner.predict(&xf, n, p);
        Ok(crate::ml::metrics::classification::accuracy_score(&yl, &preds))
    }

    #[getter] fn classes_(&self) -> Vec<i32> { self.inner.classes.clone() }
    #[getter] fn alpha_(&self) -> f64 { self.inner.alpha }

    fn __repr__(&self) -> String { format!("MultinomialNB(alpha={})", self.inner.alpha) }
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "BernoulliNB")]
pub struct PyBernoulliNB {
    inner: crate::ml::naive_bayes::bernoulli::BernoulliNB,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyBernoulliNB {
    #[new]
    #[pyo3(signature = (alpha=1.0, binarize=0.0))]
    fn py_new(alpha: f64, binarize: f64) -> Self {
        Self { inner: crate::ml::naive_bayes::bernoulli::BernoulliNB::new(alpha, binarize) }
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        self.inner.fit(&xf, n, p, &yl);
        Ok(())
    }

    fn predict(&self, x: &PyAny) -> PyResult<Vec<i32>> {
        let (xf, n, p) = extract_flat(x)?;
        Ok(self.inner.predict(&xf, n, p))
    }

    fn predict_proba(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let flat = self.inner.predict_proba(&xf, n, p);
        let k = self.inner.classes.len();
        flat_to_np2d(py, flat, n, k)
    }

    fn score(&self, x: &PyAny, y: &PyAny) -> PyResult<f64> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        let preds = self.inner.predict(&xf, n, p);
        Ok(crate::ml::metrics::classification::accuracy_score(&yl, &preds))
    }

    #[getter] fn classes_(&self) -> Vec<i32> { self.inner.classes.clone() }
    #[getter] fn alpha_(&self) -> f64 { self.inner.alpha }
    #[getter] fn binarize_(&self) -> f64 { self.inner.binarize }

    fn __repr__(&self) -> String { format!("BernoulliNB(alpha={})", self.inner.alpha) }
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "LinearSVC")]
pub struct PyLinearSVC {
    inner: crate::ml::svm::svm::LinearSVC,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyLinearSVC {
    #[new]
    #[pyo3(signature = (c=1.0, max_iter=1000, tol=1e-4, fit_intercept=true))]
    fn py_new(c: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> Self {
        Self { inner: crate::ml::svm::svm::LinearSVC::with_fit_intercept(c, max_iter, tol, fit_intercept) }
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        self.inner.fit(&xf, n, p, &yl);
        Ok(())
    }

    fn predict(&self, x: &PyAny) -> PyResult<Vec<i32>> {
        let (xf, n, p) = extract_flat(x)?;
        Ok(self.inner.predict(&xf, n, p))
    }

    fn decision_function(&self, x: &PyAny) -> PyResult<Vec<f64>> {
        let (xf, n, p) = extract_flat(x)?;
        Ok(self.inner.decision_function(&xf, n, p))
    }

    fn score(&self, x: &PyAny, y: &PyAny) -> PyResult<f64> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        let preds = self.inner.predict(&xf, n, p);
        Ok(crate::ml::metrics::classification::accuracy_score(&yl, &preds))
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

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "LinearSVR")]
pub struct PyLinearSVR {
    inner: crate::ml::svm::svm::LinearSVR,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyLinearSVR {
    #[new]
    #[pyo3(signature = (c=1.0, epsilon=0.1, max_iter=1000, tol=1e-4, fit_intercept=true))]
    fn py_new(c: f64, epsilon: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> Self {
        Self { inner: crate::ml::svm::svm::LinearSVR::with_fit_intercept(c, epsilon, max_iter, tol, fit_intercept) }
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yt = extract_targets(y)?;
        self.inner.fit(&xf, n, p, &yt);
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

    #[getter] fn coef_(&self) -> Vec<f64> { self.inner.coef().to_vec() }
    #[getter] fn intercept_(&self) -> f64 { self.inner.intercept() }
    #[getter] fn C_(&self) -> f64 { self.inner.c }
    #[getter] fn epsilon_(&self) -> f64 { self.inner.epsilon }
    #[getter] fn max_iter_(&self) -> usize { self.inner.max_iter }
    #[getter] fn tol_(&self) -> f64 { self.inner.tol }
    #[getter] fn fit_intercept_(&self) -> bool { self.inner.fit_intercept }

    fn __repr__(&self) -> String { format!("LinearSVR(C={})", self.inner.c) }
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "StandardScaler")]
pub struct PyStandardScaler {
    inner: crate::ml::preprocessing::scalers::StandardScaler,
    n: usize,
    p: usize,
}

#[cfg(feature = "python")]
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
        let flat = self.inner.transform(&xf, n, p);
        flat_to_np2d(py, flat, n, p)
    }

    fn fit_transform(&mut self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        self.n = n; self.p = p;
        let flat = self.inner.fit_transform(&xf, n, p);
        flat_to_np2d(py, flat, n, p)
    }

    fn inverse_transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let flat = self.inner.inverse_transform(&xf, n, p);
        flat_to_np2d(py, flat, n, p)
    }

    #[getter] fn mean_(&self) -> Vec<f64> { self.inner.mean.clone() }
    #[getter] fn scale_(&self) -> Vec<f64> { self.inner.scale.clone() }
    #[getter] fn with_mean_(&self) -> bool { self.inner.with_mean }
    #[getter] fn with_std_(&self) -> bool { self.inner.with_std }
    #[getter] fn var_(&self) -> Vec<f64> { self.inner.scale.iter().map(|s| s * s).collect() }

    fn __repr__(&self) -> String { "StandardScaler()".to_string() }
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "MinMaxScaler")]
pub struct PyMinMaxScaler {
    inner: crate::ml::preprocessing::scalers::MinMaxScaler,
}

#[cfg(feature = "python")]
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
        let flat = self.inner.transform(&xf, n, p);
        flat_to_np2d(py, flat, n, p)
    }

    fn fit_transform(&mut self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let flat = self.inner.fit_transform(&xf, n, p);
        flat_to_np2d(py, flat, n, p)
    }

    fn inverse_transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let flat = self.inner.inverse_transform(&xf, n, p);
        flat_to_np2d(py, flat, n, p)
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

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "RobustScaler")]
pub struct PyRobustScaler {
    inner: crate::ml::preprocessing::scalers::RobustScaler,
}

#[cfg(feature = "python")]
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
        let flat = self.inner.transform(&xf, n, p);
        flat_to_np2d(py, flat, n, p)
    }

    fn fit_transform(&mut self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let flat = self.inner.fit_transform(&xf, n, p);
        flat_to_np2d(py, flat, n, p)
    }

    #[getter] fn center_(&self) -> Vec<f64> { self.inner.center.clone() }
    #[getter] fn scale_(&self) -> Vec<f64> { self.inner.scale.clone() }
    #[getter] fn with_centering_(&self) -> bool { self.inner.with_centering }
    #[getter] fn with_scaling_(&self) -> bool { self.inner.with_scaling }
    #[getter] fn quantile_range_(&self) -> (f64, f64) { self.inner.quantile_range }

    fn __repr__(&self) -> String { "RobustScaler()".to_string() }
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "PCA")]
pub struct PyPCA {
    inner: crate::ml::decomposition::pca::PCA,
}

#[cfg(feature = "python")]
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

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "TruncatedSVD")]
pub struct PyTruncatedSVD {
    inner: crate::ml::decomposition::pca::TruncatedSVD,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyTruncatedSVD {
    #[new]
    #[pyo3(signature = (n_components=2))]
    fn py_new(n_components: usize) -> Self {
        Self { inner: crate::ml::decomposition::pca::TruncatedSVD::new(n_components) }
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

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn accuracy_score(y_true: Vec<i32>, y_pred: Vec<i32>) -> f64 {
    crate::ml::metrics::classification::accuracy_score(&y_true, &y_pred)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn mean_squared_error(y_true: Vec<f64>, y_pred: Vec<f64>) -> f64 {
    crate::ml::metrics::regression::mean_squared_error(&y_true, &y_pred)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn mean_absolute_error(y_true: Vec<f64>, y_pred: Vec<f64>) -> f64 {
    crate::ml::metrics::regression::mean_absolute_error(&y_true, &y_pred)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn r2_score(y_true: Vec<f64>, y_pred: Vec<f64>) -> f64 {
    crate::ml::metrics::regression::r2_score(&y_true, &y_pred)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (y_true, y_pred))]
pub fn classification_report(y_true: Vec<i32>, y_pred: Vec<i32>) -> String {
    crate::ml::metrics::classification::classification_report(&y_true, &y_pred)
}

#[cfg(feature = "python")]
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

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "RidgeClassifier")]
pub struct PyRidgeClassifier {
    inner: crate::ml::linear::ridge::RidgeClassifier,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyRidgeClassifier {
    #[new]
    #[pyo3(signature = (alpha=1.0))]
    fn py_new(alpha: f64) -> Self {
        Self { inner: crate::ml::linear::ridge::RidgeClassifier::new(alpha) }
    }

    fn fit(&mut self, x: &PyAny, y: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        let yl = extract_labels(y)?;
        self.inner.fit(&xf, n, p, &yl);
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

    #[getter] fn coef_(&self) -> Vec<f64> { self.inner.ridge.coef.clone() }
    #[getter] fn intercept_(&self) -> f64 { self.inner.ridge.intercept }
    #[getter] fn classes_(&self) -> Vec<i32> { self.inner.classes.clone() }

    fn __repr__(&self) -> String { format!("RidgeClassifier(alpha={})", self.inner.ridge.alpha) }
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "MaxAbsScaler")]
pub struct PyMaxAbsScaler {
    inner: crate::ml::preprocessing::scalers::MaxAbsScaler,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyMaxAbsScaler {
    #[new]
    fn py_new() -> Self {
        Self { inner: crate::ml::preprocessing::scalers::MaxAbsScaler::new() }
    }

    fn fit(&mut self, x: &PyAny) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        self.inner.fit(&xf, n, p);
        Ok(())
    }

    fn transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let flat = self.inner.transform(&xf, n, p);
        flat_to_np2d(py, flat, n, p)
    }

    fn fit_transform(&mut self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let flat = self.inner.fit_transform(&xf, n, p);
        flat_to_np2d(py, flat, n, p)
    }

    #[getter] fn max_abs_(&self) -> Vec<f64> { self.inner.max_abs.clone() }

    fn __repr__(&self) -> String { "MaxAbsScaler()".to_string() }
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "Normalizer")]
pub struct PyNormalizer {
    inner: crate::ml::preprocessing::scalers::Normalizer,
}

#[cfg(feature = "python")]
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
        let flat = self.inner.transform(&xf, n, p);
        flat_to_np2d(py, flat, n, p)
    }

    fn fit_transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let flat = self.inner.transform(&xf, n, p);
        flat_to_np2d(py, flat, n, p)
    }

    fn __repr__(&self) -> String { "Normalizer()".to_string() }
}

#[cfg(feature = "python")]
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

    m.add_class::<PyPCA>()?;
    m.add_class::<PyTruncatedSVD>()?;

    m.add_function(wrap_pyfunction!(accuracy_score, m)?)?;
    m.add_function(wrap_pyfunction!(mean_squared_error, m)?)?;
    m.add_function(wrap_pyfunction!(mean_absolute_error, m)?)?;
    m.add_function(wrap_pyfunction!(r2_score, m)?)?;
    m.add_function(wrap_pyfunction!(classification_report, m)?)?;
    m.add_function(wrap_pyfunction!(train_test_split, m)?)?;

    m.add_function(wrap_pyfunction!(ml_registry, m)?)?;

    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction]
fn ml_registry(py: Python<'_>) -> PyResult<PyObject> {
    let dict = pyo3::types::PyDict::new_bound(py);

    let regression: Vec<&str> = vec![
        "LinearRegression", "Ridge", "Lasso", "ElasticNet", "SGDRegressor",
        "DecisionTreeRegressor", "RandomForestRegressor", "GradientBoostingRegressor",
        "AdaBoostRegressor", "KNeighborsRegressor", "LinearSVR",
    ];
    let classification: Vec<&str> = vec![
        "LogisticRegression", "SGDClassifier", "DecisionTreeClassifier",
        "RandomForestClassifier", "GradientBoostingClassifier", "AdaBoostClassifier",
        "KNeighborsClassifier", "NearestCentroid", "GaussianNB", "MultinomialNB",
        "BernoulliNB", "LinearSVC", "RidgeClassifier",
    ];
    let preprocessing: Vec<&str> = vec![
        "StandardScaler", "MinMaxScaler", "RobustScaler", "MaxAbsScaler", "Normalizer",
    ];
    let dimensionality_reduction: Vec<&str> = vec![
        "PCA", "TruncatedSVD",
    ];
    let model_selection: Vec<&str> = vec![
        "train_test_split",
    ];
    let metrics: Vec<&str> = vec![
        "accuracy_score", "mean_squared_error", "mean_absolute_error",
        "r2_score", "classification_report",
    ];

    dict.set_item("regression", regression)?;
    dict.set_item("classification", classification)?;
    dict.set_item("preprocessing", preprocessing)?;
    dict.set_item("dimensionality_reduction", dimensionality_reduction)?;
    dict.set_item("model_selection", model_selection)?;
    dict.set_item("metrics", metrics)?;

    Ok(dict.into())
}
