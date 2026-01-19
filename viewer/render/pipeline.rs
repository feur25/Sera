pub struct PointComputeBuilder {
    batch_size: usize,
    cache_threshold: usize,
    use_simd: bool,
}

impl Default for PointComputeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PointComputeBuilder {
    pub fn new() -> Self {
        Self {
            batch_size: 10000,
            cache_threshold: 50000,
            use_simd: cfg!(target_arch = "x86_64"),
        }
    }

    pub fn with_batch_size(mut self, size: usize) -> Self {
        self.batch_size = size.max(1000);
        self
    }

    pub fn with_cache_threshold(mut self, threshold: usize) -> Self {
        self.cache_threshold = threshold;
        self
    }

    pub fn enable_simd(mut self, enable: bool) -> Self {
        self.use_simd = enable;
        self
    }

    pub fn build(self) -> PointCompute {
        PointCompute {
            batch_size: self.batch_size,
            cache_threshold: self.cache_threshold,
            use_simd: self.use_simd,
        }
    }

    pub fn compute_points(
        &self,
        values: &[f64],
        indices: &[usize],
        max_val: f64,
        plot_rect: egui::Rect,
        vertical: bool,
    ) -> Vec<egui::Pos2> {
        let point_count = indices.len();
        let mut points = Vec::with_capacity(point_count);

        if point_count > self.cache_threshold && self.use_simd {
            self.compute_points_simd(&mut points, values, indices, max_val, plot_rect, vertical);
        } else {
            self.compute_points_standard(&mut points, values, indices, max_val, plot_rect, vertical);
        }

        points
    }

    fn compute_points_standard(
        &self,
        points: &mut Vec<egui::Pos2>,
        values: &[f64],
        indices: &[usize],
        max_val: f64,
        plot_rect: egui::Rect,
        vertical: bool,
    ) {
        let max_val_f64 = max_val.max(1.0);
        let plot_width = plot_rect.width();
        let plot_height = plot_rect.height();
        let plot_left = plot_rect.left();
        let plot_bottom = plot_rect.bottom();
        let plot_top = plot_rect.top();
        let point_count = indices.len() as f32;

        for (idx, &actual_idx) in indices.iter().enumerate() {
            if actual_idx < values.len() {
                let value = values[actual_idx];
                let norm_val = (value / max_val_f64) as f32;

                let pos = if vertical {
                    let x = plot_left + (plot_width / (point_count - 1.0).max(1.0)) * idx as f32;
                    let y = plot_bottom - norm_val * plot_height;
                    egui::pos2(x, y)
                } else {
                    let x = plot_left + norm_val * plot_width;
                    let y = plot_top + (plot_height / (point_count - 1.0).max(1.0)) * idx as f32;
                    egui::pos2(x, y)
                };

                points.push(pos);
            }
        }
    }

    fn compute_points_simd(
        &self,
        points: &mut Vec<egui::Pos2>,
        values: &[f64],
        indices: &[usize],
        max_val: f64,
        plot_rect: egui::Rect,
        vertical: bool,
    ) {
        self.compute_points_standard(points, values, indices, max_val, plot_rect, vertical);
    }
}

pub struct PointCompute {
    batch_size: usize,
    cache_threshold: usize,
    use_simd: bool,
}

impl PointCompute {
    pub fn compute_points(
        &self,
        values: &[f64],
        indices: &[usize],
        max_val: f64,
        plot_rect: egui::Rect,
        vertical: bool,
    ) -> Vec<egui::Pos2> {
        let point_count = indices.len();
        let mut points = Vec::with_capacity(point_count);

        if point_count > self.cache_threshold && self.use_simd {
            self.compute_points_simd(&mut points, values, indices, max_val, plot_rect, vertical);
        } else {
            self.compute_points_standard(&mut points, values, indices, max_val, plot_rect, vertical);
        }

        points
    }

    fn compute_points_standard(
        &self,
        points: &mut Vec<egui::Pos2>,
        values: &[f64],
        indices: &[usize],
        max_val: f64,
        plot_rect: egui::Rect,
        vertical: bool,
    ) {
        let max_val_f64 = max_val.max(1.0);
        let plot_width = plot_rect.width();
        let plot_height = plot_rect.height();
        let plot_left = plot_rect.left();
        let plot_bottom = plot_rect.bottom();
        let plot_top = plot_rect.top();
        let point_count = indices.len() as f32;

        for (idx, &actual_idx) in indices.iter().enumerate() {
            if actual_idx < values.len() {
                let value = values[actual_idx];
                let norm_val = (value / max_val_f64) as f32;

                let pos = if vertical {
                    let x = plot_left + (plot_width / (point_count - 1.0).max(1.0)) * idx as f32;
                    let y = plot_bottom - norm_val * plot_height;
                    egui::pos2(x, y)
                } else {
                    let x = plot_left + norm_val * plot_width;
                    let y = plot_top + (plot_height / (point_count - 1.0).max(1.0)) * idx as f32;
                    egui::pos2(x, y)
                };

                points.push(pos);
            }
        }
    }

    fn compute_points_simd(
        &self,
        points: &mut Vec<egui::Pos2>,
        values: &[f64],
        indices: &[usize],
        max_val: f64,
        plot_rect: egui::Rect,
        vertical: bool,
    ) {
        self.compute_points_standard(points, values, indices, max_val, plot_rect, vertical);
    }
}

pub struct ChunkRenderBuilder {
    chunk_size: usize,
    enable_lod: bool,
    lod_threshold: usize,
}

impl Default for ChunkRenderBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ChunkRenderBuilder {
    pub fn new() -> Self {
        Self {
            chunk_size: 5000,
            enable_lod: true,
            lod_threshold: 50000,
        }
    }

    pub fn with_chunk_size(mut self, size: usize) -> Self {
        self.chunk_size = size.max(1000);
        self
    }

    pub fn enable_lod(mut self, enable: bool) -> Self {
        self.enable_lod = enable;
        self
    }

    pub fn with_lod_threshold(mut self, threshold: usize) -> Self {
        self.lod_threshold = threshold;
        self
    }

    pub fn build(self) -> ChunkRenderer {
        ChunkRenderer {
            chunk_size: self.chunk_size,
            enable_lod: self.enable_lod,
            lod_threshold: self.lod_threshold,
        }
    }

    pub fn render_chunks(
        &self,
        painter: &egui::Painter,
        points: &[egui::Pos2],
        colors: &[egui::Color32],
        radius: f32,
    ) {
        if self.enable_lod && points.len() > self.lod_threshold {
            self.render_lod(painter, points, colors, radius);
        } else {
            self.render_standard(painter, points, colors, radius);
        }
    }

    fn render_standard(
        &self,
        painter: &egui::Painter,
        points: &[egui::Pos2],
        colors: &[egui::Color32],
        radius: f32,
    ) {
        for chunk_points in points.chunks(self.chunk_size) {
            let start_idx = (points.len() - chunk_points.len()) / 1;
            for (i, &pos) in chunk_points.iter().enumerate() {
                let color = colors[(start_idx + i) % colors.len()];
                painter.circle_filled(pos, radius, color);
            }
        }
    }

    fn render_lod(
        &self,
        painter: &egui::Painter,
        points: &[egui::Pos2],
        colors: &[egui::Color32],
        radius: f32,
    ) {
        let lod_skip = (points.len() / self.lod_threshold).max(1);
        for (i, &pos) in points.iter().enumerate().step_by(lod_skip) {
            let color = colors[i % colors.len()];
            painter.circle_filled(pos, radius, color);
        }
    }
}

pub struct ChunkRenderer {
    chunk_size: usize,
    enable_lod: bool,
    lod_threshold: usize,
}

impl ChunkRenderer {
    pub fn render(&self, painter: &egui::Painter, points: &[egui::Pos2], colors: &[egui::Color32], radius: f32) {
        if self.enable_lod && points.len() > self.lod_threshold {
            self.render_lod(painter, points, colors, radius);
        } else {
            self.render_standard(painter, points, colors, radius);
        }
    }

    fn render_standard(
        &self,
        painter: &egui::Painter,
        points: &[egui::Pos2],
        colors: &[egui::Color32],
        radius: f32,
    ) {
        for chunk_points in points.chunks(self.chunk_size) {
            let start_idx = (points.len() - chunk_points.len()) / 1;
            for (i, &pos) in chunk_points.iter().enumerate() {
                let color = colors[(start_idx + i) % colors.len()];
                painter.circle_filled(pos, radius, color);
            }
        }
    }

    fn render_lod(
        &self,
        painter: &egui::Painter,
        points: &[egui::Pos2],
        colors: &[egui::Color32],
        radius: f32,
    ) {
        let lod_skip = (points.len() / self.lod_threshold).max(1);
        for (i, &pos) in points.iter().enumerate().step_by(lod_skip) {
            let color = colors[i % colors.len()];
            painter.circle_filled(pos, radius, color);
        }
    }
}

pub struct RenderPipelineBuilder {
    enable_async: bool,
    enable_cache: bool,
    enable_compression: bool,
    max_pipeline_depth: usize,
}

impl Default for RenderPipelineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderPipelineBuilder {
    pub fn new() -> Self {
        Self {
            enable_async: true,
            enable_cache: true,
            enable_compression: false,
            max_pipeline_depth: 3,
        }
    }

    pub fn enable_async(mut self, enable: bool) -> Self {
        self.enable_async = enable;
        self
    }

    pub fn enable_cache(mut self, enable: bool) -> Self {
        self.enable_cache = enable;
        self
    }

    pub fn enable_compression(mut self, enable: bool) -> Self {
        self.enable_compression = enable;
        self
    }

    pub fn build(self) -> RenderPipeline {
        RenderPipeline {
            enable_async: self.enable_async,
            enable_cache: self.enable_cache,
            enable_compression: self.enable_compression,
            max_pipeline_depth: self.max_pipeline_depth,
        }
    }
}

pub struct RenderPipeline {
    pub enable_async: bool,
    pub enable_cache: bool,
    pub enable_compression: bool,
    pub max_pipeline_depth: usize,
}

impl RenderPipeline {
    pub fn builder() -> RenderPipelineBuilder {
        RenderPipelineBuilder::new()
    }

    pub fn can_process(&self, data_size: usize) -> bool {
        data_size <= 1_000_000
    }

    pub fn get_optimal_batch_size(&self, data_size: usize) -> usize {
        if data_size < 10_000 {
            data_size
        } else if data_size < 100_000 {
            10_000
        } else if data_size < 500_000 {
            25_000
        } else {
            50_000
        }
    }

    pub fn get_rendering_strategy(&self, data_size: usize) -> RenderingStrategy {
        if data_size < 5_000 {
            RenderingStrategy::Direct
        } else if data_size < 50_000 {
            RenderingStrategy::Batched
        } else if data_size < 500_000 {
            RenderingStrategy::LOD
        } else {
            RenderingStrategy::Chunked
        }
    }
}

pub enum RenderingStrategy {
    Direct,
    Batched,
    LOD,
    Chunked,
}

pub struct VisibilityOptimizer {
    viewport_padding: f32,
    use_frustum_culling: bool,
}

impl VisibilityOptimizer {
    pub fn new() -> Self {
        Self {
            viewport_padding: 50.0,
            use_frustum_culling: true,
        }
    }

    pub fn filter_visible(
        &self,
        points: &[egui::Pos2],
        viewport: egui::Rect,
    ) -> Vec<usize> {
        let padded_rect = egui::Rect::from_min_max(
            egui::pos2(
                viewport.left() - self.viewport_padding,
                viewport.top() - self.viewport_padding,
            ),
            egui::pos2(
                viewport.right() + self.viewport_padding,
                viewport.bottom() + self.viewport_padding,
            ),
        );

        points
            .iter()
            .enumerate()
            .filter(|(_, &pos)| padded_rect.contains(pos))
            .map(|(i, _)| i)
            .collect()
    }

    pub fn set_padding(&mut self, padding: f32) {
        self.viewport_padding = padding.max(0.0);
    }
}

impl Default for VisibilityOptimizer {
    fn default() -> Self {
        Self::new()
    }
}
