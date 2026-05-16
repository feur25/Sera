use super::common::render_with;
use super::config::PieConfig;
use std::f64::consts::PI;

pub const DEMO_KWARGS: &str = "labels=[\"Apple\",\"Banana\",\"Cherry\",\"Date\",\"Fig\"], values=[40,25,20,10,5]";
pub fn render(cfg: &PieConfig) -> String {
    render_with(cfg, cfg.pull, |p, _| {
        p.arc_start = PI;
        p.arc_span = PI;
        if p.donut <= 0.0 { p.donut = 0.45; }
    })
}


