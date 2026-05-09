use super::common::render_with;
use super::config::PieConfig;
use std::f64::consts::PI;

pub fn render(cfg: &PieConfig) -> String {
    render_with(cfg, cfg.pull, |p, _| {
        p.arc_start = PI;
        p.arc_span = PI;
        if p.donut <= 0.0 { p.donut = 0.45; }
    })
}


