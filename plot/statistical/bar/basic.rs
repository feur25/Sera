use super::config::BarConfig;

#[crate::chart_demo("labels=[\"Alpha\",\"Beta\",\"Gamma\",\"Delta\",\"Epsilon\"], values=[24,38,17,42,29]")]
#[crate::params(paramsList = ["title", "labels", "values", "width", "height", "hover", "color_groups", "show_text", "x_label", "y_label", "palette", "color_hex", "gridlines", "sort_order"])]
pub fn render(cfg: &BarConfig, orient: u8) -> String {
    crate::plot::default::render_bars_html(
        cfg.title, cfg.labels, cfg.values, cfg.width, cfg.height,
        cfg.hover, orient, cfg.color_groups, cfg.show_text,
        cfg.x_label, cfg.y_label, cfg.palette, cfg.color_hex,
        cfg.gridlines, cfg.sort_order,
    )
}


