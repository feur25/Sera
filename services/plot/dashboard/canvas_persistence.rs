use super::canvas_core::Canvas;
use pyo3::prelude::*;
use std::fs;

#[pymethods]
impl Canvas {
    pub fn to_json(&self) -> PyResult<String> {
        serde_json::to_string_pretty(&self.to_state())
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    pub fn save(&self, path: &str) -> PyResult<()> {
        if let Some(parent) = std::path::Path::new(path).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|e| {
                    pyo3::exceptions::PyIOError::new_err(format!(
                        "cannot create directory for '{}': {}",
                        path, e
                    ))
                })?;
            }
        }
        let json = self.to_json()?;
        fs::write(path, json).map_err(|e| {
            pyo3::exceptions::PyIOError::new_err(format!("cannot write '{}': {}", path, e))
        })
    }
}
