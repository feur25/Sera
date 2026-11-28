use crate::plot::seaborn::_3d::render_scatter_spheres_html;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::plot_web::layout3d;
use crate::plot::statistical::{PlotWebConfig, PlotWebVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("x=[12,25,40,55,70,85], y=[18,42,28,55,38,72], labels=[\"A\",\"B\",\"C\",\"D\",\"E\",\"F\"], categories=[\"G1\",\"G1\",\"G2\",\"G2\",\"G3\",\"G3\"]")]
#[crate::params(paramsList["title","x","y","sizes","labels","categories","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("plot_web3d", "plot_web_3d", "plot_web3d_chart", "plot_web3d_family", "plot_webs3d")]
#[crate::sera_builder]
pub fn build_plot_web3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x_values = a.x.unwrap_or_default();
    let y_values = a.y.unwrap_or_default();
    let sizes = a.sizes.or(a.size).unwrap_or_default();
    let labels = a.labels.unwrap_or_default();
    let mut groups = a.categories.or(o.color_groups.clone()).unwrap_or_default();
    if groups.is_empty() {
        groups = vec![String::new(); x_values.len()];
    }
    let variant = PlotWebVariant::from_str(o.variant.as_deref().unwrap_or("scatter"));
    let cfg = PlotWebConfig { variant, title, x_values: &x_values, y_values: &y_values, sizes: &sizes, labels: &labels, groups: &groups, ..PlotWebConfig::default() };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let (blocks, names) = layout3d::layout_named(&cfg, &Budget::new(o.max_points));
    let html = render_scatter_spheres_html(
        title,
        &blocks,
        &names,
        (&o.xl(), &o.yl(), &o.zl()),
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
        id: 107,
        name: "plot_web_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_plot_web3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::PlotWebVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/plot_web/", PlotWebVariant::keys_and_aliases(), PlotWebVariant::default_key())
    }

    #[test]
    fn every_plot_web_variant_has_a_working_3d_counterpart() {
        for (key, json) in &demos() {
            let html = build_plot_web3d_chart(json);
            assert!(!html.is_empty(), "{key} must render");
            assert!(!html.contains("var BN="), "{key} must render as round spheres, not Bar3DBlock cuboids");
            assert!(html.contains("var S="), "{key} must carry per-node sphere sizes");
        }
    }

    #[test]
    fn every_3d_plane_applies_to_every_plot_web_variant() {
        twin::check_planes(build_plot_web3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_plot_web_variant() {
        for (key, json) in &demos() {
            for (scene_key, _) in crate::plot::scene3d::Scene3DVariant::keys_and_aliases() {
                let html = build_plot_web3d_chart(&twin::set_field(json, "scene", scene_key));
                assert!(!html.is_empty(), "{key} under {scene_key} must render");
            }
        }
    }

    #[test]
    fn every_chart_theme_styles_every_plot_web_variant() {
        twin::check_themes(build_plot_web3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_plot_web_variants_and_the_view_axes() {
        twin::check_axes("plot_web3d", PlotWebVariant::all().len());
    }

    #[test]
    fn every_plot_web_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_plot_web3d_chart, &demos());
    }

    #[test]
    fn every_plot_web_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_plot_web3d_chart, &demos(), 2000);
    }

    #[test]
    fn point_labels_reach_the_tooltip_names() {
        let html = build_plot_web3d_chart(r#"{"title":"t","x":[1,2],"y":[3,4],"labels":["Atlas","Nova"]}"#);
        assert!(html.contains("'Atlas'") && html.contains("'Nova'"));
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("plot_web3d", build_plot_web3d_chart, &demos(), PlotWebVariant::default_key());
    }
}
