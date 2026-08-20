use super::config::SankeyConfig;
use crate::html::hover::{html_id, html_prefix, html_suffix, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hash01, hex6, palette_color, push_b, push_f2, push_i};

#[crate::chart_demo(
    "title=\"Big Data Visualization\", labels=[\"S0\",\"S1\",\"S2\",\"S3\",\"S4\",\"S5\",\"S6\",\"S7\",\"S8\",\"S9\",\"S10\",\"S11\",\"S12\",\"S13\",\"S14\",\"S15\",\"S16\",\"S17\",\"S18\",\"S19\",\"S20\",\"S21\",\"S22\",\"S23\",\"S24\",\"S25\",\"S26\",\"S27\",\"S28\",\"S29\",\"S30\",\"S31\",\"S32\",\"S33\",\"S34\",\"S35\",\"S36\",\"S37\",\"S38\",\"S39\",\"S40\",\"S41\",\"S42\",\"S43\",\"S44\",\"S45\",\"S46\",\"S47\",\"S48\",\"S49\",\"S50\",\"S51\",\"S52\",\"S53\",\"S54\",\"S55\",\"S56\",\"S57\",\"S58\",\"S59\",\"S60\",\"S61\",\"S62\",\"S63\",\"S64\",\"S65\",\"S66\",\"S67\",\"S68\",\"S69\",\"S70\",\"S71\",\"S72\",\"S73\",\"S74\",\"S75\",\"S76\",\"S77\",\"S78\",\"S79\",\"S80\",\"S81\",\"S82\",\"S83\",\"S84\",\"S85\",\"S86\",\"S87\",\"S88\",\"S89\",\"S90\",\"S91\",\"S92\",\"S93\",\"S94\",\"S95\",\"S96\",\"S97\",\"S98\",\"S99\",\"S100\",\"S101\",\"S102\",\"S103\",\"S104\",\"S105\",\"S106\",\"S107\",\"S108\",\"S109\",\"S110\",\"S111\",\"S112\",\"S113\",\"S114\",\"S115\",\"S116\",\"S117\",\"S118\",\"S119\",\"S120\",\"S121\",\"S122\",\"S123\",\"S124\",\"S125\",\"S126\",\"S127\",\"S128\",\"S129\",\"S130\",\"S131\",\"S132\",\"S133\",\"S134\",\"S135\",\"S136\",\"S137\",\"S138\",\"S139\",\"S140\",\"S141\",\"S142\",\"S143\",\"S144\",\"S145\",\"S146\",\"S147\",\"S148\",\"S149\",\"S150\",\"S151\",\"S152\",\"S153\",\"S154\",\"S155\",\"S156\",\"S157\",\"S158\",\"S159\",\"S160\",\"S161\",\"S162\",\"S163\",\"S164\",\"S165\",\"S166\",\"S167\",\"S168\",\"S169\",\"S170\",\"S171\",\"S172\",\"S173\",\"S174\",\"S175\",\"S176\",\"S177\",\"S178\",\"S179\",\"S180\",\"S181\",\"S182\",\"S183\",\"S184\",\"S185\",\"S186\",\"S187\",\"S188\",\"S189\",\"S190\",\"S191\",\"S192\",\"S193\",\"S194\",\"S195\",\"S196\",\"S197\",\"S198\",\"S199\",\"S200\",\"S201\",\"S202\",\"S203\",\"S204\",\"S205\",\"S206\",\"S207\",\"S208\",\"S209\",\"S210\",\"S211\",\"S212\",\"S213\",\"S214\",\"S215\",\"S216\",\"S217\",\"S218\",\"S219\",\"Purchased\",\"Abandoned Cart\",\"Browsed Only\",\"Bounced\"], edges_i=[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99,100,101,102,103,104,105,106,107,108,109,110,111,112,113,114,115,116,117,118,119,120,121,122,123,124,125,126,127,128,129,130,131,132,133,134,135,136,137,138,139,140,141,142,143,144,145,146,147,148,149,150,151,152,153,154,155,156,157,158,159,160,161,162,163,164,165,166,167,168,169,170,171,172,173,174,175,176,177,178,179,180,181,182,183,184,185,186,187,188,189,190,191,192,193,194,195,196,197,198,199,200,201,202,203,204,205,206,207,208,209,210,211,212,213,214,215,216,217,218,219], edges_j=[220,221,221,222,221,222,222,222,221,220,220,222,221,220,221,221,221,222,223,223,222,223,220,220,223,220,223,221,222,221,221,222,223,221,220,222,221,223,223,220,222,223,220,223,222,222,221,220,222,221,222,223,223,220,222,222,223,221,222,222,221,220,223,222,223,223,223,221,222,220,222,220,221,223,222,221,221,222,222,223,221,221,220,220,220,222,220,223,223,221,222,223,222,221,222,220,221,221,221,220,221,222,220,222,222,220,221,222,220,220,221,222,223,223,221,222,222,221,223,223,223,221,223,222,222,221,221,223,223,223,223,223,223,222,222,220,221,221,221,223,221,222,220,221,220,220,222,221,223,222,223,221,221,222,223,220,222,222,221,221,220,222,223,222,223,221,223,223,220,223,223,222,221,220,220,222,220,222,223,222,221,220,222,223,222,220,221,220,222,220,220,220,220,222,223,220,222,220,221,222,223,221,223,221,223,222,222,222,220,221,222,222,223,223,220,222,221,221,221,221], edges_w=[120.6,43.7,21.2,29.1,48.9,34.7,6.3,28.2,33.9,88.7,58.6,18.3,23.8,69.5,26.1,68.4,17.4,20.5,5.3,4.8,16.7,6.6,96.4,115.5,2.2,67.6,8.8,26.6,15.0,36.8,54.7,14.0,7.4,48.3,127.1,30.0,20.1,5.7,2.3,83.0,7.3,8.8,122.6,10.0,19.2,30.5,47.5,83.0,8.9,31.2,23.0,3.7,9.1,53.5,9.1,8.8,7.9,37.7,25.5,21.6,62.0,84.3,8.1,21.1,8.6,3.4,3.3,27.4,15.0,41.4,15.8,61.4,54.4,5.8,19.4,62.8,28.0,6.0,16.0,5.1,15.2,50.1,54.7,65.9,125.0,11.1,93.1,1.3,8.2,44.9,19.9,8.3,14.7,52.6,13.7,138.6,42.0,36.4,17.1,53.0,26.2,22.9,114.8,14.1,6.0,100.8,34.8,32.3,107.7,51.7,61.1,11.5,8.5,5.2,22.2,8.4,10.7,69.9,1.0,2.0,2.0,59.2,7.5,13.6,5.2,50.8,39.2,7.0,4.3,9.3,8.3,3.7,5.8,5.1,31.8,114.5,63.7,19.0,33.8,2.0,24.0,13.9,119.8,18.8,119.4,124.9,12.3,67.5,3.2,6.1,1.0,25.7,55.0,34.1,6.2,108.4,15.7,16.9,48.2,54.5,134.9,8.8,4.1,12.7,1.3,22.8,7.5,9.7,83.2,9.4,3.6,31.3,35.6,122.0,63.1,7.6,54.7,19.1,2.9,21.4,27.9,123.8,15.5,10.0,12.8,121.0,22.5,71.4,29.7,108.5,65.7,76.4,57.7,22.6,3.8,113.4,18.0,62.5,28.9,20.0,5.0,45.6,1.5,40.3,7.4,27.7,30.9,25.1,109.1,59.3,8.6,27.2,6.6,1.8,95.4,13.1,50.4,66.3,35.2,46.9], width=1150, height=980, variant=\"matrix\""
)]
pub fn render(cfg: &SankeyConfig) -> String {
    let n = cfg.labels.len();
    let e = cfg.sources.len().min(cfg.targets.len()).min(cfg.weights.len());
    if n == 0 || e == 0 {
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
    let source_idx: Vec<usize> = (0..n).filter(|&i| is_source[i] && !is_target[i]).collect();
    let target_idx: Vec<usize> = (0..n).filter(|&i| is_target[i]).collect();
    let ns = source_idx.len();
    let nt = target_idx.len();
    if ns == 0 || nt == 0 {
        return String::new();
    }

    let pad_l = 34i32;
    let pad_t = 60i32;
    let pad_b = 34i32;
    let pad_r = 150i32;
    let pw = (cfg.width - pad_l - pad_r) as f64;
    let ph = (cfg.height - pad_t - pad_b) as f64;
    let ink: u32 = 0x1a202c;
    let sub: u32 = 0x6b7280;

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n * 160 + e * 220 + 8192);
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

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, cfg.width / 2);
        push_b(&mut buf, b"\" y=\"32\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"16\" font-weight=\"700\" fill=\"#");
        buf.extend_from_slice(&hex6(ink));
        push_b(&mut buf, b"\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    let mut tgt_total = vec![0.0_f64; n];
    for k in 0..e {
        let t = cfg.targets[k] as usize;
        if t < n {
            tgt_total[t] += cfg.weights[k];
        }
    }
    let mut wedge_of = vec![-1i32; n];
    for (wi, &li) in target_idx.iter().enumerate() {
        wedge_of[li] = wi as i32;
    }

    let cols = ((ns as f64).sqrt() * 1.35).ceil().max(6.0) as usize;
    let rows = ns.div_ceil(cols);
    let grid_w = pw * 0.44;
    let grid_x0 = pad_l as f64;
    let grid_y0 = pad_t as f64 + 6.0;
    let grid_h = ph - 30.0;
    let dx = grid_w / cols as f64;
    let dy = grid_h / rows as f64;

    push_b(&mut buf, b"<g stroke=\"#e5e7eb\" stroke-width=\"1\">");
    for c in 0..=cols {
        let x = grid_x0 + c as f64 * dx;
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, x);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, grid_y0);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, x);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, grid_y0 + grid_h);
        push_b(&mut buf, b"\"/>");
    }
    push_b(&mut buf, b"</g>");
    push_b(&mut buf, b"<g stroke=\"#cbd5e1\" stroke-width=\"1\">");
    for c in 0..=cols {
        let x = grid_x0 + c as f64 * dx;
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, x);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, grid_y0 + grid_h + 6.0);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, x);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, grid_y0 + grid_h + 12.0);
        push_b(&mut buf, b"\"/>");
    }
    push_b(&mut buf, b"</g>");

    let max_w = cfg.weights[..e].iter().cloned().fold(0.0_f64, f64::max).max(1.0);
    let mut src_pt = vec![(0.0_f64, 0.0_f64); n];
    for (i, &li) in source_idx.iter().enumerate() {
        let col = i % cols;
        let row = i / cols;
        let jx = (hash01(li * 9973 + 1) - 0.5) * dx * 0.5;
        let jy = (hash01(li * 9973 + 2) - 0.5) * dy * 0.5;
        let px = grid_x0 + (col as f64 + 0.5) * dx + jx;
        let py = grid_y0 + (row as f64 + 0.5) * dy + jy;
        src_pt[li] = (px, py);
    }

    let tgt_x = pad_l as f64 + pw * 0.90;
    let tgt_y: Vec<f64> = (0..nt)
        .map(|wi| grid_y0 + (wi as f64 + 0.5) * grid_h / nt as f64)
        .collect();

    let mut order: Vec<usize> = (0..e).collect();
    order.sort_by(|&a, &b| cfg.weights[b].partial_cmp(&cfg.weights[a]).unwrap_or(std::cmp::Ordering::Equal));

    push_b(&mut buf, b"<g fill=\"none\">");
    for &k in &order {
        let s = cfg.sources[k] as usize;
        let t = cfg.targets[k] as usize;
        if s >= n || t >= n || wedge_of[t] < 0 {
            continue;
        }
        let (sx, sy) = src_pt[s];
        let ty = tgt_y[wedge_of[t] as usize];
        let color = palette_color(cfg.palette, wedge_of[t] as usize);
        let hx = hex6(color);
        let c1x = sx + (tgt_x - sx) * 0.42;
        let c2x = sx + (tgt_x - sx) * 0.68;
        push_b(&mut buf, b"<path data-src=\"");
        push_i(&mut buf, s as i32);
        push_b(&mut buf, b"\" data-tgt=\"");
        push_i(&mut buf, t as i32);
        push_b(&mut buf, b"\" d=\"M ");
        push_f2(&mut buf, sx);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, sy);
        push_b(&mut buf, b" C ");
        push_f2(&mut buf, c1x);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, sy);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, c2x);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, ty);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, tgt_x);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, ty);
        push_b(&mut buf, b"\" stroke=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke-width=\"0.6\" stroke-opacity=\"0.16\"/>");
    }
    push_b(&mut buf, b"</g>");

    for (i, &li) in source_idx.iter().enumerate() {
        let e0 = (0..e).find(|&k| cfg.sources[k] as usize == li);
        let (w, t) = match e0 {
            Some(k) => (cfg.weights[k], cfg.targets[k] as usize),
            None => (0.0, usize::MAX),
        };
        let wi = if t < n { wedge_of[t] } else { -1 };
        let color = if wi >= 0 { palette_color(cfg.palette, wi as usize) } else { 0x94a3b8 };
        let hx = hex6(color);
        let r = 1.4 + (w / max_w).sqrt() * 5.0;
        let (px, py) = src_pt[li];
        push_b(&mut buf, b"<circle data-idx=\"");
        push_i(&mut buf, li as i32);
        push_b(&mut buf, b"\" cx=\"");
        push_f2(&mut buf, px);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, py);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, r);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.82\"/>");
    }

    let tgt_max = target_idx.iter().map(|&li| tgt_total[li]).fold(0.0_f64, f64::max).max(1.0);
    for (wi, &li) in target_idx.iter().enumerate() {
        let color = palette_color(cfg.palette, wi);
        let hx = hex6(color);
        let r = 6.0 + (tgt_total[li] / tgt_max).sqrt() * 22.0;
        let ty = tgt_y[wi];
        push_b(&mut buf, b"<circle data-idx=\"");
        push_i(&mut buf, li as i32);
        push_b(&mut buf, b"\" cx=\"");
        push_f2(&mut buf, tgt_x);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, ty);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, r);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.9\" stroke=\"#ffffff\" stroke-width=\"2\"/>");

        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, tgt_x + r + 10.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ty - 3.0);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"12\" font-weight=\"700\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\">");
        escape_xml(&mut buf, &cfg.labels[li]);
        push_b(&mut buf, b"</text>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, tgt_x + r + 10.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ty + 12.0);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#");
        buf.extend_from_slice(&hex6(sub));
        push_b(&mut buf, b"\">");
        let share = tgt_total[li] / tgt_total.iter().sum::<f64>().max(1e-9) * 100.0;
        push_f2(&mut buf, share);
        push_b(&mut buf, b"% of total</text>");
    }

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
            height: 980,
            ..SankeyConfig::default()
        }
    }

    fn synth(n: usize) -> (Vec<String>, Vec<i32>, Vec<i32>, Vec<f64>) {
        let labels: Vec<String> = (0..n).map(|i| format!("S{i}")).chain(["A", "B", "C", "D"].iter().map(|s| s.to_string())).collect();
        let mut si = Vec::new();
        let mut ti = Vec::new();
        let mut w = Vec::new();
        for s in 0..n {
            si.push(s as i32);
            ti.push((n + s % 4) as i32);
            w.push(5.0 + ((s * 13) % 90) as f64);
        }
        (labels, si, ti, w)
    }

    #[test]
    #[ignore]
    fn write_preview_asset() {
        use crate::plot::chart_demo_registry::{iter_entries, render_demo_html};
        for entry in iter_entries() {
            if !entry.file.replace('\\', "/").ends_with("sankey/matrix.rs") {
                continue;
            }
            let html = render_demo_html(entry).expect("demo html");
            std::fs::write("docs/previews/sankey-matrix.html", html).unwrap();
        }
    }

    #[test]
    fn renders_one_grid_dot_per_source_and_one_ribbon_per_edge() {
        let (labels, si, ti, w) = synth(40);
        let html = render(&cfg(&labels, &si, &ti, &w));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<circle data-idx=\"").count(), 40 + 4);
        assert_eq!(html.matches("data-src=\"").count(), si.len());
    }

    #[test]
    fn every_target_share_sums_to_one_hundred_percent() {
        let (labels, si, ti, w) = synth(40);
        let html = render(&cfg(&labels, &si, &ti, &w));
        let mut total = 0.0;
        for chunk in html.split("</circle>").collect::<Vec<_>>() {
            let _ = chunk;
        }
        assert!(html.matches("% of total</text>").count() == 4);
        let mut acc = 0.0_f64;
        for part in html.split("% of total</text>").take(4) {
            if let Some(p) = part.rfind('>') {
                if let Ok(v) = part[p + 1..].trim().parse::<f64>() {
                    acc += v;
                }
            }
        }
        assert!((acc - 100.0).abs() < 1.0, "shares should sum to ~100%, got {acc}");
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let empty_s: Vec<String> = vec![];
        let empty_i: Vec<i32> = vec![];
        let empty_w: Vec<f64> = vec![];
        assert!(render(&cfg(&empty_s, &empty_i, &empty_i, &empty_w)).is_empty());
    }

    #[test]
    fn perf_rendering_many_records_stays_fast() {
        let (labels, si, ti, w) = synth(1500);
        let start = std::time::Instant::now();
        let html = render(&cfg(&labels, &si, &ti, &w));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 400, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
