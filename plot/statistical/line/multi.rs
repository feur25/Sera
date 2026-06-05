use super::config::LineConfig;
use crate::plot::statistical::{render_multiline_html, MultiLineConfig};

#[crate::chart_demo("x_labels=[\"Jan\",\"Feb\",\"Mar\",\"Apr\",\"May\",\"Jun\"], series=[[12,19,15,22,28,24],[8,14,18,16,22,20],[5,9,12,15,18,21]], series_names=[\"A\",\"B\",\"C\"]")]

pub fn render(cfg: &LineConfig) -> String {
    let mc = MultiLineConfig {
        title: cfg.title,
        x_label: cfg.x_label,
        y_label: cfg.y_label,
        width: cfg.width,
        height: cfg.height,
        gridlines: cfg.gridlines,
        sort_order: cfg.sort_order,
        hover: cfg.hover,
        legend_position: cfg.legend_position,
        x_labels: cfg.x_labels,
        series: cfg.series,
        palette: cfg.palette,
        show_points: cfg.show_points,
    };
    render_multiline_html(&mc)
}
