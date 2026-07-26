use super::circular_common::render as render_circular;
use super::config::BarConfig;

#[crate::chart_demo(
    "labels=[\"A\",\"B\",\"C\",\"D\",\"E\",\"F\",\"G\",\"H\",\"I\",\"J\"], values=[24,38,17,42,29,33,20,15,27,31], variant=\"circular_grid\""
)]

pub fn render(cfg: &BarConfig) -> String {
    render_circular(cfg, true, true, false)
}
