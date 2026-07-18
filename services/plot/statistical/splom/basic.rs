use super::common::{axis_labels, cell_border, cell_point_px, diagonal_label, finalize, open_svg, prepare};
use super::config::SplomConfig;
use crate::plot::statistical::common::{palette_color, push_b, push_f2, hex6};

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
    let col = hex6(palette_color(cfg.palette, 0));
    for r in 0..p.m {
        for c in 0..p.m {
            if r == c {
                diagonal_label(&mut b, &p, r, &cfg.axes[r]);
                continue;
            }
            cell_border(&mut b, &p, r, c);
            for i in 0..p.n {
                let (px, py) = cell_point_px(&p, r, c, p.cols[c][i], p.cols[r][i]);
                push_b(&mut b, b"<circle cx=\"");
                push_f2(&mut b, px);
                push_b(&mut b, b"\" cy=\"");
                push_f2(&mut b, py);
                push_b(&mut b, b"\" r=\"");
                push_f2(&mut b, cfg.point_size);
                push_b(&mut b, b"\" fill=\"#");
                b.extend_from_slice(&col);
                push_b(&mut b, b"\" fill-opacity=\"0.55\"/>");
            }
        }
    }
    axis_labels(&mut b, &p, cfg);
    finalize(b, cfg)
}
