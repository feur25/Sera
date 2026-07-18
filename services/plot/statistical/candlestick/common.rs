use super::config::CandlestickConfig;
use crate::html::hover::{build_chart_html, slots_to_json};
use crate::plot::statistical::common::{
    escape_xml, hex6, push_b, push_f2, push_i, sort_indices, sorted, svg_axis_lines, svg_open,
    svg_title, svg_x_label, svg_y_label, truncate,
};

pub struct Layout {
    pub pad_l: i32,
    pub pad_t: i32,
    pub pad_b: i32,
    pub pad_r: i32,
    pub plot_w: i32,
    pub plot_h: i32,
    pub vmin: f64,
    pub vmax: f64,
    pub vrange: f64,
    pub pitch: f64,
    pub body_w: i32,
}

pub struct Prepared {
    pub n: usize,
    pub labels: Vec<String>,
    pub open: Vec<f64>,
    pub high: Vec<f64>,
    pub low: Vec<f64>,
    pub close: Vec<f64>,
    pub layout: Layout,
    pub up_color: u32,
    pub dn_color: u32,
}

pub fn prepare(cfg: &CandlestickConfig) -> Option<Prepared> {
    let n = cfg
        .labels
        .len()
        .min(cfg.open.len())
        .min(cfg.high.len())
        .min(cfg.low.len())
        .min(cfg.close.len());
    if n == 0 {
        return None;
    }
    let idx = sort_indices(n, cfg.close, cfg.labels, cfg.sort_order);
    let labels = sorted(&idx, cfg.labels);
    let open = sorted(&idx, cfg.open);
    let high = sorted(&idx, cfg.high);
    let low = sorted(&idx, cfg.low);
    let close = sorted(&idx, cfg.close);
    let mut vmin = f64::INFINITY;
    let mut vmax = f64::NEG_INFINITY;
    for i in 0..n {
        if low[i] < vmin {
            vmin = low[i];
        }
        if high[i] > vmax {
            vmax = high[i];
        }
    }
    let pad_val = (vmax - vmin) * 0.08;
    vmin -= pad_val;
    vmax += pad_val;
    let vrange = (vmax - vmin).max(1.0);
    let up_color: u32 = if cfg.palette.len() >= 2 {
        cfg.palette[0]
    } else {
        0x22C55E
    };
    let dn_color: u32 = if cfg.palette.len() >= 2 {
        cfg.palette[1]
    } else {
        0xEF4444
    };
    let pad_l: i32 = 68;
    let pad_t: i32 = 38;
    let pad_b: i32 = 52;
    let pad_r: i32 = 20;
    let plot_w = cfg.width - pad_l - pad_r;
    let plot_h = cfg.height - pad_t - pad_b;
    let pitch = plot_w as f64 / n as f64;
    let body_w = (pitch * 0.6).max(3.0).min(20.0) as i32;
    Some(Prepared {
        n,
        labels,
        open,
        high,
        low,
        close,
        layout: Layout {
            pad_l,
            pad_t,
            pad_b,
            pad_r,
            plot_w,
            plot_h,
            vmin,
            vmax,
            vrange,
            pitch,
            body_w,
        },
        up_color,
        dn_color,
    })
}

pub fn val_to_y(l: &Layout, v: f64) -> i32 {
    l.pad_t + ((1.0 - (v - l.vmin) / l.vrange) * l.plot_h as f64) as i32
}

pub fn cx_at(l: &Layout, i: usize) -> i32 {
    l.pad_l + (i as f64 * l.pitch + l.pitch / 2.0) as i32
}

pub fn open_with_axes(buf: &mut Vec<u8>, cfg: &CandlestickConfig, p: &Prepared) {
    let l = &p.layout;
    svg_open(buf, cfg.width, cfg.height);
    svg_title(buf, cfg.title, l.pad_l + l.plot_w / 2, 26);
    let n_yticks = 8i32;
    for i in 0..=n_yticks {
        let frac = i as f64 / n_yticks as f64;
        let y = l.pad_t + ((1.0 - frac) * l.plot_h as f64) as i32;
        let val = l.vmin + frac * l.vrange;
        if cfg.gridlines && i > 0 {
            push_b(buf, b"<line class=\"sp-gl\" x1=\"");
            push_i(buf, l.pad_l);
            push_b(buf, b"\" y1=\"");
            push_i(buf, y);
            push_b(buf, b"\" x2=\"");
            push_i(buf, l.pad_l + l.plot_w);
            push_b(buf, b"\" y2=\"");
            push_i(buf, y);
            push_b(buf, b"\" stroke=\"#e5e7eb\" stroke-width=\"0.5\"/>");
        }
        push_b(buf, b"<text class=\"sp-yt\" x=\"");
        push_i(buf, l.pad_l - 6);
        push_b(buf, b"\" y=\"");
        push_i(buf, y + 3);
        push_b(buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\">");
        if val >= 1000.0 {
            push_i(buf, val as i32);
        } else {
            push_f2(buf, val);
        }
        push_b(buf, b"</text>");
    }
    svg_axis_lines(buf, l.pad_l, l.pad_t, l.plot_w, l.plot_h);
    svg_x_label(
        buf,
        cfg.x_label,
        l.pad_l + l.plot_w / 2,
        l.pad_t + l.plot_h + 42,
    );
    svg_y_label(buf, cfg.y_label, 14, l.pad_t, l.plot_h);
    let n = p.n;
    if n <= 60 {
        for i in 0..n {
            if i % ((n / 12).max(1)) == 0 {
                push_b(buf, b"<text x=\"");
                push_i(buf, cx_at(l, i));
                push_b(buf, b"\" y=\"");
                push_i(buf, l.pad_t + l.plot_h + 14);
                push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#6b7280\" class=\"sp-xt\">");
                escape_xml(buf, truncate(&p.labels[i], 10));
                push_b(buf, b"</text>");
            }
        }
    }
}

pub fn data_attrs(buf: &mut Vec<u8>, p: &Prepared, i: usize) {
    push_b(buf, b" data-idx=\"");
    push_i(buf, i as i32);
    push_b(buf, b"\" data-lbl=\"");
    escape_xml(buf, &p.labels[i]);
    push_b(buf, b"\" data-kv-Open=\"");
    push_f2(buf, p.open[i]);
    push_b(buf, b"\" data-kv-High=\"");
    push_f2(buf, p.high[i]);
    push_b(buf, b"\" data-kv-Low=\"");
    push_f2(buf, p.low[i]);
    push_b(buf, b"\" data-kv-Close=\"");
    push_f2(buf, p.close[i]);
    push_b(buf, b"\"");
}

pub fn finalize(buf: Vec<u8>, cfg: &CandlestickConfig) -> String {
    let mut b = buf;
    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    let slots_json;
    let json: &str = if cfg.hover.is_empty() {
        "[]"
    } else {
        slots_json = slots_to_json(cfg.hover);
        &slots_json
    };
    build_chart_html(cfg.title, &svg, json)
}

pub fn color_hex(c: u32) -> [u8; 6] {
    hex6(c)
}

pub fn heikin_ashi(
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let n = open.len();
    let mut ho = vec![0.0; n];
    let mut hh = vec![0.0; n];
    let mut hl = vec![0.0; n];
    let mut hc = vec![0.0; n];
    if n == 0 {
        return (ho, hh, hl, hc);
    }
    hc[0] = (open[0] + high[0] + low[0] + close[0]) / 4.0;
    ho[0] = (open[0] + close[0]) / 2.0;
    hh[0] = high[0].max(ho[0]).max(hc[0]);
    hl[0] = low[0].min(ho[0]).min(hc[0]);
    for i in 1..n {
        hc[i] = (open[i] + high[i] + low[i] + close[i]) / 4.0;
        ho[i] = (ho[i - 1] + hc[i - 1]) / 2.0;
        hh[i] = high[i].max(ho[i]).max(hc[i]);
        hl[i] = low[i].min(ho[i]).min(hc[i]);
    }
    (ho, hh, hl, hc)
}
