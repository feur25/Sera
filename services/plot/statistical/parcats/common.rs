use super::config::ParcatsConfig;
use std::collections::HashMap;

pub struct Prepared {
    pub labels: Vec<String>,
    pub sources: Vec<i32>,
    pub targets: Vec<i32>,
    pub weights: Vec<f64>,
}

pub fn prepare(cfg: &ParcatsConfig) -> Option<Prepared> {
    let m = cfg.axes.len();
    let n = cfg.category_series.len();
    if m < 2 || n == 0 {
        return None;
    }

    let mut node_index: HashMap<(usize, String), usize> = HashMap::new();
    let mut labels: Vec<String> = Vec::new();
    for axis in 0..m {
        for row in cfg.category_series {
            if row.len() <= axis {
                continue;
            }
            let cat = row[axis].clone();
            node_index.entry((axis, cat.clone())).or_insert_with(|| {
                labels.push(cat);
                labels.len() - 1
            });
        }
    }
    if labels.is_empty() {
        return None;
    }

    let mut edge_weights: HashMap<(usize, usize), f64> = HashMap::new();
    for row in cfg.category_series {
        if row.len() < m {
            continue;
        }
        for axis in 0..m - 1 {
            let s = match node_index.get(&(axis, row[axis].clone())) {
                Some(&v) => v,
                None => continue,
            };
            let t = match node_index.get(&(axis + 1, row[axis + 1].clone())) {
                Some(&v) => v,
                None => continue,
            };
            *edge_weights.entry((s, t)).or_insert(0.0) += 1.0;
        }
    }
    if edge_weights.is_empty() {
        return None;
    }

    let mut pairs: Vec<((usize, usize), f64)> = edge_weights.into_iter().collect();
    pairs.sort_by_key(|&((s, t), _)| (s, t));

    let mut sources = Vec::with_capacity(pairs.len());
    let mut targets = Vec::with_capacity(pairs.len());
    let mut weights = Vec::with_capacity(pairs.len());
    for ((s, t), w) in pairs {
        sources.push(s as i32);
        targets.push(t as i32);
        weights.push(w);
    }

    Some(Prepared {
        labels,
        sources,
        targets,
        weights,
    })
}
