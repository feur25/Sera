use super::canvas_core::Canvas;
use super::element::El;
use super::html_util::guess_mime;
use base64::Engine;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::fs;

#[pymethods]
impl Canvas {
    #[new]
    #[pyo3(signature = (width, height, bg = "#0a0a0f"))]
    pub fn new(width: u32, height: u32, bg: &str) -> Self {
        Canvas {
            width,
            height,
            background: bg.to_string(),
            elements: Vec::new(),
            placed: Vec::new(),
            pins: HashMap::new(),
            names: HashMap::new(),
            groups: HashMap::new(),
            custom_css: Vec::new(),
            custom_js: Vec::new(),
            slots: HashMap::new(),
            tips: Vec::new(),
        }
    }

    #[pyo3(signature = (chart, x, y, w, h, rotation = 0.0, opacity = 1.0, clip = "", group = "", name = ""))]
    pub fn place(
        &mut self,
        chart: &crate::Chart,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        rotation: f64,
        opacity: f64,
        clip: &str,
        group: &str,
        name: &str,
    ) -> usize {
        self.place_internal(chart, x, y, w, h, rotation, opacity, clip, group, name)
    }

    #[pyo3(signature = (src, x, y, w, h, rotation = 0.0, opacity = 1.0, clip = "", group = "", name = ""))]
    pub fn image(
        &mut self,
        src: &str,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        rotation: f64,
        opacity: f64,
        clip: &str,
        group: &str,
        name: &str,
    ) -> PyResult<usize> {
        let resolved = if src.starts_with("data:")
            || src.starts_with("http://")
            || src.starts_with("https://")
        {
            src.to_string()
        } else {
            let bytes = fs::read(src).map_err(|e| {
                pyo3::exceptions::PyIOError::new_err(format!(
                    "cannot read image '{}': {}",
                    src, e
                ))
            })?;
            let mime = guess_mime(src);
            format!(
                "data:{};base64,{}",
                mime,
                base64::engine::general_purpose::STANDARD.encode(bytes)
            )
        };
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Image {
            src: resolved,
            x,
            y,
            w,
            h,
            rotation,
            opacity,
            clip: clip.to_string(),
            group: group.to_string(),
            name: name.to_string(),
        });
        Ok(element_idx)
    }

    pub fn slot(&mut self, name: &str, x: f64, y: f64, w: f64, h: f64) {
        self.slots.insert(name.to_string(), (x, y, w, h));
    }

    pub fn slot_rect(&self, name: &str) -> Option<(f64, f64, f64, f64)> {
        self.slots.get(name).copied()
    }

    #[pyo3(signature = (slot_name, chart, rotation = 0.0, opacity = 1.0, clip = "", group = "", name = ""))]
    pub fn fill(
        &mut self,
        slot_name: &str,
        chart: &crate::Chart,
        rotation: f64,
        opacity: f64,
        clip: &str,
        group: &str,
        name: &str,
    ) -> PyResult<usize> {
        let (x, y, w, h) = self.slots.get(slot_name).copied().ok_or_else(|| {
            pyo3::exceptions::PyKeyError::new_err(format!("no slot named '{}'", slot_name))
        })?;
        let resolved_name = if name.is_empty() { slot_name } else { name };
        Ok(self.place_internal(chart, x, y, w, h, rotation, opacity, clip, group, resolved_name))
    }

    #[pyo3(signature = (x, y, w, h, rows, cols, gap_x = 0.0, gap_y = 0.0))]
    pub fn grid(
        &mut self,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        rows: usize,
        cols: usize,
        gap_x: f64,
        gap_y: f64,
    ) -> Vec<String> {
        let rows = rows.max(1);
        let cols = cols.max(1);
        let cell_w = (w - gap_x * (cols as f64 - 1.0)) / cols as f64;
        let cell_h = (h - gap_y * (rows as f64 - 1.0)) / rows as f64;
        let mut names = Vec::with_capacity(rows * cols);
        for r in 0..rows {
            for c in 0..cols {
                let cx = x + c as f64 * (cell_w + gap_x);
                let cy = y + r as f64 * (cell_h + gap_y);
                let name = format!("cell_{}_{}", r, c);
                self.slots.insert(name.clone(), (cx, cy, cell_w, cell_h));
                names.push(name);
            }
        }
        names
    }

    pub fn refill(&mut self, name: &str, chart: &crate::Chart) -> PyResult<bool> {
        let Some(&element_idx) = self.names.get(name) else {
            return Ok(false);
        };
        let (chart_ref, w, h) = match self.elements.get(element_idx) {
            Some(El::Chart { ref_id, w, h, .. }) => (*ref_id, *w, *h),
            _ => return Ok(false),
        };
        let frame = super::anchors::chart_frame(&chart.html, w, h);
        if let Some(El::Chart {
            html,
            native_w,
            native_h,
            ..
        }) = self.elements.get_mut(element_idx)
        {
            *html = chart.html.clone();
            *native_w = frame.native_w;
            *native_h = frame.native_h;
        }
        if let Some(info) = self.placed.get_mut(chart_ref) {
            info.native_w = frame.native_w;
            info.native_h = frame.native_h;
            info.plot = frame.plot;
        }
        self.clear_pins_for(chart_ref);
        Ok(true)
    }

    pub fn derive(&self) -> Canvas {
        self.clone()
    }

    pub fn template(&self) -> Canvas {
        let mut kept_elements: Vec<El> = Vec::new();
        let mut old_to_new: HashMap<usize, usize> = HashMap::new();
        for (old_idx, el) in self.elements.iter().enumerate() {
            if matches!(el, El::Chart { .. } | El::Image { .. }) {
                continue;
            }
            let new_idx = kept_elements.len();
            old_to_new.insert(old_idx, new_idx);
            kept_elements.push(el.clone());
        }
        let names: HashMap<String, usize> = self
            .names
            .iter()
            .filter_map(|(k, v)| old_to_new.get(v).map(|nv| (k.clone(), *nv)))
            .collect();
        let groups: HashMap<String, Vec<String>> = self
            .groups
            .iter()
            .map(|(k, members)| {
                (
                    k.clone(),
                    members
                        .iter()
                        .filter(|m| names.contains_key(*m))
                        .cloned()
                        .collect::<Vec<String>>(),
                )
            })
            .filter(|(_, members)| !members.is_empty())
            .collect();
        Canvas {
            width: self.width,
            height: self.height,
            background: self.background.clone(),
            elements: kept_elements,
            placed: Vec::new(),
            pins: HashMap::new(),
            names,
            groups,
            custom_css: self.custom_css.clone(),
            custom_js: self.custom_js.clone(),
            slots: self.slots.clone(),
            tips: self.tips.clone(),
        }
    }
}
