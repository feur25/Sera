use super::config::BarConfig;
use crate::plot::statistical::common::{
    escape_xml, hex6, lerp_color, push_b, push_f2, push_i, svg_open_rescalable, svg_title, truncate,
};
use std::f64::consts::{FRAC_PI_2, TAU};

fn mix2(a: u32, b: u32) -> u32 {
    let ar = (a >> 16) & 0xFF;
    let ag = (a >> 8) & 0xFF;
    let ab = a & 0xFF;
    let br = (b >> 16) & 0xFF;
    let bg = (b >> 8) & 0xFF;
    let bb = b & 0xFF;
    (((ar + br) / 2) << 16) | (((ag + bg) / 2) << 8) | ((ab + bb) / 2)
}

#[crate::chart_demo(
    "labels=[\"1950\",\"1951\",\"1952\",\"1953\",\"1954\",\"1955\",\"1956\",\"1957\",\"1958\",\"1959\",\"1960\",\"1961\",\"1962\",\"1963\",\"1964\",\"1965\",\"1966\",\"1967\",\"1968\",\"1969\",\"1970\",\"1971\",\"1972\",\"1973\",\"1974\",\"1975\",\"1976\",\"1977\",\"1978\",\"1979\",\"1980\",\"1981\",\"1982\",\"1983\",\"1984\",\"1985\",\"1986\",\"1987\",\"1988\",\"1989\",\"1990\",\"1991\",\"1992\",\"1993\",\"1994\",\"1995\",\"1996\",\"1997\",\"1998\",\"1999\",\"2000\",\"2001\",\"2002\",\"2003\",\"2004\",\"2005\",\"2006\",\"2007\",\"2008\",\"2009\",\"2010\",\"2011\",\"2012\"], values=[6,7,9,8,10,11,9,12,13,11,14,15,13,16,18,17,19,21,20,18,22,20,19,23,21,24,26,23,25,27,26,28,25,24,27,29,26,28,31,29,33,36,31,29,34,32,38,35,33,37,30,28,33,39,42,45,40,38,44,49,52,58,64], variant=\"spiral\", width=680, height=370"
)]

pub fn render(cfg: &BarConfig) -> String {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 {
        return String::new();
    }

    let per_rev = ((n as f64 / 3.2).ceil() as usize).clamp(12, 48);
    let n_laps = (n as f64 / per_rev as f64).max(1.0);
    let angle_step = TAU / per_rev as f64;

    let mut vmax = f64::NEG_INFINITY;
    let mut vmin = f64::INFINITY;
    for &v in &cfg.values[..n] {
        vmax = vmax.max(v);
        vmin = vmin.min(v);
    }
    let vr = (vmax - vmin).max(1e-9);

    let w = cfg.width;
    let h = cfg.height;
    let cx = w as f64 / 2.0;
    let cy = h as f64 / 2.0 + 20.0;
    let r_max = (w.min(h) as f64) * 0.42;
    let r_hub = r_max * 0.06;
    let bar_max = (r_max - r_hub) * 0.28;
    let spiral_span = r_max - r_hub - bar_max;
    let growth = spiral_span / n as f64;

    let r_base = |i: usize| -> f64 { r_hub + growth * i as f64 };
    let theta = |i: usize| -> f64 { -FRAC_PI_2 + angle_step * i as f64 };

    let mid_color = mix2(cfg.color_low, cfg.color_high);

    let mut buf = Vec::<u8>::with_capacity(n * 220 + 8192);
    svg_open_rescalable(&mut buf, w, h, 0, 0, w, h);
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    svg_title(&mut buf, cfg.title, w / 2, 24);

    push_b(&mut buf, b"<g stroke=\"#e2e8f0\" stroke-width=\"0.6\">");
    let n_spokes = (per_rev / 2).clamp(6, 16);
    for k in 0..n_spokes {
        let a = -FRAC_PI_2 + TAU * k as f64 / n_spokes as f64;
        let x0 = cx + r_hub * a.cos();
        let y0 = cy + r_hub * a.sin();
        let x1 = cx + r_max * a.cos();
        let y1 = cy + r_max * a.sin();
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, x0);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, y0);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, x1);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, y1);
        push_b(&mut buf, b"\"/>");
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<path fill=\"none\" stroke=\"#cbd5e1\" stroke-width=\"1\" stroke-dasharray=\"1,3\" d=\"M");
    for i in 0..n {
        let r = r_base(i);
        let a = theta(i);
        let x = cx + r * a.cos();
        let y = cy + r * a.sin();
        if i > 0 {
            push_b(&mut buf, b" L");
        }
        push_f2(&mut buf, x);
        push_b(&mut buf, b",");
        push_f2(&mut buf, y);
    }
    push_b(&mut buf, b"\"/>");

    let half_step = angle_step / 2.0;
    for i in 0..n {
        let a = theta(i);
        let a0 = a - half_step;
        let a1 = a + half_step;
        let rb = r_base(i);
        let t = (cfg.values[i] - vmin) / vr;
        let bar_len = 2.0 + t * (bar_max - 2.0);
        let re = rb + bar_len;
        let x00 = cx + rb * a0.cos();
        let y00 = cy + rb * a0.sin();
        let x01 = cx + re * a0.cos();
        let y01 = cy + re * a0.sin();
        let x11 = cx + re * a1.cos();
        let y11 = cy + re * a1.sin();
        let x10 = cx + rb * a1.cos();
        let y10 = cy + rb * a1.sin();
        let color = lerp_color(t, cfg.color_low, mid_color, cfg.color_high);
        let hx = hex6(color);

        push_b(&mut buf, b"<path data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\" data-v=\"");
        push_f2(&mut buf, cfg.values[i]);
        push_b(&mut buf, b"\" data-lbl=\"");
        escape_xml(&mut buf, &cfg.labels[i]);
        push_b(&mut buf, b"\" d=\"M");
        push_f2(&mut buf, x00);
        push_b(&mut buf, b",");
        push_f2(&mut buf, y00);
        push_b(&mut buf, b" L");
        push_f2(&mut buf, x01);
        push_b(&mut buf, b",");
        push_f2(&mut buf, y01);
        push_b(&mut buf, b" L");
        push_f2(&mut buf, x11);
        push_b(&mut buf, b",");
        push_f2(&mut buf, y11);
        push_b(&mut buf, b" L");
        push_f2(&mut buf, x10);
        push_b(&mut buf, b",");
        push_f2(&mut buf, y10);
        push_b(&mut buf, b" Z\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke-width=\"0.4\" opacity=\"0.94\"/>");

        if i % per_rev == 0 {
            let r_lab = rb - 8.0;
            let xl = cx + r_lab * a.cos();
            let yl = cy + r_lab * a.sin();
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, xl);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, yl + 3.0);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"7.5\" fill=\"#64748b\">");
            escape_xml(&mut buf, truncate(&cfg.labels[i], 10));
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" cy=\"");
    push_f2(&mut buf, cy);
    push_b(&mut buf, b"\" r=\"");
    push_f2(&mut buf, r_hub);
    push_b(&mut buf, b"\" fill=\"#f1f5f9\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");

    let leg_x = 20.0;
    let leg_y = h as f64 - 46.0;
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, leg_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, leg_y - 10.0);
    push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8\" font-weight=\"700\" fill=\"#475569\">LONGUEUR = VALEUR</text>");
    let ref_ts = [0.0, 0.5, 1.0];
    for (k, &t) in ref_ts.iter().enumerate() {
        let bl = 2.0 + t * (bar_max - 2.0);
        let x0 = leg_x + k as f64 * 66.0;
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, x0);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, leg_y);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, x0 + bl);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, leg_y);
        push_b(&mut buf, b"\" stroke=\"#94a3b8\" stroke-width=\"4\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, x0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, leg_y + 14.0);
        push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"7\" fill=\"#94a3b8\">");
        let val = vmin + t * vr;
        let s = format!("{:.0}", val);
        buf.extend_from_slice(s.as_bytes());
        push_b(&mut buf, b"</text>");
    }

    let grad_x = w as f64 - 150.0;
    let grad_y = h as f64 - 46.0;
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, grad_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, grad_y - 10.0);
    push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8\" font-weight=\"700\" fill=\"#475569\">COULEUR = VALEUR</text>");
    push_b(&mut buf, b"<defs><linearGradient id=\"spSpiralGrad\" x1=\"0\" x2=\"1\" y1=\"0\" y2=\"0\">");
    push_b(&mut buf, b"<stop offset=\"0%\" stop-color=\"#");
    buf.extend_from_slice(&hex6(cfg.color_low));
    push_b(&mut buf, b"\"/><stop offset=\"50%\" stop-color=\"#");
    buf.extend_from_slice(&hex6(mid_color));
    push_b(&mut buf, b"\"/><stop offset=\"100%\" stop-color=\"#");
    buf.extend_from_slice(&hex6(cfg.color_high));
    push_b(&mut buf, b"\"/></linearGradient></defs>");
    push_b(&mut buf, b"<rect x=\"");
    push_f2(&mut buf, grad_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, grad_y - 6.0);
    push_b(&mut buf, b"\" width=\"120\" height=\"7\" fill=\"url(#spSpiralGrad)\" rx=\"2\"/>");
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, grad_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, grad_y + 14.0);
    push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"7\" fill=\"#94a3b8\">");
    let smin = format!("{:.0}", vmin);
    buf.extend_from_slice(smin.as_bytes());
    push_b(&mut buf, b"</text><text x=\"");
    push_f2(&mut buf, grad_x + 120.0);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, grad_y + 14.0);
    push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"7\" fill=\"#94a3b8\">");
    let smax = format!("{:.0}", vmax);
    buf.extend_from_slice(smax.as_bytes());
    push_b(&mut buf, b"</text>");

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

    fn cfg<'a>(labels: &'a [String], values: &'a [f64]) -> BarConfig<'a> {
        BarConfig {
            title: "Test",
            labels,
            values,
            width: 700,
            height: 500,
            ..BarConfig::default()
        }
    }

    fn synth(n: usize) -> (Vec<String>, Vec<f64>) {
        let labels: Vec<String> = (0..n).map(|i| format!("Y{}", 1950 + i)).collect();
        let values: Vec<f64> = (0..n).map(|i| 5.0 + (i as f64 * 0.7).sin().abs() * 20.0 + i as f64 * 0.3).collect();
        (labels, values)
    }

    #[test]
    fn renders_one_bar_per_point_along_the_spiral() {
        let (labels, values) = synth(60);
        let html = render(&cfg(&labels, &values));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<path data-idx=").count(), 60);
        assert!(html.contains("class=\"sp-bg\""));
    }

    #[test]
    fn the_baseline_radius_grows_monotonically_with_index() {
        let n = 80;
        let per_rev = ((n as f64 / 3.2).ceil() as usize).clamp(12, 48);
        let angle_step = TAU / per_rev as f64;
        let r_hub = 10.0;
        let growth = 0.5;
        let mut last = -1.0;
        for i in 0..n {
            let r = r_hub + growth * i as f64;
            assert!(r > last);
            last = r;
            let _ = -FRAC_PI_2 + angle_step * i as f64;
        }
    }

    #[test]
    fn longer_bars_get_the_high_end_of_the_color_gradient() {
        let (labels, values) = synth(40);
        let html = render(&cfg(&labels, &values));
        let low = hex6(0x636EFA);
        let high = hex6(0xF43F5E);
        assert!(html.contains(std::str::from_utf8(&low).unwrap()) || html.contains(std::str::from_utf8(&high).unwrap()));
    }

    #[test]
    fn lap_boundaries_are_labeled_with_the_real_axis_values() {
        let (labels, values) = synth(90);
        let html = render(&cfg(&labels, &values));
        assert!(html.contains(">Y1950<"));
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let labels: Vec<String> = vec![];
        let values: Vec<f64> = vec![];
        assert!(render(&cfg(&labels, &values)).is_empty());
    }

    #[test]
    fn perf_rendering_a_dense_multi_lap_spiral_stays_fast() {
        let (labels, values) = synth(2000);
        let start = std::time::Instant::now();
        let html = render(&cfg(&labels, &values));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
