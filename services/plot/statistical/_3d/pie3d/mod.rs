use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::common::apply_sort;
use crate::plot::statistical::pie::layout3d;
use crate::plot::statistical::PieVariant;
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\"], values=[30,50,20]")]
#[crate::params(paramsList["title","labels","values","secondary_values","sort_order","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("pie3d", "pie_3d", "pie3d_chart", "pie3d_family", "pies3d")]
#[crate::sera_builder]
pub fn build_pie3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let secondary_values = o.secondary_values.clone().unwrap_or_default();
    let srt = o.srt();
    let (labels, values) = apply_sort(&labels, &values, &srt);
    let variant = PieVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let view = BlockView::new(0.7, "jet").with_zone(o.zone.as_deref());
    let (blocks, names) = layout3d::layout_named(&labels, &values, &secondary_values, variant, &Budget::new(o.max_points));
    let html = render_blocks3d_view_html(
        title,
        &blocks,
        &view,
        (&o.xl(), &o.yl(), &o.zl()),
        &names,
        o.w(700),
        o.h(560),
        bg_default,
        env,
    );
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 74,
        name: "pie_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_pie3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::PieVariant;

    fn demos() -> twin::Demos {
        let base = r#"{"labels":["A","B","C"],"values":[30,50,20],"secondary_values":[10,15,5]}"#;
        PieVariant::keys_and_aliases().iter().map(|(key, _)| (*key, twin::set_field(base, "variant", key))).collect()
    }

    #[test]
    fn every_pie_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_pie3d_chart, &demos(), PieVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_pie_variant() {
        twin::check_planes(build_pie3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_pie_variant() {
        twin::check_scenes(build_pie3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_pie_variant() {
        twin::check_themes(build_pie3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_pie_variants_and_the_view_axes() {
        twin::check_axes("pie3d", PieVariant::all().len());
    }

    #[test]
    fn every_pie_variant_honours_an_explicit_zone() {
        twin::check_zone(build_pie3d_chart, &demos());
    }

    #[test]
    fn every_pie_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_pie3d_chart, &demos());
    }

    #[test]
    fn every_pie_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_pie3d_chart, &demos(), 400);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("pie3d", build_pie3d_chart, &demos(), PieVariant::default_key());
    }
}
