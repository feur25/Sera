use super::common::{prepare, max_for, open_svg, label_left, value_text, data_value_rect, finalize};
use super::config::BulletConfig;
use crate::plot::statistical::common::{push_b, push_i};

#[crate::chart_demo("labels=[\"Revenue\",\"Profit\",\"CSAT\"], values=[80,65,4.2], targets=[90,70,4.5], max_vals=[120,100,5]")]

pub fn render(cfg: &BulletConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = Vec::<u8>::with_capacity(p.n * 240 + 1024);
    open_svg(&mut b, cfg, &p);
    let bar_h = (p.row_h as f64 * 0.18).max(4.0) as i32;
    for i in 0..p.n {
        let max_v = max_for(&p, i);
        let by = p.pad_t + i as i32 * p.row_h + (p.row_h - bar_h) / 2;
        let value_w = ((p.values[i] / max_v).min(1.0) * p.plot_w as f64) as i32;
        push_b(&mut b, b"<line x1=\""); push_i(&mut b, p.pad_l);
        push_b(&mut b, b"\" y1=\""); push_i(&mut b, by + bar_h / 2);
        push_b(&mut b, b"\" x2=\""); push_i(&mut b, p.pad_l + p.plot_w);
        push_b(&mut b, b"\" y2=\""); push_i(&mut b, by + bar_h / 2);
        push_b(&mut b, b"\" stroke=\"#e2e8f0\" stroke-width=\"1\"/>");
        data_value_rect(&mut b, &p, i, p.pad_l, by, value_w, bar_h, 0x6366F1, bar_h / 2, 1.0);
        if p.targets[i] > 0.0 {
            let tx = p.pad_l + ((p.targets[i] / max_v).min(1.0) * p.plot_w as f64) as i32;
            push_b(&mut b, b"<line x1=\""); push_i(&mut b, tx);
            push_b(&mut b, b"\" y1=\""); push_i(&mut b, by - 6);
            push_b(&mut b, b"\" x2=\""); push_i(&mut b, tx);
            push_b(&mut b, b"\" y2=\""); push_i(&mut b, by + bar_h + 6);
            push_b(&mut b, b"\" stroke=\"#0f172a\" stroke-width=\"2\"/>");
        }
        label_left(&mut b, &p, i, by, bar_h);
        value_text(&mut b, p.values[i], p.pad_l + value_w + 5, by + bar_h / 2 + 4);
    }
    finalize(b, cfg)
}

