use super::common::{render_with_style, Curve, StackMode};
use super::config::AreaConfig;

#[crate::chart_demo(
    "x_labels=[\"1\",\"2\",\"3\",\"4\",\"5\",\"6\",\"7\",\"8\",\"9\",\"10\"], series=[[18,22,16,25,30,21,27,24,29,26],[15,19,24,17,20,26,18,23,19,22],[22,17,20,23,15,19,25,18,21,17],[12,15,18,14,17,13,16,19,15,18],[25,21,19,26,22,28,20,24,27,23],[10,13,11,15,12,14,16,11,13,12],[20,18,23,19,25,16,21,22,18,24]], series_names=[\"A\",\"B\",\"C\",\"D\",\"E\",\"F\",\"G\"], variant=\"ribbon\""
)]

pub fn render(cfg: &AreaConfig) -> String {
    render_with_style(cfg, StackMode::Percent, Curve::Linear, false, Some(0x1a1a1a), Some(0.88))
}
