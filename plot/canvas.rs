use crate::core::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Canvas {
    width: f32,
    height: f32,
    layout: Layout,
    traces: Vec<Trace>,
    selection: HashMap<String, Vec<usize>>,
    hover_point: Option<(f32, f32)>,
}

impl Canvas {
    pub fn new(width: f32, height: f32, layout: Layout) -> Self {
        Self { width, height, layout, traces: Vec::new(), selection: HashMap::new(), hover_point: None }
    }

    pub fn add_trace(mut self, trace: Trace) -> Self {
        self.traces.push(trace);
        self
    }

    pub fn add_traces(mut self, traces: Vec<Trace>) -> Self {
        self.traces.extend(traces);
        self
    }

    pub fn add_trace_mut(&mut self, trace: Trace) {
        self.traces.push(trace);
    }

    pub fn set_hover(&mut self, x: f32, y: f32) {
        self.hover_point = Some((x, y));
    }

    pub fn select_point(&mut self, trace_id: &str, idx: usize) {
        self.selection.entry(trace_id.to_string()).or_insert_with(Vec::new).push(idx);
    }

    pub fn clear_selection(&mut self) {
        self.selection.clear();
    }

    pub fn traces(&self) -> &[Trace] { &self.traces }
    pub fn layout(&self) -> &Layout { &self.layout }
    pub fn dimensions(&self) -> (f32, f32) { (self.width, self.height) }
    pub fn hover(&self) -> Option<(f32, f32)> { self.hover_point }
    pub fn selection(&self, trace_id: &str) -> Option<&Vec<usize>> { self.selection.get(trace_id) }

    fn margin_bounds(&self) -> (f32, f32, f32, f32) {
        let m = &self.layout.margin;
        (m.left as f32, self.width - m.right as f32, m.top as f32, self.height - m.bottom as f32)
    }

    fn axis_bounds(&self) -> Option<((f64, f64), (f64, f64))> {
        let x = (self.layout.x_axis.range.as_ref()?.min, self.layout.x_axis.range.as_ref()?.max);
        let y = (self.layout.y_axis.range.as_ref()?.min, self.layout.y_axis.range.as_ref()?.max);
        Some((x, y))
    }

    pub fn pixel_to_data(&self, px: f32, py: f32) -> Option<(f64, f64)> {
        let (x_min_px, x_max_px, y_max_px, y_min_px) = self.margin_bounds();
        if px < x_min_px || px > x_max_px || py < y_max_px || py > y_min_px { return None; }
        
        let ((x_min, x_max), (y_min, y_max)) = self.axis_bounds()?;
        let x = x_min + ((px - x_min_px) as f64 / (x_max_px - x_min_px) as f64) * (x_max - x_min);
        let y = y_max - ((py - y_max_px) as f64 / (y_min_px - y_max_px) as f64) * (y_max - y_min);
        Some((x, y))
    }

    pub fn data_to_pixel(&self, x: f64, y: f64) -> Option<(f32, f32)> {
        let (x_min_px, x_max_px, y_max_px, y_min_px) = self.margin_bounds();
        let ((x_min, x_max), (y_min, y_max)) = self.axis_bounds()?;

        let (x_width, y_height) = (x_max - x_min, y_max - y_min);
        if x_width <= 0.0 || y_height <= 0.0 { return None; }

        let px = x_min_px + ((x - x_min) / x_width) as f32 * (x_max_px - x_min_px);
        let py = y_min_px - ((y - y_min) / y_height) as f32 * (y_min_px - y_max_px);

        if px.is_finite() && py.is_finite() { Some((px, py)) } else { None }
    }

    pub fn nearest_point(&self, data_x: f64, data_y: f64, max_distance: f32) -> Option<(String, usize, f32)> {
        let mut nearest: Option<(String, usize, f32)> = None;
        let mut min_dist = max_distance;

        for trace in &self.traces {
            for (i, (xi, yi)) in trace.x.iter().zip(trace.y.iter()).enumerate() {
                if let Some((px, py)) = self.data_to_pixel(*xi, *yi) {
                    let dist = ((px - data_x as f32).powi(2) + (py - data_y as f32).powi(2)).sqrt();
                    if dist < min_dist {
                        min_dist = dist;
                        nearest = Some((trace.id.clone(), i, dist));
                    }
                }
            }
        }
        nearest
    }    pub fn auto_scale(&mut self) {
        if self.traces.is_empty() { return; }
        
        let (mut x_min, mut x_max, mut y_min, mut y_max) = (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY);
        
        for trace in &self.traces {
            if let Some(r) = Range::from_slice(&trace.x) { x_min = x_min.min(r.min); x_max = x_max.max(r.max); }
            if let Some(r) = Range::from_slice(&trace.y) { y_min = y_min.min(r.min); y_max = y_max.max(r.max); }
        }

        if x_min.is_finite() && x_max.is_finite() {
            self.layout.x_axis.range = Some(Range { min: x_min, max: x_max });
        }
        if y_min.is_finite() && y_max.is_finite() {
            self.layout.y_axis.range = Some(Range { min: y_min, max: y_max });
        }
    }

    pub fn zoom(&mut self, center_x: f64, center_y: f64, factor: f32) {
        if let Some(x_range) = &mut self.layout.x_axis.range {
            let w = (x_range.max - x_range.min) * (factor as f64 - 1.0) / 2.0;
            x_range.min = center_x - (center_x - x_range.min) / factor as f64 - w;
            x_range.max = center_x + (x_range.max - center_x) / factor as f64 + w;
        }
        if let Some(y_range) = &mut self.layout.y_axis.range {
            let h = (y_range.max - y_range.min) * (factor as f64 - 1.0) / 2.0;
            y_range.min = center_y - (center_y - y_range.min) / factor as f64 - h;
            y_range.max = center_y + (y_range.max - center_y) / factor as f64 + h;
        }
    }

    pub fn pan(&mut self, dx: f64, dy: f64) {
        if let Some(r) = &mut self.layout.x_axis.range { r.min -= dx; r.max -= dx; }
        if let Some(r) = &mut self.layout.y_axis.range { r.min -= dy; r.max -= dy; }
    }
}

pub struct App {
    canvas: Canvas,
    is_panning: bool,
    pan_start: Option<(f32, f32)>,
    legend_visible: bool,
}

impl App {
    pub fn new(canvas: Canvas) -> Self {
        Self { canvas, is_panning: false, pan_start: None, legend_visible: true }
    }

    pub fn on_mouse_down(&mut self, x: f32, y: f32) {
        self.is_panning = true;
        self.pan_start = Some((x, y));
    }

    pub fn on_mouse_up(&mut self) {
        self.is_panning = false;
        self.pan_start = None;
    }

    pub fn on_mouse_move(&mut self, x: f32, y: f32) {
        self.canvas.set_hover(x, y);
        if self.is_panning {
            if let Some((sx, sy)) = self.pan_start {
                self.canvas.pan((x - sx) as f64 / 100.0, (y - sy) as f64 / 100.0);
                self.pan_start = Some((x, y));
            }
        }
    }

    pub fn on_scroll(&mut self, x: f32, y: f32, delta: f32) {
        if let Some((dx, dy)) = self.canvas.pixel_to_data(x, y) {
            self.canvas.zoom(dx, dy, 1.0 - delta * 0.1);
        }
    }

    pub fn toggle_legend(&mut self) { self.legend_visible = !self.legend_visible; }
    pub fn canvas(&self) -> &Canvas { &self.canvas }
    pub fn canvas_mut(&mut self) -> &mut Canvas { &mut self.canvas }
    pub fn legend_visible(&self) -> bool { self.legend_visible }
}
