use super::common::{axis_labels, cell_border, cell_point_px, diagonal_label, finalize, open_svg, prepare};
use super::config::SplomConfig;
use crate::plot::statistical::common::{hex6, palette_color, push_b, push_f2};

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
    let line_col = hex6(palette_color(cfg.palette, 1));
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
                push_b(&mut b, b"\" fill-opacity=\"0.5\"/>");
            }
            if let Some((slope, intercept)) = least_squares(&p.cols[c], &p.cols[r]) {
                let x0 = p.mins[c];
                let x1 = p.maxs[c];
                let (lx0, ly0) = cell_point_px(&p, r, c, x0, slope * x0 + intercept);
                let (lx1, ly1) = cell_point_px(&p, r, c, x1, slope * x1 + intercept);
                push_b(&mut b, b"<line x1=\"");
                push_f2(&mut b, lx0);
                push_b(&mut b, b"\" y1=\"");
                push_f2(&mut b, ly0);
                push_b(&mut b, b"\" x2=\"");
                push_f2(&mut b, lx1);
                push_b(&mut b, b"\" y2=\"");
                push_f2(&mut b, ly1);
                push_b(&mut b, b"\" stroke=\"#");
                b.extend_from_slice(&line_col);
                push_b(&mut b, b"\" stroke-width=\"1.6\" stroke-opacity=\"0.85\"/>");
            }
        }
    }
    axis_labels(&mut b, &p, cfg);
    finalize(b, cfg)
}

fn least_squares(xs: &[f64], ys: &[f64]) -> Option<(f64, f64)> {
    let n = xs.len().min(ys.len());
    if n < 2 {
        return None;
    }
    let xbar = xs[..n].iter().sum::<f64>() / n as f64;
    let ybar = ys[..n].iter().sum::<f64>() / n as f64;
    let mut num = 0.0;
    let mut den = 0.0;
    for i in 0..n {
        let dx = xs[i] - xbar;
        num += dx * (ys[i] - ybar);
        den += dx * dx;
    }
    if den.abs() < 1e-12 {
        return None;
    }
    let slope = num / den;
    let intercept = ybar - slope * xbar;
    Some((slope, intercept))
}
