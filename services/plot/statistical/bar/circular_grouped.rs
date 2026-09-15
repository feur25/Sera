use super::block3d::Bar3DBlock;
use super::circular_common::render as render_circular;
use super::config::BarConfig;

pub fn layout_3d(cfg: &BarConfig) -> Vec<Bar3DBlock> {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 {
        return Vec::new();
    }
    let use_groups = cfg.color_groups.len() == n;
    let mut groups: Vec<&str> = Vec::new();
    if use_groups {
        for g in &cfg.color_groups[..n] {
            if !groups.contains(&g.as_str()) {
                groups.push(g.as_str());
            }
        }
    }
    let n_groups = groups.len().max(1);
    let gap = if n_groups > 1 {
        std::f64::consts::TAU * 0.03
    } else {
        0.0
    };
    let usable = std::f64::consts::TAU - gap * n_groups as f64;
    let r = 3.2;
    let mut out = Vec::with_capacity(n);
    let mut cursor = -std::f64::consts::FRAC_PI_2;
    for gi in 0..n_groups {
        let idxs: Vec<usize> = if use_groups {
            (0..n).filter(|&i| cfg.color_groups[i] == groups[gi]).collect()
        } else {
            (0..n).collect()
        };
        let count = idxs.len().max(1);
        let group_angle = usable / n_groups as f64;
        let slot = group_angle / count as f64;
        for (k, &i) in idxs.iter().enumerate() {
            let theta = cursor + slot * (k as f64 + 0.5);
            out.push(Bar3DBlock::new(r * theta.cos(), r * theta.sin(), 0.0, cfg.values[i], 0.24, 0.24, gi));
        }
        cursor += group_angle + gap;
    }
    out
}

#[crate::chart_demo(
    "labels=[\"A1\",\"A2\",\"A3\",\"B1\",\"B2\",\"B3\",\"C1\",\"C2\",\"C3\"], values=[24,38,17,42,29,33,20,15,27], color_groups=[\"Group A\",\"Group A\",\"Group A\",\"Group B\",\"Group B\",\"Group B\",\"Group C\",\"Group C\",\"Group C\"], show_values=True, variant=\"circular_grouped\""
)]

pub fn render(cfg: &BarConfig) -> String {
    render_circular(cfg, cfg.show_text, cfg.gridlines, true)
}
