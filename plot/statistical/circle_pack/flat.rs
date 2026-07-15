use super::config::CirclePackConfig;
use crate::html::hover::{html_id, html_prefix, html_suffix, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i};

#[crate::chart_demo("labels=[\"Alpha\",\"Beta\",\"Gamma\",\"Delta\",\"Epsilon\"], values=[40,30,25,20,15]")]
pub fn render(cfg: &CirclePackConfig) -> String {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 { return String::new(); }

    let total: f64 = cfg.values[..n].iter().sum();
    let max_r = (cfg.width.min(cfg.height) as f64 / 2.0 - 20.0).max(10.0);

    let radii: Vec<f64> = cfg.values[..n].iter()
        .map(|&v| (v / total.max(1.0)).sqrt() * max_r)
        .collect();

    let cx = cfg.width as f64 / 2.0;
    let cy = cfg.height as f64 / 2.0;

    let positions: Vec<(f64, f64)> = (0..n).map(|i| {
        let angle = 2.0 * std::f64::consts::PI * i as f64 / n as f64;
        let orbit = if n == 1 { 0.0 } else {
            max_r * 0.55
        };
        (cx + orbit * angle.cos(), cy + orbit * angle.sin())
    }).collect();

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n * 200 + 4096);
    html_prefix(&mut buf, cfg.title, hid);
    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| radii[b].partial_cmp(&radii[a]).unwrap_or(std::cmp::Ordering::Equal));

    for &i in &order {
        let color = palette_color(cfg.palette, i);
        let hx = hex6(color);
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, positions[i].0);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, positions[i].1);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, radii[i].max(4.0));
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.75\" stroke=\"#fff\" stroke-width=\"1.5\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\"/>");
        if cfg.show_labels && radii[i] > 10.0 {
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, positions[i].0);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, positions[i].1 + 4.0);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" fill=\"#fff\" font-size=\"");
            push_f2(&mut buf, (radii[i] * 0.35).min(13.0).max(8.0));
            push_b(&mut buf, b"\">");
            escape_xml(&mut buf, &cfg.labels[i]);
            push_b(&mut buf, b"</text>");
        }
    }
    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}
