use crate::html::js_3d::render_3d_html;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::{colormap_3d, heatmap_layout_3d, HeatmapConfig, HeatmapVariant, HEIGHT_RATIO_3D};
use crate::plot::{apply_bg3d, parse_all};

pub fn render_heatmap3d_html(
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
        9,
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

#[crate::chart_demo("labels=[\"R1\",\"R2\"], categories=[\"C1\",\"C2\"], matrix=[[1,2],[3,4]]")]
#[crate::params(paramsList["title","labels","col_labels","categories","matrix","values","x_labels","variant","widths","ranges","bins","sort_order","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("heatmap3d", "heatmap_3d", "heatmap3d_chart", "heatmaps3d")]
#[crate::sera_builder]
pub fn build_heatmap3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let column_names = o.col_labels.clone().or_else(|| a.x_labels.clone());
    let (row_labels, col_labels) = match a.categories.clone() {
        Some(categories) => (categories, column_names.or_else(|| a.labels.clone()).unwrap_or_default()),
        None => (a.labels.clone().unwrap_or_default(), column_names.unwrap_or_default()),
    };
    let flat_matrix: Vec<f64> = match a.matrix {
        Some(rows) => rows.into_iter().flatten().collect(),
        None => a.values.unwrap_or_default(),
    };
    let variant = HeatmapVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let widths = o.widths.clone().unwrap_or_default();
    let heights = o.ranges.clone().unwrap_or_default();
    let cfg = HeatmapConfig {
        title,
        variant,
        row_labels: &row_labels,
        col_labels: &col_labels,
        flat_matrix: &flat_matrix,
        x_widths: &widths,
        y_heights: &heights,
        discrete_steps: o.bins.unwrap_or(0).max(0) as usize,
        ..HeatmapConfig::default()
    };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() {
        Some("#090d18")
    } else {
        bg_str.as_deref()
    };
    let view = BlockView::new(HEIGHT_RATIO_3D, colormap_3d(variant)).with_zone(o.zone.as_deref());
    let html = render_blocks3d_view_html(
        title,
        &heatmap_layout_3d(&cfg, &Budget::new(o.max_points)),
        &view,
        (&o.xl(), &o.yl(), &o.zl()),
        &[],
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
        id: 76,
        name: "heatmap_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use crate::plot::build_heatmap3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::HeatmapVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/heatmap/", HeatmapVariant::keys_and_aliases(), HeatmapVariant::default_key())
    }

    #[test]
    fn every_heatmap_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_heatmap3d_chart, &demos(), HeatmapVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_heatmap_variant() {
        twin::check_planes(build_heatmap3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_heatmap_variant() {
        twin::check_scenes(build_heatmap3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_heatmap_variant() {
        twin::check_themes(build_heatmap3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_heatmap_variants_and_the_view_axes() {
        twin::check_axes("heatmap3d", HeatmapVariant::all().len());
    }

    #[test]
    fn the_legacy_categories_and_labels_layout_still_renders() {
        let json = r#"{"title":"t","labels":["R1","R2"],"categories":["C1","C2"],"matrix":[[1,2],[3,4]]}"#;
        twin::assert_blocks(&build_heatmap3d_chart(json), "legacy heatmap3d call");
    }

    #[test]
    fn every_heatmap_variant_honours_an_explicit_zone() {
        twin::check_zone(build_heatmap3d_chart, &demos());
    }

    #[test]
    fn every_heatmap_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_heatmap3d_chart, &demos());
    }

    #[test]
    fn every_heatmap_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_heatmap3d_chart, &demos(), 2000);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("heatmap3d", build_heatmap3d_chart, &demos(), HeatmapVariant::default_key());
    }
}
