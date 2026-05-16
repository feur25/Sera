use super::common::{clone_cfg, render_core};
use super::config::HeatmapConfig;

pub const DEMO_KWARGS: &str = "labels=[\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\"], col_labels=[\"8h\",\"12h\",\"16h\",\"20h\"], values=[5,9,7,3,6,12,10,4,8,15,13,7,4,8,11,5,3,7,9,2]";
pub fn render(cfg: &HeatmapConfig) -> String {
    let levels = if cfg.contour_levels == 0 { 5 } else { cfg.contour_levels };
    let c = HeatmapConfig {
        smooth: true,
        contour: true,
        contour_levels: levels,
        show_values: false,
        ..clone_cfg(cfg)
    };
    render_core(&c)
}


