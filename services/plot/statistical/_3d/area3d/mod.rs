use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::area::layout3d;
use crate::plot::statistical::{AreaConfig, AreaVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo(
    "x_labels=[\"Q1\",\"Q2\",\"Q3\",\"Q4\"], series=[[11800,11500,12300,12800],[10500,10900,11100,11400],[10700,10800,10500,11300]], series_names=[\"North\",\"South\",\"East\"]"
)]
#[crate::params(paramsList["title","x_labels","series","series_names","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("area3d", "area_3d", "area3d_chart", "area3d_family", "areas3d")]
#[crate::sera_builder]
pub fn build_area3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x_labels = a.x_labels.or(a.labels).unwrap_or_default();
    let series_flat = a.series.or(a.values.map(|v| vec![v])).unwrap_or_default();
    let sn = o.series_names.clone().unwrap_or_default();
    let names: Vec<String> = if sn.is_empty() { (0..series_flat.len()).map(|_| String::new()).collect() } else { sn };
    let series: Vec<(String, Vec<f64>)> = names.into_iter().zip(series_flat).collect();
    let variant = AreaVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = AreaConfig { variant, title, x_labels: &x_labels, series: &series, ..AreaConfig::default() };
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
        id: 99,
        name: "area_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_area3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::AreaVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/area/", AreaVariant::keys_and_aliases(), AreaVariant::default_key())
    }

    #[test]
    fn every_area_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_area3d_chart, &demos(), AreaVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_area_variant() {
        twin::check_planes(build_area3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_area_variant() {
        twin::check_scenes(build_area3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_area_variant() {
        twin::check_themes(build_area3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_area_variants_and_the_view_axes() {
        twin::check_axes("area3d", AreaVariant::all().len());
    }

    #[test]
    fn every_area_variant_honours_an_explicit_zone() {
        twin::check_zone(build_area3d_chart, &demos());
    }

    #[test]
    fn every_area_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_area3d_chart, &demos());
    }

    #[test]
    fn every_area_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_area3d_chart, &demos(), 2000);
    }

    #[test]
    fn series_labels_reach_the_tooltip_names() {
        let html = build_area3d_chart(r#"{"title":"t","x_labels":["a","b"],"series":[[1,2]],"series_names":["North"]}"#);
        assert!(html.contains("'North'"));
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("area3d", build_area3d_chart, &demos(), AreaVariant::default_key());
    }
}
