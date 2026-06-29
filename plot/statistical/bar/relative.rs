use super::block3d::Bar3DBlock;
use super::config::BarConfig;
use crate::plot::statistical::common::{
    escape_xml, palette_color, push_b, push_hex, push_i, Frame,
};

pub fn layout_3d(cfg: &BarConfig) -> Vec<Bar3DBlock> {
    let n_cats = cfg.category_labels.len();
    let n_ser = cfg.series.len();
    if n_cats == 0 || n_ser == 0 {
        return Vec::new();
    }
    let mut out = Vec::new();
    for ci in 0..n_cats {
        let mut pos_acc = 0.0;
        let mut neg_acc = 0.0;
        for (si, (_, vals)) in cfg.series.iter().enumerate() {
            let v = vals.get(ci).copied().unwrap_or(0.0);
            if !v.is_finite() {
                continue;
            }
            let (z0, z1) = if v >= 0.0 {
                let z0 = pos_acc;
                pos_acc += v;
                (z0, pos_acc)
            } else {
                let z1 = neg_acc;
                neg_acc += v;
                (neg_acc, z1)
            };
            out.push(Bar3DBlock::new(ci as f64, 0.0, z0, z1, 0.32, 0.32, si));
        }
    }
    out
}

#[crate::chart_demo("labels=[\"Q1\",\"Q2\",\"Q3\",\"Q4\"], series=[[24,38,17,42],[18,29,33,21],[12,15,28,30]], series_names=[\"Revenue\",\"Cost\",\"Tax\"]")]

pub fn render(cfg: &BarConfig) -> String {
    let n_cats = cfg.category_labels.len();
    let n_ser = cfg.series.len();
    if n_cats == 0 || n_ser == 0 {
        return String::new();
    }

    let mut pos_sum = vec![0.0f64; n_cats];
    let mut neg_sum = vec![0.0f64; n_cats];
    for (_, vals) in cfg.series.iter() {
        for ci in 0..n_cats {
            let v = vals.get(ci).copied().unwrap_or(0.0);
            if v.is_finite() {
                if v >= 0.0 {
                    pos_sum[ci] += v;
                } else {
                    neg_sum[ci] += v;
                }
            }
        }
    }
    let y_max = pos_sum.iter().cloned().fold(0.0f64, f64::max).max(1.0);
    let y_min = neg_sum.iter().cloned().fold(0.0f64, f64::min).min(0.0);
    let y_range = y_max - y_min;

    let legend_w = 160;
    let mut f = Frame::new_html(
        cfg.title,
        cfg.width,
        cfg.height,
        56,
        42,
        52,
        legend_w,
        n_cats * n_ser * 220 + 4096,
    );
    f.open(cfg.title, true);
    f.y_grid(6, y_min, y_max, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let zero_y = f.pt + (((y_max - 0.0) / y_range) * f.ph as f64) as i32;
    push_b(&mut f.buf, b"<line x1=\"");
    push_i(&mut f.buf, f.pl);
    push_b(&mut f.buf, b"\" y1=\"");
    push_i(&mut f.buf, zero_y);
    push_b(&mut f.buf, b"\" x2=\"");
    push_i(&mut f.buf, f.pl + f.pw);
    push_b(&mut f.buf, b"\" y2=\"");
    push_i(&mut f.buf, zero_y);
    push_b(&mut f.buf, b"\" stroke=\"#94a3b8\" stroke-width=\"1\"/>");

    let group_w = f.pw as f64 / n_cats as f64;
    let bar_w = (group_w * (1.0 - cfg.bar_gap)) as i32;

    for (ci, cat) in cfg.category_labels.iter().enumerate() {
        let cx = f.pl + (ci as f64 * group_w + group_w / 2.0) as i32;
        let bx = cx - bar_w / 2;
        let mut pos_acc = 0.0f64;
        let mut neg_acc = 0.0f64;
        for (si, (_, vals)) in cfg.series.iter().enumerate() {
            let v = vals.get(ci).copied().unwrap_or(0.0);
            if !v.is_finite() {
                continue;
            }
            let color = palette_color(cfg.palette, si);
            let (top_v, bot_v) = if v >= 0.0 {
                let t = pos_acc + v;
                let b = pos_acc;
                pos_acc += v;
                (t, b)
            } else {
                let t = neg_acc;
                let b = neg_acc + v;
                neg_acc += v;
                (t, b)
            };
            let y_top = f.pt + (((y_max - top_v) / y_range) * f.ph as f64) as i32;
            let y_bot = f.pt + (((y_max - bot_v) / y_range) * f.ph as f64) as i32;
            let h = (y_bot - y_top).max(1);
            push_b(&mut f.buf, b"<rect x=\"");
            push_i(&mut f.buf, bx);
            push_b(&mut f.buf, b"\" y=\"");
            push_i(&mut f.buf, y_top);
            push_b(&mut f.buf, b"\" width=\"");
            push_i(&mut f.buf, bar_w);
            push_b(&mut f.buf, b"\" height=\"");
            push_i(&mut f.buf, h);
            if cfg.corner_radius > 0 {
                push_b(&mut f.buf, b"\" rx=\"");
                push_i(&mut f.buf, cfg.corner_radius);
            }
            push_b(&mut f.buf, b"\" fill=\"");
            push_hex(&mut f.buf, color);
            push_b(&mut f.buf, b"\" stroke=\"#fff\" stroke-width=\"0.5\"/>");
        }
        push_b(&mut f.buf, b"<text x=\"");
        push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, f.pt + f.ph + 16);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#475569\">");
        escape_xml(&mut f.buf, cat);
        push_b(&mut f.buf, b"</text>");
    }

    let names: Vec<&str> = cfg.series.iter().map(|(n, _)| n.as_str()).collect();
    f.legend_pos(&names, cfg.palette, cfg.legend_position);
    f.html("[]")
}
