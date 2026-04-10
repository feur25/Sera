use super::common::{palette_color, push_b, push_i, push_f2, escape_xml, hex6, truncate, svg_legend_item, Frame};
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
    let n_total = n_cats * n_ser;
    let mut f = Frame::new(cfg.width, cfg.height, 56, 42, 52, legend_w, n_total * 260 + 4096);
    let group_w = f.pw as f64 / n_cats as f64;
    let bar_w = if cfg.stacked {
        group_w * 0.62
    } else {
        group_w / (n_ser as f64 + 0.8) * 0.88
    };
    f.open(cfg.title, true);
    f.y_grid(6, 0.0, max_val, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);
    for (ci, cat) in cat_labels.iter().enumerate() {
        let gx = f.pl as f64 + ci as f64 * group_w;
        let base_y = (f.pt + f.ph) as f64;
        let label_x = gx + group_w / 2.0;
        push_b(&mut f.buf, b"<text x=\"");
        push_f2(&mut f.buf, label_x);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, f.pt + f.ph + 16);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\">");
        escape_xml(&mut f.buf, truncate(cat, 16));
        push_b(&mut f.buf, b"</text>");
        if cfg.stacked {
            let mut stack_y = base_y;
            for (si, (sname, svals)) in series_ref.iter().enumerate() {
                let val = svals.get(ci).copied().unwrap_or(0.0).max(0.0);
                let bh = (val / max_val * f.ph as f64).max(0.0);
                let bx = gx + (group_w - bar_w) / 2.0;
                let by = stack_y - bh;
                let color = palette_color(cfg.palette, si);
                let hx = hex6(color);
                let hover_idx = ci * n_ser + si;
                push_b(&mut f.buf, b"<rect data-idx=\""); push_i(&mut f.buf, hover_idx as i32);
                push_b(&mut f.buf, b"\" data-series=\""); push_i(&mut f.buf, si as i32);
                push_b(&mut f.buf, b"\" data-v=\""); push_f2(&mut f.buf, val);
                push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, sname);
                push_b(&mut f.buf, b" \xe2\x80\x94 "); escape_xml(&mut f.buf, cat);
                push_b(&mut f.buf, b"\" x=\""); push_f2(&mut f.buf, bx);
                push_b(&mut f.buf, b"\" y=\""); push_f2(&mut f.buf, by);
                push_b(&mut f.buf, b"\" width=\""); push_f2(&mut f.buf, bar_w);
                push_b(&mut f.buf, b"\" height=\""); push_f2(&mut f.buf, bh);
                push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
                push_b(&mut f.buf, b"\" rx=\"3\" stroke=\"#fff\" stroke-width=\"0.5\"/>");
                stack_y -= bh;
            }
        } else {
            let offset_start = (group_w - bar_w * n_ser as f64) / 2.0;
            for (si, (sname, svals)) in series_ref.iter().enumerate() {
                let val = svals.get(ci).copied().unwrap_or(0.0).max(0.0);
                let bh = (val / max_val * f.ph as f64).max(0.0);
                let bx = gx + offset_start + si as f64 * bar_w;
                let by = base_y - bh;
                let color = palette_color(cfg.palette, si);
                let hx = hex6(color);
                let hover_idx = ci * n_ser + si;
                push_b(&mut f.buf, b"<rect data-idx=\""); push_i(&mut f.buf, hover_idx as i32);
                push_b(&mut f.buf, b"\" data-series=\""); push_i(&mut f.buf, si as i32);
                push_b(&mut f.buf, b"\" data-v=\""); push_f2(&mut f.buf, val);
                push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, sname);
                push_b(&mut f.buf, b" \xe2\x80\x94 "); escape_xml(&mut f.buf, cat);
                push_b(&mut f.buf, b"\" x=\""); push_f2(&mut f.buf, bx);
                push_b(&mut f.buf, b"\" y=\""); push_f2(&mut f.buf, by);
                push_b(&mut f.buf, b"\" width=\""); push_f2(&mut f.buf, bar_w.max(1.0));
                push_b(&mut f.buf, b"\" height=\""); push_f2(&mut f.buf, bh);
                push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
                push_b(&mut f.buf, b"\" rx=\"3\" fill-opacity=\"0.88\" stroke=\"#fff\" stroke-width=\"0.4\"/>");
                if cfg.show_values && bh as i32 >= cfg.value_min_height {
                    push_b(&mut f.buf, b"<text x=\"");
                    push_f2(&mut f.buf, bx + bar_w / 2.0);
                    push_b(&mut f.buf, b"\" y=\"");
                    push_f2(&mut f.buf, by - 2.0);
                    push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#1a202c\">");
                    push_f2(&mut f.buf, val);
                    push_b(&mut f.buf, b"</text>");
                }
            }
        }
    }
    let leg_x = f.pl + f.pw + 12;
    let leg_top = f.pt + 8;
    for (si, (sname, _)) in series_ref.iter().enumerate() {
        let color = palette_color(cfg.palette, si);
        svg_legend_item(&mut f.buf, si as i32, sname, color, leg_x, leg_top + si as i32 * 22, 20);
    }
    let svg = f.svg();
    let slots: &[HoverSlot] = cfg.hover;
    build_chart_html(cfg.title, &svg, &slots_to_json(slots))
}
