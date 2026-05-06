use super::common::{prepare, open, axes_grid, legend, finalize, point, write_dots};
use super::config::ParallelConfig;
use crate::plot::statistical::common::{palette_color, push_b, push_i, push_f2, escape_xml, hex6};

pub fn render(cfg: &ParallelConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = open(cfg, &p);
    axes_grid(&mut b, cfg, &p);
    for si in 0..p.n_series {
        let v = &cfg.series_values[si];
        let col = palette_color(cfg.palette, si);
        let hx = hex6(col);
        write_curve(&mut b, &p, v, &hx, 4.0, 0.12, si, &cfg.series_names[si]);
        write_curve(&mut b, &p, v, &hx, 1.8, 0.75, si, &cfg.series_names[si]);
        write_dots(&mut b, &p, v, col, 3.0, 0.85, si);
    }
    legend(&mut b, cfg, &p);
    finalize(b, cfg)
}

fn write_curve(buf: &mut Vec<u8>, p: &super::common::Prepared, vals: &[f64], hx: &[u8;6], sw: f64, op: f64, si: usize, name: &str) {
    let m = p.n_axes.min(vals.len());
    if m < 2 { return; }
    let mut pts: Vec<(f64,f64)> = (0..m).map(|ai| point(p, ai, vals[ai])).collect();
    let first = pts[0]; let last = pts[m-1];
    pts.insert(0, first); pts.push(last);
    push_b(buf, b"<path data-idx=\""); push_i(buf, si as i32);
    push_b(buf, b"\" data-series=\""); push_i(buf, si as i32);
    push_b(buf, b"\" data-lbl=\""); escape_xml(buf, name);
    push_b(buf, b"\" fill=\"none\" stroke=\"#"); buf.extend_from_slice(hx);
    push_b(buf, b"\" stroke-width=\""); push_f2(buf, sw);
    push_b(buf, b"\" stroke-opacity=\""); push_f2(buf, op);
    push_b(buf, b"\" stroke-linejoin=\"round\" d=\"M ");
    push_f2(buf, pts[1].0); push_b(buf, b" "); push_f2(buf, pts[1].1);
    for i in 1..pts.len()-2 {
        let p0 = pts[i-1]; let p1 = pts[i]; let p2 = pts[i+1]; let p3 = pts[i+2];
        let c1x = p1.0 + (p2.0 - p0.0) / 6.0;
        let c1y = p1.1 + (p2.1 - p0.1) / 6.0;
        let c2x = p2.0 - (p3.0 - p1.0) / 6.0;
        let c2y = p2.1 - (p3.1 - p1.1) / 6.0;
        push_b(buf, b" C "); push_f2(buf, c1x); push_b(buf, b" "); push_f2(buf, c1y);
        push_b(buf, b", "); push_f2(buf, c2x); push_b(buf, b" "); push_f2(buf, c2y);
        push_b(buf, b", "); push_f2(buf, p2.0); push_b(buf, b" "); push_f2(buf, p2.1);
    }
    push_b(buf, b"\"/>");
}
