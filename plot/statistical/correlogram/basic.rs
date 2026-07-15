use super::config::CorrelogramConfig;
use crate::html::hover::{html_id, html_prefix, html_suffix, slots_to_json};
use crate::plot::statistical::common::{escape_xml, push_b, push_f2, push_i};

fn corr_color(v: f64) -> ([u8; 6], [u8; 6]) {
    let t = v.clamp(-1.0, 1.0);
    let (r, g, b): (u8, u8, u8) = if t >= 0.0 {
        let u = (t * 255.0) as u8;
        (u, (u as f64 * 0.35) as u8, (u as f64 * 0.35) as u8)
    } else {
        let u = ((-t) * 255.0) as u8;
        ((u as f64 * 0.35) as u8, (u as f64 * 0.55) as u8, u)
    };

    fn to_hex(n: u8) -> [u8; 6] {
        let hi = n >> 4;
        let lo = n & 0xf;
        fn h(x: u8) -> u8 { if x < 10 { b'0' + x } else { b'a' + x - 10 } }
        [h(hi), h(lo), h(hi), h(lo), h(hi), h(lo)]
    }
    let hx = to_hex(r);
    let _ = g; let _ = b;

    let hi_r = r >> 4; let lo_r = r & 0xf;
    let hi_g = g >> 4; let lo_g = g & 0xf;
    let hi_b = b >> 4; let lo_b = b & 0xf;
    fn h(x: u8) -> u8 { if x < 10 { b'0' + x } else { b'a' + x - 10 } }
    let fill = [h(hi_r), h(lo_r), h(hi_g), h(lo_g), h(hi_b), h(lo_b)];
    let stroke = fill;
    (fill, stroke)
}

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\",\"D\"], matrix=[1,0.8,-0.3,0.5, 0.8,1,0.1,-0.2, -0.3,0.1,1,0.7, 0.5,-0.2,0.7,1]")]
pub fn render(cfg: &CorrelogramConfig) -> String {
    render_impl(cfg, false, false, false, false)
}

pub fn render_heatmap(cfg: &CorrelogramConfig)  -> String { render_impl(cfg, true,  false, false, false) }
pub fn render_text(cfg: &CorrelogramConfig)     -> String { render_impl(cfg, false, true,  false, false) }
pub fn render_mixed(cfg: &CorrelogramConfig)    -> String { render_impl(cfg, false, false, true,  false) }
pub fn render_gradient(cfg: &CorrelogramConfig) -> String { render_impl(cfg, false, false, false, true)  }

fn render_impl(cfg: &CorrelogramConfig, heatmap: bool, text_only: bool, mixed: bool, _gradient: bool) -> String {
    let n = cfg.labels.len();
    if n == 0 || cfg.matrix.len() < n * n { return String::new(); }

    let title_h = if cfg.title.is_empty() { 0.0f64 } else { 22.0 };
    let pad_l = 52.0f64;
    let pad_t = title_h + 12.0;
    let pad_b = 52.0f64;
    let pad_r = 12.0f64;
    let cell = ((cfg.width as f64 - pad_l - pad_r) / n as f64)
        .min((cfg.height as f64 - pad_t - pad_b) / n as f64);

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n * n * 200 + 4096);
    html_prefix(&mut buf, cfg.title, hid);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cfg.width as f64 / 2.0);
        push_b(&mut buf, b"\" y=\"18\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"13\" font-weight=\"700\" fill=\"#1a202c\" class=\"sp-ttl\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    for row in 0..n {
        for col in 0..n {
            let v = cfg.matrix[row * n + col];
            let (fill, _) = corr_color(v);
            let cx = pad_l + (col as f64 + 0.5) * cell;
            let cy = pad_t + (row as f64 + 0.5) * cell;

            if heatmap {
                let x = pad_l + col as f64 * cell;
                let y = pad_t + row as f64 * cell;
                push_b(&mut buf, b"<rect x=\"");
                push_f2(&mut buf, x);
                push_b(&mut buf, b"\" y=\"");
                push_f2(&mut buf, y);
                push_b(&mut buf, b"\" width=\"");
                push_f2(&mut buf, cell);
                push_b(&mut buf, b"\" height=\"");
                push_f2(&mut buf, cell);
                push_b(&mut buf, b"\" fill=\"#");
                buf.extend_from_slice(&fill);
                push_b(&mut buf, b"\" fill-opacity=\"0.85\" stroke=\"#fff\" stroke-width=\"0.5\" data-idx=\"");
                push_i(&mut buf, (row * n + col) as i32);
                push_b(&mut buf, b"\"/>");
            } else if !text_only {
                let r = (v.abs() * cell * 0.46).max(1.5);
                push_b(&mut buf, b"<circle cx=\"");
                push_f2(&mut buf, cx);
                push_b(&mut buf, b"\" cy=\"");
                push_f2(&mut buf, cy);
                push_b(&mut buf, b"\" r=\"");
                push_f2(&mut buf, r);
                push_b(&mut buf, b"\" fill=\"#");
                buf.extend_from_slice(&fill);
                push_b(&mut buf, b"\" fill-opacity=\"0.80\" data-idx=\"");
                push_i(&mut buf, (row * n + col) as i32);
                push_b(&mut buf, b"\"/>");
            }

            if text_only || mixed || cfg.show_values || (heatmap && cell > 28.0) {
                let label = format!("{:.2}", v);
                push_b(&mut buf, b"<text x=\"");
                push_f2(&mut buf, cx);
                push_b(&mut buf, b"\" y=\"");
                push_f2(&mut buf, cy + 4.0);
                push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"");
                push_f2(&mut buf, (cell * 0.28).min(11.0).max(7.0));
                let fg: &[u8] = if heatmap || text_only { b"\" fill=\"#1e293b\">" } else { b"\" fill=\"#1e293b\">" };
                buf.extend_from_slice(fg);
                buf.extend_from_slice(label.as_bytes());
                push_b(&mut buf, b"</text>");
            }
        }
    }

    let fs_axis = (cell * 0.3).min(10.5).max(7.5);
    for i in 0..n {
        let label = cfg.labels.get(i).map(|s| s.as_str()).unwrap_or("");
        let cx = pad_l + (i as f64 + 0.5) * cell;
        let cy = pad_t + (i as f64 + 0.5) * cell;

        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, pad_l - 4.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy + 3.5);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"");
        push_f2(&mut buf, fs_axis);
        push_b(&mut buf, b"\" fill=\"#374151\">");
        escape_xml(&mut buf, label);
        push_b(&mut buf, b"</text>");

        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, pad_t + n as f64 * cell + 14.0);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"");
        push_f2(&mut buf, fs_axis);
        push_b(&mut buf, b"\" fill=\"#374151\">");
        escape_xml(&mut buf, label);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}
