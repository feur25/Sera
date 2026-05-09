use super::common::{prepare, open_with_axes, finalize, val_to_y, cx_at, data_attrs, color_hex};
use super::config::CandlestickConfig;
use crate::plot::statistical::common::{push_b, push_i};

pub fn render(cfg: &CandlestickConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = Vec::<u8>::with_capacity(p.n * 80 + 4096);
    open_with_axes(&mut b, cfg, &p);
    let l = &p.layout;
    let hx = color_hex(p.up_color);
    push_b(&mut b, b"<polyline fill=\"none\" stroke=\"#"); b.extend_from_slice(&hx);
    push_b(&mut b, b"\" stroke-width=\"2.2\" stroke-linejoin=\"round\" stroke-linecap=\"round\" points=\"");
    for i in 0..p.n {
        let cx = cx_at(l, i);
        let y = val_to_y(l, p.close[i]);
        if i > 0 { push_b(&mut b, b" "); }
        push_i(&mut b, cx); push_b(&mut b, b","); push_i(&mut b, y);
    }
    push_b(&mut b, b"\"/>");
    let r = (l.body_w / 2).max(2);
    for i in 0..p.n {
        let cx = cx_at(l, i);
        let y = val_to_y(l, p.close[i]);
        push_b(&mut b, b"<circle"); data_attrs(&mut b, &p, i);
        push_b(&mut b, b" cx=\""); push_i(&mut b, cx);
        push_b(&mut b, b"\" cy=\""); push_i(&mut b, y);
        push_b(&mut b, b"\" r=\""); push_i(&mut b, r);
        push_b(&mut b, b"\" fill=\"#fff\" stroke=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"1.6\"/>");
    }
    finalize(b, cfg)
}


