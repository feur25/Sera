use super::common::build_circles;
use super::config::CirclePackConfig;
use crate::html::hover::{html_id, html_prefix, html_suffix, slots_to_json};
use crate::plot::statistical::common::{
    escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_open,
};

#[crate::chart_demo("labels=[\"Root\",\"A\",\"B\",\"C\",\"A1\",\"A2\",\"B1\"], parents=[\"\",\"Root\",\"Root\",\"Root\",\"A\",\"A\",\"B\"], values=[0,40,30,20,20,20,30]")]
pub fn render(cfg: &CirclePackConfig) -> String {
    render_impl(cfg, false, false)
}

pub fn render_outlined(cfg: &CirclePackConfig) -> String {
    render_impl(cfg, false, true)
}

pub fn render_gradient(cfg: &CirclePackConfig) -> String {
    render_impl(cfg, true, false)
}

fn render_impl(cfg: &CirclePackConfig, gradient: bool, outlined: bool) -> String {
    let n = cfg
        .labels
        .len()
        .min(cfg.parents.len())
        .min(cfg.values.len());
    if n == 0 {
        return String::new();
    }

    let cx = cfg.width as f64 / 2.0;
    let cy = cfg.height as f64 / 2.0;
    let r_max = (cfg.width.min(cfg.height) as f64 / 2.0 - 20.0).max(30.0);

    let circles = build_circles(
        cfg.labels,
        cfg.parents,
        cfg.values,
        cx,
        cy,
        r_max,
        cfg.padding,
    );

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n * 200 + 4096);
    html_prefix(&mut buf, cfg.title, hid);

    svg_open(&mut buf, cfg.width, cfg.height);

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"18\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"14\" font-weight=\"700\" fill=\"#1a202c\" class=\"sp-ttl\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    if gradient {
        push_b(&mut buf, b"<defs>");
        for (i, c) in circles.iter().enumerate() {
            let color = palette_color(cfg.palette, c.color_idx);
            let hx = hex6(color);
            let (op0, op1): (&[u8], &[u8]) = match c.depth {
                0 => (b"0.14", b"0.05"),
                1 => (b"0.42", b"0.18"),
                _ => (b"0.82", b"0.48"),
            };
            push_b(&mut buf, b"<radialGradient id=\"cpg");
            push_i(&mut buf, i as i32);
            push_b(&mut buf, b"\"><stop offset=\"0%\" stop-color=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stop-opacity=\"");
            buf.extend_from_slice(op0);
            push_b(&mut buf, b"\"/><stop offset=\"100%\" stop-color=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stop-opacity=\"");
            buf.extend_from_slice(op1);
            push_b(&mut buf, b"\"/></radialGradient>");
        }
        push_b(&mut buf, b"</defs>");
    }

    let mut sorted_idx: Vec<usize> = (0..circles.len()).collect();
    sorted_idx.sort_by(|&a, &b| {
        circles[b]
            .r
            .partial_cmp(&circles[a].r)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for &i in &sorted_idx {
        let c = &circles[i];
        let color = palette_color(cfg.palette, c.color_idx);
        let hx = hex6(color);

        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, c.x);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, c.y);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, c.r.max(2.0));
        push_b(&mut buf, b"\" data-idx=\"");
        push_i(&mut buf, i as i32);
        if outlined {
            push_b(&mut buf, b"\" fill=\"none\" stroke=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stroke-width=\"2\"");
        } else if gradient {
            push_b(&mut buf, b"\" fill=\"url(#cpg");
            push_i(&mut buf, i as i32);
            push_b(&mut buf, b")\"");
        } else {
            push_b(&mut buf, b"\" fill=\"#");
            buf.extend_from_slice(&hx);
            let opacity: &[u8] = match c.depth {
                0 => b"\" fill-opacity=\"0.25\"",
                1 => b"\" fill-opacity=\"0.55\"",
                _ => b"\" fill-opacity=\"0.80\"",
            };
            buf.extend_from_slice(opacity);
        }
        push_b(&mut buf, b"/>");

        if cfg.show_labels && c.r > 12.0 {
            let txt_fill: &[u8] = if gradient && c.depth <= 1 {
                b"rgba(255,255,255,0.92)"
            } else {
                b"#1e293b"
            };
            let fw: &[u8] = if c.depth <= 1 { b"700" } else { b"400" };
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, c.x);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, c.y + 4.0);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" fill=\"");
            buf.extend_from_slice(txt_fill);
            push_b(&mut buf, b"\" font-weight=\"");
            buf.extend_from_slice(fw);
            push_b(&mut buf, b"\" font-size=\"");
            push_f2(&mut buf, (c.r * 0.28).min(13.0).max(8.0));
            push_b(&mut buf, b"\">");
            escape_xml(&mut buf, &c.label);
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}
