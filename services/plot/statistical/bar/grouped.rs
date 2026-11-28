use super::block3d::Bar3DBlock;
use super::config::BarConfig;
use crate::plot::statistical::{render_grouped_bar_html, GroupedBarConfig};

pub fn layout_3d(cfg: &BarConfig, stacked: bool) -> Vec<Bar3DBlock> {
    let n_cats = cfg.category_labels.len();
    let n_ser = cfg.series.len();
    if n_cats == 0 || n_ser == 0 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(n_cats * n_ser);
    for ci in 0..n_cats {
        if stacked {
            let mut acc = 0.0;
            for (si, (_, vals)) in cfg.series.iter().enumerate() {
                let v = vals.get(ci).copied().unwrap_or(0.0);
                if !v.is_finite() {
                    continue;
                }
                out.push(Bar3DBlock::new(ci as f64, 0.0, acc, acc + v, 0.32, 0.32, si));
                acc += v;
            }
        } else {
            for (si, (_, vals)) in cfg.series.iter().enumerate() {
                let v = vals.get(ci).copied().unwrap_or(0.0);
                if !v.is_finite() {
                    continue;
                }
                let cy = si as f64 - (n_ser as f64 - 1.0) / 2.0;
                out.push(Bar3DBlock::new(ci as f64, cy, 0.0, v, 0.32, 0.32, si));
            }
        }
    }
    out
}

#[crate::chart_demo("labels=[\"Q1\",\"Q2\",\"Q3\",\"Q4\"], series=[[24,38,17,42],[18,29,33,21],[12,15,28,30]], series_names=[\"Product A\",\"Product B\",\"Product C\"]")]

pub fn render(cfg: &BarConfig, stacked: bool) -> String {
    let g = GroupedBarConfig {
        title: cfg.title,
        x_label: cfg.x_label,
        y_label: cfg.y_label,
        gridlines: cfg.gridlines,
        sort_order: cfg.sort_order,
        hover: cfg.hover,
        legend_position: cfg.legend_position,
        width: cfg.width,
        height: cfg.height,
        category_labels: cfg.category_labels,
        series: cfg.series,
        palette: cfg.palette,
        stacked,
        show_values: cfg.show_text,
        value_min_height: 16,
        orientation: cfg.orientation,
    };
    render_grouped_bar_html(&g)
}
