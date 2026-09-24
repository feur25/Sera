use super::common::prepare;
use super::config::ParcatsConfig;
use super::variant::ParcatsVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::bar::Bar3DBlock;
use crate::plot::statistical::sankey::layout3d::flow_3d;

fn max_edge(variant: ParcatsVariant) -> f64 {
    use ParcatsVariant::*;
    match variant {
        Basic => 0.18,
        Highlight => 0.1,
    }
}

fn parcats_3d(cfg: &ParcatsConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let Some(p) = prepare(cfg) else {
        return (Vec::new(), Vec::new());
    };
    flow_3d(&p.labels, &p.sources, &p.targets, &p.weights, cfg.node_width, cfg.node_gap, false, max_edge(cfg.variant))
}

pub fn layout_3d(cfg: &ParcatsConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &ParcatsConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    parcats_3d(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rows() -> (Vec<String>, Vec<Vec<String>>) {
        (
            ["Gender", "Survived", "Class"].iter().map(|s| s.to_string()).collect(),
            vec![
                vec!["Male".into(), "No".into(), "3rd".into()],
                vec!["Female".into(), "Yes".into(), "1st".into()],
                vec!["Male".into(), "No".into(), "2nd".into()],
                vec!["Female".into(), "Yes".into(), "1st".into()],
                vec!["Male".into(), "Yes".into(), "1st".into()],
                vec!["Female".into(), "No".into(), "3rd".into()],
            ],
        )
    }

    fn draw(variant: ParcatsVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (axes, category_series) = rows();
        let cfg = ParcatsConfig { variant, axes: &axes, category_series: &category_series, ..ParcatsConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_one_column_per_distinct_category_plus_ribbons() {
        for &variant in ParcatsVariant::all() {
            let (blocks, names) = draw(variant);
            assert!(names.len() >= 6, "{variant:?}");
            assert!(blocks.len() > names.len(), "{variant:?}");
        }
    }

    #[test]
    fn highlight_edges_are_thinner_than_basic_edges() {
        let (basic, names) = draw(ParcatsVariant::Basic);
        let (highlight, _) = draw(ParcatsVariant::Highlight);
        let n = names.len();
        let max_hw = |blocks: &[Bar3DBlock]| blocks.iter().map(|b| b.hw).fold(0.0f64, f64::max);
        assert!(max_hw(&highlight[n..]) < max_hw(&basic[n..]));
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = layout_named(&ParcatsConfig::default(), &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }
}
