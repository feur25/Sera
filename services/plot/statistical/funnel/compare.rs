use super::config::FunnelConfig;
use crate::html::hover::{build_chart_html, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_i, truncate};

fn stage_text(buf: &mut Vec<u8>, info: &str, val: f64, initial: f64, previous: f64, total: f64) {
    let mut parts: Vec<String> = Vec::new();
    for part in info.split('+') {
        match part {
            "value" => parts.push(format_value(val)),
            "percent_initial" => parts.push(format!("{:.1}%", if initial > 0.0 { val / initial * 100.0 } else { 0.0 })),
            "percent_previous" => parts.push(format!("{:.1}%", if previous > 0.0 { val / previous * 100.0 } else { 0.0 })),
            "percent_total" => parts.push(format!("{:.1}%", if total > 0.0 { val / total * 100.0 } else { 0.0 })),
            _ => {}
        }
    }
    escape_xml(buf, &parts.join(" · "));
}

fn format_value(v: f64) -> String {
    if v >= 1_000_000.0 {
        format!("{:.1}M", v / 1_000_000.0)
    } else if v >= 1000.0 {
        format!("{:.1}k", v / 1000.0)
    } else {
        format!("{}", v)
    }
}

#[crate::chart_demo(
    "series=[[120,60,30,20],[100,60,40,30,20],[90,70,50,30,10,5]], series_names=[\"Montreal\",\"Toronto\",\"Vancouver\"], category_series=[[\"Website visit\",\"Downloads\",\"Potential customers\",\"Requested price\"],[\"Website visit\",\"Downloads\",\"Potential customers\",\"Requested price\",\"Invoice sent\"],[\"Website visit\",\"Downloads\",\"Potential customers\",\"Requested price\",\"Invoice sent\",\"Finalized\"]], variant=\"compare\""
)]

pub fn render(cfg: &FunnelConfig) -> String {
    let n_funnels = cfg.series.len();
    if n_funnels == 0 {
        return String::new();
    }

    let pad_l = 20i32;
    let pad_r = 20i32;
    let pad_t = 50i32;
    let pad_b = 30i32;
    let col_gap = 24i32;
    let plot_w = cfg.width - pad_l - pad_r;
    let plot_h = cfg.height - pad_t - pad_b;
    let col_w = (plot_w - col_gap * (n_funnels as i32 - 1).max(0)) / n_funnels as i32;

    let mut b = Vec::<u8>::with_capacity(cfg.series.iter().map(|(_, v)| v.len()).sum::<usize>() * 260 + 2048);
    push_b(&mut b, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut b, cfg.width);
    push_b(&mut b, b"\" height=\"");
    push_i(&mut b, cfg.height);
    push_b(&mut b, b"\" viewBox=\"0 0 ");
    push_i(&mut b, cfg.width);
    push_b(&mut b, b" ");
    push_i(&mut b, cfg.height);
    push_b(&mut b, b"\">");
    push_b(&mut b, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    if !cfg.title.is_empty() {
        push_b(&mut b, b"<text x=\"");
        push_i(&mut b, cfg.width / 2);
        push_b(&mut b, b"\" y=\"26\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(&mut b, cfg.title);
        push_b(&mut b, b"</text>");
    }

    let mut idx = 0i32;
    for (fi, (name, values)) in cfg.series.iter().enumerate() {
        let stages = cfg.stage_labels.get(fi).cloned().unwrap_or_default();
        let n = values.len();
        if n == 0 {
            continue;
        }
        let max_val = values.iter().copied().fold(0.0_f64, f64::max).max(1e-12);
        let total: f64 = values.iter().sum();
        let initial = values[0];
        let col_x0 = pad_l + fi as i32 * (col_w + col_gap);

        push_b(&mut b, b"<text x=\"");
        push_i(&mut b, col_x0 + col_w / 2);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, pad_t - 14);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"12\" font-weight=\"700\" fill=\"#374151\">");
        escape_xml(&mut b, name);
        push_b(&mut b, b"</text>");

        let color = hex6(palette_color(cfg.palette, fi));
        let step_h = (plot_h - 2 * (n as i32 - 1).max(0)) / n as i32;
        let mut prev_ratio = 1.0_f64;
        for i in 0..n {
            let ratio = values[i] / max_val;
            let y = pad_t + i as i32 * (step_h + 2);
            let top_w = (prev_ratio * col_w as f64) as i32;
            let bot_w = (ratio * col_w as f64) as i32;
            let cx = col_x0 + col_w / 2;
            draw_trapezoid(&mut b, idx, name, cx, y, top_w, bot_w, step_h, &color);
            idx += 1;
            if cfg.show_text {
                let previous = if i > 0 { values[i - 1] } else { initial };
                push_b(&mut b, b"<text class=\"sp-val\" x=\"");
                push_i(&mut b, cx);
                push_b(&mut b, b"\" y=\"");
                push_i(&mut b, y + step_h / 2 - 2);
                push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9.5\" font-weight=\"700\" fill=\"#ffffff\" pointer-events=\"none\">");
                stage_text(&mut b, cfg.text_info, values[i], initial, previous, total);
                push_b(&mut b, b"</text>");
                if let Some(lbl) = stages.get(i) {
                    push_b(&mut b, b"<text x=\"");
                    push_i(&mut b, cx);
                    push_b(&mut b, b"\" y=\"");
                    push_i(&mut b, y + step_h / 2 + 10);
                    push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8.5\" fill=\"#ffffff\" fill-opacity=\"0.9\" pointer-events=\"none\">");
                    escape_xml(&mut b, truncate(lbl, 16));
                    push_b(&mut b, b"</text>");
                }
            }
            prev_ratio = ratio;
        }
    }

    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}

#[allow(clippy::too_many_arguments)]
fn draw_trapezoid(
    buf: &mut Vec<u8>,
    idx: i32,
    name: &str,
    cx: i32,
    y: i32,
    top_w: i32,
    bot_w: i32,
    step_h: i32,
    color_hex: &[u8; 6],
) {
    let xt0 = cx - top_w / 2;
    let xt1 = cx + top_w / 2;
    let xb0 = cx - bot_w / 2;
    let xb1 = cx + bot_w / 2;
    push_b(buf, b"<path data-idx=\"");
    push_i(buf, idx);
    push_b(buf, b"\" data-lbl=\"");
    escape_xml(buf, name);
    push_b(buf, b"\" data-group=\"");
    escape_xml(buf, name);
    push_b(buf, b"\" d=\"M ");
    push_i(buf, xt0);
    buf.push(b' ');
    push_i(buf, y);
    push_b(buf, b" L ");
    push_i(buf, xt1);
    buf.push(b' ');
    push_i(buf, y);
    push_b(buf, b" L ");
    push_i(buf, xb1);
    buf.push(b' ');
    push_i(buf, y + step_h);
    push_b(buf, b" L ");
    push_i(buf, xb0);
    buf.push(b' ');
    push_i(buf, y + step_h);
    push_b(buf, b" Z\" fill=\"#");
    buf.extend_from_slice(color_hex);
    push_b(buf, b"\" opacity=\"0.9\" stroke=\"#fff\" stroke-width=\"1\"/>");
}
