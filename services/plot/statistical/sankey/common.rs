use crate::plot::statistical::common::{push_b, push_f2};

pub struct SankeyLayout {
    pub n_nodes:  usize,
    pub layers:   Vec<usize>,
    pub y:        Vec<f64>,
    pub h:        Vec<f64>,
    pub x:        Vec<f64>,
    pub n_layers: usize,
}

pub fn compute_layout(
    n: usize,
    sources: &[i32],
    targets: &[i32],
    weights: &[f64],
    pad_l: i32,
    pad_t: i32,
    plot_w: i32,
    plot_h: i32,
    node_width: i32,
    node_gap: i32,
) -> SankeyLayout {
    compute_layout_impl(
        n, sources, targets, weights, pad_l, pad_t, plot_w, plot_h, node_width, node_gap, false,
    )
}

pub fn compute_layout_sorted(
    n: usize,
    sources: &[i32],
    targets: &[i32],
    weights: &[f64],
    pad_l: i32,
    pad_t: i32,
    plot_w: i32,
    plot_h: i32,
    node_width: i32,
    node_gap: i32,
) -> SankeyLayout {
    compute_layout_impl(
        n, sources, targets, weights, pad_l, pad_t, plot_w, plot_h, node_width, node_gap, true,
    )
}

fn compute_layout_impl(
    n: usize,
    sources: &[i32],
    targets: &[i32],
    weights: &[f64],
    pad_l: i32,
    pad_t: i32,
    plot_w: i32,
    plot_h: i32,
    node_width: i32,
    node_gap: i32,
    sort_by_value: bool,
) -> SankeyLayout {
    let e = sources.len().min(targets.len()).min(weights.len());

    let mut depth = vec![0usize; n];
    let mut in_edges: Vec<Vec<usize>> = vec![Vec::new(); n];
    for k in 0..e {
        let s = sources[k] as usize;
        let t = targets[k] as usize;
        if s < n && t < n {
            in_edges[t].push(s);
        }
    }
    let mut changed = true;
    while changed {
        changed = false;
        for t in 0..n {
            for &s in &in_edges[t] {
                if depth[s] + 1 > depth[t] {
                    depth[t] = depth[s] + 1;
                    changed = true;
                }
            }
        }
    }
    let n_layers = depth.iter().copied().max().unwrap_or(0) + 1;

    let mut by_layer: Vec<Vec<usize>> = vec![Vec::new(); n_layers];
    for i in 0..n {
        by_layer[depth[i]].push(i);
    }

    let layer_w = if n_layers > 1 {
        (plot_w - node_width) / (n_layers as i32 - 1).max(1)
    } else {
        plot_w / 2
    };

    let mut node_x = vec![0.0f64; n];
    for l in 0..n_layers {
        let x = pad_l as f64 + l as f64 * layer_w as f64;
        for &i in &by_layer[l] {
            node_x[i] = x;
        }
    }

    let mut node_val = vec![0.0f64; n];
    for k in 0..e {
        let s = sources[k] as usize;
        let t = targets[k] as usize;
        if s < n && t < n {
            node_val[s] += weights[k];
            node_val[t] += weights[k];
        }
    }
    for i in 0..n {
        if node_val[i] == 0.0 {
            node_val[i] = 1.0;
        }
    }
    if sort_by_value {
        for layer in by_layer.iter_mut() {
            layer.sort_by(|&a, &b| node_val[b].partial_cmp(&node_val[a]).unwrap_or(std::cmp::Ordering::Equal));
        }
    }
    let total: f64 = by_layer.iter().map(|layer| {
        layer.iter().map(|&i| node_val[i]).sum::<f64>()
    }).fold(0.0f64, f64::max);

    let scale = (plot_h as f64 - node_gap as f64 * n as f64) / total.max(1.0);

    let mut node_h = vec![0.0f64; n];
    let mut node_y = vec![0.0f64; n];
    for l in 0..n_layers {
        let y = pad_t as f64;
        let layer = &by_layer[l];
        let total_h: f64 = layer.iter().map(|&i| node_val[i] * scale).sum();
        let slack = (plot_h as f64 - total_h).max(0.0);
        let gap = if layer.len() > 1 {
            slack / (layer.len() as f64 - 1.0)
        } else {
            slack / 2.0
        };
        let start_y = if layer.len() == 1 { y + slack / 2.0 } else { y };
        let mut cur_y = start_y;
        for &i in layer {
            node_h[i] = (node_val[i] * scale).max(4.0);
            node_y[i] = cur_y;
            cur_y += node_h[i] + gap.max(node_gap as f64);
        }
    }

    SankeyLayout {
        n_nodes:  n,
        layers:   depth,
        y:        node_y,
        h:        node_h,
        x:        node_x,
        n_layers,
    }
}

pub fn sankey_link_path(
    buf: &mut Vec<u8>,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    h0: f64,
    h1: f64,
    node_width: i32,
) {
    let sx = x0 + node_width as f64;
    let ex = x1;
    let cx = (sx + ex) / 2.0;
    push_b(buf, b"M");
    push_f2(buf, sx);
    push_b(buf, b",");
    push_f2(buf, y0);
    push_b(buf, b"C");
    push_f2(buf, cx);
    push_b(buf, b",");
    push_f2(buf, y0);
    push_b(buf, b" ");
    push_f2(buf, cx);
    push_b(buf, b",");
    push_f2(buf, y1);
    push_b(buf, b" ");
    push_f2(buf, ex);
    push_b(buf, b",");
    push_f2(buf, y1);
    push_b(buf, b"L");
    push_f2(buf, ex);
    push_b(buf, b",");
    push_f2(buf, y1 + h1);
    push_b(buf, b"C");
    push_f2(buf, cx);
    push_b(buf, b",");
    push_f2(buf, y1 + h1);
    push_b(buf, b" ");
    push_f2(buf, cx);
    push_b(buf, b",");
    push_f2(buf, y0 + h0);
    push_b(buf, b" ");
    push_f2(buf, sx);
    push_b(buf, b",");
    push_f2(buf, y0 + h0);
    push_b(buf, b"Z");
}