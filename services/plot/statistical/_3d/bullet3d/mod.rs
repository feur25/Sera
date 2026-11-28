use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::bullet::layout3d;
use crate::plot::statistical::{BulletConfig, BulletVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("labels=[\"Revenue\",\"Profit\",\"CSAT\"], values=[80,65,4.2], targets=[90,70,4.5], max_vals=[120,100,5]")]
#[crate::params(paramsList["title","labels","values","targets","max_vals","ranges","comparisons","variant","sort_order","x_label","y_label","z_label","bg_color","scene","orientation3d","theme","zone","max_points","width","height"])]
#[crate::sera_alias("bullet3d", "bullet_3d", "bullet3d_chart", "bullet3d_family", "bullets3d")]
#[crate::sera_builder]
pub fn build_bullet3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let targets = o.targets.clone().unwrap_or_default();
    let max_vals = o.max_vals.clone().unwrap_or_default();
    let ranges = o.ranges.clone().unwrap_or_default();
    let comparisons = o.comparisons.clone().unwrap_or_default();
    let sort_order = o.srt();
    let cfg = BulletConfig {
        variant: BulletVariant::from_str(o.variant.as_deref().unwrap_or("basic")),
        title,
        labels: &labels,
        values: &values,
        targets: &targets,
        max_vals: &max_vals,
        ranges: &ranges,
        comparisons: &comparisons,
        sort_order: &sort_order,
        ..BulletConfig::default()
    };
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
        id: 92,
        name: "bullet_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_bullet3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::BulletVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/bullet/", BulletVariant::keys_and_aliases(), BulletVariant::default_key())
    }

    #[test]
    fn every_bullet_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_bullet3d_chart, &demos(), BulletVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_bullet_variant() {
        twin::check_planes(build_bullet3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_bullet_variant() {
        twin::check_scenes(build_bullet3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_bullet_variant() {
        twin::check_themes(build_bullet3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_bullet_variants_and_the_view_axes() {
        twin::check_axes("bullet3d", BulletVariant::all().len());
    }

    #[test]
    fn every_bullet_variant_honours_an_explicit_zone() {
        twin::check_zone(build_bullet3d_chart, &demos());
    }

    #[test]
    fn every_bullet_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_bullet3d_chart, &demos());
    }

    #[test]
    fn every_bullet_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_bullet3d_chart, &demos(), 2000);
    }

    #[test]
    fn hover_names_reach_the_engine() {
        let html = build_bullet3d_chart(r#"{"title":"t","labels":["Revenue"],"values":[80],"targets":[90],"max_vals":[120]}"#);
        assert!(html.contains("Revenue · 80 of 120"));
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("bullet3d", build_bullet3d_chart, &demos(), BulletVariant::default_key());
    }
}
