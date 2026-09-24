use super::config::WordCloudConfig;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::generic::spiral_columns;
use crate::plot::statistical::_3d::lineage::{markers, weighted_paths, Point};
use crate::plot::statistical::bar::Bar3DBlock;

const HW: f64 = 0.34;
const NODE_HW: f64 = 0.2;
const MIN_EDGE: f64 = 0.02;
const MAX_EDGE: f64 = 0.1;

fn positioned(cfg: &WordCloudConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.words.len().min(cfg.points_x.len()).min(cfg.points_y.len());
    let peak = cfg.frequencies[..cfg.frequencies.len().min(n)].iter().copied().fold(0.0f64, f64::max).max(1e-9);
    let positions: Vec<Point> = (0..n).map(|i| (cfg.points_x[i], cfg.points_y[i], 0.0)).collect();
    let clusters = cfg.point_clusters;
    let mut blocks = markers(&positions, NODE_HW, |i| clusters.get(i).map(|&c| c.max(0) as usize).unwrap_or(0), |i| {
        (cfg.frequencies.get(i).copied().unwrap_or(0.0) / peak).clamp(0.0, 1.0)
    });

    let e = cfg.edges_i.len().min(cfg.edges_j.len());
    let edges: Vec<(Vec<Point>, f64, usize)> = (0..e)
        .filter_map(|k| {
            let s = cfg.edges_i[k] as usize;
            let t = cfg.edges_j[k] as usize;
            if s >= n || t >= n || s == t {
                return None;
            }
            let w = cfg.edges_w.get(k).copied().unwrap_or(1.0).max(0.0);
            Some((vec![positions[s], positions[t]], w, clusters.get(s).map(|&c| c.max(0) as usize).unwrap_or(0)))
        })
        .collect();
    if !edges.is_empty() {
        let links: Vec<Vec<Point>> = edges.iter().map(|(p, _, _)| p.clone()).collect();
        let weights: Vec<f64> = edges.iter().map(|(_, w, _)| *w).collect();
        let classes: Vec<usize> = edges.iter().map(|(_, _, c)| *c).collect();
        let peak_edge = weights.iter().copied().fold(1e-12, f64::max);
        blocks.extend(weighted_paths(&links, &weights, MIN_EDGE, MAX_EDGE, 3, |li| classes[li], |li| weights[li] / peak_edge));
    }
    (blocks, cfg.words[..n].to_vec())
}

fn spiraled(cfg: &WordCloudConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.words.len().min(cfg.frequencies.len());
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let mut blocks = spiral_columns(&cfg.frequencies[..n], HW, HW);
    let peak = cfg.frequencies[..n].iter().copied().fold(0.0f64, f64::max).max(1e-9);
    for (i, b) in blocks.iter_mut().enumerate() {
        b.tone = Some((cfg.frequencies[i] / peak).clamp(0.0, 1.0));
    }
    (blocks, cfg.words[..n].to_vec())
}

fn wordcloud_3d(cfg: &WordCloudConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    if cfg.points_x.len() >= 2 && cfg.points_x.len() == cfg.points_y.len() {
        positioned(cfg)
    } else {
        spiraled(cfg)
    }
}

pub fn layout_3d(cfg: &WordCloudConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &WordCloudConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    wordcloud_3d(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::variant::WordCloudVariant;

    fn words() -> (Vec<String>, Vec<f64>) {
        (
            ["rust", "python", "wasm", "plot", "data", "viz", "chart", "graph"].iter().map(|s| s.to_string()).collect(),
            vec![42.0, 38.0, 30.0, 28.0, 25.0, 22.0, 18.0, 15.0],
        )
    }

    fn draw(variant: WordCloudVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (words, frequencies) = words();
        let cfg = WordCloudConfig { variant, words: &words, frequencies: &frequencies, ..WordCloudConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_one_column_per_word_and_names_every_word() {
        for &variant in WordCloudVariant::all() {
            let (blocks, names) = draw(variant);
            assert_eq!(blocks.len(), 8, "{variant:?}");
            assert_eq!(names.len(), 8, "{variant:?}");
        }
    }

    #[test]
    fn a_more_frequent_word_stands_taller() {
        let (blocks, names) = draw(WordCloudVariant::Basic);
        let rust = blocks[names.iter().position(|n| n == "rust").unwrap()];
        let graph = blocks[names.iter().position(|n| n == "graph").unwrap()];
        assert!(rust.z1 > graph.z1);
    }

    #[test]
    fn supplying_real_positions_and_edges_switches_to_a_node_link_layout() {
        let (words, frequencies) = words();
        let points_x = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        let points_y = vec![0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0];
        let edges_i = vec![0, 1, 2];
        let edges_j = vec![1, 2, 3];
        let edges_w = vec![2.0, 1.0, 3.0];
        let cfg = WordCloudConfig {
            words: &words,
            frequencies: &frequencies,
            points_x: &points_x,
            points_y: &points_y,
            edges_i: &edges_i,
            edges_j: &edges_j,
            edges_w: &edges_w,
            ..WordCloudConfig::default()
        };
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert_eq!(names.len(), 8);
        assert!(blocks.len() > 8);
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = layout_named(&WordCloudConfig::default(), &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }
}
