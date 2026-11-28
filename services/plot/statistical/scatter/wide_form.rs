use super::common::{draw_marker, make_frame};
use super::config::ScatterConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i};

#[crate::chart_demo(
    "x=[1,2,3,4,5,6,7,8,9,10,11,12], series=[[0.5,1.4,0.3,1.8,2.6,2.1,3.4,3.0,4.2,3.7,4.9,5.6],[-0.3,-1.1,-0.8,-1.9,-1.2,-2.4,-1.8,-2.9,-2.2,-3.4,-2.8,-3.9],[0.2,0.9,1.6,1.1,2.0,2.7,2.3,3.1,2.8,3.6,4.1,3.8],[1.0,0.6,1.3,0.4,1.1,0.2,0.9,-0.1,0.5,-0.3,0.3,-0.6]], series_names=[\"a\",\"b\",\"c\",\"d\"], variant=\"wide_form\""
)]

pub fn render(cfg: &ScatterConfig) -> String {
    let n_series = cfg.series.len();
    if n_series == 0 {
        return String::new();
    }
    let n = cfg.x_values.len();
    if n == 0 {
        return String::new();
    }

    let mut xmin = f64::INFINITY;
    let mut xmax = f64::NEG_INFINITY;
    let mut ymin = f64::INFINITY;
    let mut ymax = f64::NEG_INFINITY;
    for i in 0..n {
        let x = cfg.x_values[i];
        xmin = xmin.min(x);
        xmax = xmax.max(x);
        for (_, vals) in cfg.series.iter() {
            if let Some(&y) = vals.get(i) {
                ymin = ymin.min(y);
                ymax = ymax.max(y);
            }
        }
    }
    if !xmin.is_finite() || !ymin.is_finite() {
        return String::new();
    }
    let xr = (xmax - xmin).max(1e-9);
    let yr = (ymax - ymin).max(1e-9);
    let xpad = xr * 0.06;
    let ypad = yr * 0.08;
    let xmin2 = xmin - xpad;
    let xmax2 = xmax + xpad;
    let ymin2 = ymin - ypad;
    let ymax2 = ymax + ypad;
    let xr2 = xmax2 - xmin2;
    let yr2 = ymax2 - ymin2;

    let legend_w: i32 = 150;
    let mut f = make_frame(cfg, n * n_series, legend_w);
    f.open(cfg.title, true);
    f.x_grid(6, xmin2, xmax2, cfg.gridlines);
    f.y_grid(5, ymin2, ymax2, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    for (si, (_, vals)) in cfg.series.iter().enumerate() {
        let col = palette_color(cfg.palette, si);
        let hx = hex6(col);
        for i in 0..n {
            let y = match vals.get(i) {
                Some(&v) => v,
                None => continue,
            };
            let cx = f.pl + (((cfg.x_values[i] - xmin2) / xr2) * f.pw as f64) as i32;
            let cy = f.pt + f.ph - (((y - ymin2) / yr2) * f.ph as f64) as i32;
            push_b(&mut f.buf, b"<g data-idx=\"");
            push_i(&mut f.buf, (si * n + i) as i32);
            push_b(&mut f.buf, b"\" data-x=\"");
            push_f2(&mut f.buf, cfg.x_values[i]);
            push_b(&mut f.buf, b"\" data-y=\"");
            push_f2(&mut f.buf, y);
            push_b(&mut f.buf, b"\">");
            draw_marker(&mut f.buf, "circle", cx, cy, cfg.point_size, &hx, &hx, cfg.stroke_width, 0.78);
            push_b(&mut f.buf, b"</g>");
        }
    }

    let lx = cfg.width - legend_w + 10;
    for (si, (name, _)) in cfg.series.iter().enumerate() {
        let col = palette_color(cfg.palette, si);
        let hx = hex6(col);
        let ly = f.pt + 10 + si as i32 * 22;
        draw_marker(&mut f.buf, "circle", lx + 8, ly, 6.0, &hx, &hx, 1.2, 0.85);
        push_b(&mut f.buf, b"<text x=\"");
        push_i(&mut f.buf, lx + 22);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, ly + 4);
        push_b(
            &mut f.buf,
            b"\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#cbd5e1\">",
        );
        escape_xml(&mut f.buf, name);
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
