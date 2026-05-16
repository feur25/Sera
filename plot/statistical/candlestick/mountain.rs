use super::common::{prepare, open_with_axes, finalize, val_to_y, cx_at, data_attrs, color_hex};
use super::config::CandlestickConfig;
use crate::plot::statistical::common::{push_b, push_i};

pub const DEMO_KWARGS: &str = "labels=[\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\"], open=[100,102,105,103,108], high=[105,107,109,110,114], low=[99,101,103,102,107], close=[102,105,103,108,112]";
pub fn render(cfg: &CandlestickConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = Vec::<u8>::with_capacity(p.n * 90 + 4096);
    open_with_axes(&mut b, cfg, &p);
    let l = &p.layout;
    let hx = color_hex(p.up_color);
    let y_base = l.pad_t + l.plot_h;
    push_b(&mut b, b"<defs><linearGradient id=\"sp-cm-grad\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\">");
    push_b(&mut b, b"<stop offset=\"0%\" stop-color=\"#"); b.extend_from_slice(&hx); push_b(&mut b, b"\" stop-opacity=\"0.55\"/>");
    push_b(&mut b, b"<stop offset=\"100%\" stop-color=\"#"); b.extend_from_slice(&hx); push_b(&mut b, b"\" stop-opacity=\"0.05\"/>");
    push_b(&mut b, b"</linearGradient></defs>");
    push_b(&mut b, b"<polygon fill=\"url(#sp-cm-grad)\" stroke=\"none\" points=\"");
    push_i(&mut b, cx_at(l, 0)); push_b(&mut b, b","); push_i(&mut b, y_base);
    for i in 0..p.n {
        push_b(&mut b, b" "); push_i(&mut b, cx_at(l, i));
        push_b(&mut b, b","); push_i(&mut b, val_to_y(l, p.close[i]));
    }
    push_b(&mut b, b" "); push_i(&mut b, cx_at(l, p.n - 1)); push_b(&mut b, b","); push_i(&mut b, y_base);
    push_b(&mut b, b"\"/>");
    push_b(&mut b, b"<polyline fill=\"none\" stroke=\"#"); b.extend_from_slice(&hx);
    push_b(&mut b, b"\" stroke-width=\"2\" stroke-linejoin=\"round\" stroke-linecap=\"round\" points=\"");
    for i in 0..p.n {
        let cx = cx_at(l, i);
        let y = val_to_y(l, p.close[i]);
        if i > 0 { push_b(&mut b, b" "); }
        push_i(&mut b, cx); push_b(&mut b, b","); push_i(&mut b, y);
    }
    push_b(&mut b, b"\"/>");
    for i in 0..p.n {
        let cx = cx_at(l, i);
        let y = val_to_y(l, p.close[i]);
        push_b(&mut b, b"<circle"); data_attrs(&mut b, &p, i);
        push_b(&mut b, b" cx=\""); push_i(&mut b, cx);
        push_b(&mut b, b"\" cy=\""); push_i(&mut b, y);
        push_b(&mut b, b"\" r=\"2\" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\"/>");
    }
    finalize(b, cfg)
}


