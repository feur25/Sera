use super::config::CirclePackConfig;
use crate::html::hover::{html_id, html_prefix, html_suffix, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i};

#[crate::chart_demo("labels=[\"Alpha\",\"Beta\",\"Gamma\",\"Delta\",\"Epsilon\",\"Zeta\",\"Eta\"], values=[40,30,25,20,15,12,10]")]
pub fn render(cfg: &CirclePackConfig) -> String {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 {
        return String::new();
    }

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| {
        cfg.values[b]
            .partial_cmp(&cfg.values[a])
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let total: f64 = cfg.values[..n].iter().sum::<f64>().max(1.0);
    let unit_r = (cfg.width.min(cfg.height) as f64) * 0.30;
    let radii_by_order: Vec<f64> = order
        .iter()
        .map(|&i| (cfg.values[i] / total).sqrt() * unit_r)
        .collect();

    let local_pos = pack_positions(&radii_by_order);

    let mut min_x = f64::MAX;
    let mut max_x = f64::MIN;
    let mut min_y = f64::MAX;
    let mut max_y = f64::MIN;
    for (k, &(px, py)) in local_pos.iter().enumerate() {
        let r = radii_by_order[k];
        min_x = min_x.min(px - r);
        max_x = max_x.max(px + r);
        min_y = min_y.min(py - r);
        max_y = max_y.max(py + r);
    }
    let pack_w = (max_x - min_x).max(1.0);
    let pack_h = (max_y - min_y).max(1.0);
    let pad = 16.0;
    let fit_scale = ((cfg.width as f64 - pad * 2.0) / pack_w)
        .min((cfg.height as f64 - pad * 2.0) / pack_h)
        .min(1.0);
    let cx = cfg.width as f64 / 2.0;
    let cy = cfg.height as f64 / 2.0;
    let mid_x = (min_x + max_x) / 2.0;
    let mid_y = (min_y + max_y) / 2.0;

    let mut positions = vec![(0.0, 0.0); n];
    let mut radii = vec![0.0; n];
    for (k, &orig_idx) in order.iter().enumerate() {
        let (lx, ly) = local_pos[k];
        positions[orig_idx] = (
            cx + (lx - mid_x) * fit_scale,
            cy + (ly - mid_y) * fit_scale,
        );
        radii[orig_idx] = radii_by_order[k] * fit_scale;
    }

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n * 200 + 4096);
    html_prefix(&mut buf, cfg.title, hid);
    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"18\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"14\" font-weight=\"700\" fill=\"#1a202c\" class=\"sp-ttl\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    for &i in &order {
        let color = palette_color(cfg.palette, i);
        let hx = hex6(color);
        let (px, py) = positions[i];
        let r = radii[i].max(3.0);
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, px);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, py);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, r);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.82\" stroke=\"#fff\" stroke-width=\"1.5\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\"/>");

        if cfg.show_labels && r > 11.0 {
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, px);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, py + 4.0);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" fill=\"#fff\" font-size=\"");
            push_f2(&mut buf, (r * 0.3).min(13.0).max(8.0));
            push_b(&mut buf, b"\">");
            escape_xml(&mut buf, &cfg.labels[i]);
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}

fn pack_positions(radii: &[f64]) -> Vec<(f64, f64)> {
    let n = radii.len();
    let mut pos = vec![(0.0, 0.0); n];
    if n <= 1 {
        return pos;
    }
    pos[1] = (radii[0] + radii[1], 0.0);
    if n == 2 {
        return pos;
    }

    for i in 2..n {
        let ri = radii[i];
        let mut best: Option<(f64, f64)> = None;
        let mut best_dist = f64::MAX;

        for a in 0..i {
            for b in (a + 1)..i {
                let (ax, ay) = pos[a];
                let (bx, by) = pos[b];
                let ra = radii[a] + ri;
                let rb = radii[b] + ri;
                let dx = bx - ax;
                let dy = by - ay;
                let d = (dx * dx + dy * dy).sqrt();
                if d < 1e-9 || d > ra + rb || d < (ra - rb).abs() {
                    continue;
                }
                let aa = (ra * ra - rb * rb + d * d) / (2.0 * d);
                let h2 = ra * ra - aa * aa;
                if h2 < 0.0 {
                    continue;
                }
                let h = h2.sqrt();
                let mx = ax + aa * dx / d;
                let my = ay + aa * dy / d;
                let ox = -dy / d * h;
                let oy = dx / d * h;
                for &(px, py) in &[(mx + ox, my + oy), (mx - ox, my - oy)] {
                    let mut ok = true;
                    for (j, &(jx, jy)) in pos.iter().enumerate().take(i) {
                        if j == a || j == b {
                            continue;
                        }
                        let dd = ((jx - px).powi(2) + (jy - py).powi(2)).sqrt();
                        if dd < radii[j] + ri - 1e-6 {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        let dist = (px * px + py * py).sqrt();
                        if dist < best_dist {
                            best_dist = dist;
                            best = Some((px, py));
                        }
                    }
                }
            }
        }

        pos[i] = best.unwrap_or_else(|| spiral_fallback(&pos[..i], &radii[..i], ri));
    }

    pos
}

fn spiral_fallback(placed: &[(f64, f64)], placed_r: &[f64], ri: f64) -> (f64, f64) {
    let mut angle = 0.0f64;
    let mut radius = ri + 5.0;
    for _ in 0..4000 {
        let px = radius * angle.cos();
        let py = radius * angle.sin();
        let ok = placed.iter().zip(placed_r.iter()).all(|(&(jx, jy), &jr)| {
            ((jx - px).powi(2) + (jy - py).powi(2)).sqrt() >= jr + ri
        });
        if ok {
            return (px, py);
        }
        angle += 0.28;
        radius += 0.7;
    }
    (radius, 0.0)
}
