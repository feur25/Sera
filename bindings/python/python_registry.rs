#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use crate::Chart;

#[cfg(feature = "python")]
use super::python::*;

#[cfg(feature = "python")]
const CHART_ALIASES: &[(&str, &str)] = &[
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
    ("kmeans",          "build_kmeans_chart"),
    ("kmeans_chart",    "build_kmeans_chart"),
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
    ("plot",           "plot"),
    ("grid",           "build_grid"),
    ("save",           "savefig"),
    ("save_fig",       "savefig"),
];

#[cfg(feature = "python")]
pub fn register_submodules(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(set_bg_fn, m)?)?;
    m.add_function(wrap_pyfunction!(show_chart_value, m)?)?;
    m.add_function(wrap_pyfunction!(bench_chart_value, m)?)?;
    m.add_function(wrap_pyfunction!(set_chart_kind, m)?)?;
    m.add_function(wrap_pyfunction!(set_chart_orientation, m)?)?;
    m.add_function(wrap_pyfunction!(bench_pure_rust, m)?)?;
    m.add_function(wrap_pyfunction!(build_html_chart, m)?)?;

    m.add_function(wrap_pyfunction!(build_bar_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_hbar, m)?)?;
    m.add_function(wrap_pyfunction!(build_line_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_scatter_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_dbscan_chart, m)?)?;;
    m.add_class::<DbscanModel>()?;
    m.add_function(wrap_pyfunction!(build_dbscan_chart_3d, m)?)?;
    m.add_function(wrap_pyfunction!(build_kmeans_chart, m)?)?;
    m.add_class::<KMeansModel>()?;

    m.add_function(wrap_pyfunction!(build_histogram, m)?)?;
    m.add_function(wrap_pyfunction!(build_histogram_overlay, m)?)?;

    m.add_function(wrap_pyfunction!(build_grouped_bar, m)?)?;
    m.add_function(wrap_pyfunction!(build_stacked_bar, m)?)?;

    m.add_function(wrap_pyfunction!(build_heatmap, m)?)?;
    m.add_function(wrap_pyfunction!(build_pie_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_donut_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_boxplot, m)?)?;
    m.add_function(wrap_pyfunction!(build_violin, m)?)?;
    m.add_function(wrap_pyfunction!(build_slope, m)?)?;
    m.add_function(wrap_pyfunction!(build_sunburst, m)?)?;
    m.add_function(wrap_pyfunction!(build_funnel, m)?)?;
    m.add_function(wrap_pyfunction!(build_treemap, m)?)?;
    m.add_function(wrap_pyfunction!(build_multiline_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_area_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_waterfall, m)?)?;
    m.add_function(wrap_pyfunction!(build_bullet, m)?)?;

    m.add_function(wrap_pyfunction!(build_bubble_map, m)?)?;
    m.add_function(wrap_pyfunction!(build_choropleth, m)?)?;

    m.add_function(wrap_pyfunction!(build_scatter3d_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_bar3d_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_line3d_chart, m)?)?;
    
    m.add_function(wrap_pyfunction!(build_radar_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_radar3d_chart, m)?)?;

    m.add_function(wrap_pyfunction!(build_lollipop_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_lollipop3d_chart, m)?)?;

    m.add_function(wrap_pyfunction!(build_kde_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_kde3d_chart, m)?)?;

    m.add_function(wrap_pyfunction!(build_ridgeline_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_ridgeline3d_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_bubble3d_chart, m)?)?;

    m.add_function(wrap_pyfunction!(build_pie3d_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_violin3d_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_heatmap3d_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_candlestick3d_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_dumbbell3d_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_funnel3d_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_sunburst3d_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_stacked_bar3d_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_globe3d_chart, m)?)?;

    m.add_function(wrap_pyfunction!(build_wordcloud, m)?)?;
    m.add_function(wrap_pyfunction!(build_candlestick, m)?)?;
    m.add_function(wrap_pyfunction!(build_dumbbell, m)?)?;
    m.add_function(wrap_pyfunction!(build_bubble, m)?)?;
    m.add_function(wrap_pyfunction!(build_gauge, m)?)?;
    m.add_function(wrap_pyfunction!(build_parallel, m)?)?;
    m.add_function(wrap_pyfunction!(build_grid, m)?)?;
    m.add_function(wrap_pyfunction!(build_slideshow, m)?)?;
    
    m.add_function(wrap_pyfunction!(build_hover_json, m)?)?;
    m.add_function(wrap_pyfunction!(plot_chart, m)?)?;
    m.add_function(wrap_pyfunction!(savefig, m)?)?;

    for (alias, canonical) in CHART_ALIASES {
        if let Ok(func) = m.getattr(*canonical) {
            let _ = m.add(*alias, func);
        }
    }

    super::python_ml::register_ml_classes(m)?;

    Ok(())
}
