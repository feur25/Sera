use super::common::{prepare, open_frame, finalize, data_dot, x_at};
use super::config::DumbbellConfig;
use crate::plot::statistical::common::{push_b, push_i, escape_xml, truncate};

pub fn render(cfg: &DumbbellConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut f = open_frame(cfg, &p);
    let pitch = f.ph as f64 / p.n as f64;
    let n = p.n;
    for i in 0..n {
        let cy = f.pt + (i as f64 * pitch + pitch / 2.0) as i32;
        let x1 = x_at(&f, &p, p.start[i]);
        let x2 = x_at(&f, &p, p.end[i]);
        let rank = n - i;
        push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, f.pl - 110);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, cy + 4);
        push_b(&mut f.buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"12\" font-weight=\"700\" fill=\"#6366F1\">#");
        push_i(&mut f.buf, rank as i32);
        push_b(&mut f.buf, b"</text>");
        push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, f.pl - 6);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, cy + 3);
        push_b(&mut f.buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">");
        escape_xml(&mut f.buf, truncate(&p.labels[i], 16));
        push_b(&mut f.buf, b"</text>");
        push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, x1);
        push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, x2);
        push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" stroke=\"#475569\" stroke-width=\"3.2\" stroke-linecap=\"round\"/>");
        data_dot(&mut f, &p, i, 0, cfg.series_names.0, x1, cy, 6, p.c1);
        data_dot(&mut f, &p, i, 1, cfg.series_names.1, x2, cy, 6, p.c2);
    }
    finalize(f, cfg, &p)
}
