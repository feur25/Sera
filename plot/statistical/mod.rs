pub mod common;
pub mod pie;
pub mod heatmap;
pub mod histogram;
pub mod grouped_bar;

pub use pie::{Pie, PieConfig, render_pie_html};
pub use heatmap::{Heatmap, HeatmapConfig, render_heatmap_html};
pub use histogram::{Histogram, HistogramConfig, render_histogram_html, compute_bins};
pub use grouped_bar::{GroupedBar, GroupedBarConfig, render_grouped_bar_html};
pub use crate::html::hover::{HoverSlot, slots_to_json, parse_hover_json};

use crate::plot::controller::chart_controller::{ChartTypeBuilder, get_group_registry};

pub const PIE_ID:          u8 = 60;
pub const HEATMAP_ID:      u8 = 61;
pub const HISTOGRAM_ID:    u8 = 62;
pub const GROUPED_BAR_ID:  u8 = 63;

pub fn register_statistical_types() {
    use crate::plot::default::PlotRenderContext;

    fn noop_renderer(_ctx: PlotRenderContext) {}
    fn noop_svg_renderer(
        _svg: &mut String, _values: &[f64], _colors: &[&'static str],
        _pad: i32, _plot_w: i32, _plot_h: i32, _max: f64, _vert: bool,
    ) {}

    let ids: Vec<u8> = vec![PIE_ID, HEATMAP_ID, HISTOGRAM_ID, GROUPED_BAR_ID];
    let names = ["pie", "heatmap", "histogram", "grouped_bar"];

    for (&id, &name) in ids.iter().zip(names.iter()) {
        let _ = ChartTypeBuilder::new(id)
            .with_name(name)
            .with_renderer(noop_renderer)
            .build();
        if let Ok(mut reg) = crate::plot::controller::chart_controller::get_registry().lock() {
            reg.register_svg(id, noop_svg_renderer);
        }
    }

    if let Ok(mut grp_reg) = get_group_registry().lock() {
        grp_reg.register_group("statistical".to_string(), ids);
    }
}
