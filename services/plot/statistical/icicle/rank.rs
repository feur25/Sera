use super::common::{finalize, label_in_rect, node_data_attrs, node_rect, open_svg, prepare};
use super::config::IcicleConfig;
use crate::plot::statistical::common::{heat_color, hex6, push_b};

#[crate::chart_demo("labels=[\"Root\",\"A\",\"B\",\"A1\",\"A2\",\"B1\",\"B2\"], parents=[\"\",\"Root\",\"Root\",\"A\",\"A\",\"B\",\"B\"], values=[0,40,30,20,20,15,15]")]

pub fn render(cfg: &IcicleConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut b = Vec::<u8>::with_capacity(p.n * 240 + 1536);
    open_svg(&mut b, cfg);

    let max_depth = p.depth.iter().copied().max().unwrap_or(0);
    let mut rank_frac = vec![0.0f64; p.n];
    for d in 0..=max_depth {
        let mut idxs: Vec<usize> = (0..p.n).filter(|&i| p.depth[i] == d).collect();
        idxs.sort_by(|&a, &b| {
            p.values_eff[a]
                .partial_cmp(&p.values_eff[b])
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let m = idxs.len();
        for (rank, &i) in idxs.iter().enumerate() {
            rank_frac[i] = if m > 1 {
                rank as f64 / (m as f64 - 1.0)
            } else {
                1.0
            };
        }
    }

    let order = p.bfs_order.clone();
    for i in order {
        let r = node_rect(&p, i);
        if r.w < 0.5 {
            continue;
        }
        let hx = hex6(heat_color(rank_frac[i]));
        push_b(&mut b, b"<rect");
        node_data_attrs(&mut b, &p, i);
        super::common::rect_attrs(&mut b, r);
        push_b(&mut b, b" fill=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke=\"#fff\" stroke-width=\"1\"/>");
        label_in_rect(&mut b, &p, i, r, true);
    }
    finalize(b, cfg)
}
