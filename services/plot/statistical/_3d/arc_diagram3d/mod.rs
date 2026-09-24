use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::arc_diagram::layout3d;
use crate::plot::statistical::{ArcDiagramConfig, ArcDiagramVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\",\"D\",\"E\",\"F\"], edges_i=[0,0,1,2,3,4], edges_j=[1,2,3,4,5,0], edges_w=[3,5,2,4,6,1]")]
#[crate::params(paramsList["title","labels","edges_i","edges_j","edges_w","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("arc_diagram3d", "arc_diagram_3d", "arc_diagram3d_chart", "arc_diagram3d_family", "linear_network3d")]
#[crate::sera_builder]
pub fn build_arc_diagram3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let sources = o.edges_i.clone().unwrap_or_default();
    let targets = o.edges_j.clone().unwrap_or_default();
    let weights = o.edges_w.clone().unwrap_or_else(|| vec![1.0; sources.len()]);
    let variant = ArcDiagramVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = ArcDiagramConfig {
        variant,
        title,
        labels: &labels,
        sources: &sources,
        targets: &targets,
        weights: &weights,
        ..ArcDiagramConfig::default()
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
        o.w(900),
        o.h(500),
        bg_default,
        env,
    );
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 111,
        name: "arc_diagram_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_arc_diagram3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::ArcDiagramVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/arc_diagram/", ArcDiagramVariant::keys_and_aliases(), ArcDiagramVariant::default_key())
    }

    #[test]
    fn every_arc_diagram_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_arc_diagram3d_chart, &demos(), ArcDiagramVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_arc_diagram_variant() {
        twin::check_planes(build_arc_diagram3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_arc_diagram_variant() {
        twin::check_scenes(build_arc_diagram3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_arc_diagram_variant() {
        twin::check_themes(build_arc_diagram3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_arc_diagram_variants_and_the_view_axes() {
        twin::check_axes("arc_diagram3d", ArcDiagramVariant::all().len());
    }

    #[test]
    fn every_arc_diagram_variant_honours_an_explicit_zone() {
        twin::check_zone(build_arc_diagram3d_chart, &demos());
    }

    #[test]
    fn every_arc_diagram_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_arc_diagram3d_chart, &demos());
    }

    #[test]
    fn every_arc_diagram_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_arc_diagram3d_chart, &demos(), 400);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("arc_diagram3d", build_arc_diagram3d_chart, &demos(), ArcDiagramVariant::default_key());
    }
}
