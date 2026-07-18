use super::config::HiveConfig;
use crate::html::hover::{html_id, html_prefix, html_suffix, slots_to_json};
use crate::plot::statistical::common::{
    escape_xml, hex6, palette_color, push_arrowhead, push_b, push_f2, push_i,
};

#[crate::chart_demo("axes=[\"Biology\",\"Chemistry\",\"Physics\"], labels=[\"n1\",\"n2\",\"n3\",\"n4\",\"n5\",\"n6\"], categories=[\"Biology\",\"Biology\",\"Chemistry\",\"Chemistry\",\"Physics\",\"Physics\"], values=[0.3,0.7,0.2,0.9,0.5,0.8], edges_i=[0,1,2,4], edges_j=[2,3,4,5], edges_w=[1,2,1.5,0.8]")]
pub fn render(cfg: &HiveConfig) -> String {
    render_impl(cfg, false, false, false, false)
}

pub fn render_curved(cfg: &HiveConfig)   -> String { render_impl(cfg, true,  false, false, false) }
pub fn render_weighted(cfg: &HiveConfig) -> String { render_impl(cfg, false, true,  false, false) }
pub fn render_minimal(cfg: &HiveConfig)  -> String { render_impl(cfg, false, false, false, true)  }
pub fn render_directed(cfg: &HiveConfig) -> String { render_impl(cfg, false, false, true,  false) }

fn node_pos(axis_angle: f64, value: f64, inner_r: f64, outer_r: f64) -> (f64, f64) {
    let r = inner_r + value.clamp(0.0, 1.0) * (outer_r - inner_r);
    (r * axis_angle.cos(), r * axis_angle.sin())
}

fn render_impl(cfg: &HiveConfig, curved: bool, weighted: bool, directed: bool, minimal: bool) -> String {
    use std::f64::consts::PI;

    let na = cfg.axes.len();
    if na == 0 { return String::new(); }

    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let cx = w / 2.0;
    let cy = h / 2.0;

    let outer_r = cfg.outer_r.min(w.min(h) / 2.0 - 48.0).max(60.0);
    let inner_r = cfg.inner_r.min(outer_r * 0.2);

    let axis_angles: Vec<f64> = (0..na)
        .map(|i| 2.0 * PI * i as f64 / na as f64 - PI / 2.0)
        .collect();

    let mut axis_of_node = std::collections::HashMap::new();
    for (i, cat) in cfg.categories.iter().enumerate() {
        if let Some(ai) = cfg.axes.iter().position(|a| a == cat) {
            axis_of_node.insert(i, ai);
        }
    }

    let mut axis_nodes: Vec<Vec<usize>> = vec![Vec::new(); na];
    for i in 0..cfg.labels.len() {
        if let Some(&ai) = axis_of_node.get(&i) {
            axis_nodes[ai].push(i);
        }
    }

    let n_nodes = cfg.labels.len();
    let mut positions: Vec<(f64, f64)> = vec![(0.0, 0.0); n_nodes];
    for ai in 0..na {
        let mn = axis_nodes[ai].len();
        for (k, &ni) in axis_nodes[ai].iter().enumerate() {
            let v = if mn > 1 {
                cfg.values.get(ni).copied().unwrap_or((k as f64 + 1.0) / mn as f64)
            } else {
                0.5
            };
            let (px, py) = node_pos(axis_angles[ai], v, inner_r, outer_r);
            positions[ni] = (cx + px, cy + py);
        }
    }

    let e = cfg.sources.len().min(cfg.targets.len());
    let max_w = cfg.weights.iter().copied().fold(1.0f64, f64::max);

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(na * 120 + e * 200 + n_nodes * 80 + 4096);
    html_prefix(&mut buf, cfg.title, hid);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"18\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"13\" font-weight=\"700\" fill=\"#1a202c\" class=\"sp-ttl\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    for (ai, &angle) in axis_angles.iter().enumerate() {
        let (ex, ey) = node_pos(angle, 1.0, inner_r, outer_r);
        let (ix, iy) = node_pos(angle, 0.0, inner_r, inner_r);
        let color = palette_color(cfg.palette, ai);
        let hx = hex6(color);

        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, cx + ix);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, cy + iy);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, cx + ex);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, cy + ey);
        push_b(&mut buf, b"\" stroke=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke-width=\"2\" stroke-opacity=\"0.6\"/>");

        let (lx, ly) = node_pos(angle, 1.12, inner_r, outer_r);
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx + lx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy + ly + 4.0);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"11\" font-weight=\"600\" fill=\"#1e293b\">");
        escape_xml(&mut buf, cfg.axes.get(ai).map(|s| s.as_str()).unwrap_or(""));
        push_b(&mut buf, b"</text>");
    }

    for k in 0..e {
        let si = cfg.sources[k] as usize;
        let ti = cfg.targets[k] as usize;
        if si >= n_nodes || ti >= n_nodes { continue; }
        let (sx, sy) = positions[si];
        let (tx, ty) = positions[ti];
        let sw = if weighted {
            let w = cfg.weights.get(k).copied().unwrap_or(1.0);
            (w / max_w * 3.5 + 0.5).max(0.5)
        } else if minimal {
            0.6
        } else {
            1.2
        };
        let s_opacity: &[u8] = if minimal { b"0.28" } else { b"0.5" };

        let sai = axis_of_node.get(&si).copied().unwrap_or(0);
        let tai = axis_of_node.get(&ti).copied().unwrap_or(1);
        let color = palette_color(cfg.palette, sai.min(tai));
        let hx = hex6(color);

        push_b(&mut buf, b"<path fill=\"none\" stroke=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke-width=\"");
        push_f2(&mut buf, sw);
        push_b(&mut buf, b"\" stroke-opacity=\"");
        buf.extend_from_slice(s_opacity);
        push_b(&mut buf, b"\" d=\"M");
        push_f2(&mut buf, sx); push_b(&mut buf, b","); push_f2(&mut buf, sy);

        if curved {
            push_b(&mut buf, b"C"); push_f2(&mut buf, cx); push_b(&mut buf, b","); push_f2(&mut buf, cy);
            push_b(&mut buf, b" "); push_f2(&mut buf, cx); push_b(&mut buf, b","); push_f2(&mut buf, cy);
            push_b(&mut buf, b" ");
        } else {
            push_b(&mut buf, b"L");
        }
        push_f2(&mut buf, tx); push_b(&mut buf, b","); push_f2(&mut buf, ty);
        push_b(&mut buf, b"\"/>");

        if directed {
            let angle = (ty - sy).atan2(tx - sx);
            let tip_x = tx - 6.0 * angle.cos();
            let tip_y = ty - 6.0 * angle.sin();
            let ac = format!("#{}", std::str::from_utf8(&hx).unwrap_or("1e293b"));
            push_arrowhead(&mut buf, tip_x, tip_y, angle, 7.0, ac.as_bytes());
        }
    }

    for i in 0..n_nodes {
        let (px, py) = positions[i];
        let ai = axis_of_node.get(&i).copied().unwrap_or(0);
        let color = palette_color(cfg.palette, ai);
        let hx = hex6(color);
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, px);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, py);
        if minimal {
            push_b(&mut buf, b"\" r=\"2.2\" fill=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stroke=\"none\" data-idx=\"");
        } else {
            push_b(&mut buf, b"\" r=\"4\" fill=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stroke=\"#fff\" stroke-width=\"1.2\" data-idx=\"");
        }
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\"/>");
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}
