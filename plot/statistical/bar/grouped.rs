use super::config::BarConfig;
use crate::plot::statistical::{render_grouped_bar_html, GroupedBarConfig};

#[crate::chart_demo("labels=[\"Q1\",\"Q2\",\"Q3\",\"Q4\"], series=[[24,38,17,42],[18,29,33,21],[12,15,28,30]], series_names=[\"Product A\",\"Product B\",\"Product C\"]")]

pub fn render(cfg: &BarConfig, stacked: bool) -> String {
    let g = GroupedBarConfig {
        title: cfg.title,
        x_label: cfg.x_label,
        y_label: cfg.y_label,
        gridlines: cfg.gridlines,
        sort_order: cfg.sort_order,
        hover: cfg.hover,
        legend_position: cfg.legend_position,
        width: cfg.width,
        height: cfg.height,
        category_labels: cfg.category_labels,
        series: cfg.series,
        palette: cfg.palette,
        stacked,
        show_values: cfg.show_text,
        value_min_height: 16,
        orientation: cfg.orientation,
    };
    render_grouped_bar_html(&g)
}
