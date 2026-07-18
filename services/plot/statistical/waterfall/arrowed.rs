use super::common::{
    axes, bar_color, bar_x, data_attrs, finalize, open_svg, prepare, val_to_y, value_text, xlabel,
};
use super::config::WaterfallConfig;
use crate::plot::statistical::common::{hex6, push_b, push_i};

#[crate::chart_demo("labels=[\"Start\",\"Q1\",\"Q2\",\"Q3\",\"End\"], values=[100,30,-15,40,155]")]

pub fn render(cfg: &WaterfallConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let l = &p.layout;
    let mut b = Vec::<u8>::with_capacity(p.n * 260 + 2048);
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
        push_b(&mut b, b" x=\"");
        push_i(&mut b, bx);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, top);
        push_b(&mut b, b"\" width=\"");
        push_i(&mut b, l.bar_w);
        push_b(&mut b, b"\" height=\"");
        push_i(&mut b, h);
        push_b(&mut b, b"\" fill=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" rx=\"2\" opacity=\"0.88\"/>");
        let cx = bx + l.bar_w / 2;
        let up = !p.is_total[i] && p.values[i] >= 0.0;
        let tip_y = if up { top - 8 } else { top + h + 8 };
        let base_y = if up { top } else { top + h };
        let half = (l.bar_w / 2).max(4);
        push_b(&mut b, b"<polygon points=\"");
        push_i(&mut b, cx);
        b.push(b',');
        push_i(&mut b, tip_y);
        b.push(b' ');
        push_i(&mut b, cx - half);
        b.push(b',');
        push_i(&mut b, base_y);
        b.push(b' ');
        push_i(&mut b, cx + half);
        b.push(b',');
        push_i(&mut b, base_y);
        push_b(&mut b, b"\" fill=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\"/>");
        if i + 1 < p.n && !p.is_total[i + 1] {
            let next_bx = bar_x(l, l.pad_l, i + 1);
            let cy = val_to_y(l, p.ends[i]);
            push_b(&mut b, b"<line x1=\"");
            push_i(&mut b, bx + l.bar_w);
            push_b(&mut b, b"\" y1=\"");
            push_i(&mut b, cy);
            push_b(&mut b, b"\" x2=\"");
            push_i(&mut b, next_bx);
            push_b(&mut b, b"\" y2=\"");
            push_i(&mut b, cy);
            push_b(
                &mut b,
                b"\" stroke=\"#6b7280\" stroke-width=\"0.8\" stroke-dasharray=\"2,2\"/>",
            );
        }
        xlabel(&mut b, cx, l.pad_t + l.plot_h + 14, &p.labels[i]);
        if cfg.show_text && h > 10 {
            let v = if p.is_total[i] {
                p.ends[i]
            } else {
                p.values[i]
            };
            value_text(&mut b, cx, top - 14, v);
        }
    }
    finalize(b, cfg)
}
