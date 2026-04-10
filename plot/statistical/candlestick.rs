use super::common::{palette_color, push_b, push_i, push_f2, escape_xml, hex6, truncate, svg_open, svg_title, svg_hgrid, svg_tick_y, svg_tick_x, svg_axis_lines, svg_x_label, svg_y_label};
use crate::html::hover::{HoverSlot, slots_to_json, build_chart_html};

pub struct CandlestickConfig<'a> {
    pub title: &'a str,
    pub labels: &'a [String],
    pub open: &'a [f64],
    pub high: &'a [f64],
    pub low: &'a [f64],
    pub close: &'a [f64],
    pub palette: &'a [u32],
    pub width: i32,
    pub height: i32,
    pub x_label: &'a str,
    pub y_label: &'a str,
    pub gridlines: bool,
    pub hover: &'a [HoverSlot],
}

impl<'a> Default for CandlestickConfig<'a> {
    fn default() -> Self {
        Self {
            title: "", labels: &[], open: &[], high: &[], low: &[], close: &[],
            palette: &[], width: 1100, height: 500,
            x_label: "Date", y_label: "Price",
            gridlines: false, hover: &[],
        }
    }
}

pub fn render_candlestick_html(cfg: &CandlestickConfig) -> String {
    let n = cfg.labels.len().min(cfg.open.len()).min(cfg.high.len()).min(cfg.low.len()).min(cfg.close.len());
    if n == 0 { return String::new(); }

    let mut global_min = f64::INFINITY;
    let mut global_max = f64::NEG_INFINITY;
    for i in 0..n {
        if cfg.low[i] < global_min { global_min = cfg.low[i]; }
        if cfg.high[i] > global_max { global_max = cfg.high[i]; }
    }
    let pad_val = (global_max - global_min) * 0.08;
    global_min -= pad_val;
    global_max += pad_val;
    let val_range = (global_max - global_min).max(1.0);

    let up_color: u32 = if cfg.palette.len() >= 2 { cfg.palette[0] } else { 0x22C55E };
    let dn_color: u32 = if cfg.palette.len() >= 2 { cfg.palette[1] } else { 0xEF4444 };

    let pad_l: i32 = 68;
    let pad_t: i32 = 38;
    let pad_b: i32 = 52;
    let pad_r: i32 = 20;
    let plot_w = cfg.width - pad_l - pad_r;
    let plot_h = cfg.height - pad_t - pad_b;

    let mut buf = Vec::<u8>::with_capacity(n * 400 + 2048);
    svg_open(&mut buf, cfg.width, cfg.height);
    svg_title(&mut buf, cfg.title, pad_l + plot_w / 2, 26);

    let n_yticks = 8i32;
    for i in 0..=n_yticks {
        let frac = i as f64 / n_yticks as f64;
        let y = pad_t + ((1.0 - frac) * plot_h as f64) as i32;
        let val = global_min + frac * val_range;
        if cfg.gridlines && i > 0 {
            push_b(&mut buf, b"<line class=\"sp-gl\" x1=\""); push_i(&mut buf, pad_l);
            push_b(&mut buf, b"\" y1=\""); push_i(&mut buf, y);
            push_b(&mut buf, b"\" x2=\""); push_i(&mut buf, pad_l + plot_w);
            push_b(&mut buf, b"\" y2=\""); push_i(&mut buf, y);
            push_b(&mut buf, b"\" stroke=\"#e5e7eb\" stroke-width=\"0.5\"/>");
        }
        push_b(&mut buf, b"<text class=\"sp-yt\" x=\""); push_i(&mut buf, pad_l - 6);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, y + 3);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\">");
        if val >= 1000.0 { push_i(&mut buf, val as i32); } else { push_f2(&mut buf, val); }
        push_b(&mut buf, b"</text>");
    }

    svg_axis_lines(&mut buf, pad_l, pad_t, plot_w, plot_h);
    svg_x_label(&mut buf, cfg.x_label, pad_l + plot_w / 2, pad_t + plot_h + 42);
    svg_y_label(&mut buf, cfg.y_label, 14, pad_t, plot_h);

    let candle_pitch = plot_w as f64 / n as f64;
    let body_w = (candle_pitch * 0.6).max(3.0).min(20.0) as i32;

    for i in 0..n {
        let is_up = cfg.close[i] >= cfg.open[i];
        let color = if is_up { up_color } else { dn_color };
        let hx = hex6(color);

        let cx = pad_l + (i as f64 * candle_pitch + candle_pitch / 2.0) as i32;

        let wick_top = pad_t + ((1.0 - (cfg.high[i] - global_min) / val_range) * plot_h as f64) as i32;
        let wick_bot = pad_t + ((1.0 - (cfg.low[i] - global_min) / val_range) * plot_h as f64) as i32;

        let body_top_val = if is_up { cfg.close[i] } else { cfg.open[i] };
        let body_bot_val = if is_up { cfg.open[i] } else { cfg.close[i] };
        let body_top = pad_t + ((1.0 - (body_top_val - global_min) / val_range) * plot_h as f64) as i32;
        let body_bot = pad_t + ((1.0 - (body_bot_val - global_min) / val_range) * plot_h as f64) as i32;
        let body_h = (body_bot - body_top).max(1);

        push_b(&mut buf, b"<line x1=\""); push_i(&mut buf, cx);
        push_b(&mut buf, b"\" y1=\""); push_i(&mut buf, wick_top);
        push_b(&mut buf, b"\" x2=\""); push_i(&mut buf, cx);
        push_b(&mut buf, b"\" y2=\""); push_i(&mut buf, wick_bot);
        push_b(&mut buf, b"\" stroke=\"#"); buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke-width=\"1.5\"/>");

        push_b(&mut buf, b"<rect data-idx=\""); push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\" data-lbl=\""); escape_xml(&mut buf, &cfg.labels[i]);
        push_b(&mut buf, b"\" data-kv-Open=\""); push_f2(&mut buf, cfg.open[i]);
        push_b(&mut buf, b"\" data-kv-High=\""); push_f2(&mut buf, cfg.high[i]);
        push_b(&mut buf, b"\" data-kv-Low=\""); push_f2(&mut buf, cfg.low[i]);
        push_b(&mut buf, b"\" data-kv-Close=\""); push_f2(&mut buf, cfg.close[i]);
        push_b(&mut buf, b"\" x=\""); push_i(&mut buf, cx - body_w / 2);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, body_top);
        push_b(&mut buf, b"\" width=\""); push_i(&mut buf, body_w);
        push_b(&mut buf, b"\" height=\""); push_i(&mut buf, body_h);
        if is_up {
            push_b(&mut buf, b"\" fill=\"#"); buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stroke=\"#"); buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stroke-width=\"1\"/>");
        } else {
            push_b(&mut buf, b"\" fill=\"#"); buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" rx=\"1\"/>");
        }

        if n <= 60 && i % ((n / 12).max(1)) == 0 {
            push_b(&mut buf, b"<text x=\""); push_i(&mut buf, cx);
            push_b(&mut buf, b"\" y=\""); push_i(&mut buf, pad_t + plot_h + 14);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#6b7280\">");
            escape_xml(&mut buf, truncate(&cfg.labels[i], 10));
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let slots_json;
    let json: &str = if cfg.hover.is_empty() { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    build_chart_html(cfg.title, &svg, json)
}
