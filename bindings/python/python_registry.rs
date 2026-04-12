#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use crate::Chart;

#[cfg(feature = "python")]
use super::python::*;

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
    
    Ok(())
}
