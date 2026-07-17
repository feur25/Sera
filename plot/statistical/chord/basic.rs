use super::common::{arc_path, compute_layout, ribbon_path};
use super::config::ChordConfig;
use crate::html::hover::{html_id, html_prefix, html_suffix, slots_to_json};
use crate::plot::statistical::common::{
    escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_open,
};

#[crate::chart_demo(
    "labels=[\"A\",\"B\",\"C\",\"D\"], matrix=[[0,10,5,8],[10,0,7,3],[5,7,0,12],[8,3,12,0]]"
)]
pub fn render(cfg: &ChordConfig) -> String {
    render_impl(cfg, false, false, false, false)
}

pub fn render_gradient(cfg: &ChordConfig) -> String {
    render_impl(cfg, true, false, false, false)
}

pub fn render_ribbon(cfg: &ChordConfig) -> String {
    render_impl(cfg, false, true, false, false)
}

pub fn render_arc(cfg: &ChordConfig) -> String {
    render_impl(cfg, false, false, true, false)
}

pub fn render_labeled(cfg: &ChordConfig) -> String {
    render_impl(cfg, false, false, false, true)
}

fn render_impl(cfg: &ChordConfig, gradient: bool, wide: bool, arc_only: bool, labeled: bool) -> String {
    let n = cfg.labels.len().min(cfg.matrix.len());
    if n == 0 {
        return String::new();
    }

    let Some(lay) = compute_layout(cfg) else {
        return String::new();
    };

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n * n * 200 + 4096);
    html_prefix(&mut buf, cfg.title, hid);

    svg_open(&mut buf, cfg.width, cfg.height);

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, lay.cx);
        push_b(&mut buf, b"\" y=\"18\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"14\" font-weight=\"700\" fill=\"#1a202c\" class=\"sp-ttl\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    if gradient {
        push_b(&mut buf, b"<defs>");
        for i in 0..n {
            for j in i..n {
                if j == i {
                    continue;
                }
                let ci = hex6(palette_color(cfg.palette, i));
                let cj = hex6(palette_color(cfg.palette, j));
                push_b(&mut buf, b"<linearGradient id=\"cg");
                push_i(&mut buf, i as i32);
                push_b(&mut buf, b"_");
                push_i(&mut buf, j as i32);
                push_b(&mut buf, b"\" gradientUnits=\"userSpaceOnUse\" x1=\"");
                push_f2(&mut buf, lay.cx + lay.r_in * lay.arcs[i].0.cos());
                push_b(&mut buf, b"\" y1=\"");
                push_f2(&mut buf, lay.cy + lay.r_in * lay.arcs[i].0.sin());
                push_b(&mut buf, b"\" x2=\"");
                push_f2(&mut buf, lay.cx + lay.r_in * lay.arcs[j].0.cos());
                push_b(&mut buf, b"\" y2=\"");
                push_f2(&mut buf, lay.cy + lay.r_in * lay.arcs[j].0.sin());
                push_b(&mut buf, b"\"><stop offset=\"0%\" stop-color=\"#");
                buf.extend_from_slice(&ci);
                push_b(&mut buf, b"\"/><stop offset=\"100%\" stop-color=\"#");
                buf.extend_from_slice(&cj);
                push_b(&mut buf, b"\"/></linearGradient>");
            }
        }
        push_b(&mut buf, b"</defs>");
    }

    if !arc_only {
        for i in 0..n {
            for j in 0..n {
                let v = cfg
                    .matrix
                    .get(i)
                    .and_then(|r| r.get(j))
                    .copied()
                    .unwrap_or(0.0);
                if v <= 0.0 || j < i {
                    continue;
                }
                let (sa1, sa2) = lay.sub_arcs[i][j];
                let (ta1, ta2) = lay.sub_arcs[j][i];
                let fill = if gradient {
                    format!("url(#cg{}_{}", i.min(j), i.max(j)) + ")"
                } else {
                    let c = hex6(palette_color(cfg.palette, i));
                    format!("#{}", std::str::from_utf8(&c).unwrap_or("636efa"))
                };
                push_b(&mut buf, b"<path fill=\"");
                buf.extend_from_slice(fill.as_bytes());
                push_b(&mut buf, b"\" fill-opacity=\"");
                buf.extend_from_slice(if wide { b"0.6" } else { b"0.4" });
                push_b(&mut buf, b"\" d=\"");
                ribbon_path(&mut buf, lay.cx, lay.cy, lay.r_in, sa1, sa2, ta1, ta2);
                push_b(&mut buf, b"\"/>");

                if labeled && i != j {
                    let sam = (sa1 + sa2) / 2.0;
                    let tam = (ta1 + ta2) / 2.0;
                    let sx = lay.cx + lay.r_in * sam.cos();
                    let sy = lay.cy + lay.r_in * sam.sin();
                    let tx = lay.cx + lay.r_in * tam.cos();
                    let ty = lay.cy + lay.r_in * tam.sin();
                    push_b(&mut buf, b"<text x=\"");
                    push_f2(&mut buf, (sx + tx) / 2.0);
                    push_b(&mut buf, b"\" y=\"");
                    push_f2(&mut buf, (sy + ty) / 2.0);
                    push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#1e293b\" font-weight=\"600\" paint-order=\"stroke\" stroke=\"#fff\" stroke-width=\"3\">");
                    push_f2(&mut buf, v);
                    push_b(&mut buf, b"</text>");
                }
            }
        }
    }

    for i in 0..n {
        let (a1, a2) = lay.arcs[i];
        let c = hex6(palette_color(cfg.palette, i));
        push_b(&mut buf, b"<path fill=\"#");
        buf.extend_from_slice(&c);
        push_b(&mut buf, b"\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\" d=\"");
        arc_path(&mut buf, lay.cx, lay.cy, lay.r_in, lay.r_out, a1, a2);
        push_b(&mut buf, b"\"/>");

        if cfg.show_labels {
            let am = (a1 + a2) / 2.0;
            let lx = lay.cx + lay.r_label * am.cos();
            let ly = lay.cy + lay.r_label * am.sin();
            let anchor = if am.cos() >= 0.0 { "start" } else { "end" };
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, lx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, ly + 4.0);
            push_b(&mut buf, b"\" text-anchor=\"");
            buf.extend_from_slice(anchor.as_bytes());
            push_b(
                &mut buf,
                b"\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">",
            );
            escape_xml(&mut buf, &cfg.labels[i]);
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}
