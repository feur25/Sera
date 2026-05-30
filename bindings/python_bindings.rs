#[macro_export]
macro_rules! impl_python_build_grid {
    () => {
        #[pyfunction]
        #[pyo3(signature = (charts, cols=2, gap=16, bg=None, cell_height=520))]
        pub fn build_grid(
            charts: Vec<pyo3::PyRef<crate::Chart>>,
            cols: usize,
            gap: i32,
            bg: Option<&str>,
            cell_height: i32,
        ) -> crate::Chart {
            let html_strs: Vec<String> = charts.iter().map(|c| c.html.clone()).collect();
            crate::Chart::new_doc(
                crate::bindings::commands::charts::build_grid(
                    &serde_json::json!({"charts": html_strs, "cols": cols, "gap": gap, "background": bg, "cell_height": cell_height}).to_string()
                ),
                crate::bindings::commands::docs::DOC_BUILD_GRID,
            )
        }
    };
}

#[macro_export]
macro_rules! impl_python_build_slideshow {
    () => {
        #[pyfunction]
        #[pyo3(signature = (charts, interval_ms=2500, title="", width=900, height=520))]
        pub fn build_slideshow(
            charts: Vec<pyo3::PyRef<crate::Chart>>,
            interval_ms: u32,
            title: &str,
            width: i32,
            height: i32,
        ) -> crate::Chart {
            let html_strs: Vec<String> = charts.iter().map(|c| c.html.clone()).collect();
            crate::Chart::new_doc(
                crate::bindings::commands::charts::build_slideshow(
                    &serde_json::json!({"charts": html_strs, "interval_ms": interval_ms, "title": title, "width": width, "height": height}).to_string()
                ),
                crate::bindings::commands::docs::DOC_BUILD_SLIDESHOW,
            )
        }
    };
}

#[macro_export]
macro_rules! impl_python_build_hover_json {
    () => {
        #[pyfunction]
        #[pyo3(signature = (labels, images=None, descriptions=None))]
        pub fn build_hover_json(
            labels: Vec<String>,
            images: Option<Vec<Option<String>>>,
            descriptions: Option<Vec<Vec<(String, String)>>>,
        ) -> String {
            let desc_json: Option<Vec<Vec<Vec<String>>>> = descriptions.map(|outer|
                outer.into_iter().map(|row| row.into_iter().map(|(k, v)| vec![k, v]).collect()).collect()
            );
            crate::bindings::commands::charts::build_hover_json(
                &serde_json::json!({"labels": labels, "images": images, "descriptions": desc_json}).to_string()
            )
        }
    };
}
#[macro_export]
macro_rules! impl_python_bindings {
    () => {
        use pyo3::prelude::*;
        use pyo3::types::{PyAny, PyDict};

        fn py_any_val(val: &Bound<'_, PyAny>) -> serde_json::Value {
            if val.is_none() { return serde_json::Value::Null; }
            if let Ok(b) = val.extract::<bool>() { return serde_json::Value::Bool(b); }
            if let Ok(i) = val.extract::<i64>() { return serde_json::json!(i); }
            if let Ok(f) = val.extract::<f64>() { return serde_json::json!(f); }
            if let Ok(s) = val.extract::<String>() { return serde_json::Value::String(s); }
            if let Ok(list) = val.extract::<Vec<Vec<f64>>>() { return serde_json::json!(list); }
            if let Ok(list) = val.extract::<Vec<i64>>() { return serde_json::json!(list); }
            if let Ok(list) = val.extract::<Vec<f64>>() { return serde_json::json!(list); }
            if let Ok(list) = val.extract::<Vec<String>>() { return serde_json::json!(list); }
            let py = val.py();
            if let Ok(json_mod) = py.import_bound("json") {
                if let Ok(s) = json_mod.call_method1("dumps", (val,)) {
                    if let Ok(ss) = s.extract::<String>() {
                        if let Ok(jv) = serde_json::from_str::<serde_json::Value>(&ss) {
                            return jv;
                        }
                    }
                }
            }
            serde_json::Value::Null
        }

        fn py_args_to_json(
            title: &str,
            labels: Option<&Bound<'_, PyAny>>,
            values: Option<&Bound<'_, PyAny>>,
            theme: Option<&str>,
            kwargs: Option<&Bound<'_, PyDict>>,
        ) -> String {
            let mut m = serde_json::Map::new();
            if !title.is_empty() { m.insert("title".into(), serde_json::json!(title)); }
            if let Some(v) = labels {
                let jv = py_any_val(v);
                if jv != serde_json::Value::Null { m.insert("labels".into(), jv); }
            }
            if let Some(v) = values {
                if let Ok(vv) = v.extract::<Vec<Vec<f64>>>() {
                    m.insert("series".into(), serde_json::json!(vv));
                } else {
                    let jv = py_any_val(v);
                    if jv != serde_json::Value::Null { m.insert("values".into(), jv); }
                }
            }
            if let Some(t) = theme {
                m.insert("theme".into(), serde_json::json!(t));
            }
            if let Some(d) = kwargs {
                for (key, val) in d.iter() {
                    if let Ok(ks) = key.str() {
                        let ks = ks.to_string_lossy().to_string();
                        let v = py_any_val(&val);
                        if v != serde_json::Value::Null { m.insert(ks, v); }
                    }
                }
            }
            serde_json::Value::Object(m).to_string()
        }

        macro_rules! impl_python {
            ($fn:ident, $_js:literal) => {
                #[pyfunction]
                #[pyo3(signature = (title="", labels=None, values=None, theme=None, **kwargs))]
                pub fn $fn(
                    title: &str,
                    labels: Option<&Bound<'_, PyAny>>,
                    values: Option<&Bound<'_, PyAny>>,
                    theme: Option<&str>,
                    kwargs: Option<&Bound<'_, PyDict>>,
                ) -> PyResult<crate::Chart> {
                    let t = std::time::Instant::now();
                    let result = crate::Chart::new_doc(
                        crate::bindings::commands::charts::$fn(&py_args_to_json(title, labels, values, theme, kwargs)),
                        "",
                    );
                    let _dc = labels.and_then(|l| l.len().ok()).unwrap_or(0) as u64;
                    let mut _ev = crate::telemetry::TelemetryEvent::new(stringify!($fn), t.elapsed().as_secs_f64() * 1000.0);
                    if _dc > 0 { _ev = _ev.with_data(_dc, 0.0); }
                    crate::telemetry::record(_ev);
                    Ok(result)
                }
            };
        }
        for_each_json_chart_fn!(impl_python);

        macro_rules! impl_python_json_ml {
            ($fn:ident, $_js:literal) => {
                #[pyfunction]
                pub fn $fn(input: &str) -> String {
                    crate::bindings::commands::ml::$fn(input)
                }
            };
        }
        for_each_ml_oneshot_fn!(impl_python_json_ml);

        macro_rules! impl_python_json_util {
            ($fn:ident, $_js:literal) => {
                #[pyfunction]
                pub fn $fn(input: &str) -> String {
                    crate::bindings::commands::charts::$fn(input)
                }
            };
        }
        for_each_auto_util_fn!(impl_python_json_util);

        #[pyfunction]
        #[pyo3(name = "set_bg", signature = (html, color=None))]
        pub fn set_bg_fn(html: &Bound<'_, PyAny>, color: Option<&str>) -> PyResult<crate::Chart> {
            let raw: String = if let Ok(chart) = html.extract::<PyRef<crate::Chart>>() {
                chart.html.clone()
            } else { html.extract::<String>()? };
            Ok(crate::Chart::new(crate::html::hover::apply_bg(raw, color)))
        }

        #[pyfunction]
        pub fn show_chart_value(chart_json: &str) -> bool { crate::bindings::commands::charts::show_chart_value(chart_json) }

        #[pyfunction]
        pub fn bench_chart_value(chart_json: &str) -> bool { crate::bindings::commands::charts::bench_chart_value(chart_json) }

        #[pyfunction]
        pub fn set_chart_kind(kind: u8) -> bool { crate::bindings::commands::charts::set_chart_kind(kind); true }

        #[pyfunction]
        pub fn set_chart_orientation(vertical: bool) -> bool { crate::bindings::commands::charts::set_chart_orientation(vertical); true }

        #[pyfunction]
        #[pyo3(signature = (n=2000))]
        pub fn bench_pure_rust(n: usize) -> (f64, f64, f64, f64) { crate::bindings::commands::charts::bench_pure_rust(n) }

        #[pyclass(module = "seraplot", name = "DBSCAN")]
        pub struct DbscanModel(crate::ml::bindings::DbscanState);

        #[pymethods]
        impl DbscanModel {
            #[new]
            #[pyo3(signature = (eps=0.5, min_samples=5))]
            pub fn py_new(eps: f64, min_samples: usize) -> Self {
                DbscanModel(crate::ml::bindings::DbscanState::new(eps, min_samples))
            }
            #[pyo3(signature = (x))]
            pub fn fit(&mut self, x: &Bound<'_, PyAny>) -> PyResult<()> {
                let t = std::time::Instant::now();
                super::ml::with_flat(x, |xf, n, d| {
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
                Ok(crate::bindings::commands::ml::vec_i32_to_np(py, self.0.labels.clone()))
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
        pub struct KMeansModel(crate::ml::bindings::KMeansState);

        #[pymethods]
        impl KMeansModel {
            #[new]
            #[pyo3(signature = (k=3, max_iter=300, tol=1e-4, mini_batch=false, batch_size=1000, n_init=10, n_clusters=None, random_state=None))]
            pub fn py_new(k: usize, max_iter: usize, tol: f64, mini_batch: bool, batch_size: usize, n_init: usize, n_clusters: Option<usize>, random_state: Option<i64>) -> Self {
                let _ = random_state;
                let kk = n_clusters.unwrap_or(k);
                KMeansModel(crate::ml::bindings::KMeansState::new(kk, max_iter, tol, mini_batch, batch_size, n_init))
            }
            #[pyo3(signature = (x))]
            pub fn fit(&mut self, x: &Bound<'_, PyAny>) -> PyResult<()> {
                let t = std::time::Instant::now();
                super::ml::with_flat(x, |xf, n, d| {
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
                Ok(crate::bindings::commands::ml::vec_i32_to_np(py, self.0.labels.clone()))
            }
            #[pyo3(signature = (x))]
            pub fn transform(&self, py: Python<'_>, x: &Bound<'_, PyAny>) -> PyResult<pyo3::PyObject> {
                let (flat, n, dims) = super::ml::extract_flat(x)?;
                crate::bindings::commands::ml::vv_to_np2d(py, self.0.transform_flat(&flat, n, dims))
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
                        return Ok(crate::bindings::commands::ml::vec_i32_to_np(py, self.0.predict_flat(s, n, dims)));
                    }
                }
                let (flat, n, dims) = super::ml::extract_flat(x)?;
                Ok(crate::bindings::commands::ml::vec_i32_to_np(py, self.0.predict_flat(&flat, n, dims)))
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

        crate::impl_python_build_grid!();
        crate::impl_python_build_slideshow!();
        crate::impl_python_build_hover_json!();

        #[pyfunction]
        #[pyo3(name = "plot", signature = (x, y=None, *, title="", kind="line", color_hex=0x6366F1_u32, width=900_i32, height=480_i32, x_label="", y_label="", gridlines=false, palette=None, background=None, show_points=true))]
        pub fn plot_chart(
            py: Python<'_>,
            x: &Bound<'_, PyAny>,
            y: Option<&Bound<'_, PyAny>>,
            title: &str,
            kind: &str,
            color_hex: u32,
            width: i32,
            height: i32,
            x_label: &str,
            y_label: &str,
            gridlines: bool,
            palette: Option<Vec<u32>>,
            background: Option<&str>,
            show_points: bool,
        ) -> PyResult<crate::Chart> {
            let t = std::time::Instant::now();
            let (bg, pal, grid) = crate::merge_global_opts(background, palette, gridlines);
            let pal_opt: Option<Vec<u32>> = if pal.is_empty() { None } else { Some(pal) };
            if let Ok(cols_obj) = x.getattr("columns") {
                let cols: Vec<String> = cols_obj.extract()?;
                if cols.len() >= 2 {
                    let x_col = x.get_item(cols[0].as_str())?;
                    let y_col = x.get_item(cols[1].as_str())?;
                    let xv = crate::extract_f64_vec(py, &x_col)?;
                    let yv = crate::extract_f64_vec(py, &y_col)?;
                    let auto_title = if title.is_empty() { format!("{} vs {}", cols[0], cols[1]) } else { title.to_string() };
                    let result = Ok(crate::Chart::new_doc(
                        crate::bindings::commands::charts::plot_chart(
                            &serde_json::json!({"x":xv,"y":yv,"kind":"scatter","title":auto_title,"color_hex":color_hex,"width":width,"height":height,"x_label":x_label,"y_label":y_label,"gridlines":grid,"palette":pal_opt,"background":bg}).to_string()
                        ), "",
                    ));
                    crate::telemetry::record(crate::telemetry::TelemetryEvent::new("plot", t.elapsed().as_secs_f64() * 1000.0));
                    return result;
                }
            }
            let xv = crate::extract_f64_vec(py, x)?;
            let yv: Option<Vec<f64>> = if let Some(yobj) = y { Some(crate::extract_f64_vec(py, yobj)?) } else { None };
            let res = Ok(crate::Chart::new_doc(
                crate::bindings::commands::charts::plot_chart(
                    &serde_json::json!({"x":xv,"y":yv,"kind":kind,"title":title,"color_hex":color_hex,"show_points":show_points,"width":width,"height":height,"x_label":x_label,"y_label":y_label,"gridlines":grid,"palette":pal_opt,"background":bg}).to_string()
                ), "",
            ));
            crate::telemetry::record(crate::telemetry::TelemetryEvent::new("plot", t.elapsed().as_secs_f64() * 1000.0).with_data(xv.len() as u64, 0.0));
            res
        }

    };
}

