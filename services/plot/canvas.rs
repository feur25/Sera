use std::collections::HashMap;

#[derive(Clone)]
pub struct Canvas {
    width: f32,
    height: f32,
    _labels: Vec<String>,
    values: Vec<f64>,
    type_id: u8,
    selection: HashMap<String, Vec<usize>>,
    hover_point: Option<(f32, f32)>,
}

impl Canvas {
    pub fn new(
        width: f32,
        height: f32,
        labels: Vec<String>,
        values: Vec<f64>,
        type_id: u8,
    ) -> Self {
        Self {
            width,
            height,
            _labels: labels,
            values,
            type_id,
            selection: HashMap::new(),
            hover_point: None,
        }
    }

    pub fn new_with_data(labels: &[String], values: &[f64], type_id: u8) -> Self {
        Self::new(800.0, 600.0, labels.to_vec(), values.to_vec(), type_id)
    }

    pub fn set_hover(&mut self, x: f32, y: f32) {
        self.hover_point = Some((x, y));
    }

    pub fn select_point(&mut self, trace_id: &str, idx: usize) {
        self.selection
            .entry(trace_id.to_string())
            .or_insert_with(Vec::new)
            .push(idx);
    }

    pub fn clear_selection(&mut self) {
        self.selection.clear();
    }

    pub fn dimensions(&self) -> (f32, f32) {
        (self.width, self.height)
    }
    pub fn hover(&self) -> Option<(f32, f32)> {
        self.hover_point
    }
    pub fn selection(&self, trace_id: &str) -> Option<&Vec<usize>> {
        self.selection.get(trace_id)
    }

    pub fn render_svg(&self) -> String {
        let Some(svg_renderer) = crate::plot::controller::chart_controller::get_svg_renderer(self.type_id) else {
            eprintln!(
                "seraplot: Canvas::render_svg found no SVG renderer registered for chart type {} \
                 -- rendering a blank chart",
                self.type_id
            );
            return String::new();
        };
        let mut svg = String::new();
        let colors: Vec<&'static str> = vec!["#4a90e2"];
        let max_val = self.values.iter().copied().fold(0.0, f64::max).max(1.0);
        svg_renderer(
            &mut svg,
            &self.values,
            &colors,
            0,
            self.width as i32,
            self.height as i32,
            max_val,
            true,
        );
        svg
    }

    pub fn zoom(&mut self, _center_x: f64, _center_y: f64, _factor: f32) {
        self.width *= _factor;
        self.height *= _factor;
    }

    pub fn pan(&mut self, _dx: f64, _dy: f64) {
        self.width += _dx as f32;
        self.height += _dy as f32;
    }
}

pub struct ViewerApp {
    canvas: Canvas,
    is_panning: bool,
    pan_start: Option<(f32, f32)>,
    legend_visible: bool,
}

impl ViewerApp {
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
            if let Some((sx, sy)) = self.pan_start {
                self.canvas
                    .pan((x - sx) as f64 / 100.0, (y - sy) as f64 / 100.0);
                self.pan_start = Some((x, y));
            }
        }
    }

    pub fn on_scroll(&mut self, _x: f32, _y: f32, _delta: f32) {
        self.canvas.zoom(_x as f64, _y as f64, 1.1);
    }

    pub fn toggle_legend(&mut self) {
        self.legend_visible = !self.legend_visible;
    }
    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }
    pub fn canvas_mut(&mut self) -> &mut Canvas {
        &mut self.canvas
    }
    pub fn legend_visible(&self) -> bool {
        self.legend_visible
    }
}
