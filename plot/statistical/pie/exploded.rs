use super::common::render_with;
use super::config::PieConfig;

pub const DEMO_KWARGS: &str = "labels=[\"Apple\",\"Banana\",\"Cherry\",\"Date\",\"Fig\"], values=[40,25,20,10,5]";
pub fn render(cfg: &PieConfig) -> String {
    if !cfg.pull.is_empty() {
        return render_with(cfg, cfg.pull, |_, _| {});
    }
    let n = cfg.values.len();
    let mut auto = vec![0.0_f64; n];
    let mut best: Option<(usize, f64)> = None;
    for (i, &v) in cfg.values.iter().enumerate() {
        match best {
            None => best = Some((i, v)),
            Some((_, mv)) if v > mv => best = Some((i, v)),
            _ => {}
        }
    }
    if let Some((idx, _)) = best {
        if idx < n { auto[idx] = 0.18; }
    }
    render_with(cfg, &auto, |_, _| {})
}


