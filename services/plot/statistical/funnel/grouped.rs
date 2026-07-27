use super::config::FunnelConfig;
use crate::html::hover::{build_chart_html, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i, truncate};

#[crate::chart_demo(
    "labels=[\"Website visit\",\"Downloads\",\"Potential customers\",\"Requested price\",\"invoice sent\"], series=[[39,27.4,20.6,11,3],[52,36,18,14,5]], series_names=[\"Montreal\",\"Toronto\"], variant=\"grouped\""
)]

pub fn render(cfg: &FunnelConfig) -> String {
    let n_stages = cfg.labels.len();
    let n_groups = cfg.series.len();
    if n_stages == 0 || n_groups == 0 {
        return String::new();
    }

    let row_totals: Vec<f64> = (0..n_stages)
        .map(|i| {
            cfg.series
                .iter()
                .map(|(_, v)| v.get(i).copied().unwrap_or(0.0).max(0.0))
                .sum::<f64>()
        })
        .collect();
    let max_total = row_totals.iter().copied().fold(0.0_f64, f64::max).max(1e-12);

    let pad_l = 150i32;
    let pad_r = 130i32;
    let pad_t = 20i32;
    let pad_b = 20i32;
    let plot_w = (cfg.width - pad_l - pad_r).max(1);
    let plot_h = (cfg.height - pad_t - pad_b).max(1);
    let gap = 3i32;
    let step_h = (plot_h - gap * (n_stages as i32 - 1).max(0)) / n_stages as i32;
    let cx = pad_l + plot_w / 2;
    let scale = plot_w as f64 / max_total;

    let mut b = Vec::<u8>::with_capacity(n_stages * n_groups * 220 + 2048);
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
    let mut prev_vals: Vec<f64> = cfg
        .series
        .iter()
        .map(|(_, v)| v.first().copied().unwrap_or(0.0).max(0.0))
        .collect();

    for i in 0..n_stages {
        let y0 = pad_t + i as i32 * (step_h + gap);
        let y1 = y0 + step_h;
        let top_total = prev_vals.iter().sum::<f64>() * scale;
        let bot_total = row_totals[i] * scale;
        let mut cursor_top = cx as f64 - top_total / 2.0;
        let mut cursor_bot = cx as f64 - bot_total / 2.0;

        for (g, (name, vals)) in cfg.series.iter().enumerate() {
            let cur_val = vals.get(i).copied().unwrap_or(0.0).max(0.0);
            let top_w = prev_vals[g] * scale;
            let bot_w = cur_val * scale;
            let color = hex6(palette_color(cfg.palette, g));

            push_b(&mut b, b"<path data-idx=\"");
            push_i(&mut b, idx);
            push_b(&mut b, b"\" data-lbl=\"");
            escape_xml(&mut b, name);
            push_b(&mut b, b"\" data-group=\"");
            escape_xml(&mut b, name);
            push_b(&mut b, b"\" data-v=\"");
            push_f2(&mut b, cur_val);
            push_b(&mut b, b"\" d=\"M");
            push_f2(&mut b, cursor_top);
            b.push(b',');
            push_i(&mut b, y0);
            push_b(&mut b, b" L");
            push_f2(&mut b, cursor_top + top_w);
            b.push(b',');
            push_i(&mut b, y0);
            push_b(&mut b, b" L");
            push_f2(&mut b, cursor_bot + bot_w);
            b.push(b',');
            push_i(&mut b, y1);
            push_b(&mut b, b" L");
            push_f2(&mut b, cursor_bot);
            b.push(b',');
            push_i(&mut b, y1);
            push_b(&mut b, b" Z\" fill=\"#");
            b.extend_from_slice(&color);
            push_b(&mut b, b"\" opacity=\"0.9\"/>");

            if cfg.show_text && bot_w > 16.0 {
                push_b(&mut b, b"<text class=\"sp-val\" x=\"");
                push_f2(&mut b, cursor_bot + bot_w / 2.0);
                push_b(&mut b, b"\" y=\"");
                push_i(&mut b, y0 + step_h / 2 + 4);
                push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"11\" font-weight=\"700\" fill=\"#fff\" pointer-events=\"none\">");
                push_f2(&mut b, cur_val);
                push_b(&mut b, b"</text>");
            }

            cursor_top += top_w;
            cursor_bot += bot_w;
            prev_vals[g] = cur_val;
            idx += 1;
        }

        push_b(&mut b, b"<text x=\"");
        push_i(&mut b, pad_l - 12);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, y0 + step_h / 2 + 4);
        push_b(&mut b, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">");
        escape_xml(&mut b, truncate(&cfg.labels[i], 22));
        push_b(&mut b, b"</text>");
    }

    let leg_x = cfg.width - pad_r + 24;
    let mut leg_y = pad_t + 10;
    push_b(&mut b, b"<text x=\"");
    push_i(&mut b, leg_x);
    push_b(&mut b, b"\" y=\"");
    push_i(&mut b, leg_y);
    push_b(&mut b, b"\" font-family=\"Arial,sans-serif\" font-size=\"11\" font-weight=\"700\" fill=\"#374151\">group</text>");
    leg_y += 20;
    for (g, (name, _)) in cfg.series.iter().enumerate() {
        let color = hex6(palette_color(cfg.palette, g));
        push_b(&mut b, b"<rect x=\"");
        push_i(&mut b, leg_x);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, leg_y - 9);
        push_b(&mut b, b"\" width=\"11\" height=\"11\" rx=\"2\" fill=\"#");
        b.extend_from_slice(&color);
        push_b(&mut b, b"\"/>");
        push_b(&mut b, b"<text x=\"");
        push_i(&mut b, leg_x + 16);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, leg_y);
        push_b(&mut b, b"\" font-family=\"Arial,sans-serif\" font-size=\"10.5\" fill=\"#374151\">");
        escape_xml(&mut b, truncate(name, 16));
        push_b(&mut b, b"</text>");
        leg_y += 20;
    }

    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}
