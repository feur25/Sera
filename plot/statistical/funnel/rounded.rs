use super::common::{prepare, open_svg, finalize, label_inside, label_left_pct, label_right_val, step_color};
use super::config::FunnelConfig;
use crate::plot::statistical::common::{push_b, push_i, escape_xml};

pub const DEMO_KWARGS: &str = "labels=[\"Visits\",\"Signups\",\"Trial\",\"Paid\",\"Renewed\"], values=[1000,520,210,85,40]";
pub fn render(cfg: &FunnelConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let l = &p.layout;
    let mut b = Vec::<u8>::with_capacity(p.n * 320 + 1024);
    open_svg(&mut b, cfg);
    let r: i32 = 10;
    let mut prev_ratio = 1.0_f64;
    for i in 0..p.n {
        let ratio = p.values[i] / p.max_val;
        let y = l.pad_t + i as i32 * (l.step_h + l.gap);
        let h = l.step_h;
        let top_w = (prev_ratio * l.plot_w as f64) as i32;
        let bot_w = (ratio * l.plot_w as f64) as i32;
        let xt0 = l.cx - top_w / 2; let xt1 = l.cx + top_w / 2;
        let xb0 = l.cx - bot_w / 2; let xb1 = l.cx + bot_w / 2;
        let hx = step_color(cfg.palette, i);
        push_b(&mut b, b"<path data-idx=\""); push_i(&mut b, i as i32);
        push_b(&mut b, b"\" data-lbl=\""); escape_xml(&mut b, &p.labels[i]);
        push_b(&mut b, b"\" d=\"M ");
        push_i(&mut b, xt0 + r); b.push(b' '); push_i(&mut b, y);
        push_b(&mut b, b" L "); push_i(&mut b, xt1 - r); b.push(b' '); push_i(&mut b, y);
        push_b(&mut b, b" Q "); push_i(&mut b, xt1); b.push(b' '); push_i(&mut b, y);
        b.push(b' '); push_i(&mut b, xt1); b.push(b' '); push_i(&mut b, y + r);
        push_b(&mut b, b" L "); push_i(&mut b, xb1); b.push(b' '); push_i(&mut b, y + h - r);
        push_b(&mut b, b" Q "); push_i(&mut b, xb1); b.push(b' '); push_i(&mut b, y + h);
        b.push(b' '); push_i(&mut b, xb1 - r); b.push(b' '); push_i(&mut b, y + h);
        push_b(&mut b, b" L "); push_i(&mut b, xb0 + r); b.push(b' '); push_i(&mut b, y + h);
        push_b(&mut b, b" Q "); push_i(&mut b, xb0); b.push(b' '); push_i(&mut b, y + h);
        b.push(b' '); push_i(&mut b, xb0); b.push(b' '); push_i(&mut b, y + h - r);
        push_b(&mut b, b" L "); push_i(&mut b, xt0); b.push(b' '); push_i(&mut b, y + r);
        push_b(&mut b, b" Q "); push_i(&mut b, xt0); b.push(b' '); push_i(&mut b, y);
        b.push(b' '); push_i(&mut b, xt0 + r); b.push(b' '); push_i(&mut b, y);
        push_b(&mut b, b" Z\" fill=\"#"); b.extend_from_slice(&hx); push_b(&mut b, b"\" opacity=\"0.92\"/>");
        if cfg.show_text {
            label_inside(&mut b, l.cx, y + h / 2, &p.labels[i]);
            label_left_pct(&mut b, l.pad_l - 8, y + h / 2, p.values[i], p.max_val);
            label_right_val(&mut b, cfg.width - l.pad_r + 8, y + h / 2, p.values[i]);
        }
        prev_ratio = ratio;
    }
    finalize(b, cfg)
}


