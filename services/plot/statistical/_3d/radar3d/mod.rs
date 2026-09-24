use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::radar::layout3d;
use crate::plot::statistical::RadarVariant;
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("axes=[\"Python\",\"Rust\",\"SQL\",\"ML\",\"DevOps\"], series=[[9,7,8,8,6],[5,10,6,4,9]], series_names=[\"Alice\",\"Bob\"]")]
#[crate::params(paramsList["title","axes","series","series_names","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("radar3d", "radar_3d", "radar3d_chart", "radar3d_family")]
#[crate::sera_builder]
pub fn build_radar3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let axes = a.axes.unwrap_or_default();
    let series_flat = a.series.unwrap_or_default();
    let names: Vec<String> = o.series_names.clone().unwrap_or_else(|| (0..series_flat.len()).map(|_| String::new()).collect());
    let series: Vec<(String, Vec<f64>)> = names.iter().cloned().zip(series_flat.iter().cloned()).collect();
    let variant = RadarVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let view = BlockView::new(0.65, "jet").with_zone(o.zone.as_deref());
    let (blocks, out_names) = layout3d::layout_named(&axes, &series, variant, &Budget::new(o.max_points));
    let html = render_blocks3d_view_html(
        title,
        &blocks,
        &view,
        (&o.xl(), &o.yl(), &o.zl()),
        &out_names,
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
        id: 70,
        name: "radar_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_radar3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::RadarVariant;

    fn demos() -> twin::Demos {
        let base = r#"{"axes":["Python","Rust","SQL","ML","DevOps"],"series":[[9,7,8,8,6],[5,10,6,4,9]],"series_names":["Alice","Bob"]}"#;
        RadarVariant::keys_and_aliases().iter().map(|(key, _)| (*key, twin::set_field(base, "variant", key))).collect()
    }

    #[test]
    fn every_radar_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_radar3d_chart, &demos(), RadarVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_radar_variant() {
        twin::check_planes(build_radar3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_radar_variant() {
        twin::check_scenes(build_radar3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_radar_variant() {
        twin::check_themes(build_radar3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_radar_variants_and_the_view_axes() {
        twin::check_axes("radar3d", RadarVariant::all().len());
    }

    #[test]
    fn every_radar_variant_honours_an_explicit_zone() {
        twin::check_zone(build_radar3d_chart, &demos());
    }

    #[test]
    fn every_radar_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_radar3d_chart, &demos());
    }

    #[test]
    fn every_radar_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_radar3d_chart, &demos(), 400);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("radar3d", build_radar3d_chart, &demos(), RadarVariant::default_key());
    }
}
