use super::common::{
    dot, draw_axes, finalize, label_left, label_right, open_svg, prepare, val_to_y,
};
use super::config::SlopeConfig;
use crate::plot::statistical::common::{escape_xml, push_b, push_i};

#[crate::chart_demo(
    "labels=[\"A\",\"B\",\"C\",\"D\",\"E\"], left=[20,35,15,42,28], right=[35,28,40,55,22]"
)]

pub fn render(cfg: &SlopeConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut b = Vec::<u8>::with_capacity(p.n * 240 + 1024);
    open_svg(&mut b, cfg);
    draw_axes(&mut b, cfg, &p);
    let l = &p.layout;
    let mid_x = (l.x_left + l.x_right) / 2;
    for i in 0..p.n {
        let y1 = val_to_y(&p, p.values_left[i]);
        let y2 = val_to_y(&p, p.values_right[i]);
        let up = p.values_right[i] >= p.values_left[i];
        let color: &[u8] = if up { b"#10B981" } else { b"#F43F5E" };
        push_b(&mut b, b"<path data-idx=\"");
        push_i(&mut b, i as i32);
        push_b(&mut b, b"\" data-lbl=\"");
        escape_xml(&mut b, &p.labels[i]);
        push_b(&mut b, b"\" d=\"M ");
        push_i(&mut b, l.x_left);
        b.push(b' ');
        push_i(&mut b, y1);
        push_b(&mut b, b" L ");
        push_i(&mut b, mid_x);
        b.push(b' ');
        push_i(&mut b, y1);
        push_b(&mut b, b" L ");
        push_i(&mut b, mid_x);
        b.push(b' ');
        push_i(&mut b, y2);
        push_b(&mut b, b" L ");
        push_i(&mut b, l.x_right);
        b.push(b' ');
        push_i(&mut b, y2);
        push_b(&mut b, b"\" fill=\"none\" stroke=\"");
        b.extend_from_slice(color);
        push_b(&mut b, b"\" stroke-width=\"2\" stroke-linejoin=\"round\" stroke-linecap=\"round\" opacity=\"0.85\"/>");
        dot(&mut b, l.x_left, y1, color, 4.0);
        dot(&mut b, l.x_right, y2, color, 4.0);
        if cfg.show_text {
            label_left(&mut b, l.x_left, y1, &p.labels[i], p.values_left[i]);
            label_right(&mut b, l.x_right, y2, &p.labels[i], p.values_right[i]);
        }
    }
    finalize(b, cfg)
}
