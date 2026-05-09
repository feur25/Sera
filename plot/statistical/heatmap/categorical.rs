use super::common::{clone_cfg, render_core};
use super::config::HeatmapConfig;
use crate::plot::statistical::common::PALETTE;

pub fn render(cfg: &HeatmapConfig) -> String {
    let pal: &[u32] = if cfg.palette.is_empty() { PALETTE } else { cfg.palette };
    let c = HeatmapConfig {
        categorical: true,
        palette: pal,
        show_values: cfg.show_values,
        ..clone_cfg(cfg)
    };
    render_core(&c)
}


