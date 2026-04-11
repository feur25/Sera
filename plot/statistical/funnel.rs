use super::common::{palette_color, push_b, push_i, push_f2, escape_xml, hex6, apply_sort};
use crate::html::hover::{build_chart_html, slots_to_json};

crate::chart_config!(FunnelConfig, 800, 480;
    struct {
        pub labels: &'a [String],
        pub values: &'a [f64],
        pub palette: &'a [u32],
        pub show_text: bool,
    }
    defaults {
        labels: &[],
        values: &[],
        palette: &[],
        show_text: true,
    }
);

pub fn render_funnel_html(cfg: &FunnelConfig) -> String {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 { return String::new(); }
    let (labels, values) = apply_sort(&cfg.labels[..n], &cfg.values[..n], cfg.sort_order);
    let max_val = values.iter().copied().fold(0.0_f64, f64::max).max(1.0);
    let pad_l: i32 = 80;
    let pad_r: i32 = 80;
    let pad_t: i32 = 46;
    let pad_b: i32 = 20;
    let plot_w = cfg.width - pad_l - pad_r;
    let plot_h = cfg.height - pad_t - pad_b;
    let step_h = plot_h / n as i32;
    let gap: i32 = 3;
    let mut b = Vec::<u8>::with_capacity(n * 200 + 1024);
    push_b(&mut b, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut b, cfg.width); push_b(&mut b, b"\" height=\"");
    push_i(&mut b, cfg.height); push_b(&mut b, b"\" viewBox=\"0 0 ");
    push_i(&mut b, cfg.width); push_b(&mut b, b" ");
    push_i(&mut b, cfg.height); push_b(&mut b, b"\">");
    push_b(&mut b, b"<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>");
    if !cfg.title.is_empty() {
        push_b(&mut b, b"<text x=\""); push_i(&mut b, cfg.width / 2);
        push_b(&mut b, b"\" y=\"26\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(&mut b, cfg.title);
        push_b(&mut b, b"</text>");
    }
    let cx = cfg.width / 2;
    for i in 0..n {
        let ratio = values[i] / max_val;
        let row_w = (plot_w as f64 * ratio) as i32;
        let top_w = if i > 0 {
            let prev_ratio = values[i - 1] / max_val;
            (plot_w as f64 * prev_ratio) as i32
        } else {
            row_w
        };
        let y_top = pad_t + i as i32 * step_h;
        let y_bot = y_top + step_h - gap;
        let x_top_l = cx - top_w / 2;
        let x_top_r = cx + top_w / 2;
        let x_bot_l = cx - row_w / 2;
        let x_bot_r = cx + row_w / 2;
        let color = palette_color(cfg.palette, i);
        let hx = hex6(color);
        push_b(&mut b, b"<polygon data-idx=\"");
        push_i(&mut b, i as i32);
        push_b(&mut b, b"\" data-y=\""); push_f2(&mut b, values[i]);
        push_b(&mut b, b"\" data-lbl=\""); escape_xml(&mut b, &labels[i]);
        push_b(&mut b, b"\" points=\"");
        push_i(&mut b, x_top_l); b.push(b','); push_i(&mut b, y_top);
        b.push(b' ');
        push_i(&mut b, x_top_r); b.push(b','); push_i(&mut b, y_top);
        b.push(b' ');
        push_i(&mut b, x_bot_r); b.push(b','); push_i(&mut b, y_bot);
        b.push(b' ');
        push_i(&mut b, x_bot_l); b.push(b','); push_i(&mut b, y_bot);
        push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" opacity=\"0.92\" stroke=\"#fff\" stroke-width=\"1.5\" stroke-linejoin=\"round\"/>");
        let label_y = y_top + step_h / 2 + 4;
        push_b(&mut b, b"<text x=\""); push_i(&mut b, cx);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, label_y);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"12\" font-weight=\"600\" fill=\"#fff\" pointer-events=\"none\">");
        escape_xml(&mut b, &labels[i]);
        push_b(&mut b, b"</text>");
        if cfg.show_text {
            push_b(&mut b, b"<text x=\""); push_i(&mut b, cx - row_w / 2 - 6);
            push_b(&mut b, b"\" y=\""); push_i(&mut b, label_y);
            push_b(&mut b, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\" pointer-events=\"none\">");
            let pct = values[i] / max_val * 100.0;
            push_f2(&mut b, pct); push_b(&mut b, b"%");
            push_b(&mut b, b"</text>");
            push_b(&mut b, b"<text x=\""); push_i(&mut b, cx + row_w / 2 + 6);
            push_b(&mut b, b"\" y=\""); push_i(&mut b, label_y);
            push_b(&mut b, b"\" text-anchor=\"start\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\" pointer-events=\"none\">");
            if values[i] >= 1_000_000.0 {
                push_f2(&mut b, values[i] / 1_000_000.0);
                push_b(&mut b, b"M");
            } else if values[i] >= 1_000.0 {
                push_f2(&mut b, values[i] / 1_000.0);
                push_b(&mut b, b"k");
            } else {
                push_f2(&mut b, values[i]);
            }
            push_b(&mut b, b"</text>");
        }
    }
    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}
