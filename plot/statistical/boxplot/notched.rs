use super::config::BoxplotConfig;

pub fn render(cfg: &BoxplotConfig) -> String {
    super::basic::render_with(cfg, true, cfg.show_points, false)
}


