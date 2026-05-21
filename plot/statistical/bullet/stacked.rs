use super::common::{prepare, max_for, open_svg, label_left, value_text, data_value_rect, finalize};
use super::config::BulletConfig;
use crate::plot::statistical::common::{push_b, push_i, hex6};

#[crate::chart_demo("labels=[\"Revenue\",\"Profit\",\"CSAT\"], values=[80,65,4.2], targets=[90,70,4.5], max_vals=[120,100,5], ranges=[[40,70,100],[30,60,90],[2,3,4]]")]

pub fn render(cfg: &BulletConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = Vec::<u8>::with_capacity(p.n * 380 + 1024);
    open_svg(&mut b, cfg, &p);
    let bar_h = (p.row_h as f64 * 0.34) as i32;
    let range_h = (p.row_h as f64 * 0.62) as i32;
    let zones = [(0.40_f64, 0xCBD5E1_u32), (0.75_f64, 0xE0E7FF_u32), (1.00_f64, 0xC7D2FE_u32)];
    for i in 0..p.n {
        let max_v = max_for(&p, i);
        let ry = p.pad_t + i as i32 * p.row_h + (p.row_h - range_h) / 2;
        let by = p.pad_t + i as i32 * p.row_h + (p.row_h - bar_h) / 2;
        let mut prev = 0_i32;
        for (frac, color) in zones.iter() {
            let x_end = p.pad_l + ((*frac) * p.plot_w as f64) as i32;
            let hx = hex6(*color);
            push_b(&mut b, b"<rect x=\""); push_i(&mut b, p.pad_l + prev);
            push_b(&mut b, b"\" y=\""); push_i(&mut b, ry);
            push_b(&mut b, b"\" width=\""); push_i(&mut b, x_end - (p.pad_l + prev));
            push_b(&mut b, b"\" height=\""); push_i(&mut b, range_h);
            push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx);
            push_b(&mut b, b"\"/>");
            prev = x_end - p.pad_l;
        }
        let value_w = ((p.values[i] / max_v).min(1.0) * p.plot_w as f64) as i32;
        data_value_rect(&mut b, &p, i, p.pad_l, by, value_w, bar_h, 0x1E293B, 2, 0.92);
        if p.targets[i] > 0.0 {
            let tx = p.pad_l + ((p.targets[i] / max_v).min(1.0) * p.plot_w as f64) as i32;
            push_b(&mut b, b"<line x1=\""); push_i(&mut b, tx);
            push_b(&mut b, b"\" y1=\""); push_i(&mut b, ry - 2);
            push_b(&mut b, b"\" x2=\""); push_i(&mut b, tx);
            push_b(&mut b, b"\" y2=\""); push_i(&mut b, ry + range_h + 2);
            push_b(&mut b, b"\" stroke=\"#dc2626\" stroke-width=\"2.6\"/>");
        }
        label_left(&mut b, &p, i, by, bar_h);
        value_text(&mut b, p.values[i], p.pad_l + value_w + 5, by + bar_h / 2 + 4);
    }
    finalize(b, cfg)
}

