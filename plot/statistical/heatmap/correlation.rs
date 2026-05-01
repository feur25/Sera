use super::common::{clone_cfg, render_core};
use super::config::HeatmapConfig;

pub fn render(cfg: &HeatmapConfig) -> String {
    let c = HeatmapConfig {
        diverging: true,
        show_values: true,
        value_min_cell: 22,
        ..clone_cfg(cfg)
    };
    render_core(&c)
}
