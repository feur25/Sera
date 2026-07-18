use super::common::{axis_labels, cell_point_px, diagonal_label, finalize, open_svg, pearson, prepare};
use super::config::SplomConfig;
use crate::plot::statistical::common::{push_b, push_f2, push_i, hex6};
use crate::plot::statistical::heatmap::common::colorscale_color;

#[crate::chart_demo(
    "axes=[\"Speed\",\"Power\",\"Range\"], series=[[80,65,70],[60,80,55],[40,70,90],[90,40,60],[55,85,45],[70,55,80]]"
)]

pub fn render(cfg: &SplomConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut b = Vec::<u8>::with_capacity(p.m * p.m * (p.n * 24 + 256) + 4096);
    open_svg(&mut b, cfg);
    let scale = if cfg.colorscale.is_empty() { "rdbu" } else { cfg.colorscale };
    for r in 0..p.m {
        for c in 0..p.m {
            if r == c {
                diagonal_label(&mut b, &p, r, &cfg.axes[r]);
                continue;
            }
            let corr = pearson(&p.cols[c], &p.cols[r]);
            let t = (corr + 1.0) / 2.0;
            let bg = colorscale_color(scale, t);
            push_b(&mut b, b"<rect data-idx=\"");
            push_i(&mut b, (r * p.m + c) as i32);
            push_b(&mut b, b"\" data-y=\"");
            push_f2(&mut b, corr);
            push_b(&mut b, b"\" x=\"");
            push_f2(&mut b, p.pad_l + c as f64 * p.cell_w);
            push_b(&mut b, b"\" y=\"");
            push_f2(&mut b, p.pad_t + r as f64 * p.cell_h);
            push_b(&mut b, b"\" width=\"");
            push_f2(&mut b, p.cell_w);
            push_b(&mut b, b"\" height=\"");
            push_f2(&mut b, p.cell_h);
            push_b(&mut b, b"\" fill=\"#");
            b.extend_from_slice(&hex6(bg));
            push_b(&mut b, b"\" fill-opacity=\"0.85\" stroke=\"#e2e8f0\" stroke-width=\"0.6\"/>");
            for i in 0..p.n {
                let (px, py) = cell_point_px(&p, r, c, p.cols[c][i], p.cols[r][i]);
                push_b(&mut b, b"<circle cx=\"");
                push_f2(&mut b, px);
                push_b(&mut b, b"\" cy=\"");
                push_f2(&mut b, py);
                push_b(&mut b, b"\" r=\"");
                push_f2(&mut b, cfg.point_size * 0.85);
                push_b(&mut b, b"\" fill=\"#1f2937\" fill-opacity=\"0.55\"/>");
            }
        }
    }
    axis_labels(&mut b, &p, cfg);
    finalize(b, cfg)
}
