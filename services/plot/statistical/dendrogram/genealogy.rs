use super::common::{ordered_leaves, svg_header, tree_for, write_radial_link, TreeNode};
use super::config::DendrogramConfig;
use crate::html::hover::{html_id, html_suffix, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i};
use std::f64::consts::PI;
#[cfg(test)]
use super::common::node_color;

fn assign_positions_spiral(
    nodes: &mut Vec<TreeNode>,
    roots: &[usize],
    cx: f64, cy: f64, r_max: f64,
    twist: f64,
) {
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

fn angular_color(x: f64, y: f64, cx: f64, cy: f64, r_max: f64, influence: f64) -> u32 {
    let dx = x - cx;
    let dy = y - cy;
    let r = (dx * dx + dy * dy).sqrt();
    if r < 3.0 {
        return 0x334155;
    }
    let hue = (dy.atan2(dx) + PI) / (2.0 * PI) * 360.0;
    let light = 0.30 + 0.20 * (r / r_max).min(1.0);
    let sat = 0.30 + 0.42 * influence.clamp(0.0, 1.0).sqrt();
    hsl_to_rgb(hue, sat, light)
}

fn render_impl(cfg: &DendrogramConfig, twist: f64) -> String {
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

    let colors: Vec<u32> = (0..n)
        .map(|i| {
            let influence = (subtree_size[i] as f64 / max_subtree).sqrt();
            angular_color(nodes[i].x, nodes[i].y, cx, cy, r_max, influence)
        })
        .collect();

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

#[crate::chart_demo(
    "labels=[\"f0g0\",\"f0g1n0\",\"f0g1n1\",\"f0g1n2\",\"f0g1n3\",\"f0g1n4\",\"f0g1n5\",\"f0g1n6\",\"f0g1n7\",\"f0g1n8\",\"f0g1n9\",\"f0g1n10\",\"f0g1n11\",\"f0g1n12\",\"f0g2n13\",\"f0g2n14\",\"f0g2n15\",\"f0g2n16\",\"f0g2n17\",\"f0g2n18\",\"f0g2n19\",\"f0g2n20\",\"f0g2n21\",\"f0g2n22\",\"f0g2n23\",\"f0g2n24\",\"f0g2n25\",\"f0g2n26\",\"f0g2n27\",\"f0g2n28\",\"f0g2n29\",\"f0g2n30\",\"f0g2n31\",\"f0g2n32\",\"f0g2n33\",\"f0g2n34\",\"f0g2n35\",\"f0g2n36\",\"f0g2n37\",\"f0g2n38\",\"f0g2n39\",\"f0g2n40\",\"f0g3n41\",\"f0g3n42\",\"f0g3n43\",\"f0g3n44\",\"f0g3n45\",\"f0g3n46\",\"f0g3n47\",\"f0g3n48\",\"f0g3n49\",\"f0g3n50\",\"f0g3n51\",\"f0g3n52\",\"f0g3n53\",\"f0g3n54\",\"f0g3n55\",\"f0g3n56\",\"f0g3n57\",\"f0g3n58\",\"f0g3n59\",\"f0g3n60\",\"f0g3n61\",\"f0g3n62\",\"f0g3n63\",\"f0g3n64\",\"f0g3n65\",\"f0g3n66\",\"f0g3n67\",\"f0g3n68\",\"f0g3n69\",\"f0g3n70\",\"f0g3n71\",\"f0g3n72\",\"f0g3n73\",\"f0g3n74\",\"f0g3n75\",\"f0g3n76\",\"f0g3n77\",\"f0g3n78\",\"f0g3n79\",\"f0g3n80\",\"f0g4n81\",\"f0g4n82\",\"f0g4n83\",\"f0g4n84\",\"f0g4n85\",\"f0g4n86\",\"f0g4n87\",\"f0g4n88\",\"f0g4n89\",\"f0g4n90\",\"f0g4n91\",\"f0g4n92\",\"f0g4n93\",\"f0g4n94\",\"f0g4n95\",\"f0g4n96\",\"f0g4n97\",\"f0g4n98\",\"f0g4n99\",\"f0g4n100\",\"f0g4n101\",\"f0g4n102\",\"f0g4n103\",\"f0g4n104\",\"f0g4n105\",\"f0g4n106\",\"f0g4n107\",\"f0g4n108\",\"f0g4n109\",\"f0g4n110\",\"f0g4n111\",\"f0g4n112\",\"f0g4n113\",\"f0g4n114\",\"f0g4n115\",\"f0g4n116\",\"f0g4n117\",\"f0g4n118\",\"f0g4n119\",\"f0g4n120\",\"f0g4n121\",\"f0g4n122\",\"f0g4n123\",\"f0g4n124\",\"f0g4n125\",\"f0g4n126\",\"f0g4n127\",\"f0g4n128\",\"f0g4n129\",\"f0g4n130\",\"f0g4n131\",\"f0g4n132\",\"f0g4n133\",\"f0g4n134\",\"f0g4n135\",\"f0g4n136\",\"f0g4n137\",\"f0g4n138\",\"f0g4n139\",\"f0g4n140\",\"f0g4n141\",\"f0g4n142\",\"f0g4n143\",\"f0g4n144\",\"f0g4n145\",\"f0g4n146\",\"f0g4n147\",\"f0g4n148\",\"f0g4n149\",\"f0g4n150\",\"f0g4n151\",\"f1g0\",\"f1g1n0\",\"f1g1n1\",\"f1g1n2\",\"f1g1n3\",\"f1g1n4\",\"f1g1n5\",\"f1g1n6\",\"f1g1n7\",\"f1g1n8\",\"f1g2n9\",\"f1g2n10\",\"f1g2n11\",\"f1g2n12\",\"f1g2n13\",\"f1g2n14\",\"f1g2n15\",\"f1g2n16\",\"f1g2n17\",\"f1g2n18\",\"f1g2n19\",\"f1g2n20\",\"f1g2n21\",\"f1g2n22\",\"f1g2n23\",\"f1g2n24\",\"f1g2n25\",\"f1g3n26\",\"f1g3n27\",\"f1g3n28\",\"f1g3n29\",\"f1g3n30\",\"f1g3n31\",\"f1g3n32\",\"f1g3n33\",\"f1g3n34\",\"f1g3n35\",\"f1g3n36\",\"f1g3n37\",\"f1g3n38\",\"f1g3n39\",\"f1g3n40\",\"f1g3n41\",\"f1g3n42\",\"f1g3n43\",\"f1g3n44\",\"f1g3n45\",\"f1g3n46\",\"f1g3n47\",\"f1g3n48\",\"f1g3n49\",\"f1g3n50\",\"f2g0\",\"f2g1n0\",\"f2g1n1\",\"f2g1n2\",\"f2g1n3\",\"f2g1n4\",\"f2g1n5\",\"f2g1n6\",\"f2g1n7\",\"f2g1n8\",\"f2g1n9\",\"f2g1n10\",\"f2g2n11\",\"f2g2n12\",\"f2g2n13\",\"f2g2n14\",\"f2g2n15\",\"f2g2n16\",\"f2g2n17\",\"f2g2n18\",\"f2g2n19\",\"f2g2n20\",\"f2g2n21\",\"f2g2n22\",\"f2g2n23\",\"f2g2n24\",\"f2g2n25\",\"f2g2n26\",\"f2g2n27\",\"f2g2n28\",\"f2g3n29\",\"f2g3n30\",\"f2g3n31\",\"f2g3n32\",\"f2g3n33\",\"f2g3n34\",\"f2g3n35\",\"f2g3n36\",\"f2g3n37\",\"f2g3n38\",\"f2g3n39\",\"f2g3n40\",\"f2g3n41\",\"f2g3n42\",\"f2g3n43\",\"f2g3n44\",\"f2g3n45\",\"f2g3n46\",\"f2g3n47\",\"f2g3n48\",\"f2g3n49\",\"f2g3n50\",\"f2g3n51\",\"f2g3n52\",\"f2g4n53\",\"f2g4n54\",\"f2g4n55\",\"f2g4n56\",\"f2g4n57\",\"f2g4n58\",\"f2g4n59\",\"f2g4n60\",\"f2g4n61\",\"f2g4n62\",\"f2g4n63\",\"f2g4n64\",\"f2g4n65\",\"f2g4n66\",\"f2g4n67\",\"f2g4n68\",\"f2g4n69\",\"f2g4n70\",\"f2g4n71\",\"f2g4n72\",\"f2g4n73\",\"f2g4n74\",\"f2g4n75\",\"f2g4n76\",\"f2g4n77\",\"f2g4n78\",\"f2g4n79\",\"f2g4n80\",\"f2g4n81\",\"f2g4n82\",\"f2g4n83\",\"f2g4n84\",\"f2g4n85\",\"f2g4n86\",\"f2g4n87\",\"f2g5n88\",\"f2g5n89\",\"f2g5n90\",\"f2g5n91\",\"f2g5n92\",\"f2g5n93\",\"f2g5n94\",\"f2g5n95\",\"f2g5n96\",\"f2g5n97\",\"f2g5n98\",\"f2g5n99\",\"f2g5n100\",\"f2g5n101\",\"f2g5n102\",\"f2g5n103\",\"f2g5n104\",\"f2g5n105\",\"f2g5n106\",\"f2g5n107\",\"f2g5n108\",\"f2g5n109\",\"f2g5n110\",\"f2g5n111\",\"f2g5n112\",\"f2g5n113\",\"f2g5n114\",\"f2g5n115\",\"f2g5n116\",\"f2g5n117\",\"f2g5n118\",\"f2g5n119\",\"f2g5n120\",\"f2g5n121\",\"f2g5n122\",\"f2g5n123\",\"f2g5n124\",\"f2g5n125\",\"f2g5n126\",\"f2g5n127\",\"f2g5n128\",\"f2g5n129\",\"f2g5n130\",\"f2g5n131\",\"f2g5n132\",\"f2g5n133\",\"f2g5n134\",\"f2g5n135\",\"f2g5n136\",\"f3g0\",\"f3g1n0\",\"f3g1n1\",\"f3g1n2\",\"f3g1n3\",\"f3g1n4\",\"f3g1n5\",\"f3g1n6\",\"f3g1n7\",\"f3g1n8\",\"f3g2n9\",\"f3g2n10\",\"f3g2n11\",\"f3g2n12\",\"f3g2n13\",\"f3g2n14\",\"f3g2n15\",\"f3g2n16\",\"f3g2n17\",\"f3g2n18\",\"f3g2n19\",\"f3g2n20\",\"f3g2n21\",\"f3g2n22\",\"f4g0\",\"f4g1n0\",\"f4g1n1\",\"f4g1n2\",\"f4g1n3\",\"f4g1n4\",\"f4g1n5\",\"f4g1n6\",\"f4g1n7\",\"f4g1n8\",\"f4g1n9\",\"f4g1n10\",\"f4g1n11\",\"f4g1n12\",\"f4g1n13\",\"f4g1n14\",\"f4g2n15\",\"f4g2n16\",\"f4g2n17\",\"f4g2n18\",\"f4g2n19\",\"f4g2n20\",\"f4g2n21\",\"f4g2n22\",\"f4g2n23\",\"f4g2n24\",\"f4g2n25\",\"f4g2n26\",\"f4g2n27\",\"f4g2n28\",\"f4g2n29\",\"f4g2n30\",\"f4g2n31\",\"f4g2n32\",\"f4g2n33\",\"f4g2n34\",\"f4g2n35\",\"f4g2n36\",\"f4g2n37\",\"f4g2n38\",\"f4g2n39\",\"f4g2n40\",\"f4g2n41\",\"f4g2n42\",\"f4g2n43\",\"f4g2n44\",\"f4g3n45\",\"f4g3n46\",\"f4g3n47\",\"f4g3n48\",\"f4g3n49\",\"f4g3n50\",\"f4g3n51\",\"f4g3n52\",\"f4g3n53\",\"f4g3n54\",\"f4g3n55\",\"f4g3n56\",\"f4g3n57\",\"f4g3n58\",\"f4g3n59\",\"f4g3n60\",\"f4g3n61\",\"f4g3n62\",\"f4g3n63\",\"f4g3n64\",\"f4g3n65\",\"f4g3n66\",\"f4g3n67\",\"f4g3n68\",\"f4g3n69\",\"f4g3n70\",\"f4g3n71\",\"f4g3n72\",\"f4g3n73\",\"f4g3n74\",\"f4g3n75\",\"f4g3n76\",\"f4g3n77\",\"f4g3n78\",\"f4g3n79\",\"f4g3n80\",\"f4g3n81\",\"f4g3n82\",\"f4g3n83\",\"f4g3n84\",\"f4g3n85\",\"f4g3n86\",\"f4g3n87\",\"f4g3n88\",\"f4g3n89\",\"f4g3n90\",\"f4g3n91\",\"f4g3n92\",\"f4g4n93\",\"f4g4n94\",\"f4g4n95\",\"f4g4n96\",\"f4g4n97\",\"f4g4n98\",\"f4g4n99\",\"f4g4n100\",\"f4g4n101\",\"f4g4n102\",\"f4g4n103\",\"f4g4n104\",\"f4g4n105\",\"f4g4n106\",\"f4g4n107\",\"f4g4n108\",\"f4g4n109\",\"f4g4n110\",\"f4g4n111\",\"f4g4n112\",\"f4g4n113\",\"f4g4n114\",\"f4g4n115\",\"f4g4n116\",\"f4g4n117\",\"f4g4n118\",\"f4g4n119\",\"f4g4n120\",\"f4g4n121\",\"f4g4n122\",\"f4g4n123\",\"f4g4n124\",\"f4g4n125\",\"f4g4n126\",\"f4g4n127\",\"f4g4n128\",\"f4g4n129\",\"f4g4n130\",\"f4g4n131\",\"f4g4n132\",\"f4g4n133\",\"f4g4n134\",\"f4g4n135\",\"f4g4n136\",\"f4g4n137\",\"f4g4n138\",\"f4g4n139\",\"f4g4n140\",\"f4g4n141\",\"f4g4n142\",\"f4g4n143\",\"f4g4n144\",\"f4g4n145\",\"f4g4n146\",\"f4g4n147\",\"f4g4n148\",\"f4g4n149\",\"f4g4n150\",\"f4g4n151\",\"f4g4n152\",\"f4g4n153\",\"f4g4n154\",\"f4g4n155\",\"f4g4n156\",\"f4g4n157\",\"f4g4n158\",\"f4g4n159\",\"f4g4n160\",\"f4g4n161\",\"f4g4n162\",\"f4g4n163\",\"f4g4n164\",\"f4g4n165\",\"f5g0\",\"f5g1n0\",\"f5g1n1\",\"f5g1n2\",\"f5g1n3\",\"f5g1n4\",\"f5g1n5\",\"f5g1n6\",\"f5g1n7\",\"f5g1n8\",\"f5g1n9\",\"f6g0\",\"f6g1n0\",\"f6g1n1\",\"f6g1n2\",\"f6g1n3\",\"f6g1n4\",\"f6g1n5\",\"f6g1n6\",\"f6g1n7\",\"f6g1n8\",\"f6g1n9\",\"f6g1n10\",\"f6g1n11\",\"f6g1n12\",\"f6g1n13\",\"f6g1n14\",\"f6g1n15\",\"f6g2n16\",\"f6g2n17\",\"f6g2n18\",\"f6g2n19\",\"f6g2n20\",\"f6g2n21\",\"f6g2n22\",\"f6g2n23\",\"f6g2n24\",\"f6g2n25\",\"f6g2n26\",\"f6g2n27\",\"f6g2n28\",\"f6g2n29\",\"f6g2n30\",\"f6g2n31\",\"f6g2n32\",\"f6g2n33\",\"f6g2n34\",\"f6g2n35\",\"f6g2n36\",\"f6g2n37\",\"f6g2n38\",\"f6g2n39\",\"f6g2n40\",\"f6g2n41\",\"f6g2n42\",\"f6g2n43\",\"f6g2n44\",\"f6g2n45\",\"f6g2n46\",\"f6g2n47\",\"f6g2n48\",\"f6g2n49\",\"f6g3n50\",\"f6g3n51\",\"f6g3n52\",\"f6g3n53\",\"f6g3n54\",\"f6g3n55\",\"f6g3n56\",\"f6g3n57\",\"f6g3n58\",\"f6g3n59\",\"f6g3n60\",\"f6g3n61\",\"f6g3n62\",\"f6g3n63\",\"f6g3n64\",\"f6g3n65\",\"f6g3n66\",\"f6g3n67\",\"f6g3n68\",\"f6g3n69\",\"f6g3n70\",\"f6g3n71\",\"f6g3n72\",\"f6g3n73\",\"f6g3n74\",\"f6g3n75\",\"f6g3n76\",\"f6g3n77\",\"f6g3n78\",\"f6g3n79\",\"f6g3n80\",\"f6g3n81\",\"f6g3n82\",\"f6g3n83\",\"f6g3n84\",\"f6g3n85\",\"f6g3n86\",\"f6g3n87\",\"f6g3n88\",\"f6g3n89\",\"f6g3n90\",\"f6g3n91\",\"f6g3n92\",\"f6g3n93\",\"f6g3n94\",\"f6g3n95\",\"f6g3n96\",\"f6g3n97\",\"f6g3n98\",\"f6g3n99\",\"f6g4n100\",\"f6g4n101\",\"f6g4n102\",\"f6g4n103\",\"f6g4n104\",\"f6g4n105\",\"f6g4n106\",\"f6g4n107\",\"f6g4n108\",\"f6g4n109\",\"f6g4n110\",\"f6g4n111\",\"f6g4n112\",\"f6g4n113\",\"f6g4n114\",\"f6g4n115\",\"f6g4n116\",\"f6g4n117\",\"f6g4n118\",\"f6g4n119\",\"f6g4n120\",\"f6g4n121\",\"f6g4n122\",\"f6g4n123\",\"f6g4n124\",\"f6g4n125\",\"f6g4n126\",\"f6g4n127\",\"f6g4n128\",\"f6g4n129\",\"f6g4n130\",\"f6g4n131\",\"f6g4n132\",\"f6g4n133\",\"f6g4n134\",\"f6g4n135\",\"f6g4n136\",\"f6g4n137\",\"f6g4n138\",\"f6g4n139\",\"f6g4n140\",\"f6g4n141\",\"f6g4n142\",\"f6g4n143\",\"f6g4n144\",\"f6g4n145\",\"f6g4n146\",\"f6g4n147\",\"f6g4n148\",\"f6g4n149\",\"f6g4n150\",\"f6g4n151\",\"f6g4n152\",\"f6g4n153\",\"f6g4n154\",\"f6g4n155\",\"f6g4n156\",\"f6g4n157\",\"f6g4n158\",\"f6g4n159\",\"f6g4n160\",\"f6g4n161\",\"f6g4n162\",\"f6g4n163\",\"f6g4n164\",\"f6g4n165\",\"f6g4n166\",\"f6g4n167\",\"f6g4n168\",\"f6g4n169\",\"f6g4n170\",\"f6g4n171\",\"f6g4n172\",\"f6g4n173\",\"f6g4n174\",\"f6g4n175\",\"f6g4n176\",\"f6g4n177\",\"f7g0\",\"f7g1n0\",\"f7g1n1\",\"f7g1n2\",\"f7g1n3\",\"f7g1n4\",\"f7g1n5\",\"f7g2n6\",\"f7g2n7\",\"f7g2n8\",\"f7g2n9\",\"f7g2n10\",\"f7g2n11\",\"f7g2n12\",\"f7g2n13\",\"f7g2n14\",\"f7g2n15\",\"f8g0\",\"f8g1n0\",\"f8g1n1\",\"f8g1n2\",\"f8g1n3\",\"f8g1n4\",\"f8g1n5\",\"f8g1n6\",\"f8g1n7\",\"f8g1n8\",\"f8g1n9\",\"f8g1n10\",\"f8g1n11\",\"f8g2n12\",\"f8g2n13\",\"f8g2n14\",\"f8g2n15\",\"f8g2n16\",\"f8g2n17\",\"f8g2n18\",\"f8g2n19\",\"f8g2n20\",\"f8g2n21\",\"f8g2n22\",\"f8g2n23\",\"f8g2n24\",\"f8g2n25\",\"f8g2n26\",\"f8g2n27\",\"f8g3n28\",\"f8g3n29\",\"f8g3n30\",\"f8g3n31\",\"f8g3n32\",\"f8g3n33\",\"f8g3n34\",\"f8g3n35\",\"f8g3n36\",\"f8g3n37\",\"f8g3n38\",\"f8g3n39\",\"f8g3n40\",\"f8g3n41\",\"f8g3n42\",\"f8g3n43\",\"f8g3n44\",\"f8g3n45\",\"f8g3n46\",\"f8g3n47\",\"f8g3n48\",\"f8g3n49\"], parents=[\"\",\"f0g0\",\"f0g0\",\"f0g0\",\"f0g0\",\"f0g0\",\"f0g0\",\"f0g0\",\"f0g0\",\"f0g0\",\"f0g0\",\"f0g0\",\"f0g0\",\"f0g0\",\"f0g1n0\",\"f0g1n0\",\"f0g1n0\",\"f0g1n1\",\"f0g1n1\",\"f0g1n2\",\"f0g1n2\",\"f0g1n3\",\"f0g1n3\",\"f0g1n3\",\"f0g1n4\",\"f0g1n4\",\"f0g1n4\",\"f0g1n5\",\"f0g1n6\",\"f0g1n7\",\"f0g1n7\",\"f0g1n7\",\"f0g1n8\",\"f0g1n8\",\"f0g1n9\",\"f0g1n9\",\"f0g1n9\",\"f0g1n10\",\"f0g1n10\",\"f0g1n10\",\"f0g1n11\",\"f0g1n12\",\"f0g2n13\",\"f0g2n13\",\"f0g2n14\",\"f0g2n14\",\"f0g2n15\",\"f0g2n16\",\"f0g2n17\",\"f0g2n18\",\"f0g2n18\",\"f0g2n19\",\"f0g2n19\",\"f0g2n20\",\"f0g2n21\",\"f0g2n22\",\"f0g2n23\",\"f0g2n24\",\"f0g2n25\",\"f0g2n26\",\"f0g2n27\",\"f0g2n28\",\"f0g2n28\",\"f0g2n29\",\"f0g2n29\",\"f0g2n30\",\"f0g2n30\",\"f0g2n31\",\"f0g2n32\",\"f0g2n33\",\"f0g2n33\",\"f0g2n34\",\"f0g2n34\",\"f0g2n35\",\"f0g2n36\",\"f0g2n37\",\"f0g2n37\",\"f0g2n38\",\"f0g2n38\",\"f0g2n39\",\"f0g2n39\",\"f0g2n40\",\"f0g3n41\",\"f0g3n41\",\"f0g3n41\",\"f0g3n42\",\"f0g3n42\",\"f0g3n43\",\"f0g3n43\",\"f0g3n44\",\"f0g3n45\",\"f0g3n45\",\"f0g3n45\",\"f0g3n46\",\"f0g3n46\",\"f0g3n47\",\"f0g3n48\",\"f0g3n49\",\"f0g3n49\",\"f0g3n49\",\"f0g3n50\",\"f0g3n51\",\"f0g3n51\",\"f0g3n52\",\"f0g3n53\",\"f0g3n53\",\"f0g3n54\",\"f0g3n54\",\"f0g3n55\",\"f0g3n56\",\"f0g3n57\",\"f0g3n57\",\"f0g3n57\",\"f0g3n58\",\"f0g3n59\",\"f0g3n60\",\"f0g3n61\",\"f0g3n62\",\"f0g3n62\",\"f0g3n63\",\"f0g3n63\",\"f0g3n64\",\"f0g3n64\",\"f0g3n64\",\"f0g3n65\",\"f0g3n65\",\"f0g3n66\",\"f0g3n66\",\"f0g3n67\",\"f0g3n68\",\"f0g3n68\",\"f0g3n68\",\"f0g3n69\",\"f0g3n69\",\"f0g3n69\",\"f0g3n70\",\"f0g3n71\",\"f0g3n71\",\"f0g3n71\",\"f0g3n72\",\"f0g3n72\",\"f0g3n73\",\"f0g3n73\",\"f0g3n74\",\"f0g3n75\",\"f0g3n75\",\"f0g3n76\",\"f0g3n76\",\"f0g3n77\",\"f0g3n78\",\"f0g3n78\",\"f0g3n79\",\"f0g3n80\",\"\",\"f1g0\",\"f1g0\",\"f1g0\",\"f1g0\",\"f1g0\",\"f1g0\",\"f1g0\",\"f1g0\",\"f1g0\",\"f1g1n0\",\"f1g1n0\",\"f1g1n0\",\"f1g1n1\",\"f1g1n2\",\"f1g1n3\",\"f1g1n4\",\"f1g1n4\",\"f1g1n5\",\"f1g1n5\",\"f1g1n6\",\"f1g1n7\",\"f1g1n7\",\"f1g1n7\",\"f1g1n8\",\"f1g1n8\",\"f1g1n8\",\"f1g2n9\",\"f1g2n10\",\"f1g2n10\",\"f1g2n11\",\"f1g2n12\",\"f1g2n13\",\"f1g2n13\",\"f1g2n14\",\"f1g2n14\",\"f1g2n15\",\"f1g2n16\",\"f1g2n16\",\"f1g2n17\",\"f1g2n17\",\"f1g2n18\",\"f1g2n19\",\"f1g2n20\",\"f1g2n20\",\"f1g2n21\",\"f1g2n21\",\"f1g2n22\",\"f1g2n23\",\"f1g2n24\",\"f1g2n25\",\"f1g2n25\",\"\",\"f2g0\",\"f2g0\",\"f2g0\",\"f2g0\",\"f2g0\",\"f2g0\",\"f2g0\",\"f2g0\",\"f2g0\",\"f2g0\",\"f2g0\",\"f2g1n0\",\"f2g1n1\",\"f2g1n2\",\"f2g1n3\",\"f2g1n3\",\"f2g1n4\",\"f2g1n4\",\"f2g1n5\",\"f2g1n6\",\"f2g1n6\",\"f2g1n6\",\"f2g1n7\",\"f2g1n7\",\"f2g1n8\",\"f2g1n8\",\"f2g1n9\",\"f2g1n9\",\"f2g1n10\",\"f2g2n11\",\"f2g2n12\",\"f2g2n13\",\"f2g2n14\",\"f2g2n15\",\"f2g2n16\",\"f2g2n16\",\"f2g2n17\",\"f2g2n17\",\"f2g2n18\",\"f2g2n18\",\"f2g2n19\",\"f2g2n20\",\"f2g2n20\",\"f2g2n21\",\"f2g2n22\",\"f2g2n22\",\"f2g2n23\",\"f2g2n24\",\"f2g2n25\",\"f2g2n25\",\"f2g2n26\",\"f2g2n27\",\"f2g2n28\",\"f2g3n29\",\"f2g3n30\",\"f2g3n31\",\"f2g3n31\",\"f2g3n32\",\"f2g3n32\",\"f2g3n33\",\"f2g3n34\",\"f2g3n34\",\"f2g3n35\",\"f2g3n36\",\"f2g3n37\",\"f2g3n38\",\"f2g3n39\",\"f2g3n39\",\"f2g3n40\",\"f2g3n41\",\"f2g3n41\",\"f2g3n42\",\"f2g3n42\",\"f2g3n43\",\"f2g3n43\",\"f2g3n44\",\"f2g3n44\",\"f2g3n45\",\"f2g3n45\",\"f2g3n46\",\"f2g3n47\",\"f2g3n48\",\"f2g3n49\",\"f2g3n50\",\"f2g3n50\",\"f2g3n51\",\"f2g3n52\",\"f2g3n52\",\"f2g4n53\",\"f2g4n54\",\"f2g4n55\",\"f2g4n56\",\"f2g4n57\",\"f2g4n57\",\"f2g4n58\",\"f2g4n58\",\"f2g4n59\",\"f2g4n59\",\"f2g4n60\",\"f2g4n61\",\"f2g4n62\",\"f2g4n63\",\"f2g4n64\",\"f2g4n64\",\"f2g4n65\",\"f2g4n65\",\"f2g4n66\",\"f2g4n67\",\"f2g4n68\",\"f2g4n68\",\"f2g4n69\",\"f2g4n70\",\"f2g4n70\",\"f2g4n71\",\"f2g4n71\",\"f2g4n72\",\"f2g4n73\",\"f2g4n74\",\"f2g4n75\",\"f2g4n75\",\"f2g4n76\",\"f2g4n76\",\"f2g4n77\",\"f2g4n77\",\"f2g4n78\",\"f2g4n79\",\"f2g4n80\",\"f2g4n81\",\"f2g4n81\",\"f2g4n82\",\"f2g4n83\",\"f2g4n83\",\"f2g4n84\",\"f2g4n84\",\"f2g4n85\",\"f2g4n86\",\"f2g4n87\",\"\",\"f3g0\",\"f3g0\",\"f3g0\",\"f3g0\",\"f3g0\",\"f3g0\",\"f3g0\",\"f3g0\",\"f3g0\",\"f3g1n0\",\"f3g1n0\",\"f3g1n1\",\"f3g1n2\",\"f3g1n3\",\"f3g1n3\",\"f3g1n4\",\"f3g1n4\",\"f3g1n5\",\"f3g1n6\",\"f3g1n6\",\"f3g1n7\",\"f3g1n8\",\"f3g1n8\",\"\",\"f4g0\",\"f4g0\",\"f4g0\",\"f4g0\",\"f4g0\",\"f4g0\",\"f4g0\",\"f4g0\",\"f4g0\",\"f4g0\",\"f4g0\",\"f4g0\",\"f4g0\",\"f4g0\",\"f4g0\",\"f4g1n0\",\"f4g1n1\",\"f4g1n1\",\"f4g1n2\",\"f4g1n2\",\"f4g1n2\",\"f4g1n3\",\"f4g1n4\",\"f4g1n4\",\"f4g1n4\",\"f4g1n5\",\"f4g1n5\",\"f4g1n5\",\"f4g1n6\",\"f4g1n6\",\"f4g1n7\",\"f4g1n7\",\"f4g1n8\",\"f4g1n8\",\"f4g1n8\",\"f4g1n9\",\"f4g1n10\",\"f4g1n10\",\"f4g1n10\",\"f4g1n11\",\"f4g1n12\",\"f4g1n13\",\"f4g1n14\",\"f4g1n14\",\"f4g1n14\",\"f4g2n15\",\"f4g2n16\",\"f4g2n16\",\"f4g2n17\",\"f4g2n17\",\"f4g2n18\",\"f4g2n18\",\"f4g2n19\",\"f4g2n19\",\"f4g2n20\",\"f4g2n20\",\"f4g2n21\",\"f4g2n21\",\"f4g2n22\",\"f4g2n22\",\"f4g2n23\",\"f4g2n24\",\"f4g2n25\",\"f4g2n25\",\"f4g2n26\",\"f4g2n27\",\"f4g2n27\",\"f4g2n28\",\"f4g2n29\",\"f4g2n30\",\"f4g2n31\",\"f4g2n31\",\"f4g2n32\",\"f4g2n32\",\"f4g2n33\",\"f4g2n33\",\"f4g2n34\",\"f4g2n35\",\"f4g2n35\",\"f4g2n36\",\"f4g2n36\",\"f4g2n37\",\"f4g2n38\",\"f4g2n39\",\"f4g2n40\",\"f4g2n40\",\"f4g2n41\",\"f4g2n41\",\"f4g2n42\",\"f4g2n42\",\"f4g2n43\",\"f4g2n44\",\"f4g2n44\",\"f4g3n45\",\"f4g3n45\",\"f4g3n46\",\"f4g3n46\",\"f4g3n47\",\"f4g3n48\",\"f4g3n48\",\"f4g3n49\",\"f4g3n50\",\"f4g3n51\",\"f4g3n51\",\"f4g3n52\",\"f4g3n53\",\"f4g3n53\",\"f4g3n54\",\"f4g3n55\",\"f4g3n56\",\"f4g3n57\",\"f4g3n57\",\"f4g3n58\",\"f4g3n58\",\"f4g3n59\",\"f4g3n60\",\"f4g3n61\",\"f4g3n61\",\"f4g3n62\",\"f4g3n63\",\"f4g3n64\",\"f4g3n64\",\"f4g3n65\",\"f4g3n65\",\"f4g3n66\",\"f4g3n66\",\"f4g3n67\",\"f4g3n68\",\"f4g3n68\",\"f4g3n69\",\"f4g3n69\",\"f4g3n70\",\"f4g3n70\",\"f4g3n71\",\"f4g3n71\",\"f4g3n72\",\"f4g3n72\",\"f4g3n73\",\"f4g3n73\",\"f4g3n74\",\"f4g3n75\",\"f4g3n76\",\"f4g3n76\",\"f4g3n77\",\"f4g3n78\",\"f4g3n78\",\"f4g3n79\",\"f4g3n79\",\"f4g3n80\",\"f4g3n80\",\"f4g3n81\",\"f4g3n82\",\"f4g3n83\",\"f4g3n83\",\"f4g3n84\",\"f4g3n84\",\"f4g3n85\",\"f4g3n86\",\"f4g3n87\",\"f4g3n87\",\"f4g3n88\",\"f4g3n89\",\"f4g3n90\",\"f4g3n91\",\"f4g3n92\",\"f4g3n92\",\"\",\"f5g0\",\"f5g0\",\"f5g0\",\"f5g0\",\"f5g0\",\"f5g0\",\"f5g0\",\"f5g0\",\"f5g0\",\"f5g0\",\"\",\"f6g0\",\"f6g0\",\"f6g0\",\"f6g0\",\"f6g0\",\"f6g0\",\"f6g0\",\"f6g0\",\"f6g0\",\"f6g0\",\"f6g0\",\"f6g0\",\"f6g0\",\"f6g0\",\"f6g0\",\"f6g0\",\"f6g1n0\",\"f6g1n0\",\"f6g1n0\",\"f6g1n1\",\"f6g1n1\",\"f6g1n1\",\"f6g1n2\",\"f6g1n2\",\"f6g1n2\",\"f6g1n3\",\"f6g1n3\",\"f6g1n4\",\"f6g1n5\",\"f6g1n5\",\"f6g1n5\",\"f6g1n6\",\"f6g1n7\",\"f6g1n8\",\"f6g1n8\",\"f6g1n9\",\"f6g1n9\",\"f6g1n10\",\"f6g1n11\",\"f6g1n11\",\"f6g1n11\",\"f6g1n12\",\"f6g1n13\",\"f6g1n13\",\"f6g1n14\",\"f6g1n14\",\"f6g1n14\",\"f6g1n15\",\"f6g1n15\",\"f6g1n15\",\"f6g2n16\",\"f6g2n17\",\"f6g2n18\",\"f6g2n19\",\"f6g2n19\",\"f6g2n20\",\"f6g2n20\",\"f6g2n21\",\"f6g2n22\",\"f6g2n22\",\"f6g2n23\",\"f6g2n24\",\"f6g2n25\",\"f6g2n25\",\"f6g2n26\",\"f6g2n26\",\"f6g2n27\",\"f6g2n28\",\"f6g2n28\",\"f6g2n29\",\"f6g2n29\",\"f6g2n30\",\"f6g2n31\",\"f6g2n32\",\"f6g2n32\",\"f6g2n33\",\"f6g2n33\",\"f6g2n34\",\"f6g2n34\",\"f6g2n35\",\"f6g2n35\",\"f6g2n36\",\"f6g2n37\",\"f6g2n38\",\"f6g2n39\",\"f6g2n40\",\"f6g2n41\",\"f6g2n41\",\"f6g2n42\",\"f6g2n43\",\"f6g2n43\",\"f6g2n44\",\"f6g2n44\",\"f6g2n45\",\"f6g2n45\",\"f6g2n46\",\"f6g2n47\",\"f6g2n47\",\"f6g2n48\",\"f6g2n49\",\"f6g3n50\",\"f6g3n50\",\"f6g3n51\",\"f6g3n51\",\"f6g3n52\",\"f6g3n52\",\"f6g3n53\",\"f6g3n53\",\"f6g3n54\",\"f6g3n54\",\"f6g3n55\",\"f6g3n56\",\"f6g3n57\",\"f6g3n58\",\"f6g3n58\",\"f6g3n59\",\"f6g3n60\",\"f6g3n61\",\"f6g3n61\",\"f6g3n62\",\"f6g3n63\",\"f6g3n63\",\"f6g3n64\",\"f6g3n65\",\"f6g3n65\",\"f6g3n66\",\"f6g3n66\",\"f6g3n67\",\"f6g3n68\",\"f6g3n69\",\"f6g3n69\",\"f6g3n70\",\"f6g3n70\",\"f6g3n71\",\"f6g3n72\",\"f6g3n72\",\"f6g3n73\",\"f6g3n73\",\"f6g3n74\",\"f6g3n74\",\"f6g3n75\",\"f6g3n76\",\"f6g3n76\",\"f6g3n77\",\"f6g3n78\",\"f6g3n78\",\"f6g3n79\",\"f6g3n79\",\"f6g3n80\",\"f6g3n81\",\"f6g3n81\",\"f6g3n82\",\"f6g3n83\",\"f6g3n84\",\"f6g3n84\",\"f6g3n85\",\"f6g3n86\",\"f6g3n87\",\"f6g3n87\",\"f6g3n88\",\"f6g3n89\",\"f6g3n89\",\"f6g3n90\",\"f6g3n91\",\"f6g3n92\",\"f6g3n93\",\"f6g3n93\",\"f6g3n94\",\"f6g3n94\",\"f6g3n95\",\"f6g3n95\",\"f6g3n96\",\"f6g3n96\",\"f6g3n97\",\"f6g3n98\",\"f6g3n98\",\"f6g3n99\",\"f6g3n99\",\"\",\"f7g0\",\"f7g0\",\"f7g0\",\"f7g0\",\"f7g0\",\"f7g0\",\"f7g1n0\",\"f7g1n0\",\"f7g1n1\",\"f7g1n1\",\"f7g1n2\",\"f7g1n2\",\"f7g1n3\",\"f7g1n3\",\"f7g1n4\",\"f7g1n5\",\"\",\"f8g0\",\"f8g0\",\"f8g0\",\"f8g0\",\"f8g0\",\"f8g0\",\"f8g0\",\"f8g0\",\"f8g0\",\"f8g0\",\"f8g0\",\"f8g0\",\"f8g1n0\",\"f8g1n1\",\"f8g1n1\",\"f8g1n2\",\"f8g1n3\",\"f8g1n4\",\"f8g1n5\",\"f8g1n6\",\"f8g1n7\",\"f8g1n8\",\"f8g1n9\",\"f8g1n9\",\"f8g1n10\",\"f8g1n10\",\"f8g1n11\",\"f8g1n11\",\"f8g2n12\",\"f8g2n13\",\"f8g2n13\",\"f8g2n14\",\"f8g2n15\",\"f8g2n15\",\"f8g2n16\",\"f8g2n17\",\"f8g2n17\",\"f8g2n18\",\"f8g2n19\",\"f8g2n19\",\"f8g2n20\",\"f8g2n21\",\"f8g2n21\",\"f8g2n22\",\"f8g2n22\",\"f8g2n23\",\"f8g2n24\",\"f8g2n25\",\"f8g2n26\",\"f8g2n27\"], variant=\"genealogy\", show_values=False, width=900, height=400"
)]
pub fn render(cfg: &DendrogramConfig) -> String {
    render_impl(cfg, PI * 0.9)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let html = render_impl(&c, PI * 0.9);

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
        let html = render_impl(&c, PI * 0.9);
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
        let html = render_impl(&c, PI * 0.9);
        assert!(html.contains("viewBox=\"0 0 600 600\""));
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let labels: Vec<String> = vec![];
        let parents: Vec<String> = vec![];
        let c = cfg(&labels, &parents);
        assert!(render_impl(&c, PI * 0.9).is_empty());
    }

    #[test]
    fn perf_rendering_a_large_multi_family_forest_stays_fast() {
        let (labels, parents) = forest(6, 6, 2);
        let c = cfg(&labels, &parents);
        let start = std::time::Instant::now();
        let html = render_impl(&c, PI * 0.9);
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
        let html = render_impl(&c, PI * 0.9);
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 400, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }

    #[test]
    fn angular_color_sweeps_through_distinct_hues_around_the_circle() {
        let cx = 300.0;
        let cy = 300.0;
        let r_max = 250.0;
        let east = angular_color(cx + 200.0, cy, cx, cy, r_max, 0.5);
        let south = angular_color(cx, cy + 200.0, cx, cy, r_max, 0.5);
        let west = angular_color(cx - 200.0, cy, cx, cy, r_max, 0.5);
        let mut distinct = std::collections::HashSet::new();
        distinct.insert(east);
        distinct.insert(south);
        distinct.insert(west);
        assert_eq!(distinct.len(), 3, "opposite angles should map to visibly different hues");
    }

    #[test]
    fn angular_color_at_the_shared_center_stays_neutral() {
        let c = angular_color(300.0, 300.0, 300.0, 300.0, 250.0, 0.5);
        assert_eq!(c, 0x334155);
    }

    #[test]
    fn angular_color_grows_more_saturated_with_more_influence() {
        let cx = 300.0;
        let cy = 300.0;
        let r_max = 250.0;
        let low = angular_color(cx + 200.0, cy, cx, cy, r_max, 0.0);
        let high = angular_color(cx + 200.0, cy, cx, cy, r_max, 1.0);
        assert_ne!(low, high, "influence should visibly change the color, not just the hue");
    }
}
