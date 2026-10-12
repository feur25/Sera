pub mod alerts;
pub mod annotations;
pub mod board;
pub mod downsample;
pub mod health;
pub mod history;
pub mod incident_report;
pub mod metrics;
pub mod rate_limit;
pub mod remote_push;
#[cfg(feature = "sera-report")]
pub mod report;
pub(crate) use super::studio;
pub(crate) use super::token_bucket;
pub(crate) use super::frame_codec::atomic_write;

pub(crate) fn require_hid(html: &str) -> pyo3::PyResult<u64> {
    crate::plot::push_registry::extract_hid(html)
        .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("this method requires a chart built via one of the framework's native fast paths"))
}
