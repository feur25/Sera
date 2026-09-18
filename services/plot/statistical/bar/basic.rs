use super::block3d::Bar3DBlock;
use super::config::BarConfig;

fn padded_values(cfg: &BarConfig) -> Vec<f64> {
    let n = cfg.values.len().max(cfg.labels.len());
    (0..n).map(|i| cfg.values.get(i).copied().unwrap_or(0.0)).collect()
}

pub fn layout_3d(cfg: &BarConfig) -> Vec<Bar3DBlock> {
    crate::plot::statistical::_3d::generic::linear_columns(&padded_values(cfg), 0.35, 0.35)
}

pub fn layout_3d_horizontal(cfg: &BarConfig) -> Vec<Bar3DBlock> {
    crate::plot::statistical::_3d::generic::linear_columns_horizontal(&padded_values(cfg), 0.35, 0.7)
}

#[crate::chart_demo(
    "labels=[\"Alpha\",\"Beta\",\"Gamma\",\"Delta\",\"Epsilon\"], values=[24,38,17,42,29]"
)]
pub fn render(cfg: &BarConfig, orient: u8) -> String {
    crate::plot::default::render_bars_html(
        cfg.title,
        cfg.labels,
        cfg.values,
        cfg.width,
        cfg.height,
        cfg.hover,
        orient,
        cfg.color_groups,
        cfg.show_text,
        cfg.x_label,
        cfg.y_label,
        cfg.palette,
        cfg.color_hex,
        cfg.gridlines,
        cfg.sort_order,
        cfg.legend_position,
    )
}
