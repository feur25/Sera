use crate::plot::map::cartogram::layout3d;
use crate::plot::map::cartogram::{CartogramConfig, CartogramVariant};
use crate::plot::map::regions;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo(
    "labels=[\"USA\",\"CHN\",\"IND\",\"BRA\",\"RUS\",\"DEU\",\"FRA\",\"GBR\"], values=[331,1412,1408,215,144,84,68,67], lats=[39,35,21,-10,61,51,47,54], lons=[-98,105,78,-51,90,10,2,-2]"
)]
#[crate::params(paramsList["title","labels","values","lats","lons","map","region","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("cartogram3d", "cartogram_3d", "cartogram3d_chart")]
#[crate::sera_builder]
pub fn build_cartogram3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let lats = a.lats.unwrap_or_default();
    let lons = a.lons.unwrap_or_default();
    let region = regions::resolve(o.map.as_deref().unwrap_or("")).or_else(regions::default_region_set).expect("world region set must be registered");
    let variant = CartogramVariant::from_str(o.variant.as_deref().unwrap_or("dorling"));
    let mut cfg = CartogramConfig::new(region);
    cfg.variant = variant;
    cfg.title = title;
    cfg.labels = &labels;
    cfg.values = &values;
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
        o.h(650),
        bg_default,
        env,
    );
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "map",
        id: 120,
        name: "cartogram_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_cartogram3d_chart;
    use crate::plot::map::cartogram::CartogramVariant;
    use crate::plot::statistical::_3d::twin;

    fn demos() -> twin::Demos {
        twin::variant_demos("map/cartogram/", CartogramVariant::keys_and_aliases(), CartogramVariant::default_key())
    }

    #[test]
    fn every_cartogram_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_cartogram3d_chart, &demos(), CartogramVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_cartogram_variant() {
        twin::check_planes(build_cartogram3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_cartogram_variant() {
        twin::check_scenes(build_cartogram3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_cartogram_variant() {
        twin::check_themes(build_cartogram3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_cartogram_variants_and_the_view_axes() {
        twin::check_axes("cartogram3d", CartogramVariant::all().len());
    }

    #[test]
    fn every_cartogram_variant_honours_an_explicit_zone() {
        twin::check_zone(build_cartogram3d_chart, &demos());
    }

    #[test]
    fn every_cartogram_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_cartogram3d_chart, &demos());
    }

    #[test]
    fn every_cartogram_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_cartogram3d_chart, &demos(), 400);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("cartogram3d", build_cartogram3d_chart, &demos(), CartogramVariant::default_key());
    }
}
