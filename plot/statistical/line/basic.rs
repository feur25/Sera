use super::config::LineConfig;

#[crate::chart_demo("x_labels=[\"Jan\",\"Feb\",\"Mar\",\"Apr\",\"May\",\"Jun\"], values=[12,19,15,22,28,24]")]

pub fn render(cfg: &LineConfig) -> String {
    let color = if cfg.color_hex != 0 { cfg.color_hex } else { 0x6366F1 };
    crate::plot::default::render_lines_html(
        cfg.title, cfg.labels, cfg.values, cfg.width, cfg.height,
        cfg.hover, color, cfg.x_label, cfg.y_label,
        cfg.gridlines, cfg.show_points, cfg.sort_order,
    )
}

