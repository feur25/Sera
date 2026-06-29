use super::common::{finalize, open_svg, prepare};
use super::config::SlopeConfig;
use crate::plot::statistical::common::{escape_xml, push_b, push_f2, push_i};

#[crate::chart_demo(
    "labels=[\"A\",\"B\",\"C\",\"D\",\"E\"], left=[20,35,15,42,28], right=[35,28,40,55,22]"
)]

pub fn render(cfg: &SlopeConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let l = &p.layout;
    let deltas: Vec<f64> = (0..p.n)
        .map(|i| p.values_right[i] - p.values_left[i])
        .collect();
    let max_abs = deltas
        .iter()
        .map(|d| d.abs())
        .fold(0.0_f64, f64::max)
        .max(1e-12);

    let mut b = Vec::<u8>::with_capacity(p.n * 220 + 1024);
    open_svg(&mut b, cfg);

    let baseline = (cfg.width) / 2;
    let half_w = ((l.x_right - l.x_left) / 2 - 30) as f64;

    push_b(&mut b, b"<line x1=\"");
    push_i(&mut b, baseline);
    push_b(&mut b, b"\" y1=\"");
    push_i(&mut b, l.pad_t);
    push_b(&mut b, b"\" x2=\"");
    push_i(&mut b, baseline);
    push_b(&mut b, b"\" y2=\"");
    push_i(&mut b, l.pad_t + l.plot_h);
    push_b(
        &mut b,
        b"\" stroke=\"#94a3b8\" stroke-width=\"1.4\" stroke-dasharray=\"4,3\"/>",
    );
    push_b(&mut b, b"<text x=\"");
    push_i(&mut b, baseline);
    push_b(&mut b, b"\" y=\"");
    push_i(&mut b, l.pad_t - 10);
    push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"12\" font-weight=\"700\" fill=\"#374151\">");
    push_b(&mut b, b"&#916; (");
    escape_xml(&mut b, cfg.right_label);
    push_b(&mut b, b" - ");
    escape_xml(&mut b, cfg.left_label);
    push_b(&mut b, b")</text>");

    let row_h = (l.plot_h as f64 / p.n as f64).max(8.0);
    for i in 0..p.n {
        let y_top = l.pad_t + (i as f64 * row_h) as i32 + 2;
        let y_bot = l.pad_t + ((i + 1) as f64 * row_h) as i32 - 2;
        let y_mid = (y_top + y_bot) / 2;
        let d = deltas[i];
        let frac = d / max_abs;
        let bar_w = (frac.abs() * half_w) as i32;
        let pos = d >= 0.0;
        let color: &[u8] = if pos { b"#10B981" } else { b"#F43F5E" };
        let bx = if pos { baseline } else { baseline - bar_w };
        push_b(&mut b, b"<rect data-idx=\"");
        push_i(&mut b, i as i32);
        push_b(&mut b, b"\" data-lbl=\"");
        escape_xml(&mut b, &p.labels[i]);
        push_b(&mut b, b"\" x=\"");
        push_i(&mut b, bx);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, y_top);
        push_b(&mut b, b"\" width=\"");
        push_i(&mut b, bar_w.max(1));
        push_b(&mut b, b"\" height=\"");
        push_i(&mut b, (y_bot - y_top).max(2));
        push_b(&mut b, b"\" rx=\"3\" fill=\"");
        b.extend_from_slice(color);
        push_b(&mut b, b"\" opacity=\"0.85\"/>");

        let lbl_x = if pos { baseline - 8 } else { baseline + 8 };
        let anchor = if pos { "end" } else { "start" };
        push_b(&mut b, b"<text x=\"");
        push_i(&mut b, lbl_x);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, y_mid + 4);
        push_b(&mut b, b"\" text-anchor=\"");
        b.extend_from_slice(anchor.as_bytes());
        push_b(
            &mut b,
            b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">",
        );
        let short = if p.labels[i].len() > 14 {
            &p.labels[i][..14]
        } else {
            &p.labels[i]
        };
        escape_xml(&mut b, short);
        push_b(&mut b, b"</text>");

        let val_x = if pos {
            baseline + bar_w + 6
        } else {
            baseline - bar_w - 6
        };
        let val_anchor = if pos { "start" } else { "end" };
        if cfg.show_text {
            push_b(&mut b, b"<text x=\"");
            push_i(&mut b, val_x);
            push_b(&mut b, b"\" y=\"");
            push_i(&mut b, y_mid + 4);
            push_b(&mut b, b"\" text-anchor=\"");
            b.extend_from_slice(val_anchor.as_bytes());
            push_b(
                &mut b,
                b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" font-weight=\"700\" fill=\"",
            );
            b.extend_from_slice(color);
            push_b(&mut b, b"\">");
            if d > 0.0 {
                push_b(&mut b, b"+");
            }
            push_f2(&mut b, d);
            push_b(&mut b, b"</text>");
        }
    }
    finalize(b, cfg)
}
