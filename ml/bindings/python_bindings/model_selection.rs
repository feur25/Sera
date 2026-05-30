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
    fn py_new(estimator: &Bound<'_, PyAny>, param_grid: &Bound<'_, PyDict>, cv: usize, seed: u64, scoring: &str) -> PyResult<Self> {
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
    fn fit(&mut self, x: &Bound<'_, PyAny>, y: &Bound<'_, PyAny>, task_id: Option<u64>) -> PyResult<()> {
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
    fn py_new(estimator: &Bound<'_, PyAny>, param_distributions: &Bound<'_, PyDict>, n_iter: usize, cv: usize, seed: u64, scoring: &str) -> PyResult<Self> {
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
    fn fit(&mut self, x: &Bound<'_, PyAny>, y: &Bound<'_, PyAny>, task_id: Option<u64>) -> PyResult<()> {
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
    fn py_new(estimator: &Bound<'_, PyAny>, param_grid: &Bound<'_, PyDict>, cv: usize, factor: usize, seed: u64, scoring: &str) -> PyResult<Self> {
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
    fn fit(&mut self, x: &Bound<'_, PyAny>, y: &Bound<'_, PyAny>, task_id: Option<u64>) -> PyResult<()> {
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
    fn py_new(estimator: &Bound<'_, PyAny>, param_distributions: &Bound<'_, PyDict>, n_candidates: usize, cv: usize, factor: usize, seed: u64, scoring: &str) -> PyResult<Self> {
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
    fn fit(&mut self, x: &Bound<'_, PyAny>, y: &Bound<'_, PyAny>, task_id: Option<u64>) -> PyResult<()> {
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
