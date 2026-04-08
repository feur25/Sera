pub mod bar;
pub mod line;
pub mod scatter;
pub mod _3d;
pub mod svg;
pub mod chart;

pub use bar::{render_bars, render_bars_fast, render_bars_html};
pub use line::{render_lines, render_lines_fast, render_lines_html};
pub use scatter::{render_points, render_scatter_fast, render_scatter_html};
pub use _3d::render_scatter3d_html;
pub use _3d::render_bar3d_html;
pub use _3d::render_line3d_html;
pub use _3d::{Bar3DRenderContext, Line3DRenderContext, Scatter3DRenderContext};
pub use _3d::*;
pub use svg::SvgChart;
pub use chart::register_default_types;
pub use crate::plot::controller::chart_controller::{ChartTypeBuilder, ChartGroupBuilder, render_by_type, get_current_group_types, set_current_chart_group};

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

pub fn render_chart_by_type(
    chart_type: u8,
    svg: &mut String,
    values: &[f64],
    colors: &[&'static str],
    pad: i32,
    plot_width: i32,
    plot_height: i32,
    max_val: f64,
    vertical: bool,
) {
    if let Ok(reg) = crate::plot::controller::chart_controller::get_registry().lock() {
        if let Some(ctx) = crate::plot::controller::chart_controller::get_svg_renderer(chart_type) {
            ctx(svg, values, colors, pad, plot_width, plot_height, max_val, vertical);
        }
    }
}
