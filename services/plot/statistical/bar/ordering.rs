use super::config::BarConfig;
use crate::plot::statistical::common::{apply_sort, apply_sort_groups};

pub struct Ordered {
    pub labels: Vec<String>,
    pub values: Vec<f64>,
    pub groups: Vec<String>,
}

pub fn is_reordered(cfg: &BarConfig) -> bool {
    !cfg.values.is_empty()
        && cfg.series.is_empty()
        && !cfg.sort_order.is_empty()
        && cfg.sort_order != "none"
}

pub fn ordered(cfg: &BarConfig) -> Ordered {
    let n = cfg.labels.len().min(cfg.values.len());
    if !cfg.color_groups.is_empty() && cfg.color_groups.len() >= n {
        let (labels, values, groups) = apply_sort_groups(cfg.labels, cfg.values, cfg.color_groups, cfg.sort_order);
        return Ordered { labels, values, groups };
    }
    let (labels, values) = apply_sort(cfg.labels, cfg.values, cfg.sort_order);
    Ordered { labels, values, groups: Vec::new() }
}
