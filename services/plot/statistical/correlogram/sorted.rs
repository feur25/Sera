use super::common::render_impl;
use super::config::CorrelogramConfig;

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\",\"D\"], matrix=[[1,0.8,-0.3,0.5],[0.8,1,0.1,-0.2],[-0.3,0.1,1,0.7],[0.5,-0.2,0.7,1]], variant=\"sorted\"")]
pub fn render(cfg: &CorrelogramConfig) -> String {
    let n = cfg.labels.len();
    if n == 0 || cfg.matrix.len() < n * n {
        return String::new();
    }
    let mut scores: Vec<(usize, f64)> = (0..n)
        .map(|i| {
            let s: f64 = (0..n).filter(|&j| j != i).map(|j| cfg.matrix[i * n + j].abs()).sum();
            (i, s)
        })
        .collect();
    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let perm: Vec<usize> = scores.into_iter().map(|(i, _)| i).collect();

    let sorted_labels: Vec<String> = perm.iter().map(|&i| cfg.labels[i].clone()).collect();
    let mut sorted_matrix: Vec<f64> = vec![0.0; n * n];
    for (r, &pr) in perm.iter().enumerate() {
        for (c, &pc) in perm.iter().enumerate() {
            sorted_matrix[r * n + c] = cfg.matrix[pr * n + pc];
        }
    }

    let sorted_cfg = CorrelogramConfig {
        title: cfg.title,
        x_label: cfg.x_label,
        y_label: cfg.y_label,
        gridlines: cfg.gridlines,
        sort_order: cfg.sort_order,
        hover: cfg.hover,
        legend_position: cfg.legend_position,
        width: cfg.width,
        height: cfg.height,
        variant: cfg.variant,
        labels: &sorted_labels,
        matrix: &sorted_matrix,
        palette: cfg.palette,
        show_values: cfg.show_values,
    };
    render_impl(&sorted_cfg, false, false, false)
}
