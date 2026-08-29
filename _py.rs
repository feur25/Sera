use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyModule};

pub struct PyFnEntry {
    pub register: fn(&Bound<'_, PyModule>) -> PyResult<()>,
}
inventory::collect!(PyFnEntry);

fn write_json_str(buf: &mut String, s: &str) {
    buf.push('"');
    for c in s.chars() {
        match c {
            '"' => buf.push_str("\\\""),
            '\\' => buf.push_str("\\\\"),
            '\n' => buf.push_str("\\n"),
            '\r' => buf.push_str("\\r"),
            '\t' => buf.push_str("\\t"),
            c if (c as u32) < 0x20 => buf.push_str(&format!("\\u{:04x}", c as u32)),
            c => buf.push(c),
        }
    }
    buf.push('"');
}

fn write_f64_array(buf: &mut String, key: &str, nums: &[f64]) {
    buf.push(',');
    write_json_str(buf, key);
    buf.push(':');
    buf.push('[');
    for (i, f) in nums.iter().enumerate() {
        if i > 0 {
            buf.push(',');
        }
        if f.is_finite() {
            buf.push_str(&f.to_string());
        } else {
            buf.push('0');
        }
    }
    buf.push(']');
}

pub fn python_py_args_to_json(
    title: &str,
    labels: Option<&Bound<'_, PyAny>>,
    values: Option<&Bound<'_, PyAny>>,
    theme: Option<&str>,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> String {
    let mut buf = String::with_capacity(256);
    buf.push('{');
    write_json_str(&mut buf, "title");
    buf.push(':');
    write_json_str(&mut buf, title);

    if let Some(v) = values {
        if let Ok(nums) = v.extract::<Vec<f64>>() {
            write_f64_array(&mut buf, "values", &nums);
        }
    }

    if let Some(l) = labels {
        if let Ok(v) = l.extract::<Vec<String>>() {
            buf.push(',');
            write_json_str(&mut buf, "labels");
            buf.push(':');
            buf.push('[');
            for (i, s) in v.iter().enumerate() {
                if i > 0 {
                    buf.push(',');
                }
                write_json_str(&mut buf, s);
            }
            buf.push(']');
        } else if let Ok(v) = l.extract::<Vec<f64>>() {
            buf.push(',');
            write_json_str(&mut buf, "labels");
            buf.push(':');
            buf.push('[');
            for (i, f) in v.iter().enumerate() {
                if i > 0 {
                    buf.push(',');
                }
                write_json_str(&mut buf, &format_num_label(*f));
            }
            buf.push(']');
        }
    }

    if let Some(t) = theme {
        buf.push(',');
        write_json_str(&mut buf, "theme");
        buf.push(':');
        write_json_str(&mut buf, t);
    }

    if let Some(kw) = kwargs {
        for (k, v) in kw.iter() {
            if let Ok(key) = k.extract::<String>() {
                buf.push(',');
                write_json_str(&mut buf, &key);
                buf.push(':');
                buf.push_str(&py_any_to_json(&v).to_string());
            }
        }
    }

    buf.push('}');
    buf
}

pub fn python_chart_from_html(html: String, doc: &'static str) -> crate::Chart {
    if doc.is_empty() {
        crate::Chart::new(html)
    } else {
        crate::Chart::new_doc(html, doc)
    }
}

#[inline]
fn format_num_label(f: f64) -> String {
    if f.fract() == 0.0 && f.abs() < 1e15 {
        (f as i64).to_string()
    } else {
        f.to_string()
    }
}

fn py_any_to_json(v: &Bound<'_, PyAny>) -> serde_json::Value {
    if let Ok(d) = v.downcast::<PyDict>() {
        let mut map = serde_json::Map::new();
        for (k, val) in d.iter() {
            if let Ok(key) = k.extract::<String>() {
                map.insert(key, py_any_to_json(&val));
            }
        }
        return serde_json::Value::Object(map);
    }
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
    let target = crate::bindings::alias_registry::resolve_call_target(name);
    match crate::bindings::fn_registry::find(&target) {
        Some(entry) => Ok(crate::bindings::fn_registry::invoke(entry, json)),
        None => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "seraplot: unknown function '{}'",
            name
        ))),
    }
}

pub(crate) enum NumSource<'py, T: numpy::Element> {
    Owned(Vec<T>),
    Zerocopy(numpy::PyReadonlyArray1<'py, T>),
}

impl<'py, T: numpy::Element> NumSource<'py, T> {
    pub(crate) fn as_slice(&self) -> &[T] {
        match self {
            NumSource::Owned(v) => v.as_slice(),
            NumSource::Zerocopy(a) => a.as_slice().unwrap_or(&[]),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.as_slice().len()
    }
}

pub(crate) fn extract_num_source<'py, T>(v: &Bound<'py, PyAny>) -> Option<NumSource<'py, T>>
where
    T: numpy::Element,
    Vec<T>: pyo3::FromPyObject<'py>,
{
    if let Ok(arr) = v.extract::<numpy::PyReadonlyArray1<'py, T>>() {
        return Some(NumSource::Zerocopy(arr));
    }
    v.extract::<Vec<T>>().ok().map(NumSource::Owned)
}

fn kwarg_num_source<'py, T>(kwargs: Option<&Bound<'py, PyDict>>, key: &str) -> Option<NumSource<'py, T>>
where
    T: numpy::Element,
    Vec<T>: pyo3::FromPyObject<'py>,
{
    let item = kwargs?.get_item(key).ok()??;
    extract_num_source(&item)
}

#[pyfunction]
#[pyo3(signature = (name, title="", labels=None, values=None, theme=None, **kwargs))]
fn _sera_call_fast(
    name: &str,
    title: &str,
    labels: Option<&Bound<'_, PyAny>>,
    values: Option<&Bound<'_, PyAny>>,
    theme: Option<&str>,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<String> {
    let target = crate::bindings::alias_registry::resolve_call_target(name);

    for entry in inventory::iter::<crate::plot::canvas_points::NativeChartEntry>() {
        if entry.name != target {
            continue;
        }
        let Some(y) = values
            .and_then(extract_num_source)
            .or_else(|| kwarg_num_source(kwargs, "y"))
        else {
            break;
        };
        if y.len() <= entry.threshold {
            break;
        }
        let x = native_x_values(kwargs, labels, y.len());
        let categories: Vec<String> = kwargs
            .and_then(|k| k.get_item("categories").ok().flatten())
            .and_then(|v| v.extract::<Vec<String>>().ok())
            .unwrap_or_default();
        let variant = kwarg::<String>(kwargs, "variant").unwrap_or_default();
        let opts = crate::plot::canvas_points::NativeChartOpts {
            width: kwarg::<i32>(kwargs, "width").unwrap_or(900),
            height: kwarg::<i32>(kwargs, "height").unwrap_or(500),
            color_hex: kwarg::<u32>(kwargs, "color_hex").unwrap_or(0),
            gridlines: kwarg::<bool>(kwargs, "gridlines").unwrap_or(true),
            x_label: &kwarg::<String>(kwargs, "x_label").unwrap_or_default(),
            y_label: &kwarg::<String>(kwargs, "y_label").unwrap_or_default(),
            show_regression: kwarg::<bool>(kwargs, "show_regression").unwrap_or(false),
            regression_type: &kwarg::<String>(kwargs, "regression_type")
                .unwrap_or_else(|| "linear".to_string()),
            cols: 0,
            categories: &categories,
            variant: &variant,
        };
        let (html, hid) = (entry.render)(title, x.as_slice(), y.as_slice(), &opts);
        #[cfg(feature = "sera-pulse")]
        {
            let meta = crate::plot::push_registry::PushMeta::from_xy(x.as_slice(), y.as_slice(), opts.width, opts.height);
            crate::plot::push_registry::register(hid, meta);
            crate::plot::chart_source_registry::register(hid, entry.name, python_py_args_to_json(title, labels, values, theme, kwargs));
        }
        #[cfg(not(feature = "sera-pulse"))]
        let _ = hid;
        return Ok(html);
    }

    for entry in inventory::iter::<crate::plot::canvas_points::LabeledChartEntry>() {
        if entry.name != target {
            continue;
        }
        let Some(v) = values
            .and_then(extract_num_source)
            .or_else(|| kwarg_num_source(kwargs, "values"))
        else {
            break;
        };
        if v.len() <= entry.threshold {
            break;
        }
        let Some(lbls) = labels.and_then(|l| l.extract::<Vec<String>>().ok()) else {
            break;
        };
        let col_lbls: Vec<String> = kwargs
            .and_then(|k| k.get_item("col_labels").ok().flatten())
            .and_then(|v| v.extract::<Vec<String>>().ok())
            .unwrap_or_default();
        let cols = col_lbls.len() as i32;
        let shape_ok = if cols > 0 {
            (lbls.len() as i64) * (cols as i64) >= v.len() as i64
        } else {
            lbls.len() >= v.len()
        };
        if !shape_ok {
            break;
        }
        let variant = kwarg::<String>(kwargs, "variant").unwrap_or_default();
        let opts = crate::plot::canvas_points::NativeChartOpts {
            width: kwarg::<i32>(kwargs, "width").unwrap_or(900),
            height: kwarg::<i32>(kwargs, "height").unwrap_or(500),
            color_hex: kwarg::<u32>(kwargs, "color_hex").unwrap_or(0),
            gridlines: kwarg::<bool>(kwargs, "gridlines").unwrap_or(true),
            x_label: &kwarg::<String>(kwargs, "x_label").unwrap_or_default(),
            y_label: &kwarg::<String>(kwargs, "y_label").unwrap_or_default(),
            show_regression: false,
            regression_type: "linear",
            cols,
            categories: &[],
            variant: &variant,
        };
        let (html, hid) = (entry.render)(title, &lbls, v.as_slice(), &opts);
        #[cfg(feature = "sera-pulse")]
        {
            let axis_px = if opts.cols > 0 { 1000 } else { crate::plot::default::bar::bar_plot_h(opts.height) };
            let meta = crate::plot::push_registry::PushMeta::from_values(v.as_slice(), axis_px);
            crate::plot::push_registry::register(hid, meta);
            crate::plot::chart_source_registry::register(hid, entry.name, python_py_args_to_json(title, labels, values, theme, kwargs));
        }
        #[cfg(not(feature = "sera-pulse"))]
        let _ = hid;
        return Ok(html);
    }

    for entry in inventory::iter::<crate::plot::canvas_points::OhlcChartEntry>() {
        if entry.name != target {
            continue;
        }
        let Some(open) = kwarg_num_source::<f64>(kwargs, "open") else {
            break;
        };
        let Some(high) = kwarg_num_source::<f64>(kwargs, "high") else {
            break;
        };
        let Some(low) = kwarg_num_source::<f64>(kwargs, "low") else {
            break;
        };
        let Some(close) = kwarg_num_source::<f64>(kwargs, "close") else {
            break;
        };
        if close.len() <= entry.threshold {
            break;
        }
        let lbls: Vec<String> = labels.and_then(|l| l.extract::<Vec<String>>().ok()).unwrap_or_default();
        let opts = crate::plot::canvas_points::NativeChartOpts {
            width: kwarg::<i32>(kwargs, "width").unwrap_or(1100),
            height: kwarg::<i32>(kwargs, "height").unwrap_or(500),
            color_hex: kwarg::<u32>(kwargs, "color_hex").unwrap_or(0),
            gridlines: kwarg::<bool>(kwargs, "gridlines").unwrap_or(true),
            x_label: &kwarg::<String>(kwargs, "x_label").unwrap_or_default(),
            y_label: &kwarg::<String>(kwargs, "y_label").unwrap_or_default(),
            show_regression: false,
            regression_type: "linear",
            cols: 0,
            categories: &[],
            variant: "",
        };
        let (html, hid) = (entry.render)(title, &lbls, open.as_slice(), high.as_slice(), low.as_slice(), close.as_slice(), &opts);
        #[cfg(feature = "sera-pulse")]
        {
            let plot_h = (opts.height - 36 - 48).max(10);
            let meta = crate::plot::push_registry::PushMeta::from_vector_shared_scale(
                &[open.as_slice(), high.as_slice(), low.as_slice(), close.as_slice()],
                plot_h,
            );
            crate::plot::push_registry::register(hid, meta);
            crate::plot::chart_source_registry::register(hid, entry.name, python_py_args_to_json(title, labels, values, theme, kwargs));
        }
        #[cfg(not(feature = "sera-pulse"))]
        let _ = hid;
        return Ok(html);
    }

    for entry in inventory::iter::<crate::plot::canvas_points::HierarchyChartEntry>() {
        if entry.name != target {
            continue;
        }
        let Some(v) = values
            .and_then(extract_num_source)
            .or_else(|| kwarg_num_source(kwargs, "values"))
        else {
            break;
        };
        if v.len() <= entry.threshold {
            break;
        }
        let Some(lbls) = labels.and_then(|l| l.extract::<Vec<String>>().ok()) else {
            break;
        };
        let Some(parents) = kwargs
            .and_then(|k| k.get_item("parents").ok().flatten())
            .and_then(|p| p.extract::<Vec<String>>().ok())
        else {
            break;
        };
        if lbls.len() < v.len() || parents.len() < v.len() {
            break;
        }
        let opts = crate::plot::canvas_points::NativeChartOpts {
            width: kwarg::<i32>(kwargs, "width").unwrap_or(760),
            height: kwarg::<i32>(kwargs, "height").unwrap_or(520),
            color_hex: kwarg::<u32>(kwargs, "color_hex").unwrap_or(0),
            gridlines: kwarg::<bool>(kwargs, "gridlines").unwrap_or(true),
            x_label: &kwarg::<String>(kwargs, "x_label").unwrap_or_default(),
            y_label: &kwarg::<String>(kwargs, "y_label").unwrap_or_default(),
            show_regression: false,
            regression_type: "linear",
            cols: 0,
            categories: &[],
            variant: "",
        };
        let (html, hid, min_val, range_val) = (entry.render)(title, &lbls, &parents, v.as_slice(), &opts);
        #[cfg(feature = "sera-pulse")]
        {
            let meta = crate::plot::push_registry::PushMeta::Scalar(crate::plot::push_registry::ScalarMeta {
                min_val,
                range_val,
                axis_px: 1000,
            });
            crate::plot::push_registry::register(hid, meta);
            crate::plot::chart_source_registry::register(hid, entry.name, python_py_args_to_json(title, labels, values, theme, kwargs));
        }
        #[cfg(not(feature = "sera-pulse"))]
        {
            let _ = hid;
            let _ = min_val;
            let _ = range_val;
        }
        return Ok(html);
    }

    for entry in inventory::iter::<crate::plot::canvas_points::SeriesChartEntry>() {
        if entry.name != target {
            continue;
        }
        let Some(series) = kwargs.and_then(|k| k.get_item("series").ok().flatten()).and_then(|s| s.extract::<Vec<Vec<f64>>>().ok()) else {
            break;
        };
        if series.is_empty() {
            break;
        }
        let total_points: usize = series.iter().map(|s| s.len()).sum();
        if total_points <= entry.threshold {
            break;
        }
        let payload = python_py_args_to_json(title, labels, values, theme, kwargs);
        let Some(fe) = crate::bindings::fn_registry::find(&target) else {
            break;
        };
        let html = crate::bindings::fn_registry::invoke(fe, &payload);
        #[cfg(feature = "sera-pulse")]
        if let Some(hid) = crate::plot::push_registry::extract_hid(&html) {
            let refs: Vec<&[f64]> = series.iter().map(|s| s.as_slice()).collect();
            let height = kwarg::<i32>(kwargs, "height").unwrap_or(480);
            let plot_h = (height - 36 - 48).max(10);
            let meta = crate::plot::push_registry::PushMeta::from_vector_shared_scale(&refs, plot_h);
            crate::plot::push_registry::register(hid, meta);
            crate::plot::chart_source_registry::register(hid, entry.name, payload);
        }
        return Ok(html);
    }

    match crate::bindings::fn_registry::find(&target) {
        Some(entry) => {
            let payload = python_py_args_to_json(title, labels, values, theme, kwargs);
            Ok(crate::bindings::fn_registry::invoke(entry, &payload))
        }
        None => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "seraplot: unknown function '{}'",
            name
        ))),
    }
}

fn kwarg<T: for<'a> pyo3::FromPyObject<'a>>(kwargs: Option<&Bound<'_, PyDict>>, key: &str) -> Option<T> {
    kwargs?.get_item(key).ok()??.extract::<T>().ok()
}

fn native_x_values<'py>(
    kwargs: Option<&Bound<'py, PyDict>>,
    labels: Option<&Bound<'py, PyAny>>,
    len: usize,
) -> NumSource<'py, f64> {
    kwarg_num_source(kwargs, "x")
        .or_else(|| labels.and_then(extract_num_source))
        .unwrap_or_else(|| NumSource::Owned((0..len).map(|i| i as f64).collect()))
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
#[pyo3(signature = (chart = None, variant = None))]
fn _true_required_params_json(chart: Option<&str>, variant: Option<&str>) -> String {
    serde_json::to_string(&crate::true_required_params(chart, variant)).unwrap_or_default()
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
fn _map_regions_json() -> String {
    serde_json::to_string(&crate::map_regions()).unwrap_or_default()
}

#[pyfunction]
fn _docs_json() -> String {
    serde_json::to_string(&crate::doc_registry::all_docs()).unwrap_or_default()
}

#[pyfunction]
fn _chart_methods_json() -> String {
    let names: Vec<&str> = crate::bindings::method_registry::iter_entries().map(|e| e.name).collect();
    serde_json::to_string(&names).unwrap_or_default()
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
fn _alias_load(path: Option<&str>) -> PyResult<bool> {
    crate::bindings::alias_registry::load_from_disk(path)
        .map_err(pyo3::exceptions::PyOSError::new_err)
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

impl crate::Chart {
    fn show_impl(&self, py: Python<'_>) -> PyResult<()> {
        let ipython = py.import_bound("IPython.display")?;
        let html_cls = ipython.getattr("HTML")?;
        let display_fn = ipython.getattr("display")?;
        let html_obj = html_cls.call1((self.chart_iframe().as_str(),))?;
        display_fn.call1((html_obj,))?;
        Ok(())
    }
}

#[pymethods]
impl crate::Chart {
    #[new]
    #[pyo3(signature = (html = String::new()))]
    fn py_new(html: String) -> Self {
        crate::Chart::new(html)
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
        self.show_impl(py)
    }

    fn display(&self, py: Python<'_>) -> PyResult<()> {
        self.show_impl(py)
    }

    fn render(&self, py: Python<'_>) -> PyResult<()> {
        self.show_impl(py)
    }

    fn view(&self, py: Python<'_>) -> PyResult<()> {
        self.show_impl(py)
    }

    fn plot(&self, py: Python<'_>) -> PyResult<()> {
        self.show_impl(py)
    }

    fn preview(&self, py: Python<'_>) -> PyResult<()> {
        self.show_impl(py)
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
    m.add_class::<crate::plot::dashboard::Canvas>()?;
    m.add_function(wrap_pyfunction!(crate::plot::dashboard::canvas, m)?)?;
    m.add_function(wrap_pyfunction!(crate::plot::dashboard::canvas_load, m)?)?;
    m.add_function(wrap_pyfunction!(crate::plot::dashboard::canvas_save_named, m)?)?;
    m.add_function(wrap_pyfunction!(crate::plot::dashboard::canvas_load_named, m)?)?;
    m.add_function(wrap_pyfunction!(crate::plot::dashboard::canvas_default_dir, m)?)?;
    m.add_class::<crate::PyDataset>()?;
    m.add_class::<crate::data::Table>()?;
    m.add_class::<crate::data::SeraDFrame_>()?;
    m.add_class::<crate::data::SeraDFrameGroupBy>()?;
    m.add_class::<crate::data::DFrameBuilder>()?;
    #[cfg(feature = "sera-secure")]
    m.add_class::<crate::data::SecureDFrame_>()?;
    #[cfg(feature = "sera-secure")]
    m.add_class::<crate::data::SecureDFrameBuilder>()?;
    #[cfg(feature = "sera-secure")]
    m.add_class::<crate::data::SeraKey_>()?;
    m.add_class::<crate::PyDatasetStats>()?;
    m.add_class::<crate::LiveStream>()?;
    #[cfg(feature = "webapp")]
    m.add_class::<crate::webapp::App>()?;
    #[cfg(feature = "sera-pulse")]
    m.add_function(wrap_pyfunction!(crate::plot::pulse::chart_methods::pulse_activate, m)?)?;
    #[cfg(feature = "sera-pulse")]
    m.add_function(wrap_pyfunction!(crate::plot::pulse::chart_methods::pulse_status, m)?)?;
    #[cfg(feature = "sera-pulse")]
    m.add_function(wrap_pyfunction!(crate::plot::pulse::chart_methods::pulse_machine_id, m)?)?;
    #[cfg(feature = "sera-pulse")]
    m.add_function(wrap_pyfunction!(crate::plot::pulse::features::metrics::pulse_serve_metrics, m)?)?;
    #[cfg(feature = "sera-pulse")]
    m.add_function(wrap_pyfunction!(crate::plot::pulse::features::health::pulse_serve_health, m)?)?;
    #[cfg(feature = "sera-pulse")]
    m.add_function(wrap_pyfunction!(crate::plot::pulse::features::health::pulse_health, m)?)?;
    #[cfg(feature = "sera-report")]
    m.add_function(wrap_pyfunction!(crate::plot::pulse::features::report::export_pdf_report, m)?)?;
    #[cfg(feature = "sera-pulse")]
    m.add_function(wrap_pyfunction!(crate::plot::pulse::features::board::export_board_html, m)?)?;
    #[cfg(all(feature = "sera-pulse", feature = "webapp"))]
    m.add_function(wrap_pyfunction!(crate::plot::pulse::chart_methods::serve_board, m)?)?;
    m.add_function(wrap_pyfunction!(crate::theme, m)?)?;
    m.add_function(wrap_pyfunction!(crate::config, m)?)?;
    m.add_function(wrap_pyfunction!(_sera_call, m)?)?;
    m.add_function(wrap_pyfunction!(_sera_call_fast, m)?)?;
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
    m.add_function(wrap_pyfunction!(_true_required_params_json, m)?)?;
    m.add_function(wrap_pyfunction!(_chart_variants_json, m)?)?;
    m.add_function(wrap_pyfunction!(_chart_themes_json, m)?)?;
    m.add_function(wrap_pyfunction!(_map_regions_json, m)?)?;
    m.add_function(wrap_pyfunction!(_scenes3d_json, m)?)?;
    m.add_function(wrap_pyfunction!(_docs_json, m)?)?;
    m.add_function(wrap_pyfunction!(_chart_methods_json, m)?)?;
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
