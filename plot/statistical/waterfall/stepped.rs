use super::common::{prepare, open_svg, finalize, axes, val_to_y, xlabel, value_text, bar_color, data_attrs};
use super::config::WaterfallConfig;
use crate::plot::statistical::common::{push_b, push_i, hex6};

#[crate::chart_demo("labels=[\"Start\",\"Q1\",\"Q2\",\"Q3\",\"End\"], values=[100,30,-15,40,155]")]

pub fn render(cfg: &WaterfallConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let l = &p.layout;
    let mut b = Vec::<u8>::with_capacity(p.n * 220 + 2048);
    open_svg(&mut b, cfg);
    axes(&mut b, cfg, &p);
    let bw = l.bar_step.max(2);
    for i in 0..p.n {
        let bx = l.pad_l + i as i32 * l.bar_step;
        let y_s = val_to_y(l, p.starts[i]);
        let y_e = val_to_y(l, p.ends[i]);
        let top = y_s.min(y_e);
        let h = (y_s - y_e).abs().max(2);
        let hx = hex6(bar_color(&p, i));
        push_b(&mut b, b"<rect");
        data_attrs(&mut b, &p, i);
        push_b(&mut b, b" x=\""); push_i(&mut b, bx);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, top);
        push_b(&mut b, b"\" width=\""); push_i(&mut b, bw);
        push_b(&mut b, b"\" height=\""); push_i(&mut b, h);
        push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke=\"#fff\" stroke-width=\"1\" opacity=\"0.92\"/>");
        xlabel(&mut b, bx + bw / 2, l.pad_t + l.plot_h + 14, &p.labels[i]);
        if cfg.show_text && h > 10 {
            let v = if p.is_total[i] { p.ends[i] } else { p.values[i] };
            value_text(&mut b, bx + bw / 2, top - 3, v);
        }
    }
    finalize(b, cfg)
}

