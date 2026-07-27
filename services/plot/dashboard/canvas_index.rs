use super::canvas_core::{Canvas, CanvasState};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[pyfunction]
#[pyo3(signature = (width, height, bg = "#0a0a0f"))]
pub fn canvas(width: u32, height: u32, bg: &str) -> Canvas {
    Canvas::new(width, height, bg)
}

fn seraplot_home_dir() -> std::path::PathBuf {
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    std::path::Path::new(&home).join(".seraplot").join("canvas")
}

fn canvas_index_path() -> std::path::PathBuf {
    seraplot_home_dir().join("index.json")
}

#[derive(Serialize, Deserialize, Default)]
struct CanvasIndexEntry {
    path: String,
    updated_at: u64,
}

#[derive(Serialize, Deserialize, Default)]
struct CanvasIndex {
    version: u32,
    canvases: HashMap<String, CanvasIndexEntry>,
}

fn load_index() -> PyResult<CanvasIndex> {
    let path = canvas_index_path();
    match fs::read_to_string(&path) {
        Ok(s) => serde_json::from_str(&s).map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!(
                "canvas index at '{}' is corrupted: {}",
                path.display(),
                e
            ))
        }),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(CanvasIndex {
            version: 1,
            canvases: HashMap::new(),
        }),
        Err(e) => Err(pyo3::exceptions::PyIOError::new_err(format!(
            "cannot read canvas index at '{}': {}",
            path.display(),
            e
        ))),
    }
}

fn save_index(index: &CanvasIndex) -> PyResult<()> {
    let dir = seraplot_home_dir();
    fs::create_dir_all(&dir).map_err(|e| {
        pyo3::exceptions::PyIOError::new_err(format!("cannot create '{}': {}", dir.display(), e))
    })?;
    let json = serde_json::to_string_pretty(index)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    fs::write(canvas_index_path(), json).map_err(|e| {
        pyo3::exceptions::PyIOError::new_err(format!("cannot write canvas index: {}", e))
    })
}

#[pyfunction]
pub fn canvas_default_dir() -> String {
    seraplot_home_dir().to_string_lossy().into_owned()
}

#[pyfunction]
pub fn canvas_load(path: &str) -> PyResult<Canvas> {
    let json = fs::read_to_string(path).map_err(|e| {
        pyo3::exceptions::PyIOError::new_err(format!("cannot read '{}': {}", path, e))
    })?;
    let state: CanvasState = serde_json::from_str(&json)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(Canvas::from_state(state))
}

#[pyfunction]
pub fn canvas_save_named(canvas: &Canvas, name: &str) -> PyResult<String> {
    let dir = seraplot_home_dir();
    let path = dir.join(format!("{}.json", name));
    canvas.save(&path.to_string_lossy())?;
    let mut index = load_index()?;
    let updated_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    index.canvases.insert(
        name.to_string(),
        CanvasIndexEntry {
            path: path.to_string_lossy().into_owned(),
            updated_at,
        },
    );
    save_index(&index)?;
    Ok(path.to_string_lossy().into_owned())
}

#[pyfunction]
pub fn canvas_load_named(name: &str) -> PyResult<Canvas> {
    let index = load_index()?;
    let entry = index.canvases.get(name).ok_or_else(|| {
        pyo3::exceptions::PyKeyError::new_err(format!("no saved canvas named '{}'", name))
    })?;
    canvas_load(&entry.path)
}
