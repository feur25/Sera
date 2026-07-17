use super::common::{color_hex, finalize, node_data_attrs, open_svg, prepare};
use crate::plot::statistical::common::{push_b, push_f2};
use super::config::IcicleConfig;

#[crate::chart_demo("labels=[\"Root\",\"A\",\"B\",\"A1\",\"A2\",\"B1\",\"B2\"], parents=[\"\",\"Root\",\"Root\",\"A\",\"A\",\"B\",\"B\"], values=[0,40,30,20,20,15,15]")]

pub fn render(cfg: &IcicleConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut b = Vec::<u8>::with_capacity(p.n * 260 + 1536);
    open_svg(&mut b, cfg);

    let cx = cfg.width as f64 / 2.0;
    let cy = cfg.height as f64 / 2.0 + if cfg.title.is_empty() { 0.0 } else { 8.0 };
    let outer = (cfg.width.min(cfg.height) as f64 / 2.0 - 24.0).max(30.0);
    let rings = (p.layout.max_depth + 1).max(1) as f64;
    let ring_w = outer / rings;

    let order = p.bfs_order.clone();
    for i in order {
        let (x0, x1) = p.xspan[i];
        let span = x1 - x0;
        if span < 0.0008 {
            continue;
        }
        let r0 = p.depth[i] as f64 * ring_w;
        let r1 = r0 + ring_w;
        let hx = color_hex(&p, i);
        let opacity: &[u8] = match p.depth[i] {
            0 => b"0.92",
            1 => b"0.82",
            2 => b"0.72",
            _ => b"0.64",
        };

        push_b(&mut b, b"<path");
        node_data_attrs(&mut b, &p, i);
        push_b(&mut b, b" d=\"");
        if span > 0.999 {
            arc_ring(&mut b, cx, cy, r0, r1);
        } else {
            arc_sector(&mut b, cx, cy, r0, r1, x0, x1);
        }
        push_b(&mut b, b"\" fill=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" opacity=\"");
        b.extend_from_slice(opacity);
        push_b(&mut b, b"\" stroke=\"#fff\" stroke-width=\"1\"/>");

        if span > 0.05 && r1 - r0 > 12.0 {
            label_on_arc(&mut b, &p, i, cx, cy, (r0 + r1) / 2.0, x0, x1);
        }
    }
    finalize(b, cfg)
}

fn theta(t: f64) -> f64 {
    t * std::f64::consts::TAU - std::f64::consts::FRAC_PI_2
}

fn arc_sector(buf: &mut Vec<u8>, cx: f64, cy: f64, r0: f64, r1: f64, x0: f64, x1: f64) {
    let t0 = theta(x0);
    let t1 = theta(x1);
    let large = if x1 - x0 > 0.5 { 1 } else { 0 };
    let (ox0, oy0) = (cx + r1 * t0.cos(), cy + r1 * t0.sin());
    let (ox1, oy1) = (cx + r1 * t1.cos(), cy + r1 * t1.sin());
    if r0 < 0.5 {
        push_b(buf, b"M ");
        push_f2(buf, cx);
        push_b(buf, b",");
        push_f2(buf, cy);
        push_b(buf, b" L ");
        push_f2(buf, ox0);
        push_b(buf, b",");
        push_f2(buf, oy0);
        push_b(buf, b" A ");
        push_f2(buf, r1);
        push_b(buf, b",");
        push_f2(buf, r1);
        push_b(buf, b" 0 ");
        push_b(buf, if large == 1 { b"1" } else { b"0" });
        push_b(buf, b",1 ");
        push_f2(buf, ox1);
        push_b(buf, b",");
        push_f2(buf, oy1);
        push_b(buf, b" Z");
        return;
    }
    let (ix0, iy0) = (cx + r0 * t0.cos(), cy + r0 * t0.sin());
    let (ix1, iy1) = (cx + r0 * t1.cos(), cy + r0 * t1.sin());
    push_b(buf, b"M ");
    push_f2(buf, ix0);
    push_b(buf, b",");
    push_f2(buf, iy0);
    push_b(buf, b" L ");
    push_f2(buf, ox0);
    push_b(buf, b",");
    push_f2(buf, oy0);
    push_b(buf, b" A ");
    push_f2(buf, r1);
    push_b(buf, b",");
    push_f2(buf, r1);
    push_b(buf, b" 0 ");
    push_b(buf, if large == 1 { b"1" } else { b"0" });
    push_b(buf, b",1 ");
    push_f2(buf, ox1);
    push_b(buf, b",");
    push_f2(buf, oy1);
    push_b(buf, b" L ");
    push_f2(buf, ix1);
    push_b(buf, b",");
    push_f2(buf, iy1);
    push_b(buf, b" A ");
    push_f2(buf, r0);
    push_b(buf, b",");
    push_f2(buf, r0);
    push_b(buf, b" 0 ");
    push_b(buf, if large == 1 { b"1" } else { b"0" });
    push_b(buf, b",0 ");
    push_f2(buf, ix0);
    push_b(buf, b",");
    push_f2(buf, iy0);
    push_b(buf, b" Z");
}

fn arc_ring(buf: &mut Vec<u8>, cx: f64, cy: f64, r0: f64, r1: f64) {
    if r0 < 0.5 {
        push_b(buf, b"M ");
        push_f2(buf, cx - r1);
        push_b(buf, b",");
        push_f2(buf, cy);
        push_b(buf, b" A ");
        push_f2(buf, r1);
        push_b(buf, b",");
        push_f2(buf, r1);
        push_b(buf, b" 0 1,1 ");
        push_f2(buf, cx + r1);
        push_b(buf, b",");
        push_f2(buf, cy);
        push_b(buf, b" A ");
        push_f2(buf, r1);
        push_b(buf, b",");
        push_f2(buf, r1);
        push_b(buf, b" 0 1,1 ");
        push_f2(buf, cx - r1);
        push_b(buf, b",");
        push_f2(buf, cy);
        push_b(buf, b" Z");
        return;
    }
    push_b(buf, b"M ");
    push_f2(buf, cx - r1);
    push_b(buf, b",");
    push_f2(buf, cy);
    push_b(buf, b" A ");
    push_f2(buf, r1);
    push_b(buf, b",");
    push_f2(buf, r1);
    push_b(buf, b" 0 1,1 ");
    push_f2(buf, cx + r1);
    push_b(buf, b",");
    push_f2(buf, cy);
    push_b(buf, b" A ");
    push_f2(buf, r1);
    push_b(buf, b",");
    push_f2(buf, r1);
    push_b(buf, b" 0 1,1 ");
    push_f2(buf, cx - r1);
    push_b(buf, b",");
    push_f2(buf, cy);
    push_b(buf, b" Z M ");
    push_f2(buf, cx - r0);
    push_b(buf, b",");
    push_f2(buf, cy);
    push_b(buf, b" A ");
    push_f2(buf, r0);
    push_b(buf, b",");
    push_f2(buf, r0);
    push_b(buf, b" 0 1,0 ");
    push_f2(buf, cx + r0);
    push_b(buf, b",");
    push_f2(buf, cy);
    push_b(buf, b" A ");
    push_f2(buf, r0);
    push_b(buf, b",");
    push_f2(buf, r0);
    push_b(buf, b" 0 1,0 ");
    push_f2(buf, cx - r0);
    push_b(buf, b",");
    push_f2(buf, cy);
    push_b(buf, b" Z");
}

fn label_on_arc(
    buf: &mut Vec<u8>,
    p: &super::common::Prepared,
    i: usize,
    cx: f64,
    cy: f64,
    rmid: f64,
    x0: f64,
    x1: f64,
) {
    let tm = theta((x0 + x1) / 2.0);
    let lx = cx + rmid * tm.cos();
    let ly = cy + rmid * tm.sin();
    let max_chars = (((x1 - x0) * std::f64::consts::TAU * rmid / 7.0) as usize).max(1);
    let label = crate::plot::statistical::common::truncate(&p.labels[i], max_chars);
    push_b(buf, b"<text x=\"");
    push_f2(buf, lx);
    push_b(buf, b"\" y=\"");
    push_f2(buf, ly + 3.0);
    push_b(
        buf,
        b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#fff\" pointer-events=\"none\">",
    );
    crate::plot::statistical::common::escape_xml(buf, label);
    push_b(buf, b"</text>");
}
