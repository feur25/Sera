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
    push_b(&mut f.buf, b"<defs><marker id=\"sp-db-arr\" viewBox=\"0 0 10 10\" refX=\"8\" refY=\"5\" markerWidth=\"7\" markerHeight=\"7\" orient=\"auto\"><path d=\"M0,0 L10,5 L0,10 z\" fill=\"#475569\"/></marker></defs>");
    for i in 0..p.n {
        let cy = f.pt + (i as f64 * pitch + pitch / 2.0) as i32;
        let x1 = x_at(&f, &p, p.start[i]);
        let x2 = x_at(&f, &p, p.end[i]);
        label_left(&mut f, &p, i, cy);
        let hx1 = hex6(p.c1);
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
            b"\" stroke=\"#475569\" stroke-width=\"2.2\" marker-end=\"url(#sp-db-arr)\"/>",
        );
        push_b(&mut f.buf, b"<circle cx=\"");
        push_i(&mut f.buf, x1);
        push_b(&mut f.buf, b"\" cy=\"");
        push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\"5\" fill=\"#fff\" stroke=\"#");
        f.buf.extend_from_slice(&hx1);
        push_b(&mut f.buf, b"\" stroke-width=\"2\"/>");
        data_dot(&mut f, &p, i, 0, cfg.series_names.0, x1, cy, 5, p.c1);
        data_dot(&mut f, &p, i, 1, cfg.series_names.1, x2, cy, 6, p.c2);
    }
    finalize(f, cfg, &p)
}
