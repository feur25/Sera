use super::common::{prepare, open_svg, finalize, axes, val_to_y, xlabel, value_text, bar_color, bar_x, data_attrs};
use super::config::WaterfallConfig;
use crate::plot::statistical::common::{push_b, push_i, hex6};

pub fn render(cfg: &WaterfallConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let l = &p.layout;
    let mut b = Vec::<u8>::with_capacity(p.n * 220 + 2048);
    open_svg(&mut b, cfg);
    axes(&mut b, cfg, &p);
    for i in 0..p.n {
        let bx = bar_x(l, l.pad_l, i);
        let cx = bx + l.bar_w / 2;
        let y_s = val_to_y(l, p.starts[i]);
        let y_e = val_to_y(l, p.ends[i]);
        let hx = hex6(bar_color(&p, i));
        push_b(&mut b, b"<line");
        data_attrs(&mut b, &p, i);
        push_b(&mut b, b" x1=\""); push_i(&mut b, cx);
        push_b(&mut b, b"\" y1=\""); push_i(&mut b, y_s);
        push_b(&mut b, b"\" x2=\""); push_i(&mut b, cx);
        push_b(&mut b, b"\" y2=\""); push_i(&mut b, y_e);
        push_b(&mut b, b"\" stroke=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"3\" stroke-linecap=\"round\"/>");
        push_b(&mut b, b"<circle cx=\""); push_i(&mut b, cx);
        push_b(&mut b, b"\" cy=\""); push_i(&mut b, y_e);
        push_b(&mut b, b"\" r=\"6\" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke=\"#fff\" stroke-width=\"1.5\"/>");
        if i + 1 < p.n && !p.is_total[i + 1] {
            let next_cx = bar_x(l, l.pad_l, i + 1) + l.bar_w / 2;
            push_b(&mut b, b"<line x1=\""); push_i(&mut b, cx);
            push_b(&mut b, b"\" y1=\""); push_i(&mut b, y_e);
            push_b(&mut b, b"\" x2=\""); push_i(&mut b, next_cx);
            push_b(&mut b, b"\" y2=\""); push_i(&mut b, y_e);
            push_b(&mut b, b"\" stroke=\"#cbd5e1\" stroke-width=\"0.8\" stroke-dasharray=\"2,2\"/>");
        }
        xlabel(&mut b, cx, l.pad_t + l.plot_h + 14, &p.labels[i]);
        if cfg.show_text {
            let v = if p.is_total[i] { p.ends[i] } else { p.values[i] };
            value_text(&mut b, cx, y_e.min(y_s) - 10, v);
        }
    }
    finalize(b, cfg)
}


