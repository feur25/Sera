use super::config::WordCloudConfig;
use super::variant::WordCloudVariant;
use crate::plot::statistical::_3d::budget::{even_indices, pick, Budget, SAMPLE_CAP};
use crate::plot::statistical::_3d::generic::{packed_columns, phyllotaxis_columns, spiral_columns};
use crate::plot::statistical::_3d::lineage::{markers, weighted_paths, Point};
use crate::plot::statistical::bar::Bar3DBlock;

const HW: f64 = 0.34;
const NODE_HW: f64 = 0.2;
const MIN_EDGE: f64 = 0.02;
const MAX_EDGE: f64 = 0.1;
const SIZE_MIN: f64 = 0.16;
const SIZE_MAX: f64 = 0.6;

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

fn spiraled(cfg: &WordCloudConfig, budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n_all = cfg.words.len().min(cfg.frequencies.len());
    if n_all == 0 {
        return (Vec::new(), Vec::new());
    }
    let cap = budget.points.min(SAMPLE_CAP);
    let keep = even_indices(n_all, cap);
    let words = pick(&cfg.words[..n_all], &keep);
    let freqs = pick(&cfg.frequencies[..n_all], &keep);
    let n = freqs.len();
    let (lo, hi) = freqs.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| (lo.min(v), hi.max(v)));
    let range = (hi - lo).max(1e-9);
    let sizes: Vec<f64> = freqs.iter().map(|&v| SIZE_MIN + ((v - lo) / range).clamp(0.0, 1.0) * (SIZE_MAX - SIZE_MIN)).collect();
    let packed = matches!(cfg.variant, WordCloudVariant::Bubble);
    let mut blocks = match cfg.variant {
        WordCloudVariant::Bubble => packed_columns(&sizes, &freqs),
        WordCloudVariant::Cosmos | WordCloudVariant::Network | WordCloudVariant::Context | WordCloudVariant::Neuron => phyllotaxis_columns(&freqs, HW, HW),
        _ => spiral_columns(&freqs, HW, HW),
    };
    for (i, b) in blocks.iter_mut().enumerate().take(n) {
        let frac = ((freqs[i] - lo) / range).clamp(0.0, 1.0);
        if !packed {
            b.hw = sizes[i];
            b.hd = sizes[i];
        }
        b.tone = Some(frac);
    }
    (blocks, words)
}

fn wordcloud_3d(cfg: &WordCloudConfig, budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    if cfg.points_x.len() >= 2 && cfg.points_x.len() == cfg.points_y.len() {
        positioned(cfg)
    } else {
        spiraled(cfg, budget)
    }
}

pub fn layout_3d(cfg: &WordCloudConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

pub fn layout_named(cfg: &WordCloudConfig, budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    wordcloud_3d(cfg, budget)
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
    fn a_more_frequent_word_gets_a_bigger_marker() {
        let (blocks, names) = draw(WordCloudVariant::Basic);
        let rust = blocks[names.iter().position(|n| n == "rust").unwrap()];
        let graph = blocks[names.iter().position(|n| n == "graph").unwrap()];
        assert!(rust.hw > graph.hw);
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

    #[test]
    fn bubble_packing_stays_fast_even_with_thousands_of_raw_words() {
        let n = 6000;
        let words: Vec<String> = (0..n).map(|i| format!("w{i}")).collect();
        let frequencies: Vec<f64> = (0..n).map(|i| 1.0 + (i % 97) as f64).collect();
        let cfg = WordCloudConfig { variant: WordCloudVariant::Bubble, words: &words, frequencies: &frequencies, ..WordCloudConfig::default() };
        let started = std::time::Instant::now();
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert!(started.elapsed().as_secs() < 5, "bubble packing took {:?} on {n} raw words", started.elapsed());
        assert!(blocks.len() <= SAMPLE_CAP);
        assert_eq!(blocks.len(), names.len());
    }

    #[test]
    fn bubble_packs_words_into_non_overlapping_circles() {
        let (blocks, _) = draw(WordCloudVariant::Bubble);
        for i in 0..blocks.len() {
            for j in (i + 1)..blocks.len() {
                let d = (blocks[i].cx - blocks[j].cx).hypot(blocks[i].cy - blocks[j].cy);
                assert!(d >= blocks[i].hw + blocks[j].hw - 1e-6, "bubble {i} and {j} overlap");
            }
        }
    }

    #[test]
    fn cosmos_uses_a_genuinely_different_placement_than_basic() {
        let (basic, _) = draw(WordCloudVariant::Basic);
        let (cosmos, _) = draw(WordCloudVariant::Cosmos);
        let differing = basic.iter().zip(cosmos.iter()).filter(|(a, b)| (a.cx - b.cx).abs() > 1e-6 || (a.cy - b.cy).abs() > 1e-6).count();
        assert!(differing > 0, "cosmos must not reuse basic's exact word positions");
    }

    #[test]
    fn every_variant_still_makes_a_more_frequent_word_bigger_or_taller() {
        for &variant in WordCloudVariant::all() {
            let (blocks, names) = draw(variant);
            let rust = blocks[names.iter().position(|n| n == "rust").unwrap()];
            let graph = blocks[names.iter().position(|n| n == "graph").unwrap()];
            assert!(rust.z1 >= graph.z1, "{variant:?}");
        }
    }
}
