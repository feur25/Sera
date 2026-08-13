use super::config::BarConfig;
use crate::plot::statistical::common::{
    escape_xml, hex6, palette_color, push_arrowhead, push_b, push_f2, push_i, push_wedge_path, svg_open_rescalable, svg_title, truncate,
};
use std::f64::consts::{FRAC_PI_2, PI};

#[allow(clippy::too_many_arguments)]
fn straight_bar(buf: &mut Vec<u8>, x0: f64, y_base: f64, w: f64, bar_h: f64, color: u32, data_idx: i32, value: f64, label: &str) {
    let hx = hex6(color);
    push_b(buf, b"<rect data-idx=\"");
    push_i(buf, data_idx);
    push_b(buf, b"\" data-v=\"");
    push_f2(buf, value);
    push_b(buf, b"\" data-lbl=\"");
    escape_xml(buf, label);
    push_b(buf, b"\" x=\"");
    push_f2(buf, x0);
    push_b(buf, b"\" y=\"");
    push_f2(buf, y_base - bar_h);
    push_b(buf, b"\" width=\"");
    push_f2(buf, w.max(1.0));
    push_b(buf, b"\" height=\"");
    push_f2(buf, bar_h.max(1.0));
    push_b(buf, b"\" fill=\"#");
    buf.extend_from_slice(&hx);
    push_b(buf, b"\" rx=\"1.5\"/>");
}

#[allow(clippy::too_many_arguments)]
fn curved_bar(buf: &mut Vec<u8>, cx: f64, cy: f64, a0: f64, a1: f64, r0: f64, r1: f64, color: u32, data_idx: i32, value: f64, label: &str) {
    let hx = hex6(color);
    push_b(buf, b"<path data-idx=\"");
    push_i(buf, data_idx);
    push_b(buf, b"\" data-v=\"");
    push_f2(buf, value);
    push_b(buf, b"\" data-lbl=\"");
    escape_xml(buf, label);
    push_b(buf, b"\" d=\"");
    push_wedge_path(buf, cx, cy, a0, a1, r0, r1);
    push_b(buf, b"\" fill=\"#");
    buf.extend_from_slice(&hx);
    push_b(buf, b"\" stroke=\"#fff\" stroke-width=\"0.5\"/>");
}

#[crate::chart_demo(
    "labels=[\"Participation a des salons\",\"Visite de salons\",\"Prospection telephonique\",\"Voyages et conventions d'affaires\",\"Site web\",\"Mailing\",\"Emailing\",\"Reponses aux appels d'offres publics\",\"Recommandations des clients\",\"Reseaux de professionnels\",\"Club d'entreprises\",\"Action des CCI\",\"Action des organismes de promotion du design\"], series_names=[\"Designer independant\",\"Agence de design\"], series=[[7.3,4.3,3.2,6.7,11.7,2.9,9.3,4.2,36.6,33.3,18.6,20.3,15.8],[4.7,3.9,2.5,2.9,8.7,3.8,6.7,4.4,20.2,18.2,11.4,10.9,9.6]], palette=[3039066,6271907], variant=\"multicategory_arc\", width=850, height=440"
)]

pub fn render(cfg: &BarConfig) -> String {
    let n_cats = cfg.category_labels.len();
    if n_cats == 0 {
        return String::new();
    }
    let n_ser = cfg.series.len().max(1);

    let max_val: f64 = if cfg.series.is_empty() {
        cfg.values.iter().cloned().filter(|v| v.is_finite()).fold(0.0f64, f64::max).max(1.0)
    } else {
        cfg.series.iter().flat_map(|(_, v)| v.iter().copied()).filter(|v| v.is_finite()).fold(0.0f64, f64::max).max(1.0)
    };

    let w = cfg.width;
    let h = cfg.height;
    let wf = w as f64;
    let hf = h as f64;

    let pad_x = 50.0;
    let usable_w = wf - 2.0 * pad_x;
    let cat_w = usable_w / n_cats as f64;
    let x_of = |ci: usize| pad_x + cat_w * (ci as f64 + 0.5);

    let axis_y = hf * 0.40;
    let gap = 11.0;
    let top_base = axis_y - gap;
    let bottom_hub_y = axis_y + gap;

    let top_margin = 40.0;
    let bar_max_h = (top_base - top_margin).max(10.0);

    let cx = wf / 2.0;
    let r_hub = 12.0;
    let r_max = (hf - bottom_hub_y - 74.0).min(wf / 2.0 - 60.0).max(r_hub + 10.0);

    let angle_of = |ci: usize| PI * (1.0 - (ci as f64 + 0.5) / n_cats as f64);
    let slot = PI / n_cats as f64;
    let half_slot = slot * 0.42;

    let mut buf = Vec::<u8>::with_capacity(n_cats * n_ser * 260 + 12_000);
    svg_open_rescalable(&mut buf, w, h, 0, 0, w, h);
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    svg_title(&mut buf, cfg.title, w / 2, 20);

    for ci in 0..n_cats {
        let cxi = x_of(ci);
        if cfg.series.is_empty() {
            let v = cfg.values.get(ci).copied().unwrap_or(0.0);
            let color = palette_color(cfg.palette, ci);
            let bh = (v / max_val) * bar_max_h;
            let bw = cat_w * 0.62;
            straight_bar(&mut buf, cxi - bw / 2.0, top_base, bw, bh, color, ci as i32, v, &cfg.category_labels[ci]);
            let a = angle_of(ci);
            let re = r_hub + (v / max_val) * (r_max - r_hub);
            curved_bar(&mut buf, cx, bottom_hub_y, a - half_slot, a + half_slot, r_hub, re, color, (n_cats + ci) as i32, v, &cfg.category_labels[ci]);
        } else {
            let group_w = cat_w * 0.74;
            let bw = group_w / n_ser as f64;
            let sub_w = (2.0 * half_slot) / n_ser as f64;
            let a = angle_of(ci);
            for (si, (sname, vals)) in cfg.series.iter().enumerate() {
                let v = vals.get(ci).copied().unwrap_or(0.0);
                if !v.is_finite() {
                    continue;
                }
                let color = palette_color(cfg.palette, si);
                let bh = (v / max_val) * bar_max_h;
                let bx = cxi - group_w / 2.0 + si as f64 * bw;
                let mut lbl = cfg.category_labels[ci].clone();
                lbl.push_str(" \u{2014} ");
                lbl.push_str(sname);
                straight_bar(&mut buf, bx, top_base, bw - 1.5, bh, color, (ci * n_ser + si) as i32, v, &lbl);
                let a0 = a - half_slot + si as f64 * sub_w;
                let re = r_hub + (v / max_val) * (r_max - r_hub);
                curved_bar(&mut buf, cx, bottom_hub_y, a0, a0 + sub_w, r_hub, re, color, (n_cats * n_ser + ci * n_ser + si) as i32, v, &lbl);
            }
        }
    }

    if !cfg.super_categories.is_empty() {
        let mut start = 0usize;
        while start < n_cats {
            let cur = cfg.super_categories.get(start).map(|s| s.as_str()).unwrap_or("");
            let mut end = start + 1;
            while end < n_cats && cfg.super_categories.get(end).map(|s| s.as_str()).unwrap_or("") == cur {
                end += 1;
            }
            let x1 = pad_x + start as f64 * cat_w + 3.0;
            let x2 = pad_x + end as f64 * cat_w - 3.0;
            let y_line = top_margin - 10.0;
            push_b(&mut buf, b"<line x1=\"");
            push_f2(&mut buf, x1);
            push_b(&mut buf, b"\" y1=\"");
            push_f2(&mut buf, y_line);
            push_b(&mut buf, b"\" x2=\"");
            push_f2(&mut buf, x2);
            push_b(&mut buf, b"\" y2=\"");
            push_f2(&mut buf, y_line);
            push_b(&mut buf, b"\" stroke=\"#94a3b8\" stroke-width=\"1\"/>");
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, (x1 + x2) / 2.0);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, y_line - 5.0);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" font-weight=\"700\" fill=\"#334155\">");
            escape_xml(&mut buf, cur);
            push_b(&mut buf, b"</text>");
            start = end;
        }
    }

    for ci in 0..n_cats {
        let cxi = x_of(ci);
        push_arrowhead(&mut buf, cxi, top_base + 4.5, FRAC_PI_2, 4.5, b"#94a3b8");
        push_arrowhead(&mut buf, cxi, bottom_hub_y - 4.5, -FRAC_PI_2, 4.5, b"#94a3b8");
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, cxi);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, axis_y);
        push_b(&mut buf, b"\" r=\"6\" fill=\"#fff\" stroke=\"#94a3b8\" stroke-width=\"1\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cxi);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, axis_y + 2.6);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"6.6\" font-weight=\"700\" fill=\"#334155\">");
        let s = (ci + 1).to_string();
        buf.extend_from_slice(s.as_bytes());
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" cy=\"");
    push_f2(&mut buf, bottom_hub_y);
    push_b(&mut buf, b"\" r=\"");
    push_f2(&mut buf, r_hub);
    push_b(&mut buf, b"\" fill=\"#f1f5f9\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");

    if !cfg.series.is_empty() {
        let leg_x = wf - 150.0;
        let leg_y = 40.0;
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, leg_x);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, leg_y - 10.0);
        push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8\" font-weight=\"700\" fill=\"#475569\">SERIE</text>");
        for (si, (name, _)) in cfg.series.iter().enumerate() {
            let sy = leg_y + si as f64 * 13.0;
            let color = palette_color(cfg.palette, si);
            push_b(&mut buf, b"<rect x=\"");
            push_f2(&mut buf, leg_x);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, sy - 7.0);
            push_b(&mut buf, b"\" width=\"9\" height=\"9\" rx=\"2\" fill=\"#");
            buf.extend_from_slice(&hex6(color));
            push_b(&mut buf, b"\"/><text x=\"");
            push_f2(&mut buf, leg_x + 13.0);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, sy);
            push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"7.5\" fill=\"#475569\">");
            escape_xml(&mut buf, name);
            push_b(&mut buf, b"</text>");
        }
    }

    let idx_cols = 3.0;
    let idx_x0 = 16.0;
    let idx_row_h = 12.2;
    let idx_col_w = (wf - 32.0) / idx_cols;
    let idx_y0 = hf - 60.0;
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, idx_x0);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, idx_y0 - 8.0);
    push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"7.6\" font-weight=\"700\" fill=\"#475569\">CATEGORIES, PAR NUMERO</text>");
    let rows_per_col = (n_cats as f64 / idx_cols).ceil();
    for ci in 0..n_cats {
        let col = (ci as f64 / rows_per_col).floor();
        let row = ci as f64 - col * rows_per_col;
        let ix = idx_x0 + col * idx_col_w;
        let iy = idx_y0 + row * idx_row_h;
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, ix);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, iy);
        push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"7.2\" fill=\"#475569\"><tspan font-weight=\"700\">");
        let s = (ci + 1).to_string();
        buf.extend_from_slice(s.as_bytes());
        push_b(&mut buf, b".</tspan> ");
        escape_xml(&mut buf, truncate(&cfg.category_labels[ci], 40));
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let slots_json;
    let json: &str = if cfg.hover.is_empty() {
        "[]"
    } else {
        slots_json = crate::html::hover::slots_to_json(cfg.hover);
        &slots_json
    };
    crate::html::hover::build_chart_html(cfg.title, &svg, json)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg<'a>(category_labels: &'a [String], series: &'a [(String, Vec<f64>)]) -> BarConfig<'a> {
        BarConfig {
            title: "Test",
            category_labels,
            series,
            width: 850,
            height: 440,
            ..BarConfig::default()
        }
    }

    fn synth(n: usize, n_series: usize) -> (Vec<String>, Vec<(String, Vec<f64>)>) {
        let labels: Vec<String> = (0..n).map(|i| format!("Item {i}")).collect();
        let series: Vec<(String, Vec<f64>)> = (0..n_series)
            .map(|s| {
                let vals: Vec<f64> = (0..n).map(|i| 5.0 + ((i + s) as f64 * 0.6).sin().abs() * 20.0).collect();
                (format!("Series {s}"), vals)
            })
            .collect();
        (labels, series)
    }

    #[test]
    fn renders_a_straight_bar_and_a_curved_wedge_per_item_per_series() {
        let (labels, series) = synth(13, 2);
        let html = render(&cfg(&labels, &series));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<rect data-idx=").count(), 13 * 2);
        assert_eq!(html.matches("<path data-idx=").count(), 13 * 2);
        assert!(html.contains("class=\"sp-bg\""));
    }

    #[test]
    fn single_series_falls_back_to_one_bar_per_category() {
        let labels: Vec<String> = (0..6).map(|i| format!("Item {i}")).collect();
        let series: Vec<(String, Vec<f64>)> = vec![];
        let html = render(&cfg(&labels, &series));
        assert_eq!(html.matches("<rect data-idx=").count(), 6);
        assert_eq!(html.matches("<path data-idx=").count(), 6);
    }

    #[test]
    fn the_index_legend_lists_every_category() {
        let (labels, series) = synth(13, 2);
        let html = render(&cfg(&labels, &series));
        for l in &labels {
            assert!(html.contains(l.as_str()));
        }
    }

    #[test]
    fn category_zero_sits_on_the_left_in_both_panels() {
        let (labels, series) = synth(4, 1);
        let html = render(&cfg(&labels, &series));
        let first_rect = html.find("<rect data-idx=\"0\"").unwrap();
        let first_x = html[first_rect..].find("x=\"").unwrap() + first_rect;
        let x_val: f64 = html[first_x + 3..].split('"').next().unwrap().parse().unwrap();
        assert!(x_val < 850.0 / 2.0, "category 0's straight bar should sit left of center, got x={x_val}");
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let labels: Vec<String> = vec![];
        let series: Vec<(String, Vec<f64>)> = vec![];
        assert!(render(&cfg(&labels, &series)).is_empty());
    }

    #[test]
    fn perf_rendering_many_categories_stays_fast() {
        let (labels, series) = synth(400, 3);
        let start = std::time::Instant::now();
        let html = render(&cfg(&labels, &series));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
