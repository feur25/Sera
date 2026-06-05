pub mod _3d;
pub mod area;
pub mod bar;
pub mod boxplot;
pub mod bubble;
pub mod bullet;
pub mod candlestick;
pub mod common;
pub mod dumbbell;
pub mod funnel;
pub mod gauge;
pub mod grouped_bar;
pub mod heatmap;
pub mod histogram;
pub mod kde;
pub mod line;
pub mod lollipop;
pub mod multiline;
pub mod parallel;
pub mod pie;
pub mod radar;
pub mod ridgeline;
pub mod scatter;
pub mod slope;
pub mod sunburst;
pub mod theme;
pub mod treemap;
pub mod violin;
pub mod waterfall;
pub mod wordcloud;

pub use area::build_area_chart;
pub use bar::build_bar;
pub use boxplot::build_boxplot;
pub use bubble::build_bubble;
pub use bullet::build_bullet;
pub use candlestick::build_candlestick;
pub use dumbbell::build_dumbbell;
pub use funnel::build_funnel;
pub use gauge::build_gauge;
pub use grouped_bar::build_grouped_bar;
pub use grouped_bar::build_stacked_bar;
pub use heatmap::build_heatmap;
pub use histogram::build_histogram;
pub use kde::build_kde_chart;
pub use line::build_line;
pub use lollipop::build_lollipop_chart;
pub use multiline::build_multiline_chart;
pub use parallel::build_parallel;
pub use pie::build_donut_chart;
pub use pie::build_pie;
pub use pie::build_pie_chart;
pub use radar::build_radar_chart;
pub use ridgeline::build_ridgeline_chart;
pub use scatter::build_scatter_chart;
pub use slope::build_slope;
pub use sunburst::build_sunburst;
pub use treemap::build_treemap;
pub use violin::build_violin;
pub use waterfall::build_waterfall;
pub use wordcloud::build_wordcloud;

pub use crate::html::hover::{parse_hover_json, slots_to_json, HoverSlot};
pub use _3d::register_statistical_3d_types;
pub use _3d::*;
pub use area::{render_area_html, Area, AreaConfig};
pub use bar::{render_bar_html, BarConfig, BarVariant};
pub use boxplot::{render_boxplot_html, BoxplotConfig, BoxplotVariant};
pub use bubble::BubbleVariant;
pub use bullet::{render_bullet_html, BulletConfig, BulletVariant};
pub use candlestick::{render_candlestick_html, CandlestickConfig, CandlestickVariant};
pub use dumbbell::{render_dumbbell_html, DumbbellConfig, DumbbellVariant};
pub use funnel::{render_funnel_html, FunnelConfig, FunnelVariant};
pub use gauge::{render_gauge_html, GaugeConfig, GaugeVariant};
pub use grouped_bar::{render_grouped_bar_html, GroupedBar, GroupedBarConfig};
pub use heatmap::{render_heatmap_html, Heatmap, HeatmapConfig, HeatmapVariant};
pub use histogram::{
    compute_bins, render_histogram_html, Histogram, HistogramConfig, HistogramVariant,
};
pub use kde::{kde_eval, render_kde_html, scott_bw, KdeConfig, KdeVariant};
pub use line::{render_line_html, LineConfig, LineVariant};
pub use lollipop::{render_lollipop_html, LollipopConfig, LollipopVariant};
pub use multiline::{render_multiline_html, MultiLine, MultiLineConfig};
pub use parallel::{render_parallel_html, ParallelConfig, ParallelVariant};
pub use pie::{render_pie_html, Pie, PieConfig, PieVariant};
pub use radar::{render_radar_html, RadarConfig, RadarVariant};
pub use ridgeline::{render_ridgeline_html, RidgelineConfig, RidgelineVariant};
pub use scatter::ScatterVariant;
pub use slope::{render_slope_html, SlopeConfig, SlopeVariant};
pub use sunburst::{render_sunburst_html, SunburstConfig, SunburstVariant};
pub use theme::{apply_chart_theme, ChartTheme};
pub use treemap::{render_treemap_html, TreemapConfig, TreemapVariant};
pub use violin::{render_violin_html, ViolinConfig, ViolinVariant};
pub use waterfall::{render_waterfall_html, WaterfallConfig, WaterfallVariant};
pub use wordcloud::{render_wordcloud_html, WordCloudConfig, WordCloudShape, WordCloudVariant};

use crate::plot::controller::chart_controller::{get_group_registry, ChartTypeBuilder};

pub const PIE_ID: u8 = 60;
pub const HEATMAP_ID: u8 = 61;
pub const HISTOGRAM_ID: u8 = 62;
pub const GROUPED_BAR_ID: u8 = 63;

pub fn register_statistical_types() {
    use crate::plot::default::PlotRenderContext;

    fn noop_renderer(_ctx: PlotRenderContext) {}
    fn noop_svg_renderer(
        _svg: &mut String,
        _values: &[f64],
        _colors: &[&'static str],
        _pad: i32,
        _plot_w: i32,
        _plot_h: i32,
        _max: f64,
        _vert: bool,
    ) {
    }

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
