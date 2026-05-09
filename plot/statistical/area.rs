use super::common::{sorted, sort_indices, palette_color, push_b, push_i, push_f2, escape_xml, hex6, truncate, Frame};
use crate::html::hover::slots_to_json;

pub struct Area;

crate::chart_config!(AreaConfig, 1100, 480;
    struct {
        pub x_labels: &'a [String],
        pub series: &'a [(String, Vec<f64>)],
        pub palette: &'a [u32],
        pub stacked: bool,
    }
    defaults {
        x_labels: &[],
        series: &[],
        palette: &[],
        stacked: false,
    }
);

pub fn render_area_html(cfg: &AreaConfig) -> String {
    let n_pts = cfg.x_labels.len();
    let n_ser = cfg.series.len();
    if n_pts < 2 || n_ser == 0 { return String::new(); }
    let series: Vec<(String, Vec<f64>)> = if cfg.sort_order != "none" && !cfg.sort_order.is_empty() && n_ser > 1 {
        let totals: Vec<f64> = cfg.series.iter().map(|(_, v)| v.iter().sum::<f64>()).collect();
        let names: Vec<String> = cfg.series.iter().map(|(n, _)| n.clone()).collect();
        sorted(&sort_indices(n_ser, &totals, &names, cfg.sort_order), cfg.series)
    } else {
        cfg.series.to_vec()
    };
    let mut stacked_sums: Vec<Vec<f64>> = Vec::with_capacity(n_ser);
    if cfg.stacked {
        let mut running = vec![0.0_f64; n_pts];
        for (_, svals) in series.iter() {
            for i in 0..n_pts {
                running[i] += svals.get(i).copied().unwrap_or(0.0).max(0.0);
            }
            stacked_sums.push(running.clone());
        }
    }
    let max_val = if cfg.stacked {
        stacked_sums.last().map(|s| s.iter().copied().fold(0.0_f64, f64::max)).unwrap_or(1.0)
    } else {
        series.iter()
            .flat_map(|(_, v)| v.iter().copied())
            .filter(|v| v.is_finite())
            .fold(0.0_f64, f64::max)
    }.max(1.0);
    let legend_w: i32 = 160;
    let auto_hover = cfg.hover.is_empty();
    let n_total = n_pts * n_ser;
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 56, 42, 52, legend_w, n_total * 60 + 2048);
    let step_x = f.pw as f64 / (n_pts - 1).max(1) as f64;
    let base_y = f.pt + f.ph;
    f.open(cfg.title, true);
    f.y_grid_rc(6, 0.0, max_val, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);
    for si in (0..n_ser).rev() {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        push_b(&mut f.buf, b"<path data-series=\""); push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b"\" d=\"M"); push_i(&mut f.buf, f.pl); f.buf.push(b','); push_i(&mut f.buf, base_y);
        for i in 0..n_pts {
            let x = f.pl + (i as f64 * step_x) as i32;
            let val = if cfg.stacked { stacked_sums[si][i] } else { cfg.series[si].1.get(i).copied().unwrap_or(0.0) };
            let frac = (val / max_val).clamp(0.0, 1.0);
            let y = f.pt + ((1.0 - frac) * f.ph as f64) as i32;
            push_b(&mut f.buf, b" L"); push_i(&mut f.buf, x); f.buf.push(b','); push_i(&mut f.buf, y);
        }
        if cfg.stacked && si > 0 {
            for i in (0..n_pts).rev() {
                let x = f.pl + (i as f64 * step_x) as i32;
                let prev_val = stacked_sums[si - 1][i];
                let frac = (prev_val / max_val).clamp(0.0, 1.0);
                let y = f.pt + ((1.0 - frac) * f.ph as f64) as i32;
                push_b(&mut f.buf, b" L"); push_i(&mut f.buf, x); f.buf.push(b','); push_i(&mut f.buf, y);
            }
        } else {
            let x_last = f.pl + ((n_pts - 1) as f64 * step_x) as i32;
            push_b(&mut f.buf, b" L"); push_i(&mut f.buf, x_last); f.buf.push(b','); push_i(&mut f.buf, base_y);
            push_b(&mut f.buf, b" Z");
        }
        push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\".35\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"1.5\"/>");
    }
    let hover_step = ((n_pts as f64 / 30.0).ceil() as usize).max(1);
    for si in 0..n_ser {
        let (sname, svals) = &series[si];
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        let mut sname_esc = Vec::with_capacity(sname.len() + 8);
        escape_xml(&mut sname_esc, sname);
        for i in (0..n_pts).step_by(hover_step) {
            let val = if cfg.stacked { stacked_sums[si][i] } else { svals.get(i).copied().unwrap_or(0.0) };
            let frac = (val / max_val).clamp(0.0, 1.0);
            let x = f.pl + (i as f64 * step_x) as i32;
            let y = f.pt + ((1.0 - frac) * f.ph as f64) as i32;
            let idx = (si * n_pts + i) as i32;
            push_b(&mut f.buf, b"<circle data-series=\""); push_i(&mut f.buf, si as i32);
            push_b(&mut f.buf, b"\" data-idx=\""); push_i(&mut f.buf, idx);
            push_b(&mut f.buf, b"\" data-y=\""); push_f2(&mut f.buf, svals.get(i).copied().unwrap_or(0.0));
            push_b(&mut f.buf, b"\" data-lbl=\""); f.buf.extend_from_slice(&sname_esc);
            push_b(&mut f.buf, b"\" cx=\""); push_i(&mut f.buf, x);
            push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, y);
            push_b(&mut f.buf, b"\" r=\"2.5\" fill=\"#"); f.buf.extend_from_slice(&hx);
            push_b(&mut f.buf, b"\" stroke=\"#fff\" stroke-width=\".8\" opacity=\"0\"/>");
        }
    }
    let tick_step = ((n_pts as f64 / 12.0).ceil() as usize).max(1);
    for i in (0..n_pts).step_by(tick_step) {
        let x = f.pl + (i as f64 * step_x) as i32;
        push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, x);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, f.pt + f.ph + 14);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#6b7280\" class=\"sp-xt\">");
        escape_xml(&mut f.buf, truncate(&cfg.x_labels[i], 12));
        push_b(&mut f.buf, b"</text>");
    }
    let leg_x = cfg.width - legend_w + 14;
    for (si, (sname, _)) in series.iter().enumerate() {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        let ly = f.pt + 6 + si as i32 * 18;
        push_b(&mut f.buf, b"<g data-legend=\"1\" data-series=\""); push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b"\"><rect x=\""); push_i(&mut f.buf, leg_x);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, ly);
        push_b(&mut f.buf, b"\" width=\"12\" height=\"12\" rx=\"2\" fill=\"#");
        f.buf.extend_from_slice(&hx); push_b(&mut f.buf, b"\" fill-opacity=\".5\"/>");
        push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, leg_x + 16);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, ly + 10);
        push_b(&mut f.buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#374151\">");
        escape_xml(&mut f.buf, truncate(sname, 18));
        push_b(&mut f.buf, b"</text></g>");
    }
    let slots_json;
    let json: &str = if auto_hover { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    f.html(json)
}


