use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::dendrogram::layout3d;
use crate::plot::statistical::{DendrogramConfig, DendrogramVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo(
    "labels=[\"A1\",\"A2\",\"A3\",\"B1\",\"B2\",\"B3\",\"C1\",\"C2\",\"C3\"], matrix=[[1,1],[1.2,0.9],[0.9,1.1],[5,5],[5.2,4.8],[4.9,5.1],[1,5],[1.1,4.9],[0.9,5.2]]"
)]
#[crate::params(paramsList["title","labels","parents","matrix","k","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("dendrogram3d", "dendrogram_3d", "dendrogram3d_chart", "dendrogram3d_family", "dendrograms3d")]
#[crate::sera_builder]
pub fn build_dendrogram3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let parents = a.parents.unwrap_or_default();
    let matrix = a.matrix.unwrap_or_default();
    let variant = DendrogramVariant::from_str(o.variant.as_deref().unwrap_or("vertical"));
    let cfg = DendrogramConfig {
        variant,
        title,
        labels: &labels,
        parents: &parents,
        values: &matrix,
        clusters: o.k.unwrap_or(3),
        ..DendrogramConfig::default()
    };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let view = BlockView::new(layout3d::HEIGHT_RATIO, "jet").with_zone(o.zone.as_deref());
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
        id: 98,
        name: "dendrogram_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_dendrogram3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::DendrogramVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/dendrogram/", DendrogramVariant::keys_and_aliases(), DendrogramVariant::default_key())
    }

    #[test]
    fn every_dendrogram_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_dendrogram3d_chart, &demos(), DendrogramVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_dendrogram_variant() {
        twin::check_planes(build_dendrogram3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_dendrogram_variant() {
        twin::check_scenes(build_dendrogram3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_dendrogram_variant() {
        twin::check_themes(build_dendrogram3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_dendrogram_variants_and_the_view_axes() {
        twin::check_axes("dendrogram3d", DendrogramVariant::all().len());
    }

    #[test]
    fn every_dendrogram_variant_honours_an_explicit_zone() {
        twin::check_zone(build_dendrogram3d_chart, &demos());
    }

    #[test]
    fn every_dendrogram_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_dendrogram3d_chart, &demos());
    }

    #[test]
    fn every_dendrogram_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_dendrogram3d_chart, &demos(), 400);
    }

    #[test]
    fn node_labels_reach_the_tooltip_names() {
        let html = build_dendrogram3d_chart(r#"{"title":"t","labels":["Root","A"],"parents":["","Root"]}"#);
        assert!(html.contains("'Root'") && html.contains("'A'"));
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("dendrogram3d", build_dendrogram3d_chart, &demos(), DendrogramVariant::default_key());
    }
}
