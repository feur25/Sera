use super::common::{compute_layout, draw_marker, make_frame, point_px};
use super::config::ScatterConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i};

pub fn fit_linear(xs: &[f64], ys: &[f64]) -> Option<(f64, f64, f64)> {
    let n = xs.len() as f64;
    if xs.len() < 2 {
        return None;
    }
    let sx: f64 = xs.iter().sum();
    let sy: f64 = ys.iter().sum();
    let sxx: f64 = xs.iter().map(|x| x * x).sum();
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| x * y).sum();
    let den = n * sxx - sx * sx;
    if den.abs() < 1e-12 {
        return None;
    }
    let slope = (n * sxy - sx * sy) / den;
    let intercept = (sy - slope * sx) / n;
    let mean_y = sy / n;
    let ss_tot: f64 = ys.iter().map(|y| (y - mean_y).powi(2)).sum();
    let ss_res: f64 = xs
        .iter()
        .zip(ys)
        .map(|(x, y)| (y - (intercept + slope * x)).powi(2))
        .sum();
    let r2 = if ss_tot > 1e-12 {
        1.0 - ss_res / ss_tot
    } else {
        0.0
    };
    Some((slope, intercept, r2))
}

fn fit_poly2(xs: &[f64], ys: &[f64]) -> Option<(f64, f64, f64)> {
    if xs.len() < 3 {
        return None;
    }
    let n = xs.len() as f64;
    let sx: f64 = xs.iter().sum();
    let sx2: f64 = xs.iter().map(|x| x * x).sum();
    let sx3: f64 = xs.iter().map(|x| x.powi(3)).sum();
    let sx4: f64 = xs.iter().map(|x| x.powi(4)).sum();
    let sy: f64 = ys.iter().sum();
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| x * y).sum();
    let sx2y: f64 = xs.iter().zip(ys).map(|(x, y)| x * x * y).sum();
    let m = [[n, sx, sx2], [sx, sx2, sx3], [sx2, sx3, sx4]];
    let b = [sy, sxy, sx2y];
    fn det3(a: [[f64; 3]; 3]) -> f64 {
        a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1])
            - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
            + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0])
    }
    let d = det3(m);
    if d.abs() < 1e-12 {
        return None;
    }
    let c0 = det3([
        [b[0], m[0][1], m[0][2]],
        [b[1], m[1][1], m[1][2]],
        [b[2], m[2][1], m[2][2]],
    ]) / d;
    let c1 = det3([
        [m[0][0], b[0], m[0][2]],
        [m[1][0], b[1], m[1][2]],
        [m[2][0], b[2], m[2][2]],
    ]) / d;
    let c2 = det3([
        [m[0][0], m[0][1], b[0]],
        [m[1][0], m[1][1], b[1]],
        [m[2][0], m[2][1], b[2]],
    ]) / d;
    Some((c0, c1, c2))
}

#[crate::chart_demo("x=[1,2,3,4,5,6,7,8,9,10], y=[2,3.8,5.1,7.2,8.5,10.3,11.8,13.4,15.1,16.7]")]

pub fn render(cfg: &ScatterConfig) -> String {
    let layout = match compute_layout(cfg) {
        Some(l) => l,
        None => return String::new(),
    };
    let mut f = make_frame(cfg, layout.n, 20);
    f.open(cfg.title, true);
    f.x_grid(6, layout.xmin2, layout.xmax2, cfg.gridlines);
    f.y_grid(5, layout.ymin2, layout.ymax2, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let color = if cfg.color_hex != 0 {
        cfg.color_hex
    } else {
        0x636EFA
    };
    let hx = hex6(color);

    for i in 0..layout.n {
        let (cx, cy) = point_px(&layout, &f, cfg.x_values[i], cfg.y_values[i]);
        push_b(&mut f.buf, b"<g data-idx=\"");
        push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-x=\"");
        push_f2(&mut f.buf, cfg.x_values[i]);
        push_b(&mut f.buf, b"\" data-y=\"");
        push_f2(&mut f.buf, cfg.y_values[i]);
        if i < cfg.labels.len() {
            push_b(&mut f.buf, b"\" data-lbl=\"");
            escape_xml(&mut f.buf, &cfg.labels[i]);
        }
        push_b(&mut f.buf, b"\">");
        draw_marker(
            &mut f.buf,
            cfg.symbol,
            cx,
            cy,
            cfg.point_size,
            &hx,
            &hx,
            cfg.stroke_width,
            0.55,
        );
        push_b(&mut f.buf, b"</g>");
    }

    let xs = &cfg.x_values[..layout.n];
    let ys = &cfg.y_values[..layout.n];
    let line_color = cfg.color_high;
    let steps = 60usize;
    let xmin = layout.xmin2;
    let xmax = layout.xmax2;
    let dx = (xmax - xmin) / steps as f64;

    let (curve, equation) = match cfg.regression_type {
        "polynomial2" | "poly2" | "quadratic" => {
            if let Some((c0, c1, c2)) = fit_poly2(xs, ys) {
                let pts: Vec<(f64, f64)> = (0..=steps)
                    .map(|k| {
                        let xv = xmin + dx * k as f64;
                        (xv, c0 + c1 * xv + c2 * xv * xv)
                    })
                    .collect();
                (pts, format!("y = {:.3}x² + {:.3}x + {:.3}", c2, c1, c0))
            } else {
                (Vec::new(), String::new())
            }
        }
        _ => {
            if let Some((slope, intercept, r2)) = fit_linear(xs, ys) {
                let pts: Vec<(f64, f64)> = (0..=steps)
                    .map(|k| {
                        let xv = xmin + dx * k as f64;
                        (xv, intercept + slope * xv)
                    })
                    .collect();
                (
                    pts,
                    format!("y = {:.3}x + {:.3}   R² = {:.3}", slope, intercept, r2),
                )
            } else {
                (Vec::new(), String::new())
            }
        }
    };

    if !curve.is_empty() {
        let mut tmp: Vec<u8> = Vec::with_capacity(curve.len() * 16);
        if curve.len() >= 2 {
            let hx = hex6(line_color);
            push_b(&mut tmp, b"<path fill=\"none\" stroke=\"#");
            tmp.extend_from_slice(&hx);
            push_b(
                &mut tmp,
                b"\" stroke-width=\"2.4\" stroke-dasharray=\"6,4\" stroke-linecap=\"round\" d=\"",
            );
            for (k, &(xv, yv)) in curve.iter().enumerate() {
                let (px, py) = point_px(&layout, &f, xv, yv);
                if k == 0 {
                    tmp.push(b'M');
                } else {
                    tmp.push(b'L');
                }
                push_i(&mut tmp, px);
                tmp.push(b' ');
                push_i(&mut tmp, py);
            }
            push_b(&mut tmp, b"\"/>");
        }
        f.buf.extend_from_slice(&tmp);
    }

    if !equation.is_empty() {
        let tx = f.pl + 12;
        let ty = f.pt + 18;
        push_b(&mut f.buf, b"<rect x=\"");
        push_i(&mut f.buf, tx - 6);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, ty - 13);
        push_b(&mut f.buf, b"\" rx=\"4\" width=\"");
        push_i(&mut f.buf, (equation.len() as i32) * 7 + 14);
        push_b(
            &mut f.buf,
            b"\" height=\"20\" fill=\"#0f172a\" fill-opacity=\"0.78\" stroke=\"#",
        );
        f.buf.extend_from_slice(&hex6(line_color));
        push_b(&mut f.buf, b"\" stroke-width=\"1\"/>");
        push_b(&mut f.buf, b"<text x=\"");
        push_i(&mut f.buf, tx);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, ty);
        push_b(
            &mut f.buf,
            b"\" font-family=\"Menlo,Consolas,monospace\" font-size=\"11\" fill=\"#e0e7ff\">",
        );
        escape_xml(&mut f.buf, &equation);
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
