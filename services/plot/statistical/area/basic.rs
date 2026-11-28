use super::common::{render_with, Curve, StackMode};
use super::config::AreaConfig;

#[crate::chart_demo(
    "x_labels=[\"Q1\",\"Q2\",\"Q3\",\"Q4\"], series=[[11800,11500,12300,12800],[10500,10900,11100,11400],[10700,10800,10500,11300]], series_names=[\"North\",\"South\",\"East\"]"
)]

pub fn render(cfg: &AreaConfig) -> String {
    render_with(cfg, StackMode::None, Curve::Linear, false)
}
