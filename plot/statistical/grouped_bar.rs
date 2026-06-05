use super::common::{
    escape_xml, hex6, palette_color, push_b, push_f2, push_i, sort_indices, sorted,
    svg_legend_item, truncate, Frame,
};
use crate::html::hover::slots_to_json;
use crate::plot::{apply_h, parse_all};

pub struct GroupedBar;

crate::chart_config!(GroupedBarConfig, 1100, 480;
    struct {
        pub category_labels: &'a [String],
        pub series: &'a [(String, Vec<f64>)],
        pub palette: &'a [u32],
        pub stacked: bool,
        pub show_values: bool,
        pub value_min_height: i32,
        pub orientation: u8,
    }
    defaults {
        category_labels: &[],
        series: &[],
        palette: &[],
        stacked: false,
        show_values: false,
        value_min_height: 16,
        orientation: b'v',
    }
);

pub fn render_grouped_bar_html(cfg: &GroupedBarConfig) -> String {
    if cfg.orientation == b'h' {
        return render_grouped_bar_horiz(cfg);
    }
    let n_cats = cfg.category_labels.len();
    let n_ser = cfg.series.len();
    if n_cats == 0 || n_ser == 0 {
        return String::new();
    }
    let sums: Vec<f64> = (0..n_cats)
        .map(|i| {
            cfg.series
                .iter()
                .filter_map(|(_, v)| v.get(i).copied())
                .sum()
        })
        .collect();
    let idx = sort_indices(n_cats, &sums, cfg.category_labels, cfg.sort_order);
    let cat_labels = sorted(&idx, cfg.category_labels);
    let series_ref: Vec<(String, Vec<f64>)> = cfg
        .series
        .iter()
        .map(|(name, vals)| {
            let sv: Vec<f64> = idx
                .iter()
                .map(|&i| vals.get(i).copied().unwrap_or(0.0))
                .collect();
            (name.clone(), sv)
        })
        .collect();
    let n_cats = cat_labels.len();
    let max_val = if cfg.stacked {
        (0..n_cats)
            .map(|ci| {
                series_ref
                    .iter()
                    .filter_map(|(_, vals)| vals.get(ci).copied())
                    .filter(|v| v.is_finite())
                    .sum::<f64>()
            })
            .fold(0.0_f64, f64::max)
    } else {
        series_ref
            .iter()
            .flat_map(|(_, vals)| vals.iter().copied())
            .filter(|v| v.is_finite())
            .fold(0.0_f64, f64::max)
    }
    .max(1.0);
    let legend_w: i32 = 160;
    let n_total = n_cats * n_ser;
    let mut f = Frame::new_html(
        cfg.title,
        cfg.width,
        cfg.height,
        56,
        42,
        52,
        legend_w,
        n_total * 260 + 4096,
    );
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
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, f.pt + f.ph + 16);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\" class=\"sp-xt\">");
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
                push_b(&mut f.buf, b"<rect data-idx=\"");
                push_i(&mut f.buf, hover_idx as i32);
                push_b(&mut f.buf, b"\" data-series=\"");
                push_i(&mut f.buf, si as i32);
                push_b(&mut f.buf, b"\" data-v=\"");
                push_f2(&mut f.buf, val);
                push_b(&mut f.buf, b"\" data-lbl=\"");
                escape_xml(&mut f.buf, sname);
                push_b(&mut f.buf, b" \xe2\x80\x94 ");
                escape_xml(&mut f.buf, cat);
                push_b(&mut f.buf, b"\" x=\"");
                push_f2(&mut f.buf, bx);
                push_b(&mut f.buf, b"\" y=\"");
                push_f2(&mut f.buf, by);
                push_b(&mut f.buf, b"\" width=\"");
                push_f2(&mut f.buf, bar_w);
                push_b(&mut f.buf, b"\" height=\"");
                push_f2(&mut f.buf, bh);
                push_b(&mut f.buf, b"\" fill=\"#");
                f.buf.extend_from_slice(&hx);
                push_b(
                    &mut f.buf,
                    b"\" rx=\"3\" stroke=\"#fff\" stroke-width=\"0.5\"/>",
                );
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
                push_b(&mut f.buf, b"<rect data-idx=\"");
                push_i(&mut f.buf, hover_idx as i32);
                push_b(&mut f.buf, b"\" data-series=\"");
                push_i(&mut f.buf, si as i32);
                push_b(&mut f.buf, b"\" data-v=\"");
                push_f2(&mut f.buf, val);
                push_b(&mut f.buf, b"\" data-lbl=\"");
                escape_xml(&mut f.buf, sname);
                push_b(&mut f.buf, b" \xe2\x80\x94 ");
                escape_xml(&mut f.buf, cat);
                push_b(&mut f.buf, b"\" x=\"");
                push_f2(&mut f.buf, bx);
                push_b(&mut f.buf, b"\" y=\"");
                push_f2(&mut f.buf, by);
                push_b(&mut f.buf, b"\" width=\"");
                push_f2(&mut f.buf, bar_w.max(1.0));
                push_b(&mut f.buf, b"\" height=\"");
                push_f2(&mut f.buf, bh);
                push_b(&mut f.buf, b"\" fill=\"#");
                f.buf.extend_from_slice(&hx);
                push_b(
                    &mut f.buf,
                    b"\" rx=\"3\" fill-opacity=\"0.88\" stroke=\"#fff\" stroke-width=\"0.4\"/>",
                );
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
        svg_legend_item(
            &mut f.buf,
            si as i32,
            sname,
            color,
            leg_x,
            leg_top + si as i32 * 22,
            20,
        );
    }
    let slots = cfg.hover;
    f.html(&slots_to_json(slots))
}

fn render_grouped_bar_horiz(cfg: &GroupedBarConfig) -> String {
    let n_cats = cfg.category_labels.len();
    let n_ser = cfg.series.len();
    if n_cats == 0 || n_ser == 0 {
        return String::new();
    }
    let sums: Vec<f64> = (0..n_cats)
        .map(|i| {
            cfg.series
                .iter()
                .filter_map(|(_, v)| v.get(i).copied())
                .sum()
        })
        .collect();
    let idx = sort_indices(n_cats, &sums, cfg.category_labels, cfg.sort_order);
    let cat_labels = sorted(&idx, cfg.category_labels);
    let series_ref: Vec<(String, Vec<f64>)> = cfg
        .series
        .iter()
        .map(|(name, vals)| {
            let sv: Vec<f64> = idx
                .iter()
                .map(|&i| vals.get(i).copied().unwrap_or(0.0))
                .collect();
            (name.clone(), sv)
        })
        .collect();
    let n_cats = cat_labels.len();
    let max_val = if cfg.stacked {
        (0..n_cats)
            .map(|ci| {
                series_ref
                    .iter()
                    .filter_map(|(_, vals)| vals.get(ci).copied())
                    .filter(|v| v.is_finite())
                    .sum::<f64>()
            })
            .fold(0.0_f64, f64::max)
    } else {
        series_ref
            .iter()
            .flat_map(|(_, vals)| vals.iter().copied())
            .filter(|v| v.is_finite())
            .fold(0.0_f64, f64::max)
    }
    .max(1.0);
    let legend_w: i32 = 160;
    let n_total = n_cats * n_ser;
    let mut f = Frame::new_html(
        cfg.title,
        cfg.width,
        cfg.height,
        38,
        120,
        38,
        legend_w,
        n_total * 260 + 4096,
    );
    let row_h = f.ph as f64 / n_cats as f64;
    let bar_h = if cfg.stacked {
        row_h * 0.62
    } else {
        row_h / (n_ser as f64 + 0.8) * 0.88
    };
    f.open(cfg.title, true);
    f.x_grid(6, 0.0, max_val, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);
    for (ci, cat) in cat_labels.iter().enumerate() {
        let gy = f.pt as f64 + ci as f64 * row_h;
        let label_y = gy + row_h / 2.0 + 4.0;
        push_b(&mut f.buf, b"<text x=\"");
        push_i(&mut f.buf, f.pl - 6);
        push_b(&mut f.buf, b"\" y=\"");
        push_f2(&mut f.buf, label_y);
        push_b(&mut f.buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\" class=\"sp-yt\">");
        escape_xml(&mut f.buf, truncate(cat, 16));
        push_b(&mut f.buf, b"</text>");
        if cfg.stacked {
            let mut stack_x = f.pl as f64;
            for (si, (sname, svals)) in series_ref.iter().enumerate() {
                let val = svals.get(ci).copied().unwrap_or(0.0).max(0.0);
                let bw = (val / max_val * f.pw as f64).max(0.0);
                let bx = stack_x;
                let by = gy + (row_h - bar_h) / 2.0;
                let color = palette_color(cfg.palette, si);
                let hx = hex6(color);
                let hover_idx = ci * n_ser + si;
                push_b(&mut f.buf, b"<rect data-idx=\"");
                push_i(&mut f.buf, hover_idx as i32);
                push_b(&mut f.buf, b"\" data-series=\"");
                push_i(&mut f.buf, si as i32);
                push_b(&mut f.buf, b"\" data-v=\"");
                push_f2(&mut f.buf, val);
                push_b(&mut f.buf, b"\" data-lbl=\"");
                escape_xml(&mut f.buf, sname);
                push_b(&mut f.buf, b" \xe2\x80\x94 ");
                escape_xml(&mut f.buf, cat);
                push_b(&mut f.buf, b"\" x=\"");
                push_f2(&mut f.buf, bx);
                push_b(&mut f.buf, b"\" y=\"");
                push_f2(&mut f.buf, by);
                push_b(&mut f.buf, b"\" width=\"");
                push_f2(&mut f.buf, bw);
                push_b(&mut f.buf, b"\" height=\"");
                push_f2(&mut f.buf, bar_h);
                push_b(&mut f.buf, b"\" fill=\"#");
                f.buf.extend_from_slice(&hx);
                push_b(
                    &mut f.buf,
                    b"\" rx=\"3\" stroke=\"#fff\" stroke-width=\"0.5\"/>",
                );
                stack_x += bw;
            }
        } else {
            let offset_start = (row_h - bar_h * n_ser as f64) / 2.0;
            for (si, (sname, svals)) in series_ref.iter().enumerate() {
                let val = svals.get(ci).copied().unwrap_or(0.0).max(0.0);
                let bw = (val / max_val * f.pw as f64).max(0.0);
                let bx = f.pl as f64;
                let by = gy + offset_start + si as f64 * bar_h;
                let color = palette_color(cfg.palette, si);
                let hx = hex6(color);
                let hover_idx = ci * n_ser + si;
                push_b(&mut f.buf, b"<rect data-idx=\"");
                push_i(&mut f.buf, hover_idx as i32);
                push_b(&mut f.buf, b"\" data-series=\"");
                push_i(&mut f.buf, si as i32);
                push_b(&mut f.buf, b"\" data-v=\"");
                push_f2(&mut f.buf, val);
                push_b(&mut f.buf, b"\" data-lbl=\"");
                escape_xml(&mut f.buf, sname);
                push_b(&mut f.buf, b" \xe2\x80\x94 ");
                escape_xml(&mut f.buf, cat);
                push_b(&mut f.buf, b"\" x=\"");
                push_f2(&mut f.buf, bx);
                push_b(&mut f.buf, b"\" y=\"");
                push_f2(&mut f.buf, by);
                push_b(&mut f.buf, b"\" width=\"");
                push_f2(&mut f.buf, bw.max(1.0));
                push_b(&mut f.buf, b"\" height=\"");
                push_f2(&mut f.buf, bar_h);
                push_b(&mut f.buf, b"\" fill=\"#");
                f.buf.extend_from_slice(&hx);
                push_b(
                    &mut f.buf,
                    b"\" rx=\"3\" fill-opacity=\"0.88\" stroke=\"#fff\" stroke-width=\"0.4\"/>",
                );
                if cfg.show_values && bw as i32 >= cfg.value_min_height {
                    push_b(&mut f.buf, b"<text x=\"");
                    push_f2(&mut f.buf, bx + bw + 3.0);
                    push_b(&mut f.buf, b"\" y=\"");
                    push_f2(&mut f.buf, by + bar_h / 2.0 + 4.0);
                    push_b(&mut f.buf, b"\" text-anchor=\"start\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#1a202c\">");
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
        svg_legend_item(
            &mut f.buf,
            si as i32,
            sname,
            color,
            leg_x,
            leg_top + si as i32 * 22,
            20,
        );
    }
    let slots = cfg.hover;
    f.html(&slots_to_json(slots))
}

#[crate::sera_alias("grouped_bar", "grouped_bars", "grouped_bar_chart", "group_bar")]
#[crate::sera_builder]
pub fn build_grouped_bar(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let category_labels = a.labels.unwrap_or_default();
    let series_flat: Vec<f64> = a.values.unwrap_or_default();
    use crate::plot::statistical::{render_grouped_bar_html, GroupedBarConfig};
    let hover = o.hj();
    let sn = o.series_names.clone().unwrap_or_default();
    let n_cats = category_labels.len();
    let n_series = if !sn.is_empty() {
        sn.len()
    } else if n_cats > 0 {
        (series_flat.len() + n_cats - 1) / n_cats
    } else {
        0
    };
    let names: Vec<String> = if !sn.is_empty() {
        sn
    } else {
        (0..n_series).map(|_| String::new()).collect()
    };
    let series: Vec<(String, Vec<f64>)> = names
        .iter()
        .enumerate()
        .map(|(si, name)| {
            let vals: Vec<f64> = (0..n_cats)
                .map(|ci| series_flat.get(si * n_cats + ci).copied().unwrap_or(0.0))
                .collect();
            (name.clone(), vals)
        })
        .collect();
    let html = render_grouped_bar_html(&GroupedBarConfig {
        title,
        category_labels: &category_labels,
        series: &series,
        palette: &o.pal(),
        x_label: &o.xl(),
        y_label: &o.yl(),
        show_values: o.show_values.unwrap_or(false),
        gridlines: o.grid(),
        sort_order: &o.srt(),
        hover: &hover,
        orientation: o.orient_byte(),
        ..GroupedBarConfig::default()
    });
    apply_h(html, &o)
}

#[crate::sera_alias("stacked_bar", "stacked_bars", "stacked_bar_chart", "stack_bar")]
#[crate::sera_builder]
pub fn build_stacked_bar(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let category_labels = a.labels.unwrap_or_default();
    let series_flat: Vec<f64> = a.values.unwrap_or_default();
    use crate::plot::statistical::{render_grouped_bar_html, GroupedBarConfig};
    let hover = o.hj();
    let sn = o.series_names.clone().unwrap_or_default();
    let n_cats = category_labels.len();
    let n_series = if !sn.is_empty() {
        sn.len()
    } else if n_cats > 0 {
        (series_flat.len() + n_cats - 1) / n_cats
    } else {
        0
    };
    let names: Vec<String> = if !sn.is_empty() {
        sn
    } else {
        (0..n_series).map(|_| String::new()).collect()
    };
    let series: Vec<(String, Vec<f64>)> = names
        .iter()
        .enumerate()
        .map(|(si, name)| {
            let vals: Vec<f64> = (0..n_cats)
                .map(|ci| series_flat.get(si * n_cats + ci).copied().unwrap_or(0.0))
                .collect();
            (name.clone(), vals)
        })
        .collect();
    let html = render_grouped_bar_html(&GroupedBarConfig {
        title,
        category_labels: &category_labels,
        series: &series,
        palette: &o.pal(),
        x_label: &o.xl(),
        y_label: &o.yl(),
        show_values: o.show_values.unwrap_or(false),
        gridlines: o.grid(),
        sort_order: &o.srt(),
        hover: &hover,
        stacked: true,
        orientation: o.orient_byte(),
        ..GroupedBarConfig::default()
    });
    apply_h(html, &o)
}
