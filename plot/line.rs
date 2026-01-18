use super::generic::*;
use super::renderers::ChartConfig;

struct LineStrategy;
impl RenderStrategy for LineStrategy {
    fn draw_geometry(&self, painter: &egui::Painter, config: &ChartConfig, state: &PlotState, plot_rect: egui::Rect) {
        let visible: Vec<_> = config.points.iter().filter(|p| p.visible).collect();
        if visible.len() > 1 {
            let mut positions = Vec::new();
            let is_vertical = plot_rect.width() > plot_rect.height();
            
            for (idx, point) in visible.iter().enumerate() {
                let pos = if is_vertical {
                    VerticalMapper.map(idx, point, state.max_val, plot_rect)
                } else {
                    HorizontalMapper.map(idx, point, state.max_val, plot_rect)
                };
                positions.push(pos);
            }

            for window in positions.windows(2) {
                painter.line_segment(
                    [window[0], window[1]],
                    egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 150, 255)),
                );
            }
        }
    }

    fn draw_point(&self, painter: &egui::Painter, pos: egui::Pos2, idx: usize, is_hovered: bool) {
        let (radius, color) = if is_hovered {
            (6.0, egui::Color32::from_rgb(255, 200, 0))
        } else {
            (4.0, egui::Color32::from_rgb(100, 150, 255))
        };
        painter.circle_filled(pos, radius, color);
        painter.circle_stroke(pos, radius, egui::Stroke::new(1.0, egui::Color32::WHITE));
    }
}

pub fn render_line_vertical(
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
    let renderer = GenericRenderer::with_tooltip(VerticalMapper, LineStrategy, tooltip);
    renderer.render(config, ctx, ui, dims, hovered_idx);
}

pub fn render_line_horizontal(
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
    let renderer = GenericRenderer::with_tooltip(HorizontalMapper, LineStrategy, tooltip);
    renderer.render(config, ctx, ui, dims, hovered_idx);
}
