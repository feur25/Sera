use crate::html::js_3d::render_3d_html;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::dumbbell::layout3d;
use crate::plot::statistical::{DumbbellConfig, DumbbellVariant};
use crate::plot::{apply_bg3d, parse_all};

pub fn render_dumbbell3d_html(
    title: &str,
    x: &[f64],
    y: &[f64],
    z: &[f64],
    axis_labels: (&str, &str, &str),
    colors: &[f64],
    color_labels: &[String],
    w: i32,
    h: i32,
    bg_color: Option<&str>,
    scene: &str,
) -> String {
    render_3d_html(
        11,
        title,
        x,
        y,
        z,
        axis_labels,
        colors,
        color_labels,
        w,
        h,
        bg_color,
        scene,
    )
}

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\"], start=[10,20,15], end=[30,25,40]")]
#[crate::params(paramsList["title","labels","start","end","series_name_start","series_name_end","variant","sort_order","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("dumbbell3d", "dumbbell_3d", "dumbbell3d_chart", "dumbbell3d_family", "dumbbells3d")]
#[crate::sera_builder]
pub fn build_dumbbell3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.clone().unwrap_or_default();
    let start = a.start.clone().unwrap_or_default();
    let end = a.end.clone().unwrap_or_default();
    let series = (o.series_name_start.as_deref().unwrap_or("Start"), o.series_name_end.as_deref().unwrap_or("End"));
    let sort_order = o.srt();
    let variant = DumbbellVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = DumbbellConfig {
        variant,
        title,
        labels: &labels,
        values_start: &start,
        values_end: &end,
        series_names: series,
        sort_order: &sort_order,
        ..DumbbellConfig::default()
    };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let view = BlockView::new(layout3d::HEIGHT_RATIO, layout3d::colormap(variant)).with_zone(o.zone.as_deref());
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
        id: 78,
        name: "dumbbell_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_dumbbell3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::DumbbellVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/dumbbell/", DumbbellVariant::keys_and_aliases(), DumbbellVariant::default_key())
    }

    #[test]
    fn every_dumbbell_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_dumbbell3d_chart, &demos(), DumbbellVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_dumbbell_variant() {
        twin::check_planes(build_dumbbell3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_dumbbell_variant() {
        twin::check_scenes(build_dumbbell3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_dumbbell_variant() {
        twin::check_themes(build_dumbbell3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_dumbbell_variants_and_the_view_axes() {
        twin::check_axes("dumbbell3d", DumbbellVariant::all().len());
    }

    #[test]
    fn every_dumbbell_variant_honours_an_explicit_zone() {
        twin::check_zone(build_dumbbell3d_chart, &demos());
    }

    #[test]
    fn every_dumbbell_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_dumbbell3d_chart, &demos());
    }

    #[test]
    fn every_dumbbell_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_dumbbell3d_chart, &demos(), 2000);
    }

    #[test]
    fn the_original_labels_start_end_call_renders_blocks_named_after_both_ends() {
        let html = build_dumbbell3d_chart(r#"{"title":"t","labels":["A","B"],"start":[1,2],"end":[3,1]}"#);
        assert!(html.contains("var BN="));
        assert!(html.contains("A · Start") && html.contains("B · End"));
    }

    #[test]
    fn custom_series_names_replace_start_and_end() {
        let html = build_dumbbell3d_chart(r#"{"title":"t","labels":["A"],"start":[1],"end":[3],"series_name_start":"2019","series_name_end":"2024"}"#);
        assert!(html.contains("A · 2019") && html.contains("A · 2024"));
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("dumbbell3d", build_dumbbell3d_chart, &demos(), DumbbellVariant::default_key());
    }
}
