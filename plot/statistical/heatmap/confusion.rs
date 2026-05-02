use super::common::{clone_cfg, render_core};
use super::config::HeatmapConfig;

pub fn render(cfg: &HeatmapConfig) -> String {
    let c = HeatmapConfig {
        confusion_mode: true,
        annotate: true,
        show_values: true,
        smooth: true,
        colorscale: if cfg.colorscale.is_empty() { "blues" } else { cfg.colorscale },
        ..clone_cfg(cfg)
    };
    render_core(&c)
}
