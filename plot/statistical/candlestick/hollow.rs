use super::common::{prepare, open_with_axes, finalize, val_to_y, cx_at, data_attrs, color_hex};
use super::config::CandlestickConfig;
use crate::plot::statistical::common::{push_b, push_i};

pub const DEMO_KWARGS: &str = "labels=[\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\"], open=[100,102,105,103,108], high=[105,107,109,110,114], low=[99,101,103,102,107], close=[102,105,103,108,112]";
pub fn render(cfg: &CandlestickConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = Vec::<u8>::with_capacity(p.n * 240 + 4096);
    open_with_axes(&mut b, cfg, &p);
    let l = &p.layout;
    let bw = l.body_w;
    for i in 0..p.n {
        let cx = cx_at(l, i);
        let y_high = val_to_y(l, p.high[i]);
        let y_low = val_to_y(l, p.low[i]);
        let y_open = val_to_y(l, p.open[i]);
        let y_close = val_to_y(l, p.close[i]);
        let up = p.close[i] >= p.open[i];
        let col = if up { p.up_color } else { p.dn_color };
        let hx = color_hex(col);
        let (top, bot) = if y_open < y_close { (y_open, y_close) } else { (y_close, y_open) };
        let bh = (bot - top).max(1);
        push_b(&mut b, b"<line"); data_attrs(&mut b, &p, i);
        push_b(&mut b, b" x1=\""); push_i(&mut b, cx);
        push_b(&mut b, b"\" y1=\""); push_i(&mut b, y_high);
        push_b(&mut b, b"\" x2=\""); push_i(&mut b, cx);
        push_b(&mut b, b"\" y2=\""); push_i(&mut b, y_low);
        push_b(&mut b, b"\" stroke=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"1.4\"/>");
        push_b(&mut b, b"<rect"); data_attrs(&mut b, &p, i);
        push_b(&mut b, b" x=\""); push_i(&mut b, cx - bw / 2);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, top);
        push_b(&mut b, b"\" width=\""); push_i(&mut b, bw);
        push_b(&mut b, b"\" height=\""); push_i(&mut b, bh);
        if up {
            push_b(&mut b, b"\" fill=\"#ffffff\" stroke=\"#"); b.extend_from_slice(&hx);
            push_b(&mut b, b"\" stroke-width=\"1.5\"/>");
        } else {
            push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx);
            push_b(&mut b, b"\"/>");
        }
    }
    finalize(b, cfg)
}


