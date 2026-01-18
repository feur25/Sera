use super::generic::*;
use super::renderers::ChartConfig;

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

