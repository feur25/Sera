use super::config::BarConfig;
use crate::plot::statistical::_3d::budget::{Budget, Buckets};

pub struct Reduced {
    pub labels: Vec<String>,
    pub values: Vec<f64>,
    pub category_labels: Vec<String>,
    pub series: Vec<(String, Vec<f64>)>,
    pub color_groups: Vec<String>,
    pub widths: Vec<f64>,
    pub super_categories: Vec<String>,
}

fn categories(cfg: &BarConfig) -> usize {
    [cfg.labels.len(), cfg.values.len(), cfg.category_labels.len()]
        .into_iter()
        .chain(cfg.series.iter().map(|(_, v)| v.len()))
        .max()
        .unwrap_or(0)
}

pub fn reduced(cfg: &BarConfig, budget: &Budget) -> Option<Reduced> {
    let n = categories(cfg);
    let buckets = Buckets::new(n, budget.points);
    if buckets.is_identity() {
        return None;
    }
    Some(Reduced {
        labels: buckets.first(cfg.labels),
        values: buckets.mean(cfg.values),
        category_labels: buckets.first(cfg.category_labels),
        series: cfg.series.iter().map(|(name, v)| (name.clone(), buckets.mean(v))).collect(),
        color_groups: buckets.first(cfg.color_groups),
        widths: if cfg.widths.len() == n { buckets.mean(cfg.widths) } else { cfg.widths.to_vec() },
        super_categories: buckets.first(cfg.super_categories),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn long(n: usize) -> (Vec<String>, Vec<f64>) {
        ((0..n).map(|i| format!("L{i}")).collect(), (0..n).map(|i| i as f64).collect())
    }

    #[test]
    fn short_inputs_are_left_alone() {
        let (labels, values) = long(100);
        let cfg = BarConfig { labels: &labels, values: &values, ..BarConfig::default() };
        assert!(reduced(&cfg, &Budget::default()).is_none());
    }

    #[test]
    fn long_inputs_are_pooled_to_the_budget_with_matching_labels_series_and_groups() {
        let (labels, values) = long(10_000);
        let groups: Vec<String> = (0..10_000).map(|i| format!("G{}", i / 1000)).collect();
        let series = vec![("a".to_string(), values.clone()), ("b".to_string(), values.clone())];
        let cfg = BarConfig {
            labels: &labels,
            values: &values,
            category_labels: &labels,
            series: &series,
            color_groups: &groups,
            ..BarConfig::default()
        };
        let r = reduced(&cfg, &Budget::new(Some(100))).expect("pooled");
        assert_eq!((r.labels.len(), r.values.len(), r.category_labels.len(), r.color_groups.len()), (100, 100, 100, 100));
        assert!(r.series.iter().all(|(_, v)| v.len() == 100));
        assert_eq!(r.labels[1], "L100");
        assert_eq!(r.values[0], 49.5);
    }
}
