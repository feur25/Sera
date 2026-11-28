use super::block3d::Bar3DBlock;
use super::circular_common::render as render_circular;
use super::config::BarConfig;

pub fn layout_3d(cfg: &BarConfig) -> Vec<Bar3DBlock> {
    let n = cfg.labels.len().min(cfg.values.len());
    crate::plot::statistical::_3d::generic::radial_grouped_columns(&cfg.values[..n], cfg.color_groups, 3.2, 0.24, 0.24)
}

#[crate::chart_demo(
    "labels=[\"A1\",\"A2\",\"A3\",\"B1\",\"B2\",\"B3\",\"C1\",\"C2\",\"C3\"], values=[24,38,17,42,29,33,20,15,27], color_groups=[\"Group A\",\"Group A\",\"Group A\",\"Group B\",\"Group B\",\"Group B\",\"Group C\",\"Group C\",\"Group C\"], show_values=True, variant=\"circular_grouped\""
)]

pub fn render(cfg: &BarConfig) -> String {
    render_circular(cfg, cfg.show_text, cfg.gridlines, true)
}
