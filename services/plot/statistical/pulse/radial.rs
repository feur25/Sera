use super::common::render_impl;
use super::config::PulseConfig;

#[crate::chart_demo("labels=[\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\",\"Sat\",\"Sun\"], values=[0.4,0.7,0.9,0.6,0.8,0.3,0.5]")]
pub fn render(cfg: &PulseConfig) -> String {
    render_impl(cfg, false, false)
}
