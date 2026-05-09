use super::config::BarConfig;

pub fn render(cfg: &BarConfig, orient: u8) -> String {
    crate::plot::default::render_bars_html(
        cfg.title, cfg.labels, cfg.values, cfg.width, cfg.height,
        cfg.hover, orient, cfg.color_groups, cfg.show_text,
        cfg.x_label, cfg.y_label, cfg.palette, cfg.color_hex,
        cfg.gridlines, cfg.sort_order,
    )
}


