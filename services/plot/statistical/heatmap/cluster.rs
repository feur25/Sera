use super::common::{clone_cfg, hierarchical_dendrogram, render_core, Dendrogram};
use super::config::HeatmapConfig;
use crate::plot::statistical::common::{push_b, push_f2};

const EXTRA_PAD_LEFT: i32 = 56;
const EXTRA_PAD_TOP: i32 = 52;
const CHAR_W: f64 = 5.3;
const GAP: f64 = 6.0;

#[crate::chart_demo("labels=[\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\"], col_labels=[\"8h\",\"12h\",\"16h\",\"20h\"], values=[5,9,7,3,6,12,10,4,8,15,13,7,4,8,11,5,3,7,9,2]")]

pub fn render(cfg: &HeatmapConfig) -> String {
    let n_rows = cfg.row_labels.len();
    let cols_lbl = if cfg.col_labels.is_empty() {
        cfg.row_labels
    } else {
        cfg.col_labels
    };
    let n_cols = cols_lbl.len();
    if n_rows == 0 || n_cols == 0 || cfg.flat_matrix.len() < n_rows * n_cols {
        return String::new();
    }
    let row_vectors: Vec<Vec<f64>> = (0..n_rows)
        .map(|r| {
            (0..n_cols)
                .map(|c| {
                    let v = cfg.flat_matrix[r * n_cols + c];
                    if v.is_finite() {
                        v
                    } else {
                        0.0
                    }
                })
                .collect()
        })
        .collect();
    let col_vectors: Vec<Vec<f64>> = (0..n_cols)
        .map(|c| {
            (0..n_rows)
                .map(|r| {
                    let v = cfg.flat_matrix[r * n_cols + c];
                    if v.is_finite() {
                        v
                    } else {
                        0.0
                    }
                })
                .collect()
        })
        .collect();
    let row_dendro = hierarchical_dendrogram(&row_vectors);
    let col_dendro = hierarchical_dendrogram(&col_vectors);
    let row_order = &row_dendro.order;
    let col_order = &col_dendro.order;
    let new_rows: Vec<String> = row_order.iter().map(|&i| cfg.row_labels[i].clone()).collect();
    let new_cols: Vec<String> = col_order.iter().map(|&i| cols_lbl[i].clone()).collect();
    let mut new_mat = vec![0.0f64; n_rows * n_cols];
    for (nr, &orig_r) in row_order.iter().enumerate() {
        for (nc, &orig_c) in col_order.iter().enumerate() {
            new_mat[nr * n_cols + nc] = cfg.flat_matrix[orig_r * n_cols + orig_c];
        }
    }
    let c = HeatmapConfig {
        row_labels: &new_rows,
        col_labels: &new_cols,
        flat_matrix: &new_mat,
        smooth: true,
        cluster_mode: true,
        extra_pad_left: EXTRA_PAD_LEFT,
        extra_pad_top: EXTRA_PAD_TOP,
        ..clone_cfg(cfg)
    };
    let html = render_core(&c);
    if html.is_empty() {
        return html;
    }

    let pad_left = 100 + EXTRA_PAD_LEFT;
    let pad_top = 88 + EXTRA_PAD_TOP;
    let right_bar = !c.categorical && c.colorbar_position.eq_ignore_ascii_case("right");
    let pad_right: i32 = if right_bar { 90 } else { 24 };
    let plot_w = (c.width - pad_left - pad_right).max(40);
    let cell_w_uni = (plot_w / n_cols as i32).max(4);
    let svg_h = if c.height > 0 {
        c.height
    } else {
        pad_top + cell_w_uni * n_rows as i32 + 52
    };
    let plot_h = (svg_h - pad_top - 52).max(40);
    let cell_h_uni = (plot_h / n_rows as i32).max(4);

    let mut b = Vec::<u8>::with_capacity(
        (row_dendro.merges.len() + col_dendro.merges.len()) * 200 + 256,
    );
    draw_row_dendrogram(&mut b, &row_dendro, &new_rows, pad_left, pad_top, cell_h_uni);
    draw_col_dendrogram(
        &mut b,
        &col_dendro,
        &new_cols,
        pad_left,
        pad_top,
        cell_w_uni,
        c.col_label_angle,
    );
    let dendro_svg = unsafe { String::from_utf8_unchecked(b) };
    html.replacen("</svg>", &format!("{}</svg>", dendro_svg), 1)
}

fn label_px_w(label: &str) -> f64 {
    label.chars().count().min(14) as f64 * CHAR_W
}

fn draw_row_dendrogram(
    buf: &mut Vec<u8>,
    d: &Dendrogram,
    labels: &[String],
    pad_left: i32,
    pad_top: i32,
    cell_h: i32,
) {
    let root_x = 12.0;
    let leaf_ref_x = pad_left as f64 - 5.0 - GAP;
    let mh = d.max_height;
    let x_of = |h: f64| leaf_ref_x - (h / mh) * (leaf_ref_x - root_x);
    let y_of = |pos: f64| pad_top as f64 + (pos + 0.5) * cell_h as f64;
    let leaf_tip_x = |pos: f64| -> f64 {
        let lbl = labels.get(pos.round() as usize).map(String::as_str).unwrap_or("");
        (pad_left as f64 - 5.0 - label_px_w(lbl) - GAP).max(root_x)
    };
    for m in &d.merges {
        let yl = y_of(m.xl);
        let yr = y_of(m.xr);
        let xlh = if m.hl <= 1e-9 { leaf_tip_x(m.xl) } else { x_of(m.hl) };
        let xrh = if m.hr <= 1e-9 { leaf_tip_x(m.xr) } else { x_of(m.hr) };
        let xm = x_of(m.h);
        hline(buf, xlh, xm, yl);
        hline(buf, xrh, xm, yr);
        vline(buf, xm, yl, yr);
    }
}

fn draw_col_dendrogram(
    buf: &mut Vec<u8>,
    d: &Dendrogram,
    labels: &[String],
    pad_left: i32,
    pad_top: i32,
    cell_w: i32,
    angle_deg: i32,
) {
    let root_y = 26.0;
    let leaf_ref_y = pad_top as f64 - 8.0 - GAP;
    let mh = d.max_height;
    let angle_rad = (angle_deg as f64).to_radians();
    let sin_a = angle_rad.sin().abs().max(0.05);
    let y_of = |h: f64| leaf_ref_y - (h / mh) * (leaf_ref_y - root_y);
    let x_of = |pos: f64| pad_left as f64 + (pos + 0.5) * cell_w as f64;
    let leaf_tip_y = |pos: f64| -> f64 {
        let lbl = labels.get(pos.round() as usize).map(String::as_str).unwrap_or("");
        (pad_top as f64 - 8.0 - label_px_w(lbl) * sin_a - GAP).max(root_y)
    };
    for m in &d.merges {
        let xl = x_of(m.xl);
        let xr = x_of(m.xr);
        let ylh = if m.hl <= 1e-9 { leaf_tip_y(m.xl) } else { y_of(m.hl) };
        let yrh = if m.hr <= 1e-9 { leaf_tip_y(m.xr) } else { y_of(m.hr) };
        let ym = y_of(m.h);
        vline(buf, xl, ylh, ym);
        vline(buf, xr, yrh, ym);
        hline(buf, xl, xr, ym);
    }
}

fn hline(buf: &mut Vec<u8>, x1: f64, x2: f64, y: f64) {
    push_b(buf, b"<line x1=\"");
    push_f2(buf, x1);
    push_b(buf, b"\" y1=\"");
    push_f2(buf, y);
    push_b(buf, b"\" x2=\"");
    push_f2(buf, x2);
    push_b(buf, b"\" y2=\"");
    push_f2(buf, y);
    push_b(buf, b"\" stroke=\"#94a3b8\" stroke-width=\"1.2\" fill=\"none\"/>");
}

fn vline(buf: &mut Vec<u8>, x: f64, y1: f64, y2: f64) {
    push_b(buf, b"<line x1=\"");
    push_f2(buf, x);
    push_b(buf, b"\" y1=\"");
    push_f2(buf, y1);
    push_b(buf, b"\" x2=\"");
    push_f2(buf, x);
    push_b(buf, b"\" y2=\"");
    push_f2(buf, y2);
    push_b(buf, b"\" stroke=\"#94a3b8\" stroke-width=\"1.2\" fill=\"none\"/>");
}
