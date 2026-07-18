use super::block3d::Bar3DBlock;
use super::config::BarConfig;

pub fn layout_3d(cfg: &BarConfig) -> Vec<Bar3DBlock> {
    let n = cfg.values.len().max(cfg.labels.len());
    (0..n)
        .map(|i| {
            let v = cfg.values.get(i).copied().unwrap_or(0.0);
            Bar3DBlock::new(i as f64, 0.0, 0.0, v, 0.35, 0.35, i)
        })
        .collect()
}

pub fn layout_3d_horizontal(cfg: &BarConfig) -> Vec<Bar3DBlock> {
    let n = cfg.values.len().max(cfg.labels.len());
    (0..n)
        .map(|i| {
            let v = cfg.values.get(i).copied().unwrap_or(0.0);
            Bar3DBlock::new(v / 2.0, i as f64, 0.0, 0.22, v.abs() / 2.0, 0.35, i)
        })
        .collect()
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
