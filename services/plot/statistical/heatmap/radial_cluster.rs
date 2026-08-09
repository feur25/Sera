use super::common::{cell_color, finite_minmax, hierarchical_dendrogram, DendroMerge};
use super::config::HeatmapConfig;
use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i};
use std::f64::consts::PI;

#[crate::chart_demo("labels=[\"GSM1\",\"GSM2\",\"GSM3\",\"GSM4\",\"GSM5\",\"GSM6\"], col_labels=[\"10001_at\",\"10005_at\",\"10013_at\",\"10020_at\",\"10025_at\",\"10004_at\",\"10007_at\",\"10014_at\",\"10023_at\",\"10008_at\",\"10019_at\",\"10038_at\"], values=[9,9,10,8,10,7,4,3,5,2,4,3,10,9,11,9,11,8,5,4,6,3,5,4,8,8,9,7,9,6,3,2,4,1,3,2,11,10,12,10,12,9,6,5,7,4,6,5,7,7,8,6,8,5,2,1,3,0,2,1,12,11,13,11,13,10,7,6,8,5,7,6]")]

pub fn render(cfg: &HeatmapConfig) -> String {
    let nr = cfg.row_labels.len();
    let cols_lbl = if cfg.col_labels.is_empty() {
        cfg.row_labels
    } else {
        cfg.col_labels
    };
    let nc = cols_lbl.len();
    if nr == 0 || nc < 2 || cfg.flat_matrix.len() < nr * nc {
        return String::new();
    }

    let col_vectors: Vec<Vec<f64>> = (0..nc)
        .map(|c| {
            (0..nr)
                .map(|r| {
                    let v = cfg.flat_matrix[r * nc + c];
                    if v.is_finite() {
                        v
                    } else {
                        0.0
                    }
                })
                .collect()
        })
        .collect();
    let dendro = hierarchical_dendrogram(&col_vectors);
    let order = &dendro.order;
    let new_cols: Vec<String> = order.iter().map(|&i| cols_lbl[i].clone()).collect();
    let mut new_mat = vec![0.0f64; nr * nc];
    for (nci, &orig_c) in order.iter().enumerate() {
        for r in 0..nr {
            new_mat[r * nc + nci] = cfg.flat_matrix[r * nc + orig_c];
        }
    }

    let w = cfg.width;
    let h = cfg.height;
    let side = w.min(h) as f64;
    let cx = w as f64 / 2.0;
    let cy = h as f64 / 2.0 + 10.0;
    let inner_r = side * 0.145;
    let outer_r = side * 0.385;
    let ring_h = (outer_r - inner_r) / nr as f64;
    let gap_rad = PI / 180.0 * 0.35;

    let (v_min, v_max) = finite_minmax(&new_mat);
    let norm_v = |v: f64| -> f64 {
        if v_max > v_min {
            ((v - v_min) / (v_max - v_min)).clamp(0.0, 1.0)
        } else {
            0.5
        }
    };

    let n_total = nr * nc;
    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n_total);
    let mut buf = Vec::<u8>::with_capacity(n_total * 220 + 16384);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut buf, w);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, h);
    push_b(&mut buf, b"\">");
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    let col_lo = hex6(cfg.color_low);
    let col_hi = hex6(cfg.color_high);
    push_b(&mut buf, b"<defs><linearGradient id=\"rc-cscale\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">");
    push_b(&mut buf, b"<stop offset=\"0\" stop-color=\"#");
    buf.extend_from_slice(&col_lo);
    push_b(&mut buf, b"\"/><stop offset=\"1\" stop-color=\"#");
    buf.extend_from_slice(&col_hi);
    push_b(&mut buf, b"\"/></linearGradient></defs>");

    for ri in 1..nr {
        let r = inner_r + ri as f64 * ring_h;
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, cy);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, r);
        push_b(&mut buf, b"\" fill=\"none\" stroke=\"#0f172a\" stroke-width=\"1\"/>");
    }

    for ri in 0..nr {
        let r0 = inner_r + ri as f64 * ring_h + 0.8;
        let r1 = inner_r + (ri + 1) as f64 * ring_h - 0.8;
        for ci in 0..nc {
            let val = new_mat[ri * nc + ci];
            let t = norm_v(val);
            let col = cell_color(t, cfg);
            let hx = hex6(col);

            let a0 = -PI * 0.5 + ci as f64 * 2.0 * PI / nc as f64 + gap_rad;
            let a1 = -PI * 0.5 + (ci + 1) as f64 * 2.0 * PI / nc as f64 - gap_rad;

            let x00 = cx + r0 * a0.cos();
            let y00 = cy + r0 * a0.sin();
            let x01 = cx + r0 * a1.cos();
            let y01 = cy + r0 * a1.sin();
            let x10 = cx + r1 * a1.cos();
            let y10 = cy + r1 * a1.sin();
            let x11 = cx + r1 * a0.cos();
            let y11 = cy + r1 * a0.sin();

            push_b(&mut buf, b"<path data-idx=\"");
            push_i(&mut buf, (ri * nc + ci) as i32);
            push_b(&mut buf, b"\" d=\"M ");
            push_f2(&mut buf, x00);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, y00);
            push_b(&mut buf, b" A ");
            push_f2(&mut buf, r0);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, r0);
            push_b(&mut buf, b" 0 0 1 ");
            push_f2(&mut buf, x01);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, y01);
            push_b(&mut buf, b" L ");
            push_f2(&mut buf, x10);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, y10);
            push_b(&mut buf, b" A ");
            push_f2(&mut buf, r1);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, r1);
            push_b(&mut buf, b" 0 0 0 ");
            push_f2(&mut buf, x11);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, y11);
            push_b(&mut buf, b" Z\" fill=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\"/>");

            let row_label = cfg.row_labels.get(ri).map(|s| s.as_str()).unwrap_or("?");
            let col_label = new_cols.get(ci).map(|s| s.as_str()).unwrap_or("");
            slots.push(
                HoverSlot::new(format!("{} · {}", row_label, col_label))
                    .kv("Valeur", format!("{val:.2}"))
                    .kv("Ligne", row_label.to_string())
                    .kv("Colonne (regroupee)", col_label.to_string()),
            );
        }
    }

    draw_radial_dendrogram(&mut buf, &dendro.merges, dendro.max_height, nc, cx, cy, inner_r);

    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" cy=\"");
    push_f2(&mut buf, cy);
    push_b(&mut buf, b"\" r=\"");
    push_f2(&mut buf, outer_r);
    push_b(&mut buf, b"\" fill=\"none\" stroke=\"#1e293b\" stroke-width=\"1\"/>");

    for ci in 0..nc {
        let a_mid = -PI * 0.5 + (ci as f64 + 0.5) * 2.0 * PI / nc as f64;
        let tx0 = cx + outer_r * a_mid.cos();
        let ty0 = cy + outer_r * a_mid.sin();
        let tx1 = cx + (outer_r + 6.0) * a_mid.cos();
        let ty1 = cy + (outer_r + 6.0) * a_mid.sin();
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, tx0);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, ty0);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, tx1);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, ty1);
        push_b(&mut buf, b"\" stroke=\"#1e293b\" stroke-width=\"0.6\"/>");

        let label = new_cols.get(ci).map(|s| s.as_str()).unwrap_or("");
        let lr = outer_r + 9.0;
        let deg = a_mid.to_degrees() + if a_mid.cos() < 0.0 { 180.0 } else { 0.0 };
        let lx = cx + lr * a_mid.cos();
        let ly = cy + lr * a_mid.sin();
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, lx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b"\" transform=\"rotate(");
        push_f2(&mut buf, deg);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, lx);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b")\" text-anchor=\"");
        push_b(&mut buf, if a_mid.cos() < 0.0 { b"end" } else { b"start" });
        push_b(&mut buf, b"\" dominant-baseline=\"middle\" font-family=\"system-ui,sans-serif\" font-size=\"7\" fill=\"#334155\">");
        escape_xml(&mut buf, label);
        push_b(&mut buf, b"</text>");
    }

    let bar_y = (cy + outer_r + 30.0) as i32;
    let bar_w = 200i32;
    let bar_x = (w - bar_w) / 2;
    push_b(&mut buf, b"<rect x=\"");
    push_i(&mut buf, bar_x);
    push_b(&mut buf, b"\" y=\"");
    push_i(&mut buf, bar_y);
    push_b(&mut buf, b"\" width=\"");
    push_i(&mut buf, bar_w);
    push_b(&mut buf, b"\" height=\"5\" rx=\"2\" fill=\"url(#rc-cscale)\"/>");
    push_b(&mut buf, b"<text x=\"");
    push_i(&mut buf, bar_x);
    push_b(&mut buf, b"\" y=\"");
    push_i(&mut buf, bar_y + 14);
    push_b(&mut buf, b"\" text-anchor=\"start\" font-family=\"system-ui,sans-serif\" font-size=\"6.5\" fill=\"#334155\">");
    escape_xml(&mut buf, &format!("{v_min:.0}"));
    push_b(&mut buf, b"</text>");
    push_b(&mut buf, b"<text x=\"");
    push_i(&mut buf, bar_x + bar_w);
    push_b(&mut buf, b"\" y=\"");
    push_i(&mut buf, bar_y + 14);
    push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"system-ui,sans-serif\" font-size=\"6.5\" fill=\"#334155\">");
    escape_xml(&mut buf, &format!("{v_max:.0}"));
    push_b(&mut buf, b"</text>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"20\" text-anchor=\"middle\" font-family=\"system-ui,sans-serif\" \
          font-size=\"9.5\" font-weight=\"700\" fill=\"#1a2744\" letter-spacing=\"3\" class=\"sp-ttl\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"</svg>");

    let svg_str = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg_str, &slots_to_json(&slots))
}

fn draw_radial_dendrogram(buf: &mut Vec<u8>, merges: &[DendroMerge], max_height: f64, nc: usize, cx: f64, cy: f64, inner_r: f64) {
    if merges.is_empty() {
        return;
    }
    let root_r = inner_r * 0.1;
    let leaf_r = inner_r * 0.92;
    let mh = max_height.max(1e-9);
    let r_of = |height: f64| leaf_r - (height / mh) * (leaf_r - root_r);
    let angle_of = |pos: f64| -PI * 0.5 + (pos + 0.5) * 2.0 * PI / nc as f64;

    for m in merges {
        let al = angle_of(m.xl);
        let ar = angle_of(m.xr);
        let rl = if m.hl <= 1e-9 { leaf_r } else { r_of(m.hl) };
        let rr = if m.hr <= 1e-9 { leaf_r } else { r_of(m.hr) };
        let rm = r_of(m.h);

        radial_line(buf, cx, cy, al, rl, rm);
        radial_line(buf, cx, cy, ar, rr, rm);
        radial_arc(buf, cx, cy, rm, al, ar);
    }
}

fn radial_line(buf: &mut Vec<u8>, cx: f64, cy: f64, angle: f64, r1: f64, r2: f64) {
    let x1 = cx + r1 * angle.cos();
    let y1 = cy + r1 * angle.sin();
    let x2 = cx + r2 * angle.cos();
    let y2 = cy + r2 * angle.sin();
    push_b(buf, b"<line x1=\"");
    push_f2(buf, x1);
    push_b(buf, b"\" y1=\"");
    push_f2(buf, y1);
    push_b(buf, b"\" x2=\"");
    push_f2(buf, x2);
    push_b(buf, b"\" y2=\"");
    push_f2(buf, y2);
    push_b(buf, b"\" stroke=\"#64748b\" stroke-width=\"1\" fill=\"none\"/>");
}

fn radial_arc(buf: &mut Vec<u8>, cx: f64, cy: f64, r: f64, a0: f64, a1: f64) {
    let (a0, a1) = if a0 <= a1 { (a0, a1) } else { (a1, a0) };
    let x0 = cx + r * a0.cos();
    let y0 = cy + r * a0.sin();
    let x1 = cx + r * a1.cos();
    let y1 = cy + r * a1.sin();
    let large_arc = if (a1 - a0) > PI { 1 } else { 0 };
    push_b(buf, b"<path d=\"M ");
    push_f2(buf, x0);
    push_b(buf, b" ");
    push_f2(buf, y0);
    push_b(buf, b" A ");
    push_f2(buf, r);
    push_b(buf, b" ");
    push_f2(buf, r);
    push_b(buf, b" 0 ");
    buf.extend_from_slice(if large_arc == 1 { b"1" } else { b"0" });
    push_b(buf, b" 1 ");
    push_f2(buf, x1);
    push_b(buf, b" ");
    push_f2(buf, y1);
    push_b(buf, b"\" stroke=\"#64748b\" stroke-width=\"1\" fill=\"none\"/>");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg<'a>(row_labels: &'a [String], col_labels: &'a [String], flat_matrix: &'a [f64]) -> HeatmapConfig<'a> {
        HeatmapConfig {
            title: "Test",
            row_labels,
            col_labels,
            flat_matrix,
            width: 720,
            height: 720,
            color_low: 0x3B82F6,
            color_high: 0xEF4444,
            ..HeatmapConfig::default()
        }
    }

    #[test]
    fn renders_one_wedge_per_column_and_one_ring_per_row() {
        let rows = vec!["r1".to_string(), "r2".to_string(), "r3".to_string()];
        let cols = vec!["c1".to_string(), "c2".to_string(), "c3".to_string(), "c4".to_string()];
        let values = vec![1.0, 2.0, 3.0, 4.0, 2.0, 3.0, 4.0, 5.0, 3.0, 4.0, 5.0, 6.0];
        let html = render(&cfg(&rows, &cols, &values));
        assert_eq!(html.matches("<path data-idx=").count(), 12);
    }

    #[test]
    fn embeds_a_radial_dendrogram_with_merge_arcs() {
        let rows = vec!["r1".to_string(), "r2".to_string()];
        let cols = vec!["c1".to_string(), "c2".to_string(), "c3".to_string()];
        let values = vec![1.0, 5.0, 1.2, 2.0, 5.1, 2.1];
        let html = render(&cfg(&rows, &cols, &values));
        assert!(html.matches(" A ").count() >= cols.len(), "expected at least one arc per leaf merged");
    }

    #[test]
    fn columns_are_reordered_by_hierarchical_clustering_so_similar_values_land_adjacently() {
        let rows = vec!["r1".to_string()];
        let cols = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let values = vec![0.0, 100.0, 1.0];
        let html = render(&cfg(&rows, &cols, &values));
        let pos_a = html.find(">a<").unwrap();
        let pos_c = html.find(">c<").unwrap();
        let pos_b = html.find(">b<").unwrap();
        let diff_ac = (pos_a as i64 - pos_c as i64).abs();
        let diff_ab = (pos_a as i64 - pos_b as i64).abs();
        assert!(diff_ac < diff_ab, "a (0.0) and c (1.0) are far closer to each other than either is to b (100.0), so clustering must place a next to c, not b");
    }

    #[test]
    fn missing_col_labels_falls_back_to_a_square_matrix_using_row_labels_like_its_sibling_cluster_variant() {
        let rows = vec!["r1".to_string(), "r2".to_string(), "r3".to_string()];
        let no_cols: Vec<String> = vec![];
        let values = vec![1.0, 2.0, 3.0, 2.0, 3.0, 4.0, 3.0, 4.0, 5.0];
        let html = render(&cfg(&rows, &no_cols, &values));
        assert!(!html.is_empty(), "labels+values with no col_labels must still render (title, labels, values, variant is the whole payload the playground sends)");
        assert_eq!(html.matches("<path data-idx=").count(), 9);
    }

    #[test]
    fn empty_or_single_column_input_returns_empty_string_instead_of_a_broken_chart() {
        let rows = vec!["r1".to_string()];
        let cols: Vec<String> = vec![];
        let values: Vec<f64> = vec![];
        assert!(render(&cfg(&rows, &cols, &values)).is_empty());

        let one_col = vec!["only".to_string()];
        let one_val = vec![1.0];
        assert!(render(&cfg(&rows, &one_col, &one_val)).is_empty(), "a single wedge has nothing to cluster against");
    }

    #[test]
    fn perf_rendering_a_realistic_circular_dendrogram_heatmap_stays_fast() {
        let rows: Vec<String> = (0..10).map(|i| format!("row{i}")).collect();
        let cols: Vec<String> = (0..40).map(|i| format!("sample{i}")).collect();
        let values: Vec<f64> = (0..(10 * 40)).map(|i| ((i * 37) % 97) as f64).collect();
        let start = std::time::Instant::now();
        let html = render(&cfg(&rows, &cols, &values));
        let elapsed = start.elapsed();
        println!("radial_cluster perf: 10x40 cells in {elapsed:?}, {} bytes", html.len());
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
