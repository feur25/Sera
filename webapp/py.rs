use std::sync::{Arc, Mutex};

use pyo3::prelude::*;

use super::components::{button_html, checkbox_html, dropdown_html, slider_html, text_input_html};
use super::registry::{AppState, ComponentKind};
use super::server::run_server;

#[pyclass(name = "App", module = "seraplot")]
pub struct App {
    state: Arc<Mutex<AppState>>,
}

impl App {
    fn with_state(&self, caller: &str, f: impl FnOnce(&mut AppState)) {
        match self.state.lock() {
            Ok(mut state) => f(&mut state),
            Err(_) => eprintln!("seraplot webapp: App.{caller} skipped, state lock poisoned"),
        }
    }
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
        slf.with_state("page", |state| state.page(&path, title));
        slf
    }

    #[pyo3(signature = (id, options, value = None))]
    fn dropdown(slf: PyRefMut<'_, Self>, id: String, options: Vec<String>, value: Option<String>) -> PyRefMut<'_, Self> {
        let selected = value.unwrap_or_else(|| options.first().cloned().unwrap_or_default());
        let html = dropdown_html(&id, &options, &selected);
        slf.with_state("dropdown", |state| state.add_component(&id, html, &selected, ComponentKind::Text));
        slf
    }

    #[pyo3(signature = (id, min, max, step = 1.0, value = None))]
    fn slider(slf: PyRefMut<'_, Self>, id: String, min: f64, max: f64, step: f64, value: Option<f64>) -> PyRefMut<'_, Self> {
        let v = value.unwrap_or(min);
        let html = slider_html(&id, min, max, step, v);
        slf.with_state("slider", |state| state.add_component(&id, html, &v.to_string(), ComponentKind::Slider));
        slf
    }

    #[pyo3(signature = (id, label))]
    fn button(slf: PyRefMut<'_, Self>, id: String, label: String) -> PyRefMut<'_, Self> {
        let html = button_html(&id, &label);
        slf.with_state("button", |state| state.add_component(&id, html, "", ComponentKind::Text));
        slf
    }

    #[pyo3(signature = (id, value = String::new(), placeholder = String::new()))]
    fn text_input(slf: PyRefMut<'_, Self>, id: String, value: String, placeholder: String) -> PyRefMut<'_, Self> {
        let html = text_input_html(&id, &value, &placeholder);
        slf.with_state("text_input", |state| state.add_component(&id, html, &value, ComponentKind::Text));
        slf
    }

    #[pyo3(signature = (id, label, checked = false))]
    fn checkbox(slf: PyRefMut<'_, Self>, id: String, label: String, checked: bool) -> PyRefMut<'_, Self> {
        let html = checkbox_html(&id, &label, checked);
        slf.with_state("checkbox", |state| {
            state.add_component(&id, html, if checked { "true" } else { "false" }, ComponentKind::Checkbox)
        });
        slf
    }

    #[pyo3(signature = (id, html = String::new()))]
    fn chart(slf: PyRefMut<'_, Self>, id: String, html: String) -> PyRefMut<'_, Self> {
        slf.with_state("chart", |state| state.set_output(&id, html));
        slf
    }

    #[pyo3(signature = (inputs, output, handler))]
    fn add_callback(slf: PyRefMut<'_, Self>, inputs: Vec<String>, output: String, handler: PyObject) -> PyRefMut<'_, Self> {
        slf.with_state("add_callback", |state| state.add_callback(inputs, &output, handler));
        slf
    }

    #[pyo3(signature = (seconds, output, handler))]
    fn interval(slf: PyRefMut<'_, Self>, seconds: f64, output: String, handler: PyObject) -> PyRefMut<'_, Self> {
        slf.with_state("interval", |state| state.add_timer(seconds, &output, handler));
        slf
    }

    #[pyo3(signature = (username, password))]
    fn auth(slf: PyRefMut<'_, Self>, username: String, password: String) -> PyRefMut<'_, Self> {
        slf.with_state("auth", |state| state.set_auth(&username, &password));
        slf
    }

    #[pyo3(signature = (id, html))]
    fn push(&self, py: Python<'_>, id: String, html: PyObject) -> PyResult<()> {
        let html = if let Ok(s) = html.extract::<String>(py) {
            s
        } else {
            html.getattr(py, "html")?.extract::<String>(py)?
        };
        self.with_state("push", |state| state.push(&id, html));
        Ok(())
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
