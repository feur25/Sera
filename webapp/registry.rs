use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

static STATE_LOCK_WARNED: AtomicBool = AtomicBool::new(false);

fn warn_state_lock_once(caller: &str) {
    if !STATE_LOCK_WARNED.swap(true, Ordering::Relaxed) {
        eprintln!(
            "seraplot: webapp AppState mutex poisoned in {caller} -- page renders and event \
             callbacks will silently stop updating from here on; this warning prints once"
        );
    }
}

use pyo3::prelude::*;
use pyo3::types::PyTuple;

use super::components::BOOTSTRAP_JS;
use super::server::EventDispatcher;

struct RegisteredCallback {
    input_ids: Vec<String>,
    output_id: String,
    handler: PyObject,
}

struct PageState {
    title: String,
    layout: Vec<String>,
}

pub struct AppState {
    app_title: String,
    pages: HashMap<String, PageState>,
    current_page: String,
    component_html: HashMap<String, String>,
    values: HashMap<String, String>,
    callbacks: Vec<RegisteredCallback>,
}

impl AppState {
    pub fn new(title: &str) -> Self {
        let mut pages = HashMap::new();
        pages.insert("/".to_string(), PageState { title: title.to_string(), layout: Vec::new() });
        Self {
            app_title: title.to_string(),
            pages,
            current_page: "/".to_string(),
            component_html: HashMap::new(),
            values: HashMap::new(),
            callbacks: Vec::new(),
        }
    }

    pub fn page(&mut self, path: &str, title: Option<String>) {
        if !self.pages.contains_key(path) {
            let t = title.clone().unwrap_or_else(|| self.app_title.clone());
            self.pages.insert(path.to_string(), PageState { title: t, layout: Vec::new() });
        } else if let Some(t) = title {
            self.pages.get_mut(path).unwrap().title = t;
        }
        self.current_page = path.to_string();
    }

    pub fn add_component(&mut self, id: &str, html: String, initial_value: &str) {
        if let Some(p) = self.pages.get_mut(&self.current_page) {
            p.layout.push(id.to_string());
        }
        self.component_html.insert(id.to_string(), html);
        self.values.insert(id.to_string(), initial_value.to_string());
    }

    pub fn set_output(&mut self, id: &str, html: String) {
        if let Some(p) = self.pages.get_mut(&self.current_page) {
            if !p.layout.iter().any(|existing| existing == id) {
                p.layout.push(id.to_string());
            }
        }
        self.component_html.insert(id.to_string(), html);
    }

    pub fn add_callback(&mut self, inputs: Vec<String>, output: &str, handler: PyObject) {
        self.callbacks.push(RegisteredCallback { input_ids: inputs, output_id: output.to_string(), handler });
    }

    fn render_page(&self, path: &str) -> Option<String> {
        let page = self.pages.get(path)?;
        let body: String = page
            .layout
            .iter()
            .map(|id| format!("<div id=\"{}-wrap\" role=\"status\" aria-live=\"polite\" aria-atomic=\"true\">{}</div>", id, self.component_html.get(id).cloned().unwrap_or_default()))
            .collect();
        Some(format!(
            "<!doctype html><html><head><meta charset=\"utf-8\"><title>{}</title></head><body>{}<script>{}</script></body></html>",
            page.title, body, BOOTSTRAP_JS
        ))
    }
}

fn extract_html(py: Python<'_>, obj: &PyObject) -> PyResult<String> {
    if let Ok(s) = obj.extract::<String>(py) {
        return Ok(s);
    }
    obj.getattr(py, "html")?.extract::<String>(py)
}

impl EventDispatcher for Mutex<AppState> {
    fn page_html(&self, path: &str) -> Option<String> {
        match self.lock() {
            Ok(state) => state.render_page(path),
            Err(_) => {
                warn_state_lock_once("page_html");
                None
            }
        }
    }

    fn on_event(&self, component_id: &str, value: &str) -> Vec<(String, String)> {
        let mut updates = Vec::new();
        Python::with_gil(|py| {
            let mut state = match self.lock() {
                Ok(state) => state,
                Err(_) => {
                    warn_state_lock_once("on_event");
                    return;
                }
            };
            state.values.insert(component_id.to_string(), value.to_string());

            let matching: Vec<usize> = state
                .callbacks
                .iter()
                .enumerate()
                .filter(|(_, cb)| cb.input_ids.iter().any(|i| i == component_id))
                .map(|(i, _)| i)
                .collect();

            for i in matching {
                let (input_ids, output_id, handler) = {
                    let cb = &state.callbacks[i];
                    (cb.input_ids.clone(), cb.output_id.clone(), cb.handler.clone_ref(py))
                };
                let args: Vec<String> = input_ids.iter().map(|id| state.values.get(id).cloned().unwrap_or_default()).collect();
                let py_args = PyTuple::new_bound(py, &args);
                if let Ok(result) = handler.call1(py, py_args) {
                    if let Ok(html) = extract_html(py, &result) {
                        state.component_html.insert(output_id.clone(), html.clone());
                        // render_page() only ever emits the DOM id "{output_id}-wrap" (see
                        // below); the live-update message must target that same id or the
                        // client's getElementById lookup silently finds nothing.
                        updates.push((format!("{output_id}-wrap"), html));
                    }
                }
            }
        });
        updates
    }
}

#[cfg(test)]
mod live_update_dom_id_tests {
    use super::*;

    #[test]
    fn on_event_update_id_matches_an_id_actually_present_in_the_rendered_page() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let state = Mutex::new(AppState::new("Test"));
            {
                let mut s = state.lock().unwrap();
                s.add_component("dd", "<select id=\"dd\"></select>".to_string(), "A");
                s.set_output("out", "<p>initial</p>".to_string());
                let handler: PyObject = py
                    .eval_bound("lambda v: '<p>' + v + '</p>'", None, None)
                    .unwrap()
                    .into();
                s.add_callback(vec!["dd".to_string()], "out", handler);
            }

            let updates = EventDispatcher::on_event(&state, "dd", "B");
            assert_eq!(updates.len(), 1);
            let (update_id, _html) = &updates[0];

            let page = EventDispatcher::page_html(&state, "/").unwrap();
            assert!(
                page.contains(&format!("id=\"{update_id}\"")),
                "the id targeted by a live update ({update_id}) must exist in the initial page, \
                 otherwise the browser's getElementById lookup silently finds nothing and the \
                 update never reaches the DOM. Page: {page}"
            );
        });
    }
}
