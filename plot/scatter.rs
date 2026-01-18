use super::generic::*;
use super::renderers::ChartConfig;

pub struct ScatterRenderContext<'a> {
    pub painter: &'a egui::Painter,
    pub plot_rect: egui::Rect,
    pub colors: &'a [egui::Color32],
    pub hovered_idx: Option<usize>,
    pub values: &'a [f64],
    pub max_val: f64,
    pub visible_indices: &'a [usize],
    pub vertical: bool,
}

pub fn render_points(ctx: ScatterRenderContext) {
    let visible_count = ctx.visible_indices.len();
    
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
        
        let is_hovered = ctx.hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
        let color = ctx.colors[actual_idx % ctx.colors.len()];
        let (radius, display_color) = if is_hovered { 
            (6.0, egui::Color32::from_rgb(255, 200, 0)) 
        } else { 
            (4.0, color) 
        };
        ctx.painter.circle_filled(pos, radius, display_color);
    }
}

fn render_scatter<M: PointMapper>(config: &ChartConfig, ctx: &egui::Context, ui: &mut egui::Ui, hovered_idx: &mut Option<usize>, mapper: M) {
    let dims = if config.orientation { PlotDimensions::vertical(config.zoom) } else { PlotDimensions::horizontal(config.zoom) };
    let renderer = GenericRenderer::new(mapper, PointRenderer);
    renderer.render(config, ctx, ui, dims, hovered_idx);
}

pub fn render_scatter_vertical(config: &ChartConfig, ctx: &egui::Context, ui: &mut egui::Ui, hovered_idx: &mut Option<usize>) {
    render_scatter(config, ctx, ui, hovered_idx, VerticalMapper);
}

pub fn render_scatter_horizontal(config: &ChartConfig, ctx: &egui::Context, ui: &mut egui::Ui, hovered_idx: &mut Option<usize>) {
    render_scatter(config, ctx, ui, hovered_idx, HorizontalMapper);
}

