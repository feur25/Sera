use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::venn::layout3d;
use crate::plot::statistical::{VennConfig, VennVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("labels=[\"Set A\",\"Set B\",\"Set C\"], values=[40,30,20,15,10,5,8]")]
#[crate::params(paramsList["title","labels","values","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("venn3d", "venn_3d", "venn3d_chart", "euler3d", "set_diagram3d")]
#[crate::sera_builder]
pub fn build_venn3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let variant = VennVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = VennConfig { variant, title, labels: &labels, values: &values, ..VennConfig::default() };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let view = BlockView::new(0.65, "jet").with_zone(o.zone.as_deref());
    let (blocks, names) = layout3d::layout_named(&cfg, &Budget::new(o.max_points));
    let html = render_blocks3d_view_html(
        title,
        &blocks,
        &view,
        (&o.xl(), &o.yl(), &o.zl()),
        &names,
        o.w(560),
        o.h(420),
        bg_default,
        env,
    );
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 115,
        name: "venn_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_venn3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::VennVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/venn/", VennVariant::keys_and_aliases(), VennVariant::default_key())
    }

    #[test]
    fn every_venn_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_venn3d_chart, &demos(), VennVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_venn_variant() {
        twin::check_planes(build_venn3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_venn_variant() {
        twin::check_scenes(build_venn3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_venn_variant() {
        twin::check_themes(build_venn3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_venn_variants_and_the_view_axes() {
        twin::check_axes("venn3d", VennVariant::all().len());
    }

    #[test]
    fn every_venn_variant_honours_an_explicit_zone() {
        twin::check_zone(build_venn3d_chart, &demos());
    }

    #[test]
    fn every_venn_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_venn3d_chart, &demos());
    }

    #[test]
    fn every_venn_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_venn3d_chart, &demos(), 400);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("venn3d", build_venn3d_chart, &demos(), VennVariant::default_key());
    }
}
