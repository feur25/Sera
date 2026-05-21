use super::common::{clone_cfg, render_core};
use super::config::HeatmapConfig;

#[crate::chart_demo("labels=[\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\"], col_labels=[\"8h\",\"12h\",\"16h\",\"20h\"], values=[5,9,7,3,6,12,10,4,8,15,13,7,4,8,11,5,3,7,9,2]")]

pub fn render(cfg: &HeatmapConfig) -> String {
    let n_rows = cfg.row_labels.len();
    let n_cols = if cfg.col_labels.is_empty() { n_rows } else { cfg.col_labels.len() };
    let xw_default: Vec<f64> = if cfg.x_widths.is_empty() && n_cols > 0 {
        (0..n_cols).map(|i| 1.0 + (i as f64 * 1.31).sin().abs() * 1.6).collect()
    } else { Vec::new() };
    let yh_default: Vec<f64> = if cfg.y_heights.is_empty() && n_rows > 0 {
        (0..n_rows).map(|i| 1.0 + (i as f64 * 0.83).cos().abs() * 1.4).collect()
    } else { Vec::new() };
    let xw: &[f64] = if cfg.x_widths.is_empty() { &xw_default[..] } else { cfg.x_widths };
    let yh: &[f64] = if cfg.y_heights.is_empty() { &yh_default[..] } else { cfg.y_heights };
    let c = HeatmapConfig {
        x_widths: xw,
        y_heights: yh,
        ..clone_cfg(cfg)
    };
    render_core(&c)
}

