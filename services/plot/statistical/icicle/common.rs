use super::config::IcicleConfig;
use crate::html::hover::{build_chart_html, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i, truncate};
use std::collections::HashMap;

pub const ICICLE_PALETTE: &[u32] = &[
    0x636EFA, 0xEF553B, 0x00CC96, 0xAB63FA, 0xFFA15A, 0x19D3F3, 0xFF6692, 0xB6E880, 0xFF97FF,
    0xFECB52, 0x818CF8, 0xFB7185, 0x34D399, 0xFBBF24, 0xA78BFA, 0x22D3EE, 0xF472B6, 0xA3E635,
    0xF87171, 0x2DD4BF,
];

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

pub struct Layout {
    pub plot_x: i32,
    pub plot_y: i32,
    pub plot_w: i32,
    pub plot_h: i32,
    pub band_h: f64,
    pub max_depth: usize,
}

pub struct Prepared {
    pub n: usize,
    pub labels: Vec<String>,
    pub values_eff: Vec<f64>,
    pub bfs_order: Vec<usize>,
    pub depth: Vec<usize>,
    pub xspan: Vec<(f64, f64)>,
    pub cidx: Vec<usize>,
    pub roots: Vec<usize>,
    pub layout: Layout,
    pub palette: Vec<u32>,
    pub grand_total: f64,
}

pub fn prepare(cfg: &IcicleConfig) -> Option<Prepared> {
    let n = cfg
        .labels
        .len()
        .min(cfg.values.len())
        .min(cfg.parents.len());
    if n == 0 {
        return None;
    }
    let labels: Vec<String> = cfg.labels[..n].to_vec();

    let mut label_idx: HashMap<&str, usize> = HashMap::with_capacity(n);
    for i in 0..n {
        label_idx.insert(labels[i].as_str(), i);
    }

    let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut roots: Vec<usize> = Vec::new();
    for i in 0..n {
        let par = cfg.parents[i].as_str();
        if par.is_empty() {
            roots.push(i);
        } else if let Some(&pi) = label_idx.get(par) {
            children[pi].push(i);
        }
    }
    if roots.is_empty() {
        return None;
    }

    let mut sv: Vec<f64> = cfg.values[..n].to_vec();
    let mut stack: Vec<(usize, bool)> = roots.iter().map(|&r| (r, false)).collect();
    while let Some((i, visited)) = stack.pop() {
        if visited {
            let cs: f64 = children[i].iter().map(|&c| sv[c]).sum();
            if cs > sv[i] {
                sv[i] = cs;
            }
        } else {
            stack.push((i, true));
            for &c in children[i].iter().rev() {
                stack.push((c, false));
            }
        }
    }

    let mut depth = vec![0usize; n];
    let mut bfs_order: Vec<usize> = Vec::with_capacity(n);
    bfs_order.extend_from_slice(&roots);
    let mut head = 0usize;
    while head < bfs_order.len() {
        let i = bfs_order[head];
        head += 1;
        for &c in &children[i] {
            depth[c] = depth[i] + 1;
            bfs_order.push(c);
        }
    }
    let max_depth = bfs_order.iter().map(|&i| depth[i]).max().unwrap_or(0);

    let mut xspan: Vec<(f64, f64)> = vec![(0.0, 0.0); n];
    let root_total: f64 = roots.iter().map(|&r| sv[r]).sum::<f64>().max(1e-12);
    let mut cursor = 0.0;
    for &r in &roots {
        let span = sv[r] / root_total;
        xspan[r] = (cursor, cursor + span);
        cursor += span;
    }
    for &i in &bfs_order {
        let ch = &children[i];
        if ch.is_empty() {
            continue;
        }
        let ct: f64 = ch.iter().map(|&c| sv[c]).sum::<f64>().max(1e-12);
        let (x0, x1) = xspan[i];
        let span = x1 - x0;
        let mut c_cur = x0;
        for &j in ch {
            let sweep = span * sv[j] / ct;
            xspan[j] = (c_cur, c_cur + sweep);
            c_cur += sweep;
        }
    }

    let mut cidx: Vec<usize> = vec![0usize; n];
    for (ri, &r) in roots.iter().enumerate() {
        let mut stk: Vec<usize> = vec![r];
        while let Some(i) = stk.pop() {
            cidx[i] = ri;
            for &c in &children[i] {
                stk.push(c);
            }
        }
    }

    let title_h: i32 = if cfg.title.is_empty() { 8 } else { 38 };
    let pad = 4;
    let plot_x = pad;
    let plot_y = title_h;
    let plot_w = (cfg.width - pad * 2).max(1);
    let plot_h = (cfg.height - title_h - pad).max(1);
    let band_h = plot_h as f64 / (max_depth as f64 + 1.0);

    let palette: Vec<u32> = if cfg.palette.is_empty() {
        ICICLE_PALETTE.to_vec()
    } else {
        cfg.palette.to_vec()
    };
    let grand_total: f64 = roots.iter().map(|&r| sv[r]).sum::<f64>().max(1e-12);

    Some(Prepared {
        n,
        labels,
        values_eff: sv,
        bfs_order,
        depth,
        xspan,
        cidx,
        roots,
        palette,
        grand_total,
        layout: Layout {
            plot_x,
            plot_y,
            plot_w,
            plot_h,
            band_h,
            max_depth,
        },
    })
}

pub fn node_rect(p: &Prepared, i: usize) -> Rect {
    let l = &p.layout;
    let (x0, x1) = p.xspan[i];
    Rect {
        x: l.plot_x as f64 + x0 * l.plot_w as f64,
        y: l.plot_y as f64 + p.depth[i] as f64 * l.band_h,
        w: (x1 - x0) * l.plot_w as f64,
        h: l.band_h,
    }
}

pub fn node_rect_horizontal(p: &Prepared, i: usize) -> Rect {
    let l = &p.layout;
    let (y0, y1) = p.xspan[i];
    Rect {
        x: l.plot_x as f64 + p.depth[i] as f64 * l.band_h,
        y: l.plot_y as f64 + y0 * l.plot_h as f64,
        w: l.band_h,
        h: (y1 - y0) * l.plot_h as f64,
    }
}

pub fn color_hex(p: &Prepared, i: usize) -> [u8; 6] {
    hex6(p.palette[p.cidx[i] % p.palette.len()])
}

pub fn open_svg(buf: &mut Vec<u8>, cfg: &IcicleConfig) {
    push_b(buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(buf, cfg.width);
    push_b(buf, b"\" height=\"");
    push_i(buf, cfg.height);
    push_b(buf, b"\" viewBox=\"0 0 ");
    push_i(buf, cfg.width);
    push_b(buf, b" ");
    push_i(buf, cfg.height);
    push_b(buf, b"\">");
    push_b(
        buf,
        b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>",
    );
    if !cfg.title.is_empty() {
        push_b(buf, b"<text x=\"");
        push_i(buf, cfg.width / 2);
        push_b(buf, b"\" y=\"24\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(buf, cfg.title);
        push_b(buf, b"</text>");
    }
}

pub fn rect_attrs(buf: &mut Vec<u8>, r: Rect) {
    push_b(buf, b" x=\"");
    push_f2(buf, r.x);
    push_b(buf, b"\" y=\"");
    push_f2(buf, r.y);
    push_b(buf, b"\" width=\"");
    push_f2(buf, r.w);
    push_b(buf, b"\" height=\"");
    push_f2(buf, r.h);
    push_b(buf, b"\"");
}

pub fn node_data_attrs(buf: &mut Vec<u8>, p: &Prepared, i: usize) {
    push_b(buf, b" data-idx=\"");
    push_i(buf, i as i32);
    push_b(buf, b"\" data-y=\"");
    push_f2(buf, p.values_eff[i]);
    push_b(buf, b"\" data-kv-pct=\"");
    push_f2(buf, p.values_eff[i] / p.grand_total * 100.0);
    push_b(buf, b"%");
    push_b(buf, b"\" data-lbl=\"");
    escape_xml(buf, &p.labels[i]);
    push_b(buf, b"\"");
}

pub fn label_in_rect(buf: &mut Vec<u8>, p: &Prepared, i: usize, r: Rect, white: bool) {
    if r.w < 26.0 || r.h < 14.0 {
        return;
    }
    let max_chars = ((r.w / 6.5) as usize).max(1);
    let label = truncate(&p.labels[i], max_chars);
    let font_size: i32 = if r.h > 30.0 { 11 } else { 9 };
    let fill: &[u8] = if white { b"#fff" } else { b"#1f2937" };
    push_b(buf, b"<text x=\"");
    push_f2(buf, r.x + 6.0);
    push_b(buf, b"\" y=\"");
    push_f2(buf, r.y + r.h / 2.0 + 4.0);
    push_b(
        buf,
        b"\" font-family=\"Arial,sans-serif\" font-size=\"",
    );
    push_i(buf, font_size);
    push_b(buf, b"\" fill=\"");
    buf.extend_from_slice(fill);
    push_b(buf, b"\" pointer-events=\"none\">");
    escape_xml(buf, label);
    push_b(buf, b"</text>");
}

pub fn finalize(mut buf: Vec<u8>, cfg: &IcicleConfig) -> String {
    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}
