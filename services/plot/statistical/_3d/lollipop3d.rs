use crate::html::js_3d::render_3d_html;
use crate::plot::statistical::_3d::budget::{even_indices, pick, Budget};
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::common::format_axis_label;
use crate::plot::statistical::lollipop::layout3d;
use crate::plot::statistical::{LollipopConfig, LollipopVariant};
use crate::plot::{apply_bg3d, parse_all, ChartOpts};

pub fn render_lollipop3d_html(
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
        4,
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

fn legacy_points(title: &str, x: &[f64], y: &[f64], z: &[f64], o: &ChartOpts) -> String {
    let cl = o.color_labels.clone().unwrap_or_default();
    let keep = even_indices(x.len().min(y.len()).min(z.len()), Budget::new(o.max_points).elements());
    let (x, y, z) = (pick(x, &keep), pick(y, &keep), pick(z, &keep));
    let bg_str = o.bg_str();
    apply_bg3d(
        render_lollipop3d_html(
            title,
            &x,
            &y,
            &z,
            (&o.xl(), &o.yl(), &o.zl()),
            &[],
            &cl,
            o.w(900),
            o.h(560),
            bg_str.as_deref(),
            &o.scene3d(),
        ),
        o,
    )
}

#[crate::chart_demo("x=[1,2,3], y=[1,2,3], z=[4,5,6]")]
#[crate::params(paramsList["title","x","y","z","labels","values","color_groups","color_labels","variant","sort_order","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("lollipop3d", "lollipop_3d", "lollipop3d_chart", "lollipop3d_family", "lollipops3d")]
#[crate::sera_builder]
pub fn build_lollipop3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    if let Some(z) = a.z.as_deref() {
        return legacy_points(title, a.x.as_deref().unwrap_or_default(), a.y.as_deref().unwrap_or_default(), z, &o);
    }
    let labels = a
        .labels
        .clone()
        .unwrap_or_else(|| a.x.as_ref().map(|xs| xs.iter().map(|&v| format_axis_label(v)).collect()).unwrap_or_default());
    let values = a.values.clone().unwrap_or_else(|| a.y.clone().unwrap_or_default());
    let groups = o.color_groups.clone().unwrap_or_default();
    let sort_order = o.srt();
    let cfg = LollipopConfig {
        variant: LollipopVariant::from_str(o.variant.as_deref().unwrap_or("basic")),
        title,
        labels: &labels,
        values: &values,
        groups: &groups,
        sort_order: &sort_order,
        ..LollipopConfig::default()
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
        id: 71,
        name: "lollipop_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_lollipop3d_chart;
    use crate::plot::statistical::_3d::budget::Budget;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::LollipopVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/lollipop/", LollipopVariant::keys_and_aliases(), LollipopVariant::default_key())
    }

    fn columns(n: usize) -> serde_json::Value {
        serde_json::json!({"title": "t", "x": twin::wave(n, 0), "y": twin::wave(n, 1), "z": twin::wave(n, 2)})
    }

    #[test]
    fn every_lollipop_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_lollipop3d_chart, &demos(), LollipopVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_lollipop_variant() {
        twin::check_planes(build_lollipop3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_lollipop_variant() {
        twin::check_scenes(build_lollipop3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_lollipop_variant() {
        twin::check_themes(build_lollipop3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_lollipop_variants_and_the_view_axes() {
        twin::check_axes("lollipop3d", LollipopVariant::all().len());
    }

    #[test]
    fn every_lollipop_variant_honours_an_explicit_zone() {
        twin::check_zone(build_lollipop3d_chart, &demos());
    }

    #[test]
    fn every_lollipop_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_lollipop3d_chart, &demos());
    }

    #[test]
    fn every_lollipop_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_lollipop3d_chart, &demos(), 2000);
    }

    #[test]
    fn the_legacy_spatial_call_still_renders_points_not_blocks() {
        let html = build_lollipop3d_chart(r#"{"title":"t","x":[1,2,3],"y":[1,2,3],"z":[4,5,6]}"#);
        assert!(html.contains("class=\"c3w\""));
        assert!(!html.contains("var BN="));
    }

    #[test]
    fn a_big_legacy_lollipop_chart_is_capped_by_the_budget_and_small_ones_are_untouched() {
        twin::check_capped(build_lollipop3d_chart, columns, Budget::elements);
    }

    #[test]
    fn category_labels_reach_the_tooltip_names() {
        let html = build_lollipop3d_chart(r#"{"title":"t","labels":["Alpha","Beta"],"values":[3,5]}"#);
        assert!(html.contains("var CL=['Alpha','Beta']"));
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("lollipop3d", build_lollipop3d_chart, &demos(), LollipopVariant::default_key());
    }
}
