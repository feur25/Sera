use super::common::prepare;
use super::config::ParcatsConfig;
use crate::html::hover::{html_id, html_prefix, html_suffix};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_title, truncate};
use crate::plot::statistical::sankey::common::{compute_layout, sankey_link_path};

#[crate::chart_demo(
    "axes=[\"Gender\",\"Survived\",\"Class\"], category_series=[[\"Male\",\"No\",\"3rd\"],[\"Female\",\"Yes\",\"1st\"],[\"Male\",\"No\",\"2nd\"],[\"Female\",\"Yes\",\"1st\"],[\"Male\",\"Yes\",\"1st\"],[\"Female\",\"No\",\"3rd\"]]"
)]

pub fn render(cfg: &ParcatsConfig) -> String {
    render_impl(cfg, false)
}

pub fn render_gradient(cfg: &ParcatsConfig) -> String {
    render_impl(cfg, true)
}

fn render_impl(cfg: &ParcatsConfig, gradient: bool) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let n = p.labels.len();
    let e = p.sources.len();

    let pad_l = 10i32;
    let pad_t = 40i32;
    let pad_b = 12i32;
    let pad_r = 10i32;
    let plot_w = cfg.width - pad_l - pad_r;
    let plot_h = cfg.height - pad_t - pad_b;

    let layout = compute_layout(
        n,
        &p.sources,
        &p.targets,
        &p.weights,
        cfg.width,
        cfg.height,
        pad_l,
        pad_t,
        plot_w,
        plot_h,
        cfg.node_width,
        cfg.node_gap,
    );

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n * 200 + e * 400 + 8192);
    html_prefix(&mut buf, cfg.title, hid);
    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    svg_title(&mut buf, cfg.title, cfg.width / 2, 24);

    if gradient {
        push_b(&mut buf, b"<defs>");
        for k in 0..e {
            let s = p.sources[k] as usize;
            let t = p.targets[k] as usize;
            let cs = hex6(palette_color(cfg.palette, s));
            let ct = hex6(palette_color(cfg.palette, t));
            push_b(&mut buf, b"<linearGradient id=\"pcg");
            push_i(&mut buf, k as i32);
            push_b(
                &mut buf,
                b"\" x1=\"0%\" y1=\"0%\" x2=\"100%\" y2=\"0%\"><stop offset=\"0%\" stop-color=\"#",
            );
            buf.extend_from_slice(&cs);
            push_b(&mut buf, b"\" stop-opacity=\"0.55\"/><stop offset=\"100%\" stop-color=\"#");
            buf.extend_from_slice(&ct);
            push_b(&mut buf, b"\" stop-opacity=\"0.55\"/></linearGradient>");
        }
        push_b(&mut buf, b"</defs>");
    }

    let mut src_offset = vec![0.0f64; n];
    let mut tgt_offset = vec![0.0f64; n];

    for k in 0..e {
        let s = p.sources[k] as usize;
        let t = p.targets[k] as usize;
        let w = p.weights[k];
        let total_src: f64 = (0..e).filter(|&i| p.sources[i] as usize == s).map(|i| p.weights[i]).sum();
        let total_tgt: f64 = (0..e).filter(|&i| p.targets[i] as usize == t).map(|i| p.weights[i]).sum();
        let src_h = layout.h[s] * w / total_src.max(1.0);
        let tgt_h = layout.h[t] * w / total_tgt.max(1.0);

        let y0 = layout.y[s] + src_offset[s];
        let y1 = layout.y[t] + tgt_offset[t];
        src_offset[s] += src_h;
        tgt_offset[t] += tgt_h;

        let color = if gradient {
            format!("url(#pcg{})", k)
        } else {
            let c = hex6(palette_color(cfg.palette, s));
            format!("#{}", std::str::from_utf8(&c).unwrap_or("636efa"))
        };

        push_b(&mut buf, b"<path data-idx=\"");
        push_i(&mut buf, k as i32);
        push_b(&mut buf, b"\" data-y=\"");
        push_f2(&mut buf, w);
        push_b(&mut buf, b"\" fill=\"");
        buf.extend_from_slice(color.as_bytes());
        push_b(&mut buf, b"\" fill-opacity=\"0.45\" d=\"");
        sankey_link_path(&mut buf, layout.x[s], y0, layout.x[t], y1, src_h, tgt_h, cfg.node_width);
        push_b(&mut buf, b"\"/>");
    }

    for i in 0..n {
        let c = hex6(palette_color(cfg.palette, i));
        push_b(&mut buf, b"<rect x=\"");
        push_f2(&mut buf, layout.x[i]);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, layout.y[i]);
        push_b(&mut buf, b"\" width=\"");
        push_i(&mut buf, cfg.node_width);
        push_b(&mut buf, b"\" height=\"");
        push_f2(&mut buf, layout.h[i].max(4.0));
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&c);
        push_b(&mut buf, b"\" rx=\"3\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\"/>");

        if layout.h[i] > 13.0 {
            let max_chars = ((cfg.node_width as f64 - 6.0) / 6.0).max(1.0) as usize;
            let label = truncate(&p.labels[i], max_chars);
            let lx = layout.x[i] + cfg.node_width as f64 / 2.0;
            let ly = layout.y[i] + layout.h[i] / 2.0 + 3.5;
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, lx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, ly);
            push_b(
                &mut buf,
                b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#fff\" pointer-events=\"none\">",
            );
            escape_xml(&mut buf, label);
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, "[]");
    unsafe { String::from_utf8_unchecked(buf) }
}
