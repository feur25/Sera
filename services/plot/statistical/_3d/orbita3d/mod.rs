use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::orbita::layout3d;
use crate::plot::statistical::{OrbitaConfig, OrbitaVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("series_names=[\"2021\",\"2022\",\"2023\"], labels=[\"Q1\",\"Q2\",\"Q3\",\"Q4\"], matrix=[[0.4,0.7,0.5,0.8],[0.6,0.3,0.9,0.5],[0.8,0.6,0.4,0.7]]")]
#[crate::params(paramsList["title","series_names","labels","matrix","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("orbita3d", "orbita_3d", "orbita3d_chart", "orbita3d_family", "orbitas3d")]
#[crate::sera_builder]
pub fn build_orbita3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let series_names = o.series_names.clone().unwrap_or_default();
    let labels = a.labels.unwrap_or_default();
    let matrix: Vec<f64> = a.matrix.unwrap_or_default().into_iter().flatten().collect();
    let variant = OrbitaVariant::from_str(o.variant.as_deref().unwrap_or("classic"));
    let cfg = OrbitaConfig { variant, title, series_names: &series_names, labels: &labels, matrix: &matrix, ..OrbitaConfig::default() };
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
        id: 108,
        name: "orbita_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_orbita3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::OrbitaVariant;

    fn demos() -> twin::Demos {
        let base = r#"{"series_names":["2021","2022","2023"],"labels":["Q1","Q2","Q3","Q4"],"matrix":[[0.4,0.7,0.5,0.8],[0.6,0.3,0.9,0.5],[0.8,0.6,0.4,0.7]]}"#;
        OrbitaVariant::keys_and_aliases().iter().map(|(key, _)| (*key, twin::set_field(base, "variant", key))).collect()
    }

    #[test]
    fn every_orbita_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_orbita3d_chart, &demos(), OrbitaVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_orbita_variant() {
        twin::check_planes(build_orbita3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_orbita_variant() {
        twin::check_scenes(build_orbita3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_orbita_variant() {
        twin::check_themes(build_orbita3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_orbita_variants_and_the_view_axes() {
        twin::check_axes("orbita3d", OrbitaVariant::all().len());
    }

    #[test]
    fn every_orbita_variant_honours_an_explicit_zone() {
        twin::check_zone(build_orbita3d_chart, &demos());
    }

    #[test]
    fn every_orbita_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_orbita3d_chart, &demos());
    }

    #[test]
    fn every_orbita_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_orbita3d_chart, &demos(), 2000);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("orbita3d", build_orbita3d_chart, &demos(), OrbitaVariant::default_key());
    }
}
