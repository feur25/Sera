use super::common::{finalize, label_left, open_frame, prepare, x_at};
use super::config::EventplotConfig;
use crate::plot::statistical::common::{hex6, palette_color, push_b, push_f2, push_i};
use crate::plot::statistical::kde::common::{kde_eval, scott_bw};

#[crate::chart_demo(
    "x=[1,2,2.5,4,5,5.5,6,2,3,7], categories=[\"A\",\"A\",\"A\",\"A\",\"A\",\"B\",\"B\",\"B\",\"B\",\"B\"]"
)]

pub fn render(cfg: &EventplotConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut f = open_frame(cfg, &p);
    let n_rows = p.row_names.len();
    let pitch = f.ph as f64 / n_rows as f64;
    let tick_h = (pitch * 0.35).max(3.0);
    let curve_h = pitch * 0.55;
    let samples = 64usize;
    let xr = (p.xmax - p.xmin).max(1e-9);

    for (ri, name) in p.row_names.iter().enumerate() {
        let base_y = f.pt as f64 + (ri + 1) as f64 * pitch;
        let cy = base_y - pitch / 2.0;
        let color = hex6(palette_color(cfg.palette, ri));
        label_left(&mut f, name, cy);

        let events = &p.row_events[ri];
        let bw = scott_bw(events).max(1e-6);
        let mut dens = vec![0.0f64; samples];
        let mut maxd = 0.0f64;
        for s in 0..samples {
            let x = p.xmin + xr * s as f64 / (samples - 1) as f64;
            let d = kde_eval(events, x, bw);
            dens[s] = d;
            if d > maxd {
                maxd = d;
            }
        }
        if maxd <= 0.0 {
            maxd = 1.0;
        }

        push_b(&mut f.buf, b"<polyline points=\"");
        for s in 0..samples {
            let xv = p.xmin + xr * s as f64 / (samples - 1) as f64;
            let x = x_at(&f, &p, xv);
            let y = base_y - 4.0 - (dens[s] / maxd) * curve_h;
            if s > 0 {
                push_b(&mut f.buf, b" ");
            }
            push_f2(&mut f.buf, x);
            push_b(&mut f.buf, b",");
            push_f2(&mut f.buf, y);
        }
        push_b(&mut f.buf, b"\" fill=\"none\" stroke=\"#");
        f.buf.extend_from_slice(&color);
        push_b(&mut f.buf, b"\" stroke-width=\"1.6\" stroke-opacity=\"0.85\"/>");

        for (ei, &v) in events.iter().enumerate() {
            let x = x_at(&f, &p, v);
            push_b(&mut f.buf, b"<line data-idx=\"");
            push_i(&mut f.buf, ei as i32);
            push_b(&mut f.buf, b"\" data-y=\"");
            push_f2(&mut f.buf, v);
            push_b(&mut f.buf, b"\" x1=\"");
            push_f2(&mut f.buf, x);
            push_b(&mut f.buf, b"\" y1=\"");
            push_f2(&mut f.buf, base_y - tick_h);
            push_b(&mut f.buf, b"\" x2=\"");
            push_f2(&mut f.buf, x);
            push_b(&mut f.buf, b"\" y2=\"");
            push_f2(&mut f.buf, base_y);
            push_b(&mut f.buf, b"\" stroke=\"#");
            f.buf.extend_from_slice(&color);
            push_b(&mut f.buf, b"\" stroke-width=\"1.2\" stroke-opacity=\"0.7\"/>");
        }
    }
    finalize(f, cfg)
}
