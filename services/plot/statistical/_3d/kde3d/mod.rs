use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::spread::grouped_by_label;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::kde::layout3d;
use crate::plot::statistical::{KdeConfig, KdeVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("categories=[\"A\",\"A\",\"A\",\"B\",\"B\",\"B\"], values=[1,2,3,4,5,6]")]
#[crate::params(paramsList["title","categories","values","x","y","variant","bins","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("kde3d", "kde_3d", "kde3d_chart", "density3d")]
#[crate::sera_builder]
pub fn build_kde3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let y_values = a.y.clone().unwrap_or_default();
    let values = a.x.clone().or_else(|| a.values.clone()).unwrap_or_default();
    let categories = a.categories.unwrap_or_default();
    let series: Vec<(String, Vec<f64>)> = if categories.is_empty() { vec![("Series".to_string(), values)] } else { grouped_by_label(&categories, &values) };
    let y_series: Vec<Vec<f64>> = if categories.is_empty() || y_values.len() != categories.len() {
        Vec::new()
    } else {
        grouped_by_label(&categories, &y_values).into_iter().map(|(_, v)| v).collect()
    };
    let variant = KdeVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = KdeConfig { variant, title, series: &series, y_values: &y_values, y_series: &y_series, bins: o.bins.map(|b| b.max(1) as usize).unwrap_or(30), ..KdeConfig::default() };
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
        id: 72,
        name: "kde_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_kde3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::KdeVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/kde/", KdeVariant::keys_and_aliases(), KdeVariant::default_key())
    }

    #[test]
    fn every_kde_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_kde3d_chart, &demos(), KdeVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_kde_variant() {
        twin::check_planes(build_kde3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_kde_variant() {
        twin::check_scenes(build_kde3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_kde_variant() {
        twin::check_themes(build_kde3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_kde_variants_and_the_view_axes() {
        twin::check_axes("kde3d", KdeVariant::all().len());
    }

    #[test]
    fn every_kde_variant_honours_an_explicit_zone() {
        twin::check_zone(build_kde3d_chart, &demos());
    }

    #[test]
    fn every_kde_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_kde3d_chart, &demos());
    }

    #[test]
    fn every_kde_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_kde3d_chart, &demos(), 2000);
    }

    #[test]
    fn category_labels_reach_the_tooltip_names() {
        let html = build_kde3d_chart(r#"{"title":"t","categories":["Atlas","Atlas","Nova","Nova"],"values":[1,2,3,4]}"#);
        assert!(html.contains("'Atlas'") && html.contains("'Nova'"));
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("kde3d", build_kde3d_chart, &demos(), KdeVariant::default_key());
    }
}
