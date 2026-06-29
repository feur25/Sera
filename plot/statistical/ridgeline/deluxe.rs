use super::common::{area_path, close_svg, finalize, polyline, prepare, project_pts, ridge_label};
use super::config::RidgelineConfig;
use crate::plot::statistical::common::{hex6, push_b, push_f2, push_i, svg_open, svg_title};

fn aurora_hue(gi: usize, n: usize) -> (u32, u32) {
    let t = if n <= 1 {
        0.0
    } else {
        gi as f64 / (n - 1) as f64
    };
    let pairs = [
        (0x06B6D4_u32, 0x0EA5E9_u32),
        (0x22D3EE, 0x636EFA),
        (0x8B5CF6, 0xA855F7),
        (0xF43F5E, 0xEC4899),
        (0xF59E0B, 0xFBBF24),
        (0x10B981, 0x06B6D4),
    ];
    let idx = (t * (pairs.len() - 1) as f64) as usize;
    let idx = idx.min(pairs.len() - 1);
    pairs[idx]
}

#[crate::chart_demo("categories=[\"A\",\"A\",\"A\",\"A\",\"A\",\"B\",\"B\",\"B\",\"B\",\"B\",\"C\",\"C\",\"C\",\"C\",\"C\",\"D\",\"D\",\"D\",\"D\",\"D\"], values=[1.2,2.4,2.7,3.1,3.5,2.0,2.8,3.2,3.6,4.1,1.8,2.2,2.6,3.0,3.4,2.3,2.9,3.5,3.9,4.4]")]

pub fn render(cfg: &RidgelineConfig) -> String {
    let p = match prepare(cfg, Some(0.8)) {
        Some(v) => v,
        None => return String::new(),
    };
    let n_groups = p.group_order.len();
    let mut b = Vec::<u8>::with_capacity(n_groups * p.xs.len() * 32 + 4096);

    svg_open(&mut b, cfg.width, cfg.height);
    svg_title(
        &mut b,
        cfg.title,
        cfg.width / 2,
        if p.layout.title_h > 0 { 24 } else { 0 },
    );

    let n_xticks: i32 = 6;
    push_b(&mut b, b"<defs>");
    push_b(
        &mut b,
        b"<filter id=\"dlxrf\" x=\"-30%\" y=\"-60%\" width=\"160%\" height=\"220%\">",
    );
    push_b(&mut b, b"<feGaussianBlur stdDeviation=\"5\" result=\"b\"/>");
    push_b(
        &mut b,
        b"<feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>",
    );
    push_b(&mut b, b"</filter>");
    for gi in 0..n_groups {
        let (c0, c1) = aurora_hue(gi, n_groups);
        push_b(&mut b, b"<linearGradient id=\"dlxrlg");
        push_i(&mut b, gi as i32);
        push_b(&mut b, b"\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">");
        push_b(&mut b, b"<stop offset=\"0\" stop-color=\"#");
        b.extend_from_slice(&hex6(c0));
        push_b(&mut b, b"\" stop-opacity=\"0.85\"/>");
        push_b(&mut b, b"<stop offset=\"0.5\" stop-color=\"#");
        b.extend_from_slice(&hex6(c1));
        push_b(&mut b, b"\" stop-opacity=\"0.7\"/>");
        push_b(&mut b, b"<stop offset=\"1\" stop-color=\"#");
        b.extend_from_slice(&hex6(c0));
        push_b(&mut b, b"\" stop-opacity=\"0.4\"/>");
        push_b(&mut b, b"</linearGradient>");
    }
    push_b(&mut b, b"</defs>");

    for ti in 0..=n_xticks {
        let frac = ti as f64 / n_xticks as f64;
        let x = p.layout.pad_l + (p.layout.plot_w as f64 * frac) as i32;
        let val = p.x0 + p.xr * frac;
        push_b(&mut b, b"<line x1=\"");
        push_i(&mut b, x);
        push_b(&mut b, b"\" y1=\"");
        push_i(&mut b, p.layout.title_h);
        push_b(&mut b, b"\" x2=\"");
        push_i(&mut b, x);
        push_b(&mut b, b"\" y2=\"");
        push_i(&mut b, p.layout.axis_y);
        push_b(&mut b, b"\" stroke=\"#e2e8f0\" stroke-width=\"0.8\"/>");
        push_b(&mut b, b"<text x=\"");
        push_i(&mut b, x);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, p.layout.axis_y + 14);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#475569\">");
        push_f2(&mut b, val);
        push_b(&mut b, b"</text>");
    }
    push_b(&mut b, b"<line x1=\"");
    push_i(&mut b, p.layout.pad_l);
    push_b(&mut b, b"\" y1=\"");
    push_i(&mut b, p.layout.axis_y);
    push_b(&mut b, b"\" x2=\"");
    push_i(&mut b, p.layout.pad_l + p.layout.plot_w);
    push_b(&mut b, b"\" y2=\"");
    push_i(&mut b, p.layout.axis_y);
    push_b(&mut b, b"\" stroke=\"#1e293b\" stroke-width=\"1\"/>");

    for gi in (0..n_groups).rev() {
        let (c0, _c1) = aurora_hue(gi, n_groups);
        let hx = hex6(c0);
        let base_y = p.layout.title_h + (gi + 1) as i32 * p.layout.row_h;
        let pts = project_pts(&p, &p.curves[gi], base_y);

        push_b(&mut b, b"<g data-series=\"");
        push_i(&mut b, gi as i32);
        push_b(&mut b, b"\" data-idx=\"");
        push_i(&mut b, gi as i32);
        push_b(&mut b, b"\">");
        area_path(&mut b, &pts, base_y as f64);
        push_b(&mut b, b" fill=\"#0f172a\"/>");
        area_path(&mut b, &pts, base_y as f64);
        push_b(&mut b, b" fill=\"url(#dlxrlg");
        push_i(&mut b, gi as i32);
        push_b(&mut b, b")\" filter=\"url(#dlxrf)\"/>");
        polyline(&mut b, &pts, &hx, 1.8);

        push_b(&mut b, b"<line x1=\"");
        push_i(&mut b, p.layout.pad_l - 4);
        push_b(&mut b, b"\" y1=\"");
        push_i(&mut b, base_y);
        push_b(&mut b, b"\" x2=\"");
        push_i(&mut b, p.layout.pad_l + p.layout.plot_w);
        push_b(&mut b, b"\" y2=\"");
        push_i(&mut b, base_y);
        push_b(&mut b, b"\" stroke=\"#1e293b\" stroke-width=\"0.5\"/>");
        push_b(&mut b, b"<text x=\"");
        push_i(&mut b, p.layout.pad_l - 8);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, base_y - p.layout.row_h / 2 + 4);
        push_b(&mut b, b"\" text-anchor=\"end\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"11\" font-weight=\"600\" fill=\"#94a3b8\">");
        let name_str = if p.group_order[gi].len() > 14 {
            &p.group_order[gi][..14]
        } else {
            &p.group_order[gi]
        };
        b.extend_from_slice(name_str.as_bytes());
        push_b(&mut b, b"</text>");
        push_b(&mut b, b"</g>");
    }

    let _ = ridge_label;
    close_svg(&mut b, cfg, &p, false);
    finalize(b, cfg)
}
