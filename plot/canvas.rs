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
        Self {
            width,
            height,
            layout,
            traces: Vec::new(),
            selection: HashMap::new(),
            hover_point: None,
        }
    }

    pub fn add_trace(mut self, trace: Trace) -> Self {
        self.traces.push(trace);
        self
    }

    pub fn add_trace_mut(&mut self, trace: Trace) {
        self.traces.push(trace);
    }

    pub fn add_traces(mut self, traces: Vec<Trace>) -> Self {
        self.traces.extend(traces);
        self
    }

    pub fn set_hover(&mut self, x: f32, y: f32) {
        self.hover_point = Some((x, y));
    }

    pub fn select_point(&mut self, trace_id: &str, idx: usize) {
        self.selection.entry(trace_id.to_string())
            .or_insert_with(Vec::new)
            .push(idx);
    }

    pub fn clear_selection(&mut self) {
        self.selection.clear();
    }

    pub fn get_traces(&self) -> &[Trace] {
        &self.traces
    }

    pub fn get_layout(&self) -> &Layout {
        &self.layout
    }

    pub fn get_dimensions(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    pub fn get_hover(&self) -> Option<(f32, f32)> {
        self.hover_point
    }

    pub fn get_selection(&self, trace_id: &str) -> Option<&Vec<usize>> {
        self.selection.get(trace_id)
    }

    pub fn pixel_to_data(&self, px: f32, py: f32) -> Option<(f64, f64)> {
        let margin = &self.layout.margin;
        let x_px_min = margin.left as f32;
        let x_px_max = self.width - margin.right as f32;
        let y_px_max = margin.top as f32;
        let y_px_min = self.height - margin.bottom as f32;

        if px < x_px_min || px > x_px_max || py < y_px_max || py > y_px_min {
            return None;
        }

        let x_range = self.layout.x_axis.range.as_ref()?;
        let y_range = self.layout.y_axis.range.as_ref()?;
        let x_min = x_range.min;
        let x_max = x_range.max;
        let y_min = y_range.min;
        let y_max = y_range.max;

        let x_data = x_min + ((px - x_px_min) as f64 / (x_px_max - x_px_min) as f64) * (x_max - x_min);
        let y_data = y_max - ((py - y_px_max) as f64 / (y_px_min - y_px_max) as f64) * (y_max - y_min);

        Some((x_data, y_data))
    }

    pub fn data_to_pixel(&self, x: f64, y: f64) -> Option<(f32, f32)> {
        let margin = &self.layout.margin;
        let x_px_min = margin.left as f32;
        let x_px_max = self.width - margin.right as f32;
        let y_px_max = margin.top as f32;
        let y_px_min = self.height - margin.bottom as f32;

        let x_range = self.layout.x_axis.range.as_ref()?;
        let y_range = self.layout.y_axis.range.as_ref()?;
        let x_min = x_range.min;
        let x_max = x_range.max;
        let y_min = y_range.min;
        let y_max = y_range.max;

        let x_width = x_max - x_min;
        let y_height = y_max - y_min;

        if x_width <= 0.0 || y_height <= 0.0 {
            return None;
        }

        let px = x_px_min + ((x - x_min) / x_width) as f32 * (x_px_max - x_px_min);
        let py = y_px_min - ((y - y_min) / y_height) as f32 * (y_px_min - y_px_max);

        if px.is_finite() && py.is_finite() {
            Some((px, py))
        } else {
            None
        }
    }

    pub fn get_nearest_point(&self, data_x: f64, data_y: f64, max_distance: f32) -> Option<(String, usize, f32)> {
        let mut nearest: Option<(String, usize, f32)> = None;
        let mut min_dist = max_distance;

        for trace in &self.traces {
            for (i, (xi, yi)) in trace.x.iter().zip(trace.y.iter()).enumerate() {
                if let Some((px, py)) = self.data_to_pixel(*xi, *yi) {
                    let dist_x = px - data_x as f32;
                    let dist_y = py - data_y as f32;
                    let dist = (dist_x * dist_x + dist_y * dist_y).sqrt();

                    if dist < min_dist {
                        min_dist = dist;
                        nearest = Some((trace.id.clone(), i, dist));
                    }
                }
            }
        }

        nearest
    }

    pub fn auto_scale(&mut self) {
        if self.traces.is_empty() {
            return;
        }

        let mut x_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;

        for trace in &self.traces {
            if let Some(range) = Range::from_slice(&trace.x) {
                x_min = x_min.min(range.min);
                x_max = x_max.max(range.max);
            }
            if let Some(range) = Range::from_slice(&trace.y) {
                y_min = y_min.min(range.min);
                y_max = y_max.max(range.max);
            }
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
            let x_width = (x_range.max - x_range.min) * (factor as f64 - 1.0) / 2.0;
            x_range.min = center_x - (center_x - x_range.min) / factor as f64 - x_width;
            x_range.max = center_x + (x_range.max - center_x) / factor as f64 + x_width;
        }
        if let Some(y_range) = &mut self.layout.y_axis.range {
            let y_height = (y_range.max - y_range.min) * (factor as f64 - 1.0) / 2.0;
            y_range.min = center_y - (center_y - y_range.min) / factor as f64 - y_height;
            y_range.max = center_y + (y_range.max - center_y) / factor as f64 + y_height;
        }
    }

    pub fn pan(&mut self, dx: f64, dy: f64) {
        if let Some(x_range) = &mut self.layout.x_axis.range {
            x_range.min -= dx;
            x_range.max -= dx;
        }
        if let Some(y_range) = &mut self.layout.y_axis.range {
            y_range.min -= dy;
            y_range.max -= dy;
        }
    }

    pub fn reset_view(&mut self) {
        self.auto_scale();
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
        Self {
            canvas,
            is_panning: false,
            pan_start: None,
            legend_visible: true,
        }
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
            if let Some((start_x, start_y)) = self.pan_start {
                let dx = (x - start_x) as f64 / 100.0;
                let dy = (y - start_y) as f64 / 100.0;
                self.canvas.pan(-dx, dy);
                self.pan_start = Some((x, y));
            }
        }
    }

    pub fn on_scroll(&mut self, x: f32, y: f32, delta: f32) {
        let factor = 1.0 - delta * 0.1;
        if let Some((data_x, data_y)) = self.canvas.pixel_to_data(x, y) {
            self.canvas.zoom(data_x, data_y, factor);
        }
    }

    pub fn toggle_legend(&mut self) {
        self.legend_visible = !self.legend_visible;
    }

    pub fn get_canvas(&self) -> &Canvas {
        &self.canvas
    }

    pub fn get_canvas_mut(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    pub fn is_legend_visible(&self) -> bool {
        self.legend_visible
    }
}
