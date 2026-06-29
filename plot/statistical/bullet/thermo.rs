use super::common::{max_for, prepare};
use super::config::BulletConfig;
use crate::html::hover::{build_chart_html, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i, truncate};

#[crate::chart_demo("labels=[\"Revenue\",\"Profit\",\"CSAT\"], values=[80,65,4.2], targets=[90,70,4.5], max_vals=[120,100,5]")]

pub fn render(cfg: &BulletConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let pad_t = if cfg.title.is_empty() { 26 } else { 50 };
    let pad_b = 44;
    let pad_l = 36;
    let pad_r = 24;
    let auto_w = if cfg.width < p.n as i32 * 90 + pad_l + pad_r {
        p.n as i32 * 90 + pad_l + pad_r
    } else {
        cfg.width
    };
    let plot_h = cfg.height - pad_t - pad_b;
    let mut b = Vec::<u8>::with_capacity(p.n * 360 + 1024);
    push_b(
        &mut b,
        b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"",
    );
    push_i(&mut b, auto_w);
    push_b(&mut b, b"\" height=\"");
    push_i(&mut b, cfg.height);
    push_b(&mut b, b"\" viewBox=\"0 0 ");
    push_i(&mut b, auto_w);
    push_b(&mut b, b" ");
    push_i(&mut b, cfg.height);
    push_b(&mut b, b"\">");
    push_b(
        &mut b,
        b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>",
    );
    if !cfg.title.is_empty() {
        push_b(&mut b, b"<text x=\"");
        push_i(&mut b, auto_w / 2);
        push_b(&mut b, b"\" y=\"26\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(&mut b, cfg.title);
        push_b(&mut b, b"</text>");
    }
    let col_w = (auto_w - pad_l - pad_r) / p.n as i32;
    let tube_w = (col_w as f64 * 0.34) as i32;
    let bulb_r = (tube_w / 2 + 6).max(10);
    let hx_track = hex6(0xE5E7EB);
    let hx_value = hex6(0x636EFA);
    for i in 0..p.n {
        let max_v = max_for(&p, i);
        let cx = pad_l + i as i32 * col_w + col_w / 2;
        let ty = pad_t;
        let by = pad_t + plot_h - bulb_r;
        let tube_h = by - ty;
        push_b(&mut b, b"<rect x=\"");
        push_i(&mut b, cx - tube_w / 2);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, ty);
        push_b(&mut b, b"\" width=\"");
        push_i(&mut b, tube_w);
        push_b(&mut b, b"\" height=\"");
        push_i(&mut b, tube_h);
        push_b(&mut b, b"\" fill=\"#");
        b.extend_from_slice(&hx_track);
        push_b(&mut b, b"\" rx=\"");
        push_i(&mut b, tube_w / 2);
        push_b(&mut b, b"\"/>");
        let frac = (p.values[i] / max_v).min(1.0).max(0.0);
        let fill_h = (tube_h as f64 * frac) as i32;
        let fy = by - fill_h;
        push_b(&mut b, b"<rect data-idx=\"");
        push_i(&mut b, i as i32);
        push_b(&mut b, b"\" data-y=\"");
        push_f2(&mut b, p.values[i]);
        push_b(&mut b, b"\" data-lbl=\"");
        escape_xml(&mut b, &p.labels[i]);
        push_b(&mut b, b"\" x=\"");
        push_i(&mut b, cx - tube_w / 2);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, fy);
        push_b(&mut b, b"\" width=\"");
        push_i(&mut b, tube_w);
        push_b(&mut b, b"\" height=\"");
        push_i(&mut b, fill_h);
        push_b(&mut b, b"\" fill=\"#");
        b.extend_from_slice(&hx_value);
        push_b(&mut b, b"\" rx=\"");
        push_i(&mut b, tube_w / 2);
        push_b(&mut b, b"\"/>");
        push_b(&mut b, b"<circle cx=\"");
        push_i(&mut b, cx);
        push_b(&mut b, b"\" cy=\"");
        push_i(&mut b, by + bulb_r - 2);
        push_b(&mut b, b"\" r=\"");
        push_i(&mut b, bulb_r);
        push_b(&mut b, b"\" fill=\"#");
        b.extend_from_slice(&hx_value);
        push_b(&mut b, b"\"/>");
        if p.targets[i] > 0.0 {
            let yt = by - ((p.targets[i] / max_v).min(1.0) * tube_h as f64) as i32;
            push_b(&mut b, b"<line x1=\"");
            push_i(&mut b, cx - tube_w);
            push_b(&mut b, b"\" y1=\"");
            push_i(&mut b, yt);
            push_b(&mut b, b"\" x2=\"");
            push_i(&mut b, cx + tube_w);
            push_b(&mut b, b"\" y2=\"");
            push_i(&mut b, yt);
            push_b(&mut b, b"\" stroke=\"#1a202c\" stroke-width=\"2.2\"/>");
        }
        push_b(&mut b, b"<text x=\"");
        push_i(&mut b, cx);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, cfg.height - pad_b + 18);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">");
        escape_xml(&mut b, truncate(&p.labels[i], 12));
        push_b(&mut b, b"</text>");
    }
    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}
