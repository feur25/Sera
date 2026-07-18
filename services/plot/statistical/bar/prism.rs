use super::config::BarConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{
    escape_xml, hex6, lerp_rgb, palette_color, push_b, push_f2, push_i, Frame,
};

fn prism_color(i: usize) -> u32 {
    const COLS: [u32; 8] = [
        0xF43F5E, 0xFB923C, 0xFACC15, 0x34D399, 0x38BDF8, 0x818CF8, 0xC084FC, 0xF472B6,
    ];
    COLS[i % COLS.len()]
}

#[crate::chart_demo(
    "labels=[\"Alpha\",\"Beta\",\"Gamma\",\"Delta\",\"Epsilon\"], values=[24,38,17,42,29]"
)]

pub fn render(cfg: &BarConfig) -> String {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 {
        return String::new();
    }
    let max_v = cfg.values[..n]
        .iter()
        .cloned()
        .fold(0.0_f64, f64::max)
        .max(1e-9);
    let mut f = Frame::new_html(
        cfg.title,
        cfg.width,
        cfg.height,
        52,
        38,
        52,
        20,
        n * 400 + 4096,
    );
    f.open(cfg.title, false);

    push_b(&mut f.buf, b"<defs>");
    push_b(
        &mut f.buf,
        b"<filter id=\"prsmf\" x=\"-20%\" y=\"-20%\" width=\"140%\" height=\"140%\">",
    );
    push_b(
        &mut f.buf,
        b"<feGaussianBlur stdDeviation=\"2\" result=\"b\"/>",
    );
    push_b(
        &mut f.buf,
        b"<feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>",
    );
    push_b(&mut f.buf, b"</filter>");
    for i in 0..n {
        let base = if !cfg.palette.is_empty() {
            palette_color(cfg.palette, i)
        } else if cfg.color_hex != 0 {
            cfg.color_hex
        } else {
            prism_color(i)
        };
        let mid = lerp_rgb(base, 0xFFFFFF, 0.35);
        let bright = lerp_rgb(base, 0xFFFFFF, 0.7);
        push_b(&mut f.buf, b"<linearGradient id=\"prsmg");
        push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" x1=\"0\" y1=\"1\" x2=\"0\" y2=\"0\">");
        push_b(&mut f.buf, b"<stop offset=\"0\" stop-color=\"#");
        f.buf.extend_from_slice(&hex6(base));
        push_b(&mut f.buf, b"\" stop-opacity=\"1\"/>");
        push_b(&mut f.buf, b"<stop offset=\"0.5\" stop-color=\"#");
        f.buf.extend_from_slice(&hex6(mid));
        push_b(&mut f.buf, b"\" stop-opacity=\"0.95\"/>");
        push_b(&mut f.buf, b"<stop offset=\"1\" stop-color=\"#");
        f.buf.extend_from_slice(&hex6(bright));
        push_b(&mut f.buf, b"\" stop-opacity=\"1\"/>");
        push_b(&mut f.buf, b"</linearGradient>");
        push_b(&mut f.buf, b"<linearGradient id=\"prsmfacet");
        push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">");
        push_b(
            &mut f.buf,
            b"<stop offset=\"0\" stop-color=\"#ffffff\" stop-opacity=\"0.35\"/>",
        );
        push_b(&mut f.buf, b"<stop offset=\"0.4\" stop-color=\"#");
        f.buf.extend_from_slice(&hex6(bright));
        push_b(&mut f.buf, b"\" stop-opacity=\"0.5\"/>");
        push_b(
            &mut f.buf,
            b"<stop offset=\"1\" stop-color=\"#ffffff\" stop-opacity=\"0.0\"/>",
        );
        push_b(&mut f.buf, b"</linearGradient>");
    }
    push_b(&mut f.buf, b"</defs>");

    f.y_grid(5, 0.0, max_v, cfg.gridlines);

    let slot_w = f.pw as f64 / n as f64;
    let bar_w = (slot_w * 0.68) as i32;
    let facet_off = (bar_w as f64 * 0.18) as i32;

    for i in 0..n {
        let bar_h = (cfg.values[i] / max_v * f.ph as f64) as i32;
        let bx = f.pl + (i as f64 * slot_w + (slot_w - bar_w as f64) / 2.0) as i32;
        let by = f.pt + f.ph - bar_h;

        push_b(&mut f.buf, b"<rect data-idx=\"");
        push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-lbl=\"");
        escape_xml(&mut f.buf, &cfg.labels[i]);
        push_b(&mut f.buf, b"\" data-y=\"");
        push_f2(&mut f.buf, cfg.values[i]);
        push_b(&mut f.buf, b"\" x=\"");
        push_i(&mut f.buf, bx);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, by);
        push_b(&mut f.buf, b"\" width=\"");
        push_i(&mut f.buf, bar_w.max(1));
        push_b(&mut f.buf, b"\" height=\"");
        push_i(&mut f.buf, bar_h.max(1));
        push_b(&mut f.buf, b"\" fill=\"url(#prsmg");
        push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b")\" rx=\"3\"/>");

        push_b(&mut f.buf, b"<rect x=\"");
        push_i(&mut f.buf, bx);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, by);
        push_b(&mut f.buf, b"\" width=\"");
        push_i(&mut f.buf, bar_w.max(1));
        push_b(&mut f.buf, b"\" height=\"");
        push_i(&mut f.buf, bar_h.max(1));
        push_b(&mut f.buf, b"\" fill=\"url(#prsmfacet");
        push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b")\" rx=\"3\"/>");

        if bar_h > 10 {
            push_b(&mut f.buf, b"<line x1=\"");
            push_i(&mut f.buf, bx + facet_off);
            push_b(&mut f.buf, b"\" y1=\"");
            push_i(&mut f.buf, by);
            push_b(&mut f.buf, b"\" x2=\"");
            push_i(&mut f.buf, bx);
            push_b(&mut f.buf, b"\" y2=\"");
            push_i(&mut f.buf, by + facet_off);
            push_b(
                &mut f.buf,
                b"\" stroke=\"#ffffff\" stroke-width=\"0.7\" stroke-opacity=\"0.4\"/>",
            );
            push_b(&mut f.buf, b"<line x1=\"");
            push_i(&mut f.buf, bx + bar_w - facet_off);
            push_b(&mut f.buf, b"\" y1=\"");
            push_i(&mut f.buf, by);
            push_b(&mut f.buf, b"\" x2=\"");
            push_i(&mut f.buf, bx + bar_w);
            push_b(&mut f.buf, b"\" y2=\"");
            push_i(&mut f.buf, by + facet_off);
            push_b(
                &mut f.buf,
                b"\" stroke=\"#ffffff\" stroke-width=\"0.7\" stroke-opacity=\"0.25\"/>",
            );
        }

        push_b(&mut f.buf, b"<rect x=\"");
        push_i(&mut f.buf, bx);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, by);
        push_b(&mut f.buf, b"\" width=\"");
        push_i(&mut f.buf, (bar_w as f64 * 0.28) as i32);
        push_b(&mut f.buf, b"\" height=\"");
        push_i(&mut f.buf, bar_h.max(1));
        push_b(
            &mut f.buf,
            b"\" fill=\"#ffffff\" fill-opacity=\"0.08\" rx=\"3\" filter=\"url(#prsmf)\"/>",
        );

        let cx = bx + bar_w / 2;
        push_b(&mut f.buf, b"<text x=\"");
        push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, f.pt + f.ph + 14);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">");
        escape_xml(&mut f.buf, &cfg.labels[i]);
        push_b(&mut f.buf, b"</text>");
        if cfg.show_text && bar_h > 16 {
            push_b(&mut f.buf, b"<text x=\"");
            push_i(&mut f.buf, cx);
            push_b(&mut f.buf, b"\" y=\"");
            push_i(&mut f.buf, by + 14);
            push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" font-weight=\"600\" fill=\"#1e293b\">");
            push_f2(&mut f.buf, cfg.values[i]);
            push_b(&mut f.buf, b"</text>");
        }
    }

    push_b(&mut f.buf, b"<line x1=\"");
    push_i(&mut f.buf, f.pl);
    push_b(&mut f.buf, b"\" y1=\"");
    push_i(&mut f.buf, f.pt);
    push_b(&mut f.buf, b"\" x2=\"");
    push_i(&mut f.buf, f.pl);
    push_b(&mut f.buf, b"\" y2=\"");
    push_i(&mut f.buf, f.pt + f.ph);
    push_b(&mut f.buf, b"\" stroke=\"#6b7280\" stroke-width=\"1.5\"/>");
    push_b(&mut f.buf, b"<line x1=\"");
    push_i(&mut f.buf, f.pl);
    push_b(&mut f.buf, b"\" y1=\"");
    push_i(&mut f.buf, f.pt + f.ph);
    push_b(&mut f.buf, b"\" x2=\"");
    push_i(&mut f.buf, f.pl + f.pw);
    push_b(&mut f.buf, b"\" y2=\"");
    push_i(&mut f.buf, f.pt + f.ph);
    push_b(&mut f.buf, b"\" stroke=\"#6b7280\" stroke-width=\"1.5\"/>");

    f.html(&slots_to_json(cfg.hover))
}
