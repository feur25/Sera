use super::common::{color_hex, cx_at, data_attrs, finalize, open_with_axes, prepare, val_to_y};
use super::config::CandlestickConfig;
use crate::plot::statistical::common::{escape_xml, push_b, push_f2, push_i};

#[crate::chart_demo(
    "labels=[\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\",\"Sat\",\"Sun\"], open=[100,102,105,103,108,111,109], high=[105,107,109,110,114,116,113], low=[99,101,103,102,107,109,106], close=[102,105,103,108,112,110,107], volume=[1200,900,1500,700,2100,1800,1000], variant=\"volume\""
)]

pub fn render(cfg: &CandlestickConfig) -> String {
    let mut p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let n = p.n;
    let has_vol = cfg.volume.len() >= n;
    let gap = 10;
    let orig_h = p.layout.plot_h;
    let vol_h = if has_vol { (orig_h as f64 * 0.22) as i32 } else { 0 };
    let candle_h = (orig_h - vol_h - if has_vol { gap } else { 0 }).max(10);
    p.layout.plot_h = candle_h;

    let mut b = Vec::<u8>::with_capacity(n * 300 + 4096);
    open_with_axes(&mut b, cfg, &p);
    let l = &p.layout;
    let bw = l.body_w;
    for i in 0..n {
        let cx = cx_at(l, i);
        let y_high = val_to_y(l, p.high[i]);
        let y_low = val_to_y(l, p.low[i]);
        let y_open = val_to_y(l, p.open[i]);
        let y_close = val_to_y(l, p.close[i]);
        let up = p.close[i] >= p.open[i];
        let col = if up { p.up_color } else { p.dn_color };
        let hx = color_hex(col);
        let (top, bot) = if y_open < y_close {
            (y_open, y_close)
        } else {
            (y_close, y_open)
        };
        let bh = (bot - top).max(1);
        push_b(&mut b, b"<line");
        data_attrs(&mut b, &p, i);
        push_b(&mut b, b" x1=\"");
        push_i(&mut b, cx);
        push_b(&mut b, b"\" y1=\"");
        push_i(&mut b, y_high);
        push_b(&mut b, b"\" x2=\"");
        push_i(&mut b, cx);
        push_b(&mut b, b"\" y2=\"");
        push_i(&mut b, y_low);
        push_b(&mut b, b"\" stroke=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"1.4\"/>");
        push_b(&mut b, b"<rect");
        data_attrs(&mut b, &p, i);
        push_b(&mut b, b" x=\"");
        push_i(&mut b, cx - bw / 2);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, top);
        push_b(&mut b, b"\" width=\"");
        push_i(&mut b, bw);
        push_b(&mut b, b"\" height=\"");
        push_i(&mut b, bh);
        push_b(&mut b, b"\" rx=\"1\" fill=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\"/>");
    }

    if has_vol {
        let vol_top = l.pad_t + candle_h + gap;
        let vmax = cfg.volume[..n]
            .iter()
            .cloned()
            .fold(0.0_f64, f64::max)
            .max(1.0);
        push_b(&mut b, b"<line x1=\"");
        push_i(&mut b, l.pad_l);
        push_b(&mut b, b"\" y1=\"");
        push_i(&mut b, vol_top);
        push_b(&mut b, b"\" x2=\"");
        push_i(&mut b, l.pad_l + l.plot_w);
        push_b(&mut b, b"\" y2=\"");
        push_i(&mut b, vol_top);
        push_b(&mut b, b"\" stroke=\"#e5e7eb\" stroke-width=\"0.6\"/>");
        for i in 0..n {
            let cx = cx_at(l, i);
            let v = cfg.volume[i].max(0.0);
            let bh = ((v / vmax) * vol_h as f64).max(1.0) as i32;
            let up = p.close[i] >= p.open[i];
            let col = if up { p.up_color } else { p.dn_color };
            let hx = color_hex(col);
            push_b(&mut b, b"<rect data-idx=\"");
            push_i(&mut b, i as i32);
            push_b(&mut b, b"\" data-lbl=\"");
            escape_xml(&mut b, &p.labels[i]);
            push_b(&mut b, b"\" data-kv-Volume=\"");
            push_f2(&mut b, v);
            push_b(&mut b, b"\" x=\"");
            push_i(&mut b, cx - bw / 2);
            push_b(&mut b, b"\" y=\"");
            push_i(&mut b, vol_top + vol_h - bh);
            push_b(&mut b, b"\" width=\"");
            push_i(&mut b, bw);
            push_b(&mut b, b"\" height=\"");
            push_i(&mut b, bh);
            push_b(&mut b, b"\" fill=\"#");
            b.extend_from_slice(&hx);
            push_b(&mut b, b"\" fill-opacity=\"0.55\"/>");
        }
    }
    finalize(b, cfg)
}
