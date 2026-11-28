use crate::plot::statistical::bar::Bar3DBlock;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::scatterternary::layout3d;
use crate::plot::statistical::{ScatterTernaryConfig, ScatterTernaryVariant};
use crate::plot::{apply_bg3d, parse_all};

fn render_ternary_points_html(
    title: &str,
    blocks: &[Bar3DBlock],
    names: &[String],
    axis_labels: (&str, &str, &str),
    w: i32,
    h: i32,
    bg: Option<&str>,
    scene: &str,
) -> String {
    if blocks.is_empty() {
        return crate::html::js_3d::render_3d_html_impl(0, title, &[0.0], &[0.0], &[0.0], axis_labels, &[], &[], w, h, bg, scene, b"");
    }
    let x: Vec<f64> = blocks.iter().map(|b| b.cx).collect();
    let y: Vec<f64> = blocks.iter().map(|b| b.cy).collect();
    let z: Vec<f64> = blocks.iter().map(|b| (b.z0 + b.z1) / 2.0).collect();
    let colors: Vec<f64> = blocks.iter().map(|b| b.ci as f64).collect();
    crate::html::js_3d::render_3d_html_impl(0, title, &x, &y, &z, axis_labels, &colors, names, w, h, bg, scene, b"")
}

fn render_ternary_spheres_html(
    title: &str,
    blocks: &[Bar3DBlock],
    names: &[String],
    axis_labels: (&str, &str, &str),
    w: i32,
    h: i32,
    bg: Option<&str>,
    scene: &str,
) -> String {
    if blocks.is_empty() {
        return crate::html::js_3d::render_3d_html_impl(16, title, &[0.0], &[0.0], &[0.0], axis_labels, &[], &[], w, h, bg, scene, b"var S=[];");
    }
    let x: Vec<f64> = blocks.iter().map(|b| b.cx).collect();
    let y: Vec<f64> = blocks.iter().map(|b| b.cy).collect();
    let z: Vec<f64> = blocks.iter().map(|b| (b.z0 + b.z1) / 2.0).collect();
    let (hlo, hhi) = blocks.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), b| (lo.min(b.hw), hi.max(b.hw)));
    let hrange = (hhi - hlo).max(1e-9);
    let size_js = format!(
        "var S=[{}];",
        blocks.iter().map(|b| format!("{:.4}", (b.hw - hlo) / hrange)).collect::<Vec<_>>().join(",")
    );
    let colors: Vec<f64> = blocks.iter().map(|b| b.ci as f64).collect();
    crate::html::js_3d::render_3d_html_impl(16, title, &x, &y, &z, axis_labels, &colors, names, w, h, bg, scene, size_js.as_bytes())
}

#[crate::chart_demo("x=[0.7,0.2,0.1,0.4,0.33], y=[0.2,0.6,0.1,0.3,0.33], z=[0.1,0.2,0.8,0.3,0.34]")]
#[crate::params(paramsList["title","x","y","z","labels","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("scatterternary3d", "scatterternary_3d", "scatterternary3d_chart", "scatterternary3d_family", "scatterternaries3d")]
#[crate::sera_builder]
pub fn build_scatterternary3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let a_values = a.x.unwrap_or_default();
    let b_values = a.y.unwrap_or_default();
    let c_values = a.z.unwrap_or_default();
    let labels = a.labels.unwrap_or_default();
    let color_values = o.color_values.clone().unwrap_or_default();
    let variant = ScatterTernaryVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = ScatterTernaryConfig {
        variant,
        title,
        a_values: &a_values,
        b_values: &b_values,
        c_values: &c_values,
        labels: &labels,
        color_values: &color_values,
        ..ScatterTernaryConfig::default()
    };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let (blocks, names) = layout3d::layout_named(&cfg, &Budget::new(o.max_points));
    let axis_labels = (o.xl(), o.yl(), o.zl());
    let axis_refs = (axis_labels.0.as_str(), axis_labels.1.as_str(), axis_labels.2.as_str());
    let html = if matches!(variant, ScatterTernaryVariant::Bubble) {
        render_ternary_spheres_html(title, &blocks, &names, axis_refs, o.w(900), o.h(560), bg_default, env)
    } else {
        render_ternary_points_html(title, &blocks, &names, axis_refs, o.w(900), o.h(560), bg_default, env)
    };
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 106,
        name: "scatterternary_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_scatterternary3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::ScatterTernaryVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/scatterternary/", ScatterTernaryVariant::keys_and_aliases(), ScatterTernaryVariant::default_key())
    }

    #[test]
    fn every_scatterternary_variant_has_a_working_3d_counterpart() {
        let all = demos();
        assert_eq!(all.len(), ScatterTernaryVariant::all().len());
        for (key, json) in &all {
            let html = build_scatterternary3d_chart(json);
            assert!(!html.is_empty(), "{key} must render");
            assert!(!html.contains("var BN="), "{key} must render as round points, not Bar3DBlock cuboids");
        }
    }

    #[test]
    fn every_3d_plane_applies_to_every_scatterternary_variant() {
        twin::check_planes(build_scatterternary3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_scatterternary_variant() {
        for (key, json) in &demos() {
            for (scene_key, _) in crate::plot::scene3d::Scene3DVariant::keys_and_aliases() {
                let html = build_scatterternary3d_chart(&twin::set_field(json, "scene", scene_key));
                assert!(!html.is_empty(), "{key} under {scene_key} must render");
            }
        }
    }

    #[test]
    fn every_chart_theme_styles_every_scatterternary_variant() {
        twin::check_themes(build_scatterternary3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_scatterternary_variants_and_the_view_axes() {
        twin::check_axes("scatterternary3d", ScatterTernaryVariant::all().len());
    }

    #[test]
    fn every_scatterternary_variant_renders_regardless_of_an_explicit_zone_hint() {
        for (key, json) in &demos() {
            let html = build_scatterternary3d_chart(&twin::set_field(json, "zone", "[2.0,1.0,1.0]"));
            assert!(!html.is_empty(), "{key} must still render with a zone hint present");
        }
    }

    #[test]
    fn every_scatterternary_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_scatterternary3d_chart, &demos());
    }

    #[test]
    fn every_scatterternary_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_scatterternary3d_chart, &demos(), 2000);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("scatterternary3d", build_scatterternary3d_chart, &demos(), ScatterTernaryVariant::default_key());
    }
}
