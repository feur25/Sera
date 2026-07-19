use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
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
use tokio::sync::broadcast;

use super::components::BOOTSTRAP_JS;
use super::server::EventDispatcher;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ComponentKind {
    Slider,
    Checkbox,
    Text,
}

impl ComponentKind {
    fn to_py(self, py: Python<'_>, raw: &str) -> PyObject {
        match self {
            ComponentKind::Slider => raw.parse::<f64>().unwrap_or(0.0).into_py(py),
            ComponentKind::Checkbox => (raw == "true").into_py(py),
            ComponentKind::Text => raw.into_py(py),
        }
    }
}

struct RegisteredCallback {
    input_ids: Vec<String>,
    output_id: String,
    handler: PyObject,
}

struct RegisteredTimer {
    every_secs: f64,
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
    kinds: HashMap<String, ComponentKind>,
    sessions: HashMap<u64, HashMap<String, String>>,
    next_session: AtomicU64,
    callbacks: Vec<RegisteredCallback>,
    timers: Vec<RegisteredTimer>,
    credentials: Option<(String, String)>,
    push_tx: broadcast::Sender<(String, String)>,
}

impl AppState {
    pub fn new(title: &str) -> Self {
        let mut pages = HashMap::new();
        pages.insert("/".to_string(), PageState { title: title.to_string(), layout: Vec::new() });
        let (push_tx, _) = broadcast::channel(64);
        Self {
            app_title: title.to_string(),
            pages,
            current_page: "/".to_string(),
            component_html: HashMap::new(),
            values: HashMap::new(),
            kinds: HashMap::new(),
            sessions: HashMap::new(),
            next_session: AtomicU64::new(1),
            callbacks: Vec::new(),
            timers: Vec::new(),
            credentials: None,
            push_tx,
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

    pub fn add_component(&mut self, id: &str, html: String, initial_value: &str, kind: ComponentKind) {
        if let Some(p) = self.pages.get_mut(&self.current_page) {
            p.layout.push(id.to_string());
        }
        self.component_html.insert(id.to_string(), html);
        self.values.insert(id.to_string(), initial_value.to_string());
        self.kinds.insert(id.to_string(), kind);
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

    pub fn add_timer(&mut self, every_secs: f64, output: &str, handler: PyObject) {
        self.timers.push(RegisteredTimer { every_secs, output_id: output.to_string(), handler });
    }

    pub fn set_auth(&mut self, username: &str, password: &str) {
        self.credentials = Some((username.to_string(), password.to_string()));
    }

    pub fn push(&mut self, id: &str, html: String) {
        self.component_html.insert(id.to_string(), html.clone());
        let _ = self.push_tx.send((format!("{id}-wrap"), html));
    }

    fn new_session(&self) -> u64 {
        self.next_session.fetch_add(1, Ordering::Relaxed)
    }

    fn session_value(&self, session: u64, id: &str) -> String {
        self.sessions
            .get(&session)
            .and_then(|vals| vals.get(id))
            .cloned()
            .or_else(|| self.values.get(id).cloned())
            .unwrap_or_default()
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

fn run_callback(py: Python<'_>, output_id: &str, handler: &PyObject, py_args: &Bound<'_, PyTuple>) -> Option<(String, String)> {
    match handler.call1(py, py_args) {
        Ok(result) => match extract_html(py, &result) {
            Ok(html) => Some((format!("{output_id}-wrap"), html)),
            Err(e) => {
                eprintln!("seraplot webapp: callback for output '{output_id}' did not return renderable html/Chart: {e}");
                None
            }
        },
        Err(e) => {
            e.print(py);
            eprintln!("seraplot webapp: callback for output '{output_id}' raised an exception (see traceback above)");
            None
        }
    }
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

    fn open_session(&self) -> u64 {
        match self.lock() {
            Ok(state) => state.new_session(),
            Err(_) => {
                warn_state_lock_once("open_session");
                0
            }
        }
    }

    fn credentials(&self) -> Option<(String, String)> {
        match self.lock() {
            Ok(state) => state.credentials.clone(),
            Err(_) => None,
        }
    }

    fn subscribe(&self) -> broadcast::Receiver<(String, String)> {
        match self.lock() {
            Ok(state) => state.push_tx.subscribe(),
            Err(_) => broadcast::channel(1).1,
        }
    }

    fn timer_intervals(&self) -> Vec<f64> {
        match self.lock() {
            Ok(state) => state.timers.iter().map(|t| t.every_secs).collect(),
            Err(_) => Vec::new(),
        }
    }

    fn tick_timer(&self, index: usize) {
        Python::with_gil(|py| {
            let mut state = match self.lock() {
                Ok(state) => state,
                Err(_) => {
                    warn_state_lock_once("tick_timer");
                    return;
                }
            };
            let Some(timer) = state.timers.get(index) else { return };
            let (output_id, handler) = (timer.output_id.clone(), timer.handler.clone_ref(py));
            let py_args = PyTuple::empty_bound(py);
            if let Some((_, html)) = run_callback(py, &output_id, &handler, &py_args) {
                state.push(&output_id, html);
            }
        });
    }

    fn on_event(&self, session: u64, component_id: &str, value: &str) -> Vec<(String, String)> {
        let mut updates = Vec::new();
        Python::with_gil(|py| {
            let mut state = match self.lock() {
                Ok(state) => state,
                Err(_) => {
                    warn_state_lock_once("on_event");
                    return;
                }
            };
            state.sessions.entry(session).or_default().insert(component_id.to_string(), value.to_string());

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
                let args: Vec<PyObject> = input_ids
                    .iter()
                    .map(|id| {
                        let raw = state.session_value(session, id);
                        let kind = state.kinds.get(id).copied().unwrap_or(ComponentKind::Text);
                        kind.to_py(py, &raw)
                    })
                    .collect();
                let py_args = PyTuple::new_bound(py, &args);
                if let Some(update) = run_callback(py, &output_id, &handler, &py_args) {
                    state.component_html.insert(output_id, update.1.clone());
                    updates.push(update);
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
                s.add_component("dd", "<select id=\"dd\"></select>".to_string(), "A", ComponentKind::Text);
                s.set_output("out", "<p>initial</p>".to_string());
                let handler: PyObject = py
                    .eval_bound("lambda v: '<p>' + v + '</p>'", None, None)
                    .unwrap()
                    .into();
                s.add_callback(vec!["dd".to_string()], "out", handler);
            }

            let session = EventDispatcher::open_session(&state);
            let updates = EventDispatcher::on_event(&state, session, "dd", "B");
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

    #[test]
    fn two_sessions_do_not_clobber_each_others_input_values() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let state = Mutex::new(AppState::new("Test"));
            {
                let mut s = state.lock().unwrap();
                s.add_component("slider", "<input>".to_string(), "0", ComponentKind::Slider);
                s.set_output("out", "<p>0</p>".to_string());
                let handler: PyObject = py.eval_bound("lambda v: '<p>' + str(v) + '</p>'", None, None).unwrap().into();
                s.add_callback(vec!["slider".to_string()], "out", handler);
            }

            let session_a = EventDispatcher::open_session(&state);
            let session_b = EventDispatcher::open_session(&state);
            EventDispatcher::on_event(&state, session_a, "slider", "10");
            let updates_b = EventDispatcher::on_event(&state, session_b, "slider", "20");

            assert_eq!(updates_b[0].1, "<p>20.0</p>");
            let s = state.lock().unwrap();
            assert_eq!(s.session_value(session_a, "slider"), "10");
            assert_eq!(s.session_value(session_b, "slider"), "20");
        });
    }

    #[test]
    fn typed_slider_argument_arrives_as_a_python_float_not_a_string() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let state = Mutex::new(AppState::new("Test"));
            {
                let mut s = state.lock().unwrap();
                s.add_component("slider", "<input>".to_string(), "5", ComponentKind::Slider);
                s.set_output("out", "<p></p>".to_string());
                let handler: PyObject = py.eval_bound("lambda v: '<p>' + str(v * 2) + '</p>'", None, None).unwrap().into();
                s.add_callback(vec!["slider".to_string()], "out", handler);
            }
            let session = EventDispatcher::open_session(&state);
            let updates = EventDispatcher::on_event(&state, session, "slider", "7.5");
            assert_eq!(updates[0].1, "<p>15.0</p>");
        });
    }
}
