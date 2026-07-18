use super::common::{bar_data_attrs, finalize, label_left, legend, open_frame, prepare, row_color, x_at};
use super::config::GanttConfig;
use crate::plot::statistical::common::{hex6, push_b, push_f2, push_i};

#[crate::chart_demo(
    "labels=[\"Design\",\"Build\",\"Test\",\"Launch\"], start=[0,5,12,18], end=[6,14,19,22], categories=[\"Plan\",\"Dev\",\"Dev\",\"Plan\"]"
)]

pub fn render(cfg: &GanttConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut f = open_frame(cfg, &p);
    let pitch = f.ph as f64 / p.n as f64;
    let bar_h = (pitch * 0.55).max(6.0);
    for i in 0..p.n {
        let cy = f.pt + (i as f64 * pitch + pitch / 2.0) as i32;
        let x1 = x_at(&f, &p, p.start[i]);
        let x2 = x_at(&f, &p, p.end[i]);
        let color = hex6(row_color(&p, cfg, i));
        label_left(&mut f, &p, i, cy);
        push_b(&mut f.buf, b"<rect");
        bar_data_attrs(&mut f, &p, i);
        push_b(&mut f.buf, b" x=\"");
        push_i(&mut f.buf, x1);
        push_b(&mut f.buf, b"\" y=\"");
        push_f2(&mut f.buf, cy as f64 - bar_h / 2.0);
        push_b(&mut f.buf, b"\" width=\"");
        push_i(&mut f.buf, (x2 - x1).max(2));
        push_b(&mut f.buf, b"\" height=\"");
        push_f2(&mut f.buf, bar_h);
        push_b(&mut f.buf, b"\" rx=\"3\" fill=\"#");
        f.buf.extend_from_slice(&color);
        push_b(&mut f.buf, b"\"/>");
    }
    legend(&mut f, &p);
    finalize(f, cfg)
}
