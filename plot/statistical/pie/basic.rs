use super::common::render_with;
use super::config::PieConfig;

#[crate::chart_demo("labels=[\"Apple\",\"Banana\",\"Cherry\",\"Date\",\"Fig\"], values=[40,25,20,10,5]")]

pub fn render(cfg: &PieConfig) -> String {
    render_with(cfg, cfg.pull, |_, _| {})
}

