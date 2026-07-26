use super::circular_common::render as render_circular;
use super::config::BarConfig;

#[crate::chart_demo(
    "labels=[\"A\",\"B\",\"C\",\"D\",\"E\",\"F\",\"G\",\"H\"], values=[24,38,17,42,29,33,20,15], variant=\"circular\""
)]

pub fn render(cfg: &BarConfig) -> String {
    render_circular(cfg, false, false, false)
}
