use super::generic::*;
use super::renderers::ChartConfig;
use std::collections::HashMap;

pub struct BarRenderContext<'a> {
    pub painter: &'a egui::Painter,
    pub plot_rect: egui::Rect,
    pub colors: &'a [egui::Color32],
    pub hovered_idx: Option<usize>,
    pub values: &'a [f64],
    pub max_val: f64,
    pub visible_indices: &'a [usize],
    pub vertical: bool,
}

pub fn render_bars(ctx: BarRenderContext) {
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
        
        let color = ctx.colors[actual_idx % ctx.colors.len()];
        let is_hovered = ctx.hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
        let bar_width = (ctx.plot_rect.width() / visible_count as f32) * 0.6;
        let bar_height = ctx.plot_rect.bottom() - pos.y;
        
        let rect = if ctx.vertical {
            egui::Rect::from_min_size(
                egui::pos2(pos.x - bar_width / 2.0, pos.y),
                egui::vec2(bar_width, bar_height),
            )
        } else {
            egui::Rect::from_min_size(
                egui::pos2(ctx.plot_rect.left(), pos.y - bar_width / 2.0),
                egui::vec2(bar_height, bar_width),
            )
        };
        
        let display_color = if is_hovered { 
            egui::Color32::from_rgb(255, 200, 0) 
        } else { 
            color 
        };
        ctx.painter.rect_filled(rect, 0.0, display_color);
    }
}

fn render_bar_generic<M: PointMapper>(
    config: &ChartConfig,
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    hovered_idx: &mut Option<usize>,
    mapper: M,
) {
    let dims = if config.orientation {
        PlotDimensions::vertical(config.zoom)
    } else {
        PlotDimensions::horizontal(config.zoom)
    };
    let renderer = GenericRenderer::new(mapper, BarRenderer);
    renderer.render(config, ctx, ui, dims, hovered_idx);
}

pub fn render_bar_vertical(
    config: &ChartConfig,
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    hovered_idx: &mut Option<usize>,
) {
    render_bar_generic(config, ctx, ui, hovered_idx, VerticalMapper);
}

pub fn render_bar_horizontal(
    config: &ChartConfig,
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    hovered_idx: &mut Option<usize>,
) {
    render_bar_generic(config, ctx, ui, hovered_idx, HorizontalMapper);
}
