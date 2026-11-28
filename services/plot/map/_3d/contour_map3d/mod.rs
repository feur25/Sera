use crate::plot::map::contour_map::layout3d;
use crate::plot::map::contour_map::{ContourMapConfig, ContourMapVariant};
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo(
    "lats=[10,15,20,25,30,-10,-15,-20,0,5], lons=[-40,-45,-50,-55,-60,20,25,30,0,-10], field=[25,35,45,60,75,30,40,50,55,65]"
)]
#[crate::params(paramsList["title","lats","lons","field","levels","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("contour_map3d", "contour_map_3d", "contour_map3d_chart", "contourmap3d")]
#[crate::sera_builder]
pub fn build_contour_map3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let lats = a.lats.unwrap_or_default();
    let lons = a.lons.unwrap_or_default();
    let field = a.field.clone().or(a.values.clone()).unwrap_or_default();
    let variant = ContourMapVariant::from_str(o.variant.as_deref().unwrap_or("filled"));
    let cfg = ContourMapConfig {
        variant,
        title,
        lats: &lats,
        lons: &lons,
        field: &field,
        width: o.w(1200),
        height: o.h(650),
        levels: o.bins.unwrap_or(6).max(2) as usize,
        color_low: o.color_low.unwrap_or(0x1e3a8a),
        color_high: o.color_high.unwrap_or(0xdc2626),
    };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let view = BlockView::new(0.5, "jet").with_zone(o.zone.as_deref());
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
        id: 122,
        name: "contour_map_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_contour_map3d_chart;
    use crate::plot::map::contour_map::ContourMapVariant;
    use crate::plot::statistical::_3d::twin;

    fn demos() -> twin::Demos {
        twin::variant_demos("map/contour_map/", ContourMapVariant::keys_and_aliases(), ContourMapVariant::default_key())
    }

    #[test]
    fn every_contour_map_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_contour_map3d_chart, &demos(), ContourMapVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_contour_map_variant() {
        twin::check_planes(build_contour_map3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_contour_map_variant() {
        twin::check_scenes(build_contour_map3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_contour_map_variant() {
        twin::check_themes(build_contour_map3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_contour_map_variants_and_the_view_axes() {
        twin::check_axes("contour_map3d", ContourMapVariant::all().len());
    }

    #[test]
    fn every_contour_map_variant_honours_an_explicit_zone() {
        twin::check_zone(build_contour_map3d_chart, &demos());
    }

    #[test]
    fn every_contour_map_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_contour_map3d_chart, &demos());
    }

    #[test]
    fn every_contour_map_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_contour_map3d_chart, &demos(), 400);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("contour_map3d", build_contour_map3d_chart, &demos(), ContourMapVariant::default_key());
    }
}
