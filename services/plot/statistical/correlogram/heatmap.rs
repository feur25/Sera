use super::common::render_impl;
use super::config::CorrelogramConfig;

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\",\"D\"], matrix=[[1,0.8,-0.3,0.5],[0.8,1,0.1,-0.2],[-0.3,0.1,1,0.7],[0.5,-0.2,0.7,1]], variant=\"heatmap\"")]
pub fn render(cfg: &CorrelogramConfig) -> String {
    render_impl(cfg, "square", "", "full", false)
}
