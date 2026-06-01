use pyo3::prelude::*;

#[crate::sera_doc(category = "telemetry", file = "about/telemetry.md", en = "Uploads pending telemetry events to the specified endpoint with an auth token.", fr = "Televerse les evenements de telemetrie en attente vers l'endpoint specifie avec un jeton d'authentification.", param(name = "endpoint", ty = "str", en = "HTTPS URL of the telemetry collection endpoint.", fr = "URL HTTPS de l'endpoint de collecte de telemetrie."), param(name = "token", ty = "str", en = "Authentication token for the endpoint.", fr = "Jeton d'authentification pour l'endpoint."))]
#[pyfunction]
#[pyo3(signature = (endpoint, token))]
pub fn push_telemetry(py: Python<'_>, endpoint: &str, token: &str) -> PyResult<usize> {
    py.allow_threads(|| crate::telemetry::push_pending_to_endpoint(endpoint, token))
        .map_err(pyo3::exceptions::PyRuntimeError::new_err)
}