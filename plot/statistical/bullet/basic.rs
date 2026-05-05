use super::common::{prepare, max_for, open_svg, label_left, value_text, data_value_rect, finalize};
use super::config::BulletConfig;
use crate::plot::statistical::common::{push_b, push_i, hex6};

pub fn render(cfg: &BulletConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = Vec::<u8>::with_capacity(p.n * 320 + 1024);
    open_svg(&mut b, cfg, &p);
    let bar_h = (p.row_h as f64 * 0.38) as i32;
    let range_h = (p.row_h as f64 * 0.6) as i32;
    let hx_track = hex6(0xE5E7EB);
    let hx_range = hex6(0xC7D2FE);
    for i in 0..p.n {
        let max_v = max_for(&p, i);
        let ry = p.pad_t + i as i32 * p.row_h + (p.row_h - range_h) / 2;
        let by = p.pad_t + i as i32 * p.row_h + (p.row_h - bar_h) / 2;
        let range_pct = if p.ranges[i] > 0.0 { (p.ranges[i] / max_v).min(1.0) } else { 0.75 };
        let range_w = (p.plot_w as f64 * range_pct) as i32;
        let value_w = ((p.values[i] / max_v).min(1.0) * p.plot_w as f64) as i32;
        push_b(&mut b, b"<rect x=\""); push_i(&mut b, p.pad_l);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, ry);
        push_b(&mut b, b"\" width=\""); push_i(&mut b, p.plot_w);
        push_b(&mut b, b"\" height=\""); push_i(&mut b, range_h);
        push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx_track);
        push_b(&mut b, b"\" rx=\"3\"/>");
        push_b(&mut b, b"<rect x=\""); push_i(&mut b, p.pad_l);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, ry);
        push_b(&mut b, b"\" width=\""); push_i(&mut b, range_w);
        push_b(&mut b, b"\" height=\""); push_i(&mut b, range_h);
        push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx_range);
        push_b(&mut b, b"\" rx=\"3\"/>");
        data_value_rect(&mut b, &p, i, p.pad_l, by, value_w, bar_h, 0x6366F1, 2, 0.9);
        if p.targets[i] > 0.0 {
            let tx = p.pad_l + ((p.targets[i] / max_v).min(1.0) * p.plot_w as f64) as i32;
            push_b(&mut b, b"<line x1=\""); push_i(&mut b, tx);
            push_b(&mut b, b"\" y1=\""); push_i(&mut b, ry);
            push_b(&mut b, b"\" x2=\""); push_i(&mut b, tx);
            push_b(&mut b, b"\" y2=\""); push_i(&mut b, ry + range_h);
            push_b(&mut b, b"\" stroke=\"#1a202c\" stroke-width=\"2.5\"/>");
        }
        label_left(&mut b, &p, i, by, bar_h);
        value_text(&mut b, p.values[i], p.pad_l + value_w + 5, by + bar_h / 2 + 4);
    }
    finalize(b, cfg)
}
