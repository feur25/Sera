use super::canvas_core::Canvas;
use super::element::El;
use pyo3::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

#[pymethods]
impl Canvas {
    pub fn link(&mut self, group_name: &str, member_names: Vec<String>) -> usize {
        let mut linked = 0;
        for member in &member_names {
            let Some(&idx) = self.names.get(member) else {
                continue;
            };
            let Some(el) = self.elements.get_mut(idx) else {
                continue;
            };
            let target = match el {
                El::Chart { group, .. } => Some(group),
                El::Line { group, .. } => Some(group),
                El::Circle { group, .. } => Some(group),
                El::Text { group, .. } => Some(group),
                El::Rect { group, .. } => Some(group),
                El::Wedge { group, .. } => Some(group),
                _ => None,
            };
            if let Some(g) = target {
                *g = group_name.to_string();
                linked += 1;
            }
        }
        linked
    }

    #[pyo3(signature = (group_name, members))]
    pub fn group(&mut self, group_name: &str, members: Vec<String>) {
        self.groups.insert(group_name.to_string(), members);
    }

    #[pyo3(signature = (group_name, dx, dy))]
    pub fn move_group(&mut self, group_name: &str, dx: f64, dy: f64) -> usize {
        let Some(members) = self.groups.get(group_name).cloned() else {
            return 0;
        };
        let mut moved = 0;
        for member in &members {
            if self.apply_delta_by_name(member, dx, dy, 0.0, 0.0) {
                moved += 1;
            }
        }
        moved
    }

    pub fn group_members(&self, group_name: &str) -> Vec<String> {
        self.groups.get(group_name).cloned().unwrap_or_default()
    }

    pub fn nudge(&mut self, name: &str, dx: f64, dy: f64) -> bool {
        self.apply_delta_by_name(name, dx, dy, 0.0, 0.0)
    }

    pub fn resize(&mut self, name: &str, dw: f64, dh: f64) -> bool {
        self.apply_delta_by_name(name, 0.0, 0.0, dw, dh)
    }

    pub fn apply_deltas_json(&mut self, json_str: &str) -> PyResult<usize> {
        #[derive(Deserialize)]
        struct Delta {
            #[serde(default)]
            dx: f64,
            #[serde(default)]
            dy: f64,
            #[serde(default)]
            dw: f64,
            #[serde(default)]
            dh: f64,
        }
        let parsed: HashMap<String, Delta> = serde_json::from_str(json_str)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let mut applied = 0;
        for (name, d) in parsed {
            if self.apply_delta_by_name(&name, d.dx, d.dy, d.dw, d.dh) {
                applied += 1;
            }
        }
        Ok(applied)
    }

    pub fn style(&mut self, name: &str, css: &str) {
        self.custom_css.push((name.to_string(), css.to_string()));
    }

    pub fn script(&mut self, js: &str) {
        self.custom_js.push(js.to_string());
    }

    pub fn element_ref(&self, name: &str) -> Option<usize> {
        self.names.get(name).copied()
    }

    pub fn set_group(&mut self, chart_ref: usize, group: &str) {
        if let Some(info) = self.placed.get(chart_ref) {
            let idx = info.element_idx;
            if let Some(El::Chart { group: g, .. }) = self.elements.get_mut(idx) {
                *g = group.to_string();
            }
        }
    }
}
