use pyo3::prelude::*;
use pyo3::types::PyAny;

use crate::bindings::registry_macro::{for_each_auto_util_fn, for_each_json_chart_fn, for_each_ml_oneshot_fn};

pub fn python_chart_from_html(html: String, doc: &'static str) -> crate::Chart {
    crate::Chart::new_doc(html, doc)
}

pub fn python_chart_from_payload(
    builder: fn(&str) -> String,
    payload: String,
    doc: &'static str,
) -> crate::Chart {
    python_chart_from_html(builder(&payload), doc)
}

pub fn python_chart_refs_to_html(charts: &[pyo3::PyRef<'_, crate::Chart>]) -> Vec<String> {
    charts.iter().map(|c| c.html.clone()).collect()
}

pub fn python_extract_chart_html(html: &pyo3::Bound<'_, pyo3::types::PyAny>) -> pyo3::PyResult<String> {
    if let Ok(chart) = html.extract::<pyo3::PyRef<'_, crate::Chart>>() {
        Ok(chart.html.clone())
    } else {
        html.extract::<String>()
    }
}

pub fn python_py_any_to_json(val: &pyo3::Bound<'_, pyo3::types::PyAny>) -> serde_json::Value {
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

pub fn python_py_args_to_json(
    title: &str,
    labels: Option<&pyo3::Bound<'_, pyo3::types::PyAny>>,
    values: Option<&pyo3::Bound<'_, pyo3::types::PyAny>>,
    theme: Option<&str>,
    kwargs: Option<&pyo3::Bound<'_, pyo3::types::PyDict>>,
) -> String {
    let mut m = serde_json::Map::new();
    if !title.is_empty() { m.insert("title".into(), serde_json::json!(title)); }
    if let Some(v) = labels {
        let jv = python_py_any_to_json(v);
        if jv != serde_json::Value::Null { m.insert("labels".into(), jv); }
    }
    if let Some(v) = values {
        if let Ok(vv) = v.extract::<Vec<Vec<f64>>>() {
            m.insert("series".into(), serde_json::json!(vv));
        } else {
            let jv = python_py_any_to_json(v);
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
                let v = python_py_any_to_json(&val);
                if v != serde_json::Value::Null { m.insert(ks, v); }
            }
        }
    }
    serde_json::Value::Object(m).to_string()
}

macro_rules! impl_python_json_chart_binding {
    ($fn:ident, $_js:literal) => {
        #[crate::sera_binding(python, py_json, chart)]
        fn $fn(input: &str) -> String {
            crate::bindings::commands::charts::$fn(input)
        }
    };
}
for_each_json_chart_fn!(impl_python_json_chart_binding);

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

#[crate::sera_sig(html, color=None)]
#[crate::sera_binding(python, py_chart, chart)]
fn set_bg(html: &str, color: Option<&str>) -> String {
    crate::html::hover::apply_bg(html.to_string(), color)
}

#[crate::sera_binding(python, py_chart)]
fn show_chart_value(chart_json: &str) -> bool {
    crate::bindings::commands::charts::show_chart_value(chart_json)
}

#[crate::sera_binding(python, py_chart)]
fn bench_chart_value(chart_json: &str) -> bool {
    crate::bindings::commands::charts::bench_chart_value(chart_json)
}

#[crate::sera_binding(python, py_true)]
fn set_chart_kind(kind: u8) {
    crate::bindings::commands::charts::set_chart_kind(kind)
}

#[crate::sera_binding(python, py_true)]
fn set_chart_orientation(vertical: bool) {
    crate::bindings::commands::charts::set_chart_orientation(vertical)
}

#[crate::sera_sig(n=2000)]
#[crate::sera_binding(python)]
fn bench_pure_rust(n: usize) -> (f64, f64, f64, f64) {
    crate::bindings::commands::charts::bench_pure_rust(n)
}

#[pyfunction]
#[pyo3(signature = (charts, cols=2, gap=16, bg=None, cell_height=520))]
pub fn build_grid(
    charts: Vec<pyo3::PyRef<crate::Chart>>,
    cols: usize,
    gap: i32,
    bg: Option<&str>,
    cell_height: i32,
) -> crate::Chart {
    let html_strs = python_chart_refs_to_html(&charts);
    python_chart_from_payload(
        crate::bindings::commands::charts::build_grid,
        serde_json::json!({"charts": html_strs, "cols": cols, "gap": gap, "background": bg, "cell_height": cell_height}).to_string(),
        crate::bindings::commands::docs::DOC_BUILD_GRID,
    )
}

#[pyfunction]
#[pyo3(signature = (charts, interval_ms=2500, title="", width=900, height=520))]
pub fn build_slideshow(
    charts: Vec<pyo3::PyRef<crate::Chart>>,
    interval_ms: u32,
    title: &str,
    width: i32,
    height: i32,
) -> crate::Chart {
    let html_strs = python_chart_refs_to_html(&charts);
    python_chart_from_payload(
        crate::bindings::commands::charts::build_slideshow,
        serde_json::json!({"charts": html_strs, "interval_ms": interval_ms, "title": title, "width": width, "height": height}).to_string(),
        crate::bindings::commands::docs::DOC_BUILD_SLIDESHOW,
    )
}

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
            let result = Ok(python_chart_from_payload(
                crate::bindings::commands::charts::plot_chart,
                serde_json::json!({"x":xv,"y":yv,"kind":"scatter","title":auto_title,"color_hex":color_hex,"width":width,"height":height,"x_label":x_label,"y_label":y_label,"gridlines":grid,"palette":pal_opt,"background":bg}).to_string(),
                "",
            ));
            crate::telemetry::record(crate::telemetry::TelemetryEvent::new("plot", t.elapsed().as_secs_f64() * 1000.0));
            return result;
        }
    }
    let xv = crate::extract_f64_vec(py, x)?;
    let yv: Option<Vec<f64>> = if let Some(yobj) = y { Some(crate::extract_f64_vec(py, yobj)?) } else { None };
    let res = Ok(python_chart_from_payload(
        crate::bindings::commands::charts::plot_chart,
        serde_json::json!({"x":xv,"y":yv,"kind":kind,"title":title,"color_hex":color_hex,"show_points":show_points,"width":width,"height":height,"x_label":x_label,"y_label":y_label,"gridlines":grid,"palette":pal_opt,"background":bg}).to_string(),
        "",
    ));
    crate::telemetry::record(crate::telemetry::TelemetryEvent::new("plot", t.elapsed().as_secs_f64() * 1000.0).with_data(xv.len() as u64, 0.0));
    res
}