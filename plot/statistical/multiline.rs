use super::common::{
    escape_xml, hex6, palette_color, push_b, push_f2, push_i, sort_indices, sorted,
    svg_legend_item, truncate, Frame,
};
use crate::html::hover::slots_to_json;
use crate::plot::{apply, parse_all};

pub struct MultiLine;

crate::chart_config!(MultiLineConfig, 1100, 480;
    struct {
        pub x_labels: &'a [String],
        pub series: &'a [(String, Vec<f64>)],
        pub palette: &'a [u32],
        pub show_points: bool,
    }
    defaults {
        x_labels: &[],
        series: &[],
        palette: &[],
        show_points: true,
    }
);

pub fn render_multiline_html(cfg: &MultiLineConfig) -> String {
    let n_pts = cfg.x_labels.len();
    let n_ser = cfg.series.len();
    if n_pts < 2 || n_ser == 0 {
        return String::new();
    }
    let series: Vec<(String, Vec<f64>)> =
        if cfg.sort_order != "none" && !cfg.sort_order.is_empty() && n_ser > 1 {
            let totals: Vec<f64> = cfg
                .series
                .iter()
                .map(|(_, v)| v.iter().sum::<f64>())
                .collect();
            let names: Vec<String> = cfg.series.iter().map(|(n, _)| n.clone()).collect();
            sorted(
                &sort_indices(n_ser, &totals, &names, cfg.sort_order),
                cfg.series,
            )
        } else {
            cfg.series.to_vec()
        };
    let max_val = series
        .iter()
        .flat_map(|(_, v)| v.iter().copied())
        .filter(|v| v.is_finite())
        .fold(0.0_f64, f64::max)
        .max(1.0);
    let min_val = series
        .iter()
        .flat_map(|(_, v)| v.iter().copied())
        .filter(|v| v.is_finite())
        .fold(f64::INFINITY, f64::min)
        .min(0.0);
    let range = (max_val - min_val).max(1e-12);
    let legend_w: i32 = 160;
    let auto_hover = cfg.hover.is_empty();
    let n_total = n_pts * n_ser;
    let mut f = Frame::new_html(
        cfg.title,
        cfg.width,
        cfg.height,
        56,
        42,
        52,
        legend_w,
        n_total * 80 + 2048,
    );
    let step_x = f.pw as f64 / (n_pts - 1).max(1) as f64;
    f.open(cfg.title, true);
    f.y_grid_rc(6, min_val, max_val, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);
    let tick_step = ((n_pts as f64 / 12.0).ceil() as usize).max(1);
    for i in (0..n_pts).step_by(tick_step) {
        let x = f.pl + (i as f64 * step_x) as i32;
        push_b(&mut f.buf, b"<text x=\"");
        push_i(&mut f.buf, x);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, f.pt + f.ph + 14);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#6b7280\" class=\"sp-xt\">");
        escape_xml(&mut f.buf, truncate(&cfg.x_labels[i], 12));
        push_b(&mut f.buf, b"</text>");
    }
    for (si, (sname, svals)) in series.iter().enumerate() {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        let mut sname_esc = Vec::with_capacity(sname.len() + 8);
        escape_xml(&mut sname_esc, sname);
        push_b(&mut f.buf, b"<polyline data-series=\"");
        push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b"\" data-idx=\"");
        push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b"\" fill=\"none\" stroke=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(
            &mut f.buf,
            b"\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" points=\"",
        );
        for i in 0..n_pts {
            let val = svals.get(i).copied().unwrap_or(0.0);
            let frac = if val.is_finite() {
                (val - min_val) / range
            } else {
                0.0
            };
            let x = f.pl + (i as f64 * step_x) as i32;
            let y = f.pt + ((1.0 - frac) * f.ph as f64) as i32;
            if i > 0 {
                f.buf.push(b' ');
            }
            push_i(&mut f.buf, x);
            f.buf.push(b',');
            push_i(&mut f.buf, y);
        }
        push_b(&mut f.buf, b"\"/>");
        if cfg.show_points {
            let pt_step = ((n_pts as f64 / 20.0).ceil() as usize).max(1);
            for i in (0..n_pts).step_by(pt_step) {
                let val = svals.get(i).copied().unwrap_or(0.0);
                let frac = if val.is_finite() {
                    (val - min_val) / range
                } else {
                    0.0
                };
                let x = f.pl + (i as f64 * step_x) as i32;
                let y = f.pt + ((1.0 - frac) * f.ph as f64) as i32;
                let idx = (si * n_pts + i) as i32;
                push_b(&mut f.buf, b"<circle data-series=\"");
                push_i(&mut f.buf, si as i32);
                push_b(&mut f.buf, b"\" data-idx=\"");
                push_i(&mut f.buf, idx);
                push_b(&mut f.buf, b"\" data-y=\"");
                push_f2(&mut f.buf, val);
                push_b(&mut f.buf, b"\" data-lbl=\"");
                f.buf.extend_from_slice(&sname_esc);
                push_b(&mut f.buf, b"\" cx=\"");
                push_i(&mut f.buf, x);
                push_b(&mut f.buf, b"\" cy=\"");
                push_i(&mut f.buf, y);
                push_b(&mut f.buf, b"\" r=\"3\" fill=\"#");
                f.buf.extend_from_slice(&hx);
                push_b(&mut f.buf, b"\" stroke=\"#fff\" stroke-width=\"1\"/>");
            }
        }
    }
    let leg_x = cfg.width - legend_w + 14;
    for (si, (sname, _)) in series.iter().enumerate() {
        let color = palette_color(cfg.palette, si);
        svg_legend_item(
            &mut f.buf,
            si as i32,
            sname,
            color,
            leg_x,
            f.pt + 6 + si as i32 * 18,
            18,
        );
    }
    let slots_json;
    let json: &str = if auto_hover {
        "[]"
    } else {
        slots_json = slots_to_json(cfg.hover);
        &slots_json
    };
    f.html(json)
}

#[crate::sera_alias(
    "multiline",
    "multilines",
    "multiline_chart",
    "multiline_family",
    "multi_line",
    "multi_line_chart"
)]
#[crate::sera_builder]
pub fn build_multiline_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x_labels = a.x_labels.or(a.labels).unwrap_or_default();
    let series_flat = a.series.unwrap_or_default();
    use crate::plot::statistical::{render_multiline_html, MultiLineConfig};
    let hover = o.hj();
    let sn = o.series_names.clone().unwrap_or_default();
    let names: Vec<String> = if sn.is_empty() {
        (0..series_flat.len()).map(|_| String::new()).collect()
    } else {
        sn
    };
    let series: Vec<(String, Vec<f64>)> = names.into_iter().zip(series_flat.into_iter()).collect();
    let html = render_multiline_html(&MultiLineConfig {
        title,
        x_labels: &x_labels,
        series: &series,
        palette: &o.pal(),
        x_label: &o.xl(),
        y_label: &o.yl(),
        show_points: o.show_points.unwrap_or(true),
        gridlines: o.grid(),
        width: o.w(1100),
        height: o.h(480),
        hover: &hover,
        sort_order: &o.srt(),
        ..MultiLineConfig::default()
    });
    apply(html, &o)
}
