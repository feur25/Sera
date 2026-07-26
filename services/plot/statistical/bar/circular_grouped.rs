use super::circular_common::render as render_circular;
use super::config::BarConfig;

#[crate::chart_demo(
    "labels=[\"A1\",\"A2\",\"A3\",\"B1\",\"B2\",\"B3\",\"C1\",\"C2\",\"C3\"], values=[24,38,17,42,29,33,20,15,27], color_groups=[\"Group A\",\"Group A\",\"Group A\",\"Group B\",\"Group B\",\"Group B\",\"Group C\",\"Group C\",\"Group C\"], show_values=True, variant=\"circular_grouped\""
)]

pub fn render(cfg: &BarConfig) -> String {
    render_circular(cfg, cfg.show_text, cfg.gridlines, true)
}
