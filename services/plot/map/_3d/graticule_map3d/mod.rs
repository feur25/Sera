use crate::plot::map::graticule_map::layout3d;
use crate::plot::map::graticule_map::{GraticuleMapConfig, GraticuleMapVariant};
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("step=15")]
#[crate::params(paramsList["title","step","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("graticule_map3d", "graticule_map_3d", "graticule_map3d_chart", "graticulemap3d")]
#[crate::sera_builder]
pub fn build_graticule_map3d_chart(input: &str) -> String {
    let (title_s, _a, o) = parse_all(input);
    let title = title_s.as_str();
    let variant = GraticuleMapVariant::from_str(o.variant.as_deref().unwrap_or("lines"));
    let mut cfg = GraticuleMapConfig::new();
    cfg.variant = variant;
    cfg.title = title;
    cfg.step = o.step.unwrap_or(15.0);
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let view = BlockView::new(0.35, "jet").with_zone(o.zone.as_deref());
    let (blocks, names) = layout3d::layout_named(&cfg, &Budget::new(o.max_points));
    let html = render_blocks3d_view_html(
        title,
        &blocks,
        &view,
        (&o.xl(), &o.yl(), &o.zl()),
        &names,
        o.w(1200),
        o.h(650),
        bg_default,
        env,
    );
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "map",
        id: 123,
        name: "graticule_map_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_graticule_map3d_chart;
    use crate::plot::map::graticule_map::GraticuleMapVariant;
    use crate::plot::statistical::_3d::twin;

    fn demos() -> twin::Demos {
        twin::variant_demos("map/graticule_map/", GraticuleMapVariant::keys_and_aliases(), GraticuleMapVariant::default_key())
    }

    #[test]
    fn every_graticule_map_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_graticule_map3d_chart, &demos(), GraticuleMapVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_graticule_map_variant() {
        twin::check_planes(build_graticule_map3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_graticule_map_variant() {
        twin::check_scenes(build_graticule_map3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_graticule_map_variant() {
        twin::check_themes(build_graticule_map3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_graticule_map_variants_and_the_view_axes() {
        twin::check_axes("graticule_map3d", GraticuleMapVariant::all().len());
    }

    #[test]
    fn every_graticule_map_variant_honours_an_explicit_zone() {
        twin::check_zone(build_graticule_map3d_chart, &demos());
    }

    #[test]
    fn every_graticule_map_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_graticule_map3d_chart, &demos());
    }

    #[test]
    fn every_graticule_map_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_graticule_map3d_chart, &demos(), 400);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("graticule_map3d", build_graticule_map3d_chart, &demos(), GraticuleMapVariant::default_key());
    }
}
