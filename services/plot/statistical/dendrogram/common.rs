use super::config::DendrogramConfig;
use crate::html::hover::{html_id, html_prefix, html_suffix, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i};
use std::collections::HashMap;

const TRUNK_COLOR: u32 = 0x64748B;

pub fn node_color(cfg: &DendrogramConfig, node: &TreeNode) -> u32 {
    if node.above_cut {
        TRUNK_COLOR
    } else {
        palette_color(cfg.palette, node.color_idx)
    }
}

pub fn tree_for(cfg: &DendrogramConfig) -> Option<(Vec<TreeNode>, Vec<usize>)> {
    let (nodes, roots) = if !cfg.values.is_empty() {
        build_tree_from_values(cfg.labels, cfg.values, cfg.clusters)
    } else {
        build_tree(cfg.labels, cfg.parents)
    };
    if nodes.is_empty() { None } else { Some((nodes, roots)) }
}

pub struct TreeNode {
    pub x:         f64,
    pub y:         f64,
    pub depth:     usize,
    pub height:    f64,
    pub color_idx: usize,
    pub above_cut: bool,
    pub label:     String,
    pub children:  Vec<usize>,
    pub parent:    Option<usize>,
}

pub fn build_tree(labels: &[String], parents: &[String]) -> (Vec<TreeNode>, Vec<usize>) {
    let n = labels.len().min(parents.len());
    if n == 0 { return (Vec::new(), Vec::new()); }

    let mut label_idx: HashMap<&str, usize> = HashMap::with_capacity(n);
    for i in 0..n {
        label_idx.insert(labels[i].as_str(), i);
    }

    let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut parent_of: Vec<Option<usize>> = vec![None; n];
    let mut roots = Vec::new();

    for i in 0..n {
        let par = parents[i].as_str();
        if par.is_empty() {
            roots.push(i);
        } else if let Some(&pi) = label_idx.get(par) {
            children[pi].push(i);
            parent_of[i] = Some(pi);
        } else {
            roots.push(i);
        }
    }

    let mut depth = vec![0usize; n];
    let mut color_idx = vec![0usize; n];
    let mut stack: Vec<(usize, usize)> = roots.iter().enumerate().map(|(ci, &r)| (r, ci)).collect();
    while let Some((i, cidx)) = stack.pop() {
        color_idx[i] = cidx;
        for &c in &children[i] {
            depth[c] = depth[i] + 1;
            stack.push((c, cidx));
        }
    }
    let max_depth = depth.iter().copied().max().unwrap_or(0).max(1) as f64;

    let nodes: Vec<TreeNode> = (0..n).map(|i| TreeNode {
        x: 0.0, y: 0.0,
        depth: depth[i],
        height: max_depth - depth[i] as f64,
        color_idx: color_idx[i],
        above_cut: false,
        label: labels[i].clone(),
        children: children[i].clone(),
        parent: parent_of[i],
    }).collect();

    (nodes, roots)
}

fn pairwise_distances(vectors: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = vectors.len();
    let mut dist = vec![vec![0.0f64; n]; n];
    for i in 0..n {
        for j in (i + 1)..n {
            let d = vectors[i]
                .iter()
                .zip(vectors[j].iter())
                .map(|(a, b)| (a - b) * (a - b))
                .sum::<f64>()
                .sqrt();
            dist[i][j] = d;
            dist[j][i] = d;
        }
    }
    dist
}

/// Real average-linkage agglomerative clustering. Returns a tree whose internal
/// nodes carry a genuine merge height (average pairwise distance between the two
/// merged clusters, monotonically increasing toward the root), matching what
/// hclust/scipy show -- not a synthetic depth counter.
pub fn build_tree_from_values(labels: &[String], vectors: &[Vec<f64>], clusters: usize) -> (Vec<TreeNode>, Vec<usize>) {
    let n = labels.len().min(vectors.len());
    if n == 0 { return (Vec::new(), Vec::new()); }
    if n == 1 {
        return (
            vec![TreeNode {
                x: 0.0, y: 0.0, depth: 0, height: 0.0, color_idx: 0, above_cut: false,
                label: labels[0].clone(), children: Vec::new(), parent: None,
            }],
            vec![0],
        );
    }

    let dist = pairwise_distances(&vectors[..n]);
    let mut members: Vec<Vec<usize>> = (0..n).map(|i| vec![i]).collect();
    // node_id[cluster_slot] -> arena index in `arena` below; children/height recorded per merge
    let mut arena_children: Vec<(usize, usize)> = Vec::new(); // (left_slot, right_slot) for slots >= n
    let mut arena_height: Vec<f64> = vec![0.0; n];
    let mut alive: Vec<usize> = (0..n).collect();

    while alive.len() > 1 {
        let mut best_d = f64::INFINITY;
        let mut best = (0usize, 1usize);
        for ii in 0..alive.len() {
            for jj in (ii + 1)..alive.len() {
                let a = alive[ii];
                let b = alive[jj];
                let ma = &members[a];
                let mb = &members[b];
                let mut sum = 0.0;
                for &x in ma {
                    for &y in mb {
                        sum += dist[x][y];
                    }
                }
                let avg = sum / (ma.len() * mb.len()) as f64;
                if avg < best_d {
                    best_d = avg;
                    best = (ii, jj);
                }
            }
        }
        let (ii, jj) = best;
        let a = alive[ii];
        let b = alive[jj];
        let mut merged = members[a].clone();
        merged.extend(members[b].iter().copied());
        members.push(merged);
        arena_children.push((a, b));
        arena_height.push(best_d);
        let new_slot = arena_height.len() - 1;
        alive.remove(jj);
        alive.remove(ii);
        alive.push(new_slot);
    }
    let root_slot = alive[0];
    let max_height = arena_height.iter().copied().fold(0.0_f64, f64::max).max(1e-9);

    // Cut the tree into `clusters` top-level groups (standard "split the highest
    // merge first" rule), coloring everything below a cut point and leaving the
    // trunk above it in a neutral color.
    let k = clusters.max(1).min(n);
    let mut cut_slots: Vec<usize> = vec![root_slot];
    while cut_slots.len() < k {
        let (pos, _) = cut_slots
            .iter()
            .enumerate()
            .filter(|(_, &s)| s >= n)
            .max_by(|a, b| arena_height[*a.1].partial_cmp(&arena_height[*b.1]).unwrap())
            .expect("more clusters requested than mergeable internal nodes");
        let slot = cut_slots.remove(pos);
        let (l, r) = arena_children[slot - n];
        cut_slots.push(l);
        cut_slots.push(r);
    }

    let mut nodes: Vec<TreeNode> = (0..n)
        .map(|i| TreeNode {
            x: 0.0, y: 0.0, depth: 0, height: 0.0, color_idx: 0, above_cut: false,
            label: labels[i].clone(), children: Vec::new(), parent: None,
        })
        .collect();
    for h in n..arena_height.len() {
        nodes.push(TreeNode {
            x: 0.0, y: 0.0, depth: 0, height: arena_height[h], color_idx: 0, above_cut: true,
            label: String::new(), children: Vec::new(), parent: None,
        });
    }
    for (slot, &(l, r)) in arena_children.iter().enumerate() {
        let idx = slot + n;
        nodes[idx].children = vec![l, r];
        nodes[l].parent = Some(idx);
        nodes[r].parent = Some(idx);
    }

    fn mark_depth(nodes: &mut [TreeNode], i: usize, d: usize) {
        nodes[i].depth = d;
        let kids = nodes[i].children.clone();
        for c in kids {
            mark_depth(nodes, c, d + 1);
        }
    }
    mark_depth(&mut nodes, root_slot, 0);

    fn paint(nodes: &mut [TreeNode], i: usize, color_idx: usize) {
        nodes[i].color_idx = color_idx;
        nodes[i].above_cut = false;
        let kids = nodes[i].children.clone();
        for c in kids {
            paint(nodes, c, color_idx);
        }
    }
    for (ci, &slot) in cut_slots.iter().enumerate() {
        paint(&mut nodes, slot, ci);
    }
    let _ = max_height;

    (nodes, vec![root_slot])
}

fn collect_leaves(nodes: &[TreeNode], i: usize, out: &mut Vec<usize>) {
    if nodes[i].children.is_empty() {
        out.push(i);
    } else {
        for &c in &nodes[i].children {
            collect_leaves(nodes, c, out);
        }
    }
}

fn collect_descendants(nodes: &[TreeNode], i: usize, out: &mut Vec<usize>) {
    for &c in &nodes[i].children {
        out.push(c);
        collect_descendants(nodes, c, out);
    }
}

fn collapse_frontier(nodes: &[TreeNode], roots: &[usize], max_depth: usize) -> Vec<usize> {
    fn visit(nodes: &[TreeNode], i: usize, max_depth: usize, frontier: &mut Vec<usize>) {
        if nodes[i].children.is_empty() || nodes[i].depth >= max_depth {
            frontier.push(i);
        } else {
            for &c in &nodes[i].children {
                visit(nodes, c, max_depth, frontier);
            }
        }
    }
    let mut frontier = Vec::new();
    for &r in roots {
        visit(nodes, r, max_depth, &mut frontier);
    }
    frontier
}

fn ordered_leaves(nodes: &[TreeNode], roots: &[usize]) -> Vec<usize> {
    let n = nodes.len();
    let mut leaves: Vec<usize> = (0..n).filter(|&i| nodes[i].children.is_empty()).collect();
    leaves.sort_by_key(|&i| {
        let mut path = Vec::new();
        let mut cur = i;
        loop {
            path.push(cur);
            match nodes[cur].parent {
                Some(p) => cur = p,
                None => break,
            }
        }
        path.reverse();
        path
    });
    let _ = roots;
    leaves
}

pub fn assign_positions_vertical(
    nodes: &mut Vec<TreeNode>,
    roots: &[usize],
    width: f64, height: f64,
    pad_l: f64, pad_r: f64, pad_t: f64, pad_b: f64,
) {
    let n = nodes.len();
    let max_height = nodes.iter().map(|nd| nd.height).fold(0.0_f64, f64::max).max(1e-9);

    let leaves = ordered_leaves(nodes, roots);

    let lw = width - pad_l - pad_r;
    let step = if leaves.len() > 1 { lw / (leaves.len() as f64 - 1.0) } else { 0.0 };
    for (k, &li) in leaves.iter().enumerate() {
        nodes[li].x = pad_l + k as f64 * step;
    }

    let y_of_height = |hgt: f64| -> f64 {
        pad_t + (height - pad_t - pad_b) * (1.0 - hgt / max_height)
    };

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| nodes[a].depth.cmp(&nodes[b].depth).reverse());

    for &i in &order {
        if !nodes[i].children.is_empty() {
            let cx: f64 = nodes[i].children.iter().map(|&c| nodes[c].x).sum::<f64>()
                / nodes[i].children.len() as f64;
            nodes[i].x = cx;
        }
        nodes[i].y = y_of_height(nodes[i].height);
    }
}

pub fn assign_positions_horizontal(
    nodes: &mut Vec<TreeNode>,
    roots: &[usize],
    width: f64, height: f64,
    pad_l: f64, pad_r: f64, pad_t: f64, pad_b: f64,
) {
    let n = nodes.len();
    let max_height = nodes.iter().map(|nd| nd.height).fold(0.0_f64, f64::max).max(1e-9);

    let leaves = ordered_leaves(nodes, roots);

    let lh = height - pad_t - pad_b;
    let step = if leaves.len() > 1 { lh / (leaves.len() as f64 - 1.0) } else { 0.0 };
    for (k, &li) in leaves.iter().enumerate() {
        nodes[li].y = pad_t + k as f64 * step;
    }

    let x_of_height = |hgt: f64| -> f64 {
        pad_l + (width - pad_l - pad_r) * (1.0 - hgt / max_height)
    };

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| nodes[a].depth.cmp(&nodes[b].depth).reverse());

    for &i in &order {
        if !nodes[i].children.is_empty() {
            let cy: f64 = nodes[i].children.iter().map(|&c| nodes[c].y).sum::<f64>()
                / nodes[i].children.len() as f64;
            nodes[i].y = cy;
        }
        nodes[i].x = x_of_height(nodes[i].height);
    }
}

pub fn assign_positions_radial(
    nodes: &mut Vec<TreeNode>,
    roots: &[usize],
    cx: f64, cy: f64, r_max: f64,
) {
    use std::f64::consts::PI;
    let n = nodes.len();
    let max_height = nodes.iter().map(|nd| nd.height).fold(0.0_f64, f64::max).max(1e-9);

    let leaves = ordered_leaves(nodes, roots);
    let nl = leaves.len().max(1);

    let r_of_height = |hgt: f64| -> f64 { r_max * (1.0 - hgt / max_height) };

    let mut leaf_angles: Vec<f64> = vec![0.0; n];
    for (k, &li) in leaves.iter().enumerate() {
        let angle = 2.0 * PI * k as f64 / nl as f64 - PI / 2.0;
        leaf_angles[li] = angle;
        nodes[li].x = cx + r_max * angle.cos();
        nodes[li].y = cy + r_max * angle.sin();
    }

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| nodes[a].depth.cmp(&nodes[b].depth).reverse());

    for &i in &order {
        if !nodes[i].children.is_empty() {
            let avg_a: f64 = nodes[i].children.iter().map(|&c| leaf_angles[c]).sum::<f64>()
                / nodes[i].children.len() as f64;
            leaf_angles[i] = avg_a;
            let r = r_of_height(nodes[i].height);
            nodes[i].x = cx + r * avg_a.cos();
            nodes[i].y = cy + r * avg_a.sin();
        }
    }
}

pub fn assign_positions_spiral(
    nodes: &mut Vec<TreeNode>,
    roots: &[usize],
    cx: f64, cy: f64, r_max: f64,
    twist: f64,
) {
    use std::f64::consts::PI;
    let n = nodes.len();
    let max_height = nodes.iter().map(|nd| nd.height).fold(0.0_f64, f64::max).max(1e-9);
    let max_depth = nodes.iter().map(|nd| nd.depth).max().unwrap_or(0).max(1) as f64;

    let leaves = ordered_leaves(nodes, roots);
    let nl = leaves.len().max(1);

    let r_of_height = |hgt: f64| -> f64 { r_max * (1.0 - hgt / max_height) };

    let mut base_angle: Vec<f64> = vec![0.0; n];
    for (k, &li) in leaves.iter().enumerate() {
        base_angle[li] = 2.0 * PI * k as f64 / nl as f64 - PI / 2.0;
    }

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| nodes[a].depth.cmp(&nodes[b].depth).reverse());
    for &i in &order {
        if !nodes[i].children.is_empty() {
            base_angle[i] = nodes[i].children.iter().map(|&c| base_angle[c]).sum::<f64>()
                / nodes[i].children.len() as f64;
        }
    }

    for i in 0..n {
        let r = r_of_height(nodes[i].height);
        let a = base_angle[i] + twist * (nodes[i].depth as f64 / max_depth);
        nodes[i].x = cx + r * a.cos();
        nodes[i].y = cy + r * a.sin();
    }
}

fn write_radial_link(
    buf: &mut Vec<u8>,
    cx: f64, cy: f64,
    px: f64, py: f64,
    x: f64, y: f64,
    hx: &[u8; 6],
    width: f64, opacity: f64,
    smooth: bool,
) {
    fn angle_of(cx: f64, cy: f64, x: f64, y: f64) -> f64 {
        (y - cy).atan2(x - cx)
    }
    let r_parent = ((px - cx).powi(2) + (py - cy).powi(2)).sqrt();
    let a_parent = angle_of(cx, cy, px, py);
    let a_child = angle_of(cx, cy, x, y);
    push_b(buf, b"<path fill=\"none\" stroke=\"#");
    buf.extend_from_slice(hx);
    push_b(buf, b"\" stroke-width=\"");
    push_f2(buf, width);
    push_b(buf, b"\" stroke-opacity=\"");
    push_f2(buf, opacity);
    push_b(buf, b"\" d=\"M");
    push_f2(buf, px);
    push_b(buf, b",");
    push_f2(buf, py);
    if smooth {
        let large_arc = if (a_child - a_parent).abs() > std::f64::consts::PI { 1 } else { 0 };
        let sweep = if a_child > a_parent { 1 } else { 0 };
        let ax = cx + r_parent * a_child.cos();
        let ay = cy + r_parent * a_child.sin();
        push_b(buf, b" A");
        push_f2(buf, r_parent);
        push_b(buf, b",");
        push_f2(buf, r_parent);
        push_b(buf, b" 0 ");
        push_i(buf, large_arc);
        push_b(buf, b",");
        push_i(buf, sweep);
        push_b(buf, b" ");
        push_f2(buf, ax);
        push_b(buf, b",");
        push_f2(buf, ay);
        push_b(buf, b" L");
        push_f2(buf, x);
        push_b(buf, b",");
        push_f2(buf, y);
    } else {
        push_b(buf, b" L");
        push_f2(buf, x);
        push_b(buf, b",");
        push_f2(buf, y);
    }
    push_b(buf, b"\"/>");
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> u32 {
    let h = h.rem_euclid(360.0) / 60.0;
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - (h % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r1, g1, b1) = match h as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let r = ((r1 + m) * 255.0).round().clamp(0.0, 255.0) as u32;
    let g = ((g1 + m) * 255.0).round().clamp(0.0, 255.0) as u32;
    let b = ((b1 + m) * 255.0).round().clamp(0.0, 255.0) as u32;
    (r << 16) | (g << 8) | b
}

fn angular_color(x: f64, y: f64, cx: f64, cy: f64, r_max: f64) -> u32 {
    use std::f64::consts::PI;
    let dx = x - cx;
    let dy = y - cy;
    let r = (dx * dx + dy * dy).sqrt();
    if r < 3.0 {
        return 0x334155;
    }
    let hue = (dy.atan2(dx) + PI) / (2.0 * PI) * 360.0;
    let light = 0.30 + 0.20 * (r / r_max).min(1.0);
    hsl_to_rgb(hue, 0.62, light)
}

pub fn render_genealogy_impl(cfg: &DendrogramConfig, twist: f64) -> String {
    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let cx = w / 2.0;
    let cy = h / 2.0;
    let r_max = (w.min(h) / 2.0 - 26.0).max(20.0);

    let Some((mut nodes, roots)) = tree_for(cfg) else { return String::new(); };
    assign_positions_spiral(&mut nodes, &roots, cx, cy, r_max, twist);

    let n = nodes.len();
    let mut subtree_size = vec![1usize; n];
    let mut by_depth_desc: Vec<usize> = (0..n).collect();
    by_depth_desc.sort_by(|&a, &b| nodes[b].depth.cmp(&nodes[a].depth));
    for &i in &by_depth_desc {
        if let Some(pi) = nodes[i].parent {
            subtree_size[pi] += subtree_size[i];
        }
    }
    let max_subtree = subtree_size.iter().copied().max().unwrap_or(1).max(1) as f64;

    let colors: Vec<u32> = (0..n).map(|i| angular_color(nodes[i].x, nodes[i].y, cx, cy, r_max)).collect();

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n * 320 + 4096);
    svg_header(&mut buf, cfg, hid, cx);

    for i in 0..n {
        if let Some(pi) = nodes[i].parent {
            let hx = hex6(colors[i]);
            let fade = 0.10 + 0.55 * (nodes[i].depth as f64 / n.max(1) as f64).min(1.0);
            write_radial_link(
                &mut buf, cx, cy,
                nodes[pi].x, nodes[pi].y, nodes[i].x, nodes[i].y,
                &hx, cfg.line_width * 2.6, fade * 0.16, false,
            );
            write_radial_link(
                &mut buf, cx, cy,
                nodes[pi].x, nodes[pi].y, nodes[i].x, nodes[i].y,
                &hx, cfg.line_width * 0.45, fade.max(0.24), false,
            );
        }
    }

    for i in 0..n {
        let hx = hex6(colors[i]);
        let bump = (subtree_size[i] as f64 / max_subtree).sqrt();
        let r = if nodes[i].children.is_empty() { 1.5 } else { 1.1 + bump * 4.8 };
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, nodes[i].x);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, nodes[i].y);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, r);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.92\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\"/>");

        if cfg.show_labels && nodes[i].children.is_empty() {
            let dx = nodes[i].x - cx;
            let dy = nodes[i].y - cy;
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
            push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"7\" fill=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\">");
            escape_xml(&mut buf, &nodes[i].label);
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}

#[cfg(test)]
mod genealogy_tests {
    use super::*;
    use crate::plot::statistical::dendrogram::config::DendrogramConfig;
    use std::f64::consts::PI;

    fn forest(n_families: usize, gens: usize, branch: usize) -> (Vec<String>, Vec<String>) {
        let mut labels = Vec::new();
        let mut parents = Vec::new();
        for f in 0..n_families {
            let root = format!("f{f}g0");
            labels.push(root.clone());
            parents.push(String::new());
            let mut frontier = vec![root];
            for g in 1..=gens {
                let mut next = Vec::new();
                for p in &frontier {
                    for b in 0..branch {
                        let name = format!("{p}-{g}-{b}");
                        labels.push(name.clone());
                        parents.push(p.clone());
                        next.push(name);
                    }
                }
                frontier = next;
            }
        }
        (labels, parents)
    }

    fn cfg<'a>(labels: &'a [String], parents: &'a [String]) -> DendrogramConfig<'a> {
        DendrogramConfig {
            title: "Test",
            labels,
            parents,
            width: 600,
            height: 600,
            ..DendrogramConfig::default()
        }
    }

    #[test]
    fn every_family_root_lands_at_the_shared_center() {
        let (labels, parents) = forest(3, 2, 2);
        let c = cfg(&labels, &parents);
        let (mut nodes, roots) = tree_for(&c).unwrap();
        assert_eq!(roots.len(), 3);
        assign_positions_spiral(&mut nodes, &roots, 300.0, 300.0, 250.0, PI * 0.9);
        for &r in &roots {
            assert!((nodes[r].x - 300.0).abs() < 1e-6);
            assert!((nodes[r].y - 300.0).abs() < 1e-6);
        }
    }

    #[test]
    fn leaves_reach_the_outer_radius_and_stay_within_it() {
        let (labels, parents) = forest(2, 3, 2);
        let c = cfg(&labels, &parents);
        let (mut nodes, roots) = tree_for(&c).unwrap();
        assign_positions_spiral(&mut nodes, &roots, 300.0, 300.0, 250.0, PI * 0.9);
        for n in &nodes {
            let r = ((n.x - 300.0).powi(2) + (n.y - 300.0).powi(2)).sqrt();
            assert!(r <= 250.0 + 1e-6, "node radius {r} exceeds r_max");
        }
        let max_r = nodes
            .iter()
            .map(|n| ((n.x - 300.0).powi(2) + (n.y - 300.0).powi(2)).sqrt())
            .fold(0.0_f64, f64::max);
        assert!((max_r - 250.0).abs() < 1e-6, "no node reached the outer radius: {max_r}");
    }

    #[test]
    fn renders_one_dot_and_a_glow_plus_crisp_link_per_node_minus_roots() {
        let (labels, parents) = forest(3, 2, 2);
        let n = labels.len();
        let n_roots = 3;
        let c = cfg(&labels, &parents);
        let html = render_genealogy_impl(&c, PI * 0.9);
        assert!(!html.is_empty());
        assert_eq!(html.matches("<circle").count(), n);
        assert_eq!(html.matches("<path").count(), (n - n_roots) * 2);
        assert!(html.contains("class=\"sp-bg\""));
    }

    #[test]
    fn distinct_family_roots_get_distinct_colors() {
        let (labels, parents) = forest(3, 1, 1);
        let c = DendrogramConfig { palette: &[0xFF0000, 0x00FF00, 0x0000FF], ..cfg(&labels, &parents) };
        let (nodes, roots) = tree_for(&c).unwrap();
        let colors: std::collections::HashSet<u32> = roots.iter().map(|&r| node_color(&c, &nodes[r])).collect();
        assert_eq!(colors.len(), 3);
    }

    #[test]
    fn a_node_with_more_descendants_draws_a_bigger_circle_than_a_leaf() {
        let (labels, parents) = forest(1, 3, 3);
        let c = cfg(&labels, &parents);
        let html = render_genealogy_impl(&c, PI * 0.9);
        let mut by_idx: std::collections::HashMap<i32, f64> = std::collections::HashMap::new();
        for tag in html.split("<circle").skip(1) {
            let tag = &tag[..tag.find("/>").unwrap_or(tag.len())];
            let r_key = "r=\"";
            let r_start = tag.find(r_key).unwrap() + r_key.len();
            let r: f64 = tag[r_start..].split('"').next().unwrap().parse().unwrap();
            let idx_key = "data-idx=\"";
            let idx_start = tag.find(idx_key).unwrap() + idx_key.len();
            let idx: i32 = tag[idx_start..].split('"').next().unwrap().parse().unwrap();
            by_idx.insert(idx, r);
        }
        let root_r = by_idx[&0];
        let leaf_r = by_idx
            .iter()
            .filter(|(&i, _)| i != 0)
            .map(|(_, &r)| r)
            .fold(f64::INFINITY, f64::min);
        assert!(root_r > leaf_r, "root radius {root_r} should exceed the smallest leaf radius {leaf_r}");
    }

    #[test]
    fn the_svg_carries_a_matching_viewbox_so_it_scales_instead_of_clipping() {
        let (labels, parents) = forest(2, 2, 2);
        let c = cfg(&labels, &parents);
        let html = render_genealogy_impl(&c, PI * 0.9);
        assert!(html.contains("viewBox=\"0 0 600 600\""));
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let labels: Vec<String> = vec![];
        let parents: Vec<String> = vec![];
        let c = cfg(&labels, &parents);
        assert!(render_genealogy_impl(&c, PI * 0.9).is_empty());
    }

    #[test]
    fn perf_rendering_a_large_multi_family_forest_stays_fast() {
        let (labels, parents) = forest(6, 6, 2);
        let c = cfg(&labels, &parents);
        let start = std::time::Instant::now();
        let html = render_genealogy_impl(&c, PI * 0.9);
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }

    #[test]
    fn perf_rendering_a_dense_thousand_plus_node_forest_stays_fast() {
        let (labels, parents) = forest(7, 4, 5);
        let c = cfg(&labels, &parents);
        assert!(labels.len() > 1000, "forest only produced {} nodes, widen the fixture", labels.len());
        let start = std::time::Instant::now();
        let html = render_genealogy_impl(&c, PI * 0.9);
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 400, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }

    #[test]
    fn angular_color_sweeps_through_distinct_hues_around_the_circle() {
        let cx = 300.0;
        let cy = 300.0;
        let r_max = 250.0;
        let east = angular_color(cx + 200.0, cy, cx, cy, r_max);
        let south = angular_color(cx, cy + 200.0, cx, cy, r_max);
        let west = angular_color(cx - 200.0, cy, cx, cy, r_max);
        let mut distinct = std::collections::HashSet::new();
        distinct.insert(east);
        distinct.insert(south);
        distinct.insert(west);
        assert_eq!(distinct.len(), 3, "opposite angles should map to visibly different hues");
    }

    #[test]
    fn angular_color_at_the_shared_center_stays_neutral() {
        let c = angular_color(300.0, 300.0, 300.0, 300.0, 250.0);
        assert_eq!(c, 0x334155);
    }
}

fn svg_header(buf: &mut Vec<u8>, cfg: &DendrogramConfig, hid: u64, title_x: f64) {
    html_prefix(buf, cfg.title, hid);
    push_b(buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(buf, cfg.width);
    push_b(buf, b"\" height=\"");
    push_i(buf, cfg.height);
    push_b(buf, b"\" viewBox=\"0 0 ");
    push_i(buf, cfg.width);
    push_b(buf, b" ");
    push_i(buf, cfg.height);
    push_b(buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    if !cfg.title.is_empty() {
        push_b(buf, b"<text x=\"");
        push_f2(buf, title_x);
        push_b(buf, b"\" y=\"18\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"13\" font-weight=\"700\" fill=\"#1a202c\" class=\"sp-ttl\">");
        escape_xml(buf, cfg.title);
        push_b(buf, b"</text>");
    }
}

pub fn render_impl(cfg: &DendrogramConfig, horizontal: bool, compact: bool) -> String {
    let pad = if compact { (16.0f64, 16.0, 28.0, 36.0) } else { (20.0f64, 40.0, 32.0, 48.0) };
    let (pad_l, pad_r, pad_t, pad_b) = pad;
    let w = cfg.width as f64;
    let h = cfg.height as f64;

    let Some((mut nodes, roots)) = tree_for(cfg) else { return String::new(); };

    if horizontal {
        assign_positions_horizontal(&mut nodes, &roots, w, h, pad_l, pad_r, pad_t, pad_b);
    } else {
        assign_positions_vertical(&mut nodes, &roots, w, h, pad_l, pad_r, pad_t, pad_b);
    }

    let frontier = if compact { collapse_frontier(&nodes, &roots, 2) } else { Vec::new() };
    let hidden: std::collections::HashSet<usize> = if compact {
        let mut hidden = std::collections::HashSet::new();
        for &f in &frontier {
            let mut desc = Vec::new();
            collect_descendants(&nodes, f, &mut desc);
            for d in desc {
                hidden.insert(d);
            }
        }
        hidden
    } else {
        std::collections::HashSet::new()
    };

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(nodes.len() * 180 + 4096);
    svg_header(&mut buf, cfg, hid, w / 2.0);

    for i in 0..nodes.len() {
        if hidden.contains(&i) {
            continue;
        }
        if let Some(pi) = nodes[i].parent {
            let hx = hex6(node_color(cfg, &nodes[i]));
            if horizontal {
                let mid_x = (nodes[pi].x + nodes[i].x) / 2.0;
                push_b(&mut buf, b"<path fill=\"none\" stroke=\"#");
                buf.extend_from_slice(&hx);
                push_b(&mut buf, b"\" stroke-width=\"");
                push_f2(&mut buf, cfg.line_width);
                push_b(&mut buf, b"\" stroke-opacity=\"0.8\" d=\"M");
                push_f2(&mut buf, nodes[pi].x); push_b(&mut buf, b","); push_f2(&mut buf, nodes[pi].y);
                push_b(&mut buf, b"H"); push_f2(&mut buf, mid_x);
                push_b(&mut buf, b"V"); push_f2(&mut buf, nodes[i].y);
                push_b(&mut buf, b"H"); push_f2(&mut buf, nodes[i].x);
                push_b(&mut buf, b"\"/>");
            } else {
                let mid_y = (nodes[pi].y + nodes[i].y) / 2.0;
                push_b(&mut buf, b"<path fill=\"none\" stroke=\"#");
                buf.extend_from_slice(&hx);
                push_b(&mut buf, b"\" stroke-width=\"");
                push_f2(&mut buf, cfg.line_width);
                push_b(&mut buf, b"\" stroke-opacity=\"0.8\" d=\"M");
                push_f2(&mut buf, nodes[pi].x); push_b(&mut buf, b","); push_f2(&mut buf, nodes[pi].y);
                push_b(&mut buf, b"V"); push_f2(&mut buf, mid_y);
                push_b(&mut buf, b"H"); push_f2(&mut buf, nodes[i].x);
                push_b(&mut buf, b"V"); push_f2(&mut buf, nodes[i].y);
                push_b(&mut buf, b"\"/>");
            }
        }
    }

    if compact {
        for &f in &frontier {
            if nodes[f].children.is_empty() {
                continue;
            }
            let mut leaves_under = Vec::new();
            collect_leaves(&nodes, f, &mut leaves_under);
            let x_min = leaves_under.iter().map(|&li| nodes[li].x).fold(f64::INFINITY, f64::min);
            let x_max = leaves_under.iter().map(|&li| nodes[li].x).fold(f64::NEG_INFINITY, f64::max);
            let base_y = nodes[leaves_under[0]].y;
            let hx = hex6(node_color(cfg, &nodes[f]));
            push_b(&mut buf, b"<path d=\"M");
            push_f2(&mut buf, nodes[f].x); push_b(&mut buf, b","); push_f2(&mut buf, nodes[f].y);
            push_b(&mut buf, b"L"); push_f2(&mut buf, x_min); push_b(&mut buf, b","); push_f2(&mut buf, base_y);
            push_b(&mut buf, b"L"); push_f2(&mut buf, x_max); push_b(&mut buf, b","); push_f2(&mut buf, base_y);
            push_b(&mut buf, b"Z\" fill=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" fill-opacity=\"0.28\" stroke=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stroke-width=\"1.2\" data-idx=\"");
            push_i(&mut buf, f as i32);
            push_b(&mut buf, b"\"/>");
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, (x_min + x_max) / 2.0);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, base_y + 14.0);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" font-weight=\"700\" fill=\"#374151\">(");
            push_i(&mut buf, leaves_under.len() as i32);
            push_b(&mut buf, b")</text>");
        }
    }

    let font_size = if compact { 8.5f64 } else { 10.0 };
    for i in 0..nodes.len() {
        if hidden.contains(&i) {
            continue;
        }
        let hx = hex6(node_color(cfg, &nodes[i]));
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, nodes[i].x);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, nodes[i].y);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, if nodes[i].children.is_empty() { 3.5 } else { 3.0 });
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke=\"#fff\" stroke-width=\"1.2\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\"/>");

        if cfg.show_labels && nodes[i].children.is_empty() {
            let (tx, ty, anchor) = if horizontal {
                (nodes[i].x + 7.0, nodes[i].y + 3.5, b"start" as &[u8])
            } else {
                (nodes[i].x, nodes[i].y + 14.0, b"middle" as &[u8])
            };
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, tx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, ty);
            push_b(&mut buf, b"\" text-anchor=\"");
            buf.extend_from_slice(anchor);
            push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"");
            push_f2(&mut buf, font_size);
            push_b(&mut buf, b"\" fill=\"#374151\">");
            escape_xml(&mut buf, &nodes[i].label);
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}

pub fn render_radial_impl(cfg: &DendrogramConfig, smooth: bool) -> String {
    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let cx = w / 2.0;
    let cy = h / 2.0 + 6.0;
    let r_max = (w.min(h) / 2.0 - 58.0).max(30.0);

    let Some((mut nodes, roots)) = tree_for(cfg) else { return String::new(); };
    assign_positions_radial(&mut nodes, &roots, cx, cy, r_max);

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(nodes.len() * 200 + 4096);
    svg_header(&mut buf, cfg, hid, cx);

    for ring in 1..=4 {
        let rr = r_max * ring as f64 / 4.0;
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, cy);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, rr);
        push_b(&mut buf, b"\" fill=\"none\" stroke=\"#e5e7eb\" stroke-width=\"0.6\" stroke-dasharray=\"2,3\"/>");
    }

    for i in 0..nodes.len() {
        if let Some(pi) = nodes[i].parent {
            let hx = hex6(node_color(cfg, &nodes[i]));
            write_radial_link(
                &mut buf, cx, cy,
                nodes[pi].x, nodes[pi].y, nodes[i].x, nodes[i].y,
                &hx, cfg.line_width, 0.85, smooth,
            );
        }
    }

    for i in 0..nodes.len() {
        let hx = hex6(node_color(cfg, &nodes[i]));
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, nodes[i].x);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, nodes[i].y);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, if nodes[i].children.is_empty() { 3.2 } else { 2.4 });
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke=\"#fff\" stroke-width=\"1\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\"/>");

        if cfg.show_labels && nodes[i].children.is_empty() {
            let dx = nodes[i].x - cx;
            let dy = nodes[i].y - cy;
            let len = (dx * dx + dy * dy).sqrt().max(1.0);
            let tx = nodes[i].x + dx / len * 10.0;
            let ty = nodes[i].y + dy / len * 10.0;
            let anchor: &[u8] = if dx > 1.0 { b"start" } else if dx < -1.0 { b"end" } else { b"middle" };
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, tx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, ty + 3.5);
            push_b(&mut buf, b"\" text-anchor=\"");
            buf.extend_from_slice(anchor);
            push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\">");
            escape_xml(&mut buf, &nodes[i].label);
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}

pub fn render_elegant_impl(cfg: &DendrogramConfig) -> String {
    let pad_l = 20.0f64; let pad_r = 40.0; let pad_t = 32.0; let pad_b = 48.0;
    let w = cfg.width as f64;
    let h = cfg.height as f64;

    let Some((mut nodes, roots)) = tree_for(cfg) else { return String::new(); };
    assign_positions_vertical(&mut nodes, &roots, w, h, pad_l, pad_r, pad_t, pad_b);

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(nodes.len() * 200 + 4096);
    svg_header(&mut buf, cfg, hid, w / 2.0);

    for i in 0..nodes.len() {
        if let Some(pi) = nodes[i].parent {
            let hx = hex6(node_color(cfg, &nodes[i]));
            push_b(&mut buf, b"<path fill=\"none\" stroke=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stroke-width=\"");
            push_f2(&mut buf, cfg.line_width);
            push_b(&mut buf, b"\" stroke-opacity=\"0.8\" d=\"M");
            push_f2(&mut buf, nodes[pi].x); push_b(&mut buf, b","); push_f2(&mut buf, nodes[pi].y);
            push_b(&mut buf, b"C");
            push_f2(&mut buf, nodes[pi].x); push_b(&mut buf, b",");
            push_f2(&mut buf, (nodes[pi].y + nodes[i].y) / 2.0);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, nodes[i].x); push_b(&mut buf, b",");
            push_f2(&mut buf, (nodes[pi].y + nodes[i].y) / 2.0);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, nodes[i].x); push_b(&mut buf, b","); push_f2(&mut buf, nodes[i].y);
            push_b(&mut buf, b"\"/>");
        }
    }

    for i in 0..nodes.len() {
        let hx = hex6(node_color(cfg, &nodes[i]));
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, nodes[i].x);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, nodes[i].y);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, if nodes[i].children.is_empty() { 3.5 } else { 3.0 });
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke=\"#fff\" stroke-width=\"1.2\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\"/>");

        if cfg.show_labels && nodes[i].children.is_empty() {
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, nodes[i].x);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, nodes[i].y + 14.0);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">");
            escape_xml(&mut buf, &nodes[i].label);
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}

pub fn render_triangular_impl(cfg: &DendrogramConfig) -> String {
    let pad_l = 20.0f64; let pad_r = 40.0; let pad_t = 32.0; let pad_b = 48.0;
    let w = cfg.width as f64;
    let h = cfg.height as f64;

    let Some((mut nodes, roots)) = tree_for(cfg) else { return String::new(); };
    assign_positions_vertical(&mut nodes, &roots, w, h, pad_l, pad_r, pad_t, pad_b);

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(nodes.len() * 180 + 4096);
    svg_header(&mut buf, cfg, hid, w / 2.0);

    for i in 0..nodes.len() {
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
            push_b(&mut buf, b"\" stroke-opacity=\"0.8\" stroke-linecap=\"round\"/>");
        }
    }

    for i in 0..nodes.len() {
        let hx = hex6(node_color(cfg, &nodes[i]));
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, nodes[i].x);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, nodes[i].y);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, if nodes[i].children.is_empty() { 3.5 } else { 3.0 });
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke=\"#fff\" stroke-width=\"1.2\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\"/>");

        if cfg.show_labels && nodes[i].children.is_empty() {
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, nodes[i].x);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, nodes[i].y + 14.0);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">");
            escape_xml(&mut buf, &nodes[i].label);
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}
