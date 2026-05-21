use super::common::{prepare, open_with_axes, finalize, val_to_y, cx_at, data_attrs, color_hex};
use super::config::CandlestickConfig;
use crate::plot::statistical::common::{push_b, push_i};

#[crate::chart_demo("labels=[\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\"], open=[100,102,105,103,108], high=[105,107,109,110,114], low=[99,101,103,102,107], close=[102,105,103,108,112]")]

pub fn render(cfg: &CandlestickConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = Vec::<u8>::with_capacity(p.n * 160 + 4096);
    open_with_axes(&mut b, cfg, &p);
    let l = &p.layout;
    let bw = l.body_w;
    let hx = color_hex(p.up_color);
    for i in 0..p.n {
        let cx = cx_at(l, i);
        let y_high = val_to_y(l, p.high[i]);
        let y_low = val_to_y(l, p.low[i]);
        let h = (y_low - y_high).max(2);
        push_b(&mut b, b"<rect"); data_attrs(&mut b, &p, i);
        push_b(&mut b, b" x=\""); push_i(&mut b, cx - bw / 2);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, y_high);
        push_b(&mut b, b"\" width=\""); push_i(&mut b, bw);
        push_b(&mut b, b"\" height=\""); push_i(&mut b, h);
        push_b(&mut b, b"\" rx=\"3\" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" fill-opacity=\"0.65\"/>");
    }
    finalize(b, cfg)
}

