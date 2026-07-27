use super::common::{color_for, finalize, open_svg, prepare_with};
use super::config::GaugeConfig;
use crate::plot::statistical::common::{hex6, lerp_rgb, push_b, push_f2};

#[crate::chart_demo(
    "value=72, min_val=0, max_val=100, label=\"Score\", history=[55,58,60,57,63,66,64,68,70,72], variant=\"sparkline\""
)]

pub fn render(cfg: &GaugeConfig) -> String {
    let p = prepare_with(cfg, 0.42, 0.27);
    let mut b = Vec::<u8>::with_capacity(4096);
    open_svg(&mut b, cfg);
    super::basic::draw(&mut b, cfg, &p);

    let n = cfg.history.len();
    if n > 1 {
        let mut lo = f64::INFINITY;
        let mut hi = f64::NEG_INFINITY;
        for &v in cfg.history {
            if v.is_finite() {
                lo = lo.min(v);
                hi = hi.max(v);
            }
        }
        if hi > lo {
            let col = color_for(&p, p.frac);
            let hx = hex6(col);
            let light = hex6(lerp_rgb(col, 0xFFFFFF, 0.85));

            let pad_x = 22.0;
            let panel_x = pad_x;
            let panel_y = p.cy + 62.0;
            let panel_w = cfg.width as f64 - pad_x * 2.0;
            let panel_h = (cfg.height as f64 - panel_y - 14.0).max(36.0);

            push_b(&mut b, b"<rect x=\"");
            push_f2(&mut b, panel_x);
            push_b(&mut b, b"\" y=\"");
            push_f2(&mut b, panel_y);
            push_b(&mut b, b"\" width=\"");
            push_f2(&mut b, panel_w);
            push_b(&mut b, b"\" height=\"");
            push_f2(&mut b, panel_h);
            push_b(&mut b, b"\" rx=\"10\" fill=\"#f8fafc\" stroke=\"#e2e8f0\" stroke-width=\"1\"/>");

            let first = cfg.history[0];
            let last = cfg.history[n - 1];
            let delta = if first.abs() > 1e-9 {
                (last - first) / first.abs() * 100.0
            } else {
                0.0
            };
            let up = delta >= 0.0;
            let badge_col: &[u8] = if up { b"#10B981" } else { b"#EF4444" };
            let arrow = if up { "\u{25B2}" } else { "\u{25BC}" };
            let badge_txt = format!("{} {:.1}%", arrow, delta.abs());
            let bw = badge_txt.chars().count() as f64 * 6.0 + 14.0;
            let bx = panel_x + panel_w - bw - 8.0;
            let by = panel_y + 8.0;
            push_b(&mut b, b"<rect x=\"");
            push_f2(&mut b, bx);
            push_b(&mut b, b"\" y=\"");
            push_f2(&mut b, by);
            push_b(&mut b, b"\" width=\"");
            push_f2(&mut b, bw);
            push_b(&mut b, b"\" height=\"17\" rx=\"8.5\" fill=\"");
            b.extend_from_slice(badge_col);
            push_b(&mut b, b"\" opacity=\"0.94\"/>");
            push_b(&mut b, b"<text x=\"");
            push_f2(&mut b, bx + bw / 2.0);
            push_b(&mut b, b"\" y=\"");
            push_f2(&mut b, by + 12.5);
            push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" font-weight=\"700\" fill=\"#fff\">");
            b.extend_from_slice(badge_txt.as_bytes());
            push_b(&mut b, b"</text>");

            push_b(&mut b, b"<text x=\"");
            push_f2(&mut b, panel_x + 10.0);
            push_b(&mut b, b"\" y=\"");
            push_f2(&mut b, panel_y + 17.0);
            push_b(&mut b, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" font-weight=\"700\" fill=\"#94a3b8\">HISTORY</text>");

            let sx0 = panel_x + 10.0;
            let sx1 = panel_x + panel_w - 10.0;
            let sy0 = panel_y + 26.0;
            let sy1 = panel_y + panel_h - 8.0;
            let sw = sx1 - sx0;
            let sh = (sy1 - sy0).max(6.0);

            let pts: Vec<(f64, f64)> = cfg
                .history
                .iter()
                .enumerate()
                .map(|(i, &v)| {
                    let x = sx0 + (i as f64 / (n - 1) as f64) * sw;
                    let y = sy0 + sh - ((v - lo) / (hi - lo)) * sh;
                    (x, y)
                })
                .collect();

            push_b(&mut b, b"<path d=\"M ");
            push_f2(&mut b, pts[0].0);
            b.push(b',');
            push_f2(&mut b, sy1);
            for &(x, y) in &pts {
                push_b(&mut b, b" L ");
                push_f2(&mut b, x);
                b.push(b',');
                push_f2(&mut b, y);
            }
            push_b(&mut b, b" L ");
            push_f2(&mut b, pts[n - 1].0);
            b.push(b',');
            push_f2(&mut b, sy1);
            push_b(&mut b, b" Z\" fill=\"#");
            b.extend_from_slice(&light);
            push_b(&mut b, b"\" opacity=\"0.65\"/>");

            push_b(&mut b, b"<polyline points=\"");
            for (i, &(x, y)) in pts.iter().enumerate() {
                if i > 0 {
                    b.push(b' ');
                }
                push_f2(&mut b, x);
                b.push(b',');
                push_f2(&mut b, y);
            }
            push_b(
                &mut b,
                b"\" fill=\"none\" stroke=\"#ffffff\" stroke-width=\"3.6\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>",
            );
            push_b(&mut b, b"<polyline points=\"");
            for (i, &(x, y)) in pts.iter().enumerate() {
                if i > 0 {
                    b.push(b' ');
                }
                push_f2(&mut b, x);
                b.push(b',');
                push_f2(&mut b, y);
            }
            push_b(&mut b, b"\" fill=\"none\" stroke=\"#");
            b.extend_from_slice(&hx);
            push_b(
                &mut b,
                b"\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>",
            );

            let (lx, ly) = pts[n - 1];
            push_b(&mut b, b"<circle cx=\"");
            push_f2(&mut b, lx);
            push_b(&mut b, b"\" cy=\"");
            push_f2(&mut b, ly);
            push_b(&mut b, b"\" r=\"5.5\" fill=\"#");
            b.extend_from_slice(&hx);
            push_b(&mut b, b"\" opacity=\"0.25\"/>");
            push_b(&mut b, b"<circle cx=\"");
            push_f2(&mut b, lx);
            push_b(&mut b, b"\" cy=\"");
            push_f2(&mut b, ly);
            push_b(&mut b, b"\" r=\"3\" fill=\"#");
            b.extend_from_slice(&hx);
            push_b(&mut b, b"\" stroke=\"#fff\" stroke-width=\"1.3\"/>");
        }
    }

    finalize(b, cfg)
}
