use super::common::{
    axes, bar_color, bar_x, data_attrs, finalize, open_svg, prepare, val_to_y, value_text, xlabel,
};
use super::config::WaterfallConfig;
use crate::plot::statistical::common::{hex6, push_b, push_i};

#[crate::chart_demo(
    "labels=[\"Start\",\"Q1\",\"Q2\",\"Q3\",\"Q4\",\"End\"], values=[100,30,-15,40,12,167], variant=\"trend\""
)]

pub fn render(cfg: &WaterfallConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let l = &p.layout;
    let mut b = Vec::<u8>::with_capacity(p.n * 260 + 2048);
    open_svg(&mut b, cfg);
    axes(&mut b, cfg, &p);

    let mut pts: Vec<(i32, i32)> = Vec::with_capacity(p.n);
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
        push_b(&mut b, b"\" rx=\"2\" opacity=\"0.55\"/>");

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
                b"\" stroke=\"#475569\" stroke-width=\"1.3\" stroke-dasharray=\"4,3\"/>",
            );
        }
        xlabel(
            &mut b,
            bx + l.bar_w / 2,
            l.pad_t + l.plot_h + 14,
            &p.labels[i],
        );
        if cfg.show_text && h > 10 {
            let v = if p.is_total[i] {
                p.ends[i]
            } else {
                p.values[i]
            };
            value_text(&mut b, bx + l.bar_w / 2, top - 3, v);
        }
        pts.push((bx + l.bar_w / 2, val_to_y(l, p.ends[i])));
    }

    if pts.len() > 1 {
        let poly: String = pts
            .iter()
            .map(|(x, y)| format!("{},{}", x, y))
            .collect::<Vec<_>>()
            .join(" ");
        push_b(&mut b, b"<polyline points=\"");
        b.extend_from_slice(poly.as_bytes());
        push_b(
            &mut b,
            b"\" fill=\"none\" stroke=\"#ffffff\" stroke-width=\"4\" stroke-linecap=\"round\" stroke-linejoin=\"round\" opacity=\"0.9\"/>",
        );
        push_b(&mut b, b"<polyline points=\"");
        b.extend_from_slice(poly.as_bytes());
        push_b(
            &mut b,
            b"\" fill=\"none\" stroke=\"#f59e0b\" stroke-width=\"2.2\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>",
        );
        for (x, y) in &pts {
            push_b(&mut b, b"<circle cx=\"");
            push_i(&mut b, *x);
            push_b(&mut b, b"\" cy=\"");
            push_i(&mut b, *y);
            push_b(&mut b, b"\" r=\"3.4\" fill=\"#f59e0b\" stroke=\"#fff\" stroke-width=\"1.2\"/>");
        }
    }

    finalize(b, cfg)
}
