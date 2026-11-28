use crate::plot::map::flow_map::layout3d;
use crate::plot::map::flow_map::{FlowMapConfig, FlowMapVariant};
use crate::plot::map::regions;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("labels=[\"US\",\"CN\",\"DE\",\"GB\",\"JP\"], edges_i=[0,0,1,2], edges_j=[1,2,3,4], edges_w=[420,380,210,190]")]
#[crate::params(paramsList["title","labels","edges_i","edges_j","edges_w","lats","lons","map","region","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("flow_map3d", "flow_map_3d", "flow_map3d_chart", "flowmap3d")]
#[crate::sera_builder]
pub fn build_flow_map3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let sources = o.edges_i.clone().unwrap_or_default();
    let targets = o.edges_j.clone().unwrap_or_default();
    let weights = o.edges_w.clone().unwrap_or_default();
    let lats = a.lats.unwrap_or_default();
    let lons = a.lons.unwrap_or_default();
    let region = regions::resolve(o.map.as_deref().unwrap_or("")).or_else(regions::default_region_set).expect("world region set must be registered");
    let variant = FlowMapVariant::from_str(o.variant.as_deref().unwrap_or("arc"));
    let mut cfg = FlowMapConfig::new(region);
    cfg.variant = variant;
    cfg.title = title;
    cfg.labels = &labels;
    cfg.sources = &sources;
    cfg.targets = &targets;
    cfg.weights = &weights;
    cfg.lats = &lats;
    cfg.lons = &lons;
    cfg.group = o.region.as_deref().unwrap_or("");
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let view = BlockView::new(0.6, "jet").with_zone(o.zone.as_deref());
    let (blocks, names) = layout3d::layout_named(&cfg, &Budget::new(o.max_points));
    let html = render_blocks3d_view_html(
        title,
        &blocks,
        &view,
        (&o.xl(), &o.yl(), &o.zl()),
        &names,
        o.w(1200),
        o.h(600),
        bg_default,
        env,
    );
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "map",
        id: 121,
        name: "flow_map_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_flow_map3d_chart;
    use crate::plot::map::flow_map::FlowMapVariant;
    use crate::plot::statistical::_3d::twin;

    fn demos() -> twin::Demos {
        twin::variant_demos("map/flow_map/", FlowMapVariant::keys_and_aliases(), FlowMapVariant::default_key())
    }

    #[test]
    fn every_flow_map_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_flow_map3d_chart, &demos(), FlowMapVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_flow_map_variant() {
        twin::check_planes(build_flow_map3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_flow_map_variant() {
        twin::check_scenes(build_flow_map3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_flow_map_variant() {
        twin::check_themes(build_flow_map3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_flow_map_variants_and_the_view_axes() {
        twin::check_axes("flow_map3d", FlowMapVariant::all().len());
    }

    #[test]
    fn every_flow_map_variant_honours_an_explicit_zone() {
        twin::check_zone(build_flow_map3d_chart, &demos());
    }

    #[test]
    fn every_flow_map_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_flow_map3d_chart, &demos());
    }

    #[test]
    fn every_flow_map_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_flow_map3d_chart, &demos(), 400);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("flow_map3d", build_flow_map3d_chart, &demos(), FlowMapVariant::default_key());
    }
}
