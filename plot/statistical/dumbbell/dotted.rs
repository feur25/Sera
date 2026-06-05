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
    let hx1 = hex6(p.c1);
    let hx2 = hex6(p.c2);
    for i in 0..p.n {
        let cy = f.pt + (i as f64 * pitch + pitch / 2.0) as i32;
        let x1 = x_at(&f, &p, p.start[i]);
        let x2 = x_at(&f, &p, p.end[i]);
        label_left(&mut f, &p, i, cy);
        push_b(&mut f.buf, b"<line x1=\"");
        push_i(&mut f.buf, x1);
        push_b(&mut f.buf, b"\" y1=\"");
        push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" x2=\"");
        push_i(&mut f.buf, x2);
        push_b(&mut f.buf, b"\" y2=\"");
        push_i(&mut f.buf, cy);
        push_b(
            &mut f.buf,
            b"\" stroke=\"#94a3b8\" stroke-width=\"1.5\" stroke-dasharray=\"4 3\"/>",
        );
        push_b(&mut f.buf, b"<circle cx=\"");
        push_i(&mut f.buf, x1);
        push_b(&mut f.buf, b"\" cy=\"");
        push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\"5\" fill=\"#fff\" stroke=\"#");
        f.buf.extend_from_slice(&hx1);
        push_b(&mut f.buf, b"\" stroke-width=\"2.2\"/>");
        push_b(&mut f.buf, b"<circle cx=\"");
        push_i(&mut f.buf, x2);
        push_b(&mut f.buf, b"\" cy=\"");
        push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\"5\" fill=\"#fff\" stroke=\"#");
        f.buf.extend_from_slice(&hx2);
        push_b(&mut f.buf, b"\" stroke-width=\"2.2\"/>");
        data_dot(&mut f, &p, i, 0, cfg.series_names.0, x1, cy, 5, p.c1);
        data_dot(&mut f, &p, i, 1, cfg.series_names.1, x2, cy, 5, p.c2);
    }
    finalize(f, cfg, &p)
}
