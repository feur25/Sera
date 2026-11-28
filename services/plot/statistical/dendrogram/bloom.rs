use super::common::{node_color, svg_header, tree_for, TreeNode};
use super::config::DendrogramConfig;
use crate::html::hover::{html_id, html_suffix, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i};
use std::f64::consts::PI;

fn place_subtree(nodes: &mut [TreeNode], i: usize, cx: f64, cy: f64, r: f64) {
    let kids = nodes[i].children.clone();
    let m = kids.len();
    if m == 0 {
        return;
    }
    for (k, &c) in kids.iter().enumerate() {
        let a = 2.0 * PI * k as f64 / m as f64 - PI / 2.0;
        let x = cx + r * a.cos();
        let y = cy + r * a.sin();
        nodes[c].x = x;
        nodes[c].y = y;
        place_subtree(nodes, c, x, y, r * 0.55);
    }
}

#[crate::chart_demo(
    "labels=[\"Felicidad\",\"Serenidad\",\"Optimismo\",\"Gratitud\",\"Diversion\",\"Orgullo\",\"Plenitud\",\"Confianza\",\"Amor\",\"Cariño\",\"Ternura\",\"Pasion\",\"Devocion\",\"Compasion\",\"Intimidad\",\"Alegria\",\"Euforia\",\"Entusiasmo\",\"Contento\",\"Jubilo\",\"Deleite\",\"Satisfaccion\",\"Sorpresa\",\"Asombro\",\"Sobresalto\",\"Desconcierto\",\"Confusion\",\"Ira\",\"Furia\",\"Resentimiento\",\"Indignacion\",\"Irritacion\",\"Hostilidad\",\"Celos\",\"Tristeza\",\"Melancolia\",\"Soledad\",\"Desanimo\",\"Decepcion\",\"Nostalgia\",\"Pena\",\"Miedo\",\"Panico\",\"Terror\",\"Inseguridad\",\"Alarma\",\"Horror\",\"Asco\",\"Rechazo\",\"Desprecio\",\"Repulsion\",\"Aversion\",\"Ansiedad\",\"Angustia\",\"Estres\",\"Nerviosismo\",\"Preocupacion\",\"Tension\"], parents=[\"\",\"Felicidad\",\"Felicidad\",\"Felicidad\",\"Felicidad\",\"Felicidad\",\"Felicidad\",\"Felicidad\",\"\",\"Amor\",\"Amor\",\"Amor\",\"Amor\",\"Amor\",\"Amor\",\"\",\"Alegria\",\"Alegria\",\"Alegria\",\"Alegria\",\"Alegria\",\"Alegria\",\"\",\"Sorpresa\",\"Sorpresa\",\"Sorpresa\",\"Sorpresa\",\"\",\"Ira\",\"Ira\",\"Ira\",\"Ira\",\"Ira\",\"Ira\",\"\",\"Tristeza\",\"Tristeza\",\"Tristeza\",\"Tristeza\",\"Tristeza\",\"Tristeza\",\"\",\"Miedo\",\"Miedo\",\"Miedo\",\"Miedo\",\"Miedo\",\"\",\"Asco\",\"Asco\",\"Asco\",\"Asco\",\"\",\"Ansiedad\",\"Ansiedad\",\"Ansiedad\",\"Ansiedad\",\"Ansiedad\"], show_values=True, variant=\"bloom\", width=620, height=390"
)]

pub fn render(cfg: &DendrogramConfig) -> String {
    let Some((mut nodes, roots)) = tree_for(cfg) else { return String::new(); };
    let n = nodes.len();
    if roots.is_empty() {
        return String::new();
    }

    let mut subtree_size = vec![1usize; n];
    let mut by_depth_desc: Vec<usize> = (0..n).collect();
    by_depth_desc.sort_by(|&a, &b| nodes[b].depth.cmp(&nodes[a].depth));
    for &i in &by_depth_desc {
        if let Some(pi) = nodes[i].parent {
            subtree_size[pi] += subtree_size[i];
        }
    }

    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let cx = w / 2.0;
    let cy = h / 2.0 + 8.0;
    let pack_r = (w.min(h) / 2.0 - 70.0).max(40.0);

    let n_roots = roots.len();
    let max_size = roots.iter().map(|&r| subtree_size[r]).max().unwrap_or(1).max(1) as f64;
    let golden_angle = PI * (3.0 - 5.0_f64.sqrt());

    let mut blob_r = vec![0.0_f64; n];
    for (k, &r) in roots.iter().enumerate() {
        let frac = if n_roots > 1 { k as f64 / (n_roots - 1) as f64 } else { 0.0 };
        let radius = pack_r * frac.sqrt();
        let angle = k as f64 * golden_angle;
        let hx = cx + radius * angle.cos();
        let hy = cy + radius * angle.sin();
        nodes[r].x = hx;
        nodes[r].y = hy;
        let sz = subtree_size[r] as f64;
        blob_r[r] = 26.0 + (sz / max_size).sqrt() * 78.0;
        place_subtree(&mut nodes, r, hx, hy, blob_r[r] * 0.72);
    }

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n * 260 + 8192);
    svg_header(&mut buf, cfg, hid, cx);

    for &r in &roots {
        let hx = hex6(node_color(cfg, &nodes[r]));
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, nodes[r].x);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, nodes[r].y);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, blob_r[r]);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.16\"/>");
    }

    for i in 0..n {
        if let Some(pi) = nodes[i].parent {
            let hx = hex6(node_color(cfg, &nodes[i]));
            push_b(&mut buf, b"<line x1=\"");
            push_f2(&mut buf, nodes[pi].x);
            push_b(&mut buf, b"\" y1=\"");
            push_f2(&mut buf, nodes[pi].y);
            push_b(&mut buf, b"\" x2=\"");
            push_f2(&mut buf, nodes[i].x);
            push_b(&mut buf, b"\" y2=\"");
            push_f2(&mut buf, nodes[i].y);
            push_b(&mut buf, b"\" stroke=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stroke-width=\"");
            push_f2(&mut buf, cfg.line_width);
            push_b(&mut buf, b"\" stroke-opacity=\"0.55\"/>");
        }
    }

    for i in 0..n {
        let is_root = nodes[i].parent.is_none();
        let hx = hex6(node_color(cfg, &nodes[i]));
        let r = if is_root { 10.0 + (subtree_size[i] as f64).sqrt() * 0.9 } else { 3.4 };
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, nodes[i].x);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, nodes[i].y);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, r);
        push_b(&mut buf, b"\" fill=\"#fff\" stroke=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke-width=\"1.6\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\"/>");

        if is_root {
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, nodes[i].x);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, nodes[i].y + 3.0);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" font-weight=\"700\" fill=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\">");
            escape_xml(&mut buf, &nodes[i].label);
            push_b(&mut buf, b"</text>");
        } else if cfg.show_labels && nodes[i].children.is_empty() {
            let dx = nodes[i].x - nodes[nodes[i].parent.unwrap()].x;
            let dy = nodes[i].y - nodes[nodes[i].parent.unwrap()].y;
            let len = (dx * dx + dy * dy).sqrt().max(1.0);
            let tx = nodes[i].x + dx / len * 8.0;
            let ty = nodes[i].y + dy / len * 8.0;
            let anchor: &[u8] = if dx > 1.0 { b"start" } else if dx < -1.0 { b"end" } else { b"middle" };
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, tx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, ty + 3.0);
            push_b(&mut buf, b"\" text-anchor=\"");
            buf.extend_from_slice(anchor);
            push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"6.5\" fill=\"#475569\">");
            escape_xml(&mut buf, &nodes[i].label);
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cluster_set(n_cats: usize, per_cat: usize) -> (Vec<String>, Vec<String>) {
        let mut labels = Vec::new();
        let mut parents = Vec::new();
        for c in 0..n_cats {
            let cat = format!("Cat{c}");
            labels.push(cat.clone());
            parents.push(String::new());
            for i in 0..per_cat {
                labels.push(format!("Cat{c}Item{i}"));
                parents.push(cat.clone());
            }
        }
        (labels, parents)
    }

    fn cfg<'a>(labels: &'a [String], parents: &'a [String]) -> DendrogramConfig<'a> {
        DendrogramConfig {
            title: "Test",
            labels,
            parents,
            width: 700,
            height: 700,
            ..DendrogramConfig::default()
        }
    }

    #[test]
    fn renders_one_hub_circle_and_leaf_dot_per_node() {
        let (labels, parents) = cluster_set(5, 6);
        let html = render(&cfg(&labels, &parents));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<circle").count() - 5, labels.len());
        assert!(html.contains("class=\"sp-bg\""));
    }

    #[test]
    fn each_cluster_gets_a_translucent_blob_and_a_named_hub() {
        let (labels, parents) = cluster_set(3, 4);
        let html = render(&cfg(&labels, &parents));
        assert_eq!(html.matches("fill-opacity=\"0.16\"").count(), 3);
        for c in 0..3 {
            assert!(html.contains(&format!(">Cat{c}<")));
        }
    }

    #[test]
    fn a_denser_cluster_gets_a_bigger_blob_than_a_sparse_one() {
        let mut labels = vec!["Big".to_string(), "Small".to_string()];
        let mut parents = vec![String::new(), String::new()];
        for i in 0..20 {
            labels.push(format!("BigItem{i}"));
            parents.push("Big".to_string());
        }
        labels.push("SmallItem0".to_string());
        parents.push("Small".to_string());
        let html = render(&cfg(&labels, &parents));
        let mut radii = Vec::new();
        for tag in html.split("fill-opacity=\"0.16\"").skip(1) {
            let before = &html[..html.find(tag).unwrap()];
            let start = before.rfind("<circle").unwrap();
            let seg = &before[start..];
            let r_key = "r=\"";
            let r_start = seg.rfind(r_key).unwrap() + r_key.len();
            let r: f64 = seg[r_start..].split('"').next().unwrap().parse().unwrap();
            radii.push(r);
        }
        assert_eq!(radii.len(), 2);
        assert!(radii[0] > radii[1], "expected the 20-item cluster's blob to be bigger: {radii:?}");
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let labels: Vec<String> = vec![];
        let parents: Vec<String> = vec![];
        assert!(render(&cfg(&labels, &parents)).is_empty());
    }

    #[test]
    fn perf_rendering_a_large_multi_cluster_bloom_stays_fast() {
        let (labels, parents) = cluster_set(12, 40);
        let c = cfg(&labels, &parents);
        let start = std::time::Instant::now();
        let html = render(&c);
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
