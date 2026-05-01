use super::common::render_with;
use super::config::PieConfig;

pub fn render(cfg: &PieConfig) -> String {
    render_with(cfg, cfg.pull, |p, _| {
        if p.donut <= 0.0 { p.donut = 0.55; }
    })
}
