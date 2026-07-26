use super::common::{render_with, Curve, StackMode};
use super::config::AreaConfig;

#[crate::chart_demo(
    "x_labels=[\"Jan\",\"Feb\",\"Mar\",\"Apr\",\"May\",\"Jun\",\"Jul\",\"Aug\"], series=[[8200,7100,9400,12300,15800,19200,21500,20100]], series_names=[\"Revenue\"], variant=\"gradient\""
)]

pub fn render(cfg: &AreaConfig) -> String {
    render_with(cfg, StackMode::None, Curve::Spline, true)
}
