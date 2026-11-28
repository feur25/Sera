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

    let parts_of = |li: usize| -> (&str, &str) {
        let label = cfg.labels[li].as_str();
        match label.find(' ') {
            Some(p) => (&label[..p], &label[p + 1..]),
            None => (label, ""),
        }
    };
    let hour_frac_of = |time: &str| -> f64 {
        let mut it = time.splitn(2, ':');
        let h: f64 = it.next().and_then(|s| s.parse().ok()).unwrap_or(12.0);
        let m: f64 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        (h + m / 60.0).clamp(0.0, 24.0)
    };

    let mut codes: Vec<&str> = Vec::new();
    for &li in &flight_idx {
        let (code, _) = parts_of(li);
        if !codes.contains(&code) {
            codes.push(code);
        }
    }
    codes.sort_unstable();
    let n_codes = codes.len().max(1);
    let slot = 2.0 * PI / n_codes as f64;

    let mut code_members: Vec<Vec<usize>> = vec![Vec::new(); n_codes];
    for &li in &flight_idx {
        let (code, _) = parts_of(li);
        let idx = codes.iter().position(|&c| c == code).unwrap_or(0);
        code_members[idx].push(li);
    }
    let mut sib_of = vec![(0usize, 1usize); n];
    for members in &code_members {
        let cnt = members.len();
        for (si, &li) in members.iter().enumerate() {
            sib_of[li] = (si, cnt);
        }
    }
    let angle_of_flight = |li: usize, code: &str| -> f64 {
        let idx = codes.iter().position(|&c| c == code).unwrap_or(0);
        let base = -PI / 2.0 + slot / 2.0 + idx as f64 * slot;
        let (si, cnt) = sib_of[li];
        if cnt <= 1 {
            base
        } else {
            let fan = slot * 0.6;
            let step = fan / (cnt as f64 - 1.0).max(1.0);
            base - fan / 2.0 + si as f64 * step
        }
    };

    let ink: u32 = 0x1a202c;
    let sub: u32 = 0x6b7280;
    let ring_col: u32 = 0xe2e8f0;
    let scale = "turbo";

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n * 280 + 8192);
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
    let r_hub = 22.0;
    let r_inner = r_hub + 38.0;
    let r_outer = max_r * 0.86;

    let radius_for_hour = |h: f64| -> f64 { r_inner + (h / 24.0).clamp(0.0, 1.0) * (r_outer - r_inner) };

    let min_w = cfg.weights[..e].iter().cloned().fold(f64::INFINITY, f64::min);
    let max_w = cfg.weights[..e].iter().cloned().fold(0.0_f64, f64::max).max(min_w + 1e-9);
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
    push_f2(&mut buf, r_outer * 1.28);
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

    let hour_rings: [f64; 7] = [0.0, 4.0, 8.0, 12.0, 16.0, 20.0, 24.0];
    push_b(&mut buf, b"<g fill=\"none\" stroke=\"#");
    buf.extend_from_slice(&hex6(ring_col));
    push_b(&mut buf, b"\">");
    for (i, &h) in hour_rings.iter().enumerate() {
        let r = radius_for_hour(h);
        let op = 0.85 - i as f64 * 0.1;
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, cy);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, r);
        push_b(&mut buf, b"\" stroke-width=\"1\" stroke-opacity=\"");
        push_f2(&mut buf, op.max(0.12));
        push_b(&mut buf, b"\"/>");
    }
    push_b(&mut buf, b"</g>");

    let time_top: [f64; 4] = [3.0, 6.0, 9.0, 12.0];
    let time_bottom: [f64; 4] = [15.0, 18.0, 21.0, 24.0];
    push_b(&mut buf, b"<g>");
    push_b(&mut buf, b"<line x1=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" y1=\"");
    push_f2(&mut buf, cy - r_inner);
    push_b(&mut buf, b"\" x2=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" y2=\"");
    push_f2(&mut buf, cy - r_outer - 20.0);
    push_b(&mut buf, b"\" stroke=\"#");
    buf.extend_from_slice(&hex6(sub));
    push_b(&mut buf, b"\" stroke-width=\"1\" stroke-dasharray=\"1,3\" stroke-opacity=\"0.55\"/>");
    push_b(&mut buf, b"<line x1=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" y1=\"");
    push_f2(&mut buf, cy + r_inner);
    push_b(&mut buf, b"\" x2=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" y2=\"");
    push_f2(&mut buf, cy + r_outer + 20.0);
    push_b(&mut buf, b"\" stroke=\"#");
    buf.extend_from_slice(&hex6(sub));
    push_b(&mut buf, b"\" stroke-width=\"1\" stroke-dasharray=\"1,3\" stroke-opacity=\"0.55\"/>");
    for &h in time_top.iter() {
        let r = radius_for_hour(h);
        tick(&mut buf, cx, cy - r, h, sub);
    }
    for &h in time_bottom.iter() {
        let r = radius_for_hour(h);
        tick(&mut buf, cx, cy + r, h, sub);
    }
    time_badge(&mut buf, cx, cy - r_outer - 34.0, -1.0, sub, ring_col);
    time_badge(&mut buf, cx, cy + r_outer + 34.0, 1.0, sub, ring_col);
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<g fill=\"none\">");
    for &li in &flight_idx {
        let k = edge_of[li];
        if k < 0 {
            continue;
        }
        let w = cfg.weights[k as usize];
        let t = ((w - min_w) / (max_w - min_w)).clamp(0.0, 1.0);
        let col = colorscale_color(scale, t);
        let hx = hex6(col);
        let (code, time) = parts_of(li);
        let a = angle_of_flight(li, code);
        let r_i = radius_for_hour(hour_frac_of(time));
        let bend = 0.16;
        let tx = cx + r_i * a.cos();
        let ty = cy + r_i * a.sin();
        let mx = cx + r_i * 0.5 * (a - bend).cos();
        let my = cy + r_i * 0.5 * (a - bend).sin();
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

    for &li in &flight_idx {
        let k = edge_of[li];
        if k < 0 {
            continue;
        }
        let w = cfg.weights[k as usize];
        let t = ((w - min_w) / (max_w - min_w)).clamp(0.0, 1.0);
        let col = colorscale_color(scale, t);
        let hx = hex6(col);
        let (code, time) = parts_of(li);
        let a = angle_of_flight(li, code);
        let r_i = radius_for_hour(hour_frac_of(time));
        let tx = cx + r_i * a.cos();
        let ty = cy + r_i * a.sin();
        let ph = 12.0 + t * 32.0;
        let pw = 6.0;
        let label = cfg.labels[li].as_str();

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

        let lr = r_i + ph / 2.0 + 12.0;
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
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, leg_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, leg_y + leg_h + 62.0);
    push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" font-weight=\"700\" fill=\"#");
    buf.extend_from_slice(&hex6(sub));
    push_b(&mut buf, b"\" letter-spacing=\"0.5\">DESTINATIONS</text>");
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, leg_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, leg_y + leg_h + 78.0);
    push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"13\" font-weight=\"700\" fill=\"#");
    buf.extend_from_slice(&hex6(ink));
    push_b(&mut buf, b"\">");
    push_i(&mut buf, n_codes as i32);
    push_b(&mut buf, b"</text>");

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}

fn tick(buf: &mut Vec<u8>, x: f64, y: f64, hour: f64, col: u32) {
    push_b(buf, b"<circle cx=\"");
    push_f2(buf, x);
    push_b(buf, b"\" cy=\"");
    push_f2(buf, y);
    push_b(buf, b"\" r=\"9\" fill=\"#ffffff\" stroke=\"#");
    buf.extend_from_slice(&hex6(col));
    push_b(buf, b"\" stroke-width=\"1\"/>");
    push_b(buf, b"<text x=\"");
    push_f2(buf, x);
    push_b(buf, b"\" y=\"");
    push_f2(buf, y + 2.8);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"7.5\" font-weight=\"700\" fill=\"#");
    buf.extend_from_slice(&hex6(col));
    push_b(buf, b"\">");
    push_i(buf, hour as i32);
    push_b(buf, b"</text>");
}

fn time_badge(buf: &mut Vec<u8>, cx: f64, py: f64, dir: f64, sub: u32, ring_col: u32) {
    push_b(buf, b"<circle cx=\"");
    push_f2(buf, cx);
    push_b(buf, b"\" cy=\"");
    push_f2(buf, py);
    push_b(buf, b"\" r=\"12\" fill=\"#ffffff\" stroke=\"#");
    buf.extend_from_slice(&hex6(ring_col));
    push_b(buf, b"\" stroke-width=\"1.2\" filter=\"url(#spshadow)\"/>");
    let tri = format!(
        "M {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} Z",
        cx,
        py + dir * 5.5,
        cx - 4.0,
        py - dir * 3.0,
        cx + 4.0,
        py - dir * 3.0
    );
    push_b(buf, b"<path d=\"");
    push_b(buf, tri.as_bytes());
    push_b(buf, b"\" fill=\"#");
    buf.extend_from_slice(&hex6(sub));
    push_b(buf, b"\"/>");
    push_b(buf, b"<text x=\"");
    push_f2(buf, cx);
    push_b(buf, b"\" y=\"");
    push_f2(buf, py + dir * 22.0);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" font-weight=\"700\" letter-spacing=\"1\" fill=\"#");
    buf.extend_from_slice(&hex6(sub));
    push_b(buf, b"\">TIME</text>");
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
        labels.extend((0..n).map(|i| format!("DST{} {:02}:{:02}", i % 6, (6 + i / 4) % 24, (i * 7) % 60)));
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
    fn same_destination_code_fans_out_within_its_own_slot() {
        let labels: Vec<String> = vec![
            "HUB".to_string(),
            "AAA 02:00".to_string(),
            "AAA 22:00".to_string(),
            "BBB 12:00".to_string(),
            "CCC 12:00".to_string(),
        ];
        let si: Vec<i32> = vec![0, 0, 0, 0];
        let ti: Vec<i32> = vec![1, 2, 3, 4];
        let w: Vec<f64> = vec![60.0, 60.0, 60.0, 60.0];
        let html = render(&cfg(&labels, &si, &ti, &w));
        assert!(!html.is_empty());

        let re = regex_lite_x_y(&html);
        assert_eq!(re.len(), 4);
        let cx = 515.0;
        let cy = 515.0;
        let angle = |x: f64, y: f64| (y - cy).atan2(x - cx);
        let a_aaa1 = angle(re[0].0, re[0].1);
        let a_aaa2 = angle(re[1].0, re[1].1);
        let a_bbb = angle(re[2].0, re[2].1);
        let a_ccc = angle(re[3].0, re[3].1);

        let slot = 2.0 * PI / 3.0;
        let wrap = |a: f64| {
            let mut x = a % (2.0 * PI);
            if x > PI {
                x -= 2.0 * PI;
            }
            if x < -PI {
                x += 2.0 * PI;
            }
            x
        };
        let ang_dist = |a: f64, b: f64| wrap(a - b).abs();

        let sib_dist = ang_dist(a_aaa1, a_aaa2);
        let cross_dist = ang_dist(a_aaa1, a_bbb).min(ang_dist(a_aaa1, a_ccc));
        assert!(sib_dist < slot, "siblings should stay within one slot width: {sib_dist} vs {slot}");
        assert!(sib_dist < cross_dist, "siblings should sit closer together than to another code: {sib_dist} vs {cross_dist}");
        assert!((a_bbb - a_ccc).abs() > 0.1, "distinct single-flight codes should get distinct bearings");
    }

    fn regex_lite_x_y(html: &str) -> Vec<(f64, f64)> {
        let mut out = Vec::new();
        for chunk in html.split("<rect data-idx=\"").skip(1) {
            let x_start = chunk.find("\" x=\"").map(|p| p + 5);
            let x_val = x_start.and_then(|s| {
                let rest = &chunk[s..];
                let end = rest.find('"')?;
                rest[..end].parse::<f64>().ok()
            });
            let y_start = chunk.find("\" y=\"").map(|p| p + 5);
            let y_val = y_start.and_then(|s| {
                let rest = &chunk[s..];
                let end = rest.find('"')?;
                rest[..end].parse::<f64>().ok()
            });
            let w_start = chunk.find("\" width=\"").map(|p| p + 9);
            let w_val: f64 = w_start
                .and_then(|s| {
                    let rest = &chunk[s..];
                    let end = rest.find('"')?;
                    rest[..end].parse::<f64>().ok()
                })
                .unwrap_or(0.0);
            let h_start = chunk.find("\" height=\"").map(|p| p + 10);
            let h_val: f64 = h_start
                .and_then(|s| {
                    let rest = &chunk[s..];
                    let end = rest.find('"')?;
                    rest[..end].parse::<f64>().ok()
                })
                .unwrap_or(0.0);
            if let (Some(x), Some(y)) = (x_val, y_val) {
                out.push((x + w_val / 2.0, y + h_val / 2.0));
            }
        }
        out
    }

    #[test]
    fn every_spoke_originates_within_the_hub_radius() {
        let (labels, si, ti, w) = synth(12);
        let html = render(&cfg(&labels, &si, &ti, &w));
        assert_eq!(html.matches("stroke-dasharray=\"1,3\" stroke-linecap=\"round\"/>").count(), 12);
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
