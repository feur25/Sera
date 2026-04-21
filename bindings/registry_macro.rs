#[macro_export]
macro_rules! for_each_json_chart_fn {
    ($mac:ident) => {
        $mac!(build_html_chart,          "buildHtmlChart");
        $mac!(build_bar_chart,           "buildBarChart");
        $mac!(build_hbar,                "buildHbar");
        $mac!(build_line_chart,          "buildLineChart");
        $mac!(build_dbscan_chart,        "buildDbscanChart");
        $mac!(build_dbscan_chart_3d,     "buildDbscanChart3d");
        $mac!(build_kmeans_chart,        "buildKmeansChart");
        $mac!(build_scatter_chart,       "buildScatterChart");
        $mac!(build_histogram,           "buildHistogram");
        $mac!(build_histogram_overlay,   "buildHistogramOverlay");
        $mac!(build_grouped_bar,         "buildGroupedBar");
        $mac!(build_stacked_bar,         "buildStackedBar");
        $mac!(build_heatmap,             "buildHeatmap");
        $mac!(build_pie_chart,           "buildPieChart");
        $mac!(build_donut_chart,         "buildDonutChart");
        $mac!(build_boxplot,             "buildBoxplot");
        $mac!(build_violin,              "buildViolin");
        $mac!(build_slope,               "buildSlope");
        $mac!(build_sunburst,            "buildSunburst");
        $mac!(build_funnel,              "buildFunnel");
        $mac!(build_treemap,             "buildTreemap");
        $mac!(build_multiline_chart,     "buildMultilineChart");
        $mac!(build_area_chart,          "buildAreaChart");
        $mac!(build_waterfall,           "buildWaterfall");
        $mac!(build_bullet,              "buildBullet");
        $mac!(build_bubble_map,          "buildBubbleMap");
        $mac!(build_choropleth,          "buildChoropleth");
        $mac!(build_scatter3d_chart,     "buildScatter3dChart");
        $mac!(build_bar3d_chart,         "buildBar3dChart");
        $mac!(build_line3d_chart,        "buildLine3dChart");
        $mac!(build_radar_chart,         "buildRadarChart");
        $mac!(build_radar3d_chart,       "buildRadar3dChart");
        $mac!(build_lollipop_chart,      "buildLollipopChart");
        $mac!(build_lollipop3d_chart,    "buildLollipop3dChart");
        $mac!(build_kde_chart,           "buildKdeChart");
        $mac!(build_kde3d_chart,         "buildKde3dChart");
        $mac!(build_ridgeline_chart,     "buildRidgelineChart");
        $mac!(build_ridgeline3d_chart,   "buildRidgeline3dChart");
        $mac!(build_bubble3d_chart,      "buildBubble3dChart");
        $mac!(build_pie3d_chart,         "buildPie3dChart");
        $mac!(build_violin3d_chart,      "buildViolin3dChart");
        $mac!(build_heatmap3d_chart,     "buildHeatmap3dChart");
        $mac!(build_candlestick3d_chart, "buildCandlestick3dChart");
        $mac!(build_dumbbell3d_chart,    "buildDumbbell3dChart");
        $mac!(build_funnel3d_chart,      "buildFunnel3dChart");
        $mac!(build_sunburst3d_chart,    "buildSunburst3dChart");
        $mac!(build_stacked_bar3d_chart, "buildStackedBar3dChart");
        $mac!(build_globe3d_chart,       "buildGlobe3dChart");
        $mac!(build_wordcloud,           "buildWordcloud");
        $mac!(build_candlestick,         "buildCandlestick");
        $mac!(build_dumbbell,            "buildDumbbell");
        $mac!(build_bubble,              "buildBubble");
        $mac!(build_gauge,               "buildGauge");
        $mac!(build_parallel,            "buildParallel");
    };
}

#[macro_export]
macro_rules! for_each_chart_fn {
    ($mac:ident) => {
        crate::for_each_json_chart_fn!($mac);
        $mac!(build_grid,                "buildGrid");
        $mac!(build_slideshow,           "buildSlideshow");
    };
}

#[macro_export]
macro_rules! for_each_chart_class {
    ($mac:ident) => {
        $mac!(DbscanModel);
        $mac!(KMeansModel);
    };
}

pub const CHART_ALIASES: &[(&str, &str)] = &[
    ("bar",            "build_bar_chart"),
    ("bar_chart",      "build_bar_chart"),
    ("bars",           "build_bar_chart"),
    ("hbar",           "build_hbar"),
    ("barh",           "build_hbar"),
    ("horizontal_bar", "build_hbar"),
    ("line",           "build_line_chart"),
    ("line_chart",     "build_line_chart"),
    ("scatter",        "build_scatter_chart"),
    ("scatter_chart",  "build_scatter_chart"),
    ("hist",           "build_histogram"),
    ("histogram",      "build_histogram"),
    ("pie",            "build_pie_chart"),
    ("pie_chart",      "build_pie_chart"),
    ("donut",          "build_donut_chart"),
    ("donut_chart",    "build_donut_chart"),
    ("heatmap",        "build_heatmap"),
    ("boxplot",        "build_boxplot"),
    ("box_plot",       "build_boxplot"),
    ("violin",         "build_violin"),
    ("radar",          "build_radar_chart"),
    ("radar_chart",    "build_radar_chart"),
    ("lollipop",       "build_lollipop_chart"),
    ("kde",            "build_kde_chart"),
    ("ridgeline",      "build_ridgeline_chart"),
    ("bubble",         "build_bubble"),
    ("candlestick",    "build_candlestick"),
    ("dumbbell",       "build_dumbbell"),
    ("funnel",         "build_funnel"),
    ("waterfall",      "build_waterfall"),
    ("treemap",        "build_treemap"),
    ("sunburst",       "build_sunburst"),
    ("gauge",          "build_gauge"),
    ("parallel",       "build_parallel"),
    ("grouped_bar",    "build_grouped_bar"),
    ("stacked_bar",    "build_stacked_bar"),
    ("slope",          "build_slope"),
    ("bullet",         "build_bullet"),
    ("area",           "build_area_chart"),
    ("area_chart",     "build_area_chart"),
    ("multiline",      "build_multiline_chart"),
    ("bubble_map",     "build_bubble_map"),
    ("choropleth",     "build_choropleth"),
    ("wordcloud",      "build_wordcloud"),
    ("kmeans",         "build_kmeans_chart"),
    ("kmeans_chart",   "build_kmeans_chart"),
    ("dbscan",         "build_dbscan_chart"),
    ("scatter3d",      "build_scatter3d_chart"),
    ("bar3d",          "build_bar3d_chart"),
    ("line3d",         "build_line3d_chart"),
    ("radar3d",        "build_radar3d_chart"),
    ("lollipop3d",     "build_lollipop3d_chart"),
    ("kde3d",          "build_kde3d_chart"),
    ("ridgeline3d",    "build_ridgeline3d_chart"),
    ("bubble3d",       "build_bubble3d_chart"),
    ("pie3d",          "build_pie3d_chart"),
    ("violin3d",       "build_violin3d_chart"),
    ("heatmap3d",      "build_heatmap3d_chart"),
    ("candlestick3d",  "build_candlestick3d_chart"),
    ("dumbbell3d",     "build_dumbbell3d_chart"),
    ("funnel3d",       "build_funnel3d_chart"),
    ("sunburst3d",     "build_sunburst3d_chart"),
    ("stacked_bar3d",  "build_stacked_bar3d_chart"),
    ("globe3d",        "build_globe3d_chart"),
    ("dbscan3d",       "build_dbscan_chart_3d"),
    ("plot",           "plot_chart"),
    ("grid",           "build_grid"),
    ("save",           "savefig"),
    ("save_fig",       "savefig"),
];

#[macro_export]
macro_rules! for_each_fn {
    ($mac:ident) => {
        $crate::for_each_chart_fn!($mac);
        $mac!(build_hover_json,          "buildHoverJson");
        $mac!(ml_dbscan_fit_predict,     "mlDbscanFitPredict");
        $mac!(ml_kmeans_fit_predict,     "mlKmeansFitPredict");
        $mac!(set_theme,                 "setTheme");
        $mac!(reset_theme,               "resetTheme");
        $mac!(themes,                    "themes");
        $mac!(set_global_background,     "setGlobalBackground");
        $mac!(reset_global_background,   "resetGlobalBackground");
    };
}
