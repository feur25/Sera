use super::common::render_with;
use super::config::PieConfig;

pub fn render(cfg: &PieConfig) -> String {
    render_with(cfg, cfg.pull, |p, _| {
        if p.pattern.is_empty() { p.pattern = "stripes".to_string(); }
    })
}
