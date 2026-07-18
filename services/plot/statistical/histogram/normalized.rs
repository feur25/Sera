use super::common::compute_bins;
use super::config::HistogramConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{hex6, push_b, push_f2, push_i, Frame};

fn gaussian_kde(values: &[f64], xs: &[f64]) -> Vec<f64> {
    let n = values.len();
    if n == 0 {
        return vec![0.0; xs.len()];
    }
    let mean = values.iter().sum::<f64>() / n as f64;
    let var = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n as f64;
    let std = var.sqrt().max(1e-9);
    let h = (1.06 * std * (n as f64).powf(-0.2)).max(1e-9);
    let inv_h = 1.0 / h;
    let norm = 1.0 / ((2.0 * std::f64::consts::PI).sqrt() * h * n as f64);
    xs.iter()
        .map(|&x| {
            values
                .iter()
                .map(|&v| {
                    let z = (x - v) * inv_h;
                    (-0.5 * z * z).exp()
                })
                .sum::<f64>()
                * norm
        })
        .collect()
}

#[crate::chart_demo("values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]")]

pub fn render(cfg: &HistogramConfig) -> String {
    if cfg.values.is_empty() {
        return String::new();
    }
    let (bin_counts, edges) = compute_bins(cfg.values, cfg.bins);
    let n_bins = bin_counts.len();
    if n_bins == 0 {
        return String::new();
    }
    let total = bin_counts.iter().sum::<u64>().max(1) as f64;
    let bw_data = if edges.len() >= 2 {
        (edges[1] - edges[0]).max(1e-12)
    } else {
        1.0
    };
    let densities: Vec<f64> = bin_counts
        .iter()
        .map(|&c| c as f64 / total / bw_data)
        .collect();

    let x_min = edges[0];
    let x_max = *edges.last().unwrap_or(&x_min);
    let n_curve = 96usize;
    let xs: Vec<f64> = (0..n_curve)
        .map(|i| x_min + (x_max - x_min) * i as f64 / (n_curve as f64 - 1.0))
        .collect();
    let kde = gaussian_kde(cfg.values, &xs);

    let max_d = densities
        .iter()
        .cloned()
        .fold(0.0_f64, f64::max)
        .max(kde.iter().cloned().fold(0.0_f64, f64::max))
        .max(1e-9);

    let pad_l: i32 = 56;
    let pad_r: i32 = 20;
    let plot_w = cfg.width - pad_l - pad_r;
    let bw_px = plot_w as f64 / n_bins as f64;
    let mut f = Frame::new_html(
        cfg.title,
        cfg.width,
        cfg.height,
        pad_l,
        36,
        46,
        pad_r,
        n_bins * 240 + n_curve * 32 + 2048,
    );
    f.open(cfg.title, false);
    f.y_grid(5, 0.0, max_d, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);
    let op_i = (cfg.opacity as f64 / 255.0 * 100.0 + 0.5) as i32;
    let hx = hex6(cfg.color);

    for (i, &d) in densities.iter().enumerate() {
        let bh = (d / max_d * f.ph as f64) as i32;
        let x = f.pl + (i as f64 * bw_px) as i32;
        let y = f.pt + f.ph - bh;
        let w_px = (bw_px as i32 - cfg.gap.max(0)).max(1);
        push_b(&mut f.buf, b"<rect data-idx=\"");
        push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-series=\"0\" data-lbl=\"");
        push_f2(&mut f.buf, edges[i]);
        f.buf.extend_from_slice("\u{2013}".as_bytes());
        push_f2(&mut f.buf, edges.get(i + 1).copied().unwrap_or(edges[i]));
        push_b(&mut f.buf, b"\" data-kv-Density=\"");
        push_f2(&mut f.buf, d);
        push_b(&mut f.buf, b"\" data-kv-Count=\"");
        push_i(&mut f.buf, bin_counts[i] as i32);
        push_b(&mut f.buf, b"\" x=\"");
        push_i(&mut f.buf, x);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, y);
        push_b(&mut f.buf, b"\" width=\"");
        push_i(&mut f.buf, w_px);
        push_b(&mut f.buf, b"\" height=\"");
        push_i(&mut f.buf, bh);
        push_b(&mut f.buf, b"\" fill=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.");
        push_i(&mut f.buf, op_i);
        push_b(
            &mut f.buf,
            b"\" rx=\"2\" stroke=\"#fff\" stroke-width=\"0.4\"/>",
        );
    }

    push_b(&mut f.buf, b"<path d=\"");
    let xrange = (x_max - x_min).max(1e-12);
    for (i, (&xv, &yv)) in xs.iter().zip(kde.iter()).enumerate() {
        let px = f.pl + ((xv - x_min) / xrange * f.pw as f64) as i32;
        let py = f.pt + f.ph - (yv / max_d * f.ph as f64) as i32;
        if i == 0 {
            push_b(&mut f.buf, b"M");
        } else {
            push_b(&mut f.buf, b" L");
        }
        push_i(&mut f.buf, px);
        push_b(&mut f.buf, b" ");
        push_i(&mut f.buf, py);
    }
    push_b(&mut f.buf, b"\" fill=\"none\" stroke=\"#f59e0b\" stroke-width=\"2\" stroke-linejoin=\"round\" stroke-linecap=\"round\"/>");

    let tick_step = ((n_bins as f64 / 8.0).ceil() as usize).max(1);
    for i in (0..=n_bins).step_by(tick_step) {
        let x = f.pl + (i as f64 * bw_px) as i32;
        let val = if i < edges.len() {
            edges[i]
        } else {
            *edges.last().unwrap_or(&0.0)
        };
        push_b(&mut f.buf, b"<text x=\"");
        push_i(&mut f.buf, x);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, f.pt + f.ph + 14);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\" class=\"sp-xt\">");
        push_f2(&mut f.buf, val);
        push_b(&mut f.buf, b"</text>");
    }

    let slots_json;
    let json: &str = if cfg.hover.is_empty() {
        "[]"
    } else {
        slots_json = slots_to_json(cfg.hover);
        &slots_json
    };
    f.html(json)
}
