use super::common::{finalize, label_left, open_frame, prepare, x_at};
use super::config::EventplotConfig;
use crate::plot::statistical::common::{hex6, push_b, push_f2, push_i};
use crate::plot::statistical::heatmap::common::colorscale_color;

#[crate::chart_demo(
    "x=[1,2,2.5,4,5,5.5,6,2,3,7], categories=[\"A\",\"A\",\"A\",\"A\",\"A\",\"B\",\"B\",\"B\",\"B\",\"B\"], colorscale=\"plasma\""
)]

pub fn render(cfg: &EventplotConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let scale = if cfg.colorscale.is_empty() {
        "viridis"
    } else {
        cfg.colorscale
    };
    let span = (p.xmax - p.xmin).max(1e-9);
    let mut f = open_frame(cfg, &p);
    let n_rows = p.row_names.len();
    let pitch = f.ph as f64 / n_rows as f64;
    let tick_h = (pitch * 0.6).max(4.0);
    for (ri, name) in p.row_names.iter().enumerate() {
        let cy = f.pt as f64 + ri as f64 * pitch + pitch / 2.0;
        label_left(&mut f, name, cy);
        for (ei, &v) in p.row_events[ri].iter().enumerate() {
            let x = x_at(&f, &p, v);
            let t = (v - p.xmin) / span;
            let color = hex6(colorscale_color(scale, t));
            push_b(&mut f.buf, b"<line data-idx=\"");
            push_i(&mut f.buf, ei as i32);
            push_b(&mut f.buf, b"\" data-y=\"");
            push_f2(&mut f.buf, v);
            push_b(&mut f.buf, b"\" x1=\"");
            push_f2(&mut f.buf, x);
            push_b(&mut f.buf, b"\" y1=\"");
            push_f2(&mut f.buf, cy - tick_h / 2.0);
            push_b(&mut f.buf, b"\" x2=\"");
            push_f2(&mut f.buf, x);
            push_b(&mut f.buf, b"\" y2=\"");
            push_f2(&mut f.buf, cy + tick_h / 2.0);
            push_b(&mut f.buf, b"\" stroke=\"#");
            f.buf.extend_from_slice(&color);
            push_b(&mut f.buf, b"\" stroke-width=\"2\"/>");
        }
    }
    finalize(f, cfg)
}
