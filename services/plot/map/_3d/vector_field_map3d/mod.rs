use crate::plot::map::vector_field_map::layout3d;
use crate::plot::map::vector_field_map::{VectorFieldMapConfig, VectorFieldMapVariant};
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("lats=[-55,-35,-15,15,45], lons=[-150,-90,-30,30,90], u=[-12,30,-18,-18,25], v=[3,-3,6,-6,3]")]
#[crate::params(paramsList["title","lats","lons","u","v","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("vector_field_map3d", "vector_field_map_3d", "vector_field_map3d_chart", "vectorfieldmap3d")]
#[crate::sera_builder]
pub fn build_vector_field_map3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let lats = a.lats.unwrap_or_default();
    let lons = a.lons.unwrap_or_default();
    let u = a.u.unwrap_or_default();
    let v = a.v.unwrap_or_default();
    let variant = VectorFieldMapVariant::from_str(o.variant.as_deref().unwrap_or("arrows"));
    let cfg = VectorFieldMapConfig {
        variant,
        title,
        lats: &lats,
        lons: &lons,
        u: &u,
        v: &v,
        width: o.w(1200),
        height: o.h(650),
        color_low: o.color_low.unwrap_or(0x38bdf8),
        color_high: o.color_high.unwrap_or(0xf97316),
    };
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
        o.h(650),
        bg_default,
        env,
    );
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "map",
        id: 124,
        name: "vector_field_map_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_vector_field_map3d_chart;
    use crate::plot::map::vector_field_map::VectorFieldMapVariant;
    use crate::plot::statistical::_3d::twin;

    fn demos() -> twin::Demos {
        twin::variant_demos("map/vector_field_map/", VectorFieldMapVariant::keys_and_aliases(), VectorFieldMapVariant::default_key())
    }

    #[test]
    fn every_vector_field_map_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_vector_field_map3d_chart, &demos(), VectorFieldMapVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_vector_field_map_variant() {
        twin::check_planes(build_vector_field_map3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_vector_field_map_variant() {
        twin::check_scenes(build_vector_field_map3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_vector_field_map_variant() {
        twin::check_themes(build_vector_field_map3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_vector_field_map_variants_and_the_view_axes() {
        twin::check_axes("vector_field_map3d", VectorFieldMapVariant::all().len());
    }

    #[test]
    fn every_vector_field_map_variant_honours_an_explicit_zone() {
        twin::check_zone(build_vector_field_map3d_chart, &demos());
    }

    #[test]
    fn every_vector_field_map_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_vector_field_map3d_chart, &demos());
    }

    #[test]
    fn every_vector_field_map_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_vector_field_map3d_chart, &demos(), 400);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("vector_field_map3d", build_vector_field_map3d_chart, &demos(), VectorFieldMapVariant::default_key());
    }
}
