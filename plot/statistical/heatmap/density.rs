use super::common::{clone_cfg, render_core};
use super::config::HeatmapConfig;

pub fn render(cfg: &HeatmapConfig) -> String {
    let c = HeatmapConfig {
        smooth: true,
        viridis: true,
        show_values: false,
        ..clone_cfg(cfg)
    };
    render_core(&c)
}
