use super::common::{clone_cfg, render_core};
use super::config::HeatmapConfig;

pub fn render(cfg: &HeatmapConfig) -> String {
    let steps = if cfg.discrete_steps == 0 { 5 } else { cfg.discrete_steps };
    let c = HeatmapConfig {
        discrete_steps: steps,
        ..clone_cfg(cfg)
    };
    render_core(&c)
}


