use super::common::{node_color, render_impl, TreeNode};
use super::config::DendrogramConfig;
use crate::plot::statistical::common::{hex6, push_b, push_f2, push_i};
use std::collections::HashSet;

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

pub(crate) fn frontier_and_hidden(nodes: &[TreeNode], roots: &[usize]) -> (Vec<usize>, HashSet<usize>) {
    let frontier = collapse_frontier(nodes, roots, 2);
    let mut hidden = HashSet::new();
    for &f in &frontier {
        let mut desc = Vec::new();
        collect_descendants(nodes, f, &mut desc);
        for d in desc {
            hidden.insert(d);
        }
    }
    (frontier, hidden)
}

pub(crate) fn write_frontier_wedges(buf: &mut Vec<u8>, cfg: &DendrogramConfig, nodes: &[TreeNode], frontier: &[usize]) {
    for &f in frontier {
        if nodes[f].children.is_empty() {
            continue;
        }
        let mut leaves_under = Vec::new();
        collect_leaves(nodes, f, &mut leaves_under);
        let x_min = leaves_under.iter().map(|&li| nodes[li].x).fold(f64::INFINITY, f64::min);
        let x_max = leaves_under.iter().map(|&li| nodes[li].x).fold(f64::NEG_INFINITY, f64::max);
        let base_y = nodes[leaves_under[0]].y;
        let hx = hex6(node_color(cfg, &nodes[f]));
        push_b(buf, b"<path d=\"M");
        push_f2(buf, nodes[f].x); push_b(buf, b","); push_f2(buf, nodes[f].y);
        push_b(buf, b"L"); push_f2(buf, x_min); push_b(buf, b","); push_f2(buf, base_y);
        push_b(buf, b"L"); push_f2(buf, x_max); push_b(buf, b","); push_f2(buf, base_y);
        push_b(buf, b"Z\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(buf, b"\" fill-opacity=\"0.28\" stroke=\"#");
        buf.extend_from_slice(&hx);
        push_b(buf, b"\" stroke-width=\"1.2\" data-idx=\"");
        push_i(buf, f as i32);
        push_b(buf, b"\"/>");
        push_b(buf, b"<text x=\"");
        push_f2(buf, (x_min + x_max) / 2.0);
        push_b(buf, b"\" y=\"");
        push_f2(buf, base_y + 14.0);
        push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" font-weight=\"700\" fill=\"#374151\">(");
        push_i(buf, leaves_under.len() as i32);
        push_b(buf, b")</text>");
    }
}

#[crate::chart_demo("labels=[\"A1\",\"A2\",\"A3\",\"B1\",\"B2\",\"B3\",\"C1\",\"C2\",\"C3\"], matrix=[[1,1],[1.2,0.9],[0.9,1.1],[5,5],[5.2,4.8],[4.9,5.1],[1,5],[1.1,4.9],[0.9,5.2]], variant=\"compact\"")]
pub fn render(cfg: &DendrogramConfig) -> String {
    render_impl(cfg, false, true)
}
