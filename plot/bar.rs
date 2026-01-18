use super::generic::*;
use super::renderers::ChartConfig;

pub fn render_bar_vertical(
    config: &ChartConfig,
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    hovered_idx: &mut Option<usize>,
) {
    let renderer = ChartBuilder::new(VerticalMapper, BarRenderer)
        .with_default_tooltip()
        .build();
    renderer.render(config, ctx, ui, PlotDimensions::vertical(config.zoom), hovered_idx);
}

pub fn render_bar_horizontal(
    config: &ChartConfig,
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    hovered_idx: &mut Option<usize>,
) {
    let renderer = ChartBuilder::new(HorizontalMapper, BarRenderer)
        .with_default_tooltip()
        .build();
    renderer.render(config, ctx, ui, PlotDimensions::horizontal(config.zoom), hovered_idx);
}
