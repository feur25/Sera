use super::common::render_core;
use super::config::HeatmapConfig;

#[crate::chart_demo("labels=[\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\"], col_labels=[\"8h\",\"12h\",\"16h\",\"20h\"], values=[5,9,7,3,6,12,10,4,8,15,13,7,4,8,11,5,3,7,9,2]")]

pub fn render(cfg: &HeatmapConfig) -> String {
    render_core(cfg)
}

