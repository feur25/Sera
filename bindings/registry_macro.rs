#[macro_export]
macro_rules! for_each_json_chart_fn {
    ($mac:ident) => {
        $mac!(build_html_chart,          "buildHtmlChart");
        $mac!(build_bar_chart,           "buildBarChart");
        $mac!(build_bar,                 "buildBar");
        $mac!(build_hbar,                "buildHbar");
        $mac!(build_line_chart,          "buildLineChart");
        $mac!(build_line,                "buildLine");
        $mac!(build_dbscan_chart,        "buildDbscanChart");
        $mac!(build_dbscan_chart_3d,     "buildDbscanChart3d");
        $mac!(build_kmeans_chart,        "buildKmeansChart");
        $mac!(build_scatter_chart,       "buildScatterChart");
        $mac!(build_histogram,           "buildHistogram");
        $mac!(build_histogram_overlay,   "buildHistogramOverlay");
        $mac!(build_grouped_bar,         "buildGroupedBar");
        $mac!(build_stacked_bar,         "buildStackedBar");
        $mac!(build_heatmap,             "buildHeatmap");
        $mac!(build_pie_chart,           "buildPieChart");
        $mac!(build_pie,                 "buildPie");
        $mac!(build_donut_chart,         "buildDonutChart");
        $mac!(build_boxplot,             "buildBoxplot");
        $mac!(build_violin,              "buildViolin");
        $mac!(build_slope,               "buildSlope");
        $mac!(build_sunburst,            "buildSunburst");
        $mac!(build_funnel,              "buildFunnel");
        $mac!(build_treemap,             "buildTreemap");
        $mac!(build_multiline_chart,     "buildMultilineChart");
        $mac!(build_area_chart,          "buildAreaChart");
        $mac!(build_waterfall,           "buildWaterfall");
        $mac!(build_bullet,              "buildBullet");
        $mac!(build_bubble_map,          "buildBubbleMap");
        $mac!(build_choropleth,          "buildChoropleth");
        $mac!(build_scatter3d_chart,     "buildScatter3dChart");
        $mac!(build_bar3d_chart,         "buildBar3dChart");
        $mac!(build_line3d_chart,        "buildLine3dChart");
        $mac!(build_radar_chart,         "buildRadarChart");
        $mac!(build_radar3d_chart,       "buildRadar3dChart");
        $mac!(build_lollipop_chart,      "buildLollipopChart");
        $mac!(build_lollipop3d_chart,    "buildLollipop3dChart");
        $mac!(build_kde_chart,           "buildKdeChart");
        $mac!(build_kde3d_chart,         "buildKde3dChart");
        $mac!(build_ridgeline_chart,     "buildRidgelineChart");
        $mac!(build_ridgeline3d_chart,   "buildRidgeline3dChart");
        $mac!(build_bubble3d_chart,      "buildBubble3dChart");
        $mac!(build_pie3d_chart,         "buildPie3dChart");
        $mac!(build_violin3d_chart,      "buildViolin3dChart");
        $mac!(build_heatmap3d_chart,     "buildHeatmap3dChart");
        $mac!(build_candlestick3d_chart, "buildCandlestick3dChart");
        $mac!(build_dumbbell3d_chart,    "buildDumbbell3dChart");
        $mac!(build_funnel3d_chart,      "buildFunnel3dChart");
        $mac!(build_sunburst3d_chart,    "buildSunburst3dChart");
        $mac!(build_stacked_bar3d_chart, "buildStackedBar3dChart");
        $mac!(build_globe3d_chart,       "buildGlobe3dChart");
        $mac!(build_wordcloud,           "buildWordcloud");
        $mac!(build_candlestick,         "buildCandlestick");
        $mac!(build_dumbbell,            "buildDumbbell");
        $mac!(build_bubble,              "buildBubble");
        $mac!(build_gauge,               "buildGauge");
        $mac!(build_parallel,            "buildParallel");
    };
}

#[macro_export]
macro_rules! for_each_chart_fn {
    ($mac:ident) => {
        crate::for_each_json_chart_fn!($mac);
        $mac!(build_grid,                "buildGrid");
        $mac!(build_slideshow,           "buildSlideshow");
    };
}


#[macro_export]
macro_rules! for_each_chart_class {
    ($mac:ident) => {
        $mac!(DbscanModel);
        $mac!(KMeansModel);
    };
}

pub const CHART_ALIASES: &[(&str, &str)] = &[
    ("bar",            "build_bar"),
    ("bar_chart",      "build_bar_chart"),
    ("bars",           "build_bar"),
    ("bar_unified",    "build_bar"),
    ("bars_unified",   "build_bar"),
    ("bar_family",     "build_bar"),
    ("hbar",           "build_hbar"),
    ("barh",           "build_hbar"),
    ("horizontal_bar", "build_hbar"),
    ("line",           "build_line"),
    ("line_chart",     "build_line"),
    ("line_unified",   "build_line"),
    ("lines_unified",  "build_line"),
    ("line_family",    "build_line"),
    ("lines_family",   "build_line"),
    ("scatter",        "build_scatter_chart"),
    ("scatter_chart",  "build_scatter_chart"),
    ("scatter_family", "build_scatter_chart"),
    ("scatter_unified","build_scatter_chart"),
    ("scatters",       "build_scatter_chart"),
    ("hist",           "build_histogram"),
    ("histogram",      "build_histogram"),
    ("histograms",     "build_histogram"),
    ("histogram_family","build_histogram"),
    ("histogram_unified","build_histogram"),
    ("pie",            "build_pie"),
    ("pie_chart",      "build_pie_chart"),
    ("pie_unified",    "build_pie"),
    ("pie_family",     "build_pie"),
    ("pies",           "build_pie"),
    ("semi_pie",       "build_pie"),
    ("half_pie",       "build_pie"),
    ("kpi_pie",        "build_pie"),
    ("kpi_donut",      "build_pie"),
    ("nested_pie",     "build_pie"),
    ("concentric_pie", "build_pie"),
    ("pattern_pie",    "build_pie"),
    ("donut",          "build_donut_chart"),
    ("donut_chart",    "build_donut_chart"),
    ("heatmap",        "build_heatmap"),
    ("heatmaps",       "build_heatmap"),
    ("heatmap_family", "build_heatmap"),
    ("heatmap_unified","build_heatmap"),
    ("boxplot",        "build_boxplot"),
    ("box_plot",       "build_boxplot"),
    ("violin",         "build_violin"),
    ("violins",        "build_violin"),
    ("violin_chart",   "build_violin"),
    ("violin_family",  "build_violin"),
    ("violin_unified", "build_violin"),
    ("radar",          "build_radar_chart"),
    ("radar_chart",    "build_radar_chart"),
    ("lollipop",       "build_lollipop_chart"),
    ("kde",            "build_kde_chart"),
    ("ridgeline",      "build_ridgeline_chart"),
    ("bubble",         "build_bubble"),
    ("bubble_family",  "build_bubble"),
    ("bubble_unified", "build_bubble"),
    ("bubbles",        "build_bubble"),
    ("candlestick",    "build_candlestick"),
    ("dumbbell",       "build_dumbbell"),
    ("funnel",         "build_funnel"),
    ("waterfall",      "build_waterfall"),
    ("treemap",        "build_treemap"),
    ("sunburst",       "build_sunburst"),
    ("gauge",          "build_gauge"),
    ("parallel",       "build_parallel"),
    ("grouped_bar",    "build_grouped_bar"),
    ("stacked_bar",    "build_stacked_bar"),
    ("slope",          "build_slope"),
    ("bullet",         "build_bullet"),
    ("area",           "build_area_chart"),
    ("area_chart",     "build_area_chart"),
    ("multiline",      "build_multiline_chart"),
    ("bubble_map",     "build_bubble_map"),
    ("choropleth",     "build_choropleth"),
    ("wordcloud",      "build_wordcloud"),
    ("word_cloud",     "build_wordcloud"),
    ("wordCloud",      "build_wordcloud"),
    ("tag_cloud",      "build_wordcloud"),
    ("tagcloud",       "build_wordcloud"),
    ("cloud",          "build_wordcloud"),
    ("token_cloud",    "build_wordcloud"),
    ("text_cloud",     "build_wordcloud"),
    ("kmeans",         "build_kmeans_chart"),
    ("kmeans_chart",   "build_kmeans_chart"),
    ("dbscan",         "build_dbscan_chart"),
    ("scatter3d",      "build_scatter3d_chart"),
    ("bar3d",          "build_bar3d_chart"),
    ("line3d",         "build_line3d_chart"),
    ("radar3d",        "build_radar3d_chart"),
    ("lollipop3d",     "build_lollipop3d_chart"),
    ("kde3d",          "build_kde3d_chart"),
    ("ridgeline3d",    "build_ridgeline3d_chart"),
    ("bubble3d",       "build_bubble3d_chart"),
    ("pie3d",          "build_pie3d_chart"),
    ("violin3d",       "build_violin3d_chart"),
    ("heatmap3d",      "build_heatmap3d_chart"),
    ("candlestick3d",  "build_candlestick3d_chart"),
    ("dumbbell3d",     "build_dumbbell3d_chart"),
    ("funnel3d",       "build_funnel3d_chart"),
    ("sunburst3d",     "build_sunburst3d_chart"),
    ("stacked_bar3d",  "build_stacked_bar3d_chart"),
    ("globe3d",        "build_globe3d_chart"),
    ("dbscan3d",       "build_dbscan_chart_3d"),
    ("plot",           "plot_chart"),
    ("grid",           "build_grid"),
    ("save",           "savefig"),
    ("save_fig",       "savefig"),
];

#[macro_export]
macro_rules! for_each_ml_oneshot_fn {
    ($mac:ident) => {
        $mac!(ml_dbscan_fit_predict,             "mlDbscanFitPredict");
        $mac!(ml_kmeans_fit_predict,             "mlKmeansFitPredict");
        $mac!(ml_linear_regression,              "mlLinearRegression");
        $mac!(ml_ridge,                          "mlRidge");
        $mac!(ml_lasso,                          "mlLasso");
        $mac!(ml_elastic_net,                    "mlElasticNet");
        $mac!(ml_logistic_regression,            "mlLogisticRegression");
        $mac!(ml_decision_tree_classifier,       "mlDecisionTreeClassifier");
        $mac!(ml_decision_tree_regressor,        "mlDecisionTreeRegressor");
        $mac!(ml_random_forest_classifier,       "mlRandomForestClassifier");
        $mac!(ml_random_forest_regressor,        "mlRandomForestRegressor");
        $mac!(ml_gradient_boosting_classifier,   "mlGradientBoostingClassifier");
        $mac!(ml_gradient_boosting_regressor,    "mlGradientBoostingRegressor");
        $mac!(ml_knn_classifier,                 "mlKnnClassifier");
        $mac!(ml_knn_regressor,                  "mlKnnRegressor");
        $mac!(ml_metric_score,                   "mlMetricScore");
        $mac!(ml_metric_curve,                   "mlMetricCurve");
        $mac!(ml_fit_transform,                  "mlFitTransform");
        $mac!(ml_kfold_split,                    "mlKfoldSplit");
        $mac!(ml_isolation_forest,               "mlIsolationForest");
        $mac!(ml_permutation_importance,         "mlPermutationImportance");
    };
}

#[macro_export]
macro_rules! for_each_fn {
    ($mac:ident) => {
        $crate::for_each_chart_fn!($mac);
        $mac!(plot_chart,                "plotChart");
        $mac!(build_hover_json,          "buildHoverJson");
        $crate::for_each_ml_oneshot_fn!($mac);
        $mac!(set_theme,                 "setTheme");
        $mac!(reset_theme,               "resetTheme");
        $mac!(themes,                    "themes");
        $mac!(set_global_background,     "setGlobalBackground");
        $mac!(reset_global_background,   "resetGlobalBackground");
        $mac!(export_svg,                "exportSvg");
        $mac!(export_data_url,           "exportDataUrl");
        $mac!(export_html_file,          "exportHtmlFile");
        $mac!(chart_append,              "chartAppend");
        $mac!(ml_save_model,             "mlSaveModel");
        $mac!(ml_load_model,             "mlLoadModel");
        $mac!(chart_info,                "chartInfo");
        $mac!(validate_input,            "validateInput");
        $mac!(downsample_lttb,           "downsampleLttb");
        $mac!(chart_diff,                "chartDiff");
        $mac!(drift_ks,                  "driftKs");
    };
}

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
macro_rules! impl_wasm_bindings {
    () => {
        use wasm_bindgen::prelude::*;
        macro_rules! impl_wasm {
            ($fn:ident, $js:literal) => {
                #[wasm_bindgen(js_name = $js)]
                pub fn $fn(input: &str) -> String {
                    crate::bindings::commands::charts::$fn(input)
                }
            };
        }
        crate::for_each_fn!(impl_wasm);

        #[wasm_bindgen(js_name = "demo")]
        pub fn demo(input: &str) -> String {
            #[derive(serde::Deserialize, Default)]
            struct In { family: Option<String>, variant: Option<String> }
            let i: In = serde_json::from_str(input).unwrap_or_default();
            let f = i.family.unwrap_or_default();
            let v = i.variant.unwrap_or_else(|| "basic".to_string());
            crate::demo_snippet(&f, &v).unwrap_or_default()
        }
    };
}

#[macro_export]
macro_rules! impl_cffi_bindings {
    () => {
        macro_rules! impl_cffi {
            ($fn:ident, $_js:literal) => {
                #[no_mangle]
                pub unsafe extern "C" fn $fn(input: *const std::os::raw::c_char) -> *mut std::os::raw::c_char {
                    let s = std::ffi::CStr::from_ptr(input).to_str().unwrap_or("");
                    std::ffi::CString::new(crate::bindings::commands::charts::$fn(s))
                        .unwrap_or_default()
                        .into_raw()
                }
            };
        }
        crate::for_each_fn!(impl_cffi);
        #[no_mangle]
        pub unsafe extern "C" fn seraplot_free(ptr: *mut std::os::raw::c_char) {
            if !ptr.is_null() { drop(std::ffi::CString::from_raw(ptr)); }
        }
    };
}

#[macro_export]
macro_rules! impl_python_bindings {
    () => {
        use pyo3::prelude::*;
        use pyo3::types::PyAny;

        fn py_any_val(val: &PyAny) -> serde_json::Value {
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
            if let Ok(json_mod) = py.import("json") {
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
            labels: Option<&pyo3::types::PyAny>,
            values: Option<&pyo3::types::PyAny>,
            theme: Option<&str>,
            kwargs: Option<&pyo3::types::PyDict>,
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
                        let v = py_any_val(val);
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
                    labels: Option<&pyo3::types::PyAny>,
                    values: Option<&pyo3::types::PyAny>,
                    theme: Option<&str>,
                    kwargs: Option<&pyo3::types::PyDict>,
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
        crate::for_each_json_chart_fn!(impl_python);

        macro_rules! impl_python_json {
            ($fn:ident, $_js:literal) => {
                #[pyfunction]
                pub fn $fn(input: &str) -> String {
                    crate::bindings::commands::charts::$fn(input)
                }
            };
        }
        crate::for_each_ml_oneshot_fn!(impl_python_json);

        #[pyfunction]
        #[pyo3(name = "set_bg", signature = (html, color=None))]
        pub fn set_bg_fn(html: &PyAny, color: Option<&str>) -> PyResult<crate::Chart> {
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
        pub struct DbscanModel(crate::bindings::commands::charts::DbscanState);

        #[pymethods]
        impl DbscanModel {
            #[new]
            #[pyo3(signature = (eps=0.5, min_samples=5))]
            pub fn py_new(eps: f64, min_samples: usize) -> Self {
                DbscanModel(crate::bindings::commands::charts::DbscanState::new(eps, min_samples))
            }
            #[pyo3(signature = (x))]
            pub fn fit(&mut self, x: &PyAny) -> PyResult<()> {
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
            pub fn fit_predict(&mut self, py: Python<'_>, x: &PyAny) -> PyResult<pyo3::PyObject> {
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
        pub struct KMeansModel(crate::bindings::commands::charts::KMeansState);

        #[pymethods]
        impl KMeansModel {
            #[new]
            #[pyo3(signature = (k=3, max_iter=300, tol=1e-4, mini_batch=false, batch_size=1000, n_init=10, n_clusters=None, random_state=None))]
            pub fn py_new(k: usize, max_iter: usize, tol: f64, mini_batch: bool, batch_size: usize, n_init: usize, n_clusters: Option<usize>, random_state: Option<i64>) -> Self {
                let _ = random_state;
                let kk = n_clusters.unwrap_or(k);
                KMeansModel(crate::bindings::commands::charts::KMeansState::new(kk, max_iter, tol, mini_batch, batch_size, n_init))
            }
            #[pyo3(signature = (x))]
            pub fn fit(&mut self, x: &PyAny) -> PyResult<()> {
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
            pub fn fit_predict(&mut self, py: Python<'_>, x: &PyAny) -> PyResult<pyo3::PyObject> {
                self.fit(x)?;
                Ok(crate::bindings::commands::ml::vec_i32_to_np(py, self.0.labels.clone()))
            }
            #[pyo3(signature = (x))]
            pub fn transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<pyo3::PyObject> {
                let (flat, n, dims) = super::ml::extract_flat(x)?;
                crate::bindings::commands::ml::vv_to_np2d(py, self.0.transform_flat(&flat, n, dims))
            }
            #[pyo3(signature = (x))]
            pub fn predict(&self, py: Python<'_>, x: &PyAny) -> PyResult<pyo3::PyObject> {
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
            x: &PyAny,
            y: Option<&PyAny>,
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
                    let xv = crate::py_to_f64_vec(py, x.get_item(&cols[0])?)?;
                    let yv = crate::py_to_f64_vec(py, x.get_item(&cols[1])?)?;
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
            let xv = crate::py_to_f64_vec(py, x)?;
            let yv: Option<Vec<f64>> = if let Some(yobj) = y { Some(crate::py_to_f64_vec(py, yobj)?) } else { None };
            let res = Ok(crate::Chart::new_doc(
                crate::bindings::commands::charts::plot_chart(
                    &serde_json::json!({"x":xv,"y":yv,"kind":kind,"title":title,"color_hex":color_hex,"show_points":show_points,"width":width,"height":height,"x_label":x_label,"y_label":y_label,"gridlines":grid,"palette":pal_opt,"background":bg}).to_string()
                ), "",
            ));
            crate::telemetry::record(crate::telemetry::TelemetryEvent::new("plot", t.elapsed().as_secs_f64() * 1000.0).with_data(xv.len() as u64, 0.0));
            res
        }

        #[pyfunction]
        #[pyo3(signature = (chart, path))]
        pub fn savefig(chart: &crate::Chart, path: &str) -> PyResult<()> {
            std::fs::write(path, &chart.html).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
        }

        #[pyfunction]
        pub fn export_svg(chart: &crate::Chart) -> String {
            crate::bindings::commands::charts::export_svg(
                &serde_json::json!({"html": chart.html}).to_string()
            )
        }

        #[pyfunction]
        pub fn export_data_url(chart: &crate::Chart) -> String {
            crate::bindings::commands::charts::export_data_url(
                &serde_json::json!({"html": chart.html}).to_string()
            )
        }

        #[pyfunction]
        #[pyo3(signature = (chart, path))]
        pub fn export_png(chart: &crate::Chart, path: &str) -> PyResult<()> {
            let svg = crate::bindings::commands::charts::export_svg(
                &serde_json::json!({"html": chart.html}).to_string()
            );
            if svg.is_empty() {
                return Err(pyo3::exceptions::PyValueError::new_err("no <svg> in chart html"));
            }
            let p = std::path::Path::new(path);
            let target = if matches!(p.extension().and_then(|e| e.to_str()), Some("svg")) {
                path.to_string()
            } else {
                let mut q = std::path::PathBuf::from(path);
                q.set_extension("svg");
                q.to_string_lossy().to_string()
            };
            std::fs::write(&target, svg).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
        }

        #[pyfunction]
        pub fn chart_info(chart: &crate::Chart) -> String {
            crate::bindings::commands::charts::chart_info(
                &serde_json::json!({"html": chart.html}).to_string()
            )
        }

        #[pyclass(module = "seraplot", name = "LiveStream")]
        pub struct LiveStream {
            kind: String,
            xs: Vec<f64>,
            ys: Vec<f64>,
            title: String,
            max_points: usize,
            color_hex: u32,
            width: i32,
            height: i32,
        }

        #[pymethods]
        impl LiveStream {
            #[new]
            #[pyo3(signature = (kind="line", title="", max_points=500, color_hex=0x6366F1_u32, width=900, height=420))]
            pub fn py_new(kind: &str, title: &str, max_points: usize, color_hex: u32, width: i32, height: i32) -> Self {
                Self { kind: kind.to_string(), xs: Vec::new(), ys: Vec::new(), title: title.to_string(), max_points, color_hex, width, height }
            }
            #[pyo3(signature = (x, y))]
            pub fn push(&mut self, x: f64, y: f64) {
                self.xs.push(x); self.ys.push(y);
                if self.xs.len() > self.max_points { let cut = self.xs.len() - self.max_points; self.xs.drain(..cut); }
                if self.ys.len() > self.max_points { let cut = self.ys.len() - self.max_points; self.ys.drain(..cut); }
            }
            #[pyo3(signature = (xs, ys))]
            pub fn extend(&mut self, xs: Vec<f64>, ys: Vec<f64>) {
                self.xs.extend(xs); self.ys.extend(ys);
                if self.xs.len() > self.max_points { let cut = self.xs.len() - self.max_points; self.xs.drain(..cut); }
                if self.ys.len() > self.max_points { let cut = self.ys.len() - self.max_points; self.ys.drain(..cut); }
            }
            pub fn render(&self) -> crate::Chart {
                let payload = serde_json::json!({
                    "kind": self.kind, "title": self.title, "color_hex": self.color_hex,
                    "width": self.width, "height": self.height,
                    "prev_x": self.xs, "prev_y": self.ys
                });
                let out = crate::bindings::commands::charts::chart_append(&payload.to_string());
                let v: serde_json::Value = serde_json::from_str(&out).unwrap_or(serde_json::Value::Null);
                let html = v.get("html").and_then(|h| h.as_str()).unwrap_or("").to_string();
                crate::Chart::new(html)
            }
            pub fn clear(&mut self) { self.xs.clear(); self.ys.clear(); }
            #[getter] pub fn n(&self) -> usize { self.xs.len() }
        }

        #[pyfunction]
        #[pyo3(signature = (charts, cols=2, gap=16, bg=None, cell_height=520))]
        pub fn facet(
            charts: Vec<pyo3::PyRef<crate::Chart>>,
            cols: usize,
            gap: i32,
            bg: Option<&str>,
            cell_height: i32,
        ) -> crate::Chart {
            let html_strs: Vec<String> = charts.iter().map(|c| c.html.clone()).collect();
            crate::Chart::new(
                crate::bindings::commands::charts::build_grid(
                    &serde_json::json!({"charts": html_strs, "cols": cols, "gap": gap, "background": bg, "cell_height": cell_height}).to_string()
                )
            )
        }

        #[pyfunction]
        #[pyo3(signature = (reference, current))]
        pub fn drift(reference: Vec<f64>, current: Vec<f64>) -> String {
            crate::bindings::commands::charts::drift_ks(
                &serde_json::json!({"reference": reference, "current": current}).to_string()
            )
        }

        #[pyfunction]
        #[pyo3(signature = (input, threshold=2000))]
        pub fn lttb(input: Vec<(f64, f64)>, threshold: usize) -> Vec<(f64, f64)> {
            let xs: Vec<f64> = input.iter().map(|p| p.0).collect();
            let ys: Vec<f64> = input.iter().map(|p| p.1).collect();
            let res = crate::bindings::commands::charts::downsample_lttb(
                &serde_json::json!({"x": xs, "y": ys, "threshold": threshold}).to_string()
            );
            let v: serde_json::Value = match serde_json::from_str(&res) { Ok(v) => v, Err(_) => return input };
            let nx: Vec<f64> = v.get("x").and_then(|a| a.as_array()).map(|a| a.iter().filter_map(|n| n.as_f64()).collect()).unwrap_or_default();
            let ny: Vec<f64> = v.get("y").and_then(|a| a.as_array()).map(|a| a.iter().filter_map(|n| n.as_f64()).collect()).unwrap_or_default();
            nx.into_iter().zip(ny).collect()
        }

        #[pyfunction]
        pub fn show(py: Python<'_>, chart: &crate::Chart) -> PyResult<()> {
            chart.show(py)
        }

        #[pyfunction]
        #[pyo3(signature = (x, y, models=None))]
        pub fn auto_classify(py: Python<'_>, x: &PyAny, y: Vec<i64>, models: Option<Vec<String>>) -> PyResult<pyo3::Py<pyo3::types::PyDict>> {
            let (flat, n, dims) = crate::bindings::commands::ml::extract_flat(x)?;
            let chosen = models.unwrap_or_else(|| vec![
                "knn".to_string(),
                "decision_tree".to_string(),
                "gradient_boosting".to_string(),
            ]);
            let mut leaderboard: Vec<(String, f64)> = Vec::new();
            let flat2d: Vec<Vec<f64>> = flat.chunks(dims.max(1)).map(|c| c.to_vec()).collect();
            for name in chosen.iter() {
                let payload = serde_json::json!({"x_train": &flat2d, "x_test": &flat2d, "y_train": &y, "n": n, "dims": dims, "y": y}).to_string();
                let payload_for_call = payload.clone();
                let name_owned = name.clone();
                let raw = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| match name_owned.as_str() {
                    "logistic" => crate::bindings::commands::charts::ml_logistic_regression(&payload_for_call),
                    "knn" => crate::bindings::commands::charts::ml_knn_classifier(&payload_for_call),
                    "decision_tree" => crate::bindings::commands::charts::ml_decision_tree_classifier(&payload_for_call),
                    "random_forest" => crate::bindings::commands::charts::ml_random_forest_classifier(&payload_for_call),
                    "gradient_boosting" => crate::bindings::commands::charts::ml_gradient_boosting_classifier(&payload_for_call),
                    _ => String::new(),
                }));
                let raw = match raw {
                    Ok(s) if !s.is_empty() => s,
                    _ => { leaderboard.push((name.clone(), f64::NAN)); continue; }
                };
                let score = serde_json::from_str::<serde_json::Value>(&raw).ok()
                    .and_then(|v| {
                        if let Some(s) = v.get("score").and_then(|s| s.as_f64()) { return Some(s); }
                        if let Some(s) = v.get("accuracy").and_then(|s| s.as_f64()) { return Some(s); }
                        if let Some(preds) = v.get("predictions").and_then(|p| p.as_array()) {
                            if preds.len() != y.len() { return Some(f64::NAN); }
                            let mut hits = 0usize;
                            for (i, p) in preds.iter().enumerate() {
                                if let Some(pi) = p.as_i64() { if pi == y[i] { hits += 1; } }
                            }
                            return Some(hits as f64 / y.len() as f64);
                        }
                        None
                    })
                    .unwrap_or(f64::NAN);
                leaderboard.push((name.clone(), score));
            }
            leaderboard.sort_by(|a, b| {
                let an = a.1.is_nan(); let bn = b.1.is_nan();
                match (an, bn) {
                    (true, true) => std::cmp::Ordering::Equal,
                    (true, false) => std::cmp::Ordering::Greater,
                    (false, true) => std::cmp::Ordering::Less,
                    _ => b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal),
                }
            });
            let dict = pyo3::types::PyDict::new(py);
            let lb = pyo3::types::PyList::empty(py);
            for (name, score) in leaderboard.iter() {
                let row = pyo3::types::PyDict::new(py);
                row.set_item("model", name)?;
                row.set_item("score", score)?;
                lb.append(row)?;
            }
            dict.set_item("leaderboard", lb)?;
            if let Some((best_name, best_score)) = leaderboard.first() {
                dict.set_item("best_model", best_name)?;
                dict.set_item("best_score", best_score)?;
            }
            Ok(dict.into())
        }

        #[pyclass(module = "seraplot", name = "Pipeline")]
        pub struct Pipeline {
            steps: Vec<(String, pyo3::PyObject)>,
        }

        #[pymethods]
        impl Pipeline {
            #[new]
            #[pyo3(signature = (steps))]
            pub fn py_new(steps: Vec<(String, pyo3::PyObject)>) -> Self {
                Self { steps }
            }
            pub fn fit(&mut self, py: Python<'_>, x: &PyAny, y: Option<&PyAny>) -> PyResult<()> {
                let mut current: pyo3::PyObject = x.into();
                for (i, (_, step)) in self.steps.iter().enumerate() {
                    let st = step.as_ref(py);
                    let is_last = i == self.steps.len() - 1;
                    if is_last {
                        if let Some(yy) = y {
                            if st.call_method1("fit", (current.as_ref(py), yy)).is_err() {
                                let _ = st.call_method1("fit", (current.as_ref(py),));
                            }
                        } else {
                            let _ = st.call_method1("fit", (current.as_ref(py),));
                        }
                    } else {
                        let _ = st.call_method1("fit", (current.as_ref(py),));
                        if let Ok(t) = st.call_method1("transform", (current.as_ref(py),)) {
                            current = t.into();
                        }
                    }
                }
                Ok(())
            }
            pub fn predict(&self, py: Python<'_>, x: &PyAny) -> PyResult<pyo3::PyObject> {
                let mut current: pyo3::PyObject = x.into();
                for (i, (_, step)) in self.steps.iter().enumerate() {
                    let st = step.as_ref(py);
                    let is_last = i == self.steps.len() - 1;
                    if is_last {
                        return Ok(st.call_method1("predict", (current.as_ref(py),))?.into());
                    }
                    if let Ok(t) = st.call_method1("transform", (current.as_ref(py),)) {
                        current = t.into();
                    }
                }
                Ok(current)
            }
            pub fn transform(&self, py: Python<'_>, x: &PyAny) -> PyResult<pyo3::PyObject> {
                let mut current: pyo3::PyObject = x.into();
                for (_, step) in self.steps.iter() {
                    let st = step.as_ref(py);
                    if let Ok(t) = st.call_method1("transform", (current.as_ref(py),)) {
                        current = t.into();
                    }
                }
                Ok(current)
            }
            pub fn predict_proba(&self, py: Python<'_>, x: &PyAny) -> PyResult<pyo3::PyObject> {
                let mut current: pyo3::PyObject = x.into();
                for (i, (_, step)) in self.steps.iter().enumerate() {
                    let st = step.as_ref(py);
                    let is_last = i == self.steps.len() - 1;
                    if is_last {
                        return Ok(st.call_method1("predict_proba", (current.as_ref(py),))?.into());
                    }
                    if let Ok(t) = st.call_method1("transform", (current.as_ref(py),)) {
                        current = t.into();
                    }
                }
                Ok(current)
            }
            pub fn decision_function(&self, py: Python<'_>, x: &PyAny) -> PyResult<pyo3::PyObject> {
                let mut current: pyo3::PyObject = x.into();
                for (i, (_, step)) in self.steps.iter().enumerate() {
                    let st = step.as_ref(py);
                    let is_last = i == self.steps.len() - 1;
                    if is_last {
                        return Ok(st.call_method1("decision_function", (current.as_ref(py),))?.into());
                    }
                    if let Ok(t) = st.call_method1("transform", (current.as_ref(py),)) {
                        current = t.into();
                    }
                }
                Ok(current)
            }
            pub fn score(&self, py: Python<'_>, x: &PyAny, y: &PyAny) -> PyResult<f64> {
                let mut current: pyo3::PyObject = x.into();
                for (i, (_, step)) in self.steps.iter().enumerate() {
                    let st = step.as_ref(py);
                    let is_last = i == self.steps.len() - 1;
                    if is_last {
                        if let Ok(s) = st.call_method1("score", (current.as_ref(py), y)) {
                            return s.extract::<f64>();
                        }
                        let pred: pyo3::PyObject = st.call_method1("predict", (current.as_ref(py),))?.into();
                        let yv: Vec<f64> = y.extract().unwrap_or_default();
                        let pv: Vec<f64> = pred.as_ref(py).extract().unwrap_or_default();
                        if yv.is_empty() || pv.is_empty() || yv.len() != pv.len() { return Ok(0.0); }
                        let n = yv.len();
                        let any_non_int = yv.iter().any(|v| (v - v.round()).abs() > 1e-9);
                        if any_non_int {
                            let ymean: f64 = yv.iter().sum::<f64>() / n as f64;
                            let ss_tot: f64 = yv.iter().map(|v| (v - ymean).powi(2)).sum();
                            let ss_res: f64 = yv.iter().zip(pv.iter()).map(|(a, b)| (a - b).powi(2)).sum();
                            return Ok(if ss_tot > 0.0 { 1.0 - ss_res / ss_tot } else { 0.0 });
                        }
                        let correct: usize = yv.iter().zip(pv.iter()).filter(|(a, b)| ((**a) - (**b)).abs() < 1e-9).count();
                        return Ok(correct as f64 / n as f64);
                    }
                    if let Ok(t) = st.call_method1("transform", (current.as_ref(py),)) {
                        current = t.into();
                    }
                }
                Ok(0.0)
            }
            pub fn fit_predict(&mut self, py: Python<'_>, x: &PyAny, y: Option<&PyAny>) -> PyResult<pyo3::PyObject> {
                self.fit(py, x, y)?;
                self.predict(py, x)
            }
            pub fn fit_transform(&mut self, py: Python<'_>, x: &PyAny, y: Option<&PyAny>) -> PyResult<pyo3::PyObject> {
                self.fit(py, x, y)?;
                self.transform(py, x)
            }
            #[getter] pub fn step_names(&self) -> Vec<String> { self.steps.iter().map(|(n, _)| n.clone()).collect() }
            fn __repr__(&self) -> String {
                format!("Pipeline(steps={:?})", self.steps.iter().map(|(n, _)| n.clone()).collect::<Vec<_>>())
            }
        }
    };
}

#[macro_export]
macro_rules! for_each_chart_params {
    ($mac:ident) => {[
        $mac!("bar", "basic",            r#"labels=["Alpha","Beta","Gamma","Delta","Epsilon"], values=[24,38,17,42,29]"#),
        $mac!("bar", "deluxe",           r#"labels=["Alpha","Beta","Gamma","Delta","Epsilon"], values=[24,38,17,42,29]"#),
        $mac!("bar", "grouped",          r#"labels=["Q1","Q2","Q3","Q4"], series=[[24,38,17,42],[18,29,33,21],[12,15,28,30]], series_names=["Product A","Product B","Product C"]"#),
        $mac!("bar", "grouped_stacked",  r#"labels=["Q1","Q2","Q3","Q4"], series=[[24,38,17,42],[18,29,33,21],[12,15,28,30],[20,25,18,22]], series_names=["2023 A","2023 B","2024 A","2024 B"], offset_groups=["2023","2023","2024","2024"]"#),
        $mac!("bar", "marimekko",        r#"labels=["North","South","East","West"], series=[[42,28,33,17],[18,38,22,30],[24,15,28,20]], series_names=["Product A","Product B","Product C"], widths=[2.0,1.5,1.0,1.2]"#),
        $mac!("bar", "multicategory",    r#"labels=["Q1","Q2","Q3","Q4"], series=[[24,38,17,42],[18,29,33,21]], series_names=["2023","2024"], super_categories=["H1","H1","H2","H2"]"#),
        $mac!("bar", "pictogram",        r#"labels=["Bikes","Cars","Buses","Trains"], values=[24,38,17,42], unit_description="units", units_per_icon=2.0, icon_size=24, max_icons_per_column=10"#),
        $mac!("bar", "prism",            r#"labels=["Alpha","Beta","Gamma","Delta","Epsilon"], values=[24,38,17,42,29]"#),
        $mac!("bar", "relative",         r#"labels=["Q1","Q2","Q3","Q4"], series=[[24,38,17,42],[18,29,33,21],[12,15,28,30]], series_names=["Revenue","Cost","Tax"]"#),
        $mac!("boxplot", "basic",        r#"labels=["A","B","C"], series=[[1.2,2.4,2.7,3.1,3.5,3.8,4.2,5.1,6.0],[2.0,2.8,3.2,3.6,4.1,4.5,5.0,5.7,6.5],[1.8,2.2,2.6,3.0,3.4,3.9,4.3,4.9,5.5]]"#),
        $mac!("boxplot", "grouped",      r#"labels=["A","B","C"], series=[[1.2,2.4,2.7,3.1,3.5,3.8,4.2,5.1,6.0],[2.0,2.8,3.2,3.6,4.1,4.5,5.0,5.7,6.5],[1.8,2.2,2.6,3.0,3.4,3.9,4.3,4.9,5.5]]"#),
        $mac!("boxplot", "horizontal",   r#"labels=["A","B","C"], series=[[1.2,2.4,2.7,3.1,3.5,3.8,4.2,5.1,6.0],[2.0,2.8,3.2,3.6,4.1,4.5,5.0,5.7,6.5],[1.8,2.2,2.6,3.0,3.4,3.9,4.3,4.9,5.5]]"#),
        $mac!("boxplot", "letter_value", r#"labels=["A","B","C"], series=[[1.2,2.4,2.7,3.1,3.5,3.8,4.2,5.1,6.0],[2.0,2.8,3.2,3.6,4.1,4.5,5.0,5.7,6.5],[1.8,2.2,2.6,3.0,3.4,3.9,4.3,4.9,5.5]]"#),
        $mac!("boxplot", "notched",      r#"labels=["A","B","C"], series=[[1.2,2.4,2.7,3.1,3.5,3.8,4.2,5.1,6.0],[2.0,2.8,3.2,3.6,4.1,4.5,5.0,5.7,6.5],[1.8,2.2,2.6,3.0,3.4,3.9,4.3,4.9,5.5]]"#),
        $mac!("boxplot", "outliers",     r#"labels=["A","B","C"], series=[[1.2,2.4,2.7,3.1,3.5,3.8,4.2,5.1,6.0],[2.0,2.8,3.2,3.6,4.1,4.5,5.0,5.7,6.5],[1.8,2.2,2.6,3.0,3.4,3.9,4.3,4.9,5.5]]"#),
        $mac!("boxplot", "points",       r#"labels=["A","B","C"], series=[[1.2,2.4,2.7,3.1,3.5,3.8,4.2,5.1,6.0],[2.0,2.8,3.2,3.6,4.1,4.5,5.0,5.7,6.5],[1.8,2.2,2.6,3.0,3.4,3.9,4.3,4.9,5.5]]"#),
        $mac!("boxplot", "rainbow",      r#"labels=["A","B","C"], series=[[1.2,2.4,2.7,3.1,3.5,3.8,4.2,5.1,6.0],[2.0,2.8,3.2,3.6,4.1,4.5,5.0,5.7,6.5],[1.8,2.2,2.6,3.0,3.4,3.9,4.3,4.9,5.5]]"#),
        $mac!("boxplot", "strip",        r#"labels=["A","B","C"], series=[[1.2,2.4,2.7,3.1,3.5,3.8,4.2,5.1,6.0],[2.0,2.8,3.2,3.6,4.1,4.5,5.0,5.7,6.5],[1.8,2.2,2.6,3.0,3.4,3.9,4.3,4.9,5.5]]"#),
        $mac!("boxplot", "violin",       r#"labels=["A","B","C"], series=[[1.2,2.4,2.7,3.1,3.5,3.8,4.2,5.1,6.0],[2.0,2.8,3.2,3.6,4.1,4.5,5.0,5.7,6.5],[1.8,2.2,2.6,3.0,3.4,3.9,4.3,4.9,5.5]]"#),
        $mac!("bubble", "basic",         r#"x=[1,2,3,4,5,6,7], y=[3,5,2,7,6,8,4], sizes=[20,40,15,55,30,45,25], labels=["A","B","C","D","E","F","G"]"#),
        $mac!("bubble", "categorical",   r#"x=[1,2,3,4,5,6,7], y=[3,5,2,7,6,8,4], sizes=[20,40,15,55,30,45,25], labels=["A","B","C","D","E","F","G"], categories=["X","Y","X","Y","Z","X","Z"]"#),
        $mac!("bubble", "deluxe",        r#"x=[1,2,3,4,5,6,7], y=[3,5,2,7,6,8,4], sizes=[20,40,15,55,30,45,25], labels=["A","B","C","D","E","F","G"]"#),
        $mac!("bubble", "gradient",      r#"x=[1,2,3,4,5,6,7], y=[3,5,2,7,6,8,4], sizes=[20,40,15,55,30,45,25], values=[1,2,3,4,5,6,7]"#),
        $mac!("bubble", "labeled",       r#"x=[1,2,3,4,5,6,7], y=[3,5,2,7,6,8,4], sizes=[20,40,15,55,30,45,25], labels=["A","B","C","D","E","F","G"]"#),
        $mac!("bubble", "negative",      r#"x=[1,2,3,4,5,6,7], y=[3,5,2,7,6,8,4], sizes=[20,40,15,55,30,45,25], labels=["A","B","C","D","E","F","G"]"#),
        $mac!("bubble", "outlined",      r#"x=[1,2,3,4,5,6,7], y=[3,5,2,7,6,8,4], sizes=[20,40,15,55,30,45,25], labels=["A","B","C","D","E","F","G"]"#),
        $mac!("bubble", "plasma",        r#"x=[1,2,3,4,5,6,7], y=[3,5,2,7,6,8,4], sizes=[20,40,15,55,30,45,25], labels=["A","B","C","D","E","F","G"]"#),
        $mac!("bullet", "basic",         r#"labels=["Revenue","Profit","CSAT"], values=[80,65,4.2], targets=[90,70,4.5], max_vals=[120,100,5]"#),
        $mac!("bullet", "compare",       r#"labels=["Revenue","Profit","CSAT"], values=[80,65,4.2], targets=[90,70,4.5], max_vals=[120,100,5], comparisons=[70,55,3.8]"#),
        $mac!("bullet", "dot",           r#"labels=["Revenue","Profit","CSAT"], values=[80,65,4.2], targets=[90,70,4.5], max_vals=[120,100,5]"#),
        $mac!("bullet", "minimal",       r#"labels=["Revenue","Profit","CSAT"], values=[80,65,4.2], targets=[90,70,4.5], max_vals=[120,100,5]"#),
        $mac!("bullet", "progress",      r#"labels=["Revenue","Profit","CSAT"], values=[80,65,4.2], targets=[90,70,4.5], max_vals=[120,100,5]"#),
        $mac!("bullet", "segmented",     r#"labels=["Revenue","Profit","CSAT"], values=[80,65,4.2], targets=[90,70,4.5], max_vals=[120,100,5], ranges=[[40,70,100],[30,60,90],[2,3,4]]"#),
        $mac!("bullet", "stacked",       r#"labels=["Revenue","Profit","CSAT"], values=[80,65,4.2], targets=[90,70,4.5], max_vals=[120,100,5], ranges=[[40,70,100],[30,60,90],[2,3,4]]"#),
        $mac!("bullet", "thermo",        r#"labels=["Revenue","Profit","CSAT"], values=[80,65,4.2], targets=[90,70,4.5], max_vals=[120,100,5]"#),
        $mac!("candlestick", "basic",    r#"labels=["Mon","Tue","Wed","Thu","Fri"], open=[100,102,105,103,108], high=[105,107,109,110,114], low=[99,101,103,102,107], close=[102,105,103,108,112]"#),
        $mac!("candlestick", "heikin",   r#"labels=["Mon","Tue","Wed","Thu","Fri"], open=[100,102,105,103,108], high=[105,107,109,110,114], low=[99,101,103,102,107], close=[102,105,103,108,112]"#),
        $mac!("candlestick", "hollow",   r#"labels=["Mon","Tue","Wed","Thu","Fri"], open=[100,102,105,103,108], high=[105,107,109,110,114], low=[99,101,103,102,107], close=[102,105,103,108,112]"#),
        $mac!("candlestick", "line",     r#"labels=["Mon","Tue","Wed","Thu","Fri"], open=[100,102,105,103,108], high=[105,107,109,110,114], low=[99,101,103,102,107], close=[102,105,103,108,112]"#),
        $mac!("candlestick", "mountain", r#"labels=["Mon","Tue","Wed","Thu","Fri"], open=[100,102,105,103,108], high=[105,107,109,110,114], low=[99,101,103,102,107], close=[102,105,103,108,112]"#),
        $mac!("candlestick", "ohlc",     r#"labels=["Mon","Tue","Wed","Thu","Fri"], open=[100,102,105,103,108], high=[105,107,109,110,114], low=[99,101,103,102,107], close=[102,105,103,108,112]"#),
        $mac!("candlestick", "outlined", r#"labels=["Mon","Tue","Wed","Thu","Fri"], open=[100,102,105,103,108], high=[105,107,109,110,114], low=[99,101,103,102,107], close=[102,105,103,108,112]"#),
        $mac!("candlestick", "range",    r#"labels=["Mon","Tue","Wed","Thu","Fri"], open=[100,102,105,103,108], high=[105,107,109,110,114], low=[99,101,103,102,107], close=[102,105,103,108,112]"#),
        $mac!("dumbbell", "arrow",       r#"labels=["A","B","C","D","E"], start=[20,35,15,42,28], end=[60,52,38,68,55]"#),
        $mac!("dumbbell", "barbell",     r#"labels=["A","B","C","D","E"], start=[20,35,15,42,28], end=[60,52,38,68,55]"#),
        $mac!("dumbbell", "basic",       r#"labels=["A","B","C","D","E"], start=[20,35,15,42,28], end=[60,52,38,68,55]"#),
        $mac!("dumbbell", "delta",       r#"labels=["A","B","C","D","E"], start=[20,35,15,42,28], end=[60,52,38,68,55]"#),
        $mac!("dumbbell", "dotted",      r#"labels=["A","B","C","D","E"], start=[20,35,15,42,28], end=[60,52,38,68,55]"#),
        $mac!("dumbbell", "glow",        r#"labels=["A","B","C","D","E"], start=[20,35,15,42,28], end=[60,52,38,68,55]"#),
        $mac!("dumbbell", "ranked",      r#"labels=["A","B","C","D","E"], start=[20,35,15,42,28], end=[60,52,38,68,55]"#),
        $mac!("funnel", "basic",         r#"labels=["Visits","Signups","Trial","Paid","Renewed"], values=[1000,520,210,85,40]"#),
        $mac!("funnel", "chevron",       r#"labels=["Visits","Signups","Trial","Paid","Renewed"], values=[1000,520,210,85,40]"#),
        $mac!("funnel", "conversion",    r#"labels=["Visits","Signups","Trial","Paid","Renewed"], values=[1000,520,210,85,40]"#),
        $mac!("funnel", "gradient",      r#"labels=["Visits","Signups","Trial","Paid","Renewed"], values=[1000,520,210,85,40]"#),
        $mac!("funnel", "inverted",      r#"labels=["Visits","Signups","Trial","Paid","Renewed"], values=[1000,520,210,85,40]"#),
        $mac!("funnel", "pyramid",       r#"labels=["Visits","Signups","Trial","Paid","Renewed"], values=[1000,520,210,85,40]"#),
        $mac!("funnel", "rounded",       r#"labels=["Visits","Signups","Trial","Paid","Renewed"], values=[1000,520,210,85,40]"#),
        $mac!("funnel", "stepped",       r#"labels=["Visits","Signups","Trial","Paid","Renewed"], values=[1000,520,210,85,40]"#),
        $mac!("gauge", "arc270",         r#"value=72, min_val=0, max_val=100, label="Score""#),
        $mac!("gauge", "basic",          r#"value=72, min_val=0, max_val=100, label="Score""#),
        $mac!("gauge", "concentric",     r#"value=72, min_val=0, max_val=100, label="Score""#),
        $mac!("gauge", "glow",           r#"value=72, min_val=0, max_val=100, label="Score""#),
        $mac!("gauge", "radial",         r#"value=72, min_val=0, max_val=100, label="Score""#),
        $mac!("gauge", "segmented",      r#"value=72, min_val=0, max_val=100, label="Score""#),
        $mac!("gauge", "sleek",          r#"value=72, min_val=0, max_val=100, label="Score""#),
        $mac!("gauge", "tick",           r#"value=72, min_val=0, max_val=100, label="Score""#),
        $mac!("heatmap", "annotated",    r#"labels=["Mon","Tue","Wed","Thu","Fri"], col_labels=["8h","12h","16h","20h"], values=[5,9,7,3,6,12,10,4,8,15,13,7,4,8,11,5,3,7,9,2]"#),
        $mac!("heatmap", "basic",        r#"labels=["Mon","Tue","Wed","Thu","Fri"], col_labels=["8h","12h","16h","20h"], values=[5,9,7,3,6,12,10,4,8,15,13,7,4,8,11,5,3,7,9,2]"#),
        $mac!("heatmap", "bubble",       r#"labels=["Mon","Tue","Wed","Thu","Fri"], col_labels=["8h","12h","16h","20h"], values=[5,9,7,3,6,12,10,4,8,15,13,7,4,8,11,5,3,7,9,2]"#),
        $mac!("heatmap", "categorical",  r#"labels=["Mon","Tue","Wed","Thu","Fri"], col_labels=["8h","12h","16h","20h"], values=[5,9,7,3,6,12,10,4,8,15,13,7,4,8,11,5,3,7,9,2]"#),
        $mac!("heatmap", "cluster",      r#"labels=["Mon","Tue","Wed","Thu","Fri"], col_labels=["8h","12h","16h","20h"], values=[5,9,7,3,6,12,10,4,8,15,13,7,4,8,11,5,3,7,9,2]"#),
        $mac!("heatmap", "confusion",    r#"labels=["Mon","Tue","Wed","Thu","Fri"], col_labels=["8h","12h","16h","20h"], values=[5,9,7,3,6,12,10,4,8,15,13,7,4,8,11,5,3,7,9,2]"#),
        $mac!("heatmap", "contour",      r#"labels=["Mon","Tue","Wed","Thu","Fri"], col_labels=["8h","12h","16h","20h"], values=[5,9,7,3,6,12,10,4,8,15,13,7,4,8,11,5,3,7,9,2]"#),
        $mac!("heatmap", "correlation",  r#"labels=["Mon","Tue","Wed","Thu","Fri"], col_labels=["8h","12h","16h","20h"], values=[5,9,7,3,6,12,10,4,8,15,13,7,4,8,11,5,3,7,9,2]"#),
        $mac!("heatmap", "density",      r#"labels=["Mon","Tue","Wed","Thu","Fri"], col_labels=["8h","12h","16h","20h"], values=[5,9,7,3,6,12,10,4,8,15,13,7,4,8,11,5,3,7,9,2]"#),
        $mac!("heatmap", "discrete",     r#"labels=["Mon","Tue","Wed","Thu","Fri"], col_labels=["8h","12h","16h","20h"], values=[5,9,7,3,6,12,10,4,8,15,13,7,4,8,11,5,3,7,9,2]"#),
        $mac!("heatmap", "log",          r#"labels=["Mon","Tue","Wed","Thu","Fri"], col_labels=["8h","12h","16h","20h"], values=[5,9,7,3,6,12,10,4,8,15,13,7,4,8,11,5,3,7,9,2]"#),
        $mac!("heatmap", "marginal",     r#"labels=["Mon","Tue","Wed","Thu","Fri"], col_labels=["8h","12h","16h","20h"], values=[5,9,7,3,6,12,10,4,8,15,13,7,4,8,11,5,3,7,9,2]"#),
        $mac!("heatmap", "pivot",        r#"labels=["Mon","Tue","Wed","Thu","Fri"], col_labels=["8h","12h","16h","20h"], values=[5,9,7,3,6,12,10,4,8,15,13,7,4,8,11,5,3,7,9,2]"#),
        $mac!("heatmap", "temporal",     r#"labels=["Mon","Tue","Wed","Thu","Fri"], col_labels=["8h","12h","16h","20h"], values=[5,9,7,3,6,12,10,4,8,15,13,7,4,8,11,5,3,7,9,2]"#),
        $mac!("heatmap", "unequal",      r#"labels=["Mon","Tue","Wed","Thu","Fri"], col_labels=["8h","12h","16h","20h"], values=[5,9,7,3,6,12,10,4,8,15,13,7,4,8,11,5,3,7,9,2]"#),
        $mac!("histogram", "basic",      r#"values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]"#),
        $mac!("histogram", "cumulative", r#"values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]"#),
        $mac!("histogram", "deluxe",     r#"values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]"#),
        $mac!("histogram", "horizontal", r#"values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]"#),
        $mac!("histogram", "normalized", r#"values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]"#),
        $mac!("histogram", "overlay",    r#"values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]"#),
        $mac!("histogram", "stacked",    r#"values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]"#),
        $mac!("histogram", "step",       r#"values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]"#),
        $mac!("kde", "basic",            r#"values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]"#),
        $mac!("kde", "cumulative",       r#"values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]"#),
        $mac!("kde", "gradient",         r#"values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]"#),
        $mac!("kde", "histogram",        r#"values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]"#),
        $mac!("kde", "normalized",       r#"values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]"#),
        $mac!("kde", "outline",          r#"values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]"#),
        $mac!("kde", "rug",              r#"values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]"#),
        $mac!("kde", "stepped",          r#"values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]"#),
        $mac!("line", "basic",           r#"x_labels=["Jan","Feb","Mar","Apr","May","Jun"], values=[12,19,15,22,28,24]"#),
        $mac!("line", "connected_scatter", r#"x_labels=["Jan","Feb","Mar","Apr","May","Jun"], series=[[12,19,15,22,28,24],[8,14,18,16,22,20]], series_names=["A","B"]"#),
        $mac!("line", "dashed",          r#"x_labels=["Jan","Feb","Mar","Apr","May","Jun"], values=[12,19,15,22,28,24]"#),
        $mac!("line", "filled",          r#"x_labels=["Jan","Feb","Mar","Apr","May","Jun"], values=[12,19,15,22,28,24]"#),
        $mac!("line", "gapped",          r#"x_labels=["Jan","Feb","Mar","Apr","May","Jun"], values=[12,19,15,22,28,24]"#),
        $mac!("line", "multi",           r#"x_labels=["Jan","Feb","Mar","Apr","May","Jun"], series=[[12,19,15,22,28,24],[8,14,18,16,22,20],[5,9,12,15,18,21]], series_names=["A","B","C"]"#),
        $mac!("line", "neon",            r#"x_labels=["Jan","Feb","Mar","Apr","May","Jun"], values=[12,19,15,22,28,24]"#),
        $mac!("line", "sparkline",       r#"x_labels=["Jan","Feb","Mar","Apr","May","Jun"], values=[12,19,15,22,28,24]"#),
        $mac!("line", "spline",          r#"x_labels=["Jan","Feb","Mar","Apr","May","Jun"], values=[12,19,15,22,28,24]"#),
        $mac!("line", "stepped",         r#"x_labels=["Jan","Feb","Mar","Apr","May","Jun"], values=[12,19,15,22,28,24]"#),
        $mac!("lollipop", "basic",       r#"labels=["Alpha","Beta","Gamma","Delta","Epsilon"], values=[24,38,17,42,29]"#),
        $mac!("lollipop", "circular",    r#"labels=["Alpha","Beta","Gamma","Delta","Epsilon"], values=[24,38,17,42,29]"#),
        $mac!("lollipop", "cleveland",   r#"labels=["Alpha","Beta","Gamma","Delta","Epsilon"], series=[[24,38,17,42,29],[18,28,22,35,33]], series_names=["2023","2024"]"#),
        $mac!("lollipop", "diverging",   r#"labels=["Alpha","Beta","Gamma","Delta","Epsilon"], values=[24,-38,17,-42,29]"#),
        $mac!("lollipop", "highlight",   r#"labels=["Alpha","Beta","Gamma","Delta","Epsilon"], values=[24,38,17,42,29]"#),
        $mac!("lollipop", "office",      r#"labels=["Alpha","Beta","Gamma","Delta","Epsilon"], values=[24,38,17,42,29]"#),
        $mac!("parallel", "arc",         r#"axes=["Speed","Power","Range","Cost"], series=[[80,65,70,40],[60,80,55,60],[40,70,90,75]], series_names=["A","B","C"]"#),
        $mac!("parallel", "basic",       r#"axes=["Speed","Power","Range","Cost"], series=[[80,65,70,40],[60,80,55,60],[40,70,90,75]], series_names=["A","B","C"]"#),
        $mac!("parallel", "categorical", r#"axes=["Speed","Power","Range","Cost"], series=[[80,65,70,40],[60,80,55,60],[40,70,90,75]], series_names=["A","B","C"]"#),
        $mac!("parallel", "deluxe",      r#"axes=["Speed","Power","Range","Cost"], series=[[80,65,70,40],[60,80,55,60],[40,70,90,75]], series_names=["A","B","C"]"#),
        $mac!("parallel", "density",     r#"axes=["Speed","Power","Range","Cost"], series=[[80,65,70,40],[60,80,55,60],[40,70,90,75]], series_names=["A","B","C"]"#),
        $mac!("parallel", "gradient",    r#"axes=["Speed","Power","Range","Cost"], series=[[80,65,70,40],[60,80,55,60],[40,70,90,75]], series_names=["A","B","C"]"#),
        $mac!("parallel", "highlight",   r#"axes=["Speed","Power","Range","Cost"], series=[[80,65,70,40],[60,80,55,60],[40,70,90,75]], series_names=["A","B","C"]"#),
        $mac!("parallel", "ribbon",      r#"axes=["Speed","Power","Range","Cost"], series=[[80,65,70,40],[60,80,55,60],[40,70,90,75]], series_names=["A","B","C"]"#),
        $mac!("parallel", "smooth",      r#"axes=["Speed","Power","Range","Cost"], series=[[80,65,70,40],[60,80,55,60],[40,70,90,75]], series_names=["A","B","C"]"#),
        $mac!("pie", "basic",            r#"labels=["Apple","Banana","Cherry","Date","Fig"], values=[40,25,20,10,5]"#),
        $mac!("pie", "donut",            r#"labels=["Apple","Banana","Cherry","Date","Fig"], values=[40,25,20,10,5]"#),
        $mac!("pie", "exploded",         r#"labels=["Apple","Banana","Cherry","Date","Fig"], values=[40,25,20,10,5]"#),
        $mac!("pie", "kpi",              r#"labels=["Apple","Banana","Cherry","Date","Fig"], values=[40,25,20,10,5]"#),
        $mac!("pie", "nested",           r#"labels=["Apple","Banana","Cherry","Date","Fig"], values=[40,25,20,10,5], secondary_labels=["X","Y","Z"], secondary_values=[55,30,15]"#),
        $mac!("pie", "pattern",          r#"labels=["Apple","Banana","Cherry","Date","Fig"], values=[40,25,20,10,5]"#),
        $mac!("pie", "proportional",     r#"labels=["Apple","Banana","Cherry","Date","Fig"], values=[40,25,20,10,5]"#),
        $mac!("pie", "semi",             r#"labels=["Apple","Banana","Cherry","Date","Fig"], values=[40,25,20,10,5]"#),
        $mac!("pie", "subplots",         r#"labels=["A","B","C","D"], series=[[40,25,20,15],[30,30,20,20],[50,20,15,15]], series_names=["P1","P2","P3"]"#),
        $mac!("radar", "basic",          r#"axes=["Speed","Power","Range","Cost","Style"], series=[[80,65,70,40,75],[60,80,55,60,70]], series_names=["A","B"]"#),
        $mac!("radar", "dashed",         r#"axes=["Speed","Power","Range","Cost","Style"], series=[[80,65,70,40,75],[60,80,55,60,70]], series_names=["A","B"]"#),
        $mac!("radar", "deluxe",         r#"axes=["Speed","Power","Range","Cost","Style"], series=[[80,65,70,40,75],[60,80,55,60,70]], series_names=["A","B"]"#),
        $mac!("radar", "filled",         r#"axes=["Speed","Power","Range","Cost","Style"], series=[[80,65,70,40,75],[60,80,55,60,70]], series_names=["A","B"]"#),
        $mac!("radar", "gradient",       r#"axes=["Speed","Power","Range","Cost","Style"], series=[[80,65,70,40,75],[60,80,55,60,70]], series_names=["A","B"]"#),
        $mac!("radar", "lines",          r#"axes=["Speed","Power","Range","Cost","Style"], series=[[80,65,70,40,75],[60,80,55,60,70]], series_names=["A","B"]"#),
        $mac!("radar", "markers",        r#"axes=["Speed","Power","Range","Cost","Style"], series=[[80,65,70,40,75],[60,80,55,60,70]], series_names=["A","B"]"#),
        $mac!("radar", "polar_bar",      r#"axes=["Speed","Power","Range","Cost","Style"], series=[[80,65,70,40,75],[60,80,55,60,70]], series_names=["A","B"]"#),
        $mac!("radar", "stacked",        r#"axes=["Speed","Power","Range","Cost","Style"], series=[[80,65,70,40,75],[60,80,55,60,70]], series_names=["A","B"]"#),
        $mac!("ridgeline", "basic",      r#"categories=["A","A","A","A","A","B","B","B","B","B","C","C","C","C","C","D","D","D","D","D"], values=[1.2,2.4,2.7,3.1,3.5,2.0,2.8,3.2,3.6,4.1,1.8,2.2,2.6,3.0,3.4,2.3,2.9,3.5,3.9,4.4]"#),
        $mac!("ridgeline", "deluxe",     r#"categories=["A","A","A","A","A","B","B","B","B","B","C","C","C","C","C","D","D","D","D","D"], values=[1.2,2.4,2.7,3.1,3.5,2.0,2.8,3.2,3.6,4.1,1.8,2.2,2.6,3.0,3.4,2.3,2.9,3.5,3.9,4.4]"#),
        $mac!("ridgeline", "gradient",   r#"categories=["A","A","A","A","A","B","B","B","B","B","C","C","C","C","C","D","D","D","D","D"], values=[1.2,2.4,2.7,3.1,3.5,2.0,2.8,3.2,3.6,4.1,1.8,2.2,2.6,3.0,3.4,2.3,2.9,3.5,3.9,4.4]"#),
        $mac!("ridgeline", "heatmap",    r#"categories=["A","A","A","A","A","B","B","B","B","B","C","C","C","C","C","D","D","D","D","D"], values=[1.2,2.4,2.7,3.1,3.5,2.0,2.8,3.2,3.6,4.1,1.8,2.2,2.6,3.0,3.4,2.3,2.9,3.5,3.9,4.4]"#),
        $mac!("ridgeline", "lines",      r#"categories=["A","A","A","A","A","B","B","B","B","B","C","C","C","C","C","D","D","D","D","D"], values=[1.2,2.4,2.7,3.1,3.5,2.0,2.8,3.2,3.6,4.1,1.8,2.2,2.6,3.0,3.4,2.3,2.9,3.5,3.9,4.4]"#),
        $mac!("ridgeline", "mean",       r#"categories=["A","A","A","A","A","B","B","B","B","B","C","C","C","C","C","D","D","D","D","D"], values=[1.2,2.4,2.7,3.1,3.5,2.0,2.8,3.2,3.6,4.1,1.8,2.2,2.6,3.0,3.4,2.3,2.9,3.5,3.9,4.4]"#),
        $mac!("ridgeline", "quartiles",  r#"categories=["A","A","A","A","A","B","B","B","B","B","C","C","C","C","C","D","D","D","D","D"], values=[1.2,2.4,2.7,3.1,3.5,2.0,2.8,3.2,3.6,4.1,1.8,2.2,2.6,3.0,3.4,2.3,2.9,3.5,3.9,4.4]"#),
        $mac!("ridgeline", "rug",        r#"categories=["A","A","A","A","A","B","B","B","B","B","C","C","C","C","C","D","D","D","D","D"], values=[1.2,2.4,2.7,3.1,3.5,2.0,2.8,3.2,3.6,4.1,1.8,2.2,2.6,3.0,3.4,2.3,2.9,3.5,3.9,4.4]"#),
        $mac!("ridgeline", "spaced",     r#"categories=["A","A","A","A","A","B","B","B","B","B","C","C","C","C","C","D","D","D","D","D"], values=[1.2,2.4,2.7,3.1,3.5,2.0,2.8,3.2,3.6,4.1,1.8,2.2,2.6,3.0,3.4,2.3,2.9,3.5,3.9,4.4]"#),
        $mac!("scatter", "basic",        r#"x=[1,2,3,4,5,6,7,8,9,10], y=[2,5,3,8,7,9,6,11,9,13]"#),
        $mac!("scatter", "categorical",  r#"x=[1,2,3,4,5,6,7,8,9,10], y=[2,5,3,8,7,9,6,11,9,13], categories=["A","B","A","B","C","A","C","B","A","C"]"#),
        $mac!("scatter", "deluxe",       r#"x=[1,2,3,4,5,6,7,8,9,10], y=[2,5,3,8,7,9,6,11,9,13]"#),
        $mac!("scatter", "galaxy",       r#"x=[1,2,3,4,5,6,7,8,9,10], y=[2,5,3,8,7,9,6,11,9,13]"#),
        $mac!("scatter", "gradient",     r#"x=[1,2,3,4,5,6,7,8,9,10], y=[2,5,3,8,7,9,6,11,9,13], values=[1,2,3,4,5,6,7,8,9,10]"#),
        $mac!("scatter", "labeled",      r#"x=[1,2,3,4,5,6], y=[2,5,3,8,7,9], labels=["A","B","C","D","E","F"]"#),
        $mac!("scatter", "nova",         r#"x=[1,2,3,4,5,6,7,8,9,10], y=[2,5,3,8,7,9,6,11,9,13]"#),
        $mac!("scatter", "regression",   r#"x=[1,2,3,4,5,6,7,8,9,10], y=[2,3.8,5.1,7.2,8.5,10.3,11.8,13.4,15.1,16.7]"#),
        $mac!("scatter", "symbols",      r#"x=[1,2,3,4,5,6,7,8,9,10], y=[2,5,3,8,7,9,6,11,9,13]"#),
        $mac!("slope", "basic",          r#"labels=["A","B","C","D","E"], left=[20,35,15,42,28], right=[35,28,40,55,22]"#),
        $mac!("slope", "bumps",          r#"labels=["A","B","C","D","E"], left=[20,35,15,42,28], right=[35,28,40,55,22]"#),
        $mac!("slope", "curved",         r#"labels=["A","B","C","D","E"], left=[20,35,15,42,28], right=[35,28,40,55,22]"#),
        $mac!("slope", "diverging",      r#"labels=["A","B","C","D","E"], left=[20,35,15,42,28], right=[35,28,40,55,22]"#),
        $mac!("slope", "highlighted",    r#"labels=["A","B","C","D","E"], left=[20,35,15,42,28], right=[35,28,40,55,22]"#),
        $mac!("slope", "monochrome",     r#"labels=["A","B","C","D","E"], left=[20,35,15,42,28], right=[35,28,40,55,22]"#),
        $mac!("slope", "stepped",        r#"labels=["A","B","C","D","E"], left=[20,35,15,42,28], right=[35,28,40,55,22]"#),
        $mac!("slope", "thick",          r#"labels=["A","B","C","D","E"], left=[20,35,15,42,28], right=[35,28,40,55,22]"#),
        $mac!("sunburst", "basic",       r#"labels=["Root","A","B","A1","A2","B1","B2"], parents=["","Root","Root","A","A","B","B"], values=[0,40,30,20,20,15,15]"#),
        $mac!("sunburst", "depth_fade",  r#"labels=["Root","A","B","A1","A2","B1","B2"], parents=["","Root","Root","A","A","B","B"], values=[0,40,30,20,20,15,15]"#),
        $mac!("sunburst", "donut",       r#"labels=["Root","A","B","A1","A2","B1","B2"], parents=["","Root","Root","A","A","B","B"], values=[0,40,30,20,20,15,15]"#),
        $mac!("sunburst", "flame",       r#"labels=["Root","A","B","A1","A2","B1","B2"], parents=["","Root","Root","A","A","B","B"], values=[0,40,30,20,20,15,15]"#),
        $mac!("sunburst", "gapped",      r#"labels=["Root","A","B","A1","A2","B1","B2"], parents=["","Root","Root","A","A","B","B"], values=[0,40,30,20,20,15,15]"#),
        $mac!("sunburst", "mono",        r#"labels=["Root","A","B","A1","A2","B1","B2"], parents=["","Root","Root","A","A","B","B"], values=[0,40,30,20,20,15,15]"#),
        $mac!("sunburst", "outlined",    r#"labels=["Root","A","B","A1","A2","B1","B2"], parents=["","Root","Root","A","A","B","B"], values=[0,40,30,20,20,15,15]"#),
        $mac!("sunburst", "rainbow",     r#"labels=["Root","A","B","A1","A2","B1","B2"], parents=["","Root","Root","A","A","B","B"], values=[0,40,30,20,20,15,15]"#),
        $mac!("treemap", "basic",        r#"labels=["A","B","C","D","E","F","G"], values=[40,25,20,10,5,8,12]"#),
        $mac!("treemap", "flat",         r#"labels=["A","B","C","D","E","F","G"], values=[40,25,20,10,5,8,12]"#),
        $mac!("treemap", "gapped",       r#"labels=["A","B","C","D","E","F","G"], values=[40,25,20,10,5,8,12]"#),
        $mac!("treemap", "gradient",     r#"labels=["A","B","C","D","E","F","G"], values=[40,25,20,10,5,8,12]"#),
        $mac!("treemap", "heat",         r#"labels=["A","B","C","D","E","F","G"], values=[40,25,20,10,5,8,12]"#),
        $mac!("treemap", "mono",         r#"labels=["A","B","C","D","E","F","G"], values=[40,25,20,10,5,8,12]"#),
        $mac!("treemap", "nested",       r#"labels=["Root","A","B","A1","A2","B1"], parents=["","Root","Root","A","A","B"], values=[0,40,30,20,20,30]"#),
        $mac!("treemap", "outlined",     r#"labels=["A","B","C","D","E","F","G"], values=[40,25,20,10,5,8,12]"#),
        $mac!("violin", "aurora",        r#"labels=["A","B","C"], values=[1.2,2.4,2.7,3.1,3.5,3.8,2.0,2.8,3.2,3.6,4.1,4.5,1.8,2.2,2.6,3.0,3.4,3.9], categories=["A","A","A","A","A","A","B","B","B","B","B","B","C","C","C","C","C","C"]"#),
        $mac!("violin", "basic",         r#"labels=["A","B","C"], values=[1.2,2.4,2.7,3.1,3.5,3.8,2.0,2.8,3.2,3.6,4.1,4.5,1.8,2.2,2.6,3.0,3.4,3.9], categories=["A","A","A","A","A","A","B","B","B","B","B","B","C","C","C","C","C","C"]"#),
        $mac!("violin", "crystal",       r#"labels=["A","B","C"], values=[1.2,2.4,2.7,3.1,3.5,3.8,2.0,2.8,3.2,3.6,4.1,4.5,1.8,2.2,2.6,3.0,3.4,3.9], categories=["A","A","A","A","A","A","B","B","B","B","B","B","C","C","C","C","C","C"]"#),
        $mac!("violin", "deluxe",        r#"labels=["A","B","C"], values=[1.2,2.4,2.7,3.1,3.5,3.8,2.0,2.8,3.2,3.6,4.1,4.5,1.8,2.2,2.6,3.0,3.4,3.9], categories=["A","A","A","A","A","A","B","B","B","B","B","B","C","C","C","C","C","C"]"#),
        $mac!("violin", "half",          r#"labels=["A","B","C"], values=[1.2,2.4,2.7,3.1,3.5,3.8,2.0,2.8,3.2,3.6,4.1,4.5,1.8,2.2,2.6,3.0,3.4,3.9], categories=["A","A","A","A","A","A","B","B","B","B","B","B","C","C","C","C","C","C"]"#),
        $mac!("violin", "horizontal",    r#"labels=["A","B","C"], values=[1.2,2.4,2.7,3.1,3.5,3.8,2.0,2.8,3.2,3.6,4.1,4.5,1.8,2.2,2.6,3.0,3.4,3.9], categories=["A","A","A","A","A","A","B","B","B","B","B","B","C","C","C","C","C","C"]"#),
        $mac!("violin", "mean",          r#"labels=["A","B","C"], values=[1.2,2.4,2.7,3.1,3.5,3.8,2.0,2.8,3.2,3.6,4.1,4.5,1.8,2.2,2.6,3.0,3.4,3.9], categories=["A","A","A","A","A","A","B","B","B","B","B","B","C","C","C","C","C","C"]"#),
        $mac!("violin", "points",        r#"labels=["A","B","C"], values=[1.2,2.4,2.7,3.1,3.5,3.8,2.0,2.8,3.2,3.6,4.1,4.5,1.8,2.2,2.6,3.0,3.4,3.9], categories=["A","A","A","A","A","A","B","B","B","B","B","B","C","C","C","C","C","C"]"#),
        $mac!("violin", "quartile",      r#"labels=["A","B","C"], values=[1.2,2.4,2.7,3.1,3.5,3.8,2.0,2.8,3.2,3.6,4.1,4.5,1.8,2.2,2.6,3.0,3.4,3.9], categories=["A","A","A","A","A","A","B","B","B","B","B","B","C","C","C","C","C","C"]"#),
        $mac!("violin", "rainbow",       r#"labels=["A","B","C"], values=[1.2,2.4,2.7,3.1,3.5,3.8,2.0,2.8,3.2,3.6,4.1,4.5,1.8,2.2,2.6,3.0,3.4,3.9], categories=["A","A","A","A","A","A","B","B","B","B","B","B","C","C","C","C","C","C"]"#),
        $mac!("violin", "split",         r#"labels=["A","B","C"], values=[1.2,2.4,2.7,3.1,3.5,3.8,2.0,2.8,3.2,3.6,4.1,4.5,1.8,2.2,2.6,3.0,3.4,3.9], categories=["A","A","A","A","A","A","B","B","B","B","B","B","C","C","C","C","C","C"], color_groups=["L","L","L","R","R","R","L","L","L","R","R","R","L","L","L","R","R","R"]"#),
        $mac!("violin", "strip",         r#"labels=["A","B","C"], values=[1.2,2.4,2.7,3.1,3.5,3.8,2.0,2.8,3.2,3.6,4.1,4.5,1.8,2.2,2.6,3.0,3.4,3.9], categories=["A","A","A","A","A","A","B","B","B","B","B","B","C","C","C","C","C","C"]"#),
        $mac!("violin", "with_box",      r#"labels=["A","B","C"], values=[1.2,2.4,2.7,3.1,3.5,3.8,2.0,2.8,3.2,3.6,4.1,4.5,1.8,2.2,2.6,3.0,3.4,3.9], categories=["A","A","A","A","A","A","B","B","B","B","B","B","C","C","C","C","C","C"]"#),
        $mac!("waterfall", "arrowed",    r#"labels=["Start","Q1","Q2","Q3","End"], values=[100,30,-15,40,155]"#),
        $mac!("waterfall", "basic",      r#"labels=["Start","Q1","Q2","Q3","End"], values=[100,30,-15,40,155]"#),
        $mac!("waterfall", "delta",      r#"labels=["Start","Q1","Q2","Q3","End"], values=[100,30,-15,40,155]"#),
        $mac!("waterfall", "horizontal", r#"labels=["Start","Q1","Q2","Q3","End"], values=[100,30,-15,40,155]"#),
        $mac!("waterfall", "lollipop",   r#"labels=["Start","Q1","Q2","Q3","End"], values=[100,30,-15,40,155]"#),
        $mac!("waterfall", "stepped",    r#"labels=["Start","Q1","Q2","Q3","End"], values=[100,30,-15,40,155]"#),
        $mac!("wordcloud", "basic",      r#"words=["rust","python","wasm","plot","data","viz","chart","graph","fast","native","async","macro","trait","enum","crate"], frequencies=[42,38,30,28,25,22,18,15,12,10,9,8,7,6,5]"#),
        $mac!("wordcloud", "bubble",     r#"words=["rust","python","wasm","plot","data","viz","chart","graph","fast","native","async","macro","trait","enum","crate"], frequencies=[42,38,30,28,25,22,18,15,12,10,9,8,7,6,5]"#),
        $mac!("wordcloud", "context",    r#"words=["rust","python","wasm","plot","data","viz","chart","graph","fast","native","async","macro","trait","enum","crate"], frequencies=[42,38,30,28,25,22,18,15,12,10,9,8,7,6,5]"#),
        $mac!("wordcloud", "image",      r#"words=["rust","python","wasm","plot","data","viz","chart","graph","fast","native","async","macro","trait","enum","crate"], frequencies=[42,38,30,28,25,22,18,15,12,10,9,8,7,6,5]"#),
        $mac!("wordcloud", "labelmap",   r#"words=["rust","python","wasm","plot","data","viz","chart","graph","fast","native","async","macro","trait","enum","crate"], frequencies=[42,38,30,28,25,22,18,15,12,10,9,8,7,6,5]"#),
        $mac!("wordcloud", "network",    r#"words=["rust","python","wasm","plot","data","viz","chart","graph","fast","native","async","macro","trait","enum","crate"], frequencies=[42,38,30,28,25,22,18,15,12,10,9,8,7,6,5]"#),
        $mac!("wordcloud", "neuron",     r#"words=["rust","python","wasm","plot","data","viz","chart","graph","fast","native","async","macro","trait","enum","crate"], frequencies=[42,38,30,28,25,22,18,15,12,10,9,8,7,6,5]"#),
    ]};
}


