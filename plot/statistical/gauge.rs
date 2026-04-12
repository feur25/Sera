use super::common::{push_b, push_i, push_f2, hex6, PALETTE};
use crate::html::hover::{build_chart_html, slots_to_json};

crate::chart_config!(GaugeConfig, 400, 300;
    struct {
        pub value: f64,
        pub min_val: f64,
        pub max_val: f64,
        pub label: &'a str,
        pub thresholds: &'a [(f64, u32)],
    }
    defaults {
        value: 0.0,
        min_val: 0.0,
        max_val: 100.0,
        label: "",
        thresholds: &[],
    }
);

pub fn render_gauge_html(cfg: &GaugeConfig) -> String {
    let range = (cfg.max_val - cfg.min_val).max(1e-9);
    let frac = ((cfg.value - cfg.min_val) / range).clamp(0.0, 1.0);

    let cx = cfg.width as f64 / 2.0;
    let cy = cfg.height as f64 * 0.65;
    let radius = (cfg.width.min(cfg.height) as f64) * 0.38;
    let arc_w = radius * 0.18;

    // Default thresholds: green (0-60%), yellow (60-80%), red (80-100%)
    let default_thresholds: Vec<(f64, u32)> = if cfg.thresholds.is_empty() {
        vec![
            (0.0, 0x10B981),
            (0.6, 0xF59E0B),
            (0.8, 0xEF4444),
        ]
    } else {
        cfg.thresholds.iter().map(|&(v, c)| ((v - cfg.min_val) / range, c)).collect()
    };

    let mut buf = Vec::<u8>::with_capacity(4096);
    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\" viewBox=\"0 0 ");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b" ");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    // Title
    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"28\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-weight=\"700\" font-size=\"15\" fill=\"#1e293b\">");
        buf.extend_from_slice(cfg.title.as_bytes());
        push_b(&mut buf, b"</text>");
    }

    // Draw arc segments
    let start_angle: f64 = std::f64::consts::PI;
    let end_angle: f64 = 0.0;
    let total_arc = std::f64::consts::PI;

    for i in 0..default_thresholds.len() {
        let f0 = default_thresholds[i].0;
        let f1 = if i + 1 < default_thresholds.len() { default_thresholds[i + 1].0 } else { 1.0 };
        let col = default_thresholds[i].1;
        let a0 = start_angle - f0 * total_arc;
        let a1 = start_angle - f1 * total_arc;
        let hx = hex6(col);

        let x1 = cx + radius * a0.cos();
        let y1 = cy - radius * a0.sin();
        let x2 = cx + radius * a1.cos();
        let y2 = cy - radius * a1.sin();
        let large = if (f1 - f0) > 0.5 { 1 } else { 0 };

        push_b(&mut buf, b"<path d=\"M ");
        push_f2(&mut buf, x1); push_b(&mut buf, b" "); push_f2(&mut buf, y1);
        push_b(&mut buf, b" A "); push_f2(&mut buf, radius); push_b(&mut buf, b" "); push_f2(&mut buf, radius);
        push_b(&mut buf, b" 0 "); push_i(&mut buf, large); push_b(&mut buf, b" 0 ");
        push_f2(&mut buf, x2); push_b(&mut buf, b" "); push_f2(&mut buf, y2);
        push_b(&mut buf, b"\" fill=\"none\" stroke=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke-width=\"");
        push_f2(&mut buf, arc_w);
        push_b(&mut buf, b"\" stroke-linecap=\"round\" opacity=\"0.25\"/>");
    }

    // Active arc (filled portion)
    if frac > 0.001 {
        let a_end = start_angle - frac * total_arc;
        let ax1 = cx + radius * start_angle.cos();
        let ay1 = cy - radius * start_angle.sin();
        let ax2 = cx + radius * a_end.cos();
        let ay2 = cy - radius * a_end.sin();
        let large_a = if frac > 0.5 { 1 } else { 0 };

        // Determine color from thresholds
        let mut active_col = default_thresholds[0].1;
        for &(t, c) in &default_thresholds {
            if frac >= t { active_col = c; }
        }
        let ahx = hex6(active_col);

        push_b(&mut buf, b"<path d=\"M ");
        push_f2(&mut buf, ax1); push_b(&mut buf, b" "); push_f2(&mut buf, ay1);
        push_b(&mut buf, b" A "); push_f2(&mut buf, radius); push_b(&mut buf, b" "); push_f2(&mut buf, radius);
        push_b(&mut buf, b" 0 "); push_i(&mut buf, large_a); push_b(&mut buf, b" 0 ");
        push_f2(&mut buf, ax2); push_b(&mut buf, b" "); push_f2(&mut buf, ay2);
        push_b(&mut buf, b"\" fill=\"none\" stroke=\"#");
        buf.extend_from_slice(&ahx);
        push_b(&mut buf, b"\" stroke-width=\"");
        push_f2(&mut buf, arc_w + 2.0);
        push_b(&mut buf, b"\" stroke-linecap=\"round\"/>");

        // Needle dot
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, ax2);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, ay2);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, arc_w * 0.4);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&ahx);
        push_b(&mut buf, b"\"/>");
    }

    // Needle line
    let needle_angle = start_angle - frac * total_arc;
    let nl = radius * 0.72;
    let nx = cx + nl * needle_angle.cos();
    let ny = cy - nl * needle_angle.sin();
    push_b(&mut buf, b"<line x1=\"");
    push_f2(&mut buf, cx); push_b(&mut buf, b"\" y1=\""); push_f2(&mut buf, cy);
    push_b(&mut buf, b"\" x2=\""); push_f2(&mut buf, nx);
    push_b(&mut buf, b"\" y2=\""); push_f2(&mut buf, ny);
    push_b(&mut buf, b"\" stroke=\"#1e293b\" stroke-width=\"2.5\" stroke-linecap=\"round\"/>");

    // Center circle
    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx); push_b(&mut buf, b"\" cy=\""); push_f2(&mut buf, cy);
    push_b(&mut buf, b"\" r=\"8\" fill=\"#1e293b\"/>");
    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx); push_b(&mut buf, b"\" cy=\""); push_f2(&mut buf, cy);
    push_b(&mut buf, b"\" r=\"4\" fill=\"#fff\"/>");

    // Value text
    push_b(&mut buf, b"<text x=\""); push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" y=\""); push_f2(&mut buf, cy + 30.0);
    push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-weight=\"800\" font-size=\"28\" fill=\"#1e293b\">");
    let val_str = format!("{:.1}", cfg.value);
    buf.extend_from_slice(val_str.as_bytes());
    push_b(&mut buf, b"</text>");

    // Label
    if !cfg.label.is_empty() {
        push_b(&mut buf, b"<text x=\""); push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\""); push_f2(&mut buf, cy + 50.0);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"12\" fill=\"#64748b\">");
        buf.extend_from_slice(cfg.label.as_bytes());
        push_b(&mut buf, b"</text>");
    }

    // Min/Max labels
    push_b(&mut buf, b"<text x=\""); push_f2(&mut buf, cx - radius - 10.0);
    push_b(&mut buf, b"\" y=\""); push_f2(&mut buf, cy + 16.0);
    push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#94a3b8\">");
    let min_str = format!("{:.0}", cfg.min_val);
    buf.extend_from_slice(min_str.as_bytes());
    push_b(&mut buf, b"</text>");

    push_b(&mut buf, b"<text x=\""); push_f2(&mut buf, cx + radius + 10.0);
    push_b(&mut buf, b"\" y=\""); push_f2(&mut buf, cy + 16.0);
    push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#94a3b8\">");
    let max_str = format!("{:.0}", cfg.max_val);
    buf.extend_from_slice(max_str.as_bytes());
    push_b(&mut buf, b"</text>");

    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}
