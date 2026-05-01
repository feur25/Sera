pub mod common;
pub mod _3d;
pub mod pie;
pub mod heatmap;
pub mod histogram;
pub mod grouped_bar;
pub mod bar;
pub mod line;
pub mod multiline;
pub mod area;
pub mod treemap;
pub mod boxplot;
pub mod funnel;
pub mod sunburst;
pub mod waterfall;
pub mod violin;
pub mod slope;
pub mod bullet;
pub mod wordcloud;
pub mod radar;
pub mod lollipop;
pub mod kde;
pub mod ridgeline;
pub mod candlestick;
pub mod dumbbell;
pub mod bubble;
pub mod scatter;
pub mod gauge;
pub mod parallel;

pub use pie::{Pie, PieConfig, PieVariant, render_pie_html};
pub use heatmap::{Heatmap, HeatmapConfig, HeatmapVariant, render_heatmap_html};
pub use histogram::{Histogram, HistogramConfig, HistogramVariant, render_histogram_html, compute_bins};
pub use grouped_bar::{GroupedBar, GroupedBarConfig, render_grouped_bar_html};
pub use bar::{BarVariant, BarConfig, render_bar_html};
pub use line::{LineVariant, LineConfig, render_line_html};
pub use multiline::{MultiLine, MultiLineConfig, render_multiline_html};
pub use area::{Area, AreaConfig, render_area_html};
pub use treemap::{Treemap, TreemapConfig, render_treemap_html};
pub use boxplot::{BoxplotConfig, render_boxplot_html};
pub use funnel::{FunnelConfig, render_funnel_html};
pub use sunburst::{SunburstConfig, render_sunburst_html};
pub use waterfall::{WaterfallConfig, render_waterfall_html};
pub use violin::{ViolinConfig, render_violin_html};
pub use slope::{SlopeConfig, render_slope_html};
pub use bullet::{BulletConfig, render_bullet_html};
pub use wordcloud::{WordCloudConfig, render_wordcloud_html};
pub use radar::{RadarConfig, render_radar_html};
pub use lollipop::{LollipopConfig, render_lollipop_html};
pub use kde::{KdeConfig, render_kde_html, scott_bw, kde_eval};
pub use ridgeline::{RidgelineConfig, render_ridgeline_html};
pub use crate::html::hover::{HoverSlot, slots_to_json, parse_hover_json};
pub use _3d::register_statistical_3d_types;

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
