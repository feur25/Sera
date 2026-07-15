use super::config::ScatterTernaryConfig;
use crate::html::hover::{build_chart_html, slots_to_json};
use crate::plot::statistical::common::{escape_xml, push_b, push_f2, push_i};

pub struct Layout {
    pub ox: f64,
    pub oy: f64,
    pub side: f64,
}

pub struct Prepared {
    pub n: usize,
    pub layout: Layout,
}

pub fn prepare(cfg: &ScatterTernaryConfig) -> Option<Prepared> {
    let n = cfg.a_values.len().min(cfg.b_values.len()).min(cfg.c_values.len());
    if n == 0 {
        return None;
    }
    let title_h = if cfg.title.is_empty() { 10.0 } else { 40.0 };
    let pad = 56.0;
    let avail_w = cfg.width as f64 - pad * 2.0;
    let avail_h = cfg.height as f64 - title_h - pad;
    let side = avail_w.min(avail_h / (3f64.sqrt() / 2.0));
    let ox = (cfg.width as f64 - side) / 2.0;
    let oy = title_h + side * (3f64.sqrt() / 2.0);
    Some(Prepared {
        n,
        layout: Layout { ox, oy, side },
    })
}

pub fn tri_point(l: &Layout, a: f64, b: f64, c: f64) -> (f64, f64) {
    let total = (a + b + c).max(1e-12);
    let fa = a / total;
    let fb = b / total;
    let px = l.ox + (0.5 * fa + fb) * l.side;
    let py = l.oy - fa * (3f64.sqrt() / 2.0) * l.side;
    (px, py)
}

pub fn open_svg(buf: &mut Vec<u8>, cfg: &ScatterTernaryConfig) {
    push_b(buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(buf, cfg.width);
    push_b(buf, b"\" height=\"");
    push_i(buf, cfg.height);
    push_b(buf, b"\" viewBox=\"0 0 ");
    push_i(buf, cfg.width);
    push_b(buf, b" ");
    push_i(buf, cfg.height);
    push_b(buf, b"\">");
    push_b(buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    if !cfg.title.is_empty() {
        push_b(buf, b"<text x=\"");
        push_i(buf, cfg.width / 2);
        push_b(buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(buf, cfg.title);
        push_b(buf, b"</text>");
    }
}

pub fn triangle_outline(buf: &mut Vec<u8>, l: &Layout) {
    let (tx, ty) = (l.ox + l.side * 0.5, l.oy - l.side * (3f64.sqrt() / 2.0));
    push_b(buf, b"<polygon points=\"");
    push_f2(buf, tx);
    push_b(buf, b",");
    push_f2(buf, ty);
    push_b(buf, b" ");
    push_f2(buf, l.ox + l.side);
    push_b(buf, b",");
    push_f2(buf, l.oy);
    push_b(buf, b" ");
    push_f2(buf, l.ox);
    push_b(buf, b",");
    push_f2(buf, l.oy);
    push_b(buf, b"\" fill=\"none\" stroke=\"#334155\" stroke-width=\"1.4\"/>");
}

pub fn gridlines(buf: &mut Vec<u8>, l: &Layout) {
    let steps: [f64; 4] = [0.2, 0.4, 0.6, 0.8];
    for &k in &steps {
        let segs = [
            (tri_point(l, k, 0.0, 1.0 - k), tri_point(l, k, 1.0 - k, 0.0)),
            (tri_point(l, 1.0 - k, k, 0.0), tri_point(l, 0.0, k, 1.0 - k)),
            (tri_point(l, 1.0 - k, 0.0, k), tri_point(l, 0.0, 1.0 - k, k)),
        ];
        for ((x1, y1), (x2, y2)) in segs {
            push_b(buf, b"<line x1=\"");
            push_f2(buf, x1);
            push_b(buf, b"\" y1=\"");
            push_f2(buf, y1);
            push_b(buf, b"\" x2=\"");
            push_f2(buf, x2);
            push_b(buf, b"\" y2=\"");
            push_f2(buf, y2);
            push_b(buf, b"\" stroke=\"#e2e8f0\" stroke-width=\"0.6\"/>");
        }
    }
}

pub fn axis_labels(buf: &mut Vec<u8>, l: &Layout, cfg: &ScatterTernaryConfig) {
    let (tx, ty) = (l.ox + l.side * 0.5, l.oy - l.side * (3f64.sqrt() / 2.0) - 10.0);
    push_b(buf, b"<text x=\"");
    push_f2(buf, tx);
    push_b(buf, b"\" y=\"");
    push_f2(buf, ty);
    push_b(
        buf,
        b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"12\" font-weight=\"600\" fill=\"#334155\">",
    );
    escape_xml(buf, cfg.a_label);
    push_b(buf, b"</text>");

    push_b(buf, b"<text x=\"");
    push_f2(buf, l.ox + l.side + 8.0);
    push_b(buf, b"\" y=\"");
    push_f2(buf, l.oy + 4.0);
    push_b(
        buf,
        b"\" text-anchor=\"start\" font-family=\"Arial,sans-serif\" font-size=\"12\" font-weight=\"600\" fill=\"#334155\">",
    );
    escape_xml(buf, cfg.b_label);
    push_b(buf, b"</text>");

    push_b(buf, b"<text x=\"");
    push_f2(buf, l.ox - 8.0);
    push_b(buf, b"\" y=\"");
    push_f2(buf, l.oy + 4.0);
    push_b(
        buf,
        b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"12\" font-weight=\"600\" fill=\"#334155\">",
    );
    escape_xml(buf, cfg.c_label);
    push_b(buf, b"</text>");
}

pub fn finalize(mut buf: Vec<u8>, cfg: &ScatterTernaryConfig) -> String {
    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}
