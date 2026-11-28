use super::config::BarConfig;

#[crate::chart_demo(
    "labels=[\"Q1\",\"Q2\",\"Q3\",\"Q4\"], series=[[5,30,60,10],[8,45,70,15],[4,25,50,8]], series_names=[\"Product A\",\"Product B\",\"Product C\"], variant=\"stacked\""
)]

pub fn render(cfg: &BarConfig) -> String {
    super::grouped::render(cfg, true)
}
