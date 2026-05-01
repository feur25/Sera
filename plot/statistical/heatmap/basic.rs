use super::common::render_core;
use super::config::HeatmapConfig;

pub fn render(cfg: &HeatmapConfig) -> String {
    render_core(cfg)
}
