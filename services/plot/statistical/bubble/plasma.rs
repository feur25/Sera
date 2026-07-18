use super::common::{compute_layout, make_frame, point_px, radius};
use super::config::BubbleConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i};

fn plasma_color(i: usize) -> u32 {
    const COLS: [u32; 8] = [
        0x00E5FF, 0xFF00C8, 0x00FF88, 0xFFD600, 0xFF4D00, 0x7B00FF, 0x00FFD0, 0xFF0055,
    ];
    COLS[i % COLS.len()]
}

#[crate::chart_demo("x=[1,2,3,4,5,6,7], y=[3,5,2,7,6,8,4], sizes=[20,40,15,55,30,45,25], labels=[\"A\",\"B\",\"C\",\"D\",\"E\",\"F\",\"G\"]")]

pub fn render(cfg: &BubbleConfig) -> String {
    let layout = match compute_layout(cfg) {
        Some(l) => l,
        None => return String::new(),
    };
    let n = layout.n;
    let mut f = make_frame(cfg, n, 20);
    f.open(cfg.title, true);
    f.x_grid(6, layout.xmin2, layout.xmax2, cfg.gridlines);
    f.y_grid(5, layout.ymin2, layout.ymax2, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    push_b(&mut f.buf, b"<defs>");
    push_b(
        &mut f.buf,
        b"<filter id=\"plsf\" x=\"-80%\" y=\"-80%\" width=\"360%\" height=\"360%\">",
    );
    push_b(
        &mut f.buf,
        b"<feGaussianBlur stdDeviation=\"4\" result=\"b\"/>",
    );
    push_b(
        &mut f.buf,
        b"<feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>",
    );
    push_b(&mut f.buf, b"</filter>");
    push_b(
        &mut f.buf,
        b"<filter id=\"plscore\" x=\"-100%\" y=\"-100%\" width=\"400%\" height=\"400%\">",
    );
    push_b(
        &mut f.buf,
        b"<feGaussianBlur stdDeviation=\"2.5\" result=\"b\"/>",
    );
    push_b(
        &mut f.buf,
        b"<feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>",
    );
    push_b(&mut f.buf, b"</filter>");
    push_b(&mut f.buf, b"</defs>");

    for &i in &layout.indices {
        let (cx, cy) = point_px(&layout, &f, cfg.x_values[i], cfg.y_values[i]);
        let r = radius(cfg, &layout, i);
        let col = plasma_color(i);
        let hx = hex6(col);

        push_b(&mut f.buf, b"<g data-idx=\"");
        push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-y=\"");
        push_f2(&mut f.buf, cfg.y_values[i]);
        if i < cfg.labels.len() {
            push_b(&mut f.buf, b"\" data-lbl=\"");
            escape_xml(&mut f.buf, &cfg.labels[i]);
        }
        push_b(&mut f.buf, b"\">");

        push_b(&mut f.buf, b"<circle cx=\"");
        push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" cy=\"");
        push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\"");
        push_f2(&mut f.buf, r * 2.2);
        push_b(&mut f.buf, b"\" fill=\"none\" stroke=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(
            &mut f.buf,
            b"\" stroke-width=\"1\" stroke-opacity=\"0.12\" filter=\"url(#plsf)\"/>",
        );

        push_b(&mut f.buf, b"<circle cx=\"");
        push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" cy=\"");
        push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\"");
        push_f2(&mut f.buf, r * 1.6);
        push_b(&mut f.buf, b"\" fill=\"none\" stroke=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(
            &mut f.buf,
            b"\" stroke-width=\"1.5\" stroke-opacity=\"0.25\" filter=\"url(#plsf)\"/>",
        );

        push_b(&mut f.buf, b"<circle cx=\"");
        push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" cy=\"");
        push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\"");
        push_f2(&mut f.buf, r * 1.15);
        push_b(&mut f.buf, b"\" fill=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.06\" stroke=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(
            &mut f.buf,
            b"\" stroke-width=\"2\" stroke-opacity=\"0.55\" filter=\"url(#plsf)\"/>",
        );

        push_b(&mut f.buf, b"<circle cx=\"");
        push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" cy=\"");
        push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\"");
        push_f2(&mut f.buf, r * 0.55);
        push_b(&mut f.buf, b"\" fill=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(
            &mut f.buf,
            b"\" fill-opacity=\"0.9\" filter=\"url(#plscore)\"/>",
        );

        push_b(&mut f.buf, b"</g>");
    }

    let slots_json;
    let json: &str = if cfg.hover.is_empty() {
        "[]"
    } else {
        slots_json = slots_to_json(cfg.hover);
        &slots_json
    };
    f.html(json)
}
