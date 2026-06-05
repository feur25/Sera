pub mod _3d;
pub mod bar;
pub mod chart;
pub mod line;
pub mod scatter;
pub mod svg;

pub use bar::build_hbar;
pub use bar::{render_bars, render_bars_fast, render_bars_html};
pub use line::{render_lines, render_lines_fast, render_lines_html};
pub mod kmeans;
pub use crate::plot::controller::chart_controller::{
    get_current_group_types, render_by_type, set_current_chart_group, ChartGroupBuilder,
    ChartTypeBuilder,
};
pub use _3d::render_bar3d_html;
pub use _3d::render_line3d_html;
pub use _3d::render_scatter3d_html;
pub use _3d::*;
pub use _3d::{Bar3DRenderContext, Line3DRenderContext, Scatter3DRenderContext};
pub use bar::build_bar_chart;
pub use chart::register_default_types;
pub use kmeans::build_kmeans_chart;
pub use kmeans::{
    kmeans_core_2d, kmeans_core_flat, kmeans_core_flat_ninit, kmeans_core_nd,
    minibatch_kmeans_core_2d, minibatch_kmeans_core_flat, minibatch_kmeans_core_nd,
    render_kmeans_html, KMeansConfig,
};
pub use line::build_line_chart;
pub use scatter::build_dbscan_chart;
pub use scatter::build_dbscan_chart_3d;
pub use scatter::{render_dbscan_html, render_points, render_scatter_fast, render_scatter_html};
pub use svg::SvgChart;

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
    if let Ok(_reg) = crate::plot::controller::chart_controller::get_registry().lock() {
        if let Some(ctx) = crate::plot::controller::chart_controller::get_svg_renderer(chart_type) {
            ctx(
                svg,
                values,
                colors,
                pad,
                plot_width,
                plot_height,
                max_val,
                vertical,
            );
        }
    }
}
