use std::collections::HashMap;

pub struct TreeNode {
    pub x:         f64,
    pub y:         f64,
    pub depth:     usize,
    pub color_idx: usize,
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

    let nodes: Vec<TreeNode> = (0..n).map(|i| TreeNode {
        x: 0.0, y: 0.0,
        depth: depth[i],
        color_idx: color_idx[i],
        label: labels[i].clone(),
        children: children[i].clone(),
        parent: parent_of[i],
    }).collect();

    (nodes, roots)
}

pub fn assign_positions_vertical(
    nodes: &mut Vec<TreeNode>,
    roots: &[usize],
    width: f64, height: f64,
    pad_l: f64, pad_r: f64, pad_t: f64, pad_b: f64,
) {
    let n = nodes.len();
    let max_depth = nodes.iter().map(|nd| nd.depth).max().unwrap_or(0);

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

    let lw = width - pad_l - pad_r;
    let step = if leaves.len() > 1 { lw / (leaves.len() as f64 - 1.0) } else { 0.0 };
    for (k, &li) in leaves.iter().enumerate() {
        nodes[li].x = pad_l + k as f64 * step;
        nodes[li].y = height - pad_b;
    }

    let depth_y = |d: usize| -> f64 {
        if max_depth == 0 { pad_t }
        else { pad_t + (height - pad_t - pad_b) * (1.0 - d as f64 / max_depth as f64) }
    };

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| nodes[b].depth.cmp(&nodes[a].depth));

    for &i in &order {
        if !nodes[i].children.is_empty() {
            let cx: f64 = nodes[i].children.iter().map(|&c| nodes[c].x).sum::<f64>()
                / nodes[i].children.len() as f64;
            nodes[i].x = cx;
        }
        nodes[i].y = depth_y(nodes[i].depth);
    }
}

pub fn assign_positions_horizontal(
    nodes: &mut Vec<TreeNode>,
    roots: &[usize],
    width: f64, height: f64,
    pad_l: f64, pad_r: f64, pad_t: f64, pad_b: f64,
) {
    let n = nodes.len();
    let max_depth = nodes.iter().map(|nd| nd.depth).max().unwrap_or(0);

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

    let lh = height - pad_t - pad_b;
    let step = if leaves.len() > 1 { lh / (leaves.len() as f64 - 1.0) } else { 0.0 };
    for (k, &li) in leaves.iter().enumerate() {
        nodes[li].y = pad_t + k as f64 * step;
        nodes[li].x = width - pad_r;
    }

    let depth_x = |d: usize| -> f64 {
        if max_depth == 0 { pad_l }
        else { pad_l + (width - pad_l - pad_r) * (d as f64 / max_depth as f64) }
    };

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| nodes[b].depth.cmp(&nodes[a].depth));

    for &i in &order {
        if !nodes[i].children.is_empty() {
            let cy: f64 = nodes[i].children.iter().map(|&c| nodes[c].y).sum::<f64>()
                / nodes[i].children.len() as f64;
            nodes[i].y = cy;
        }
        nodes[i].x = depth_x(nodes[i].depth);
    }
}

pub fn assign_positions_radial(
    nodes: &mut Vec<TreeNode>,
    roots: &[usize],
    cx: f64, cy: f64, r_max: f64,
) {
    use std::f64::consts::PI;
    let n = nodes.len();
    let max_depth = nodes.iter().map(|nd| nd.depth).max().unwrap_or(0);

    let leaves: Vec<usize> = {
        let mut ls: Vec<usize> = (0..n).filter(|&i| nodes[i].children.is_empty()).collect();
        ls.sort_by_key(|&i| {
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
        ls
    };

    let nl = leaves.len();
    for (k, &li) in leaves.iter().enumerate() {
        let angle = 2.0 * PI * k as f64 / nl as f64 - PI / 2.0;
        nodes[li].x = cx + r_max * angle.cos();
        nodes[li].y = cy + r_max * angle.sin();
    }

    let depth_r = |d: usize| -> f64 {
        if max_depth == 0 { 0.0 }
        else { r_max * (d as f64 / max_depth as f64) }
    };

    let mut leaf_angles: Vec<f64> = vec![0.0; n];
    for (k, &li) in leaves.iter().enumerate() {
        leaf_angles[li] = 2.0 * PI * k as f64 / nl as f64 - PI / 2.0;
    }

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| nodes[b].depth.cmp(&nodes[a].depth));

    for &i in &order {
        if !nodes[i].children.is_empty() {
            let avg_a: f64 = nodes[i].children.iter().map(|&c| leaf_angles[c]).sum::<f64>()
                / nodes[i].children.len() as f64;
            leaf_angles[i] = avg_a;
            let r = depth_r(nodes[i].depth);
            nodes[i].x = cx + r * avg_a.cos();
            nodes[i].y = cy + r * avg_a.sin();
        }
    }
}
