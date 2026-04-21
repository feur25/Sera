#[cfg(feature = "js")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "js")]
macro_rules! impl_wasm {
    ($fn:ident, $js:literal) => {
        #[wasm_bindgen(js_name = $js)]
        pub fn $fn(input: &str) -> String {
            crate::bindings::commands::charts::$fn(input)
        }
    };
}
#[cfg(feature = "js")]
crate::for_each_fn!(impl_wasm);

#[cfg(feature = "ffi")]
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
#[cfg(feature = "ffi")]
crate::for_each_fn!(impl_cffi);

#[cfg(feature = "ffi")]
#[no_mangle]
pub unsafe extern "C" fn seraplot_free(ptr: *mut std::os::raw::c_char) {
    if !ptr.is_null() {
        drop(std::ffi::CString::from_raw(ptr));
    }
}

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use numpy::{PyReadonlyArray2, PyUntypedArrayMethods, PyArrayMethods};

#[cfg(feature = "python")]
use crate::Chart;

#[cfg(feature = "python")]
use pyo3::types::PyAny;

#[cfg(feature = "python")]
fn chart_auto(name: &str, title: &str, n: usize) -> crate::ml::cache::TaskHandle {
    let fp = crate::ml::cache::Fp::new(name).str(title).usize(n).finish();
    crate::ml::cache::TaskHandle::auto(name, fp)
}

#[cfg(feature = "python")]
fn kwargs_to_json(kwargs: Option<&pyo3::types::PyDict>) -> String {
    fn v(val: &PyAny) -> serde_json::Value {
        if val.is_none() { return serde_json::Value::Null; }
        if let Ok(b) = val.extract::<bool>() { return serde_json::Value::Bool(b); }
        if let Ok(i) = val.extract::<i64>() { return serde_json::json!(i); }
        if let Ok(f) = val.extract::<f64>() { return serde_json::json!(f); }
        if let Ok(s) = val.extract::<String>() { return serde_json::Value::String(s); }
        if let Ok(list) = val.extract::<Vec<Vec<f64>>>() { return serde_json::json!(list); }
        if let Ok(list) = val.extract::<Vec<f64>>() { return serde_json::json!(list); }
        if let Ok(list) = val.extract::<Vec<String>>() { return serde_json::json!(list); }
        if let Ok(list) = val.extract::<Vec<i64>>() { return serde_json::json!(list); }
        serde_json::Value::Null
    }
    let Some(d) = kwargs else { return "{}".to_string() };
    serde_json::Value::Object(
        d.iter()
            .filter_map(|(k, val)| Some((k.str().ok()?.to_string_lossy().to_string(), v(val))))
            .collect(),
    ).to_string()
}

#[cfg(feature = "python")]
macro_rules! impl_python {
    ($fn:ident, $_js:literal) => {
        #[pyfunction]
        #[pyo3(signature = (**kwargs))]
        pub fn $fn(kwargs: Option<&pyo3::types::PyDict>) -> PyResult<crate::Chart> {
            Ok(crate::Chart::new_doc(
                crate::bindings::commands::charts::$fn(&kwargs_to_json(kwargs)),
                "",
            ))
        }
    };
}
#[cfg(feature = "python")]
crate::for_each_json_chart_fn!(impl_python);

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(name = "set_bg", signature = (html, color=None))]
pub fn set_bg_fn(html: &PyAny, color: Option<&str>) -> PyResult<Chart> {
    let raw: String = if let Ok(chart) = html.extract::<PyRef<Chart>>() {
        chart.html.clone()
    } else {
        html.extract::<String>()?
    };
    Ok(Chart::new(crate::html::hover::apply_bg(raw, color)))
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn show_chart_value(chart_json: &str) -> bool {
    let chart_json_c = std::ffi::CString::new(chart_json).unwrap_or_default();
    unsafe {
        crate::viewer::chart::sera_show_chart_value(chart_json_c.as_ptr())
    }
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn bench_chart_value(chart_json: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(chart_json).is_ok()
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn set_chart_kind(kind: u8) -> bool {
    crate::viewer::chart::sera_set_current_chart_kind(kind);
    true
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn set_chart_orientation(vertical: bool) -> bool {
    crate::viewer::chart::sera_set_chart_orientation(vertical);
    true
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (n=2000))]
pub fn bench_pure_rust(n: usize) -> (f64, f64, f64, f64) {
    use std::time::Instant;
    let ages: Vec<f64> = (0..891).map(|i| 10.0 + (i % 70) as f64).collect();
    let fare: Vec<f64> = (0..891).map(|i| (i % 50) as f64 * 2.5).collect();
    let ages100: Vec<f64> = ages[..100].to_vec();
    let fare100: Vec<f64> = fare[..100].to_vec();
    let labs: Vec<String> = (0..30).map(|i| format!("Cat {i}")).collect();
    let vals: Vec<f64> = (0..30).map(|i| i as f64 * 3.7).collect();
    let n_lbl = 11usize;
    let corr_labels: Vec<String> = (0..n_lbl).map(|i| format!("F{i}")).collect();
    let flat: Vec<f64> = (0..n_lbl * n_lbl).map(|i| ((i % 11) as f64 - 5.0) * 0.15).collect();
    use crate::plot::statistical::{HistogramConfig, render_histogram_html, HeatmapConfig, render_heatmap_html};
    let t0 = Instant::now();
    for _ in 0..n {
        let _ = render_histogram_html(&HistogramConfig {
            title: "B", values: &ages, bins: 20, width: 900, height: 400,
            ..HistogramConfig::default()
        });
    }
    let hist_ms = t0.elapsed().as_secs_f64() * 1000.0 / n as f64;
    let t0 = Instant::now();
    for _ in 0..n {
        let _ = crate::plot::default::render_bars_html("B", &labs, &vals, 900, 480, &[], b'v', &[], false, "", "", &[], 0, true, "");
    }
    let bar_ms = t0.elapsed().as_secs_f64() * 1000.0 / n as f64;
    let t0 = Instant::now();
    for _ in 0..n {
        let _ = crate::plot::default::render_scatter_html("B", &ages100, &fare100, &[], 900, 540, &[], &[], &[], &[], "", "", 0, true, false, false, "linear");
    }
    let scatter_ms = t0.elapsed().as_secs_f64() * 1000.0 / n as f64;
    let t0 = Instant::now();
    for _ in 0..n {
        let _ = render_heatmap_html(&HeatmapConfig {
            title: "B", row_labels: &corr_labels, col_labels: &[], flat_matrix: &flat,
            width: 800, ..HeatmapConfig::default()
        });
    }
    let heatmap_ms = t0.elapsed().as_secs_f64() * 1000.0 / n as f64;
    (hist_ms, bar_ms, scatter_ms, heatmap_ms)
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "DBSCAN")]
pub struct DbscanModel {
    eps: f64,
    min_samples: usize,
    labels_: Vec<i32>,
    n_clusters_: usize,
    n_noise_: usize,
}

#[cfg(feature = "python")]
#[pymethods]
impl DbscanModel {
    #[new]
    #[pyo3(signature = (eps=0.5, min_samples=5))]
    pub fn py_new(eps: f64, min_samples: usize) -> Self {
        DbscanModel { eps, min_samples, labels_: Vec::new(), n_clusters_: 0, n_noise_: 0 }
    }

    #[pyo3(signature = (x))]
    pub fn fit(&mut self, x: Vec<Vec<f64>>) -> PyResult<()> {
        let n = x.len();
        let d = x.first().map_or(0, |r| r.len());
        let fp = crate::ml::cache::Fp::new("DBSCAN.fit").f64(self.eps).usize(self.min_samples).usize(n).usize(d).finish();
        let _h = crate::ml::cache::TaskHandle::auto("DBSCAN.fit", fp);
        let (labels, n_clusters) = crate::plot::default::scatter::dbscan_core_nd(&x, self.eps, self.min_samples);
        self.n_noise_ = labels.iter().filter(|&&l| l < 0).count();
        self.labels_ = labels;
        self.n_clusters_ = n_clusters;
        _h.complete(&crate::ml::cache::PartialState::default());
        Ok(())
    }

    #[pyo3(signature = (x))]
    pub fn fit_predict(&mut self, x: Vec<Vec<f64>>) -> PyResult<Vec<i32>> {
        self.fit(x)?;
        Ok(self.labels_.clone())
    }

    #[getter]
    pub fn labels_(&self) -> Vec<i32> { self.labels_.clone() }

    #[getter]
    pub fn n_clusters_(&self) -> usize { self.n_clusters_ }

    #[getter]
    pub fn n_noise_(&self) -> usize { self.n_noise_ }

    #[getter]
    pub fn eps(&self) -> f64 { self.eps }

    #[getter]
    pub fn min_samples(&self) -> usize { self.min_samples }

    fn doc(&self) -> &'static str {
        crate::bindings::commands::docs::DOC_DBSCAN_MODEL
    }

    fn __repr__(&self) -> String {
        format!(
            "DBSCAN(eps={}, min_samples={}) -> {} clusters, {} noise",
            self.eps, self.min_samples, self.n_clusters_, self.n_noise_
        )
    }
}

#[cfg(feature = "python")]
fn extract_flat(x: &PyAny) -> PyResult<(Vec<f64>, usize, usize)> {
    if let Ok(arr) = x.extract::<PyReadonlyArray2<f64>>() {
        let shape = arr.shape();
        let n = shape[0]; let dims = shape[1];
        let view = arr.as_array();
        if view.is_standard_layout() {
            return Ok((view.as_slice().unwrap().to_vec(), n, dims));
        }
        let c = view.as_standard_layout();
        return Ok((c.as_slice().unwrap().to_vec(), n, dims));
    }
    if let Ok(arr) = x.extract::<PyReadonlyArray2<f32>>() {
        let shape = arr.shape();
        let n = shape[0]; let dims = shape[1];
        let view = arr.as_array();
        if view.is_standard_layout() {
            return Ok((view.as_slice().unwrap().iter().map(|&v| v as f64).collect(), n, dims));
        }
        let c = view.as_standard_layout();
        return Ok((c.as_slice().unwrap().iter().map(|&v| v as f64).collect(), n, dims));
    }
    let rows: Vec<Vec<f64>> = x.extract()?;
    let n = rows.len();
    if n == 0 { return Ok((Vec::new(), 0, 0)); }
    let dims = rows[0].len();
    let mut flat = vec![0.0f64; n * dims];
    for (i, row) in rows.iter().enumerate() {
        flat[i*dims..(i+1)*dims].copy_from_slice(row);
    }
    Ok((flat, n, dims))
}

#[cfg(feature = "python")]
#[pyclass(module = "seraplot", name = "KMeans")]
pub struct KMeansModel {
    k: usize,
    max_iter: usize,
    tol: f64,
    mini_batch: bool,
    batch_size: usize,
    n_init: usize,
    labels_: Vec<i32>,
    centroids_: Vec<Vec<f64>>,
    inertia_: f64,
    n_iter_: usize,
}

#[cfg(feature = "python")]
#[pymethods]
impl KMeansModel {
    #[new]
    #[pyo3(signature = (k=3, max_iter=300, tol=1e-4, mini_batch=false, batch_size=1000, n_init=10))]
    pub fn py_new(k: usize, max_iter: usize, tol: f64, mini_batch: bool, batch_size: usize, n_init: usize) -> Self {
        KMeansModel { k, max_iter, tol, mini_batch, batch_size, n_init, labels_: Vec::new(), centroids_: Vec::new(), inertia_: 0.0, n_iter_: 0 }
    }

    #[pyo3(signature = (x))]
    pub fn fit(&mut self, x: &PyAny) -> PyResult<()> {
        let (flat, n, dims) = extract_flat(x)?;
        let fp = crate::ml::cache::Fp::new("KMeans.fit").usize(self.k).usize(self.max_iter).bval(self.mini_batch).data(&flat, n, dims).finish();
        let _h = crate::ml::cache::TaskHandle::auto("KMeans.fit", fp);
        if self.mini_batch {
            let (labels, flat_c, inertia) = crate::plot::default::minibatch_kmeans_core_flat(&flat, n, dims, self.k, self.max_iter, self.batch_size);
            self.labels_ = labels;
            self.centroids_ = (0..self.k.min(n)).map(|ki| flat_c[ki*dims..(ki+1)*dims].to_vec()).collect();
            self.inertia_ = inertia;
            self.n_iter_ = self.max_iter;
        } else {
            let (labels, flat_c, inertia) = crate::plot::default::kmeans_core_flat_ninit(&flat, n, dims, self.k, self.max_iter, self.tol, self.n_init);
            self.labels_ = labels;
            self.centroids_ = (0..self.k.min(n)).map(|ki| flat_c[ki*dims..(ki+1)*dims].to_vec()).collect();
            self.inertia_ = inertia;
            self.n_iter_ = self.max_iter;
        }
        _h.complete(&crate::ml::cache::PartialState::default());
        Ok(())
    }

    #[pyo3(signature = (x))]
    pub fn fit_predict(&mut self, x: &PyAny) -> PyResult<Vec<i32>> {
        self.fit(x)?;
        Ok(self.labels_.clone())
    }

    #[pyo3(signature = (x))]
    pub fn transform(&self, x: &PyAny) -> PyResult<Vec<Vec<f64>>> {
        let (flat, n, dims) = extract_flat(x)?;
        Ok((0..n).map(|i| {
            self.centroids_.iter().map(|c| crate::plot::default::kmeans::sq_dist_flat(&flat[i*dims..(i+1)*dims], c).sqrt()).collect()
        }).collect())
    }

    #[pyo3(signature = (x))]
    pub fn predict(&self, x: &PyAny) -> PyResult<Vec<i32>> {
        let (flat, n, dims) = extract_flat(x)?;
        Ok((0..n).map(|i| {
            self.centroids_.iter().enumerate()
                .map(|(ki, c)| (ki, crate::plot::default::kmeans::sq_dist_flat(&flat[i*dims..(i+1)*dims], c)))
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .map(|(ki, _)| ki as i32).unwrap_or(0)
        }).collect())
    }

    #[getter] pub fn labels_(&self) -> Vec<i32> { self.labels_.clone() }
    #[getter] pub fn centroids_(&self) -> Vec<Vec<f64>> { self.centroids_.clone() }
    #[getter] pub fn inertia_(&self) -> f64 { self.inertia_ }
    #[getter] pub fn n_iter_(&self) -> usize { self.n_iter_ }
    #[getter] pub fn n_clusters(&self) -> usize { self.centroids_.len() }
    #[getter] pub fn k(&self) -> usize { self.k }

    fn doc(&self) -> &'static str {
        crate::bindings::commands::docs::DOC_KMEANS_MODEL
    }

    fn __repr__(&self) -> String {
        format!("KMeans(k={}, max_iter={}, n_init={}, mini_batch={}) -> inertia={:.2}", self.k, self.max_iter, self.n_init, self.mini_batch, self.inertia_)
    }
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (charts, cols=2, gap=16, bg=None, cell_height=520))]
pub fn build_grid(
    charts: Vec<PyRef<Chart>>,
    cols: usize,
    gap: i32,
    bg: Option<&str>,
    cell_height: i32,
) -> Chart {
    let _h = chart_auto("build_grid", "", 0);
    let bg_color = bg
        .map(|s| s.to_string())
        .or_else(crate::get_global_background)
        .unwrap_or_else(|| "#f0f2f5".to_string());
    let cols = cols.max(1);
    let mut buf = format!(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><style>\
        *{{box-sizing:border-box}}body{{margin:0;padding:{gap}px;background:{bg_color}}}\
        .spg{{display:grid;grid-template-columns:repeat({cols},1fr);gap:{gap}px}}\
        .spg-c iframe{{width:100%;height:{cell_height}px;border:none;display:block;\
        border-radius:12px;overflow:hidden;box-shadow:0 2px 8px rgba(0,0,0,.07)}}\
        </style></head><body><div class=\"spg\">"
    );
    for c in charts.iter() {
        let esc = c.html.replace('&', "&amp;").replace('"', "&quot;");
        buf.push_str("<div class=\"spg-c\"><iframe srcdoc=\"");
        buf.push_str(&esc);
        buf.push_str("\"></iframe></div>");
    }
    buf.push_str("</div></body></html>");
    Chart::new_doc(buf, crate::bindings::commands::docs::DOC_BUILD_GRID)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (charts, interval_ms=2500, title="", width=900, height=520))]
pub fn build_slideshow(
    charts: Vec<PyRef<Chart>>,
    interval_ms: u32,
    title: &str,
    width: i32,
    height: i32,
) -> Chart {
    let _h = chart_auto("build_slideshow", "", 0);
    if charts.is_empty() { return Chart::new_doc(String::new(), crate::bindings::commands::docs::DOC_BUILD_SLIDESHOW); }
    let svgs: Vec<String> = charts.iter().map(|c| {
        let h = &c.html;
        let start = h.find("<svg").unwrap_or(0);
        let end = h.rfind("</svg>").map(|i| i + 6).unwrap_or(h.len());
        if start < end { h[start..end].to_string() }
        else { "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"100\" height=\"100\"></svg>".to_string() }
    }).collect();
    let json_array = format!("[{}]", svgs.iter()
        .map(|s| serde_json::to_string(s).unwrap_or_else(|_| "\"\"".to_string()))
        .collect::<Vec<_>>().join(","));
    let n = charts.len();
    let ivms = interval_ms;
    let title_html = if title.is_empty() { String::new() } else {
        format!("<div style=\"color:#1e293b;font-family:system-ui;font-size:22px;font-weight:700;text-align:center;margin-bottom:16px\">{}</div>", title)
    };
    Chart::new_doc(format!(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><style>\
        body{{margin:0;padding:24px;background:#f0f2f5;display:flex;flex-direction:column;align-items:center;font-family:system-ui}}\
        .sp-frm{{border-radius:12px;overflow:hidden;box-shadow:0 2px 12px rgba(0,0,0,.1);background:#fff}}\
        .sp-frm svg{{width:{width}px;height:{height}px;display:block}}\
        .sp-ctrl{{display:flex;gap:10px;margin-top:14px;align-items:center}}\
        .sp-btn{{cursor:pointer;background:#6366f1;color:#fff;border:none;border-radius:8px;padding:7px 20px;font-size:14px;font-weight:600}}\
        .sp-btn:hover{{background:#4f46e5}}\
        .sp-ctr{{color:#64748b;font-size:13px;min-width:64px;text-align:center}}\
        .sp-prog{{width:{width}px;height:4px;background:#e2e8f0;border-radius:2px;margin-top:10px;overflow:hidden}}\
        .sp-bar{{height:100%;background:#6366f1;border-radius:2px;width:0%}}\
        </style></head><body>\
        {title_html}\
        <div class=\"sp-frm\" id=\"sp-frm\"></div>\
        <div class=\"sp-ctrl\">\
        <button class=\"sp-btn\" id=\"sp-p\">&#9664;</button>\
        <div class=\"sp-ctr\" id=\"sp-c\">1 / {n}</div>\
        <button class=\"sp-btn\" id=\"sp-n\">&#9654;</button>\
        </div>\
        <div class=\"sp-prog\"><div class=\"sp-bar\" id=\"sp-b\"></div></div>\
        <script type=\"application/json\" id=\"sp-d\">{json_array}</script>\
        <script>\
        const frames=JSON.parse(document.getElementById('sp-d').textContent);\
        let idx=0,timer;\
        function show(i){{idx=((i%frames.length)+frames.length)%frames.length;\
          document.getElementById('sp-frm').innerHTML=frames[idx];\
          document.getElementById('sp-c').textContent=(idx+1)+' / '+frames.length;\
          const b=document.getElementById('sp-b');\
          b.style.transition='none';b.style.width='0%';\
          setTimeout(()=>{{b.style.transition='width {ivms}ms linear';b.style.width='100%';}},20);}}\
        function play(){{clearInterval(timer);timer=setInterval(()=>{{show(idx+1);}},{ivms});}}\
        show(0);play();\
        document.getElementById('sp-p').onclick=()=>{{clearInterval(timer);show(idx-1);play();}};\
        document.getElementById('sp-n').onclick=()=>{{clearInterval(timer);show(idx+1);play();}};\
        </script></body></html>"
    ), crate::bindings::commands::docs::DOC_BUILD_SLIDESHOW)
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn build_hover_json(
    labels: Vec<String>,
    images: Option<Vec<Option<String>>>,
    descriptions: Option<&PyAny>,
) -> PyResult<String> {
    use crate::html::hover::{HoverSlot, slots_to_json};
    let mut slots = Vec::new();
    let n = labels.len();
    for i in 0..n {
        let mut slot = HoverSlot::new(&labels.get(i).cloned().unwrap_or_default());
        if let Some(ref imgs) = images {
            if i < imgs.len() {
                if let Some(ref img_url) = imgs[i] {
                    slot = slot.image(img_url.clone());
                }
            }
        }
        if let Some(descs_py) = descriptions {
            if let Ok(descs_list) = descs_py.extract::<Vec<Vec<(String, String)>>>() {
                if i < descs_list.len() {
                    for (k, v) in &descs_list[i] {
                        slot = slot.kv(k.clone(), v.clone());
                    }
                }
            }
        }
        slots.push(slot);
    }
    Ok(slots_to_json(&slots))
}

#[cfg(feature = "python")]
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
) -> PyResult<Chart> {
    let (bg, pal, grid) = crate::merge_global_opts(background, palette, gridlines);
    let pal_opt: Option<Vec<u32>> = if pal.is_empty() { None } else { Some(pal) };

    if let Ok(df) = x.getattr("columns") {
        let cols: Vec<String> = df.extract()?;
        if cols.len() >= 2 {
            let xv = crate::py_to_f64_vec(py, x.get_item(&cols[0])?)?;
            let yv = crate::py_to_f64_vec(py, x.get_item(&cols[1])?)?;
            let auto_title = if title.is_empty() { format!("{} vs {}", cols[0], cols[1]) } else { String::new() };
            let t = if title.is_empty() { auto_title.as_str() } else { title };
            let html = crate::bindings::commands::charts::build_scatter_chart(
                &serde_json::json!({"title":t,"x":xv,"y":yv,"color_hex":color_hex,"width":width,"height":height,"x_label":x_label,"y_label":y_label,"gridlines":grid,"palette":pal_opt,"background":bg}).to_string()
            );
            return Ok(Chart::new_doc(html, ""));
        }
    }

    let xv = crate::py_to_f64_vec(py, x)?;
    if let Some(yobj) = y {
        let yv = crate::py_to_f64_vec(py, yobj)?;
        let html = if kind == "scatter" {
            crate::bindings::commands::charts::build_scatter_chart(
                &serde_json::json!({"title":title,"x":xv,"y":yv,"color_hex":color_hex,"width":width,"height":height,"x_label":x_label,"y_label":y_label,"gridlines":grid,"palette":pal_opt,"background":bg}).to_string()
            )
        } else {
            let labels: Vec<String> = xv.iter().map(|v| v.to_string()).collect();
            crate::bindings::commands::charts::build_line_chart(
                &serde_json::json!({"title":title,"labels":labels,"values":yv,"color_hex":color_hex,"show_points":show_points,"width":width,"height":height,"x_label":x_label,"y_label":y_label,"gridlines":grid,"palette":pal_opt,"background":bg}).to_string()
            )
        };
        Ok(Chart::new_doc(html, ""))
    } else {
        let labels: Vec<String> = (0..xv.len()).map(|i| i.to_string()).collect();
        let html = crate::bindings::commands::charts::build_line_chart(
            &serde_json::json!({"title":title,"labels":labels,"values":xv,"color_hex":color_hex,"show_points":show_points,"width":width,"height":height,"x_label":x_label,"y_label":y_label,"gridlines":grid,"palette":pal_opt,"background":bg}).to_string()
        );
        Ok(Chart::new_doc(html, ""))
    }
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (chart, path))]
pub fn savefig(chart: &Chart, path: &str) -> PyResult<()> {
    std::fs::write(path, &chart.html).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
}
