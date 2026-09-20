use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::histogram::layout3d;
use crate::plot::statistical::{HistogramConfig, HistogramVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]")]
#[crate::params(paramsList["title","values","overlay","color_groups","series_names","variant","bins","x_label","y_label","z_label","bg_color","scene","orientation3d","theme","zone","width","height"])]
#[crate::sera_alias("histogram3d", "histogram_3d", "histogram3d_chart", "hist3d")]
#[crate::sera_builder]
pub fn build_histogram3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let values = a.values.unwrap_or_default();
    let overlay = a.overlay.unwrap_or_default();
    let categories = o.color_groups.clone().unwrap_or_default();
    let names = o.series_names.clone().unwrap_or_default();
    let series_names = (names.len() >= 2).then(|| (names[0].as_str(), names[1].as_str()));
    let variant = HistogramVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = HistogramConfig {
        title,
        variant,
        values: &values,
        bins: o.bins.unwrap_or(0).max(0) as usize,
        overlay_values: (!overlay.is_empty()).then_some(overlay.as_slice()),
        categories: &categories,
        series_names,
        ..HistogramConfig::default()
    };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() {
        Some("#090d18")
    } else {
        bg_str.as_deref()
    };
    let view = BlockView::new(layout3d::HEIGHT_RATIO, "").with_zone(o.zone.as_deref());
    let color_labels: Vec<String> = if categories.is_empty() { names.clone() } else { Vec::new() };
    let html = render_blocks3d_view_html(
        title,
        &layout3d::layout_3d(&cfg),
        &view,
        (&o.xl(), &o.yl(), &o.zl()),
        &color_labels,
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
        id: 91,
        name: "histogram_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use crate::plot::build_histogram3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::HistogramVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/histogram/", HistogramVariant::keys_and_aliases(), HistogramVariant::default_key())
    }

    #[test]
    fn every_histogram_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_histogram3d_chart, &demos(), HistogramVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_histogram_variant() {
        twin::check_planes(build_histogram3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_histogram_variant() {
        twin::check_scenes(build_histogram3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_histogram_variant() {
        twin::check_themes(build_histogram3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_histogram_variants_and_the_view_axes() {
        twin::check_axes("histogram3d", HistogramVariant::all().len());
    }

    #[test]
    fn every_histogram_variant_honours_an_explicit_zone() {
        twin::check_zone(build_histogram3d_chart, &demos());
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("histogram3d", build_histogram3d_chart, &demos(), HistogramVariant::default_key());
    }
}
