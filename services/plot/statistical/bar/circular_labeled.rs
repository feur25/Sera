use super::circular_common::render as render_circular;
use super::config::BarConfig;

#[crate::chart_demo(
    "labels=[\"Rust\",\"Python\",\"Wasm\",\"Plot\",\"Data\",\"Viz\",\"Chart\",\"Fast\"], values=[24,38,17,42,29,33,20,15], variant=\"circular_labeled\""
)]

pub fn render(cfg: &BarConfig) -> String {
    render_circular(cfg, true, false, false)
}
