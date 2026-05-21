use super::common::{prepare, max_for, open_svg, label_left, value_text, finalize};
use super::config::BulletConfig;
use crate::plot::statistical::common::{push_b, push_i, push_f2, escape_xml, hex6};

#[crate::chart_demo("labels=[\"Revenue\",\"Profit\",\"CSAT\"], values=[80,65,4.2], targets=[90,70,4.5], max_vals=[120,100,5]")]

pub fn render(cfg: &BulletConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = Vec::<u8>::with_capacity(p.n * 280 + 1024);
    open_svg(&mut b, cfg, &p);
    let bar_h = 6;
    let dot_r = (p.row_h as f64 * 0.22).max(7.0) as i32;
    let hx_track = hex6(0xE5E7EB);
    let hx_value = hex6(0x6366F1);
    for i in 0..p.n {
        let max_v = max_for(&p, i);
        let cy = p.pad_t + i as i32 * p.row_h + p.row_h / 2;
        let cx = p.pad_l + ((p.values[i] / max_v).min(1.0) * p.plot_w as f64) as i32;
        push_b(&mut b, b"<rect x=\""); push_i(&mut b, p.pad_l);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, cy - bar_h / 2);
        push_b(&mut b, b"\" width=\""); push_i(&mut b, p.plot_w);
        push_b(&mut b, b"\" height=\""); push_i(&mut b, bar_h);
        push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx_track);
        push_b(&mut b, b"\" rx=\""); push_i(&mut b, bar_h / 2);
        push_b(&mut b, b"\"/>");
        if p.targets[i] > 0.0 {
            let tx = p.pad_l + ((p.targets[i] / max_v).min(1.0) * p.plot_w as f64) as i32;
            push_b(&mut b, b"<line x1=\""); push_i(&mut b, tx);
            push_b(&mut b, b"\" y1=\""); push_i(&mut b, cy - dot_r - 2);
            push_b(&mut b, b"\" x2=\""); push_i(&mut b, tx);
            push_b(&mut b, b"\" y2=\""); push_i(&mut b, cy + dot_r + 2);
            push_b(&mut b, b"\" stroke=\"#0f172a\" stroke-width=\"2.2\" stroke-dasharray=\"3 2\"/>");
        }
        push_b(&mut b, b"<circle data-idx=\""); push_i(&mut b, i as i32);
        push_b(&mut b, b"\" data-y=\""); push_f2(&mut b, p.values[i]);
        push_b(&mut b, b"\" data-lbl=\""); escape_xml(&mut b, &p.labels[i]);
        push_b(&mut b, b"\" cx=\""); push_i(&mut b, cx);
        push_b(&mut b, b"\" cy=\""); push_i(&mut b, cy);
        push_b(&mut b, b"\" r=\""); push_i(&mut b, dot_r);
        push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx_value);
        push_b(&mut b, b"\" stroke=\"#fff\" stroke-width=\"2\"/>");
        label_left(&mut b, &p, i, cy - 6, 12);
        value_text(&mut b, p.values[i], cx + dot_r + 5, cy + 4);
    }
    finalize(b, cfg)
}

