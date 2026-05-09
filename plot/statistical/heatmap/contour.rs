use super::common::{clone_cfg, render_core};
use super::config::HeatmapConfig;

pub fn render(cfg: &HeatmapConfig) -> String {
    let levels = if cfg.contour_levels == 0 { 5 } else { cfg.contour_levels };
    let c = HeatmapConfig {
        smooth: true,
        contour: true,
        contour_levels: levels,
        show_values: false,
        ..clone_cfg(cfg)
    };
    render_core(&c)
}


