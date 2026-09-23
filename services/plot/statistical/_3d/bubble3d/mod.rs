use crate::plot::statistical::_3d::budget::{even_indices, pick, Budget};
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::bubble::layout3d;
use crate::plot::statistical::{BubbleConfig, BubbleVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("x=[1,2,3], y=[1,2,3], z=[1,2,3], sizes=[10,20,30]")]
#[crate::params(paramsList["title","x","y","z","sizes","labels","categories","variant","palette","bg_color","scene","orientation3d","theme","zone","max_points","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias(
    "bubble3d",
    "bubble_3d",
    "bubble3d_chart",
    "bubble3d_family",
    "bubbles3d"
)]
#[crate::sera_builder]
pub fn build_bubble3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    if a.z.is_some() {
        let x = a.x.unwrap_or_default();
        let y = a.y.unwrap_or_default();
        let z = a.z.unwrap_or_default();
        let size_values = a.size.or(a.sizes).unwrap_or_default();
        let cv = o.color_values.clone().unwrap_or_default();
        let cl = o.color_labels.clone().unwrap_or_default();
        let n = x.len().min(y.len()).min(z.len()).min(size_values.len());
        let keep = even_indices(n, Budget::new(o.max_points).elements());
        let (x, y, z, size_values, cv) = (pick(&x, &keep), pick(&y, &keep), pick(&z, &keep), pick(&size_values, &keep), pick(&cv, &keep));
        let n = keep.len();
        let smn = size_values[..n].iter().cloned().fold(f64::INFINITY, f64::min);
        let smx = size_values[..n].iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let sr = (smx - smn).max(1e-9);
        let size_js = format!(
            "var S=[{}];",
            size_values[..n].iter().map(|&s| format!("{:.3}", (s - smn) / sr)).collect::<Vec<_>>().join(",")
        );
        let bg_str = o.bg_str();
        let html = crate::html::js_3d::render_3d_html_impl(
            16,
            title,
            &x[..n],
            &y[..n],
            &z[..n],
            (&o.xl(), &o.yl(), &o.zl()),
            &cv,
            &cl,
            o.w(900),
            o.h(560),
            bg_str.as_deref(),
            &o.scene3d(),
            size_js.as_bytes(),
        );
        return apply_bg3d(html, &o);
    }

    let x_values = a.x.unwrap_or_default();
    let y_values = a.y.unwrap_or_default();
    let sizes = a.size.or(a.sizes).unwrap_or_default();
    let labels = a.labels.unwrap_or_default();
    let categories = a.categories.unwrap_or_default();
    let x_categories = a.x_categories.unwrap_or_default();
    let y_categories = a.y_categories.unwrap_or_default();
    let variant = BubbleVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = BubbleConfig {
        variant,
        title,
        x_values: &x_values,
        y_values: &y_values,
        sizes: &sizes,
        labels: &labels,
        categories: &categories,
        x_categories: &x_categories,
        y_categories: &y_categories,
        ..BubbleConfig::default()
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

#[cfg(test)]
mod tests {
    use super::build_bubble3d_chart;
    use crate::plot::statistical::_3d::budget::Budget;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::BubbleVariant;

    fn columns(n: usize) -> serde_json::Value {
        serde_json::json!({"title": "t", "x": twin::wave(n, 0), "y": twin::wave(n, 1), "z": twin::wave(n, 2), "sizes": twin::wave(n, 3)})
    }

    #[test]
    fn a_big_bubble_chart_is_capped_by_the_budget_and_small_ones_are_untouched() {
        twin::check_capped(build_bubble3d_chart, columns, Budget::elements);
    }

    #[test]
    fn sizes_follow_the_kept_bubbles() {
        let html = build_bubble3d_chart(&columns(100_000).to_string());
        let sizes = html.split("var S=[").nth(1).and_then(|s| s.split(']').next()).map(|s| s.split(',').count());
        assert_eq!(sizes, Some(Budget::default().elements()));
    }

    fn demos() -> twin::Demos {
        let base = r#"{"x":[1,2,3,4,5],"y":[2,3,1,4,2.5],"sizes":[5,10,15,20,8]}"#;
        let split = r#"{"x_categories":["A","A","B"],"y_categories":["X","Y","X"],"categories":["S1","S2","S1"],"sizes":[5,8,3]}"#;
        BubbleVariant::keys_and_aliases()
            .iter()
            .map(|(key, _)| {
                let src = if *key == "split" { split } else { base };
                (*key, twin::set_field(src, "variant", key))
            })
            .collect()
    }

    #[test]
    fn every_bubble_variant_has_a_working_3d_counterpart_without_z() {
        twin::check_variants(build_bubble3d_chart, &demos(), BubbleVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_bubble_variant() {
        twin::check_planes(build_bubble3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_bubble_variant() {
        twin::check_scenes(build_bubble3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_bubble_variant() {
        twin::check_themes(build_bubble3d_chart, &demos());
    }

    #[test]
    fn every_bubble_variant_honours_an_explicit_zone() {
        twin::check_zone(build_bubble3d_chart, &demos());
    }

    #[test]
    fn every_bubble_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_bubble3d_chart, &demos());
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("bubble3d", build_bubble3d_chart, &demos(), BubbleVariant::default_key());
    }
}
