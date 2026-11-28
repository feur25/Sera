use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::sankey::layout3d;
use crate::plot::statistical::{SankeyConfig, SankeyVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\",\"D\"], edges_i=[0,0,1], edges_j=[2,3,3], edges_w=[5,3,4]")]
#[crate::params(paramsList["title","labels","edges_i","edges_j","edges_w","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("sankey3d", "sankey_3d", "sankey3d_chart", "sankey3d_family", "sankeys3d")]
#[crate::sera_builder]
pub fn build_sankey3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let sources = o.edges_i.clone().unwrap_or_default();
    let targets = o.edges_j.clone().unwrap_or_default();
    let weights = o.edges_w.clone().unwrap_or_else(|| vec![1.0; sources.len()]);
    let variant = SankeyVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = SankeyConfig {
        variant,
        title,
        labels: &labels,
        sources: &sources,
        targets: &targets,
        weights: &weights,
        node_width: 18,
        node_gap: 10,
        ..SankeyConfig::default()
    };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let view = BlockView::new(layout3d::HEIGHT_RATIO, "jet").with_zone(o.zone.as_deref());
    let (blocks, names) = layout3d::layout_named(&cfg, &Budget::new(o.max_points));
    let html = render_blocks3d_view_html(
        title,
        &blocks,
        &view,
        (&o.xl(), &o.yl(), &o.zl()),
        &names,
        o.w(900),
        o.h(560),
        bg_default,
        env,
    );
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 109,
        name: "sankey_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_sankey3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::SankeyVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/sankey/", SankeyVariant::keys_and_aliases(), SankeyVariant::default_key())
    }

    #[test]
    fn every_sankey_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_sankey3d_chart, &demos(), SankeyVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_sankey_variant() {
        twin::check_planes(build_sankey3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_sankey_variant() {
        twin::check_scenes(build_sankey3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_sankey_variant() {
        twin::check_themes(build_sankey3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_sankey_variants_and_the_view_axes() {
        twin::check_axes("sankey3d", SankeyVariant::all().len());
    }

    #[test]
    fn every_sankey_variant_honours_an_explicit_zone() {
        twin::check_zone(build_sankey3d_chart, &demos());
    }

    #[test]
    fn every_sankey_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_sankey3d_chart, &demos());
    }

    #[test]
    fn every_sankey_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_sankey3d_chart, &demos(), 400);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("sankey3d", build_sankey3d_chart, &demos(), SankeyVariant::default_key());
    }
}
