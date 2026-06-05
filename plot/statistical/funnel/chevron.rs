use super::common::{
    finalize, label_inside, label_left_pct, label_right_val, open_svg, prepare, step_color,
};
use super::config::FunnelConfig;
use crate::plot::statistical::common::{escape_xml, push_b, push_i};

#[crate::chart_demo(
    "labels=[\"Visits\",\"Signups\",\"Trial\",\"Paid\",\"Renewed\"], values=[1000,520,210,85,40]"
)]

pub fn render(cfg: &FunnelConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let l = &p.layout;
    let mut b = Vec::<u8>::with_capacity(p.n * 280 + 1024);
    open_svg(&mut b, cfg);
    let tip: i32 = 16;
    for i in 0..p.n {
        let ratio = p.values[i] / p.max_val;
        let w = (ratio * l.plot_w as f64) as i32;
        let x0 = l.cx - w / 2;
        let x1 = l.cx + w / 2;
        let y = l.pad_t + i as i32 * (l.step_h + l.gap);
        let h = l.step_h;
        let hx = step_color(cfg.palette, i);
        push_b(&mut b, b"<path data-idx=\"");
        push_i(&mut b, i as i32);
        push_b(&mut b, b"\" data-lbl=\"");
        escape_xml(&mut b, &p.labels[i]);
        push_b(&mut b, b"\" d=\"M ");
        push_i(&mut b, x0);
        b.push(b' ');
        push_i(&mut b, y);
        push_b(&mut b, b" L ");
        push_i(&mut b, x1 - tip);
        b.push(b' ');
        push_i(&mut b, y);
        push_b(&mut b, b" L ");
        push_i(&mut b, x1);
        b.push(b' ');
        push_i(&mut b, y + h / 2);
        push_b(&mut b, b" L ");
        push_i(&mut b, x1 - tip);
        b.push(b' ');
        push_i(&mut b, y + h);
        push_b(&mut b, b" L ");
        push_i(&mut b, x0);
        b.push(b' ');
        push_i(&mut b, y + h);
        push_b(&mut b, b" L ");
        push_i(&mut b, x0 + tip);
        b.push(b' ');
        push_i(&mut b, y + h / 2);
        push_b(&mut b, b" Z\" fill=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" opacity=\"0.92\"/>");
        if cfg.show_text {
            label_inside(&mut b, l.cx, y + h / 2, &p.labels[i]);
            label_left_pct(&mut b, l.pad_l - 8, y + h / 2, p.values[i], p.max_val);
            label_right_val(&mut b, cfg.width - l.pad_r + 8, y + h / 2, p.values[i]);
        }
    }
    finalize(b, cfg)
}
