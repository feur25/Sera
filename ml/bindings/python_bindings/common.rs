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

pub fn extract_flat(x: &Bound<'_, PyAny>) -> PyResult<(Vec<f64>, usize, usize)> {
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

pub fn with_flat<R>(x: &Bound<'_, PyAny>, f: impl FnOnce(&[f64], usize, usize) -> R) -> PyResult<R> {
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

fn extract_labels(y: &Bound<'_, PyAny>) -> PyResult<Vec<i32>> {
    if let Ok(arr) = y.extract::<PyReadonlyArray1<i64>>() { return Ok(arr.as_slice().unwrap().iter().map(|&v| v as i32).collect()); }
    if let Ok(arr) = y.extract::<PyReadonlyArray1<i32>>() { return Ok(arr.as_slice().unwrap().to_vec()); }
    if let Ok(arr) = y.extract::<PyReadonlyArray1<f64>>() { return Ok(arr.as_slice().unwrap().iter().map(|&v| v as i32).collect()); }
    y.extract::<Vec<i32>>()
}

fn extract_targets(y: &Bound<'_, PyAny>) -> PyResult<Vec<f64>> {
    if let Ok(arr) = y.extract::<PyReadonlyArray1<f64>>() { return Ok(arr.as_slice().unwrap().to_vec()); }
    if let Ok(arr) = y.extract::<PyReadonlyArray1<f32>>() { return Ok(arr.as_slice().unwrap().iter().map(|&v| v as f64).collect()); }
    if let Ok(arr) = y.extract::<PyReadonlyArray1<i64>>() { return Ok(arr.as_slice().unwrap().iter().map(|&v| v as f64).collect()); }
    y.extract::<Vec<f64>>()
}

pub fn extract_estimator_name(est: &Bound<'_, PyAny>) -> PyResult<String> {
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
            fn fit(&mut self, x: &Bound<'_, PyAny>, y: &Bound<'_, PyAny>) -> PyResult<()> {
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
            fn predict(&self, py: Python<'_>, x: &Bound<'_, PyAny>) -> PyResult<PyObject> {
                let v = with_flat(x, |xf, n, p| self.inner.predict(xf, n, p))?;
                Ok(vec_f64_to_np(py, v))
            }
            fn score(&self, x: &Bound<'_, PyAny>, y: &Bound<'_, PyAny>) -> PyResult<f64> {
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
            fn fit(&mut self, x: &Bound<'_, PyAny>, y: &Bound<'_, PyAny>) -> PyResult<()> {
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
            fn predict(&self, py: Python<'_>, x: &Bound<'_, PyAny>) -> PyResult<PyObject> {
                let v = with_flat(x, |xf, n, p| self.inner.predict(xf, n, p))?;
                Ok(vec_i32_to_np(py, v))
            }
            fn score(&self, x: &Bound<'_, PyAny>, y: &Bound<'_, PyAny>) -> PyResult<f64> {
                let yl = extract_labels(y)?;
                let preds = with_flat(x, |xf, n, p| self.inner.predict(xf, n, p))?;
                Ok(crate::ml::metrics::classification::accuracy_score(&yl, &preds))
            }
        }
