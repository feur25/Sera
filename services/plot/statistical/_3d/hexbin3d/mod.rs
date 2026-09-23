use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::hexbin::layout3d;
use crate::plot::statistical::{HexbinConfig, HexbinVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("x=[1,2,2,3,3,3,4,4,4,4,5,5,5,2,3,4,1,2,3,4], y=[1,1,2,2,3,3,3,4,4,4,5,5,5,3,4,5,2,3,4,1]")]
#[crate::params(paramsList["title","x","y","values","variant","bins","min_count","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("hexbin3d", "hexbin_3d", "hexbin3d_chart", "hexbin3d_family", "hexbins3d")]
#[crate::sera_builder]
pub fn build_hexbin3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x_values = a.x.unwrap_or_default();
    let y_values = a.y.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let variant = HexbinVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let gridsize = o.bins.map(|b| b.max(2) as usize).unwrap_or(20);
    let min_count = o.min_count.map(|v| v.max(0) as u32).unwrap_or(0);
    let cfg = HexbinConfig { variant, title, x_values: &x_values, y_values: &y_values, values: &values, gridsize, min_count, ..HexbinConfig::default() };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let view = BlockView::new(layout3d::HEIGHT_RATIO, layout3d::COLORMAP).with_zone(o.zone.as_deref());
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
        id: 104,
        name: "hexbin_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_hexbin3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::HexbinVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/hexbin/", HexbinVariant::keys_and_aliases(), HexbinVariant::default_key())
    }

    #[test]
    fn every_hexbin_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_hexbin3d_chart, &demos(), HexbinVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_hexbin_variant() {
        twin::check_planes(build_hexbin3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_hexbin_variant() {
        twin::check_scenes(build_hexbin3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_hexbin_variant() {
        twin::check_themes(build_hexbin3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_hexbin_variants_and_the_view_axes() {
        twin::check_axes("hexbin3d", HexbinVariant::all().len());
    }

    #[test]
    fn every_hexbin_variant_honours_an_explicit_zone() {
        twin::check_zone(build_hexbin3d_chart, &demos());
    }

    #[test]
    fn every_hexbin_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_hexbin3d_chart, &demos());
    }

    #[test]
    fn every_hexbin_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_hexbin3d_chart, &demos(), 2000);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("hexbin3d", build_hexbin3d_chart, &demos(), HexbinVariant::default_key());
    }
}
