use super::renderers::{hsv_to_rgb, ChartConfig, ChartPoint};

pub struct PlotState {
    pub dims: PlotDimensions,
    pub hovered_idx: Option<usize>,
    pub max_val: f64,
}

pub struct PlotDimensions {
    pub width: f32,
    pub height: f32,
    pub pad_x: f32,
    pub pad_y: f32,
}

impl PlotDimensions {
    pub fn new(width: f32, height: f32, pad_x: f32, pad_y: f32) -> Self {
        Self {
            width,
            height,
            pad_x,
            pad_y,
        }
    }

    pub fn vertical(zoom: f32) -> Self {
        Self::new(600.0 * zoom, 400.0 * zoom, 40.0, 20.0)
    }

    pub fn horizontal(zoom: f32) -> Self {
        Self::new(400.0 * zoom, 600.0 * zoom, 80.0, 20.0)
    }

    pub fn total_width(&self) -> f32 {
        self.width + self.pad_x * 2.0
    }

    pub fn total_height(&self) -> f32 {
        self.height + self.pad_y * 2.0
    }

    pub fn plot_rect(&self, base: egui::Pos2) -> egui::Rect {
        egui::Rect::from_min_size(
            base + egui::Vec2::new(self.pad_x, self.pad_y),
            egui::Vec2::new(self.width, self.height),
        )
    }
}

pub trait PointMapper: Sized {
    fn map(
        &self,
        idx: usize,
        point: &ChartPoint,
        max_val: f64,
        plot_rect: egui::Rect,
    ) -> egui::Pos2;
    fn detect(
        &self,
        rel_pos: egui::Vec2,
        plot_rect: egui::Rect,
        config: &ChartConfig,
    ) -> Option<usize>;
}

pub struct VerticalMapper;
impl PointMapper for VerticalMapper {
    fn map(
        &self,
        idx: usize,
        point: &ChartPoint,
        max_val: f64,
        plot_rect: egui::Rect,
    ) -> egui::Pos2 {
        let x = plot_rect.left() + ((idx as f32 + 0.5) / 10.0) * plot_rect.width();
        let norm_y = (point.value / max_val.max(1.0)) as f32;
        let y = plot_rect.bottom() - norm_y * plot_rect.height();
        egui::pos2(x, y)
    }

    fn detect(
        &self,
        rel_pos: egui::Vec2,
        plot_rect: egui::Rect,
        config: &ChartConfig,
    ) -> Option<usize> {
        if rel_pos.x < 0.0
            || rel_pos.x > plot_rect.width()
            || rel_pos.y < 0.0
            || rel_pos.y > plot_rect.height()
        {
            return None;
        }
        let visible_count = config.visible_points().len();
        if visible_count == 0 {
            return None;
        }
        let norm_x = rel_pos.x / plot_rect.width();
        let vis_idx = ((norm_x * visible_count as f32) as usize).min(visible_count - 1);
        Some(vis_idx)
    }
}

pub struct HorizontalMapper;
impl PointMapper for HorizontalMapper {
    fn map(
        &self,
        idx: usize,
        point: &ChartPoint,
        max_val: f64,
        plot_rect: egui::Rect,
    ) -> egui::Pos2 {
        let norm_x = (point.value / max_val.max(1.0)) as f32;
        let x = plot_rect.left() + norm_x * plot_rect.width();
        let y = plot_rect.top() + ((idx as f32 + 0.5) / 10.0) * plot_rect.height();
        egui::pos2(x, y)
    }

    fn detect(
        &self,
        rel_pos: egui::Vec2,
        plot_rect: egui::Rect,
        config: &ChartConfig,
    ) -> Option<usize> {
        if rel_pos.x < 0.0
            || rel_pos.x > plot_rect.width()
            || rel_pos.y < 0.0
            || rel_pos.y > plot_rect.height()
        {
            return None;
        }
        let visible_count = config.visible_points().len();
        if visible_count == 0 {
            return None;
        }
        let norm_y = rel_pos.y / plot_rect.height();
        let vis_idx = ((norm_y * visible_count as f32) as usize).min(visible_count - 1);
        Some(vis_idx)
    }
}

pub trait RenderStrategy {
    fn draw_geometry(
        &self,
        painter: &egui::Painter,
        config: &ChartConfig,
        state: &PlotState,
        plot_rect: egui::Rect,
    );
    fn draw_point(&self, painter: &egui::Painter, pos: egui::Pos2, idx: usize, is_hovered: bool);
}

pub struct PointRenderer;
impl RenderStrategy for PointRenderer {
    fn draw_geometry(
        &self,
        _painter: &egui::Painter,
        _config: &ChartConfig,
        _state: &PlotState,
        _plot_rect: egui::Rect,
    ) {
    }

    fn draw_point(&self, painter: &egui::Painter, pos: egui::Pos2, _idx: usize, is_hovered: bool) {
        let (radius, color) = if is_hovered {
            (7.0, egui::Color32::from_rgb(255, 200, 0))
        } else {
            let hue = (_idx as f32 / 10.0) % 1.0;
            (5.0, hsv_to_rgb(hue, 0.7, 0.9))
        };
        painter.circle_filled(pos, radius, color);
        painter.circle_stroke(pos, radius, egui::Stroke::new(1.0, egui::Color32::WHITE));
    }
}

pub struct BarRenderer;
impl RenderStrategy for BarRenderer {
    fn draw_geometry(
        &self,
        _painter: &egui::Painter,
        _config: &ChartConfig,
        _state: &PlotState,
        _plot_rect: egui::Rect,
    ) {
    }

    fn draw_point(&self, painter: &egui::Painter, pos: egui::Pos2, idx: usize, is_hovered: bool) {
        let color = if is_hovered {
            egui::Color32::from_rgb(255, 200, 0)
        } else {
            let hue = (idx as f32 / 10.0) % 1.0;
            hsv_to_rgb(hue, 0.7, 0.9)
        };
        let rect = egui::Rect::from_center_size(pos, egui::vec2(40.0, 20.0));
        painter.rect_filled(rect, 0.0, color);
        painter.rect_stroke(
            rect,
            0.0,
            egui::Stroke::new(1.0, egui::Color32::from_gray(100)),
        );
    }
}

pub trait TooltipHandler: Sized {
    fn render_tooltip(
        &self,
        painter: &egui::Painter,
        ctx: &egui::Context,
        point: &ChartPoint,
        point_pos: egui::Pos2,
        zoom: f32,
    );
}

pub struct DefaultTooltipHandler;
impl TooltipHandler for DefaultTooltipHandler {
    fn render_tooltip(
        &self,
        _painter: &egui::Painter,
        _ctx: &egui::Context,
        _point: &ChartPoint,
        _point_pos: egui::Pos2,
        _zoom: f32,
    ) {
    }
}

pub struct TooltipStyle {
    pub bg: (u8, u8, u8, u8),
    pub text: (u8, u8, u8, u8),
}

impl TooltipStyle {
    pub fn default_dark() -> Self {
        Self {
            bg: (30, 30, 40, 220),
            text: (255, 255, 255, 255),
        }
    }

    pub fn new(bg: (u8, u8, u8, u8), text: (u8, u8, u8, u8)) -> Self {
        Self { bg, text }
    }
}

pub struct RichTooltipHandler {
    pub tooltip_bg: (u8, u8, u8, u8),
    pub tooltip_text: (u8, u8, u8, u8),
    pub image_loader: Option<std::sync::Arc<dyn std::any::Any>>,
}

impl RichTooltipHandler {
    pub fn with_style(style: TooltipStyle) -> Self {
        Self {
            tooltip_bg: style.bg,
            tooltip_text: style.text,
            image_loader: None,
        }
    }
}

impl TooltipHandler for RichTooltipHandler {
    fn render_tooltip(
        &self,
        painter: &egui::Painter,
        ctx: &egui::Context,
        point: &ChartPoint,
        point_pos: egui::Pos2,
        zoom: f32,
    ) {
        let has_image = point.hover_data.contains_key("image");
        let field_count = point
            .hover_data
            .iter()
            .filter(|(k, _)| k.as_str() != "image")
            .count() as f32;

        let actual_image_height = if has_image { 120.0 * zoom } else { 0.0 };
        let field_height = 18.0 * zoom;
        let padding = 20.0 * zoom;
        let actual_content_height = actual_image_height + (field_count * field_height) + padding;

        let mut max_text_width: f32 = 100.0;
        for (key, val) in &point.hover_data {
            if key != "image" {
                let display = format!("{}: {}", key, val);
                let galley = ctx.fonts(|f| {
                    f.layout_no_wrap(
                        display,
                        egui::FontId::proportional(10.0),
                        egui::Color32::WHITE,
                    )
                });
                max_text_width = max_text_width.max(galley.rect.width());
            }
        }

        let tooltip_w = (max_text_width * 1.2 + 40.0 * zoom).max(120.0 * zoom);
        let tooltip_h = actual_content_height.max(100.0 * zoom);
        let tooltip_y = point_pos.y - 20.0 * zoom - tooltip_h;
        let tooltip_x = point_pos.x - tooltip_w / 2.0;

        let tooltip_rect = egui::Rect::from_min_size(
            egui::pos2(tooltip_x, tooltip_y),
            egui::vec2(tooltip_w, tooltip_h),
        );

        let (bg_r, bg_g, bg_b, bg_a) = self.tooltip_bg;
        let bg_color = egui::Color32::from_rgba_unmultiplied(bg_r, bg_g, bg_b, bg_a);
        painter.rect_filled(tooltip_rect, 4.0, bg_color);

        let (txt_r, txt_g, txt_b, txt_a) = self.tooltip_text;
        let text_color = egui::Color32::from_rgba_unmultiplied(txt_r, txt_g, txt_b, txt_a);
        let font_size = 10.0;
        let font_id = egui::FontId::proportional(font_size);

        let mut field_offset = if has_image { 130.0 * zoom } else { 15.0 * zoom };

        if has_image {
            if let Some(_img_path) = point.hover_data.get("image") {
                painter.text(
                    egui::pos2(tooltip_x + tooltip_w / 2.0, tooltip_y + 10.0 * zoom),
                    egui::Align2::CENTER_TOP,
                    "🖼",
                    egui::FontId::proportional(font_size * 2.0),
                    text_color,
                );
            }
        }

        for (key, val) in &point.hover_data {
            if key != "image" {
                let display = format!("{}: {}", key, val);
                painter.text(
                    egui::pos2(tooltip_x + 8.0, tooltip_y + field_offset),
                    egui::Align2::LEFT_TOP,
                    display,
                    font_id.clone(),
                    text_color,
                );
                field_offset += field_height + 4.0 * zoom;
            }
        }

        painter.text(
            egui::pos2(tooltip_x + tooltip_w / 2.0, tooltip_y + tooltip_h - 20.0),
            egui::Align2::CENTER_TOP,
            &point.label,
            egui::FontId::proportional(font_size * 1.1),
            text_color,
        );

        painter.text(
            egui::pos2(tooltip_x + tooltip_w / 2.0, tooltip_y + tooltip_h - 8.0),
            egui::Align2::CENTER_TOP,
            &format!("{:.1}", point.value),
            egui::FontId::proportional(font_size * 1.2),
            text_color,
        );
    }
}

pub fn draw_axes(painter: &egui::Painter, plot_rect: egui::Rect) {
    painter.line_segment(
        [
            egui::pos2(plot_rect.left(), plot_rect.bottom()),
            egui::pos2(plot_rect.right(), plot_rect.bottom()),
        ],
        egui::Stroke::new(1.5, egui::Color32::from_gray(200)),
    );
    painter.line_segment(
        [
            egui::pos2(plot_rect.left(), plot_rect.top()),
            egui::pos2(plot_rect.left(), plot_rect.bottom()),
        ],
        egui::Stroke::new(1.5, egui::Color32::from_gray(200)),
    );
}

pub fn draw_scale(painter: &egui::Painter, plot_rect: egui::Rect, max_val: f64, vertical: bool) {
    let font_size = 11.0;
    if vertical {
        for i in 0..=5 {
            let y = plot_rect.bottom() - (plot_rect.height() / 5.0) * i as f32;
            let val = (max_val / 5.0) * i as f64;
            painter.text(
                egui::pos2(plot_rect.left() - 15.0, y),
                egui::Align2::RIGHT_CENTER,
                &format!("{:.1}", val),
                egui::FontId::proportional(font_size),
                egui::Color32::from_gray(100),
            );
        }
    } else {
        for i in 0..=5 {
            let x = plot_rect.left() + (plot_rect.width() / 5.0) * i as f32;
            let val = (max_val / 5.0) * i as f64;
            painter.text(
                egui::pos2(x, plot_rect.bottom() + 15.0),
                egui::Align2::CENTER_TOP,
                &format!("{:.1}", val),
                egui::FontId::proportional(font_size),
                egui::Color32::from_gray(100),
            );
        }
    }
}

pub fn create_plot_rect(response_rect: egui::Rect, dims: &PlotDimensions) -> egui::Rect {
    egui::Rect::from_min_size(
        response_rect.min + egui::Vec2::new(dims.pad_x, dims.pad_y),
        egui::Vec2::new(dims.width, dims.height),
    )
}

pub struct GenericRenderer<M: PointMapper, S: RenderStrategy, T: TooltipHandler> {
    mapper: M,
    strategy: S,
    tooltip: T,
}

impl<M: PointMapper, S: RenderStrategy> GenericRenderer<M, S, DefaultTooltipHandler> {
    pub fn new(mapper: M, strategy: S) -> Self {
        Self {
            mapper,
            strategy,
            tooltip: DefaultTooltipHandler,
        }
    }
}

impl<M: PointMapper, S: RenderStrategy, T: TooltipHandler> GenericRenderer<M, S, T> {
    pub fn with_tooltip(mapper: M, strategy: S, tooltip: T) -> Self {
        Self {
            mapper,
            strategy,
            tooltip,
        }
    }

    pub fn render(
        &self,
        config: &ChartConfig,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        dims: PlotDimensions,
        hovered_idx: &mut Option<usize>,
    ) {
        let max_val = config.max_value();
        let (response, painter) = ui.allocate_painter(
            egui::Vec2::new(dims.total_width(), dims.total_height()),
            egui::Sense::hover(),
        );

        let plot_rect = dims.plot_rect(response.rect.min);

        draw_axes(&painter, plot_rect);
        draw_scale(&painter, plot_rect, max_val, dims.width > dims.height);

        let visible_indices: Vec<usize> = config
            .points
            .iter()
            .enumerate()
            .filter(|(_, p)| p.visible)
            .map(|(i, _)| i)
            .collect();

        for (vis_idx, &actual_idx) in visible_indices.iter().enumerate() {
            let point = &config.points[actual_idx];
            let pos = self.mapper.map(vis_idx, point, max_val, plot_rect);
            let is_hovered = hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
            self.strategy
                .draw_point(&painter, pos, actual_idx, is_hovered);
            if is_hovered {
                self.tooltip
                    .render_tooltip(&painter, ctx, point, pos, config.zoom);
            }
        }

        if response.hovered() {
            if let Some(pos) = ctx.pointer_latest_pos() {
                let rel_pos = pos - plot_rect.min;
                if let Some(vis_idx) = self.mapper.detect(rel_pos, plot_rect, config) {
                    *hovered_idx = Some(visible_indices[vis_idx]);
                }
            }
        }
    }
}

pub struct ChartBuilder<M: PointMapper, S: RenderStrategy> {
    mapper: M,
    strategy: S,
    tooltip_style: Option<TooltipStyle>,
}

impl<M: PointMapper, S: RenderStrategy> ChartBuilder<M, S> {
    pub fn new(mapper: M, strategy: S) -> Self {
        Self {
            mapper,
            strategy,
            tooltip_style: None,
        }
    }

    pub fn with_default_tooltip(mut self) -> Self {
        self.tooltip_style = Some(TooltipStyle::default_dark());
        self
    }

    pub fn build(self) -> GenericRenderer<M, S, RichTooltipHandler> {
        let tooltip_style = self
            .tooltip_style
            .unwrap_or_else(TooltipStyle::default_dark);
        let tooltip = RichTooltipHandler::with_style(tooltip_style);
        GenericRenderer::with_tooltip(self.mapper, self.strategy, tooltip)
    }

    pub fn build_default(self) -> GenericRenderer<M, S, DefaultTooltipHandler> {
        GenericRenderer::new(self.mapper, self.strategy)
    }
}
