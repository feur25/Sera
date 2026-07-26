use super::config::BarConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::boxplot::common::{compute_box, global_range};
use crate::plot::statistical::common::{escape_xml, palette_color, push_b, push_f2, push_i, Frame};

#[crate::chart_demo(
    "labels=[\"Control\",\"Treatment A\",\"Treatment B\"], series=[[23,25,19,30,22,27,24],[15,18,20,22,17,19,16],[30,32,28,35,31,29,33]], variant=\"distribution\""
)]

pub fn render(cfg: &BarConfig) -> String {
    let n_cats = cfg.category_labels.len().min(cfg.series.len());
    if n_cats == 0 {
        return String::new();
    }
    let dists: Vec<&Vec<f64>> = (0..n_cats).map(|i| &cfg.series[i].1).collect();
    let stats: Vec<_> = dists.iter().map(|d| compute_box(d)).collect();
    let range = global_range(&stats);
    let y_max = range.y_max.max(0.0);
    let y_min = range.y_min.min(0.0);
    let y_range = (y_max - y_min).max(1e-9);

    let legend_w = 0;
    let mut f = Frame::new_html(
        cfg.title,
        cfg.width,
        cfg.height,
        56,
        42,
        52,
        legend_w,
        n_cats * 400 + 4096,
    );
    f.open(cfg.title, true);
    f.y_grid(6, y_min, y_max, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let group_w = f.pw as f64 / n_cats as f64;
    let bar_w = group_w * (1.0 - cfg.bar_gap.max(0.1));
    let box_w = bar_w * 0.42;
    let base_y = f.pt as f64 + ((y_max - 0.0) / y_range) * f.ph as f64;

    let y_of = |v: f64| f.pt as f64 + ((y_max - v) / y_range) * f.ph as f64;

    for (ci, cat) in cfg.category_labels[..n_cats].iter().enumerate() {
        let cx = f.pl as f64 + ci as f64 * group_w + group_w / 2.0;
        let s = &stats[ci];
        let color = palette_color(cfg.palette, ci);
        let hx = crate::plot::statistical::common::hex6(color);

        let mean_y = y_of(s.mean);
        let (bar_top, bar_h) = if s.mean >= 0.0 {
            (mean_y, base_y - mean_y)
        } else {
            (base_y, mean_y - base_y)
        };
        push_b(&mut f.buf, b"<rect data-idx=\"");
        push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b"\" data-lbl=\"");
        escape_xml(&mut f.buf, cat);
        push_b(&mut f.buf, b"\" data-v=\"");
        push_f2(&mut f.buf, s.mean);
        push_b(&mut f.buf, b"\" data-kv-Mean=\"");
        push_f2(&mut f.buf, s.mean);
        push_b(&mut f.buf, b"\" data-kv-Median=\"");
        push_f2(&mut f.buf, s.median);
        push_b(&mut f.buf, b"\" data-kv-Q1=\"");
        push_f2(&mut f.buf, s.q1);
        push_b(&mut f.buf, b"\" data-kv-Q3=\"");
        push_f2(&mut f.buf, s.q3);
        push_b(&mut f.buf, b"\" data-kv-N=\"");
        push_i(&mut f.buf, s.n as i32);
        push_b(&mut f.buf, b"\" x=\"");
        push_f2(&mut f.buf, cx - bar_w / 2.0);
        push_b(&mut f.buf, b"\" y=\"");
        push_f2(&mut f.buf, bar_top);
        push_b(&mut f.buf, b"\" width=\"");
        push_f2(&mut f.buf, bar_w);
        push_b(&mut f.buf, b"\" height=\"");
        push_f2(&mut f.buf, bar_h.max(1.0));
        push_b(&mut f.buf, b"\" fill=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.32\" rx=\"3\"/>");

        let y_whisk_lo = y_of(s.whisker_lo);
        let y_whisk_hi = y_of(s.whisker_hi);
        push_b(&mut f.buf, b"<line x1=\"");
        push_f2(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y1=\"");
        push_f2(&mut f.buf, y_whisk_hi);
        push_b(&mut f.buf, b"\" x2=\"");
        push_f2(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y2=\"");
        push_f2(&mut f.buf, y_whisk_lo);
        push_b(&mut f.buf, b"\" stroke=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"1.4\"/>");

        let y_q1 = y_of(s.q1);
        let y_q3 = y_of(s.q3);
        let (box_top, box_h) = if y_q3 < y_q1 {
            (y_q3, y_q1 - y_q3)
        } else {
            (y_q1, y_q3 - y_q1)
        };
        push_b(&mut f.buf, b"<rect x=\"");
        push_f2(&mut f.buf, cx - box_w / 2.0);
        push_b(&mut f.buf, b"\" y=\"");
        push_f2(&mut f.buf, box_top);
        push_b(&mut f.buf, b"\" width=\"");
        push_f2(&mut f.buf, box_w);
        push_b(&mut f.buf, b"\" height=\"");
        push_f2(&mut f.buf, box_h.max(1.0));
        push_b(&mut f.buf, b"\" fill=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke=\"#1f2937\" stroke-width=\"1\"/>");

        let y_med = y_of(s.median);
        push_b(&mut f.buf, b"<line x1=\"");
        push_f2(&mut f.buf, cx - box_w / 2.0);
        push_b(&mut f.buf, b"\" y1=\"");
        push_f2(&mut f.buf, y_med);
        push_b(&mut f.buf, b"\" x2=\"");
        push_f2(&mut f.buf, cx + box_w / 2.0);
        push_b(&mut f.buf, b"\" y2=\"");
        push_f2(&mut f.buf, y_med);
        push_b(&mut f.buf, b"\" stroke=\"#1f2937\" stroke-width=\"1.6\"/>");

        push_b(&mut f.buf, b"<text x=\"");
        push_f2(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, f.pt + f.ph + 16);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#475569\">");
        escape_xml(&mut f.buf, cat);
        push_b(&mut f.buf, b"</text>");
    }

    f.html(&slots_to_json(cfg.hover))
}
