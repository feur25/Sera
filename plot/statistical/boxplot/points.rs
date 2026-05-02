use super::config::BoxplotConfig;

pub fn render(cfg: &BoxplotConfig) -> String {
    super::basic::render_with(cfg, cfg.notch, true, false)
}
