use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyModule};

pub struct PyFnEntry {
    pub register: fn(&Bound<'_, PyModule>) -> PyResult<()>,
}
inventory::collect!(PyFnEntry);

pub fn python_py_args_to_json(
    title: &str,
    labels: Option<&Bound<'_, PyAny>>,
    values: Option<&Bound<'_, PyAny>>,
    theme: Option<&str>,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> String {
    let mut map = serde_json::Map::new();
    map.insert(
        "title".to_string(),
        serde_json::Value::String(title.to_string()),
    );
    if let Some(l) = labels {
        if let Ok(v) = l.extract::<Vec<String>>() {
            map.insert(
                "labels".to_string(),
                serde_json::Value::Array(v.into_iter().map(serde_json::Value::String).collect()),
            );
        }
    }
    if let Some(v) = values {
        if let Ok(nums) = v.extract::<Vec<f64>>() {
            map.insert(
                "values".to_string(),
                serde_json::Value::Array(nums.into_iter().map(|f| serde_json::json!(f)).collect()),
            );
        }
    }
    if let Some(t) = theme {
        map.insert(
            "theme".to_string(),
            serde_json::Value::String(t.to_string()),
        );
    }
    if let Some(kw) = kwargs {
        for (k, v) in kw.iter() {
            if let Ok(key) = k.extract::<String>() {
                map.insert(key, py_any_to_json(&v));
            }
        }
    }
    serde_json::Value::Object(map).to_string()
}

pub fn python_chart_from_html(html: String, _doc: &str) -> crate::Chart {
    crate::Chart { html, doc_str: "" }
}

fn py_any_to_json(v: &Bound<'_, PyAny>) -> serde_json::Value {
    let candidates = [
        v.extract::<bool>().ok().map(serde_json::Value::Bool),
        v.extract::<i64>().ok().map(|i| serde_json::json!(i)),
        v.extract::<f64>().ok().map(|f| serde_json::json!(f)),
        v.extract::<String>().ok().map(serde_json::Value::String),
        v.extract::<Vec<Bound<'_, PyAny>>>()
            .ok()
            .map(|items| serde_json::Value::Array(items.iter().map(py_any_to_json).collect())),
    ];
    candidates.into_iter().flatten().next().unwrap_or(serde_json::Value::Null)
}

#[pyfunction]
fn _sera_call(name: &str, json: &str) -> PyResult<String> {
    let resolved = crate::bindings::alias_registry::resolve(name);
    let target = resolved.as_deref().unwrap_or(name);
    for entry in crate::bindings::fn_registry::iter_entries() {
        if entry.name == target {
            return Ok((entry.invoke)(json));
        }
    }
    Err(pyo3::exceptions::PyValueError::new_err(format!(
        "seraplot: unknown function '{}'",
        name
    )))
}

#[pyfunction]
fn _sera_list() -> Vec<&'static str> {
    crate::bindings::fn_registry::iter_entries()
        .map(|e| e.name)
        .collect()
}

#[pyfunction]
fn _sera_aliases() -> Vec<(&'static str, &'static str)> {
    crate::CHART_ALIAS_REGISTRY
        .iter()
        .map(|(a, f)| (*a, *f))
        .collect()
}

#[pyfunction]
fn _themes() -> Vec<String> {
    crate::themes()
}

#[pyfunction]
fn _reset_theme() {
    crate::reset_theme();
}

#[pyfunction]
fn _set_global_background(color: String) {
    crate::set_global_background(&color);
}

#[pyfunction]
fn _reset_global_background() {
    crate::reset_global_background();
}

#[pyfunction]
fn _set_auto_display(enabled: bool) {
    crate::set_auto_display(enabled);
}

#[pyfunction]
fn _reset_config() {
    crate::reset_config();
}

#[pyfunction]
fn _demos() -> Vec<&'static str> {
    crate::demos()
}

#[pyfunction]
#[pyo3(signature = (chart, variant = None))]
fn _demo(chart: &str, variant: Option<String>) -> Option<String> {
    crate::demo(chart, variant.as_deref())
}

#[pyfunction]
#[pyo3(signature = (chart = None, variant = None))]
fn _params_json(chart: Option<&str>, variant: Option<&str>) -> String {
    serde_json::to_string(&crate::params(chart, variant)).unwrap_or_default()
}

#[pyfunction]
#[pyo3(signature = (chart = None, variant = None))]
fn _required_params_json(chart: Option<&str>, variant: Option<&str>) -> String {
    serde_json::to_string(&crate::required_params(chart, variant)).unwrap_or_default()
}

#[pyfunction]
fn _chart_variants_json() -> String {
    serde_json::to_string(&crate::chart_variants()).unwrap_or_default()
}

#[pyfunction]
fn _chart_themes_json() -> String {
    serde_json::to_string(&crate::chart_themes()).unwrap_or_default()
}

#[pyfunction]
fn _scenes3d_json() -> String {
    serde_json::to_string(&crate::scenes3d()).unwrap_or_default()
}

#[pyfunction]
fn _docs_json() -> String {
    serde_json::to_string(&crate::doc_registry::all_docs()).unwrap_or_default()
}

#[pyfunction]
fn _alias_add(method: &str, alias: &str) -> bool {
    crate::bindings::alias_registry::add_alias(method, alias)
}

#[pyfunction]
fn _alias_remove(method: &str, alias: &str) -> bool {
    crate::bindings::alias_registry::remove_alias(method, alias)
}

#[pyfunction]
fn _alias_reset() {
    crate::bindings::alias_registry::reset();
}

#[pyfunction]
fn _alias_list_json() -> String {
    crate::bindings::alias_registry::aliases_json()
}

#[pyfunction]
fn _alias_resolve(name: &str) -> Option<String> {
    crate::bindings::alias_registry::resolve(name)
}

#[pyfunction]
#[pyo3(signature = (path = None))]
fn _alias_save(path: Option<&str>) -> PyResult<String> {
    crate::bindings::alias_registry::save_to_disk(path)
        .map_err(pyo3::exceptions::PyOSError::new_err)
}

#[pyfunction]
#[pyo3(signature = (path = None))]
fn _alias_load(path: Option<&str>) -> bool {
    crate::bindings::alias_registry::load_from_disk(path)
}

#[pyfunction]
#[pyo3(signature = (bindings))]
fn bind_colors(bindings: &Bound<'_, pyo3::types::PyDict>) -> PyResult<()> {
    let mut vec: Vec<(String, u32)> = Vec::new();
    for (k, v) in bindings.iter() {
        let lbl = k.extract::<String>()?;
        let col = if let Ok(i) = v.extract::<u64>() {
            (i & 0xFFFFFF) as u32
        } else if let Ok(s) = v.extract::<String>() {
            let s = s.trim_start_matches('#');
            u32::from_str_radix(s, 16).unwrap_or(0)
        } else {
            continue;
        };
        vec.push((lbl, col));
    }
    crate::bind_colors(vec);
    Ok(())
}

#[pyfunction]
fn clear_color_bindings() {
    crate::clear_color_bindings();
}

#[pyfunction]
fn chart_info(chart: &crate::Chart) -> String {
    let payload = serde_json::json!({ "html": chart.html }).to_string();
    crate::plot::utils::chart_info(&payload)
}

#[pymethods]
impl crate::Chart {
    #[new]
    #[pyo3(signature = (html = String::new()))]
    fn py_new(html: String) -> Self {
        crate::Chart { html, doc_str: "" }
    }

    #[getter]
    #[pyo3(name = "html")]
    fn py_html(&self) -> &str {
        &self.html
    }

    fn __str__(&self) -> &str {
        &self.html
    }

    fn __repr__(&self) -> String {
        format!("Chart({} bytes)", self.html.len())
    }

    fn __len__(&self) -> usize {
        self.html.len()
    }

    fn __bool__(&self) -> bool {
        !self.html.is_empty()
    }

    fn __getstate__(&self) -> String {
        self.html.clone()
    }

    fn __setstate__(&mut self, state: String) {
        self.html = state;
    }

    fn _repr_html_(&self) -> String {
        self.chart_iframe()
    }

    #[pyo3(signature = (**kwargs))]
    fn _ipython_display_(
        &self,
        py: Python<'_>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<()> {
        let _ = kwargs;
        let ipython = py.import_bound("IPython.display")?;
        let html_cls = ipython.getattr("HTML")?;
        let display_fn = ipython.getattr("display")?;
        let html_obj = html_cls.call1((self.chart_iframe().as_str(),))?;
        display_fn.call1((html_obj,))?;
        Ok(())
    }

    fn show(&self, py: Python<'_>) -> PyResult<()> {
        let ipython = py.import_bound("IPython.display")?;
        let html_cls = ipython.getattr("HTML")?;
        let display_fn = ipython.getattr("display")?;
        let html_obj = html_cls.call1((self.chart_iframe().as_str(),))?;
        display_fn.call1((html_obj,))?;
        Ok(())
    }

    fn doc(&self) -> &'static str {
        self.doc_str
    }

    #[pyo3(name = "export_png", signature = (path, scale = 2.0))]
    fn py_export_png(&self, py: Python<'_>, path: &str, scale: f64) -> PyResult<()> {
        let h = &self.html;
        let start = h
            .find("<svg")
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("No SVG in chart"))?;
        let end = h
            .rfind("</svg>")
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Malformed SVG"))?
            + 6;
        let svg = h[start..end].to_string();
        match py.import_bound("cairosvg") {
            Ok(m) => {
                let kw = PyDict::new_bound(py);
                kw.set_item("write_to", path)?;
                kw.set_item("scale", scale)?;
                m.call_method("svg2png", (svg.as_str(),), Some(&kw))?;
                Ok(())
            }
            Err(_) => Err(pyo3::exceptions::PyImportError::new_err(
                "PNG export requires cairosvg: pip install cairosvg",
            )),
        }
    }

    #[pyo3(name = "save")]
    fn py_save(&self, path: &str) -> PyResult<()> {
        std::fs::write(path, &self.html)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    #[pyo3(name = "save_html")]
    fn py_save_alias_save_html(&self, path: &str) -> PyResult<()> {
        self.py_save(path)
    }

    #[pyo3(name = "write")]
    fn py_save_alias_write(&self, path: &str) -> PyResult<()> {
        self.py_save(path)
    }

    #[pyo3(name = "export_html")]
    fn py_save_alias_export_html(&self, path: &str) -> PyResult<()> {
        self.py_save(path)
    }

    #[pyo3(name = "export_svg")]
    fn py_export_svg(&self, path: &str) -> PyResult<()> {
        let svg = self
            .to_svg()
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("No SVG in chart"))?;
        std::fs::write(path, svg).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    #[pyo3(name = "save_svg")]
    fn py_export_svg_alias_save_svg(&self, path: &str) -> PyResult<()> {
        self.py_export_svg(path)
    }

    #[pyo3(name = "svg_export")]
    fn py_export_svg_alias_svg_export(&self, path: &str) -> PyResult<()> {
        self.py_export_svg(path)
    }

    #[pyo3(name = "write_svg")]
    fn py_export_svg_alias_write_svg(&self, path: &str) -> PyResult<()> {
        self.py_export_svg(path)
    }

    #[pyo3(name = "to_svg")]
    fn py_to_svg(&self) -> Option<String> {
        self.to_svg()
    }

    #[pyo3(name = "svg")]
    fn py_to_svg_alias_svg(&self) -> Option<String> {
        self.py_to_svg()
    }

    #[pyo3(name = "as_svg")]
    fn py_to_svg_alias_as_svg(&self) -> Option<String> {
        self.py_to_svg()
    }

    #[pyo3(name = "get_svg")]
    fn py_to_svg_alias_get_svg(&self) -> Option<String> {
        self.py_to_svg()
    }
}

pub fn __init(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<crate::Chart>()?;
    m.add_class::<crate::canvas::Canvas>()?;
    m.add_function(wrap_pyfunction!(crate::canvas::canvas, m)?)?;
    m.add_function(wrap_pyfunction!(crate::canvas::canvas_load, m)?)?;
    m.add_function(wrap_pyfunction!(crate::canvas::canvas_save_named, m)?)?;
    m.add_function(wrap_pyfunction!(crate::canvas::canvas_load_named, m)?)?;
    m.add_function(wrap_pyfunction!(crate::canvas::canvas_default_dir, m)?)?;
    m.add_class::<crate::PyDataset>()?;
    m.add_class::<crate::data::Table>()?;
    m.add_class::<crate::data::SeraDFrame_>()?;
    m.add_class::<crate::data::SeraDFrameGroupBy>()?;
    m.add_class::<crate::data::DFrameBuilder>()?;
    m.add_class::<crate::PyDatasetStats>()?;
    m.add_class::<crate::LiveStream>()?;
    #[cfg(feature = "webapp")]
    m.add_class::<crate::webapp::App>()?;
    m.add_function(wrap_pyfunction!(crate::theme, m)?)?;
    m.add_function(wrap_pyfunction!(crate::config, m)?)?;
    m.add_function(wrap_pyfunction!(_sera_call, m)?)?;
    m.add_function(wrap_pyfunction!(_sera_list, m)?)?;
    m.add_function(wrap_pyfunction!(_sera_aliases, m)?)?;
    m.add_function(wrap_pyfunction!(_themes, m)?)?;
    m.add_function(wrap_pyfunction!(_reset_theme, m)?)?;
    m.add_function(wrap_pyfunction!(_set_global_background, m)?)?;
    m.add_function(wrap_pyfunction!(_reset_global_background, m)?)?;
    m.add_function(wrap_pyfunction!(_set_auto_display, m)?)?;
    m.add_function(wrap_pyfunction!(_reset_config, m)?)?;
    m.add_function(wrap_pyfunction!(_demos, m)?)?;
    m.add_function(wrap_pyfunction!(_demo, m)?)?;
    m.add_function(wrap_pyfunction!(_params_json, m)?)?;
    m.add_function(wrap_pyfunction!(_required_params_json, m)?)?;
    m.add_function(wrap_pyfunction!(_chart_variants_json, m)?)?;
    m.add_function(wrap_pyfunction!(_chart_themes_json, m)?)?;
    m.add_function(wrap_pyfunction!(_scenes3d_json, m)?)?;
    m.add_function(wrap_pyfunction!(_docs_json, m)?)?;
    m.add_function(wrap_pyfunction!(_alias_add, m)?)?;
    m.add_function(wrap_pyfunction!(_alias_remove, m)?)?;
    m.add_function(wrap_pyfunction!(_alias_reset, m)?)?;
    m.add_function(wrap_pyfunction!(_alias_list_json, m)?)?;
    m.add_function(wrap_pyfunction!(_alias_resolve, m)?)?;
    m.add_function(wrap_pyfunction!(_alias_save, m)?)?;
    m.add_function(wrap_pyfunction!(_alias_load, m)?)?;
    m.add_function(wrap_pyfunction!(chart_info, m)?)?;
    m.add_function(wrap_pyfunction!(bind_colors, m)?)?;
    m.add_function(wrap_pyfunction!(clear_color_bindings, m)?)?;
    for entry in inventory::iter::<PyFnEntry>() {
        (entry.register)(m)?;
    }
    m.add("__version__", crate::VERSION)?;
    Ok(())
}
