use std::cell::RefCell;

pub struct DrawCommand {
    pub pos: egui::Pos2,
    pub radius: f32,
    pub color: egui::Color32,
}

pub struct LineCommand {
    pub from: egui::Pos2,
    pub to: egui::Pos2,
    pub color: egui::Color32,
    pub width: f32,
}

pub struct TextCommand {
    pub pos: egui::Pos2,
    pub text: String,
    pub color: egui::Color32,
}

pub struct VectorPool {
    circle_pool: RefCell<Vec<Vec<DrawCommand>>>,
    line_pool: RefCell<Vec<Vec<LineCommand>>>,
    text_pool: RefCell<Vec<Vec<TextCommand>>>,
}

impl VectorPool {
    pub fn new() -> Self {
        Self {
            circle_pool: RefCell::new(vec![]),
            line_pool: RefCell::new(vec![]),
            text_pool: RefCell::new(vec![]),
        }
    }

    pub fn acquire_circles(&self) -> Vec<DrawCommand> {
        self.circle_pool
            .borrow_mut()
            .pop()
            .unwrap_or_else(|| Vec::with_capacity(50000))
    }

    pub fn acquire_lines(&self) -> Vec<LineCommand> {
        self.line_pool
            .borrow_mut()
            .pop()
            .unwrap_or_else(|| Vec::with_capacity(25000))
    }

    pub fn acquire_text(&self) -> Vec<TextCommand> {
        self.text_pool
            .borrow_mut()
            .pop()
            .unwrap_or_else(|| Vec::with_capacity(1000))
    }

    pub fn release_circles(&self, mut v: Vec<DrawCommand>) {
        if self.circle_pool.borrow().len() < 3 {
            v.clear();
            self.circle_pool.borrow_mut().push(v);
        }
    }

    pub fn release_lines(&self, mut v: Vec<LineCommand>) {
        if self.line_pool.borrow().len() < 3 {
            v.clear();
            self.line_pool.borrow_mut().push(v);
        }
    }

    pub fn release_text(&self, mut v: Vec<TextCommand>) {
        if self.text_pool.borrow().len() < 3 {
            v.clear();
            self.text_pool.borrow_mut().push(v);
        }
    }
}

pub struct AdvancedBatchRenderer {
    circles: Vec<DrawCommand>,
    lines: Vec<LineCommand>,
    text_items: Vec<TextCommand>,
    pool: std::sync::Arc<VectorPool>,
    batch_size: usize,
}

impl AdvancedBatchRenderer {
    pub fn new() -> Self {
        Self::with_capacity(50000)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let pool = std::sync::Arc::new(VectorPool::new());
        Self {
            circles: Vec::with_capacity(capacity),
            lines: Vec::with_capacity(capacity / 2),
            text_items: Vec::with_capacity(1000),
            pool,
            batch_size: 1000,
        }
    }

    pub fn add_circle(&mut self, pos: egui::Pos2, radius: f32, color: egui::Color32) {
        self.circles.push(DrawCommand { pos, radius, color });
    }

    pub fn add_circles(&mut self, commands: Vec<DrawCommand>) {
        self.circles.extend(commands);
    }

    pub fn add_line(
        &mut self,
        from: egui::Pos2,
        to: egui::Pos2,
        color: egui::Color32,
        width: f32,
    ) {
        self.lines.push(LineCommand { from, to, color, width });
    }

    pub fn add_text(&mut self, pos: egui::Pos2, text: String, color: egui::Color32) {
        self.text_items.push(TextCommand { pos, text, color });
    }

    pub fn flush(&self, painter: &egui::Painter) {
        for chunk in self.circles.chunks(self.batch_size) {
            for cmd in chunk {
                painter.circle_filled(cmd.pos, cmd.radius, cmd.color);
            }
        }

        for chunk in self.lines.chunks(self.batch_size) {
            for cmd in chunk {
                painter.line_segment([cmd.from, cmd.to], egui::Stroke::new(cmd.width, cmd.color));
            }
        }

        for cmd in &self.text_items {
            painter.text(
                cmd.pos,
                egui::Align2::CENTER_CENTER,
                &cmd.text,
                egui::FontId::proportional(10.0),
                cmd.color,
            );
        }
    }

    pub fn clear(&mut self) {
        self.circles.clear();
        self.lines.clear();
        self.text_items.clear();
    }

    pub fn capacity(&self) -> usize {
        self.circles.capacity()
    }

    pub fn len(&self) -> usize {
        self.circles.len() + self.lines.len() + self.text_items.len()
    }

    pub fn is_full(&self) -> bool {
        self.circles.len() >= self.circles.capacity() * 90 / 100
    }
}

pub struct AdvancedBatchRendererBuilder {
    capacity: usize,
    batch_size: usize,
}

impl Default for AdvancedBatchRendererBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AdvancedBatchRendererBuilder {
    pub fn new() -> Self {
        Self {
            capacity: 50000,
            batch_size: 1000,
        }
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }

    pub fn with_batch_size(mut self, size: usize) -> Self {
        self.batch_size = size.max(100);
        self
    }

    pub fn build(self) -> AdvancedBatchRenderer {
        let mut renderer = AdvancedBatchRenderer::with_capacity(self.capacity);
        renderer.batch_size = self.batch_size;
        renderer
    }
}

pub struct RenderState {
    pub max_points: usize,
    pub screen_width: f32,
    pub screen_height: f32,
    pub zoom: f32,
    pub viewport_dirty: bool,
}

impl RenderState {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            max_points: 100000,
            screen_width: width,
            screen_height: height,
            zoom: 1.0,
            viewport_dirty: false,
        }
    }

    pub fn update_viewport(&mut self, width: f32, height: f32) {
        if (self.screen_width - width).abs() > 0.1 || (self.screen_height - height).abs() > 0.1 {
            self.screen_width = width;
            self.screen_height = height;
            self.viewport_dirty = true;
        }
    }

    pub fn update_zoom(&mut self, new_zoom: f32) {
        if (self.zoom - new_zoom).abs() > 0.01 {
            self.zoom = new_zoom;
            self.viewport_dirty = true;
        }
    }

    pub fn mark_clean(&mut self) {
        self.viewport_dirty = false;
    }
}

pub struct DataCache {
    cached_points: Vec<egui::Pos2>,
    cached_colors: Vec<egui::Color32>,
    data_hash: u64,
    is_valid: bool,
}

impl DataCache {
    pub fn new() -> Self {
        Self {
            cached_points: Vec::with_capacity(100000),
            cached_colors: Vec::with_capacity(100000),
            data_hash: 0,
            is_valid: false,
        }
    }

    pub fn update(&mut self, values: &[f64], colors: &[egui::Color32], hash: u64) {
        self.data_hash = hash;
        self.is_valid = self.data_hash == hash && !self.cached_points.is_empty();
    }

    pub fn points(&self) -> &[egui::Pos2] {
        &self.cached_points
    }

    pub fn colors(&self) -> &[egui::Color32] {
        &self.cached_colors
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    pub fn invalidate(&mut self) {
        self.is_valid = false;
    }

    pub fn set_data(&mut self, points: Vec<egui::Pos2>, colors: Vec<egui::Color32>, hash: u64) {
        self.cached_points = points;
        self.cached_colors = colors;
        self.data_hash = hash;
        self.is_valid = true;
    }
}
