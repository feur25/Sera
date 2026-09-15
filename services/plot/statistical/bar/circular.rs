use super::block3d::Bar3DBlock;
use super::circular_common::render as render_circular;
use super::config::BarConfig;

pub fn layout_3d(cfg: &BarConfig) -> Vec<Bar3DBlock> {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 {
        return Vec::new();
    }
    let r = 3.2;
    (0..n)
        .map(|i| {
            let theta = -std::f64::consts::FRAC_PI_2 + std::f64::consts::TAU * i as f64 / n as f64;
            Bar3DBlock::new(r * theta.cos(), r * theta.sin(), 0.0, cfg.values[i], 0.28, 0.28, i)
        })
        .collect()
}

#[crate::chart_demo(
    "labels=[\"A\",\"B\",\"C\",\"D\",\"E\",\"F\",\"G\",\"H\"], values=[24,38,17,42,29,33,20,15], show_values=True, gridlines=True, variant=\"circular\""
)]

pub fn render(cfg: &BarConfig) -> String {
    render_circular(cfg, cfg.show_text, cfg.gridlines, false)
}
