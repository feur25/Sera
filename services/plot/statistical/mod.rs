pub mod _3d;
pub mod area;
pub mod bar;
pub mod boxplot;
pub mod bubble;
pub mod bullet;
pub mod candlestick;
pub mod chart_registry;
pub mod common;
pub mod dumbbell;
pub mod eventplot;
pub mod facet;
pub mod funnel;
pub mod gantt;
pub mod gauge;
pub mod grouped_bar;
pub mod heatmap;
pub mod hexbin;
pub mod histogram;
pub mod icicle;
pub mod joint;
pub mod kde;
pub mod line;
pub mod lollipop;
pub mod multiline;
pub mod parallel;
pub mod parcats;
pub mod pie;
pub mod radar;
pub mod ridgeline;
pub mod scatter;
pub mod scatterternary;
pub mod slope;
pub mod splom;
pub mod stackplot;
pub mod sunburst;
pub mod theme;
pub mod treemap;
pub mod violin;
pub mod waterfall;
pub mod wordcloud;

pub mod arc_diagram;
pub mod chord;
pub mod circle_pack;
pub mod circos;
pub mod correlogram;
pub mod dendrogram;
pub mod hive;
pub mod orbita;
pub mod plot_web;
pub mod pulse;
pub mod sankey;
pub mod venn;

pub use arc_diagram::build_arc_diagram;
pub use chord::build_chord;
pub use circle_pack::build_circle_pack;
pub use circos::build_circos;
pub use correlogram::build_correlogram;
pub use dendrogram::build_dendrogram;
pub use hive::build_hive;
pub use orbita::build_orbita;
pub use plot_web::build_plot_web;
pub use pulse::build_pulse;
pub use sankey::build_sankey;
pub use venn::build_venn;

pub use area::build_area_chart;
pub use bar::build_bar;
pub use boxplot::build_boxplot;
pub use bubble::build_bubble;
pub use bullet::build_bullet;
pub use candlestick::build_candlestick;
pub use dumbbell::build_dumbbell;
pub use eventplot::build_eventplot;
pub use facet::build_facet;
pub use funnel::build_funnel;
pub use gantt::build_gantt;
pub use gauge::build_gauge;
pub use grouped_bar::build_grouped_bar;
pub use grouped_bar::build_stacked_bar;
pub use heatmap::build_heatmap;
pub use hexbin::build_hexbin;
pub use histogram::build_histogram;
pub use icicle::build_icicle;
pub use joint::build_joint;
pub use kde::build_kde_chart;
pub use line::build_line;
pub use lollipop::build_lollipop_chart;
pub use multiline::build_multiline_chart;
pub use parallel::build_parallel;
pub use parcats::build_parcats;
pub use pie::build_donut_chart;
pub use pie::build_pie;
pub use pie::build_pie_chart;
pub use radar::build_radar_chart;
pub use ridgeline::build_ridgeline_chart;
pub use scatter::build_scatter_chart;
pub use scatterternary::build_scatter_ternary;
pub use slope::build_slope;
pub use splom::build_splom;
pub use stackplot::build_stackplot;
pub use sunburst::build_sunburst;
pub use treemap::build_treemap;
pub use violin::build_violin;
pub use waterfall::build_waterfall;
pub use wordcloud::build_wordcloud;

pub use crate::html::hover::{parse_hover_json, slots_to_json, HoverSlot};
pub use _3d::register_statistical_3d_types;
pub use _3d::*;
pub use area::{render_area_html, AreaConfig, AreaVariant};
pub use bar::{render_bar_html, BarConfig, BarVariant};
pub use boxplot::{render_boxplot_html, BoxplotConfig, BoxplotVariant};
pub use bubble::BubbleVariant;
pub use bullet::{render_bullet_html, BulletConfig, BulletVariant};
pub use candlestick::{render_candlestick_html, CandlestickConfig, CandlestickVariant};
pub use dumbbell::{render_dumbbell_html, DumbbellConfig, DumbbellVariant};
pub use eventplot::{render_eventplot_html, EventplotConfig, EventplotVariant};
pub use funnel::{render_funnel_html, FunnelConfig, FunnelVariant};
pub use gantt::{render_gantt_html, GanttConfig, GanttVariant};
pub use gauge::{render_gauge_html, GaugeConfig, GaugeVariant};
pub use grouped_bar::{render_grouped_bar_html, GroupedBar, GroupedBarConfig};
pub use heatmap::{render_heatmap_html, render_heatmap_native, Heatmap, HeatmapConfig, HeatmapVariant};
pub use histogram::{
    compute_bins, render_histogram_html, Histogram, HistogramConfig, HistogramVariant,
};
pub use hexbin::{render_hexbin_html, HexbinConfig, HexbinVariant};
pub use icicle::{render_icicle_html, IcicleConfig, IcicleVariant};
pub use kde::{kde_eval, render_kde_html, scott_bw, KdeConfig, KdeVariant};
pub use line::{render_line_html, LineConfig, LineVariant};
pub use lollipop::{render_lollipop_html, LollipopConfig, LollipopVariant};
pub use multiline::{render_multiline_html, MultiLine, MultiLineConfig};
pub use parallel::{render_parallel_html, ParallelConfig, ParallelVariant};
pub use parcats::{render_parcats_html, ParcatsConfig, ParcatsVariant};
pub use pie::{render_pie_html, render_pie_native, Pie, PieConfig, PieVariant};
pub use radar::{render_radar_html, RadarConfig, RadarVariant};
pub use ridgeline::{render_ridgeline_html, RidgelineConfig, RidgelineVariant};
pub use plot_web::{render_plot_web_html, PlotWebConfig, PlotWebVariant};
pub use scatter::ScatterVariant;
pub use scatterternary::{render_scatter_ternary_html, ScatterTernaryConfig, ScatterTernaryVariant};
pub use slope::{render_slope_html, SlopeConfig, SlopeVariant};
pub use splom::{render_splom_html, SplomConfig, SplomVariant};
pub use stackplot::{render_stackplot_html, StackplotConfig, StackplotVariant};
pub use sunburst::{render_sunburst_html, SunburstConfig, SunburstVariant};
pub use theme::{apply_chart_theme, ChartTheme};
pub use treemap::{render_treemap_html, TreemapConfig, TreemapVariant};
pub use violin::{render_violin_html, ViolinConfig, ViolinVariant};
pub use waterfall::{render_waterfall_html, WaterfallConfig, WaterfallVariant};
pub use wordcloud::{render_wordcloud_html, WordCloudConfig, WordCloudShape, WordCloudVariant};

use crate::plot::controller::chart_controller::{ChartTypeEntry, SvgChartRenderer};

pub const PIE_ID: u8 = 60;
pub const HEATMAP_ID: u8 = 61;
pub const HISTOGRAM_ID: u8 = 62;
pub const GROUPED_BAR_ID: u8 = 63;

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

inventory::submit! {
    ChartTypeEntry {
        group: "statistical",
        id: PIE_ID,
        name: "pie",
        renderer: pie::render_pie_native as crate::plot::controller::chart_controller::ChartRenderer,
        svg_renderer: Some(noop_svg_renderer as SvgChartRenderer),
        color: 0x8B5CF6,
    }
}
inventory::submit! {
    ChartTypeEntry {
        group: "statistical",
        id: HEATMAP_ID,
        name: "heatmap",
        renderer: heatmap::render_heatmap_native as crate::plot::controller::chart_controller::ChartRenderer,
        svg_renderer: Some(noop_svg_renderer as SvgChartRenderer),
        color: 0xEF4444,
    }
}
inventory::submit! {
    ChartTypeEntry {
        group: "statistical",
        id: HISTOGRAM_ID,
        name: "histogram",
        renderer: crate::plot::default::render_bars as crate::plot::controller::chart_controller::ChartRenderer,
        svg_renderer: Some(crate::plot::default::bar::render_svg_bars as SvgChartRenderer),
        color: 0x06B6D4,
    }
}
inventory::submit! {
    ChartTypeEntry {
        group: "statistical",
        id: GROUPED_BAR_ID,
        name: "grouped_bar",
        renderer: crate::plot::default::render_bars as crate::plot::controller::chart_controller::ChartRenderer,
        svg_renderer: Some(crate::plot::default::bar::render_svg_bars as SvgChartRenderer),
        color: 0x22C55E,
    }
}

pub fn register_statistical_types() {
    crate::plot::controller::chart_controller::register_group_from_inventory("statistical");
}

#[cfg(test)]
mod inventory_tests {
    use crate::plot::controller::chart_controller::test_support::*;

    #[test]
    fn statistical_group_is_well_formed() {
        assert_group_well_formed("statistical");
    }

    #[test]
    fn register_statistical_types_matches_inventory() {
        assert_registered_group_matches_inventory("statistical", super::register_statistical_types);
    }
}
