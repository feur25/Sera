use std::sync::{Arc, Mutex};

use pyo3::prelude::*;

use super::components::{button_html, checkbox_html, dropdown_html, slider_html, text_input_html};
use super::registry::AppState;
use super::server::run_server;

#[pyclass(name = "App", module = "seraplot")]
pub struct App {
    state: Arc<Mutex<AppState>>,
}

#[pymethods]
impl App {
    #[new]
    #[pyo3(signature = (title = "SeraPlot App"))]
    fn new(title: &str) -> Self {
        App { state: Arc::new(Mutex::new(AppState::new(title))) }
    }

    #[pyo3(signature = (path, title = None))]
    fn page(slf: PyRefMut<'_, Self>, path: String, title: Option<String>) -> PyRefMut<'_, Self> {
        if let Ok(mut state) = slf.state.lock() {
            state.page(&path, title);
        }
        slf
    }

    #[pyo3(signature = (id, options, value = None))]
    fn dropdown(slf: PyRefMut<'_, Self>, id: String, options: Vec<String>, value: Option<String>) -> PyRefMut<'_, Self> {
        let selected = value.unwrap_or_else(|| options.first().cloned().unwrap_or_default());
        let html = dropdown_html(&id, &options, &selected);
        if let Ok(mut state) = slf.state.lock() {
            state.add_component(&id, html, &selected);
        }
        slf
    }

    #[pyo3(signature = (id, min, max, step = 1.0, value = None))]
    fn slider(slf: PyRefMut<'_, Self>, id: String, min: f64, max: f64, step: f64, value: Option<f64>) -> PyRefMut<'_, Self> {
        let v = value.unwrap_or(min);
        let html = slider_html(&id, min, max, step, v);
        if let Ok(mut state) = slf.state.lock() {
            state.add_component(&id, html, &v.to_string());
        }
        slf
    }

    #[pyo3(signature = (id, label))]
    fn button(slf: PyRefMut<'_, Self>, id: String, label: String) -> PyRefMut<'_, Self> {
        let html = button_html(&id, &label);
        if let Ok(mut state) = slf.state.lock() {
            state.add_component(&id, html, "");
        }
        slf
    }

    #[pyo3(signature = (id, value = String::new(), placeholder = String::new()))]
    fn text_input(slf: PyRefMut<'_, Self>, id: String, value: String, placeholder: String) -> PyRefMut<'_, Self> {
        let html = text_input_html(&id, &value, &placeholder);
        if let Ok(mut state) = slf.state.lock() {
            state.add_component(&id, html, &value);
        }
        slf
    }

    #[pyo3(signature = (id, label, checked = false))]
    fn checkbox(slf: PyRefMut<'_, Self>, id: String, label: String, checked: bool) -> PyRefMut<'_, Self> {
        let html = checkbox_html(&id, &label, checked);
        if let Ok(mut state) = slf.state.lock() {
            state.add_component(&id, html, if checked { "true" } else { "false" });
        }
        slf
    }

    #[pyo3(signature = (id, html = String::new()))]
    fn chart(slf: PyRefMut<'_, Self>, id: String, html: String) -> PyRefMut<'_, Self> {
        if let Ok(mut state) = slf.state.lock() {
            state.set_output(&id, html);
        }
        slf
    }

    #[pyo3(signature = (inputs, output, handler))]
    fn add_callback(slf: PyRefMut<'_, Self>, inputs: Vec<String>, output: String, handler: PyObject) -> PyRefMut<'_, Self> {
        if let Ok(mut state) = slf.state.lock() {
            state.add_callback(inputs, &output, handler);
        }
        slf
    }

    #[pyo3(signature = (port = 8787, host = "127.0.0.1"))]
    fn serve(&self, py: Python<'_>, port: u16, host: &str) -> PyResult<()> {
        let state = self.state.clone();
        let addr = format!("{host}:{port}");
        py.allow_threads(move || {
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            rt.block_on(async move {
                run_server(&addr, state)
                    .await
                    .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
            })
        })
    }
}
