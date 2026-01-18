use super::generic::*;
use super::renderers::ChartConfig;

pub struct LineRenderContext<'a> {
    pub painter: &'a egui::Painter,
    pub plot_rect: egui::Rect,
    pub colors: &'a [egui::Color32],
    pub hovered_idx: Option<usize>,
    pub values: &'a [f64],
    pub max_val: f64,
    pub visible_indices: &'a [usize],
    pub vertical: bool,
}

pub fn render_lines(ctx: LineRenderContext) {
    if ctx.visible_indices.len() < 2 {
        return;
    }
    
    let visible_count = ctx.visible_indices.len();
    let mut prev_pos: Option<egui::Pos2> = None;
    
    for (vis_idx, &actual_idx) in ctx.visible_indices.iter().enumerate() {
        let value = ctx.values[actual_idx];
        let norm_val = value / ctx.max_val.max(1.0);
        
        let pos = if ctx.vertical {
            let x = ctx.plot_rect.left() + (ctx.plot_rect.width() / (visible_count as f32 - 1.0).max(1.0)) * vis_idx as f32;
            let y = ctx.plot_rect.bottom() - norm_val as f32 * ctx.plot_rect.height();
            egui::pos2(x, y)
        } else {
            let x = ctx.plot_rect.left() + norm_val as f32 * ctx.plot_rect.width();
            let y = ctx.plot_rect.top() + (ctx.plot_rect.height() / (visible_count as f32 - 1.0).max(1.0)) * vis_idx as f32;
            egui::pos2(x, y)
        };
        
        let color = ctx.colors[actual_idx % ctx.colors.len()];
        if let Some(p) = prev_pos {
            ctx.painter.line_segment([p, pos], egui::Stroke::new(2.0, color));
        }
        
        let is_hovered = ctx.hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
        let (radius, display_color) = if is_hovered { 
            (6.0, egui::Color32::from_rgb(255, 200, 0)) 
        } else { 
            (4.0, color) 
        };
        ctx.painter.circle_filled(pos, radius, display_color);
        prev_pos = Some(pos);
    }
}

struct LineStrategy;
impl RenderStrategy for LineStrategy {
    fn draw_geometry(&self, painter: &egui::Painter, config: &ChartConfig, state: &PlotState, plot_rect: egui::Rect) {
        let visible: Vec<_> = config.points.iter().filter(|p| p.visible).collect();
        if visible.len() > 1 {
            let is_vertical = plot_rect.width() > plot_rect.height();
            let mut positions = Vec::new();
            for (idx, point) in visible.iter().enumerate() {
                let pos = if is_vertical {
                    VerticalMapper.map(idx, point, state.max_val, plot_rect)
                } else {
                    HorizontalMapper.map(idx, point, state.max_val, plot_rect)
                };
                positions.push(pos);
            }
            for window in positions.windows(2) {
                painter.line_segment([window[0], window[1]], egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 150, 255)));
            }
        }
    }

    fn draw_point(&self, painter: &egui::Painter, pos: egui::Pos2, _idx: usize, is_hovered: bool) {
        let (radius, color) = if is_hovered {
            (6.0, egui::Color32::from_rgb(255, 200, 0))
        } else {
            (4.0, egui::Color32::from_rgb(100, 150, 255))
        };
        painter.circle_filled(pos, radius, color);
        painter.circle_stroke(pos, radius, egui::Stroke::new(1.0, egui::Color32::WHITE));
    }
}

fn render_line<M: PointMapper>(config: &ChartConfig, ctx: &egui::Context, ui: &mut egui::Ui, hovered_idx: &mut Option<usize>, mapper: M) {
    let dims = if config.orientation { PlotDimensions::vertical(config.zoom) } else { PlotDimensions::horizontal(config.zoom) };
    let renderer = GenericRenderer::new(mapper, LineStrategy);
    renderer.render(config, ctx, ui, dims, hovered_idx);
}

pub fn render_line_vertical(config: &ChartConfig, ctx: &egui::Context, ui: &mut egui::Ui, hovered_idx: &mut Option<usize>,) {
    render_line(config, ctx, ui, hovered_idx, VerticalMapper);
}

pub fn render_line_horizontal(config: &ChartConfig, ctx: &egui::Context, ui: &mut egui::Ui, hovered_idx: &mut Option<usize>,) {
    render_line(config, ctx, ui, hovered_idx, HorizontalMapper);
}
