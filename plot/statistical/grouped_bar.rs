use super::common::{palette_color, push_b, push_i, push_f2, escape_xml, hex6, truncate, svg_open_rescalable, svg_title, svg_hgrid, svg_tick_y, svg_axis_lines, svg_y_label, svg_x_label, svg_legend_item};
use crate::html::hover::{HoverSlot, slots_to_json, build_chart_html};

pub struct GroupedBar;

pub struct GroupedBarConfig<'a> {
    pub title: &'a str,
    pub x_label: &'a str,
    pub y_label: &'a str,
    pub category_labels: &'a [String],
    pub series: &'a [(String, Vec<f64>)],
    pub palette: &'a [u32],
    pub width: i32,
    pub height: i32,
    pub stacked: bool,
    pub show_values: bool,
    pub value_min_height: i32,
    pub gridlines: bool,
    pub hover: &'a [HoverSlot],
    pub sort_order: &'a str,
}

impl<'a> Default for GroupedBarConfig<'a> {
    fn default() -> Self {
        Self {
            title: "",
            x_label: "",
            y_label: "",
            category_labels: &[],
            series: &[],
            palette: &[],
            width: 1100,
            height: 480,
            stacked: false,
            show_values: false,
            value_min_height: 16,
            gridlines: true,
            hover: &[],
            sort_order: "",
        }
    }
}

pub fn render_grouped_bar_html(cfg: &GroupedBarConfig) -> String {
    let n_cats = cfg.category_labels.len();
    let n_ser = cfg.series.len();
    if n_cats == 0 || n_ser == 0 { return String::new(); }
    let so = cfg.sort_order;
    let (cat_labels, sorted_series) = if !so.is_empty() && so != "none" {
        let mut idx: Vec<usize> = (0..n_cats).collect();
        match so {
            "asc" | "ascending" => idx.sort_by(|&a, &b| {
                let ta: f64 = cfg.series.iter().filter_map(|(_, v)| v.get(a).copied()).sum();
                let tb: f64 = cfg.series.iter().filter_map(|(_, v)| v.get(b).copied()).sum();
                ta.partial_cmp(&tb).unwrap_or(std::cmp::Ordering::Equal)
            }),
            "desc" | "descending" => idx.sort_by(|&a, &b| {
                let ta: f64 = cfg.series.iter().filter_map(|(_, v)| v.get(a).copied()).sum();
                let tb: f64 = cfg.series.iter().filter_map(|(_, v)| v.get(b).copied()).sum();
                tb.partial_cmp(&ta).unwrap_or(std::cmp::Ordering::Equal)
            }),
            "alpha" | "alphabetical" => idx.sort_by(|&a, &b| cfg.category_labels[a].cmp(&cfg.category_labels[b])),
            "alpha_desc" => idx.sort_by(|&a, &b| cfg.category_labels[b].cmp(&cfg.category_labels[a])),
            _ => {}
        }
        let sl: Vec<String> = idx.iter().map(|&i| cfg.category_labels[i].clone()).collect();
        let ss: Vec<(String, Vec<f64>)> = cfg.series.iter().map(|(name, vals)| {
            let sv: Vec<f64> = idx.iter().map(|&i| vals.get(i).copied().unwrap_or(0.0)).collect();
            (name.clone(), sv)
        }).collect();
        (sl, ss)
    } else {
        (cfg.category_labels.to_vec(), cfg.series.to_vec())
    };
    let series_ref: Vec<(String, Vec<f64>)> = sorted_series;
    let n_cats = cat_labels.len();
    let max_val = if cfg.stacked {
        (0..n_cats).map(|ci| {
            series_ref.iter()
                .filter_map(|(_, vals)| vals.get(ci).copied())
                .filter(|v| v.is_finite())
                .sum::<f64>()
        }).fold(0.0_f64, f64::max)
    } else {
        series_ref.iter()
            .flat_map(|(_, vals)| vals.iter().copied())
            .filter(|v| v.is_finite())
            .fold(0.0_f64, f64::max)
    }.max(1.0);
    let legend_w: i32 = 160;
    let pad_l: i32 = 56;
    let pad_t: i32 = 42;
    let pad_b: i32 = 52;
    let pad_r: i32 = legend_w;
    let plot_w = cfg.width - pad_l - pad_r;
    let plot_h = cfg.height - pad_t - pad_b;
    let group_w = plot_w as f64 / n_cats as f64;
    let bar_w = if cfg.stacked {
        group_w * 0.62
    } else {
        group_w / (n_ser as f64 + 0.8) * 0.88
    };
    let n_total = n_cats * n_ser;
    let mut buf = Vec::<u8>::with_capacity(n_total * 260 + 4096);
    svg_open_rescalable(&mut buf, cfg.width, cfg.height, pad_l, pad_t, plot_w, plot_h);
    svg_title(&mut buf, cfg.title, (cfg.width - legend_w) / 2 + pad_l, 26);
    let n_yticks: i32 = 6;
    for i in 0..=n_yticks {
        let frac = i as f64 / n_yticks as f64;
        let y = pad_t + ((1.0 - frac) * plot_h as f64) as i32;
        let vval = frac * max_val;
        if cfg.gridlines && i > 0 {
            svg_hgrid(&mut buf, pad_l, pad_l + plot_w, y);
        }
        svg_tick_y(&mut buf, pad_l - 4, y + 3, vval);
    }
    svg_y_label(&mut buf, cfg.y_label, 14, pad_t, plot_h);
    svg_axis_lines(&mut buf, pad_l, pad_t, plot_w, plot_h);
    for (ci, cat) in cat_labels.iter().enumerate() {
        let gx = pad_l as f64 + ci as f64 * group_w;
        let base_y = (pad_t + plot_h) as f64;
        let label_x = gx + group_w / 2.0;
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, label_x);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, pad_t + plot_h + 16);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\">");
        escape_xml(&mut buf, truncate(cat, 16));
        push_b(&mut buf, b"</text>");
        if cfg.stacked {
            let mut stack_y = base_y;
            for (si, (sname, svals)) in series_ref.iter().enumerate() {
                let val = svals.get(ci).copied().unwrap_or(0.0).max(0.0);
                let bh = (val / max_val * plot_h as f64).max(0.0);
                let bx = gx + (group_w - bar_w) / 2.0;
                let by = stack_y - bh;
                let color = palette_color(cfg.palette, si);
                let hx = hex6(color);
                let hover_idx = ci * n_ser + si;
                push_b(&mut buf, b"<rect data-idx=\""); push_i(&mut buf, hover_idx as i32);
                push_b(&mut buf, b"\" data-series=\""); push_i(&mut buf, si as i32);
                push_b(&mut buf, b"\" data-v=\""); push_f2(&mut buf, val);
                push_b(&mut buf, b"\" data-lbl=\""); escape_xml(&mut buf, sname);
                push_b(&mut buf, b" \xe2\x80\x94 "); escape_xml(&mut buf, cat);
                push_b(&mut buf, b"\" x=\""); push_f2(&mut buf, bx);
                push_b(&mut buf, b"\" y=\""); push_f2(&mut buf, by);
                push_b(&mut buf, b"\" width=\""); push_f2(&mut buf, bar_w);
                push_b(&mut buf, b"\" height=\""); push_f2(&mut buf, bh);
                push_b(&mut buf, b"\" fill=\"#"); buf.extend_from_slice(&hx);
                push_b(&mut buf, b"\" rx=\"3\" stroke=\"#fff\" stroke-width=\"0.5\"/>");
                stack_y -= bh;
            }
        } else {
            let offset_start = (group_w - bar_w * n_ser as f64) / 2.0;
            for (si, (sname, svals)) in series_ref.iter().enumerate() {
                let val = svals.get(ci).copied().unwrap_or(0.0).max(0.0);
                let bh = (val / max_val * plot_h as f64).max(0.0);
                let bx = gx + offset_start + si as f64 * bar_w;
                let by = base_y - bh;
                let color = palette_color(cfg.palette, si);
                let hx = hex6(color);
                let hover_idx = ci * n_ser + si;
                push_b(&mut buf, b"<rect data-idx=\""); push_i(&mut buf, hover_idx as i32);
                push_b(&mut buf, b"\" data-series=\""); push_i(&mut buf, si as i32);
                push_b(&mut buf, b"\" data-v=\""); push_f2(&mut buf, val);
                push_b(&mut buf, b"\" data-lbl=\""); escape_xml(&mut buf, sname);
                push_b(&mut buf, b" \xe2\x80\x94 "); escape_xml(&mut buf, cat);
                push_b(&mut buf, b"\" x=\""); push_f2(&mut buf, bx);
                push_b(&mut buf, b"\" y=\""); push_f2(&mut buf, by);
                push_b(&mut buf, b"\" width=\""); push_f2(&mut buf, bar_w.max(1.0));
                push_b(&mut buf, b"\" height=\""); push_f2(&mut buf, bh);
                push_b(&mut buf, b"\" fill=\"#"); buf.extend_from_slice(&hx);
                push_b(&mut buf, b"\" rx=\"3\" fill-opacity=\"0.88\" stroke=\"#fff\" stroke-width=\"0.4\"/>");
                if cfg.show_values && bh as i32 >= cfg.value_min_height {
                    push_b(&mut buf, b"<text x=\"");
                    push_f2(&mut buf, bx + bar_w / 2.0);
                    push_b(&mut buf, b"\" y=\"");
                    push_f2(&mut buf, by - 2.0);
                    push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#1a202c\">");
                    push_f2(&mut buf, val);
                    push_b(&mut buf, b"</text>");
                }
            }
        }
    }
    let leg_x = pad_l + plot_w + 12;
    let leg_top = pad_t + 8;
    for (si, (sname, _)) in series_ref.iter().enumerate() {
        let color = palette_color(cfg.palette, si);
        svg_legend_item(&mut buf, si as i32, sname, color, leg_x, leg_top + si as i32 * 22, 20);
    }
    svg_x_label(&mut buf, cfg.x_label, pad_l + plot_w / 2, cfg.height - 6);
    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let slots: &[HoverSlot] = cfg.hover;
    build_chart_html(cfg.title, &svg, &slots_to_json(slots))
}
