use super::common::{compute_layout, compute_layout_sorted, sankey_link_path};
use super::config::SankeyConfig;
use crate::html::hover::{html_id, html_prefix, html_suffix, slots_to_json};
use crate::plot::statistical::common::{
    escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_title,
};

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\",\"D\",\"E\"], edges_i=[0,0,1,2], edges_j=[2,3,4,4], edges_w=[10,5,8,7]")]
pub fn render(cfg: &SankeyConfig) -> String {
    render_impl(cfg, false, false)
}

pub fn render_minimal(cfg: &SankeyConfig) -> String {
    render_impl(cfg, true, false)
}

pub fn render_sorted(cfg: &SankeyConfig) -> String {
    render_impl(cfg, false, true)
}

fn render_impl(cfg: &SankeyConfig, minimal: bool, sorted: bool) -> String {
    let n = cfg.labels.len();
    let e = cfg.sources.len().min(cfg.targets.len()).min(cfg.weights.len());
    if n == 0 || e == 0 {
        return String::new();
    }

    let pad_l = 8i32;
    let pad_t = 40i32;
    let pad_b = 12i32;
    let pad_r = 8i32;
    let plot_w = cfg.width - pad_l - pad_r;
    let plot_h = cfg.height - pad_t - pad_b;

    let layout_fn = if sorted { compute_layout_sorted } else { compute_layout };
    let layout = layout_fn(
        n,
        cfg.sources,
        cfg.targets,
        cfg.weights,
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

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    svg_title(&mut buf, cfg.title, cfg.width / 2, 24);

    let mut src_offset = vec![0.0f64; n];
    let mut tgt_offset = vec![0.0f64; n];

    for k in 0..e {
        let s = cfg.sources[k] as usize;
        let t = cfg.targets[k] as usize;
        if s >= n || t >= n { continue; }
        let w = cfg.weights[k];
        let total_src: f64 = (0..e).filter(|&i| cfg.sources[i] as usize == s).map(|i| cfg.weights[i]).sum();
        let total_tgt: f64 = (0..e).filter(|&i| cfg.targets[i] as usize == t).map(|i| cfg.weights[i]).sum();
        let src_h = layout.h[s] * w / total_src.max(1.0);
        let tgt_h = layout.h[t] * w / total_tgt.max(1.0);

        let y0 = layout.y[s] + src_offset[s];
        let y1 = layout.y[t] + tgt_offset[t];

        src_offset[s] += src_h;
        tgt_offset[t] += tgt_h;

        let c = hex6(palette_color(cfg.palette, s));
        let color = format!("#{}", std::str::from_utf8(&c).unwrap_or("636efa"));

        push_b(&mut buf, b"<path fill=\"");
        buf.extend_from_slice(color.as_bytes());
        if minimal {
            push_b(&mut buf, b"\" fill-opacity=\"0.2\" stroke=\"");
            buf.extend_from_slice(color.as_bytes());
            push_b(&mut buf, b"\" stroke-width=\"1.5\" d=\"");
        } else {
            push_b(&mut buf, b"\" fill-opacity=\"0.45\" d=\"");
        }
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
        push_b(&mut buf, b"\" rx=\"2\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\"/>");

        let lx = layout.x[i] + cfg.node_width as f64 + 4.0;
        let ly = layout.y[i] + layout.h[i] / 2.0 + 4.0;
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, lx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">");
        escape_xml(&mut buf, &cfg.labels[i]);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}