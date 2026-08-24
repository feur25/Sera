use std::collections::HashMap;

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub r: f64,
    pub depth: usize,
    pub color_idx: usize,
    pub label: String,
    pub children: Vec<usize>,
}

pub fn build_circles(
    labels: &[String],
    parents: &[String],
    values: &[f64],
    cx: f64,
    cy: f64,
    r_max: f64,
    padding: f64,
) -> Vec<Circle> {
    let n = labels.len().min(parents.len()).min(values.len());
    if n == 0 {
        return Vec::new();
    }

    let mut label_idx: HashMap<&str, usize> = HashMap::with_capacity(n);
    for i in 0..n {
        label_idx.insert(labels[i].as_str(), i);
    }

    let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut roots = Vec::new();
    for i in 0..n {
        let par = parents[i].as_str();
        if par.is_empty() {
            roots.push(i);
        } else if let Some(&pi) = label_idx.get(par) {
            children[pi].push(i);
        }
    }

    let mut sv: Vec<f64> = values[..n].to_vec();
    let mut stack: Vec<(usize, bool)> = roots.iter().map(|&r| (r, false)).collect();
    while let Some((i, visited)) = stack.pop() {
        if visited {
            let cs: f64 = children[i].iter().map(|&c| sv[c]).sum();
            if cs > sv[i] {
                sv[i] = cs;
            }
        } else {
            stack.push((i, true));
            for &c in children[i].iter().rev() {
                stack.push((c, false));
            }
        }
    }
    let total: f64 = roots.iter().map(|&r| sv[r]).sum();

    let mut depth = vec![0usize; n];
    let mut color_idx = vec![0usize; n];
    let mut q: std::collections::VecDeque<usize> = roots.iter().copied().collect();
    let mut ci = 0usize;
    while let Some(i) = q.pop_front() {
        for &c in children[i].iter() {
            depth[c] = depth[i] + 1;
            color_idx[c] = ci;
            q.push_back(c);
        }
        if depth[i] == 0 {
            color_idx[i] = ci;
            ci += 1;
        }
    }

    let mut circles: Vec<Circle> = (0..n)
        .map(|i| Circle {
            x: 0.0,
            y: 0.0,
            r: (sv[i] / total.max(1.0)).sqrt() * r_max,
            depth: depth[i],
            color_idx: color_idx[i],
            label: labels[i].clone(),
            children: children[i].clone(),
        })
        .collect();

    fn pack_children(
        circles: &mut Vec<Circle>,
        weights: &[f64],
        node_children: &[usize],
        px: f64,
        py: f64,
        pr: f64,
        padding: f64,
    ) {
        let n = node_children.len();
        if n == 0 {
            return;
        }
        let avail = (pr - padding * 2.0).max(1.0);
        if n == 1 {
            let c = node_children[0];
            circles[c].x = px;
            circles[c].y = py;
            circles[c].r = avail * 0.88;
            return;
        }
        let angles: Vec<f64> = (0..n)
            .map(|i| 2.0 * std::f64::consts::PI * i as f64 / n as f64 - std::f64::consts::FRAC_PI_2)
            .collect();
        let base_r = avail / (1.0 + 1.0 / (std::f64::consts::PI / n as f64).sin()).max(2.0);
        let max_w = node_children
            .iter()
            .map(|&c| weights[c].max(0.0).sqrt())
            .fold(0.0f64, f64::max)
            .max(1.0);
        for (k, &c) in node_children.iter().enumerate() {
            let a = angles[k];
            let ratio = (weights[c].max(0.0).sqrt() / max_w).clamp(0.42, 1.0);
            let child_r = (base_r * ratio * 0.9).max(2.0);
            let orbit_r = (avail - child_r - padding).max(0.0);
            circles[c].x = px + orbit_r * a.cos();
            circles[c].y = py + orbit_r * a.sin();
            circles[c].r = child_r;
        }
    }

    let max_root = roots
        .iter()
        .map(|&i| sv[i].max(0.0).sqrt())
        .fold(0.0f64, f64::max)
        .max(1.0);

    for (ri, &root) in roots.iter().enumerate() {
        let angle = 2.0 * std::f64::consts::PI * ri as f64 / roots.len() as f64
            - std::f64::consts::FRAC_PI_2;
        let r = if roots.len() > 1 {
            r_max * 0.34 * (sv[root].max(0.0).sqrt() / max_root).clamp(0.55, 1.0)
        } else {
            r_max * 0.92
        };
        let orbit = if roots.len() > 1 {
            (r_max - r - padding).max(0.0) * 0.82
        } else {
            0.0
        };
        circles[root].x = cx + orbit * angle.cos();
        circles[root].y = cy + orbit * angle.sin();
        circles[root].r = r;
    }

    fn pack_node(
        circles: &mut Vec<Circle>,
        children: &[Vec<usize>],
        weights: &[f64],
        idx: usize,
        padding: f64,
    ) {
        let node_children = children[idx].clone();
        if node_children.is_empty() {
            return;
        }
        let (px, py, pr) = (circles[idx].x, circles[idx].y, circles[idx].r);
        pack_children(circles, weights, &node_children, px, py, pr, padding);
        for child in node_children {
            pack_node(circles, children, weights, child, padding);
        }
    }

    for &root in &roots {
        pack_node(&mut circles, &children, &sv, root, padding);
    }

    circles
}

fn spiral_fallback(placed: &[(f64, f64)], placed_r: &[f64], ri: f64, padding: f64) -> (f64, f64) {
    let mut angle = 0.0f64;
    let mut radius = ri + 5.0;
    for _ in 0..2500 {
        let px = radius * angle.cos();
        let py = radius * angle.sin();
        let ok = placed.iter().zip(placed_r.iter()).all(|(&(jx, jy), &jr)| {
            ((jx - px).powi(2) + (jy - py).powi(2)).sqrt() >= jr + ri + padding
        });
        if ok {
            return (px, py);
        }
        angle += 0.31;
        radius += 0.6;
    }
    (radius, 0.0)
}

pub fn pack_local(radii: &[f64], padding: f64) -> Vec<(f64, f64)> {
    let n = radii.len();
    let mut pos = vec![(0.0, 0.0); n];
    if n <= 1 {
        return pos;
    }
    pos[1] = (radii[0] + radii[1] + padding, 0.0);
    if n == 2 {
        return pos;
    }
    for i in 2..n {
        let ri = radii[i];
        let cand_start = if i > 64 { i - 64 } else { 0 };
        let check_start = if i > 180 { i - 180 } else { 0 };
        let mut best: Option<(f64, f64)> = None;
        let mut best_dist = f64::MAX;
        for a in cand_start..i {
            for b in (a + 1)..i {
                let (ax, ay) = pos[a];
                let (bx, by) = pos[b];
                let ra = radii[a] + ri + padding;
                let rb = radii[b] + ri + padding;
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
                    for j in check_start..i {
                        if j == a || j == b {
                            continue;
                        }
                        let (jx, jy) = pos[j];
                        let dd = ((jx - px).powi(2) + (jy - py).powi(2)).sqrt();
                        if dd < radii[j] + ri + padding - 1e-6 {
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
        pos[i] = best.unwrap_or_else(|| {
            spiral_fallback(&pos[check_start..i], &radii[check_start..i], ri, padding)
        });
    }
    pos
}
