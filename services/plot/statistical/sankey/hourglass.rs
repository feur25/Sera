use super::config::SankeyConfig;
use crate::html::hover::{html_id, html_prefix, html_suffix, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i};
use std::f64::consts::PI;

#[crate::chart_demo(
    "title=\"Food to Nutrient Flow\", labels=[\"Spinach\",\"Kale\",\"Salmon\",\"Beef Liver\",\"Egg Yolk\",\"Orange\",\"Red Bell Pepper\",\"Almonds\",\"Sunflower Seeds\",\"Milk\",\"Yogurt\",\"Sardines\",\"Broccoli\",\"Sweet Potato\",\"Carrots\",\"Oysters\",\"Beef Chuck\",\"Pumpkin Seeds\",\"Chickpeas\",\"Lentils\",\"Asparagus\",\"Avocado\",\"Banana\",\"Potato\",\"White Beans\",\"Dark Chocolate\",\"Tuna\",\"Brazil Nuts\",\"Turkey Breast\",\"Chicken Breast\",\"Pork Chop\",\"Cheddar Cheese\",\"Tofu\",\"Brown Rice\",\"Bell Pepper Green\",\"Strawberries\",\"Kiwi\",\"Papaya\",\"Watermelon\",\"Mango\",\"Cantaloupe\",\"Fortified Cereal\",\"Mushrooms\",\"Sesame Seeds\",\"Black Beans\",\"Vitamin K\",\"Iron\",\"Folate\",\"Magnesium\",\"Vitamin A\",\"Vitamin C\",\"Vitamin D\",\"Vitamin B12\",\"Selenium\",\"Potassium\",\"Choline\",\"Vitamin B6\",\"Vitamin E\",\"Calcium\",\"Zinc\"], edges_i=[0,0,0,0,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4,5,5,5,6,6,6,7,7,7,8,8,8,9,9,9,10,10,10,11,11,11,11,12,12,12,13,13,13,14,14,15,15,15,15,16,16,16,17,17,17,18,18,18,19,19,19,20,20,21,21,21,22,22,23,23,23,24,24,24,25,25,25,26,26,26,27,27,27,28,28,28,29,29,30,30,31,31,31,32,32,32,33,33,34,34,35,35,36,36,37,37,38,38,39,39,40,40,41,41,41,42,42,42,43,43,43,44,44,44], edges_j=[45,46,47,48,45,49,50,51,52,53,54,49,52,46,47,51,52,53,55,50,47,54,50,49,56,57,48,58,57,53,48,58,51,52,58,52,59,58,51,52,53,50,45,47,49,54,50,49,45,59,52,53,46,59,46,52,59,48,46,47,46,48,47,46,54,47,45,54,57,47,54,56,54,50,56,46,48,54,48,46,59,51,52,53,53,48,59,59,56,53,56,53,56,59,58,52,59,58,46,48,48,56,50,56,50,47,50,45,50,47,50,49,49,50,49,50,52,46,47,51,53,54,58,46,59,47,46,48], edges_w=[180,15,49,20,684,206,134,66,117,36,12,522,1386,34,65,9,22,20,66,88,10,5,213,31,10,48,19,7,82,24,13,18,15,18,15,12,6,35,46,67,26,89,92,14,214,10,6,184,13,493,324,77,33,43,15,67,23,37,12,71,13,12,90,18,14,34,45,14,10,20,9,20,12,27,15,24,23,13,16,35,18,39,80,131,989,26,7,12,36,31,26,26,31,12,20,14,14,20,15,14,11,7,100,9,97,9,103,31,144,14,21,8,12,67,68,65,25,45,50,18,10,9,18,22,15,64,10,18], width=1250, height=1080, variant=\"hourglass\""
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
    let source_idx: Vec<usize> = (0..n).filter(|&i| is_source[i] && !is_target[i]).collect();
    let target_idx: Vec<usize> = (0..n).filter(|&i| is_target[i]).collect();
    let ns = source_idx.len();
    let nt = target_idx.len();
    if ns == 0 || nt == 0 {
        return String::new();
    }

    let pad_l = 190i32;
    let pad_t = 60i32;
    let pad_b = 30i32;
    let pad_r = 90i32;
    let pw = (cfg.width - pad_l - pad_r) as f64;
    let ph = (cfg.height - pad_t - pad_b) as f64;
    let ink: u32 = 0x1a202c;
    let sub: u32 = 0x6b7280;

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n * 200 + e * 300 + 8192);
    html_prefix(&mut buf, cfg.title, hid);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
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

    let row_h = ph / ns as f64;
    let mut row_of = vec![-1i32; n];
    for (i, &li) in source_idx.iter().enumerate() {
        row_of[li] = i as i32;
    }
    let src_x = pad_l as f64;
    let src_y: Vec<f64> = (0..ns).map(|i| pad_t as f64 + (i as f64 + 0.5) * row_h).collect();

    let cx = pad_l as f64 + pw * 0.68;
    let cy = pad_t as f64 + ph * 0.47;
    let r_outer = (pw * 0.185).max(60.0);
    let r_inner = r_outer - 24.0;
    let start_a = -100.0_f64.to_radians();
    let sweep = 245.0_f64.to_radians();
    let gap_a = 0.9_f64.to_radians();

    let mut tgt_total = vec![0.0_f64; n];
    for k in 0..e {
        let t = cfg.targets[k] as usize;
        if t < n {
            tgt_total[t] += cfg.weights[k];
        }
    }
    let tgt_display: Vec<f64> = (0..n).map(|i| tgt_total[i].sqrt()).collect();
    let grand_display: f64 = target_idx.iter().map(|&i| tgt_display[i]).sum::<f64>().max(1.0);
    let usable = sweep - gap_a * nt as f64;

    let mut angle_start = vec![0.0_f64; n];
    let mut angle_end = vec![0.0_f64; n];
    let mut wedge_of = vec![-1i32; n];
    let mut acc = start_a;
    for (wi, &li) in target_idx.iter().enumerate() {
        let frac = (tgt_display[li] / grand_display).max(0.006);
        let a = usable * frac;
        angle_start[li] = acc;
        angle_end[li] = acc + a;
        wedge_of[li] = wi as i32;
        acc += a + gap_a;
    }

    let mut wedge_cursor = vec![0.0_f64; n];
    let max_w = cfg.weights[..e].iter().cloned().fold(0.0_f64, f64::max).max(1.0);

    let mut order: Vec<usize> = (0..e).collect();
    order.sort_by(|&a, &b| cfg.weights[a].partial_cmp(&cfg.weights[b]).unwrap_or(std::cmp::Ordering::Equal));

    let pinch_x = pad_l as f64 + pw * 0.40;
    let pinch_y = cy;

    push_b(&mut buf, b"<g fill=\"none\">");
    for &k in &order {
        let s = cfg.sources[k] as usize;
        let t = cfg.targets[k] as usize;
        if s >= n || t >= n || row_of[s] < 0 || wedge_of[t] < 0 {
            continue;
        }
        let w = cfg.weights[k];
        let span = angle_end[t] - angle_start[t];
        let frac_before = wedge_cursor[t] / tgt_total[t].max(1e-9);
        wedge_cursor[t] += w;
        let frac_after = wedge_cursor[t] / tgt_total[t].max(1e-9);
        let mid = angle_start[t] + span * (frac_before + frac_after) * 0.5;
        let tx = cx + r_inner * mid.cos();
        let ty = cy + r_inner * mid.sin();

        let sy = src_y[row_of[s] as usize];
        let color = palette_color(cfg.palette, wedge_of[t] as usize);
        let hx = hex6(color);
        let sw = 0.35 + (w / max_w).sqrt() * 5.2;

        push_b(&mut buf, b"<path data-src=\"");
        push_i(&mut buf, s as i32);
        push_b(&mut buf, b"\" data-tgt=\"");
        push_i(&mut buf, t as i32);
        push_b(&mut buf, b"\" d=\"M ");
        push_f2(&mut buf, src_x);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, sy);
        push_b(&mut buf, b" C ");
        push_f2(&mut buf, (src_x + pinch_x) / 2.0);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, sy);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, (src_x + pinch_x) / 2.0);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, pinch_y);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, pinch_x);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, pinch_y);
        push_b(&mut buf, b" C ");
        push_f2(&mut buf, (pinch_x + tx) / 2.0);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, pinch_y);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, (pinch_x + tx) / 2.0);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, ty);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, tx);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, ty);
        push_b(&mut buf, b"\" stroke=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke-width=\"");
        push_f2(&mut buf, sw);
        push_b(&mut buf, b"\" stroke-opacity=\"0.55\" stroke-linecap=\"round\"/>");
    }
    push_b(&mut buf, b"</g>");

    for (i, &li) in source_idx.iter().enumerate() {
        push_b(&mut buf, b"<circle data-idx=\"");
        push_i(&mut buf, li as i32);
        push_b(&mut buf, b"\" cx=\"");
        push_f2(&mut buf, src_x);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, src_y[i]);
        push_b(&mut buf, b"\" r=\"2\" fill=\"#");
        buf.extend_from_slice(&hex6(ink));
        push_b(&mut buf, b"\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, src_x - 8.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, src_y[i] + 3.0);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"8.6\" fill=\"#");
        buf.extend_from_slice(&hex6(sub));
        push_b(&mut buf, b"\">");
        escape_xml(&mut buf, &cfg.labels[li]);
        push_b(&mut buf, b"</text>");
    }

    for (wi, &li) in target_idx.iter().enumerate() {
        let color = palette_color(cfg.palette, wi);
        let hx = hex6(color);
        let a0 = angle_start[li];
        let a1 = angle_end[li];
        push_b(&mut buf, b"<path data-idx=\"");
        push_i(&mut buf, li as i32);
        push_b(&mut buf, b"\" d=\"");
        wedge_path(&mut buf, cx, cy, r_inner, r_outer, a0, a1);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke=\"#ffffff\" stroke-width=\"1.4\"/>");

        let am = (a0 + a1) / 2.0;
        let lr = r_outer + 10.0;
        let lx = cx + lr * am.cos();
        let ly = cy + lr * am.sin();
        let flip = am.cos() < 0.0;
        let rot_deg = if flip { (am + PI).to_degrees() } else { am.to_degrees() };
        let anchor: &[u8] = if flip { b"end" } else { b"start" };
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, lx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b"\" text-anchor=\"");
        buf.extend_from_slice(anchor);
        push_b(&mut buf, b"\" transform=\"rotate(");
        push_f2(&mut buf, rot_deg);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, lx);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b")\" font-family=\"Arial,sans-serif\" font-size=\"11\" font-weight=\"700\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" dominant-baseline=\"middle\">");
        escape_xml(&mut buf, &cfg.labels[li]);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}

fn wedge_path(buf: &mut Vec<u8>, cx: f64, cy: f64, r0: f64, r1: f64, a0: f64, a1: f64) {
    let (x0i, y0i) = (cx + r0 * a0.cos(), cy + r0 * a0.sin());
    let (x1i, y1i) = (cx + r0 * a1.cos(), cy + r0 * a1.sin());
    let (x0o, y0o) = (cx + r1 * a0.cos(), cy + r1 * a0.sin());
    let (x1o, y1o) = (cx + r1 * a1.cos(), cy + r1 * a1.sin());
    let large = if (a1 - a0).abs() > PI { 1 } else { 0 };
    push_b(buf, b"M ");
    push_f2(buf, x0i);
    buf.push(b' ');
    push_f2(buf, y0i);
    push_b(buf, b" L ");
    push_f2(buf, x0o);
    buf.push(b' ');
    push_f2(buf, y0o);
    push_b(buf, b" A ");
    push_f2(buf, r1);
    buf.push(b' ');
    push_f2(buf, r1);
    push_b(buf, b" 0 ");
    push_i(buf, large);
    push_b(buf, b" 1 ");
    push_f2(buf, x1o);
    buf.push(b' ');
    push_f2(buf, y1o);
    push_b(buf, b" L ");
    push_f2(buf, x1i);
    buf.push(b' ');
    push_f2(buf, y1i);
    push_b(buf, b" A ");
    push_f2(buf, r0);
    buf.push(b' ');
    push_f2(buf, r0);
    push_b(buf, b" 0 ");
    push_i(buf, large);
    push_b(buf, b" 0 ");
    push_f2(buf, x0i);
    buf.push(b' ');
    push_f2(buf, y0i);
    push_b(buf, b" Z");
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
            width: 1250,
            height: 1080,
            ..SankeyConfig::default()
        }
    }

    fn synth() -> (Vec<String>, Vec<i32>, Vec<i32>, Vec<f64>) {
        let labels: Vec<String> = (0..10).map(|i| format!("Food{i}")).chain((0..4).map(|i| format!("Nutrient{i}"))).collect();
        let mut si = Vec::new();
        let mut ti = Vec::new();
        let mut w = Vec::new();
        for s in 0..10 {
            for t in 0..2 {
                si.push(s);
                ti.push(10 + ((s + t) % 4));
                w.push(10.0 + ((s * 7 + t * 3) % 40) as f64);
            }
        }
        (labels, si, ti, w)
    }

    #[test]
    #[ignore]
    fn write_preview_asset() {
        use crate::plot::chart_demo_registry::{iter_entries, render_demo_html};
        for entry in iter_entries() {
            if !entry.file.replace('\\', "/").ends_with("sankey/hourglass.rs") {
                continue;
            }
            let html = render_demo_html(entry).expect("demo html");
            std::fs::write("docs/previews/sankey-hourglass.html", html).unwrap();
        }
    }

    #[test]
    fn renders_one_ribbon_per_edge() {
        let (labels, si, ti, w) = synth();
        let html = render(&cfg(&labels, &si, &ti, &w));
        assert!(!html.is_empty());
        assert_eq!(html.matches("data-src=\"").count(), si.len());
    }

    #[test]
    fn renders_one_wedge_per_target_and_labels_every_source_row() {
        let (labels, si, ti, w) = synth();
        let html = render(&cfg(&labels, &si, &ti, &w));
        assert_eq!(html.matches("<circle data-idx=\"").count(), 10);
        let n_targets = ti.iter().cloned().collect::<std::collections::HashSet<_>>().len();
        assert_eq!(html.matches("stroke=\"#ffffff\" stroke-width=\"1.4\"").count(), n_targets);
    }

    #[test]
    fn every_ribbon_passes_through_the_pinch_column() {
        let (labels, si, ti, w) = synth();
        let html = render(&cfg(&labels, &si, &ti, &w));
        let pinch_x = 190.0 + (1250.0 - 190.0 - 90.0) * 0.40;
        let needle = format!("{:.2}", pinch_x);
        assert!(html.matches(needle.as_str()).count() >= si.len());
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let empty_s: Vec<String> = vec![];
        let empty_i: Vec<i32> = vec![];
        let empty_w: Vec<f64> = vec![];
        assert!(render(&cfg(&empty_s, &empty_i, &empty_i, &empty_w)).is_empty());
    }

    #[test]
    fn perf_rendering_a_dense_flow_stays_fast() {
        let mut labels: Vec<String> = (0..120).map(|i| format!("Food{i}")).collect();
        labels.extend((0..20).map(|i| format!("Nutrient{i}")));
        let mut si = Vec::new();
        let mut ti = Vec::new();
        let mut w = Vec::new();
        for s in 0..120 {
            for t in 0..3 {
                si.push(s);
                ti.push(120 + ((s + t * 7) % 20));
                w.push(5.0 + ((s * 11 + t * 13) % 90) as f64);
            }
        }
        let start = std::time::Instant::now();
        let html = render(&cfg(&labels, &si, &ti, &w));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 500, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
