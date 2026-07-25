use super::common::{clone_cfg, hierarchical_leaf_order, render_core};
use super::config::HeatmapConfig;

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
    let row_order = hierarchical_leaf_order(&row_vectors);
    let col_order = hierarchical_leaf_order(&col_vectors);
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
        ..clone_cfg(cfg)
    };
    render_core(&c)
}
