use super::generic::*;
use super::renderers::ChartConfig;

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
