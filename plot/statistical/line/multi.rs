use super::config::LineConfig;
use crate::plot::statistical::{MultiLineConfig, render_multiline_html};

pub fn render(cfg: &LineConfig) -> String {
    let mc = MultiLineConfig {
        title: cfg.title,
        x_label: cfg.x_label, y_label: cfg.y_label,
        width: cfg.width, height: cfg.height,
        gridlines: cfg.gridlines, sort_order: cfg.sort_order,
        hover: cfg.hover, legend_position: cfg.legend_position,
        x_labels: cfg.x_labels, series: cfg.series,
        palette: cfg.palette, show_points: cfg.show_points,
    };
    render_multiline_html(&mc)
}


