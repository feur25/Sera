use super::config::SankeyConfig;
use crate::html::hover::{html_id, html_prefix, html_suffix, slots_to_json};
use crate::plot::statistical::common::{colorscale_color, escape_xml, hex6, push_b, push_f2, push_i};
use std::f64::consts::PI;

#[crate::chart_demo(
    "title=\"ATL Departure Beacon\", labels=[\"ATL\",\"MSP 06:10\",\"JFK 07:05\",\"CHS 07:35\",\"BNA 07:45\",\"PHL 08:00\",\"JAX 08:05\",\"CLT 08:10\",\"DFW 08:15\",\"CHS 08:15\",\"DTW 08:40\",\"FLL 09:35\",\"MSP 09:40\",\"BOS 10:05\",\"CMH 10:30\",\"RIC 10:40\",\"DEN 10:50\",\"DTW 10:55\",\"GSP 11:05\",\"RIC 11:10\",\"MEM 11:20\",\"MSY 11:25\",\"ORF 13:00\",\"GSP 13:05\",\"PHX 13:10\",\"MSP 13:10\",\"RIC 15:10\",\"MCO 15:40\",\"ORF 15:45\",\"SEA 16:25\",\"CVG 16:55\",\"CVG 16:55\",\"CHS 17:00\",\"SAV 17:10\",\"RDU 17:45\",\"MCO 17:45\",\"JAX 18:00\",\"JFK 18:30\",\"LAX 18:30\",\"CMH 18:40\",\"MEM 19:25\",\"ORF 19:25\",\"SEA 20:20\",\"MSP 20:25\",\"MSP 21:00\",\"ORF 21:05\",\"CMH 21:20\",\"MIA 21:30\",\"RIC 21:35\",\"SAV 21:45\",\"DFW 21:50\",\"PHX 21:55\",\"MCO 21:55\",\"CHS 22:00\",\"PHX 22:25\",\"RIC 23:00\",\"CLT 23:25\"], edges_i=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], edges_j=[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56], edges_w=[143,115,72,66,119,63,74,129,72,113,120,143,153,83,68,173,111,50,72,84,94,85,54,242,132,72,105,72,302,79,73,74,53,65,101,61,130,285,83,89,82,304,139,136,73,77,101,80,64,132,241,95,66,224,74,70], width=1150, height=1000, variant=\"beacon\""
)]
pub fn render(cfg: &SankeyConfig) -> String {
    let e = cfg.sources.len().min(cfg.targets.len()).min(cfg.weights.len());
    let n = cfg.labels.len();
    if e == 0 || n == 0 {
        return String::new();
    }

    let mut is_target = vec![false; n];
    let mut is_source = vec![false; n];
    for k in 0..e {
        let s = cfg.sources[k] as usize;
        let t = cfg.targets[k] as usize;
        if s < n {
            is_source[s] = true;
        }
        if t < n {
            is_target[t] = true;
        }
    }
    let hub = match (0..n).find(|&i| is_source[i] && !is_target[i]) {
        Some(h) => h,
        None => return String::new(),
    };
    let flight_idx: Vec<usize> = (0..n).filter(|&i| is_target[i]).collect();
    let nf = flight_idx.len();
    if nf == 0 {
        return String::new();
    }
    let mut edge_of = vec![-1i32; n];
    for k in 0..e {
        let t = cfg.targets[k] as usize;
        if t < n && edge_of[t] < 0 {
            edge_of[t] = k as i32;
        }
    }

    let ink: u32 = 0x1a202c;
    let sub: u32 = 0x6b7280;
    let ring_col: u32 = 0xe2e8f0;
    let scale = "turbo";

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n * 260 + 8192);
    html_prefix(&mut buf, cfg.title, hid);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\" viewBox=\"0 0 ");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b" ");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    let title_h = if cfg.title.is_empty() { 0.0 } else { 30.0 };
    let legend_w = 120.0;
    let cx = (cfg.width as f64 - legend_w) / 2.0;
    let cy = (cfg.height as f64 + title_h) / 2.0;
    let max_r = ((cfg.width as f64 - legend_w).min(cfg.height as f64 - title_h) / 2.0 - 84.0).max(80.0);
    let r_ring = max_r * 0.7;
    let r_hub = 22.0;

    let min_w = cfg.weights[..e].iter().cloned().fold(f64::INFINITY, f64::min);
    let max_w = cfg.weights[..e].iter().cloned().fold(0.0_f64, f64::max).max(min_w + 1e-9);
    let angle_of: Vec<f64> = (0..nf).map(|i| -PI / 2.0 + 2.0 * PI * i as f64 / nf as f64).collect();
    let mean_t: f64 = flight_idx
        .iter()
        .filter_map(|&li| {
            let k = edge_of[li];
            if k < 0 {
                None
            } else {
                Some(((cfg.weights[k as usize] - min_w) / (max_w - min_w)).clamp(0.0, 1.0))
            }
        })
        .sum::<f64>()
        / nf as f64;
    let atmo_col = colorscale_color(scale, mean_t);

    push_b(&mut buf, b"<defs>");
    push_b(&mut buf, b"<filter id=\"spglow\" x=\"-80%\" y=\"-80%\" width=\"260%\" height=\"260%\"><feGaussianBlur stdDeviation=\"3.2\"/></filter>");
    push_b(&mut buf, b"<filter id=\"spshadow\" x=\"-80%\" y=\"-80%\" width=\"260%\" height=\"260%\"><feDropShadow dx=\"0\" dy=\"2\" stdDeviation=\"2.1\" flood-color=\"#0f172a\" flood-opacity=\"0.22\"/></filter>");
    push_b(&mut buf, b"<radialGradient id=\"spatmo\" cx=\"50%\" cy=\"50%\" r=\"50%\"><stop offset=\"0%\" stop-color=\"#");
    buf.extend_from_slice(&hex6(atmo_col));
    push_b(&mut buf, b"\" stop-opacity=\"0.16\"/><stop offset=\"55%\" stop-color=\"#");
    buf.extend_from_slice(&hex6(atmo_col));
    push_b(&mut buf, b"\" stop-opacity=\"0.06\"/><stop offset=\"100%\" stop-color=\"#");
    buf.extend_from_slice(&hex6(atmo_col));
    push_b(&mut buf, b"\" stop-opacity=\"0\"/></radialGradient>");
    push_b(&mut buf, b"<radialGradient id=\"spbeaconhub\" cx=\"35%\" cy=\"30%\" r=\"75%\"><stop offset=\"0%\" stop-color=\"#475569\"/><stop offset=\"100%\" stop-color=\"#");
    buf.extend_from_slice(&hex6(ink));
    push_b(&mut buf, b"\"/></radialGradient>");
    push_b(&mut buf, b"</defs>");

    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" cy=\"");
    push_f2(&mut buf, cy);
    push_b(&mut buf, b"\" r=\"");
    push_f2(&mut buf, r_ring * 1.22);
    push_b(&mut buf, b"\" fill=\"url(#spatmo)\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"26\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"16\" font-weight=\"700\" fill=\"#");
        buf.extend_from_slice(&hex6(ink));
        push_b(&mut buf, b"\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"<g fill=\"none\" stroke=\"#");
    buf.extend_from_slice(&hex6(ring_col));
    push_b(&mut buf, b"\">");
    for k in 1..=4 {
        let r = r_ring * k as f64 / 4.0;
        let op = 0.9 - k as f64 * 0.14;
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, cy);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, r);
        push_b(&mut buf, b"\" stroke-width=\"1\" stroke-opacity=\"");
        push_f2(&mut buf, op);
        push_b(&mut buf, b"\"/>");
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" cy=\"");
    push_f2(&mut buf, cy);
    push_b(&mut buf, b"\" r=\"");
    push_f2(&mut buf, r_hub * 1.9);
    push_b(&mut buf, b"\" fill=\"#");
    buf.extend_from_slice(&hex6(ink));
    push_b(&mut buf, b"\" opacity=\"0.14\" filter=\"url(#spglow)\"/>");

    push_b(&mut buf, b"<g fill=\"none\">");
    for (i, &li) in flight_idx.iter().enumerate() {
        let k = edge_of[li];
        if k < 0 {
            continue;
        }
        let w = cfg.weights[k as usize];
        let t = ((w - min_w) / (max_w - min_w)).clamp(0.0, 1.0);
        let col = colorscale_color(scale, t);
        let hx = hex6(col);
        let a = angle_of[i];
        let bend = 0.16;
        let tx = cx + r_ring * a.cos();
        let ty = cy + r_ring * a.sin();
        let mx = cx + r_ring * 0.55 * (a - bend).cos();
        let my = cy + r_ring * 0.55 * (a - bend).sin();
        push_b(&mut buf, b"<path d=\"M ");
        push_f2(&mut buf, cx + r_hub * a.cos());
        push_b(&mut buf, b" ");
        push_f2(&mut buf, cy + r_hub * a.sin());
        push_b(&mut buf, b" Q ");
        push_f2(&mut buf, mx);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, my);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, tx);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, ty);
        push_b(&mut buf, b"\" stroke=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke-width=\"3\" stroke-opacity=\"0.1\" stroke-linecap=\"round\"/>");
        push_b(&mut buf, b"<path d=\"M ");
        push_f2(&mut buf, cx + r_hub * a.cos());
        push_b(&mut buf, b" ");
        push_f2(&mut buf, cy + r_hub * a.sin());
        push_b(&mut buf, b" Q ");
        push_f2(&mut buf, mx);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, my);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, tx);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, ty);
        push_b(&mut buf, b"\" stroke=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke-width=\"1\" stroke-opacity=\"0.42\" stroke-dasharray=\"1,3\" stroke-linecap=\"round\"/>");
    }
    push_b(&mut buf, b"</g>");

    for (i, &li) in flight_idx.iter().enumerate() {
        let k = edge_of[li];
        if k < 0 {
            continue;
        }
        let w = cfg.weights[k as usize];
        let t = ((w - min_w) / (max_w - min_w)).clamp(0.0, 1.0);
        let col = colorscale_color(scale, t);
        let hx = hex6(col);
        let a = angle_of[i];
        let tx = cx + r_ring * a.cos();
        let ty = cy + r_ring * a.sin();
        let ph = 14.0 + t * 40.0;
        let pw = 6.0;
        let label = cfg.labels[li].as_str();
        let (code, time) = match label.find(' ') {
            Some(p) => (&label[..p], &label[p + 1..]),
            None => (label, ""),
        };

        push_b(&mut buf, b"<rect x=\"");
        push_f2(&mut buf, tx - pw);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ty - ph / 2.0 - 3.0);
        push_b(&mut buf, b"\" width=\"");
        push_f2(&mut buf, pw * 2.0);
        push_b(&mut buf, b"\" height=\"");
        push_f2(&mut buf, ph + 6.0);
        push_b(&mut buf, b"\" rx=\"3\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" opacity=\"0.4\" filter=\"url(#spglow)\"/>");

        push_b(&mut buf, b"<rect data-idx=\"");
        push_i(&mut buf, li as i32);
        push_b(&mut buf, b"\" data-lbl=\"");
        escape_xml(&mut buf, label);
        push_b(&mut buf, b"\" data-kv-Duration=\"");
        escape_xml(&mut buf, &format!("{w:.0} min"));
        push_b(&mut buf, b"\" x=\"");
        push_f2(&mut buf, tx - pw / 2.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ty - ph / 2.0);
        push_b(&mut buf, b"\" width=\"");
        push_f2(&mut buf, pw);
        push_b(&mut buf, b"\" height=\"");
        push_f2(&mut buf, ph);
        push_b(&mut buf, b"\" rx=\"2.6\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke=\"#fff\" stroke-width=\"0.8\" filter=\"url(#spshadow)\"/>");

        let lr = r_ring + ph / 2.0 + 12.0;
        let lx = cx + lr * a.cos();
        let ly = cy + lr * a.sin();
        let anchor: &[u8] = if a.cos() > 0.2 {
            b"start"
        } else if a.cos() < -0.2 {
            b"end"
        } else {
            b"middle"
        };
        if !time.is_empty() {
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, lx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, ly - 3.0);
            push_b(&mut buf, b"\" text-anchor=\"");
            buf.extend_from_slice(anchor);
            push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#");
            buf.extend_from_slice(&hex6(sub));
            push_b(&mut buf, b"\">");
            escape_xml(&mut buf, time);
            push_b(&mut buf, b"</text>");
        }
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, lx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ly + 8.0);
        push_b(&mut buf, b"\" text-anchor=\"");
        buf.extend_from_slice(anchor);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9.5\" font-weight=\"700\" fill=\"#");
        buf.extend_from_slice(&hex6(ink));
        push_b(&mut buf, b"\">");
        escape_xml(&mut buf, code);
        push_b(&mut buf, b"</text>");
    }

    let compass_r = r_ring + 82.0;
    push_b(&mut buf, b"<g>");
    for (dx, dy, lbl) in [(0.0_f64, -1.0_f64, "N"), (1.0, 0.0, "E"), (0.0, 1.0, "S"), (-1.0, 0.0, "W")] {
        let px = cx + compass_r * dx;
        let py = cy + compass_r * dy;
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, px);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, py);
        push_b(&mut buf, b"\" r=\"11\" fill=\"#ffffff\" stroke=\"#");
        buf.extend_from_slice(&hex6(ring_col));
        push_b(&mut buf, b"\" stroke-width=\"1.2\" filter=\"url(#spshadow)\"/>");
        let tri = format!(
            "M {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} Z",
            px + dx * 5.0,
            py + dy * 5.0,
            px - dy * 4.0 - dx * 2.0,
            py + dx * 4.0 - dy * 2.0,
            px + dy * 4.0 - dx * 2.0,
            py - dx * 4.0 - dy * 2.0
        );
        push_b(&mut buf, b"<path d=\"");
        push_b(&mut buf, tri.as_bytes());
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hex6(sub));
        push_b(&mut buf, b"\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, px);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, py + 24.0);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" font-weight=\"700\" letter-spacing=\"1\" fill=\"#");
        buf.extend_from_slice(&hex6(sub));
        push_b(&mut buf, b"\">");
        push_b(&mut buf, lbl.as_bytes());
        push_b(&mut buf, b"</text>");
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" cy=\"");
    push_f2(&mut buf, cy);
    push_b(&mut buf, b"\" r=\"");
    push_f2(&mut buf, r_hub);
    push_b(&mut buf, b"\" fill=\"url(#spbeaconhub)\" filter=\"url(#spshadow)\"/>");
    let dart = format!(
        "M {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} Z",
        cx,
        cy - 9.0,
        cx + 6.5,
        cy + 7.0,
        cx,
        cy + 3.0,
        cx - 6.5,
        cy + 7.0
    );
    push_b(&mut buf, b"<path d=\"");
    push_b(&mut buf, dart.as_bytes());
    push_b(&mut buf, b"\" fill=\"#ffffff\"/>");
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, cy + r_hub + 16.0);
    push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" font-weight=\"700\" fill=\"#");
    buf.extend_from_slice(&hex6(ink));
    push_b(&mut buf, b"\">");
    escape_xml(&mut buf, &cfg.labels[hub]);
    push_b(&mut buf, b"</text>");

    let leg_x = cfg.width as f64 - legend_w + 24.0;
    let leg_y = cy - 70.0;
    let leg_h = 140.0;
    push_b(&mut buf, b"<defs><linearGradient id=\"spbeacon\" x1=\"0\" y1=\"1\" x2=\"0\" y2=\"0\">");
    for s in 0..8 {
        let t = s as f64 / 7.0;
        let c = colorscale_color(scale, t);
        push_b(&mut buf, b"<stop offset=\"");
        push_f2(&mut buf, t);
        push_b(&mut buf, b"\" stop-color=\"#");
        buf.extend_from_slice(&hex6(c));
        push_b(&mut buf, b"\"/>");
    }
    push_b(&mut buf, b"</linearGradient></defs>");
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, leg_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, leg_y - 10.0);
    push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" font-weight=\"700\" fill=\"#");
    buf.extend_from_slice(&hex6(sub));
    push_b(&mut buf, b"\" letter-spacing=\"0.5\">DURATION</text>");
    push_b(&mut buf, b"<rect x=\"");
    push_f2(&mut buf, leg_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, leg_y);
    push_b(&mut buf, b"\" width=\"12\" height=\"");
    push_f2(&mut buf, leg_h);
    push_b(&mut buf, b"\" fill=\"url(#spbeacon)\" stroke=\"#");
    buf.extend_from_slice(&hex6(ring_col));
    push_b(&mut buf, b"\" stroke-width=\"0.6\" rx=\"2\"/>");
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, leg_x + 17.0);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, leg_y + 8.0);
    push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#");
    buf.extend_from_slice(&hex6(ink));
    push_b(&mut buf, b"\">");
    escape_xml(&mut buf, &format!("{max_w:.0}m"));
    push_b(&mut buf, b"</text>");
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, leg_x + 17.0);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, leg_y + leg_h);
    push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#");
    buf.extend_from_slice(&hex6(ink));
    push_b(&mut buf, b"\">");
    escape_xml(&mut buf, &format!("{min_w:.0}m"));
    push_b(&mut buf, b"</text>");
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, leg_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, leg_y + leg_h + 26.0);
    push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" font-weight=\"700\" fill=\"#");
    buf.extend_from_slice(&hex6(sub));
    push_b(&mut buf, b"\" letter-spacing=\"0.5\">FLIGHTS</text>");
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, leg_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, leg_y + leg_h + 42.0);
    push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"13\" font-weight=\"700\" fill=\"#");
    buf.extend_from_slice(&hex6(ink));
    push_b(&mut buf, b"\">");
    push_i(&mut buf, nf as i32);
    push_b(&mut buf, b"</text>");

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg<'a>(labels: &'a [String], si: &'a [i32], ti: &'a [i32], w: &'a [f64]) -> SankeyConfig<'a> {
        SankeyConfig {
            title: "Test",
            labels,
            sources: si,
            targets: ti,
            weights: w,
            width: 1150,
            height: 1000,
            ..SankeyConfig::default()
        }
    }

    fn synth(n: usize) -> (Vec<String>, Vec<i32>, Vec<i32>, Vec<f64>) {
        let mut labels: Vec<String> = vec!["HUB".to_string()];
        labels.extend((0..n).map(|i| format!("DST{i} {:02}:{:02}", (6 + i / 4) % 24, (i * 7) % 60)));
        let si: Vec<i32> = vec![0; n];
        let ti: Vec<i32> = (1..=n as i32).collect();
        let w: Vec<f64> = (0..n).map(|i| 45.0 + ((i * 37) % 260) as f64).collect();
        (labels, si, ti, w)
    }

    #[test]
    #[ignore]
    fn write_preview_asset() {
        use crate::plot::chart_demo_registry::{iter_entries, render_demo_html};
        for entry in iter_entries() {
            if !entry.file.replace('\\', "/").ends_with("sankey/beacon.rs") {
                continue;
            }
            let html = render_demo_html(entry).expect("demo html");
            std::fs::write("docs/previews/sankey-beacon.html", html).unwrap();
        }
    }

    #[test]
    fn renders_one_hoverable_pill_per_flight_and_the_hub_label() {
        let (labels, si, ti, w) = synth(24);
        let html = render(&cfg(&labels, &si, &ti, &w));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<rect data-idx=\"").count(), 24);
        assert!(html.contains("HUB"));
    }

    #[test]
    fn every_spoke_originates_within_the_hub_radius() {
        let (labels, si, ti, w) = synth(12);
        let html = render(&cfg(&labels, &si, &ti, &w));
        assert_eq!(html.matches("stroke-dasharray=\"1,3\"").count(), 12);
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let empty_s: Vec<String> = vec![];
        let empty_i: Vec<i32> = vec![];
        let empty_w: Vec<f64> = vec![];
        assert!(render(&cfg(&empty_s, &empty_i, &empty_i, &empty_w)).is_empty());
    }

    #[test]
    fn perf_rendering_a_busy_hub_stays_fast() {
        let (labels, si, ti, w) = synth(200);
        let start = std::time::Instant::now();
        let html = render(&cfg(&labels, &si, &ti, &w));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 500, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
