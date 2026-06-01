use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use pyo3::types::PyAny;

pub mod charts;
pub mod ml;
pub mod root;
pub mod telemetry;

pub use charts::*;
#[allow(unused_imports)]
pub use ml::*;
pub use root::*;
pub use telemetry::*;

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
    charts.iter().map(|chart| chart.html.clone()).collect()
}

pub fn python_extract_chart_html(html: &pyo3::Bound<'_, pyo3::types::PyAny>) -> pyo3::PyResult<String> {
    if let Ok(chart) = html.extract::<pyo3::PyRef<'_, crate::Chart>>() {
        Ok(chart.html.clone())
    } else {
        html.extract::<String>()
    }
}

pub fn python_py_any_to_json(val: &pyo3::Bound<'_, pyo3::types::PyAny>) -> serde_json::Value {
    if val.is_none() {
        return serde_json::Value::Null;
    }
    if let Ok(boolean) = val.extract::<bool>() {
        return serde_json::Value::Bool(boolean);
    }
    if let Ok(integer) = val.extract::<i64>() {
        return serde_json::json!(integer);
    }
    if let Ok(number) = val.extract::<f64>() {
        return serde_json::json!(number);
    }
    if let Ok(text) = val.extract::<String>() {
        return serde_json::Value::String(text);
    }
    if let Ok(list) = val.extract::<Vec<Vec<f64>>>() {
        return serde_json::json!(list);
    }
    if let Ok(list) = val.extract::<Vec<i64>>() {
        return serde_json::json!(list);
    }
    if let Ok(list) = val.extract::<Vec<f64>>() {
        return serde_json::json!(list);
    }
    if let Ok(list) = val.extract::<Vec<String>>() {
        return serde_json::json!(list);
    }
    let py = val.py();
    if let Ok(json_mod) = py.import_bound("json") {
        if let Ok(serialized) = json_mod.call_method1("dumps", (val,)) {
            if let Ok(text) = serialized.extract::<String>() {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                    return json;
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
    let mut map = serde_json::Map::new();
    if !title.is_empty() {
        map.insert("title".into(), serde_json::json!(title));
    }
    if let Some(value) = labels {
        let json = python_py_any_to_json(value);
        if json != serde_json::Value::Null {
            map.insert("labels".into(), json);
        }
    }
    if let Some(value) = values {
        if let Ok(series) = value.extract::<Vec<Vec<f64>>>() {
            map.insert("series".into(), serde_json::json!(series));
        } else {
            let json = python_py_any_to_json(value);
            if json != serde_json::Value::Null {
                map.insert("values".into(), json);
            }
        }
    }
    if let Some(theme_name) = theme {
        map.insert("theme".into(), serde_json::json!(theme_name));
    }
    if let Some(dict) = kwargs {
        for (key, value) in dict.iter() {
            if let Ok(text) = key.str() {
                let key_text = text.to_string_lossy().to_string();
                let json = python_py_any_to_json(&value);
                if json != serde_json::Value::Null {
                    map.insert(key_text, json);
                }
            }
        }
    }
    serde_json::Value::Object(map).to_string()
}

pub(crate) fn extract_f64_vec(_py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<Vec<f64>> {
    if let Ok(values) = obj.extract::<Vec<f64>>() {
        return Ok(values);
    }
    if let Ok(array) = obj.extract::<PyReadonlyArray1<f64>>() {
        return Ok(array.as_slice()?.to_vec());
    }
    if let Ok(array) = obj.extract::<PyReadonlyArray1<f32>>() {
        return Ok(array.as_slice()?.iter().map(|&value| value as f64).collect());
    }
    if let Ok(to_numpy) = obj.getattr("to_numpy") {
        let array = to_numpy.call0()?;
        if let Ok(list) = array.call_method0("tolist")?.extract::<Vec<f64>>() {
            return Ok(list);
        }
    }
    if let Ok(values) = obj.getattr("values") {
        if let Ok(to_list) = values.getattr("tolist") {
            if let Ok(list) = to_list.call0()?.extract::<Vec<f64>>() {
                return Ok(list);
            }
        }
    }
    Err(pyo3::exceptions::PyTypeError::new_err(
        "Expected list, numpy array, or pandas Series of numbers",
    ))
}

pub(crate) fn merge_global_opts(
    background: Option<&str>,
    palette: Option<Vec<u32>>,
    gridlines: bool,
) -> (Option<String>, Vec<u32>, bool) {
    let background = background.map(|value| value.to_string()).or_else(crate::get_global_background);
    let palette = if palette.as_ref().map(|values| !values.is_empty()).unwrap_or(false) {
        palette.unwrap_or_default()
    } else {
        crate::get_global_palette().unwrap_or_default()
    };
    let gridlines = gridlines || crate::get_global_gridlines();
    (background, palette, gridlines)
}