use super::common::{prepare, max_for, open_svg, label_left, finalize};
use super::config::BulletConfig;
use crate::plot::statistical::common::{push_b, push_i, push_f2, escape_xml, hex6};

#[crate::chart_demo("labels=[\"Revenue\",\"Profit\",\"CSAT\"], values=[80,65,4.2], targets=[90,70,4.5], max_vals=[120,100,5]")]

pub fn render(cfg: &BulletConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = Vec::<u8>::with_capacity(p.n * 360 + 1024);
    open_svg(&mut b, cfg, &p);
    let bar_h = (p.row_h as f64 * 0.5).max(14.0) as i32;
    push_b(&mut b, b"<defs><linearGradient id=\"sp-bp-grad\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\"><stop offset=\"0%\" stop-color=\"#6366F1\"/><stop offset=\"100%\" stop-color=\"#22D3EE\"/></linearGradient></defs>");
    let hx_track = hex6(0xE5E7EB);
    for i in 0..p.n {
        let max_v = max_for(&p, i);
        let by = p.pad_t + i as i32 * p.row_h + (p.row_h - bar_h) / 2;
        let frac = (p.values[i] / max_v).min(1.0).max(0.0);
        let value_w = (frac * p.plot_w as f64) as i32;
        push_b(&mut b, b"<rect x=\""); push_i(&mut b, p.pad_l);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, by);
        push_b(&mut b, b"\" width=\""); push_i(&mut b, p.plot_w);
        push_b(&mut b, b"\" height=\""); push_i(&mut b, bar_h);
        push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx_track);
        push_b(&mut b, b"\" rx=\""); push_i(&mut b, bar_h / 2);
        push_b(&mut b, b"\"/>");
        push_b(&mut b, b"<rect data-idx=\""); push_i(&mut b, i as i32);
        push_b(&mut b, b"\" data-y=\""); push_f2(&mut b, p.values[i]);
        push_b(&mut b, b"\" data-lbl=\""); escape_xml(&mut b, &p.labels[i]);
        push_b(&mut b, b"\" x=\""); push_i(&mut b, p.pad_l);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, by);
        push_b(&mut b, b"\" width=\""); push_i(&mut b, value_w.max(2));
        push_b(&mut b, b"\" height=\""); push_i(&mut b, bar_h);
        push_b(&mut b, b"\" fill=\"url(#sp-bp-grad)\" rx=\""); push_i(&mut b, bar_h / 2);
        push_b(&mut b, b"\"/>");
        push_b(&mut b, b"<text x=\""); push_i(&mut b, p.pad_l + p.plot_w / 2);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, by + bar_h / 2 + 5);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"12\" font-weight=\"700\" fill=\"#0f172a\">");
        push_f2(&mut b, frac * 100.0);
        push_b(&mut b, b"%</text>");
        if p.targets[i] > 0.0 {
            let tx = p.pad_l + ((p.targets[i] / max_v).min(1.0) * p.plot_w as f64) as i32;
            push_b(&mut b, b"<line x1=\""); push_i(&mut b, tx);
            push_b(&mut b, b"\" y1=\""); push_i(&mut b, by - 3);
            push_b(&mut b, b"\" x2=\""); push_i(&mut b, tx);
            push_b(&mut b, b"\" y2=\""); push_i(&mut b, by + bar_h + 3);
            push_b(&mut b, b"\" stroke=\"#0f172a\" stroke-width=\"2.4\"/>");
        }
        label_left(&mut b, &p, i, by, bar_h);
    }
    finalize(b, cfg)
}

