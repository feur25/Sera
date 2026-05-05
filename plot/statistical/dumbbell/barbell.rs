use super::common::{prepare, open_frame, finalize, label_left, data_dot, x_at};
use super::config::DumbbellConfig;
use crate::plot::statistical::common::{push_b, push_i, hex6};

pub fn render(cfg: &DumbbellConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut f = open_frame(cfg, &p);
    let pitch = f.ph as f64 / p.n as f64;
    for i in 0..p.n {
        let cy = f.pt + (i as f64 * pitch + pitch / 2.0) as i32;
        let x1 = x_at(&f, &p, p.start[i]);
        let x2 = x_at(&f, &p, p.end[i]);
        label_left(&mut f, &p, i, cy);
        let (lo, hi) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        push_b(&mut f.buf, b"<rect x=\""); push_i(&mut f.buf, lo);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, cy - 5);
        push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, (hi - lo).max(1));
        push_b(&mut f.buf, b"\" height=\"10\" rx=\"2\" fill=\"#475569\"/>");
        let hx1 = hex6(p.c1);
        let hx2 = hex6(p.c2);
        push_b(&mut f.buf, b"<rect data-idx=\""); push_i(&mut f.buf, (i as i32) * 2);
        push_b(&mut f.buf, b"\" data-series=\"0\" x=\""); push_i(&mut f.buf, x1 - 7);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, cy - 11);
        push_b(&mut f.buf, b"\" width=\"14\" height=\"22\" rx=\"2\" fill=\"#"); f.buf.extend_from_slice(&hx1);
        push_b(&mut f.buf, b"\"/>");
        push_b(&mut f.buf, b"<rect data-idx=\""); push_i(&mut f.buf, (i as i32) * 2 + 1);
        push_b(&mut f.buf, b"\" data-series=\"1\" x=\""); push_i(&mut f.buf, x2 - 7);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, cy - 11);
        push_b(&mut f.buf, b"\" width=\"14\" height=\"22\" rx=\"2\" fill=\"#"); f.buf.extend_from_slice(&hx2);
        push_b(&mut f.buf, b"\"/>");
        let _ = data_dot;
    }
    finalize(f, cfg, &p)
}
