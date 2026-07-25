use super::common::render_impl;
use super::config::DendrogramConfig;

#[crate::chart_demo("labels=[\"A1\",\"A2\",\"A3\",\"B1\",\"B2\",\"B3\",\"C1\",\"C2\",\"C3\"], values=[[1,1],[1.2,0.9],[0.9,1.1],[5,5],[5.2,4.8],[4.9,5.1],[1,5],[1.1,4.9],[0.9,5.2]], variant=\"horizontal\"")]
pub fn render(cfg: &DendrogramConfig) -> String {
    render_impl(cfg, true, false)
}
