use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::correlogram::layout3d;
use crate::plot::statistical::{CorrelogramConfig, CorrelogramVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\",\"D\"], matrix=[[1,0.8,0.2,-0.5],[0.8,1,0.1,-0.3],[0.2,0.1,1,0.4],[-0.5,-0.3,0.4,1]]")]
#[crate::params(paramsList["title","labels","matrix","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("correlogram3d", "correlogram_3d", "correlogram3d_chart", "correlogram3d_family", "correlograms3d")]
#[crate::sera_builder]
pub fn build_correlogram3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let matrix: Vec<f64> = a.matrix.unwrap_or_default().into_iter().flatten().collect();
    let variant = CorrelogramVariant::from_str(o.variant.as_deref().unwrap_or("circle"));
    let cfg = CorrelogramConfig { variant, title, labels: &labels, matrix: &matrix, ..CorrelogramConfig::default() };
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
        id: 102,
        name: "correlogram_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_correlogram3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::CorrelogramVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/correlogram/", CorrelogramVariant::keys_and_aliases(), CorrelogramVariant::default_key())
    }

    #[test]
    fn every_correlogram_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_correlogram3d_chart, &demos(), CorrelogramVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_correlogram_variant() {
        twin::check_planes(build_correlogram3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_correlogram_variant() {
        twin::check_scenes(build_correlogram3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_correlogram_variant() {
        twin::check_themes(build_correlogram3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_correlogram_variants_and_the_view_axes() {
        twin::check_axes("correlogram3d", CorrelogramVariant::all().len());
    }

    #[test]
    fn every_correlogram_variant_honours_an_explicit_zone() {
        twin::check_zone(build_correlogram3d_chart, &demos());
    }

    #[test]
    fn every_correlogram_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_correlogram3d_chart, &demos());
    }

    #[test]
    fn every_correlogram_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_correlogram3d_chart, &demos(), 40);
    }

    #[test]
    fn cell_labels_reach_the_tooltip_names() {
        let html = build_correlogram3d_chart(r#"{"title":"t","labels":["X","Y"],"matrix":[[1,0.5],[0.5,1]]}"#);
        assert!(html.contains("X"));
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("correlogram3d", build_correlogram3d_chart, &demos(), CorrelogramVariant::default_key());
    }
}
