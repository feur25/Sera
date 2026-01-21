pub mod bar;
pub mod line;
pub mod scatter;
pub mod _3d;
pub mod svg;
pub mod chart;
pub mod scale_renderer;

pub use bar::render_bars;
pub use line::render_lines;
pub use scatter::render_points;
pub use _3d::{Bar3DRenderContext, Line3DRenderContext, Scatter3DRenderContext};
pub use _3d::*;
pub use svg::SvgChart;
pub use chart::register_default_types;
pub use crate::plot::controller::chart_controller::{ChartTypeBuilder, ChartGroupBuilder, render_by_type, get_current_group_types, set_current_chart_group};
pub use scale_renderer::*;
pub struct PlotRenderContext<'a> {
    pub painter: &'a egui::Painter,
    pub plot_rect: egui::Rect,
    pub colors: &'a [egui::Color32],
    pub hovered_idx: Option<usize>,
    pub values: &'a [f64],
    pub max_val: f64,
    pub visible_indices: &'a [usize],
    pub vertical: bool,
    pub labels: &'a [String],
}

pub fn render_plot_by_type(chart_type: u8, ctx: PlotRenderContext) {
    render_by_type(chart_type, ctx);
}
