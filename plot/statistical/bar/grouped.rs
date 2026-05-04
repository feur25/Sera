use super::config::BarConfig;
use crate::plot::statistical::{GroupedBarConfig, render_grouped_bar_html};

pub fn render(cfg: &BarConfig, stacked: bool) -> String {
    let g = GroupedBarConfig {
        title: cfg.title, x_label: cfg.x_label, y_label: cfg.y_label,
        gridlines: cfg.gridlines, sort_order: cfg.sort_order,
        hover: cfg.hover, legend_position: cfg.legend_position,
        width: cfg.width, height: cfg.height,
        category_labels: cfg.category_labels, series: cfg.series,
        palette: cfg.palette, stacked,
        show_values: cfg.show_text, value_min_height: 16,
        orientation: cfg.orientation,
    };
    render_grouped_bar_html(&g)
}
