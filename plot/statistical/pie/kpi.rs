use super::common::render_with;
use super::config::PieConfig;

pub fn render(cfg: &PieConfig) -> String {
    let total: f64 = cfg.values.iter().filter(|v| v.is_finite() && **v >= 0.0).sum();
    let auto_text = if cfg.center_text.is_empty() { format_total(total) } else { cfg.center_text.to_string() };
    let auto_sub = if cfg.center_subtext.is_empty() { "TOTAL".to_string() } else { cfg.center_subtext.to_string() };
    render_with(cfg, cfg.pull, move |p, _| {
        if p.donut <= 0.0 { p.donut = 0.62; }
        p.center_text = auto_text;
        p.center_subtext = auto_sub;
    })
}

fn format_total(v: f64) -> String {
    let abs = v.abs();
    if abs >= 1.0e9 { format!("{:.1}B", v / 1.0e9) }
    else if abs >= 1.0e6 { format!("{:.1}M", v / 1.0e6) }
    else if abs >= 1.0e3 { format!("{:.1}K", v / 1.0e3) }
    else if abs >= 100.0 { format!("{:.0}", v) }
    else { format!("{:.1}", v) }
}


