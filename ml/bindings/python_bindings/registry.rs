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
    fn fit(&mut self, x: &Bound<'_, PyAny>) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        self.inner.fit(&xf, n, p);
        Ok(())
    }
    fn transform(&self, py: Python<'_>, x: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let out = self.inner.transform(&xf, n, p);
        flat_to_np2d(py, out, n, p)
    }
    fn fit_transform(&mut self, py: Python<'_>, x: &Bound<'_, PyAny>) -> PyResult<PyObject> {
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
    fn fit(&mut self, x: &Bound<'_, PyAny>) -> PyResult<()> {
        with_flat(x, |xf, n, p| self.inner.fit(xf, n, p))?;
        Ok(())
    }
    fn transform(&self, py: Python<'_>, x: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let (out, n) = with_flat(x, |xf, nn, pp| (self.inner.transform(xf, nn, pp), nn))?;
        let cols = self.inner.n_output_features();
        flat_to_np2d(py, out, n, cols)
    }
    fn fit_transform(&mut self, py: Python<'_>, x: &Bound<'_, PyAny>) -> PyResult<PyObject> {
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
    fn fit(&mut self, x: &Bound<'_, PyAny>) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        self.inner.fit(&xf, n, p);
        Ok(())
    }
    fn transform(&self, py: Python<'_>, x: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let out = self.inner.transform(&xf, n, p);
        flat_to_np2d(py, out, n, p)
    }
    fn fit_transform(&mut self, py: Python<'_>, x: &Bound<'_, PyAny>) -> PyResult<PyObject> {
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
    fn fit(&mut self, x: &Bound<'_, PyAny>) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        self.inner.fit(&xf, n, p);
        Ok(())
    }
    fn transform(&self, py: Python<'_>, x: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let out = self.inner.transform(&xf, n, p);
        flat_to_np2d(py, out, n, p)
    }
    fn fit_transform(&mut self, py: Python<'_>, x: &Bound<'_, PyAny>) -> PyResult<PyObject> {
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
    fn fit(&mut self, x: &Bound<'_, PyAny>) -> PyResult<()> {
        let (xf, n, p) = extract_flat(x)?;
        self.inner.fit(&xf, n, p);
        Ok(())
    }
    fn transform(&self, py: Python<'_>, x: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let (xf, n, p) = extract_flat(x)?;
        let out = self.inner.transform(&xf, n, p);
        flat_to_np2d(py, out, n, p)
    }
    fn fit_transform(&mut self, py: Python<'_>, x: &Bound<'_, PyAny>) -> PyResult<PyObject> {
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
pub fn log_loss(y_true: Vec<i32>, y_proba: &Bound<'_, PyAny>, eps: f64) -> PyResult<f64> {
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
pub fn mean_squared_log_error(y_true: &Bound<'_, PyAny>, y_pred: &Bound<'_, PyAny>) -> PyResult<f64> {
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
pub fn silhouette_score(x: &Bound<'_, PyAny>, labels: Vec<i32>) -> PyResult<f64> {
    let (xf, n, p) = extract_flat(x)?;
    Ok(crate::ml::metrics::clustering::silhouette_score(&xf, &labels, n, p))
}

#[pyfunction]
#[pyo3(signature = (x, labels))]
pub fn davies_bouldin_score(x: &Bound<'_, PyAny>, labels: Vec<i32>) -> PyResult<f64> {
    let (xf, n, p) = extract_flat(x)?;
    Ok(crate::ml::metrics::clustering::davies_bouldin_score(&xf, &labels, n, p))
}

#[pyfunction]
#[pyo3(signature = (x, labels))]
pub fn calinski_harabasz_score(x: &Bound<'_, PyAny>, labels: Vec<i32>) -> PyResult<f64> {
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
fn export_powerbi(name: &str, table: &str, x: &Bound<'_, PyAny>, y: Option<&Bound<'_, PyAny>>, y_pred: Option<&Bound<'_, PyAny>>) -> PyResult<String> {
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
fn export_tableau_tds(name: &str, x: &Bound<'_, PyAny>, y: Option<&Bound<'_, PyAny>>, y_pred: Option<&Bound<'_, PyAny>>) -> PyResult<String> {
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
fn export_tableau_csv(name: &str, x: &Bound<'_, PyAny>, y: Option<&Bound<'_, PyAny>>, y_pred: Option<&Bound<'_, PyAny>>) -> PyResult<String> {
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

