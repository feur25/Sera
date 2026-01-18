use super::generic::*;
use super::renderers::ChartConfig;

pub fn render_scatter_vertical(
    config: &ChartConfig,
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    hovered_idx: &mut Option<usize>,
) {
    let dims = PlotDimensions::vertical(config.zoom);
    let tooltip = RichTooltipHandler {
        tooltip_bg: (30, 30, 40, 220),
        tooltip_text: (255, 255, 255, 255),
        image_loader: None,
    };
    let renderer = GenericRenderer::with_tooltip(VerticalMapper, PointRenderer, tooltip);
    renderer.render(config, ctx, ui, dims, hovered_idx);
}

pub fn render_scatter_horizontal(
    config: &ChartConfig,
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    hovered_idx: &mut Option<usize>,
) {
    let dims = PlotDimensions::horizontal(config.zoom);
    let tooltip = RichTooltipHandler {
        tooltip_bg: (30, 30, 40, 220),
        tooltip_text: (255, 255, 255, 255),
        image_loader: None,
    };
    let renderer = GenericRenderer::with_tooltip(HorizontalMapper, PointRenderer, tooltip);
    renderer.render(config, ctx, ui, dims, hovered_idx);
}
