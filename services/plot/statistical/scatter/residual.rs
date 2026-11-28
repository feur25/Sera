use super::common::{draw_marker, make_frame};
use super::config::ScatterConfig;
use super::regression::{fit_linear, fit_poly2};
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{hex6, push_b, push_f2, push_i};

#[crate::chart_demo("x=[1,2,3,4,5,6,7,8,9,10], y=[2,3.8,5.1,7.2,8.5,10.3,11.8,13.4,15.1,16.7]")]

pub fn render(cfg: &ScatterConfig) -> String {
    let n = cfg.x_values.len().min(cfg.y_values.len());
    if n < 2 {
        return String::new();
    }
    let xs = &cfg.x_values[..n];
    let ys = &cfg.y_values[..n];

    let fitted: Vec<f64> = match cfg.regression_type {
        "polynomial2" | "poly2" | "quadratic" => match fit_poly2(xs, ys) {
            Some((c0, c1, c2)) => xs.iter().map(|&x| c0 + c1 * x + c2 * x * x).collect(),
            None => return String::new(),
        },
        _ => match fit_linear(xs, ys) {
            Some((slope, intercept, _)) => xs.iter().map(|&x| intercept + slope * x).collect(),
            None => return String::new(),
        },
    };
    let residuals: Vec<f64> = ys.iter().zip(fitted.iter()).map(|(&y, &f)| y - f).collect();

    let mut xmin = f64::INFINITY;
    let mut xmax = f64::NEG_INFINITY;
    let mut rmax_abs = 0.0f64;
    for i in 0..n {
        xmin = xmin.min(xs[i]);
        xmax = xmax.max(xs[i]);
        rmax_abs = rmax_abs.max(residuals[i].abs());
    }
    let xr = (xmax - xmin).max(1e-9);
    let xpad = xr * 0.06;
    let xmin2 = xmin - xpad;
    let xmax2 = xmax + xpad;
    let rpad = rmax_abs.max(1e-9) * 0.18;
    let ymin2 = -rmax_abs - rpad;
    let ymax2 = rmax_abs + rpad;
    let yr2 = (ymax2 - ymin2).max(1e-9);
    let xr2 = xmax2 - xmin2;

    let mut f = make_frame(cfg, n, 20);
    f.open(cfg.title, true);
    f.y_grid(5, ymin2, ymax2, cfg.gridlines);
    f.x_grid(6, xmin2, xmax2, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let zero_y = f.pt + f.ph - (((0.0 - ymin2) / yr2) * f.ph as f64) as i32;
    push_b(&mut f.buf, b"<line x1=\"");
    push_i(&mut f.buf, f.pl);
    push_b(&mut f.buf, b"\" y1=\"");
    push_i(&mut f.buf, zero_y);
    push_b(&mut f.buf, b"\" x2=\"");
    push_i(&mut f.buf, f.pl + f.pw);
    push_b(&mut f.buf, b"\" y2=\"");
    push_i(&mut f.buf, zero_y);
    push_b(
        &mut f.buf,
        b"\" stroke=\"#6b7280\" stroke-width=\"1.6\" stroke-dasharray=\"6,4\"/>",
    );

    let color = if cfg.color_hex != 0 { cfg.color_hex } else { 0x636EFA };
    let hx = hex6(color);
    for i in 0..n {
        let cx = f.pl + (((xs[i] - xmin2) / xr2) * f.pw as f64) as i32;
        let cy = f.pt + f.ph - (((residuals[i] - ymin2) / yr2) * f.ph as f64) as i32;
        push_b(&mut f.buf, b"<g data-idx=\"");
        push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-x=\"");
        push_f2(&mut f.buf, xs[i]);
        push_b(&mut f.buf, b"\" data-kv-Residual=\"");
        push_f2(&mut f.buf, residuals[i]);
        push_b(&mut f.buf, b"\">");
        draw_marker(&mut f.buf, cfg.symbol, cx, cy, cfg.point_size, &hx, &hx, cfg.stroke_width, 0.7);
        push_b(&mut f.buf, b"</g>");
    }

    let json;
    let hover_json: &str = if cfg.hover.is_empty() {
        "[]"
    } else {
        json = slots_to_json(cfg.hover);
        &json
    };
    f.html(hover_json)
}
