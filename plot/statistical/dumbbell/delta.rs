use super::common::{data_dot, finalize, label_left, open_frame, prepare, x_at};
use super::config::DumbbellConfig;
use crate::plot::statistical::common::{hex6, push_b, push_i};

#[crate::chart_demo(
    "labels=[\"A\",\"B\",\"C\",\"D\",\"E\"], start=[20,35,15,42,28], end=[60,52,38,68,55]"
)]

pub fn render(cfg: &DumbbellConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut f = open_frame(cfg, &p);
    let pitch = f.ph as f64 / p.n as f64;
    let up: u32 = 0x10B981;
    let dn: u32 = 0xEF4444;
    for i in 0..p.n {
        let cy = f.pt + (i as f64 * pitch + pitch / 2.0) as i32;
        let x1 = x_at(&f, &p, p.start[i]);
        let x2 = x_at(&f, &p, p.end[i]);
        let positive = p.end[i] >= p.start[i];
        let bar = if positive { up } else { dn };
        let hx = hex6(bar);
        label_left(&mut f, &p, i, cy);
        let (lo, hi) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        push_b(&mut f.buf, b"<rect x=\"");
        push_i(&mut f.buf, lo);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, cy - 4);
        push_b(&mut f.buf, b"\" width=\"");
        push_i(&mut f.buf, (hi - lo).max(1));
        push_b(&mut f.buf, b"\" height=\"8\" rx=\"4\" fill=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.55\"/>");
        data_dot(&mut f, &p, i, 0, cfg.series_names.0, x1, cy, 6, p.c1);
        data_dot(&mut f, &p, i, 1, cfg.series_names.1, x2, cy, 6, p.c2);
    }
    finalize(f, cfg, &p)
}
