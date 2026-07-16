use super::config::ArcDiagramConfig;
use crate::html::hover::{html_id, html_prefix, html_suffix, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i};

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\",\"D\",\"E\",\"F\"], edges_i=[0,0,1,2,3,4], edges_j=[1,2,3,4,5,0], edges_w=[3,5,2,4,6,1]")]
pub fn render(cfg: &ArcDiagramConfig) -> String {
    render_impl(cfg, false, false, false)
}
pub fn render_bilateral(cfg: &ArcDiagramConfig)  -> String { render_impl(cfg, true,  false, false) }
pub fn render_weighted(cfg: &ArcDiagramConfig)   -> String { render_impl(cfg, false, true,  false) }
pub fn render_gradient(cfg: &ArcDiagramConfig)   -> String { render_impl(cfg, false, false, true)  }
pub fn render_minimal(cfg: &ArcDiagramConfig)    -> String { render_impl(cfg, false, false, false)  }

fn render_impl(cfg: &ArcDiagramConfig, bilateral: bool, weighted: bool, gradient: bool) -> String {
    let n = cfg.labels.len();
    let e = cfg.sources.len().min(cfg.targets.len());
    if n == 0 { return String::new(); }

    let pad_l = 20i32;
    let pad_r = 20i32;
    let pad_t = 30i32;
    let pad_b = 60i32;
    let plot_w = cfg.width - pad_l - pad_r;
    let node_y = pad_t + (cfg.height - pad_t - pad_b) / 2;

    let step = if n > 1 { plot_w as f64 / (n as f64 - 1.0) } else { 0.0 };

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n * 100 + e * 200 + 4096);
    html_prefix(&mut buf, cfg.title, hid);
    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, cfg.width / 2);
        push_b(&mut buf, b"\" y=\"20\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"13\" font-weight=\"700\" fill=\"#1a202c\" class=\"sp-ttl\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    let max_w = cfg.weights.iter().copied().fold(1.0f64, f64::max);

    for k in 0..e {
        let s = cfg.sources[k] as usize;
        let t = cfg.targets[k] as usize;
        if s >= n || t >= n || s == t { continue; }
        let w = cfg.weights.get(k).copied().unwrap_or(1.0);
        let sw = if weighted { (w / max_w * 4.0 + 0.5).max(0.5) } else { 1.2 };

        let x1 = pad_l as f64 + s as f64 * step;
        let x2 = pad_l as f64 + t as f64 * step;
        let mid_x = (x1 + x2) / 2.0;
        let arc_h = ((x2 - x1).abs() / 2.0).min((cfg.height - pad_t - pad_b) as f64 / 2.2);
        let above = !bilateral || k % 2 == 0;
        let arc_y = if above { node_y as f64 - arc_h } else { node_y as f64 + arc_h };

        let color = if gradient {
            let cs = hex6(palette_color(cfg.palette, s));
            let ct = hex6(palette_color(cfg.palette, t));
            format!("url(#ag{})", k)
        } else {
            let c = hex6(palette_color(cfg.palette, s));
            format!("#{}", std::str::from_utf8(&c).unwrap_or("636efa"))
        };

        push_b(&mut buf, b"<path fill=\"none\" stroke=\"");
        buf.extend_from_slice(color.as_bytes());
        push_b(&mut buf, b"\" stroke-width=\"");
        push_f2(&mut buf, sw);
        push_b(&mut buf, b"\" stroke-opacity=\"0.65\" d=\"M");
        push_f2(&mut buf, x1); push_b(&mut buf, b","); push_f2(&mut buf, node_y as f64);
        push_b(&mut buf, b"Q"); push_f2(&mut buf, mid_x); push_b(&mut buf, b","); push_f2(&mut buf, arc_y);
        push_b(&mut buf, b" "); push_f2(&mut buf, x2); push_b(&mut buf, b","); push_f2(&mut buf, node_y as f64);
        push_b(&mut buf, b"\"/>");
    }

    push_b(&mut buf, b"<line x1=\"");
    push_i(&mut buf, pad_l);
    push_b(&mut buf, b"\" y1=\"");
    push_i(&mut buf, node_y);
    push_b(&mut buf, b"\" x2=\"");
    push_i(&mut buf, pad_l + plot_w);
    push_b(&mut buf, b"\" y2=\"");
    push_i(&mut buf, node_y);
    push_b(&mut buf, b"\" stroke=\"#6b7280\" stroke-width=\"1\"/>");

    for i in 0..n {
        let x = pad_l as f64 + i as f64 * step;
        let color = palette_color(cfg.palette, i);
        let hx = hex6(color);
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, x);
        push_b(&mut buf, b"\" cy=\"");
        push_i(&mut buf, node_y);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, cfg.node_r);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke=\"#fff\" stroke-width=\"1.5\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, x);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, node_y + cfg.node_r as i32 + 14);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">");
        escape_xml(&mut buf, &cfg.labels[i]);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}
