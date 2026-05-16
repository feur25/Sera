use super::common::{prepare, open_svg, finalize, axes, val_to_y, xlabel, value_text, bar_color, bar_x, data_attrs};
use super::config::WaterfallConfig;
use crate::plot::statistical::common::{push_b, push_i, hex6};

pub const DEMO_KWARGS: &str = "labels=[\"Start\",\"Q1\",\"Q2\",\"Q3\",\"End\"], values=[100,30,-15,40,155]";
pub fn render(cfg: &WaterfallConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let l = &p.layout;
    let mut b = Vec::<u8>::with_capacity(p.n * 220 + 2048);
    open_svg(&mut b, cfg);
    axes(&mut b, cfg, &p);
    for i in 0..p.n {
        let bx = bar_x(l, l.pad_l, i);
        let y_s = val_to_y(l, p.starts[i]);
        let y_e = val_to_y(l, p.ends[i]);
        let top = y_s.min(y_e);
        let h = (y_s - y_e).abs().max(2);
        let hx = hex6(bar_color(&p, i));
        push_b(&mut b, b"<rect");
        data_attrs(&mut b, &p, i);
        push_b(&mut b, b" x=\""); push_i(&mut b, bx);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, top);
        push_b(&mut b, b"\" width=\""); push_i(&mut b, l.bar_w);
        push_b(&mut b, b"\" height=\""); push_i(&mut b, h);
        push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" rx=\"2\" opacity=\"0.9\"/>");
        if i + 1 < p.n && !p.is_total[i + 1] {
            let next_bx = bar_x(l, l.pad_l, i + 1);
            let cy = val_to_y(l, p.ends[i]);
            push_b(&mut b, b"<line x1=\""); push_i(&mut b, bx + l.bar_w);
            push_b(&mut b, b"\" y1=\""); push_i(&mut b, cy);
            push_b(&mut b, b"\" x2=\""); push_i(&mut b, next_bx);
            push_b(&mut b, b"\" y2=\""); push_i(&mut b, cy);
            push_b(&mut b, b"\" stroke=\"#cbd5e1\" stroke-width=\"0.8\" stroke-dasharray=\"2,2\"/>");
        }
        xlabel(&mut b, bx + l.bar_w / 2, l.pad_t + l.plot_h + 14, &p.labels[i]);
        if cfg.show_text && h > 10 {
            let v = if p.is_total[i] { p.ends[i] } else { p.values[i] };
            value_text(&mut b, bx + l.bar_w / 2, top - 3, v);
        }
    }
    finalize(b, cfg)
}


