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
    if let Ok(b) = v.extract::<bool>() {
        return serde_json::Value::Bool(b);
    }
    if let Ok(i) = v.extract::<i64>() {
        return serde_json::json!(i);
    }
    if let Ok(f) = v.extract::<f64>() {
        return serde_json::json!(f);
    }
    if let Ok(s) = v.extract::<String>() {
        return serde_json::Value::String(s);
    }
    if let Ok(list) = v.extract::<Vec<Bound<'_, PyAny>>>() {
        return serde_json::Value::Array(list.iter().map(py_any_to_json).collect());
    }
    serde_json::Value::Null
}

#[pyfunction]
fn _sera_call(name: &str, json: &str) -> PyResult<String> {
    for entry in crate::bindings::fn_registry::iter_entries() {
        if entry.name == name {
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

    #[pyo3(name = "export_svg")]
    fn py_export_svg(&self, path: &str) -> PyResult<()> {
        let svg = self
            .to_svg()
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("No SVG in chart"))?;
        std::fs::write(path, svg).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    #[pyo3(name = "to_svg")]
    fn py_to_svg(&self) -> Option<String> {
        self.to_svg()
    }

    #[pyo3(name = "diff")]
    fn py_diff(&self, other: &crate::Chart) -> String {
        self.diff(other)
    }

    #[pyo3(name = "set_bg", signature = (color = None))]
    fn py_set_bg(&self, color: Option<String>) -> crate::Chart {
        self.set_bg(color.as_deref())
    }

    #[pyo3(name = "inject_css")]
    fn py_inject_css(&self, css: &str) -> crate::Chart {
        self.inject_css(css)
    }

    #[pyo3(name = "inject_js")]
    fn py_inject_js(&self, js: &str) -> crate::Chart {
        self.inject_js(js)
    }

    #[pyo3(name = "no_x_axis")]
    fn py_no_x_axis(&self) -> crate::Chart {
        self.no_x_axis()
    }

    #[pyo3(name = "no_y_axis")]
    fn py_no_y_axis(&self) -> crate::Chart {
        self.no_y_axis()
    }

    #[pyo3(name = "no_axes")]
    fn py_no_axes(&self) -> crate::Chart {
        self.no_axes()
    }

    #[pyo3(name = "no_hover")]
    fn py_no_hover(&self) -> crate::Chart {
        self.no_hover()
    }

    #[pyo3(name = "show_grid")]
    fn py_show_grid(&self) -> crate::Chart {
        self.show_grid()
    }

    #[pyo3(name = "hide_grid")]
    fn py_hide_grid(&self) -> crate::Chart {
        self.hide_grid()
    }

    #[pyo3(name = "crosshair")]
    fn py_crosshair(&self) -> crate::Chart {
        self.crosshair()
    }

    #[pyo3(name = "zoom")]
    fn py_zoom(&self) -> crate::Chart {
        self.zoom()
    }

    #[pyo3(name = "flip")]
    fn py_flip(&self) -> crate::Chart {
        self.flip()
    }

    #[pyo3(name = "horizontal")]
    fn py_horizontal(&self) -> crate::Chart {
        self.horizontal()
    }

    #[pyo3(name = "responsive")]
    fn py_responsive(&self) -> crate::Chart {
        self.responsive()
    }

    #[pyo3(name = "export_button")]
    fn py_export_button(&self) -> crate::Chart {
        self.export_button()
    }

    #[pyo3(name = "csp_safe")]
    fn py_csp_safe(&self) -> crate::Chart {
        self.csp_safe()
    }

    #[pyo3(name = "no_legend")]
    fn py_no_legend(&self) -> crate::Chart {
        self.no_legend()
    }

    #[pyo3(name = "no_title")]
    fn py_no_title(&self) -> crate::Chart {
        self.no_title()
    }

    #[pyo3(name = "show_title")]
    fn py_show_title(&self) -> crate::Chart {
        self.show_title()
    }

    #[pyo3(name = "show_legend")]
    fn py_show_legend(&self) -> crate::Chart {
        self.show_legend()
    }

    #[pyo3(name = "set_font_size")]
    fn py_set_font_size(&self, px: u32) -> crate::Chart {
        self.set_font_size(px)
    }

    #[pyo3(name = "scale")]
    fn py_scale(&self, factor: f64) -> crate::Chart {
        self.scale(factor)
    }

    #[pyo3(name = "font")]
    fn py_font(&self, name: &str) -> crate::Chart {
        self.font(name)
    }

    #[pyo3(name = "title_size")]
    fn py_title_size(&self, px: i32) -> crate::Chart {
        self.title_size(px)
    }

    #[pyo3(name = "border_radius")]
    fn py_border_radius(&self, px: i32) -> crate::Chart {
        self.border_radius(px)
    }

    #[pyo3(name = "set_opacity")]
    fn py_set_opacity(&self, value: f64) -> crate::Chart {
        self.set_opacity(value)
    }

    #[pyo3(name = "set_margin")]
    fn py_set_margin(&self, px: i32) -> crate::Chart {
        self.set_margin(px)
    }

    #[pyo3(name = "bar_gap")]
    fn py_bar_gap(&self, ratio: f64) -> crate::Chart {
        self.bar_gap(ratio)
    }

    #[pyo3(name = "set_padding")]
    fn py_set_padding(&self, px: i32) -> crate::Chart {
        self.set_padding(px)
    }

    #[pyo3(name = "rotate_labels")]
    fn py_rotate_labels(&self, angle: i32) -> crate::Chart {
        self.rotate_labels(angle)
    }

    #[pyo3(name = "rotate")]
    fn py_rotate(&self, deg: i32) -> crate::Chart {
        self.rotate(deg)
    }

    #[pyo3(name = "corner_radius_bars")]
    fn py_corner_radius_bars(&self, radius: &str) -> crate::Chart {
        self.corner_radius_bars(radius)
    }

    #[pyo3(name = "text_angle")]
    fn py_text_angle(&self, degrees: i32) -> crate::Chart {
        self.text_angle(degrees)
    }

    #[pyo3(name = "text_position")]
    fn py_text_position(&self, position: &str) -> crate::Chart {
        self.text_position(position)
    }

    #[pyo3(name = "legend_position")]
    fn py_legend_position(&self, position: &str) -> crate::Chart {
        self.legend_position(position)
    }

    #[pyo3(name = "set_frame", signature = (color = None))]
    fn py_set_frame(&self, color: Option<String>) -> crate::Chart {
        self.set_frame(color.as_deref())
    }

    #[pyo3(name = "show_labels", signature = (position = "bottom", labels = None, colors = None))]
    fn py_show_labels(
        &self,
        position: &str,
        labels: Option<Vec<String>>,
        colors: Option<Vec<String>>,
    ) -> crate::Chart {
        self.show_labels(position, labels, colors)
    }

    #[pyo3(name = "label_position", signature = (position = "bottom"))]
    fn py_label_position(&self, position: &str) -> crate::Chart {
        self.label_position(position)
    }

    #[pyo3(name = "legend", signature = (position = "right"))]
    fn py_legend(&self, position: &str) -> crate::Chart {
        self.legend(position)
    }

    #[pyo3(name = "sort_by", signature = (order = "desc"))]
    fn py_sort_by(&self, order: &str) -> crate::Chart {
        self.sort_by(order)
    }

    #[pyo3(name = "animate", signature = (duration = 300))]
    fn py_animate(&self, duration: i32) -> crate::Chart {
        self.animate(duration)
    }

    #[pyo3(name = "axis_label_angle", signature = (x_angle = None, y_angle = None))]
    fn py_axis_label_angle(&self, x_angle: Option<i32>, y_angle: Option<i32>) -> crate::Chart {
        self.axis_label_angle(x_angle, y_angle)
    }

    #[pyo3(name = "a11y", signature = (title = "", desc = ""))]
    fn py_a11y(&self, title: &str, desc: &str) -> crate::Chart {
        self.a11y(title, desc)
    }

    #[pyo3(name = "downsample", signature = (n = 2000, method = "lttb"))]
    fn py_downsample(&self, n: usize, method: &str) -> crate::Chart {
        self.downsample(n, method)
    }

    #[pyo3(name = "text_auto", signature = (format = None, position = None, angle = None, font_size = None, color = None))]
    fn py_text_auto(
        &self,
        format: Option<String>,
        position: Option<String>,
        angle: Option<i32>,
        font_size: Option<i32>,
        color: Option<String>,
    ) -> crate::Chart {
        self.text_auto(
            format.as_deref(),
            position.as_deref(),
            angle,
            font_size,
            color.as_deref(),
        )
    }

    #[pyo3(name = "text_font", signature = (family = None, size = None, color = None))]
    fn py_text_font(
        &self,
        family: Option<String>,
        size: Option<i32>,
        color: Option<String>,
    ) -> crate::Chart {
        self.text_font(family.as_deref(), size, color.as_deref())
    }

    #[pyo3(name = "uniform_text", signature = (min_size = 8, mode = "hide"))]
    fn py_uniform_text(&self, min_size: i32, mode: &str) -> crate::Chart {
        self.uniform_text(min_size, mode)
    }
}

pub fn __init(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<crate::Chart>()?;
    m.add_class::<crate::PyDataset>()?;
    m.add_class::<crate::PyDatasetStats>()?;
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
    for entry in inventory::iter::<PyFnEntry>() {
        (entry.register)(m)?;
    }
    m.add("__version__", crate::VERSION)?;
    Ok(())
}
