use super::element::{El, PlacedInfo};
use super::render::{resize_element, translate_element};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[pyclass]
#[derive(Clone)]
pub struct Canvas {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) background: String,
    pub(super) elements: Vec<El>,
    pub(super) placed: Vec<PlacedInfo>,
    pub(super) pins: HashMap<String, (f64, f64)>,
    pub(super) names: HashMap<String, usize>,
    pub(super) groups: HashMap<String, Vec<String>>,
    pub(super) custom_css: Vec<(String, String)>,
    pub(super) custom_js: Vec<String>,
    pub(super) slots: HashMap<String, (f64, f64, f64, f64)>,
    pub(super) tips: Vec<crate::html::hover::HoverSlot>,
}

#[derive(Serialize, Deserialize)]
pub(super) struct CanvasState {
    version: u32,
    width: u32,
    height: u32,
    background: String,
    elements: Vec<El>,
    pins: HashMap<String, (f64, f64)>,
    names: HashMap<String, usize>,
    groups: HashMap<String, Vec<String>>,
    custom_css: Vec<(String, String)>,
    custom_js: Vec<String>,
    #[serde(default)]
    slots: HashMap<String, (f64, f64, f64, f64)>,
    #[serde(default)]
    tips: Vec<crate::html::hover::HoverSlot>,
}

impl Canvas {
    pub(super) fn pin_key(chart_ref: usize, name: &str) -> String {
        format!("{}::{}", chart_ref, name)
    }

    pub(super) fn register_name(&mut self, name: &str, element_idx: usize) {
        if !name.is_empty() {
            self.names.insert(name.to_string(), element_idx);
        }
    }

    pub(super) fn apply_delta_by_name(&mut self, name: &str, dx: f64, dy: f64, dw: f64, dh: f64) -> bool {
        let Some(&element_idx) = self.names.get(name) else {
            return false;
        };
        let chart_ref = match self.elements.get(element_idx) {
            Some(El::Chart { ref_id, .. }) => Some(*ref_id),
            _ => None,
        };
        let mut changed = false;
        if let Some(el) = self.elements.get_mut(element_idx) {
            if dx != 0.0 || dy != 0.0 {
                translate_element(el, dx, dy);
                changed = true;
            }
            if dw != 0.0 || dh != 0.0 {
                changed = resize_element(el, dw, dh) || changed;
            }
        }
        if !changed {
            return false;
        }
        let Some(chart_ref) = chart_ref else {
            return true;
        };
        if dx != 0.0 || dy != 0.0 {
            if let Some(info) = self.placed.get_mut(chart_ref) {
                info.x += dx;
                info.y += dy;
            }
            let prefix = format!("{}::", chart_ref);
            for (key, point) in self.pins.iter_mut() {
                if key.starts_with(&prefix) {
                    point.0 += dx;
                    point.1 += dy;
                }
            }
        }
        if dw != 0.0 || dh != 0.0 {
            if let Some(info) = self.placed.get_mut(chart_ref) {
                info.w = (info.w + dw).max(4.0);
                info.h = (info.h + dh).max(4.0);
            }
        }
        true
    }

    pub(super) fn rebuild_placed(&mut self) {
        self.placed.clear();
        for (element_idx, el) in self.elements.iter().enumerate() {
            if let El::Chart {
                html,
                x,
                y,
                w,
                h,
                native_w,
                native_h,
                rotation,
                ..
            } = el
            {
                let plot = super::anchors::chart_frame(html, *w, *h).plot;
                self.placed.push(PlacedInfo {
                    x: *x,
                    y: *y,
                    w: *w,
                    h: *h,
                    native_w: *native_w,
                    native_h: *native_h,
                    rotation: *rotation,
                    element_idx,
                    plot,
                });
            }
        }
    }

    pub(super) fn to_state(&self) -> CanvasState {
        CanvasState {
            version: 1,
            width: self.width,
            height: self.height,
            background: self.background.clone(),
            elements: self.elements.clone(),
            pins: self.pins.clone(),
            names: self.names.clone(),
            groups: self.groups.clone(),
            custom_css: self.custom_css.clone(),
            custom_js: self.custom_js.clone(),
            slots: self.slots.clone(),
            tips: self.tips.clone(),
        }
    }

    pub(super) fn from_state(state: CanvasState) -> Canvas {
        let mut canvas = Canvas {
            width: state.width,
            height: state.height,
            background: state.background,
            elements: state.elements,
            placed: Vec::new(),
            pins: state.pins,
            names: state.names,
            groups: state.groups,
            custom_css: state.custom_css,
            custom_js: state.custom_js,
            slots: state.slots,
            tips: state.tips,
        };
        canvas.rebuild_placed();
        canvas
    }

    pub(super) fn clear_pins_for(&mut self, chart_ref: usize) {
        let prefix = format!("{}::", chart_ref);
        self.pins.retain(|key, _| !key.starts_with(&prefix));
    }

    pub(super) fn place_internal(
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
        if !name.is_empty() {
            if let Some(&existing_idx) = self.names.get(name) {
                if let Some(El::Chart { ref_id, .. }) = self.elements.get(existing_idx) {
                    let chart_ref = *ref_id;
                    let frame = super::anchors::chart_frame(&chart.html, w, h);
                    self.clear_pins_for(chart_ref);
                    if let Some(info) = self.placed.get_mut(chart_ref) {
                        info.x = x;
                        info.y = y;
                        info.w = w;
                        info.h = h;
                        info.native_w = frame.native_w;
                        info.native_h = frame.native_h;
                        info.rotation = rotation;
                        info.plot = frame.plot;
                    }
                    if let Some(El::Chart {
                        html: el_html,
                        x: el_x,
                        y: el_y,
                        w: el_w,
                        h: el_h,
                        native_w: el_nw,
                        native_h: el_nh,
                        rotation: el_rot,
                        opacity: el_op,
                        clip: el_clip,
                        group: el_grp,
                        ..
                    }) = self.elements.get_mut(existing_idx)
                    {
                        *el_html = chart.html.clone();
                        *el_x = x;
                        *el_y = y;
                        *el_w = w;
                        *el_h = h;
                        *el_nw = frame.native_w;
                        *el_nh = frame.native_h;
                        *el_rot = rotation;
                        *el_op = opacity;
                        *el_clip = clip.to_string();
                        *el_grp = group.to_string();
                    }
                    return chart_ref;
                }
            }
        }
        let idx = self.placed.len();
        let frame = super::anchors::chart_frame(&chart.html, w, h);
        let element_idx = self.elements.len();
        self.placed.push(PlacedInfo {
            x,
            y,
            w,
            h,
            native_w: frame.native_w,
            native_h: frame.native_h,
            rotation,
            element_idx,
            plot: frame.plot,
        });
        self.register_name(name, element_idx);
        self.elements.push(El::Chart {
            html: chart.html.clone(),
            x,
            y,
            w,
            h,
            native_w: frame.native_w,
            native_h: frame.native_h,
            rotation,
            opacity,
            clip: clip.to_string(),
            group: group.to_string(),
            name: name.to_string(),
            ref_id: idx,
        });
        idx
    }

    pub(super) fn map_native_point(info: &PlacedInfo, local_x: f64, local_y: f64) -> (f64, f64) {
        let mt = super::anchors::NativeTransform::new(info.w, info.h, info.native_w, info.native_h);
        let (lx, ly) = mt.map(local_x, local_y);
        let mut ax = info.x + lx;
        let mut ay = info.y + ly;
        if info.rotation.abs() > 0.001 {
            let cx = info.x + info.w * 0.5;
            let cy = info.y + info.h * 0.5;
            let a = info.rotation.to_radians();
            let s = a.sin();
            let c = a.cos();
            let dx = ax - cx;
            let dy = ay - cy;
            ax = cx + dx * c - dy * s;
            ay = cy + dx * s + dy * c;
        }
        (ax, ay)
    }

    pub(super) fn register_native_pin(&mut self, chart_ref: usize, name: &str, local_x: f64, local_y: f64) {
        if let Some(info) = self.placed.get(chart_ref) {
            let point = Self::map_native_point(info, local_x, local_y);
            self.pins.insert(Self::pin_key(chart_ref, name), point);
        }
    }

    pub(super) fn chart_html(&self, chart_ref: usize) -> Option<&str> {
        let info = self.placed.get(chart_ref)?;
        match self.elements.get(info.element_idx)? {
            El::Chart { html, .. } => Some(html.as_str()),
            _ => None,
        }
    }

    pub(super) fn update_chart_space(
        &mut self,
        chart_ref: usize,
        native_w: f64,
        native_h: f64,
        plot: Option<super::anchors::PlotFrame>,
    ) {
        if let Some(info) = self.placed.get_mut(chart_ref) {
            if native_w.is_finite() && native_w > 0.0 {
                info.native_w = native_w;
            }
            if native_h.is_finite() && native_h > 0.0 {
                info.native_h = native_h;
            }
            if info.plot.is_none() {
                info.plot = plot;
            }
            if let Some(El::Chart {
                native_w: el_w,
                native_h: el_h,
                ..
            }) = self.elements.get_mut(info.element_idx)
            {
                *el_w = info.native_w;
                *el_h = info.native_h;
            }
        }
    }

    pub(super) fn plot_or(&self, chart_ref: usize, fallback: super::anchors::PlotFrame) -> super::anchors::PlotFrame {
        self.placed
            .get(chart_ref)
            .and_then(|info| info.plot)
            .unwrap_or(fallback)
    }
}
