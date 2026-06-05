use super::config::BoxplotConfig;

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\"], series=[[1.2,2.4,2.7,3.1,3.5,3.8,4.2,5.1,6.0],[2.0,2.8,3.2,3.6,4.1,4.5,5.0,5.7,6.5],[1.8,2.2,2.6,3.0,3.4,3.9,4.3,4.9,5.5]]")]

pub fn render(cfg: &BoxplotConfig) -> String {
    super::basic::render_with(cfg, cfg.notch, true, false)
}
