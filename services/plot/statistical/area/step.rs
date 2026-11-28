use super::common::{render_with, Curve, StackMode};
use super::config::AreaConfig;

#[crate::chart_demo(
    "x_labels=[\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\",\"Sat\",\"Sun\"], series=[[42,42,58,58,58,71,71]], series_names=[\"Active servers\"], variant=\"step\""
)]

pub fn render(cfg: &AreaConfig) -> String {
    render_with(cfg, StackMode::None, Curve::Step, false)
}
